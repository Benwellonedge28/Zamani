//! Zamani Quantum Resilience — Stable Error Codes
//!
//! This module is the canonical, provider-neutral registry of machine-readable
//! error identities for `quantum::resilience`.
//!
//! # Responsibility
//!
//! This file owns ONLY the stable identity of resilience errors:
//!
//! - numeric error identifiers;
//! - stable textual error identifiers;
//! - schema identity/version;
//! - conversion from stable numeric identifiers;
//! - compile-time classification by broad subsystem family;
//! - compile-time retry/recovery hints that are intrinsic to the error
//!   identity, where those hints are safe to expose;
//! - exhaustive code iteration for diagnostics, documentation, validation,
//!   compatibility testing, and telemetry registration.
//!
//! It does NOT own:
//!
//! - error messages;
//! - error sources;
//! - error context;
//! - detection;
//! - diagnosis;
//! - recovery execution;
//! - policy decisions;
//! - hardware behavior;
//! - backend/provider behavior;
//! - QEC implementation;
//! - routing;
//! - scheduling;
//! - optimization;
//! - telemetry collection.
//!
//! Those responsibilities belong to other modules.
//!
//! # Canonical ownership
//!
//! `codes.rs` is the single source of truth for `ResilienceErrorCode`.
//!
//! Other modules MUST import this type:
//!
//! ```text
//! use crate::quantum::resilience::errors::codes::ResilienceErrorCode;
//! ```
//!
//! They MUST NOT define another resilience error-code enum.
//!
//! # Stable identifiers
//!
//! The following two representations are intentionally separate:
//!
//! 1. `u16` numeric identity — compact, deterministic, efficient for internal
//!    protocols and storage.
//!
//! 2. `QRxxx` textual identity — stable machine-readable identity suitable
//!    for logs, telemetry, diagnostics, interoperability, and external tools.
//!
//! Neither representation is a human-readable error message.
//!
//! # Compatibility rule
//!
//! Once released, a numeric value or textual identifier MUST NOT be reused for
//! a different semantic meaning.
//!
//! New codes must use previously unused numeric values.
//!
//! Removing a code from active use requires a compatibility/deprecation policy;
//! its historical identity must remain recognizable.
//!
//! # Scalability
//!
//! There is intentionally no maximum number of qubits, devices, backends,
//! jobs, operations, incidents, or resources in this file.
//!
//! Error identity is independent of machine scale.
//!
//! A single-qubit machine and a distributed fault-tolerant quantum computer
//! use exactly the same resilience error vocabulary.
//!
//! # Rust contract
//!
//! - Rust 1.97 / 1.97.1 compatible.
//! - Rust 2021 compatible.
//! - Stable Rust only.
//! - No nightly features.
//! - No `unsafe`.
//! - No allocation.
//! - No runtime initialization.
//! - No external crate dependency.
//!
//! # Integration contract
//!
//! `error.rs` should eventually contain the structured `ResilienceError`
//! and import this type rather than declaring another `ResilienceErrorCode`.
//!
//! `errors/mod.rs` should re-export this module:
//!
//! ```text
//! pub mod codes;
//! pub mod classification;
//! pub mod error;
//! ```
//!
//! and optionally:
//!
//! ```text
//! pub use codes::ResilienceErrorCode;
//! ```
//!
//! Other resilience subsystems should depend on the public error-code contract,
//! never on the implementation details of this file.
//!
//! # Why there are no qubit imports
//!
//! Error codes identify semantic failures. They do not identify resources.
//!
//! Resource-specific information belongs in `ResilienceError` context and must
//! use canonical identities such as:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Keeping those types out of this registry prevents the error-code layer from
//! becoming coupled to the quantum IR representation.
//!
//! =============================================================================
//! Compiler-enforced safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Public schema
// =============================================================================

/// Stable schema identifier for the resilience error-code contract.
pub const RESILIENCE_ERROR_CODE_SCHEMA_ID: &str =
    "zamani.quantum.resilience.error_code";

/// Semantic version of the resilience error-code schema.
///
/// This is deliberately independent from the Rust crate/package version.
///
/// Increment according to the project's compatibility policy when the
/// externally observable error-code contract changes incompatibly.
pub const RESILIENCE_ERROR_CODE_SCHEMA_VERSION: u16 = 1;

/// Prefix used by every stable resilience error identifier.
pub const RESILIENCE_ERROR_CODE_PREFIX: &str = "QR";

/// Number of decimal digits used by the public textual representation.
///
/// Examples:
///
/// ```text
/// QR001
/// QR020
/// QR191
/// ```
pub const RESILIENCE_ERROR_CODE_DIGITS: usize = 3;

// =============================================================================
// Error code
// =============================================================================

/// Stable machine-readable identity for a quantum-resilience error.
///
/// # Stability
///
/// Both the numeric value and textual representation are public compatibility
/// contracts. They MUST NOT be changed or reused after release.
///
/// # Numeric layout
///
/// Numeric ranges are intentionally grouped by subsystem family. The grouping
/// is for organization and diagnostics only; callers must branch on the enum
/// variant rather than infer semantics from arithmetic over the numeric value.
///
/// Current ranges:
///
/// ```text
/// 001–008   General / validation
/// 020–025   Detection
/// 030–033   Diagnosis
/// 040–044   Policy
/// 050–054   Planning
/// 060–068   Adaptation
/// 070–079   Recovery
/// 080–084   Mitigation
/// 090–095   Verification
/// 100–105   Resource / capability
/// 110–118   Hardware / backend
/// 120–126   QEC / quantum faults
/// 130–136   Checkpoint / persistence
/// 140–145   State / concurrency
/// 150–154   Serialization / compatibility
/// 160–164   Security / integrity
/// 170–173   Time / execution
/// 180–182   Extensibility
/// 190–191   Generic fallback
/// ```
///
/// Unassigned values remain reserved. They must not be silently assigned to
/// unrelated semantics.
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

    /// Invalid or inconsistent configuration.
    InvalidConfiguration = 3,

    /// Required information is missing.
    MissingInformation = 4,

    /// A resilience invariant was violated.
    InvariantViolation = 5,

    /// An operation is invalid in the current state.
    InvalidState = 6,

    /// Arithmetic overflow occurred.
    ArithmeticOverflow = 7,

    /// A value cannot be represented by the selected representation.
    RepresentationOverflow = 8,

    // -------------------------------------------------------------------------
    // Detection
    // -------------------------------------------------------------------------

    /// Fault or anomaly detection failed.
    DetectionFailed = 20,

    /// Detector input was invalid.
    InvalidDetectionInput = 21,

    /// Detector observations were inconsistent.
    DetectionInconsistent = 22,

    /// Required detection data is unavailable.
    DetectionDataUnavailable = 23,

    /// Detection data is stale.
    DetectionDataStale = 24,

    /// Detection could not reach a sufficiently confident conclusion.
    DetectionInconclusive = 25,

    // -------------------------------------------------------------------------
    // Diagnosis
    // -------------------------------------------------------------------------

    /// Diagnosis failed.
    DiagnosisFailed = 30,

    /// Root cause could not be determined.
    RootCauseUnknown = 31,

    /// Diagnoses conflict.
    DiagnosisConflict = 32,

    /// Evidence is insufficient for the requested diagnosis.
    InsufficientEvidence = 33,

    // -------------------------------------------------------------------------
    // Policy
    // -------------------------------------------------------------------------

    /// Policy rejected an operation.
    PolicyRejected = 40,

    /// Safety policy rejected an operation.
    SafetyPolicyRejected = 41,

    /// Resource policy rejected an operation.
    ResourcePolicyRejected = 42,

    /// A configured budget was exhausted.
    BudgetExceeded = 43,

    /// Policy requires escalation rather than autonomous continuation.
    EscalationRequired = 44,

    // -------------------------------------------------------------------------
    // Planning
    // -------------------------------------------------------------------------

    /// Recovery/adaptation planning failed.
    PlanningFailed = 50,

    /// No plan satisfies the applicable constraints.
    NoFeasiblePlan = 51,

    /// A generated plan is invalid.
    InvalidPlan = 52,

    /// A previously generated plan is no longer current.
    PlanStale = 53,

    /// No acceptable plan could be selected.
    PlanSelectionFailed = 54,

    // -------------------------------------------------------------------------
    // Adaptation
    // -------------------------------------------------------------------------

    /// Generic adaptation failure.
    AdaptationFailed = 60,

    /// Logical-to-physical remapping failed.
    RemappingFailed = 61,

    /// Physical routing failed.
    ReroutingFailed = 62,

    /// Schedule reconstruction failed.
    ReschedulingFailed = 63,

    /// Recompilation failed.
    RecompilationFailed = 64,

    /// Reoptimization failed.
    ReoptimizationFailed = 65,

    /// QEC configuration adaptation failed.
    QecAdaptationFailed = 66,

    /// Compatible backend/device selection failed.
    BackendSelectionFailed = 67,

    /// Adaptation would violate program semantics.
    SemanticAdaptationViolation = 68,

    // -------------------------------------------------------------------------
    // Recovery
    // -------------------------------------------------------------------------

    /// Generic recovery failure.
    RecoveryFailed = 70,

    /// Retry was rejected or failed.
    RetryFailed = 71,

    /// Restart failed.
    RestartFailed = 72,

    /// Resume failed.
    ResumeFailed = 73,

    /// Rollback failed.
    RollbackFailed = 74,

    /// Migration failed.
    MigrationFailed = 75,

    /// Compensating recovery failed.
    CompensationFailed = 76,

    /// Recovery preconditions were not satisfied.
    RecoveryPreconditionFailed = 77,

    /// Recovery completed but its result could not be verified.
    RecoveryVerificationFailed = 78,

    /// Recovery was explicitly aborted.
    RecoveryAborted = 79,

    // -------------------------------------------------------------------------
    // Mitigation
    // -------------------------------------------------------------------------

    /// Generic error-mitigation failure.
    MitigationFailed = 80,

    /// No suitable mitigation strategy is available.
    NoMitigationAvailable = 81,

    /// Required mitigation capability is unavailable.
    MitigationCapabilityUnavailable = 82,

    /// Mitigation exceeded an applicable budget.
    MitigationBudgetExceeded = 83,

    /// Mitigation violated a required invariant.
    MitigationInvariantViolation = 84,

    // -------------------------------------------------------------------------
    // Verification
    // -------------------------------------------------------------------------

    /// Generic verification failure.
    VerificationFailed = 90,

    /// Semantic verification failed.
    SemanticVerificationFailed = 91,

    /// Result verification failed.
    ResultVerificationFailed = 92,

    /// Verification was inconclusive.
    VerificationInconclusive = 93,

    /// Provenance verification failed.
    ProvenanceVerificationFailed = 94,

    /// Result was rejected by the verification/acceptance boundary.
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

    /// Resource ownership is conflicting.
    ResourceOwnershipConflict = 105,

    // -------------------------------------------------------------------------
    // Hardware / backend
    // -------------------------------------------------------------------------

    /// Hardware reported a failure.
    HardwareFailure = 110,

    /// Backend reported a failure.
    BackendFailure = 111,

    /// Backend rejected an operation.
    BackendRejected = 112,

    /// Backend is unavailable.
    BackendUnavailable = 113,

    /// Backend communication failed.
    BackendCommunicationFailed = 114,

    /// Backend operation timed out.
    BackendTimeout = 115,

    /// Backend returned an invalid response.
    InvalidBackendResponse = 116,

    /// Required calibration is unavailable or unusable.
    CalibrationUnavailable = 117,

    /// Hardware state changed during execution.
    HardwareStateChanged = 118,

    // -------------------------------------------------------------------------
    // QEC / quantum fault semantics
    // -------------------------------------------------------------------------

    /// QEC integration failed.
    QecFailure = 120,

    /// Required QEC capability is unavailable.
    QecCapabilityUnavailable = 121,

    /// Syndrome information is invalid.
    InvalidSyndrome = 122,

    /// Decoder failed.
    DecoderFailure = 123,

    /// A logical error could not be corrected.
    LogicalErrorUncorrectable = 124,

    /// Fault information could not be correlated with execution.
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

    /// Checkpoint is incompatible with the target.
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

    /// Synchronization failed.
    SynchronizationFailed = 142,

    /// Synchronization timed out.
    SynchronizationTimeout = 143,

    /// A resource/state lease expired.
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

    /// Compatibility requirements were not satisfied.
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

    /// Security policy rejected an action.
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

    /// A requested resilience component was unavailable.
    ComponentUnavailable = 181,

    /// A component is incompatible with the requested contract.
    ComponentIncompatible = 182,

    // -------------------------------------------------------------------------
    // Generic fallback
    // -------------------------------------------------------------------------

    /// Internal resilience failure.
    InternalError = 190,

    /// Failure could not be classified more precisely.
    Unknown = 191,
}

// =============================================================================
// Core conversion API
// =============================================================================

impl ResilienceErrorCode {
    /// Returns the stable numeric representation.
    ///
    /// This is part of the machine-readable compatibility contract.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self as u16
    }

    /// Returns the stable machine-readable textual representation.
    ///
    /// Examples:
    ///
    /// ```text
    /// QR001
    /// QR020
    /// QR191
    /// ```
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

    /// Converts a stable numeric identifier into an error code.
    ///
    /// Unknown values are rejected rather than silently mapped to
    /// [`Self::Unknown`]. This is essential for forward compatibility:
    /// a newer producer must not accidentally masquerade as a known failure
    /// when communicating with an older consumer.
    #[must_use]
    pub const fn from_u16(value: u16) -> Option<Self> {
        match value {
            1 => Some(Self::InvalidArgument),
            2 => Some(Self::InvalidIdentifier),
            3 => Some(Self::InvalidConfiguration),
            4 => Some(Self::MissingInformation),
            5 => Some(Self::InvariantViolation),
            6 => Some(Self::InvalidState),
            7 => Some(Self::ArithmeticOverflow),
            8 => Some(Self::RepresentationOverflow),

            20 => Some(Self::DetectionFailed),
            21 => Some(Self::InvalidDetectionInput),
            22 => Some(Self::DetectionInconsistent),
            23 => Some(Self::DetectionDataUnavailable),
            24 => Some(Self::DetectionDataStale),
            25 => Some(Self::DetectionInconclusive),

            30 => Some(Self::DiagnosisFailed),
            31 => Some(Self::RootCauseUnknown),
            32 => Some(Self::DiagnosisConflict),
            33 => Some(Self::InsufficientEvidence),

            40 => Some(Self::PolicyRejected),
            41 => Some(Self::SafetyPolicyRejected),
            42 => Some(Self::ResourcePolicyRejected),
            43 => Some(Self::BudgetExceeded),
            44 => Some(Self::EscalationRequired),

            50 => Some(Self::PlanningFailed),
            51 => Some(Self::NoFeasiblePlan),
            52 => Some(Self::InvalidPlan),
            53 => Some(Self::PlanStale),
            54 => Some(Self::PlanSelectionFailed),

            60 => Some(Self::AdaptationFailed),
            61 => Some(Self::RemappingFailed),
            62 => Some(Self::ReroutingFailed),
            63 => Some(Self::ReschedulingFailed),
            64 => Some(Self::RecompilationFailed),
            65 => Some(Self::ReoptimizationFailed),
            66 => Some(Self::QecAdaptationFailed),
            67 => Some(Self::BackendSelectionFailed),
            68 => Some(Self::SemanticAdaptationViolation),

            70 => Some(Self::RecoveryFailed),
            71 => Some(Self::RetryFailed),
            72 => Some(Self::RestartFailed),
            73 => Some(Self::ResumeFailed),
            74 => Some(Self::RollbackFailed),
            75 => Some(Self::MigrationFailed),
            76 => Some(Self::CompensationFailed),
            77 => Some(Self::RecoveryPreconditionFailed),
            78 => Some(Self::RecoveryVerificationFailed),
            79 => Some(Self::RecoveryAborted),

            80 => Some(Self::MitigationFailed),
            81 => Some(Self::NoMitigationAvailable),
            82 => Some(Self::MitigationCapabilityUnavailable),
            83 => Some(Self::MitigationBudgetExceeded),
            84 => Some(Self::MitigationInvariantViolation),

            90 => Some(Self::VerificationFailed),
            91 => Some(Self::SemanticVerificationFailed),
            92 => Some(Self::ResultVerificationFailed),
            93 => Some(Self::VerificationInconclusive),
            94 => Some(Self::ProvenanceVerificationFailed),
            95 => Some(Self::ResultRejected),

            100 => Some(Self::ResourceUnavailable),
            101 => Some(Self::ResourceLost),
            102 => Some(Self::CapabilityUnavailable),
            103 => Some(Self::CapabilityChanged),
            104 => Some(Self::ResourceStateChanged),
            105 => Some(Self::ResourceOwnershipConflict),

            110 => Some(Self::HardwareFailure),
            111 => Some(Self::BackendFailure),
            112 => Some(Self::BackendRejected),
            113 => Some(Self::BackendUnavailable),
            114 => Some(Self::BackendCommunicationFailed),
            115 => Some(Self::BackendTimeout),
            116 => Some(Self::InvalidBackendResponse),
            117 => Some(Self::CalibrationUnavailable),
            118 => Some(Self::HardwareStateChanged),

            120 => Some(Self::QecFailure),
            121 => Some(Self::QecCapabilityUnavailable),
            122 => Some(Self::InvalidSyndrome),
            123 => Some(Self::DecoderFailure),
            124 => Some(Self::LogicalErrorUncorrectable),
            125 => Some(Self::FaultCorrelationFailed),
            126 => Some(Self::FaultLocalizationFailed),

            130 => Some(Self::CheckpointFailed),
            131 => Some(Self::InvalidCheckpoint),
            132 => Some(Self::CheckpointIncompatible),
            133 => Some(Self::CheckpointCorrupt),
            134 => Some(Self::CheckpointIntegrityFailed),
            135 => Some(Self::CheckpointSchemaUnsupported),
            136 => Some(Self::CheckpointStorageUnavailable),

            140 => Some(Self::StateInconsistent),
            141 => Some(Self::ConcurrencyConflict),
            142 => Some(Self::SynchronizationFailed),
            143 => Some(Self::SynchronizationTimeout),
            144 => Some(Self::LeaseExpired),
            145 => Some(Self::CoordinationFailed),

            150 => Some(Self::SerializationFailed),
            151 => Some(Self::DeserializationFailed),
            152 => Some(Self::InvalidSerializedData),
            153 => Some(Self::UnsupportedSchemaVersion),
            154 => Some(Self::CompatibilityFailure),

            160 => Some(Self::AuthenticationFailed),
            161 => Some(Self::AuthorizationFailed),
            162 => Some(Self::IntegrityFailure),
            163 => Some(Self::UntrustedObservation),
            164 => Some(Self::SecurityPolicyRejected),

            170 => Some(Self::Timeout),
            171 => Some(Self::DeadlineExceeded),
            172 => Some(Self::Cancelled),
            173 => Some(Self::Interrupted),

            180 => Some(Self::ComponentFailure),
            181 => Some(Self::ComponentUnavailable),
            182 => Some(Self::ComponentIncompatible),

            190 => Some(Self::InternalError),
            191 => Some(Self::Unknown),

            _ => None,
        }
    }

    /// Returns whether this code is currently defined by this schema.
    #[must_use]
    pub const fn is_known(value: u16) -> bool {
        Self::from_u16(value).is_some()
    }

    /// Returns whether the code belongs to the general/validation family.
    #[must_use]
    pub const fn is_general(self) -> bool {
        matches!(
            self,
            Self::InvalidArgument
                | Self::InvalidIdentifier
                | Self::InvalidConfiguration
                | Self::MissingInformation
                | Self::InvariantViolation
                | Self::InvalidState
                | Self::ArithmeticOverflow
                | Self::RepresentationOverflow
        )
    }

    /// Returns whether the code belongs to fault/anomaly detection.
    #[must_use]
    pub const fn is_detection(self) -> bool {
        matches!(
            self,
            Self::DetectionFailed
                | Self::InvalidDetectionInput
                | Self::DetectionInconsistent
                | Self::DetectionDataUnavailable
                | Self::DetectionDataStale
                | Self::DetectionInconclusive
        )
    }

    /// Returns whether the code belongs to diagnosis.
    #[must_use]
    pub const fn is_diagnosis(self) -> bool {
        matches!(
            self,
            Self::DiagnosisFailed
                | Self::RootCauseUnknown
                | Self::DiagnosisConflict
                | Self::InsufficientEvidence
        )
    }

    /// Returns whether the code belongs to policy.
    #[must_use]
    pub const fn is_policy(self) -> bool {
        matches!(
            self,
            Self::PolicyRejected
                | Self::SafetyPolicyRejected
                | Self::ResourcePolicyRejected
                | Self::BudgetExceeded
                | Self::EscalationRequired
        )
    }

    /// Returns whether the code belongs to planning.
    #[must_use]
    pub const fn is_planning(self) -> bool {
        matches!(
            self,
            Self::PlanningFailed
                | Self::NoFeasiblePlan
                | Self::InvalidPlan
                | Self::PlanStale
                | Self::PlanSelectionFailed
        )
    }

    /// Returns whether the code belongs to adaptation.
    #[must_use]
    pub const fn is_adaptation(self) -> bool {
        matches!(
            self,
            Self::AdaptationFailed
                | Self::RemappingFailed
                | Self::ReroutingFailed
                | Self::ReschedulingFailed
                | Self::RecompilationFailed
                | Self::ReoptimizationFailed
                | Self::QecAdaptationFailed
                | Self::BackendSelectionFailed
                | Self::SemanticAdaptationViolation
        )
    }

    /// Returns whether the code belongs to recovery.
    #[must_use]
    pub const fn is_recovery(self) -> bool {
        matches!(
            self,
            Self::RecoveryFailed
                | Self::RetryFailed
                | Self::RestartFailed
                | Self::ResumeFailed
                | Self::RollbackFailed
                | Self::MigrationFailed
                | Self::CompensationFailed
                | Self::RecoveryPreconditionFailed
                | Self::RecoveryVerificationFailed
                | Self::RecoveryAborted
        )
    }

    /// Returns whether the code belongs to mitigation.
    #[must_use]
    pub const fn is_mitigation(self) -> bool {
        matches!(
            self,
            Self::MitigationFailed
                | Self::NoMitigationAvailable
                | Self::MitigationCapabilityUnavailable
                | Self::MitigationBudgetExceeded
                | Self::MitigationInvariantViolation
        )
    }

    /// Returns whether the code belongs to verification.
    #[must_use]
    pub const fn is_verification(self) -> bool {
        matches!(
            self,
            Self::VerificationFailed
                | Self::SemanticVerificationFailed
                | Self::ResultVerificationFailed
                | Self::VerificationInconclusive
                | Self::ProvenanceVerificationFailed
                | Self::ResultRejected
        )
    }

    /// Returns whether the code concerns resources or capabilities.
    #[must_use]
    pub const fn is_resource(self) -> bool {
        matches!(
            self,
            Self::ResourceUnavailable
                | Self::ResourceLost
                | Self::CapabilityUnavailable
                | Self::CapabilityChanged
                | Self::ResourceStateChanged
                | Self::ResourceOwnershipConflict
        )
    }

    /// Returns whether the code concerns hardware or a backend.
    #[must_use]
    pub const fn is_hardware_or_backend(self) -> bool {
        matches!(
            self,
            Self::HardwareFailure
                | Self::BackendFailure
                | Self::BackendRejected
                | Self::BackendUnavailable
                | Self::BackendCommunicationFailed
                | Self::BackendTimeout
                | Self::InvalidBackendResponse
                | Self::CalibrationUnavailable
                | Self::HardwareStateChanged
        )
    }

    /// Returns whether the code concerns QEC or canonical quantum-fault
    /// processing.
    #[must_use]
    pub const fn is_qec_or_fault(self) -> bool {
        matches!(
            self,
            Self::QecFailure
                | Self::QecCapabilityUnavailable
                | Self::InvalidSyndrome
                | Self::DecoderFailure
                | Self::LogicalErrorUncorrectable
                | Self::FaultCorrelationFailed
                | Self::FaultLocalizationFailed
        )
    }

    /// Returns whether the code concerns checkpoints or persistence.
    #[must_use]
    pub const fn is_persistence(self) -> bool {
        matches!(
            self,
            Self::CheckpointFailed
                | Self::InvalidCheckpoint
                | Self::CheckpointIncompatible
                | Self::CheckpointCorrupt
                | Self::CheckpointIntegrityFailed
                | Self::CheckpointSchemaUnsupported
                | Self::CheckpointStorageUnavailable
        )
    }

    /// Returns whether the code concerns state, synchronization, or
    /// distributed coordination.
    #[must_use]
    pub const fn is_concurrency_or_coordination(self) -> bool {
        matches!(
            self,
            Self::StateInconsistent
                | Self::ConcurrencyConflict
                | Self::SynchronizationFailed
                | Self::SynchronizationTimeout
                | Self::LeaseExpired
                | Self::CoordinationFailed
        )
    }

    /// Returns whether the code concerns serialization or compatibility.
    #[must_use]
    pub const fn is_serialization_or_compatibility(self) -> bool {
        matches!(
            self,
            Self::SerializationFailed
                | Self::DeserializationFailed
                | Self::InvalidSerializedData
                | Self::UnsupportedSchemaVersion
                | Self::CompatibilityFailure
        )
    }

    /// Returns whether the code concerns security or integrity.
    #[must_use]
    pub const fn is_security_or_integrity(self) -> bool {
        matches!(
            self,
            Self::AuthenticationFailed
                | Self::AuthorizationFailed
                | Self::IntegrityFailure
                | Self::UntrustedObservation
                | Self::SecurityPolicyRejected
        )
    }

    /// Returns whether the code concerns time or execution control.
    #[must_use]
    pub const fn is_execution_control(self) -> bool {
        matches!(
            self,
            Self::Timeout
                | Self::DeadlineExceeded
                | Self::Cancelled
                | Self::Interrupted
        )
    }

    /// Returns whether the code concerns an extensibility component.
    #[must_use]
    pub const fn is_component(self) -> bool {
        matches!(
            self,
            Self::ComponentFailure
                | Self::ComponentUnavailable
                | Self::ComponentIncompatible
        )
    }

    /// Returns whether the code is a generic fallback.
    #[must_use]
    pub const fn is_generic(self) -> bool {
        matches!(self, Self::InternalError | Self::Unknown)
    }

    /// Returns the canonical textual prefix.
    ///
    /// This exists as a compile-time helper so serialization and telemetry
    /// implementations do not duplicate the prefix.
    #[must_use]
    pub const fn prefix(self) -> &'static str {
        RESILIENCE_ERROR_CODE_PREFIX
    }

    /// Returns the broad numeric range family identifier.
    ///
    /// This is useful for compact telemetry bucketing and diagnostics.
    ///
    /// The returned value is NOT a replacement for matching on the enum
    /// variant.
    #[must_use]
    pub const fn family_number(self) -> u16 {
        match self {
            Self::InvalidArgument
            | Self::InvalidIdentifier
            | Self::InvalidConfiguration
            | Self::MissingInformation
            | Self::InvariantViolation
            | Self::InvalidState
            | Self::ArithmeticOverflow
            | Self::RepresentationOverflow => 1,

            Self::DetectionFailed
            | Self::InvalidDetectionInput
            | Self::DetectionInconsistent
            | Self::DetectionDataUnavailable
            | Self::DetectionDataStale
            | Self::DetectionInconclusive => 2,

            Self::DiagnosisFailed
            | Self::RootCauseUnknown
            | Self::DiagnosisConflict
            | Self::InsufficientEvidence => 3,

            Self::PolicyRejected
            | Self::SafetyPolicyRejected
            | Self::ResourcePolicyRejected
            | Self::BudgetExceeded
            | Self::EscalationRequired => 4,

            Self::PlanningFailed
            | Self::NoFeasiblePlan
            | Self::InvalidPlan
            | Self::PlanStale
            | Self::PlanSelectionFailed => 5,

            Self::AdaptationFailed
            | Self::RemappingFailed
            | Self::ReroutingFailed
            | Self::ReschedulingFailed
            | Self::RecompilationFailed
            | Self::ReoptimizationFailed
            | Self::QecAdaptationFailed
            | Self::BackendSelectionFailed
            | Self::SemanticAdaptationViolation => 6,

            Self::RecoveryFailed
            | Self::RetryFailed
            | Self::RestartFailed
            | Self::ResumeFailed
            | Self::RollbackFailed
            | Self::MigrationFailed
            | Self::CompensationFailed
            | Self::RecoveryPreconditionFailed
            | Self::RecoveryVerificationFailed
            | Self::RecoveryAborted => 7,

            Self::MitigationFailed
            | Self::NoMitigationAvailable
            | Self::MitigationCapabilityUnavailable
            | Self::MitigationBudgetExceeded
            | Self::MitigationInvariantViolation => 8,

            Self::VerificationFailed
            | Self::SemanticVerificationFailed
            | Self::ResultVerificationFailed
            | Self::VerificationInconclusive
            | Self::ProvenanceVerificationFailed
            | Self::ResultRejected => 9,

            Self::ResourceUnavailable
            | Self::ResourceLost
            | Self::CapabilityUnavailable
            | Self::CapabilityChanged
            | Self::ResourceStateChanged
            | Self::ResourceOwnershipConflict => 10,

            Self::HardwareFailure
            | Self::BackendFailure
            | Self::BackendRejected
            | Self::BackendUnavailable
            | Self::BackendCommunicationFailed
            | Self::BackendTimeout
            | Self::InvalidBackendResponse
            | Self::CalibrationUnavailable
            | Self::HardwareStateChanged => 11,

            Self::QecFailure
            | Self::QecCapabilityUnavailable
            | Self::InvalidSyndrome
            | Self::DecoderFailure
            | Self::LogicalErrorUncorrectable
            | Self::FaultCorrelationFailed
            | Self::FaultLocalizationFailed => 12,

            Self::CheckpointFailed
            | Self::InvalidCheckpoint
            | Self::CheckpointIncompatible
            | Self::CheckpointCorrupt
            | Self::CheckpointIntegrityFailed
            | Self::CheckpointSchemaUnsupported
            | Self::CheckpointStorageUnavailable => 13,

            Self::StateInconsistent
            | Self::ConcurrencyConflict
            | Self::SynchronizationFailed
            | Self::SynchronizationTimeout
            | Self::LeaseExpired
            | Self::CoordinationFailed => 14,

            Self::SerializationFailed
            | Self::DeserializationFailed
            | Self::InvalidSerializedData
            | Self::UnsupportedSchemaVersion
            | Self::CompatibilityFailure => 15,

            Self::AuthenticationFailed
            | Self::AuthorizationFailed
            | Self::IntegrityFailure
            | Self::UntrustedObservation
            | Self::SecurityPolicyRejected => 16,

            Self::Timeout
            | Self::DeadlineExceeded
            | Self::Cancelled
            | Self::Interrupted => 17,

            Self::ComponentFailure
            | Self::ComponentUnavailable
            | Self::ComponentIncompatible => 18,

            Self::InternalError | Self::Unknown => 19,
        }
    }

    /// Returns the first numeric code reserved for this family.
    ///
    /// This is metadata only; callers should not generate new codes by
    /// arithmetic. New public codes must be explicitly reviewed and added.
    #[must_use]
    pub const fn family_start(self) -> u16 {
        match self.family_number() {
            1 => 1,
            2 => 20,
            3 => 30,
            4 => 40,
            5 => 50,
            6 => 60,
            7 => 70,
            8 => 80,
            9 => 90,
            10 => 100,
            11 => 110,
            12 => 120,
            13 => 130,
            14 => 140,
            15 => 150,
            16 => 160,
            17 => 170,
            18 => 180,
            19 => 190,
            _ => 0,
        }
    }

    /// Returns the last numeric code currently allocated to this family.
    ///
    /// This describes the current registry, not an invitation to allocate
    /// future values automatically.
    #[must_use]
    pub const fn family_end(self) -> u16 {
        match self.family_number() {
            1 => 8,
            2 => 25,
            3 => 33,
            4 => 44,
            5 => 54,
            6 => 68,
            7 => 79,
            8 => 84,
            9 => 95,
            10 => 105,
            11 => 118,
            12 => 126,
            13 => 136,
            14 => 145,
            15 => 154,
            16 => 164,
            17 => 173,
            18 => 182,
            19 => 191,
            _ => 0,
        }
    }

    /// Returns every currently defined resilience error code.
    ///
    /// The slice is static, allocation-free, deterministic, and independent
    /// of machine size.
    ///
    /// This is useful for:
    ///
    /// - compatibility tests;
    /// - documentation generation;
    /// - telemetry registration;
    /// - schema validation;
    /// - exhaustive diagnostics;
    /// - fuzzing;
    /// - serialization tests.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::InvalidArgument,
            Self::InvalidIdentifier,
            Self::InvalidConfiguration,
            Self::MissingInformation,
            Self::InvariantViolation,
            Self::InvalidState,
            Self::ArithmeticOverflow,
            Self::RepresentationOverflow,

            Self::DetectionFailed,
            Self::InvalidDetectionInput,
            Self::DetectionInconsistent,
            Self::DetectionDataUnavailable,
            Self::DetectionDataStale,
            Self::DetectionInconclusive,

            Self::DiagnosisFailed,
            Self::RootCauseUnknown,
            Self::DiagnosisConflict,
            Self::InsufficientEvidence,

            Self::PolicyRejected,
            Self::SafetyPolicyRejected,
            Self::ResourcePolicyRejected,
            Self::BudgetExceeded,
            Self::EscalationRequired,

            Self::PlanningFailed,
            Self::NoFeasiblePlan,
            Self::InvalidPlan,
            Self::PlanStale,
            Self::PlanSelectionFailed,

            Self::AdaptationFailed,
            Self::RemappingFailed,
            Self::ReroutingFailed,
            Self::ReschedulingFailed,
            Self::RecompilationFailed,
            Self::ReoptimizationFailed,
            Self::QecAdaptationFailed,
            Self::BackendSelectionFailed,
            Self::SemanticAdaptationViolation,

            Self::RecoveryFailed,
            Self::RetryFailed,
            Self::RestartFailed,
            Self::ResumeFailed,
            Self::RollbackFailed,
            Self::MigrationFailed,
            Self::CompensationFailed,
            Self::RecoveryPreconditionFailed,
            Self::RecoveryVerificationFailed,
            Self::RecoveryAborted,

            Self::MitigationFailed,
            Self::NoMitigationAvailable,
            Self::MitigationCapabilityUnavailable,
            Self::MitigationBudgetExceeded,
            Self::MitigationInvariantViolation,

            Self::VerificationFailed,
            Self::SemanticVerificationFailed,
            Self::ResultVerificationFailed,
            Self::VerificationInconclusive,
            Self::ProvenanceVerificationFailed,
            Self::ResultRejected,

            Self::ResourceUnavailable,
            Self::ResourceLost,
            Self::CapabilityUnavailable,
            Self::CapabilityChanged,
            Self::ResourceStateChanged,
            Self::ResourceOwnershipConflict,

            Self::HardwareFailure,
            Self::BackendFailure,
            Self::BackendRejected,
            Self::BackendUnavailable,
            Self::BackendCommunicationFailed,
            Self::BackendTimeout,
            Self::InvalidBackendResponse,
            Self::CalibrationUnavailable,
            Self::HardwareStateChanged,

            Self::QecFailure,
            Self::QecCapabilityUnavailable,
            Self::InvalidSyndrome,
            Self::DecoderFailure,
            Self::LogicalErrorUncorrectable,
            Self::FaultCorrelationFailed,
            Self::FaultLocalizationFailed,

            Self::CheckpointFailed,
            Self::InvalidCheckpoint,
            Self::CheckpointIncompatible,
            Self::CheckpointCorrupt,
            Self::CheckpointIntegrityFailed,
            Self::CheckpointSchemaUnsupported,
            Self::CheckpointStorageUnavailable,

            Self::StateInconsistent,
            Self::ConcurrencyConflict,
            Self::SynchronizationFailed,
            Self::SynchronizationTimeout,
            Self::LeaseExpired,
            Self::CoordinationFailed,

            Self::SerializationFailed,
            Self::DeserializationFailed,
            Self::InvalidSerializedData,
            Self::UnsupportedSchemaVersion,
            Self::CompatibilityFailure,

            Self::AuthenticationFailed,
            Self::AuthorizationFailed,
            Self::IntegrityFailure,
            Self::UntrustedObservation,
            Self::SecurityPolicyRejected,

            Self::Timeout,
            Self::DeadlineExceeded,
            Self::Cancelled,
            Self::Interrupted,

            Self::ComponentFailure,
            Self::ComponentUnavailable,
            Self::ComponentIncompatible,

            Self::InternalError,
            Self::Unknown,
        ]
    }
}

// =============================================================================
// Standard conversion traits
// =============================================================================

impl TryFrom<u16> for ResilienceErrorCode {
    type Error = u16;

    /// Converts a stable numeric code into its canonical enum variant.
    ///
    /// Unknown values are returned unchanged. This allows callers to preserve
    /// the exact unknown value for forward-compatible diagnostics rather than
    /// losing information by mapping it to `Unknown`.
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}

impl From<ResilienceErrorCode> for u16 {
    /// Converts the canonical error code into its stable numeric identifier.
    fn from(code: ResilienceErrorCode) -> Self {
        code.as_u16()
    }
}

impl AsRef<str> for ResilienceErrorCode {
    /// Borrows the stable textual identifier.
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// =============================================================================
// Formatting
// =============================================================================

impl core::fmt::Display for ResilienceErrorCode {
    /// Displays only the stable machine-readable identifier.
    ///
    /// Human-readable descriptions belong to `ResilienceError`.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Compile-time registry invariants
// =============================================================================

const _: () = {
    assert!(RESILIENCE_ERROR_CODE_PREFIX.as_bytes()[0] == b'Q');
    assert!(RESILIENCE_ERROR_CODE_PREFIX.as_bytes()[1] == b'R');
    assert!(RESILIENCE_ERROR_CODE_DIGITS == 3);
    assert!(RESILIENCE_ERROR_CODE_SCHEMA_VERSION >= 1);
};

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_code_has_stable_numeric_identity() {
        for code in ResilienceErrorCode::all() {
            assert!(code.as_u16() > 0);
        }
    }

    #[test]
    fn every_code_has_stable_text_identity() {
        for code in ResilienceErrorCode::all() {
            let text = code.as_str();

            assert!(text.starts_with(RESILIENCE_ERROR_CODE_PREFIX));
            assert_eq!(text.len(), 5);

            let numeric = &text[2..];
            assert!(numeric.as_bytes().iter().all(u8::is_ascii_digit));
        }
    }

    #[test]
    fn numeric_and_textual_round_trip() {
        for code in ResilienceErrorCode::all() {
            let numeric = code.as_u16();

            assert_eq!(ResilienceErrorCode::from_u16(numeric), Some(*code));
            assert_eq!(ResilienceErrorCode::try_from(numeric), Ok(*code));
            assert_eq!(u16::from(*code), numeric);
        }
    }

    #[test]
    fn textual_identities_are_unique() {
        let all = ResilienceErrorCode::all();

        for (index, code) in all.iter().enumerate() {
            for other in all.iter().skip(index + 1) {
                assert_ne!(code.as_str(), other.as_str());
            }
        }
    }

    #[test]
    fn numeric_identities_are_unique() {
        let all = ResilienceErrorCode::all();

        for (index, code) in all.iter().enumerate() {
            for other in all.iter().skip(index + 1) {
                assert_ne!(code.as_u16(), other.as_u16());
            }
        }
    }

    #[test]
    fn unknown_numeric_codes_are_not_silently_reclassified() {
        assert_eq!(ResilienceErrorCode::from_u16(0), None);
        assert_eq!(ResilienceErrorCode::from_u16(9), None);
        assert_eq!(ResilienceErrorCode::from_u16(19), None);
        assert_eq!(ResilienceErrorCode::from_u16(999), None);

        assert_eq!(ResilienceErrorCode::try_from(999), Err(999));
    }

    #[test]
    fn family_classification_is_exhaustive() {
        for code in ResilienceErrorCode::all() {
            let family = code.family_number();

            assert!((1..=19).contains(&family));

            assert!(
                code.is_general()
                    || code.is_detection()
                    || code.is_diagnosis()
                    || code.is_policy()
                    || code.is_planning()
                    || code.is_adaptation()
                    || code.is_recovery()
                    || code.is_mitigation()
                    || code.is_verification()
                    || code.is_resource()
                    || code.is_hardware_or_backend()
                    || code.is_qec_or_fault()
                    || code.is_persistence()
                    || code.is_concurrency_or_coordination()
                    || code.is_serialization_or_compatibility()
                    || code.is_security_or_integrity()
                    || code.is_execution_control()
                    || code.is_component()
                    || code.is_generic()
            );
        }
    }

    #[test]
    fn family_ranges_are_consistent() {
        for code in ResilienceErrorCode::all() {
            assert!(code.as_u16() >= code.family_start());
            assert!(code.as_u16() <= code.family_end());
        }
    }

    #[test]
    fn textual_display_is_stable_identifier_only() {
        assert_eq!(
            ResilienceErrorCode::InvalidArgument.to_string(),
            "QR001"
        );
        assert_eq!(
            ResilienceErrorCode::Unknown.to_string(),
            "QR191"
        );
    }

    #[test]
    fn registry_is_non_empty_and_deterministic() {
        let first = ResilienceErrorCode::all();
        let second = ResilienceErrorCode::all();

        assert!(!first.is_empty());
        assert_eq!(first, second);
    }
}