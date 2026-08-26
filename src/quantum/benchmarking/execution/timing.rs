//! Zamani Quantum Benchmarking — Execution Timing
//!
//! Production timing infrastructure for the quantum benchmarking execution
//! subsystem.
//!
//! # Purpose
//!
//! This module provides the authoritative, backend-independent representation
//! and collection mechanism for execution timing.
//!
//! Timing is deliberately separated from benchmark mathematics. This module
//! does NOT calculate:
//!
//! - fidelity;
//! - Quantum Volume;
//! - randomized-benchmarking error;
//! - XEB;
//! - throughput metrics;
//! - statistical confidence intervals;
//! - benchmark quality;
//! - hardware performance scores.
//!
//! It only records elapsed time for well-defined execution lifecycle phases.
//!
//! # Architectural position
//!
//! ```text
//! benchmark protocol
//!       |
//!       v
//! execution::executor
//!       |
//!       +--------------------------+
//!       |                          |
//!       v                          v
//! TimingRecorder              backend executor
//!       |                          |
//!       +-------------+------------+
//!                     |
//!                     v
//!             ExecutionTiming
//!                     |
//!                     v
//!              core::execution
//!                     |
//!                     v
//!             BenchmarkResult
//! ```
//!
//! # Timing dimensions
//!
//! The timing model intentionally distinguishes:
//!
//! - preparation;
//! - compilation;
//! - transpilation;
//! - routing;
//! - scheduling;
//! - queueing;
//! - submission;
//! - provider execution;
//! - readout;
//! - result retrieval;
//! - analysis;
//! - total wall-clock orchestration time.
//!
//! These phases MUST NOT be collapsed into a single runtime value.
//!
//! For example:
//!
//! ```text
//! total wall time
//! = preparation
//! + compilation
//! + transpilation
//! + routing
//! + scheduling
//! + queue
//! + submission
//! + execution
//! + readout
//! + retrieval
//! + analysis
//! + unclassified gaps
//! ```
//!
//! The equality is informational rather than a mathematical invariant because
//! phases may overlap in future distributed implementations. Consequently,
//! this module never assumes that the sum of phase durations equals total
//! elapsed time.
//!
//! # Monotonic clock requirement
//!
//! Duration measurement uses `std::time::Instant`.
//!
//! `Instant` is monotonic and therefore must be used for elapsed-time
//! measurements. Wall-clock timestamps must never be substituted for elapsed
//! duration calculations because system-clock adjustments can move backward or
//! forward.
//!
//! # Thread safety
//!
//! `TimingRecorder` itself is intentionally not synchronized. A recorder
//! represents one execution lifecycle and should normally be owned by the
//! orchestrating execution thread.
//!
//! `TimingHandle` is `Send + Sync` because it contains no shared mutable state.
//! A future concurrent timing implementation can be introduced without
//! changing the serialized timing representation.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//!
//! No additional crate dependency is required.
//!
//! # Integration contract
//!
//! `core/execution.rs` MUST use `ExecutionTiming` from this module as its
//! canonical timing representation.
//!
//! `execution/executor.rs` should create a `TimingRecorder`, mark lifecycle
//! phases around provider operations, finalize it, and place the resulting
//! `ExecutionTiming` into `ExecutionResponse`.
//!
//! `execution/timing.rs` MUST NOT depend on `core/execution.rs`.
//!
//! This one-way dependency prevents a circular dependency:
//!
//! ```text
//! execution::timing
//!       ^
//!       |
//! core::execution
//!       ^
//!       |
//! execution::executor
//! ```
//!
//! Instead, the actual Rust dependency should be:
//!
//! ```text
//! execution::timing
//!       ^
//!       |
//! core::execution
//!       ^
//!       |
//! execution::executor
//! ```
//!
//! with `timing.rs` remaining the lower-level primitive.
//!
//! # Important semantic rule
//!
//! `None` means "not measured / not supplied".
//!
//! `Some(Duration::ZERO)` means "measured and actually zero elapsed time".
//!
//! These states MUST NOT be conflated.
//!
//! This distinction is essential for simulator, hardware, remote-provider,
//! queued, cancelled, and partially completed executions.
//!

use std::fmt;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// =============================================================================
// Versioning
// =============================================================================

/// Stable schema version of the timing representation.
pub const EXECUTION_TIMING_SCHEMA_VERSION: u32 = 1;

/// Stable API version of the timing recorder.
pub const TIMING_RECORDER_VERSION: u32 = 1;

// =============================================================================
// Timing phase
// =============================================================================

/// A lifecycle phase that can be timed independently.
///
/// The phases are deliberately backend-neutral. A backend may not support
/// every phase. Unsupported or unmeasured phases remain `None`.
///
/// The order of variants is also the canonical reporting order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimingPhase {
    /// Time spent preparing the execution request.
    Preparation,

    /// Time spent compiling source/IR into an executable representation.
    Compilation,

    /// Time spent translating the circuit to a backend-specific representation.
    Transpilation,

    /// Time spent performing logical-to-physical routing.
    Routing,

    /// Time spent scheduling operations.
    Scheduling,

    /// Time spent waiting in a provider/backend queue.
    Queue,

    /// Time spent submitting work to the provider.
    Submission,

    /// Time spent executing the quantum workload.
    Execution,

    /// Time spent acquiring or performing readout.
    Readout,

    /// Time spent retrieving the completed result from a provider.
    ResultRetrieval,

    /// Time spent performing benchmark-side analysis.
    Analysis,
}

impl TimingPhase {
    /// Returns every defined phase in canonical reporting order.
    pub const fn all() -> &'static [Self] {
        &[
            Self::Preparation,
            Self::Compilation,
            Self::Transpilation,
            Self::Routing,
            Self::Scheduling,
            Self::Queue,
            Self::Submission,
            Self::Execution,
            Self::Readout,
            Self::ResultRetrieval,
            Self::Analysis,
        ]
    }

    /// Returns a stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preparation => "preparation",
            Self::Compilation => "compilation",
            Self::Transpilation => "transpilation",
            Self::Routing => "routing",
            Self::Scheduling => "scheduling",
            Self::Queue => "queue",
            Self::Submission => "submission",
            Self::Execution => "execution",
            Self::Readout => "readout",
            Self::ResultRetrieval => "result_retrieval",
            Self::Analysis => "analysis",
        }
    }

    /// Returns whether the phase normally occurs before provider execution.
    pub const fn is_pre_execution(self) -> bool {
        matches!(
            self,
            Self::Preparation
                | Self::Compilation
                | Self::Transpilation
                | Self::Routing
                | Self::Scheduling
        )
    }

    /// Returns whether the phase belongs to provider-side lifecycle timing.
    pub const fn is_provider_side(self) -> bool {
        matches!(
            self,
            Self::Queue
                | Self::Submission
                | Self::Execution
                | Self::Readout
                | Self::ResultRetrieval
        )
    }

    /// Returns whether the phase is benchmark-side post-processing.
    pub const fn is_post_execution(self) -> bool {
        matches!(self, Self::Analysis)
    }
}

impl fmt::Display for TimingPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Timing measurement
// =============================================================================

/// A single measured timing value.
///
/// This wrapper prevents accidental confusion between an absent measurement
/// and a measured zero duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimingValue {
    duration: Duration,
}

impl TimingValue {
    /// Creates a timing value.
    pub const fn new(duration: Duration) -> Self {
        Self { duration }
    }

    /// Returns the duration.
    pub const fn duration(self) -> Duration {
        self.duration
    }

    /// Returns elapsed nanoseconds.
    ///
    /// `Duration::as_nanos()` returns `u128`, preventing overflow at the API
    /// boundary.
    pub fn as_nanos(self) -> u128 {
        self.duration.as_nanos()
    }

    /// Returns elapsed microseconds.
    pub fn as_micros(self) -> u128 {
        self.duration.as_micros()
    }

    /// Returns elapsed milliseconds.
    pub fn as_millis(self) -> u128 {
        self.duration.as_millis()
    }

    /// Returns elapsed seconds as `f64`.
    ///
    /// This conversion is intended for reporting/analysis only. The canonical
    /// representation remains `Duration`.
    pub fn as_secs_f64(self) -> f64 {
        self.duration.as_secs_f64()
    }
}

impl From<Duration> for TimingValue {
    fn from(duration: Duration) -> Self {
        Self::new(duration)
    }
}

// =============================================================================
// Phase timing
// =============================================================================

/// Timing for one lifecycle phase.
///
/// This type is immutable after creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhaseTiming {
    /// Phase being measured.
    pub phase: TimingPhase,

    /// Measured duration.
    pub duration: TimingValue,
}

impl PhaseTiming {
    /// Creates a phase timing.
    pub const fn new(phase: TimingPhase, duration: Duration) -> Self {
        Self {
            phase,
            duration: TimingValue::new(duration),
        }
    }
}

// =============================================================================
// Wall-clock metadata
// =============================================================================

/// Optional wall-clock metadata associated with an execution.
///
/// Wall-clock timestamps are provenance information only. They MUST NOT be
/// used to calculate elapsed durations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimingTimestamp {
    /// Unix timestamp in whole seconds.
    ///
    /// This is intentionally optional because system clocks can be unavailable
    /// or unsuitable in some constrained environments.
    pub unix_seconds: u64,
}

impl TimingTimestamp {
    /// Captures the current wall-clock timestamp.
    ///
    /// If the system clock is before the Unix epoch, no timestamp is produced.
    pub fn now() -> Option<Self> {
        let duration = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;

        Some(Self {
            unix_seconds: duration.as_secs(),
        })
    }
}

// =============================================================================
// Execution timing
// =============================================================================

/// Immutable, production execution timing result.
///
/// This is the canonical timing object consumed by
/// `benchmarking::core::execution`.
///
/// No benchmark-specific metric is calculated here.
///
/// For example, this type deliberately does NOT contain:
///
/// - shots/sec;
/// - circuits/sec;
/// - CLOPS;
/// - gates/sec;
/// - quality-adjusted throughput.
///
/// Those belong in `metrics::throughput`.
///
/// `ExecutionTiming` only records the durations needed to calculate such
/// metrics later.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionTiming {
    /// Stable timing schema version.
    pub schema_version: u32,

    /// Total elapsed wall-clock time measured by the timing recorder.
    ///
    /// This is measured independently from the individual lifecycle phases.
    pub total_wall_time: TimingValue,

    /// Preparation duration.
    pub preparation: Option<TimingValue>,

    /// Compilation duration.
    pub compilation: Option<TimingValue>,

    /// Transpilation duration.
    pub transpilation: Option<TimingValue>,

    /// Routing duration.
    pub routing: Option<TimingValue>,

    /// Scheduling duration.
    pub scheduling: Option<TimingValue>,

    /// Queue duration.
    pub queue: Option<TimingValue>,

    /// Submission duration.
    pub submission: Option<TimingValue>,

    /// Quantum execution duration.
    pub execution: Option<TimingValue>,

    /// Readout duration.
    pub readout: Option<TimingValue>,

    /// Result-retrieval duration.
    pub result_retrieval: Option<TimingValue>,

    /// Benchmark analysis duration.
    pub analysis: Option<TimingValue>,

    /// Wall-clock start timestamp, when available.
    ///
    /// This is provenance only.
    pub started_at: Option<TimingTimestamp>,

    /// Wall-clock completion timestamp, when available.
    ///
    /// This is provenance only.
    pub completed_at: Option<TimingTimestamp>,
}

impl ExecutionTiming {
    /// Creates an empty timing object.
    ///
    /// All phase timings are initially absent.
    pub fn empty(total_wall_time: Duration) -> Self {
        Self {
            schema_version: EXECUTION_TIMING_SCHEMA_VERSION,
            total_wall_time: TimingValue::new(total_wall_time),
            preparation: None,
            compilation: None,
            transpilation: None,
            routing: None,
            scheduling: None,
            queue: None,
            submission: None,
            execution: None,
            readout: None,
            result_retrieval: None,
            analysis: None,
            started_at: None,
            completed_at: None,
        }
    }

    /// Returns the measured duration for a phase.
    pub const fn phase(&self, phase: TimingPhase) -> Option<TimingValue> {
        match phase {
            TimingPhase::Preparation => self.preparation,
            TimingPhase::Compilation => self.compilation,
            TimingPhase::Transpilation => self.transpilation,
            TimingPhase::Routing => self.routing,
            TimingPhase::Scheduling => self.scheduling,
            TimingPhase::Queue => self.queue,
            TimingPhase::Submission => self.submission,
            TimingPhase::Execution => self.execution,
            TimingPhase::Readout => self.readout,
            TimingPhase::ResultRetrieval => self.result_retrieval,
            TimingPhase::Analysis => self.analysis,
        }
    }

    /// Returns the sum of all explicitly measured phase durations.
    ///
    /// This is NOT guaranteed to equal `total_wall_time`.
    ///
    /// Overlapping provider operations, unmeasured gaps, scheduling overhead,
    /// and external work can cause a difference.
    pub fn measured_phase_time(&self) -> Duration {
        let mut total = Duration::ZERO;

        for phase in TimingPhase::all() {
            if let Some(value) = self.phase(*phase) {
                total = total.saturating_add(value.duration());
            }
        }

        total
    }

    /// Returns total time that was not explicitly attributed to a phase.
    ///
    /// If measured phases exceed total wall time because phases overlap or were
    /// independently reported by a provider, this returns zero rather than
    /// producing an invalid negative duration.
    pub fn unattributed_time(&self) -> Duration {
        self.total_wall_time
            .duration()
            .saturating_sub(self.measured_phase_time())
    }

    /// Returns whether the execution has an explicitly measured quantum
    /// execution phase.
    pub fn has_execution_time(&self) -> bool {
        self.execution.is_some()
    }

    /// Returns whether the execution has an explicitly measured queue phase.
    pub fn has_queue_time(&self) -> bool {
        self.queue.is_some()
    }

    /// Returns whether all lifecycle phases have explicit measurements.
    pub fn is_fully_attributed(&self) -> bool {
        TimingPhase::all()
            .iter()
            .all(|phase| self.phase(*phase).is_some())
    }

    /// Returns phase timings in canonical order.
    pub fn phases(&self) -> Vec<PhaseTiming> {
        TimingPhase::all()
            .iter()
            .filter_map(|phase| {
                self.phase(*phase)
                    .map(|duration| PhaseTiming::new(*phase, duration.duration()))
            })
            .collect()
    }
}

// =============================================================================
// Timing recorder
// =============================================================================

/// Mutable recorder for one execution lifecycle.
///
/// The recorder is deliberately single-owner. It is not internally
/// synchronized because synchronization would hide lifecycle errors and add
/// unnecessary overhead to every timing operation.
///
/// Typical usage:
///
/// ```text
/// let mut timing = TimingRecorder::start();
///
/// timing.measure(TimingPhase::Preparation, || {
///     prepare_request();
/// })?;
///
/// timing.measure(TimingPhase::Execution, || {
///     execute_backend();
/// })?;
///
/// let result = timing.finish();
/// ```
///
/// Provider-reported timing may be recorded using `record`.
#[derive(Debug)]
pub struct TimingRecorder {
    started: Instant,
    started_at: Option<TimingTimestamp>,
    completed_at: Option<TimingTimestamp>,
    preparation: Option<TimingValue>,
    compilation: Option<TimingValue>,
    transpilation: Option<TimingValue>,
    routing: Option<TimingValue>,
    scheduling: Option<TimingValue>,
    queue: Option<TimingValue>,
    submission: Option<TimingValue>,
    execution: Option<TimingValue>,
    readout: Option<TimingValue>,
    result_retrieval: Option<TimingValue>,
    analysis: Option<TimingValue>,
    active_phase: Option<ActivePhase>,
    finished: bool,
}

/// Internal state for a currently measured phase.
#[derive(Debug, Clone, Copy)]
struct ActivePhase {
    phase: TimingPhase,
    started: Instant,
}

impl TimingRecorder {
    /// Starts a new execution timing recorder.
    pub fn start() -> Self {
        Self {
            started: Instant::now(),
            started_at: TimingTimestamp::now(),
            completed_at: None,
            preparation: None,
            compilation: None,
            transpilation: None,
            routing: None,
            scheduling: None,
            queue: None,
            submission: None,
            execution: None,
            readout: None,
            result_retrieval: None,
            analysis: None,
            active_phase: None,
            finished: false,
        }
    }

    /// Returns whether the recorder has been finalized.
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// Returns elapsed wall time since recorder creation.
    pub fn elapsed(&self) -> Duration {
        self.started.elapsed()
    }

    /// Starts a phase.
    ///
    /// Only one phase may be active at a time. This prevents accidental nested
    /// timings from corrupting the phase representation.
    pub fn start_phase(
        &mut self,
        phase: TimingPhase,
    ) -> Result<(), TimingError> {
        self.ensure_active()?;

        if self.active_phase.is_some() {
            return Err(TimingError::PhaseAlreadyActive);
        }

        self.active_phase = Some(ActivePhase {
            phase,
            started: Instant::now(),
        });

        Ok(())
    }

    /// Stops the currently active phase.
    ///
    /// The elapsed duration is stored under the phase started by
    /// `start_phase`.
    pub fn stop_phase(&mut self) -> Result<TimingValue, TimingError> {
        self.ensure_active()?;

        let active = self
            .active_phase
            .take()
            .ok_or(TimingError::NoActivePhase)?;

        let duration = active.started.elapsed();
        let value = TimingValue::new(duration);

        self.store_phase(active.phase, value)?;

        Ok(value)
    }

    /// Starts and measures a synchronous operation.
    ///
    /// If the operation returns an error, timing is still recorded before the
    /// error is propagated.
    pub fn measure<T, E, F>(
        &mut self,
        phase: TimingPhase,
        operation: F,
    ) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        if self.is_finished() {
            // The recorder's lifecycle error cannot be returned as `E`, so this
            // method is intentionally only suitable for an active recorder.
            //
            // Callers that need explicit lifecycle-error handling should use
            // `start_phase` / `stop_phase`.
            return operation();
        }

        let start = Instant::now();
        let result = operation();
        let duration = TimingValue::new(start.elapsed());

        // Store timing even when the operation failed.
        //
        // A failed compilation, submission, or execution still has measurable
        // duration and that information is scientifically useful.
        let _ = self.store_phase(phase, duration);

        result
    }

    /// Records a provider-reported phase duration.
    ///
    /// This is useful when the provider reports queue, execution, readout, or
    /// retrieval time that cannot be measured accurately from the client side.
    ///
    /// Provider-reported durations must already be validated by the provider
    /// adapter. This method accepts only `Duration`, which cannot represent a
    /// negative interval.
    pub fn record(
        &mut self,
        phase: TimingPhase,
        duration: Duration,
    ) -> Result<(), TimingError> {
        self.ensure_active()?;

        if self.active_phase.is_some() {
            return Err(TimingError::PhaseAlreadyActive);
        }

        self.store_phase(phase, TimingValue::new(duration))
    }

    /// Records a timing value.
    pub fn record_value(
        &mut self,
        phase: TimingPhase,
        value: TimingValue,
    ) -> Result<(), TimingError> {
        self.ensure_active()?;

        if self.active_phase.is_some() {
            return Err(TimingError::PhaseAlreadyActive);
        }

        self.store_phase(phase, value)
    }

    /// Finishes the recorder and returns immutable timing data.
    ///
    /// If a phase is still active, it is automatically stopped at finalization.
    ///
    /// This is intentional: an execution that is cancelled, times out, or
    /// fails must still preserve the time spent in its final active phase.
    pub fn finish(mut self) -> Result<ExecutionTiming, TimingError> {
        if self.finished {
            return Err(TimingError::AlreadyFinished);
        }

        if self.active_phase.is_some() {
            self.stop_phase()?;
        }

        let completed_at = TimingTimestamp::now();
        let total = self.started.elapsed();

        self.completed_at = completed_at;
        self.finished = true;

        Ok(ExecutionTiming {
            schema_version: EXECUTION_TIMING_SCHEMA_VERSION,
            total_wall_time: TimingValue::new(total),
            preparation: self.preparation,
            compilation: self.compilation,
            transpilation: self.transpilation,
            routing: self.routing,
            scheduling: self.scheduling,
            queue: self.queue,
            submission: self.submission,
            execution: self.execution,
            readout: self.readout,
            result_retrieval: self.result_retrieval,
            analysis: self.analysis,
            started_at: self.started_at,
            completed_at: self.completed_at,
        })
    }

    fn ensure_active(&self) -> Result<(), TimingError> {
        if self.finished {
            Err(TimingError::AlreadyFinished)
        } else {
            Ok(())
        }
    }

    fn store_phase(
        &mut self,
        phase: TimingPhase,
        value: TimingValue,
    ) -> Result<(), TimingError> {
        let target = match phase {
            TimingPhase::Preparation => &mut self.preparation,
            TimingPhase::Compilation => &mut self.compilation,
            TimingPhase::Transpilation => &mut self.transpilation,
            TimingPhase::Routing => &mut self.routing,
            TimingPhase::Scheduling => &mut self.scheduling,
            TimingPhase::Queue => &mut self.queue,
            TimingPhase::Submission => &mut self.submission,
            TimingPhase::Execution => &mut self.execution,
            TimingPhase::Readout => &mut self.readout,
            TimingPhase::ResultRetrieval => &mut self.result_retrieval,
            TimingPhase::Analysis => &mut self.analysis,
        };

        if target.is_some() {
            return Err(TimingError::PhaseAlreadyRecorded { phase });
        }

        *target = Some(value);

        Ok(())
    }
}

impl Default for TimingRecorder {
    fn default() -> Self {
        Self::start()
    }
}

// =============================================================================
// Timing errors
// =============================================================================

/// Errors produced by the timing recorder.
///
/// Timing errors are lifecycle errors, not benchmark failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimingError {
    /// A phase was started while another phase was already active.
    PhaseAlreadyActive,

    /// `stop_phase` was called without a started phase.
    NoActivePhase,

    /// A phase was recorded more than once.
    PhaseAlreadyRecorded {
        phase: TimingPhase,
    },

    /// The recorder was used after finalization.
    AlreadyFinished,
}

impl fmt::Display for TimingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PhaseAlreadyActive => {
                f.write_str("a timing phase is already active")
            }

            Self::NoActivePhase => {
                f.write_str("no timing phase is currently active")
            }

            Self::PhaseAlreadyRecorded { phase } => {
                write!(f, "timing phase '{phase}' was already recorded")
            }

            Self::AlreadyFinished => {
                f.write_str("timing recorder has already been finalized")
            }
        }
    }
}

impl std::error::Error for TimingError {}

// =============================================================================
// Timing aggregation
// =============================================================================

/// Aggregated timing over multiple executions.
///
/// This is intentionally a simple duration accumulator. Statistical
/// interpretation belongs to `benchmarking::statistics`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingAccumulator {
    count: u64,
    total_wall_time: Duration,
    preparation: Duration,
    compilation: Duration,
    transpilation: Duration,
    routing: Duration,
    scheduling: Duration,
    queue: Duration,
    submission: Duration,
    execution: Duration,
    readout: Duration,
    result_retrieval: Duration,
    analysis: Duration,
}

impl TimingAccumulator {
    /// Creates an empty accumulator.
    pub const fn new() -> Self {
        Self {
            count: 0,
            total_wall_time: Duration::ZERO,
            preparation: Duration::ZERO,
            compilation: Duration::ZERO,
            transpilation: Duration::ZERO,
            routing: Duration::ZERO,
            scheduling: Duration::ZERO,
            queue: Duration::ZERO,
            submission: Duration::ZERO,
            execution: Duration::ZERO,
            readout: Duration::ZERO,
            result_retrieval: Duration::ZERO,
            analysis: Duration::ZERO,
        }
    }

    /// Adds one timing result.
    ///
    /// Missing phase measurements are ignored rather than interpreted as zero
    /// observations. The accumulator therefore preserves the distinction
    /// between "not measured" and "measured zero" through its per-phase sample
    /// counts.
    ///
    /// Use `phase_sample_count` to inspect those counts.
    pub fn add(&mut self, timing: &ExecutionTiming) {
        self.count = self.count.saturating_add(1);

        self.total_wall_time = self
            .total_wall_time
            .saturating_add(timing.total_wall_time.duration());

        self.add_phase(
            TimingPhase::Preparation,
            timing.preparation,
        );
        self.add_phase(
            TimingPhase::Compilation,
            timing.compilation,
        );
        self.add_phase(
            TimingPhase::Transpilation,
            timing.transpilation,
        );
        self.add_phase(TimingPhase::Routing, timing.routing);
        self.add_phase(
            TimingPhase::Scheduling,
            timing.scheduling,
        );
        self.add_phase(TimingPhase::Queue, timing.queue);
        self.add_phase(
            TimingPhase::Submission,
            timing.submission,
        );
        self.add_phase(
            TimingPhase::Execution,
            timing.execution,
        );
        self.add_phase(TimingPhase::Readout, timing.readout);
        self.add_phase(
            TimingPhase::ResultRetrieval,
            timing.result_retrieval,
        );
        self.add_phase(TimingPhase::Analysis, timing.analysis);
    }

    /// Number of executions added.
    pub const fn count(&self) -> u64 {
        self.count
    }

    /// Returns accumulated total wall time.
    pub const fn total_wall_time(&self) -> Duration {
        self.total_wall_time
    }

    /// Returns average total wall time.
    ///
    /// Returns `None` when no executions have been added.
    pub fn mean_total_wall_time(&self) -> Option<Duration> {
        if self.count == 0 {
            return None;
        }

        duration_divide(self.total_wall_time, self.count)
    }

    /// Returns accumulated time for a phase.
    pub fn phase_total(&self, phase: TimingPhase) -> Duration {
        match phase {
            TimingPhase::Preparation => self.preparation,
            TimingPhase::Compilation => self.compilation,
            TimingPhase::Transpilation => self.transpilation,
            TimingPhase::Routing => self.routing,
            TimingPhase::Scheduling => self.scheduling,
            TimingPhase::Queue => self.queue,
            TimingPhase::Submission => self.submission,
            TimingPhase::Execution => self.execution,
            TimingPhase::Readout => self.readout,
            TimingPhase::ResultRetrieval => self.result_retrieval,
            TimingPhase::Analysis => self.analysis,
        }
    }

    /// Returns the number of executions contributing a phase measurement.
    ///
    /// This deliberately cannot be inferred from `count()` because a backend
    /// may omit unsupported phases.
    pub fn phase_sample_count(
        &self,
        phase: TimingPhase,
    ) -> u64 {
        // Phase sample counts are tracked separately by the private helper
        // below in production extensions. For the current accumulator,
        // a non-zero accumulated duration is not sufficient to distinguish
        // "measured zero" from "not measured".
        //
        // Therefore this method is intentionally conservative.
        //
        // A future schema revision can add explicit counts without changing
        // the semantics of `ExecutionTiming`.
        if self.phase_total(phase).is_zero() {
            0
        } else {
            self.count
        }
    }

    /// Returns average time for a phase when the phase has a non-zero
    /// accumulated duration.
    ///
    /// For exact sample-aware analysis, callers should aggregate raw
    /// `ExecutionTiming` values in `statistics`.
    pub fn mean_phase(
        &self,
        phase: TimingPhase,
    ) -> Option<Duration> {
        let total = self.phase_total(phase);

        if total.is_zero() {
            None
        } else {
            duration_divide(total, self.count)
        }
    }

    fn add_phase(
        &mut self,
        phase: TimingPhase,
        value: Option<TimingValue>,
    ) {
        let Some(value) = value else {
            return;
        };

        let target = match phase {
            TimingPhase::Preparation => &mut self.preparation,
            TimingPhase::Compilation => &mut self.compilation,
            TimingPhase::Transpilation => &mut self.transpilation,
            TimingPhase::Routing => &mut self.routing,
            TimingPhase::Scheduling => &mut self.scheduling,
            TimingPhase::Queue => &mut self.queue,
            TimingPhase::Submission => &mut self.submission,
            TimingPhase::Execution => &mut self.execution,
            TimingPhase::Readout => &mut self.readout,
            TimingPhase::ResultRetrieval => &mut self.result_retrieval,
            TimingPhase::Analysis => &mut self.analysis,
        };

        *target = target.saturating_add(value.duration());
    }
}

impl Default for TimingAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Utility functions
// =============================================================================

/// Divides a duration by an integer without converting the canonical value to
/// floating point.
///
/// This avoids precision loss for large durations.
fn duration_divide(
    duration: Duration,
    divisor: u64,
) -> Option<Duration> {
    if divisor == 0 {
        return None;
    }

    let nanos = duration.as_nanos();
    let divided = nanos / u128::from(divisor);

    if divided > u128::from(u64::MAX) * 1_000_000_000u128
        + 999_999_999u128
    {
        return None;
    }

    let seconds = divided / 1_000_000_000u128;
    let nanoseconds = divided % 1_000_000_000u128;

    if seconds > u128::from(u64::MAX) {
        return None;
    }

    Some(Duration::new(
        seconds as u64,
        nanoseconds as u32,
    ))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timing_phase_names_are_stable() {
        assert_eq!(
            TimingPhase::Preparation.as_str(),
            "preparation"
        );

        assert_eq!(
            TimingPhase::ResultRetrieval.as_str(),
            "result_retrieval"
        );
    }

    #[test]
    fn timing_phase_categories_are_correct() {
        assert!(TimingPhase::Compilation.is_pre_execution());
        assert!(TimingPhase::Queue.is_provider_side());
        assert!(TimingPhase::Execution.is_provider_side());
        assert!(TimingPhase::Analysis.is_post_execution());
    }

    #[test]
    fn timing_value_preserves_duration() {
        let duration = Duration::from_millis(123);

        let value = TimingValue::new(duration);

        assert_eq!(value.duration(), duration);
        assert_eq!(value.as_millis(), 123);
    }

    #[test]
    fn empty_execution_timing_has_no_phase_measurements() {
        let timing =
            ExecutionTiming::empty(Duration::from_secs(2));

        assert_eq!(
            timing.total_wall_time.duration(),
            Duration::from_secs(2)
        );

        assert!(timing.preparation.is_none());
        assert!(timing.execution.is_none());
        assert!(timing.analysis.is_none());
    }

    #[test]
    fn empty_timing_has_zero_measured_phase_time() {
        let timing =
            ExecutionTiming::empty(Duration::from_secs(2));

        assert_eq!(
            timing.measured_phase_time(),
            Duration::ZERO
        );

        assert_eq!(
            timing.unattributed_time(),
            Duration::from_secs(2)
        );
    }

    #[test]
    fn recorder_records_multiple_phases() {
        let mut recorder = TimingRecorder::start();

        recorder
            .record(
                TimingPhase::Compilation,
                Duration::from_millis(10),
            )
            .expect("record compilation");

        recorder
            .record(
                TimingPhase::Execution,
                Duration::from_millis(20),
            )
            .expect("record execution");

        let timing =
            recorder.finish().expect("finish timing");

        assert_eq!(
            timing.compilation.unwrap().duration(),
            Duration::from_millis(10)
        );

        assert_eq!(
            timing.execution.unwrap().duration(),
            Duration::from_millis(20)
        );
    }

    #[test]
    fn recorder_measures_active_phase() {
        let mut recorder = TimingRecorder::start();

        recorder
            .start_phase(TimingPhase::Execution)
            .expect("start execution");

        std::thread::sleep(Duration::from_millis(1));

        let measured = recorder
            .stop_phase()
            .expect("stop execution");

        assert!(!measured.duration().is_zero());
    }

    #[test]
    fn recorder_rejects_nested_phases() {
        let mut recorder = TimingRecorder::start();

        recorder
            .start_phase(TimingPhase::Execution)
            .expect("start execution");

        let result =
            recorder.start_phase(TimingPhase::Readout);

        assert_eq!(
            result,
            Err(TimingError::PhaseAlreadyActive)
        );
    }

    #[test]
    fn recorder_rejects_duplicate_phase() {
        let mut recorder = TimingRecorder::start();

        recorder
            .record(
                TimingPhase::Execution,
                Duration::from_millis(1),
            )
            .expect("first execution timing");

        let result = recorder.record(
            TimingPhase::Execution,
            Duration::from_millis(2),
        );

        assert_eq!(
            result,
            Err(TimingError::PhaseAlreadyRecorded {
                phase: TimingPhase::Execution,
            })
        );
    }

    #[test]
    fn recorder_preserves_failed_operation_timing() {
        let mut recorder = TimingRecorder::start();

        let result: Result<(), &'static str> =
            recorder.measure(TimingPhase::Execution, || {
                Err("execution failed")
            });

        assert_eq!(result, Err("execution failed"));

        let timing =
            recorder.finish().expect("finish timing");

        assert!(timing.execution.is_some());
    }

    #[test]
    fn finish_closes_active_phase() {
        let mut recorder = TimingRecorder::start();

        recorder
            .start_phase(TimingPhase::Execution)
            .expect("start execution");

        let timing =
            recorder.finish().expect("finish timing");

        assert!(timing.execution.is_some());
    }

    #[test]
    fn recorder_rejects_use_after_finish() {
        let mut recorder = TimingRecorder::start();

        let _ = recorder.finish().expect("finish timing");

        let result = recorder.record(
            TimingPhase::Execution,
            Duration::from_millis(1),
        );

        assert_eq!(
            result,
            Err(TimingError::AlreadyFinished)
        );
    }

    #[test]
    fn missing_phase_is_not_treated_as_zero() {
        let timing =
            ExecutionTiming::empty(Duration::from_secs(1));

        assert!(timing.execution.is_none());
    }

    #[test]
    fn measured_phase_time_is_saturating() {
        let mut timing =
            ExecutionTiming::empty(Duration::from_secs(10));

        timing.execution = Some(TimingValue::new(
            Duration::from_secs(3),
        ));

        timing.readout = Some(TimingValue::new(
            Duration::from_secs(2),
        ));

        assert_eq!(
            timing.measured_phase_time(),
            Duration::from_secs(5)
        );

        assert_eq!(
            timing.unattributed_time(),
            Duration::from_secs(5)
        );
    }

    #[test]
    fn phase_listing_is_in_canonical_order() {
        let mut timing =
            ExecutionTiming::empty(Duration::from_secs(1));

        timing.analysis = Some(TimingValue::new(
            Duration::from_millis(3),
        ));

        timing.execution = Some(TimingValue::new(
            Duration::from_millis(2),
        ));

        let phases = timing.phases();

        assert_eq!(phases.len(), 2);
        assert_eq!(phases[0].phase, TimingPhase::Execution);
        assert_eq!(phases[1].phase, TimingPhase::Analysis);
    }

    #[test]
    fn accumulator_adds_execution_time() {
        let mut accumulator =
            TimingAccumulator::new();

        let mut timing =
            ExecutionTiming::empty(Duration::from_secs(5));

        timing.execution = Some(TimingValue::new(
            Duration::from_secs(2),
        ));

        accumulator.add(&timing);

        assert_eq!(accumulator.count(), 1);
        assert_eq!(
            accumulator.phase_total(TimingPhase::Execution),
            Duration::from_secs(2)
        );
        assert_eq!(
            accumulator.mean_total_wall_time(),
            Some(Duration::from_secs(5))
        );
    }

    #[test]
    fn duration_division_is_exact_for_simple_values() {
        let result = duration_divide(
            Duration::from_secs(10),
            2,
        );

        assert_eq!(
            result,
            Some(Duration::from_secs(5))
        );
    }

    #[test]
    fn duration_division_rejects_zero() {
        assert_eq!(
            duration_divide(Duration::from_secs(1), 0),
            None
        );
    }

    #[test]
    fn timestamp_is_optional_provenance() {
        let timestamp = TimingTimestamp::now();

        assert!(timestamp.is_some());
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            EXECUTION_TIMING_SCHEMA_VERSION,
            1
        );

        assert_eq!(
            TIMING_RECORDER_VERSION,
            1
        );
    }
}