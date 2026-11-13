
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! Zamani Universal Runtime — Security Isolation Boundary.
//!
//! Path: src/zur/security/isolation.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Defines the provider-neutral runtime interface for establishing,
//! validating, maintaining, and closing execution-isolation sessions.
//!
//! This module is the runtime enforcement boundary between validated
//! security/isolation intent and platform-specific isolation providers.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Runtime isolation identifiers.
//! - Runtime isolation requests.
//! - Required, preferred, and prohibited isolation directives.
//! - Provider-reported isolation evidence.
//! - Isolation provider and session contracts.
//! - Mandatory requirement verification.
//! - Isolation session leases and cleanup.
//! - Fail-closed isolation establishment.
//!
//! This module does not own:
//!
//! - Source-language parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Security requirement interpretation.
//! - Authentication.
//! - Authorization.
//! - Policy evaluation.
//! - Trust establishment.
//! - Credential management.
//! - Cryptographic implementations.
//! - Hardware discovery or selection.
//! - Resource allocation.
//! - Process, container, VM, enclave, or OS sandbox implementations.
//! - Scheduling or workload placement.
//! - Native code generation.
//! - Domain-specific IR.
//!
//! # Canonical authorities
//!
//! Source-level isolation intent is owned by:
//!
//! - grammar/security/sandbox.g4
//! - grammar/security/security-constraints.g4
//! - grammar/security/security.g4
//!
//! Permission declarations are owned by:
//!
//! - grammar/security/permissions.g4
//!
//! Authentication and authorization remain owned by:
//!
//! - distributed/security/authentication.rs
//! - distributed/security/authorization.rs
//! - distributed/security/policy.rs
//!
//! The frontend semantic layer must resolve and validate source-level
//! isolation intent before constructing an IsolationRequest.
//!
//! Platform-specific providers perform actual isolation.
//!
//! # Security invariants
//!
//! 1. A declared isolation requirement is not proof of enforcement.
//! 2. A registered provider is not proof that isolation is active.
//! 3. A provider must establish an actual isolation session.
//! 4. Mandatory controls must have affirmative evidence.
//! 5. Prohibited controls must have affirmative evidence of absence.
//! 6. Unknown evidence never satisfies a mandatory requirement.
//! 7. Unmet preferences are reported, not silently upgraded to guarantees.
//! 8. Provider identity must match the identity in its evidence.
//! 9. An inactive isolation session cannot be returned as established.
//! 10. Failed establishment or validation must not authorize execution.
//! 11. Closing an isolation session must be explicit and fallible.
//! 12. Dropping a lease attempts best-effort cleanup.
//! 13. Authorization remains a separate mandatory security boundary.
//! 14. No provider may manufacture successful evidence for an operation
//!     it did not actually enforce.
//!
//! # Open-world and scalability contract
//!
//! Isolation control identifiers are open-ended namespaced identifiers.
//!
//! This module does not enumerate operating systems, sandbox technologies,
//! security mechanisms, hardware, devices, domains, providers, or vendors.
//!
//! It imposes no arbitrary maximum on:
//!
//! - Isolation controls.
//! - Isolation directives.
//! - Isolation providers.
//! - Isolation sessions.
//! - Execution subjects.
//! - Security mechanisms.
//! - Computational domains.
//! - Devices or machines.
//!
//! Actual capacity is determined by available memory, provider capabilities,
//! platform limits, and explicitly configured operational budgets.
//!
//! Literal infinite physical capacity is not promised.
//!
//! # Concurrency
//!
//! Providers and sessions are explicitly owned and injected.
//!
//! This module creates no global mutable state, background threads,
//! hidden task queues, or implicit singleton.
//!
//! Providers and sessions must implement their own documented
//! concurrency and synchronization guarantees.
//!
//! # Provider trust
//!
//! A provider is an enforcement authority supplied by the runtime
//! composition root.
//!
//! Its evidence is only as trustworthy as the provider's implementation,
//! admission process, platform integration, and operational environment.
//!
//! This module does not cryptographically attest providers or hardware.
//! Attestation and trust verification belong to their respective
//! security and platform authorities.

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

// ============================================================================
// Results and errors
// ============================================================================

/// Result type for runtime isolation operations.
pub type IsolationResult<T> = Result<T, IsolationError>;

/// Classification of runtime isolation failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IsolationErrorKind {
    /// An isolation identifier or request is invalid.
    InvalidRequest,

    /// A provider failed to establish or inspect isolation.
    ProviderFailure,

    /// The evidence identifies a different provider.
    ProviderIdentityMismatch,

    /// An isolation session is not active.
    SessionInactive,

    /// A mandatory isolation control is not satisfied.
    RequiredControlUnsatisfied,

    /// A prohibited control is active.
    ProhibitedControlEnabled,

    /// The provider cannot establish the state of a mandatory control.
    ControlStateIndeterminate,

    /// Isolation cleanup failed.
    CleanupFailed,
}

/// Structured error for runtime isolation operations.
///
/// Error messages deliberately avoid including credentials, secrets,
/// provider-private data, or arbitrary platform error details.
///
/// Providers should map their internal errors to an appropriate
/// IsolationErrorKind at their adapter boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsolationError {
    kind: IsolationErrorKind,
    control: Option<IsolationIdentifier>,
}

impl IsolationError {
    /// Construct an isolation error without control-specific context.
    #[must_use]
    pub const fn new(kind: IsolationErrorKind) -> Self {
        Self {
            kind,
            control: None,
        }
    }

    /// Construct an isolation error associated with a particular control.
    #[must_use]
    pub fn for_control(
        kind: IsolationErrorKind,
        control: IsolationIdentifier,
    ) -> Self {
        Self {
            kind,
            control: Some(control),
        }
    }

    /// Return the error classification.
    #[must_use]
    pub const fn kind(&self) -> IsolationErrorKind {
        self.kind
    }

    /// Return the associated isolation control, if present.
    #[must_use]
    pub fn control(&self) -> Option<&IsolationIdentifier> {
        self.control.as_ref()
    }
}

impl fmt::Display for IsolationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let description = match self.kind {
            IsolationErrorKind::InvalidRequest =>
                "invalid isolation request",

            IsolationErrorKind::ProviderFailure =>
                "isolation provider operation failed",

            IsolationErrorKind::ProviderIdentityMismatch =>
                "isolation provider identity does not match its evidence",

            IsolationErrorKind::SessionInactive =>
                "isolation session is not active",

            IsolationErrorKind::RequiredControlUnsatisfied =>
                "mandatory isolation control is not satisfied",

            IsolationErrorKind::ProhibitedControlEnabled =>
                "a prohibited isolation control is enabled",

            IsolationErrorKind::ControlStateIndeterminate =>
                "isolation control state is indeterminate",

            IsolationErrorKind::CleanupFailed =>
                "isolation session cleanup failed",
        };

        match &self.control {
            Some(control) => {
                write!(formatter, "{description}: {control}")
            }
            None => formatter.write_str(description),
        }
    }
}

impl std::error::Error for IsolationError {}

// ============================================================================
// Open-world identifiers
// ============================================================================

/// Open-ended, namespaced identifier used by the isolation boundary.
///
/// This identifier can represent an isolation subject, provider, session,
/// or control. Its meaning is established by the relevant contract and
/// upstream semantic model.
///
/// No fixed vocabulary or arbitrary maximum length is imposed.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IsolationIdentifier(String);

impl IsolationIdentifier {
    /// Construct a validated isolation identifier.
    ///
    /// Identifiers must:
    ///
    /// - Not be empty.
    /// - Not contain surrounding whitespace.
    /// - Not contain whitespace.
    /// - Not contain control characters.
    ///
    /// Namespaced identifiers such as `security.network.isolation`
    /// and `future.security.execution_boundary` are supported.
    pub fn new(
        value: impl Into<String>,
    ) -> IsolationResult<Self> {
        let value = value.into();

        if value.is_empty()
            || value.trim() != value
            || value.chars().any(|character| {
                character.is_whitespace()
                    || character.is_control()
            })
        {
            return Err(IsolationError::new(
                IsolationErrorKind::InvalidRequest,
            ));
        }

        Ok(Self(value))
    }

    /// Return the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the identifier and return its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for IsolationIdentifier {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Isolation directives
// ============================================================================

/// Semantic strength or polarity of an isolation directive.
///
/// The frontend semantic adapter is responsible for translating source
/// declarations into these normalized runtime directives.
///
/// This enum does not represent a closed list of isolation mechanisms.
/// The control identifiers remain open-world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IsolationDisposition {
    /// The named control must be enabled.
    Required,

    /// The named control is desirable but not mandatory.
    Preferred,

    /// The named control must not be enabled.
    Prohibited,
}

/// Runtime isolation directive for one open-world control.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsolationDirective {
    control: IsolationIdentifier,
    disposition: IsolationDisposition,
}

impl IsolationDirective {
    /// Construct an isolation directive.
    #[must_use]
    pub const fn new(
        control: IsolationIdentifier,
        disposition: IsolationDisposition,
    ) -> Self {
        Self {
            control,
            disposition,
        }
    }

    /// Return the control identifier.
    #[must_use]
    pub fn control(&self) -> &IsolationIdentifier {
        &self.control
    }

    /// Return the directive disposition.
    #[must_use]
    pub const fn disposition(&self) -> IsolationDisposition {
        self.disposition
    }
}

// ============================================================================
// Isolation requests
// ============================================================================

/// Runtime projection of an already validated isolation subject and
/// its isolation directives.
///
/// This is not an AST, semantic model, policy language, or source parser.
///
/// The frontend semantic adapter must resolve source-level names,
/// expressions, nested sandbox composition, contradictions, provenance,
/// and applicable constraints before constructing this request.
///
/// Runtime code must not reinterpret the original source syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsolationRequest {
    subject: IsolationIdentifier,
    directives: Vec<IsolationDirective>,
}

impl IsolationRequest {
    /// Construct an isolation request from validated runtime identifiers
    /// and normalized directives.
    ///
    /// Conflicting dispositions for the same control are rejected.
    /// Repeated identical directives are preserved.
    ///
    /// An empty directive collection is permitted: the provider may still
    /// establish a baseline isolation boundary for the subject.
    pub fn new(
        subject: IsolationIdentifier,
        directives: Vec<IsolationDirective>,
    ) -> IsolationResult<Self> {
        let mut dispositions =
            BTreeMap::<IsolationIdentifier, IsolationDisposition>::new();

        for directive in &directives {
            if let Some(previous) = dispositions.get(&directive.control) {
                if *previous != directive.disposition {
                    return Err(IsolationError::for_control(
                        IsolationErrorKind::InvalidRequest,
                        directive.control.clone(),
                    ));
                }
            } else {
                dispositions.insert(
                    directive.control.clone(),
                    directive.disposition,
                );
            }
        }

        Ok(Self {
            subject,
            directives,
        })
    }

    /// Return the subject to which isolation applies.
    #[must_use]
    pub fn subject(&self) -> &IsolationIdentifier {
        &self.subject
    }

    /// Return the normalized isolation directives.
    #[must_use]
    pub fn directives(&self) -> &[IsolationDirective] {
        &self.directives
    }
}

// ============================================================================
// Provider evidence
// ============================================================================

/// Provider-reported state of an isolation control.
///
/// `Unknown` is deliberately different from `Disabled`.
///
/// A provider must not report `Disabled` merely because it did not inspect
/// a control. It must have sufficient evidence to justify the reported state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IsolationControlState {
    /// The named control is actually enabled.
    Enabled,

    /// The named control is actually disabled.
    Disabled,

    /// The provider cannot establish the control's state.
    Unknown,
}

/// Evidence returned by an isolation provider.
///
/// Evidence is a provider assertion, not cryptographic attestation.
///
/// The runtime verifies that the evidence is internally consistent with
/// the request. Platform trust, attestation and provider admission remain
/// responsibilities of their authoritative security systems.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsolationEvidence {
    provider: IsolationIdentifier,
    controls: BTreeMap<IsolationIdentifier, IsolationControlState>,
}

impl IsolationEvidence {
    /// Construct provider evidence.
    ///
    /// Duplicate controls with conflicting states are rejected.
    /// Repeated identical states are accepted.
    pub fn new(
        provider: IsolationIdentifier,
        controls: impl IntoIterator<
            Item = (IsolationIdentifier, IsolationControlState),
        >,
    ) -> IsolationResult<Self> {
        let mut states =
            BTreeMap::<IsolationIdentifier, IsolationControlState>::new();

        for (control, state) in controls {
            if let Some(previous) = states.get(&control) {
                if *previous != state {
                    return Err(IsolationError::for_control(
                        IsolationErrorKind::InvalidRequest,
                        control,
                    ));
                }
            } else {
                states.insert(control, state);
            }
        }

        Ok(Self {
            provider,
            controls: states,
        })
    }

    /// Return the identity of the provider that produced this evidence.
    #[must_use]
    pub fn provider(&self) -> &IsolationIdentifier {
        &self.provider
    }

    /// Return the reported state of a control.
    ///
    /// Missing evidence is treated as `Unknown`.
    #[must_use]
    pub fn state(
        &self,
        control: &IsolationIdentifier,
    ) -> IsolationControlState {
        self.controls
            .get(control)
            .copied()
            .unwrap_or(IsolationControlState::Unknown)
    }

    /// Return all controls reported by the provider.
    #[must_use]
    pub fn controls(
        &self,
    ) -> &BTreeMap<IsolationIdentifier, IsolationControlState> {
        &self.controls
    }
}

// ============================================================================
// Isolation session and provider contracts
// ============================================================================

/// An established provider-owned isolation session.
///
/// Implementations represent actual platform/provider isolation state.
///
/// The provider must not return a session unless its isolation mechanism
/// has genuinely been established.
///
/// Implementations must be thread-safe. `close` must be idempotent and
/// must report cleanup failures rather than silently claiming success.
///
/// `provider_context` exposes a safe, provider-specific context object.
/// Platform adapters may downcast it to their own safe Rust context type.
/// It must not expose undocumented raw pointers or require unsafe Rust.
pub trait IsolationSession: Send + Sync {
    /// Return the provider-assigned session identifier.
    fn id(&self) -> &IsolationIdentifier;

    /// Return the evidence associated with this session.
    fn evidence(&self) -> &IsolationEvidence;

    /// Determine whether the isolation session is currently active.
    ///
    /// Provider inspection failures must be returned as errors.
    fn is_active(&self) -> IsolationResult<bool>;

    /// Close the isolation session.
    ///
    /// This operation must be idempotent.
    ///
    /// An implementation must not report success while protected resources
    /// or isolation state that it owns remain active.
    fn close(&self) -> IsolationResult<()>;

    /// Return the provider-specific safe context associated with the
    /// isolation session.
    ///
    /// This is intended for platform/execution adapters that need to bind
    /// their actual workload execution to the established isolation session.
    ///
    /// The returned value must remain valid for the lifetime of the session.
    fn provider_context(&self) -> &(dyn Any + Send + Sync);
}

/// Runtime isolation provider.
///
/// A provider is responsible for implementing the actual isolation
/// mechanism, such as an operating-system sandbox, process boundary,
/// container, virtual machine, enclave, or another supported mechanism.
///
/// This module does not implement any of those mechanisms itself.
///
/// Providers must:
///
/// - Establish real isolation before returning a session.
/// - Return evidence for requested controls.
/// - Report unknown states honestly.
/// - Preserve the association between the subject and the isolation session.
/// - Reject unsupported mandatory requirements.
/// - Support explicit lifecycle inspection and cleanup.
/// - Avoid fabricated success.
/// - Use safe Rust in their Zamani-facing implementation.
pub trait IsolationProvider: Send + Sync {
    /// Return the provider's stable namespaced identity.
    fn id(&self) -> &IsolationIdentifier;

    /// Establish isolation for the supplied validated request.
    ///
    /// Returning a session means the provider claims that the isolation
    /// boundary has actually been established.
    ///
    /// The runtime independently checks the provider identity, session
    /// activity and returned control evidence before accepting the session.
    fn establish(
        &self,
        request: &IsolationRequest,
    ) -> IsolationResult<Box<dyn IsolationSession>>;
}

// ============================================================================
// Isolation report and lease
// ============================================================================

/// Verified result of runtime isolation establishment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsolationReport {
    provider: IsolationIdentifier,
    session: IsolationIdentifier,
    satisfied_controls: Vec<IsolationIdentifier>,
    unmet_preferences: Vec<IsolationIdentifier>,
    evidence: IsolationEvidence,
}

impl IsolationReport {
    /// Return the provider identity.
    #[must_use]
    pub fn provider(&self) -> &IsolationIdentifier {
        &self.provider
    }

    /// Return the provider-assigned session identity.
    #[must_use]
    pub fn session(&self) -> &IsolationIdentifier {
        &self.session
    }

    /// Return controls whose requested states were satisfied.
    #[must_use]
    pub fn satisfied_controls(&self) -> &[IsolationIdentifier] {
        &self.satisfied_controls
    }

    /// Return preferences that the provider could not satisfy.
    ///
    /// These are not mandatory requirement failures.
    #[must_use]
    pub fn unmet_preferences(&self) -> &[IsolationIdentifier] {
        &self.unmet_preferences
    }

    /// Return the provider evidence used to verify this report.
    #[must_use]
    pub fn evidence(&self) -> &IsolationEvidence {
        &self.evidence
    }
}

/// An accepted isolation session held by the runtime.
///
/// The lease owns the provider session. It must remain alive for the
/// protected execution that depends on it.
///
/// Explicit `close` is preferred because it reports cleanup failures.
/// Dropping the lease performs best-effort cleanup as a safety net.
pub struct IsolationLease {
    session: Option<Box<dyn IsolationSession>>,
    report: IsolationReport,
}

impl IsolationLease {
    /// Return the verified isolation report.
    #[must_use]
    pub fn report(&self) -> &IsolationReport {
        &self.report
    }

    /// Return the active provider session, if the lease has not been closed.
    #[must_use]
    pub fn session(&self) -> Option<&dyn IsolationSession> {
        self.session.as_deref()
    }

    /// Return whether the provider session is currently active.
    ///
    /// A closed lease returns `Ok(false)`.
    pub fn is_active(&self) -> IsolationResult<bool> {
        match self.session() {
            Some(session) => session.is_active(),
            None => Ok(false),
        }
    }

    /// Close the isolation session and release the lease.
    ///
    /// If provider cleanup fails, the session remains owned by the lease
    /// so that a caller may inspect the error and retry cleanup.
    pub fn close(&mut self) -> IsolationResult<()> {
        let Some(session) = self.session.as_ref() else {
            return Ok(());
        };

        session.close()?;

        self.session.take();

        Ok(())
    }
}

impl fmt::Debug for IsolationLease {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("IsolationLease")
            .field("report", &self.report)
            .field("active", &self.session.is_some())
            .finish()
    }
}

impl Drop for IsolationLease {
    fn drop(&mut self) {
        if let Some(session) = self.session.as_ref() {
            // Drop cannot return an error. Explicit close() is therefore
            // the required path when callers need to observe cleanup failure.
            //
            // Providers must implement best-effort cleanup in their own
            // session lifecycle as well.
            let _ = session.close();
        }
    }
}

// ============================================================================
// Isolation manager
// ============================================================================

/// Coordinates isolation establishment through one explicitly injected
/// provider.
///
/// This manager is not a provider registry, hardware selector, policy
/// engine, authorization service, or scheduler.
///
/// Provider discovery and admission belong to the runtime composition
/// and provider-management authorities.
pub struct IsolationManager {
    provider: Arc<dyn IsolationProvider>,
}

impl IsolationManager {
    /// Construct a manager with an explicitly selected provider.
    ///
    /// Provider selection and trust validation must already have occurred
    /// through the appropriate runtime composition and security boundaries.
    #[must_use]
    pub fn new(provider: Arc<dyn IsolationProvider>) -> Self {
        Self { provider }
    }

    /// Return the injected provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &IsolationIdentifier {
        self.provider.id()
    }

    /// Establish and verify an isolation session.
    ///
    /// The method fails closed when:
    ///
    /// - Provider establishment fails.
    /// - Provider evidence identifies another provider.
    /// - The returned session is inactive.
    /// - A mandatory control is disabled or unknown.
    /// - A prohibited control is enabled or unknown.
    ///
    /// Unmet preferences are retained in the returned report.
    ///
    /// If validation fails after a session was created, the manager
    /// attempts to close it before returning the error.
    pub fn establish(
        &self,
        request: &IsolationRequest,
    ) -> IsolationResult<IsolationLease> {
        let session = self.provider.establish(request)?;

        if session.evidence().provider() != self.provider.id() {
            return Self::reject_session(
                session,
                IsolationError::new(
                    IsolationErrorKind::ProviderIdentityMismatch,
                ),
            );
        }

        match session.is_active() {
            Ok(true) => {}
            Ok(false) => {
                return Self::reject_session(
                    session,
                    IsolationError::new(
                        IsolationErrorKind::SessionInactive,
                    ),
                );
            }
            Err(error) => {
                return Self::reject_session(session, error);
            }
        }

        let report = match Self::verify_request(
            request,
            session.id(),
            self.provider.id(),
            session.evidence(),
        ) {
            Ok(report) => report,
            Err(error) => {
                return Self::reject_session(session, error);
            }
        };

        Ok(IsolationLease {
            session: Some(session),
            report,
        })
    }

    fn verify_request(
        request: &IsolationRequest,
        session_id: &IsolationIdentifier,
        provider_id: &IsolationIdentifier,
        evidence: &IsolationEvidence,
    ) -> IsolationResult<IsolationReport> {
        let mut satisfied_controls = Vec::new();
        let mut unmet_preferences = Vec::new();

        for directive in request.directives() {
            let state = evidence.state(directive.control());

            match directive.disposition() {
                IsolationDisposition::Required => {
                    match state {
                        IsolationControlState::Enabled => {
                            satisfied_controls.push(
                                directive.control().clone(),
                            );
                        }

                        IsolationControlState::Disabled => {
                            return Err(IsolationError::for_control(
                                IsolationErrorKind::RequiredControlUnsatisfied,
                                directive.control().clone(),
                            ));
                        }

                        IsolationControlState::Unknown => {
                            return Err(IsolationError::for_control(
                                IsolationErrorKind::ControlStateIndeterminate,
                                directive.control().clone(),
                            ));
                        }
                    }
                }

                IsolationDisposition::Prohibited => {
                    match state {
                        IsolationControlState::Disabled => {
                            satisfied_controls.push(
                                directive.control().clone(),
                            );
                        }

                        IsolationControlState::Enabled => {
                            return Err(IsolationError::for_control(
                                IsolationErrorKind::ProhibitedControlEnabled,
                                directive.control().clone(),
                            ));
                        }

                        IsolationControlState::Unknown => {
                            return Err(IsolationError::for_control(
                                IsolationErrorKind::ControlStateIndeterminate,
                                directive.control().clone(),
                            ));
                        }
                    }
                }

                IsolationDisposition::Preferred => {
                    if state == IsolationControlState::Enabled {
                        satisfied_controls.push(
                            directive.control().clone(),
                        );
                    } else {
                        unmet_preferences.push(
                            directive.control().clone(),
                        );
                    }
                }
            }
        }

        Ok(IsolationReport {
            provider: provider_id.clone(),
            session: session_id.clone(),
            satisfied_controls,
            unmet_preferences,
            evidence: evidence.clone(),
        })
    }

    fn reject_session(
        session: Box<dyn IsolationSession>,
        original_error: IsolationError,
    ) -> IsolationResult<IsolationLease> {
        match session.close() {
            Ok(()) => Err(original_error),

            Err(_) => Err(IsolationError::new(
                IsolationErrorKind::CleanupFailed,
            )),
        }
    }
}

impl fmt::Debug for IsolationManager {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("IsolationManager")
            .field("provider", &self.provider.id())
            .finish()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{
        AtomicBool,
        AtomicUsize,
        Ordering,
    };

    fn identifier(value: &str) -> IsolationIdentifier {
        IsolationIdentifier::new(value)
            .expect("test identifier must be valid")
    }

    fn directive(
        control: &str,
        disposition: IsolationDisposition,
    ) -> IsolationDirective {
        IsolationDirective::new(
            identifier(control),
            disposition,
        )
    }

    struct TestSession {
        id: IsolationIdentifier,
        evidence: IsolationEvidence,
        active: AtomicBool,
        close_calls: Arc<AtomicUsize>,
        context: String,
    }

    impl IsolationSession for TestSession {
        fn id(&self) -> &IsolationIdentifier {
            &self.id
        }

        fn evidence(&self) -> &IsolationEvidence {
            &self.evidence
        }

        fn is_active(&self) -> IsolationResult<bool> {
            Ok(self.active.load(Ordering::SeqCst))
        }

        fn close(&self) -> IsolationResult<()> {
            if self.active.swap(false, Ordering::SeqCst) {
                self.close_calls.fetch_add(1, Ordering::SeqCst);
            }

            Ok(())
        }

        fn provider_context(&self) -> &(dyn Any + Send + Sync) {
            &self.context
        }
    }

    struct TestProvider {
        id: IsolationIdentifier,
        evidence: IsolationEvidence,
        active: Arc<AtomicBool>,
        close_calls: Arc<AtomicUsize>,
    }

    impl TestProvider {
        fn new(
            provider_id: &str,
            evidence_provider_id: &str,
            controls: Vec<(IsolationIdentifier, IsolationControlState)>,
        ) -> Self {
            Self {
                id: identifier(provider_id),
                evidence: IsolationEvidence::new(
                    identifier(evidence_provider_id),
                    controls,
                )
                .expect("test evidence must be valid"),
                active: Arc::new(AtomicBool::new(true)),
                close_calls: Arc::new(AtomicUsize::new(0)),
            }
        }

        fn close_calls(&self) -> usize {
            self.close_calls.load(Ordering::SeqCst)
        }
    }

    impl IsolationProvider for TestProvider {
        fn id(&self) -> &IsolationIdentifier {
            &self.id
        }

        fn establish(
            &self,
            _request: &IsolationRequest,
        ) -> IsolationResult<Box<dyn IsolationSession>> {
            Ok(Box::new(TestSession {
                id: identifier("test.session.instance"),
                evidence: self.evidence.clone(),
                active: AtomicBool::new(
                    self.active.load(Ordering::SeqCst),
                ),
                close_calls: self.close_calls.clone(),
                context: "test-provider-context".to_owned(),
            }))
        }
    }

    fn manager(
        provider: Arc<TestProvider>,
    ) -> IsolationManager {
        IsolationManager::new(provider)
    }

    fn request(
        directives: Vec<IsolationDirective>,
    ) -> IsolationRequest {
        IsolationRequest::new(
            identifier("execution.subject"),
            directives,
        )
        .expect("test request must be valid")
    }

    #[test]
    fn identifier_rejects_empty_values() {
        assert!(
            IsolationIdentifier::new("")
                .is_err()
        );
    }

    #[test]
    fn identifier_rejects_whitespace_and_control_characters() {
        assert!(
            IsolationIdentifier::new("security control")
                .is_err()
        );

        assert!(
            IsolationIdentifier::new("security\ncontrol")
                .is_err()
        );

        assert!(
            IsolationIdentifier::new(" security.control")
                .is_err()
        );
    }

    #[test]
    fn namespaced_future_identifiers_are_supported() {
        let value = identifier(
            "future.security.confidential_execution",
        );

        assert_eq!(
            value.as_str(),
            "future.security.confidential_execution",
        );
    }

    #[test]
    fn conflicting_directives_are_rejected() {
        let result = IsolationRequest::new(
            identifier("execution.subject"),
            vec![
                directive(
                    "security.network_access",
                    IsolationDisposition::Required,
                ),
                directive(
                    "security.network_access",
                    IsolationDisposition::Prohibited,
                ),
            ],
        );

        assert_eq!(
            result.expect_err("conflicting directives must fail").kind(),
            IsolationErrorKind::InvalidRequest,
        );
    }

    #[test]
    fn mandatory_control_requires_affirmative_evidence() {
        let provider = Arc::new(TestProvider::new(
            "provider.local",
            "provider.local",
            vec![(
                identifier("security.process_isolation"),
                IsolationControlState::Enabled,
            )],
        ));

        let request = request(vec![
            directive(
                "security.process_isolation",
                IsolationDisposition::Required,
            ),
        ]);

        let lease = manager(provider)
            .establish(&request)
            .expect("mandatory control should be satisfied");

        assert_eq!(
            lease.report().satisfied_controls().len(),
            1,
        );

        assert!(lease.is_active().expect("session status"));
    }

    #[test]
    fn unknown_mandatory_control_fails_closed() {
        let provider = Arc::new(TestProvider::new(
            "provider.local",
            "provider.local",
            Vec::new(),
        ));

        let request = request(vec![
            directive(
                "security.memory_isolation",
                IsolationDisposition::Required,
            ),
        ]);

        let result = manager(provider.clone())
            .establish(&request);

        assert_eq!(
            result.expect_err("unknown state must fail").kind(),
            IsolationErrorKind::ControlStateIndeterminate,
        );

        assert_eq!(provider.close_calls(), 1);
    }

    #[test]
    fn disabled_mandatory_control_is_rejected() {
        let provider = Arc::new(TestProvider::new(
            "provider.local",
            "provider.local",
            vec![(
                identifier("security.process_isolation"),
                IsolationControlState::Disabled,
            )],
        ));

        let request = request(vec![
            directive(
                "security.process_isolation",
                IsolationDisposition::Required,
            ),
        ]);

        let result = manager(provider)
            .establish(&request);

        assert_eq!(
            result.expect_err("disabled mandatory control must fail").kind(),
            IsolationErrorKind::RequiredControlUnsatisfied,
        );
    }

    #[test]
    fn prohibited_control_must_be_disabled() {
        let provider = Arc::new(TestProvider::new(
            "provider.local",
            "provider.local",
            vec![(
                identifier("security.network_access"),
                IsolationControlState::Enabled,
            )],
        ));

        let request = request(vec![
            directive(
                "security.network_access",
                IsolationDisposition::Prohibited,
            ),
        ]);

        let result = manager(provider)
            .establish(&request);

        assert_eq!(
            result.expect_err("prohibited control must fail").kind(),
            IsolationErrorKind::ProhibitedControlEnabled,
        );
    }

    #[test]
    fn disabled_preference_is_reported_without_becoming_a_guarantee() {
        let provider = Arc::new(TestProvider::new(
            "provider.local",
            "provider.local",
            vec![(
                identifier("security.confidential_memory"),
                IsolationControlState::Disabled,
            )],
        ));

        let request = request(vec![
            directive(
                "security.confidential_memory",
                IsolationDisposition::Preferred,
            ),
        ]);

        let lease = manager(provider)
            .establish(&request)
            .expect("an unmet preference is not a mandatory failure");

        assert_eq!(
            lease.report().unmet_preferences(),
            &[identifier("security.confidential_memory")],
        );
    }

    #[test]
    fn missing_preference_evidence_is_reported() {
        let provider = Arc::new(TestProvider::new(
            "provider.local",
            "provider.local",
            Vec::new(),
        ));

        let request = request(vec![
            directive(
                "future.security.feature",
                IsolationDisposition::Preferred,
            ),
        ]);

        let lease = manager(provider)
            .establish(&request)
            .expect("unknown preference is not mandatory");

        assert_eq!(
            lease.report().unmet_preferences(),
            &[identifier("future.security.feature")],
        );
    }

    #[test]
    fn mismatched_provider_evidence_is_rejected() {
        let provider = Arc::new(TestProvider::new(
            "provider.local",
            "provider.other",
            Vec::new(),
        ));

        let request = request(Vec::new());

        let result = manager(provider.clone())
            .establish(&request);

        assert_eq!(
            result.expect_err("provider mismatch must fail").kind(),
            IsolationErrorKind::ProviderIdentityMismatch,
        );

        assert_eq!(provider.close_calls(), 1);
    }

    #[test]
    fn provider_context_is_available_to_execution_adapters() {
        let provider = Arc::new(TestProvider::new(
            "provider.local",
            "provider.local",
            Vec::new(),
        ));

        let lease = manager(provider)
            .establish(&request(Vec::new()))
            .expect("isolation establishment");

        let context = lease
            .session()
            .expect("active session")
            .provider_context()
            .downcast_ref::<String>()
            .expect("provider context type");

        assert_eq!(context, "test-provider-context");
    }

    #[test]
    fn explicit_close_releases_the_session() {
        let provider = Arc::new(TestProvider::new(
            "provider.local",
            "provider.local",
            Vec::new(),
        ));

        let mut lease = manager(provider.clone())
            .establish(&request(Vec::new()))
            .expect("isolation establishment");

        lease.close().expect("close isolation");

        assert!(!lease.is_active().expect("closed state"));
        assert_eq!(provider.close_calls(), 1);

        // A second close is idempotent.
        lease.close().expect("repeat close");
        assert_eq!(provider.close_calls(), 1);
    }

    #[test]
    fn dropping_a_lease_attempts_cleanup() {
        let provider = Arc::new(TestProvider::new(
            "provider.local",
            "provider.local",
            Vec::new(),
        ));

        let lease = manager(provider.clone())
            .establish(&request(Vec::new()))
            .expect("isolation establishment");

        drop(lease);

        assert_eq!(provider.close_calls(), 1);
    }

    #[test]
    fn empty_directives_can_establish_a_baseline_boundary() {
        let provider = Arc::new(TestProvider::new(
            "provider.local",
            "provider.local",
            Vec::new(),
        ));

        let lease = manager(provider)
            .establish(&request(Vec::new()))
            .expect("provider may establish baseline isolation");

        assert!(lease.is_active().expect("active session"));
    }
}
