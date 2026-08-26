//! Zamani Quantum Benchmarking — Cross-Entropy Benchmarking (XEB)
//!
//! Production-grade Cross-Entropy Benchmarking protocol boundary.
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - XEB configuration;
//! - XEB protocol identity/versioning;
//! - ideal-probability model metadata;
//! - normalized execution contracts;
//! - Linear XEB;
//! - cross entropy;
//! - cross-entropy difference;
//! - statistical validation;
//! - deterministic circuit-seed derivation;
//! - XEB result construction;
//! - XEB result validation;
//! - backend-neutral generator/executor traits.
//!
//! It deliberately does NOT own:
//!
//! - Zamani source parsing;
//! - OpenQASM parsing;
//! - Quantum IR construction;
//! - circuit compilation;
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware selection;
//! - simulator implementation;
//! - network I/O;
//! - report serialization beyond the protocol result schema;
//! - backend-specific execution objects.
//!
//! # Mathematical definition
//!
//! For an `n`-qubit circuit C, let:
//!
//! `q_C(x)`
//!
//! be the ideal probability of output bitstring `x`.
//!
//! If the experimental device produces samples `x_i`, Linear XEB is:
//!
//! `F_XEB = 2^n * mean_i(q_C(x_i)) - 1`
//!
//! For shot-count observations this is evaluated as a weighted mean.
//!
//! The implementation also exposes:
//!
//! `H(q) = -sum_x q(x) ln(q(x))`
//!
//! and the observed-to-ideal cross entropy:
//!
//! `H(p_exp, q) = -sum_x p_exp(x) ln(q(x))`
//!
//! with cross-entropy difference:
//!
//! `DeltaH = H(q) - H(p_exp, q)`.
//!
//! # Important scientific limitation
//!
//! Linear XEB must NOT automatically be treated as a physical fidelity.
//!
//! Under suitable random-circuit and noise assumptions it can estimate a
//! circuit fidelity, but that interpretation is model-dependent. In
//! particular, Linear XEB can be negative or greater than one for finite
//! samples or non-Porter-Thomas / non-random distributions.
//!
//! Therefore `XebResult::linear_xeb_mean` is explicitly a benchmark score.
//!
//! # Ideal-model boundary
//!
//! XEB requires ideal probabilities for observed outputs. Exact classical
//! simulation becomes exponentially expensive with increasing qubit count.
//!
//! This module therefore records whether the ideal model is:
//!
//! - Exact;
//! - Approximate;
//! - Partial.
//!
//! A partial or approximate model is never silently represented as exact.
//!
//! # Architectural integration
//!
//! ```text
//! generators/random_circuits.rs
//!             │
//!             ▼
//!       Quantum IR circuit
//!             │
//!             ▼
//! execution/executor.rs
//!             │
//!             ▼
//!       XebExecution
//!             │
//!             ▼
//! protocols/xeb.rs
//!       ┌─────┴─────┐
//!       ▼           ▼
//! statistics    core::result
//! ::bootstrap        │
//!       │            ▼
//!       └──────► reporting / analysis / CI
//! ```
//!
//! Existing `core::observation` can be adapted into `XebExecution` by an
//! adapter without changing this protocol file. Existing
//! `statistics::bootstrap` can consume `XebResult::circuit_xeb_values()`.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - no nightly features
//! - no unsafe code

#![deny(unsafe_code)]

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

// =============================================================================
// Public protocol identity
// =============================================================================

/// Stable benchmark identifier.
pub const XEB_BENCHMARK_ID: &str = "xeb";

/// Semantic XEB protocol version.
///
/// Increment this when the mathematical interpretation, execution contract,
/// or serialized result semantics change incompatibly.
pub const XEB_PROTOCOL_VERSION: &str = "1.0.0";

/// Current XEB result schema version.
pub const XEB_RESULT_SCHEMA_VERSION: u16 = 1;

/// Default confidence level used for the descriptive normal interval.
pub const DEFAULT_CONFIDENCE_LEVEL: f64 = 0.95;

/// Default deterministic experiment seed.
pub const DEFAULT_SEED: u64 = 0x5A4D_5845_4200_0001;

/// Maximum number of circuits in one experiment.
pub const DEFAULT_MAX_CIRCUITS: usize = 10_000;

/// Maximum shots per circuit.
pub const DEFAULT_MAX_SHOTS_PER_CIRCUIT: u64 = 10_000_000;

/// Maximum total shots in one experiment.
pub const DEFAULT_MAX_TOTAL_SHOTS: u64 = 1_000_000_000;

/// Maximum qubit width accepted by this protocol boundary.
pub const DEFAULT_MAX_QUBITS: usize = 65_536;

/// Numerical tolerance for probability normalization.
pub const DISTRIBUTION_TOLERANCE: f64 = 1.0e-12;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by XEB configuration, validation, analysis, and execution
/// adapters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum XebError {
    InvalidQubitCount {
        value: usize,
    },

    InvalidCircuitCount {
        value: usize,
    },

    InvalidShotCount {
        value: u64,
    },

    InvalidConfidenceLevel {
        value: f64,
    },

    InvalidSeed,

    CircuitLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    ShotsPerCircuitLimitExceeded {
        requested: u64,
        maximum: u64,
    },

    TotalShotsLimitExceeded {
        requested: u64,
        maximum: u64,
    },

    QubitLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    ArithmeticOverflow {
        operation: &'static str,
    },

    EmptyIdealDistribution,

    InvalidIdealDistributionSum {
        sum: f64,
    },

    InvalidProbability {
        bitstring: String,
        probability: f64,
    },

    InvalidBitstring {
        bitstring: String,
        expected_bits: usize,
    },

    EmptyObservedSamples,

    InvalidObservedShotCount {
        bitstring: String,
        shots: u64,
    },

    ObservedShotsMismatch {
        expected: u64,
        actual: u64,
    },

    MissingIdealProbability {
        bitstring: String,
    },

    ZeroIdealProbability {
        bitstring: String,
    },

    NonFiniteProbability {
        bitstring: String,
        probability: f64,
    },

    NonFiniteStatistic {
        statistic: &'static str,
        value: f64,
    },

    InvalidInterval {
        lower: f64,
        upper: f64,
    },

    UnsupportedIdealModel {
        reason: String,
    },

    IncompleteIdealDistribution {
        covered_probability: f64,
    },

    CircuitCountMismatch {
        expected: usize,
        actual: usize,
    },

    CircuitWidthMismatch {
        expected: usize,
        actual: usize,
    },

    Generation(String),

    Execution(String),

    Cancelled,

    Timeout,
}

impl fmt::Display for XebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubitCount { value } => {
                write!(
                    f,
                    "XEB qubit count must be greater than zero, got {value}"
                )
            }

            Self::InvalidCircuitCount { value } => {
                write!(
                    f,
                    "XEB circuit count must be greater than zero, got {value}"
                )
            }

            Self::InvalidShotCount { value } => {
                write!(
                    f,
                    "XEB shots per circuit must be greater than zero, got {value}"
                )
            }

            Self::InvalidConfidenceLevel { value } => {
                write!(
                    f,
                    "XEB confidence level must be finite and in (0, 1), got {value}"
                )
            }

            Self::InvalidSeed => {
                write!(f, "XEB seed configuration is invalid")
            }

            Self::CircuitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "XEB circuit count {requested} exceeds maximum {maximum}"
                )
            }

            Self::ShotsPerCircuitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "XEB shots per circuit {requested} exceeds maximum {maximum}"
                )
            }

            Self::TotalShotsLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "XEB total shots {requested} exceeds maximum {maximum}"
                )
            }

            Self::QubitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "XEB qubit count {requested} exceeds maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { operation } => {
                write!(f, "XEB arithmetic overflow during {operation}")
            }

            Self::EmptyIdealDistribution => {
                write!(f, "XEB ideal distribution is empty")
            }

            Self::InvalidIdealDistributionSum { sum } => {
                write!(
                    f,
                    "XEB ideal distribution probabilities sum to {sum}, expected 1"
                )
            }

            Self::InvalidProbability {
                bitstring,
                probability,
            } => {
                write!(
                    f,
                    "invalid ideal probability {probability} for output '{bitstring}'"
                )
            }

            Self::InvalidBitstring {
                bitstring,
                expected_bits,
            } => {
                write!(
                    f,
                    "invalid XEB bitstring '{bitstring}', expected exactly \
                     {expected_bits} binary bits"
                )
            }

            Self::EmptyObservedSamples => {
                write!(f, "XEB requires at least one observed sample")
            }

            Self::InvalidObservedShotCount {
                bitstring,
                shots,
            } => {
                write!(
                    f,
                    "observed output '{bitstring}' has invalid shot count {shots}"
                )
            }

            Self::ObservedShotsMismatch { expected, actual } => {
                write!(
                    f,
                    "observed shot count mismatch: expected {expected}, got {actual}"
                )
            }

            Self::MissingIdealProbability { bitstring } => {
                write!(
                    f,
                    "no ideal probability is available for observed output \
                     '{bitstring}'"
                )
            }

            Self::ZeroIdealProbability { bitstring } => {
                write!(
                    f,
                    "ideal probability for observed output '{bitstring}' is zero; \
                     logarithmic cross entropy is undefined"
                )
            }

            Self::NonFiniteProbability {
                bitstring,
                probability,
            } => {
                write!(
                    f,
                    "non-finite ideal probability {probability} for output \
                     '{bitstring}'"
                )
            }

            Self::NonFiniteStatistic { statistic, value } => {
                write!(
                    f,
                    "XEB statistic '{statistic}' is non-finite: {value}"
                )
            }

            Self::InvalidInterval { lower, upper } => {
                write!(
                    f,
                    "invalid XEB confidence interval [{lower}, {upper}]"
                )
            }

            Self::UnsupportedIdealModel { reason } => {
                write!(f, "XEB ideal-model limitation: {reason}")
            }

            Self::IncompleteIdealDistribution {
                covered_probability,
            } => {
                write!(
                    f,
                    "ideal distribution is incomplete; covered probability is \
                     {covered_probability}"
                )
            }

            Self::CircuitCountMismatch { expected, actual } => {
                write!(
                    f,
                    "XEB circuit count mismatch: expected {expected}, got {actual}"
                )
            }

            Self::CircuitWidthMismatch { expected, actual } => {
                write!(
                    f,
                    "XEB circuit width mismatch: expected {expected}, got {actual}"
                )
            }

            Self::Generation(message) => {
                write!(f, "XEB circuit generation failed: {message}")
            }

            Self::Execution(message) => {
                write!(f, "XEB execution failed: {message}")
            }

            Self::Cancelled => {
                write!(f, "XEB execution was cancelled")
            }

            Self::Timeout => {
                write!(f, "XEB execution timed out")
            }
        }
    }
}

impl Error for XebError {}

// =============================================================================
// Ideal-model provenance
// =============================================================================

/// Source/quality of the ideal probability model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdealModelKind {
    /// Exact probabilities from an exact classical reference computation.
    Exact,

    /// Probabilities produced by an explicitly approximate reference method.
    Approximate,

    /// Only part of the ideal distribution is available.
    Partial,
}

impl IdealModelKind {
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }
}

/// Metadata describing the ideal/reference distribution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XebIdealModel {
    /// Exact/approximate/partial classification.
    pub kind: IdealModelKind,

    /// Name of the reference implementation.
    pub source: String,

    /// Version of that reference implementation.
    pub algorithm_version: String,

    /// Whether the distribution covers the complete computational basis.
    pub complete: bool,

    /// Probability mass represented by the supplied distribution.
    pub covered_probability: f64,
}

impl XebIdealModel {
    /// Creates metadata for a complete exact ideal distribution.
    pub fn exact(
        source: impl Into<String>,
        algorithm_version: impl Into<String>,
    ) -> Self {
        Self {
            kind: IdealModelKind::Exact,
            source: source.into(),
            algorithm_version: algorithm_version.into(),
            complete: true,
            covered_probability: 1.0,
        }
    }

    /// Creates metadata for an approximate reference distribution.
    pub fn approximate(
        source: impl Into<String>,
        algorithm_version: impl Into<String>,
        covered_probability: f64,
    ) -> Result<Self, XebError> {
        validate_unit_probability(covered_probability, "ideal coverage")?;

        Ok(Self {
            kind: IdealModelKind::Approximate,
            source: source.into(),
            algorithm_version: algorithm_version.into(),
            complete: false,
            covered_probability,
        })
    }

    /// Creates metadata for a partial reference distribution.
    pub fn partial(
        source: impl Into<String>,
        algorithm_version: impl Into<String>,
        covered_probability: f64,
    ) -> Result<Self, XebError> {
        validate_unit_probability(covered_probability, "ideal coverage")?;

        Ok(Self {
            kind: IdealModelKind::Partial,
            source: source.into(),
            algorithm_version: algorithm_version.into(),
            complete: false,
            covered_probability,
        })
    }

    /// Validates model metadata.
    pub fn validate(&self) -> Result<(), XebError> {
        if self.source.trim().is_empty() {
            return Err(XebError::UnsupportedIdealModel {
                reason: "ideal-model source must not be empty".into(),
            });
        }

        if self.algorithm_version.trim().is_empty() {
            return Err(XebError::UnsupportedIdealModel {
                reason: "ideal-model algorithm version must not be empty".into(),
            });
        }

        validate_unit_probability(
            self.covered_probability,
            "ideal coverage",
        )?;

        if self.kind.is_exact()
            && (!self.complete
                || (self.covered_probability - 1.0).abs()
                    > DISTRIBUTION_TOLERANCE)
        {
            return Err(XebError::UnsupportedIdealModel {
                reason:
                    "an exact ideal model must be complete and cover probability 1"
                        .into(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Complete XEB experiment configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XebConfig {
    /// Number of qubits in every benchmark circuit.
    pub num_qubits: usize,

    /// Number of randomized circuits.
    pub circuits: usize,

    /// Number of shots requested for each circuit.
    pub shots_per_circuit: u64,

    /// Two-sided confidence level for the descriptive normal interval.
    pub confidence_level: f64,

    /// Experiment seed.
    pub seed: u64,

    /// Maximum permitted qubit count.
    pub max_qubits: usize,

    /// Maximum permitted circuit count.
    pub max_circuits: usize,

    /// Maximum shots per circuit.
    pub max_shots_per_circuit: u64,

    /// Maximum total shots.
    pub max_total_shots: u64,

    /// Whether exact complete ideal probabilities are required.
    pub require_exact_ideal_model: bool,
}

impl Default for XebConfig {
    fn default() -> Self {
        Self {
            num_qubits: 1,
            circuits: 100,
            shots_per_circuit: 1_000,
            confidence_level: DEFAULT_CONFIDENCE_LEVEL,
            seed: DEFAULT_SEED,
            max_qubits: DEFAULT_MAX_QUBITS,
            max_circuits: DEFAULT_MAX_CIRCUITS,
            max_shots_per_circuit: DEFAULT_MAX_SHOTS_PER_CIRCUIT,
            max_total_shots: DEFAULT_MAX_TOTAL_SHOTS,
            require_exact_ideal_model: true,
        }
    }
}

impl XebConfig {
    /// Creates a validated XEB configuration.
    pub fn new(
        num_qubits: usize,
        circuits: usize,
        shots_per_circuit: u64,
        seed: u64,
    ) -> Result<Self, XebError> {
        let config = Self {
            num_qubits,
            circuits,
            shots_per_circuit,
            seed,
            ..Self::default()
        };

        config.validate()?;

        Ok(config)
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<(), XebError> {
        if self.num_qubits == 0 {
            return Err(XebError::InvalidQubitCount {
                value: self.num_qubits,
            });
        }

        if self.circuits == 0 {
            return Err(XebError::InvalidCircuitCount {
                value: self.circuits,
            });
        }

        if self.shots_per_circuit == 0 {
            return Err(XebError::InvalidShotCount {
                value: self.shots_per_circuit,
            });
        }

        if !self.confidence_level.is_finite()
            || !(0.0 < self.confidence_level
                && self.confidence_level < 1.0)
        {
            return Err(XebError::InvalidConfidenceLevel {
                value: self.confidence_level,
            });
        }

        if self.max_qubits == 0 || self.num_qubits > self.max_qubits {
            return Err(XebError::QubitLimitExceeded {
                requested: self.num_qubits,
                maximum: self.max_qubits,
            });
        }

        if self.max_circuits == 0 || self.circuits > self.max_circuits {
            return Err(XebError::CircuitLimitExceeded {
                requested: self.circuits,
                maximum: self.max_circuits,
            });
        }

        if self.shots_per_circuit > self.max_shots_per_circuit {
            return Err(XebError::ShotsPerCircuitLimitExceeded {
                requested: self.shots_per_circuit,
                maximum: self.max_shots_per_circuit,
            });
        }

        let total = (self.circuits as u64)
            .checked_mul(self.shots_per_circuit)
            .ok_or(XebError::ArithmeticOverflow {
                operation: "total shots",
            })?;

        if total > self.max_total_shots {
            return Err(XebError::TotalShotsLimitExceeded {
                requested: total,
                maximum: self.max_total_shots,
            });
        }

        Ok(())
    }

    /// Returns the configured total shot count.
    #[must_use]
    pub fn total_shots(&self) -> u64 {
        (self.circuits as u64)
            .saturating_mul(self.shots_per_circuit)
    }
}

// =============================================================================
// Circuit and execution-neutral contracts
// =============================================================================

/// Minimal circuit metadata needed by the XEB protocol.
///
/// The actual circuit can remain a Zamani Quantum IR object or any backend
/// adapter implementing `XebCircuitGenerator`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XebCircuitMetadata {
    pub circuit_id: String,
    pub num_qubits: usize,
    pub depth: usize,
    pub gate_count: u64,
    pub two_qubit_gate_count: u64,
    pub seed: u64,
}

impl XebCircuitMetadata {
    pub fn validate(
        &self,
        expected_qubits: usize,
    ) -> Result<(), XebError> {
        if self.circuit_id.trim().is_empty() {
            return Err(XebError::Generation(
                "circuit_id must not be empty".into(),
            ));
        }

        if self.num_qubits != expected_qubits {
            return Err(XebError::CircuitWidthMismatch {
                expected: expected_qubits,
                actual: self.num_qubits,
            });
        }

        Ok(())
    }
}

/// One observed computational-basis output and its shot multiplicity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XebObservedSample {
    pub bitstring: String,
    pub shots: u64,
}

impl XebObservedSample {
    pub fn new(
        bitstring: impl Into<String>,
        shots: u64,
    ) -> Self {
        Self {
            bitstring: bitstring.into(),
            shots,
        }
    }

    pub fn validate(
        &self,
        num_qubits: usize,
    ) -> Result<(), XebError> {
        validate_bitstring(&self.bitstring, num_qubits)?;

        if self.shots == 0 {
            return Err(XebError::InvalidObservedShotCount {
                bitstring: self.bitstring.clone(),
                shots: self.shots,
            });
        }

        Ok(())
    }
}

/// Normalized observations from one executed circuit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XebCircuitExecution {
    pub circuit: XebCircuitMetadata,
    pub samples: Vec<XebObservedSample>,
    pub total_shots: u64,
}

impl XebCircuitExecution {
    pub fn new(
        circuit: XebCircuitMetadata,
        samples: Vec<XebObservedSample>,
    ) -> Result<Self, XebError> {
        let total = checked_sample_total(&samples)?;

        Ok(Self {
            circuit,
            samples,
            total_shots: total,
        })
    }

    pub fn validate(
        &self,
        expected_qubits: usize,
        expected_shots: Option<u64>,
    ) -> Result<(), XebError> {
        self.circuit.validate(expected_qubits)?;

        if self.samples.is_empty() {
            return Err(XebError::EmptyObservedSamples);
        }

        for sample in &self.samples {
            sample.validate(expected_qubits)?;
        }

        let actual = checked_sample_total(&self.samples)?;

        if actual != self.total_shots {
            return Err(XebError::ObservedShotsMismatch {
                expected: self.total_shots,
                actual,
            });
        }

        if let Some(expected) = expected_shots {
            if actual != expected {
                return Err(XebError::ObservedShotsMismatch {
                    expected,
                    actual,
                });
            }
        }

        Ok(())
    }
}

/// Complete normalized execution set for an XEB experiment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XebExecution {
    pub circuits: Vec<XebCircuitExecution>,
}

impl XebExecution {
    pub fn validate(
        &self,
        config: &XebConfig,
    ) -> Result<(), XebError> {
        if self.circuits.len() != config.circuits {
            return Err(XebError::CircuitCountMismatch {
                expected: config.circuits,
                actual: self.circuits.len(),
            });
        }

        for circuit in &self.circuits {
            circuit.validate(
                config.num_qubits,
                Some(config.shots_per_circuit),
            )?;
        }

        let total = self.total_shots()?;

        if total > config.max_total_shots {
            return Err(XebError::TotalShotsLimitExceeded {
                requested: total,
                maximum: config.max_total_shots,
            });
        }

        Ok(())
    }

    pub fn total_shots(&self) -> Result<u64, XebError> {
        self.circuits.iter().try_fold(
            0_u64,
            |acc, circuit| {
                acc.checked_add(circuit.total_shots).ok_or(
                    XebError::ArithmeticOverflow {
                        operation: "execution total shots",
                    },
                )
            },
        )
    }
}

// =============================================================================
// Backend-neutral integration traits
// =============================================================================

/// Generates an XEB circuit.
///
/// The concrete circuit can be a Zamani Quantum IR circuit or an adapter
/// object. This file intentionally does not depend on the concrete IR type.
pub trait XebCircuitGenerator {
    type Circuit;

    fn generate(
        &mut self,
        config: &XebConfig,
        circuit_index: usize,
        seed: u64,
    ) -> Result<(Self::Circuit, XebCircuitMetadata), XebError>;
}

/// Executes one generated circuit and returns normalized sampled outputs.
pub trait XebExecutor<C> {
    fn execute(
        &mut self,
        circuit: &C,
        shots: u64,
    ) -> Result<Vec<XebObservedSample>, XebError>;
}

// =============================================================================
// Result model
// =============================================================================

/// XEB result for one circuit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XebCircuitResult {
    pub circuit_id: String,
    pub num_qubits: usize,
    pub depth: usize,
    pub shots: u64,

    /// Linear XEB score for this circuit.
    pub linear_xeb: f64,

    /// Mean ideal probability of observed samples.
    pub mean_ideal_probability: f64,

    /// Observed-to-ideal cross entropy in natural-log units.
    pub cross_entropy: Option<f64>,

    /// Ideal Shannon entropy in natural-log units.
    pub ideal_entropy: Option<f64>,

    /// Cross-entropy difference.
    pub cross_entropy_difference: Option<f64>,
}

/// Confidence interval method used for the aggregate XEB score.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum XebIntervalMethod {
    /// Normal approximation over per-circuit XEB estimates.
    ///
    /// The existing `statistics::bootstrap` subsystem should be used for
    /// publication-grade non-parametric uncertainty when appropriate.
    NormalApproximation,
}

/// Confidence interval for the aggregate XEB score.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct XebInterval {
    pub lower: f64,
    pub upper: f64,
    pub confidence_level: f64,
    pub method: XebIntervalMethod,
}

impl XebInterval {
    pub fn validate(
        &self,
        expected_confidence: f64,
    ) -> Result<(), XebError> {
        if !self.lower.is_finite()
            || !self.upper.is_finite()
            || self.lower > self.upper
            || !self.confidence_level.is_finite()
            || (self.confidence_level - expected_confidence).abs()
                > 1.0e-15
        {
            return Err(XebError::InvalidInterval {
                lower: self.lower,
                upper: self.upper,
            });
        }

        Ok(())
    }
}

/// Complete XEB benchmark result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XebResult {
    pub schema_version: u16,
    pub benchmark_id: String,
    pub protocol_version: String,

    pub num_qubits: usize,
    pub circuits: usize,
    pub total_shots: u64,

    pub seed: u64,
    pub confidence_level: f64,

    pub ideal_model: XebIdealModel,

    pub circuit_results: Vec<XebCircuitResult>,

    /// Mean Linear XEB score across circuits.
    pub linear_xeb_mean: f64,

    /// Minimum per-circuit XEB score.
    pub linear_xeb_min: f64,

    /// Maximum per-circuit XEB score.
    pub linear_xeb_max: f64,

    /// Standard error across circuit-level XEB scores.
    pub linear_xeb_standard_error: f64,

    pub linear_xeb_confidence_interval: XebInterval,

    pub aggregate_cross_entropy: Option<f64>,
    pub aggregate_ideal_entropy: Option<f64>,
    pub aggregate_cross_entropy_difference: Option<f64>,

    /// True when this result is descriptive rather than an automatic physical
    /// fidelity claim.
    pub statistically_descriptive: bool,
}

impl XebResult {
    pub fn validate(&self) -> Result<(), XebError> {
        if self.schema_version != XEB_RESULT_SCHEMA_VERSION {
            return Err(XebError::UnsupportedIdealModel {
                reason: format!(
                    "unsupported XEB result schema version {}",
                    self.schema_version
                ),
            });
        }

        if self.benchmark_id != XEB_BENCHMARK_ID {
            return Err(XebError::UnsupportedIdealModel {
                reason: format!(
                    "unexpected benchmark identifier '{}'",
                    self.benchmark_id
                ),
            });
        }

        if self.protocol_version.trim().is_empty() {
            return Err(XebError::UnsupportedIdealModel {
                reason: "protocol version is empty".into(),
            });
        }

        if self.num_qubits == 0 {
            return Err(XebError::InvalidQubitCount {
                value: self.num_qubits,
            });
        }

        if self.circuits == 0 {
            return Err(XebError::InvalidCircuitCount {
                value: self.circuits,
            });
        }

        if self.total_shots == 0 {
            return Err(XebError::InvalidShotCount {
                value: self.total_shots,
            });
        }

        if !self.linear_xeb_mean.is_finite() {
            return Err(XebError::NonFiniteStatistic {
                statistic: "linear XEB aggregate",
                value: self.linear_xeb_mean,
            });
        }

        if !self.linear_xeb_standard_error.is_finite() {
            return Err(XebError::NonFiniteStatistic {
                statistic: "linear XEB standard error",
                value: self.linear_xeb_standard_error,
            });
        }

        self.ideal_model.validate()?;

        self.linear_xeb_confidence_interval
            .validate(self.confidence_level)?;

        if self.circuit_results.len() != self.circuits {
            return Err(XebError::CircuitCountMismatch {
                expected: self.circuits,
                actual: self.circuit_results.len(),
            });
        }

        Ok(())
    }

    /// Returns circuit-level XEB scores.
    ///
    /// This is the intended input for the existing
    /// `statistics::bootstrap` module.
    #[must_use]
    pub fn circuit_xeb_values(&self) -> Vec<f64> {
        self.circuit_results
            .iter()
            .map(|result| result.linear_xeb)
            .collect()
    }
}

// =============================================================================
// Protocol
// =============================================================================

/// Stateless XEB protocol coordinator.
#[derive(Debug, Default, Clone, Copy)]
pub struct XebProtocol;

impl XebProtocol {
    pub const fn new() -> Self {
        Self
    }

    /// Runs the complete backend-neutral XEB workflow.
    ///
    /// Circuit generation and execution are supplied by adapters. The ideal
    /// probability model is supplied by the caller because calculating it may
    /// require a state-vector simulator, tensor-network simulator, stabilizer
    /// approximation, patch simulation, or another reference implementation.
    pub fn run<G, E>(
        &self,
        config: &XebConfig,
        ideal_model: XebIdealModel,
        ideal_probabilities: &BTreeMap<String, f64>,
        generator: &mut G,
        executor: &mut E,
    ) -> Result<XebResult, XebError>
    where
        G: XebCircuitGenerator,
        E: XebExecutor<G::Circuit>,
    {
        config.validate()?;

        validate_ideal_distribution(
            ideal_probabilities,
            config.num_qubits,
            &ideal_model,
            config.require_exact_ideal_model,
        )?;

        let mut executions =
            Vec::with_capacity(config.circuits);

        for circuit_index in 0..config.circuits {
            let seed = derive_circuit_seed(
                config.seed,
                circuit_index as u64,
            );

            let (circuit, metadata) =
                generator.generate(config, circuit_index, seed)?;

            metadata.validate(config.num_qubits)?;

            let samples =
                executor.execute(&circuit, config.shots_per_circuit)?;

            let execution =
                XebCircuitExecution::new(metadata, samples)?;

            execution.validate(
                config.num_qubits,
                Some(config.shots_per_circuit),
            )?;

            executions.push(execution);
        }

        self.analyze(
            config,
            ideal_model,
            ideal_probabilities,
            &XebExecution {
                circuits: executions,
            },
        )
    }

    /// Analyzes already-normalized observations.
    ///
    /// This is the most important unit-test and re-analysis boundary.
    pub fn analyze(
        &self,
        config: &XebConfig,
        ideal_model: XebIdealModel,
        ideal_probabilities: &BTreeMap<String, f64>,
        execution: &XebExecution,
    ) -> Result<XebResult, XebError> {
        config.validate()?;

        ideal_model.validate()?;

        validate_ideal_distribution(
            ideal_probabilities,
            config.num_qubits,
            &ideal_model,
            config.require_exact_ideal_model,
        )?;

        execution.validate(config)?;

        let mut circuit_results =
            Vec::with_capacity(execution.circuits.len());

        for circuit in &execution.circuits {
            circuit_results.push(analyze_circuit(
                circuit,
                ideal_probabilities,
                config.num_qubits,
                &ideal_model,
            )?);
        }

        if circuit_results.is_empty() {
            return Err(XebError::EmptyObservedSamples);
        }

        let values: Vec<f64> = circuit_results
            .iter()
            .map(|result| result.linear_xeb)
            .collect();

        let mean_value = mean(&values)?;
        let standard_error = standard_error(&values)?;

        let quantile_probability =
            0.5 + config.confidence_level / 2.0;

        let z =
            inverse_standard_normal_cdf(quantile_probability)?;

        let margin = z * standard_error;

        let interval = XebInterval {
            lower: mean_value - margin,
            upper: mean_value + margin,
            confidence_level: config.confidence_level,
            method: XebIntervalMethod::NormalApproximation,
        };

        interval.validate(config.confidence_level)?;

        let total_shots = execution.total_shots()?;

        let aggregate_cross_entropy =
            weighted_optional_mean(
                &circuit_results,
                |result| result.cross_entropy,
            )?;

        let aggregate_ideal_entropy =
            weighted_optional_mean(
                &circuit_results,
                |result| result.ideal_entropy,
            )?;

        let aggregate_cross_entropy_difference =
            match (
                aggregate_ideal_entropy,
                aggregate_cross_entropy,
            ) {
                (Some(ideal), Some(cross)) => {
                    Some(ideal - cross)
                }
                _ => None,
            };

        let result = XebResult {
            schema_version: XEB_RESULT_SCHEMA_VERSION,
            benchmark_id: XEB_BENCHMARK_ID.to_string(),
            protocol_version: XEB_PROTOCOL_VERSION.to_string(),
            num_qubits: config.num_qubits,
            circuits: config.circuits,
            total_shots,
            seed: config.seed,
            confidence_level: config.confidence_level,
            ideal_model,
            circuit_results,
            linear_xeb_mean: mean_value,
            linear_xeb_min: values
                .iter()
                .copied()
                .fold(f64::INFINITY, f64::min),
            linear_xeb_max: values
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max),
            linear_xeb_standard_error: standard_error,
            linear_xeb_confidence_interval: interval,
            aggregate_cross_entropy,
            aggregate_ideal_entropy,
            aggregate_cross_entropy_difference,
            statistically_descriptive: true,
        };

        result.validate()?;

        Ok(result)
    }
}

// =============================================================================
// Circuit analysis
// =============================================================================

fn analyze_circuit(
    execution: &XebCircuitExecution,
    ideal: &BTreeMap<String, f64>,
    num_qubits: usize,
    model: &XebIdealModel,
) -> Result<XebCircuitResult, XebError> {
    let mut weighted_probability_sum = 0.0_f64;
    let mut weighted_log_probability_sum = 0.0_f64;

    for sample in &execution.samples {
        let probability = *ideal
            .get(&sample.bitstring)
            .ok_or_else(|| {
                XebError::MissingIdealProbability {
                    bitstring: sample.bitstring.clone(),
                }
            })?;

        validate_probability(
            &sample.bitstring,
            probability,
        )?;

        weighted_probability_sum +=
            sample.shots as f64 * probability;

        if probability <= 0.0 {
            return Err(XebError::ZeroIdealProbability {
                bitstring: sample.bitstring.clone(),
            });
        }

        weighted_log_probability_sum +=
            sample.shots as f64 * probability.ln();
    }

    let shots = execution.total_shots as f64;

    if shots <= 0.0 {
        return Err(XebError::EmptyObservedSamples);
    }

    let mean_probability =
        weighted_probability_sum / shots;

    let dimension = exp2_checked(num_qubits)?;

    let linear_xeb =
        dimension * mean_probability - 1.0;

    let cross_entropy =
        Some(-weighted_log_probability_sum / shots);

    let ideal_entropy = if model.complete {
        Some(shannon_entropy_natural(ideal)?)
    } else {
        None
    };

    let cross_entropy_difference =
        match (ideal_entropy, cross_entropy) {
            (Some(ideal), Some(cross)) => {
                Some(ideal - cross)
            }
            _ => None,
        };

    ensure_finite(
        "mean ideal probability",
        mean_probability,
    )?;

    ensure_finite(
        "linear XEB",
        linear_xeb,
    )?;

    if let Some(value) = cross_entropy {
        ensure_finite("cross entropy", value)?;
    }

    Ok(XebCircuitResult {
        circuit_id: execution.circuit.circuit_id.clone(),
        num_qubits,
        depth: execution.circuit.depth,
        shots: execution.total_shots,
        linear_xeb,
        mean_ideal_probability: mean_probability,
        cross_entropy,
        ideal_entropy,
        cross_entropy_difference,
    })
}

// =============================================================================
// Public analysis functions
// =============================================================================

/// Calculates Linear XEB directly from shot-count observations.
///
/// This is useful for re-analysis of existing experimental data without
/// executing the benchmark again.
pub fn linear_xeb_from_counts(
    num_qubits: usize,
    samples: &[XebObservedSample],
    ideal_probabilities: &BTreeMap<String, f64>,
) -> Result<f64, XebError> {
    if samples.is_empty() {
        return Err(XebError::EmptyObservedSamples);
    }

    for sample in samples {
        sample.validate(num_qubits)?;
    }

    validate_ideal_distribution_shape(
        ideal_probabilities,
        num_qubits,
    )?;

    let total =
        checked_sample_total(samples)? as f64;

    let mut weighted_probability_sum = 0.0_f64;

    for sample in samples {
        let probability = *ideal_probabilities
            .get(&sample.bitstring)
            .ok_or_else(|| {
                XebError::MissingIdealProbability {
                    bitstring: sample.bitstring.clone(),
                }
            })?;

        validate_probability(
            &sample.bitstring,
            probability,
        )?;

        weighted_probability_sum +=
            sample.shots as f64 * probability;
    }

    let score =
        exp2_checked(num_qubits)?
            * (weighted_probability_sum / total)
            - 1.0;

    ensure_finite("linear XEB", score)?;

    Ok(score)
}

/// Returns the average ideal probability assigned to observed samples.
pub fn mean_ideal_probability(
    samples: &[XebObservedSample],
    ideal_probabilities: &BTreeMap<String, f64>,
) -> Result<f64, XebError> {
    if samples.is_empty() {
        return Err(XebError::EmptyObservedSamples);
    }

    let total =
        checked_sample_total(samples)? as f64;

    let mut weighted_sum = 0.0_f64;

    for sample in samples {
        let probability = *ideal_probabilities
            .get(&sample.bitstring)
            .ok_or_else(|| {
                XebError::MissingIdealProbability {
                    bitstring: sample.bitstring.clone(),
                }
            })?;

        validate_probability(
            &sample.bitstring,
            probability,
        )?;

        weighted_sum +=
            sample.shots as f64 * probability;
    }

    let result = weighted_sum / total;

    ensure_finite(
        "mean ideal probability",
        result,
    )?;

    Ok(result)
}

/// Calculates observed-to-ideal cross entropy using natural logarithms.
pub fn cross_entropy(
    samples: &[XebObservedSample],
    ideal_probabilities: &BTreeMap<String, f64>,
) -> Result<f64, XebError> {
    if samples.is_empty() {
        return Err(XebError::EmptyObservedSamples);
    }

    let total =
        checked_sample_total(samples)? as f64;

    let mut log_sum = 0.0_f64;

    for sample in samples {
        let probability = *ideal_probabilities
            .get(&sample.bitstring)
            .ok_or_else(|| {
                XebError::MissingIdealProbability {
                    bitstring: sample.bitstring.clone(),
                }
            })?;

        validate_probability(
            &sample.bitstring,
            probability,
        )?;

        if probability <= 0.0 {
            return Err(XebError::ZeroIdealProbability {
                bitstring: sample.bitstring.clone(),
            });
        }

        log_sum +=
            sample.shots as f64 * probability.ln();
    }

    let result = -log_sum / total;

    ensure_finite("cross entropy", result)?;

    Ok(result)
}

/// Calculates the ideal Shannon entropy.
///
/// The supplied distribution must be complete and normalized.
pub fn ideal_entropy(
    ideal_probabilities: &BTreeMap<String, f64>,
) -> Result<f64, XebError> {
    let sum =
        validate_ideal_distribution_shape(
            ideal_probabilities,
            0,
        )?;

    if (sum - 1.0).abs()
        > DISTRIBUTION_TOLERANCE
    {
        return Err(
            XebError::InvalidIdealDistributionSum {
                sum,
            },
        );
    }

    shannon_entropy_natural(
        ideal_probabilities,
    )
}

/// Calculates cross-entropy difference.
///
/// `DeltaH = H(ideal) - H(observed, ideal)`.
pub fn cross_entropy_difference(
    samples: &[XebObservedSample],
    ideal_probabilities: &BTreeMap<String, f64>,
) -> Result<f64, XebError> {
    let ideal =
        ideal_entropy(ideal_probabilities)?;

    let cross =
        cross_entropy(
            samples,
            ideal_probabilities,
        )?;

    let result = ideal - cross;

    ensure_finite(
        "cross entropy difference",
        result,
    )?;

    Ok(result)
}

// =============================================================================
// Ideal-distribution validation
// =============================================================================

fn validate_ideal_distribution(
    distribution: &BTreeMap<String, f64>,
    num_qubits: usize,
    model: &XebIdealModel,
    require_exact: bool,
) -> Result<(), XebError> {
    model.validate()?;

    if distribution.is_empty() {
        return Err(XebError::EmptyIdealDistribution);
    }

    let sum =
        validate_ideal_distribution_shape(
            distribution,
            num_qubits,
        )?;

    if require_exact && !model.complete {
        return Err(
            XebError::UnsupportedIdealModel {
                reason:
                    "configuration requires a complete exact ideal model"
                        .into(),
            },
        );
    }

    if model.complete
        && (sum - 1.0).abs()
            > DISTRIBUTION_TOLERANCE
    {
        return Err(
            XebError::InvalidIdealDistributionSum {
                sum,
            },
        );
    }

    if !model.complete
        && sum > 1.0 + DISTRIBUTION_TOLERANCE
    {
        return Err(
            XebError::InvalidIdealDistributionSum {
                sum,
            },
        );
    }

    if !model.complete
        && (model.covered_probability - sum).abs()
            > DISTRIBUTION_TOLERANCE
    {
        return Err(
            XebError::IncompleteIdealDistribution {
                covered_probability: sum,
            },
        );
    }

    Ok(())
}

fn validate_ideal_distribution_shape(
    distribution: &BTreeMap<String, f64>,
    num_qubits: usize,
) -> Result<f64, XebError> {
    if distribution.is_empty() {
        return Err(XebError::EmptyIdealDistribution);
    }

    let mut sum = 0.0_f64;

    for (bitstring, probability) in distribution {
        if num_qubits > 0 {
            validate_bitstring(
                bitstring,
                num_qubits,
            )?;
        }

        validate_probability(
            bitstring,
            *probability,
        )?;

        sum += *probability;

        if !sum.is_finite() {
            return Err(
                XebError::ArithmeticOverflow {
                    operation:
                        "ideal probability sum",
                },
            );
        }
    }

    Ok(sum)
}

fn validate_probability(
    bitstring: &str,
    probability: f64,
) -> Result<(), XebError> {
    if !probability.is_finite() {
        return Err(
            XebError::NonFiniteProbability {
                bitstring: bitstring.to_string(),
                probability,
            },
        );
    }

    if !(0.0..=1.0).contains(&probability) {
        return Err(
            XebError::InvalidProbability {
                bitstring: bitstring.to_string(),
                probability,
            },
        );
    }

    Ok(())
}

fn validate_unit_probability(
    value: f64,
    field: &str,
) -> Result<(), XebError> {
    if !value.is_finite()
        || !(0.0..=1.0).contains(&value)
    {
        return Err(
            XebError::InvalidProbability {
                bitstring: field.to_string(),
                probability: value,
            },
        );
    }

    Ok(())
}

fn validate_bitstring(
    bitstring: &str,
    num_qubits: usize,
) -> Result<(), XebError> {
    if bitstring.len() != num_qubits
        || !bitstring
            .bytes()
            .all(|byte| byte == b'0' || byte == b'1')
    {
        return Err(
            XebError::InvalidBitstring {
                bitstring: bitstring.to_string(),
                expected_bits: num_qubits,
            },
        );
    }

    Ok(())
}

fn checked_sample_total(
    samples: &[XebObservedSample],
) -> Result<u64, XebError> {
    samples.iter().try_fold(
        0_u64,
        |acc, sample| {
            acc.checked_add(sample.shots)
                .ok_or(
                    XebError::ArithmeticOverflow {
                        operation:
                            "sample shot total",
                    },
                )
        },
    )
}

// =============================================================================
// Statistical helpers
// =============================================================================

fn mean(values: &[f64]) -> Result<f64, XebError> {
    if values.is_empty() {
        return Err(XebError::EmptyObservedSamples);
    }

    let mut sum = 0.0_f64;

    for value in values {
        if !value.is_finite() {
            return Err(
                XebError::NonFiniteStatistic {
                    statistic: "sample",
                    value: *value,
                },
            );
        }

        sum += *value;

        if !sum.is_finite() {
            return Err(
                XebError::ArithmeticOverflow {
                    operation: "sample mean",
                },
            );
        }
    }

    let result =
        sum / values.len() as f64;

    ensure_finite("mean", result)?;

    Ok(result)
}

fn standard_error(
    values: &[f64],
) -> Result<f64, XebError> {
    if values.len() < 2 {
        return Ok(0.0);
    }

    let average = mean(values)?;

    let mut sum_squared = 0.0_f64;

    for value in values {
        let difference = *value - average;
        sum_squared += difference * difference;

        if !sum_squared.is_finite() {
            return Err(
                XebError::ArithmeticOverflow {
                    operation:
                        "XEB variance",
                },
            );
        }
    }

    let variance =
        sum_squared
            / (values.len() - 1) as f64;

    let result =
        (variance / values.len() as f64).sqrt();

    ensure_finite(
        "standard error",
        result,
    )?;

    Ok(result)
}

fn weighted_optional_mean<F>(
    results: &[XebCircuitResult],
    accessor: F,
) -> Result<Option<f64>, XebError>
where
    F: Fn(&XebCircuitResult) -> Option<f64>,
{
    let mut total_weight = 0_u64;
    let mut weighted_sum = 0.0_f64;
    let mut present = false;

    for result in results {
        if let Some(value) = accessor(result) {
            ensure_finite(
                "weighted statistic",
                value,
            )?;

            total_weight =
                total_weight
                    .checked_add(result.shots)
                    .ok_or(
                        XebError::ArithmeticOverflow {
                            operation:
                                "weighted statistic shots",
                        },
                    )?;

            weighted_sum +=
                value * result.shots as f64;

            present = true;
        }
    }

    if !present {
        return Ok(None);
    }

    let result =
        weighted_sum / total_weight as f64;

    ensure_finite(
        "weighted statistic mean",
        result,
    )?;

    Ok(Some(result))
}

fn shannon_entropy_natural(
    distribution: &BTreeMap<String, f64>,
) -> Result<f64, XebError> {
    let mut entropy = 0.0_f64;

    for (bitstring, probability) in distribution {
        validate_probability(
            bitstring,
            *probability,
        )?;

        if *probability > 0.0 {
            entropy -=
                probability * probability.ln();
        }
    }

    ensure_finite(
        "ideal entropy",
        entropy,
    )?;

    Ok(entropy)
}

fn ensure_finite(
    statistic: &'static str,
    value: f64,
) -> Result<(), XebError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(XebError::NonFiniteStatistic {
            statistic,
            value,
        })
    }
}

// =============================================================================
// Reproducibility
// =============================================================================

/// Derives a deterministic independent seed for a circuit.
///
/// This is stream separation, not cryptographic randomness.
#[must_use]
pub fn derive_circuit_seed(
    experiment_seed: u64,
    circuit_index: u64,
) -> u64 {
    let mut value =
        experiment_seed.wrapping_add(
            0x9E37_79B9_7F4A_7C15u64
                .wrapping_mul(
                    circuit_index.wrapping_add(1),
                ),
        );

    value =
        (value ^ (value >> 30))
            .wrapping_mul(
                0xBF58_476D_1CE4_E5B9,
            );

    value =
        (value ^ (value >> 27))
            .wrapping_mul(
                0x94D0_49BB_1331_11EB,
            );

    value ^ (value >> 31)
}

/// Returns a deterministic RNG for one XEB circuit.
#[must_use]
pub fn rng_for_circuit(
    experiment_seed: u64,
    circuit_index: u64,
) -> StdRng {
    StdRng::seed_from_u64(
        derive_circuit_seed(
            experiment_seed,
            circuit_index,
        ),
    )
}

// =============================================================================
// Numerical helpers
// =============================================================================

/// Calculates `2^n` safely for XEB.
fn exp2_checked(
    num_qubits: usize,
) -> Result<f64, XebError> {
    // 2^1024 overflows f64.
    if num_qubits >= 1024 {
        return Err(
            XebError::ArithmeticOverflow {
                operation: "2^num_qubits in f64",
            },
        );
    }

    let result =
        2_f64.powi(num_qubits as i32);

    ensure_finite(
        "2^num_qubits",
        result,
    )?;

    Ok(result)
}

/// Inverse standard-normal CDF.
///
/// Acklam's rational approximation is used so the protocol does not require
/// an additional numerical dependency merely for confidence intervals.
fn inverse_standard_normal_cdf(
    probability: f64,
) -> Result<f64, XebError> {
    if !probability.is_finite()
        || !(0.0 < probability
            && probability < 1.0)
    {
        return Err(
            XebError::InvalidConfidenceLevel {
                value: probability,
            },
        );
    }

    const A: [f64; 6] = [
        -3.969_683_028_665_376e1,
        2.209_460_984_245_205e2,
        -2.759_285_104_469_687e2,
        1.383_577_518_672_69e2,
        -3.066_479_806_614_716e1,
        2.506_628_277_459_239,
    ];

    const B: [f64; 5] = [
        -5.447_609_879_822_406e1,
        1.615_858_368_580_409e2,
        -1.556_989_798_598_866e2,
        6.680_131_188_771_972e1,
        -1.328_068_155_288_572e1,
    ];

    const C: [f64; 6] = [
        -7.784_894_002_430_293e-3,
        -3.223_964_580_411_365e-1,
        -2.400_758_277_161_838,
        -2.549_732_539_343_734,
        4.374_664_141_464_968,
        2.938_163_982_698_783,
    ];

    const D: [f64; 4] = [
        7.784_695_709_041_462e-3,
        3.224_671_290_700_398e-1,
        2.445_134_137_142_996,
        3.754_408_661_907_416,
    ];

    const LOW: f64 = 0.024_25;
    const HIGH: f64 = 1.0 - LOW;

    let result = if probability < LOW {
        let q =
            (-2.0 * probability.ln()).sqrt();

        (((((C[0] * q + C[1]) * q + C[2])
            * q
            + C[3])
            * q
            + C[4])
            * q
            + C[5])
            / ((((D[0] * q + D[1]) * q + D[2])
                * q
                + D[3])
                * q
                + 1.0))
    } else if probability <= HIGH {
        let q =
            probability - 0.5;

        let r = q * q;

        (((((A[0] * r + A[1]) * r + A[2])
            * r
            + A[3])
            * r
            + A[4])
            * r
            + A[5])
            * q)
            / (((((B[0] * r + B[1]) * r + B[2])
                * r
                + B[3])
                * r
                + B[4])
                * r
                + 1.0))
    } else {
        let q =
            (-2.0 * (1.0 - probability).ln())
                .sqrt();

        -(((((C[0] * q + C[1]) * q + C[2])
            * q
            + C[3])
            * q
            + C[4])
            * q
            + C[5])
            / ((((D[0] * q + D[1]) * q + D[2])
                * q
                + D[3])
                * q
                + 1.0))
    };

    ensure_finite(
        "normal quantile",
        result,
    )?;

    Ok(result)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn ideal_two_qubit() -> BTreeMap<String, f64> {
        BTreeMap::from([
            ("00".to_string(), 0.25),
            ("01".to_string(), 0.25),
            ("10".to_string(), 0.25),
            ("11".to_string(), 0.25),
        ])
    }

    fn uniform_execution(shots: u64) -> XebExecution {
        let circuit = XebCircuitMetadata {
            circuit_id: "c0".into(),
            num_qubits: 2,
            depth: 2,
            gate_count: 4,
            two_qubit_gate_count: 1,
            seed: 7,
        };

        XebExecution {
            circuits: vec![
                XebCircuitExecution::new(
                    circuit,
                    vec![
                        XebObservedSample::new(
                            "00",
                            shots,
                        ),
                    ],
                )
                .unwrap(),
            ],
        }
    }

    #[test]
    fn uniform_two_qubit_distribution_has_zero_xeb() {
        let config =
            XebConfig::new(2, 1, 100, 42)
                .unwrap();

        let model =
            XebIdealModel::exact(
                "fixture",
                "1",
            );

        let result =
            XebProtocol::new()
                .analyze(
                    &config,
                    model,
                    &ideal_two_qubit(),
                    &uniform_execution(100),
                )
                .unwrap();

        assert!(
            result.linear_xeb_mean.abs()
                < 1.0e-12
        );
    }

    #[test]
    fn deterministic_output_can_exceed_one() {
        let ideal = BTreeMap::from([
            ("00".to_string(), 1.0),
            ("01".to_string(), 0.0),
            ("10".to_string(), 0.0),
            ("11".to_string(), 0.0),
        ]);

        let config =
            XebConfig::new(2, 1, 100, 42)
                .unwrap();

        let model =
            XebIdealModel::exact(
                "fixture",
                "1",
            );

        let result =
            XebProtocol::new()
                .analyze(
                    &config,
                    model,
                    &ideal,
                    &uniform_execution(100),
                )
                .unwrap();

        // For a deterministic 2-qubit ideal distribution:
        // 2^2 * 1 - 1 = 3.
        //
        // This is intentional and demonstrates why XEB must not blindly
        // impose a [0,1] bound.
        assert!(
            (result.linear_xeb_mean - 3.0).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn missing_ideal_probability_is_rejected() {
        let ideal =
            BTreeMap::from([
                ("00".to_string(), 1.0),
            ]);

        let config =
            XebConfig::new(2, 1, 100, 42)
                .unwrap();

        let model =
            XebIdealModel::partial(
                "fixture",
                "1",
                1.0,
            )
            .unwrap();

        let error =
            XebProtocol::new()
                .analyze(
                    &config,
                    model,
                    &ideal,
                    &uniform_execution(100),
                )
                .unwrap_err();

        assert!(matches!(
            error,
            XebError::UnsupportedIdealModel { .. }
        ));
    }

    #[test]
    fn invalid_bitstring_is_rejected() {
        let sample =
            XebObservedSample::new(
                "0x",
                1,
            );

        assert!(matches!(
            sample.validate(2),
            Err(
                XebError::InvalidBitstring { .. }
            )
        ));
    }

    #[test]
    fn zero_shots_are_rejected() {
        let sample =
            XebObservedSample::new(
                "00",
                0,
            );

        assert!(matches!(
            sample.validate(2),
            Err(
                XebError::InvalidObservedShotCount { .. }
            )
        ));
    }

    #[test]
    fn seed_derivation_is_deterministic() {
        let first =
            derive_circuit_seed(42, 0);

        let second =
            derive_circuit_seed(42, 0);

        let different =
            derive_circuit_seed(42, 1);

        assert_eq!(first, second);
        assert_ne!(first, different);

        let mut rng1 =
            rng_for_circuit(42, 0);

        let mut rng2 =
            rng_for_circuit(42, 0);

        assert_eq!(
            rng1.gen::<u64>(),
            rng2.gen::<u64>()
        );
    }

    #[test]
    fn two_sigma_quantile_is_two() {
        let probability =
            0.977_249_868_051_820_8;

        let z =
            inverse_standard_normal_cdf(
                probability,
            )
            .unwrap();

        assert!(
            (z - 2.0).abs()
                < 1.0e-6
        );
    }

    #[test]
    fn uniform_distribution_has_zero_cross_entropy_difference() {
        let ideal =
            ideal_two_qubit();

        let samples = vec![
            XebObservedSample::new(
                "00",
                100,
            ),
        ];

        let difference =
            cross_entropy_difference(
                &samples,
                &ideal,
            )
            .unwrap();

        assert!(
            difference.abs()
                < 1.0e-12
        );
    }

    #[test]
    fn sample_count_overflow_is_rejected() {
        let samples = vec![
            XebObservedSample::new(
                "00",
                u64::MAX,
            ),
            XebObservedSample::new(
                "01",
                1,
            ),
        ];

        assert!(matches!(
            checked_sample_total(&samples),
            Err(
                XebError::ArithmeticOverflow { .. }
            )
        ));
    }

    #[test]
    fn configuration_limits_are_enforced() {
        let mut config =
            XebConfig::default();

        config.num_qubits = 100;
        config.max_qubits = 10;

        assert!(matches!(
            config.validate(),
            Err(
                XebError::QubitLimitExceeded { .. }
            )
        ));
    }

    #[test]
    fn exact_model_requires_complete_distribution() {
        let model =
            XebIdealModel {
                kind: IdealModelKind::Exact,
                source: "fixture".into(),
                algorithm_version: "1".into(),
                complete: false,
                covered_probability: 0.5,
            };

        assert!(matches!(
            model.validate(),
            Err(
                XebError::UnsupportedIdealModel { .. }
            )
        ));
    }

    #[test]
    fn result_exposes_circuit_values_for_bootstrap() {
        let config =
            XebConfig::new(
                2,
                1,
                100,
                42,
            )
            .unwrap();

        let model =
            XebIdealModel::exact(
                "fixture",
                "1",
            );

        let result =
            XebProtocol::new()
                .analyze(
                    &config,
                    model,
                    &ideal_two_qubit(),
                    &uniform_execution(100),
                )
                .unwrap();

        let values =
            result.circuit_xeb_values();

        assert_eq!(values.len(), 1);
        assert!(
            values[0].is_finite()
        );
    }
}