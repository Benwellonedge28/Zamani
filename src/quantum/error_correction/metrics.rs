//! Production-grade metrics and observability for Zamani Quantum Error
//! Correction.
//!
//! # Architectural contract
//!
//! Metrics are strictly observational. They must never determine whether a
//! QEC operation succeeds or fails.
//!
//! ```text
//!                         QEC EXECUTION
//!                              │
//!             ┌────────────────┴────────────────┐
//!             │                                 │
//!             ▼                                 ▼
//!       CORRECTNESS PATH                 OBSERVABILITY PATH
//!             │                                 │
//!       decoder result                 MetricsCollector
//!             │                                 │
//!       logical outcome                 MetricsSnapshot
//!                                             │
//!                         ┌───────────────────┼──────────────────┐
//!                         ▼                   ▼                  ▼
//!                     telemetry          threshold          checkpoint
//! ```
//!
//! # Resource architecture
//!
//! ```text
//! QecConfig
//!     │
//!     ▼
//! QecLimits ───────────────► ResourceManager
//!     │                            │
//!     │                            ▼
//!     │                    ResourceSnapshot
//!     │                            │
//!     └────────────────────────────┘
//!                                  │
//!                                  ▼
//!                         MetricsCollector
//! ```
//!
//! `limits.rs` remains the declarative policy layer.
//! `resources.rs` remains the enforcement/accounting layer.
//! `metrics.rs` observes both execution and resource state.
//!
//! Metrics never create an independent resource policy.
//!
//! # Determinism
//!
//! Counters are stored as integers and updated atomically. Floating-point
//! rates are derived only when a snapshot is requested.
//!
//! Distributed metric aggregation is performed by integer addition/max
//! operations, avoiding floating-point accumulation order dependence.
//!
//! Wall-clock measurements are explicitly observational and therefore are not
//! part of deterministic correctness comparisons.
//!
//! # Security
//!
//! This module deliberately stores aggregate values rather than:
//!
//! - raw syndrome streams;
//! - quantum circuits;
//! - measurement payloads;
//! - QPU credentials;
//! - device secrets;
//! - topology secrets;
//! - user data.
//!
//! Such data belongs outside the metrics layer and must be governed by the
//! telemetry/security policy.
//!
//! # Overflow policy
//!
//! Metrics must never cause a QEC computation to fail because a counter became
//! too large. Counters therefore saturate at their representable maximum.
//!
//! Resource enforcement remains the responsibility of `resources.rs`.
//!
//! # Large-scale execution
//!
//! The collector stores bounded aggregate state only. It does not retain
//! individual events, graph nodes, corrections, syndrome streams, or worker
//! histories.
//!
//! This makes it suitable for:
//!
//! - streaming QEC;
//! - partitioned decoding;
//! - distributed decoding;
//! - large threshold experiments;
//! - QPU workloads;
//! - long-running services.

use core::fmt;
use std::sync::atomic::{
    AtomicBool,
    AtomicU64,
    Ordering,
};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::resources::ResourceSnapshot;

/* ========================================================================== */
/* Errors                                                                     */
/* ========================================================================== */

/// Errors associated with metric configuration or aggregation.
///
/// Normal metric recording is intentionally infallible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetricsError {
    /// Configuration specified an invalid capacity.
    InvalidCapacity {
        name: &'static str,
    },

    /// A metrics aggregation operation encountered incompatible metadata.
    IncompatibleConfiguration {
        reason: &'static str,
    },
}

impl fmt::Display for MetricsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCapacity { name } => {
                write!(f, "invalid metrics capacity for {name}")
            }

            Self::IncompatibleConfiguration { reason } => {
                write!(
                    f,
                    "incompatible metrics configuration: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for MetricsError {}

/* ========================================================================== */
/* Decoder identity                                                           */
/* ========================================================================== */

/// Stable decoder identifier used by metrics.
///
/// This deliberately remains independent from the error hierarchy so metrics
/// can be collected by low-level workers without importing decoder errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecoderId {
    SurfaceCode,
    Mwpm,
    UnionFind,
    BeliefPropagation,
    TensorNetwork,
    LookupTable,
    Streaming,
    Distributed,
    Custom,
}

impl DecoderId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceCode => "surface_code",
            Self::Mwpm => "mwpm",
            Self::UnionFind => "union_find",
            Self::BeliefPropagation => "belief_propagation",
            Self::TensorNetwork => "tensor_network",
            Self::LookupTable => "lookup_table",
            Self::Streaming => "streaming",
            Self::Distributed => "distributed",
            Self::Custom => "custom",
        }
    }
}

impl Default for DecoderId {
    fn default() -> Self {
        Self::Custom
    }
}

impl fmt::Display for DecoderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/* ========================================================================== */
/* Backend identity                                                           */
/* ========================================================================== */

/// Execution backend represented in metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendKind {
    Cpu,
    ParallelCpu,
    Gpu,
    Accelerator,
    Distributed,
    Simulator,
    Emulator,
    Qpu,
    Custom,
}

impl BackendKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::ParallelCpu => "parallel_cpu",
            Self::Gpu => "gpu",
            Self::Accelerator => "accelerator",
            Self::Distributed => "distributed",
            Self::Simulator => "simulator",
            Self::Emulator => "emulator",
            Self::Qpu => "qpu",
            Self::Custom => "custom",
        }
    }

    pub const fn is_qpu(self) -> bool {
        matches!(self, Self::Qpu)
    }
}

impl Default for BackendKind {
    fn default() -> Self {
        Self::Cpu
    }
}

impl fmt::Display for BackendKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/* ========================================================================== */
/* QPU metrics                                                                */
/* ========================================================================== */

/// Aggregate QPU execution metrics.
///
/// No credentials, circuit contents, raw measurements, or device secrets are
/// stored here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct QpuMetrics {
    /// Number of QPU shots submitted.
    pub shots: u64,

    /// Number of QPU circuits submitted.
    pub circuits: u64,

    /// Number of QPU measurement operations observed.
    pub measurement_count: u64,

    /// Aggregate queue time.
    pub queue_time: Duration,

    /// Aggregate QPU execution time.
    pub execution_time: Duration,

    /// Number of QPU submissions that failed.
    pub submission_failures: u64,

    /// Number of malformed/invalid measurement results rejected.
    pub invalid_measurements: u64,

    /// Number of QPU jobs completed.
    pub completed_jobs: u64,

    /// Number of QPU jobs cancelled.
    pub cancelled_jobs: u64,

    /// Readout-error observations, when supplied by the backend.
    pub readout_error_count: u64,

    /// Readout-error opportunities, when supplied by the backend.
    pub readout_error_opportunities: u64,
}

impl QpuMetrics {
    pub fn readout_error_rate(&self) -> Option<f64> {
        ratio(
            self.readout_error_count,
            self.readout_error_opportunities,
        )
    }

    fn saturating_add_assign(&mut self, other: &Self) {
        self.shots = self.shots.saturating_add(other.shots);
        self.circuits = self.circuits.saturating_add(other.circuits);
        self.measurement_count =
            self.measurement_count.saturating_add(other.measurement_count);

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
            .saturating_add(other.readout_error_opportunities);
    }
}

/* ========================================================================== */
/* Configuration                                                              */
/* ========================================================================== */

/// Configuration controlling metric behavior.
///
/// Metrics remain aggregate-only. `max_custom_counters` is retained as an API
/// compatibility boundary for future bounded custom-counter support.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetricsConfig {
    /// Maximum custom metric cardinality permitted by future extensions.
    pub max_custom_counters: usize,

    /// Whether worker count should be tracked.
    pub track_worker_count: bool,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            max_custom_counters: 1024,
            track_worker_count: true,
        }
    }
}

impl MetricsConfig {
    pub fn validate(&self) -> Result<(), MetricsError> {
        if self.max_custom_counters == 0 {
            return Err(MetricsError::InvalidCapacity {
                name: "max_custom_counters",
            });
        }

        Ok(())
    }
}

/* ========================================================================== */
/* Metrics snapshot                                                           */
/* ========================================================================== */

/// Immutable aggregate metrics snapshot.
///
/// All counters are integer based. Rates are derived from those counters.
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
    /* Parallel/distributed execution                                         */
    /* ---------------------------------------------------------------------- */

    pub worker_count: u64,

    /* ---------------------------------------------------------------------- */
    /* Timing                                                                  */
    /* ---------------------------------------------------------------------- */

    /// Sum of decoder operation latency.
    pub decoder_latency: Duration,

    pub max_decoder_latency: Duration,

    pub min_decoder_latency: Option<Duration>,

    pub average_decoder_latency: Option<Duration>,

    /// Wall-clock duration since collector creation.
    ///
    /// This is observational and must not be used for deterministic
    /// correctness comparisons.
    pub wall_time: Duration,

    /// Backend-reported compute time.
    pub compute_time: Duration,

    /* ---------------------------------------------------------------------- */
    /* Resources                                                               */
    /* ---------------------------------------------------------------------- */

    pub peak_memory: u64,
    pub current_memory: u64,

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

    /// True only when every started operation has reached a terminal state.
    pub operations_balanced: bool,

    /* ---------------------------------------------------------------------- */
    /* QPU                                                                      */
    /* ---------------------------------------------------------------------- */

    pub qpu: QpuMetrics,
}

impl MetricsSnapshot {
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

            worker_count: 0,

            decoder_latency: Duration::ZERO,
            max_decoder_latency: Duration::ZERO,
            min_decoder_latency: None,
            average_decoder_latency: None,

            wall_time: Duration::ZERO,
            compute_time: Duration::ZERO,

            peak_memory: 0,
            current_memory: 0,

            physical_error_rate: None,
            logical_error_rate: None,
            decoder_success_rate: None,
            decoder_failure_rate: None,

            had_logical_failure: false,
            operations_balanced: true,

            qpu: QpuMetrics::default(),
        }
    }

    pub fn logical_error_rate_or_zero(&self) -> f64 {
        self.logical_error_rate.unwrap_or(0.0)
    }

    pub fn physical_error_rate_or_zero(&self) -> f64 {
        self.physical_error_rate.unwrap_or(0.0)
    }

    pub fn all_decodes_successful(&self) -> bool {
        self.decode_operations > 0
            && self.decoder_failure == 0
            && self.cancellation_count == 0
            && self.operations_balanced
    }

    pub fn terminal_operations(&self) -> u64 {
        self.decoder_success
            .saturating_add(self.decoder_failure)
            .saturating_add(self.cancellation_count)
    }

    /// Adds another immutable snapshot into this snapshot.
    ///
    /// Identity and high-water-mark fields are reconciled deterministically.
    pub fn merge(&mut self, other: &Self) -> Result<(), MetricsError> {
        if self.decoder != other.decoder {
            return Err(MetricsError::IncompatibleConfiguration {
                reason: "decoder identities differ",
            });
        }

        if self.backend != other.backend {
            return Err(MetricsError::IncompatibleConfiguration {
                reason: "backend identities differ",
            });
        }

        self.decode_operations =
            self.decode_operations.saturating_add(other.decode_operations);

        self.decoder_success =
            self.decoder_success.saturating_add(other.decoder_success);

        self.decoder_failure =
            self.decoder_failure.saturating_add(other.decoder_failure);

        self.cancellation_count =
            self.cancellation_count.saturating_add(other.cancellation_count);

        self.correction_count =
            self.correction_count.saturating_add(other.correction_count);

        self.detection_event_count =
            self.detection_event_count.saturating_add(other.detection_event_count);

        self.physical_error_count =
            self.physical_error_count.saturating_add(other.physical_error_count);

        self.physical_error_opportunities = self
            .physical_error_opportunities
            .saturating_add(other.physical_error_opportunities);

        self.logical_failure_count =
            self.logical_failure_count.saturating_add(other.logical_failure_count);

        self.logical_error_opportunities = self
            .logical_error_opportunities
            .saturating_add(other.logical_error_opportunities);

        self.matching_count =
            self.matching_count.saturating_add(other.matching_count);

        self.decoder_iterations =
            self.decoder_iterations.saturating_add(other.decoder_iterations);

        self.graph_nodes =
            self.graph_nodes.saturating_add(other.graph_nodes);

        self.graph_edges =
            self.graph_edges.saturating_add(other.graph_edges);

        self.qubit_count = self.qubit_count.max(other.qubit_count);
        self.stabilizer_count =
            self.stabilizer_count.max(other.stabilizer_count);

        self.measurement_rounds =
            self.measurement_rounds.max(other.measurement_rounds);

        self.worker_count =
            self.worker_count.max(other.worker_count);

        self.decoder_latency = saturating_duration_add(
            self.decoder_latency,
            other.decoder_latency,
        );

        self.max_decoder_latency =
            self.max_decoder_latency.max(other.max_decoder_latency);

        self.min_decoder_latency =
            match (self.min_decoder_latency, other.min_decoder_latency) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            };

        self.average_decoder_latency =
            if self.decode_operations > 0 {
                Some(duration_divide(
                    self.decoder_latency,
                    self.decode_operations,
                ))
            } else {
                None
            };

        self.peak_memory =
            self.peak_memory.max(other.peak_memory);

        self.current_memory = other.current_memory;

        self.compute_time = saturating_duration_add(
            self.compute_time,
            other.compute_time,
        );

        self.had_logical_failure |= other.had_logical_failure;

        self.qpu.saturating_add_assign(&other.qpu);

        self.recalculate_rates();

        self.operations_balanced =
            self.decode_operations == self.terminal_operations();

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

/// Aggregate result suitable for threshold experiments.
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

            logical_error_rate: snapshot.logical_error_rate,

            decoder_failures: snapshot.decoder_failure,
            cancelled_trials: snapshot.cancellation_count,

            total_latency: snapshot.decoder_latency,

            average_latency: snapshot.average_decoder_latency,

            peak_memory: snapshot.peak_memory,
        }
    }
}

/* ========================================================================== */
/* Atomic counters                                                            */
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

    worker_count: AtomicU64,

    decoder_latency_nanos: AtomicU64,
    max_decoder_latency_nanos: AtomicU64,
    min_decoder_latency_nanos: AtomicU64,

    peak_memory: AtomicU64,
    current_memory: AtomicU64,

    compute_time_nanos: AtomicU64,

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
            decode_operations: AtomicU64::new(0),
            decoder_success: AtomicU64::new(0),
            decoder_failure: AtomicU64::new(0),
            cancellation_count: AtomicU64::new(0),

            correction_count: AtomicU64::new(0),
            detection_event_count: AtomicU64::new(0),

            physical_error_count: AtomicU64::new(0),
            physical_error_opportunities: AtomicU64::new(0),

            logical_failure_count: AtomicU64::new(0),
            logical_error_opportunities: AtomicU64::new(0),

            matching_count: AtomicU64::new(0),
            decoder_iterations: AtomicU64::new(0),

            graph_nodes: AtomicU64::new(0),
            graph_edges: AtomicU64::new(0),

            qubit_count: AtomicU64::new(0),
            stabilizer_count: AtomicU64::new(0),
            measurement_rounds: AtomicU64::new(0),

            worker_count: AtomicU64::new(0),

            decoder_latency_nanos: AtomicU64::new(0),
            max_decoder_latency_nanos: AtomicU64::new(0),
            min_decoder_latency_nanos: AtomicU64::new(u64::MAX),

            peak_memory: AtomicU64::new(0),
            current_memory: AtomicU64::new(0),

            compute_time_nanos: AtomicU64::new(0),

            qpu_shots: AtomicU64::new(0),
            qpu_circuits: AtomicU64::new(0),
            qpu_measurement_count: AtomicU64::new(0),
            qpu_queue_time_nanos: AtomicU64::new(0),
            qpu_execution_time_nanos: AtomicU64::new(0),
            qpu_submission_failures: AtomicU64::new(0),
            qpu_invalid_measurements: AtomicU64::new(0),
            qpu_completed_jobs: AtomicU64::new(0),
            qpu_cancelled_jobs: AtomicU64::new(0),
            qpu_readout_error_count: AtomicU64::new(0),
            qpu_readout_error_opportunities: AtomicU64::new(0),

            had_logical_failure: AtomicBool::new(false),
        }
    }
}

/* ========================================================================== */
/* Collector                                                                  */
/* ========================================================================== */

/// Thread-safe aggregate metrics collector.
///
/// It can safely be shared by decoder workers.
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

    pub const fn decoder(&self) -> DecoderId {
        self.decoder
    }

    pub const fn backend(&self) -> BackendKind {
        self.backend
    }

    pub const fn config(&self) -> MetricsConfig {
        self.config
    }

    /* ---------------------------------------------------------------------- */
    /* Decoder lifecycle                                                      */
    /* ---------------------------------------------------------------------- */

    /// Begins one decoder operation.
    pub fn begin_decode(&self) {
        saturating_increment(
            &self.counters.decode_operations,
        );
    }

    pub fn record_success(&self, latency: Duration) {
        saturating_increment(
            &self.counters.decoder_success,
        );

        self.record_latency(latency);
    }

    pub fn record_failure(&self, latency: Duration) {
        saturating_increment(
            &self.counters.decoder_failure,
        );

        self.record_latency(latency);
    }

    pub fn record_cancellation(&self, latency: Duration) {
        saturating_increment(
            &self.counters.cancellation_count,
        );

        self.record_latency(latency);
    }

    /// Records a complete distributed/remote decoder outcome.
    ///
    /// Unlike `record_success`, `record_failure`, and `record_cancellation`,
    /// this method starts the operation itself.
    pub fn record_outcome(
        &self,
        success: bool,
        cancelled: bool,
        latency: Duration,
    ) {
        self.begin_decode();

        match (success, cancelled) {
            (_, true) => self.record_cancellation(latency),
            (true, false) => self.record_success(latency),
            (false, false) => self.record_failure(latency),
        }
    }

    /* ---------------------------------------------------------------------- */
    /* QEC activity                                                            */
    /* ---------------------------------------------------------------------- */

    pub fn record_corrections(&self, count: u64) {
        saturating_add(
            &self.counters.correction_count,
            count,
        );
    }

    pub fn record_detection_events(&self, count: u64) {
        saturating_add(
            &self.counters.detection_event_count,
            count,
        );
    }

    pub fn record_physical_errors(&self, count: u64) {
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
                .store(true, Ordering::Release);
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

    pub fn record_matching(&self, count: u64) {
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

    pub fn record_graph_nodes(&self, count: u64) {
        saturating_add(
            &self.counters.graph_nodes,
            count,
        );
    }

    pub fn record_graph_edges(&self, count: u64) {
        saturating_add(
            &self.counters.graph_edges,
            count,
        );
    }

    /// Qubit count is a high-water mark.
    pub fn record_qubits(&self, count: u64) {
        saturating_max(
            &self.counters.qubit_count,
            count,
        );
    }

    /// Stabilizer count is a high-water mark.
    pub fn record_stabilizers(&self, count: u64) {
        saturating_max(
            &self.counters.stabilizer_count,
            count,
        );
    }

    /// Measurement rounds are a high-water mark.
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
    /* Parallel/distributed execution                                          */
    /* ---------------------------------------------------------------------- */

    pub fn record_workers(&self, workers: usize) {
        if !self.config.track_worker_count {
            return;
        }

        saturating_max(
            &self.counters.worker_count,
            usize_to_u64_saturating(workers),
        );
    }

    /* ---------------------------------------------------------------------- */
    /* Timing                                                                  */
    /* ---------------------------------------------------------------------- */

    pub fn record_latency(&self, latency: Duration) {
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
            duration_to_nanos_saturating(duration),
        );
    }

    pub fn start_timer(&self) -> MetricsTimer<'_> {
        MetricsTimer {
            collector: self,
            started: Instant::now(),
            stopped: false,
        }
    }

    /* ---------------------------------------------------------------------- */
    /* Resource integration                                                    */
    /* ---------------------------------------------------------------------- */

    /// Observes a resource-manager snapshot.
    ///
    /// This method never enforces resource limits. Enforcement remains inside
    /// `ResourceManager`.
    pub fn record_resource_snapshot(
        &self,
        snapshot: ResourceSnapshot,
    ) {
        self.record_memory(snapshot.allocated_bytes);

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

        self.record_workers(snapshot.parallel_workers);

        self.record_qubits(
            usize_to_u64_saturating(snapshot.qubits),
        );

        self.record_stabilizers(
            usize_to_u64_saturating(snapshot.stabilizers),
        );

        self.record_measurement_rounds(
            usize_to_u64_saturating(
                snapshot.measurement_rounds,
            ),
        );

        saturating_max(
            &self.counters.compute_time_nanos,
            duration_to_nanos_saturating(
                snapshot.compute_time,
            ),
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
    /* QPU                                                                      */
    /* ---------------------------------------------------------------------- */

    pub fn record_qpu_shots(&self, count: u64) {
        saturating_add(
            &self.counters.qpu_shots,
            count,
        );
    }

    pub fn record_qpu_circuits(
        &self,
        count: u64,
    ) {
        saturating_add(
            &self.counters.qpu_circuits,
            count,
        );
    }

    pub fn record_qpu_measurements(
        &self,
        count: u64,
    ) {
        saturating_add(
            &self.counters.qpu_measurement_count,
            count,
        );
    }

    pub fn record_qpu_queue_time(
        &self,
        duration: Duration,
    ) {
        saturating_add(
            &self.counters.qpu_queue_time_nanos,
            duration_to_nanos_saturating(duration),
        );
    }

    pub fn record_qpu_execution_time(
        &self,
        duration: Duration,
    ) {
        saturating_add(
            &self.counters.qpu_execution_time_nanos,
            duration_to_nanos_saturating(duration),
        );
    }

    pub fn record_qpu_submission_failure(&self) {
        saturating_increment(
            &self.counters.qpu_submission_failures,
        );
    }

    pub fn record_qpu_invalid_measurements(
        &self,
        count: u64,
    ) {
        saturating_add(
            &self.counters.qpu_invalid_measurements,
            count,
        );
    }

    pub fn record_qpu_job_completed(&self) {
        saturating_increment(
            &self.counters.qpu_completed_jobs,
        );
    }

    pub fn record_qpu_job_cancelled(&self) {
        saturating_increment(
            &self.counters.qpu_cancelled_jobs,
        );
    }

    pub fn record_qpu_readout_trial(
        &self,
        error_occurred: bool,
    ) {
        saturating_increment(
            &self.counters.qpu_readout_error_opportunities,
        );

        if error_occurred {
            saturating_increment(
                &self.counters.qpu_readout_error_count,
            );
        }
    }

    pub fn record_qpu_metrics(
        &self,
        metrics: QpuMetrics,
    ) {
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
            &self.counters.qpu_submission_failures,
            metrics.submission_failures,
        );

        saturating_add(
            &self.counters.qpu_invalid_measurements,
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
            &self.counters.qpu_readout_error_count,
            metrics.readout_error_count,
        );

        saturating_add(
            &self.counters.qpu_readout_error_opportunities,
            metrics.readout_error_opportunities,
        );
    }

    /* ---------------------------------------------------------------------- */
    /* Snapshot                                                                 */
    /* ---------------------------------------------------------------------- */

    pub fn snapshot(&self) -> MetricsSnapshot {
        let decode_operations =
            load(&self.counters.decode_operations);

        let decoder_success =
            load(&self.counters.decoder_success);

        let decoder_failure =
            load(&self.counters.decoder_failure);

        let cancellation_count =
            load(&self.counters.cancellation_count);

        let decoder_latency_nanos =
            load(&self.counters.decoder_latency_nanos);

        let min_latency =
            load(&self.counters.min_decoder_latency_nanos);

        let physical_errors =
            load(&self.counters.physical_error_count);

        let physical_opportunities =
            load(&self.counters.physical_error_opportunities);

        let logical_failures =
            load(&self.counters.logical_failure_count);

        let logical_opportunities =
            load(&self.counters.logical_error_opportunities);

        let qpu = QpuMetrics {
            shots: load(
                &self.counters.qpu_shots,
            ),
            circuits: load(
                &self.counters.qpu_circuits,
            ),
            measurement_count: load(
                &self.counters.qpu_measurement_count,
            ),
            queue_time: Duration::from_nanos(
                load(
                    &self.counters.qpu_queue_time_nanos,
                ),
            ),
            execution_time: Duration::from_nanos(
                load(
                    &self.counters.qpu_execution_time_nanos,
                ),
            ),
            submission_failures: load(
                &self.counters.qpu_submission_failures,
            ),
            invalid_measurements: load(
                &self.counters.qpu_invalid_measurements,
            ),
            completed_jobs: load(
                &self.counters.qpu_completed_jobs,
            ),
            cancelled_jobs: load(
                &self.counters.qpu_cancelled_jobs,
            ),
            readout_error_count: load(
                &self.counters.qpu_readout_error_count,
            ),
            readout_error_opportunities: load(
                &self.counters.qpu_readout_error_opportunities,
            ),
        };

        let terminal_operations =
            decoder_success
                .saturating_add(decoder_failure)
                .saturating_add(cancellation_count);

        let decoder_latency =
            Duration::from_nanos(
                decoder_latency_nanos,
            );

        MetricsSnapshot {
            decoder: self.decoder,
            backend: self.backend,

            decode_operations,

            decoder_success,
            decoder_failure,
            cancellation_count,

            correction_count: load(
                &self.counters.correction_count,
            ),

            detection_event_count: load(
                &self.counters.detection_event_count,
            ),

            physical_error_count: physical_errors,
            physical_error_opportunities: physical_opportunities,

            logical_failure_count: logical_failures,
            logical_error_opportunities: logical_opportunities,

            matching_count: load(
                &self.counters.matching_count,
            ),

            decoder_iterations: load(
                &self.counters.decoder_iterations,
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
                &self.counters.measurement_rounds,
            ),

            worker_count: load(
                &self.counters.worker_count,
            ),

            decoder_latency,

            max_decoder_latency: Duration::from_nanos(
                load(
                    &self.counters.max_decoder_latency_nanos,
                ),
            ),

            min_decoder_latency:
                if min_latency == u64::MAX {
                    None
                } else {
                    Some(Duration::from_nanos(
                        min_latency,
                    ))
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

            wall_time: self.started.elapsed(),

            compute_time: Duration::from_nanos(
                load(
                    &self.counters.compute_time_nanos,
                ),
            ),

            peak_memory: load(
                &self.counters.peak_memory,
            ),

            current_memory: load(
                &self.counters.current_memory,
            ),

            physical_error_rate: ratio(
                physical_errors,
                physical_opportunities,
            ),

            logical_error_rate: ratio(
                logical_failures,
                logical_opportunities,
            ),

            decoder_success_rate: ratio(
                decoder_success,
                decode_operations,
            ),

            decoder_failure_rate: ratio(
                decoder_failure,
                decode_operations,
            ),

            had_logical_failure: self
                .counters
                .had_logical_failure
                .load(Ordering::Acquire),

            operations_balanced:
                decode_operations == terminal_operations,

            qpu,
        }
    }

    /* ---------------------------------------------------------------------- */
    /* Reset                                                                    */
    /* ---------------------------------------------------------------------- */

    /// Resets the current measurement window.
    ///
    /// Decoder identity and configuration are preserved.
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
            .peak_memory
            .store(0, Ordering::Release);

        self.counters
            .current_memory
            .store(0, Ordering::Release);

        self.counters
            .compute_time_nanos
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

/// RAII timer for decoder latency.
///
/// The timer records exactly once:
///
/// - explicit `stop()`, or
/// - `Drop`.
#[derive(Debug)]
pub struct MetricsTimer<'a> {
    collector: &'a MetricsCollector,
    started: Instant,
    stopped: bool,
}

impl MetricsTimer<'_> {
    pub fn stop(mut self) -> Duration {
        if self.stopped {
            return self.started.elapsed();
        }

        let elapsed = self.started.elapsed();

        self.collector.record_latency(elapsed);

        self.stopped = true;

        elapsed
    }

    pub fn elapsed(&self) -> Duration {
        self.started.elapsed()
    }
}

impl Drop for MetricsTimer<'_> {
    fn drop(&mut self) {
        if !self.stopped {
            self.collector
                .record_latency(self.started.elapsed());

            self.stopped = true;
        }
    }
}

/* ========================================================================== */
/* Shared collector                                                           */
/* ========================================================================== */

pub type SharedMetrics = Arc<MetricsCollector>;

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

fn load(value: &AtomicU64) -> u64 {
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
            Some(current.saturating_add(amount))
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
        .min(u64::MAX as u128) as u64
}

fn saturating_duration_add(
    a: Duration,
    b: Duration,
) -> Duration {
    let nanos = a
        .as_nanos()
        .saturating_add(b.as_nanos())
        .min(u64::MAX as u128);

    Duration::from_nanos(nanos as u64)
}

fn duration_divide(
    duration: Duration,
    divisor: u64,
) -> Duration {
    if divisor == 0 {
        return Duration::ZERO;
    }

    let nanos = duration.as_nanos() / divisor as u128;

    Duration::from_nanos(
        nanos.min(u64::MAX as u128) as u64,
    )
}

fn usize_to_u64_saturating(
    value: usize,
) -> u64 {
    value.min(u64::MAX as usize) as u64
}

/* ========================================================================== */
/* Tests                                                                      */
/* ========================================================================== */

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn standard_collector_starts_empty() {
        let metrics = MetricsCollector::standard(
            DecoderId::Mwpm,
            BackendKind::Cpu,
        );

        let snapshot = metrics.snapshot();

        assert_eq!(
            snapshot.decoder,
            DecoderId::Mwpm
        );

        assert_eq!(
            snapshot.backend,
            BackendKind::Cpu
        );

        assert_eq!(
            snapshot.decode_operations,
            0
        );

        assert_eq!(
            snapshot.decoder_success,
            0
        );

        assert_eq!(
            snapshot.decoder_failure,
            0
        );

        assert_eq!(
            snapshot.physical_error_rate,
            None
        );

        assert_eq!(
            snapshot.logical_error_rate,
            None
        );

        assert_eq!(
            snapshot.qpu.shots,
            0
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
            .record_physical_error_opportunities(100);

        metrics.record_physical_errors(10);

        let snapshot =
            metrics.snapshot();

        assert_eq!(
            snapshot.physical_error_rate,
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
            metrics.record_logical_trial(false);
        }

        for _ in 0..5 {
            metrics.record_logical_trial(true);
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

        assert!(
            (snapshot
                .logical_error_rate
                .unwrap()
                - 0.05)
                .abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn decoder_outcomes_are_balanced() {
        let metrics =
            MetricsCollector::standard(
                DecoderId::UnionFind,
                BackendKind::Cpu,
            );

        metrics.record_outcome(
            true,
            false,
            Duration::from_millis(2),
        );

        metrics.record_outcome(
            false,
            false,
            Duration::from_millis(3),
        );

        metrics.record_outcome(
            false,
            true,
            Duration::from_millis(1),
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
    fn latency_statistics_are_recorded() {
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
    fn memory_is_a_high_water_mark() {
        let metrics =
            MetricsCollector::standard(
                DecoderId::Mwpm,
                BackendKind::Cpu,
            );

        metrics.record_memory(1024);
        metrics.record_memory(4096);
        metrics.record_memory(2048);

        let snapshot =
            metrics.snapshot();

        assert_eq!(
            snapshot.current_memory,
            2048
        );

        assert_eq!(
            snapshot.peak_memory,
            4096
        );
    }

    #[test]
    fn qpu_metrics_are_recorded() {
        let metrics =
            MetricsCollector::standard(
                DecoderId::SurfaceCode,
                BackendKind::Qpu,
            );

        metrics.record_qpu_shots(1000);
        metrics.record_qpu_circuits(20);
        metrics.record_qpu_measurements(4000);

        metrics.record_qpu_queue_time(
            Duration::from_millis(20),
        );

        metrics.record_qpu_execution_time(
            Duration::from_millis(100),
        );

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
    fn resource_snapshot_updates_metrics() {
        let metrics =
            MetricsCollector::standard(
                DecoderId::Mwpm,
                BackendKind::ParallelCpu,
            );

        let resources =
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

                checkpoint_bytes: 0,
                partitions: 0,
                stream_buffer_events: 0,

                qpu_shots: 0,
                qpu_circuits: 0,

                verification_operations: 0,

                wall_time: Duration::from_secs(1),
                compute_time: Duration::from_millis(500),
            };

        metrics.record_resource_snapshot(
            resources,
        );

        let snapshot =
            metrics.snapshot();

        assert_eq!(
            snapshot.current_memory,
            10_000
        );

        assert_eq!(
            snapshot.peak_memory,
            20_000
        );

        assert_eq!(
            snapshot.detection_event_count,
            100
        );

        assert_eq!(
            snapshot.graph_nodes,
            200
        );

        assert_eq!(
            snapshot.graph_edges,
            400
        );

        assert_eq!(
            snapshot.decoder_iterations,
            50
        );

        assert_eq!(
            snapshot.worker_count,
            8
        );

        assert_eq!(
            snapshot.qubit_count,
            625
        );

        assert_eq!(
            snapshot.stabilizer_count,
            624
        );
    }

    #[test]
    fn timer_records_on_drop() {
        let metrics =
            MetricsCollector::standard(
                DecoderId::Mwpm,
                BackendKind::Cpu,
            );

        {
            let _timer =
                metrics.start_timer();

            thread::sleep(
                Duration::from_millis(1),
            );
        }

        let snapshot =
            metrics.snapshot();

        assert!(
            snapshot.decoder_latency
                >= Duration::from_millis(1)
        );
    }

    #[test]
    fn timer_records_only_once_when_stopped() {
        let metrics =
            MetricsCollector::standard(
                DecoderId::Mwpm,
                BackendKind::Cpu,
            );

        let timer =
            metrics.start_timer();

        let _ = timer.stop();

        let snapshot =
            metrics.snapshot();

        assert!(
            snapshot.decoder_latency
                > Duration::ZERO
        );
    }

    #[test]
    fn snapshots_can_be_merged_for_distributed_execution() {
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
    fn incompatible_snapshots_are_rejected() {
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
    fn shared_metrics_are_thread_safe() {
        let metrics =
            shared_metrics(
                DecoderId::Mwpm,
                BackendKind::ParallelCpu,
            );

        let mut handles =
            Vec::new();

        for _ in 0..4 {
            let metrics =
                Arc::clone(&metrics);

            handles.push(
                thread::spawn(
                    move || {
                        for _ in 0..1000 {
                            metrics
                                .record_detection_events(1);

                            metrics
                                .record_matching(1);
                        }
                    },
                ),
            );
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let snapshot =
            metrics.snapshot();

        assert_eq!(
            snapshot.detection_event_count,
            4000
        );

        assert_eq!(
            snapshot.matching_count,
            4000
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

        assert!(
            snapshot.logical_error_rate
                .is_none()
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
            metrics.record_logical_trial(false);
        }

        metrics.record_logical_trial(true);

        let snapshot =
            metrics.snapshot();

        let threshold =
            ThresholdMetrics::from_snapshot(
                &snapshot,
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
    fn configuration_rejects_zero_capacity() {
        let config =
            MetricsConfig {
                max_custom_counters: 0,
                track_worker_count: true,
            };

        assert!(
            config.validate().is_err()
        );
    }

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
    }
}