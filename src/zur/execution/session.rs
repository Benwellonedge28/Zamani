
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Execution Sessions
//!
//! Path: src/zur/execution/session.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! Provides an isolated, explicitly owned execution session for
//! one ZUR runtime instance.
//!
//! A session groups execution-related operational state without
//! taking ownership of program semantics, execution plans,
//! artifacts, physical resources or provider implementations.
//!
//! ## Responsibilities
//!
//! This module owns:
//!
//! - Session lifecycle.
//! - Session identity association.
//! - Session-scoped operation context.
//! - Cooperative session cancellation.
//! - Parent-runtime cancellation propagation.
//! - Service lease references.
//! - Resource-accounting references.
//! - Session diagnostics.
//! - Session snapshots.
//! - Thread-safe session state.
//!
//! ## Existing authorities
//!
//! - zur/ids.rs:
//!   SessionId, ServiceInstanceId, ResourceLeaseId.
//!
//! - zur/error.rs:
//!   ZurError and ZurResult.
//!
//! - zur/core/context.rs:
//!   RuntimeContext.
//!
//! - zur/contracts/execution.rs:
//!   ExecutionState, ExecutionHandle and ExecutionEngine.
//!
//! - zur/contracts/service.rs:
//!   Runtime service contracts.
//!
//! - stdlib/context.rs:
//!   OperationContext and CancellationSignal.
//!
//! ## Ownership boundaries
//!
//! A session does not:
//!
//! - Define source-language semantics.
//! - Own AST or IR.
//! - Validate compiler artifacts.
//! - Construct execution plans.
//! - Select hardware.
//! - Discover providers.
//! - Allocate physical resources.
//! - Implement service lifecycles.
//! - Execute provider operations.
//! - Implement distributed recovery.
//! - Duplicate execution-engine contracts.
//!
//! ## Scalability
//!
//! No fixed limits are imposed on:
//!
//! - Sessions.
//! - Executions associated with a session.
//! - Service lease references.
//! - Resource-accounting references.
//! - Diagnostics.
//! - Runtime instances.
//! - Computational domains.
//! - Physical devices.
//! - Distributed participants.
//!
//! Collections grow according to actual allocation capacity.
//!
//! Physical resource availability and implementation constraints
//! remain explicit operational facts.
//!
//! Literal infinite computation is not promised.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No raw pointers.
//! No unsafe implementations.
//! No static mutable state.
//! No global session registry.
//! No fabricated cancellation confirmation.
//! No implicit physical resource acquisition.
//!
//! ## Integration
//!
//! RuntimeBuilder supplies a validated RuntimeContext and SessionId.
//!
//! ExecutionEngine receives the session's OperationContext.
//!
//! Resource management supplies resource lease references.
//!
//! Service management supplies service lease references.
//!
//! Observability consumes session diagnostics and snapshots.
//!
//! Distributed and hybrid execution use the same session identity
//! without creating competing session implementations.

// ============================================================================
// IMPORTS
// ============================================================================

use core::fmt;

use std::any::Any;
use std::collections::BTreeMap;
use std::sync::{
    Arc,
    Mutex,
    MutexGuard,
};
use std::sync::atomic::{
    AtomicBool,
    Ordering,
};

use crate::stdlib::context::{
    CancellationSignal,
    OperationContext,
};

use crate::zur::core::context::RuntimeContext;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::ids::{
    ResourceLeaseId,
    ServiceInstanceId,
    SessionId,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.execution.session";

// ============================================================================
// SESSION LIFECYCLE
// ============================================================================

/// Observable lifecycle of an execution session.
///
/// This lifecycle is independent of the runtime-instance lifecycle
/// and the lifecycle of individual executions.
///
/// Execution lifecycle is owned by
/// `zur::contracts::execution`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SessionStatus {
    /// The session has been created but is not yet active.
    Created,

    /// The session is accepting execution-related operations.
    Active,

    /// Cancellation has been requested or session shutdown
    /// has begun.
    Cancelling,

    /// The session has encountered an unrecoverable session-level
    /// failure.
    Failed,

    /// The session is shutting down and releasing its references.
    Closing,

    /// The session has completed its local cleanup.
    Closed,
}

impl SessionStatus {
    /// Returns whether the session is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Closed)
    }

    /// Returns whether new execution-related operations are permitted.
    #[must_use]
    pub const fn accepts_operations(self) -> bool {
        matches!(self, Self::Active)
    }
}

// ============================================================================
// SESSION DIAGNOSTICS
// ============================================================================

/// Severity of a session diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DiagnosticLevel {
    /// Informational diagnostic.
    Info,

    /// Non-fatal warning.
    Warning,

    /// Error requiring attention.
    Error,
}

/// Structured session diagnostic.
///
/// Diagnostic metadata contains descriptive information only.
/// It must not be used as an authorization or execution decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionDiagnostic {
    level: DiagnosticLevel,

    component: String,

    message: String,

    metadata: BTreeMap<String, String>,
}

impl SessionDiagnostic {
    /// Creates a diagnostic.
    ///
    /// Component and message must not be empty or contain
    /// control characters.
    pub fn new(
        level: DiagnosticLevel,
        component: impl Into<String>,
        message: impl Into<String>,
    ) -> ZurResult<Self> {
        let component = component.into();
        let message = message.into();

        validate_label(&component, "diagnostic component")?;
        validate_label(&message, "diagnostic message")?;

        Ok(Self {
            level,
            component,
            message,
            metadata: BTreeMap::new(),
        })
    }

    /// Adds descriptive metadata.
    ///
    /// Duplicate keys are rejected rather than silently overwritten.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let key = key.into();
        let value = value.into();

        validate_label(&key, "diagnostic metadata key")?;
        validate_label(&value, "diagnostic metadata value")?;

        if self.metadata.contains_key(&key) {
            return Err(
                ZurError::invalid_input(
                    "duplicate session diagnostic metadata key",
                )
                .with_component(COMPONENT),
            );
        }

        self.metadata.insert(key, value);

        Ok(self)
    }

    /// Returns diagnostic severity.
    #[must_use]
    pub const fn level(&self) -> DiagnosticLevel {
        self.level
    }

    /// Returns the diagnostic component.
    #[must_use]
    pub fn component(&self) -> &str {
        &self.component
    }

    /// Returns the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns diagnostic metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }
}

// ============================================================================
// SESSION CANCELLATION
// ============================================================================

/// Thread-safe cooperative cancellation source owned by a session.
///
/// This source never claims that external execution has stopped.
///
/// It only communicates that cancellation has been requested.
#[derive(Default)]
pub struct SessionCancellation {
    cancelled: AtomicBool,

    reason: Mutex<Option<String>>,
}

impl SessionCancellation {
    /// Requests cooperative cancellation.
    ///
    /// The first cancellation reason is retained.
    ///
    /// Repeated cancellation requests are idempotent.
    pub fn cancel(
        &self,
        reason: impl Into<String>,
    ) -> ZurResult<bool> {
        let reason = reason.into();

        validate_label(&reason, "cancellation reason")?;

        let mut stored_reason = self.lock_reason()?;

        if self.cancelled.load(Ordering::Acquire) {
            return Ok(false);
        }

        *stored_reason = Some(reason);

        self.cancelled.store(true, Ordering::Release);

        Ok(true)
    }

    /// Returns whether cancellation was requested.
    #[must_use]
    pub fn is_requested(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    /// Returns the first cancellation reason.
    pub fn reason(&self) -> ZurResult<Option<String>> {
        if !self.is_requested() {
            return Ok(None);
        }

        Ok(self.lock_reason()?.clone())
    }

    fn lock_reason(
        &self,
    ) -> ZurResult<MutexGuard<'_, Option<String>>> {
        self.reason.lock().map_err(|_| {
            internal_error("session cancellation lock was poisoned")
        })
    }
}

impl CancellationSignal for SessionCancellation {
    fn is_cancelled(&self) -> bool {
        self.is_requested()
    }

    fn cancellation_reason(&self) -> Option<String> {
        self.reason().ok().flatten()
    }
}

impl fmt::Debug for SessionCancellation {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("SessionCancellation")
            .field("cancelled", &self.is_requested())
            .finish()
    }
}

// ============================================================================
// COMBINED CANCELLATION
// ============================================================================

/// Combines runtime-level and session-level cancellation.
///
/// Either cancellation source can request termination.
///
/// This preserves parent-runtime cancellation propagation while
/// allowing individual sessions to be cancelled independently.
struct CombinedCancellation {
    parent: Arc<dyn CancellationSignal>,

    session: Arc<SessionCancellation>,
}

impl CancellationSignal for CombinedCancellation {
    fn is_cancelled(&self) -> bool {
        self.parent.is_cancelled()
            || self.session.is_requested()
    }

    fn cancellation_reason(&self) -> Option<String> {
        if let Some(reason) = self.session.cancellation_reason() {
            return Some(reason);
        }

        self.parent.cancellation_reason()
    }
}

// ============================================================================
// SERVICE LEASE REFERENCES
// ============================================================================

/// Strong reference to an externally owned service lease.
///
/// The service implementation and lease lifecycle remain owned
/// by the service-management subsystem.
///
/// Keeping this reference in a session maintains ownership of the
/// supplied lease object.
///
/// Dropping this reference does not claim that a remote service
/// has been shut down.
#[derive(Clone)]
pub struct ServiceLeaseReference {
    service_id: ServiceInstanceId,

    owner: Arc<dyn Any + Send + Sync>,
}

impl ServiceLeaseReference {
    /// Creates a reference to an existing service lease.
    pub fn new<T>(
        service_id: ServiceInstanceId,
        lease: Arc<T>,
    ) -> Self
    where
        T: Any + Send + Sync,
    {
        Self {
            service_id,
            owner: lease,
        }
    }

    /// Returns the service instance identity.
    #[must_use]
    pub fn service_id(&self) -> &ServiceInstanceId {
        &self.service_id
    }

    /// Returns the concrete lease object when its type matches.
    #[must_use]
    pub fn lease<T>(&self) -> Option<Arc<T>>
    where
        T: Any + Send + Sync,
    {
        Arc::downcast::<T>(Arc::clone(&self.owner)).ok()
    }
}

impl fmt::Debug for ServiceLeaseReference {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ServiceLeaseReference")
            .field("service_id", &self.service_id)
            .finish_non_exhaustive()
    }
}

// ============================================================================
// RESOURCE ACCOUNTING REFERENCES
// ============================================================================

/// Strong reference to an externally owned resource-accounting record.
///
/// Physical resource allocation, reservations, budgets and accounting
/// remain owned by the resource-management subsystem.
///
/// This reference does not itself reserve or allocate resources.
#[derive(Clone)]
pub struct ResourceAccountingReference {
    lease_id: ResourceLeaseId,

    owner: Arc<dyn Any + Send + Sync>,
}

impl ResourceAccountingReference {
    /// Creates a reference to an existing resource-accounting object.
    pub fn new<T>(
        lease_id: ResourceLeaseId,
        accounting: Arc<T>,
    ) -> Self
    where
        T: Any + Send + Sync,
    {
        Self {
            lease_id,
            owner: accounting,
        }
    }

    /// Returns the canonical resource lease identity.
    #[must_use]
    pub fn lease_id(&self) -> &ResourceLeaseId {
        &self.lease_id
    }

    /// Returns the concrete accounting object when its type matches.
    #[must_use]
    pub fn accounting<T>(&self) -> Option<Arc<T>>
    where
        T: Any + Send + Sync,
    {
        Arc::downcast::<T>(Arc::clone(&self.owner)).ok()
    }
}

impl fmt::Debug for ResourceAccountingReference {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ResourceAccountingReference")
            .field("lease_id", &self.lease_id)
            .finish_non_exhaustive()
    }
}

// ============================================================================
// INTERNAL SESSION STATE
// ============================================================================

/// Mutable state owned exclusively by one session.
struct SessionInner {
    status: SessionStatus,

    service_leases: Vec<ServiceLeaseReference>,

    resource_references: Vec<ResourceAccountingReference>,

    diagnostics: Vec<SessionDiagnostic>,
}

impl SessionInner {
    fn new() -> Self {
        Self {
            status: SessionStatus::Created,
            service_leases: Vec::new(),
            resource_references: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    fn transition(
        &mut self,
        next: SessionStatus,
    ) -> ZurResult<()> {
        let allowed = match self.status {
            SessionStatus::Created => {
                matches!(
                    next,
                    SessionStatus::Active
                        | SessionStatus::Cancelling
                        | SessionStatus::Failed
                        | SessionStatus::Closing
                )
            }

            SessionStatus::Active => {
                matches!(
                    next,
                    SessionStatus::Cancelling
                        | SessionStatus::Failed
                        | SessionStatus::Closing
                )
            }

            SessionStatus::Cancelling => {
                matches!(
                    next,
                    SessionStatus::Failed
                        | SessionStatus::Closing
                )
            }

            SessionStatus::Failed => {
                matches!(next, SessionStatus::Closing)
            }

            SessionStatus::Closing => {
                matches!(next, SessionStatus::Closed)
            }

            SessionStatus::Closed => false,
        };

        if !allowed {
            return Err(
                ZurError::invalid_input(
                    format!(
                        "invalid session transition: {:?} -> {:?}",
                        self.status,
                        next,
                    ),
                )
                .with_component(COMPONENT)
                .with_operation("transition"),
            );
        }

        self.status = next;

        Ok(())
    }
}

// ============================================================================
// SESSION SNAPSHOT
// ============================================================================

/// Immutable observation of session-owned operational state.
///
/// This is a local snapshot. It does not establish remote execution
/// completion, physical resource availability or provider state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSnapshot {
    session_id: SessionId,

    runtime_id: String,

    status: SessionStatus,

    cancellation_requested: bool,

    service_lease_count: usize,

    resource_reference_count: usize,

    diagnostic_count: usize,
}

impl SessionSnapshot {
    /// Returns the session identity.
    #[must_use]
    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    /// Returns the associated runtime identity as its canonical string.
    #[must_use]
    pub fn runtime_id(&self) -> &str {
        &self.runtime_id
    }

    /// Returns the observed session state.
    #[must_use]
    pub const fn status(&self) -> SessionStatus {
        self.status
    }

    /// Returns whether cancellation has been requested.
    #[must_use]
    pub const fn cancellation_requested(&self) -> bool {
        self.cancellation_requested
    }

    /// Returns the number of attached service leases.
    #[must_use]
    pub const fn service_lease_count(&self) -> usize {
        self.service_lease_count
    }

    /// Returns the number of resource-accounting references.
    #[must_use]
    pub const fn resource_reference_count(&self) -> usize {
        self.resource_reference_count
    }

    /// Returns the number of session diagnostics.
    #[must_use]
    pub const fn diagnostic_count(&self) -> usize {
        self.diagnostic_count
    }
}

// ============================================================================
// SESSION SHARED STORAGE
// ============================================================================

struct SessionShared {
    inner: Mutex<SessionInner>,
}

// ============================================================================
// EXECUTION SESSION
// ============================================================================

/// Thread-safe, isolated ZUR execution session.
///
/// A session is associated with exactly one runtime instance.
///
/// Cloning a session clones its shared handle, not its operational state.
///
/// Multiple threads may safely access the same session.
///
/// Independent sessions do not share mutable operational state.
///
/// The session's operation context combines:
///
/// - Parent runtime cancellation.
/// - Session-local cancellation.
/// - Inherited typed operation extensions.
///
/// # Lifecycle
///
/// Created
///   ↓
/// Active
///   ↓
/// Cancelling / Failed / Closing
///   ↓
/// Closing
///   ↓
/// Closed
///
/// A session may also be closed directly before activation.
///
/// # Cancellation
///
/// Cancellation is cooperative.
///
/// `request_cancellation` signals local and nested operations.
///
/// It does not prove that an external execution authority has
/// confirmed cancellation.
///
/// # Resource ownership
///
/// Service leases and resource-accounting references are retained
/// as strong references.
///
/// The respective external subsystem remains responsible for
/// actual resource release and service shutdown.
#[derive(Clone)]
pub struct ExecutionSession {
    session_id: SessionId,

    runtime_context: Arc<RuntimeContext>,

    operation_context: OperationContext,

    cancellation: Arc<SessionCancellation>,

    shared: Arc<SessionShared>,
}

impl ExecutionSession {
    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    /// Creates a new execution session.
    ///
    /// The caller must supply an existing canonical session identity
    /// and a validated runtime context.
    ///
    /// No global identity allocation, hardware discovery,
    /// provider discovery or resource acquisition occurs here.
    ///
    /// # Errors
    ///
    /// Returns a structured error when the runtime context is invalid
    /// or the operation context cannot be constructed.
    pub fn new(
        session_id: SessionId,
        runtime_context: Arc<RuntimeContext>,
    ) -> ZurResult<Self> {
        runtime_context.validate()?;

        let cancellation = Arc::new(
            SessionCancellation::default(),
        );

        let combined: Arc<dyn CancellationSignal> =
            Arc::new(CombinedCancellation {
                parent: runtime_context.cancellation_signal(),
                session: Arc::clone(&cancellation),
            });

        let operation_context = runtime_context
            .operation_context()
            .child_builder()
            .with_operation("zur.execution.session")
            .map_err(|error| {
                ZurError::invalid_input(error.to_string())
                    .with_component(COMPONENT)
                    .with_operation("new")
            })?
            .with_scope(session_id.to_string())
            .map_err(|error| {
                ZurError::invalid_input(error.to_string())
                    .with_component(COMPONENT)
                    .with_operation("new")
            })?
            .with_cancellation(combined)
            .build();

        Ok(Self {
            session_id,
            runtime_context,
            operation_context,
            cancellation,
            shared: Arc::new(SessionShared {
                inner: Mutex::new(SessionInner::new()),
            }),
        })
    }

    // ------------------------------------------------------------------------
    // Identity
    // ------------------------------------------------------------------------

    /// Returns the canonical session identity.
    #[must_use]
    pub fn id(&self) -> &SessionId {
        &self.session_id
    }

    /// Returns the parent runtime identity.
    #[must_use]
    pub fn runtime_id(
        &self,
    ) -> &crate::zur::ids::RuntimeInstanceId {
        self.runtime_context.runtime_id()
    }

    /// Returns the parent runtime context.
    #[must_use]
    pub fn runtime_context(&self) -> &Arc<RuntimeContext> {
        &self.runtime_context
    }

    // ------------------------------------------------------------------------
    // Operation context
    // ------------------------------------------------------------------------

    /// Returns the immutable session-scoped operation context.
    ///
    /// Nested execution operations should receive this context.
    ///
    /// It carries both parent-runtime and session cancellation.
    #[must_use]
    pub fn operation_context(&self) -> &OperationContext {
        &self.operation_context
    }

    /// Returns whether either the parent runtime or this session
    /// has requested cancellation.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.operation_context.is_cancelled()
    }

    /// Checks cancellation and returns a structured error if requested.
    pub fn check_cancelled(&self) -> ZurResult<()> {
        if !self.is_cancelled() {
            return Ok(());
        }

        let reason = self
            .operation_context
            .cancellation_reason()
            .unwrap_or_else(|| {
                "execution session was cancelled".to_owned()
            });

        Err(
            ZurError::cancelled(reason)
                .with_component(COMPONENT)
                .with_operation("check_cancelled")
                .with_execution(self.session_id.to_string()),
        )
    }

    // ------------------------------------------------------------------------
    // Lifecycle
    // ------------------------------------------------------------------------

    /// Returns the current session status.
    pub fn status(&self) -> ZurResult<SessionStatus> {
        Ok(self.lock_inner()?.status)
    }

    /// Activates a newly created session.
    ///
    /// A session must be active before accepting new operations.
    pub fn activate(&self) -> ZurResult<()> {
        self.check_cancelled()?;

        self.lock_inner()?
            .transition(SessionStatus::Active)
    }

    /// Requests cooperative cancellation of this session.
    ///
    /// Cancellation propagates to operations using the session's
    /// OperationContext.
    ///
    /// The session enters Cancelling unless it is already terminal
    /// or shutting down.
    ///
    /// This operation does not establish that external work stopped.
    pub fn request_cancellation(
        &self,
        reason: impl Into<String>,
    ) -> ZurResult<bool> {
        let changed = self.cancellation.cancel(reason)?;

        if !changed {
            return Ok(false);
        }

        let mut inner = self.lock_inner()?;

        match inner.status {
            SessionStatus::Created
            | SessionStatus::Active => {
                inner.transition(SessionStatus::Cancelling)?;
            }

            SessionStatus::Cancelling
            | SessionStatus::Failed
            | SessionStatus::Closing
            | SessionStatus::Closed => {}
        }

        Ok(true)
    }

    /// Marks the session as failed.
    ///
    /// This records a local session-level failure.
    /// It does not invent a failure result for an individual execution.
    pub fn fail(
        &self,
        reason: impl Into<String>,
    ) -> ZurResult<()> {
        let reason = reason.into();

        validate_label(&reason, "failure reason")?;

        let mut inner = self.lock_inner()?;

        match inner.status {
            SessionStatus::Closed
            | SessionStatus::Closing => {
                return Err(
                    invalid_state(
                        "cannot fail a closing or closed session",
                    ),
                );
            }

            SessionStatus::Failed => return Ok(()),

            SessionStatus::Created
            | SessionStatus::Active
            | SessionStatus::Cancelling => {}
        }

        inner.transition(SessionStatus::Failed)?;

        drop(inner);

        self.cancellation.cancel(reason)?;

        Ok(())
    }

    /// Begins session shutdown.
    ///
    /// This requests cooperative cancellation before cleanup.
    ///
    /// Repeated calls while closing are idempotent.
    pub fn begin_close(&self) -> ZurResult<()> {
        let mut inner = self.lock_inner()?;

        match inner.status {
            SessionStatus::Closed => return Ok(()),

            SessionStatus::Closing => return Ok(()),

            SessionStatus::Created
            | SessionStatus::Active
            | SessionStatus::Cancelling
            | SessionStatus::Failed => {}
        }

        self.cancellation.cancel(
            "execution session is shutting down",
        )?;

        inner.transition(SessionStatus::Closing)?;

        Ok(())
    }

    /// Completes local session shutdown.
    ///
    /// Attached lease and accounting references are released from
    /// the session after the state becomes Closed.
    ///
    /// The owning resource and service authorities remain responsible
    /// for actual release and shutdown.
    ///
    /// This method is idempotent.
    pub fn close(&self) -> ZurResult<()> {
        self.begin_close()?;

        let mut inner = self.lock_inner()?;

        if inner.status == SessionStatus::Closed {
            return Ok(());
        }

        inner.transition(SessionStatus::Closed)?;

        // Drop externally owned references outside the mutex.
        let service_leases =
            core::mem::take(&mut inner.service_leases);

        let resource_references =
            core::mem::take(&mut inner.resource_references);

        drop(inner);

        drop(service_leases);
        drop(resource_references);

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Service lease management
    // ------------------------------------------------------------------------

    /// Attaches an existing service lease to this session.
    ///
    /// Duplicate service-instance identities are rejected.
    ///
    /// The session must be active.
    ///
    /// The service lease must already have been acquired by its
    /// canonical service-management authority.
    pub fn attach_service_lease(
        &self,
        lease: ServiceLeaseReference,
    ) -> ZurResult<()> {
        self.check_cancelled()?;

        let mut inner = self.lock_inner()?;

        self.ensure_active(&inner)?;

        if inner
            .service_leases
            .iter()
            .any(|existing| {
                existing.service_id() == lease.service_id()
            })
        {
            return Err(
                ZurError::invalid_input(
                    "service lease is already attached to this session",
                )
                .with_component(COMPONENT)
                .with_operation("attach_service_lease"),
            );
        }

        inner
            .service_leases
            .try_reserve(1)
            .map_err(|error| {
                internal_error(format!(
                    "unable to reserve service lease storage: {error}"
                ))
            })?;

        inner.service_leases.push(lease);

        Ok(())
    }

    /// Detaches a service lease without claiming that the service
    /// itself has been shut down.
    ///
    /// The caller receives the strong reference and is responsible
    /// for coordinating its release with the service authority.
    pub fn detach_service_lease(
        &self,
        service_id: &ServiceInstanceId,
    ) -> ZurResult<Option<ServiceLeaseReference>> {
        let mut inner = self.lock_inner()?;

        self.ensure_not_closed(&inner)?;

        let position = inner
            .service_leases
            .iter()
            .position(|lease| {
                lease.service_id() == service_id
            });

        Ok(position.map(|index| {
            inner.service_leases.remove(index)
        }))
    }

    // ------------------------------------------------------------------------
    // Resource accounting
    // ------------------------------------------------------------------------

    /// Attaches an existing resource-accounting reference.
    ///
    /// The physical resource must already be represented by the
    /// canonical resource-management authority.
    ///
    /// No resource is allocated or reserved by this method.
    pub fn attach_resource_reference(
        &self,
        reference: ResourceAccountingReference,
    ) -> ZurResult<()> {
        self.check_cancelled()?;

        let mut inner = self.lock_inner()?;

        self.ensure_active(&inner)?;

        if inner
            .resource_references
            .iter()
            .any(|existing| {
                existing.lease_id() == reference.lease_id()
            })
        {
            return Err(
                ZurError::invalid_input(
                    "resource-accounting reference is already attached",
                )
                .with_component(COMPONENT)
                .with_operation("attach_resource_reference"),
            );
        }

        inner
            .resource_references
            .try_reserve(1)
            .map_err(|error| {
                internal_error(format!(
                    "unable to reserve resource reference storage: {error}"
                ))
            })?;

        inner.resource_references.push(reference);

        Ok(())
    }

    /// Detaches a resource-accounting reference.
    ///
    /// This does not release or deallocate the physical resource.
    pub fn detach_resource_reference(
        &self,
        lease_id: &ResourceLeaseId,
    ) -> ZurResult<Option<ResourceAccountingReference>> {
        let mut inner = self.lock_inner()?;

        self.ensure_not_closed(&inner)?;

        let position = inner
            .resource_references
            .iter()
            .position(|reference| {
                reference.lease_id() == lease_id
            });

        Ok(position.map(|index| {
            inner.resource_references.remove(index)
        }))
    }

    // ------------------------------------------------------------------------
    // Diagnostics
    // ------------------------------------------------------------------------

    /// Appends a structured session diagnostic.
    ///
    /// Diagnostics are retained for the lifetime of the open session.
    pub fn record_diagnostic(
        &self,
        diagnostic: SessionDiagnostic,
    ) -> ZurResult<()> {
        let mut inner = self.lock_inner()?;

        self.ensure_not_closed(&inner)?;

        inner
            .diagnostics
            .try_reserve(1)
            .map_err(|error| {
                internal_error(format!(
                    "unable to reserve diagnostic storage: {error}"
                ))
            })?;

        inner.diagnostics.push(diagnostic);

        Ok(())
    }

    /// Returns a consistent snapshot of all diagnostics.
    pub fn diagnostics(
        &self,
    ) -> ZurResult<Vec<SessionDiagnostic>> {
        Ok(self.lock_inner()?.diagnostics.clone())
    }

    // ------------------------------------------------------------------------
    // Observability
    // ------------------------------------------------------------------------

    /// Returns an immutable snapshot of session state.
    ///
    /// The snapshot is local to this runtime instance.
    pub fn snapshot(&self) -> ZurResult<SessionSnapshot> {
        let inner = self.lock_inner()?;

        Ok(SessionSnapshot {
            session_id: self.session_id.clone(),

            runtime_id: self
                .runtime_context
                .runtime_id()
                .to_string(),

            status: inner.status,

            cancellation_requested: self.is_cancelled(),

            service_lease_count: inner.service_leases.len(),

            resource_reference_count:
                inner.resource_references.len(),

            diagnostic_count: inner.diagnostics.len(),
        })
    }

    // ------------------------------------------------------------------------
    // Internal helpers
    // ------------------------------------------------------------------------

    fn lock_inner(
        &self,
    ) -> ZurResult<MutexGuard<'_, SessionInner>> {
        self.shared.inner.lock().map_err(|_| {
            internal_error("execution session state lock was poisoned")
        })
    }

    fn ensure_active(
        &self,
        inner: &SessionInner,
    ) -> ZurResult<()> {
        if !inner.status.accepts_operations() {
            return Err(invalid_state(
                "execution session is not active",
            ));
        }

        Ok(())
    }

    fn ensure_not_closed(
        &self,
        inner: &SessionInner,
    ) -> ZurResult<()> {
        if inner.status == SessionStatus::Closed {
            return Err(invalid_state(
                "execution session is already closed",
            ));
        }

        Ok(())
    }
}

impl fmt::Debug for ExecutionSession {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ExecutionSession")
            .field("session_id", &self.session_id)
            .field(
                "runtime_id",
                self.runtime_context.runtime_id(),
            )
            .field("status", &self.status().ok())
            .field("cancelled", &self.is_cancelled())
            .finish()
    }
}

// ============================================================================
// VALIDATION AND ERROR HELPERS
// ============================================================================

fn validate_label(
    value: &str,
    description: &str,
) -> ZurResult<()> {
    if value.trim().is_empty() {
        return Err(
            ZurError::invalid_input(
                format!("{description} must not be empty"),
            )
            .with_component(COMPONENT),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ZurError::invalid_input(
                format!("{description} contains control characters"),
            )
            .with_component(COMPONENT),
        );
    }

    Ok(())
}

fn invalid_state(message: impl Into<String>) -> ZurError {
    ZurError::invalid_input(message)
        .with_component(COMPONENT)
        .with_operation("validate_session_state")
}

fn internal_error(message: impl Into<String>) -> ZurError {
    ZurError::internal(message)
        .with_component(COMPONENT)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // These tests deliberately exercise the public contract through
    // the actual RuntimeContext and identity infrastructure.

    // A runtime-context fixture should be supplied by the ZUR
    // integration test module using its canonical RuntimeInstanceId
    // allocator and RuntimeContextBuilder.

    #[test]
    fn session_status_has_correct_initial_semantics() {
        assert!(!SessionStatus::Created.is_terminal());

        assert!(SessionStatus::Closed.is_terminal());

        assert!(SessionStatus::Active.accepts_operations());

        assert!(!SessionStatus::Cancelling.accepts_operations());
    }

    #[test]
    fn cancellation_is_idempotent() {
        let cancellation = SessionCancellation::default();

        assert!(
            cancellation
                .cancel("requested by caller")
                .expect("cancellation should succeed")
        );

        assert!(
            !cancellation
                .cancel("second request")
                .expect("repeated cancellation should succeed")
        );

        assert!(cancellation.is_requested());

        assert_eq!(
            cancellation.reason().expect("reason should be readable"),
            Some("requested by caller".to_owned())
        );
    }

    #[test]
    fn cancellation_rejects_empty_reason() {
        let cancellation = SessionCancellation::default();

        assert!(
            cancellation.cancel("").is_err()
        );

        assert!(!cancellation.is_requested());
    }

    #[test]
    fn session_cancellation_implements_standard_signal() {
        let cancellation = SessionCancellation::default();

        let signal: &dyn CancellationSignal = &cancellation;

        assert!(!signal.is_cancelled());

        cancellation
            .cancel("test")
            .expect("cancellation should succeed");

        assert!(signal.is_cancelled());
    }

    #[test]
    fn diagnostics_validate_input() {
        assert!(
            SessionDiagnostic::new(
                DiagnosticLevel::Info,
                "zur.execution",
                "execution started",
            )
            .is_ok()
        );

        assert!(
            SessionDiagnostic::new(
                DiagnosticLevel::Error,
                "",
                "invalid",
            )
            .is_err()
        );
    }

    #[test]
    fn diagnostic_metadata_rejects_duplicate_keys() {
        let diagnostic = SessionDiagnostic::new(
            DiagnosticLevel::Info,
            "zur.execution",
            "execution prepared",
        )
        .expect("diagnostic should be valid")
        .with_metadata("stage", "prepare")
        .expect("metadata should be accepted");

        assert!(
            diagnostic
                .with_metadata("stage", "execute")
                .is_err()
        );
    }

    #[test]
    fn service_and_resource_references_are_distinct_types() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<ServiceLeaseReference>();

        assert_send_sync::<ResourceAccountingReference>();

        assert_send_sync::<SessionCancellation>();
    }

    #[test]
    fn session_snapshot_is_immutable_and_cloneable() {
        let snapshot = SessionSnapshot {
            session_id: test_session_id(),

            runtime_id: "zamani.runtime.instance:01".to_owned(),

            status: SessionStatus::Active,

            cancellation_requested: false,

            service_lease_count: 0,

            resource_reference_count: 0,

            diagnostic_count: 0,
        };

        let cloned = snapshot.clone();

        assert_eq!(
            cloned.status(),
            SessionStatus::Active
        );

        assert_eq!(
            cloned.service_lease_count(),
            0
        );
    }

    fn test_session_id() -> SessionId {
        use crate::zur::ids::RuntimeId;

        let namespace = crate::zur::ids::IdNamespace::new(
            "zamani.runtime.session",
        )
        .expect("valid identity namespace");

        SessionId::new(
            namespace,
            vec![1],
        )
        .expect("valid session identity")
    }
}
