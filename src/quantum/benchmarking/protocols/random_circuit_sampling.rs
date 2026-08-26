//! Zamani Quantum Benchmarking — Random Circuit Sampling
//!
//! Production Random Circuit Sampling (RCS) benchmark protocol.
//!
//! # Purpose
//!
//! This module defines the protocol-level contract for Random Circuit Sampling
//! (RCS) without coupling the protocol to:
//!
//! - a particular random-circuit generation algorithm;
//! - a particular simulator;
//! - a particular QPU vendor;
//! - a hardware topology;
//! - a compiler;
//! - a router;
//! - a scheduler;
//! - an XEB implementation;
//! - a reporting format;
//! - a statistical framework;
//! - a network provider;
//! - an async runtime.
//!
//! RCS is deliberately separated from random-circuit generation and
//! cross-entropy benchmarking:
//!
//! ```text
//! generators::random_circuits
//!          │
//!          ▼
//!     BenchmarkCircuit
//!          │
//!          ▼
//! execution::executor
//!          │
//!          ▼
//!     raw observations
//!          │
//!          ▼
//! protocols::random_circuit_sampling
//!          │
//!          ├──────────► RCS sampling statistics
//!          │
//!          └──────────► protocols::xeb
//!                           │
//!                           ▼
//!                     ideal probabilities
//! ```
//!
//! # Important semantic distinction
//!
//! RCS answers:
//!
//! > Can the selected backend execute and sample the specified family of
//! > random quantum circuits, and what was actually sampled?
//!
//! XEB answers a different question:
//!
//! > How correlated are the observed samples with a separately supplied ideal
//! > output distribution?
//!
//! Therefore this module MUST NOT require ideal probabilities.
//!
//! In particular, the following is valid RCS:
//!
//! ```text
//! random circuit
//!       │
//!       ▼
//! hardware
//!       │
//!       ▼
//! bitstring samples
//!       │
//!       ▼
//! RCS result
//! ```
//!
//! And this is the separate XEB pipeline:
//!
//! ```text
//! random circuit
//!       ├──────────────► ideal simulator
//!       │                    │
//!       │                    ▼
//!       │             ideal probabilities
//!       │                    │
//!       ▼                    │
//! hardware                   │
//!       │                    │
//!       ▼                    ▼
//! samples ───────────────► XEB analysis
//! ```
//!
//! # Scientific scope
//!
//! RCS is particularly useful for deep/random circuit workloads where the
//! output distribution is difficult to reproduce classically at increasing
//! circuit volume. The benchmark is therefore intentionally capable of
//! executing circuits for which no classical reference distribution is
//! available.
//!
//! This module provides:
//!
//! - explicit benchmark identity;
//! - versioned protocol semantics;
//! - validated width/depth configuration;
//! - explicit circuit count;
//! - explicit shot count;
//! - deterministic seed metadata;
//! - deterministic case identity;
//! - bounded resource usage;
//! - per-circuit sample preservation;
//! - aggregate sample statistics;
//! - unique-output statistics;
//! - collision statistics;
//! - entropy estimators where statistically meaningful;
//! - distribution-shape diagnostics;
//! - exact/partial reference-independent RCS analysis;
//! - explicit distinction between RCS and XEB;
//! - explicit validation status;
//! - execution-independent analysis;
//! - support for partial execution;
//! - duplicate detection;
//! - deterministic ordering;
//! - no global state;
//! - no logging;
//! - no printing;
//! - no unsafe code.
//!
//! # Architectural boundary
//!
//! This file DOES:
//!
//! - define the RCS experiment configuration;
//! - define the RCS case identity;
//! - validate RCS observations;
//! - aggregate RCS observations;
//! - compute sampling-level statistics;
//! - preserve enough information for later XEB analysis;
//! - expose an adapter boundary to the universal benchmark result.
//!
//! This file DOES NOT:
//!
//! - generate random circuits;
//! - execute circuits;
//! - select hardware;
//! - perform routing;
//! - perform scheduling;
//! - perform compilation;
//! - calculate ideal probabilities;
//! - claim quantum advantage;
//! - claim quantum supremacy;
//! - calculate XEB;
//! - silently remove failed samples.
//!
//! # Integration
//!
//! Existing Zamani architecture:
//!
//! ```text
//! generators/random_circuits.rs
//!             │
//!             ▼
//! core/circuit.rs
//!             │
//!             ▼
//! execution/*
//!             │
//!             ▼
//! core/observation.rs
//!             │
//!             ▼
//! protocols/random_circuit_sampling.rs
//!             │
//!       ┌─────┴──────────────┐
//!       ▼                    ▼
//! protocols/xeb.rs       reporting/*
//!                            │
//!                            ▼
//!                     core/result.rs
//! ```
//!
//! The current random-circuit generator already owns generation of canonical
//! logical random circuits and explicitly does not perform execution or
//! statistical analysis. This protocol therefore consumes generated benchmark
//! circuits rather than duplicating circuit generation.
//!
//! The current execution contract likewise defines a backend-independent
//! execution boundary and preserves raw observations. This protocol consumes
//! those observations rather than calling hardware directly.
//!
//! # Observation adapter
//!
//! The canonical observation layer supports bitstring counts and probability
//! distributions. This file intentionally also provides a small protocol-local
//! `RcsSampleSet` adapter so that RCS analysis can be performed from:
//!
//! - `core::observation`;
//! - execution fixtures;
//! - simulator output;
//! - hardware output;
//! - persisted benchmark data;
//! - future external benchmark-result data.
//!
//! The protocol-local representation is NOT a replacement for
//! `core::observation`; it is an analysis boundary.
//!
//! # Resource safety
//!
//! No configuration is accepted without checking:
//!
//! - qubit count;
//! - circuit count;
//! - shots;
//! - maximum total shots;
//! - identifier length;
//! - metadata limits;
//! - aggregate sample limits;
//! - bitstring width;
//! - arithmetic overflow.
//!
//! This is important because RCS intentionally creates large numbers of
//! samples. A malformed benchmark configuration must not be able to trigger
//! unbounded allocation.
//!
//! # Statistical interpretation
//!
//! RCS-native statistics include:
//!
//! - total samples;
//! - unique output count;
//! - collision count;
//! - collision probability;
//! - unique-output fraction;
//! - empirical Shannon entropy;
//! - min/max observed multiplicity;
//! - mean observed multiplicity;
//! - optional Porter-Thomas diagnostic statistics.
//!
//! These statistics are descriptive unless an explicit reference distribution
//! or hypothesis test is supplied.
//!
//! In particular:
//!
//! ```text
//! RCS sample diversity != fidelity
//! RCS entropy != XEB
//! RCS collisions != proof of quantum advantage
//! ```
//!
//! Any claim requiring an ideal reference distribution belongs to XEB or a
//! separate verification protocol.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features.
//! No unsafe code.
//! No additional dependency is required by this file.
//!
//! # Integration stability
//!
//! This file intentionally does not require later modifications merely to
//! connect:
//!
//! - XEB;
//! - reporting;
//! - registry;
//! - baseline analysis;
//! - regression analysis;
//! - Zamani-language bindings.
//!
//! Those integrations consume the public types and methods defined here.

#![deny(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;

// =============================================================================
// Public protocol identity
// =============================================================================

/// Stable benchmark identifier.
pub const RCS_BENCHMARK_ID: &str = "random_circuit_sampling";

/// Semantic protocol version.
///
/// This changes when RCS semantics, result interpretation, or required
/// invariants change.
pub const RCS_PROTOCOL_VERSION: &str = "1.0.0";

/// Stable identifier for the protocol's sampling-result schema.
pub const RCS_RESULT_SCHEMA_VERSION: u32 = 1;

/// Stable identifier describing the separation between RCS and XEB.
pub const RCS_ANALYSIS_CONVENTION: &str =
    "samples-first-reference-independent-v1";

/// Default number of random circuits.
pub const DEFAULT_CIRCUIT_COUNT: usize = 100;

/// Default number of shots per circuit.
pub const DEFAULT_SHOTS: usize = 1_000;

/// Default minimum circuit width.
pub const DEFAULT_MIN_QUBITS: usize = 2;

/// Default maximum circuit width.
///
/// This is a protocol safety default, not a physical limit.
pub const DEFAULT_MAX_QUBITS: usize = 1_024;

/// Default minimum circuit depth.
pub const DEFAULT_MIN_DEPTH: usize = 1;

/// Default maximum circuit depth.
pub const DEFAULT_MAX_DEPTH: usize = 1_024;

/// Maximum accepted identifier length.
pub const MAX_IDENTIFIER_LENGTH: usize = 4_096;

/// Maximum metadata entries.
pub const MAX_METADATA_ENTRIES: usize = 16_384;

/// Maximum aggregate samples represented by one protocol analysis.
///
/// A caller can run multiple independent benchmark cases when more samples
/// are required.
pub const DEFAULT_MAX_TOTAL_SAMPLES: u64 = 100_000_000;

/// Maximum bitstring width accepted by this protocol.
pub const MAX_BITSTRING_BITS: usize = 65_536;

/// Minimum number of samples required before entropy is reported.
pub const MIN_ENTROPY_SAMPLES: u64 = 2;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the RCS protocol.
#[derive(Debug, Clone, PartialEq)]
pub enum RcsError {
    /// A configuration field is invalid.
    InvalidConfiguration {
        field: &'static str,
        reason: String,
    },

    /// A configured resource ceiling was exceeded.
    ResourceLimitExceeded {
        resource: &'static str,
        requested: u64,
        maximum: u64,
    },

    /// Arithmetic overflow occurred.
    ArithmeticOverflow {
        operation: &'static str,
    },

    /// An identifier is invalid.
    InvalidIdentifier {
        field: &'static str,
        reason: &'static str,
    },

    /// A bitstring is invalid.
    InvalidBitstring {
        reason: String,
    },

    /// A probability is invalid.
    InvalidProbability {
        field: &'static str,
        value: f64,
    },

    /// A count is invalid.
    InvalidCount {
        field: &'static str,
        value: u64,
    },

    /// Counts do not match the declared shot count.
    ShotCountMismatch {
        expected: u64,
        observed: u64,
    },

    /// No samples were supplied.
    EmptySamples,

    /// A circuit case has no samples.
    EmptyCircuitSamples {
        case_id: String,
    },

    /// A duplicate circuit identity was supplied.
    DuplicateCircuit {
        case_id: String,
    },

    /// A case is missing from an aggregate result.
    MissingCircuit {
        case_id: String,
    },

    /// The supplied circuit width is inconsistent with the benchmark.
    WidthMismatch {
        expected: usize,
        actual: usize,
    },

    /// The supplied depth is inconsistent with the benchmark.
    DepthMismatch {
        expected: usize,
        actual: usize,
    },

    /// The supplied sample width is inconsistent with the circuit width.
    SampleWidthMismatch {
        expected: usize,
        actual: usize,
    },

    /// Statistical estimation is not valid for the available sample size.
    InsufficientSamples {
        statistic: &'static str,
        samples: u64,
        minimum: u64,
    },

    /// A numerical statistic became non-finite.
    NonFiniteStatistic {
        statistic: &'static str,
    },

    /// A required value was not available.
    Unavailable {
        quantity: &'static str,
        reason: String,
    },
}

impl fmt::Display for RcsError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidConfiguration {
                field,
                reason,
            } => write!(
                f,
                "invalid RCS configuration `{field}`: {reason}"
            ),

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => write!(
                f,
                "RCS resource `{resource}` exceeded: requested {requested}, \
                 maximum {maximum}"
            ),

            Self::ArithmeticOverflow { operation } => write!(
                f,
                "RCS arithmetic overflow while calculating {operation}"
            ),

            Self::InvalidIdentifier {
                field,
                reason,
            } => write!(
                f,
                "invalid RCS identifier `{field}`: {reason}"
            ),

            Self::InvalidBitstring { reason } => write!(
                f,
                "invalid RCS bitstring: {reason}"
            ),

            Self::InvalidProbability {
                field,
                value,
            } => write!(
                f,
                "invalid probability `{field}`: {value}"
            ),

            Self::InvalidCount {
                field,
                value,
            } => write!(
                f,
                "invalid count `{field}`: {value}"
            ),

            Self::ShotCountMismatch {
                expected,
                observed,
            } => write!(
                f,
                "RCS shot-count mismatch: expected {expected}, observed \
                 {observed}"
            ),

            Self::EmptySamples => {
                write!(f, "RCS received no samples")
            }

            Self::EmptyCircuitSamples { case_id } => write!(
                f,
                "RCS circuit case `{case_id}` contains no samples"
            ),

            Self::DuplicateCircuit { case_id } => write!(
                f,
                "duplicate RCS circuit case `{case_id}`"
            ),

            Self::MissingCircuit { case_id } => write!(
                f,
                "RCS circuit case `{case_id}` is missing"
            ),

            Self::WidthMismatch {
                expected,
                actual,
            } => write!(
                f,
                "RCS circuit width mismatch: expected {expected}, \
                 got {actual}"
            ),

            Self::DepthMismatch {
                expected,
                actual,
            } => write!(
                f,
                "RCS circuit depth mismatch: expected {expected}, \
                 got {actual}"
            ),

            Self::SampleWidthMismatch {
                expected,
                actual,
            } => write!(
                f,
                "RCS sample width mismatch: expected {expected}, got {actual}"
            ),

            Self::InsufficientSamples {
                statistic,
                samples,
                minimum,
            } => write!(
                f,
                "insufficient RCS samples for `{statistic}`: \
                 {samples}, minimum {minimum}"
            ),

            Self::NonFiniteStatistic { statistic } => write!(
                f,
                "RCS statistic `{statistic}` is non-finite"
            ),

            Self::Unavailable {
                quantity,
                reason,
            } => write!(
                f,
                "RCS quantity `{quantity}` is unavailable: {reason}"
            ),
        }
    }
}

impl std::error::Error for RcsError {}

// =============================================================================
// Configuration
// =============================================================================

/// Production RCS benchmark configuration.
///
/// Circuit generation is intentionally not configured here beyond the
/// dimensions required by the protocol. The actual random-circuit recipe
/// belongs to `generators::random_circuits`.
#[derive(Debug, Clone, PartialEq)]
pub struct RcsConfig {
    /// Minimum circuit width.
    pub min_qubits: usize,

    /// Maximum circuit width.
    pub max_qubits: usize,

    /// Minimum circuit depth.
    pub min_depth: usize,

    /// Maximum circuit depth.
    pub max_depth: usize,

    /// Number of independently generated random circuits per point.
    pub circuit_count: usize,

    /// Shots per circuit.
    pub shots: usize,

    /// Maximum total samples allowed by this experiment.
    pub max_total_samples: u64,

    /// Optional deterministic benchmark seed.
    pub seed: Option<u64>,

    /// Stable benchmark identifier supplied by the caller.
    pub benchmark_id: String,

    /// Stable experiment identifier.
    pub experiment_id: String,

    /// Arbitrary immutable benchmark metadata.
    pub metadata: BTreeMap<String, String>,
}

impl Default for RcsConfig {
    fn default() -> Self {
        Self {
            min_qubits: DEFAULT_MIN_QUBITS,
            max_qubits: DEFAULT_MAX_QUBITS,
            min_depth: DEFAULT_MIN_DEPTH,
            max_depth: DEFAULT_MAX_DEPTH,
            circuit_count: DEFAULT_CIRCUIT_COUNT,
            shots: DEFAULT_SHOTS,
            max_total_samples: DEFAULT_MAX_TOTAL_SAMPLES,
            seed: None,
            benchmark_id: RCS_BENCHMARK_ID.to_owned(),
            experiment_id: "rcs-experiment".to_owned(),
            metadata: BTreeMap::new(),
        }
    }
}

impl RcsConfig {
    /// Creates the default configuration and validates it.
    pub fn new() -> Result<Self, RcsError> {
        let config = Self::default();
        config.validate()?;
        Ok(config)
    }

    /// Validates all configuration invariants.
    pub fn validate(&self) -> Result<(), RcsError> {
        validate_identifier(
            "benchmark_id",
            &self.benchmark_id,
        )?;

        validate_identifier(
            "experiment_id",
            &self.experiment_id,
        )?;

        if self.min_qubits == 0 {
            return Err(RcsError::InvalidConfiguration {
                field: "min_qubits",
                reason: "must be greater than zero".to_owned(),
            });
        }

        if self.max_qubits < self.min_qubits {
            return Err(RcsError::InvalidConfiguration {
                field: "max_qubits",
                reason: "must be greater than or equal to min_qubits"
                    .to_owned(),
            });
        }

        if self.max_qubits > DEFAULT_MAX_QUBITS {
            return Err(RcsError::ResourceLimitExceeded {
                resource: "qubits",
                requested: self.max_qubits as u64,
                maximum: DEFAULT_MAX_QUBITS as u64,
            });
        }

        if self.min_depth == 0 {
            return Err(RcsError::InvalidConfiguration {
                field: "min_depth",
                reason: "must be greater than zero".to_owned(),
            });
        }

        if self.max_depth < self.min_depth {
            return Err(RcsError::InvalidConfiguration {
                field: "max_depth",
                reason: "must be greater than or equal to min_depth"
                    .to_owned(),
            });
        }

        if self.max_depth > DEFAULT_MAX_DEPTH {
            return Err(RcsError::ResourceLimitExceeded {
                resource: "depth",
                requested: self.max_depth as u64,
                maximum: DEFAULT_MAX_DEPTH as u64,
            });
        }

        if self.circuit_count == 0 {
            return Err(RcsError::InvalidConfiguration {
                field: "circuit_count",
                reason: "must be greater than zero".to_owned(),
            });
        }

        if self.shots == 0 {
            return Err(RcsError::InvalidConfiguration {
                field: "shots",
                reason: "must be greater than zero".to_owned(),
            });
        }

        let total_samples = (self.circuit_count as u64)
            .checked_mul(self.shots as u64)
            .ok_or(RcsError::ArithmeticOverflow {
                operation: "circuit_count * shots",
            })?;

        if total_samples > self.max_total_samples {
            return Err(RcsError::ResourceLimitExceeded {
                resource: "total_samples",
                requested: total_samples,
                maximum: self.max_total_samples,
            });
        }

        if self.max_total_samples == 0 {
            return Err(RcsError::InvalidConfiguration {
                field: "max_total_samples",
                reason: "must be greater than zero".to_owned(),
            });
        }

        if self.metadata.len() > MAX_METADATA_ENTRIES {
            return Err(RcsError::ResourceLimitExceeded {
                resource: "metadata_entries",
                requested: self.metadata.len() as u64,
                maximum: MAX_METADATA_ENTRIES as u64,
            });
        }

        for (key, value) in &self.metadata {
            validate_identifier("metadata_key", key)?;

            if value.len() > MAX_IDENTIFIER_LENGTH {
                return Err(RcsError::ResourceLimitExceeded {
                    resource: "metadata_value_bytes",
                    requested: value.len() as u64,
                    maximum: MAX_IDENTIFIER_LENGTH as u64,
                });
            }
        }

        Ok(())
    }

    /// Sets the deterministic experiment seed.
    #[must_use]
    pub fn with_seed(
        mut self,
        seed: u64,
    ) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Sets the width range.
    pub fn with_qubits(
        mut self,
        min_qubits: usize,
        max_qubits: usize,
    ) -> Result<Self, RcsError> {
        self.min_qubits = min_qubits;
        self.max_qubits = max_qubits;
        self.validate()?;
        Ok(self)
    }

    /// Sets the depth range.
    pub fn with_depth(
        mut self,
        min_depth: usize,
        max_depth: usize,
    ) -> Result<Self, RcsError> {
        self.min_depth = min_depth;
        self.max_depth = max_depth;
        self.validate()?;
        Ok(self)
    }

    /// Sets the circuit count.
    pub fn with_circuit_count(
        mut self,
        circuit_count: usize,
    ) -> Result<Self, RcsError> {
        self.circuit_count = circuit_count;
        self.validate()?;
        Ok(self)
    }

    /// Sets shots per circuit.
    pub fn with_shots(
        mut self,
        shots: usize,
    ) -> Result<Self, RcsError> {
        self.shots = shots;
        self.validate()?;
        Ok(self)
    }

    /// Sets a stable experiment identifier.
    pub fn with_experiment_id(
        mut self,
        experiment_id: impl Into<String>,
    ) -> Result<Self, RcsError> {
        self.experiment_id = experiment_id.into();
        self.validate()?;
        Ok(self)
    }

    /// Adds immutable benchmark metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, RcsError> {
        let key = key.into();
        let value = value.into();

        validate_identifier("metadata_key", &key)?;

        if value.len() > MAX_IDENTIFIER_LENGTH {
            return Err(RcsError::ResourceLimitExceeded {
                resource: "metadata_value_bytes",
                requested: value.len() as u64,
                maximum: MAX_IDENTIFIER_LENGTH as u64,
            });
        }

        self.metadata.insert(key, value);
        self.validate()?;
        Ok(self)
    }

    /// Returns the number of width/depth points represented by this
    /// rectangular configuration.
    #[must_use]
    pub fn grid_point_count(&self) -> usize {
        let widths = self.max_qubits - self.min_qubits + 1;
        let depths = self.max_depth - self.min_depth + 1;

        widths.saturating_mul(depths)
    }

    /// Returns the maximum number of samples represented by one experiment.
    pub fn total_sample_budget(&self) -> Result<u64, RcsError> {
        (self.circuit_count as u64)
            .checked_mul(self.shots as u64)
            .ok_or(RcsError::ArithmeticOverflow {
                operation: "circuit_count * shots",
            })
    }
}

// =============================================================================
// Circuit identity
// =============================================================================

/// Stable identity for one RCS circuit case.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RcsCircuitId(String);

impl RcsCircuitId {
    /// Creates a validated circuit identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, RcsError> {
        let value = value.into();

        validate_identifier("circuit_id", &value)?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RcsCircuitId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// =============================================================================
// Sample representation
// =============================================================================

/// One computational-basis sample.
///
/// The string is represented as a compact binary string such as `"01011"`.
///
/// RCS intentionally uses an explicit bitstring representation at this
/// protocol boundary because it is backend-independent and can be converted
/// to/from `core::observation` counts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RcsBitstring(String);

impl RcsBitstring {
    /// Creates a validated bitstring.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, RcsError> {
        let value = value.into();

        if value.is_empty() {
            return Err(RcsError::InvalidBitstring {
                reason: "bitstring cannot be empty".to_owned(),
            });
        }

        if value.len() > MAX_BITSTRING_BITS {
            return Err(RcsError::ResourceLimitExceeded {
                resource: "bitstring_bits",
                requested: value.len() as u64,
                maximum: MAX_BITSTRING_BITS as u64,
            });
        }

        if !value
            .bytes()
            .all(|byte| byte == b'0' || byte == b'1')
        {
            return Err(RcsError::InvalidBitstring {
                reason: "bitstrings may contain only 0 and 1".to_owned(),
            });
        }

        Ok(Self(value))
    }

    /// Returns the number of measured qubits.
    #[must_use]
    pub fn width(&self) -> usize {
        self.0.len()
    }

    /// Returns the bitstring.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RcsBitstring {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A single sampled output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RcsSample {
    /// Circuit that produced the sample.
    pub circuit_id: RcsCircuitId,

    /// Output bitstring.
    pub bitstring: RcsBitstring,

    /// Zero-based shot number when known.
    ///
    /// `None` is allowed for aggregated count data.
    pub shot_index: Option<u64>,
}

impl RcsSample {
    /// Creates one sample.
    pub fn new(
        circuit_id: RcsCircuitId,
        bitstring: RcsBitstring,
    ) -> Self {
        Self {
            circuit_id,
            bitstring,
            shot_index: None,
        }
    }

    /// Sets the shot index.
    #[must_use]
    pub fn with_shot_index(
        mut self,
        shot_index: u64,
    ) -> Self {
        self.shot_index = Some(shot_index);
        self
    }
}

// =============================================================================
// Count representation
// =============================================================================

/// A validated count for one bitstring.
///
/// This is the preferred adapter when `core::observation` already contains
/// counts instead of materialized individual samples.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RcsCount {
    /// Output state.
    pub bitstring: RcsBitstring,

    /// Number of occurrences.
    pub count: u64,
}

impl RcsCount {
    /// Creates a count.
    pub fn new(
        bitstring: RcsBitstring,
        count: u64,
    ) -> Result<Self, RcsError> {
        if count == 0 {
            return Err(RcsError::InvalidCount {
                field: "count",
                value: count,
            });
        }

        Ok(Self {
            bitstring,
            count,
        })
    }
}

// =============================================================================
// Per-circuit observation
// =============================================================================

/// Raw RCS observation for one circuit.
///
/// This is intentionally close to what execution adapters need to provide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RcsCircuitObservation {
    /// Stable circuit identity.
    pub circuit_id: RcsCircuitId,

    /// Logical circuit width.
    pub qubits: usize,

    /// Logical circuit depth.
    pub depth: usize,

    /// Declared shots.
    pub shots: u64,

    /// Observed counts.
    pub counts: Vec<RcsCount>,

    /// Whether this observation is known to be complete.
    pub complete: bool,
}

impl RcsCircuitObservation {
    /// Creates an observation from aggregated counts.
    pub fn new(
        circuit_id: RcsCircuitId,
        qubits: usize,
        depth: usize,
        shots: u64,
        counts: Vec<RcsCount>,
        complete: bool,
    ) -> Result<Self, RcsError> {
        if qubits == 0 {
            return Err(RcsError::InvalidConfiguration {
                field: "qubits",
                reason: "must be greater than zero".to_owned(),
            });
        }

        if depth == 0 {
            return Err(RcsError::InvalidConfiguration {
                field: "depth",
                reason: "must be greater than zero".to_owned(),
            });
        }

        if shots == 0 {
            return Err(RcsError::InvalidCount {
                field: "shots",
                value: shots,
            });
        }

        if counts.is_empty() {
            return Err(RcsError::EmptyCircuitSamples {
                case_id: circuit_id.as_str().to_owned(),
            });
        }

        let mut total = 0u64;

        for item in &counts {
            if item.bitstring.width() != qubits {
                return Err(RcsError::SampleWidthMismatch {
                    expected: qubits,
                    actual: item.bitstring.width(),
                });
            }

            total = total
                .checked_add(item.count)
                .ok_or(RcsError::ArithmeticOverflow {
                    operation: "sum of RCS counts",
                })?;
        }

        if total > shots {
            return Err(RcsError::ShotCountMismatch {
                expected: shots,
                observed: total,
            });
        }

        if complete && total != shots {
            return Err(RcsError::ShotCountMismatch {
                expected: shots,
                observed: total,
            });
        }

        Ok(Self {
            circuit_id,
            qubits,
            depth,
            shots,
            counts,
            complete,
        })
    }

    /// Returns the number of observed samples.
    #[must_use]
    pub fn observed_samples(&self) -> u64 {
        self.counts
            .iter()
            .map(|item| item.count)
            .sum()
    }

    /// Returns the number of unique observed outputs.
    #[must_use]
    pub fn unique_outputs(&self) -> usize {
        self.counts.len()
    }

    /// Returns whether all requested shots were observed.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.complete && self.observed_samples() == self.shots
    }

    /// Returns the collision count.
    ///
    /// For multiplicities `c_i`, this is:
    ///
    /// `sum_i c_i (c_i - 1) / 2`
    pub fn collision_count(&self) -> Result<u64, RcsError> {
        let mut total = 0u64;

        for item in &self.counts {
            let contribution = item
                .count
                .checked_mul(item.count.saturating_sub(1))
                .ok_or(RcsError::ArithmeticOverflow {
                    operation: "RCS collision count",
                })?
                / 2;

            total = total
                .checked_add(contribution)
                .ok_or(RcsError::ArithmeticOverflow {
                    operation: "RCS collision count accumulation",
                })?;
        }

        Ok(total)
    }

    /// Returns the empirical collision probability.
    ///
    /// This uses unordered pairs:
    ///
    /// `sum_i C(c_i, 2) / C(N, 2)`
    pub fn collision_probability(&self) -> Result<f64, RcsError> {
        let n = self.observed_samples();

        if n < 2 {
            return Err(RcsError::InsufficientSamples {
                statistic: "collision_probability",
                samples: n,
                minimum: 2,
            });
        }

        let numerator = self.collision_count()? as f64;
        let denominator = (n as f64) * ((n - 1) as f64) / 2.0;

        let probability = numerator / denominator;

        validate_finite(
            "collision_probability",
            probability,
        )?;

        Ok(probability)
    }

    /// Returns the unique-output fraction.
    pub fn unique_output_fraction(&self) -> Result<f64, RcsError> {
        let n = self.observed_samples();

        if n == 0 {
            return Err(RcsError::EmptySamples);
        }

        let fraction = self.unique_outputs() as f64 / n as f64;

        validate_probability(
            "unique_output_fraction",
            fraction,
        )?;

        Ok(fraction)
    }

    /// Returns empirical Shannon entropy in bits.
    ///
    /// This is the entropy of the observed empirical distribution, not the
    /// entropy of the ideal random-circuit output distribution.
    pub fn empirical_entropy_bits(&self) -> Result<f64, RcsError> {
        let n = self.observed_samples();

        if n < MIN_ENTROPY_SAMPLES {
            return Err(RcsError::InsufficientSamples {
                statistic: "empirical_entropy_bits",
                samples: n,
                minimum: MIN_ENTROPY_SAMPLES,
            });
        }

        let denominator = n as f64;
        let mut entropy = 0.0;

        for item in &self.counts {
            let p = item.count as f64 / denominator;

            if p <= 0.0 {
                continue;
            }

            entropy -= p * p.log2();
        }

        validate_finite(
            "empirical_entropy_bits",
            entropy,
        )?;

        Ok(entropy)
    }

    /// Returns the maximum observed multiplicity.
    #[must_use]
    pub fn maximum_multiplicity(&self) -> u64 {
        self.counts
            .iter()
            .map(|item| item.count)
            .max()
            .unwrap_or(0)
    }

    /// Returns the minimum non-zero observed multiplicity.
    #[must_use]
    pub fn minimum_multiplicity(&self) -> u64 {
        self.counts
            .iter()
            .map(|item| item.count)
            .min()
            .unwrap_or(0)
    }

    /// Returns the mean multiplicity among observed unique outputs.
    pub fn mean_multiplicity(&self) -> Result<f64, RcsError> {
        if self.counts.is_empty() {
            return Err(RcsError::EmptySamples);
        }

        let mean =
            self.observed_samples() as f64 / self.counts.len() as f64;

        validate_finite("mean_multiplicity", mean)?;

        Ok(mean)
    }
}

// =============================================================================
// Aggregate result
// =============================================================================

/// Aggregate descriptive statistics for one RCS benchmark point.
#[derive(Debug, Clone, PartialEq)]
pub struct RcsPointResult {
    /// Number of qubits.
    pub qubits: usize,

    /// Circuit depth.
    pub depth: usize,

    /// Number of circuits successfully observed.
    pub circuit_count: usize,

    /// Number of complete circuits.
    pub complete_circuit_count: usize,

    /// Number of incomplete circuits.
    pub incomplete_circuit_count: usize,

    /// Requested shots per circuit.
    pub requested_shots_per_circuit: u64,

    /// Total observed samples.
    pub observed_samples: u64,

    /// Total unique output strings summed per circuit.
    pub summed_unique_outputs: u64,

    /// Mean unique-output fraction across circuits.
    pub mean_unique_output_fraction: f64,

    /// Mean collision probability across circuits where estimable.
    pub mean_collision_probability: Option<f64>,

    /// Mean empirical entropy across circuits where estimable.
    pub mean_empirical_entropy_bits: Option<f64>,

    /// Mean multiplicity among observed outputs.
    pub mean_multiplicity: Option<f64>,

    /// Maximum multiplicity observed across all circuits.
    pub maximum_multiplicity: u64,

    /// Whether the point has complete execution coverage.
    pub complete: bool,
}

impl RcsPointResult {
    /// Returns whether this point has any usable sample data.
    #[must_use]
    pub fn has_samples(&self) -> bool {
        self.observed_samples > 0
    }

    /// Returns the fraction of circuits that completed.
    pub fn circuit_completion_fraction(&self) -> Result<f64, RcsError> {
        if self.circuit_count == 0 {
            return Err(RcsError::InvalidCount {
                field: "circuit_count",
                value: 0,
            });
        }

        let fraction =
            self.complete_circuit_count as f64 / self.circuit_count as f64;

        validate_probability(
            "circuit_completion_fraction",
            fraction,
        )?;

        Ok(fraction)
    }
}

/// Complete RCS benchmark result.
///
/// This result deliberately contains no XEB score.
#[derive(Debug, Clone, PartialEq)]
pub struct RcsResult {
    /// Result schema version.
    pub schema_version: u32,

    /// Benchmark identifier.
    pub benchmark_id: String,

    /// Protocol version.
    pub protocol_version: String,

    /// Experiment identifier.
    pub experiment_id: String,

    /// Optional deterministic benchmark seed.
    pub seed: Option<u64>,

    /// Results grouped by `(qubits, depth)`.
    pub points: Vec<RcsPointResult>,

    /// Total circuit cases received.
    pub circuit_count: usize,

    /// Total observed samples.
    pub observed_samples: u64,

    /// Number of complete circuit cases.
    pub complete_circuit_count: usize,

    /// Number of incomplete circuit cases.
    pub incomplete_circuit_count: usize,

    /// Human/machine-readable protocol assumptions.
    pub assumptions: Vec<String>,

    /// Structured warnings.
    pub warnings: Vec<String>,

    /// Immutable benchmark metadata.
    pub metadata: BTreeMap<String, String>,
}

impl RcsResult {
    /// Returns whether every supplied circuit completed.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.incomplete_circuit_count == 0
            && self.circuit_count > 0
    }

    /// Returns whether an XEB score can be derived from this result alone.
    ///
    /// It intentionally returns `false`: RCS does not contain ideal output
    /// probabilities.
    #[must_use]
    pub const fn has_xeb_score(&self) -> bool {
        false
    }

    /// Returns the protocol identity used by downstream registry/reporting
    /// layers.
    #[must_use]
    pub const fn protocol_id(&self) -> &'static str {
        RCS_BENCHMARK_ID
    }
}

// =============================================================================
// Protocol
// =============================================================================

/// Stateless RCS protocol implementation.
#[derive(Debug, Clone, Copy, Default)]
pub struct RandomCircuitSampling;

impl RandomCircuitSampling {
    /// Returns the stable protocol identifier.
    #[must_use]
    pub const fn benchmark_id() -> &'static str {
        RCS_BENCHMARK_ID
    }

    /// Returns the semantic protocol version.
    #[must_use]
    pub const fn protocol_version() -> &'static str {
        RCS_PROTOCOL_VERSION
    }

    /// Validates an RCS configuration.
    pub fn validate(
        config: &RcsConfig,
    ) -> Result<(), RcsError> {
        config.validate()
    }

    /// Analyzes RCS observations.
    ///
    /// This function performs no execution and no circuit generation.
    ///
    /// The supplied observations may be:
    ///
    /// - complete;
    /// - partially complete;
    /// - simulator-derived;
    /// - hardware-derived;
    /// - persisted/replayed.
    ///
    /// Missing cases are not silently invented.
    pub fn analyze(
        config: &RcsConfig,
        observations: &[RcsCircuitObservation],
    ) -> Result<RcsResult, RcsError> {
        config.validate()?;

        if observations.is_empty() {
            return Err(RcsError::EmptySamples);
        }

        let mut seen = BTreeMap::<String, ()>::new();

        for observation in observations {
            validate_observation(config, observation)?;

            if seen
                .insert(observation.circuit_id.as_str().to_owned(), ())
                .is_some()
            {
                return Err(RcsError::DuplicateCircuit {
                    case_id: observation.circuit_id.as_str().to_owned(),
                });
            }
        }

        let mut groups =
            BTreeMap::<(usize, usize), Vec<&RcsCircuitObservation>>::new();

        for observation in observations {
            groups
                .entry((observation.qubits, observation.depth))
                .or_default()
                .push(observation);
        }

        let mut points = Vec::with_capacity(groups.len());

        for ((qubits, depth), cases) in groups {
            points.push(analyze_point(
                qubits,
                depth,
                config.shots as u64,
                &cases,
            )?);
        }

        let circuit_count = observations.len();

        let observed_samples = observations
            .iter()
            .try_fold(0u64, |total, observation| {
                total
                    .checked_add(observation.observed_samples())
                    .ok_or(RcsError::ArithmeticOverflow {
                        operation: "aggregate RCS observed samples",
                    })
            })?;

        if observed_samples > config.max_total_samples {
            return Err(RcsError::ResourceLimitExceeded {
                resource: "aggregate_observed_samples",
                requested: observed_samples,
                maximum: config.max_total_samples,
            });
        }

        let complete_circuit_count = observations
            .iter()
            .filter(|observation| observation.is_complete())
            .count();

        let incomplete_circuit_count =
            circuit_count - complete_circuit_count;

        let mut warnings = Vec::new();

        if incomplete_circuit_count > 0 {
            warnings.push(
                "one or more RCS circuit cases are incomplete; aggregate \
                 statistics must be interpreted as partial execution"
                    .to_owned(),
            );
        }

        if config.seed.is_none() {
            warnings.push(
                "no benchmark seed was supplied; exact random-circuit \
                 replay cannot be guaranteed by this protocol"
                    .to_owned(),
            );
        }

        Ok(RcsResult {
            schema_version: RCS_RESULT_SCHEMA_VERSION,
            benchmark_id: config.benchmark_id.clone(),
            protocol_version: RCS_PROTOCOL_VERSION.to_owned(),
            experiment_id: config.experiment_id.clone(),
            seed: config.seed,
            points,
            circuit_count,
            observed_samples,
            complete_circuit_count,
            incomplete_circuit_count,
            assumptions: default_assumptions(),
            warnings,
            metadata: config.metadata.clone(),
        })
    }
}

// =============================================================================
// Point analysis
// =============================================================================

fn analyze_point(
    qubits: usize,
    depth: usize,
    requested_shots: u64,
    cases: &[&RcsCircuitObservation],
) -> Result<RcsPointResult, RcsError> {
    if cases.is_empty() {
        return Err(RcsError::EmptySamples);
    }

    let mut observed_samples = 0u64;
    let mut summed_unique_outputs = 0u64;
    let mut maximum_multiplicity = 0u64;

    let mut unique_fraction_sum = 0.0;
    let mut unique_fraction_count = 0usize;

    let mut collision_probability_sum = 0.0;
    let mut collision_probability_count = 0usize;

    let mut entropy_sum = 0.0;
    let mut entropy_count = 0usize;

    let mut multiplicity_sum = 0.0;
    let mut multiplicity_count = 0usize;

    let mut complete_circuit_count = 0usize;

    for observation in cases {
        let samples = observation.observed_samples();

        observed_samples = observed_samples
            .checked_add(samples)
            .ok_or(RcsError::ArithmeticOverflow {
                operation: "point observed samples",
            })?;

        summed_unique_outputs = summed_unique_outputs
            .checked_add(observation.unique_outputs() as u64)
            .ok_or(RcsError::ArithmeticOverflow {
                operation: "point unique outputs",
            })?;

        maximum_multiplicity =
            maximum_multiplicity.max(observation.maximum_multiplicity());

        if observation.is_complete() {
            complete_circuit_count += 1;
        }

        if let Ok(value) =
            observation.unique_output_fraction()
        {
            unique_fraction_sum += value;
            unique_fraction_count += 1;
        }

        if samples >= 2 {
            if let Ok(value) =
                observation.collision_probability()
            {
                collision_probability_sum += value;
                collision_probability_count += 1;
            }
        }

        if samples >= MIN_ENTROPY_SAMPLES {
            if let Ok(value) =
                observation.empirical_entropy_bits()
            {
                entropy_sum += value;
                entropy_count += 1;
            }
        }

        if !observation.counts.is_empty() {
            if let Ok(value) =
                observation.mean_multiplicity()
            {
                multiplicity_sum += value;
                multiplicity_count += 1;
            }
        }
    }

    if observed_samples == 0 {
        return Err(RcsError::EmptySamples);
    }

    let mean_unique_output_fraction =
        if unique_fraction_count == 0 {
            return Err(RcsError::Unavailable {
                quantity: "mean_unique_output_fraction",
                reason: "no usable circuit observations".to_owned(),
            });
        } else {
            unique_fraction_sum / unique_fraction_count as f64
        };

    validate_probability(
        "mean_unique_output_fraction",
        mean_unique_output_fraction,
    )?;

    let mean_collision_probability =
        if collision_probability_count == 0 {
            None
        } else {
            Some(
                collision_probability_sum
                    / collision_probability_count as f64,
            )
        };

    if let Some(value) = mean_collision_probability {
        validate_probability(
            "mean_collision_probability",
            value,
        )?;
    }

    let mean_empirical_entropy_bits =
        if entropy_count == 0 {
            None
        } else {
            Some(entropy_sum / entropy_count as f64)
        };

    if let Some(value) = mean_empirical_entropy_bits {
        validate_finite(
            "mean_empirical_entropy_bits",
            value,
        )?;
    }

    let mean_multiplicity =
        if multiplicity_count == 0 {
            None
        } else {
            Some(multiplicity_sum / multiplicity_count as f64)
        };

    if let Some(value) = mean_multiplicity {
        validate_finite("mean_multiplicity", value)?;
    }

    let circuit_count = cases.len();
    let incomplete_circuit_count =
        circuit_count - complete_circuit_count;

    Ok(RcsPointResult {
        qubits,
        depth,
        circuit_count,
        complete_circuit_count,
        incomplete_circuit_count,
        requested_shots_per_circuit: requested_shots,
        observed_samples,
        summed_unique_outputs,
        mean_unique_output_fraction,
        mean_collision_probability,
        mean_empirical_entropy_bits,
        mean_multiplicity,
        maximum_multiplicity,
        complete: incomplete_circuit_count == 0,
    })
}

// =============================================================================
// Observation validation
// =============================================================================

fn validate_observation(
    config: &RcsConfig,
    observation: &RcsCircuitObservation,
) -> Result<(), RcsError> {
    if observation.qubits < config.min_qubits
        || observation.qubits > config.max_qubits
    {
        return Err(RcsError::WidthMismatch {
            expected: config.max_qubits,
            actual: observation.qubits,
        });
    }

    if observation.depth < config.min_depth
        || observation.depth > config.max_depth
    {
        return Err(RcsError::DepthMismatch {
            expected: config.max_depth,
            actual: observation.depth,
        });
    }

    if observation.shots != config.shots as u64 {
        return Err(RcsError::ShotCountMismatch {
            expected: config.shots as u64,
            observed: observation.shots,
        });
    }

    if observation.counts.is_empty() {
        return Err(RcsError::EmptyCircuitSamples {
            case_id: observation.circuit_id.as_str().to_owned(),
        });
    }

    let mut total = 0u64;

    for count in &observation.counts {
        if count.bitstring.width() != observation.qubits {
            return Err(RcsError::SampleWidthMismatch {
                expected: observation.qubits,
                actual: count.bitstring.width(),
            });
        }

        if count.count == 0 {
            return Err(RcsError::InvalidCount {
                field: "sample_count",
                value: count.count,
            });
        }

        total = total
            .checked_add(count.count)
            .ok_or(RcsError::ArithmeticOverflow {
                operation: "observation count accumulation",
            })?;
    }

    if total > observation.shots {
        return Err(RcsError::ShotCountMismatch {
            expected: observation.shots,
            observed: total,
        });
    }

    if observation.complete
        && total != observation.shots
    {
        return Err(RcsError::ShotCountMismatch {
            expected: observation.shots,
            observed: total,
        });
    }

    Ok(())
}

// =============================================================================
// Conversion helpers
// =============================================================================

/// Converts individual RCS samples into aggregated counts.
///
/// This is useful for execution adapters which expose individual shots.
pub fn aggregate_samples(
    circuit_id: RcsCircuitId,
    qubits: usize,
    depth: usize,
    shots: u64,
    samples: &[RcsSample],
    complete: bool,
) -> Result<RcsCircuitObservation, RcsError> {
    if samples.is_empty() {
        return Err(RcsError::EmptySamples);
    }

    if samples.len() as u64 > shots {
        return Err(RcsError::ShotCountMismatch {
            expected: shots,
            observed: samples.len() as u64,
        });
    }

    let mut counts = BTreeMap::<RcsBitstring, u64>::new();

    for sample in samples {
        if sample.circuit_id != circuit_id {
            return Err(RcsError::InvalidConfiguration {
                field: "sample.circuit_id",
                reason: "sample belongs to another circuit case"
                    .to_owned(),
            });
        }

        if sample.bitstring.width() != qubits {
            return Err(RcsError::SampleWidthMismatch {
                expected: qubits,
                actual: sample.bitstring.width(),
            });
        }

        let entry = counts
            .entry(sample.bitstring.clone())
            .or_insert(0);

        *entry = entry
            .checked_add(1)
            .ok_or(RcsError::ArithmeticOverflow {
                operation: "sample count aggregation",
            })?;
    }

    let counts = counts
        .into_iter()
        .map(|(bitstring, count)| {
            RcsCount::new(bitstring, count)
        })
        .collect::<Result<Vec<_>, _>>()?;

    RcsCircuitObservation::new(
        circuit_id,
        qubits,
        depth,
        shots,
        counts,
        complete,
    )
}

/// Converts a normalized probability distribution into expected count data.
///
/// This function deliberately rejects the conversion because probabilities
/// alone are not raw RCS samples. Rounding probabilities to counts would
/// fabricate observations and could bias collision/entropy statistics.
///
/// Use actual execution counts instead.
pub fn probabilities_are_not_samples() -> Result<(), RcsError> {
    Err(RcsError::Unavailable {
        quantity: "RCS_samples",
        reason:
            "probabilities are not raw samples; supply execution counts \
             or individual bitstring samples"
                .to_owned(),
    })
}

// =============================================================================
// Statistical helpers
// =============================================================================

/// Returns the expected number of unique outputs under a uniform distribution
/// over `2^n` states.
///
/// This is a diagnostic reference only. It is NOT the expected unique-output
/// count for a general random quantum circuit because the ideal output
/// distribution is not necessarily uniform.
pub fn uniform_expected_unique_outputs(
    qubits: usize,
    shots: u64,
) -> Result<f64, RcsError> {
    if qubits == 0 {
        return Err(RcsError::InvalidConfiguration {
            field: "qubits",
            reason: "must be greater than zero".to_owned(),
        });
    }

    if qubits >= 64 {
        return Err(RcsError::Unavailable {
            quantity: "uniform_expected_unique_outputs",
            reason:
                "the exact 2^n state-space calculation is intentionally \
                 bounded below 64 qubits"
                    .to_owned(),
        });
    }

    if shots == 0 {
        return Err(RcsError::InvalidCount {
            field: "shots",
            value: shots,
        });
    }

    let states = 2f64.powi(qubits as i32);
    let expected =
        states * (1.0 - (1.0 - 1.0 / states).powf(shots as f64));

    validate_finite(
        "uniform_expected_unique_outputs",
        expected,
    )?;

    Ok(expected)
}

/// Returns the expected collision probability under a uniform distribution
/// over `2^n` states.
///
/// This is a reference diagnostic, not an RCS fidelity measure.
pub fn uniform_collision_probability(
    qubits: usize,
) -> Result<f64, RcsError> {
    if qubits == 0 {
        return Err(RcsError::InvalidConfiguration {
            field: "qubits",
            reason: "must be greater than zero".to_owned(),
        });
    }

    if qubits >= 64 {
        return Err(RcsError::Unavailable {
            quantity: "uniform_collision_probability",
            reason:
                "exact 2^n state-space calculation is intentionally \
                 bounded below 64 qubits"
                    .to_owned(),
        });
    }

    let states = 2f64.powi(qubits as i32);
    let probability = 1.0 / states;

    validate_probability(
        "uniform_collision_probability",
        probability,
    )?;

    Ok(probability)
}

/// Returns the Porter-Thomas exponential-distribution mean in natural units.
///
/// For a Hilbert space dimension `D`, ideal chaotic-circuit probabilities are
/// often analyzed through the scaled variable `D * p`.
///
/// The expected value of that scaled variable under the exponential model is
/// 1.0.
///
/// This helper exists only as a diagnostic convention and does not establish
/// that an arbitrary circuit is Porter-Thomas distributed.
#[must_use]
pub const fn porter_thomas_scaled_mean() -> f64 {
    1.0
}

// =============================================================================
// Protocol assumptions
// =============================================================================

fn default_assumptions() -> Vec<String> {
    vec![
        "RCS measures execution and sampling of a specified random-circuit \
         workload; it is not itself an XEB calculation."
            .to_owned(),
        "Circuit generation is owned by generators::random_circuits and is \
         not reimplemented by this protocol."
            .to_owned(),
        "Bitstring samples/counts are treated as empirical execution \
         observations."
            .to_owned(),
        "Incomplete execution is represented explicitly rather than silently \
         imputed."
            .to_owned(),
        "Descriptive entropy and collision statistics do not by themselves \
         establish fidelity, quantum advantage, or computational supremacy."
            .to_owned(),
        "Uniform-distribution diagnostics are references only and must not be \
         interpreted as the ideal output distribution of every random circuit."
            .to_owned(),
        "Any statistic requiring ideal output probabilities belongs to an \
         XEB/reference-analysis layer."
            .to_owned(),
        "Exact reproducibility requires the same circuit generator, generator \
         revision, RNG algorithm, seed, Quantum IR semantics, and benchmark \
         configuration."
            .to_owned(),
    ]
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
) -> Result<(), RcsError> {
    if value.trim().is_empty() {
        return Err(RcsError::InvalidIdentifier {
            field,
            reason: "identifier cannot be empty",
        });
    }

    if value.len() > MAX_IDENTIFIER_LENGTH {
        return Err(RcsError::InvalidIdentifier {
            field,
            reason: "identifier exceeds the maximum permitted length",
        });
    }

    if !value
        .bytes()
        .all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(
                    byte,
                    b'-' | b'_' | b'.' | b':' | b'/'
                )
        })
    {
        return Err(RcsError::InvalidIdentifier {
            field,
            reason:
                "identifier may contain only ASCII letters, digits, \
                 '-', '_', '.', ':' and '/'",
        });
    }

    Ok(())
}

fn validate_probability(
    field: &'static str,
    value: f64,
) -> Result<(), RcsError> {
    if !value.is_finite() {
        return Err(RcsError::NonFiniteStatistic {
            statistic: field,
        });
    }

    if !(0.0..=1.0).contains(&value) {
        return Err(RcsError::InvalidProbability {
            field,
            value,
        });
    }

    Ok(())
}

fn validate_finite(
    statistic: &'static str,
    value: f64,
) -> Result<(), RcsError> {
    if !value.is_finite() {
        return Err(RcsError::NonFiniteStatistic {
            statistic,
        });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn bit(value: &str) -> RcsBitstring {
        RcsBitstring::new(value).unwrap()
    }

    fn circuit(
        id: &str,
        counts: &[(&str, u64)],
        shots: u64,
    ) -> RcsCircuitObservation {
        let id = RcsCircuitId::new(id).unwrap();

        let values = counts
            .iter()
            .map(|(value, count)| {
                RcsCount::new(bit(value), *count).unwrap()
            })
            .collect::<Vec<_>>();

        RcsCircuitObservation::new(
            id,
            counts[0].0.len(),
            10,
            shots,
            values,
            true,
        )
        .unwrap()
    }

    #[test]
    fn default_configuration_is_valid() {
        let config = RcsConfig::new().unwrap();

        assert_eq!(
            config.benchmark_id,
            RCS_BENCHMARK_ID
        );
        assert_eq!(
            config.circuit_count,
            DEFAULT_CIRCUIT_COUNT
        );
        assert_eq!(
            config.shots,
            DEFAULT_SHOTS
        );
    }

    #[test]
    fn invalid_zero_shots_are_rejected() {
        let result =
            RcsConfig::default().with_shots(0);

        assert!(result.is_err());
    }

    #[test]
    fn invalid_bitstring_is_rejected() {
        let result = RcsBitstring::new("012");

        assert!(result.is_err());
    }

    #[test]
    fn collision_probability_is_correct_for_two_identical_samples() {
        let observation = circuit(
            "c0",
            &[("00", 2)],
            2,
        );

        let probability =
            observation.collision_probability().unwrap();

        assert!((probability - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn collision_probability_is_zero_for_distinct_samples() {
        let observation = circuit(
            "c0",
            &[("00", 1), ("01", 1)],
            2,
        );

        let probability =
            observation.collision_probability().unwrap();

        assert!(probability.abs() < 1.0e-12);
    }

    #[test]
    fn entropy_of_two_uniform_outputs_is_one_bit() {
        let observation = circuit(
            "c0",
            &[("00", 1), ("01", 1)],
            2,
        );

        let entropy =
            observation.empirical_entropy_bits().unwrap();

        assert!((entropy - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn incomplete_counts_are_allowed_when_marked_incomplete() {
        let id =
            RcsCircuitId::new("c0").unwrap();

        let observation =
            RcsCircuitObservation::new(
                id,
                2,
                10,
                10,
                vec![
                    RcsCount::new(bit("00"), 3).unwrap(),
                ],
                false,
            );

        assert!(observation.is_ok());

        let observation =
            observation.unwrap();

        assert!(!observation.is_complete());
        assert_eq!(
            observation.observed_samples(),
            3
        );
    }

    #[test]
    fn complete_counts_must_match_shots() {
        let id =
            RcsCircuitId::new("c0").unwrap();

        let result =
            RcsCircuitObservation::new(
                id,
                2,
                10,
                10,
                vec![
                    RcsCount::new(bit("00"), 3).unwrap(),
                ],
                true,
            );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_circuit_ids_are_rejected() {
        let config = RcsConfig::default()
            .with_qubits(2, 2)
            .unwrap()
            .with_depth(10, 10)
            .unwrap()
            .with_circuit_count(2)
            .unwrap()
            .with_shots(2)
            .unwrap();

        let a = circuit(
            "same",
            &[("00", 1), ("01", 1)],
            2,
        );

        let b = circuit(
            "same",
            &[("00", 1), ("01", 1)],
            2,
        );

        let result =
            RandomCircuitSampling::analyze(
                &config,
                &[a, b],
            );

        assert!(matches!(
            result,
            Err(RcsError::DuplicateCircuit { .. })
        ));
    }

    #[test]
    fn aggregate_samples_produces_counts() {
        let id =
            RcsCircuitId::new("c0").unwrap();

        let samples = vec![
            RcsSample::new(
                id.clone(),
                bit("00"),
            ),
            RcsSample::new(
                id.clone(),
                bit("01"),
            ),
            RcsSample::new(
                id.clone(),
                bit("00"),
            ),
        ];

        let observation =
            aggregate_samples(
                id,
                2,
                10,
                3,
                &samples,
                true,
            )
            .unwrap();

        assert_eq!(
            observation.unique_outputs(),
            2
        );

        assert_eq!(
            observation.observed_samples(),
            3
        );
    }

    #[test]
    fn uniform_reference_for_two_qubits_is_valid() {
        let probability =
            uniform_collision_probability(2)
                .unwrap();

        assert!((probability - 0.25).abs() < 1.0e-12);
    }

    #[test]
    fn uniform_unique_output_reference_is_reasonable() {
        let expected =
            uniform_expected_unique_outputs(2, 4)
                .unwrap();

        assert!(expected > 0.0);
        assert!(expected <= 4.0);
    }

    #[test]
    fn result_never_contains_xeb() {
        let config = RcsConfig::default()
            .with_qubits(2, 2)
            .unwrap()
            .with_depth(10, 10)
            .unwrap()
            .with_circuit_count(1)
            .unwrap()
            .with_shots(2)
            .unwrap();

        let observation = circuit(
            "c0",
            &[("00", 1), ("01", 1)],
            2,
        );

        let result =
            RandomCircuitSampling::analyze(
                &config,
                &[observation],
            )
            .unwrap();

        assert!(!result.has_xeb_score());
    }

    #[test]
    fn deterministic_ordering_is_preserved() {
        let id =
            RcsCircuitId::new("c0").unwrap();

        let samples = vec![
            RcsSample::new(
                id.clone(),
                bit("11"),
            ),
            RcsSample::new(
                id.clone(),
                bit("00"),
            ),
            RcsSample::new(
                id.clone(),
                bit("11"),
            ),
            RcsSample::new(
                id.clone(),
                bit("01"),
            ),
        ];

        let observation =
            aggregate_samples(
                id,
                2,
                10,
                4,
                &samples,
                true,
            )
            .unwrap();

        assert_eq!(
            observation.counts[0].bitstring.as_str(),
            "00"
        );

        assert_eq!(
            observation.counts[1].bitstring.as_str(),
            "01"
        );

        assert_eq!(
            observation.counts[2].bitstring.as_str(),
            "11"
        );
    }
}