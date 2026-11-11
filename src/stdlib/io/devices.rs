
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Portable Device I/O
//!
//! Path: `src/stdlib/io/devices.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines portable, provider-neutral contracts for discovering,
//! describing, opening, reading from, and writing to byte-oriented
//! devices.
//!
//! A device may be supplied by:
//!
//! - an operating-system device provider;
//! - an embedded-system provider;
//! - a network-connected device provider;
//! - a simulator;
//! - a hardware abstraction provider;
//! - a virtual device provider;
//! - a future computational-domain provider.
//!
//! The standard library does not assume that a device is local,
//! physical, persistent, synchronous at the hardware boundary,
//! or attached to a particular operating system.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - portable device identity types;
//! - device-kind and capability identifiers;
//! - device access modes;
//! - validated device descriptors;
//! - device-provider extension contracts;
//! - byte-oriented device transport contracts;
//! - cancellation-aware device connections;
//! - device connection lifecycle semantics.
//!
//! This module does not own:
//!
//! - physical hardware discovery algorithms;
//! - operating-system device enumeration;
//! - device allocation;
//! - resource scheduling;
//! - hardware topology;
//! - physical memory management;
//! - quantum execution;
//! - HDL synthesis or placement;
//! - sensor-specific protocols;
//! - actuator-specific protocols;
//! - authentication or authorization;
//! - provider registration;
//! - runtime task scheduling;
//! - compiler lowering;
//! - hardware control policy.
//!
//! These responsibilities remain with their canonical repository owners.
//!
//! ## Existing repository authorities
//!
//! - `crate::stdlib::error`:
//!   standard-library errors.
//!
//! - `crate::stdlib::context`:
//!   operation context and cooperative cancellation.
//!
//! - `crate::stdlib::provider`:
//!   common provider identity, metadata and asynchronous provider contract.
//!
//! - `crate::stdlib::io::streams`:
//!   generic byte-stream abstractions.
//!
//! - `crate::quantum::hardware`:
//!   quantum hardware discovery, providers and execution.
//!
//! - `crate::hdl`:
//!   hardware-description and logical hardware interfaces.
//!
//! - `crate::runtime`:
//!   execution, resource acquisition and scheduling.
//!
//! ## Device identity
//!
//! Device identifiers are provider-scoped.
//!
//! A device identity is not a globally meaningful physical address.
//! It is an opaque identifier interpreted by its owning provider.
//!
//! Device identifiers are open-world strings. New providers and device
//! classes do not require modifications to this module.
//!
//! ## Provider boundary
//!
//! Device providers extend the existing standard-library `Provider`
//! contract.
//!
//! The generic provider module owns provider identity and lifecycle.
//! This module adds device-specific discovery and transport operations.
//!
//! The provider must validate that a device is still available when
//! opening it. A discovery result is an observation, not a reservation.
//!
//! ## Device transport
//!
//! This module provides byte-oriented device transport.
//!
//! Device-specific event models, command protocols, quantum workloads,
//! sensor values, actuator commands and HDL interfaces belong to their
//! specialized domain APIs.
//!
//! A provider may adapt those systems to this byte-oriented interface
//! when byte transport is appropriate.
//!
//! ## Cancellation
//!
//! Cancellation is cooperative.
//!
//! The connection checks its operation context before reading or writing.
//!
//! Cancellation cannot forcibly interrupt an underlying blocking
//! operating-system or provider operation.
//!
//! Closing a connection is not blocked by cancellation because cleanup
//! must remain possible after a cancellation request.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - devices;
//! - providers;
//! - device identifiers;
//! - capability identifiers;
//! - metadata entries;
//! - device kinds;
//! - input bytes;
//! - output bytes;
//! - concurrent device connections;
//! - computational domains.
//!
//! Collections grow according to available memory and platform limits.
//!
//! Operational budgets belong to explicit runtime or provider policy.
//! They must not become language-level device-count or hardware limits.
//!
//! This module does not promise literally infinite physical resources.
//!
//! ## Security
//!
//! Device descriptors contain descriptive metadata, not credentials.
//!
//! Providers must not put passwords, API keys, private keys, bearer
//! tokens, authorization headers or other secrets in device metadata.
//!
//! Device discovery does not authorize device access.
//!
//! Provider registration does not establish provider trust.
//!
//! Authorization and hardware admission remain the responsibility of
//! their respective security and runtime authorities.
//!
//! ## Error guarantees
//!
//! Invalid input is reported as a structured error.
//!
//! Unsupported operations are reported explicitly.
//!
//! Provider failures are propagated.
//!
//! A failed operation is never represented as successful I/O.
//!
//! The module does not fabricate device discovery results.
//!
//! ## Freeze criteria
//!
//! This module is ready to freeze when:
//!
//! 1. `io.rs` declares this module.
//! 2. The standard-library error, context and provider modules are exposed.
//! 3. Device providers implement the device-specific extension contract.
//! 4. Runtime/provider adapters manage actual device access.
//! 5. Device access modes are enforced.
//! 6. Cancellation behavior is tested.
//! 7. Invalid and duplicate discovery results are rejected.
//! 8. Connection closure and transport errors are tested.
//! 9. No arbitrary hardware ceilings are introduced.
//! 10. Formatting, compilation, tests and Clippy pass.

use core::fmt;

use std::collections::{BTreeMap, BTreeSet};

use crate::stdlib::context::OperationContext;

use crate::stdlib::error::{
    Error,
    ErrorKind,
    Result,
};

use crate::stdlib::provider::{
    Provider,
    ProviderFuture,
    ProviderId,
};

// ============================================================================
// COMPONENT IDENTIFIERS
// ============================================================================

/// Stable standard-library component identifier.
pub const COMPONENT: &str = "stdlib.io.devices";

/// Stable device-discovery operation identifier.
pub const DISCOVER_OPERATION: &str = "io.devices.discover";

/// Stable device-opening operation identifier.
pub const OPEN_OPERATION: &str = "io.devices.open";

/// Stable device-read operation identifier.
pub const READ_OPERATION: &str = "io.devices.read";

/// Stable device-write operation identifier.
pub const WRITE_OPERATION: &str = "io.devices.write";

/// Stable device-flush operation identifier.
pub const FLUSH_OPERATION: &str = "io.devices.flush";

/// Stable device-close operation identifier.
pub const CLOSE_OPERATION: &str = "io.devices.close";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn device_error(
    kind: ErrorKind,
    message: impl Into<String>,
    operation: &'static str,
) -> Error {
    Error::new(kind, message)
        .in_component(COMPONENT)
        .during_operation(operation)
}

fn invalid_identifier(
    kind: &'static str,
    value: &str,
) -> Error {
    device_error(
        ErrorKind::InvalidInput,
        format!("{kind} must be non-empty and contain no whitespace or control characters"),
        DISCOVER_OPERATION,
    )
    .with_context("identifier", value)
    .unwrap_or_else(|_| {
        device_error(
            ErrorKind::InvalidInput,
            format!("invalid {kind}"),
            DISCOVER_OPERATION,
        )
    })
}

fn validate_identifier(
    kind: &'static str,
    value: &str,
) -> Result<()> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(|character| {
            character.is_control() || character.is_whitespace()
        })
    {
        return Err(invalid_identifier(kind, value));
    }

    Ok(())
}

fn validate_display_name(
    value: &str,
) -> Result<()> {
    if value.trim().is_empty()
        || value.chars().any(char::is_control)
    {
        return Err(device_error(
            ErrorKind::InvalidInput,
            "device display name must be non-empty and contain no control characters",
            DISCOVER_OPERATION,
        ));
    }

    Ok(())
}

// ============================================================================
// DEVICE IDENTIFIERS
// ============================================================================

/// Opaque, provider-scoped device identity.
///
/// The identifier is interpreted by the provider that created it.
///
/// It is not a physical address, operating-system path, global device
/// index, or hardware allocation request.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId(String);

impl DeviceId {
    /// Creates a validated device identifier.
    ///
    /// Identifiers are open-world strings. No arbitrary maximum length
    /// is imposed.
    ///
    /// # Errors
    ///
    /// Returns an error when the identifier is empty, contains
    /// whitespace, or contains control characters.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self> {
        let value = value.into();

        validate_identifier("device identifier", &value)?;

        Ok(Self(value))
    }

    /// Returns the exact provider-defined identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DeviceId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Open-world device-kind identifier.
///
/// Examples include `sensor`, `actuator`, `serial`, `camera`,
/// `audio.input`, `audio.output`, and provider-defined kinds.
///
/// This is deliberately not a closed enum.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceKindId(String);

impl DeviceKindId {
    /// Creates a validated device-kind identifier.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty identifier, whitespace,
    /// or control characters.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self> {
        let value = value.into();

        validate_identifier("device-kind identifier", &value)?;

        Ok(Self(value))
    }

    /// Returns the exact device-kind identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DeviceKindId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Open-world device capability identifier.
///
/// Capabilities describe provider-declared functionality. They do not
/// prove that the functionality is currently available or authorized.
///
/// A current capability snapshot must be obtained from the appropriate
/// provider or runtime authority.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceCapabilityId(String);

impl DeviceCapabilityId {
    /// Creates a validated device capability identifier.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty identifier, whitespace,
    /// or control characters.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self> {
        let value = value.into();

        validate_identifier("device capability identifier", &value)?;

        Ok(Self(value))
    }

    /// Returns the exact capability identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DeviceCapabilityId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// DEVICE ACCESS
// ============================================================================

/// Describes the access requested when opening a device.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DeviceAccess {
    /// Open for input only.
    Read,

    /// Open for output only.
    Write,

    /// Open for both input and output.
    ReadWrite,
}

impl DeviceAccess {
    /// Returns whether this access mode permits reading.
    #[must_use]
    pub const fn permits_read(self) -> bool {
        matches!(
            self,
            Self::Read | Self::ReadWrite
        )
    }

    /// Returns whether this access mode permits writing.
    #[must_use]
    pub const fn permits_write(self) -> bool {
        matches!(
            self,
            Self::Write | Self::ReadWrite
        )
    }
}

// ============================================================================
// DEVICE DESCRIPTOR
// ============================================================================

/// Validated, provider-neutral device description.
///
/// A descriptor represents an observation of a device supplied by a
/// provider. It does not reserve, allocate, authenticate or open the
/// device.
///
/// The descriptor's identity is scoped to `provider_id`.
///
/// Capability identifiers and properties are descriptive metadata.
/// They are not a substitute for live capability evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceDescriptor {
    provider_id: ProviderId,
    id: DeviceId,
    kind: DeviceKindId,
    display_name: String,
    capabilities: BTreeSet<DeviceCapabilityId>,
    properties: BTreeMap<String, String>,
}

impl DeviceDescriptor {
    /// Constructs and validates a device descriptor.
    ///
    /// `properties` must contain descriptive, non-secret metadata.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid identifiers, an invalid display
    /// name, an empty property key, or a property key containing
    /// control characters.
    pub fn new(
        provider_id: ProviderId,
        id: DeviceId,
        kind: DeviceKindId,
        display_name: impl Into<String>,
        capabilities: impl IntoIterator<Item = DeviceCapabilityId>,
        properties: BTreeMap<String, String>,
    ) -> Result<Self> {
        let display_name = display_name.into();

        validate_display_name(&display_name)?;

        for key in properties.keys() {
            if key.trim().is_empty()
                || key.chars().any(char::is_control)
            {
                return Err(device_error(
                    ErrorKind::InvalidInput,
                    "device property keys must be non-empty and contain no control characters",
                    DISCOVER_OPERATION,
                ));
            }
        }

        Ok(Self {
            provider_id,
            id,
            kind,
            display_name,
            capabilities: capabilities.into_iter().collect(),
            properties,
        })
    }

    /// Returns the provider that owns this device identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the provider-scoped device identity.
    #[must_use]
    pub fn id(&self) -> &DeviceId {
        &self.id
    }

    /// Returns the open-world device-kind identifier.
    #[must_use]
    pub fn kind(&self) -> &DeviceKindId {
        &self.kind
    }

    /// Returns the human-readable device name.
    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    /// Returns the provider-declared capability identifiers.
    ///
    /// These declarations are descriptive, not live availability
    /// evidence.
    #[must_use]
    pub fn capabilities(&self) -> &BTreeSet<DeviceCapabilityId> {
        &self.capabilities
    }

    /// Returns descriptive, non-secret device properties.
    #[must_use]
    pub fn properties(&self) -> &BTreeMap<String, String> {
        &self.properties
    }
}

// ============================================================================
// DEVICE TRANSPORT
// ============================================================================

/// Provider-implemented byte-oriented device transport.
///
/// Implementations perform the actual device I/O.
///
/// They must not report successful transfers that did not occur.
///
/// `read` follows stream semantics: returning zero indicates that
/// the input stream has reached end-of-stream.
///
/// Devices with event-oriented or message-oriented semantics should
/// expose their specialized APIs rather than pretending that every
/// device is a byte stream.
///
/// The transport is mutable and `Send`. It is not required to be
/// `Sync`, because one connection owns its active I/O session.
pub trait DeviceTransport: Send {
    /// Reads bytes from the underlying device.
    ///
    /// Returns the number of bytes actually written into `buffer`.
    ///
    /// Returning zero indicates end-of-stream.
    ///
    /// # Errors
    ///
    /// Returns the provider's structured error when the device read
    /// fails.
    fn read(
        &mut self,
        buffer: &mut [u8],
    ) -> Result<usize>;

    /// Writes bytes to the underlying device.
    ///
    /// Returns the number of bytes actually accepted.
    ///
    /// Partial writes are permitted. Callers requiring complete
    /// transfer must continue writing the remaining bytes.
    ///
    /// # Errors
    ///
    /// Returns the provider's structured error when the device write
    /// fails.
    fn write(
        &mut self,
        buffer: &[u8],
    ) -> Result<usize>;

    /// Flushes pending output according to the provider's contract.
    ///
    /// A successful flush does not necessarily mean that a physical
    /// device has durably persisted the data.
    ///
    /// # Errors
    ///
    /// Returns an error if flushing fails.
    fn flush(&mut self) -> Result<()>;

    /// Closes the underlying device session.
    ///
    /// Implementations should make successful repeated closure
    /// harmless.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider cannot complete closure.
    fn close(&mut self) -> Result<()>;
}

// ============================================================================
// DEVICE PROVIDER
// ============================================================================

/// Device-specific extension of the canonical standard-library provider.
///
/// This trait does not replace [`Provider`].
///
/// It adds device discovery and opening operations while reusing the
/// common provider identity, metadata, status, capability and lifecycle
/// contracts.
///
/// The provider owns all physical discovery and device-specific behavior.
pub trait DeviceProvider: Provider {
    /// Performs provider-specific device discovery.
    ///
    /// The returned descriptors must all belong to this provider.
    ///
    /// Discovery must not reserve or allocate devices.
    ///
    /// # Errors
    ///
    /// Returns a structured provider error if discovery fails.
    fn list_devices<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, Vec<DeviceDescriptor>>;

    /// Opens a provider-specific device transport.
    ///
    /// The provider must revalidate that the device is still available.
    /// A previously returned descriptor is not a permanent reservation.
    ///
    /// The provider must enforce its own security and access policy.
    ///
    /// # Errors
    ///
    /// Returns an error if the device is unknown, unavailable,
    /// unauthorized, incompatible, or cannot be opened.
    fn open_transport<'a>(
        &'a self,
        descriptor: &'a DeviceDescriptor,
        access: DeviceAccess,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, Box<dyn DeviceTransport>>;

    /// Discovers and validates devices supplied by this provider.
    ///
    /// Results are sorted by device identity to make the returned
    /// snapshot deterministic for a fixed provider result.
    ///
    /// Duplicate device identities are rejected.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - the operation is cancelled;
    /// - the provider fails discovery;
    /// - a descriptor belongs to another provider;
    /// - duplicate device identities are returned.
    fn discover_devices<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, Vec<DeviceDescriptor>> {
        Box::pin(async move {
            context.check_cancelled()?;

            let mut devices = self.list_devices(context).await?;

            let expected_provider = self.descriptor().id();

            let mut identities = BTreeSet::new();

            for device in &devices {
                if device.provider_id() != expected_provider {
                    return Err(device_error(
                        ErrorKind::ContractViolation,
                        "provider returned a device descriptor owned by another provider",
                        DISCOVER_OPERATION,
                    ));
                }

                if !identities.insert(device.id().clone()) {
                    return Err(device_error(
                        ErrorKind::ContractViolation,
                        format!(
                            "provider returned duplicate device identity: {}",
                            device.id()
                        ),
                        DISCOVER_OPERATION,
                    ));
                }
            }

            devices.sort_by(|left, right| {
                left.id().cmp(right.id())
            });

            Ok(devices)
        })
    }

    /// Opens a device and returns a cancellation-aware connection.
    ///
    /// The descriptor must belong to this provider.
    ///
    /// The provider revalidates the device before opening it.
    ///
    /// # Errors
    ///
    /// Returns an error if the descriptor belongs to another provider,
    /// the operation is cancelled, or the provider cannot open the device.
    fn open<'a>(
        &'a self,
        descriptor: &'a DeviceDescriptor,
        access: DeviceAccess,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, DeviceConnection> {
        Box::pin(async move {
            context.check_cancelled()?;

            if descriptor.provider_id() != self.descriptor().id() {
                return Err(device_error(
                    ErrorKind::InvalidInput,
                    "device descriptor belongs to a different provider",
                    OPEN_OPERATION,
                ));
            }

            let transport = self
                .open_transport(
                    descriptor,
                    access,
                    context,
                )
                .await?;

            Ok(DeviceConnection {
                descriptor: descriptor.clone(),
                access,
                context: context.clone(),
                transport,
                closed: false,
            })
        })
    }
}

// ============================================================================
// DEVICE CONNECTION
// ============================================================================

/// An opened, provider-backed device connection.
///
/// The connection enforces the access mode requested when it was opened
/// and checks cooperative cancellation before input and output operations.
///
/// It does not select hardware, schedule work, or allocate resources.
/// Those responsibilities remain with the provider and runtime.
///
/// A connection is exclusively owned by its caller. The transport is
/// not shared concurrently through this API.
pub struct DeviceConnection {
    descriptor: DeviceDescriptor,
    access: DeviceAccess,
    context: OperationContext,
    transport: Box<dyn DeviceTransport>,
    closed: bool,
}

impl DeviceConnection {
    /// Returns the descriptor associated with this connection.
    #[must_use]
    pub fn descriptor(&self) -> &DeviceDescriptor {
        &self.descriptor
    }

    /// Returns the access mode used to open this connection.
    #[must_use]
    pub const fn access(&self) -> DeviceAccess {
        self.access
    }

    /// Returns whether the connection has been successfully closed.
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        self.closed
    }

    /// Reads bytes from the device.
    ///
    /// A zero-length buffer returns zero without contacting the provider.
    ///
    /// The returned count must not exceed the supplied buffer length.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - the connection is closed;
    /// - the connection does not permit reading;
    /// - cancellation has been requested;
    /// - the provider fails;
    /// - the provider violates the transfer contract.
    pub fn read(
        &mut self,
        buffer: &mut [u8],
    ) -> Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        self.ensure_open(READ_OPERATION)?;

        if !self.access.permits_read() {
            return Err(device_error(
                ErrorKind::PermissionDenied,
                "device connection was not opened for reading",
                READ_OPERATION,
            ));
        }

        self.context.check_cancelled()?;

        let amount = self.transport.read(buffer)?;

        if amount > buffer.len() {
            return Err(device_error(
                ErrorKind::ContractViolation,
                "device provider reported more bytes read than the supplied buffer can hold",
                READ_OPERATION,
            ));
        }

        Ok(amount)
    }

    /// Writes bytes to the device.
    ///
    /// The returned count represents bytes actually accepted by the
    /// underlying transport.
    ///
    /// Partial writes are permitted.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - the connection is closed;
    /// - the connection does not permit writing;
    /// - cancellation has been requested;
    /// - the provider fails;
    /// - the provider violates the transfer contract.
    pub fn write(
        &mut self,
        buffer: &[u8],
    ) -> Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        self.ensure_open(WRITE_OPERATION)?;

        if !self.access.permits_write() {
            return Err(device_error(
                ErrorKind::PermissionDenied,
                "device connection was not opened for writing",
                WRITE_OPERATION,
            ));
        }

        self.context.check_cancelled()?;

        let amount = self.transport.write(buffer)?;

        if amount > buffer.len() {
            return Err(device_error(
                ErrorKind::ContractViolation,
                "device provider reported more bytes written than the supplied buffer contains",
                WRITE_OPERATION,
            ));
        }

        Ok(amount)
    }

    /// Writes the entire supplied buffer.
    ///
    /// This method handles partial writes by continuing until all
    /// bytes have been accepted or an error occurs.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection is closed, writing is not
    /// permitted, cancellation occurs, the provider fails, or the
    /// provider makes no progress while bytes remain.
    pub fn write_all(
        &mut self,
        buffer: &[u8],
    ) -> Result<()> {
        let mut remaining = buffer;

        while !remaining.is_empty() {
            let amount = self.write(remaining)?;

            if amount == 0 {
                return Err(device_error(
                    ErrorKind::Io,
                    "device provider made no progress while bytes remained",
                    WRITE_OPERATION,
                ));
            }

            remaining = &remaining[amount..];
        }

        Ok(())
    }

    /// Flushes pending device output.
    ///
    /// Flushing is permitted only when the connection allows writing.
    ///
    /// Flush is deliberately not blocked by cancellation so that
    /// accepted output can be completed during cleanup.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection is closed, writing is not
    /// permitted, or the provider fails.
    pub fn flush(&mut self) -> Result<()> {
        self.ensure_open(FLUSH_OPERATION)?;

        if !self.access.permits_write() {
            return Err(device_error(
                ErrorKind::PermissionDenied,
                "device connection was not opened for writing",
                FLUSH_OPERATION,
            ));
        }

        self.transport.flush()
    }

    /// Closes the underlying device connection.
    ///
    /// Closing is not blocked by cancellation.
    ///
    /// A successful repeated call is harmless.
    ///
    /// If the provider reports a closure failure, the connection
    /// remains logically open so that the caller may retry or apply
    /// provider-specific recovery.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider cannot close the connection.
    pub fn close(&mut self) -> Result<()> {
        if self.closed {
            return Ok(());
        }

        self.transport.close()?;

        self.closed = true;

        Ok(())
    }

    /// Flushes pending output and closes the connection.
    ///
    /// This method consumes the connection.
    ///
    /// It does not claim durable persistence unless the provider's
    /// own contract explicitly guarantees it.
    ///
    /// # Errors
    ///
    /// Returns an error if flushing or closing fails.
    pub fn finish(
        mut self,
    ) -> Result<()> {
        if self.access.permits_write() {
            self.flush()?;
        }

        self.close()
    }

    fn ensure_open(
        &self,
        operation: &'static str,
    ) -> Result<()> {
        if self.closed {
            return Err(device_error(
                ErrorKind::InvalidInput,
                "device connection is already closed",
                operation,
            ));
        }

        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::provider::ProviderId;

    use std::collections::BTreeMap;

    fn provider_id() -> ProviderId {
        ProviderId::new("test.device.provider")
            .expect("valid provider identity")
    }

    fn descriptor() -> DeviceDescriptor {
        DeviceDescriptor::new(
            provider_id(),
            DeviceId::new("device/test-1")
                .expect("valid device identity"),
            DeviceKindId::new("test.byte-device")
                .expect("valid device kind"),
            "Test device",
            [
                DeviceCapabilityId::new("io.read")
                    .expect("valid capability"),
                DeviceCapabilityId::new("io.write")
                    .expect("valid capability"),
            ],
            BTreeMap::new(),
        )
        .expect("valid descriptor")
    }

    struct MemoryTransport {
        input: Vec<u8>,
        output: Vec<u8>,
        closed: bool,
    }

    impl DeviceTransport for MemoryTransport {
        fn read(
            &mut self,
            buffer: &mut [u8],
        ) -> Result<usize> {
            let amount = buffer.len().min(self.input.len());

            buffer[..amount].copy_from_slice(
                &self.input[..amount],
            );

            self.input.drain(..amount);

            Ok(amount)
        }

        fn write(
            &mut self,
            buffer: &[u8],
        ) -> Result<usize> {
            self.output.extend_from_slice(buffer);

            Ok(buffer.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }

        fn close(&mut self) -> Result<()> {
            self.closed = true;

            Ok(())
        }
    }

    fn connection(
        access: DeviceAccess,
    ) -> DeviceConnection {
        DeviceConnection {
            descriptor: descriptor(),
            access,
            context: OperationContext::builder().build(),
            transport: Box::new(MemoryTransport {
                input: b"Zamani".to_vec(),
                output: Vec::new(),
                closed: false,
            }),
            closed: false,
        }
    }

    #[test]
    fn identifiers_preserve_provider_defined_values() {
        let id = DeviceId::new("provider/namespace/device-1")
            .expect("valid identifier");

        assert_eq!(
            id.as_str(),
            "provider/namespace/device-1"
        );
    }

    #[test]
    fn identifiers_reject_empty_values() {
        assert!(
            DeviceId::new("").is_err()
        );
    }

    #[test]
    fn identifiers_reject_whitespace() {
        assert!(
            DeviceId::new("device one").is_err()
        );
    }

    #[test]
    fn descriptors_preserve_capabilities() {
        let value = descriptor();

        assert_eq!(
            value.capabilities().len(),
            2
        );

        assert_eq!(
            value.id().as_str(),
            "device/test-1"
        );
    }

    #[test]
    fn read_access_reads_input() {
        let mut value = connection(DeviceAccess::Read);

        let mut buffer = [0_u8; 6];

        let amount = value.read(&mut buffer)
            .expect("read succeeds");

        assert_eq!(amount, 6);
        assert_eq!(&buffer, b"Zamani");
    }

    #[test]
    fn read_access_rejects_writes() {
        let mut value = connection(DeviceAccess::Read);

        assert!(
            value.write(b"output").is_err()
        );
    }

    #[test]
    fn write_access_rejects_reads() {
        let mut value = connection(DeviceAccess::Write);

        let mut buffer = [0_u8; 4];

        assert!(
            value.read(&mut buffer).is_err()
        );
    }

    #[test]
    fn write_all_accepts_complete_output() {
        let mut value = connection(DeviceAccess::Write);

        value.write_all(b"Zamani")
            .expect("complete write succeeds");
    }

    #[test]
    fn empty_reads_do_not_contact_transport() {
        let mut value = connection(DeviceAccess::Read);

        assert_eq!(
            value.read(&mut []).expect("empty read succeeds"),
            0
        );
    }

    #[test]
    fn empty_writes_do_not_contact_transport() {
        let mut value = connection(DeviceAccess::Write);

        assert_eq!(
            value.write(&[]).expect("empty write succeeds"),
            0
        );
    }

    #[test]
    fn close_is_idempotent() {
        let mut value = connection(DeviceAccess::ReadWrite);

        value.close().expect("first close succeeds");
        value.close().expect("second close succeeds");

        assert!(value.is_closed());
    }

    #[test]
    fn closed_connection_rejects_io() {
        let mut value = connection(DeviceAccess::ReadWrite);

        value.close().expect("close succeeds");

        let mut buffer = [0_u8; 1];

        assert!(value.read(&mut buffer).is_err());
        assert!(value.write(b"x").is_err());
        assert!(value.flush().is_err());
    }

    #[test]
    fn read_write_access_allows_both_directions() {
        let mut value = connection(DeviceAccess::ReadWrite);

        let mut buffer = [0_u8; 6];

        assert_eq!(
            value.read(&mut buffer).expect("read succeeds"),
            6
        );

        assert_eq!(
            value.write(b"output").expect("write succeeds"),
            6
        );
    }
}
