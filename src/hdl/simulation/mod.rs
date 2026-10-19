
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani HDL Simulation
//!
//! Path: `src/hdl/simulation/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module defines the public contract for HDL simulation requests,
//! execution control, cancellation, operational budgets, outcomes, and
//! backend integration.
//!
//! It is an interface and lifecycle boundary, not a simulation kernel.
//! Concrete event queues, signal evaluation, process scheduling, waveform
//! storage, and HDL behavioral execution belong in their respective
//! implementation modules.
//!
//! ## Normative authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/simulation.g4`
//! - `grammar/hdl/verification.g4`
//! - `grammar/hdl/assertions.g4`
//! - `grammar/execution/simulation.g4`
//! - `grammar/statements/simulate.g4`
//! - `grammar/expressions/simulation.g4`
//! - `grammar/policies/simulation.g4`
//! - `grammar/effects/simulation.g4`
//!
//! The grammar and specifications define source-level meaning. The existing
//! frontend parser and semantic validator remain authoritative for parsing,
//! AST construction, and semantic validation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Simulation request and subject-reference contracts.
//! - Stop conditions and execution configuration.
//! - Cancellation and cooperative execution checkpoints.
//! - Event and delta-cycle budget accounting.
//! - Structured simulation outcomes and infrastructure errors.
//! - The backend-facing simulation engine trait.
//!
//! This module does not own:
//!
//! - A second parser, AST, semantic model, or source-span representation.
//! - HDL intermediate representation or its builders and passes.
//! - Signal values, process semantics, or event-queue implementation.
//! - Assertion semantics, formal verification, or coverage algorithms.
//! - Physical hardware allocation, target discovery, placement, or routing.
//! - Vendor-specific simulator APIs or device selection.
//! - Waveform serialization or file-system access.
//! - Synthesis scheduling or hardware deployment.
//!
//! All HDLIR implementation remains exclusively under `src/hdl/hdlir/`
//! and is outside the scope of this module.
//!
//! ## Scalability
//!
//! The API imposes no universal limit on design size, signal count, hardware
//! width, hierarchy depth, or physical device capacity.
//!
//! Optional event and delta-cycle budgets are operational safeguards.
//! They are not language restrictions. Wall-clock limits are optional.
//! Cancellation is cooperative, so engines must check execution checkpoints
//! frequently enough to honor the caller's responsiveness requirements.
//!
//! A finite machine cannot process literally infinite work. A simulation
//! that exhausts a budget or is cancelled must report that outcome rather
//! than claiming that the requested simulation completed successfully.
//!
//! ## Integration
//!
//! The existing frontend should eventually adapt validated HDL simulation
//! intent into `SimulationRequest` without transferring parser or semantic
//! ownership into this module.
//!
//! A future event scheduler implements `SimulationEngine` and uses
//! `SimulationContext` for cooperative cancellation and budget accounting.
//! A future public `src/hdl/mod.rs` may expose this module after its
//! integration contracts and tests are accepted.
//!
//! No dependency on `src/hdl/hdlir/` is required.

use std::error::Error;
use std::fmt;
use std::num::NonZeroUsize;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

// =============================================================================
// Simulation subject
// =============================================================================

/// An opaque reference to a simulation subject.
///
/// The reference identifies an already-resolved subject supplied by an
/// integration adapter. It is not a parser-level expression and does not
/// imply that a target, device, or simulator has been selected.
///
/// The owning integration layer is responsible for ensuring that the
/// reference is meaningful to the selected simulation engine.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SimulationSubject {
    reference: Arc<str>,
}

impl SimulationSubject {
    /// Construct a subject reference.
    ///
    /// An empty reference is rejected because it cannot identify a subject.
    /// The contents otherwise remain opaque to this module.
    pub fn new(reference: impl Into<Arc<str>>) -> Result<Self, SimulationError> {
        let reference = reference.into();

        if reference.trim().is_empty() {
            return Err(SimulationError::InvalidRequest(
                "simulation subject reference must not be empty".to_owned(),
            ));
        }

        Ok(Self { reference })
    }

    /// Return the opaque subject reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.reference
    }
}

// =============================================================================
// Stop conditions
// =============================================================================

/// Declares the requested logical stopping condition.
///
/// The logical time representation belongs to the selected simulation
/// engine. This avoids imposing a universal integer width or time unit.
///
/// A stop condition is a request, not a guarantee that execution can reach
/// it. Cancellation, operational budgets, and engine failures can terminate
/// a run earlier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationStopCondition<T> {
    /// Stop when the engine reaches the requested logical time.
    UntilTime(T),

    /// Stop when the engine establishes that no further events are pending
    /// under its documented quiescence semantics.
    UntilQuiescent,

    /// Continue until cancellation, a configured operational limit, or
    /// another explicit engine termination condition.
    UntilCancelled,
}

// =============================================================================
// Operational configuration
// =============================================================================

/// Operational safeguards for a simulation invocation.
///
/// `None` means that this API does not impose that particular budget.
/// It does not mean that the host has infinite resources.
///
/// Defaults are intentionally unbounded by this API. Applications that
/// process untrusted designs should configure suitable operational budgets.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SimulationConfig {
    /// Maximum number of events the engine may process.
    ///
    /// When set, the run must not process more than this number of events.
    pub event_budget: Option<NonZeroUsize>,

    /// Maximum number of delta cycles the engine may process.
    ///
    /// Delta cycles are zero-physical-time evaluation steps, as defined by
    /// the selected engine's execution model.
    pub delta_cycle_budget: Option<NonZeroUsize>,

    /// Optional wall-clock duration for the invocation.
    ///
    /// This is an operational timeout, not simulated time.
    pub wall_clock_timeout: Option<Duration>,

    /// Whether a backend that supports assertion callbacks should stop
    /// when an assertion fails.
    ///
    /// Assertion evaluation and failure classification remain the
    /// responsibility of the verification integration.
    pub stop_on_assertion_failure: bool,
}

impl SimulationConfig {
    /// Create an unbounded configuration.
    ///
    /// This disables the optional limits owned by this API. It does not
    /// disable process-level limits or make an engine immune to exhaustion.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            event_budget: None,
            delta_cycle_budget: None,
            wall_clock_timeout: None,
            stop_on_assertion_failure: false,
        }
    }

    /// Configure the maximum event count.
    #[must_use]
    pub const fn with_event_budget(mut self, budget: NonZeroUsize) -> Self {
        self.event_budget = Some(budget);
        self
    }

    /// Configure the maximum delta-cycle count.
    #[must_use]
    pub const fn with_delta_cycle_budget(mut self, budget: NonZeroUsize) -> Self {
        self.delta_cycle_budget = Some(budget);
        self
    }

    /// Configure an optional wall-clock timeout.
    #[must_use]
    pub const fn with_wall_clock_timeout(mut self, timeout: Duration) -> Self {
        self.wall_clock_timeout = Some(timeout);
        self
    }
}

// =============================================================================
// Request
// =============================================================================

/// A request to simulate an already-resolved HDL subject.
///
/// This request contains no HDLIR, parser nodes, physical device handles,
/// or vendor-specific state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationRequest<T> {
    /// Subject to simulate.
    pub subject: SimulationSubject,

    /// Requested stopping condition.
    pub stop_condition: SimulationStopCondition<T>,

    /// Invocation-specific operational policy.
    pub config: SimulationConfig,
}

impl<T> SimulationRequest<T> {
    /// Construct a simulation request.
    #[must_use]
    pub fn new(
        subject: SimulationSubject,
        stop_condition: SimulationStopCondition<T>,
        config: SimulationConfig,
    ) -> Self {
        Self {
            subject,
            stop_condition,
            config,
        }
    }
}

// =============================================================================
// Cancellation
// =============================================================================

/// A clonable cooperative cancellation handle.
///
/// Clones refer to the same cancellation state. Cancelling one handle
/// requests cancellation for every engine using a clone.
///
/// Cancellation is a request, not a mechanism for forcibly interrupting
/// arbitrary backend code. Engines must check their execution checkpoints.
#[derive(Debug, Clone, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Create a fresh, uncancelled token.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Request cancellation.
    ///
    /// This operation is idempotent.
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
// Stop reasons and execution accounting
// =============================================================================

/// Why a simulation execution stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SimulationStopReason {
    /// The requested logical stopping time was reached.
    ReachedTime,

    /// The engine established quiescence.
    Quiescent,

    /// The caller requested cancellation.
    Cancelled,

    /// The configured event budget was exhausted.
    EventBudgetExceeded,

    /// The configured delta-cycle budget was exhausted.
    DeltaCycleBudgetExceeded,

    /// The wall-clock timeout elapsed.
    WallClockTimeout,

    /// An assertion failure requested termination.
    AssertionFailure,

    /// The engine terminated according to an engine-specific condition.
    EngineRequestedTermination,
}

impl SimulationStopReason {
    /// Whether the requested simulation is known to have completed normally.
    ///
    /// Budget exhaustion, cancellation, and timeout do not establish that
    /// the requested simulation has completed.
    #[must_use]
    pub const fn is_normal_completion(self) -> bool {
        matches!(self, Self::ReachedTime | Self::Quiescent)
    }
}

/// Event and delta-cycle counters for a simulation invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SimulationCounters {
    events_processed: usize,
    delta_cycles_processed: usize,
}

impl SimulationCounters {
    /// Number of events processed.
    #[must_use]
    pub const fn events_processed(&self) -> usize {
        self.events_processed
    }

    /// Number of delta cycles processed.
    #[must_use]
    pub const fn delta_cycles_processed(&self) -> usize {
        self.delta_cycles_processed
    }
}

// =============================================================================
// Execution context
// =============================================================================

/// Cooperative execution context shared with a simulation engine.
///
/// The context owns invocation-local accounting and timing. It does not own
/// the engine's event queue or mutable hardware state.
#[derive(Debug)]
pub struct SimulationContext {
    config: SimulationConfig,
    cancellation: CancellationToken,
    started_at: Instant,
    counters: SimulationCounters,
}

impl SimulationContext {
    /// Create an execution context for one invocation.
    #[must_use]
    pub fn new(config: SimulationConfig, cancellation: CancellationToken) -> Self {
        Self {
            config,
            cancellation,
            started_at: Instant::now(),
            counters: SimulationCounters::default(),
        }
    }

    /// Return the invocation's cancellation token.
    #[must_use]
    pub fn cancellation_token(&self) -> &CancellationToken {
        &self.cancellation
    }

    /// Return a snapshot of execution counters.
    #[must_use]
    pub const fn counters(&self) -> SimulationCounters {
        self.counters
    }

    /// Return elapsed wall-clock time.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    /// Check cancellation and time limits before doing more work.
    ///
    /// This method does not mutate simulation state.
    pub fn checkpoint(&self) -> Result<(), SimulationStopReason> {
        if self.cancellation.is_cancelled() {
            return Err(SimulationStopReason::Cancelled);
        }

        if self
            .config
            .wall_clock_timeout
            .is_some_and(|timeout| self.started_at.elapsed() >= timeout)
        {
            return Err(SimulationStopReason::WallClockTimeout);
        }

        if self
            .config
            .event_budget
            .is_some_and(|budget| self.counters.events_processed >= budget.get())
        {
            return Err(SimulationStopReason::EventBudgetExceeded);
        }

        if self
            .config
            .delta_cycle_budget
            .is_some_and(|budget| self.counters.delta_cycles_processed >= budget.get())
        {
            return Err(SimulationStopReason::DeltaCycleBudgetExceeded);
        }

        Ok(())
    }

    /// Record one successfully processed event.
    ///
    /// The caller must invoke this once for each event processed according
    /// to the engine's documented accounting rules.
    ///
    /// The counter is updated with checked arithmetic. Counter overflow
    /// returns an infrastructure error instead of wrapping or saturating.
    pub fn record_event(&mut self) -> Result<(), SimulationError> {
        self.checkpoint()
            .map_err(SimulationError::ExecutionStopped)?;

        self.counters.events_processed = self
            .counters
            .events_processed
            .checked_add(1)
            .ok_or(SimulationError::CounterOverflow("event"))?;

        Ok(())
    }

    /// Record one successfully processed delta cycle.
    pub fn record_delta_cycle(&mut self) -> Result<(), SimulationError> {
        self.checkpoint()
            .map_err(SimulationError::ExecutionStopped)?;

        self.counters.delta_cycles_processed = self
            .counters
            .delta_cycles_processed
            .checked_add(1)
            .ok_or(SimulationError::CounterOverflow("delta-cycle"))?;

        Ok(())
    }

    /// Check whether processing another event or delta cycle is permitted.
    ///
    /// A backend should call this at scheduling boundaries and during
    /// potentially long-running evaluation loops.
    pub fn check_work_allowed(&self) -> Result<(), SimulationStopReason> {
        self.checkpoint()
    }

    /// Return the configured operational policy.
    #[must_use]
    pub const fn config(&self) -> &SimulationConfig {
        &self.config
    }
}

// =============================================================================
// Outcomes
// =============================================================================

/// Outcome of a simulation invocation.
///
/// An outcome always records why execution stopped. The engine must not
/// report normal completion merely because it stopped processing work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationOutcome<T> {
    /// Final logical time, if the engine can determine it.
    pub final_time: Option<T>,

    /// Reason execution stopped.
    pub stop_reason: SimulationStopReason,

    /// Work completed before termination.
    pub counters: SimulationCounters,

    /// Whether the backend completed its requested work without a known
    /// semantic, assertion, or infrastructure failure.
    ///
    /// This flag is separate from the stop reason so integrations can
    /// represent engine-specific successful termination explicitly.
    pub completed: bool,
}

impl<T> SimulationOutcome<T> {
    /// Construct a normally completed outcome.
    ///
    /// This constructor rejects stop reasons that do not represent normal
    /// completion. It does not establish semantic correctness of the
    /// simulation; that remains the engine and verification layer's duty.
    pub fn completed(
        final_time: Option<T>,
        stop_reason: SimulationStopReason,
        counters: SimulationCounters,
    ) -> Result<Self, SimulationError> {
        if !stop_reason.is_normal_completion() {
            return Err(SimulationError::InvalidOutcome(
                "normal completion requires ReachedTime or Quiescent".to_owned(),
            ));
        }

        Ok(Self {
            final_time,
            stop_reason,
            counters,
            completed: true,
        })
    }

    /// Construct an explicitly incomplete outcome.
    #[must_use]
    pub fn incomplete(
        final_time: Option<T>,
        stop_reason: SimulationStopReason,
        counters: SimulationCounters,
    ) -> Self {
        Self {
            final_time,
            stop_reason,
            counters,
            completed: false,
        }
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors at the simulation API and engine-integration boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SimulationError {
    /// The request is malformed or missing required information.
    InvalidRequest(String),

    /// The engine cannot execute this request.
    UnsupportedOperation(String),

    /// An engine or integration contract was violated.
    ContractViolation(String),

    /// A work counter cannot represent another increment.
    CounterOverflow(&'static str),

    /// Execution stopped at an operational checkpoint.
    ///
    /// This is not equivalent to successful completion.
    ExecutionStopped(SimulationStopReason),

    /// The engine produced an inconsistent outcome.
    InvalidOutcome(String),

    /// The engine failed while executing the simulation.
    EngineFailure(String),
}

impl fmt::Display for SimulationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest(message) => {
                write!(f, "invalid simulation request: {message}")
            }
            Self::UnsupportedOperation(message) => {
                write!(f, "unsupported simulation operation: {message}")
            }
            Self::ContractViolation(message) => {
                write!(f, "simulation contract violation: {message}")
            }
            Self::CounterOverflow(counter) => {
                write!(f, "simulation {counter} counter overflow")
            }
            Self::ExecutionStopped(reason) => {
                write!(f, "simulation execution stopped: {reason:?}")
            }
            Self::InvalidOutcome(message) => {
                write!(f, "invalid simulation outcome: {message}")
            }
            Self::EngineFailure(message) => {
                write!(f, "simulation engine failure: {message}")
            }
        }
    }
}

impl Error for SimulationError {}

// =============================================================================
// Engine contract
// =============================================================================

/// Contract implemented by a concrete HDL simulation engine.
///
/// `Time` is engine-defined. Its ordering must agree with that engine's
/// logical-time semantics. This interface deliberately does not mandate
/// integer widths, time units, event-queue structures, or vendor APIs.
///
/// Implementations must:
///
/// - Honor the requested stop condition when it is reachable.
/// - Check cancellation and operational budgets cooperatively.
/// - Report incomplete runs honestly.
/// - Preserve the design's validated HDL semantics.
/// - Never silently convert an unsupported or infeasible simulation into
///   a successful result.
/// - Return deterministic results when the engine's declared execution
///   policy promises determinism.
///
/// This trait does not provide an event scheduler. A scheduler or execution
/// engine must implement the actual behavioral semantics.
pub trait SimulationEngine {
    /// The engine's logical-time representation.
    type Time: Clone + PartialEq + Eq;

    /// Execute one simulation request.
    fn run(
        &mut self,
        request: &SimulationRequest<Self::Time>,
        context: &mut SimulationContext,
    ) -> Result<SimulationOutcome<Self::Time>, SimulationError>;
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn subject() -> SimulationSubject {
        SimulationSubject::new("design:test_top").expect("valid subject")
    }

    #[test]
    fn rejects_empty_subject_reference() {
        assert!(SimulationSubject::new("").is_err());
        assert!(SimulationSubject::new("  ").is_err());
    }

    #[test]
    fn preserves_subject_reference() {
        let subject = subject();
        assert_eq!(subject.as_str(), "design:test_top");
    }

    #[test]
    fn defaults_do_not_impose_hidden_budgets() {
        let config = SimulationConfig::default();

        assert!(config.event_budget.is_none());
        assert!(config.delta_cycle_budget.is_none());
        assert!(config.wall_clock_timeout.is_none());
        assert!(!config.stop_on_assertion_failure);
    }

    #[test]
    fn cancellation_is_shared_between_clones() {
        let first = CancellationToken::new();
        let second = first.clone();

        assert!(!first.is_cancelled());

        second.cancel();

        assert!(first.is_cancelled());
        assert!(second.is_cancelled());
    }

    #[test]
    fn event_budget_stops_additional_work() {
        let budget = NonZeroUsize::new(2).expect("nonzero");
        let config = SimulationConfig::default().with_event_budget(budget);
        let mut context = SimulationContext::new(config, CancellationToken::new());

        context.record_event().expect("first event");
        context.record_event().expect("second event");

        assert_eq!(
            context.check_work_allowed(),
            Err(SimulationStopReason::EventBudgetExceeded)
        );

        assert_eq!(context.counters().events_processed(), 2);
    }

    #[test]
    fn delta_cycle_budget_stops_additional_work() {
        let budget = NonZeroUsize::new(1).expect("nonzero");
        let config = SimulationConfig::default().with_delta_cycle_budget(budget);
        let mut context = SimulationContext::new(config, CancellationToken::new());

        context
            .record_delta_cycle()
            .expect("first delta cycle");

        assert_eq!(
            context.check_work_allowed(),
            Err(SimulationStopReason::DeltaCycleBudgetExceeded)
        );

        assert_eq!(context.counters().delta_cycles_processed(), 1);
    }

    #[test]
    fn cancellation_is_observed_at_checkpoint() {
        let token = CancellationToken::new();
        let context = SimulationContext::new(
            SimulationConfig::default(),
            token.clone(),
        );

        token.cancel();

        assert_eq!(
            context.check_work_allowed(),
            Err(SimulationStopReason::Cancelled)
        );
    }

    #[test]
    fn zero_duration_timeout_stops_work() {
        let config = SimulationConfig::default()
            .with_wall_clock_timeout(Duration::ZERO);

        let context =
            SimulationContext::new(config, CancellationToken::new());

        assert_eq!(
            context.check_work_allowed(),
            Err(SimulationStopReason::WallClockTimeout)
        );
    }

    #[test]
    fn successful_outcome_requires_normal_stop_reason() {
        let counters = SimulationCounters::default();

        assert!(SimulationOutcome::completed(
            Some(10_u64),
            SimulationStopReason::ReachedTime,
            counters,
        )
        .is_ok());

        assert!(SimulationOutcome::completed(
            None::<u64>,
            SimulationStopReason::Cancelled,
            counters,
        )
        .is_err());
    }

    #[test]
    fn incomplete_outcome_never_claims_completion() {
        let outcome = SimulationOutcome::incomplete(
            None::<u64>,
            SimulationStopReason::EventBudgetExceeded,
            SimulationCounters::default(),
        );

        assert!(!outcome.completed);
        assert_eq!(
            outcome.stop_reason,
            SimulationStopReason::EventBudgetExceeded
        );
    }

    #[test]
    fn request_preserves_stop_condition_and_config() {
        let budget = NonZeroUsize::new(100).expect("nonzero");
        let config = SimulationConfig::default().with_event_budget(budget);

        let request = SimulationRequest::new(
            subject(),
            SimulationStopCondition::UntilTime(42_u64),
            config.clone(),
        );

        assert_eq!(request.subject.as_str(), "design:test_top");
        assert_eq!(
            request.stop_condition,
            SimulationStopCondition::UntilTime(42)
        );
        assert_eq!(request.config, config);
    }

    #[test]
    fn counters_start_at_zero() {
        let context = SimulationContext::new(
            SimulationConfig::default(),
            CancellationToken::new(),
        );

        assert_eq!(context.counters().events_processed(), 0);
        assert_eq!(context.counters().delta_cycles_processed(), 0);
    }
}
