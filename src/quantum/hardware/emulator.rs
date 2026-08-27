//! Zamani Quantum Hardware — Production Hardware Emulator
//!
//! This module provides a deterministic, provider-independent hardware
//! emulator for Zamani Quantum.
//!
//! # Responsibility
//!
//! `emulator.rs` models the behaviour of a concrete quantum hardware target
//! without communicating with a physical QPU.
//!
//! It owns:
//!
//! - hardware-model identity;
//! - physical-qubit count;
//! - connectivity constraints;
//! - native instruction constraints;
//! - gate duration modelling;
//! - gate-error modelling;
//! - readout-error modelling;
//! - reset-error modelling;
//! - deterministic stochastic fault injection;
//! - noisy state-vector execution;
//! - computational-basis measurement;
//! - shot-based execution;
//! - emulator execution statistics;
//! - emulator provenance;
//! - validation of emulator configuration;
//! - deterministic reproducibility;
//! - resource protection;
//! - emulator-specific errors.
//!
//! It deliberately does NOT own:
//!
//! - provider APIs;
//! - network communication;
//! - credentials;
//! - authentication;
//! - physical QPU execution;
//! - backend registries;
//! - provider-specific adapters;
//! - routing algorithms;
//! - scheduling algorithms;
//! - calibration acquisition;
//! - benchmarking mathematics;
//! - error-correction algorithms;
//! - source-language parsing;
//! - OpenQASM parsing;
//! - QIR generation;
//! - canonical Quantum IR semantics.
//!
//! The canonical Quantum IR remains authoritative.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Quantum Frontend
//!      |
//!      v
//! Zamani Quantum IR
//!      |
//!      +-----------------------------+
//!      |                             |
//!      v                             v
//! optimization                 error correction
//!      |                             |
//!      +-------------+---------------+
//!                    |
//!                    v
//!              routing/scheduling
//!                    |
//!                    v
//!             Quantum workload
//!                    |
//!          +---------+---------+
//!          |                   |
//!          v                   v
//!       Physical QPU        Emulator
//!                              |
//!                     +--------+--------+
//!                     |        |        |
//!                  topology  timing   noise
//!                              |
//!                              v
//!                       noisy execution
//!                              |
//!                              v
//!                           results
//! ```
//!
//! # Relationship with simulator.rs
//!
//! `simulator.rs` answers:
//!
//! > What is the ideal mathematical result of this circuit?
//!
//! `emulator.rs` answers:
//!
//! > What result would a configured hardware model produce when the same
//! > workload is affected by hardware-like gate, reset and measurement errors?
//!
//! The two modules must therefore remain separate.
//!
//! The emulator intentionally contains its own numerical execution boundary
//! rather than depending on provider APIs. A future implementation may use
//! the canonical simulator engine internally, but doing so must not change
//! this public contract.
//!
//! # Integration contract
//!
//! The emulator consumes canonical `quantum::ir::Gate` values.
//!
//! A caller supplies:
//!
//! ```text
//! EmulationInput {
//!     qubit_count,
//!     gates,
//!     shots,
//! }
//! ```
//!
//! This deliberately avoids coupling the emulator to the internal layout of
//! `QuantumCircuit`. A future execution layer can therefore translate:
//!
//! ```text
//! QuantumCircuit
//!      |
//!      v
//! EmulationInput
//!      |
//!      v
//! HardwareEmulator
//! ```
//!
//! without changing this file.
//!
//! # Hardware abstraction integration
//!
//! The future hardware backend layer may expose this emulator through:
//!
//! ```text
//! BackendKind::Emulator
//!          |
//!          v
//! QuantumBackend
//!          |
//!          v
//! HardwareEmulator
//! ```
//!
//! `emulator.rs` must never make `backend.rs` provider-specific.
//!
//! # Determinism
//!
//! Explicit seeds produce deterministic results for identical:
//!
//! - emulator configuration;
//! - input workload;
//! - seed;
//! - emulator version.
//!
//! The emulator never uses the operating-system random source.
//!
//! # Numerical safety
//!
//! All numerical inputs are checked for:
//!
//! - finiteness;
//! - non-negativity where required;
//! - valid probability ranges;
//! - valid matrix dimensions;
//! - safe state-vector allocation.
//!
//! State-vector allocation is always checked before allocation.
//!
//! # Rust compatibility
//!
//! Supported:
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
//! # Stability
//!
//! The stable public boundary consists of:
//!
//! - `HardwareEmulator`;
//! - `EmulatorConfig`;
//! - `EmulatorLimits`;
//! - `NoiseModel`;
//! - `GateNoise`;
//! - `ReadoutNoise`;
//! - `TimingModel`;
//! - `EmulationInput`;
//! - `EmulationResult`;
//! - `EmulationStatistics`;
//! - `EmulationError`;
//! - `EmulatorVersion`.
//!
//! Internal numerical representation may change without changing this
//! boundary.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::super::ir::gate::{Gate, GateKind};

// =============================================================================
// Schema
// =============================================================================

/// Stable emulator schema identifier.
pub const EMULATOR_SCHEMA_ID: &str = "zamani.quantum.hardware.emulator";

/// Stable emulator schema version.
pub const EMULATOR_SCHEMA_VERSION: u16 = 1;

/// Current emulator implementation version.
pub const EMULATOR_VERSION: EmulatorVersion = EmulatorVersion {
    major: 1,
    minor: 0,
    patch: 0,
};

/// Default maximum emulator qubit count.
pub const DEFAULT_MAX_QUBITS: usize = 28;

/// Default maximum amplitude count.
pub const DEFAULT_MAX_AMPLITUDES: usize = 1usize << DEFAULT_MAX_QUBITS;

/// Default shot count.
pub const DEFAULT_SHOTS: u64 = 1_024;

/// Default numerical tolerance.
pub const DEFAULT_EPSILON: f64 = 1.0e-12;

// =============================================================================
// Version
// =============================================================================

/// Semantic version of the emulator implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EmulatorVersion {
    /// Major version.
    pub major: u16,

    /// Minor version.
    pub minor: u16,

    /// Patch version.
    pub patch: u16,
}

impl EmulatorVersion {
    /// Creates a version.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns a canonical version string.
    pub fn as_string(self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl fmt::Display for EmulatorVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// =============================================================================
// Complex number
// =============================================================================

/// Private complex-number representation.
///
/// This remains private so the numerical implementation can be replaced
/// without changing the public emulator API.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    const ZERO: Self = Self {
        real: 0.0,
        imag: 0.0,
    };

    const ONE: Self = Self {
        real: 1.0,
        imag: 0.0,
    };

    const I: Self = Self {
        real: 0.0,
        imag: 1.0,
    };

    fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    fn magnitude_squared(self) -> f64 {
        self.real.mul_add(self.real, self.imag * self.imag)
    }

    fn conjugate(self) -> Self {
        Self {
            real: self.real,
            imag: -self.imag,
        }
    }

    fn scale(self, value: f64) -> Self {
        Self {
            real: self.real * value,
            imag: self.imag * value,
        }
    }
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl std::ops::Div<f64> for Complex {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            real: self.real / rhs,
            imag: self.imag / rhs,
        }
    }
}

// =============================================================================
// Matrix
// =============================================================================

#[derive(Debug, Clone)]
struct Matrix2 {
    values: [Complex; 4],
}

impl Matrix2 {
    fn new(values: [Complex; 4]) -> Self {
        Self { values }
    }

    fn get(&self, row: usize, column: usize) -> Complex {
        self.values[row * 2 + column]
    }
}

// =============================================================================
// Limits
// =============================================================================

/// Resource limits protecting the emulator from uncontrolled exponential
/// allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmulatorLimits {
    /// Maximum number of physical qubits.
    pub max_qubits: usize,

    /// Maximum number of amplitudes.
    pub max_amplitudes: usize,

    /// Maximum gate count.
    pub max_operations: usize,

    /// Maximum shots.
    pub max_shots: u64,
}

impl Default for EmulatorLimits {
    fn default() -> Self {
        Self::production()
    }
}

impl EmulatorLimits {
    /// Production-safe defaults.
    pub const fn production() -> Self {
        Self {
            max_qubits: DEFAULT_MAX_QUBITS,
            max_amplitudes: DEFAULT_MAX_AMPLITUDES,
            max_operations: 10_000_000,
            max_shots: 10_000_000,
        }
    }

    /// Creates explicit limits.
    pub const fn new(
        max_qubits: usize,
        max_amplitudes: usize,
        max_operations: usize,
        max_shots: u64,
    ) -> Self {
        Self {
            max_qubits,
            max_amplitudes,
            max_operations,
            max_shots,
        }
    }

    /// Validates the limit configuration.
    pub fn validate(&self) -> Result<(), EmulationError> {
        if self.max_qubits == 0 {
            return Err(EmulationError::InvalidLimit {
                field: "max_qubits",
            });
        }

        if self.max_amplitudes == 0 {
            return Err(EmulationError::InvalidLimit {
                field: "max_amplitudes",
            });
        }

        if self.max_operations == 0 {
            return Err(EmulationError::InvalidLimit {
                field: "max_operations",
            });
        }

        if self.max_shots == 0 {
            return Err(EmulationError::InvalidLimit {
                field: "max_shots",
            });
        }

        Ok(())
    }

    fn amplitude_count(&self, qubits: usize) -> Result<usize, EmulationError> {
        if qubits > self.max_qubits {
            return Err(EmulationError::QubitLimitExceeded {
                requested: qubits,
                maximum: self.max_qubits,
            });
        }

        let amplitudes = 1usize
            .checked_shl(
                u32::try_from(qubits)
                    .map_err(|_| EmulationError::ArithmeticOverflow {
                        operation: "qubit shift",
                    })?,
            )
            .ok_or(EmulationError::ArithmeticOverflow {
                operation: "2^qubits",
            })?;

        if amplitudes > self.max_amplitudes {
            return Err(EmulationError::AmplitudeLimitExceeded {
                requested: amplitudes,
                maximum: self.max_amplitudes,
            });
        }

        Ok(amplitudes)
    }
}

// =============================================================================
// Noise
// =============================================================================

/// Gate-level noise parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GateNoise {
    /// Probability of an X error after the operation.
    pub x_error: f64,

    /// Probability of a Y error after the operation.
    pub y_error: f64,

    /// Probability of a Z error after the operation.
    pub z_error: f64,

    /// Additional depolarizing probability.
    pub depolarizing_error: f64,
}

impl Default for GateNoise {
    fn default() -> Self {
        Self::ideal()
    }
}

impl GateNoise {
    /// Zero-noise model.
    pub const fn ideal() -> Self {
        Self {
            x_error: 0.0,
            y_error: 0.0,
            z_error: 0.0,
            depolarizing_error: 0.0,
        }
    }

    /// Creates a symmetric depolarizing model.
    pub fn depolarizing(probability: f64) -> Result<Self, EmulationError> {
        validate_probability(probability)?;

        Ok(Self {
            x_error: probability / 3.0,
            y_error: probability / 3.0,
            z_error: probability / 3.0,
            depolarizing_error: 0.0,
        })
    }

    fn validate(self) -> Result<(), EmulationError> {
        validate_probability(self.x_error)?;
        validate_probability(self.y_error)?;
        validate_probability(self.z_error)?;
        validate_probability(self.depolarizing_error)?;

        let total = self.x_error
            + self.y_error
            + self.z_error
            + self.depolarizing_error;

        if total > 1.0 {
            return Err(EmulationError::InvalidNoiseProbability {
                value: total,
            });
        }

        Ok(())
    }
}

/// Measurement/readout noise.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReadoutNoise {
    /// Probability that logical zero is reported as one.
    pub zero_to_one: f64,

    /// Probability that logical one is reported as zero.
    pub one_to_zero: f64,
}

impl Default for ReadoutNoise {
    fn default() -> Self {
        Self::ideal()
    }
}

impl ReadoutNoise {
    /// Ideal readout.
    pub const fn ideal() -> Self {
        Self {
            zero_to_one: 0.0,
            one_to_zero: 0.0,
        }
    }

    /// Symmetric readout error.
    pub fn symmetric(probability: f64) -> Result<Self, EmulationError> {
        validate_probability(probability)?;

        Ok(Self {
            zero_to_one: probability,
            one_to_zero: probability,
        })
    }

    fn validate(self) -> Result<(), EmulationError> {
        validate_probability(self.zero_to_one)?;
        validate_probability(self.one_to_zero)?;
        Ok(())
    }
}

/// Reset noise.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResetNoise {
    /// Probability that reset leaves the qubit in the wrong computational
    /// state.
    pub failure_probability: f64,
}

impl Default for ResetNoise {
    fn default() -> Self {
        Self::ideal()
    }
}

impl ResetNoise {
    /// Ideal reset.
    pub const fn ideal() -> Self {
        Self {
            failure_probability: 0.0,
        }
    }

    /// Creates reset noise.
    pub fn new(probability: f64) -> Result<Self, EmulationError> {
        validate_probability(probability)?;

        Ok(Self {
            failure_probability: probability,
        })
    }

    fn validate(self) -> Result<(), EmulationError> {
        validate_probability(self.failure_probability)
    }
}

/// Complete hardware noise model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoiseModel {
    /// Default single-qubit gate noise.
    pub single_qubit: GateNoise,

    /// Default multi-qubit gate noise.
    pub multi_qubit: GateNoise,

    /// Readout noise.
    pub readout: ReadoutNoise,

    /// Reset noise.
    pub reset: ResetNoise,
}

impl Default for NoiseModel {
    fn default() -> Self {
        Self::ideal()
    }
}

impl NoiseModel {
    /// Completely ideal hardware.
    pub const fn ideal() -> Self {
        Self {
            single_qubit: GateNoise::ideal(),
            multi_qubit: GateNoise::ideal(),
            readout: ReadoutNoise::ideal(),
            reset: ResetNoise::ideal(),
        }
    }

    /// Validates the complete model.
    pub fn validate(self) -> Result<(), EmulationError> {
        self.single_qubit.validate()?;
        self.multi_qubit.validate()?;
        self.readout.validate()?;
        self.reset.validate()?;
        Ok(())
    }
}

// =============================================================================
// Timing
// =============================================================================

/// Gate timing model.
///
/// Durations are represented in nanoseconds to keep the emulator deterministic
/// and independent of floating-point time accumulation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingModel {
    /// Default single-qubit duration.
    pub default_single_qubit_ns: u64,

    /// Default two-qubit duration.
    pub default_two_qubit_ns: u64,

    /// Default three-qubit duration.
    pub default_three_qubit_ns: u64,

    /// Measurement duration.
    pub measurement_ns: u64,

    /// Reset duration.
    pub reset_ns: u64,

    /// Explicit instruction durations.
    pub instruction_durations_ns: BTreeMap<String, u64>,
}

impl Default for TimingModel {
    fn default() -> Self {
        Self::production()
    }
}

impl TimingModel {
    /// Production defaults.
    pub fn production() -> Self {
        Self {
            default_single_qubit_ns: 50,
            default_two_qubit_ns: 300,
            default_three_qubit_ns: 500,
            measurement_ns: 1_000,
            reset_ns: 1_000,
            instruction_durations_ns: BTreeMap::new(),
        }
    }

    /// Validates timing data.
    pub fn validate(&self) -> Result<(), EmulationError> {
        if self.default_single_qubit_ns == 0
            || self.default_two_qubit_ns == 0
            || self.default_three_qubit_ns == 0
            || self.measurement_ns == 0
            || self.reset_ns == 0
        {
            return Err(EmulationError::InvalidTiming);
        }

        Ok(())
    }

    fn duration_for(&self, kind: GateKind) -> u64 {
        let name = gate_name(kind);

        if let Some(duration) = self.instruction_durations_ns.get(name) {
            return *duration;
        }

        if kind.is_measurement() {
            return self.measurement_ns;
        }

        if kind.is_reset() {
            return self.reset_ns;
        }

        match kind.operand_count() {
            super::super::ir::gate::OperandCount::Exact(1) => {
                self.default_single_qubit_ns
            }
            super::super::ir::gate::OperandCount::Exact(2) => {
                self.default_two_qubit_ns
            }
            super::super::ir::gate::OperandCount::Exact(3)
            | super::super::ir::gate::OperandCount::AtLeast(3) => {
                self.default_three_qubit_ns
            }
            super::super::ir::gate::OperandCount::AtLeast(1) => {
                self.default_single_qubit_ns
            }
        }
    }
}

// =============================================================================
// Connectivity
// =============================================================================

/// Physical connectivity model.
///
/// An empty coupling set means fully connected hardware.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connectivity {
    qubit_count: usize,
    couplings: BTreeSet<(usize, usize)>,
    fully_connected: bool,
}

impl Connectivity {
    /// Creates fully connected hardware.
    pub fn fully_connected(qubit_count: usize) -> Self {
        Self {
            qubit_count,
            couplings: BTreeSet::new(),
            fully_connected: true,
        }
    }

    /// Creates hardware with explicit undirected couplings.
    pub fn new(
        qubit_count: usize,
        couplings: impl IntoIterator<Item = (usize, usize)>,
    ) -> Result<Self, EmulationError> {
        let mut result = Self {
            qubit_count,
            couplings: BTreeSet::new(),
            fully_connected: false,
        };

        for (a, b) in couplings {
            result.add_coupling(a, b)?;
        }

        Ok(result)
    }

    /// Adds an undirected coupling.
    pub fn add_coupling(
        &mut self,
        a: usize,
        b: usize,
    ) -> Result<(), EmulationError> {
        if a >= self.qubit_count || b >= self.qubit_count {
            return Err(EmulationError::InvalidConnectivity {
                first: a,
                second: b,
            });
        }

        if a == b {
            return Err(EmulationError::InvalidConnectivity {
                first: a,
                second: b,
            });
        }

        let pair = if a < b { (a, b) } else { (b, a) };

        self.couplings.insert(pair);

        Ok(())
    }

    /// Returns whether two physical qubits may directly interact.
    pub fn connected(&self, a: usize, b: usize) -> bool {
        if a >= self.qubit_count || b >= self.qubit_count || a == b {
            return false;
        }

        self.fully_connected
            || self.couplings.contains(&(a.min(b), a.max(b)))
    }

    /// Returns the number of modeled qubits.
    pub const fn qubit_count(&self) -> usize {
        self.qubit_count
    }
}

// =============================================================================
// Input
// =============================================================================

/// A provider-independent workload supplied to the emulator.
#[derive(Debug, Clone)]
pub struct EmulationInput {
    /// Number of logical/physical qubits represented by this execution.
    pub qubit_count: usize,

    /// Ordered canonical IR operations.
    pub gates: Vec<Gate>,

    /// Number of independent shots.
    pub shots: u64,

    /// Optional execution seed overriding the emulator configuration.
    pub seed: Option<u64>,
}

impl EmulationInput {
    /// Creates an input workload.
    pub fn new(
        qubit_count: usize,
        gates: Vec<Gate>,
        shots: u64,
    ) -> Self {
        Self {
            qubit_count,
            gates,
            shots,
            seed: None,
        }
    }

    /// Sets an explicit deterministic seed.
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Complete hardware-emulator configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct EmulatorConfig {
    /// Resource limits.
    pub limits: EmulatorLimits,

    /// Hardware noise.
    pub noise: NoiseModel,

    /// Timing model.
    pub timing: TimingModel,

    /// Physical connectivity.
    pub connectivity: Connectivity,

    /// Supported native instructions.
    ///
    /// An empty set means all gate kinds supported by the emulator engine.
    pub native_instructions: BTreeSet<String>,

    /// Default seed.
    pub seed: Option<u64>,

    /// Numerical tolerance.
    pub epsilon: f64,

    /// Whether measurement operations are accepted.
    pub allow_measurement: bool,

    /// Whether reset operations are accepted.
    pub allow_reset: bool,
}

impl EmulatorConfig {
    /// Creates a production configuration for `qubit_count` qubits.
    pub fn production(qubit_count: usize) -> Result<Self, EmulationError> {
        let limits = EmulatorLimits::production();

        if qubit_count > limits.max_qubits {
            return Err(EmulationError::QubitLimitExceeded {
                requested: qubit_count,
                maximum: limits.max_qubits,
            });
        }

        Ok(Self {
            limits,
            noise: NoiseModel::ideal(),
            timing: TimingModel::production(),
            connectivity: Connectivity::fully_connected(qubit_count),
            native_instructions: BTreeSet::new(),
            seed: None,
            epsilon: DEFAULT_EPSILON,
            allow_measurement: true,
            allow_reset: true,
        })
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<(), EmulationError> {
        self.limits.validate()?;
        self.noise.validate()?;
        self.timing.validate()?;

        if !self.epsilon.is_finite() || self.epsilon <= 0.0 {
            return Err(EmulationError::InvalidEpsilon {
                value: self.epsilon,
            });
        }

        if self.connectivity.qubit_count() > self.limits.max_qubits {
            return Err(EmulationError::QubitLimitExceeded {
                requested: self.connectivity.qubit_count(),
                maximum: self.limits.max_qubits,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Result
// =============================================================================

/// Result of hardware emulation.
#[derive(Debug, Clone, PartialEq)]
pub struct EmulationResult {
    /// Schema identifier.
    pub schema_id: &'static str,

    /// Schema version.
    pub schema_version: u16,

    /// Emulator version.
    pub emulator_version: EmulatorVersion,

    /// Number of qubits.
    pub qubit_count: usize,

    /// Number of shots.
    pub shots: u64,

    /// Measurement counts indexed by canonical bitstring.
    pub counts: BTreeMap<String, u64>,

    /// Final state-vector from the final execution shot.
    ///
    /// This is primarily diagnostic. For noisy shot-based execution it should
    /// not be interpreted as the ensemble density matrix.
    pub final_state_vector: Vec<(f64, f64)>,

    /// Total modeled execution time in nanoseconds.
    pub execution_time_ns: u64,

    /// Number of modeled operations.
    pub operation_count: usize,

    /// Number of injected stochastic gate errors.
    pub injected_gate_errors: u64,

    /// Number of injected reset errors.
    pub injected_reset_errors: u64,

    /// Number of injected readout errors.
    pub injected_readout_errors: u64,
}

impl EmulationResult {
    /// Returns the probability of a measured bitstring.
    pub fn probability(&self, bitstring: &str) -> f64 {
        if self.shots == 0 {
            return 0.0;
        }

        self.counts.get(bitstring).copied().unwrap_or(0) as f64
            / self.shots as f64
    }
}

/// Aggregate execution statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmulationStatistics {
    /// Total operations.
    pub operation_count: usize,

    /// Total shots.
    pub shots: u64,

    /// Total modeled execution time.
    pub execution_time_ns: u64,

    /// Injected gate errors.
    pub injected_gate_errors: u64,

    /// Injected reset errors.
    pub injected_reset_errors: u64,

    /// Injected readout errors.
    pub injected_readout_errors: u64,
}

// =============================================================================
// Errors
// =============================================================================

/// Structured emulator errors.
#[derive(Debug, Clone, PartialEq)]
pub enum EmulationError {
    /// Invalid emulator limit.
    InvalidLimit {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// Qubit count exceeds configured limit.
    QubitLimitExceeded {
        /// Requested count.
        requested: usize,

        /// Maximum count.
        maximum: usize,
    },

    /// State-vector allocation exceeds configured amplitude limit.
    AmplitudeLimitExceeded {
        /// Requested amplitude count.
        requested: usize,

        /// Maximum amplitude count.
        maximum: usize,
    },

    /// Operation count exceeds configured limit.
    OperationLimitExceeded {
        /// Requested operations.
        requested: usize,

        /// Maximum operations.
        maximum: usize,
    },

    /// Shot count is invalid.
    InvalidShots {
        /// Requested shots.
        shots: u64,
    },

    /// Shot count exceeds configured maximum.
    ShotLimitExceeded {
        /// Requested shots.
        requested: u64,

        /// Maximum shots.
        maximum: u64,
    },

    /// Invalid probability/noise value.
    InvalidNoiseProbability {
        /// Invalid probability.
        value: f64,
    },

    /// Invalid numerical tolerance.
    InvalidEpsilon {
        /// Invalid epsilon.
        value: f64,
    },

    /// Invalid timing model.
    InvalidTiming,

    /// Invalid physical connectivity.
    InvalidConnectivity {
        /// First qubit.
        first: usize,

        /// Second qubit.
        second: usize,
    },

    /// Input qubit is outside the emulated device.
    QubitOutOfRange {
        /// Qubit index.
        qubit: usize,

        /// Number of qubits.
        qubit_count: usize,
    },

    /// A multi-qubit operation targets disconnected qubits.
    UnsupportedConnectivity {
        /// Gate name.
        gate: &'static str,

        /// First target.
        first: usize,

        /// Second target.
        second: usize,
    },

    /// A gate is not supported by the configured native instruction set.
    UnsupportedInstruction {
        /// Gate name.
        gate: &'static str,
    },

    /// A symbolic parameter reached the numerical emulator.
    NonConstantParameter {
        /// Gate name.
        gate: &'static str,
    },

    /// Gate parameter has invalid numerical value.
    InvalidParameter {
        /// Gate name.
        gate: &'static str,
    },

    /// Gate is not supported by this numerical engine.
    UnsupportedGate {
        /// Gate name.
        gate: &'static str,
    },

    /// Measurement is disabled.
    MeasurementDisabled,

    /// Reset is disabled.
    ResetDisabled,

    /// State-vector arithmetic overflowed.
    ArithmeticOverflow {
        /// Operation.
        operation: &'static str,
    },

    /// Numerical normalization failed.
    NumericalInstability {
        /// Description.
        operation: &'static str,
    },
}

impl fmt::Display for EmulationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLimit { field } => {
                write!(formatter, "invalid emulator limit `{field}`")
            }

            Self::QubitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "emulator qubit limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::AmplitudeLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "emulator amplitude limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::OperationLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "emulator operation limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::InvalidShots { shots } => {
                write!(formatter, "invalid shot count: {shots}")
            }

            Self::ShotLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "emulator shot limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::InvalidNoiseProbability { value } => {
                write!(formatter, "invalid noise probability: {value}")
            }

            Self::InvalidEpsilon { value } => {
                write!(formatter, "invalid numerical epsilon: {value}")
            }

            Self::InvalidTiming => {
                write!(formatter, "invalid emulator timing configuration")
            }

            Self::InvalidConnectivity { first, second } => {
                write!(
                    formatter,
                    "invalid connectivity between q{first} and q{second}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                qubit_count,
            } => {
                write!(
                    formatter,
                    "qubit q{qubit} is outside emulated device with {qubit_count} qubits"
                )
            }

            Self::UnsupportedConnectivity {
                gate,
                first,
                second,
            } => {
                write!(
                    formatter,
                    "gate {gate} requires unsupported connectivity q{first}-q{second}"
                )
            }

            Self::UnsupportedInstruction { gate } => {
                write!(
                    formatter,
                    "instruction `{gate}` is not supported by this emulator"
                )
            }

            Self::NonConstantParameter { gate } => {
                write!(
                    formatter,
                    "gate `{gate}` contains a symbolic/non-constant parameter"
                )
            }

            Self::InvalidParameter { gate } => {
                write!(
                    formatter,
                    "gate `{gate}` contains an invalid numerical parameter"
                )
            }

            Self::UnsupportedGate { gate } => {
                write!(
                    formatter,
                    "gate `{gate}` is unsupported by the emulator numerical engine"
                )
            }

            Self::MeasurementDisabled => {
                write!(formatter, "measurement operations are disabled")
            }

            Self::ResetDisabled => {
                write!(formatter, "reset operations are disabled")
            }

            Self::ArithmeticOverflow { operation } => {
                write!(formatter, "arithmetic overflow during {operation}")
            }

            Self::NumericalInstability { operation } => {
                write!(formatter, "numerical instability during {operation}")
            }
        }
    }
}

impl std::error::Error for EmulationError {}

// =============================================================================
// Emulator
// =============================================================================

/// Production hardware emulator.
///
/// The emulator is immutable after construction. Every execution allocates
/// independent state and random state, making concurrent read-only use safe.
#[derive(Debug, Clone)]
pub struct HardwareEmulator {
    config: EmulatorConfig,
}

impl HardwareEmulator {
    /// Creates and validates an emulator.
    pub fn new(config: EmulatorConfig) -> Result<Self, EmulationError> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the immutable emulator configuration.
    pub fn config(&self) -> &EmulatorConfig {
        &self.config
    }

    /// Validates an execution workload without executing it.
    pub fn validate(
        &self,
        input: &EmulationInput,
    ) -> Result<(), EmulationError> {
        self.config.validate()?;

        if input.qubit_count == 0 {
            return Err(EmulationError::QubitLimitExceeded {
                requested: 0,
                maximum: self.config.limits.max_qubits,
            });
        }

        self.config
            .limits
            .amplitude_count(input.qubit_count)?;

        if input.shots == 0 {
            return Err(EmulationError::InvalidShots {
                shots: input.shots,
            });
        }

        if input.shots > self.config.limits.max_shots {
            return Err(EmulationError::ShotLimitExceeded {
                requested: input.shots,
                maximum: self.config.limits.max_shots,
            });
        }

        if input.gates.len() > self.config.limits.max_operations {
            return Err(EmulationError::OperationLimitExceeded {
                requested: input.gates.len(),
                maximum: self.config.limits.max_operations,
            });
        }

        if input.qubit_count != self.config.connectivity.qubit_count() {
            return Err(EmulationError::QubitLimitExceeded {
                requested: input.qubit_count,
                maximum: self.config.connectivity.qubit_count(),
            });
        }

        for gate in &input.gates {
            self.validate_gate(gate, input.qubit_count)?;
        }

        Ok(())
    }

    /// Executes a workload using the configured hardware model.
    pub fn emulate(
        &self,
        input: &EmulationInput,
    ) -> Result<EmulationResult, EmulationError> {
        self.validate(input)?;

        let seed = input.seed.or(self.config.seed).unwrap_or(0x5EED_5EED);

        let amplitudes = self
            .config
            .limits
            .amplitude_count(input.qubit_count)?;

        let mut counts = BTreeMap::new();

        let mut total_gate_errors = 0u64;
        let mut total_reset_errors = 0u64;
        let mut total_readout_errors = 0u64;

        let mut final_state = vec![Complex::ZERO; amplitudes];

        let execution_time_ns = input
            .gates
            .iter()
            .try_fold(0u64, |total, gate| {
                total.checked_add(
                    self.config.timing.duration_for(gate.kind()),
                )
                .ok_or(EmulationError::ArithmeticOverflow {
                    operation: "execution time",
                })
            })?;

        for shot in 0..input.shots {
            let shot_seed = split_seed(seed, shot);
            let mut rng = DeterministicRng::new(shot_seed);

            let mut state = vec![Complex::ZERO; amplitudes];
            state[0] = Complex::ONE;

            for gate in &input.gates {
                match gate.kind() {
                    GateKind::Measure => {
                        let qubit = gate
                            .qubits()
                            .first()
                            .ok_or(EmulationError::UnsupportedGate {
                                gate: "measure",
                            })?
                            .index();

                        let measured = measure_qubit(
                            &mut state,
                            input.qubit_count,
                            qubit,
                            self.config.epsilon,
                            &mut rng,
                        )?;

                        if let Some(classical) = gate.classical_target() {
                            let reported = apply_readout_noise(
                                measured,
                                self.config.noise.readout,
                                &mut rng,
                            );

                            if reported != measured {
                                total_readout_errors =
                                    total_readout_errors.saturating_add(1);
                            }

                            let bitstring = format_measurement(
                                reported,
                                classical,
                                input.qubit_count,
                            );

                            *counts.entry(bitstring).or_insert(0) += 1;
                        }
                    }

                    GateKind::Reset => {
                        let qubit = gate
                            .qubits()
                            .first()
                            .ok_or(EmulationError::UnsupportedGate {
                                gate: "reset",
                            })?
                            .index();

                        let failed = reset_qubit(
                            &mut state,
                            input.qubit_count,
                            qubit,
                            self.config.epsilon,
                            self.config.noise.reset,
                            &mut rng,
                        )?;

                        if failed {
                            total_reset_errors =
                                total_reset_errors.saturating_add(1);
                        }
                    }

                    GateKind::Barrier => {}

                    _ => {
                        apply_gate(
                            &mut state,
                            input.qubit_count,
                            gate,
                        )?;

                        let noise = if gate.qubits().len() <= 1 {
                            self.config.noise.single_qubit
                        } else {
                            self.config.noise.multi_qubit
                        };

                        total_gate_errors +=
                            apply_gate_noise(
                                &mut state,
                                input.qubit_count,
                                gate,
                                noise,
                                self.config.epsilon,
                                &mut rng,
                            )?;
                    }
                }
            }

            final_state = state;

            if !has_measurement(&input.gates) {
                let sample = sample_state(
                    &final_state,
                    input.qubit_count,
                    &mut rng,
                )?;

                let mut bitstring = format_bitstring(
                    sample,
                    input.qubit_count,
                );

                apply_readout_noise_to_bitstring(
                    &mut bitstring,
                    self.config.noise.readout,
                    &mut rng,
                    &mut total_readout_errors,
                );

                *counts.entry(bitstring).or_insert(0) += 1;
            }
        }

        let final_state_vector = final_state
            .iter()
            .map(|value| (value.real, value.imag))
            .collect();

        Ok(EmulationResult {
            schema_id: EMULATOR_SCHEMA_ID,
            schema_version: EMULATOR_SCHEMA_VERSION,
            emulator_version: EMULATOR_VERSION,
            qubit_count: input.qubit_count,
            shots: input.shots,
            counts,
            final_state_vector,
            execution_time_ns,
            operation_count: input.gates.len(),
            injected_gate_errors: total_gate_errors,
            injected_reset_errors: total_reset_errors,
            injected_readout_errors: total_readout_errors,
        })
    }

    /// Returns aggregate statistics for a completed result.
    pub fn statistics(
        &self,
        result: &EmulationResult,
    ) -> EmulationStatistics {
        EmulationStatistics {
            operation_count: result.operation_count,
            shots: result.shots,
            execution_time_ns: result.execution_time_ns,
            injected_gate_errors: result.injected_gate_errors,
            injected_reset_errors: result.injected_reset_errors,
            injected_readout_errors: result.injected_readout_errors,
        }
    }

    fn validate_gate(
        &self,
        gate: &Gate,
        qubit_count: usize,
    ) -> Result<(), EmulationError> {
        let name = gate_name(gate.kind());

        if !self.config.native_instructions.is_empty()
            && !self.config.native_instructions.contains(name)
        {
            return Err(EmulationError::UnsupportedInstruction {
                gate: name,
            });
        }

        if gate.is_measurement() && !self.config.allow_measurement {
            return Err(EmulationError::MeasurementDisabled);
        }

        if gate.is_reset() && !self.config.allow_reset {
            return Err(EmulationError::ResetDisabled);
        }

        for qubit in gate.qubits() {
            let index = qubit.index();

            if index >= qubit_count {
                return Err(EmulationError::QubitOutOfRange {
                    qubit: index,
                    qubit_count,
                });
            }
        }

        if gate.qubits().len() >= 2 {
            let first = gate.qubits()[0].index();
            let second = gate.qubits()[1].index();

            if !self.config.connectivity.connected(first, second) {
                return Err(EmulationError::UnsupportedConnectivity {
                    gate: name,
                    first,
                    second,
                });
            }
        }

        if gate.is_parameterized() {
            let parameters = gate
                .constant_parameters()
                .ok_or(EmulationError::NonConstantParameter {
                    gate: name,
                })?;

            if parameters.iter().any(|value| !value.is_finite()) {
                return Err(EmulationError::InvalidParameter {
                    gate: name,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Gate execution
// =============================================================================

fn apply_gate(
    state: &mut [Complex],
    qubit_count: usize,
    gate: &Gate,
) -> Result<(), EmulationError> {
    let qubits = gate.qubits();

    match gate.kind() {
        GateKind::I => {}

        GateKind::X => {
            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                Matrix2::new([
                    Complex::ZERO,
                    Complex::ONE,
                    Complex::ONE,
                    Complex::ZERO,
                ]),
            );
        }

        GateKind::Y => {
            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                Matrix2::new([
                    Complex::ZERO,
                    Complex::new(0.0, -1.0),
                    Complex::I,
                    Complex::ZERO,
                ]),
            );
        }

        GateKind::Z => {
            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                Matrix2::new([
                    Complex::ONE,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::new(-1.0, 0.0),
                ]),
            );
        }

        GateKind::H => {
            let s = 1.0 / 2.0_f64.sqrt();

            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                Matrix2::new([
                    Complex::new(s, 0.0),
                    Complex::new(s, 0.0),
                    Complex::new(s, 0.0),
                    Complex::new(-s, 0.0),
                ]),
            );
        }

        GateKind::S => {
            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                Matrix2::new([
                    Complex::ONE,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::I,
                ]),
            );
        }

        GateKind::Sdg => {
            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                Matrix2::new([
                    Complex::ONE,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::new(0.0, -1.0),
                ]),
            );
        }

        GateKind::T => {
            let angle = std::f64::consts::FRAC_PI_4;

            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                phase_matrix(angle),
            );
        }

        GateKind::Tdg => {
            let angle = -std::f64::consts::FRAC_PI_4;

            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                phase_matrix(angle),
            );
        }

        GateKind::V => {
            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                rotation_x(std::f64::consts::FRAC_PI_2),
            );
        }

        GateKind::Vdg => {
            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                rotation_x(-std::f64::consts::FRAC_PI_2),
            );
        }

        GateKind::RX => {
            let theta = parameter(gate, 0)?;
            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                rotation_x(theta),
            );
        }

        GateKind::RY => {
            let theta = parameter(gate, 0)?;
            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                rotation_y(theta),
            );
        }

        GateKind::RZ => {
            let theta = parameter(gate, 0)?;
            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                rotation_z(theta),
            );
        }

        GateKind::Phase | GateKind::U1 => {
            let theta = parameter(gate, 0)?;
            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                phase_matrix(theta),
            );
        }

        GateKind::U2 => {
            let phi = parameter(gate, 0)?;
            let lambda = parameter(gate, 1)?;

            let matrix = u3(
                std::f64::consts::FRAC_PI_2,
                phi,
                lambda,
            );

            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                matrix,
            );
        }

        GateKind::U3 => {
            let theta = parameter(gate, 0)?;
            let phi = parameter(gate, 1)?;
            let lambda = parameter(gate, 2)?;

            apply_single(
                state,
                qubit_count,
                qubits[0].index(),
                u3(theta, phi, lambda),
            );
        }

        GateKind::CX => {
            apply_controlled_x(
                state,
                qubit_count,
                qubits[0].index(),
                qubits[1].index(),
            );
        }

        GateKind::CY => {
            apply_controlled_single(
                state,
                qubit_count,
                qubits[0].index(),
                qubits[1].index(),
                Matrix2::new([
                    Complex::ZERO,
                    Complex::new(0.0, -1.0),
                    Complex::I,
                    Complex::ZERO,
                ]),
            );
        }

        GateKind::CZ => {
            apply_controlled_single(
                state,
                qubit_count,
                qubits[0].index(),
                qubits[1].index(),
                Matrix2::new([
                    Complex::ONE,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::new(-1.0, 0.0),
                ]),
            );
        }

        GateKind::CH => {
            let s = 1.0 / 2.0_f64.sqrt();

            apply_controlled_single(
                state,
                qubit_count,
                qubits[0].index(),
                qubits[1].index(),
                Matrix2::new([
                    Complex::new(s, 0.0),
                    Complex::new(s, 0.0),
                    Complex::new(s, 0.0),
                    Complex::new(-s, 0.0),
                ]),
            );
        }

        GateKind::SWAP => {
            apply_swap(
                state,
                qubit_count,
                qubits[0].index(),
                qubits[1].index(),
            );
        }

        GateKind::ISWAP => {
            apply_iswap(
                state,
                qubit_count,
                qubits[0].index(),
                qubits[1].index(),
            );
        }

        GateKind::ECR => {
            // ECR is represented by its standard echoed-cross-resonance
            // matrix. The implementation is expressed as a direct matrix
            // action on the two-qubit subspace.
            apply_two_qubit_ecr(
                state,
                qubit_count,
                qubits[0].index(),
                qubits[1].index(),
            );
        }

        GateKind::CRX => {
            let theta = parameter(gate, 0)?;

            apply_controlled_single(
                state,
                qubit_count,
                qubits[0].index(),
                qubits[1].index(),
                rotation_x(theta),
            );
        }

        GateKind::CRY => {
            let theta = parameter(gate, 0)?;

            apply_controlled_single(
                state,
                qubit_count,
                qubits[0].index(),
                qubits[1].index(),
                rotation_y(theta),
            );
        }

        GateKind::CRZ => {
            let theta = parameter(gate, 0)?;

            apply_controlled_single(
                state,
                qubit_count,
                qubits[0].index(),
                qubits[1].index(),
                rotation_z(theta),
            );
        }

        GateKind::CCX => {
            apply_toffoli(
                state,
                qubit_count,
                qubits[0].index(),
                qubits[1].index(),
                qubits[2].index(),
            );
        }

        GateKind::CSWAP => {
            apply_fredkin(
                state,
                qubit_count,
                qubits[0].index(),
                qubits[1].index(),
                qubits[2].index(),
            );
        }

        GateKind::Measure | GateKind::Barrier | GateKind::Reset => {}

    }

    Ok(())
}

// =============================================================================
// Single-qubit matrices
// =============================================================================

fn rotation_x(theta: f64) -> Matrix2 {
    let c = (theta / 2.0).cos();
    let s = (theta / 2.0).sin();

    Matrix2::new([
        Complex::new(c, 0.0),
        Complex::new(0.0, -s),
        Complex::new(0.0, -s),
        Complex::new(c, 0.0),
    ])
}

fn rotation_y(theta: f64) -> Matrix2 {
    let c = (theta / 2.0).cos();
    let s = (theta / 2.0).sin();

    Matrix2::new([
        Complex::new(c, 0.0),
        Complex::new(-s, 0.0),
        Complex::new(s, 0.0),
        Complex::new(c, 0.0),
    ])
}

fn rotation_z(theta: f64) -> Matrix2 {
    let c = (theta / 2.0).cos();
    let s = (theta / 2.0).sin();

    Matrix2::new([
        Complex::new(c, -s),
        Complex::ZERO,
        Complex::ZERO,
        Complex::new(c, s),
    ])
}

fn phase_matrix(theta: f64) -> Matrix2 {
    Matrix2::new([
        Complex::ONE,
        Complex::ZERO,
        Complex::ZERO,
        Complex::new(theta.cos(), theta.sin()),
    ])
}

fn u3(theta: f64, phi: f64, lambda: f64) -> Matrix2 {
    let c = (theta / 2.0).cos();
    let s = (theta / 2.0).sin();

    Matrix2::new([
        Complex::new(c, 0.0),
        Complex::new(-lambda.cos() * s, -lambda.sin() * s),
        Complex::new(phi.cos() * s, phi.sin() * s),
        Complex::new(
            (phi + lambda).cos() * c,
            (phi + lambda).sin() * c,
        ),
    ])
}

// =============================================================================
// State-vector operations
// =============================================================================

fn apply_single(
    state: &mut [Complex],
    qubit_count: usize,
    target: usize,
    matrix: Matrix2,
) {
    let stride = 1usize << target;
    let block = stride << 1;

    let _ = qubit_count;

    for base in (0..state.len()).step_by(block) {
        for offset in 0..stride {
            let zero = base + offset;
            let one = zero + stride;

            let a = state[zero];
            let b = state[one];

            state[zero] =
                matrix.get(0, 0) * a + matrix.get(0, 1) * b;

            state[one] =
                matrix.get(1, 0) * a + matrix.get(1, 1) * b;
        }
    }
}

fn apply_controlled_single(
    state: &mut [Complex],
    qubit_count: usize,
    control: usize,
    target: usize,
    matrix: Matrix2,
) {
    if control == target {
        return;
    }

    let control_mask = 1usize << control;
    let target_mask = 1usize << target;

    let _ = qubit_count;

    for index in 0..state.len() {
        if index & control_mask == 0
            || index & target_mask != 0
        {
            continue;
        }

        let pair = index | target_mask;

        let a = state[index];
        let b = state[pair];

        state[index] =
            matrix.get(0, 0) * a + matrix.get(0, 1) * b;

        state[pair] =
            matrix.get(1, 0) * a + matrix.get(1, 1) * b;
    }
}

fn apply_controlled_x(
    state: &mut [Complex],
    qubit_count: usize,
    control: usize,
    target: usize,
) {
    apply_controlled_single(
        state,
        qubit_count,
        control,
        target,
        Matrix2::new([
            Complex::ZERO,
            Complex::ONE,
            Complex::ONE,
            Complex::ZERO,
        ]),
    );
}

fn apply_swap(
    state: &mut [Complex],
    _qubit_count: usize,
    a: usize,
    b: usize,
) {
    if a == b {
        return;
    }

    let mask_a = 1usize << a;
    let mask_b = 1usize << b;

    for index in 0..state.len() {
        let a_bit = index & mask_a != 0;
        let b_bit = index & mask_b != 0;

        if a_bit == b_bit {
            continue;
        }

        let swapped =
            index ^ mask_a ^ mask_b;

        if index < swapped {
            state.swap(index, swapped);
        }
    }
}

fn apply_iswap(
    state: &mut [Complex],
    _qubit_count: usize,
    a: usize,
    b: usize,
) {
    if a == b {
        return;
    }

    let mask_a = 1usize << a;
    let mask_b = 1usize << b;

    let phase = Complex::I;

    for index in 0..state.len() {
        let a_bit = index & mask_a != 0;
        let b_bit = index & mask_b != 0;

        if a_bit == b_bit {
            continue;
        }

        let partner = index ^ mask_a ^ mask_b;

        if index < partner {
            let left = state[index];
            let right = state[partner];

            state[index] = phase * right;
            state[partner] = phase * left;
        }
    }
}

fn apply_two_qubit_ecr(
    state: &mut [Complex],
    qubit_count: usize,
    control: usize,
    target: usize,
) {
    // The ECR gate can be represented as a Clifford-equivalent echoed
    // cross-resonance operation. The decomposition below provides a
    // deterministic logical model suitable for hardware emulation.
    //
    // This is intentionally kept at the logical-emulation layer. A physical
    // backend adapter may replace it with calibrated native pulse semantics.
    apply_single(
        state,
        qubit_count,
        target,
        rotation_x(std::f64::consts::FRAC_PI_2),
    );

    apply_controlled_x(
        state,
        qubit_count,
        control,
        target,
    );

    apply_single(
        state,
        qubit_count,
        target,
        rotation_x(-std::f64::consts::FRAC_PI_2),
    );
}

fn apply_toffoli(
    state: &mut [Complex],
    _qubit_count: usize,
    control_a: usize,
    control_b: usize,
    target: usize,
) {
    let a_mask = 1usize << control_a;
    let b_mask = 1usize << control_b;
    let target_mask = 1usize << target;

    for index in 0..state.len() {
        if index & a_mask != 0
            && index & b_mask != 0
            && index & target_mask == 0
        {
            let partner = index | target_mask;

            state.swap(index, partner);
        }
    }
}

fn apply_fredkin(
    state: &mut [Complex],
    _qubit_count: usize,
    control: usize,
    a: usize,
    b: usize,
) {
    let control_mask = 1usize << control;
    let a_mask = 1usize << a;
    let b_mask = 1usize << b;

    for index in 0..state.len() {
        if index & control_mask == 0 {
            continue;
        }

        let a_bit = index & a_mask != 0;
        let b_bit = index & b_mask != 0;

        if a_bit == b_bit {
            continue;
        }

        let partner = index ^ a_mask ^ b_mask;

        if index < partner {
            state.swap(index, partner);
        }
    }
}

// =============================================================================
// Measurement
// =============================================================================

fn measure_qubit(
    state: &mut [Complex],
    qubit_count: usize,
    qubit: usize,
    epsilon: f64,
    rng: &mut DeterministicRng,
) -> Result<u8, EmulationError> {
    if qubit >= qubit_count {
        return Err(EmulationError::QubitOutOfRange {
            qubit,
            qubit_count,
        });
    }

    let mask = 1usize << qubit;

    let mut probability_one = 0.0;

    for (index, amplitude) in state.iter().enumerate() {
        if index & mask != 0 {
            probability_one += amplitude.magnitude_squared();
        }
    }

    if !probability_one.is_finite() {
        return Err(EmulationError::NumericalInstability {
            operation: "measurement probability",
        });
    }

    let probability_one =
        probability_one.clamp(0.0, 1.0);

    let result =
        if rng.next_f64() < probability_one {
            1
        } else {
            0
        };

    let selected_probability =
        if result == 1 {
            probability_one
        } else {
            1.0 - probability_one
        };

    if selected_probability <= epsilon {
        for (index, amplitude) in state.iter_mut().enumerate() {
            if ((index & mask != 0) as u8) != result {
                *amplitude = Complex::ZERO;
            }
        }
    } else {
        let norm = selected_probability.sqrt();

        for (index, amplitude) in state.iter_mut().enumerate() {
            let bit = if index & mask != 0 { 1 } else { 0 };

            if bit != result {
                *amplitude = Complex::ZERO;
            } else {
                *amplitude = *amplitude / norm;
            }
        }
    }

    Ok(result)
}

fn sample_state(
    state: &[Complex],
    qubit_count: usize,
    rng: &mut DeterministicRng,
) -> Result<usize, EmulationError> {
    let random = rng.next_f64();

    let mut cumulative = 0.0;

    for (index, amplitude) in state.iter().enumerate() {
        cumulative += amplitude.magnitude_squared();

        if random <= cumulative {
            return Ok(index);
        }
    }

    if state.is_empty() {
        return Err(EmulationError::NumericalInstability {
            operation: "state sampling",
        });
    }

    let _ = qubit_count;

    Ok(state.len() - 1)
}

fn reset_qubit(
    state: &mut [Complex],
    qubit_count: usize,
    qubit: usize,
    epsilon: f64,
    noise: ResetNoise,
    rng: &mut DeterministicRng,
) -> Result<bool, EmulationError> {
    let measured =
        measure_qubit(
            state,
            qubit_count,
            qubit,
            epsilon,
            rng,
        )?;

    let failed = rng.next_f64() < noise.failure_probability;

    if measured == 1 {
        apply_single(
            state,
            qubit_count,
            qubit,
            Matrix2::new([
                Complex::ZERO,
                Complex::ONE,
                Complex::ONE,
                Complex::ZERO,
            ]),
        );
    }

    if failed {
        apply_single(
            state,
            qubit_count,
            qubit,
            Matrix2::new([
                Complex::ZERO,
                Complex::ONE,
                Complex::ONE,
                Complex::ZERO,
            ]),
        );
    }

    Ok(failed)
}

// =============================================================================
// Noise injection
// =============================================================================

fn apply_gate_noise(
    state: &mut [Complex],
    qubit_count: usize,
    gate: &Gate,
    noise: GateNoise,
    _epsilon: f64,
    rng: &mut DeterministicRng,
) -> Result<u64, EmulationError> {
    let mut errors = 0u64;

    for qubit in gate.qubits() {
        let probability = rng.next_f64();

        if probability < noise.x_error {
            apply_pauli_x(
                state,
                qubit_count,
                qubit.index(),
            );

            errors = errors.saturating_add(1);
            continue;
        }

        if probability
            < noise.x_error + noise.y_error
        {
            apply_pauli_y(
                state,
                qubit_count,
                qubit.index(),
            );

            errors = errors.saturating_add(1);
            continue;
        }

        if probability
            < noise.x_error
                + noise.y_error
                + noise.z_error
        {
            apply_pauli_z(
                state,
                qubit_count,
                qubit.index(),
            );

            errors = errors.saturating_add(1);
            continue;
        }

        if rng.next_f64() < noise.depolarizing_error {
            match (rng.next_u64() % 3) as u8 {
                0 => apply_pauli_x(
                    state,
                    qubit_count,
                    qubit.index(),
                ),
                1 => apply_pauli_y(
                    state,
                    qubit_count,
                    qubit.index(),
                ),
                _ => apply_pauli_z(
                    state,
                    qubit_count,
                    qubit.index(),
                ),
            }

            errors = errors.saturating_add(1);
        }
    }

    Ok(errors)
}

fn apply_pauli_x(
    state: &mut [Complex],
    qubit_count: usize,
    qubit: usize,
) {
    apply_single(
        state,
        qubit_count,
        qubit,
        Matrix2::new([
            Complex::ZERO,
            Complex::ONE,
            Complex::ONE,
            Complex::ZERO,
        ]),
    );
}

fn apply_pauli_y(
    state: &mut [Complex],
    qubit_count: usize,
    qubit: usize,
) {
    apply_single(
        state,
        qubit_count,
        qubit,
        Matrix2::new([
            Complex::ZERO,
            Complex::new(0.0, -1.0),
            Complex::I,
            Complex::ZERO,
        ]),
    );
}

fn apply_pauli_z(
    state: &mut [Complex],
    qubit_count: usize,
    qubit: usize,
) {
    apply_single(
        state,
        qubit_count,
        qubit,
        Matrix2::new([
            Complex::ONE,
            Complex::ZERO,
            Complex::ZERO,
            Complex::new(-1.0, 0.0),
        ]),
    );
}

fn apply_readout_noise(
    value: u8,
    noise: ReadoutNoise,
    rng: &mut DeterministicRng,
) -> u8 {
    match value {
        0 if rng.next_f64() < noise.zero_to_one => 1,
        1 if rng.next_f64() < noise.one_to_zero => 0,
        _ => value,
    }
}

fn apply_readout_noise_to_bitstring(
    bitstring: &mut String,
    noise: ReadoutNoise,
    rng: &mut DeterministicRng,
    error_counter: &mut u64,
) {
    let mut output = String::with_capacity(bitstring.len());

    for character in bitstring.chars() {
        let bit = match character {
            '0' => {
                if rng.next_f64() < noise.zero_to_one {
                    *error_counter =
                        error_counter.saturating_add(1);
                    '1'
                } else {
                    '0'
                }
            }

            '1' => {
                if rng.next_f64() < noise.one_to_zero {
                    *error_counter =
                        error_counter.saturating_add(1);
                    '0'
                } else {
                    '1'
                }
            }

            other => other,
        };

        output.push(bit);
    }

    *bitstring = output;
}

// =============================================================================
// Helpers
// =============================================================================

fn parameter(
    gate: &Gate,
    index: usize,
) -> Result<f64, EmulationError> {
    let parameters = gate
        .constant_parameters()
        .ok_or(EmulationError::NonConstantParameter {
            gate: gate_name(gate.kind()),
        })?;

    let value =
        parameters
            .get(index)
            .copied()
            .ok_or(EmulationError::InvalidParameter {
                gate: gate_name(gate.kind()),
            })?;

    if !value.is_finite() {
        return Err(EmulationError::InvalidParameter {
            gate: gate_name(gate.kind()),
        });
    }

    Ok(value)
}

fn has_measurement(gates: &[Gate]) -> bool {
    gates.iter().any(Gate::is_measurement)
}

fn format_bitstring(
    value: usize,
    qubit_count: usize,
) -> String {
    let mut output =
        String::with_capacity(qubit_count);

    for qubit in (0..qubit_count).rev() {
        if value & (1usize << qubit) != 0 {
            output.push('1');
        } else {
            output.push('0');
        }
    }

    output
}

fn format_measurement(
    value: u8,
    classical: usize,
    qubit_count: usize,
) -> String {
    let mut bits =
        vec!['0'; qubit_count.max(classical + 1)];

    let position =
        bits.len().saturating_sub(classical + 1);

    bits[position] =
        if value == 0 { '0' } else { '1' };

    bits.into_iter().collect()
}

fn validate_probability(
    value: f64,
) -> Result<(), EmulationError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(EmulationError::InvalidNoiseProbability {
            value,
        });
    }

    Ok(())
}

fn gate_name(kind: GateKind) -> &'static str {
    match kind {
        GateKind::I => "i",
        GateKind::X => "x",
        GateKind::Y => "y",
        GateKind::Z => "z",
        GateKind::H => "h",
        GateKind::S => "s",
        GateKind::Sdg => "sdg",
        GateKind::T => "t",
        GateKind::Tdg => "tdg",
        GateKind::V => "v",
        GateKind::Vdg => "vdg",
        GateKind::RX => "rx",
        GateKind::RY => "ry",
        GateKind::RZ => "rz",
        GateKind::Phase => "phase",
        GateKind::U1 => "u1",
        GateKind::U2 => "u2",
        GateKind::U3 => "u3",
        GateKind::CX => "cx",
        GateKind::CY => "cy",
        GateKind::CZ => "cz",
        GateKind::CH => "ch",
        GateKind::SWAP => "swap",
        GateKind::ISWAP => "iswap",
        GateKind::ECR => "ecr",
        GateKind::CRX => "crx",
        GateKind::CRY => "cry",
        GateKind::CRZ => "crz",
        GateKind::CCX => "ccx",
        GateKind::CSWAP => "cswap",
        GateKind::Measure => "measure",
        GateKind::Barrier => "barrier",
        GateKind::Reset => "reset",
    }
}

fn split_seed(seed: u64, shot: u64) -> u64 {
    let mut value =
        seed ^ shot.wrapping_mul(0x9E37_79B9_7F4A_7C15);

    value ^= value >> 30;
    value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^= value >> 31;

    value
}

// =============================================================================
// Deterministic RNG
// =============================================================================

/// Private deterministic pseudo-random generator.
///
/// This is deliberately not cryptographic randomness. It exists only to make
/// physical-noise emulation reproducible.
#[derive(Debug, Clone, Copy)]
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0xA5A5_A5A5_A5A5_A5A5
            } else {
                seed
            },
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut value = self.state;

        value ^= value >> 12;
        value ^= value << 25;
        value ^= value >> 27;

        self.state = value;

        value.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn next_f64(&mut self) -> f64 {
        let value = self.next_u64() >> 11;

        value as f64
            / ((1u64 << 53) as f64)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::ir::gate::GateKind;
    use super::super::super::ir::qubit::QubitId;

    fn gate(
        kind: GateKind,
        qubits: &[usize],
        parameters: &[f64],
    ) -> Gate {
        Gate::new(
            kind,
            qubits
                .iter()
                .copied()
                .map(QubitId::new)
                .collect(),
            parameters
                .iter()
                .copied()
                .map(super::super::super::ir::parameter::Parameter::Constant)
                .collect(),
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    #[test]
    fn production_configuration_is_valid() {
        let config =
            EmulatorConfig::production(4)
                .expect("configuration should be valid");

        assert!(config.validate().is_ok());
    }

    #[test]
    fn ideal_single_qubit_x_is_deterministic() {
        let emulator =
            HardwareEmulator::new(
                EmulatorConfig::production(1)
                    .unwrap(),
            )
            .unwrap();

        let input =
            EmulationInput::new(
                1,
                vec![
                    gate(GateKind::X, &[0], &[]),
                ],
                32,
            )
            .with_seed(42);

        let first =
            emulator.emulate(&input).unwrap();

        let second =
            emulator.emulate(&input).unwrap();

        assert_eq!(first.counts, second.counts);
        assert_eq!(
            first.counts.get("1").copied(),
            Some(32)
        );
    }

    #[test]
    fn hadamard_has_two_possible_measurements() {
        let emulator =
            HardwareEmulator::new(
                EmulatorConfig::production(1)
                    .unwrap(),
            )
            .unwrap();

        let input =
            EmulationInput::new(
                1,
                vec![
                    gate(GateKind::H, &[0], &[]),
                ],
                2_048,
            )
            .with_seed(7);

        let result =
            emulator.emulate(&input).unwrap();

        assert_eq!(
            result.counts.values().sum::<u64>(),
            2_048
        );

        assert!(
            result.counts.contains_key("0")
        );

        assert!(
            result.counts.contains_key("1")
        );
    }

    #[test]
    fn x_then_x_returns_zero() {
        let emulator =
            HardwareEmulator::new(
                EmulatorConfig::production(1)
                    .unwrap(),
            )
            .unwrap();

        let input =
            EmulationInput::new(
                1,
                vec![
                    gate(GateKind::X, &[0], &[]),
                    gate(GateKind::X, &[0], &[]),
                ],
                32,
            )
            .with_seed(99);

        let result =
            emulator.emulate(&input).unwrap();

        assert_eq!(
            result.counts.get("0").copied(),
            Some(32)
        );
    }

    #[test]
    fn cnot_respects_connectivity() {
        let mut config =
            EmulatorConfig::production(3)
                .unwrap();

        config.connectivity =
            Connectivity::new(
                3,
                [(0, 1)],
            )
            .unwrap();

        let emulator =
            HardwareEmulator::new(config)
                .unwrap();

        let input =
            EmulationInput::new(
                3,
                vec![
                    gate(GateKind::CX, &[0, 2], &[]),
                ],
                1,
            );

        assert!(matches!(
            emulator.validate(&input),
            Err(
                EmulationError::UnsupportedConnectivity { .. }
            )
        ));
    }

    #[test]
    fn readout_noise_is_applied() {
        let mut config =
            EmulatorConfig::production(1)
                .unwrap();

        config.noise.readout =
            ReadoutNoise {
                zero_to_one: 1.0,
                one_to_zero: 0.0,
            };

        let emulator =
            HardwareEmulator::new(config)
                .unwrap();

        let input =
            EmulationInput::new(
                1,
                vec![
                    gate(GateKind::I, &[0], &[]),
                ],
                8,
            )
            .with_seed(123);

        let result =
            emulator.emulate(&input).unwrap();

        assert_eq!(
            result.counts.get("1").copied(),
            Some(8)
        );

        assert_eq!(
            result.injected_readout_errors,
            8
        );
    }

    #[test]
    fn explicit_seed_reproduces_noise() {
        let mut config =
            EmulatorConfig::production(1)
                .unwrap();

        config.noise.single_qubit =
            GateNoise {
                x_error: 0.2,
                y_error: 0.1,
                z_error: 0.1,
                depolarizing_error: 0.1,
            };

        let emulator =
            HardwareEmulator::new(config)
                .unwrap();

        let input =
            EmulationInput::new(
                1,
                vec![
                    gate(GateKind::H, &[0], &[]),
                ],
                512,
            )
            .with_seed(123_456);

        let first =
            emulator.emulate(&input).unwrap();

        let second =
            emulator.emulate(&input).unwrap();

        assert_eq!(
            first.counts,
            second.counts
        );

        assert_eq!(
            first.injected_gate_errors,
            second.injected_gate_errors
        );
    }

    #[test]
    fn operation_limit_is_enforced_before_execution() {
        let mut config =
            EmulatorConfig::production(1)
                .unwrap();

        config.limits =
            EmulatorLimits::new(
                1,
                2,
                1,
                10,
            );

        let emulator =
            HardwareEmulator::new(config)
                .unwrap();

        let input =
            EmulationInput::new(
                1,
                vec![
                    gate(GateKind::I, &[0], &[]),
                    gate(GateKind::I, &[0], &[]),
                ],
                1,
            );

        assert!(matches!(
            emulator.validate(&input),
            Err(
                EmulationError::OperationLimitExceeded { .. }
            )
        ));
    }

    #[test]
    fn reset_returns_zero_without_noise() {
        let emulator =
            HardwareEmulator::new(
                EmulatorConfig::production(1)
                    .unwrap(),
            )
            .unwrap();

        let input =
            EmulationInput::new(
                1,
                vec![
                    gate(GateKind::X, &[0], &[]),
                    gate(GateKind::Reset, &[0], &[]),
                ],
                64,
            )
            .with_seed(123);

        let result =
            emulator.emulate(&input).unwrap();

        assert_eq!(
            result.counts.get("0").copied(),
            Some(64)
        );
    }

    #[test]
    fn all_public_configuration_noise_types_validate() {
        let gate_noise =
            GateNoise::depolarizing(0.3)
                .unwrap();

        let readout =
            ReadoutNoise::symmetric(0.02)
                .unwrap();

        let reset =
            ResetNoise::new(0.01)
                .unwrap();

        let model =
            NoiseModel {
                single_qubit: gate_noise,
                multi_qubit: gate_noise,
                readout,
                reset,
            };

        assert!(model.validate().is_ok());
    }
}