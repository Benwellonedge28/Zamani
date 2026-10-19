
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
//! Define the engine-independent contract for starting, advancing, observing,
//! and controlling HDL simulations.
//!
//! This module is an interface and orchestration boundary. It is not an event
//! kernel, a hardware model, a parser, a semantic analyzer, or an IR.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/simulation.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//!
//! The grammar defines source syntax and simulation intent. The canonical AST
//! owns source structure. The semantic layer owns source-level correctness.
//! A simulation engine owns execution semantics and its internal event model.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The engine/session integration traits.
//! - Simulation execution configuration.
//! - Cooperative cancellation.
//! - Explicit execution budgets.
//! - Structured run outcomes and counters.
//! - A generic driver for advancing a simulation session.
//!
//! This module does not own:
//!
//! - The canonical AST or semantic model.
//! - HDLIR or any other intermediate representation.
//! - Event-queue implementation or event ordering rules.
//! - Logic evaluation, signal storage, or memory models.
//! - Clock generation, numerical solvers, or waveform storage.
//! - Formal verification or assertion evaluation.
//! - Target discovery, synthesis, placement, or routing.
//! - Physical resource allocation or vendor-specific APIs.
//!
//! ## Scalability and correctness
//!
//! No universal design-size, signal-count, clock-count, or event-count ceiling
//! is imposed. Optional execution budgets are operational policies, not HDL
//! language restrictions.
//!
//! No finite implementation can execute literally infinite work. This API
//! allows simulations to scale subject to available resources, representable
//! values, engine capabilities, and explicitly configured budgets.
//!
//! A budget-exhausted or cancelled run is never reported as completed.
//! A quiescent simulation is not automatically proof of correctness.
//! Simulation results must not be presented as formal verification results.
//!
//! ## Integration contract
//!
//! A concrete engine implements [`SimulationEngine`] and its session implements
//! [`SimulationSession`]. The engine accepts a design representation owned by
//! its integration adapter. This interface deliberately does not assume the
//! existence or shape of an AST, semantic-model, or HDLIR constructor.
//!
//! The AST/semantic adapter validates and resolves the source design before
//! passing an appropriate engine-owned representation to `start`.
//! The session implements the simulation language's actual event semantics.
//! The driver in this file controls lifecycle, cancellation, and budgets.
//!
//! The parent `simulation/mod.rs` should declare this module only after this
//! file and its sibling modules have been reviewed. No existing repository
//! module is assumed to expose an unverified API.
//!
//! ## Determinism
//!
//! This interface does not impose a universal event ordering. The engine must
//! document and implement the ordering required by the selected HDL semantics.
//! Given equivalent designs, configuration, inputs, and engine versions, an
//! engine should provide reproducible results wherever its contract promises
//! determinism.
//!
//! ## Freeze criteria
//!
//! - The public API compiles on the repository's supported Rust baseline.
//! - Engine failures remain distinct from normal termination.
//! - Cancellation and budget exhaustion remain distinguishable.
//! - Counter overflow cannot silently wrap.
//! - No incomplete run is classified as completed.
//! - Tests cover lifecycle, cancellation, budgets, and failure propagation.
//! - The interface remains independent of HDLIR and target realization.

use std::error::Error;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// =============================================================================
// Cancellation
// =============================================================================

/// A clonable, thread-safe cooperative cancellation token.
///
/// Any clone can request cancellation. A session should check the token during
/// long-running operations, particularly when one call to [`SimulationSession::step`]
/// may perform substantial work.
///
/// Cancellation is cooperative: this token cannot forcibly interrupt a
/// blocking or non-cooperative engine implementation.
#[derive(Clone, Debug, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Create a fresh token that has not been cancelled.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Request cancellation.
    ///
    /// Repeated calls are safe. Once requested, cancellation remains requested
    /// for every clone of this token.
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

/// Optional operational budgets for one simulation run.
///
/// `None` means that this interface imposes no limit for that budget. It does
/// not promise unlimited memory, unlimited execution time, or unlimited engine
/// capacity.
///
/// A service embedding this API may apply its own explicit operational policy.
/// Such policy must not change the source program's meaning silently.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SimulationBudget {
    /// Maximum number of successful session advancement calls.
    ///
    /// This counts calls returning a normal advancement outcome, including a
    /// call that reports quiescence or termination. Engine errors do not count
    /// as successful advancement calls.
    pub max_steps: Option<u128>,

    /// Maximum number of events the engine reports as processed.
    ///
    /// The session reports events processed by each advancement call. If an
    /// advancement crosses this budget, those events have already occurred;
    /// the driver reports budget exhaustion and never claims normal completion.
    pub max_events: Option<u128>,
}

/// Configuration for one simulation invocation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SimulationConfig {
    /// Explicit operational budgets for the run.
    pub budget: SimulationBudget,

    /// Cooperative cancellation token shared with the caller and session.
    pub cancellation: CancellationToken,
}

impl SimulationConfig {
    /// Create a configuration with no explicit step or event budget.
    #[must_use]
    pub fn unbounded() -> Self {
        Self::default()
    }

    /// Create a configuration with the supplied operational budget.
    #[must_use]
    pub fn with_budget(budget: SimulationBudget) -> Self {
        Self {
            budget,
            cancellation: CancellationToken::new(),
        }
    }

    /// Return a clone of this run's cancellation token.
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
/// A session's `step` method must implement the event semantics documented by
/// its engine. In particular, it must not report `Quiescent` merely because
/// its implementation chose not to process pending work.
///
/// `events_processed` counts events actually processed during this advancement.
/// A zero-event advancement is valid when the engine's documented semantics
/// require progress that does not process an event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StepOutcome {
    /// The engine performed an advancement operation.
    Progress {
        /// Number of events actually processed by this operation.
        events_processed: u128,
    },

    /// No further event can currently advance this session.
    ///
    /// This is not a correctness proof. The engine must define how pending
    /// inputs, external stimuli, and future events affect quiescence.
    Quiescent,

    /// The engine reached its own normal termination condition.
    Finished,
}

/// A running HDL simulation session.
///
/// Implementations must preserve their engine's documented simulation
/// semantics. `Time` is engine-defined so this interface does not force all
/// time representations into a host integer, floating-point number, or fixed
/// universal timescale.
///
/// `Observation` is likewise engine-defined. An engine may expose snapshots,
/// selected signal values, traces, or another documented observation type.
pub trait SimulationSession {
    /// The engine's simulation-time representation.
    type Time: Clone + fmt::Debug;

    /// The representation returned by an observation request.
    type Observation;

    /// The error type produced by the engine.
    type Error: Error + Send + Sync + 'static;

    /// Return the current logical simulation time.
    fn current_time(&self) -> &Self::Time;

    /// Advance according to the engine's documented event semantics.
    ///
    /// The implementation should periodically inspect `cancellation` if the
    /// operation can run for a substantial period. It must return an error
    /// for engine failures instead of disguising them as quiescence.
    fn step(
        &mut self,
        cancellation: &CancellationToken,
    ) -> Result<StepOutcome, Self::Error>;

    /// Observe the current simulation state.
    ///
    /// The meaning, consistency guarantees, and cost of an observation are
    /// engine-defined. Implementations should avoid copying the entire design
    /// or all signal histories when a selective observation is sufficient.
    fn observe(&self) -> Result<Self::Observation, Self::Error>;
}

/// An engine capable of starting a simulation for its own design type.
///
/// `Design` is deliberately associated with the engine. This allows adapters
/// to integrate existing Zamani frontend or semantic representations without
/// this module guessing their constructors or imposing a second AST or IR.
pub trait SimulationEngine {
    /// The design representation accepted by this engine.
    type Design;

    /// The session created by this engine.
    type Session: SimulationSession<Error = Self::Error>;

    /// The engine's startup and execution error type.
    type Error: Error + Send + Sync + 'static;

    /// Start a session for a previously validated or explicitly prepared
    /// engine-compatible design.
    ///
    /// This method must not claim that source-level semantic validation has
    /// occurred unless its caller or the engine can establish that fact.
    fn start(
        &self,
        design: &Self::Design,
        config: &SimulationConfig,
    ) -> Result<Self::Session, Self::Error>;
}

// =============================================================================
// Run outcomes
// =============================================================================

/// Reason an otherwise active run stopped before normal completion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopReason {
    /// Cancellation was requested.
    Cancelled,

    /// The configured number of advancement calls was exhausted.
    StepBudgetExhausted,

    /// The configured event-processing budget was exhausted.
    EventBudgetExhausted,
}

/// Normal termination status of a simulation run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunStatus {
    /// The engine reached its normal terminal condition.
    Completed,

    /// The session has no currently processable event.
    Quiescent,

    /// The run stopped without establishing normal completion.
    Stopped(StopReason),
}

/// Counters collected by the driver.
///
/// Counters describe work reported by the engine and the driver. They are not
/// physical hardware resource measurements.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RunCounters {
    /// Successful advancement calls.
    pub steps: u128,

    /// Events reported as processed by successful advancement calls.
    pub events_processed: u128,
}

/// Summary of a simulation run.
///
/// `Completed` and `Quiescent` describe execution status only. Neither status
/// implies that assertions passed, a specification was proved, or the design
/// is feasible for physical realization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunReport<Time> {
    /// Final execution status.
    pub status: RunStatus,

    /// Last observed simulation time.
    pub final_time: Time,

    /// Work counters collected during this run.
    pub counters: RunCounters,
}

/// Failures of the generic run driver.
#[derive(Debug)]
pub enum RunError<E> {
    /// The underlying engine failed to advance or observe.
    Engine(E),

    /// A run counter exceeded its representable range.
    ///
    /// The driver fails closed rather than wrapping and returning misleading
    /// counters or falsely reporting completion.
    CounterOverflow {
        /// Counter that could not be incremented safely.
        counter: &'static str,
    },
}

impl<E: fmt::Display> fmt::Display for RunError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Engine(error) => write!(f, "HDL simulation engine failed: {error}"),
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
/// exhausts a configured operational budget.
///
/// This function deliberately does not evaluate assertions, verify behavioral
/// contracts, or infer physical feasibility. Those are separate concerns.
///
/// A successful report means the driver completed according to the returned
/// status. It does not mean the design is correct.
///
/// If an engine returns an error, the error is propagated. If a counter would
/// overflow, the driver returns `CounterOverflow` rather than wrapping.
///
/// The step budget is checked before the next advancement call. The event
/// budget is checked after an advancement reports its actual event count;
/// consequently, the last advancement may exceed the configured event budget.
/// Such a run is reported as stopped, not completed.
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
        let config = SimulationConfig::unbounded();

        let report = run_session(&mut simulation, &config).expect("run");

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
    }

    #[test]
    fn event_budget_stops_when_reported_work_exceeds_limit() {
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
    fn cancellation_token_is_usable_across_thread_boundaries() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<CancellationToken>();
    }

    #[test]
    fn generic_engine_contract_does_not_require_an_ir_type() {
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

        let result = engine.start(&design, &SimulationConfig::default());

        assert!(result.is_ok());
    }

    #[test]
    fn infallible_is_available_for_adapters_without_startup_failure() {
        fn accepts_error<E: Error>() {}

        accepts_error::<Infallible>();
    }
}
