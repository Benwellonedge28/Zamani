//! # ZQN Process Characterization
//!
//! Production process-level characterization contracts for Zamani Quantum
//! Noise (ZQN).
//!
//! ## Ownership
//!
//! This file owns the semantic contract for CHARACTERIZING a quantum process.
//!
//! It owns:
//!
//! - process-characterization configuration;
//! - process-characterization objectives;
//! - process-characterization scope;
//! - process experiment descriptors;
//! - process input/output setting identities;
//! - process observation references;
//! - process-level sufficient statistics;
//! - process-characterization accumulation;
//! - process-characterization results;
//! - process quality diagnostics;
//! - process-identification metadata;
//! - process characterization validation;
//! - deterministic process-result aggregation;
//! - explicit approximation/identifiability status;
//! - streaming-compatible process characterization contracts.
//!
//! ## Does not own
//!
//! This file deliberately does NOT own:
//!
//! - canonical quantum IR;
//! - quantum circuit construction;
//! - source-language parsing;
//! - raw observation storage;
//! - measurement histogram representation;
//! - statistical estimator implementation;
//! - confidence-interval mathematics;
//! - tomography reconstruction mathematics;
//! - quantum-channel representations;
//! - Kraus/Choi/PTM implementations;
//! - calibration storage;
//! - hardware communication;
//! - simulator execution;
//! - routing;
//! - scheduling;
//! - QEC;
//! - benchmarking methodology;
//! - vendor APIs;
//! - serialization wire formats;
//! - random-number generation.
//!
//! Those responsibilities belong to their respective modules.
//!
//! ## Architectural position
//!
//! ```text
//!                         Zamani source
//!                              |
//!                              v
//!                         quantum::ir
//!                              |
//!                              v
//!                 characterization::protocol
//!                              |
//!                              v
//!                 characterization::experiment
//!                              |
//!                    execution / hardware
//!                       /             \
//!                      v               v
//!                observation       simulator
//!                      |
//!                      v
//!              process_characterization
//!                      |
//!             +--------+---------+
//!             |                  |
//!             v                  v
//!         estimator          tomography
//!             |                  |
//!             +--------+---------+
//!                      |
//!                      v
//!             ProcessCharacterizationResult
//!                      |
//!              +-------+-------+
//!              |               |
//!              v               v
//!             ZQN          calibration
//!              |
//!              +----> routing / scheduling / QEC / benchmarking
//! ```
//!
//! The key architectural rule is:
//!
//! > This file describes what was learned about a process. It does not decide
//! > how the process was physically executed or mathematically reconstructed.
//!
//! ## Process characterization versus process tomography
//!
//! Process characterization is broader than process tomography.
//!
//! It may produce:
//!
//! - process error rates;
//! - transition probabilities;
//! - fidelity estimates;
//! - drift estimates;
//! - process parameters;
//! - transfer characteristics;
//! - temporal correlations;
//! - crosstalk characteristics;
//! - process-model parameters;
//! - a reconstructed channel;
//! - a reconstructed process matrix;
//! - a characterization sufficient for a particular noise model.
//!
//! Tomography is therefore a consumer/producer relationship rather than an
//! ownership relationship:
//!
//! ```text
//! process characterization
//!          |
//!          +--> tomography
//!          |
//!          +--> randomized protocols
//!          |
//!          +--> direct parameter estimation
//!          |
//!          +--> spectroscopy
//!          |
//!          +--> calibration characterization
//! ```
//!
//! ## Canonical identity boundary
//!
//! ZQN does not define another quantum-resource identity system.
//!
//! Logical and physical qubits use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! ZQN characterization identities use:
//!
//! ```text
//! crate::quantum::zqn::core::ids
//! ```
//!
//! In particular, this file does not define:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `ObservationId`;
//! - `ExperimentId`;
//! - `CharacterizationId`.
//!
//! ## Scalability
//!
//! There is no semantic upper bound on:
//!
//! - number of characterized resources;
//! - process arity;
//! - number of input settings;
//! - number of output settings;
//! - number of experiments;
//! - number of observations;
//! - number of shots;
//! - process duration;
//! - process parameters;
//! - distributed process participants.
//!
//! A process is therefore represented by data and iterators rather than
//! fixed-size structures.
//!
//! The implementation does NOT claim that an arbitrary process can be
//! materialized in finite memory. Large characterization workloads must be
//! streamable.
//!
//! ```text
//! tiny process
//!     |
//!     +--> in-memory characterization
//!
//! large process
//!     |
//!     +--> streaming observations
//!     |
//!     +--> bounded accumulation
//!     |
//!     +--> distributed reduction
//! ```
//!
//! Resource limits are policy. They are not semantic limits.
//!
//! ## Determinism
//!
//! This file:
//!
//! - never creates random numbers;
//! - never uses a global RNG;
//! - never reads the system clock;
//! - never generates identifiers;
//! - never depends on unordered iteration;
//! - never relies on thread scheduling for semantic results.
//!
//! Parallel implementations must preserve deterministic merge order when
//! deterministic execution is requested.
//!
//! ## Numerical safety
//!
//! NaN and infinities are rejected.
//!
//! Negative counts are impossible because counts are unsigned.
//!
//! Floating-point values are never silently clamped, normalized, or replaced.
//!
//! Approximation is represented explicitly.
//!
//! ## Resource safety
//!
//! Potentially unbounded process-characterization data must be processed
//! incrementally.
//!
//! The accumulator below stores only process-level sufficient statistics,
//! rather than requiring all observations to remain in memory.
//!
//! Explicit limits are caller supplied.
//!
//! ## Serialization
//!
//! This file owns semantic structures only.
//!
//! Versioned serialization belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! Rust memory layout is therefore never the serialization contract.
//!
//! ## Integration
//!
//! ### Protocol
//!
//! `characterization::protocol` defines the experiment contract.
//!
//! This module references its stable protocol identity but does not execute
//! the protocol.
//!
//! ### Observation
//!
//! `characterization::observation` owns raw evidence.
//!
//! An integration adapter converts raw observations into
//! [`ProcessObservation`] records.
//!
//! This avoids coupling process characterization to one physical observation
//! representation.
//!
//! ### Estimator
//!
//! `characterization::estimator` owns generic statistical inference.
//!
//! This file records process-level sufficient statistics and result contracts.
//! Statistical estimators may populate [`ProcessParameterEstimate`].
//!
//! ### Tomography
//!
//! `characterization::tomography` owns reconstruction algorithms.
//!
//! A tomography result can be attached to
//! [`ProcessCharacterizationResult`] through a caller-defined representation
//! reference without making tomography the only characterization mechanism.
//!
//! ### Calibration
//!
//! A process-characterization result may be consumed by calibration to update
//! a calibration snapshot.
//!
//! ### Noise
//!
//! A validated characterization result may be converted into a ZQN
//! [`NoiseModel`] by an integration layer.
//!
//! ### Hardware
//!
//! Hardware adapters produce observations and capability information. They do
//! not become dependencies of this file.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler enforced.

#![forbid(unsafe_code)]

use core::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::characterization::protocol::{ProtocolId, ProtocolVersion};
use crate::quantum::zqn::core::ids::{
    CalibrationId, CharacterizationId, ExperimentId, ObservationId,
};

/// Result type for process characterization.
pub type ProcessCharacterizationResult<T> =
    Result<T, ProcessCharacterizationError>;

/// Errors produced by process characterization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProcessCharacterizationError {
    /// Required identifier is missing.
    MissingIdentifier {
        field: &'static str,
    },

    /// An identifier was malformed.
    InvalidIdentifier {
        field: &'static str,
    },

    /// A process scope is invalid.
    InvalidScope,

    /// A process setting is invalid.
    InvalidSetting,

    /// A process setting has an invalid dimension.
    InvalidDimension,

    /// A numerical value is NaN or infinite.
    NonFiniteValue {
        field: &'static str,
    },

    /// A numerical value violates a declared range.
    InvalidNumericValue {
        field: &'static str,
    },

    /// An observation has an invalid count.
    InvalidCount,

    /// Integer arithmetic overflow occurred.
    ArithmeticOverflow {
        operation: &'static str,
    },

    /// The process cannot be identified from the supplied observations.
    NonIdentifiable,

    /// More observations are required.
    InsufficientObservations,

    /// The supplied observations describe incompatible process settings.
    IncompatibleObservation,

    /// The supplied characterization configuration is inconsistent.
    InvalidConfiguration {
        reason: &'static str,
    },

    /// A caller-selected resource limit was exceeded.
    ResourceLimitExceeded {
        resource: &'static str,
    },

    /// The requested characterization method is unsupported.
    UnsupportedMethod {
        method: &'static str,
    },

    /// A result requires an approximation that was not explicitly allowed.
    ApproximationRequired,

    /// A requested exact result cannot be produced by the selected method.
    ExactResultUnavailable,

    /// A process result failed validation.
    ValidationFailure {
        reason: &'static str,
    },

    /// Two accumulators cannot be merged.
    IncompatibleAccumulator,

    /// The process result contains inconsistent metadata.
    InconsistentMetadata,
}

impl fmt::Display for ProcessCharacterizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingIdentifier { field } => {
                write!(formatter, "missing process-characterization identifier: {field}")
            }
            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid process-characterization identifier: {field}")
            }
            Self::InvalidScope => {
                formatter.write_str("invalid process-characterization scope")
            }
            Self::InvalidSetting => {
                formatter.write_str("invalid process-characterization setting")
            }
            Self::InvalidDimension => {
                formatter.write_str("invalid process-characterization dimension")
            }
            Self::NonFiniteValue { field } => {
                write!(formatter, "{field} must be finite")
            }
            Self::InvalidNumericValue { field } => {
                write!(formatter, "invalid numerical value for {field}")
            }
            Self::InvalidCount => {
                formatter.write_str("invalid process-characterization count")
            }
            Self::ArithmeticOverflow { operation } => {
                write!(formatter, "arithmetic overflow during {operation}")
            }
            Self::NonIdentifiable => {
                formatter.write_str("the characterized process is not identifiable")
            }
            Self::InsufficientObservations => {
                formatter.write_str("insufficient observations for process characterization")
            }
            Self::IncompatibleObservation => {
                formatter.write_str("observation is incompatible with the process contract")
            }
            Self::InvalidConfiguration { reason } => {
                write!(formatter, "invalid process-characterization configuration: {reason}")
            }
            Self::ResourceLimitExceeded { resource } => {
                write!(formatter, "process-characterization resource limit exceeded: {resource}")
            }
            Self::UnsupportedMethod { method } => {
                write!(formatter, "unsupported process-characterization method: {method}")
            }
            Self::ApproximationRequired => {
                formatter.write_str("the requested characterization requires explicit approximation")
            }
            Self::ExactResultUnavailable => {
                formatter.write_str("the selected characterization method cannot provide an exact result")
            }
            Self::ValidationFailure { reason } => {
                write!(formatter, "process-characterization validation failed: {reason}")
            }
            Self::IncompatibleAccumulator => {
                formatter.write_str("incompatible process-characterization accumulators")
            }
            Self::InconsistentMetadata => {
                formatter.write_str("inconsistent process-characterization metadata")
            }
        }
    }
}

impl std::error::Error for ProcessCharacterizationError {}


// ============================================================================
// Stable schema identity
// ============================================================================

/// Semantic schema identifier.
pub const PROCESS_CHARACTERIZATION_SCHEMA_ID: &str =
    "zamani.quantum.zqn.characterization.process";

/// Semantic schema version.
///
/// This is independent of the global ZQN version and serialization version.
pub const PROCESS_CHARACTERIZATION_SCHEMA_VERSION: u32 = 1;


// ============================================================================
// Process modality
// ============================================================================

/// Broad class of process being characterized.
///
/// This enum intentionally does not enumerate hardware vendors or fixed
/// gate sets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProcessKind {
    /// A unitary or approximately unitary process.
    Unitary,

    /// A general quantum channel/process.
    QuantumChannel,

    /// A measurement process.
    Measurement,

    /// A state-preparation process.
    StatePreparation,

    /// A reset process.
    Reset,

    /// A transport process.
    Transport,

    /// A time-evolution process.
    Evolution,

    /// A composite process containing multiple operations/resources.
    Composite,

    /// A target-defined process not covered by the built-in categories.
    Custom,
}


// ============================================================================
// Characterization method
// ============================================================================

/// Process-characterization methodology.
///
/// The method is descriptive. The implementation belongs to the appropriate
/// characterization module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProcessCharacterizationMethod {
    /// Direct statistical characterization.
    Direct,

    /// Process tomography.
    Tomography,

    /// Randomized characterization.
    Randomized,

    /// Cycle-based characterization.
    Cycle,

    /// Spectral/frequency-domain characterization.
    Spectral,

    /// Time-domain characterization.
    TimeDomain,

    /// Gate-set/process-family characterization.
    GateSet,

    /// User-defined method.
    Custom(String),
}

impl ProcessCharacterizationMethod {
    fn validate(&self) -> ProcessCharacterizationResult<()> {
        if let Self::Custom(value) = self {
            if value.trim().is_empty() {
                return Err(ProcessCharacterizationError::InvalidConfiguration {
                    reason: "custom characterization method must not be empty",
                });
            }
        }

        Ok(())
    }
}


// ============================================================================
// Approximation contract
// ============================================================================

/// Explicit scientific result contract.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ApproximationPolicy {
    /// Only mathematically exact results are accepted.
    ExactOnly,

    /// Approximation is allowed up to the specified absolute error.
    AbsoluteTolerance(f64),

    /// Approximation is allowed up to a relative error.
    RelativeTolerance(f64),

    /// A bounded error guarantee is required.
    BoundedError(f64),

    /// A statistical result is acceptable at the specified confidence.
    StatisticalConfidence(f64),
}

impl ApproximationPolicy {
    /// Validates the approximation policy.
    pub fn validate(self) -> ProcessCharacterizationResult<()> {
        match self {
            Self::ExactOnly => Ok(()),

            Self::AbsoluteTolerance(value)
            | Self::RelativeTolerance(value)
            | Self::BoundedError(value) => {
                if !value.is_finite() || value <= 0.0 {
                    return Err(ProcessCharacterizationError::InvalidNumericValue {
                        field: "approximation tolerance",
                    });
                }

                Ok(())
            }

            Self::StatisticalConfidence(value) => {
                if !value.is_finite() || !(0.0 < value && value < 1.0) {
                    return Err(ProcessCharacterizationError::InvalidNumericValue {
                        field: "statistical confidence",
                    });
                }

                Ok(())
            }
        }
    }

    /// Whether statistical estimation is permitted.
    pub const fn allows_statistical_estimation(self) -> bool {
        !matches!(self, Self::ExactOnly)
    }
}


// ============================================================================
// Process resource
// ============================================================================

/// Quantum resource participating in the characterized process.
///
/// This is deliberately extensible and does not assume all quantum
/// technologies are qubit-based.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessResource {
    /// Logical qubit from canonical quantum IR.
    LogicalQubit(QubitId),

    /// Physical qubit from canonical quantum IR.
    PhysicalQubit(PhysicalQubitId),

    /// An arbitrary non-qubit resource.
    Named(String),

    /// An opaque resource identity supplied by an integration layer.
    Opaque(String),
}

impl ProcessResource {
    /// Validates the resource.
    pub fn validate(&self) -> ProcessCharacterizationResult<()> {
        match self {
            Self::LogicalQubit(_) | Self::PhysicalQubit(_) => Ok(()),

            Self::Named(value) | Self::Opaque(value) => {
                if value.trim().is_empty() {
                    return Err(ProcessCharacterizationError::InvalidScope);
                }

                Ok(())
            }
        }
    }
}


// ============================================================================
// Process scope
// ============================================================================

/// Resources entering or leaving a characterized process.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProcessScope {
    /// Input resources.
    pub inputs: Vec<ProcessResource>,

    /// Output resources.
    pub outputs: Vec<ProcessResource>,

    /// Whether the scope is target-defined rather than explicitly enumerated.
    pub target_defined: bool,

    /// Whether the process may span distributed execution domains.
    pub distributed: bool,
}

impl ProcessScope {
    /// Creates an explicitly enumerated scope.
    pub fn explicit(
        inputs: Vec<ProcessResource>,
        outputs: Vec<ProcessResource>,
    ) -> ProcessCharacterizationResult<Self> {
        let scope = Self {
            inputs,
            outputs,
            target_defined: false,
            distributed: false,
        };

        scope.validate()?;

        Ok(scope)
    }

    /// Creates a target-defined scope.
    pub fn target_defined() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
            target_defined: true,
            distributed: false,
        }
    }

    /// Marks the scope as distributed.
    pub fn distributed(mut self) -> Self {
        self.distributed = true;
        self
    }

    /// Validates the scope.
    pub fn validate(&self) -> ProcessCharacterizationResult<()> {
        if !self.target_defined && self.inputs.is_empty() && self.outputs.is_empty() {
            return Err(ProcessCharacterizationError::InvalidScope);
        }

        for resource in self.inputs.iter().chain(self.outputs.iter()) {
            resource.validate()?;
        }

        Ok(())
    }

    /// Returns the explicitly enumerated input arity.
    ///
    /// `None` means that the target defines the arity dynamically.
    pub fn input_arity(&self) -> Option<usize> {
        if self.target_defined {
            None
        } else {
            Some(self.inputs.len())
        }
    }

    /// Returns the explicitly enumerated output arity.
    pub fn output_arity(&self) -> Option<usize> {
        if self.target_defined {
            None
        } else {
            Some(self.outputs.len())
        }
    }
}


// ============================================================================
// Process setting
// ============================================================================

/// Identifies one experimental input configuration.
///
/// A setting is intentionally represented as an opaque ordered vector rather
/// than a fixed number of parameters.
#[derive(Clone, Debug, PartialEq)]
pub struct ProcessSetting {
    /// Stable setting index within an experiment.
    pub index: u64,

    /// Ordered real-valued control parameters.
    ///
    /// The interpretation belongs to the protocol.
    pub parameters: Vec<f64>,

    /// Optional external semantic identifier.
    pub identifier: Option<String>,
}

impl ProcessSetting {
    /// Creates a process setting.
    pub fn new(
        index: u64,
        parameters: Vec<f64>,
    ) -> ProcessCharacterizationResult<Self> {
        for value in &parameters {
            if !value.is_finite() {
                return Err(ProcessCharacterizationError::NonFiniteValue {
                    field: "process setting parameter",
                });
            }
        }

        Ok(Self {
            index,
            parameters,
            identifier: None,
        })
    }

    /// Adds an explicit identifier.
    pub fn with_identifier(
        mut self,
        identifier: impl Into<String>,
    ) -> ProcessCharacterizationResult<Self> {
        let identifier = identifier.into();

        if identifier.trim().is_empty() {
            return Err(ProcessCharacterizationError::InvalidIdentifier {
                field: "process setting identifier",
            });
        }

        self.identifier = Some(identifier);

        Ok(self)
    }
}


// ============================================================================
// Process observation
// ============================================================================

/// One process-level observation supplied by an execution/observation adapter.
///
/// This is intentionally smaller than the raw observation model.
///
/// `observation_id` points back to the canonical raw evidence.
#[derive(Clone, Debug, PartialEq)]
pub struct ProcessObservation {
    /// Raw observation identity.
    pub observation_id: ObservationId,

    /// Experiment identity.
    pub experiment_id: ExperimentId,

    /// Input setting used for this observation.
    pub setting_index: u64,

    /// Number of shots represented.
    pub shots: u64,

    /// Scalar response associated with the process.
///
/// The meaning is protocol-defined. Examples:
/// - probability;
/// - expectation value;
/// - survival fraction;
/// - transition frequency;
/// - correlation;
/// - measured process feature.
    pub response: f64,

    /// Optional variance supplied by the estimator/execution layer.
    pub variance: Option<f64>,

    /// Optional weight.
///
/// If omitted, the characterization implementation chooses its documented
/// weighting rule.
    pub weight: Option<f64>,
}

impl ProcessObservation {
    /// Validates the observation.
    pub fn validate(&self) -> ProcessCharacterizationResult<()> {
        if self.shots == 0 {
            return Err(ProcessCharacterizationError::InvalidCount);
        }

        if !self.response.is_finite() {
            return Err(ProcessCharacterizationError::NonFiniteValue {
                field: "process response",
            });
        }

        if let Some(variance) = self.variance {
            if !variance.is_finite() || variance < 0.0 {
                return Err(ProcessCharacterizationError::InvalidNumericValue {
                    field: "process variance",
                });
            }
        }

        if let Some(weight) = self.weight {
            if !weight.is_finite() || weight <= 0.0 {
                return Err(ProcessCharacterizationError::InvalidNumericValue {
                    field: "process observation weight",
                });
            }
        }

        Ok(())
    }
}


// ============================================================================
// Process parameter estimate
// ============================================================================

/// One inferred process parameter.
///
/// The actual statistical estimator lives in `estimator.rs`.
#[derive(Clone, Debug, PartialEq)]
pub struct ProcessParameterEstimate {
    /// Stable parameter name.
    pub name: String,

    /// Point estimate.
    pub value: f64,

    /// Optional standard error.
    pub standard_error: Option<f64>,

    /// Optional lower confidence/boundary value.
    pub lower_bound: Option<f64>,

    /// Optional upper confidence/boundary value.
    pub upper_bound: Option<f64>,

    /// Number of shots contributing to the estimate.
    pub shots: u64,

    /// Whether this is statistically inferred.
    pub statistical: bool,
}

impl ProcessParameterEstimate {
    /// Creates a parameter estimate.
    pub fn new(
        name: impl Into<String>,
        value: f64,
        shots: u64,
    ) -> ProcessCharacterizationResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ProcessCharacterizationError::InvalidIdentifier {
                field: "process parameter name",
            });
        }

        if !value.is_finite() {
            return Err(ProcessCharacterizationError::NonFiniteValue {
                field: "process parameter value",
            });
        }

        Ok(Self {
            name,
            value,
            standard_error: None,
            lower_bound: None,
            upper_bound: None,
            shots,
            statistical: true,
        })
    }

    /// Adds a standard error.
    pub fn with_standard_error(
        mut self,
        value: f64,
    ) -> ProcessCharacterizationResult<Self> {
        if !value.is_finite() || value < 0.0 {
            return Err(ProcessCharacterizationError::InvalidNumericValue {
                field: "standard error",
            });
        }

        self.standard_error = Some(value);
        Ok(self)
    }

    /// Adds an interval.
    pub fn with_bounds(
        mut self,
        lower: f64,
        upper: f64,
    ) -> ProcessCharacterizationResult<Self> {
        if !lower.is_finite() || !upper.is_finite() || lower > upper {
            return Err(ProcessCharacterizationError::InvalidNumericValue {
                field: "parameter bounds",
            });
        }

        if self.value < lower || self.value > upper {
            return Err(ProcessCharacterizationError::InvalidNumericValue {
                field: "parameter value outside bounds",
            });
        }

        self.lower_bound = Some(lower);
        self.upper_bound = Some(upper);

        Ok(self)
    }
}


// ============================================================================
// Quality diagnostics
// ============================================================================

/// Quality/identifiability status of a process characterization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessQuality {
    /// Enough information exists and the process passed the configured checks.
    Valid,

    /// The process is statistically characterized but uncertainty remains.
    Uncertain,

    /// The supplied data do not uniquely identify the requested process.
    NonIdentifiable,

    /// More observations are required.
    InsufficientData,

    /// The result relies on an explicitly permitted approximation.
    Approximate,

    /// The result is statistically inferred.
    Statistical,

    /// Validation could not establish physical validity.
    ValidationIncomplete,
}


/// Numerical diagnostics attached to a process result.
#[derive(Clone, Debug, PartialEq)]
pub struct ProcessDiagnostics {
    /// Number of process observations consumed.
    pub observations: u64,

    /// Total number of shots represented.
    pub shots: u64,

    /// Sum of observation weights.
    pub total_weight: f64,

    /// Weighted mean response.
    pub weighted_mean_response: Option<f64>,

    /// Weighted second central moment when available.
    pub weighted_variance: Option<f64>,

    /// Whether all observations were accepted.
    pub complete: bool,

    /// Whether the process was identifiable under the requested contract.
    pub identifiable: bool,

    /// Human-readable diagnostic code.
    pub code: Option<String>,
}

impl Default for ProcessDiagnostics {
    fn default() -> Self {
        Self {
            observations: 0,
            shots: 0,
            total_weight: 0.0,
            weighted_mean_response: None,
            weighted_variance: None,
            complete: true,
            identifiable: false,
            code: None,
        }
    }
}


// ============================================================================
// Result representation reference
// ============================================================================

/// Identifies the representation produced by a downstream reconstruction
/// subsystem.
///
/// This avoids making process characterization depend on a particular
/// channel/tomography representation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProcessRepresentation {
    /// No reconstructed process representation was produced.
    None,

    /// A channel representation exists elsewhere.
    ChannelReference(String),

    /// A tomography result exists elsewhere.
    TomographyReference(String),

    /// A process matrix is stored elsewhere.
    ProcessMatrixReference(String),

    /// A target-specific representation is stored elsewhere.
    TargetReference(String),
}

impl Default for ProcessRepresentation {
    fn default() -> Self {
        Self::None
    }
}


// ============================================================================
// Provenance
// ============================================================================

/// Process-characterization provenance.
///
/// The values are references, not embedded copies of external objects.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProcessCharacterizationProvenance {
    /// Protocol identity.
    pub protocol_id: Option<ProtocolId>,

    /// Protocol semantic version.
    pub protocol_version: Option<ProtocolVersion>,

    /// Characterization identity.
    pub characterization_id: Option<CharacterizationId>,

    /// Calibration used by the experiment.
    pub calibration_id: Option<CalibrationId>,

    /// Number of source experiments.
    pub experiments: u64,

    /// Optional externally supplied execution identity.
    pub execution_identity: Option<String>,

    /// Optional model identity.
    pub model_identity: Option<String>,

    /// Optional canonical configuration identity.
    pub configuration_identity: Option<String>,
}


// ============================================================================
// Resource limits
// ============================================================================

/// Explicit process-characterization resource policy.
///
/// None means no limit imposed by this layer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProcessCharacterizationLimits {
    /// Maximum number of observations accepted by one accumulator.
    pub max_observations: Option<u64>,

    /// Maximum number of shots represented.
    pub max_shots: Option<u64>,

    /// Maximum number of process parameters.
    pub max_parameters: Option<u64>,

    /// Maximum number of input settings.
    pub max_settings: Option<u64>,

    /// Maximum bytes permitted for materialized process metadata.
    pub max_metadata_bytes: Option<u64>,
}

impl ProcessCharacterizationLimits {
    fn check_observations(&self, value: u64) -> ProcessCharacterizationResult<()> {
        if let Some(limit) = self.max_observations {
            if value > limit {
                return Err(ProcessCharacterizationError::ResourceLimitExceeded {
                    resource: "observations",
                });
            }
        }

        Ok(())
    }

    fn check_shots(&self, value: u64) -> ProcessCharacterizationResult<()> {
        if let Some(limit) = self.max_shots {
            if value > limit {
                return Err(ProcessCharacterizationError::ResourceLimitExceeded {
                    resource: "shots",
                });
            }
        }

        Ok(())
    }
}


// ============================================================================
// Process characterization specification
// ============================================================================

/// Immutable configuration for one process-characterization task.
#[derive(Clone, Debug, PartialEq)]
pub struct ProcessCharacterizationSpec {
    /// Process kind.
    pub process_kind: ProcessKind,

    /// Characterization methodology.
    pub method: ProcessCharacterizationMethod,

    /// Process scope.
    pub scope: ProcessScope,

    /// Required approximation semantics.
    pub approximation: ApproximationPolicy,

    /// Resource limits.
    pub limits: ProcessCharacterizationLimits,

    /// Optional expected number of input settings.
    ///
    /// This is metadata/validation, not an architectural maximum.
    pub expected_settings: Option<u64>,

    /// Optional expected process parameter count.
    pub expected_parameters: Option<u64>,
}

impl ProcessCharacterizationSpec {
    /// Validates the entire characterization specification.
    pub fn validate(&self) -> ProcessCharacterizationResult<()> {
        self.method.validate()?;
        self.approximation.validate()?;
        self.scope.validate()?;

        if let Some(settings) = self.expected_settings {
            if settings == 0 {
                return Err(ProcessCharacterizationError::InvalidConfiguration {
                    reason: "expected settings must be non-zero",
                });
            }

            if let Some(limit) = self.limits.max_settings {
                if settings > limit {
                    return Err(ProcessCharacterizationError::ResourceLimitExceeded {
                        resource: "settings",
                    });
                }
            }
        }

        if let Some(parameters) = self.expected_parameters {
            if parameters == 0 {
                return Err(ProcessCharacterizationError::InvalidConfiguration {
                    reason: "expected parameter count must be non-zero",
                });
            }

            if let Some(limit) = self.limits.max_parameters {
                if parameters > limit {
                    return Err(ProcessCharacterizationError::ResourceLimitExceeded {
                        resource: "parameters",
                    });
                }
            }
        }

        Ok(())
    }
}


// ============================================================================
// Streaming accumulator
// ============================================================================

/// Streaming sufficient-statistics accumulator for process characterization.
///
/// The accumulator does not retain all observations.
///
/// It maintains:
///
/// ```text
/// count
/// total shots
/// sum(weight)
/// sum(weight * response)
/// sum(weight * response²)
/// ```
///
/// This makes ordinary scalar process characterization O(1) memory with
/// respect to the number of observations.
///
/// It does NOT claim that all characterization algorithms are O(1) memory.
/// Tomography and other high-dimensional reconstructions have their own
/// representation requirements.
#[derive(Clone, Debug, PartialEq)]
pub struct ProcessAccumulator {
    observations: u64,
    shots: u64,

    weight_sum: f64,
    weighted_response_sum: f64,
    weighted_response_squared_sum: f64,

    limits: ProcessCharacterizationLimits,
}

impl ProcessAccumulator {
    /// Creates an empty accumulator.
    pub fn new(
        limits: ProcessCharacterizationLimits,
    ) -> Self {
        Self {
            observations: 0,
            shots: 0,
            weight_sum: 0.0,
            weighted_response_sum: 0.0,
            weighted_response_squared_sum: 0.0,
            limits,
        }
    }

    /// Adds one observation.
    pub fn push(
        &mut self,
        observation: &ProcessObservation,
    ) -> ProcessCharacterizationResult<()> {
        observation.validate()?;

        let next_observations = self
            .observations
            .checked_add(1)
            .ok_or(ProcessCharacterizationError::ArithmeticOverflow {
                operation: "observation count",
            })?;

        let next_shots = self
            .shots
            .checked_add(observation.shots)
            .ok_or(ProcessCharacterizationError::ArithmeticOverflow {
                operation: "shot count",
            })?;

        self.limits.check_observations(next_observations)?;
        self.limits.check_shots(next_shots)?;

        let weight = observation.weight.unwrap_or(1.0);

        let weighted_response = weight * observation.response;

        if !weighted_response.is_finite() {
            return Err(ProcessCharacterizationError::NonFiniteValue {
                field: "weighted process response",
            });
        }

        let squared = observation.response * observation.response;

        if !squared.is_finite() {
            return Err(ProcessCharacterizationError::NonFiniteValue {
                field: "squared process response",
            });
        }

        let weighted_squared = weight * squared;

        if !weighted_squared.is_finite() {
            return Err(ProcessCharacterizationError::NonFiniteValue {
                field: "weighted squared process response",
            });
        }

        let next_weight_sum = self.weight_sum + weight;
        let next_response_sum = self.weighted_response_sum + weighted_response;
        let next_squared_sum =
            self.weighted_response_squared_sum + weighted_squared;

        if !next_weight_sum.is_finite()
            || !next_response_sum.is_finite()
            || !next_squared_sum.is_finite()
        {
            return Err(ProcessCharacterizationError::NonFiniteValue {
                field: "process accumulator",
            });
        }

        self.observations = next_observations;
        self.shots = next_shots;
        self.weight_sum = next_weight_sum;
        self.weighted_response_sum = next_response_sum;
        self.weighted_response_squared_sum = next_squared_sum;

        Ok(())
    }

    /// Deterministically merges another accumulator.
    ///
    /// The caller controls merge order. This method performs no parallel
    /// scheduling itself.
    pub fn merge(
        &mut self,
        other: &Self,
    ) -> ProcessCharacterizationResult<()> {
        let observations = self
            .observations
            .checked_add(other.observations)
            .ok_or(ProcessCharacterizationError::ArithmeticOverflow {
                operation: "merged observation count",
            })?;

        let shots = self
            .shots
            .checked_add(other.shots)
            .ok_or(ProcessCharacterizationError::ArithmeticOverflow {
                operation: "merged shot count",
            })?;

        self.limits.check_observations(observations)?;
        self.limits.check_shots(shots)?;

        let weight_sum = self.weight_sum + other.weight_sum;
        let response_sum =
            self.weighted_response_sum + other.weighted_response_sum;
        let squared_sum =
            self.weighted_response_squared_sum
                + other.weighted_response_squared_sum;

        if !weight_sum.is_finite()
            || !response_sum.is_finite()
            || !squared_sum.is_finite()
        {
            return Err(ProcessCharacterizationError::NonFiniteValue {
                field: "merged process accumulator",
            });
        }

        self.observations = observations;
        self.shots = shots;
        self.weight_sum = weight_sum;
        self.weighted_response_sum = response_sum;
        self.weighted_response_squared_sum = squared_sum;

        Ok(())
    }

    /// Number of observations.
    pub const fn observations(&self) -> u64 {
        self.observations
    }

    /// Number of shots.
    pub const fn shots(&self) -> u64 {
        self.shots
    }

    /// Total weight.
    pub const fn weight_sum(&self) -> f64 {
        self.weight_sum
    }

    /// Returns the weighted mean.
    pub fn mean(&self) -> ProcessCharacterizationResult<f64> {
        if self.observations == 0 || self.weight_sum <= 0.0 {
            return Err(ProcessCharacterizationError::InsufficientObservations);
        }

        let value = self.weighted_response_sum / self.weight_sum;

        if !value.is_finite() {
            return Err(ProcessCharacterizationError::NonFiniteValue {
                field: "weighted process mean",
            });
        }

        Ok(value)
    }

    /// Returns the population-style weighted variance.
    pub fn variance(&self) -> ProcessCharacterizationResult<f64> {
        if self.observations == 0 || self.weight_sum <= 0.0 {
            return Err(ProcessCharacterizationError::InsufficientObservations);
        }

        let mean = self.mean()?;

        let raw_second_moment =
            self.weighted_response_squared_sum / self.weight_sum;

        let variance = raw_second_moment - mean * mean;

        if !variance.is_finite() {
            return Err(ProcessCharacterizationError::NonFiniteValue {
                field: "weighted process variance",
            });
        }

        // A very small negative value can arise solely from floating-point
        // cancellation. Do not silently clamp a materially negative value.
        //
        // The scale is derived from the computed second moment rather than a
        // fixed machine-specific constant.
        let scale = raw_second_moment.abs().max(mean.abs() * mean.abs());

        let numerical_tolerance =
            64.0 * f64::EPSILON * scale.max(1.0);

        if variance < 0.0 {
            if variance >= -numerical_tolerance {
                return Ok(0.0);
            }

            return Err(ProcessCharacterizationError::ValidationFailure {
                reason: "computed weighted variance is materially negative",
            });
        }

        Ok(variance)
    }

    /// Converts the accumulated statistics into diagnostics.
    pub fn diagnostics(&self) -> ProcessCharacterizationResult<ProcessDiagnostics> {
        if self.observations == 0 {
            return Ok(ProcessDiagnostics {
                complete: false,
                ..ProcessDiagnostics::default()
            });
        }

        let mean = self.mean()?;
        let variance = self.variance()?;

        Ok(ProcessDiagnostics {
            observations: self.observations,
            shots: self.shots,
            total_weight: self.weight_sum,
            weighted_mean_response: Some(mean),
            weighted_variance: Some(variance),
            complete: true,
            identifiable: false,
            code: None,
        })
    }
}


// ============================================================================
// Result
// ============================================================================

/// Immutable result of process characterization.
#[derive(Clone, Debug, PartialEq)]
pub struct ProcessCharacterizationResultData {
    /// Characterization identity.
    pub characterization_id: CharacterizationId,

    /// Process kind.
    pub process_kind: ProcessKind,

    /// Method used.
    pub method: ProcessCharacterizationMethod,

    /// Characterized scope.
    pub scope: ProcessScope,

    /// Scientific approximation contract.
    pub approximation: ApproximationPolicy,

    /// Overall quality.
    pub quality: ProcessQuality,

    /// Statistical diagnostics.
    pub diagnostics: ProcessDiagnostics,

    /// Inferred process parameters.
    pub parameters: Vec<ProcessParameterEstimate>,

    /// Optional reconstructed representation reference.
    pub representation: ProcessRepresentation,

    /// Provenance.
    pub provenance: ProcessCharacterizationProvenance,
}

impl ProcessCharacterizationResultData {
    /// Validates the complete result.
    pub fn validate(&self) -> ProcessCharacterizationResult<()> {
        self.scope.validate()?;
        self.approximation.validate()?;

        for parameter in &self.parameters {
            if parameter.name.trim().is_empty() {
                return Err(ProcessCharacterizationError::InvalidIdentifier {
                    field: "process parameter name",
                });
            }

            if !parameter.value.is_finite() {
                return Err(ProcessCharacterizationError::NonFiniteValue {
                    field: "process parameter value",
                });
            }

            if let Some(error) = parameter.standard_error {
                if !error.is_finite() || error < 0.0 {
                    return Err(ProcessCharacterizationError::InvalidNumericValue {
                        field: "process parameter standard error",
                    });
                }
            }

            match (parameter.lower_bound, parameter.upper_bound) {
                (Some(lower), Some(upper)) => {
                    if !lower.is_finite()
                        || !upper.is_finite()
                        || lower > upper
                        || parameter.value < lower
                        || parameter.value > upper
                    {
                        return Err(ProcessCharacterizationError::InvalidNumericValue {
                            field: "process parameter bounds",
                        });
                    }
                }

                (None, None) => {}

                _ => {
                    return Err(ProcessCharacterizationError::InvalidNumericValue {
                        field: "incomplete process parameter bounds",
                    });
                }
            }
        }

        Ok(())
    }
}


// ============================================================================
// Builder
// ============================================================================

/// Streaming builder for a process-characterization result.
///
/// The builder separates accumulation from final result construction.
#[derive(Clone, Debug)]
pub struct ProcessCharacterizationBuilder {
    specification: ProcessCharacterizationSpec,
    accumulator: ProcessAccumulator,
    parameters: Vec<ProcessParameterEstimate>,
    representation: ProcessRepresentation,
    provenance: ProcessCharacterizationProvenance,
    characterization_id: Option<CharacterizationId>,
}

impl ProcessCharacterizationBuilder {
    /// Creates a builder.
    pub fn new(
        specification: ProcessCharacterizationSpec,
    ) -> ProcessCharacterizationResult<Self> {
        specification.validate()?;

        Ok(Self {
            accumulator: ProcessAccumulator::new(
                specification.limits.clone(),
            ),
            specification,
            parameters: Vec::new(),
            representation: ProcessRepresentation::None,
            provenance: ProcessCharacterizationProvenance::default(),
            characterization_id: None,
        })
    }

    /// Assigns the characterization identity.
    pub fn with_characterization_id(
        mut self,
        id: CharacterizationId,
    ) -> Self {
        self.characterization_id = Some(id);
        self
    }

    /// Assigns provenance.
    pub fn with_provenance(
        mut self,
        provenance: ProcessCharacterizationProvenance,
    ) -> Self {
        self.provenance = provenance;
        self
    }

    /// Adds one observation.
    pub fn push_observation(
        &mut self,
        observation: &ProcessObservation,
    ) -> ProcessCharacterizationResult<()> {
        self.accumulator.push(observation)
    }

    /// Adds a parameter estimate.
    pub fn add_parameter(
        &mut self,
        parameter: ProcessParameterEstimate,
    ) -> ProcessCharacterizationResult<()> {
        let current = u64::try_from(self.parameters.len()).map_err(|_| {
            ProcessCharacterizationError::ArithmeticOverflow {
                operation: "parameter count conversion",
            }
        })?;

        let next = current.checked_add(1).ok_or(
            ProcessCharacterizationError::ArithmeticOverflow {
                operation: "parameter count",
            },
        )?;

        if let Some(limit) = self.specification.limits.max_parameters {
            if next > limit {
                return Err(ProcessCharacterizationError::ResourceLimitExceeded {
                    resource: "parameters",
                });
            }
        }

        self.parameters.push(parameter);

        Ok(())
    }

    /// Attaches a reconstructed representation reference.
    pub fn with_representation(
        mut self,
        representation: ProcessRepresentation,
    ) -> Self {
        self.representation = representation;
        self
    }

    /// Returns the current accumulator.
    pub fn accumulator(&self) -> &ProcessAccumulator {
        &self.accumulator
    }

    /// Consumes the builder and produces a validated result.
    pub fn finish(
        mut self,
    ) -> ProcessCharacterizationResult<ProcessCharacterizationResultData> {
        let characterization_id =
            self.characterization_id.ok_or(
                ProcessCharacterizationError::MissingIdentifier {
                    field: "characterization_id",
                },
            )?;

        let mut diagnostics = self.accumulator.diagnostics()?;

        let identifiable =
            self.accumulator.observations() > 0
                && (!self.parameters.is_empty()
                    || !matches!(
                        self.representation,
                        ProcessRepresentation::None
                    ));

        diagnostics.identifiable = identifiable;

        let quality = if self.accumulator.observations() == 0 {
            ProcessQuality::InsufficientData
        } else if !identifiable {
            ProcessQuality::NonIdentifiable
        } else {
            match self.specification.approximation {
                ApproximationPolicy::ExactOnly => ProcessQuality::Valid,

                ApproximationPolicy::AbsoluteTolerance(_)
                | ApproximationPolicy::RelativeTolerance(_)
                | ApproximationPolicy::BoundedError(_) => {
                    ProcessQuality::Approximate
                }

                ApproximationPolicy::StatisticalConfidence(_) => {
                    ProcessQuality::Statistical
                }
            }
        };

        let result = ProcessCharacterizationResultData {
            characterization_id,
            process_kind: self.specification.process_kind,
            method: self.specification.method,
            scope: self.specification.scope,
            approximation: self.specification.approximation,
            quality,
            diagnostics,
            parameters: self.parameters,
            representation: self.representation,
            provenance: self.provenance,
        };

        result.validate()?;

        Ok(result)
    }
}


// ============================================================================
// Streaming source contract
// ============================================================================

/// Source of process observations.
///
/// This trait is the integration boundary between raw observation storage and
/// process characterization.
///
/// Implementations can be:
///
/// - in-memory;
/// - file-backed;
/// - database-backed;
/// - distributed;
/// - hardware streaming;
/// - simulator streaming;
/// - online estimators.
pub trait ProcessObservationSource {
    /// Returns the next process observation.
    ///
    /// `Ok(None)` indicates end-of-stream.
    fn next_observation(
        &mut self,
    ) -> ProcessCharacterizationResult<Option<ProcessObservation>>;
}


/// Characterizes a stream without materializing the entire observation set.
pub fn characterize_stream<S>(
    specification: ProcessCharacterizationSpec,
    characterization_id: CharacterizationId,
    source: &mut S,
) -> ProcessCharacterizationResult<ProcessCharacterizationResultData>
where
    S: ProcessObservationSource,
{
    let mut builder =
        ProcessCharacterizationBuilder::new(specification)?
            .with_characterization_id(characterization_id);

    while let Some(observation) = source.next_observation()? {
        builder.push_observation(&observation)?;
    }

    builder.finish()
}


// ============================================================================
// Process-characterization contract
// ============================================================================

/// Generic contract implemented by concrete process-characterization
/// algorithms.
///
/// This trait intentionally does not prescribe how experiments are generated
/// or how observations are represented internally.
pub trait ProcessCharacterizer: Send + Sync {
    /// Validates the requested characterization.
    fn validate(
        &self,
        specification: &ProcessCharacterizationSpec,
    ) -> ProcessCharacterizationResult<()>;

    /// Consumes a stream and returns the characterization result.
    ///
    /// Implementations may use streaming, iterative, distributed, or
    /// materialized algorithms.
    fn characterize<S>(
        &self,
        specification: &ProcessCharacterizationSpec,
        characterization_id: CharacterizationId,
        source: &mut S,
    ) -> ProcessCharacterizationResult<ProcessCharacterizationResultData>
    where
        S: ProcessObservationSource;
}


// ============================================================================
// Simple streaming characterizer
// ============================================================================

/// Default process characterizer for scalar response characterization.
///
/// This is intentionally conservative.
///
/// It does NOT claim to reconstruct a quantum channel. It produces validated
/// scalar process statistics and can be used as the common base for richer
/// algorithms.
#[derive(Clone, Debug, Default)]
pub struct StreamingProcessCharacterizer;

impl StreamingProcessCharacterizer {
    /// Creates the default characterizer.
    pub const fn new() -> Self {
        Self
    }
}

impl ProcessCharacterizer for StreamingProcessCharacterizer {
    fn validate(
        &self,
        specification: &ProcessCharacterizationSpec,
    ) -> ProcessCharacterizationResult<()> {
        specification.validate()
    }

    fn characterize<S>(
        &self,
        specification: &ProcessCharacterizationSpec,
        characterization_id: CharacterizationId,
        source: &mut S,
    ) -> ProcessCharacterizationResult<ProcessCharacterizationResultData>
    where
        S: ProcessObservationSource,
    {
        self.validate(specification)?;

        characterize_stream(
            specification.clone(),
            characterization_id,
            source,
        )
    }
}


// ============================================================================
// Validation helpers
// ============================================================================

/// Validates a finite floating-point value.
pub fn validate_finite(
    value: f64,
    field: &'static str,
) -> ProcessCharacterizationResult<()> {
    if !value.is_finite() {
        return Err(ProcessCharacterizationError::NonFiniteValue {
            field,
        });
    }

    Ok(())
}


// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct VecSource {
        observations: Vec<ProcessObservation>,
        index: usize,
    }

    impl ProcessObservationSource for VecSource {
        fn next_observation(
            &mut self,
        ) -> ProcessCharacterizationResult<Option<ProcessObservation>> {
            if self.index >= self.observations.len() {
                return Ok(None);
            }

            let observation = self.observations[self.index].clone();
            self.index += 1;

            Ok(Some(observation))
        }
    }

    fn specification() -> ProcessCharacterizationSpec {
        ProcessCharacterizationSpec {
            process_kind: ProcessKind::QuantumChannel,
            method: ProcessCharacterizationMethod::Direct,
            scope: ProcessScope {
                inputs: vec![ProcessResource::Named("input".to_string())],
                outputs: vec![ProcessResource::Named("output".to_string())],
                target_defined: false,
                distributed: false,
            },
            approximation: ApproximationPolicy::StatisticalConfidence(0.95),
            limits: ProcessCharacterizationLimits::default(),
            expected_settings: None,
            expected_parameters: None,
        }
    }

    #[test]
    fn setting_rejects_non_finite_parameter() {
        let result = ProcessSetting::new(0, vec![f64::NAN]);

        assert!(matches!(
            result,
            Err(ProcessCharacterizationError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn scope_rejects_empty_explicit_scope() {
        let scope = ProcessScope {
            inputs: Vec::new(),
            outputs: Vec::new(),
            target_defined: false,
            distributed: false,
        };

        assert_eq!(
            scope.validate(),
            Err(ProcessCharacterizationError::InvalidScope)
        );
    }

    #[test]
    fn accumulator_is_streaming() {
        let mut accumulator =
            ProcessAccumulator::new(ProcessCharacterizationLimits::default());

        let observation = ProcessObservation {
            observation_id: unsafe_placeholder_observation_id(),
            experiment_id: unsafe_placeholder_experiment_id(),
            setting_index: 0,
            shots: 100,
            response: 0.5,
            variance: None,
            weight: None,
        };

        assert!(accumulator.push(&observation).is_ok());
        assert_eq!(accumulator.observations(), 1);
        assert_eq!(accumulator.shots(), 100);
        assert_eq!(accumulator.mean().unwrap(), 0.5);
    }

    #[test]
    fn weighted_mean_is_correct() {
        let mut accumulator =
            ProcessAccumulator::new(ProcessCharacterizationLimits::default());

        let first = ProcessObservation {
            observation_id: unsafe_placeholder_observation_id(),
            experiment_id: unsafe_placeholder_experiment_id(),
            setting_index: 0,
            shots: 10,
            response: 0.2,
            variance: None,
            weight: Some(1.0),
        };

        let second = ProcessObservation {
            observation_id: unsafe_placeholder_observation_id(),
            experiment_id: unsafe_placeholder_experiment_id(),
            setting_index: 1,
            shots: 10,
            response: 0.8,
            variance: None,
            weight: Some(3.0),
        };

        accumulator.push(&first).unwrap();
        accumulator.push(&second).unwrap();

        let mean = accumulator.mean().unwrap();

        assert!((mean - 0.65).abs() < 1.0e-12);
    }

    #[test]
    fn accumulator_rejects_non_finite_response() {
        let mut accumulator =
            ProcessAccumulator::new(ProcessCharacterizationLimits::default());

        let observation = ProcessObservation {
            observation_id: unsafe_placeholder_observation_id(),
            experiment_id: unsafe_placeholder_experiment_id(),
            setting_index: 0,
            shots: 1,
            response: f64::INFINITY,
            variance: None,
            weight: None,
        };

        assert!(matches!(
            accumulator.push(&observation),
            Err(ProcessCharacterizationError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn limits_are_policy_not_semantics() {
        let limits = ProcessCharacterizationLimits {
            max_observations: Some(1),
            ..ProcessCharacterizationLimits::default()
        };

        let mut accumulator = ProcessAccumulator::new(limits);

        let observation = ProcessObservation {
            observation_id: unsafe_placeholder_observation_id(),
            experiment_id: unsafe_placeholder_experiment_id(),
            setting_index: 0,
            shots: 1,
            response: 0.5,
            variance: None,
            weight: None,
        };

        accumulator.push(&observation).unwrap();

        assert!(matches!(
            accumulator.push(&observation),
            Err(ProcessCharacterizationError::ResourceLimitExceeded {
                resource: "observations"
            })
        ));
    }

    // These constructors are isolated to tests so production code never
    // fabricates identifiers.
    //
    // IMPORTANT:
    // The exact constructors of the repository's canonical ZQN identifiers
    // are owned by core::ids. These helpers should be replaced by the
    // canonical constructors already exposed there if the repository's
    // current API changes.
    fn unsafe_placeholder_observation_id() -> ObservationId {
        // This function intentionally does not use unsafe Rust.
        //
        // The repository's current ZqnIdValue-based ID implementation exposes
        // numeric construction. Keeping construction here avoids polluting
        // production APIs with synthetic identifiers.
        ObservationId::new(1)
    }

    fn unsafe_placeholder_experiment_id() -> ExperimentId {
        ExperimentId::new(1)
    }
}