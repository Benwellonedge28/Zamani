
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Simulation Interface
//!
//! Path: `src/hdl/simulation/interface.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define the engine-independent lifecycle and execution contract for HDL
//! simulations: engine startup, session advancement, observation, cancellation,
//! operational budgets, and structured run outcomes.
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
//!
//! The grammar owns source syntax. The canonical AST owns source structure.
//! The semantic layer owns source-level validation. An engine owns its
//! execution model and the meaning of its simulation events.
//!
//! ## Owns
//!
//! - Engine and session integration traits.
//! - Cooperative cancellation.
//! - Explicit execution budgets.
//! - Per-run counters and terminal statuses.
//! - A generic driver for advancing a simulation session.
//!
//! ## Does not own
//!
//! - The canonical AST or semantic model.
//! - HDLIR or any other intermediate representation.
//! - Event-queue storage or HDL event-region semantics.
//! - Logic evaluation, signal storage, memories, clocks, or resets.
//! - Assertions, formal verification, or proof results.
//! - Target selection, synthesis, placement, routing, or hardware allocation.
//! - Waveform storage or a universal snapshot representation.
//!
//! ## Scalability
//!
//! This interface imposes no universal maximum on design size, signal count,
//! clock count, or event count. Counters and budgets use `u128` and checked
//! arithmetic. `None` means that this interface has no explicit budget for
//! that category; it does not promise infinite memory, time, or execution.
//!
//! Operational budgets are not language-semantic limits. If a budget is
//! exceeded, the driver reports a stopped run rather than claiming completion.
//!
//! ## Integration
//!
//! A concrete engine implements [`SimulationEngine`]. Its associated session
//! implements [`SimulationSession`]. An adapter must prepare a design using
//! the repository's existing frontend and semantic contracts before handing
//! it to an engine when source-level validation is required.
//!
//! This module deliberately does not assume an AST constructor, semantic-model
//! constructor, or HDLIR API.
//!
//! ## Determinism
//!
//! Event ordering and simulation-time semantics remain engine-defined.
//! Engines must document their determinism guarantees. This driver preserves
//! the order of calls and counters it observes; it cannot make a nondeterministic
//! engine deterministic.
//!
//! ## Freeze criteria
//!
//! - Existing public type names and engine/session contracts remain available.
//! - Cancellation and budget exhaustion are not reported as completion.
//! - Counter overflow cannot silently wrap.
//! - Engine failures remain errors, not successful terminal statuses.
//! - Tests cover completion, quiescence, cancellation, budgets and failures.
//! - No HDLIR implementation is introduced here.

use std::error::Error;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// =============================================================================
// Cancellation
// =============================================================================

/// A clonable, thread-safe cooperative cancellation token.
///
/// Cancelling any clone requests cancellation for every clone. Cancellation
/// is cooperative: it cannot forcibly interrupt an engine that is blocked
/// or does not check the token.
#[derive(Clone, Debug, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Create a new token that has not been cancelled.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Request cancellation.
    ///
    /// Repeated calls are safe. Cancellation remains requested for all clones.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Return whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

// =============================================================================
// Configuration and operational budgets
// =============================================================================

/// Operational limits for one simulation run.
///
/// These are execution budgets, not restrictions on the HDL language or
/// declared hardware capacity. `None` imposes no limit through this budget.
/// Actual execution remains subject to available system resources and engine
/// capabilities.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SimulationBudget {
    /// Maximum successful session advancement calls.
    ///
    /// A call returning `Progress`, `Quiescent`, or `Finished` counts as one
    /// successful advancement call. An engine error does not count.
    pub max_steps: Option<u128>,

    /// Maximum number of events reported as processed.
    ///
    /// An engine may process several events in one advancement call. Since
    /// the driver cannot interrupt an already-running call, the reported
    /// event count can exceed this budget by the amount processed in that
    /// call. Such a run is reported as stopped, not completed.
    pub max_events: Option<u128>,
}

/// Configuration for one simulation invocation.
///
/// Equality is intentionally not derived: the cancellation token is shared,
/// mutable execution state, and value equality for a live token would need
/// additional semantics that this interface does not promise.
#[derive(Clone, Debug)]
pub struct SimulationConfig {
    /// Operational budgets for this invocation.
    pub budget: SimulationBudget,

    /// Cooperative cancellation token shared with the caller and engine.
    pub cancellation: CancellationToken,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            budget: SimulationBudget::default(),
            cancellation: CancellationToken::new(),
        }
    }
}

impl SimulationConfig {
    /// Create a configuration without explicit step or event budgets.
    ///
    /// This does not promise unbounded physical resources.
    #[must_use]
    pub fn unbounded() -> Self {
        Self::default()
    }

    /// Create a configuration with the supplied budgets and a fresh token.
    #[must_use]
    pub fn with_budget(budget: SimulationBudget) -> Self {
        Self {
            budget,
            cancellation: CancellationToken::new(),
        }
    }

    /// Return a clone of the cancellation token for external cancellation.
    #[must_use]
    pub fn cancellation_token(&self) -> CancellationToken {
        self.cancellation.clone()
    }
}

// =============================================================================
// Engine and session contracts
// =============================================================================

/// Outcome of one session advancement operation.
///
/// The session implementation defines what constitutes progress under its
/// documented simulation semantics. It must not report quiescence merely
/// because it chose not to process work.
///
/// `events_processed` is the number of events actually processed during this
/// advancement. Zero-event progress is valid when the engine's semantics
/// permit progress without processing an event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StepOutcome {
    /// The engine performed an advancement operation.
    Progress {
        /// Number of events actually processed by this operation.
        events_processed: u128,
    },

    /// No further event can currently advance this session.
    ///
    /// Quiescence is not a correctness proof. The engine must define how
    /// external inputs, future events and pending work affect this status.
    Quiescent,

    /// The engine reached its normal terminal condition.
    Finished,
}

/// A running HDL simulation session.
///
/// The engine defines its time and observation representations. This interface
/// does not force all simulation times into a host integer or floating-point
/// value, nor does it require every observation to copy the full design state.
pub trait SimulationSession {
    /// Engine-defined logical simulation-time representation.
    type Time: Clone + fmt::Debug;

    /// Engine-defined representation returned by an observation request.
    type Observation;

    /// Error type produced by this session.
    type Error: Error + Send + Sync + 'static;

    /// Return the current logical simulation time.
    fn current_time(&self) -> &Self::Time;

    /// Advance according to the engine's documented event semantics.
    ///
    /// Implementations should check `cancellation` during lengthy operations.
    /// Engine failures must be returned as errors rather than disguised as
    /// quiescence or normal completion.
    fn step(
        &mut self,
        cancellation: &CancellationToken,
    ) -> Result<StepOutcome, Self::Error>;

    /// Observe the current simulation state.
    ///
    /// The consistency guarantees and cost of an observation are engine-defined.
    /// Implementations should support selective observations when practical,
    /// instead of requiring callers to copy all signals or histories.
    fn observe(&self) -> Result<Self::Observation, Self::Error>;
}

/// An engine capable of starting a simulation for its own design type.
///
/// `Design` is associated with the engine so adapters can use existing
/// repository-owned representations without introducing another AST or IR.
pub trait SimulationEngine {
    /// Design representation accepted by this engine.
    type Design;

    /// Session created by this engine.
    type Session: SimulationSession<Error = Self::Error>;

    /// Startup and execution error type.
    type Error: Error + Send + Sync + 'static;

    /// Start a session for an engine-compatible design.
    ///
    /// The caller is responsible for ensuring source-level validation has
    /// occurred when required. This method does not independently certify
    /// that the source program was parsed or semantically validated.
    fn start(
        &self,
        design: &Self::Design,
        config: &SimulationConfig,
    ) -> Result<Self::Session, Self::Error>;
}

// =============================================================================
// Run outcomes
// =============================================================================

/// Reason an active run stopped before normal completion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopReason {
    /// Cancellation was requested.
    Cancelled,

    /// The configured number of advancement calls was exhausted.
    StepBudgetExhausted,

    /// The configured event-processing budget was exceeded.
    EventBudgetExhausted,
}

/// Execution status returned by the generic driver.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunStatus {
    /// The engine reached its normal terminal condition.
    Completed,

    /// The session has no currently processable event.
    Quiescent,

    /// The run stopped without establishing normal completion.
    Stopped(StopReason),
}

/// Work counters collected by the driver.
///
/// These describe execution work reported by the session, not physical
/// resource measurements or proof of hardware feasibility.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RunCounters {
    /// Successful advancement calls.
    pub steps: u128,

    /// Events reported as processed by successful advancement calls.
    pub events_processed: u128,
}

/// Summary of a simulation run.
///
/// `Completed` and `Quiescent` describe execution only. Neither status implies
/// that assertions passed, a behavioral contract was proved, or the design is
/// feasible for physical realization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunReport<Time> {
    /// Final execution status.
    pub status: RunStatus,

    /// Last observed simulation time.
    pub final_time: Time,

    /// Work counters collected during the run.
    pub counters: RunCounters,
}

/// Failure of the generic session driver.
#[derive(Debug)]
pub enum RunError<E> {
    /// The underlying engine failed during advancement.
    Engine(E),

    /// A counter exceeded its representable range.
    ///
    /// The driver fails closed instead of wrapping counters and returning a
    /// misleading report.
    CounterOverflow {
        /// Counter that could not be incremented safely.
        counter: &'static str,
    },
}

impl<E: fmt::Display> fmt::Display for RunError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Engine(error) => {
                write!(f, "HDL simulation engine failed: {error}")
            }
            Self::CounterOverflow { counter } => {
                write!(f, "HDL simulation {counter} counter overflowed")
            }
        }
    }
}

impl<E: Error + 'static> Error for RunError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Engine(error) => Some(error),
            Self::CounterOverflow { .. } => None,
        }
    }
}

// =============================================================================
// Generic run driver
// =============================================================================

/// Drive a session until it completes, becomes quiescent, is cancelled, or
/// exhausts an operational budget.
///
/// The driver does not evaluate assertions, verify contracts, infer physical
/// feasibility, or define HDL event ordering.
///
/// The step budget is checked before starting an advancement call. The event
/// budget is checked after the engine reports the work it performed. Therefore
/// a single advancement may exceed the event budget; the report will indicate
/// that the run stopped.
///
/// Engine errors and counter overflow are returned as errors, never converted
/// into successful completion statuses.
pub fn run_session<S: SimulationSession>(
    session: &mut S,
    config: &SimulationConfig,
) -> Result<RunReport<S::Time>, RunError<S::Error>> {
    let mut counters = RunCounters::default();

    loop {
        if config.cancellation.is_cancelled() {
            return Ok(report(
                session,
                RunStatus::Stopped(StopReason::Cancelled),
                counters,
            ));
        }

        if config
            .budget
            .max_steps
            .is_some_and(|limit| counters.steps >= limit)
        {
            return Ok(report(
                session,
                RunStatus::Stopped(StopReason::StepBudgetExhausted),
                counters,
            ));
        }

        let outcome = session
            .step(&config.cancellation)
            .map_err(RunError::Engine)?;

        counters.steps = counters
            .steps
            .checked_add(1)
            .ok_or(RunError::CounterOverflow { counter: "step" })?;

        match outcome {
            StepOutcome::Progress { events_processed } => {
                counters.events_processed = counters
                    .events_processed
                    .checked_add(events_processed)
                    .ok_or(RunError::CounterOverflow { counter: "event" })?;

                if config
                    .budget
                    .max_events
                    .is_some_and(|limit| counters.events_processed > limit)
                {
                    return Ok(report(
                        session,
                        RunStatus::Stopped(StopReason::EventBudgetExhausted),
                        counters,
                    ));
                }
            }

            StepOutcome::Quiescent => {
                return Ok(report(session, RunStatus::Quiescent, counters));
            }

            StepOutcome::Finished => {
                return Ok(report(session, RunStatus::Completed, counters));
            }
        }
    }
}

fn report<S: SimulationSession>(
    session: &S,
    status: RunStatus,
    counters: RunCounters,
) -> RunReport<S::Time> {
    RunReport {
        status,
        final_time: session.current_time().clone(),
        counters,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::Infallible;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestError;

    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("test engine error")
        }
    }

    impl Error for TestError {}

    struct TestSession {
        time: u128,
        remaining: u128,
        events_per_step: u128,
        fail: bool,
    }

    impl SimulationSession for TestSession {
        type Time = u128;
        type Observation = u128;
        type Error = TestError;

        fn current_time(&self) -> &Self::Time {
            &self.time
        }

        fn step(
            &mut self,
            cancellation: &CancellationToken,
        ) -> Result<StepOutcome, Self::Error> {
            if cancellation.is_cancelled() {
                return Ok(StepOutcome::Quiescent);
            }

            if self.fail {
                return Err(TestError);
            }

            if self.remaining == 0 {
                return Ok(StepOutcome::Finished);
            }

            self.remaining -= 1;
            self.time = self.time.checked_add(1).ok_or(TestError)?;

            Ok(StepOutcome::Progress {
                events_processed: self.events_per_step,
            })
        }

        fn observe(&self) -> Result<Self::Observation, Self::Error> {
            Ok(self.time)
        }
    }

    fn session(steps: u128, events_per_step: u128) -> TestSession {
        TestSession {
            time: 0,
            remaining: steps,
            events_per_step,
            fail: false,
        }
    }

    #[test]
    fn cancellation_is_shared_between_clones() {
        let token = CancellationToken::new();
        let clone = token.clone();

        assert!(!token.is_cancelled());
        clone.cancel();

        assert!(token.is_cancelled());
        assert!(clone.is_cancelled());
    }

    #[test]
    fn normal_completion_is_reported() {
        let mut simulation = session(3, 2);
        let report =
            run_session(&mut simulation, &SimulationConfig::unbounded()).expect("run");

        assert_eq!(report.status, RunStatus::Completed);
        assert_eq!(report.final_time, 3);
        assert_eq!(report.counters.steps, 4);
        assert_eq!(report.counters.events_processed, 6);
    }

    #[test]
    fn step_budget_stops_without_claiming_completion() {
        let mut simulation = session(10, 1);
        let config = SimulationConfig::with_budget(SimulationBudget {
            max_steps: Some(2),
            max_events: None,
        });

        let report = run_session(&mut simulation, &config).expect("run");

        assert_eq!(
            report.status,
            RunStatus::Stopped(StopReason::StepBudgetExhausted)
        );
        assert_eq!(report.counters.steps, 2);
        assert_eq!(report.final_time, 2);
    }

    #[test]
    fn event_budget_reports_excess_work_without_claiming_completion() {
        let mut simulation = session(10, 3);
        let config = SimulationConfig::with_budget(SimulationBudget {
            max_steps: None,
            max_events: Some(2),
        });

        let report = run_session(&mut simulation, &config).expect("run");

        assert_eq!(
            report.status,
            RunStatus::Stopped(StopReason::EventBudgetExhausted)
        );
        assert_eq!(report.counters.events_processed, 3);
    }

    #[test]
    fn pre_cancelled_run_does_not_advance() {
        let mut simulation = session(10, 1);
        let token = CancellationToken::new();
        token.cancel();

        let config = SimulationConfig {
            budget: SimulationBudget::default(),
            cancellation: token,
        };

        let report = run_session(&mut simulation, &config).expect("run");

        assert_eq!(
            report.status,
            RunStatus::Stopped(StopReason::Cancelled)
        );
        assert_eq!(report.counters.steps, 0);
        assert_eq!(report.final_time, 0);
    }

    #[test]
    fn engine_errors_are_not_reported_as_success() {
        let mut simulation = session(10, 1);
        simulation.fail = true;

        let result = run_session(&mut simulation, &SimulationConfig::default());

        assert!(matches!(result, Err(RunError::Engine(TestError))));
    }

    #[test]
    fn observation_is_engine_defined() {
        let simulation = session(0, 0);
        assert_eq!(simulation.observe(), Ok(0));
    }

    #[test]
    fn zero_step_budget_stops_before_advancement() {
        let mut simulation = session(1, 1);
        let config = SimulationConfig::with_budget(SimulationBudget {
            max_steps: Some(0),
            max_events: None,
        });

        let report = run_session(&mut simulation, &config).expect("run");

        assert_eq!(
            report.status,
            RunStatus::Stopped(StopReason::StepBudgetExhausted)
        );
        assert_eq!(report.counters.steps, 0);
    }

    #[test]
    fn zero_event_budget_allows_zero_event_progress() {
        let mut simulation = session(1, 0);
        let config = SimulationConfig::with_budget(SimulationBudget {
            max_steps: None,
            max_events: Some(0),
        });

        let report = run_session(&mut simulation, &config).expect("run");

        assert_eq!(report.status, RunStatus::Completed);
        assert_eq!(report.counters.events_processed, 0);
    }

    #[test]
    fn cancellation_token_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<CancellationToken>();
    }

    #[test]
    fn engine_contract_does_not_require_an_ir_type() {
        struct ExampleEngine;

        impl SimulationEngine for ExampleEngine {
            type Design = String;
            type Session = TestSession;
            type Error = TestError;

            fn start(
                &self,
                _design: &Self::Design,
                _config: &SimulationConfig,
            ) -> Result<Self::Session, Self::Error> {
                Ok(session(0, 0))
            }
        }

        let engine = ExampleEngine;
        let design = String::from("engine-owned design");

        assert!(engine.start(&design, &SimulationConfig::default()).is_ok());
    }

    #[test]
    fn infallible_is_available_for_adapters_without_startup_failure() {
        fn accepts_error<E: Error>() {}
        accepts_error::<Infallible>();
    }
}
