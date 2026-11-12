
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Execution Contracts
//!
//! Path: `src/zur/contracts/execution.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! Defines the ZUR-level contract for submitting, observing and cancelling
//! executions of already-compiled Zamani artifacts.
//!
//! This module provides the boundary between a validated executable artifact
//! and an execution engine. It does not implement the compiler, select a
//! backend, allocate physical resources, schedule domain operations, or
//! implement a hardware provider.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Opaque references to externally owned executable artifacts.
//! - Opaque references to externally owned execution plans.
//! - Validated execution requests.
//! - Execution handles.
//! - Execution lifecycle states and transition validation.
//! - Execution observations.
//! - Cancellation dispositions.
//! - The asynchronous execution-engine interface.
//!
//! It does not own:
//!
//! - Source-language semantics, types, effects or requirements.
//! - Capability definitions or capability evidence.
//! - Feasibility evaluation or resource reservation.
//! - Compiler plans or planning algorithms.
//! - Artifact manifests, artifact storage or artifact construction.
//! - Runtime ABI definitions.
//! - Provider identity, provider registration or provider discovery.
//! - Provider operation payloads or provider-specific sessions.
//! - Classical, quantum, HDL, AI, tensor or other domain IR.
//! - Hybrid or distributed execution planning.
//! - Physical hardware discovery, allocation or scheduling.
//! - Authentication, authorization or trust decisions.
//!
//! Those responsibilities remain with their existing repository owners.
//!
//! ## Important distinctions
//!
//! - Artifact reference != artifact integrity.
//! - Plan reference != plan validation.
//! - Execution request != feasibility.
//! - Execution submission != execution completion.
//! - Cancellation request != confirmed cancellation.
//! - Provider registration != provider availability.
//! - Capability evidence != resource reservation.
//! - Runtime execution != source-language semantics.
//!
//! ## Scalability
//!
//! No universal maximum is imposed on:
//!
//! - Executions.
//! - Sessions.
//! - Artifacts.
//! - Plan records.
//! - Execution references.
//! - Providers.
//! - Computational domains.
//! - Devices.
//! - Operations.
//! - Resource quantities.
//!
//! Collections and identifiers are bounded only by actual implementation,
//! address-space, allocator and platform constraints.
//!
//! This is not a promise of literal infinite physical computation.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No raw pointers.
//! No unsafe implementations.
//! No global mutable state.
//! No fabricated execution success.
//! No implicit resource acquisition.
//! No implicit provider selection.
//! No arbitrary hardware or domain ceilings.
//!
//! ## Integration
//!
//! - `abi.rs` owns `RuntimeAbi`, entry-point descriptors and invocation modes.
//! - `provider.rs` owns provider execution operations, payloads, sessions,
//!   streaming, provider submission and provider-specific outcomes.
//! - `service.rs` owns runtime service contracts.
//! - `version.rs` owns ZUR version identities and compatibility policies.
//! - `ids.rs` owns `ExecutionId` and `SessionId`.
//! - `error.rs` owns `ZurError` and `ZurResult`.
//! - `stdlib/context.rs` supplies the immutable operation context and
//!   cooperative cancellation observation.
//! - `compiler/artifacts/` owns canonical compiler artifacts.
//! - `backend/manifest.rs` owns backend artifact manifests.
//! - `compiler/` owns compilation and execution-planning inputs.
//! - `hybrid/`, `distributed/`, `quantum/`, `hdl/` and `ai/` retain their
//!   respective computational authorities.
//!
//! This module defines an execution boundary, not an execution implementation.

use core::fmt;
use core::future::Future;
use core::pin::Pin;

use crate::stdlib::context::OperationContext;

use crate::zur::contracts::abi::{
    AbiSymbol,
    InvocationMode,
    RuntimeAbi,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::ids::{
    ExecutionId,
    SessionId,
};

// =============================================================================
// ASYNCHRONOUS EXECUTION CONTRACT
// =============================================================================

/// Future returned by an asynchronous ZUR execution operation.
///
/// The future is tied to the lifetime of the engine, request, handle or
/// operation context supplied to the corresponding method.
///
/// `Send` permits compatible execution engines to drive operations through
/// multithreaded executors.
///
/// A single-threaded implementation can provide an appropriate adapter.
pub type ExecutionFuture<'a, T> =
    Pin<Box<dyn Future<Output = ZurResult<T>> + Send + 'a>>;

// =============================================================================
// EXTERNAL EXECUTION REFERENCES
// =============================================================================

/// Validates an opaque external reference component.
///
/// Authority identifiers use the same open, namespaced identifier
/// convention as ZUR provider interface identifiers.
///
/// Record identifiers are opaque. They may contain identifiers generated
/// by an external artifact or planning authority, but cannot be empty,
/// have surrounding whitespace or contain control characters.
fn validate_reference_component(
    value: &str,
    description: &str,
) -> ZurResult<()> {
    if value.is_empty() {
        return Err(
            ZurError::invalid_input(
                format!("{description} must not be empty"),
            )
            .with_component("zur.contracts.execution"),
        );
    }

    if value.trim() != value {
        return Err(
            ZurError::invalid_input(
                format!(
                    "{description} must not contain surrounding whitespace"
                ),
            )
            .with_component("zur.contracts.execution"),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ZurError::invalid_input(
                format!(
                    "{description} must not contain control characters"
                ),
            )
            .with_component("zur.contracts.execution"),
        );
    }

    Ok(())
}

/// Validates a namespaced authority identifier.
///
/// This is deliberately independent of computational-domain identity.
/// A future artifact or planning authority can use its own namespace.
fn validate_authority(
    value: &str,
) -> ZurResult<()> {
    validate_reference_component(value, "reference authority")?;

    for segment in value.split('.') {
        if segment.is_empty() {
            return Err(
                ZurError::invalid_input(
                    "reference authority contains an empty namespace segment",
                )
                .with_component("zur.contracts.execution"),
            );
        }

        if !segment.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'-'
                || byte == b'_'
        }) {
            return Err(
                ZurError::invalid_input(
                    "reference authority contains an invalid character",
                )
                .with_component("zur.contracts.execution"),
            );
        }

        if segment.starts_with('-')
            || segment.starts_with('_')
            || segment.ends_with('-')
            || segment.ends_with('_')
        {
            return Err(
                ZurError::invalid_input(
                    "reference authority has an invalid segment boundary",
                )
                .with_component("zur.contracts.execution"),
            );
        }
    }

    if value.split('.').count() < 2 {
        return Err(
            ZurError::invalid_input(
                "reference authority must contain at least two namespace segments",
            )
            .with_component("zur.contracts.execution"),
        );
    }

    Ok(())
}

/// An opaque reference to a record owned by another subsystem.
///
/// The reference identifies an external record. It does not load, validate,
/// authenticate, interpret or execute that record.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExecutionReference {
    authority: String,
    record: String,
}

impl ExecutionReference {
    /// Creates a validated external record reference.
    pub fn new(
        authority: impl Into<String>,
        record: impl Into<String>,
    ) -> ZurResult<Self> {
        let authority = authority.into();
        let record = record.into();

        validate_authority(&authority)?;
        validate_reference_component(&record, "reference record")?;

        Ok(Self {
            authority,
            record,
        })
    }

    /// Returns the owning authority identifier.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Returns the opaque external record identifier.
    #[must_use]
    pub fn record(&self) -> &str {
        &self.record
    }
}

impl fmt::Display for ExecutionReference {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}:{}",
            self.authority,
            self.record,
        )
    }
}

// =============================================================================
// ARTIFACT AND PLAN REFERENCES
// =============================================================================

/// Reference to an executable artifact owned by the compiler/artifact
/// subsystem.
///
/// This type does not duplicate `Artifact`, `ArtifactManifest`,
/// artifact identity allocation, artifact storage or artifact validation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactReference(ExecutionReference);

impl ArtifactReference {
    /// Creates a reference to an externally owned executable artifact.
    pub fn new(
        authority: impl Into<String>,
        record: impl Into<String>,
    ) -> ZurResult<Self> {
        Ok(Self(
            ExecutionReference::new(authority, record)?,
        ))
    }

    /// Returns the underlying opaque artifact reference.
    #[must_use]
    pub fn reference(&self) -> &ExecutionReference {
        &self.0
    }
}

/// Reference to a validated execution plan owned by the compiler,
/// hybrid, distributed or domain-specific planning authority.
///
/// The reference is not the plan itself and does not establish that the
/// plan is feasible, current, compatible or executable.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExecutionPlanReference(ExecutionReference);

impl ExecutionPlanReference {
    /// Creates a reference to an externally owned execution plan.
    pub fn new(
        authority: impl Into<String>,
        record: impl Into<String>,
    ) -> ZurResult<Self> {
        Ok(Self(
            ExecutionReference::new(authority, record)?,
        ))
    }

    /// Returns the underlying opaque plan reference.
    #[must_use]
    pub fn reference(&self) -> &ExecutionReference {
        &self.0
    }
}

// =============================================================================
// EXECUTION REQUEST
// =============================================================================

/// A validated request to execute an already-compiled artifact.
///
/// The request references:
///
/// - A canonical executable artifact.
/// - A canonical execution plan.
/// - An entry point declared by the artifact's runtime ABI.
/// - A ZUR execution identity.
/// - An optional session identity.
///
/// The request does not contain a second copy of the artifact manifest,
/// compiler plan, source-language semantic model or domain IR.
///
/// The supplied `OperationContext` is immutable and provides descriptive
/// operation metadata and cooperative cancellation observation.
///
/// Constructing this value validates only its internal structural
/// consistency. It does not establish artifact integrity, plan validity,
/// provider availability, authorization, feasibility or resource reservation.
pub struct ExecutionRequest<'a> {
    execution_id: ExecutionId,
    artifact: ArtifactReference,
    plan: ExecutionPlanReference,
    abi_version: crate::zur::version::RuntimeAbiVersion,
    entry_point: &'a crate::zur::contracts::abi::EntryPointDescriptor,
    invocation_mode: InvocationMode,
    session_id: Option<SessionId>,
    context: OperationContext,
}

impl<'a> ExecutionRequest<'a> {
    /// Creates an execution request and validates its ABI entry point.
    ///
    /// The entry-point symbol must exist in the supplied canonical runtime
    /// ABI. The invocation mode is obtained from that entry-point descriptor;
    /// callers cannot supply a contradictory invocation mode.
    ///
    /// Session invocation requires a session identity. Other invocation
    /// modes do not acquire or reserve a session implicitly.
    pub fn new(
        execution_id: ExecutionId,
        artifact: ArtifactReference,
        plan: ExecutionPlanReference,
        abi: &'a RuntimeAbi,
        entry_point_symbol: &AbiSymbol,
        session_id: Option<SessionId>,
        context: OperationContext,
    ) -> ZurResult<Self> {
        let entry_point = abi
            .entry_point(entry_point_symbol)
            .ok_or_else(|| {
                ZurError::invalid_input(
                    format!(
                        "entry point `{entry_point_symbol}` is not declared by the supplied runtime ABI"
                    ),
                )
                .with_component("zur.contracts.execution")
            })?;

        let invocation_mode = entry_point.invocation_mode();

        if invocation_mode == InvocationMode::Session
            && session_id.is_none()
        {
            return Err(
                ZurError::invalid_input(
                    "session invocation requires an existing ZUR session identity",
                )
                .with_component("zur.contracts.execution"),
            );
        }

        if context.is_cancelled() {
            return Err(
                ZurError::invalid_input(
                    "cannot construct an execution request with an already-cancelled operation context",
                )
                .with_component("zur.contracts.execution"),
            );
        }

        Ok(Self {
            execution_id,
            artifact,
            plan,
            abi_version: abi.version().clone(),
            entry_point,
            invocation_mode,
            session_id,
            context,
        })
    }

    /// Returns the ZUR execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the executable artifact reference.
    #[must_use]
    pub fn artifact(&self) -> &ArtifactReference {
        &self.artifact
    }

    /// Returns the canonical execution-plan reference.
    #[must_use]
    pub fn plan(&self) -> &ExecutionPlanReference {
        &self.plan
    }

    /// Returns the ABI version declared by the supplied runtime ABI.
    #[must_use]
    pub fn abi_version(
        &self,
    ) -> &crate::zur::version::RuntimeAbiVersion {
        &self.abi_version
    }

    /// Returns the validated ABI entry-point descriptor.
    #[must_use]
    pub fn entry_point(
        &self,
    ) -> &crate::zur::contracts::abi::EntryPointDescriptor {
        self.entry_point
    }

    /// Returns the ABI invocation mode.
    #[must_use]
    pub const fn invocation_mode(&self) -> InvocationMode {
        self.invocation_mode
    }

    /// Returns the optional ZUR session identity.
    #[must_use]
    pub fn session_id(&self) -> Option<&SessionId> {
        self.session_id.as_ref()
    }

    /// Returns the immutable operation context.
    #[must_use]
    pub fn context(&self) -> &OperationContext {
        &self.context
    }
}

// =============================================================================
// EXECUTION LIFECYCLE
// =============================================================================

/// Observable state of one ZUR-managed execution.
///
/// These states describe execution lifecycle, not computational domains.
/// A provider may have additional internal states, but the ZUR adapter must
/// map them honestly to this lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ExecutionStatus {
    /// The request has been constructed but not yet validated by an engine.
    Created,

    /// The engine is validating the artifact, plan and execution contract.
    Validating,

    /// Validation succeeded and the execution is prepared.
    Prepared,

    /// The engine has submitted the execution to an execution authority.
    Submitted,

    /// The execution is actively running.
    Running,

    /// The execution is producing a stream of results.
    Streaming,

    /// Cancellation has been requested but is not yet confirmed.
    Cancelling,

    /// The execution completed successfully.
    Completed,

    /// The execution completed with a valid partial result.
    PartiallyCompleted,

    /// Cancellation was confirmed by the execution authority.
    Cancelled,

    /// The execution timed out.
    TimedOut,

    /// The execution failed.
    Failed,
}

impl ExecutionStatus {
    /// Returns whether this status is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed
                | Self::PartiallyCompleted
                | Self::Cancelled
                | Self::TimedOut
                | Self::Failed
        )
    }

    /// Returns whether this status represents successful full completion.
    #[must_use]
    pub const fn is_successful(self) -> bool {
        matches!(self, Self::Completed)
    }
}

/// A validated execution lifecycle state.
///
/// This value validates local state transitions. It does not control a
/// provider, stop a process, release a resource or prove remote state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExecutionState {
    status: ExecutionStatus,
}

impl ExecutionState {
    /// Creates the initial state for a new execution.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            status: ExecutionStatus::Created,
        }
    }

    /// Returns the current lifecycle state.
    #[must_use]
    pub const fn status(&self) -> ExecutionStatus {
        self.status
    }

    /// Returns whether the execution has reached a terminal state.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.status.is_terminal()
    }

    /// Attempts a validated lifecycle transition.
    ///
    /// Invalid transitions return a structured error. Terminal states
    /// cannot be reopened.
    pub fn transition(
        &mut self,
        next: ExecutionStatus,
    ) -> ZurResult<()> {
        if self.status.is_terminal() {
            return Err(
                ZurError::invalid_input(
                    "a terminal execution state cannot transition",
                )
                .with_component("zur.contracts.execution"),
            );
        }

        let allowed = match self.status {
            ExecutionStatus::Created => {
                matches!(
                    next,
                    ExecutionStatus::Validating
                        | ExecutionStatus::Failed
                        | ExecutionStatus::Cancelled
                )
            }

            ExecutionStatus::Validating => {
                matches!(
                    next,
                    ExecutionStatus::Prepared
                        | ExecutionStatus::Failed
                        | ExecutionStatus::Cancelled
                )
            }

            ExecutionStatus::Prepared => {
                matches!(
                    next,
                    ExecutionStatus::Submitted
                        | ExecutionStatus::Running
                        | ExecutionStatus::Failed
                        | ExecutionStatus::Cancelled
                )
            }

            ExecutionStatus::Submitted => {
                matches!(
                    next,
                    ExecutionStatus::Running
                        | ExecutionStatus::Streaming
                        | ExecutionStatus::Cancelling
                        | ExecutionStatus::TimedOut
                        | ExecutionStatus::Failed
                        | ExecutionStatus::Cancelled
                )
            }

            ExecutionStatus::Running => {
                matches!(
                    next,
                    ExecutionStatus::Streaming
                        | ExecutionStatus::Cancelling
                        | ExecutionStatus::Completed
                        | ExecutionStatus::PartiallyCompleted
                        | ExecutionStatus::TimedOut
                        | ExecutionStatus::Failed
                        | ExecutionStatus::Cancelled
                )
            }

            ExecutionStatus::Streaming => {
                matches!(
                    next,
                    ExecutionStatus::Running
                        | ExecutionStatus::Cancelling
                        | ExecutionStatus::Completed
                        | ExecutionStatus::PartiallyCompleted
                        | ExecutionStatus::TimedOut
                        | ExecutionStatus::Failed
                        | ExecutionStatus::Cancelled
                )
            }

            ExecutionStatus::Cancelling => {
                matches!(
                    next,
                    ExecutionStatus::Completed
                        | ExecutionStatus::PartiallyCompleted
                        | ExecutionStatus::Cancelled
                        | ExecutionStatus::TimedOut
                        | ExecutionStatus::Failed
                )
            }

            ExecutionStatus::Completed
            | ExecutionStatus::PartiallyCompleted
            | ExecutionStatus::Cancelled
            | ExecutionStatus::TimedOut
            | ExecutionStatus::Failed => false,
        };

        if !allowed {
            return Err(
                ZurError::invalid_input(
                    format!(
                        "invalid execution transition: {:?} -> {:?}",
                        self.status,
                        next,
                    ),
                )
                .with_component("zur.contracts.execution"),
            );
        }

        self.status = next;

        Ok(())
    }
}

impl Default for ExecutionState {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// EXECUTION HANDLE
// =============================================================================

/// Handle for an execution whose submission was explicitly accepted.
///
/// A handle identifies a ZUR execution and its original artifact and plan
/// references. It does not imply that execution has completed or that a
/// physical resource remains available.
///
/// `provider_reference` is opaque provider-side correlation data. It must
/// not be interpreted as a device identity, reservation or credential.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionHandle {
    execution_id: ExecutionId,
    artifact: ArtifactReference,
    plan: ExecutionPlanReference,
    provider_reference: Option<String>,
}

impl ExecutionHandle {
    /// Creates a handle after an execution authority has explicitly accepted
    /// the corresponding submission.
    ///
    /// The caller must not construct this handle merely because a request
    /// was transmitted or because a provider was registered.
    pub fn new(
        execution_id: ExecutionId,
        artifact: ArtifactReference,
        plan: ExecutionPlanReference,
        provider_reference: Option<String>,
    ) -> ZurResult<Self> {
        if let Some(reference) = &provider_reference {
            validate_reference_component(
                reference,
                "provider execution reference",
            )?;
        }

        Ok(Self {
            execution_id,
            artifact,
            plan,
            provider_reference,
        })
    }

    /// Returns the ZUR execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the original artifact reference.
    #[must_use]
    pub fn artifact(&self) -> &ArtifactReference {
        &self.artifact
    }

    /// Returns the original plan reference.
    #[must_use]
    pub fn plan(&self) -> &ExecutionPlanReference {
        &self.plan
    }

    /// Returns the optional opaque provider-side correlation reference.
    #[must_use]
    pub fn provider_reference(&self) -> Option<&str> {
        self.provider_reference.as_deref()
    }
}

// =============================================================================
// EXECUTION OBSERVATION
// =============================================================================

/// An observation of execution state reported by an execution authority.
///
/// The observation is a snapshot. It may become stale immediately after
/// it is returned.
///
/// Result and failure references are opaque references to records owned
/// by their respective authorities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionObservation {
    execution_id: ExecutionId,
    state: ExecutionState,
    result: Option<ExecutionReference>,
    failure: Option<ExecutionReference>,
}

impl ExecutionObservation {
    /// Creates an execution observation.
    ///
    /// A successful full or partial result may have a result reference.
    /// A failed execution may have a failure reference.
    ///
    /// This constructor does not independently authenticate the observation.
    pub fn new(
        execution_id: ExecutionId,
        state: ExecutionState,
        result: Option<ExecutionReference>,
        failure: Option<ExecutionReference>,
    ) -> ZurResult<Self> {
        if result.is_some() && failure.is_some() {
            return Err(
                ZurError::invalid_input(
                    "an execution observation cannot identify both a result and a failure record",
                )
                .with_component("zur.contracts.execution"),
            );
        }

        if failure.is_some()
            && state.status() != ExecutionStatus::Failed
        {
            return Err(
                ZurError::invalid_input(
                    "a failure reference requires a failed execution state",
                )
                .with_component("zur.contracts.execution"),
            );
        }

        if result.is_some()
            && !matches!(
                state.status(),
                ExecutionStatus::Completed
                    | ExecutionStatus::PartiallyCompleted
            )
        {
            return Err(
                ZurError::invalid_input(
                    "a result reference requires a completed or partially completed execution state",
                )
                .with_component("zur.contracts.execution"),
            );
        }

        Ok(Self {
            execution_id,
            state,
            result,
            failure,
        })
    }

    /// Returns the observed execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the observed lifecycle state.
    #[must_use]
    pub const fn state(&self) -> ExecutionState {
        self.state
    }

    /// Returns the optional result-record reference.
    #[must_use]
    pub fn result(&self) -> Option<&ExecutionReference> {
        self.result.as_ref()
    }

    /// Returns the optional failure-record reference.
    #[must_use]
    pub fn failure(&self) -> Option<&ExecutionReference> {
        self.failure.as_ref()
    }
}

// =============================================================================
// CANCELLATION
// =============================================================================

/// Actual cancellation disposition reported by the execution authority.
///
/// A cancellation request being accepted is not proof that execution has
/// stopped. Only `Confirmed` represents an explicit confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CancellationDisposition {
    /// The cancellation request was accepted but is not yet confirmed.
    Requested,

    /// The execution authority confirmed that execution was cancelled.
    Confirmed,

    /// The execution had already reached a terminal state.
    AlreadyTerminal,

    /// The execution authority rejected the cancellation request.
    Rejected,

    /// The cancellation outcome could not be established.
    Unknown,
}

impl CancellationDisposition {
    /// Returns whether cancellation was explicitly confirmed.
    #[must_use]
    pub const fn is_confirmed(self) -> bool {
        matches!(self, Self::Confirmed)
    }
}

// =============================================================================
// EXECUTION ENGINE
// =============================================================================

/// ZUR execution-engine contract.
///
/// An execution engine accepts validated requests and exposes the
/// observable lifecycle of the resulting executions.
///
/// Implementations may coordinate native execution, runtime services,
/// domain providers, hybrid execution or distributed execution.
///
/// This trait does not define provider selection, target selection,
/// compiler planning, physical resource allocation or domain semantics.
///
/// Implementations must:
///
/// - Validate the artifact and its integrity through the artifact authority.
/// - Validate the referenced plan through its owning authority.
/// - Check runtime ABI compatibility using an explicit compatibility policy.
/// - Check entry-point and invocation compatibility.
/// - Enforce authorization through the security authority.
/// - Check actual provider and service availability.
/// - Acquire resources only through the canonical resource authority.
/// - Propagate cancellation cooperatively.
/// - Return structured errors for rejected or indeterminate submissions.
/// - Report observed execution state rather than predicted success.
/// - Preserve partial results and failure information.
/// - Release owned resources through their actual owning authorities.
///
/// An engine must not report a successful submission unless acceptance is
/// established. It must not report successful completion merely because
/// submission succeeded.
pub trait ExecutionEngine: Send + Sync {
    /// Submits an execution request.
    ///
    /// A returned handle means the execution authority explicitly accepted
    /// the submission. It does not mean the computation has completed.
    ///
    /// Rejected or indeterminate submissions must return a structured error.
    fn submit<'a>(
        &'a self,
        request: ExecutionRequest<'a>,
    ) -> ExecutionFuture<'a, ExecutionHandle>;

    /// Obtains an observation of an accepted execution.
    ///
    /// The observation describes the state established by the execution
    /// authority at the time of observation.
    fn observe<'a>(
        &'a self,
        handle: &'a ExecutionHandle,
        context: &'a OperationContext,
    ) -> ExecutionFuture<'a, ExecutionObservation>;

    /// Requests cancellation of an accepted execution.
    ///
    /// The result distinguishes an accepted request from confirmed
    /// cancellation. An engine must not report `Confirmed` merely because
    /// a cancellation message was sent.
    fn cancel<'a>(
        &'a self,
        handle: &'a ExecutionHandle,
        context: &'a OperationContext,
    ) -> ExecutionFuture<'a, CancellationDisposition>;
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execution_reference_accepts_open_world_authorities() {
        let reference = ExecutionReference::new(
            "future.photonics.artifacts",
            "artifact-record-001",
        )
        .expect("valid external reference");

        assert_eq!(
            reference.authority(),
            "future.photonics.artifacts",
        );

        assert_eq!(
            reference.record(),
            "artifact-record-001",
        );
    }

    #[test]
    fn execution_reference_rejects_empty_authority() {
        assert!(
            ExecutionReference::new(
                "",
                "artifact-record-001",
            )
            .is_err()
        );
    }

    #[test]
    fn execution_reference_rejects_empty_namespace_segments() {
        assert!(
            ExecutionReference::new(
                "future..photonics",
                "artifact-record-001",
            )
            .is_err()
        );
    }

    #[test]
    fn execution_reference_rejects_surrounding_whitespace() {
        assert!(
            ExecutionReference::new(
                "future.photonics",
                " artifact-record-001",
            )
            .is_err()
        );
    }

    #[test]
    fn artifact_and_plan_references_are_distinct_types() {
        let artifact = ArtifactReference::new(
            "zamani.compiler.artifacts",
            "artifact-001",
        )
        .expect("valid artifact reference");

        let plan = ExecutionPlanReference::new(
            "zamani.compiler.planning",
            "plan-001",
        )
        .expect("valid plan reference");

        assert_eq!(
            artifact.reference().authority(),
            "zamani.compiler.artifacts",
        );

        assert_eq!(
            plan.reference().authority(),
            "zamani.compiler.planning",
        );
    }

    #[test]
    fn lifecycle_starts_in_created_state() {
        let state = ExecutionState::new();

        assert_eq!(
            state.status(),
            ExecutionStatus::Created,
        );

        assert!(!state.is_terminal());
    }

    #[test]
    fn lifecycle_accepts_a_valid_execution_path() {
        let mut state = ExecutionState::new();

        assert!(state.transition(ExecutionStatus::Validating).is_ok());
        assert!(state.transition(ExecutionStatus::Prepared).is_ok());
        assert!(state.transition(ExecutionStatus::Submitted).is_ok());
        assert!(state.transition(ExecutionStatus::Running).is_ok());
        assert!(state.transition(ExecutionStatus::Completed).is_ok());

        assert!(state.is_terminal());
        assert!(state.status().is_successful());
    }

    #[test]
    fn lifecycle_rejects_skipping_required_validation() {
        let mut state = ExecutionState::new();

        assert!(
            state.transition(ExecutionStatus::Running).is_err()
        );

        assert_eq!(
            state.status(),
            ExecutionStatus::Created,
        );
    }

    #[test]
    fn lifecycle_rejects_transitions_after_completion() {
        let mut state = ExecutionState::new();

        assert!(state.transition(ExecutionStatus::Validating).is_ok());
        assert!(state.transition(ExecutionStatus::Prepared).is_ok());
        assert!(state.transition(ExecutionStatus::Running).is_ok());
        assert!(state.transition(ExecutionStatus::Completed).is_ok());

        assert!(
            state.transition(ExecutionStatus::Running).is_err()
        );

        assert_eq!(
            state.status(),
            ExecutionStatus::Completed,
        );
    }

    #[test]
    fn cancellation_request_is_not_confirmation() {
        assert!(
            !CancellationDisposition::Requested.is_confirmed()
        );

        assert!(
            !CancellationDisposition::Unknown.is_confirmed()
        );

        assert!(
            CancellationDisposition::Confirmed.is_confirmed()
        );
    }

    #[test]
    fn partial_completion_is_not_full_success() {
        assert!(
            !ExecutionStatus::PartiallyCompleted.is_successful()
        );

        assert!(
            ExecutionStatus::Completed.is_successful()
        );
    }
}
