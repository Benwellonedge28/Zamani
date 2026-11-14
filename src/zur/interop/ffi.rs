
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Foreign Function Interface
//!
//! Path: `src/zur/interop/ffi.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Provides the ZUR runtime integration boundary for foreign-function
//! invocation.
//!
//! It connects semantically validated foreign-function declarations and
//! canonical runtime ABI contracts to explicitly supplied foreign-call
//! execution adapters.
//!
//! This module supports interoperability with:
//!
//! - Native libraries.
//! - Other programming languages.
//! - Operating-system services.
//! - External runtime environments.
//! - Accelerator services.
//! - FPGA and ASIC execution services.
//! - Quantum execution environments.
//! - Distributed services.
//! - Future computational technologies.
//!
//! Computational domains and foreign implementation technologies are
//! intentionally open-ended.
//!
//! # ARCHITECTURAL AUTHORITIES
//!
//! Source syntax:
//!     grammar/interoperability/ffi.g4
//!
//! Foreign declarations:
//!     grammar/interoperability/foreign-functions.g4
//!
//! ABI syntax:
//!     grammar/interoperability/abi.g4
//!
//! Calling-convention syntax:
//!     grammar/interoperability/calling-conventions.g4
//!
//! Normative interoperability contract:
//!     grammar/spec/interoperability.md
//!
//! Semantic authority:
//!     crate::frontend::semantic
//!
//! Canonical runtime ABI:
//!     crate::zur::contracts::abi
//!
//! Runtime errors:
//!     crate::zur::error
//!
//! Operation context and cancellation:
//!     crate::stdlib::context
//!
//! Runtime authorization:
//!     crate::zur::security::permissions
//!
//! Classical external-call representation:
//!     crate::classical::interoperability
//!
//! Quantum external-call representation:
//!     crate::quantum::ir::classical::extern_call
//!
//! # OWNERSHIP
//!
//! This module owns:
//!
//! - Safe foreign-call invocation requests.
//! - Foreign-call argument transport.
//! - Opaque external handle references.
//! - Foreign-call execution adapter interfaces.
//! - Foreign-call authorization adapter interfaces.
//! - Foreign-call response validation.
//! - Foreign-call dispatch coordination.
//!
//! This module does NOT own:
//!
//! - Source-language parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Source-language type definitions.
//! - ABI definitions.
//! - ABI versioning.
//! - Calling-convention implementation.
//! - Foreign-language type systems.
//! - Serialization algorithms.
//! - Native dynamic-library loading.
//! - Native symbol resolution.
//! - Linker implementations.
//! - Backend registration.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Quantum routing or scheduling.
//! - Distributed execution planning.
//! - Hybrid execution planning.
//! - Authentication.
//! - Authorization policy definitions.
//! - Foreign-language exception handling.
//! - Physical execution.
//!
//! # CRITICAL SAFETY CONTRACT
//!
//! This module exposes no raw pointers, native function pointers,
//! pointer dereferencing, arbitrary memory access, or unsafe operations.
//!
//! Actual foreign invocation is delegated to a supplied implementation
//! of ForeignCallExecutor.
//!
//! That implementation must itself satisfy the repository's safe-Rust
//! policy. Native bindings must use an appropriately safe implementation
//! or an externally verified boundary that does not introduce unsafe
//! into this crate.
//!
//! An unavailable invocation mechanism is an error, never simulated
//! success.
//!
//! # SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Foreign functions.
//! - Arguments.
//! - Results.
//! - Calling conventions.
//! - Foreign languages.
//! - Providers.
//! - Interfaces.
//! - Devices.
//! - Computational domains.
//! - Hardware resources.
//! - Distributed participants.
//!
//! Collections grow subject to available memory, address-space limits,
//! explicit operational budgets, and actual implementation capabilities.
//!
//! Literal infinite physical computation is not promised.
//!
//! # OWNERSHIP AND PORTABILITY
//!
//! ABI contracts remain owned by zur::contracts::abi.
//!
//! Foreign value bytes are opaque ABI-encoded representations.
//!
//! This module does not invent a universal binary encoding for all
//! programming languages or computational domains.
//!
//! A compatible executor must implement the applicable marshalling,
//! calling-convention, ownership, lifetime, and error-transfer rules.
//!
//! # SECURITY
//!
//! Every foreign invocation requires an explicitly supplied authorizer.
//!
//! There is no implicit allow-all authorizer.
//!
//! Authentication and authorization remain owned by their canonical
//! security authorities.
//!
//! Foreign-call providers must not receive credentials through ordinary
//! argument metadata.
//!
//! # FAILURE BEHAVIOR
//!
//! Authorization failure prevents invocation.
//!
//! Unsupported calling conventions prevent invocation.
//!
//! Invalid arguments prevent invocation.
//!
//! Invalid results are rejected.
//!
//! Provider errors are propagated.
//!
//! Cancellation is cooperative and does not imply that an external
//! operation has physically stopped.
//!
//! Submission and successful completion are distinct outcomes.

use core::fmt;
use core::future::Future;
use core::pin::Pin;

use std::sync::Arc;

use crate::stdlib::context::OperationContext;

use crate::zur::contracts::abi::{
    AbiSymbol,
    AbiTypeId,
    CallingConvention,
    EntryPointDescriptor,
    InvocationMode,
    OwnershipMode,
    PassingMode,
    ValueContract,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

// ============================================================================
// FUTURE
// ============================================================================

/// Future returned by a foreign-call execution adapter.
///
/// The future is Send so compatible runtime executors can drive
/// foreign calls across multithreaded execution environments.
///
/// Implementations that require single-threaded operation may provide
/// an appropriate compatible adapter.
pub type ForeignCallFuture<'a> =
    Pin<Box<dyn Future<Output = ZurResult<ForeignCallResponse>> + Send + 'a>>;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn ffi_input_error(message: impl Into<String>) -> ZurError {
    ZurError::invalid_input(message)
        .with_component("zur.interop.ffi")
}

fn ffi_data_error(message: impl Into<String>) -> ZurError {
    ZurError::invalid_data(message)
        .with_component("zur.interop.ffi")
}

fn ffi_unsupported_error(message: impl Into<String>) -> ZurError {
    ZurError::unsupported(message)
        .with_component("zur.interop.ffi")
}

// ============================================================================
// EXTERNAL HANDLE
// ============================================================================

/// An opaque identifier for an externally managed object.
///
/// This identifier is not a memory address, native pointer, device
/// address, or physical resource location.
///
/// The actual meaning and lifetime of the referenced object belong
/// to the external provider and its declared ABI contract.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ForeignHandle {
    identity: String,
}

impl ForeignHandle {
    /// Constructs an opaque external handle identity.
    ///
    /// Empty identities, surrounding whitespace, and control
    /// characters are rejected.
    pub fn new(
        identity: impl Into<String>,
    ) -> ZurResult<Self> {
        let identity = identity.into();

        if identity.is_empty()
            || identity.trim() != identity
            || identity.chars().any(char::is_control)
        {
            return Err(ffi_input_error(
                "foreign handle identity is invalid",
            ));
        }

        Ok(Self { identity })
    }

    /// Returns the opaque identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.identity
    }
}

impl fmt::Display for ForeignHandle {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// FOREIGN VALUE
// ============================================================================

/// Payload transported across a foreign-call boundary.
///
/// Encoded bytes are owned by the request or response.
///
/// Handle references identify objects owned by an external authority.
///
/// Neither representation exposes native memory addresses.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ForeignValuePayload {
    Encoded(Vec<u8>),
    Handle(ForeignHandle),
}

/// A typed, safely transported foreign ABI value.
///
/// The ABI type identity comes from the canonical ZUR ABI contract.
///
/// Encoded bytes must use the representation established by the
/// applicable ABI marshalling implementation.
///
/// This type does not independently interpret those bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignValue {
    type_id: AbiTypeId,
    payload: ForeignValuePayload,
}

impl ForeignValue {
    /// Constructs an encoded foreign value.
    ///
    /// The bytes are opaque and are not interpreted by this module.
    pub fn encoded(
        type_id: AbiTypeId,
        bytes: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            type_id,
            payload: ForeignValuePayload::Encoded(
                bytes.into(),
            ),
        }
    }

    /// Constructs a value containing an opaque external handle.
    pub fn handle(
        type_id: AbiTypeId,
        handle: ForeignHandle,
    ) -> Self {
        Self {
            type_id,
            payload: ForeignValuePayload::Handle(handle),
        }
    }

    /// Returns the canonical ABI type identity.
    #[must_use]
    pub fn type_id(&self) -> &AbiTypeId {
        &self.type_id
    }

    /// Returns encoded bytes when this value uses byte transport.
    #[must_use]
    pub fn encoded_bytes(&self) -> Option<&[u8]> {
        match &self.payload {
            ForeignValuePayload::Encoded(bytes) => {
                Some(bytes)
            }

            ForeignValuePayload::Handle(_) => None,
        }
    }

    /// Returns the opaque handle when this value uses handle transport.
    #[must_use]
    pub fn external_handle(&self) -> Option<&ForeignHandle> {
        match &self.payload {
            ForeignValuePayload::Encoded(_) => None,

            ForeignValuePayload::Handle(handle) => {
                Some(handle)
            }
        }
    }

    fn encoded_bytes_mut(&mut self) -> Option<&mut Vec<u8>> {
        match &mut self.payload {
            ForeignValuePayload::Encoded(bytes) => {
                Some(bytes)
            }

            ForeignValuePayload::Handle(_) => None,
        }
    }

    fn has_handle_payload(&self) -> bool {
        matches!(
            self.payload,
            ForeignValuePayload::Handle(_)
        )
    }
}

// ============================================================================
// FOREIGN VALUE VALIDATION
// ============================================================================

fn validate_value(
    value: &ForeignValue,
    contract: &ValueContract,
    description: &str,
) -> ZurResult<()> {
    if value.type_id() != contract.type_id() {
        return Err(ffi_data_error(
            format!(
                "{description} has ABI type `{}`, expected `{}`",
                value.type_id(),
                contract.type_id(),
            ),
        ));
    }

    let expects_handle = matches!(
        contract.passing(),
        PassingMode::OpaqueHandle | PassingMode::Stream
    );

    if value.has_handle_payload() != expects_handle {
        return Err(ffi_data_error(
            format!(
                "{description} uses a transport representation \
                 inconsistent with its ABI passing mode"
            ),
        ));
    }

    Ok(())
}

// ============================================================================
// FOREIGN CALL REQUEST
// ============================================================================

/// A validated foreign-function invocation request.
///
/// The entry-point descriptor is cloned from the canonical ABI.
/// No second ABI schema is introduced.
///
/// Arguments preserve their declared positional order.
///
/// An argument may be consumed only when its canonical ABI ownership
/// contract declares ownership transfer to the receiver.
#[derive(Debug)]
pub struct ForeignCallRequest {
    entry_point: EntryPointDescriptor,
    arguments: Vec<Option<ForeignValue>>,
}

impl ForeignCallRequest {
    /// Constructs a request and validates its argument contracts.
    ///
    /// The argument count and ABI type identities must match the
    /// canonical entry-point descriptor.
    pub fn new(
        entry_point: EntryPointDescriptor,
        arguments: Vec<ForeignValue>,
    ) -> ZurResult<Self> {
        let parameters = entry_point.parameters();

        if arguments.len() != parameters.len() {
            return Err(ffi_input_error(
                format!(
                    "foreign function `{}` expects {} arguments, received {}",
                    entry_point.symbol(),
                    parameters.len(),
                    arguments.len(),
                ),
            ));
        }

        for (index, (argument, parameter)) in arguments
            .iter()
            .zip(parameters.iter())
            .enumerate()
        {
            validate_value(
                argument,
                parameter.contract(),
                &format!("argument {index}"),
            )?;
        }

        Ok(Self {
            entry_point,
            arguments: arguments.into_iter().map(Some).collect(),
        })
    }

    /// Returns the canonical entry-point descriptor.
    #[must_use]
    pub fn entry_point(&self) -> &EntryPointDescriptor {
        &self.entry_point
    }

    /// Returns the canonical foreign symbol.
    #[must_use]
    pub fn symbol(&self) -> &AbiSymbol {
        self.entry_point.symbol()
    }

    /// Returns the number of declared arguments.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Returns the arguments in declaration order.
    ///
    /// A transferred argument may no longer be present.
    pub fn arguments(
        &self,
    ) -> impl Iterator<Item = Option<&ForeignValue>> {
        self.arguments.iter().map(Option::as_ref)
    }

    /// Returns an argument by position.
    ///
    /// Returns None when the position is invalid or the argument
    /// has already been transferred.
    #[must_use]
    pub fn argument(
        &self,
        index: usize,
    ) -> Option<&ForeignValue> {
        self.arguments.get(index)?.as_ref()
    }

    /// Returns mutable encoded bytes for an argument whose ABI
    /// passing mode is BorrowedMutable.
    ///
    /// Other passing modes cannot use this operation.
    pub fn argument_bytes_mut(
        &mut self,
        index: usize,
    ) -> ZurResult<&mut [u8]> {
        let parameter = self
            .entry_point
            .parameters()
            .get(index)
            .ok_or_else(|| {
                ffi_input_error(
                    "foreign argument index is invalid",
                )
            })?;

        if parameter.contract().passing()
            != PassingMode::BorrowedMutable
        {
            return Err(ffi_input_error(
                "mutable access requires BorrowedMutable ABI passing mode",
            ));
        }

        let argument = self
            .arguments
            .get_mut(index)
            .and_then(Option::as_mut)
            .ok_or_else(|| {
                ffi_input_error(
                    "foreign argument has already been transferred",
                )
            })?;

        argument
            .encoded_bytes_mut()
            .map(Vec::as_mut_slice)
            .ok_or_else(|| {
                ffi_data_error(
                    "mutable encoded bytes are unavailable",
                )
            })
    }

    /// Transfers an argument to the foreign-call executor.
    ///
    /// This operation is permitted only when the canonical ABI
    /// declares OwnershipMode::TransferredToReceiver.
    ///
    /// The argument cannot subsequently be retrieved from the request.
    pub fn take_transferred_argument(
        &mut self,
        index: usize,
    ) -> ZurResult<ForeignValue> {
        let parameter = self
            .entry_point
            .parameters()
            .get(index)
            .ok_or_else(|| {
                ffi_input_error(
                    "foreign argument index is invalid",
                )
            })?;

        if parameter.contract().ownership()
            != OwnershipMode::TransferredToReceiver
        {
            return Err(ffi_input_error(
                "argument ownership is not transferable",
            ));
        }

        self.arguments
            .get_mut(index)
            .and_then(Option::take)
            .ok_or_else(|| {
                ffi_input_error(
                    "foreign argument has already been transferred",
                )
            })
    }
}

// ============================================================================
// FOREIGN CALL OUTPUT
// ============================================================================

/// Actual output category of a foreign invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForeignCallOutput {
    /// The invocation completed without a return value.
    Unit,

    /// The invocation produced an ordinary ABI value.
    Value(ForeignValue),

    /// The invocation established a streaming result.
    Stream(ForeignValue),

    /// The invocation established a persistent session.
    Session(ForeignValue),
}

/// Successful foreign-call response.
///
/// A response represents an actual result reported by the execution
/// adapter, not merely acceptance or submission of a request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignCallResponse {
    output: ForeignCallOutput,
}

impl ForeignCallResponse {
    /// Constructs a foreign-call response.
    ///
    /// Full validation is performed by ForeignCallService before
    /// the response is returned to its caller.
    #[must_use]
    pub fn new(output: ForeignCallOutput) -> Self {
        Self { output }
    }

    /// Returns the actual invocation output.
    #[must_use]
    pub fn output(&self) -> &ForeignCallOutput {
        &self.output
    }
}

// ============================================================================
// AUTHORIZATION CONTRACT
// ============================================================================

/// Authorization adapter for foreign-function invocation.
///
/// Implementations must delegate to the canonical security authority.
///
/// This trait does not define authentication, permissions, policy
/// evaluation, or authorization grants.
///
/// There is intentionally no default allow-all implementation.
pub trait ForeignCallAuthorizer: Send + Sync {
    /// Authorizes an invocation before it reaches an executor.
    ///
    /// An error prevents the executor from being invoked.
    fn authorize(
        &self,
        context: &OperationContext,
        entry_point: &EntryPointDescriptor,
    ) -> ZurResult<()>;
}

// ============================================================================
// EXECUTION ADAPTER CONTRACT
// ============================================================================

/// Safe execution adapter for foreign-function calls.
///
/// Implementations may delegate to an appropriately safe native
/// binding, host service, IPC transport, or external execution system.
///
/// The adapter must not claim that a calling convention is supported
/// unless it can actually implement the corresponding contract.
pub trait ForeignCallExecutor: Send + Sync {
    /// Returns the stable identity of this executor.
    fn provider_identity(&self) -> &str;

    /// Determines whether this executor implements the supplied
    /// calling convention and invocation mode.
    ///
    /// This is a provider compatibility declaration, not proof
    /// of current physical resource availability.
    fn supports(
        &self,
        convention: &CallingConvention,
        mode: InvocationMode,
    ) -> bool;

    /// Executes an authorized foreign-call request.
    ///
    /// The mutable request permits the executor to access mutable
    /// borrowed arguments and consume transferred arguments.
    ///
    /// The operation context provides cooperative cancellation
    /// observation and caller-supplied metadata.
    ///
    /// The future must report the actual execution outcome.
    ///
    /// Submission alone must not be reported as successful completion.
    fn invoke<'a>(
        &'a self,
        context: &'a OperationContext,
        request: &'a mut ForeignCallRequest,
    ) -> ForeignCallFuture<'a>;
}

// ============================================================================
// FOREIGN CALL SERVICE
// ============================================================================

/// Coordinates authorization, compatibility checks, foreign execution,
/// and result validation.
///
/// The authorizer and executor are explicitly injected.
///
/// No global provider registry, implicit library loader, hardware
/// discovery, or implicit security configuration is created here.
pub struct ForeignCallService {
    authorizer: Arc<dyn ForeignCallAuthorizer>,
    executor: Arc<dyn ForeignCallExecutor>,
    provider_identity: String,
}

impl ForeignCallService {
    /// Constructs a foreign-call service.
    ///
    /// Both the authorizer and executor are mandatory.
    ///
    /// The executor must provide a nonempty stable provider identity.
    pub fn new(
        authorizer: Arc<dyn ForeignCallAuthorizer>,
        executor: Arc<dyn ForeignCallExecutor>,
    ) -> ZurResult<Self> {
        let provider_identity =
            executor.provider_identity().to_owned();

        if provider_identity.is_empty()
            || provider_identity.trim() != provider_identity
            || provider_identity.chars().any(char::is_control)
        {
            return Err(ffi_input_error(
                "foreign executor identity is invalid",
            ));
        }

        Ok(Self {
            authorizer,
            executor,
            provider_identity,
        })
    }

    /// Returns the executor identity.
    #[must_use]
    pub fn provider_identity(&self) -> &str {
        &self.provider_identity
    }

    /// Authorizes and invokes a foreign function.
    ///
    /// Processing order:
    ///
    /// 1. Check cooperative cancellation.
    /// 2. Authorize the declared operation.
    /// 3. Check cancellation again.
    /// 4. Verify calling-convention and invocation-mode support.
    /// 5. Invoke the supplied executor.
    /// 6. Validate the returned ABI value.
    /// 7. Return the actual response.
    ///
    /// Authorization failure, unsupported functionality, provider
    /// failure, and invalid results remain failures.
    ///
    /// Cancellation after external submission is controlled by the
    /// executor and the external system. This method does not claim
    /// that cancellation forcibly stops external execution.
    pub fn invoke<'a>(
        &'a self,
        context: &'a OperationContext,
        mut request: ForeignCallRequest,
    ) -> ForeignCallFuture<'a> {
        Box::pin(async move {
            if context.is_cancelled() {
                return Err(
                    ZurError::cancelled(
                        "foreign invocation cancelled before authorization",
                    )
                    .with_component("zur.interop.ffi"),
                );
            }

            self.authorizer
                .authorize(context, request.entry_point())
                .map_err(|error| {
                    error
                        .with_component("zur.interop.ffi")
                        .with_operation("authorize")
                })?;

            if context.is_cancelled() {
                return Err(
                    ZurError::cancelled(
                        "foreign invocation cancelled before dispatch",
                    )
                    .with_component("zur.interop.ffi"),
                );
            }

            let entry_point = request.entry_point();

            if !self.executor.supports(
                entry_point.calling_convention(),
                entry_point.invocation_mode(),
            ) {
                return Err(
                    ZurError::capability_unavailable(
                        format!(
                            "executor `{}` does not support calling convention `{}` or invocation mode `{:?}`",
                            self.provider_identity,
                            entry_point.calling_convention(),
                            entry_point.invocation_mode(),
                        ),
                    )
                    .with_component("zur.interop.ffi")
                    .with_provider(self.provider_identity.clone()),
                );
            }

            let response = self
                .executor
                .invoke(context, &mut request)
                .await
                .map_err(|error| {
                    error
                        .with_component("zur.interop.ffi")
                        .with_operation("foreign_invocation")
                        .with_provider(self.provider_identity.clone())
                })?;

            validate_response(
                request.entry_point(),
                &response,
            )
            .map_err(|error| {
                error
                    .with_provider(self.provider_identity.clone())
                    .with_operation("validate_foreign_response")
            })?;

            Ok(response)
        })
    }
}

// ============================================================================
// RESPONSE VALIDATION
// ============================================================================

fn validate_response(
    entry_point: &EntryPointDescriptor,
    response: &ForeignCallResponse,
) -> ZurResult<()> {
    let expected = entry_point.result();

    match (
        entry_point.invocation_mode(),
        expected,
        response.output(),
    ) {
        (
            InvocationMode::Synchronous
                | InvocationMode::Asynchronous,
            None,
            ForeignCallOutput::Unit,
        ) => Ok(()),

        (
            InvocationMode::Synchronous
                | InvocationMode::Asynchronous,
            Some(contract),
            ForeignCallOutput::Value(value),
        ) => {
            if contract.passing() == PassingMode::Stream {
                return Err(ffi_data_error(
                    "stream result requires streaming invocation mode",
                ));
            }

            validate_value(
                value,
                contract,
                "foreign result",
            )
        }

        (
            InvocationMode::Streaming,
            Some(contract),
            ForeignCallOutput::Stream(value),
        ) => {
            if contract.passing() != PassingMode::Stream {
                return Err(ffi_data_error(
                    "stream output requires a stream ABI result contract",
                ));
            }

            validate_value(
                value,
                contract,
                "foreign stream result",
            )
        }

        (
            InvocationMode::Session,
            Some(contract),
            ForeignCallOutput::Session(value),
        ) => {
            if contract.passing() != PassingMode::OpaqueHandle {
                return Err(ffi_data_error(
                    "session output requires an opaque-handle ABI result contract",
                ));
            }

            validate_value(
                value,
                contract,
                "foreign session result",
            )
        }

        _ => Err(ffi_data_error(
            format!(
                "foreign executor returned an output inconsistent with ABI invocation mode `{:?}` and its result contract",
                entry_point.invocation_mode(),
            ),
        )),
    }
}

// ============================================================================
// DEBUG
// ============================================================================

impl fmt::Debug for ForeignCallService {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ForeignCallService")
            .field(
                "provider_identity",
                &self.provider_identity,
            )
            .field(
                "authorizer",
                &"[INJECTED AUTHORIZER]",
            )
            .field(
                "executor",
                &"[INJECTED EXECUTOR]",
            )
            .finish()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::task::{Context, Poll, Wake, Waker};

    use std::sync::atomic::{
        AtomicUsize,
        Ordering as AtomicOrdering,
    };

    use std::task::Waker as TaskWaker;

    struct NoopWake;

    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}
    }

    fn poll_ready<F: Future>(
        future: F,
    ) -> F::Output {
        let waker = TaskWaker::from(
            Arc::new(NoopWake),
        );

        let mut context = Context::from_waker(&waker);

        let mut future = Box::pin(future);

        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => output,

            Poll::Pending => {
                panic!("test future unexpectedly pending")
            }
        }
    }

    fn type_id(value: &str) -> AbiTypeId {
        AbiTypeId::new(value)
            .expect("valid test ABI type")
    }

    fn symbol(value: &str) -> AbiSymbol {
        AbiSymbol::new(value)
            .expect("valid test ABI symbol")
    }

    fn descriptor() -> EntryPointDescriptor {
        use crate::zur::contracts::abi::{
            ErrorTransferContract,
            LifetimeScope,
        };

        let parameter = crate::zur::contracts::abi::AbiParameter::new(
            symbol("input"),
            ValueContract::new(
                type_id("test.core.bytes"),
                PassingMode::ByValue,
                OwnershipMode::CallerRetains,
                LifetimeScope::Invocation,
            )
            .expect("valid parameter contract"),
        );

        let result = ValueContract::new(
            type_id("test.core.bytes"),
            PassingMode::ByValue,
            OwnershipMode::CallerRetains,
            LifetimeScope::Invocation,
        )
        .expect("valid result contract");

        let errors = ErrorTransferContract::new(
            type_id("test.runtime.error"),
            OwnershipMode::TransferredToReceiver,
            LifetimeScope::Invocation,
        )
        .expect("valid error contract");

        EntryPointDescriptor::new(
            symbol("test_foreign_function"),
            CallingConvention::new("test.native.system")
                .expect("valid calling convention"),
            InvocationMode::Synchronous,
            vec![parameter],
            Some(result),
            errors,
        )
        .expect("valid entry point")
    }

    fn request() -> ForeignCallRequest {
        ForeignCallRequest::new(
            descriptor(),
            vec![
                ForeignValue::encoded(
                    type_id("test.core.bytes"),
                    vec![1, 2, 3],
                ),
            ],
        )
        .expect("valid foreign request")
    }

    struct AllowAuthorizer;

    impl ForeignCallAuthorizer for AllowAuthorizer {
        fn authorize(
            &self,
            _context: &OperationContext,
            _entry_point: &EntryPointDescriptor,
        ) -> ZurResult<()> {
            Ok(())
        }
    }

    struct DenyAuthorizer;

    impl ForeignCallAuthorizer for DenyAuthorizer {
        fn authorize(
            &self,
            _context: &OperationContext,
            _entry_point: &EntryPointDescriptor,
        ) -> ZurResult<()> {
            Err(ZurError::permission_denied(
                "test authorization denied",
            ))
        }
    }

    struct TestExecutor {
        calls: Arc<AtomicUsize>,
        supported: bool,
        output: ForeignCallOutput,
    }

    impl ForeignCallExecutor for TestExecutor {
        fn provider_identity(&self) -> &str {
            "test.provider"
        }

        fn supports(
            &self,
            _convention: &CallingConvention,
            _mode: InvocationMode,
        ) -> bool {
            self.supported
        }

        fn invoke<'a>(
            &'a self,
            _context: &'a OperationContext,
            _request: &'a mut ForeignCallRequest,
        ) -> ForeignCallFuture<'a> {
            self.calls.fetch_add(
                1,
                AtomicOrdering::SeqCst,
            );

            let output = self.output.clone();

            Box::pin(async move {
                Ok(ForeignCallResponse::new(output))
            })
        }
    }

    fn service(
        authorizer: Arc<dyn ForeignCallAuthorizer>,
        supported: bool,
    ) -> (
        ForeignCallService,
        Arc<AtomicUsize>,
    ) {
        let calls = Arc::new(AtomicUsize::new(0));

        let executor = Arc::new(TestExecutor {
            calls: Arc::clone(&calls),
            supported,
            output: ForeignCallOutput::Value(
                ForeignValue::encoded(
                    type_id("test.core.bytes"),
                    vec![4, 5, 6],
                ),
            ),
        });

        let service = ForeignCallService::new(
            authorizer,
            executor,
        )
        .expect("valid foreign-call service");

        (service, calls)
    }

    #[test]
    fn accepts_valid_foreign_arguments() {
        let request = request();

        assert_eq!(request.argument_count(), 1);

        assert_eq!(
            request
                .argument(0)
                .expect("argument exists")
                .encoded_bytes(),
            Some(&[1, 2, 3][..]),
        );
    }

    #[test]
    fn rejects_incorrect_argument_count() {
        let result = ForeignCallRequest::new(
            descriptor(),
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_argument_type_mismatch() {
        let result = ForeignCallRequest::new(
            descriptor(),
            vec![
                ForeignValue::encoded(
                    type_id("test.other.bytes"),
                    vec![1],
                ),
            ],
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_incompatible_payload_representation() {
        let result = ForeignCallRequest::new(
            descriptor(),
            vec![
                ForeignValue::handle(
                    type_id("test.core.bytes"),
                    ForeignHandle::new("external.object")
                        .expect("valid handle"),
                ),
            ],
        );

        assert!(result.is_err());
    }

    #[test]
    fn accepts_opaque_external_handles() {
        let handle = ForeignHandle::new(
            "external.provider.object",
        )
        .expect("valid external handle");

        assert_eq!(
            handle.as_str(),
            "external.provider.object",
        );
    }

    #[test]
    fn rejects_invalid_external_handles() {
        assert!(ForeignHandle::new("").is_err());

        assert!(ForeignHandle::new(" invalid").is_err());

        assert!(
            ForeignHandle::new("invalid\nhandle").is_err()
        );
    }

    #[test]
    fn authorization_precedes_execution() {
        let (service, calls) = service(
            Arc::new(DenyAuthorizer),
            true,
        );

        let context = OperationContext::builder().build();

        let result = poll_ready(
            service.invoke(&context, request()),
        );

        assert!(result.is_err());

        assert_eq!(
            calls.load(AtomicOrdering::SeqCst),
            0,
        );
    }

    #[test]
    fn supported_foreign_call_returns_validated_result() {
        let (service, calls) = service(
            Arc::new(AllowAuthorizer),
            true,
        );

        let context = OperationContext::builder().build();

        let result = poll_ready(
            service.invoke(&context, request()),
        )
        .expect("foreign call succeeds");

        assert_eq!(
            result.output(),
            &ForeignCallOutput::Value(
                ForeignValue::encoded(
                    type_id("test.core.bytes"),
                    vec![4, 5, 6],
                ),
            ),
        );

        assert_eq!(
            calls.load(AtomicOrdering::SeqCst),
            1,
        );
    }

    #[test]
    fn unsupported_calling_convention_prevents_execution() {
        let (service, calls) = service(
            Arc::new(AllowAuthorizer),
            false,
        );

        let context = OperationContext::builder().build();

        let result = poll_ready(
            service.invoke(&context, request()),
        );

        assert!(result.is_err());

        assert_eq!(
            calls.load(AtomicOrdering::SeqCst),
            0,
        );
    }

    #[test]
    fn rejects_invalid_provider_identity() {
        struct InvalidExecutor;

        impl ForeignCallExecutor for InvalidExecutor {
            fn provider_identity(&self) -> &str {
                ""
            }

            fn supports(
                &self,
                _convention: &CallingConvention,
                _mode: InvocationMode,
            ) -> bool {
                false
            }

            fn invoke<'a>(
                &'a self,
                _context: &'a OperationContext,
                _request: &'a mut ForeignCallRequest,
            ) -> ForeignCallFuture<'a> {
                Box::pin(async {
                    Err(ZurError::unsupported(
                        "executor must not run",
                    ))
                })
            }
        }

        let result = ForeignCallService::new(
            Arc::new(AllowAuthorizer),
            Arc::new(InvalidExecutor),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_executor_result_type() {
        let result = validate_response(
            &descriptor(),
            &ForeignCallResponse::new(
                ForeignCallOutput::Value(
                    ForeignValue::encoded(
                        type_id("test.incompatible.type"),
                        vec![1],
                    ),
                ),
            ),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_unit_for_non_unit_entry_point() {
        let result = validate_response(
            &descriptor(),
            &ForeignCallResponse::new(
                ForeignCallOutput::Unit,
            ),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_wrong_invocation_output_category() {
        let result = validate_response(
            &descriptor(),
            &ForeignCallResponse::new(
                ForeignCallOutput::Stream(
                    ForeignValue::handle(
                        type_id("test.core.bytes"),
                        ForeignHandle::new("test.stream")
                            .expect("valid handle"),
                    ),
                ),
            ),
        );

        assert!(result.is_err());
    }

    #[test]
    fn preserves_declared_argument_order() {
        let request = request();

        let values: Vec<_> = request.arguments().collect();

        assert_eq!(values.len(), 1);

        assert_eq!(
            values[0]
                .expect("argument exists")
                .encoded_bytes(),
            Some(&[1, 2, 3][..]),
        );
    }
}
