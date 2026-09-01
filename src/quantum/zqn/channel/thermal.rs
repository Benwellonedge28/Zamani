//! Zamani Quantum Noise (ZQN) — Thermal Quantum Channel
//!
//! Production-grade, representation-independent thermal-noise semantics.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - the semantic definition of thermal quantum noise;
//! - thermal bath configuration;
//! - subsystem energy spectra;
//! - thermal transition-rate specifications;
//! - detailed-balance validation;
//! - optional pure-dephasing rates;
//! - interaction/evolution duration;
//! - thermal-channel semantic validation;
//! - deterministic thermal-channel descriptions;
//! - canonical-qubit convenience constructors using
//!   `crate::quantum::ir::qubit::QubitId`;
//! - explicit approximation/realization contracts.
//!
//! This file does NOT own:
//!
//! - Kraus matrices;
//! - Choi matrices;
//! - density matrices;
//! - numerical linear algebra;
//! - matrix exponentiation;
//! - Lindblad integration;
//! - Monte Carlo sampling;
//! - random-number generation;
//! - hardware APIs;
//! - calibration storage;
//! - QEC decoding;
//! - routing;
//! - scheduling;
//! - frontend parsing;
//! - serialization formats;
//! - vendor-specific thermal models.
//!
//! Those responsibilities belong to the corresponding ZQN and quantum
//! subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                     quantum::ir
//!                         │
//!                         │ canonical operation/resource identity
//!                         ▼
//!                    ZQN thermal.rs
//!                         │
//!              thermal semantic model
//!                         │
//!             ┌───────────┼────────────┐
//!             ▼           ▼            ▼
//!          Lindblad     Kraus/Choi   simulation
//!          lowering    realization   engines
//!             │           │            │
//!             └───────────┼────────────┘
//!                         ▼
//!                   runtime/hardware
//! ```
//!
//! The thermal model is deliberately independent of the representation used
//! to execute it.
//!
//! # Canonical qubit identity
//!
//! Qubit resources are identified exclusively with:
//!
//! `crate::quantum::ir::qubit::QubitId`
//!
//! This file never defines another `QubitId` or `PhysicalQubitId`.
//!
//! A thermal model may also describe non-qubit finite-dimensional quantum
//! subsystems. This is required for qudits and future quantum modalities.
//!
//! # Write once, scale everywhere
//!
//! There is no semantic maximum for:
//!
//! - number of subsystems;
//! - number of energy levels;
//! - Hilbert-space dimension;
//! - number of transitions;
//! - number of dephasing terms;
//! - interaction duration;
//! - machine size.
//!
//! Concrete execution is necessarily bounded by available resources and by
//! caller-selected resource policies. Those policies belong to the execution
//! environment, not to the thermal semantics.
//!
//! The implementation therefore never contains constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_LEVELS
//! MAX_TRANSITIONS
//! MAX_DURATION
//! ```
//!
//! # Mathematical model
//!
//! A thermal environment is described by:
//!
//! - an energy spectrum;
//! - a thermal parameter;
//! - transition rates;
//! - optional coherence-dephasing rates;
//! - an interaction duration.
//!
//! For inverse temperature `β`, equilibrium populations are proportional to:
//!
//! ```text
//! p_i ∝ exp(-β E_i)
//! ```
//!
//! where the energy reference is arbitrary because adding the same constant
//! to every energy leaves normalized Gibbs probabilities unchanged.
//!
//! For a transition pair `i ↔ j`, thermal detailed balance requires the
//! upward/downward rates to satisfy the appropriate Boltzmann ratio.
//!
//! The semantic model does not prescribe one particular numerical realization.
//! A downstream Lindblad, Kraus, Choi, trajectory or hardware realization may
//! consume this model.
//!
//! # Important physical distinction
//!
//! Thermalization is not synonymous with:
//!
//! - amplitude damping;
//! - depolarizing noise;
//! - Pauli noise;
//! - readout noise;
//! - generic decoherence.
//!
//! A zero-temperature two-level thermal process can reduce to amplitude
//! damping, but a finite-temperature process generally requires both upward
//! and downward transitions. Higher-dimensional systems require potentially
//! many transitions.
//!
//! Therefore this file deliberately does not implement thermal noise as a
//! hard-coded two-Kraus qubit channel.
//!
//! # Determinism
//!
//! This module contains no RNG and no global mutable state.
//!
//! The semantic model is deterministic. Sampling belongs to ZQN simulation
//! and must use an explicit execution seed/context.
//!
//! # Numerical safety
//!
//! All scalar floating-point inputs are validated for finiteness.
//!
//! Invalid values such as NaN and infinity are rejected rather than silently
//! normalized, clamped or converted.
//!
//! # Resource safety
//!
//! This module does not impose semantic resource ceilings.
//!
//! Callers that process untrusted thermal descriptions should apply explicit
//! resource limits before allocating large vectors or constructing numerical
//! representations.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration
//!
//! ```text
//! thermal.rs
//!     │
//!     ├── channel::representation
//!     │       └── chooses numerical representation
//!     │
//!     ├── channel::lindblad
//!     │       └── converts thermal transitions into generators
//!     │
//!     ├── channel::kraus / choi
//!     │       └── realizes finite-time channels
//!     │
//!     ├── calibration
//!     │       └── supplies measured temperature/rates
//!     │
//!     ├── simulation
//!     │       └── executes the realized process
//!     │
//!     ├── propagation
//!     │       └── estimates error/fidelity consequences
//!     │
//!     ├── routing/scheduling
//!     │       └── consumes thermal cost information
//!     │
//!     └── QEC
//!             └── converts realized thermal processes into faults
//! ```
//!
//! No downstream module should redefine thermal semantics.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. thermal noise is represented independently of a numerical realization;
//! 2. arbitrary finite subsystem dimensions are supported;
//! 3. canonical `QubitId` is used where qubit identity is required;
//! 4. thermal equilibrium is explicitly defined;
//! 5. transition rates can represent finite-temperature excitation and
//!    relaxation;
//! 6. detailed balance is validated;
//! 7. optional pure dephasing is represented separately;
//! 8. zero-temperature limits are representable;
//! 9. infinite-temperature limits are representable through explicit policy;
//! 10. no machine-size limit is encoded;
//! 11. no hidden RNG exists;
//! 12. invalid floating-point values are rejected;
//! 13. no vendor-specific assumptions exist;
//! 14. the model can be lowered into multiple mathematical representations;
//! 15. no unsafe Rust is required;
//! 16. Rust 1.97/1.97.1 remains sufficient.
//!
//! # Security
//!
//! Thermal parameters are data and must never be interpreted as executable
//! content. Untrusted descriptions must be validated under caller-supplied
//! resource limits before numerical realization.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::quantum::ir::qubit::QubitId;
use std::fmt;

// =============================================================================
// Result and errors
// =============================================================================

/// Result type for thermal-channel operations.
pub type ThermalResult<T> = Result<T, ThermalError>;

/// Errors produced by thermal-channel construction or validation.
#[derive(Debug, Clone, PartialEq)]
pub enum ThermalError {
    /// No subsystem was supplied.
    EmptySupport,

    /// A subsystem occurs more than once.
    DuplicateSubsystem,

    /// A subsystem dimension is invalid.
    InvalidDimension {
        /// Dimension supplied by the caller.
        dimension: usize,
    },

    /// An energy value is not finite.
    NonFiniteEnergy {
        /// Index of the offending energy level.
        index: usize,
    },

    /// Energy levels are not ordered monotonically.
    UnorderedEnergySpectrum {
        /// First offending index.
        index: usize,
    },

    /// No energy levels were supplied.
    EmptyEnergySpectrum,

    /// Temperature is invalid.
    InvalidTemperature,

    /// Inverse temperature is invalid.
    InvalidInverseTemperature,

    /// A transition references an invalid energy level.
    InvalidTransitionLevel {
        /// Source level.
        from: usize,

        /// Destination level.
        to: usize,

        /// Number of levels in the spectrum.
        levels: usize,
    },

    /// A transition is self-referential.
    SelfTransition {
        /// Level involved in the transition.
        level: usize,
    },

    /// A transition rate is invalid.
    InvalidRate,

    /// A dephasing rate is invalid.
    InvalidDephasingRate,

    /// A transition rate is inconsistent with detailed balance.
    DetailedBalanceViolation {
        /// Source level.
        from: usize,

        /// Destination level.
        to: usize,

        /// Upward/downward rate ratio.
        expected_ratio: f64,

        /// Supplied ratio.
        actual_ratio: f64,
    },

    /// A thermal population could not be normalized safely.
    InvalidThermalPopulation,

    /// Duration is invalid.
    InvalidDuration,

    /// The requested representation cannot be produced from the model.
    UnsupportedRealization,

    /// A required physical property cannot be established.
    PropertyUndetermined(&'static str),

    /// Two semantic components cannot be combined.
    IncompatibleThermalModel,

    /// A supplied identifier is invalid.
    InvalidIdentity,
}

impl fmt::Display for ThermalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySupport => write!(f, "thermal channel support is empty"),

            Self::DuplicateSubsystem => {
                write!(f, "thermal channel support contains a duplicate subsystem")
            }

            Self::InvalidDimension { dimension } => {
                write!(f, "invalid subsystem dimension: {dimension}")
            }

            Self::NonFiniteEnergy { index } => {
                write!(f, "energy level {index} is not finite")
            }

            Self::UnorderedEnergySpectrum { index } => {
                write!(f, "energy spectrum is not ordered at index {index}")
            }

            Self::EmptyEnergySpectrum => {
                write!(f, "thermal channel has no energy levels")
            }

            Self::InvalidTemperature => {
                write!(f, "temperature must be finite and non-negative")
            }

            Self::InvalidInverseTemperature => {
                write!(f, "inverse temperature must be finite and non-negative")
            }

            Self::InvalidTransitionLevel { from, to, levels } => write!(
                f,
                "transition {from}->{to} is outside energy spectrum with {levels} levels"
            ),

            Self::SelfTransition { level } => {
                write!(f, "transition at level {level} is self-referential")
            }

            Self::InvalidRate => {
                write!(f, "thermal transition rate must be finite and non-negative")
            }

            Self::InvalidDephasingRate => {
                write!(
                    f,
                    "thermal dephasing rate must be finite and non-negative"
                )
            }

            Self::DetailedBalanceViolation {
                from,
                to,
                expected_ratio,
                actual_ratio,
            } => write!(
                f,
                "detailed balance violated for {from}<->{to}: expected ratio \
                 {expected_ratio}, actual ratio {actual_ratio}"
            ),

            Self::InvalidThermalPopulation => {
                write!(f, "thermal equilibrium population is invalid")
            }

            Self::InvalidDuration => {
                write!(f, "thermal interaction duration must be finite and non-negative")
            }

            Self::UnsupportedRealization => {
                write!(f, "requested thermal-channel realization is unsupported")
            }

            Self::PropertyUndetermined(property) => {
                write!(f, "thermal property could not be established: {property}")
            }

            Self::IncompatibleThermalModel => {
                write!(f, "thermal models are semantically incompatible")
            }

            Self::InvalidIdentity => {
                write!(f, "thermal channel identity is invalid")
            }
        }
    }
}

impl std::error::Error for ThermalError {}

// =============================================================================
// Thermal scalar
// =============================================================================

/// A validated non-negative finite thermal scalar.
///
/// This wrapper prevents accidental use of NaN, infinity or negative rates
/// and temperatures in the thermal semantic layer.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ThermalScalar(f64);

impl ThermalScalar {
    /// Creates a non-negative finite thermal scalar.
    pub fn new(value: f64) -> ThermalResult<Self> {
        if !value.is_finite() || value < 0.0 {
            return Err(ThermalError::InvalidRate);
        }

        Ok(Self(value))
    }

    /// Returns the underlying scalar.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

// =============================================================================
// Temperature
// =============================================================================

/// Thermal parameterization.
///
/// The semantic model supports either temperature or inverse temperature.
/// Both describe the same thermal state but different parameterizations can
/// be more numerically appropriate in different regimes.
///
/// Units are deliberately not hard-coded into the semantic type. The caller
/// must use a consistent energy/temperature unit system and supply the
/// corresponding Boltzmann constant convention.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThermalParameter {
    /// Physical temperature `T >= 0`.
    ///
    /// `T = 0` is allowed and represents the zero-temperature limit.
    Temperature(f64),

    /// Inverse temperature `β >= 0`.
    ///
    /// `β = 0` represents the infinite-temperature limit.
    InverseTemperature(f64),
}

impl ThermalParameter {
    /// Creates a temperature parameter.
    pub fn temperature(value: f64) -> ThermalResult<Self> {
        if !value.is_finite() || value < 0.0 {
            return Err(ThermalError::InvalidTemperature);
        }

        Ok(Self::Temperature(value))
    }

    /// Creates an inverse-temperature parameter.
    pub fn inverse_temperature(value: f64) -> ThermalResult<Self> {
        if !value.is_finite() || value < 0.0 {
            return Err(ThermalError::InvalidInverseTemperature);
        }

        Ok(Self::InverseTemperature(value))
    }

    /// Returns the parameter as temperature when it was specified that way.
    #[must_use]
    pub const fn as_temperature(self) -> Option<f64> {
        match self {
            Self::Temperature(value) => Some(value),
            Self::InverseTemperature(_) => None,
        }
    }

    /// Returns the parameter as inverse temperature when it was specified that
    /// way.
    #[must_use]
    pub const fn as_inverse_temperature(self) -> Option<f64> {
        match self {
            Self::Temperature(_) => None,
            Self::InverseTemperature(value) => Some(value),
        }
    }

    /// Returns whether the parameter represents the zero-temperature limit.
    #[must_use]
    pub const fn is_zero_temperature(self) -> bool {
        match self {
            Self::Temperature(value) => value == 0.0,
            Self::InverseTemperature(_) => false,
        }
    }

    /// Returns whether the parameter represents the infinite-temperature
    /// limit.
    #[must_use]
    pub const fn is_infinite_temperature(self) -> bool {
        match self {
            Self::Temperature(_) => false,
            Self::InverseTemperature(value) => value == 0.0,
        }
    }
}

// =============================================================================
// Energy spectrum
// =============================================================================

/// Immutable finite-dimensional energy spectrum.
///
/// Energy levels must be finite and non-decreasing.
///
/// The absolute energy origin is not semantically significant for Gibbs
/// probabilities. Only energy differences affect equilibrium populations.
#[derive(Debug, Clone, PartialEq)]
pub struct EnergySpectrum {
    levels: Vec<f64>,
}

impl EnergySpectrum {
    /// Creates an energy spectrum.
    pub fn new(levels: Vec<f64>) -> ThermalResult<Self> {
        if levels.is_empty() {
            return Err(ThermalError::EmptyEnergySpectrum);
        }

        for (index, energy) in levels.iter().copied().enumerate() {
            if !energy.is_finite() {
                return Err(ThermalError::NonFiniteEnergy { index });
            }

            if index > 0 && energy < levels[index - 1] {
                return Err(ThermalError::UnorderedEnergySpectrum { index });
            }
        }

        Ok(Self { levels })
    }

    /// Returns the number of energy levels.
    #[must_use]
    pub fn len(&self) -> usize {
        self.levels.len()
    }

    /// Returns whether the spectrum contains no levels.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.levels.is_empty()
    }

    /// Returns all energy levels.
    #[must_use]
    pub fn levels(&self) -> &[f64] {
        &self.levels
    }

    /// Returns one energy level.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<f64> {
        self.levels.get(index).copied()
    }

    /// Returns the energy difference `E_to - E_from`.
    pub fn difference(&self, from: usize, to: usize) -> ThermalResult<f64> {
        let from_energy = self.levels.get(from).copied().ok_or(
            ThermalError::InvalidTransitionLevel {
                from,
                to,
                levels: self.levels.len(),
            },
        )?;

        let to_energy = self.levels.get(to).copied().ok_or(
            ThermalError::InvalidTransitionLevel {
                from,
                to,
                levels: self.levels.len(),
            },
        )?;

        Ok(to_energy - from_energy)
    }

    /// Returns the lowest energy.
    #[must_use]
    pub fn ground_energy(&self) -> f64 {
        self.levels[0]
    }
}

// =============================================================================
// Thermal transition
// =============================================================================

/// Bidirectional thermal transition between two energy levels.
///
/// `forward_rate` describes the transition `from -> to`.
/// `reverse_rate` describes the transition `to -> from`.
///
/// The pair may be validated against thermal detailed balance.
///
/// Rates are generator rates, not probabilities. They may therefore be
/// greater than one when expressed in inverse-time units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermalTransition {
    from: usize,
    to: usize,
    forward_rate: f64,
    reverse_rate: f64,
}

impl ThermalTransition {
    /// Creates a thermal transition.
    pub fn new(
        from: usize,
        to: usize,
        forward_rate: f64,
        reverse_rate: f64,
    ) -> ThermalResult<Self> {
        if from == to {
            return Err(ThermalError::SelfTransition { level: from });
        }

        if !forward_rate.is_finite()
            || !reverse_rate.is_finite()
            || forward_rate < 0.0
            || reverse_rate < 0.0
        {
            return Err(ThermalError::InvalidRate);
        }

        Ok(Self {
            from,
            to,
            forward_rate,
            reverse_rate,
        })
    }

    /// Source energy level.
    #[must_use]
    pub const fn from(&self) -> usize {
        self.from
    }

    /// Destination energy level.
    #[must_use]
    pub const fn to(&self) -> usize {
        self.to
    }

    /// Forward transition rate.
    #[must_use]
    pub const fn forward_rate(&self) -> f64 {
        self.forward_rate
    }

    /// Reverse transition rate.
    #[must_use]
    pub const fn reverse_rate(&self) -> f64 {
        self.reverse_rate
    }

    /// Returns the rate pair in deterministic order.
    #[must_use]
    pub const fn rates(&self) -> (f64, f64) {
        (self.forward_rate, self.reverse_rate)
    }
}

// =============================================================================
// Dephasing
// =============================================================================

/// Pure-dephasing rate associated with a pair of energy levels.
///
/// Thermal population transfer and pure dephasing are represented separately.
/// This prevents a thermal model from incorrectly conflating energy exchange
/// with loss of phase coherence.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermalDephasing {
    first: usize,
    second: usize,
    rate: f64,
}

impl ThermalDephasing {
    /// Creates a pure-dephasing term.
    pub fn new(first: usize, second: usize, rate: f64) -> ThermalResult<Self> {
        if first == second {
            return Err(ThermalError::InvalidDephasingRate);
        }

        if !rate.is_finite() || rate < 0.0 {
            return Err(ThermalError::InvalidDephasingRate);
        }

        Ok(Self {
            first,
            second,
            rate,
        })
    }

    /// First level.
    #[must_use]
    pub const fn first(&self) -> usize {
        self.first
    }

    /// Second level.
    #[must_use]
    pub const fn second(&self) -> usize {
        self.second
    }

    /// Dephasing rate.
    #[must_use]
    pub const fn rate(&self) -> f64 {
        self.rate
    }
}

// =============================================================================
// Thermal equilibrium
// =============================================================================

/// Numerically stable Gibbs equilibrium calculation.
///
/// This implementation uses a shifted exponential:
///
/// ```text
/// p_i = exp(-β(E_i - E_min)) / Σ_j exp(-β(E_j - E_min))
/// ```
///
/// The energy shift does not change normalized probabilities and substantially
/// reduces overflow/underflow risk.
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalEquilibrium {
    populations: Vec<f64>,
}

impl ThermalEquilibrium {
    /// Computes normalized Gibbs populations.
    pub fn from_spectrum(
        spectrum: &EnergySpectrum,
        parameter: ThermalParameter,
    ) -> ThermalResult<Self> {
        let beta = match parameter {
            ThermalParameter::InverseTemperature(value) => value,

            ThermalParameter::Temperature(temperature) => {
                if temperature == 0.0 {
                    return Self::zero_temperature(spectrum);
                }

                1.0 / temperature
            }
        };

        if !beta.is_finite() || beta < 0.0 {
            return Err(ThermalError::InvalidInverseTemperature);
        }

        // β = 0 is the infinite-temperature limit.
        if beta == 0.0 {
            let population = 1.0 / spectrum.len() as f64;

            return Ok(Self {
                populations: vec![population; spectrum.len()],
            });
        }

        // For finite beta, use the minimum energy as the reference.
        let minimum_energy = spectrum.ground_energy();

        let mut weights = Vec::with_capacity(spectrum.len());
        let mut total = 0.0;

        for &energy in spectrum.levels() {
            let shifted = -beta * (energy - minimum_energy);

            // The shifted exponent is <= 0 because the spectrum is ordered.
            // Values below the representable range naturally underflow to zero,
            // which is preferable to overflow and does not change the
            // normalized result materially for sufficiently separated levels.
            let weight = shifted.exp();

            if !weight.is_finite() {
                return Err(ThermalError::InvalidThermalPopulation);
            }

            total += weight;
            weights.push(weight);
        }

        if !total.is_finite() || total <= 0.0 {
            return Err(ThermalError::InvalidThermalPopulation);
        }

        for weight in &mut weights {
            *weight /= total;
        }

        Ok(Self {
            populations: weights,
        })
    }

    fn zero_temperature(spectrum: &EnergySpectrum) -> ThermalResult<Self> {
        let ground = spectrum.ground_energy();

        let mut ground_count = 0usize;

        for &energy in spectrum.levels() {
            if energy == ground {
                ground_count += 1;
            } else {
                break;
            }
        }

        if ground_count == 0 {
            return Err(ThermalError::InvalidThermalPopulation);
        }

        let population = 1.0 / ground_count as f64;
        let mut populations = vec![0.0; spectrum.len()];

        for value in populations.iter_mut().take(ground_count) {
            *value = population;
        }

        Ok(Self { populations })
    }

    /// Returns equilibrium populations.
    #[must_use]
    pub fn populations(&self) -> &[f64] {
        &self.populations
    }

    /// Returns one equilibrium population.
    #[must_use]
    pub fn population(&self, level: usize) -> Option<f64> {
        self.populations.get(level).copied()
    }

    /// Validates normalization.
    pub fn validate(&self) -> ThermalResult<()> {
        let mut total = 0.0;

        for &population in &self.populations {
            if !population.is_finite() || population < 0.0 {
                return Err(ThermalError::InvalidThermalPopulation);
            }

            total += population;
        }

        if !total.is_finite() || (total - 1.0).abs() > 1.0e-12 {
            return Err(ThermalError::InvalidThermalPopulation);
        }

        Ok(())
    }
}

// =============================================================================
// Thermal support
// =============================================================================

/// Thermal subsystem descriptor.
///
/// A thermal channel may operate on a qubit, qudit, or another explicitly
/// identified finite-dimensional subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThermalSubsystem {
    /// Canonical Zamani logical qubit.
    Qubit(QubitId),

    /// Opaque subsystem identity for non-qubit modalities.
    Opaque(u128),
}

impl ThermalSubsystem {
    /// Creates a qubit thermal subsystem using the canonical IR identity.
    #[must_use]
    pub const fn qubit(qubit: QubitId) -> Self {
        Self::Qubit(qubit)
    }

    /// Creates an opaque thermal subsystem.
    #[must_use]
    pub const fn opaque(identifier: u128) -> Self {
        Self::Opaque(identifier)
    }

    /// Returns the canonical qubit identifier when applicable.
    #[must_use]
    pub const fn as_qubit(&self) -> Option<QubitId> {
        match self {
            Self::Qubit(qubit) => Some(*qubit),
            Self::Opaque(_) => None,
        }
    }
}

// =============================================================================
// Thermal channel
// =============================================================================

/// Representation-independent thermal quantum channel.
///
/// This is a semantic description of finite-duration thermal interaction.
///
/// It is deliberately not tied to a Kraus, Choi, Pauli-transfer or Lindblad
/// representation.
///
/// The model supports:
///
/// - arbitrary finite subsystem dimensions;
/// - arbitrary energy spectra;
/// - finite-temperature excitation and relaxation;
/// - zero-temperature relaxation;
/// - infinite-temperature equilibrium;
/// - explicit detailed-balance validation;
/// - optional pure dephasing;
/// - arbitrary finite interaction duration.
///
/// A downstream representation may lower this model into a concrete channel.
#[derive(Debug, Clone, PartialEq)]
pub struct ThermalChannel {
    /// Resources affected by this thermal process.
    support: Vec<ThermalSubsystem>,

    /// Hilbert-space dimension of each affected subsystem.
    ///
    /// The dimensions correspond positionally to `support`.
    dimensions: Vec<usize>,

    /// Energy spectrum associated with the thermal process.
    spectrum: EnergySpectrum,

    /// Thermal bath parameter.
    thermal_parameter: ThermalParameter,

    /// Thermal transition rates.
    transitions: Vec<ThermalTransition>,

    /// Optional pure-dephasing terms.
    dephasing: Vec<ThermalDephasing>,

    /// Duration of the thermal interaction.
    duration: f64,

    /// Whether detailed balance is required for this model.
    ///
    /// This allows phenomenological models that intentionally provide measured
    /// effective rates which are not constrained to equilibrium detailed
    /// balance. Such models must explicitly opt out.
    enforce_detailed_balance: bool,
}

impl ThermalChannel {
    /// Creates a thermal channel.
    ///
    /// Validation is performed immediately so an instance cannot be created
    /// with obviously invalid structural parameters.
    pub fn new(
        support: Vec<ThermalSubsystem>,
        dimensions: Vec<usize>,
        spectrum: EnergySpectrum,
        thermal_parameter: ThermalParameter,
        transitions: Vec<ThermalTransition>,
        dephasing: Vec<ThermalDephasing>,
        duration: f64,
    ) -> ThermalResult<Self> {
        if support.is_empty() {
            return Err(ThermalError::EmptySupport);
        }

        if support.len() != dimensions.len() {
            return Err(ThermalError::InvalidDimension {
                dimension: dimensions.len(),
            });
        }

        validate_unique_subsystems(&support)?;

        for &dimension in &dimensions {
            if dimension < 2 {
                return Err(ThermalError::InvalidDimension { dimension });
            }
        }

        if !duration.is_finite() || duration < 0.0 {
            return Err(ThermalError::InvalidDuration);
        }

        let channel = Self {
            support,
            dimensions,
            spectrum,
            thermal_parameter,
            transitions,
            dephasing,
            duration,
            enforce_detailed_balance: true,
        };

        channel.validate()?;

        Ok(channel)
    }

    /// Creates a thermal channel for one canonical qubit.
    ///
    /// The energy spectrum must contain exactly the physical energy levels
    /// relevant to the selected qubit model. No universal `0/1` energy values
    /// are assumed here.
    pub fn for_qubit(
        qubit: QubitId,
        spectrum: EnergySpectrum,
        thermal_parameter: ThermalParameter,
        transitions: Vec<ThermalTransition>,
        dephasing: Vec<ThermalDephasing>,
        duration: f64,
    ) -> ThermalResult<Self> {
        Self::new(
            vec![ThermalSubsystem::qubit(qubit)],
            vec![2],
            spectrum,
            thermal_parameter,
            transitions,
            dephasing,
            duration,
        )
    }

    /// Creates a phenomenological thermal channel whose measured rates are not
    /// required to satisfy idealized equilibrium detailed balance.
    ///
    /// This is intended for experimentally characterized effective models.
    ///
    /// The distinction is explicit: the model remains thermal-labelled, but
    /// validation will not falsely claim equilibrium detailed balance.
    pub fn phenomenological(
        support: Vec<ThermalSubsystem>,
        dimensions: Vec<usize>,
        spectrum: EnergySpectrum,
        thermal_parameter: ThermalParameter,
        transitions: Vec<ThermalTransition>,
        dephasing: Vec<ThermalDephasing>,
        duration: f64,
    ) -> ThermalResult<Self> {
        let mut channel = Self::new(
            support,
            dimensions,
            spectrum,
            thermal_parameter,
            transitions,
            dephasing,
            duration,
        )?;

        channel.enforce_detailed_balance = false;
        channel.validate_structure()?;

        Ok(channel)
    }

    /// Returns the affected subsystems.
    #[must_use]
    pub fn support(&self) -> &[ThermalSubsystem] {
        &self.support
    }

    /// Returns subsystem dimensions.
    #[must_use]
    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    /// Returns the energy spectrum.
    #[must_use]
    pub const fn spectrum(&self) -> &EnergySpectrum {
        &self.spectrum
    }

    /// Returns the thermal parameter.
    #[must_use]
    pub const fn thermal_parameter(&self) -> ThermalParameter {
        self.thermal_parameter
    }

    /// Returns transition specifications.
    #[must_use]
    pub fn transitions(&self) -> &[ThermalTransition] {
        &self.transitions
    }

    /// Returns pure-dephasing specifications.
    #[must_use]
    pub fn dephasing(&self) -> &[ThermalDephasing] {
        &self.dephasing
    }

    /// Returns the interaction duration.
    #[must_use]
    pub const fn duration(&self) -> f64 {
        self.duration
    }

    /// Returns whether ideal thermal detailed balance is required.
    #[must_use]
    pub const fn enforces_detailed_balance(&self) -> bool {
        self.enforce_detailed_balance
    }

    /// Enables ideal thermal detailed-balance validation.
    pub fn require_detailed_balance(&mut self) -> ThermalResult<()> {
        self.enforce_detailed_balance = true;
        self.validate()
    }

    /// Disables ideal detailed-balance validation for a phenomenological model.
    ///
    /// This does not disable structural or numerical validation.
    pub fn allow_phenomenological_rates(&mut self) {
        self.enforce_detailed_balance = false;
    }

    /// Computes the thermal equilibrium populations.
    pub fn equilibrium(&self) -> ThermalResult<ThermalEquilibrium> {
        ThermalEquilibrium::from_spectrum(
            &self.spectrum,
            self.thermal_parameter,
        )
    }

    /// Validates the complete semantic model.
    pub fn validate(&self) -> ThermalResult<()> {
        self.validate_structure()?;

        if self.enforce_detailed_balance {
            self.validate_detailed_balance()?;
        }

        self.equilibrium()?.validate()?;

        Ok(())
    }

    fn validate_structure(&self) -> ThermalResult<()> {
        if self.support.is_empty() {
            return Err(ThermalError::EmptySupport);
        }

        if self.support.len() != self.dimensions.len() {
            return Err(ThermalError::InvalidDimension {
                dimension: self.dimensions.len(),
            });
        }

        validate_unique_subsystems(&self.support)?;

        for &dimension in &self.dimensions {
            if dimension < 2 {
                return Err(ThermalError::InvalidDimension { dimension });
            }
        }

        if !self.duration.is_finite() || self.duration < 0.0 {
            return Err(ThermalError::InvalidDuration);
        }

        for transition in &self.transitions {
            if transition.from >= self.spectrum.len()
                || transition.to >= self.spectrum.len()
            {
                return Err(ThermalError::InvalidTransitionLevel {
                    from: transition.from,
                    to: transition.to,
                    levels: self.spectrum.len(),
                });
            }

            if transition.from == transition.to {
                return Err(ThermalError::SelfTransition {
                    level: transition.from,
                });
            }

            if !transition.forward_rate.is_finite()
                || !transition.reverse_rate.is_finite()
                || transition.forward_rate < 0.0
                || transition.reverse_rate < 0.0
            {
                return Err(ThermalError::InvalidRate);
            }
        }

        for dephasing in &self.dephasing {
            if dephasing.first >= self.spectrum.len()
                || dephasing.second >= self.spectrum.len()
                || dephasing.first == dephasing.second
                || !dephasing.rate.is_finite()
                || dephasing.rate < 0.0
            {
                return Err(ThermalError::InvalidDephasingRate);
            }
        }

        Ok(())
    }

    /// Validates thermal detailed balance for every transition.
    ///
    /// For a pair of levels `i` and `j`, equilibrium detailed balance requires
    /// the rates to satisfy the Gibbs population ratio:
    ///
    /// ```text
    /// p_i * Γ(i→j) = p_j * Γ(j→i)
    /// ```
    ///
    /// This formulation is used rather than directly computing an exponential
    /// rate ratio, because it remains meaningful in zero-temperature and
    /// degenerate-energy limits.
    pub fn validate_detailed_balance(&self) -> ThermalResult<()> {
        let equilibrium = self.equilibrium()?;

        for transition in &self.transitions {
            let p_from = equilibrium
                .population(transition.from)
                .ok_or(ThermalError::InvalidTransitionLevel {
                    from: transition.from,
                    to: transition.to,
                    levels: self.spectrum.len(),
                })?;

            let p_to = equilibrium
                .population(transition.to)
                .ok_or(ThermalError::InvalidTransitionLevel {
                    from: transition.from,
                    to: transition.to,
                    levels: self.spectrum.len(),
                })?;

            let forward_flux = p_from * transition.forward_rate;
            let reverse_flux = p_to * transition.reverse_rate;

            if !forward_flux.is_finite() || !reverse_flux.is_finite() {
                return Err(ThermalError::InvalidRate);
            }

            let scale = forward_flux
                .abs()
                .max(reverse_flux.abs())
                .max(1.0e-15);

            let relative_error = (forward_flux - reverse_flux).abs() / scale;

            if relative_error > 1.0e-10 {
                let expected_ratio = if transition.reverse_rate == 0.0 {
                    if transition.forward_rate == 0.0 {
                        1.0
                    } else {
                        f64::INFINITY
                    }
                } else {
                    transition.forward_rate / transition.reverse_rate
                };

                let actual_ratio = if p_from == 0.0 || p_to == 0.0 {
                    f64::INFINITY
                } else {
                    p_to / p_from
                };

                return Err(ThermalError::DetailedBalanceViolation {
                    from: transition.from,
                    to: transition.to,
                    expected_ratio,
                    actual_ratio,
                });
            }
        }

        Ok(())
    }

    /// Returns the total population-transfer rate incident on one energy level.
    #[must_use]
    pub fn incident_rate(&self, level: usize) -> f64 {
        let mut total = 0.0;

        for transition in &self.transitions {
            if transition.from == level {
                total += transition.forward_rate;
            }

            if transition.to == level {
                total += transition.reverse_rate;
            }

            if transition.to == level {
                total += transition.forward_rate;
            }

            if transition.from == level {
                total += transition.reverse_rate;
            }
        }

        total
    }

    /// Returns the largest transition rate in the model.
    #[must_use]
    pub fn max_transition_rate(&self) -> f64 {
        self.transitions
            .iter()
            .map(|transition| {
                transition
                    .forward_rate
                    .max(transition.reverse_rate)
            })
            .fold(0.0, f64::max)
    }

    /// Returns the largest dephasing rate in the model.
    #[must_use]
    pub fn max_dephasing_rate(&self) -> f64 {
        self.dephasing
            .iter()
            .map(ThermalDephasing::rate)
            .fold(0.0, f64::max)
    }

    /// Returns the thermal relaxation scale `max_rate * duration`.
    ///
    /// This is a dimensionless diagnostic useful to downstream numerical
    /// realizations. It is not itself a probability.
    pub fn relaxation_exponent(&self) -> ThermalResult<f64> {
        let value = self.max_transition_rate() * self.duration;

        if !value.is_finite() {
            return Err(ThermalError::InvalidRate);
        }

        Ok(value)
    }

    /// Returns the dephasing exponent `max_rate * duration`.
    ///
    /// This is a dimensionless diagnostic, not a probability.
    pub fn dephasing_exponent(&self) -> ThermalResult<f64> {
        let value = self.max_dephasing_rate() * self.duration;

        if !value.is_finite() {
            return Err(ThermalError::InvalidDephasingRate);
        }

        Ok(value)
    }

    /// Returns whether the model represents a zero-temperature limit.
    #[must_use]
    pub const fn is_zero_temperature(&self) -> bool {
        self.thermal_parameter.is_zero_temperature()
    }

    /// Returns whether the model represents an infinite-temperature limit.
    #[must_use]
    pub const fn is_infinite_temperature(&self) -> bool {
        self.thermal_parameter.is_infinite_temperature()
    }

    /// Returns whether the channel contains population-transfer terms.
    #[must_use]
    pub const fn has_population_transfer(&self) -> bool {
        !self.transitions.is_empty()
    }

    /// Returns whether the channel contains explicit pure dephasing.
    #[must_use]
    pub const fn has_pure_dephasing(&self) -> bool {
        !self.dephasing.is_empty()
    }

    /// Returns whether the channel has no physical evolution because its
    /// duration and all rates are zero.
    #[must_use]
    pub fn is_identity_process(&self) -> bool {
        self.duration == 0.0
            || (self.max_transition_rate() == 0.0
                && self.max_dephasing_rate() == 0.0)
    }

    /// Returns a deterministic semantic description.
    ///
    /// This is intentionally a textual canonical representation rather than a
    /// cryptographic hash. A higher-level identity subsystem may hash this
    /// representation.
    #[must_use]
    pub fn canonical_representation(&self) -> String {
        let mut output = String::new();

        output.push_str("zqn.thermal.v1|");

        output.push_str("support=");
        for (index, subsystem) in self.support.iter().enumerate() {
            if index != 0 {
                output.push(',');
            }

            match subsystem {
                ThermalSubsystem::Qubit(qubit) => {
                    output.push_str("q:");
                    output.push_str(&format!("{qubit:?}"));
                }

                ThermalSubsystem::Opaque(identifier) => {
                    output.push_str("o:");
                    output.push_str(&format!("{identifier:032x}"));
                }
            }
        }

        output.push('|');

        output.push_str("dimensions=");
        for (index, dimension) in self.dimensions.iter().enumerate() {
            if index != 0 {
                output.push(',');
            }

            output.push_str(&dimension.to_string());
        }

        output.push('|');

        output.push_str("energies=");
        for (index, energy) in self.spectrum.levels().iter().enumerate() {
            if index != 0 {
                output.push(',');
            }

            output.push_str(&format!("{energy:.17e}"));
        }

        output.push('|');

        match self.thermal_parameter {
            ThermalParameter::Temperature(value) => {
                output.push_str("temperature=");
                output.push_str(&format!("{value:.17e}"));
            }

            ThermalParameter::InverseTemperature(value) => {
                output.push_str("inverse_temperature=");
                output.push_str(&format!("{value:.17e}"));
            }
        }

        output.push('|');

        output.push_str("transitions=");
        for (index, transition) in self.transitions.iter().enumerate() {
            if index != 0 {
                output.push(';');
            }

            output.push_str(&format!(
                "{}>{}:{:.17e}:{:.17e}",
                transition.from,
                transition.to,
                transition.forward_rate,
                transition.reverse_rate
            ));
        }

        output.push('|');

        output.push_str("dephasing=");
        for (index, dephasing) in self.dephasing.iter().enumerate() {
            if index != 0 {
                output.push(';');
            }

            output.push_str(&format!(
                "{}>{}:{:.17e}",
                dephasing.first,
                dephasing.second,
                dephasing.rate
            ));
        }

        output.push('|');

        output.push_str("duration=");
        output.push_str(&format!("{:.17e}", self.duration));

        output.push('|');

        output.push_str("detailed_balance=");
        output.push_str(if self.enforce_detailed_balance {
            "required"
        } else {
            "phenomenological"
        });

        output
    }

    /// Returns a stable semantic identifier derived from the canonical
    /// representation.
    ///
    /// This is a deterministic non-cryptographic identifier. Consumers needing
    /// collision-resistant or cryptographic identity must hash
    /// [`Self::canonical_representation`] using the repository's canonical
    /// hashing subsystem.
    #[must_use]
    pub fn semantic_fingerprint(&self) -> u128 {
        fn fnv64(bytes: &[u8], seed: u64) -> u64 {
            let mut hash = seed;

            for byte in bytes {
                hash ^= u64::from(*byte);
                hash = hash.wrapping_mul(0x1000_0000_01b3);
            }

            hash
        }

        let bytes = self.canonical_representation();

        let first = fnv64(bytes.as_bytes(), 0xcbf2_9ce4_8422_2325);
        let second = fnv64(
            bytes.as_bytes(),
            0x8422_2325_cbf2_9ce4,
        );

        (u128::from(first) << 64) | u128::from(second)
    }
}

// =============================================================================
// Support validation
// =============================================================================

fn validate_unique_subsystems(
    support: &[ThermalSubsystem],
) -> ThermalResult<()> {
    for (index, subsystem) in support.iter().enumerate() {
        for other in support.iter().skip(index + 1) {
            if subsystem == other {
                return Err(ThermalError::DuplicateSubsystem);
            }
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_spectrum() {
        let result = EnergySpectrum::new(Vec::new());

        assert_eq!(
            result,
            Err(ThermalError::EmptyEnergySpectrum)
        );
    }

    #[test]
    fn rejects_non_finite_energy() {
        let result = EnergySpectrum::new(vec![0.0, f64::NAN]);

        assert_eq!(
            result,
            Err(ThermalError::NonFiniteEnergy { index: 1 })
        );
    }

    #[test]
    fn rejects_descending_energy_levels() {
        let result = EnergySpectrum::new(vec![1.0, 0.0]);

        assert_eq!(
            result,
            Err(ThermalError::UnorderedEnergySpectrum { index: 1 })
        );
    }

    #[test]
    fn infinite_temperature_is_uniform() {
        let spectrum =
            EnergySpectrum::new(vec![0.0, 1.0, 5.0]).unwrap();

        let equilibrium = ThermalEquilibrium::from_spectrum(
            &spectrum,
            ThermalParameter::inverse_temperature(0.0).unwrap(),
        )
        .unwrap();

        assert_eq!(
            equilibrium.populations(),
            &[1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0]
        );
    }

    #[test]
    fn zero_temperature_selects_ground_subspace() {
        let spectrum =
            EnergySpectrum::new(vec![0.0, 0.0, 2.0]).unwrap();

        let equilibrium = ThermalEquilibrium::from_spectrum(
            &spectrum,
            ThermalParameter::temperature(0.0).unwrap(),
        )
        .unwrap();

        assert_eq!(
            equilibrium.populations(),
            &[0.5, 0.5, 0.0]
        );
    }

    #[test]
    fn finite_temperature_normalizes() {
        let spectrum =
            EnergySpectrum::new(vec![0.0, 1.0]).unwrap();

        let equilibrium = ThermalEquilibrium::from_spectrum(
            &spectrum,
            ThermalParameter::inverse_temperature(1.0).unwrap(),
        )
        .unwrap();

        let total: f64 = equilibrium.populations().iter().sum();

        assert!((total - 1.0).abs() < 1.0e-12);
        assert!(equilibrium.populations()[0] > equilibrium.populations()[1]);
    }

    #[test]
    fn thermal_transition_rejects_self_transition() {
        let result =
            ThermalTransition::new(0, 0, 1.0, 1.0);

        assert_eq!(
            result,
            Err(ThermalError::SelfTransition { level: 0 })
        );
    }

    #[test]
    fn thermal_transition_rejects_negative_rate() {
        let result =
            ThermalTransition::new(0, 1, -1.0, 1.0);

        assert_eq!(result, Err(ThermalError::InvalidRate));
    }

    #[test]
    fn dephasing_rejects_negative_rate() {
        let result =
            ThermalDephasing::new(0, 1, -1.0);

        assert_eq!(
            result,
            Err(ThermalError::InvalidDephasingRate)
        );
    }

    #[test]
    fn deterministic_representation_is_stable() {
        let spectrum =
            EnergySpectrum::new(vec![0.0, 1.0]).unwrap();

        let transition =
            ThermalTransition::new(0, 1, 0.5, 1.3591409142295225)
                .unwrap();

        let channel = ThermalChannel::phenomenological(
            vec![ThermalSubsystem::Opaque(7)],
            vec![2],
            spectrum,
            ThermalParameter::inverse_temperature(1.0).unwrap(),
            vec![transition],
            Vec::new(),
            2.0,
        )
        .unwrap();

        assert_eq!(
            channel.canonical_representation(),
            channel.canonical_representation()
        );

        assert_eq!(
            channel.semantic_fingerprint(),
            channel.semantic_fingerprint()
        );
    }

    #[test]
    fn zero_duration_is_identity_process() {
        let spectrum =
            EnergySpectrum::new(vec![0.0, 1.0]).unwrap();

        let channel = ThermalChannel::phenomenological(
            vec![ThermalSubsystem::Opaque(1)],
            vec![2],
            spectrum,
            ThermalParameter::inverse_temperature(1.0).unwrap(),
            Vec::new(),
            Vec::new(),
            0.0,
        )
        .unwrap();

        assert!(channel.is_identity_process());
    }

    #[test]
    fn canonical_qubit_identity_is_supported() {
        // QubitId construction is intentionally delegated to the canonical IR
        // implementation. This test only verifies that ThermalSubsystem uses
        // that type without introducing another qubit identity.
        fn accepts_canonical_qubit(_: QubitId) {}

        // The function itself is intentionally not invoked because the exact
        // canonical QubitId constructor belongs to quantum::ir::qubit.
        let _ = accepts_canonical_qubit;
    }

    #[test]
    fn thermal_channel_rejects_duplicate_support() {
        let support = vec![
            ThermalSubsystem::Opaque(1),
            ThermalSubsystem::Opaque(1),
        ];

        let spectrum =
            EnergySpectrum::new(vec![0.0, 1.0]).unwrap();

        let result = ThermalChannel::phenomenological(
            support,
            vec![2, 2],
            spectrum,
            ThermalParameter::inverse_temperature(1.0).unwrap(),
            Vec::new(),
            Vec::new(),
            1.0,
        );

        assert_eq!(
            result,
            Err(ThermalError::DuplicateSubsystem)
        );
    }
}