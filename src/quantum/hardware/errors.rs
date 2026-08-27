//! Zamani Quantum — Canonical Hardware Error System
//!
//! Production-grade, provider-neutral error taxonomy for
//! `crate::quantum::hardware`.
//!
//! # Responsibility
//!
//! This module defines the canonical error vocabulary for the quantum
//! hardware abstraction layer.
//!
//! It owns:
//!
//! - stable hardware error categories;
//! - stable machine-readable error codes;
//! - severity classification;
//! - retryability classification;
//! - error-source classification;
//! - safe human-readable diagnostics;
//! - provider-neutral contextual information;
//! - topology-error conversion;
//! - validation/execution/provider/transport error boundaries;
//! - safe error chaining metadata;
//! - deterministic error formatting;
//! - redaction rules for sensitive diagnostic context;
//! - compatibility information required by higher-level hardware modules.
//!
//! It deliberately does NOT own:
//!
//! - provider SDK errors;
//! - provider HTTP clients;
//! - authentication;
//! - credentials;
//! - API tokens;
//! - secrets;
//! - backend metadata;
//! - workload definitions;
//! - validation algorithms;
//! - execution algorithms;
//! - retry loops;
//! - job management;
//! - queue management;
//! - provider selection;
//! - benchmarking;
//! - routing;
//! - scheduling;
//! - QEC algorithms.
//!
//! Those systems consume this error contract.
//!
//! # Architectural position
//!
//! ```text
//! provider SDK / transport
//!          |
//!          v
//! provider adapter
//!          |
//!          v
//! hardware::errors
//!          |
//!          +--------------------+
//!          |                    |
//!          v                    v
//! hardware::validation     hardware::execution
//!          |                    |
//!          +---------+----------+
//!                    |
//!                    v
//!              hardware API
//!                    |
//!          +---------+----------+
//!          |                    |
//!          v                    v
//!       Danga             benchmarking
//! ```
//!
//! Hardware errors are lower-level than benchmarking and Danga.
//!
//! `quantum::hardware` MUST NOT depend on `quantum::benchmarking`.
//!
//! # Design goals
//!
//! The error system is designed around six production requirements:
//!
//! 1. **Stable codes**
//!
//!    Applications must not parse human-readable messages.
//!
//! 2. **Provider independence**
//!
//!    IBM, IonQ, AWS Braket, Rigetti, IQM, Quantinuum, QuEra and future
//!    adapters must map their native errors into this vocabulary.
//!
//! 3. **Retry classification**
//!
//!    A caller must be able to determine whether retrying is potentially
//!    meaningful without parsing a message.
//!
//! 4. **Security**
//!
//!    Error messages and context must never require credentials or secret
//!    material.
//!
//! 5. **Determinism**
//!
//!    Formatting and classification must not depend on provider/network state.
//!
//! 6. **Extensibility**
//!
//!    Adding a provider must not require adding provider-specific variants to
//!    this module.
//!
//! # Important architectural rule
//!
//! Provider-specific information belongs in:
//!
//! ```text
//! provider adapter -> provider_code / provider_category / safe_context
//! ```
//!
//! It MUST NOT result in variants such as:
//!
//! ```text
//! IbmError
//! IonqError
//! BraketError
//! ```
//!
//! in this core module.
//!
//! # Migration contract
//!
//! The current `backend.rs` contains a `BackendError` enum. That enum is the
//! legacy location of the backend error taxonomy.
//!
//! The intended final architecture is:
//!
//! ```text
//! hardware/errors.rs
//!        |
//!        v
//! canonical HardwareError
//!        |
//!        +---- backend.rs
//!        +---- validation.rs
//!        +---- execution.rs
//!        +---- backend_trait.rs
//!        +---- job.rs
//!        +---- queue.rs
//!        +---- adapters/*
//! ```
//!
//! During migration, `backend.rs` may temporarily retain its compatibility
//! `BackendError` surface and delegate/conversion implementations to this
//! module. The canonical definitions in this file deliberately avoid importing
//! `backend.rs`, preventing a dependency cycle.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no unsafe Rust.
//!
//! # Safety
//!
//! ```text
//! #![deny(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! #![deny(unused_must_use)]
//! ```
//!
//! No unsafe operations are required.
//!
//! # Serialization
//!
//! This module intentionally does not depend directly on Serde. Serialization
//! belongs to `hardware/serialization.rs`.
//!
//! The following fields are stable serialization primitives:
//!
//! - schema ID;
//! - schema version;
//! - error code;
//! - category;
//! - severity;
//! - retryability;
//! - source;
//! - message;
//! - provider code, when safe;
//! - backend/job/request identifiers, when safe;
//! - context fields.
//!
//! `serialization.rs` may derive/implement its serialized representation
//! without changing the semantic error contract.
//!
//! # No-secret invariant
//!
//! This module MUST NOT contain:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - cookies;
//! - authorization headers;
//! - credential bodies;
//! - raw authentication responses.
//!
//! Provider adapters are responsible for stripping such data before creating
//! an error.
//!
//! -----------------------------------------------------------------------------
//! Schema
//! -----------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::error::Error;
use std::fmt;

use super::topology::TopologyError;

/// Stable schema identifier for the canonical hardware error model.
pub const HARDWARE_ERROR_SCHEMA_ID: &str =
    "zamani.quantum.hardware.errors";

/// Semantic version of the canonical hardware error model.
pub const HARDWARE_ERROR_SCHEMA_VERSION: u16 = 1;

/// Maximum provider error-code length.
pub const MAX_PROVIDER_ERROR_CODE_LENGTH: usize = 256;

/// Maximum diagnostic message length.
pub const MAX_ERROR_MESSAGE_LENGTH: usize = 4096;

/// Maximum error context key length.
pub const MAX_ERROR_CONTEXT_KEY_LENGTH: usize = 256;

/// Maximum error context value length.
pub const MAX_ERROR_CONTEXT_VALUE_LENGTH: usize = 4096;

/// Maximum number of structured context fields.
pub const MAX_ERROR_CONTEXT_FIELDS: usize = 64;

/// Maximum backend identifier length accepted in error context.
pub const MAX_ERROR_BACKEND_ID_LENGTH: usize = 512;

/// Maximum job identifier length accepted in error context.
pub const MAX_ERROR_JOB_ID_LENGTH: usize = 1024;

/// Maximum request identifier length accepted in error context.
pub const MAX_ERROR_REQUEST_ID_LENGTH: usize = 512;

// =============================================================================
// Error category
// =============================================================================

/// High-level, provider-neutral category of a hardware error.
///
/// Categories are deliberately coarse enough to remain stable across
/// providers while still allowing automation and observability systems to
/// classify failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HardwareErrorCategory {
    /// Backend identity or descriptor is invalid.
    Identity,

    /// Backend capability negotiation failed.
    Capability,

    /// Hardware resource constraints were violated.
    Resource,

    /// Hardware topology is invalid or insufficient.
    Topology,

    /// Calibration data is unavailable, invalid or stale.
    Calibration,

    /// Hardware timing requirements could not be satisfied.
    Timing,

    /// Workload is malformed or semantically invalid.
    Validation,

    /// Execution request could not be accepted.
    Submission,

    /// Job execution failed after acceptance.
    Execution,

    /// Queue operation failed.
    Queue,

    /// Result retrieval or normalization failed.
    Result,

    /// Cancellation failed or was rejected.
    Cancellation,

    /// Provider authentication failed.
    Authentication,

    /// Caller lacks permission.
    Authorization,

    /// Network/transport operation failed.
    Transport,

    /// Serialization/deserialization failed.
    Serialization,

    /// Provider-specific operation failed after normalization.
    Provider,

    /// Operation exceeded its deadline.
    Timeout,

    /// Internal invariant was violated.
    Internal,

    /// Requested operation is not implemented.
    Unsupported,
}

impl HardwareErrorCategory {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::Capability => "capability",
            Self::Resource => "resource",
            Self::Topology => "topology",
            Self::Calibration => "calibration",
            Self::Timing => "timing",
            Self::Validation => "validation",
            Self::Submission => "submission",
            Self::Execution => "execution",
            Self::Queue => "queue",
            Self::Result => "result",
            Self::Cancellation => "cancellation",
            Self::Authentication => "authentication",
            Self::Authorization => "authorization",
            Self::Transport => "transport",
            Self::Serialization => "serialization",
            Self::Provider => "provider",
            Self::Timeout => "timeout",
            Self::Internal => "internal",
            Self::Unsupported => "unsupported",
        }
    }
}

impl fmt::Display for HardwareErrorCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Error severity
// =============================================================================

/// Severity of a hardware error.
///
/// Severity is independent of retryability.
///
/// For example, a transport failure may be severe enough to abort the current
/// execution but still be retryable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HardwareErrorSeverity {
    /// Informational condition.
    Info,

    /// Recoverable warning-like condition.
    Warning,

    /// Operation failed but the process may continue.
    Error,

    /// System invariant or execution safety boundary was violated.
    Critical,
}

impl HardwareErrorSeverity {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Critical => "critical",
        }
    }

    /// Returns whether this severity represents an unsuccessful operation.
    pub const fn is_failure(self) -> bool {
        matches!(self, Self::Error | Self::Critical)
    }
}

impl fmt::Display for HardwareErrorSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Retryability
// =============================================================================

/// Whether an operation may be retried.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Retryability {
    /// Retrying the same request is not expected to help.
    Never,

    /// Retrying may help after external state changes.
    Conditional,

    /// Retrying is normally appropriate according to the caller's retry
    /// policy.
    Recommended,

    /// The retryability cannot safely be determined.
    Unknown,
}

impl Retryability {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Never => "never",
            Self::Conditional => "conditional",
            Self::Recommended => "recommended",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether automatic retry may be considered.
    pub const fn may_retry(self) -> bool {
        matches!(self, Self::Conditional | Self::Recommended)
    }
}

impl fmt::Display for Retryability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Error source
// =============================================================================

/// Origin of the normalized error.
///
/// This is deliberately independent of provider names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HardwareErrorSource {
    /// Error originated from Zamani validation/model logic.
    Zamani,

    /// Error originated from the provider adapter boundary.
    ProviderAdapter,

    /// Error originated from a remote provider.
    Provider,

    /// Error originated from network/transport.
    Transport,

    /// Error originated from a local simulator/emulator.
    LocalExecution,

    /// Error originated from hardware itself.
    Hardware,

    /// Error was converted from another canonical hardware subsystem.
    Subsystem,

    /// Error origin could not be established.
    Unknown,
}

impl HardwareErrorSource {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Zamani => "zamani",
            Self::ProviderAdapter => "provider_adapter",
            Self::Provider => "provider",
            Self::Transport => "transport",
            Self::LocalExecution => "local_execution",
            Self::Hardware => "hardware",
            Self::Subsystem => "subsystem",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for HardwareErrorSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Stable error code
// =============================================================================

/// Stable machine-readable hardware error code.
///
/// Codes are deliberately represented as an enum rather than requiring
/// consumers to parse strings.
///
/// Provider-specific errors use `HardwareError::provider_code` instead of
/// expanding this enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HardwareErrorCode {
    // -------------------------------------------------------------------------
    // Identity
    // -------------------------------------------------------------------------

    /// Backend identifier is invalid.
    InvalidBackendId,

    /// Generic identifier is invalid.
    InvalidIdentifier,

    /// Identifier exceeded the allowed length.
    IdentifierTooLong,

    /// Backend descriptor is invalid.
    InvalidBackendDescriptor,

    // -------------------------------------------------------------------------
    // Metadata/security
    // -------------------------------------------------------------------------

    /// Metadata field is invalid.
    InvalidMetadata,

    /// Metadata collection exceeded its configured bound.
    MetadataLimitExceeded,

    /// Metadata appears to contain secret material.
    SecretLikeMetadata,

    /// Error context contains forbidden sensitive material.
    SensitiveErrorContext,

    // -------------------------------------------------------------------------
    // Resource/workload
    // -------------------------------------------------------------------------

    /// Requested shot count is zero or otherwise invalid.
    InvalidShots,

    /// Workload requires at least one quantum resource.
    ZeroQubits,

    /// Requested physical resource count exceeds backend capacity.
    QubitLimitExceeded,

    /// Requested logical resource count exceeds backend capacity.
    LogicalQubitLimitExceeded,

    /// Circuit depth exceeds backend limit.
    CircuitDepthExceeded,

    /// Operation count exceeds backend limit.
    OperationLimitExceeded,

    /// Shot count exceeds backend limit.
    ShotLimitExceeded,

    /// Classical-bit count exceeds backend limit.
    ClassicalBitLimitExceeded,

    /// Workload is malformed.
    InvalidWorkload,

    /// Declared and inferred workload categories disagree.
    InconsistentWorkloadKind,

    // -------------------------------------------------------------------------
    // Capability
    // -------------------------------------------------------------------------

    /// Backend does not support the requested workload.
    UnsupportedWorkload,

    /// Required capability is unavailable.
    UnsupportedCapability,

    /// Required capability is experimental and was not accepted.
    ExperimentalCapabilityNotAccepted,

    /// Required instruction/gate is unavailable.
    UnsupportedInstruction,

    /// Required gate is unavailable.
    UnsupportedGate,

    /// Backend does not support measurement.
    MeasurementUnsupported,

    /// Backend does not support reset.
    ResetUnsupported,

    /// Backend does not support mid-circuit measurement.
    MidCircuitMeasurementUnsupported,

    /// Backend does not support classical control/feed-forward.
    ClassicalControlUnsupported,

    /// Backend does not support dynamic circuits.
    DynamicCircuitUnsupported,

    /// Backend does not support pulse execution.
    PulseControlUnsupported,

    /// Backend does not support analog execution.
    AnalogControlUnsupported,

    /// Backend does not support annealing.
    AnnealingUnsupported,

    /// Backend does not expose logical qubits.
    LogicalQubitsUnsupported,

    /// Backend does not support fault-tolerant execution.
    FaultToleranceUnsupported,

    /// Backend does not support deterministic seeding.
    DeterministicSeedingUnsupported,

    /// Backend does not expose state-vector results.
    StateVectorUnsupported,

    /// Backend does not expose density-matrix results.
    DensityMatrixUnsupported,

    /// Backend does not expose expectation-value results.
    ExpectationValuesUnsupported,

    /// Backend does not expose a native instruction set.
    NativeInstructionSetUnavailable,

    // -------------------------------------------------------------------------
    // Topology
    // -------------------------------------------------------------------------

    /// Requested qubit/resource does not exist.
    InvalidQubit,

    /// Requested connection does not exist.
    UnsupportedConnection,

    /// Topology data is unavailable.
    TopologyUnavailable,

    /// Topology violates an invariant.
    InvalidTopology,

    /// Required topology edge count exceeds the safety bound.
    RequiredTopologyEdgeLimitExceeded,

    /// Required instruction count exceeds the safety bound.
    RequiredInstructionLimitExceeded,

    // -------------------------------------------------------------------------
    // Calibration
    // -------------------------------------------------------------------------

    /// Calibration data is unavailable.
    CalibrationUnavailable,

    /// Fresh calibration evidence is required.
    FreshCalibrationRequired,

    /// Calibration data is malformed.
    InvalidCalibration,

    /// Calibration data is stale.
    StaleCalibration,

    // -------------------------------------------------------------------------
    // Timing
    // -------------------------------------------------------------------------

    /// Hardware timing requirement is invalid.
    InvalidTiming,

    /// Requested timing cannot be represented by the backend.
    TimingConstraintViolation,

    // -------------------------------------------------------------------------
    // Backend availability
    // -------------------------------------------------------------------------

    /// Backend is unavailable.
    BackendUnavailable,

    /// Backend has been retired.
    BackendRetired,

    /// Backend is undergoing maintenance.
    BackendMaintenance,

    // -------------------------------------------------------------------------
    // Submission/execution
    // -------------------------------------------------------------------------

    /// Execution is not available.
    ExecutionUnavailable,

    /// Execution was explicitly rejected.
    ExecutionRejected,

    /// Execution request was malformed.
    InvalidExecutionRequest,

    /// Execution request was submitted but could not be accepted.
    SubmissionRejected,

    /// Provider submission failed.
    SubmissionFailed,

    /// Execution failed after submission.
    ExecutionFailed,

    /// Operation timed out.
    Timeout,

    // -------------------------------------------------------------------------
    // Jobs/queues
    // -------------------------------------------------------------------------

    /// Job identifier is invalid.
    InvalidJobId,

    /// Job state transition is invalid.
    InvalidJobStateTransition,

    /// Queue is unavailable.
    QueueUnavailable,

    /// Job was cancelled.
    JobCancelled,

    /// Job cancellation was rejected.
    CancellationRejected,

    /// Cancellation operation failed.
    CancellationFailed,

    /// Job expired.
    JobExpired,

    // -------------------------------------------------------------------------
    // Results
    // -------------------------------------------------------------------------

    /// Result is unavailable.
    ResultUnavailable,

    /// Result could not be decoded.
    ResultDecodingFailed,

    /// Result structure is invalid.
    InvalidResult,

    /// Result count arithmetic overflowed.
    ResultCountOverflow,

    /// Result represents more shots than requested.
    ResultShotsExceeded,

    /// Result bitstring is invalid.
    InvalidBitstring,

    // -------------------------------------------------------------------------
    // Authentication/authorization
    // -------------------------------------------------------------------------

    /// Authentication failed.
    AuthenticationFailed,

    /// Credentials are unavailable.
    CredentialsUnavailable,

    /// Caller is not authorized.
    AuthorizationDenied,

    /// Provider account/project is invalid or inaccessible.
    AccountUnavailable,

    // -------------------------------------------------------------------------
    // Transport/provider
    // -------------------------------------------------------------------------

    /// Network/transport operation failed.
    TransportFailed,

    /// Provider returned an unmappable error.
    ProviderError,

    /// Provider returned an unsupported response.
    ProviderResponseInvalid,

    /// Provider API version is incompatible.
    ProviderApiIncompatible,

    // -------------------------------------------------------------------------
    // Serialization
    // -------------------------------------------------------------------------

    /// Serialization failed.
    SerializationFailed,

    /// Deserialization failed.
    DeserializationFailed,

    /// Serialized schema is unsupported.
    UnsupportedSchemaVersion,

    // -------------------------------------------------------------------------
    // Internal/unsupported
    // -------------------------------------------------------------------------

    /// Internal invariant was violated.
    InternalInvariantViolation,

    /// Requested operation is not implemented.
    NotImplemented,
}

impl HardwareErrorCode {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidBackendId => "hardware.backend.invalid_id",
            Self::InvalidIdentifier => "hardware.identifier.invalid",
            Self::IdentifierTooLong => "hardware.identifier.too_long",
            Self::InvalidBackendDescriptor => {
                "hardware.backend.invalid_descriptor"
            }

            Self::InvalidMetadata => "hardware.metadata.invalid",
            Self::MetadataLimitExceeded => {
                "hardware.metadata.limit_exceeded"
            }
            Self::SecretLikeMetadata => "hardware.metadata.secret_like",
            Self::SensitiveErrorContext => {
                "hardware.error_context.sensitive"
            }

            Self::InvalidShots => "hardware.workload.shots.invalid",
            Self::ZeroQubits => "hardware.workload.qubits.zero",
            Self::QubitLimitExceeded => {
                "hardware.resource.qubits.limit_exceeded"
            }
            Self::LogicalQubitLimitExceeded => {
                "hardware.resource.logical_qubits.limit_exceeded"
            }
            Self::CircuitDepthExceeded => {
                "hardware.resource.depth.limit_exceeded"
            }
            Self::OperationLimitExceeded => {
                "hardware.resource.operations.limit_exceeded"
            }
            Self::ShotLimitExceeded => {
                "hardware.resource.shots.limit_exceeded"
            }
            Self::ClassicalBitLimitExceeded => {
                "hardware.resource.classical_bits.limit_exceeded"
            }
            Self::InvalidWorkload => "hardware.workload.invalid",
            Self::InconsistentWorkloadKind => {
                "hardware.workload.kind.inconsistent"
            }

            Self::UnsupportedWorkload => {
                "hardware.capability.workload.unsupported"
            }
            Self::UnsupportedCapability => {
                "hardware.capability.unsupported"
            }
            Self::ExperimentalCapabilityNotAccepted => {
                "hardware.capability.experimental_not_accepted"
            }
            Self::UnsupportedInstruction => {
                "hardware.instruction.unsupported"
            }
            Self::UnsupportedGate => "hardware.gate.unsupported",
            Self::MeasurementUnsupported => {
                "hardware.measurement.unsupported"
            }
            Self::ResetUnsupported => "hardware.reset.unsupported",
            Self::MidCircuitMeasurementUnsupported => {
                "hardware.mid_circuit_measurement.unsupported"
            }
            Self::ClassicalControlUnsupported => {
                "hardware.classical_control.unsupported"
            }
            Self::DynamicCircuitUnsupported => {
                "hardware.dynamic_circuit.unsupported"
            }
            Self::PulseControlUnsupported => {
                "hardware.pulse.unsupported"
            }
            Self::AnalogControlUnsupported => {
                "hardware.analog.unsupported"
            }
            Self::AnnealingUnsupported => {
                "hardware.annealing.unsupported"
            }
            Self::LogicalQubitsUnsupported => {
                "hardware.logical_qubits.unsupported"
            }
            Self::FaultToleranceUnsupported => {
                "hardware.fault_tolerance.unsupported"
            }
            Self::DeterministicSeedingUnsupported => {
                "hardware.deterministic_seeding.unsupported"
            }
            Self::StateVectorUnsupported => {
                "hardware.result.state_vector.unsupported"
            }
            Self::DensityMatrixUnsupported => {
                "hardware.result.density_matrix.unsupported"
            }
            Self::ExpectationValuesUnsupported => {
                "hardware.result.expectation_values.unsupported"
            }
            Self::NativeInstructionSetUnavailable => {
                "hardware.instruction_set.unavailable"
            }

            Self::InvalidQubit => "hardware.topology.qubit.invalid",
            Self::UnsupportedConnection => {
                "hardware.topology.connection.unsupported"
            }
            Self::TopologyUnavailable => {
                "hardware.topology.unavailable"
            }
            Self::InvalidTopology => "hardware.topology.invalid",
            Self::RequiredTopologyEdgeLimitExceeded => {
                "hardware.topology.edge_limit_exceeded"
            }
            Self::RequiredInstructionLimitExceeded => {
                "hardware.instruction.limit_exceeded"
            }

            Self::CalibrationUnavailable => {
                "hardware.calibration.unavailable"
            }
            Self::FreshCalibrationRequired => {
                "hardware.calibration.fresh_required"
            }
            Self::InvalidCalibration => "hardware.calibration.invalid",
            Self::StaleCalibration => "hardware.calibration.stale",

            Self::InvalidTiming => "hardware.timing.invalid",
            Self::TimingConstraintViolation => {
                "hardware.timing.constraint_violation"
            }

            Self::BackendUnavailable => "hardware.backend.unavailable",
            Self::BackendRetired => "hardware.backend.retired",
            Self::BackendMaintenance => "hardware.backend.maintenance",

            Self::ExecutionUnavailable => {
                "hardware.execution.unavailable"
            }
            Self::ExecutionRejected => "hardware.execution.rejected",
            Self::InvalidExecutionRequest => {
                "hardware.execution.request.invalid"
            }
            Self::SubmissionRejected => {
                "hardware.execution.submission.rejected"
            }
            Self::SubmissionFailed => {
                "hardware.execution.submission.failed"
            }
            Self::ExecutionFailed => "hardware.execution.failed",
            Self::Timeout => "hardware.execution.timeout",

            Self::InvalidJobId => "hardware.job.id.invalid",
            Self::InvalidJobStateTransition => {
                "hardware.job.state.invalid_transition"
            }
            Self::QueueUnavailable => "hardware.queue.unavailable",
            Self::JobCancelled => "hardware.job.cancelled",
            Self::CancellationRejected => {
                "hardware.cancellation.rejected"
            }
            Self::CancellationFailed => {
                "hardware.cancellation.failed"
            }
            Self::JobExpired => "hardware.job.expired",

            Self::ResultUnavailable => "hardware.result.unavailable",
            Self::ResultDecodingFailed => "hardware.result.decoding_failed",
            Self::InvalidResult => "hardware.result.invalid",
            Self::ResultCountOverflow => "hardware.result.count_overflow",
            Self::ResultShotsExceeded => "hardware.result.shots_exceeded",
            Self::InvalidBitstring => "hardware.result.bitstring.invalid",

            Self::AuthenticationFailed => {
                "hardware.authentication.failed"
            }
            Self::CredentialsUnavailable => {
                "hardware.authentication.credentials_unavailable"
            }
            Self::AuthorizationDenied => {
                "hardware.authorization.denied"
            }
            Self::AccountUnavailable => {
                "hardware.authorization.account_unavailable"
            }

            Self::TransportFailed => "hardware.transport.failed",
            Self::ProviderError => "hardware.provider.error",
            Self::ProviderResponseInvalid => {
                "hardware.provider.response_invalid"
            }
            Self::ProviderApiIncompatible => {
                "hardware.provider.api_incompatible"
            }

            Self::SerializationFailed => {
                "hardware.serialization.failed"
            }
            Self::DeserializationFailed => {
                "hardware.serialization.deserialization_failed"
            }
            Self::UnsupportedSchemaVersion => {
                "hardware.serialization.unsupported_schema"
            }

            Self::InternalInvariantViolation => {
                "hardware.internal.invariant_violation"
            }
            Self::NotImplemented => "hardware.unsupported.not_implemented",
        }
    }

    /// Returns the category associated with this code.
    pub const fn category(self) -> HardwareErrorCategory {
        match self {
            Self::InvalidBackendId
            | Self::InvalidIdentifier
            | Self::IdentifierTooLong
            | Self::InvalidBackendDescriptor => {
                HardwareErrorCategory::Identity
            }

            Self::InvalidMetadata
            | Self::MetadataLimitExceeded
            | Self::SecretLikeMetadata
            | Self::SensitiveErrorContext => {
                HardwareErrorCategory::Identity
            }

            Self::InvalidShots
            | Self::ZeroQubits
            | Self::QubitLimitExceeded
            | Self::LogicalQubitLimitExceeded
            | Self::CircuitDepthExceeded
            | Self::OperationLimitExceeded
            | Self::ShotLimitExceeded
            | Self::ClassicalBitLimitExceeded => {
                HardwareErrorCategory::Resource
            }

            Self::InvalidWorkload
            | Self::InconsistentWorkloadKind
            | Self::InvalidExecutionRequest => {
                HardwareErrorCategory::Validation
            }

            Self::UnsupportedWorkload
            | Self::UnsupportedCapability
            | Self::ExperimentalCapabilityNotAccepted
            | Self::UnsupportedInstruction
            | Self::UnsupportedGate
            | Self::MeasurementUnsupported
            | Self::ResetUnsupported
            | Self::MidCircuitMeasurementUnsupported
            | Self::ClassicalControlUnsupported
            | Self::DynamicCircuitUnsupported
            | Self::PulseControlUnsupported
            | Self::AnalogControlUnsupported
            | Self::AnnealingUnsupported
            | Self::LogicalQubitsUnsupported
            | Self::FaultToleranceUnsupported
            | Self::DeterministicSeedingUnsupported
            | Self::StateVectorUnsupported
            | Self::DensityMatrixUnsupported
            | Self::ExpectationValuesUnsupported
            | Self::NativeInstructionSetUnavailable => {
                HardwareErrorCategory::Capability
            }

            Self::InvalidQubit
            | Self::UnsupportedConnection
            | Self::TopologyUnavailable
            | Self::InvalidTopology
            | Self::RequiredTopologyEdgeLimitExceeded
            | Self::RequiredInstructionLimitExceeded => {
                HardwareErrorCategory::Topology
            }

            Self::CalibrationUnavailable
            | Self::FreshCalibrationRequired
            | Self::InvalidCalibration
            | Self::StaleCalibration => {
                HardwareErrorCategory::Calibration
            }

            Self::InvalidTiming | Self::TimingConstraintViolation => {
                HardwareErrorCategory::Timing
            }

            Self::BackendUnavailable
            | Self::BackendRetired
            | Self::BackendMaintenance => {
                HardwareErrorCategory::Execution
            }

            Self::ExecutionUnavailable
            | Self::ExecutionRejected
            | Self::InvalidExecutionRequest
            | Self::SubmissionRejected
            | Self::SubmissionFailed => {
                HardwareErrorCategory::Submission
            }

            Self::ExecutionFailed | Self::Timeout => {
                HardwareErrorCategory::Execution
            }

            Self::InvalidJobId
            | Self::InvalidJobStateTransition
            | Self::QueueUnavailable => HardwareErrorCategory::Queue,

            Self::JobCancelled
            | Self::CancellationRejected
            | Self::CancellationFailed => {
                HardwareErrorCategory::Cancellation
            }

            Self::JobExpired => HardwareErrorCategory::Queue,

            Self::ResultUnavailable
            | Self::ResultDecodingFailed
            | Self::InvalidResult
            | Self::ResultCountOverflow
            | Self::ResultShotsExceeded
            | Self::InvalidBitstring => HardwareErrorCategory::Result,

            Self::AuthenticationFailed
            | Self::CredentialsUnavailable => {
                HardwareErrorCategory::Authentication
            }

            Self::AuthorizationDenied | Self::AccountUnavailable => {
                HardwareErrorCategory::Authorization
            }

            Self::TransportFailed => HardwareErrorCategory::Transport,

            Self::ProviderError
            | Self::ProviderResponseInvalid
            | Self::ProviderApiIncompatible => {
                HardwareErrorCategory::Provider
            }

            Self::SerializationFailed
            | Self::DeserializationFailed
            | Self::UnsupportedSchemaVersion => {
                HardwareErrorCategory::Serialization
            }

            Self::InternalInvariantViolation => {
                HardwareErrorCategory::Internal
            }

            Self::NotImplemented => HardwareErrorCategory::Unsupported,
        }
    }

    /// Returns the default severity for this error code.
    pub const fn default_severity(self) -> HardwareErrorSeverity {
        match self {
            Self::InternalInvariantViolation => HardwareErrorSeverity::Critical,

            Self::InvalidMetadata
            | Self::MetadataLimitExceeded
            | Self::SecretLikeMetadata
            | Self::SensitiveErrorContext
            | Self::InvalidBackendId
            | Self::InvalidIdentifier
            | Self::IdentifierTooLong
            | Self::InvalidBackendDescriptor => {
                HardwareErrorSeverity::Error
            }

            _ => HardwareErrorSeverity::Error,
        }
    }

    /// Returns the conservative default retryability.
    pub const fn default_retryability(self) -> Retryability {
        match self {
            // Never retry malformed caller input.
            Self::InvalidBackendId
            | Self::InvalidIdentifier
            | Self::IdentifierTooLong
            | Self::InvalidBackendDescriptor
            | Self::InvalidMetadata
            | Self::MetadataLimitExceeded
            | Self::SecretLikeMetadata
            | Self::SensitiveErrorContext
            | Self::InvalidShots
            | Self::ZeroQubits
            | Self::QubitLimitExceeded
            | Self::LogicalQubitLimitExceeded
            | Self::CircuitDepthExceeded
            | Self::OperationLimitExceeded
            | Self::ShotLimitExceeded
            | Self::ClassicalBitLimitExceeded
            | Self::InvalidWorkload
            | Self::InconsistentWorkloadKind
            | Self::UnsupportedWorkload
            | Self::UnsupportedCapability
            | Self::ExperimentalCapabilityNotAccepted
            | Self::UnsupportedInstruction
            | Self::UnsupportedGate
            | Self::MeasurementUnsupported
            | Self::ResetUnsupported
            | Self::MidCircuitMeasurementUnsupported
            | Self::ClassicalControlUnsupported
            | Self::DynamicCircuitUnsupported
            | Self::PulseControlUnsupported
            | Self::AnalogControlUnsupported
            | Self::AnnealingUnsupported
            | Self::LogicalQubitsUnsupported
            | Self::FaultToleranceUnsupported
            | Self::DeterministicSeedingUnsupported
            | Self::StateVectorUnsupported
            | Self::DensityMatrixUnsupported
            | Self::ExpectationValuesUnsupported
            | Self::NativeInstructionSetUnavailable
            | Self::InvalidQubit
            | Self::UnsupportedConnection
            | Self::InvalidTopology
            | Self::RequiredTopologyEdgeLimitExceeded
            | Self::RequiredInstructionLimitExceeded
            | Self::InvalidCalibration
            | Self::InvalidTiming
            | Self::TimingConstraintViolation
            | Self::InvalidExecutionRequest
            | Self::InvalidJobId
            | Self::InvalidJobStateTransition
            | Self::InvalidResult
            | Self::ResultCountOverflow
            | Self::ResultShotsExceeded
            | Self::InvalidBitstring
            | Self::AuthorizationDenied
            | Self::AccountUnavailable
            | Self::UnsupportedSchemaVersion
            | Self::InternalInvariantViolation
            | Self::NotImplemented => Retryability::Never,

            // Conditions that can change independently of the request.
            Self::BackendUnavailable
            | Self::BackendMaintenance
            | Self::QueueUnavailable
            | Self::CalibrationUnavailable
            | Self::FreshCalibrationRequired
            | Self::StaleCalibration
            | Self::TopologyUnavailable
            | Self::ExecutionUnavailable
            | Self::ExecutionRejected
            | Self::SubmissionRejected
            | Self::SubmissionFailed
            | Self::ExecutionFailed
            | Self::Timeout
            | Self::ResultUnavailable
            | Self::ResultDecodingFailed
            | Self::JobExpired
            | Self::CancellationRejected
            | Self::CancellationFailed
            | Self::ProviderResponseInvalid
            | Self::ProviderApiIncompatible => Retryability::Conditional,

            // These can often succeed after transient external recovery.
            Self::AuthenticationFailed
            | Self::CredentialsUnavailable
            | Self::TransportFailed
            | Self::ProviderError
            | Self::SerializationFailed
            | Self::DeserializationFailed => Retryability::Unknown,

            Self::JobCancelled => Retryability::Never,

            Self::BackendRetired => Retryability::Never,
        }
    }
}

impl fmt::Display for HardwareErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Error context
// =============================================================================

/// A safe, bounded, non-secret diagnostic context entry.
///
/// Context values MUST be safe to expose in logs and telemetry.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ErrorContext {
    key: String,
    value: String,
}

impl ErrorContext {
    /// Creates a safe context entry.
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, HardwareError> {
        let key = key.into();
        let value = value.into();

        validate_context_field(&key, &value)?;

        if looks_sensitive(&key) {
            return Err(HardwareError::new(
                HardwareErrorCode::SensitiveErrorContext,
                "error context contains a sensitive field",
            ));
        }

        Ok(Self { key, value })
    }

    /// Returns the context key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the context value.
    pub fn value(&self) -> &str {
        &self.value
    }
}

// =============================================================================
// Canonical hardware error
// =============================================================================

/// Canonical provider-neutral quantum hardware error.
///
/// This is the semantic error contract consumed by the hardware subsystem.
///
/// Provider adapters MUST translate native provider errors into this type
/// before returning from the provider-neutral boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareError {
    /// Error schema identifier.
    pub schema_id: &'static str,

    /// Error schema version.
    pub schema_version: u16,

    /// Stable machine-readable error code.
    pub code: HardwareErrorCode,

    /// Error category.
    pub category: HardwareErrorCategory,

    /// Severity.
    pub severity: HardwareErrorSeverity,

    /// Retry classification.
    pub retryability: Retryability,

    /// Origin of the error.
    pub source: HardwareErrorSource,

    /// Safe human-readable message.
    pub message: String,

    /// Optional safe provider-specific error code.
    pub provider_code: Option<String>,

    /// Optional backend identifier.
    pub backend_id: Option<String>,

    /// Optional job identifier.
    pub job_id: Option<String>,

    /// Optional caller request identifier.
    pub request_id: Option<String>,

    /// Bounded, deterministic, non-secret diagnostic context.
    pub context: Vec<ErrorContext>,
}

impl HardwareError {
    /// Creates a canonical error using the code's default classification.
    pub fn new(
        code: HardwareErrorCode,
        message: impl Into<String>,
    ) -> Self {
        Self {
            schema_id: HARDWARE_ERROR_SCHEMA_ID,
            schema_version: HARDWARE_ERROR_SCHEMA_VERSION,
            category: code.category(),
            severity: code.default_severity(),
            retryability: code.default_retryability(),
            source: HardwareErrorSource::Zamani,
            code,
            message: sanitize_message(message.into()),
            provider_code: None,
            backend_id: None,
            job_id: None,
            request_id: None,
            context: Vec::new(),
        }
    }

    /// Creates an error with explicit severity and retryability.
    pub fn classified(
        code: HardwareErrorCode,
        severity: HardwareErrorSeverity,
        retryability: Retryability,
        message: impl Into<String>,
    ) -> Self {
        Self {
            schema_id: HARDWARE_ERROR_SCHEMA_ID,
            schema_version: HARDWARE_ERROR_SCHEMA_VERSION,
            category: code.category(),
            severity,
            retryability,
            source: HardwareErrorSource::Zamani,
            code,
            message: sanitize_message(message.into()),
            provider_code: None,
            backend_id: None,
            job_id: None,
            request_id: None,
            context: Vec::new(),
        }
    }

    /// Changes the error source.
    pub fn with_source(mut self, source: HardwareErrorSource) -> Self {
        self.source = source;
        self
    }

    /// Adds a safe provider error code.
    pub fn with_provider_code(
        mut self,
        provider_code: impl Into<String>,
    ) -> Result<Self, HardwareError> {
        let provider_code = provider_code.into();

        validate_provider_code(&provider_code)?;

        if looks_sensitive(&provider_code) {
            return Err(HardwareError::new(
                HardwareErrorCode::SensitiveErrorContext,
                "provider error code appears to contain sensitive material",
            ));
        }

        self.provider_code = Some(provider_code);
        Ok(self)
    }

    /// Adds a backend identifier.
    pub fn with_backend_id(
        mut self,
        backend_id: impl Into<String>,
    ) -> Result<Self, HardwareError> {
        let backend_id = backend_id.into();

        validate_identifier(
            "backend_id",
            &backend_id,
            MAX_ERROR_BACKEND_ID_LENGTH,
        )?;

        self.backend_id = Some(backend_id);
        Ok(self)
    }

    /// Adds a job identifier.
    pub fn with_job_id(
        mut self,
        job_id: impl Into<String>,
    ) -> Result<Self, HardwareError> {
        let job_id = job_id.into();

        validate_identifier(
            "job_id",
            &job_id,
            MAX_ERROR_JOB_ID_LENGTH,
        )?;

        self.job_id = Some(job_id);
        Ok(self)
    }

    /// Adds a caller request identifier.
    pub fn with_request_id(
        mut self,
        request_id: impl Into<String>,
    ) -> Result<Self, HardwareError> {
        let request_id = request_id.into();

        validate_identifier(
            "request_id",
            &request_id,
            MAX_ERROR_REQUEST_ID_LENGTH,
        )?;

        self.request_id = Some(request_id);
        Ok(self)
    }

    /// Adds safe structured context.
    pub fn with_context(
        mut self,
        context: ErrorContext,
    ) -> Result<Self, HardwareError> {
        if self.context.len() >= MAX_ERROR_CONTEXT_FIELDS {
            return Err(HardwareError::new(
                HardwareErrorCode::MetadataLimitExceeded,
                "hardware error context field limit exceeded",
            ));
        }

        if self.context.iter().any(|existing| {
            existing.key() == context.key()
        }) {
            return Err(HardwareError::new(
                HardwareErrorCode::InvalidMetadata,
                "duplicate hardware error context key",
            ));
        }

        self.context.push(context);
        self.context.sort();

        Ok(self)
    }

    /// Adds a context key/value pair after security validation.
    pub fn with_context_value(
        self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, HardwareError> {
        let context = ErrorContext::new(key, value)?;
        self.with_context(context)
    }

    /// Returns true when automatic retry may be considered.
    pub const fn is_retryable(&self) -> bool {
        self.retryability.may_retry()
    }

    /// Returns true when the error represents a critical condition.
    pub const fn is_critical(&self) -> bool {
        matches!(self.severity, HardwareErrorSeverity::Critical)
    }

    /// Returns the stable machine-readable error code.
    pub const fn code(&self) -> HardwareErrorCode {
        self.code
    }

    /// Returns the stable code string.
    pub const fn code_str(&self) -> &'static str {
        self.code.as_str()
    }

    /// Returns the category.
    pub const fn category(&self) -> HardwareErrorCategory {
        self.category
    }

    /// Returns the severity.
    pub const fn severity(&self) -> HardwareErrorSeverity {
        self.severity
    }

    /// Returns retryability.
    pub const fn retryability(&self) -> Retryability {
        self.retryability
    }

    /// Returns the safe human-readable message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns whether this error contains a job identifier.
    pub fn has_job_id(&self) -> bool {
        self.job_id.is_some()
    }

    /// Returns a safe compact representation for telemetry/logging.
    ///
    /// This intentionally excludes provider credentials and never exposes
    /// arbitrary raw payloads.
    pub fn safe_summary(&self) -> String {
        let mut summary = format!(
            "{} [{}] {}",
            self.code,
            self.severity,
            self.message
        );

        if let Some(backend_id) = &self.backend_id {
            summary.push_str("; backend=");
            summary.push_str(backend_id);
        }

        if let Some(job_id) = &self.job_id {
            summary.push_str("; job=");
            summary.push_str(job_id);
        }

        summary
    }
}

impl fmt::Display for HardwareError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {}",
            self.code.as_str(),
            self.message
        )
    }
}

impl Error for HardwareError {}

// =============================================================================
// Topology integration
// =============================================================================

impl From<TopologyError> for HardwareError {
    fn from(error: TopologyError) -> Self {
        let message = error.to_string();

        let code = match error {
            TopologyError::ZeroQubits
            | TopologyError::ZeroResources => HardwareErrorCode::ZeroQubits,

            TopologyError::InvalidQubit { .. }
            | TopologyError::InvalidResource { .. } => {
                HardwareErrorCode::InvalidQubit
            }

            TopologyError::SelfCoupling { .. }
            | TopologyError::DuplicateCoupling { .. }
            | TopologyError::MissingCoupling { .. }
            | TopologyError::NoPath { .. } => {
                HardwareErrorCode::UnsupportedConnection
            }

            TopologyError::InvalidTopology { .. } => {
                HardwareErrorCode::InvalidTopology
            }

            TopologyError::NumericOverflow { .. } => {
                HardwareErrorCode::InvalidTopology
            }
        };

        Self::new(code, message)
            .with_source(HardwareErrorSource::Subsystem)
    }
}

// =============================================================================
// Error constructors for common paths
// =============================================================================

impl HardwareError {
    /// Invalid backend identifier.
    pub fn invalid_backend_id() -> Self {
        Self::new(
            HardwareErrorCode::InvalidBackendId,
            "backend identifier is invalid",
        )
    }

    /// Backend unavailable.
    pub fn backend_unavailable() -> Self {
        Self::classified(
            HardwareErrorCode::BackendUnavailable,
            HardwareErrorSeverity::Error,
            Retryability::Conditional,
            "quantum backend is unavailable",
        )
    }

    /// Unsupported capability.
    pub fn unsupported_capability(
        capability: impl Into<String>,
    ) -> Self {
        let capability = sanitize_message(capability.into());

        Self::new(
            HardwareErrorCode::UnsupportedCapability,
            format!(
                "backend does not support required capability '{}'",
                capability
            ),
        )
    }

    /// Unsupported gate/instruction.
    pub fn unsupported_gate(gate: impl Into<String>) -> Self {
        let gate = sanitize_message(gate.into());

        Self::new(
            HardwareErrorCode::UnsupportedGate,
            format!("backend does not support gate '{}'", gate),
        )
    }

    /// Invalid workload.
    pub fn invalid_workload(message: impl Into<String>) -> Self {
        Self::new(
            HardwareErrorCode::InvalidWorkload,
            format!("invalid quantum workload: {}", sanitize_message(message.into())),
        )
    }

    /// Execution unavailable.
    pub fn execution_unavailable(message: impl Into<String>) -> Self {
        Self::classified(
            HardwareErrorCode::ExecutionUnavailable,
            HardwareErrorSeverity::Error,
            Retryability::Conditional,
            format!("execution unavailable: {}", sanitize_message(message.into())),
        )
    }

    /// Execution rejected.
    pub fn execution_rejected(message: impl Into<String>) -> Self {
        Self::classified(
            HardwareErrorCode::ExecutionRejected,
            HardwareErrorSeverity::Error,
            Retryability::Conditional,
            format!("execution rejected: {}", sanitize_message(message.into())),
        )
    }

    /// Authentication failure.
    pub fn authentication_failed() -> Self {
        Self::classified(
            HardwareErrorCode::AuthenticationFailed,
            HardwareErrorSeverity::Error,
            Retryability::Unknown,
            "hardware provider authentication failed",
        )
    }

    /// Authorization failure.
    pub fn authorization_denied() -> Self {
        Self::new(
            HardwareErrorCode::AuthorizationDenied,
            "hardware provider authorization denied the operation",
        )
    }

    /// Transport failure.
    pub fn transport_failed(message: impl Into<String>) -> Self {
        Self::classified(
            HardwareErrorCode::TransportFailed,
            HardwareErrorSeverity::Error,
            Retryability::Recommended,
            format!(
                "hardware provider transport failed: {}",
                sanitize_message(message.into())
            ),
        )
        .with_source(HardwareErrorSource::Transport)
    }

    /// Provider error.
    pub fn provider_error(
        provider_code: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        let mut error = Self::classified(
            HardwareErrorCode::ProviderError,
            HardwareErrorSeverity::Error,
            Retryability::Unknown,
            sanitize_message(message.into()),
        )
        .with_source(HardwareErrorSource::Provider);

        if let Some(code) = provider_code {
            if validate_provider_code(&code).is_ok()
                && !looks_sensitive(&code)
            {
                error.provider_code = Some(code);
            }
        }

        error
    }

    /// Timeout.
    pub fn timeout(message: impl Into<String>) -> Self {
        Self::classified(
            HardwareErrorCode::Timeout,
            HardwareErrorSeverity::Error,
            Retryability::Conditional,
            format!("hardware operation timed out: {}", sanitize_message(message.into())),
        )
    }

    /// Result unavailable.
    pub fn result_unavailable() -> Self {
        Self::classified(
            HardwareErrorCode::ResultUnavailable,
            HardwareErrorSeverity::Error,
            Retryability::Conditional,
            "execution result is not currently available",
        )
    }

    /// Invalid result.
    pub fn invalid_result(message: impl Into<String>) -> Self {
        Self::new(
            HardwareErrorCode::InvalidResult,
            format!(
                "invalid quantum execution result: {}",
                sanitize_message(message.into())
            ),
        )
    }

    /// Internal invariant violation.
    pub fn internal_invariant(message: impl Into<String>) -> Self {
        Self::classified(
            HardwareErrorCode::InternalInvariantViolation,
            HardwareErrorSeverity::Critical,
            Retryability::Never,
            format!(
                "hardware subsystem invariant violation: {}",
                sanitize_message(message.into())
            ),
        )
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), HardwareError> {
    if value.trim().is_empty() {
        return Err(HardwareError::classified(
            HardwareErrorCode::InvalidIdentifier,
            HardwareErrorSeverity::Error,
            Retryability::Never,
            format!("{} identifier cannot be empty", field),
        ));
    }

    if value.len() > maximum {
        return Err(HardwareError::new(
            HardwareErrorCode::IdentifierTooLong,
            format!(
                "{} identifier exceeds maximum length {}",
                field, maximum
            ),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(HardwareError::new(
            HardwareErrorCode::InvalidIdentifier,
            format!("{} identifier contains control characters", field),
        ));
    }

    Ok(())
}

fn validate_provider_code(
    provider_code: &str,
) -> Result<(), HardwareError> {
    if provider_code.trim().is_empty() {
        return Err(HardwareError::new(
            HardwareErrorCode::ProviderError,
            "provider error code cannot be empty",
        ));
    }

    if provider_code.len() > MAX_PROVIDER_ERROR_CODE_LENGTH {
        return Err(HardwareError::new(
            HardwareErrorCode::ProviderResponseInvalid,
            "provider error code exceeds maximum length",
        ));
    }

    if provider_code.chars().any(char::is_control) {
        return Err(HardwareError::new(
            HardwareErrorCode::ProviderResponseInvalid,
            "provider error code contains control characters",
        ));
    }

    Ok(())
}

fn validate_context_field(
    key: &str,
    value: &str,
) -> Result<(), HardwareError> {
    if key.trim().is_empty() {
        return Err(HardwareError::new(
            HardwareErrorCode::InvalidMetadata,
            "error context key cannot be empty",
        ));
    }

    if key.len() > MAX_ERROR_CONTEXT_KEY_LENGTH {
        return Err(HardwareError::new(
            HardwareErrorCode::MetadataLimitExceeded,
            "error context key exceeds maximum length",
        ));
    }

    if value.len() > MAX_ERROR_CONTEXT_VALUE_LENGTH {
        return Err(HardwareError::new(
            HardwareErrorCode::MetadataLimitExceeded,
            "error context value exceeds maximum length",
        ));
    }

    if key.chars().any(char::is_control)
        || value.chars().any(char::is_control)
    {
        return Err(HardwareError::new(
            HardwareErrorCode::InvalidMetadata,
            "error context contains control characters",
        ));
    }

    Ok(())
}

/// Detects common secret-bearing field names.
///
/// This is deliberately conservative. False positives are preferable to
/// accidentally placing credentials into logs.
fn looks_sensitive(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();

    const SENSITIVE_MARKERS: &[&str] = &[
        "api_key",
        "apikey",
        "access_token",
        "access-token",
        "authorization",
        "auth_header",
        "bearer",
        "password",
        "passwd",
        "secret",
        "private_key",
        "private-key",
        "client_secret",
        "client-secret",
        "refresh_token",
        "refresh-token",
        "session_cookie",
        "cookie",
        "credential",
        "credentials",
    ];

    SENSITIVE_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
}

/// Sanitizes human-readable diagnostic text.
///
/// The function does not attempt to identify arbitrary secrets embedded in
/// prose. Callers MUST never pass raw provider responses containing credentials.
/// This function primarily removes control characters and bounds the message.
fn sanitize_message(mut message: String) -> String {
    if message.len() > MAX_ERROR_MESSAGE_LENGTH {
        message.truncate(MAX_ERROR_MESSAGE_LENGTH);
    }

    message
        .chars()
        .filter(|character| {
            !character.is_control()
                || matches!(character, '\n' | '\r' | '\t')
        })
        .collect()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes_are_stable() {
        assert_eq!(
            HardwareErrorCode::InvalidBackendId.as_str(),
            "hardware.backend.invalid_id"
        );

        assert_eq!(
            HardwareErrorCode::ExecutionFailed.as_str(),
            "hardware.execution.failed"
        );

        assert_eq!(
            HardwareErrorCode::TransportFailed.as_str(),
            "hardware.transport.failed"
        );
    }

    #[test]
    fn categories_are_stable() {
        assert_eq!(
            HardwareErrorCode::UnsupportedGate.category(),
            HardwareErrorCategory::Capability
        );

        assert_eq!(
            HardwareErrorCode::InvalidTopology.category(),
            HardwareErrorCategory::Topology
        );

        assert_eq!(
            HardwareErrorCode::ExecutionFailed.category(),
            HardwareErrorCategory::Execution
        );

        assert_eq!(
            HardwareErrorCode::ResultUnavailable.category(),
            HardwareErrorCategory::Result
        );
    }

    #[test]
    fn malformed_input_is_not_retryable() {
        assert_eq!(
            HardwareErrorCode::InvalidWorkload.default_retryability(),
            Retryability::Never
        );

        assert_eq!(
            HardwareErrorCode::UnsupportedCapability.default_retryability(),
            Retryability::Never
        );
    }

    #[test]
    fn transient_errors_are_retryable_or_conditional() {
        assert_eq!(
            HardwareErrorCode::TransportFailed.default_retryability(),
            Retryability::Unknown
        );

        assert_eq!(
            HardwareErrorCode::BackendUnavailable.default_retryability(),
            Retryability::Conditional
        );

        assert_eq!(
            HardwareErrorCode::QueueUnavailable.default_retryability(),
            Retryability::Conditional
        );
    }

    #[test]
    fn error_context_rejects_sensitive_keys() {
        let result = ErrorContext::new(
            "api_key",
            "secret-value",
        );

        assert!(result.is_err());

        let error = result.unwrap_err();

        assert_eq!(
            error.code(),
            HardwareErrorCode::SensitiveErrorContext
        );
    }

    #[test]
    fn error_context_rejects_control_characters() {
        let result = ErrorContext::new(
            "provider",
            "bad\nvalue",
        );

        assert!(result.is_err());
    }

    #[test]
    fn error_context_is_deterministically_sorted() {
        let error = HardwareError::new(
            HardwareErrorCode::ProviderError,
            "provider failure",
        )
        .with_context_value("z", "last")
        .unwrap()
        .with_context_value("a", "first")
        .unwrap();

        assert_eq!(error.context[0].key(), "a");
        assert_eq!(error.context[1].key(), "z");
    }

    #[test]
    fn backend_identifier_is_bounded() {
        let error = HardwareError::new(
            HardwareErrorCode::BackendUnavailable,
            "backend unavailable",
        )
        .with_backend_id("local://simulator")
        .unwrap();

        assert_eq!(
            error.backend_id.as_deref(),
            Some("local://simulator")
        );
    }

    #[test]
    fn provider_code_is_bounded() {
        let error = HardwareError::provider_error(
            Some("DEVICE_BUSY".to_string()),
            "provider rejected request",
        );

        assert_eq!(
            error.provider_code.as_deref(),
            Some("DEVICE_BUSY")
        );
    }

    #[test]
    fn provider_code_rejects_secret_like_values() {
        let error = HardwareError::provider_error(
            Some("api_key=super-secret-value".to_string()),
            "provider rejected request",
        );

        assert!(error.provider_code.is_none());
    }

    #[test]
    fn topology_errors_convert() {
        let topology_error = TopologyError::ZeroQubits;
        let hardware_error: HardwareError = topology_error.into();

        assert_eq!(
            hardware_error.code(),
            HardwareErrorCode::ZeroQubits
        );
    }

    #[test]
    fn topology_invalid_errors_convert() {
        let topology_error = TopologyError::InvalidTopology {
            message: "broken invariant".to_string(),
        };

        let hardware_error: HardwareError = topology_error.into();

        assert_eq!(
            hardware_error.code(),
            HardwareErrorCode::InvalidTopology
        );
    }

    #[test]
    fn safe_summary_contains_no_raw_context_values() {
        let error = HardwareError::new(
            HardwareErrorCode::ExecutionFailed,
            "execution failed",
        )
        .with_backend_id("local://test")
        .unwrap()
        .with_job_id("job-123")
        .unwrap()
        .with_context_value("provider", "local")
        .unwrap();

        let summary = error.safe_summary();

        assert!(summary.contains("hardware.execution.failed"));
        assert!(summary.contains("local://test"));
        assert!(summary.contains("job-123"));
        assert!(!summary.contains("provider"));
    }

    #[test]
    fn error_is_send_sync_compatible() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<HardwareError>();
        assert_send_sync::<ErrorContext>();
    }

    #[test]
    fn critical_invariant_is_never_retryable() {
        let error = HardwareError::internal_invariant(
            "backend invariant violated",
        );

        assert!(error.is_critical());
        assert_eq!(
            error.retryability(),
            Retryability::Never
        );
    }

    #[test]
    fn execution_error_has_expected_classification() {
        let error = HardwareError::new(
            HardwareErrorCode::ExecutionFailed,
            "execution failed",
        );

        assert_eq!(
            error.category(),
            HardwareErrorCategory::Execution
        );

        assert_eq!(
            error.severity(),
            HardwareErrorSeverity::Error
        );
    }

    #[test]
    fn schema_is_stable() {
        let error = HardwareError::new(
            HardwareErrorCode::InvalidWorkload,
            "invalid workload",
        );

        assert_eq!(
            error.schema_id,
            HARDWARE_ERROR_SCHEMA_ID
        );

        assert_eq!(
            error.schema_version,
            HARDWARE_ERROR_SCHEMA_VERSION
        );
    }

    #[test]
    fn messages_are_bounded() {
        let message = "x".repeat(MAX_ERROR_MESSAGE_LENGTH + 100);

        let error = HardwareError::new(
            HardwareErrorCode::ProviderError,
            message,
        );

        assert_eq!(
            error.message.len(),
            MAX_ERROR_MESSAGE_LENGTH
        );
    }

    #[test]
    fn control_characters_are_sanitized() {
        let error = HardwareError::new(
            HardwareErrorCode::ProviderError,
            "bad\u{0000}message",
        );

        assert!(!error.message.contains('\u{0000}'));
    }
}