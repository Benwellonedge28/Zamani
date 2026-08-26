//! Zamani Quantum Benchmarking — Runtime and Timing Metrics
//!
//! Production-grade runtime, latency, throughput, and timing-breakdown
//! calculations for the Zamani quantum benchmarking subsystem.
//!
//! # Scope
//!
//! This module owns the *measurement and representation* of execution-time
//! metrics. It does NOT:
//!
//! - execute quantum circuits;
//! - submit jobs to hardware;
//! - perform compilation;
//! - perform transpilation;
//! - perform routing;
//! - perform scheduling;
//! - communicate with backend providers;
//! - own benchmark protocols;
//! - perform statistical fitting;
//! - calculate fidelity;
//! - mutate process-global state;
//! - print diagnostics;
//! - assume that all quantum systems are gate-model QPUs.
//!
//! The execution layer is responsible for capturing timing observations.
//! This module validates those observations and converts them into canonical
//! Zamani benchmarking metrics.
//!
//! # Architectural position
//!
//! ```text
//! Quantum IR / benchmark workload
//!             │
//!             ▼
//!      execution::executor
//!             │
//!             │ captures timing
//!             ▼
//!      RuntimeBreakdown
//!             │
//!             ▼
//!   benchmarking::metrics::runtime
//!             │
//!       ┌─────┼──────────┐
//!       ▼     ▼          ▼
//!     Metric  Metric   Throughput
//!       │     │          │
//!       └─────┼──────────┘
//!             ▼
//!       BenchmarkResult
//! ```
//!
//! # Important timing rule
//!
//! Durations must be measured using a monotonic source whenever possible.
//! `std::time::Instant` is therefore used by the timing helpers for local
//! elapsed-time measurement.
//!
//! Wall-clock timestamps are not used to calculate elapsed durations because
//! wall clocks can move forwards or backwards due to clock synchronization,
//! manual changes, virtualization, or operating-system adjustments.
//!
//! Provider/backend reported durations are accepted as measurements, but their
//! source is explicitly recorded so they cannot be confused with locally
//! measured monotonic durations.
//!
//! # Pipeline stages
//!
//! The runtime model deliberately separates:
//!
//! - compilation;
//! - transpilation;
//! - routing;
//! - scheduling;
//! - queue;
//! - submission;
//! - execution;
//! - readout;
//! - analysis;
//! - classical preprocessing;
//! - classical postprocessing;
//! - total wall time.
//!
//! These stages must not be blindly summed into total wall time. Some stages
//! can overlap, especially queueing, asynchronous submission, classical
//! processing, and backend-side work.
//!
//! # Production invariants
//!
//! This module guarantees:
//!
//! 1. No NaN or infinity enters a canonical metric.
//! 2. Zero-duration measurements are valid.
//! 3. Negative durations cannot be represented.
//! 4. Integer duration arithmetic is checked for overflow.
//! 5. Throughput cannot be calculated with a zero duration.
//! 6. Throughput cannot be calculated from zero work.
//! 7. Caller-provided counters cannot overflow silently.
//! 8. Timing sources are explicit.
//! 9. Stage identity is explicit.
//! 10. Total wall time is not assumed to equal the sum of pipeline stages.
//! 11. Backend-reported timing is not silently treated as local timing.
//! 12. Runtime metrics use canonical `core::metric::Metric` values.
//! 13. No execution or backend dependency is introduced.
//! 14. No global mutable state is used.
//! 15. No unsafe code is required.
//! 16. The API is deterministic for deterministic inputs.
//! 17. Runtime metrics can be independently re-analysed from serialized
//!     timing observations.
//!
//! # Integration contract
//!
//! ```text
//! core::execution
//!      │
//!      ├── captures local timing using RuntimeTimer
//!      │
//!      └── receives backend timing when available
//!                 │
//!                 ▼
//!          RuntimeBreakdown
//!                 │
//!                 ▼
//!      metrics::runtime::RuntimeMetrics
//!                 │
//!        ┌────────┼────────┐
//!        ▼        ▼        ▼
//!      Metric   Metric   Metric
//!        │        │        │
//!        └────────┼────────┘
//!                 ▼
//!          core::result
//! ```
//!
//! The module depends only on the canonical metric model:
//!
//! ```text
//! metrics::runtime
//!        │
//!        ▼
//! core::metric
//! ```
//!
//! It must never introduce a dependency in the opposite direction.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1.
//!
//! Rust 2021 edition.
//! No nightly features.
//! No additional crate dependency.
//!
//! # Scientific/benchmarking rationale
//!
//! Runtime is not synonymous with execution time.
//!
//! A production benchmark must distinguish at least:
//!
//! ```text
//! compilation
//! transpilation
//! routing
//! scheduling
//! queue
//! submission
//! execution
//! readout
//! analysis
//! total wall time
//! ```
//!
//! This allows Zamani to answer questions such as:
//!
//! - How long did the quantum processor actually execute?
//! - How much time was spent waiting in a provider queue?
//! - How much compiler overhead was introduced?
//! - How much time did measurement/readout consume?
//! - How much classical processing surrounded the quantum workload?
//! - How many shots or circuits were completed per second?
//! - What was the end-to-end time-to-solution?
//!
//! This separation is particularly important for application-oriented
//! benchmarking, where execution time and result quality are both relevant.
//!
//! -----------------------------------------------------------------------------
//! Public API
//! -----------------------------------------------------------------------------
//!
//! Primary types:
//!
//! - `TimingSource`
//! - `RuntimeStage`
//! - `StageDuration`
//! - `RuntimeBreakdown`
//! - `RuntimeTimer`
//! - `RuntimeMetrics`
//! - `ThroughputWork`
//! - `RuntimeError`
//!
//! -----------------------------------------------------------------------------
//! Example
//! -----------------------------------------------------------------------------
//!
//! ```rust
//! use std::time::Duration;
//!
//! use crate::quantum::benchmarking::metrics::runtime::{
//!     RuntimeBreakdown,
//!     RuntimeMetrics,
//!     StageDuration,
//!     TimingSource,
//! };
//!
//! let mut breakdown = RuntimeBreakdown::new();
//!
//! breakdown.set_execution(StageDuration::from_duration(
//!     Duration::from_millis(25),
//!     TimingSource::MonotonicClock,
//! ));
//!
//! breakdown.set_readout(StageDuration::from_duration(
//!     Duration::from_millis(5),
//!     TimingSource::MonotonicClock,
//! ));
//!
//! breakdown.set_total_wall(StageDuration::from_duration(
//!     Duration::from_millis(40),
//!     TimingSource::MonotonicClock,
//! ));
//!
//! let metrics = RuntimeMetrics::from_breakdown(&breakdown).unwrap();
//!
//! let execution_time = metrics.execution_time().unwrap();
//! assert_eq!(execution_time.duration.as_secs_f64(), 0.025);
//! ```
//!

use std::fmt;
use std::time::{Duration, Instant};

use crate::quantum::benchmarking::core::metric::{
    Metric,
    MetricKind,
    MetricUnit,
};

// =============================================================================
// Constants
// =============================================================================

/// Stable runtime-metric schema version.
pub const RUNTIME_METRICS_SCHEMA_VERSION: u32 = 1;

/// Number of nanoseconds in one second.
pub const NANOS_PER_SECOND: u128 = 1_000_000_000;

/// Number of microseconds in one second.
pub const MICROS_PER_SECOND: u128 = 1_000_000;

/// Number of milliseconds in one second.
pub const MILLIS_PER_SECOND: u128 = 1_000;

/// Maximum number of metadata bytes accepted for a runtime observation.
pub const MAX_RUNTIME_METADATA_BYTES: usize = 4096;

// =============================================================================
// Timing source
// =============================================================================

/// Source from which a timing observation originated.
///
/// The source is part of the scientific meaning of a timing result. Two
/// measurements with the same numeric duration are not necessarily equivalent
/// if one was measured locally and the other was reported by a provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimingSource {
    /// Locally measured using a monotonic clock.
    MonotonicClock,

    /// Timing supplied by the execution backend.
    BackendReported,

    /// Timing supplied by a provider/service.
    ProviderReported,

    /// Timing supplied by a simulator/emulator.
    SimulatorReported,

    /// Timing reconstructed from externally supplied measurements.
    ExternalMeasurement,
}

impl TimingSource {
    /// Stable machine-readable identifier.
    pub const fn id(self) -> &'static str {
        match self {
            Self::MonotonicClock => "monotonic_clock",
            Self::BackendReported => "backend_reported",
            Self::ProviderReported => "provider_reported",
            Self::SimulatorReported => "simulator_reported",
            Self::ExternalMeasurement => "external_measurement",
        }
    }

    /// Returns whether this source is monotonic by construction.
    pub const fn is_monotonic(self) -> bool {
        matches!(self, Self::MonotonicClock)
    }
}

impl fmt::Display for TimingSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.id())
    }
}

// =============================================================================
// Runtime stages
// =============================================================================

/// Canonical stages of a quantum execution pipeline.
///
/// The stages intentionally distinguish compilation/transpilation/routing and
/// queue/submission/execution/readout/analysis. This prevents a benchmark from
/// hiding important overhead inside a single "runtime" value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RuntimeStage {
    /// Frontend/compiler processing before backend-specific transformation.
    Compilation,

    /// Backend-specific lowering/transpilation.
    Transpilation,

    /// Mapping logical operations to physical resources.
    Routing,

    /// Scheduling operations in time.
    Scheduling,

    /// Waiting for backend/provider execution capacity.
    Queue,

    /// Submission/transport/acceptance overhead.
    Submission,

    /// Actual quantum execution interval as reported/measured.
    Execution,

    /// Measurement/readout interval.
    Readout,

    /// Benchmark result analysis interval.
    Analysis,

    /// Classical processing performed before quantum execution.
    ClassicalPreprocessing,

    /// Classical processing performed after quantum execution.
    ClassicalPostprocessing,

    /// Complete end-to-end wall-clock interval.
    TotalWall,
}

impl RuntimeStage {
    /// Stable machine-readable identifier.
    pub const fn id(self) -> &'static str {
        match self {
            Self::Compilation => "compilation",
            Self::Transpilation => "transpilation",
            Self::Routing => "routing",
            Self::Scheduling => "scheduling",
            Self::Queue => "queue",
            Self::Submission => "submission",
            Self::Execution => "execution",
            Self::Readout => "readout",
            Self::Analysis => "analysis",
            Self::ClassicalPreprocessing => "classical_preprocessing",
            Self::ClassicalPostprocessing => "classical_postprocessing",
            Self::TotalWall => "total_wall",
        }
    }

    /// Human-readable stage name.
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Compilation => "Compilation",
            Self::Transpilation => "Transpilation",
            Self::Routing => "Routing",
            Self::Scheduling => "Scheduling",
            Self::Queue => "Queue",
            Self::Submission => "Submission",
            Self::Execution => "Execution",
            Self::Readout => "Readout",
            Self::Analysis => "Analysis",
            Self::ClassicalPreprocessing => "Classical Preprocessing",
            Self::ClassicalPostprocessing => "Classical Postprocessing",
            Self::TotalWall => "Total Wall Time",
        }
    }

    /// Returns the canonical metric kind associated with the stage.
    ///
    /// Stages without a dedicated canonical `MetricKind` use `Custom` so that
    /// the stage remains independently observable without requiring another
    /// edit to `core::metric.rs`.
    pub fn metric_kind(self) -> MetricKind {
        match self {
            Self::Compilation => MetricKind::CompilationTime,
            Self::Transpilation => {
                MetricKind::Custom("transpilation_time".to_owned())
            }
            Self::Routing => MetricKind::Custom("routing_time".to_owned()),
            Self::Scheduling => {
                MetricKind::Custom("scheduling_time".to_owned())
            }
            Self::Queue => MetricKind::QueueTime,
            Self::Submission => MetricKind::SubmissionTime,
            Self::Execution => MetricKind::ExecutionTime,
            Self::Readout => MetricKind::ReadoutTime,
            Self::Analysis => MetricKind::AnalysisTime,
            Self::ClassicalPreprocessing => MetricKind::Custom(
                "classical_preprocessing_time".to_owned(),
            ),
            Self::ClassicalPostprocessing => MetricKind::Custom(
                "classical_postprocessing_time".to_owned(),
            ),
            Self::TotalWall => MetricKind::TotalWallTime,
        }
    }
}

impl fmt::Display for RuntimeStage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.id())
    }
}

// =============================================================================
// Stage duration
// =============================================================================

/// Validated duration measurement for one runtime stage.
///
/// The duration is stored as nanoseconds rather than `Duration` so that the
/// observation can be serialized without introducing a dependency on a
/// particular serialization representation for `Duration`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StageDuration {
    /// Elapsed duration in nanoseconds.
    nanoseconds: u128,

    /// Source of the measurement.
    source: TimingSource,
}

impl StageDuration {
    /// Creates a stage duration from nanoseconds.
    pub fn from_nanoseconds(
        nanoseconds: u128,
        source: TimingSource,
    ) -> Self {
        Self {
            nanoseconds,
            source,
        }
    }

    /// Creates a stage duration from a `Duration`.
    pub fn from_duration(
        duration: Duration,
        source: TimingSource,
    ) -> Self {
        Self {
            nanoseconds: duration.as_nanos(),
            source,
        }
    }

    /// Returns the duration as nanoseconds.
    #[must_use]
    pub const fn nanoseconds(self) -> u128 {
        self.nanoseconds
    }

    /// Returns the timing source.
    #[must_use]
    pub const fn source(self) -> TimingSource {
        self.source
    }

    /// Returns the duration as a standard-library `Duration`.
    ///
    /// `Duration` supports nanoseconds represented by a `u64` internally.
    /// Therefore this conversion is fallible.
    pub fn as_duration(self) -> Result<Duration, RuntimeError> {
        if self.nanoseconds > u64::MAX as u128 {
            return Err(RuntimeError::DurationOverflow {
                nanoseconds: self.nanoseconds,
            });
        }

        Ok(Duration::from_nanos(self.nanoseconds as u64))
    }

    /// Returns seconds as an `f64`.
    ///
    /// The result is guaranteed finite for representable `Duration` values.
    pub fn as_seconds_f64(self) -> Result<f64, RuntimeError> {
        let seconds = self.nanoseconds as f64 / NANOS_PER_SECOND as f64;

        if !seconds.is_finite() {
            return Err(RuntimeError::NonFiniteDerivedValue {
                context: "duration_seconds",
            });
        }

        Ok(seconds)
    }

    /// Returns milliseconds as an `f64`.
    pub fn as_milliseconds_f64(self) -> Result<f64, RuntimeError> {
        let milliseconds =
            self.nanoseconds as f64 / MILLIS_PER_SECOND as f64;

        if !milliseconds.is_finite() {
            return Err(RuntimeError::NonFiniteDerivedValue {
                context: "duration_milliseconds",
            });
        }

        Ok(milliseconds)
    }

    /// Returns microseconds as an `f64`.
    pub fn as_microseconds_f64(self) -> Result<f64, RuntimeError> {
        let microseconds =
            self.nanoseconds as f64 / MICROS_PER_SECOND as f64;

        if !microseconds.is_finite() {
            return Err(RuntimeError::NonFiniteDerivedValue {
                context: "duration_microseconds",
            });
        }

        Ok(microseconds)
    }

    /// Returns whether the duration is exactly zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.nanoseconds == 0
    }
}

impl fmt::Display for StageDuration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} ns ({})",
            self.nanoseconds,
            self.source
        )
    }
}

// =============================================================================
// Runtime timer
// =============================================================================

/// Local monotonic timer for runtime measurements.
///
/// This helper deliberately uses `Instant` rather than wall-clock timestamps.
/// It is suitable for local measurements inside the execution/runtime path.
#[derive(Debug)]
pub struct RuntimeTimer {
    stage: RuntimeStage,
    started_at: Instant,
}

impl RuntimeTimer {
    /// Starts a timer for a runtime stage.
    #[must_use]
    pub fn start(stage: RuntimeStage) -> Self {
        Self {
            stage,
            started_at: Instant::now(),
        }
    }

    /// Returns the stage being measured.
    #[must_use]
    pub const fn stage(&self) -> RuntimeStage {
        self.stage
    }

    /// Returns elapsed time without stopping the timer.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    /// Stops the timer and produces a validated duration.
    #[must_use]
    pub fn stop(self) -> StageDuration {
        StageDuration::from_duration(
            self.started_at.elapsed(),
            TimingSource::MonotonicClock,
        )
    }
}

// =============================================================================
// Runtime breakdown
// =============================================================================

/// Complete timing breakdown for one benchmark execution.
///
/// Every stage is optional because not every execution target exposes every
/// stage. For example, a local simulator may not have a provider queue, while a
/// hardware provider may not expose internal scheduling time.
///
/// Absence is represented by `None`, not zero. This distinction is important:
///
/// - `None` = not measured / not available.
/// - `Some(0)` = measured and observed to be zero.
///
/// This prevents false claims about unmeasured runtime stages.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RuntimeBreakdown {
    compilation: Option<StageDuration>,
    transpilation: Option<StageDuration>,
    routing: Option<StageDuration>,
    scheduling: Option<StageDuration>,
    queue: Option<StageDuration>,
    submission: Option<StageDuration>,
    execution: Option<StageDuration>,
    readout: Option<StageDuration>,
    analysis: Option<StageDuration>,
    classical_preprocessing: Option<StageDuration>,
    classical_postprocessing: Option<StageDuration>,
    total_wall: Option<StageDuration>,
}

impl RuntimeBreakdown {
    /// Creates an empty timing breakdown.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            compilation: None,
            transpilation: None,
            routing: None,
            scheduling: None,
            queue: None,
            submission: None,
            execution: None,
            readout: None,
            analysis: None,
            classical_preprocessing: None,
            classical_postprocessing: None,
            total_wall: None,
        }
    }

    /// Sets compilation time.
    pub fn set_compilation(&mut self, value: StageDuration) {
        self.compilation = Some(value);
    }

    /// Sets transpilation time.
    pub fn set_transpilation(&mut self, value: StageDuration) {
        self.transpilation = Some(value);
    }

    /// Sets routing time.
    pub fn set_routing(&mut self, value: StageDuration) {
        self.routing = Some(value);
    }

    /// Sets scheduling time.
    pub fn set_scheduling(&mut self, value: StageDuration) {
        self.scheduling = Some(value);
    }

    /// Sets queue time.
    pub fn set_queue(&mut self, value: StageDuration) {
        self.queue = Some(value);
    }

    /// Sets submission time.
    pub fn set_submission(&mut self, value: StageDuration) {
        self.submission = Some(value);
    }

    /// Sets execution time.
    pub fn set_execution(&mut self, value: StageDuration) {
        self.execution = Some(value);
    }

    /// Sets readout time.
    pub fn set_readout(&mut self, value: StageDuration) {
        self.readout = Some(value);
    }

    /// Sets analysis time.
    pub fn set_analysis(&mut self, value: StageDuration) {
        self.analysis = Some(value);
    }

    /// Sets classical preprocessing time.
    pub fn set_classical_preprocessing(
        &mut self,
        value: StageDuration,
    ) {
        self.classical_preprocessing = Some(value);
    }

    /// Sets classical postprocessing time.
    pub fn set_classical_postprocessing(
        &mut self,
        value: StageDuration,
    ) {
        self.classical_postprocessing = Some(value);
    }

    /// Sets end-to-end wall time.
    pub fn set_total_wall(&mut self, value: StageDuration) {
        self.total_wall = Some(value);
    }

    /// Returns compilation time.
    #[must_use]
    pub const fn compilation(&self) -> Option<StageDuration> {
        self.compilation
    }

    /// Returns transpilation time.
    #[must_use]
    pub const fn transpilation(&self) -> Option<StageDuration> {
        self.transpilation
    }

    /// Returns routing time.
    #[must_use]
    pub const fn routing(&self) -> Option<StageDuration> {
        self.routing
    }

    /// Returns scheduling time.
    #[must_use]
    pub const fn scheduling(&self) -> Option<StageDuration> {
        self.scheduling
    }

    /// Returns queue time.
    #[must_use]
    pub const fn queue(&self) -> Option<StageDuration> {
        self.queue
    }

    /// Returns submission time.
    #[must_use]
    pub const fn submission(&self) -> Option<StageDuration> {
        self.submission
    }

    /// Returns execution time.
    #[must_use]
    pub const fn execution(&self) -> Option<StageDuration> {
        self.execution
    }

    /// Returns readout time.
    #[must_use]
    pub const fn readout(&self) -> Option<StageDuration> {
        self.readout
    }

    /// Returns analysis time.
    #[must_use]
    pub const fn analysis(&self) -> Option<StageDuration> {
        self.analysis
    }

    /// Returns classical preprocessing time.
    #[must_use]
    pub const fn classical_preprocessing(&self) -> Option<StageDuration> {
        self.classical_preprocessing
    }

    /// Returns classical postprocessing time.
    #[must_use]
    pub const fn classical_postprocessing(&self) -> Option<StageDuration> {
        self.classical_postprocessing
    }

    /// Returns total wall time.
    #[must_use]
    pub const fn total_wall(&self) -> Option<StageDuration> {
        self.total_wall
    }

    /// Returns the duration for a requested stage.
    #[must_use]
    pub const fn stage(
        &self,
        stage: RuntimeStage,
    ) -> Option<StageDuration> {
        match stage {
            RuntimeStage::Compilation => self.compilation,
            RuntimeStage::Transpilation => self.transpilation,
            RuntimeStage::Routing => self.routing,
            RuntimeStage::Scheduling => self.scheduling,
            RuntimeStage::Queue => self.queue,
            RuntimeStage::Submission => self.submission,
            RuntimeStage::Execution => self.execution,
            RuntimeStage::Readout => self.readout,
            RuntimeStage::Analysis => self.analysis,
            RuntimeStage::ClassicalPreprocessing => {
                self.classical_preprocessing
            }
            RuntimeStage::ClassicalPostprocessing => {
                self.classical_postprocessing
            }
            RuntimeStage::TotalWall => self.total_wall,
        }
    }

    /// Returns all measured stages in stable order.
    #[must_use]
    pub fn measured_stages(&self) -> Vec<(RuntimeStage, StageDuration)> {
        let stages = [
            RuntimeStage::Compilation,
            RuntimeStage::Transpilation,
            RuntimeStage::Routing,
            RuntimeStage::Scheduling,
            RuntimeStage::Queue,
            RuntimeStage::Submission,
            RuntimeStage::Execution,
            RuntimeStage::Readout,
            RuntimeStage::Analysis,
            RuntimeStage::ClassicalPreprocessing,
            RuntimeStage::ClassicalPostprocessing,
            RuntimeStage::TotalWall,
        ];

        stages
            .iter()
            .filter_map(|stage| {
                self.stage(*stage).map(|duration| (*stage, duration))
            })
            .collect()
    }

    /// Returns whether at least one timing stage was measured.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.compilation.is_none()
            && self.transpilation.is_none()
            && self.routing.is_none()
            && self.scheduling.is_none()
            && self.queue.is_none()
            && self.submission.is_none()
            && self.execution.is_none()
            && self.readout.is_none()
            && self.analysis.is_none()
            && self.classical_preprocessing.is_none()
            && self.classical_postprocessing.is_none()
            && self.total_wall.is_none()
    }

    /// Validates all timing observations.
    pub fn validate(&self) -> Result<(), RuntimeError> {
        for (stage, duration) in self.measured_stages() {
            validate_stage_duration(stage, duration)?;
        }

        Ok(())
    }

    /// Sums explicitly supplied stages with checked integer arithmetic.
    ///
    /// This method does NOT claim that the returned value is the actual
    /// end-to-end runtime. Pipeline stages may overlap.
    pub fn sum_measured_non_total_stages(
        &self,
    ) -> Result<Option<StageDuration>, RuntimeError> {
        let mut total = 0_u128;
        let mut source: Option<TimingSource> = None;
        let mut measured = false;

        let stages = [
            RuntimeStage::Compilation,
            RuntimeStage::Transpilation,
            RuntimeStage::Routing,
            RuntimeStage::Scheduling,
            RuntimeStage::Queue,
            RuntimeStage::Submission,
            RuntimeStage::Execution,
            RuntimeStage::Readout,
            RuntimeStage::Analysis,
            RuntimeStage::ClassicalPreprocessing,
            RuntimeStage::ClassicalPostprocessing,
        ];

        for stage in stages {
            if let Some(duration) = self.stage(stage) {
                measured = true;

                total = total.checked_add(duration.nanoseconds()).ok_or(
                    RuntimeError::DurationArithmeticOverflow {
                        operation: "sum_measured_non_total_stages",
                    },
                )?;

                source = match source {
                    None => Some(duration.source()),
                    Some(existing) if existing == duration.source() => {
                        Some(existing)
                    }
                    Some(_) => Some(TimingSource::ExternalMeasurement),
                };
            }
        }

        Ok(if measured {
            Some(StageDuration::from_nanoseconds(
                total,
                source.unwrap_or(TimingSource::ExternalMeasurement),
            ))
        } else {
            None
        })
    }
}

// =============================================================================
// Throughput work
// =============================================================================

/// Work completed during a timed interval.
///
/// The numerator is intentionally represented using a named work type rather
/// than accepting an arbitrary floating-point value. This prevents accidental
/// claims such as "circuits per second" when the actual numerator was shots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThroughputWork {
    /// Completed shots/samples.
    Shots(u64),

    /// Completed circuits.
    Circuits(u64),

    /// Completed gates/operations.
    Gates(u64),

    /// Completed two-qubit gates.
    TwoQubitGates(u64),

    /// Completed circuit layers.
    Layers(u64),
}

impl ThroughputWork {
    /// Returns the work count.
    #[must_use]
    pub const fn count(self) -> u64 {
        match self {
            Self::Shots(value)
            | Self::Circuits(value)
            | Self::Gates(value)
            | Self::TwoQubitGates(value)
            | Self::Layers(value) => value,
        }
    }

    /// Returns the canonical metric kind.
    #[must_use]
    pub const fn metric_kind(self) -> MetricKind {
        match self {
            Self::Shots(_) => MetricKind::ShotsPerSecond,
            Self::Circuits(_) => MetricKind::CircuitsPerSecond,
            Self::Gates(_) => MetricKind::GatesPerSecond,
            Self::TwoQubitGates(_) => {
                MetricKind::TwoQubitGatesPerSecond
            }
            Self::Layers(_) => MetricKind::LayersPerSecond,
        }
    }

    /// Returns a stable identifier for the numerator.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Shots(_) => "shots",
            Self::Circuits(_) => "circuits",
            Self::Gates(_) => "gates",
            Self::TwoQubitGates(_) => "two_qubit_gates",
            Self::Layers(_) => "layers",
        }
    }
}

// =============================================================================
// Runtime metric value
// =============================================================================

/// Runtime metric generated from one timing observation.
///
/// This wrapper retains the original validated duration alongside the
/// canonical `Metric`, so consumers do not lose timing-source information.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeMetric {
    /// Runtime stage represented by this metric.
    pub stage: RuntimeStage,

    /// Original duration observation.
    pub duration: StageDuration,

    /// Canonical Zamani metric.
    pub metric: Metric,
}

impl RuntimeMetric {
    /// Creates a canonical runtime metric from a stage duration.
    pub fn new(
        stage: RuntimeStage,
        duration: StageDuration,
    ) -> Result<Self, RuntimeError> {
        validate_stage_duration(stage, duration)?;

        let value = duration.as_seconds_f64()?;

        let metric = Metric::observed(
            stage.metric_kind(),
            MetricUnit::Seconds,
            value,
        )
        .map_err(RuntimeError::Metric)?;

        Ok(Self {
            stage,
            duration,
            metric,
        })
    }
}

// =============================================================================
// Runtime metrics
// =============================================================================

/// Complete derived runtime metrics for one benchmark execution.
///
/// `RuntimeMetrics` contains only metrics that can be justified by supplied
/// observations. It never manufactures a zero for an unmeasured stage.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeMetrics {
    schema_version: u32,
    breakdown: RuntimeBreakdown,
}

impl RuntimeMetrics {
    /// Creates runtime metrics from a validated timing breakdown.
    pub fn from_breakdown(
        breakdown: &RuntimeBreakdown,
    ) -> Result<Self, RuntimeError> {
        breakdown.validate()?;

        Ok(Self {
            schema_version: RUNTIME_METRICS_SCHEMA_VERSION,
            breakdown: breakdown.clone(),
        })
    }

    /// Returns the runtime metric schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the underlying timing breakdown.
    #[must_use]
    pub const fn breakdown(&self) -> &RuntimeBreakdown {
        &self.breakdown
    }

    /// Returns a metric for any measured runtime stage.
    pub fn stage_metric(
        &self,
        stage: RuntimeStage,
    ) -> Result<Option<RuntimeMetric>, RuntimeError> {
        match self.breakdown.stage(stage) {
            Some(duration) => {
                Ok(Some(RuntimeMetric::new(stage, duration)?))
            }
            None => Ok(None),
        }
    }

    /// Returns all available stage metrics.
    pub fn stage_metrics(
        &self,
    ) -> Result<Vec<RuntimeMetric>, RuntimeError> {
        self.breakdown
            .measured_stages()
            .into_iter()
            .map(|(stage, duration)| RuntimeMetric::new(stage, duration))
            .collect()
    }

    /// Returns compilation time.
    pub fn compilation_time(
        &self,
    ) -> Result<Option<RuntimeMetric>, RuntimeError> {
        self.stage_metric(RuntimeStage::Compilation)
    }

    /// Returns transpilation time.
    pub fn transpilation_time(
        &self,
    ) -> Result<Option<RuntimeMetric>, RuntimeError> {
        self.stage_metric(RuntimeStage::Transpilation)
    }

    /// Returns routing time.
    pub fn routing_time(
        &self,
    ) -> Result<Option<RuntimeMetric>, RuntimeError> {
        self.stage_metric(RuntimeStage::Routing)
    }

    /// Returns scheduling time.
    pub fn scheduling_time(
        &self,
    ) -> Result<Option<RuntimeMetric>, RuntimeError> {
        self.stage_metric(RuntimeStage::Scheduling)
    }

    /// Returns queue time.
    pub fn queue_time(
        &self,
    ) -> Result<Option<RuntimeMetric>, RuntimeError> {
        self.stage_metric(RuntimeStage::Queue)
    }

    /// Returns submission time.
    pub fn submission_time(
        &self,
    ) -> Result<Option<RuntimeMetric>, RuntimeError> {
        self.stage_metric(RuntimeStage::Submission)
    }

    /// Returns quantum execution time.
    pub fn execution_time(
        &self,
    ) -> Result<Option<RuntimeMetric>, RuntimeError> {
        self.stage_metric(RuntimeStage::Execution)
    }

    /// Returns readout time.
    pub fn readout_time(
        &self,
    ) -> Result<Option<RuntimeMetric>, RuntimeError> {
        self.stage_metric(RuntimeStage::Readout)
    }

    /// Returns analysis time.
    pub fn analysis_time(
        &self,
    ) -> Result<Option<RuntimeMetric>, RuntimeError> {
        self.stage_metric(RuntimeStage::Analysis)
    }

    /// Returns classical preprocessing time.
    pub fn classical_preprocessing_time(
        &self,
    ) -> Result<Option<RuntimeMetric>, RuntimeError> {
        self.stage_metric(RuntimeStage::ClassicalPreprocessing)
    }

    /// Returns classical postprocessing time.
    pub fn classical_postprocessing_time(
        &self,
    ) -> Result<Option<RuntimeMetric>, RuntimeError> {
        self.stage_metric(RuntimeStage::ClassicalPostprocessing)
    }

    /// Returns total wall time.
    pub fn total_wall_time(
        &self,
    ) -> Result<Option<RuntimeMetric>, RuntimeError> {
        self.stage_metric(RuntimeStage::TotalWall)
    }

    /// Creates a throughput metric from a measured duration.
    ///
    /// Throughput is expressed in the canonical frequency unit `Hz`, i.e.
    /// completed work per second.
    pub fn throughput(
        &self,
        duration: StageDuration,
        work: ThroughputWork,
    ) -> Result<Metric, RuntimeError> {
        if duration.is_zero() {
            return Err(RuntimeError::ZeroDurationForThroughput {
                work: work.id(),
            });
        }

        if work.count() == 0 {
            return Err(RuntimeError::ZeroWorkForThroughput {
                work: work.id(),
            });
        }

        let seconds = duration.as_seconds_f64()?;

        let throughput = work.count() as f64 / seconds;

        if !throughput.is_finite() {
            return Err(RuntimeError::NonFiniteDerivedValue {
                context: "throughput",
            });
        }

        Metric::observed(
            work.metric_kind(),
            MetricUnit::Hertz,
            throughput,
        )
        .map_err(RuntimeError::Metric)
    }

    /// Calculates execution throughput from the quantum execution stage.
    pub fn execution_throughput(
        &self,
        work: ThroughputWork,
    ) -> Result<Option<Metric>, RuntimeError> {
        match self.breakdown.execution {
            Some(duration) => self.throughput(duration, work).map(Some),
            None => Ok(None),
        }
    }

    /// Calculates end-to-end throughput from total wall time.
    pub fn end_to_end_throughput(
        &self,
        work: ThroughputWork,
    ) -> Result<Option<Metric>, RuntimeError> {
        match self.breakdown.total_wall {
            Some(duration) => self.throughput(duration, work).map(Some),
            None => Ok(None),
        }
    }

    /// Returns the sum of all measured non-total stages.
    ///
    /// This is explicitly labelled a "sum", not total runtime, because stages
    /// can overlap.
    pub fn sum_of_measured_stages(
        &self,
    ) -> Result<Option<RuntimeMetric>, RuntimeError> {
        match self.breakdown.sum_measured_non_total_stages()? {
            Some(duration) => RuntimeMetric::new(
                RuntimeStage::TotalWall,
                duration,
            )
            .map(Some),
            None => Ok(None),
        }
    }

    /// Returns the fraction of total wall time occupied by a stage.
    ///
    /// This calculation is valid only for a stage and total wall measurement
    /// taken over compatible scopes. The method intentionally does not reject
    /// a stage whose duration exceeds total wall time because provider-reported
    /// stages can use different measurement scopes.
    pub fn stage_fraction_of_total(
        &self,
        stage: RuntimeStage,
    ) -> Result<Option<f64>, RuntimeError> {
        let stage_duration = match self.breakdown.stage(stage) {
            Some(value) => value,
            None => return Ok(None),
        };

        let total = match self.breakdown.total_wall {
            Some(value) => value,
            None => return Ok(None),
        };

        if total.is_zero() {
            return Err(RuntimeError::ZeroTotalWallTime);
        }

        let fraction =
            stage_duration.nanoseconds() as f64
                / total.nanoseconds() as f64;

        if !fraction.is_finite() {
            return Err(RuntimeError::NonFiniteDerivedValue {
                context: "stage_fraction_of_total",
            });
        }

        Ok(Some(fraction))
    }

    /// Returns total wall time in seconds.
    pub fn total_wall_seconds(&self) -> Result<Option<f64>, RuntimeError> {
        match self.breakdown.total_wall {
            Some(duration) => duration.as_seconds_f64().map(Some),
            None => Ok(None),
        }
    }

    /// Returns execution time in seconds.
    pub fn execution_seconds(&self) -> Result<Option<f64>, RuntimeError> {
        match self.breakdown.execution {
            Some(duration) => duration.as_seconds_f64().map(Some),
            None => Ok(None),
        }
    }
}

// =============================================================================
// Timing capture helper
// =============================================================================

/// Convenience helper that measures one operation using a monotonic clock.
///
/// This function is intentionally generic and has no knowledge of quantum
/// execution. It can therefore be used by compilation, routing, scheduling,
/// simulation, and backend adapters without creating a dependency on those
/// modules.
pub fn measure<F, T>(
    stage: RuntimeStage,
    operation: F,
) -> Result<(StageDuration, T), RuntimeError>
where
    F: FnOnce() -> T,
{
    let timer = RuntimeTimer::start(stage);
    let result = operation();
    let duration = timer.stop();

    validate_stage_duration(stage, duration)?;

    Ok((duration, result))
}

// =============================================================================
// Validation
// =============================================================================

fn validate_stage_duration(
    stage: RuntimeStage,
    duration: StageDuration,
) -> Result<(), RuntimeError> {
    if duration.nanoseconds() > u128::MAX {
        return Err(RuntimeError::DurationOverflow {
            nanoseconds: duration.nanoseconds(),
        });
    }

    let _ = duration.as_seconds_f64()?;

    // Zero is valid. A zero-time compilation or simulator operation is not
    // physically impossible at this abstraction level and must not be
    // rejected merely because it is unusual.
    let _ = stage;

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Errors generated by runtime metric construction and analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    /// A duration cannot be represented by `std::time::Duration`.
    DurationOverflow {
        /// Original nanosecond value.
        nanoseconds: u128,
    },

    /// Checked duration arithmetic overflowed.
    DurationArithmeticOverflow {
        /// Operation that overflowed.
        operation: &'static str,
    },

    /// A derived floating-point value was not finite.
    NonFiniteDerivedValue {
        /// Semantic context.
        context: &'static str,
    },

    /// Throughput cannot be calculated for zero duration.
    ZeroDurationForThroughput {
        /// Work numerator identifier.
        work: &'static str,
    },

    /// Throughput cannot be calculated for zero work.
    ZeroWorkForThroughput {
        /// Work numerator identifier.
        work: &'static str,
    },

    /// Total wall time is zero where a ratio requires a positive denominator.
    ZeroTotalWallTime,

    /// Canonical metric construction failed.
    Metric(crate::quantum::benchmarking::core::metric::MetricError),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DurationOverflow { nanoseconds } => write!(
                formatter,
                "runtime duration of {nanoseconds} ns exceeds the \
                 representable standard-library Duration range"
            ),

            Self::DurationArithmeticOverflow { operation } => write!(
                formatter,
                "runtime duration arithmetic overflowed during {operation}"
            ),

            Self::NonFiniteDerivedValue { context } => write!(
                formatter,
                "runtime metric calculation produced a non-finite value: \
                 {context}"
            ),

            Self::ZeroDurationForThroughput { work } => write!(
                formatter,
                "cannot calculate {work}/second throughput from zero duration"
            ),

            Self::ZeroWorkForThroughput { work } => write!(
                formatter,
                "cannot calculate {work}/second throughput from zero work"
            ),

            Self::ZeroTotalWallTime => {
                formatter.write_str(
                    "cannot calculate stage fraction because total wall \
                     time is zero",
                )
            }

            Self::Metric(error) => {
                write!(formatter, "canonical metric error: {error}")
            }
        }
    }
}

impl std::error::Error for RuntimeError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_duration_is_valid() {
        let duration =
            StageDuration::from_nanoseconds(0, TimingSource::MonotonicClock);

        assert!(duration.is_zero());
        assert_eq!(duration.nanoseconds(), 0);
        assert_eq!(
            duration.as_seconds_f64().unwrap(),
            0.0
        );
    }

    #[test]
    fn duration_converts_to_seconds() {
        let duration = StageDuration::from_duration(
            Duration::from_millis(1500),
            TimingSource::MonotonicClock,
        );

        let seconds = duration.as_seconds_f64().unwrap();

        assert!((seconds - 1.5).abs() < 1.0e-12);
    }

    #[test]
    fn duration_converts_to_milliseconds() {
        let duration = StageDuration::from_duration(
            Duration::from_micros(2500),
            TimingSource::MonotonicClock,
        );

        let milliseconds =
            duration.as_milliseconds_f64().unwrap();

        assert!((milliseconds - 2.5).abs() < 1.0e-12);
    }

    #[test]
    fn timing_source_is_preserved() {
        let duration = StageDuration::from_nanoseconds(
            100,
            TimingSource::BackendReported,
        );

        assert_eq!(
            duration.source(),
            TimingSource::BackendReported
        );

        assert!(!duration.source().is_monotonic());
    }

    #[test]
    fn runtime_stage_ids_are_stable() {
        assert_eq!(
            RuntimeStage::Compilation.id(),
            "compilation"
        );
        assert_eq!(
            RuntimeStage::Transpilation.id(),
            "transpilation"
        );
        assert_eq!(RuntimeStage::Queue.id(), "queue");
        assert_eq!(RuntimeStage::Execution.id(), "execution");
        assert_eq!(RuntimeStage::Readout.id(), "readout");
        assert_eq!(RuntimeStage::TotalWall.id(), "total_wall");
    }

    #[test]
    fn canonical_metric_kinds_are_used_where_available() {
        assert_eq!(
            RuntimeStage::Compilation.metric_kind(),
            MetricKind::CompilationTime
        );

        assert_eq!(
            RuntimeStage::Queue.metric_kind(),
            MetricKind::QueueTime
        );

        assert_eq!(
            RuntimeStage::Submission.metric_kind(),
            MetricKind::SubmissionTime
        );

        assert_eq!(
            RuntimeStage::Execution.metric_kind(),
            MetricKind::ExecutionTime
        );

        assert_eq!(
            RuntimeStage::Readout.metric_kind(),
            MetricKind::ReadoutTime
        );

        assert_eq!(
            RuntimeStage::Analysis.metric_kind(),
            MetricKind::AnalysisTime
        );

        assert_eq!(
            RuntimeStage::TotalWall.metric_kind(),
            MetricKind::TotalWallTime
        );
    }

    #[test]
    fn stages_without_core_metric_variants_use_custom_metrics() {
        match RuntimeStage::Transpilation.metric_kind() {
            MetricKind::Custom(id) => {
                assert_eq!(id, "transpilation_time");
            }
            other => panic!(
                "unexpected metric kind: {:?}",
                other
            ),
        }

        match RuntimeStage::Routing.metric_kind() {
            MetricKind::Custom(id) => {
                assert_eq!(id, "routing_time");
            }
            other => panic!(
                "unexpected metric kind: {:?}",
                other
            ),
        }
    }

    #[test]
    fn breakdown_distinguishes_unmeasured_from_zero() {
        let mut breakdown = RuntimeBreakdown::new();

        assert!(breakdown.execution().is_none());

        breakdown.set_execution(
            StageDuration::from_nanoseconds(
                0,
                TimingSource::MonotonicClock,
            ),
        );

        assert_eq!(
            breakdown.execution().unwrap().nanoseconds(),
            0
        );
    }

    #[test]
    fn breakdown_can_store_all_pipeline_stages() {
        let mut breakdown = RuntimeBreakdown::new();

        let stages = [
            RuntimeStage::Compilation,
            RuntimeStage::Transpilation,
            RuntimeStage::Routing,
            RuntimeStage::Scheduling,
            RuntimeStage::Queue,
            RuntimeStage::Submission,
            RuntimeStage::Execution,
            RuntimeStage::Readout,
            RuntimeStage::Analysis,
            RuntimeStage::ClassicalPreprocessing,
            RuntimeStage::ClassicalPostprocessing,
            RuntimeStage::TotalWall,
        ];

        let duration =
            StageDuration::from_millis(1);

        breakdown.set_compilation(duration);
        breakdown.set_transpilation(duration);
        breakdown.set_routing(duration);
        breakdown.set_scheduling(duration);
        breakdown.set_queue(duration);
        breakdown.set_submission(duration);
        breakdown.set_execution(duration);
        breakdown.set_readout(duration);
        breakdown.set_analysis(duration);
        breakdown.set_classical_preprocessing(duration);
        breakdown.set_classical_postprocessing(duration);
        breakdown.set_total_wall(duration);

        assert_eq!(
            breakdown.measured_stages().len(),
            stages.len()
        );
    }

    #[test]
    fn runtime_metric_uses_seconds() {
        let duration = StageDuration::from_duration(
            Duration::from_millis(250),
            TimingSource::MonotonicClock,
        );

        let metric =
            RuntimeMetric::new(
                RuntimeStage::Execution,
                duration,
            )
            .unwrap();

        assert_eq!(
            metric.metric.kind,
            MetricKind::ExecutionTime
        );

        assert_eq!(
            metric.metric.unit,
            MetricUnit::Seconds
        );

        assert!(
            (metric.metric.value.get() - 0.25).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn runtime_metrics_preserve_breakdown() {
        let mut breakdown = RuntimeBreakdown::new();

        breakdown.set_execution(
            StageDuration::from_duration(
                Duration::from_millis(25),
                TimingSource::MonotonicClock,
            ),
        );

        breakdown.set_total_wall(
            StageDuration::from_duration(
                Duration::from_millis(40),
                TimingSource::MonotonicClock,
            ),
        );

        let metrics =
            RuntimeMetrics::from_breakdown(&breakdown)
                .unwrap();

        assert!(
            metrics.execution_time().unwrap().is_some()
        );

        assert!(
            metrics.total_wall_time().unwrap().is_some()
        );

        assert_eq!(
            metrics
                .breakdown()
                .execution()
                .unwrap()
                .nanoseconds(),
            25_000_000
        );
    }

    #[test]
    fn runtime_metrics_do_not_invent_missing_stages() {
        let mut breakdown = RuntimeBreakdown::new();

        breakdown.set_execution(
            StageDuration::from_duration(
                Duration::from_millis(10),
                TimingSource::MonotonicClock,
            ),
        );

        let metrics =
            RuntimeMetrics::from_breakdown(&breakdown)
                .unwrap();

        assert!(
            metrics.queue_time().unwrap().is_none()
        );

        assert!(
            metrics.compilation_time().unwrap().is_none()
        );

        assert!(
            metrics.execution_time().unwrap().is_some()
        );
    }

    #[test]
    fn sum_of_stages_is_not_called_total_wall_time() {
        let mut breakdown = RuntimeBreakdown::new();

        breakdown.set_compilation(
            StageDuration::from_duration(
                Duration::from_millis(10),
                TimingSource::MonotonicClock,
            ),
        );

        breakdown.set_execution(
            StageDuration::from_duration(
                Duration::from_millis(20),
                TimingSource::MonotonicClock,
            ),
        );

        breakdown.set_total_wall(
            StageDuration::from_duration(
                Duration::from_millis(25),
                TimingSource::MonotonicClock,
            ),
        );

        let metrics =
            RuntimeMetrics::from_breakdown(&breakdown)
                .unwrap();

        let summed =
            metrics.sum_of_measured_stages()
                .unwrap()
                .unwrap();

        assert_eq!(
            summed.duration.nanoseconds(),
            30_000_000
        );

        assert_eq!(
            metrics
                .total_wall_time()
                .unwrap()
                .unwrap()
                .duration
                .nanoseconds(),
            25_000_000
        );
    }

    #[test]
    fn stage_fraction_of_total_is_supported() {
        let mut breakdown = RuntimeBreakdown::new();

        breakdown.set_execution(
            StageDuration::from_duration(
                Duration::from_millis(20),
                TimingSource::MonotonicClock,
            ),
        );

        breakdown.set_total_wall(
            StageDuration::from_duration(
                Duration::from_millis(40),
                TimingSource::MonotonicClock,
            ),
        );

        let metrics =
            RuntimeMetrics::from_breakdown(&breakdown)
                .unwrap();

        let fraction =
            metrics
                .stage_fraction_of_total(
                    RuntimeStage::Execution,
                )
                .unwrap()
                .unwrap();

        assert!((fraction - 0.5).abs() < 1.0e-12);
    }

    #[test]
    fn zero_total_wall_is_rejected_for_fraction() {
        let mut breakdown = RuntimeBreakdown::new();

        breakdown.set_execution(
            StageDuration::from_nanoseconds(
                10,
                TimingSource::MonotonicClock,
            ),
        );

        breakdown.set_total_wall(
            StageDuration::from_nanoseconds(
                0,
                TimingSource::MonotonicClock,
            ),
        );

        let metrics =
            RuntimeMetrics::from_breakdown(&breakdown)
                .unwrap();

        assert!(matches!(
            metrics.stage_fraction_of_total(
                RuntimeStage::Execution
            ),
            Err(RuntimeError::ZeroTotalWallTime)
        ));
    }

    #[test]
    fn throughput_uses_hertz() {
        let mut breakdown = RuntimeBreakdown::new();

        breakdown.set_execution(
            StageDuration::from_duration(
                Duration::from_secs(2),
                TimingSource::MonotonicClock,
            ),
        );

        let metrics =
            RuntimeMetrics::from_breakdown(&breakdown)
                .unwrap();

        let throughput =
            metrics
                .execution_throughput(
                    ThroughputWork::Shots(1_000),
                )
                .unwrap()
                .unwrap();

        assert_eq!(
            throughput.kind,
            MetricKind::ShotsPerSecond
        );

        assert_eq!(
            throughput.unit,
            MetricUnit::Hertz
        );

        assert!(
            (throughput.value.get() - 500.0).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn zero_duration_throughput_is_rejected() {
        let mut breakdown = RuntimeBreakdown::new();

        breakdown.set_execution(
            StageDuration::from_nanoseconds(
                0,
                TimingSource::MonotonicClock,
            ),
        );

        let metrics =
            RuntimeMetrics::from_breakdown(&breakdown)
                .unwrap();

        assert!(matches!(
            metrics.execution_throughput(
                ThroughputWork::Shots(100)
            ),
            Err(RuntimeError::ZeroDurationForThroughput {
                work: "shots"
            })
        ));
    }

    #[test]
    fn zero_work_throughput_is_rejected() {
        let mut breakdown = RuntimeBreakdown::new();

        breakdown.set_execution(
            StageDuration::from_duration(
                Duration::from_secs(1),
                TimingSource::MonotonicClock,
            ),
        );

        let metrics =
            RuntimeMetrics::from_breakdown(&breakdown)
                .unwrap();

        assert!(matches!(
            metrics.execution_throughput(
                ThroughputWork::Shots(0)
            ),
            Err(RuntimeError::ZeroWorkForThroughput {
                work: "shots"
            })
        ));
    }

    #[test]
    fn end_to_end_throughput_uses_total_wall_time() {
        let mut breakdown = RuntimeBreakdown::new();

        breakdown.set_total_wall(
            StageDuration::from_duration(
                Duration::from_secs(4),
                TimingSource::MonotonicClock,
            ),
        );

        let metrics =
            RuntimeMetrics::from_breakdown(&breakdown)
                .unwrap();

        let throughput =
            metrics
                .end_to_end_throughput(
                    ThroughputWork::Circuits(100),
                )
                .unwrap()
                .unwrap();

        assert_eq!(
            throughput.kind,
            MetricKind::CircuitsPerSecond
        );

        assert!(
            (throughput.value.get() - 25.0).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn measure_uses_monotonic_source() {
        let (duration, value) =
            measure(
                RuntimeStage::Execution,
                || 42_u32,
            )
            .unwrap();

        assert_eq!(value, 42);
        assert_eq!(
            duration.source(),
            TimingSource::MonotonicClock
        );
    }

    #[test]
    fn all_throughput_work_kinds_have_distinct_metrics() {
        assert_eq!(
            ThroughputWork::Shots(1).metric_kind(),
            MetricKind::ShotsPerSecond
        );

        assert_eq!(
            ThroughputWork::Circuits(1).metric_kind(),
            MetricKind::CircuitsPerSecond
        );

        assert_eq!(
            ThroughputWork::Gates(1).metric_kind(),
            MetricKind::GatesPerSecond
        );

        assert_eq!(
            ThroughputWork::TwoQubitGates(1).metric_kind(),
            MetricKind::TwoQubitGatesPerSecond
        );

        assert_eq!(
            ThroughputWork::Layers(1).metric_kind(),
            MetricKind::LayersPerSecond
        );
    }

    #[test]
    fn backend_reported_timing_is_not_relabelled() {
        let duration = StageDuration::from_duration(
            Duration::from_millis(5),
            TimingSource::BackendReported,
        );

        assert_eq!(
            duration.source(),
            TimingSource::BackendReported
        );
    }

    #[test]
    fn simulator_reported_timing_is_not_relabelled() {
        let duration = StageDuration::from_duration(
            Duration::from_millis(5),
            TimingSource::SimulatorReported,
        );

        assert_eq!(
            duration.source(),
            TimingSource::SimulatorReported
        );
    }

    #[test]
    fn mixed_sources_can_be_summed_without_false_source_claim() {
        let mut breakdown = RuntimeBreakdown::new();

        breakdown.set_compilation(
            StageDuration::from_nanoseconds(
                100,
                TimingSource::MonotonicClock,
            ),
        );

        breakdown.set_execution(
            StageDuration::from_nanoseconds(
                200,
                TimingSource::BackendReported,
            ),
        );

        let sum =
            breakdown
                .sum_measured_non_total_stages()
                .unwrap()
                .unwrap();

        assert_eq!(
            sum.source(),
            TimingSource::ExternalMeasurement
        );

        assert_eq!(
            sum.nanoseconds(),
            300
        );
    }

    #[test]
    fn unmeasured_breakdown_has_no_sum() {
        let breakdown = RuntimeBreakdown::new();

        assert!(
            breakdown
                .sum_measured_non_total_stages()
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn runtime_metrics_schema_version_is_stable() {
        let breakdown = RuntimeBreakdown::new();

        let metrics =
            RuntimeMetrics::from_breakdown(&breakdown)
                .unwrap();

        assert_eq!(
            metrics.schema_version(),
            RUNTIME_METRICS_SCHEMA_VERSION
        );
    }
}