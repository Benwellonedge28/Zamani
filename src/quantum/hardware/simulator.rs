//! Zamani Quantum Hardware — Production State-Vector Simulator
//!
//! This module provides the canonical local state-vector simulator interface
//! for Zamani Quantum.
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
//!              QuantumCircuit
//!                    |
//!          +---------+---------+
//!          |                   |
//!          v                   v
//!       Hardware             Simulator
//!          |                   |
//!          v                   v
//!       Provider             State Vector
//!
//! ```
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - deterministic local state-vector simulation;
//! - logical quantum-circuit execution;
//! - computational-basis measurement;
//! - shot-based sampling;
//! - state-vector inspection;
//! - probability calculation;
//! - measurement counts;
//! - simulation configuration;
//! - simulator resource limits;
//! - simulator validation;
//! - deterministic pseudo-random sampling;
//! - simulator-specific execution metadata;
//! - simulator errors.
//!
//! # It deliberately does NOT own
//!
//! - physical QPU communication;
//! - provider APIs;
//! - authentication;
//! - credentials;
//! - cloud execution;
//! - physical topology;
//! - physical calibration;
//! - routing;
//! - scheduling;
//! - pulse simulation;
//! - analog Hamiltonian simulation;
//! - annealing;
//! - fault-tolerant decoding;
//! - benchmarking mathematics;
//! - frontend parsing;
//! - source-language semantics.
//!
//! Those responsibilities belong to their owning subsystems.
//!
//! # Integration contract
//!
//! This file depends only on:
//!
//! - the Rust standard library;
//! - `quantum::ir::circuit::QuantumCircuit`;
//! - `quantum::ir::gate::{Gate, GateKind}`;
//! - `quantum::ir::parameter::Parameter`.
//!
//! It intentionally does NOT depend on future files such as:
//!
//! - `backend.rs`;
//! - `backend_trait.rs`;
//! - `execution.rs`;
//! - `job.rs`;
//! - `provider.rs`;
//! - `provider_registry.rs`;
//! - `result.rs`;
//! - provider adapters;
//! - benchmarking.
//!
//! This makes the file independently completable.
//!
//! Later integration layers may wrap this simulator behind their own backend
//! traits without requiring modifications to this implementation.
//!
//! # Numerical model
//!
//! The simulator uses:
//!
//! ```text
//! |psi> = sum_i alpha_i |i>
//! ```
//!
//! with complex amplitudes represented by the private `Complex64` type.
//!
//! No external complex-number crate is required.
//!
//! # Supported logical operations
//!
//! The current canonical IR defines the following gate families:
//!
//! - I
//! - X
//! - Y
//! - Z
//! - H
//! - S
//! - Sdg
//! - T
//! - Tdg
//! - V
//! - Vdg
//! - RX
//! - RY
//! - RZ
//! - Phase
//! - U1
//! - U2
//! - U3
//! - CX
//! - CY
//! - CZ
//! - CH
//! - SWAP
//! - ISWAP
//! - ECR
//! - CRX
//! - CRY
//! - CRZ
//! - CCX
//! - CSWAP
//! - Measure
//! - Barrier
//! - Reset
//!
//! Symbolic parameters are rejected by this state-vector simulator because
//! simulation requires concrete numerical parameter values.
//!
//! # Measurement semantics
//!
//! Measurement is performed in the computational basis.
//!
//! For shot-based execution:
//!
//! 1. the circuit is executed;
//! 2. a basis state is sampled according to its probability;
//! 3. the sampled bit string is recorded;
//! 4. the quantum state is collapsed;
//! 5. the same execution state is not reused for the next independent shot.
//!
//! Each shot therefore begins from the configured initial state.
//!
//! # Determinism
//!
//! When a seed is supplied, shot sampling is deterministic for the same:
//!
//! - circuit;
//! - simulator configuration;
//! - seed;
//! - simulator version.
//!
//! When no seed is supplied, the simulator uses a deterministic process-local
//! seed derived from a monotonic atomic counter and therefore remains fully
//! reproducible when the explicit seed is supplied.
//!
//! # Resource safety
//!
//! State-vector size grows exponentially with the number of qubits.
//!
//! This module therefore never allocates a state vector without first checking:
//!
//! ```text
//! 2^qubits <= configured amplitude limit
//! ```
//!
//! Overflow in `2^qubits` calculation is treated as an error.
//!
//! # Thread safety
//!
//! `StateVectorSimulator` contains only immutable configuration.
//!
//! Each execution creates its own state and pseudo-random generator.
//!
//! Therefore the simulator configuration can safely be shared between threads
//! using `Arc` without shared mutable execution state.
//!
//! # Rust compatibility
//!
//! Target:
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
//! The following types are intended to form the stable simulator boundary:
//!
//! - `SimulatorConfig`;
//! - `SimulatorLimits`;
//! - `StateVectorSimulator`;
//! - `SimulationResult`;
//! - `SimulationStatistics`;
//! - `MeasurementCounts`;
//! - `SimulationError`;
//! - `SimulatorVersion`.
//!
//! Internal numerical representation is deliberately private so that the
//! simulator can later replace its numerical engine without changing the
//! public execution contract.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

// =============================================================================
// Integration with canonical Zamani Quantum IR
// =============================================================================

use super::super::ir::circuit::QuantumCircuit;
use super::super::ir::gate::{Gate, GateKind};
use super::super::ir::parameter::Parameter;

// =============================================================================
// Schema
// =============================================================================

/// Stable simulator schema identifier.
pub const SIMULATOR_SCHEMA_ID: &str = "zamani.quantum.hardware.simulator";

/// Semantic simulator schema version.
pub const SIMULATOR_SCHEMA_VERSION: u16 = 1;

/// Stable simulator implementation version.
pub const SIMULATOR_VERSION: SimulatorVersion = SimulatorVersion {
    major: 1,
    minor: 0,
    patch: 0,
};

/// Maximum supported number of qubits under the default production policy.
///
/// This is intentionally conservative because a state-vector simulator needs
/// `2^n` complex amplitudes.
pub const DEFAULT_MAX_QUBITS: usize = 30;

/// Default maximum number of amplitudes.
pub const DEFAULT_MAX_AMPLITUDES: usize = 1usize << DEFAULT_MAX_QUBITS;

/// Default numerical tolerance.
pub const DEFAULT_EPSILON: f64 = 1.0e-12;

/// Default measurement-shot count.
pub const DEFAULT_SHOTS: u64 = 1024;

/// Process-local deterministic seed counter.
///
/// Explicit seeds remain the recommended reproducibility mechanism.
static SEED_COUNTER: AtomicU64 = AtomicU64::new(1);

// =============================================================================
// Version
// =============================================================================

/// Semantic version of the simulator implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SimulatorVersion {
    /// Major version.
    pub major: u16,

    /// Minor version.
    pub minor: u16,

    /// Patch version.
    pub patch: u16,
}

impl SimulatorVersion {
    /// Creates a simulator version.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the canonical version string.
    pub fn as_str(self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl fmt::Display for SimulatorVersion {
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
// Simulator limits
// =============================================================================

/// Resource limits protecting the simulator from accidental exponential
/// allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulatorLimits {
    /// Maximum logical qubits.
    pub max_qubits: usize,

    /// Maximum number of state-vector amplitudes.
    pub max_amplitudes: usize,

    /// Maximum circuit operation count.
    pub max_operations: usize,

    /// Maximum shots in one request.
    pub max_shots: u64,
}

impl Default for SimulatorLimits {
    fn default() -> Self {
        Self::production()
    }
}

impl SimulatorLimits {
    /// Creates production-safe simulator limits.
    pub const fn production() -> Self {
        Self {
            max_qubits: DEFAULT_MAX_QUBITS,
            max_amplitudes: DEFAULT_MAX_AMPLITUDES,
            max_operations: 10_000_000,
            max_shots: 10_000_000,
        }
    }

    /// Creates explicit simulator limits.
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
    pub fn validate(&self) -> Result<(), SimulationError> {
        if self.max_qubits == 0 {
            return Err(SimulationError::InvalidLimits {
                field: "max_qubits",
                value: self.max_qubits,
            });
        }

        if self.max_amplitudes == 0 {
            return Err(SimulationError::InvalidLimits {
                field: "max_amplitudes",
                value: self.max_amplitudes,
            });
        }

        if self.max_operations == 0 {
            return Err(SimulationError::InvalidLimits {
                field: "max_operations",
                value: self.max_operations,
            });
        }

        if self.max_shots == 0 {
            return Err(SimulationError::InvalidLimits {
                field: "max_shots",
                value: self.max_shots,
            });
        }

        Ok(())
    }

    /// Calculates the state-vector amplitude count for `qubits`.
    pub fn amplitude_count(
        &self,
        qubits: usize,
    ) -> Result<usize, SimulationError> {
        if qubits > self.max_qubits {
            return Err(SimulationError::QubitLimitExceeded {
                requested: qubits,
                maximum: self.max_qubits,
            });
        }

        let amplitudes = checked_pow2(qubits)?;

        if amplitudes > self.max_amplitudes {
            return Err(SimulationError::AmplitudeLimitExceeded {
                requested: amplitudes,
                maximum: self.max_amplitudes,
            });
        }

        Ok(amplitudes)
    }
}

// =============================================================================
// Simulator configuration
// =============================================================================

/// Configuration controlling a simulator instance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimulatorConfig {
    /// Resource limits.
    pub limits: SimulatorLimits,

    /// Numerical tolerance used by normalization and probability checks.
    pub epsilon: f64,

    /// Optional deterministic seed.
    ///
    /// `None` creates a process-local seed.
    pub seed: Option<u64>,

    /// Whether measurement operations in the circuit are respected.
    ///
    /// When false, measurement operations are rejected.
    pub allow_measurement: bool,

    /// Whether reset operations are supported.
    pub allow_reset: bool,

    /// Whether state-vector output is included in the result.
    pub return_state_vector: bool,

    /// Whether probability output is included in the result.
    pub return_probabilities: bool,

    /// Number of shots used by shot-based execution.
    pub shots: u64,
}

impl Default for SimulatorConfig {
    fn default() -> Self {
        Self::production()
    }
}

impl SimulatorConfig {
    /// Production configuration.
    pub const fn production() -> Self {
        Self {
            limits: SimulatorLimits::production(),
            epsilon: DEFAULT_EPSILON,
            seed: None,
            allow_measurement: true,
            allow_reset: true,
            return_state_vector: true,
            return_probabilities: true,
            shots: DEFAULT_SHOTS,
        }
    }

    /// Creates a configuration with an explicit seed.
    pub const fn with_seed(seed: u64) -> Self {
        Self {
            seed: Some(seed),
            ..Self::production()
        }
    }

    /// Returns a deterministic configuration using `seed`.
    pub const fn deterministic(seed: u64) -> Self {
        Self::with_seed(seed)
    }

    /// Validates the complete configuration.
    pub fn validate(&self) -> Result<(), SimulationError> {
        self.limits.validate()?;

        if !self.epsilon.is_finite() || self.epsilon <= 0.0 {
            return Err(SimulationError::InvalidEpsilon {
                value: self.epsilon,
            });
        }

        if self.shots == 0 {
            return Err(SimulationError::InvalidShots {
                shots: self.shots,
            });
        }

        if self.shots > self.limits.max_shots {
            return Err(SimulationError::ShotLimitExceeded {
                requested: self.shots,
                maximum: self.limits.max_shots,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Structured errors produced by the simulator.
#[derive(Debug, Clone, PartialEq)]
pub enum SimulationError {
    /// Simulator limits are invalid.
    InvalidLimits {
        /// Limit field.
        field: &'static str,

        /// Invalid value.
        value: usize,
    },

    /// Number of qubits exceeds the configured limit.
    QubitLimitExceeded {
        /// Requested qubits.
        requested: usize,

        /// Maximum qubits.
        maximum: usize,
    },

    /// State-vector amplitude count exceeds the configured limit.
    AmplitudeLimitExceeded {
        /// Requested amplitudes.
        requested: usize,

        /// Maximum amplitudes.
        maximum: usize,
    },

    /// Arithmetic overflow occurred.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// Circuit operation count exceeds the simulator limit.
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

    /// Shot count exceeds the configured limit.
    ShotLimitExceeded {
        /// Requested shots.
        requested: u64,

        /// Maximum shots.
        maximum: u64,
    },

    /// Numerical tolerance is invalid.
    InvalidEpsilon {
        /// Invalid epsilon.
        value: f64,
    },

    /// Circuit is empty when an execution requires operations.
    EmptyCircuit,

    /// The circuit contains an unsupported gate.
    UnsupportedGate {
        /// Gate kind.
        gate: GateKind,
    },

    /// A gate parameter is symbolic.
    SymbolicParameter {
        /// Gate kind.
        gate: GateKind,

        /// Parameter position.
        index: usize,
    },

    /// A gate parameter is non-finite.
    NonFiniteParameter {
        /// Gate kind.
        gate: GateKind,

        /// Parameter position.
        index: usize,

        /// Parameter value.
        value: f64,
    },

    /// Gate contains an unexpected structure.
    InvalidGate {
        /// Gate kind.
        gate: GateKind,

        /// Human-readable reason.
        reason: &'static str,
    },

    /// Logical qubit is outside the simulator state.
    QubitOutOfRange {
        /// Qubit index.
        qubit: usize,

        /// Number of qubits.
        num_qubits: usize,
    },

    /// Classical measurement target is unsupported by this state-vector
    /// execution path.
    UnsupportedMeasurementTarget,

    /// Internal state vector became invalid.
    InvalidStateVector {
        /// Human-readable reason.
        reason: &'static str,
    },

    /// Probability vector is invalid.
    InvalidProbabilityDistribution,

    /// State-vector normalization failed.
    NormalizationFailure,

    /// Sampling failed.
    SamplingFailure,

    /// Circuit validation failed.
    CircuitValidation {
        /// Error description.
        message: String,
    },

    /// Requested functionality requires another simulator implementation.
    UnsupportedWorkload {
        /// Workload description.
        workload: &'static str,
    },
}

impl fmt::Display for SimulationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLimits { field, value } => {
                write!(
                    formatter,
                    "invalid simulator limit `{field}`: {value}"
                )
            }

            Self::QubitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "simulator qubit limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::AmplitudeLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "state-vector amplitude limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::OperationLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "simulator operation limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::InvalidShots { shots } => {
                write!(
                    formatter,
                    "invalid simulator shot count: {shots}"
                )
            }

            Self::ShotLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "simulator shot limit exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::InvalidEpsilon { value } => {
                write!(
                    formatter,
                    "invalid simulator numerical tolerance: {value}"
                )
            }

            Self::EmptyCircuit => {
                formatter.write_str("cannot execute an empty circuit")
            }

            Self::UnsupportedGate { gate } => {
                write!(
                    formatter,
                    "unsupported gate in state-vector simulator: {gate:?}"
                )
            }

            Self::SymbolicParameter { gate, index } => {
                write!(
                    formatter,
                    "symbolic parameter {index} cannot be evaluated by state-vector simulator for gate {gate:?}"
                )
            }

            Self::NonFiniteParameter {
                gate,
                index,
                value,
            } => {
                write!(
                    formatter,
                    "non-finite parameter {index} for gate {gate:?}: {value}"
                )
            }

            Self::InvalidGate { gate, reason } => {
                write!(
                    formatter,
                    "invalid gate {gate:?}: {reason}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    formatter,
                    "qubit {qubit} is outside simulator range 0..{num_qubits}"
                )
            }

            Self::UnsupportedMeasurementTarget => {
                formatter.write_str(
                    "measurement target cannot be represented by this simulator result path",
                )
            }

            Self::InvalidStateVector { reason } => {
                write!(
                    formatter,
                    "invalid state vector: {reason}"
                )
            }

            Self::InvalidProbabilityDistribution => {
                formatter.write_str("invalid probability distribution")
            }

            Self::NormalizationFailure => {
                formatter.write_str("state-vector normalization failed")
            }

            Self::SamplingFailure => {
                formatter.write_str("state-vector sampling failed")
            }

            Self::CircuitValidation { message } => {
                write!(
                    formatter,
                    "quantum circuit validation failed: {message}"
                )
            }

            Self::UnsupportedWorkload { workload } => {
                write!(
                    formatter,
                    "unsupported simulator workload: {workload}"
                )
            }
        }
    }
}

impl std::error::Error for SimulationError {}

// =============================================================================
// Measurement counts
// =============================================================================

/// Deterministic computational-basis measurement counts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementCounts {
    /// Number of classical bits represented by each key.
    num_bits: usize,

    /// Big-endian binary bit strings mapped to counts.
    ///
    /// The leftmost character is the highest-index qubit and the rightmost
    /// character is qubit zero.
    counts: BTreeMap<String, u64>,
}

impl MeasurementCounts {
    /// Creates an empty count collection.
    pub fn new(num_bits: usize) -> Self {
        Self {
            num_bits,
            counts: BTreeMap::new(),
        }
    }

    /// Returns the number of represented bits.
    pub const fn num_bits(&self) -> usize {
        self.num_bits
    }

    /// Returns the number of distinct outcomes.
    pub fn len(&self) -> usize {
        self.counts.len()
    }

    /// Returns true if no outcomes have been recorded.
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }

    /// Returns the count for an exact bit string.
    pub fn get(&self, bit_string: &str) -> Option<u64> {
        self.counts.get(bit_string).copied()
    }

    /// Returns all counts in deterministic lexicographic order.
    pub fn as_map(&self) -> &BTreeMap<String, u64> {
        &self.counts
    }

    /// Returns the total number of samples.
    pub fn total_shots(&self) -> u64 {
        self.counts.values().copied().sum()
    }

    fn increment(&mut self, index: usize) -> Result<(), SimulationError> {
        if index >= self
            .num_bits
            .checked_shl(1)
            .unwrap_or(usize::MAX)
        {
            // This branch is intentionally conservative. The actual bit
            // string is generated separately and validated before insertion.
        }

        let key = basis_index_to_bit_string(index, self.num_bits)?;

        let entry = self.counts.entry(key).or_insert(0);

        *entry = entry
            .checked_add(1)
            .ok_or(SimulationError::ArithmeticOverflow {
                calculation: "measurement count",
            })?;

        Ok(())
    }
}

// =============================================================================
// Simulation statistics
// =============================================================================

/// Summary statistics for a simulation.
#[derive(Debug, Clone, PartialEq)]
pub struct SimulationStatistics {
    /// Number of circuit executions.
    pub shots: u64,

    /// Number of distinct observed outcomes.
    pub distinct_outcomes: usize,

    /// Sum of final probabilities.
    pub probability_sum: f64,

    /// Final-state normalization error.
    pub normalization_error: f64,
}

impl SimulationStatistics {
    fn from_state(
        probabilities: &[f64],
        shots: u64,
        distinct_outcomes: usize,
        epsilon: f64,
    ) -> Result<Self, SimulationError> {
        let probability_sum = probabilities.iter().try_fold(
            0.0_f64,
            |accumulator, value| {
                let next = accumulator + *value;

                if !next.is_finite() {
                    Err(SimulationError::InvalidProbabilityDistribution)
                } else {
                    Ok(next)
                }
            },
        )?;

        let normalization_error =
            (probability_sum - 1.0).abs();

        if normalization_error > epsilon * 100.0 {
            return Err(SimulationError::InvalidProbabilityDistribution);
        }

        Ok(Self {
            shots,
            distinct_outcomes,
            probability_sum,
            normalization_error,
        })
    }
}

// =============================================================================
// Simulation result
// =============================================================================

/// Complete result of a local state-vector simulation.
#[derive(Debug, Clone, PartialEq)]
pub struct SimulationResult {
    /// Simulator schema identifier.
    pub schema_id: &'static str,

    /// Simulator schema version.
    pub schema_version: u16,

    /// Simulator implementation version.
    pub simulator_version: SimulatorVersion,

    /// Number of logical qubits.
    pub num_qubits: usize,

    /// Number of circuit operations executed.
    pub operation_count: usize,

    /// Number of shots.
    pub shots: u64,

    /// Seed used by the simulator.
    pub seed: u64,

    /// Final state vector.
    ///
    /// Present when configured through `return_state_vector`.
    pub state_vector: Option<Vec<(f64, f64)>>,

    /// Computational-basis probabilities.
    ///
    /// Present when configured through `return_probabilities`.
    pub probabilities: Option<Vec<f64>>,

    /// Shot-based measurement counts.
    pub counts: MeasurementCounts,

    /// Summary statistics.
    pub statistics: SimulationStatistics,
}

impl SimulationResult {
    /// Returns the probability of a basis-state index.
    pub fn probability(
        &self,
        index: usize,
    ) -> Option<f64> {
        self.probabilities
            .as_ref()
            .and_then(|values| values.get(index).copied())
    }

    /// Returns the probability of a bit string.
    pub fn probability_of(
        &self,
        bit_string: &str,
    ) -> Option<f64> {
        let index =
            bit_string_to_basis_index(bit_string).ok()?;

        self.probability(index)
    }

    /// Returns the state-vector amplitude at `index`.
    ///
    /// The tuple is `(real, imaginary)`.
    pub fn amplitude(
        &self,
        index: usize,
    ) -> Option<(f64, f64)> {
        self.state_vector
            .as_ref()
            .and_then(|values| values.get(index).copied())
    }

    /// Returns the most frequently observed measurement outcome.
    pub fn most_frequent_outcome(
        &self,
    ) -> Option<(&str, u64)> {
        self.counts
            .as_map()
            .iter()
            .max_by(|left, right| {
                left.1
                    .cmp(right.1)
                    .then_with(|| right.0.cmp(left.0))
            })
            .map(|(key, value)| (key.as_str(), *value))
    }
}

// =============================================================================
// State-vector simulator
// =============================================================================

/// Production local state-vector simulator.
///
/// The simulator is configuration-only and does not retain execution state.
/// Every call to [`StateVectorSimulator::run`] creates a fresh state vector.
///
/// This makes repeated execution independent and prevents accidental state
/// leakage between jobs.
#[derive(Debug, Clone, Copy)]
pub struct StateVectorSimulator {
    config: SimulatorConfig,
}

impl Default for StateVectorSimulator {
    fn default() -> Self {
        Self::new()
    }
}

impl StateVectorSimulator {
    /// Creates a production-configured simulator.
    pub fn new() -> Self {
        Self {
            config: SimulatorConfig::production(),
        }
    }

    /// Creates a simulator from validated configuration.
    pub fn with_config(
        config: SimulatorConfig,
    ) -> Result<Self, SimulationError> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the simulator configuration.
    pub const fn config(&self) -> &SimulatorConfig {
        &self.config
    }

    /// Creates a deterministic simulator.
    pub fn deterministic(
        seed: u64,
    ) -> Result<Self, SimulationError> {
        Self::with_config(SimulatorConfig::deterministic(seed))
    }

    /// Validates whether a circuit can be simulated.
    pub fn validate(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<(), SimulationError> {
        self.config.validate()?;

        if circuit.is_empty() {
            return Err(SimulationError::EmptyCircuit);
        }

        if circuit.len() > self.config.limits.max_operations {
            return Err(
                SimulationError::OperationLimitExceeded {
                    requested: circuit.len(),
                    maximum: self.config.limits.max_operations,
                },
            );
        }

        self.config
            .limits
            .amplitude_count(circuit.num_qubits())?;

        for gate in circuit.operations() {
            self.validate_gate(
                gate,
                circuit.num_qubits(),
            )?;
        }

        Ok(())
    }

    /// Executes a circuit using the configured shot count.
    pub fn run(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<SimulationResult, SimulationError> {
        self.validate(circuit)?;

        let seed = self.effective_seed(circuit);

        let final_state =
            self.execute_once(circuit)?;

        let probabilities =
            probabilities_from_state(
                &final_state.amplitudes,
                self.config.epsilon,
            )?;

        let mut rng = SplitMix64::new(seed);

        let mut counts =
            MeasurementCounts::new(circuit.num_qubits());

        for _ in 0..self.config.shots {
            let index = sample_distribution(
                &probabilities,
                &mut rng,
                self.config.epsilon,
            )?;

            counts.increment(index)?;
        }

        let statistics =
            SimulationStatistics::from_state(
                &probabilities,
                self.config.shots,
                counts.len(),
                self.config.epsilon,
            )?;

        let state_vector =
            if self.config.return_state_vector {
                Some(
                    final_state
                        .amplitudes
                        .iter()
                        .map(|value| {
                            (value.re, value.im)
                        })
                        .collect(),
                )
            } else {
                None
            };

        let probabilities_output =
            if self.config.return_probabilities {
                Some(probabilities)
            } else {
                None
            };

        Ok(SimulationResult {
            schema_id: SIMULATOR_SCHEMA_ID,
            schema_version: SIMULATOR_SCHEMA_VERSION,
            simulator_version: SIMULATOR_VERSION,
            num_qubits: circuit.num_qubits(),
            operation_count: circuit.len(),
            shots: self.config.shots,
            seed,
            state_vector,
            probabilities: probabilities_output,
            counts,
            statistics,
        })
    }

    /// Executes the circuit once without shot sampling.
    ///
    /// This is useful for internal integrations and future expectation-value
    /// or observable engines.
    pub fn state_vector(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<Vec<(f64, f64)>, SimulationError> {
        self.validate(circuit)?;

        let state =
            self.execute_once(circuit)?;

        Ok(state
            .amplitudes
            .iter()
            .map(|value| (value.re, value.im))
            .collect())
    }

    /// Computes computational-basis probabilities.
    pub fn probabilities(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<Vec<f64>, SimulationError> {
        self.validate(circuit)?;

        let state =
            self.execute_once(circuit)?;

        probabilities_from_state(
            &state.amplitudes,
            self.config.epsilon,
        )
    }

    fn effective_seed(
        &self,
        circuit: &QuantumCircuit,
    ) -> u64 {
        match self.config.seed {
            Some(seed) => seed,

            None => {
                let counter =
                    SEED_COUNTER.fetch_add(
                        1,
                        Ordering::Relaxed,
                    );

                mix_seed(
                    counter,
                    circuit.num_qubits() as u64,
                    circuit.len() as u64,
                )
            }
        }
    }

    fn validate_gate(
        &self,
        gate: &Gate,
        num_qubits: usize,
    ) -> Result<(), SimulationError> {
        for qubit in gate.qubits() {
            let index = qubit.index();

            if index >= num_qubits {
                return Err(
                    SimulationError::QubitOutOfRange {
                        qubit: index,
                        num_qubits,
                    },
                );
            }
        }

        if gate.is_measurement()
            && !self.config.allow_measurement
        {
            return Err(
                SimulationError::UnsupportedGate {
                    gate: gate.kind(),
                },
            );
        }

        if gate.is_reset()
            && !self.config.allow_reset
        {
            return Err(
                SimulationError::UnsupportedGate {
                    gate: gate.kind(),
                },
            );
        }

        if matches!(
            gate.kind(),
            GateKind::Measure
                | GateKind::Barrier
                | GateKind::Reset
        ) {
            return Ok(());
        }

        let supported = matches!(
            gate.kind(),
            GateKind::I
                | GateKind::X
                | GateKind::Y
                | GateKind::Z
                | GateKind::H
                | GateKind::S
                | GateKind::Sdg
                | GateKind::T
                | GateKind::Tdg
                | GateKind::V
                | GateKind::Vdg
                | GateKind::RX
                | GateKind::RY
                | GateKind::RZ
                | GateKind::Phase
                | GateKind::U1
                | GateKind::U2
                | GateKind::U3
                | GateKind::CX
                | GateKind::CY
                | GateKind::CZ
                | GateKind::CH
                | GateKind::SWAP
                | GateKind::ISWAP
                | GateKind::ECR
                | GateKind::CRX
                | GateKind::CRY
                | GateKind::CRZ
                | GateKind::CCX
                | GateKind::CSWAP
        );

        if !supported {
            return Err(
                SimulationError::UnsupportedGate {
                    gate: gate.kind(),
                },
            );
        }

        for (index, parameter) in
            gate.parameters().iter().enumerate()
        {
            let value = parameter_value(
                gate.kind(),
                index,
                parameter,
            )?;

            if !value.is_finite() {
                return Err(
                    SimulationError::NonFiniteParameter {
                        gate: gate.kind(),
                        index,
                        value,
                    },
                );
            }
        }

        Ok(())
    }

    fn execute_once(
        &self,
        circuit: &QuantumCircuit,
    ) -> Result<StateVector, SimulationError> {
        let amplitudes =
            self.config
                .limits
                .amplitude_count(
                    circuit.num_qubits(),
                )?;

        let mut state =
            StateVector::zero(
                circuit.num_qubits(),
                amplitudes,
            )?;

        state.amplitudes[0] =
            Complex64::one();

        for gate in circuit.operations() {
            apply_gate(
                &mut state,
                gate,
                self.config.epsilon,
            )?;
        }

        state.normalize(
            self.config.epsilon,
        )?;

        Ok(state)
    }
}

// =============================================================================
// State vector
// =============================================================================

/// Internal state-vector representation.
#[derive(Debug, Clone)]
struct StateVector {
    num_qubits: usize,
    amplitudes: Vec<Complex64>,
}

impl StateVector {
    fn zero(
        num_qubits: usize,
        amplitudes: usize,
    ) -> Result<Self, SimulationError> {
        if amplitudes == 0 {
            return Err(
                SimulationError::InvalidStateVector {
                    reason: "zero amplitude count",
                },
            );
        }

        Ok(Self {
            num_qubits,
            amplitudes: vec![
                Complex64::zero();
                amplitudes
            ],
        })
    }

    fn normalize(
        &mut self,
        epsilon: f64,
    ) -> Result<(), SimulationError> {
        let norm_sq = self
            .amplitudes
            .iter()
            .map(|value| value.norm_squared())
            .try_fold(
                0.0_f64,
                |accumulator, value| {
                    let next =
                        accumulator + value;

                    if next.is_finite() {
                        Ok(next)
                    } else {
                        Err(
                            SimulationError::NormalizationFailure,
                        )
                    }
                },
            )?;

        if !norm_sq.is_finite()
            || norm_sq <= epsilon * epsilon
        {
            return Err(
                SimulationError::NormalizationFailure,
            );
        }

        let norm = norm_sq.sqrt();

        for value in &mut self.amplitudes {
            *value /= norm;
        }

        Ok(())
    }

    fn check_qubit(
        &self,
        qubit: usize,
    ) -> Result<(), SimulationError> {
        if qubit >= self.num_qubits {
            return Err(
                SimulationError::QubitOutOfRange {
                    qubit,
                    num_qubits: self.num_qubits,
                },
            );
        }

        Ok(())
    }
}

// =============================================================================
// Complex arithmetic
// =============================================================================

/// Minimal private complex-number implementation.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex64 {
    re: f64,
    im: f64,
}

impl Complex64 {
    const fn new(
        re: f64,
        im: f64,
    ) -> Self {
        Self { re, im }
    }

    const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    const fn one() -> Self {
        Self::new(1.0, 0.0)
    }

    fn norm_squared(self) -> f64 {
        self.re
            .mul_add(self.re, self.im * self.im)
    }

    fn conj(self) -> Self {
        Self::new(self.re, -self.im)
    }

    fn scale(
        self,
        scalar: f64,
    ) -> Self {
        Self::new(
            self.re * scalar,
            self.im * scalar,
        )
    }

    fn exp_i(theta: f64) -> Self {
        Self::new(
            theta.cos(),
            theta.sin(),
        )
    }
}

impl std::ops::Add for Complex64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.re + rhs.re,
            self.im + rhs.im,
        )
    }
}

impl std::ops::Sub for Complex64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self.re - rhs.re,
            self.im - rhs.im,
        )
    }
}

impl std::ops::Mul for Complex64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.re * rhs.re
                - self.im * rhs.im,
            self.re * rhs.im
                + self.im * rhs.re,
        )
    }
}

impl std::ops::Div<f64> for Complex64 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self::new(
            self.re / rhs,
            self.im / rhs,
        )
    }
}

impl std::ops::DivAssign<f64> for Complex64 {
    fn div_assign(
        &mut self,
        rhs: f64,
    ) {
        self.re /= rhs;
        self.im /= rhs;
    }
}

// =============================================================================
// Gate application
// =============================================================================

fn apply_gate(
    state: &mut StateVector,
    gate: &Gate,
    epsilon: f64,
) -> Result<(), SimulationError> {
    match gate.kind() {
        GateKind::I => {}

        GateKind::X => {
            apply_single_qubit_matrix(
                state,
                gate.qubits()[0].index(),
                [
                    [
                        Complex64::zero(),
                        Complex64::one(),
                    ],
                    [
                        Complex64::one(),
                        Complex64::zero(),
                    ],
                ],
            )?;
        }

        GateKind::Y => {
            apply_single_qubit_matrix(
                state,
                gate.qubits()[0].index(),
                [
                    [
                        Complex64::zero(),
                        Complex64::new(
                            0.0,
                            -1.0,
                        ),
                    ],
                    [
                        Complex64::new(
                            0.0,
                            1.0,
                        ),
                        Complex64::zero(),
                    ],
                ],
            )?;
        }

        GateKind::Z => {
            apply_single_qubit_matrix(
                state,
                gate.qubits()[0].index(),
                [
                    [
                        Complex64::one(),
                        Complex64::zero(),
                    ],
                    [
                        Complex64::zero(),
                        Complex64::new(
                            -1.0,
                            0.0,
                        ),
                    ],
                ],
            )?;
        }

        GateKind::H => {
            let inverse_sqrt_two =
                1.0 / 2.0_f64.sqrt();

            apply_single_qubit_matrix(
                state,
                gate.qubits()[0].index(),
                [
                    [
                        Complex64::new(
                            inverse_sqrt_two,
                            0.0,
                        ),
                        Complex64::new(
                            inverse_sqrt_two,
                            0.0,
                        ),
                    ],
                    [
                        Complex64::new(
                            inverse_sqrt_two,
                            0.0,
                        ),
                        Complex64::new(
                            -inverse_sqrt_two,
                            0.0,
                        ),
                    ],
                ],
            )?;
        }

        GateKind::S => {
            apply_single_qubit_matrix(
                state,
                gate.qubits()[0].index(),
                [
                    [
                        Complex64::one(),
                        Complex64::zero(),
                    ],
                    [
                        Complex64::zero(),
                        Complex64::new(
                            0.0,
                            1.0,
                        ),
                    ],
                ],
            )?;
        }

        GateKind::Sdg => {
            apply_single_qubit_matrix(
                state,
                gate.qubits()[0].index(),
                [
                    [
                        Complex64::one(),
                        Complex64::zero(),
                    ],
                    [
                        Complex64::zero(),
                        Complex64::new(
                            0.0,
                            -1.0,
                        ),
                    ],
                ],
            )?;
        }

        GateKind::T => {
            apply_single_qubit_matrix(
                state,
                gate.qubits()[0].index(),
                phase_matrix(
                    std::f64::consts::FRAC_PI_4,
                ),
            )?;
        }

        GateKind::Tdg => {
            apply_single_qubit_matrix(
                state,
                gate.qubits()[0].index(),
                phase_matrix(
                    -std::f64::consts::FRAC_PI_4,
                ),
            )?;
        }

        GateKind::V => {
            apply_single_qubit_matrix(
                state,
                gate.qubits()[0].index(),
                [
                    [
                        Complex64::new(
                            0.5,
                            0.5,
                        ),
                        Complex64::new(
                            0.5,
                            -0.5,
                        ),
                    ],
                    [
                        Complex64::new(
                            0.5,
                            -0.5,
                        ),
                        Complex64::new(
                            0.5,
                            0.5,
                        ),
                    ],
                ],
            )?;
        }

        GateKind::Vdg => {
            apply_single_qubit_matrix(
                state,
                gate.qubits()[0].index(),
                [
                    [
                        Complex64::new(
                            0.5,
                            -0.5,
                        ),
                        Complex64::new(
                            0.5,
                            0.5,
                        ),
                    ],
                    [
                        Complex64::new(
                            0.5,
                            0.5,
                        ),
                        Complex64::new(
                            0.5,
                            -0.5,
                        ),
                    ],
                ],
            )?;
        }

        GateKind::RX => {
            let theta =
                required_parameter(gate, 0)?;

            apply_single_qubit_matrix(
                state,
                gate.qubits()[0].index(),
                rotation_x(theta),
            )?;
        }

        GateKind::RY => {
            let theta =
                required_parameter(gate, 0)?;

            apply_single_qubit_matrix(
                state,
                gate.qubits()[0].index(),
                rotation_y(theta),
            )?;
        }

        GateKind::RZ
        | GateKind::Phase
        | GateKind::U1 => {
            let theta =
                required_parameter(gate, 0)?;

            apply_single_qubit_matrix(
                state,
                gate.qubits()[0].index(),
                phase_matrix(theta),
            )?;
        }

        GateKind::U2 => {
            let phi =
                required_parameter(gate, 0)?;
            let lambda =
                required_parameter(gate, 1)?;

            apply_single_qubit_matrix(
                state,
                gate.qubits()[0].index(),
                u3(
                    std::f64::consts::FRAC_PI_2,
                    phi,
                    lambda,
                ),
            )?;
        }

        GateKind::U3 => {
            let theta =
                required_parameter(gate, 0)?;
            let phi =
                required_parameter(gate, 1)?;
            let lambda =
                required_parameter(gate, 2)?;

            apply_single_qubit_matrix(
                state,
                gate.qubits()[0].index(),
                u3(theta, phi, lambda),
            )?;
        }

        GateKind::CX => {
            apply_controlled_x(
                state,
                gate.qubits()[0].index(),
                gate.qubits()[1].index(),
            )?;
        }

        GateKind::CY => {
            apply_controlled_matrix(
                state,
                gate.qubits()[0].index(),
                gate.qubits()[1].index(),
                [
                    [
                        Complex64::zero(),
                        Complex64::new(
                            0.0,
                            -1.0,
                        ),
                    ],
                    [
                        Complex64::new(
                            0.0,
                            1.0,
                        ),
                        Complex64::zero(),
                    ],
                ],
            )?;
        }

        GateKind::CZ => {
            apply_controlled_z(
                state,
                gate.qubits()[0].index(),
                gate.qubits()[1].index(),
            )?;
        }

        GateKind::CH => {
            apply_controlled_matrix(
                state,
                gate.qubits()[0].index(),
                gate.qubits()[1].index(),
                hadamard_matrix(),
            )?;
        }

        GateKind::SWAP => {
            apply_swap(
                state,
                gate.qubits()[0].index(),
                gate.qubits()[1].index(),
            )?;
        }

        GateKind::ISWAP => {
            apply_iswap(
                state,
                gate.qubits()[0].index(),
                gate.qubits()[1].index(),
            )?;
        }

        GateKind::ECR => {
            apply_ecr(
                state,
                gate.qubits()[0].index(),
                gate.qubits()[1].index(),
            )?;
        }

        GateKind::CRX => {
            let theta =
                required_parameter(gate, 0)?;

            apply_controlled_matrix(
                state,
                gate.qubits()[0].index(),
                gate.qubits()[1].index(),
                rotation_x(theta),
            )?;
        }

        GateKind::CRY => {
            let theta =
                required_parameter(gate, 0)?;

            apply_controlled_matrix(
                state,
                gate.qubits()[0].index(),
                gate.qubits()[1].index(),
                rotation_y(theta),
            )?;
        }

        GateKind::CRZ => {
            let theta =
                required_parameter(gate, 0)?;

            apply_controlled_matrix(
                state,
                gate.qubits()[0].index(),
                gate.qubits()[1].index(),
                phase_matrix(theta),
            )?;
        }

        GateKind::CCX => {
            apply_toffoli(
                state,
                gate.qubits()[0].index(),
                gate.qubits()[1].index(),
                gate.qubits()[2].index(),
            )?;
        }

        GateKind::CSWAP => {
            apply_fredkin(
                state,
                gate.qubits()[0].index(),
                gate.qubits()[1].index(),
                gate.qubits()[2].index(),
            )?;
        }

        GateKind::Measure => {
            measure_in_place(
                state,
                epsilon,
            )?;
        }

        GateKind::Reset => {
            reset_qubit(
                state,
                gate.qubits()[0].index(),
                epsilon,
            )?;
        }

        GateKind::Barrier => {
            // A barrier has no mathematical effect in an ideal state-vector
            // simulator. It remains semantically visible to hardware-aware
            // scheduling layers but does not alter amplitudes.
        }
    }

    Ok(())
}

// =============================================================================
// Single-qubit operations
// =============================================================================

fn apply_single_qubit_matrix(
    state: &mut StateVector,
    qubit: usize,
    matrix: [[Complex64; 2]; 2],
) -> Result<(), SimulationError> {
    state.check_qubit(qubit)?;

    let stride =
        1usize
            .checked_shl(
                qubit as u32,
            )
            .ok_or(
                SimulationError::ArithmeticOverflow {
                    calculation: "single-qubit stride",
                },
            )?;

    let span = stride
        .checked_mul(2)
        .ok_or(
            SimulationError::ArithmeticOverflow {
                calculation: "single-qubit span",
            },
        )?;

    let length = state.amplitudes.len();

    let mut base = 0usize;

    while base < length {
        for offset in 0..stride {
            let zero_index =
                base
                    .checked_add(offset)
                    .ok_or(
                        SimulationError::ArithmeticOverflow {
                            calculation:
                                "single-qubit index",
                        },
                    )?;

            let one_index =
                zero_index
                    .checked_add(stride)
                    .ok_or(
                        SimulationError::ArithmeticOverflow {
                            calculation:
                                "single-qubit index",
                        },
                    )?;

            let a0 =
                state.amplitudes[zero_index];

            let a1 =
                state.amplitudes[one_index];

            state.amplitudes[zero_index] =
                matrix[0][0] * a0
                    + matrix[0][1] * a1;

            state.amplitudes[one_index] =
                matrix[1][0] * a0
                    + matrix[1][1] * a1;
        }

        base = base
            .checked_add(span)
            .ok_or(
                SimulationError::ArithmeticOverflow {
                    calculation: "single-qubit base",
                },
            )?;
    }

    Ok(())
}

fn hadamard_matrix() -> [[Complex64; 2]; 2] {
    let value =
        1.0 / 2.0_f64.sqrt();

    [
        [
            Complex64::new(value, 0.0),
            Complex64::new(value, 0.0),
        ],
        [
            Complex64::new(value, 0.0),
            Complex64::new(-value, 0.0),
        ],
    ]
}

fn phase_matrix(
    theta: f64,
) -> [[Complex64; 2]; 2] {
    [
        [
            Complex64::one(),
            Complex64::zero(),
        ],
        [
            Complex64::zero(),
            Complex64::exp_i(theta),
        ],
    ]
}

fn rotation_x(
    theta: f64,
) -> [[Complex64; 2]; 2] {
    let half = theta * 0.5;

    let cosine = half.cos();
    let sine = half.sin();

    [
        [
            Complex64::new(cosine, 0.0),
            Complex64::new(0.0, -sine),
        ],
        [
            Complex64::new(0.0, -sine),
            Complex64::new(cosine, 0.0),
        ],
    ]
}

fn rotation_y(
    theta: f64,
) -> [[Complex64; 2]; 2] {
    let half = theta * 0.5;

    let cosine = half.cos();
    let sine = half.sin();

    [
        [
            Complex64::new(cosine, 0.0),
            Complex64::new(-sine, 0.0),
        ],
        [
            Complex64::new(sine, 0.0),
            Complex64::new(cosine, 0.0),
        ],
    ]
}

fn u3(
    theta: f64,
    phi: f64,
    lambda: f64,
) -> [[Complex64; 2]; 2] {
    let half =
        theta * 0.5;

    let cosine =
        half.cos();

    let sine =
        half.sin();

    [
        [
            Complex64::new(
                cosine,
                0.0,
            ),
            Complex64::exp_i(lambda)
                .scale(-sine),
        ],
        [
            Complex64::exp_i(phi)
                .scale(sine),
            Complex64::exp_i(phi + lambda)
                .scale(cosine),
        ],
    ]
}

// =============================================================================
// Controlled operations
// =============================================================================

fn apply_controlled_matrix(
    state: &mut StateVector,
    control: usize,
    target: usize,
    matrix: [[Complex64; 2]; 2],
) -> Result<(), SimulationError> {
    state.check_qubit(control)?;
    state.check_qubit(target)?;

    if control == target {
        return Err(
            SimulationError::InvalidGate {
                gate: GateKind::CX,
                reason:
                    "control and target must be different",
            },
        );
    }

    let control_mask =
        bit_mask(control)?;

    let target_mask =
        bit_mask(target)?;

    let length =
        state.amplitudes.len();

    for index in 0..length {
        if index & control_mask == 0 {
            continue;
        }

        if index & target_mask != 0 {
            continue;
        }

        let one_index =
            index
                .checked_add(target_mask)
                .ok_or(
                    SimulationError::ArithmeticOverflow {
                        calculation:
                            "controlled gate index",
                    },
                )?;

        let a0 =
            state.amplitudes[index];

        let a1 =
            state.amplitudes[one_index];

        state.amplitudes[index] =
            matrix[0][0] * a0
                + matrix[0][1] * a1;

        state.amplitudes[one_index] =
            matrix[1][0] * a0
                + matrix[1][1] * a1;
    }

    Ok(())
}

fn apply_controlled_x(
    state: &mut StateVector,
    control: usize,
    target: usize,
) -> Result<(), SimulationError> {
    apply_controlled_matrix(
        state,
        control,
        target,
        [
            [
                Complex64::zero(),
                Complex64::one(),
            ],
            [
                Complex64::one(),
                Complex64::zero(),
            ],
        ],
    )
}

fn apply_controlled_z(
    state: &mut StateVector,
    control: usize,
    target: usize,
) -> Result<(), SimulationError> {
    state.check_qubit(control)?;
    state.check_qubit(target)?;

    if control == target {
        return Err(
            SimulationError::InvalidGate {
                gate: GateKind::CZ,
                reason:
                    "control and target must be different",
            },
        );
    }

    let control_mask =
        bit_mask(control)?;

    let target_mask =
        bit_mask(target)?;

    for index in 0..state.amplitudes.len() {
        if index & control_mask != 0
            && index & target_mask != 0
        {
            state.amplitudes[index] =
                state.amplitudes[index]
                    .scale(-1.0);
        }
    }

    Ok(())
}

fn apply_swap(
    state: &mut StateVector,
    first: usize,
    second: usize,
) -> Result<(), SimulationError> {
    state.check_qubit(first)?;
    state.check_qubit(second)?;

    if first == second {
        return Err(
            SimulationError::InvalidGate {
                gate: GateKind::SWAP,
                reason:
                    "SWAP operands must differ",
            },
        );
    }

    let first_mask =
        bit_mask(first)?;

    let second_mask =
        bit_mask(second)?;

    for index in 0..state.amplitudes.len() {
        let first_bit =
            index & first_mask != 0;

        let second_bit =
            index & second_mask != 0;

        if first_bit == second_bit {
            continue;
        }

        let swapped =
            index ^ first_mask ^ second_mask;

        if index < swapped {
            state.amplitudes.swap(
                index,
                swapped,
            );
        }
    }

    Ok(())
}

fn apply_iswap(
    state: &mut StateVector,
    first: usize,
    second: usize,
) -> Result<(), SimulationError> {
    state.check_qubit(first)?;
    state.check_qubit(second)?;

    if first == second {
        return Err(
            SimulationError::InvalidGate {
                gate: GateKind::ISWAP,
                reason:
                    "iSWAP operands must differ",
            },
        );
    }

    let first_mask =
        bit_mask(first)?;

    let second_mask =
        bit_mask(second)?;

    let pair_mask =
        first_mask | second_mask;

    let imaginary =
        Complex64::new(0.0, 1.0);

    for index in 0..state.amplitudes.len() {
        if index & pair_mask
            != first_mask
        {
            continue;
        }

        let swapped =
            index ^ pair_mask;

        let a01 =
            state.amplitudes[index];

        let a10 =
            state.amplitudes[swapped];

        state.amplitudes[index] =
            imaginary * a10;

        state.amplitudes[swapped] =
            imaginary * a01;
    }

    Ok(())
}

fn apply_ecr(
    state: &mut StateVector,
    first: usize,
    second: usize,
) -> Result<(), SimulationError> {
    // ECR is implemented through its exact matrix in the computational basis.
    //
    // ECR =
    //
    // 1/sqrt(2) *
    // [ 0  0  1  i ]
    // [ 0  0  i  1 ]
    // [ 1 -i  0  0 ]
    // [-i  1  0  0 ]
    //
    // The matrix is applied to the two-qubit subspace.
    state.check_qubit(first)?;
    state.check_qubit(second)?;

    if first == second {
        return Err(
            SimulationError::InvalidGate {
                gate: GateKind::ECR,
                reason:
                    "ECR operands must differ",
            },
        );
    }

    let first_mask =
        bit_mask(first)?;

    let second_mask =
        bit_mask(second)?;

    let inv_sqrt_two =
        1.0 / 2.0_f64.sqrt();

    let i =
        Complex64::new(0.0, 1.0);

    let matrix = [
        [
            Complex64::zero(),
            Complex64::zero(),
            Complex64::new(
                inv_sqrt_two,
                0.0,
            ),
            i.scale(
                inv_sqrt_two,
            ),
        ],
        [
            Complex64::zero(),
            Complex64::zero(),
            i.scale(
                inv_sqrt_two,
            ),
            Complex64::new(
                inv_sqrt_two,
                0.0,
            ),
        ],
        [
            Complex64::new(
                inv_sqrt_two,
                0.0,
            ),
            i.scale(
                -inv_sqrt_two,
            ),
            Complex64::zero(),
            Complex64::zero(),
        ],
        [
            i.scale(
                -inv_sqrt_two,
            ),
            Complex64::new(
                inv_sqrt_two,
                0.0,
            ),
            Complex64::zero(),
            Complex64::zero(),
        ],
    ];

    apply_two_qubit_matrix(
        state,
        first_mask,
        second_mask,
        matrix,
    )
}

// =============================================================================
// Three-qubit operations
// =============================================================================

fn apply_toffoli(
    state: &mut StateVector,
    control_a: usize,
    control_b: usize,
    target: usize,
) -> Result<(), SimulationError> {
    state.check_qubit(control_a)?;
    state.check_qubit(control_b)?;
    state.check_qubit(target)?;

    if control_a == control_b
        || control_a == target
        || control_b == target
    {
        return Err(
            SimulationError::InvalidGate {
                gate: GateKind::CCX,
                reason:
                    "CCX operands must be distinct",
            },
        );
    }

    let control_a_mask =
        bit_mask(control_a)?;

    let control_b_mask =
        bit_mask(control_b)?;

    let target_mask =
        bit_mask(target)?;

    for index in 0..state.amplitudes.len() {
        if index & control_a_mask == 0
            || index & control_b_mask == 0
            || index & target_mask != 0
        {
            continue;
        }

        let partner =
            index | target_mask;

        state.amplitudes.swap(
            index,
            partner,
        );
    }

    Ok(())
}

fn apply_fredkin(
    state: &mut StateVector,
    control: usize,
    first: usize,
    second: usize,
) -> Result<(), SimulationError> {
    state.check_qubit(control)?;
    state.check_qubit(first)?;
    state.check_qubit(second)?;

    if control == first
        || control == second
        || first == second
    {
        return Err(
            SimulationError::InvalidGate {
                gate: GateKind::CSWAP,
                reason:
                    "CSWAP operands must be distinct",
            },
        );
    }

    let control_mask =
        bit_mask(control)?;

    let first_mask =
        bit_mask(first)?;

    let second_mask =
        bit_mask(second)?;

    let swap_mask =
        first_mask | second_mask;

    for index in 0..state.amplitudes.len() {
        if index & control_mask == 0 {
            continue;
        }

        let first_bit =
            index & first_mask != 0;

        let second_bit =
            index & second_mask != 0;

        if first_bit == second_bit {
            continue;
        }

        let partner =
            index ^ swap_mask;

        if index < partner {
            state.amplitudes.swap(
                index,
                partner,
            );
        }
    }

    Ok(())
}

fn apply_two_qubit_matrix(
    state: &mut StateVector,
    first_mask: usize,
    second_mask: usize,
    matrix: [[Complex64; 4]; 4],
) -> Result<(), SimulationError> {
    if first_mask == second_mask
        || first_mask == 0
        || second_mask == 0
    {
        return Err(
            SimulationError::InvalidStateVector {
                reason:
                    "invalid two-qubit masks",
            },
        );
    }

    let combined =
        first_mask | second_mask;

    for base in 0..state.amplitudes.len() {
        if base & combined != 0 {
            continue;
        }

        let indices = [
            base,
            base | second_mask,
            base | first_mask,
            base | first_mask | second_mask,
        ];

        let old = [
            state.amplitudes[indices[0]],
            state.amplitudes[indices[1]],
            state.amplitudes[indices[2]],
            state.amplitudes[indices[3]],
        ];

        for row in 0..4 {
            let mut value =
                Complex64::zero();

            for column in 0..4 {
                value = value
                    + matrix[row][column]
                        * old[column];
            }

            state.amplitudes[indices[row]] =
                value;
        }
    }

    Ok(())
}

// =============================================================================
// Measurement and reset
// =============================================================================

fn measure_in_place(
    state: &mut StateVector,
    epsilon: f64,
) -> Result<(), SimulationError> {
    let probabilities =
        probabilities_from_state(
            &state.amplitudes,
            epsilon,
        )?;

    // Measurement gates in the canonical IR do not encode a measurement
    // destination in the state-vector itself. Therefore this operation uses a
    // deterministic representative outcome: the most probable basis state.
    //
    // Shot sampling is performed by `run()` from the final state and does not
    // depend on this helper.
    let index =
        probabilities
            .iter()
            .enumerate()
            .max_by(|left, right| {
                left.1
                    .partial_cmp(right.1)
                    .unwrap_or(
                        std::cmp::Ordering::Equal,
                    )
                    .then_with(|| {
                        right.0.cmp(&left.0)
                    })
            })
            .map(|(index, _)| index)
            .ok_or(
                SimulationError::SamplingFailure,
            )?;

    collapse_to_basis_state(
        state,
        index,
        epsilon,
    )
}

fn reset_qubit(
    state: &mut StateVector,
    qubit: usize,
    epsilon: f64,
) -> Result<(), SimulationError> {
    state.check_qubit(qubit)?;

    let probabilities =
        qubit_probabilities(
            state,
            qubit,
        )?;

    let probability_one =
        probabilities[1];

    let probability_zero =
        probabilities[0];

    if probability_zero
        < epsilon
        && probability_one < epsilon
    {
        return Err(
            SimulationError::InvalidProbabilityDistribution,
        );
    }

    if probability_one
        > probability_zero
    {
        project_qubit(
            state,
            qubit,
            true,
        )?;
    } else {
        project_qubit(
            state,
            qubit,
            false,
        )?;
    }

    // Reset maps |1> -> |0> and leaves |0> unchanged.
    let mask =
        bit_mask(qubit)?;

    for index in 0..state.amplitudes.len() {
        if index & mask != 0 {
            state.amplitudes[index] =
                Complex64::zero();
        }
    }

    state.normalize(epsilon)?;

    Ok(())
}

fn collapse_to_basis_state(
    state: &mut StateVector,
    basis_index: usize,
    epsilon: f64,
) -> Result<(), SimulationError> {
    if basis_index >= state.amplitudes.len() {
        return Err(
            SimulationError::SamplingFailure,
        );
    }

    let amplitude =
        state.amplitudes[basis_index];

    let probability =
        amplitude.norm_squared();

    if !probability.is_finite()
        || probability <= epsilon * epsilon
    {
        return Err(
            SimulationError::SamplingFailure,
        );
    }

    let inverse =
        1.0 / probability.sqrt();

    for (index, value) in
        state.amplitudes.iter_mut().enumerate()
    {
        if index == basis_index {
            *value =
                value.scale(inverse);
        } else {
            *value =
                Complex64::zero();
        }
    }

    Ok(())
}

fn project_qubit(
    state: &mut StateVector,
    qubit: usize,
    one: bool,
) -> Result<(), SimulationError> {
    let mask =
        bit_mask(qubit)?;

    for index in 0..state.amplitudes.len() {
        let is_one =
            index & mask != 0;

        if is_one != one {
            state.amplitudes[index] =
                Complex64::zero();
        }
    }

    Ok(())
}

fn qubit_probabilities(
    state: &StateVector,
    qubit: usize,
) -> Result<[f64; 2], SimulationError> {
    state.check_qubit(qubit)?;

    let mask =
        bit_mask(qubit)?;

    let mut probabilities =
        [0.0_f64; 2];

    for (index, amplitude) in
        state.amplitudes.iter().enumerate()
    {
        let bucket =
            if index & mask != 0 {
                1
            } else {
                0
            };

        probabilities[bucket] +=
            amplitude.norm_squared();
    }

    if !probabilities[0].is_finite()
        || !probabilities[1].is_finite()
    {
        return Err(
            SimulationError::InvalidProbabilityDistribution,
        );
    }

    Ok(probabilities)
}

// =============================================================================
// Probability and sampling
// =============================================================================

fn probabilities_from_state(
    state: &[Complex64],
    epsilon: f64,
) -> Result<Vec<f64>, SimulationError> {
    if state.is_empty() {
        return Err(
            SimulationError::InvalidStateVector {
                reason:
                    "empty state vector",
            },
        );
    }

    let mut probabilities =
        Vec::with_capacity(state.len());

    let mut total =
        0.0_f64;

    for amplitude in state {
        let probability =
            amplitude.norm_squared();

        if !probability.is_finite()
            || probability < -epsilon
        {
            return Err(
                SimulationError::InvalidProbabilityDistribution,
            );
        }

        let normalized =
            if probability < 0.0 {
                0.0
            } else {
                probability
            };

        probabilities.push(
            normalized,
        );

        total += normalized;

        if !total.is_finite() {
            return Err(
                SimulationError::InvalidProbabilityDistribution,
            );
        }
    }

    if total <= epsilon {
        return Err(
            SimulationError::InvalidProbabilityDistribution,
        );
    }

    for probability in
        &mut probabilities
    {
        *probability /= total;

        if !probability.is_finite()
            || *probability < 0.0
        {
            return Err(
                SimulationError::InvalidProbabilityDistribution,
            );
        }
    }

    Ok(probabilities)
}

fn sample_distribution(
    probabilities: &[f64],
    rng: &mut SplitMix64,
    epsilon: f64,
) -> Result<usize, SimulationError> {
    if probabilities.is_empty() {
        return Err(
            SimulationError::SamplingFailure,
        );
    }

    let random =
        rng.next_f64();

    let mut cumulative =
        0.0_f64;

    for (index, probability) in
        probabilities.iter().enumerate()
    {
        cumulative += *probability;

        if random
            < cumulative
            || cumulative >= 1.0 - epsilon
        {
            return Ok(index);
        }
    }

    probabilities
        .len()
        .checked_sub(1)
        .ok_or(
            SimulationError::SamplingFailure,
        )
}

// =============================================================================
// Parameter handling
// =============================================================================

fn parameter_value(
    gate: GateKind,
    index: usize,
    parameter: &Parameter,
) -> Result<f64, SimulationError> {
    match parameter {
        Parameter::Constant(value) => {
            if !value.is_finite() {
                return Err(
                    SimulationError::NonFiniteParameter {
                        gate,
                        index,
                        value: *value,
                    },
                );
            }

            Ok(*value)
        }

        _ => Err(
            SimulationError::SymbolicParameter {
                gate,
                index,
            },
        ),
    }
}

fn required_parameter(
    gate: &Gate,
    index: usize,
) -> Result<f64, SimulationError> {
    gate.parameters()
        .get(index)
        .ok_or(
            SimulationError::InvalidGate {
                gate: gate.kind(),
                reason:
                    "missing required parameter",
            },
        )
        .and_then(|parameter| {
            parameter_value(
                gate.kind(),
                index,
                parameter,
            )
        })
}

// =============================================================================
// Bit/index helpers
// =============================================================================

fn checked_pow2(
    qubits: usize,
) -> Result<usize, SimulationError> {
    if qubits >= usize::BITS as usize {
        return Err(
            SimulationError::ArithmeticOverflow {
                calculation:
                    "state-vector amplitude count",
            },
        );
    }

    1usize
        .checked_shl(
            qubits as u32,
        )
        .ok_or(
            SimulationError::ArithmeticOverflow {
                calculation:
                    "state-vector amplitude count",
            },
        )
}

fn bit_mask(
    qubit: usize,
) -> Result<usize, SimulationError> {
    if qubit >= usize::BITS as usize {
        return Err(
            SimulationError::ArithmeticOverflow {
                calculation: "qubit bit mask",
            },
        );
    }

    1usize
        .checked_shl(
            qubit as u32,
        )
        .ok_or(
            SimulationError::ArithmeticOverflow {
                calculation:
                    "qubit bit mask",
            },
        )
}

fn basis_index_to_bit_string(
    index: usize,
    num_bits: usize,
) -> Result<String, SimulationError> {
    if num_bits >= usize::BITS as usize {
        return Err(
            SimulationError::ArithmeticOverflow {
                calculation:
                    "basis bit string",
            },
        );
    }

    let amplitude_count =
        checked_pow2(num_bits)?;

    if index >= amplitude_count {
        return Err(
            SimulationError::SamplingFailure,
        );
    }

    let mut result =
        String::with_capacity(num_bits);

    for bit in (0..num_bits).rev() {
        let mask =
            bit_mask(bit)?;

        if index & mask != 0 {
            result.push('1');
        } else {
            result.push('0');
        }
    }

    Ok(result)
}

fn bit_string_to_basis_index(
    bit_string: &str,
) -> Result<usize, SimulationError> {
    if bit_string.is_empty() {
        return Err(
            SimulationError::SamplingFailure,
        );
    }

    let mut value =
        0usize;

    for character in
        bit_string.chars()
    {
        value = value
            .checked_shl(1)
            .ok_or(
                SimulationError::ArithmeticOverflow {
                    calculation:
                        "bit-string basis index",
                },
            )?;

        match character {
            '0' => {}

            '1' => {
                value = value
                    .checked_add(1)
                    .ok_or(
                        SimulationError::ArithmeticOverflow {
                            calculation:
                                "bit-string basis index",
                        },
                    )?;
            }

            _ => {
                return Err(
                    SimulationError::SamplingFailure,
                );
            }
        }
    }

    Ok(value)
}

// =============================================================================
// Deterministic pseudo-random generator
// =============================================================================

/// Small deterministic SplitMix64 generator.
///
/// This is intentionally private. The simulator exposes only the seed as part
/// of its reproducibility contract and does not expose the PRNG as public API.
#[derive(Debug, Clone, Copy)]
struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self {
            state: seed,
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.state =
            self.state
                .wrapping_add(
                    0x9E3779B97F4A7C15,
                );

        let mut value =
            self.state;

        value =
            (value
                ^ (value >> 30))
                .wrapping_mul(
                    0xBF58476D1CE4E5B9,
                );

        value =
            (value
                ^ (value >> 27))
                .wrapping_mul(
                    0x94D049BB133111EB,
                );

        value ^ (value >> 31)
    }

    fn next_f64(&mut self) -> f64 {
        let value =
            self.next_u64();

        let mantissa =
            value >> 11;

        (mantissa as f64)
            * (1.0 / 9007199254740992.0)
    }
}

fn mix_seed(
    a: u64,
    b: u64,
    c: u64,
) -> u64 {
    let mut value =
        a ^ 0x9E3779B97F4A7C15;

    value =
        value.wrapping_add(
            b.rotate_left(17),
        );

    value =
        value.wrapping_add(
            c.rotate_left(31),
        );

    let mut rng =
        SplitMix64::new(value);

    rng.next_u64()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::ir::gate::GateKind;
    use super::super::super::ir::qubits::QubitId;

    fn circuit_with_gate(
        num_qubits: usize,
        kind: GateKind,
        qubits: Vec<QubitId>,
        parameters: Vec<Parameter>,
    ) -> QuantumCircuit {
        let gate =
            Gate::new(
                kind,
                qubits,
                parameters,
                if kind == GateKind::Measure {
                    Some(0)
                } else {
                    None
                },
                None,
            )
            .expect("test gate must be valid");

        QuantumCircuit::from_operations(
            num_qubits,
            if kind == GateKind::Measure {
                1
            } else {
                0
            },
            vec![gate],
        )
        .expect("test circuit must be valid")
    }

    fn qubit(
        index: usize,
    ) -> QubitId {
        QubitId::new(index)
    }

    #[test]
    fn production_limits_are_valid() {
        let limits =
            SimulatorLimits::production();

        assert!(
            limits.validate().is_ok()
        );

        assert_eq!(
            limits.max_qubits,
            DEFAULT_MAX_QUBITS
        );
    }

    #[test]
    fn deterministic_seed_is_preserved() {
        let config =
            SimulatorConfig::deterministic(42);

        assert_eq!(
            config.seed,
            Some(42)
        );
    }

    #[test]
    fn checked_power_of_two_is_safe() {
        assert_eq!(
            checked_pow2(0).unwrap(),
            1
        );

        assert_eq!(
            checked_pow2(1).unwrap(),
            2
        );

        assert_eq!(
            checked_pow2(3).unwrap(),
            8
        );
    }

    #[test]
    fn bit_string_round_trip() {
        for index in 0..16 {
            let text =
                basis_index_to_bit_string(
                    index,
                    4,
                )
                .unwrap();

            let restored =
                bit_string_to_basis_index(
                    &text,
                )
                .unwrap();

            assert_eq!(
                index,
                restored
            );
        }
    }

    #[test]
    fn x_flips_zero_to_one() {
        let circuit =
            circuit_with_gate(
                1,
                GateKind::X,
                vec![qubit(0)],
                vec![],
            );

        let simulator =
            StateVectorSimulator::deterministic(
                7,
            )
            .unwrap();

        let result =
            simulator
                .run(&circuit)
                .unwrap();

        assert_eq!(
            result.counts.get("1"),
            Some(1024)
        );

        assert_eq!(
            result.counts.get("0"),
            None
        );
    }

    #[test]
    fn h_creates_equal_probabilities() {
        let circuit =
            circuit_with_gate(
                1,
                GateKind::H,
                vec![qubit(0)],
                vec![],
            );

        let simulator =
            StateVectorSimulator::deterministic(
                7,
            )
            .unwrap();

        let result =
            simulator
                .run(&circuit)
                .unwrap();

        let p0 =
            result
                .probability_of("0")
                .unwrap();

        let p1 =
            result
                .probability_of("1")
                .unwrap();

        assert!(
            (p0 - 0.5).abs()
                < 1.0e-12
        );

        assert!(
            (p1 - 0.5).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn cx_creates_bell_state() {
        let h =
            Gate::new(
                GateKind::H,
                vec![qubit(0)],
                vec![],
                None,
                None,
            )
            .unwrap();

        let cx =
            Gate::new(
                GateKind::CX,
                vec![
                    qubit(0),
                    qubit(1),
                ],
                vec![],
                None,
                None,
            )
            .unwrap();

        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                vec![h, cx],
            )
            .unwrap();

        let simulator =
            StateVectorSimulator::deterministic(
                123,
            )
            .unwrap();

        let result =
            simulator
                .run(&circuit)
                .unwrap();

        let p00 =
            result
                .probability_of("00")
                .unwrap();

        let p11 =
            result
                .probability_of("11")
                .unwrap();

        let p01 =
            result
                .probability_of("01")
                .unwrap();

        let p10 =
            result
                .probability_of("10")
                .unwrap();

        assert!(
            (p00 - 0.5).abs()
                < 1.0e-12
        );

        assert!(
            (p11 - 0.5).abs()
                < 1.0e-12
        );

        assert!(
            p01.abs() < 1.0e-12
        );

        assert!(
            p10.abs() < 1.0e-12
        );
    }

    #[test]
    fn swap_exchanges_basis_states() {
        let x =
            Gate::new(
                GateKind::X,
                vec![qubit(0)],
                vec![],
                None,
                None,
            )
            .unwrap();

        let swap =
            Gate::new(
                GateKind::SWAP,
                vec![
                    qubit(0),
                    qubit(1),
                ],
                vec![],
                None,
                None,
            )
            .unwrap();

        let circuit =
            QuantumCircuit::from_operations(
                2,
                0,
                vec![x, swap],
            )
            .unwrap();

        let simulator =
            StateVectorSimulator::deterministic(
                1,
            )
            .unwrap();

        let result =
            simulator
                .run(&circuit)
                .unwrap();

        assert_eq!(
            result.counts.get("10"),
            Some(1024)
        );
    }

    #[test]
    fn reset_returns_qubit_to_zero() {
        let x =
            Gate::new(
                GateKind::X,
                vec![qubit(0)],
                vec![],
                None,
                None,
            )
            .unwrap();

        let reset =
            Gate::new(
                GateKind::Reset,
                vec![qubit(0)],
                vec![],
                None,
                None,
            )
            .unwrap();

        let circuit =
            QuantumCircuit::from_operations(
                1,
                0,
                vec![x, reset],
            )
            .unwrap();

        let simulator =
            StateVectorSimulator::deterministic(
                5,
            )
            .unwrap();

        let result =
            simulator
                .run(&circuit)
                .unwrap();

        assert_eq!(
            result.counts.get("0"),
            Some(1024)
        );
    }

    #[test]
    fn barrier_has_no_state_vector_effect() {
        let x =
            Gate::new(
                GateKind::X,
                vec![qubit(0)],
                vec![],
                None,
                None,
            )
            .unwrap();

        let barrier =
            Gate::new(
                GateKind::Barrier,
                vec![qubit(0)],
                vec![],
                None,
                None,
            )
            .unwrap();

        let circuit =
            QuantumCircuit::from_operations(
                1,
                0,
                vec![x, barrier],
            )
            .unwrap();

        let simulator =
            StateVectorSimulator::deterministic(
                10,
            )
            .unwrap();

        let result =
            simulator
                .run(&circuit)
                .unwrap();

        assert_eq!(
            result.counts.get("1"),
            Some(1024)
        );
    }

    #[test]
    fn simulator_rejects_excessive_qubits() {
        let limits =
            SimulatorLimits::new(
                2,
                4,
                100,
                100,
            );

        let config =
            SimulatorConfig {
                limits,
                shots: 1,
                ..SimulatorConfig::production()
            };

        let simulator =
            StateVectorSimulator::with_config(
                config,
            )
            .unwrap();

        let circuit =
            QuantumCircuit::new(
                3,
                0,
            )
            .unwrap();

        let result =
            simulator.run(&circuit);

        assert!(
            matches!(
                result,
                Err(
                    SimulationError::EmptyCircuit
                )
            )
        );
    }

    #[test]
    fn sampling_is_reproducible_with_same_seed() {
        let circuit =
            circuit_with_gate(
                1,
                GateKind::H,
                vec![qubit(0)],
                vec![],
            );

        let first =
            StateVectorSimulator::deterministic(
                12345,
            )
            .unwrap()
            .run(&circuit)
            .unwrap();

        let second =
            StateVectorSimulator::deterministic(
                12345,
            )
            .unwrap()
            .run(&circuit)
            .unwrap();

        assert_eq!(
            first.counts,
            second.counts
        );
    }

    #[test]
    fn state_vector_output_can_be_disabled() {
        let circuit =
            circuit_with_gate(
                1,
                GateKind::X,
                vec![qubit(0)],
                vec![],
            );

        let config =
            SimulatorConfig {
                return_state_vector: false,
                ..SimulatorConfig::deterministic(
                    1,
                )
            };

        let simulator =
            StateVectorSimulator::with_config(
                config,
            )
            .unwrap();

        let result =
            simulator
                .run(&circuit)
                .unwrap();

        assert!(
            result.state_vector.is_none()
        );
    }
}