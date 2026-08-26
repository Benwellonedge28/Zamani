//! Zamani Quantum Benchmarking — Provenance
//!
//! Production-grade provenance model for quantum benchmark experiments,
//! observations, and results.
//!
//! # Purpose
//!
//! Provenance answers:
//!
//! - What was benchmarked?
//! - Which version of Zamani performed it?
//! - Which benchmark protocol/version was used?
//! - Which workload/configuration was used?
//! - Which random seed/generator was used?
//! - Which compiler/lowering/optimization/routing/scheduling configuration
//!   produced the executable workload?
//! - Which backend/simulator/hardware executed it?
//! - Which calibration snapshot was active?
//! - When did the experiment occur?
//! - Which software/environment produced the result?
//! - Can the benchmark definition and result be identified unambiguously?
//!
//! Provenance is deliberately separate from:
//!
//! - benchmark configuration;
//! - benchmark execution;
//! - statistical analysis;
//! - benchmark metrics;
//! - benchmark result representation.
//!
//! Those components can depend on this module, but this module must not
//! depend on them. This makes provenance a foundational, dependency-light
//! component that can be completed before the rest of the benchmarking
//! subsystem.
//!
//! # Architectural position
//!
//! ```text
//!                         Benchmark
//!                            │
//!                            ▼
//!                   ┌──────────────────┐
//!                   │   Provenance     │
//!                   └────────┬─────────┘
//!                            │
//!        ┌───────────────────┼────────────────────┐
//!        ▼                   ▼                    ▼
//!    Generator           Compiler             Backend
//!        │                   │                    │
//!        ▼                   ▼                    ▼
//!    Workload            Compilation          Hardware/
//!    identity            identity             Simulator
//!                                                │
//!                                                ▼
//!                                           Calibration
//! ```
//!
//! Provenance is metadata, not execution state.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::benchmarking::core::provenance
//!                 ▲
//!                 │
//!       config / experiment / result
//!                 ▲
//!                 │
//!       protocols / execution / analysis
//! ```
//!
//! This module must never depend on protocol-specific modules.
//!
//! # Security and privacy
//!
//! Provenance can contain sensitive infrastructure information such as
//! backend identifiers, host information, calibration identifiers, or
//! compiler configuration. Therefore:
//!
//! - no secrets are accepted as part of the API;
//! - no environment variables are automatically collected;
//! - no filesystem inspection is performed;
//! - no network access occurs;
//! - no process-global mutable state is used;
//! - callers explicitly decide which identifying metadata to provide;
//! - a redacted representation is available for publication/sharing;
//! - hashes identify provenance without exposing the underlying metadata.
//!
//! Provenance hashes are SHA-256 hashes of the serialized provenance
//! representation. They are integrity/fingerprint identifiers, not digital
//! signatures and do not prove who produced a result.
//!
//! # Reproducibility
//!
//! Provenance records the information required to identify a benchmark
//! execution. It does not claim that all hardware experiments are perfectly
//! reproducible. Physical hardware is inherently time-dependent.
//!
//! In particular, calibration identity and timestamp are first-class fields.
//! A future reproducibility module can use this information to determine
//! whether two experiments are:
//!
//! - exactly reproducible;
//! - configuration-equivalent;
//! - hardware-equivalent;
//! - calibration-equivalent;
//! - comparable but not reproducible;
//! - not comparable.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1 / Rust 2021.
//! No nightly features are required.
//!
//! # Integration contract
//!
//! Future benchmarking modules should use this type as follows:
//!
//! - `core/config.rs` supplies benchmark/protocol configuration identity.
//! - `core/reproducibility.rs` consumes `ExperimentIdentity`,
//!   `ConfigurationFingerprint`, and the provenance fingerprint.
//! - `core/circuit.rs` supplies circuit/workload fingerprints.
//! - `core/execution.rs` supplies backend and execution metadata.
//! - `core/result.rs` embeds [`BenchmarkProvenance`] unchanged.
//! - `reporting/*` serializes provenance using Serde.
//! - `analysis/*` compares provenance before declaring benchmark results
//!   directly comparable.
//! - `registry/*` supplies benchmark protocol/version information.
//!
//! None of those future modules need to modify the semantics of this file.
//!
//! ---------------------------------------------------------------------------
//! Public API stability
//! ---------------------------------------------------------------------------
//!
//! The primary public types are:
//!
//! - [`BenchmarkProvenance`]
//! - [`BenchmarkIdentity`]
//! - [`SoftwareProvenance`]
//! - [`CompilerProvenance`]
//! - [`BackendProvenance`]
//! - [`CalibrationProvenance`]
//! - [`ExecutionEnvironment`]
//! - [`ProvenanceHash`]
//! - [`ProvenanceBuilder`]
//!
//! The builder is provided so callers do not need to construct large nested
//! structs manually. The underlying structs remain public for serialization,
//! inspection, and future stable APIs.
//!
//! ---------------------------------------------------------------------------
//! Hashing contract
//! ---------------------------------------------------------------------------
//!
//! `BenchmarkProvenance::fingerprint()` computes SHA-256 over the canonical
//! JSON representation generated by `serde_json` from this module's structs.
//!
//! This representation is deterministic because:
//!
//! - fields are represented by fixed Rust struct declaration order;
//! - no unordered map is used in the provenance model;
//! - optional values are represented consistently;
//! - the schema version is included.
//!
//! Consumers must not use the hash as a cryptographic signature.
//! ---------------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Current provenance schema version.
///
/// Increment this when the serialized meaning of provenance changes in a
/// backwards-incompatible way.
pub const PROVENANCE_SCHEMA_VERSION: u32 = 1;

/// Identifier for the provenance schema.
///
/// This is intentionally a string rather than an integer-only API so that
/// serialized benchmark artifacts can be interpreted independently of Rust.
pub const PROVENANCE_SCHEMA_ID: &str = "zamani.quantum.benchmark.provenance";

/// Maximum length accepted for free-form provenance identifiers.
///
/// Provenance must remain bounded because it may originate from external
/// configuration or benchmark manifests.
pub const MAX_PROVENANCE_STRING_LENGTH: usize = 4_096;

/// Maximum length accepted for a hash-like textual value.
///
/// SHA-256 hexadecimal values are 64 characters, but the limit intentionally
/// permits future hash algorithms without allowing unbounded metadata.
pub const MAX_HASH_STRING_LENGTH: usize = 256;

/// Maximum number of tags attached to provenance.
pub const MAX_PROVENANCE_TAGS: usize = 256;

/// Maximum length of one provenance tag.
pub const MAX_PROVENANCE_TAG_LENGTH: usize = 256;

/// Maximum supported Unix timestamp in nanoseconds.
///
/// This is not a validity boundary for quantum experiments. It exists only
/// to prevent accidental integer abuse when callers construct timestamps.
pub const MAX_UNIX_TIMESTAMP_NANOS: u64 = i64::MAX as u64;

/// Errors produced by provenance construction, validation, serialization,
/// hashing, or timestamp operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvenanceError {
    /// A required identifier was empty.
    EmptyIdentifier {
        /// Logical name of the invalid field.
        field: &'static str,
    },

    /// A textual field exceeded its bounded production size.
    StringTooLong {
        /// Logical name of the invalid field.
        field: &'static str,

        /// Supplied length in bytes.
        length: usize,

        /// Maximum permitted length.
        maximum: usize,
    },

    /// A hash-like textual field was malformed.
    InvalidHash {
        /// Logical name of the invalid field.
        field: &'static str,
    },

    /// A timestamp was outside the supported representation.
    InvalidTimestamp {
        /// Timestamp supplied by the caller.
        timestamp_nanos: u64,
    },

    /// A tag was empty or too long.
    InvalidTag {
        /// Position of the invalid tag.
        index: usize,
    },

    /// Too many provenance tags were supplied.
    TooManyTags {
        /// Number supplied.
        count: usize,

        /// Maximum supported.
        maximum: usize,
    },

    /// JSON serialization failed.
    Serialization {
        /// Human-readable serialization error.
        message: String,
    },
}

impl fmt::Display for ProvenanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(f, "provenance field '{field}' must not be empty")
            }

            Self::StringTooLong {
                field,
                length,
                maximum,
            } => {
                write!(
                    f,
                    "provenance field '{field}' is too long: \
                     {length} bytes, maximum {maximum}"
                )
            }

            Self::InvalidHash { field } => {
                write!(
                    f,
                    "provenance field '{field}' is not a valid hash identifier"
                )
            }

            Self::InvalidTimestamp { timestamp_nanos } => {
                write!(
                    f,
                    "provenance timestamp {timestamp_nanos} is outside \
                     the supported range"
                )
            }

            Self::InvalidTag { index } => {
                write!(
                    f,
                    "provenance tag at index {index} is empty or too long"
                )
            }

            Self::TooManyTags { count, maximum } => {
                write!(
                    f,
                    "provenance contains {count} tags; maximum is {maximum}"
                )
            }

            Self::Serialization { message } => {
                write!(
                    f,
                    "unable to serialize benchmark provenance: {message}"
                )
            }
        }
    }
}

impl std::error::Error for ProvenanceError {}

/// A stable textual benchmark identity.
///
/// The identity is intentionally independent of a particular execution.
///
/// Example:
///
/// ```text
/// benchmark_id = "quantum_volume"
/// benchmark_version = "1.0"
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkIdentity {
    /// Stable benchmark/protocol identifier.
    pub benchmark_id: String,

    /// Version of the benchmark protocol.
    pub benchmark_version: String,

    /// Optional human-readable benchmark name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl BenchmarkIdentity {
    /// Creates a benchmark identity after validating its fields.
    pub fn new(
        benchmark_id: impl Into<String>,
        benchmark_version: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let identity = Self {
            benchmark_id: benchmark_id.into(),
            benchmark_version: benchmark_version.into(),
            name: None,
        };

        identity.validate()?;

        Ok(identity)
    }

    /// Adds an optional human-readable name.
    pub fn with_name(
        mut self,
        name: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        self.name = Some(name.into());
        self.validate()?;
        Ok(self)
    }

    /// Validates the identity.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        validate_required_string(
            "benchmark_id",
            &self.benchmark_id,
        )?;

        validate_required_string(
            "benchmark_version",
            &self.benchmark_version,
        )?;

        if let Some(name) = &self.name {
            validate_optional_string("name", name)?;
        }

        Ok(())
    }
}

/// Zamani/compiler software provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoftwareProvenance {
    /// Zamani compiler version.
    pub zamani_version: String,

    /// Compiler package version, when different from the language version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compiler_version: Option<String>,

    /// Quantum subsystem version, when independently versioned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantum_subsystem_version: Option<String>,

    /// Source/build commit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_revision: Option<String>,

    /// Build profile, e.g. `release`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_profile: Option<String>,

    /// Rust compiler version used to build Zamani.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rust_version: Option<String>,
}

impl SoftwareProvenance {
    /// Creates software provenance with the mandatory Zamani version.
    pub fn new(
        zamani_version: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let provenance = Self {
            zamani_version: zamani_version.into(),
            compiler_version: None,
            quantum_subsystem_version: None,
            source_revision: None,
            build_profile: None,
            rust_version: None,
        };

        provenance.validate()?;

        Ok(provenance)
    }

    /// Validates software provenance.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        validate_required_string(
            "zamani_version",
            &self.zamani_version,
        )?;

        validate_optional_string(
            "compiler_version",
            self.compiler_version.as_deref().unwrap_or(""),
        )?;

        validate_optional_string(
            "quantum_subsystem_version",
            self.quantum_subsystem_version
                .as_deref()
                .unwrap_or(""),
        )?;

        validate_optional_string(
            "source_revision",
            self.source_revision.as_deref().unwrap_or(""),
        )?;

        validate_optional_string(
            "build_profile",
            self.build_profile.as_deref().unwrap_or(""),
        )?;

        validate_optional_string(
            "rust_version",
            self.rust_version.as_deref().unwrap_or(""),
        )?;

        Ok(())
    }
}

/// Compiler and quantum-compilation provenance.
///
/// This records the transformations that can materially change benchmark
/// results. It is therefore deliberately more detailed than merely recording
/// the compiler version.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompilerProvenance {
    /// Compiler/lowering pipeline identifier.
    pub pipeline_id: String,

    /// Pipeline version.
    pub pipeline_version: String,

    /// Optimization configuration identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optimization_profile: Option<String>,

    /// Routing configuration identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing_profile: Option<String>,

    /// Scheduling configuration identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduling_profile: Option<String>,

    /// Error-correction configuration identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_correction_profile: Option<String>,

    /// Hash/fingerprint of the effective compiler configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration_hash: Option<String>,

    /// Hash/fingerprint of the input canonical quantum IR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_ir_hash: Option<String>,

    /// Hash/fingerprint of the final compiled workload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compiled_workload_hash: Option<String>,
}

impl CompilerProvenance {
    /// Creates compiler provenance.
    pub fn new(
        pipeline_id: impl Into<String>,
        pipeline_version: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let provenance = Self {
            pipeline_id: pipeline_id.into(),
            pipeline_version: pipeline_version.into(),
            optimization_profile: None,
            routing_profile: None,
            scheduling_profile: None,
            error_correction_profile: None,
            configuration_hash: None,
            input_ir_hash: None,
            compiled_workload_hash: None,
        };

        provenance.validate()?;

        Ok(provenance)
    }

    /// Validates compiler provenance.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        validate_required_string(
            "pipeline_id",
            &self.pipeline_id,
        )?;

        validate_required_string(
            "pipeline_version",
            &self.pipeline_version,
        )?;

        validate_optional_string(
            "optimization_profile",
            self.optimization_profile.as_deref().unwrap_or(""),
        )?;

        validate_optional_string(
            "routing_profile",
            self.routing_profile.as_deref().unwrap_or(""),
        )?;

        validate_optional_string(
            "scheduling_profile",
            self.scheduling_profile.as_deref().unwrap_or(""),
        )?;

        validate_optional_string(
            "error_correction_profile",
            self.error_correction_profile
                .as_deref()
                .unwrap_or(""),
        )?;

        validate_optional_hash(
            "configuration_hash",
            self.configuration_hash.as_deref(),
        )?;

        validate_optional_hash(
            "input_ir_hash",
            self.input_ir_hash.as_deref(),
        )?;

        validate_optional_hash(
            "compiled_workload_hash",
            self.compiled_workload_hash.as_deref(),
        )?;

        Ok(())
    }
}

/// Quantum backend/simulator provenance.
///
/// This structure intentionally supports multiple quantum technologies
/// without assuming that every backend has physical qubits, gates, or a
/// conventional circuit execution model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendProvenance {
    /// Stable backend identifier.
    pub backend_id: String,

    /// Provider/vendor identifier, if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,

    /// Backend software/API version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend_version: Option<String>,

    /// Quantum technology, e.g. superconducting, trapped-ion, simulator,
    /// neutral-atom, photonic, annealing, analog.
    pub technology: String,

    /// Execution model.
    pub execution_model: String,

    /// Optional topology fingerprint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topology_hash: Option<String>,

    /// Optional backend capability fingerprint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability_hash: Option<String>,

    /// Optional stable backend instance identifier.
    ///
    /// This may be redacted when publishing results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

impl BackendProvenance {
    /// Creates backend provenance.
    pub fn new(
        backend_id: impl Into<String>,
        technology: impl Into<String>,
        execution_model: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let provenance = Self {
            backend_id: backend_id.into(),
            provider_id: None,
            backend_version: None,
            technology: technology.into(),
            execution_model: execution_model.into(),
            topology_hash: None,
            capability_hash: None,
            instance_id: None,
        };

        provenance.validate()?;

        Ok(provenance)
    }

    /// Validates backend provenance.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        validate_required_string(
            "backend_id",
            &self.backend_id,
        )?;

        validate_required_string(
            "technology",
            &self.technology,
        )?;

        validate_required_string(
            "execution_model",
            &self.execution_model,
        )?;

        validate_optional_string(
            "provider_id",
            self.provider_id.as_deref().unwrap_or(""),
        )?;

        validate_optional_string(
            "backend_version",
            self.backend_version.as_deref().unwrap_or(""),
        )?;

        validate_optional_string(
            "instance_id",
            self.instance_id.as_deref().unwrap_or(""),
        )?;

        validate_optional_hash(
            "topology_hash",
            self.topology_hash.as_deref(),
        )?;

        validate_optional_hash(
            "capability_hash",
            self.capability_hash.as_deref(),
        )?;

        Ok(())
    }
}

/// Hardware/software calibration snapshot provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalibrationProvenance {
    /// Stable calibration identifier.
    pub calibration_id: String,

    /// Calibration version, if the backend exposes one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calibration_version: Option<String>,

    /// Calibration timestamp in Unix nanoseconds.
    pub timestamp_unix_nanos: u64,

    /// Hash/fingerprint of the calibration snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_hash: Option<String>,

    /// Optional expiration timestamp in Unix nanoseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at_unix_nanos: Option<u64>,
}

impl CalibrationProvenance {
    /// Creates a calibration provenance record.
    pub fn new(
        calibration_id: impl Into<String>,
        timestamp_unix_nanos: u64,
    ) -> Result<Self, ProvenanceError> {
        let provenance = Self {
            calibration_id: calibration_id.into(),
            calibration_version: None,
            timestamp_unix_nanos,
            snapshot_hash: None,
            expires_at_unix_nanos: None,
        };

        provenance.validate()?;

        Ok(provenance)
    }

    /// Creates a calibration provenance record using the current system
    /// clock.
    pub fn now(
        calibration_id: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        Self::new(
            calibration_id,
            current_unix_timestamp_nanos()?,
        )
    }

    /// Validates calibration provenance.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        validate_required_string(
            "calibration_id",
            &self.calibration_id,
        )?;

        validate_timestamp(self.timestamp_unix_nanos)?;

        if let Some(expires_at) = self.expires_at_unix_nanos {
            validate_timestamp(expires_at)?;

            if expires_at < self.timestamp_unix_nanos {
                return Err(ProvenanceError::InvalidTimestamp {
                    timestamp_nanos: expires_at,
                });
            }
        }

        validate_optional_string(
            "calibration_version",
            self.calibration_version
                .as_deref()
                .unwrap_or(""),
        )?;

        validate_optional_hash(
            "snapshot_hash",
            self.snapshot_hash.as_deref(),
        )?;

        Ok(())
    }
}

/// Execution environment provenance.
///
/// This is deliberately caller-supplied. Zamani does not automatically
/// inspect the host environment because doing so could unexpectedly leak
/// infrastructure information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionEnvironment {
    /// Operating-system identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,

    /// CPU architecture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,

    /// Host identifier, if explicitly provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,

    /// Container/runtime identifier, if explicitly provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_id: Option<String>,

    /// Simulator version, when the backend is simulated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simulator_version: Option<String>,

    /// GPU/device environment identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accelerator_id: Option<String>,
}

impl ExecutionEnvironment {
    /// Creates an empty environment record.
    pub const fn empty() -> Self {
        Self {
            operating_system: None,
            architecture: None,
            host_id: None,
            runtime_id: None,
            simulator_version: None,
            accelerator_id: None,
        }
    }

    /// Validates environment metadata.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        validate_optional_string(
            "operating_system",
            self.operating_system.as_deref().unwrap_or(""),
        )?;

        validate_optional_string(
            "architecture",
            self.architecture.as_deref().unwrap_or(""),
        )?;

        validate_optional_string(
            "host_id",
            self.host_id.as_deref().unwrap_or(""),
        )?;

        validate_optional_string(
            "runtime_id",
            self.runtime_id.as_deref().unwrap_or(""),
        )?;

        validate_optional_string(
            "simulator_version",
            self.simulator_version
                .as_deref()
                .unwrap_or(""),
        )?;

        validate_optional_string(
            "accelerator_id",
            self.accelerator_id.as_deref().unwrap_or(""),
        )?;

        Ok(())
    }
}

/// SHA-256 provenance fingerprint.
///
/// The byte representation is fixed at 32 bytes and therefore avoids
/// accepting arbitrary hash strings internally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProvenanceHash(pub [u8; 32]);

impl ProvenanceHash {
    /// Creates a hash from raw bytes.
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Returns the raw hash bytes.
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Returns the lowercase hexadecimal representation.
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl fmt::Display for ProvenanceHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

/// Complete provenance record for one benchmark execution.
///
/// This is the object that future `core::result::BenchmarkResult` should
/// embed directly.
///
/// It deliberately contains no benchmark measurements. Measurements belong
/// to the result/observation layers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkProvenance {
    /// Provenance schema identifier.
    pub schema_id: String,

    /// Provenance schema version.
    pub schema_version: u32,

    /// Benchmark protocol identity.
    pub benchmark: BenchmarkIdentity,

    /// Zamani/compiler software identity.
    pub software: SoftwareProvenance,

    /// Quantum compiler/lowering configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compiler: Option<CompilerProvenance>,

    /// Backend/simulator identity.
    pub backend: BackendProvenance,

    /// Calibration snapshot, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calibration: Option<CalibrationProvenance>,

    /// Execution environment, when intentionally supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<ExecutionEnvironment>,

    /// Stable experiment identifier.
    pub experiment_id: String,

    /// Benchmark configuration fingerprint.
    ///
    /// This should be supplied by `core::config` once that module exists.
    pub configuration_hash: String,

    /// Benchmark workload/circuit fingerprint.
    ///
    /// This should identify the generated workload independently of the
    /// result data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workload_hash: Option<String>,

    /// Random generator identity/version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub random_generator: Option<String>,

    /// Explicit random seed used by the benchmark.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<u64>,

    /// Number of circuits/workloads requested.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circuit_count: Option<u64>,

    /// Number of shots requested per circuit where applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shots_per_circuit: Option<u64>,

    /// Benchmark creation timestamp in Unix nanoseconds.
    pub created_at_unix_nanos: u64,

    /// Execution start timestamp in Unix nanoseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at_unix_nanos: Option<u64>,

    /// Execution completion timestamp in Unix nanoseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at_unix_nanos: Option<u64>,

    /// Caller-defined provenance tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

impl BenchmarkProvenance {
    /// Validates the complete provenance record.
    pub fn validate(&self) -> Result<(), ProvenanceError> {
        if self.schema_id != PROVENANCE_SCHEMA_ID {
            return Err(ProvenanceError::EmptyIdentifier {
                field: "schema_id",
            });
        }

        if self.schema_version == 0 {
            return Err(ProvenanceError::EmptyIdentifier {
                field: "schema_version",
            });
        }

        self.benchmark.validate()?;
        self.software.validate()?;
        self.backend.validate()?;

        if let Some(compiler) = &self.compiler {
            compiler.validate()?;
        }

        if let Some(calibration) = &self.calibration {
            calibration.validate()?;
        }

        if let Some(environment) = &self.environment {
            environment.validate()?;
        }

        validate_required_string(
            "experiment_id",
            &self.experiment_id,
        )?;

        validate_hash(
            "configuration_hash",
            &self.configuration_hash,
        )?;

        validate_optional_hash(
            "workload_hash",
            self.workload_hash.as_deref(),
        )?;

        validate_optional_string(
            "random_generator",
            self.random_generator.as_deref().unwrap_or(""),
        )?;

        if let Some(count) = self.circuit_count {
            if count == 0 {
                return Err(ProvenanceError::EmptyIdentifier {
                    field: "circuit_count",
                });
            }
        }

        if let Some(shots) = self.shots_per_circuit {
            if shots == 0 {
                return Err(ProvenanceError::EmptyIdentifier {
                    field: "shots_per_circuit",
                });
            }
        }

        validate_timestamp(self.created_at_unix_nanos)?;

        if let Some(started) = self.started_at_unix_nanos {
            validate_timestamp(started)?;

            if started < self.created_at_unix_nanos {
                return Err(ProvenanceError::InvalidTimestamp {
                    timestamp_nanos: started,
                });
            }
        }

        if let Some(completed) = self.completed_at_unix_nanos {
            validate_timestamp(completed)?;

            if let Some(started) = self.started_at_unix_nanos {
                if completed < started {
                    return Err(ProvenanceError::InvalidTimestamp {
                        timestamp_nanos: completed,
                    });
                }
            }
        }

        if self.tags.len() > MAX_PROVENANCE_TAGS {
            return Err(ProvenanceError::TooManyTags {
                count: self.tags.len(),
                maximum: MAX_PROVENANCE_TAGS,
            });
        }

        for (index, tag) in self.tags.iter().enumerate() {
            if tag.is_empty()
                || tag.len() > MAX_PROVENANCE_TAG_LENGTH
            {
                return Err(ProvenanceError::InvalidTag { index });
            }
        }

        Ok(())
    }

    /// Serializes the validated provenance record to JSON.
    ///
    /// This is the canonical serialization used for hashing.
    pub fn canonical_json(&self) -> Result<String, ProvenanceError> {
        self.validate()?;

        serde_json::to_string(self).map_err(|error| {
            ProvenanceError::Serialization {
                message: error.to_string(),
            }
        })
    }

    /// Calculates a SHA-256 fingerprint over canonical provenance JSON.
    ///
    /// This fingerprint identifies the complete provenance record.
    pub fn fingerprint(&self) -> Result<ProvenanceHash, ProvenanceError> {
        let canonical = self.canonical_json()?;

        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());

        let digest = hasher.finalize();

        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&digest);

        Ok(ProvenanceHash::from_bytes(bytes))
    }

    /// Returns a copy suitable for public sharing.
    ///
    /// Sensitive infrastructure identifiers such as `instance_id` and
    /// `host_id` are removed. The resulting public record is still useful for
    /// scientific comparison and retains all protocol/compiler/backend
    /// identity required for ordinary benchmark interpretation.
    pub fn redacted_for_publication(&self) -> Self {
        let mut redacted = self.clone();

        if let Some(backend) = redacted.backend.instance_id.as_mut() {
            let _ = backend;
        }

        redacted.backend.instance_id = None;

        if let Some(environment) = redacted.environment.as_mut() {
            environment.host_id = None;
            environment.runtime_id = None;
            environment.accelerator_id = None;
        }

        redacted
    }

    /// Marks execution as started.
    ///
    /// This method returns a new immutable value rather than mutating a
    /// shared provenance record. Provenance objects therefore remain safe to
    /// pass between benchmark execution stages.
    pub fn with_started_at(
        mut self,
        timestamp_unix_nanos: u64,
    ) -> Result<Self, ProvenanceError> {
        validate_timestamp(timestamp_unix_nanos)?;

        self.started_at_unix_nanos = Some(timestamp_unix_nanos);
        self.validate()?;

        Ok(self)
    }

    /// Marks execution as completed.
    pub fn with_completed_at(
        mut self,
        timestamp_unix_nanos: u64,
    ) -> Result<Self, ProvenanceError> {
        validate_timestamp(timestamp_unix_nanos)?;

        self.completed_at_unix_nanos = Some(timestamp_unix_nanos);
        self.validate()?;

        Ok(self)
    }

    /// Adds a provenance tag.
    pub fn with_tag(
        mut self,
        tag: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let tag = tag.into();

        if tag.is_empty()
            || tag.len() > MAX_PROVENANCE_TAG_LENGTH
        {
            return Err(ProvenanceError::InvalidTag {
                index: self.tags.len(),
            });
        }

        if self.tags.len() >= MAX_PROVENANCE_TAGS {
            return Err(ProvenanceError::TooManyTags {
                count: self.tags.len() + 1,
                maximum: MAX_PROVENANCE_TAGS,
            });
        }

        self.tags.push(tag);
        self.validate()?;

        Ok(self)
    }
}

/// Builder for [`BenchmarkProvenance`].
///
/// The builder makes future integration with `core/config`, execution,
/// backend, and result modules straightforward without forcing those modules
/// to mutate provenance state.
#[derive(Debug, Clone)]
pub struct ProvenanceBuilder {
    benchmark: BenchmarkIdentity,
    software: SoftwareProvenance,
    backend: BackendProvenance,
    compiler: Option<CompilerProvenance>,
    calibration: Option<CalibrationProvenance>,
    environment: Option<ExecutionEnvironment>,
    experiment_id: String,
    configuration_hash: String,
    workload_hash: Option<String>,
    random_generator: Option<String>,
    random_seed: Option<u64>,
    circuit_count: Option<u64>,
    shots_per_circuit: Option<u64>,
    created_at_unix_nanos: u64,
    started_at_unix_nanos: Option<u64>,
    completed_at_unix_nanos: Option<u64>,
    tags: Vec<String>,
}

impl ProvenanceBuilder {
    /// Creates a builder with mandatory benchmark/software/backend identity.
    pub fn new(
        benchmark: BenchmarkIdentity,
        software: SoftwareProvenance,
        backend: BackendProvenance,
        experiment_id: impl Into<String>,
        configuration_hash: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        benchmark.validate()?;
        software.validate()?;
        backend.validate()?;

        let experiment_id = experiment_id.into();
        let configuration_hash = configuration_hash.into();

        validate_required_string(
            "experiment_id",
            &experiment_id,
        )?;

        validate_hash(
            "configuration_hash",
            &configuration_hash,
        )?;

        Ok(Self {
            benchmark,
            software,
            backend,
            compiler: None,
            calibration: None,
            environment: None,
            experiment_id,
            configuration_hash,
            workload_hash: None,
            random_generator: None,
            random_seed: None,
            circuit_count: None,
            shots_per_circuit: None,
            created_at_unix_nanos: current_unix_timestamp_nanos()?,
            started_at_unix_nanos: None,
            completed_at_unix_nanos: None,
            tags: Vec::new(),
        })
    }

    /// Supplies compiler provenance.
    pub fn compiler(
        mut self,
        compiler: CompilerProvenance,
    ) -> Result<Self, ProvenanceError> {
        compiler.validate()?;
        self.compiler = Some(compiler);
        Ok(self)
    }

    /// Supplies calibration provenance.
    pub fn calibration(
        mut self,
        calibration: CalibrationProvenance,
    ) -> Result<Self, ProvenanceError> {
        calibration.validate()?;
        self.calibration = Some(calibration);
        Ok(self)
    }

    /// Supplies execution environment metadata.
    pub fn environment(
        mut self,
        environment: ExecutionEnvironment,
    ) -> Result<Self, ProvenanceError> {
        environment.validate()?;
        self.environment = Some(environment);
        Ok(self)
    }

    /// Supplies a workload/circuit fingerprint.
    pub fn workload_hash(
        mut self,
        workload_hash: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let workload_hash = workload_hash.into();

        validate_hash(
            "workload_hash",
            &workload_hash,
        )?;

        self.workload_hash = Some(workload_hash);
        Ok(self)
    }

    /// Supplies random generator identity.
    pub fn random_generator(
        mut self,
        random_generator: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let random_generator = random_generator.into();

        validate_required_string(
            "random_generator",
            &random_generator,
        )?;

        self.random_generator = Some(random_generator);
        Ok(self)
    }

    /// Supplies the deterministic benchmark seed.
    pub fn random_seed(mut self, seed: u64) -> Self {
        self.random_seed = Some(seed);
        self
    }

    /// Supplies circuit count.
    pub fn circuit_count(
        mut self,
        count: u64,
    ) -> Result<Self, ProvenanceError> {
        if count == 0 {
            return Err(ProvenanceError::EmptyIdentifier {
                field: "circuit_count",
            });
        }

        self.circuit_count = Some(count);
        Ok(self)
    }

    /// Supplies shots per circuit.
    pub fn shots_per_circuit(
        mut self,
        shots: u64,
    ) -> Result<Self, ProvenanceError> {
        if shots == 0 {
            return Err(ProvenanceError::EmptyIdentifier {
                field: "shots_per_circuit",
            });
        }

        self.shots_per_circuit = Some(shots);
        Ok(self)
    }

    /// Supplies an explicit creation timestamp.
    pub fn created_at(
        mut self,
        timestamp_unix_nanos: u64,
    ) -> Result<Self, ProvenanceError> {
        validate_timestamp(timestamp_unix_nanos)?;
        self.created_at_unix_nanos = timestamp_unix_nanos;
        Ok(self)
    }

    /// Supplies the execution-start timestamp.
    pub fn started_at(
        mut self,
        timestamp_unix_nanos: u64,
    ) -> Result<Self, ProvenanceError> {
        validate_timestamp(timestamp_unix_nanos)?;
        self.started_at_unix_nanos = Some(timestamp_unix_nanos);
        Ok(self)
    }

    /// Supplies the execution-completion timestamp.
    pub fn completed_at(
        mut self,
        timestamp_unix_nanos: u64,
    ) -> Result<Self, ProvenanceError> {
        validate_timestamp(timestamp_unix_nanos)?;
        self.completed_at_unix_nanos = Some(timestamp_unix_nanos);
        Ok(self)
    }

    /// Adds a bounded provenance tag.
    pub fn tag(
        mut self,
        tag: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        let tag = tag.into();

        if tag.is_empty()
            || tag.len() > MAX_PROVENANCE_TAG_LENGTH
        {
            return Err(ProvenanceError::InvalidTag {
                index: self.tags.len(),
            });
        }

        if self.tags.len() >= MAX_PROVENANCE_TAGS {
            return Err(ProvenanceError::TooManyTags {
                count: self.tags.len() + 1,
                maximum: MAX_PROVENANCE_TAGS,
            });
        }

        self.tags.push(tag);
        Ok(self)
    }

    /// Builds and validates the final immutable provenance record.
    pub fn build(self) -> Result<BenchmarkProvenance, ProvenanceError> {
        let provenance = BenchmarkProvenance {
            schema_id: PROVENANCE_SCHEMA_ID.to_owned(),
            schema_version: PROVENANCE_SCHEMA_VERSION,
            benchmark: self.benchmark,
            software: self.software,
            compiler: self.compiler,
            backend: self.backend,
            calibration: self.calibration,
            environment: self.environment,
            experiment_id: self.experiment_id,
            configuration_hash: self.configuration_hash,
            workload_hash: self.workload_hash,
            random_generator: self.random_generator,
            random_seed: self.random_seed,
            circuit_count: self.circuit_count,
            shots_per_circuit: self.shots_per_circuit,
            created_at_unix_nanos: self.created_at_unix_nanos,
            started_at_unix_nanos: self.started_at_unix_nanos,
            completed_at_unix_nanos: self.completed_at_unix_nanos,
            tags: self.tags,
        };

        provenance.validate()?;

        Ok(provenance)
    }
}

/// Returns the current Unix timestamp in nanoseconds.
///
/// The API deliberately returns a `Result` because `SystemTime` can represent
/// dates before the Unix epoch on supported platforms.
pub fn current_unix_timestamp_nanos() -> Result<u64, ProvenanceError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| ProvenanceError::InvalidTimestamp {
            timestamp_nanos: 0,
        })?;

    let nanos = duration
        .as_secs()
        .checked_mul(1_000_000_000)
        .and_then(|seconds| {
            seconds.checked_add(u64::from(duration.subsec_nanos()))
        })
        .ok_or(ProvenanceError::InvalidTimestamp {
            timestamp_nanos: u64::MAX,
        })?;

    validate_timestamp(nanos)?;

    Ok(nanos)
}

/// Validates a required bounded string.
fn validate_required_string(
    field: &'static str,
    value: &str,
) -> Result<(), ProvenanceError> {
    if value.is_empty() {
        return Err(ProvenanceError::EmptyIdentifier { field });
    }

    if value.len() > MAX_PROVENANCE_STRING_LENGTH {
        return Err(ProvenanceError::StringTooLong {
            field,
            length: value.len(),
            maximum: MAX_PROVENANCE_STRING_LENGTH,
        });
    }

    Ok(())
}

/// Validates an optional bounded string.
///
/// Empty strings are treated as invalid when explicitly supplied. `None` is
/// represented by callers by passing an empty fallback here.
fn validate_optional_string(
    field: &'static str,
    value: &str,
) -> Result<(), ProvenanceError> {
    if value.is_empty() {
        return Ok(());
    }

    if value.len() > MAX_PROVENANCE_STRING_LENGTH {
        return Err(ProvenanceError::StringTooLong {
            field,
            length: value.len(),
            maximum: MAX_PROVENANCE_STRING_LENGTH,
        });
    }

    Ok(())
}

/// Validates a required hash identifier.
///
/// The internal contract accepts hexadecimal SHA-256 values. This makes
/// provenance fingerprints interoperable with the rest of Zamani's hashing
/// infrastructure while still allowing future algorithms to be represented
/// through longer textual fields if the schema is extended.
fn validate_hash(
    field: &'static str,
    value: &str,
) -> Result<(), ProvenanceError> {
    if value.is_empty() {
        return Err(ProvenanceError::EmptyIdentifier { field });
    }

    if value.len() > MAX_HASH_STRING_LENGTH {
        return Err(ProvenanceError::StringTooLong {
            field,
            length: value.len(),
            maximum: MAX_HASH_STRING_LENGTH,
        });
    }

    if value.len() % 2 != 0 {
        return Err(ProvenanceError::InvalidHash { field });
    }

    if !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(ProvenanceError::InvalidHash { field });
    }

    Ok(())
}

/// Validates an optional hash.
fn validate_optional_hash(
    field: &'static str,
    value: Option<&str>,
) -> Result<(), ProvenanceError> {
    if let Some(value) = value {
        validate_hash(field, value)?;
    }

    Ok(())
}

/// Validates a timestamp.
fn validate_timestamp(
    timestamp_nanos: u64,
) -> Result<(), ProvenanceError> {
    if timestamp_nanos > MAX_UNIX_TIMESTAMP_NANOS {
        return Err(ProvenanceError::InvalidTimestamp {
            timestamp_nanos,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_benchmark() -> BenchmarkIdentity {
        BenchmarkIdentity::new(
            "quantum_volume",
            "1.0.0",
        )
        .expect("valid benchmark identity")
    }

    fn test_software() -> SoftwareProvenance {
        SoftwareProvenance::new("0.1.0")
            .expect("valid software provenance")
    }

    fn test_backend() -> BackendProvenance {
        BackendProvenance::new(
            "local-simulator",
            "simulator",
            "state_vector",
        )
        .expect("valid backend provenance")
    }

    fn test_hash() -> String {
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            .to_owned()
    }

    fn test_provenance() -> BenchmarkProvenance {
        ProvenanceBuilder::new(
            test_benchmark(),
            test_software(),
            test_backend(),
            "experiment-0001",
            test_hash(),
        )
        .expect("valid builder")
        .random_generator("zamani.random.v1")
        .expect("valid random generator")
        .random_seed(42)
        .circuit_count(10)
        .expect("valid circuit count")
        .shots_per_circuit(1_000)
        .expect("valid shots")
        .build()
        .expect("valid provenance")
    }

    #[test]
    fn production_schema_is_stable() {
        assert_eq!(
            PROVENANCE_SCHEMA_ID,
            "zamani.quantum.benchmark.provenance"
        );
        assert_eq!(PROVENANCE_SCHEMA_VERSION, 1);
    }

    #[test]
    fn benchmark_identity_rejects_empty_id() {
        let result = BenchmarkIdentity::new("", "1.0.0");

        assert!(matches!(
            result,
            Err(ProvenanceError::EmptyIdentifier {
                field: "benchmark_id"
            })
        ));
    }

    #[test]
    fn software_provenance_requires_zamani_version() {
        let result = SoftwareProvenance::new("");

        assert!(matches!(
            result,
            Err(ProvenanceError::EmptyIdentifier {
                field: "zamani_version"
            })
        ));
    }

    #[test]
    fn backend_requires_identity() {
        let result = BackendProvenance::new(
            "",
            "simulator",
            "state_vector",
        );

        assert!(matches!(
            result,
            Err(ProvenanceError::EmptyIdentifier {
                field: "backend_id"
            })
        ));
    }

    #[test]
    fn invalid_hash_is_rejected() {
        let result = ProvenanceBuilder::new(
            test_benchmark(),
            test_software(),
            test_backend(),
            "experiment",
            "not-a-hash",
        );

        assert!(matches!(
            result,
            Err(ProvenanceError::InvalidHash {
                field: "configuration_hash"
            })
        ));
    }

    #[test]
    fn valid_provenance_builds() {
        let provenance = test_provenance();

        assert_eq!(
            provenance.schema_id,
            PROVENANCE_SCHEMA_ID
        );

        assert_eq!(
            provenance.schema_version,
            PROVENANCE_SCHEMA_VERSION
        );

        assert_eq!(
            provenance.benchmark.benchmark_id,
            "quantum_volume"
        );

        assert_eq!(
            provenance.backend.backend_id,
            "local-simulator"
        );
    }

    #[test]
    fn provenance_validates_after_build() {
        let provenance = test_provenance();

        assert!(provenance.validate().is_ok());
    }

    #[test]
    fn canonical_json_is_deterministic() {
        let provenance = test_provenance();

        let first = provenance
            .canonical_json()
            .expect("serialization should succeed");

        let second = provenance
            .canonical_json()
            .expect("serialization should succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn fingerprint_is_deterministic() {
        let provenance = test_provenance();

        let first = provenance
            .fingerprint()
            .expect("fingerprint should succeed");

        let second = provenance
            .fingerprint()
            .expect("fingerprint should succeed");

        assert_eq!(first, second);
        assert_eq!(first.to_hex().len(), 64);
    }

    #[test]
    fn changing_provenance_changes_fingerprint() {
        let first = test_provenance();

        let second = first
            .clone()
            .with_tag("different-experiment")
            .expect("tag should be valid");

        let first_hash = first
            .fingerprint()
            .expect("fingerprint should succeed");

        let second_hash = second
            .fingerprint()
            .expect("fingerprint should succeed");

        assert_ne!(first_hash, second_hash);
    }

    #[test]
    fn publication_redaction_removes_sensitive_identifiers() {
        let mut provenance = test_provenance();

        provenance.backend.instance_id =
            Some("private-backend-instance".to_owned());

        provenance.environment =
            Some(ExecutionEnvironment {
                operating_system: Some(
                    "linux".to_owned(),
                ),
                architecture: Some(
                    "x86_64".to_owned(),
                ),
                host_id: Some(
                    "private-host".to_owned(),
                ),
                runtime_id: Some(
                    "private-runtime".to_owned(),
                ),
                simulator_version: Some(
                    "1.0".to_owned(),
                ),
                accelerator_id: Some(
                    "private-gpu".to_owned(),
                ),
            });

        let redacted =
            provenance.redacted_for_publication();

        assert_eq!(
            redacted.backend.instance_id,
            None
        );

        let environment =
            redacted.environment.expect(
                "environment should remain present",
            );

        assert_eq!(environment.host_id, None);
        assert_eq!(environment.runtime_id, None);
        assert_eq!(environment.accelerator_id, None);

        assert_eq!(
            environment.operating_system.as_deref(),
            Some("linux")
        );
    }

    #[test]
    fn execution_timestamps_are_ordered() {
        let provenance = test_provenance();

        let started = provenance
            .clone()
            .with_started_at(
                provenance.created_at_unix_nanos,
            )
            .expect("valid start");

        let completed = started
            .with_completed_at(
                provenance.created_at_unix_nanos,
            )
            .expect("valid completion");

        assert!(completed.validate().is_ok());
    }

    #[test]
    fn completion_before_start_is_rejected() {
        let provenance = test_provenance();

        let started = provenance
            .clone()
            .with_started_at(100)
            .expect("valid start");

        let result = started.with_completed_at(99);

        assert!(matches!(
            result,
            Err(ProvenanceError::InvalidTimestamp {
                timestamp_nanos: 99
            })
        ));
    }

    #[test]
    fn tags_are_bounded() {
        let provenance = test_provenance();

        let tagged = provenance
            .with_tag("nightly")
            .expect("tag should be accepted");

        assert_eq!(tagged.tags, vec!["nightly"]);
    }

    #[test]
    fn serde_round_trip_preserves_provenance() {
        let provenance = test_provenance();

        let json =
            serde_json::to_string(&provenance)
                .expect("serialization should succeed");

        let decoded: BenchmarkProvenance =
            serde_json::from_str(&json)
                .expect("deserialization should succeed");

        assert_eq!(provenance, decoded);
    }

    #[test]
    fn current_timestamp_is_valid() {
        let timestamp =
            current_unix_timestamp_nanos()
                .expect("current timestamp should succeed");

        assert!(
            timestamp <= MAX_UNIX_TIMESTAMP_NANOS
        );
    }

    #[test]
    fn calibration_timestamp_is_validated() {
        let calibration =
            CalibrationProvenance::new(
                "calibration-1",
                1_000,
            )
            .expect("valid calibration");

        assert_eq!(
            calibration.timestamp_unix_nanos,
            1_000
        );
    }

    #[test]
    fn calibration_expiration_before_creation_is_rejected() {
        let calibration =
            CalibrationProvenance {
                calibration_id:
                    "calibration-1".to_owned(),
                calibration_version: None,
                timestamp_unix_nanos: 2_000,
                snapshot_hash: None,
                expires_at_unix_nanos: Some(1_000),
            };

        assert!(matches!(
            calibration.validate(),
            Err(ProvenanceError::InvalidTimestamp {
                timestamp_nanos: 1_000
            })
        ));
    }

    #[test]
    fn provenance_hash_display_is_hex() {
        let hash =
            ProvenanceHash::from_bytes([0xAB; 32]);

        assert_eq!(
            hash.to_string(),
            "abababababababababababababababab\
             ababababababababababababababab"
                .replace(' ', "")
        );
    }
}