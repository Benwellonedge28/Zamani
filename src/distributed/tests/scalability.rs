
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Distributed execution scalability and portability conformance tests.
//!
//! Path: src/distributed/tests/scalability.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! These tests exercise the existing provider-neutral distributed executor.
//! They do not define another executor, scheduler, IR, task model, or
//! resource-discovery implementation.
//!
//! A finite test workload is a measurement input, not a language-level
//! maximum or a claim of infinite physical capacity.
//!
//! Actual execution remains bounded by available memory, address space,
//! provider capacity, operating-system limits, and explicit resource policy.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::distributed::error::DistributedResult;
use crate::distributed::execution::executor::{
    DistributedExecutor,
    ExecutablePlan,
    ExecutionControl,
    ExecutionProvider,
    ProviderAssessment,
};
use crate::distributed::ids::{PlanId, TaskId};

// -----------------------------------------------------------------------------
// Test-only adapter types
// -----------------------------------------------------------------------------
//
// These types adapt test work to the existing executor contract.
// They are not replacements for Zamani's canonical distributed task model,
// semantic representation, ZUIR, ZCIR, quantum::ir, or HDLIR.

/// A deterministic test work item.
#[derive(Clone, Debug)]
struct TestWork {
    id: TaskId,
    input: u128,
}

/// A validated test plan.
///
/// Production integration must adapt an authoritative plan produced by
/// distributed planning and the existing canonical IR infrastructure.
struct TestPlan {
    id: PlanId,
    work: Vec<TestWork>,
}

impl TestPlan {
    /// Creates a test plan with the supplied logical work items.
    fn new(sequence: u128, work: Vec<TestWork>) -> Self {
        Self {
            id: PlanId::new(1, sequence)
                .expect("test plan identity must be valid"),
            work,
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
        Ok(())
    }
}

/// Creates one work item with a stable logical identity.
fn make_work(sequence: u128) -> TestWork {
    TestWork {
        id: TaskId::new(1, sequence)
            .expect("test task identity must be valid"),
        input: sequence,
    }
}

/// Creates a plan containing a dynamically selected number of work items.
///
/// Allocation failure is reported by the test instead of silently
/// truncating the workload.
fn make_plan(
    plan_sequence: u128,
    workload_size: usize,
) -> TestPlan {
    let mut work = Vec::new();

    work.try_reserve_exact(workload_size)
        .expect("test workload could not reserve the requested memory");

    for index in 0..workload_size {
        let sequence = u128::try_from(index)
            .expect("usize index must fit the task ID sequence")
            .checked_add(1)
            .expect("task ID sequence overflow");

        work.push(make_work(sequence));
    }

    TestPlan::new(plan_sequence, work)
}

// -----------------------------------------------------------------------------
// Test execution provider
// -----------------------------------------------------------------------------

/// A provider that executes the supplied test work.
///
/// The returned output is derived from the actual input. Invocation counts
/// let tests distinguish real provider calls from fabricated success.
struct TestProvider {
    id: String,
    invocations: Arc<AtomicUsize>,
}

impl TestProvider {
    /// Creates a provider with the supplied identifier.
    fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            invocations: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Returns a shared invocation counter.
    fn invocation_counter(&self) -> Arc<AtomicUsize> {
        Arc::clone(&self.invocations)
    }
}

impl ExecutionProvider<TestWork, u128> for TestProvider {
    fn provider_id(&self) -> &str {
        &self.id
    }

    fn assess(
        &self,
        _work: &TestWork,
    ) -> DistributedResult<ProviderAssessment> {
        Ok(ProviderAssessment::eligible(
            0,
            "test provider supports this test workload",
        ))
    }

    fn execute(
        &self,
        work: &TestWork,
        control: &ExecutionControl,
    ) -> DistributedResult<u128> {
        // Providers must observe cancellation and deadlines.
        control.check()?;

        self.invocations.fetch_add(1, Ordering::Relaxed);

        // This is actual test-provider work, not a fabricated constant.
        Ok(work.input)
    }
}

/// Creates an executor with one registered test provider.
fn executor_with_provider(
) -> (
    DistributedExecutor<TestWork, u128>,
    Arc<AtomicUsize>,
) {
    let executor = DistributedExecutor::<TestWork, u128>::new();
    let provider = TestProvider::new("scalability-test-provider");
    let calls = provider.invocation_counter();

    executor
        .registry()
        .register(Arc::new(provider))
        .expect("test provider registration must succeed");

    (executor, calls)
}

// -----------------------------------------------------------------------------
// Empty and minimal workloads
// -----------------------------------------------------------------------------

/// An empty workload is a valid execution and does not require a provider.
#[test]
fn empty_plan_executes_without_registered_providers() {
    let executor = DistributedExecutor::<TestWork, u128>::new();
    let plan = make_plan(1, 0);

    let report = executor
        .execute(&plan, &ExecutionControl::new())
        .expect("an empty valid plan should execute");

    assert!(report.is_empty());
    assert_eq!(report.completed_tasks(), 0);
}

/// The smallest non-empty workload must execute through a real provider.
#[test]
fn single_task_executes_and_returns_its_actual_result() {
    let (executor, calls) = executor_with_provider();
    let plan = make_plan(2, 1);

    let report = executor
        .execute(&plan, &ExecutionControl::new())
        .expect("one task should execute");

    assert_eq!(report.completed_tasks(), 1);
    assert_eq!(report.results()[0].task_id(), plan.work[0].id);
    assert_eq!(*report.results()[0].output(), 1);
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}

// -----------------------------------------------------------------------------
// Workload growth
// -----------------------------------------------------------------------------

/// Increasing workload sizes must not change the executor's semantic
/// contract or introduce a test-defined universal capacity ceiling.
///
/// These are finite CI probes. They are not maximum supported workloads.
#[test]
fn executes_progressively_larger_workloads() {
    let (executor, calls) = executor_with_provider();

    let workload_sizes = [0_usize, 1, 2, 17, 257, 1_024];

    for (plan_index, workload_size) in
        workload_sizes.into_iter().enumerate()
    {
        let plan_sequence = u128::try_from(plan_index)
            .expect("plan index must fit u128")
            .checked_add(10)
            .expect("plan sequence overflow");

        let plan = make_plan(plan_sequence, workload_size);

        let before = calls.load(Ordering::Relaxed);

        let report = executor
            .execute(&plan, &ExecutionControl::new())
            .expect("the requested workload should execute");

        assert_eq!(report.completed_tasks(), workload_size);
        assert_eq!(report.results().len(), workload_size);

        for (index, result) in report.results().iter().enumerate() {
            let expected_sequence = u128::try_from(index)
                .expect("result index must fit u128")
                .checked_add(1)
                .expect("expected sequence overflow");

            assert_eq!(
                result.task_id(),
                TaskId::new(1, expected_sequence)
                    .expect("expected task identity must be valid")
            );

            assert_eq!(*result.output(), expected_sequence);
        }

        let after = calls.load(Ordering::Relaxed);

        assert_eq!(
            after.checked_sub(before),
            Some(workload_size),
            "every work item must reach the provider exactly once"
        );
    }
}

/// The test workload can be enlarged by the test environment without
/// changing the language or executor implementation.
///
/// `ZAMANI_DISTRIBUTED_SCALABILITY_WORK_ITEMS` is a test input only.
/// It is deliberately not clamped to a hard-coded maximum.
#[test]
fn supports_environment_selected_workload_growth() {
    let workload_size = match std::env::var(
        "ZAMANI_DISTRIBUTED_SCALABILITY_WORK_ITEMS",
    ) {
        Ok(value) => value
            .parse::<usize>()
            .expect("scalability workload environment value must be usize"),

        Err(std::env::VarError::NotPresent) => 1_024,

        Err(error) => {
            panic!("cannot read scalability workload setting: {error}");
        }
    };

    let (executor, calls) = executor_with_provider();
    let plan = make_plan(100, workload_size);

    let report = executor
        .execute(&plan, &ExecutionControl::new())
        .expect("environment-selected workload should execute");

    assert_eq!(report.completed_tasks(), workload_size);
    assert_eq!(
        calls.load(Ordering::Relaxed),
        workload_size
    );
}

// -----------------------------------------------------------------------------
// Provider-neutral portability
// -----------------------------------------------------------------------------

/// The same logical plan must preserve task identities and results when
/// executed through different compatible providers.
#[test]
fn same_logical_plan_preserves_results_across_provider_realizations() {
    let plan = make_plan(200, 257);

    let first_executor =
        DistributedExecutor::<TestWork, u128>::new();

    let first_provider = TestProvider::new("realization-a");
    first_executor
        .registry()
        .register(Arc::new(first_provider))
        .expect("first provider registration must succeed");

    let second_executor =
        DistributedExecutor::<TestWork, u128>::new();

    let second_provider = TestProvider::new("realization-b");
    second_executor
        .registry()
        .register(Arc::new(second_provider))
        .expect("second provider registration must succeed");

    let first = first_executor
        .execute(&plan, &ExecutionControl::new())
        .expect("first realization should execute");

    let second = second_executor
        .execute(&plan, &ExecutionControl::new())
        .expect("second realization should execute");

    assert_eq!(first.plan_id(), second.plan_id());
    assert_eq!(first.completed_tasks(), second.completed_tasks());

    for (left, right) in first.results().iter().zip(second.results()) {
        assert_eq!(left.task_id(), right.task_id());
        assert_eq!(*left.output(), *right.output());
    }

    assert_eq!(
        first.results()[0].provider_id(),
        "realization-a"
    );

    assert_eq!(
        second.results()[0].provider_id(),
        "realization-b"
    );
}

// -----------------------------------------------------------------------------
// Identity and representational boundaries
// -----------------------------------------------------------------------------

/// Task identities must remain distinct as workload cardinality increases.
#[test]
fn generated_task_identities_are_unique() {
    use std::collections::HashSet;

    let workload_size = 4_096_usize;
    let plan = make_plan(300, workload_size);

    let identities: HashSet<TaskId> =
        plan.work.iter().map(|work| work.id).collect();

    assert_eq!(identities.len(), workload_size);
}

/// The largest representable task sequence must not wrap into another ID.
#[test]
fn maximum_task_sequence_is_representable_without_wrapping() {
    let maximum = TaskId::new(1, u128::MAX)
        .expect("maximum non-zero sequence should be representable");

    assert_eq!(maximum.sequence(), u128::MAX);
    assert_eq!(
        maximum.to_string().parse::<TaskId>(),
        Ok(maximum)
    );
}

/// Workload construction must preserve the requested number of items.
#[test]
fn plan_cardinality_matches_the_requested_workload() {
    for size in [0_usize, 1, 3, 19, 128, 1_024] {
        let plan = make_plan(400, size);

        assert_eq!(plan.len(), size);
        assert_eq!(plan.work.len(), size);
    }
}
