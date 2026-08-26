//! Zamani Quantum Benchmarking — Hardware Calibration Evidence
//!
//! Production-grade, backend-independent representation of calibration data
//! consumed by the quantum benchmarking subsystem.
//!
//! # Architectural responsibility
//!
//! This file represents calibration STATE/EVIDENCE.
//!
//! It does NOT:
//!
//! - perform hardware calibration;
//! - communicate with a device;
//! - communicate over a network;
//! - invoke a simulator;
//! - execute quantum circuits;
//! - mutate Quantum IR;
//! - perform routing;
//! - perform scheduling;
//! - decide whether a benchmark passes;
//! - estimate benchmark statistics;
//! - silently substitute missing calibration values;
//! - read environment variables;
//! - read files;
//! - maintain global mutable state.
//!
//! Hardware/provider adapters are responsible for obtaining calibration data
//! and constructing `CalibrationSnapshot` values.
//!
//! Benchmark protocols consume snapshots as immutable experimental context.
//!
//! # Architectural position
//!
//! ```text
//!                         Hardware / Simulator
//!                                  │
//!                                  │ provider adapter
//!                                  ▼
//!                    CalibrationSnapshot::builder
//!                                  │
//!                                  ▼
//!                  ┌─────────────────────────────┐
//!                  │ This module                 │
//!                  │ calibration evidence/state  │
//!                  └──────────────┬──────────────┘
//!                                 │
//!              ┌──────────────────┼──────────────────┐
//!              ▼                  ▼                  ▼
//!        drift benchmark    benchmark provenance   executor
//!              │                  │                  │
//!              ▼                  ▼                  ▼
//!        protocols::drift    core::provenance   execution
//!
//! ```
//!
//! # Important semantic rule
//!
//! Missing calibration is represented by `Option<T>`.
//!
//! Zero is a valid physical value for some quantities and must therefore never
//! be used as a sentinel for "unknown".
//!
//! # Scientific rule
//!
//! A calibration value is an observation, not an absolute truth.
//!
//! Every empirical calibration metric may therefore carry:
//!
//! - sample count;
//! - uncertainty;
//! - confidence level;
//! - method;
//! - measurement timestamp;
//! - optional source identifier.
//!
//! This allows later benchmark analysis to distinguish:
//!
//! ```text
//! exact backend metadata
//! measured calibration
//! inferred calibration
//! unavailable calibration
//! ```
//!
//! # Determinism
//!
//! Collections use `BTreeMap`/`BTreeSet` so serialized snapshots have stable
//! ordering. Fingerprints are SHA-256 hashes over canonical JSON generated
//! from this deterministic representation.
//!
//! The fingerprint is an integrity/fingerprint identifier. It is NOT a
//! cryptographic signature and does not prove who supplied the calibration.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This file is intentionally independent from the rest of
//! `quantum::benchmarking`.
//!
//! Future modules consume it as follows:
//!
//! - `benchmarking::hardware::capabilities`
//!     validates backend capabilities against calibration state.
//!
//! - `benchmarking::hardware::metadata`
//!     supplies backend/provider identity.
//!
//! - `benchmarking::hardware::timing`
//!     consumes calibrated gate/readout durations.
//!
//! - `benchmarking::protocols::drift`
//!     consumes calibration metrics as time-series evidence.
//!
//! - `benchmarking::protocols::coherence`
//!     consumes T1/T2/T2* calibration values.
//!
//! - `benchmarking::protocols::crosstalk`
//!     consumes pair/parallel calibration metrics.
//!
//! - `benchmarking::metrics::readout`
//!     consumes readout assignment/error data.
//!
//! - `benchmarking::metrics::gate_error`
//!     consumes gate/cycle/error metrics.
//!
//! - `benchmarking::core::provenance`
//!     records the snapshot identity, timestamp and fingerprint.
//!
//! - `benchmarking::core::result`
//!     attaches calibration identity to a benchmark result.
//!
//! - `benchmarking::analysis::*`
//!     compares calibration fingerprints before declaring results directly
//!     comparable.
//!
//! - `reporting::*`
//!     serializes this structure as part of benchmark artifacts.
//!
//! - `registry::*`
//!     does not depend on this module directly.
//!
//! - `quantum::hardware::calibration`
//!     remains the hardware-domain calibration model. A provider adapter may
//!     explicitly translate that hardware representation into this immutable
//!     benchmarking snapshot.
//!
//! This dependency direction is intentional:
//!
//! ```text
//! quantum::hardware
//!        │
//!        ▼
//! benchmarking::hardware::calibration
//!        │
//!        ├──► benchmarking protocols
//!        ├──► metrics
//!        ├──► provenance
//!        └──► analysis
//! ```
//!
//! The benchmark calibration module must never become a provider SDK.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

// ============================================================================
// Schema
// ============================================================================

/// Stable identifier for this calibration schema.
pub const CALIBRATION_SCHEMA_ID: &str =
    "zamani.quantum.benchmark.calibration";

/// Current serialized schema version.
pub const CALIBRATION_SCHEMA_VERSION: u16 = 1;

/// Maximum supported backend identifier length.
pub const MAX_BACKEND_ID_LENGTH: usize = 512;

/// Maximum supported provider identifier length.
pub const MAX_PROVIDER_ID_LENGTH: usize = 512;

/// Maximum supported calibration source identifier length.
pub const MAX_SOURCE_ID_LENGTH: usize = 512;

/// Maximum number of physical qubits in one snapshot.
pub const DEFAULT_MAX_QUBITS: usize = 1_000_000;

/// Maximum number of gate records in one snapshot.
pub const DEFAULT_MAX_GATES: usize = 5_000_000;

/// Maximum number of arbitrary metadata fields.
pub const DEFAULT_MAX_METADATA_FIELDS: usize = 4_096;

/// Maximum length of an arbitrary metadata key.
pub const MAX_METADATA_KEY_LENGTH: usize = 512;

/// Maximum length of an arbitrary metadata value.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4_096;

/// Maximum number of custom metric records.
pub const DEFAULT_MAX_CUSTOM_METRICS: usize = 1_000_000;

/// Maximum number of qubits referenced by one gate calibration.
pub const MAX_GATE_QUBITS: usize = 16;

/// Maximum supported uncertainty/confidence representation.
const PROBABILITY_EPSILON: f64 = 1.0e-12;

// ============================================================================
// Error model
// ============================================================================

/// Errors produced while constructing or validating calibration evidence.
#[derive(Debug, Clone, PartialEq)]
pub enum CalibrationError {
    /// A required textual identifier is empty.
    EmptyIdentifier {
        field: &'static str,
    },

    /// A textual identifier is too long.
    IdentifierTooLong {
        field: &'static str,
        length: usize,
        maximum: usize,
    },

    /// A backend/provider/source identifier contains unsupported content.
    InvalidIdentifier {
        field: &'static str,
    },

    /// A qubit identifier is invalid.
    InvalidQubit {
        qubit: usize,
    },

    /// Too many qubits were supplied.
    QubitLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// A gate name is invalid.
    InvalidGate {
        gate: String,
    },

    /// A gate references too many qubits.
    GateQubitLimitExceeded {
        gate: String,
        requested: usize,
        maximum: usize,
    },

    /// A gate contains duplicate qubit identifiers.
    DuplicateGateQubit {
        gate: String,
        qubit: usize,
    },

    /// Too many gate records were supplied.
    GateLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// A probability is outside [0, 1].
    InvalidProbability {
        field: String,
        value: f64,
    },

    /// A numeric calibration value is non-finite or invalid.
    InvalidNumericValue {
        field: String,
        value: f64,
    },

    /// A duration is invalid.
    InvalidDuration {
        field: String,
        value_ns: u64,
    },

    /// A timestamp is invalid.
    InvalidTimestamp {
        field: &'static str,
        value_ns: u64,
    },

    /// A validity interval is inconsistent.
    InvalidValidityInterval {
        valid_from_ns: u64,
        valid_until_ns: Option<u64>,
    },

    /// A confidence level is invalid.
    InvalidConfidenceLevel {
        value: f64,
    },

    /// An uncertainty is invalid.
    InvalidUncertainty {
        field: String,
        value: f64,
    },

    /// A measurement sample count is invalid.
    InvalidSampleCount {
        field: String,
    },

    /// A custom metric identifier is invalid.
    InvalidMetricId {
        metric: String,
    },

    /// Too many custom metrics were supplied.
    CustomMetricLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Too many metadata fields were supplied.
    MetadataLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// A metadata key is invalid.
    InvalidMetadataKey {
        key: String,
    },

    /// A metadata value is too long.
    MetadataValueTooLong {
        key: String,
        length: usize,
        maximum: usize,
    },

    /// A snapshot contains no calibration evidence.
    EmptySnapshot,

    /// A snapshot is stale according to a caller-supplied policy.
    StaleCalibration {
        age_ns: u64,
        maximum_age_ns: u64,
    },

    /// A requested calibration record does not exist.
    CalibrationUnavailable {
        resource: String,
    },

    /// Two records conflict.
    ConflictingCalibration {
        resource: String,
        message: String,
    },

    /// Canonical serialization failed.
    Serialization {
        message: String,
    },

    /// Fingerprint generation failed.
    Fingerprint {
        message: String,
    },
}

impl fmt::Display for CalibrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} cannot be empty")
            }

            Self::IdentifierTooLong {
                field,
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "{field} is {length} bytes long; maximum is {maximum}"
                )
            }

            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid identifier for {field}")
            }

            Self::InvalidQubit { qubit } => {
                write!(formatter, "invalid physical qubit identifier {qubit}")
            }

            Self::QubitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "calibration contains {requested} qubits; maximum is {maximum}"
                )
            }

            Self::InvalidGate { gate } => {
                write!(formatter, "invalid gate identifier `{gate}`")
            }

            Self::GateQubitLimitExceeded {
                gate,
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "gate `{gate}` references {requested} qubits; \
                     maximum is {maximum}"
                )
            }

            Self::DuplicateGateQubit { gate, qubit } => {
                write!(
                    formatter,
                    "gate `{gate}` references qubit {qubit} more than once"
                )
            }

            Self::GateLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "calibration contains {requested} gate records; \
                     maximum is {maximum}"
                )
            }

            Self::InvalidProbability { field, value } => {
                write!(
                    formatter,
                    "invalid probability for `{field}`: {value}"
                )
            }

            Self::InvalidNumericValue { field, value } => {
                write!(
                    formatter,
                    "invalid numeric calibration value for `{field}`: {value}"
                )
            }

            Self::InvalidDuration {
                field,
                value_ns,
            } => {
                write!(
                    formatter,
                    "invalid duration for `{field}`: {value_ns} ns"
                )
            }

            Self::InvalidTimestamp { field, value_ns } => {
                write!(
                    formatter,
                    "invalid timestamp for `{field}`: {value_ns} ns"
                )
            }

            Self::InvalidValidityInterval {
                valid_from_ns,
                valid_until_ns,
            } => {
                write!(
                    formatter,
                    "invalid calibration validity interval: \
                     from={valid_from_ns}, until={valid_until_ns:?}"
                )
            }

            Self::InvalidConfidenceLevel { value } => {
                write!(
                    formatter,
                    "confidence level must be finite and in (0, 1), got {value}"
                )
            }

            Self::InvalidUncertainty { field, value } => {
                write!(
                    formatter,
                    "uncertainty for `{field}` must be finite and \
                     non-negative, got {value}"
                )
            }

            Self::InvalidSampleCount { field } => {
                write!(
                    formatter,
                    "sample count for `{field}` must be greater than zero"
                )
            }

            Self::InvalidMetricId { metric } => {
                write!(
                    formatter,
                    "invalid calibration metric identifier `{metric}`"
                )
            }

            Self::CustomMetricLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "calibration contains {requested} custom metrics; \
                     maximum is {maximum}"
                )
            }

            Self::MetadataLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "calibration contains {requested} metadata fields; \
                     maximum is {maximum}"
                )
            }

            Self::InvalidMetadataKey { key } => {
                write!(formatter, "invalid calibration metadata key `{key}`")
            }

            Self::MetadataValueTooLong {
                key,
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "metadata value `{key}` is {length} bytes; \
                     maximum is {maximum}"
                )
            }

            Self::EmptySnapshot => {
                formatter.write_str(
                    "calibration snapshot contains no calibration evidence",
                )
            }

            Self::StaleCalibration {
                age_ns,
                maximum_age_ns,
            } => {
                write!(
                    formatter,
                    "calibration is stale: age={age_ns} ns; \
                     maximum allowed age={maximum_age_ns} ns"
                )
            }

            Self::CalibrationUnavailable { resource } => {
                write!(
                    formatter,
                    "calibration unavailable for `{resource}`"
                )
            }

            Self::ConflictingCalibration {
                resource,
                message,
            } => {
                write!(
                    formatter,
                    "conflicting calibration for `{resource}`: {message}"
                )
            }

            Self::Serialization { message } => {
                write!(
                    formatter,
                    "unable to serialize calibration snapshot: {message}"
                )
            }

            Self::Fingerprint { message } => {
                write!(
                    formatter,
                    "unable to fingerprint calibration snapshot: {message}"
                )
            }
        }
    }
}

impl std::error::Error for CalibrationError {}

// ============================================================================
// Calibration source
// ============================================================================

/// Describes where calibration evidence came from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CalibrationSource {
    /// Direct measurement performed by a provider/backend.
    Measured,

    /// Calibration supplied by backend metadata.
    BackendMetadata,

    /// Calibration obtained from a simulator/emulator model.
    SimulatorModel,

    /// Calibration inferred from another benchmark.
    BenchmarkDerived,

    /// Calibration imported from another system.
    Imported,

    /// Calibration manually supplied by an operator.
    Manual,

    /// Source is explicitly unknown.
    Unknown,
}

impl Default for CalibrationSource {
    fn default() -> Self {
        Self::Unknown
    }
}

// ============================================================================
// Calibration timestamp / validity
// ============================================================================

/// Explicit timestamp used by a calibration snapshot.
///
/// The value is Unix epoch nanoseconds.
///
/// Callers should prefer the timestamp supplied by the hardware/provider.
/// `now()` exists for local/synthetic backends and tests.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct CalibrationTimestamp {
    unix_ns: u64,
}

impl CalibrationTimestamp {
    /// Creates a timestamp from Unix nanoseconds.
    pub const fn from_unix_nanos(unix_ns: u64) -> Self {
        Self { unix_ns }
    }

    /// Returns Unix nanoseconds.
    pub const fn as_unix_nanos(self) -> u64 {
        self.unix_ns
    }

    /// Captures current system time.
    ///
    /// This method is intentionally not used automatically by snapshot
    /// mutation. Providers should normally supply their actual calibration
    /// timestamp.
    pub fn now() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);

        Self {
            unix_ns: nanos.min(u64::MAX as u128) as u64,
        }
    }

    /// Returns the elapsed age relative to a supplied current timestamp.
    pub fn age_since(
        self,
        now: CalibrationTimestamp,
    ) -> u64 {
        now.unix_ns.saturating_sub(self.unix_ns)
    }
}

/// Validity interval for a calibration snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalibrationValidity {
    /// Time from which the calibration is considered valid.
    pub valid_from: CalibrationTimestamp,

    /// Optional explicit expiration time.
    pub valid_until: Option<CalibrationTimestamp>,
}

impl CalibrationValidity {
    /// Creates an open-ended validity interval.
    pub fn from(
        valid_from: CalibrationTimestamp,
    ) -> Self {
        Self {
            valid_from,
            valid_until: None,
        }
    }

    /// Creates a bounded validity interval.
    pub fn until(
        valid_from: CalibrationTimestamp,
        valid_until: CalibrationTimestamp,
    ) -> Result<Self, CalibrationError> {
        if valid_until < valid_from {
            return Err(
                CalibrationError::InvalidValidityInterval {
                    valid_from_ns: valid_from.as_unix_nanos(),
                    valid_until_ns: Some(
                        valid_until.as_unix_nanos(),
                    ),
                },
            );
        }

        Ok(Self {
            valid_from,
            valid_until: Some(valid_until),
        })
    }

    /// Returns whether a timestamp falls inside the interval.
    pub fn contains(
        &self,
        timestamp: CalibrationTimestamp,
    ) -> bool {
        if timestamp < self.valid_from {
            return false;
        }

        match self.valid_until {
            Some(end) => timestamp <= end,
            None => true,
        }
    }

    /// Validates the interval.
    pub fn validate(&self) -> Result<(), CalibrationError> {
        if let Some(valid_until) = self.valid_until {
            if valid_until < self.valid_from {
                return Err(
                    CalibrationError::InvalidValidityInterval {
                        valid_from_ns: self.valid_from.as_unix_nanos(),
                        valid_until_ns: Some(
                            valid_until.as_unix_nanos(),
                        ),
                    },
                );
            }
        }

        Ok(())
    }
}

// ============================================================================
// Evidence metadata
// ============================================================================

/// Statistical metadata attached to one empirical calibration value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibrationEvidence {
    /// Number of observations/shots/experiments supporting the value.
    pub sample_count: u64,

    /// Optional standard error or equivalent uncertainty.
    ///
    /// The interpretation is specified by `uncertainty_kind`.
    pub uncertainty: Option<f64>,

    /// Human/machine-readable uncertainty interpretation.
    pub uncertainty_kind: Option<String>,

    /// Confidence level associated with the uncertainty, e.g. 0.95.
    pub confidence_level: Option<f64>,

    /// Method used to obtain the value.
    pub method: Option<String>,

    /// Optional source/run identifier.
    pub source_id: Option<String>,
}

impl Default for CalibrationEvidence {
    fn default() -> Self {
        Self {
            sample_count: 0,
            uncertainty: None,
            uncertainty_kind: None,
            confidence_level: None,
            method: None,
            source_id: None,
        }
    }
}

impl CalibrationEvidence {
    /// Creates metadata for an empirical measurement.
    pub fn measured(sample_count: u64) -> Result<Self, CalibrationError> {
        if sample_count == 0 {
            return Err(
                CalibrationError::InvalidSampleCount {
                    field: "sample_count".to_string(),
                },
            );
        }

        Ok(Self {
            sample_count,
            ..Self::default()
        })
    }

    /// Adds an uncertainty.
    pub fn with_uncertainty(
        mut self,
        uncertainty: f64,
        kind: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        if !uncertainty.is_finite() || uncertainty < 0.0 {
            return Err(
                CalibrationError::InvalidUncertainty {
                    field: "uncertainty".to_string(),
                    value: uncertainty,
                },
            );
        }

        let kind = kind.into();

        if kind.trim().is_empty() {
            return Err(
                CalibrationError::InvalidIdentifier {
                    field: "uncertainty_kind",
                },
            );
        }

        self.uncertainty = Some(uncertainty);
        self.uncertainty_kind = Some(kind);
        Ok(self)
    }

    /// Adds a confidence level.
    pub fn with_confidence_level(
        mut self,
        confidence_level: f64,
    ) -> Result<Self, CalibrationError> {
        validate_confidence_level(confidence_level)?;
        self.confidence_level = Some(confidence_level);
        Ok(self)
    }

    /// Adds the measurement method.
    pub fn with_method(
        mut self,
        method: impl Into<String>,
    ) -> Self {
        self.method = Some(method.into());
        self
    }

    /// Adds a source identifier.
    pub fn with_source_id(
        mut self,
        source_id: impl Into<String>,
    ) -> Self {
        self.source_id = Some(source_id.into());
        self
    }

    /// Validates the evidence.
    pub fn validate(
        &self,
        field_prefix: &str,
    ) -> Result<(), CalibrationError> {
        if self.sample_count == 0
            && (self.uncertainty.is_some()
                || self.confidence_level.is_some()
                || self.method.is_some()
                || self.source_id.is_some())
        {
            return Err(
                CalibrationError::InvalidSampleCount {
                    field: field_prefix.to_string(),
                },
            );
        }

        if let Some(uncertainty) = self.uncertainty {
            if !uncertainty.is_finite() || uncertainty < 0.0 {
                return Err(
                    CalibrationError::InvalidUncertainty {
                        field: field_prefix.to_string(),
                        value: uncertainty,
                    },
                );
            }
        }

        if let Some(confidence) = self.confidence_level {
            validate_confidence_level(confidence)?;
        }

        Ok(())
    }
}

// ============================================================================
// Metric value
// ============================================================================

/// A scalar calibration metric.
///
/// `value` is always present when a record exists. Missing measurements are
/// represented by the absence of the corresponding metric record rather than
/// by zero.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibrationMetric {
    /// Stable metric identifier.
    pub metric_id: String,

    /// Measured/inferred value.
    pub value: f64,

    /// Unit identifier.
    pub unit: String,

    /// Higher-level interpretation of the metric.
    pub direction: CalibrationMetricDirection,

    /// Evidence supporting this value.
    pub evidence: CalibrationEvidence,

    /// Origin of the metric.
    pub source: CalibrationSource,
}

impl CalibrationMetric {
    /// Creates a calibration metric.
    pub fn new(
        metric_id: impl Into<String>,
        value: f64,
        unit: impl Into<String>,
        direction: CalibrationMetricDirection,
        source: CalibrationSource,
    ) -> Result<Self, CalibrationError> {
        let metric_id = normalize_metric_id(&metric_id.into())?;

        if !value.is_finite() {
            return Err(
                CalibrationError::InvalidNumericValue {
                    field: format!("metric.{metric_id}"),
                    value,
                },
            );
        }

        let unit = unit.into();

        if unit.trim().is_empty() {
            return Err(
                CalibrationError::InvalidIdentifier {
                    field: "metric.unit",
                },
            );
        }

        Ok(Self {
            metric_id,
            value,
            unit,
            direction,
            evidence: CalibrationEvidence::default(),
            source,
        })
    }

    /// Adds evidence.
    pub fn with_evidence(
        mut self,
        evidence: CalibrationEvidence,
    ) -> Result<Self, CalibrationError> {
        evidence.validate(&format!(
            "metric.{}",
            self.metric_id
        ))?;

        self.evidence = evidence;
        Ok(self)
    }
}

/// Quality direction of a calibration metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CalibrationMetricDirection {
    /// Larger is normally better.
    HigherIsBetter,

    /// Smaller is normally better.
    LowerIsBetter,

    /// No universal quality direction.
    Neutral,
}

// ============================================================================
// Qubit calibration
// ============================================================================

/// Calibration evidence for one physical qubit.
///
/// All fields are optional because real backends may expose only a subset of
/// these values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QubitCalibration {
    /// Physical qubit identifier.
    pub qubit: usize,

    /// Qubit is currently usable by the backend.
    pub operational: bool,

    /// Relaxation time T1 in nanoseconds.
    pub t1_ns: Option<CalibrationMetric>,

    /// Dephasing/coherence time T2 in nanoseconds.
    pub t2_ns: Option<CalibrationMetric>,

    /// Inhomogeneous dephasing time T2* in nanoseconds.
    pub t2_star_ns: Option<CalibrationMetric>,

    /// Qubit transition frequency in Hz.
    pub frequency_hz: Option<CalibrationMetric>,

    /// Reset error probability.
    pub reset_error: Option<CalibrationMetric>,

    /// Leakage probability.
    pub leakage_error: Option<CalibrationMetric>,

    /// Custom qubit-level metrics.
    pub metrics: BTreeMap<String, CalibrationMetric>,
}

impl QubitCalibration {
    /// Creates an empty qubit calibration record.
    pub fn new(qubit: usize) -> Result<Self, CalibrationError> {
        validate_qubit(qubit)?;

        Ok(Self {
            qubit,
            operational: true,
            t1_ns: None,
            t2_ns: None,
            t2_star_ns: None,
            frequency_hz: None,
            reset_error: None,
            leakage_error: None,
            metrics: BTreeMap::new(),
        })
    }

    /// Marks the qubit operational/unavailable.
    pub fn with_operational(
        mut self,
        operational: bool,
    ) -> Self {
        self.operational = operational;
        self
    }

    /// Sets T1.
    pub fn with_t1_ns(
        mut self,
        value: f64,
        evidence: CalibrationEvidence,
        source: CalibrationSource,
    ) -> Result<Self, CalibrationError> {
        self.t1_ns = Some(CalibrationMetric::new(
            "t1",
            validate_positive_finite(
                "qubit.t1_ns",
                value,
            )?,
            "ns",
            CalibrationMetricDirection::HigherIsBetter,
            source,
        )?
        .with_evidence(evidence)?);

        Ok(self)
    }

    /// Sets T2.
    pub fn with_t2_ns(
        mut self,
        value: f64,
        evidence: CalibrationEvidence,
        source: CalibrationSource,
    ) -> Result<Self, CalibrationError> {
        self.t2_ns = Some(CalibrationMetric::new(
            "t2",
            validate_positive_finite(
                "qubit.t2_ns",
                value,
            )?,
            "ns",
            CalibrationMetricDirection::HigherIsBetter,
            source,
        )?
        .with_evidence(evidence)?);

        Ok(self)
    }

    /// Sets T2*.
    pub fn with_t2_star_ns(
        mut self,
        value: f64,
        evidence: CalibrationEvidence,
        source: CalibrationSource,
    ) -> Result<Self, CalibrationError> {
        self.t2_star_ns = Some(CalibrationMetric::new(
            "t2_star",
            validate_positive_finite(
                "qubit.t2_star_ns",
                value,
            )?,
            "ns",
            CalibrationMetricDirection::HigherIsBetter,
            source,
        )?
        .with_evidence(evidence)?);

        Ok(self)
    }

    /// Sets transition frequency.
    pub fn with_frequency_hz(
        mut self,
        value: f64,
        evidence: CalibrationEvidence,
        source: CalibrationSource,
    ) -> Result<Self, CalibrationError> {
        self.frequency_hz = Some(CalibrationMetric::new(
            "frequency",
            validate_non_negative_finite(
                "qubit.frequency_hz",
                value,
            )?,
            "Hz",
            CalibrationMetricDirection::Neutral,
            source,
        )?
        .with_evidence(evidence)?);

        Ok(self)
    }

    /// Sets reset error.
    pub fn with_reset_error(
        mut self,
        value: f64,
        evidence: CalibrationEvidence,
        source: CalibrationSource,
    ) -> Result<Self, CalibrationError> {
        validate_probability(
            "qubit.reset_error",
            value,
        )?;

        self.reset_error = Some(
            CalibrationMetric::new(
                "reset_error",
                value,
                "probability",
                CalibrationMetricDirection::LowerIsBetter,
                source,
            )?
            .with_evidence(evidence)?,
        );

        Ok(self)
    }

    /// Sets leakage error.
    pub fn with_leakage_error(
        mut self,
        value: f64,
        evidence: CalibrationEvidence,
        source: CalibrationSource,
    ) -> Result<Self, CalibrationError> {
        validate_probability(
            "qubit.leakage_error",
            value,
        )?;

        self.leakage_error = Some(
            CalibrationMetric::new(
                "leakage_error",
                value,
                "probability",
                CalibrationMetricDirection::LowerIsBetter,
                source,
            )?
            .with_evidence(evidence)?,
        );

        Ok(self)
    }

    /// Inserts a custom qubit-level metric.
    pub fn insert_metric(
        &mut self,
        metric: CalibrationMetric,
    ) -> Result<(), CalibrationError> {
        metric.evidence.validate(&format!(
            "qubit.{}.metric.{}",
            self.qubit,
            metric.metric_id
        ))?;

        self.metrics
            .insert(metric.metric_id.clone(), metric);

        Ok(())
    }

    /// Returns effective coherence when both T1 and T2 exist.
    pub fn effective_coherence_ns(&self) -> Option<f64> {
        match (&self.t1_ns, &self.t2_ns) {
            (Some(t1), Some(t2)) => {
                Some(t1.value.min(t2.value))
            }
            (Some(t1), None) => Some(t1.value),
            (None, Some(t2)) => Some(t2.value),
            (None, None) => None,
        }
    }

    /// Validates this record.
    pub fn validate(&self) -> Result<(), CalibrationError> {
        validate_qubit(self.qubit)?;

        if let Some(metric) = &self.t1_ns {
            validate_metric(
                metric,
                "qubit.t1_ns",
            )?;

            if metric.value <= 0.0 {
                return Err(
                    CalibrationError::InvalidNumericValue {
                        field: "qubit.t1_ns".to_string(),
                        value: metric.value,
                    },
                );
            }
        }

        if let Some(metric) = &self.t2_ns {
            validate_metric(
                metric,
                "qubit.t2_ns",
            )?;

            if metric.value <= 0.0 {
                return Err(
                    CalibrationError::InvalidNumericValue {
                        field: "qubit.t2_ns".to_string(),
                        value: metric.value,
                    },
                );
            }
        }

        if let Some(metric) = &self.t2_star_ns {
            validate_metric(
                metric,
                "qubit.t2_star_ns",
            )?;

            if metric.value <= 0.0 {
                return Err(
                    CalibrationError::InvalidNumericValue {
                        field: "qubit.t2_star_ns".to_string(),
                        value: metric.value,
                    },
                );
            }
        }

        if let Some(metric) = &self.frequency_hz {
            validate_metric(
                metric,
                "qubit.frequency_hz",
            )?;

            if metric.value < 0.0 {
                return Err(
                    CalibrationError::InvalidNumericValue {
                        field: "qubit.frequency_hz".to_string(),
                        value: metric.value,
                    },
                );
            }
        }

        if let Some(metric) = &self.reset_error {
            validate_metric(
                metric,
                "qubit.reset_error",
            )?;
            validate_probability(
                "qubit.reset_error",
                metric.value,
            )?;
        }

        if let Some(metric) = &self.leakage_error {
            validate_metric(
                metric,
                "qubit.leakage_error",
            )?;
            validate_probability(
                "qubit.leakage_error",
                metric.value,
            )?;
        }

        for metric in self.metrics.values() {
            validate_metric(metric, "qubit.custom_metric")?;
        }

        Ok(())
    }
}

// ============================================================================
// Readout calibration
// ============================================================================

/// Readout assignment calibration for one physical qubit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadoutCalibration {
    /// Physical qubit identifier.
    pub qubit: usize,

    /// P(measure 1 | prepared 0).
    pub p01: CalibrationMetric,

    /// P(measure 0 | prepared 1).
    pub p10: CalibrationMetric,

    /// Optional full assignment matrix for a single qubit.
    ///
    /// Entries are ordered:
    ///
    /// ```text
    /// [[P(0|0), P(0|1)],
    ///  [P(1|0), P(1|1)]]
    /// ```
    ///
    /// The matrix is represented as four values to avoid a floating-point
    /// nested-array abstraction with ambiguous ordering.
    pub assignment_matrix: Option<[f64; 4]>,

    /// Optional simultaneous-readout aggregate error.
    pub simultaneous_error: Option<CalibrationMetric>,

    /// Additional readout metrics.
    pub metrics: BTreeMap<String, CalibrationMetric>,
}

impl ReadoutCalibration {
    /// Creates readout calibration from asymmetric assignment errors.
    pub fn new(
        qubit: usize,
        p01: f64,
        p10: f64,
        evidence: CalibrationEvidence,
        source: CalibrationSource,
    ) -> Result<Self, CalibrationError> {
        validate_qubit(qubit)?;
        validate_probability(
            "readout.p01",
            p01,
        )?;
        validate_probability(
            "readout.p10",
            p10,
        )?;

        let p01_metric = CalibrationMetric::new(
            "p01",
            p01,
            "probability",
            CalibrationMetricDirection::LowerIsBetter,
            source.clone(),
        )?
        .with_evidence(evidence.clone())?;

        let p10_metric = CalibrationMetric::new(
            "p10",
            p10,
            "probability",
            CalibrationMetricDirection::LowerIsBetter,
            source,
        )?
        .with_evidence(evidence)?;

        Ok(Self {
            qubit,
            p01: p01_metric,
            p10: p10_metric,
            assignment_matrix: None,
            simultaneous_error: None,
            metrics: BTreeMap::new(),
        })
    }

    /// Returns average isolated readout error.
    pub fn average_error(&self) -> f64 {
        (self.p01.value + self.p10.value) / 2.0
    }

    /// Returns average readout fidelity.
    pub fn average_fidelity(&self) -> f64 {
        1.0 - self.average_error()
    }

    /// Sets the assignment matrix.
    pub fn with_assignment_matrix(
        mut self,
        matrix: [f64; 4],
    ) -> Result<Self, CalibrationError> {
        for (index, value) in matrix.iter().enumerate() {
            validate_probability(
                match index {
                    0 => "assignment_matrix.p00",
                    1 => "assignment_matrix.p01",
                    2 => "assignment_matrix.p10",
                    _ => "assignment_matrix.p11",
                },
                *value,
            )?;
        }

        let row0 = matrix[0] + matrix[1];
        let row1 = matrix[2] + matrix[3];

        if (row0 - 1.0).abs() > PROBABILITY_EPSILON
            || (row1 - 1.0).abs() > PROBABILITY_EPSILON
        {
            return Err(
                CalibrationError::ConflictingCalibration {
                    resource: format!(
                        "readout.qubit.{}",
                        self.qubit
                    ),
                    message:
                        "assignment matrix rows must sum to one"
                            .to_string(),
                },
            );
        }

        self.assignment_matrix = Some(matrix);
        Ok(self)
    }

    /// Sets a simultaneous readout error.
    pub fn with_simultaneous_error(
        mut self,
        value: f64,
        evidence: CalibrationEvidence,
        source: CalibrationSource,
    ) -> Result<Self, CalibrationError> {
        validate_probability(
            "readout.simultaneous_error",
            value,
        )?;

        self.simultaneous_error = Some(
            CalibrationMetric::new(
                "simultaneous_error",
                value,
                "probability",
                CalibrationMetricDirection::LowerIsBetter,
                source,
            )?
            .with_evidence(evidence)?,
        );

        Ok(self)
    }

    /// Inserts a custom readout metric.
    pub fn insert_metric(
        &mut self,
        metric: CalibrationMetric,
    ) -> Result<(), CalibrationError> {
        validate_metric(
            &metric,
            "readout.custom_metric",
        )?;

        self.metrics
            .insert(metric.metric_id.clone(), metric);

        Ok(())
    }

    /// Validates readout calibration.
    pub fn validate(&self) -> Result<(), CalibrationError> {
        validate_qubit(self.qubit)?;

        validate_metric(
            &self.p01,
            "readout.p01",
        )?;

        validate_metric(
            &self.p10,
            "readout.p10",
        )?;

        validate_probability(
            "readout.p01",
            self.p01.value,
        )?;

        validate_probability(
            "readout.p10",
            self.p10.value,
        )?;

        if let Some(matrix) = self.assignment_matrix {
            for value in matrix {
                validate_probability(
                    "readout.assignment_matrix",
                    value,
                )?;
            }

            if (matrix[0] + matrix[1] - 1.0).abs()
                > PROBABILITY_EPSILON
                || (matrix[2] + matrix[3] - 1.0).abs()
                    > PROBABILITY_EPSILON
            {
                return Err(
                    CalibrationError::ConflictingCalibration {
                        resource: format!(
                            "readout.qubit.{}",
                            self.qubit
                        ),
                        message:
                            "assignment matrix rows must sum to one"
                                .to_string(),
                    },
                );
            }
        }

        if let Some(metric) = &self.simultaneous_error {
            validate_metric(
                metric,
                "readout.simultaneous_error",
            )?;
            validate_probability(
                "readout.simultaneous_error",
                metric.value,
            )?;
        }

        for metric in self.metrics.values() {
            validate_metric(
                metric,
                "readout.custom_metric",
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// Gate calibration
// ============================================================================

/// Calibration evidence for a physical gate instance.
///
/// The same logical/native gate can have different calibration parameters on
/// different qubits or qubit pairs, so the qubit tuple is part of the identity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GateCalibration {
    /// Canonical gate identifier.
    pub gate: String,

    /// Physical qubits targeted by this gate.
    pub qubits: Vec<usize>,

    /// Whether the operation is currently operational.
    pub operational: bool,

    /// Gate duration in nanoseconds.
    pub duration_ns: Option<u64>,

    /// Average gate error probability, if known.
    pub error_rate: Option<CalibrationMetric>,

    /// Incoherent error component, if known.
    pub incoherent_error: Option<CalibrationMetric>,

    /// Pauli error component, if known.
    pub pauli_error: Option<CalibrationMetric>,

    /// Coherent error indicator, if available.
    pub coherent_error: Option<CalibrationMetric>,

    /// Optional cycle fidelity.
    pub cycle_fidelity: Option<CalibrationMetric>,

    /// Additional gate-level metrics.
    pub metrics: BTreeMap<String, CalibrationMetric>,
}

impl GateCalibration {
    /// Creates a gate calibration record.
    pub fn new(
        gate: impl Into<String>,
        qubits: Vec<usize>,
    ) -> Result<Self, CalibrationError> {
        let gate = normalize_gate_name(&gate.into())?;

        validate_gate_qubits(
            &gate,
            &qubits,
        )?;

        Ok(Self {
            gate,
            qubits,
            operational: true,
            duration_ns: None,
            error_rate: None,
            incoherent_error: None,
            pauli_error: None,
            coherent_error: None,
            cycle_fidelity: None,
            metrics: BTreeMap::new(),
        })
    }

    /// Sets operational state.
    pub fn with_operational(
        mut self,
        operational: bool,
    ) -> Self {
        self.operational = operational;
        self
    }

    /// Sets duration.
    pub fn with_duration_ns(
        mut self,
        duration_ns: u64,
    ) -> Result<Self, CalibrationError> {
        if duration_ns == 0 {
            return Err(
                CalibrationError::InvalidDuration {
                    field: format!(
                        "gate.{}.duration_ns",
                        self.gate
                    ),
                    value_ns: duration_ns,
                },
            );
        }

        self.duration_ns = Some(duration_ns);
        Ok(self)
    }

    /// Sets average error rate.
    pub fn with_error_rate(
        mut self,
        value: f64,
        evidence: CalibrationEvidence,
        source: CalibrationSource,
    ) -> Result<Self, CalibrationError> {
        self.error_rate = Some(
            probability_metric(
                "error_rate",
                value,
                CalibrationMetricDirection::LowerIsBetter,
                evidence,
                source,
            )?,
        );

        Ok(self)
    }

    /// Sets incoherent error.
    pub fn with_incoherent_error(
        mut self,
        value: f64,
        evidence: CalibrationEvidence,
        source: CalibrationSource,
    ) -> Result<Self, CalibrationError> {
        self.incoherent_error = Some(
            probability_metric(
                "incoherent_error",
                value,
                CalibrationMetricDirection::LowerIsBetter,
                evidence,
                source,
            )?,
        );

        Ok(self)
    }

    /// Sets Pauli error.
    pub fn with_pauli_error(
        mut self,
        value: f64,
        evidence: CalibrationEvidence,
        source: CalibrationSource,
    ) -> Result<Self, CalibrationError> {
        self.pauli_error = Some(
            probability_metric(
                "pauli_error",
                value,
                CalibrationMetricDirection::LowerIsBetter,
                evidence,
                source,
            )?,
        );

        Ok(self)
    }

    /// Sets coherent-error metric.
    pub fn with_coherent_error(
        mut self,
        value: f64,
        unit: impl Into<String>,
        evidence: CalibrationEvidence,
        source: CalibrationSource,
    ) -> Result<Self, CalibrationError> {
        self.coherent_error = Some(
            CalibrationMetric::new(
                "coherent_error",
                validate_non_negative_finite(
                    "gate.coherent_error",
                    value,
                )?,
                unit,
                CalibrationMetricDirection::LowerIsBetter,
                source,
            )?
            .with_evidence(evidence)?,
        );

        Ok(self)
    }

    /// Sets cycle fidelity.
    pub fn with_cycle_fidelity(
        mut self,
        value: f64,
        evidence: CalibrationEvidence,
        source: CalibrationSource,
    ) -> Result<Self, CalibrationError> {
        validate_probability(
            "gate.cycle_fidelity",
            value,
        )?;

        self.cycle_fidelity = Some(
            CalibrationMetric::new(
                "cycle_fidelity",
                value,
                "probability",
                CalibrationMetricDirection::HigherIsBetter,
                source,
            )?
            .with_evidence(evidence)?,
        );

        Ok(self)
    }

    /// Inserts a custom gate metric.
    pub fn insert_metric(
        &mut self,
        metric: CalibrationMetric,
    ) -> Result<(), CalibrationError> {
        validate_metric(
            &metric,
            "gate.custom_metric",
        )?;

        self.metrics
            .insert(metric.metric_id.clone(), metric);

        Ok(())
    }

    /// Returns a stable gate key.
    pub fn key(&self) -> String {
        gate_key(
            &self.gate,
            &self.qubits,
        )
    }

    /// Validates this gate record.
    pub fn validate(&self) -> Result<(), CalibrationError> {
        let normalized = normalize_gate_name(&self.gate)?;

        if normalized != self.gate {
            return Err(
                CalibrationError::InvalidGate {
                    gate: self.gate.clone(),
                },
            );
        }

        validate_gate_qubits(
            &self.gate,
            &self.qubits,
        )?;

        if let Some(duration_ns) = self.duration_ns {
            if duration_ns == 0 {
                return Err(
                    CalibrationError::InvalidDuration {
                        field: format!(
                            "gate.{}.duration_ns",
                            self.gate
                        ),
                        value_ns: duration_ns,
                    },
                );
            }
        }

        validate_optional_probability_metric(
            &self.error_rate,
            "gate.error_rate",
        )?;

        validate_optional_probability_metric(
            &self.incoherent_error,
            "gate.incoherent_error",
        )?;

        validate_optional_probability_metric(
            &self.pauli_error,
            "gate.pauli_error",
        )?;

        if let Some(metric) = &self.coherent_error {
            validate_metric(
                metric,
                "gate.coherent_error",
            )?;
        }

        if let Some(metric) = &self.cycle_fidelity {
            validate_metric(
                metric,
                "gate.cycle_fidelity",
            )?;
            validate_probability(
                "gate.cycle_fidelity",
                metric.value,
            )?;
        }

        for metric in self.metrics.values() {
            validate_metric(
                metric,
                "gate.custom_metric",
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// Crosstalk / parallel calibration
// ============================================================================

/// Calibration information for an interaction between resources.
///
/// This is deliberately separate from `GateCalibration` because crosstalk
/// describes the effect of operating one resource while another resource is
/// active.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrosstalkCalibration {
    /// Primary qubit/resource.
    pub target_qubits: Vec<usize>,

    /// Concurrently active spectator/neighbor resources.
    pub spectator_qubits: Vec<usize>,

    /// Stable experiment identifier.
    pub experiment_id: Option<String>,

    /// Crosstalk error metric.
    pub error_rate: CalibrationMetric,

    /// Additional crosstalk metrics.
    pub metrics: BTreeMap<String, CalibrationMetric>,
}

impl CrosstalkCalibration {
    /// Creates a crosstalk record.
    pub fn new(
        target_qubits: Vec<usize>,
        spectator_qubits: Vec<usize>,
        error_rate: f64,
        evidence: CalibrationEvidence,
        source: CalibrationSource,
    ) -> Result<Self, CalibrationError> {
        validate_qubit_list(
            "crosstalk.target_qubits",
            &target_qubits,
        )?;

        validate_qubit_list(
            "crosstalk.spectator_qubits",
            &spectator_qubits,
        )?;

        if target_qubits
            .iter()
            .any(|qubit| spectator_qubits.contains(qubit))
        {
            return Err(
                CalibrationError::ConflictingCalibration {
                    resource: "crosstalk".to_string(),
                    message:
                        "target and spectator qubits must be disjoint"
                            .to_string(),
                },
            );
        }

        let metric = probability_metric(
            "crosstalk_error",
            error_rate,
            CalibrationMetricDirection::LowerIsBetter,
            evidence,
            source,
        )?;

        Ok(Self {
            target_qubits,
            spectator_qubits,
            experiment_id: None,
            error_rate: metric,
            metrics: BTreeMap::new(),
        })
    }

    /// Returns a deterministic key.
    pub fn key(&self) -> String {
        format!(
            "target:{}|spectator:{}",
            qubit_list_key(&self.target_qubits),
            qubit_list_key(&self.spectator_qubits)
        )
    }

    /// Validates the record.
    pub fn validate(&self) -> Result<(), CalibrationError> {
        validate_qubit_list(
            "crosstalk.target_qubits",
            &self.target_qubits,
        )?;

        validate_qubit_list(
            "crosstalk.spectator_qubits",
            &self.spectator_qubits,
        )?;

        if target_and_spectator_overlap(
            &self.target_qubits,
            &self.spectator_qubits,
        ) {
            return Err(
                CalibrationError::ConflictingCalibration {
                    resource: "crosstalk".to_string(),
                    message:
                        "target and spectator qubits overlap"
                            .to_string(),
                },
            );
        }

        validate_metric(
            &self.error_rate,
            "crosstalk.error_rate",
        )?;

        validate_probability(
            "crosstalk.error_rate",
            self.error_rate.value,
        )?;

        Ok(())
    }
}

// ============================================================================
// Snapshot limits
// ============================================================================

/// Resource limits applied to one calibration snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalibrationLimits {
    /// Maximum physical qubit records.
    pub max_qubits: usize,

    /// Maximum gate calibration records.
    pub max_gates: usize,

    /// Maximum custom metric records.
    pub max_custom_metrics: usize,

    /// Maximum metadata fields.
    pub max_metadata_fields: usize,
}

impl Default for CalibrationLimits {
    fn default() -> Self {
        Self {
            max_qubits: DEFAULT_MAX_QUBITS,
            max_gates: DEFAULT_MAX_GATES,
            max_custom_metrics: DEFAULT_MAX_CUSTOM_METRICS,
            max_metadata_fields: DEFAULT_MAX_METADATA_FIELDS,
        }
    }
}

impl CalibrationLimits {
    /// Creates conservative production defaults.
    pub const fn production() -> Self {
        Self {
            max_qubits: DEFAULT_MAX_QUBITS,
            max_gates: DEFAULT_MAX_GATES,
            max_custom_metrics: DEFAULT_MAX_CUSTOM_METRICS,
            max_metadata_fields: DEFAULT_MAX_METADATA_FIELDS,
        }
    }

    /// Validates limits.
    pub fn validate(&self) -> Result<(), CalibrationError> {
        if self.max_qubits == 0 {
            return Err(
                CalibrationError::QubitLimitExceeded {
                    requested: 0,
                    maximum: self.max_qubits,
                },
            );
        }

        if self.max_gates == 0 {
            return Err(
                CalibrationError::GateLimitExceeded {
                    requested: 0,
                    maximum: self.max_gates,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Calibration snapshot
// ============================================================================

/// Immutable-at-the-consumer-boundary calibration snapshot.
///
/// Construction is performed through `CalibrationSnapshotBuilder` or the
/// mutation methods provided here before the snapshot is handed to benchmark
/// execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibrationSnapshot {
    /// Schema identifier.
    pub schema_id: String,

    /// Serialized schema version.
    pub schema_version: u16,

    /// Stable backend identifier.
    pub backend_id: String,

    /// Backend/provider identifier.
    pub provider_id: Option<String>,

    /// Human-readable processor/model identifier.
    pub processor_id: Option<String>,

    /// Hardware technology label.
    ///
    /// Examples:
    ///
    /// - superconducting
    /// - trapped_ion
    /// - neutral_atom
    /// - photonic
    /// - spin
    /// - semiconductor
    /// - topological
    /// - simulator
    /// - emulator
    /// - annealing
    /// - analog
    pub technology: Option<String>,

    /// Origin of the snapshot.
    pub source: CalibrationSource,

    /// Calibration collection timestamp.
    pub captured_at: CalibrationTimestamp,

    /// Validity interval.
    pub validity: CalibrationValidity,

    /// Optional provider calibration identifier.
    pub calibration_id: Option<String>,

    /// Physical qubit calibration.
    pub qubits: BTreeMap<usize, QubitCalibration>,

    /// Gate calibration keyed by deterministic gate signature.
    pub gates: BTreeMap<String, GateCalibration>,

    /// Readout calibration keyed by physical qubit.
    pub readout: BTreeMap<usize, ReadoutCalibration>,

    /// Crosstalk/parallel-operation calibration.
    pub crosstalk: BTreeMap<String, CrosstalkCalibration>,

    /// Snapshot-level custom metrics.
    pub metrics: BTreeMap<String, CalibrationMetric>,

    /// Explicit metadata.
    pub metadata: BTreeMap<String, String>,

    /// Resource limits used to construct/validate the snapshot.
    pub limits: CalibrationLimits,
}

impl CalibrationSnapshot {
    /// Creates an empty snapshot using the current time.
    ///
    /// Provider integrations should normally prefer
    /// `with_capture_time(...)`.
    pub fn new(
        backend_id: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        Self::with_capture_time(
            backend_id,
            CalibrationTimestamp::now(),
        )
    }

    /// Creates an empty snapshot with an explicit capture timestamp.
    pub fn with_capture_time(
        backend_id: impl Into<String>,
        captured_at: CalibrationTimestamp,
    ) -> Result<Self, CalibrationError> {
        let backend_id = validate_backend_id(
            &backend_id.into(),
        )?;

        let validity = CalibrationValidity::from(
            captured_at,
        );

        let snapshot = Self {
            schema_id: CALIBRATION_SCHEMA_ID.to_string(),
            schema_version: CALIBRATION_SCHEMA_VERSION,
            backend_id,
            provider_id: None,
            processor_id: None,
            technology: None,
            source: CalibrationSource::Unknown,
            captured_at,
            validity,
            calibration_id: None,
            qubits: BTreeMap::new(),
            gates: BTreeMap::new(),
            readout: BTreeMap::new(),
            crosstalk: BTreeMap::new(),
            metrics: BTreeMap::new(),
            metadata: BTreeMap::new(),
            limits: CalibrationLimits::production(),
        };

        snapshot.validate()?;

        Ok(snapshot)
    }

    /// Sets provider identifier.
    pub fn with_provider_id(
        mut self,
        provider_id: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        self.provider_id = Some(
            validate_provider_id(
                &provider_id.into(),
            )?,
        );
        Ok(self)
    }

    /// Sets processor identifier.
    pub fn with_processor_id(
        mut self,
        processor_id: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        self.processor_id = Some(
            validate_identifier(
                "processor_id",
                &processor_id.into(),
                MAX_SOURCE_ID_LENGTH,
            )?,
        );
        Ok(self)
    }

    /// Sets technology label.
    pub fn with_technology(
        mut self,
        technology: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        let technology = technology.into();

        if technology.trim().is_empty() {
            return Err(
                CalibrationError::EmptyIdentifier {
                    field: "technology",
                },
            );
        }

        self.technology = Some(technology);
        Ok(self)
    }

    /// Sets calibration source.
    pub fn with_source(
        mut self,
        source: CalibrationSource,
    ) -> Self {
        self.source = source;
        self
    }

    /// Sets provider calibration identifier.
    pub fn with_calibration_id(
        mut self,
        calibration_id: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        self.calibration_id = Some(
            validate_identifier(
                "calibration_id",
                &calibration_id.into(),
                MAX_SOURCE_ID_LENGTH,
            )?,
        );
        Ok(self)
    }

    /// Sets a validity interval.
    pub fn with_validity(
        mut self,
        validity: CalibrationValidity,
    ) -> Result<Self, CalibrationError> {
        validity.validate()?;
        self.validity = validity;
        Ok(self)
    }

    /// Sets resource limits.
    pub fn with_limits(
        mut self,
        limits: CalibrationLimits,
    ) -> Result<Self, CalibrationError> {
        limits.validate()?;
        self.limits = limits;
        Ok(self)
    }

    /// Inserts qubit calibration.
    pub fn insert_qubit(
        &mut self,
        calibration: QubitCalibration,
    ) -> Result<(), CalibrationError> {
        calibration.validate()?;

        if !self.qubits.contains_key(&calibration.qubit)
            && self.qubits.len() >= self.limits.max_qubits
        {
            return Err(
                CalibrationError::QubitLimitExceeded {
                    requested: self.qubits.len() + 1,
                    maximum: self.limits.max_qubits,
                },
            );
        }

        self.qubits.insert(
            calibration.qubit,
            calibration,
        );

        Ok(())
    }

    /// Inserts gate calibration.
    pub fn insert_gate(
        &mut self,
        calibration: GateCalibration,
    ) -> Result<(), CalibrationError> {
        calibration.validate()?;

        let key = calibration.key();

        if !self.gates.contains_key(&key)
            && self.gates.len() >= self.limits.max_gates
        {
            return Err(
                CalibrationError::GateLimitExceeded {
                    requested: self.gates.len() + 1,
                    maximum: self.limits.max_gates,
                },
            );
        }

        self.gates.insert(
            key,
            calibration,
        );

        Ok(())
    }

    /// Inserts readout calibration.
    pub fn insert_readout(
        &mut self,
        calibration: ReadoutCalibration,
    ) -> Result<(), CalibrationError> {
        calibration.validate()?;

        self.readout.insert(
            calibration.qubit,
            calibration,
        );

        Ok(())
    }

    /// Inserts crosstalk calibration.
    pub fn insert_crosstalk(
        &mut self,
        calibration: CrosstalkCalibration,
    ) -> Result<(), CalibrationError> {
        calibration.validate()?;

        self.crosstalk.insert(
            calibration.key(),
            calibration,
        );

        Ok(())
    }

    /// Inserts a snapshot-level metric.
    pub fn insert_metric(
        &mut self,
        metric: CalibrationMetric,
    ) -> Result<(), CalibrationError> {
        validate_metric(
            &metric,
            "snapshot.metric",
        )?;

        if !self.metrics.contains_key(
            &metric.metric_id,
        ) && self.metrics.len()
            >= self.limits.max_custom_metrics
        {
            return Err(
                CalibrationError::CustomMetricLimitExceeded {
                    requested: self.metrics.len() + 1,
                    maximum: self.limits.max_custom_metrics,
                },
            );
        }

        self.metrics.insert(
            metric.metric_id.clone(),
            metric,
        );

        Ok(())
    }

    /// Inserts bounded metadata.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), CalibrationError> {
        let key = key.into();
        let value = value.into();

        validate_metadata_key(&key)?;

        if value.len() > MAX_METADATA_VALUE_LENGTH {
            return Err(
                CalibrationError::MetadataValueTooLong {
                    key,
                    length: value.len(),
                    maximum: MAX_METADATA_VALUE_LENGTH,
                },
            );
        }

        if !self.metadata.contains_key(&key)
            && self.metadata.len()
                >= self.limits.max_metadata_fields
        {
            return Err(
                CalibrationError::MetadataLimitExceeded {
                    requested: self.metadata.len() + 1,
                    maximum: self.limits.max_metadata_fields,
                },
            );
        }

        self.metadata.insert(
            key,
            value,
        );

        Ok(())
    }

    /// Returns a qubit calibration.
    pub fn qubit(
        &self,
        qubit: usize,
    ) -> Option<&QubitCalibration> {
        self.qubits.get(&qubit)
    }

    /// Returns readout calibration.
    pub fn readout(
        &self,
        qubit: usize,
    ) -> Option<&ReadoutCalibration> {
        self.readout.get(&qubit)
    }

    /// Returns gate calibration for an exact physical gate instance.
    pub fn gate(
        &self,
        gate: &str,
        qubits: &[usize],
    ) -> Option<&GateCalibration> {
        let normalized = normalize_gate_name(gate).ok()?;

        self.gates.get(
            &gate_key(
                &normalized,
                qubits,
            ),
        )
    }

    /// Returns crosstalk calibration.
    pub fn crosstalk(
        &self,
        target_qubits: &[usize],
        spectator_qubits: &[usize],
    ) -> Option<&CrosstalkCalibration> {
        let key = format!(
            "target:{}|spectator:{}",
            qubit_list_key(target_qubits),
            qubit_list_key(spectator_qubits)
        );

        self.crosstalk.get(&key)
    }

    /// Returns a snapshot-level metric.
    pub fn metric(
        &self,
        metric_id: &str,
    ) -> Option<&CalibrationMetric> {
        self.metrics.get(metric_id)
    }

    /// Returns whether this snapshot contains any calibration evidence.
    pub fn is_empty(&self) -> bool {
        self.qubits.is_empty()
            && self.gates.is_empty()
            && self.readout.is_empty()
            && self.crosstalk.is_empty()
            && self.metrics.is_empty()
    }

    /// Returns the age relative to a supplied timestamp.
    pub fn age_ns(
        &self,
        now: CalibrationTimestamp,
    ) -> u64 {
        self.captured_at.age_since(now)
    }

    /// Returns whether the snapshot is stale under a supplied age policy.
    pub fn is_stale(
        &self,
        now: CalibrationTimestamp,
        maximum_age_ns: u64,
    ) -> bool {
        self.age_ns(now) > maximum_age_ns
    }

    /// Validates freshness and returns a structured error if stale.
    pub fn require_fresh(
        &self,
        now: CalibrationTimestamp,
        maximum_age_ns: u64,
    ) -> Result<(), CalibrationError> {
        let age_ns = self.age_ns(now);

        if age_ns > maximum_age_ns {
            return Err(
                CalibrationError::StaleCalibration {
                    age_ns,
                    maximum_age_ns,
                },
            );
        }

        Ok(())
    }

    /// Validates the complete snapshot.
    pub fn validate(&self) -> Result<(), CalibrationError> {
        if self.schema_id != CALIBRATION_SCHEMA_ID {
            return Err(
                CalibrationError::ConflictingCalibration {
                    resource: "schema_id".to_string(),
                    message: format!(
                        "expected `{CALIBRATION_SCHEMA_ID}`, got `{}`",
                        self.schema_id
                    ),
                },
            );
        }

        if self.schema_version
            != CALIBRATION_SCHEMA_VERSION
        {
            return Err(
                CalibrationError::ConflictingCalibration {
                    resource: "schema_version".to_string(),
                    message: format!(
                        "unsupported calibration schema version {}",
                        self.schema_version
                    ),
                },
            );
        }

        validate_backend_id(
            &self.backend_id,
        )?;

        if let Some(provider_id) = &self.provider_id {
            validate_provider_id(provider_id)?;
        }

        if let Some(processor_id) = &self.processor_id {
            validate_identifier(
                "processor_id",
                processor_id,
                MAX_SOURCE_ID_LENGTH,
            )?;
        }

        if let Some(calibration_id) =
            &self.calibration_id
        {
            validate_identifier(
                "calibration_id",
                calibration_id,
                MAX_SOURCE_ID_LENGTH,
            )?;
        }

        self.limits.validate()?;
        self.validity.validate()?;

        if self.is_empty() {
            return Err(
                CalibrationError::EmptySnapshot,
            );
        }

        if self.qubits.len() > self.limits.max_qubits {
            return Err(
                CalibrationError::QubitLimitExceeded {
                    requested: self.qubits.len(),
                    maximum: self.limits.max_qubits,
                },
            );
        }

        if self.gates.len() > self.limits.max_gates {
            return Err(
                CalibrationError::GateLimitExceeded {
                    requested: self.gates.len(),
                    maximum: self.limits.max_gates,
                },
            );
        }

        for calibration in self.qubits.values() {
            calibration.validate()?;
        }

        for calibration in self.gates.values() {
            calibration.validate()?;
        }

        for calibration in self.readout.values() {
            calibration.validate()?;
        }

        for calibration in self.crosstalk.values() {
            calibration.validate()?;
        }

        for metric in self.metrics.values() {
            validate_metric(
                metric,
                "snapshot.metric",
            )?;
        }

        for (key, value) in &self.metadata {
            validate_metadata_key(key)?;

            if value.len()
                > MAX_METADATA_VALUE_LENGTH
            {
                return Err(
                    CalibrationError::MetadataValueTooLong {
                        key: key.clone(),
                        length: value.len(),
                        maximum:
                            MAX_METADATA_VALUE_LENGTH,
                    },
                );
            }
        }

        Ok(())
    }

    /// Serializes the snapshot into deterministic JSON.
    pub fn to_canonical_json(
        &self,
    ) -> Result<String, CalibrationError> {
        self.validate()?;

        serde_json::to_string(self).map_err(
            |error| CalibrationError::Serialization {
                message: error.to_string(),
            },
        )
    }

    /// Calculates a SHA-256 fingerprint over canonical JSON.
    pub fn fingerprint(
        &self,
    ) -> Result<CalibrationFingerprint, CalibrationError> {
        let json = self.to_canonical_json()?;

        let mut hasher = Sha256::new();
        hasher.update(json.as_bytes());

        let digest = hasher.finalize();

        Ok(CalibrationFingerprint {
            algorithm: "sha256".to_string(),
            hex: hex::encode(digest),
        })
    }

    /// Returns a stable identity for provenance integration.
    pub fn identity(
        &self,
    ) -> Result<CalibrationIdentity, CalibrationError> {
        let fingerprint = self.fingerprint()?;

        Ok(CalibrationIdentity {
            schema_id: self.schema_id.clone(),
            schema_version: self.schema_version,
            backend_id: self.backend_id.clone(),
            provider_id: self.provider_id.clone(),
            processor_id: self.processor_id.clone(),
            calibration_id: self.calibration_id.clone(),
            captured_at: self.captured_at,
            fingerprint,
        })
    }
}

// ============================================================================
// Calibration identity / fingerprint
// ============================================================================

/// Stable fingerprint of a calibration snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalibrationFingerprint {
    /// Hash algorithm.
    pub algorithm: String,

    /// Lowercase hexadecimal digest.
    pub hex: String,
}

impl CalibrationFingerprint {
    /// Returns the digest string.
    pub fn as_str(&self) -> &str {
        &self.hex
    }
}

/// Compact calibration identity suitable for benchmark provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalibrationIdentity {
    /// Calibration schema identifier.
    pub schema_id: String,

    /// Calibration schema version.
    pub schema_version: u16,

    /// Backend identity.
    pub backend_id: String,

    /// Provider identity.
    pub provider_id: Option<String>,

    /// Processor identity.
    pub processor_id: Option<String>,

    /// Provider calibration ID.
    pub calibration_id: Option<String>,

    /// Calibration capture time.
    pub captured_at: CalibrationTimestamp,

    /// Snapshot fingerprint.
    pub fingerprint: CalibrationFingerprint,
}

// ============================================================================
// Builder
// ============================================================================

/// Builder for constructing validated calibration snapshots.
///
/// The builder exists to make provider adapters explicit and deterministic.
///
/// Once built, callers should treat `CalibrationSnapshot` as immutable
/// experimental context.
#[derive(Debug, Clone)]
pub struct CalibrationSnapshotBuilder {
    snapshot: CalibrationSnapshot,
}

impl CalibrationSnapshotBuilder {
    /// Starts a snapshot using the current timestamp.
    pub fn new(
        backend_id: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        Ok(Self {
            snapshot: CalibrationSnapshot::new(
                backend_id,
            )?,
        })
    }

    /// Starts a snapshot with an explicit timestamp.
    pub fn with_capture_time(
        backend_id: impl Into<String>,
        captured_at: CalibrationTimestamp,
    ) -> Result<Self, CalibrationError> {
        Ok(Self {
            snapshot:
                CalibrationSnapshot::with_capture_time(
                    backend_id,
                    captured_at,
                )?,
        })
    }

    /// Sets provider.
    pub fn provider_id(
        mut self,
        provider_id: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        self.snapshot =
            self.snapshot.with_provider_id(
                provider_id,
            )?;
        Ok(self)
    }

    /// Sets processor.
    pub fn processor_id(
        mut self,
        processor_id: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        self.snapshot =
            self.snapshot.with_processor_id(
                processor_id,
            )?;
        Ok(self)
    }

    /// Sets technology.
    pub fn technology(
        mut self,
        technology: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        self.snapshot =
            self.snapshot.with_technology(
                technology,
            )?;
        Ok(self)
    }

    /// Sets source.
    pub fn source(
        mut self,
        source: CalibrationSource,
    ) -> Self {
        self.snapshot =
            self.snapshot.with_source(source);
        self
    }

    /// Sets calibration ID.
    pub fn calibration_id(
        mut self,
        calibration_id: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        self.snapshot =
            self.snapshot.with_calibration_id(
                calibration_id,
            )?;
        Ok(self)
    }

    /// Sets validity.
    pub fn validity(
        mut self,
        validity: CalibrationValidity,
    ) -> Result<Self, CalibrationError> {
        self.snapshot =
            self.snapshot.with_validity(
                validity,
            )?;
        Ok(self)
    }

    /// Sets resource limits.
    pub fn limits(
        mut self,
        limits: CalibrationLimits,
    ) -> Result<Self, CalibrationError> {
        self.snapshot =
            self.snapshot.with_limits(
                limits,
            )?;
        Ok(self)
    }

    /// Adds a qubit.
    pub fn qubit(
        mut self,
        calibration: QubitCalibration,
    ) -> Result<Self, CalibrationError> {
        self.snapshot.insert_qubit(
            calibration,
        )?;
        Ok(self)
    }

    /// Adds a gate.
    pub fn gate(
        mut self,
        calibration: GateCalibration,
    ) -> Result<Self, CalibrationError> {
        self.snapshot.insert_gate(
            calibration,
        )?;
        Ok(self)
    }

    /// Adds readout calibration.
    pub fn readout(
        mut self,
        calibration: ReadoutCalibration,
    ) -> Result<Self, CalibrationError> {
        self.snapshot.insert_readout(
            calibration,
        )?;
        Ok(self)
    }

    /// Adds crosstalk calibration.
    pub fn crosstalk(
        mut self,
        calibration: CrosstalkCalibration,
    ) -> Result<Self, CalibrationError> {
        self.snapshot.insert_crosstalk(
            calibration,
        )?;
        Ok(self)
    }

    /// Adds a snapshot-level metric.
    pub fn metric(
        mut self,
        metric: CalibrationMetric,
    ) -> Result<Self, CalibrationError> {
        self.snapshot.insert_metric(
            metric,
        )?;
        Ok(self)
    }

    /// Adds metadata.
    pub fn metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        self.snapshot.insert_metadata(
            key,
            value,
        )?;
        Ok(self)
    }

    /// Finishes construction and performs complete validation.
    pub fn build(
        self,
    ) -> Result<CalibrationSnapshot, CalibrationError> {
        self.snapshot.validate()?;
        Ok(self.snapshot)
    }
}

// ============================================================================
// Helper functions
// ============================================================================

fn validate_backend_id(
    backend_id: &str,
) -> Result<String, CalibrationError> {
    validate_identifier(
        "backend_id",
        backend_id,
        MAX_BACKEND_ID_LENGTH,
    )
}

fn validate_provider_id(
    provider_id: &str,
) -> Result<String, CalibrationError> {
    validate_identifier(
        "provider_id",
        provider_id,
        MAX_PROVIDER_ID_LENGTH,
    )
}

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<String, CalibrationError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(
            CalibrationError::EmptyIdentifier {
                field,
            },
        );
    }

    if trimmed.len() > maximum {
        return Err(
            CalibrationError::IdentifierTooLong {
                field,
                length: trimmed.len(),
                maximum,
            },
        );
    }

    if trimmed.chars().any(char::is_control) {
        return Err(
            CalibrationError::InvalidIdentifier {
                field,
            },
        );
    }

    Ok(trimmed.to_string())
}

fn validate_qubit(
    qubit: usize,
) -> Result<(), CalibrationError> {
    if qubit == usize::MAX {
        return Err(
            CalibrationError::InvalidQubit {
                qubit,
            },
        );
    }

    Ok(())
}

fn normalize_gate_name(
    gate: &str,
) -> Result<String, CalibrationError> {
    let normalized = gate.trim().to_ascii_lowercase();

    if normalized.is_empty()
        || normalized.len() > MAX_SOURCE_ID_LENGTH
        || normalized.chars().any(char::is_control)
    {
        return Err(
            CalibrationError::InvalidGate {
                gate: gate.to_string(),
            },
        );
    }

    Ok(normalized)
}

fn normalize_metric_id(
    metric: &str,
) -> Result<String, CalibrationError> {
    let metric = metric.trim().to_ascii_lowercase();

    if metric.is_empty()
        || metric.len() > MAX_SOURCE_ID_LENGTH
        || metric.chars().any(char::is_control)
    {
        return Err(
            CalibrationError::InvalidMetricId {
                metric,
            },
        );
    }

    Ok(metric)
}

fn validate_gate_qubits(
    gate: &str,
    qubits: &[usize],
) -> Result<(), CalibrationError> {
    if qubits.is_empty() {
        return Err(
            CalibrationError::InvalidGate {
                gate: gate.to_string(),
            },
        );
    }

    if qubits.len() > MAX_GATE_QUBITS {
        return Err(
            CalibrationError::GateQubitLimitExceeded {
                gate: gate.to_string(),
                requested: qubits.len(),
                maximum: MAX_GATE_QUBITS,
            },
        );
    }

    let mut seen = BTreeSet::new();

    for &qubit in qubits {
        validate_qubit(qubit)?;

        if !seen.insert(qubit) {
            return Err(
                CalibrationError::DuplicateGateQubit {
                    gate: gate.to_string(),
                    qubit,
                },
            );
        }
    }

    Ok(())
}

fn validate_qubit_list(
    field: &'static str,
    qubits: &[usize],
) -> Result<(), CalibrationError> {
    if qubits.is_empty() {
        return Err(
            CalibrationError::InvalidIdentifier {
                field,
            },
        );
    }

    let mut seen = BTreeSet::new();

    for &qubit in qubits {
        validate_qubit(qubit)?;

        if !seen.insert(qubit) {
            return Err(
                CalibrationError::ConflictingCalibration {
                    resource: field.to_string(),
                    message: format!(
                        "duplicate qubit {qubit}"
                    ),
                },
            );
        }
    }

    Ok(())
}

fn target_and_spectator_overlap(
    target: &[usize],
    spectator: &[usize],
) -> bool {
    target
        .iter()
        .any(|qubit| spectator.contains(qubit))
}

fn gate_key(
    gate: &str,
    qubits: &[usize],
) -> String {
    format!(
        "{}:{}",
        gate,
        qubit_list_key(qubits)
    )
}

fn qubit_list_key(
    qubits: &[usize],
) -> String {
    qubits
        .iter()
        .map(|qubit| qubit.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn validate_probability(
    field: &str,
    value: f64,
) -> Result<(), CalibrationError> {
    if !value.is_finite()
        || !(0.0..=1.0).contains(&value)
    {
        return Err(
            CalibrationError::InvalidProbability {
                field: field.to_string(),
                value,
            },
        );
    }

    Ok(())
}

fn validate_non_negative_finite(
    field: &str,
    value: f64,
) -> Result<f64, CalibrationError> {
    if !value.is_finite() || value < 0.0 {
        return Err(
            CalibrationError::InvalidNumericValue {
                field: field.to_string(),
                value,
            },
        );
    }

    Ok(value)
}

fn validate_positive_finite(
    field: &str,
    value: f64,
) -> Result<f64, CalibrationError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(
            CalibrationError::InvalidNumericValue {
                field: field.to_string(),
                value,
            },
        );
    }

    Ok(value)
}

fn validate_confidence_level(
    value: f64,
) -> Result<(), CalibrationError> {
    if !value.is_finite()
        || value <= 0.0
        || value >= 1.0
    {
        return Err(
            CalibrationError::InvalidConfidenceLevel {
                value,
            },
        );
    }

    Ok(())
}

fn validate_metric(
    metric: &CalibrationMetric,
    field: &str,
) -> Result<(), CalibrationError> {
    let normalized =
        normalize_metric_id(&metric.metric_id)?;

    if normalized != metric.metric_id {
        return Err(
            CalibrationError::InvalidMetricId {
                metric: metric.metric_id.clone(),
            },
        );
    }

    if !metric.value.is_finite() {
        return Err(
            CalibrationError::InvalidNumericValue {
                field: field.to_string(),
                value: metric.value,
            },
        );
    }

    if metric.unit.trim().is_empty() {
        return Err(
            CalibrationError::InvalidIdentifier {
                field: "metric.unit",
            },
        );
    }

    metric.evidence.validate(field)?;

    Ok(())
}

fn validate_optional_probability_metric(
    metric: &Option<CalibrationMetric>,
    field: &str,
) -> Result<(), CalibrationError> {
    if let Some(metric) = metric {
        validate_metric(metric, field)?;
        validate_probability(
            field,
            metric.value,
        )?;
    }

    Ok(())
}

fn probability_metric(
    metric_id: &str,
    value: f64,
    direction: CalibrationMetricDirection,
    evidence: CalibrationEvidence,
    source: CalibrationSource,
) -> Result<CalibrationMetric, CalibrationError> {
    validate_probability(
        metric_id,
        value,
    )?;

    CalibrationMetric::new(
        metric_id,
        value,
        "probability",
        direction,
        source,
    )?
    .with_evidence(evidence)
}

fn validate_metadata_key(
    key: &str,
) -> Result<(), CalibrationError> {
    if key.trim().is_empty()
        || key.len() > MAX_METADATA_KEY_LENGTH
        || key.chars().any(char::is_control)
    {
        return Err(
            CalibrationError::InvalidMetadataKey {
                key: key.to_string(),
            },
        );
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence() -> CalibrationEvidence {
        CalibrationEvidence::measured(1_000)
            .expect("valid evidence")
    }

    #[test]
    fn empty_snapshot_is_rejected() {
        let snapshot =
            CalibrationSnapshot::new("test-backend")
                .expect("snapshot creation");

        assert_eq!(
            snapshot.validate(),
            Err(CalibrationError::EmptySnapshot)
        );
    }

    #[test]
    fn missing_values_are_not_encoded_as_zero() {
        let qubit =
            QubitCalibration::new(0)
                .expect("valid qubit");

        assert!(qubit.t1_ns.is_none());
        assert!(qubit.t2_ns.is_none());
        assert!(qubit.frequency_hz.is_none());
    }

    #[test]
    fn probability_is_validated() {
        let result =
            QubitCalibration::new(0)
                .expect("valid qubit")
                .with_reset_error(
                    1.1,
                    evidence(),
                    CalibrationSource::Measured,
                );

        assert!(matches!(
            result,
            Err(
                CalibrationError::InvalidProbability {
                    ..
                }
            )
        ));
    }

    #[test]
    fn gate_identity_is_deterministic() {
        let gate =
            GateCalibration::new(
                "CZ",
                vec![1, 4],
            )
            .expect("valid gate");

        assert_eq!(
            gate.key(),
            "cz:1,4"
        );
    }

    #[test]
    fn duplicate_gate_qubits_are_rejected() {
        let result =
            GateCalibration::new(
                "cz",
                vec![1, 1],
            );

        assert!(matches!(
            result,
            Err(
                CalibrationError::DuplicateGateQubit {
                    ..
                }
            )
        ));
    }

    #[test]
    fn readout_matrix_must_have_valid_rows() {
        let result =
            ReadoutCalibration::new(
                0,
                0.01,
                0.02,
                evidence(),
                CalibrationSource::Measured,
            )
            .expect("valid readout")
            .with_assignment_matrix([
                0.99,
                0.02,
                0.01,
                0.98,
            ]);

        assert!(result.is_err());
    }

    #[test]
    fn readout_fidelity_is_computed_correctly() {
        let readout =
            ReadoutCalibration::new(
                0,
                0.01,
                0.03,
                evidence(),
                CalibrationSource::Measured,
            )
            .expect("valid readout");

        assert!(
            (readout.average_error() - 0.02).abs()
                < 1.0e-12
        );

        assert!(
            (readout.average_fidelity() - 0.98).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn snapshot_can_be_fingerprinted() {
        let qubit =
            QubitCalibration::new(0)
                .expect("valid qubit")
                .with_t1_ns(
                    50_000.0,
                    evidence(),
                    CalibrationSource::Measured,
                )
                .expect("valid T1");

        let mut snapshot =
            CalibrationSnapshot::with_capture_time(
                "backend",
                CalibrationTimestamp::from_unix_nanos(
                    1_000_000,
                ),
            )
            .expect("snapshot");

        snapshot
            .insert_qubit(qubit)
            .expect("insert");

        let fingerprint =
            snapshot
                .fingerprint()
                .expect("fingerprint");

        assert_eq!(
            fingerprint.algorithm,
            "sha256"
        );

        assert_eq!(
            fingerprint.hex.len(),
            64
        );
    }

    #[test]
    fn fingerprint_is_deterministic() {
        let timestamp =
            CalibrationTimestamp::from_unix_nanos(
                123_456,
            );

        let mut first =
            CalibrationSnapshot::with_capture_time(
                "backend",
                timestamp,
            )
            .expect("snapshot");

        first
            .insert_qubit(
                QubitCalibration::new(0)
                    .expect("qubit"),
            )
            .expect("insert");

        let mut second =
            CalibrationSnapshot::with_capture_time(
                "backend",
                timestamp,
            )
            .expect("snapshot");

        second
            .insert_qubit(
                QubitCalibration::new(0)
                    .expect("qubit"),
            )
            .expect("insert");

        assert_eq!(
            first.fingerprint()
                .expect("fingerprint"),
            second.fingerprint()
                .expect("fingerprint")
        );
    }

    #[test]
    fn stale_detection_is_explicit() {
        let captured =
            CalibrationTimestamp::from_unix_nanos(
                1_000,
            );

        let current =
            CalibrationTimestamp::from_unix_nanos(
                2_000,
            );

        let mut snapshot =
            CalibrationSnapshot::with_capture_time(
                "backend",
                captured,
            )
            .expect("snapshot");

        snapshot
            .insert_qubit(
                QubitCalibration::new(0)
                    .expect("qubit"),
            )
            .expect("insert");

        assert!(
            snapshot.is_stale(
                current,
                500
            )
        );

        assert!(
            !snapshot.is_stale(
                current,
                2_000
            )
        );
    }

    #[test]
    fn crosstalk_target_and_spectator_must_be_disjoint() {
        let result =
            CrosstalkCalibration::new(
                vec![0, 1],
                vec![1, 2],
                0.01,
                evidence(),
                CalibrationSource::Measured,
            );

        assert!(result.is_err());
    }

    #[test]
    fn builder_produces_valid_snapshot() {
        let snapshot =
            CalibrationSnapshotBuilder::with_capture_time(
                "backend",
                CalibrationTimestamp::from_unix_nanos(
                    10,
                ),
            )
            .expect("builder")
            .provider_id("provider")
            .expect("provider")
            .processor_id("processor")
            .expect("processor")
            .technology("superconducting")
            .expect("technology")
            .source(CalibrationSource::Measured)
            .qubit(
                QubitCalibration::new(0)
                    .expect("qubit")
                    .with_t1_ns(
                        40_000.0,
                        evidence(),
                        CalibrationSource::Measured,
                    )
                    .expect("T1"),
            )
            .expect("insert qubit")
            .readout(
                ReadoutCalibration::new(
                    0,
                    0.01,
                    0.02,
                    evidence(),
                    CalibrationSource::Measured,
                )
                .expect("readout"),
            )
            .expect("insert readout")
            .gate(
                GateCalibration::new(
                    "cz",
                    vec![0, 1],
                )
                .expect("gate")
                .with_duration_ns(40)
                .expect("duration")
                .with_error_rate(
                    0.005,
                    evidence(),
                    CalibrationSource::Measured,
                )
                .expect("error"),
            )
            .expect("insert gate")
            .build()
            .expect("valid snapshot");

        assert_eq!(
            snapshot.backend_id,
            "backend"
        );

        assert_eq!(
            snapshot.qubits.len(),
            1
        );

        assert_eq!(
            snapshot.readout.len(),
            1
        );

        assert_eq!(
            snapshot.gates.len(),
            1
        );
    }
}