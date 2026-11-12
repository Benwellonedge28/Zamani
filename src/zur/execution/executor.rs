
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Execution Executor
//!
//! Path: src/zur/execution/executor.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! PURPOSE
//!
//! Coordinates the lifecycle of an execution accepted by a canonical
//! ZUR ExecutionEngine.
//!
//! RESPONSIBILITIES
//!
//! - Submit validated execution requests.
//! - Verify accepted execution handles.
//! - Observe actual execution state.
//! - Monitor execution until a terminal observation is obtained.
//! - Request cooperative cancellation.
//! - Collect structured execution results.
//! - Preserve partial results and failure information.
//! - Reject inconsistent execution observations.
//!
//! ARCHITECTURAL OWNERSHIP
//!
//! This module does not implement another execution engine.
//!
//! It consumes:
//!
//! - zur::contracts::execution::ExecutionEngine
//! - zur::contracts::execution::ExecutionRequest
//! - zur::contracts::execution::ExecutionHandle
//! - zur::contracts::execution::ExecutionObservation
//! - zur::execution::result::ExecutionResult
//! - zur::execution::result::RecoveryReport
//! - zur::error
//! - stdlib::context::OperationContext
//!
//! The execution engine remains responsible for:
//!
//! - Artifact validation.
//! - Execution-plan validation.
//! - ABI compatibility.
//! - Authorization.
//! - Provider availability.
//! - Resource acquisition.
//! - Actual program invocation.
//! - Provider-specific execution.
//! - Resource release.
//!
//! The executor does not duplicate these responsibilities.
//!
//! IMPORTANT INVARIANTS
//!
//! Submission is not execution completion.
//! Cancellation requested is not cancellation confirmed.
//! Provider registration is not provider availability.
//! Feasibility is not resource reservation.
//! A successful observation is not fabricated by the executor.
//!
//! SCALABILITY
//!
//! No arbitrary limits are imposed on:
//!
//! - Execution count.
//! - Execution duration.
//! - Result references.
//! - Providers.
//! - Computational domains.
//! - Hardware devices.
//! - Tasks.
//! - Distributed participants.
//! - Resource quantities.
//!
//! Actual scalability depends on available memory, execution
//! infrastructure, providers and physical resources.
//!
//! SAFETY
//!
//! Safe Rust only.
//! No unsafe.
//! No raw pointers.
//! No global mutable state.
//! No fabricated execution success.

use core::fmt;

use crate::stdlib::context::OperationContext;

use crate::zur::contracts::execution::{
    CancellationDisposition,
    ExecutionEngine,
    ExecutionFuture,
    ExecutionHandle,
    ExecutionObservation,
    ExecutionRequest,
    ExecutionStatus,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::execution::result::{
    ExecutionResult,
    RecoveryReport,
};

use crate::zur::ids::ExecutionId;


// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.execution.executor";


// ============================================================================
// EXECUTION WAITING CONTRACT
// ============================================================================

/// Supplies an asynchronous waiting operation between execution
/// observations.
///
/// Waiting is deliberately delegated to the embedding execution
/// environment.
///
/// Implementations may use:
///
/// - An asynchronous timer.
/// - An event notification.
/// - A provider notification.
/// - An operating-system event.
/// - A distributed execution notification.
/// - Another platform-supported waiting mechanism.
///
/// A waiter should return promptly when cancellation is requested.
///
/// It must not block an asynchronous executor thread unnecessarily.
///
/// This interface does not select an execution provider or scheduler.
pub trait ExecutionWaiter: Send + Sync {

    /// Waits before another execution observation.
    ///
    /// The supplied context identifies the execution monitoring
    /// operation and carries cooperative cancellation.
    ///
    /// # Errors
    ///
    /// Returns an error if waiting fails or the operation is cancelled.
    fn wait<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ExecutionFuture<'a, ()>;
}


// ============================================================================
// EXECUTION OUTCOME
// ============================================================================

/// Complete outcome returned after monitoring an execution.
///
/// The accepted execution handle is retained alongside the result.
///
/// This permits callers to correlate the final result with the
/// original execution authority and provider reference.
#[derive(Debug)]
pub struct ExecutionOutcome {

    handle: ExecutionHandle,

    result: ExecutionResult,
}

impl ExecutionOutcome {

    /// Creates an execution outcome.
    ///
    /// The result must identify the same execution as the handle.
    pub fn new(
        handle: ExecutionHandle,
        result: ExecutionResult,
    ) -> ZurResult<Self> {

        if handle.execution_id() != result.execution_id() {
            return Err(
                ZurError::compatibility(
                    "execution outcome handle and result identify different executions",
                )
                .with_component(COMPONENT)
                .with_operation("outcome.new")
                .with_execution(
                    handle.execution_id().to_string(),
                ),
            );
        }

        Ok(Self {
            handle,
            result,
        })
    }

    /// Returns the canonical execution handle.
    #[must_use]
    pub fn handle(&self) -> &ExecutionHandle {
        &self.handle
    }

    /// Returns the canonical execution result.
    #[must_use]
    pub fn result(&self) -> &ExecutionResult {
        &self.result
    }

    /// Returns the canonical execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        self.handle.execution_id()
    }

    /// Returns whether execution fully completed successfully.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.result.is_successful()
    }

    /// Returns whether execution partially completed.
    #[must_use]
    pub fn is_partial(&self) -> bool {
        self.result.is_partial()
    }

    /// Returns whether execution failed.
    #[must_use]
    pub fn is_failed(&self) -> bool {
        self.result.is_failed()
    }

    /// Returns whether execution was confirmed as cancelled.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.result.is_cancelled()
    }

    /// Returns whether execution timed out.
    #[must_use]
    pub fn is_timed_out(&self) -> bool {
        self.result.is_timed_out()
    }
}


// ============================================================================
// EXECUTION EXECUTOR
// ============================================================================

/// Stateless coordinator for canonical ZUR execution.
///
/// All execution authority and waiting behavior is explicitly injected.
///
/// This object does not maintain a global execution registry or queue.
///
/// Multiple executors can operate concurrently when their supplied
/// execution engines and waiters support concurrent access.
pub struct ExecutionExecutor<'a> {

    engine: &'a dyn ExecutionEngine,

    waiter: &'a dyn ExecutionWaiter,
}

impl<'a> ExecutionExecutor<'a> {

    /// Constructs an execution executor.
    ///
    /// No provider discovery, hardware discovery, resource acquisition
    /// or global initialization occurs during construction.
    #[must_use]
    pub const fn new(
        engine: &'a dyn ExecutionEngine,
        waiter: &'a dyn ExecutionWaiter,
    ) -> Self {
        Self {
            engine,
            waiter,
        }
    }


    // ------------------------------------------------------------------------
    // SUBMISSION
    // ------------------------------------------------------------------------

    /// Submits a validated execution request.
    ///
    /// The canonical execution engine remains responsible for validating
    /// artifacts, plans, ABI compatibility, authorization, providers
    /// and resources.
    ///
    /// Before returning, this method verifies that the accepted handle
    /// identifies the exact execution, artifact and plan requested.
    ///
    /// A returned handle proves acceptance only.
    ///
    /// It does not prove that execution has started or completed.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The request is already cancelled.
    /// - The engine rejects submission.
    /// - Submission produces an indeterminate failure.
    /// - The returned handle identifies another execution.
    /// - The returned handle identifies another artifact.
    /// - The returned handle identifies another execution plan.
    pub fn submit<'b>(
        &'b self,
        request: ExecutionRequest<'b>,
    ) -> ExecutionFuture<'b, ExecutionHandle> {

        Box::pin(async move {

            let execution_id =
                request.execution_id().clone();

            let artifact =
                request.artifact().clone();

            let plan =
                request.plan().clone();

            // Check cancellation before transferring the request.
            check_cancelled(
                request.context(),
                &execution_id,
                "submit",
            )?;

            // The canonical engine performs actual submission.
            let handle = self.engine
                .submit(request)
                .await
                .map_err(|error| {
                    error
                        .with_component(COMPONENT)
                        .with_operation("submit")
                        .with_execution(
                            execution_id.to_string(),
                        )
                })?;

            // Verify the actual accepted execution identity.
            if handle.execution_id() != &execution_id {
                return Err(
                    compatibility_error(
                        "execution engine returned a handle for another execution",
                        &execution_id,
                    ),
                );
            }

            // Verify artifact binding.
            if handle.artifact() != &artifact {
                return Err(
                    compatibility_error(
                        "execution engine returned a handle for another artifact",
                        &execution_id,
                    ),
                );
            }

            // Verify plan binding.
            if handle.plan() != &plan {
                return Err(
                    compatibility_error(
                        "execution engine returned a handle for another execution plan",
                        &execution_id,
                    ),
                );
            }

            Ok(handle)
        })
    }


    // ------------------------------------------------------------------------
    // OBSERVATION
    // ------------------------------------------------------------------------

    /// Obtains one actual execution observation.
    ///
    /// The returned observation must identify the exact execution
    /// represented by the supplied handle.
    ///
    /// This operation does not infer execution status from submission.
    ///
    /// # Errors
    ///
    /// Returns an error for cancellation, provider failure,
    /// unavailable execution authorities or identity mismatch.
    pub fn observe<'b>(
        &'b self,
        handle: &'b ExecutionHandle,
        context: &'b OperationContext,
    ) -> ExecutionFuture<'b, ExecutionObservation> {

        Box::pin(async move {

            check_cancelled(
                context,
                handle.execution_id(),
                "observe",
            )?;

            let observation = self.engine
                .observe(handle, context)
                .await
                .map_err(|error| {
                    error
                        .with_component(COMPONENT)
                        .with_operation("observe")
                        .with_execution(
                            handle.execution_id().to_string(),
                        )
                })?;

            validate_observation(
                handle,
                &observation,
            )?;

            Ok(observation)
        })
    }


    // ------------------------------------------------------------------------
    // CANCELLATION
    // ------------------------------------------------------------------------

    /// Requests cancellation of an accepted execution.
    ///
    /// The returned disposition is the actual disposition reported
    /// by the execution authority.
    ///
    /// IMPORTANT:
    ///
    /// Requested != Confirmed.
    ///
    /// A cancellation request may be accepted while the underlying
    /// computation continues running.
    ///
    /// This method never converts Requested, Unknown or Rejected
    /// into confirmed cancellation.
    ///
    /// The caller should continue observing the execution until
    /// a terminal state is established.
    ///
    /// The supplied context must remain usable for the cancellation
    /// operation. A separate operation context may be appropriate
    /// when the original execution context has already been cancelled.
    pub fn cancel<'b>(
        &'b self,
        handle: &'b ExecutionHandle,
        context: &'b OperationContext,
    ) -> ExecutionFuture<'b, CancellationDisposition> {

        Box::pin(async move {

            let disposition = self.engine
                .cancel(handle, context)
                .await
                .map_err(|error| {
                    error
                        .with_component(COMPONENT)
                        .with_operation("cancel")
                        .with_execution(
                            handle.execution_id().to_string(),
                        )
                })?;

            Ok(disposition)
        })
    }


    // ------------------------------------------------------------------------
    // MONITORING
    // ------------------------------------------------------------------------

    /// Monitors an accepted execution until its authority reports
    /// a terminal lifecycle state.
    ///
    /// The monitoring sequence is:
    ///
    /// 1. Check cooperative cancellation.
    /// 2. Obtain an actual execution observation.
    /// 3. Validate execution identity.
    /// 4. Determine whether the observed state is terminal.
    /// 5. If nonterminal, wait using the supplied ExecutionWaiter.
    /// 6. Repeat.
    /// 7. Construct the canonical ExecutionResult from the terminal
    ///    observation.
    ///
    /// No polling interval or maximum polling count is hard-coded.
    ///
    /// The waiting strategy belongs to the embedding environment.
    ///
    /// Cancellation is deliberately separate from monitoring.
    /// Call cancel() explicitly and continue monitoring if confirmation
    /// of termination is required.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The operation context is cancelled.
    /// - Observation fails.
    /// - Waiting fails.
    /// - The execution authority returns inconsistent observations.
    /// - The execution identity changes.
    ///
    /// A nonterminal observation is never reported as successful.
    pub fn monitor<'b>(
        &'b self,
        handle: &'b ExecutionHandle,
        context: &'b OperationContext,
    ) -> ExecutionFuture<'b, ExecutionOutcome> {

        Box::pin(async move {

            loop {

                check_cancelled(
                    context,
                    handle.execution_id(),
                    "monitor",
                )?;

                let observation = self
                    .observe(handle, context)
                    .await?;

                let status =
                    observation.state().status();

                if status.is_terminal() {

                    let result = result_from_observation(
                        &observation,
                    )?;

                    return ExecutionOutcome::new(
                        handle.clone(),
                        result,
                    );
                }

                // Waiting is supplied by the embedding environment.
                //
                // This permits timer-driven, event-driven and
                // provider-notification-driven implementations.
                self.waiter
                    .wait(context)
                    .await
                    .map_err(|error| {
                        error
                            .with_component(COMPONENT)
                            .with_operation("monitor.wait")
                            .with_execution(
                                handle.execution_id().to_string(),
                            )
                    })?;
            }
        })
    }


    // ------------------------------------------------------------------------
    // SINGLE-OBSERVATION RESULT COLLECTION
    // ------------------------------------------------------------------------

    /// Converts one terminal observation into a canonical result.
    ///
    /// This method is useful when the caller already manages monitoring
    /// or receives provider notifications.
    ///
    /// Nonterminal observations are rejected.
    pub fn collect(
        &self,
        observation: &ExecutionObservation,
    ) -> ZurResult<ExecutionResult> {

        if !observation.state().is_terminal() {
            return Err(
                ZurError::invalid_input(
                    "cannot collect a final execution result from a nonterminal observation",
                )
                .with_component(COMPONENT)
                .with_operation("collect")
                .with_execution(
                    observation.execution_id().to_string(),
                ),
            );
        }

        result_from_observation(observation)
    }
}


// ============================================================================
// RESULT NORMALIZATION
// ============================================================================

/// Converts a canonical terminal execution observation into a
/// structured execution result.
///
/// This function does not invent outputs or recovery evidence.
///
/// Output references are copied only when supplied by the execution
/// authority.
///
/// Failure information remains explicit.
///
/// Unknown future terminal states are reported as incomplete rather
/// than being interpreted as successful completion.
fn result_from_observation(
    observation: &ExecutionObservation,
) -> ZurResult<ExecutionResult> {

    let execution_id =
        observation.execution_id().clone();

    let status =
        observation.state().status();

    let mut outputs = Vec::new();

    if let Some(reference) = observation.result() {
        outputs.push(reference.clone());
    }

    let recovery =
        RecoveryReport::not_attempted();

    match status {

        ExecutionStatus::Completed => {

            Ok(
                ExecutionResult::completed(
                    execution_id,
                    outputs,
                    recovery,
                ),
            )
        }

        ExecutionStatus::PartiallyCompleted => {

            Ok(
                ExecutionResult::partially_completed(
                    execution_id,
                    outputs,
                    ZurError::execution(
                        "execution authority reported partial completion",
                    )
                    .with_component(COMPONENT)
                    .with_operation("collect"),
                    recovery,
                ),
            )
        }

        ExecutionStatus::Failed => {

            let error = match observation.failure() {

                Some(reference) => {
                    ZurError::execution(
                        format!(
                            "execution authority reported failure record {}",
                            reference,
                        ),
                    )
                }

                None => {
                    ZurError::execution(
                        "execution authority reported execution failure",
                    )
                }
            }
            .with_component(COMPONENT)
            .with_operation("collect")
            .with_execution(
                execution_id.to_string(),
            );

            Ok(
                ExecutionResult::failed(
                    execution_id,
                    outputs,
                    error,
                    recovery,
                ),
            )
        }

        ExecutionStatus::Cancelled => {

            ExecutionResult::cancelled(
                execution_id,
                outputs,
                CancellationDisposition::Confirmed,
                recovery,
            )
        }

        ExecutionStatus::TimedOut => {

            Ok(
                ExecutionResult::timed_out(
                    execution_id,
                    outputs,
                    ZurError::timeout(
                        "execution authority reported an execution timeout",
                    )
                    .with_component(COMPONENT)
                    .with_operation("collect"),
                    recovery,
                ),
            )
        }

        _ => {

            Ok(
                ExecutionResult::incomplete(
                    execution_id,
                    outputs,
                    ZurError::execution(
                        format!(
                            "execution reached an unrecognized terminal state: {status:?}",
                        ),
                    )
                    .with_component(COMPONENT)
                    .with_operation("collect"),
                    recovery,
                ),
            )
        }
    }
}


// ============================================================================
// OBSERVATION VALIDATION
// ============================================================================

/// Validates that an observation belongs to the exact accepted execution.
fn validate_observation(
    handle: &ExecutionHandle,
    observation: &ExecutionObservation,
) -> ZurResult<()> {

    if observation.execution_id() != handle.execution_id() {
        return Err(
            compatibility_error(
                "execution authority returned an observation for another execution",
                handle.execution_id(),
            ),
        );
    }

    Ok(())
}


// ============================================================================
// CANCELLATION VALIDATION
// ============================================================================

/// Checks cooperative cancellation at an executor boundary.
fn check_cancelled(
    context: &OperationContext,
    execution_id: &ExecutionId,
    operation: &str,
) -> ZurResult<()> {

    if !context.is_cancelled() {
        return Ok(());
    }

    let reason = context
        .cancellation_reason()
        .unwrap_or_else(|| {
            "execution operation was cancelled".to_owned()
        });

    Err(
        ZurError::cancelled(reason)
            .with_component(COMPONENT)
            .with_operation(operation)
            .with_execution(
                execution_id.to_string(),
            ),
    )
}


// ============================================================================
// ERROR HELPERS
// ============================================================================

fn compatibility_error(
    message: impl Into<String>,
    execution_id: &ExecutionId,
) -> ZurError {

    ZurError::compatibility(message)
        .with_component(COMPONENT)
        .with_operation("validate")
        .with_execution(
            execution_id.to_string(),
        )
}


// ============================================================================
// DEBUG
// ============================================================================

impl fmt::Debug for ExecutionExecutor<'_> {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        formatter
            .debug_struct("ExecutionExecutor")
            .field(
                "engine",
                &"explicitly injected",
            )
            .field(
                "waiter",
                &"explicitly injected",
            )
            .finish()
    }
}


// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    use crate::zur::contracts::execution::{
        ExecutionReference,
        ExecutionState,
    };

    use crate::zur::ids::{
        ExecutionId,
        IdentityNamespace,
    };


    fn execution_id() -> ExecutionId {

        ExecutionId::new(
            IdentityNamespace::new(
                "zamani.runtime.execution",
            )
            .expect("valid namespace"),
            vec![1],
        )
        .expect("valid execution identity")
    }


    fn reference(
        value: &str,
    ) -> ExecutionReference {

        ExecutionReference::new(
            "zamani.execution.results",
            value,
        )
        .expect("valid execution reference")
    }


    fn observation(
        status: ExecutionStatus,
    ) -> ExecutionObservation {

        let mut state =
            ExecutionState::new();

        state.transition(
            ExecutionStatus::Validating,
        )
        .expect("valid transition");

        state.transition(
            ExecutionStatus::Prepared,
        )
        .expect("valid transition");

        state.transition(
            ExecutionStatus::Submitted,
        )
        .expect("valid transition");

        if matches!(
            status,
            ExecutionStatus::Running
                | ExecutionStatus::Streaming
                | ExecutionStatus::Cancelling
                | ExecutionStatus::Completed
                | ExecutionStatus::PartiallyCompleted
                | ExecutionStatus::Cancelled
                | ExecutionStatus::TimedOut
                | ExecutionStatus::Failed
        ) {
            state.transition(
                ExecutionStatus::Running,
            )
            .expect("valid transition");
        }

        if status != ExecutionStatus::Running
            && status != ExecutionStatus::Streaming
            && status != ExecutionStatus::Cancelling
        {
            state.transition(status)
                .expect("valid terminal transition");
        }

        let result = if matches!(
            status,
            ExecutionStatus::Completed
                | ExecutionStatus::PartiallyCompleted
        ) {
            Some(reference("output-001"))
        } else {
            None
        };

        let failure = if status == ExecutionStatus::Failed {
            Some(reference("failure-001"))
        } else {
            None
        };

        ExecutionObservation::new(
            execution_id(),
            state,
            result,
            failure,
        )
        .expect("valid observation")
    }


    #[test]
    fn collects_completed_execution() {

        let result = result_from_observation(
            &observation(
                ExecutionStatus::Completed,
            ),
        )
        .expect("completed result");

        assert!(result.is_successful());

        assert_eq!(
            result.outputs().len(),
            1,
        );
    }


    #[test]
    fn collects_partial_execution_without_claiming_full_success() {

        let result = result_from_observation(
            &observation(
                ExecutionStatus::PartiallyCompleted,
            ),
        )
        .expect("partial result");

        assert!(result.is_partial());

        assert!(!result.is_successful());

        assert!(result.failure().is_some());
    }


    #[test]
    fn collects_failed_execution() {

        let result = result_from_observation(
            &observation(
                ExecutionStatus::Failed,
            ),
        )
        .expect("failed result");

        assert!(result.is_failed());

        assert!(!result.is_successful());

        assert!(result.failure().is_some());
    }


    #[test]
    fn collects_confirmed_cancellation() {

        let result = result_from_observation(
            &observation(
                ExecutionStatus::Cancelled,
            ),
        )
        .expect("cancelled result");

        assert!(result.is_cancelled());

        assert!(!result.is_successful());

        assert_eq!(
            result.cancellation_disposition(),
            Some(CancellationDisposition::Confirmed),
        );
    }


    #[test]
    fn collects_execution_timeout() {

        let result = result_from_observation(
            &observation(
                ExecutionStatus::TimedOut,
            ),
        )
        .expect("timeout result");

        assert!(result.is_timed_out());

        assert!(!result.is_successful());
    }


    #[test]
    fn rejects_nonterminal_result_collection() {

        let executor = ExecutionExecutor {
            engine: &UnsupportedEngine,
            waiter: &UnsupportedWaiter,
        };

        assert!(
            executor
                .collect(
                    &observation(
                        ExecutionStatus::Running,
                    ),
                )
                .is_err()
        );
    }


    #[test]
    fn rejects_observation_for_another_execution() {

        let handle = ExecutionHandle::new(
            execution_id(),
            crate::zur::contracts::execution::ArtifactReference::new(
                "zamani.compiler.artifacts",
                "artifact-001",
            )
            .expect("valid artifact"),
            crate::zur::contracts::execution::ExecutionPlanReference::new(
                "zamani.compiler.planning",
                "plan-001",
            )
            .expect("valid plan"),
            None,
        )
        .expect("valid handle");

        let mut other = observation(
            ExecutionStatus::Completed,
        );

        let different_id = ExecutionId::new(
            IdentityNamespace::new(
                "zamani.runtime.execution",
            )
            .expect("valid namespace"),
            vec![2],
        )
        .expect("valid identity");

        let mut state = ExecutionState::new();

        state.transition(ExecutionStatus::Validating)
            .expect("valid transition");

        state.transition(ExecutionStatus::Prepared)
            .expect("valid transition");

        state.transition(ExecutionStatus::Submitted)
            .expect("valid transition");

        state.transition(ExecutionStatus::Running)
            .expect("valid transition");

        state.transition(ExecutionStatus::Completed)
            .expect("valid transition");

        other = ExecutionObservation::new(
            different_id,
            state,
            Some(reference("output-001")),
            None,
        )
        .expect("valid observation");

        assert!(
            validate_observation(
                &handle,
                &other,
            )
            .is_err()
        );
    }


    // These test-only implementations establish that the executor
    // accepts explicit dependencies without global initialization.

    struct UnsupportedEngine;

    impl ExecutionEngine for UnsupportedEngine {

        fn submit<'a>(
            &'a self,
            _request: ExecutionRequest<'a>,
        ) -> ExecutionFuture<'a, ExecutionHandle> {

            Box::pin(async {
                Err(
                    ZurError::unsupported(
                        "test execution engine",
                    ),
                )
            })
        }

        fn observe<'a>(
            &'a self,
            _handle: &'a ExecutionHandle,
            _context: &'a OperationContext,
        ) -> ExecutionFuture<'a, ExecutionObservation> {

            Box::pin(async {
                Err(
                    ZurError::unsupported(
                        "test execution engine",
                    ),
                )
            })
        }

        fn cancel<'a>(
            &'a self,
            _handle: &'a ExecutionHandle,
            _context: &'a OperationContext,
        ) -> ExecutionFuture<'a, CancellationDisposition> {

            Box::pin(async {
                Err(
                    ZurError::unsupported(
                        "test execution engine",
                    ),
                )
            })
        }
    }


    struct UnsupportedWaiter;

    impl ExecutionWaiter for UnsupportedWaiter {

        fn wait<'a>(
            &'a self,
            _context: &'a OperationContext,
        ) -> ExecutionFuture<'a, ()> {

            Box::pin(async {
                Err(
                    ZurError::unsupported(
                        "test execution waiter",
                    ),
                )
            })
        }
    }


    #[test]
    fn executor_accepts_explicit_engine_and_waiter() {

        let executor = ExecutionExecutor::new(
            &UnsupportedEngine,
            &UnsupportedWaiter,
        );

        assert!(
            format!("{executor:?}")
                .contains("ExecutionExecutor")
        );
    }


    #[test]
    fn future_domain_references_are_supported() {

        let future = ExecutionReference::new(
            "future.photonics.execution",
            "execution-record-001",
        )
        .expect("future domain reference");

        assert_eq!(
            future.authority(),
            "future.photonics.execution",
        );
    }


    #[test]
    fn execution_outcome_rejects_mismatched_identity() {

        let handle = ExecutionHandle::new(
            execution_id(),
            crate::zur::contracts::execution::ArtifactReference::new(
                "zamani.compiler.artifacts",
                "artifact-001",
            )
            .expect("valid artifact"),
            crate::zur::contracts::execution::ExecutionPlanReference::new(
                "zamani.compiler.planning",
                "plan-001",
            )
            .expect("valid plan"),
            None,
        )
        .expect("valid handle");

        let different_id = ExecutionId::new(
            IdentityNamespace::new(
                "zamani.runtime.execution",
            )
            .expect("valid namespace"),
            vec![3],
        )
        .expect("valid identity");

        let result = ExecutionResult::completed(
            different_id,
            Vec::new(),
            RecoveryReport::not_attempted(),
        );

        assert!(
            ExecutionOutcome::new(
                handle,
                result,
            )
            .is_err()
        );
    }
}
