
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Operation Context
//!
//! Path: src/zhal/context.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides an immutable, shareable context for operations crossing
//! the universal hardware abstraction boundary.
//!
//! ZHAL contexts transport operation metadata, cancellation,
//! deadline observations, provider references, correlation
//! references and opaque security/diagnostic context references.
//!
//! This module is an integration boundary, not an execution engine.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - ZhalContext.
//! - ZhalContextBuilder.
//! - ZHAL-specific contextual metadata.
//! - Provider association references.
//! - Operation correlation references.
//! - Caller, security and diagnostic references.
//! - The deadline observation integration contract.
//! - ZHAL context validation.
//! - Child ZHAL context construction.
//! - ZHAL operation-state checking.
//!
//! This module does NOT own:
//!
//! - Standard-library OperationContext.
//! - Cancellation token implementation.
//! - Runtime cancellation state.
//! - Distributed cancellation implementation.
//! - Runtime deadline implementation.
//! - Trace ID or span ID generation.
//! - Provider identity definitions.
//! - Provider registration or lifecycle.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Resource scheduling.
//! - Device sessions.
//! - Authentication.
//! - Authorization.
//! - Security policy evaluation.
//! - Semantic analysis.
//! - Execution planning.
//! - Quantum routing.
//! - HDL realization.
//! - Distributed execution.
//!
//! ## EXISTING AUTHORITIES
//!
//! Reuses:
//!
//! - crate::stdlib::context::OperationContext
//! - crate::stdlib::context::OperationContextBuilder
//! - crate::stdlib::context::CancellationSignal
//! - crate::stdlib::provider::ProviderId
//! - crate::stdlib::error::{Error, ErrorKind, Result}
//!
//! Runtime and distributed integrations provide deadline observations.
//!
//! Distributed tracing supplies correlation metadata through its
//! existing tracing contracts.
//!
//! Security and authorization remain owned by their existing authorities.
//!
//! ## GRAMMAR BOUNDARY
//!
//! Relevant grammar:
//!
//! - grammar/execution/execution-context.g4
//! - grammar/execution/distributed-execution.g4
//!
//! Normative specifications:
//!
//! - grammar/specification/execution-model.md
//! - grammar/specification/semantic-model.md
//!
//! The grammar defines source-level execution intent.
//!
//! This module does not parse grammar productions, interpret source
//! expressions, perform semantic analysis or choose physical hardware.
//!
//! A frontend or runtime adapter may attach already validated,
//! typed information through the contextual metadata and extension APIs.
//!
//! ## POCO-REAF
//!
//! No arbitrary hardware or computational ceilings are introduced.
//!
//! There are no fixed limits on:
//!
//! - hardware devices;
//! - providers;
//! - processors;
//! - cores;
//! - threads;
//! - accelerators;
//! - quantum processors;
//! - qubits;
//! - memory;
//! - storage;
//! - computational domains;
//! - distributed participants;
//! - operation contexts;
//! - extension types;
//! - metadata lengths.
//!
//! Memory consumption is governed by actual available resources
//! and platform representational limits.
//!
//! No implementation can promise literal infinite physical resources.
//!
//! ## IMMUTABILITY
//!
//! ZhalContext is immutable after construction.
//!
//! Cloning a context shares its internal OperationContext,
//! metadata and deadline observer through Arc.
//!
//! Context extensions are delegated to the standard-library
//! OperationContext implementation.
//!
//! ## CANCELLATION
//!
//! Cancellation is cooperative.
//!
//! The canonical standard-library cancellation contract is reused.
//!
//! No competing cancellation token or cancellation state machine
//! is introduced.
//!
//! Cancellation does not prove that physical hardware execution,
//! remote execution or an external process has stopped.
//!
//! ## DEADLINES
//!
//! Deadlines are observed through the DeadlineObservation trait.
//!
//! The actual deadline implementation belongs to the runtime
//! or distributed execution authority.
//!
//! A monotonic deadline is process-local. It must not be serialized
//! or compared directly between different machines.
//!
//! Remote propagation must translate remaining duration using
//! the responsible transport/runtime contract.
//!
//! ## SECURITY
//!
//! Caller, security and diagnostic references are descriptive
//! references, not authorization grants.
//!
//! Never store credentials, access tokens, passwords or private
//! keys in context metadata.
//!
//! Typed extensions must not automatically be trusted.
//!
//! ## ERROR CONTRACT
//!
//! Existing standard-library errors are reused.
//!
//! Cancellation returns ErrorKind::Cancelled.
//!
//! Expired deadlines return ErrorKind::TimedOut.
//!
//! Invalid contextual metadata returns ErrorKind::InvalidInput.
//!
//! No ordinary invalid input should cause a panic.
//!
//! ## FREEZE CRITERIA
//!
//! 1. Standard-library context remains the cancellation authority.
//! 2. Provider identity reuses ProviderId.
//! 3. Deadline observation is externally implemented.
//! 4. Child contexts preserve inherited cancellation.
//! 5. Child contexts preserve applicable deadline observations.
//! 6. Security references are not treated as authorization.
//! 7. Correlation references are not generated here.
//! 8. Extensions reuse standard-library typed storage.
//! 9. No arbitrary hardware ceilings are introduced.
//! 10. Tests cover context construction, propagation and validation.

use core::fmt;

use std::sync::Arc;
use std::time::Duration;

use crate::stdlib::context::{
    CancellationSignal,
    OperationContext,
    OperationContextBuilder,
};

use crate::stdlib::error::{
    Error,
    ErrorKind,
    Result,
};

use crate::stdlib::provider::ProviderId;

// ============================================================================
// DEADLINE OBSERVATION CONTRACT
// ============================================================================

/// Read-only observation of an externally owned operation deadline.
///
/// This interface does not implement deadline creation, cancellation,
/// timeout scheduling, task interruption or execution management.
///
/// The runtime or distributed execution authority supplies the
/// implementation.
///
/// Implementations must be thread-safe because ZHAL contexts may
/// be shared across execution threads.
///
/// A deadline represents a local execution-clock boundary.
/// Implementations must not treat local monotonic deadlines as
/// portable timestamps.
///
/// # Distributed integration
///
/// The distributed execution subsystem can implement this contract
/// by adapting its existing ExecutionControl.
///
/// The adapter must delegate to the existing deadline implementation
/// rather than creating another deadline state machine.
pub trait DeadlineObservation: Send + Sync + 'static {
    /// Returns whether the externally owned deadline has expired.
    fn is_expired(&self) -> bool;

    /// Returns the remaining duration.
    ///
    /// Returns None when no deadline is configured.
    ///
    /// Returns Some(Duration::ZERO) when the deadline has expired.
    fn remaining(&self) -> Option<Duration>;
}

// ============================================================================
// ZHAL CONTEXT METADATA
// ============================================================================

/// Descriptive metadata associated with a ZHAL operation.
///
/// This structure contains references and identifiers only.
///
/// It does not contain physical device handles, resource reservations,
/// credentials, authentication tokens or mutable execution state.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct ZhalContextMetadata {
    correlation_id: Option<Arc<str>>,
    caller_reference: Option<Arc<str>>,
    diagnostic_reference: Option<Arc<str>>,
    security_context_reference: Option<Arc<str>>,
    provider_id: Option<ProviderId>,
}

impl ZhalContextMetadata {
    /// Returns the optional operation correlation reference.
    ///
    /// Correlation identity is supplied by the responsible caller
    /// or observability authority.
    #[must_use]
    pub fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }

    /// Returns the optional caller reference.
    ///
    /// This is descriptive metadata, not authenticated identity.
    #[must_use]
    pub fn caller_reference(&self) -> Option<&str> {
        self.caller_reference.as_deref()
    }

    /// Returns the optional diagnostic reference.
    ///
    /// This reference may identify an externally managed diagnostic
    /// or observability context.
    #[must_use]
    pub fn diagnostic_reference(&self) -> Option<&str> {
        self.diagnostic_reference.as_deref()
    }

    /// Returns the optional security-context reference.
    ///
    /// This reference does not establish authentication,
    /// authorization or provider trust.
    #[must_use]
    pub fn security_context_reference(&self) -> Option<&str> {
        self.security_context_reference.as_deref()
    }

    /// Returns the associated provider identity.
    ///
    /// Provider identity is reused from the standard library.
    #[must_use]
    pub fn provider_id(&self) -> Option<&ProviderId> {
        self.provider_id.as_ref()
    }
}

impl fmt::Debug for ZhalContextMetadata {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ZhalContextMetadata")
            .field("correlation_id", &self.correlation_id)
            .field("caller_reference", &self.caller_reference)
            .field("diagnostic_reference", &self.diagnostic_reference)
            .field(
                "security_context_reference",
                &self.security_context_reference.as_ref().map(|_| "[REDACTED REFERENCE]"),
            )
            .field("provider_id", &self.provider_id)
            .finish()
    }
}

// ============================================================================
// ZHAL OPERATION CONTEXT
// ============================================================================

/// Immutable, shareable context for universal hardware operations.
///
/// ZhalContext wraps the existing standard-library OperationContext.
///
/// It adds ZHAL-specific descriptive metadata and an optional
/// externally managed deadline observation.
///
/// It does not replace or duplicate the standard-library context.
///
/// # Examples
///
/// ```
/// use zamani::zhal::context::ZhalContext;
///
/// let context = ZhalContext::builder()
///     .with_component("zhal")
///     .unwrap()
///     .with_operation("hardware.discovery")
///     .unwrap()
///     .with_correlation_id("request-001")
///     .unwrap()
///     .build();
///
/// assert_eq!(
///     context.correlation_id(),
///     Some("request-001")
/// );
///
/// assert!(context.check_active().is_ok());
/// ```
#[derive(Clone)]
pub struct ZhalContext {
    operation_context: OperationContext,
    metadata: Arc<ZhalContextMetadata>,
    deadline: Option<Arc<dyn DeadlineObservation>>,
}

impl ZhalContext {
    /// Creates a new ZHAL context builder.
    #[must_use]
    pub fn builder() -> ZhalContextBuilder {
        ZhalContextBuilder::new()
    }

    /// Returns the underlying canonical standard-library context.
    ///
    /// This provides integration with existing standard-library,
    /// provider and runtime APIs.
    #[must_use]
    pub fn operation_context(&self) -> &OperationContext {
        &self.operation_context
    }

    /// Returns the immutable ZHAL-specific metadata.
    #[must_use]
    pub fn metadata(&self) -> &ZhalContextMetadata {
        &self.metadata
    }

    /// Returns the optional operation correlation reference.
    #[must_use]
    pub fn correlation_id(&self) -> Option<&str> {
        self.metadata.correlation_id()
    }

    /// Returns the optional caller reference.
    #[must_use]
    pub fn caller_reference(&self) -> Option<&str> {
        self.metadata.caller_reference()
    }

    /// Returns the optional diagnostic reference.
    #[must_use]
    pub fn diagnostic_reference(&self) -> Option<&str> {
        self.metadata.diagnostic_reference()
    }

    /// Returns the optional security-context reference.
    #[must_use]
    pub fn security_context_reference(&self) -> Option<&str> {
        self.metadata.security_context_reference()
    }

    /// Returns the optional provider identity.
    #[must_use]
    pub fn provider_id(&self) -> Option<&ProviderId> {
        self.metadata.provider_id()
    }

    /// Returns the optional externally owned deadline observer.
    #[must_use]
    pub fn deadline_observation(
        &self,
    ) -> Option<&Arc<dyn DeadlineObservation>> {
        self.deadline.as_ref()
    }

    /// Returns the remaining deadline duration.
    ///
    /// None means that no deadline was configured.
    #[must_use]
    pub fn remaining(&self) -> Option<Duration> {
        self.deadline
            .as_ref()
            .and_then(|deadline| deadline.remaining())
    }

    /// Returns whether the deadline has expired.
    ///
    /// A context without a deadline is not deadline-expired.
    #[must_use]
    pub fn is_deadline_expired(&self) -> bool {
        self.deadline
            .as_ref()
            .is_some_and(|deadline| deadline.is_expired())
    }

    /// Returns whether cancellation has been requested.
    ///
    /// Delegates to the canonical standard-library context.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.operation_context.is_cancelled()
    }

    /// Returns the existing read-only cancellation signal.
    ///
    /// No new cancellation mechanism is created.
    #[must_use]
    pub fn cancellation_signal(&self) -> Arc<dyn CancellationSignal> {
        self.operation_context.cancellation_signal()
    }

    /// Checks whether cancellation has been requested.
    ///
    /// Delegates to the existing standard-library cancellation
    /// implementation.
    ///
    /// # Errors
    ///
    /// Returns the canonical standard-library cancellation error.
    pub fn check_cancelled(&self) -> Result<()> {
        self.operation_context.check_cancelled()
    }

    /// Checks whether the deadline has expired.
    ///
    /// This check performs no dispatch and makes no assertion about
    /// an operation that has already been submitted to a provider.
    ///
    /// # Errors
    ///
    /// Returns ErrorKind::TimedOut if the deadline has expired.
    pub fn check_deadline(&self) -> Result<()> {
        if !self.is_deadline_expired() {
            return Ok(());
        }

        Err(
            Error::new(
                ErrorKind::TimedOut,
                "the ZHAL operation deadline has expired",
            )
            .in_component("zhal.context")
            .during_operation("check_deadline"),
        )
    }

    /// Checks cancellation and deadline conditions.
    ///
    /// Cancellation takes precedence when both conditions are
    /// observed during the same check.
    ///
    /// This method does not check:
    ///
    /// - provider availability;
    /// - device availability;
    /// - hardware health;
    /// - authorization;
    /// - capability compatibility;
    /// - resource feasibility;
    /// - reservation status.
    ///
    /// Those checks belong to their respective authorities.
    ///
    /// # Errors
    ///
    /// Returns a standard-library cancellation or timeout error.
    pub fn check_active(&self) -> Result<()> {
        self.check_cancelled()?;
        self.check_deadline()
    }

    /// Creates a builder for a child hardware operation.
    ///
    /// The child inherits:
    ///
    /// - standard-library component context;
    /// - standard-library cancellation;
    /// - existing standard-library typed extensions;
    /// - caller reference;
    /// - security-context reference;
    /// - diagnostic reference;
    /// - provider identity;
    /// - deadline observation.
    ///
    /// The child inherits the correlation reference by default.
    /// A tracing integration may replace it with a child span
    /// or other externally generated correlation reference.
    ///
    /// The child operation name must be supplied independently.
    #[must_use]
    pub fn child_builder(&self) -> ZhalContextBuilder {
        ZhalContextBuilder {
            operation_context: self.operation_context.child_builder(),
            metadata: (*self.metadata).clone(),
            deadline: self.deadline.clone(),
        }
    }

    /// Retrieves an existing typed extension.
    ///
    /// Extension storage is owned by the standard-library context.
    #[must_use]
    pub fn extension<T>(&self) -> Option<&T>
    where
        T: std::any::Any + Send + Sync,
    {
        self.operation_context.extension::<T>()
    }

    /// Retrieves a shared typed extension.
    #[must_use]
    pub fn extension_arc<T>(&self) -> Option<Arc<T>>
    where
        T: std::any::Any + Send + Sync,
    {
        self.operation_context.extension_arc::<T>()
    }

    /// Returns whether an extension of the requested type exists.
    #[must_use]
    pub fn has_extension<T>(&self) -> bool
    where
        T: std::any::Any + Send + Sync,
    {
        self.operation_context.has_extension::<T>()
    }

    /// Returns the number of attached extensions.
    #[must_use]
    pub fn extension_count(&self) -> usize {
        self.operation_context.extension_count()
    }
}

impl Default for ZhalContext {
    fn default() -> Self {
        ZhalContextBuilder::new().build()
    }
}

impl fmt::Debug for ZhalContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ZhalContext")
            .field("operation_context", &self.operation_context)
            .field("metadata", &self.metadata)
            .field(
                "deadline_expired",
                &self.is_deadline_expired(),
            )
            .field("remaining", &self.remaining())
            .finish()
    }
}

// ============================================================================
// BUILDER
// ============================================================================

/// Builder for an immutable ZHAL operation context.
///
/// Construction performs no hardware discovery, provider registration,
/// device access, resource allocation, authentication, authorization
/// or runtime initialization.
pub struct ZhalContextBuilder {
    operation_context: OperationContextBuilder,
    metadata: ZhalContextMetadata,
    deadline: Option<Arc<dyn DeadlineObservation>>,
}

impl ZhalContextBuilder {
    /// Creates a new ZHAL context builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            operation_context: OperationContext::builder(),
            metadata: ZhalContextMetadata::default(),
            deadline: None,
        }
    }

    /// Sets the standard-library component identity.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput for invalid metadata.
    pub fn with_component(
        mut self,
        component: impl Into<String>,
    ) -> Result<Self> {
        self.operation_context =
            self.operation_context.with_component(component)?;

        Ok(self)
    }

    /// Sets the operation identity.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput for invalid metadata.
    pub fn with_operation(
        mut self,
        operation: impl Into<String>,
    ) -> Result<Self> {
        self.operation_context =
            self.operation_context.with_operation(operation)?;

        Ok(self)
    }

    /// Sets the logical operation scope.
    ///
    /// A scope is not a physical hardware location.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput for invalid metadata.
    pub fn with_scope(
        mut self,
        scope: impl Into<String>,
    ) -> Result<Self> {
        self.operation_context =
            self.operation_context.with_scope(scope)?;

        Ok(self)
    }

    /// Sets the canonical standard-library cancellation signal.
    ///
    /// This delegates cancellation ownership to the supplied
    /// standard-library CancellationSignal implementation.
    #[must_use]
    pub fn with_cancellation(
        mut self,
        cancellation: Arc<dyn CancellationSignal>,
    ) -> Self {
        self.operation_context =
            self.operation_context.with_cancellation(cancellation);

        self
    }

    /// Sets an externally generated operation correlation reference.
    ///
    /// This method does not generate trace IDs, span IDs or globally
    /// unique identifiers.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput for empty, whitespace-only or control-
    /// character-containing references.
    pub fn with_correlation_id(
        mut self,
        correlation_id: impl Into<String>,
    ) -> Result<Self> {
        self.metadata.correlation_id = Some(
            validated_reference(
                correlation_id,
                "correlation_id",
            )?,
        );

        Ok(self)
    }

    /// Sets the descriptive caller reference.
    ///
    /// This reference is not proof of authenticated identity.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput for invalid metadata.
    pub fn with_caller_reference(
        mut self,
        reference: impl Into<String>,
    ) -> Result<Self> {
        self.metadata.caller_reference = Some(
            validated_reference(reference, "caller_reference")?,
        );

        Ok(self)
    }

    /// Sets the diagnostic or observability context reference.
    ///
    /// Actual diagnostic and tracing services remain externally owned.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput for invalid metadata.
    pub fn with_diagnostic_reference(
        mut self,
        reference: impl Into<String>,
    ) -> Result<Self> {
        self.metadata.diagnostic_reference = Some(
            validated_reference(
                reference,
                "diagnostic_reference",
            )?,
        );

        Ok(self)
    }

    /// Sets the external security-context reference.
    ///
    /// This does not authenticate a caller or authorize an operation.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput for invalid metadata.
    pub fn with_security_context_reference(
        mut self,
        reference: impl Into<String>,
    ) -> Result<Self> {
        self.metadata.security_context_reference = Some(
            validated_reference(
                reference,
                "security_context_reference",
            )?,
        );

        Ok(self)
    }

    /// Associates an existing canonical provider identity.
    ///
    /// Provider registration and validation remain external.
    #[must_use]
    pub fn with_provider_id(
        mut self,
        provider_id: ProviderId,
    ) -> Self {
        self.metadata.provider_id = Some(provider_id);
        self
    }

    /// Associates an externally owned deadline observer.
    ///
    /// The supplied implementation remains responsible for deadline
    /// creation, expiration and monotonic-clock behavior.
    ///
    /// This method does not create a deadline or cancellation token.
    #[must_use]
    pub fn with_deadline(
        mut self,
        deadline: Arc<dyn DeadlineObservation>,
    ) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Attaches a typed extension to the canonical operation context.
    ///
    /// Duplicate concrete extension types are rejected by the
    /// standard-library context.
    ///
    /// # Errors
    ///
    /// Returns the existing standard-library extension error.
    pub fn with_extension<T>(
        mut self,
        extension: T,
    ) -> Result<Self>
    where
        T: std::any::Any + Send + Sync,
    {
        self.operation_context =
            self.operation_context.with_extension(extension)?;

        Ok(self)
    }

    /// Attaches a shared typed extension.
    ///
    /// # Errors
    ///
    /// Returns the existing standard-library extension error.
    pub fn with_shared_extension<T>(
        mut self,
        extension: Arc<T>,
    ) -> Result<Self>
    where
        T: std::any::Any + Send + Sync,
    {
        self.operation_context =
            self.operation_context.with_shared_extension(extension)?;

        Ok(self)
    }

    /// Explicitly replaces a typed extension.
    ///
    /// Replacement semantics are delegated to the standard library.
    #[must_use]
    pub fn replace_extension<T>(
        mut self,
        extension: T,
    ) -> Self
    where
        T: std::any::Any + Send + Sync,
    {
        self.operation_context =
            self.operation_context.replace_extension(extension);

        self
    }

    /// Builds the immutable ZHAL context.
    #[must_use]
    pub fn build(self) -> ZhalContext {
        ZhalContext {
            operation_context: self.operation_context.build(),
            metadata: Arc::new(self.metadata),
            deadline: self.deadline,
        }
    }
}

impl Default for ZhalContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for ZhalContextBuilder {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ZhalContextBuilder")
            .field("metadata", &self.metadata)
            .field("has_deadline", &self.deadline.is_some())
            .finish_non_exhaustive()
    }
}

// ============================================================================
// CHILD CONTEXT BUILDER INTEGRATION
// ============================================================================

impl ZhalContextBuilder {
    /// Constructs a child builder from an existing ZHAL context.
    ///
    /// Prefer ZhalContext::child_builder() for ordinary child creation.
    ///
    /// This method is provided for integrations that receive a context
    /// and need to construct a nested operation.
    #[must_use]
    pub fn from_parent(parent: &ZhalContext) -> Self {
        parent.child_builder()
    }
}

// ============================================================================
// VALIDATION
// ============================================================================

/// Validates a descriptive external reference.
///
/// References remain open-world strings.
///
/// No arbitrary maximum length, hardware-specific namespace,
/// vendor list or device limit is introduced.
///
/// Control characters are rejected to avoid unsafe diagnostic
/// and logging representations.
fn validated_reference(
    value: impl Into<String>,
    field: &'static str,
) -> Result<Arc<str>> {
    let value = value.into();

    if value.trim().is_empty() {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                format!("{field} must not be empty or whitespace-only"),
            )
            .in_component("zhal.context")
            .during_operation("validate_reference"),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                format!("{field} must not contain control characters"),
            )
            .in_component("zhal.context")
            .during_operation("validate_reference"),
        );
    }

    Ok(Arc::<str>::from(value))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{
        AtomicBool,
        Ordering,
    };

    use std::time::Duration;

    #[derive(Debug, Default)]
    struct TestCancellation {
        cancelled: AtomicBool,
    }

    impl TestCancellation {
        fn cancel(&self) {
            self.cancelled.store(true, Ordering::Release);
        }
    }

    impl CancellationSignal for TestCancellation {
        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::Acquire)
        }
    }

    #[derive(Debug)]
    struct TestDeadline {
        expired: AtomicBool,
        remaining: Duration,
    }

    impl TestDeadline {
        fn new(remaining: Duration) -> Self {
            Self {
                expired: AtomicBool::new(false),
                remaining,
            }
        }

        fn expire(&self) {
            self.expired.store(true, Ordering::Release);
        }
    }

    impl DeadlineObservation for TestDeadline {
        fn is_expired(&self) -> bool {
            self.expired.load(Ordering::Acquire)
        }

        fn remaining(&self) -> Option<Duration> {
            Some(if self.is_expired() {
                Duration::ZERO
            } else {
                self.remaining
            })
        }
    }

    #[test]
    fn default_context_is_valid() {
        let context = ZhalContext::default();

        assert!(context.check_active().is_ok());
        assert!(!context.is_cancelled());
        assert!(!context.is_deadline_expired());
        assert_eq!(context.correlation_id(), None);
        assert_eq!(context.provider_id(), None);
    }

    #[test]
    fn builder_preserves_standard_operation_metadata() {
        let context = ZhalContext::builder()
            .with_component("zhal")
            .unwrap()
            .with_operation("hardware.discovery")
            .unwrap()
            .with_scope("application.request")
            .unwrap()
            .build();

        assert_eq!(
            context.operation_context().component(),
            Some("zhal")
        );

        assert_eq!(
            context.operation_context().operation(),
            Some("hardware.discovery")
        );

        assert_eq!(
            context.operation_context().scope(),
            Some("application.request")
        );
    }

    #[test]
    fn correlation_reference_is_preserved() {
        let context = ZhalContext::builder()
            .with_correlation_id("request-123")
            .unwrap()
            .build();

        assert_eq!(
            context.correlation_id(),
            Some("request-123")
        );
    }

    #[test]
    fn caller_security_and_diagnostic_references_are_preserved() {
        let context = ZhalContext::builder()
            .with_caller_reference("caller.reference")
            .unwrap()
            .with_security_context_reference("security.reference")
            .unwrap()
            .with_diagnostic_reference("diagnostic.reference")
            .unwrap()
            .build();

        assert_eq!(
            context.caller_reference(),
            Some("caller.reference")
        );

        assert_eq!(
            context.security_context_reference(),
            Some("security.reference")
        );

        assert_eq!(
            context.diagnostic_reference(),
            Some("diagnostic.reference")
        );
    }

    #[test]
    fn provider_identity_uses_existing_standard_library_type() {
        let provider_id = ProviderId::new(
            "org.example.hardware"
        )
        .unwrap();

        let context = ZhalContext::builder()
            .with_provider_id(provider_id.clone())
            .build();

        assert_eq!(
            context.provider_id(),
            Some(&provider_id)
        );
    }

    #[test]
    fn cancellation_is_delegated_to_standard_library_context() {
        let cancellation = Arc::new(TestCancellation::default());

        let context = ZhalContext::builder()
            .with_cancellation(cancellation.clone())
            .build();

        assert!(!context.is_cancelled());

        cancellation.cancel();

        assert!(context.is_cancelled());

        assert_eq!(
            context.check_cancelled().unwrap_err().kind(),
            ErrorKind::Cancelled
        );
    }

    #[test]
    fn deadline_observation_is_external() {
        let deadline = Arc::new(
            TestDeadline::new(Duration::from_secs(30))
        );

        let context = ZhalContext::builder()
            .with_deadline(deadline.clone())
            .build();

        assert!(!context.is_deadline_expired());

        assert_eq!(
            context.remaining(),
            Some(Duration::from_secs(30))
        );

        deadline.expire();

        assert!(context.is_deadline_expired());

        assert_eq!(
            context.remaining(),
            Some(Duration::ZERO)
        );

        assert_eq!(
            context.check_deadline().unwrap_err().kind(),
            ErrorKind::TimedOut
        );
    }

    #[test]
    fn cancellation_takes_precedence_over_deadline() {
        let cancellation = Arc::new(TestCancellation::default());

        let deadline = Arc::new(
            TestDeadline::new(Duration::ZERO)
        );

        deadline.expire();
        cancellation.cancel();

        let context = ZhalContext::builder()
            .with_cancellation(cancellation)
            .with_deadline(deadline)
            .build();

        assert_eq!(
            context.check_active().unwrap_err().kind(),
            ErrorKind::Cancelled
        );
    }

    #[test]
    fn child_context_inherits_cancellation() {
        let cancellation = Arc::new(TestCancellation::default());

        let parent = ZhalContext::builder()
            .with_cancellation(cancellation.clone())
            .with_operation("hardware.discovery")
            .unwrap()
            .build();

        let child = parent
            .child_builder()
            .with_operation("hardware.device.inspect")
            .unwrap()
            .build();

        assert!(!child.is_cancelled());

        cancellation.cancel();

        assert!(parent.is_cancelled());
        assert!(child.is_cancelled());
    }

    #[test]
    fn child_context_inherits_deadline() {
        let deadline = Arc::new(
            TestDeadline::new(Duration::from_secs(10))
        );

        let parent = ZhalContext::builder()
            .with_deadline(deadline.clone())
            .build();

        let child = parent
            .child_builder()
            .with_operation("hardware.inspect")
            .unwrap()
            .build();

        deadline.expire();

        assert!(parent.is_deadline_expired());
        assert!(child.is_deadline_expired());
    }

    #[test]
    fn child_context_inherits_metadata() {
        let parent = ZhalContext::builder()
            .with_correlation_id("trace-001")
            .unwrap()
            .with_caller_reference("caller-001")
            .unwrap()
            .with_diagnostic_reference("diagnostic-001")
            .unwrap()
            .build();

        let child = parent
            .child_builder()
            .with_operation("hardware.inspect")
            .unwrap()
            .build();

        assert_eq!(
            child.correlation_id(),
            Some("trace-001")
        );

        assert_eq!(
            child.caller_reference(),
            Some("caller-001")
        );

        assert_eq!(
            child.diagnostic_reference(),
            Some("diagnostic-001")
        );
    }

    #[test]
    fn child_can_replace_correlation_reference() {
        let parent = ZhalContext::builder()
            .with_correlation_id("parent-trace")
            .unwrap()
            .build();

        let child = parent
            .child_builder()
            .with_correlation_id("child-span")
            .unwrap()
            .build();

        assert_eq!(
            parent.correlation_id(),
            Some("parent-trace")
        );

        assert_eq!(
            child.correlation_id(),
            Some("child-span")
        );
    }

    #[test]
    fn standard_library_extensions_are_reused() {
        #[derive(Debug, PartialEq, Eq)]
        struct HardwareRequest {
            description: String,
        }

        let context = ZhalContext::builder()
            .with_extension(HardwareRequest {
                description: "device observation".to_owned(),
            })
            .unwrap()
            .build();

        assert_eq!(
            context.extension::<HardwareRequest>(),
            Some(&HardwareRequest {
                description: "device observation".to_owned()
            })
        );
    }

    #[test]
    fn child_context_shares_existing_extensions() {
        #[derive(Debug, PartialEq, Eq)]
        struct HardwareRequest {
            id: String,
        }

        let parent = ZhalContext::builder()
            .with_extension(HardwareRequest {
                id: "request-001".to_owned(),
            })
            .unwrap()
            .build();

        let child = parent
            .child_builder()
            .with_operation("hardware.inspect")
            .unwrap()
            .build();

        assert_eq!(
            child.extension::<HardwareRequest>()
                .map(|request| request.id.as_str()),
            Some("request-001")
        );
    }

    #[test]
    fn invalid_correlation_reference_is_rejected() {
        assert!(
            ZhalContext::builder()
                .with_correlation_id("   ")
                .is_err()
        );
    }

    #[test]
    fn control_characters_are_rejected() {
        assert!(
            ZhalContext::builder()
                .with_caller_reference("caller\ninjected")
                .is_err()
        );
    }

    #[test]
    fn no_deadline_is_distinct_from_expired_deadline() {
        let context = ZhalContext::default();

        assert_eq!(context.remaining(), None);
        assert!(!context.is_deadline_expired());
        assert!(context.check_deadline().is_ok());
    }

    #[test]
    fn context_clones_share_the_same_metadata_and_state() {
        let cancellation = Arc::new(TestCancellation::default());

        let context = ZhalContext::builder()
            .with_cancellation(cancellation.clone())
            .with_correlation_id("shared-operation")
            .unwrap()
            .build();

        let clone = context.clone();

        cancellation.cancel();

        assert!(context.is_cancelled());
        assert!(clone.is_cancelled());

        assert_eq!(
            clone.correlation_id(),
            Some("shared-operation")
        );
    }
}
