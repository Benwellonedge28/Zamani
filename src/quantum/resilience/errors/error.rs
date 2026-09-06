//! Zamani Quantum Resilience — Production Error Contract
//!
//! This module defines the foundational, provider-neutral error contract for
//! `quantum::resilience`.
//!
//! # Architectural responsibility
//!
//! This file owns the stable representation of errors produced by the
//! resilience subsystem. It provides:
//!
//! - stable machine-readable error codes;
//! - semantic error categories;
//! - severity;
//! - retryability;
//! - recovery eligibility;
//! - safe structured diagnostic context;
//! - logical-qubit identification;
//! - physical-qubit identification;
//! - resource identification;
//! - operation identification;
//! - optional underlying error preservation;
//! - deterministic display formatting;
//! - conversion from standard Rust errors;
//! - compatibility helpers for resilience orchestration;
//! - no hardware-size assumptions;
//! - no provider-specific behavior.
//!
//! It deliberately does NOT own:
//!
//! - fault detection;
//! - fault diagnosis;
//! - recovery planning;
//! - recovery execution;
//! - mitigation;
//! - QEC;
//! - routing;
//! - scheduling;
//! - hardware discovery;
//! - backend execution;
//! - checkpoint storage;
//! - telemetry collection;
//! - policy decisions.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Architectural position
//!
//! ```text
//!                    quantum::resilience
//!                            |
//!             +--------------+--------------+
//!             |              |              |
//!             v              v              v
//!          detection      planning       recovery
//!             |              |              |
//!             +--------------+--------------+
//!                            |
//!                            v
//!                    ResilienceError
//!                            |
//!          +-----------------+------------------+
//!          |                 |                  |
//!          v                 v                  v
//!       runtime          telemetry          diagnostics
//! ```
//!
//! Every fallible public resilience operation should ultimately return:
//!
//! ```text
//! Result<T, ResilienceError>
//! ```
//!
//! or an error that preserves `ResilienceError` without discarding its
//! structured information.
//!
//! # Write once, scale everywhere
//!
//! This module contains no architectural quantum-machine limit.
//!
//! It MUST NOT contain assumptions such as:
//!
//! ```text
//! MAX_QUBITS = 127
//! MAX_QUBITS = 1000
//! MAX_QUBITS = 1000000
//! retry_count = 3
//! ```
//!
//! Concrete limits belong to:
//!
//! - resilience policy;
//! - target capabilities;
//! - execution resources;
//! - security policy;
//! - runtime configuration.
//!
//! This error type therefore accepts resource identifiers and quantities as
//! data rather than encoding a particular machine size.
//!
//! # Canonical qubit identity
//!
//! Resilience must never introduce its own qubit identifier.
//!
//! The canonical identities are:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! A logical qubit and a physical qubit remain different Rust types.
//! This prevents resilience code from accidentally treating a logical
//! identity as a hardware location.
//!
//! # Error semantics
//!
//! Error display text is intended for humans.
//!
//! Machine consumers MUST use:
//!
//! ```text
//! ResilienceError::code()
//! ResilienceError::category()
//! ResilienceError::severity()
//! ResilienceError::retryability()
//! ResilienceError::recoverability()
//! ```
//!
//! Display text is not a stable machine protocol.
//!
//! # Security
//!
//! Error values may cross subsystem and process boundaries. Consequently,
//! callers must never put secrets into diagnostic messages or context.
//!
//! In particular, error context MUST NOT contain:
//!
//! - credentials;
//! - API keys;
//! - access tokens;
//! - private keys;
//! - passwords;
//! - authorization headers;
//! - session secrets;
//! - raw device pointers;
//! - memory addresses;
//! - unredacted private program data;
//! - backend authentication material.
//!
//! The error model itself does not attempt to guess whether an arbitrary
//! string is secret. Callers are responsible for supplying safe context.
//!
//! # Underlying errors
//!
//! An optional source error can be retained through an `Arc` so that the
//! resilience error remains clonable and can safely cross ownership
//! boundaries.
//!
//! Source errors are never rendered automatically into the public display
//! message. This avoids accidentally exposing backend/provider internals or
//! secrets.
//!
//! # Determinism
//!
//! The structured fields are deterministic when their inputs are deterministic.
//!
//! Context entries preserve insertion order because ordering can be useful for
//! reproducible diagnostics and deterministic tests.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! This file intentionally has no dependency on:
//!
//! ```text
//! resilience::model
//! resilience::detection
//! resilience::diagnosis
//! resilience::policy
//! resilience::planning
//! resilience::adaptation
//! resilience::recovery
//! resilience::mitigation
//! resilience::verification
//! resilience::state
//! resilience::checkpoint
//! resilience::telemetry
//! resilience::history
//! resilience::learning
//! resilience::coordination
//! ```
//!
//! That makes this file safe to implement first.
//!
//! Other resilience modules depend on this file, not the reverse.
//!
//! =============================================================================
//! Compiler-enforced safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Imports
// =============================================================================

use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;

use thiserror::Error;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Public schema constants
// =============================================================================

/// Stable schema identifier for the resilience error contract.
pub const RESILIENCE_ERROR_SCHEMA_ID: &str = "zamani.quantum.resilience.error";

/// Semantic version of the resilience error schema.
///
/// Increment according to the project's compatibility policy when the
/// externally observable semantic contract changes.
pub const RESILIENCE_ERROR_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Error code
// =============================================================================

/// Stable machine-readable identity for a resilience failure.
///
/// Numeric values are deliberately stable and must not be reused for a
/// different semantic meaning once released.
///
/// The string returned by [`ResilienceErrorCode::as_str`] is the preferred
/// cross-process and telemetry representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum ResilienceErrorCode {
    // -------------------------------------------------------------------------
    // General / validation
    // -------------------------------------------------------------------------

    /// Generic invalid argument.
    InvalidArgument = 1,

    /// Invalid or inconsistent identifier.
    InvalidIdentifier = 2,

    /// Invalid configuration.
    InvalidConfiguration = 3,

    /// Required information is missing.
    MissingInformation = 4,

    /// An invariant required by the resilience contract was violated.
    InvariantViolation = 5,

    /// An operation is not valid in the current state.
    InvalidState = 6,

    /// An arithmetic operation overflowed.
    ArithmeticOverflow = 7,

    /// A requested value cannot be represented by the host representation.
    RepresentationOverflow = 8,

    // -------------------------------------------------------------------------
    // Detection
    // -------------------------------------------------------------------------

    /// Fault/anomaly detection failed.
    DetectionFailed = 20,

    /// Detector input was invalid.
    InvalidDetectionInput = 21,

    /// Detector produced inconsistent observations.
    DetectionInconsistent = 22,

    /// Telemetry required by a detector is unavailable.
    DetectionDataUnavailable = 23,

    /// Detection data is stale.
    DetectionDataStale = 24,

    /// Detection was inconclusive.
    DetectionInconclusive = 25,

    // -------------------------------------------------------------------------
    // Diagnosis
    // -------------------------------------------------------------------------

    /// Diagnosis failed.
    DiagnosisFailed = 30,

    /// Root cause could not be determined with sufficient confidence.
    RootCauseUnknown = 31,

    /// Multiple mutually incompatible diagnoses were produced.
    DiagnosisConflict = 32,

    /// Diagnosis evidence was insufficient.
    InsufficientEvidence = 33,

    // -------------------------------------------------------------------------
    // Policy
    // -------------------------------------------------------------------------

    /// Policy rejected the requested operation.
    PolicyRejected = 40,

    /// A safety policy rejected an action.
    SafetyPolicyRejected = 41,

    /// A resource policy rejected an action.
    ResourcePolicyRejected = 42,

    /// A configured budget was exhausted.
    BudgetExceeded = 43,

    /// An escalation boundary was reached.
    EscalationRequired = 44,

    // -------------------------------------------------------------------------
    // Planning
    // -------------------------------------------------------------------------

    /// Recovery planning failed.
    PlanningFailed = 50,

    /// No recovery plan satisfies the current constraints.
    NoFeasiblePlan = 51,

    /// A generated plan is internally inconsistent.
    InvalidPlan = 52,

    /// A planned action became infeasible before execution.
    PlanStale = 53,

    /// Plan ranking could not establish an acceptable candidate.
    PlanSelectionFailed = 54,

    // -------------------------------------------------------------------------
    // Adaptation
    // -------------------------------------------------------------------------

    /// Generic adaptation failure.
    AdaptationFailed = 60,

    /// Logical-to-physical remapping failed.
    RemappingFailed = 61,

    /// Rerouting failed.
    ReroutingFailed = 62,

    /// Rescheduling failed.
    ReschedulingFailed = 63,

    /// Recompilation failed.
    RecompilationFailed = 64,

    /// Reoptimization failed.
    ReoptimizationFailed = 65,

    /// QEC adaptation failed.
    QecAdaptationFailed = 66,

    /// Backend/device selection failed.
    BackendSelectionFailed = 67,

    /// Adaptation would violate program semantics.
    SemanticAdaptationViolation = 68,

    // -------------------------------------------------------------------------
    // Recovery
    // -------------------------------------------------------------------------

    /// Generic recovery failure.
    RecoveryFailed = 70,

    /// Retry is not permitted or could not be performed.
    RetryFailed = 71,

    /// Restart failed.
    RestartFailed = 72,

    /// Resume failed.
    ResumeFailed = 73,

    /// Rollback failed.
    RollbackFailed = 74,

    /// Migration failed.
    MigrationFailed = 75,

    /// Compensation failed.
    CompensationFailed = 76,

    /// Recovery precondition was not satisfied.
    RecoveryPreconditionFailed = 77,

    /// Recovery produced a result that could not be verified.
    RecoveryVerificationFailed = 78,

    /// Recovery was explicitly aborted.
    RecoveryAborted = 79,

    // -------------------------------------------------------------------------
    // Mitigation
    // -------------------------------------------------------------------------

    /// Generic error-mitigation failure.
    MitigationFailed = 80,

    /// No suitable mitigation strategy exists.
    NoMitigationAvailable = 81,

    /// Mitigation is incompatible with the target.
    MitigationCapabilityUnavailable = 82,

    /// Mitigation overhead exceeds the applicable policy.
    MitigationBudgetExceeded = 83,

    /// Mitigation changed an invariant that must be preserved.
    MitigationInvariantViolation = 84,

    // -------------------------------------------------------------------------
    // Verification
    // -------------------------------------------------------------------------

    /// Verification failed.
    VerificationFailed = 90,

    /// A semantic invariant failed.
    SemanticVerificationFailed = 91,

    /// Result verification failed.
    ResultVerificationFailed = 92,

    /// Verification confidence is insufficient.
    VerificationInconclusive = 93,

    /// Provenance verification failed.
    ProvenanceVerificationFailed = 94,

    /// Recovered result cannot be accepted.
    ResultRejected = 95,

    // -------------------------------------------------------------------------
    // Resource / capability
    // -------------------------------------------------------------------------

    /// Required resource is unavailable.
    ResourceUnavailable = 100,

    /// Required resource was lost.
    ResourceLost = 101,

    /// Required capability is unavailable.
    CapabilityUnavailable = 102,

    /// Target capabilities changed.
    CapabilityChanged = 103,

    /// Resource state changed while an operation was in progress.
    ResourceStateChanged = 104,

    /// Resource ownership conflict.
    ResourceOwnershipConflict = 105,

    // -------------------------------------------------------------------------
    // Hardware / backend
    // -------------------------------------------------------------------------

    /// Hardware reported a failure.
    HardwareFailure = 110,

    /// Backend reported a failure.
    BackendFailure = 111,

    /// Backend rejected the operation.
    BackendRejected = 112,

    /// Backend became unavailable.
    BackendUnavailable = 113,

    /// Backend communication failed.
    BackendCommunicationFailed = 114,

    /// Backend operation timed out.
    BackendTimeout = 115,

    /// Backend returned an invalid response.
    InvalidBackendResponse = 116,

    /// Calibration required by the operation is unavailable or stale.
    CalibrationUnavailable = 117,

    /// Hardware state changed during execution.
    HardwareStateChanged = 118,

    // -------------------------------------------------------------------------
    // QEC / quantum fault semantics
    // -------------------------------------------------------------------------

    /// QEC integration failed.
    QecFailure = 120,

    /// QEC capability is unavailable.
    QecCapabilityUnavailable = 121,

    /// Syndrome information is invalid or unavailable.
    InvalidSyndrome = 122,

    /// Decoder failed.
    DecoderFailure = 123,

    /// Logical error could not be corrected.
    LogicalErrorUncorrectable = 124,

    /// Fault information could not be correlated with the execution.
    FaultCorrelationFailed = 125,

    /// A quantum fault location could not be resolved.
    FaultLocalizationFailed = 126,

    // -------------------------------------------------------------------------
    // Checkpoint / persistence
    // -------------------------------------------------------------------------

    /// Checkpoint operation failed.
    CheckpointFailed = 130,

    /// Checkpoint is invalid.
    InvalidCheckpoint = 131,

    /// Checkpoint cannot be restored on the target.
    CheckpointIncompatible = 132,

    /// Checkpoint data is corrupt.
    CheckpointCorrupt = 133,

    /// Checkpoint integrity verification failed.
    CheckpointIntegrityFailed = 134,

    /// Checkpoint schema version is unsupported.
    CheckpointSchemaUnsupported = 135,

    /// Checkpoint storage is unavailable.
    CheckpointStorageUnavailable = 136,

    // -------------------------------------------------------------------------
    // State / concurrency
    // -------------------------------------------------------------------------

    /// Resilience state is inconsistent.
    StateInconsistent = 140,

    /// Concurrent operations conflict.
    ConcurrencyConflict = 141,

    /// Required synchronization failed.
    SynchronizationFailed = 142,

    /// Synchronization timed out.
    SynchronizationTimeout = 143,

    /// State lease/ownership expired.
    LeaseExpired = 144,

    /// Distributed coordination failed.
    CoordinationFailed = 145,

    // -------------------------------------------------------------------------
    // Serialization / compatibility
    // -------------------------------------------------------------------------

    /// Serialization failed.
    SerializationFailed = 150,

    /// Deserialization failed.
    DeserializationFailed = 151,

    /// Serialized data is invalid.
    InvalidSerializedData = 152,

    /// Serialized schema version is unsupported.
    UnsupportedSchemaVersion = 153,

    /// Version compatibility requirements were not satisfied.
    CompatibilityFailure = 154,

    // -------------------------------------------------------------------------
    // Security / integrity
    // -------------------------------------------------------------------------

    /// Authentication failed.
    AuthenticationFailed = 160,

    /// Authorization failed.
    AuthorizationFailed = 161,

    /// Integrity verification failed.
    IntegrityFailure = 162,

    /// An untrusted observation was rejected.
    UntrustedObservation = 163,

    /// A recovery action was rejected by the security boundary.
    SecurityPolicyRejected = 164,

    // -------------------------------------------------------------------------
    // Time / execution
    // -------------------------------------------------------------------------

    /// Generic timeout.
    Timeout = 170,

    /// Execution deadline was exceeded.
    DeadlineExceeded = 171,

    /// Operation was cancelled.
    Cancelled = 172,

    /// Execution was interrupted.
    Interrupted = 173,

    // -------------------------------------------------------------------------
    // Extensibility
    // -------------------------------------------------------------------------

    /// A registered resilience component failed.
    ComponentFailure = 180,

    /// A requested extension was not found.
    ComponentUnavailable = 181,

    /// A component reported an incompatible contract.
    ComponentIncompatible = 182,

    // -------------------------------------------------------------------------
    // Generic fallback
    // -------------------------------------------------------------------------

    /// Internal resilience failure.
    InternalError = 190,

    /// Unknown failure that could not be classified more precisely.
    Unknown = 191,
}

impl ResilienceErrorCode {
    /// Returns the stable numeric code.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self as u16
    }

    /// Returns the stable machine-readable code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidArgument => "QR001",
            Self::InvalidIdentifier => "QR002",
            Self::InvalidConfiguration => "QR003",
            Self::MissingInformation => "QR004",
            Self::InvariantViolation => "QR005",
            Self::InvalidState => "QR006",
            Self::ArithmeticOverflow => "QR007",
            Self::RepresentationOverflow => "QR008",

            Self::DetectionFailed => "QR020",
            Self::InvalidDetectionInput => "QR021",
            Self::DetectionInconsistent => "QR022",
            Self::DetectionDataUnavailable => "QR023",
            Self::DetectionDataStale => "QR024",
            Self::DetectionInconclusive => "QR025",

            Self::DiagnosisFailed => "QR030",
            Self::RootCauseUnknown => "QR031",
            Self::DiagnosisConflict => "QR032",
            Self::InsufficientEvidence => "QR033",

            Self::PolicyRejected => "QR040",
            Self::SafetyPolicyRejected => "QR041",
            Self::ResourcePolicyRejected => "QR042",
            Self::BudgetExceeded => "QR043",
            Self::EscalationRequired => "QR044",

            Self::PlanningFailed => "QR050",
            Self::NoFeasiblePlan => "QR051",
            Self::InvalidPlan => "QR052",
            Self::PlanStale => "QR053",
            Self::PlanSelectionFailed => "QR054",

            Self::AdaptationFailed => "QR060",
            Self::RemappingFailed => "QR061",
            Self::ReroutingFailed => "QR062",
            Self::ReschedulingFailed => "QR063",
            Self::RecompilationFailed => "QR064",
            Self::ReoptimizationFailed => "QR065",
            Self::QecAdaptationFailed => "QR066",
            Self::BackendSelectionFailed => "QR067",
            Self::SemanticAdaptationViolation => "QR068",

            Self::RecoveryFailed => "QR070",
            Self::RetryFailed => "QR071",
            Self::RestartFailed => "QR072",
            Self::ResumeFailed => "QR073",
            Self::RollbackFailed => "QR074",
            Self::MigrationFailed => "QR075",
            Self::CompensationFailed => "QR076",
            Self::RecoveryPreconditionFailed => "QR077",
            Self::RecoveryVerificationFailed => "QR078",
            Self::RecoveryAborted => "QR079",

            Self::MitigationFailed => "QR080",
            Self::NoMitigationAvailable => "QR081",
            Self::MitigationCapabilityUnavailable => "QR082",
            Self::MitigationBudgetExceeded => "QR083",
            Self::MitigationInvariantViolation => "QR084",

            Self::VerificationFailed => "QR090",
            Self::SemanticVerificationFailed => "QR091",
            Self::ResultVerificationFailed => "QR092",
            Self::VerificationInconclusive => "QR093",
            Self::ProvenanceVerificationFailed => "QR094",
            Self::ResultRejected => "QR095",

            Self::ResourceUnavailable => "QR100",
            Self::ResourceLost => "QR101",
            Self::CapabilityUnavailable => "QR102",
            Self::CapabilityChanged => "QR103",
            Self::ResourceStateChanged => "QR104",
            Self::ResourceOwnershipConflict => "QR105",

            Self::HardwareFailure => "QR110",
            Self::BackendFailure => "QR111",
            Self::BackendRejected => "QR112",
            Self::BackendUnavailable => "QR113",
            Self::BackendCommunicationFailed => "QR114",
            Self::BackendTimeout => "QR115",
            Self::InvalidBackendResponse => "QR116",
            Self::CalibrationUnavailable => "QR117",
            Self::HardwareStateChanged => "QR118",

            Self::QecFailure => "QR120",
            Self::QecCapabilityUnavailable => "QR121",
            Self::InvalidSyndrome => "QR122",
            Self::DecoderFailure => "QR123",
            Self::LogicalErrorUncorrectable => "QR124",
            Self::FaultCorrelationFailed => "QR125",
            Self::FaultLocalizationFailed => "QR126",

            Self::CheckpointFailed => "QR130",
            Self::InvalidCheckpoint => "QR131",
            Self::CheckpointIncompatible => "QR132",
            Self::CheckpointCorrupt => "QR133",
            Self::CheckpointIntegrityFailed => "QR134",
            Self::CheckpointSchemaUnsupported => "QR135",
            Self::CheckpointStorageUnavailable => "QR136",

            Self::StateInconsistent => "QR140",
            Self::ConcurrencyConflict => "QR141",
            Self::SynchronizationFailed => "QR142",
            Self::SynchronizationTimeout => "QR143",
            Self::LeaseExpired => "QR144",
            Self::CoordinationFailed => "QR145",

            Self::SerializationFailed => "QR150",
            Self::DeserializationFailed => "QR151",
            Self::InvalidSerializedData => "QR152",
            Self::UnsupportedSchemaVersion => "QR153",
            Self::CompatibilityFailure => "QR154",

            Self::AuthenticationFailed => "QR160",
            Self::AuthorizationFailed => "QR161",
            Self::IntegrityFailure => "QR162",
            Self::UntrustedObservation => "QR163",
            Self::SecurityPolicyRejected => "QR164",

            Self::Timeout => "QR170",
            Self::DeadlineExceeded => "QR171",
            Self::Cancelled => "QR172",
            Self::Interrupted => "QR173",

            Self::ComponentFailure => "QR180",
            Self::ComponentUnavailable => "QR181",
            Self::ComponentIncompatible => "QR182",

            Self::InternalError => "QR190",
            Self::Unknown => "QR191",
        }
    }

    /// Returns the semantic category of the error.
    #[must_use]
    pub const fn category(self) -> ResilienceErrorCategory {
        match self {
            Self::InvalidArgument
            | Self::InvalidIdentifier
            | Self::InvalidConfiguration
            | Self::MissingInformation
            | Self::InvariantViolation
            | Self::InvalidState
            | Self::ArithmeticOverflow
            | Self::RepresentationOverflow => ResilienceErrorCategory::Validation,

            Self::DetectionFailed
            | Self::InvalidDetectionInput
            | Self::DetectionInconsistent
            | Self::DetectionDataUnavailable
            | Self::DetectionDataStale
            | Self::DetectionInconclusive => ResilienceErrorCategory::Detection,

            Self::DiagnosisFailed
            | Self::RootCauseUnknown
            | Self::DiagnosisConflict
            | Self::InsufficientEvidence => ResilienceErrorCategory::Diagnosis,

            Self::PolicyRejected
            | Self::SafetyPolicyRejected
            | Self::ResourcePolicyRejected
            | Self::BudgetExceeded
            | Self::EscalationRequired => ResilienceErrorCategory::Policy,

            Self::PlanningFailed
            | Self::NoFeasiblePlan
            | Self::InvalidPlan
            | Self::PlanStale
            | Self::PlanSelectionFailed => ResilienceErrorCategory::Planning,

            Self::AdaptationFailed
            | Self::RemappingFailed
            | Self::ReroutingFailed
            | Self::ReschedulingFailed
            | Self::RecompilationFailed
            | Self::ReoptimizationFailed
            | Self::QecAdaptationFailed
            | Self::BackendSelectionFailed
            | Self::SemanticAdaptationViolation => ResilienceErrorCategory::Adaptation,

            Self::RecoveryFailed
            | Self::RetryFailed
            | Self::RestartFailed
            | Self::ResumeFailed
            | Self::RollbackFailed
            | Self::MigrationFailed
            | Self::CompensationFailed
            | Self::RecoveryPreconditionFailed
            | Self::RecoveryVerificationFailed
            | Self::RecoveryAborted => ResilienceErrorCategory::Recovery,

            Self::MitigationFailed
            | Self::NoMitigationAvailable
            | Self::MitigationCapabilityUnavailable
            | Self::MitigationBudgetExceeded
            | Self::MitigationInvariantViolation => ResilienceErrorCategory::Mitigation,

            Self::VerificationFailed
            | Self::SemanticVerificationFailed
            | Self::ResultVerificationFailed
            | Self::VerificationInconclusive
            | Self::ProvenanceVerificationFailed
            | Self::ResultRejected => ResilienceErrorCategory::Verification,

            Self::ResourceUnavailable
            | Self::ResourceLost
            | Self::CapabilityUnavailable
            | Self::CapabilityChanged
            | Self::ResourceStateChanged
            | Self::ResourceOwnershipConflict => ResilienceErrorCategory::Resource,

            Self::HardwareFailure
            | Self::BackendFailure
            | Self::BackendRejected
            | Self::BackendUnavailable
            | Self::BackendCommunicationFailed
            | Self::BackendTimeout
            | Self::InvalidBackendResponse
            | Self::CalibrationUnavailable
            | Self::HardwareStateChanged => ResilienceErrorCategory::Hardware,

            Self::QecFailure
            | Self::QecCapabilityUnavailable
            | Self::InvalidSyndrome
            | Self::DecoderFailure
            | Self::LogicalErrorUncorrectable
            | Self::FaultCorrelationFailed
            | Self::FaultLocalizationFailed => ResilienceErrorCategory::Qec,

            Self::CheckpointFailed
            | Self::InvalidCheckpoint
            | Self::CheckpointIncompatible
            | Self::CheckpointCorrupt
            | Self::CheckpointIntegrityFailed
            | Self::CheckpointSchemaUnsupported
            | Self::CheckpointStorageUnavailable => ResilienceErrorCategory::Checkpoint,

            Self::StateInconsistent
            | Self::ConcurrencyConflict
            | Self::SynchronizationFailed
            | Self::SynchronizationTimeout
            | Self::LeaseExpired
            | Self::CoordinationFailed => ResilienceErrorCategory::Concurrency,

            Self::SerializationFailed
            | Self::DeserializationFailed
            | Self::InvalidSerializedData
            | Self::UnsupportedSchemaVersion
            | Self::CompatibilityFailure => ResilienceErrorCategory::Serialization,

            Self::AuthenticationFailed
            | Self::AuthorizationFailed
            | Self::IntegrityFailure
            | Self::UntrustedObservation
            | Self::SecurityPolicyRejected => ResilienceErrorCategory::Security,

            Self::Timeout
            | Self::DeadlineExceeded
            | Self::Cancelled
            | Self::Interrupted => ResilienceErrorCategory::Execution,

            Self::ComponentFailure
            | Self::ComponentUnavailable
            | Self::ComponentIncompatible => ResilienceErrorCategory::Component,

            Self::InternalError | Self::Unknown => ResilienceErrorCategory::Internal,
        }
    }

    /// Returns the default retryability classification.
    ///
    /// This is a classification, not a command to retry. The actual policy
    /// remains owned by `resilience::policy`.
    #[must_use]
    pub const fn retryability(self) -> Retryability {
        match self {
            Self::DetectionDataStale
            | Self::DetectionDataUnavailable
            | Self::BackendUnavailable
            | Self::BackendCommunicationFailed
            | Self::BackendTimeout
            | Self::ResourceUnavailable
            | Self::SynchronizationTimeout
            | Self::Timeout
            | Self::Interrupted => Retryability::ConditionallyRetryable,

            Self::BackendRejected
            | Self::PolicyRejected
            | Self::SafetyPolicyRejected
            | Self::ResourcePolicyRejected
            | Self::BudgetExceeded
            | Self::EscalationRequired
            | Self::NoFeasiblePlan
            | Self::InvalidPlan
            | Self::SemanticAdaptationViolation
            | Self::RecoveryPreconditionFailed
            | Self::RecoveryVerificationFailed
            | Self::MitigationInvariantViolation
            | Self::SemanticVerificationFailed
            | Self::ResultVerificationFailed
            | Self::ResultRejected
            | Self::AuthorizationFailed
            | Self::SecurityPolicyRejected
            | Self::Cancelled => Retryability::NotRetryable,

            _ => Retryability::Unknown,
        }
    }

    /// Returns the default recovery eligibility classification.
    #[must_use]
    pub const fn recoverability(self) -> Recoverability {
        match self {
            Self::DetectionFailed
            | Self::DetectionInconclusive
            | Self::DiagnosisFailed
            | Self::RootCauseUnknown
            | Self::DiagnosisConflict
            | Self::InsufficientEvidence
            | Self::PlanningFailed
            | Self::NoFeasiblePlan
            | Self::PlanSelectionFailed
            | Self::VerificationFailed
            | Self::VerificationInconclusive
            | Self::InternalError
            | Self::Unknown => Recoverability::Escalate,

            Self::SemanticAdaptationViolation
            | Self::SemanticVerificationFailed
            | Self::ResultVerificationFailed
            | Self::ResultRejected
            | Self::CheckpointIntegrityFailed
            | Self::IntegrityFailure
            | Self::AuthorizationFailed
            | Self::SecurityPolicyRejected
            | Self::LogicalErrorUncorrectable => Recoverability::NonRecoverable,

            _ => Recoverability::PotentiallyRecoverable,
        }
    }

    /// Returns the default severity associated with this error code.
    #[must_use]
    pub const fn severity(self) -> ResilienceSeverity {
        match self {
            Self::SecurityPolicyRejected
            | Self::IntegrityFailure
            | Self::SemanticAdaptationViolation
            | Self::SemanticVerificationFailed
            | Self::ResultVerificationFailed
            | Self::ResultRejected
            | Self::LogicalErrorUncorrectable
            | Self::CheckpointIntegrityFailed
            | Self::InvariantViolation => ResilienceSeverity::Critical,

            Self::HardwareFailure
            | Self::BackendFailure
            | Self::BackendUnavailable
            | Self::QecFailure
            | Self::DecoderFailure
            | Self::StateInconsistent
            | Self::RecoveryFailed
            | Self::RecoveryVerificationFailed
            | Self::NoFeasiblePlan
            | Self::EscalationRequired => ResilienceSeverity::Major,

            Self::DetectionInconclusive
            | Self::DiagnosisFailed
            | Self::RootCauseUnknown
            | Self::DetectionDataStale
            | Self::CapabilityChanged
            | Self::ResourceStateChanged
            | Self::Timeout
            | Self::BackendTimeout => ResilienceSeverity::Degraded,

            _ => ResilienceSeverity::Error,
        }
    }
}

// =============================================================================
// Error category
// =============================================================================

/// Broad semantic category of a resilience error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResilienceErrorCategory {
    /// Input or invariant validation failed.
    Validation,

    /// Fault/anomaly observation failed.
    Detection,

    /// Root-cause analysis failed.
    Diagnosis,

    /// A resilience policy rejected or constrained an action.
    Policy,

    /// Recovery planning failed.
    Planning,

    /// Program/resource adaptation failed.
    Adaptation,

    /// Recovery execution failed.
    Recovery,

    /// Error mitigation failed.
    Mitigation,

    /// Result or semantic verification failed.
    Verification,

    /// Resource/capability failure.
    Resource,

    /// Hardware/backend failure.
    Hardware,

    /// QEC/fault-correction integration failure.
    Qec,

    /// Checkpoint/snapshot failure.
    Checkpoint,

    /// Concurrency/coordination/state synchronization failure.
    Concurrency,

    /// Serialization/schema compatibility failure.
    Serialization,

    /// Security/integrity failure.
    Security,

    /// Runtime execution timing/cancellation failure.
    Execution,

    /// Extensible component/plugin failure.
    Component,

    /// Internal/unknown failure.
    Internal,
}

impl ResilienceErrorCategory {
    /// Returns a stable machine-readable category name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Validation => "validation",
            Self::Detection => "detection",
            Self::Diagnosis => "diagnosis",
            Self::Policy => "policy",
            Self::Planning => "planning",
            Self::Adaptation => "adaptation",
            Self::Recovery => "recovery",
            Self::Mitigation => "mitigation",
            Self::Verification => "verification",
            Self::Resource => "resource",
            Self::Hardware => "hardware",
            Self::Qec => "qec",
            Self::Checkpoint => "checkpoint",
            Self::Concurrency => "concurrency",
            Self::Serialization => "serialization",
            Self::Security => "security",
            Self::Execution => "execution",
            Self::Component => "component",
            Self::Internal => "internal",
        }
    }
}

// =============================================================================
// Severity
// =============================================================================

/// Operational severity of a resilience error.
///
/// Severity is descriptive. It does not itself authorize a recovery action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResilienceSeverity {
    /// Informational condition that does not indicate execution failure.
    Info,

    /// Execution can continue but resilience state has degraded.
    Degraded,

    /// An operation failed and intervention may be required.
    Error,

    /// A significant failure affects execution or resource availability.
    Major,

    /// Correctness, integrity or safety may be compromised.
    Critical,
}

impl ResilienceSeverity {
    /// Returns a stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Degraded => "degraded",
            Self::Error => "error",
            Self::Major => "major",
            Self::Critical => "critical",
        }
    }

    /// Returns whether the error is safety-critical.
    #[must_use]
    pub const fn is_critical(self) -> bool {
        matches!(self, Self::Critical)
    }
}

// =============================================================================
// Retryability
// =============================================================================

/// Classification of whether retrying an operation may be meaningful.
///
/// This type deliberately does not contain retry counts. Retry budgets belong
/// to resilience policy and must remain configurable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Retryability {
    /// Retry is not semantically valid.
    NotRetryable,

    /// Retry may be valid depending on policy and current conditions.
    ConditionallyRetryable,

    /// The error does not provide enough information to classify retryability.
    Unknown,
}

impl Retryability {
    /// Returns a stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRetryable => "not_retryable",
            Self::ConditionallyRetryable => "conditionally_retryable",
            Self::Unknown => "unknown",
        }
    }
}

// =============================================================================
// Recoverability
// =============================================================================

/// Classification of whether resilience can potentially recover from an error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Recoverability {
    /// Recovery is potentially possible.
    PotentiallyRecoverable,

    /// Recovery cannot safely be determined from the error alone.
    Escalate,

    /// Recovery must not be attempted automatically.
    NonRecoverable,
}

impl Recoverability {
    /// Returns a stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PotentiallyRecoverable => "potentially_recoverable",
            Self::Escalate => "escalate",
            Self::NonRecoverable => "non_recoverable",
        }
    }

    /// Returns whether automatic recovery may be considered.
    #[must_use]
    pub const fn may_recover(self) -> bool {
        matches!(self, Self::PotentiallyRecoverable)
    }
}

// =============================================================================
// Diagnostic context
// =============================================================================

/// A structured, caller-supplied diagnostic context entry.
///
/// Context is deliberately represented as an ordered collection rather than a
/// fixed map with a fixed number of entries. This avoids imposing an
/// architectural diagnostic-size limit.
///
/// Context values are strings because resilience errors must not assume the
/// representation of identifiers owned by other subsystems.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErrorContext {
    key: String,
    value: String,
}

impl ErrorContext {
    /// Creates a context entry.
    ///
    /// The caller is responsible for ensuring the value is safe for
    /// diagnostics and does not contain secrets.
    #[must_use]
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Returns the context key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the context value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

// =============================================================================
// Resource identity
// =============================================================================

/// Provider-neutral resource identity attached to a resilience error.
///
/// This type deliberately does not attempt to model the complete hardware
/// resource hierarchy. The authoritative resource/capability model belongs to
/// the hardware subsystem.
///
/// It exists only to identify the resource associated with an error.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResilienceResource {
    /// A logical quantum resource.
    LogicalQubit(QubitId),

    /// A physical target qubit.
    PhysicalQubit(PhysicalQubitId),

    /// A generic externally defined resource identifier.
    ///
    /// The owner of the resource namespace remains responsible for its
    /// semantics.
    Named(String),
}

impl ResilienceResource {
    /// Returns the resource as a stable human-readable value.
    #[must_use]
    pub fn as_str(&self) -> String {
        match self {
            Self::LogicalQubit(id) => id.to_string(),
            Self::PhysicalQubit(id) => id.to_string(),
            Self::Named(value) => value.clone(),
        }
    }

    /// Returns the logical qubit if this resource represents one.
    #[must_use]
    pub const fn logical_qubit(&self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(id) => Some(*id),
            Self::PhysicalQubit(_) | Self::Named(_) => None,
        }
    }

    /// Returns the physical qubit if this resource represents one.
    #[must_use]
    pub const fn physical_qubit(&self) -> Option<PhysicalQubitId> {
        match self {
            Self::PhysicalQubit(id) => Some(*id),
            Self::LogicalQubit(_) | Self::Named(_) => None,
        }
    }
}

impl From<QubitId> for ResilienceResource {
    fn from(value: QubitId) -> Self {
        Self::LogicalQubit(value)
    }
}

impl From<PhysicalQubitId> for ResilienceResource {
    fn from(value: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(value)
    }
}

// =============================================================================
// Error source
// =============================================================================

/// Type-erased source error retained by a resilience error.
///
/// `Arc` permits cloning while preserving the original error object.
pub type ResilienceErrorSource = Arc<dyn StdError + Send + Sync + 'static>;

// =============================================================================
// Main error
// =============================================================================

/// Canonical production error for `quantum::resilience`.
///
/// The type is intentionally structured rather than a large enum containing
/// every possible subsystem-specific payload. New resilience functionality can
/// therefore introduce new context without requiring a breaking change to
/// this foundational error representation.
///
/// The semantic identity of an error is determined by [`ResilienceErrorCode`].
#[derive(Clone)]
pub struct ResilienceError {
    code: ResilienceErrorCode,
    message: String,
    operation: Option<String>,
    resource: Option<ResilienceResource>,
    severity: ResilienceSeverity,
    retryability: Retryability,
    recoverability: Recoverability,
    context: Vec<ErrorContext>,
    source: Option<ResilienceErrorSource>,
}

impl fmt::Debug for ResilienceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = formatter.debug_struct("ResilienceError");

        debug
            .field("code", &self.code.as_str())
            .field("category", &self.code.category())
            .field("severity", &self.severity)
            .field("retryability", &self.retryability)
            .field("recoverability", &self.recoverability)
            .field("message", &self.message)
            .field("operation", &self.operation)
            .field("resource", &self.resource)
            .field("context", &self.context);

        // Do not expose the source's Debug representation automatically.
        // Backend errors can contain sensitive provider implementation data.
        debug.field("source", &self.source.as_ref().map(|_| "<redacted-source>"));

        debug.finish()
    }
}

impl fmt::Display for ResilienceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[{}] {}",
            self.code.as_str(),
            self.message
        )?;

        if let Some(operation) = &self.operation {
            write!(formatter, " operation={operation}")?;
        }

        if let Some(resource) = &self.resource {
            write!(formatter, " resource={}", resource.as_str())?;
        }

        Ok(())
    }
}

impl StdError for ResilienceError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn StdError + 'static))
    }
}

// =============================================================================
// Constructors
// =============================================================================

impl ResilienceError {
    /// Creates a new resilience error using the default metadata associated
    /// with the supplied error code.
    #[must_use]
    pub fn new<M>(code: ResilienceErrorCode, message: M) -> Self
    where
        M: Into<String>,
    {
        Self {
            code,
            message: message.into(),
            operation: None,
            resource: None,
            severity: code.severity(),
            retryability: code.retryability(),
            recoverability: code.recoverability(),
            context: Vec::new(),
            source: None,
        }
    }

    /// Creates an error with an explicit severity.
    #[must_use]
    pub fn with_severity<M>(
        code: ResilienceErrorCode,
        severity: ResilienceSeverity,
        message: M,
    ) -> Self
    where
        M: Into<String>,
    {
        Self {
            code,
            message: message.into(),
            operation: None,
            resource: None,
            severity,
            retryability: code.retryability(),
            recoverability: code.recoverability(),
            context: Vec::new(),
            source: None,
        }
    }

    /// Creates an error while explicitly supplying retryability and
    /// recoverability.
    ///
    /// This is useful when a concrete operation has stronger information than
    /// the generic classification associated with the error code.
    #[must_use]
    pub fn classified<M>(
        code: ResilienceErrorCode,
        severity: ResilienceSeverity,
        retryability: Retryability,
        recoverability: Recoverability,
        message: M,
    ) -> Self
    where
        M: Into<String>,
    {
        Self {
            code,
            message: message.into(),
            operation: None,
            resource: None,
            severity,
            retryability,
            recoverability,
            context: Vec::new(),
            source: None,
        }
    }

    /// Creates a validation error.
    #[must_use]
    pub fn invalid_argument<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::InvalidArgument, message)
    }

    /// Creates an invalid-state error.
    #[must_use]
    pub fn invalid_state<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::InvalidState, message)
    }

    /// Creates an invariant-violation error.
    #[must_use]
    pub fn invariant_violation<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::InvariantViolation, message)
    }

    /// Creates a resource-unavailable error.
    #[must_use]
    pub fn resource_unavailable<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::ResourceUnavailable, message)
    }

    /// Creates a hardware failure.
    #[must_use]
    pub fn hardware_failure<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::HardwareFailure, message)
    }

    /// Creates a backend failure.
    #[must_use]
    pub fn backend_failure<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::BackendFailure, message)
    }

    /// Creates a recovery failure.
    #[must_use]
    pub fn recovery_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::RecoveryFailed, message)
    }

    /// Creates a verification failure.
    #[must_use]
    pub fn verification_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::VerificationFailed, message)
    }

    /// Creates an internal error.
    #[must_use]
    pub fn internal<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::InternalError, message)
    }
}

// =============================================================================
// Accessors
// =============================================================================

impl ResilienceError {
    /// Returns the stable error code.
    #[must_use]
    pub const fn code(&self) -> ResilienceErrorCode {
        self.code
    }

    /// Returns the stable machine-readable code string.
    #[must_use]
    pub const fn code_str(&self) -> &'static str {
        self.code.as_str()
    }

    /// Returns the error category.
    #[must_use]
    pub const fn category(&self) -> ResilienceErrorCategory {
        self.code.category()
    }

    /// Returns the error severity.
    #[must_use]
    pub const fn severity(&self) -> ResilienceSeverity {
        self.severity
    }

    /// Returns retryability.
    #[must_use]
    pub const fn retryability(&self) -> Retryability {
        self.retryability
    }

    /// Returns recoverability.
    #[must_use]
    pub const fn recoverability(&self) -> Recoverability {
        self.recoverability
    }

    /// Returns the human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the associated operation, if any.
    #[must_use]
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Returns the associated resource, if any.
    #[must_use]
    pub const fn resource(&self) -> Option<&ResilienceResource> {
        self.resource.as_ref()
    }

    /// Returns diagnostic context in insertion order.
    #[must_use]
    pub fn context(&self) -> &[ErrorContext] {
        &self.context
    }

    /// Returns whether this error is classified as retryable.
    #[must_use]
    pub const fn is_retryable(&self) -> bool {
        matches!(
            self.retryability,
            Retryability::ConditionallyRetryable
        )
    }

    /// Returns whether automatic recovery may be considered.
    #[must_use]
    pub const fn may_recover(&self) -> bool {
        self.recoverability.may_recover()
    }

    /// Returns whether the error is critical.
    #[must_use]
    pub const fn is_critical(&self) -> bool {
        self.severity.is_critical()
    }

    /// Returns whether this is a security-related error.
    #[must_use]
    pub const fn is_security_error(&self) -> bool {
        matches!(self.category(), ResilienceErrorCategory::Security)
    }

    /// Returns whether this is a verification-related error.
    #[must_use]
    pub const fn is_verification_error(&self) -> bool {
        matches!(self.category(), ResilienceErrorCategory::Verification)
    }
}

// =============================================================================
// Builder-style enrichment
// =============================================================================

impl ResilienceError {
    /// Attaches an operation name.
    ///
    /// The operation is semantic/logical context. It should not contain
    /// secrets or unbounded diagnostic payloads.
    #[must_use]
    pub fn with_operation<S>(mut self, operation: S) -> Self
    where
        S: Into<String>,
    {
        self.operation = Some(operation.into());
        self
    }

    /// Attaches a logical-qubit resource.
    #[must_use]
    pub fn with_logical_qubit(mut self, qubit: QubitId) -> Self {
        self.resource = Some(ResilienceResource::LogicalQubit(qubit));
        self
    }

    /// Attaches a physical-qubit resource.
    #[must_use]
    pub fn with_physical_qubit(mut self, qubit: PhysicalQubitId) -> Self {
        self.resource = Some(ResilienceResource::PhysicalQubit(qubit));
        self
    }

    /// Attaches an arbitrary provider-neutral resource identifier.
    #[must_use]
    pub fn with_resource<R>(mut self, resource: R) -> Self
    where
        R: Into<ResilienceResource>,
    {
        self.resource = Some(resource.into());
        self
    }

    /// Adds structured diagnostic context.
    ///
    /// Context is not deduplicated so that callers can intentionally preserve
    /// an ordered sequence of observations.
    #[must_use]
    pub fn with_context<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.context.push(ErrorContext::new(key, value));
        self
    }

    /// Adds an already constructed context entry.
    #[must_use]
    pub fn with_context_entry(mut self, context: ErrorContext) -> Self {
        self.context.push(context);
        self
    }

    /// Replaces the retryability classification.
    ///
    /// This should only be used when the concrete operation has stronger
    /// semantic information than the generic error code.
    #[must_use]
    pub const fn with_retryability(mut self, retryability: Retryability) -> Self {
        self.retryability = retryability;
        self
    }

    /// Replaces the recoverability classification.
    ///
    /// A caller must never use this to bypass a safety policy.
    #[must_use]
    pub const fn with_recoverability(mut self, recoverability: Recoverability) -> Self {
        self.recoverability = recoverability;
        self
    }

    /// Replaces the severity classification.
    #[must_use]
    pub const fn with_severity_value(mut self, severity: ResilienceSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Preserves an underlying source error.
    ///
    /// The source is retained for programmatic inspection through
    /// [`StdError::source`]. It is intentionally excluded from `Display` and
    /// `Debug` output to reduce accidental disclosure of backend internals.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Arc::new(source));
        self
    }

    /// Preserves an already type-erased source error.
    #[must_use]
    pub fn with_error_source(mut self, source: ResilienceErrorSource) -> Self {
        self.source = Some(source);
        self
    }
}

// =============================================================================
// Context helpers
// =============================================================================

impl ResilienceError {
    /// Returns the first context value associated with `key`.
    #[must_use]
    pub fn context_value(&self, key: &str) -> Option<&str> {
        self.context
            .iter()
            .find(|entry| entry.key() == key)
            .map(ErrorContext::value)
    }

    /// Returns all context values associated with `key`.
    ///
    /// This does not allocate and preserves insertion order.
    #[must_use]
    pub fn context_values<'a>(&'a self, key: &'a str) -> impl Iterator<Item = &'a str> + 'a {
        self.context
            .iter()
            .filter(move |entry| entry.key() == key)
            .map(ErrorContext::value)
    }
}

// =============================================================================
// Error conversion
// =============================================================================

/// Generic boxed-error conversion helper.
///
/// This is intentionally implemented for `Box<dyn Error + Send + Sync>` rather
/// than depending on `anyhow`, because this foundational error contract must
/// remain usable by low-level resilience modules.
impl From<Box<dyn StdError + Send + Sync + 'static>> for ResilienceError {
    fn from(source: Box<dyn StdError + Send + Sync + 'static>) -> Self {
        Self::new(
            ResilienceErrorCode::InternalError,
            "an underlying resilience operation failed",
        )
        .with_source_box(source)
    }
}

impl ResilienceError {
    fn with_source_box(
        mut self,
        source: Box<dyn StdError + Send + Sync + 'static>,
    ) -> Self {
        self.source = Some(Arc::from(source));
        self
    }
}

/// Converts ordinary I/O failures without exposing their display text as the
/// primary resilience error message.
impl From<std::io::Error> for ResilienceError {
    fn from(source: std::io::Error) -> Self {
        Self::new(
            ResilienceErrorCode::InternalError,
            "an I/O operation failed while executing a resilience operation",
        )
        .with_source(source)
    }
}

// =============================================================================
// Result aliases
// =============================================================================

/// Standard result type for fallible resilience operations.
pub type ResilienceResult<T> = Result<T, ResilienceError>;

// =============================================================================
// thiserror integration helpers
// =============================================================================
//
// The repository already depends on `thiserror`. The resilience error itself
// intentionally implements `std::error::Error` manually because its source is
// type-erased behind `Arc`, allowing the public error to remain cloneable.
//
// This small wrapper exists for integrations that need a stable error source
// classification without making the core resilience error depend on concrete
// downstream error enums.

/// Error wrapper for attaching a resilience error to another error hierarchy.
///
/// Downstream modules can use this when their own public error type is an
/// enum generated with `thiserror`.
#[derive(Debug, Error)]
#[error("{0}")]
pub struct ResilienceErrorWrapper(pub ResilienceError);

impl From<ResilienceError> for ResilienceErrorWrapper {
    fn from(error: ResilienceError) -> Self {
        Self(error)
    }
}

impl From<ResilienceErrorWrapper> for ResilienceError {
    fn from(wrapper: ResilienceErrorWrapper) -> Self {
        wrapper.0
    }
}

// =============================================================================
// Canonical constructors for common resilience boundaries
// =============================================================================

impl ResilienceError {
    /// Creates an error for failed detection.
    #[must_use]
    pub fn detection_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::DetectionFailed, message)
    }

    /// Creates an error for failed diagnosis.
    #[must_use]
    pub fn diagnosis_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::DiagnosisFailed, message)
    }

    /// Creates an error for a rejected policy decision.
    #[must_use]
    pub fn policy_rejected<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::PolicyRejected, message)
    }

    /// Creates an error for failed planning.
    #[must_use]
    pub fn planning_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::PlanningFailed, message)
    }

    /// Creates an error for failed adaptation.
    #[must_use]
    pub fn adaptation_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::AdaptationFailed, message)
    }

    /// Creates an error for failed mitigation.
    #[must_use]
    pub fn mitigation_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::MitigationFailed, message)
    }

    /// Creates an error for failed QEC integration.
    #[must_use]
    pub fn qec_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::QecFailure, message)
    }

    /// Creates an error for failed checkpoint processing.
    #[must_use]
    pub fn checkpoint_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::CheckpointFailed, message)
    }

    /// Creates an error for a timeout.
    #[must_use]
    pub fn timeout<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::Timeout, message)
            .with_retryability(Retryability::ConditionallyRetryable)
    }

    /// Creates an error for cancellation.
    #[must_use]
    pub fn cancelled<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::Cancelled, message)
            .with_retryability(Retryability::NotRetryable)
    }

    /// Creates an error for an incompatible capability.
    #[must_use]
    pub fn capability_unavailable<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::CapabilityUnavailable, message)
    }

    /// Creates an error for an unavailable backend.
    #[must_use]
    pub fn backend_unavailable<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(ResilienceErrorCode::BackendUnavailable, message)
            .with_retryability(Retryability::ConditionallyRetryable)
    }

    /// Creates an error for a result that cannot be accepted safely.
    #[must_use]
    pub fn result_rejected<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::classified(
            ResilienceErrorCode::ResultRejected,
            ResilienceSeverity::Critical,
            Retryability::NotRetryable,
            Recoverability::NonRecoverable,
            message,
        )
    }
}

// =============================================================================
// Semantic predicates
// =============================================================================

impl ResilienceError {
    /// Returns whether the error indicates that the execution target may have
    /// changed while the operation was running.
    #[must_use]
    pub const fn indicates_target_change(&self) -> bool {
        matches!(
            self.code,
            ResilienceErrorCode::CapabilityChanged
                | ResilienceErrorCode::ResourceStateChanged
                | ResilienceErrorCode::HardwareStateChanged
                | ResilienceErrorCode::CalibrationUnavailable
        )
    }

    /// Returns whether the error indicates that semantic verification is
    /// required before a result can be accepted.
    #[must_use]
    pub const fn requires_verification(&self) -> bool {
        matches!(
            self.code,
            ResilienceErrorCode::RecoveryFailed
                | ResilienceErrorCode::RecoveryVerificationFailed
                | ResilienceErrorCode::MitigationFailed
                | ResilienceErrorCode::MitigationInvariantViolation
                | ResilienceErrorCode::SemanticAdaptationViolation
                | ResilienceErrorCode::CheckpointIncompatible
        )
    }

    /// Returns whether the error should prevent automatic result acceptance.
    #[must_use]
    pub const fn blocks_result_acceptance(&self) -> bool {
        matches!(
            self.code,
            ResilienceErrorCode::SemanticVerificationFailed
                | ResilienceErrorCode::ResultVerificationFailed
                | ResilienceErrorCode::VerificationFailed
                | ResilienceErrorCode::ResultRejected
                | ResilienceErrorCode::IntegrityFailure
                | ResilienceErrorCode::CheckpointIntegrityFailed
                | ResilienceErrorCode::LogicalErrorUncorrectable
        )
    }
}

// =============================================================================
// Equality semantics
// =============================================================================
//
// `ResilienceError` intentionally does not implement `PartialEq`/`Eq`.
//
// Two errors with the same code are not necessarily semantically identical:
// message, operation, resource, context and source can differ.
//
// Consumers requiring stable comparison should compare:
// - `code()`;
// - category;
// - severity;
// - retryability;
// - recoverability;
// - selected structured fields.
//
// This prevents accidental coupling to diagnostic text or backend error
// representations.

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_codes_are_stable_and_non_empty() {
        let codes = [
            ResilienceErrorCode::InvalidArgument,
            ResilienceErrorCode::DetectionFailed,
            ResilienceErrorCode::DiagnosisFailed,
            ResilienceErrorCode::PolicyRejected,
            ResilienceErrorCode::PlanningFailed,
            ResilienceErrorCode::AdaptationFailed,
            ResilienceErrorCode::RecoveryFailed,
            ResilienceErrorCode::MitigationFailed,
            ResilienceErrorCode::VerificationFailed,
            ResilienceErrorCode::ResourceUnavailable,
            ResilienceErrorCode::HardwareFailure,
            ResilienceErrorCode::QecFailure,
            ResilienceErrorCode::CheckpointFailed,
            ResilienceErrorCode::SerializationFailed,
            ResilienceErrorCode::SecurityPolicyRejected,
            ResilienceErrorCode::InternalError,
        ];

        for code in codes {
            assert!(!code.as_str().is_empty());
            assert!(code.as_u16() > 0);
        }
    }

    #[test]
    fn logical_and_physical_qubits_remain_distinct() {
        let logical = ResilienceResource::from(QubitId::new(7));
        let physical = ResilienceResource::from(PhysicalQubitId::new(7));

        assert_eq!(logical.logical_qubit(), Some(QubitId::new(7)));
        assert_eq!(logical.physical_qubit(), None);

        assert_eq!(physical.logical_qubit(), None);
        assert_eq!(
            physical.physical_qubit(),
            Some(PhysicalQubitId::new(7))
        );
    }

    #[test]
    fn error_enrichment_is_deterministic() {
        let error = ResilienceError::recovery_failed("recovery could not complete")
            .with_operation("resume")
            .with_logical_qubit(QubitId::new(4))
            .with_context("incident", "incident-001")
            .with_context("attempt", "1");

        assert_eq!(error.code(), ResilienceErrorCode::RecoveryFailed);
        assert_eq!(error.operation(), Some("resume"));
        assert_eq!(
            error.resource(),
            Some(&ResilienceResource::LogicalQubit(QubitId::new(4)))
        );

        assert_eq!(error.context_value("incident"), Some("incident-001"));

        let values = error
            .context_values("attempt")
            .collect::<Vec<_>>();

        assert_eq!(values, vec!["1"]);
    }

    #[test]
    fn display_does_not_include_source_error() {
        #[derive(Debug)]
        struct SecretError;

        impl fmt::Display for SecretError {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "secret-provider-token")
            }
        }

        impl StdError for SecretError {}

        let error = ResilienceError::backend_failure("backend operation failed")
            .with_source(SecretError);

        let rendered = error.to_string();

        assert!(rendered.contains("QR111"));
        assert!(rendered.contains("backend operation failed"));
        assert!(!rendered.contains("secret-provider-token"));
        assert!(error.source().is_some());
    }

    #[test]
    fn result_rejection_is_not_retryable() {
        let error = ResilienceError::result_rejected(
            "the recovered result did not satisfy semantic verification",
        );

        assert_eq!(
            error.retryability(),
            Retryability::NotRetryable
        );
        assert_eq!(
            error.recoverability(),
            Recoverability::NonRecoverable
        );
        assert!(error.is_critical());
        assert!(error.blocks_result_acceptance());
    }

    #[test]
    fn backend_unavailability_is_conditionally_retryable() {
        let error = ResilienceError::backend_unavailable(
            "execution target is temporarily unavailable",
        );

        assert_eq!(
            error.retryability(),
            Retryability::ConditionallyRetryable
        );
        assert!(error.is_retryable());
    }

    #[test]
    fn target_change_requires_adaptation_awareness() {
        let error = ResilienceError::new(
            ResilienceErrorCode::CapabilityChanged,
            "target capabilities changed",
        );

        assert!(error.indicates_target_change());
    }

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            RESILIENCE_ERROR_SCHEMA_ID,
            "zamani.quantum.resilience.error"
        );
        assert_eq!(RESILIENCE_ERROR_SCHEMA_VERSION, 1);
    }

    #[test]
    fn error_is_cloneable() {
        let original = ResilienceError::hardware_failure("hardware failure")
            .with_physical_qubit(PhysicalQubitId::new(12))
            .with_context("source", "hardware");

        let cloned = original.clone();

        assert_eq!(cloned.code(), original.code());
        assert_eq!(cloned.message(), original.message());
        assert_eq!(cloned.resource(), original.resource());
        assert_eq!(cloned.context(), original.context());
    }
}