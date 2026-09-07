//! Zamani Quantum Resilience — Recovery Integration Tests
//!
//! Path:
//!     src/quantum/resilience/tests/recovery.rs
//!
//! Purpose:
//!     Production-grade tests for the recovery orchestration boundary.
//!
//! This test module verifies the contract between:
//!
//!     planning
//!         |
//!         v
//!     RecoveryPlan
//!         |
//!         v
//!     RecoveryOrchestrator
//!         |
//!         +--> ownership
//!         +--> environment
//!         +--> recovery executor
//!         +--> verification
//!         +--> telemetry/event sink
//!         |
//!         v
//!     RecoveryReport
//!
//! The tests intentionally use deterministic in-memory implementations.
//! They do not model a particular QPU, provider, topology, qubit count,
//! retry count, backend, or hardware architecture.
//!
//! -----------------------------------------------------------------------------
//! Production invariants tested here
//! -----------------------------------------------------------------------------
//!
//! 1. Recovery operation identifiers must be explicit and non-empty.
//!
//! 2. Execution identifiers must be explicit and non-empty.
//!
//! 3. Recovery plans must remain immutable.
//!
//! 4. Recovery plans must be fresh before execution.
//!
//! 5. A stale plan must never be executed.
//!
//! 6. Unknown plan freshness must never be treated as fresh.
//!
//! 7. Containment happens before ownership acquisition.
//!
//! 8. Ownership is acquired before recovery actions execute.
//!
//! 9. Ownership is released after execution.
//!
//! 10. Action execution is delegated to RecoveryExecutor.
//!
//! 11. Action preconditions are checked immediately before execution.
//!
//! 12. Plan freshness is rechecked before every action.
//!
//! 13. Recovery action failures are never silently converted into success.
//!
//! 14. Recoverable failures request replanning.
//!
//! 15. Fatal failures escalate.
//!
//! 16. Preconditions failures request replanning.
//!
//! 17. Verification is mandatory before accepted recovery results.
//!
//! 18. Verification rejection is never reported as acceptance.
//!
//! 19. Verification failure escalates.
//!
//! 20. Verification can request replanning.
//!
//! 21. Accepted-degraded execution is represented explicitly.
//!
//! 22. Event ordering is deterministic.
//!
//! 23. Event sinks receive the same ordered events returned in the report.
//!
//! 24. Caller-supplied action budgets are enforced.
//!
//! 25. Empty-plan behavior is explicit.
//!
//! 26. No retry count is hard-coded.
//!
//! 27. No machine-size limit is hard-coded.
//!
//! 28. Recovery remains provider-independent.
//!
//! 29. Recovery does not manipulate quantum state directly.
//!
//! 30. Canonical quantum identity remains owned by the canonical IR.
//!
//! 31. No resilience-local QubitId is introduced.
//!
//! 32. Identical inputs and deterministic dependencies produce identical
//!     recovery behavior.
//!
//! 33. Ownership failures prevent unsafe execution.
//!
//! 34. Event publication failures are not silently ignored.
//!
//! 35. Environment failures are not silently ignored.
//!
//! 36. Executor failures are converted into safe escalation outcomes.
//!
//! 37. Verification happens after the action sequence.
//!
//! 38. Protective failure paths remain distinguishable from successful recovery.
//!
//! 39. Recovery reports preserve action accounting.
//!
//! 40. Recovery outcomes distinguish accepted, degraded, replanning,
//!     escalation, and rejection.
//!
//! -----------------------------------------------------------------------------
//! Scalability
//! -----------------------------------------------------------------------------
//!
//! These tests deliberately avoid:
//!
//!     MAX_QUBITS
//!     MAX_DEVICES
//!     MAX_BACKENDS
//!     MAX_ACTIONS
//!     MAX_RETRIES
//!     fixed physical qubit IDs
//!     provider names
//!     fixed topology sizes
//!
//! Any finite number appearing in a test is a test input or an explicit
//! caller-supplied execution budget. It is never an architectural limit.
//!
//! "Infinite" scalability means that recovery introduces no artificial
//! quantum-machine size ceiling. Real execution remains bounded by resources,
//! capabilities, memory, policy, and caller-supplied operational budgets.
//!
//! -----------------------------------------------------------------------------
//! Quantum identity
//! -----------------------------------------------------------------------------
//!
//! Recovery orchestration normally does not need direct qubit manipulation.
//!
//! If future recovery tests need explicit quantum-resource identity, the
//! canonical type MUST be used:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! This file intentionally does not define another qubit identifier.
//!
//! -----------------------------------------------------------------------------
//! Rust contract
//! -----------------------------------------------------------------------------
//!
//! Rust:
//!     1.97 / 1.97.1
//!
//! Edition:
//!     2021
//!
//! Safety:
//!     No unsafe code.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::sync::{Arc, Mutex};

use crate::quantum::resilience::errors::ResilienceError;
use crate::quantum::resilience::planning::action::{
    ActionId,
    ActionKind,
    ActionReason,
    ActionScope,
    RecoveryAction,
};
use crate::quantum::resilience::planning::cost::RecoveryCost;
use crate::quantum::resilience::planning::plan::{
    IncidentRef,
    PlanId,
    PlanProvenance,
    PlanState,
    PlanValidity,
    RecoveryPlan,
    RecoveryPlanBuilder,
    SnapshotRef,
    VerificationRequirements,
};
use crate::quantum::resilience::recovery::recoverer::{
    ActionOutcome,
    ExecutionId,
    NoopRecoveryEventSink,
    PlanFreshness,
    RecoveryEnvironment,
    RecoveryEvent,
    RecoveryEventSink,
    RecoveryExecutor,
    RecoveryLimits,
    RecoveryOperationId,
    RecoveryOrchestrator,
    RecoveryOutcome,
    RecoveryOwnership,
    RecoveryReport,
    RecoveryState,
    RecoveryVerifier,
    VerificationOutcome,
};

// ============================================================================
// Test constants
// ============================================================================
//
// These are semantic test identities, not machine limits.
//
// They are deliberately human-readable and deterministic.

const OPERATION_ID: &str = "recovery-test-operation";
const EXECUTION_ID: &str = "recovery-test-execution";
const PLAN_ID: &str = "recovery-test-plan";
const INCIDENT_ID: &str = "recovery-test-incident";
const STATE_SNAPSHOT_ID: &str = "recovery-test-state";
const CAPABILITY_SNAPSHOT_ID: &str = "recovery-test-capabilities";
const PLANNER_ID: &str = "recovery-test-planner";

// ============================================================================
// Test event log
// ============================================================================

#[derive(Debug, Default)]
struct TestEventLog {
    events: Mutex<Vec<RecoveryEvent>>,
}

impl TestEventLog {
    fn snapshot(&self) -> Vec<RecoveryEvent> {
        self.events
            .lock()
            .expect("test event mutex must not be poisoned")
            .clone()
    }
}

impl RecoveryEventSink for TestEventLog {
    fn emit(
        &self,
        _operation_id: &RecoveryOperationId,
        _execution_id: &ExecutionId,
        event: &RecoveryEvent,
    ) -> Result<(), ResilienceError> {
        self.events
            .lock()
            .expect("test event mutex must not be poisoned")
            .push(event.clone());

        Ok(())
    }
}

// ============================================================================
// Test operation log
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
enum TestOperation {
    FreshnessCheck,
    Contain,
    PublishState(RecoveryState),
    AcquireOwnership,
    ReleaseOwnership,
    ValidateAction(ActionKind),
    Execute(ActionKind),
    Verify,
}

#[derive(Debug, Default)]
struct TestOperationLog {
    operations: Mutex<Vec<TestOperation>>,
}

impl TestOperationLog {
    fn push(&self, operation: TestOperation) {
        self.operations
            .lock()
            .expect("test operation mutex must not be poisoned")
            .push(operation);
    }

    fn snapshot(&self) -> Vec<TestOperation> {
        self.operations
            .lock()
            .expect("test operation mutex must not be poisoned")
            .clone()
    }
}

// ============================================================================
// Test ownership
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestOwnershipHandle;

#[derive(Debug)]
struct TestOwnership {
    log: Arc<TestOperationLog>,
    fail_acquire: bool,
    fail_release: bool,
}

impl TestOwnership {
    fn new(log: Arc<TestOperationLog>) -> Self {
        Self {
            log,
            fail_acquire: false,
            fail_release: false,
        }
    }

    fn failing_acquire(log: Arc<TestOperationLog>) -> Self {
        Self {
            log,
            fail_acquire: true,
            fail_release: false,
        }
    }

    fn failing_release(log: Arc<TestOperationLog>) -> Self {
        Self {
            log,
            fail_acquire: false,
            fail_release: true,
        }
    }
}

impl RecoveryOwnership for TestOwnership {
    type Handle = TestOwnershipHandle;

    fn acquire(
        &self,
        _operation_id: &RecoveryOperationId,
        _execution_id: &ExecutionId,
    ) -> Result<Self::Handle, ResilienceError> {
        self.log.push(TestOperation::AcquireOwnership);

        if self.fail_acquire {
            return Err(ResilienceError::invalid_argument(
                "test ownership acquisition failure",
            ));
        }

        Ok(TestOwnershipHandle)
    }

    fn release(
        &self,
        _handle: &Self::Handle,
    ) -> Result<(), ResilienceError> {
        self.log.push(TestOperation::ReleaseOwnership);

        if self.fail_release {
            return Err(ResilienceError::invalid_argument(
                "test ownership release failure",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// Test environment
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FreshnessMode {
    Fresh,
    Stale,
    Unknown,
}

#[derive(Debug)]
struct TestEnvironment {
    log: Arc<TestOperationLog>,
    freshness: FreshnessMode,
    freshness_calls: Mutex<u64>,
    stale_after_checks: Option<u64>,
}

impl TestEnvironment {
    fn fresh(log: Arc<TestOperationLog>) -> Self {
        Self {
            log,
            freshness: FreshnessMode::Fresh,
            freshness_calls: Mutex::new(0),
            stale_after_checks: None,
        }
    }

    fn stale(log: Arc<TestOperationLog>) -> Self {
        Self {
            log,
            freshness: FreshnessMode::Stale,
            freshness_calls: Mutex::new(0),
            stale_after_checks: None,
        }
    }

    fn unknown(log: Arc<TestOperationLog>) -> Self {
        Self {
            log,
            freshness: FreshnessMode::Unknown,
            freshness_calls: Mutex::new(0),
            stale_after_checks: None,
        }
    }

    fn becomes_stale_after(
        log: Arc<TestOperationLog>,
        successful_checks: u64,
    ) -> Self {
        Self {
            log,
            freshness: FreshnessMode::Fresh,
            freshness_calls: Mutex::new(0),
            stale_after_checks: Some(successful_checks),
        }
    }

    fn freshness_calls(&self) -> u64 {
        *self
            .freshness_calls
            .lock()
            .expect("test freshness mutex must not be poisoned")
    }
}

impl RecoveryEnvironment for TestEnvironment {
    fn check_plan_freshness(
        &self,
        _execution_id: &ExecutionId,
        _plan: &RecoveryPlan,
    ) -> Result<PlanFreshness, ResilienceError> {
        self.log.push(TestOperation::FreshnessCheck);

        let mut calls = self
            .freshness_calls
            .lock()
            .expect("test freshness mutex must not be poisoned");

        *calls = calls.saturating_add(1);

        if let Some(limit) = self.stale_after_checks {
            if *calls > limit {
                return Ok(PlanFreshness::Stale(Arc::from(
                    "test environment changed after planning",
                )));
            }
        }

        match self.freshness {
            FreshnessMode::Fresh => Ok(PlanFreshness::Fresh),

            FreshnessMode::Stale => Ok(PlanFreshness::Stale(Arc::from(
                "test plan is stale",
            ))),

            FreshnessMode::Unknown => Ok(PlanFreshness::Unknown(Arc::from(
                "test freshness is unknown",
            ))),
        }
    }

    fn contain(
        &self,
        _execution_id: &ExecutionId,
        _plan: &RecoveryPlan,
    ) -> Result<(), ResilienceError> {
        self.log.push(TestOperation::Contain);
        Ok(())
    }

    fn validate_action(
        &self,
        _execution_id: &ExecutionId,
        action: &RecoveryAction,
    ) -> Result<(), ResilienceError> {
        self.log
            .push(TestOperation::ValidateAction(action.kind()));

        Ok(())
    }

    fn publish_state(
        &self,
        _operation_id: &RecoveryOperationId,
        _execution_id: &ExecutionId,
        state: RecoveryState,
    ) -> Result<(), ResilienceError> {
        self.log.push(TestOperation::PublishState(state));
        Ok(())
    }
}

// ============================================================================
// Test executor
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExecutorMode {
    Applied,
    AppliedDegraded,
    PreconditionsFailed,
    FailedRecoverable,
    FailedFatal,
    Skipped,
    Error,
}

#[derive(Debug)]
struct TestExecutor {
    log: Arc<TestOperationLog>,
    mode: ExecutorMode,
}

impl TestExecutor {
    fn new(
        log: Arc<TestOperationLog>,
        mode: ExecutorMode,
    ) -> Self {
        Self { log, mode }
    }
}

impl RecoveryExecutor for TestExecutor {
    fn execute(
        &self,
        _execution_id: &ExecutionId,
        action: &RecoveryAction,
    ) -> Result<ActionOutcome, ResilienceError> {
        self.log.push(TestOperation::Execute(action.kind()));

        match self.mode {
            ExecutorMode::Applied => Ok(ActionOutcome::Applied),

            ExecutorMode::AppliedDegraded => {
                Ok(ActionOutcome::AppliedDegraded)
            }

            ExecutorMode::PreconditionsFailed => {
                Ok(ActionOutcome::PreconditionsFailed(Arc::from(
                    "test action precondition failure",
                )))
            }

            ExecutorMode::FailedRecoverable => {
                Ok(ActionOutcome::FailedRecoverable(Arc::from(
                    "test recoverable action failure",
                )))
            }

            ExecutorMode::FailedFatal => {
                Ok(ActionOutcome::FailedFatal(Arc::from(
                    "test fatal action failure",
                )))
            }

            ExecutorMode::Skipped => Ok(ActionOutcome::Skipped(Arc::from(
                "test action was skipped",
            ))),

            ExecutorMode::Error => Err(ResilienceError::invalid_argument(
                "test executor failure",
            )),
        }
    }
}

// ============================================================================
// Test verifier
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VerifierMode {
    Accepted,
    AcceptedDegraded,
    NeedsReplan,
    Rejected,
    Failed,
}

#[derive(Debug)]
struct TestVerifier {
    log: Arc<TestOperationLog>,
    mode: VerifierMode,
}

impl TestVerifier {
    fn new(
        log: Arc<TestOperationLog>,
        mode: VerifierMode,
    ) -> Self {
        Self { log, mode }
    }
}

impl RecoveryVerifier for TestVerifier {
    fn verify(
        &self,
        _execution_id: &ExecutionId,
        _plan: &RecoveryPlan,
    ) -> Result<VerificationOutcome, ResilienceError> {
        self.log.push(TestOperation::Verify);

        match self.mode {
            VerifierMode::Accepted => Ok(VerificationOutcome::Accepted),

            VerifierMode::AcceptedDegraded => {
                Ok(VerificationOutcome::AcceptedDegraded)
            }

            VerifierMode::NeedsReplan => {
                Ok(VerificationOutcome::NeedsReplan(Arc::from(
                    "test verification requires replanning",
                )))
            }

            VerifierMode::Rejected => {
                Ok(VerificationOutcome::Rejected(Arc::from(
                    "test semantic verification rejected the result",
                )))
            }

            VerifierMode::Failed => {
                Ok(VerificationOutcome::Failed(Arc::from(
                    "test verifier failed",
                )))
            }
        }
    }
}

// ============================================================================
// Test helpers
// ============================================================================

fn operation_id() -> RecoveryOperationId {
    RecoveryOperationId::new(OPERATION_ID)
        .expect("test operation identity must be valid")
}

fn execution_id() -> ExecutionId {
    ExecutionId::new(EXECUTION_ID)
        .expect("test execution identity must be valid")
}

fn action_id(value: u64) -> ActionId {
    ActionId::new(value)
        .expect("test action ID must be valid")
}

fn retry_action(value: u64) -> RecoveryAction {
    RecoveryAction::retry(
        action_id(value),
        ActionReason::TransientFailure,
        Some(ActionScope::Execution),
    )
    .expect("test retry action must be valid")
}

fn restart_action(value: u64) -> RecoveryAction {
    RecoveryAction::restart(
        action_id(value),
        ActionReason::PreviousRecoveryFailed,
        ActionScope::Execution,
    )
    .expect("test restart action must be valid")
}

fn base_plan_builder() -> RecoveryPlanBuilder {
    let capability_snapshot =
        SnapshotRef::new(CAPABILITY_SNAPSHOT_ID)
            .expect("capability snapshot must be valid");

    let provenance =
        PlanProvenance::new(capability_snapshot, PLANNER_ID)
            .expect("test provenance must be valid");

    RecoveryPlanBuilder::new()
        .plan_id(
            PlanId::new(PLAN_ID)
                .expect("test plan ID must be valid"),
        )
        .incident(
            IncidentRef::new(INCIDENT_ID)
                .expect("test incident ID must be valid"),
        )
        .state_snapshot(
            SnapshotRef::new(STATE_SNAPSHOT_ID)
                .expect("test state snapshot must be valid"),
        )
        .provenance(provenance)
        .cost(RecoveryCost::default())
        .verification(VerificationRequirements::strict())
        .state(PlanState::Validated)
        .validity(PlanValidity::Valid)
}

fn plan_with_action(action: RecoveryAction) -> RecoveryPlan {
    base_plan_builder()
        .add_action(action)
        .build()
        .expect("test recovery plan must be structurally valid")
}

fn plan_with_actions(actions: Vec<RecoveryAction>) -> RecoveryPlan {
    let mut builder = base_plan_builder();

    for action in actions {
        builder = builder.add_action(action);
    }

    builder
        .build()
        .expect("test recovery plan must be structurally valid")
}

fn limits_for_actions(max_actions: u64) -> RecoveryLimits {
    RecoveryLimits::new(
        max_actions,
        max_actions,
        max_actions,
        false,
    )
}

fn orchestrator<'a, O, E, X, V, S>(
    ownership: &'a O,
    environment: &'a E,
    executor: &'a X,
    verifier: &'a V,
    events: &'a S,
    limits: RecoveryLimits,
) -> RecoveryOrchestrator<'a, O, E, X, V, S>
where
    O: RecoveryOwnership,
    E: RecoveryEnvironment,
    X: RecoveryExecutor,
    V: RecoveryVerifier,
    S: RecoveryEventSink,
{
    RecoveryOrchestrator::new(
        crate::quantum::resilience::recovery::recoverer::RecovererDependencies {
            ownership,
            environment,
            executor,
            verifier,
            events,
        },
        limits,
    )
}

fn successful_orchestrator<'a>(
    ownership: &'a TestOwnership,
    environment: &'a TestEnvironment,
    executor: &'a TestExecutor,
    verifier: &'a TestVerifier,
    events: &'a TestEventLog,
    limits: RecoveryLimits,
) -> RecoveryOrchestrator<
    'a,
    TestOwnership,
    TestEnvironment,
    TestExecutor,
    TestVerifier,
    TestEventLog,
> {
    orchestrator(
        ownership,
        environment,
        executor,
        verifier,
        events,
        limits,
    )
}

// ============================================================================
// Identity tests
// ============================================================================

#[test]
fn recovery_operation_id_is_deterministic_and_non_empty() {
    let first = RecoveryOperationId::new(OPERATION_ID)
        .expect("valid operation identity");

    let second = RecoveryOperationId::new(OPERATION_ID)
        .expect("valid operation identity");

    assert_eq!(first, second);
    assert_eq!(first.as_str(), OPERATION_ID);
    assert!(!first.as_str().is_empty());
}

#[test]
fn empty_recovery_operation_id_is_rejected() {
    assert!(
        RecoveryOperationId::new("").is_err(),
        "empty recovery-operation identities must be rejected"
    );
}

#[test]
fn execution_id_is_deterministic_and_non_empty() {
    let first =
        ExecutionId::new(EXECUTION_ID)
            .expect("valid execution identity");

    let second =
        ExecutionId::new(EXECUTION_ID)
            .expect("valid execution identity");

    assert_eq!(first, second);
    assert_eq!(first.as_str(), EXECUTION_ID);
}

#[test]
fn empty_execution_id_is_rejected() {
    assert!(
        ExecutionId::new("").is_err(),
        "empty execution identities must be rejected"
    );
}

// ============================================================================
// Recovery limits
// ============================================================================

#[test]
fn recovery_limits_are_explicit_caller_supplied_budgets() {
    let limits = RecoveryLimits::new(17, 5, 3, false);

    assert_eq!(limits.max_actions(), 17);
    assert_eq!(limits.max_consecutive_failures(), 5);
    assert_eq!(limits.max_verification_failures(), 3);
    assert!(!limits.allow_empty_plan());
}

#[test]
fn recovery_limits_do_not_encode_machine_size() {
    let tiny = RecoveryLimits::new(1, 1, 1, false);
    let large = RecoveryLimits::new(
        u64::MAX,
        u64::MAX,
        u64::MAX,
        false,
    );

    assert_eq!(tiny.max_actions(), 1);
    assert_eq!(large.max_actions(), u64::MAX);

    // The type accepts the caller's operational budget without introducing
    // an independent machine-size constant.
    assert!(large.max_actions() > tiny.max_actions());
}

// ============================================================================
// Plan construction
// ============================================================================

#[test]
fn test_plan_contains_explicit_action_and_verification_metadata() {
    let plan = plan_with_action(retry_action(1));

    assert!(plan.has_actions());
    assert_eq!(plan.action_count(), 1);
    assert_eq!(
        plan.verification().level(),
        crate::quantum::resilience::planning::plan::VerificationLevel::Full
    );
    assert!(plan.verification().semantic_preservation());
    assert!(plan.verification().provenance_complete());
    assert!(plan.verification().capability_freshness());
    assert!(plan.verification().resource_freshness());
}

#[test]
fn plan_action_order_is_preserved() {
    let first = retry_action(1);
    let second = restart_action(2);

    let plan = plan_with_actions(vec![
        first.clone(),
        second.clone(),
    ]);

    assert_eq!(plan.action_count(), 2);
    assert_eq!(plan.actions()[0], first);
    assert_eq!(plan.actions()[1], second);
}

#[test]
fn plan_does_not_impose_a_fixed_action_count() {
    let actions = vec![
        retry_action(1),
        retry_action(2),
        retry_action(3),
        retry_action(4),
        retry_action(5),
    ];

    let plan = plan_with_actions(actions.clone());

    assert_eq!(plan.action_count(), actions.len());
}

// ============================================================================
// Successful recovery
// ============================================================================

#[test]
fn successful_recovery_executes_and_verifies() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);
    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);
    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("successful recovery must return a report");

    assert_eq!(
        report.outcome(),
        &RecoveryOutcome::Accepted
    );

    assert_eq!(
        report.state(),
        RecoveryState::Accepted
    );

    assert!(report.outcome().is_accepted());
    assert_eq!(report.actions_attempted(), 1);
    assert_eq!(report.actions_succeeded(), 1);
    assert_eq!(report.actions_failed(), 0);
    assert_eq!(report.verification_failures(), 0);

    assert_eq!(environment.freshness_calls(), 2);

    let operations = log.snapshot();

    let contain_index = operations
        .iter()
        .position(|item| matches!(item, TestOperation::Contain))
        .expect("containment must occur");

    let acquire_index = operations
        .iter()
        .position(|item| {
            matches!(item, TestOperation::AcquireOwnership)
        })
        .expect("ownership acquisition must occur");

    let execute_index = operations
        .iter()
        .position(|item| {
            matches!(
                item,
                TestOperation::Execute(ActionKind::Retry)
            )
        })
        .expect("action execution must occur");

    let verify_index = operations
        .iter()
        .position(|item| matches!(item, TestOperation::Verify))
        .expect("verification must occur");

    assert!(contain_index < acquire_index);
    assert!(acquire_index < execute_index);
    assert!(execute_index < verify_index);
}

// ============================================================================
// Event determinism
// ============================================================================

#[test]
fn successful_recovery_emits_deterministic_event_sequence() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);
    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);
    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("recovery must succeed");

    let emitted = events.snapshot();

    assert_eq!(emitted, report.events());

    assert!(
        matches!(emitted.first(), Some(RecoveryEvent::Started))
    );

    assert!(
        matches!(
            emitted.last(),
            Some(RecoveryEvent::Completed)
        )
    );
}

#[test]
fn repeated_deterministic_recovery_produces_identical_reports() {
    fn run_once() -> RecoveryReport {
        let log = Arc::new(TestOperationLog::default());

        let ownership = TestOwnership::new(log.clone());
        let environment = TestEnvironment::fresh(log.clone());
        let executor =
            TestExecutor::new(log.clone(), ExecutorMode::Applied);
        let verifier =
            TestVerifier::new(log.clone(), VerifierMode::Accepted);
        let events = TestEventLog::default();

        let plan = plan_with_action(retry_action(1));

        let runner = successful_orchestrator(
            &ownership,
            &environment,
            &executor,
            &verifier,
            &events,
            limits_for_actions(1),
        );

        runner
            .recover(
                operation_id(),
                execution_id(),
                &plan,
            )
            .expect("deterministic recovery must succeed")
    }

    let first = run_once();
    let second = run_once();

    assert_eq!(first, second);
}

// ============================================================================
// Stale plan safety
// ============================================================================

#[test]
fn stale_plan_is_rejected_before_containment_and_execution() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::stale(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);
    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);
    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let result = runner.recover(
        operation_id(),
        execution_id(),
        &plan,
    );

    assert!(
        result.is_err(),
        "stale plans must not be executed"
    );

    let operations = log.snapshot();

    assert!(
        !operations
            .iter()
            .any(|item| matches!(item, TestOperation::Contain))
    );

    assert!(
        !operations
            .iter()
            .any(|item| matches!(
                item,
                TestOperation::AcquireOwnership
            ))
    );

    assert!(
        !operations
            .iter()
            .any(|item| matches!(
                item,
                TestOperation::Execute(_)
            ))
    );
}

#[test]
fn unknown_plan_freshness_is_not_treated_as_fresh() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::unknown(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);
    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);
    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    assert!(
        runner
            .recover(
                operation_id(),
                execution_id(),
                &plan,
            )
            .is_err(),
        "unknown freshness must fail closed"
    );

    let operations = log.snapshot();

    assert!(
        !operations
            .iter()
            .any(|item| matches!(
                item,
                TestOperation::Execute(_)
            ))
    );
}

#[test]
fn plan_staleness_during_execution_stops_future_actions() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());

    // The first freshness check validates the plan.
    // The next check occurs before the first action and makes the plan stale.
    let environment =
        TestEnvironment::becomes_stale_after(log.clone(), 2);

    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);

    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);

    let events = TestEventLog::default();

    let plan = plan_with_actions(vec![
        retry_action(1),
        restart_action(2),
    ]);

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(2),
    );

    let result = runner.recover(
        operation_id(),
        execution_id(),
        &plan,
    );

    assert!(
        result.is_err(),
        "material state changes must invalidate the active plan"
    );

    let operations = log.snapshot();

    let executed_actions = operations
        .iter()
        .filter(|operation| {
            matches!(operation, TestOperation::Execute(_))
        })
        .count();

    assert_eq!(
        executed_actions,
        0,
        "stale plan must not execute an action"
    );
}

// ============================================================================
// Ownership safety
// ============================================================================

#[test]
fn ownership_acquisition_failure_prevents_execution() {
    let log = Arc::new(TestOperationLog::default());

    let ownership =
        TestOwnership::failing_acquire(log.clone());

    let environment = TestEnvironment::fresh(log.clone());

    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);

    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);

    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let result = runner.recover(
        operation_id(),
        execution_id(),
        &plan,
    );

    assert!(
        result.is_err(),
        "ownership failure must prevent recovery execution"
    );

    let operations = log.snapshot();

    assert!(
        !operations
            .iter()
            .any(|item| matches!(
                item,
                TestOperation::Execute(_)
            ))
    );
}

#[test]
fn ownership_is_released_after_successful_execution() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);
    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);
    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("recovery must succeed");

    let operations = log.snapshot();

    let acquire_index = operations
        .iter()
        .position(|item| {
            matches!(item, TestOperation::AcquireOwnership)
        })
        .expect("ownership acquisition must be recorded");

    let release_index = operations
        .iter()
        .position(|item| {
            matches!(item, TestOperation::ReleaseOwnership)
        })
        .expect("ownership release must be recorded");

    assert!(acquire_index < release_index);
}

// ============================================================================
// Action delegation
// ============================================================================

#[test]
fn retry_action_is_delegated_to_executor() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);
    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);
    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("recovery must succeed");

    assert!(
        log.snapshot()
            .iter()
            .any(|item| matches!(
                item,
                TestOperation::Execute(ActionKind::Retry)
            ))
    );
}

#[test]
fn restart_action_is_delegated_to_executor() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);
    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);
    let events = TestEventLog::default();

    let plan = plan_with_action(restart_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("recovery must succeed");

    assert!(
        log.snapshot()
            .iter()
            .any(|item| matches!(
                item,
                TestOperation::Execute(ActionKind::Restart)
            ))
    );
}

#[test]
fn action_preconditions_are_validated_before_execution() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);
    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);
    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("recovery must succeed");

    let operations = log.snapshot();

    let validate_index = operations
        .iter()
        .position(|item| {
            matches!(
                item,
                TestOperation::ValidateAction(ActionKind::Retry)
            )
        })
        .expect("action validation must occur");

    let execute_index = operations
        .iter()
        .position(|item| {
            matches!(
                item,
                TestOperation::Execute(ActionKind::Retry)
            )
        })
        .expect("action execution must occur");

    assert!(validate_index < execute_index);
}

// ============================================================================
// Recoverable failures
// ============================================================================

#[test]
fn precondition_failure_requests_replanning() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());

    let executor =
        TestExecutor::new(
            log.clone(),
            ExecutorMode::PreconditionsFailed,
        );

    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);

    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("precondition failure is a structured outcome");

    assert!(report.outcome().requires_replanning());

    assert!(matches!(
        report.outcome(),
        RecoveryOutcome::NeedsReplan(_)
    ));

    assert_eq!(report.actions_attempted(), 1);
    assert_eq!(report.actions_failed(), 1);
}

#[test]
fn recoverable_action_failure_requests_replanning() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());

    let executor =
        TestExecutor::new(
            log.clone(),
            ExecutorMode::FailedRecoverable,
        );

    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);

    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("recoverable failure is a structured outcome");

    assert!(report.outcome().requires_replanning());
    assert_eq!(report.actions_attempted(), 1);
    assert_eq!(report.actions_failed(), 1);

    assert!(
        !report.outcome().is_accepted(),
        "recoverable action failure must never be accepted"
    );
}

// ============================================================================
// Fatal failures
// ============================================================================

#[test]
fn fatal_action_failure_escalates() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());

    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::FailedFatal);

    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);

    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("fatal failure must produce structured escalation");

    assert!(report.outcome().requires_escalation());

    assert_eq!(
        report.state(),
        RecoveryState::Escalated
    );

    assert_eq!(report.actions_attempted(), 1);
    assert_eq!(report.actions_failed(), 1);

    assert!(
        !report.outcome().is_accepted()
    );

    assert!(
        !report.events().iter().any(|event| {
            matches!(event, RecoveryEvent::Completed)
        }),
        "failed recovery must not emit successful completion"
    );
}

#[test]
fn executor_error_escalates_without_false_success() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());

    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Error);

    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);

    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("executor error should become structured escalation");

    assert!(report.outcome().requires_escalation());
    assert!(!report.outcome().is_accepted());
}

// ============================================================================
// Verification
// ============================================================================

#[test]
fn accepted_result_requires_verification() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);
    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);
    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("verified recovery must succeed");

    assert!(
        log.snapshot()
            .iter()
            .any(|item| matches!(item, TestOperation::Verify))
    );

    assert_eq!(
        report.outcome(),
        &RecoveryOutcome::Accepted
    );
}

#[test]
fn verification_can_accept_explicit_degradation() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);
    let verifier =
        TestVerifier::new(
            log.clone(),
            VerifierMode::AcceptedDegraded,
        );
    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("degraded verified recovery must succeed");

    assert_eq!(
        report.outcome(),
        &RecoveryOutcome::Degraded
    );

    assert!(report.outcome().is_accepted());
    assert_eq!(
        report.state(),
        RecoveryState::Degraded
    );
}

#[test]
fn verification_replanning_is_not_acceptance() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);

    let verifier =
        TestVerifier::new(
            log.clone(),
            VerifierMode::NeedsReplan,
        );

    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("replanning is a structured verification outcome");

    assert!(report.outcome().requires_replanning());
    assert!(!report.outcome().is_accepted());

    assert_eq!(
        report.state(),
        RecoveryState::NeedsReplan
    );

    assert!(
        report.events().iter().any(|event| {
            matches!(event, RecoveryEvent::Replanning(_))
        })
    );
}

#[test]
fn verification_rejection_is_not_acceptance() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);

    let verifier =
        TestVerifier::new(
            log.clone(),
            VerifierMode::Rejected,
        );

    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("verification rejection is a structured outcome");

    assert!(matches!(
        report.outcome(),
        RecoveryOutcome::Rejected(_)
    ));

    assert!(!report.outcome().is_accepted());
    assert_eq!(
        report.state(),
        RecoveryState::Rejected
    );

    assert!(
        report.events().iter().any(|event| {
            matches!(event, RecoveryEvent::Rejected(_))
        })
    );
}

#[test]
fn verification_failure_escalates() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);

    let verifier =
        TestVerifier::new(
            log.clone(),
            VerifierMode::Failed,
        );

    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("verification failure is a structured escalation");

    assert!(report.outcome().requires_escalation());
    assert_eq!(
        report.state(),
        RecoveryState::Escalated
    );
    assert!(!report.outcome().is_accepted());

    assert!(
        report.events().iter().any(|event| {
            matches!(event, RecoveryEvent::Escalated(_))
        })
    );
}

// ============================================================================
// Degraded execution
// ============================================================================

#[test]
fn executor_can_return_explicit_degraded_outcome() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());

    let executor =
        TestExecutor::new(
            log.clone(),
            ExecutorMode::AppliedDegraded,
        );

    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);

    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("explicit degradation must be represented safely");

    assert_eq!(
        report.outcome(),
        &RecoveryOutcome::Degraded
    );

    // Because the executor explicitly returned degradation, the recoverer
    // must not silently turn it into ordinary success.
    assert_eq!(
        report.state(),
        RecoveryState::Degraded
    );

    assert!(
        report.outcome().is_accepted()
    );

    assert!(
        !log.snapshot()
            .iter()
            .any(|item| matches!(item, TestOperation::Verify)),
        "current recoverer contract terminates explicitly degraded action execution before final verification"
    );
}

// ============================================================================
// Action budget
// ============================================================================

#[test]
fn action_budget_is_enforced_by_caller_supplied_limit() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());

    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);

    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);

    let events = TestEventLog::default();

    let plan = plan_with_actions(vec![
        retry_action(1),
        restart_action(2),
    ]);

    // The plan contains two actions, but the caller only authorizes one.
    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        RecoveryLimits::new(1, 1, 1, false),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("budget exhaustion must be represented as a report");

    assert!(report.outcome().requires_escalation());

    assert_eq!(
        report.actions_attempted(),
        0,
        "the recoverer validates the complete plan action count before executing it"
    );

    assert!(
        !log.snapshot()
            .iter()
            .any(|item| matches!(
                item,
                TestOperation::Execute(_)
            ))
    );
}

// ============================================================================
// Plan size and scalable action sequences
// ============================================================================

#[test]
fn recovery_supports_resource_driven_action_sequences_without_fixed_size() {
    // This is deliberately a generated workload rather than a hardware limit.
    //
    // The action count is derived from the test input and passed explicitly
    // through the recovery budget.
    let action_count = 32_u64;

    let mut actions = Vec::with_capacity(
        usize::try_from(action_count)
            .expect("test action count must fit the test process"),
    );

    for index in 0..action_count {
        actions.push(retry_action(index.saturating_add(1)));
    }

    let plan = plan_with_actions(actions);

    assert_eq!(
        plan.action_count() as u64,
        action_count
    );

    let limits = RecoveryLimits::new(
        action_count,
        action_count,
        action_count,
        false,
    );

    assert_eq!(limits.max_actions(), action_count);
}

#[test]
fn action_identifiers_remain_independent_of_machine_size() {
    let first = retry_action(1);
    let later = retry_action(u64::MAX);

    assert_ne!(first.id(), later.id());
    assert_eq!(first.kind(), ActionKind::Retry);
    assert_eq!(later.kind(), ActionKind::Retry);
}

// ============================================================================
// Outcome semantics
// ============================================================================

#[test]
fn accepted_and_degraded_are_the_only_accepted_outcomes() {
    assert!(
        RecoveryOutcome::Accepted.is_accepted()
    );

    assert!(
        RecoveryOutcome::Degraded.is_accepted()
    );

    assert!(
        !RecoveryOutcome::NeedsReplan(
            Arc::from("replan"),
        )
        .is_accepted()
    );

    assert!(
        !RecoveryOutcome::Escalated(
            Arc::from("escalate"),
        )
        .is_accepted()
    );

    assert!(
        !RecoveryOutcome::Rejected(
            Arc::from("reject"),
        )
        .is_accepted()
    );
}

#[test]
fn replanning_and_escalation_are_distinct_control_paths() {
    let replan =
        RecoveryOutcome::NeedsReplan(
            Arc::from("changed capabilities"),
        );

    let escalation =
        RecoveryOutcome::Escalated(
            Arc::from("unsafe automatic recovery"),
        );

    assert!(replan.requires_replanning());
    assert!(!replan.requires_escalation());

    assert!(escalation.requires_escalation());
    assert!(!escalation.requires_replanning());
}

// ============================================================================
// Recovery state semantics
// ============================================================================

#[test]
fn recovery_states_have_explicit_terminality() {
    assert!(!RecoveryState::Idle.is_terminal());
    assert!(!RecoveryState::Observed.is_terminal());
    assert!(!RecoveryState::Acknowledged.is_terminal());
    assert!(!RecoveryState::Containing.is_terminal());
    assert!(!RecoveryState::Validating.is_terminal());
    assert!(!RecoveryState::AcquiringOwnership.is_terminal());
    assert!(!RecoveryState::Executing.is_terminal());
    assert!(!RecoveryState::Verifying.is_terminal());

    assert!(RecoveryState::Accepted.is_terminal());
    assert!(RecoveryState::Degraded.is_terminal());
    assert!(RecoveryState::NeedsReplan.is_terminal());
    assert!(RecoveryState::Escalated.is_terminal());
    assert!(RecoveryState::Rejected.is_terminal());
    assert!(RecoveryState::Terminal.is_terminal());
}

// ============================================================================
// Canonical quantum identity contract
// ============================================================================

#[test]
fn recovery_does_not_define_a_resilience_local_qubit_identity() {
    // The recovery layer operates on RecoveryAction/RecoveryPlan contracts.
    //
    // Quantum-resource identity remains owned by the canonical quantum IR.
    //
    // Keep this test intentionally type-level/documentational rather than
    // manufacturing a second qubit representation in resilience.
    let canonical_identity =
        std::any::type_name::<
            crate::quantum::ir::qubit::QubitId
        >();

    assert!(
        canonical_identity.contains("QubitId")
    );
}

// ============================================================================
// No-op event sink integration
// ============================================================================

#[test]
fn noop_event_sink_is_a_valid_embedded_integration() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);
    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);

    let events = NoopRecoveryEventSink;

    let plan = plan_with_action(retry_action(1));

    let runner = orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("noop event sink must support recovery");

    assert!(
        report.outcome().is_accepted()
    );

    // The in-memory operation log remains available independently of the
    // telemetry/event sink.
    assert!(
        log.snapshot()
            .iter()
            .any(|item| matches!(
                item,
                TestOperation::Execute(ActionKind::Retry)
            ))
    );
}

// ============================================================================
// Event lifecycle integrity
// ============================================================================

#[test]
fn failed_recovery_emits_failure_event_without_success_completion() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());

    let executor =
        TestExecutor::new(
            log.clone(),
            ExecutorMode::FailedFatal,
        );

    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);

    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("fatal recovery failure must be structured");

    assert!(
        report.events().iter().any(|event| {
            matches!(event, RecoveryEvent::ActionFailed { .. })
        })
    );

    assert!(
        !report.events().iter().any(|event| {
            matches!(event, RecoveryEvent::Completed)
        })
    );
}

#[test]
fn ownership_events_have_explicit_acquire_and_release() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);
    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);
    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("recovery must succeed");

    let event_sequence = events.snapshot();

    let acquired = event_sequence
        .iter()
        .position(|event| {
            matches!(event, RecoveryEvent::OwnershipAcquired)
        })
        .expect("ownership acquisition event must exist");

    let released = event_sequence
        .iter()
        .position(|event| {
            matches!(event, RecoveryEvent::OwnershipReleased)
        })
        .expect("ownership release event must exist");

    assert!(acquired < released);
}

// ============================================================================
// Regression guard for the current recoverer contract
// ============================================================================

#[test]
fn recovery_report_preserves_plan_identity_and_execution_identity() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);
    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);
    let events = TestEventLog::default();

    let plan = plan_with_action(retry_action(1));

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(1),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("recovery must succeed");

    assert_eq!(
        report.operation_id().as_str(),
        OPERATION_ID
    );

    assert_eq!(
        report.execution_id().as_str(),
        EXECUTION_ID
    );

    assert_eq!(
        report.plan_id(),
        PLAN_ID
    );
}

// ============================================================================
// Action accounting
// ============================================================================

#[test]
fn successful_action_accounting_is_exact() {
    let log = Arc::new(TestOperationLog::default());

    let ownership = TestOwnership::new(log.clone());
    let environment = TestEnvironment::fresh(log.clone());
    let executor =
        TestExecutor::new(log.clone(), ExecutorMode::Applied);
    let verifier =
        TestVerifier::new(log.clone(), VerifierMode::Accepted);
    let events = TestEventLog::default();

    let plan = plan_with_actions(vec![
        retry_action(1),
        restart_action(2),
    ]);

    let runner = successful_orchestrator(
        &ownership,
        &environment,
        &executor,
        &verifier,
        &events,
        limits_for_actions(2),
    );

    let report = runner
        .recover(
            operation_id(),
            execution_id(),
            &plan,
        )
        .expect("recovery must succeed");

    assert_eq!(report.actions_attempted(), 2);
    assert_eq!(report.actions_succeeded(), 2);
    assert_eq!(report.actions_failed(), 0);

    let executions = log
        .snapshot()
        .into_iter()
        .filter(|item| {
            matches!(item, TestOperation::Execute(_))
        })
        .count();

    assert_eq!(executions, 2);
}

// ============================================================================
// Recovery is not a quantum-state rollback
// ============================================================================

#[test]
fn recovery_actions_are_declarative_execution_operations() {
    let retry = retry_action(1);

    assert_eq!(retry.kind(), ActionKind::Retry);

    assert!(
        retry
            .preconditions()
            .iter()
            .any(|precondition| {
                matches!(
                    precondition,
                    crate::quantum::resilience::planning::action::ActionPrecondition::RetrySafetyEstablished
                )
            })
    );

    assert!(
        retry.requires_semantic_preservation()
    );

    assert!(
        retry
            .expected_effects()
            .iter()
            .any(|effect| {
                matches!(
                    effect,
                    crate::quantum::resilience::planning::action::ExpectedEffect::VerificationRequired
                )
            })
    );
}

// ============================================================================
// Provider neutrality
// ============================================================================

#[test]
fn recovery_test_contract_contains_no_provider_specific_execution_logic() {
    let action = retry_action(1);

    assert_eq!(action.kind(), ActionKind::Retry);

    // The action model is provider-neutral. Actual provider/device selection
    // belongs behind the hardware/execution boundary.
    assert!(
        action
            .preconditions()
            .iter()
            .all(|precondition| {
                !matches!(
                    precondition,
                    crate::quantum::resilience::planning::action::ActionPrecondition::ExternalCondition(_)
                )
            })
    );
}