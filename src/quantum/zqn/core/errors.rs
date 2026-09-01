//! Zamani Quantum Noise (ZQN) — Canonical Error and Diagnostic Model.
//!
//! This module defines the foundational, target-independent error vocabulary
//! for the Zamani Quantum Noise subsystem.
//!
//! # Architectural role
//!
//! `zqn::core::errors` is a dependency-low foundation layer.
//!
//! It must remain usable by every other ZQN subsystem without depending on
//! those subsystems itself.
//!
//! The error model therefore deliberately does NOT depend on:
//!
//! - `quantum::ir::qubit`;
//! - quantum gates;
//! - measurements;
//! - quantum channels;
//! - noise models;
//! - calibration;
//! - characterization;
//! - simulation;
//! - routing;
//! - scheduling;
//! - QEC;
//! - hardware;
//! - runtime;
//! - benchmarking;
//! - frontend formats;
//! - vendor SDKs;
//! - serialization implementations;
//! - external error crates.
//!
//! # Canonical ownership
//!
//! ```text
//! quantum::ir::qubit
//!     │
//!     └── owns QubitId / PhysicalQubitId
//!
//! ZQN core::errors
//!     │
//!     └── owns ZQN diagnostic vocabulary
//!
//! ZQN noise/channel/fault/etc.
//!     │
//!     └── report failures through ZqnError
//! ```
//!
//! A ZQN error may describe a failure involving a qubit, operation, channel,
//! calibration resource, or other quantum object, but this module does not
//! define those objects.
//!
//! # Why this module does not import `quantum::ir::qubit`
//!
//! The canonical IR explicitly requires `quantum::ir::qubit::QubitId` and
//! `PhysicalQubitId` to remain the sole authoritative identity types.
//!
//! Importing those types here would make the lowest ZQN diagnostic layer depend
//! on the IR quantum-domain layer. That is unnecessary for diagnostics and
//! creates avoidable coupling.
//!
//! Instead, callers pass identity information into structured diagnostic
//! context using stable textual representations or their own higher-level
//! structured context.
//!
//! Example:
//!
//! ```text
//! ZqnError::invalid_qubit("physical", "q17")
//! ```
//!
//! The caller remains responsible for obtaining and validating the canonical
//! `QubitId` from `quantum::ir::qubit`.
//!
//! # Write once, scale everywhere
//!
//! This file imposes no semantic limit on:
//!
//! - qubit count;
//! - logical resource count;
//! - physical resource count;
//! - operation count;
//! - channel dimension;
//! - noise-model size;
//! - correlation-domain size;
//! - calibration count;
//! - circuit depth;
//! - machine size;
//! - topology size;
//! - execution shots.
//!
//! Diagnostic counters use portable integer representations only where a
//! diagnostic needs to report a concrete finite value. Those representations
//! are not architectural limits on Zamani.
//!
//! # Resource safety
//!
//! Errors must remain cheap enough to construct on failure paths.
//!
//! This implementation therefore:
//!
//! - uses owned strings only where diagnostic information is actually needed;
//! - avoids hidden global state;
//! - avoids global allocation pools;
//! - avoids global caches;
//! - avoids `unsafe`;
//! - avoids recursive diagnostic structures;
//! - avoids unbounded automatic context generation;
//! - does not capture process-specific memory addresses.
//!
//! Callers that operate under explicit resource limits should reject excessive
//! work before constructing huge diagnostic payloads.
//!
//! # Determinism
//!
//! Error classification and formatting are deterministic.
//!
//! The result does not depend on:
//!
//! - memory addresses;
//! - hash-map iteration order;
//! - thread identity;
//! - process identity;
//! - random numbers;
//! - wall-clock time.
//!
//! Machine-readable consumers should use `ZqnErrorKind` and `ZqnErrorCode`.
//! Human-readable text is intentionally not a stable protocol.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Integration contract
//!
//! ```text
//! probability/*
//! channel/*
//! fault/*
//! noise/*
//! operations/*
//! calibration/*
//! characterization/*
//! simulation/*
//! propagation/*
//! target/*
//! integration/*
//! io/*
//!        │
//!        ▼
//! core/errors.rs
//!        │
//!        ▼
//! ZqnError / ZqnResult
//! ```
//!
//! No higher-level ZQN module should create a competing top-level error type
//! for ordinary ZQN failures.
//!
//! A subsystem may define a small domain-specific error enum only when it is
//! genuinely useful as a domain API, but it should provide a conversion into
//! `ZqnError` rather than creating an incompatible error hierarchy.
//!
//! # Serialization contract
//!
//! This module does not implement a wire format.
//!
//! `ZqnErrorCode`, `ZqnErrorKind`, and `ZqnErrorSeverity` provide stable
//! semantic classifications. A future serialization module may encode them
//! without requiring this file to depend on a serialization framework.
//!
//! # Versioning contract
//!
//! Error codes are part of the ZQN diagnostic protocol.
//!
//! Existing codes must not silently change meaning.
//!
//! New codes may be added in future compatible releases.
//!
//! Removing or repurposing an existing code requires a deliberate breaking
//! compatibility decision.
//!
//! # Testing contract
//!
//! This file owns tests for:
//!
//! - deterministic display;
//! - stable code formatting;
//! - category/code consistency;
//! - context preservation;
//! - source-span preservation;
//! - builder behavior;
//! - equality and hashing;
//! - `std::error::Error` integration;
//! - no accidental machine-size assumptions.
//!
//! Higher-level semantic validation belongs to the modules that own those
//! semantics.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::error::Error;
use std::fmt;

// ============================================================================
// Result aliases
// ============================================================================

/// Canonical result type for ZQN operations that can fail.
pub type ZqnResult<T> = Result<T, ZqnError>;

/// Explicit alias for APIs whose primary purpose is producing diagnostics.
///
/// This is intentionally an alias rather than a second error hierarchy.
pub type ZqnDiagnosticResult<T> = Result<T, ZqnError>;

// ============================================================================
// Severity
// ============================================================================

/// Severity of a ZQN diagnostic.
///
/// `ZqnError` normally represents an actual failure and therefore has
/// `Error` severity, but the severity field exists because the same diagnostic
/// vocabulary is also useful to validation and analysis layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ZqnErrorSeverity {
    /// Informational diagnostic.
    Info,

    /// Warning that does not necessarily prevent the requested operation.
    Warning,

    /// Failure that prevents the requested operation from completing
    /// correctly.
    Error,
}

impl Default for ZqnErrorSeverity {
    fn default() -> Self {
        Self::Error
    }
}

impl fmt::Display for ZqnErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => f.write_str("info"),
            Self::Warning => f.write_str("warning"),
            Self::Error => f.write_str("error"),
        }
    }
}

// ============================================================================
// High-level diagnostic categories
// ============================================================================

/// Stable high-level category for a ZQN diagnostic.
///
/// Categories are intentionally broader than individual implementation
/// modules. Adding a new noise model, channel representation, calibration
/// format, or future quantum technology should normally not require changing
/// this enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ZqnErrorKind {
    /// Explicit resource/security policy was violated.
    Limits,

    /// An identifier or identity reference is invalid.
    Identifier,

    /// A probability or probability distribution is invalid.
    Probability,

    /// A statistical representation is invalid.
    Statistics,

    /// A quantum-channel representation is invalid.
    Channel,

    /// A channel representation conversion failed.
    Representation,

    /// A discrete physical or abstract fault is invalid.
    Fault,

    /// A noise model is invalid.
    Noise,

    /// A noise application is invalid.
    Application,

    /// A correlation model is invalid.
    Correlation,

    /// A temporal-noise model is invalid.
    Temporal,

    /// A spatial-noise model is invalid.
    Spatial,

    /// Crosstalk semantics are invalid.
    Crosstalk,

    /// Calibration data is invalid.
    Calibration,

    /// Characterization data or protocol is invalid.
    Characterization,

    /// Simulation configuration or execution failed.
    Simulation,

    /// Sampling or stochastic execution failed.
    Sampling,

    /// Reproducibility/determinism requirements failed.
    Determinism,

    /// Error-budget or uncertainty propagation failed.
    Propagation,

    /// A target capability or target requirement is invalid.
    Target,

    /// A compatibility decision failed.
    Compatibility,

    /// Integration with another Zamani subsystem failed.
    Integration,

    /// Serialization failed.
    Serialization,

    /// Schema/version processing failed.
    Version,

    /// Provenance is invalid or incomplete where required.
    Provenance,

    /// An extension is invalid or unsupported.
    Extension,

    /// Requested functionality is outside the current contract.
    Unsupported,

    /// A ZQN invariant was violated.
    Invariant,

    /// Generic structural data is invalid.
    Structure,

    /// An internal implementation invariant failed.
    Internal,
}

impl fmt::Display for ZqnErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Limits => "limits",
            Self::Identifier => "identifier",
            Self::Probability => "probability",
            Self::Statistics => "statistics",
            Self::Channel => "channel",
            Self::Representation => "representation",
            Self::Fault => "fault",
            Self::Noise => "noise",
            Self::Application => "application",
            Self::Correlation => "correlation",
            Self::Temporal => "temporal",
            Self::Spatial => "spatial",
            Self::Crosstalk => "crosstalk",
            Self::Calibration => "calibration",
            Self::Characterization => "characterization",
            Self::Simulation => "simulation",
            Self::Sampling => "sampling",
            Self::Determinism => "determinism",
            Self::Propagation => "propagation",
            Self::Target => "target",
            Self::Compatibility => "compatibility",
            Self::Integration => "integration",
            Self::Serialization => "serialization",
            Self::Version => "version",
            Self::Provenance => "provenance",
            Self::Extension => "extension",
            Self::Unsupported => "unsupported",
            Self::Invariant => "invariant",
            Self::Structure => "structure",
            Self::Internal => "internal",
        };

        f.write_str(value)
    }
}

// ============================================================================
// Stable machine-readable error codes
// ============================================================================

/// Stable machine-readable ZQN diagnostic code.
///
/// The textual code returned by [`ZqnErrorCode::as_str`] is the public
/// diagnostic protocol identifier.
///
/// Code meanings must remain stable once released.
///
/// New semantic failures should receive new codes rather than changing the
/// meaning of an existing code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ZqnErrorCode {
    // ------------------------------------------------------------------------
    // Limits
    // ------------------------------------------------------------------------

    /// An explicit configured limit was exceeded.
    LimitExceeded,

    /// A resource-count arithmetic operation overflowed.
    ResourceOverflow,

    /// A size calculation overflowed.
    SizeOverflow,

    /// An allocation was rejected by an explicit resource policy.
    AllocationRejected,

    /// A configured operation was cancelled.
    Cancelled,

    // ------------------------------------------------------------------------
    // Identity
    // ------------------------------------------------------------------------

    /// An identifier is invalid.
    InvalidIdentifier,

    /// An identifier is duplicated.
    DuplicateIdentifier,

    /// An identifier does not belong to the expected identity domain.
    IdentityDomainMismatch,

    /// A referenced resource does not exist.
    UnknownResource,

    // ------------------------------------------------------------------------
    // Probability
    // ------------------------------------------------------------------------

    /// A probability is outside its semantic domain.
    InvalidProbability,

    /// A probability is NaN or infinite.
    NonFiniteProbability,

    /// A probability distribution is empty where emptiness is not allowed.
    EmptyDistribution,

    /// A distribution is not normalized when normalization is required.
    DistributionNotNormalized,

    /// Distribution entries are inconsistent.
    InvalidDistribution,

    /// Sampling configuration is invalid.
    InvalidSamplingConfiguration,

    // ------------------------------------------------------------------------
    // Statistics
    // ------------------------------------------------------------------------

    /// A statistical observation is invalid.
    InvalidObservation,

    /// A statistical estimate is invalid.
    InvalidEstimate,

    /// A confidence/uncertainty specification is invalid.
    InvalidUncertainty,

    /// There are insufficient observations for the requested operation.
    InsufficientObservations,

    // ------------------------------------------------------------------------
    // Channel
    // ------------------------------------------------------------------------

    /// A quantum channel is invalid.
    InvalidChannel,

    /// A channel violates complete-positivity requirements.
    ChannelNotCompletelyPositive,

    /// A channel violates trace-preservation requirements.
    ChannelNotTracePreserving,

    /// Channel dimensions are inconsistent.
    ChannelDimensionMismatch,

    /// Channel composition is invalid.
    InvalidChannelComposition,

    /// Channel tensor-product construction is invalid.
    InvalidChannelTensorProduct,

    /// A channel parameter is invalid.
    InvalidChannelParameter,

    // ------------------------------------------------------------------------
    // Representations
    // ------------------------------------------------------------------------

    /// A channel representation is invalid.
    InvalidRepresentation,

    /// Conversion between representations is unsupported.
    UnsupportedRepresentationConversion,

    /// Conversion failed because the representation cannot express the
    /// requested semantics.
    UnrepresentableChannel,

    /// Numerical conversion failed.
    RepresentationConversionFailed,

    // ------------------------------------------------------------------------
    // Faults
    // ------------------------------------------------------------------------

    /// A fault is invalid.
    InvalidFault,

    /// A fault location is invalid.
    InvalidFaultLocation,

    /// A fault classification is invalid.
    InvalidFaultClassification,

    /// A correlated-fault specification is invalid.
    InvalidCorrelatedFault,

    /// Leakage specification is invalid.
    InvalidLeakage,

    /// Erasure specification is invalid.
    InvalidErasure,

    /// Loss specification is invalid.
    InvalidLoss,

    // ------------------------------------------------------------------------
    // Noise
    // ------------------------------------------------------------------------

    /// A noise model is invalid.
    InvalidNoiseModel,

    /// A noise specification is invalid.
    InvalidNoiseSpecification,

    /// Noise cannot be applied to the requested operation/resource.
    InvalidNoiseApplication,

    /// Noise application is ambiguous.
    AmbiguousNoiseApplication,

    /// Noise requires unavailable context.
    MissingNoiseContext,

    /// A conditional noise rule is invalid.
    InvalidConditionalNoise,

    // ------------------------------------------------------------------------
    // Correlation
    // ------------------------------------------------------------------------

    /// Correlation specification is invalid.
    InvalidCorrelation,

    /// Correlation domain is invalid.
    InvalidCorrelationDomain,

    /// Correlation parameters are invalid.
    InvalidCorrelationParameter,

    /// Correlation data cannot be represented by the selected model.
    UnrepresentableCorrelation,

    // ------------------------------------------------------------------------
    // Temporal / spatial / crosstalk
    // ------------------------------------------------------------------------

    /// Temporal model is invalid.
    InvalidTemporalModel,

    /// Spatial model is invalid.
    InvalidSpatialModel,

    /// Temporal correlation data is invalid.
    InvalidTemporalCorrelation,

    /// Spatial correlation data is invalid.
    InvalidSpatialCorrelation,

    /// Crosstalk model is invalid.
    InvalidCrosstalkModel,

    /// Crosstalk dependency information is incomplete.
    MissingCrosstalkContext,

    // ------------------------------------------------------------------------
    // Calibration
    // ------------------------------------------------------------------------

    /// Calibration snapshot is invalid.
    InvalidCalibration,

    /// Calibration parameter is invalid.
    InvalidCalibrationParameter,

    /// Calibration snapshot is expired or outside its validity interval.
    CalibrationExpired,

    /// Calibration data is not valid for the requested resource.
    CalibrationResourceMismatch,

    /// Calibration versions are incompatible.
    CalibrationVersionMismatch,

    /// Calibration interpolation failed.
    CalibrationInterpolationFailed,

    // ------------------------------------------------------------------------
    // Characterization
    // ------------------------------------------------------------------------

    /// Characterization experiment is invalid.
    InvalidCharacterizationExperiment,

    /// Characterization protocol is invalid.
    InvalidCharacterizationProtocol,

    /// Characterization observation is invalid.
    InvalidCharacterizationObservation,

    /// Characterization estimation failed.
    CharacterizationEstimationFailed,

    /// Requested characterization data is unavailable.
    CharacterizationDataUnavailable,

    // ------------------------------------------------------------------------
    // Simulation / sampling
    // ------------------------------------------------------------------------

    /// Simulation configuration is invalid.
    InvalidSimulationConfiguration,

    /// Simulation cannot realize the requested semantics.
    UnsupportedSimulationSemantics,

    /// Simulation numerical evaluation failed.
    SimulationNumericalFailure,

    /// Sampling failed.
    SamplingFailed,

    /// Sampler state is invalid.
    InvalidSamplerState,

    /// Sampling stream was unexpectedly exhausted.
    SamplingExhausted,

    // ------------------------------------------------------------------------
    // Determinism
    // ------------------------------------------------------------------------

    /// Deterministic execution requirements were violated.
    DeterminismViolation,

    /// Required deterministic seed information is missing.
    MissingDeterministicSeed,

    /// Reproducibility metadata is incomplete.
    IncompleteReproducibilityContext,

    /// Parallel and sequential deterministic realization diverged.
    ParallelDeterminismMismatch,

    // ------------------------------------------------------------------------
    // Propagation
    // ------------------------------------------------------------------------

    /// Error-budget specification is invalid.
    InvalidErrorBudget,

    /// Uncertainty propagation failed.
    UncertaintyPropagationFailed,

    /// Fidelity calculation failed.
    FidelityCalculationFailed,

    /// Sensitivity analysis failed.
    SensitivityAnalysisFailed,

    /// Error accumulation model is invalid.
    InvalidAccumulationModel,

    // ------------------------------------------------------------------------
    // Target / capability
    // ------------------------------------------------------------------------

    /// Target capability description is invalid.
    InvalidTargetCapability,

    /// Target requirement is invalid.
    InvalidTargetRequirement,

    /// Requested noise semantics are incompatible with the target.
    TargetCapabilityMismatch,

    /// Target cannot faithfully represent the requested model.
    TargetCannotRepresent,

    /// Target lowering failed.
    TargetLoweringFailed,

    // ------------------------------------------------------------------------
    // Compatibility
    // ------------------------------------------------------------------------

    /// ZQN versions are incompatible.
    IncompatibleVersion,

    /// Schema versions are incompatible.
    IncompatibleSchema,

    /// Required compatibility conversion is unavailable.
    CompatibilityConversionUnavailable,

    /// An older representation cannot be safely upgraded.
    UnsupportedMigration,

    // ------------------------------------------------------------------------
    // Integration
    // ------------------------------------------------------------------------

    /// Integration with canonical quantum IR failed.
    IrIntegrationFailed,

    /// Integration with routing failed.
    RoutingIntegrationFailed,

    /// Integration with scheduling failed.
    SchedulingIntegrationFailed,

    /// Integration with QEC failed.
    QecIntegrationFailed,

    /// Integration with hardware failed.
    HardwareIntegrationFailed,

    /// Integration with runtime failed.
    RuntimeIntegrationFailed,

    /// Integration with memory/state simulation failed.
    MemoryIntegrationFailed,

    /// Integration with benchmarking failed.
    BenchmarkingIntegrationFailed,

    // ------------------------------------------------------------------------
    // Serialization
    // ------------------------------------------------------------------------

    /// Serialization failed.
    SerializationFailed,

    /// Deserialization failed.
    DeserializationFailed,

    /// Serialized data is malformed.
    MalformedSerializedData,

    /// Serialized data is truncated.
    TruncatedSerializedData,

    /// Serialized data exceeds explicit policy.
    SerializedSizeExceeded,

    /// Canonical serialization requirements were violated.
    NonCanonicalSerialization,

    // ------------------------------------------------------------------------
    // Version
    // ------------------------------------------------------------------------

    /// ZQN version is invalid.
    InvalidVersion,

    /// ZQN version is unsupported.
    UnsupportedVersion,

    /// Data uses a future version that this implementation cannot interpret.
    FutureVersion,

    // ------------------------------------------------------------------------
    // Provenance
    // ------------------------------------------------------------------------

    /// Provenance is invalid.
    InvalidProvenance,

    /// Required provenance information is missing.
    MissingProvenance,

    /// Provenance references an unknown artifact.
    UnknownProvenanceArtifact,

    // ------------------------------------------------------------------------
    // Extensions
    // ------------------------------------------------------------------------

    /// Extension is invalid.
    InvalidExtension,

    /// Extension is unsupported.
    UnsupportedExtension,

    /// Extension violates a ZQN invariant.
    ExtensionInvariantViolation,

    // ------------------------------------------------------------------------
    // Support
    // ------------------------------------------------------------------------

    /// Requested functionality is unsupported.
    UnsupportedFeature,

    /// Requested semantics cannot be represented exactly.
    UnrepresentableFeature,

    /// An explicit approximation would be required.
    ApproximationRequired,

    /// Approximation policy was not supplied.
    MissingApproximationPolicy,

    // ------------------------------------------------------------------------
    // Structure / invariants
    // ------------------------------------------------------------------------

    /// Generic structural representation is invalid.
    InvalidStructure,

    /// Required data is missing.
    MissingData,

    /// Unexpected data was supplied.
    UnexpectedData,

    /// A required invariant failed.
    InvariantViolation,

    // ------------------------------------------------------------------------
    // Internal
    // ------------------------------------------------------------------------

    /// An internal implementation invariant failed.
    InternalInvariant,

    /// An internal implementation operation failed.
    InternalFailure,
}

impl ZqnErrorCode {
    /// Returns the stable machine-readable diagnostic code.
    ///
    /// These strings are suitable for logs, telemetry, structured diagnostic
    /// output and external tooling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LimitExceeded => "ZQN-LIMIT-001",
            Self::ResourceOverflow => "ZQN-LIMIT-002",
            Self::SizeOverflow => "ZQN-LIMIT-003",
            Self::AllocationRejected => "ZQN-LIMIT-004",
            Self::Cancelled => "ZQN-LIMIT-005",

            Self::InvalidIdentifier => "ZQN-ID-001",
            Self::DuplicateIdentifier => "ZQN-ID-002",
            Self::IdentityDomainMismatch => "ZQN-ID-003",
            Self::UnknownResource => "ZQN-ID-004",

            Self::InvalidProbability => "ZQN-PROB-001",
            Self::NonFiniteProbability => "ZQN-PROB-002",
            Self::EmptyDistribution => "ZQN-PROB-003",
            Self::DistributionNotNormalized => "ZQN-PROB-004",
            Self::InvalidDistribution => "ZQN-PROB-005",
            Self::InvalidSamplingConfiguration => "ZQN-PROB-006",

            Self::InvalidObservation => "ZQN-STAT-001",
            Self::InvalidEstimate => "ZQN-STAT-002",
            Self::InvalidUncertainty => "ZQN-STAT-003",
            Self::InsufficientObservations => "ZQN-STAT-004",

            Self::InvalidChannel => "ZQN-CHANNEL-001",
            Self::ChannelNotCompletelyPositive => "ZQN-CHANNEL-002",
            Self::ChannelNotTracePreserving => "ZQN-CHANNEL-003",
            Self::ChannelDimensionMismatch => "ZQN-CHANNEL-004",
            Self::InvalidChannelComposition => "ZQN-CHANNEL-005",
            Self::InvalidChannelTensorProduct => "ZQN-CHANNEL-006",
            Self::InvalidChannelParameter => "ZQN-CHANNEL-007",

            Self::InvalidRepresentation => "ZQN-REP-001",
            Self::UnsupportedRepresentationConversion => "ZQN-REP-002",
            Self::UnrepresentableChannel => "ZQN-REP-003",
            Self::RepresentationConversionFailed => "ZQN-REP-004",

            Self::InvalidFault => "ZQN-FAULT-001",
            Self::InvalidFaultLocation => "ZQN-FAULT-002",
            Self::InvalidFaultClassification => "ZQN-FAULT-003",
            Self::InvalidCorrelatedFault => "ZQN-FAULT-004",
            Self::InvalidLeakage => "ZQN-FAULT-005",
            Self::InvalidErasure => "ZQN-FAULT-006",
            Self::InvalidLoss => "ZQN-FAULT-007",

            Self::InvalidNoiseModel => "ZQN-NOISE-001",
            Self::InvalidNoiseSpecification => "ZQN-NOISE-002",
            Self::InvalidNoiseApplication => "ZQN-NOISE-003",
            Self::AmbiguousNoiseApplication => "ZQN-NOISE-004",
            Self::MissingNoiseContext => "ZQN-NOISE-005",
            Self::InvalidConditionalNoise => "ZQN-NOISE-006",

            Self::InvalidCorrelation => "ZQN-CORR-001",
            Self::InvalidCorrelationDomain => "ZQN-CORR-002",
            Self::InvalidCorrelationParameter => "ZQN-CORR-003",
            Self::UnrepresentableCorrelation => "ZQN-CORR-004",

            Self::InvalidTemporalModel => "ZQN-TEMP-001",
            Self::InvalidSpatialModel => "ZQN-SPATIAL-001",
            Self::InvalidTemporalCorrelation => "ZQN-TEMP-002",
            Self::InvalidSpatialCorrelation => "ZQN-SPATIAL-002",
            Self::InvalidCrosstalkModel => "ZQN-XTALK-001",
            Self::MissingCrosstalkContext => "ZQN-XTALK-002",

            Self::InvalidCalibration => "ZQN-CAL-001",
            Self::InvalidCalibrationParameter => "ZQN-CAL-002",
            Self::CalibrationExpired => "ZQN-CAL-003",
            Self::CalibrationResourceMismatch => "ZQN-CAL-004",
            Self::CalibrationVersionMismatch => "ZQN-CAL-005",
            Self::CalibrationInterpolationFailed => "ZQN-CAL-006",

            Self::InvalidCharacterizationExperiment => "ZQN-CHAR-001",
            Self::InvalidCharacterizationProtocol => "ZQN-CHAR-002",
            Self::InvalidCharacterizationObservation => "ZQN-CHAR-003",
            Self::CharacterizationEstimationFailed => "ZQN-CHAR-004",
            Self::CharacterizationDataUnavailable => "ZQN-CHAR-005",

            Self::InvalidSimulationConfiguration => "ZQN-SIM-001",
            Self::UnsupportedSimulationSemantics => "ZQN-SIM-002",
            Self::SimulationNumericalFailure => "ZQN-SIM-003",
            Self::SamplingFailed => "ZQN-SIM-004",
            Self::InvalidSamplerState => "ZQN-SIM-005",
            Self::SamplingExhausted => "ZQN-SIM-006",

            Self::DeterminismViolation => "ZQN-DET-001",
            Self::MissingDeterministicSeed => "ZQN-DET-002",
            Self::IncompleteReproducibilityContext => "ZQN-DET-003",
            Self::ParallelDeterminismMismatch => "ZQN-DET-004",

            Self::InvalidErrorBudget => "ZQN-PROP-001",
            Self::UncertaintyPropagationFailed => "ZQN-PROP-002",
            Self::FidelityCalculationFailed => "ZQN-PROP-003",
            Self::SensitivityAnalysisFailed => "ZQN-PROP-004",
            Self::InvalidAccumulationModel => "ZQN-PROP-005",

            Self::InvalidTargetCapability => "ZQN-TARGET-001",
            Self::InvalidTargetRequirement => "ZQN-TARGET-002",
            Self::TargetCapabilityMismatch => "ZQN-TARGET-003",
            Self::TargetCannotRepresent => "ZQN-TARGET-004",
            Self::TargetLoweringFailed => "ZQN-TARGET-005",

            Self::IncompatibleVersion => "ZQN-COMPAT-001",
            Self::IncompatibleSchema => "ZQN-COMPAT-002",
            Self::CompatibilityConversionUnavailable => "ZQN-COMPAT-003",
            Self::UnsupportedMigration => "ZQN-COMPAT-004",

            Self::IrIntegrationFailed => "ZQN-INTEGRATION-001",
            Self::RoutingIntegrationFailed => "ZQN-INTEGRATION-002",
            Self::SchedulingIntegrationFailed => "ZQN-INTEGRATION-003",
            Self::QecIntegrationFailed => "ZQN-INTEGRATION-004",
            Self::HardwareIntegrationFailed => "ZQN-INTEGRATION-005",
            Self::RuntimeIntegrationFailed => "ZQN-INTEGRATION-006",
            Self::MemoryIntegrationFailed => "ZQN-INTEGRATION-007",
            Self::BenchmarkingIntegrationFailed => "ZQN-INTEGRATION-008",

            Self::SerializationFailed => "ZQN-SERIALIZE-001",
            Self::DeserializationFailed => "ZQN-SERIALIZE-002",
            Self::MalformedSerializedData => "ZQN-SERIALIZE-003",
            Self::TruncatedSerializedData => "ZQN-SERIALIZE-004",
            Self::SerializedSizeExceeded => "ZQN-SERIALIZE-005",
            Self::NonCanonicalSerialization => "ZQN-SERIALIZE-006",

            Self::InvalidVersion => "ZQN-VERSION-001",
            Self::UnsupportedVersion => "ZQN-VERSION-002",
            Self::FutureVersion => "ZQN-VERSION-003",

            Self::InvalidProvenance => "ZQN-PROV-001",
            Self::MissingProvenance => "ZQN-PROV-002",
            Self::UnknownProvenanceArtifact => "ZQN-PROV-003",

            Self::InvalidExtension => "ZQN-EXT-001",
            Self::UnsupportedExtension => "ZQN-EXT-002",
            Self::ExtensionInvariantViolation => "ZQN-EXT-003",

            Self::UnsupportedFeature => "ZQN-SUPPORT-001",
            Self::UnrepresentableFeature => "ZQN-SUPPORT-002",
            Self::ApproximationRequired => "ZQN-SUPPORT-003",
            Self::MissingApproximationPolicy => "ZQN-SUPPORT-004",

            Self::InvalidStructure => "ZQN-STRUCT-001",
            Self::MissingData => "ZQN-STRUCT-002",
            Self::UnexpectedData => "ZQN-STRUCT-003",
            Self::InvariantViolation => "ZQN-STRUCT-004",

            Self::InternalInvariant => "ZQN-INTERNAL-001",
            Self::InternalFailure => "ZQN-INTERNAL-002",
        }
    }
}

impl fmt::Display for ZqnErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Source span
// ============================================================================

/// Source or generated-artifact location associated with a diagnostic.
///
/// ZQN itself does not parse source languages, so this structure is deliberately
/// language-independent.
///
/// A frontend can translate its own source location into this structure when
/// propagating an error into ZQN.
///
/// `start` and `end` are byte offsets in the associated source/artifact.
///
/// The optional `line` and `column` fields are convenience information. They
/// are not used to determine semantic identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceSpan {
    /// Inclusive byte offset of the start of the span.
    pub start: u64,

    /// Exclusive byte offset of the end of the span.
    pub end: u64,

    /// Optional one-based line number.
    pub line: Option<u64>,

    /// Optional one-based column number.
    pub column: Option<u64>,
}

impl SourceSpan {
    /// Creates a span from byte offsets.
    ///
    /// Returns `None` when `end < start`.
    pub const fn new(start: u64, end: u64) -> Option<Self> {
        if end < start {
            None
        } else {
            Some(Self {
                start,
                end,
                line: None,
                column: None,
            })
        }
    }

    /// Creates a span with line/column information.
    ///
    /// Returns `None` when `end < start`.
    pub const fn with_location(
        start: u64,
        end: u64,
        line: u64,
        column: u64,
    ) -> Option<Self> {
        if end < start {
            None
        } else {
            Some(Self {
                start,
                end,
                line: Some(line),
                column: Some(column),
            })
        }
    }

    /// Returns the byte length of the span.
    pub const fn len(self) -> u64 {
        self.end - self.start
    }

    /// Returns whether the span is empty.
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

impl fmt::Display for SourceSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)?;

        match (self.line, self.column) {
            (Some(line), Some(column)) => {
                write!(f, " (line {line}, column {column})")
            }
            (Some(line), None) => {
                write!(f, " (line {line})")
            }
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Structured diagnostic context
// ============================================================================

/// A deterministic key/value diagnostic context entry.
///
/// This is intentionally a simple ordered entry rather than a hash map.
/// Ordering is part of deterministic diagnostic formatting.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ZqnErrorContext {
    key: String,
    value: String,
}

impl ZqnErrorContext {
    /// Creates a context entry.
    ///
    /// Empty keys are rejected because they make structured diagnostics
    /// ambiguous.
    pub fn new<K, V>(key: K, value: V) -> Option<Self>
    where
        K: Into<String>,
        V: Into<String>,
    {
        let key = key.into();

        if key.is_empty() {
            return None;
        }

        Some(Self {
            key,
            value: value.into(),
        })
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

impl fmt::Display for ZqnErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

// ============================================================================
// Main error type
// ============================================================================

/// Canonical ZQN error.
///
/// This is the single foundational error representation shared by the ZQN
/// subsystem.
///
/// The structure deliberately contains only diagnostic information. It does
/// not own the object that failed and does not retain references to arbitrary
/// runtime state.
///
/// This makes it:
///
/// - `Send`;
/// - `Sync`;
/// - `Clone`;
/// - `Eq`;
/// - `Hash`;
/// - deterministic;
/// - safe to move across execution boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ZqnError {
    kind: ZqnErrorKind,
    code: ZqnErrorCode,
    severity: ZqnErrorSeverity,
    message: String,
    span: Option<SourceSpan>,
    context: Vec<ZqnErrorContext>,
}

impl ZqnError {
    /// Creates a canonical error.
    pub fn new<M>(
        kind: ZqnErrorKind,
        code: ZqnErrorCode,
        message: M,
    ) -> Self
    where
        M: Into<String>,
    {
        Self {
            kind,
            code,
            severity: ZqnErrorSeverity::Error,
            message: message.into(),
            span: None,
            context: Vec::new(),
        }
    }

    /// Creates an informational diagnostic.
    pub fn info<M>(
        kind: ZqnErrorKind,
        code: ZqnErrorCode,
        message: M,
    ) -> Self
    where
        M: Into<String>,
    {
        Self {
            kind,
            code,
            severity: ZqnErrorSeverity::Info,
            message: message.into(),
            span: None,
            context: Vec::new(),
        }
    }

    /// Creates a warning diagnostic.
    pub fn warning<M>(
        kind: ZqnErrorKind,
        code: ZqnErrorCode,
        message: M,
    ) -> Self
    where
        M: Into<String>,
    {
        Self {
            kind,
            code,
            severity: ZqnErrorSeverity::Warning,
            message: message.into(),
            span: None,
            context: Vec::new(),
        }
    }

    /// Returns the high-level error category.
    pub const fn kind(&self) -> ZqnErrorKind {
        self.kind
    }

    /// Returns the stable machine-readable error code.
    pub const fn code(&self) -> ZqnErrorCode {
        self.code
    }

    /// Returns the stable textual error code.
    pub const fn code_str(&self) -> &'static str {
        self.code.as_str()
    }

    /// Returns the diagnostic severity.
    pub const fn severity(&self) -> ZqnErrorSeverity {
        self.severity
    }

    /// Returns the human-readable diagnostic message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the optional source/artifact span.
    pub const fn span(&self) -> Option<SourceSpan> {
        self.span
    }

    /// Returns structured diagnostic context in deterministic insertion order.
    pub fn context(&self) -> &[ZqnErrorContext] {
        &self.context
    }

    /// Attaches a source/artifact span.
    pub fn with_span(mut self, span: SourceSpan) -> Self {
        self.span = Some(span);
        self
    }

    /// Attaches a context entry.
    ///
    /// Empty keys are ignored rather than creating malformed diagnostics.
    pub fn with_context<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        if let Some(entry) = ZqnErrorContext::new(key, value) {
            self.context.push(entry);
        }

        self
    }

    /// Attaches a context entry from an already constructed value.
    pub fn with_context_entry(mut self, entry: ZqnErrorContext) -> Self {
        self.context.push(entry);
        self
    }

    /// Adds an identity-domain context.
    ///
    /// This is intentionally textual so that this foundational module does
    /// not depend on `quantum::ir::qubit` or any other identity owner.
    pub fn with_identity<S>(self, domain: &str, identity: S) -> Self
    where
        S: Into<String>,
    {
        self.with_context("identity_domain", domain)
            .with_context("identity", identity)
    }

    /// Adds a resource name/context value.
    pub fn with_resource<S>(self, resource: S) -> Self
    where
        S: Into<String>,
    {
        self.with_context("resource", resource)
    }

    /// Adds an operation identity/context value.
    pub fn with_operation<S>(self, operation: S) -> Self
    where
        S: Into<String>,
    {
        self.with_context("operation", operation)
    }

    /// Adds a model identity/context value.
    pub fn with_model<S>(self, model: S) -> Self
    where
        S: Into<String>,
    {
        self.with_context("model", model)
    }

    /// Adds a target identity/context value.
    pub fn with_target<S>(self, target: S) -> Self
    where
        S: Into<String>,
    {
        self.with_context("target", target)
    }

    /// Adds a calibration identity/context value.
    pub fn with_calibration<S>(self, calibration: S) -> Self
    where
        S: Into<String>,
    {
        self.with_context("calibration", calibration)
    }

    /// Adds a correlation-domain context value.
    pub fn with_correlation<S>(self, correlation: S) -> Self
    where
        S: Into<String>,
    {
        self.with_context("correlation", correlation)
    }

    /// Adds a numerical-value context.
    pub fn with_value<S>(self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.with_context("value", value)
    }

    /// Returns whether this diagnostic represents an error.
    pub const fn is_error(&self) -> bool {
        matches!(self.severity, ZqnErrorSeverity::Error)
    }

    /// Returns whether this diagnostic represents a warning.
    pub const fn is_warning(&self) -> bool {
        matches!(self.severity, ZqnErrorSeverity::Warning)
    }

    /// Returns whether this diagnostic represents informational output.
    pub const fn is_info(&self) -> bool {
        matches!(self.severity, ZqnErrorSeverity::Info)
    }

    // ========================================================================
    // Common constructors — limits
    // ========================================================================

    /// Creates a limit-exceeded error.
    pub fn limit_exceeded<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::LimitExceeded,
            message,
        )
    }

    /// Creates a resource-overflow error.
    pub fn resource_overflow<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::ResourceOverflow,
            message,
        )
    }

    /// Creates a size-overflow error.
    pub fn size_overflow<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::SizeOverflow,
            message,
        )
    }

    /// Creates an allocation-policy rejection.
    pub fn allocation_rejected<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::AllocationRejected,
            message,
        )
    }

    /// Creates a cancellation diagnostic.
    pub fn cancelled<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Limits,
            ZqnErrorCode::Cancelled,
            message,
        )
    }

    // ========================================================================
    // Common constructors — identifiers
    // ========================================================================

    /// Creates an invalid-identifier error.
    pub fn invalid_identifier<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Identifier,
            ZqnErrorCode::InvalidIdentifier,
            message,
        )
    }

    /// Creates a duplicate-identifier error.
    pub fn duplicate_identifier<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Identifier,
            ZqnErrorCode::DuplicateIdentifier,
            message,
        )
    }

    /// Creates an identity-domain mismatch.
    pub fn identity_domain_mismatch<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Identifier,
            ZqnErrorCode::IdentityDomainMismatch,
            message,
        )
    }

    /// Creates an unknown-resource error.
    pub fn unknown_resource<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Identifier,
            ZqnErrorCode::UnknownResource,
            message,
        )
    }

    // ========================================================================
    // Common constructors — probability/statistics
    // ========================================================================

    /// Creates an invalid-probability error.
    pub fn invalid_probability<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Probability,
            ZqnErrorCode::InvalidProbability,
            message,
        )
    }

    /// Creates a non-finite-probability error.
    pub fn non_finite_probability<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Probability,
            ZqnErrorCode::NonFiniteProbability,
            message,
        )
    }

    /// Creates an invalid-distribution error.
    pub fn invalid_distribution<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Probability,
            ZqnErrorCode::InvalidDistribution,
            message,
        )
    }

    /// Creates a non-normalized-distribution error.
    pub fn distribution_not_normalized<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Probability,
            ZqnErrorCode::DistributionNotNormalized,
            message,
        )
    }

    /// Creates an invalid-observation error.
    pub fn invalid_observation<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Statistics,
            ZqnErrorCode::InvalidObservation,
            message,
        )
    }

    /// Creates an invalid-estimate error.
    pub fn invalid_estimate<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Statistics,
            ZqnErrorCode::InvalidEstimate,
            message,
        )
    }

    /// Creates an invalid-uncertainty error.
    pub fn invalid_uncertainty<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Statistics,
            ZqnErrorCode::InvalidUncertainty,
            message,
        )
    }

    // ========================================================================
    // Common constructors — channels
    // ========================================================================

    /// Creates an invalid-channel error.
    pub fn invalid_channel<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Channel,
            ZqnErrorCode::InvalidChannel,
            message,
        )
    }

    /// Creates a complete-positivity failure.
    pub fn channel_not_completely_positive<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Channel,
            ZqnErrorCode::ChannelNotCompletelyPositive,
            message,
        )
    }

    /// Creates a trace-preservation failure.
    pub fn channel_not_trace_preserving<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Channel,
            ZqnErrorCode::ChannelNotTracePreserving,
            message,
        )
    }

    /// Creates a channel-dimension mismatch.
    pub fn channel_dimension_mismatch<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Channel,
            ZqnErrorCode::ChannelDimensionMismatch,
            message,
        )
    }

    /// Creates an invalid-channel-composition error.
    pub fn invalid_channel_composition<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Channel,
            ZqnErrorCode::InvalidChannelComposition,
            message,
        )
    }

    /// Creates an invalid channel representation error.
    pub fn invalid_representation<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Representation,
            ZqnErrorCode::InvalidRepresentation,
            message,
        )
    }

    /// Creates an unsupported representation-conversion error.
    pub fn unsupported_representation_conversion<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Representation,
            ZqnErrorCode::UnsupportedRepresentationConversion,
            message,
        )
    }

    /// Creates an unrepresentable-channel error.
    pub fn unrepresentable_channel<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Representation,
            ZqnErrorCode::UnrepresentableChannel,
            message,
        )
    }

    // ========================================================================
    // Common constructors — faults/noise
    // ========================================================================

    /// Creates an invalid-fault error.
    pub fn invalid_fault<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Fault,
            ZqnErrorCode::InvalidFault,
            message,
        )
    }

    /// Creates an invalid-fault-location error.
    pub fn invalid_fault_location<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Fault,
            ZqnErrorCode::InvalidFaultLocation,
            message,
        )
    }

    /// Creates an invalid-correlated-fault error.
    pub fn invalid_correlated_fault<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Fault,
            ZqnErrorCode::InvalidCorrelatedFault,
            message,
        )
    }

    /// Creates an invalid-noise-model error.
    pub fn invalid_noise_model<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Noise,
            ZqnErrorCode::InvalidNoiseModel,
            message,
        )
    }

    /// Creates an invalid-noise-specification error.
    pub fn invalid_noise_specification<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Noise,
            ZqnErrorCode::InvalidNoiseSpecification,
            message,
        )
    }

    /// Creates an invalid-noise-application error.
    pub fn invalid_noise_application<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Application,
            ZqnErrorCode::InvalidNoiseApplication,
            message,
        )
    }

    /// Creates an ambiguous-noise-application error.
    pub fn ambiguous_noise_application<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Application,
            ZqnErrorCode::AmbiguousNoiseApplication,
            message,
        )
    }

    /// Creates a missing-noise-context error.
    pub fn missing_noise_context<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Application,
            ZqnErrorCode::MissingNoiseContext,
            message,
        )
    }

    /// Creates an invalid-correlation error.
    pub fn invalid_correlation<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Correlation,
            ZqnErrorCode::InvalidCorrelation,
            message,
        )
    }

    // ========================================================================
    // Common constructors — calibration/characterization
    // ========================================================================

    /// Creates an invalid-calibration error.
    pub fn invalid_calibration<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Calibration,
            ZqnErrorCode::InvalidCalibration,
            message,
        )
    }

    /// Creates an expired-calibration error.
    pub fn calibration_expired<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Calibration,
            ZqnErrorCode::CalibrationExpired,
            message,
        )
    }

    /// Creates a calibration-resource mismatch.
    pub fn calibration_resource_mismatch<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Calibration,
            ZqnErrorCode::CalibrationResourceMismatch,
            message,
        )
    }

    /// Creates an invalid-characterization experiment error.
    pub fn invalid_characterization_experiment<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Characterization,
            ZqnErrorCode::InvalidCharacterizationExperiment,
            message,
        )
    }

    /// Creates a characterization-estimation failure.
    pub fn characterization_estimation_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Characterization,
            ZqnErrorCode::CharacterizationEstimationFailed,
            message,
        )
    }

    // ========================================================================
    // Common constructors — simulation/determinism
    // ========================================================================

    /// Creates a simulation configuration error.
    pub fn invalid_simulation_configuration<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Simulation,
            ZqnErrorCode::InvalidSimulationConfiguration,
            message,
        )
    }

    /// Creates a simulation numerical failure.
    pub fn simulation_numerical_failure<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Simulation,
            ZqnErrorCode::SimulationNumericalFailure,
            message,
        )
    }

    /// Creates a sampling failure.
    pub fn sampling_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Sampling,
            ZqnErrorCode::SamplingFailed,
            message,
        )
    }

    /// Creates a deterministic-execution violation.
    pub fn determinism_violation<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Determinism,
            ZqnErrorCode::DeterminismViolation,
            message,
        )
    }

    /// Creates a missing-seed error.
    pub fn missing_deterministic_seed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Determinism,
            ZqnErrorCode::MissingDeterministicSeed,
            message,
        )
    }

    // ========================================================================
    // Common constructors — propagation
    // ========================================================================

    /// Creates an invalid error-budget error.
    pub fn invalid_error_budget<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Propagation,
            ZqnErrorCode::InvalidErrorBudget,
            message,
        )
    }

    /// Creates an uncertainty-propagation failure.
    pub fn uncertainty_propagation_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Propagation,
            ZqnErrorCode::UncertaintyPropagationFailed,
            message,
        )
    }

    // ========================================================================
    // Common constructors — target/integration
    // ========================================================================

    /// Creates a target-capability mismatch.
    pub fn target_capability_mismatch<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Target,
            ZqnErrorCode::TargetCapabilityMismatch,
            message,
        )
    }

    /// Creates a target representation failure.
    pub fn target_cannot_represent<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Target,
            ZqnErrorCode::TargetCannotRepresent,
            message,
        )
    }

    /// Creates an IR integration error.
    pub fn ir_integration_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Integration,
            ZqnErrorCode::IrIntegrationFailed,
            message,
        )
    }

    /// Creates a QEC integration error.
    pub fn qec_integration_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Integration,
            ZqnErrorCode::QecIntegrationFailed,
            message,
        )
    }

    /// Creates a hardware integration error.
    pub fn hardware_integration_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Integration,
            ZqnErrorCode::HardwareIntegrationFailed,
            message,
        )
    }

    /// Creates a runtime integration error.
    pub fn runtime_integration_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Integration,
            ZqnErrorCode::RuntimeIntegrationFailed,
            message,
        )
    }

    // ========================================================================
    // Common constructors — serialization/versioning
    // ========================================================================

    /// Creates a serialization failure.
    pub fn serialization_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Serialization,
            ZqnErrorCode::SerializationFailed,
            message,
        )
    }

    /// Creates a deserialization failure.
    pub fn deserialization_failed<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Serialization,
            ZqnErrorCode::DeserializationFailed,
            message,
        )
    }

    /// Creates a malformed-data failure.
    pub fn malformed_serialized_data<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Serialization,
            ZqnErrorCode::MalformedSerializedData,
            message,
        )
    }

    /// Creates an incompatible-version failure.
    pub fn incompatible_version<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Version,
            ZqnErrorCode::IncompatibleVersion,
            message,
        )
    }

    /// Creates an unsupported-version failure.
    pub fn unsupported_version<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Version,
            ZqnErrorCode::UnsupportedVersion,
            message,
        )
    }

    // ========================================================================
    // Common constructors — support/invariants
    // ========================================================================

    /// Creates an unsupported-feature error.
    pub fn unsupported_feature<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Unsupported,
            ZqnErrorCode::UnsupportedFeature,
            message,
        )
    }

    /// Creates an unrepresentable-feature error.
    pub fn unrepresentable_feature<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Unsupported,
            ZqnErrorCode::UnrepresentableFeature,
            message,
        )
    }

    /// Creates an explicit-approximation-required error.
    pub fn approximation_required<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Unsupported,
            ZqnErrorCode::ApproximationRequired,
            message,
        )
    }

    /// Creates an invariant violation.
    pub fn invariant_violation<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Invariant,
            ZqnErrorCode::InvariantViolation,
            message,
        )
    }

    /// Creates a generic structural error.
    pub fn invalid_structure<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Structure,
            ZqnErrorCode::InvalidStructure,
            message,
        )
    }

    /// Creates an internal invariant failure.
    pub fn internal_invariant<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Internal,
            ZqnErrorCode::InternalInvariant,
            message,
        )
    }

    /// Creates an internal implementation failure.
    pub fn internal_failure<M>(message: M) -> Self
    where
        M: Into<String>,
    {
        Self::new(
            ZqnErrorKind::Internal,
            ZqnErrorCode::InternalFailure,
            message,
        )
    }
}

impl fmt::Display for ZqnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.code(),
            self.kind(),
            self.message
        )?;

        if let Some(span) = self.span {
            write!(f, " @ {span}")?;
        }

        if !self.context.is_empty() {
            f.write_str(" [")?;

            for (index, entry) in self.context.iter().enumerate() {
                if index != 0 {
                    f.write_str(", ")?;
                }

                entry.fmt(f)?;
            }

            f.write_str("]")?;
        }

        Ok(())
    }
}

impl Error for ZqnError {}

// ============================================================================
// Conversion helpers
// ============================================================================

/// Converts an arbitrary standard-library error into a structured ZQN
/// integration error without retaining the source object.
///
/// The original error's display representation is copied into the diagnostic.
///
/// This intentionally avoids storing `Box<dyn Error>` so that `ZqnError` can
/// remain `Clone + Eq + Hash + Send + Sync` and deterministic.
pub fn external_error<E>(kind: ZqnErrorKind, code: ZqnErrorCode, error: E) -> ZqnError
where
    E: Error,
{
    ZqnError::new(kind, code, error.to_string())
}

/// Converts an arbitrary displayable failure into a ZQN integration error.
///
/// This is useful for dependencies whose error type does not implement
/// `std::error::Error`.
pub fn external_display_error<E>(
    kind: ZqnErrorKind,
    code: ZqnErrorCode,
    error: E,
) -> ZqnError
where
    E: fmt::Display,
{
    ZqnError::new(kind, code, error.to_string())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_alias_uses_canonical_error() {
        let result: ZqnResult<()> = Err(ZqnError::invalid_probability(
            "probability is outside [0, 1]",
        ));

        assert!(result.is_err());
    }

    #[test]
    fn error_code_is_stable() {
        assert_eq!(
            ZqnErrorCode::InvalidProbability.as_str(),
            "ZQN-PROB-001"
        );

        assert_eq!(
            ZqnErrorCode::InvalidChannel.as_str(),
            "ZQN-CHANNEL-001"
        );

        assert_eq!(
            ZqnErrorCode::TargetCapabilityMismatch.as_str(),
            "ZQN-TARGET-003"
        );
    }

    #[test]
    fn display_is_deterministic() {
        let error = ZqnError::invalid_probability("invalid probability")
            .with_context("parameter", "p")
            .with_context("value", "1.5");

        let first = error.to_string();
        let second = error.to_string();

        assert_eq!(first, second);
        assert_eq!(
            first,
            "[ZQN-PROB-001] probability: invalid probability \
             [parameter=p, value=1.5]"
        );
    }

    #[test]
    fn source_span_rejects_reversed_ranges() {
        assert!(SourceSpan::new(10, 5).is_none());
        assert!(SourceSpan::new(10, 10).is_some());
        assert!(SourceSpan::new(10, 15).is_some());
    }

    #[test]
    fn source_span_length_is_safe() {
        let span = SourceSpan::new(10, 25).expect("valid span");

        assert_eq!(span.len(), 15);
        assert!(!span.is_empty());
    }

    #[test]
    fn context_rejects_empty_keys() {
        assert!(ZqnErrorContext::new("", "value").is_none());
        assert!(ZqnErrorContext::new("key", "value").is_some());
    }

    #[test]
    fn context_order_is_preserved() {
        let error = ZqnError::invalid_noise_model("invalid model")
            .with_context("first", "1")
            .with_context("second", "2")
            .with_context("third", "3");

        assert_eq!(error.context()[0].key(), "first");
        assert_eq!(error.context()[1].key(), "second");
        assert_eq!(error.context()[2].key(), "third");
    }

    #[test]
    fn identity_context_does_not_define_a_second_qubit_type() {
        let error = ZqnError::invalid_fault("invalid physical fault")
            .with_identity("physical_qubit", "q17");

        assert_eq!(
            error.context()[0].key(),
            "identity_domain"
        );
        assert_eq!(
            error.context()[0].value(),
            "physical_qubit"
        );
        assert_eq!(
            error.context()[1].key(),
            "identity"
        );
        assert_eq!(
            error.context()[1].value(),
            "q17"
        );
    }

    #[test]
    fn error_is_std_error() {
        let error = ZqnError::invalid_channel("invalid channel");
        let standard_error: &dyn Error = &error;

        assert_eq!(standard_error.to_string(), error.to_string());
    }

    #[test]
    fn error_is_cloneable_and_comparable() {
        let error = ZqnError::invalid_noise_model("invalid")
            .with_model("model-a")
            .with_target("target-a");

        let clone = error.clone();

        assert_eq!(error, clone);
    }

    #[test]
    fn severity_helpers_are_correct() {
        let error = ZqnError::invalid_fault("fault");

        let warning = ZqnError::warning(
            ZqnErrorKind::Noise,
            ZqnErrorCode::InvalidNoiseModel,
            "warning",
        );

        let info = ZqnError::info(
            ZqnErrorKind::Noise,
            ZqnErrorCode::InvalidNoiseModel,
            "info",
        );

        assert!(error.is_error());
        assert!(!error.is_warning());
        assert!(!error.is_info());

        assert!(!warning.is_error());
        assert!(warning.is_warning());
        assert!(!warning.is_info());

        assert!(!info.is_error());
        assert!(!info.is_warning());
        assert!(info.is_info());
    }

    #[test]
    fn common_constructors_have_matching_categories_and_codes() {
        let probability = ZqnError::invalid_probability("bad");

        assert_eq!(
            probability.kind(),
            ZqnErrorKind::Probability
        );
        assert_eq!(
            probability.code(),
            ZqnErrorCode::InvalidProbability
        );

        let channel = ZqnError::invalid_channel("bad");

        assert_eq!(channel.kind(), ZqnErrorKind::Channel);
        assert_eq!(channel.code(), ZqnErrorCode::InvalidChannel);

        let noise = ZqnError::invalid_noise_model("bad");

        assert_eq!(noise.kind(), ZqnErrorKind::Noise);
        assert_eq!(noise.code(), ZqnErrorCode::InvalidNoiseModel);

        let calibration = ZqnError::invalid_calibration("bad");

        assert_eq!(calibration.kind(), ZqnErrorKind::Calibration);
        assert_eq!(
            calibration.code(),
            ZqnErrorCode::InvalidCalibration
        );
    }

    #[test]
    fn display_with_span_is_deterministic() {
        let span = SourceSpan::with_location(10, 20, 4, 7)
            .expect("valid span");

        let error = ZqnError::invalid_noise_model("bad")
            .with_span(span);

        assert_eq!(
            error.to_string(),
            "[ZQN-NOISE-001] noise: bad @ 10..20 \
             (line 4, column 7)"
        );
    }

    #[test]
    fn external_display_error_is_structured() {
        let error = external_display_error(
            ZqnErrorKind::Integration,
            ZqnErrorCode::RuntimeIntegrationFailed,
            "runtime unavailable",
        );

        assert_eq!(
            error.kind(),
            ZqnErrorKind::Integration
        );

        assert_eq!(
            error.code(),
            ZqnErrorCode::RuntimeIntegrationFailed
        );

        assert_eq!(error.message(), "runtime unavailable");
    }

    #[test]
    fn no_architectural_machine_size_is_encoded() {
        // This test intentionally verifies the design contract rather than a
        // numeric maximum. ZQN errors describe concrete failures supplied by
        // callers; this foundational module does not define machine capacity.
        let error = ZqnError::limit_exceeded(
            "configured execution policy rejected the requested operation",
        );

        assert_eq!(error.kind(), ZqnErrorKind::Limits);
        assert_eq!(
            error.code(),
            ZqnErrorCode::LimitExceeded
        );
    }
}