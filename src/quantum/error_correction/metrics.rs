//! Production-grade metrics and observability for Zamani QEC.
//!
//! This module provides thread-safe, bounded, deterministic metrics for
//! quantum-error-correction workloads.
//!
//! # Design goals
//!
//! - No panics for normal metric recording.
//! - Thread-safe recording from decoder workers.
//! - Deterministic snapshots.
//! - Integer-based counters for reproducibility.
//! - Floating-point rates calculated only when a snapshot is requested.
//! - Explicit distinction between physical and logical errors.
//! - Decoder success/failure accounting.
//! - Resource-aware metrics.
//! - Streaming-friendly incremental recording.
//! - Distributed-worker aggregation.
//! - Threshold-experiment support.
//! - No correctness dependency on telemetry or metrics.
//!
//! Metrics are observational only:
//!
//! ```text
//! QEC computation
//!       |
//!       +----> correctness path
//!       |
//!       +----> metrics path
//! ```
//!
//! A metrics failure must never change the mathematical result of a decoder.
//!
//! # Determinism
//!
//! Counters are accumulated using integer atomics. Floating-point values such
//! as error rates are derived from integer counters when a snapshot is taken.
//! This avoids nondeterministic floating-point accumulation order.
//!
//! # Large-scale execution
//!
//! The collector does not store individual syndrome events, graph nodes, or
//! decoder operations. It stores bounded aggregate counters, making it suitable
//! for streaming and very large QEC workloads.
//!
//! # Threshold experiments
//!
//! The snapshot contains physical error rate, logical error rate, logical
//! failures, corrections, and decoder outcomes, allowing threshold experiments
//! to aggregate results across code distances and physical error rates.

use core::fmt;
use std::sync::atomic::{
    AtomicBool,
    AtomicU64,
    AtomicUsize,
    Ordering,
};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::resources::ResourceSnapshot;

#[cfg(feature = "std")]
use super::errors::QecResult;

/// Result type used by this module when the QEC error system is available.
#[cfg(feature = "std")]
pub type MetricsResult<T> = QecResult<T>;

/// Lightweight result type used internally by metric operations.
///
/// Metric recording is deliberately infallible for normal operations.
pub type MetricsRecordResult<T> = Result<T, MetricsError>;

/// Errors specific to metric configuration or aggregation.
///
/// Recording counters should normally never fail. These errors are reserved
/// for invalid configuration or arithmetic/aggregation conditions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetricsError {
    /// A configured counter capacity was zero.
    InvalidCapacity {
        name: &'static str,
    },

    /// An aggregation operation could not be represented safely.
    ArithmeticOverflow {
        field: &'static str,
    },

    /// Two metric configurations cannot be combined.
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

            Self::ArithmeticOverflow { field } => {
                write!(f, "metrics arithmetic overflow for {field}")
            }

            Self::IncompatibleConfiguration { reason } => {
                write!(f, "incompatible metrics configuration: {reason}")
            }
        }
    }
}

impl std::error::Error for MetricsError {}

/// Stable decoder identifier.
///
/// This intentionally does not depend on `errors.rs` so metrics can be used
/// independently by execution backends and distributed workers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecoderId {
    SurfaceCode,
    Mwpm,
    UnionFind,
    Custom,
}

impl DecoderId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceCode => "surface_code",
            Self::Mwpm => "mwpm",
            Self::UnionFind => "union_find",
            Self::Custom => "custom",
        }
    }
}

impl Default for DecoderId {
    fn default() -> Self {
        Self::Custom
    }
}

/// Execution backend represented in metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendKind {
    Cpu,
    ParallelCpu,
    Gpu,
    Accelerator,
    Distributed,
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
            Self::Custom => "custom",
        }
    }
}

impl Default for BackendKind {
    fn default() -> Self {
        Self::Cpu
    }
}

/// Configuration controlling metrics behavior.
///
/// Metrics are aggregate-only by default. This prevents unbounded memory
/// growth from turning observability into a resource-exhaustion vector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetricsConfig {
    /// Maximum number of custom counters.
    ///
    /// The built-in counters do not consume this capacity.
    pub max_custom_counters: usize,

    /// Whether worker/node identifiers should be represented in aggregated
    /// metrics.
    ///
    /// Disabled by default because per-worker cardinality can become
    /// unbounded in distributed systems.
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

/// Aggregate metrics snapshot.
///
/// This is an immutable point-in-time representation suitable for:
///
/// - telemetry;
/// - logs;
/// - checkpoints;
/// - threshold experiments;
/// - regression tests;
/// - distributed aggregation;
/// - dashboards.
///
/// All counters are integer based. Rates are derived deterministically.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricsSnapshot {
    /// Decoder identity.
    pub decoder: DecoderId,

    /// Execution backend.
    pub backend: BackendKind,

    /// Number of decode operations started.
    pub decode_operations: u64,

    /// Number of successful decoder operations.
    pub decoder_success: u64,

    /// Number of failed decoder operations.
    pub decoder_failure: u64,

    /// Number of explicitly cancelled operations.
    pub cancellation_count: u64,

    /// Number of correction operations produced.
    pub correction_count: u64,

    /// Number of syndrome/detection events processed.
    pub detection_event_count: u64,

    /// Number of physical errors observed/injected.
    pub physical_error_count: u64,

    /// Number of physical-error opportunities/trials.
    pub physical_error_opportunities: u64,

    /// Number of logical failures.
    pub logical_failure_count: u64,

    /// Number of logical-error opportunities/trials.
    pub logical_error_opportunities: u64,

    /// Number of matching operations.
    pub matching_count: u64,

    /// Number of decoder iterations.
    pub decoder_iterations: u64,

    /// Number of graph nodes processed.
    pub graph_nodes: u64,

    /// Number of graph edges processed.
    pub graph_edges: u64,

    /// Number of qubits involved.
    pub qubit_count: u64,

    /// Number of stabilizers involved.
    pub stabilizer_count: u64,

    /// Number of measurement rounds.
    pub measurement_rounds: u64,

    /// Number of workers observed.
    pub worker_count: u64,

    /// Total decoder latency.
    pub decoder_latency: Duration,

    /// Maximum observed decoder latency.
    pub max_decoder_latency: Duration,

    /// Minimum observed decoder latency.
    pub min_decoder_latency: Option<Duration>,

    /// Maximum memory observed.
    pub peak_memory: u64,

    /// Current memory reported by the resource layer.
    pub current_memory: u64,

    /// Wall-clock time represented by the collector.
    pub wall_time: Duration,

    /// Backend-reported compute time.
    pub compute_time: Duration,

    /// Physical error rate.
    ///
    /// Defined as:
    ///
    /// `physical_error_count / physical_error_opportunities`
    pub physical_error_rate: Option<f64>,

    /// Logical error rate.
    ///
    /// Defined as:
    ///
    /// `logical_failure_count / logical_error_opportunities`
    pub logical_error_rate: Option<f64>,

    /// Decoder success rate.
    pub decoder_success_rate: Option<f64>,

    /// Decoder failure rate.
    pub decoder_failure_rate: Option<f64>,

    /// Average decoder latency.
    pub average_decoder_latency: Option<Duration>,

    /// Whether the collector observed any logical failure.
    pub had_logical_failure: bool,

    /// Whether all started decoder operations have reached a terminal state.
    pub operations_balanced: bool,
}

impl MetricsSnapshot {
    /// Creates a zero-valued metrics snapshot.
    pub fn empty(decoder: DecoderId, backend: BackendKind) -> Self {
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
            peak_memory: 0,
            current_memory: 0,
            wall_time: Duration::ZERO,
            compute_time: Duration::ZERO,
            physical_error_rate: None,
            logical_error_rate: None,
            decoder_success_rate: None,
            decoder_failure_rate: None,
            average_decoder_latency: None,
            had_logical_failure: false,
            operations_balanced: true,
        }
    }

    /// Returns the logical failure probability/rate when enough trials exist.
    pub fn logical_error_rate_or_zero(&self) -> f64 {
        self.logical_error_rate.unwrap_or(0.0)
    }

    /// Returns the physical error probability/rate when enough trials exist.
    pub fn physical_error_rate_or_zero(&self) -> f64 {
        self.physical_error_rate.unwrap_or(0.0)
    }

    /// Returns whether the decoder succeeded for every completed operation.
    pub fn all_decodes_successful(&self) -> bool {
        self.decode_operations > 0
            && self.decoder_failure == 0
            && self.cancellation_count == 0
    }

    /// Returns the number of terminal decoder operations.
    pub fn terminal_operations(&self) -> u64 {
        self.decoder_success
            .saturating_add(self.decoder_failure)
            .saturating_add(self.cancellation_count)
    }
}

/// Metrics specifically useful for threshold experiments.
#[derive(Debug, Clone, PartialEq)]
pub struct ThresholdMetrics {
    /// Code distance used for the experiment.
    pub code_distance: u64,

    /// Physical error probability used by the noise model.
    pub physical_error_probability: f64,

    /// Number of trials.
    pub trials: u64,

    /// Number of logical failures.
    pub logical_failures: u64,

    /// Estimated logical error rate.
    pub logical_error_rate: Option<f64>,

    /// Number of decoder failures unrelated to logical failure.
    pub decoder_failures: u64,

    /// Total decoding latency.
    pub total_latency: Duration,

    /// Average decoding latency.
    pub average_latency: Option<Duration>,

    /// Maximum memory observed.
    pub peak_memory: u64,
}

impl ThresholdMetrics {
    /// Constructs threshold metrics from a general QEC metrics snapshot.
    pub fn from_snapshot(
        snapshot: &MetricsSnapshot,
        code_distance: u64,
        physical_error_probability: f64,
    ) -> Self {
        Self {
            code_distance,
            physical_error_probability,
            trials: snapshot.decode_operations,
            logical_failures: snapshot.logical_failure_count,
            logical_error_rate: snapshot.logical_error_rate,
            decoder_failures: snapshot.decoder_failure,
            total_latency: snapshot.decoder_latency,
            average_latency: snapshot.average_decoder_latency,
            peak_memory: snapshot.peak_memory,
        }
    }
}

/// Internal atomic metrics storage.
///
/// No floating-point values are stored here. This is deliberate: integer
/// counters guarantee reproducible aggregation regardless of worker ordering.
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

    /// `u64::MAX` means no latency has been recorded.
    min_decoder_latency_nanos: AtomicU64,

    peak_memory: AtomicU64,
    current_memory: AtomicU64,

    compute_time_nanos: AtomicU64,

    /// Set whenever a logical failure is recorded.
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

            had_logical_failure: AtomicBool::new(false),
        }
    }
}

/// Thread-safe QEC metrics collector.
///
/// It can be shared between decoder workers:
///
/// ```text
///                 MetricsCollector
///                       │
///          ┌────────────┼────────────┐
///          ▼            ▼            ▼
///       worker 1     worker 2     worker N
///          │            │            │
///          └────────────┼────────────┘
///                       ▼
///                  deterministic
///                    snapshot
/// ```
///
/// The collector itself does not own the decoder and therefore cannot
/// interfere with correctness.
#[derive(Debug)]
pub struct MetricsCollector {
    decoder: DecoderId,
    backend: BackendKind,
    config: MetricsConfig,
    counters: MetricsCounters,
    started: Instant,
}

impl MetricsCollector {
    /// Creates a new metrics collector.
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

    /// Creates a collector with the default configuration.
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

    // ---------------------------------------------------------------------
    // Decoder operations
    // ---------------------------------------------------------------------

    /// Records the beginning of a decoder operation.
    pub fn begin_decode(&self) {
        saturating_increment(&self.counters.decode_operations);
    }

    /// Records a successful decoder operation and its latency.
    pub fn record_success(&self, latency: Duration) {
        saturating_increment(&self.counters.decoder_success);
        self.record_latency(latency);
    }

    /// Records a failed decoder operation and its latency.
    pub fn record_failure(&self, latency: Duration) {
        saturating_increment(&self.counters.decoder_failure);
        self.record_latency(latency);
    }

    /// Records explicit cancellation.
    pub fn record_cancellation(&self, latency: Duration) {
        saturating_increment(&self.counters.cancellation_count);
        self.record_latency(latency);
    }

    /// Records an already completed decoder operation.
    ///
    /// Useful when a distributed worker reports results to a coordinator.
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

    // ---------------------------------------------------------------------
    // Error-correction metrics
    // ---------------------------------------------------------------------

    /// Records correction operations.
    pub fn record_corrections(&self, count: u64) {
        saturating_add(
            &self.counters.correction_count,
            count,
        );
    }

    /// Records syndrome/detection events.
    pub fn record_detection_events(&self, count: u64) {
        saturating_add(
            &self.counters.detection_event_count,
            count,
        );
    }

    /// Records physical errors.
    pub fn record_physical_errors(&self, count: u64) {
        saturating_add(
            &self.counters.physical_error_count,
            count,
        );
    }

    /// Records physical-error opportunities/trials.
    pub fn record_physical_error_opportunities(&self, count: u64) {
        saturating_add(
            &self.counters.physical_error_opportunities,
            count,
        );
    }

    /// Records a physical-error observation and its opportunity together.
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

    /// Records logical failures.
    pub fn record_logical_failures(&self, count: u64) {
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

    /// Records logical-error opportunities/trials.
    pub fn record_logical_error_opportunities(&self, count: u64) {
        saturating_add(
            &self.counters.logical_error_opportunities,
            count,
        );
    }

    /// Records a logical trial.
    ///
    /// This is the preferred method for threshold experiments because it
    /// cannot accidentally forget to increment the denominator.
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

    // ---------------------------------------------------------------------
    // Decoder internals
    // ---------------------------------------------------------------------

    pub fn record_matching(&self, count: u64) {
        saturating_add(
            &self.counters.matching_count,
            count,
        );
    }

    pub fn record_decoder_iterations(&self, count: u64) {
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

    pub fn record_qubits(&self, count: u64) {
        saturating_max(
            &self.counters.qubit_count,
            count,
        );
    }

    pub fn record_stabilizers(&self, count: u64) {
        saturating_max(
            &self.counters.stabilizer_count,
            count,
        );
    }

    pub fn record_measurement_rounds(&self, count: u64) {
        saturating_max(
            &self.counters.measurement_rounds,
            count,
        );
    }

    // ---------------------------------------------------------------------
    // Workers / parallelism
    // ---------------------------------------------------------------------

    /// Records the observed worker count.
    ///
    /// This is a high-water mark rather than an ever-growing event count.
    pub fn record_workers(&self, workers: usize) {
        if !self.config.track_worker_count {
            return;
        }

        let workers = workers as u64;

        saturating_max(
            &self.counters.worker_count,
            workers,
        );
    }

    // ---------------------------------------------------------------------
    // Latency
    // ---------------------------------------------------------------------

    /// Records decoder latency.
    pub fn record_latency(&self, latency: Duration) {
        let nanos = duration_to_nanos_saturating(latency);

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

    /// Starts a latency timer.
    pub fn start_timer(&self) -> MetricsTimer<'_> {
        MetricsTimer {
            collector: self,
            started: Instant::now(),
            stopped: false,
        }
    }

    // ---------------------------------------------------------------------
    // Memory / resource integration
    // ---------------------------------------------------------------------

    /// Records current memory usage.
    ///
    /// `current_memory` is a gauge while `peak_memory` is a high-water mark.
    pub fn record_memory(&self, current_memory: u64) {
        self.counters
            .current_memory
            .store(current_memory, Ordering::Release);

        saturating_max(
            &self.counters.peak_memory,
            current_memory,
        );
    }

    /// Integrates a resource-manager snapshot.
    ///
    /// This keeps resource accounting and metrics conceptually separate:
    /// resources enforce limits; metrics observe them.
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

        let compute_nanos =
            duration_to_nanos_saturating(snapshot.compute_time);

        saturating_max(
            &self.counters.compute_time_nanos,
            compute_nanos,
        );
    }

    // ---------------------------------------------------------------------
    // Snapshot
    // ---------------------------------------------------------------------

    /// Produces an immutable deterministic metrics snapshot.
    pub fn snapshot(&self) -> MetricsSnapshot {
        let decode_operations =
            load(&self.counters.decode_operations);

        let decoder_success =
            load(&self.counters.decoder_success);

        let decoder_failure =
            load(&self.counters.decoder_failure);

        let cancellation_count =
            load(&self.counters.cancellation_count);

        let physical_error_count =
            load(&self.counters.physical_error_count);

        let physical_error_opportunities =
            load(&self.counters.physical_error_opportunities);

        let logical_failure_count =
            load(&self.counters.logical_failure_count);

        let logical_error_opportunities =
            load(&self.counters.logical_error_opportunities);

        let decoder_latency_nanos =
            load(&self.counters.decoder_latency_nanos);

        let max_decoder_latency_nanos =
            load(&self.counters.max_decoder_latency_nanos);

        let min_decoder_latency_nanos =
            load(&self.counters.min_decoder_latency_nanos);

        let physical_error_rate =
            ratio(
                physical_error_count,
                physical_error_opportunities,
            );

        let logical_error_rate =
            ratio(
                logical_failure_count,
                logical_error_opportunities,
            );

        let decoder_success_rate =
            ratio(
                decoder_success,
                decode_operations,
            );

        let decoder_failure_rate =
            ratio(
                decoder_failure,
                decode_operations,
            );

        let terminal_operations =
            decoder_success
                .saturating_add(decoder_failure)
                .saturating_add(cancellation_count);

        let average_decoder_latency =
            if decode_operations > 0 {
                Some(Duration::from_nanos(
                    decoder_latency_nanos
                        / decode_operations,
                ))
            } else {
                None
            };

        let min_decoder_latency =
            if min_decoder_latency_nanos == u64::MAX {
                None
            } else {
                Some(Duration::from_nanos(
                    min_decoder_latency_nanos,
                ))
            };

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

            physical_error_count,
            physical_error_opportunities,

            logical_failure_count,
            logical_error_opportunities,

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

            decoder_latency: Duration::from_nanos(
                decoder_latency_nanos,
            ),

            max_decoder_latency: Duration::from_nanos(
                max_decoder_latency_nanos,
            ),

            min_decoder_latency,

            peak_memory: load(
                &self.counters.peak_memory,
            ),

            current_memory: load(
                &self.counters.current_memory,
            ),

            wall_time: self.started.elapsed(),

            compute_time: Duration::from_nanos(
                load(&self.counters.compute_time_nanos),
            ),

            physical_error_rate,
            logical_error_rate,

            decoder_success_rate,
            decoder_failure_rate,

            average_decoder_latency,

            had_logical_failure: self
                .counters
                .had_logical_failure
                .load(Ordering::Acquire),

            operations_balanced:
                decode_operations == terminal_operations,
        }
    }

    /// Resets all aggregate counters.
    ///
    /// This is intended for starting a new measurement window. It does not
    /// alter decoder configuration.
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
            .had_logical_failure
            .store(false, Ordering::Release);
    }
}

/// RAII decoder-latency timer.
///
/// Dropping an active timer records the elapsed latency automatically.
///
/// This prevents early-return/error paths from silently losing latency data.
#[derive(Debug)]
pub struct MetricsTimer<'a> {
    collector: &'a MetricsCollector,
    started: Instant,
    stopped: bool,
}

impl<'a> MetricsTimer<'a> {
    /// Stops the timer and records its latency.
    pub fn stop(mut self) -> Duration {
        let elapsed = self.started.elapsed();

        if !self.stopped {
            self.collector.record_latency(elapsed);
            self.stopped = true;
        }

        elapsed
    }

    /// Returns elapsed time without recording it.
    pub fn elapsed(&self) -> Duration {
        self.started.elapsed()
    }
}

impl Drop for MetricsTimer<'_> {
    fn drop(&mut self) {
        if !self.stopped {
            let elapsed = self.started.elapsed();
            self.collector.record_latency(elapsed);
            self.stopped = true;
        }
    }
}

/// Shared metrics collector suitable for worker pools.
pub type SharedMetrics = Arc<MetricsCollector>;

/// Creates a shared collector.
pub fn shared_metrics(
    decoder: DecoderId,
    backend: BackendKind,
) -> SharedMetrics {
    Arc::new(MetricsCollector::standard(
        decoder,
        backend,
    ))
}

// -------------------------------------------------------------------------
// Deterministic integer helpers
// -------------------------------------------------------------------------

fn load(value: &AtomicU64) -> u64 {
    value.load(Ordering::Acquire)
}

fn saturating_increment(value: &AtomicU64) {
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
            if candidate > current {
                Some(candidate)
            } else {
                Some(current)
            }
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
            if candidate < current {
                Some(candidate)
            } else {
                Some(current)
            }
        },
    );
}

/// Computes a rate without introducing floating-point accumulation.
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

/// Converts Duration to nanoseconds without overflowing u64.
fn duration_to_nanos_saturating(
    duration: Duration,
) -> u64 {
    duration
        .as_nanos()
        .min(u64::MAX as u128) as u64
}

// -------------------------------------------------------------------------
// Tests
// -------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_collector_starts_empty() {
        let metrics = MetricsCollector::standard(
            DecoderId::Mwpm,
            BackendKind::Cpu,
        );

        let snapshot = metrics.snapshot();

        assert_eq!(snapshot.decoder, DecoderId::Mwpm);
        assert_eq!(snapshot.backend, BackendKind::Cpu);

        assert_eq!(snapshot.decode_operations, 0);
        assert_eq!(snapshot.decoder_success, 0);
        assert_eq!(snapshot.decoder_failure, 0);

        assert_eq!(
            snapshot.physical_error_rate,
            None
        );

        assert_eq!(
            snapshot.logical_error_rate,
            None
        );
    }

    #[test]
    fn physical_error_rate_is_computed_from_integer_counters() {
        let metrics = MetricsCollector::standard(
            DecoderId::Mwpm,
            BackendKind::Cpu,
        );

        metrics.record_physical_error_opportunities(100);
        metrics.record_physical_errors(10);

        let snapshot = metrics.snapshot();

        assert_eq!(
            snapshot.physical_error_rate,
            Some(0.10)
        );
    }

    #[test]
    fn logical_error_rate_is_computed_correctly() {
        let metrics = MetricsCollector::standard(
            DecoderId::Mwpm,
            BackendKind::Cpu,
        );

        for _ in 0..100 {
            metrics.record_logical_trial(false);
        }

        for _ in 0..5 {
            metrics.record_logical_trial(true);
        }

        let snapshot = metrics.snapshot();

        assert_eq!(
            snapshot.logical_error_opportunities,
            105
        );

        assert_eq!(
            snapshot.logical_failure_count,
            5
        );

        assert!(
            (snapshot.logical_error_rate.unwrap()
                - (5.0 / 105.0))
                .abs()
                < f64::EPSILON
        );

        assert!(
            snapshot.had_logical_failure
        );
    }

    #[test]
    fn decoder_outcomes_are_balanced() {
        let metrics = MetricsCollector::standard(
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

        let snapshot = metrics.snapshot();

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
        let metrics = MetricsCollector::standard(
            DecoderId::Mwpm,
            BackendKind::Cpu,
        );

        metrics.record_success(
            Duration::from_nanos(100),
        );

        metrics.record_success(
            Duration::from_nanos(300),
        );

        let snapshot = metrics.snapshot();

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
    fn memory_peak_is_high_water_mark() {
        let metrics = MetricsCollector::standard(
            DecoderId::Mwpm,
            BackendKind::Cpu,
        );

        metrics.record_memory(1024);
        metrics.record_memory(4096);
        metrics.record_memory(2048);

        let snapshot = metrics.snapshot();

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
    fn resource_snapshot_integrates_without_ownership() {
        let metrics = MetricsCollector::standard(
            DecoderId::Mwpm,
            BackendKind::ParallelCpu,
        );

        let resources = ResourceSnapshot {
            allocated_bytes: 10_000,
            peak_bytes: 20_000,
            syndrome_events: 100,
            graph_nodes: 200,
            graph_edges: 400,
            decoder_iterations: 50,
            parallel_workers: 8,
            wall_time: Duration::from_secs(1),
            compute_time: Duration::from_millis(500),
        };

        metrics.record_resource_snapshot(resources);

        let snapshot = metrics.snapshot();

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
    }

    #[test]
    fn logical_trial_prevents_denominator_omission() {
        let metrics = MetricsCollector::standard(
            DecoderId::UnionFind,
            BackendKind::Cpu,
        );

        metrics.record_logical_trial(false);
        metrics.record_logical_trial(true);

        let snapshot = metrics.snapshot();

        assert_eq!(
            snapshot.logical_error_opportunities,
            2
        );

        assert_eq!(
            snapshot.logical_failure_count,
            1
        );

        assert_eq!(
            snapshot.logical_error_rate,
            Some(0.5)
        );
    }

    #[test]
    fn timer_records_latency_on_drop() {
        let metrics = MetricsCollector::standard(
            DecoderId::Mwpm,
            BackendKind::Cpu,
        );

        {
            let _timer = metrics.start_timer();

            std::thread::sleep(
                Duration::from_millis(1),
            );
        }

        let snapshot = metrics.snapshot();

        assert!(
            snapshot.decoder_latency
                >= Duration::from_millis(1)
        );
    }

    #[test]
    fn timer_can_be_stopped_explicitly() {
        let metrics = MetricsCollector::standard(
            DecoderId::Mwpm,
            BackendKind::Cpu,
        );

        let timer = metrics.start_timer();
        let elapsed = timer.stop();

        let snapshot = metrics.snapshot();

        assert!(
            snapshot.decoder_latency >= elapsed
        );
    }

    #[test]
    fn reset_clears_measurement_window() {
        let metrics = MetricsCollector::standard(
            DecoderId::Mwpm,
            BackendKind::Cpu,
        );

        metrics.begin_decode();
        metrics.record_success(
            Duration::from_millis(1),
        );

        metrics.record_logical_trial(true);
        metrics.record_memory(4096);

        metrics.reset();

        let snapshot = metrics.snapshot();

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

        assert!(
            snapshot.logical_error_rate.is_none()
        );
    }

    #[test]
    fn threshold_metrics_are_constructed() {
        let metrics = MetricsCollector::standard(
            DecoderId::Mwpm,
            BackendKind::Cpu,
        );

        metrics.record_outcome(
            true,
            false,
            Duration::from_millis(2),
        );

        metrics.record_logical_trial(true);

        let snapshot = metrics.snapshot();

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
            1
        );

        assert_eq!(
            threshold.logical_failures,
            1
        );

        assert_eq!(
            threshold.physical_error_probability,
            0.01
        );
    }

    #[test]
    fn configuration_rejects_zero_capacity() {
        let config = MetricsConfig {
            max_custom_counters: 0,
            track_worker_count: true,
        };

        assert!(
            config.validate().is_err()
        );
    }

    #[test]
    fn deterministic_snapshot_contains_no_event_history() {
        let metrics = MetricsCollector::standard(
            DecoderId::Mwpm,
            BackendKind::Cpu,
        );

        metrics.record_corrections(10);
        metrics.record_matching(4);
        metrics.record_graph_nodes(20);
        metrics.record_graph_edges(30);

        let a = metrics.snapshot();
        let b = metrics.snapshot();

        assert_eq!(
            a.correction_count,
            b.correction_count
        );

        assert_eq!(
            a.matching_count,
            b.matching_count
        );

        assert_eq!(
            a.graph_nodes,
            b.graph_nodes
        );

        assert_eq!(
            a.graph_edges,
            b.graph_edges
        );
    }

    #[test]
    fn shared_metrics_can_be_used_by_multiple_workers() {
        let metrics = shared_metrics(
            DecoderId::Mwpm,
            BackendKind::ParallelCpu,
        );

        let mut handles = Vec::new();

        for _ in 0..4 {
            let metrics = Arc::clone(&metrics);

            handles.push(
                std::thread::spawn(move || {
                    for _ in 0..1000 {
                        metrics.record_detection_events(1);
                        metrics.record_matching(1);
                    }
                }),
            );
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let snapshot = metrics.snapshot();

        assert_eq!(
            snapshot.detection_event_count,
            4000
        );

        assert_eq!(
            snapshot.matching_count,
            4000
        );
    }
}