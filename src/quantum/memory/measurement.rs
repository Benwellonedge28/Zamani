//! Zamani Quantum Memory — Measurement Subsystem
//!
//! Production-grade, representation-independent quantum measurement contract.
//!
//! # Responsibility
//!
//! This module owns the provider-neutral semantics and data contracts for:
//!
//! - computational-basis measurement;
//! - Pauli-X measurement;
//! - Pauli-Y measurement;
//! - arbitrary single-qubit projective bases;
//! - multi-qubit measurement requests;
//! - mid-circuit measurement;
//! - destructive and non-destructive measurement semantics;
//! - measurement collapse policy;
//! - shot-based execution;
//! - single-shot results;
//! - aggregate histograms;
//! - probabilities;
//! - expectation values derived from measurement results;
//! - classical-bit destinations;
//! - logical-to-physical measurement association;
//! - provider/QPU measurement metadata;
//! - readout-error metadata;
//! - measurement capability negotiation;
//! - deterministic simulator sampling;
//! - provider-controlled randomness;
//! - measurement validation;
//! - result validation;
//! - provider-neutral measurement execution contracts.
//!
//! # Architectural rule
//!
//! `measurement.rs` does NOT implement representation-specific quantum
//! mathematics.
//!
//! It does NOT directly implement:
//!
//! - state-vector collapse;
//! - density-matrix projection;
//! - stabilizer tableau measurement;
//! - sparse-state measurement;
//! - tensor-network measurement;
//! - GPU kernels;
//! - QPU network communication;
//! - hardware calibration;
//! - readout mitigation algorithms;
//! - routing;
//! - scheduling;
//! - QEC decoding;
//! - compiler parsing.
//!
//! Those responsibilities remain with their respective modules.
//!
//! # Architecture
//!
//! ```text
//!                  quantum::frontend
//!                         |
//!                         v
//!                    quantum::ir
//!                         |
//!              +----------+----------+
//!              |                     |
//!              v                     v
//!          routing               scheduling
//!              |                     |
//!              +----------+----------+
//!                         |
//!                         v
//!                 execution layer
//!                         |
//!                         v
//!              memory::measurement
//!                         |
//!       +-----------------+------------------+
//!       |                 |                  |
//!       v                 v                  v
//!  state-vector      density-matrix     stabilizer
//!       |                 |                  |
//!       +-----------------+------------------+
//!                         |
//!                         v
//!                 provider boundary
//!                         |
//!       +-----------------+------------------+
//!       |                 |                  |
//!       v                 v                  v
//!      CPU               GPU                QPU
//!                                             |
//!                    +------------------------+
//!                    |
//!                    v
//!            physical hardware
//! ```
//!
//! # QPU neutrality
//!
//! A real QPU does not necessarily expose:
//!
//! - amplitudes;
//! - state vectors;
//! - probability vectors;
//! - collapse operations;
//! - deterministic sampling;
//! - arbitrary measurement bases;
//! - non-destructive measurement;
//! - raw individual shots;
//! - readout before/after state inspection.
//!
//! Therefore this module never assumes simulator semantics for a QPU.
//!
//! A provider must explicitly advertise capabilities.
//!
//! # Canonical identities
//!
//! Logical qubits use:
//!
//! `crate::quantum::ir::QubitId`
//!
//! Physical qubits use:
//!
//! `crate::quantum::ir::PhysicalQubitId`
//!
//! Classical destinations use:
//!
//! `crate::quantum::ir::ClassicalBitId`
//!
//! No replacement identifier types are defined here.
//!
//! # Error boundary
//!
//! All fallible operations return:
//!
//! `Result<T, MemoryError>`
//!
//! from `memory::errors`.
//!
//! # Determinism
//!
//! There is no hidden global RNG.
//!
//! Simulator-side sampling uses an explicitly supplied `MeasurementRandomSource`.
//!
//! Hardware providers own physical randomness and must report that they use
//! provider-controlled randomness.
//!
//! # Security
//!
//! This module never stores or exposes:
//!
//! - credentials;
//! - API keys;
//! - provider tokens;
//! - private keys;
//! - raw device pointers;
//! - network authentication material.
//!
//! Provider execution identifiers are treated as opaque metadata.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly-only features are required.
//!
//! # Integration contract
//!
//! Earlier foundational modules:
//!
//! ```text
//! types.rs
//! errors.rs
//! numeric.rs
//! complex.rs
//! representation.rs
//! limits.rs
//! layout.rs
//! indexing.rs
//! qubit.rs
//! state.rs
//! ```
//!
//! Later modules consume this contract:
//!
//! ```text
//! collapse.rs
//! reset.rs
//! state_vector.rs
//! density_matrix.rs
//! stabilizer.rs
//! sparse.rs
//! tensor_network.rs
//! backend_state.rs
//! snapshot.rs
//! checkpoint.rs
//! diagnostics.rs
//! telemetry.rs
//! ```
//!
//! Other quantum subsystems consume measurement results:
//!
//! ```text
//! quantum::ir
//! quantum::routing
//! quantum::scheduling
//! quantum::error_correction
//! quantum::hardware
//! quantum::benchmarking
//! runtime/execution
//! ```
//!
//! This module must not depend on those higher-level subsystems.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::num::NonZeroU64;

use crate::quantum::ir::{ClassicalBitId, PhysicalQubitId, QubitId};

use super::errors::MemoryError;

// =============================================================================
// Constants
// =============================================================================

/// Stable schema identifier.
pub const MEASUREMENT_SCHEMA_ID: &str = "zamani.quantum.memory.measurement";

/// Semantic version of this measurement contract.
pub const MEASUREMENT_SCHEMA_VERSION: u16 = 1;

/// Maximum number of qubits in one measurement request.
///
/// This is a structural safety bound, not a hardware limit. Larger workloads
/// should use multiple requests or a provider-specific batching interface.
pub const MAX_MEASUREMENT_QUBITS: usize = 1_048_576;

/// Maximum number of classical destination bits.
pub const MAX_CLASSICAL_DESTINATIONS: usize = 1_048_576;

/// Maximum shots in a single logical request.
///
/// Providers may impose a smaller limit.
pub const MAX_SHOTS: u64 = 1_000_000_000_000;

/// Maximum length of a provider-defined basis name.
pub const MAX_BASIS_NAME_LENGTH: usize = 256;

/// Maximum number of provider-defined basis parameters.
pub const MAX_BASIS_PARAMETERS: usize = 1024;

/// Maximum number of provider metadata entries.
pub const MAX_PROVIDER_METADATA_ENTRIES: usize = 256;

/// Maximum provider metadata key length.
pub const MAX_PROVIDER_METADATA_KEY_LENGTH: usize = 128;

/// Maximum provider metadata value length.
pub const MAX_PROVIDER_METADATA_VALUE_LENGTH: usize = 4096;

/// Probability tolerance used when validating normalized distributions.
pub const DEFAULT_PROBABILITY_TOLERANCE: f64 = 1.0e-12;

/// Default numerical tolerance for expectation values.
pub const DEFAULT_EXPECTATION_TOLERANCE: f64 = 1.0e-12;

/// Maximum number of histogram outcomes permitted in one in-memory result.
///
/// This protects callers against pathological result expansion.
pub const MAX_HISTOGRAM_OUTCOMES: usize = 1_048_576;

// =============================================================================
// Result alias
// =============================================================================

/// Canonical result type for measurement operations.
pub type MeasurementResult<T> = Result<T, MemoryError>;

// =============================================================================
// Measurement basis
// =============================================================================

/// Standard projective measurement basis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum MeasurementBasis {
    /// Computational/Z basis.
    Z,

    /// Pauli-X basis.
    X,

    /// Pauli-Y basis.
    Y,
}

impl MeasurementBasis {
    /// Returns the canonical basis name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Z => "z",
            Self::X => "x",
            Self::Y => "y",
        }
    }

    /// Returns the corresponding Pauli-axis vector.
    pub const fn axis(self) -> BlochAxis {
        match self {
            Self::Z => BlochAxis::new_const(0.0, 0.0, 1.0),
            Self::X => BlochAxis::new_const(1.0, 0.0, 0.0),
            Self::Y => BlochAxis::new_const(0.0, 1.0, 0.0),
        }
    }
}

impl fmt::Display for MeasurementBasis {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Bloch axis
// =============================================================================

/// Unit Bloch-sphere axis for arbitrary single-qubit projective measurement.
///
/// The observable is:
///
/// `n_x X + n_y Y + n_z Z`
///
/// where:
///
/// `n_x² + n_y² + n_z² = 1`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlochAxis {
    x: f64,
    y: f64,
    z: f64,
}

impl BlochAxis {
    /// Creates and validates a Bloch axis.
    pub fn new(x: f64, y: f64, z: f64) -> MeasurementResult<Self> {
        if !x.is_finite() || !y.is_finite() || !z.is_finite() {
            return Err(invalid_measurement(
                "measurement axis components must be finite",
            ));
        }

        let norm_squared = x
            .mul_add(x, y.mul_add(y, z * z));

        if !norm_squared.is_finite() {
            return Err(invalid_measurement(
                "measurement axis norm is non-finite",
            ));
        }

        if (norm_squared - 1.0).abs() > DEFAULT_PROBABILITY_TOLERANCE {
            return Err(invalid_measurement(
                "measurement axis must be normalized",
            ));
        }

        Ok(Self { x, y, z })
    }

    /// Creates a compile-time-known normalized axis.
    pub const fn new_const(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// X component.
    pub const fn x(self) -> f64 {
        self.x
    }

    /// Y component.
    pub const fn y(self) -> f64 {
        self.y
    }

    /// Z component.
    pub const fn z(self) -> f64 {
        self.z
    }

    /// Returns the squared norm.
    pub fn norm_squared(self) -> f64 {
        self.x
            .mul_add(self.x, self.y.mul_add(self.y, self.z * self.z))
    }
}

// =============================================================================
// Measurement observable
// =============================================================================

/// Provider-neutral measurement observable.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum MeasurementObservable {
    /// Standard Pauli/computational measurement.
    Standard(MeasurementBasis),

    /// Arbitrary single-qubit projective measurement.
    BlochAxis(BlochAxis),

    /// Provider-defined observable.
    ///
    /// This is required for hardware that exposes native measurement bases
    /// which do not map directly to X/Y/Z or a simple Bloch axis.
    ProviderDefined {
        /// Stable provider-neutral operation name.
        name: String,

        /// Provider-defined numerical parameters.
        parameters: Vec<f64>,
    },
}

impl MeasurementObservable {
    /// Validates the observable.
    pub fn validate(&self) -> MeasurementResult<()> {
        match self {
            Self::Standard(_) | Self::BlochAxis(_) => Ok(()),

            Self::ProviderDefined {
                name,
                parameters,
            } => {
                validate_text(
                    name,
                    MAX_BASIS_NAME_LENGTH,
                    "measurement basis name",
                )?;

                if parameters.len() > MAX_BASIS_PARAMETERS {
                    return Err(invalid_measurement(
                        "too many provider-defined measurement parameters",
                    ));
                }

                if parameters.iter().any(|value| !value.is_finite()) {
                    return Err(invalid_measurement(
                        "provider-defined measurement parameters must be finite",
                    ));
                }

                Ok(())
            }
        }
    }
}

impl Default for MeasurementObservable {
    fn default() -> Self {
        Self::Standard(MeasurementBasis::Z)
    }
}

// =============================================================================
// Measurement mode
// =============================================================================

/// Quantum measurement execution mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum MeasurementMode {
    /// Measurement is a terminal operation for the measured state/resource.
    Destructive,

    /// Measurement occurs while execution continues.
    MidCircuit,

    /// Measurement attempts to preserve the quantum state.
    ///
    /// This MUST NOT be assumed to be supported. A provider must explicitly
    /// advertise `NON_DESTRUCTIVE`.
    NonDestructive,
}

impl MeasurementMode {
    /// Returns whether execution can continue after the measurement.
    pub const fn allows_continuation(self) -> bool {
        matches!(self, Self::MidCircuit | Self::NonDestructive)
    }
}

// =============================================================================
// Collapse policy
// =============================================================================

/// Policy describing what happens to the quantum state after measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum CollapsePolicy {
    /// Apply the mathematically correct projective collapse.
    Collapse,

    /// Preserve the state if and only if the provider supports it.
    PreserveIfSupported,

    /// The provider determines the post-measurement behavior and reports it.
    ProviderDefined,
}

impl Default for CollapsePolicy {
    fn default() -> Self {
        Self::Collapse
    }
}

// =============================================================================
// Classical destination
// =============================================================================

/// Mapping from one measured qubit to one classical bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClassicalDestination {
    /// Logical qubit being measured.
    pub qubit: QubitId,

    /// Classical bit receiving the result.
    pub classical_bit: ClassicalBitId,
}

impl ClassicalDestination {
    /// Creates a destination mapping.
    pub const fn new(qubit: QubitId, classical_bit: ClassicalBitId) -> Self {
        Self {
            qubit,
            classical_bit,
        }
    }
}

// =============================================================================
// Measurement request
// =============================================================================

/// Complete provider-neutral measurement request.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementRequest {
    /// Logical qubits to measure, in result-bit order.
    ///
    /// The first qubit corresponds to result bit index 0.
    pub qubits: Vec<QubitId>,

    /// Optional physical-qubit mapping.
    ///
    /// The memory layer records this association but does not perform routing.
    pub physical_qubits: Vec<PhysicalQubitId>,

    /// Measurement observable.
    pub observable: MeasurementObservable,

    /// Measurement mode.
    pub mode: MeasurementMode,

    /// Post-measurement state policy.
    pub collapse: CollapsePolicy,

    /// Number of repetitions/shots.
    pub shots: NonZeroU64,

    /// Optional classical destinations.
    ///
    /// If empty, results remain in measurement-result space.
    pub classical_destinations: Vec<ClassicalDestination>,

    /// Whether raw per-shot results should be retained.
    pub retain_shots: bool,

    /// Whether aggregate counts must be returned.
    pub retain_counts: bool,

    /// Whether probabilities must be calculated from returned counts.
    pub calculate_probabilities: bool,

    /// Optional deterministic simulator seed.
    ///
    /// A physical QPU may reject deterministic-seed requirements because its
    /// physical measurement randomness is outside the simulator RNG contract.
    pub deterministic_seed: Option<u64>,
}

impl MeasurementRequest {
    /// Creates a computational-basis measurement request.
    pub fn new(
        qubits: Vec<QubitId>,
        shots: NonZeroU64,
    ) -> MeasurementResult<Self> {
        let request = Self {
            qubits,
            physical_qubits: Vec::new(),
            observable: MeasurementObservable::default(),
            mode: MeasurementMode::Destructive,
            collapse: CollapsePolicy::Collapse,
            shots,
            classical_destinations: Vec::new(),
            retain_shots: false,
            retain_counts: true,
            calculate_probabilities: true,
            deterministic_seed: None,
        };

        request.validate()?;
        Ok(request)
    }

    /// Validates the complete request.
    pub fn validate(&self) -> MeasurementResult<()> {
        if self.qubits.is_empty() {
            return Err(invalid_measurement(
                "measurement request must contain at least one qubit",
            ));
        }

        if self.qubits.len() > MAX_MEASUREMENT_QUBITS {
            return Err(invalid_measurement(
                "measurement request exceeds the maximum qubit count",
            ));
        }

        if self.physical_qubits.len() > MAX_MEASUREMENT_QUBITS {
            return Err(invalid_measurement(
                "physical-qubit mapping exceeds the maximum size",
            ));
        }

        if !self.physical_qubits.is_empty()
            && self.physical_qubits.len() != self.qubits.len()
        {
            return Err(invalid_measurement(
                "physical-qubit mapping must have exactly one entry per logical qubit",
            ));
        }

        if self.shots.get() > MAX_SHOTS {
            return Err(invalid_measurement(
                "measurement shot count exceeds the configured maximum",
            ));
        }

        self.observable.validate()?;

        validate_unique_qubits(&self.qubits)?;

        if self.classical_destinations.len() > MAX_CLASSICAL_DESTINATIONS {
            return Err(invalid_measurement(
                "too many classical measurement destinations",
            ));
        }

        validate_destinations(
            &self.qubits,
            &self.classical_destinations,
        )?;

        match self.mode {
            MeasurementMode::Destructive => {
                if matches!(
                    self.collapse,
                    CollapsePolicy::PreserveIfSupported
                ) {
                    return Err(invalid_measurement(
                        "destructive measurement cannot request state preservation",
                    ));
                }
            }

            MeasurementMode::MidCircuit | MeasurementMode::NonDestructive => {}
        }

        Ok(())
    }

    /// Returns the number of measured qubits.
    pub fn qubit_count(&self) -> usize {
        self.qubits.len()
    }

    /// Returns the requested shot count.
    pub fn shot_count(&self) -> u64 {
        self.shots.get()
    }

    /// Returns whether this request requires deterministic simulator sampling.
    pub fn requires_deterministic_sampling(&self) -> bool {
        self.deterministic_seed.is_some()
    }
}

// =============================================================================
// Measurement outcome
// =============================================================================

/// One measured binary value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MeasurementBit {
    /// Classical zero.
    Zero,

    /// Classical one.
    One,
}

impl MeasurementBit {
    /// Converts to a numerical bit.
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
        }
    }

    /// Creates a measurement bit from 0/1.
    pub fn from_u8(value: u8) -> MeasurementResult<Self> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            _ => Err(invalid_measurement(
                "measurement bit must be zero or one",
            )),
        }
    }
}

impl fmt::Display for MeasurementBit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => formatter.write_str("0"),
            Self::One => formatter.write_str("1"),
        }
    }
}

// =============================================================================
// Measurement bit string
// =============================================================================

/// Ordered measurement result.
///
/// Bit index 0 corresponds to the first qubit in `MeasurementRequest::qubits`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeasurementBitString(Vec<u8>);

impl MeasurementBitString {
    /// Creates a validated bit string.
    pub fn new(bits: Vec<u8>) -> MeasurementResult<Self> {
        if bits.len() > MAX_MEASUREMENT_QUBITS {
            return Err(invalid_measurement(
                "measurement result exceeds the maximum supported width",
            ));
        }

        if bits.iter().any(|bit| *bit > 1) {
            return Err(invalid_measurement(
                "measurement result contains a value other than zero or one",
            ));
        }

        Ok(Self(bits))
    }

    /// Creates a zero-filled bit string.
    pub fn zeros(width: usize) -> MeasurementResult<Self> {
        if width > MAX_MEASUREMENT_QUBITS {
            return Err(invalid_measurement(
                "measurement result width exceeds the configured maximum",
            ));
        }

        Ok(Self(vec![0; width]))
    }

    /// Returns the number of bits.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether there are no bits.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns one bit.
    pub fn get(&self, index: usize) -> Option<MeasurementBit> {
        self.0
            .get(index)
            .copied()
            .and_then(|value| MeasurementBit::from_u8(value).ok())
    }

    /// Returns the raw bits.
    pub fn as_bits(&self) -> &[u8] {
        &self.0
    }

    /// Returns a hexadecimal representation when the result fits into bytes.
    ///
    /// The representation is informational; it does not change bit ordering.
    pub fn to_hex_string(&self) -> String {
        if self.0.is_empty() {
            return String::new();
        }

        let mut output = String::with_capacity((self.0.len() + 3) / 4);

        let mut index = 0usize;

        while index < self.0.len() {
            let remaining = self.0.len() - index;
            let width = remaining.min(4);

            let mut nibble = 0u8;

            for offset in 0..width {
                nibble = (nibble << 1) | self.0[index + offset];
            }

            if width < 4 {
                nibble <<= 4 - width;
            }

            let digit = match nibble {
                0..=9 => (b'0' + nibble) as char,
                10..=15 => (b'a' + (nibble - 10)) as char,
                _ => unreachable!(),
            };

            output.push(digit);
            index += width;
        }

        output
    }
}

impl fmt::Display for MeasurementBitString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for bit in &self.0 {
            formatter.write_str(if *bit == 0 { "0" } else { "1" })?;
        }

        Ok(())
    }
}

// =============================================================================
// One measurement shot
// =============================================================================

/// One physical/simulated measurement shot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementShot {
    /// Result bits in request qubit order.
    pub outcome: MeasurementBitString,

    /// Optional classical destination values.
    ///
    /// Each entry corresponds to one `ClassicalDestination` in the request.
    pub classical_values: Vec<MeasurementBit>,

    /// Whether the provider reports that the quantum state collapsed.
    pub state_collapsed: bool,
}

impl MeasurementShot {
    /// Creates a shot.
    pub fn new(
        outcome: MeasurementBitString,
        classical_values: Vec<MeasurementBit>,
        state_collapsed: bool,
    ) -> MeasurementResult<Self> {
        if classical_values.len() > MAX_CLASSICAL_DESTINATIONS {
            return Err(invalid_measurement(
                "too many classical measurement values",
            ));
        }

        Ok(Self {
            outcome,
            classical_values,
            state_collapsed,
        })
    }
}

// =============================================================================
// Histogram
// =============================================================================

/// Aggregate measurement counts.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MeasurementHistogram {
    counts: BTreeMap<MeasurementBitString, u64>,
    total_shots: u64,
}

impl MeasurementHistogram {
    /// Creates an empty histogram.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds one occurrence.
    pub fn record(
        &mut self,
        outcome: MeasurementBitString,
    ) -> MeasurementResult<()> {
        if !self.counts.contains_key(&outcome)
            && self.counts.len() >= MAX_HISTOGRAM_OUTCOMES
        {
            return Err(invalid_measurement(
                "measurement histogram exceeds the maximum outcome count",
            ));
        }

        let current = self.counts.get(&outcome).copied().unwrap_or(0);

        let next = current.checked_add(1).ok_or_else(|| {
            invalid_measurement("measurement histogram count overflow")
        })?;

        self.counts.insert(outcome, next);

        self.total_shots = self.total_shots.checked_add(1).ok_or_else(|| {
            invalid_measurement("measurement histogram shot count overflow")
        })?;

        Ok(())
    }

    /// Adds a pre-aggregated count.
    pub fn record_count(
        &mut self,
        outcome: MeasurementBitString,
        count: u64,
    ) -> MeasurementResult<()> {
        if count == 0 {
            return Ok(());
        }

        if !self.counts.contains_key(&outcome)
            && self.counts.len() >= MAX_HISTOGRAM_OUTCOMES
        {
            return Err(invalid_measurement(
                "measurement histogram exceeds the maximum outcome count",
            ));
        }

        let current = self.counts.get(&outcome).copied().unwrap_or(0);

        let next = current.checked_add(count).ok_or_else(|| {
            invalid_measurement("measurement histogram count overflow")
        })?;

        self.counts.insert(outcome, next);

        self.total_shots = self.total_shots.checked_add(count).ok_or_else(|| {
            invalid_measurement("measurement histogram shot count overflow")
        })?;

        Ok(())
    }

    /// Returns total shots.
    pub const fn total_shots(&self) -> u64 {
        self.total_shots
    }

    /// Returns number of distinct outcomes.
    pub fn outcome_count(&self) -> usize {
        self.counts.len()
    }

    /// Returns one count.
    pub fn count(&self, outcome: &MeasurementBitString) -> u64 {
        self.counts.get(outcome).copied().unwrap_or(0)
    }

    /// Iterates over outcomes and counts.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&MeasurementBitString, &u64)> {
        self.counts.iter()
    }

    /// Returns a probability distribution.
    pub fn probabilities(
        &self,
    ) -> MeasurementResult<MeasurementDistribution> {
        if self.total_shots == 0 {
            return Err(invalid_measurement(
                "cannot construct probabilities from zero shots",
            ));
        }

        let mut probabilities = BTreeMap::new();

        for (outcome, count) in &self.counts {
            probabilities.insert(
                outcome.clone(),
                (*count as f64) / (self.total_shots as f64),
            );
        }

        MeasurementDistribution::new(probabilities)
    }

    /// Validates histogram invariants.
    pub fn validate(&self) -> MeasurementResult<()> {
        let mut total = 0u64;

        for count in self.counts.values() {
            total = total.checked_add(*count).ok_or_else(|| {
                invalid_measurement("measurement histogram total overflow")
            })?;
        }

        if total != self.total_shots {
            return Err(invalid_measurement(
                "measurement histogram total does not match stored shot count",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// Probability distribution
// =============================================================================

/// Normalized measurement probability distribution.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementDistribution {
    probabilities: BTreeMap<MeasurementBitString, f64>,
}

impl MeasurementDistribution {
    /// Creates and validates a distribution.
    pub fn new(
        probabilities: BTreeMap<MeasurementBitString, f64>,
    ) -> MeasurementResult<Self> {
        if probabilities.is_empty() {
            return Err(invalid_measurement(
                "measurement probability distribution cannot be empty",
            ));
        }

        let mut total = 0.0f64;

        for probability in probabilities.values() {
            if !probability.is_finite()
                || *probability < 0.0
                || *probability > 1.0
            {
                return Err(invalid_measurement(
                    "measurement probability must be finite and within [0, 1]",
                ));
            }

            total += *probability;
        }

        if !total.is_finite()
            || (total - 1.0).abs() > DEFAULT_PROBABILITY_TOLERANCE
        {
            return Err(invalid_measurement(
                "measurement probabilities must sum to one",
            ));
        }

        Ok(Self { probabilities })
    }

    /// Returns the probability of an outcome.
    pub fn probability(&self, outcome: &MeasurementBitString) -> f64 {
        self.probabilities
            .get(outcome)
            .copied()
            .unwrap_or(0.0)
    }

    /// Iterates over the distribution.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&MeasurementBitString, &f64)> {
        self.probabilities.iter()
    }

    /// Returns number of outcomes.
    pub fn len(&self) -> usize {
        self.probabilities.len()
    }

    /// Returns whether the distribution is empty.
    pub fn is_empty(&self) -> bool {
        self.probabilities.is_empty()
    }
}

// =============================================================================
// Readout information
// =============================================================================

/// Readout information supplied by a simulator or hardware provider.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct ReadoutMetadata {
    /// Whether the provider reports calibrated readout data.
    pub calibrated: bool,

    /// Optional per-bit assignment-error probabilities.
    ///
    /// Each tuple is:
    ///
    /// `(P(measured 1 | prepared 0), P(measured 0 | prepared 1))`
    pub assignment_error_rates: Vec<(f64, f64)>,

    /// Whether the returned result has already undergone readout mitigation.
    pub mitigated: bool,

    /// Provider-defined calibration identifier.
    pub calibration_id: Option<String>,
}

impl Default for ReadoutMetadata {
    fn default() -> Self {
        Self {
            calibrated: false,
            assignment_error_rates: Vec::new(),
            mitigated: false,
            calibration_id: None,
        }
    }
}

impl ReadoutMetadata {
    /// Validates readout metadata.
    pub fn validate(&self, measured_qubits: usize) -> MeasurementResult<()> {
        if self.assignment_error_rates.len() > measured_qubits {
            return Err(invalid_measurement(
                "readout metadata contains more error rates than measured qubits",
            ));
        }

        for (zero_to_one, one_to_zero) in &self.assignment_error_rates {
            if !zero_to_one.is_finite()
                || !one_to_zero.is_finite()
                || !(0.0..=1.0).contains(zero_to_one)
                || !(0.0..=1.0).contains(one_to_zero)
            {
                return Err(invalid_measurement(
                    "readout error rates must be finite and within [0, 1]",
                ));
            }
        }

        if let Some(calibration_id) = &self.calibration_id {
            validate_text(
                calibration_id,
                MAX_PROVIDER_METADATA_VALUE_LENGTH,
                "calibration identifier",
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Provider metadata
// =============================================================================

/// Safe provider metadata associated with one measurement execution.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MeasurementProviderMetadata {
    entries: BTreeMap<String, String>,
}

impl MeasurementProviderMetadata {
    /// Creates empty metadata.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a safe metadata field.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> MeasurementResult<()> {
        if self.entries.len() >= MAX_PROVIDER_METADATA_ENTRIES {
            return Err(invalid_measurement(
                "measurement provider metadata entry limit exceeded",
            ));
        }

        let key = key.into();
        let value = value.into();

        validate_text(
            &key,
            MAX_PROVIDER_METADATA_KEY_LENGTH,
            "provider metadata key",
        )?;

        validate_text(
            &value,
            MAX_PROVIDER_METADATA_VALUE_LENGTH,
            "provider metadata value",
        )?;

        if is_sensitive_metadata_key(&key) {
            return Err(invalid_measurement(
                "sensitive provider metadata cannot be stored",
            ));
        }

        self.entries.insert(key, value);

        Ok(())
    }

    /// Gets a metadata value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    /// Iterates over metadata.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&String, &String)> {
        self.entries.iter()
    }
}

// =============================================================================
// Measurement execution
// =============================================================================

/// Provider-neutral measurement execution returned by a simulator or QPU.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementExecution {
    /// Request schema identifier.
    pub schema_id: &'static str,

    /// Request schema version.
    pub schema_version: u16,

    /// Aggregate measurement counts, if supplied.
    pub histogram: Option<MeasurementHistogram>,

    /// Individual shots, if requested and available.
    pub shots: Option<Vec<MeasurementShot>>,

    /// Readout metadata.
    pub readout: ReadoutMetadata,

    /// Whether the provider reports that the state was collapsed.
    pub state_collapsed: bool,

    /// Whether the measurement was destructive.
    pub destructive: bool,

    /// Opaque provider execution identifier.
    pub provider_execution_id: Option<String>,

    /// Provider metadata.
    pub provider_metadata: MeasurementProviderMetadata,
}

impl MeasurementExecution {
    /// Creates an empty execution result.
    pub fn empty() -> Self {
        Self {
            schema_id: MEASUREMENT_SCHEMA_ID,
            schema_version: MEASUREMENT_SCHEMA_VERSION,
            histogram: None,
            shots: None,
            readout: ReadoutMetadata::default(),
            state_collapsed: false,
            destructive: false,
            provider_execution_id: None,
            provider_metadata: MeasurementProviderMetadata::new(),
        }
    }

    /// Validates the execution against the request.
    pub fn validate(
        &self,
        request: &MeasurementRequest,
    ) -> MeasurementResult<()> {
        if self.schema_id != MEASUREMENT_SCHEMA_ID {
            return Err(invalid_measurement(
                "measurement execution schema identifier is incompatible",
            ));
        }

        if self.schema_version != MEASUREMENT_SCHEMA_VERSION {
            return Err(invalid_measurement(
                "measurement execution schema version is incompatible",
            ));
        }

        if self.histogram.is_none() && self.shots.is_none() {
            return Err(invalid_measurement(
                "measurement execution contains neither counts nor raw shots",
            ));
        }

        if let Some(histogram) = &self.histogram {
            histogram.validate()?;

            if histogram.total_shots() != request.shot_count() {
                return Err(invalid_measurement(
                    "measurement histogram shot count does not match request",
                ));
            }

            for outcome in histogram.counts.keys() {
                if outcome.len() != request.qubit_count() {
                    return Err(invalid_measurement(
                        "measurement histogram outcome width does not match request",
                    ));
                }
            }
        }

        if let Some(shots) = &self.shots {
            let expected = request.shot_count();

            if shots.len() as u64 != expected {
                return Err(invalid_measurement(
                    "raw measurement shot count does not match request",
                ));
            }

            for shot in shots {
                if shot.outcome.len() != request.qubit_count() {
                    return Err(invalid_measurement(
                        "raw measurement outcome width does not match request",
                    ));
                }

                if shot.classical_values.len()
                    != request.classical_destinations.len()
                {
                    return Err(invalid_measurement(
                        "raw classical measurement result width does not match request",
                    ));
                }
            }
        }

        self.readout.validate(request.qubit_count())?;

        Ok(())
    }

    /// Converts raw shots into a histogram.
    pub fn histogram_from_shots(
        &self,
    ) -> MeasurementResult<MeasurementHistogram> {
        let shots = self.shots.as_ref().ok_or_else(|| {
            invalid_measurement(
                "raw measurement shots are unavailable",
            )
        })?;

        let mut histogram = MeasurementHistogram::new();

        for shot in shots {
            histogram.record(shot.outcome.clone())?;
        }

        Ok(histogram)
    }

    /// Returns a histogram from either provider counts or raw shots.
    pub fn effective_histogram(
        &self,
    ) -> MeasurementResult<MeasurementHistogram> {
        if let Some(histogram) = &self.histogram {
            return Ok(histogram.clone());
        }

        self.histogram_from_shots()
    }
}

// =============================================================================
// Final measurement result
// =============================================================================

/// Complete normalized result exposed to Zamani execution consumers.
#[derive(Debug, Clone, PartialEq)]
pub struct FinalMeasurementResult {
    /// Measurement request.
    pub request: MeasurementRequest,

    /// Aggregate counts.
    pub histogram: MeasurementHistogram,

    /// Optional normalized probabilities.
    pub probabilities: Option<MeasurementDistribution>,

    /// Optional raw shots.
    pub shots: Option<Vec<MeasurementShot>>,

    /// Readout metadata.
    pub readout: ReadoutMetadata,

    /// Whether state collapse occurred.
    pub state_collapsed: bool,

    /// Whether the operation was destructive.
    pub destructive: bool,

    /// Opaque provider execution identifier.
    pub provider_execution_id: Option<String>,

    /// Provider metadata.
    pub provider_metadata: MeasurementProviderMetadata,
}

impl FinalMeasurementResult {
    /// Builds a normalized final result from a provider execution.
    pub fn from_execution(
        request: MeasurementRequest,
        execution: MeasurementExecution,
    ) -> MeasurementResult<Self> {
        request.validate()?;
        execution.validate(&request)?;

        let histogram = execution.effective_histogram()?;

        let probabilities = if request.calculate_probabilities {
            Some(histogram.probabilities()?)
        } else {
            None
        };

        if request.retain_counts && histogram.total_shots() == 0 {
            return Err(invalid_measurement(
                "measurement result requested counts but contains zero shots",
            ));
        }

        let shots = if request.retain_shots {
            execution.shots
        } else {
            None
        };

        Ok(Self {
            request,
            histogram,
            probabilities,
            shots,
            readout: execution.readout,
            state_collapsed: execution.state_collapsed,
            destructive: execution.destructive,
            provider_execution_id: execution.provider_execution_id,
            provider_metadata: execution.provider_metadata,
        })
    }

    /// Returns the number of measured shots.
    pub const fn shots(&self) -> u64 {
        self.histogram.total_shots()
    }

    /// Returns the count for one outcome.
    pub fn count(&self, outcome: &MeasurementBitString) -> u64 {
        self.histogram.count(outcome)
    }

    /// Returns the probability of one outcome.
    pub fn probability(&self, outcome: &MeasurementBitString) -> f64 {
        self.probabilities
            .as_ref()
            .map(|distribution| distribution.probability(outcome))
            .unwrap_or_else(|| {
                if self.shots() == 0 {
                    0.0
                } else {
                    self.count(outcome) as f64 / self.shots() as f64
                }
            })
    }

    /// Computes the expectation value of the parity observable:
    ///
    /// `+1` for even parity and `-1` for odd parity.
    ///
    /// This is useful for Z-basis multi-qubit parity measurements.
    pub fn parity_expectation(&self) -> MeasurementResult<f64> {
        if self.shots() == 0 {
            return Err(invalid_measurement(
                "cannot compute expectation value from zero shots",
            ));
        }

        let mut expectation = 0.0f64;

        for (outcome, count) in self.histogram.iter() {
            let parity = outcome
                .as_bits()
                .iter()
                .fold(0u8, |accumulator, bit| accumulator ^ *bit);

            let eigenvalue = if parity == 0 { 1.0 } else { -1.0 };

            expectation +=
                eigenvalue * (*count as f64 / self.shots() as f64);
        }

        if !expectation.is_finite() {
            return Err(invalid_measurement(
                "measurement expectation value is non-finite",
            ));
        }

        Ok(expectation)
    }

    /// Computes the expectation value of a single measured qubit.
    ///
    /// Bit `0` is treated as eigenvalue `+1`, bit `1` as eigenvalue `-1`.
    pub fn single_qubit_expectation(
        &self,
        qubit_index: usize,
    ) -> MeasurementResult<f64> {
        if qubit_index >= self.request.qubit_count() {
            return Err(invalid_measurement(
                "requested expectation-value qubit is out of range",
            ));
        }

        if self.shots() == 0 {
            return Err(invalid_measurement(
                "cannot compute expectation value from zero shots",
            ));
        }

        let mut expectation = 0.0f64;

        for (outcome, count) in self.histogram.iter() {
            let bit = outcome
                .get(qubit_index)
                .ok_or_else(|| {
                    invalid_measurement(
                        "measurement outcome does not contain requested qubit",
                    )
                })?;

            let eigenvalue =
                if bit == MeasurementBit::Zero { 1.0 } else { -1.0 };

            expectation +=
                eigenvalue * (*count as f64 / self.shots() as f64);
        }

        if !expectation.is_finite() {
            return Err(invalid_measurement(
                "measurement expectation value is non-finite",
            ));
        }

        Ok(expectation)
    }
}

// =============================================================================
// Measurement capabilities
// =============================================================================

/// Measurement capabilities exposed by a state representation or hardware
/// provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeasurementCapabilities(u64);

impl MeasurementCapabilities {
    /// No measurement capabilities.
    pub const NONE: Self = Self(0);

    /// Computational/Z-basis measurement.
    pub const Z_BASIS: Self = Self(1 << 0);

    /// X-basis measurement.
    pub const X_BASIS: Self = Self(1 << 1);

    /// Y-basis measurement.
    pub const Y_BASIS: Self = Self(1 << 2);

    /// Arbitrary Bloch-axis projective measurement.
    pub const ARBITRARY_PROJECTIVE: Self = Self(1 << 3);

    /// Provider-defined measurement bases.
    pub const PROVIDER_DEFINED: Self = Self(1 << 4);

    /// Multiple qubits in one measurement request.
    pub const MULTI_QUBIT: Self = Self(1 << 5);

    /// Mid-circuit measurement.
    pub const MID_CIRCUIT: Self = Self(1 << 6);

    /// Non-destructive measurement.
    pub const NON_DESTRUCTIVE: Self = Self(1 << 7);

    /// Quantum-state collapse.
    pub const COLLAPSE: Self = Self(1 << 8);

    /// Classical destination mapping.
    pub const CLASSICAL_DESTINATION: Self = Self(1 << 9);

    /// Raw individual shots.
    pub const RAW_SHOTS: Self = Self(1 << 10);

    /// Aggregate histogram results.
    pub const HISTOGRAM: Self = Self(1 << 11);

    /// Direct probability distribution.
    pub const PROBABILITIES: Self = Self(1 << 12);

    /// Deterministic seeded simulator sampling.
    pub const DETERMINISTIC_SAMPLING: Self = Self(1 << 13);

    /// Provider-controlled physical randomness.
    pub const PROVIDER_RANDOMNESS: Self = Self(1 << 14);

    /// Readout calibration metadata.
    pub const READOUT_CALIBRATION: Self = Self(1 << 15);

    /// Hardware readout mitigation is already applied.
    pub const READOUT_MITIGATION: Self = Self(1 << 16);

    /// Dynamic classical feedback.
    pub const CLASSICAL_FEEDBACK: Self = Self(1 << 17);

    /// Distributed measurement.
    pub const DISTRIBUTED: Self = Self(1 << 18);

    /// Returns raw bits.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Creates capabilities from raw bits.
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Tests whether all requested capabilities exist.
    pub const fn contains(self, required: Self) -> bool {
        self.0 & required.0 == required.0
    }

    /// Combines capabilities.
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Returns whether no capabilities exist.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl std::ops::BitOr for MeasurementCapabilities {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl std::ops::BitOrAssign for MeasurementCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

// =============================================================================
// Random source
// =============================================================================

/// Explicit randomness source for simulator-side measurement sampling.
///
/// This deliberately does not depend on the `rand` crate, so the memory
/// measurement contract remains lightweight and deterministic under testing.
pub trait MeasurementRandomSource: Send {
    /// Returns the next uniformly distributed 64-bit value.
    fn next_u64(&mut self) -> u64;

    /// Returns a uniformly distributed value in `[0, 1)`.
    fn next_unit_f64(&mut self) -> f64 {
        const SCALE: f64 = 1.0 / ((u64::MAX as f64) + 1.0);

        (self.next_u64() as f64) * SCALE
    }
}

/// Small deterministic PRNG suitable for reproducible simulator tests.
///
/// This is NOT intended as a cryptographic random-number generator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeterministicMeasurementRng {
    state: u64,
}

impl DeterministicMeasurementRng {
    /// Creates a deterministic RNG.
    ///
    /// A zero seed is replaced with a fixed non-zero state.
    pub const fn new(seed: u64) -> Self {
        let state = if seed == 0 {
            0x9E37_79B9_7F4A_7C15
        } else {
            seed
        };

        Self { state }
    }

    /// Returns the current internal state.
    pub const fn state(&self) -> u64 {
        self.state
    }
}

impl MeasurementRandomSource for DeterministicMeasurementRng {
    fn next_u64(&mut self) -> u64 {
        // xorshift64*.
        let mut x = self.state;

        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;

        self.state = x;

        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

// =============================================================================
// Distribution sampler
// =============================================================================

/// Samples measurement outcomes from an already-normalized probability
/// distribution.
///
/// State representations calculate the actual probabilities; this utility
/// performs only the statistical sampling.
pub fn sample_distribution(
    distribution: &MeasurementDistribution,
    shots: NonZeroU64,
    rng: &mut dyn MeasurementRandomSource,
) -> MeasurementResult<MeasurementHistogram> {
    let mut histogram = MeasurementHistogram::new();

    for _ in 0..shots.get() {
        let random = rng.next_unit_f64();

        if !random.is_finite() || !(0.0..1.0).contains(&random) {
            return Err(invalid_measurement(
                "measurement RNG returned an invalid sample",
            ));
        }

        let mut cumulative = 0.0f64;
        let mut selected: Option<MeasurementBitString> = None;

        for (outcome, probability) in distribution.iter() {
            cumulative += *probability;

            if random < cumulative {
                selected = Some(outcome.clone());
                break;
            }
        }

        let selected = selected.ok_or_else(|| {
            invalid_measurement(
                "measurement distribution failed to select an outcome",
            )
        })?;

        histogram.record(selected)?;
    }

    histogram.validate()?;

    Ok(histogram)
}

// =============================================================================
// Provider contract
// =============================================================================

/// Provider-neutral measurement execution contract.
///
/// Implementations may be:
///
/// - state-vector simulators;
/// - density-matrix simulators;
/// - stabilizer simulators;
/// - sparse simulators;
/// - tensor-network simulators;
/// - CPU backends;
/// - GPU backends;
/// - distributed simulators;
/// - physical QPUs;
/// - cloud QPU adapters;
/// - hardware emulators.
///
/// The provider owns the actual measurement implementation.
pub trait MeasurementProvider: Send + Sync {
    /// Returns provider measurement capabilities.
    fn capabilities(&self) -> MeasurementCapabilities;

    /// Executes a measurement request.
    fn measure(
        &mut self,
        request: &MeasurementRequest,
    ) -> MeasurementResult<MeasurementExecution>;

    /// Optional provider name.
    fn provider_name(&self) -> Option<&str> {
        None
    }

    /// Validates whether the provider can execute a request.
    fn supports_request(
        &self,
        request: &MeasurementRequest,
    ) -> MeasurementResult<()> {
        request.validate()?;

        let capabilities = self.capabilities();

        validate_capabilities(request, capabilities)?;

        Ok(())
    }

    /// Executes and normalizes a measurement request.
    fn measure_final(
        &mut self,
        request: MeasurementRequest,
    ) -> MeasurementResult<FinalMeasurementResult> {
        self.supports_request(&request)?;

        let execution = self.measure(&request)?;

        FinalMeasurementResult::from_execution(
            request,
            execution,
        )
    }
}

// =============================================================================
// Capability validation
// =============================================================================

/// Validates a measurement request against provider capabilities.
pub fn validate_capabilities(
    request: &MeasurementRequest,
    capabilities: MeasurementCapabilities,
) -> MeasurementResult<()> {
    let observable_capability = match &request.observable {
        MeasurementObservable::Standard(MeasurementBasis::Z) => {
            MeasurementCapabilities::Z_BASIS
        }

        MeasurementObservable::Standard(MeasurementBasis::X) => {
            MeasurementCapabilities::X_BASIS
        }

        MeasurementObservable::Standard(MeasurementBasis::Y) => {
            MeasurementCapabilities::Y_BASIS
        }

        MeasurementObservable::BlochAxis(_) => {
            MeasurementCapabilities::ARBITRARY_PROJECTIVE
        }

        MeasurementObservable::ProviderDefined { .. } => {
            MeasurementCapabilities::PROVIDER_DEFINED
        }
    };

    if !capabilities.contains(observable_capability) {
        return Err(MemoryError::unsupported_operation(
            "requested measurement basis",
            "measurement provider",
        ));
    }

    if request.qubits.len() > 1
        && !capabilities.contains(MeasurementCapabilities::MULTI_QUBIT)
    {
        return Err(MemoryError::unsupported_operation(
            "multi-qubit measurement",
            "measurement provider",
        ));
    }

    if request.mode == MeasurementMode::MidCircuit
        && !capabilities.contains(MeasurementCapabilities::MID_CIRCUIT)
    {
        return Err(MemoryError::unsupported_operation(
            "mid-circuit measurement",
            "measurement provider",
        ));
    }

    if request.mode == MeasurementMode::NonDestructive
        && !capabilities.contains(MeasurementCapabilities::NON_DESTRUCTIVE)
    {
        return Err(MemoryError::unsupported_operation(
            "non-destructive measurement",
            "measurement provider",
        ));
    }

    if request.collapse == CollapsePolicy::Collapse
        && !capabilities.contains(MeasurementCapabilities::COLLAPSE)
    {
        return Err(MemoryError::unsupported_operation(
            "measurement collapse",
            "measurement provider",
        ));
    }

    if !request.classical_destinations.is_empty()
        && !capabilities.contains(
            MeasurementCapabilities::CLASSICAL_DESTINATION,
        )
    {
        return Err(MemoryError::unsupported_operation(
            "classical measurement destinations",
            "measurement provider",
        ));
    }

    if request.retain_shots
        && !capabilities.contains(MeasurementCapabilities::RAW_SHOTS)
    {
        return Err(MemoryError::unsupported_operation(
            "raw measurement shots",
            "measurement provider",
        ));
    }

    if request.retain_counts
        && !capabilities.contains(MeasurementCapabilities::HISTOGRAM)
    {
        return Err(MemoryError::unsupported_operation(
            "measurement histogram",
            "measurement provider",
        ));
    }

    if request.calculate_probabilities
        && !capabilities.contains(MeasurementCapabilities::PROBABILITIES)
        && !capabilities.contains(MeasurementCapabilities::HISTOGRAM)
        && !capabilities.contains(MeasurementCapabilities::RAW_SHOTS)
    {
        return Err(MemoryError::unsupported_operation(
            "measurement probability calculation",
            "measurement provider",
        ));
    }

    if request.requires_deterministic_sampling()
        && !capabilities.contains(
            MeasurementCapabilities::DETERMINISTIC_SAMPLING,
        )
    {
        return Err(MemoryError::unsupported_operation(
            "deterministic measurement sampling",
            "measurement provider",
        ));
    }

    Ok(())
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_unique_qubits(
    qubits: &[QubitId],
) -> MeasurementResult<()> {
    let mut unique = BTreeSet::new();

    for qubit in qubits {
        if !unique.insert(*qubit) {
            return Err(invalid_measurement(
                "measurement request contains a duplicate logical qubit",
            ));
        }
    }

    Ok(())
}

fn validate_destinations(
    qubits: &[QubitId],
    destinations: &[ClassicalDestination],
) -> MeasurementResult<()> {
    let measured: BTreeSet<QubitId> =
        qubits.iter().copied().collect();

    let mut classical = BTreeSet::new();

    for destination in destinations {
        if !measured.contains(&destination.qubit) {
            return Err(invalid_measurement(
                "classical destination references a qubit that is not measured",
            ));
        }

        if !classical.insert(destination.classical_bit) {
            return Err(invalid_measurement(
                "multiple measurement results target the same classical bit",
            ));
        }
    }

    Ok(())
}

fn validate_text(
    value: &str,
    maximum: usize,
    field: &str,
) -> MeasurementResult<()> {
    if value.is_empty() {
        return Err(invalid_measurement(
            "measurement text field cannot be empty",
        ));
    }

    if value.len() > maximum {
        return Err(invalid_measurement(
            field,
        ));
    }

    if value.trim() != value {
        return Err(invalid_measurement(
            "measurement text field must not contain leading or trailing whitespace",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_measurement(
            "measurement text field contains a control character",
        ));
    }

    Ok(())
}

fn is_sensitive_metadata_key(key: &str) -> bool {
    let normalized = key.to_ascii_lowercase();

    normalized.contains("token")
        || normalized.contains("password")
        || normalized.contains("secret")
        || normalized.contains("private_key")
        || normalized.contains("authorization")
        || normalized.contains("api_key")
        || normalized.contains("credential")
}

fn invalid_measurement(message: &str) -> MemoryError {
    MemoryError::invalid_argument(message)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct FixedProvider {
        capabilities: MeasurementCapabilities,
    }

    impl MeasurementProvider for FixedProvider {
        fn capabilities(&self) -> MeasurementCapabilities {
            self.capabilities
        }

        fn measure(
            &mut self,
            request: &MeasurementRequest,
        ) -> MeasurementResult<MeasurementExecution> {
            self.supports_request(request)?;

            let mut histogram = MeasurementHistogram::new();

            let zero =
                MeasurementBitString::zeros(request.qubit_count())?;

            histogram.record_count(
                zero,
                request.shot_count(),
            )?;

            let mut execution = MeasurementExecution::empty();

            execution.histogram = Some(histogram);
            execution.state_collapsed =
                matches!(request.collapse, CollapsePolicy::Collapse);
            execution.destructive =
                request.mode == MeasurementMode::Destructive;

            Ok(execution)
        }
    }

    #[test]
    fn computational_measurement_request_is_valid() {
        let q0 = QubitId::from(0usize);
        let q1 = QubitId::from(1usize);

        let request = MeasurementRequest::new(
            vec![q0, q1],
            NonZeroU64::new(100).expect("non-zero"),
        )
        .expect("valid request");

        assert_eq!(request.qubit_count(), 2);
        assert_eq!(request.shot_count(), 100);
    }

    #[test]
    fn duplicate_qubits_are_rejected() {
        let q0 = QubitId::from(0usize);

        let result = MeasurementRequest::new(
            vec![q0, q0],
            NonZeroU64::new(1).expect("non-zero"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn invalid_bloch_axis_is_rejected() {
        let result = BlochAxis::new(2.0, 0.0, 0.0);

        assert!(result.is_err());
    }

    #[test]
    fn histogram_counts_are_correct() {
        let zero =
            MeasurementBitString::new(vec![0]).expect("valid");
        let one =
            MeasurementBitString::new(vec![1]).expect("valid");

        let mut histogram = MeasurementHistogram::new();

        histogram
            .record_count(zero.clone(), 60)
            .expect("record");

        histogram
            .record_count(one.clone(), 40)
            .expect("record");

        assert_eq!(histogram.total_shots(), 100);
        assert_eq!(histogram.count(&zero), 60);
        assert_eq!(histogram.count(&one), 40);
    }

    #[test]
    fn histogram_probabilities_are_normalized() {
        let zero =
            MeasurementBitString::new(vec![0]).expect("valid");
        let one =
            MeasurementBitString::new(vec![1]).expect("valid");

        let mut histogram = MeasurementHistogram::new();

        histogram
            .record_count(zero.clone(), 50)
            .expect("record");

        histogram
            .record_count(one.clone(), 50)
            .expect("record");

        let distribution =
            histogram.probabilities().expect("probabilities");

        assert!((distribution.probability(&zero) - 0.5).abs() < 1.0e-12);
        assert!((distribution.probability(&one) - 0.5).abs() < 1.0e-12);
    }

    #[test]
    fn deterministic_sampling_is_reproducible() {
        let zero =
            MeasurementBitString::new(vec![0]).expect("valid");
        let one =
            MeasurementBitString::new(vec![1]).expect("valid");

        let mut probabilities = BTreeMap::new();
        probabilities.insert(zero, 0.5);
        probabilities.insert(one, 0.5);

        let distribution =
            MeasurementDistribution::new(probabilities)
                .expect("valid distribution");

        let mut rng_a =
            DeterministicMeasurementRng::new(1234);

        let mut rng_b =
            DeterministicMeasurementRng::new(1234);

        let a = sample_distribution(
            &distribution,
            NonZeroU64::new(1000).expect("non-zero"),
            &mut rng_a,
        )
        .expect("sampling");

        let b = sample_distribution(
            &distribution,
            NonZeroU64::new(1000).expect("non-zero"),
            &mut rng_b,
        )
        .expect("sampling");

        assert_eq!(a, b);
    }

    #[test]
    fn parity_expectation_for_all_zero_is_one() {
        let q0 = QubitId::from(0usize);
        let q1 = QubitId::from(1usize);

        let request = MeasurementRequest::new(
            vec![q0, q1],
            NonZeroU64::new(100).expect("non-zero"),
        )
        .expect("request");

        let outcome =
            MeasurementBitString::new(vec![0, 0])
                .expect("outcome");

        let mut histogram = MeasurementHistogram::new();

        histogram
            .record_count(outcome, 100)
            .expect("record");

        let execution = MeasurementExecution {
            schema_id: MEASUREMENT_SCHEMA_ID,
            schema_version: MEASUREMENT_SCHEMA_VERSION,
            histogram: Some(histogram),
            shots: None,
            readout: ReadoutMetadata::default(),
            state_collapsed: true,
            destructive: true,
            provider_execution_id: None,
            provider_metadata: MeasurementProviderMetadata::new(),
        };

        let result =
            FinalMeasurementResult::from_execution(
                request,
                execution,
            )
            .expect("result");

        assert!(
            (result.parity_expectation().expect("expectation") - 1.0)
                .abs()
                < DEFAULT_EXPECTATION_TOLERANCE
        );
    }

    #[test]
    fn provider_capability_validation_rejects_unsupported_basis() {
        let q0 = QubitId::from(0usize);

        let mut request = MeasurementRequest::new(
            vec![q0],
            NonZeroU64::new(1).expect("non-zero"),
        )
        .expect("request");

        request.observable =
            MeasurementObservable::Standard(MeasurementBasis::X);

        let capabilities =
            MeasurementCapabilities::Z_BASIS
                | MeasurementCapabilities::HISTOGRAM
                | MeasurementCapabilities::COLLAPSE;

        assert!(
            validate_capabilities(&request, capabilities)
                .is_err()
        );
    }

    #[test]
    fn provider_can_execute_final_measurement() {
        let q0 = QubitId::from(0usize);

        let request = MeasurementRequest::new(
            vec![q0],
            NonZeroU64::new(10).expect("non-zero"),
        )
        .expect("request");

        let capabilities =
            MeasurementCapabilities::Z_BASIS
                | MeasurementCapabilities::COLLAPSE
                | MeasurementCapabilities::HISTOGRAM
                | MeasurementCapabilities::PROBABILITIES;

        let mut provider =
            FixedProvider { capabilities };

        let result =
            provider.measure_final(request)
                .expect("measurement");

        assert_eq!(result.shots(), 10);
        assert!(result.probabilities.is_some());
        assert!(result.state_collapsed);
    }

    #[test]
    fn sensitive_provider_metadata_is_rejected() {
        let mut metadata =
            MeasurementProviderMetadata::new();

        assert!(
            metadata
                .insert("api_key", "secret")
                .is_err()
        );
    }

    #[test]
    fn measurement_bit_string_rejects_invalid_bits() {
        assert!(
            MeasurementBitString::new(vec![0, 1, 2]).is_err()
        );
    }

    #[test]
    fn raw_shots_can_be_converted_to_histogram() {
        let outcome =
            MeasurementBitString::new(vec![1])
                .expect("outcome");

        let shot =
            MeasurementShot::new(
                outcome.clone(),
                Vec::new(),
                true,
            )
            .expect("shot");

        let execution = MeasurementExecution {
            schema_id: MEASUREMENT_SCHEMA_ID,
            schema_version: MEASUREMENT_SCHEMA_VERSION,
            histogram: None,
            shots: Some(vec![
                shot.clone(),
                shot,
            ]),
            readout: ReadoutMetadata::default(),
            state_collapsed: true,
            destructive: true,
            provider_execution_id: None,
            provider_metadata: MeasurementProviderMetadata::new(),
        };

        let histogram =
            execution
                .histogram_from_shots()
                .expect("histogram");

        assert_eq!(histogram.total_shots(), 2);
        assert_eq!(histogram.count(&outcome), 2);
    }
}