
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Operation Context
//!
//! Path: `src/stdlib/context.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides a target-independent, immutable and shareable context for
//! standard-library operations.
//!
//! An operation context carries caller-supplied descriptive metadata,
//! a cooperative cancellation signal and typed extension services.
//!
//! It allows standard-library operations to receive common contextual
//! information without depending directly on compiler, runtime, hardware,
//! provider, security, policy or observability implementations.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - OperationContext.
//! - OperationContextBuilder.
//! - The standard-library cancellation observation contract.
//! - A never-cancelled signal.
//! - Caller-supplied operation metadata.
//! - Typed extension storage.
//! - Immutable context sharing.
//! - Child-context construction.
//! - Cancellation checking.
//!
//! This module does NOT own:
//!
//! - Source-language execution-context semantics.
//! - Parser context or parser-session state.
//! - Semantic analysis context.
//! - Compiler context or compilation-session state.
//! - Runtime execution context.
//! - Authentication or authorization.
//! - Identity verification.
//! - Policy definitions or policy evaluation.
//! - Distributed tracing or trace identity generation.
//! - Resource requirements or resource quantities.
//! - Capability definitions or capability discovery.
//! - Preference scoring.
//! - Constraint evaluation.
//! - Hint interpretation.
//! - Feasibility assessment.
//! - Provider registration or provider selection.
//! - Hardware discovery or allocation.
//! - Scheduling, routing or execution.
//! - Runtime deadlines.
//! - Global initialization or shutdown.
//!
//! These responsibilities remain with their canonical repository owners.
//!
//! ## ARCHITECTURAL AUTHORITIES
//!
//! Existing repository authorities include:
//!
//! - `src/frontend/parser/context.rs`:
//!   parser services and parser configuration.
//!
//! - `src/frontend/semantic/context.rs`:
//!   semantic configuration and semantic extension snapshots.
//!
//! - `src/compiler/context/`:
//!   compiler-operation context.
//!
//! - `src/ai/context.rs`:
//!   AI-specific service context.
//!
//! - `src/distributed/execution/cancellation.rs`:
//!   distributed cancellation and execution deadlines.
//!
//! - `src/distributed/observability/tracing.rs`:
//!   distributed tracing and trace propagation.
//!
//! - `src/stdlib/error.rs`:
//!   standard-library errors and result types.
//!
//! - `src/stdlib/capability.rs`:
//!   standard-library capability contracts.
//!
//! - `src/stdlib/requirement.rs`:
//!   standard-library requirement contracts.
//!
//! - `src/stdlib/preference.rs`:
//!   standard-library preference contracts.
//!
//! - `src/stdlib/constraint.rs`:
//!   standard-library constraint contracts.
//!
//! - `src/stdlib/hint.rs`:
//!   non-binding implementation hints.
//!
//! - `src/stdlib/feasibility.rs`:
//!   evidence-based feasibility reports.
//!
//! ## GRAMMAR BOUNDARY
//!
//! The source-language execution-context grammar is:
//!
//! `grammar/execution/execution-context.g4`
//!
//! Distributed execution intent is described by:
//!
//! `grammar/execution/distributed-execution.g4`
//!
//! The normative execution model is:
//!
//! `grammar/specification/execution-model.md`
//!
//! The semantic model is:
//!
//! `grammar/specification/semantic-model.md`
//!
//! Those files define source-language syntax and semantics.
//!
//! This module does not parse, represent or reinterpret their productions.
//!
//! A frontend or runtime adapter may place an already-established,
//! typed external representation into this context using the extension API.
//!
//! ## FUNDAMENTAL DESIGN
//!
//! The context is an envelope, not an execution engine.
//!
//! It transports information supplied by its caller. It does not decide
//! what that information means.
//!
//! For example:
//!
//! - An identity extension does not prove that an identity is authentic.
//! - A policy extension does not authorize an operation.
//! - A tracing extension does not create a trace.
//! - A scope reference does not select a physical machine.
//! - A cancellation signal does not forcibly terminate an operation.
//!
//! ## POCO-REAF AND SCALABILITY
//!
//! This module introduces no fixed maximum for:
//!
//! - processors;
//! - cores;
//! - threads;
//! - accelerators;
//! - quantum processors;
//! - qubits;
//! - FPGA or ASIC resources;
//! - memory;
//! - storage;
//! - tensor dimensions or rank;
//! - distributed participants;
//! - computational domains;
//! - extension types;
//! - metadata string lengths;
//! - context instances.
//!
//! Context allocations and extension storage grow subject to actual
//! available memory and platform representational limits.
//!
//! This is not a claim that physical memory or computation is infinite.
//!
//! Operational budgets belong to explicit compiler, provider or runtime
//! configuration. They must not be converted into language-level limits.
//!
//! ## IMMUTABILITY AND CONCURRENCY
//!
//! An OperationContext is immutable after construction.
//!
//! Cloning a context shares its internal allocation through Arc.
//!
//! Extension values are shared through Arc and must implement Send + Sync.
//!
//! Mutable operation state belongs to the operation, session or runtime
//! that owns it, not to this context.
//!
//! ## CANCELLATION
//!
//! Cancellation is cooperative.
//!
//! An operation must explicitly check its cancellation signal at suitable
//! boundaries and propagate cancellation to nested work.
//!
//! A cancellation request does not prove that a device, process, remote
//! worker, network request or physical operation has stopped.
//!
//! This module deliberately does not implement a second cancellation-token
//! state machine.
//!
//! Existing runtime and distributed cancellation implementations can
//! implement the CancellationSignal interface through adapters.
//!
//! ## EXTENSIONS
//!
//! Extensions allow external authorities to attach typed values without
//! this module depending on their concrete types.
//!
//! Examples include:
//!
//! - authenticated principal references;
//! - policy evaluation context;
//! - distributed trace context;
//! - request correlation data;
//! - domain-specific operation metadata;
//! - provider-neutral service references.
//!
//! Extension values are opaque to this module.
//!
//! Extension insertion rejects an existing value of the same concrete type.
//! Replacement requires an explicit replacement operation.
//!
//! Extensions are process-local values. They are not automatically
//! serialized, transmitted, persisted or propagated across machines.
//!
//! ## SECURITY
//!
//! Do not place passwords, private keys, access tokens, authentication
//! secrets or unrestricted sensitive payloads in descriptive metadata.
//!
//! Context metadata is not an authorization mechanism.
//!
//! A caller must not treat the presence of an extension as proof that
//! its contents are trustworthy.
//!
//! ## ERROR CONTRACT
//!
//! Ordinary invalid metadata and duplicate extension insertion return
//! the existing `crate::stdlib::error::Error`.
//!
//! Cancellation checks return the same standard-library error type.
//!
//! No ordinary invalid input causes a panic.
//!
//! ## FREEZE CRITERIA
//!
//! This module is ready to freeze when:
//!
//! 1. `src/stdlib/mod.rs` declares this module.
//! 2. The existing standard-library error module is available.
//! 3. Runtime cancellation adapters implement CancellationSignal.
//! 4. Identity, policy and tracing integrations use typed extensions.
//! 5. Existing semantic, parser, compiler and AI contexts remain distinct.
//! 6. Context sharing and child-context behavior are tested.
//! 7. Cancellation propagation is tested with an external signal.
//! 8. Extension insertion and replacement behavior is tested.
//! 9. No hardware-specific assumptions or arbitrary ceilings are introduced.
//! 10. Formatting, compilation, tests and Clippy pass.

use core::any::{Any, TypeId};
use core::fmt;

use std::collections::HashMap;
use std::sync::Arc;

use crate::stdlib::error::{Error, ErrorKind, Result};

// ============================================================================
// CANCELLATION CONTRACT
// ============================================================================

/// Read-only cooperative cancellation interface.
///
/// This is an observation contract, not a cancellation implementation.
///
/// Runtime, distributed and other execution authorities may implement
/// this trait for their existing cancellation mechanisms.
///
/// Implementations must be thread-safe because an operation context
/// may be shared across threads.
///
/// Cancellation reasons are descriptive information only. They must
/// not be parsed to make machine-readable control-flow decisions.
pub trait CancellationSignal: Send + Sync + 'static {
    /// Returns whether cancellation has been requested.
    fn is_cancelled(&self) -> bool;

    /// Returns an optional descriptive cancellation reason.
    ///
    /// The default implementation provides no reason.
    ///
    /// Callers should not place secrets or credentials in this value.
    fn cancellation_reason(&self) -> Option<String> {
        None
    }
}

/// Cancellation signal that never requests cancellation.
///
/// This is the default signal for contexts without an external
/// cancellation source.
#[derive(Debug, Clone, Copy, Default)]
pub struct NeverCancelled;

impl CancellationSignal for NeverCancelled {
    fn is_cancelled(&self) -> bool {
        false
    }
}

// ============================================================================
// INTERNAL CONTEXT STORAGE
// ============================================================================

/// Internal immutable context data.
struct ContextInner {
    /// Descriptive component identity supplied by the caller.
    component: Option<Arc<str>>,

    /// Descriptive operation identity supplied by the caller.
    operation: Option<Arc<str>>,

    /// Opaque logical scope reference supplied by the caller.
    ///
    /// This is not a physical location or hardware identifier.
    scope: Option<Arc<str>>,

    /// Read-only cancellation signal.
    cancellation: Arc<dyn CancellationSignal>,

    /// Typed extension values.
    ///
    /// TypeId is used only for local type lookup. It is not a persistent
    /// identifier and must never be serialized as an extension identity.
    extensions: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

// ============================================================================
// OPERATION CONTEXT
// ============================================================================

/// Immutable, shareable context for a standard-library operation.
///
/// An OperationContext carries descriptive metadata, cooperative
/// cancellation observation and typed extension services.
///
/// It does not own the operation's mutable state.
///
/// Context cloning is inexpensive because clones share the same
/// immutable internal allocation.
///
/// # Examples
///
/// ```
/// use zamani::stdlib::context::OperationContext;
///
/// let context = OperationContext::builder()
///     .with_component("stdlib.numeric")
///     .expect("valid component")
///     .with_operation("matrix.multiply")
///     .expect("valid operation")
///     .with_scope("application.request")
///     .expect("valid scope")
///     .build();
///
/// assert_eq!(context.component(), Some("stdlib.numeric"));
/// assert_eq!(context.operation(), Some("matrix.multiply"));
/// assert_eq!(context.scope(), Some("application.request"));
/// assert!(!context.is_cancelled());
/// ```
#[derive(Clone)]
pub struct OperationContext {
    inner: Arc<ContextInner>,
}

impl OperationContext {
    /// Creates a builder for a new operation context.
    #[must_use]
    pub fn builder() -> OperationContextBuilder {
        OperationContextBuilder::new()
    }

    /// Returns the optional descriptive component identity.
    ///
    /// This value is supplied by the caller. It does not establish
    /// that the named component is registered or available.
    #[must_use]
    pub fn component(&self) -> Option<&str> {
        self.inner.component.as_deref()
    }

    /// Returns the optional descriptive operation identity.
    ///
    /// This value is not necessarily globally unique.
    #[must_use]
    pub fn operation(&self) -> Option<&str> {
        self.inner.operation.as_deref()
    }

    /// Returns the optional opaque logical scope reference.
    ///
    /// A scope reference does not identify a physical machine,
    /// processor, device, network endpoint or execution provider.
    #[must_use]
    pub fn scope(&self) -> Option<&str> {
        self.inner.scope.as_deref()
    }

    /// Returns whether the operation has received a cancellation request.
    ///
    /// This method does not forcibly interrupt the operation.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.inner.cancellation.is_cancelled()
    }

    /// Returns an optional descriptive cancellation reason.
    #[must_use]
    pub fn cancellation_reason(&self) -> Option<String> {
        self.inner.cancellation.cancellation_reason()
    }

    /// Returns a shared read-only cancellation signal.
    ///
    /// The returned handle can be passed to nested operations.
    /// It does not provide a method to cancel the caller's operation.
    #[must_use]
    pub fn cancellation_signal(&self) -> Arc<dyn CancellationSignal> {
        Arc::clone(&self.inner.cancellation)
    }

    /// Checks whether cancellation has been requested.
    ///
    /// This method checks cancellation only. It does not check deadlines,
    /// resource budgets, permissions, capabilities or feasibility.
    ///
    /// # Errors
    ///
    /// Returns a standard-library Cancelled error when cancellation
    /// has been requested.
    pub fn check_cancelled(&self) -> Result<()> {
        if !self.is_cancelled() {
            return Ok(());
        }

        let message = match self.operation() {
            Some(operation) => {
                format!("operation '{operation}' was cancelled")
            }
            None => "operation was cancelled".to_owned(),
        };

        Err(
            Error::new(ErrorKind::Cancelled, message)
                .in_component(
                    self.component()
                        .unwrap_or("stdlib.context"),
                )
                .during_operation(
                    self.operation()
                        .unwrap_or("operation"),
                ),
        )
    }

    /// Retrieves an extension by its concrete Rust type.
    ///
    /// The extension remains owned by the context.
    ///
    /// The returned reference is valid for the lifetime of the context
    /// borrow.
    ///
    /// Extension types must be `'static + Send + Sync`.
    #[must_use]
    pub fn extension<T>(&self) -> Option<&T>
    where
        T: Any + Send + Sync,
    {
        self.inner
            .extensions
            .get(&TypeId::of::<T>())
            .and_then(|value| value.downcast_ref::<T>())
    }

    /// Retrieves a shared extension handle by its concrete Rust type.
    ///
    /// Cloning the returned Arc does not clone the underlying extension
    /// value.
    #[must_use]
    pub fn extension_arc<T>(&self) -> Option<Arc<T>>
    where
        T: Any + Send + Sync,
    {
        self.inner
            .extensions
            .get(&TypeId::of::<T>())
            .cloned()
            .and_then(|value| Arc::downcast::<T>(value).ok())
    }

    /// Returns whether an extension of the requested concrete type exists.
    #[must_use]
    pub fn has_extension<T>(&self) -> bool
    where
        T: Any + Send + Sync,
    {
        self.inner.extensions.contains_key(&TypeId::of::<T>())
    }

    /// Returns the number of attached typed extensions.
    ///
    /// This is descriptive metadata, not a capacity limit.
    #[must_use]
    pub fn extension_count(&self) -> usize {
        self.inner.extensions.len()
    }

    /// Creates a builder for a child operation context.
    ///
    /// The child inherits:
    ///
    /// - component identity;
    /// - logical scope;
    /// - cancellation signal;
    /// - all existing typed extensions.
    ///
    /// The child does not automatically inherit the parent's operation
    /// name. The caller should provide the child operation name explicitly.
    ///
    /// Inherited extension values are shared through Arc.
    #[must_use]
    pub fn child_builder(&self) -> OperationContextBuilder {
        OperationContextBuilder {
            component: self.inner.component.clone(),
            operation: None,
            scope: self.inner.scope.clone(),
            cancellation: Arc::clone(&self.inner.cancellation),
            extensions: self.inner.extensions.clone(),
        }
    }
}

impl Default for OperationContext {
    fn default() -> Self {
        OperationContextBuilder::new().build()
    }
}

impl fmt::Debug for OperationContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OperationContext")
            .field("component", &self.component())
            .field("operation", &self.operation())
            .field("scope", &self.scope())
            .field("cancelled", &self.is_cancelled())
            .field("extension_count", &self.extension_count())
            .finish()
    }
}

// ============================================================================
// OPERATION CONTEXT BUILDER
// ============================================================================

/// Builder for an immutable OperationContext.
///
/// A builder is mutable and should normally be owned by one context
/// construction operation.
///
/// Once built, the resulting context is immutable and shareable.
pub struct OperationContextBuilder {
    component: Option<Arc<str>>,
    operation: Option<Arc<str>>,
    scope: Option<Arc<str>>,
    cancellation: Arc<dyn CancellationSignal>,
    extensions: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl OperationContextBuilder {
    /// Creates an empty operation-context builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            component: None,
            operation: None,
            scope: None,
            cancellation: Arc::new(NeverCancelled),
            extensions: HashMap::new(),
        }
    }

    /// Sets the descriptive component identity.
    ///
    /// The value is opaque metadata and is not resolved against a
    /// component registry.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when the value is empty, whitespace-only
    /// or contains control characters.
    pub fn with_component(
        mut self,
        component: impl Into<String>,
    ) -> Result<Self> {
        self.component = Some(validated_label(
            component,
            "component",
        )?);

        Ok(self)
    }

    /// Sets the descriptive operation identity.
    ///
    /// The value is opaque metadata and is not resolved against an
    /// operation registry.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when the value is empty, whitespace-only
    /// or contains control characters.
    pub fn with_operation(
        mut self,
        operation: impl Into<String>,
    ) -> Result<Self> {
        self.operation = Some(validated_label(
            operation,
            "operation",
        )?);

        Ok(self)
    }

    /// Sets an opaque logical scope reference.
    ///
    /// The reference is not interpreted as a physical location,
    /// hardware identifier, device address or provider identity.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when the value is empty, whitespace-only
    /// or contains control characters.
    pub fn with_scope(
        mut self,
        scope: impl Into<String>,
    ) -> Result<Self> {
        self.scope = Some(validated_label(scope, "scope")?);

        Ok(self)
    }

    /// Sets the read-only cancellation signal.
    ///
    /// The supplied signal remains owned through Arc.
    ///
    /// Runtime-specific cancellation tokens can be adapted to
    /// CancellationSignal without this module depending on the runtime.
    #[must_use]
    pub fn with_cancellation(
        mut self,
        cancellation: Arc<dyn CancellationSignal>,
    ) -> Self {
        self.cancellation = cancellation;
        self
    }

    /// Attaches a typed extension.
    ///
    /// Extensions are indexed by their concrete Rust type.
    ///
    /// Inserting a second extension of the same concrete type is rejected.
    /// Use [`Self::replace_extension`] when replacement is intentional.
    ///
    /// Extension values must be thread-safe and have a `'static` type.
    ///
    /// # Errors
    ///
    /// Returns AlreadyExists if an extension of the same concrete type
    /// is already present.
    pub fn with_extension<T>(
        mut self,
        extension: T,
    ) -> Result<Self>
    where
        T: Any + Send + Sync,
    {
        let type_id = TypeId::of::<T>();

        if self.extensions.contains_key(&type_id) {
            return Err(
                Error::new(
                    ErrorKind::AlreadyExists,
                    "an extension of this concrete type already exists",
                )
                .in_component("stdlib.context")
                .during_operation("with_extension"),
            );
        }

        self.extensions.insert(
            type_id,
            Arc::new(extension),
        );

        Ok(self)
    }

    /// Attaches a shared typed extension.
    ///
    /// The supplied Arc is stored without cloning the underlying value.
    ///
    /// # Errors
    ///
    /// Returns AlreadyExists if an extension of the same concrete type
    /// is already present.
    pub fn with_shared_extension<T>(
        mut self,
        extension: Arc<T>,
    ) -> Result<Self>
    where
        T: Any + Send + Sync,
    {
        let type_id = TypeId::of::<T>();

        if self.extensions.contains_key(&type_id) {
            return Err(
                Error::new(
                    ErrorKind::AlreadyExists,
                    "an extension of this concrete type already exists",
                )
                .in_component("stdlib.context")
                .during_operation("with_shared_extension"),
            );
        }

        self.extensions.insert(type_id, extension);

        Ok(self)
    }

    /// Explicitly replaces an extension of the same concrete type.
    ///
    /// This method exists to make replacement intentional rather than
    /// silently overwriting an existing extension during insertion.
    ///
    /// Returns the builder for continued construction.
    #[must_use]
    pub fn replace_extension<T>(
        mut self,
        extension: T,
    ) -> Self
    where
        T: Any + Send + Sync,
    {
        self.extensions.insert(
            TypeId::of::<T>(),
            Arc::new(extension),
        );

        self
    }

    /// Builds the immutable operation context.
    ///
    /// Construction performs no I/O, hardware discovery, provider lookup,
    /// authentication, policy evaluation, tracing initialization or
    /// runtime initialization.
    #[must_use]
    pub fn build(self) -> OperationContext {
        OperationContext {
            inner: Arc::new(ContextInner {
                component: self.component,
                operation: self.operation,
                scope: self.scope,
                cancellation: self.cancellation,
                extensions: self.extensions,
            }),
        }
    }
}

impl Default for OperationContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for OperationContextBuilder {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OperationContextBuilder")
            .field("component", &self.component)
            .field("operation", &self.operation)
            .field("scope", &self.scope)
            .field("cancelled", &self.cancellation.is_cancelled())
            .field("extension_count", &self.extensions.len())
            .finish()
    }
}

// ============================================================================
// VALIDATION
// ============================================================================

/// Validates descriptive context labels.
///
/// Labels remain open-world strings. This function does not impose
/// a character whitelist, namespace grammar, maximum length or
/// hardware-specific interpretation.
///
/// Control characters are rejected because these values may appear
/// in logs, diagnostics and structured metadata.
fn validated_label(
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
            .in_component("stdlib.context")
            .during_operation("validate_context_metadata"),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                format!("{field} must not contain control characters"),
            )
            .in_component("stdlib.context")
            .during_operation("validate_context_metadata"),
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

    use std::sync::atomic::{AtomicBool, Ordering};

    #[derive(Debug)]
    struct TestCancellation {
        cancelled: AtomicBool,
    }

    impl TestCancellation {
        fn new() -> Self {
            Self {
                cancelled: AtomicBool::new(false),
            }
        }

        fn cancel(&self) {
            self.cancelled.store(true, Ordering::Release);
        }
    }

    impl CancellationSignal for TestCancellation {
        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::Acquire)
        }

        fn cancellation_reason(&self) -> Option<String> {
            self.is_cancelled()
                .then(|| "test cancellation".to_owned())
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct TestIdentity {
        subject: String,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct TestPolicyContext {
        policy_reference: String,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct TestTraceContext {
        correlation_reference: String,
    }

    #[test]
    fn default_context_is_valid_and_not_cancelled() {
        let context = OperationContext::default();

        assert_eq!(context.component(), None);
        assert_eq!(context.operation(), None);
        assert_eq!(context.scope(), None);
        assert!(!context.is_cancelled());
        assert!(context.check_cancelled().is_ok());
    }

    #[test]
    fn builder_preserves_caller_metadata() {
        let context = OperationContext::builder()
            .with_component("stdlib.numeric")
            .expect("valid component")
            .with_operation("matrix.multiply")
            .expect("valid operation")
            .with_scope("application.request")
            .expect("valid scope")
            .build();

        assert_eq!(context.component(), Some("stdlib.numeric"));
        assert_eq!(context.operation(), Some("matrix.multiply"));
        assert_eq!(context.scope(), Some("application.request"));
    }

    #[test]
    fn metadata_rejects_empty_values() {
        assert!(
            OperationContext::builder()
                .with_component("")
                .is_err()
        );

        assert!(
            OperationContext::builder()
                .with_operation("   ")
                .is_err()
        );

        assert!(
            OperationContext::builder()
                .with_scope("\t")
                .is_err()
        );
    }

    #[test]
    fn metadata_rejects_control_characters() {
        assert!(
            OperationContext::builder()
                .with_component("stdlib\nnumeric")
                .is_err()
        );

        assert!(
            OperationContext::builder()
                .with_operation("operation\u{0000}name")
                .is_err()
        );
    }

    #[test]
    fn metadata_does_not_impose_an_arbitrary_length_limit() {
        let long_value = "x".repeat(100_000);

        let context = OperationContext::builder()
            .with_operation(long_value.clone())
            .expect("value should not be rejected by an arbitrary length limit")
            .build();

        assert_eq!(
            context.operation(),
            Some(long_value.as_str())
        );
    }

    #[test]
    fn context_clones_share_immutable_data() {
        let context = OperationContext::builder()
            .with_operation("test.operation")
            .expect("valid operation")
            .build();

        let cloned = context.clone();

        assert_eq!(context.operation(), cloned.operation());
        assert!(Arc::ptr_eq(&context.inner, &cloned.inner));
    }

    #[test]
    fn extensions_are_retrieved_by_concrete_type() {
        let context = OperationContext::builder()
            .with_extension(TestIdentity {
                subject: "principal-reference".to_owned(),
            })
            .expect("unique extension")
            .build();

        let identity = context
            .extension::<TestIdentity>()
            .expect("identity extension");

        assert_eq!(
            identity.subject,
            "principal-reference"
        );

        assert!(context.has_extension::<TestIdentity>());
        assert!(!context.has_extension::<TestPolicyContext>());
    }

    #[test]
    fn different_extension_types_can_coexist() {
        let context = OperationContext::builder()
            .with_extension(TestIdentity {
                subject: "principal".to_owned(),
            })
            .expect("identity extension")
            .with_extension(TestPolicyContext {
                policy_reference: "policy-reference".to_owned(),
            })
            .expect("policy extension")
            .with_extension(TestTraceContext {
                correlation_reference: "trace-reference".to_owned(),
            })
            .expect("trace extension")
            .build();

        assert_eq!(context.extension_count(), 3);

        assert!(
            context.extension::<TestIdentity>().is_some()
        );

        assert!(
            context.extension::<TestPolicyContext>().is_some()
        );

        assert!(
            context.extension::<TestTraceContext>().is_some()
        );
    }

    #[test]
    fn duplicate_extension_insertion_is_rejected() {
        let result = OperationContext::builder()
            .with_extension(TestIdentity {
                subject: "first".to_owned(),
            })
            .expect("first extension")
            .with_extension(TestIdentity {
                subject: "second".to_owned(),
            });

        assert!(result.is_err());
    }

    #[test]
    fn explicit_extension_replacement_works() {
        let context = OperationContext::builder()
            .with_extension(TestIdentity {
                subject: "first".to_owned(),
            })
            .expect("first extension")
            .replace_extension(TestIdentity {
                subject: "second".to_owned(),
            })
            .build();

        assert_eq!(
            context
                .extension::<TestIdentity>()
                .expect("replacement")
                .subject,
            "second"
        );
    }

    #[test]
    fn shared_extension_does_not_clone_underlying_value() {
        let identity = Arc::new(TestIdentity {
            subject: "shared-principal".to_owned(),
        });

        let context = OperationContext::builder()
            .with_shared_extension(Arc::clone(&identity))
            .expect("shared extension")
            .build();

        let retrieved = context
            .extension_arc::<TestIdentity>()
            .expect("shared extension handle");

        assert!(Arc::ptr_eq(&identity, &retrieved));
    }

    #[test]
    fn child_context_inherits_scope_cancellation_and_extensions() {
        let cancellation = Arc::new(TestCancellation::new());

        let parent = OperationContext::builder()
            .with_component("stdlib")
            .expect("component")
            .with_operation("parent.operation")
            .expect("operation")
            .with_scope("logical.scope")
            .expect("scope")
            .with_cancellation(cancellation.clone())
            .with_extension(TestIdentity {
                subject: "parent-principal".to_owned(),
            })
            .expect("identity")
            .build();

        let child = parent
            .child_builder()
            .with_operation("child.operation")
            .expect("child operation")
            .build();

        assert_eq!(child.component(), Some("stdlib"));
        assert_eq!(child.operation(), Some("child.operation"));
        assert_eq!(child.scope(), Some("logical.scope"));

        assert_eq!(
            child
                .extension::<TestIdentity>()
                .expect("inherited identity")
                .subject,
            "parent-principal"
        );

        cancellation.cancel();

        assert!(parent.is_cancelled());
        assert!(child.is_cancelled());
        assert!(child.check_cancelled().is_err());
    }

    #[test]
    fn cancellation_check_returns_standard_library_error() {
        let cancellation = Arc::new(TestCancellation::new());

        let context = OperationContext::builder()
            .with_operation("long.operation")
            .expect("operation")
            .with_cancellation(cancellation.clone())
            .build();

        assert!(context.check_cancelled().is_ok());

        cancellation.cancel();

        let error = context
            .check_cancelled()
            .expect_err("cancellation must be reported");

        assert_eq!(error.kind(), ErrorKind::Cancelled);
        assert_eq!(error.component(), Some("stdlib.context"));
        assert_eq!(error.operation(), Some("long.operation"));
    }

    #[test]
    fn cancellation_reason_is_available_without_becoming_control_flow() {
        let cancellation = Arc::new(TestCancellation::new());

        let context = OperationContext::builder()
            .with_cancellation(cancellation.clone())
            .build();

        assert_eq!(context.cancellation_reason(), None);

        cancellation.cancel();

        assert_eq!(
            context.cancellation_reason().as_deref(),
            Some("test cancellation")
        );
    }

    #[test]
    fn never_cancelled_signal_is_stable() {
        let signal = NeverCancelled;

        assert!(!signal.is_cancelled());
        assert_eq!(signal.cancellation_reason(), None);
    }

    #[test]
    fn child_context_can_replace_an_inherited_extension_explicitly() {
        let parent = OperationContext::builder()
            .with_extension(TestIdentity {
                subject: "parent".to_owned(),
            })
            .expect("parent identity")
            .build();

        let child = parent
            .child_builder()
            .replace_extension(TestIdentity {
                subject: "child".to_owned(),
            })
            .build();

        assert_eq!(
            parent
                .extension::<TestIdentity>()
                .expect("parent identity")
                .subject,
            "parent"
        );

        assert_eq!(
            child
                .extension::<TestIdentity>()
                .expect("child identity")
                .subject,
            "child"
        );
    }
}
