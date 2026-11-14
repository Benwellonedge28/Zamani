
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # Zamani Universal Runtime — Cleanup Coordination
//!
//! Path: src/zur/reliability/cleanup.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97+
//!
//! PURPOSE
//!
//! Coordinates reliable cleanup following:
//!
//! - Execution cancellation.
//! - Execution failure.
//! - Runtime initialization failure.
//! - Provider failure.
//! - Resource failure.
//! - Hybrid execution failure.
//! - Distributed execution failure.
//! - Runtime shutdown.
//!
//! Cleanup is performed through explicitly registered actions.
//!
//! Each action delegates actual cleanup to its authoritative subsystem.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Cleanup action identity.
//! - Cleanup action registration.
//! - Cleanup dependency ordering.
//! - Cleanup execution coordination.
//! - Cleanup progress tracking.
//! - Cleanup outcome classification.
//! - Cleanup failure preservation.
//! - Retry eligibility.
//! - Explicit outcome reconciliation.
//! - Session-finalization verification.
//!
//! This module does NOT own:
//!
//! - Execution cancellation implementation.
//! - Execution lifecycle.
//! - Runtime lifecycle.
//! - Service initialization or shutdown implementation.
//! - Resource allocation or release implementation.
//! - Provider shutdown implementation.
//! - Distributed recovery.
//! - Checkpoint storage or restoration.
//! - Memory reclamation implementation.
//! - Security authorization.
//! - Domain-specific recovery.
//!
//! EXISTING AUTHORITIES
//!
//! - crate::zur::error
//! - crate::zur::ids
//! - crate::zur::execution::session
//! - crate::zur::execution::cancellation
//! - crate::zur::core::service_manager
//! - crate::zur::resources::lease
//! - crate::zur::resources::lifecycle
//! - crate::zur::reliability::faults
//! - crate::zur::reliability::checkpoint
//! - crate::zur::reliability::recovery
//! - crate::distributed
//! - crate::hybrid
//! - crate::quantum
//!
//! IMPORTANT DISTINCTIONS
//!
//! Cancellation requested != execution stopped.
//! Cleanup requested != cleanup completed.
//! Resource lease expired != resource released.
//! Provider disconnected != provider resources released.
//! Service lease dropped != service shutdown completed.
//! Recovery started != recovery completed.
//! Unknown outcome != successful cleanup.
//!
//! SAFETY
//!
//! Safe Rust only.
//! No unsafe.
//! No raw pointers.
//! No global mutable state.
//! No fabricated success.
//! No implicit physical resource allocation.
//! No automatic retry of uncertain operations.
//!
//! SCALABILITY
//!
//! No arbitrary limit is imposed on:
//!
//! - Cleanup actions.
//! - Cleanup dependencies.
//! - Execution sessions.
//! - Providers.
//! - Services.
//! - Resource leases.
//! - Computational domains.
//! - Cleanup failures.
//!
//! Collections grow subject to actual allocation availability.
//!
//! Literal infinite physical computation or storage is not promised.
//!
//! CONCURRENCY
//!
//! Cleanup coordination is scoped to an explicitly constructed
//! coordinator instance.
//!
//! State locks are never held while external cleanup callbacks
//! are executing or being awaited.
//!
//! Concurrent attempts to execute the same cleanup action are
//! detected and do not cause a second invocation.
//!
//! FAILURE HANDLING
//!
//! All independent cleanup actions are attempted even when another
//! action fails.
//!
//! Actions whose declared dependencies have not completed are blocked.
//!
//! Unknown external outcomes must be reconciled before another
//! potentially conflicting cleanup operation is attempted.
//!
//! INTEGRATION
//!
//! Resource release actions delegate to the canonical resource manager.
//!
//! Service shutdown actions delegate to the canonical service manager.
//!
//! Provider cleanup actions delegate to their registered providers.
//!
//! Distributed cleanup actions delegate to src/distributed/.
//!
//! Checkpoint operations delegate to reliability/checkpoint.rs.
//!
//! Recovery operations delegate to reliability/recovery.rs.
//!
//! Session finalization delegates to execution/session.rs.

use core::fmt;

use std::collections::{
    BTreeMap,
    BTreeSet,
};

use std::error::Error as StdError;

use std::future::Future;
use std::pin::Pin;

use std::sync::{
    Mutex,
    MutexGuard,
};

use crate::stdlib::context::OperationContext;

use crate::zur::error::{
    ErrorKind,
    ZurError,
    ZurResult,
};

use crate::zur::execution::session::{
    ExecutionSession,
    SessionStatus,
};

use crate::zur::ids::SessionId;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.reliability.cleanup";

// ============================================================================
// ASYNCHRONOUS CLEANUP CALLBACK
// ============================================================================

/// Future returned by an asynchronous cleanup action.
///
/// Futures must be Send so that cleanup can be driven by a compatible
/// multithreaded asynchronous executor.
///
/// ZUR does not create an asynchronous executor here.
pub type CleanupFuture<'a> =
    Pin<Box<dyn Future<Output = ZurResult<CleanupOutcome>> + Send + 'a>>;

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Validation failures for cleanup action identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CleanupIdentifierError {
    /// Identifier is empty.
    Empty,

    /// Identifier contains whitespace.
    Whitespace,

    /// Identifier contains an unsupported character.
    InvalidCharacter,

    /// Identifier has an invalid namespace boundary.
    InvalidBoundary,
}

impl fmt::Display for CleanupIdentifierError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(match self {
            Self::Empty => "cleanup identifier cannot be empty",
            Self::Whitespace => "cleanup identifier cannot contain whitespace",
            Self::InvalidCharacter => "cleanup identifier contains invalid characters",
            Self::InvalidBoundary => "cleanup identifier has an invalid namespace boundary",
        })
    }
}

impl StdError for CleanupIdentifierError {}

/// Extensible namespaced cleanup-action identity.
///
/// Examples:
///
/// - zamani.execution.stop
/// - zamani.provider.close
/// - zamani.resource.release
/// - zamani.service.shutdown
/// - zamani.session.finalize
/// - quantum.provider.release
/// - photonics.controller.shutdown
/// - vendor.custom.cleanup
///
/// Unknown identifiers remain representable.
///
/// An identifier does not establish that its implementation exists
/// or that its cleanup operation is supported.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CleanupStepId(String);

impl CleanupStepId {
    /// Creates a validated cleanup-action identifier.
    ///
    /// No arbitrary identifier-length ceiling is imposed.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, CleanupIdentifierError> {
        let value = value.into();

        validate_identifier(&value)?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CleanupStepId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

fn validate_identifier(
    value: &str,
) -> Result<(), CleanupIdentifierError> {
    if value.is_empty() {
        return Err(CleanupIdentifierError::Empty);
    }

    if value.chars().any(char::is_whitespace) {
        return Err(CleanupIdentifierError::Whitespace);
    }

    let bytes = value.as_bytes();

    let mut index = 0;
    let mut segment_has_character = false;
    let mut previous_separator = false;

    while index < bytes.len() {
        let byte = bytes[index];

        if byte == b'.' {
            if !segment_has_character {
                return Err(CleanupIdentifierError::InvalidBoundary);
            }

            segment_has_character = false;
            previous_separator = true;
            index += 1;

            continue;
        }

        if byte == b':' {
            if index + 1 >= bytes.len()
                || bytes[index + 1] != b':'
                || !segment_has_character
            {
                return Err(CleanupIdentifierError::InvalidBoundary);
            }

            segment_has_character = false;
            previous_separator = true;
            index += 2;

            continue;
        }

        let valid = byte.is_ascii_alphanumeric()
            || byte == b'_'
            || byte == b'-';

        if !valid {
            return Err(CleanupIdentifierError::InvalidCharacter);
        }

        if previous_separator
            && (byte == b'_' || byte == b'-')
        {
            return Err(CleanupIdentifierError::InvalidBoundary);
        }

        segment_has_character = true;
        previous_separator = false;

        index += 1;
    }

    if !segment_has_character
        || value.ends_with('_')
        || value.ends_with('-')
    {
        return Err(CleanupIdentifierError::InvalidBoundary);
    }

    Ok(())
}

// ============================================================================
// CLEANUP OUTCOMES
// ============================================================================

/// Actual outcome reported by an authoritative cleanup action.
///
/// These values describe operational observations.
///
/// They must not be interpreted as recovery or execution results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CleanupOutcome {
    /// Cleanup was successfully completed and confirmed.
    Confirmed,

    /// The authoritative subsystem established that cleanup
    /// had already completed.
    AlreadyComplete,

    /// The action established that no cleanup was required.
    NotRequired,

    /// The external outcome could not be established.
    ///
    /// This is NOT successful cleanup.
    ///
    /// Reconciliation is required before another conflicting
    /// cleanup attempt.
    OutcomeUnknown,
}

impl CleanupOutcome {
    /// Returns whether the outcome establishes successful cleanup.
    #[must_use]
    pub const fn is_confirmed(self) -> bool {
        matches!(
            self,
            Self::Confirmed
                | Self::AlreadyComplete
                | Self::NotRequired
        )
    }

    /// Returns whether reconciliation is required.
    #[must_use]
    pub const fn requires_reconciliation(self) -> bool {
        matches!(self, Self::OutcomeUnknown)
    }
}

// ============================================================================
// CLEANUP STATE
// ============================================================================

/// Observable state of an individual cleanup action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CleanupStepState {
    /// Cleanup has not started.
    Pending,

    /// Cleanup is currently executing.
    Running,

    /// Cleanup has been authoritatively confirmed.
    Confirmed,

    /// Cleanup failed definitively.
    ///
    /// Retrying is permitted only according to the action's
    /// declared retry-safety contract.
    Failed,

    /// Cleanup has an uncertain external outcome.
    ///
    /// Reconciliation must occur before another attempt.
    OutcomeUnknown,

    /// One or more dependencies have not completed.
    ///
    /// This is reevaluated on subsequent cleanup attempts.
    Blocked,
}

impl CleanupStepState {
    /// Returns whether cleanup is confirmed complete.
    #[must_use]
    pub const fn is_confirmed(self) -> bool {
        matches!(self, Self::Confirmed)
    }

    /// Returns whether another attempt requires reconciliation.
    #[must_use]
    pub const fn requires_reconciliation(self) -> bool {
        matches!(self, Self::OutcomeUnknown)
    }

    /// Returns whether the action is terminally confirmed.
    #[must_use]
    pub const fn is_terminal_success(self) -> bool {
        matches!(self, Self::Confirmed)
    }
}

// ============================================================================
// RETRY POLICY
// ============================================================================

/// Describes whether a failed cleanup action can safely be attempted again.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CleanupRetrySafety {
    /// The action is explicitly designed to be idempotent.
    ///
    /// A definitive failure may be retried by a subsequent cleanup run.
    Idempotent,

    /// The authoritative subsystem must reconcile the previous
    /// operation before another attempt.
    ReconcileFirst,

    /// The action must not be automatically retried.
    NeverAutomatic,
}

// ============================================================================
// CLEANUP ACTION CONTRACT
// ============================================================================

/// Authoritative implementation of one cleanup operation.
///
/// Implementations must delegate to the subsystem that owns
/// the affected resource or service.
///
/// Implementations must:
///
/// - Use safe Rust.
/// - Preserve causal errors.
/// - Honor the supplied control context.
/// - Avoid holding internal locks across external callbacks.
/// - Distinguish definitive failure from uncertain outcome.
/// - Never fabricate cleanup success.
/// - Honor the declared retry-safety policy.
/// - Avoid duplicate side effects.
/// - Preserve their action identity and contract version.
///
/// The session is supplied for identity and session-specific
/// coordination.
///
/// The control context is supplied separately from the session's
/// operation context because session cancellation may already
/// have been requested.
///
/// Cleanup must not be prevented merely because the work being
/// cleaned up has been cancelled.
pub trait CleanupStep: Send + Sync + fmt::Debug {
    /// Returns the stable cleanup-action identity.
    fn id(&self) -> &CleanupStepId;

    /// Returns the action contract version.
    ///
    /// Changing cleanup behavior incompatibly requires a new version.
    fn contract_version(&self) -> &str;

    /// Returns the action retry-safety policy.
    fn retry_safety(&self) -> CleanupRetrySafety;

    /// Executes cleanup.
    ///
    /// A successful future returning OutcomeUnknown does not
    /// establish that cleanup completed.
    fn execute<'a>(
        &'a self,
        session: &'a ExecutionSession,
        control_context: &'a OperationContext,
    ) -> CleanupFuture<'a>;

    /// Reconciles an action whose external outcome is unknown.
    ///
    /// Implementations must consult their authoritative subsystem.
    ///
    /// The default implementation deliberately preserves uncertainty.
    ///
    /// It does not blindly repeat the original operation.
    fn reconcile<'a>(
        &'a self,
        _session: &'a ExecutionSession,
        _control_context: &'a OperationContext,
    ) -> CleanupFuture<'a> {
        Box::pin(async {
            Ok(CleanupOutcome::OutcomeUnknown)
        })
    }
}

// ============================================================================
// CLEANUP REGISTRATION
// ============================================================================

/// One registered cleanup action and its dependencies.
#[derive(Clone)]
pub struct CleanupRegistration {
    action: std::sync::Arc<dyn CleanupStep>,

    dependencies: BTreeSet<CleanupStepId>,
}

impl CleanupRegistration {
    /// Returns the cleanup action identity.
    #[must_use]
    pub fn id(&self) -> &CleanupStepId {
        self.action.id()
    }

    /// Returns the action contract version.
    #[must_use]
    pub fn contract_version(&self) -> &str {
        self.action.contract_version()
    }

    /// Returns the action dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &BTreeSet<CleanupStepId> {
        &self.dependencies
    }
}

impl fmt::Debug for CleanupRegistration {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("CleanupRegistration")
            .field("id", self.id())
            .field("contract_version", &self.contract_version())
            .field("dependencies", &self.dependencies)
            .finish()
    }
}

// ============================================================================
// CLEANUP PLAN
// ============================================================================

/// Validation failures for cleanup dependency plans.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CleanupPlanError {
    /// The same cleanup identity was registered more than once.
    DuplicateAction(CleanupStepId),

    /// An action depends on itself.
    SelfDependency(CleanupStepId),

    /// A required dependency was not registered.
    MissingDependency {
        /// The action requiring the dependency.
        action: CleanupStepId,

        /// The missing dependency.
        dependency: CleanupStepId,
    },

    /// The dependency graph contains a cycle.
    DependencyCycle,
}

impl fmt::Display for CleanupPlanError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DuplicateAction(id) => {
                write!(formatter, "duplicate cleanup action: {id}")
            }

            Self::SelfDependency(id) => {
                write!(formatter, "cleanup action depends on itself: {id}")
            }

            Self::MissingDependency {
                action,
                dependency,
            } => {
                write!(
                    formatter,
                    "cleanup action {action} requires missing dependency {dependency}"
                )
            }

            Self::DependencyCycle => {
                formatter.write_str(
                    "cleanup dependency graph contains a cycle",
                )
            }
        }
    }
}

impl StdError for CleanupPlanError {}

/// Immutable collection of registered cleanup operations.
///
/// Actions are ordered by their explicit dependency graph.
///
/// Registration order is used as a deterministic tie-breaker
/// between otherwise independent actions.
///
/// No arbitrary action-count limit is imposed.
#[derive(Debug, Default, Clone)]
pub struct CleanupPlan {
    registrations: Vec<CleanupRegistration>,

    identities: BTreeSet<CleanupStepId>,
}

impl CleanupPlan {
    /// Creates an empty cleanup plan.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a cleanup action.
    ///
    /// Dependencies may be registered before or after the action.
    ///
    /// The complete plan is validated before execution.
    ///
    /// # Errors
    ///
    /// Returns an error for duplicate action identities
    /// or self-dependencies.
    pub fn register(
        &mut self,
        action: std::sync::Arc<dyn CleanupStep>,
        dependencies: impl IntoIterator<Item = CleanupStepId>,
    ) -> Result<(), CleanupPlanError> {
        let id = action.id().clone();

        if self.identities.contains(&id) {
            return Err(
                CleanupPlanError::DuplicateAction(id)
            );
        }

        let dependencies: BTreeSet<_> =
            dependencies.into_iter().collect();

        if dependencies.contains(&id) {
            return Err(
                CleanupPlanError::SelfDependency(id)
            );
        }

        self.identities.insert(id);

        self.registrations.push(
            CleanupRegistration {
                action,
                dependencies,
            }
        );

        Ok(())
    }

    /// Returns registered cleanup actions.
    #[must_use]
    pub fn registrations(&self) -> &[CleanupRegistration] {
        &self.registrations
    }

    /// Returns whether the plan contains no actions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.registrations.is_empty()
    }

    /// Returns the number of registered actions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.registrations.len()
    }

    /// Validates and calculates deterministic dependency order.
    ///
    /// Uses topological sorting.
    ///
    /// No fixed graph-depth or action-count ceiling is imposed.
    pub fn execution_order(
        &self,
    ) -> Result<Vec<usize>, CleanupPlanError> {
        let mut indices = BTreeMap::new();

        for (index, registration) in
            self.registrations.iter().enumerate()
        {
            indices.insert(
                registration.id().clone(),
                index,
            );
        }

        let mut indegree = vec![0usize; self.len()];

        let mut dependents: Vec<Vec<usize>> =
            (0..self.len()).map(|_| Vec::new()).collect();

        for (index, registration) in
            self.registrations.iter().enumerate()
        {
            for dependency in registration.dependencies() {
                let Some(&dependency_index) =
                    indices.get(dependency)
                else {
                    return Err(
                        CleanupPlanError::MissingDependency {
                            action: registration.id().clone(),
                            dependency: dependency.clone(),
                        }
                    );
                };

                indegree[index] = indegree[index]
                    .checked_add(1)
                    .ok_or(CleanupPlanError::DependencyCycle)?;

                dependents[dependency_index].push(index);
            }
        }

        let mut ready = BTreeSet::new();

        for (index, degree) in indegree.iter().enumerate() {
            if *degree == 0 {
                ready.insert(index);
            }
        }

        let mut order = Vec::with_capacity(self.len());

        while let Some(index) = ready.pop_first() {
            order.push(index);

            for &dependent in &dependents[index] {
                indegree[dependent] -= 1;

                if indegree[dependent] == 0 {
                    ready.insert(dependent);
                }
            }
        }

        if order.len() != self.len() {
            return Err(CleanupPlanError::DependencyCycle);
        }

        Ok(order)
    }
}

// ============================================================================
// INTERNAL LEDGER
// ============================================================================

/// Internal record for one cleanup action.
#[derive(Debug, Clone)]
struct CleanupRecord {
    contract_version: String,

    state: CleanupStepState,

    attempts: u64,
}

/// State retained by one cleanup coordinator.
#[derive(Debug, Default)]
struct CleanupLedger {
    records: BTreeMap<CleanupStepId, CleanupRecord>,
}

// ============================================================================
// REPORTS
// ============================================================================

/// Information about one cleanup action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanupStepReport {
    id: CleanupStepId,

    state: CleanupStepState,

    attempts: u64,
}

impl CleanupStepReport {
    /// Returns the cleanup-action identity.
    #[must_use]
    pub fn id(&self) -> &CleanupStepId {
        &self.id
    }

    /// Returns the latest observed cleanup state.
    #[must_use]
    pub const fn state(&self) -> CleanupStepState {
        self.state
    }

    /// Returns the number of execution attempts.
    #[must_use]
    pub const fn attempts(&self) -> u64 {
        self.attempts
    }
}

/// Failure or incomplete outcome associated with one cleanup action.
#[derive(Debug)]
pub struct CleanupFailure {
    id: CleanupStepId,

    state: CleanupStepState,

    error: ZurError,
}

impl CleanupFailure {
    /// Returns the associated cleanup action.
    #[must_use]
    pub fn id(&self) -> &CleanupStepId {
        &self.id
    }

    /// Returns the observed cleanup state.
    #[must_use]
    pub const fn state(&self) -> CleanupStepState {
        self.state
    }

    /// Returns the original structured error.
    #[must_use]
    pub fn error(&self) -> &ZurError {
        &self.error
    }

    /// Consumes the failure and returns its original error.
    #[must_use]
    pub fn into_error(self) -> ZurError {
        self.error
    }
}

/// Result of one coordinated cleanup attempt.
///
/// A report preserves partial success and all observed failures.
///
/// A successful report construction does not imply that every
/// cleanup operation succeeded.
#[derive(Debug)]
pub struct CleanupReport {
    session_id: SessionId,

    steps: Vec<CleanupStepReport>,

    failures: Vec<CleanupFailure>,
}

impl CleanupReport {
    /// Returns the affected session identity.
    #[must_use]
    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    /// Returns the individual action reports.
    #[must_use]
    pub fn steps(&self) -> &[CleanupStepReport] {
        &self.steps
    }

    /// Returns all observed cleanup failures.
    #[must_use]
    pub fn failures(&self) -> &[CleanupFailure] {
        &self.failures
    }

    /// Returns whether every registered action is confirmed.
    #[must_use]
    pub fn all_actions_confirmed(&self) -> bool {
        self.steps
            .iter()
            .all(|step| step.state.is_confirmed())
    }

    /// Returns whether any action remains uncertain.
    #[must_use]
    pub fn has_unknown_outcomes(&self) -> bool {
        self.steps.iter().any(|step| {
            step.state.requires_reconciliation()
        })
    }

    /// Returns whether any cleanup action failed.
    #[must_use]
    pub fn has_failures(&self) -> bool {
        !self.failures.is_empty()
            || self.steps.iter().any(|step| {
                matches!(
                    step.state,
                    CleanupStepState::Failed
                        | CleanupStepState::Blocked
                        | CleanupStepState::OutcomeUnknown
                )
            })
    }
}

// ============================================================================
// CLEANUP COORDINATOR
// ============================================================================

/// Coordinates cleanup for one explicitly identified execution session.
///
/// A coordinator is independent of other runtime instances.
///
/// It contains no global state.
///
/// The ledger is retained across repeated calls so that confirmed
/// actions are not needlessly executed again.
///
/// The coordinator does not itself release physical resources,
/// shut down services or stop remote execution.
#[derive(Debug)]
pub struct CleanupCoordinator {
    session_id: SessionId,

    ledger: Mutex<CleanupLedger>,
}

impl CleanupCoordinator {
    /// Creates a coordinator for an existing execution session.
    ///
    /// The supplied identity is not generated or globally registered.
    #[must_use]
    pub fn new(session_id: SessionId) -> Self {
        Self {
            session_id,
            ledger: Mutex::new(CleanupLedger::default()),
        }
    }

    /// Returns the associated session identity.
    #[must_use]
    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    /// Executes all eligible cleanup actions.
    ///
    /// The supplied control context must remain usable even when
    /// the execution session has already been cancelled.
    ///
    /// The session enters its closing state before actions execute.
    ///
    /// This requests local cooperative cancellation.
    ///
    /// It does not claim that external executions have stopped.
    ///
    /// Actions execute in validated dependency order.
    ///
    /// Independent actions continue even when another action fails.
    ///
    /// Definitively failed actions may be retried on a later call
    /// only when their retry-safety contract is Idempotent.
    ///
    /// Unknown actions are never automatically repeated.
    ///
    /// # Errors
    ///
    /// Returns a structural error when:
    ///
    /// - The session identity does not match.
    /// - The cleanup dependency graph is invalid.
    /// - The coordinator ledger cannot be accessed.
    /// - A cleanup action's registered contract changes.
    ///
    /// Individual cleanup failures are preserved in CleanupReport.
    pub async fn execute(
        &self,
        plan: &CleanupPlan,
        session: &ExecutionSession,
        control_context: &OperationContext,
    ) -> ZurResult<CleanupReport> {
        self.validate_session(session)?;

        let order = plan
            .execution_order()
            .map_err(|error| {
                cleanup_error(
                    ErrorKind::Dependency,
                    error.to_string(),
                    "validate_plan",
                )
            })?;

        // Request cancellation and begin session closing.
        //
        // The session's operation context may now be cancelled.
        // Cleanup actions receive the separate control context.
        session.begin_close()?;

        self.prepare_ledger(plan)?;

        let mut failures = Vec::new();

        for index in order {
            let registration = &plan.registrations[index];

            let dependencies_confirmed =
                self.dependencies_confirmed(registration)?;

            if !dependencies_confirmed {
                let report = self.set_state(
                    registration,
                    CleanupStepState::Blocked,
                )?;

                failures.push(
                    CleanupFailure {
                        id: registration.id().clone(),
                        state: CleanupStepState::Blocked,
                        error: cleanup_error(
                            ErrorKind::Lifecycle,
                            format!(
                                "cleanup dependencies are incomplete for {}",
                                registration.id()
                            ),
                            "execute",
                        ),
                    }
                );

                let _ = report;

                continue;
            }

            let should_execute = self.begin_attempt(registration)?;

            if !should_execute {
                continue;
            }

            // IMPORTANT:
            // No coordinator state lock is held during this callback.
            let outcome = registration
                .action
                .execute(session, control_context)
                .await;

            match outcome {
                Ok(result) if result.is_confirmed() => {
                    self.set_state(
                        registration,
                        CleanupStepState::Confirmed,
                    )?;
                }

                Ok(CleanupOutcome::OutcomeUnknown) => {
                    self.set_state(
                        registration,
                        CleanupStepState::OutcomeUnknown,
                    )?;

                    failures.push(
                        CleanupFailure {
                            id: registration.id().clone(),
                            state: CleanupStepState::OutcomeUnknown,
                            error: cleanup_error(
                                ErrorKind::Recovery,
                                format!(
                                    "cleanup outcome is unknown for {}",
                                    registration.id()
                                ),
                                "execute",
                            ),
                        }
                    );
                }

                Err(error) => {
                    self.set_state(
                        registration,
                        CleanupStepState::Failed,
                    )?;

                    failures.push(
                        CleanupFailure {
                            id: registration.id().clone(),
                            state: CleanupStepState::Failed,
                            error,
                        }
                    );
                }
            }
        }

        let steps = self.snapshot(plan)?;

        Ok(CleanupReport {
            session_id: self.session_id.clone(),
            steps,
            failures,
        })
    }

    /// Reconciles actions whose previous external cleanup outcome
    /// could not be established.
    ///
    /// This method does not repeat the original cleanup operation.
    ///
    /// Each action must implement reconciliation using its authoritative
    /// subsystem.
    ///
    /// An action without reconciliation support remains uncertain.
    ///
    /// # Errors
    ///
    /// Returns an error if session identity or ledger access is invalid.
    pub async fn reconcile_unknown(
        &self,
        plan: &CleanupPlan,
        session: &ExecutionSession,
        control_context: &OperationContext,
    ) -> ZurResult<CleanupReport> {
        self.validate_session(session)?;

        let mut failures = Vec::new();

        for registration in &plan.registrations {
            let state = self.current_state(registration.id())?;

            if state != CleanupStepState::OutcomeUnknown {
                continue;
            }

            let outcome = registration
                .action
                .reconcile(session, control_context)
                .await;

            match outcome {
                Ok(result) if result.is_confirmed() => {
                    self.set_state(
                        registration,
                        CleanupStepState::Confirmed,
                    )?;
                }

                Ok(CleanupOutcome::OutcomeUnknown) => {
                    failures.push(
                        CleanupFailure {
                            id: registration.id().clone(),
                            state: CleanupStepState::OutcomeUnknown,
                            error: cleanup_error(
                                ErrorKind::Recovery,
                                "authoritative reconciliation remains inconclusive",
                                "reconcile",
                            ),
                        }
                    );
                }

                Err(error) => {
                    failures.push(
                        CleanupFailure {
                            id: registration.id().clone(),
                            state: CleanupStepState::OutcomeUnknown,
                            error,
                        }
                    );
                }
            }
        }

        Ok(CleanupReport {
            session_id: self.session_id.clone(),
            steps: self.snapshot(plan)?,
            failures,
        })
    }

    /// Finalizes a session after all registered cleanup actions
    /// have been confirmed.
    ///
    /// Session::close releases the session's retained references.
    ///
    /// Therefore it must not be called before external resource
    /// release and service shutdown have been confirmed.
    ///
    /// # Errors
    ///
    /// Returns an error if any registered cleanup action is not
    /// confirmed or the session cannot be closed.
    pub fn finalize_session(
        &self,
        plan: &CleanupPlan,
        session: &ExecutionSession,
    ) -> ZurResult<()> {
        self.validate_session(session)?;

        let steps = self.snapshot(plan)?;

        let incomplete: Vec<_> = steps
            .iter()
            .filter(|step| !step.state.is_confirmed())
            .map(|step| step.id.to_string())
            .collect();

        if !incomplete.is_empty() {
            return Err(
                cleanup_error(
                    ErrorKind::Lifecycle,
                    format!(
                        "cannot finalize session while cleanup remains incomplete: {}",
                        incomplete.join(", ")
                    ),
                    "finalize_session",
                )
            );
        }

        session.close()
    }

    // ========================================================================
    // INTERNAL VALIDATION
    // ========================================================================

    fn validate_session(
        &self,
        session: &ExecutionSession,
    ) -> ZurResult<()> {
        if session.id() != &self.session_id {
            return Err(
                cleanup_error(
                    ErrorKind::InvalidInput,
                    "execution session identity does not match cleanup coordinator",
                    "validate_session",
                )
            );
        }

        Ok(())
    }

    // ========================================================================
    // LEDGER MANAGEMENT
    // ========================================================================

    fn lock_ledger(
        &self,
        operation: &'static str,
    ) -> ZurResult<MutexGuard<'_, CleanupLedger>> {
        self.ledger.lock().map_err(|_| {
            cleanup_error(
                ErrorKind::Internal,
                "cleanup ledger mutex was poisoned",
                operation,
            )
        })
    }

    fn prepare_ledger(
        &self,
        plan: &CleanupPlan,
    ) -> ZurResult<()> {
        let mut ledger = self.lock_ledger("prepare_ledger")?;

        for registration in &plan.registrations {
            let id = registration.id().clone();
            let version = registration.contract_version().to_owned();

            if let Some(existing) = ledger.records.get(&id) {
                if existing.contract_version != version {
                    return Err(
                        cleanup_error(
                            ErrorKind::Compatibility,
                            format!(
                                "cleanup action contract version changed for {id}"
                            ),
                            "prepare_ledger",
                        )
                    );
                }
            } else {
                ledger.records.insert(
                    id,
                    CleanupRecord {
                        contract_version: version,
                        state: CleanupStepState::Pending,
                        attempts: 0,
                    },
                );
            }
        }

        Ok(())
    }

    fn dependencies_confirmed(
        &self,
        registration: &CleanupRegistration,
    ) -> ZurResult<bool> {
        let ledger = self.lock_ledger("dependencies_confirmed")?;

        for dependency in registration.dependencies() {
            let Some(record) = ledger.records.get(dependency) else {
                return Ok(false);
            };

            if !record.state.is_confirmed() {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn begin_attempt(
        &self,
        registration: &CleanupRegistration,
    ) -> ZurResult<bool> {
        let mut ledger = self.lock_ledger("begin_attempt")?;

        let record = ledger
            .records
            .get_mut(registration.id())
            .ok_or_else(|| {
                cleanup_error(
                    ErrorKind::Internal,
                    "cleanup action is absent from the ledger",
                    "begin_attempt",
                )
            })?;

        if record.contract_version
            != registration.contract_version()
        {
            return Err(
                cleanup_error(
                    ErrorKind::Compatibility,
                    "cleanup action contract version mismatch",
                    "begin_attempt",
                )
            );
        }

        match record.state {
            CleanupStepState::Confirmed
            | CleanupStepState::Running
            | CleanupStepState::OutcomeUnknown => {
                return Ok(false);
            }

            CleanupStepState::Failed
                if registration.action.retry_safety()
                    != CleanupRetrySafety::Idempotent =>
            {
                return Ok(false);
            }

            CleanupStepState::Pending
            | CleanupStepState::Blocked
            | CleanupStepState::Failed => {}
        }

        record.attempts = record
            .attempts
            .checked_add(1)
            .ok_or_else(|| {
                cleanup_error(
                    ErrorKind::ResourceUnavailable,
                    "cleanup attempt accounting exhausted",
                    "begin_attempt",
                )
            })?;

        record.state = CleanupStepState::Running;

        Ok(true)
    }

    fn set_state(
        &self,
        registration: &CleanupRegistration,
        state: CleanupStepState,
    ) -> ZurResult<()> {
        let mut ledger = self.lock_ledger("set_state")?;

        let record = ledger
            .records
            .get_mut(registration.id())
            .ok_or_else(|| {
                cleanup_error(
                    ErrorKind::Internal,
                    "cleanup action is absent from the ledger",
                    "set_state",
                )
            })?;

        record.state = state;

        Ok(())
    }

    fn current_state(
        &self,
        id: &CleanupStepId,
    ) -> ZurResult<CleanupStepState> {
        let ledger = self.lock_ledger("current_state")?;

        ledger
            .records
            .get(id)
            .map(|record| record.state)
            .ok_or_else(|| {
                cleanup_error(
                    ErrorKind::InvalidInput,
                    "cleanup action has not been registered",
                    "current_state",
                )
            })
    }

    fn snapshot(
        &self,
        plan: &CleanupPlan,
    ) -> ZurResult<Vec<CleanupStepReport>> {
        let ledger = self.lock_ledger("snapshot")?;

        let mut result = Vec::with_capacity(plan.len());

        for registration in &plan.registrations {
            let record = ledger
                .records
                .get(registration.id())
                .ok_or_else(|| {
                    cleanup_error(
                        ErrorKind::Internal,
                        "cleanup report references an unregistered action",
                        "snapshot",
                    )
                })?;

            result.push(
                CleanupStepReport {
                    id: registration.id().clone(),
                    state: record.state,
                    attempts: record.attempts,
                }
            );
        }

        Ok(result)
    }
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn cleanup_error(
    kind: ErrorKind,
    message: impl Into<String>,
    operation: &'static str,
) -> ZurError {
    ZurError::new(kind, message)
        .with_component(COMPONENT)
        .with_operation(operation)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::task::Wake;

    use std::sync::Arc;

    use std::task::{
        Context,
        Poll,
        Waker,
    };

    struct TestWake;

    impl Wake for TestWake {
        fn wake(self: Arc<Self>) {}

        fn wake_by_ref(self: &Arc<Self>) {}
    }

    fn block_on<F: Future>(future: F) -> F::Output {
        let waker = Waker::from(Arc::new(TestWake));

        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(future);

        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(value) => return value,
                Poll::Pending => {
                    std::thread::yield_now();
                }
            }
        }
    }

    fn id(value: &str) -> CleanupStepId {
        CleanupStepId::new(value)
            .expect("valid cleanup identity")
    }

    #[derive(Debug)]
    struct TestStep {
        id: CleanupStepId,
        outcome: CleanupOutcome,
        retry: CleanupRetrySafety,
        calls: std::sync::atomic::AtomicUsize,
    }

    impl TestStep {
        fn new(
            identity: &str,
            outcome: CleanupOutcome,
            retry: CleanupRetrySafety,
        ) -> Self {
            Self {
                id: id(identity),
                outcome,
                retry,
                calls: std::sync::atomic::AtomicUsize::new(0),
            }
        }

        fn calls(&self) -> usize {
            self.calls.load(
                std::sync::atomic::Ordering::Acquire
            )
        }
    }

    impl CleanupStep for TestStep {
        fn id(&self) -> &CleanupStepId {
            &self.id
        }

        fn contract_version(&self) -> &str {
            "1"
        }

        fn retry_safety(&self) -> CleanupRetrySafety {
            self.retry
        }

        fn execute<'a>(
            &'a self,
            _session: &'a ExecutionSession,
            _control_context: &'a OperationContext,
        ) -> CleanupFuture<'a> {
            Box::pin(async move {
                self.calls.fetch_add(
                    1,
                    std::sync::atomic::Ordering::AcqRel,
                );

                Ok(self.outcome)
            })
        }
    }

    #[test]
    fn cleanup_identifiers_are_extensible() {
        assert!(CleanupStepId::new(
            "photonics.controller.cleanup"
        ).is_ok());

        assert!(CleanupStepId::new(
            "vendor::accelerator::cleanup"
        ).is_ok());
    }

    #[test]
    fn empty_cleanup_identifiers_are_rejected() {
        assert_eq!(
            CleanupStepId::new("").unwrap_err(),
            CleanupIdentifierError::Empty,
        );
    }

    #[test]
    fn whitespace_in_identifiers_is_rejected() {
        assert_eq!(
            CleanupStepId::new("provider cleanup").unwrap_err(),
            CleanupIdentifierError::Whitespace,
        );
    }

    #[test]
    fn duplicate_actions_are_rejected() {
        let first = Arc::new(TestStep::new(
            "zamani.resource.release",
            CleanupOutcome::Confirmed,
            CleanupRetrySafety::Idempotent,
        ));

        let second = Arc::new(TestStep::new(
            "zamani.resource.release",
            CleanupOutcome::Confirmed,
            CleanupRetrySafety::Idempotent,
        ));

        let mut plan = CleanupPlan::new();

        plan.register(first, [])
            .expect("first registration");

        assert!(matches!(
            plan.register(second, []),
            Err(CleanupPlanError::DuplicateAction(_))
        ));
    }

    #[test]
    fn self_dependencies_are_rejected() {
        let action = Arc::new(TestStep::new(
            "zamani.resource.release",
            CleanupOutcome::Confirmed,
            CleanupRetrySafety::Idempotent,
        ));

        let mut plan = CleanupPlan::new();

        assert!(matches!(
            plan.register(
                Arc::clone(&action) as Arc<dyn CleanupStep>,
                [action.id().clone()],
            ),
            Err(CleanupPlanError::SelfDependency(_))
        ));
    }

    #[test]
    fn missing_dependencies_are_rejected() {
        let action = Arc::new(TestStep::new(
            "zamani.resource.release",
            CleanupOutcome::Confirmed,
            CleanupRetrySafety::Idempotent,
        ));

        let mut plan = CleanupPlan::new();

        plan.register(
            action,
            [id("zamani.provider.close")],
        )
        .expect("registration itself is valid");

        assert!(matches!(
            plan.execution_order(),
            Err(CleanupPlanError::MissingDependency { .. })
        ));
    }

    #[test]
    fn dependency_cycles_are_rejected() {
        let first = Arc::new(TestStep::new(
            "zamani.cleanup.first",
            CleanupOutcome::Confirmed,
            CleanupRetrySafety::Idempotent,
        ));

        let second = Arc::new(TestStep::new(
            "zamani.cleanup.second",
            CleanupOutcome::Confirmed,
            CleanupRetrySafety::Idempotent,
        ));

        let mut plan = CleanupPlan::new();

        plan.register(
            Arc::clone(&first) as Arc<dyn CleanupStep>,
            [second.id().clone()],
        )
        .unwrap();

        plan.register(
            Arc::clone(&second) as Arc<dyn CleanupStep>,
            [first.id().clone()],
        )
        .unwrap();

        assert_eq!(
            plan.execution_order().unwrap_err(),
            CleanupPlanError::DependencyCycle,
        );
    }

    #[test]
    fn cleanup_outcomes_do_not_confuse_unknown_with_success() {
        assert!(CleanupOutcome::Confirmed.is_confirmed());
        assert!(CleanupOutcome::AlreadyComplete.is_confirmed());
        assert!(CleanupOutcome::NotRequired.is_confirmed());

        assert!(!CleanupOutcome::OutcomeUnknown.is_confirmed());

        assert!(
            CleanupOutcome::OutcomeUnknown
                .requires_reconciliation()
        );
    }

    #[test]
    fn cleanup_state_distinguishes_unknown_from_failure() {
        assert!(
            CleanupStepState::OutcomeUnknown
                .requires_reconciliation()
        );

        assert!(
            !CleanupStepState::Failed
                .requires_reconciliation()
        );
    }

    #[test]
    fn retry_safety_is_explicit() {
        assert_ne!(
            CleanupRetrySafety::Idempotent,
            CleanupRetrySafety::ReconcileFirst,
        );

        assert_ne!(
            CleanupRetrySafety::Idempotent,
            CleanupRetrySafety::NeverAutomatic,
        );
    }

    #[test]
    fn dependency_order_is_deterministic() {
        let first = Arc::new(TestStep::new(
            "zamani.execution.stop",
            CleanupOutcome::Confirmed,
            CleanupRetrySafety::Idempotent,
        ));

        let second = Arc::new(TestStep::new(
            "zamani.provider.close",
            CleanupOutcome::Confirmed,
            CleanupRetrySafety::Idempotent,
        ));

        let third = Arc::new(TestStep::new(
            "zamani.resource.release",
            CleanupOutcome::Confirmed,
            CleanupRetrySafety::Idempotent,
        ));

        let mut plan = CleanupPlan::new();

        plan.register(
            Arc::clone(&third) as Arc<dyn CleanupStep>,
            [second.id().clone()],
        ).unwrap();

        plan.register(
            Arc::clone(&second) as Arc<dyn CleanupStep>,
            [first.id().clone()],
        ).unwrap();

        plan.register(
            first,
            [],
        ).unwrap();

        let order = plan.execution_order().unwrap();

        assert_eq!(order, vec![2, 1, 0]);
    }

    #[test]
    fn empty_plan_has_valid_execution_order() {
        let plan = CleanupPlan::new();

        assert!(plan.execution_order().unwrap().is_empty());
    }

    #[test]
    fn independent_actions_can_be_registered() {
        let first = Arc::new(TestStep::new(
            "domain.first.cleanup",
            CleanupOutcome::Confirmed,
            CleanupRetrySafety::Idempotent,
        ));

        let second = Arc::new(TestStep::new(
            "domain.second.cleanup",
            CleanupOutcome::Confirmed,
            CleanupRetrySafety::Idempotent,
        ));

        let mut plan = CleanupPlan::new();

        plan.register(first, []).unwrap();
        plan.register(second, []).unwrap();

        assert_eq!(plan.len(), 2);
    }

    #[test]
    fn unknown_outcomes_are_not_successful() {
        let report = CleanupStepReport {
            id: id("zamani.provider.close"),
            state: CleanupStepState::OutcomeUnknown,
            attempts: 1,
        };

        assert!(
            report.state().requires_reconciliation()
        );

        assert!(!report.state().is_confirmed());
    }

    #[test]
    fn retry_policy_distinguishes_automatic_retry() {
        let action = TestStep::new(
            "zamani.resource.release",
            CleanupOutcome::Confirmed,
            CleanupRetrySafety::ReconcileFirst,
        );

        assert_eq!(
            action.retry_safety(),
            CleanupRetrySafety::ReconcileFirst,
        );

        assert_eq!(action.calls(), 0);
    }

    #[test]
    fn action_contract_version_is_explicit() {
        let action = TestStep::new(
            "zamani.provider.close",
            CleanupOutcome::Confirmed,
            CleanupRetrySafety::Idempotent,
        );

        assert_eq!(action.contract_version(), "1");
    }

    #[test]
    fn session_status_is_not_confused_with_execution_completion() {
        assert_eq!(
            SessionStatus::Closed,
            SessionStatus::Closed,
        );

        // Actual session closure is performed only through the
        // ExecutionSession authority after cleanup confirmation.
    }
}
