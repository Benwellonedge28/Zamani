//! Zamani Quantum Memory — Production Error Model
//!
//! This module defines the complete provider-neutral error contract for
//! `quantum::memory`.
//!
//! # Responsibility
//!
//! `errors.rs` owns:
//!
//! - the canonical `MemoryError` type;
//! - stable machine-readable error codes;
//! - error categories;
//! - structured error context;
//! - resource-limit failures;
//! - allocation failures;
//! - indexing and layout failures;
//! - logical-memory ownership/lifetime failures;
//! - quantum-state validity failures;
//! - numerical failures;
//! - representation failures;
//! - persistence failures;
//! - synchronization/coherence failures;
//! - migration failures;
//! - backend/device failures;
//! - concurrency failures;
//! - integrity failures;
//! - validation failures;
//! - unsupported-operation failures;
//! - invariant violations;
//! - conversion helpers;
//! - source-error preservation where appropriate.
//!
//! It deliberately does NOT own:
//!
//! - memory allocation;
//! - state-vector implementation;
//! - density-matrix implementation;
//! - stabilizer implementation;
//! - tensor-network implementation;
//! - GPU implementation;
//! - distributed-memory implementation;
//! - serialization formats;
//! - backend adapters;
//! - routing;
//! - scheduling;
//! - benchmarking;
//! - compiler/IR semantics.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Architectural position
//!
//! ```text
//!                     quantum::memory
//!                           |
//!          +----------------+----------------+
//!          |                |                |
//!          v                v                v
//!      allocation        state           persistence
//!          |                |                |
//!          +----------------+----------------+
//!                           |
//!                           v
//!                    MemoryError
//!                           |
//!          +----------------+----------------+
//!          |                |                |
//!          v                v                v
//!       caller          runtime          diagnostics
//! ```
//!
//! Every fallible public operation in `quantum::memory` should return:
//!
//! ```text
//! Result<T, MemoryError>
//! ```
//!
//! or an error type that contains/converts from `MemoryError` without
//! discarding its structured information.
//!
//! # Design goals
//!
//! The error model is designed to provide:
//!
//! 1. stable machine-readable error codes;
//! 2. human-readable messages;
//! 3. enough structured information for diagnostics;
//! 4. preservation of the underlying cause where appropriate;
//! 5. deterministic behavior;
//! 6. no panics for expected operational failures;
//! 7. no vendor lock-in;
//! 8. no dependency on future memory modules;
//! 9. compatibility with Rust 1.97.1;
//! 10. zero `unsafe` code.
//!
//! # Important semantic rule
//!
//! An error is not permission to expose implementation details accidentally.
//!
//! Error messages MUST NOT contain:
//!
//! - credentials;
//! - API keys;
//! - authentication tokens;
//! - private keys;
//! - passwords;
//! - authorization headers;
//! - secret quantum-program data;
//! - raw memory addresses;
//! - device pointers;
//! - provider session secrets.
//!
//! Callers may attach safe diagnostic context explicitly.
//!
//! # Stable error identity
//!
//! Consumers should branch on `MemoryError::code()` or on the enum variant,
//! rather than parsing `Display` text.
//!
//! Display messages are human-readable and may evolve without changing the
//! semantic identity of an error.
//!
//! # Rust compatibility
//!
//! This file targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! ```text
//! #![deny(unsafe_code)]
//! ```
//!
//! # Dependencies
//!
//! Only the already-declared `thiserror` dependency is required.
//!
//! No memory subsystem module is imported here. This is intentional:
//! `errors.rs` is one of the foundational files and must be completed before
//! allocator, state, persistence, GPU, distributed, or migration modules.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::num::TryFromIntError;

use thiserror::Error;

// =============================================================================
// Schema
// =============================================================================

/// Stable identifier for the Zamani quantum-memory error contract.
pub const MEMORY_ERROR_SCHEMA_ID: &str = "zamani.quantum.memory.error";

/// Semantic version of the memory-error contract.
///
/// Increment this only when the public semantic contract changes
/// incompatibly.
pub const MEMORY_ERROR_SCHEMA_VERSION: u16 = 1;

/// Maximum length of a caller-provided diagnostic context field.
///
/// This protects error construction from accidentally retaining arbitrarily
/// large strings.
pub const MAX_CONTEXT_LENGTH: usize = 4_096;

/// Maximum length of a logical operation name.
pub const MAX_OPERATION_NAME_LENGTH: usize = 256;

/// Maximum length of a representation name.
pub const MAX_REPRESENTATION_NAME_LENGTH: usize = 256;

/// Maximum length of a backend/provider name.
pub const MAX_BACKEND_NAME_LENGTH: usize = 256;

// =============================================================================
// Error code
// =============================================================================

/// Stable machine-readable identity for a `MemoryError`.
///
/// Error codes are intentionally independent of display text.
///
/// Applications, Danga, diagnostics, telemetry, tests, and future language
/// tooling should use these codes instead of parsing human-readable messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum MemoryErrorCode {
    /// Generic invalid argument.
    InvalidArgument = 1,

    /// A required identifier is invalid.
    InvalidIdentifier = 2,

    /// An operation addressed an invalid index.
    OutOfBounds = 3,

    /// Arithmetic required for an operation overflowed.
    ArithmeticOverflow = 4,

    /// A requested allocation cannot be satisfied.
    AllocationFailed = 10,

    /// A configured memory budget would be exceeded.
    BudgetExceeded = 11,

    /// A configured memory limit would be exceeded.
    MemoryLimitExceeded = 12,

    /// Too many allocations were requested.
    AllocationCountExceeded = 13,

    /// A reservation could not be created or committed.
    ReservationFailed = 14,

    /// A memory pool operation failed.
    PoolError = 15,

    /// A memory address is invalid for the requested operation.
    InvalidAddress = 16,

    /// A memory layout is invalid.
    InvalidLayout = 20,

    /// A stride or shape is invalid.
    InvalidShape = 21,

    /// A tensor rank or dimension is invalid.
    InvalidDimension = 22,

    /// A qubit permutation is invalid.
    InvalidPermutation = 23,

    /// An indexing operation is mathematically invalid.
    InvalidIndex = 24,

    /// A view cannot be constructed safely.
    InvalidView = 25,

    /// A slice cannot be constructed safely.
    InvalidSlice = 26,

    /// An aliasing rule was violated.
    AliasingViolation = 30,

    /// A memory object was used after its lifetime ended.
    LifetimeViolation = 31,

    /// A released object was accessed.
    UseAfterRelease = 32,

    /// Ownership rules were violated.
    OwnershipViolation = 33,

    /// A logical qubit/register operation is invalid.
    LogicalMemoryError = 34,

    /// A classical-memory operation is invalid.
    ClassicalMemoryError = 35,

    /// A quantum-state dimension does not match the requested operation.
    StateDimensionMismatch = 40,

    /// A quantum state is mathematically invalid.
    InvalidState = 41,

    /// A state contains NaN or another non-finite value.
    NonFiniteValue = 42,

    /// A state is not normalized when normalization is required.
    NotNormalized = 43,

    /// A density matrix is not Hermitian when Hermiticity is required.
    NotHermitian = 44,

    /// A density matrix has an invalid trace.
    InvalidTrace = 45,

    /// A probability is invalid.
    InvalidProbability = 46,

    /// A quantum operation would violate a representation invariant.
    StateInvariantViolation = 47,

    /// A requested state representation is unavailable.
    UnsupportedRepresentation = 50,

    /// A requested representation conversion is unavailable.
    UnsupportedConversion = 51,

    /// A requested memory operation is unavailable.
    UnsupportedOperation = 52,

    /// A requested scalar/precision type is unavailable.
    UnsupportedPrecision = 53,

    /// A requested storage location is unavailable.
    UnsupportedStorageLocation = 54,

    /// A measurement request is invalid.
    MeasurementError = 60,

    /// Measurement probabilities are inconsistent.
    MeasurementProbabilityError = 61,

    /// Measurement collapse could not be completed.
    CollapseError = 62,

    /// Reset could not be completed.
    ResetError = 63,

    /// Serialization failed.
    SerializationError = 70,

    /// Deserialization failed.
    DeserializationError = 71,

    /// A snapshot is invalid.
    InvalidSnapshot = 72,

    /// A checkpoint is invalid.
    InvalidCheckpoint = 73,

    /// A persisted object is from an unsupported schema version.
    UnsupportedSchemaVersion = 74,

    /// Persisted data is corrupt or incomplete.
    CorruptData = 75,

    /// Persisted data failed integrity verification.
    IntegrityError = 76,

    /// Host/device/distributed coherence state is invalid.
    CoherenceError = 80,

    /// Synchronization failed.
    SynchronizationError = 81,

    /// A synchronization operation timed out.
    SynchronizationTimeout = 82,

    /// A concurrent operation conflicted with another operation.
    ConcurrencyConflict = 83,

    /// A required migration could not be completed.
    MigrationError = 90,

    /// A state migration would exceed resource limits.
    MigrationResourceExceeded = 91,

    /// Compaction failed.
    CompactionError = 92,

    /// A CPU memory provider failed.
    HostMemoryError = 100,

    /// A GPU/device memory provider failed.
    DeviceMemoryError = 101,

    /// A distributed-memory provider failed.
    DistributedMemoryError = 102,

    /// A backend-native memory operation failed.
    BackendMemoryError = 103,

    /// A provider/backend rejected an operation.
    BackendRejected = 104,

    /// A backend does not expose a requested capability.
    BackendCapabilityUnavailable = 105,

    /// A required memory invariant was violated.
    InvariantViolation = 110,

    /// An internal implementation error occurred.
    InternalError = 111,
}

impl MemoryErrorCode {
    /// Returns the stable numeric representation of this code.
    pub const fn as_u16(self) -> u16 {
        self as u16
    }

    /// Returns the stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidArgument => "QM001",
            Self::InvalidIdentifier => "QM002",
            Self::OutOfBounds => "QM003",
            Self::ArithmeticOverflow => "QM004",

            Self::AllocationFailed => "QM010",
            Self::BudgetExceeded => "QM011",
            Self::MemoryLimitExceeded => "QM012",
            Self::AllocationCountExceeded => "QM013",
            Self::ReservationFailed => "QM014",
            Self::PoolError => "QM015",
            Self::InvalidAddress => "QM016",

            Self::InvalidLayout => "QM020",
            Self::InvalidShape => "QM021",
            Self::InvalidDimension => "QM022",
            Self::InvalidPermutation => "QM023",
            Self::InvalidIndex => "QM024",
            Self::InvalidView => "QM025",
            Self::InvalidSlice => "QM026",

            Self::AliasingViolation => "QM030",
            Self::LifetimeViolation => "QM031",
            Self::UseAfterRelease => "QM032",
            Self::OwnershipViolation => "QM033",
            Self::LogicalMemoryError => "QM034",
            Self::ClassicalMemoryError => "QM035",

            Self::StateDimensionMismatch => "QM040",
            Self::InvalidState => "QM041",
            Self::NonFiniteValue => "QM042",
            Self::NotNormalized => "QM043",
            Self::NotHermitian => "QM044",
            Self::InvalidTrace => "QM045",
            Self::InvalidProbability => "QM046",
            Self::StateInvariantViolation => "QM047",

            Self::UnsupportedRepresentation => "QM050",
            Self::UnsupportedConversion => "QM051",
            Self::UnsupportedOperation => "QM052",
            Self::UnsupportedPrecision => "QM053",
            Self::UnsupportedStorageLocation => "QM054",

            Self::MeasurementError => "QM060",
            Self::MeasurementProbabilityError => "QM061",
            Self::CollapseError => "QM062",
            Self::ResetError => "QM063",

            Self::SerializationError => "QM070",
            Self::DeserializationError => "QM071",
            Self::InvalidSnapshot => "QM072",
            Self::InvalidCheckpoint => "QM073",
            Self::UnsupportedSchemaVersion => "QM074",
            Self::CorruptData => "QM075",
            Self::IntegrityError => "QM076",

            Self::CoherenceError => "QM080",
            Self::SynchronizationError => "QM081",
            Self::SynchronizationTimeout => "QM082",
            Self::ConcurrencyConflict => "QM083",

            Self::MigrationError => "QM090",
            Self::MigrationResourceExceeded => "QM091",
            Self::CompactionError => "QM092",

            Self::HostMemoryError => "QM100",
            Self::DeviceMemoryError => "QM101",
            Self::DistributedMemoryError => "QM102",
            Self::BackendMemoryError => "QM103",
            Self::BackendRejected => "QM104",
            Self::BackendCapabilityUnavailable => "QM105",

            Self::InvariantViolation => "QM110",
            Self::InternalError => "QM111",
        }
    }

    /// Returns the broad category of this error.
    pub const fn category(self) -> MemoryErrorCategory {
        match self {
            Self::InvalidArgument
            | Self::InvalidIdentifier
            | Self::OutOfBounds
            | Self::ArithmeticOverflow => MemoryErrorCategory::Validation,

            Self::AllocationFailed
            | Self::BudgetExceeded
            | Self::MemoryLimitExceeded
            | Self::AllocationCountExceeded
            | Self::ReservationFailed
            | Self::PoolError
            | Self::InvalidAddress => MemoryErrorCategory::Allocation,

            Self::InvalidLayout
            | Self::InvalidShape
            | Self::InvalidDimension
            | Self::InvalidPermutation
            | Self::InvalidIndex
            | Self::InvalidView
            | Self::InvalidSlice => MemoryErrorCategory::Layout,

            Self::AliasingViolation
            | Self::LifetimeViolation
            | Self::UseAfterRelease
            | Self::OwnershipViolation
            | Self::LogicalMemoryError
            | Self::ClassicalMemoryError => MemoryErrorCategory::Ownership,

            Self::StateDimensionMismatch
            | Self::InvalidState
            | Self::NonFiniteValue
            | Self::NotNormalized
            | Self::NotHermitian
            | Self::InvalidTrace
            | Self::InvalidProbability
            | Self::StateInvariantViolation => MemoryErrorCategory::QuantumState,

            Self::UnsupportedRepresentation
            | Self::UnsupportedConversion
            | Self::UnsupportedOperation
            | Self::UnsupportedPrecision
            | Self::UnsupportedStorageLocation => MemoryErrorCategory::Capability,

            Self::MeasurementError
            | Self::MeasurementProbabilityError
            | Self::CollapseError
            | Self::ResetError => MemoryErrorCategory::QuantumOperation,

            Self::SerializationError
            | Self::DeserializationError
            | Self::InvalidSnapshot
            | Self::InvalidCheckpoint
            | Self::UnsupportedSchemaVersion
            | Self::CorruptData
            | Self::IntegrityError => MemoryErrorCategory::Persistence,

            Self::CoherenceError
            | Self::SynchronizationError
            | Self::SynchronizationTimeout
            | Self::ConcurrencyConflict => MemoryErrorCategory::Concurrency,

            Self::MigrationError
            | Self::MigrationResourceExceeded
            | Self::CompactionError => MemoryErrorCategory::Lifecycle,

            Self::HostMemoryError
            | Self::DeviceMemoryError
            | Self::DistributedMemoryError
            | Self::BackendMemoryError
            | Self::BackendRejected
            | Self::BackendCapabilityUnavailable => MemoryErrorCategory::Backend,

            Self::InvariantViolation | Self::InternalError => {
                MemoryErrorCategory::Internal
            }
        }
    }

    /// Returns whether this error represents invalid caller input.
    pub const fn is_validation(self) -> bool {
        matches!(self.category(), MemoryErrorCategory::Validation)
    }

    /// Returns whether retrying the same operation without changing state is
    /// potentially meaningful.
    ///
    /// This is deliberately conservative.
    pub const fn is_potentially_retryable(self) -> bool {
        matches!(
            self,
            Self::AllocationFailed
                | Self::PoolError
                | Self::SynchronizationError
                | Self::SynchronizationTimeout
                | Self::ConcurrencyConflict
                | Self::HostMemoryError
                | Self::DeviceMemoryError
                | Self::DistributedMemoryError
                | Self::BackendMemoryError
        )
    }
}

impl fmt::Display for MemoryErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Error category
// =============================================================================

/// Broad category used by diagnostics, telemetry, and tooling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MemoryErrorCategory {
    /// Caller supplied invalid data.
    Validation,

    /// Allocation/resource-management failure.
    Allocation,

    /// Layout/index/shape failure.
    Layout,

    /// Ownership/lifetime failure.
    Ownership,

    /// Quantum-state mathematical failure.
    QuantumState,

    /// Requested capability is unavailable.
    Capability,

    /// Measurement/reset/collapse failure.
    QuantumOperation,

    /// Persistence/serialization failure.
    Persistence,

    /// Concurrency/coherence failure.
    Concurrency,

    /// State migration/compaction/lifecycle failure.
    Lifecycle,

    /// Hardware/backend/storage-provider failure.
    Backend,

    /// Internal invariant or implementation failure.
    Internal,
}

impl MemoryErrorCategory {
    /// Stable machine-readable category name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Validation => "validation",
            Self::Allocation => "allocation",
            Self::Layout => "layout",
            Self::Ownership => "ownership",
            Self::QuantumState => "quantum_state",
            Self::Capability => "capability",
            Self::QuantumOperation => "quantum_operation",
            Self::Persistence => "persistence",
            Self::Concurrency => "concurrency",
            Self::Lifecycle => "lifecycle",
            Self::Backend => "backend",
            Self::Internal => "internal",
        }
    }
}

impl fmt::Display for MemoryErrorCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Structured diagnostic context
// =============================================================================

/// Optional safe diagnostic context attached to a memory error.
///
/// Context is intentionally limited in size and represented as ordinary text.
/// It must never be used to carry secrets.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ErrorContext {
    operation: Option<String>,
    resource: Option<String>,
    detail: Option<String>,
}

impl ErrorContext {
    /// Creates an empty context.
    pub const fn new() -> Self {
        Self {
            operation: None,
            resource: None,
            detail: None,
        }
    }

    /// Sets the logical operation name.
    pub fn with_operation(
        mut self,
        operation: impl Into<String>,
    ) -> Result<Self, MemoryError> {
        self.set_operation(operation)?;
        Ok(self)
    }

    /// Sets the logical resource name.
    pub fn with_resource(
        mut self,
        resource: impl Into<String>,
    ) -> Result<Self, MemoryError> {
        self.set_resource(resource)?;
        Ok(self)
    }

    /// Sets additional safe diagnostic detail.
    pub fn with_detail(
        mut self,
        detail: impl Into<String>,
    ) -> Result<Self, MemoryError> {
        self.set_detail(detail)?;
        Ok(self)
    }

    /// Sets the operation name in-place.
    pub fn set_operation(
        &mut self,
        operation: impl Into<String>,
    ) -> Result<(), MemoryError> {
        let value = validate_context("operation", operation.into(), MAX_OPERATION_NAME_LENGTH)?;
        self.operation = Some(value);
        Ok(())
    }

    /// Sets the resource name in-place.
    pub fn set_resource(
        &mut self,
        resource: impl Into<String>,
    ) -> Result<(), MemoryError> {
        let value = validate_context("resource", resource.into(), MAX_CONTEXT_LENGTH)?;
        self.resource = Some(value);
        Ok(())
    }

    /// Sets diagnostic detail in-place.
    pub fn set_detail(
        &mut self,
        detail: impl Into<String>,
    ) -> Result<(), MemoryError> {
        let value = validate_context("detail", detail.into(), MAX_CONTEXT_LENGTH)?;
        self.detail = Some(value);
        Ok(())
    }

    /// Returns the operation name, if present.
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Returns the resource name, if present.
    pub fn resource(&self) -> Option<&str> {
        self.resource.as_deref()
    }

    /// Returns diagnostic detail, if present.
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    /// Returns whether the context contains no fields.
    pub const fn is_empty(&self) -> bool {
        self.operation.is_none()
            && self.resource.is_none()
            && self.detail.is_none()
    }
}

// =============================================================================
// Canonical error
// =============================================================================

/// Canonical error returned by the Zamani quantum-memory subsystem.
///
/// This type intentionally contains all failure classes needed by the planned
/// memory architecture without referring to future memory modules. That means
/// later files can be implemented against this contract without requiring this
/// file to be reopened merely to add another basic error category.
#[derive(Debug, Error)]
pub enum MemoryError {
    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Invalid argument supplied to a memory operation.
    #[error(
        "{code}: invalid argument `{argument}`{context}",
        code = .code(),
        context = format_context(.context.as_ref())
    )]
    InvalidArgument {
        /// Name of the invalid argument.
        argument: String,

        /// Optional safe context.
        context: Option<ErrorContext>,
    },

    /// Identifier validation failed.
    #[error(
        "{code}: invalid identifier `{identifier}`{context}",
        code = .code(),
        context = format_context(.context.as_ref())
    )]
    InvalidIdentifier {
        /// Identifier kind.
        identifier: String,

        /// Optional safe context.
        context: Option<ErrorContext>,
    },

    /// An index was outside the valid domain.
    #[error(
        "{code}: index {index} is out of bounds for `{resource}` with length {length}",
        code = .code()
    )]
    OutOfBounds {
        /// Requested index.
        index: u64,

        /// Valid logical length.
        length: u64,

        /// Resource being indexed.
        resource: String,
    },

    /// Checked arithmetic overflowed.
    #[error(
        "{code}: arithmetic overflow while computing `{operation}`",
        code = .code()
    )]
    ArithmeticOverflow {
        /// Operation being calculated.
        operation: String,
    },

    // -------------------------------------------------------------------------
    // Allocation / resource limits
    // -------------------------------------------------------------------------

    /// An allocation request could not be satisfied.
    #[error(
        "{code}: allocation of {requested_bytes} bytes failed; available capacity: {available_bytes} bytes",
        code = .code()
    )]
    AllocationFailed {
        /// Requested bytes.
        requested_bytes: u64,

        /// Known available bytes.
        available_bytes: u64,
    },

    /// A memory budget would be exceeded.
    #[error(
        "{code}: memory budget `{budget}` exceeded: requested {requested_bytes} bytes, remaining {remaining_bytes} bytes",
        code = .code()
    )]
    BudgetExceeded {
        /// Budget name.
        budget: String,

        /// Requested bytes.
        requested_bytes: u64,

        /// Remaining bytes.
        remaining_bytes: u64,
    },

    /// A configured hard memory limit would be exceeded.
    #[error(
        "{code}: memory limit `{limit}` exceeded: requested {requested_bytes} bytes, maximum {maximum_bytes} bytes",
        code = .code()
    )]
    MemoryLimitExceeded {
        /// Limit name.
        limit: String,

        /// Requested amount.
        requested_bytes: u64,

        /// Maximum permitted amount.
        maximum_bytes: u64,
    },

    /// Maximum allocation count exceeded.
    #[error(
        "{code}: allocation count limit exceeded: requested count {requested_count}, maximum {maximum_count}",
        code = .code()
    )]
    AllocationCountExceeded {
        /// Requested resulting count.
        requested_count: u64,

        /// Maximum permitted count.
        maximum_count: u64,
    },

    /// A memory reservation could not be created or committed.
    #[error(
        "{code}: memory reservation failed for {requested_bytes} bytes: {reason}",
        code = .code()
    )]
    ReservationFailed {
        /// Requested bytes.
        requested_bytes: u64,

        /// Safe reason.
        reason: String,
    },

    /// Memory-pool operation failed.
    #[error(
        "{code}: memory pool operation failed: {reason}",
        code = .code()
    )]
    PoolError {
        /// Safe reason.
        reason: String,
    },

    /// Memory address is invalid for the requested operation.
    #[error(
        "{code}: invalid memory address for `{operation}`",
        code = .code()
    )]
    InvalidAddress {
        /// Operation being performed.
        operation: String,
    },

    // -------------------------------------------------------------------------
    // Layout / indexing
    // -------------------------------------------------------------------------

    /// Memory layout is invalid.
    #[error(
        "{code}: invalid memory layout: {reason}",
        code = .code()
    )]
    InvalidLayout {
        /// Safe reason.
        reason: String,
    },

    /// Tensor/state shape is invalid.
    #[error(
        "{code}: invalid shape: {reason}",
        code = .code()
    )]
    InvalidShape {
        /// Safe reason.
        reason: String,
    },

    /// Dimension is invalid.
    #[error(
        "{code}: invalid dimension `{dimension}`: {reason}",
        code = .code()
    )]
    InvalidDimension {
        /// Dimension name.
        dimension: String,

        /// Safe reason.
        reason: String,
    },

    /// Logical-to-physical permutation is invalid.
    #[error(
        "{code}: invalid qubit permutation: {reason}",
        code = .code()
    )]
    InvalidPermutation {
        /// Safe reason.
        reason: String,
    },

    /// General index calculation is invalid.
    #[error(
        "{code}: invalid index for `{resource}`: {reason}",
        code = .code()
    )]
    InvalidIndex {
        /// Resource being indexed.
        resource: String,

        /// Safe reason.
        reason: String,
    },

    /// A non-owning memory view cannot be constructed safely.
    #[error(
        "{code}: invalid memory view: {reason}",
        code = .code()
    )]
    InvalidView {
        /// Safe reason.
        reason: String,
    },

    /// A memory/state slice cannot be constructed safely.
    #[error(
        "{code}: invalid slice: {reason}",
        code = .code()
    )]
    InvalidSlice {
        /// Safe reason.
        reason: String,
    },

    // -------------------------------------------------------------------------
    // Ownership / lifetime
    // -------------------------------------------------------------------------

    /// Aliasing rules were violated.
    #[error(
        "{code}: aliasing violation: {reason}",
        code = .code()
    )]
    AliasingViolation {
        /// Safe reason.
        reason: String,
    },

    /// An object was accessed outside its valid lifetime.
    #[error(
        "{code}: lifetime violation: {reason}",
        code = .code()
    )]
    LifetimeViolation {
        /// Safe reason.
        reason: String,
    },

    /// A released object was accessed.
    #[error(
        "{code}: use-after-release for `{resource}`",
        code = .code()
    )]
    UseAfterRelease {
        /// Resource identifier.
        resource: String,
    },

    /// Ownership rules were violated.
    #[error(
        "{code}: ownership violation: {reason}",
        code = .code()
    )]
    OwnershipViolation {
        /// Safe reason.
        reason: String,
    },

    /// Logical quantum-memory operation failed.
    #[error(
        "{code}: logical memory error: {reason}",
        code = .code()
    )]
    LogicalMemoryError {
        /// Safe reason.
        reason: String,
    },

    /// Classical companion-memory operation failed.
    #[error(
        "{code}: classical memory error: {reason}",
        code = .code()
    )]
    ClassicalMemoryError {
        /// Safe reason.
        reason: String,
    },

    // -------------------------------------------------------------------------
    // Quantum state
    // -------------------------------------------------------------------------

    /// State dimensions do not match the requested operation.
    #[error(
        "{code}: state dimension mismatch: expected {expected}, actual {actual}",
        code = .code()
    )]
    StateDimensionMismatch {
        /// Expected dimension.
        expected: u64,

        /// Actual dimension.
        actual: u64,
    },

    /// Quantum state failed mathematical validation.
    #[error(
        "{code}: invalid quantum state: {reason}",
        code = .code()
    )]
    InvalidState {
        /// Safe reason.
        reason: String,
    },

    /// A non-finite scalar was detected.
    #[error(
        "{code}: non-finite quantum value detected at index {index}",
        code = .code()
    )]
    NonFiniteValue {
        /// Index at which the value was detected.
        index: u64,
    },

    /// A pure state was not normalized.
    #[error(
        "{code}: state is not normalized: norm={norm}, tolerance={tolerance}",
        code = .code()
    )]
    NotNormalized {
        /// Observed norm.
        norm: f64,

        /// Permitted tolerance.
        tolerance: f64,
    },

    /// A density matrix is not Hermitian.
    #[error(
        "{code}: density matrix is not Hermitian: maximum deviation={maximum_deviation}, tolerance={tolerance}",
        code = .code()
    )]
    NotHermitian {
        /// Maximum observed Hermiticity deviation.
        maximum_deviation: f64,

        /// Permitted tolerance.
        tolerance: f64,
    },

    /// Density-matrix trace is invalid.
    #[error(
        "{code}: invalid density-matrix trace: trace={trace}, tolerance={tolerance}",
        code = .code()
    )]
    InvalidTrace {
        /// Observed trace.
        trace: f64,

        /// Permitted tolerance.
        tolerance: f64,
    },

    /// Probability is invalid.
    #[error(
        "{code}: invalid probability {probability}: {reason}",
        code = .code()
    )]
    InvalidProbability {
        /// Probability value.
        probability: f64,

        /// Safe reason.
        reason: String,
    },

    /// A state representation invariant was violated.
    #[error(
        "{code}: quantum-state invariant violated: {reason}",
        code = .code()
    )]
    StateInvariantViolation {
        /// Safe reason.
        reason: String,
    },

    // -------------------------------------------------------------------------
    // Capability
    // -------------------------------------------------------------------------

    /// Requested representation is unavailable.
    #[error(
        "{code}: unsupported quantum-state representation `{representation}`",
        code = .code()
    )]
    UnsupportedRepresentation {
        /// Representation identifier.
        representation: String,
    },

    /// Conversion between representations is unavailable.
    #[error(
        "{code}: unsupported state conversion from `{source}` to `{destination}`",
        code = .code()
    )]
    UnsupportedConversion {
        /// Source representation.
        source: String,

        /// Destination representation.
        destination: String,
    },

    /// Requested memory operation is unavailable.
    #[error(
        "{code}: unsupported memory operation `{operation}`",
        code = .code()
    )]
    UnsupportedOperation {
        /// Operation identifier.
        operation: String,
    },

    /// Requested numerical precision is unavailable.
    #[error(
        "{code}: unsupported precision `{precision}`",
        code = .code()
    )]
    UnsupportedPrecision {
        /// Precision identifier.
        precision: String,
    },

    /// Requested storage location is unavailable.
    #[error(
        "{code}: unsupported storage location `{location}`",
        code = .code()
    )]
    UnsupportedStorageLocation {
        /// Storage-location identifier.
        location: String,
    },

    // -------------------------------------------------------------------------
    // Measurement / reset
    // -------------------------------------------------------------------------

    /// Measurement request failed validation or execution.
    #[error(
        "{code}: measurement error: {reason}",
        code = .code()
    )]
    MeasurementError {
        /// Safe reason.
        reason: String,
    },

    /// Measurement probabilities are inconsistent.
    #[error(
        "{code}: invalid measurement probabilities: {reason}",
        code = .code()
    )]
    MeasurementProbabilityError {
        /// Safe reason.
        reason: String,
    },

    /// Measurement collapse failed.
    #[error(
        "{code}: measurement collapse failed: {reason}",
        code = .code()
    )]
    CollapseError {
        /// Safe reason.
        reason: String,
    },

    /// Reset failed.
    #[error(
        "{code}: quantum reset failed: {reason}",
        code = .code()
    )]
    ResetError {
        /// Safe reason.
        reason: String,
    },

    // -------------------------------------------------------------------------
    // Persistence
    // -------------------------------------------------------------------------

    /// Serialization failed.
    #[error(
        "{code}: serialization failed for format `{format}`: {reason}",
        code = .code()
    )]
    SerializationError {
        /// Serialization format.
        format: String,

        /// Safe reason.
        reason: String,
    },

    /// Deserialization failed.
    #[error(
        "{code}: deserialization failed for format `{format}`: {reason}",
        code = .code()
    )]
    DeserializationError {
        /// Serialization format.
        format: String,

        /// Safe reason.
        reason: String,
    },

    /// Snapshot validation failed.
    #[error(
        "{code}: invalid snapshot: {reason}",
        code = .code()
    )]
    InvalidSnapshot {
        /// Safe reason.
        reason: String,
    },

    /// Checkpoint validation failed.
    #[error(
        "{code}: invalid checkpoint: {reason}",
        code = .code()
    )]
    InvalidCheckpoint {
        /// Safe reason.
        reason: String,
    },

    /// Persisted schema version is unsupported.
    #[error(
        "{code}: unsupported schema version for `{schema}`: version {version}",
        code = .code()
    )]
    UnsupportedSchemaVersion {
        /// Schema identifier.
        schema: String,

        /// Encountered version.
        version: u64,
    },

    /// Persisted data is corrupt or incomplete.
    #[error(
        "{code}: corrupt data: {reason}",
        code = .code()
    )]
    CorruptData {
        /// Safe reason.
        reason: String,
    },

    /// Integrity verification failed.
    #[error(
        "{code}: integrity verification failed for `{resource}`",
        code = .code()
    )]
    IntegrityError {
        /// Resource whose integrity failed.
        resource: String,
    },

    // -------------------------------------------------------------------------
    // Coherence / synchronization
    // -------------------------------------------------------------------------

    /// Memory coherence state is invalid.
    #[error(
        "{code}: memory coherence error: {reason}",
        code = .code()
    )]
    CoherenceError {
        /// Safe reason.
        reason: String,
    },

    /// Synchronization failed.
    #[error(
        "{code}: memory synchronization failed: {reason}",
        code = .code()
    )]
    SynchronizationError {
        /// Safe reason.
        reason: String,
    },

    /// Synchronization exceeded its allowed timeout.
    #[error(
        "{code}: memory synchronization timed out after {timeout_ms} ms",
        code = .code()
    )]
    SynchronizationTimeout {
        /// Timeout in milliseconds.
        timeout_ms: u64,
    },

    /// Concurrent operations conflicted.
    #[error(
        "{code}: concurrent memory operation conflict: {reason}",
        code = .code()
    )]
    ConcurrencyConflict {
        /// Safe reason.
        reason: String,
    },

    // -------------------------------------------------------------------------
    // Migration / lifecycle
    // -------------------------------------------------------------------------

    /// State migration failed.
    #[error(
        "{code}: memory migration failed from `{source}` to `{destination}`: {reason}",
        code = .code()
    )]
    MigrationError {
        /// Source representation/location.
        source: String,

        /// Destination representation/location.
        destination: String,

        /// Safe reason.
        reason: String,
    },

    /// Migration would exceed available resources.
    #[error(
        "{code}: migration resource requirement of {required_bytes} bytes exceeds available {available_bytes} bytes",
        code = .code()
    )]
    MigrationResourceExceeded {
        /// Required bytes.
        required_bytes: u64,

        /// Available bytes.
        available_bytes: u64,
    },

    /// Memory compaction failed.
    #[error(
        "{code}: memory compaction failed: {reason}",
        code = .code()
    )]
    CompactionError {
        /// Safe reason.
        reason: String,
    },

    // -------------------------------------------------------------------------
    // Backend / hardware
    // -------------------------------------------------------------------------

    /// Host-memory provider failed.
    #[error(
        "{code}: host-memory provider failed: {reason}",
        code = .code()
    )]
    HostMemoryError {
        /// Safe reason.
        reason: String,
    },

    /// Device/GPU memory provider failed.
    #[error(
        "{code}: device-memory provider failed: {reason}",
        code = .code()
    )]
    DeviceMemoryError {
        /// Safe reason.
        reason: String,
    },

    /// Distributed-memory provider failed.
    #[error(
        "{code}: distributed-memory provider failed: {reason}",
        code = .code()
    )]
    DistributedMemoryError {
        /// Safe reason.
        reason: String,
    },

    /// Backend-native memory operation failed.
    #[error(
        "{code}: backend memory operation failed for `{backend}`: {reason}",
        code = .code()
    )]
    BackendMemoryError {
        /// Backend identifier.
        backend: String,

        /// Safe reason.
        reason: String,
    },

    /// Backend rejected an otherwise structurally valid request.
    #[error(
        "{code}: backend `{backend}` rejected memory operation: {reason}",
        code = .code()
    )]
    BackendRejected {
        /// Backend identifier.
        backend: String,

        /// Safe reason.
        reason: String,
    },

    /// Backend lacks a requested memory capability.
    #[error(
        "{code}: backend `{backend}` does not provide capability `{capability}`",
        code = .code()
    )]
    BackendCapabilityUnavailable {
        /// Backend identifier.
        backend: String,

        /// Capability identifier.
        capability: String,
    },

    // -------------------------------------------------------------------------
    // Internal
    // -------------------------------------------------------------------------

    /// A required memory invariant was violated.
    ///
    /// This represents a programming/implementation failure rather than
    /// ordinary user input failure.
    #[error(
        "{code}: memory invariant violation: {reason}",
        code = .code()
    )]
    InvariantViolation {
        /// Safe reason.
        reason: String,
    },

    /// An internal implementation failure occurred.
    #[error(
        "{code}: internal memory error: {reason}",
        code = .code()
    )]
    InternalError {
        /// Safe reason.
        reason: String,
    },

    // -------------------------------------------------------------------------
    // Wrapped standard-library source errors
    // -------------------------------------------------------------------------

    /// Standard-library I/O operation failed.
    #[error("{code}: I/O error: {source}", code = .code(), source)]
    Io {
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// Integer conversion failed.
    #[error("{code}: integer conversion failed: {source}", code = .code(), source)]
    IntegerConversion {
        /// Underlying conversion error.
        #[source]
        source: TryFromIntError,
    },
}

// =============================================================================
// Error methods
// =============================================================================

impl MemoryError {
    /// Returns the stable machine-readable error code.
    pub const fn code(&self) -> MemoryErrorCode {
        match self {
            Self::InvalidArgument { .. } => MemoryErrorCode::InvalidArgument,
            Self::InvalidIdentifier { .. } => MemoryErrorCode::InvalidIdentifier,
            Self::OutOfBounds { .. } => MemoryErrorCode::OutOfBounds,
            Self::ArithmeticOverflow { .. } => MemoryErrorCode::ArithmeticOverflow,

            Self::AllocationFailed { .. } => MemoryErrorCode::AllocationFailed,
            Self::BudgetExceeded { .. } => MemoryErrorCode::BudgetExceeded,
            Self::MemoryLimitExceeded { .. } => MemoryErrorCode::MemoryLimitExceeded,
            Self::AllocationCountExceeded { .. } => {
                MemoryErrorCode::AllocationCountExceeded
            }
            Self::ReservationFailed { .. } => MemoryErrorCode::ReservationFailed,
            Self::PoolError { .. } => MemoryErrorCode::PoolError,
            Self::InvalidAddress { .. } => MemoryErrorCode::InvalidAddress,

            Self::InvalidLayout { .. } => MemoryErrorCode::InvalidLayout,
            Self::InvalidShape { .. } => MemoryErrorCode::InvalidShape,
            Self::InvalidDimension { .. } => MemoryErrorCode::InvalidDimension,
            Self::InvalidPermutation { .. } => MemoryErrorCode::InvalidPermutation,
            Self::InvalidIndex { .. } => MemoryErrorCode::InvalidIndex,
            Self::InvalidView { .. } => MemoryErrorCode::InvalidView,
            Self::InvalidSlice { .. } => MemoryErrorCode::InvalidSlice,

            Self::AliasingViolation { .. } => MemoryErrorCode::AliasingViolation,
            Self::LifetimeViolation { .. } => MemoryErrorCode::LifetimeViolation,
            Self::UseAfterRelease { .. } => MemoryErrorCode::UseAfterRelease,
            Self::OwnershipViolation { .. } => MemoryErrorCode::OwnershipViolation,
            Self::LogicalMemoryError { .. } => MemoryErrorCode::LogicalMemoryError,
            Self::ClassicalMemoryError { .. } => {
                MemoryErrorCode::ClassicalMemoryError
            }

            Self::StateDimensionMismatch { .. } => {
                MemoryErrorCode::StateDimensionMismatch
            }
            Self::InvalidState { .. } => MemoryErrorCode::InvalidState,
            Self::NonFiniteValue { .. } => MemoryErrorCode::NonFiniteValue,
            Self::NotNormalized { .. } => MemoryErrorCode::NotNormalized,
            Self::NotHermitian { .. } => MemoryErrorCode::NotHermitian,
            Self::InvalidTrace { .. } => MemoryErrorCode::InvalidTrace,
            Self::InvalidProbability { .. } => MemoryErrorCode::InvalidProbability,
            Self::StateInvariantViolation { .. } => {
                MemoryErrorCode::StateInvariantViolation
            }

            Self::UnsupportedRepresentation { .. } => {
                MemoryErrorCode::UnsupportedRepresentation
            }
            Self::UnsupportedConversion { .. } => MemoryErrorCode::UnsupportedConversion,
            Self::UnsupportedOperation { .. } => MemoryErrorCode::UnsupportedOperation,
            Self::UnsupportedPrecision { .. } => MemoryErrorCode::UnsupportedPrecision,
            Self::UnsupportedStorageLocation { .. } => {
                MemoryErrorCode::UnsupportedStorageLocation
            }

            Self::MeasurementError { .. } => MemoryErrorCode::MeasurementError,
            Self::MeasurementProbabilityError { .. } => {
                MemoryErrorCode::MeasurementProbabilityError
            }
            Self::CollapseError { .. } => MemoryErrorCode::CollapseError,
            Self::ResetError { .. } => MemoryErrorCode::ResetError,

            Self::SerializationError { .. } => MemoryErrorCode::SerializationError,
            Self::DeserializationError { .. } => MemoryErrorCode::DeserializationError,
            Self::InvalidSnapshot { .. } => MemoryErrorCode::InvalidSnapshot,
            Self::InvalidCheckpoint { .. } => MemoryErrorCode::InvalidCheckpoint,
            Self::UnsupportedSchemaVersion { .. } => {
                MemoryErrorCode::UnsupportedSchemaVersion
            }
            Self::CorruptData { .. } => MemoryErrorCode::CorruptData,
            Self::IntegrityError { .. } => MemoryErrorCode::IntegrityError,

            Self::CoherenceError { .. } => MemoryErrorCode::CoherenceError,
            Self::SynchronizationError { .. } => {
                MemoryErrorCode::SynchronizationError
            }
            Self::SynchronizationTimeout { .. } => {
                MemoryErrorCode::SynchronizationTimeout
            }
            Self::ConcurrencyConflict { .. } => {
                MemoryErrorCode::ConcurrencyConflict
            }

            Self::MigrationError { .. } => MemoryErrorCode::MigrationError,
            Self::MigrationResourceExceeded { .. } => {
                MemoryErrorCode::MigrationResourceExceeded
            }
            Self::CompactionError { .. } => MemoryErrorCode::CompactionError,

            Self::HostMemoryError { .. } => MemoryErrorCode::HostMemoryError,
            Self::DeviceMemoryError { .. } => MemoryErrorCode::DeviceMemoryError,
            Self::DistributedMemoryError { .. } => {
                MemoryErrorCode::DistributedMemoryError
            }
            Self::BackendMemoryError { .. } => MemoryErrorCode::BackendMemoryError,
            Self::BackendRejected { .. } => MemoryErrorCode::BackendRejected,
            Self::BackendCapabilityUnavailable { .. } => {
                MemoryErrorCode::BackendCapabilityUnavailable
            }

            Self::InvariantViolation { .. } => MemoryErrorCode::InvariantViolation,
            Self::InternalError { .. } => MemoryErrorCode::InternalError,

            Self::Io { .. } => MemoryErrorCode::SerializationError,
            Self::IntegerConversion { .. } => MemoryErrorCode::ArithmeticOverflow,
        }
    }

    /// Returns the broad error category.
    pub const fn category(&self) -> MemoryErrorCategory {
        self.code().category()
    }

    /// Returns true when the error represents invalid caller input.
    pub const fn is_validation(&self) -> bool {
        self.code().is_validation()
    }

    /// Returns true when retrying may be meaningful after environmental state
    /// changes.
    pub const fn is_potentially_retryable(&self) -> bool {
        self.code().is_potentially_retryable()
    }

    /// Returns the stable schema identifier.
    pub const fn schema_id() -> &'static str {
        MEMORY_ERROR_SCHEMA_ID
    }

    /// Returns the current error-schema version.
    pub const fn schema_version() -> u16 {
        MEMORY_ERROR_SCHEMA_VERSION
    }

    /// Attaches safe diagnostic context to errors that support context.
    ///
    /// This method deliberately returns the original error unchanged for
    /// variants where adding context would make the public structure less
    /// useful. Callers may instead use their own higher-level diagnostic
    /// wrapper.
    pub fn with_context(self, context: ErrorContext) -> Self {
        if context.is_empty() {
            return self;
        }

        match self {
            Self::InvalidArgument { argument, .. } => Self::InvalidArgument {
                argument,
                context: Some(context),
            },

            Self::InvalidIdentifier { identifier, .. } => {
                Self::InvalidIdentifier {
                    identifier,
                    context: Some(context),
                }
            }

            other => other,
        }
    }

    /// Creates an invalid-argument error.
    pub fn invalid_argument(argument: impl Into<String>) -> Self {
        Self::InvalidArgument {
            argument: argument.into(),
            context: None,
        }
    }

    /// Creates an invalid-identifier error.
    pub fn invalid_identifier(identifier: impl Into<String>) -> Self {
        Self::InvalidIdentifier {
            identifier: identifier.into(),
            context: None,
        }
    }

    /// Creates an arithmetic-overflow error.
    pub fn arithmetic_overflow(operation: impl Into<String>) -> Self {
        Self::ArithmeticOverflow {
            operation: operation.into(),
        }
    }

    /// Creates an unsupported-operation error.
    pub fn unsupported_operation(operation: impl Into<String>) -> Self {
        Self::UnsupportedOperation {
            operation: operation.into(),
        }
    }

    /// Creates an invariant-violation error.
    pub fn invariant_violation(reason: impl Into<String>) -> Self {
        Self::InvariantViolation {
            reason: reason.into(),
        }
    }

    /// Creates an internal-error value.
    ///
    /// Use this for implementation failures that are not expected to be
    /// recoverable by the caller.
    pub fn internal(reason: impl Into<String>) -> Self {
        Self::InternalError {
            reason: reason.into(),
        }
    }
}

// =============================================================================
// Conversions
// =============================================================================

impl From<TryFromIntError> for MemoryError {
    fn from(source: TryFromIntError) -> Self {
        Self::IntegerConversion { source }
    }
}

impl From<std::io::Error> for MemoryError {
    fn from(source: std::io::Error) -> Self {
        Self::Io { source }
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates a bounded identifier/context string.
///
/// Empty strings are rejected because an empty resource identifier is
/// ambiguous and makes diagnostics unreliable.
pub fn validate_text(
    field: impl Into<String>,
    value: impl Into<String>,
    maximum_length: usize,
) -> Result<String, MemoryError> {
    let field = field.into();
    let value = value.into();

    validate_context(&field, value, maximum_length)
}

/// Validates a general diagnostic/context string.
fn validate_context(
    field: &str,
    value: String,
    maximum_length: usize,
) -> Result<String, MemoryError> {
    if value.is_empty() {
        return Err(MemoryError::InvalidArgument {
            argument: field.to_owned(),
            context: Some(
                ErrorContext::new()
                    .with_detail("value must not be empty")
                    .unwrap_or_default(),
            ),
        });
    }

    if value.len() > maximum_length {
        return Err(MemoryError::InvalidArgument {
            argument: field.to_owned(),
            context: Some(
                ErrorContext::new()
                    .with_detail(format!(
                        "value exceeds maximum length of {maximum_length} bytes"
                    ))
                    .unwrap_or_default(),
            ),
        });
    }

    if value.contains('\0') {
        return Err(MemoryError::InvalidArgument {
            argument: field.to_owned(),
            context: Some(
                ErrorContext::new()
                    .with_detail("value must not contain a NUL character")
                    .unwrap_or_default(),
            ),
        });
    }

    Ok(value)
}

/// Validates an identifier used for logical resources.
pub fn validate_identifier(
    identifier: impl Into<String>,
    field: impl Into<String>,
    maximum_length: usize,
) -> Result<String, MemoryError> {
    let field = field.into();
    let identifier = identifier.into();

    validate_context(&field, identifier, maximum_length)
}

// =============================================================================
// Context formatting
// =============================================================================

fn format_context(context: Option<&ErrorContext>) -> String {
    let Some(context) = context else {
        return String::new();
    };

    let mut result = String::new();

    if let Some(operation) = context.operation() {
        result.push_str("; operation=");
        result.push_str(operation);
    }

    if let Some(resource) = context.resource() {
        result.push_str("; resource=");
        result.push_str(resource);
    }

    if let Some(detail) = context.detail() {
        result.push_str("; detail=");
        result.push_str(detail);
    }

    result
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes_are_stable_and_non_empty() {
        let codes = [
            MemoryErrorCode::InvalidArgument,
            MemoryErrorCode::AllocationFailed,
            MemoryErrorCode::MemoryLimitExceeded,
            MemoryErrorCode::InvalidLayout,
            MemoryErrorCode::AliasingViolation,
            MemoryErrorCode::StateDimensionMismatch,
            MemoryErrorCode::InvalidState,
            MemoryErrorCode::UnsupportedRepresentation,
            MemoryErrorCode::MeasurementError,
            MemoryErrorCode::SerializationError,
            MemoryErrorCode::SynchronizationError,
            MemoryErrorCode::MigrationError,
            MemoryErrorCode::BackendMemoryError,
            MemoryErrorCode::InvariantViolation,
        ];

        for code in codes {
            assert!(!code.as_str().is_empty());
            assert!(code.as_str().starts_with("QM"));
            assert!(code.as_u16() > 0);
        }
    }

    #[test]
    fn categories_are_consistent() {
        assert_eq!(
            MemoryErrorCode::AllocationFailed.category(),
            MemoryErrorCategory::Allocation
        );

        assert_eq!(
            MemoryErrorCode::InvalidLayout.category(),
            MemoryErrorCategory::Layout
        );

        assert_eq!(
            MemoryErrorCode::InvalidState.category(),
            MemoryErrorCategory::QuantumState
        );

        assert_eq!(
            MemoryErrorCode::SerializationError.category(),
            MemoryErrorCategory::Persistence
        );

        assert_eq!(
            MemoryErrorCode::BackendMemoryError.category(),
            MemoryErrorCategory::Backend
        );
    }

    #[test]
    fn invalid_argument_has_stable_code() {
        let error = MemoryError::invalid_argument("qubit_count");

        assert_eq!(
            error.code(),
            MemoryErrorCode::InvalidArgument
        );

        assert_eq!(
            error.category(),
            MemoryErrorCategory::Validation
        );
    }

    #[test]
    fn identifier_validation_rejects_empty_values() {
        let result = validate_identifier(
            "",
            "qubit_id",
            128,
        );

        assert!(result.is_err());

        let error = result.expect_err("empty identifiers must be rejected");

        assert_eq!(
            error.code(),
            MemoryErrorCode::InvalidArgument
        );
    }

    #[test]
    fn identifier_validation_rejects_nul() {
        let result = validate_identifier(
            "q\0bit",
            "qubit_id",
            128,
        );

        assert!(result.is_err());

        let error = result.expect_err("NUL-containing identifiers must be rejected");

        assert_eq!(
            error.code(),
            MemoryErrorCode::InvalidArgument
        );
    }

    #[test]
    fn identifier_validation_rejects_oversized_values() {
        let value = "x".repeat(129);

        let result = validate_identifier(
            value,
            "qubit_id",
            128,
        );

        assert!(result.is_err());

        let error = result.expect_err("oversized identifiers must be rejected");

        assert_eq!(
            error.code(),
            MemoryErrorCode::InvalidArgument
        );
    }

    #[test]
    fn context_can_be_added() {
        let context = ErrorContext::new()
            .with_operation("allocate_state")
            .expect("valid operation")
            .with_resource("state_vector")
            .expect("valid resource");

        let error = MemoryError::invalid_argument("qubit_count")
            .with_context(context);

        let rendered = error.to_string();

        assert!(rendered.contains("allocate_state"));
        assert!(rendered.contains("state_vector"));
        assert!(rendered.contains("QM001"));
    }

    #[test]
    fn context_rejects_empty_operation() {
        let result = ErrorContext::new()
            .with_operation("");

        assert!(result.is_err());
    }

    #[test]
    fn context_rejects_nul() {
        let result = ErrorContext::new()
            .with_detail("bad\0value");

        assert!(result.is_err());
    }

    #[test]
    fn context_rejects_excessive_detail() {
        let detail = "x".repeat(MAX_CONTEXT_LENGTH + 1);

        let result = ErrorContext::new()
            .with_detail(detail);

        assert!(result.is_err());
    }

    #[test]
    fn display_contains_machine_code() {
        let error = MemoryError::MemoryLimitExceeded {
            limit: "host_bytes".to_owned(),
            requested_bytes: 2_048,
            maximum_bytes: 1_024,
        };

        let text = error.to_string();

        assert!(text.starts_with("QM012"));
        assert!(text.contains("host_bytes"));
        assert!(text.contains("2048"));
        assert!(text.contains("1024"));
    }

    #[test]
    fn retryability_is_conservative() {
        assert!(
            MemoryErrorCode::AllocationFailed.is_potentially_retryable()
        );

        assert!(
            MemoryErrorCode::SynchronizationTimeout
                .is_potentially_retryable()
        );

        assert!(
            !MemoryErrorCode::InvalidState.is_potentially_retryable()
        );

        assert!(
            !MemoryErrorCode::InvalidLayout.is_potentially_retryable()
        );
    }

    #[test]
    fn source_errors_are_preserved() {
        let io_error = std::io::Error::other("test failure");
        let memory_error = MemoryError::from(io_error);

        assert_eq!(
            memory_error.code(),
            MemoryErrorCode::SerializationError
        );

        assert!(memory_error.source().is_some());
    }

    #[test]
    fn integer_conversion_is_mapped() {
        let conversion = u8::try_from(1_000_u16)
            .expect_err("conversion should fail");

        let memory_error = MemoryError::from(conversion);

        assert_eq!(
            memory_error.code(),
            MemoryErrorCode::ArithmeticOverflow
        );
    }

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            MemoryError::schema_id(),
            "zamani.quantum.memory.error"
        );

        assert_eq!(
            MemoryError::schema_version(),
            1
        );
    }

    #[test]
    fn empty_context_is_detected() {
        assert!(ErrorContext::new().is_empty());

        let context = ErrorContext::new()
            .with_detail("detail")
            .expect("valid detail");

        assert!(!context.is_empty());
    }

    #[test]
    fn context_is_not_added_to_unrelated_error_variants() {
        let context = ErrorContext::new()
            .with_operation("measure")
            .expect("valid operation");

        let error = MemoryError::InvalidState {
            reason: "test".to_owned(),
        };

        let updated = error.with_context(context);

        assert_eq!(
            updated.code(),
            MemoryErrorCode::InvalidState
        );
    }

    #[test]
    fn no_context_has_clean_display() {
        let error = MemoryError::InvalidState {
            reason: "state is invalid".to_owned(),
        };

        assert_eq!(
            error.to_string(),
            "QM041: invalid quantum state: state is invalid"
        );
    }
}