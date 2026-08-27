//! Zamani Quantum — Canonical Hardware Result Model
//!
//! Production-grade, provider-neutral result boundary for
//! `crate::quantum::hardware`.
//!
//! # Responsibility
//!
//! This module defines the canonical representation of results produced by
//! quantum execution targets.
//!
//! It supports results originating from:
//!
//! - gate-model QPUs;
//! - dynamic circuits;
//! - simulators;
//! - hardware emulators;
//! - pulse-level systems;
//! - analog quantum processors;
//! - quantum annealers;
//! - logical/fault-tolerant processors;
//! - photonic/bosonic systems;
//! - continuous-variable systems;
//! - distributed quantum systems;
//! - future provider-specific execution targets.
//!
//! The canonical result model supports:
//!
//! - measurement counts;
//! - probabilities;
//! - raw samples;
//! - bitstrings;
//! - expectation values;
//! - observables;
//! - amplitudes;
//! - state vectors;
//! - density matrices;
//! - classical registers;
//! - logical measurements;
//! - analog observables;
//! - annealing samples;
//! - pulse/acquisition data;
//! - provider-neutral metadata;
//! - execution provenance;
//! - calibration provenance;
//! - backend provenance;
//! - compiler/transpiler provenance;
//! - deterministic seeds;
//! - shot accounting;
//! - validation;
//! - result integrity checks;
//! - partial-result detection;
//! - normalized result semantics.
//!
//! # Explicit non-responsibilities
//!
//! This module does NOT:
//!
//! - communicate with providers;
//! - authenticate providers;
//! - store credentials;
//! - submit jobs;
//! - poll jobs;
//! - cancel jobs;
//! - schedule jobs;
//! - route circuits;
//! - transpile programs;
//! - parse OpenQASM;
//! - generate QIR;
//! - perform benchmarking mathematics;
//! - perform error correction;
//! - perform error mitigation;
//! - acquire calibration data;
//! - persist results;
//! - define provider-specific result types.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! Quantum Frontend
//!       |
//!       v
//! Zamani Quantum IR
//!       |
//!       v
//! compilation / optimization / routing / scheduling
//!       |
//!       v
//! provider-neutral execution
//!       |
//!       v
//! QuantumBackendAdapter
//!       |
//!       v
//! provider execution
//!       |
//!       v
//! raw provider result
//!       |
//!       v
//! adapter normalization
//!       |
//!       v
//! QuantumExecutionResult
//!       |
//!       +--------------------+-------------------+
//!       |                    |                   |
//!       v                    v                   v
//! benchmarking          applications       persistence
//! ```
//!
//! # Critical semantic rule
//!
//! A result is not merely a map of bitstrings to integers.
//!
//! Real quantum hardware may produce:
//!
//! - samples;
//! - probabilities;
//! - expectation values;
//! - amplitudes;
//! - state vectors;
//! - density matrices;
//! - logical measurements;
//! - analog observables;
//! - annealing solutions;
//! - pulse acquisitions.
//!
//! Therefore `QuantumExecutionResult` is a tagged, provider-neutral result
//! envelope containing one or more typed result payloads.
//!
//! # Result integrity
//!
//! A provider adapter MUST normalize provider output into this model before
//! exposing it to higher layers.
//!
//! A result MUST NOT be considered valid merely because it can be parsed.
//!
//! Validation checks:
//!
//! - finite floating-point values;
//! - non-negative probabilities;
//! - normalized probability distributions where required;
//! - non-negative shot counts;
//! - shot accounting;
//! - valid dimensions;
//! - valid complex amplitudes;
//! - matrix dimensions;
//! - backend/job provenance;
//! - result status;
//! - duplicate-key semantics;
//! - absence of contradictory terminal state;
//! - bounded collection sizes.
//!
//! # Provenance
//!
//! Every production result should be traceable to:
//!
//! - backend;
//! - provider;
//! - job;
//! - request;
//! - calibration snapshot when available;
//! - topology version when available;
//! - instruction-set version when available;
//! - compiler version when available;
//! - adapter version;
//! - provider API version.
//!
//! This is essential for reproducibility and quantum benchmarking.
//!
//! # Security
//!
//! Result metadata MUST NOT contain:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - authorization headers;
//! - cookies;
//! - secret credentials.
//!
//! Provider adapters are responsible for removing provider secrets before
//! constructing a result.
//!
//! Program payloads are never stored by this module.
//!
//! # Determinism
//!
//! Externally observable mappings use `BTreeMap` rather than `HashMap`.
//!
//! Ordering of:
//!
//! - measurement outcomes;
//! - classical registers;
//! - observables;
//! - metadata;
//! - provenance fields
//!
//! is therefore deterministic.
//!
//! Floating-point validation never relies on nondeterministic provider state.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no unsafe Rust.
//!
//! # Integration contract
//!
//! This module intentionally depends only on the Rust standard library.
//!
//! It therefore remains independently complete before:
//!
//! - `execution.rs`;
//! - `job.rs`;
//! - `queue.rs`;
//! - `provider.rs`;
//! - adapters;
//! - `serialization.rs`;
//! - benchmarking;
//! - Danga
//!
//! are integrated.
//!
//! Future modules consume this model:
//!
//! `execution.rs`
//!     Accepts `QuantumExecutionResult` from adapters and performs lifecycle
//!     integrity checks.
//!
//! `backend.rs`
//!     Existing `ExecutionResult` compatibility semantics should eventually
//!     be bridged/re-exported to this canonical model.
//!
//! `backend_trait.rs`
//!     Provider adapters normalize provider output into this model.
//!
//! `job.rs`
//!     Associates result identity with job lifecycle.
//!
//! `serialization.rs`
//!     Owns wire serialization of this model.
//!
//! `benchmarking`
//!     Consumes counts, probabilities, samples, expectations and provenance.
//!
//! `Danga`
//!     Exposes normalized results through the Zamani quantum command surface.
//!
//! `adapters/*`
//!     Convert provider-native result formats into this model.
//!
//! Adding a provider MUST NOT require changing this file.
//!
//! # No-reedit contract
//!
//! This file owns the semantic definition of a quantum execution result.
//!
//! Future modules must adapt to this contract rather than requiring this file
//! to be changed for provider-specific result formats.
//!
//! New provider result formats belong in adapters.
//!
//! New serialization formats belong in `serialization.rs`.
//!
//! New benchmark metrics belong in benchmarking.
//!
//! New execution lifecycle states belong in `job.rs`/`execution.rs`.
//!
//! # Stability
//!
//! The public result model is provider-neutral.
//!
//! Provider-specific fields must be represented through bounded metadata or
//! adapter-local structures and must never become mandatory fields here.
//!
//! ```text
//! raw provider result
//!       |
//!       v
//! provider adapter
//!       |
//!       v
//! normalized QuantumExecutionResult
//!       |
//!       +------------+-------------+
//!       |            |             |
//!       v            v             v
//! execution      benchmark     application
//! ```

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier.
pub const RESULT_SCHEMA_ID: &str =
    "zamani.quantum.hardware.result";

/// Semantic version of the canonical result schema.
///
/// Increment this only when the semantic meaning of the public result
/// contract changes incompatibly.
pub const RESULT_SCHEMA_VERSION: u16 = 1;

/// Maximum result metadata entries.
pub const MAX_RESULT_METADATA_ENTRIES: usize = 4096;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 256;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum measurement outcomes in one result.
pub const MAX_MEASUREMENT_OUTCOMES: usize = 10_000_000;

/// Maximum raw samples in one result.
pub const MAX_SAMPLES: usize = 10_000_000;

/// Maximum amplitudes in one result.
pub const MAX_AMPLITUDES: usize = 67_108_864;

/// Maximum state-vector amplitudes.
///
/// This corresponds to 2^26 complex amplitudes and deliberately prevents
/// accidental unbounded memory allocation through malformed provider data.
pub const MAX_STATE_VECTOR_AMPLITUDES: usize = 67_108_864;

/// Maximum density-matrix elements.
pub const MAX_DENSITY_MATRIX_ELEMENTS: usize = 536_870_912;

/// Maximum classical register values.
pub const MAX_CLASSICAL_REGISTER_VALUES: usize = 10_000_000;

/// Maximum observables.
pub const MAX_OBSERVABLES: usize = 1_000_000;

/// Maximum annealing samples.
pub const MAX_ANNEALING_SAMPLES: usize = 10_000_000;

/// Maximum pulse acquisition samples.
pub const MAX_ACQUISITION_SAMPLES: usize = 100_000_000;

/// Maximum logical measurement records.
pub const MAX_LOGICAL_MEASUREMENTS: usize = 10_000_000;

/// Floating-point tolerance used for probability normalization.
pub const PROBABILITY_NORMALIZATION_TOLERANCE: f64 = 1.0e-9;

/// Floating-point tolerance used for matrix/state normalization.
pub const STATE_NORMALIZATION_TOLERANCE: f64 = 1.0e-9;

// =============================================================================
// Result status
// =============================================================================

/// Semantic status of a normalized result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResultStatus {
    /// Result was successfully produced and validated.
    Complete,

    /// Provider returned only part of the requested result.
    Partial,

    /// Result exists but failed semantic validation.
    Invalid,

    /// Execution failed and no valid quantum result exists.
    Failed,

    /// Execution was cancelled before a valid result was produced.
    Cancelled,

    /// Execution timed out before a valid result was obtained.
    TimedOut,
}

impl ResultStatus {
    /// Returns true if the result is terminal.
    pub const fn is_terminal(self) -> bool {
        true
    }

    /// Returns true if quantum payloads may be consumed as authoritative.
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Invalid => "invalid",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
        }
    }
}

impl fmt::Display for ResultStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Complex numbers
// =============================================================================

/// Provider-neutral complex number.
///
/// Quantum amplitudes are represented as:
///
/// `real + i * imaginary`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex64 {
    /// Real component.
    pub real: f64,

    /// Imaginary component.
    pub imaginary: f64,
}

impl Complex64 {
    /// Creates a complex value after validating both components.
    pub fn new(real: f64, imaginary: f64) -> Result<Self, ResultError> {
        if !real.is_finite() || !imaginary.is_finite() {
            return Err(ResultError::NonFiniteValue);
        }

        Ok(Self { real, imaginary })
    }

    /// Complex zero.
    pub const fn zero() -> Self {
        Self {
            real: 0.0,
            imaginary: 0.0,
        }
    }

    /// Complex one.
    pub const fn one() -> Self {
        Self {
            real: 1.0,
            imaginary: 0.0,
        }
    }

    /// Returns squared magnitude.
    pub fn norm_squared(self) -> f64 {
        self.real.mul_add(self.real, self.imaginary * self.imaginary)
    }

    /// Returns magnitude.
    pub fn magnitude(self) -> f64 {
        self.norm_squared().sqrt()
    }

    /// Returns the complex conjugate.
    pub const fn conjugate(self) -> Self {
        Self {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }
}

// =============================================================================
// Measurement counts
// =============================================================================

/// Deterministic measurement-count distribution.
///
/// Keys are canonical measurement outcomes, normally bitstrings such as:
///
/// `00`
/// `01`
/// `10`
/// `11`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementCounts {
    counts: BTreeMap<String, u64>,
    total_shots: u64,
}

impl MeasurementCounts {
    /// Creates a validated count distribution.
    pub fn new(
        counts: BTreeMap<String, u64>,
    ) -> Result<Self, ResultError> {
        if counts.len() > MAX_MEASUREMENT_OUTCOMES {
            return Err(ResultError::CollectionLimitExceeded {
                collection: "measurement_counts",
                maximum: MAX_MEASUREMENT_OUTCOMES,
            });
        }

        let mut total = 0_u64;

        for (outcome, count) in &counts {
            validate_measurement_outcome(outcome)?;

            total = total.checked_add(*count).ok_or(
                ResultError::ShotCountOverflow,
            )?;
        }

        Ok(Self {
            counts,
            total_shots: total,
        })
    }

    /// Creates an empty distribution.
    pub fn empty() -> Self {
        Self {
            counts: BTreeMap::new(),
            total_shots: 0,
        }
    }

    /// Adds one outcome count.
    pub fn insert(
        &mut self,
        outcome: impl Into<String>,
        count: u64,
    ) -> Result<(), ResultError> {
        let outcome = outcome.into();

        validate_measurement_outcome(&outcome)?;

        if !self.counts.contains_key(&outcome)
            && self.counts.len() >= MAX_MEASUREMENT_OUTCOMES
        {
            return Err(ResultError::CollectionLimitExceeded {
                collection: "measurement_counts",
                maximum: MAX_MEASUREMENT_OUTCOMES,
            });
        }

        let previous = self.counts.get(&outcome).copied().unwrap_or(0);

        let new_value = previous
            .checked_add(count)
            .ok_or(ResultError::ShotCountOverflow)?;

        self.total_shots = self
            .total_shots
            .checked_sub(previous)
            .and_then(|value| value.checked_add(new_value))
            .ok_or(ResultError::ShotCountOverflow)?;

        self.counts.insert(outcome, new_value);

        Ok(())
    }

    /// Returns counts in deterministic order.
    pub fn as_map(&self) -> &BTreeMap<String, u64> {
        &self.counts
    }

    /// Returns the number of distinct outcomes.
    pub fn len(&self) -> usize {
        self.counts.len()
    }

    /// Returns whether no outcomes were observed.
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }

    /// Returns total shots represented by the distribution.
    pub const fn total_shots(&self) -> u64 {
        self.total_shots
    }

    /// Converts counts to probabilities.
    pub fn probabilities(
        &self,
    ) -> Result<ProbabilityDistribution, ResultError> {
        if self.total_shots == 0 {
            return Ok(ProbabilityDistribution::empty());
        }

        let denominator = self.total_shots as f64;
        let mut probabilities = BTreeMap::new();

        for (outcome, count) in &self.counts {
            probabilities.insert(
                outcome.clone(),
                *count as f64 / denominator,
            );
        }

        ProbabilityDistribution::new(probabilities)
    }
}

// =============================================================================
// Probability distributions
// =============================================================================

/// Normalized probability distribution over quantum outcomes.
#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilityDistribution {
    probabilities: BTreeMap<String, f64>,
}

impl ProbabilityDistribution {
    /// Creates a validated probability distribution.
    pub fn new(
        probabilities: BTreeMap<String, f64>,
    ) -> Result<Self, ResultError> {
        if probabilities.len() > MAX_MEASUREMENT_OUTCOMES {
            return Err(ResultError::CollectionLimitExceeded {
                collection: "probabilities",
                maximum: MAX_MEASUREMENT_OUTCOMES,
            });
        }

        let mut total = 0.0;

        for (outcome, probability) in &probabilities {
            validate_measurement_outcome(outcome)?;

            validate_probability(*probability)?;

            total += *probability;
        }

        if !probabilities.is_empty()
            && (total - 1.0).abs() > PROBABILITY_NORMALIZATION_TOLERANCE
        {
            return Err(ResultError::ProbabilityNotNormalized {
                total,
            });
        }

        Ok(Self { probabilities })
    }

    /// Creates an empty distribution.
    pub fn empty() -> Self {
        Self {
            probabilities: BTreeMap::new(),
        }
    }

    /// Returns probabilities in deterministic order.
    pub fn as_map(&self) -> &BTreeMap<String, f64> {
        &self.probabilities
    }

    /// Returns the probability of one outcome.
    pub fn get(&self, outcome: &str) -> Option<f64> {
        self.probabilities.get(outcome).copied()
    }

    /// Returns the number of outcomes.
    pub fn len(&self) -> usize {
        self.probabilities.len()
    }

    /// Returns whether the distribution is empty.
    pub fn is_empty(&self) -> bool {
        self.probabilities.is_empty()
    }

    /// Returns the sum of all probabilities.
    pub fn total_probability(&self) -> f64 {
        self.probabilities.values().copied().sum()
    }
}

// =============================================================================
// Raw samples
// =============================================================================

/// Raw measurement samples in execution order.
///
/// Each sample is represented as a canonical bitstring.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasurementSamples {
    samples: Vec<String>,
}

impl MeasurementSamples {
    /// Creates validated samples.
    pub fn new(samples: Vec<String>) -> Result<Self, ResultError> {
        if samples.len() > MAX_SAMPLES {
            return Err(ResultError::CollectionLimitExceeded {
                collection: "measurement_samples",
                maximum: MAX_SAMPLES,
            });
        }

        for sample in &samples {
            validate_measurement_outcome(sample)?;
        }

        Ok(Self { samples })
    }

    /// Returns samples in original execution order.
    pub fn as_slice(&self) -> &[String] {
        &self.samples
    }

    /// Returns number of samples.
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Returns whether no samples exist.
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Converts samples into deterministic counts.
    pub fn counts(&self) -> Result<MeasurementCounts, ResultError> {
        let mut counts = MeasurementCounts::empty();

        for sample in &self.samples {
            counts.insert(sample.clone(), 1)?;
        }

        Ok(counts)
    }
}

// =============================================================================
// Expectation values
// =============================================================================

/// Named expectation value.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpectationValue {
    /// Observable identifier.
    pub observable: String,

    /// Expectation value.
    pub value: f64,

    /// Optional statistical standard error.
    pub standard_error: Option<f64>,
}

impl ExpectationValue {
    /// Creates a validated expectation value.
    pub fn new(
        observable: impl Into<String>,
        value: f64,
    ) -> Result<Self, ResultError> {
        let observable = validate_named_value(
            "observable",
            observable.into(),
        )?;

        validate_finite(value)?;

        Ok(Self {
            observable,
            value,
            standard_error: None,
        })
    }

    /// Adds a validated standard error.
    pub fn with_standard_error(
        mut self,
        standard_error: f64,
    ) -> Result<Self, ResultError> {
        if !standard_error.is_finite() || standard_error < 0.0 {
            return Err(ResultError::InvalidStandardError);
        }

        self.standard_error = Some(standard_error);

        Ok(self)
    }
}

/// Deterministically ordered expectation values.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpectationValues {
    values: BTreeMap<String, ExpectationValue>,
}

impl ExpectationValues {
    /// Creates an empty expectation-value collection.
    pub fn empty() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Creates a validated collection.
    pub fn new(
        values: Vec<ExpectationValue>,
    ) -> Result<Self, ResultError> {
        if values.len() > MAX_OBSERVABLES {
            return Err(ResultError::CollectionLimitExceeded {
                collection: "expectation_values",
                maximum: MAX_OBSERVABLES,
            });
        }

        let mut map = BTreeMap::new();

        for value in values {
            if map.insert(value.observable.clone(), value).is_some() {
                return Err(ResultError::DuplicateObservable);
            }
        }

        Ok(Self { values: map })
    }

    /// Inserts an expectation value.
    pub fn insert(
        &mut self,
        value: ExpectationValue,
    ) -> Result<(), ResultError> {
        if !self.values.contains_key(&value.observable)
            && self.values.len() >= MAX_OBSERVABLES
        {
            return Err(ResultError::CollectionLimitExceeded {
                collection: "expectation_values",
                maximum: MAX_OBSERVABLES,
            });
        }

        if self
            .values
            .insert(value.observable.clone(), value)
            .is_some()
        {
            return Err(ResultError::DuplicateObservable);
        }

        Ok(())
    }

    /// Returns all expectation values.
    pub fn as_map(&self) -> &BTreeMap<String, ExpectationValue> {
        &self.values
    }

    /// Returns number of expectation values.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

// =============================================================================
// Amplitudes
// =============================================================================

/// Named complex amplitude.
#[derive(Debug, Clone, PartialEq)]
pub struct Amplitude {
    /// Basis-state identifier.
    pub basis_state: String,

    /// Complex amplitude.
    pub value: Complex64,
}

impl Amplitude {
    /// Creates a validated amplitude.
    pub fn new(
        basis_state: impl Into<String>,
        value: Complex64,
    ) -> Result<Self, ResultError> {
        let basis_state = validate_named_value(
            "basis_state",
            basis_state.into(),
        )?;

        Ok(Self {
            basis_state,
            value,
        })
    }
}

/// Deterministically ordered amplitude collection.
#[derive(Debug, Clone, PartialEq)]
pub struct Amplitudes {
    values: BTreeMap<String, Complex64>,
}

impl Amplitudes {
    /// Creates an amplitude collection.
    pub fn new(
        values: Vec<Amplitude>,
    ) -> Result<Self, ResultError> {
        if values.len() > MAX_AMPLITUDES {
            return Err(ResultError::CollectionLimitExceeded {
                collection: "amplitudes",
                maximum: MAX_AMPLITUDES,
            });
        }

        let mut map = BTreeMap::new();

        for amplitude in values {
            if map
                .insert(amplitude.basis_state.clone(), amplitude)
                .is_some()
            {
                return Err(ResultError::DuplicateAmplitude);
            }
        }

        Ok(Self { values: map })
    }

    /// Returns amplitudes.
    pub fn as_map(&self) -> &BTreeMap<String, Amplitude> {
        &self.values
    }

    /// Returns squared norm.
    pub fn norm_squared(&self) -> f64 {
        self.values
            .values()
            .map(|amplitude| amplitude.value.norm_squared())
            .sum()
    }

    /// Validates state normalization.
    pub fn validate_normalized(&self) -> Result<(), ResultError> {
        let norm = self.norm_squared();

        if (norm - 1.0).abs() > STATE_NORMALIZATION_TOLERANCE {
            return Err(ResultError::StateNotNormalized { norm });
        }

        Ok(())
    }

    /// Number of amplitudes.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

// =============================================================================
// State vector
// =============================================================================

/// Dense quantum state vector.
///
/// The vector is stored in computational-basis order.
///
/// For `n` qubits the vector normally contains `2^n` amplitudes.
#[derive(Debug, Clone, PartialEq)]
pub struct StateVector {
    amplitudes: Vec<Complex64>,
    qubit_count: usize,
}

impl StateVector {
    /// Creates a validated state vector.
    pub fn new(
        qubit_count: usize,
        amplitudes: Vec<Complex64>,
    ) -> Result<Self, ResultError> {
        let expected = checked_power_of_two(qubit_count)?;

        if expected != amplitudes.len() {
            return Err(ResultError::InvalidStateVectorDimension {
                expected,
                actual: amplitudes.len(),
            });
        }

        if amplitudes.len() > MAX_STATE_VECTOR_AMPLITUDES {
            return Err(ResultError::CollectionLimitExceeded {
                collection: "state_vector",
                maximum: MAX_STATE_VECTOR_AMPLITUDES,
            });
        }

        let vector = Self {
            amplitudes,
            qubit_count,
        };

        vector.validate()?;

        Ok(vector)
    }

    /// Returns qubit count.
    pub const fn qubit_count(&self) -> usize {
        self.qubit_count
    }

    /// Returns amplitudes.
    pub fn amplitudes(&self) -> &[Complex64] {
        &self.amplitudes
    }

    /// Returns squared norm.
    pub fn norm_squared(&self) -> f64 {
        self.amplitudes
            .iter()
            .map(|amplitude| amplitude.norm_squared())
            .sum()
    }

    /// Validates the state vector.
    pub fn validate(&self) -> Result<(), ResultError> {
        let norm = self.norm_squared();

        if !norm.is_finite() {
            return Err(ResultError::NonFiniteValue);
        }

        if (norm - 1.0).abs() > STATE_NORMALIZATION_TOLERANCE {
            return Err(ResultError::StateNotNormalized { norm });
        }

        Ok(())
    }
}

// =============================================================================
// Density matrix
// =============================================================================

/// Dense quantum density matrix.
///
/// The matrix must be square and have dimension `2^n`.
#[derive(Debug, Clone, PartialEq)]
pub struct DensityMatrix {
    elements: Vec<Complex64>,
    dimension: usize,
    qubit_count: usize,
}

impl DensityMatrix {
    /// Creates a validated density matrix.
    pub fn new(
        qubit_count: usize,
        elements: Vec<Complex64>,
    ) -> Result<Self, ResultError> {
        let dimension = checked_power_of_two(qubit_count)?;

        let expected_elements = dimension
            .checked_mul(dimension)
            .ok_or(ResultError::DimensionOverflow)?;

        if expected_elements != elements.len() {
            return Err(ResultError::InvalidDensityMatrixDimension {
                expected: expected_elements,
                actual: elements.len(),
            });
        }

        if elements.len() > MAX_DENSITY_MATRIX_ELEMENTS {
            return Err(ResultError::CollectionLimitExceeded {
                collection: "density_matrix",
                maximum: MAX_DENSITY_MATRIX_ELEMENTS,
            });
        }

        let matrix = Self {
            elements,
            dimension,
            qubit_count,
        };

        matrix.validate()?;

        Ok(matrix)
    }

    /// Returns qubit count.
    pub const fn qubit_count(&self) -> usize {
        self.qubit_count
    }

    /// Returns matrix dimension.
    pub const fn dimension(&self) -> usize {
        self.dimension
    }

    /// Returns row-major matrix elements.
    pub fn elements(&self) -> &[Complex64] {
        &self.elements
    }

    /// Returns an element by row and column.
    pub fn get(
        &self,
        row: usize,
        column: usize,
    ) -> Option<Complex64> {
        if row >= self.dimension || column >= self.dimension {
            return None;
        }

        self.elements
            .get(row * self.dimension + column)
            .copied()
    }

    /// Validates Hermiticity and unit trace.
    ///
    /// This deliberately checks the mathematical invariants that can be
    /// verified without introducing an external linear-algebra dependency.
    pub fn validate(&self) -> Result<(), ResultError> {
        for row in 0..self.dimension {
            for column in 0..self.dimension {
                let a = self
                    .get(row, column)
                    .ok_or(ResultError::MatrixIndexError)?;

                let b = self
                    .get(column, row)
                    .ok_or(ResultError::MatrixIndexError)?;

                if (a.real - b.real).abs()
                    > STATE_NORMALIZATION_TOLERANCE
                    || (a.imaginary + b.imaginary).abs()
                        > STATE_NORMALIZATION_TOLERANCE
                {
                    return Err(ResultError::DensityMatrixNotHermitian);
                }
            }
        }

        let mut trace = 0.0;

        for index in 0..self.dimension {
            let diagonal = self
                .get(index, index)
                .ok_or(ResultError::MatrixIndexError)?;

            if diagonal.imaginary.abs()
                > STATE_NORMALIZATION_TOLERANCE
            {
                return Err(ResultError::DensityMatrixInvalidTrace);
            }

            trace += diagonal.real;
        }

        if (trace - 1.0).abs() > STATE_NORMALIZATION_TOLERANCE {
            return Err(ResultError::DensityMatrixTraceInvalid {
                trace,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Classical registers
// =============================================================================

/// Classical register returned by an execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalRegister {
    /// Register name.
    pub name: String,

    /// Register values in provider-neutral integer representation.
    pub values: Vec<u64>,
}

impl ClassicalRegister {
    /// Creates a validated register.
    pub fn new(
        name: impl Into<String>,
        values: Vec<u64>,
    ) -> Result<Self, ResultError> {
        if values.len() > MAX_CLASSICAL_REGISTER_VALUES {
            return Err(ResultError::CollectionLimitExceeded {
                collection: "classical_register",
                maximum: MAX_CLASSICAL_REGISTER_VALUES,
            });
        }

        let name = validate_named_value(
            "classical_register",
            name.into(),
        )?;

        Ok(Self { name, values })
    }
}

/// Deterministically ordered classical registers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalRegisters {
    registers: BTreeMap<String, ClassicalRegister>,
}

impl ClassicalRegisters {
    /// Creates an empty collection.
    pub fn empty() -> Self {
        Self {
            registers: BTreeMap::new(),
        }
    }

    /// Inserts a register.
    pub fn insert(
        &mut self,
        register: ClassicalRegister,
    ) -> Result<(), ResultError> {
        if self
            .registers
            .insert(register.name.clone(), register)
            .is_some()
        {
            return Err(ResultError::DuplicateClassicalRegister);
        }

        Ok(())
    }

    /// Returns registers.
    pub fn as_map(
        &self,
    ) -> &BTreeMap<String, ClassicalRegister> {
        &self.registers
    }

    /// Returns number of registers.
    pub fn len(&self) -> usize {
        self.registers.len()
    }

    /// Returns whether empty.
    pub fn is_empty(&self) -> bool {
        self.registers.is_empty()
    }
}

// =============================================================================
// Logical measurements
// =============================================================================

/// Logical-qubit measurement result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalMeasurement {
    /// Logical qubit identifier.
    pub logical_qubit: u64,

    /// Measurement value.
    pub value: bool,

    /// Optional physical-qubit mapping used for this measurement.
    pub physical_qubits: Vec<u64>,
}

impl LogicalMeasurement {
    /// Creates a logical measurement.
    pub fn new(
        logical_qubit: u64,
        value: bool,
        physical_qubits: Vec<u64>,
    ) -> Result<Self, ResultError> {
        Ok(Self {
            logical_qubit,
            value,
            physical_qubits,
        })
    }
}

// =============================================================================
// Analog results
// =============================================================================

/// Named analog observable result.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalogObservable {
    /// Observable identifier.
    pub name: String,

    /// Measured value.
    pub value: f64,

    /// Optional uncertainty.
    pub uncertainty: Option<f64>,

    /// Optional physical unit.
    pub unit: Option<String>,
}

impl AnalogObservable {
    /// Creates an analog observable.
    pub fn new(
        name: impl Into<String>,
        value: f64,
    ) -> Result<Self, ResultError> {
        let name =
            validate_named_value("analog_observable", name.into())?;

        validate_finite(value)?;

        Ok(Self {
            name,
            value,
            uncertainty: None,
            unit: None,
        })
    }

    /// Sets uncertainty.
    pub fn with_uncertainty(
        mut self,
        uncertainty: f64,
    ) -> Result<Self, ResultError> {
        if !uncertainty.is_finite() || uncertainty < 0.0 {
            return Err(ResultError::InvalidStandardError);
        }

        self.uncertainty = Some(uncertainty);

        Ok(self)
    }

    /// Sets the unit.
    pub fn with_unit(
        mut self,
        unit: impl Into<String>,
    ) -> Result<Self, ResultError> {
        self.unit = Some(validate_named_value(
            "unit",
            unit.into(),
        )?);

        Ok(self)
    }
}

// =============================================================================
// Annealing results
// =============================================================================

/// One annealing solution/sample.
#[derive(Debug, Clone, PartialEq)]
pub struct AnnealingSample {
    /// Assignment of variable names to binary/spin values.
    pub assignment: BTreeMap<String, i8>,

    /// Objective energy/cost.
    pub energy: f64,

    /// Number of occurrences represented by this sample.
    pub occurrences: u64,
}

impl AnnealingSample {
    /// Creates a validated annealing sample.
    pub fn new(
        assignment: BTreeMap<String, i8>,
        energy: f64,
        occurrences: u64,
    ) -> Result<Self, ResultError> {
        if assignment.is_empty() {
            return Err(ResultError::EmptyAssignment);
        }

        for value in assignment.values() {
            if *value != -1 && *value != 0 && *value != 1 {
                return Err(ResultError::InvalidAnnealingValue);
            }
        }

        validate_finite(energy)?;

        Ok(Self {
            assignment,
            energy,
            occurrences,
        })
    }
}

/// Annealing result collection.
#[derive(Debug, Clone, PartialEq)]
pub struct AnnealingResults {
    samples: Vec<AnnealingSample>,
}

impl AnnealingResults {
    /// Creates validated annealing results.
    pub fn new(
        samples: Vec<AnnealingSample>,
    ) -> Result<Self, ResultError> {
        if samples.len() > MAX_ANNEALING_SAMPLES {
            return Err(ResultError::CollectionLimitExceeded {
                collection: "annealing_samples",
                maximum: MAX_ANNEALING_SAMPLES,
            });
        }

        Ok(Self { samples })
    }

    /// Returns samples.
    pub fn samples(&self) -> &[AnnealingSample] {
        &self.samples
    }

    /// Returns number of samples.
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Returns whether empty.
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

// =============================================================================
// Pulse acquisition
// =============================================================================

/// A normalized pulse/acquisition sample.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AcquisitionSample {
    /// Sample time.
    pub time: f64,

    /// Real component.
    pub real: f64,

    /// Imaginary component.
    pub imaginary: f64,
}

impl AcquisitionSample {
    /// Creates a validated acquisition sample.
    pub fn new(
        time: f64,
        real: f64,
        imaginary: f64,
    ) -> Result<Self, ResultError> {
        validate_finite(time)?;
        validate_finite(real)?;
        validate_finite(imaginary)?;

        Ok(Self {
            time,
            real,
            imaginary,
        })
    }
}

/// Normalized pulse acquisition result.
#[derive(Debug, Clone, PartialEq)]
pub struct PulseAcquisition {
    /// Acquisition channel identifier.
    pub channel: String,

    /// Samples in acquisition order.
    pub samples: Vec<AcquisitionSample>,

    /// Optional sample rate in samples/second.
    pub sample_rate_hz: Option<f64>,
}

impl PulseAcquisition {
    /// Creates a pulse acquisition result.
    pub fn new(
        channel: impl Into<String>,
        samples: Vec<AcquisitionSample>,
    ) -> Result<Self, ResultError> {
        if samples.len() > MAX_ACQUISITION_SAMPLES {
            return Err(ResultError::CollectionLimitExceeded {
                collection: "pulse_acquisition",
                maximum: MAX_ACQUISITION_SAMPLES,
            });
        }

        let channel =
            validate_named_value("pulse_channel", channel.into())?;

        Ok(Self {
            channel,
            samples,
            sample_rate_hz: None,
        })
    }

    /// Sets the sample rate.
    pub fn with_sample_rate(
        mut self,
        sample_rate_hz: f64,
    ) -> Result<Self, ResultError> {
        if !sample_rate_hz.is_finite() || sample_rate_hz <= 0.0 {
            return Err(ResultError::InvalidSampleRate);
        }

        self.sample_rate_hz = Some(sample_rate_hz);

        Ok(self)
    }
}

// =============================================================================
// Result payload
// =============================================================================

/// One typed result payload.
///
/// A single execution may contain multiple payloads.
///
/// For example:
///
/// ```text
/// Counts
/// + ExpectationValues
/// + ClassicalRegisters
/// ```
///
/// may all belong to the same execution.
#[derive(Debug, Clone, PartialEq)]
pub enum ResultPayload {
    /// Measurement counts.
    Counts(MeasurementCounts),

    /// Normalized outcome probabilities.
    Probabilities(ProbabilityDistribution),

    /// Raw measurement samples.
    Samples(MeasurementSamples),

    /// Expectation values.
    ExpectationValues(ExpectationValues),

    /// Named complex amplitudes.
    Amplitudes(Amplitudes),

    /// Full state vector.
    StateVector(StateVector),

    /// Full density matrix.
    DensityMatrix(DensityMatrix),

    /// Classical register values.
    ClassicalRegisters(ClassicalRegisters),

    /// Logical-qubit measurements.
    LogicalMeasurements(Vec<LogicalMeasurement>),

    /// Analog observables.
    AnalogObservables(Vec<AnalogObservable>),

    /// Annealing samples.
    Annealing(AnnealingResults),

    /// Pulse/acquisition data.
    PulseAcquisition(Vec<PulseAcquisition>),
}

impl ResultPayload {
    /// Stable payload kind identifier.
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Counts(_) => "counts",
            Self::Probabilities(_) => "probabilities",
            Self::Samples(_) => "samples",
            Self::ExpectationValues(_) => "expectation_values",
            Self::Amplitudes(_) => "amplitudes",
            Self::StateVector(_) => "state_vector",
            Self::DensityMatrix(_) => "density_matrix",
            Self::ClassicalRegisters(_) => "classical_registers",
            Self::LogicalMeasurements(_) => "logical_measurements",
            Self::AnalogObservables(_) => "analog_observables",
            Self::Annealing(_) => "annealing",
            Self::PulseAcquisition(_) => "pulse_acquisition",
        }
    }

    /// Returns the represented shot count where meaningful.
    pub fn shot_count(&self) -> Option<u64> {
        match self {
            Self::Counts(counts) => Some(counts.total_shots()),
            Self::Samples(samples) => {
                Some(samples.len() as u64)
            }
            _ => None,
        }
    }
}

// =============================================================================
// Provenance
// =============================================================================

/// Reproducibility/provenance information attached to a result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultProvenance {
    /// Canonical backend identifier.
    pub backend_id: String,

    /// Provider identifier.
    pub provider_id: Option<String>,

    /// Provider job identifier.
    pub job_id: Option<String>,

    /// Caller request identifier.
    pub request_id: Option<String>,

    /// Calibration snapshot identifier.
    pub calibration_snapshot_id: Option<String>,

    /// Topology version.
    pub topology_version: Option<String>,

    /// Instruction-set version.
    pub instruction_set_version: Option<String>,

    /// Backend/hardware revision.
    pub hardware_revision: Option<String>,

    /// Firmware version.
    pub firmware_version: Option<String>,

    /// Backend API version.
    pub api_version: Option<String>,

    /// Adapter version.
    pub adapter_version: Option<String>,

    /// Compiler version.
    pub compiler_version: Option<String>,

    /// Quantum IR version.
    pub quantum_ir_version: Option<String>,

    /// Deterministic execution seed.
    pub seed: Option<u64>,
}

impl ResultProvenance {
    /// Creates provenance with mandatory backend identity.
    pub fn new(
        backend_id: impl Into<String>,
    ) -> Result<Self, ResultError> {
        let backend_id =
            validate_identifier("backend_id", backend_id.into())?;

        Ok(Self {
            backend_id,
            provider_id: None,
            job_id: None,
            request_id: None,
            calibration_snapshot_id: None,
            topology_version: None,
            instruction_set_version: None,
            hardware_revision: None,
            firmware_version: None,
            api_version: None,
            adapter_version: None,
            compiler_version: None,
            quantum_ir_version: None,
            seed: None,
        })
    }

    /// Sets provider identity.
    pub fn with_provider(
        mut self,
        provider_id: impl Into<String>,
    ) -> Result<Self, ResultError> {
        self.provider_id = Some(validate_identifier(
            "provider_id",
            provider_id.into(),
        )?);

        Ok(self)
    }

    /// Sets job identity.
    pub fn with_job_id(
        mut self,
        job_id: impl Into<String>,
    ) -> Result<Self, ResultError> {
        self.job_id = Some(validate_identifier(
            "job_id",
            job_id.into(),
        )?);

        Ok(self)
    }

    /// Sets request identity.
    pub fn with_request_id(
        mut self,
        request_id: impl Into<String>,
    ) -> Result<Self, ResultError> {
        self.request_id = Some(validate_identifier(
            "request_id",
            request_id.into(),
        )?);

        Ok(self)
    }

    /// Sets calibration snapshot.
    pub fn with_calibration_snapshot(
        mut self,
        id: impl Into<String>,
    ) -> Result<Self, ResultError> {
        self.calibration_snapshot_id = Some(
            validate_identifier(
                "calibration_snapshot_id",
                id.into(),
            )?,
        );

        Ok(self)
    }

    /// Sets deterministic seed.
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Returns whether the provenance contains the mandatory backend.
    pub fn is_valid(&self) -> bool {
        !self.backend_id.is_empty()
    }
}

// =============================================================================
// Result metadata
// =============================================================================

/// Safe provider-neutral metadata.
///
/// Values are deliberately strings because this module is the semantic result
/// layer. Structured serialization belongs to `serialization.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultMetadata {
    values: BTreeMap<String, String>,
}

impl ResultMetadata {
    /// Creates empty metadata.
    pub fn empty() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Inserts safe metadata.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), ResultError> {
        let key = key.into();
        let value = value.into();

        validate_metadata_key(&key)?;
        validate_metadata_value(&value)?;

        if !self.values.contains_key(&key)
            && self.values.len() >= MAX_RESULT_METADATA_ENTRIES
        {
            return Err(ResultError::CollectionLimitExceeded {
                collection: "result_metadata",
                maximum: MAX_RESULT_METADATA_ENTRIES,
            });
        }

        if contains_secret_indicator(&key)
            || contains_secret_indicator(&value)
        {
            return Err(ResultError::SecretLikeMetadata);
        }

        self.values.insert(key, value);

        Ok(())
    }

    /// Returns metadata.
    pub fn as_map(&self) -> &BTreeMap<String, String> {
        &self.values
    }

    /// Returns number of entries.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

// =============================================================================
// Canonical execution result
// =============================================================================

/// Canonical normalized quantum execution result.
///
/// This is the authoritative result envelope for Zamani Quantum hardware.
///
/// A result can contain multiple payloads because real providers may return
/// counts together with expectation values, classical registers, logical
/// measurements, or other observables.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumExecutionResult {
    /// Schema identifier.
    pub schema_id: &'static str,

    /// Schema version.
    pub schema_version: u16,

    /// Result status.
    pub status: ResultStatus,

    /// Execution provenance.
    pub provenance: ResultProvenance,

    /// Number of requested shots, when applicable.
    pub requested_shots: Option<u64>,

    /// Number of successfully represented shots, when applicable.
    pub completed_shots: Option<u64>,

    /// Typed quantum result payloads.
    payloads: Vec<ResultPayload>,

    /// Safe provider-neutral metadata.
    pub metadata: ResultMetadata,
}

impl QuantumExecutionResult {
    /// Creates a complete result with mandatory provenance.
    pub fn new(
        provenance: ResultProvenance,
    ) -> Self {
        Self {
            schema_id: RESULT_SCHEMA_ID,
            schema_version: RESULT_SCHEMA_VERSION,
            status: ResultStatus::Complete,
            provenance,
            requested_shots: None,
            completed_shots: None,
            payloads: Vec::new(),
            metadata: ResultMetadata::empty(),
        }
    }

    /// Sets requested shot count.
    pub fn with_requested_shots(
        mut self,
        shots: u64,
    ) -> Self {
        self.requested_shots = Some(shots);
        self
    }

    /// Sets completed shot count.
    pub fn with_completed_shots(
        mut self,
        shots: u64,
    ) -> Self {
        self.completed_shots = Some(shots);
        self
    }

    /// Sets result status.
    pub fn with_status(
        mut self,
        status: ResultStatus,
    ) -> Self {
        self.status = status;
        self
    }

    /// Adds a payload.
    pub fn add_payload(
        &mut self,
        payload: ResultPayload,
    ) -> Result<(), ResultError> {
        if self.payloads.len() >= 128 {
            return Err(ResultError::TooManyPayloadKinds);
        }

        self.payloads.push(payload);

        Ok(())
    }

    /// Adds safe metadata.
    pub fn add_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), ResultError> {
        self.metadata.insert(key, value)
    }

    /// Returns all payloads.
    pub fn payloads(&self) -> &[ResultPayload] {
        &self.payloads
    }

    /// Returns first payload of the requested type.
    pub fn find_payload(
        &self,
        kind: &str,
    ) -> Option<&ResultPayload> {
        self.payloads.iter().find(|payload| {
            payload.kind() == kind
        })
    }

    /// Returns measurement counts if present.
    pub fn counts(&self) -> Option<&MeasurementCounts> {
        self.payloads.iter().find_map(|payload| match payload {
            ResultPayload::Counts(value) => Some(value),
            _ => None,
        })
    }

    /// Returns probabilities if present.
    pub fn probabilities(
        &self,
    ) -> Option<&ProbabilityDistribution> {
        self.payloads.iter().find_map(|payload| match payload {
            ResultPayload::Probabilities(value) => Some(value),
            _ => None,
        })
    }

    /// Returns expectation values if present.
    pub fn expectation_values(
        &self,
    ) -> Option<&ExpectationValues> {
        self.payloads.iter().find_map(|payload| match payload {
            ResultPayload::ExpectationValues(value) => Some(value),
            _ => None,
        })
    }

    /// Returns state vector if present.
    pub fn state_vector(&self) -> Option<&StateVector> {
        self.payloads.iter().find_map(|payload| match payload {
            ResultPayload::StateVector(value) => Some(value),
            _ => None,
        })
    }

    /// Returns density matrix if present.
    pub fn density_matrix(&self) -> Option<&DensityMatrix> {
        self.payloads.iter().find_map(|payload| match payload {
            ResultPayload::DensityMatrix(value) => Some(value),
            _ => None,
        })
    }

    /// Returns the effective represented shot count.
    ///
    /// Explicit `completed_shots` takes precedence. Otherwise this is inferred
    /// from a counts or samples payload when available.
    pub fn effective_shots(&self) -> Option<u64> {
        if let Some(shots) = self.completed_shots {
            return Some(shots);
        }

        self.payloads
            .iter()
            .find_map(ResultPayload::shot_count)
    }

    /// Validates the complete result.
    ///
    /// This should be called by provider adapters before returning the result
    /// to `execution.rs`.
    pub fn validate(&self) -> Result<(), ResultError> {
        if self.schema_id != RESULT_SCHEMA_ID {
            return Err(ResultError::SchemaMismatch);
        }

        if self.schema_version != RESULT_SCHEMA_VERSION {
            return Err(ResultError::UnsupportedSchemaVersion {
                expected: RESULT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        if !self.provenance.is_valid() {
            return Err(ResultError::MissingBackendIdentity);
        }

        if let Some(requested) = self.requested_shots {
            if let Some(completed) = self.completed_shots {
                if completed > requested {
                    return Err(
                        ResultError::CompletedShotsExceedRequested {
                            requested,
                            completed,
                        },
                    );
                }
            }

            for payload in &self.payloads {
                if let Some(shots) = payload.shot_count() {
                    if shots > requested {
                        return Err(
                            ResultError::PayloadShotsExceedRequested {
                                requested,
                                actual: shots,
                            },
                        );
                    }
                }
            }
        }

        for payload in &self.payloads {
            validate_payload(payload)?;
        }

        if matches!(self.status, ResultStatus::Complete)
            && self.payloads.is_empty()
        {
            return Err(ResultError::CompleteResultWithoutPayload);
        }

        if matches!(self.status, ResultStatus::Complete) {
            if let Some(requested) = self.requested_shots {
                if let Some(completed) = self.completed_shots {
                    if completed != requested {
                        return Err(
                            ResultError::CompleteResultShotMismatch {
                                requested,
                                completed,
                            },
                        );
                    }
                }
            }
        }

        Ok(())
    }

    /// Returns whether the result passes all integrity checks.
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Returns whether this is a complete usable result.
    pub fn is_complete(&self) -> bool {
        self.status == ResultStatus::Complete
            && self.is_valid()
    }
}

// =============================================================================
// Validation
// =============================================================================

fn validate_payload(
    payload: &ResultPayload,
) -> Result<(), ResultError> {
    match payload {
        ResultPayload::Counts(counts) => {
            if counts.total_shots() == 0 {
                return Err(ResultError::EmptyShotResult);
            }
        }

        ResultPayload::Probabilities(probabilities) => {
            if !probabilities.is_empty() {
                let total = probabilities.total_probability();

                if (total - 1.0).abs()
                    > PROBABILITY_NORMALIZATION_TOLERANCE
                {
                    return Err(
                        ResultError::ProbabilityNotNormalized {
                            total,
                        },
                    );
                }
            }
        }

        ResultPayload::Samples(samples) => {
            if samples.is_empty() {
                return Err(ResultError::EmptyShotResult);
            }
        }

        ResultPayload::ExpectationValues(values) => {
            if values.is_empty() {
                return Err(ResultError::EmptyExpectationValues);
            }

            for value in values.as_map().values() {
                validate_finite(value.value)?;

                if let Some(error) = value.standard_error {
                    if !error.is_finite() || error < 0.0 {
                        return Err(ResultError::InvalidStandardError);
                    }
                }
            }
        }

        ResultPayload::Amplitudes(amplitudes) => {
            amplitudes.validate_normalized()?;
        }

        ResultPayload::StateVector(state) => {
            state.validate()?;
        }

        ResultPayload::DensityMatrix(matrix) => {
            matrix.validate()?;
        }

        ResultPayload::ClassicalRegisters(registers) => {
            if registers.is_empty() {
                return Err(ResultError::EmptyClassicalRegisters);
            }
        }

        ResultPayload::LogicalMeasurements(measurements) => {
            if measurements.len() > MAX_LOGICAL_MEASUREMENTS {
                return Err(ResultError::CollectionLimitExceeded {
                    collection: "logical_measurements",
                    maximum: MAX_LOGICAL_MEASUREMENTS,
                });
            }
        }

        ResultPayload::AnalogObservables(observables) => {
            if observables.len() > MAX_OBSERVABLES {
                return Err(ResultError::CollectionLimitExceeded {
                    collection: "analog_observables",
                    maximum: MAX_OBSERVABLES,
                });
            }

            for observable in observables {
                validate_finite(observable.value)?;

                if let Some(uncertainty) = observable.uncertainty {
                    if !uncertainty.is_finite()
                        || uncertainty < 0.0
                    {
                        return Err(
                            ResultError::InvalidStandardError,
                        );
                    }
                }
            }
        }

        ResultPayload::Annealing(results) => {
            if results.is_empty() {
                return Err(ResultError::EmptyAnnealingResult);
            }
        }

        ResultPayload::PulseAcquisition(acquisitions) => {
            if acquisitions.len() > MAX_OBSERVABLES {
                return Err(ResultError::CollectionLimitExceeded {
                    collection: "pulse_acquisition",
                    maximum: MAX_OBSERVABLES,
                });
            }

            for acquisition in acquisitions {
                if acquisition.samples.is_empty() {
                    return Err(ResultError::EmptyPulseAcquisition);
                }

                if let Some(rate) = acquisition.sample_rate_hz {
                    if !rate.is_finite() || rate <= 0.0 {
                        return Err(ResultError::InvalidSampleRate);
                    }
                }
            }
        }
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Canonical result validation/integrity error.
#[derive(Debug, Clone, PartialEq)]
pub enum ResultError {
    /// Schema identifier did not match the canonical schema.
    SchemaMismatch,

    /// Result schema version is unsupported.
    UnsupportedSchemaVersion {
        /// Canonical expected version.
        expected: u16,

        /// Actual version.
        actual: u16,
    },

    /// Required backend provenance is absent.
    MissingBackendIdentity,

    /// String identifier is empty.
    EmptyIdentifier {
        /// Field name.
        field: &'static str,
    },

    /// String identifier contains invalid control characters.
    InvalidIdentifier {
        /// Field name.
        field: &'static str,
    },

    /// Identifier exceeds its maximum length.
    IdentifierTooLong {
        /// Field name.
        field: &'static str,

        /// Maximum allowed bytes.
        maximum: usize,
    },

    /// Named value is empty.
    EmptyNamedValue {
        /// Field name.
        field: &'static str,
    },

    /// Floating-point value is NaN or infinite.
    NonFiniteValue,

    /// Probability lies outside [0, 1].
    InvalidProbability {
        /// Invalid value.
        value: f64,
    },

    /// Probability distribution is not normalized.
    ProbabilityNotNormalized {
        /// Observed total.
        total: f64,
    },

    /// Shot count arithmetic overflowed.
    ShotCountOverflow,

    /// Payload contains more shots than requested.
    PayloadShotsExceedRequested {
        /// Requested shots.
        requested: u64,

        /// Payload shots.
        actual: u64,
    },

    /// Completed shots exceed requested shots.
    CompletedShotsExceedRequested {
        /// Requested shots.
        requested: u64,

        /// Completed shots.
        completed: u64,
    },

    /// Complete result does not represent all requested shots.
    CompleteResultShotMismatch {
        /// Requested shots.
        requested: u64,

        /// Completed shots.
        completed: u64,
    },

    /// Complete result has no payload.
    CompleteResultWithoutPayload,

    /// Result contains zero shots where a shot payload is required.
    EmptyShotResult,

    /// Too many values in a collection.
    CollectionLimitExceeded {
        /// Collection identifier.
        collection: &'static str,

        /// Maximum supported entries.
        maximum: usize,
    },

    /// State-vector dimension overflow.
    DimensionOverflow,

    /// State-vector dimension does not match qubit count.
    InvalidStateVectorDimension {
        /// Expected amplitude count.
        expected: usize,

        /// Actual amplitude count.
        actual: usize,
    },

    /// State vector is not normalized.
    StateNotNormalized {
        /// Observed squared norm.
        norm: f64,
    },

    /// Density-matrix dimension is invalid.
    InvalidDensityMatrixDimension {
        /// Expected element count.
        expected: usize,

        /// Actual element count.
        actual: usize,
    },

    /// Density matrix is not Hermitian.
    DensityMatrixNotHermitian,

    /// Density-matrix trace contains an invalid imaginary component.
    DensityMatrixInvalidTrace,

    /// Density-matrix trace is not one.
    DensityMatrixTraceInvalid {
        /// Observed trace.
        trace: f64,
    },

    /// Matrix indexing failed.
    MatrixIndexError,

    /// Duplicate observable.
    DuplicateObservable,

    /// Duplicate amplitude basis state.
    DuplicateAmplitude,

    /// Duplicate classical register.
    DuplicateClassicalRegister,

    /// Invalid expectation-value uncertainty.
    InvalidStandardError,

    /// Empty expectation-value collection.
    EmptyExpectationValues,

    /// Empty classical register collection.
    EmptyClassicalRegisters,

    /// Empty annealing assignment.
    EmptyAssignment,

    /// Annealing variable is not binary or spin-valued.
    InvalidAnnealingValue,

    /// Empty annealing result.
    EmptyAnnealingResult,

    /// Empty pulse acquisition.
    EmptyPulseAcquisition,

    /// Invalid pulse sample rate.
    InvalidSampleRate,

    /// Metadata key/value contains a secret-like field.
    SecretLikeMetadata,

    /// Metadata key is invalid.
    InvalidMetadataKey,

    /// Metadata value is too large.
    MetadataValueTooLong,

    /// Too many distinct payload kinds.
    TooManyPayloadKinds,
}

impl fmt::Display for ResultError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::SchemaMismatch => {
                formatter.write_str("result schema mismatch")
            }

            Self::UnsupportedSchemaVersion {
                expected,
                actual,
            } => write!(
                formatter,
                "unsupported result schema version: expected {expected}, got {actual}"
            ),

            Self::MissingBackendIdentity => {
                formatter.write_str("result is missing backend identity")
            }

            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} is empty")
            }

            Self::InvalidIdentifier { field } => {
                write!(formatter, "{field} contains invalid characters")
            }

            Self::IdentifierTooLong { field, maximum } => {
                write!(
                    formatter,
                    "{field} exceeds maximum length of {maximum} bytes"
                )
            }

            Self::EmptyNamedValue { field } => {
                write!(formatter, "{field} is empty")
            }

            Self::NonFiniteValue => {
                formatter.write_str("result contains a non-finite numeric value")
            }

            Self::InvalidProbability { value } => {
                write!(formatter, "invalid probability: {value}")
            }

            Self::ProbabilityNotNormalized { total } => {
                write!(
                    formatter,
                    "probability distribution is not normalized: total={total}"
                )
            }

            Self::ShotCountOverflow => {
                formatter.write_str("shot count overflow")
            }

            Self::PayloadShotsExceedRequested {
                requested,
                actual,
            } => write!(
                formatter,
                "payload contains {actual} shots but only {requested} were requested"
            ),

            Self::CompletedShotsExceedRequested {
                requested,
                completed,
            } => write!(
                formatter,
                "completed shots {completed} exceed requested shots {requested}"
            ),

            Self::CompleteResultShotMismatch {
                requested,
                completed,
            } => write!(
                formatter,
                "complete result contains {completed} shots but {requested} were requested"
            ),

            Self::CompleteResultWithoutPayload => {
                formatter.write_str(
                    "complete result contains no result payload",
                )
            }

            Self::EmptyShotResult => {
                formatter.write_str("shot result contains zero shots")
            }

            Self::CollectionLimitExceeded {
                collection,
                maximum,
            } => write!(
                formatter,
                "{collection} exceeds maximum size of {maximum}"
            ),

            Self::DimensionOverflow => {
                formatter.write_str("quantum dimension calculation overflowed")
            }

            Self::InvalidStateVectorDimension {
                expected,
                actual,
            } => write!(
                formatter,
                "invalid state-vector dimension: expected {expected}, got {actual}"
            ),

            Self::StateNotNormalized { norm } => {
                write!(
                    formatter,
                    "quantum state is not normalized: squared norm={norm}"
                )
            }

            Self::InvalidDensityMatrixDimension {
                expected,
                actual,
            } => write!(
                formatter,
                "invalid density-matrix dimension: expected {expected}, got {actual}"
            ),

            Self::DensityMatrixNotHermitian => {
                formatter.write_str("density matrix is not Hermitian")
            }

            Self::DensityMatrixInvalidTrace => {
                formatter.write_str(
                    "density matrix has an invalid complex trace",
                )
            }

            Self::DensityMatrixTraceInvalid { trace } => {
                write!(
                    formatter,
                    "density matrix trace must equal one: trace={trace}"
                )
            }

            Self::MatrixIndexError => {
                formatter.write_str("density-matrix index out of bounds")
            }

            Self::DuplicateObservable => {
                formatter.write_str("duplicate observable")
            }

            Self::DuplicateAmplitude => {
                formatter.write_str("duplicate amplitude basis state")
            }

            Self::DuplicateClassicalRegister => {
                formatter.write_str("duplicate classical register")
            }

            Self::InvalidStandardError => {
                formatter.write_str("invalid standard error")
            }

            Self::EmptyExpectationValues => {
                formatter.write_str("empty expectation-value collection")
            }

            Self::EmptyClassicalRegisters => {
                formatter.write_str("empty classical-register collection")
            }

            Self::EmptyAssignment => {
                formatter.write_str("annealing assignment is empty")
            }

            Self::InvalidAnnealingValue => {
                formatter.write_str(
                    "annealing assignment must contain -1, 0 or 1",
                )
            }

            Self::EmptyAnnealingResult => {
                formatter.write_str("annealing result is empty")
            }

            Self::EmptyPulseAcquisition => {
                formatter.write_str("pulse acquisition contains no samples")
            }

            Self::InvalidSampleRate => {
                formatter.write_str("invalid acquisition sample rate")
            }

            Self::SecretLikeMetadata => {
                formatter.write_str(
                    "result metadata contains secret-like information",
                )
            }

            Self::InvalidMetadataKey => {
                formatter.write_str("invalid result metadata key")
            }

            Self::MetadataValueTooLong => {
                formatter.write_str(
                    "result metadata value exceeds maximum length",
                )
            }

            Self::TooManyPayloadKinds => {
                formatter.write_str("too many result payload kinds")
            }
        }
    }
}

impl std::error::Error for ResultError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_finite(value: f64) -> Result<(), ResultError> {
    if !value.is_finite() {
        return Err(ResultError::NonFiniteValue);
    }

    Ok(())
}

fn validate_probability(
    value: f64,
) -> Result<(), ResultError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(ResultError::InvalidProbability { value });
    }

    Ok(())
}

fn validate_measurement_outcome(
    outcome: &str,
) -> Result<(), ResultError> {
    if outcome.is_empty() {
        return Err(ResultError::EmptyNamedValue {
            field: "measurement_outcome",
        });
    }

    if outcome.len() > MAX_METADATA_VALUE_LENGTH {
        return Err(ResultError::IdentifierTooLong {
            field: "measurement_outcome",
            maximum: MAX_METADATA_VALUE_LENGTH,
        });
    }

    if outcome.chars().any(|character| {
        character.is_control()
            || !(character.is_ascii_digit()
                || character == ' '
                || character == '_'
                || character == '-'
                || character == ','
                || character == '|')
    }) {
        return Err(ResultError::InvalidIdentifier {
            field: "measurement_outcome",
        });
    }

    Ok(())
}

fn validate_identifier(
    field: &'static str,
    value: String,
) -> Result<String, ResultError> {
    if value.is_empty() {
        return Err(ResultError::EmptyIdentifier { field });
    }

    if value.len() > 1024 {
        return Err(ResultError::IdentifierTooLong {
            field,
            maximum: 1024,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(ResultError::InvalidIdentifier { field });
    }

    Ok(value)
}

fn validate_named_value(
    field: &'static str,
    value: String,
) -> Result<String, ResultError> {
    if value.trim().is_empty() {
        return Err(ResultError::EmptyNamedValue { field });
    }

    if value.chars().any(char::is_control) {
        return Err(ResultError::InvalidIdentifier { field });
    }

    Ok(value)
}

fn validate_metadata_key(
    key: &str,
) -> Result<(), ResultError> {
    if key.trim().is_empty() || key.len() > MAX_METADATA_KEY_LENGTH {
        return Err(ResultError::InvalidMetadataKey);
    }

    if key.chars().any(|character| {
        character.is_control()
            || !(character.is_ascii_alphanumeric()
                || matches!(
                    character,
                    '_' | '-' | '.' | ':'
                ))
    }) {
        return Err(ResultError::InvalidMetadataKey);
    }

    Ok(())
}

fn validate_metadata_value(
    value: &str,
) -> Result<(), ResultError> {
    if value.len() > MAX_METADATA_VALUE_LENGTH {
        return Err(ResultError::MetadataValueTooLong);
    }

    if value.chars().any(char::is_control) {
        return Err(ResultError::InvalidMetadataKey);
    }

    Ok(())
}

fn contains_secret_indicator(value: &str) -> bool {
    let value = value.to_ascii_lowercase();

    let indicators = [
        "api_key",
        "apikey",
        "access_token",
        "authorization",
        "password",
        "private_key",
        "secret",
        "cookie",
        "bearer ",
        "client_secret",
        "refresh_token",
    ];

    indicators
        .iter()
        .any(|indicator| value.contains(indicator))
}

fn checked_power_of_two(
    qubit_count: usize,
) -> Result<usize, ResultError> {
    if qubit_count >= usize::BITS as usize {
        return Err(ResultError::DimensionOverflow);
    }

    1usize
        .checked_shl(qubit_count as u32)
        .ok_or(ResultError::DimensionOverflow)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_values_must_be_finite() {
        assert!(Complex64::new(f64::NAN, 0.0).is_err());
        assert!(Complex64::new(0.0, f64::INFINITY).is_err());
    }

    #[test]
    fn measurement_counts_are_deterministic() {
        let mut counts = BTreeMap::new();

        counts.insert("1".to_string(), 3);
        counts.insert("0".to_string(), 7);

        let counts =
            MeasurementCounts::new(counts).expect("valid counts");

        assert_eq!(counts.total_shots(), 10);
        assert_eq!(
            counts.as_map().keys().collect::<Vec<_>>(),
            vec!["0", "1"]
        );
    }

    #[test]
    fn counts_convert_to_normalized_probabilities() {
        let mut counts = BTreeMap::new();

        counts.insert("0".to_string(), 3);
        counts.insert("1".to_string(), 1);

        let counts =
            MeasurementCounts::new(counts).expect("valid counts");

        let probabilities =
            counts.probabilities().expect("valid probabilities");

        assert!((probabilities.total_probability() - 1.0).abs() < 1e-12);
        assert_eq!(probabilities.get("0"), Some(0.75));
    }

    #[test]
    fn invalid_probability_is_rejected() {
        let mut probabilities = BTreeMap::new();

        probabilities.insert("0".to_string(), 2.0);

        assert!(
            ProbabilityDistribution::new(probabilities).is_err()
        );
    }

    #[test]
    fn invalid_probability_normalization_is_rejected() {
        let mut probabilities = BTreeMap::new();

        probabilities.insert("0".to_string(), 0.4);
        probabilities.insert("1".to_string(), 0.4);

        assert!(
            ProbabilityDistribution::new(probabilities).is_err()
        );
    }

    #[test]
    fn samples_can_be_converted_to_counts() {
        let samples = MeasurementSamples::new(vec![
            "0".to_string(),
            "1".to_string(),
            "1".to_string(),
        ])
        .expect("valid samples");

        let counts = samples.counts().expect("valid counts");

        assert_eq!(counts.total_shots(), 3);
        assert_eq!(counts.as_map().get("1"), Some(&2));
    }

    #[test]
    fn state_vector_requires_power_of_two_dimension() {
        let amplitudes = vec![
            Complex64::one(),
            Complex64::zero(),
        ];

        let state =
            StateVector::new(1, amplitudes).expect("valid state");

        assert_eq!(state.qubit_count(), 1);
        assert!((state.norm_squared() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn invalid_state_vector_is_rejected() {
        let amplitudes = vec![
            Complex64::new(0.5, 0.0).expect("finite"),
            Complex64::new(0.5, 0.0).expect("finite"),
        ];

        assert!(StateVector::new(1, amplitudes).is_err());
    }

    #[test]
    fn valid_density_matrix_is_accepted() {
        let elements = vec![
            Complex64::one(),
            Complex64::zero(),
            Complex64::zero(),
            Complex64::zero(),
        ];

        let matrix =
            DensityMatrix::new(1, elements)
                .expect("valid density matrix");

        assert_eq!(matrix.dimension(), 2);
    }

    #[test]
    fn non_hermitian_density_matrix_is_rejected() {
        let elements = vec![
            Complex64::one(),
            Complex64::new(1.0, 0.0).expect("finite"),
            Complex64::zero(),
            Complex64::zero(),
        ];

        assert!(
            DensityMatrix::new(1, elements).is_err()
        );
    }

    #[test]
    fn metadata_rejects_secret_like_fields() {
        let mut metadata = ResultMetadata::empty();

        assert!(
            metadata
                .insert("api_key", "redacted")
                .is_err()
        );

        assert!(
            metadata
                .insert("provider", "example")
                .is_ok()
        );
    }

    #[test]
    fn complete_result_requires_payload() {
        let provenance =
            ResultProvenance::new("local/simulator")
                .expect("valid provenance");

        let result = QuantumExecutionResult::new(provenance);

        assert!(result.validate().is_err());
    }

    #[test]
    fn complete_result_with_counts_is_valid() {
        let provenance =
            ResultProvenance::new("local/simulator")
                .expect("valid provenance");

        let mut result =
            QuantumExecutionResult::new(provenance)
                .with_requested_shots(2)
                .with_completed_shots(2);

        let mut counts = BTreeMap::new();
        counts.insert("0".to_string(), 1);
        counts.insert("1".to_string(), 1);

        let counts =
            MeasurementCounts::new(counts)
                .expect("valid counts");

        result
            .add_payload(ResultPayload::Counts(counts))
            .expect("payload accepted");

        assert!(result.validate().is_ok());
        assert!(result.is_complete());
    }

    #[test]
    fn complete_result_cannot_claim_more_shots_than_requested() {
        let provenance =
            ResultProvenance::new("local/simulator")
                .expect("valid provenance");

        let mut result =
            QuantumExecutionResult::new(provenance)
                .with_requested_shots(2)
                .with_completed_shots(3);

        let mut counts = BTreeMap::new();
        counts.insert("0".to_string(), 3);

        let counts =
            MeasurementCounts::new(counts)
                .expect("valid counts");

        result
            .add_payload(ResultPayload::Counts(counts))
            .expect("payload accepted");

        assert!(result.validate().is_err());
    }

    #[test]
    fn complete_result_must_represent_all_requested_shots() {
        let provenance =
            ResultProvenance::new("local/simulator")
                .expect("valid provenance");

        let mut result =
            QuantumExecutionResult::new(provenance)
                .with_requested_shots(10)
                .with_completed_shots(9);

        let mut counts = BTreeMap::new();
        counts.insert("0".to_string(), 9);

        let counts =
            MeasurementCounts::new(counts)
                .expect("valid counts");

        result
            .add_payload(ResultPayload::Counts(counts))
            .expect("payload accepted");

        assert!(result.validate().is_err());
    }

    #[test]
    fn annealing_values_are_restricted() {
        let mut assignment = BTreeMap::new();
        assignment.insert("x".to_string(), 2);

        assert!(
            AnnealingSample::new(
                assignment,
                1.0,
                1
            )
            .is_err()
        );
    }

    #[test]
    fn pulse_sample_rate_must_be_positive() {
        let samples = vec![
            AcquisitionSample::new(0.0, 1.0, 0.0)
                .expect("valid sample")
        ];

        let acquisition =
            PulseAcquisition::new("drive0", samples)
                .expect("valid acquisition");

        assert!(
            acquisition
                .with_sample_rate(0.0)
                .is_err()
        );
    }
}