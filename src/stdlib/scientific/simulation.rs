
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Standard Library — Scientific Simulation
//!
//! Path:
//!     src/stdlib/scientific/simulation.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version 1.97
//!
//! Safety:
//!     Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provide a portable, domain-neutral scientific simulation API.
//!
//! This module defines:
//!
//! - Simulation model identity.
//! - Open-ended simulation domain identity.
//! - Simulation requests and execution policies.
//! - Cooperative cancellation.
//! - Operational budget accounting.
//! - Structured simulation outcomes.
//! - An engine integration contract.
//!
//! # Ownership
//!
//! This module does not own:
//!
//! - Source grammar, AST, semantic analysis, or IR.
//! - Classical numerical algorithms or tensor storage.
//! - Quantum simulation, trajectories, sampling, or QEC.
//! - HDL signal semantics or event scheduling.
//! - Hybrid execution or distributed coordination.
//! - AI model execution.
//! - Compiler optimization or lowering.
//! - Hardware discovery, allocation, or selection.
//! - Runtime scheduling or backend execution.
//!
//! Domain-specific engines remain authoritative in their
//! respective repository subsystems.
//!
//! # Scalability
//!
//! This API introduces no arbitrary universal limits on:
//!
//! - Simulation steps.
//! - Events.
//! - Models.
//! - Domains.
//! - Devices.
//! - Processors.
//! - Accelerators.
//! - Nodes.
//! - Memory.
//! - Tensor dimensions or rank.
//!
//! Optional budgets are invocation-specific operational policies.
//! They are not language-level capability limits.
//!
//! No finite machine can perform literally infinite computation.
//! Execution is bounded by actual resources and explicit policies.

use std::error::Error;
use std::fmt;
use std::num::NonZeroU64;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

// =============================================================================
// Domain identity
// =============================================================================

/// Open-world identity for a simulation domain.
///
/// This is an identifier, not a closed enumeration of computing
/// domains. New domains do not require changes to this type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimulationDomain(Arc<str>);

impl SimulationDomain {
    /// Construct a domain identifier.
    ///
    /// Empty or whitespace-only identifiers are rejected.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, SimulationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(SimulationError::InvalidIdentifier {
                field: "domain",
            });
        }

        Ok(Self(value))
    }

    /// Return the domain identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SimulationDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Model identity
// =============================================================================

/// Stable descriptive identity for a simulation model.
///
/// Identity does not imply that a model has been validated,
/// compiled, loaded, or selected for execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SimulationIdentity {
    id: Arc<str>,
    version: Arc<str>,
    domain: SimulationDomain,
}

impl SimulationIdentity {
    /// Construct a simulation model identity.
    pub fn new(
        id: impl Into<Arc<str>>,
        version: impl Into<Arc<str>>,
        domain: SimulationDomain,
    ) -> Result<Self, SimulationError> {
        let id = id.into();
        let version = version.into();

        if id.trim().is_empty() {
            return Err(SimulationError::InvalidIdentifier {
                field: "model id",
            });
        }

        if version.trim().is_empty() {
            return Err(SimulationError::InvalidIdentifier {
                field: "model version",
            });
        }

        Ok(Self {
            id,
            version,
            domain,
        })
    }

    /// Return the model identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the model version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Return the model's declared domain.
    #[must_use]
    pub fn domain(&self) -> &SimulationDomain {
        &self.domain
    }
}

/// Contract implemented by a scientific simulation model.
///
/// A model describes its identity. It does not choose a simulator,
/// hardware target, execution provider, or runtime scheduler.
pub trait SimulationModel {
    /// Return the model's stable identity.
    fn identity(&self) -> &SimulationIdentity;
}

// =============================================================================
// Simulation subject
// =============================================================================

/// Opaque reference to an already-resolved simulation subject.
///
/// The reference is intentionally independent of AST nodes,
/// semantic identifiers, IR values, and physical device identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SimulationSubject {
    reference: Arc<str>,
}

impl SimulationSubject {
    /// Construct a subject reference.
    pub fn new(
        reference: impl Into<Arc<str>>,
    ) -> Result<Self, SimulationError> {
        let reference = reference.into();

        if reference.trim().is_empty() {
            return Err(SimulationError::InvalidIdentifier {
                field: "simulation subject",
            });
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

/// Requested logical termination condition.
///
/// The meaning and representation of logical time belong to the
/// selected domain engine. This API does not impose a universal
/// time unit or integer representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationStopCondition<T> {
    /// Stop when the engine reaches the requested logical time.
    UntilTime(T),

    /// Stop when the engine establishes its documented quiescence
    /// condition.
    UntilQuiescent,

    /// Continue until cancellation, a configured budget, or another
    /// explicit engine termination condition.
    UntilCancelled,
}

// =============================================================================
// Operational policy
// =============================================================================

/// Optional operational budgets for one simulation invocation.
///
/// `None` means this API imposes no additional limit for that
/// particular resource. It does not mean the host has infinite
/// resources.
///
/// Budgets are configurable safeguards, not semantic ceilings.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SimulationConfig {
    /// Maximum number of completed simulation steps.
    pub step_budget: Option<NonZeroU64>,

    /// Maximum number of processed events.
    pub event_budget: Option<NonZeroU64>,

    /// Optional wall-clock execution timeout.
    ///
    /// This is not simulated time.
    pub wall_clock_timeout: Option<Duration>,
}

impl SimulationConfig {
    /// Create a configuration with no additional API-level budgets.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            step_budget: None,
            event_budget: None,
            wall_clock_timeout: None,
        }
    }

    /// Set the step budget.
    #[must_use]
    pub const fn with_step_budget(mut self, budget: NonZeroU64) -> Self {
        self.step_budget = Some(budget);
        self
    }

    /// Set the event budget.
    #[must_use]
    pub const fn with_event_budget(mut self, budget: NonZeroU64) -> Self {
        self.event_budget = Some(budget);
        self
    }

    /// Set the wall-clock timeout.
    #[must_use]
    pub const fn with_wall_clock_timeout(
        mut self,
        timeout: Duration,
    ) -> Self {
        self.wall_clock_timeout = Some(timeout);
        self
    }
}

// =============================================================================
// Request
// =============================================================================

/// A domain-neutral request to simulate a resolved subject.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationRequest<T> {
    subject: SimulationSubject,
    identity: SimulationIdentity,
    stop_condition: SimulationStopCondition<T>,
    config: SimulationConfig,
}

impl<T> SimulationRequest<T> {
    /// Construct a simulation request.
    pub fn new(
        subject: SimulationSubject,
        identity: SimulationIdentity,
        stop_condition: SimulationStopCondition<T>,
        config: SimulationConfig,
    ) -> Self {
        Self {
            subject,
            identity,
            stop_condition,
            config,
        }
    }

    /// Return the simulation subject.
    #[must_use]
    pub fn subject(&self) -> &SimulationSubject {
        &self.subject
    }

    /// Return the model identity.
    #[must_use]
    pub fn identity(&self) -> &SimulationIdentity {
        &self.identity
    }

    /// Return the requested stop condition.
    #[must_use]
    pub fn stop_condition(&self) -> &SimulationStopCondition<T> {
        &self.stop_condition
    }

    /// Return the operational configuration.
    #[must_use]
    pub fn config(&self) -> &SimulationConfig {
        &self.config
    }

    /// Verify that the supplied model agrees with this request.
    pub fn validate_model<M: SimulationModel>(
        &self,
        model: &M,
    ) -> Result<(), SimulationError> {
        if model.identity() != &self.identity {
            return Err(SimulationError::ModelIdentityMismatch);
        }

        Ok(())
    }
}

// =============================================================================
// Cancellation
// =============================================================================

/// Cooperative cancellation handle.
///
/// Clones share the same cancellation state.
#[derive(Debug, Clone, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Create a fresh cancellation token.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Request cancellation.
    ///
    /// Cancellation is idempotent.
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
// Execution counters
// =============================================================================

/// Checked accounting for simulation work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SimulationCounters {
    steps: u64,
    events: u64,
}

impl SimulationCounters {
    /// Number of completed steps.
    #[must_use]
    pub const fn steps(&self) -> u64 {
        self.steps
    }

    /// Number of processed events.
    #[must_use]
    pub const fn events(&self) -> u64 {
        self.events
    }
}

// =============================================================================
// Stop reasons
// =============================================================================

/// Reason a simulation engine stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SimulationStopReason {
    /// The requested logical time was reached.
    ReachedTime,

    /// The engine established quiescence.
    Quiescent,

    /// The caller requested cancellation.
    Cancelled,

    /// The configured step budget was exhausted.
    StepBudgetExceeded,

    /// The configured event budget was exhausted.
    EventBudgetExceeded,

    /// The wall-clock timeout elapsed.
    WallClockTimeout,

    /// An engine-specific normal termination condition occurred.
    EngineCompleted,

    /// The engine explicitly stopped without completing the request.
    EngineStoppedIncomplete,
}

impl SimulationStopReason {
    /// Whether this reason represents a declared normal completion.
    #[must_use]
    pub const fn is_normal_completion(self) -> bool {
        matches!(
            self,
            Self::ReachedTime
                | Self::Quiescent
                | Self::EngineCompleted
        )
    }
}

// =============================================================================
// Execution context
// =============================================================================

/// Invocation-local cooperative execution context.
///
/// This context performs cancellation and budget accounting only.
/// It does not schedule work, allocate hardware, or execute models.
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
    pub fn new(
        config: SimulationConfig,
        cancellation: CancellationToken,
    ) -> Self {
        Self {
            config,
            cancellation,
            started_at: Instant::now(),
            counters: SimulationCounters::default(),
        }
    }

    /// Return the cancellation handle.
    #[must_use]
    pub fn cancellation_token(&self) -> &CancellationToken {
        &self.cancellation
    }

    /// Return a snapshot of the counters.
    #[must_use]
    pub const fn counters(&self) -> SimulationCounters {
        self.counters
    }

    /// Return elapsed wall-clock time.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    /// Return the operational policy.
    #[must_use]
    pub const fn config(&self) -> &SimulationConfig {
        &self.config
    }

    /// Check whether further work is currently permitted.
    ///
    /// Engines should call this at execution checkpoints and
    /// during long-running calculations.
    pub fn checkpoint(&self) -> Result<(), SimulationStopReason> {
        if self.cancellation.is_cancelled() {
            return Err(SimulationStopReason::Cancelled);
        }

        if self.config.wall_clock_timeout.is_some_and(|timeout| {
            self.started_at.elapsed() >= timeout
        }) {
            return Err(SimulationStopReason::WallClockTimeout);
        }

        if self.config.step_budget.is_some_and(|budget| {
            self.counters.steps >= budget.get()
        }) {
            return Err(SimulationStopReason::StepBudgetExceeded);
        }

        if self.config.event_budget.is_some_and(|budget| {
            self.counters.events >= budget.get()
        }) {
            return Err(SimulationStopReason::EventBudgetExceeded);
        }

        Ok(())
    }

    /// Record one successfully completed simulation step.
    pub fn record_step(&mut self) -> Result<(), SimulationError> {
        self.checkpoint()
            .map_err(SimulationError::ExecutionStopped)?;

        self.counters.steps = self
            .counters
            .steps
            .checked_add(1)
            .ok_or(SimulationError::CounterOverflow("step"))?;

        Ok(())
    }

    /// Record one successfully processed event.
    pub fn record_event(&mut self) -> Result<(), SimulationError> {
        self.checkpoint()
            .map_err(SimulationError::ExecutionStopped)?;

        self.counters.events = self
            .counters
            .events
            .checked_add(1)
            .ok_or(SimulationError::CounterOverflow("event"))?;

        Ok(())
    }
}

// =============================================================================
// Outcome
// =============================================================================

/// Result of a simulation invocation.
///
/// `T` may be a compact summary, a lazy result handle, a stream,
/// or another domain-owned result. This API does not force a
/// potentially enormous simulation result into an in-memory Vec.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationOutcome<T> {
    result: T,
    stop_reason: SimulationStopReason,
    counters: SimulationCounters,
    completed: bool,
}

impl<T> SimulationOutcome<T> {
    /// Construct a normally completed outcome.
    pub fn completed(
        result: T,
        stop_reason: SimulationStopReason,
        counters: SimulationCounters,
    ) -> Result<Self, SimulationError> {
        if !stop_reason.is_normal_completion() {
            return Err(SimulationError::InvalidOutcome(
                "a completed outcome requires a normal stop reason",
            ));
        }

        Ok(Self {
            result,
            stop_reason,
            counters,
            completed: true,
        })
    }

    /// Construct an explicitly incomplete outcome.
    #[must_use]
    pub fn incomplete(
        result: T,
        stop_reason: SimulationStopReason,
        counters: SimulationCounters,
    ) -> Self {
        Self {
            result,
            stop_reason,
            counters,
            completed: false,
        }
    }

    /// Borrow the domain-owned result.
    #[must_use]
    pub const fn result(&self) -> &T {
        &self.result
    }

    /// Consume the outcome and return its result.
    #[must_use]
    pub fn into_result(self) -> T {
        self.result
    }

    /// Return the stop reason.
    #[must_use]
    pub const fn stop_reason(&self) -> SimulationStopReason {
        self.stop_reason
    }

    /// Return execution counters.
    #[must_use]
    pub const fn counters(&self) -> SimulationCounters {
        self.counters
    }

    /// Whether the requested simulation completed normally.
    #[must_use]
    pub const fn is_completed(&self) -> bool {
        self.completed
    }

    /// Validate the consistency of the outcome.
    pub fn validate(&self) -> Result<(), SimulationError> {
        if self.completed && !self.stop_reason.is_normal_completion() {
            return Err(SimulationError::InvalidOutcome(
                "completed outcome has a non-normal stop reason",
            ));
        }

        if !self.completed && self.stop_reason.is_normal_completion() {
            return Err(SimulationError::InvalidOutcome(
                "incomplete outcome has a normal stop reason",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Engine integration
// =============================================================================

/// Contract implemented by a simulation engine adapter.
///
/// An implementation may delegate to an existing domain authority:
///
/// - Classical scientific simulation implementation.
/// - Quantum simulation implementation.
/// - HDL simulation implementation.
/// - Hybrid execution integration.
/// - Distributed simulation implementation.
///
/// This trait does not prescribe how an engine executes, schedules,
/// parallelizes, distributes, or accelerates its workload.
pub trait SimulationEngine<M: SimulationModel> {
    /// Domain-owned result type.
    ///
    /// It may be a lazy handle or streaming result rather than
    /// a fully materialized collection.
    type Output;

    /// Execute one validated simulation request.
    fn run<T>(
        &mut self,
        model: &M,
        request: &SimulationRequest<T>,
        context: &mut SimulationContext,
    ) -> Result<SimulationOutcome<Self::Output>, SimulationError>;
}

/// Validate a request and invoke its selected engine adapter.
///
/// This function does not select an engine. Engine selection and
/// feasibility resolution belong to the compiler/runtime/provider
/// integration layers.
pub fn run_simulation<M, E, T>(
    engine: &mut E,
    model: &M,
    request: &SimulationRequest<T>,
    cancellation: CancellationToken,
) -> Result<SimulationOutcome<E::Output>, SimulationError>
where
    M: SimulationModel,
    E: SimulationEngine<M>,
{
    request.validate_model(model)?;

    let mut context = SimulationContext::new(
        request.config().clone(),
        cancellation,
    );

    context
        .checkpoint()
        .map_err(SimulationError::ExecutionStopped)?;

    let outcome = engine.run(model, request, &mut context)?;

    outcome.validate()?;

    // The engine's reported counters must agree with the
    // invocation context. Otherwise accounting is ambiguous.
    if outcome.counters() != context.counters() {
        return Err(SimulationError::CounterMismatch {
            context: context.counters(),
            outcome: outcome.counters(),
        });
    }

    Ok(outcome)
}

// =============================================================================
// Errors
// =============================================================================

/// Errors at the scientific simulation integration boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SimulationError {
    /// An identifier was empty or invalid.
    InvalidIdentifier {
        field: &'static str,
    },

    /// The request and supplied model have different identities.
    ModelIdentityMismatch,

    /// Execution stopped at a cooperative checkpoint.
    ExecutionStopped(SimulationStopReason),

    /// A checked counter could not be incremented.
    CounterOverflow(&'static str),

    /// Engine and context accounting disagree.
    CounterMismatch {
        context: SimulationCounters,
        outcome: SimulationCounters,
    },

    /// The engine returned an inconsistent outcome.
    InvalidOutcome(&'static str),

    /// The selected engine does not support the requested operation.
    UnsupportedOperation {
        operation: String,
        domain: SimulationDomain,
    },

    /// The engine reported an execution failure.
    EngineFailure(String),
}

impl fmt::Display for SimulationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field } => {
                write!(f, "invalid simulation {field}")
            }
            Self::ModelIdentityMismatch => {
                f.write_str("simulation request and model identities differ")
            }
            Self::ExecutionStopped(reason) => {
                write!(f, "simulation execution stopped: {reason:?}")
            }
            Self::CounterOverflow(counter) => {
                write!(f, "simulation {counter} counter overflowed")
            }
            Self::CounterMismatch { context, outcome } => write!(
                f,
                "simulation counter mismatch: context={context:?}, outcome={outcome:?}"
            ),
            Self::InvalidOutcome(message) => {
                write!(f, "invalid simulation outcome: {message}")
            }
            Self::UnsupportedOperation { operation, domain } => {
                write!(
                    f,
                    "operation '{operation}' is unsupported for domain '{domain}'"
                )
            }
            Self::EngineFailure(message) => {
                write!(f, "simulation engine failure: {message}")
            }
        }
    }
}

impl Error for SimulationError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn identity() -> SimulationIdentity {
        SimulationIdentity::new(
            "test.model",
            "1",
            SimulationDomain::new("classical.scientific")
                .expect("valid domain"),
        )
        .expect("valid identity")
    }

    struct TestModel {
        identity: SimulationIdentity,
    }

    impl SimulationModel for TestModel {
        fn identity(&self) -> &SimulationIdentity {
            &self.identity
        }
    }

    struct TestEngine;

    impl SimulationEngine<TestModel> for TestEngine {
        type Output = u64;

        fn run<T>(
            &mut self,
            _model: &TestModel,
            _request: &SimulationRequest<T>,
            context: &mut SimulationContext,
        ) -> Result<SimulationOutcome<Self::Output>, SimulationError> {
            context.record_step()?;
            context.record_event()?;

            SimulationOutcome::completed(
                42,
                SimulationStopReason::EngineCompleted,
                context.counters(),
            )
        }
    }

    fn request() -> SimulationRequest<()> {
        SimulationRequest::new(
            SimulationSubject::new("model/reference")
                .expect("valid subject"),
            identity(),
            SimulationStopCondition::UntilQuiescent,
            SimulationConfig::unbounded(),
        )
    }

    #[test]
    fn domain_identifiers_are_open_world() {
        let domain = SimulationDomain::new("future.computing.domain")
            .expect("new domains must not require enum changes");

        assert_eq!(domain.as_str(), "future.computing.domain");
    }

    #[test]
    fn empty_domain_is_rejected() {
        assert!(matches!(
            SimulationDomain::new(" "),
            Err(SimulationError::InvalidIdentifier {
                field: "domain"
            })
        ));
    }

    #[test]
    fn model_identity_is_validated() {
        let model = TestModel {
            identity: identity(),
        };

        assert!(request().validate_model(&model).is_ok());
    }

    #[test]
    fn mismatched_model_is_rejected() {
        let model = TestModel {
            identity: SimulationIdentity::new(
                "different.model",
                "1",
                SimulationDomain::new("classical.scientific")
                    .expect("valid domain"),
            )
            .expect("valid identity"),
        };

        assert_eq!(
            request().validate_model(&model),
            Err(SimulationError::ModelIdentityMismatch)
        );
    }

    #[test]
    fn cancellation_is_shared_between_clones() {
        let token = CancellationToken::new();
        let clone = token.clone();

        clone.cancel();

        assert!(token.is_cancelled());
    }

    #[test]
    fn cancellation_is_checked_at_checkpoint() {
        let token = CancellationToken::new();
        token.cancel();

        let context = SimulationContext::new(
            SimulationConfig::unbounded(),
            token,
        );

        assert_eq!(
            context.checkpoint(),
            Err(SimulationStopReason::Cancelled)
        );
    }

    #[test]
    fn step_budget_is_enforced() {
        let budget = NonZeroU64::new(1).expect("nonzero");

        let mut context = SimulationContext::new(
            SimulationConfig::unbounded().with_step_budget(budget),
            CancellationToken::new(),
        );

        context.record_step().expect("first step");

        assert_eq!(
            context.checkpoint(),
            Err(SimulationStopReason::StepBudgetExceeded)
        );
    }

    #[test]
    fn event_budget_is_enforced() {
        let budget = NonZeroU64::new(1).expect("nonzero");

        let mut context = SimulationContext::new(
            SimulationConfig::unbounded().with_event_budget(budget),
            CancellationToken::new(),
        );

        context.record_event().expect("first event");

        assert_eq!(
            context.checkpoint(),
            Err(SimulationStopReason::EventBudgetExceeded)
        );
    }

    #[test]
    fn counters_use_checked_accounting() {
        let mut context = SimulationContext::new(
            SimulationConfig::unbounded(),
            CancellationToken::new(),
        );

        context.record_step().expect("step");
        context.record_event().expect("event");

        assert_eq!(context.counters().steps(), 1);
        assert_eq!(context.counters().events(), 1);
    }

    #[test]
    fn normal_completion_requires_normal_stop_reason() {
        let result = SimulationOutcome::completed(
            42,
            SimulationStopReason::Cancelled,
            SimulationCounters::default(),
        );

        assert!(matches!(
            result,
            Err(SimulationError::InvalidOutcome(_))
        ));
    }

    #[test]
    fn incomplete_outcome_cannot_claim_normal_completion() {
        let outcome = SimulationOutcome::incomplete(
            42,
            SimulationStopReason::EngineStoppedIncomplete,
            SimulationCounters::default(),
        );

        assert!(outcome.validate().is_ok());
        assert!(!outcome.is_completed());
    }

    #[test]
    fn engine_integration_runs_with_validated_identity() {
        let model = TestModel {
            identity: identity(),
        };

        let mut engine = TestEngine;

        let outcome = run_simulation(
            &mut engine,
            &model,
            &request(),
            CancellationToken::new(),
        )
        .expect("simulation should complete");

        assert!(outcome.is_completed());
        assert_eq!(*outcome.result(), 42);
        assert_eq!(outcome.counters().steps(), 1);
        assert_eq!(outcome.counters().events(), 1);
    }

    #[test]
    fn cancelled_request_does_not_enter_engine() {
        let model = TestModel {
            identity: identity(),
        };

        let mut engine = TestEngine;
        let token = CancellationToken::new();
        token.cancel();

        let result = run_simulation(
            &mut engine,
            &model,
            &request(),
            token,
        );

        assert!(matches!(
            result,
            Err(SimulationError::ExecutionStopped(
                SimulationStopReason::Cancelled
            ))
        ));
    }

    #[test]
    fn empty_subject_is_rejected() {
        assert!(matches!(
            SimulationSubject::new(" "),
            Err(SimulationError::InvalidIdentifier {
                field: "simulation subject"
            })
        ));
    }

    #[test]
    fn incomplete_outcome_rejects_normal_stop_reason() {
        let outcome = SimulationOutcome::incomplete(
            42,
            SimulationStopReason::Quiescent,
            SimulationCounters::default(),
        );

        assert!(matches!(
            outcome.validate(),
            Err(SimulationError::InvalidOutcome(_))
        ));
    }
}
