
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Task Model
//!
//! Defines the canonical, domain-independent task model used by the
//! Zamani toolchain.
//!
//! ## Responsibilities
//!
//! - Define extensible task kinds.
//! - Define task input and output contracts.
//! - Define execution-safety requirements.
//! - Define retry, cancellation and resumability policies.
//! - Define task lifecycle states and validated transitions.
//! - Preserve task provenance and structured failures.
//!
//! ## Ownership boundaries
//!
//! This module does not own:
//!
//! - Task dependency graphs: `toolchain::build::graph`.
//! - Task registration: `toolchain::tasks::registry`.
//! - Task execution: `toolchain::tasks::executor`.
//! - Task cancellation coordination:
//!   `toolchain::tasks::cancellation`.
//! - Task reporting and serialization:
//!   `toolchain::tasks::reporting`.
//! - Compiler parallelism: `compiler`.
//! - Physical resource allocation: canonical capability providers.
//! - Runtime scheduling: `zur`.
//!
//! ## Scalability
//!
//! No fixed task-count, input-count, output-count, domain-count,
//! processor-count or hardware-count limits are imposed.
//!
//! Resource budgets and operational limits are supplied by the
//! authoritative configuration and resource-management systems.

use std::collections::BTreeMap;
use std::fmt;
use std::num::NonZeroU128;

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::ids::{
    ArtifactId,
    OperationId,
    ProviderId,
    TaskId,
    WorkspaceId,
};
use crate::toolchain::result::ToolchainResult;

// -----------------------------------------------------------------------------
// Task kind
// -----------------------------------------------------------------------------

/// Extensible identifier describing the kind of work a task performs.
///
/// Domain-specific implementations register their own task kinds.
///
/// Examples include classical compilation, quantum compilation,
/// HDL synthesis, AI model preparation and distributed packaging.
///
/// The model deliberately does not use a closed enumeration.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TaskKind {
    namespace: String,
    name: String,
    version: String,
}

impl TaskKind {
    /// Creates a task kind.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let namespace = namespace.into();
        let name = name.into();
        let version = version.into();

        validate_identifier("task namespace", &namespace)?;
        validate_identifier("task name", &name)?;
        validate_identifier("task version", &version)?;

        Ok(Self {
            namespace,
            name,
            version,
        })
    }

    /// Returns the namespace.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the task name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the task-kind version.
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl fmt::Display for TaskKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}::{}@{}",
            self.namespace,
            self.name,
            self.version
        )
    }
}

// -----------------------------------------------------------------------------
// Task inputs
// -----------------------------------------------------------------------------

/// Identifies the origin of a task input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskInputSource {
    /// Input originates from a declared project source.
    ProjectSource {
        /// Stable source reference.
        reference: String,
    },

    /// Input originates from a declared configuration value.
    Configuration {
        /// Configuration key.
        key: String,
    },

    /// Input originates from a verified package.
    Package {
        /// Package reference.
        reference: String,
    },

    /// Input originates from an existing task output.
    ///
    /// The authoritative build graph validates the dependency.
    TaskOutput {
        /// Producing task.
        task: TaskId,

        /// Name of the declared output.
        output: String,
    },

    /// Input originates from an externally supplied artifact.
    Artifact {
        /// Verified artifact identity.
        artifact: ArtifactId,
    },
}

/// Declares one input required by a task.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskInput {
    name: String,
    source: TaskInputSource,
    required: bool,
    content_identity: Option<String>,
}

impl TaskInput {
    /// Creates a task input.
    pub fn new(
        name: impl Into<String>,
        source: TaskInputSource,
        required: bool,
    ) -> ToolchainResult<Self> {
        let name = name.into();
        validate_identifier("input name", &name)?;

        if let TaskInputSource::TaskOutput { output, .. } = &source {
            validate_identifier("task output name", output)?;
        }

        Ok(Self {
            name,
            source,
            required,
            content_identity: None,
        })
    }

    /// Associates a stable content identity with the input.
    pub fn with_content_identity(
        mut self,
        identity: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let identity = identity.into();
        validate_identifier("content identity", &identity)?;

        self.content_identity = Some(identity);
        Ok(self)
    }

    /// Returns the input name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the input source.
    pub fn source(&self) -> &TaskInputSource {
        &self.source
    }

    /// Returns whether the input is required.
    pub fn is_required(&self) -> bool {
        self.required
    }

    /// Returns the optional content identity.
    pub fn content_identity(&self) -> Option<&str> {
        self.content_identity.as_deref()
    }
}

// -----------------------------------------------------------------------------
// Task outputs
// -----------------------------------------------------------------------------

/// Describes the expected output of a task.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskOutput {
    name: String,
    required: bool,
    artifact_kind: String,
}

impl TaskOutput {
    /// Creates an output declaration.
    pub fn new(
        name: impl Into<String>,
        artifact_kind: impl Into<String>,
        required: bool,
    ) -> ToolchainResult<Self> {
        let name = name.into();
        let artifact_kind = artifact_kind.into();

        validate_identifier("output name", &name)?;
        validate_identifier("artifact kind", &artifact_kind)?;

        Ok(Self {
            name,
            required,
            artifact_kind,
        })
    }

    /// Returns the output name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the artifact kind.
    pub fn artifact_kind(&self) -> &str {
        &self.artifact_kind
    }

    /// Returns whether the output is required.
    pub fn is_required(&self) -> bool {
        self.required
    }
}

// -----------------------------------------------------------------------------
// Execution contracts
// -----------------------------------------------------------------------------

/// Describes the concurrency guarantees of a task.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConcurrencySafety {
    /// Independent invocations may execute concurrently.
    Concurrent,

    /// Concurrent invocations require distinct isolated contexts.
    Isolated,

    /// Only one invocation may execute within the provider's
    /// declared coordination scope.
    Exclusive,

    /// Concurrency safety has not been established.
    Unknown,
}

/// Describes task cancellation behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancellationBehavior {
    /// Task implementation cooperatively observes cancellation.
    Cooperative,

    /// Cancellation is honored at explicitly defined safe points.
    AtSafePoint,

    /// Task cannot be interrupted once execution starts.
    NonInterruptible,

    /// Task supports compensation following cancellation.
    Compensatable,
}

/// Describes whether a failed task can safely be retried.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetrySafety {
    /// Re-execution is safe.
    Safe,

    /// Re-execution is safe only with an idempotency contract.
    Idempotent,

    /// Re-execution requires explicit authorization.
    Conditional,

    /// Re-execution is unsafe.
    Unsafe,

    /// Retry safety has not been established.
    Unknown,
}

/// Describes checkpoint and recovery support.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resumability {
    /// Task cannot resume from a checkpoint.
    NotResumable,

    /// Task can resume using a validated checkpoint.
    Checkpointed,

    /// Task supports provider-managed recovery.
    ProviderManaged,
}

/// Declares execution properties and safety requirements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskExecutionContract {
    concurrency: ConcurrencySafety,
    cancellation: CancellationBehavior,
    retry: RetrySafety,
    resumability: Resumability,
    deterministic: bool,
    requires_isolation: bool,
}

impl TaskExecutionContract {
    /// Creates an execution contract.
    pub fn new(
        concurrency: ConcurrencySafety,
        cancellation: CancellationBehavior,
        retry: RetrySafety,
        resumability: Resumability,
        deterministic: bool,
    ) -> Self {
        Self {
            concurrency,
            cancellation,
            retry,
            resumability,
            deterministic,
            requires_isolation: false,
        }
    }

    /// Requires an isolated execution context.
    pub fn with_isolation(mut self, required: bool) -> Self {
        self.requires_isolation = required;
        self
    }

    /// Returns concurrency safety.
    pub fn concurrency(&self) -> ConcurrencySafety {
        self.concurrency
    }

    /// Returns cancellation behavior.
    pub fn cancellation(&self) -> CancellationBehavior {
        self.cancellation
    }

    /// Returns retry safety.
    pub fn retry_safety(&self) -> RetrySafety {
        self.retry
    }

    /// Returns resumability.
    pub fn resumability(&self) -> Resumability {
        self.resumability
    }

    /// Returns whether the task declares deterministic behavior.
    pub fn is_deterministic(&self) -> bool {
        self.deterministic
    }

    /// Returns whether isolated execution is required.
    pub fn requires_isolation(&self) -> bool {
        self.requires_isolation
    }
}

// -----------------------------------------------------------------------------
// Task definition
// -----------------------------------------------------------------------------

/// Complete immutable declaration of a toolchain task.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskDefinition {
    id: TaskId,
    kind: TaskKind,
    workspace: WorkspaceId,
    inputs: BTreeMap<String, TaskInput>,
    outputs: BTreeMap<String, TaskOutput>,
    execution: TaskExecutionContract,
    provider: Option<ProviderId>,
    metadata: BTreeMap<String, String>,
}

impl TaskDefinition {
    /// Creates a task definition.
    pub fn new(
        id: TaskId,
        kind: TaskKind,
        workspace: WorkspaceId,
        execution: TaskExecutionContract,
    ) -> Self {
        Self {
            id,
            kind,
            workspace,
            inputs: BTreeMap::new(),
            outputs: BTreeMap::new(),
            execution,
            provider: None,
            metadata: BTreeMap::new(),
        }
    }

    /// Adds an input declaration.
    pub fn add_input(
        &mut self,
        input: TaskInput,
    ) -> ToolchainResult<()> {
        let name = input.name().to_owned();

        if self.inputs.contains_key(&name) {
            return Err(invalid_definition(
                "task.input.duplicate",
                format!("duplicate task input '{name}'"),
            ));
        }

        self.inputs.insert(name, input);
        Ok(())
    }

    /// Adds an output declaration.
    pub fn add_output(
        &mut self,
        output: TaskOutput,
    ) -> ToolchainResult<()> {
        let name = output.name().to_owned();

        if self.outputs.contains_key(&name) {
            return Err(invalid_definition(
                "task.output.duplicate",
                format!("duplicate task output '{name}'"),
            ));
        }

        self.outputs.insert(name, output);
        Ok(())
    }

    /// Associates an execution provider.
    pub fn with_provider(&mut self, provider: ProviderId) {
        self.provider = Some(provider);
    }

    /// Adds namespaced task metadata.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ToolchainResult<()> {
        let key = key.into();

        validate_identifier("metadata key", &key)?;

        if self.metadata.contains_key(&key) {
            return Err(invalid_definition(
                "task.metadata.duplicate",
                format!("duplicate metadata key '{key}'"),
            ));
        }

        self.metadata.insert(key, value.into());
        Ok(())
    }

    /// Validates task declaration consistency.
    pub fn validate(&self) -> ToolchainResult<()> {
        for input in self.inputs.values() {
            if let TaskInputSource::TaskOutput { task, .. } = input.source() {
                if *task == self.id {
                    return Err(invalid_definition(
                        "task.input.self_reference",
                        "a task cannot consume its own output",
                    ));
                }
            }
        }

        if self.execution.deterministic
            && self.execution.retry == RetrySafety::Unknown
        {
            return Err(invalid_definition(
                "task.execution.retry",
                "deterministic tasks must declare retry safety",
            ));
        }

        Ok(())
    }

    /// Returns task identity.
    pub fn id(&self) -> &TaskId {
        &self.id
    }

    /// Returns task kind.
    pub fn kind(&self) -> &TaskKind {
        &self.kind
    }

    /// Returns workspace identity.
    pub fn workspace(&self) -> &WorkspaceId {
        &self.workspace
    }

    /// Returns declared inputs.
    pub fn inputs(&self) -> &BTreeMap<String, TaskInput> {
        &self.inputs
    }

    /// Returns declared outputs.
    pub fn outputs(&self) -> &BTreeMap<String, TaskOutput> {
        &self.outputs
    }

    /// Returns execution contract.
    pub fn execution(&self) -> &TaskExecutionContract {
        &self.execution
    }

    /// Returns the selected provider.
    pub fn provider(&self) -> Option<&ProviderId> {
        self.provider.as_ref()
    }

    /// Returns task metadata.
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }
}

// -----------------------------------------------------------------------------
// Task lifecycle
// -----------------------------------------------------------------------------

/// Structured task failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskFailure {
    /// Stable error classification.
    pub code: String,

    /// Human-readable explanation.
    pub message: String,

    /// Whether retry may be considered.
    pub retryable: bool,
}

/// Task lifecycle status.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskStatus {
    /// Created but not yet scheduled.
    Pending,

    /// Ready for execution.
    Ready,

    /// Waiting for prerequisites.
    Blocked {
        /// Reason for blocking.
        reason: String,
    },

    /// Currently executing.
    Running {
        /// Current execution attempt.
        attempt: NonZeroU128,
    },

    /// Successfully completed.
    Succeeded {
        /// Successful attempt.
        attempt: NonZeroU128,
        /// Produced output references.
        outputs: BTreeMap<String, ArtifactId>,
    },

    /// Execution failed.
    Failed {
        /// Failed attempt.
        attempt: NonZeroU128,
        /// Failure details.
        failure: TaskFailure,
    },

    /// Execution was cancelled.
    Cancelled {
        /// Attempt, if execution had started.
        attempt: Option<NonZeroU128>,
        /// Cancellation explanation.
        reason: String,
    },

    /// Task was skipped.
    Skipped {
        /// Explanation.
        reason: String,
    },
}

impl TaskStatus {
    /// Returns a stable status name.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Ready => "ready",
            Self::Blocked { .. } => "blocked",
            Self::Running { .. } => "running",
            Self::Succeeded { .. } => "succeeded",
            Self::Failed { .. } => "failed",
            Self::Cancelled { .. } => "cancelled",
            Self::Skipped { .. } => "skipped",
        }
    }

    /// Returns whether execution is currently active.
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running { .. })
    }

    /// Returns whether execution completed successfully.
    pub fn is_successful(&self) -> bool {
        matches!(self, Self::Succeeded { .. })
    }

    /// Returns whether the task reached a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Succeeded { .. }
                | Self::Failed { .. }
                | Self::Cancelled { .. }
                | Self::Skipped { .. }
        )
    }
}

/// Mutable lifecycle state for a task.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskState {
    status: TaskStatus,
}

impl Default for TaskState {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskState {
    /// Creates a pending task state.
    pub fn new() -> Self {
        Self {
            status: TaskStatus::Pending,
        }
    }

    /// Returns the current status.
    pub fn status(&self) -> &TaskStatus {
        &self.status
    }

    /// Marks a task ready.
    pub fn mark_ready(&mut self) -> ToolchainResult<()> {
        match &self.status {
            TaskStatus::Pending | TaskStatus::Blocked { .. } => {
                self.status = TaskStatus::Ready;
                Ok(())
            }
            _ => Err(invalid_state(
                "task.state.ready",
                format!(
                    "cannot make task ready from '{}'",
                    self.status.as_str()
                ),
            )),
        }
    }

    /// Marks a task blocked.
    pub fn mark_blocked(
        &mut self,
        reason: impl Into<String>,
    ) -> ToolchainResult<()> {
        match &self.status {
            TaskStatus::Pending | TaskStatus::Ready => {
                self.status = TaskStatus::Blocked {
                    reason: reason.into(),
                };
                Ok(())
            }
            _ => Err(invalid_state(
                "task.state.blocked",
                "invalid transition to blocked",
            )),
        }
    }

    /// Starts a task attempt.
    pub fn start(
        &mut self,
        attempt: NonZeroU128,
    ) -> ToolchainResult<()> {
        if !matches!(&self.status, TaskStatus::Ready) {
            return Err(invalid_state(
                "task.state.start",
                "only a ready task can start",
            ));
        }

        self.status = TaskStatus::Running { attempt };
        Ok(())
    }

    /// Records successful completion.
    pub fn succeed(
        &mut self,
        outputs: BTreeMap<String, ArtifactId>,
    ) -> ToolchainResult<()> {
        let attempt = match &self.status {
            TaskStatus::Running { attempt } => *attempt,
            _ => {
                return Err(invalid_state(
                    "task.state.succeed",
                    "only a running task can succeed",
                ));
            }
        };

        self.status = TaskStatus::Succeeded {
            attempt,
            outputs,
        };

        Ok(())
    }

    /// Records execution failure.
    pub fn fail(
        &mut self,
        failure: TaskFailure,
    ) -> ToolchainResult<()> {
        let attempt = match &self.status {
            TaskStatus::Running { attempt } => *attempt,
            _ => {
                return Err(invalid_state(
                    "task.state.fail",
                    "only a running task can fail",
                ));
            }
        };

        self.status = TaskStatus::Failed {
            attempt,
            failure,
        };

        Ok(())
    }

    /// Cancels a task.
    pub fn cancel(
        &mut self,
        reason: impl Into<String>,
    ) -> ToolchainResult<()> {
        let attempt = match &self.status {
            TaskStatus::Pending
            | TaskStatus::Ready
            | TaskStatus::Blocked { .. } => None,

            TaskStatus::Running { attempt } => Some(*attempt),

            _ => {
                return Err(invalid_state(
                    "task.state.cancel",
                    format!(
                        "cannot cancel task from '{}'",
                        self.status.as_str()
                    ),
                ));
            }
        };

        self.status = TaskStatus::Cancelled {
            attempt,
            reason: reason.into(),
        };

        Ok(())
    }

    /// Skips a task that has not started.
    pub fn skip(
        &mut self,
        reason: impl Into<String>,
    ) -> ToolchainResult<()> {
        match &self.status {
            TaskStatus::Pending
            | TaskStatus::Ready
            | TaskStatus::Blocked { .. } => {
                self.status = TaskStatus::Skipped {
                    reason: reason.into(),
                };
                Ok(())
            }
            _ => Err(invalid_state(
                "task.state.skip",
                "only an unstarted task can be skipped",
            )),
        }
    }

    /// Requeues a failed or cancelled task after retry authorization.
    ///
    /// The executor must independently verify retry safety,
    /// attempt limits and resource budgets.
    pub fn requeue(&mut self) -> ToolchainResult<()> {
        match &self.status {
            TaskStatus::Failed { .. } | TaskStatus::Cancelled { .. } => {
                self.status = TaskStatus::Ready;
                Ok(())
            }
            _ => Err(invalid_state(
                "task.state.requeue",
                "only failed or cancelled tasks can be requeued",
            )),
        }
    }

    /// Returns the active or most recently completed attempt.
    pub fn current_attempt(&self) -> Option<NonZeroU128> {
        match &self.status {
            TaskStatus::Running { attempt }
            | TaskStatus::Succeeded { attempt, .. }
            | TaskStatus::Failed { attempt, .. } => Some(*attempt),

            TaskStatus::Cancelled { attempt, .. } => *attempt,

            _ => None,
        }
    }
}

// -----------------------------------------------------------------------------
// Provenance
// -----------------------------------------------------------------------------

/// Identifies the operation responsible for creating or executing a task.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskProvenance {
    /// Parent logical operation.
    pub operation: OperationId,

    /// Provider responsible for execution, when applicable.
    pub provider: Option<ProviderId>,

    /// Stable identity of the task definition.
    pub definition_identity: String,

    /// Optional parent task.
    pub parent_task: Option<TaskId>,
}

impl TaskProvenance {
    /// Creates task provenance.
    pub fn new(
        operation: OperationId,
        definition_identity: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let definition_identity = definition_identity.into();

        validate_identifier(
            "task definition identity",
            &definition_identity,
        )?;

        Ok(Self {
            operation,
            provider: None,
            definition_identity,
            parent_task: None,
        })
    }
}

// -----------------------------------------------------------------------------
// Validation helpers
// -----------------------------------------------------------------------------

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> ToolchainResult<()> {
    if value.trim().is_empty() {
        return Err(invalid_definition(
            field,
            format!("{field} cannot be empty"),
        ));
    }

    if value.len() > value.trim().len() {
        return Err(invalid_definition(
            field,
            format!("{field} cannot contain surrounding whitespace"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_definition(
            field,
            format!("{field} cannot contain control characters"),
        ));
    }

    Ok(())
}

fn invalid_definition(
    code: &'static str,
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::validation(code, message.into())
}

fn invalid_state(
    code: &'static str,
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::validation(code, message.into())
}
