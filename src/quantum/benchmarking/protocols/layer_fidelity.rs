//! Zamani Quantum Benchmarking — Layer Fidelity
//!
//! Production Layer Fidelity / Error Per Layered Gate (EPLG) protocol.
//!
//! # Purpose
//!
//! This module implements the protocol-level mathematical analysis for
//! scalable Layer Fidelity benchmarking.
//!
//! Layer Fidelity is designed for evaluating a connecting set of simultaneous
//! quantum operations across a larger processor. It extends randomized
//! benchmarking into a layered, crosstalk-aware regime.
//!
//! The protocol implemented here consumes already-captured direct-RB
//! observations. It does not execute quantum circuits.
//!
//! The architectural boundary is:
//!
//! ```text
//! generators / execution
//!          |
//!          v
//! direct-RB observations
//!          |
//!          v
//! protocols::layer_fidelity
//!          |
//!          +----> statistics::regression
//!          |
//!          +----> Layer Fidelity
//!          |
//!          +----> EPLG
//!          |
//!          v
//! core::result / reporting / analysis
//! ```
//!
//! # Scientific definition
//!
//! For each constituent subsystem `j` in a simultaneous layer, direct
//! randomized benchmarking estimates a decay parameter `alpha_j` from:
//!
//! ```text
//! P_j(m) = A_j * alpha_j^m + B_j
//! ```
//!
//! For a subsystem with Hilbert-space dimension `d`, the RB decay parameter
//! corresponds to a depolarizing-model error rate:
//!
//! ```text
//! r_j = (d - 1) / d * (1 - alpha_j)
//! ```
//!
//! The corresponding average gate fidelity is:
//!
//! ```text
//! F_avg,j = 1 - r_j
//! ```
//!
//! The corresponding process fidelity is:
//!
//! ```text
//! F_process,j = ((d + 1) * F_avg,j - 1) / d
//! ```
//!
//! which simplifies to:
//!
//! ```text
//! F_process,j = 1 - ((d^2 - 1) / d^2) * (1 - alpha_j)
//! ```
//!
//! For a one-qubit subsystem:
//!
//! ```text
//! d = 2
//! F_process = (1 + 3 * alpha) / 4
//! ```
//!
//! For a two-qubit subsystem:
//!
//! ```text
//! d = 4
//! F_process = (1 + 15 * alpha) / 16
//! ```
//!
//! The process fidelity of a simultaneous layer is:
//!
//! ```text
//! LF_layer = product(F_process,j)
//! ```
//!
//! For a complete layered benchmark consisting of multiple disjoint layers:
//!
//! ```text
//! LF = product(LF_layer)
//! ```
//!
//! For a layer containing `N_2Q` two-qubit operations:
//!
//! ```text
//! EPLG = 1 - LF^(1 / N_2Q)
//! ```
//!
//! EPLG is the process-error form of the metric.
//!
//! For two-qubit operations, IBM's current QPU reporting converts process
//! error to average-gate error using the factor:
//!
//! ```text
//! average_gate_error = 4 / 5 * process_error
//! ```
//!
//! Zamani therefore exposes both quantities explicitly and never silently
//! conflates them.
//!
//! # Important scientific limitation
//!
//! Layer Fidelity is not a proof that the underlying noise is depolarizing.
//! The conversion from RB decay to process fidelity is model-dependent.
//!
//! Results therefore expose:
//!
//! - fitted decay parameter;
//! - regression diagnostics;
//! - boundary status;
//! - process fidelity;
//! - uncertainty where available;
//! - assumptions;
//! - subsystem dimension;
//! - number of two-qubit gates;
//! - whether the result is an exact product or a lower-bound interpretation.
//!
//! When simultaneous errors or crosstalk violate independence assumptions,
//! the product construction must be interpreted conservatively.
//!
//! # Layer structure
//!
//! A Layer Fidelity experiment is represented as:
//!
//! ```text
//! Layer 0:
//!     (q0,q1)   (q2,q3)   (q4,q5)
//!
//! Layer 1:
//!     (q1,q2)   (q3,q4)
//!
//! Layer 2:
//!     ...
//! ```
//!
//! Each edge/subsystem must be disjoint within a layer.
//!
//! The protocol does not require layers themselves to be disjoint from one
//! another. In fact, connected chains normally reuse qubits across layers.
//!
//! # Direct-RB observations
//!
//! A caller supplies observations for each constituent subsystem:
//!
//! ```text
//! subsystem
//!     |
//!     +-- sequence length -> survival probability
//!     +-- sequence length -> survival probability
//!     +-- sequence length -> survival probability
//!     ...
//! ```
//!
//! The protocol performs the exponential fit through Zamani's canonical
//! `statistics::regression` implementation.
//!
//! This is important because `statistics::regression` already provides:
//!
//! - bounded decay-rate search;
//! - deterministic fitting;
//! - convergence diagnostics;
//! - R²;
//! - RMSE;
//! - AIC/BIC;
//! - covariance availability;
//! - parameter uncertainty;
//! - boundary detection.
//!
//! It would be incorrect for this protocol to maintain a second independent
//! exponential fitting implementation.
//!
//! # Execution boundary
//!
//! This file does NOT:
//!
//! - generate circuits;
//! - generate Clifford sequences;
//! - execute circuits;
//! - choose a backend;
//! - access hardware;
//! - access calibration services;
//! - perform routing;
//! - perform scheduling;
//! - lower to Quantum IR;
//! - communicate with a provider;
//! - print diagnostics;
//! - maintain global state.
//!
//! Those responsibilities remain in their owning layers.
//!
//! # Integration
//!
//! ```text
//! core::observation
//!       |
//!       v
//! execution::response
//!       |
//!       v
//! direct RB observations
//!       |
//!       v
//! protocols::layer_fidelity
//!       |
//!       +--> statistics::regression
//!       |
//!       +--> core::result
//!       |
//!       +--> metrics::fidelity
//!       +--> metrics::gate_error
//!       |
//!       v
//! reporting / analysis
//! ```
//!
//! The protocol can therefore be tested completely offline using synthetic
//! direct-RB observations.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! # Dependencies
//!
//! This file uses only dependencies already present in Zamani:
//!
//! - serde;
//! - the standard library;
//! - Zamani statistics::regression.
//!
//! No new crate dependency is required.

#![deny(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

use super::super::statistics::regression::{
    fit_exponential_decay,
    RegressionError,
    RegressionFit,
    RegressionObservation,
};

// ============================================================================
// Public constants
// ============================================================================

/// Stable protocol identifier.
pub const LAYER_FIDELITY_ID: &str = "layer_fidelity";

/// Stable schema version for Layer Fidelity results.
pub const LAYER_FIDELITY_SCHEMA_VERSION: u32 = 1;

/// Stable algorithm identifier.
///
/// Change this whenever the mathematical interpretation of the protocol
/// changes in a scientifically meaningful way.
pub const LAYER_FIDELITY_ALGORITHM_ID: &str =
    "zamani.layer_fidelity.direct_rb.v1";

/// Maximum number of layers accepted by the protocol.
///
/// This is a protocol-level safety boundary. Larger experiments can be
/// decomposed into multiple benchmark runs.
pub const DEFAULT_MAX_LAYERS: usize = 4_096;

/// Maximum number of constituent subsystems in one Layer Fidelity result.
pub const DEFAULT_MAX_SUBSYSTEMS: usize = 65_536;

/// Maximum number of observations per subsystem.
pub const DEFAULT_MAX_OBSERVATIONS_PER_SUBSYSTEM: usize = 4_096;

/// Minimum number of observations required for the exponential model.
pub const MIN_OBSERVATIONS_PER_SUBSYSTEM: usize = 4;

/// Numerical tolerance for probabilities and fidelities.
const UNIT_INTERVAL_EPSILON: f64 = 1.0e-12;

/// Two-qubit Hilbert-space dimension.
pub const TWO_QUBIT_DIMENSION: u32 = 4;

/// One-qubit Hilbert-space dimension.
pub const ONE_QUBIT_DIMENSION: u32 = 2;

// ============================================================================
// Subsystem kind
// ============================================================================

/// Physical/logical subsystem being benchmarked.
///
/// Layer Fidelity is commonly applied to two-qubit gates, but allowing
/// one-qubit subsystems is important because practical layered experiments can
/// contain spectator/idle or single-qubit operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayerSubsystemKind {
    /// One-qubit direct randomized benchmarking.
    OneQubit,

    /// Two-qubit direct randomized benchmarking.
    TwoQubit,
}

impl LayerSubsystemKind {
    /// Returns the Hilbert-space dimension.
    #[must_use]
    pub const fn dimension(self) -> u32 {
        match self {
            Self::OneQubit => ONE_QUBIT_DIMENSION,
            Self::TwoQubit => TWO_QUBIT_DIMENSION,
        }
    }

    /// Returns the number of physical qubits represented by the subsystem.
    #[must_use]
    pub const fn qubit_count(self) -> usize {
        match self {
            Self::OneQubit => 1,
            Self::TwoQubit => 2,
        }
    }

    /// Returns whether this subsystem contributes to the two-qubit EPLG
    /// denominator.
    #[must_use]
    pub const fn contributes_to_two_qubit_count(self) -> bool {
        matches!(self, Self::TwoQubit)
    }

    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OneQubit => "one_qubit",
            Self::TwoQubit => "two_qubit",
        }
    }
}

// ============================================================================
// Subsystem identifier
// ============================================================================

/// Stable identifier for one constituent operation/subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LayerSubsystemId(String);

impl LayerSubsystemId {
    /// Creates a validated identifier.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, LayerFidelityError> {
        let value = value.into();

        if value.is_empty() {
            return Err(LayerFidelityError::EmptySubsystemId);
        }

        if value.len() > 256 {
            return Err(LayerFidelityError::SubsystemIdTooLong {
                length: value.len(),
            });
        }

        if !value
            .chars()
            .all(|character| {
                character.is_ascii_alphanumeric()
                    || matches!(
                        character,
                        '_' | '-' | ':' | '.' | '(' | ')' | ','
                    )
            })
        {
            return Err(LayerFidelityError::InvalidSubsystemId);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LayerSubsystemId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Qubit set
// ============================================================================

/// Explicit qubit membership of one subsystem.
///
/// The protocol keeps this information instead of inferring topology from
/// subsystem names.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QubitSet {
    /// Physical or logical qubit identifiers.
    pub qubits: Vec<usize>,
}

impl QubitSet {
    /// Creates a validated qubit set for the supplied subsystem kind.
    pub fn new(
        kind: LayerSubsystemKind,
        qubits: Vec<usize>,
    ) -> Result<Self, LayerFidelityError> {
        if qubits.len() != kind.qubit_count() {
            return Err(LayerFidelityError::WrongQubitCount {
                kind,
                expected: kind.qubit_count(),
                actual: qubits.len(),
            });
        }

        let mut sorted = qubits;
        sorted.sort_unstable();

        for pair in sorted.windows(2) {
            if pair[0] == pair[1] {
                return Err(LayerFidelityError::DuplicateQubit {
                    qubit: pair[0],
                });
            }
        }

        Ok(Self { qubits: sorted })
    }

    /// Returns whether this set contains a qubit.
    #[must_use]
    pub fn contains(&self, qubit: usize) -> bool {
        self.qubits.binary_search(&qubit).is_ok()
    }

    /// Returns whether this set overlaps another set.
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.qubits
            .iter()
            .any(|qubit| other.contains(*qubit))
    }
}

// ============================================================================
// Layer constituent
// ============================================================================

/// One constituent subsystem in one simultaneous layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayerSubsystem {
    /// Stable subsystem identifier.
    pub id: LayerSubsystemId,

    /// One- or two-qubit subsystem.
    pub kind: LayerSubsystemKind,

    /// Qubits participating in this subsystem.
    pub qubits: QubitSet,
}

impl LayerSubsystem {
    /// Creates a validated layer subsystem.
    pub fn new(
        id: LayerSubsystemId,
        kind: LayerSubsystemKind,
        qubits: Vec<usize>,
    ) -> Result<Self, LayerFidelityError> {
        Ok(Self {
            id,
            kind,
            qubits: QubitSet::new(kind, qubits)?,
        })
    }
}

// ============================================================================
// Layer specification
// ============================================================================

/// One simultaneous layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayerSpec {
    /// Zero-based layer index.
    pub index: usize,

    /// Simultaneously benchmarked subsystems.
    pub subsystems: Vec<LayerSubsystem>,
}

impl LayerSpec {
    /// Creates a validated layer.
    pub fn new(
        index: usize,
        subsystems: Vec<LayerSubsystem>,
    ) -> Result<Self, LayerFidelityError> {
        if subsystems.is_empty() {
            return Err(LayerFidelityError::EmptyLayer { index });
        }

        validate_layer_subsystems(&subsystems, index)?;

        Ok(Self { index, subsystems })
    }

    /// Returns the number of two-qubit operations in this layer.
    #[must_use]
    pub fn two_qubit_count(&self) -> usize {
        self.subsystems
            .iter()
            .filter(|subsystem| {
                subsystem
                    .kind
                    .contributes_to_two_qubit_count()
            })
            .count()
    }

    /// Returns the total number of constituent subsystems.
    #[must_use]
    pub fn subsystem_count(&self) -> usize {
        self.subsystems.len()
    }
}

// ============================================================================
// Direct-RB observations
// ============================================================================

/// One survival-probability observation for one direct-RB subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LayerRbObservation {
    /// Number of repeated layers/cycles.
    pub sequence_length: u64,

    /// Number of successful survival measurements.
    pub successes: u64,

    /// Total number of shots.
    pub shots: u64,

    /// Optional inverse-variance weight for the regression.
    ///
    /// When omitted, the canonical regression engine uses weight 1.
    pub weight: Option<f64>,
}

impl LayerRbObservation {
    /// Creates an observation from successful and total shot counts.
    pub fn new(
        sequence_length: u64,
        successes: u64,
        shots: u64,
    ) -> Result<Self, LayerFidelityError> {
        Self::with_weight(sequence_length, successes, shots, None)
    }

    /// Creates an observation with an optional regression weight.
    pub fn with_weight(
        sequence_length: u64,
        successes: u64,
        shots: u64,
        weight: Option<f64>,
    ) -> Result<Self, LayerFidelityError> {
        if shots == 0 {
            return Err(LayerFidelityError::ZeroShots);
        }

        if successes > shots {
            return Err(LayerFidelityError::SuccessesExceedShots {
                successes,
                shots,
            });
        }

        if let Some(weight) = weight {
            if !weight.is_finite() || weight <= 0.0 {
                return Err(LayerFidelityError::InvalidWeight {
                    value: weight,
                });
            }
        }

        Ok(Self {
            sequence_length,
            successes,
            shots,
            weight,
        })
    }

    /// Returns the observed survival probability.
    pub fn survival_probability(
        &self,
    ) -> Result<f64, LayerFidelityError> {
        let probability =
            self.successes as f64 / self.shots as f64;

        validate_unit_interval(
            probability,
            "survival probability",
        )?;

        Ok(probability)
    }

    /// Converts the observation to Zamani's canonical regression format.
    pub fn as_regression_observation(
        &self,
    ) -> Result<RegressionObservation, LayerFidelityError> {
        Ok(RegressionObservation {
            x: self.sequence_length as f64,
            y: self.survival_probability()?,
            weight: self.weight,
        })
    }
}

// ============================================================================
// Subsystem experiment
// ============================================================================

/// Direct-RB observations for one constituent subsystem.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerSubsystemExperiment {
    /// Subsystem definition.
    pub subsystem: LayerSubsystem,

    /// Captured direct-RB observations.
    pub observations: Vec<LayerRbObservation>,
}

impl LayerSubsystemExperiment {
    /// Creates a subsystem experiment.
    pub fn new(
        subsystem: LayerSubsystem,
        observations: Vec<LayerRbObservation>,
    ) -> Result<Self, LayerFidelityError> {
        validate_subsystem_observations(&observations)?;

        Ok(Self {
            subsystem,
            observations,
        })
    }

    /// Returns the number of observations.
    #[must_use]
    pub fn observation_count(&self) -> usize {
        self.observations.len()
    }
}

// ============================================================================
// Layer Fidelity configuration
// ============================================================================

/// Configuration controlling Layer Fidelity analysis.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LayerFidelityConfig {
    /// Maximum accepted layer count.
    pub max_layers: usize,

    /// Maximum accepted constituent subsystem count.
    pub max_subsystems: usize,

    /// Maximum observations per subsystem.
    pub max_observations_per_subsystem: usize,

    /// Whether boundary RB fits are accepted.
    ///
    /// A boundary fit is not automatically invalid, but it is scientifically
    /// weaker. Production callers normally leave this false.
    pub allow_boundary_fits: bool,

    /// Whether non-converged regression fits are accepted.
    pub allow_non_converged_fits: bool,

    /// Whether the aggregate LF should be clipped to [0,1] after multiplication
    /// within floating-point tolerance.
    pub clip_numerical_rounding: bool,
}

impl Default for LayerFidelityConfig {
    fn default() -> Self {
        Self {
            max_layers: DEFAULT_MAX_LAYERS,
            max_subsystems: DEFAULT_MAX_SUBSYSTEMS,
            max_observations_per_subsystem:
                DEFAULT_MAX_OBSERVATIONS_PER_SUBSYSTEM,
            allow_boundary_fits: false,
            allow_non_converged_fits: false,
            clip_numerical_rounding: true,
        }
    }
}

impl LayerFidelityConfig {
    /// Validates configuration.
    pub fn validate(&self) -> Result<(), LayerFidelityError> {
        if self.max_layers == 0 {
            return Err(LayerFidelityError::InvalidConfiguration {
                reason: "max_layers must be greater than zero",
            });
        }

        if self.max_subsystems == 0 {
            return Err(LayerFidelityError::InvalidConfiguration {
                reason: "max_subsystems must be greater than zero",
            });
        }

        if self.max_observations_per_subsystem
            < MIN_OBSERVATIONS_PER_SUBSYSTEM
        {
            return Err(LayerFidelityError::InvalidConfiguration {
                reason:
                    "max_observations_per_subsystem is below the minimum \
                     required for exponential regression",
            });
        }

        Ok(())
    }
}

// ============================================================================
// Fit result
// ============================================================================

/// Process-fidelity estimate for one subsystem.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubsystemFidelityResult {
    /// Subsystem definition.
    pub subsystem: LayerSubsystem,

    /// Number of direct-RB observations.
    pub observations: usize,

    /// Fitted RB decay parameter alpha.
    pub decay_parameter: f64,

    /// Lower confidence bound for alpha when available.
    pub decay_parameter_lower: Option<f64>,

    /// Upper confidence bound for alpha when available.
    pub decay_parameter_upper: Option<f64>,

    /// Process fidelity derived from alpha.
    pub process_fidelity: f64,

    /// Lower process-fidelity bound when derivable.
    pub process_fidelity_lower: Option<f64>,

    /// Upper process-fidelity bound when derivable.
    pub process_fidelity_upper: Option<f64>,

    /// RB error-per-step quantity:
    ///
    /// `(d - 1) / d * (1 - alpha)`.
    pub rb_error_rate: f64,

    /// Full regression result for scientific diagnostics.
    pub regression: RegressionFit,
}

impl SubsystemFidelityResult {
    /// Returns the process-error quantity `1 - F_process`.
    #[must_use]
    pub fn process_error(&self) -> f64 {
        1.0 - self.process_fidelity
    }

    /// Returns the average-gate fidelity corresponding to the same
    /// depolarizing interpretation.
    #[must_use]
    pub fn average_gate_fidelity(&self) -> f64 {
        let dimension =
            self.subsystem.kind.dimension() as f64;

        1.0 - self.rb_error_rate
    }

    /// Returns the average-gate error corresponding to the depolarizing
    /// interpretation.
    #[must_use]
    pub fn average_gate_error(&self) -> f64 {
        1.0 - self.average_gate_fidelity()
    }
}

// ============================================================================
// Layer result
// ============================================================================

/// Result for one simultaneous layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerResult {
    /// Layer definition.
    pub layer: LayerSpec,

    /// Per-subsystem process-fidelity results.
    pub subsystems: Vec<SubsystemFidelityResult>,

    /// Product of constituent process fidelities.
    pub layer_fidelity: f64,

    /// Process error of the layer.
    pub layer_process_error: f64,

    /// Number of two-qubit operations in this layer.
    pub two_qubit_count: usize,
}

impl LayerResult {
    /// Returns the layer's two-qubit process error.
    #[must_use]
    pub fn process_error(&self) -> f64 {
        1.0 - self.layer_fidelity
    }
}

// ============================================================================
// Aggregate result
// ============================================================================

/// Complete Layer Fidelity result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerFidelityResult {
    /// Stable protocol identifier.
    pub benchmark_id: String,

    /// Protocol schema version.
    pub schema_version: u32,

    /// Stable analysis algorithm identifier.
    pub algorithm: String,

    /// Number of layers.
    pub layer_count: usize,

    /// Total number of constituent subsystems.
    pub subsystem_count: usize,

    /// Total number of two-qubit operations.
    pub two_qubit_gate_count: usize,

    /// Per-layer results.
    pub layers: Vec<LayerResult>,

    /// Aggregate Layer Fidelity.
    pub layer_fidelity: f64,

    /// Aggregate process error:
    ///
    /// `1 - LF`.
    pub layer_process_error: f64,

    /// Error per layered two-qubit gate:
    ///
    /// `1 - LF^(1/N_2Q)`.
    ///
    /// This is the process-error form of EPLG.
    pub eplg_process: Option<f64>,

    /// Average-gate-error presentation for two-qubit EPLG:
    ///
    /// `(4/5) * EPLG_process`.
    ///
    /// This field is only populated when there is at least one two-qubit gate.
    pub eplg_average_gate: Option<f64>,

    /// Whether the aggregate result should be interpreted as a lower bound
    /// under the simultaneous/crosstalk model.
    pub is_lower_bound_under_crosstalk: bool,

    /// Scientific warnings generated by the analysis.
    pub warnings: Vec<LayerFidelityWarning>,
}

impl LayerFidelityResult {
    /// Returns the process-form EPLG.
    #[must_use]
    pub fn eplg(&self) -> Option<f64> {
        self.eplg_process
    }

    /// Returns whether this result contains two-qubit EPLG.
    #[must_use]
    pub fn has_eplg(&self) -> bool {
        self.eplg_process.is_some()
    }
}

// ============================================================================
// Warnings
// ============================================================================

/// Structured Layer Fidelity scientific warning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayerFidelityWarning {
    /// A direct-RB fit landed at a model boundary.
    BoundaryFit {
        subsystem: String,
    },

    /// A direct-RB fit did not converge within the configured iteration
    /// budget.
    NonConvergedFit {
        subsystem: String,
    },

    /// A regression covariance estimate was unavailable.
    UncertaintyUnavailable {
        subsystem: String,
    },

    /// The aggregate result includes simultaneous operations, so crosstalk
    /// may make independent-product interpretation conservative.
    SimultaneousCrosstalkInterpretation,

    /// The process-to-average-gate conversion was applied.
    AverageGateConversionApplied,

    /// No two-qubit operations exist, so EPLG is undefined.
    NoTwoQubitGates,

    /// The supplied experiment contains one-qubit subsystems.
    IncludesOneQubitSubsystems,
}

impl LayerFidelityWarning {
    /// Stable machine-readable warning identifier.
    #[must_use]
    pub const fn id(&self) -> &'static str {
        match self {
            Self::BoundaryFit { .. } => "boundary_fit",
            Self::NonConvergedFit { .. } => "non_converged_fit",
            Self::UncertaintyUnavailable { .. } => {
                "uncertainty_unavailable"
            }
            Self::SimultaneousCrosstalkInterpretation => {
                "simultaneous_crosstalk_interpretation"
            }
            Self::AverageGateConversionApplied => {
                "average_gate_conversion_applied"
            }
            Self::NoTwoQubitGates => "no_two_qubit_gates",
            Self::IncludesOneQubitSubsystems => {
                "includes_one_qubit_subsystems"
            }
        }
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by Layer Fidelity analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LayerFidelityError {
    /// Configuration is invalid.
    InvalidConfiguration {
        reason: &'static str,
    },

    /// No layers were supplied.
    EmptyExperiment,

    /// Too many layers were supplied.
    TooManyLayers {
        requested: usize,
        maximum: usize,
    },

    /// Too many subsystems were supplied.
    TooManySubsystems {
        requested: usize,
        maximum: usize,
    },

    /// A layer contains no subsystem.
    EmptyLayer {
        index: usize,
    },

    /// A subsystem identifier is empty.
    EmptySubsystemId,

    /// A subsystem identifier is too long.
    SubsystemIdTooLong {
        length: usize,
    },

    /// A subsystem identifier contains unsupported characters.
    InvalidSubsystemId,

    /// A subsystem contains the wrong number of qubits.
    WrongQubitCount {
        kind: LayerSubsystemKind,
        expected: usize,
        actual: usize,
    },

    /// A qubit occurs twice in a subsystem.
    DuplicateQubit {
        qubit: usize,
    },

    /// Two simultaneous subsystems overlap.
    OverlappingSubsystems {
        layer: usize,
        first: String,
        second: String,
    },

    /// A subsystem has too few observations.
    InsufficientObservations {
        subsystem: String,
        observations: usize,
        minimum: usize,
    },

    /// A subsystem has too many observations.
    TooManyObservations {
        subsystem: String,
        observations: usize,
        maximum: usize,
    },

    /// A shot count is zero.
    ZeroShots,

    /// Successful shots exceed total shots.
    SuccessesExceedShots {
        successes: u64,
        shots: u64,
    },

    /// Regression weight is invalid.
    InvalidWeight {
        value: f64,
    },

    /// A probability/fidelity is outside [0,1].
    InvalidProbability {
        field: &'static str,
        value: f64,
    },

    /// The fitted RB parameter is outside the expected physical interval.
    InvalidDecayParameter {
        subsystem: String,
        value: f64,
    },

    /// The derived process fidelity is outside [0,1].
    InvalidProcessFidelity {
        subsystem: String,
        value: f64,
    },

    /// The aggregate LF could not be calculated.
    InvalidAggregateFidelity,

    /// EPLG cannot be calculated.
    EplgUndefined,

    /// A fit is at a statistical boundary and the production configuration
    /// does not allow it.
    BoundaryFitRejected {
        subsystem: String,
    },

    /// A fit failed to converge and production configuration does not allow
    /// the result.
    NonConvergedFitRejected {
        subsystem: String,
    },

    /// The underlying canonical regression failed.
    Regression(RegressionError),
}

impl fmt::Display for LayerFidelityError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { reason } => {
                write!(
                    formatter,
                    "invalid Layer Fidelity configuration: {reason}"
                )
            }

            Self::EmptyExperiment => {
                write!(
                    formatter,
                    "Layer Fidelity requires at least one layer"
                )
            }

            Self::TooManyLayers {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "Layer Fidelity requested {requested} layers; \
                     maximum is {maximum}"
                )
            }

            Self::TooManySubsystems {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "Layer Fidelity requested {requested} \
                     subsystems; maximum is {maximum}"
                )
            }

            Self::EmptyLayer { index } => {
                write!(
                    formatter,
                    "Layer {index} contains no subsystems"
                )
            }

            Self::EmptySubsystemId => {
                write!(
                    formatter,
                    "Layer Fidelity subsystem identifier cannot be empty"
                )
            }

            Self::SubsystemIdTooLong { length } => {
                write!(
                    formatter,
                    "Layer Fidelity subsystem identifier is \
                     {length} bytes; maximum is 256"
                )
            }

            Self::InvalidSubsystemId => {
                write!(
                    formatter,
                    "Layer Fidelity subsystem identifier contains \
                     unsupported characters"
                )
            }

            Self::WrongQubitCount {
                kind,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "{} subsystem requires {expected} qubits, got {actual}",
                    kind.as_str()
                )
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    formatter,
                    "qubit {qubit} occurs more than once in a subsystem"
                )
            }

            Self::OverlappingSubsystems {
                layer,
                first,
                second,
            } => {
                write!(
                    formatter,
                    "Layer {layer} contains overlapping simultaneous \
                     subsystems '{first}' and '{second}'"
                )
            }

            Self::InsufficientObservations {
                subsystem,
                observations,
                minimum,
            } => {
                write!(
                    formatter,
                    "subsystem '{subsystem}' has {observations} \
                     observations; minimum is {minimum}"
                )
            }

            Self::TooManyObservations {
                subsystem,
                observations,
                maximum,
            } => {
                write!(
                    formatter,
                    "subsystem '{subsystem}' has {observations} \
                     observations; maximum is {maximum}"
                )
            }

            Self::ZeroShots => {
                write!(formatter, "shot count must be greater than zero")
            }

            Self::SuccessesExceedShots {
                successes,
                shots,
            } => {
                write!(
                    formatter,
                    "success count {successes} exceeds shot count {shots}"
                )
            }

            Self::InvalidWeight { value } => {
                write!(
                    formatter,
                    "regression weight must be finite and positive, \
                     got {value}"
                )
            }

            Self::InvalidProbability { field, value } => {
                write!(
                    formatter,
                    "{field} must be finite and within [0,1], got {value}"
                )
            }

            Self::InvalidDecayParameter {
                subsystem,
                value,
            } => {
                write!(
                    formatter,
                    "subsystem '{subsystem}' has invalid decay \
                     parameter {value}"
                )
            }

            Self::InvalidProcessFidelity {
                subsystem,
                value,
            } => {
                write!(
                    formatter,
                    "subsystem '{subsystem}' has invalid process \
                     fidelity {value}"
                )
            }

            Self::InvalidAggregateFidelity => {
                write!(
                    formatter,
                    "aggregate Layer Fidelity is non-finite or outside \
                     [0,1]"
                )
            }

            Self::EplgUndefined => {
                write!(
                    formatter,
                    "EPLG is undefined because no two-qubit \
                     subsystem exists"
                )
            }

            Self::BoundaryFitRejected { subsystem } => {
                write!(
                    formatter,
                    "direct-RB fit for subsystem '{subsystem}' \
                     terminated at a model boundary"
                )
            }

            Self::NonConvergedFitRejected { subsystem } => {
                write!(
                    formatter,
                    "direct-RB fit for subsystem '{subsystem}' \
                     did not converge"
                )
            }

            Self::Regression(error) => {
                write!(
                    formatter,
                    "Layer Fidelity regression failed: {error}"
                )
            }
        }
    }
}

impl Error for LayerFidelityError {}

impl From<RegressionError> for LayerFidelityError {
    fn from(error: RegressionError) -> Self {
        Self::Regression(error)
    }
}

// ============================================================================
// Analyzer
// ============================================================================

/// Production Layer Fidelity analyzer.
///
/// This type owns protocol validation and interpretation while delegating
/// exponential fitting to Zamani's canonical regression engine.
#[derive(Debug, Clone, Copy)]
pub struct LayerFidelityAnalyzer {
    config: LayerFidelityConfig,
}

impl LayerFidelityAnalyzer {
    /// Creates an analyzer with production defaults.
    pub fn production() -> Result<Self, LayerFidelityError> {
        Self::new(LayerFidelityConfig::default())
    }

    /// Creates an analyzer with explicit configuration.
    pub fn new(
        config: LayerFidelityConfig,
    ) -> Result<Self, LayerFidelityError> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the analyzer configuration.
    #[must_use]
    pub const fn config(&self) -> LayerFidelityConfig {
        self.config
    }

    /// Analyzes a complete Layer Fidelity experiment.
    ///
    /// This function is deterministic for a fixed set of observations and
    /// regression configuration.
    pub fn analyze(
        &self,
        layers: &[LayerSpec],
        experiments: &[LayerSubsystemExperiment],
    ) -> Result<LayerFidelityResult, LayerFidelityError> {
        self.validate_experiment(layers, experiments)?;

        let mut layer_results =
            Vec::with_capacity(layers.len());

        let mut total_two_qubit_count = 0usize;

        for layer in layers {
            let mut subsystem_results = Vec::with_capacity(
                layer.subsystems.len(),
            );

            for subsystem in &layer.subsystems {
                let experiment = find_experiment(
                    experiments,
                    &subsystem.id,
                )?;

                let result =
                    self.analyze_subsystem(experiment)?;

                subsystem_results.push(result);
            }

            let layer_fidelity =
                product_fidelity(&subsystem_results)?;

            let two_qubit_count =
                layer.two_qubit_count();

            total_two_qubit_count = total_two_qubit_count
                .checked_add(two_qubit_count)
                .ok_or(
                    LayerFidelityError::InvalidConfiguration {
                        reason:
                            "two-qubit operation count overflowed",
                    },
                )?;

            layer_results.push(LayerResult {
                layer: layer.clone(),
                subsystems: subsystem_results,
                layer_fidelity,
                layer_process_error: 1.0 - layer_fidelity,
                two_qubit_count,
            });
        }

        let aggregate_fidelity =
            product_layer_fidelity(&layer_results)?;

        let layer_process_error =
            1.0 - aggregate_fidelity;

        let eplg_process =
            calculate_eplg(
                aggregate_fidelity,
                total_two_qubit_count,
            )?;

        let eplg_average_gate =
            eplg_process.map(|value| {
                // For two-qubit gates:
                //
                // average gate error
                //     = (d / (d + 1)) * process error
                //     = 4 / 5 * process error.
                4.0 / 5.0 * value
            });

        let mut warnings = Vec::new();

        if total_two_qubit_count == 0 {
            warnings.push(
                LayerFidelityWarning::NoTwoQubitGates,
            );
        } else {
            warnings.push(
                LayerFidelityWarning::AverageGateConversionApplied,
            );
        }

        if layers.iter().any(|layer| {
            layer.subsystems.iter().any(|subsystem| {
                matches!(
                    subsystem.kind,
                    LayerSubsystemKind::OneQubit
                )
            })
        }) {
            warnings.push(
                LayerFidelityWarning::IncludesOneQubitSubsystems,
            );
        }

        warnings.push(
            LayerFidelityWarning::SimultaneousCrosstalkInterpretation,
        );

        for layer_result in &layer_results {
            for subsystem_result in &layer_result.subsystems {
                let diagnostics =
                    &subsystem_result.regression.diagnostics;

                if diagnostics.boundary_solution {
                    warnings.push(
                        LayerFidelityWarning::BoundaryFit {
                            subsystem: subsystem_result
                                .subsystem
                                .id
                                .as_str()
                                .to_owned(),
                        },
                    );
                }

                if !diagnostics.converged {
                    warnings.push(
                        LayerFidelityWarning::NonConvergedFit {
                            subsystem: subsystem_result
                                .subsystem
                                .id
                                .as_str()
                                .to_owned(),
                        },
                    );
                }

                if !diagnostics.covariance_available {
                    warnings.push(
                        LayerFidelityWarning::UncertaintyUnavailable {
                            subsystem: subsystem_result
                                .subsystem
                                .id
                                .as_str()
                                .to_owned(),
                        },
                    );
                }
            }
        }

        Ok(LayerFidelityResult {
            benchmark_id:
                LAYER_FIDELITY_ID.to_owned(),
            schema_version:
                LAYER_FIDELITY_SCHEMA_VERSION,
            algorithm:
                LAYER_FIDELITY_ALGORITHM_ID.to_owned(),
            layer_count: layers.len(),
            subsystem_count: experiments.len(),
            two_qubit_gate_count:
                total_two_qubit_count,
            layers: layer_results,
            layer_fidelity: aggregate_fidelity,
            layer_process_error,
            eplg_process,
            eplg_average_gate,
            is_lower_bound_under_crosstalk: true,
            warnings,
        })
    }

    /// Analyze one direct-RB subsystem.
    fn analyze_subsystem(
        &self,
        experiment: &LayerSubsystemExperiment,
    ) -> Result<SubsystemFidelityResult, LayerFidelityError> {
        let regression_observations = experiment
            .observations
            .iter()
            .map(LayerRbObservation::as_regression_observation)
            .collect::<Result<Vec<_>, _>>()?;

        let regression =
            fit_exponential_decay(&regression_observations)?;

        let alpha =
            regression.decay_parameter.value;

        if !alpha.is_finite()
            || !(-UNIT_INTERVAL_EPSILON..=1.0
                + UNIT_INTERVAL_EPSILON)
                .contains(&alpha)
        {
            return Err(
                LayerFidelityError::InvalidDecayParameter {
                    subsystem: experiment
                        .subsystem
                        .id
                        .as_str()
                        .to_owned(),
                    value: alpha,
                },
            );
        }

        if regression.diagnostics.boundary_solution
            && !self.config.allow_boundary_fits
        {
            return Err(
                LayerFidelityError::BoundaryFitRejected {
                    subsystem: experiment
                        .subsystem
                        .id
                        .as_str()
                        .to_owned(),
                },
            );
        }

        if !regression.diagnostics.converged
            && !self.config.allow_non_converged_fits
        {
            return Err(
                LayerFidelityError::NonConvergedFitRejected {
                    subsystem: experiment
                        .subsystem
                        .id
                        .as_str()
                        .to_owned(),
                },
            );
        }

        let alpha =
            alpha.clamp(0.0, 1.0);

        let dimension =
            experiment.subsystem.kind.dimension()
                as f64;

        let rb_error_rate =
            ((dimension - 1.0) / dimension)
                * (1.0 - alpha);

        if !rb_error_rate.is_finite()
            || !(-UNIT_INTERVAL_EPSILON
                ..=1.0 + UNIT_INTERVAL_EPSILON)
                .contains(&rb_error_rate)
        {
            return Err(
                LayerFidelityError::InvalidProcessFidelity {
                    subsystem: experiment
                        .subsystem
                        .id
                        .as_str()
                        .to_owned(),
                    value: rb_error_rate,
                },
            );
        }

        let process_fidelity =
            1.0 - ((dimension * dimension - 1.0)
                / (dimension * dimension))
                * (1.0 - alpha);

        if !process_fidelity.is_finite()
            || !(-UNIT_INTERVAL_EPSILON
                ..=1.0 + UNIT_INTERVAL_EPSILON)
                .contains(&process_fidelity)
        {
            return Err(
                LayerFidelityError::InvalidProcessFidelity {
                    subsystem: experiment
                        .subsystem
                        .id
                        .as_str()
                        .to_owned(),
                    value: process_fidelity,
                },
            );
        }

        let process_fidelity =
            process_fidelity.clamp(0.0, 1.0);

        let (
            process_lower,
            process_upper,
        ) = process_fidelity_bounds(
            &regression,
            dimension,
        );

        Ok(SubsystemFidelityResult {
            subsystem: experiment.subsystem.clone(),
            observations: experiment.observations.len(),
            decay_parameter: alpha,
            decay_parameter_lower:
                regression.decay_parameter.lower,
            decay_parameter_upper:
                regression.decay_parameter.upper,
            process_fidelity,
            process_fidelity_lower:
                process_lower,
            process_fidelity_upper:
                process_upper,
            rb_error_rate,
            regression,
        })
    }

    fn validate_experiment(
        &self,
        layers: &[LayerSpec],
        experiments: &[LayerSubsystemExperiment],
    ) -> Result<(), LayerFidelityError> {
        if layers.is_empty() {
            return Err(
                LayerFidelityError::EmptyExperiment
            );
        }

        if layers.len() > self.config.max_layers {
            return Err(
                LayerFidelityError::TooManyLayers {
                    requested: layers.len(),
                    maximum: self.config.max_layers,
                },
            );
        }

        if experiments.is_empty() {
            return Err(
                LayerFidelityError::EmptyExperiment
            );
        }

        if experiments.len() > self.config.max_subsystems {
            return Err(
                LayerFidelityError::TooManySubsystems {
                    requested: experiments.len(),
                    maximum: self.config.max_subsystems,
                },
            );
        }

        let mut expected_ids =
            Vec::<String>::new();

        for layer in layers {
            for subsystem in &layer.subsystems {
                let id =
                    subsystem.id.as_str().to_owned();

                if expected_ids.iter().any(|existing| {
                    existing == &id
                }) {
                    continue;
                }

                expected_ids.push(id);
            }
        }

        for experiment in experiments {
            if !expected_ids.iter().any(|id| {
                id == experiment.subsystem.id.as_str()
            }) {
                return Err(
                    LayerFidelityError::InvalidConfiguration {
                        reason:
                            "an experiment was supplied for a \
                             subsystem that is not present in \
                             the layer specification",
                    },
                );
            }

            if experiment.observations.len()
                < MIN_OBSERVATIONS_PER_SUBSYSTEM
            {
                return Err(
                    LayerFidelityError::InsufficientObservations {
                        subsystem: experiment
                            .subsystem
                            .id
                            .as_str()
                            .to_owned(),
                        observations:
                            experiment.observations.len(),
                        minimum:
                            MIN_OBSERVATIONS_PER_SUBSYSTEM,
                    },
                );
            }

            if experiment.observations.len()
                > self.config.max_observations_per_subsystem
            {
                return Err(
                    LayerFidelityError::TooManyObservations {
                        subsystem: experiment
                            .subsystem
                            .id
                            .as_str()
                            .to_owned(),
                        observations:
                            experiment.observations.len(),
                        maximum: self
                            .config
                            .max_observations_per_subsystem,
                    },
                );
            }
        }

        for layer in layers {
            validate_layer_subsystems(
                &layer.subsystems,
                layer.index,
            )?;
        }

        for expected_id in &expected_ids {
            if !experiments.iter().any(|experiment| {
                experiment.subsystem.id.as_str()
                    == expected_id
            }) {
                return Err(
                    LayerFidelityError::InvalidConfiguration {
                        reason:
                            "a layer subsystem is missing its \
                             direct-RB observations",
                    },
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// Mathematical helpers
// ============================================================================

/// Calculates process-fidelity confidence bounds from alpha confidence
/// bounds.
///
/// Since process fidelity is monotonically increasing in alpha, the bounds
/// transform directly.
fn process_fidelity_bounds(
    regression: &RegressionFit,
    dimension: f64,
) -> (Option<f64>, Option<f64>) {
    let coefficient =
        (dimension * dimension - 1.0)
            / (dimension * dimension);

    let lower =
        regression.decay_parameter.lower
            .map(|alpha| {
                (1.0 - coefficient * (1.0 - alpha))
                    .clamp(0.0, 1.0)
            });

    let upper =
        regression.decay_parameter.upper
            .map(|alpha| {
                (1.0 - coefficient * (1.0 - alpha))
                    .clamp(0.0, 1.0)
            });

    (lower, upper)
}

/// Calculates the product of subsystem fidelities.
fn product_fidelity(
    results: &[SubsystemFidelityResult],
) -> Result<f64, LayerFidelityError> {
    if results.is_empty() {
        return Err(
            LayerFidelityError::InvalidAggregateFidelity
        );
    }

    let mut product = 1.0_f64;

    for result in results {
        product *= result.process_fidelity;

        if !product.is_finite() {
            return Err(
                LayerFidelityError::InvalidAggregateFidelity
            );
        }
    }

    Ok(product.clamp(0.0, 1.0))
}

/// Calculates aggregate Layer Fidelity from all layers.
fn product_layer_fidelity(
    layers: &[LayerResult],
) -> Result<f64, LayerFidelityError> {
    if layers.is_empty() {
        return Err(
            LayerFidelityError::InvalidAggregateFidelity
        );
    }

    let mut product = 1.0_f64;

    for layer in layers {
        product *= layer.layer_fidelity;

        if !product.is_finite() {
            return Err(
                LayerFidelityError::InvalidAggregateFidelity
            );
        }
    }

    Ok(product.clamp(0.0, 1.0))
}

/// Calculates process-form EPLG.
///
/// ```text
/// EPLG = 1 - LF^(1/N_2Q)
/// ```
fn calculate_eplg(
    layer_fidelity: f64,
    two_qubit_count: usize,
) -> Result<Option<f64>, LayerFidelityError> {
    if two_qubit_count == 0 {
        return Ok(None);
    }

    if !layer_fidelity.is_finite()
        || !(0.0..=1.0).contains(&layer_fidelity)
    {
        return Err(
            LayerFidelityError::InvalidAggregateFidelity
        );
    }

    let exponent =
        1.0 / two_qubit_count as f64;

    let root =
        layer_fidelity.powf(exponent);

    if !root.is_finite() {
        return Err(
            LayerFidelityError::EplgUndefined
        );
    }

    let eplg =
        (1.0 - root).clamp(0.0, 1.0);

    Ok(Some(eplg))
}

/// Validates a unit-interval value.
fn validate_unit_interval(
    value: f64,
    field: &'static str,
) -> Result<(), LayerFidelityError> {
    if !value.is_finite()
        || value < -UNIT_INTERVAL_EPSILON
        || value > 1.0 + UNIT_INTERVAL_EPSILON
    {
        return Err(
            LayerFidelityError::InvalidProbability {
                field,
                value,
            },
        );
    }

    Ok(())
}

/// Validates observations for one subsystem.
fn validate_subsystem_observations(
    observations: &[LayerRbObservation],
) -> Result<(), LayerFidelityError> {
    if observations.len()
        < MIN_OBSERVATIONS_PER_SUBSYSTEM
    {
        return Err(
            LayerFidelityError::InsufficientObservations {
                subsystem: "unknown".to_owned(),
                observations: observations.len(),
                minimum: MIN_OBSERVATIONS_PER_SUBSYSTEM,
            },
        );
    }

    for observation in observations {
        let _ =
            observation.survival_probability()?;
    }

    Ok(())
}

/// Validates that simultaneous subsystems are disjoint.
fn validate_layer_subsystems(
    subsystems: &[LayerSubsystem],
    layer_index: usize,
) -> Result<(), LayerFidelityError> {
    if subsystems.is_empty() {
        return Err(
            LayerFidelityError::EmptyLayer {
                index: layer_index,
            },
        );
    }

    for first_index in 0..subsystems.len() {
        for second_index in
            (first_index + 1)..subsystems.len()
        {
            let first =
                &subsystems[first_index];

            let second =
                &subsystems[second_index];

            if first.qubits.overlaps(
                &second.qubits,
            ) {
                return Err(
                    LayerFidelityError::OverlappingSubsystems {
                        layer: layer_index,
                        first: first.id.as_str().to_owned(),
                        second: second.id.as_str().to_owned(),
                    },
                );
            }
        }
    }

    Ok(())
}

/// Finds an experiment by subsystem identifier.
fn find_experiment<'a>(
    experiments: &'a [LayerSubsystemExperiment],
    id: &LayerSubsystemId,
) -> Result<&'a LayerSubsystemExperiment, LayerFidelityError> {
    experiments
        .iter()
        .find(|experiment| {
            &experiment.subsystem.id == id
        })
        .ok_or(
            LayerFidelityError::InvalidConfiguration {
                reason:
                    "layer subsystem has no corresponding \
                     direct-RB experiment",
            },
        )
}

// ============================================================================
// Convenience API
// ============================================================================

/// Analyzes Layer Fidelity using production configuration.
pub fn analyze_layer_fidelity(
    layers: &[LayerSpec],
    experiments: &[LayerSubsystemExperiment],
) -> Result<LayerFidelityResult, LayerFidelityError> {
    LayerFidelityAnalyzer::production()?
        .analyze(layers, experiments)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn subsystem(
        id: &str,
        kind: LayerSubsystemKind,
        qubits: Vec<usize>,
    ) -> LayerSubsystem {
        LayerSubsystem::new(
            LayerSubsystemId::new(id).unwrap(),
            kind,
            qubits,
        )
        .unwrap()
    }

    fn synthetic_observations(
        alpha: f64,
    ) -> Vec<LayerRbObservation> {
        let amplitude = 0.45;
        let offset = 0.50;

        [1_u64, 2, 4, 8, 16, 32]
            .iter()
            .map(|length| {
                let probability =
                    amplitude
                        * alpha.powf(*length as f64)
                        + offset;

                let shots = 100_000_u64;

                let successes =
                    (probability * shots as f64)
                        .round() as u64;

                LayerRbObservation::new(
                    *length,
                    successes,
                    shots,
                )
                .unwrap()
            })
            .collect()
    }

    #[test]
    fn two_qubit_dimension_is_four() {
        assert_eq!(
            LayerSubsystemKind::TwoQubit.dimension(),
            4
        );
    }

    #[test]
    fn one_qubit_dimension_is_two() {
        assert_eq!(
            LayerSubsystemKind::OneQubit.dimension(),
            2
        );
    }

    #[test]
    fn simultaneous_subsystems_must_be_disjoint() {
        let first =
            subsystem(
                "q0_q1",
                LayerSubsystemKind::TwoQubit,
                vec![0, 1],
            );

        let second =
            subsystem(
                "q1_q2",
                LayerSubsystemKind::TwoQubit,
                vec![1, 2],
            );

        let result =
            LayerSpec::new(
                0,
                vec![first, second],
            );

        assert!(matches!(
            result,
            Err(
                LayerFidelityError::OverlappingSubsystems {
                    ..
                }
            )
        ));
    }

    #[test]
    fn simultaneous_subsystems_can_be_disjoint() {
        let first =
            subsystem(
                "q0_q1",
                LayerSubsystemKind::TwoQubit,
                vec![0, 1],
            );

        let second =
            subsystem(
                "q2_q3",
                LayerSubsystemKind::TwoQubit,
                vec![2, 3],
            );

        let layer =
            LayerSpec::new(
                0,
                vec![first, second],
            )
            .unwrap();

        assert_eq!(
            layer.two_qubit_count(),
            2
        );
    }

    #[test]
    fn survival_probability_is_correct() {
        let observation =
            LayerRbObservation::new(
                10,
                750,
                1_000,
            )
            .unwrap();

        assert!(
            (observation
                .survival_probability()
                .unwrap()
                - 0.75)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn successes_cannot_exceed_shots() {
        assert!(matches!(
            LayerRbObservation::new(
                10,
                1_001,
                1_000
            ),
            Err(
                LayerFidelityError::SuccessesExceedShots {
                    ..
                }
            )
        ));
    }

    #[test]
    fn two_qubit_process_fidelity_formula_is_correct() {
        let alpha = 0.98;
        let dimension = 4.0;

        let fidelity =
            1.0
                - ((dimension * dimension - 1.0)
                    / (dimension * dimension))
                    * (1.0 - alpha);

        let expected =
            (1.0 + 15.0 * alpha) / 16.0;

        assert!(
            (fidelity - expected).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn analyzer_recovers_synthetic_two_qubit_fidelity() {
        let subsystem =
            subsystem(
                "q0_q1",
                LayerSubsystemKind::TwoQubit,
                vec![0, 1],
            );

        let experiment =
            LayerSubsystemExperiment::new(
                subsystem.clone(),
                synthetic_observations(0.98),
            )
            .unwrap();

        let layer =
            LayerSpec::new(
                0,
                vec![subsystem],
            )
            .unwrap();

        let analyzer =
            LayerFidelityAnalyzer::new(
                LayerFidelityConfig {
                    allow_boundary_fits: true,
                    allow_non_converged_fits: true,
                    ..LayerFidelityConfig::default()
                },
            )
            .unwrap();

        let result =
            analyzer
                .analyze(
                    &[layer],
                    &[experiment],
                )
                .unwrap();

        let expected =
            (1.0 + 15.0 * 0.98) / 16.0;

        assert!(
            (result.layer_fidelity - expected).abs()
                < 0.01,
            "LF={} expected={}",
            result.layer_fidelity,
            expected
        );

        assert_eq!(
            result.two_qubit_gate_count,
            1
        );

        assert!(
            result.eplg_process.is_some()
        );

        assert!(
            result.eplg_average_gate.is_some()
        );
    }

    #[test]
    fn eplg_is_zero_for_perfect_fidelity() {
        let eplg =
            calculate_eplg(
                1.0,
                99,
            )
            .unwrap()
            .unwrap();

        assert_eq!(eplg, 0.0);
    }

    #[test]
    fn eplg_is_one_for_zero_fidelity() {
        let eplg =
            calculate_eplg(
                0.0,
                99,
            )
            .unwrap()
            .unwrap();

        assert_eq!(eplg, 1.0);
    }

    #[test]
    fn eplg_matches_definition() {
        let lf = 0.81;
        let n = 9;

        let expected =
            1.0
                - lf.powf(1.0 / n as f64);

        let actual =
            calculate_eplg(lf, n)
                .unwrap()
                .unwrap();

        assert!(
            (actual - expected).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn no_eplg_without_two_qubit_gates() {
        assert_eq!(
            calculate_eplg(0.99, 0)
                .unwrap(),
            None
        );
    }

    #[test]
    fn repeated_analysis_is_deterministic() {
        let subsystem =
            subsystem(
                "q0_q1",
                LayerSubsystemKind::TwoQubit,
                vec![0, 1],
            );

        let experiment =
            LayerSubsystemExperiment::new(
                subsystem.clone(),
                synthetic_observations(0.99),
            )
            .unwrap();

        let layer =
            LayerSpec::new(
                0,
                vec![subsystem],
            )
            .unwrap();

        let analyzer =
            LayerFidelityAnalyzer::new(
                LayerFidelityConfig {
                    allow_boundary_fits: true,
                    allow_non_converged_fits: true,
                    ..LayerFidelityConfig::default()
                },
            )
            .unwrap();

        let first =
            analyzer
                .analyze(
                    &[layer.clone()],
                    &[experiment.clone()],
                )
                .unwrap();

        let second =
            analyzer
                .analyze(
                    &[layer],
                    &[experiment],
                )
                .unwrap();

        assert_eq!(
            first.layer_fidelity,
            second.layer_fidelity
        );

        assert_eq!(
            first.eplg_process,
            second.eplg_process
        );
    }

    #[test]
    fn missing_subsystem_experiment_is_rejected() {
        let layer =
            LayerSpec::new(
                0,
                vec![
                    subsystem(
                        "q0_q1",
                        LayerSubsystemKind::TwoQubit,
                        vec![0, 1],
                    ),
                ],
            )
            .unwrap();

        let analyzer =
            LayerFidelityAnalyzer::production()
                .unwrap();

        let result =
            analyzer.analyze(
                &[layer],
                &[],
            );

        assert!(result.is_err());
    }

    #[test]
    fn one_qubit_eplg_is_not_falsely_reported_as_two_qubit_eplg() {
        let subsystem =
            subsystem(
                "q0",
                LayerSubsystemKind::OneQubit,
                vec![0],
            );

        let experiment =
            LayerSubsystemExperiment::new(
                subsystem.clone(),
                synthetic_observations(0.99),
            )
            .unwrap();

        let layer =
            LayerSpec::new(
                0,
                vec![subsystem],
            )
            .unwrap();

        let analyzer =
            LayerFidelityAnalyzer::new(
                LayerFidelityConfig {
                    allow_boundary_fits: true,
                    allow_non_converged_fits: true,
                    ..LayerFidelityConfig::default()
                },
            )
            .unwrap();

        let result =
            analyzer
                .analyze(
                    &[layer],
                    &[experiment],
                )
                .unwrap();

        assert_eq!(
            result.two_qubit_gate_count,
            0
        );

        assert_eq!(
            result.eplg_process,
            None
        );

        assert_eq!(
            result.eplg_average_gate,
            None
        );
    }

    #[test]
    fn subsystem_id_validation_rejects_invalid_characters() {
        let result =
            LayerSubsystemId::new("q0 q1");

        assert!(matches!(
            result,
            Err(
                LayerFidelityError::InvalidSubsystemId
            )
        ));
    }
}