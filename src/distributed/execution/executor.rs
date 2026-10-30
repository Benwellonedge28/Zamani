
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Distributed Execution Engine
//!
//! Path: src/distributed/execution/executor.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Executes validated, target-independent distributed execution plans
//! through dynamically registered execution providers.
//!
//! The executor is deliberately independent of:
//!
//! - Physical hardware.
//! - Particular processor architectures.
//! - Network protocols.
//! - Operating-system process identifiers.
//! - Vendor-specific accelerators.
//! - Particular computational domains.
//! - Domain-specific intermediate representations.
//! - Backend code generation.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Execution-provider registration.
//! - Provider eligibility assessment.
//! - Deterministic provider selection.
//! - Validated-plan execution coordination.
//! - Cancellation and deadline checks.
//! - Execution-result aggregation.
//! - Partial-result preservation.
//! - Execution diagnostics.
//!
//! It does not own:
//!
//! - Semantic analysis.
//! - Workload partitioning.
//! - Physical resource discovery.
//! - Placement.
//! - Scheduling.
//! - Network routing.
//! - Transport implementations.
//! - Physical device allocation.
//! - Backend compilation.
//! - Checkpoint persistence.
//! - Runtime lifecycle.
//!
//! ## Integration authorities
//!
//! Distributed semantics:
//! grammar/spec/distributed.md
//!
//! Distributed requirements:
//! crate::distributed::requirements
//!
//! Distributed identifiers:
//! crate::distributed::ids
//!
//! Distributed errors:
//! crate::distributed::error
//!
//! Planning:
//! crate::distributed::planning
//!
//! Canonical IR:
//! Existing ZUIR, ZCIR, quantum::ir and HDLIR.
//!
//! Backend:
//! Existing crate::backend
//!
//! Runtime:
//! Existing crate::runtime
//!
//! Transport:
//! Existing networking and transport infrastructure.
//!
//! ## POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! The executor consumes portable execution plans and delegates their
//! realization to providers that implement the required capabilities.
//!
//! Adding another processor, accelerator, quantum system, simulator,
//! distributed runtime, or future execution domain must not require
//! changing this executor.
//!
//! ## Scalability
//!
//! There are no arbitrary universal limits on:
//!
//! - Execution providers.
//! - Logical tasks.
//! - Participants.
//! - Computational domains.
//! - Hardware architectures.
//! - Execution contexts.
//! - Distributed machines.
//! - Result records.
//!
//! Collections grow according to available memory and explicitly
//! configured operational budgets.
//!
//! Literal infinite physical capacity is not promised. Arithmetic,
//! address-space and resource exhaustion are reported explicitly.

use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::distributed::error::{
    DeliveryState,
    DistributedError,
    DistributedResult,
};
use crate::distributed::ids::{
    PlanId,
    TaskId,
};

// -----------------------------------------------------------------------------
// API version
// -----------------------------------------------------------------------------

/// In-memory public API version of the distributed execution engine.
///
/// This is not a serialized-plan or network-protocol version.
pub const DISTRIBUTED_EXECUTOR_API_VERSION: u16 = 1;

// -----------------------------------------------------------------------------
// Cancellation
// -----------------------------------------------------------------------------

/// Cancellation signal supplied by the runtime or caller.
///
/// Implementations must be thread-safe because execution providers may
/// observe cancellation from different execution contexts.
///
/// Cancellation is cooperative. Providers executing long-running
/// operations must check this signal during their execution.
pub trait CancellationSignal: Send + Sync {
    /// Returns whether cancellation has been requested.
    fn is_cancelled(&self) -> bool;
}

/// A cancellation signal that never reports cancellation.
///
/// Useful for executions without an external cancellation source.
#[derive(Clone, Copy, Debug, Default)]
pub struct NeverCancelled;

impl CancellationSignal for NeverCancelled {
    fn is_cancelled(&self) -> bool {
        false
    }
}

// -----------------------------------------------------------------------------
// Execution control
// -----------------------------------------------------------------------------

/// Execution control information.
///
/// The deadline is an absolute monotonic-clock instant. It is not a
/// distributed wall-clock timestamp.
///
/// A distributed runtime must translate remote deadlines using its
/// established clock and deadline-propagation mechanisms.
#[derive(Clone)]
pub struct ExecutionControl {
    cancellation: Arc<dyn CancellationSignal>,
    deadline: Option<Instant>,
}

impl fmt::Debug for ExecutionControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecutionControl")
            .field("has_deadline", &self.deadline.is_some())
            .field("cancelled", &self.cancellation.is_cancelled())
            .finish()
    }
}

impl ExecutionControl {
    /// Creates execution control without a deadline or external
    /// cancellation source.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cancellation: Arc::new(NeverCancelled),
            deadline: None,
        }
    }

    /// Creates execution control with an external cancellation signal.
    #[must_use]
    pub fn with_cancellation(
        mut self,
        cancellation: Arc<dyn CancellationSignal>,
    ) -> Self {
        self.cancellation = cancellation;
        self
    }

    /// Sets an absolute monotonic-clock deadline.
    #[must_use]
    pub fn with_deadline(mut self, deadline: Instant) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Sets a deadline relative to the current monotonic clock.
    ///
    /// Returns an error if the duration cannot be represented by the
    /// underlying clock.
    pub fn with_timeout(
        self,
        timeout: Duration,
    ) -> DistributedResult<Self> {
        let deadline = Instant::now()
            .checked_add(timeout)
            .ok_or_else(|| {
                DistributedError::invalid_input(
                    "execution timeout exceeds the supported clock range",
                )
            })?;

        Ok(self.with_deadline(deadline))
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }

    /// Returns the configured deadline.
    #[must_use]
    pub const fn deadline(&self) -> Option<Instant> {
        self.deadline
    }

    /// Checks cancellation and deadline conditions.
    ///
    /// # Errors
    ///
    /// Returns a typed cancellation or deadline error.
    pub fn check(&self) -> DistributedResult<()> {
        if self.is_cancelled() {
            return Err(DistributedError::Cancelled {
                operation: None,
                reason: "distributed execution cancellation requested"
                    .to_owned(),
            });
        }

        if let Some(deadline) = self.deadline {
            if Instant::now() >= deadline {
                return Err(DistributedError::DeadlineExceeded {
                    operation: "distributed execution".to_owned(),
                    delivery: DeliveryState::NotSent,
                });
            }
        }

        Ok(())
    }

    /// Returns the remaining execution time, when a deadline exists.
    #[must_use]
    pub fn remaining(&self) -> Option<Duration> {
        self.deadline
            .map(|deadline| deadline.saturating_duration_since(Instant::now()))
    }
}

impl Default for ExecutionControl {
    fn default() -> Self {
        Self::new()
    }
}

// -----------------------------------------------------------------------------
// Validated execution-plan interface
// -----------------------------------------------------------------------------

/// Contract implemented by an adapter over an authoritative Zamani
/// execution plan.
///
/// This trait deliberately does not define another intermediate
/// representation or another planning algorithm.
///
/// The implementation must adapt an already constructed and validated
/// plan produced through the existing distributed planning and canonical
/// IR infrastructure.
///
/// The returned work items must be in an execution order that preserves
/// the dependencies and scheduling constraints established by planning.
///
/// An empty work-item collection represents a valid empty execution.
pub trait ExecutablePlan: Send + Sync {
    /// Work-item representation owned by the existing execution or
    /// canonical IR integration.
    type Work: Send + Sync;

    /// Returns the stable logical plan identifier.
    fn plan_id(&self) -> PlanId;

    /// Returns the scheduled work items in execution order.
    fn work_items(&self) -> &[Self::Work];

    /// Returns the stable logical task identifier for a work item.
    fn task_id(&self, work: &Self::Work) -> TaskId;

    /// Revalidates structural and semantic execution invariants.
    ///
    /// Implementations must not silently weaken mandatory requirements.
    ///
    /// Runtime-specific resource revalidation and reservation remain
    /// responsibilities of the appropriate runtime/resource provider.
    fn validate_for_execution(&self) -> DistributedResult<()>;
}

// -----------------------------------------------------------------------------
// Provider assessment
// -----------------------------------------------------------------------------

/// Provider eligibility result.
///
/// A score ranks eligible providers. It never overrides mandatory
/// capability, resource, security, or policy requirements.
///
/// Lower scores are preferred.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProviderAssessment {
    /// The provider can execute this work item.
    Eligible {
        /// Provider-specific ranking score.
        score: u128,

        /// Explanation of the eligibility decision.
        explanation: String,
    },

    /// The provider cannot execute this work item.
    Ineligible {
        /// Explanation of the incompatibility.
        reason: String,
    },

    /// Available evidence is insufficient to establish eligibility.
    Indeterminate {
        /// Explanation of the missing evidence.
        reason: String,
    },
}

impl ProviderAssessment {
    /// Constructs an eligible assessment.
    #[must_use]
    pub fn eligible(
        score: u128,
        explanation: impl Into<String>,
    ) -> Self {
        Self::Eligible {
            score,
            explanation: explanation.into(),
        }
    }

    /// Constructs an ineligible assessment.
    #[must_use]
    pub fn ineligible(reason: impl Into<String>) -> Self {
        Self::Ineligible {
            reason: reason.into(),
        }
    }

    /// Constructs an indeterminate assessment.
    #[must_use]
    pub fn indeterminate(reason: impl Into<String>) -> Self {
        Self::Indeterminate {
            reason: reason.into(),
        }
    }

    /// Returns whether eligibility has been established.
    #[must_use]
    pub const fn is_eligible(&self) -> bool {
        matches!(self, Self::Eligible { .. })
    }
}

// -----------------------------------------------------------------------------
// Execution-provider interface
// -----------------------------------------------------------------------------

/// Provider capable of executing a particular class of Zamani work items.
///
/// Providers are adapters over existing backend, HAL, domain-runtime,
/// local-runtime or remote-execution facilities.
///
/// A provider may internally use:
///
/// - Classical processors.
/// - Quantum processors or simulators.
/// - AI and tensor accelerators.
/// - FPGA or ASIC execution.
/// - Distributed worker pools.
/// - Remote services.
/// - Future computational technologies.
///
/// The executor does not enumerate those technologies.
///
/// Provider implementations must:
///
/// 1. Establish capability and policy eligibility.
/// 2. Revalidate volatile resources before dispatch.
/// 3. Obtain any required physical resource reservations.
/// 4. Enforce authentication and authorization.
/// 5. Honor cancellation and deadlines.
/// 6. Preserve the original execution failure.
/// 7. Report uncertain remote outcomes conservatively.
/// 8. Return success only after actual execution has succeeded.
///
/// Providers must not silently retry non-idempotent operations.
pub trait ExecutionProvider<W, O>: Send + Sync
where
    W: Send + Sync,
{
    /// Returns a stable provider identifier.
    ///
    /// Identifiers must be non-empty and contain no control characters.
    fn provider_id(&self) -> &str;

    /// Assesses whether this provider can execute the supplied work item.
    ///
    /// Returning `Indeterminate` does not establish eligibility.
    ///
    /// # Errors
    ///
    /// Returns a typed error when provider assessment itself fails.
    fn assess(
        &self,
        work: &W,
    ) -> DistributedResult<ProviderAssessment>;

    /// Executes one already validated work item.
    ///
    /// This method must perform actual execution through the provider's
    /// authoritative runtime/backend integration.
    ///
    /// The provider must not return a fabricated successful result.
    ///
    /// # Errors
    ///
    /// Returns the original classified execution, transport, security,
    /// cancellation, resource or outcome-uncertainty error.
    fn execute(
        &self,
        work: &W,
        control: &ExecutionControl,
    ) -> DistributedResult<O>;
}

// -----------------------------------------------------------------------------
// Provider registry
// -----------------------------------------------------------------------------

/// Errors associated with execution-provider registration.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProviderRegistryError {
    /// Provider identifier is empty or contains control characters.
    InvalidIdentifier {
        /// Supplied identifier.
        provider_id: String,
    },

    /// Provider identifier is already registered.
    DuplicateProvider {
        /// Conflicting identifier.
        provider_id: String,
    },

    /// Provider does not exist.
    UnknownProvider {
        /// Requested identifier.
        provider_id: String,
    },

    /// Provider registry synchronization was poisoned.
    SynchronizationFailure,
}

impl fmt::Display for ProviderRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { provider_id } => {
                write!(f, "invalid execution provider ID: {provider_id:?}")
            }

            Self::DuplicateProvider { provider_id } => {
                write!(f, "execution provider already registered: {provider_id}")
            }

            Self::UnknownProvider { provider_id } => {
                write!(f, "unknown execution provider: {provider_id}")
            }

            Self::SynchronizationFailure => {
                f.write_str("execution provider registry synchronization failed")
            }
        }
    }
}

impl std::error::Error for ProviderRegistryError {}

/// Dynamically managed execution-provider registry.
///
/// The registry has no global singleton.
///
/// Its owner is the distributed runtime or execution service that
/// constructs the executor.
///
/// Providers are stored in deterministic identifier order.
pub struct ProviderRegistry<W, O>
where
    W: Send + Sync + 'static,
{
    providers: RwLock<
        BTreeMap<String, Arc<dyn ExecutionProvider<W, O>>>,
    >,
}

impl<W, O> fmt::Debug for ProviderRegistry<W, O>
where
    W: Send + Sync + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.providers.read() {
            Ok(providers) => f
                .debug_struct("ProviderRegistry")
                .field("provider_ids", &providers.keys().collect::<Vec<_>>())
                .finish(),

            Err(_) => f
                .debug_struct("ProviderRegistry")
                .field("state", &"poisoned")
                .finish(),
        }
    }
}

impl<W, O> Default for ProviderRegistry<W, O>
where
    W: Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<W, O> ProviderRegistry<W, O>
where
    W: Send + Sync + 'static,
{
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(BTreeMap::new()),
        }
    }

    /// Registers a provider.
    ///
    /// Duplicate identifiers are rejected.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid identifiers, duplicates or
    /// synchronization failures.
    pub fn register(
        &self,
        provider: Arc<dyn ExecutionProvider<W, O>>,
    ) -> Result<(), ProviderRegistryError> {
        let provider_id = provider.provider_id().to_owned();

        validate_provider_id(&provider_id)?;

        let mut providers = self
            .providers
            .write()
            .map_err(|_| ProviderRegistryError::SynchronizationFailure)?;

        if providers.contains_key(&provider_id) {
            return Err(ProviderRegistryError::DuplicateProvider {
                provider_id,
            });
        }

        providers.insert(provider_id, provider);

        Ok(())
    }

    /// Explicitly replaces an existing provider.
    ///
    /// # Errors
    ///
    /// Returns an error if the identifier is invalid or synchronization
    /// fails.
    pub fn replace(
        &self,
        provider: Arc<dyn ExecutionProvider<W, O>>,
    ) -> Result<
        Option<Arc<dyn ExecutionProvider<W, O>>>,
        ProviderRegistryError,
    > {
        let provider_id = provider.provider_id().to_owned();

        validate_provider_id(&provider_id)?;

        let mut providers = self
            .providers
            .write()
            .map_err(|_| ProviderRegistryError::SynchronizationFailure)?;

        Ok(providers.insert(provider_id, provider))
    }

    /// Removes a registered provider.
    ///
    /// # Errors
    ///
    /// Returns an error if the identifier is invalid or synchronization
    /// fails.
    pub fn remove(
        &self,
        provider_id: &str,
    ) -> Result<
        Option<Arc<dyn ExecutionProvider<W, O>>>,
        ProviderRegistryError,
    > {
        validate_provider_id(provider_id)?;

        let mut providers = self
            .providers
            .write()
            .map_err(|_| ProviderRegistryError::SynchronizationFailure)?;

        Ok(providers.remove(provider_id))
    }

    /// Returns the number of registered providers.
    ///
    /// # Errors
    ///
    /// Returns an error if synchronization fails.
    pub fn len(&self) -> Result<usize, ProviderRegistryError> {
        let providers = self
            .providers
            .read()
            .map_err(|_| ProviderRegistryError::SynchronizationFailure)?;

        Ok(providers.len())
    }

    /// Returns whether the registry contains no providers.
    ///
    /// # Errors
    ///
    /// Returns an error if synchronization fails.
    pub fn is_empty(&self) -> Result<bool, ProviderRegistryError> {
        Ok(self.len()? == 0)
    }

    /// Returns registered provider identifiers in deterministic order.
    ///
    /// # Errors
    ///
    /// Returns an error if synchronization fails.
    pub fn provider_ids(
        &self,
    ) -> Result<Vec<String>, ProviderRegistryError> {
        let providers = self
            .providers
            .read()
            .map_err(|_| ProviderRegistryError::SynchronizationFailure)?;

        Ok(providers.keys().cloned().collect())
    }

    fn snapshot(
        &self,
    ) -> Result<
        Vec<(String, Arc<dyn ExecutionProvider<W, O>>)>,
        ProviderRegistryError,
    > {
        let providers = self
            .providers
            .read()
            .map_err(|_| ProviderRegistryError::SynchronizationFailure)?;

        Ok(providers
            .iter()
            .map(|(id, provider)| {
                (id.clone(), Arc::clone(provider))
            })
            .collect())
    }
}

fn validate_provider_id(
    provider_id: &str,
) -> Result<(), ProviderRegistryError> {
    if provider_id.trim().is_empty()
        || provider_id.chars().any(char::is_control)
    {
        return Err(ProviderRegistryError::InvalidIdentifier {
            provider_id: provider_id.to_owned(),
        });
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Execution results
// -----------------------------------------------------------------------------

/// Successful result for one logical task.
pub struct TaskExecutionResult<O> {
    task_id: TaskId,
    provider_id: String,
    output: O,
    elapsed: Duration,
}

impl<O> TaskExecutionResult<O> {
    /// Returns the logical task identifier.
    #[must_use]
    pub const fn task_id(&self) -> TaskId {
        self.task_id
    }

    /// Returns the provider that executed the task.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    /// Returns the execution output.
    #[must_use]
    pub fn output(&self) -> &O {
        &self.output
    }

    /// Consumes the result and returns its output.
    #[must_use]
    pub fn into_output(self) -> O {
        self.output
    }

    /// Returns the measured local execution duration.
    #[must_use]
    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }
}

impl<O: fmt::Debug> fmt::Debug for TaskExecutionResult<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskExecutionResult")
            .field("task_id", &self.task_id)
            .field("provider_id", &self.provider_id)
            .field("output", &self.output)
            .field("elapsed", &self.elapsed)
            .finish()
    }
}

/// Successful execution report.
///
/// A report is produced only when all work items in the validated plan
/// have completed successfully.
pub struct ExecutionReport<O> {
    plan_id: PlanId,
    results: Vec<TaskExecutionResult<O>>,
    started_at: Instant,
    finished_at: Instant,
}

impl<O> ExecutionReport<O> {
    /// Returns the executed plan identifier.
    #[must_use]
    pub const fn plan_id(&self) -> PlanId {
        self.plan_id
    }

    /// Returns all successful task results in execution-plan order.
    #[must_use]
    pub fn results(&self) -> &[TaskExecutionResult<O>] {
        &self.results
    }

    /// Returns the number of successfully executed tasks.
    #[must_use]
    pub fn completed_tasks(&self) -> usize {
        self.results.len()
    }

    /// Returns whether the execution contained no tasks.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Returns the overall local execution duration.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.finished_at
            .saturating_duration_since(self.started_at)
    }
}

impl<O: fmt::Debug> fmt::Debug for ExecutionReport<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecutionReport")
            .field("plan_id", &self.plan_id)
            .field("results", &self.results)
            .field("elapsed", &self.elapsed())
            .finish()
    }
}

/// Failure of a distributed execution invocation.
///
/// Completed results are retained even when a later task fails or
/// execution is cancelled.
///
/// No incomplete execution is represented as a successful report.
pub struct ExecutionFailure<O> {
    error: DistributedError,
    partial_report: ExecutionReport<O>,
}

impl<O> ExecutionFailure<O> {
    /// Returns the original classified execution error.
    #[must_use]
    pub fn error(&self) -> &DistributedError {
        &self.error
    }

    /// Consumes the failure and returns its original error.
    #[must_use]
    pub fn into_error(self) -> DistributedError {
        self.error
    }

    /// Returns successfully completed results preceding the failure.
    #[must_use]
    pub fn partial_report(&self) -> &ExecutionReport<O> {
        &self.partial_report
    }

    /// Consumes the failure and returns both the error and partial results.
    #[must_use]
    pub fn into_parts(self) -> (DistributedError, ExecutionReport<O>) {
        (self.error, self.partial_report)
    }
}

impl<O: fmt::Debug> fmt::Debug for ExecutionFailure<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecutionFailure")
            .field("error", &self.error)
            .field("partial_report", &self.partial_report)
            .finish()
    }
}

// -----------------------------------------------------------------------------
// Executor
// -----------------------------------------------------------------------------

/// Provider-neutral distributed execution engine.
///
/// The executor is parameterized by the work-item representation and
/// provider output type. These types are supplied by the integration
/// adapter over the existing canonical execution representation.
///
/// The executor does not construct IR, modify plans, or allocate hardware.
///
/// A provider registry can be shared by multiple executor handles through
/// `Arc`.
pub struct DistributedExecutor<W, O>
where
    W: Send + Sync + 'static,
{
    providers: Arc<ProviderRegistry<W, O>>,
    marker: PhantomData<fn(W) -> O>,
}

impl<W, O> Clone for DistributedExecutor<W, O>
where
    W: Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            providers: Arc::clone(&self.providers),
            marker: PhantomData,
        }
    }
}

impl<W, O> fmt::Debug for DistributedExecutor<W, O>
where
    W: Send + Sync + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DistributedExecutor")
            .field("providers", &self.providers)
            .finish()
    }
}

impl<W, O> Default for DistributedExecutor<W, O>
where
    W: Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<W, O> DistributedExecutor<W, O>
where
    W: Send + Sync + 'static,
{
    /// Creates an executor with an empty provider registry.
    ///
    /// An executor without registered providers cannot execute
    /// non-empty plans.
    #[must_use]
    pub fn new() -> Self {
        Self {
            providers: Arc::new(ProviderRegistry::new()),
            marker: PhantomData,
        }
    }

    /// Creates an executor using an existing shared provider registry.
    #[must_use]
    pub fn with_registry(
        providers: Arc<ProviderRegistry<W, O>>,
    ) -> Self {
        Self {
            providers,
            marker: PhantomData,
        }
    }

    /// Returns the shared provider registry.
    #[must_use]
    pub fn registry(&self) -> &Arc<ProviderRegistry<W, O>> {
        &self.providers
    }

    /// Executes a previously validated plan.
    ///
    /// Execution follows the work-item order supplied by the authoritative
    /// execution plan. Dependency and scheduling correctness must already
    /// have been established by the planning subsystem.
    ///
    /// Every work item is independently assessed against registered
    /// providers. The lowest provider score is selected, with provider ID
    /// as a deterministic tie-breaker.
    ///
    /// An execution failure never triggers an automatic fallback to
    /// another provider. Once execution may have started, repeating the
    /// operation could duplicate side effects.
    ///
    /// Providers are assessed before dispatch. Runtime resource
    /// availability must be revalidated by the provider immediately
    /// before actual execution.
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionFailure` containing:
    ///
    /// - The original typed error.
    /// - Every successfully completed preceding task.
    ///
    /// A successful report is returned only after every work item
    /// completes successfully.
    pub fn execute<P>(
        &self,
        plan: &P,
        control: &ExecutionControl,
    ) -> Result<ExecutionReport<O>, ExecutionFailure<O>>
    where
        P: ExecutablePlan<Work = W>,
    {
        let started_at = Instant::now();
        let plan_id = plan.plan_id();

        let mut results = Vec::new();

        let fail = |error, results, started_at| {
            ExecutionFailure {
                error,
                partial_report: ExecutionReport {
                    plan_id,
                    results,
                    started_at,
                    finished_at: Instant::now(),
                },
            }
        };

        if let Err(error) = control.check() {
            return Err(fail(error, results, started_at));
        }

        if let Err(error) = plan.validate_for_execution() {
            return Err(fail(
                error.with_context("distributed execution plan validation"),
                results,
                started_at,
            ));
        }

        let providers = match self.providers.snapshot() {
            Ok(providers) => providers,
            Err(error) => {
                return Err(fail(
                    DistributedError::external(
                        "distributed-provider-registry",
                        "unable to obtain execution-provider snapshot",
                        error,
                    ),
                    results,
                    started_at,
                ));
            }
        };

        if !plan.work_items().is_empty() && providers.is_empty() {
            return Err(fail(
                DistributedError::Unavailable {
                    subject: "execution-provider-registry".to_owned(),
                    message: "no execution providers are registered".to_owned(),
                },
                results,
                started_at,
            ));
        }

        for work in plan.work_items() {
            if let Err(error) = control.check() {
                return Err(fail(error, results, started_at));
            }

            let task_id = plan.task_id(work);

            let mut selected: Option<(
                u128,
                String,
                Arc<dyn ExecutionProvider<W, O>>,
            )> = None;

            let mut assessment_errors = Vec::new();

            for (provider_id, provider) in &providers {
                let assessment = match provider.assess(work) {
                    Ok(assessment) => assessment,
                    Err(error) => {
                        assessment_errors.push(
                            format!("{provider_id}: {error}"),
                        );
                        continue;
                    }
                };

                if let ProviderAssessment::Eligible {
                    score,
                    ..
                } = assessment
                {
                    let replace = match &selected {
                        None => true,
                        Some((best_score, best_id, _)) => {
                            score < *best_score
                                || (score == *best_score
                                    && provider_id < best_id)
                        }
                    };

                    if replace {
                        selected = Some((
                            score,
                            provider_id.clone(),
                            Arc::clone(provider),
                        ));
                    }
                }
            }

            let Some((_, provider_id, provider)) = selected else {
                let message = if assessment_errors.is_empty() {
                    "no registered provider established execution eligibility"
                        .to_owned()
                } else {
                    format!(
                        "no provider established eligibility; assessment failures: {}",
                        assessment_errors.join("; ")
                    )
                };

                return Err(fail(
                    DistributedError::CapabilityUnsatisfied {
                        capability: format!("execution.task.{task_id}"),
                        explanation: Some(message),
                    },
                    results,
                    started_at,
                ));
            };

            if let Err(error) = control.check() {
                return Err(fail(error, results, started_at));
            }

            let task_started = Instant::now();

            let output = match provider.execute(work, control) {
                Ok(output) => output,

                Err(error) => {
                    return Err(fail(
                        error.with_context(format!(
                            "executing task {task_id} through provider {provider_id}"
                        )),
                        results,
                        started_at,
                    ));
                }
            };

            results.push(TaskExecutionResult {
                task_id,
                provider_id,
                output,
                elapsed: task_started.elapsed(),
            });
        }

        Ok(ExecutionReport {
            plan_id,
            results,
            started_at,
            finished_at: Instant::now(),
        })
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use crate::distributed::ids::PlanId;

    #[derive(Clone, Debug)]
    struct TestWork {
        id: TaskId,
        value: u64,
    }

    struct TestPlan {
        id: PlanId,
        work: Vec<TestWork>,
        valid: bool,
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
                    "test-plan",
                    "plan validation failed",
                ))
            }
        }
    }

    struct TestProvider {
        id: String,
        score: u128,
    }

    impl ExecutionProvider<TestWork, u64> for TestProvider {
        fn provider_id(&self) -> &str {
            &self.id
        }

        fn assess(
            &self,
            _work: &TestWork,
        ) -> DistributedResult<ProviderAssessment> {
            Ok(ProviderAssessment::eligible(
                self.score,
                "test provider is eligible",
            ))
        }

        fn execute(
            &self,
            work: &TestWork,
            control: &ExecutionControl,
        ) -> DistributedResult<u64> {
            control.check()?;
            Ok(work.value)
        }
    }

    fn test_plan() -> TestPlan {
        TestPlan {
            id: PlanId::new(1, 1).unwrap(),
            work: vec![
                TestWork {
                    id: TaskId::new(1, 2).unwrap(),
                    value: 10,
                },
                TestWork {
                    id: TaskId::new(1, 3).unwrap(),
                    value: 20,
                },
            ],
            valid: true,
        }
    }

    #[test]
    fn executes_all_tasks() {
        let executor = DistributedExecutor::<TestWork, u64>::new();

        executor
            .registry()
            .register(Arc::new(TestProvider {
                id: "local".to_owned(),
                score: 1,
            }))
            .unwrap();

        let report = executor
            .execute(&test_plan(), &ExecutionControl::new())
            .unwrap();

        assert_eq!(report.completed_tasks(), 2);
        assert_eq!(*report.results()[0].output(), 10);
        assert_eq!(*report.results()[1].output(), 20);
    }

    #[test]
    fn selects_the_lowest_scoring_provider() {
        let executor = DistributedExecutor::<TestWork, u64>::new();

        executor
            .registry()
            .register(Arc::new(TestProvider {
                id: "provider-b".to_owned(),
                score: 20,
            }))
            .unwrap();

        executor
            .registry()
            .register(Arc::new(TestProvider {
                id: "provider-a".to_owned(),
                score: 5,
            }))
            .unwrap();

        let report = executor
            .execute(&test_plan(), &ExecutionControl::new())
            .unwrap();

        assert_eq!(
            report.results()[0].provider_id(),
            "provider-a"
        );
    }

    #[test]
    fn rejects_invalid_plans() {
        let executor = DistributedExecutor::<TestWork, u64>::new();

        let mut plan = test_plan();
        plan.valid = false;

        let failure = executor
            .execute(&plan, &ExecutionControl::new())
            .unwrap_err();

        assert_eq!(
            failure.error().kind(),
            crate::distributed::error::DistributedErrorKind::PlanningFailed
        );

        assert_eq!(failure.partial_report().completed_tasks(), 0);
    }

    #[test]
    fn rejects_execution_without_providers() {
        let executor = DistributedExecutor::<TestWork, u64>::new();

        let failure = executor
            .execute(&test_plan(), &ExecutionControl::new())
            .unwrap_err();

        assert_eq!(
            failure.error().kind(),
            crate::distributed::error::DistributedErrorKind::Unavailable
        );
    }

    #[test]
    fn empty_plan_executes_without_providers() {
        let executor = DistributedExecutor::<TestWork, u64>::new();

        let mut plan = test_plan();
        plan.work.clear();

        let report = executor
            .execute(&plan, &ExecutionControl::new())
            .unwrap();

        assert!(report.is_empty());
        assert_eq!(report.completed_tasks(), 0);
    }

    #[test]
    fn cancellation_prevents_execution() {
        struct Cancelled;

        impl CancellationSignal for Cancelled {
            fn is_cancelled(&self) -> bool {
                true
            }
        }

        let executor = DistributedExecutor::<TestWork, u64>::new();

        executor
            .registry()
            .register(Arc::new(TestProvider {
                id: "local".to_owned(),
                score: 1,
            }))
            .unwrap();

        let control = ExecutionControl::new()
            .with_cancellation(Arc::new(Cancelled));

        let failure = executor
            .execute(&test_plan(), &control)
            .unwrap_err();

        assert_eq!(
            failure.error().kind(),
            crate::distributed::error::DistributedErrorKind::Cancelled
        );
    }

    #[test]
    fn deadline_prevents_execution() {
        let executor = DistributedExecutor::<TestWork, u64>::new();

        executor
            .registry()
            .register(Arc::new(TestProvider {
                id: "local".to_owned(),
                score: 1,
            }))
            .unwrap();

        let control = ExecutionControl::new()
            .with_deadline(Instant::now());

        let failure = executor
            .execute(&test_plan(), &control)
            .unwrap_err();

        assert_eq!(
            failure.error().kind(),
            crate::distributed::error::DistributedErrorKind::DeadlineExceeded
        );
    }

    #[test]
    fn provider_registration_rejects_duplicates() {
        let registry =
            ProviderRegistry::<TestWork, u64>::new();

        registry
            .register(Arc::new(TestProvider {
                id: "local".to_owned(),
                score: 1,
            }))
            .unwrap();

        let error = registry
            .register(Arc::new(TestProvider {
                id: "local".to_owned(),
                score: 2,
            }))
            .unwrap_err();

        assert!(matches!(
            error,
            ProviderRegistryError::DuplicateProvider { .. }
        ));
    }

    #[test]
    fn provider_assessment_can_reject_work() {
        struct RejectingProvider;

        impl ExecutionProvider<TestWork, u64> for RejectingProvider {
            fn provider_id(&self) -> &str {
                "rejecting"
            }

            fn assess(
                &self,
                _work: &TestWork,
            ) -> DistributedResult<ProviderAssessment> {
                Ok(ProviderAssessment::ineligible(
                    "unsupported execution requirements",
                ))
            }

            fn execute(
                &self,
                _work: &TestWork,
                _control: &ExecutionControl,
            ) -> DistributedResult<u64> {
                unreachable!("ineligible providers must never execute")
            }
        }

        let executor = DistributedExecutor::<TestWork, u64>::new();

        executor
            .registry()
            .register(Arc::new(RejectingProvider))
            .unwrap();

        let failure = executor
            .execute(&test_plan(), &ExecutionControl::new())
            .unwrap_err();

        assert_eq!(
            failure.error().kind(),
            crate::distributed::error::DistributedErrorKind::CapabilityUnsatisfied
        );
    }
}
