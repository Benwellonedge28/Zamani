
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Contract and regression tests for Zamani's classical concurrency subsystem.
//!
//! File: src/classical/tests/concurrency.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Ownership and integration
//!
//! This file tests the existing implementation under
//! `crate::classical::concurrency`. It does not implement an executor,
//! scheduler, AST, semantic model, or a competing intermediate representation.
//!
//! The tested components retain their canonical ownership:
//! - task identity, task metadata, task lifecycle, and cancellation: task.rs;
//! - logical index domains and partitions: data_parallel.rs;
//! - local synchronization and memory-order validation: synchronization.rs;
//! - atomic-operation descriptions: atomics.rs;
//! - local message passing: channels.rs;
//! - executor-independent future composition: futures.rs;
//! - logical ordering and determinism policies: determinism.rs.
//!
//! The language-level contract remains governed by
//! `grammar/spec/concurrency.md` and `grammar/spec/determinism.md`.
//! Source syntax is owned by the grammar; semantic validation and canonical
//! IR integration belong to their existing compiler layers.
//!
//! # POCO-REAF testing principles
//!
//! These tests check that:
//! - logical task identities are not physical worker identities;
//! - invalid task dependencies are rejected;
//! - cancellation is monotonic and checked at task entry;
//! - large logical domains can be represented without eagerly materializing
//!   all of their elements;
//! - ordering guarantees are independent of insertion order;
//! - memory-order violations and resource-policy errors are explicit;
//! - communication preserves queued messages and reports closure;
//! - future combinators preserve their documented result order;
//! - safe local synchronization works across host threads.
//!
//! Test fixture sizes are workload samples, not language-level limits.
//! These tests do not claim that arbitrary resource sizes are physically
//! executable or that these local primitives implement distributed execution.
//!
//! # Integration prerequisite
//!
//! Register this file from the intended Rust test module, for example through
//! `#[cfg(test)] #[path = "tests/concurrency.rs"] mod concurrency_tests;`
//! in the appropriate parent module. The repository's `src/classical/mod.rs`
//! must first be a valid Rust module, and `crate::classical::concurrency`
//! must be reachable. Do not add a second definition of concurrency APIs
//! merely to make these tests compile.

use crate::classical::concurrency::{
    validate_task_graph, CancellationToken, DataParallelHint, DataParallelPlan,
    DependencyKind, DeterminismError, DeterminismMode, DeterminismPolicy,
    IndexDomain, NumericReproducibility, ObservableOrder, OrderKey,
    OrderedEvents, TaskContext, TaskDefinition, TaskDependency, TaskDescriptor,
    TaskError, TaskId, TaskOptions, TaskState,
};

use crate::classical::concurrency::atomics::{
    AtomicOperationKind, AtomicOperationSpec, AtomicOperationSpecError,
};

use crate::classical::concurrency::channels::{
    channel, ChannelConfig, ChannelConfigError, RecvError, TryRecvError,
    TrySendError,
};

use crate::classical::concurrency::data_parallel::{
    materializable_length, DataParallelError, PartitionGrain,
};

use crate::classical::concurrency::determinism::{
    reduce_ordered, require_logical_key_order, require_stable_reduction,
};

use crate::classical::concurrency::futures::{join_all, try_join_all};

use crate::classical::concurrency::synchronization::{
    MemoryOrder, SyncAtomicUsize, SyncMutex, SynchronizationError,
    SynchronizationIntent, SynchronizationKind, SynchronizationScope,
};

use std::future::Future;
use std::num::{NonZeroU128, NonZeroUsize};
use std::pin::pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread;

// ---------------------------------------------------------------------------
// Shared test helpers
// ---------------------------------------------------------------------------

fn task_id(value: u128) -> TaskId {
    TaskId::new(value)
}

fn descriptor(
    id: u128,
    dependencies: impl IntoIterator<Item = TaskDependency>,
) -> TaskDescriptor {
    TaskDescriptor::new(
        task_id(id),
        TaskOptions::new().with_dependencies(dependencies),
    )
    .expect("test task descriptor should satisfy local invariants")
}

fn grain(value: u128) -> PartitionGrain {
    PartitionGrain::try_from_u128(value).expect("partition grain must be positive")
}

/// A safe no-op waker for polling futures that are expected to complete
/// immediately. It intentionally does not provide an executor.
struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}

    fn wake_by_ref(self: &Arc<Self>) {}
}

/// Poll an immediately-ready future once without an external executor.
///
/// A pending future is a test failure: this helper is only for futures whose
/// completion is guaranteed by their ready child futures.
fn poll_immediately_ready<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);
    let mut future = pin!(future);

    match future.as_mut().poll(&mut context) {
        Poll::Ready(output) => output,
        Poll::Pending => {
            panic!("future expected to complete from immediately-ready inputs")
        }
    }
}

// ---------------------------------------------------------------------------
// Task identity, metadata, lifecycle, and dependency graphs
// ---------------------------------------------------------------------------

#[test]
fn task_identity_is_logical_and_round_trips() {
    let id = task_id(u128::MAX);

    assert_eq!(id.get(), u128::MAX);
    assert_eq!(id, TaskId::new(u128::MAX));
    assert_ne!(id, task_id(0));
}

#[test]
fn task_lifecycle_rejects_resurrection_from_terminal_states() {
    assert!(TaskState::Created.can_transition_to(TaskState::Ready));
    assert!(TaskState::Ready.can_transition_to(TaskState::Running));
    assert!(TaskState::Running.can_transition_to(TaskState::Completed));

    for terminal in [
        TaskState::Completed,
        TaskState::Failed,
        TaskState::Cancelled,
    ] {
        assert!(terminal.is_terminal());
        assert!(!terminal.can_transition_to(TaskState::Ready));
        assert!(!terminal.can_transition_to(TaskState::Running));
        assert!(!terminal.can_transition_to(TaskState::Completed));
    }

    assert!(!TaskState::Created.can_transition_to(TaskState::Completed));
}

#[test]
fn task_metadata_rejects_self_and_duplicate_dependencies() {
    let id = task_id(7);

    let self_dependency = TaskOptions::new().with_dependency(
        TaskDependency::new(id, DependencyKind::Success),
    );

    assert!(matches!(
        self_dependency.validate(id),
        Err(TaskError::SelfDependency { task_id }) if task_id == id
    ));

    let dependency = TaskDependency::new(task_id(3), DependencyKind::Success);
    let duplicate_dependency = TaskOptions::new()
        .with_dependency(dependency)
        .with_dependency(dependency);

    assert!(matches!(
        duplicate_dependency.validate(id),
        Err(TaskError::DuplicateDependency { task_id, dependency_id })
            if task_id == id && dependency_id == task_id(3)
    ));
}

#[test]
fn task_graph_returns_prerequisites_before_dependents() {
    let first = descriptor(1, []);
    let second = descriptor(2, []);
    let third = descriptor(
        3,
        [TaskDependency::new(task_id(1), DependencyKind::Success)],
    );

    let order = validate_task_graph(&[third, second, first])
        .expect("acyclic graph with known dependencies should validate");

    assert_eq!(order.len(), 3);

    let first_position = order
        .iter()
        .position(|id| *id == task_id(1))
        .expect("first task must appear");

    let third_position = order
        .iter()
        .position(|id| *id == task_id(3))
        .expect("dependent task must appear");

    assert!(first_position < third_position);
}

#[test]
fn task_graph_rejects_missing_dependencies_and_cycles() {
    let missing = descriptor(
        1,
        [TaskDependency::new(task_id(999), DependencyKind::Success)],
    );

    assert!(matches!(
        validate_task_graph(&[missing]),
        Err(TaskError::MissingDependency { .. })
    ));

    let first = descriptor(
        1,
        [TaskDependency::new(task_id(2), DependencyKind::Success)],
    );

    let second = descriptor(
        2,
        [TaskDependency::new(task_id(1), DependencyKind::Success)],
    );

    assert!(matches!(
        validate_task_graph(&[first, second]),
        Err(TaskError::DependencyCycle { .. })
    ));
}

#[test]
fn task_cancellation_is_monotonic_and_shared_by_clones() {
    let token = CancellationToken::new();
    let observer = token.clone();

    assert!(!token.is_cancelled());
    assert!(token.check().is_ok());

    assert!(token.cancel());
    assert!(!token.cancel());

    assert!(observer.is_cancelled());
    assert!(matches!(observer.check(), Err(TaskError::Cancelled)));
}

#[test]
fn task_execution_checks_context_identity_and_cancellation() {
    let definition = TaskDefinition::new(
        task_id(10),
        TaskOptions::new(),
        |context| {
            context.check_cancelled()?;
            Ok(context.task_id().get() + 1)
        },
    )
    .expect("valid task definition");

    let mismatch = TaskContext::new(task_id(11), CancellationToken::new());

    assert!(matches!(
        definition.execute(mismatch),
        Err(TaskError::ContextIdentityMismatch { .. })
    ));

    let definition = TaskDefinition::new(
        task_id(12),
        TaskOptions::new(),
        |_context| Ok::<_, TaskError>(42_u32),
    )
    .expect("valid task definition");

    let cancellation = CancellationToken::new();
    cancellation.cancel();

    let context = TaskContext::new(task_id(12), cancellation);

    assert!(matches!(
        definition.execute(context),
        Err(TaskError::Cancelled)
    ));
}

#[test]
fn task_execution_returns_the_body_result_without_starting_implicitly() {
    let executions = Arc::new(AtomicUsize::new(0));
    let observed = Arc::clone(&executions);

    let definition = TaskDefinition::new(
        task_id(20),
        TaskOptions::new().with_label("portable-test-task"),
        move |_context| {
            observed.fetch_add(1, Ordering::SeqCst);
            Ok::<_, TaskError>(123_u32)
        },
    )
    .expect("valid task definition");

    assert_eq!(executions.load(Ordering::SeqCst), 0);
    assert_eq!(definition.id(), task_id(20));

    let context = TaskContext::new(task_id(20), CancellationToken::new());

    assert_eq!(definition.execute(context), Ok(123));
    assert_eq!(executions.load(Ordering::SeqCst), 1);
}

// ---------------------------------------------------------------------------
// Logical data-parallel domains and scalable partition descriptions
// ---------------------------------------------------------------------------

#[test]
fn logical_domains_handle_empty_singleton_and_invalid_ranges() {
    let empty = IndexDomain::new(5, 5).expect("empty domain is valid");
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);

    let singleton = IndexDomain::new(5, 6).expect("singleton domain is valid");
    assert!(singleton.is_singleton());
    assert!(singleton.contains(5));
    assert!(!singleton.contains(6));

    assert!(matches!(
        IndexDomain::new(6, 5),
        Err(DataParallelError::InvalidDomain { .. })
    ));
}

#[test]
fn logical_partitions_are_lazy_ordered_and_cover_the_domain() {
    let domain = IndexDomain::new(10, 17).expect("valid domain");
    let plan = DataParallelPlan::new(
        domain,
        grain(3),
        DataParallelHint::RequireParallel,
    );

    assert_eq!(plan.partition_count(), 3);
    assert_eq!(plan.hint(), DataParallelHint::RequireParallel);

    let partitions: Vec<_> = plan.partitions().collect();

    assert_eq!(partitions.len(), 3);
    assert_eq!(
        partitions
            .iter()
            .map(|partition| (partition.start(), partition.end()))
            .collect::<Vec<_>>(),
        vec![(10, 13), (13, 16), (16, 17)]
    );

    assert_eq!(
        partitions.iter().map(|partition| partition.len()).sum::<u128>(),
        domain.len()
    );
}

#[test]
fn logical_partitions_do_not_overflow_near_u128_boundary() {
    let domain = IndexDomain::new(u128::MAX - 5, u128::MAX)
        .expect("representable half-open domain");

    let plan = DataParallelPlan::new(
        domain,
        grain(2),
        DataParallelHint::Auto,
    );

    let partitions: Vec<_> = plan.partitions().collect();

    assert_eq!(partitions.len(), 3);
    assert_eq!(partitions[0].start(), u128::MAX - 5);
    assert_eq!(partitions[0].end(), u128::MAX - 3);
    assert_eq!(partitions[1].end(), u128::MAX - 1);
    assert_eq!(partitions[2].end(), u128::MAX);
}

#[test]
fn empty_domain_yields_no_partitions() {
    let domain = IndexDomain::new(u128::MAX, u128::MAX)
        .expect("empty boundary domain");

    let plan = DataParallelPlan::new(
        domain,
        grain(1),
        DataParallelHint::PreferParallel,
    );

    assert!(plan.is_empty());
    assert_eq!(plan.partition_count(), 0);
    assert_eq!(plan.partitions().count(), 0);
}

#[test]
fn host_materialization_rejects_unrepresentable_lengths_explicitly() {
    let domain = IndexDomain::new(0, u128::MAX).expect("valid logical domain");

    if usize::try_from(u128::MAX).is_err() {
        assert!(matches!(
            materializable_length(domain),
            Err(DataParallelError::DomainTooLargeForHost { .. })
        ));
    } else {
        assert_eq!(
            materializable_length(domain),
            Ok(usize::MAX)
        );
    }

    let small = IndexDomain::new(4, 9).expect("valid small domain");
    assert_eq!(materializable_length(small), Ok(5));
}

// ---------------------------------------------------------------------------
// Synchronization and safe atomic operations
// ---------------------------------------------------------------------------

#[test]
fn synchronization_intent_rejects_inapplicable_memory_order() {
    let invalid = SynchronizationIntent::new(
        SynchronizationScope::Process,
        SynchronizationKind::Mutex,
        Some(MemoryOrder::Acquire),
        true,
    );

    assert!(matches!(
        invalid,
        Err(SynchronizationError::MemoryOrderNotApplicable)
    ));

    let valid = SynchronizationIntent::new(
        SynchronizationScope::SharedMemory,
        SynchronizationKind::Atomic,
        Some(MemoryOrder::Acquire),
        true,
    )
    .expect("atomic intent may carry memory order");

    assert_eq!(valid.memory_order(), Some(MemoryOrder::Acquire));
    assert!(valid.is_required());
}

#[test]
fn atomic_operation_spec_validates_load_and_compare_exchange_orders() {
    let invalid_load = AtomicOperationSpec::new(
        AtomicOperationKind::Load,
        Some(MemoryOrder::Release),
        None,
    );

    assert!(matches!(
        invalid_load,
        Err(AtomicOperationSpecError::InvalidOrder { .. })
    ));

    let invalid_pair = AtomicOperationSpec::new(
        AtomicOperationKind::CompareExchange,
        Some(MemoryOrder::Relaxed),
        Some(MemoryOrder::Acquire),
    );

    assert!(matches!(
        invalid_pair,
        Err(AtomicOperationSpecError::InvalidCompareExchangeOrders { .. })
    ));

    let valid_pair = AtomicOperationSpec::new(
        AtomicOperationKind::CompareExchange,
        Some(MemoryOrder::AcquireRelease),
        Some(MemoryOrder::Acquire),
    )
    .expect("acquire failure order is compatible with acquire-release success");

    assert_eq!(valid_pair.kind(), AtomicOperationKind::CompareExchange);
    assert_eq!(
        valid_pair.success_order(),
        Some(MemoryOrder::AcquireRelease)
    );
    assert_eq!(valid_pair.failure_order(), Some(MemoryOrder::Acquire));
}

#[test]
fn atomic_spec_rejects_failure_order_for_non_compare_exchange() {
    let result = AtomicOperationSpec::new(
        AtomicOperationKind::FetchAdd,
        Some(MemoryOrder::Relaxed),
        Some(MemoryOrder::Relaxed),
    );

    assert!(matches!(
        result,
        Err(AtomicOperationSpecError::FailureOrderNotApplicable { .. })
    ));
}

#[test]
fn checked_atomic_add_reports_overflow_without_wrapping() {
    let counter = SyncAtomicUsize::new(usize::MAX);

    assert_eq!(counter.checked_fetch_add(1), None);
    assert_eq!(
        counter.load(MemoryOrder::SequentiallyConsistent),
        Ok(usize::MAX)
    );

    assert_eq!(counter.checked_fetch_add(0), Some(usize::MAX));
}

#[test]
fn safe_mutex_protects_shared_updates_across_threads() {
    let counter = Arc::new(SyncMutex::new(0_usize));

    // This is a bounded test workload, not a concurrency-system limit.
    let worker_count = 4;
    let increments_per_worker = 250;

    let mut workers = Vec::new();

    for _ in 0..worker_count {
        let counter = Arc::clone(&counter);

        workers.push(thread::spawn(move || {
            for _ in 0..increments_per_worker {
                let mut value = counter.lock().expect("mutex lock should succeed");
                *value += 1;
            }
        }));
    }

    for worker in workers {
        worker.join().expect("worker thread should not panic");
    }

    let expected = worker_count * increments_per_worker;
    assert_eq!(*counter.lock().expect("mutex lock should succeed"), expected);
}

// ---------------------------------------------------------------------------
// Channel configuration, message ordering, and closure
// ---------------------------------------------------------------------------

#[test]
fn bounded_channel_configuration_rejects_zero_capacity() {
    assert!(matches!(
        ChannelConfig::bounded_capacity(0),
        Err(ChannelConfigError::ZeroCapacity)
    ));

    let capacity = NonZeroUsize::new(2).expect("two is nonzero");
    let config = ChannelConfig::bounded(capacity);

    assert_eq!(config.capacity(), Some(capacity));
    assert_eq!(ChannelConfig::unbounded().capacity(), None);
}

#[test]
fn bounded_channel_preserves_message_order_and_reports_full_queue() {
    let capacity = NonZeroUsize::new(1).expect("one is nonzero");
    let (sender, receiver) = channel::<String>(ChannelConfig::bounded(capacity));

    sender.send(String::from("first")).expect("first send should succeed");

    match sender.try_send(String::from("second")) {
        Err(TrySendError::Full(value)) => assert_eq!(value, "second"),
        other => panic!("expected full queue, got {other:?}"),
    }

    assert_eq!(receiver.recv(), Ok(String::from("first")));

    sender.send(String::from("second")).expect("queue should have space");
    assert_eq!(receiver.recv(), Ok(String::from("second")));
}

#[test]
fn channel_closure_preserves_queued_messages_before_closed_error() {
    let (sender, receiver) = channel::<u32>(ChannelConfig::unbounded());

    sender.send(7).expect("send should succeed");
    sender.send(9).expect("send should succeed");
    sender.close();

    assert_eq!(receiver.recv(), Ok(7));
    assert_eq!(receiver.recv(), Ok(9));
    assert_eq!(receiver.recv(), Err(RecvError::Closed));
    assert_eq!(receiver.try_recv(), Err(TryRecvError::Closed));
}

#[test]
fn dropping_all_senders_eventually_closes_the_receiver() {
    let (sender, receiver) = channel::<u32>(ChannelConfig::unbounded());

    sender.send(42).expect("send should succeed");
    drop(sender);

    assert_eq!(receiver.recv(), Ok(42));
    assert_eq!(receiver.recv(), Err(RecvError::Closed));
}

// ---------------------------------------------------------------------------
// Future composition: result ordering and error propagation
// ---------------------------------------------------------------------------

#[test]
fn join_all_returns_results_in_input_order() {
    let output = poll_immediately_ready(join_all([
        std::future::ready(30),
        std::future::ready(10),
        std::future::ready(20),
    ]));

    assert_eq!(output, vec![30, 10, 20]);
}

#[test]
fn join_all_handles_empty_input() {
    let output: Vec<u32> = poll_immediately_ready(join_all(
        std::iter::empty::<std::future::Ready<u32>>(),
    ));

    assert!(output.is_empty());
}

#[test]
fn try_join_all_preserves_success_order() {
    let output = poll_immediately_ready(try_join_all([
        std::future::ready::<Result<_, &'static str>>(Ok(3)),
        std::future::ready::<Result<_, &'static str>>(Ok(1)),
        std::future::ready::<Result<_, &'static str>>(Ok(2)),
    ]));

    assert_eq!(output, Ok(vec![3, 1, 2]));
}

#[test]
fn try_join_all_propagates_an_error() {
    let output = poll_immediately_ready(try_join_all([
        std::future::ready::<Result<u32, &'static str>>(Ok(1)),
        std::future::ready::<Result<u32, &'static str>>(Err("failure")),
        std::future::ready::<Result<u32, &'static str>>(Ok(3)),
    ]));

    assert_eq!(output, Err("failure"));
}

// ---------------------------------------------------------------------------
// Deterministic event ordering and reproducibility contracts
// ---------------------------------------------------------------------------

#[test]
fn order_keys_compare_by_logical_path_not_insertion_time() {
    let parent = OrderKey::new([3]);
    let child = OrderKey::new([3, 1]);
    let later = OrderKey::new([4]);

    assert!(parent < child);
    assert!(child < later);
    assert_eq!(OrderKey::root().depth(), 0);
    assert_eq!(child.depth(), 2);
}

#[test]
fn ordered_events_sort_by_key_and_reject_duplicates() {
    let mut events = OrderedEvents::new();

    events
        .insert(OrderKey::new([4]), "four")
        .expect("first event should be accepted");

    events
        .insert(OrderKey::new([1]), "one")
        .expect("second event should be accepted");

    events
        .insert(OrderKey::new([3]), "three")
        .expect("third event should be accepted");

    assert_eq!(
        events
            .iter()
            .map(|(key, value)| (key.clone(), *value))
            .collect::<Vec<_>>(),
        vec![
            (OrderKey::new([1]), "one"),
            (OrderKey::new([3]), "three"),
            (OrderKey::new([4]), "four"),
        ]
    );

    let duplicate = events.insert(OrderKey::new([3]), "replacement");

    assert!(matches!(
        duplicate,
        Err((DeterminismError::DuplicateOrderKey { .. }, "replacement"))
    ));

    assert_eq!(events.get(&OrderKey::new([3])), Some(&"three"));
}

#[test]
fn ordered_reduction_preserves_the_supplied_operation_order() {
    let result = reduce_ordered([1_i32, 2, 3, 4], |left, right| {
        left * 10 + right
    });

    assert_eq!(result, Some(1234));
    assert_eq!(reduce_ordered(std::iter::empty::<i32>(), i32::wrapping_add), None);
    assert_eq!(reduce_ordered([9_i32], i32::wrapping_add), Some(9));
}

#[test]
fn determinism_policy_checks_are_explicit() {
    let logical_order_policy = DeterminismPolicy::new(
        DeterminismMode::Deterministic,
        ObservableOrder::LogicalKeyOrder,
        NumericReproducibility::StableReduction,
    );

    assert!(require_logical_key_order(logical_order_policy).is_ok());
    assert!(require_stable_reduction(logical_order_policy).is_ok());

    let unspecified = DeterminismPolicy::unspecified();

    assert!(matches!(
        require_logical_key_order(unspecified),
        Err(DeterminismError::PolicyViolation { .. })
    ));

    assert!(matches!(
        require_stable_reduction(unspecified),
        Err(DeterminismError::PolicyViolation { .. })
    ));

    assert!(DeterminismPolicy::reproducible().requires_reproducibility());
}

// ---------------------------------------------------------------------------
// Public API compatibility smoke tests
// ---------------------------------------------------------------------------

#[test]
fn canonical_concurrency_types_are_available_through_expected_modules() {
    let id = TaskId::new(1);
    let options = TaskOptions::new().with_label("api-contract");
    let task = TaskDescriptor::new(id, options).expect("descriptor should be valid");

    assert_eq!(task.id(), id);
    assert_eq!(task.options().label(), Some("api-contract"));

    let domain = IndexDomain::new(0, 1).expect("domain should be valid");
    let plan = DataParallelPlan::new(
        domain,
        grain(1),
        DataParallelHint::PreferSequential,
    );

    assert_eq!(plan.domain().len(), 1);
    assert_eq!(plan.grain().get(), 1);
}
