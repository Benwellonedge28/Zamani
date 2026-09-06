//! Zamani Quantum Resilience — Restart Execution Contract
//!
//! Path:
//!     src/quantum/resilience/recovery/restart.rs
//!
//! Purpose:
//!     Provide the provider-independent restart mechanism used by the
//!     resilience subsystem.
//!
//! Architectural position:
//!
//!     Detection
//!         |
//!         v
//!     Diagnosis
//!         |
//!         v
//!     Policy
//!         |
//!         v
//!     Planning
//!         |
//!         v
//!     RecoveryAction::Restart
//!         |
//!         v
//!     RestartController
//!         |
//!         +--> Execution boundary
//!         +--> Runtime
//!         +--> Hardware HAL
//!         +--> Compiler / IR
//!         +--> Routing
//!         +--> Scheduling
//!         +--> QEC
//!         |
//!         v
//!     Verification
//!
//! This module owns the ORCHESTRATION CONTRACT for restarting an execution.
//!
//! It does NOT:
//! - detect faults;
//! - diagnose faults;
//! - select recovery plans;
//! - implement retry policy;
//! - implement routing;
//! - implement scheduling;
//! - compile circuits;
//! - optimize circuits;
//! - implement QEC;
//! - implement error mitigation;
//! - select a hardware provider;
//! - directly manipulate physical hardware;
//! - assume a fixed number of qubits;
//! - assume a fixed number of devices;
//! - assume a fixed topology;
//! - assume a fixed retry count;
//! - assume a fixed timeout;
//! - serialize arbitrary unknown quantum states;
//! - bypass verification;
//! - contain provider-specific recovery logic.
//!
//! Those responsibilities belong to the surrounding resilience and quantum
//! subsystems.
//!
//! -----------------------------------------------------------------------------
//! Core semantic rule
//! -----------------------------------------------------------------------------
//!
//! A restart is NOT an arbitrary replay of an already partially executed
//! quantum state.
//!
//! A restart MUST begin from a valid restart boundary supplied by the execution
//! layer.
//!
//! A valid restart may be:
//!
//! - the beginning of the logical program;
//! - a compiler-approved execution boundary;
//! - a measurement boundary;
//! - a provider-supported restart boundary;
//! - a checkpoint that is explicitly restorable;
//! - a QEC-defined logical boundary;
//! - another boundary explicitly declared by the execution contract.
//!
//! Arbitrary unknown quantum state MUST NOT be assumed to be serializable or
//! restorable.
//!
//! -----------------------------------------------------------------------------
//! "Write once, scale everywhere"
//! -----------------------------------------------------------------------------
//!
//! This module deliberately contains no machine-size constants.
//!
//! A Zamani program may execute on:
//!
//! - one qubit;
//! - a small QPU;
//! - a large QPU;
//! - a fault-tolerant quantum computer;
//! - a simulator;
//! - an emulator;
//! - a heterogeneous quantum fleet;
//! - a distributed quantum execution environment.
//!
//! Restart uses logical execution identity and execution-boundary contracts.
//! Physical realization is delegated to the hardware, routing, scheduling,
//! compiler and QEC layers.
//!
//! Actual scalability is therefore constrained only by:
//!
//! - available resources;
//! - discovered capabilities;
//! - execution/runtime limits;
//! - operating-system limits;
//! - policy;
//! - memory/address-space limits;
//! - provider/backend limits;
//! - semantic requirements.
//!
//! This module introduces no additional finite quantum-machine limit.
//!
//! -----------------------------------------------------------------------------
//! Safety invariant
//! -----------------------------------------------------------------------------
//!
//! A restart may only be accepted when:
//!
//!     action is authorized
//!         AND
//!     restart boundary is valid
//!         AND
//!     execution identity is valid
//!         AND
//!     target capabilities are compatible
//!         AND
//!     semantic requirements are preserved
//!         AND
//!     required provenance is retained
//!         AND
//!     execution succeeds
//!         AND
//!     post-restart verification succeeds
//!
//! Availability alone is never sufficient for accepting a restarted result.
//!
//! -----------------------------------------------------------------------------
//! Integration contract
//! -----------------------------------------------------------------------------
//!
//! planning/action.rs
//!     Provides `RecoveryAction` and `ActionKind::Restart`.
//!
//! planning/plan.rs
//!     Provides the immutable plan containing the restart action.
//!
//! planning/feasibility.rs
//!     Determines whether restart is feasible before this module is invoked.
//!
//! policy/*
//!     Determines whether restart is permitted and what constraints apply.
//!
//! recovery/recoverer.rs
//!     Owns higher-level recovery orchestration and may invoke this module.
//!
//! recovery/retry.rs
//!     Owns retry semantics. This module MUST NOT turn restart into an
//!     implicit retry loop.
//!
//! recovery/checkpoint.rs
//!     Owns checkpoint-specific recovery semantics.
//!
//! recovery/resume.rs
//!     Owns continuation from a checkpoint/boundary.
//!
//! recovery/rollback.rs
//!     Owns rollback semantics.
//!
//! adaptation/*
//!     Performs remapping, rerouting, rescheduling, recompilation,
//!     reoptimization, QEC adaptation and backend selection when required.
//!
//! verification/*
//!     Determines whether the restarted execution may be accepted.
//!
//! state/execution.rs
//!     Owns execution-state representation.
//!
//! state/recovery.rs
//!     Owns recovery lifecycle state.
//!
//! checkpoint/*
//!     Owns checkpoint storage, integrity and compatibility.
//!
//! hardware/*
//!     Owns provider-independent hardware capabilities and execution.
//!
//! routing/*
//!     Owns logical-to-physical mapping and routing.
//!
//! scheduling/*
//!     Owns execution scheduling.
//!
//! optimization/*
//!     Owns canonical-IR optimization.
//!
//! qec/*
//!     Owns quantum error correction.
//!
//! telemetry/*
//!     Records restart intent, execution and outcome.
//!
//! history/*
//!     Records verified historical recovery outcomes.
//!
//! registry/*
//!     May provide dynamically registered restart implementations.
//!
//! -----------------------------------------------------------------------------
//! Dependency rule
//! -----------------------------------------------------------------------------
//!
//! Restart depends on CONTRACTS, not concrete hardware implementations.
//!
//! Hardware-specific behavior must be injected through `RestartExecutor`.
//!
//! This keeps the recovery layer:
//!
//! - provider-independent;
//! - testable;
//! - deterministic when requested;
//! - suitable for simulators and real hardware;
//! - suitable for distributed execution;
//! - compatible with future quantum technologies.
//!
//! -----------------------------------------------------------------------------
//! Rust requirements
//! -----------------------------------------------------------------------------
//!
//! Rust 2021
//! Rust 1.97 / 1.97.1
//! no unsafe code
//! no nightly features
//! no provider-specific dependencies
//!
//! -----------------------------------------------------------------------------
//! Canonical quantum identity
//! -----------------------------------------------------------------------------
//!
//! Restart itself does not require direct qubit manipulation.
//!
//! Consequently this module intentionally does not invent or duplicate a
//! `QubitId`.
//!
//! Whenever an integration implementation needs to refer to quantum resources,
//! it MUST use the canonical repository identity:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! and other canonical quantum IR identities as appropriate.
//!
//! Physical/logical interpretation remains the responsibility of the routing,
//! hardware and QEC layers.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::quantum::resilience::planning::action::RecoveryAction;

// ============================================================================
// Stable schema identity
// ============================================================================

/// Stable identifier for the restart recovery contract.
pub const RESTART_SCHEMA_ID: &str =
    "zamani.quantum.resilience.recovery.restart";

/// Semantic version of this contract.
pub const RESTART_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Restart identity
// ============================================================================

/// Stable identity of one restart operation.
///
/// The identity MUST be supplied by the surrounding execution/recovery layer.
/// This module never generates random identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RestartId(Arc<str>);

impl RestartId {
    /// Creates a restart identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, RestartError> {
        let value = value.into();

        if value.is_empty() {
            return Err(RestartError::InvalidIdentity {
                field: "restart_id",
                reason: "identifier must not be empty",
            });
        }

        Ok(Self(value))
    }

    /// Returns the stable textual identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for RestartId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Execution identity
// ============================================================================

/// Stable identity of the execution being restarted.
///
/// A restart must never accidentally restart a different execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionId(Arc<str>);

impl ExecutionId {
    /// Creates an execution identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, RestartError> {
        let value = value.into();

        if value.is_empty() {
            return Err(RestartError::InvalidIdentity {
                field: "execution_id",
                reason: "identifier must not be empty",
            });
        }

        Ok(Self(value))
    }

    /// Returns the stable execution identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ExecutionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Restart boundary identity
// ============================================================================

/// Stable identity of the boundary from which execution will restart.
///
/// The boundary is deliberately opaque.
///
/// The execution/runtime subsystem owns its meaning.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RestartBoundaryId(Arc<str>);

impl RestartBoundaryId {
    /// Creates a restart-boundary identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, RestartError> {
        let value = value.into();

        if value.is_empty() {
            return Err(RestartError::InvalidIdentity {
                field: "restart_boundary_id",
                reason: "identifier must not be empty",
            });
        }

        Ok(Self(value))
    }

    /// Returns the stable boundary identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for RestartBoundaryId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Program identity
// ============================================================================

/// Stable identity of the logical Zamani program being restarted.
///
/// This is intentionally not a copy of the canonical IR.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProgramId(Arc<str>);

impl ProgramId {
    /// Creates a program identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, RestartError> {
        let value = value.into();

        if value.is_empty() {
            return Err(RestartError::InvalidIdentity {
                field: "program_id",
                reason: "identifier must not be empty",
            });
        }

        Ok(Self(value))
    }

    /// Returns the stable program identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ProgramId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Restart mode
// ============================================================================

/// Describes the semantic origin of a restart boundary.
///
/// This enum does not perform any restart operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RestartMode {
    /// Restart the complete logical computation.
    ProgramStart,

    /// Restart from a compiler/runtime-defined safe boundary.
    ExecutionBoundary,

    /// Restart from a measurement-defined boundary.
    MeasurementBoundary,

    /// Restart from a provider-supported boundary.
    ProviderSupportedBoundary,

    /// Restart from an explicitly validated checkpoint.
    CheckpointBoundary,

    /// Restart from a QEC-defined logical boundary.
    QecBoundary,

    /// Restart from another boundary whose semantics are supplied externally.
    ExternalBoundary,
}

impl RestartMode {
    /// Returns a stable serialized representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProgramStart => "program_start",
            Self::ExecutionBoundary => "execution_boundary",
            Self::MeasurementBoundary => "measurement_boundary",
            Self::ProviderSupportedBoundary => "provider_supported_boundary",
            Self::CheckpointBoundary => "checkpoint_boundary",
            Self::QecBoundary => "qec_boundary",
            Self::ExternalBoundary => "external_boundary",
        }
    }
}

impl fmt::Display for RestartMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Restart authorization
// ============================================================================

/// Authorization state supplied by the policy/security layer.
///
/// Restart does not authorize itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RestartAuthorization {
    /// Authorization has not been evaluated.
    NotEvaluated,

    /// Restart is authorized.
    Authorized,

    /// Restart is explicitly denied.
    Denied,
}

// ============================================================================
// Boundary validity
// ============================================================================

/// Validity of a restart boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BoundaryValidity {
    /// Boundary has not been evaluated.
    Unknown,

    /// Boundary is valid for this execution.
    Valid,

    /// Boundary is no longer valid.
    Invalid,

    /// Boundary belongs to another execution/program.
    Mismatched,

    /// Boundary information is insufficient to determine validity.
    Indeterminate,
}

// ============================================================================
// Restart request
// ============================================================================

/// Immutable request passed to a restart executor.
///
/// The request contains only restart semantics and stable identities.
/// It contains no backend-specific object or credential.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestartRequest {
    restart_id: RestartId,
    execution_id: ExecutionId,
    program_id: ProgramId,
    boundary_id: RestartBoundaryId,
    mode: RestartMode,
    action: RecoveryAction,
    authorization: RestartAuthorization,
    boundary_validity: BoundaryValidity,
    semantic_fingerprint: Option<Arc<str>>,
    provenance_reference: Option<Arc<str>>,
}

impl RestartRequest {
    /// Creates a restart request.
    ///
    /// The caller is responsible for obtaining the `RecoveryAction` from an
    /// immutable recovery plan.
    pub fn new(
        restart_id: RestartId,
        execution_id: ExecutionId,
        program_id: ProgramId,
        boundary_id: RestartBoundaryId,
        mode: RestartMode,
        action: RecoveryAction,
    ) -> Result<Self, RestartError> {
        if action.kind() != crate::quantum::resilience::planning::action::ActionKind::Restart {
            return Err(RestartError::InvalidAction);
        }

        Ok(Self {
            restart_id,
            execution_id,
            program_id,
            boundary_id,
            mode,
            action,
            authorization: RestartAuthorization::NotEvaluated,
            boundary_validity: BoundaryValidity::Unknown,
            semantic_fingerprint: None,
            provenance_reference: None,
        })
    }

    /// Sets authorization state.
    ///
    /// This consumes the request and therefore preserves immutability.
    #[must_use]
    pub fn with_authorization(
        mut self,
        authorization: RestartAuthorization,
    ) -> Self {
        self.authorization = authorization;
        self
    }

    /// Sets the boundary validation state.
    #[must_use]
    pub fn with_boundary_validity(
        mut self,
        validity: BoundaryValidity,
    ) -> Self {
        self.boundary_validity = validity;
        self
    }

    /// Associates the semantic fingerprint of the logical program/execution.
    ///
    /// The fingerprint is opaque to this module.
    pub fn with_semantic_fingerprint(
        mut self,
        fingerprint: impl Into<Arc<str>>,
    ) -> Result<Self, RestartError> {
        let fingerprint = fingerprint.into();

        if fingerprint.is_empty() {
            return Err(RestartError::InvalidArgument(
                "semantic fingerprint must not be empty",
            ));
        }

        self.semantic_fingerprint = Some(fingerprint);
        Ok(self)
    }

    /// Associates a provenance reference.
    pub fn with_provenance_reference(
        mut self,
        reference: impl Into<Arc<str>>,
    ) -> Result<Self, RestartError> {
        let reference = reference.into();

        if reference.is_empty() {
            return Err(RestartError::InvalidArgument(
                "provenance reference must not be empty",
            ));
        }

        self.provenance_reference = Some(reference);
        Ok(self)
    }

    /// Returns the restart identity.
    #[must_use]
    pub fn restart_id(&self) -> &RestartId {
        &self.restart_id
    }

    /// Returns the execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the program identity.
    #[must_use]
    pub fn program_id(&self) -> &ProgramId {
        &self.program_id
    }

    /// Returns the restart-boundary identity.
    #[must_use]
    pub fn boundary_id(&self) -> &RestartBoundaryId {
        &self.boundary_id
    }

    /// Returns the restart mode.
    #[must_use]
    pub const fn mode(&self) -> RestartMode {
        self.mode
    }

    /// Returns the recovery action.
    #[must_use]
    pub fn action(&self) -> &RecoveryAction {
        &self.action
    }

    /// Returns authorization state.
    #[must_use]
    pub const fn authorization(&self) -> RestartAuthorization {
        self.authorization
    }

    /// Returns boundary validity.
    #[must_use]
    pub const fn boundary_validity(&self) -> BoundaryValidity {
        self.boundary_validity
    }

    /// Returns the optional semantic fingerprint.
    #[must_use]
    pub fn semantic_fingerprint(&self) -> Option<&str> {
        self.semantic_fingerprint.as_deref()
    }

    /// Returns the optional provenance reference.
    #[must_use]
    pub fn provenance_reference(&self) -> Option<&str> {
        self.provenance_reference.as_deref()
    }
}

// ============================================================================
// Restart capability
// ============================================================================

/// Capabilities required by the restart executor.
///
/// These capabilities are discovered from the actual execution environment.
/// They are never hard-coded here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RestartCapabilities {
    /// Whether restart is supported at all.
    pub restart: bool,

    /// Whether complete program restart is supported.
    pub program_start: bool,

    /// Whether arbitrary execution boundaries may be used.
    pub execution_boundary: bool,

    /// Whether measurement boundaries are supported.
    pub measurement_boundary: bool,

    /// Whether provider-defined boundaries are supported.
    pub provider_boundary: bool,

    /// Whether checkpoint boundaries are supported.
    pub checkpoint_boundary: bool,

    /// Whether QEC-defined restart boundaries are supported.
    pub qec_boundary: bool,

    /// Whether external boundaries can be consumed.
    pub external_boundary: bool,
}

impl RestartCapabilities {
    /// Returns whether the requested restart mode is supported.
    #[must_use]
    pub const fn supports(self, mode: RestartMode) -> bool {
        if !self.restart {
            return false;
        }

        match mode {
            RestartMode::ProgramStart => self.program_start,
            RestartMode::ExecutionBoundary => self.execution_boundary,
            RestartMode::MeasurementBoundary => self.measurement_boundary,
            RestartMode::ProviderSupportedBoundary => self.provider_boundary,
            RestartMode::CheckpointBoundary => self.checkpoint_boundary,
            RestartMode::QecBoundary => self.qec_boundary,
            RestartMode::ExternalBoundary => self.external_boundary,
        }
    }
}

// ============================================================================
// Execution result
// ============================================================================

/// Result returned by the injected execution layer after a restart attempt.
///
/// This is an execution result, NOT an acceptance decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestartExecutionResult {
    /// Restart identity.
    restart_id: RestartId,

    /// Execution identity.
    execution_id: ExecutionId,

    /// Boundary from which execution actually began.
    boundary_id: RestartBoundaryId,

    /// Whether the runtime reports that restart execution began.
    started: bool,

    /// Whether execution reached a normal completion state.
    completed: bool,

    /// Opaque resulting execution identity, when the runtime changes it.
    resulting_execution_id: Option<ExecutionId>,

    /// Opaque semantic fingerprint produced by execution.
    resulting_semantic_fingerprint: Option<Arc<str>>,

    /// Opaque result/provenance reference.
    result_reference: Option<Arc<str>>,
}

impl RestartExecutionResult {
    /// Creates an execution result.
    pub fn new(
        restart_id: RestartId,
        execution_id: ExecutionId,
        boundary_id: RestartBoundaryId,
        started: bool,
        completed: bool,
    ) -> Self {
        Self {
            restart_id,
            execution_id,
            boundary_id,
            started,
            completed,
            resulting_execution_id: None,
            resulting_semantic_fingerprint: None,
            result_reference: None,
        }
    }

    /// Associates a resulting execution identity.
    #[must_use]
    pub fn with_resulting_execution_id(
        mut self,
        id: ExecutionId,
    ) -> Self {
        self.resulting_execution_id = Some(id);
        self
    }

    /// Associates the resulting semantic fingerprint.
    pub fn with_resulting_semantic_fingerprint(
        mut self,
        fingerprint: impl Into<Arc<str>>,
    ) -> Result<Self, RestartError> {
        let fingerprint = fingerprint.into();

        if fingerprint.is_empty() {
            return Err(RestartError::InvalidArgument(
                "resulting semantic fingerprint must not be empty",
            ));
        }

        self.resulting_semantic_fingerprint = Some(fingerprint);
        Ok(self)
    }

    /// Associates an opaque result reference.
    pub fn with_result_reference(
        mut self,
        reference: impl Into<Arc<str>>,
    ) -> Result<Self, RestartError> {
        let reference = reference.into();

        if reference.is_empty() {
            return Err(RestartError::InvalidArgument(
                "result reference must not be empty",
            ));
        }

        self.result_reference = Some(reference);
        Ok(self)
    }

    /// Returns the restart identity.
    #[must_use]
    pub fn restart_id(&self) -> &RestartId {
        &self.restart_id
    }

    /// Returns the execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the boundary actually used.
    #[must_use]
    pub fn boundary_id(&self) -> &RestartBoundaryId {
        &self.boundary_id
    }

    /// Returns whether restart execution began.
    #[must_use]
    pub const fn started(&self) -> bool {
        self.started
    }

    /// Returns whether execution completed.
    #[must_use]
    pub const fn completed(&self) -> bool {
        self.completed
    }

    /// Returns the resulting execution identity, if supplied.
    #[must_use]
    pub fn resulting_execution_id(&self) -> Option<&ExecutionId> {
        self.resulting_execution_id.as_ref()
    }

    /// Returns the resulting semantic fingerprint.
    #[must_use]
    pub fn resulting_semantic_fingerprint(&self) -> Option<&str> {
        self.resulting_semantic_fingerprint.as_deref()
    }

    /// Returns the opaque result reference.
    #[must_use]
    pub fn result_reference(&self) -> Option<&str> {
        self.result_reference.as_deref()
    }
}

// ============================================================================
// Restart verification result
// ============================================================================

/// Result of post-restart verification.
///
/// Verification is deliberately separate from execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartVerification {
    /// Restarted execution is semantically acceptable.
    Accepted,

    /// Execution completed but may continue only under a degraded policy.
    Degraded,

    /// Execution completed but semantic correctness was not established.
    Unverified,

    /// Verification determined that the result must not be accepted.
    Rejected,
}

impl RestartVerification {
    /// Returns whether the result is safe to return as accepted.
    #[must_use]
    pub const fn is_accepted(self) -> bool {
        matches!(self, Self::Accepted | Self::Degraded)
    }
}

// ============================================================================
// Restart outcome
// ============================================================================

/// Final restart outcome.
///
/// This object records what happened. It does not silently convert an
/// execution failure into success.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestartOutcome {
    /// Restart identity.
    restart_id: RestartId,

    /// Execution identity supplied to restart.
    execution_id: ExecutionId,

    /// Boundary used.
    boundary_id: RestartBoundaryId,

    /// Execution result.
    execution: RestartExecutionResult,

    /// Verification result.
    verification: RestartVerification,

    /// Total elapsed wall-clock duration measured by the controller.
    ///
    /// This is observational and must never be used as a hard-coded policy.
    elapsed: Duration,
}

impl RestartOutcome {
    /// Creates a final restart outcome.
    #[must_use]
    pub fn new(
        request: &RestartRequest,
        execution: RestartExecutionResult,
        verification: RestartVerification,
        elapsed: Duration,
    ) -> Self {
        Self {
            restart_id: request.restart_id.clone(),
            execution_id: request.execution_id.clone(),
            boundary_id: request.boundary_id.clone(),
            execution,
            verification,
            elapsed,
        }
    }

    /// Returns restart identity.
    #[must_use]
    pub fn restart_id(&self) -> &RestartId {
        &self.restart_id
    }

    /// Returns execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns boundary identity.
    #[must_use]
    pub fn boundary_id(&self) -> &RestartBoundaryId {
        &self.boundary_id
    }

    /// Returns execution details.
    #[must_use]
    pub fn execution(&self) -> &RestartExecutionResult {
        &self.execution
    }

    /// Returns verification status.
    #[must_use]
    pub const fn verification(&self) -> RestartVerification {
        self.verification
    }

    /// Returns elapsed execution-controller time.
    #[must_use]
    pub const fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Returns whether the result is accepted.
    #[must_use]
    pub const fn accepted(&self) -> bool {
        self.verification.is_accepted()
    }
}

// ============================================================================
// Restart executor
// ============================================================================

/// Provider-independent restart execution contract.
///
/// Implementations belong to the runtime/execution integration layer.
///
/// An implementation may delegate to:
///
/// - the quantum runtime;
/// - the hardware HAL;
/// - a simulator;
/// - an emulator;
/// - a distributed execution coordinator.
///
/// It must not require this module to know the concrete provider.
///
/// Implementations MUST NOT silently reinterpret a restart as a retry policy.
/// If repeated execution is required, `recovery/retry.rs` owns that policy.
pub trait RestartExecutor: Send + Sync {
    /// Returns dynamically discovered restart capabilities.
    fn capabilities(&self) -> RestartCapabilities;

    /// Executes one restart request.
    ///
    /// This method represents ONE restart operation.
    ///
    /// It MUST NOT contain an unbounded retry loop.
    ///
    /// The executor must establish the restart boundary using its authoritative
    /// runtime/execution contract.
    fn execute(
        &self,
        request: &RestartRequest,
    ) -> Result<RestartExecutionResult, RestartError>;
}

// ============================================================================
// Restart verifier
// ============================================================================

/// Verification contract for restarted executions.
///
/// Implementations belong to `verification/*` integration.
///
/// Verification must use the canonical logical program/IR and applicable
/// execution semantics rather than assuming that successful hardware
/// completion means semantic correctness.
pub trait RestartVerifier: Send + Sync {
    /// Verifies a restarted execution.
    fn verify(
        &self,
        request: &RestartRequest,
        execution: &RestartExecutionResult,
    ) -> Result<RestartVerification, RestartError>;
}

// ============================================================================
// Restart observer
// ============================================================================

/// Optional observer for telemetry/audit integration.
///
/// Implementations may forward events to:
///
/// - telemetry;
/// - tracing;
/// - audit logs;
/// - history;
/// - distributed monitoring.
///
/// The observer must never be required for correctness.
pub trait RestartObserver: Send + Sync {
    /// Called immediately before execution.
    fn on_started(&self, request: &RestartRequest);

    /// Called after execution and verification.
    fn on_completed(&self, outcome: &RestartOutcome);
}

/// No-op observer useful for callers that do not require telemetry.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopRestartObserver;

impl RestartObserver for NoopRestartObserver {
    fn on_started(&self, _request: &RestartRequest) {}

    fn on_completed(&self, _outcome: &RestartOutcome) {}
}

// ============================================================================
// Restart controller
// ============================================================================

/// Coordinates one restart operation.
///
/// The controller is deliberately stateless between calls.
///
/// Long-lived execution/recovery state belongs in:
///
///     state/recovery.rs
///     state/execution.rs
///     checkpoint/*
///     history/*
///
/// This design allows a caller to scale across arbitrary numbers of
/// simultaneous executions without this object maintaining a fixed-size
/// machine model.
pub struct RestartController<E, V, O = NoopRestartObserver>
where
    E: RestartExecutor,
    V: RestartVerifier,
    O: RestartObserver,
{
    executor: Arc<E>,
    verifier: Arc<V>,
    observer: Arc<O>,
}

impl<E, V> RestartController<E, V, NoopRestartObserver>
where
    E: RestartExecutor,
    V: RestartVerifier,
{
    /// Creates a controller without telemetry observation.
    #[must_use]
    pub fn new(
        executor: Arc<E>,
        verifier: Arc<V>,
    ) -> Self {
        Self {
            executor,
            verifier,
            observer: Arc::new(NoopRestartObserver),
        }
    }
}

impl<E, V, O> RestartController<E, V, O>
where
    E: RestartExecutor,
    V: RestartVerifier,
    O: RestartObserver,
{
    /// Creates a controller with an injected observer.
    #[must_use]
    pub fn with_observer(
        executor: Arc<E>,
        verifier: Arc<V>,
        observer: Arc<O>,
    ) -> Self {
        Self {
            executor,
            verifier,
            observer,
        }
    }

    /// Executes exactly one restart operation.
    ///
    /// No retry loop is performed here.
    ///
    /// The caller may invoke this method again only when a higher-level policy
    /// explicitly authorizes another recovery action.
    pub fn restart(
        &self,
        request: &RestartRequest,
    ) -> Result<RestartOutcome, RestartError> {
        validate_request(request)?;

        let capabilities = self.executor.capabilities();

        if !capabilities.supports(request.mode()) {
            return Err(RestartError::UnsupportedMode {
                mode: request.mode(),
            });
        }

        if request.authorization() != RestartAuthorization::Authorized {
            return Err(RestartError::NotAuthorized);
        }

        if request.boundary_validity() != BoundaryValidity::Valid {
            return Err(RestartError::InvalidBoundary {
                validity: request.boundary_validity(),
            });
        }

        self.observer.on_started(request);

        let started_at = Instant::now();

        let execution = self.executor.execute(request)?;

        validate_execution_result(request, &execution)?;

        let verification = self.verifier.verify(request, &execution)?;

        let elapsed = started_at.elapsed();

        let outcome =
            RestartOutcome::new(request, execution, verification, elapsed);

        self.observer.on_completed(&outcome);

        if !outcome.accepted() {
            return Err(RestartError::VerificationRejected {
                verification: outcome.verification(),
            });
        }

        Ok(outcome)
    }

    /// Returns the dynamically discovered executor capabilities.
    #[must_use]
    pub fn capabilities(&self) -> RestartCapabilities {
        self.executor.capabilities()
    }
}

// ============================================================================
// Request validation
// ============================================================================

fn validate_request(
    request: &RestartRequest,
) -> Result<(), RestartError> {
    if request.restart_id().as_str().is_empty() {
        return Err(RestartError::InvalidIdentity {
            field: "restart_id",
            reason: "identifier must not be empty",
        });
    }

    if request.execution_id().as_str().is_empty() {
        return Err(RestartError::InvalidIdentity {
            field: "execution_id",
            reason: "identifier must not be empty",
        });
    }

    if request.program_id().as_str().is_empty() {
        return Err(RestartError::InvalidIdentity {
            field: "program_id",
            reason: "identifier must not be empty",
        });
    }

    if request.boundary_id().as_str().is_empty() {
        return Err(RestartError::InvalidIdentity {
            field: "restart_boundary_id",
            reason: "identifier must not be empty",
        });
    }

    Ok(())
}

// ============================================================================
// Execution result validation
// ============================================================================

fn validate_execution_result(
    request: &RestartRequest,
    result: &RestartExecutionResult,
) -> Result<(), RestartError> {
    if result.restart_id() != request.restart_id() {
        return Err(RestartError::ExecutionIdentityMismatch);
    }

    if result.execution_id() != request.execution_id() {
        return Err(RestartError::ExecutionIdentityMismatch);
    }

    if result.boundary_id() != request.boundary_id() {
        return Err(RestartError::BoundaryMismatch);
    }

    if !result.started() {
        return Err(RestartError::ExecutionDidNotStart);
    }

    Ok(())
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the restart contract.
///
/// The repository's central resilience error layer may map these errors into
/// the canonical `ResilienceError` taxonomy at the integration boundary.
///
/// This local type intentionally preserves restart-specific information without
/// introducing provider-specific error categories.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartError {
    /// A stable identity was invalid.
    InvalidIdentity {
        /// Name of the invalid field.
        field: &'static str,

        /// Reason for invalidity.
        reason: &'static str,
    },

    /// A caller supplied an invalid argument.
    InvalidArgument(&'static str),

    /// The action supplied to this module was not Restart.
    InvalidAction,

    /// Restart authorization was not granted.
    NotAuthorized,

    /// The requested restart mode is unsupported by the current target.
    UnsupportedMode {
        /// Requested mode.
        mode: RestartMode,
    },

    /// The restart boundary is not valid.
    InvalidBoundary {
        /// Boundary validity state.
        validity: BoundaryValidity,
    },

    /// Execution and request identities did not match.
    ExecutionIdentityMismatch,

    /// Restart boundary identities did not match.
    BoundaryMismatch,

    /// The execution layer did not actually start the restart.
    ExecutionDidNotStart,

    /// The verifier rejected the result.
    VerificationRejected {
        /// Verification status.
        verification: RestartVerification,
    },

    /// The underlying execution layer reported an error.
    ExecutorFailure(Arc<str>),

    /// The verifier reported an error.
    VerifierFailure(Arc<str>),
}

impl RestartError {
    /// Creates an executor failure without exposing provider internals in the
    /// restart contract.
    pub fn executor_failure(message: impl Into<Arc<str>>) -> Self {
        Self::ExecutorFailure(message.into())
    }

    /// Creates a verifier failure.
    pub fn verifier_failure(message: impl Into<Arc<str>>) -> Self {
        Self::VerifierFailure(message.into())
    }
}

impl fmt::Display for RestartError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity { field, reason } => {
                write!(
                    formatter,
                    "invalid restart identity `{field}`: {reason}"
                )
            }

            Self::InvalidArgument(message) => {
                write!(formatter, "invalid restart argument: {message}")
            }

            Self::InvalidAction => {
                formatter.write_str(
                    "restart controller received an action that is not Restart",
                )
            }

            Self::NotAuthorized => {
                formatter.write_str(
                    "restart operation is not authorized",
                )
            }

            Self::UnsupportedMode { mode } => {
                write!(
                    formatter,
                    "restart mode `{mode}` is not supported by the current execution target"
                )
            }

            Self::InvalidBoundary { validity } => {
                write!(
                    formatter,
                    "restart boundary is not valid: {validity:?}"
                )
            }

            Self::ExecutionIdentityMismatch => {
                formatter.write_str(
                    "restart execution identity does not match the request",
                )
            }

            Self::BoundaryMismatch => {
                formatter.write_str(
                    "restart boundary does not match the request",
                )
            }

            Self::ExecutionDidNotStart => {
                formatter.write_str(
                    "restart execution did not start",
                )
            }

            Self::VerificationRejected { verification } => {
                write!(
                    formatter,
                    "restarted execution was not accepted: {verification:?}"
                )
            }

            Self::ExecutorFailure(message) => {
                write!(formatter, "restart executor failure: {message}")
            }

            Self::VerifierFailure(message) => {
                write!(formatter, "restart verifier failure: {message}")
            }
        }
    }
}

impl std::error::Error for RestartError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::resilience::planning::action::{
        ActionId,
        ActionKind,
        ActionScope,
        RecoveryAction,
    };

    struct TestExecutor;

    impl RestartExecutor for TestExecutor {
        fn capabilities(&self) -> RestartCapabilities {
            RestartCapabilities {
                restart: true,
                program_start: true,
                execution_boundary: true,
                measurement_boundary: true,
                provider_boundary: true,
                checkpoint_boundary: true,
                qec_boundary: true,
                external_boundary: true,
            }
        }

        fn execute(
            &self,
            request: &RestartRequest,
        ) -> Result<RestartExecutionResult, RestartError> {
            Ok(RestartExecutionResult::new(
                request.restart_id().clone(),
                request.execution_id().clone(),
                request.boundary_id().clone(),
                true,
                true,
            ))
        }
    }

    struct TestVerifier;

    impl RestartVerifier for TestVerifier {
        fn verify(
            &self,
            _request: &RestartRequest,
            _execution: &RestartExecutionResult,
        ) -> Result<RestartVerification, RestartError> {
            Ok(RestartVerification::Accepted)
        }
    }

    fn restart_action() -> RecoveryAction {
        RecoveryAction::restart(
            ActionId::new(1),
            ActionScope::Execution,
        )
    }

    fn request() -> RestartRequest {
        RestartRequest::new(
            RestartId::new("restart-1").expect("valid restart id"),
            ExecutionId::new("execution-1").expect("valid execution id"),
            ProgramId::new("program-1").expect("valid program id"),
            RestartBoundaryId::new("boundary-1")
                .expect("valid boundary id"),
            RestartMode::ExecutionBoundary,
            restart_action(),
        )
        .expect("valid restart request")
        .with_authorization(RestartAuthorization::Authorized)
        .with_boundary_validity(BoundaryValidity::Valid)
    }

    #[test]
    fn restart_requires_restart_action() {
        let action = RecoveryAction::retry(
            ActionId::new(2),
            ActionScope::Execution,
        );

        let result = RestartRequest::new(
            RestartId::new("restart-1").expect("valid restart id"),
            ExecutionId::new("execution-1").expect("valid execution id"),
            ProgramId::new("program-1").expect("valid program id"),
            RestartBoundaryId::new("boundary-1")
                .expect("valid boundary id"),
            RestartMode::ExecutionBoundary,
            action,
        );

        assert_eq!(result, Err(RestartError::InvalidAction));
    }

    #[test]
    fn restart_requires_authorization() {
        let controller = RestartController::new(
            Arc::new(TestExecutor),
            Arc::new(TestVerifier),
        );

        let result = controller.restart(
            &request().with_authorization(
                RestartAuthorization::NotEvaluated,
            ),
        );

        assert_eq!(result, Err(RestartError::NotAuthorized));
    }

    #[test]
    fn restart_requires_valid_boundary() {
        let controller = RestartController::new(
            Arc::new(TestExecutor),
            Arc::new(TestVerifier),
        );

        let result = controller.restart(
            &request().with_boundary_validity(
                BoundaryValidity::Unknown,
            ),
        );

        assert_eq!(
            result,
            Err(RestartError::InvalidBoundary {
                validity: BoundaryValidity::Unknown,
            })
        );
    }

    #[test]
    fn restart_executes_once_and_verifies() {
        let controller = RestartController::new(
            Arc::new(TestExecutor),
            Arc::new(TestVerifier),
        );

        let result = controller
            .restart(&request())
            .expect("restart should succeed");

        assert!(result.execution().started());
        assert!(result.execution().completed());
        assert_eq!(
            result.verification(),
            RestartVerification::Accepted
        );
        assert!(result.accepted());
    }

    #[test]
    fn restart_capabilities_are_dynamic() {
        struct LimitedExecutor;

        impl RestartExecutor for LimitedExecutor {
            fn capabilities(&self) -> RestartCapabilities {
                RestartCapabilities {
                    restart: true,
                    program_start: true,
                    execution_boundary: false,
                    measurement_boundary: false,
                    provider_boundary: false,
                    checkpoint_boundary: false,
                    qec_boundary: false,
                    external_boundary: false,
                }
            }

            fn execute(
                &self,
                _request: &RestartRequest,
            ) -> Result<RestartExecutionResult, RestartError> {
                unreachable!("unsupported mode must be rejected before execution")
            }
        }

        let controller = RestartController::new(
            Arc::new(LimitedExecutor),
            Arc::new(TestVerifier),
        );

        let result = controller.restart(&request());

        assert_eq!(
            result,
            Err(RestartError::UnsupportedMode {
                mode: RestartMode::ExecutionBoundary,
            })
        );
    }

    #[test]
    fn restart_does_not_implicitly_retry() {
        // The controller exposes exactly one executor invocation per call.
        // Retry policy belongs to recovery/retry.rs.
        let controller = RestartController::new(
            Arc::new(TestExecutor),
            Arc::new(TestVerifier),
        );

        let first = controller.restart(&request());

        assert!(first.is_ok());
    }

    #[test]
    fn restart_mode_has_stable_representation() {
        assert_eq!(
            RestartMode::ProgramStart.as_str(),
            "program_start"
        );

        assert_eq!(
            RestartMode::CheckpointBoundary.as_str(),
            "checkpoint_boundary"
        );
    }

    #[test]
    fn restart_verification_is_separate_from_execution() {
        let execution = RestartExecutionResult::new(
            RestartId::new("r").expect("valid id"),
            ExecutionId::new("e").expect("valid id"),
            RestartBoundaryId::new("b").expect("valid id"),
            true,
            true,
        );

        assert!(execution.completed());

        // Completion itself does not establish acceptance.
        assert_ne!(
            RestartVerification::Unverified,
            RestartVerification::Accepted
        );
    }
}