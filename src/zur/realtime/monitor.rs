
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Real-Time Execution Monitoring
//!
//! Path: src/zur/realtime/monitor.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//!
//! ## Purpose
//!
//! Provides real-time operational monitoring of ZUR execution sessions.
//!
//! Responsibilities include:
//!
//! - Elapsed execution time.
//! - Deadline compliance observations.
//! - Remaining execution time.
//! - Deadline-exceeded observations.
//! - Cooperative cancellation observations.
//! - Structured monitoring events.
//! - Integration with external observability systems.
//!
//! ## Ownership
//!
//! This module owns execution timing observations.
//!
//! It does not own:
//!
//! - Deadline creation or policy.
//! - Scheduling algorithms.
//! - Execution semantics.
//! - Cancellation authority.
//! - Physical interruption.
//! - Resource allocation.
//! - Distributed clock synchronization.
//! - Recovery.
//! - Hardware discovery.
//! - Provider selection.
//!
//! Those responsibilities remain with their canonical ZUR
//! and repository-wide subsystem owners.
//!
//! ## Scalability
//!
//! No fixed limits are imposed on:
//!
//! - Execution sessions.
//! - Execution identities.
//! - Monitoring events.
//! - Monitoring sinks.
//! - Computational domains.
//! - Providers.
//! - Hardware devices.
//! - Execution duration.
//!
//! This module does not maintain an unbounded internal event buffer.
//! Monitoring events are delivered to an injected sink.
//!
//! Actual storage and throughput depend on available resources.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No unsafe code.
//! No global mutable state.
//! No hidden timer threads.
//! No fabricated execution completion.
//! No automatic cancellation.
//! No implicit recovery.
//!
//! ## Integration
//!
//! Uses:
//!
//! - zur/error.rs
//! - zur/ids.rs
//! - zur/execution/session.rs
//!
//! Deadline integration is through DeadlineObservation.
//! The canonical deadlines module should implement that interface.
//!
//! Observability implementations provide MonitorSink.
//!
//! Execution engines invoke monitoring checkpoints at actual
//! execution lifecycle boundaries.

use core::fmt;

use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::zur::error::{ZurError, ZurResult};

use crate::zur::execution::session::ExecutionSession;

use crate::zur::ids::ExecutionId;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.realtime.monitor";

// ============================================================================
// DEADLINE OBSERVATION CONTRACT
// ============================================================================

/// Read-only deadline information supplied by the authoritative
/// deadline subsystem.
///
/// This trait does not create, modify, extend or enforce deadlines.
///
/// Implementations must use the same monotonic clock domain as the
/// timestamp supplied to the monitoring operation.
pub trait DeadlineObservation: Send + Sync {
    /// Returns the absolute monotonic deadline.
    fn deadline_at(&self) -> Instant;

    /// Returns the remaining time at the supplied observation instant.
    ///
    /// Implementations must use saturating time arithmetic.
    fn remaining_at(&self, observed_at: Instant) -> Duration {
        self.deadline_at()
            .saturating_duration_since(observed_at)
    }

    /// Returns whether the deadline has expired at the supplied instant.
    fn is_expired_at(&self, observed_at: Instant) -> bool {
        observed_at >= self.deadline_at()
    }
}

// ============================================================================
// DEADLINE STATE
// ============================================================================

/// Observed deadline state.
///
/// This represents an observation, not an execution-control command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DeadlineStatus {
    /// No deadline was supplied.
    Unbounded,

    /// The observed execution remains within its deadline.
    WithinDeadline,

    /// The deadline has expired.
    Exceeded,
}

impl DeadlineStatus {
    /// Returns whether the deadline was exceeded.
    #[must_use]
    pub const fn is_exceeded(self) -> bool {
        matches!(self, Self::Exceeded)
    }
}

// ============================================================================
// CANCELLATION OBSERVATION
// ============================================================================

/// Observed cancellation state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CancellationStatus {
    /// No cancellation was observed.
    NotRequested,

    /// Cancellation has been requested.
    Requested,
}

impl CancellationStatus {
    /// Returns whether cancellation was requested.
    #[must_use]
    pub const fn is_requested(self) -> bool {
        matches!(self, Self::Requested)
    }
}

// ============================================================================
// EXECUTION TIMING
// ============================================================================

/// Immutable execution timing observation.
///
/// All durations are measured using a monotonic clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionTiming {
    started_at: Instant,
    observed_at: Instant,
    elapsed: Duration,
}

impl ExecutionTiming {
    /// Creates a timing observation.
    ///
    /// # Errors
    ///
    /// Returns an error when the observation timestamp precedes
    /// the supplied execution start timestamp.
    pub fn new(
        started_at: Instant,
        observed_at: Instant,
    ) -> ZurResult<Self> {
        let elapsed = observed_at
            .checked_duration_since(started_at)
            .ok_or_else(|| {
                monitoring_error(
                    "observation timestamp precedes execution start",
                    "execution_timing",
                )
            })?;

        Ok(Self {
            started_at,
            observed_at,
            elapsed,
        })
    }

    /// Returns the execution start timestamp.
    #[must_use]
    pub const fn started_at(&self) -> Instant {
        self.started_at
    }

    /// Returns the observation timestamp.
    #[must_use]
    pub const fn observed_at(&self) -> Instant {
        self.observed_at
    }

    /// Returns elapsed execution time.
    #[must_use]
    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }
}

// ============================================================================
// MONITORING OBSERVATION
// ============================================================================

/// Complete immutable monitoring observation.
///
/// An observation does not claim that execution has completed.
/// It describes only what was observed at a particular checkpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitorObservation {
    execution_id: ExecutionId,
    session_id: String,
    timing: ExecutionTiming,
    deadline_status: DeadlineStatus,
    remaining: Option<Duration>,
    cancellation_status: CancellationStatus,
}

impl MonitorObservation {
    /// Returns the execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the session identity.
    #[must_use]
    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    /// Returns the timing observation.
    #[must_use]
    pub const fn timing(&self) -> &ExecutionTiming {
        &self.timing
    }

    /// Returns the deadline status.
    #[must_use]
    pub const fn deadline_status(&self) -> DeadlineStatus {
        self.deadline_status
    }

    /// Returns remaining deadline time.
    ///
    /// None means no deadline was supplied.
    /// Some(Duration::ZERO) means no time remains.
    #[must_use]
    pub const fn remaining(&self) -> Option<Duration> {
        self.remaining
    }

    /// Returns the observed cancellation status.
    #[must_use]
    pub const fn cancellation_status(&self) -> CancellationStatus {
        self.cancellation_status
    }

    /// Returns whether the deadline was exceeded.
    #[must_use]
    pub const fn deadline_exceeded(&self) -> bool {
        self.deadline_status.is_exceeded()
    }

    /// Returns whether cancellation was requested.
    #[must_use]
    pub const fn cancellation_requested(&self) -> bool {
        self.cancellation_status.is_requested()
    }
}

// ============================================================================
// MONITORING EVENT
// ============================================================================

/// Classification of an observed runtime event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MonitorEventKind {
    /// A regular execution checkpoint was observed.
    Checkpoint,

    /// The execution deadline was exceeded.
    DeadlineExceeded,

    /// Cancellation was observed.
    CancellationRequested,

    /// Both deadline expiration and cancellation were observed.
    DeadlineAndCancellation,
}

impl MonitorEventKind {
    /// Returns the event classification.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Checkpoint => "checkpoint",
            Self::DeadlineExceeded => "deadline_exceeded",
            Self::CancellationRequested => "cancellation_requested",
            Self::DeadlineAndCancellation => {
                "deadline_and_cancellation"
            }
        }
    }
}

impl fmt::Display for MonitorEventKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Immutable monitoring event delivered to an observability sink.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonitorEvent {
    kind: MonitorEventKind,
    observation: MonitorObservation,
}

impl MonitorEvent {
    /// Creates a monitoring event from an observation.
    #[must_use]
    pub fn new(observation: MonitorObservation) -> Self {
        let kind = match (
            observation.deadline_exceeded(),
            observation.cancellation_requested(),
        ) {
            (true, true) => MonitorEventKind::DeadlineAndCancellation,
            (true, false) => MonitorEventKind::DeadlineExceeded,
            (false, true) => MonitorEventKind::CancellationRequested,
            (false, false) => MonitorEventKind::Checkpoint,
        };

        Self { kind, observation }
    }

    /// Returns the event classification.
    #[must_use]
    pub const fn kind(&self) -> MonitorEventKind {
        self.kind
    }

    /// Returns the underlying observation.
    #[must_use]
    pub const fn observation(&self) -> &MonitorObservation {
        &self.observation
    }
}

// ============================================================================
// MONITORING SINK
// ============================================================================

/// Destination for runtime monitoring events.
///
/// Implementations may forward observations to:
///
/// - ZUR metrics.
/// - ZUR tracing.
/// - ZUR logging.
/// - External telemetry systems.
/// - Application-defined monitoring infrastructure.
///
/// Implementations must not fabricate successful execution results.
///
/// Implementations should avoid blocking execution-critical threads.
/// Buffering, persistence and backpressure policies belong to the sink.
///
/// A sink failure is returned to the caller rather than silently ignored.
pub trait MonitorSink: Send + Sync {
    /// Records an immutable monitoring event.
    ///
    /// # Errors
    ///
    /// Returns the original observability failure.
    fn record(&self, event: MonitorEvent) -> ZurResult<()>;
}

// ============================================================================
// MONITOR CONFIGURATION
// ============================================================================

/// Runtime monitoring configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonitorConfig {
    /// Whether ordinary execution checkpoints are recorded.
    pub record_checkpoints: bool,

    /// Whether deadline-exceeded observations are recorded.
    pub record_deadlines: bool,

    /// Whether cancellation observations are recorded.
    pub record_cancellation: bool,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            record_checkpoints: true,
            record_deadlines: true,
            record_cancellation: true,
        }
    }
}

// ============================================================================
// REAL-TIME MONITOR
// ============================================================================

/// Explicitly configured real-time execution monitor.
///
/// The monitor is safe to share across threads.
///
/// It owns no timer thread, execution state, deadline authority,
/// cancellation token, event buffer or global registry.
#[derive(Clone)]
pub struct RealtimeMonitor {
    config: MonitorConfig,
    sink: Option<Arc<dyn MonitorSink>>,
}

impl fmt::Debug for RealtimeMonitor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RealtimeMonitor")
            .field("config", &self.config)
            .field("has_sink", &self.sink.is_some())
            .finish()
    }
}

impl RealtimeMonitor {
    /// Creates a monitor without an external event sink.
    ///
    /// Observations are returned to the caller but are not persisted.
    #[must_use]
    pub const fn new(config: MonitorConfig) -> Self {
        Self {
            config,
            sink: None,
        }
    }

    /// Creates a monitor with an injected observability sink.
    #[must_use]
    pub fn with_sink(
        config: MonitorConfig,
        sink: Arc<dyn MonitorSink>,
    ) -> Self {
        Self {
            config,
            sink: Some(sink),
        }
    }

    /// Returns the monitoring configuration.
    #[must_use]
    pub const fn config(&self) -> &MonitorConfig {
        &self.config
    }

    /// Returns whether an external monitoring sink is configured.
    #[must_use]
    pub const fn has_sink(&self) -> bool {
        self.sink.is_some()
    }

    /// Observes an execution checkpoint.
    ///
    /// The caller supplies monotonic timestamps and the authoritative
    /// deadline source. The monitor never creates a hidden timer.
    ///
    /// Cancellation is observed from the supplied execution session.
    ///
    /// This method does not request cancellation or interrupt execution.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The observation timestamp precedes execution start.
    /// - The monitoring sink fails.
    pub fn observe<D>(
        &self,
        session: &ExecutionSession,
        execution_id: ExecutionId,
        started_at: Instant,
        observed_at: Instant,
        deadline: Option<&D>,
    ) -> ZurResult<MonitorObservation>
    where
        D: DeadlineObservation + ?Sized,
    {
        let timing = ExecutionTiming::new(
            started_at,
            observed_at,
        )?;

        let deadline_status = match deadline {
            Some(deadline) if deadline.is_expired_at(observed_at) => {
                DeadlineStatus::Exceeded
            }

            Some(_) => DeadlineStatus::WithinDeadline,

            None => DeadlineStatus::Unbounded,
        };

        let remaining = deadline.map(|value| {
            value.remaining_at(observed_at)
        });

        let cancellation_status = if session.is_cancelled() {
            CancellationStatus::Requested
        } else {
            CancellationStatus::NotRequested
        };

        let observation = MonitorObservation {
            execution_id,
            session_id: session.session_id().to_string(),
            timing,
            deadline_status,
            remaining,
            cancellation_status,
        };

        let event = MonitorEvent::new(observation.clone());

        if self.should_record(event.kind()) {
            if let Some(sink) = &self.sink {
                sink.record(event)?;
            }
        }

        Ok(observation)
    }

    /// Determines whether an event is enabled by configuration.
    #[must_use]
    pub const fn should_record(
        &self,
        kind: MonitorEventKind,
    ) -> bool {
        match kind {
            MonitorEventKind::Checkpoint => {
                self.config.record_checkpoints
            }

            MonitorEventKind::DeadlineExceeded => {
                self.config.record_deadlines
            }

            MonitorEventKind::CancellationRequested => {
                self.config.record_cancellation
            }

            MonitorEventKind::DeadlineAndCancellation => {
                self.config.record_deadlines
                    && self.config.record_cancellation
            }
        }
    }
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn monitoring_error(
    message: impl Into<String>,
    operation: &'static str,
) -> ZurError {
    ZurError::invalid_input(message)
        .with_component(COMPONENT)
        .with_operation(operation)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Mutex;

    /// Test deadline implementation.
    struct TestDeadline {
        deadline: Instant,
    }

    impl DeadlineObservation for TestDeadline {
        fn deadline_at(&self) -> Instant {
            self.deadline
        }
    }

    /// Test monitoring sink.
    #[derive(Default)]
    struct TestSink {
        events: Mutex<Vec<MonitorEvent>>,
    }

    impl MonitorSink for TestSink {
        fn record(&self, event: MonitorEvent) -> ZurResult<()> {
            self.events
                .lock()
                .map_err(|_| {
                    monitoring_error(
                        "test sink lock was poisoned",
                        "test_sink",
                    )
                })?
                .push(event);

            Ok(())
        }
    }

    #[test]
    fn deadline_status_distinguishes_unbounded_execution() {
        assert_eq!(
            DeadlineStatus::Unbounded,
            DeadlineStatus::Unbounded
        );

        assert!(!DeadlineStatus::Unbounded.is_exceeded());
    }

    #[test]
    fn deadline_status_identifies_expiration() {
        assert!(DeadlineStatus::Exceeded.is_exceeded());
        assert!(!DeadlineStatus::WithinDeadline.is_exceeded());
    }

    #[test]
    fn deadline_remaining_uses_saturating_arithmetic() {
        let now = Instant::now();

        let deadline = TestDeadline {
            deadline: now,
        };

        assert_eq!(
            deadline.remaining_at(now + Duration::from_secs(1)),
            Duration::ZERO
        );

        assert!(
            deadline.is_expired_at(
                now + Duration::from_secs(1)
            )
        );
    }

    #[test]
    fn timing_rejects_observations_before_start() {
        let start = Instant::now();
        let observed = start.checked_sub(Duration::from_secs(1));

        if let Some(observed) = observed {
            assert!(
                ExecutionTiming::new(start, observed).is_err()
            );
        }
    }

    #[test]
    fn timing_records_elapsed_duration() {
        let start = Instant::now();

        let observed = start + Duration::from_secs(3);

        let timing = ExecutionTiming::new(
            start,
            observed,
        )
        .expect("valid monotonic timing");

        assert_eq!(
            timing.elapsed(),
            Duration::from_secs(3)
        );
    }

    #[test]
    fn event_classification_preserves_both_conditions() {
        let start = Instant::now();

        let observation = MonitorObservation {
            execution_id: test_execution_id(),
            session_id: "test-session".to_owned(),
            timing: ExecutionTiming::new(
                start,
                start,
            )
            .expect("valid timing"),
            deadline_status: DeadlineStatus::Exceeded,
            remaining: Some(Duration::ZERO),
            cancellation_status: CancellationStatus::Requested,
        };

        let event = MonitorEvent::new(observation);

        assert_eq!(
            event.kind(),
            MonitorEventKind::DeadlineAndCancellation
        );
    }

    #[test]
    fn monitor_configuration_is_respected() {
        let monitor = RealtimeMonitor::new(
            MonitorConfig {
                record_checkpoints: false,
                record_deadlines: true,
                record_cancellation: false,
            },
        );

        assert!(
            !monitor.should_record(MonitorEventKind::Checkpoint)
        );

        assert!(
            monitor.should_record(
                MonitorEventKind::DeadlineExceeded
            )
        );

        assert!(
            !monitor.should_record(
                MonitorEventKind::CancellationRequested
            )
        );

        assert!(
            !monitor.should_record(
                MonitorEventKind::DeadlineAndCancellation
            )
        );
    }

    fn test_execution_id() -> ExecutionId {
        // Identity construction belongs to the canonical ZUR ID allocator.
        // The test obtains an identity through that authority.
        use crate::zur::ids::ExecutionIdAllocator;

        ExecutionIdAllocator::new()
            .expect("valid test identity allocator")
            .allocate()
            .expect("execution identity allocation")
    }
}
