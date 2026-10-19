
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Simulation Results
//!
//! Path: `src/hdl/simulation/results.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define engine-independent, structured outcomes and counters for an HDL
//! simulation run.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/simulation.g4`
//! - `grammar/execution/simulation.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/simulation/interface.rs`
//!
//! The grammar owns source syntax and simulation intent. The canonical AST
//! owns source structure, and the semantic layer owns source-level validity.
//! The simulation engine owns execution semantics. The simulation interface
//! drives the session; this module defines the result vocabulary it returns.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - `StopReason`: why execution stopped before normal completion.
//! - `RunStatus`: the driver's final execution status.
//! - `RunCounters`: counts of work reported by the driver and engine.
//! - `RunReport`: the final status, time, and counters for one run.
//!
//! This module does not own:
//!
//! - Simulation execution, event scheduling, or event-ordering semantics.
//! - HDLIR or any other intermediate representation.
//! - Assertions, coverage, formal verification, or proof results.
//! - Signal snapshots, waveform storage, or trace serialization.
//! - Physical resource estimates, allocation, synthesis, placement, or routing.
//! - Source parsing, AST construction, or semantic validation.
//!
//! ## Status semantics
//!
//! `Completed` means the engine reported its normal terminal condition.
//! `Quiescent` means the engine reported that no event can currently advance
//! the session under its documented semantics. Neither status proves that the
//! design is correct, that assertions passed, or that physical realization is
//! feasible.
//!
//! `Stopped` means the driver stopped without establishing normal completion.
//! Its reason distinguishes cooperative cancellation from configured step or
//! event budget exhaustion.
//!
//! ## Scalability
//!
//! Counters use `u128` to provide a wide representable range without tying
//! counts to the host pointer width. They are still finite: callers and
//! drivers must detect overflow rather than wrap or silently saturate.
//!
//! This module imposes no universal limit on design size, signal count, event
//! queue size, simulation time, or waveform length. Such limits, when needed,
//! belong to explicit operational policies or to the relevant engine/storage
//! implementation. `Time` remains generic so this result contract does not
//! prescribe a numeric representation or timescale.
//!
//! ## Determinism
//!
//! The result types are value types with deterministic equality semantics
//! where their generic parameters support it. This module does not promise
//! deterministic simulation: that depends on the engine, its inputs, its
//! configuration, and the event semantics it implements.
//!
//! ## Integration contract
//!
//! `interface.rs` should import and publicly re-export these types:
//!
//! ```ignore
//! pub use super::results::{RunCounters, RunReport, RunStatus, StopReason};
//! ```
//!
//! The run driver remains responsible for incrementing counters with checked
//! arithmetic and for selecting a status. It must never turn cancellation,
//! budget exhaustion, counter overflow, or engine failure into a claim of
//! normal completion.
//!
//! ## Freeze criteria
//!
//! - Each status has a documented meaning distinct from verification results.
//! - Counter units and overflow policy are explicit.
//! - `RunReport` contains only generic run-summary data.
//! - Existing `interface::RunStatus`, `interface::RunCounters`,
//!   `interface::RunReport`, and `interface::StopReason` paths remain usable
//!   through re-exports.
//! - Tests cover statuses, counter defaults, and report construction.
//! - The public API remains independent of HDLIR and target realization.

// =============================================================================
// Stop reason
// =============================================================================

/// Reason an otherwise active simulation run stopped before normal completion.
///
/// A stop reason is an execution outcome, not a source-language diagnostic
/// and not evidence that a design is semantically invalid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StopReason {
    /// Cooperative cancellation was requested by the caller or host.
    ///
    /// Cancellation is cooperative. It cannot guarantee immediate interruption
    /// of a blocking or non-cooperative engine operation.
    Cancelled,

    /// The configured maximum number of successful advancement calls was
    /// reached before the engine reported normal completion or quiescence.
    StepBudgetExhausted,

    /// The configured event-processing budget was exceeded.
    ///
    /// Depending on the engine's advancement granularity, events counted by
    /// the report may exceed the configured budget: the driver can only account
    /// for events after the engine reports how many it processed. Such a run
    /// must remain stopped, not be reported as completed.
    EventBudgetExhausted,
}

// =============================================================================
// Run status
// =============================================================================

/// Final execution status reported by the simulation driver.
///
/// These statuses describe execution only. They do not indicate whether
/// assertions passed, whether a formal property was proved, or whether a
/// physical target can realize the design.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RunStatus {
    /// The engine reported its normal terminal condition.
    Completed,

    /// The engine reported that no event can currently advance the session.
    ///
    /// The engine defines how future external input or later stimulation
    /// affects quiescence. Quiescence is not automatically completion or proof
    /// of correctness.
    Quiescent,

    /// The driver stopped without establishing normal completion.
    Stopped(StopReason),
}

impl RunStatus {
    /// Return `true` if the engine reported its normal terminal condition.
    ///
    /// This method does not imply that the simulated design is correct.
    #[must_use]
    pub const fn is_completed(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Return `true` if the engine reported a quiescent session.
    #[must_use]
    pub const fn is_quiescent(self) -> bool {
        matches!(self, Self::Quiescent)
    }

    /// Return the stop reason when the run stopped before normal completion.
    #[must_use]
    pub const fn stop_reason(self) -> Option<StopReason> {
        match self {
            Self::Stopped(reason) => Some(reason),
            Self::Completed | Self::Quiescent => None,
        }
    }
}

// =============================================================================
// Run counters
// =============================================================================

/// Counts of simulation work reported by the driver and engine.
///
/// `steps` counts successful calls to the session's advancement method,
/// including calls that report quiescence or normal termination. An
/// advancement call returning an engine error is not counted as successful.
///
/// `events_processed` is the sum of event counts reported by successful
/// advancement calls. It counts engine-reported processed events, not queued
/// events, scheduled events, signal changes, or physical hardware operations.
///
/// These counters are not physical resource measurements.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RunCounters {
    /// Number of successful session advancement calls.
    pub steps: u128,

    /// Number of events reported as processed by successful advancement calls.
    pub events_processed: u128,
}

impl RunCounters {
    /// Create counters initialized to zero.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            steps: 0,
            events_processed: 0,
        }
    }

    /// Return the checked sum of two counter sets.
    ///
    /// Returns `None` if either addition exceeds the representable `u128`
    /// range. It never wraps or saturates silently.
    ///
    /// This operation is useful when aggregating independently completed run
    /// segments. Callers must ensure that the segments are disjoint if they
    /// intend the result to represent total work rather than duplicated work.
    #[must_use]
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        let steps = match self.steps.checked_add(other.steps) {
            Some(value) => value,
            None => return None,
        };

        let events_processed =
            match self.events_processed.checked_add(other.events_processed) {
                Some(value) => value,
                None => return None,
            };

        Some(Self {
            steps,
            events_processed,
        })
    }
}

// =============================================================================
// Run report
// =============================================================================

/// Summary of one simulation run.
///
/// `Time` is defined by the simulation engine. It may be an integer tick count,
/// a rational or symbolic time value, or another representation that preserves
/// the engine's documented time semantics.
///
/// The report intentionally excludes observations, waveforms, assertions,
/// coverage, physical resource estimates, and realization details. Those
/// results have different ownership, size, and lifecycle requirements and
/// should be requested from their respective APIs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunReport<Time> {
    /// Final execution status selected by the simulation driver.
    pub status: RunStatus,

    /// Last simulation time observed by the driver when producing this report.
    pub final_time: Time,

    /// Work counters collected during this run.
    pub counters: RunCounters,
}

impl<Time> RunReport<Time> {
    /// Construct a report from its final status, time, and counters.
    ///
    /// This constructor performs no execution and does not independently
    /// validate that the supplied status or counters accurately describe a
    /// run. The simulation driver is responsible for constructing truthful
    /// reports from its execution state.
    #[must_use]
    pub const fn new(
        status: RunStatus,
        final_time: Time,
        counters: RunCounters,
    ) -> Self {
        Self {
            status,
            final_time,
            counters,
        }
    }

    /// Return `true` when the engine reported normal completion.
    ///
    /// A completed run is not necessarily a correct or verified design.
    #[must_use]
    pub const fn is_completed(&self) -> bool {
        self.status.is_completed()
    }

    /// Return `true` when the engine reported quiescence.
    #[must_use]
    pub const fn is_quiescent(&self) -> bool {
        self.status.is_quiescent()
    }

    /// Return the reason if the run stopped before normal completion.
    #[must_use]
    pub const fn stop_reason(&self) -> Option<StopReason> {
        self.status.stop_reason()
    }

    /// Consume the report and return its status, final time, and counters.
    #[must_use]
    pub fn into_parts(self) -> (RunStatus, Time, RunCounters) {
        (self.status, self.final_time, self.counters)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stop_reasons_are_distinct() {
        assert_ne!(StopReason::Cancelled, StopReason::StepBudgetExhausted);
        assert_ne!(
            StopReason::StepBudgetExhausted,
            StopReason::EventBudgetExhausted
        );
        assert_ne!(StopReason::Cancelled, StopReason::EventBudgetExhausted);
    }

    #[test]
    fn status_helpers_preserve_execution_meaning() {
        assert!(RunStatus::Completed.is_completed());
        assert!(!RunStatus::Completed.is_quiescent());
        assert_eq!(RunStatus::Completed.stop_reason(), None);

        assert!(RunStatus::Quiescent.is_quiescent());
        assert!(!RunStatus::Quiescent.is_completed());
        assert_eq!(RunStatus::Quiescent.stop_reason(), None);

        let status = RunStatus::Stopped(StopReason::Cancelled);
        assert!(!status.is_completed());
        assert!(!status.is_quiescent());
        assert_eq!(status.stop_reason(), Some(StopReason::Cancelled));
    }

    #[test]
    fn counters_default_to_zero() {
        assert_eq!(RunCounters::default(), RunCounters::new());
        assert_eq!(RunCounters::new().steps, 0);
        assert_eq!(RunCounters::new().events_processed, 0);
    }

    #[test]
    fn counter_addition_is_checked() {
        let left = RunCounters {
            steps: 2,
            events_processed: 5,
        };
        let right = RunCounters {
            steps: 3,
            events_processed: 7,
        };

        assert_eq!(
            left.checked_add(right),
            Some(RunCounters {
                steps: 5,
                events_processed: 12,
            })
        );
    }

    #[test]
    fn counter_addition_does_not_wrap() {
        let left = RunCounters {
            steps: u128::MAX,
            events_processed: 0,
        };
        let right = RunCounters {
            steps: 1,
            events_processed: 0,
        };

        assert_eq!(left.checked_add(right), None);
    }

    #[test]
    fn event_counter_addition_is_checked_independently() {
        let left = RunCounters {
            steps: 0,
            events_processed: u128::MAX,
        };
        let right = RunCounters {
            steps: 0,
            events_processed: 1,
        };

        assert_eq!(left.checked_add(right), None);
    }

    #[test]
    fn report_preserves_generic_time_and_counts() {
        let report = RunReport::new(
            RunStatus::Completed,
            (12_u128, 3_u32),
            RunCounters {
                steps: 4,
                events_processed: 9,
            },
        );

        assert!(report.is_completed());
        assert!(!report.is_quiescent());
        assert_eq!(report.stop_reason(), None);
        assert_eq!(report.final_time, (12, 3));
        assert_eq!(report.counters.steps, 4);
        assert_eq!(report.counters.events_processed, 9);
    }

    #[test]
    fn report_keeps_budget_exhaustion_distinct_from_completion() {
        let report = RunReport::new(
            RunStatus::Stopped(StopReason::EventBudgetExhausted),
            "time-after-last-processed-event",
            RunCounters {
                steps: 8,
                events_processed: 101,
            },
        );

        assert!(!report.is_completed());
        assert_eq!(
            report.stop_reason(),
            Some(StopReason::EventBudgetExhausted)
        );
        assert_eq!(report.final_time, "time-after-last-processed-event");
    }

    #[test]
    fn report_can_be_decomposed_without_cloning_time() {
        let report = RunReport::new(
            RunStatus::Quiescent,
            String::from("logical-time"),
            RunCounters::new(),
        );

        let (status, time, counters) = report.into_parts();

        assert_eq!(status, RunStatus::Quiescent);
        assert_eq!(time, "logical-time");
        assert_eq!(counters, RunCounters::new());
    }
}
