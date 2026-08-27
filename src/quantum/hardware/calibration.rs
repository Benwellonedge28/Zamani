//! Zamani Quantum — Hardware Calibration
//!
//! Production-grade, provider-independent calibration state for quantum
//! hardware, simulators, emulators, logical processors, and other quantum
//! execution targets.
//!
//! # Architectural responsibility
//!
//! This module is the authoritative representation of calibration STATE.
//!
//! It does NOT:
//!
//! - communicate with hardware;
//! - communicate with provider APIs;
//! - authenticate;
//! - read credentials;
//! - read environment variables;
//! - perform calibration experiments;
//! - execute quantum programs;
//! - perform routing;
//! - perform scheduling;
//! - perform transpilation;
//! - perform benchmarking;
//! - mutate Quantum IR;
//! - maintain global mutable state.
//!
//! Provider/device adapters obtain calibration measurements and construct
//! `CalibrationSnapshot` values. Consumers such as routing, scheduling,
//! compatibility checking, execution, benchmarking, reporting, and resource
//! estimation consume snapshots as immutable evidence.
//!
//! # Architectural position
//!
//! ```text
//! Provider / Device Adapter
//!          |
//!          | measured/calculated hardware properties
//!          v
//! CalibrationSnapshot
//!          |
//!     +----+---------+----------+-------------+
//!     |              |          |             |
//!     v              v          v             v
//! compatibility   routing   scheduling   benchmarking
//!     |              |          |             |
//!     +--------------+----------+-------------+
//!                            |
//!                            v
//!                         execution
//! ```
//!
//! # Important semantic rules
//!
//! 1. Missing data is represented by `Option<T>` or an absent map entry.
//! 2. Zero is never used as an "unknown" sentinel.
//! 3. Calibration is evidence observed at a point in time.
//! 4. Every empirical measurement may carry sample count, uncertainty,
//!    confidence, method, and provenance.
//! 5. Invalid floating-point values (`NaN`, positive infinity, negative
//!    infinity) are rejected.
//! 6. Probabilities must be in `[0, 1]`.
//! 7. Durations must be positive when a measured duration is supplied.
//! 8. Calibration snapshots are immutable from the consumer's perspective;
//!    construction is performed through validated mutation methods.
//! 9. Stale calibration must be explicitly checked before hardware decisions.
//! 10. Fingerprints identify calibration content; they are not signatures and
//!     do not authenticate the calibration provider.
//!
//! # Hardware coverage
//!
//! The representation intentionally supports:
//!
//! - superconducting qubits;
//! - trapped ions;
//! - neutral atoms;
//! - photonic systems;
//! - spin systems;
//! - other physical qubit technologies;
//! - gate-model processors;
//! - dynamic circuits;
//! - analog systems;
//! - annealers;
//! - logical/fault-tolerant systems;
//! - simulators and emulators;
//! - future heterogeneous quantum resources.
//!
//! IBM's current backend information includes T1, T2, readout error, gate
//! error, instruction duration, and readout duration. Amazon Braket likewise
//! exposes device topology, calibration data, and native gate information.
//! This module therefore treats those properties as first-class calibration
//! concepts rather than provider-specific fields.
//!
//! # Rust compatibility
//!
//! Supported target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//!
//! No nightly features are required.
//!
//! # Existing repository integration
//!
//! This file intentionally depends only on the Rust standard library and
//! dependencies already present in `Cargo.toml`:
//!
//! - `serde`
//! - `serde_json`
//! - `sha2`
//!
//! It does not depend on `backend.rs`, `topology.rs`, `capabilities.rs`,
//! `routing`, `scheduling`, or benchmarking. This makes it possible to freeze
//! this file before those modules are completed.
//!
//! Consumers should import calibration types from:
//!
//! `crate::quantum::hardware::calibration`
//!
//! The backend layer should treat `CalibrationSnapshot` as authoritative
//! calibration state rather than creating another calibration representation.
//!
//! Benchmarking may translate this representation into its own evidence model,
//! but benchmarking must not become the owner of hardware calibration state.
//!
//! # Serialization
//!
//! All public state types implement Serde serialization. Collections use
//! `BTreeMap`/`BTreeSet` where ordering matters, giving deterministic logical
//! ordering for persistence and fingerprinting.
//!
//! The canonical fingerprint uses SHA-256 over deterministic JSON. The
//! fingerprint is an integrity/fingerprint identifier only. It is not a
//! cryptographic signature.
//!
//! # Security
//!
//! Calibration metadata must never contain:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - authentication headers;
//! - secrets.
//!
//! Provider adapters are responsible for preventing secret leakage before
//! constructing a snapshot.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// ============================================================================
// Schema
// ============================================================================

/// Stable schema identifier.
pub const CALIBRATION_SCHEMA_ID: &str = "zamani.quantum.hardware.calibration";

/// Serialized schema version.
///
/// Increment this when serialized semantics change incompatibly.
pub const CALIBRATION_SCHEMA_VERSION: u16 = 1;

/// Maximum backend identifier length in UTF-8 bytes.
pub const MAX_BACKEND_ID_LENGTH: usize = 512;

/// Maximum provider identifier length in UTF-8 bytes.
pub const MAX_PROVIDER_ID_LENGTH: usize = 512;

/// Maximum device identifier length in UTF-8 bytes.
pub const MAX_DEVICE_ID_LENGTH: usize = 512;

/// Maximum calibration source identifier length.
pub const MAX_SOURCE_ID_LENGTH: usize = 512;

/// Maximum method identifier length.
pub const MAX_METHOD_LENGTH: usize = 512;

/// Maximum metadata key length.
pub const MAX_METADATA_KEY_LENGTH: usize = 512;

/// Maximum metadata value length.
pub const MAX_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum number of physical qubits represented by one snapshot.
pub const DEFAULT_MAX_QUBITS: usize = 1_000_000;

/// Maximum number of gate calibration records.
pub const DEFAULT_MAX_GATES: usize = 5_000_000;

/// Maximum number of coupling calibration records.
pub const DEFAULT_MAX_COUPLINGS: usize = 5_000_000;

/// Maximum number of metadata fields.
pub const DEFAULT_MAX_METADATA_FIELDS: usize = 4096;

/// Maximum number of custom metrics.
pub const DEFAULT_MAX_CUSTOM_METRICS: usize = 1_000_000;

/// Maximum qubits participating in one calibrated instruction.
pub const MAX_INSTRUCTION_QUBITS: usize = 32;

/// Maximum number of qubits in one coupling calibration.
pub const MAX_COUPLING_QUBITS: usize = 2;

/// Default stale-calibration policy.
///
/// This is deliberately conservative. Callers should select their own policy
/// based on the hardware technology and operation being performed.
pub const DEFAULT_MAX_CALIBRATION_AGE: Duration = Duration::from_secs(24 * 60 * 60);

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by calibration construction, validation, serialization,
/// and freshness checks.
#[derive(Debug, Clone, PartialEq)]
pub enum CalibrationError {
    EmptyIdentifier {
        field: &'static str,
    },

    IdentifierTooLong {
        field: &'static str,
        length: usize,
        maximum: usize,
    },

    InvalidIdentifier {
        field: &'static str,
    },

    InvalidQubit {
        qubit: usize,
    },

    QubitLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    InvalidInstruction {
        instruction: String,
    },

    InstructionQubitLimitExceeded {
        instruction: String,
        requested: usize,
        maximum: usize,
    },

    DuplicateInstructionQubit {
        instruction: String,
        qubit: usize,
    },

    InvalidProbability {
        field: String,
        value: f64,
    },

    InvalidNumericValue {
        field: String,
        value: f64,
    },

    InvalidDuration {
        field: String,
        value_ns: u64,
    },

    InvalidFrequency {
        field: String,
        value_hz: f64,
    },

    InvalidTimestamp {
        field: &'static str,
        value_ns: u64,
    },

    InvalidValidityInterval {
        valid_from_ns: u64,
        valid_until_ns: Option<u64>,
    },

    InvalidConfidenceLevel {
        value: f64,
    },

    InvalidUncertainty {
        field: String,
        value: f64,
    },

    InvalidSampleCount {
        field: String,
    },

    GateLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    CouplingLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    MetadataLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    InvalidMetadataKey {
        key: String,
    },

    MetadataValueTooLong {
        key: String,
        length: usize,
        maximum: usize,
    },

    CustomMetricLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    InvalidMetricId {
        metric: String,
    },

    EmptySnapshot,

    StaleCalibration {
        age_ns: u64,
        maximum_age_ns: u64,
    },

    CalibrationUnavailable {
        resource: String,
    },

    ConflictingCalibration {
        resource: String,
        message: String,
    },

    Serialization {
        message: String,
    },
}

impl fmt::Display for CalibrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(f, "{field} cannot be empty")
            }

            Self::IdentifierTooLong {
                field,
                length,
                maximum,
            } => {
                write!(
                    f,
                    "{field} is {length} bytes long; maximum is {maximum}"
                )
            }

            Self::InvalidIdentifier { field } => {
                write!(f, "invalid identifier for {field}")
            }

            Self::InvalidQubit { qubit } => {
                write!(f, "invalid physical qubit identifier {qubit}")
            }

            Self::QubitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "calibration contains {requested} qubits; maximum is {maximum}"
                )
            }

            Self::InvalidInstruction { instruction } => {
                write!(f, "invalid instruction identifier `{instruction}`")
            }

            Self::InstructionQubitLimitExceeded {
                instruction,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "instruction `{instruction}` references {requested} \
                     qubits; maximum is {maximum}"
                )
            }

            Self::DuplicateInstructionQubit { instruction, qubit } => {
                write!(
                    f,
                    "instruction `{instruction}` references qubit {qubit} \
                     more than once"
                )
            }

            Self::InvalidProbability { field, value } => {
                write!(
                    f,
                    "invalid probability for `{field}`: {value}"
                )
            }

            Self::InvalidNumericValue { field, value } => {
                write!(
                    f,
                    "invalid numeric calibration value for `{field}`: {value}"
                )
            }

            Self::InvalidDuration {
                field,
                value_ns,
            } => {
                write!(
                    f,
                    "invalid duration for `{field}`: {value_ns} ns"
                )
            }

            Self::InvalidFrequency { field, value_hz } => {
                write!(
                    f,
                    "invalid frequency for `{field}`: {value_hz} Hz"
                )
            }

            Self::InvalidTimestamp { field, value_ns } => {
                write!(
                    f,
                    "invalid timestamp for `{field}`: {value_ns} ns"
                )
            }

            Self::InvalidValidityInterval {
                valid_from_ns,
                valid_until_ns,
            } => {
                write!(
                    f,
                    "invalid calibration validity interval: \
                     from={valid_from_ns}, until={valid_until_ns:?}"
                )
            }

            Self::InvalidConfidenceLevel { value } => {
                write!(
                    f,
                    "confidence level must be finite and in (0, 1), got {value}"
                )
            }

            Self::InvalidUncertainty { field, value } => {
                write!(
                    f,
                    "uncertainty for `{field}` must be finite and \
                     non-negative, got {value}"
                )
            }

            Self::InvalidSampleCount { field } => {
                write!(
                    f,
                    "sample count for `{field}` must be greater than zero"
                )
            }

            Self::GateLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "calibration contains {requested} instruction records; \
                     maximum is {maximum}"
                )
            }

            Self::CouplingLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "calibration contains {requested} coupling records; \
                     maximum is {maximum}"
                )
            }

            Self::MetadataLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "calibration contains {requested} metadata fields; \
                     maximum is {maximum}"
                )
            }

            Self::InvalidMetadataKey { key } => {
                write!(f, "invalid calibration metadata key `{key}`")
            }

            Self::MetadataValueTooLong {
                key,
                length,
                maximum,
            } => {
                write!(
                    f,
                    "metadata value `{key}` is {length} bytes; \
                     maximum is {maximum}"
                )
            }

            Self::CustomMetricLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "calibration contains {requested} custom metrics; \
                     maximum is {maximum}"
                )
            }

            Self::InvalidMetricId { metric } => {
                write!(
                    f,
                    "invalid calibration metric identifier `{metric}`"
                )
            }

            Self::EmptySnapshot => {
                f.write_str(
                    "calibration snapshot contains no calibration evidence",
                )
            }

            Self::StaleCalibration {
                age_ns,
                maximum_age_ns,
            } => {
                write!(
                    f,
                    "calibration is stale: age={age_ns} ns; \
                     maximum allowed age={maximum_age_ns} ns"
                )
            }

            Self::CalibrationUnavailable { resource } => {
                write!(
                    f,
                    "calibration unavailable for `{resource}`"
                )
            }

            Self::ConflictingCalibration {
                resource,
                message,
            } => {
                write!(
                    f,
                    "conflicting calibration for `{resource}`: {message}"
                )
            }

            Self::Serialization { message } => {
                write!(
                    f,
                    "calibration serialization error: {message}"
                )
            }
        }
    }
}

impl std::error::Error for CalibrationError {}

// ============================================================================
// Timestamp
// ============================================================================

/// Unix timestamp in nanoseconds.
///
/// A `u64` is sufficient for contemporary Unix nanosecond timestamps while
/// keeping serialization compact and interoperable.
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
#[serde(transparent)]
pub struct CalibrationTimestamp {
    unix_ns: u64,
}

impl CalibrationTimestamp {
    /// Construct a timestamp from Unix nanoseconds.
    pub const fn from_unix_nanos(unix_ns: u64) -> Self {
        Self { unix_ns }
    }

    /// Return Unix nanoseconds.
    pub const fn as_unix_nanos(self) -> u64 {
        self.unix_ns
    }

    /// Return the current system timestamp.
    ///
    /// If the system clock is before the Unix epoch, the timestamp is zero.
    pub fn now() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| {
                duration
                    .as_nanos()
                    .min(u64::MAX as u128) as u64
            })
            .unwrap_or(0);

        Self { unix_ns: nanos }
    }

    /// Calculate age relative to the current system clock.
    ///
    /// Clock rollback is handled conservatively by returning zero.
    pub fn age(self) -> Duration {
        let now = Self::now().unix_ns;

        if now <= self.unix_ns {
            Duration::ZERO
        } else {
            Duration::from_nanos(now - self.unix_ns)
        }
    }

    /// Return whether the timestamp is older than the supplied duration.
    pub fn is_older_than(self, maximum_age: Duration) -> bool {
        self.age() > maximum_age
    }
}

impl Default for CalibrationTimestamp {
    fn default() -> Self {
        Self::now()
    }
}

// ============================================================================
// Calibration provenance
// ============================================================================

/// Source classification for calibration evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CalibrationSourceKind {
    /// Directly measured by the physical device/provider.
    Measured,

    /// Supplied as authoritative backend/device metadata.
    DeviceMetadata,

    /// Calculated from other measured quantities.
    Derived,

    /// Estimated from a model.
    Estimated,

    /// Supplied by a simulator/emulator configuration.
    Simulated,

    /// Supplied by a user/application rather than a provider.
    UserProvided,

    /// Imported from an external calibration artifact.
    Imported,
}

/// Provenance attached to an individual calibration observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalibrationProvenance {
    /// Classification of the source.
    pub source_kind: CalibrationSourceKind,

    /// Optional provider identifier.
    pub provider_id: Option<String>,

    /// Optional device identifier.
    pub device_id: Option<String>,

    /// Optional source record identifier.
    pub source_id: Option<String>,

    /// Optional method/experiment name.
    pub method: Option<String>,

    /// Optional provider API/schema version.
    pub source_version: Option<String>,
}

impl Default for CalibrationProvenance {
    fn default() -> Self {
        Self {
            source_kind: CalibrationSourceKind::Measured,
            provider_id: None,
            device_id: None,
            source_id: None,
            method: None,
            source_version: None,
        }
    }
}

impl CalibrationProvenance {
    /// Construct measured provenance.
    pub fn measured() -> Self {
        Self::default()
    }

    /// Construct simulated provenance.
    pub fn simulated() -> Self {
        Self {
            source_kind: CalibrationSourceKind::Simulated,
            ..Self::default()
        }
    }

    /// Set provider identity.
    pub fn with_provider(
        mut self,
        provider_id: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        self.provider_id = Some(validate_identifier(
            "provider_id",
            &provider_id.into(),
            MAX_PROVIDER_ID_LENGTH,
        )?);

        Ok(self)
    }

    /// Set device identity.
    pub fn with_device(
        mut self,
        device_id: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        self.device_id = Some(validate_identifier(
            "device_id",
            &device_id.into(),
            MAX_DEVICE_ID_LENGTH,
        )?);

        Ok(self)
    }

    /// Set source record identity.
    pub fn with_source_id(
        mut self,
        source_id: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        self.source_id = Some(validate_identifier(
            "source_id",
            &source_id.into(),
            MAX_SOURCE_ID_LENGTH,
        )?);

        Ok(self)
    }

    /// Set calibration method.
    pub fn with_method(
        mut self,
        method: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        let value = method.into();

        if value.trim().is_empty() {
            return Err(CalibrationError::EmptyIdentifier {
                field: "method",
            });
        }

        if value.len() > MAX_METHOD_LENGTH {
            return Err(CalibrationError::IdentifierTooLong {
                field: "method",
                length: value.len(),
                maximum: MAX_METHOD_LENGTH,
            });
        }

        self.method = Some(value.trim().to_string());

        Ok(self)
    }

    /// Set provider/source version.
    pub fn with_source_version(
        mut self,
        version: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        let value = version.into();

        if value.trim().is_empty() {
            return Err(CalibrationError::EmptyIdentifier {
                field: "source_version",
            });
        }

        self.source_version = Some(value.trim().to_string());

        Ok(self)
    }

    fn validate(&self) -> Result<(), CalibrationError> {
        if let Some(value) = &self.provider_id {
            validate_identifier(
                "provider_id",
                value,
                MAX_PROVIDER_ID_LENGTH,
            )?;
        }

        if let Some(value) = &self.device_id {
            validate_identifier(
                "device_id",
                value,
                MAX_DEVICE_ID_LENGTH,
            )?;
        }

        if let Some(value) = &self.source_id {
            validate_identifier(
                "source_id",
                value,
                MAX_SOURCE_ID_LENGTH,
            )?;
        }

        if let Some(method) = &self.method {
            if method.trim().is_empty() {
                return Err(CalibrationError::EmptyIdentifier {
                    field: "method",
                });
            }

            if method.len() > MAX_METHOD_LENGTH {
                return Err(CalibrationError::IdentifierTooLong {
                    field: "method",
                    length: method.len(),
                    maximum: MAX_METHOD_LENGTH,
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// Measurement evidence
// ============================================================================

/// Statistical evidence attached to an empirical calibration value.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MeasurementEvidence {
    /// Number of observations/shots/experiments used.
    pub samples: u64,

    /// Optional standard error or equivalent uncertainty in the same units
    /// as the measured quantity.
    pub uncertainty: Option<f64>,

    /// Optional confidence level, expressed as a fraction in `(0, 1)`.
    pub confidence_level: Option<f64>,
}

impl MeasurementEvidence {
    /// Construct evidence from a positive sample count.
    pub fn new(samples: u64) -> Result<Self, CalibrationError> {
        if samples == 0 {
            return Err(CalibrationError::InvalidSampleCount {
                field: "samples".to_string(),
            });
        }

        Ok(Self {
            samples,
            uncertainty: None,
            confidence_level: None,
        })
    }

    /// Set uncertainty.
    pub fn with_uncertainty(
        mut self,
        uncertainty: f64,
    ) -> Result<Self, CalibrationError> {
        validate_non_negative_finite(
            "measurement.uncertainty",
            uncertainty,
        )?;

        self.uncertainty = Some(uncertainty);

        Ok(self)
    }

    /// Set confidence level.
    pub fn with_confidence_level(
        mut self,
        confidence_level: f64,
    ) -> Result<Self, CalibrationError> {
        validate_confidence(confidence_level)?;

        self.confidence_level = Some(confidence_level);

        Ok(self)
    }

    /// Validate evidence.
    pub fn validate(
        &self,
        field_prefix: &str,
    ) -> Result<(), CalibrationError> {
        if self.samples == 0 {
            return Err(CalibrationError::InvalidSampleCount {
                field: field_prefix.to_string(),
            });
        }

        if let Some(value) = self.uncertainty {
            validate_non_negative_finite(
                &format!("{field_prefix}.uncertainty"),
                value,
            )?;
        }

        if let Some(value) = self.confidence_level {
            validate_confidence(value)?;
        }

        Ok(())
    }
}

// ============================================================================
// Qubit calibration
// ============================================================================

/// Physical-qubit calibration state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QubitCalibration {
    /// Physical qubit identifier.
    pub qubit: usize,

    /// T1 relaxation time in nanoseconds.
    pub t1_ns: Option<f64>,

    /// T2/dephasing time in nanoseconds.
    pub t2_ns: Option<f64>,

    /// T2-star time in nanoseconds.
    pub t2_star_ns: Option<f64>,

    /// Qubit transition/resonance frequency in Hz.
    pub frequency_hz: Option<f64>,

    /// Anharmonicity in Hz, when applicable.
    pub anharmonicity_hz: Option<f64>,

    /// Thermal population of the excited state.
    pub thermal_population: Option<f64>,

    /// Leakage probability.
    pub leakage_rate: Option<f64>,

    /// Reset error probability.
    pub reset_error: Option<f64>,

    /// Readout calibration.
    pub readout: Option<ReadoutCalibration>,

    /// Optional measurement evidence for the qubit record.
    pub evidence: Option<MeasurementEvidence>,

    /// Provenance.
    pub provenance: CalibrationProvenance,
}

impl QubitCalibration {
    /// Construct a new qubit calibration.
    pub fn new(qubit: usize) -> Result<Self, CalibrationError> {
        validate_qubit_id(qubit)?;

        Ok(Self {
            qubit,
            t1_ns: None,
            t2_ns: None,
            t2_star_ns: None,
            frequency_hz: None,
            anharmonicity_hz: None,
            thermal_population: None,
            leakage_rate: None,
            reset_error: None,
            readout: None,
            evidence: None,
            provenance: CalibrationProvenance::default(),
        })
    }

    pub fn with_t1_ns(
        mut self,
        value: f64,
    ) -> Result<Self, CalibrationError> {
        validate_coherence("t1_ns", value)?;
        self.t1_ns = Some(value);
        Ok(self)
    }

    pub fn with_t2_ns(
        mut self,
        value: f64,
    ) -> Result<Self, CalibrationError> {
        validate_coherence("t2_ns", value)?;
        self.t2_ns = Some(value);
        Ok(self)
    }

    pub fn with_t2_star_ns(
        mut self,
        value: f64,
    ) -> Result<Self, CalibrationError> {
        validate_coherence("t2_star_ns", value)?;
        self.t2_star_ns = Some(value);
        Ok(self)
    }

    pub fn with_frequency_hz(
        mut self,
        value: f64,
    ) -> Result<Self, CalibrationError> {
        validate_frequency("frequency_hz", value)?;
        self.frequency_hz = Some(value);
        Ok(self)
    }

    pub fn with_anharmonicity_hz(
        mut self,
        value: f64,
    ) -> Result<Self, CalibrationError> {
        if !value.is_finite() {
            return Err(CalibrationError::InvalidFrequency {
                field: "anharmonicity_hz".to_string(),
                value_hz: value,
            });
        }

        self.anharmonicity_hz = Some(value);
        Ok(self)
    }

    pub fn with_thermal_population(
        mut self,
        value: f64,
    ) -> Result<Self, CalibrationError> {
        validate_probability("thermal_population", value)?;
        self.thermal_population = Some(value);
        Ok(self)
    }

    pub fn with_leakage_rate(
        mut self,
        value: f64,
    ) -> Result<Self, CalibrationError> {
        validate_probability("leakage_rate", value)?;
        self.leakage_rate = Some(value);
        Ok(self)
    }

    pub fn with_reset_error(
        mut self,
        value: f64,
    ) -> Result<Self, CalibrationError> {
        validate_probability("reset_error", value)?;
        self.reset_error = Some(value);
        Ok(self)
    }

    pub fn with_readout(
        mut self,
        readout: ReadoutCalibration,
    ) -> Self {
        self.readout = Some(readout);
        self
    }

    pub fn with_evidence(
        mut self,
        evidence: MeasurementEvidence,
    ) -> Result<Self, CalibrationError> {
        evidence.validate("qubit")?;
        self.evidence = Some(evidence);
        Ok(self)
    }

    pub fn with_provenance(
        mut self,
        provenance: CalibrationProvenance,
    ) -> Result<Self, CalibrationError> {
        provenance.validate()?;
        self.provenance = provenance;
        Ok(self)
    }

    /// Conservative coherence estimate.
    pub fn effective_coherence_ns(&self) -> Option<f64> {
        [
            self.t1_ns,
            self.t2_ns,
            self.t2_star_ns,
        ]
        .into_iter()
        .flatten()
        .reduce(f64::min)
    }

    /// Whether at least one coherence measurement exists.
    pub fn has_coherence_data(&self) -> bool {
        self.t1_ns.is_some()
            || self.t2_ns.is_some()
            || self.t2_star_ns.is_some()
    }

    fn validate(&self) -> Result<(), CalibrationError> {
        validate_qubit_id(self.qubit)?;

        if let Some(value) = self.t1_ns {
            validate_coherence("t1_ns", value)?;
        }

        if let Some(value) = self.t2_ns {
            validate_coherence("t2_ns", value)?;
        }

        if let Some(value) = self.t2_star_ns {
            validate_coherence("t2_star_ns", value)?;
        }

        if let Some(value) = self.frequency_hz {
            validate_frequency("frequency_hz", value)?;
        }

        if let Some(value) = self.anharmonicity_hz {
            if !value.is_finite() {
                return Err(CalibrationError::InvalidFrequency {
                    field: "anharmonicity_hz".to_string(),
                    value_hz: value,
                });
            }
        }

        if let Some(value) = self.thermal_population {
            validate_probability("thermal_population", value)?;
        }

        if let Some(value) = self.leakage_rate {
            validate_probability("leakage_rate", value)?;
        }

        if let Some(value) = self.reset_error {
            validate_probability("reset_error", value)?;
        }

        if let Some(readout) = &self.readout {
            readout.validate()?;
        }

        if let Some(evidence) = &self.evidence {
            evidence.validate("qubit")?;
        }

        self.provenance.validate()?;

        Ok(())
    }
}

// ============================================================================
// Readout calibration
// ============================================================================

/// Readout/measurement calibration for one physical qubit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReadoutCalibration {
    /// P(measure 1 | prepared 0).
    pub p01: f64,

    /// P(measure 0 | prepared 1).
    pub p10: f64,

    /// Optional measurement duration in nanoseconds.
    pub duration_ns: Option<u64>,

    /// Optional assignment-fidelity estimate.
    pub fidelity: Option<f64>,

    /// Statistical evidence.
    pub evidence: Option<MeasurementEvidence>,

    /// Provenance.
    pub provenance: CalibrationProvenance,
}

impl ReadoutCalibration {
    /// Construct readout calibration.
    pub fn new(
        p01: f64,
        p10: f64,
    ) -> Result<Self, CalibrationError> {
        validate_probability("readout.p01", p01)?;
        validate_probability("readout.p10", p10)?;

        Ok(Self {
            p01,
            p10,
            duration_ns: None,
            fidelity: None,
            evidence: None,
            provenance: CalibrationProvenance::default(),
        })
    }

    pub fn with_duration_ns(
        mut self,
        duration_ns: u64,
    ) -> Result<Self, CalibrationError> {
        validate_positive_duration(
            "readout.duration_ns",
            duration_ns,
        )?;

        self.duration_ns = Some(duration_ns);

        Ok(self)
    }

    pub fn with_fidelity(
        mut self,
        fidelity: f64,
    ) -> Result<Self, CalibrationError> {
        validate_probability("readout.fidelity", fidelity)?;
        self.fidelity = Some(fidelity);
        Ok(self)
    }

    pub fn with_evidence(
        mut self,
        evidence: MeasurementEvidence,
    ) -> Result<Self, CalibrationError> {
        evidence.validate("readout")?;
        self.evidence = Some(evidence);
        Ok(self)
    }

    pub fn with_provenance(
        mut self,
        provenance: CalibrationProvenance,
    ) -> Result<Self, CalibrationError> {
        provenance.validate()?;
        self.provenance = provenance;
        Ok(self)
    }

    /// Average assignment error.
    pub fn average_error(&self) -> f64 {
        (self.p01 + self.p10) / 2.0
    }

    /// Average assignment fidelity.
    pub fn average_fidelity(&self) -> f64 {
        1.0 - self.average_error()
    }

    /// Whether empirical evidence is attached.
    pub fn is_measured(&self) -> bool {
        self.evidence
            .as_ref()
            .map(|evidence| evidence.samples > 0)
            .unwrap_or(false)
    }

    fn validate(&self) -> Result<(), CalibrationError> {
        validate_probability("readout.p01", self.p01)?;
        validate_probability("readout.p10", self.p10)?;

        if let Some(duration_ns) = self.duration_ns {
            validate_positive_duration(
                "readout.duration_ns",
                duration_ns,
            )?;
        }

        if let Some(fidelity) = self.fidelity {
            validate_probability("readout.fidelity", fidelity)?;
        }

        if let Some(evidence) = &self.evidence {
            evidence.validate("readout")?;
        }

        self.provenance.validate()?;

        Ok(())
    }
}

// ============================================================================
// Instruction/gate calibration
// ============================================================================

/// Calibration of a physical instruction on a particular set of qubits.
///
/// This intentionally covers more than named gates. It can represent
/// measurement, reset, delay, pulse, native gates, controlled operations, or
/// other provider instructions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstructionCalibration {
    /// Canonical instruction name.
    pub instruction: String,

    /// Physical operands in provider/canonical operand order.
    pub qubits: Vec<usize>,

    /// Execution duration in nanoseconds.
    pub duration_ns: Option<u64>,

    /// Estimated operation error probability.
    pub error_rate: Option<f64>,

    /// Optional process fidelity.
    pub fidelity: Option<f64>,

    /// Optional amplitude parameter.
    pub amplitude: Option<f64>,

    /// Optional phase in radians.
    pub phase_radians: Option<f64>,

    /// Optional frequency in Hz.
    pub frequency_hz: Option<f64>,

    /// Whether this calibrated instruction is currently operational.
    pub operational: bool,

    /// Statistical evidence.
    pub evidence: Option<MeasurementEvidence>,

    /// Provenance.
    pub provenance: CalibrationProvenance,
}

impl InstructionCalibration {
    /// Construct an instruction calibration.
    pub fn new(
        instruction: impl Into<String>,
        qubits: Vec<usize>,
    ) -> Result<Self, CalibrationError> {
        let instruction = normalize_instruction_name(
            &instruction.into(),
        )?;

        validate_instruction_qubits(
            &instruction,
            &qubits,
        )?;

        Ok(Self {
            instruction,
            qubits,
            duration_ns: None,
            error_rate: None,
            fidelity: None,
            amplitude: None,
            phase_radians: None,
            frequency_hz: None,
            operational: true,
            evidence: None,
            provenance: CalibrationProvenance::default(),
        })
    }

    pub fn with_duration_ns(
        mut self,
        duration_ns: u64,
    ) -> Result<Self, CalibrationError> {
        validate_positive_duration(
            "instruction.duration_ns",
            duration_ns,
        )?;

        self.duration_ns = Some(duration_ns);

        Ok(self)
    }

    pub fn with_error_rate(
        mut self,
        error_rate: f64,
    ) -> Result<Self, CalibrationError> {
        validate_probability(
            "instruction.error_rate",
            error_rate,
        )?;

        self.error_rate = Some(error_rate);

        Ok(self)
    }

    pub fn with_fidelity(
        mut self,
        fidelity: f64,
    ) -> Result<Self, CalibrationError> {
        validate_probability(
            "instruction.fidelity",
            fidelity,
        )?;

        self.fidelity = Some(fidelity);

        Ok(self)
    }

    pub fn with_amplitude(
        mut self,
        amplitude: f64,
    ) -> Result<Self, CalibrationError> {
        validate_finite(
            "instruction.amplitude",
            amplitude,
        )?;

        self.amplitude = Some(amplitude);

        Ok(self)
    }

    pub fn with_phase_radians(
        mut self,
        phase_radians: f64,
    ) -> Result<Self, CalibrationError> {
        validate_finite(
            "instruction.phase_radians",
            phase_radians,
        )?;

        self.phase_radians = Some(phase_radians);

        Ok(self)
    }

    pub fn with_frequency_hz(
        mut self,
        frequency_hz: f64,
    ) -> Result<Self, CalibrationError> {
        validate_frequency(
            "instruction.frequency_hz",
            frequency_hz,
        )?;

        self.frequency_hz = Some(frequency_hz);

        Ok(self)
    }

    pub fn with_operational(
        mut self,
        operational: bool,
    ) -> Self {
        self.operational = operational;
        self
    }

    pub fn with_evidence(
        mut self,
        evidence: MeasurementEvidence,
    ) -> Result<Self, CalibrationError> {
        evidence.validate("instruction")?;
        self.evidence = Some(evidence);
        Ok(self)
    }

    pub fn with_provenance(
        mut self,
        provenance: CalibrationProvenance,
    ) -> Result<Self, CalibrationError> {
        provenance.validate()?;
        self.provenance = provenance;
        Ok(self)
    }

    /// Whether this instruction can currently be used from its calibration
    /// perspective.
    pub fn is_usable(&self) -> bool {
        self.operational
    }

    fn key(&self) -> InstructionKey {
        InstructionKey {
            instruction: self.instruction.clone(),
            qubits: self.qubits.clone(),
        }
    }

    fn validate(&self) -> Result<(), CalibrationError> {
        validate_instruction_qubits(
            &self.instruction,
            &self.qubits,
        )?;

        if let Some(duration_ns) = self.duration_ns {
            validate_positive_duration(
                "instruction.duration_ns",
                duration_ns,
            )?;
        }

        if let Some(error_rate) = self.error_rate {
            validate_probability(
                "instruction.error_rate",
                error_rate,
            )?;
        }

        if let Some(fidelity) = self.fidelity {
            validate_probability(
                "instruction.fidelity",
                fidelity,
            )?;
        }

        if let Some(amplitude) = self.amplitude {
            validate_finite(
                "instruction.amplitude",
                amplitude,
            )?;
        }

        if let Some(phase) = self.phase_radians {
            validate_finite(
                "instruction.phase_radians",
                phase,
            )?;
        }

        if let Some(frequency_hz) = self.frequency_hz {
            validate_frequency(
                "instruction.frequency_hz",
                frequency_hz,
            )?;
        }

        if let Some(evidence) = &self.evidence {
            evidence.validate("instruction")?;
        }

        self.provenance.validate()?;

        Ok(())
    }
}

/// Deterministic instruction lookup key.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
struct InstructionKey {
    instruction: String,
    qubits: Vec<usize>,
}

// ============================================================================
// Coupling calibration
// ============================================================================

/// Calibration of an interaction between two physical qubits.
///
/// Direction is represented by the order of `qubits`. Thus `(0, 1)` and
/// `(1, 0)` are distinct records when the provider exposes directed
/// calibration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CouplingCalibration {
    /// Physical qubits in operation order.
    pub qubits: Vec<usize>,

    /// Optional coupling/interconnect fidelity.
    pub fidelity: Option<f64>,

    /// Optional coupling error probability.
    pub error_rate: Option<f64>,

    /// Optional interaction duration in nanoseconds.
    pub duration_ns: Option<u64>,

    /// Optional crosstalk contribution.
    pub crosstalk_rate: Option<f64>,

    /// Whether the coupling is currently operational.
    pub operational: bool,

    /// Statistical evidence.
    pub evidence: Option<MeasurementEvidence>,

    /// Provenance.
    pub provenance: CalibrationProvenance,
}

impl CouplingCalibration {
    /// Construct a two-qubit coupling calibration.
    pub fn new(
        control: usize,
        target: usize,
    ) -> Result<Self, CalibrationError> {
        validate_qubit_id(control)?;
        validate_qubit_id(target)?;

        if control == target {
            return Err(CalibrationError::ConflictingCalibration {
                resource: format!("{control}->{target}"),
                message: "coupling endpoints must be different"
                    .to_string(),
            });
        }

        Ok(Self {
            qubits: vec![control, target],
            fidelity: None,
            error_rate: None,
            duration_ns: None,
            crosstalk_rate: None,
            operational: true,
            evidence: None,
            provenance: CalibrationProvenance::default(),
        })
    }

    pub fn with_fidelity(
        mut self,
        fidelity: f64,
    ) -> Result<Self, CalibrationError> {
        validate_probability(
            "coupling.fidelity",
            fidelity,
        )?;

        self.fidelity = Some(fidelity);

        Ok(self)
    }

    pub fn with_error_rate(
        mut self,
        error_rate: f64,
    ) -> Result<Self, CalibrationError> {
        validate_probability(
            "coupling.error_rate",
            error_rate,
        )?;

        self.error_rate = Some(error_rate);

        Ok(self)
    }

    pub fn with_duration_ns(
        mut self,
        duration_ns: u64,
    ) -> Result<Self, CalibrationError> {
        validate_positive_duration(
            "coupling.duration_ns",
            duration_ns,
        )?;

        self.duration_ns = Some(duration_ns);

        Ok(self)
    }

    pub fn with_crosstalk_rate(
        mut self,
        crosstalk_rate: f64,
    ) -> Result<Self, CalibrationError> {
        validate_probability(
            "coupling.crosstalk_rate",
            crosstalk_rate,
        )?;

        self.crosstalk_rate = Some(crosstalk_rate);

        Ok(self)
    }

    pub fn with_operational(
        mut self,
        operational: bool,
    ) -> Self {
        self.operational = operational;
        self
    }

    pub fn with_evidence(
        mut self,
        evidence: MeasurementEvidence,
    ) -> Result<Self, CalibrationError> {
        evidence.validate("coupling")?;
        self.evidence = Some(evidence);
        Ok(self)
    }

    pub fn with_provenance(
        mut self,
        provenance: CalibrationProvenance,
    ) -> Result<Self, CalibrationError> {
        provenance.validate()?;
        self.provenance = provenance;
        Ok(self)
    }

    fn key(&self) -> CouplingKey {
        CouplingKey {
            source: self.qubits[0],
            target: self.qubits[1],
        }
    }

    fn validate(&self) -> Result<(), CalibrationError> {
        if self.qubits.len() != MAX_COUPLING_QUBITS {
            return Err(CalibrationError::ConflictingCalibration {
                resource: "coupling".to_string(),
                message: "coupling calibration must contain exactly \
                          two distinct qubits"
                    .to_string(),
            });
        }

        validate_qubit_id(self.qubits[0])?;
        validate_qubit_id(self.qubits[1])?;

        if self.qubits[0] == self.qubits[1] {
            return Err(CalibrationError::ConflictingCalibration {
                resource: "coupling".to_string(),
                message: "coupling endpoints must be distinct"
                    .to_string(),
            });
        }

        if let Some(fidelity) = self.fidelity {
            validate_probability(
                "coupling.fidelity",
                fidelity,
            )?;
        }

        if let Some(error_rate) = self.error_rate {
            validate_probability(
                "coupling.error_rate",
                error_rate,
            )?;
        }

        if let Some(duration_ns) = self.duration_ns {
            validate_positive_duration(
                "coupling.duration_ns",
                duration_ns,
            )?;
        }

        if let Some(crosstalk_rate) = self.crosstalk_rate {
            validate_probability(
                "coupling.crosstalk_rate",
                crosstalk_rate,
            )?;
        }

        if let Some(evidence) = &self.evidence {
            evidence.validate("coupling")?;
        }

        self.provenance.validate()?;

        Ok(())
    }
}

/// Deterministic directed coupling lookup key.
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
struct CouplingKey {
    source: usize,
    target: usize,
}

// ============================================================================
// Custom calibration metrics
// ============================================================================

/// Provider-independent extension point for calibration quantities that do
/// not yet have a dedicated Zamani field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomCalibrationMetric {
    /// Stable metric identifier.
    pub metric_id: String,

    /// Numeric value.
    pub value: f64,

    /// Optional unit, e.g. `"Hz"`, `"ns"`, `"probability"`.
    pub unit: Option<String>,

    /// Optional physical qubit operands.
    pub qubits: Vec<usize>,

    /// Optional statistical evidence.
    pub evidence: Option<MeasurementEvidence>,

    /// Provenance.
    pub provenance: CalibrationProvenance,
}

impl CustomCalibrationMetric {
    /// Construct a custom metric.
    pub fn new(
        metric_id: impl Into<String>,
        value: f64,
    ) -> Result<Self, CalibrationError> {
        let metric_id = normalize_metric_id(
            &metric_id.into(),
        )?;

        validate_finite(
            "custom_metric.value",
            value,
        )?;

        Ok(Self {
            metric_id,
            value,
            unit: None,
            qubits: Vec::new(),
            evidence: None,
            provenance: CalibrationProvenance::default(),
        })
    }

    pub fn with_unit(
        mut self,
        unit: impl Into<String>,
    ) -> Self {
        let unit = unit.into();

        if !unit.trim().is_empty() {
            self.unit = Some(unit.trim().to_string());
        }

        self
    }

    pub fn with_qubits(
        mut self,
        qubits: Vec<usize>,
    ) -> Result<Self, CalibrationError> {
        for qubit in &qubits {
            validate_qubit_id(*qubit)?;
        }

        self.qubits = qubits;

        Ok(self)
    }

    pub fn with_evidence(
        mut self,
        evidence: MeasurementEvidence,
    ) -> Result<Self, CalibrationError> {
        evidence.validate("custom_metric")?;
        self.evidence = Some(evidence);
        Ok(self)
    }

    pub fn with_provenance(
        mut self,
        provenance: CalibrationProvenance,
    ) -> Result<Self, CalibrationError> {
        provenance.validate()?;
        self.provenance = provenance;
        Ok(self)
    }

    fn validate(&self) -> Result<(), CalibrationError> {
        normalize_metric_id(&self.metric_id)?;
        validate_finite(
            "custom_metric.value",
            self.value,
        )?;

        for qubit in &self.qubits {
            validate_qubit_id(*qubit)?;
        }

        if let Some(evidence) = &self.evidence {
            evidence.validate("custom_metric")?;
        }

        self.provenance.validate()?;

        Ok(())
    }
}

// ============================================================================
// Calibration validity
// ============================================================================

/// Validity interval for a calibration snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalibrationValidity {
    /// First instant at which the calibration is valid.
    pub valid_from: CalibrationTimestamp,

    /// Optional expiration instant.
    pub valid_until: Option<CalibrationTimestamp>,
}

impl CalibrationValidity {
    /// Create a validity interval.
    pub fn new(
        valid_from: CalibrationTimestamp,
        valid_until: Option<CalibrationTimestamp>,
    ) -> Result<Self, CalibrationError> {
        if let Some(until) = valid_until {
            if until < valid_from {
                return Err(
                    CalibrationError::InvalidValidityInterval {
                        valid_from_ns: valid_from.as_unix_nanos(),
                        valid_until_ns: Some(
                            until.as_unix_nanos(),
                        ),
                    },
                );
            }
        }

        Ok(Self {
            valid_from,
            valid_until,
        })
    }

    /// Create an interval beginning now.
    pub fn from_now() -> Self {
        Self {
            valid_from: CalibrationTimestamp::now(),
            valid_until: None,
        }
    }

    /// Whether a timestamp falls inside the interval.
    pub fn contains(&self, timestamp: CalibrationTimestamp) -> bool {
        if timestamp < self.valid_from {
            return false;
        }

        match self.valid_until {
            Some(until) => timestamp <= until,
            None => true,
        }
    }

    fn validate(&self) -> Result<(), CalibrationError> {
        CalibrationValidity::new(
            self.valid_from,
            self.valid_until,
        )
        .map(|_| ())
    }
}

// ============================================================================
// Freshness policy
// ============================================================================

/// Explicit policy used when deciding whether calibration can be consumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalibrationFreshnessPolicy {
    /// Maximum permitted age.
    pub maximum_age: Duration,

    /// Whether an explicitly expired validity interval is rejected.
    pub reject_expired_interval: bool,

    /// Whether calibration with a future timestamp is rejected.
    pub reject_future_timestamp: bool,
}

impl CalibrationFreshnessPolicy {
    /// Construct a conservative default policy.
    pub const fn default_policy() -> Self {
        Self {
            maximum_age: DEFAULT_MAX_CALIBRATION_AGE,
            reject_expired_interval: true,
            reject_future_timestamp: true,
        }
    }

    /// Construct a policy with a custom maximum age.
    pub const fn with_maximum_age(
        maximum_age: Duration,
    ) -> Self {
        Self {
            maximum_age,
            reject_expired_interval: true,
            reject_future_timestamp: true,
        }
    }
}

impl Default for CalibrationFreshnessPolicy {
    fn default() -> Self {
        Self::default_policy()
    }
}

// ============================================================================
// Calibration snapshot
// ============================================================================

/// Complete immutable calibration state for a backend/device at a point in
/// time.
///
/// The structure is mutable while being assembled through `insert_*` methods,
/// but consumers should treat a completed snapshot as immutable evidence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibrationSnapshot {
    /// Schema identifier.
    pub schema_id: String,

    /// Serialized schema version.
    pub schema_version: u16,

    /// Backend identifier.
    pub backend_id: String,

    /// Optional provider identifier.
    pub provider_id: Option<String>,

    /// Optional physical device identifier.
    pub device_id: Option<String>,

    /// Snapshot creation/measurement timestamp.
    pub timestamp: CalibrationTimestamp,

    /// Validity interval.
    pub validity: Option<CalibrationValidity>,

    /// Optional overall provenance.
    pub provenance: CalibrationProvenance,

    /// Physical-qubit calibration records.
    pub qubits: BTreeMap<usize, QubitCalibration>,

    /// Instruction calibration indexed by deterministic key.
    pub instructions: BTreeMap<InstructionKey, InstructionCalibration>,

    /// Directed coupling calibration.
    pub couplings: BTreeMap<CouplingKey, CouplingCalibration>,

    /// Arbitrary provider/backend calibration metadata.
    pub metadata: BTreeMap<String, String>,

    /// Extensible calibration metrics.
    pub custom_metrics: BTreeMap<String, CustomCalibrationMetric>,
}

impl CalibrationSnapshot {
    /// Construct a new snapshot with the current timestamp.
    pub fn new(
        backend_id: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        Self::with_timestamp(
            backend_id,
            CalibrationTimestamp::now(),
        )
    }

    /// Construct a snapshot using an explicit timestamp.
    pub fn with_timestamp(
        backend_id: impl Into<String>,
        timestamp: CalibrationTimestamp,
    ) -> Result<Self, CalibrationError> {
        let backend_id = validate_identifier(
            "backend_id",
            &backend_id.into(),
            MAX_BACKEND_ID_LENGTH,
        )?;

        Ok(Self {
            schema_id: CALIBRATION_SCHEMA_ID.to_string(),
            schema_version: CALIBRATION_SCHEMA_VERSION,
            backend_id,
            provider_id: None,
            device_id: None,
            timestamp,
            validity: None,
            provenance: CalibrationProvenance::default(),
            qubits: BTreeMap::new(),
            instructions: BTreeMap::new(),
            couplings: BTreeMap::new(),
            metadata: BTreeMap::new(),
            custom_metrics: BTreeMap::new(),
        })
    }

    /// Set provider identity.
    pub fn with_provider_id(
        mut self,
        provider_id: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        self.provider_id = Some(validate_identifier(
            "provider_id",
            &provider_id.into(),
            MAX_PROVIDER_ID_LENGTH,
        )?);

        Ok(self)
    }

    /// Set device identity.
    pub fn with_device_id(
        mut self,
        device_id: impl Into<String>,
    ) -> Result<Self, CalibrationError> {
        self.device_id = Some(validate_identifier(
            "device_id",
            &device_id.into(),
            MAX_DEVICE_ID_LENGTH,
        )?);

        Ok(self)
    }

    /// Set validity interval.
    pub fn with_validity(
        mut self,
        validity: CalibrationValidity,
    ) -> Result<Self, CalibrationError> {
        validity.validate()?;
        self.validity = Some(validity);
        Ok(self)
    }

    /// Set overall provenance.
    pub fn with_provenance(
        mut self,
        provenance: CalibrationProvenance,
    ) -> Result<Self, CalibrationError> {
        provenance.validate()?;
        self.provenance = provenance;
        Ok(self)
    }

    /// Insert or replace a qubit calibration.
    pub fn insert_qubit(
        &mut self,
        calibration: QubitCalibration,
    ) -> Result<(), CalibrationError> {
        calibration.validate()?;

        if !self.qubits.contains_key(&calibration.qubit)
            && self.qubits.len() >= DEFAULT_MAX_QUBITS
        {
            return Err(
                CalibrationError::QubitLimitExceeded {
                    requested: self.qubits.len() + 1,
                    maximum: DEFAULT_MAX_QUBITS,
                },
            );
        }

        self.qubits
            .insert(calibration.qubit, calibration);

        Ok(())
    }

    /// Insert or replace an instruction calibration.
    pub fn insert_instruction(
        &mut self,
        calibration: InstructionCalibration,
    ) -> Result<(), CalibrationError> {
        calibration.validate()?;

        let key = calibration.key();

        if !self.instructions.contains_key(&key)
            && self.instructions.len() >= DEFAULT_MAX_GATES
        {
            return Err(
                CalibrationError::GateLimitExceeded {
                    requested: self.instructions.len() + 1,
                    maximum: DEFAULT_MAX_GATES,
                },
            );
        }

        self.instructions.insert(key, calibration);

        Ok(())
    }

    /// Compatibility alias for gate-oriented callers.
    pub fn insert_gate(
        &mut self,
        calibration: InstructionCalibration,
    ) -> Result<(), CalibrationError> {
        self.insert_instruction(calibration)
    }

    /// Insert or replace coupling calibration.
    pub fn insert_coupling(
        &mut self,
        calibration: CouplingCalibration,
    ) -> Result<(), CalibrationError> {
        calibration.validate()?;

        let key = calibration.key();

        if !self.couplings.contains_key(&key)
            && self.couplings.len() >= DEFAULT_MAX_COUPLINGS
        {
            return Err(
                CalibrationError::CouplingLimitExceeded {
                    requested: self.couplings.len() + 1,
                    maximum: DEFAULT_MAX_COUPLINGS,
                },
            );
        }

        self.couplings.insert(key, calibration);

        Ok(())
    }

    /// Add backend/provider metadata.
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
            && self.metadata.len() >= DEFAULT_MAX_METADATA_FIELDS
        {
            return Err(
                CalibrationError::MetadataLimitExceeded {
                    requested: self.metadata.len() + 1,
                    maximum: DEFAULT_MAX_METADATA_FIELDS,
                },
            );
        }

        self.metadata.insert(
            key,
            value,
        );

        Ok(())
    }

    /// Insert a custom calibration metric.
    pub fn insert_custom_metric(
        &mut self,
        metric: CustomCalibrationMetric,
    ) -> Result<(), CalibrationError> {
        metric.validate()?;

        if !self.custom_metrics.contains_key(&metric.metric_id)
            && self.custom_metrics.len() >= DEFAULT_MAX_CUSTOM_METRICS
        {
            return Err(
                CalibrationError::CustomMetricLimitExceeded {
                    requested: self.custom_metrics.len() + 1,
                    maximum: DEFAULT_MAX_CUSTOM_METRICS,
                },
            );
        }

        self.custom_metrics.insert(
            metric.metric_id.clone(),
            metric,
        );

        Ok(())
    }

    /// Retrieve qubit calibration.
    pub fn qubit(
        &self,
        qubit: usize,
    ) -> Option<&QubitCalibration> {
        self.qubits.get(&qubit)
    }

    /// Retrieve an instruction calibration.
    pub fn instruction(
        &self,
        instruction: &str,
        qubits: &[usize],
    ) -> Option<&InstructionCalibration> {
        let instruction =
            normalize_instruction_name(instruction).ok()?;

        let key = InstructionKey {
            instruction,
            qubits: qubits.to_vec(),
        };

        self.instructions.get(&key)
    }

    /// Compatibility alias for gate-oriented callers.
    pub fn gate(
        &self,
        gate: &str,
        qubits: &[usize],
    ) -> Option<&InstructionCalibration> {
        self.instruction(gate, qubits)
    }

    /// Retrieve directed coupling calibration.
    pub fn coupling(
        &self,
        source: usize,
        target: usize,
    ) -> Option<&CouplingCalibration> {
        self.couplings.get(&CouplingKey {
            source,
            target,
        })
    }

    /// Retrieve a custom metric.
    pub fn custom_metric(
        &self,
        metric_id: &str,
    ) -> Option<&CustomCalibrationMetric> {
        self.custom_metrics.get(metric_id)
    }

    /// Number of calibrated qubits.
    pub fn qubit_count(&self) -> usize {
        self.qubits.len()
    }

    /// Number of calibrated instructions.
    pub fn instruction_count(&self) -> usize {
        self.instructions.len()
    }

    /// Compatibility alias for existing callers.
    pub fn gate_count(&self) -> usize {
        self.instruction_count()
    }

    /// Number of calibrated couplings.
    pub fn coupling_count(&self) -> usize {
        self.couplings.len()
    }

    /// Number of custom metrics.
    pub fn custom_metric_count(&self) -> usize {
        self.custom_metrics.len()
    }

    /// Whether the snapshot contains any calibration evidence.
    pub fn is_empty(&self) -> bool {
        self.qubits.is_empty()
            && self.instructions.is_empty()
            && self.couplings.is_empty()
            && self.custom_metrics.is_empty()
    }

    /// Validate the complete snapshot.
    pub fn validate(&self) -> Result<(), CalibrationError> {
        if self.schema_id != CALIBRATION_SCHEMA_ID {
            return Err(CalibrationError::ConflictingCalibration {
                resource: "schema_id".to_string(),
                message: format!(
                    "expected `{CALIBRATION_SCHEMA_ID}`, found `{}`",
                    self.schema_id
                ),
            });
        }

        if self.schema_version == 0
            || self.schema_version > CALIBRATION_SCHEMA_VERSION
        {
            return Err(CalibrationError::ConflictingCalibration {
                resource: "schema_version".to_string(),
                message: format!(
                    "unsupported calibration schema version {}",
                    self.schema_version
                ),
            });
        }

        validate_identifier(
            "backend_id",
            &self.backend_id,
            MAX_BACKEND_ID_LENGTH,
        )?;

        if let Some(provider_id) = &self.provider_id {
            validate_identifier(
                "provider_id",
                provider_id,
                MAX_PROVIDER_ID_LENGTH,
            )?;
        }

        if let Some(device_id) = &self.device_id {
            validate_identifier(
                "device_id",
                device_id,
                MAX_DEVICE_ID_LENGTH,
            )?;
        }

        self.provenance.validate()?;

        if let Some(validity) = &self.validity {
            validity.validate()?;
        }

        if self.is_empty() {
            return Err(CalibrationError::EmptySnapshot);
        }

        if self.qubits.len() > DEFAULT_MAX_QUBITS {
            return Err(
                CalibrationError::QubitLimitExceeded {
                    requested: self.qubits.len(),
                    maximum: DEFAULT_MAX_QUBITS,
                },
            );
        }

        if self.instructions.len() > DEFAULT_MAX_GATES {
            return Err(
                CalibrationError::GateLimitExceeded {
                    requested: self.instructions.len(),
                    maximum: DEFAULT_MAX_GATES,
                },
            );
        }

        if self.couplings.len() > DEFAULT_MAX_COUPLINGS {
            return Err(
                CalibrationError::CouplingLimitExceeded {
                    requested: self.couplings.len(),
                    maximum: DEFAULT_MAX_COUPLINGS,
                },
            );
        }

        if self.metadata.len() > DEFAULT_MAX_METADATA_FIELDS {
            return Err(
                CalibrationError::MetadataLimitExceeded {
                    requested: self.metadata.len(),
                    maximum: DEFAULT_MAX_METADATA_FIELDS,
                },
            );
        }

        if self.custom_metrics.len() > DEFAULT_MAX_CUSTOM_METRICS {
            return Err(
                CalibrationError::CustomMetricLimitExceeded {
                    requested: self.custom_metrics.len(),
                    maximum: DEFAULT_MAX_CUSTOM_METRICS,
                },
            );
        }

        for calibration in self.qubits.values() {
            calibration.validate()?;
        }

        for calibration in self.instructions.values() {
            calibration.validate()?;
        }

        for calibration in self.couplings.values() {
            calibration.validate()?;
        }

        for (key, metric) in &self.custom_metrics {
            if key != &metric.metric_id {
                return Err(
                    CalibrationError::ConflictingCalibration {
                        resource: "custom_metric".to_string(),
                        message:
                            "map key does not match metric identifier"
                                .to_string(),
                    },
                );
            }

            metric.validate()?;
        }

        for (key, value) in &self.metadata {
            validate_metadata_key(key)?;

            if value.len() > MAX_METADATA_VALUE_LENGTH {
                return Err(
                    CalibrationError::MetadataValueTooLong {
                        key: key.clone(),
                        length: value.len(),
                        maximum: MAX_METADATA_VALUE_LENGTH,
                    },
                );
            }
        }

        Ok(())
    }

    /// Return the age of this snapshot.
    pub fn age(&self) -> Duration {
        self.timestamp.age()
    }

    /// Whether this snapshot exceeds a caller-supplied maximum age.
    pub fn is_stale(&self, maximum_age: Duration) -> bool {
        self.age() > maximum_age
    }

    /// Validate freshness using an explicit policy.
    pub fn require_fresh(
        &self,
        policy: CalibrationFreshnessPolicy,
    ) -> Result<(), CalibrationError> {
        self.validate()?;

        let now = CalibrationTimestamp::now();

        if policy.reject_future_timestamp
            && self.timestamp > now
        {
            return Err(CalibrationError::ConflictingCalibration {
                resource: "timestamp".to_string(),
                message:
                    "calibration timestamp is in the future"
                        .to_string(),
            });
        }

        if self.is_stale(policy.maximum_age) {
            return Err(CalibrationError::StaleCalibration {
                age_ns: self
                    .age()
                    .as_nanos()
                    .min(u64::MAX as u128)
                    as u64,
                maximum_age_ns: policy
                    .maximum_age
                    .as_nanos()
                    .min(u64::MAX as u128)
                    as u64,
            });
        }

        if policy.reject_expired_interval {
            if let Some(validity) = self.validity {
                if !validity.contains(now) {
                    return Err(CalibrationError::StaleCalibration {
                        age_ns: self
                            .age()
                            .as_nanos()
                            .min(u64::MAX as u128)
                            as u64,
                        maximum_age_ns: policy
                            .maximum_age
                            .as_nanos()
                            .min(u64::MAX as u128)
                            as u64,
                    });
                }
            }
        }

        Ok(())
    }

    /// Require freshness using the repository default policy.
    pub fn require_default_freshness(
        &self,
    ) -> Result<(), CalibrationError> {
        self.require_fresh(
            CalibrationFreshnessPolicy::default(),
        )
    }

    /// Average calibrated instruction error.
    ///
    /// Only instructions with an actual error measurement contribute.
    pub fn average_instruction_error(&self) -> Option<f64> {
        let values = self
            .instructions
            .values()
            .filter_map(|instruction| {
                instruction.error_rate
            })
            .collect::<Vec<_>>();

        if values.is_empty() {
            return None;
        }

        Some(
            values.iter().copied().sum::<f64>()
                / values.len() as f64,
        )
    }

    /// Worst calibrated instruction error.
    pub fn worst_instruction_error(&self) -> Option<f64> {
        self.instructions
            .values()
            .filter_map(|instruction| {
                instruction.error_rate
            })
            .reduce(f64::max)
    }

    /// Compatibility alias for existing gate-oriented callers.
    pub fn average_gate_error(&self) -> Option<f64> {
        self.average_instruction_error()
    }

    /// Compatibility alias for existing gate-oriented callers.
    pub fn worst_gate_error(&self) -> Option<f64> {
        self.worst_instruction_error()
    }

    /// Return the best available effective coherence among calibrated qubits.
    pub fn best_effective_coherence_ns(&self) -> Option<f64> {
        self.qubits
            .values()
            .filter_map(
                QubitCalibration::effective_coherence_ns,
            )
            .reduce(f64::max)
    }

    /// Return the worst available effective coherence among calibrated
    /// qubits.
    pub fn worst_effective_coherence_ns(&self) -> Option<f64> {
        self.qubits
            .values()
            .filter_map(
                QubitCalibration::effective_coherence_ns,
            )
            .reduce(f64::min)
    }

    /// Serialize this snapshot into deterministic JSON.
    pub fn to_json(&self) -> Result<String, CalibrationError> {
        self.validate()?;

        serde_json::to_string(self).map_err(|error| {
            CalibrationError::Serialization {
                message: error.to_string(),
            }
        })
    }

    /// Deserialize and validate a calibration snapshot.
    pub fn from_json(
        json: &str,
    ) -> Result<Self, CalibrationError> {
        let snapshot: Self =
            serde_json::from_str(json).map_err(|error| {
                CalibrationError::Serialization {
                    message: error.to_string(),
                }
            })?;

        snapshot.validate()?;

        Ok(snapshot)
    }

    /// Calculate a deterministic SHA-256 fingerprint.
    ///
    /// The fingerprint covers the complete validated serialized snapshot.
    /// It does not authenticate its origin.
    pub fn fingerprint(
        &self,
    ) -> Result<String, CalibrationError> {
        let json = self.to_json()?;

        let digest = Sha256::digest(json.as_bytes());

        Ok(hex_encode(&digest))
    }
}

// ============================================================================
// Helper validation
// ============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<String, CalibrationError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(
            CalibrationError::EmptyIdentifier { field },
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
            CalibrationError::InvalidIdentifier { field },
        );
    }

    Ok(trimmed.to_string())
}

fn validate_qubit_id(
    qubit: usize,
) -> Result<(), CalibrationError> {
    if qubit == usize::MAX {
        return Err(CalibrationError::InvalidQubit {
            qubit,
        });
    }

    Ok(())
}

fn validate_finite(
    field: &str,
    value: f64,
) -> Result<(), CalibrationError> {
    if !value.is_finite() {
        return Err(
            CalibrationError::InvalidNumericValue {
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
) -> Result<(), CalibrationError> {
    if !value.is_finite() || value < 0.0 {
        return Err(
            CalibrationError::InvalidNumericValue {
                field: field.to_string(),
                value,
            },
        );
    }

    Ok(())
}

fn validate_probability(
    field: &str,
    value: f64,
) -> Result<(), CalibrationError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(
            CalibrationError::InvalidProbability {
                field: field.to_string(),
                value,
            },
        );
    }

    Ok(())
}

fn validate_coherence(
    field: &str,
    value_ns: f64,
) -> Result<(), CalibrationError> {
    if !value_ns.is_finite() || value_ns < 0.0 {
        return Err(
            CalibrationError::InvalidNumericValue {
                field: field.to_string(),
                value: value_ns,
            },
        );
    }

    Ok(())
}

fn validate_frequency(
    field: &str,
    value_hz: f64,
) -> Result<(), CalibrationError> {
    if !value_hz.is_finite() || value_hz < 0.0 {
        return Err(
            CalibrationError::InvalidFrequency {
                field: field.to_string(),
                value_hz,
            },
        );
    }

    Ok(())
}

fn validate_positive_duration(
    field: &str,
    value_ns: u64,
) -> Result<(), CalibrationError> {
    if value_ns == 0 {
        return Err(
            CalibrationError::InvalidDuration {
                field: field.to_string(),
                value_ns,
            },
        );
    }

    Ok(())
}

fn validate_confidence(
    value: f64,
) -> Result<(), CalibrationError> {
    if !value.is_finite() || !(0.0 < value && value < 1.0) {
        return Err(
            CalibrationError::InvalidConfidenceLevel {
                value,
            },
        );
    }

    Ok(())
}

fn validate_instruction_qubits(
    instruction: &str,
    qubits: &[usize],
) -> Result<(), CalibrationError> {
    if qubits.len() > MAX_INSTRUCTION_QUBITS {
        return Err(
            CalibrationError::InstructionQubitLimitExceeded {
                instruction: instruction.to_string(),
                requested: qubits.len(),
                maximum: MAX_INSTRUCTION_QUBITS,
            },
        );
    }

    let mut seen = BTreeSet::new();

    for qubit in qubits {
        validate_qubit_id(*qubit)?;

        if !seen.insert(*qubit) {
            return Err(
                CalibrationError::DuplicateInstructionQubit {
                    instruction: instruction.to_string(),
                    qubit: *qubit,
                },
            );
        }
    }

    Ok(())
}

fn normalize_instruction_name(
    instruction: &str,
) -> Result<String, CalibrationError> {
    let normalized = instruction.trim().to_ascii_lowercase();

    if normalized.is_empty() {
        return Err(CalibrationError::InvalidInstruction {
            instruction: instruction.to_string(),
        });
    }

    if normalized.chars().any(char::is_control) {
        return Err(CalibrationError::InvalidInstruction {
            instruction: instruction.to_string(),
        });
    }

    Ok(normalized)
}

fn normalize_metric_id(
    metric: &str,
) -> Result<String, CalibrationError> {
    let normalized = metric.trim().to_ascii_lowercase();

    if normalized.is_empty() {
        return Err(CalibrationError::InvalidMetricId {
            metric: metric.to_string(),
        });
    }

    if normalized.chars().any(char::is_control) {
        return Err(CalibrationError::InvalidMetricId {
            metric: metric.to_string(),
        });
    }

    Ok(normalized)
}

fn validate_metadata_key(
    key: &str,
) -> Result<(), CalibrationError> {
    if key.trim().is_empty() {
        return Err(
            CalibrationError::InvalidMetadataKey {
                key: key.to_string(),
            },
        );
    }

    if key.len() > MAX_METADATA_KEY_LENGTH {
        return Err(
            CalibrationError::InvalidMetadataKey {
                key: key.to_string(),
            },
        );
    }

    if key.chars().any(char::is_control) {
        return Err(
            CalibrationError::InvalidMetadataKey {
                key: key.to_string(),
            },
        );
    }

    Ok(())
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] =
        b"0123456789abcdef";

    let mut output =
        String::with_capacity(bytes.len() * 2);

    for byte in bytes {
        output.push(
            HEX[(byte >> 4) as usize] as char,
        );
        output.push(
            HEX[(byte & 0x0f) as usize] as char,
        );
    }

    output
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_qubit(
        qubit: usize,
    ) -> QubitCalibration {
        QubitCalibration::new(qubit)
            .expect("valid qubit")
            .with_t1_ns(100_000.0)
            .expect("valid t1")
            .with_t2_ns(80_000.0)
            .expect("valid t2")
            .with_frequency_hz(5.0e9)
            .expect("valid frequency")
            .with_reset_error(0.001)
            .expect("valid reset error")
    }

    #[test]
    fn timestamp_round_trip_is_exact() {
        let timestamp =
            CalibrationTimestamp::from_unix_nanos(
                1_700_000_000_000_000_000,
            );

        assert_eq!(
            timestamp.as_unix_nanos(),
            1_700_000_000_000_000_000
        );
    }

    #[test]
    fn invalid_probability_is_rejected() {
        assert!(
            ReadoutCalibration::new(
                1.2,
                0.1,
            )
            .is_err()
        );

        assert!(
            ReadoutCalibration::new(
                f64::NAN,
                0.1,
            )
            .is_err()
        );
    }

    #[test]
    fn invalid_coherence_is_rejected() {
        assert!(
            QubitCalibration::new(0)
                .expect("valid qubit")
                .with_t1_ns(-1.0)
                .is_err()
        );

        assert!(
            QubitCalibration::new(0)
                .expect("valid qubit")
                .with_t2_ns(f64::INFINITY)
                .is_err()
        );
    }

    #[test]
    fn readout_calibration_is_correct() {
        let readout =
            ReadoutCalibration::new(
                0.02,
                0.04,
            )
            .expect("valid readout");

        assert!(
            (readout.average_error() - 0.03).abs()
                < f64::EPSILON
        );

        assert!(
            (readout.average_fidelity() - 0.97).abs()
                < f64::EPSILON
        );

        assert!(!readout.is_measured());
    }

    #[test]
    fn measured_readout_requires_positive_samples() {
        let evidence =
            MeasurementEvidence::new(10_000)
                .expect("valid samples");

        let readout =
            ReadoutCalibration::new(
                0.01,
                0.02,
            )
            .expect("valid readout")
            .with_evidence(evidence)
            .expect("valid evidence");

        assert!(readout.is_measured());
    }

    #[test]
    fn instruction_names_are_normalized() {
        let instruction =
            InstructionCalibration::new(
                " CX ",
                vec![0, 1],
            )
            .expect("valid instruction");

        assert_eq!(
            instruction.instruction,
            "cx"
        );
    }

    #[test]
    fn instruction_operand_order_is_preserved() {
        let first =
            InstructionCalibration::new(
                "cx",
                vec![0, 1],
            )
            .expect("valid instruction");

        let second =
            InstructionCalibration::new(
                "cx",
                vec![1, 0],
            )
            .expect("valid instruction");

        assert_ne!(
            first.key(),
            second.key()
        );
    }

    #[test]
    fn duplicate_instruction_operands_are_rejected() {
        let result =
            InstructionCalibration::new(
                "cx",
                vec![0, 0],
            );

        assert!(matches!(
            result,
            Err(
                CalibrationError::DuplicateInstructionQubit {
                    ..
                }
            )
        ));
    }

    #[test]
    fn coupling_is_directional() {
        let forward =
            CouplingCalibration::new(0, 1)
                .expect("valid coupling");

        let reverse =
            CouplingCalibration::new(1, 0)
                .expect("valid coupling");

        assert_ne!(
            forward.key(),
            reverse.key()
        );
    }

    #[test]
    fn coupling_rejects_self_connection() {
        assert!(
            CouplingCalibration::new(0, 0)
                .is_err()
        );
    }

    #[test]
    fn snapshot_can_be_built_and_validated() {
        let mut snapshot =
            CalibrationSnapshot::new(
                "local://test-qpu",
            )
            .expect("valid backend");

        snapshot
            .insert_qubit(sample_qubit(0))
            .expect("qubit insertion");

        snapshot
            .insert_qubit(sample_qubit(1))
            .expect("qubit insertion");

        let gate =
            InstructionCalibration::new(
                "cx",
                vec![0, 1],
            )
            .expect("valid gate")
            .with_duration_ns(300)
            .expect("valid duration")
            .with_error_rate(0.01)
            .expect("valid error");

        snapshot
            .insert_instruction(gate)
            .expect("instruction insertion");

        let coupling =
            CouplingCalibration::new(0, 1)
                .expect("valid coupling")
                .with_error_rate(0.015)
                .expect("valid error")
                .with_duration_ns(300)
                .expect("valid duration");

        snapshot
            .insert_coupling(coupling)
            .expect("coupling insertion");

        assert!(
            snapshot.validate().is_ok()
        );

        assert_eq!(
            snapshot.qubit_count(),
            2
        );

        assert_eq!(
            snapshot.instruction_count(),
            1
        );

        assert_eq!(
            snapshot.coupling_count(),
            1
        );
    }

    #[test]
    fn snapshot_rejects_empty_state() {
        let snapshot =
            CalibrationSnapshot::new(
                "test-qpu",
            )
            .expect("valid backend");

        assert!(matches!(
            snapshot.validate(),
            Err(
                CalibrationError::EmptySnapshot
            )
        ));
    }

    #[test]
    fn snapshot_lookup_is_deterministic() {
        let mut snapshot =
            CalibrationSnapshot::new(
                "test-qpu",
            )
            .expect("valid backend");

        let gate =
            InstructionCalibration::new(
                " CX ",
                vec![0, 1],
            )
            .expect("valid gate")
            .with_duration_ns(300)
            .expect("valid duration");

        snapshot
            .insert_gate(gate)
            .expect("gate insertion");

        assert!(
            snapshot
                .gate("cx", &[0, 1])
                .is_some()
        );
    }

    #[test]
    fn optional_calibration_values_do_not_use_zero_as_unknown() {
        let qubit =
            QubitCalibration::new(0)
                .expect("valid qubit");

        assert_eq!(
            qubit.t1_ns,
            None
        );

        assert_eq!(
            qubit.t2_ns,
            None
        );

        assert_eq!(
            qubit.reset_error,
            None
        );
    }

    #[test]
    fn metadata_is_bounded() {
        let mut snapshot =
            CalibrationSnapshot::new(
                "test-qpu",
            )
            .expect("valid backend");

        snapshot
            .insert_metadata(
                "provider.api_version",
                "1.0",
            )
            .expect("metadata");

        assert_eq!(
            snapshot.metadata.get(
                "provider.api_version"
            ),
            Some(&"1.0".to_string())
        );
    }

    #[test]
    fn validity_interval_is_checked() {
        let from =
            CalibrationTimestamp::from_unix_nanos(
                100,
            );

        let until =
            CalibrationTimestamp::from_unix_nanos(
                200,
            );

        let validity =
            CalibrationValidity::new(
                from,
                Some(until),
            )
            .expect("valid interval");

        assert!(
            validity.contains(
                CalibrationTimestamp::from_unix_nanos(
                    150
                )
            )
        );

        assert!(
            !validity.contains(
                CalibrationTimestamp::from_unix_nanos(
                    201
                )
            )
        );
    }

    #[test]
    fn invalid_validity_interval_is_rejected() {
        let result =
            CalibrationValidity::new(
                CalibrationTimestamp::from_unix_nanos(
                    200,
                ),
                Some(
                    CalibrationTimestamp::from_unix_nanos(
                        100,
                    ),
                ),
            );

        assert!(result.is_err());
    }

    #[test]
    fn stale_calibration_is_detected() {
        let timestamp =
            CalibrationTimestamp::from_unix_nanos(
                0,
            );

        let snapshot =
            CalibrationSnapshot::with_timestamp(
                "test-qpu",
                timestamp,
            )
            .expect("valid backend");

        assert!(
            snapshot.is_stale(
                Duration::from_secs(1)
            )
        );
    }

    #[test]
    fn fingerprint_is_deterministic() {
        let timestamp =
            CalibrationTimestamp::from_unix_nanos(
                1_000_000,
            );

        let mut first =
            CalibrationSnapshot::with_timestamp(
                "test-qpu",
                timestamp,
            )
            .expect("valid snapshot");

        let mut second =
            CalibrationSnapshot::with_timestamp(
                "test-qpu",
                timestamp,
            )
            .expect("valid snapshot");

        first
            .insert_qubit(sample_qubit(0))
            .expect("qubit");

        second
            .insert_qubit(sample_qubit(0))
            .expect("qubit");

        assert_eq!(
            first
                .fingerprint()
                .expect("fingerprint"),
            second
                .fingerprint()
                .expect("fingerprint")
        );
    }

    #[test]
    fn json_round_trip_preserves_snapshot() {
        let timestamp =
            CalibrationTimestamp::from_unix_nanos(
                1_000_000,
            );

        let mut original =
            CalibrationSnapshot::with_timestamp(
                "test-qpu",
                timestamp,
            )
            .expect("valid snapshot");

        original
            .insert_qubit(sample_qubit(0))
            .expect("qubit");

        original
            .insert_metadata(
                "calibration.kind",
                "measured",
            )
            .expect("metadata");

        let json =
            original
                .to_json()
                .expect("serialize");

        let restored =
            CalibrationSnapshot::from_json(
                &json,
            )
            .expect("deserialize");

        assert_eq!(
            original,
            restored
        );
    }

    #[test]
    fn custom_metric_is_supported() {
        let metric =
            CustomCalibrationMetric::new(
                "rb_1q_error",
                0.001,
            )
            .expect("metric")
            .with_unit("probability");

        let mut snapshot =
            CalibrationSnapshot::new(
                "test-qpu",
            )
            .expect("snapshot");

        snapshot
            .insert_custom_metric(metric)
            .expect("metric insertion");

        assert_eq!(
            snapshot.custom_metric_count(),
            1
        );
    }

    #[test]
    fn provenance_can_record_provider_and_device() {
        let provenance =
            CalibrationProvenance::measured()
                .with_provider("ibm")
                .expect("provider")
                .with_device("ibm-example")
                .expect("device")
                .with_method("randomized_benchmarking")
                .expect("method");

        assert_eq!(
            provenance.provider_id.as_deref(),
            Some("ibm")
        );

        assert_eq!(
            provenance.device_id.as_deref(),
            Some("ibm-example")
        );
    }

    #[test]
    fn non_finite_values_are_rejected() {
        assert!(
            InstructionCalibration::new(
                "rx",
                vec![0],
            )
            .expect("instruction")
            .with_amplitude(
                f64::NAN
            )
            .is_err()
        );

        assert!(
            InstructionCalibration::new(
                "rx",
                vec![0],
            )
            .expect("instruction")
            .with_frequency_hz(
                f64::INFINITY
            )
            .is_err()
        );
    }
}