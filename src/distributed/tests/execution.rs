#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Distributed Execution Integration Tests
//!
//! Path: src/distributed/tests/execution.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verify the existing distributed execution contracts without
//! implementing a second executor or introducing a competing runtime.
//!
//! These tests cover:
//!
//! - validated-plan enforcement;
//! - execution-provider registration;
//! - provider eligibility;
//! - deterministic provider selection;
//! - actual provider invocation;
//! - execution failure propagation;
//! - preservation of successful partial results;
//! - no unsafe automatic fallback after execution begins;
//! - cancellation and deadline checks;
//! - empty-plan behavior;
//! - dynamic provider registration and removal;
//! - lifecycle transition validation;
//! - cancellation-token behavior;
//! - workload growth subject to available process resources.
//!
//! ## Architectural ownership
//!
//! These tests exercise the existing repository implementations:
//!
//! - crate::distributed::execution::executor
//! - crate::distributed::execution::lifecycle
//! - crate::distributed::execution::cancellation
//! - crate::distributed::error
//! - crate::distributed::ids
//!
//! They do not implement:
//!
//! - a second execution engine;
//! - a second provider registry;
//! - a second task identity system;
//! - a second lifecycle manager;
//! - a second cancellation runtime;
//! - a second result representation;
//! - a second planning algorithm;
//! - a transport implementation;
//! - a backend;
//! - a hardware abstraction.
//!
//! ## POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! The same logical work items must be executable through different
//! compatible providers without changing their logical identities.
//!
//! The tests use in-memory providers. They verify provider-neutral
//! execution contracts, not physical hardware, network connectivity,
//! remote authentication, or live distributed deployment.
//!
//! ## Scalability
//!
//! No universal task, provider, participant, device, or machine limit
//! is introduced by these tests.
//!
//! The workload-growth test uses a finite test workload. Its size is
//! a test input, not a semantic or architectural ceiling.
//!
//! Actual execution remains bounded by available memory, address space,
//! provider capacity, operating-system limits, and explicit operational
//! policies.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::distributed::error::{
    DistributedError,
    DistributedErrorKind,
    DistributedResult,
};

use crate::distributed::ids::{
    PlanId,
    TaskId,
};

use crate::distributed::execution::executor::{
    CancellationSignal as ExecutorCancellationSignal,
    DistributedExecutor,
    ExecutablePlan,
    ExecutionControl as ExecutorControl,
    ExecutionProvider,
    ProviderAssessment,
    ProviderRegistry,
    ProviderRegistryError,
};

use crate::distributed::execution::lifecycle::{
    ExecutionLifecycle,
    ExecutionState,
};

use crate::distributed::execution::cancellation::{
    CancellationToken,
    ExecutionControl as TokenExecutionControl,
};

// =============================================================================
// TEST IDENTITIES AND WORK ITEMS
// =============================================================================

/// Test work item.
///
/// This is a test adapter, not a replacement for Zamani's canonical IR
/// or distributed task model.
#[derive(Debug)]
struct TestWork {
    id: TaskId,
    value: u64,
}

/// A deterministic test execution plan.
///
/// Production plans must be adapted from the authoritative distributed
/// planning and canonical IR infrastructure.
struct TestPlan {
    id: PlanId,
    work: Vec<TestWork>,
    valid: bool,
}

impl TestPlan {
    /// Constructs a valid test plan from the supplied work items.
    fn new(sequence: u128, work: Vec<TestWork>) -> Self {
        Self {
            id: PlanId::new(1, sequence)
                .expect("test plan identity must be valid"),
            work,
            valid: true,
        }
    }

    /// Constructs an invalid test plan.
    fn invalid(sequence: u128, work: Vec<TestWork>) -> Self {
        Self {
            id: PlanId::new(1, sequence)
                .expect("test plan identity must be valid"),
            work,
            valid: false,
        }
    }

    /// Returns the number of work items.
    fn len(&self) -> usize {
        self.work.len()
    }
}

impl ExecutablePlan for TestPlan {
    type Work = TestWork;

    fn plan_id(&self) -> PlanId {
        self.id
    }

    fn work_items(&self) -> &[Self::Work] {
        &self.work
    }

    fn task_id(&self, work: &Self::Work) -> TaskId {
        work.id
    }

    fn validate_for_execution(&self) -> DistributedResult<()> {
        if self.valid {
            Ok(())
        } else {
            Err(DistributedError::planning_failed(
                "test-plan-validation",
                "the test plan was deliberately marked invalid",
            ))
        }
    }
}

/// Creates a test work item using a stable logical task identity.
fn work(sequence: u128, value: u64) -> TestWork {
    TestWork {
        id: TaskId::new(1, sequence)
            .expect("test task identity must be valid"),
        value,
    }
}

/// Creates a two-task plan for ordinary execution tests.
fn ordinary_plan() -> TestPlan {
    TestPlan::new(
        1,
        vec![
            work(1, 10),
            work(2, 20),
        ],
    )
}

// =============================================================================
// TEST EXECUTION PROVIDER
// =============================================================================

/// Configurable deterministic execution provider.
///
/// The provider records actual invocation counts and can be configured
/// to reject work or fail on a particular logical task.
///
/// Its successful output is derived from the supplied work item.
struct TestProvider {
    id: String,
    score: u128,
    invocations: Arc<AtomicUsize>,
    rejected: bool,
    fail_on: Option<TaskId>,
}

impl TestProvider {
    /// Constructs an eligible provider.
    fn eligible(
        id: impl Into<String>,
        score: u128,
    ) -> Self {
        Self {
            id: id.into(),
            score,
            invocations: Arc::new(AtomicUsize::new(0)),
            rejected: false,
            fail_on: None,
        }
    }

    /// Constructs a provider that refuses all work.
    fn rejecting(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            score: 0,
            invocations: Arc::new(AtomicUsize::new(0)),
            rejected: true,
            fail_on: None,
        }
    }

    /// Constructs a provider that fails on the selected task.
    fn failing(
        id: impl Into<String>,
        score: u128,
        task_id: TaskId,
    ) -> Self {
        Self {
            id: id.into(),
            score,
            invocations: Arc::new(AtomicUsize::new(0)),
            rejected: false,
            fail_on: Some(task_id),
        }
    }

    /// Returns the shared invocation counter.
    fn invocation_counter(&self) -> Arc<AtomicUsize> {
        Arc::clone(&self.invocations)
    }
}

impl ExecutionProvider<TestWork, u64> for TestProvider {
    fn provider_id(&self) -> &str {
        &self.id
    }

    fn assess(
        &self,
        _work: &TestWork,
    ) -> DistributedResult<ProviderAssessment> {
        if self.rejected {
            return Ok(ProviderAssessment::ineligible(
                "test provider deliberately rejects this workload",
            ));
        }

        Ok(ProviderAssessment::eligible(
            self.score,
            "test provider satisfies the supplied test requirements",
        ))
    }

    fn execute(
        &self,
        work: &TestWork,
        control: &ExecutorControl,
    ) -> DistributedResult<u64> {
        control.check()?;

        self.invocations.fetch_add(1, Ordering::Relaxed);

        if self.fail_on == Some(work.id) {
            return Err(DistributedError::ExecutionFailed {
                execution_id: Some(work.id.to_string()),
                message: "test provider deliberately failed this task".to_owned(),
            });
        }

        Ok(work.value)
    }
}

// =============================================================================
// CANCELLATION ADAPTER
// =============================================================================

/// Adapter between the separate cancellation interfaces currently
/// defined by executor.rs and cancellation.rs.
///
/// This adapter is deliberately explicit. It makes the current API
/// separation visible instead of pretending the two traits are already
/// the same public contract.
struct TokenSignalAdapter(CancellationToken);

impl ExecutorCancellationSignal for TokenSignalAdapter {
    fn is_cancelled(&self) -> bool {
        self.0.is_cancelled()
    }
}

// =============================================================================
// EXECUTOR TEST HELPERS
// =============================================================================

/// Creates an executor with one eligible local test provider.
fn executor_with_local_provider() -> DistributedExecutor<TestWork, u64> {
    let executor = DistributedExecutor::<TestWork, u64>::new();

    executor
        .registry()
        .register(Arc::new(TestProvider::eligible("local-test", 1)))
        .expect("local test provider registration must succeed");

    executor
}

/// Registers a provider using the executor's real registry interface.
fn register_provider(
    executor: &DistributedExecutor<TestWork, u64>,
    provider: TestProvider,
) {
    executor
        .registry()
        .register(Arc::new(provider))
        .expect("test provider registration must succeed");
}

// =============================================================================
// BASIC EXECUTION
// =============================================================================

#[test]
fn executes_every_work_item_and_returns_confirmed_results() {
    let executor = executor_with_local_provider();
    let plan = ordinary_plan();

    let report = executor
        .execute(&plan, &ExecutorControl::new())
        .expect("all test work items should execute");

    assert_eq!(report.plan_id(), plan.id);
    assert_eq!(report.completed_tasks(), plan.len());
    assert_eq!(report.results().len(), 2);

    assert_eq!(report.results()[0].task_id(), plan.work[0].id);
    assert_eq!(report.results()[0].provider_id(), "local-test");
    assert_eq!(*report.results()[0].output(), 10);

    assert_eq!(report.results()[1].task_id(), plan.work[1].id);
    assert_eq!(report.results()[1].provider_id(), "local-test");
    assert_eq!(*report.results()[1].output(), 20);

    assert!(report.elapsed() <= Duration::from_secs(30));
}

#[test]
fn report_supports_lookup_by_logical_task_identity() {
    let executor = executor_with_local_provider();
    let plan = ordinary_plan();

    let report = executor
        .execute(&plan, &ExecutorControl::new())
        .expect("execution should succeed");

    let first = report
        .result_for(plan.work[0].id)
        .expect("first task result should exist");

    assert_eq!(*first.output(), 10);

    let second = report
        .result_for(plan.work[1].id)
        .expect("second task result should exist");

    assert_eq!(*second.output(), 20);
}

#[test]
fn report_summary_matches_confirmed_results() {
    let executor = executor_with_local_provider();
    let plan = ordinary_plan();

    let report = executor
        .execute(&plan, &ExecutorControl::new())
        .expect("execution should succeed");

    let summary = report.summary();

    assert_eq!(summary.completed_tasks(), 2);
    assert!(summary.elapsed() <= Duration::from_secs(30));
}

#[test]
fn empty_plan_succeeds_without_registered_providers() {
    let executor = DistributedExecutor::<TestWork, u64>::new();
    let plan = TestPlan::new(2, Vec::new());

    let report = executor
        .execute(&plan, &ExecutorControl::new())
        .expect("an empty valid plan needs no execution provider");

    assert!(report.is_empty());
    assert_eq!(report.completed_tasks(), 0);
}

// =============================================================================
// PLAN VALIDATION
// =============================================================================

#[test]
fn invalid_plan_is_rejected_before_provider_execution() {
    let executor = executor_with_local_provider();

    let provider = TestProvider::eligible("counted-provider", 1);
    let invocations = provider.invocation_counter();

    let executor = DistributedExecutor::<TestWork, u64>::new();

    register_provider(&executor, provider);

    let plan = TestPlan::invalid(
        3,
        vec![work(3, 30)],
    );

    let failure = executor
        .execute(&plan, &ExecutorControl::new())
        .expect_err("invalid plan must not execute");

    assert_eq!(
        failure.error().kind(),
        DistributedErrorKind::PlanningFailed,
    );

    assert_eq!(failure.partial_report().completed_tasks(), 0);
    assert_eq!(invocations.load(Ordering::Relaxed), 0);
}

#[test]
fn non_empty_plan_without_providers_is_not_reported_as_success() {
    let executor = DistributedExecutor::<TestWork, u64>::new();

    let failure = executor
        .execute(&ordinary_plan(), &ExecutorControl::new())
        .expect_err("non-empty plan without providers must fail");

    assert_eq!(
        failure.error().kind(),
        DistributedErrorKind::Unavailable,
    );

    assert_eq!(failure.completed_tasks(), 0);
}

#[test]
fn provider_ineligibility_is_not_treated_as_success() {
    let executor = DistributedExecutor::<TestWork, u64>::new();

    register_provider(
        &executor,
        TestProvider::rejecting("unsupported-provider"),
    );

    let failure = executor
        .execute(&ordinary_plan(), &ExecutorControl::new())
        .expect_err("ineligible providers must not execute work");

    assert_eq!(
        failure.error().kind(),
        DistributedErrorKind::CapabilityUnsatisfied,
    );

    assert_eq!(failure.completed_tasks(), 0);
}

// =============================================================================
// PROVIDER SELECTION
// =============================================================================

#[test]
fn selects_the_lowest_scoring_eligible_provider() {
    let executor = DistributedExecutor::<TestWork, u64>::new();

    register_provider(
        &executor,
        TestProvider::eligible("slower-provider", 100),
    );

    let preferred = TestProvider::eligible("preferred-provider", 5);
    let preferred_calls = preferred.invocation_counter();

    register_provider(&executor, preferred);

    let report = executor
        .execute(&ordinary_plan(), &ExecutorControl::new())
        .expect("an eligible provider should execute the plan");

    assert_eq!(
        report.results()[0].provider_id(),
        "preferred-provider",
    );

    assert_eq!(
        preferred_calls.load(Ordering::Relaxed),
        2,
    );
}

#[test]
fn provider_selection_uses_identifier_as_deterministic_tie_breaker() {
    let executor = DistributedExecutor::<TestWork, u64>::new();

    let provider_b = TestProvider::eligible("provider-b", 10);
    let provider_b_calls = provider_b.invocation_counter();

    let provider_a = TestProvider::eligible("provider-a", 10);
    let provider_a_calls = provider_a.invocation_counter();

    // Deliberately register in reverse lexical order.
    register_provider(&executor, provider_b);
    register_provider(&executor, provider_a);

    let report = executor
        .execute(&ordinary_plan(), &ExecutorControl::new())
        .expect("execution should select an eligible provider");

    assert_eq!(
        report.results()[0].provider_id(),
        "provider-a",
    );

    assert_eq!(provider_a_calls.load(Ordering::Relaxed), 2);
    assert_eq!(provider_b_calls.load(Ordering::Relaxed), 0);
}

#[test]
fn provider_registry_returns_deterministically_ordered_identifiers() {
    let registry = ProviderRegistry::<TestWork, u64>::new();

    registry
        .register(Arc::new(TestProvider::eligible("provider-z", 1)))
        .expect("registration should succeed");

    registry
        .register(Arc::new(TestProvider::eligible("provider-a", 1)))
        .expect("registration should succeed");

    registry
        .register(Arc::new(TestProvider::eligible("provider-m", 1)))
        .expect("registration should succeed");

    assert_eq!(
        registry.provider_ids().expect("registry read should succeed"),
        vec![
            "provider-a".to_owned(),
            "provider-m".to_owned(),
            "provider-z".to_owned(),
        ],
    );
}

// =============================================================================
// PROVIDER REGISTRY MANAGEMENT
// =============================================================================

#[test]
fn duplicate_provider_registration_is_rejected() {
    let registry = ProviderRegistry::<TestWork, u64>::new();

    registry
        .register(Arc::new(TestProvider::eligible("duplicate", 1)))
        .expect("first registration should succeed");

    let error = registry
        .register(Arc::new(TestProvider::eligible("duplicate", 2)))
        .expect_err("duplicate registration must fail");

    assert!(matches!(
        error,
        ProviderRegistryError::DuplicateProvider { .. },
    ));
}

#[test]
fn invalid_provider_identifiers_are_rejected() {
    let registry = ProviderRegistry::<TestWork, u64>::new();

    let empty = registry
        .register(Arc::new(TestProvider::eligible("   ", 1)))
        .expect_err("whitespace-only identifier must fail");

    assert!(matches!(
        empty,
        ProviderRegistryError::InvalidIdentifier { .. },
    ));

    let control_character = registry
        .register(Arc::new(TestProvider::eligible("bad\nprovider", 1)))
        .expect_err("control characters must fail");

    assert!(matches!(
        control_character,
        ProviderRegistryError::InvalidIdentifier { .. },
    ));
}

#[test]
fn provider_can_be_removed_without_replacing_the_registry() {
    let registry = ProviderRegistry::<TestWork, u64>::new();

    registry
        .register(Arc::new(TestProvider::eligible("temporary", 1)))
        .expect("registration should succeed");

    assert_eq!(registry.len().expect("registry read should succeed"), 1);

    let removed = registry
        .remove("temporary")
        .expect("registry removal should succeed");

    assert!(removed.is_some());
    assert!(registry.is_empty().expect("registry read should succeed"));
}

#[test]
fn executor_clones_share_the_same_provider_registry() {
    let executor = DistributedExecutor::<TestWork, u64>::new();
    let cloned = executor.clone();

    register_provider(
        &executor,
        TestProvider::eligible("shared-provider", 1),
    );

    assert_eq!(
        cloned.registry().provider_ids().expect("registry read should succeed"),
        vec!["shared-provider".to_owned()],
    );
}

// =============================================================================
// FAILURE PROPAGATION AND PARTIAL RESULTS
// =============================================================================

#[test]
fn provider_failure_preserves_earlier_successful_results() {
    let executor = DistributedExecutor::<TestWork, u64>::new();

    let failing_task = TaskId::new(1, 2)
        .expect("test task identity must be valid");

    let provider = TestProvider::failing(
        "partially-failing-provider",
        1,
        failing_task,
    );

    register_provider(&executor, provider);

    let plan = ordinary_plan();

    let failure = executor
        .execute(&plan, &ExecutorControl::new())
        .expect_err("the second task must fail");

    assert_eq!(
        failure.error().kind(),
        DistributedErrorKind::ExecutionFailed,
    );

    assert_eq!(failure.completed_tasks(), 1);

    let partial = failure.partial_report();

    assert_eq!(partial.results().len(), 1);
    assert_eq!(partial.results()[0].task_id(), plan.work[0].id);
    assert_eq!(*partial.results()[0].output(), 10);
}

#[test]
fn executor_does_not_automatically_fallback_after_provider_execution_failure() {
    let executor = DistributedExecutor::<TestWork, u64>::new();

    let failing_task = TaskId::new(1, 1)
        .expect("test task identity must be valid");

    let preferred = TestProvider::failing(
        "preferred-provider",
        1,
        failing_task,
    );

    let preferred_calls = preferred.invocation_counter();

    let fallback = TestProvider::eligible(
        "fallback-provider",
        100,
    );

    let fallback_calls = fallback.invocation_counter();

    register_provider(&executor, fallback);
    register_provider(&executor, preferred);

    let failure = executor
        .execute(&ordinary_plan(), &ExecutorControl::new())
        .expect_err("execution failure must be propagated");

    assert_eq!(
        failure.error().kind(),
        DistributedErrorKind::ExecutionFailed,
    );

    assert_eq!(preferred_calls.load(Ordering::Relaxed), 1);

    // Retrying through another provider could repeat a side effect.
    // The executor must not silently do that.
    assert_eq!(fallback_calls.load(Ordering::Relaxed), 0);
}

#[test]
fn provider_error_context_does_not_erase_original_error_classification() {
    let executor = DistributedExecutor::<TestWork, u64>::new();

    let failing_task = TaskId::new(1, 1)
        .expect("test task identity must be valid");

    register_provider(
        &executor,
        TestProvider::failing("failure-provider", 1, failing_task),
    );

    let failure = executor
        .execute(&ordinary_plan(), &ExecutorControl::new())
        .expect_err("provider failure must be returned");

    assert_eq!(
        failure.error().kind(),
        DistributedErrorKind::ExecutionFailed,
    );

    assert_eq!(
        failure.error().code(),
        "distributed.execution_failed",
    );
}

// =============================================================================
// EXECUTOR CANCELLATION AND DEADLINES
// =============================================================================

#[test]
fn executor_rejects_already_cancelled_execution() {
    struct AlreadyCancelled;

    impl ExecutorCancellationSignal for AlreadyCancelled {
        fn is_cancelled(&self) -> bool {
            true
        }
    }

    let executor = executor_with_local_provider();

    let control = ExecutorControl::new()
        .with_cancellation(Arc::new(AlreadyCancelled));

    let failure = executor
        .execute(&ordinary_plan(), &control)
        .expect_err("cancelled execution must not start");

    assert_eq!(
        failure.error().kind(),
        DistributedErrorKind::Cancelled,
    );

    assert_eq!(failure.completed_tasks(), 0);
}

#[test]
fn executor_rejects_an_expired_deadline() {
    let executor = executor_with_local_provider();

    let control = ExecutorControl::new()
        .with_deadline(Instant::now());

    let failure = executor
        .execute(&ordinary_plan(), &control)
        .expect_err("expired deadline must prevent execution");

    assert_eq!(
        failure.error().kind(),
        DistributedErrorKind::DeadlineExceeded,
    );

    assert_eq!(failure.completed_tasks(), 0);
}

#[test]
fn executor_observes_cancellation_token_through_explicit_adapter() {
    let executor = executor_with_local_provider();

    let token = CancellationToken::new();
    assert!(token.cancel("test cancellation"));

    let signal = Arc::new(TokenSignalAdapter(token));

    let control = ExecutorControl::new()
        .with_cancellation(signal);

    let failure = executor
        .execute(&ordinary_plan(), &control)
        .expect_err("adapted cancellation must prevent execution");

    assert_eq!(
        failure.error().kind(),
        DistributedErrorKind::Cancelled,
    );
}

// =============================================================================
// CANCELLATION MODULE CONTRACT
// =============================================================================

#[test]
fn cancellation_token_clones_share_the_cancellation_state() {
    let token = CancellationToken::new();
    let cloned = token.clone();

    assert!(!token.is_cancelled());

    assert!(token.cancel("requested by test"));

    assert!(cloned.is_cancelled());
    assert_eq!(
        cloned.reason().as_deref(),
        Some("requested by test"),
    );
}

#[test]
fn cancellation_token_preserves_the_first_cancellation_reason() {
    let token = CancellationToken::new();

    assert!(token.cancel("first reason"));
    assert!(!token.cancel("second reason"));

    assert_eq!(
        token.reason().as_deref(),
        Some("first reason"),
    );
}

#[test]
fn child_cancellation_observes_parent_cancellation() {
    let parent = CancellationToken::new();

    let parent_signal: Arc<
        dyn crate::distributed::execution::cancellation::CancellationSignal,
    > = Arc::new(parent.clone());

    let child = CancellationToken::linked_to(parent_signal);

    assert!(!child.is_cancelled());

    assert!(parent.cancel("parent stopped"));

    assert!(child.is_cancelled());
    assert_eq!(
        child.reason().as_deref(),
        Some("parent stopped"),
    );
}

#[test]
fn cancellation_module_execution_control_reports_typed_cancellation() {
    let token = CancellationToken::new();

    let control = TokenExecutionControl::with_token(token.clone());

    assert!(control.check().is_ok());

    assert!(token.cancel("cancel through token"));

    let error = control
        .check()
        .expect_err("token cancellation must be visible");

    assert_eq!(
        error.kind(),
        DistributedErrorKind::Cancelled,
    );
}

#[test]
fn cancellation_module_execution_control_checks_deadlines() {
    let control = TokenExecutionControl::new()
        .with_deadline(Instant::now());

    let error = control
        .check()
        .expect_err("expired deadline must be reported");

    assert_eq!(
        error.kind(),
        DistributedErrorKind::DeadlineExceeded,
    );
}

// =============================================================================
// EXECUTION LIFECYCLE
// =============================================================================

#[test]
fn lifecycle_accepts_starts_and_completes_execution() {
    let lifecycle = ExecutionLifecycle::<String>::new();
    let execution_id = "logical-execution-1".to_owned();

    let accepted = lifecycle
        .accept(execution_id.clone())
        .expect("acceptance should succeed");

    assert_eq!(accepted.current, ExecutionState::Accepted);
    assert_eq!(accepted.revision, 0);

    let started = lifecycle
        .start(&execution_id)
        .expect("starting should succeed");

    assert_eq!(started.current, ExecutionState::Running);
    assert_eq!(started.revision, 1);

    let completed = lifecycle
        .complete(&execution_id)
        .expect("completion should succeed");

    assert_eq!(completed.current, ExecutionState::Completed);
    assert_eq!(completed.revision, 2);

    let snapshot = lifecycle
        .get(&execution_id)
        .expect("lifecycle lookup should succeed")
        .expect("execution should exist");

    assert_eq!(snapshot.state, ExecutionState::Completed);
    assert_eq!(snapshot.revision, 2);
}

#[test]
fn lifecycle_rejects_duplicate_execution_acceptance() {
    let lifecycle = ExecutionLifecycle::<String>::new();
    let execution_id = "duplicate-execution".to_owned();

    lifecycle
        .accept(execution_id.clone())
        .expect("initial acceptance should succeed");

    let error = lifecycle
        .accept(execution_id)
        .expect_err("duplicate acceptance must fail");

    assert_eq!(
        error.kind(),
        DistributedErrorKind::Conflict,
    );
}

#[test]
fn lifecycle_rejects_completion_before_execution_starts() {
    let lifecycle = ExecutionLifecycle::<String>::new();
    let execution_id = "not-started".to_owned();

    lifecycle
        .accept(execution_id.clone())
        .expect("acceptance should succeed");

    let error = lifecycle
        .complete(&execution_id)
        .expect_err("completion before start must fail");

    assert_eq!(
        error.kind(),
        DistributedErrorKind::Conflict,
    );
}

#[test]
fn lifecycle_does_not_allow_terminal_state_reopening() {
    let lifecycle = ExecutionLifecycle::<String>::new();
    let execution_id = "terminal-execution".to_owned();

    lifecycle
        .accept(execution_id.clone())
        .expect("acceptance should succeed");

    lifecycle
        .start(&execution_id)
        .expect("start should succeed");

    lifecycle
        .complete(&execution_id)
        .expect("completion should succeed");

    let error = lifecycle
        .start(&execution_id)
        .expect_err("terminal execution must not restart implicitly");

    assert_eq!(
        error.kind(),
        DistributedErrorKind::Conflict,
    );
}

#[test]
fn lifecycle_records_recovery_before_resumption() {
    let lifecycle = ExecutionLifecycle::<String>::new();
    let execution_id = "recoverable-execution".to_owned();

    lifecycle
        .accept(execution_id.clone())
        .expect("acceptance should succeed");

    lifecycle
        .start(&execution_id)
        .expect("start should succeed");

    let recovering = lifecycle
        .begin_recovery(&execution_id, "test recovery")
        .expect("recovery transition should succeed");

    assert!(matches!(
        recovering.current,
        ExecutionState::Recovering { .. },
    ));

    lifecycle
        .start(&execution_id)
        .expect("recovery may explicitly resume execution");

    lifecycle
        .complete(&execution_id)
        .expect("resumed execution may complete");
}

#[test]
fn lifecycle_events_can_be_drained_without_deleting_execution_state() {
    let lifecycle = ExecutionLifecycle::<String>::new();
    let execution_id = "event-drain-execution".to_owned();

    lifecycle
        .accept(execution_id.clone())
        .expect("acceptance should succeed");

    lifecycle
        .start(&execution_id)
        .expect("start should succeed");

    lifecycle
        .complete(&execution_id)
        .expect("completion should succeed");

    assert_eq!(
        lifecycle
            .pending_event_count()
            .expect("event count should succeed"),
        3,
    );

    let events = lifecycle
        .drain_events()
        .expect("event draining should succeed");

    assert_eq!(events.len(), 3);

    assert_eq!(
        lifecycle
            .pending_event_count()
            .expect("event count should succeed"),
        0,
    );

    assert!(
        lifecycle
            .contains(&execution_id)
            .expect("execution lookup should succeed"),
    );
}

// =============================================================================
// WORKLOAD GROWTH AND PORTABILITY
// =============================================================================

#[test]
fn executes_a_growing_workload_without_a_test_defined_semantic_ceiling() {
    let executor = executor_with_local_provider();

    // This is a finite test input, not a production maximum.
    let workload_size: usize = 1_024;

    let work_items = (1..=workload_size)
        .map(|sequence| {
            let sequence = u128::try_from(sequence)
                .expect("test sequence must fit the ID representation");

            let value = u64::try_from(sequence)
                .expect("test value must fit the test output representation");

            work(sequence, value)
        })
        .collect();

    let plan = TestPlan::new(100, work_items);

    let report = executor
        .execute(&plan, &ExecutorControl::new())
        .expect("the configured in-memory workload should execute");

    assert_eq!(report.completed_tasks(), workload_size);

    for (index, result) in report.results().iter().enumerate() {
        let expected = u64::try_from(index + 1)
            .expect("test index must fit the output representation");

        assert_eq!(*result.output(), expected);
    }
}

#[test]
fn the_same_logical_workload_can_use_different_provider_realizations() {
    let plan = ordinary_plan();

    let local_executor = DistributedExecutor::<TestWork, u64>::new();

    register_provider(
        &local_executor,
        TestProvider::eligible("local-realization", 1),
    );

    let alternate_executor = DistributedExecutor::<TestWork, u64>::new();

    register_provider(
        &alternate_executor,
        TestProvider::eligible("alternate-realization", 1),
    );

    let local_report = local_executor
        .execute(&plan, &ExecutorControl::new())
        .expect("local realization should execute");

    let alternate_report = alternate_executor
        .execute(&plan, &ExecutorControl::new())
        .expect("alternate realization should execute");

    assert_eq!(local_report.plan_id(), alternate_report.plan_id());
    assert_eq!(
        local_report.completed_tasks(),
        alternate_report.completed_tasks(),
    );

    for (local, alternate) in local_report
        .results()
        .iter()
        .zip(alternate_report.results())
    {
        assert_eq!(local.task_id(), alternate.task_id());
        assert_eq!(*local.output(), *alternate.output());
    }

    assert_eq!(
        local_report.results()[0].provider_id(),
        "local-realization",
    );

    assert_eq!(
        alternate_report.results()[0].provider_id(),
        "alternate-realization",
    );
}