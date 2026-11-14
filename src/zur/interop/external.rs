
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — External Interoperability
//!
//! Path: `src/zur/interop/external.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the operational boundary between ZUR and independently
//! implemented external execution providers.
//!
//! External providers may represent:
//!
//! - Native libraries.
//! - Foreign-language runtimes.
//! - External processes.
//! - Operating-system services.
//! - Local or remote services.
//! - IPC endpoints.
//! - Network-backed execution environments.
//! - Classical, quantum, HDL, AI, hybrid, distributed or future domains.
//!
//! Computational domains and transport implementations are not
//! enumerated or hard-coded here.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - External invocation descriptors.
//! - Explicit authorization integration.
//! - Provider/interface/operation validation.
//! - Typed request-contract validation.
//! - Typed response-contract validation.
//! - Safe delegation to canonical ZUR execution providers.
//! - External synchronous invocation.
//! - External asynchronous submission.
//! - External streaming and session delegation.
//!
//! It does not own:
//!
//! - Source-language syntax or AST construction.
//! - Foreign-language semantics.
//! - ABI declarations or ABI implementation.
//! - Artifact manifests or artifact loading.
//! - Backend registration or code generation.
//! - Provider identity or provider registration.
//! - Provider discovery or provider selection.
//! - Capability definitions or capability evidence.
//! - Resource requirements or resource reservations.
//! - Authentication credentials or secret storage.
//! - Dynamic library loading.
//! - Native pointers or physical addresses.
//! - Process creation or process supervision.
//! - Network protocols or IPC implementations.
//! - Classical, quantum, HDL, AI or other domain IR.
//! - Hybrid or distributed execution planning.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ## Important distinctions
//!
//! A provider identifier is not a provider availability guarantee.
//!
//! Provider registration is not provider initialization.
//!
//! Provider initialization is not provider admission.
//!
//! Provider admission is not authorization.
//!
//! Authorization is not capability availability.
//!
//! Capability availability is not resource reservation.
//!
//! Submission is not execution completion.
//!
//! A returned execution result must represent the provider's actual
//! reported outcome.
//!
//! ## Security
//!
//! Every operation passes through an explicitly supplied authorizer.
//!
//! There is deliberately no implicit allow-all authorizer.
//!
//! The adapter does not store credentials, access tokens, passwords,
//! private keys, network secrets or authorization headers.
//!
//! External provider implementations are responsible for their own
//! transport security, authentication and isolation contracts.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No unsafe implementations.
//! No raw-pointer interface.
//! No global mutable state.
//! No implicit library loading.
//! No implicit process execution.
//! No fabricated execution success.
//!
//! ## Scalability
//!
//! No arbitrary universal limit is imposed on:
//!
//! - Providers.
//! - Interfaces.
//! - Operations.
//! - External services.
//! - Domains.
//! - Devices.
//! - Processes.
//! - Sessions.
//! - Stream items.
//! - Payload sizes.
//! - Resource quantities.
//!
//! Actual execution is bounded by available memory, address space,
//! provider capabilities, configured budgets and platform constraints.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## Integration
//!
//! - `zur/contracts/provider.rs` owns execution-provider contracts.
//! - `zur/contracts/abi.rs` owns the canonical runtime ABI.
//! - `zur/contracts/execution.rs` owns execution lifecycle contracts.
//! - `zur/error.rs` owns structured ZUR errors.
//! - `zur/ids.rs` owns ZUR execution and session identities.
//! - `stdlib/context.rs` owns operation context and cancellation.
//! - `stdlib/provider.rs` owns common provider identities and operation IDs.
//! - `zur/interop/abi.rs` adapts external ABI declarations.
//! - `zur/domains/` owns domain-provider integration.
//! - `security/` owns security policy and authorization authorities.
//! - `distributed/` owns distributed execution and transport.
//! - `quantum/`, `hybrid/`, `hdl/`, `ai/` and `classical/` retain
//!   their respective computational authorities.
//!
//! This module delegates execution. It does not implement a second
//! provider registry, transport system, ABI or execution engine.

use core::fmt;

use crate::stdlib::context::OperationContext;

use crate::stdlib::provider::{
    ProviderId,
    ProviderOperationId,
};

use crate::zur::contracts::provider::{
    validate_provider_contract,
    ExecutionProvider,
    ProviderInvocationMode,
    ProviderOperationRequest,
    ProviderOperationResponse,
    ProviderPayload,
    ProviderSession,
    ProviderStream,
    ProviderSubmission,
    ZurProviderFuture,
};

use crate::zur::contracts::provider::ProviderInterfaceId;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::ids::SessionId;

// ============================================================================
// ERROR CLASSIFICATION
// ============================================================================

/// Identifies an external interoperability failure.
///
/// This classification supplements the canonical ZUR operational
/// error model. It does not replace `ZurError`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ExternalInteropError {
    /// The external provider identity does not match the selected provider.
    ProviderIdentityMismatch,

    /// The requested interface is not declared by the provider.
    UnknownInterface,

    /// The provider does not support the requested invocation mode.
    UnsupportedInvocationMode,

    /// The requested operation is not declared by the interface.
    UnknownOperation,

    /// The operation does not declare a request contract.
    MissingRequestContract,

    /// The request payload does not satisfy the declared request contract.
    RequestContractMismatch,

    /// The operation does not declare a response contract.
    MissingResponseContract,

    /// The response payload does not satisfy the declared response contract.
    ResponseContractMismatch,

    /// The provider returned a different operation identity.
    ResponseOperationMismatch,

    /// The provider returned a submission for a different operation.
    SubmissionMismatch,

    /// A session belongs to a different provider.
    SessionProviderMismatch,

    /// The provider returned a session belonging to a different provider.
    ReturnedSessionProviderMismatch,

    /// The external invocation was rejected by authorization.
    AuthorizationDenied,

    /// The provider contract is structurally invalid.
    InvalidProviderContract,
}

impl fmt::Display for ExternalInteropError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(match self {
            Self::ProviderIdentityMismatch =>
                "external provider identity does not match the requested provider",

            Self::UnknownInterface =>
                "external provider does not declare the requested interface",

            Self::UnsupportedInvocationMode =>
                "external provider does not support the requested invocation mode",

            Self::UnknownOperation =>
                "external provider interface does not declare the requested operation",

            Self::MissingRequestContract =>
                "external operation does not declare a request contract",

            Self::RequestContractMismatch =>
                "external request payload does not match the declared request contract",

            Self::MissingResponseContract =>
                "external operation does not declare a response contract",

            Self::ResponseContractMismatch =>
                "external response payload does not match the declared response contract",

            Self::ResponseOperationMismatch =>
                "external provider returned a response for a different operation",

            Self::SubmissionMismatch =>
                "external provider returned a submission for a different operation",

            Self::SessionProviderMismatch =>
                "external session belongs to a different provider",

            Self::ReturnedSessionProviderMismatch =>
                "external provider returned a session for a different provider",

            Self::AuthorizationDenied =>
                "external invocation was rejected by the authorization authority",

            Self::InvalidProviderContract =>
                "external provider contract is structurally invalid",
        })
    }
}

impl std::error::Error for ExternalInteropError {}

fn external_error(
    kind: ExternalInteropError,
) -> ZurError {
    ZurError::invalid_data(kind.to_string())
        .with_component("zur.interop.external")
}

fn external_unsupported(
    kind: ExternalInteropError,
) -> ZurError {
    ZurError::unsupported(kind.to_string())
        .with_component("zur.interop.external")
}

// ============================================================================
// EXTERNAL CALL DESCRIPTOR
// ============================================================================

/// Identifies one requested external provider operation.
///
/// This descriptor contains logical identities only.
///
/// It deliberately does not contain:
///
/// - Filesystem paths.
/// - Dynamic-library filenames.
/// - Native addresses.
/// - Executable paths.
/// - Network URLs.
/// - Credentials.
/// - Operating-system process IDs.
/// - Hardware addresses.
///
/// Those details belong to the selected external provider
/// implementation and its security/configuration authorities.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternalCallDescriptor {
    provider: ProviderId,
    interface: ProviderInterfaceId,
    operation: ProviderOperationId,
}

impl ExternalCallDescriptor {
    /// Constructs an external invocation descriptor.
    ///
    /// All identities are supplied explicitly.
    ///
    /// This constructor does not discover, select, initialize,
    /// authorize or execute a provider.
    #[must_use]
    pub fn new(
        provider: ProviderId,
        interface: ProviderInterfaceId,
        operation: ProviderOperationId,
    ) -> Self {
        Self {
            provider,
            interface,
            operation,
        }
    }

    /// Returns the requested provider identity.
    #[must_use]
    pub fn provider(&self) -> &ProviderId {
        &self.provider
    }

    /// Returns the requested interface identity.
    #[must_use]
    pub fn interface(&self) -> &ProviderInterfaceId {
        &self.interface
    }

    /// Returns the requested operation identity.
    #[must_use]
    pub fn operation(&self) -> &ProviderOperationId {
        &self.operation
    }
}

// ============================================================================
// AUTHORIZATION CONTRACT
// ============================================================================

/// Authorization authority for external operations.
///
/// An implementation must consult the appropriate ZUR security
/// authority and enforce its current policy.
///
/// Authorization must not be inferred from:
///
/// - Provider registration.
/// - Provider identity.
/// - Interface declaration.
/// - Capability claims.
/// - Resource requirements.
/// - Successful provider initialization.
///
/// The authorizer is injected into the adapter. No implicit
/// permissive implementation is provided.
pub trait ExternalAuthorizer: Send + Sync {
    /// Authorizes an external operation.
    ///
    /// `payload` is `None` for operations which do not transfer
    /// an operation payload, such as closing a session.
    ///
    /// Implementations must return an error when authorization
    /// cannot be established.
    fn authorize(
        &self,
        call: &ExternalCallDescriptor,
        payload: Option<&dyn ProviderPayload>,
        context: &OperationContext,
    ) -> ZurResult<()>;
}

// ============================================================================
// VALIDATED OPERATION REFERENCE
// ============================================================================

/// A validated reference to an operation declared by a provider.
///
/// The reference borrows the provider's canonical contract.
/// It does not copy or redefine the operation descriptor.
struct ValidatedExternalOperation<'a> {
    operation: &'a crate::stdlib::provider::ProviderOperationDescriptor,
}

// ============================================================================
// ADAPTER
// ============================================================================

/// Stateless external execution adapter.
///
/// The adapter receives:
///
/// - An already selected provider.
/// - An explicit external call descriptor.
/// - An explicit authorization authority.
/// - An operation context.
/// - A typed request payload.
///
/// It validates the contract boundary and delegates to the
/// provider's existing execution methods.
///
/// It does not discover or select providers.
pub struct ExternalExecutionAdapter<A> {
    authorizer: A,
}

impl<A> ExternalExecutionAdapter<A>
where
    A: ExternalAuthorizer,
{
    /// Constructs an adapter with an explicit authorization authority.
    ///
    /// There is no default allow-all policy.
    #[must_use]
    pub const fn new(
        authorizer: A,
    ) -> Self {
        Self {
            authorizer,
        }
    }

    /// Returns the configured authorization authority.
    #[must_use]
    pub const fn authorizer(&self) -> &A {
        &self.authorizer
    }

    // ------------------------------------------------------------------------
    // INTERNAL VALIDATION
    // ------------------------------------------------------------------------

    /// Validates the provider, interface, operation and invocation mode.
    ///
    /// This method establishes structural compatibility only.
    ///
    /// It does not establish provider trust, availability, capability
    /// availability, resource reservation or execution feasibility.
    fn validate_operation<'a>(
        &self,
        provider: &'a dyn ExecutionProvider,
        call: &ExternalCallDescriptor,
        mode: ProviderInvocationMode,
        payload: &dyn ProviderPayload,
    ) -> ZurResult<ValidatedExternalOperation<'a>> {
        let contract = provider.execution_contract();

        validate_provider_contract(contract)
            .map_err(|_| {
                external_error(
                    ExternalInteropError::InvalidProviderContract,
                )
            })?;

        if contract.provider_id() != call.provider() {
            return Err(external_error(
                ExternalInteropError::ProviderIdentityMismatch,
            ));
        }

        let interface = contract
            .interface(call.interface())
            .ok_or_else(|| {
                external_error(
                    ExternalInteropError::UnknownInterface,
                )
            })?;

        if !interface.supports(mode) {
            return Err(external_unsupported(
                ExternalInteropError::UnsupportedInvocationMode,
            ));
        }

        let operation = interface
            .operations()
            .iter()
            .find(|candidate| {
                candidate.id() == call.operation()
            })
            .ok_or_else(|| {
                external_error(
                    ExternalInteropError::UnknownOperation,
                )
            })?;

        let expected_request = operation
            .request_contract()
            .ok_or_else(|| {
                external_error(
                    ExternalInteropError::MissingRequestContract,
                )
            })?;

        if payload.contract_id() != expected_request {
            return Err(external_error(
                ExternalInteropError::RequestContractMismatch,
            ));
        }

        Ok(ValidatedExternalOperation {
            operation,
        })
    }

    /// Validates the provider identity and interface for operations
    /// which do not carry a request payload.
    fn validate_empty_operation<'a>(
        &self,
        provider: &'a dyn ExecutionProvider,
        call: &ExternalCallDescriptor,
        mode: ProviderInvocationMode,
    ) -> ZurResult<&'a crate::zur::contracts::provider::ProviderInterface> {
        let contract = provider.execution_contract();

        validate_provider_contract(contract)
            .map_err(|_| {
                external_error(
                    ExternalInteropError::InvalidProviderContract,
                )
            })?;

        if contract.provider_id() != call.provider() {
            return Err(external_error(
                ExternalInteropError::ProviderIdentityMismatch,
            ));
        }

        let interface = contract
            .interface(call.interface())
            .ok_or_else(|| {
                external_error(
                    ExternalInteropError::UnknownInterface,
                )
            })?;

        if !interface.supports(mode) {
            return Err(external_unsupported(
                ExternalInteropError::UnsupportedInvocationMode,
            ));
        }

        if !interface.operations().iter().any(|operation| {
            operation.id() == call.operation()
        }) {
            return Err(external_error(
                ExternalInteropError::UnknownOperation,
            ));
        }

        Ok(interface)
    }

    /// Authorizes one external operation.
    fn authorize(
        &self,
        call: &ExternalCallDescriptor,
        payload: Option<&dyn ProviderPayload>,
        context: &OperationContext,
    ) -> ZurResult<()> {
        self.authorizer
            .authorize(
                call,
                payload,
                context,
            )
            .map_err(|error| {
                ZurError::unsupported(format!(
                    "{}: {}",
                    ExternalInteropError::AuthorizationDenied,
                    error,
                ))
                .with_component("zur.interop.external")
                .with_operation("authorize")
            })
    }

    /// Validates a completed provider response.
    ///
    /// A response is accepted only if both its operation identity
    /// and its declared response payload contract match.
    fn validate_response(
        &self,
        operation: &crate::stdlib::provider::ProviderOperationDescriptor,
        call: &ExternalCallDescriptor,
        response: &ProviderOperationResponse,
    ) -> ZurResult<()> {
        if response.operation() != call.operation() {
            return Err(external_error(
                ExternalInteropError::ResponseOperationMismatch,
            ));
        }

        let expected_response = operation
            .response_contract()
            .ok_or_else(|| {
                external_error(
                    ExternalInteropError::MissingResponseContract,
                )
            })?;

        if response.payload().contract_id() != expected_response {
            return Err(external_error(
                ExternalInteropError::ResponseContractMismatch,
            ));
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // SYNCHRONOUS INVOCATION
    // ------------------------------------------------------------------------

    /// Invokes one external operation and waits for its actual result.
    ///
    /// The interface must declare synchronous invocation support.
    ///
    /// The provider must return a completed, typed response.
    ///
    /// A submission receipt is not accepted as an execution result.
    pub fn invoke<'a>(
        &'a self,
        provider: &'a dyn ExecutionProvider,
        call: &'a ExternalCallDescriptor,
        payload: &'a dyn ProviderPayload,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, ProviderOperationResponse> {
        Box::pin(async move {
            let validated = self.validate_operation(
                provider,
                call,
                ProviderInvocationMode::Synchronous,
                payload,
            )?;

            self.authorize(
                call,
                Some(payload),
                context,
            )?;

            let request = ProviderOperationRequest::new(
                call.operation(),
                payload,
            );

            let response = provider
                .invoke(
                    request,
                    context,
                )
                .await?;

            self.validate_response(
                validated.operation,
                call,
                &response,
            )?;

            Ok(response)
        })
    }

    // ------------------------------------------------------------------------
    // ASYNCHRONOUS SUBMISSION
    // ------------------------------------------------------------------------

    /// Submits an external operation for provider-managed asynchronous
    /// execution.
    ///
    /// The interface must declare asynchronous invocation support.
    ///
    /// The returned value is a submission receipt, not proof that
    /// the operation completed successfully.
    ///
    /// Completion must be observed through the provider's declared
    /// execution/session observation contract.
    pub fn submit<'a>(
        &'a self,
        provider: &'a dyn ExecutionProvider,
        call: &'a ExternalCallDescriptor,
        payload: &'a dyn ProviderPayload,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, ProviderSubmission> {
        Box::pin(async move {
            self.validate_operation(
                provider,
                call,
                ProviderInvocationMode::Asynchronous,
                payload,
            )?;

            self.authorize(
                call,
                Some(payload),
                context,
            )?;

            let request = ProviderOperationRequest::new(
                call.operation(),
                payload,
            );

            let submission = provider
                .submit(
                    request,
                    context,
                )
                .await?;

            if submission.provider_id() != call.provider()
                || submission.operation() != call.operation()
            {
                return Err(external_error(
                    ExternalInteropError::SubmissionMismatch,
                ));
            }

            Ok(submission)
        })
    }

    // ------------------------------------------------------------------------
    // STREAMING
    // ------------------------------------------------------------------------

    /// Opens an external streaming operation.
    ///
    /// The interface must declare streaming support.
    ///
    /// The returned stream remains owned by the provider.
    ///
    /// Consumers must validate each received item against the
    /// operation's declared response contract.
    pub fn open_stream<'a>(
        &'a self,
        provider: &'a dyn ExecutionProvider,
        call: &'a ExternalCallDescriptor,
        payload: &'a dyn ProviderPayload,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, Box<dyn ProviderStream>> {
        Box::pin(async move {
            self.validate_operation(
                provider,
                call,
                ProviderInvocationMode::Streaming,
                payload,
            )?;

            self.authorize(
                call,
                Some(payload),
                context,
            )?;

            let request = ProviderOperationRequest::new(
                call.operation(),
                payload,
            );

            provider
                .open_stream(
                    request,
                    context,
                )
                .await
        })
    }

    // ------------------------------------------------------------------------
    // SESSION CREATION
    // ------------------------------------------------------------------------

    /// Opens an external provider-managed session.
    ///
    /// The interface must declare session support.
    ///
    /// The session identity is supplied by ZUR.
    ///
    /// A returned session does not independently prove physical
    /// resource reservation.
    pub fn open_session<'a>(
        &'a self,
        provider: &'a dyn ExecutionProvider,
        call: &'a ExternalCallDescriptor,
        session_id: SessionId,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, Box<dyn ProviderSession>> {
        Box::pin(async move {
            self.validate_empty_operation(
                provider,
                call,
                ProviderInvocationMode::Session,
            )?;

            self.authorize(
                call,
                None,
                context,
            )?;

            let session = provider
                .open_session(
                    session_id,
                    context,
                )
                .await?;

            if session.provider_id() != call.provider() {
                return Err(external_error(
                    ExternalInteropError::ReturnedSessionProviderMismatch,
                ));
            }

            Ok(session)
        })
    }

    // ------------------------------------------------------------------------
    // SESSION INVOCATION
    // ------------------------------------------------------------------------

    /// Invokes an operation within an existing external session.
    ///
    /// The session must belong to the selected provider.
    ///
    /// The interface must declare session invocation support.
    pub fn invoke_session<'a>(
        &'a self,
        provider: &'a dyn ExecutionProvider,
        call: &'a ExternalCallDescriptor,
        session: &'a dyn ProviderSession,
        payload: &'a dyn ProviderPayload,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, ProviderOperationResponse> {
        Box::pin(async move {
            if session.provider_id() != call.provider() {
                return Err(external_error(
                    ExternalInteropError::SessionProviderMismatch,
                ));
            }

            let validated = self.validate_operation(
                provider,
                call,
                ProviderInvocationMode::Session,
                payload,
            )?;

            self.authorize(
                call,
                Some(payload),
                context,
            )?;

            let request = ProviderOperationRequest::new(
                call.operation(),
                payload,
            );

            let response = provider
                .invoke_session(
                    session,
                    request,
                    context,
                )
                .await?;

            self.validate_response(
                validated.operation,
                call,
                &response,
            )?;

            Ok(response)
        })
    }

    // ------------------------------------------------------------------------
    // SESSION CLOSURE
    // ------------------------------------------------------------------------

    /// Closes an external provider-managed session.
    ///
    /// The session must belong to the selected provider.
    ///
    /// Successful completion represents the provider's declared
    /// session-close operation. It does not claim that unrelated
    /// physical resources have been released.
    pub fn close_session<'a>(
        &'a self,
        provider: &'a dyn ExecutionProvider,
        call: &'a ExternalCallDescriptor,
        session: &'a dyn ProviderSession,
        context: &'a OperationContext,
    ) -> ZurProviderFuture<'a, ()> {
        Box::pin(async move {
            if session.provider_id() != call.provider() {
                return Err(external_error(
                    ExternalInteropError::SessionProviderMismatch,
                ));
            }

            self.validate_empty_operation(
                provider,
                call,
                ProviderInvocationMode::Session,
            )?;

            self.authorize(
                call,
                None,
                context,
            )?;

            provider
                .close_session(
                    session,
                    context,
                )
                .await
        })
    }
}

// ============================================================================
// TESTING SUPPORT
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn external_error_categories_are_distinct() {
        assert_ne!(
            ExternalInteropError::UnknownProvider,
            ExternalInteropError::UnknownInterface,
        );
    }
}
