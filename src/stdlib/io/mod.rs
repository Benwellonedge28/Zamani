
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Input and Output
//!
//! Path: `src/stdlib/io/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the stable public aggregation boundary for Zamani's
//! portable input/output subsystem.
//!
//! The I/O subsystem provides reusable interfaces for:
//!
//! - Byte-oriented input and output.
//! - Buffered and unbuffered streams.
//! - Standard input, output and error.
//! - Filesystem operations.
//! - Portable logical paths.
//! - Host-native path interoperability.
//! - Provider-neutral device communication.
//! - Incremental data transfers.
//! - Cooperative cancellation.
//! - Structured operation errors.
//!
//! The subsystem supports classical, quantum, HDL, hybrid,
//! distributed, AI, scientific, embedded and future computing
//! domains through stable, portable I/O contracts.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - The public I/O module boundary.
//! - Public I/O subsystem composition.
//! - Stable module exports.
//! - I/O subsystem identification.
//! - Public API documentation.
//!
//! This module does NOT own:
//!
//! - Operating-system resource allocation.
//! - Runtime scheduling.
//! - Physical hardware discovery.
//! - Device allocation.
//! - Quantum hardware execution.
//! - HDL synthesis or placement.
//! - Network transport implementation.
//! - Distributed routing.
//! - Storage placement.
//! - Tensor storage.
//! - Serialization formats.
//! - Source-language semantics.
//! - Compiler intermediate representations.
//! - Authentication or authorization policy.
//!
//! Those responsibilities remain with their canonical
//! repository authorities.
//!
//! # Module architecture
//!
//! ```text
//!                     stdlib::io
//!                         |
//!          +--------------+--------------+
//!          |              |              |
//!          v              v              v
//!       streams         console       filesystem
//!          |              |              |
//!          |              |              |
//!          +--------------+--------------+
//!                         |
//!             +-----------+-----------+
//!             |                       |
//!             v                       v
//!           paths                   devices
//!             |                       |
//!             +-----------+-----------+
//!                         |
//!                         v
//!                Shared I/O contracts
//!                         |
//!          +--------------+--------------+
//!          |              |              |
//!          v              v              v
//!        error         context        provider
//!                         |
//!                         v
//!                  runtime / HAL
//! ```
//!
//! # Canonical child-module ownership
//!
//! `streams`
//!     Owns portable byte-stream adapters, incremental transfers,
//!     buffering, partial-write handling and transfer accounting.
//!
//! `console`
//!     Owns standard input, standard output, standard error,
//!     line-oriented interaction and console providers.
//!
//! `filesystem`
//!     Owns filesystem API contracts and host filesystem operations.
//!
//! `paths`
//!     Owns portable logical paths and explicit host-path conversion.
//!
//! `devices`
//!     Owns provider-neutral byte-oriented device interfaces.
//!
//! Each child module has one implementation authority.
//!
//! This aggregation module must not introduce competing versions
//! of their types or operations.
//!
//! # Shared repository integration
//!
//! Structured errors:
//!     crate::stdlib::error
//!
//! Operation context and cancellation:
//!     crate::stdlib::context
//!
//! Generic provider contracts:
//!     crate::stdlib::provider
//!
//! Storage:
//!     crate::stdlib::data::storage
//!
//! Serialization:
//!     crate::stdlib::data::serialization
//!
//! Filesystem compatibility:
//!     crate::stdlib::fs
//!
//! Networking:
//!     crate::stdlib::net
//!     crate::stdlib::network
//!
//! Runtime:
//!     crate::runtime
//!
//! Hardware abstraction:
//!     Canonical HAL and hardware providers.
//!
//! Language semantics:
//!     crate::frontend::semantic
//!
//! # Portability
//!
//! The public I/O contracts must not require a particular:
//!
//! - Operating system.
//! - Filesystem.
//! - Processor architecture.
//! - CPU.
//! - GPU.
//! - FPGA.
//! - QPU.
//! - Network transport.
//! - Storage provider.
//! - Device implementation.
//!
//! Platform-specific implementations are supplied through
//! the appropriate host or provider interfaces.
//!
//! # Scalability
//!
//! This subsystem introduces no arbitrary universal maximum for:
//!
//! - File sizes.
//! - Stream sizes.
//! - Transfer sizes.
//! - Buffer capacities.
//! - Directory entries.
//! - Path components.
//! - Open streams.
//! - Open files.
//! - Devices.
//! - Providers.
//! - I/O operations.
//! - Computational domains.
//!
//! Transfers should be incremental wherever the underlying
//! operation supports incremental processing.
//!
//! Buffer capacities are operational choices, not limits on
//! the amount of data a program may process.
//!
//! Actual limits depend on:
//!
//! - Available memory.
//! - Address-space representation.
//! - Operating-system facilities.
//! - Filesystem capabilities.
//! - Provider capabilities.
//! - Hardware capabilities.
//! - Explicit operational budgets.
//!
//! No finite implementation promises literal infinite storage.
//!
//! Physical resource limitations must not become universal
//! Zamani language limitations.
//!
//! # Error guarantees
//!
//! I/O operations must report their actual outcomes.
//!
//! A successful write must represent bytes accepted by the
//! underlying writer or provider.
//!
//! A successful read must represent bytes actually obtained.
//!
//! A failed flush must remain a failure.
//!
//! A failed filesystem operation must never be represented
//! as successful completion.
//!
//! Unsupported operations must return structured errors.
//!
//! No conceptual or simulated success is permitted in
//! production host-I/O implementations.
//!
//! # Cancellation
//!
//! I/O operations use the shared OperationContext contract.
//!
//! Cancellation is cooperative.
//!
//! An operation that cannot be interrupted while blocked
//! must document that limitation.
//!
//! # Grammar boundary
//!
//! The canonical grammar root is:
//!
//!     grammar/Zamani.g4
//!
//! I/O syntax and effect declarations belong to their
//! respective imported grammar components.
//!
//! This Rust module implements APIs consumed after
//! parsing and semantic validation.
//!
//! It does not implement grammar parsing, semantic analysis,
//! source-language effects or compiler IR.
//!
//! # Compatibility
//!
//! Existing I/O functionality in:
//!
//!     crate::stdlib::io
//!
//! is migrated into this directory.
//!
//! The historical:
//!
//!     crate::stdlib::io
//!
//! path remains the canonical public path.
//!
//! The separate historical:
//!
//!     crate::stdlib::io.rs
//!
//! file must eventually become a compatibility facade
//! using this module's public APIs.
//!
//! # Safety
//!
//! All implementations must preserve the repository's
//! safe-Rust-only policy.

// ============================================================================
// MODULE IDENTIFICATION
// ============================================================================

/// Stable identifier for the Zamani standard-library I/O subsystem.
pub const COMPONENT: &str = "stdlib.io";

/// Current public I/O subsystem API version.
///
/// This identifies the I/O module contract, not the Rust compiler,
/// Zamani language, operating system or provider version.
pub const API_VERSION: &str = "1.0.0";

// ============================================================================
// CHILD MODULES
// ============================================================================

/// Portable byte-stream interfaces and adapters.
///
/// Owns incremental reading, writing, buffering, flushing,
/// completion and transfer accounting.
pub mod streams;

/// Console input and output interfaces.
///
/// Owns standard input, standard output, standard error,
/// line-oriented input and console provider integration.
pub mod console;

/// Filesystem operations and filesystem provider interfaces.
///
/// Owns file opening, reading, writing, metadata, directory
/// operations and filesystem-specific lifecycle contracts.
pub mod filesystem;

/// Portable logical paths and native path interoperability.
///
/// Owns path representation, parsing, normalization and
/// explicit conversion between portable and native paths.
pub mod paths;

/// Provider-neutral byte-oriented device communication.
///
/// Owns device descriptions, device access contracts,
/// connection interfaces and device I/O lifecycle.
pub mod devices;

// ============================================================================
// PUBLIC STREAM API
// ============================================================================

/// Public byte-stream source contract.
pub use streams::ByteSource;

/// Public byte-stream sink contract.
pub use streams::ByteSink;

/// Public stream reader.
pub use streams::StreamReader;

/// Public stream writer.
pub use streams::StreamWriter;

/// Public stream reader state.
pub use streams::ReaderState;

/// Public stream transfer statistics.
pub use streams::TransferStats;

// ============================================================================
// MODULE CONTRACT
// ============================================================================

/// Describes the architectural ownership of the I/O subsystem.
///
/// This is descriptive metadata only. It does not perform
/// provider discovery, hardware selection, allocation or execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoSubsystemDescriptor {
    /// Stable subsystem identifier.
    pub component: &'static str,

    /// Public subsystem API version.
    pub api_version: &'static str,

    /// Whether the subsystem requires a particular hardware target.
    ///
    /// The portable I/O API is target-independent.
    pub hardware_target_required: bool,

    /// Whether the subsystem requires global initialization.
    ///
    /// The module aggregation boundary itself requires no global
    /// initialization.
    pub global_initialization_required: bool,
}

impl IoSubsystemDescriptor {
    /// Returns the immutable I/O subsystem descriptor.
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
// PUBLIC DESCRIPTOR
// ============================================================================

/// Returns the current I/O subsystem descriptor.
///
/// This operation is independent of the host machine and does
/// not initialize providers or acquire operating-system resources.
#[must_use]
pub const fn descriptor() -> IoSubsystemDescriptor {
    IoSubsystemDescriptor::current()
}

// ============================================================================
// ARCHITECTURAL INVARIANTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subsystem_descriptor_is_stable() {
        let descriptor = descriptor();

        assert_eq!(descriptor.component, COMPONENT);
        assert_eq!(descriptor.api_version, API_VERSION);
    }

    #[test]
    fn subsystem_is_target_independent() {
        let descriptor = descriptor();

        assert!(!descriptor.hardware_target_required);
    }

    #[test]
    fn subsystem_requires_no_global_initialization() {
        let descriptor = descriptor();

        assert!(!descriptor.global_initialization_required);
    }

    #[test]
    fn component_identifier_is_not_empty() {
        assert!(!COMPONENT.is_empty());
    }

    #[test]
    fn api_version_is_not_empty() {
        assert!(!API_VERSION.is_empty());
    }
}
