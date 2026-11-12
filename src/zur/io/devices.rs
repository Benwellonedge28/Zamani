
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Device I/O
//!
//! Path: src/zur/io/devices.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//!
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the ZUR-facing integration boundary for portable
//! device discovery, opening and communication.
//!
//! This module integrates the existing standard-library device
//! implementation with ZUR's asynchronous execution and structured
//! error infrastructure.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - ZUR device-I/O composition.
//! - Explicit device-provider injection.
//! - Asynchronous device discovery.
//! - Asynchronous device opening.
//! - Standard-library to ZUR error adaptation.
//! - ZUR-facing device connection operations.
//!
//! This module does NOT own:
//!
//! - Device identity definitions.
//! - Device descriptor definitions.
//! - Device transport implementations.
//! - Device provider registration.
//! - Hardware discovery algorithms.
//! - Hardware allocation.
//! - Hardware scheduling.
//! - Device drivers.
//! - Hardware topology.
//! - Operating-system device enumeration.
//! - Device authentication.
//! - Device authorization.
//! - Device-specific protocols.
//! - Quantum execution.
//! - HDL synthesis or placement.
//! - Distributed transport.
//! - Runtime scheduling.
//! - Compiler lowering.
//! - Source-language semantics.
//!
//! These responsibilities remain with their canonical
//! repository authorities.
//!
//! # Canonical authorities
//!
//! Standard-library device API:
//!     crate::stdlib::io::devices
//!
//! Provider identity and lifecycle:
//!     crate::stdlib::provider
//!
//! Operation context and cancellation:
//!     crate::stdlib::context
//!
//! ZUR asynchronous provider future:
//!     crate::zur::contracts::provider
//!
//! ZUR operational errors:
//!     crate::zur::error
//!
//! Host I/O:
//!     crate::zur::io::host
//!
//! Byte streams:
//!     crate::zur::io::streams
//!
//! Platform integration:
//!     crate::zur::platforms
//!
//! Quantum hardware:
//!     crate::quantum::hardware
//!
//! # Scalability
//!
//! This module introduces no arbitrary maximum for:
//!
//! - Providers.
//! - Devices.
//! - Device identifiers.
//! - Device capabilities.
//! - Device kinds.
//! - Concurrent connections.
//! - Transfer sizes.
//! - Computational domains.
//! - Device operations.
//!
//! Actual limits are determined by available memory,
//! operating-system facilities, provider capabilities,
//! address-space representation and explicit operational policies.
//!
//! Literal infinite physical resources are not promised.
//!
//! # Safety
//!
//! No unsafe Rust.
//! No raw pointers.
//! No global mutable state.
//! No implicit provider registration.
//! No implicit hardware discovery.
//! No implicit hardware allocation.
//! No fabricated device availability.
//! No fabricated operation success.
//!
//! # Grammar integration
//!
//! Relevant grammar authorities:
//!
//! - grammar/effects/io.g4
//! - grammar/hybrid/host-device.g4
//! - grammar/execution/runtime.g4
//! - grammar/execution/runtime-capabilities.g4
//!
//! These grammars describe source-level intent.
//!
//! Semantic analysis validates the resulting representations.
//!
//! Compiler planning and realization establish compatible execution
//! requests.
//!
//! ZUR receives those requests through its execution and provider
//! integration boundaries.
//!
//! This module does not parse grammar or reinterpret AST nodes.
//!
//! # Integration
//!
//! Upstream:
//!
//! - Standard-library device contracts.
//! - Explicitly supplied device providers.
//! - Operation contexts.
//!
//! Downstream:
//!
//! - ZUR execution.
//! - ZUR I/O streams.
//! - ZUR platform integration.
//! - Hybrid execution adapters.
//! - Appropriate domain-specific hardware providers.

use std::error::Error as StdError;
use std::sync::Arc;

use crate::stdlib::context::OperationContext;

use crate::stdlib::io::devices::{
    DeviceAccess,
    DeviceConnection,
    DeviceDescriptor,
    DeviceId,
    DeviceKindId,
    DeviceCapabilityId,
    DeviceProvider,
    DeviceTransport,
};

use crate::stdlib::provider::{
    Provider,
    ProviderId,
};

use crate::zur::contracts::provider::ZurProviderFuture;

use crate::zur::error::{
    ErrorCode,
    ErrorKind,
    ZurError,
    ZurResult,
};

// ============================================================================
// COMPONENT IDENTIFICATION
// ============================================================================

/// Stable ZUR device-I/O component identifier.
pub const COMPONENT: &str = "zur.io.devices";

/// Current ZUR device-I/O integration API version.
pub const API_VERSION: &str = "1.0.0";

/// Device discovery operation identifier.
pub const DISCOVER_OPERATION: &str = "io.devices.discover";

/// Device opening operation identifier.
pub const OPEN_OPERATION: &str = "io.devices.open";

/// Device reading operation identifier.
pub const READ_OPERATION: &str = "io.devices.read";

/// Device writing operation identifier.
pub const WRITE_OPERATION: &str = "io.devices.write";

/// Device flushing operation identifier.
pub const FLUSH_OPERATION: &str = "io.devices.flush";

/// Device closing operation identifier.
pub const CLOSE_OPERATION: &str = "io.devices.close";

// ============================================================================
// CANONICAL PUBLIC TYPES
// ============================================================================

/// Canonical provider-scoped device identity.
///
/// Re-exported from the standard library. ZUR does not create
/// another device identity authority.
pub use crate::stdlib::io::devices::DeviceId as ZurDeviceId;

/// Canonical open-world device-kind identity.
pub use crate::stdlib::io::devices::DeviceKindId as ZurDeviceKindId;

/// Canonical open-world device capability identity.
pub use crate::stdlib::io::devices::DeviceCapabilityId as ZurDeviceCapabilityId;

/// Canonical provider-neutral device descriptor.
pub use crate::stdlib::io::devices::DeviceDescriptor as ZurDeviceDescriptor;

/// Canonical device access mode.
pub use crate::stdlib::io::devices::DeviceAccess as ZurDeviceAccess;

/// Canonical provider device transport.
pub use crate::stdlib::io::devices::DeviceTransport as ZurDeviceTransport;

/// Canonical device provider contract.
pub use crate::stdlib::io::devices::DeviceProvider as ZurDeviceProvider;

/// Canonical provider identity.
pub use crate::stdlib::provider::ProviderId as ZurDeviceProviderId;

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

/// Converts a standard-library device error into a structured ZUR error.
///
/// The original error is preserved as the causal source.
///
/// This adapter does not discard provider failure information.
///
/// # Arguments
///
/// * `source` - Original standard-library error.
/// * `operation` - ZUR operation that encountered the error.
///
/// # Returns
///
/// A structured ZUR error.
fn adapt_device_error(
    source: crate::stdlib::error::Error,
    operation: &'static str,
) -> ZurError {
    let message = source.to_string();

    let code = ErrorCode::new(
        "zur.io.devices.provider_failure",
    )
    .expect("reviewed static error code must remain valid");

    ZurError::new(
        ErrorKind::Provider,
        code,
        message,
    )
    .with_component(COMPONENT)
    .with_operation(operation)
    .with_source(source)
}

/// Checks cooperative cancellation before beginning a device operation.
///
/// The original standard-library cancellation error is preserved.
///
/// Closing a device is deliberately excluded from this check because
/// cleanup must remain possible after cancellation.
fn check_cancelled(
    context: &OperationContext,
    operation: &'static str,
) -> ZurResult<()> {
    context.check_cancelled().map_err(|source| {
        ZurError::cancelled(
            "device operation cancelled before execution",
        )
        .with_component(COMPONENT)
        .with_operation(operation)
        .with_source(source)
    })
}

/// Creates an error when a provider violates a device contract.
fn provider_contract_error(
    message: impl Into<String>,
    operation: &'static str,
) -> ZurError {
    let code = ErrorCode::new(
        "zur.io.devices.contract_violation",
    )
    .expect("reviewed static error code must remain valid");

    ZurError::new(
        ErrorKind::ContractViolation,
        code,
        message,
    )
    .with_component(COMPONENT)
    .with_operation(operation)
}

// ============================================================================
// DEVICE I/O SERVICE
// ============================================================================

/// Explicitly constructed ZUR device-I/O service.
///
/// This service integrates one independently supplied device provider
/// with the ZUR execution environment.
///
/// It does not register providers, discover providers, initialize
/// providers or select physical hardware.
///
/// Provider lifecycle remains with the existing provider lifecycle
/// authority.
///
/// A ZUR application can construct multiple independent DeviceIo
/// instances using different providers and operation contexts.
///
/// # Thread safety
///
/// The provider implements the canonical Provider contract, which
/// requires Send and Sync.
///
/// The provider instance is shared through Arc.
///
/// # Example
///
/// ```
/// use std::sync::Arc;
///
/// use zamani_compiler::stdlib::context::OperationContext;
///
/// use zamani_compiler::zur::io::devices::DeviceIo;
///
/// // A concrete DeviceProvider is supplied by the application.
/// // Provider registration and initialization remain explicit.
/// ```
#[derive(Clone)]
pub struct DeviceIo<P>
where
    P: DeviceProvider + ?Sized,
{
    provider: Arc<P>,
    context: OperationContext,
}

impl<P> DeviceIo<P>
where
    P: DeviceProvider + ?Sized,
{
    /// Constructs a device-I/O service with an explicit provider.
    ///
    /// A default, non-cancelled operation context is used.
    ///
    /// This constructor performs no device discovery or provider
    /// initialization.
    #[must_use]
    pub fn new(
        provider: Arc<P>,
    ) -> Self {
        Self::with_context(
            provider,
            OperationContext::builder().build(),
        )
    }

    /// Constructs a device-I/O service with an explicit provider
    /// and operation context.
    ///
    /// The context is shared with discovery and opening operations.
    ///
    /// This constructor does not create another cancellation
    /// mechanism.
    #[must_use]
    pub fn with_context(
        provider: Arc<P>,
        context: OperationContext,
    ) -> Self {
        Self {
            provider,
            context,
        }
    }

    /// Returns the canonical provider identity.
    ///
    /// The identity is descriptive and does not establish provider
    /// authentication, authorization or availability.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        self.provider.descriptor().id()
    }

    /// Returns the immutable operation context.
    #[must_use]
    pub fn context(&self) -> &OperationContext {
        &self.context
    }

    /// Returns the explicitly supplied provider.
    ///
    /// This does not expose a mutable provider reference.
    #[must_use]
    pub fn provider(&self) -> &P {
        &self.provider
    }

    /// Discovers devices through the supplied provider.
    ///
    /// The canonical standard-library DeviceProvider implementation
    /// performs descriptor ownership validation and duplicate
    /// identity detection.
    ///
    /// Results are provider-neutral descriptions, not resource
    /// reservations or guarantees of future availability.
    ///
    /// # Cancellation
    ///
    /// Cancellation is checked before discovery.
    ///
    /// The provider is responsible for checking cancellation during
    /// lengthy discovery operations.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - cancellation has been requested;
    /// - provider discovery fails;
    /// - a provider violates its discovery contract.
    pub fn discover(
        &self,
    ) -> ZurProviderFuture<'_, Vec<DeviceDescriptor>> {
        Box::pin(async move {
            check_cancelled(
                &self.context,
                DISCOVER_OPERATION,
            )?;

            self.provider
                .discover_devices(&self.context)
                .await
                .map_err(|source| {
                    adapt_device_error(
                        source,
                        DISCOVER_OPERATION,
                    )
                })
        })
    }

    /// Opens a previously discovered device.
    ///
    /// The provider must revalidate device availability when opening.
    ///
    /// A discovery result is not a reservation.
    ///
    /// Access permissions are enforced by the canonical standard-library
    /// device connection.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - Provider-owned device description.
    /// * `access` - Requested device access mode.
    ///
    /// # Returns
    ///
    /// An exclusively owned ZUR device connection.
    ///
    /// # Errors
    ///
    /// Returns an error if the device is unavailable, incompatible,
    /// unauthorized, belongs to another provider or cannot be opened.
    pub fn open<'a>(
        &'a self,
        descriptor: &'a DeviceDescriptor,
        access: DeviceAccess,
    ) -> ZurProviderFuture<'a, ZurDeviceConnection> {
        Box::pin(async move {
            check_cancelled(
                &self.context,
                OPEN_OPERATION,
            )?;

            let connection = self.provider
                .open(
                    descriptor,
                    access,
                    &self.context,
                )
                .await
                .map_err(|source| {
                    adapt_device_error(
                        source,
                        OPEN_OPERATION,
                    )
                })?;

            Ok(ZurDeviceConnection {
                inner: connection,
            })
        })
    }
}

// ============================================================================
// DEVICE CONNECTION
// ============================================================================

/// ZUR-facing device connection.
///
/// This is an adapter around the canonical standard-library
/// DeviceConnection.
///
/// It does not create another transport implementation.
///
/// The connection exclusively owns its underlying device session.
///
/// # Concurrency
///
/// A single connection must not be used concurrently through
/// multiple mutable references.
///
/// Independent connections may be created through the appropriate
/// providers and runtime resource-management authorities.
///
/// # Cancellation
///
/// Read and write operations observe the operation context supplied
/// when the connection was opened.
///
/// Closing remains possible after cancellation.
pub struct ZurDeviceConnection {
    inner: DeviceConnection,
}

impl ZurDeviceConnection {
    /// Returns the canonical device descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &DeviceDescriptor {
        self.inner.descriptor()
    }

    /// Returns the access mode used when opening the connection.
    #[must_use]
    pub const fn access(&self) -> DeviceAccess {
        self.inner.access()
    }

    /// Returns whether the connection has been successfully closed.
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    /// Reads bytes from the device.
    ///
    /// The returned count represents the actual bytes received.
    ///
    /// Partial reads are permitted.
    ///
    /// A zero result for a non-empty buffer indicates end-of-stream.
    ///
    /// An empty buffer returns zero without contacting the provider.
    ///
    /// # Errors
    ///
    /// Returns a structured ZUR error if the provider fails,
    /// access is denied, cancellation occurs or the provider
    /// violates its transport contract.
    pub fn read(
        &mut self,
        destination: &mut [u8],
    ) -> ZurResult<usize> {
        self.inner
            .read(destination)
            .map_err(|source| {
                adapt_device_error(
                    source,
                    READ_OPERATION,
                )
            })
    }

    /// Writes bytes to the device.
    ///
    /// Partial writes are permitted.
    ///
    /// The returned count represents the actual bytes accepted.
    ///
    /// A zero result with non-empty input means that the provider
    /// made no progress.
    ///
    /// # Errors
    ///
    /// Returns a structured ZUR error when writing fails.
    pub fn write(
        &mut self,
        source: &[u8],
    ) -> ZurResult<usize> {
        self.inner
            .write(source)
            .map_err(|source| {
                adapt_device_error(
                    source,
                    WRITE_OPERATION,
                )
            })
    }

    /// Writes the complete supplied buffer.
    ///
    /// Handles partial writes incrementally.
    ///
    /// The entire output is never required to be copied into
    /// another allocation.
    ///
    /// A provider that makes no progress while data remains
    /// produces an error.
    ///
    /// # Errors
    ///
    /// Returns a structured ZUR error on cancellation,
    /// provider failure or lack of progress.
    pub fn write_all(
        &mut self,
        source: &[u8],
    ) -> ZurResult<()> {
        self.inner
            .write_all(source)
            .map_err(|source| {
                adapt_device_error(
                    source,
                    WRITE_OPERATION,
                )
            })
    }

    /// Flushes pending output.
    ///
    /// A successful flush does not automatically guarantee
    /// durable physical persistence.
    ///
    /// Flushing remains available after cancellation.
    ///
    /// # Errors
    ///
    /// Returns a structured ZUR error if flushing fails.
    pub fn flush(&mut self) -> ZurResult<()> {
        self.inner
            .flush()
            .map_err(|source| {
                adapt_device_error(
                    source,
                    FLUSH_OPERATION,
                )
            })
    }

    /// Closes the underlying device connection.
    ///
    /// Closing is permitted after cancellation.
    ///
    /// A successful repeated close is harmless.
    ///
    /// If the underlying provider reports failure, the connection
    /// remains available for an explicit retry.
    ///
    /// # Errors
    ///
    /// Returns a structured ZUR error if closure fails.
    pub fn close(&mut self) -> ZurResult<()> {
        self.inner
            .close()
            .map_err(|source| {
                adapt_device_error(
                    source,
                    CLOSE_OPERATION,
                )
            })
    }

    /// Flushes pending output and closes the connection.
    ///
    /// This consumes the connection.
    ///
    /// It does not claim physical durability unless the
    /// underlying provider explicitly guarantees it.
    ///
    /// # Errors
    ///
    /// Returns a structured ZUR error if flushing or closing fails.
    pub fn finish(self) -> ZurResult<()> {
        self.inner
            .finish()
            .map_err(|source| {
                adapt_device_error(
                    source,
                    CLOSE_OPERATION,
                )
            })
    }
}

// ============================================================================
// PUBLIC TRANSPORT INTEGRATION
// ============================================================================

/// Returns the underlying canonical device transport contract.
///
/// This trait is re-exported for provider implementations.
/// ZUR does not supply a fake transport.
pub fn device_transport_contract_is_send<T>(
    _transport: &T,
) -> bool
where
    T: DeviceTransport,
{
    true
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_identity_is_stable() {
        assert_eq!(
            COMPONENT,
            "zur.io.devices",
        );
    }

    #[test]
    fn api_version_is_available() {
        assert!(!API_VERSION.is_empty());
    }

    #[test]
    fn operation_identifiers_are_distinct() {
        let operations = [
            DISCOVER_OPERATION,
            OPEN_OPERATION,
            READ_OPERATION,
            WRITE_OPERATION,
            FLUSH_OPERATION,
            CLOSE_OPERATION,
        ];

        let unique: std::collections::BTreeSet<_> =
            operations.iter().copied().collect();

        assert_eq!(
            operations.len(),
            unique.len(),
        );
    }

    #[test]
    fn device_error_preserves_operation_context() {
        let source = crate::stdlib::error::Error::new(
            crate::stdlib::error::ErrorKind::Io,
            "test device failure",
        );

        let error = adapt_device_error(
            source,
            READ_OPERATION,
        );

        assert_eq!(
            error.component(),
            Some(COMPONENT),
        );

        assert_eq!(
            error.operation(),
            Some(READ_OPERATION),
        );

        assert_eq!(
            error.kind(),
            ErrorKind::Provider,
        );

        assert!(
            error.source_error().is_some(),
        );
    }

    #[test]
    fn provider_contract_error_is_not_success() {
        let error = provider_contract_error(
            "invalid device response",
            READ_OPERATION,
        );

        assert_eq!(
            error.kind(),
            ErrorKind::ContractViolation,
        );

        assert_eq!(
            error.operation(),
            Some(READ_OPERATION),
        );
    }
}
