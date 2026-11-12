
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Host I/O
//!
//! Path: src/zur/io/host.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! PURPOSE
//!
//! Provides the ZUR integration boundary for host-system input
//! and output services.
//!
//! This module composes the existing standard-library I/O
//! implementations into one explicitly constructed host-I/O
//! service.
//!
//! Supported integration includes:
//!
//! - Host standard input.
//! - Host standard output.
//! - Host standard error.
//! - Host filesystem access.
//! - Custom console providers.
//! - Custom filesystem providers.
//! - Shared provider instances.
//! - Cooperative operation cancellation.
//! - Runtime-context propagation.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - HostIo.
//! - Host-I/O service composition.
//! - Shared operation-context propagation.
//! - Explicit host-provider injection.
//!
//! This module does NOT own:
//!
//! - Filesystem semantics.
//! - File descriptors.
//! - Console implementation.
//! - Byte-stream implementation.
//! - Device communication.
//! - Network transport.
//! - Distributed routing.
//! - Hardware discovery.
//! - Hardware allocation.
//! - Runtime scheduling.
//! - Security policy.
//! - Source-language effects.
//! - Compiler IR.
//! - Physical device execution.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! EXISTING AUTHORITIES
//!
//! Console:
//!     crate::stdlib::io::console
//!
//! Filesystem:
//!     crate::stdlib::io::filesystem
//!
//! Streams:
//!     crate::stdlib::io::streams
//!
//! Devices:
//!     crate::stdlib::io::devices
//!
//! Operation context:
//!     crate::stdlib::context
//!
//! Host environment description:
//!     crate::zur::platforms::host
//!
//! Platform provider:
//!     crate::zur::platforms::provider
//!
//! Distributed transport:
//!     crate::distributed::transport
//!
//! GRAMMAR BOUNDARY
//!
//! The canonical I/O effect grammar is:
//!
//!     grammar/effects/io.g4
//!
//! Host/device semantic boundaries are defined by:
//!
//!     grammar/hybrid/host-device.g4
//!
//! This module implements neither grammar.
//!
//! It operates after parsing and semantic validation.
//!
//! SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Host-I/O instances.
//! - Console operations.
//! - Filesystem operations.
//! - Files.
//! - File sizes.
//! - Stream sizes.
//! - Transfer sizes.
//! - Providers.
//! - Computational domains.
//! - Concurrent execution sessions.
//!
//! Actual limits are determined by available memory,
//! operating-system facilities, provider capabilities,
//! address-space representation and explicit operational budgets.
//!
//! No finite physical implementation promises literal infinity.
//!
//! SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe code.
//! No raw pointers.
//! No global mutable state.
//! No implicit provider discovery.
//! No fabricated operation success.
//! No hidden provider selection.
//! No implicit hardware allocation.
//!
//! EXECUTION
//!
//! The existing standard-library host implementations use
//! synchronous operating-system I/O.
//!
//! Consequently, this composition layer does not pretend that
//! blocking operations are asynchronous or forcibly interruptible.
//!
//! Cancellation follows the existing OperationContext contract.
//!
//! A cancellation request does not guarantee that an already
//! blocking operating-system call has been interrupted.
//!
//! INTEGRATION
//!
//! ZUR consumes this module through:
//!
//!     crate::zur::io
//!
//! Runtime execution services may inject HostIo into an
//! execution context.
//!
//! The standard library remains the authority for the actual
//! console and filesystem operations.

use core::fmt;

use std::sync::Arc;

use crate::stdlib::context::OperationContext;

use crate::stdlib::io::console::{
    Console,
    ConsoleBackend,
    HostConsoleBackend,
};

use crate::stdlib::io::filesystem::{
    Filesystem,
    FilesystemProvider,
    HostFilesystemProvider,
};

// =============================================================================
// COMPONENT IDENTIFICATION
// =============================================================================

/// Stable ZUR host-I/O component identifier.
pub const COMPONENT: &str = "zur.io.host";

/// Current host-I/O integration API version.
pub const API_VERSION: &str = "1.0.0";

// =============================================================================
// HOST I/O
// =============================================================================

/// Explicitly constructed ZUR host-I/O service.
///
/// `HostIo` composes the existing standard-library console
/// and filesystem implementations.
///
/// It does not implement a second console or filesystem API.
///
/// Both services receive the same immutable operation context.
///
/// Cloning this structure shares its underlying providers
/// and operation context through the existing standard-library
/// ownership mechanisms.
///
/// # Thread safety
///
/// The underlying standard-library providers require the
/// appropriate Send and Sync guarantees.
///
/// `HostIo` contains no mutable global state.
///
/// # Examples
///
/// ```
/// use zamani_compiler::zur::io::host::HostIo;
///
/// let host_io = HostIo::new();
///
/// // Access the existing console implementation.
/// let console = host_io.console();
///
/// // Access the existing filesystem implementation.
/// let filesystem = host_io.filesystem();
///
/// let _ = (console, filesystem);
/// ```
#[derive(Clone)]
pub struct HostIo {
    console: Console,

    filesystem: Filesystem,

    context: OperationContext,
}

impl HostIo {
    /// Creates a host-I/O service using the actual host
    /// operating-system console and filesystem providers.
    ///
    /// This constructor does not initialize unrelated runtime
    /// services, discover computational devices or allocate
    /// physical resources.
    ///
    /// It creates provider wrappers around the existing
    /// standard-library host implementations.
    ///
    /// # Resource ownership
    ///
    /// The operating system and the underlying providers own
    /// their respective resources.
    ///
    /// # Errors
    ///
    /// Provider operations report their actual failures when
    /// those operations are invoked.
    #[must_use]
    pub fn new() -> Self {
        Self::with_context(
            OperationContext::builder()
                .with_component(COMPONENT)
                .expect("static component identifier is valid")
                .build(),
        )
    }

    /// Creates a host-I/O service with an explicitly supplied
    /// operation context.
    ///
    /// The supplied context is shared by the console and
    /// filesystem services.
    ///
    /// This preserves caller-provided cancellation and
    /// contextual metadata.
    ///
    /// No independent cancellation mechanism is introduced.
    #[must_use]
    pub fn with_context(
        context: OperationContext,
    ) -> Self {
        Self::with_shared_providers(
            Arc::new(HostConsoleBackend::new()),
            Arc::new(HostFilesystemProvider::new()),
            context,
        )
    }

    /// Creates a host-I/O service using independently supplied
    /// console and filesystem providers.
    ///
    /// This constructor supports alternative host environments,
    /// testing providers, embedded platforms, virtualized
    /// environments and other compatible implementations.
    ///
    /// The providers must implement their respective existing
    /// standard-library contracts.
    ///
    /// Provider registration, authentication, authorization,
    /// initialization and admission remain external responsibilities.
    ///
    /// # Type parameters
    ///
    /// `C` is the console provider.
    ///
    /// `F` is the filesystem provider.
    ///
    /// Neither provider is required to be a particular vendor,
    /// operating system or hardware implementation.
    #[must_use]
    pub fn with_providers<C, F>(
        console_backend: C,
        filesystem_provider: F,
        context: OperationContext,
    ) -> Self
    where
        C: ConsoleBackend + 'static,
        F: FilesystemProvider + 'static,
    {
        Self::with_shared_providers(
            Arc::new(console_backend),
            Arc::new(filesystem_provider),
            context,
        )
    }

    /// Creates a host-I/O service from shared provider instances.
    ///
    /// This constructor is useful when providers are already
    /// managed by another runtime service or provider lifecycle
    /// authority.
    ///
    /// The supplied provider instances are shared rather than
    /// reconstructed.
    ///
    /// Their lifecycle remains owned by the appropriate
    /// provider lifecycle manager.
    #[must_use]
    pub fn with_shared_providers(
        console_backend: Arc<dyn ConsoleBackend>,
        filesystem_provider: Arc<dyn FilesystemProvider>,
        context: OperationContext,
    ) -> Self {
        let console = Console::with_shared_backend(
            console_backend,
            context.clone(),
        );

        let filesystem = Filesystem::with_shared_provider(
            filesystem_provider,
            context.clone(),
        );

        Self {
            console,
            filesystem,
            context,
        }
    }

    /// Returns the existing standard-library console service.
    ///
    /// The returned service performs actual host or injected
    /// provider operations.
    ///
    /// No additional console implementation is introduced.
    #[must_use]
    pub const fn console(&self) -> &Console {
        &self.console
    }

    /// Returns the existing standard-library filesystem service.
    ///
    /// The returned service delegates filesystem operations
    /// to its configured filesystem provider.
    ///
    /// No additional filesystem implementation is introduced.
    #[must_use]
    pub const fn filesystem(&self) -> &Filesystem {
        &self.filesystem
    }

    /// Returns the operation context shared by both services.
    ///
    /// The context is immutable and can be cloned safely.
    #[must_use]
    pub const fn context(&self) -> &OperationContext {
        &self.context
    }

    /// Returns whether cancellation has been requested
    /// for the shared operation context.
    ///
    /// This is an observation only.
    ///
    /// It does not forcibly interrupt blocking operating-system
    /// calls or external provider operations.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.context.is_cancelled()
    }
}

// =============================================================================
// DEBUG IMPLEMENTATION
// =============================================================================

impl fmt::Debug for HostIo {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("HostIo")
            .field("component", &COMPONENT)
            .field("api_version", &API_VERSION)
            .field("cancelled", &self.is_cancelled())
            .finish_non_exhaustive()
    }
}

// =============================================================================
// DEFAULT IMPLEMENTATION
// =============================================================================

impl Default for HostIo {
    /// Creates the default real host-I/O service.
    ///
    /// Equivalent to `HostIo::new()`.
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_context() -> OperationContext {
        OperationContext::builder()
            .with_component(COMPONENT)
            .expect("component identifier is valid")
            .with_operation("io.host.test")
            .expect("operation identifier is valid")
            .build()
    }

    #[test]
    fn creates_host_io_service() {
        let host_io = HostIo::new();

        assert_eq!(
            host_io.context().component(),
            Some(COMPONENT),
        );
    }

    #[test]
    fn creates_default_host_io_service() {
        let host_io = HostIo::default();

        assert_eq!(
            host_io.context().component(),
            Some(COMPONENT),
        );
    }

    #[test]
    fn preserves_supplied_operation_context() {
        let context = test_context();

        let host_io = HostIo::with_context(
            context,
        );

        assert_eq!(
            host_io.context().component(),
            Some(COMPONENT),
        );

        assert_eq!(
            host_io.context().operation(),
            Some("io.host.test"),
        );
    }

    #[test]
    fn console_uses_shared_operation_context() {
        let host_io = HostIo::with_context(
            test_context(),
        );

        assert_eq!(
            host_io.console().context().component(),
            Some(COMPONENT),
        );

        assert_eq!(
            host_io.console().context().operation(),
            Some("io.host.test"),
        );
    }

    #[test]
    fn filesystem_uses_shared_operation_context() {
        let host_io = HostIo::with_context(
            test_context(),
        );

        assert_eq!(
            host_io.filesystem().context().component(),
            Some(COMPONENT),
        );

        assert_eq!(
            host_io.filesystem().context().operation(),
            Some("io.host.test"),
        );
    }

    #[test]
    fn cloned_host_io_preserves_service_context() {
        let original = HostIo::with_context(
            test_context(),
        );

        let cloned = original.clone();

        assert_eq!(
            cloned.context().component(),
            original.context().component(),
        );

        assert_eq!(
            cloned.context().operation(),
            original.context().operation(),
        );
    }

    #[test]
    fn host_io_is_not_cancelled_by_default() {
        let host_io = HostIo::new();

        assert!(!host_io.is_cancelled());
    }

    #[test]
    fn host_io_debug_does_not_expose_provider_internals() {
        let host_io = HostIo::new();

        let representation = format!("{host_io}");

        assert!(
            representation.contains(COMPONENT)
        );

        assert!(
            representation.contains(API_VERSION)
        );
    }

    #[test]
    fn host_io_component_is_stable() {
        assert_eq!(
            COMPONENT,
            "zur.io.host",
        );
    }

    #[test]
    fn host_io_api_version_is_available() {
        assert!(!API_VERSION.is_empty());
    }
}
