
#![forbid(unsafe_code)]

//! Zamani classical concurrency: task contracts and lifecycle semantics.
//!
//! File: src/classical/concurrency/task.rs
//! Rust: 2021 edition, Rust 1.97+
//!
//! # Authority and responsibilities
//!
//! This module defines hardware-independent task identities, task descriptors,
//! dependency metadata, cooperative cancellation, execution context, lifecycle
//! states, and a safe task-body abstraction.
//!
//! It does NOT implement a thread pool, executor, scheduler, distributed
//! transport, operating-system process manager, or hardware backend.
//! Those are downstream realization mechanisms.
//!
//! # POCO-REAF
//!
//! A task describes computation independently of where and how it executes.
//! This module imposes no architectural maximum on task count, worker count,
//! machine count, or dependency count. Finite host representations and
//! available resources remain real constraints.
//!
//! # Integration contract
//!
//! - `concurrency/mod.rs`: declare `pub mod task;` and re-export the intended
//!   public API.
//! - `concurrency/scheduling.rs`: consume `TaskDescriptor` and dependency
//!   metadata when constructing an execution plan.
//! - `concurrency/futures.rs`: integrate result completion and awaiting.
//! - `concurrency/synchronization.rs`: provide synchronization primitives
//!   beyond this module's cancellation signal.
//! - `concurrency/determinism.rs`: define any stronger ordering guarantees.
//! - `execution/requirements.rs` and `execution/capabilities.rs`: validate
//!   placement and resource requirements before execution.
//! - Runtime/executor: call `TaskDefinition::execute` with a matching context.
//! - Diagnostics: translate `TaskError` into the repository's canonical
//!   diagnostic representation at the integration boundary.
//!
//! No AST, IR, classical value type, or hardware identity is redefined here.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Stable, logical task identity.
///
/// Identity allocation is deliberately external. A compiler, scheduler, or
/// distributed runtime must choose an allocation policy appropriate to its
/// namespace and persistence requirements.
///
/// `u128` provides a broad representation but is not infinite. Exhaustion or
/// collision must be handled by the identity allocator, never by wrapping.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TaskId(u128);

impl TaskId {
    /// Construct an identity from an externally allocated value.
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Return the underlying identity value.
    pub const fn get(self) -> u128 {
        self.0
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "task:{}", self.0)
    }
}

/// Observable lifecycle states for a logical task.
///
/// State transitions are bookkeeping contracts. The scheduler remains the
/// authority responsible for applying them to a running execution.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TaskState {
    Created,
    Ready,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl TaskState {
    /// Whether a transition is valid under the task lifecycle contract.
    ///
    /// Terminal states cannot transition back to an executable state.
    pub const fn can_transition_to(self, next: Self) -> bool {
        use TaskState::*;

        match self {
            Created => matches!(next, Ready | Cancelled | Failed),
            Ready => matches!(next, Running | Cancelled | Failed),
            Running => matches!(next, Completed | Failed | Cancelled),
            Completed | Failed | Cancelled => false,
        }
    }

    /// Whether this state represents a terminal outcome.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

/// Relationship required between a dependency and its dependent task.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DependencyKind {
    /// The dependent task may proceed only after the prerequisite succeeds.
    Success,

    /// The dependent task may proceed after the prerequisite reaches any
    /// terminal state, including failure or cancellation.
    Completion,
}

/// A dependency on another logical task.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TaskDependency {
    task_id: TaskId,
    kind: DependencyKind,
}

impl TaskDependency {
    pub const fn new(task_id: TaskId, kind: DependencyKind) -> Self {
        Self { task_id, kind }
    }

    pub const fn task_id(self) -> TaskId {
        self.task_id
    }

    pub const fn kind(self) -> DependencyKind {
        self.kind
    }
}

/// Declarative, immutable task metadata.
///
/// Labels are descriptive only. They must not determine task identity,
/// scheduling priority, security authority, or semantic ordering.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TaskOptions {
    label: Option<String>,
    dependencies: Vec<TaskDependency>,
}

impl TaskOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_dependency(mut self, dependency: TaskDependency) -> Self {
        self.dependencies.push(dependency);
        self
    }

    pub fn with_dependencies(
        mut self,
        dependencies: impl IntoIterator<Item = TaskDependency>,
    ) -> Self {
        self.dependencies.extend(dependencies);
        self
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub fn dependencies(&self) -> &[TaskDependency] {
        &self.dependencies
    }

    /// Check local metadata invariants without consulting a scheduler.
    pub fn validate(&self, task_id: TaskId) -> Result<(), TaskError> {
        let mut seen = BTreeSet::new();

        for dependency in &self.dependencies {
            if dependency.task_id == task_id {
                return Err(TaskError::SelfDependency { task_id });
            }

            if !seen.insert(dependency.task_id) {
                return Err(TaskError::DuplicateDependency {
                    task_id,
                    dependency_id: dependency.task_id,
                });
            }
        }

        Ok(())
    }
}

/// Cooperative cancellation signal shared by a task and its observers.
///
/// Cancellation is monotonic: once requested, the signal cannot be reset.
/// Cancellation does not forcibly interrupt arbitrary Rust code. Task bodies
/// must cooperate by checking their execution context at appropriate points.
///
/// Clones share one cancellation signal. They do not create child scopes.
#[derive(Clone, Debug, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self::default()
    }

    /// Request cancellation.
    ///
    /// Returns `true` only for the call that first changes the signal from
    /// not-cancelled to cancelled.
    pub fn cancel(&self) -> bool {
        !self.cancelled.swap(true, Ordering::AcqRel)
    }

    /// Observe cancellation using acquire ordering.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    /// Return an error if cancellation has been requested.
    pub fn check(&self) -> Result<(), TaskError> {
        if self.is_cancelled() {
            Err(TaskError::Cancelled)
        } else {
            Ok(())
        }
    }
}

/// Immutable execution context supplied by the executor.
///
/// Resource access, tracing, authorization, provenance, and deadlines should
/// be added through stable repository-owned context interfaces when those
/// interfaces have been identified. This type does not guess their APIs.
#[derive(Clone, Debug)]
pub struct TaskContext {
    task_id: TaskId,
    cancellation: CancellationToken,
}

impl TaskContext {
    pub fn new(task_id: TaskId, cancellation: CancellationToken) -> Self {
        Self {
            task_id,
            cancellation,
        }
    }

    pub const fn task_id(&self) -> TaskId {
        self.task_id
    }

    pub fn cancellation(&self) -> &CancellationToken {
        &self.cancellation
    }

    pub fn check_cancelled(&self) -> Result<(), TaskError> {
        self.cancellation.check()
    }
}

/// Errors produced by task-contract validation and task-body execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskError {
    Cancelled,
    ContextIdentityMismatch {
        expected: TaskId,
        actual: TaskId,
    },
    SelfDependency {
        task_id: TaskId,
    },
    DuplicateDependency {
        task_id: TaskId,
        dependency_id: TaskId,
    },
    MissingDependency {
        task_id: TaskId,
        dependency_id: TaskId,
    },
    DependencyCycle {
        task_ids: Vec<TaskId>,
    },
    InvalidTransition {
        from: TaskState,
        to: TaskState,
    },
    Panicked {
        task_id: TaskId,
        message: String,
    },
    ExecutionFailed {
        message: String,
    },
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cancelled => write!(f, "task execution was cancelled"),
            Self::ContextIdentityMismatch { expected, actual } => {
                write!(
                    f,
                    "task context identity mismatch: expected {expected}, got {actual}"
                )
            }
            Self::SelfDependency { task_id } => {
                write!(f, "{task_id} cannot depend on itself")
            }
            Self::DuplicateDependency {
                task_id,
                dependency_id,
            } => {
                write!(f, "{task_id} has duplicate dependency {dependency_id}")
            }
            Self::MissingDependency {
                task_id,
                dependency_id,
            } => {
                write!(
                    f,
                    "{task_id} refers to missing dependency {dependency_id}"
                )
            }
            Self::DependencyCycle { task_ids } => {
                write!(f, "task dependency graph contains a cycle: {task_ids:?}")
            }
            Self::InvalidTransition { from, to } => {
                write!(f, "invalid task state transition: {from:?} -> {to:?}")
            }
            Self::Panicked { task_id, message } => {
                write!(f, "{task_id} panicked: {message}")
            }
            Self::ExecutionFailed { message } => {
                write!(f, "task execution failed: {message}")
            }
        }
    }
}

impl Error for TaskError {}

/// Declarative task metadata safe to pass to a scheduler or execution planner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskDescriptor {
    id: TaskId,
    options: TaskOptions,
}

impl TaskDescriptor {
    pub fn new(id: TaskId, options: TaskOptions) -> Result<Self, TaskError> {
        options.validate(id)?;
        Ok(Self { id, options })
    }

    pub const fn id(&self) -> TaskId {
        self.id
    }

    pub fn options(&self) -> &TaskOptions {
        &self.options
    }

    pub fn dependencies(&self) -> &[TaskDependency] {
        self.options.dependencies()
    }
}

/// An owned task body.
///
/// The body is `Send` so an executor may move it to an eligible worker.
/// This does not require a particular thread count or execution target.
///
/// `Send` does not imply that the captured data is safe for arbitrary shared
/// mutation; Rust's ownership and synchronization rules still apply.
type TaskBody<T> =
    Box<dyn FnOnce(TaskContext) -> Result<T, TaskError> + Send + 'static>;

/// A task definition ready to be consumed by an executor.
///
/// This type does not start execution on construction. Calling `execute`
/// transfers ownership of the task body to the current caller.
pub struct TaskDefinition<T> {
    descriptor: TaskDescriptor,
    body: TaskBody<T>,
}

impl<T> fmt::Debug for TaskDefinition<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskDefinition")
            .field("descriptor", &self.descriptor)
            .finish_non_exhaustive()
    }
}

impl<T> TaskDefinition<T> {
    /// Create a task with explicit identity, metadata, and an owned body.
    pub fn new<F>(
        id: TaskId,
        options: TaskOptions,
        body: F,
    ) -> Result<Self, TaskError>
    where
        F: FnOnce(TaskContext) -> Result<T, TaskError> + Send + 'static,
    {
        let descriptor = TaskDescriptor::new(id, options)?;

        Ok(Self {
            descriptor,
            body: Box::new(body),
        })
    }

    pub fn descriptor(&self) -> &TaskDescriptor {
        &self.descriptor
    }

    pub const fn id(&self) -> TaskId {
        self.descriptor.id()
    }

    /// Execute the body on the caller's execution context.
    ///
    /// This method is deliberately not an executor or scheduler. It validates
    /// the supplied identity, checks cancellation before starting, converts
    /// unwinding panics into a structured error where panic unwinding is
    /// enabled, and propagates ordinary task errors.
    ///
    /// With `panic = "abort"`, the process aborts on a panic and no conversion
    /// can occur. A panic hook may also emit output before the panic is caught.
    pub fn execute(self, context: TaskContext) -> Result<T, TaskError> {
        let id = self.descriptor.id();

        if context.task_id != id {
            return Err(TaskError::ContextIdentityMismatch {
                expected: id,
                actual: context.task_id,
            });
        }

        context.check_cancelled()?;

        match catch_unwind(AssertUnwindSafe(|| (self.body)(context))) {
            Ok(result) => result,
            Err(payload) => Err(TaskError::Panicked {
                task_id: id,
                message: panic_message(payload.as_ref()),
            }),
        }
    }
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else if let Some(message) = payload.downcast_ref::<&'static str>() {
        (*message).to_owned()
    } else {
        String::from("non-string panic payload")
    }
}

/// Validate an entire task dependency graph.
///
/// Every dependency must refer to a descriptor in the supplied graph.
/// Cycles are rejected before scheduling. The result is a deterministic
/// topological order: prerequisites appear before dependent tasks.
///
/// This function describes dependency validity only. It does not decide
/// scheduling priority, resource placement, parallelism, or execution order
/// among independent tasks beyond the deterministic order it returns.
pub fn validate_task_graph(
    tasks: &[TaskDescriptor],
) -> Result<Vec<TaskId>, TaskError> {
    let mut by_id = BTreeMap::new();

    for task in tasks {
        task.options.validate(task.id)?;

        if by_id.insert(task.id, task).is_some() {
            return Err(TaskError::ExecutionFailed {
                message: format!("duplicate task identity {}", task.id),
            });
        }
    }

    let mut indegree: BTreeMap<TaskId, usize> =
        by_id.keys().copied().map(|id| (id, 0)).collect();

    let mut dependents: BTreeMap<TaskId, Vec<TaskId>> = BTreeMap::new();

    for task in tasks {
        for dependency in task.dependencies() {
            if !by_id.contains_key(&dependency.task_id) {
                return Err(TaskError::MissingDependency {
                    task_id: task.id,
                    dependency_id: dependency.task_id,
                });
            }

            let degree = indegree
                .get_mut(&task.id)
                .expect("every descriptor identity was inserted above");

            *degree = degree.checked_add(1).ok_or_else(|| {
                TaskError::ExecutionFailed {
                    message: "dependency count exceeds host representation".into(),
                }
            })?;

            dependents
                .entry(dependency.task_id)
                .or_default()
                .push(task.id);
        }
    }

    for children in dependents.values_mut() {
        children.sort_unstable();
    }

    let mut ready: BTreeSet<TaskId> = indegree
        .iter()
        .filter_map(|(id, degree)| (*degree == 0).then_some(*id))
        .collect();

    let mut order = Vec::with_capacity(tasks.len());

    while let Some(id) = ready.pop_first() {
        order.push(id);

        if let Some(children) = dependents.get(&id) {
            for child in children {
                let degree = indegree
                    .get_mut(child)
                    .expect("every dependent identity belongs to the graph");

                *degree -= 1;

                if *degree == 0 {
                    ready.insert(*child);
                }
            }
        }
    }

    if order.len() != tasks.len() {
        let remaining = indegree
            .iter()
            .filter_map(|(id, degree)| (*degree > 0).then_some(*id))
            .collect();

        return Err(TaskError::DependencyCycle {
            task_ids: remaining,
        });
    }

    Ok(order)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u128) -> TaskId {
        TaskId::new(value)
    }

    fn descriptor(
        task_id: TaskId,
        dependencies: impl IntoIterator<Item = TaskDependency>,
    ) -> TaskDescriptor {
        TaskDescriptor::new(
            task_id,
            TaskOptions::new().with_dependencies(dependencies),
        )
        .expect("test descriptor should be valid")
    }

    #[test]
    fn task_identity_preserves_large_values() {
        let value = u128::MAX;
        assert_eq!(TaskId::new(value).get(), value);
    }

    #[test]
    fn lifecycle_rejects_resurrection() {
        assert!(TaskState::Created.can_transition_to(TaskState::Ready));
        assert!(TaskState::Running.can_transition_to(TaskState::Completed));
        assert!(!TaskState::Completed.can_transition_to(TaskState::Running));
        assert!(!TaskState::Failed.can_transition_to(TaskState::Ready));
    }

    #[test]
    fn cancellation_is_shared_and_monotonic() {
        let first = CancellationToken::new();
        let second = first.clone();

        assert!(!first.is_cancelled());
        assert!(first.cancel());
        assert!(second.is_cancelled());
        assert!(!second.cancel());
        assert_eq!(first.check(), Err(TaskError::Cancelled));
    }

    #[test]
    fn task_executes_and_returns_value() {
        let task = TaskDefinition::new(id(1), TaskOptions::new(), |ctx| {
            ctx.check_cancelled()?;
            Ok::<_, TaskError>(42_u64)
        })
        .expect("task construction should succeed");

        let context = TaskContext::new(id(1), CancellationToken::new());
        assert_eq!(task.execute(context), Ok(42));
    }

    #[test]
    fn task_rejects_mismatched_context_identity() {
        let task = TaskDefinition::new(id(1), TaskOptions::new(), |_| {
            Ok::<_, TaskError>(())
        })
        .expect("task construction should succeed");

        let error = task
            .execute(TaskContext::new(id(2), CancellationToken::new()))
            .expect_err("mismatched identity must fail");

        assert_eq!(
            error,
            TaskError::ContextIdentityMismatch {
                expected: id(1),
                actual: id(2),
            }
        );
    }

    #[test]
    fn task_does_not_start_after_cancellation() {
        let task = TaskDefinition::new(id(1), TaskOptions::new(), |_| {
            panic!("cancelled task body must not execute");
            #[allow(unreachable_code)]
            Ok::<_, TaskError>(())
        })
        .expect("task construction should succeed");

        let token = CancellationToken::new();
        token.cancel();

        assert_eq!(
            task.execute(TaskContext::new(id(1), token)),
            Err(TaskError::Cancelled)
        );
    }

    #[test]
    fn task_body_errors_are_propagated() {
        let task = TaskDefinition::new(id(1), TaskOptions::new(), |_| {
            Err::<(), _>(TaskError::ExecutionFailed {
                message: "expected test failure".into(),
            })
        })
        .expect("task construction should succeed");

        assert_eq!(
            task.execute(TaskContext::new(id(1), CancellationToken::new())),
            Err(TaskError::ExecutionFailed {
                message: "expected test failure".into(),
            })
        );
    }

    #[test]
    fn task_body_panics_become_structured_errors() {
        let task = TaskDefinition::new(id(1), TaskOptions::new(), |_| {
            panic!("deliberate test panic");
            #[allow(unreachable_code)]
            Ok::<(), TaskError>(())
        })
        .expect("task construction should succeed");

        let error = task
            .execute(TaskContext::new(id(1), CancellationToken::new()))
            .expect_err("unwinding panic should become an error");

        assert_eq!(
            error,
            TaskError::Panicked {
                task_id: id(1),
                message: "deliberate test panic".into(),
            }
        );
    }

    #[test]
    fn self_dependency_is_rejected() {
        let result = TaskDescriptor::new(
            id(1),
            TaskOptions::new().with_dependency(TaskDependency::new(
                id(1),
                DependencyKind::Success,
            )),
        );

        assert_eq!(result, Err(TaskError::SelfDependency { task_id: id(1) }));
    }

    #[test]
    fn duplicate_dependencies_are_rejected() {
        let result = TaskDescriptor::new(
            id(1),
            TaskOptions::new()
                .with_dependency(TaskDependency::new(
                    id(2),
                    DependencyKind::Success,
                ))
                .with_dependency(TaskDependency::new(
                    id(2),
                    DependencyKind::Completion,
                )),
        );

        assert_eq!(
            result,
            Err(TaskError::DuplicateDependency {
                task_id: id(1),
                dependency_id: id(2),
            })
        );
    }

    #[test]
    fn graph_orders_dependencies_before_dependents() {
        let tasks = vec![
            descriptor(
                id(3),
                [TaskDependency::new(id(2), DependencyKind::Success)],
            ),
            descriptor(id(1), []),
            descriptor(
                id(2),
                [TaskDependency::new(id(1), DependencyKind::Success)],
            ),
        ];

        let order = validate_task_graph(&tasks).expect("graph should be valid");

        let position = |task_id| {
            order
                .iter()
                .position(|candidate| *candidate == task_id)
                .expect("task should occur in the result")
        };

        assert!(position(id(1)) < position(id(2)));
        assert!(position(id(2)) < position(id(3)));
    }

    #[test]
    fn graph_rejects_missing_dependencies() {
        let tasks = vec![descriptor(
            id(1),
            [TaskDependency::new(id(99), DependencyKind::Success)],
        )];

        assert_eq!(
            validate_task_graph(&tasks),
            Err(TaskError::MissingDependency {
                task_id: id(1),
                dependency_id: id(99),
            })
        );
    }

    #[test]
    fn graph_rejects_cycles() {
        let tasks = vec![
            descriptor(
                id(1),
                [TaskDependency::new(id(2), DependencyKind::Success)],
            ),
            descriptor(
                id(2),
                [TaskDependency::new(id(1), DependencyKind::Success)],
            ),
        ];

        assert_eq!(
            validate_task_graph(&tasks),
            Err(TaskError::DependencyCycle {
                task_ids: vec![id(1), id(2)],
            })
        );
    }

    #[test]
    fn independent_tasks_have_deterministic_order() {
        let tasks = vec![
            descriptor(id(8), []),
            descriptor(id(2), []),
            descriptor(id(5), []),
        ];

        let first = validate_task_graph(&tasks).expect("graph should be valid");
        let second = validate_task_graph(&tasks).expect("graph should be valid");

        assert_eq!(first, second);
        assert_eq!(first, vec![id(2), id(5), id(8)]);
    }

    #[test]
    fn empty_graph_is_valid() {
        assert_eq!(validate_task_graph(&[]), Ok(Vec::new()));
    }
}
