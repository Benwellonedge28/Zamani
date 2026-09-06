//! Zamani Quantum Resilience — Mitigation Executor
//!
//! Production execution boundary for approved quantum-error mitigation
//! strategies.
//!
//! # Responsibility
//!
//! This module executes an already-selected and authorized mitigation plan
//! through a provider-independent execution pipeline.
//!
//! It deliberately does NOT:
//!
//! - select mitigation strategies;
//! - diagnose faults;
//! - implement QEC;
//! - implement routing;
//! - implement scheduling;
//! - implement optimization;
//! - implement hardware-provider APIs;
//! - implement noise models;
//! - define a second quantum IR;
//! - define a second qubit identity;
//! - silently retry uncertain remote submissions;
//! - accept an execution merely because the backend reported completion.
//!
//! Those responsibilities belong to their owning subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                    canonical Zamani program
//!                              |
//!                              v
//!                       quantum::ir
//!                              |
//!                              v
//!                    mitigation::selection
//!                              |
//!                              v
//!                    approved strategy
//!                              |
//!                              v
//!                 +--------------------------+
//!                 | mitigation::executor     |
//!                 |                          |
//!                 | validate                 |
//!                 | authorize                |
//!                 | prepare                  |
//!                 | submit                   |
//!                 | observe                  |
//!                 | retrieve                |
//!                 | normalize               |
//!                 +------------+-------------+
//!                              |
//!                              v
//!                   normal execution pipeline
//!                              |
//!             +----------------+----------------+
//!             |                                 |
//!             v                                 v
//!       compilation                        hardware HAL
//!       routing                            / simulator
//!       scheduling                              |
//!             |                                 |
//!             +----------------+----------------+
//!                              |
//!                              v
//!                         raw result
//!                              |
//!                              v
//!                     resilience verification
//! ```
//!
//! # Write once, scale everywhere
//!
//! No architectural limit on:
//!
//! - number of logical qubits;
//! - number of physical qubits;
//! - number of mitigation steps;
//! - number of executions;
//! - circuit depth;
//! - number of mitigation variants;
//! - number of result records;
//! - backend count;
//! - distributed execution size.
//!
//! Concrete resource limits belong to:
//!
//! - the execution pipeline;
//! - target capabilities;
//! - caller policy;
//! - security/resource policy;
//! - available memory;
//! - execution budgets;
//! - backend/provider limits.
//!
//! They MUST NOT be encoded here as artificial constants.
//!
//! # Canonical quantum identities
//!
//! Whenever mitigation needs logical or physical qubit identity, this module
//! uses the canonical types:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module MUST NOT define a competing `QubitId`, `LogicalQubitId`, or
//! `PhysicalQubitId`.
//!
//! # Exactly-once safety
//!
//! Remote quantum submission can have an ambiguous outcome:
//!
//! ```text
//! submit()
//!    |
//!    +--> response received       -> known submission
//!    |
//!    +--> transport failure      -> submission may have happened
//! ```
//!
//! The executor therefore never automatically resubmits an ambiguous
//! submission. The execution pipeline must expose an explicit submission
//! outcome and idempotency identity.
//!
//! This is especially important for quantum hardware because an apparently
//! failed client request may correspond to a successfully submitted physical
//! job.
//!
//! # Determinism
//!
//! The executor itself does not create random values and does not read the
//! system clock. Randomization required by a mitigation strategy must be
//! explicitly supplied by the strategy/selection layer through an execution
//! seed or an external deterministic source.
//!
//! # Safety
//!
//! This module forbids unsafe Rust.
//!
//! # Rust
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration
//!
//! `strategy.rs`
//!     Produces an approved mitigation execution plan and strategy metadata.
//!
//! `selection.rs`
//!     Determines whether a strategy is compatible and selected.
//!
//! `readout.rs`
//! `zero_noise.rs`
//! `probabilistic.rs`
//! `twirling.rs`
//! `dynamical_decoupling.rs`
//! `custom.rs`
//!     Implement concrete mitigation semantics and produce execution plans.
//!
//! `quantum::ir`
//!     Supplies the canonical `QuantumCircuit` and canonical qubit identities.
//!
//! `quantum::routing`
//!     Performs logical-to-physical mapping.
//!
//! `quantum::scheduling`
//!     Performs timing and resource scheduling.
//!
//! `quantum::optimization`
//!     Performs semantics-preserving optimization.
//!
//! `quantum::hardware`
//!     Provides the provider-neutral execution boundary.
//!
//! `verification`
//!     Determines whether the resulting execution is semantically acceptable.
//!
//! `telemetry`
//!     Records execution observations.
//!
//! `provenance`
//!     Records mitigation execution provenance.
//!
//! The executor intentionally depends only on the execution-pipeline contract,
//! not on individual hardware providers.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::ir::QuantumCircuit;

/// Stable schema identifier for this executor contract.
pub const MITIGATION_EXECUTOR_SCHEMA_ID: &str =
    "zamani.quantum.resilience.mitigation.executor";

/// Semantic version of the executor contract.
///
/// Increment only when the public execution contract changes incompatibly.
pub const MITIGATION_EXECUTOR_SCHEMA_VERSION: u16 = 1;

/// Immutable mitigation strategy identifier.
///
/// Strategy identifiers are intentionally open-ended. The executor must not
/// contain a closed enum of known mitigation algorithms.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MitigationStrategyId(Arc<str>);

impl MitigationStrategyId {
    /// Creates a validated strategy identifier.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, MitigationExecutorError> {
        let value = value.into();

        if value.is_empty() {
            return Err(MitigationExecutorError::InvalidStrategyId);
        }

        if value.chars().any(char::is_control) {
            return Err(MitigationExecutorError::InvalidStrategyId);
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for MitigationStrategyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("MitigationStrategyId")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for MitigationStrategyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Version of a mitigation strategy.
///
/// The executor treats strategy versions as opaque semantic versions. It does
/// not interpret provider-specific versioning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MitigationStrategyVersion {
    /// Major compatibility version.
    pub major: u32,

    /// Minor feature version.
    pub minor: u32,

    /// Patch version.
    pub patch: u32,
}

impl MitigationStrategyVersion {
    /// Creates a strategy version.
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl fmt::Display for MitigationStrategyVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

/// Logical/physical target information supplied to a mitigation execution.
///
/// The executor never performs logical-to-physical mapping. Mapping must
/// already have been resolved by the routing/execution pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MitigationTarget {
    /// Logical qubits affected by the mitigation.
    ///
    /// Empty means that the mitigation is circuit/global rather than tied to a
    /// particular logical-qubit subset.
    logical_qubits: Arc<[QubitId]>,

    /// Physical qubits affected by the already-resolved execution.
    ///
    /// Empty means that physical placement is not exposed at this layer.
    physical_qubits: Arc<[PhysicalQubitId]>,
}

impl MitigationTarget {
    /// Creates a target from logical and physical qubit identities.
    ///
    /// No assumption is made about the number of qubits.
    pub fn new(
        logical_qubits: impl Into<Arc<[QubitId]>>,
        physical_qubits: impl Into<Arc<[PhysicalQubitId]>>,
    ) -> Self {
        Self {
            logical_qubits: logical_qubits.into(),
            physical_qubits: physical_qubits.into(),
        }
    }

    /// Returns logical qubits associated with the mitigation.
    pub fn logical_qubits(&self) -> &[QubitId] {
        &self.logical_qubits
    }

    /// Returns physical qubits associated with the resolved execution.
    pub fn physical_qubits(&self) -> &[PhysicalQubitId] {
        &self.physical_qubits
    }

    /// Returns whether no explicit qubit subset was supplied.
    pub fn is_global(&self) -> bool {
        self.logical_qubits.is_empty() && self.physical_qubits.is_empty()
    }
}

/// Immutable mitigation execution identifier.
///
/// This is a resilience-owned identifier, not a quantum identity.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MitigationExecutionId(Arc<str>);

impl MitigationExecutionId {
    /// Creates an execution identifier.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, MitigationExecutorError> {
        let value = value.into();

        if value.is_empty() {
            return Err(MitigationExecutorError::InvalidExecutionId);
        }

        if value.chars().any(char::is_control) {
            return Err(MitigationExecutorError::InvalidExecutionId);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for MitigationExecutionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("MitigationExecutionId")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for MitigationExecutionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Opaque authorization supplied by the policy/selection layer.
///
/// The executor does not manufacture authorization and therefore cannot
/// accidentally authorize an unselected strategy.
///
/// A real implementation can wrap a stronger capability/authorization object
/// without changing the executor's execution model.
#[derive(Clone, PartialEq, Eq)]
pub struct MitigationAuthorization {
    strategy_id: MitigationStrategyId,
    execution_id: MitigationExecutionId,
    policy_revision: Arc<str>,
}

impl MitigationAuthorization {
    /// Creates an authorization capability.
    pub fn new(
        strategy_id: MitigationStrategyId,
        execution_id: MitigationExecutionId,
        policy_revision: impl Into<Arc<str>>,
    ) -> Result<Self, MitigationExecutorError> {
        let policy_revision = policy_revision.into();

        if policy_revision.is_empty() {
            return Err(MitigationExecutorError::InvalidPolicyRevision);
        }

        if policy_revision.chars().any(char::is_control) {
            return Err(MitigationExecutorError::InvalidPolicyRevision);
        }

        Ok(Self {
            strategy_id,
            execution_id,
            policy_revision,
        })
    }

    /// Returns the authorized strategy.
    pub fn strategy_id(&self) -> &MitigationStrategyId {
        &self.strategy_id
    }

    /// Returns the authorized execution identity.
    pub fn execution_id(&self) -> &MitigationExecutionId {
        &self.execution_id
    }

    /// Returns the policy revision that authorized the execution.
    pub fn policy_revision(&self) -> &str {
        &self.policy_revision
    }
}

impl fmt::Debug for MitigationAuthorization {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MitigationAuthorization")
            .field("strategy_id", &self.strategy_id)
            .field("execution_id", &self.execution_id)
            .field("policy_revision", &self.policy_revision)
            .finish()
    }
}

/// An immutable execution plan produced by mitigation selection/strategy
/// preparation.
///
/// The executor does not interpret strategy-specific parameters. They remain
/// opaque bytes owned by the strategy implementation.
///
/// This is intentional: adding a new mitigation strategy must not require
/// changing the executor.
#[derive(Clone, PartialEq, Eq)]
pub struct MitigationExecutionPlan {
    /// Strategy identity.
    strategy_id: MitigationStrategyId,

    /// Strategy semantic version.
    strategy_version: MitigationStrategyVersion,

    /// Unique execution identity.
    execution_id: MitigationExecutionId,

    /// Already-resolved mitigation target.
    target: MitigationTarget,

    /// Strategy-specific immutable parameters.
    parameters: Arc<[u8]>,

    /// Optional deterministic seed.
    ///
    /// A missing seed means the strategy does not require an executor-provided
    /// deterministic seed.
    seed: Option<u64>,

    /// Whether the plan requires classical post-processing.
    requires_postprocessing: bool,
}

impl MitigationExecutionPlan {
    /// Creates a validated mitigation execution plan.
    pub fn new(
        strategy_id: MitigationStrategyId,
        strategy_version: MitigationStrategyVersion,
        execution_id: MitigationExecutionId,
        target: MitigationTarget,
        parameters: impl Into<Arc<[u8]>>,
        seed: Option<u64>,
        requires_postprocessing: bool,
    ) -> Self {
        Self {
            strategy_id,
            strategy_version,
            execution_id,
            target,
            parameters: parameters.into(),
            seed,
            requires_postprocessing,
        }
    }

    /// Returns the strategy identity.
    pub fn strategy_id(&self) -> &MitigationStrategyId {
        &self.strategy_id
    }

    /// Returns the strategy version.
    pub const fn strategy_version(&self) -> MitigationStrategyVersion {
        self.strategy_version
    }

    /// Returns the execution identity.
    pub fn execution_id(&self) -> &MitigationExecutionId {
        &self.execution_id
    }

    /// Returns the target.
    pub fn target(&self) -> &MitigationTarget {
        &self.target
    }

    /// Returns opaque strategy parameters.
    pub fn parameters(&self) -> &[u8] {
        &self.parameters
    }

    /// Returns the deterministic seed, when supplied.
    pub const fn seed(&self) -> Option<u64> {
        self.seed
    }

    /// Returns whether classical post-processing is required.
    pub const fn requires_postprocessing(&self) -> bool {
        self.requires_postprocessing
    }
}

impl fmt::Debug for MitigationExecutionPlan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MitigationExecutionPlan")
            .field("strategy_id", &self.strategy_id)
            .field("strategy_version", &self.strategy_version)
            .field("execution_id", &self.execution_id)
            .field("target", &self.target)
            .field("parameter_byte_len", &self.parameters.len())
            .field("seed", &self.seed)
            .field("requires_postprocessing", &self.requires_postprocessing)
            .finish()
    }
}

/// Request supplied to the execution pipeline.
///
/// The circuit is borrowed from canonical IR. The executor does not clone or
/// redefine the circuit.
pub struct MitigationExecutionRequest<'a> {
    /// Canonical semantic quantum circuit.
    pub circuit: &'a QuantumCircuit,

    /// Approved mitigation plan.
    pub plan: &'a MitigationExecutionPlan,

    /// Authorization issued by policy/selection.
    pub authorization: &'a MitigationAuthorization,
}

impl<'a> MitigationExecutionRequest<'a> {
    /// Creates a request.
    pub const fn new(
        circuit: &'a QuantumCircuit,
        plan: &'a MitigationExecutionPlan,
        authorization: &'a MitigationAuthorization,
    ) -> Self {
        Self {
            circuit,
            plan,
            authorization,
        }
    }
}

/// Execution stage reached by a mitigation operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MitigationExecutionStage {
    /// Request validation.
    Validating,

    /// Execution pipeline preparation.
    Preparing,

    /// Compilation/lowering/routing/scheduling.
    Lowering,

    /// Backend submission.
    Submitting,

    /// Backend execution.
    Executing,

    /// Result retrieval.
    Retrieving,

    /// Normalization.
    Normalizing,

    /// Optional classical post-processing.
    PostProcessing,

    /// Complete.
    Completed,
}

impl fmt::Display for MitigationExecutionStage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Validating => "validating",
            Self::Preparing => "preparing",
            Self::Lowering => "lowering",
            Self::Submitting => "submitting",
            Self::Executing => "executing",
            Self::Retrieving => "retrieving",
            Self::Normalizing => "normalizing",
            Self::PostProcessing => "post_processing",
            Self::Completed => "completed",
        };

        formatter.write_str(value)
    }
}

/// Result of mitigation execution.
///
/// The executor deliberately does not declare a quantum-result representation.
/// The normal execution pipeline owns the canonical normalized result type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MitigationExecutionResult {
    /// Execution identity.
    pub execution_id: MitigationExecutionId,

    /// Strategy identity.
    pub strategy_id: MitigationStrategyId,

    /// Strategy version.
    pub strategy_version: MitigationStrategyVersion,

    /// Terminal execution state.
    pub state: MitigationExecutionState,

    /// Normalized provider-neutral result.
    pub result: MitigationResultPayload,

    /// Whether the execution was known to have been submitted.
    pub submission: SubmissionOutcome,

    /// Last completed execution stage.
    pub stage: MitigationExecutionStage,

    /// Whether this result still requires verification before acceptance.
    pub verification_required: bool,
}

/// Provider-neutral mitigation result payload.
///
/// Result bytes belong to the normal execution/result pipeline. The executor
/// does not interpret them as a second quantum-result model.
#[derive(Clone, PartialEq, Eq)]
pub struct MitigationResultPayload {
    format: Arc<str>,
    bytes: Arc<[u8]>,
}

impl MitigationResultPayload {
    /// Creates a result payload.
    pub fn new(
        format: impl Into<Arc<str>>,
        bytes: impl Into<Arc<[u8]>>,
    ) -> Result<Self, MitigationExecutorError> {
        let format = format.into();

        if format.is_empty() {
            return Err(MitigationExecutorError::InvalidResultFormat);
        }

        if format.chars().any(char::is_control) {
            return Err(MitigationExecutorError::InvalidResultFormat);
        }

        Ok(Self {
            format,
            bytes: bytes.into(),
        })
    }

    /// Returns the result format.
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Returns the encoded result.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the result size.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the result is empty.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

impl fmt::Debug for MitigationResultPayload {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("MitigationResultPayload")
            .field("format", &self.format)
            .field("byte_len", &self.bytes.len())
            .finish()
    }
}

/// Terminal state of mitigation execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MitigationExecutionState {
    /// Execution completed and a result is available.
    Completed,

    /// Execution was rejected before submission.
    Rejected,

    /// Execution failed before the submission outcome became ambiguous.
    Failed,

    /// The submission outcome is unknown and must not be automatically
    /// resubmitted.
    SubmissionUnknown,
}

impl MitigationExecutionState {
    /// Returns whether this state is terminal.
    pub const fn is_terminal(self) -> bool {
        true
    }

    /// Returns whether automatic re-submission is safe.
    pub const fn permits_automatic_resubmission(self) -> bool {
        matches!(self, Self::Rejected | Self::Failed)
    }
}

/// Provider-neutral submission outcome.
///
/// This is essential for remote quantum execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SubmissionOutcome {
    /// The execution was never submitted.
    NotSubmitted,

    /// Submission was acknowledged and the execution identity is known.
    Submitted,

    /// The request may have reached the execution target, but confirmation
    /// was lost.
    Unknown,
}

impl SubmissionOutcome {
    /// Returns true when execution is known to have been submitted.
    pub const fn is_submitted(self) -> bool {
        matches!(self, Self::Submitted | Self::Unknown)
    }

    /// Returns true when the outcome is ambiguous.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Prepared execution artifact.
///
/// The execution pipeline owns the actual compiled/scheduled/backend-specific
/// representation.
#[derive(Clone, PartialEq, Eq)]
pub struct PreparedMitigationExecution {
    /// Provider-neutral executable format.
    pub format: Arc<str>,

    /// Opaque executable payload.
    pub payload: Arc<[u8]>,

    /// Deterministic execution fingerprint supplied by the execution
    /// pipeline.
    pub fingerprint: Arc<str>,
}

impl PreparedMitigationExecution {
    /// Creates a prepared execution artifact.
    pub fn new(
        format: impl Into<Arc<str>>,
        payload: impl Into<Arc<[u8]>>,
        fingerprint: impl Into<Arc<str>>,
    ) -> Result<Self, MitigationExecutorError> {
        let format = format.into();
        let fingerprint = fingerprint.into();

        if format.is_empty() || fingerprint.is_empty() {
            return Err(MitigationExecutorError::InvalidPreparedExecution);
        }

        if format.chars().any(char::is_control)
            || fingerprint.chars().any(char::is_control)
        {
            return Err(MitigationExecutorError::InvalidPreparedExecution);
        }

        Ok(Self {
            format,
            payload: payload.into(),
            fingerprint,
        })
    }
}

impl fmt::Debug for PreparedMitigationExecution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PreparedMitigationExecution")
            .field("format", &self.format)
            .field("payload_byte_len", &self.payload.len())
            .field("fingerprint", &self.fingerprint)
            .finish()
    }
}

/// Submission handle returned by the normal execution pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MitigationSubmission {
    /// Provider-neutral execution identifier.
    pub execution_id: MitigationExecutionId,

    /// Provider-neutral job identifier when known.
    pub backend_job_id: Option<Arc<str>>,

    /// Submission outcome.
    pub outcome: SubmissionOutcome,
}

/// Result returned by the normal execution pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineExecutionResult {
    /// Submission outcome.
    pub submission: SubmissionOutcome,

    /// Normalized result, if execution completed successfully.
    pub result: Option<MitigationResultPayload>,

    /// Terminal state.
    pub state: MitigationExecutionState,

    /// Last completed stage.
    pub stage: MitigationExecutionStage,
}

/// Provider-independent execution pipeline used by the mitigation executor.
///
/// Implementations belong outside this module.
///
/// A concrete implementation may internally perform:
///
/// ```text
/// canonical IR
///     ↓
/// optimization
///     ↓
/// routing
///     ↓
/// scheduling
///     ↓
/// compatibility/lowering
///     ↓
/// hardware::QuantumBackendAdapter
///     ↓
/// result normalization
/// ```
///
/// The executor never needs to know which provider performs those operations.
pub trait MitigationExecutionPipeline: Send + Sync {
    /// Prepares the mitigation execution.
    ///
    /// This is where the selected mitigation strategy is lowered into a
    /// provider-neutral executable representation through the normal
    /// compilation/routing/scheduling pipeline.
    fn prepare(
        &self,
        request: &MitigationExecutionRequest<'_>,
    ) -> Result<PreparedMitigationExecution, MitigationExecutorError>;

    /// Submits a prepared mitigation execution.
    ///
    /// Implementations MUST preserve the execution ID as an idempotency key
    /// whenever the underlying backend supports idempotent submission.
    ///
    /// If the transport result is ambiguous, implementations MUST return
    /// `SubmissionOutcome::Unknown` rather than claiming `NotSubmitted`.
    fn submit(
        &self,
        request: &MitigationExecutionRequest<'_>,
        prepared: &PreparedMitigationExecution,
    ) -> Result<MitigationSubmission, MitigationExecutorError>;

    /// Drives an already-submitted mitigation execution to a terminal result.
    ///
    /// This method owns provider-specific asynchronous lifecycle handling.
    ///
    /// It MUST NOT automatically submit the workload again.
    fn collect(
        &self,
        request: &MitigationExecutionRequest<'_>,
        submission: &MitigationSubmission,
    ) -> Result<PipelineExecutionResult, MitigationExecutorError>;
}

/// Optional post-processing hook.
///
/// This belongs to the mitigation strategy implementation, not to the core
/// executor. It is supplied by the caller only when the selected strategy
/// explicitly requires it.
pub trait MitigationPostProcessor: Send + Sync {
    /// Performs deterministic or explicitly controlled classical mitigation
    /// post-processing.
    ///
    /// The processor must not change the canonical quantum program.
    fn process(
        &self,
        request: &MitigationExecutionRequest<'_>,
        result: MitigationResultPayload,
    ) -> Result<MitigationResultPayload, MitigationExecutorError>;
}

/// Production mitigation executor.
///
/// The executor is intentionally stateless.
///
/// This permits:
///
/// - one executor per runtime;
/// - one executor per worker;
/// - concurrent executors;
/// - distributed executors;
/// - simulation;
/// - hardware execution;
/// - deterministic testing.
///
/// No global mutable state is required.
pub struct MitigationExecutor<P> {
    pipeline: Arc<P>,
}

impl<P> MitigationExecutor<P>
where
    P: MitigationExecutionPipeline + 'static,
{
    /// Creates an executor over a provider-independent execution pipeline.
    pub fn new(pipeline: Arc<P>) -> Self {
        Self { pipeline }
    }

    /// Returns a shared reference to the execution pipeline.
    pub fn pipeline(&self) -> &P {
        &self.pipeline
    }

    /// Executes an approved mitigation plan.
    ///
    /// The operation is deliberately single-shot at the executor layer:
    ///
    /// 1. validate;
    /// 2. authorize;
    /// 3. prepare;
    /// 4. submit;
    /// 5. collect;
    /// 6. optionally post-process;
    /// 7. return an unverified result.
    ///
    /// Verification is intentionally outside this module.
    pub fn execute(
        &self,
        request: &MitigationExecutionRequest<'_>,
        post_processor: Option<&dyn MitigationPostProcessor>,
    ) -> Result<MitigationExecutionResult, MitigationExecutorError> {
        self.validate_request(request)?;

        let prepared = self
            .pipeline
            .prepare(request)
            .map_err(|error| error.at_stage(MitigationExecutionStage::Preparing))?;

        let submission = match self.pipeline.submit(request, &prepared) {
            Ok(value) => value,
            Err(error) => {
                return Err(error.at_stage(MitigationExecutionStage::Submitting));
            }
        };

        if submission.outcome.is_unknown() {
            return Err(MitigationExecutorError::SubmissionOutcomeUnknown {
                execution_id: request.plan.execution_id.clone(),
            });
        }

        if !submission.outcome.is_submitted() {
            return Err(MitigationExecutorError::SubmissionNotConfirmed {
                execution_id: request.plan.execution_id.clone(),
            });
        }

        let collected = self
            .pipeline
            .collect(request, &submission)
            .map_err(|error| error.at_stage(MitigationExecutionStage::Retrieving))?;

        if collected.submission.is_unknown() {
            return Err(MitigationExecutorError::SubmissionOutcomeUnknown {
                execution_id: request.plan.execution_id.clone(),
            });
        }

        if collected.state != MitigationExecutionState::Completed {
            return Err(MitigationExecutorError::PipelineDidNotComplete {
                state: collected.state,
            });
        }

        let result = collected
            .result
            .ok_or(MitigationExecutorError::CompletedWithoutResult)?;

        let result = if request.plan.requires_postprocessing() {
            let processor = post_processor.ok_or(
                MitigationExecutorError::PostProcessorRequired,
            )?;

            processor
                .process(request, result)
                .map_err(|error| error.at_stage(MitigationExecutionStage::PostProcessing))?
        } else {
            result
        };

        Ok(MitigationExecutionResult {
            execution_id: request.plan.execution_id.clone(),
            strategy_id: request.plan.strategy_id.clone(),
            strategy_version: request.plan.strategy_version,
            state: MitigationExecutionState::Completed,
            result,
            submission: collected.submission,
            stage: if request.plan.requires_postprocessing() {
                MitigationExecutionStage::PostProcessing
            } else {
                MitigationExecutionStage::Normalizing
            },
            verification_required: true,
        })
    }

    /// Validates the execution request before any preparation or submission.
    ///
    /// This is intentionally strict because an invalid mitigation must never
    /// reach a real quantum backend.
    fn validate_request(
        &self,
        request: &MitigationExecutionRequest<'_>,
    ) -> Result<(), MitigationExecutorError> {
        if request.plan.strategy_id != request.authorization.strategy_id {
            return Err(MitigationExecutorError::AuthorizationStrategyMismatch {
                expected: request.plan.strategy_id.clone(),
                actual: request.authorization.strategy_id.clone(),
            });
        }

        if request.plan.execution_id != request.authorization.execution_id {
            return Err(
                MitigationExecutorError::AuthorizationExecutionMismatch {
                    expected: request.plan.execution_id.clone(),
                    actual: request.authorization.execution_id.clone(),
                },
            );
        }

        if request.plan.strategy_id.as_str().is_empty() {
            return Err(MitigationExecutorError::InvalidStrategyId);
        }

        if request.plan.execution_id.as_str().is_empty() {
            return Err(MitigationExecutorError::InvalidExecutionId);
        }

        validate_qubit_identity_sets(request.plan.target())?;

        Ok(())
    }
}

/// Validates canonical logical and physical qubit sets.
///
/// This does not impose a maximum number of qubits.
///
/// Duplicate identities are rejected because an explicitly-targeted
/// mitigation must not accidentally contain ambiguous repeated resource
/// references.
fn validate_qubit_identity_sets(
    target: &MitigationTarget,
) -> Result<(), MitigationExecutorError> {
    if contains_duplicate(target.logical_qubits()) {
        return Err(MitigationExecutorError::DuplicateLogicalQubit);
    }

    if contains_duplicate(target.physical_qubits()) {
        return Err(MitigationExecutorError::DuplicatePhysicalQubit);
    }

    Ok(())
}

fn contains_duplicate<T>(values: &[T]) -> bool
where
    T: Eq,
{
    for index in 0..values.len() {
        if values[index + 1..].iter().any(|value| value == &values[index]) {
            return true;
        }
    }

    false
}

/// Errors produced by the mitigation executor.
///
/// The error model deliberately distinguishes ambiguous submission from an
/// ordinary execution failure because automatic retry is unsafe in the former
/// case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MitigationExecutorError {
    /// Strategy identifier is missing or malformed.
    InvalidStrategyId,

    /// Execution identifier is missing or malformed.
    InvalidExecutionId,

    /// Policy revision is missing or malformed.
    InvalidPolicyRevision,

    /// Result format is missing or malformed.
    InvalidResultFormat,

    /// Prepared execution artifact is invalid.
    InvalidPreparedExecution,

    /// Logical qubit identity occurs more than once in the target.
    DuplicateLogicalQubit,

    /// Physical qubit identity occurs more than once in the target.
    DuplicatePhysicalQubit,

    /// The authorization was issued for a different strategy.
    AuthorizationStrategyMismatch {
        /// Expected strategy.
        expected: MitigationStrategyId,

        /// Authorized strategy.
        actual: MitigationStrategyId,
    },

    /// The authorization was issued for a different execution.
    AuthorizationExecutionMismatch {
        /// Expected execution.
        expected: MitigationExecutionId,

        /// Authorized execution.
        actual: MitigationExecutionId,
    },

    /// The remote submission outcome is ambiguous.
    ///
    /// This is deliberately non-retryable at this layer.
    SubmissionOutcomeUnknown {
        /// Execution whose outcome is uncertain.
        execution_id: MitigationExecutionId,
    },

    /// The pipeline returned without confirming submission.
    SubmissionNotConfirmed {
        /// Execution identity.
        execution_id: MitigationExecutionId,
    },

    /// The pipeline reached a non-completed terminal state.
    PipelineDidNotComplete {
        /// Actual terminal state.
        state: MitigationExecutionState,
    },

    /// Backend reported completion without returning a normalized result.
    CompletedWithoutResult,

    /// The selected strategy requires classical post-processing but no
    /// processor was supplied.
    PostProcessorRequired,

    /// Failure originating from an execution stage.
    Pipeline {
        /// Stage where the failure occurred.
        stage: MitigationExecutionStage,

        /// Stable textual reason.
        reason: Arc<str>,
    },
}

impl MitigationExecutorError {
    /// Attaches the execution stage to a pipeline error.
    ///
    /// Existing pipeline errors retain their original reason while acquiring
    /// the stage at which the executor observed them.
    pub fn at_stage(self, stage: MitigationExecutionStage) -> Self {
        match self {
            Self::Pipeline { reason, .. } => Self::Pipeline { stage, reason },
            other => other,
        }
    }

    /// Returns the execution stage associated with the error, if any.
    pub const fn stage(&self) -> Option<MitigationExecutionStage> {
        match self {
            Self::Pipeline { stage, .. } => Some(*stage),

            Self::InvalidStrategyId
            | Self::InvalidExecutionId
            | Self::InvalidPolicyRevision
            | Self::InvalidResultFormat
            | Self::InvalidPreparedExecution
            | Self::DuplicateLogicalQubit
            | Self::DuplicatePhysicalQubit
            | Self::AuthorizationStrategyMismatch { .. }
            | Self::AuthorizationExecutionMismatch { .. }
            | Self::SubmissionOutcomeUnknown { .. }
            | Self::SubmissionNotConfirmed { .. }
            | Self::PipelineDidNotComplete { .. }
            | Self::CompletedWithoutResult
            | Self::PostProcessorRequired => None,
        }
    }

    /// Returns whether automatic resubmission is forbidden.
    ///
    /// Ambiguous submissions always return true.
    pub const fn forbids_automatic_resubmission(&self) -> bool {
        matches!(
            self,
            Self::SubmissionOutcomeUnknown { .. }
                | Self::SubmissionNotConfirmed { .. }
        )
    }
}

impl fmt::Display for MitigationExecutorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStrategyId => {
                formatter.write_str("invalid mitigation strategy identifier")
            }

            Self::InvalidExecutionId => {
                formatter.write_str("invalid mitigation execution identifier")
            }

            Self::InvalidPolicyRevision => {
                formatter.write_str("invalid mitigation policy revision")
            }

            Self::InvalidResultFormat => {
                formatter.write_str("invalid mitigation result format")
            }

            Self::InvalidPreparedExecution => {
                formatter.write_str("invalid prepared mitigation execution")
            }

            Self::DuplicateLogicalQubit => {
                formatter.write_str("duplicate logical qubit in mitigation target")
            }

            Self::DuplicatePhysicalQubit => {
                formatter.write_str("duplicate physical qubit in mitigation target")
            }

            Self::AuthorizationStrategyMismatch { expected, actual } => {
                write!(
                    formatter,
                    "mitigation authorization strategy mismatch: expected {}, authorized {}",
                    expected, actual
                )
            }

            Self::AuthorizationExecutionMismatch { expected, actual } => {
                write!(
                    formatter,
                    "mitigation authorization execution mismatch: expected {}, authorized {}",
                    expected, actual
                )
            }

            Self::SubmissionOutcomeUnknown { execution_id } => {
                write!(
                    formatter,
                    "mitigation submission outcome is unknown for execution {}",
                    execution_id
                )
            }

            Self::SubmissionNotConfirmed { execution_id } => {
                write!(
                    formatter,
                    "mitigation submission was not confirmed for execution {}",
                    execution_id
                )
            }

            Self::PipelineDidNotComplete { state } => {
                write!(
                    formatter,
                    "mitigation execution did not complete: {}",
                    state
                )
            }

            Self::CompletedWithoutResult => {
                formatter.write_str(
                    "mitigation execution completed without a normalized result",
                )
            }

            Self::PostProcessorRequired => {
                formatter.write_str(
                    "mitigation strategy requires classical post-processing",
                )
            }

            Self::Pipeline { stage, reason } => {
                write!(
                    formatter,
                    "mitigation execution pipeline failure at {}: {}",
                    stage, reason
                )
            }
        }
    }
}

impl std::error::Error for MitigationExecutorError {}

/// Convenience constructor for pipeline errors.
///
/// Keeping this helper outside the execution implementation makes adapter
/// implementations less verbose while retaining the same stable error type.
pub fn pipeline_error(
    stage: MitigationExecutionStage,
    reason: impl Into<Arc<str>>,
) -> MitigationExecutorError {
    MitigationExecutorError::Pipeline {
        stage,
        reason: reason.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strategy_identifier_rejects_empty_value() {
        assert!(matches!(
            MitigationStrategyId::new(""),
            Err(MitigationExecutorError::InvalidStrategyId)
        ));
    }

    #[test]
    fn execution_identifier_rejects_empty_value() {
        assert!(matches!(
            MitigationExecutionId::new(""),
            Err(MitigationExecutorError::InvalidExecutionId)
        ));
    }

    #[test]
    fn target_is_global_when_no_qubits_are_supplied() {
        let target = MitigationTarget::new(
            Arc::<[QubitId]>::from([]),
            Arc::<[PhysicalQubitId]>::from([]),
        );

        assert!(target.is_global());
    }

    #[test]
    fn submission_unknown_forbids_automatic_retry() {
        let id = MitigationExecutionId::new("execution-1").unwrap();

        let error = MitigationExecutorError::SubmissionOutcomeUnknown {
            execution_id: id,
        };

        assert!(error.forbids_automatic_resubmission());
    }

    #[test]
    fn completed_state_is_terminal() {
        assert!(MitigationExecutionState::Completed.is_terminal());
    }

    #[test]
    fn unknown_submission_is_not_confirmed() {
        assert!(SubmissionOutcome::Unknown.is_unknown());
        assert!(SubmissionOutcome::Unknown.is_submitted());
    }

    #[test]
    fn result_payload_does_not_expose_bytes_in_debug() {
        let payload =
            MitigationResultPayload::new("counts", vec![1_u8, 2_u8, 3_u8]).unwrap();

        let debug = format!("{payload:?}");

        assert!(debug.contains("byte_len"));
        assert!(!debug.contains("[1, 2, 3]"));
    }

    #[test]
    fn prepared_execution_does_not_expose_payload_in_debug() {
        let prepared = PreparedMitigationExecution::new(
            "zamani-ir",
            vec![1_u8, 2_u8, 3_u8],
            "fingerprint",
        )
        .unwrap();

        let debug = format!("{prepared:?}");

        assert!(debug.contains("payload_byte_len"));
        assert!(!debug.contains("[1, 2, 3]"));
    }
}