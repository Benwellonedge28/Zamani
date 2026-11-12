
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Input and Output
//!
//! Path: `src/zur/io/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the public composition boundary for the Zamani Universal
//! Runtime (ZUR) input/output subsystem.
//!
//! This module integrates the existing standard-library I/O contracts
//! and ZUR-facing operational adapters.
//!
//! The subsystem supports:
//!
//! - Host input and output.
//! - Console operations.
//! - Filesystem integration.
//! - Synchronous byte streams.
//! - Asynchronous byte streams.
//! - Incremental transfers.
//! - Partial reads and writes.
//! - Cooperative cancellation.
//! - Device discovery through explicit providers.
//! - Device communication.
//! - I/O operation lifecycle tracking.
//! - Operation progress accounting.
//! - Operation failure reporting.
//! - Hybrid host/device I/O.
//! - Provider-backed execution environments.
//! - Future computational domains.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - I/O module composition.
//! - Public module exports.
//! - Stable I/O subsystem identification.
//! - I/O subsystem API version identification.
//! - Public architectural documentation.
//!
//! This module does NOT own:
//!
//! - Source-language I/O semantics.
//! - Filesystem implementation.
//! - Console implementation.
//! - Synchronous stream implementation.
//! - Asynchronous execution engines.
//! - Network transport.
//! - Distributed communication.
//! - Device drivers.
//! - Device discovery algorithms.
//! - Hardware allocation.
//! - Hardware scheduling.
//! - Memory allocation.
//! - Runtime task scheduling.
//! - Security authorization.
//! - Serialization.
//! - Compression.
//! - Encryption.
//! - Quantum execution.
//! - HDL synthesis.
//! - AI computation.
//! - Tensor storage.
//! - Compiler IR.
//! - Source grammar.
//!
//! Those responsibilities remain with their existing authoritative
//! repository subsystems.
//!
//! # Architecture
//!
//! ```text
//!                       ZUR I/O
//!                          |
//!           +--------------+--------------+
//!           |              |              |
//!           v              v              v
//!         HOST           STREAMS        DEVICES
//!           |              |              |
//!           |              |              |
//!           v              v              v
//!      stdlib::io      stdlib::io     stdlib::io
//!       console         streams        devices
//!      filesystem         |
//!           |              |
//!           +--------------+
//!                          |
//!                          v
//!                   I/O OPERATIONS
//!                          |
//!                          v
//!                 OperationContext
//!                          |
//!                          v
//!                    ZUR Errors
//!                          |
//!                          v
//!                  ZUR Execution
//! ```
//!
//! # Component responsibilities
//!
//! `host`
//!     Integrates standard-library console and filesystem providers
//!     into an explicitly constructed host-I/O service.
//!
//! `streams`
//!     Integrates canonical synchronous streams and provides
//!     asynchronous incremental stream coordination.
//!
//! `devices`
//!     Integrates explicitly supplied standard-library device providers
//!     with ZUR execution.
//!
//! `operations`
//!     Coordinates I/O operation identities, lifecycle, progress,
//!     cancellation observation, completion and failure reporting.
//!
//! # POCO-REAF
//!
//! The I/O subsystem participates in:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Programs describe logical I/O intent.
//!
//! The runtime and existing provider authorities determine how that
//! intent is realized on a compatible execution environment.
//!
//! The public I/O API does not require a particular:
//!
//! - Operating system.
//! - Processor architecture.
//! - CPU.
//! - GPU.
//! - FPGA.
//! - QPU.
//! - Accelerator.
//! - Filesystem.
//! - Network transport.
//! - Device vendor.
//! - Cloud provider.
//! - Distributed topology.
//! - Storage implementation.
//!
//! New providers and computational domains must integrate through
//! their canonical contracts without requiring the I/O aggregation
//! module to enumerate every possible technology.
//!
//! # Scalability
//!
//! This module introduces no arbitrary maximum for:
//!
//! - I/O operations.
//! - Streams.
//! - Devices.
//! - Providers.
//! - Concurrent connections.
//! - Transfer sizes.
//! - File sizes.
//! - Stream lengths.
//! - Buffer sizes.
//! - Computational domains.
//! - Distributed participants.
//! - Data volumes.
//!
//! Incremental transfer APIs should be preferred for large data.
//!
//! Buffer sizes are operational choices, not universal data-size limits.
//!
//! Actual execution depends on:
//!
//! - Available memory.
//! - Operating-system facilities.
//! - Provider capabilities.
//! - Filesystem capabilities.
//! - Hardware capabilities.
//! - Address-space representation.
//! - Explicit operational budgets.
//!
//! No finite machine promises literal infinite physical storage
//! or computation.
//!
//! # Error guarantees
//!
//! I/O operations must report their actual outcomes.
//!
//! In particular:
//!
//! - Successful reads represent bytes actually received.
//! - Successful writes represent bytes actually accepted.
//! - Partial transfers must be preserved accurately.
//! - Provider failures must remain failures.
//! - Unsupported operations must return structured errors.
//! - Cancellation must not be reported as successful completion.
//! - Operation submission must not imply execution completion.
//! - Progress must not imply resource reservation.
//!
//! No fabricated success is permitted.
//!
//! # Cancellation
//!
//! All I/O cancellation uses the canonical:
//!
//!     crate::stdlib::context::OperationContext
//!
//! This module introduces no independent cancellation token.
//!
//! Cancellation is cooperative.
//!
//! An already-submitted external operation may complete after
//! cancellation has been requested.
//!
//! Cleanup operations must remain possible where required.
//!
//! # Grammar integration
//!
//! The I/O subsystem is downstream of the canonical grammar,
//! frontend and semantic authorities.
//!
//! Relevant grammar files include:
//!
//! - grammar/effects/io.g4
//! - grammar/hybrid/host-device.g4
//! - grammar/execution/runtime.g4
//! - grammar/execution/runtime-capabilities.g4
//! - grammar/networking/streaming.g4
//! - grammar/data/streams.g4
//! - grammar/concurrency/channels.g4
//!
//! The grammar describes source-level intent.
//!
//! The frontend constructs the canonical AST.
//!
//! Semantic analysis validates types, effects, requirements,
//! capabilities, constraints and resource relationships.
//!
//! Compiler lowering produces the appropriate canonical
//! computational representation.
//!
//! Runtime execution consumes validated execution requests.
//!
//! This module does not parse grammar, interpret source syntax,
//! introduce I/O-specific IR or redefine semantic effects.
//!
//! # Integration boundaries
//!
//! Standard library:
//!
//!     crate::stdlib::io
//!     crate::stdlib::context
//!     crate::stdlib::error
//!     crate::stdlib::provider
//!
//! ZUR:
//!
//!     crate::zur::error
//!     crate::zur::ids
//!     crate::zur::contracts
//!     crate::zur::execution
//!     crate::zur::concurrency
//!     crate::zur::platforms
//!
//! Domain integration:
//!
//!     crate::hybrid
//!     crate::distributed
//!     crate::quantum
//!     crate::hdl
//!     crate::ai
//!
//! These systems retain their existing ownership.
//!
//! # Safety
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No unsafe functions.
//! No unsafe implementations.
//! No raw-pointer ownership.
//! No static mutable state.
//! No process-global registries.
//! No implicit provider initialization.
//! No implicit hardware discovery.
//! No fabricated execution results.
//!
//! # Initialization
//!
//! This module requires no global initialization.
//!
//! Individual services are explicitly constructed by their callers
//! or by the ZUR service-management subsystem.
//!
//! Merely importing this module does not:
//!
//! - Open files.
//! - Open devices.
//! - Initialize providers.
//! - Discover hardware.
//! - Start threads.
//! - Start an executor.
//! - Allocate device resources.
//! - Start network connections.
//!
//! # Compatibility
//!
//! Public module paths are stable composition boundaries.
//!
//! Implementation details remain inside their respective child modules.


// ============================================================================
// COMPONENT IDENTIFICATION
// ============================================================================

/// Stable identifier for the ZUR I/O subsystem.
pub const COMPONENT: &str = "zur.io";

/// Public ZUR I/O composition API version.
///
/// This version identifies the public composition contract.
///
/// It is NOT:
//!
//! - The Zamani language version.
//! - The Cargo package version.
//! - The runtime ABI version.
//! - The standard-library API version.
//! - A device-provider version.
//! - A filesystem-provider version.
//! - A network-protocol version.
pub const API_VERSION: &str = "1.0.0";


// ============================================================================
// CHILD MODULES
// ============================================================================

/// Host console and filesystem integration.
///
/// Reuses the canonical standard-library console and filesystem
/// implementations.
///
/// Provider implementations remain owned by the standard library
/// and their respective platform integrations.
pub mod host;

/// Asynchronous and synchronous byte-stream integration.
///
/// Reuses the canonical standard-library synchronous stream
/// implementation.
///
/// Adds ZUR asynchronous stream coordination, partial-transfer
/// handling, cancellation and transfer accounting.
pub mod streams;

/// Provider-backed device discovery and communication.
///
/// Reuses canonical standard-library device identities,
/// descriptors, connections and transport contracts.
///
/// Does not implement device drivers or hardware discovery algorithms.
pub mod devices;

/// I/O operation lifecycle and progress coordination.
///
/// Owns operation tracking, state transitions, cancellation
/// observation and diagnostic snapshots.
///
/// Does not implement an executor or transport.
pub mod operations;


// ============================================================================
// PUBLIC HOST I/O API
// ============================================================================

/// Explicitly constructed host-I/O service.
///
/// This is the ZUR integration adapter over the existing
/// standard-library console and filesystem APIs.
pub use host::HostIo;


// ============================================================================
// PUBLIC STREAM API
// ============================================================================

/// Canonical synchronous byte-source contract.
pub use streams::ByteSource;

/// Canonical synchronous byte-sink contract.
pub use streams::ByteSink;

/// Canonical synchronous stream reader.
pub use streams::StreamReader;

/// Canonical synchronous stream writer.
pub use streams::StreamWriter;

/// Canonical synchronous reader state.
pub use streams::ReaderState;

/// Canonical synchronous transfer statistics.
pub use streams::TransferStats;

/// Asynchronous byte-source contract.
pub use streams::AsyncByteSource;

/// Asynchronous byte-sink contract.
pub use streams::AsyncByteSink;

/// Asynchronous stream reader.
pub use streams::AsyncStreamReader;

/// Asynchronous stream writer.
pub use streams::AsyncStreamWriter;

/// Asynchronous transfer statistics.
pub use streams::AsyncTransferStats;

/// Canonical synchronous incremental stream-copy operation.
pub use streams::copy_sync_stream;


// ============================================================================
// PUBLIC DEVICE API
// ============================================================================

/// ZUR device-I/O service.
///
/// Accepts an explicitly supplied canonical device provider.
pub use devices::DeviceIo;

/// ZUR-facing device connection.
pub use devices::ZurDeviceConnection;

/// Canonical provider-scoped device identity.
pub use devices::ZurDeviceId;

/// Canonical open-world device-kind identity.
pub use devices::ZurDeviceKindId;

/// Canonical open-world device capability identity.
pub use devices::ZurDeviceCapabilityId;

/// Canonical provider-neutral device descriptor.
pub use devices::ZurDeviceDescriptor;

/// Canonical device access mode.
pub use devices::ZurDeviceAccess;

/// Canonical provider device transport.
pub use devices::ZurDeviceTransport;

/// Canonical device-provider contract.
pub use devices::ZurDeviceProvider;

/// Canonical device-provider identity.
pub use devices::ZurDeviceProviderId;


// ============================================================================
// PUBLIC OPERATION API
// ============================================================================

/// I/O operation lifecycle status.
pub use operations::OperationStatus;

/// Immutable operation failure information.
pub use operations::OperationFailure;

/// I/O operation progress accounting.
pub use operations::OperationProgress;

/// Immutable operation diagnostic snapshot.
pub use operations::OperationSnapshot;

/// Handle to a tracked I/O operation.
pub use operations::OperationHandle;

/// Explicitly constructed I/O operation registry.
pub use operations::IoOperationRegistry;


// ============================================================================
// SUBSYSTEM DESCRIPTOR
// ============================================================================

/// Immutable description of the ZUR I/O composition subsystem.
///
/// This descriptor contains architectural metadata only.
///
/// It does not:
///
/// - Initialize the subsystem.
/// - Discover providers.
/// - Allocate resources.
/// - Open devices.
/// - Open files.
/// - Establish network connections.
/// - Guarantee hardware availability.
/// - Guarantee execution feasibility.
///
/// The descriptor is independent of the host environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IoSubsystemDescriptor {
    /// Stable component identifier.
    pub component: &'static str,

    /// Public composition API version.
    pub api_version: &'static str,

    /// Whether this composition module requires a hardware target.
    ///
    /// The portable I/O composition API is target-independent.
    pub hardware_target_required: bool,

    /// Whether importing this module requires global initialization.
    ///
    /// The composition module requires no global initialization.
    pub global_initialization_required: bool,
}

impl IoSubsystemDescriptor {
    /// Returns the current immutable subsystem descriptor.
    ///
    /// No provider or resource initialization occurs.
    #[must_use]
    pub const fn current() -> Self {
        Self {
            component: COMPONENT,
            api_version: API_VERSION,
            hardware_target_required: false,
            global_initialization_required: false,
        }
    }
}


// ============================================================================
// PUBLIC DESCRIPTOR ACCESS
// ============================================================================

/// Returns the current ZUR I/O subsystem descriptor.
///
/// This function performs no I/O and requires no runtime initialization.
#[must_use]
pub const fn descriptor() -> IoSubsystemDescriptor {
    IoSubsystemDescriptor::current()
}


// ============================================================================
// SUBSYSTEM METADATA
// ============================================================================

/// Returns the stable ZUR I/O component identifier.
#[must_use]
pub const fn component_name() -> &'static str {
    COMPONENT
}

/// Returns the public ZUR I/O composition API version.
#[must_use]
pub const fn api_version() -> &'static str {
    API_VERSION
}


// ============================================================================
// ARCHITECTURAL TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // Component identity
    // ------------------------------------------------------------------------

    #[test]
    fn component_identifier_is_stable() {
        assert_eq!(
            component_name(),
            "zur.io",
        );
    }

    #[test]
    fn api_version_is_available() {
        assert_eq!(
            api_version(),
            API_VERSION,
        );
    }

    #[test]
    fn component_identifier_is_not_empty() {
        assert!(!COMPONENT.is_empty());
    }

    #[test]
    fn api_version_is_not_empty() {
        assert!(!API_VERSION.is_empty());
    }

    // ------------------------------------------------------------------------
    // Descriptor
    // ------------------------------------------------------------------------

    #[test]
    fn descriptor_identifies_the_subsystem() {
        let value = descriptor();

        assert_eq!(
            value.component,
            COMPONENT,
        );

        assert_eq!(
            value.api_version,
            API_VERSION,
        );
    }

    #[test]
    fn descriptor_is_target_independent() {
        let value = descriptor();

        assert!(
            !value.hardware_target_required,
        );
    }

    #[test]
    fn descriptor_requires_no_global_initialization() {
        let value = descriptor();

        assert!(
            !value.global_initialization_required,
        );
    }

    #[test]
    fn descriptor_is_deterministic() {
        assert_eq!(
            descriptor(),
            IoSubsystemDescriptor::current(),
        );
    }

    // ------------------------------------------------------------------------
    // Public module composition
    // ------------------------------------------------------------------------

    #[test]
    fn all_io_components_are_composed() {
        assert_eq!(
            host::COMPONENT,
            "zur.io.host",
        );

        assert_eq!(
            streams::COMPONENT,
            "zur.io.streams",
        );

        assert_eq!(
            devices::COMPONENT,
            "zur.io.devices",
        );

        assert_eq!(
            operations::COMPONENT,
            "zur.io.operations",
        );
    }

    #[test]
    fn child_api_versions_are_available() {
        assert!(!host::API_VERSION.is_empty());

        assert!(!devices::API_VERSION.is_empty());

        assert!(!operations::API_VERSION.is_empty());
    }

    // ------------------------------------------------------------------------
    // No implicit initialization
    // ------------------------------------------------------------------------

    #[test]
    fn descriptor_access_requires_no_host_resources() {
        let first = descriptor();
        let second = descriptor();

        assert_eq!(first, second);
    }
}
