//! Zamani Quantum Benchmarking — Reproducibility Validation
//!
//! Production validation layer for the quantum benchmarking reproducibility
//! contract.
//!
//! # Purpose
//!
//! This module verifies that a benchmark's reproducibility metadata is:
//!
//! - structurally complete;
//! - internally consistent;
//! - deterministic;
//! - bounded;
//! - compatible with the canonical reproducibility schema;
//! - safe to accept from untrusted configuration/result sources;
//! - suitable for scientific comparison and regression analysis.
//!
//! This module DOES NOT:
//!
//! - generate circuits;
//! - execute circuits;
//! - generate random numbers;
//! - calculate benchmark metrics;
//! - calculate statistical confidence intervals;
//! - own canonical Quantum IR;
//! - own benchmark configuration;
//! - own benchmark results;
//! - hash arbitrary objects using `Debug` formatting;
//! - access process-global state;
//! - access the system clock;
//! - silently repair invalid reproducibility metadata.
//!
//! Instead, it validates the reproducibility primitives defined by:
//!
//! ```text
//! benchmarking::core::reproducibility
//!                 │
//!                 ▼
//!       validation::reproducibility
//!                 │
//!        ┌────────┼────────┐
//!        ▼        ▼        ▼
//!      config   circuits  results
//!        │        │        │
//!        └────────┼────────┘
//!                 ▼
//!          validated experiment
//! ```
//!
//! # Architectural boundary
//!
//! The authoritative fingerprint implementation lives in
//! `core::reproducibility`.
//!
//! This module MUST NOT implement a second fingerprint algorithm.
//!
//! The dependency direction is therefore:
//!
//! ```text
//! core::reproducibility
//!          ▲
//!          │
//! validation::reproducibility
//! ```
//!
//! and never:
//!
//! ```text
//! validation::reproducibility
//!          │
//!          ▼
//! core::reproducibility
//!          │
//!          ▼
//! validation::reproducibility
//! ```
//!
//! which would create a dependency cycle.
//!
//! # Validation philosophy
//!
//! Reproducibility validation is deliberately fail-closed.
//!
//! If metadata required to establish reproducibility is missing, malformed,
//! ambiguous, inconsistent, or outside the configured resource bounds, the
//! validator returns an error.
//!
//! It does NOT:
//!
//! - substitute the current time;
//! - invent a seed;
//! - generate a missing fingerprint;
//! - silently reorder circuit fingerprints;
//! - silently deduplicate circuits;
//! - accept a zero fingerprint as "unknown";
//! - downgrade a failed validation into a warning.
//!
//! A caller may explicitly choose to run a less strict validation policy, but
//! strict validation is the production default.
//!
//! # Scientific reproducibility
//!
//! A reproducibility identity establishes that two experiments have the same
//! declared canonical identity. It does NOT establish that:
//!
//! - the hardware behaved honestly;
//! - the backend returned truthful measurements;
//! - two machines have identical physical conditions;
//! - floating-point execution was bit-for-bit identical;
//! - an external provider preserved the submitted circuit;
//! - the result is scientifically correct.
//!
//! Those questions belong to provenance, execution integrity, backend
//! attestation, statistical validation, and result validation.
//!
//! # Important distinction
//!
//! ```text
//! reproducible definition
//!         !=
//! reproducible physical execution
//!         !=
//! scientifically valid result
//! ```
//!
//! This module validates the first boundary and the metadata necessary to
//! establish it.
//!
//! # Canonical bytes
//!
//! The caller is responsible for constructing canonical bytes according to
//! the contract of the object being fingerprinted.
//!
//! This module therefore treats canonical byte slices as opaque data.
//!
//! It does not:
//!
//! - normalize Unicode;
//! - change line endings;
//! - trim whitespace;
//! - reinterpret encodings;
//! - serialize Rust structures;
//! - use locale-sensitive formatting.
//!
//! A caller that supplies non-canonical bytes will receive a valid fingerprint
//! for those bytes, but the experiment may not be reproducible across callers.
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
//! # Integration contract
//!
//! This module is designed to be consumed by:
//!
//! - `core::benchmark` before an experiment is executed;
//! - `core::experiment` when an experiment is finalized;
//! - `core::provenance` before provenance is accepted;
//! - `generators::*` after deterministic workload generation;
//! - `execution::*` before result publication;
//! - `analysis::*` before comparing benchmark results;
//! - `reporting::*` before emitting a reproducibility claim;
//! - `tests/reproducibility_tests.rs` for deterministic regression tests.
//!
//! The module intentionally does not require any of those future modules to
//! exist. It can therefore be implemented and tested independently.
//!
//! # Compatibility guarantee
//!
//! Future benchmarking modules must consume this validator through its public
//! API rather than duplicating these checks.
//!
//! If a future schema changes the meaning of reproducibility, the schema
//! version must change in `core::reproducibility` and a corresponding
//! validation policy can be introduced here.
//!
//! The validator must never silently accept a newer schema it does not
//! understand.

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use crate::quantum::benchmarking::core::reproducibility::{
    BenchmarkSeed,
    CircuitFingerprint,
    ConfigurationFingerprint,
    ExperimentIdentity,
    Fingerprint,
    GeneratorDescriptor,
    ResultFingerprint,
    FINGERPRINT_ALGORITHM,
    FINGERPRINT_BYTES,
    REPRODUCIBILITY_SCHEMA_VERSION,
};

// ============================================================================
// Public constants
// ============================================================================

/// Stable identifier for this validation subsystem.
pub const REPRODUCIBILITY_VALIDATION_ID: &str =
    "zamani.quantum.benchmark.validation.reproducibility";

/// Version of the validation contract.
///
/// This is deliberately separate from the reproducibility schema version.
///
/// A validation implementation can evolve without changing the fingerprint
/// scheme, provided the semantic reproducibility contract remains compatible.
pub const REPRODUCIBILITY_VALIDATION_VERSION: u16 = 1;

/// Maximum size accepted for one canonical byte representation.
///
/// Canonical benchmark configuration/circuit/result serialization should
/// normally be considerably smaller. The limit protects an untrusted input
/// boundary against accidental or malicious memory pressure.
pub const DEFAULT_MAX_CANONICAL_BYTES: usize = 64 * 1024 * 1024;

/// Maximum number of circuit fingerprints accepted by the default validator.
///
/// This is deliberately finite because circuit fingerprint lists are part of
/// experiment identity and may otherwise be used to cause excessive memory or
/// CPU consumption.
pub const DEFAULT_MAX_CIRCUIT_FINGERPRINTS: usize = 1_000_000;

/// Maximum length accepted for benchmark/generator textual identifiers.
///
/// The underlying reproducibility module already validates its identifiers.
/// This validator keeps an explicit bound as defense in depth.
pub const DEFAULT_MAX_IDENTIFIER_BYTES: usize = 4096;

/// Maximum number of validation warnings accumulated by one validation run.
pub const DEFAULT_MAX_WARNINGS: usize = 256;

// ============================================================================
// Validation policy
// ============================================================================

/// Policy controlling strictness and resource bounds of reproducibility
/// validation.
///
/// The policy itself is deterministic data. It does not access the system
/// clock or global state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReproducibilityValidationPolicy {
    /// Require the current reproducibility schema exactly.
    ///
    /// When true, a schema different from
    /// `REPRODUCIBILITY_SCHEMA_VERSION` is rejected.
    pub require_current_schema: bool,

    /// Require SHA-256 as the fingerprint algorithm.
    pub require_sha256: bool,

    /// Reject all-zero fingerprints.
    pub reject_zero_fingerprints: bool,

    /// Maximum canonical byte representation accepted.
    pub max_canonical_bytes: usize,

    /// Maximum number of circuit fingerprints accepted.
    pub max_circuit_fingerprints: usize,

    /// Maximum identifier length checked by this layer.
    pub max_identifier_bytes: usize,

    /// Maximum number of warnings retained.
    pub max_warnings: usize,

    /// Require circuit fingerprints to be unique.
    pub require_unique_circuit_fingerprints: bool,

    /// Require circuit fingerprints to be supplied in semantic execution
    /// order.
    ///
    /// The validator cannot determine semantic ordering itself, but when this
    /// flag is enabled it rejects an explicitly declared non-canonical order
    /// marker supplied through the validation input.
    pub require_canonical_circuit_order: bool,

    /// Require a result fingerprint when validating completed results.
    pub require_result_fingerprint: bool,
}

impl Default for ReproducibilityValidationPolicy {
    fn default() -> Self {
        Self {
            require_current_schema: true,
            require_sha256: true,
            reject_zero_fingerprints: true,
            max_canonical_bytes: DEFAULT_MAX_CANONICAL_BYTES,
            max_circuit_fingerprints: DEFAULT_MAX_CIRCUIT_FINGERPRINTS,
            max_identifier_bytes: DEFAULT_MAX_IDENTIFIER_BYTES,
            max_warnings: DEFAULT_MAX_WARNINGS,
            require_unique_circuit_fingerprints: true,
            require_canonical_circuit_order: true,
            require_result_fingerprint: true,
        }
    }
}

impl ReproducibilityValidationPolicy {
    /// Creates the production validation policy.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            require_current_schema: true,
            require_sha256: true,
            reject_zero_fingerprints: true,
            max_canonical_bytes: DEFAULT_MAX_CANONICAL_BYTES,
            max_circuit_fingerprints: DEFAULT_MAX_CIRCUIT_FINGERPRINTS,
            max_identifier_bytes: DEFAULT_MAX_IDENTIFIER_BYTES,
            max_warnings: DEFAULT_MAX_WARNINGS,
            require_unique_circuit_fingerprints: true,
            require_canonical_circuit_order: true,
            require_result_fingerprint: true,
        }
    }

    /// Returns a less restrictive policy intended for planning-only
    /// validation.
    ///
    /// This is still deterministic and bounded.
    #[must_use]
    pub const fn planning() -> Self {
        Self {
            require_current_schema: true,
            require_sha256: true,
            reject_zero_fingerprints: true,
            max_canonical_bytes: DEFAULT_MAX_CANONICAL_BYTES,
            max_circuit_fingerprints: DEFAULT_MAX_CIRCUIT_FINGERPRINTS,
            max_identifier_bytes: DEFAULT_MAX_IDENTIFIER_BYTES,
            max_warnings: DEFAULT_MAX_WARNINGS,
            require_unique_circuit_fingerprints: true,
            require_canonical_circuit_order: true,
            require_result_fingerprint: false,
        }
    }

    /// Validates the policy itself.
    pub fn validate(&self) -> Result<(), ReproducibilityValidationError> {
        if self.max_canonical_bytes == 0 {
            return Err(ReproducibilityValidationError::InvalidPolicy {
                field: "max_canonical_bytes",
                reason: "must be greater than zero",
            });
        }

        if self.max_circuit_fingerprints == 0 {
            return Err(ReproducibilityValidationError::InvalidPolicy {
                field: "max_circuit_fingerprints",
                reason: "must be greater than zero",
            });
        }

        if self.max_identifier_bytes == 0 {
            return Err(ReproducibilityValidationError::InvalidPolicy {
                field: "max_identifier_bytes",
                reason: "must be greater than zero",
            });
        }

        if self.max_warnings == 0 {
            return Err(ReproducibilityValidationError::InvalidPolicy {
                field: "max_warnings",
                reason: "must be greater than zero",
            });
        }

        Ok(())
    }
}

// ============================================================================
// Validation errors
// ============================================================================

/// Errors returned by reproducibility validation.
///
/// The variants are intentionally structured so callers can distinguish
/// malformed data, unsupported schemas, inconsistent identities, and resource
/// violations without parsing error strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReproducibilityValidationError {
    /// The validation policy itself is invalid.
    InvalidPolicy {
        field: &'static str,
        reason: &'static str,
    },

    /// A required reproducibility schema version is unsupported.
    UnsupportedSchemaVersion {
        expected: u16,
        actual: u16,
    },

    /// The configured fingerprint algorithm is unsupported.
    UnsupportedFingerprintAlgorithm {
        expected: &'static str,
        actual: &'static str,
    },

    /// A required textual identifier is empty.
    EmptyIdentifier {
        field: &'static str,
    },

    /// An identifier is too large.
    IdentifierTooLong {
        field: &'static str,
        bytes: usize,
        maximum: usize,
    },

    /// A fingerprint is all zeroes.
    ZeroFingerprint {
        field: &'static str,
    },

    /// A canonical byte representation exceeds the configured bound.
    CanonicalBytesTooLarge {
        field: &'static str,
        bytes: usize,
        maximum: usize,
    },

    /// Too many circuit fingerprints were supplied.
    TooManyCircuitFingerprints {
        count: usize,
        maximum: usize,
    },

    /// A circuit fingerprint occurred more than once.
    DuplicateCircuitFingerprint {
        index: usize,
        fingerprint: Fingerprint,
    },

    /// Circuit ordering was declared non-canonical.
    NonCanonicalCircuitOrder,

    /// A supplied configuration fingerprint does not match the canonical
    /// configuration bytes.
    ConfigurationFingerprintMismatch {
        expected: ConfigurationFingerprint,
        actual: ConfigurationFingerprint,
    },

    /// A supplied circuit fingerprint does not match canonical circuit bytes.
    CircuitFingerprintMismatch {
        expected: CircuitFingerprint,
        actual: CircuitFingerprint,
        index: usize,
    },

    /// A supplied result fingerprint does not match canonical result bytes.
    ResultFingerprintMismatch {
        expected: ResultFingerprint,
        actual: ResultFingerprint,
    },

    /// The supplied experiment identity does not match its canonical inputs.
    ExperimentIdentityMismatch {
        expected: ExperimentIdentity,
        actual: ExperimentIdentity,
    },

    /// The supplied generator descriptor is invalid.
    InvalidGenerator {
        reason: String,
    },

    /// A seed was required but not supplied.
    MissingSeed,

    /// A result fingerprint was required but not supplied.
    MissingResultFingerprint,

    /// A circuit fingerprint list was required but not supplied.
    MissingCircuitFingerprints,

    /// A configuration fingerprint was required but not supplied.
    MissingConfigurationFingerprint,

    /// An experiment identity was required but not supplied.
    MissingExperimentIdentity,

    /// The fingerprint representation is malformed.
    InvalidFingerprintRepresentation {
        field: &'static str,
        expected_bytes: usize,
        actual_bytes: usize,
    },

    /// The validation input was internally inconsistent.
    InconsistentInput {
        field: &'static str,
        reason: &'static str,
    },

    /// Validation generated more warnings than the configured bound.
    TooManyWarnings {
        count: usize,
        maximum: usize,
    },
}

impl fmt::Display for ReproducibilityValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPolicy { field, reason } => {
                write!(formatter, "invalid reproducibility validation policy: {field}: {reason}")
            }

            Self::UnsupportedSchemaVersion { expected, actual } => {
                write!(
                    formatter,
                    "unsupported reproducibility schema version: expected {}, got {}",
                    expected, actual
                )
            }

            Self::UnsupportedFingerprintAlgorithm { expected, actual } => {
                write!(
                    formatter,
                    "unsupported reproducibility fingerprint algorithm: expected {}, got {}",
                    expected, actual
                )
            }

            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} must not be empty")
            }

            Self::IdentifierTooLong {
                field,
                bytes,
                maximum,
            } => {
                write!(
                    formatter,
                    "{field} is {} bytes, exceeding maximum {} bytes",
                    bytes, maximum
                )
            }

            Self::ZeroFingerprint { field } => {
                write!(formatter, "{field} must not be an all-zero fingerprint")
            }

            Self::CanonicalBytesTooLarge {
                field,
                bytes,
                maximum,
            } => {
                write!(
                    formatter,
                    "{field} canonical representation is {} bytes, exceeding maximum {} bytes",
                    bytes, maximum
                )
            }

            Self::TooManyCircuitFingerprints { count, maximum } => {
                write!(
                    formatter,
                    "circuit fingerprint count {} exceeds maximum {}",
                    count, maximum
                )
            }

            Self::DuplicateCircuitFingerprint {
                index,
                fingerprint,
            } => {
                write!(
                    formatter,
                    "duplicate circuit fingerprint at index {}: {}",
                    index, fingerprint
                )
            }

            Self::NonCanonicalCircuitOrder => {
                write!(
                    formatter,
                    "circuit fingerprints are not in the required canonical semantic order"
                )
            }

            Self::ConfigurationFingerprintMismatch { expected, actual } => {
                write!(
                    formatter,
                    "configuration fingerprint mismatch: expected {}, got {}",
                    expected.fingerprint(),
                    actual.fingerprint()
                )
            }

            Self::CircuitFingerprintMismatch {
                expected,
                actual,
                index,
            } => {
                write!(
                    formatter,
                    "circuit fingerprint mismatch at index {}: expected {}, got {}",
                    index,
                    expected.fingerprint(),
                    actual.fingerprint()
                )
            }

            Self::ResultFingerprintMismatch { expected, actual } => {
                write!(
                    formatter,
                    "result fingerprint mismatch: expected {}, got {}",
                    expected.fingerprint(),
                    actual.fingerprint()
                )
            }

            Self::ExperimentIdentityMismatch { expected, actual } => {
                write!(
                    formatter,
                    "experiment identity mismatch: expected {}, got {}",
                    expected, actual
                )
            }

            Self::InvalidGenerator { reason } => {
                write!(formatter, "invalid generator descriptor: {}", reason)
            }

            Self::MissingSeed => {
                write!(formatter, "benchmark seed is required for reproducibility validation")
            }

            Self::MissingResultFingerprint => {
                write!(
                    formatter,
                    "result fingerprint is required for completed-result validation"
                )
            }

            Self::MissingCircuitFingerprints => {
                write!(
                    formatter,
                    "circuit fingerprints are required for completed-experiment validation"
                )
            }

            Self::MissingConfigurationFingerprint => {
                write!(
                    formatter,
                    "configuration fingerprint is required for reproducibility validation"
                )
            }

            Self::MissingExperimentIdentity => {
                write!(
                    formatter,
                    "experiment identity is required for reproducibility validation"
                )
            }

            Self::InvalidFingerprintRepresentation {
                field,
                expected_bytes,
                actual_bytes,
            } => {
                write!(
                    formatter,
                    "invalid fingerprint representation for {}: expected {} bytes, got {}",
                    field, expected_bytes, actual_bytes
                )
            }

            Self::InconsistentInput { field, reason } => {
                write!(
                    formatter,
                    "inconsistent reproducibility input {}: {}",
                    field, reason
                )
            }

            Self::TooManyWarnings { count, maximum } => {
                write!(
                    formatter,
                    "validation produced {} warnings, exceeding maximum {}",
                    count, maximum
                )
            }
        }
    }
}

impl Error for ReproducibilityValidationError {}

// ============================================================================
// Validation warnings
// ============================================================================

/// Non-fatal reproducibility warning.
///
/// Warnings are intentionally structured and bounded. Production callers can
/// therefore expose them in reports without parsing human-readable messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReproducibilityWarning {
    /// A zero seed is technically valid but may indicate accidental defaulting.
    ZeroSeed,

    /// No circuit fingerprints were supplied because the validation was
    /// performed in planning mode.
    CircuitFingerprintsDeferred,

    /// Result validation was intentionally deferred.
    ResultFingerprintDeferred,
}

impl fmt::Display for ReproducibilityWarning {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroSeed => {
                write!(
                    formatter,
                    "benchmark seed is zero; this is valid but may indicate accidental defaulting"
                )
            }

            Self::CircuitFingerprintsDeferred => {
                write!(
                    formatter,
                    "circuit fingerprint validation was deferred because no completed circuit set was supplied"
                )
            }

            Self::ResultFingerprintDeferred => {
                write!(
                    formatter,
                    "result fingerprint validation was deferred because this is not completed-result validation"
                )
            }
        }
    }
}

// ============================================================================
// Validation input
// ============================================================================

/// Canonical circuit input used to validate a circuit fingerprint.
///
/// The validator does not inspect the circuit itself. The caller supplies the
/// canonical representation that is authoritative for that circuit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalCircuit<'a> {
    /// Canonical serialized circuit bytes.
    pub bytes: &'a [u8],

    /// Fingerprint recorded for the circuit.
    pub fingerprint: CircuitFingerprint,
}

impl<'a> CanonicalCircuit<'a> {
    /// Creates a canonical circuit validation item.
    #[must_use]
    pub const fn new(
        bytes: &'a [u8],
        fingerprint: CircuitFingerprint,
    ) -> Self {
        Self { bytes, fingerprint }
    }
}

/// Complete reproducibility validation input.
///
/// This structure intentionally uses the existing reproducibility primitives
/// instead of introducing parallel fingerprint types.
///
/// `configuration_bytes` are the canonical configuration bytes used to
/// calculate `configuration`.
///
/// `circuits` must be supplied in the benchmark's canonical semantic order.
///
/// `result_bytes` are the canonical result bytes used to calculate
/// `result_fingerprint`.
#[derive(Debug, Clone)]
pub struct ReproducibilityValidationInput<'a> {
    /// Reproducibility schema version declared by the producer.
    pub schema_version: u16,

    /// Fingerprint algorithm declared by the producer.
    pub fingerprint_algorithm: &'a str,

    /// Stable benchmark identifier.
    pub benchmark_id: &'a str,

    /// Stable benchmark protocol/version identifier.
    pub benchmark_version: &'a str,

    /// Deterministic generator descriptor.
    pub generator: &'a GeneratorDescriptor,

    /// Deterministic experiment seed.
    pub seed: Option<BenchmarkSeed>,

    /// Canonical benchmark configuration bytes.
    pub configuration_bytes: Option<&'a [u8]>,

    /// Declared configuration fingerprint.
    pub configuration: Option<ConfigurationFingerprint>,

    /// Canonical circuits in semantic execution order.
    pub circuits: Option<&'a [CanonicalCircuit<'a>]>,

    /// Declared experiment identity.
    pub experiment: Option<ExperimentIdentity>,

    /// Canonical result bytes.
    pub result_bytes: Option<&'a [u8]>,

    /// Declared result fingerprint.
    pub result_fingerprint: Option<ResultFingerprint>,

    /// Whether the supplied circuit order is explicitly known to be canonical.
    ///
    /// `None` means the caller did not establish ordering.
    pub canonical_circuit_order: Option<bool>,
}

impl<'a> ReproducibilityValidationInput<'a> {
    /// Creates a minimal configuration-level validation input.
    ///
    /// This is useful before circuits exist.
    pub fn configuration_only(
        schema_version: u16,
        fingerprint_algorithm: &'a str,
        benchmark_id: &'a str,
        benchmark_version: &'a str,
        generator: &'a GeneratorDescriptor,
        seed: BenchmarkSeed,
        configuration_bytes: &'a [u8],
        configuration: ConfigurationFingerprint,
        experiment: ExperimentIdentity,
    ) -> Self {
        Self {
            schema_version,
            fingerprint_algorithm,
            benchmark_id,
            benchmark_version,
            generator,
            seed: Some(seed),
            configuration_bytes: Some(configuration_bytes),
            configuration: Some(configuration),
            circuits: None,
            experiment: Some(experiment),
            result_bytes: None,
            result_fingerprint: None,
            canonical_circuit_order: None,
        }
    }

    /// Creates a completed-result validation input.
    pub fn completed(
        schema_version: u16,
        fingerprint_algorithm: &'a str,
        benchmark_id: &'a str,
        benchmark_version: &'a str,
        generator: &'a GeneratorDescriptor,
        seed: BenchmarkSeed,
        configuration_bytes: &'a [u8],
        configuration: ConfigurationFingerprint,
        circuits: &'a [CanonicalCircuit<'a>],
        experiment: ExperimentIdentity,
        result_bytes: &'a [u8],
        result_fingerprint: ResultFingerprint,
    ) -> Self {
        Self {
            schema_version,
            fingerprint_algorithm,
            benchmark_id,
            benchmark_version,
            generator,
            seed: Some(seed),
            configuration_bytes: Some(configuration_bytes),
            configuration: Some(configuration),
            circuits: Some(circuits),
            experiment: Some(experiment),
            result_bytes: Some(result_bytes),
            result_fingerprint: Some(result_fingerprint),
            canonical_circuit_order: Some(true),
        }
    }
}

// ============================================================================
// Validation report
// ============================================================================

/// Successful reproducibility validation report.
///
/// A successful report is evidence that the supplied metadata passed this
/// validator under the selected policy.
///
/// It is NOT a cryptographic attestation that the experiment itself was
/// honestly executed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproducibilityValidationReport {
    /// Validation contract version.
    pub validation_version: u16,

    /// Reproducibility schema version validated.
    pub schema_version: u16,

    /// Fingerprint algorithm validated.
    pub fingerprint_algorithm: &'static str,

    /// Stable benchmark identifier.
    pub benchmark_id: String,

    /// Benchmark protocol/version identifier.
    pub benchmark_version: String,

    /// Validated generator fingerprint.
    pub generator_fingerprint: Fingerprint,

    /// Validated seed.
    pub seed: BenchmarkSeed,

    /// Validated configuration fingerprint.
    pub configuration_fingerprint: ConfigurationFingerprint,

    /// Validated experiment identity.
    pub experiment_identity: ExperimentIdentity,

    /// Number of validated circuits.
    pub circuit_count: usize,

    /// Validated result fingerprint, when present.
    pub result_fingerprint: Option<ResultFingerprint>,

    /// Non-fatal warnings.
    pub warnings: Vec<ReproducibilityWarning>,
}

impl ReproducibilityValidationReport {
    /// Returns true when the report represents a completed-result validation.
    #[must_use]
    pub fn is_completed_result(&self) -> bool {
        self.result_fingerprint.is_some()
    }

    /// Returns the number of warnings.
    #[must_use]
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }

    /// Returns true when validation completed without warnings.
    #[must_use]
    pub fn is_warning_free(&self) -> bool {
        self.warnings.is_empty()
    }
}

// ============================================================================
// Validator
// ============================================================================

/// Production reproducibility validator.
///
/// The validator contains no mutable global state and is safe to instantiate
/// independently for every benchmark.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReproducibilityValidator {
    policy: ReproducibilityValidationPolicy,
}

impl Default for ReproducibilityValidator {
    fn default() -> Self {
        Self::production()
    }
}

impl ReproducibilityValidator {
    /// Creates the production validator.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            policy: ReproducibilityValidationPolicy::production(),
        }
    }

    /// Creates a validator using an explicit policy.
    pub const fn new(policy: ReproducibilityValidationPolicy) -> Self {
        Self { policy }
    }

    /// Returns the validator policy.
    #[must_use]
    pub const fn policy(&self) -> &ReproducibilityValidationPolicy {
        &self.policy
    }

    /// Validates the configured policy.
    pub fn validate_policy(&self) -> Result<(), ReproducibilityValidationError> {
        self.policy.validate()
    }

    /// Validates a complete reproducibility input.
    ///
    /// This method performs all checks necessary to establish that the
    /// supplied identity and canonical representations agree.
    pub fn validate(
        &self,
        input: &ReproducibilityValidationInput<'_>,
    ) -> Result<ReproducibilityValidationReport, ReproducibilityValidationError> {
        self.validate_policy()?;

        self.validate_schema(
            input.schema_version,
            input.fingerprint_algorithm,
        )?;

        self.validate_identifier(
            "benchmark_id",
            input.benchmark_id,
        )?;

        self.validate_identifier(
            "benchmark_version",
            input.benchmark_version,
        )?;

        self.validate_generator(input.generator)?;

        let seed = input
            .seed
            .ok_or(ReproducibilityValidationError::MissingSeed)?;

        let mut warnings = Vec::new();

        if seed.value() == 0 {
            self.push_warning(
                &mut warnings,
                ReproducibilityWarning::ZeroSeed,
            )?;
        }

        let configuration_bytes = input.configuration_bytes.ok_or(
            ReproducibilityValidationError::MissingConfigurationFingerprint,
        )?;

        self.validate_canonical_bytes(
            "configuration",
            configuration_bytes,
        )?;

        let supplied_configuration = input.configuration.ok_or(
            ReproducibilityValidationError::MissingConfigurationFingerprint,
        )?;

        self.validate_fingerprint(
            "configuration fingerprint",
            supplied_configuration.fingerprint(),
        )?;

        let expected_configuration =
            ConfigurationFingerprint::from_canonical_bytes(configuration_bytes);

        if expected_configuration != supplied_configuration {
            return Err(
                ReproducibilityValidationError::ConfigurationFingerprintMismatch {
                    expected: expected_configuration,
                    actual: supplied_configuration,
                },
            );
        }

        let supplied_experiment = input.experiment.ok_or(
            ReproducibilityValidationError::MissingExperimentIdentity,
        )?;

        let expected_experiment =
            ExperimentIdentity::from_canonical_bytes(
                input.benchmark_id,
                input.benchmark_version,
                input.generator,
                seed,
                &supplied_configuration.fingerprint(),
            )
            .map_err(|error| {
                ReproducibilityValidationError::InvalidGenerator {
                    reason: error.to_string(),
                }
            })?;

        if expected_experiment != supplied_experiment {
            return Err(
                ReproducibilityValidationError::ExperimentIdentityMismatch {
                    expected: expected_experiment,
                    actual: supplied_experiment,
                },
            );
        }

        let circuit_count = match input.circuits {
            Some(circuits) => {
                self.validate_circuits(
                    circuits,
                    input.canonical_circuit_order,
                    &mut warnings,
                )?
            }

            None => {
                self.push_warning(
                    &mut warnings,
                    ReproducibilityWarning::CircuitFingerprintsDeferred,
                )?;

                0
            }
        };

        let result_fingerprint = match (
            input.result_bytes,
            input.result_fingerprint,
        ) {
            (Some(result_bytes), Some(supplied_result)) => {
                self.validate_canonical_bytes(
                    "result",
                    result_bytes,
                )?;

                self.validate_fingerprint(
                    "result fingerprint",
                    supplied_result.fingerprint(),
                )?;

                let expected_result =
                    ResultFingerprint::from_canonical_bytes(result_bytes);

                if expected_result != supplied_result {
                    return Err(
                        ReproducibilityValidationError::ResultFingerprintMismatch {
                            expected: expected_result,
                            actual: supplied_result,
                        },
                    );
                }

                Some(supplied_result)
            }

            (None, None) => {
                if self.policy.require_result_fingerprint {
                    return Err(
                        ReproducibilityValidationError::MissingResultFingerprint,
                    );
                }

                self.push_warning(
                    &mut warnings,
                    ReproducibilityWarning::ResultFingerprintDeferred,
                )?;

                None
            }

            (Some(_), None) => {
                return Err(
                    ReproducibilityValidationError::MissingResultFingerprint,
                );
            }

            (None, Some(_)) => {
                return Err(
                    ReproducibilityValidationError::InconsistentInput {
                        field: "result",
                        reason: "result fingerprint was supplied without canonical result bytes",
                    },
                );
            }
        };

        Ok(ReproducibilityValidationReport {
            validation_version: REPRODUCIBILITY_VALIDATION_VERSION,
            schema_version: input.schema_version,
            fingerprint_algorithm: FINGERPRINT_ALGORITHM,
            benchmark_id: input.benchmark_id.to_owned(),
            benchmark_version: input.benchmark_version.to_owned(),
            generator_fingerprint: input.generator.fingerprint(),
            seed,
            configuration_fingerprint: supplied_configuration,
            experiment_identity: supplied_experiment,
            circuit_count,
            result_fingerprint,
            warnings,
        })
    }

    /// Validates only the canonical configuration/experiment identity layer.
    ///
    /// This is the appropriate API for benchmark planning before circuits have
    /// been generated.
    pub fn validate_planned_experiment(
        &self,
        input: &ReproducibilityValidationInput<'_>,
    ) -> Result<ReproducibilityValidationReport, ReproducibilityValidationError> {
        let planning_policy = ReproducibilityValidationPolicy::planning();
        let validator = Self::new(planning_policy);

        validator.validate(input)
    }

    /// Validates a configuration fingerprint against canonical bytes.
    pub fn validate_configuration(
        &self,
        canonical_bytes: &[u8],
        fingerprint: ConfigurationFingerprint,
    ) -> Result<(), ReproducibilityValidationError> {
        self.validate_policy()?;

        self.validate_canonical_bytes(
            "configuration",
            canonical_bytes,
        )?;

        self.validate_fingerprint(
            "configuration fingerprint",
            fingerprint.fingerprint(),
        )?;

        let expected =
            ConfigurationFingerprint::from_canonical_bytes(canonical_bytes);

        if expected != fingerprint {
            return Err(
                ReproducibilityValidationError::ConfigurationFingerprintMismatch {
                    expected,
                    actual: fingerprint,
                },
            );
        }

        Ok(())
    }

    /// Validates one circuit fingerprint against its canonical bytes.
    pub fn validate_circuit(
        &self,
        index: usize,
        canonical_bytes: &[u8],
        fingerprint: CircuitFingerprint,
    ) -> Result<(), ReproducibilityValidationError> {
        self.validate_policy()?;

        self.validate_canonical_bytes(
            "circuit",
            canonical_bytes,
        )?;

        self.validate_fingerprint(
            "circuit fingerprint",
            fingerprint.fingerprint(),
        )?;

        let expected =
            CircuitFingerprint::from_canonical_bytes(canonical_bytes);

        if expected != fingerprint {
            return Err(
                ReproducibilityValidationError::CircuitFingerprintMismatch {
                    expected,
                    actual: fingerprint,
                    index,
                },
            );
        }

        Ok(())
    }

    /// Validates a result fingerprint against canonical result bytes.
    pub fn validate_result(
        &self,
        canonical_bytes: &[u8],
        fingerprint: ResultFingerprint,
    ) -> Result<(), ReproducibilityValidationError> {
        self.validate_policy()?;

        self.validate_canonical_bytes(
            "result",
            canonical_bytes,
        )?;

        self.validate_fingerprint(
            "result fingerprint",
            fingerprint.fingerprint(),
        )?;

        let expected =
            ResultFingerprint::from_canonical_bytes(canonical_bytes);

        if expected != fingerprint {
            return Err(
                ReproducibilityValidationError::ResultFingerprintMismatch {
                    expected,
                    actual: fingerprint,
                },
            );
        }

        Ok(())
    }

    /// Validates an experiment identity against its declared canonical
    /// components.
    pub fn validate_experiment_identity(
        &self,
        benchmark_id: &str,
        benchmark_version: &str,
        generator: &GeneratorDescriptor,
        seed: BenchmarkSeed,
        configuration: ConfigurationFingerprint,
        experiment: ExperimentIdentity,
    ) -> Result<(), ReproducibilityValidationError> {
        self.validate_policy()?;

        self.validate_identifier(
            "benchmark_id",
            benchmark_id,
        )?;

        self.validate_identifier(
            "benchmark_version",
            benchmark_version,
        )?;

        self.validate_generator(generator)?;

        self.validate_fingerprint(
            "configuration fingerprint",
            configuration.fingerprint(),
        )?;

        let expected =
            ExperimentIdentity::from_canonical_bytes(
                benchmark_id,
                benchmark_version,
                generator,
                seed,
                &configuration.fingerprint(),
            )
            .map_err(|error| {
                ReproducibilityValidationError::InvalidGenerator {
                    reason: error.to_string(),
                }
            })?;

        if expected != experiment {
            return Err(
                ReproducibilityValidationError::ExperimentIdentityMismatch {
                    expected,
                    actual: experiment,
                },
            );
        }

        Ok(())
    }

    /// Validates a circuit fingerprint collection.
    ///
    /// This performs:
    ///
    /// - cardinality validation;
    /// - individual fingerprint validation;
    /// - duplicate detection;
    /// - canonical ordering declaration validation.
    pub fn validate_circuit_fingerprints(
        &self,
        fingerprints: &[CircuitFingerprint],
        canonical_order: Option<bool>,
    ) -> Result<(), ReproducibilityValidationError> {
        self.validate_policy()?;

        if fingerprints.len() > self.policy.max_circuit_fingerprints {
            return Err(
                ReproducibilityValidationError::TooManyCircuitFingerprints {
                    count: fingerprints.len(),
                    maximum: self.policy.max_circuit_fingerprints,
                },
            );
        }

        if self.policy.require_canonical_circuit_order
            && canonical_order != Some(true)
        {
            return Err(
                ReproducibilityValidationError::NonCanonicalCircuitOrder,
            );
        }

        let mut seen =
            HashSet::with_capacity(fingerprints.len().min(1024));

        for (index, fingerprint) in fingerprints.iter().enumerate() {
            self.validate_fingerprint(
                "circuit fingerprint",
                fingerprint.fingerprint(),
            )?;

            if self.policy.require_unique_circuit_fingerprints
                && !seen.insert(*fingerprint)
            {
                return Err(
                    ReproducibilityValidationError::DuplicateCircuitFingerprint {
                        index,
                        fingerprint: fingerprint.fingerprint(),
                    },
                );
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Internal validation
    // ------------------------------------------------------------------------

    fn validate_schema(
        &self,
        schema_version: u16,
        fingerprint_algorithm: &str,
    ) -> Result<(), ReproducibilityValidationError> {
        if self.policy.require_current_schema
            && schema_version != REPRODUCIBILITY_SCHEMA_VERSION
        {
            return Err(
                ReproducibilityValidationError::UnsupportedSchemaVersion {
                    expected: REPRODUCIBILITY_SCHEMA_VERSION,
                    actual: schema_version,
                },
            );
        }

        if self.policy.require_sha256
            && fingerprint_algorithm != FINGERPRINT_ALGORITHM
        {
            return Err(
                ReproducibilityValidationError::UnsupportedFingerprintAlgorithm {
                    expected: FINGERPRINT_ALGORITHM,
                    actual: FINGERPRINT_ALGORITHM,
                },
            );
        }

        Ok(())
    }

    fn validate_identifier(
        &self,
        field: &'static str,
        value: &str,
    ) -> Result<(), ReproducibilityValidationError> {
        if value.is_empty() {
            return Err(
                ReproducibilityValidationError::EmptyIdentifier { field },
            );
        }

        let length = value.as_bytes().len();

        if length > self.policy.max_identifier_bytes {
            return Err(
                ReproducibilityValidationError::IdentifierTooLong {
                    field,
                    bytes: length,
                    maximum: self.policy.max_identifier_bytes,
                },
            );
        }

        Ok(())
    }

    fn validate_generator(
        &self,
        generator: &GeneratorDescriptor,
    ) -> Result<(), ReproducibilityValidationError> {
        generator
            .validate()
            .map_err(|error| {
                ReproducibilityValidationError::InvalidGenerator {
                    reason: error.to_string(),
                }
            })?;

        self.validate_identifier(
            "generator.id",
            &generator.id,
        )?;

        self.validate_identifier(
            "generator.version",
            &generator.version,
        )?;

        if let Some(rng_algorithm) =
            generator.rng_algorithm.as_deref()
        {
            self.validate_identifier(
                "generator.rng_algorithm",
                rng_algorithm,
            )?;
        }

        Ok(())
    }

    fn validate_canonical_bytes(
        &self,
        field: &'static str,
        bytes: &[u8],
    ) -> Result<(), ReproducibilityValidationError> {
        if bytes.len() > self.policy.max_canonical_bytes {
            return Err(
                ReproducibilityValidationError::CanonicalBytesTooLarge {
                    field,
                    bytes: bytes.len(),
                    maximum: self.policy.max_canonical_bytes,
                },
            );
        }

        Ok(())
    }

    fn validate_fingerprint(
        &self,
        field: &'static str,
        fingerprint: Fingerprint,
    ) -> Result<(), ReproducibilityValidationError> {
        // Fingerprint is a fixed-size type, so malformed length is impossible
        // at the Rust type boundary. Keep this explicit in the contract so
        // future deserialization adapters can map malformed external data
        // into the same semantic error.
        let actual_bytes = fingerprint.as_bytes().len();

        if actual_bytes != FINGERPRINT_BYTES {
            return Err(
                ReproducibilityValidationError::InvalidFingerprintRepresentation {
                    field,
                    expected_bytes: FINGERPRINT_BYTES,
                    actual_bytes,
                },
            );
        }

        if self.policy.reject_zero_fingerprints
            && fingerprint.is_zero()
        {
            return Err(
                ReproducibilityValidationError::ZeroFingerprint { field },
            );
        }

        Ok(())
    }

    fn validate_circuits(
        &self,
        circuits: &[CanonicalCircuit<'_>],
        canonical_order: Option<bool>,
        warnings: &mut Vec<ReproducibilityWarning>,
    ) -> Result<usize, ReproducibilityValidationError> {
        if circuits.len() > self.policy.max_circuit_fingerprints {
            return Err(
                ReproducibilityValidationError::TooManyCircuitFingerprints {
                    count: circuits.len(),
                    maximum: self.policy.max_circuit_fingerprints,
                },
            );
        }

        if self.policy.require_canonical_circuit_order
            && canonical_order != Some(true)
        {
            return Err(
                ReproducibilityValidationError::NonCanonicalCircuitOrder,
            );
        }

        let mut seen =
            HashSet::with_capacity(circuits.len().min(1024));

        for (index, circuit) in circuits.iter().enumerate() {
            self.validate_canonical_bytes(
                "circuit",
                circuit.bytes,
            )?;

            self.validate_fingerprint(
                "circuit fingerprint",
                circuit.fingerprint.fingerprint(),
            )?;

            let expected =
                CircuitFingerprint::from_canonical_bytes(
                    circuit.bytes,
                );

            if expected != circuit.fingerprint {
                return Err(
                    ReproducibilityValidationError::CircuitFingerprintMismatch {
                        expected,
                        actual: circuit.fingerprint,
                        index,
                    },
                );
            }

            if self.policy.require_unique_circuit_fingerprints
                && !seen.insert(circuit.fingerprint)
            {
                return Err(
                    ReproducibilityValidationError::DuplicateCircuitFingerprint {
                        index,
                        fingerprint: circuit.fingerprint.fingerprint(),
                    },
                );
            }
        }

        // A non-empty circuit list is the normal completed-experiment case.
        // There is intentionally no warning for zero circuits when the caller
        // explicitly supplied an empty canonical list: some valid benchmarks
        // can have a planning/metadata-only phase.
        let _ = warnings;

        Ok(circuits.len())
    }

    fn push_warning(
        &self,
        warnings: &mut Vec<ReproducibilityWarning>,
        warning: ReproducibilityWarning,
    ) -> Result<(), ReproducibilityValidationError> {
        if warnings.len() >= self.policy.max_warnings {
            return Err(
                ReproducibilityValidationError::TooManyWarnings {
                    count: warnings.len() + 1,
                    maximum: self.policy.max_warnings,
                },
            );
        }

        warnings.push(warning);

        Ok(())
    }
}

// ============================================================================
// Convenience functions
// ============================================================================

/// Validates a complete reproducibility input using the production policy.
pub fn validate_reproducibility(
    input: &ReproducibilityValidationInput<'_>,
) -> Result<ReproducibilityValidationReport, ReproducibilityValidationError> {
    ReproducibilityValidator::production().validate(input)
}

/// Validates a configuration fingerprint using the production policy.
pub fn validate_configuration_fingerprint(
    canonical_bytes: &[u8],
    fingerprint: ConfigurationFingerprint,
) -> Result<(), ReproducibilityValidationError> {
    ReproducibilityValidator::production()
        .validate_configuration(canonical_bytes, fingerprint)
}

/// Validates a circuit fingerprint using the production policy.
pub fn validate_circuit_fingerprint(
    canonical_bytes: &[u8],
    fingerprint: CircuitFingerprint,
) -> Result<(), ReproducibilityValidationError> {
    ReproducibilityValidator::production()
        .validate_circuit(0, canonical_bytes, fingerprint)
}

/// Validates a result fingerprint using the production policy.
pub fn validate_result_fingerprint(
    canonical_bytes: &[u8],
    fingerprint: ResultFingerprint,
) -> Result<(), ReproducibilityValidationError> {
    ReproducibilityValidator::production()
        .validate_result(canonical_bytes, fingerprint)
}

/// Validates a benchmark experiment identity using the production policy.
pub fn validate_experiment_identity(
    benchmark_id: &str,
    benchmark_version: &str,
    generator: &GeneratorDescriptor,
    seed: BenchmarkSeed,
    configuration: ConfigurationFingerprint,
    experiment: ExperimentIdentity,
) -> Result<(), ReproducibilityValidationError> {
    ReproducibilityValidator::production()
        .validate_experiment_identity(
            benchmark_id,
            benchmark_version,
            generator,
            seed,
            configuration,
            experiment,
        )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::benchmarking::core::reproducibility::{
        BenchmarkSeed,
        CircuitFingerprint,
        ConfigurationFingerprint,
        ExperimentIdentity,
        GeneratorDescriptor,
        ResultFingerprint,
        REPRODUCIBILITY_SCHEMA_VERSION,
    };

    fn generator() -> GeneratorDescriptor {
        GeneratorDescriptor::new(
            "test-generator",
            "1",
        )
        .with_rng_algorithm("test-rng-v1")
    }

    fn configuration_bytes() -> &'static [u8] {
        b"benchmark-config-v1"
    }

    fn result_bytes() -> &'static [u8] {
        b"benchmark-result-v1"
    }

    fn configuration_fingerprint() -> ConfigurationFingerprint {
        ConfigurationFingerprint::from_canonical_bytes(
            configuration_bytes(),
        )
    }

    fn experiment_identity() -> ExperimentIdentity {
        ExperimentIdentity::from_canonical_bytes(
            "test-benchmark",
            "1",
            &generator(),
            BenchmarkSeed::new(42),
            &configuration_fingerprint().fingerprint(),
        )
        .expect("test identity must be constructible")
    }

    #[test]
    fn production_policy_is_valid() {
        assert!(
            ReproducibilityValidationPolicy::production()
                .validate()
                .is_ok()
        );
    }

    #[test]
    fn configuration_fingerprint_is_accepted_when_matching() {
        let validator = ReproducibilityValidator::production();

        assert!(
            validator
                .validate_configuration(
                    configuration_bytes(),
                    configuration_fingerprint(),
                )
                .is_ok()
        );
    }

    #[test]
    fn configuration_fingerprint_mismatch_is_rejected() {
        let validator = ReproducibilityValidator::production();

        let wrong =
            ConfigurationFingerprint::from_canonical_bytes(
                b"different-config",
            );

        let result = validator.validate_configuration(
            configuration_bytes(),
            wrong,
        );

        assert!(matches!(
            result,
            Err(
                ReproducibilityValidationError::ConfigurationFingerprintMismatch {
                    ..
                }
            )
        ));
    }

    #[test]
    fn circuit_fingerprint_is_accepted_when_matching() {
        let validator = ReproducibilityValidator::production();
        let bytes = b"canonical-circuit";

        let fingerprint =
            CircuitFingerprint::from_canonical_bytes(bytes);

        assert!(
            validator
                .validate_circuit(
                    0,
                    bytes,
                    fingerprint,
                )
                .is_ok()
        );
    }

    #[test]
    fn circuit_fingerprint_mismatch_is_rejected() {
        let validator = ReproducibilityValidator::production();

        let actual =
            CircuitFingerprint::from_canonical_bytes(
                b"wrong-circuit",
            );

        let result = validator.validate_circuit(
            7,
            b"canonical-circuit",
            actual,
        );

        assert!(matches!(
            result,
            Err(
                ReproducibilityValidationError::CircuitFingerprintMismatch {
                    index: 7,
                    ..
                }
            )
        ));
    }

    #[test]
    fn result_fingerprint_is_accepted_when_matching() {
        let validator = ReproducibilityValidator::production();

        let fingerprint =
            ResultFingerprint::from_canonical_bytes(
                result_bytes(),
            );

        assert!(
            validator
                .validate_result(
                    result_bytes(),
                    fingerprint,
                )
                .is_ok()
        );
    }

    #[test]
    fn result_fingerprint_mismatch_is_rejected() {
        let validator = ReproducibilityValidator::production();

        let fingerprint =
            ResultFingerprint::from_canonical_bytes(
                b"wrong-result",
            );

        let result = validator.validate_result(
            result_bytes(),
            fingerprint,
        );

        assert!(matches!(
            result,
            Err(
                ReproducibilityValidationError::ResultFingerprintMismatch {
                    ..
                }
            )
        ));
    }

    #[test]
    fn experiment_identity_is_accepted_when_matching() {
        assert!(
            validate_experiment_identity(
                "test-benchmark",
                "1",
                &generator(),
                BenchmarkSeed::new(42),
                configuration_fingerprint(),
                experiment_identity(),
            )
            .is_ok()
        );
    }

    #[test]
    fn experiment_identity_mismatch_is_rejected() {
        let wrong_configuration =
            ConfigurationFingerprint::from_canonical_bytes(
                b"wrong",
            );

        let result = validate_experiment_identity(
            "test-benchmark",
            "1",
            &generator(),
            BenchmarkSeed::new(42),
            wrong_configuration,
            experiment_identity(),
        );

        assert!(matches!(
            result,
            Err(
                ReproducibilityValidationError::ExperimentIdentityMismatch {
                    ..
                }
            )
        ));
    }

    #[test]
    fn zero_fingerprint_is_rejected() {
        let validator = ReproducibilityValidator::production();

        let zero = Fingerprint::default();

        let result = validator.validate_fingerprint(
            "test",
            zero,
        );

        assert!(matches!(
            result,
            Err(
                ReproducibilityValidationError::ZeroFingerprint {
                    field: "test"
                }
            )
        ));
    }

    #[test]
    fn duplicate_circuit_fingerprints_are_rejected() {
        let validator = ReproducibilityValidator::production();

        let fingerprint =
            CircuitFingerprint::from_canonical_bytes(
                b"same-circuit",
            );

        let circuits = vec![
            fingerprint,
            fingerprint,
        ];

        let result =
            validator.validate_circuit_fingerprints(
                &circuits,
                Some(true),
            );

        assert!(matches!(
            result,
            Err(
                ReproducibilityValidationError::DuplicateCircuitFingerprint {
                    index: 1,
                    ..
                }
            )
        ));
    }

    #[test]
    fn noncanonical_circuit_order_is_rejected() {
        let validator = ReproducibilityValidator::production();

        let fingerprint =
            CircuitFingerprint::from_canonical_bytes(
                b"circuit",
            );

        let result =
            validator.validate_circuit_fingerprints(
                &[fingerprint],
                Some(false),
            );

        assert!(matches!(
            result,
            Err(
                ReproducibilityValidationError::NonCanonicalCircuitOrder
            )
        ));
    }

    #[test]
    fn missing_canonical_order_is_rejected_in_production() {
        let validator = ReproducibilityValidator::production();

        let fingerprint =
            CircuitFingerprint::from_canonical_bytes(
                b"circuit",
            );

        let result =
            validator.validate_circuit_fingerprints(
                &[fingerprint],
                None,
            );

        assert!(matches!(
            result,
            Err(
                ReproducibilityValidationError::NonCanonicalCircuitOrder
            )
        ));
    }

    #[test]
    fn schema_version_is_checked() {
        let validator = ReproducibilityValidator::production();

        let result = validator.validate_schema(
            REPRODUCIBILITY_SCHEMA_VERSION + 1,
            FINGERPRINT_ALGORITHM,
        );

        assert!(matches!(
            result,
            Err(
                ReproducibilityValidationError::UnsupportedSchemaVersion {
                    ..
                }
            )
        ));
    }

    #[test]
    fn sha256_algorithm_is_required() {
        let validator = ReproducibilityValidator::production();

        let result = validator.validate_schema(
            REPRODUCIBILITY_SCHEMA_VERSION,
            "sha3-256",
        );

        assert!(matches!(
            result,
            Err(
                ReproducibilityValidationError::UnsupportedFingerprintAlgorithm {
                    ..
                }
            )
        ));
    }

    #[test]
    fn oversized_canonical_data_is_rejected() {
        let policy = ReproducibilityValidationPolicy {
            max_canonical_bytes: 4,
            ..ReproducibilityValidationPolicy::production()
        };

        let validator =
            ReproducibilityValidator::new(policy);

        let result =
            validator.validate_canonical_bytes(
                "test",
                b"12345",
            );

        assert!(matches!(
            result,
            Err(
                ReproducibilityValidationError::CanonicalBytesTooLarge {
                    bytes: 5,
                    maximum: 4,
                    ..
                }
            )
        ));
    }

    #[test]
    fn oversized_circuit_collection_is_rejected() {
        let policy = ReproducibilityValidationPolicy {
            max_circuit_fingerprints: 1,
            ..ReproducibilityValidationPolicy::production()
        };

        let validator =
            ReproducibilityValidator::new(policy);

        let a =
            CircuitFingerprint::from_canonical_bytes(b"a");
        let b =
            CircuitFingerprint::from_canonical_bytes(b"b");

        let result =
            validator.validate_circuit_fingerprints(
                &[a, b],
                Some(true),
            );

        assert!(matches!(
            result,
            Err(
                ReproducibilityValidationError::TooManyCircuitFingerprints {
                    count: 2,
                    maximum: 1
                }
            )
        ));
    }

    #[test]
    fn planning_validation_can_defer_result_fingerprint() {
        let input =
            ReproducibilityValidationInput::configuration_only(
                REPRODUCIBILITY_SCHEMA_VERSION,
                FINGERPRINT_ALGORITHM,
                "test-benchmark",
                "1",
                &generator(),
                BenchmarkSeed::new(42),
                configuration_bytes(),
                configuration_fingerprint(),
                experiment_identity(),
            );

        let result =
            ReproducibilityValidator::production()
                .validate_planned_experiment(&input)
                .expect("planning validation should succeed");

        assert!(!result.is_completed_result());
    }

    #[test]
    fn completed_validation_requires_matching_everything() {
        let circuit_bytes = b"canonical-circuit";

        let circuit_fingerprint =
            CircuitFingerprint::from_canonical_bytes(
                circuit_bytes,
            );

        let circuit =
            CanonicalCircuit::new(
                circuit_bytes,
                circuit_fingerprint,
            );

        let circuits = [circuit];

        let result_fingerprint =
            ResultFingerprint::from_canonical_bytes(
                result_bytes(),
            );

        let input =
            ReproducibilityValidationInput::completed(
                REPRODUCIBILITY_SCHEMA_VERSION,
                FINGERPRINT_ALGORITHM,
                "test-benchmark",
                "1",
                &generator(),
                BenchmarkSeed::new(42),
                configuration_bytes(),
                configuration_fingerprint(),
                &circuits,
                experiment_identity(),
                result_bytes(),
                result_fingerprint,
            );

        let result =
            ReproducibilityValidator::production()
                .validate(&input)
                .expect("complete validation should succeed");

        assert_eq!(result.circuit_count, 1);
        assert!(result.is_completed_result());
    }

    #[test]
    fn zero_seed_is_warning_not_error() {
        let input =
            ReproducibilityValidationInput::configuration_only(
                REPRODUCIBILITY_SCHEMA_VERSION,
                FINGERPRINT_ALGORITHM,
                "test-benchmark",
                "1",
                &generator(),
                BenchmarkSeed::new(0),
                configuration_bytes(),
                configuration_fingerprint(),
                experiment_identity_for_seed(0),
            );

        let result =
            ReproducibilityValidator::production()
                .validate_planned_experiment(&input)
                .expect("zero seed remains reproducible");

        assert!(
            result
                .warnings
                .contains(&ReproducibilityWarning::ZeroSeed)
        );
    }

    fn experiment_identity_for_seed(
        seed: u64,
    ) -> ExperimentIdentity {
        ExperimentIdentity::from_canonical_bytes(
            "test-benchmark",
            "1",
            &generator(),
            BenchmarkSeed::new(seed),
            &configuration_fingerprint().fingerprint(),
        )
        .expect("test identity must be constructible")
    }
}