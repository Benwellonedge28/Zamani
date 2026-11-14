#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Real-Time Execution
//!
//! Path: `src/zur/realtime/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! # Purpose
//!
//! This module is the public composition boundary for ZUR's
//! real-time execution facilities.
//!
//! It exposes the existing deadline and monitoring implementations
//! through stable, deliberately separated public interfaces.
//!
//! It does not introduce another deadline engine, scheduler,
//! monitoring engine, cancellation system, or observability registry.
//!
//! # Architectural ownership
//!
//! The child modules retain their individual responsibilities.
//!
//! ## deadlines
//!
//! `deadlines.rs` owns:
//!
//! - Monotonic deadline representation.
//! - Bounded and unbounded deadlines.
//! - Relative and absolute deadline construction.
//! - Deadline inheritance.
//! - Deadline composition.
//! - Remaining-time calculations.
//! - Deadline expiration evaluation.
//! - Cancellation-aware deadline checks.
//!
//! ## monitor
//!
//! `monitor.rs` owns:
//!
//! - Execution timing observations.
//! - Deadline-compliance observations.
//! - Remaining-time observations.
//! - Cancellation-state observations.
//! - Monitoring event construction.
//! - Monitoring sink integration.
//!
//! # This module does not own
//!
//! - Source-language deadline syntax.
//! - Semantic validation.
//! - Execution-plan construction.
//! - Compiler feasibility analysis.
//! - Scheduling algorithms.
//! - Task execution.
//! - Cancellation authority.
//! - Physical interruption.
//! - Resource allocation.
//! - Hardware discovery.
//! - Provider selection.
//! - Distributed clock synchronization.
//! - Distributed deadline propagation.
//! - Recovery and retry policy.
//! - Quantum scheduling.
//! - Hybrid scheduling.
//!
//! Those responsibilities remain with their respective
//! authoritative repository subsystems.
//!
//! # Integration authorities
//!
//! - `crate::zur::error`:
//!   Structured runtime errors.
//!
//! - `crate::zur::ids`:
//!   Runtime execution identities.
//!
//! - `crate::zur::execution::session`:
//!   Session lifecycle and cooperative cancellation.
//!
//! - `crate::zur::contracts::execution`:
//!   Execution lifecycle and execution contracts.
//!
//! - `crate::zur::scheduling`:
//!   Operational scheduling.
//!
//! - `crate::zur::observability`:
//!   Logging, metrics, tracing and profiling.
//!
//! - `crate::stdlib::context`:
//!   Operation context and cancellation observation.
//!
//! - `crate::distributed::execution`:
//!   Distributed execution and its authoritative
//!   deadline propagation mechanisms.
//!
//! # Grammar boundary
//!
//! Source-level execution intent remains owned by the grammar
//! and frontend/semantic subsystems.
//!
//! Relevant grammar authorities include:
//!
//! - `grammar/execution/scheduling.g4`.
//! - `grammar/execution/execution-context.g4`.
//! - `grammar/specification/execution-model.md`.
//!
//! These files describe source-level intent and meaning.
//!
//! This module consumes already validated runtime information.
//! It must never parse grammar files or independently interpret
//! source-level declarations.
//!
//! # Clock semantics
//!
//! Local deadline calculations use `std::time::Instant`.
//!
//! An `Instant` is meaningful only within its applicable
//! monotonic clock environment.
//!
//! It must not be serialized or transmitted as a portable
//! distributed timestamp.
//!
//! Distributed integrations must use the canonical distributed
//! deadline propagation protocol and appropriate remaining-time
//! information.
//!
//! # Scalability
//!
//! This composition module introduces no fixed limits on:
//!
//! - Deadlines.
//! - Execution sessions.
//! - Monitoring observations.
//! - Monitoring sinks.
//! - Providers.
//! - Computational domains.
//! - Devices.
//! - Distributed participants.
//! - Execution duration.
//!
//! It maintains no internal event buffer.
//!
//! Storage, throughput and execution remain subject to actual
//! memory, platform and physical-resource availability.
//!
//! Literal infinite physical computation is not promised.
//!
//! # Safety
//!
//! Safe Rust only.
//!
//! No unsafe code.
//! No raw-pointer ownership.
//! No global mutable state.
//! No hidden timer threads.
//! No hidden scheduler.
//! No automatic cancellation.
//! No fabricated execution completion.
//! No implicit recovery.
//!
//! # Composition invariants
//!
//! 1. Deadline creation belongs to `deadlines`.
//! 2. Monitoring belongs to `monitor`.
//! 3. Monitoring never changes a deadline.
//! 4. Monitoring never requests cancellation.
//! 5. Deadline expiration does not prove execution has stopped.
//! 6. Cancellation requests do not prove execution has stopped.
//! 7. Scheduling remains independently owned.
//! 8. Observability sinks are explicitly injected.
//! 9. Unbounded deadlines remain unbounded.
//! 10. A bounded deadline cannot be extended by a child operation.
//! 11. Local monotonic timestamps are never treated as portable
//!     distributed timestamps.
//! 12. Adding a computational domain does not require changing
//!     this composition module.

// ============================================================================
// CHILD MODULES
// ============================================================================

/// Monotonic execution deadlines and remaining-time contracts.
///
/// Owns deadline construction, composition, expiration and
/// cancellation-aware deadline checks.
///
/// This module does not implement scheduling or monitoring.
pub mod deadlines;

/// Real-time execution monitoring and timing observations.
///
/// Owns operational monitoring events, deadline observations,
/// cancellation observations and injected monitoring sinks.
///
/// This module does not own deadline construction, cancellation
/// authority or physical execution control.
pub mod monitor;

// ============================================================================
// DEADLINE PUBLIC API
// ============================================================================

/// Combines applicable deadlines using the earliest bounded deadline.
pub use deadlines::combine_deadlines;

/// Monotonic execution deadline.
pub use deadlines::Deadline;

/// Snapshot of remaining deadline time.
pub use deadlines::RemainingTime;

// ============================================================================
// MONITORING PUBLIC API
// ============================================================================

/// Read-only deadline observation interface.
pub use monitor::DeadlineObservation;

/// Observed cancellation state.
pub use monitor::CancellationStatus;

/// Observed deadline state.
pub use monitor::DeadlineStatus;

/// Immutable execution timing observation.
pub use monitor::ExecutionTiming;

/// Classification of a monitoring event.
pub use monitor::MonitorEventKind;

/// Immutable monitoring event.
pub use monitor::MonitorEvent;

/// Complete immutable monitoring observation.
pub use monitor::MonitorObservation;

/// Monitoring configuration.
pub use monitor::MonitorConfig;

/// Destination for monitoring events.
pub use monitor::MonitorSink;

/// Explicitly configured real-time monitor.
pub use monitor::RealtimeMonitor;

// ============================================================================
// BOUNDED DEADLINE ADAPTER
// ============================================================================

/// A read-only monitoring adapter for a bounded ZUR deadline.
///
/// The canonical `Deadline` type also represents unbounded execution.
/// The monitoring interface, however, represents a supplied deadline
/// through an absolute monotonic instant.
///
/// This adapter makes that distinction explicit:
///
/// - A bounded `Deadline` can be converted into this adapter.
/// - An unbounded `Deadline` produces `None`.
/// - No deadline is invented.
/// - No deadline is modified or extended.
///
/// The adapter does not own deadline policy or deadline enforcement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundedDeadlineObservation {
    deadline_at: std::time::Instant,
}

impl BoundedDeadlineObservation {
    /// Creates a monitoring adapter from a bounded deadline.
    ///
    /// Returns `None` when the supplied deadline is unbounded.
    ///
    /// The original deadline is not modified.
    #[must_use]
    pub fn from_deadline(deadline: Deadline) -> Option<Self> {
        deadline
            .instant()
            .map(|deadline_at| Self { deadline_at })
    }

    /// Returns the absolute monotonic deadline instant.
    ///
    /// The instant is local to its applicable clock environment.
    /// It must not be serialized as a distributed timestamp.
    #[must_use]
    pub const fn deadline_at(&self) -> std::time::Instant {
        self.deadline_at
    }
}

impl monitor::DeadlineObservation for BoundedDeadlineObservation {
    /// Returns the absolute monotonic deadline.
    fn deadline_at(&self) -> std::time::Instant {
        self.deadline_at
    }
}

// ============================================================================
// PUBLIC API TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::{Duration, Instant};

    #[test]
    fn realtime_modules_are_composed() {
        let deadline = Deadline::unbounded();

        assert!(deadline.is_unbounded());
    }

    #[test]
    fn bounded_deadline_can_be_adapted_for_monitoring() {
        let instant = Instant::now()
            .checked_add(Duration::from_secs(10))
            .expect("test instant must be representable");

        let deadline = Deadline::at(instant);

        let observation =
            BoundedDeadlineObservation::from_deadline(deadline)
                .expect("deadline is bounded");

        assert_eq!(observation.deadline_at(), instant);
    }

    #[test]
    fn unbounded_deadline_has_no_bounded_monitoring_adapter() {
        let deadline = Deadline::unbounded();

        assert!(
            BoundedDeadlineObservation::from_deadline(deadline)
                .is_none()
        );
    }

    #[test]
    fn bounded_deadline_adapter_implements_monitoring_contract() {
        let instant = Instant::now()
            .checked_add(Duration::from_secs(5))
            .expect("test instant must be representable");

        let deadline = Deadline::at(instant);

        let observation =
            BoundedDeadlineObservation::from_deadline(deadline)
                .expect("deadline is bounded");

        let trait_object: &dyn DeadlineObservation = &observation;

        assert_eq!(trait_object.deadline_at(), instant);

        assert!(!trait_object.is_expired_at(Instant::now()));
    }

    #[test]
    fn expired_deadline_is_observed_as_expired() {
        let instant = Instant::now();

        let deadline = Deadline::at(instant);

        let observation =
            BoundedDeadlineObservation::from_deadline(deadline)
                .expect("deadline is bounded");

        let trait_object: &dyn DeadlineObservation = &observation;

        assert!(trait_object.is_expired_at(instant));
        assert_eq!(
            trait_object.remaining_at(
                instant + Duration::from_secs(1)
            ),
            Duration::ZERO
        );
    }

    #[test]
    fn deadline_composition_is_available_from_public_module() {
        let origin = Instant::now();

        let earlier = Deadline::at(
            origin + Duration::from_secs(5)
        );

        let later = Deadline::at(
            origin + Duration::from_secs(10)
        );

        let effective = combine_deadlines([
            earlier,
            later,
        ]);

        assert_eq!(effective, earlier);
    }

    #[test]
    fn deadline_remaining_time_is_available_from_public_module() {
        let deadline = Deadline::after(
            Duration::from_secs(10)
        )
        .expect("test deadline must be representable");

        let remaining = deadline
            .remaining_time()
            .expect("deadline is bounded");

        assert!(!remaining.is_exhausted());
    }
}