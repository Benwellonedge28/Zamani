
#![forbid(unsafe_code)]

//! # Zamani Task Executor
//!
//! Production-oriented, domain-independent task execution orchestration.
//!
//! Responsibilities:
//! - Execute validated task graphs.
//! - Resolve registered providers.
//! - Enforce prerequisite ordering.
//! - Resolve required and optional inputs.
//! - Enforce cancellation and operational budgets.
//! - Validate provider execution contracts.
//! - Verify produced artifacts through canonical infrastructure.
//! - Preserve execution provenance.
//! - Produce deterministic execution reports.
//!
//! Ownership boundaries:
//! - Task definitions belong to `tasks::model`.
//! - Provider registration belongs to `tasks::registry`.
//! - Dependency ordering belongs to `build::graph`.
//! - Build orchestration belongs to `build`.
//! - Budget accounting belongs to canonical compiler
//!   configuration infrastructure.
//! - Artifact verification belongs to canonical artifact
//!   infrastructure.
//! - Physical execution belongs to backend, ZHAL and ZUR.
//!
//! This module does not implement compilation, hardware
//! allocation, runtime scheduling or artifact storage.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::model::{
    TaskDefinition,
    TaskExecutionContract,
    TaskId,
    TaskInput,
    TaskOutput,
    TaskState,
    TaskStatus,
};

use super::registry::{
    TaskProviderDescriptor,
    TaskRegistry,
};

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::ToolchainResult;
use crate::toolchain::ids::ArtifactId;

/// Immutable identifier for one executor invocation.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExecutionId(String);

impl ExecutionId {
    pub fn new(value: impl Into<String>) -> ToolchainResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ToolchainError::validation_failed(
                "Execution identifier cannot be empty",
            ));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExecutionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Cancellation observation interface.
///
/// The cancellation authority is owned by the caller.
/// Providers must also receive cancellation information
/// through their execution context.
pub trait CancellationProbe: Send + Sync {
    fn is_cancelled(&self) -> bool;
}

/// Cancellation implementation for operations that are
/// not externally cancellable.
#[derive(Clone, Copy, Debug, Default)]
pub struct NeverCancelled;

impl CancellationProbe for NeverCancelled {
    fn is_cancelled(&self) -> bool {
        false
    }
}

/// Shared cancellation token.
///
/// The actual token should be supplied by the canonical
/// task coordination infrastructure.
pub trait CancellationToken: CancellationProbe {
    fn cancellation_reason(&self) -> Option<String>;
}

/// Resource-accounting contract.
///
/// The implementation must delegate to the canonical
/// resource budget authority.
///
/// A reservation must be released when its execution
/// scope finishes, including failure and cancellation.
pub trait ExecutionBudget: Send + Sync {
    fn reserve(
        &self,
        task: &TaskDefinition,
    ) -> ToolchainResult<Box<dyn BudgetReservation>>;

    fn check(
        &self,
        task: &TaskDefinition,
    ) -> ToolchainResult<()>;
}

/// Resource reservation lifetime.
pub trait BudgetReservation: Send {
    fn release(self: Box<Self>) -> ToolchainResult<()>;
}

/// Canonical artifact verification boundary.
///
/// This interface must be implemented by an adapter
/// over the existing compiler/backend artifact system.
///
/// The executor does not independently calculate
/// artifact integrity or invent artifact identities.
pub trait TaskArtifactVerifier: Send + Sync {
    fn verify(
        &self,
        task: &TaskDefinition,
        output: &TaskOutput,
        artifact: &ArtifactId,
    ) -> ToolchainResult<()>;
}

/// Provider execution context.
///
/// Providers receive only the information necessary
/// to execute their declared task.
pub struct TaskExecutionContext<'a> {
    execution_id: &'a ExecutionId,
    task_id: &'a TaskId,
    cancellation: &'a dyn CancellationProbe,
    budget: &'a dyn ExecutionBudget,
    inputs: &'a ResolvedTaskInputs,
}

impl<'a> TaskExecutionContext<'a> {
    pub fn execution_id(&self) -> &ExecutionId {
        self.execution_id
    }

    pub fn task_id(&self) -> &TaskId {
        self.task_id
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }

    pub fn check_cancelled(&self) -> ToolchainResult<()> {
        if self.is_cancelled() {
            return Err(ToolchainError::cancelled(
                "Task execution was cancelled",
            ));
        }

        Ok(())
    }

    pub fn inputs(&self) -> &ResolvedTaskInputs {
        self.inputs
    }

    pub fn check_budget(
        &self,
        task: &TaskDefinition,
    ) -> ToolchainResult<()> {
        self.budget.check(task)
    }
}

/// Resolved inputs for one task.
///
/// Required and optional inputs are treated separately.
/// Missing optional inputs are not execution failures.
#[derive(Clone, Debug, Default)]
pub struct ResolvedTaskInputs {
    artifacts: BTreeMap<String, ArtifactId>,
}

impl ResolvedTaskInputs {
    pub fn get(&self, name: &str) -> Option<&ArtifactId> {
        self.artifacts.get(name)
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&String, &ArtifactId)> {
        self.artifacts.iter()
    }

    fn insert(
        &mut self,
        name: String,
        artifact: ArtifactId,
    ) {
        self.artifacts.insert(name, artifact);
    }
}

/// Provider execution interface.
///
/// Providers implement domain-specific work.
/// The executor controls orchestration and validation.
pub trait TaskExecutorProvider: Send + Sync {
    fn execute(
        &self,
        task: &TaskDefinition,
        context: &TaskExecutionContext<'_>,
    ) -> ToolchainResult<TaskExecutionOutputs>;
}

/// Provider execution outputs.
#[derive(Clone, Debug, Default)]
pub struct TaskExecutionOutputs {
    artifacts: BTreeMap<String, ArtifactId>,
}

impl TaskExecutionOutputs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        &mut self,
        name: impl Into<String>,
        artifact: ArtifactId,
    ) -> ToolchainResult<()> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ToolchainError::validation_failed(
                "Task output name cannot be empty",
            ));
        }

        if self.artifacts.insert(name.clone(), artifact).is_some() {
            return Err(ToolchainError::validation_failed(
                format!("Duplicate task output: {name}"),
            ));
        }

        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&ArtifactId> {
        self.artifacts.get(name)
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&String, &ArtifactId)> {
        self.artifacts.iter()
    }
}

/// Request to execute a previously validated task graph.
///
/// Task definitions and their dependency relationships
/// are supplied by the canonical build/task planning layer.
pub struct TaskExecutionRequest<'a> {
    pub execution_id: ExecutionId,
    pub definitions: &'a BTreeMap<TaskId, TaskDefinition>,
    pub ordered_tasks: &'a [TaskId],
    pub registry: &'a TaskRegistry,
    pub cancellation: &'a dyn CancellationProbe,
    pub budget: &'a dyn ExecutionBudget,
    pub artifact_verifier: &'a dyn TaskArtifactVerifier,
}

/// Result of executing one task.
#[derive(Clone, Debug)]
pub struct TaskExecutionRecord {
    pub task_id: TaskId,
    pub status: TaskStatus,
    pub started_after: Duration,
    pub elapsed: Duration,
    pub outputs: BTreeMap<String, ArtifactId>,
    pub error: Option<String>,
}

/// Complete execution report.
#[derive(Clone, Debug)]
pub struct TaskExecutionReport {
    pub execution_id: ExecutionId,
    pub started_at: Instant,
    pub elapsed: Duration,
    pub records: Vec<TaskExecutionRecord>,
    pub succeeded: usize,
    pub failed: usize,
    pub cancelled: usize,
    pub outputs: BTreeMap<TaskId, BTreeMap<String, ArtifactId>>,
}

impl TaskExecutionReport {
    pub fn is_success(&self) -> bool {
        self.failed == 0 && self.cancelled == 0
    }

    pub fn task_count(&self) -> usize {
        self.records.len()
    }
}

/// Domain-independent task executor.
pub struct TaskExecutor;

impl TaskExecutor {
    /// Execute tasks in an already validated topological order.
    ///
    /// This initial implementation deliberately executes
    /// sequentially. Parallel execution must be introduced
    /// only after shared provider coordination, isolation,
    /// cancellation and resource accounting are enforced.
    pub fn execute(
        request: TaskExecutionRequest<'_>,
    ) -> ToolchainResult<TaskExecutionReport> {
        Self::validate_request(&request)?;

        let started_at = Instant::now();

        let mut records = Vec::new();

        let mut completed_outputs:
            BTreeMap<TaskId, BTreeMap<String, ArtifactId>> =
            BTreeMap::new();

        let mut succeeded = 0usize;
        let mut failed = 0usize;
        let mut cancelled = 0usize;

        for task_id in request.ordered_tasks {
            let elapsed = started_at.elapsed();

            if request.cancellation.is_cancelled() {
                cancelled += 1;

                records.push(TaskExecutionRecord {
                    task_id: task_id.clone(),
                    status: TaskStatus::Cancelled,
                    started_after: elapsed,
                    elapsed: Duration::ZERO,
                    outputs: BTreeMap::new(),
                    error: Some(
                        "Execution cancelled before task start".into(),
                    ),
                });

                continue;
            }

            let definition = request
                .definitions
                .get(task_id)
                .ok_or_else(|| {
                    ToolchainError::not_found(
                        format!("Missing task definition: {task_id}"),
                    )
                })?;

            let task_started = Instant::now();

            let result = Self::execute_one(
                definition,
                &request,
                &completed_outputs,
            );

            let task_elapsed = task_started.elapsed();

            match result {
                Ok(outputs) => {
                    completed_outputs.insert(
                        task_id.clone(),
                        outputs.artifacts.clone(),
                    );

                    succeeded += 1;

                    records.push(TaskExecutionRecord {
                        task_id: task_id.clone(),
                        status: TaskStatus::Succeeded,
                        started_after: elapsed,
                        elapsed: task_elapsed,
                        outputs: outputs.artifacts,
                        error: None,
                    });
                }

                Err(error) => {
                    let is_cancelled =
                        request.cancellation.is_cancelled();

                    if is_cancelled {
                        cancelled += 1;
                    } else {
                        failed += 1;
                    }

                    records.push(TaskExecutionRecord {
                        task_id: task_id.clone(),
                        status: if is_cancelled {
                            TaskStatus::Cancelled
                        } else {
                            TaskStatus::Failed
                        },
                        started_after: elapsed,
                        elapsed: task_elapsed,
                        outputs: BTreeMap::new(),
                        error: Some(error.to_string()),
                    });

                    // Fail-fast: downstream tasks must not
                    // execute with missing prerequisites.
                    Self::mark_remaining_cancelled(
                        request.ordered_tasks,
                        task_id,
                        &mut records,
                        &mut cancelled,
                        started_at,
                    );

                    break;
                }
            }
        }

        Ok(TaskExecutionReport {
            execution_id: request.execution_id,
            started_at,
            elapsed: started_at.elapsed(),
            records,
            succeeded,
            failed,
            cancelled,
            outputs: completed_outputs,
        })
    }

    /// Validate the execution request before invoking providers.
    fn validate_request(
        request: &TaskExecutionRequest<'_>,
    ) -> ToolchainResult<()> {
        let mut seen = BTreeSet::new();

        for task_id in request.ordered_tasks {
            if !seen.insert(task_id.clone()) {
                return Err(ToolchainError::validation_failed(
                    format!("Duplicate task in execution order: {task_id}"),
                ));
            }

            if !request.definitions.contains_key(task_id) {
                return Err(ToolchainError::not_found(
                    format!("Task definition not found: {task_id}"),
                ));
            }
        }

        if seen.len() != request.definitions.len() {
            return Err(ToolchainError::validation_failed(
                "Execution order and task definitions do not match",
            ));
        }

        Self::validate_prerequisite_order(
            request.definitions,
            request.ordered_tasks,
        )?;

        Ok(())
    }

    /// Ensure prerequisites occur before dependent tasks.
    fn validate_prerequisite_order(
        definitions: &BTreeMap<TaskId, TaskDefinition>,
        ordered_tasks: &[TaskId],
    ) -> ToolchainResult<()> {
        let positions: BTreeMap<&TaskId, usize> =
            ordered_tasks
                .iter()
                .enumerate()
                .map(|(index, id)| (id, index))
                .collect();

        for (task_id, definition) in definitions {
            let task_position = positions
                .get(task_id)
                .ok_or_else(|| {
                    ToolchainError::validation_failed(
                        format!("Task missing from execution order: {task_id}"),
                    )
                })?;

            for prerequisite in definition.prerequisites() {
                let prerequisite_position = positions
                    .get(prerequisite)
                    .ok_or_else(|| {
                        ToolchainError::validation_failed(
                            format!(
                                "Missing prerequisite {prerequisite} \
                                 for task {task_id}"
                            ),
                        )
                    })?;

                if prerequisite_position >= task_position {
                    return Err(ToolchainError::validation_failed(
                        format!(
                            "Invalid execution order: prerequisite \
                             {prerequisite} must precede {task_id}"
                        ),
                    ));
                }
            }
        }

        Ok(())
    }

    /// Execute one task and validate all resulting outputs.
    fn execute_one(
        definition: &TaskDefinition,
        request: &TaskExecutionRequest<'_>,
        completed_outputs:
            &BTreeMap<TaskId, BTreeMap<String, ArtifactId>>,
    ) -> ToolchainResult<TaskExecutionOutputs> {
        if request.cancellation.is_cancelled() {
            return Err(ToolchainError::cancelled(
                "Task cancelled before provider invocation",
            ));
        }

        request.budget.check(definition)?;

        let inputs = Self::resolve_inputs(
            definition,
            completed_outputs,
        )?;

        let descriptor: &TaskProviderDescriptor =
            request.registry.resolve(definition)?;

        Self::validate_provider_contract(
            definition,
            descriptor,
        )?;

        let _reservation =
            request.budget.reserve(definition)?;

        let context = TaskExecutionContext {
            execution_id: &request.execution_id,
            task_id: definition.id(),
            cancellation: request.cancellation,
            budget: request.budget,
            inputs: &inputs,
        };

        context.check_cancelled()?;

        let provider = request.registry.provider(descriptor)?;

        let outputs = provider.execute(
            definition,
            &context,
        )?;

        context.check_cancelled()?;

        request.budget.check(definition)?;

        Self::validate_outputs(
            definition,
            &outputs,
            request.artifact_verifier,
        )?;

        Ok(outputs)
    }

    /// Resolve task inputs from completed prerequisite outputs.
    ///
    /// Missing optional inputs are permitted.
    /// Missing required inputs are rejected.
    fn resolve_inputs(
        definition: &TaskDefinition,
        completed_outputs:
            &BTreeMap<TaskId, BTreeMap<String, ArtifactId>>,
    ) -> ToolchainResult<ResolvedTaskInputs> {
        let mut resolved = ResolvedTaskInputs::default();

        for input in definition.inputs() {
            let prerequisite = input.producer_task();
            let output_name = input.producer_output();

            let producer = match completed_outputs.get(prerequisite) {
                Some(producer) => producer,
                None if !input.is_required() => continue,
                None => {
                    return Err(ToolchainError::validation_failed(
                        format!(
                            "Required prerequisite task {prerequisite} \
                             has not produced outputs"
                        ),
                    ));
                }
            };

            let artifact = match producer.get(output_name) {
                Some(artifact) => artifact,
                None if !input.is_required() => continue,
                None => {
                    return Err(ToolchainError::not_found(
                        format!(
                            "Required output '{output_name}' was not \
                             produced by task {prerequisite}"
                        ),
                    ));
                }
            };

            resolved.insert(
                input.name().to_owned(),
                artifact.clone(),
            );
        }

        Ok(resolved)
    }

    /// Validate that the selected provider supports the
    /// task's declared execution contract.
    fn validate_provider_contract(
        definition: &TaskDefinition,
        descriptor: &TaskProviderDescriptor,
    ) -> ToolchainResult<()> {
        let required = definition.execution_contract();

        let provided = descriptor.execution_contract();

        match required {
            TaskExecutionContract::Exclusive
                if !provided.supports_exclusive() =>
            {
                Err(ToolchainError::validation_failed(
                    "Selected provider does not satisfy exclusive execution",
                ))
            }

            TaskExecutionContract::Isolated
                if !provided.supports_isolation() =>
            {
                Err(ToolchainError::validation_failed(
                    "Selected provider does not satisfy isolation requirements",
                ))
            }

            _ => Ok(()),
        }
    }

    /// Verify that outputs match declarations and that
    /// every artifact passes canonical verification.
    fn validate_outputs(
        definition: &TaskDefinition,
        outputs: &TaskExecutionOutputs,
        verifier: &dyn TaskArtifactVerifier,
    ) -> ToolchainResult<()> {
        for (name, artifact) in &outputs.artifacts {
            let declaration = definition
                .outputs()
                .get(name)
                .ok_or_else(|| {
                    ToolchainError::validation_failed(
                        format!("Undeclared task output: {name}"),
                    )
                })?;

            verifier.verify(
                definition,
                declaration,
                artifact,
            )?;
        }

        for (name, declaration) in definition.outputs() {
            if declaration.is_required()
                && !outputs.artifacts.contains_key(name)
            {
                return Err(ToolchainError::validation_failed(
                    format!("Required task output missing: {name}"),
                ));
            }
        }

        Ok(())
    }

    /// Mark unexecuted tasks as cancelled after fail-fast.
    fn mark_remaining_cancelled(
        ordered_tasks: &[TaskId],
        failed_task: &TaskId,
        records: &mut Vec<TaskExecutionRecord>,
        cancelled: &mut usize,
        started_at: Instant,
    ) {
        let mut found = false;

        for task_id in ordered_tasks {
            if task_id == failed_task {
                found = true;
                continue;
            }

            if !found {
                continue;
            }

            if records.iter().any(|record| {
                &record.task_id == task_id
            }) {
                continue;
            }

            records.push(TaskExecutionRecord {
                task_id: task_id.clone(),
                status: TaskStatus::Cancelled,
                started_after: started_at.elapsed(),
                elapsed: Duration::ZERO,
                outputs: BTreeMap::new(),
                error: Some(
                    "Skipped because an earlier task failed".into(),
                ),
            });

            *cancelled += 1;
        }
    }
}
