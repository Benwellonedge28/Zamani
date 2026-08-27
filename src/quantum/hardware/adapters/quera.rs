//! Zamani Quantum — QuEra Aquila Hardware Adapter
//!
//! Production-grade provider-specific adapter for QuEra's neutral-atom
//! Analog Hamiltonian Simulation (AHS) hardware exposed through Amazon
//! Braket.
//!
//! Path:
//!
//!     src/quantum/hardware/adapters/quera.rs
//!
//! # Responsibility
//!
//! This module translates Zamani's provider-neutral quantum hardware
//! execution contract into QuEra Aquila AHS quantum-task operations through
//! Amazon Braket.
//!
//! It owns:
//!
//! - QuEra provider identity;
//! - Aquila backend identity;
//! - Amazon Braket task lifecycle mapping;
//! - Aquila AHS program validation;
//! - Braket AHS action construction;
//! - Aquila shot-limit validation;
//! - Aquila atom-register validation;
//! - Aquila spatial-register validation;
//! - experimental local-detuning gating;
//! - Braket task submission;
//! - task status normalization;
//! - task cancellation normalization;
//! - task queue metadata normalization;
//! - Braket task metadata validation;
//! - AHS result-artifact retrieval through an injected artifact store;
//! - QuEra AHS result normalization;
//! - provider-neutral result construction;
//! - provider error normalization;
//! - health normalization;
//! - deterministic request construction;
//! - adapter provenance;
//! - provider-neutral `QuantumBackendAdapter` integration;
//! - provider-specific security validation;
//! - local unit tests for all deterministic transformations.
//!
//! It deliberately does NOT own:
//!
//! - AWS credentials;
//! - AWS access keys;
//! - AWS secret keys;
//! - AWS session tokens;
//! - IAM;
//! - SigV4;
//! - TLS;
//! - HTTP clients;
//! - S3 clients;
//! - credential persistence;
//! - authentication;
//! - Zamani Quantum IR;
//! - OpenQASM parsing;
//! - AHS compiler/lowering;
//! - routing;
//! - scheduling;
//! - calibration storage;
//! - benchmarking;
//! - provider registries;
//! - global mutable state.
//!
//! Authentication is supplied by the injected `ProviderTransport` and
//! `QuEraArtifactStore` implementations.
//!
//! # Provider architecture
//!
//! QuEra Aquila is a neutral-atom AHS processor. Current production access
//! is exposed through Amazon Braket.
//!
//! ```text
//! Zamani Quantum IR
//!         |
//!         v
//! Analog lowering / compatibility
//!         |
//!         v
//! BackendProgram
//!         |
//!         v
//! QuEraAquilaAdapter
//!         |
//!         +--------------------------+
//!         |                          |
//!         v                          v
//! ProviderTransport           QuEraArtifactStore
//!         |                          |
//!         v                          v
//! Amazon Braket API                  S3
//!         |
//!         v
//! QuEra Aquila
//! ```
//!
//! The provider identity remains `quera` while the execution transport is
//! explicitly Amazon Braket.
//!
//! # Canonical backend identity
//!
//! ```text
//! provider = quera
//! backend  = quera/Aquila
//!
//! device ARN:
//! arn:aws:braket:us-east-1::device/qpu/quera/Aquila
//! ```
//!
//! The device ARN is stored as non-secret backend metadata.
//!
//! # Native workload
//!
//! Aquila is an Analog Hamiltonian Simulation device.
//!
//! It is NOT a conventional gate-model QPU and therefore this adapter does
//! not advertise ordinary native gates.
//!
//! The canonical workload is:
//!
//! ```text
//! QuantumWorkloadKind::AnalogProgram
//! ```
//!
//! # AHS program format
//!
//! The provider-native program is the Amazon Braket AHS schema:
//!
//! ```text
//! braket.ir.ahs.program
//! ```
//!
//! Schema version:
//!
//! ```text
//! 1
//! ```
//!
//! The program contains an atom arrangement and time-dependent Hamiltonian
//! fields. Zamani does not compile or modify those fields here.
//!
//! # Experimental local detuning
//!
//! QuEra Aquila currently exposes local detuning as an experimental
//! capability. Experimental capabilities must be explicitly enabled by the
//! caller/provider configuration.
//!
//! Therefore this adapter rejects AHS programs containing local detuning
//! unless `allow_experimental_local_detuning` is explicitly enabled.
//!
//! # Execution lifecycle
//!
//! ```text
//! preflight
//!     |
//!     v
//! validate AHS program
//!     |
//!     v
//! CreateQuantumTask
//!     |
//!     v
//! BackendJobId
//!     |
//!     +------> GetQuantumTask
//!     |
//!     +------> CancelQuantumTask
//!     |
//!     v
//! COMPLETED
//!     |
//!     v
//! outputS3Bucket/outputS3Directory
//!     |
//!     v
//! QuEraArtifactStore
//!     |
//!     v
//! AHS result
//!     |
//!     v
//! ExecutionResult
//! ```
//!
//! Remote execution is asynchronous.
//!
//! This adapter never pretends that submission synchronously produces a
//! quantum result.
//!
//! # Security
//!
//! This module never accepts or stores credentials.
//!
//! It rejects secret-like configuration and metadata names including:
//!
//! - api_key
//! - access_token
//! - refresh_token
//! - authorization
//! - password
//! - private_key
//! - secret
//! - session_token
//! - cookie
//!
//! Provider responses are parsed into bounded structures before they enter
//! canonical Zamani types.
//!
//! # Retry safety
//!
//! `CreateQuantumTask` is a mutating operation.
//!
//! This adapter does NOT automatically retry a submission after an ambiguous
//! transport failure. Duplicate quantum-task submission is unacceptable.
//!
//! Status, result metadata and device-health reads may safely be retried by
//! the external transport/orchestration layer according to its retry policy.
//!
//! # No-reedit contract
//!
//! This file consumes the existing stable contracts:
//!
//! - `hardware/backend.rs`;
//! - `hardware/backend_trait.rs`;
//! - `hardware/adapters/generic.rs`;
//! - `serde_json`.
//!
//! It does not require modifications to those contracts.
//!
//! The only module-registration integration required is:
//!
//! ```rust
//! pub mod quera;
//! ```
//!
//! in `hardware/adapters/mod.rs` once that module is introduced.
//!
//! Provider registration remains the responsibility of
//! `provider_registry.rs`.
//!
//! Device discovery remains the responsibility of `discovery.rs`.
//!
//! Benchmarking consumes this adapter through `QuantumBackendAdapter`.
//!
//! Danga consumes the same provider-neutral execution contract.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe Rust.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! ```text
//! #![deny(unsafe_code)]
//! ```
//!
//! # External interoperability basis
//!
//! Amazon Braket currently identifies QuEra Aquila as an Analog Hamiltonian
//! Simulation QPU and exposes the Aquila device ARN through Braket.
//!
//! Aquila's AHS schema is `braket.ir.ahs.program`, version `1`.
//!
//! The adapter intentionally follows the provider's actual task lifecycle
//! rather than creating a fictional direct QuEra HTTP protocol.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use serde_json::{json, Value};

use super::generic::{
    send_request,
    ProviderOperation,
    ProviderRequest,
    ProviderTransport,
    TransportMethod,
};

use super::super::backend::{
    BackendCapabilities,
    BackendError,
    BackendHealth,
    BackendHealthState,
    BackendKind,
    BackendLimits,
    BackendMetadata,
    BackendStatus,
    ExecutionRequest,
    ExecutionResult,
    QuantumBackend,
    QuantumWorkloadKind,
};

use super::super::backend_trait::{
    BackendAdapterInfo,
    BackendCancellation,
    BackendJob,
    BackendJobId,
    BackendJobState,
    BackendJobStatus,
    BackendProgram,
    CancellationOutcome,
    QuantumBackendAdapter,
};

/// Stable Zamani provider identifier.
pub const QUERA_PROVIDER_ID: &str = "quera";

/// Stable adapter identifier.
pub const QUERA_ADAPTER_ID: &str =
    "zamani.quantum.hardware.adapters.quera";

/// Adapter semantic version.
pub const QUERA_ADAPTER_VERSION: &str = "1.0.0";

/// Amazon Braket API version represented by this adapter.
pub const QUERA_BRAKET_API_VERSION: &str = "2019-09-01";

/// Canonical Aquila backend identifier.
pub const QUERA_AQUILA_BACKEND_ID: &str = "quera/Aquila";

/// Canonical Aquila display name.
pub const QUERA_AQUILA_NAME: &str = "QuEra Aquila";

/// Current Aquila Amazon Braket device ARN.
pub const QUERA_AQUILA_DEVICE_ARN: &str =
    "arn:aws:braket:us-east-1::device/qpu/quera/Aquila";

/// Amazon Braket CreateQuantumTask endpoint.
pub const BRAKET_CREATE_QUANTUM_TASK_ENDPOINT: &str =
    "/quantum-task";

/// Amazon Braket GetQuantumTask endpoint prefix.
pub const BRAKET_GET_QUANTUM_TASK_PREFIX: &str =
    "/quantum-task/";

/// Amazon Braket CancelQuantumTask endpoint suffix.
pub const BRAKET_CANCEL_QUANTUM_TASK_SUFFIX: &str =
    "/cancel";

/// Amazon Braket GetDevice endpoint prefix.
pub const BRAKET_GET_DEVICE_PREFIX: &str = "/device/";

/// Canonical Braket AHS program format.
pub const QUERA_AHS_PROGRAM_FORMAT: &str =
    "braket.ir.ahs.program";

/// Canonical Braket AHS schema version.
pub const QUERA_AHS_SCHEMA_VERSION: &str = "1";

/// Alternate explicit Zamani format identifier.
///
/// This format is accepted only when its payload is still a valid Braket AHS
/// schema. The adapter never translates arbitrary custom JSON into AHS.
pub const QUERA_AHS_PROGRAM_V1_FORMAT: &str =
    "braket.ir.ahs.program.v1";

/// Maximum Aquila atom count.
pub const QUERA_AQUILA_MAX_QUBITS: usize = 256;

/// Current documented Aquila minimum atom separation in metres.
pub const QUERA_AQUILA_MIN_ATOM_DISTANCE_METRES: f64 = 4.0e-6;

/// Current documented Aquila horizontal device dimension in micrometres.
pub const QUERA_AQUILA_MAX_X_MICROMETRES: f64 = 76.0;

/// Current documented Aquila vertical device dimension in micrometres.
pub const QUERA_AQUILA_MAX_Y_MICROMETRES: f64 = 75.0;

/// Current documented Aquila maximum shots.
pub const QUERA_AQUILA_MAX_SHOTS: usize = 1_000;

/// Maximum provider action payload.
pub const MAX_QUERA_PROGRAM_BYTES: usize = 16 * 1024 * 1024;

/// Maximum result artifact payload retained by the adapter.
pub const MAX_QUERA_RESULT_BYTES: usize = 256 * 1024 * 1024;

/// Maximum provider task ARN length.
pub const MAX_QUERA_TASK_ARN_LENGTH: usize = 256;

/// Maximum device ARN length.
pub const MAX_QUERA_DEVICE_ARN_LENGTH: usize = 256;

/// Maximum client-token length accepted by Braket.
pub const MAX_BRAKET_CLIENT_TOKEN_LENGTH: usize = 64;

/// Maximum S3 bucket length accepted by Braket.
pub const MAX_BRAKET_S3_BUCKET_LENGTH: usize = 63;

/// Maximum S3 key-prefix length accepted by Braket.
pub const MAX_BRAKET_S3_KEY_PREFIX_LENGTH: usize = 1024;

/// Maximum Braket job-token length.
pub const MAX_BRAKET_JOB_TOKEN_LENGTH: usize = 128;

/// Maximum number of task tags.
pub const MAX_BRAKET_TAGS: usize = 50;

/// Maximum tag key length.
pub const MAX_BRAKET_TAG_KEY_LENGTH: usize = 128;

/// Maximum tag value length.
pub const MAX_BRAKET_TAG_VALUE_LENGTH: usize = 256;

/// Maximum provider status string.
pub const MAX_PROVIDER_STATUS_LENGTH: usize = 128;

/// Maximum provider failure reason.
pub const MAX_PROVIDER_FAILURE_REASON_LENGTH: usize = 4096;

/// Maximum result measurement count.
pub const MAX_RESULT_MEASUREMENTS: usize = 1_000_000;

/// Maximum AHS register site count.
pub const MAX_AHS_REGISTER_SITES: usize =
    QUERA_AQUILA_MAX_QUBITS;

/// Maximum result bitstring length.
pub const MAX_RESULT_BITSTRING_LENGTH: usize =
    QUERA_AQUILA_MAX_QUBITS;

/// Experimental capability identifier.
pub const EXPERIMENTAL_LOCAL_DETUNING: &str =
    "local_detuning";

/// Provider-native cancellation state.
pub const BRAKET_CANCELLING: &str = "CANCELLING";

/// Provider-native completed state.
pub const BRAKET_COMPLETED: &str = "COMPLETED";

/// Provider-native created state.
pub const BRAKET_CREATED: &str = "CREATED";

/// Provider-native queued state.
pub const BRAKET_QUEUED: &str = "QUEUED";

/// Provider-native running state.
pub const BRAKET_RUNNING: &str = "RUNNING";

/// Provider-native failed state.
pub const BRAKET_FAILED: &str = "FAILED";

/// Provider-native cancelled state.
pub const BRAKET_CANCELLED: &str = "CANCELLED";

/// Provider-native task API operation names.
pub mod operation {
    /// Create a quantum task.
    pub const SUBMIT: &str = "braket.create_quantum_task";

    /// Retrieve a quantum task.
    pub const STATUS: &str = "braket.get_quantum_task";

    /// Cancel a quantum task.
    pub const CANCEL: &str = "braket.cancel_quantum_task";

    /// Retrieve device information.
    pub const HEALTH: &str = "braket.get_device";
}

/// Errors specific to deterministic QuEra/Aquila normalization.
#[derive(Debug, Clone, PartialEq)]
pub enum QuEraAdapterError {
    /// Invalid backend target.
    InvalidTarget(String),

    /// Invalid device ARN.
    InvalidDeviceArn,

    /// Unsupported program format.
    UnsupportedProgramFormat(String),

    /// Invalid JSON.
    InvalidJson,

    /// AHS payload is not an object.
    InvalidProgramShape,

    /// Missing AHS schema header.
    MissingSchemaHeader,

    /// Wrong AHS schema name.
    InvalidSchemaName(String),

    /// Wrong AHS schema version.
    InvalidSchemaVersion(String),

    /// Missing required AHS field.
    MissingField(&'static str),

    /// Invalid JSON type.
    InvalidFieldType(&'static str),

    /// Empty atom register.
    EmptyAtomRegister,

    /// Too many atoms.
    AtomCountExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Filling vector does not match sites.
    FillingLengthMismatch {
        sites: usize,
        filling: usize,
    },

    /// Invalid atom coordinate.
    InvalidCoordinate,

    /// Atom coordinates are too close.
    AtomDistanceViolation {
        first: usize,
        second: usize,
        distance_metres: f64,
        minimum_metres: f64,
    },

    /// Atom lies outside the documented device dimensions.
    CoordinateOutsideDevice {
        index: usize,
        x_micrometres: f64,
        y_micrometres: f64,
    },

    /// Invalid shots.
    InvalidShots,

    /// Shots exceed the currently documented Aquila limit.
    ShotsExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Experimental capability was requested but not explicitly enabled.
    ExperimentalCapabilityDisabled(&'static str),

    /// Provider task response is malformed.
    InvalidTaskResponse,

    /// Missing task ARN.
    MissingTaskArn,

    /// Provider status missing.
    MissingStatus,

    /// Provider returned an unknown task state.
    UnknownTaskState(String),

    /// Result is not available.
    ResultUnavailable,

    /// Result JSON is malformed.
    InvalidResultJson,

    /// Result shape cannot be normalized.
    UnsupportedResultShape,

    /// Result contains malformed measurement.
    InvalidMeasurement,

    /// Result has too many measurements.
    TooManyMeasurements,

    /// Result shot accounting is inconsistent.
    ResultShotMismatch {
        requested: usize,
        successful: usize,
    },

    /// Provider returned a different device.
    DeviceMismatch {
        expected: String,
        actual: String,
    },

    /// Artifact store failed.
    ArtifactStore(String),

    /// Provider transport failed.
    Provider(String),

    /// Provider response was malformed.
    InvalidProviderResponse(String),
}

impl fmt::Display for QuEraAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTarget(target) => {
                write!(formatter, "invalid QuEra target '{target}'")
            }

            Self::InvalidDeviceArn => {
                formatter.write_str("invalid Amazon Braket Aquila device ARN")
            }

            Self::UnsupportedProgramFormat(format) => {
                write!(
                    formatter,
                    "unsupported QuEra AHS program format '{format}'"
                )
            }

            Self::InvalidJson => {
                formatter.write_str("QuEra AHS program is not valid JSON")
            }

            Self::InvalidProgramShape => {
                formatter.write_str(
                    "QuEra AHS program must be a JSON object",
                )
            }

            Self::MissingSchemaHeader => {
                formatter.write_str(
                    "QuEra AHS program is missing braketSchemaHeader",
                )
            }

            Self::InvalidSchemaName(name) => {
                write!(
                    formatter,
                    "invalid QuEra AHS schema name '{name}'"
                )
            }

            Self::InvalidSchemaVersion(version) => {
                write!(
                    formatter,
                    "unsupported QuEra AHS schema version '{version}'"
                )
            }

            Self::MissingField(field) => {
                write!(
                    formatter,
                    "QuEra AHS program is missing '{field}'"
                )
            }

            Self::InvalidFieldType(field) => {
                write!(
                    formatter,
                    "QuEra AHS field '{field}' has an invalid type"
                )
            }

            Self::EmptyAtomRegister => {
                formatter.write_str(
                    "QuEra AHS atom register cannot be empty",
                )
            }

            Self::AtomCountExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "Aquila atom count {requested} exceeds maximum {maximum}"
                )
            }

            Self::FillingLengthMismatch { sites, filling } => {
                write!(
                    formatter,
                    "Aquila register contains {sites} sites but {filling} filling values"
                )
            }

            Self::InvalidCoordinate => {
                formatter.write_str(
                    "QuEra AHS atom coordinates must be finite numeric values",
                )
            }

            Self::AtomDistanceViolation {
                first,
                second,
                distance_metres,
                minimum_metres,
            } => {
                write!(
                    formatter,
                    "Aquila atoms {first} and {second} are too close: {distance_metres:.9e} m < {minimum_metres:.9e} m"
                )
            }

            Self::CoordinateOutsideDevice {
                index,
                x_micrometres,
                y_micrometres,
            } => {
                write!(
                    formatter,
                    "Aquila atom {index} at ({x_micrometres}, {y_micrometres}) micrometres is outside the documented device dimensions"
                )
            }

            Self::InvalidShots => {
                formatter.write_str(
                    "Aquila shot count must be greater than zero",
                )
            }

            Self::ShotsExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "Aquila shot count {requested} exceeds maximum {maximum}"
                )
            }

            Self::ExperimentalCapabilityDisabled(capability) => {
                write!(
                    formatter,
                    "experimental QuEra capability '{capability}' is not enabled"
                )
            }

            Self::InvalidTaskResponse => {
                formatter.write_str(
                    "Amazon Braket returned an invalid quantum-task response",
                )
            }

            Self::MissingTaskArn => {
                formatter.write_str(
                    "Amazon Braket response is missing quantumTaskArn",
                )
            }

            Self::MissingStatus => {
                formatter.write_str(
                    "Amazon Braket task response is missing status",
                )
            }

            Self::UnknownTaskState(state) => {
                write!(
                    formatter,
                    "unknown Amazon Braket quantum-task state '{state}'"
                )
            }

            Self::ResultUnavailable => {
                formatter.write_str(
                    "QuEra result is not available",
                )
            }

            Self::InvalidResultJson => {
                formatter.write_str(
                    "QuEra AHS result is not valid JSON",
                )
            }

            Self::UnsupportedResultShape => {
                formatter.write_str(
                    "QuEra AHS result cannot be safely normalized",
                )
            }

            Self::InvalidMeasurement => {
                formatter.write_str(
                    "QuEra AHS result contains an invalid measurement",
                )
            }

            Self::TooManyMeasurements => {
                formatter.write_str(
                    "QuEra AHS result contains too many measurements",
                )
            }

            Self::ResultShotMismatch {
                requested,
                successful,
            } => {
                write!(
                    formatter,
                    "QuEra AHS result contains {successful} successful shots but {requested} were requested"
                )
            }

            Self::DeviceMismatch { expected, actual } => {
                write!(
                    formatter,
                    "Braket task targets '{actual}' instead of expected '{expected}'"
                )
            }

            Self::ArtifactStore(message) => {
                write!(
                    formatter,
                    "QuEra result artifact retrieval failed: {message}"
                )
            }

            Self::Provider(message) => {
                write!(
                    formatter,
                    "Amazon Braket provider operation failed: {message}"
                )
            }

            Self::InvalidProviderResponse(message) => {
                write!(
                    formatter,
                    "invalid Amazon Braket provider response: {message}"
                )
            }
        }
    }
}

impl std::error::Error for QuEraAdapterError {}

/// Result artifact abstraction.
///
/// Amazon Braket stores completed quantum-task results in S3. This adapter
/// deliberately does not own AWS/S3 credentials or an S3 client.
///
/// A concrete implementation can use the AWS SDK, an authenticated gateway,
/// a local test store, or another approved transport.
pub trait QuEraArtifactStore: Send + Sync {
    /// Retrieves the exact result artifact for a completed quantum task.
    fn get_task_result(
        &self,
        bucket: &str,
        directory: &str,
    ) -> Result<Vec<u8>, QuEraArtifactStoreError>;

    /// Stable artifact-store implementation identifier.
    fn store_id(&self) -> &str;
}

/// Artifact-store failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuEraArtifactStoreError {
    /// Artifact does not exist.
    NotFound,

    /// Access was denied.
    AccessDenied,

    /// Artifact store is temporarily unavailable.
    Unavailable(String),

    /// Artifact was malformed.
    InvalidPayload(String),

    /// Retrieval timed out.
    Timeout,

    /// Other safe failure.
    Other(String),
}

impl fmt::Display for QuEraArtifactStoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => {
                formatter.write_str(
                    "result artifact was not found",
                )
            }

            Self::AccessDenied => {
                formatter.write_str(
                    "access to result artifact was denied",
                )
            }

            Self::Unavailable(message) => {
                write!(
                    formatter,
                    "artifact store unavailable: {message}"
                )
            }

            Self::InvalidPayload(message) => {
                write!(
                    formatter,
                    "artifact payload is invalid: {message}"
                )
            }

            Self::Timeout => {
                formatter.write_str(
                    "result artifact retrieval timed out",
                )
            }

            Self::Other(message) => {
                write!(
                    formatter,
                    "artifact retrieval failed: {message}"
                )
            }
        }
    }
}

impl std::error::Error for QuEraArtifactStoreError {}

/// Provider-specific QuEra/Aquila configuration.
///
/// No credential material is accepted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuEraAquilaConfig {
    /// Amazon Braket device ARN.
    pub device_arn: String,

    /// S3 bucket used by Braket for task results.
    pub output_s3_bucket: String,

    /// S3 result key prefix.
    pub output_s3_key_prefix: String,

    /// Optional Braket device parameters JSON.
    pub device_parameters: Option<Vec<u8>>,

    /// Whether experimental capabilities may be enabled.
    pub allow_experimental_local_detuning: bool,

    /// Whether the provider-native AHS action is accepted.
    ///
    /// This remains explicit even though AHS is the primary native format.
    pub allow_ahs_program: bool,

    /// Optional Braket hybrid-job token.
    pub job_token: Option<String>,

    /// Optional non-secret task tags.
    pub tags: BTreeMap<String, String>,
}

impl QuEraAquilaConfig {
    /// Creates a validated production configuration.
    pub fn new(
        output_s3_bucket: impl Into<String>,
        output_s3_key_prefix: impl Into<String>,
    ) -> Result<Self, BackendError> {
        let config = Self {
            device_arn: QUERA_AQUILA_DEVICE_ARN.to_owned(),
            output_s3_bucket: output_s3_bucket.into(),
            output_s3_key_prefix: output_s3_key_prefix.into(),
            device_parameters: None,
            allow_experimental_local_detuning: false,
            allow_ahs_program: true,
            job_token: None,
            tags: BTreeMap::new(),
        };

        config.validate()?;
        Ok(config)
    }

    /// Changes the target device ARN.
    pub fn with_device_arn(
        mut self,
        device_arn: impl Into<String>,
    ) -> Result<Self, BackendError> {
        self.device_arn = device_arn.into();
        self.validate()?;
        Ok(self)
    }

    /// Adds provider device parameters.
    pub fn with_device_parameters(
        mut self,
        parameters: impl Into<Vec<u8>>,
    ) -> Result<Self, BackendError> {
        let parameters = parameters.into();

        if parameters.is_empty()
            || parameters.len() > 48 * 1024
        {
            return Err(BackendError::ExecutionUnavailable(
                "invalid QuEra device parameters size".to_owned(),
            ));
        }

        if serde_json::from_slice::<Value>(&parameters).is_err() {
            return Err(BackendError::ExecutionUnavailable(
                "QuEra device parameters must be valid JSON"
                    .to_owned(),
            ));
        }

        self.device_parameters = Some(parameters);
        Ok(self)
    }

    /// Enables experimental local detuning.
    pub fn allow_experimental_local_detuning(
        mut self,
        enabled: bool,
    ) -> Self {
        self.allow_experimental_local_detuning = enabled;
        self
    }

    /// Enables/disables AHS execution.
    pub fn allow_ahs_program(
        mut self,
        enabled: bool,
    ) -> Self {
        self.allow_ahs_program = enabled;
        self
    }

    /// Sets an optional Braket hybrid-job token.
    pub fn with_job_token(
        mut self,
        token: impl Into<String>,
    ) -> Result<Self, BackendError> {
        let token = token.into();

        validate_safe_string(
            "job_token",
            &token,
            MAX_BRAKET_JOB_TOKEN_LENGTH,
        )?;

        self.job_token = Some(token);
        Ok(self)
    }

    /// Adds a safe Braket task tag.
    pub fn with_tag(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, BackendError> {
        let key = key.into();
        let value = value.into();

        validate_safe_string(
            "tag_key",
            &key,
            MAX_BRAKET_TAG_KEY_LENGTH,
        )?;

        validate_safe_string(
            "tag_value",
            &value,
            MAX_BRAKET_TAG_VALUE_LENGTH,
        )?;

        if contains_secret_marker(&key)
            || contains_secret_marker(&value)
        {
            return Err(BackendError::SecretLikeMetadata { key });
        }

        if self.tags.len() >= MAX_BRAKET_TAGS
            && !self.tags.contains_key(&key)
        {
            return Err(BackendError::MetadataLimitExceeded {
                maximum: MAX_BRAKET_TAGS,
            });
        }

        self.tags.insert(key, value);
        Ok(self)
    }

    /// Validates configuration.
    pub fn validate(&self) -> Result<(), BackendError> {
        validate_device_arn(&self.device_arn)?;

        validate_safe_string(
            "output_s3_bucket",
            &self.output_s3_bucket,
            MAX_BRAKET_S3_BUCKET_LENGTH,
        )?;

        if self.output_s3_bucket.len() < 3 {
            return Err(BackendError::ExecutionUnavailable(
                "Braket output S3 bucket is too short".to_owned(),
            ));
        }

        validate_s3_key_prefix(&self.output_s3_key_prefix)?;

        if let Some(parameters) = &self.device_parameters {
            if parameters.is_empty()
                || parameters.len() > 48 * 1024
            {
                return Err(BackendError::ExecutionUnavailable(
                    "invalid Braket device parameters".to_owned(),
                ));
            }
        }

        if let Some(token) = &self.job_token {
            validate_safe_string(
                "job_token",
                token,
                MAX_BRAKET_JOB_TOKEN_LENGTH,
            )?;
        }

        Ok(())
    }
}

/// Production QuEra Aquila adapter.
///
/// The adapter is immutable after construction. Provider transport and
/// artifact storage are injected, keeping authentication and infrastructure
/// concerns outside the provider-specific semantic layer.
pub struct QuEraAquilaAdapter {
    backend: QuantumBackend,
    adapter_info: BackendAdapterInfo,
    config: QuEraAquilaConfig,
    transport: Arc<dyn ProviderTransport>,
    artifact_store: Arc<dyn QuEraArtifactStore>,
}

impl fmt::Debug for QuEraAquilaAdapter {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("QuEraAquilaAdapter")
            .field("backend", &self.backend)
            .field("adapter_info", &self.adapter_info)
            .field("config", &self.config)
            .field("artifact_store", &self.artifact_store.store_id())
            .finish()
    }
}

impl QuEraAquilaAdapter {
    /// Creates a production QuEra Aquila adapter.
    ///
    /// The caller supplies authenticated provider transport and result
    /// artifact storage.
    pub fn new(
        config: QuEraAquilaConfig,
        transport: Arc<dyn ProviderTransport>,
        artifact_store: Arc<dyn QuEraArtifactStore>,
    ) -> Result<Self, BackendError> {
        config.validate()?;

        let backend = build_aquila_backend(&config)?;

        let mut adapter_info = BackendAdapterInfo::new(
            QUERA_ADAPTER_ID,
            QUERA_ADAPTER_VERSION,
            true,
        )?;

        adapter_info.provider_api_version =
            Some(QUERA_BRAKET_API_VERSION.to_owned());

        Ok(Self {
            backend,
            adapter_info,
            config,
            transport,
            artifact_store,
        })
    }

    /// Returns the immutable adapter configuration.
    pub fn config(&self) -> &QuEraAquilaConfig {
        &self.config
    }

    /// Returns the injected transport.
    pub fn transport(&self) -> &Arc<dyn ProviderTransport> {
        &self.transport
    }

    /// Returns the injected artifact store.
    pub fn artifact_store(
        &self,
    ) -> &Arc<dyn QuEraArtifactStore> {
        &self.artifact_store
    }

    /// Validates a provider-native AHS program without network access.
    pub fn validate_ahs_program(
        &self,
        program: &BackendProgram,
        shots: usize,
    ) -> Result<AhsProgramInfo, BackendError> {
        validate_ahs_program(
            program,
            shots,
            &self.config,
        )
        .map_err(to_backend_error)
    }

    /// Returns the expected Aquila ARN.
    pub fn device_arn(&self) -> &str {
        &self.config.device_arn
    }

    /// Builds the exact Braket CreateQuantumTask payload.
    ///
    /// No provider request is sent.
    pub fn build_submit_payload(
        &self,
        program: &BackendProgram,
        request: &ExecutionRequest,
    ) -> Result<Vec<u8>, BackendError> {
        self.preflight(request, program)?;

        let shots = request.workload.circuit.shots;

        let action = serde_json::from_slice::<Value>(
            program.bytes(),
        )
        .map_err(|_| {
            BackendError::ExecutionRejected(
                "QuEra AHS program is not valid JSON".to_owned(),
            )
        })?;

        let mut body = serde_json::Map::new();

        body.insert(
            "action".to_owned(),
            Value::String(
                serde_json::to_string(&action)
                    .map_err(|_| {
                        BackendError::ExecutionRejected(
                            "failed to serialize QuEra AHS action"
                                .to_owned(),
                        )
                    })?,
            ),
        );

        body.insert(
            "clientToken".to_owned(),
            Value::String(
                client_token_for_request(request),
            ),
        );

        body.insert(
            "deviceArn".to_owned(),
            Value::String(self.config.device_arn.clone()),
        );

        body.insert(
            "outputS3Bucket".to_owned(),
            Value::String(
                self.config.output_s3_bucket.clone(),
            ),
        );

        body.insert(
            "outputS3KeyPrefix".to_owned(),
            Value::String(
                self.config.output_s3_key_prefix.clone(),
            ),
        );

        let shots_u64 = u64::try_from(shots)
            .map_err(|_| {
                BackendError::ExecutionRejected(
                    "shot count cannot be represented by Braket"
                        .to_owned(),
                )
            })?;

        body.insert(
            "shots".to_owned(),
            Value::Number(
                serde_json::Number::from(shots_u64),
            ),
        );

        if let Some(parameters) =
            &self.config.device_parameters
        {
            let value = serde_json::from_slice::<Value>(
                parameters,
            )
            .map_err(|_| {
                BackendError::ExecutionRejected(
                    "configured QuEra deviceParameters are invalid JSON"
                        .to_owned(),
                )
            })?;

            body.insert(
                "deviceParameters".to_owned(),
                Value::String(
                    serde_json::to_string(&value)
                        .map_err(|_| {
                            BackendError::ExecutionRejected(
                                "failed to serialize QuEra deviceParameters"
                                    .to_owned(),
                            )
                        })?,
                ),
            );
        }

        if self.config.allow_experimental_local_detuning {
            body.insert(
                "experimentalCapabilities".to_owned(),
                json!({
                    "enabled": "ALL"
                }),
            );
        }

        if let Some(job_token) = &self.config.job_token {
            body.insert(
                "jobToken".to_owned(),
                Value::String(job_token.clone()),
            );
        }

        if !self.config.tags.is_empty() {
            body.insert(
                "tags".to_owned(),
                serde_json::to_value(&self.config.tags)
                    .map_err(|_| {
                        BackendError::ExecutionRejected(
                            "failed to serialize QuEra task tags"
                                .to_owned(),
                        )
                    })?,
            );
        }

        let encoded = serde_json::to_vec(
            &Value::Object(body),
        )
        .map_err(|_| {
            BackendError::ExecutionRejected(
                "failed to serialize Braket quantum-task request"
                    .to_owned(),
            )
        })?;

        if encoded.len() > MAX_QUERA_PROGRAM_BYTES {
            return Err(BackendError::MetadataLimitExceeded {
                maximum: MAX_QUERA_PROGRAM_BYTES,
            });
        }

        Ok(encoded)
    }

    /// Builds a status request without network access.
    pub fn build_status_request(
        &self,
        job: &BackendJobId,
    ) -> Result<ProviderRequest, BackendError> {
        self.build_status_request_internal(job, true)
    }

    /// Builds a cancellation request without network access.
    pub fn build_cancel_request(
        &self,
        job: &BackendJobId,
    ) -> Result<ProviderRequest, BackendError> {
        let target = format!(
            "{}{}/{}",
            BRAKET_GET_QUANTUM_TASK_PREFIX,
            job.as_str(),
            BRAKET_CANCEL_QUANTUM_TASK_SUFFIX
        );

        let request_id = format!(
            "quera-cancel-{}",
            sanitize_request_identifier(job.as_str())
        );

        let body = json!({
            "clientToken": client_token_for_job(job)
        });

        let body = serde_json::to_vec(&body)
            .map_err(|_| {
                BackendError::ExecutionRejected(
                    "failed to serialize Braket cancellation request"
                        .to_owned(),
                )
            })?;

        ProviderRequest::builder(
            ProviderOperation::Cancel,
            TransportMethod::Put,
            target,
            request_id,
        )
        .body(body)
        .map_err(|_| {
            BackendError::ExecutionRejected(
                "failed to construct Braket cancellation request"
                    .to_owned(),
            )
        })?
        .build()
        .map_err(|_| {
            BackendError::ExecutionRejected(
                "invalid Braket cancellation request"
                    .to_owned(),
            )
        })
    }

    fn build_status_request_internal(
        &self,
        job: &BackendJobId,
        include_queue: bool,
    ) -> Result<ProviderRequest, BackendError> {
        let target = format!(
            "{}{}",
            BRAKET_GET_QUANTUM_TASK_PREFIX,
            job.as_str()
        );

        let request_id = format!(
            "quera-status-{}",
            sanitize_request_identifier(job.as_str())
        );

        let mut builder = ProviderRequest::builder(
            ProviderOperation::GetJobStatus,
            TransportMethod::Get,
            target,
            request_id,
        );

        if include_queue {
            builder = builder
                .query(
                    "additionalAttributeNames",
                    "QueueInfo",
                )
                .map_err(|_| {
                    BackendError::ExecutionRejected(
                        "failed to construct Braket queue query"
                            .to_owned(),
                    )
                })?;
        }

        builder.build().map_err(|_| {
            BackendError::ExecutionRejected(
                "invalid Braket status request".to_owned(),
            )
        })
    }

    fn send(
        &self,
        request: &ProviderRequest,
    ) -> Result<super::generic::ProviderResponse, BackendError> {
        send_request(self.transport.as_ref(), request)
            .map_err(|error| {
                BackendError::ExecutionUnavailable(
                    format!(
                        "{}: {}",
                        QUERA_PROVIDER_ID,
                        sanitize_provider_message(
                            &error.message,
                        )
                    ),
                )
            })
    }
}

impl QuantumBackendAdapter for QuEraAquilaAdapter {
    fn backend(&self) -> &QuantumBackend {
        &self.backend
    }

    fn adapter_info(&self) -> &BackendAdapterInfo {
        &self.adapter_info
    }

    fn preflight(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<(), BackendError> {
        request.validate_structure()?;

        if !self.backend.metadata.status.is_usable() {
            return Err(BackendError::BackendUnavailable {
                backend_id: self.backend.metadata.id.clone(),
                status: self.backend.metadata.status,
            });
        }

        self.backend.preflight(request)?;

        if request.workload.kind
            != QuantumWorkloadKind::AnalogProgram
        {
            return Err(BackendError::UnsupportedWorkload {
                workload: request.workload.kind,
            });
        }

        if request.workload.circuit.requires_analog_control
            != true
        {
            return Err(BackendError::ExecutionRejected(
                "QuEra Aquila requires an analog-control workload"
                    .to_owned(),
            ));
        }

        if request.workload.circuit.qubit_count
            > QUERA_AQUILA_MAX_QUBITS
        {
            return Err(BackendError::QubitLimitExceeded {
                requested: request
                    .workload
                    .circuit
                    .qubit_count,
                maximum: QUERA_AQUILA_MAX_QUBITS,
            });
        }

        let shots = request.workload.circuit.shots;

        if shots == 0 {
            return Err(BackendError::InvalidShots);
        }

        if shots > QUERA_AQUILA_MAX_SHOTS {
            return Err(BackendError::ShotLimitExceeded {
                requested: shots,
                maximum: QUERA_AQUILA_MAX_SHOTS,
            });
        }

        if program.len() > MAX_QUERA_PROGRAM_BYTES {
            return Err(BackendError::MetadataLimitExceeded {
                maximum: MAX_QUERA_PROGRAM_BYTES,
            });
        }

        self.validate_ahs_program(
            program,
            shots,
        )?;

        Ok(())
    }

    fn submit(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<BackendJob, BackendError> {
        self.preflight(request, program)?;

        let body =
            self.build_submit_payload(program, request)?;

        let request_id = request
            .request_id
            .clone()
            .unwrap_or_else(|| {
                format!(
                    "quera-submit-{}",
                    sanitize_request_identifier(
                        &self.config.device_arn,
                    )
                )
            });

        let provider_request = ProviderRequest::builder(
            ProviderOperation::Submit,
            TransportMethod::Post,
            BRAKET_CREATE_QUANTUM_TASK_ENDPOINT,
            request_id.clone(),
        )
        .body(body)
        .map_err(|_| {
            BackendError::ExecutionRejected(
                "failed to build QuEra submission payload"
                    .to_owned(),
            )
        })?
        .idempotency_key(
            client_token_for_request(request),
        )
        .map_err(|_| {
            BackendError::ExecutionRejected(
                "failed to build Braket client token"
                    .to_owned(),
            )
        })?
        .build()
        .map_err(|_| {
            BackendError::ExecutionRejected(
                "invalid Braket submission request"
                    .to_owned(),
            )
        })?;

        let response = self.send(&provider_request)?;

        if !response.is_success() {
            return Err(map_provider_http_error(
                response.status_code,
                &response.body,
            ));
        }

        let json = parse_json(
            &response.body,
        )?;

        let task_arn = json
            .get("quantumTaskArn")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                to_backend_error(
                    QuEraAdapterError::MissingTaskArn,
                )
            })?;

        validate_task_arn(task_arn)
            .map_err(to_backend_error)?;

        let job_id =
            BackendJobId::new(task_arn.to_owned())?;

        BackendJob::new(
            job_id,
            self.backend.metadata.id.clone(),
            Some(request_id),
            BackendJobState::Created,
        )
    }

    fn status(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendJobStatus, BackendError> {
        validate_task_arn(job.as_str())
            .map_err(to_backend_error)?;

        let provider_request =
            self.build_status_request(job)?;

        let response = self.send(&provider_request)?;

        if !response.is_success() {
            return Err(map_provider_http_error(
                response.status_code,
                &response.body,
            ));
        }

        let json =
            parse_json(&response.body)?;

        let actual_device = json
            .get("deviceArn")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                to_backend_error(
                    QuEraAdapterError::MissingField(
                        "deviceArn",
                    ),
                )
            })?;

        if actual_device != self.config.device_arn {
            return Err(to_backend_error(
                QuEraAdapterError::DeviceMismatch {
                    expected: self.config.device_arn.clone(),
                    actual: actual_device.to_owned(),
                },
            ));
        }

        let status = json
            .get("status")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                to_backend_error(
                    QuEraAdapterError::MissingStatus,
                )
            })?;

        let state =
            map_braket_state(status)?;

        let job = BackendJob::new(
            job.clone(),
            self.backend.metadata.id.clone(),
            None,
            state,
        )?;

        let provider_status =
            bounded_optional_string(
                status,
                MAX_PROVIDER_STATUS_LENGTH,
            )?;

        let queue_position = json
            .get("queueInfo")
            .and_then(|value| {
                value
                    .get("position")
                    .and_then(Value::as_str)
            })
            .and_then(parse_queue_position);

        let result_available =
            state == BackendJobState::Completed;

        Ok(BackendJobStatus {
            job,
            provider_status,
            queue_position,
            estimated_wait: None,
            result_available,
        })
    }

    fn result(
        &self,
        job: &BackendJobId,
    ) -> Result<ExecutionResult, BackendError> {
        validate_task_arn(job.as_str())
            .map_err(to_backend_error)?;

        let status_request =
            self.build_status_request_internal(
                job,
                false,
            )?;

        let response =
            self.send(&status_request)?;

        if !response.is_success() {
            return Err(map_provider_http_error(
                response.status_code,
                &response.body,
            ));
        }

        let task =
            parse_json(&response.body)?;

        let status = task
            .get("status")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                to_backend_error(
                    QuEraAdapterError::MissingStatus,
                )
            })?;

        if status != BRAKET_COMPLETED {
            return Err(to_backend_error(
                QuEraAdapterError::ResultUnavailable,
            ));
        }

        let device = task
            .get("deviceArn")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                to_backend_error(
                    QuEraAdapterError::MissingField(
                        "deviceArn",
                    ),
                )
            })?;

        if device != self.config.device_arn {
            return Err(to_backend_error(
                QuEraAdapterError::DeviceMismatch {
                    expected: self.config.device_arn.clone(),
                    actual: device.to_owned(),
                },
            ));
        }

        let shots = task
            .get("shots")
            .and_then(Value::as_u64)
            .and_then(|value| usize::try_from(value).ok())
            .ok_or_else(|| {
                to_backend_error(
                    QuEraAdapterError::MissingField(
                        "shots",
                    ),
                )
            })?;

        if shots == 0
            || shots > QUERA_AQUILA_MAX_SHOTS
        {
            return Err(to_backend_error(
                QuEraAdapterError::ShotsExceeded {
                    requested: shots,
                    maximum: QUERA_AQUILA_MAX_SHOTS,
                },
            ));
        }

        let successful = task
            .get("numSuccessfulShots")
            .and_then(Value::as_u64)
            .and_then(|value| usize::try_from(value).ok())
            .unwrap_or(shots);

        if successful != shots {
            return Err(to_backend_error(
                QuEraAdapterError::ResultShotMismatch {
                    requested: shots,
                    successful,
                },
            ));
        }

        let bucket = task
            .get("outputS3Bucket")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                to_backend_error(
                    QuEraAdapterError::MissingField(
                        "outputS3Bucket",
                    ),
                )
            })?;

        let directory = task
            .get("outputS3Directory")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                to_backend_error(
                    QuEraAdapterError::MissingField(
                        "outputS3Directory",
                    ),
                )
            })?;

        validate_safe_string(
            "outputS3Bucket",
            bucket,
            MAX_BRAKET_S3_BUCKET_LENGTH,
        )?;

        validate_s3_key_prefix(directory)?;

        let artifact =
            self.artifact_store
                .get_task_result(
                    bucket,
                    directory,
                )
                .map_err(|error| {
                    BackendError::ExecutionUnavailable(
                        format!(
                            "QuEra artifact store '{}': {}",
                            self.artifact_store.store_id(),
                            error
                        ),
                    )
                })?;

        if artifact.len()
            > MAX_QUERA_RESULT_BYTES
        {
            return Err(BackendError::MetadataLimitExceeded {
                maximum: MAX_QUERA_RESULT_BYTES,
            });
        }

        normalize_ahs_result(
            &artifact,
            &self.backend.metadata.id,
            shots,
            job.as_str(),
            self.artifact_store.store_id(),
        )
        .map_err(to_backend_error)
    }

    fn cancel(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendCancellation, BackendError> {
        validate_task_arn(job.as_str())
            .map_err(to_backend_error)?;

        let status = self.status(job)?;

        if status.job.state.is_terminal() {
            return Ok(BackendCancellation {
                job: job.clone(),
                outcome: CancellationOutcome::AlreadyTerminal,
            });
        }

        let provider_request =
            self.build_cancel_request(job)?;

        let response =
            self.send(&provider_request)?;

        if !response.is_success() {
            if response.status_code == 409 {
                return Ok(BackendCancellation {
                    job: job.clone(),
                    outcome: CancellationOutcome::AlreadyTerminal,
                });
            }

            return Err(map_provider_http_error(
                response.status_code,
                &response.body,
            ));
        }

        let json =
            parse_json(&response.body)?;

        let cancellation_status =
            json.get("cancellationStatus")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    to_backend_error(
                        QuEraAdapterError::MissingField(
                            "cancellationStatus",
                        ),
                    )
                })?;

        let outcome =
            match cancellation_status {
                BRAKET_CANCELLING => {
                    CancellationOutcome::Pending
                }

                BRAKET_CANCELLED => {
                    CancellationOutcome::Accepted
                }

                _ => {
                    return Err(
                        to_backend_error(
                            QuEraAdapterError::UnknownTaskState(
                                cancellation_status
                                    .to_owned(),
                            ),
                        ),
                    );
                }
            };

        Ok(BackendCancellation {
            job: job.clone(),
            outcome,
        })
    }

    fn health(
        &self,
    ) -> Result<BackendHealth, BackendError> {
        let target = format!(
            "{}{}",
            BRAKET_GET_DEVICE_PREFIX,
            self.config.device_arn
        );

        let request = ProviderRequest::builder(
            ProviderOperation::GetHealth,
            TransportMethod::Get,
            target,
            "quera-health",
        )
        .build()
        .map_err(|_| {
            BackendError::ExecutionRejected(
                "failed to build QuEra health request"
                    .to_owned(),
            )
        })?;

        let response =
            self.send(&request)?;

        if !response.is_success() {
            return Err(map_provider_http_error(
                response.status_code,
                &response.body,
            ));
        }

        let json =
            parse_json(&response.body)?;

        let provider_status =
            json.get("deviceStatus")
                .and_then(Value::as_str)
                .unwrap_or("UNKNOWN");

        let provider_status =
            provider_status.to_ascii_uppercase();

        let (state, backend_status) =
            match provider_status.as_str() {
                "ONLINE" | "AVAILABLE" => (
                    BackendHealthState::Healthy,
                    BackendStatus::Available,
                ),

                "OFFLINE" => (
                    BackendHealthState::Unreachable,
                    BackendStatus::Offline,
                ),

                "RETIRED" => (
                    BackendHealthState::Unhealthy,
                    BackendStatus::Retired,
                ),

                _ => (
                    BackendHealthState::Unknown,
                    BackendStatus::Unknown,
                ),
            };

        let message = json
            .get("deviceName")
            .and_then(Value::as_str)
            .map(|name| {
                format!("Amazon Braket device {name}")
            });

        Ok(BackendHealth {
            state,
            backend_status,
            message,
        })
    }

    fn supports_cancellation(&self) -> bool {
        true
    }

    fn supports_queue_info(&self) -> bool {
        // The canonical trait's queue_info() has no job argument.
        //
        // Braket exposes queue information per quantum task through
        // GetQuantumTask(... additionalAttributeNames=QueueInfo).
        //
        // Therefore claiming global queue support here would be misleading.
        false
    }

    fn supports_synchronous_execution(&self) -> bool {
        false
    }
}

impl super::super::backend_trait::ConformantQuantumBackendAdapter
    for QuEraAquilaAdapter
{
}

/// Metadata describing the validated AHS program.
#[derive(Debug, Clone, PartialEq)]
pub struct AhsProgramInfo {
    /// Number of declared sites.
    pub site_count: usize,

    /// Whether all sites are filled.
    pub fully_filled: bool,

    /// Whether local detuning was requested.
    pub uses_local_detuning: bool,

    /// AHS schema version.
    pub schema_version: String,
}

fn build_aquila_backend(
    config: &QuEraAquilaConfig,
) -> Result<QuantumBackend, BackendError> {
    let mut metadata = BackendMetadata::new(
        QUERA_AQUILA_BACKEND_ID,
        QUERA_AQUILA_NAME,
        QUERA_PROVIDER_ID,
        "aquila",
        BackendKind::Qpu,
    )
    .with_api_version(
        QUERA_BRAKET_API_VERSION,
    )
    .with_region("us-east-1")
    .with_hardware_revision("Aquila")
    .with_firmware_version("provider-managed");

    metadata.insert_property(
        "technology",
        "neutral_atom",
    )?;

    metadata.insert_property(
        "execution_model",
        "analog_hamiltonian_simulation",
    )?;

    metadata.insert_property(
        "program_schema",
        QUERA_AHS_PROGRAM_FORMAT,
    )?;

    metadata.insert_property(
        "program_schema_version",
        QUERA_AHS_SCHEMA_VERSION,
    )?;

    metadata.insert_property(
        "device_arn",
        &config.device_arn,
    )?;

    metadata.insert_property(
        "maximum_atoms",
        &QUERA_AQUILA_MAX_QUBITS.to_string(),
    )?;

    metadata.insert_property(
        "minimum_atom_distance_m",
        &QUERA_AQUILA_MIN_ATOM_DISTANCE_METRES
            .to_string(),
    )?;

    metadata.insert_property(
        "maximum_shots",
        &QUERA_AQUILA_MAX_SHOTS.to_string(),
    )?;

    metadata.insert_property(
        "result_store",
        "external-artifact-store",
    )?;

    metadata.insert_property(
        "result_format",
        "braket.task_result.analog_hamiltonian_simulation_task_result",
    )?;

    metadata.insert_property(
        "experimental_local_detuning",
        if config.allow_experimental_local_detuning {
            "enabled"
        } else {
            "disabled"
        },
    )?;

    let mut capabilities =
        BackendCapabilities::new();

    capabilities.measurement = true;
    capabilities.reset = false;
    capabilities.mid_circuit_measurement = false;
    capabilities.classical_control = false;
    capabilities.dynamic_circuits = false;
    capabilities.arbitrary_single_qubit_rotations = false;
    capabilities.parameterized_gates = false;
    capabilities.three_qubit_operations = false;
    capabilities.multi_qubit_operations = false;
    capabilities.parallel_operations = true;
    capabilities.batch_execution = false;
    capabilities.streaming_results = false;
    capabilities.cancellation = true;
    capabilities.queue_information = true;
    capabilities.pulse_control = false;
    capabilities.analog_control = true;
    capabilities.annealing = false;
    capabilities.logical_qubits = false;
    capabilities.fault_tolerance = false;
    capabilities.syndrome_measurement = false;
    capabilities.decoder_execution = false;
    capabilities.deterministic_seeding = false;
    capabilities.state_vector_results = false;
    capabilities.density_matrix_results = false;
    capabilities.expectation_value_results = false;
    capabilities.readout_mitigation = false;
    capabilities.error_mitigation = false;
    capabilities.calibration_data = true;
    capabilities.timing_information = true;
    capabilities.topology_information = true;
    capabilities.native_instruction_set = false;

    if config.allow_experimental_local_detuning {
        capabilities =
            capabilities.with_experimental_capability(
                EXPERIMENTAL_LOCAL_DETUNING,
            );
    }

    let limits = BackendLimits::unlimited()
        .with_max_qubits(
            QUERA_AQUILA_MAX_QUBITS,
        )
        .with_max_shots(
            QUERA_AQUILA_MAX_SHOTS,
        );

    // AHS is not a conventional gate-connectivity graph.
    //
    // The topology still needs to expose the physical resource count to the
    // canonical backend. Pairwise interaction is determined by atom position
    // and the AHS Hamiltonian rather than by a native two-qubit gate edge.
    let topology =
        super::super::topology::HardwareTopology::new(
            QUERA_AQUILA_MAX_QUBITS,
        )?;

    QuantumBackend::new(
        metadata,
        capabilities,
        limits,
        topology,
    )
}

/// Validates a complete Braket AHS program.
fn validate_ahs_program(
    program: &BackendProgram,
    shots: usize,
    config: &QuEraAquilaConfig,
) -> Result<AhsProgramInfo, QuEraAdapterError> {
    if !config.allow_ahs_program {
        return Err(
            QuEraAdapterError::UnsupportedProgramFormat(
                program.format().to_owned(),
            ),
        );
    }

    if !matches!(
        program.format(),
        QUERA_AHS_PROGRAM_FORMAT
            | QUERA_AHS_PROGRAM_V1_FORMAT
    ) {
        return Err(
            QuEraAdapterError::UnsupportedProgramFormat(
                program.format().to_owned(),
            ),
        );
    }

    if shots == 0 {
        return Err(QuEraAdapterError::InvalidShots);
    }

    if shots > QUERA_AQUILA_MAX_SHOTS {
        return Err(
            QuEraAdapterError::ShotsExceeded {
                requested: shots,
                maximum: QUERA_AQUILA_MAX_SHOTS,
            },
        );
    }

    let root: Value =
        serde_json::from_slice(program.bytes())
            .map_err(|_| {
                QuEraAdapterError::InvalidJson
            })?;

    let object =
        root.as_object().ok_or(
            QuEraAdapterError::InvalidProgramShape,
        )?;

    let header =
        object
            .get("braketSchemaHeader")
            .and_then(Value::as_object)
            .ok_or(
                QuEraAdapterError::MissingSchemaHeader,
            )?;

    let schema_name =
        header
            .get("name")
            .and_then(Value::as_str)
            .ok_or(
                QuEraAdapterError::MissingField(
                    "braketSchemaHeader.name",
                ),
            )?;

    if schema_name != QUERA_AHS_PROGRAM_FORMAT {
        return Err(
            QuEraAdapterError::InvalidSchemaName(
                schema_name.to_owned(),
            ),
        );
    }

    let schema_version =
        header
            .get("version")
            .and_then(Value::as_str)
            .ok_or(
                QuEraAdapterError::MissingField(
                    "braketSchemaHeader.version",
                ),
            )?;

    if schema_version != QUERA_AHS_SCHEMA_VERSION {
        return Err(
            QuEraAdapterError::InvalidSchemaVersion(
                schema_version.to_owned(),
            ),
        );
    }

    let setup =
        object
            .get("setup")
            .and_then(Value::as_object)
            .ok_or(
                QuEraAdapterError::MissingField(
                    "setup",
                ),
            )?;

    let register =
        setup
            .get("ahs_register")
            .and_then(Value::as_object)
            .ok_or(
                QuEraAdapterError::MissingField(
                    "setup.ahs_register",
                ),
            )?;

    let sites =
        register
            .get("sites")
            .and_then(Value::as_array)
            .ok_or(
                QuEraAdapterError::MissingField(
                    "setup.ahs_register.sites",
                ),
            )?;

    if sites.is_empty() {
        return Err(
            QuEraAdapterError::EmptyAtomRegister,
        );
    }

    if sites.len() > MAX_AHS_REGISTER_SITES {
        return Err(
            QuEraAdapterError::AtomCountExceeded {
                requested: sites.len(),
                maximum: MAX_AHS_REGISTER_SITES,
            },
        );
    }

    let filling =
        register
            .get("filling")
            .and_then(Value::as_array);

    if let Some(filling) = filling {
        if filling.len() != sites.len() {
            return Err(
                QuEraAdapterError::FillingLengthMismatch {
                    sites: sites.len(),
                    filling: filling.len(),
                },
            );
        }

        for value in filling {
            let value = value.as_u64().ok_or(
                QuEraAdapterError::InvalidFieldType(
                    "setup.ahs_register.filling",
                ),
            )?;

            if value > 1 {
                return Err(
                    QuEraAdapterError::InvalidFieldType(
                        "setup.ahs_register.filling",
                    ),
                );
            }
        }
    }

    let mut coordinates = Vec::with_capacity(
        sites.len(),
    );

    for (index, site) in sites.iter().enumerate() {
        let pair =
            site.as_array().ok_or(
                QuEraAdapterError::InvalidCoordinate,
            )?;

        if pair.len() != 2 {
            return Err(
                QuEraAdapterError::InvalidCoordinate,
            );
        }

        let x = pair[0].as_f64().ok_or(
            QuEraAdapterError::InvalidCoordinate,
        )?;

        let y = pair[1].as_f64().ok_or(
            QuEraAdapterError::InvalidCoordinate,
        )?;

        if !x.is_finite() || !y.is_finite() {
            return Err(
                QuEraAdapterError::InvalidCoordinate,
            );
        }

        let x_um = x * 1.0e6;
        let y_um = y * 1.0e6;

        if x_um < 0.0
            || y_um < 0.0
            || x_um > QUERA_AQUILA_MAX_X_MICROMETRES
            || y_um > QUERA_AQUILA_MAX_Y_MICROMETRES
        {
            return Err(
                QuEraAdapterError::CoordinateOutsideDevice {
                    index,
                    x_micrometres: x_um,
                    y_micrometres: y_um,
                },
            );
        }

        coordinates.push((x, y));
    }

    for first in 0..coordinates.len() {
        for second in (first + 1)..coordinates.len() {
            let dx =
                coordinates[first].0
                    - coordinates[second].0;

            let dy =
                coordinates[first].1
                    - coordinates[second].1;

            let distance =
                (dx * dx + dy * dy).sqrt();

            if distance
                < QUERA_AQUILA_MIN_ATOM_DISTANCE_METRES
            {
                return Err(
                    QuEraAdapterError::AtomDistanceViolation {
                        first,
                        second,
                        distance_metres: distance,
                        minimum_metres:
                            QUERA_AQUILA_MIN_ATOM_DISTANCE_METRES,
                    },
                );
            }
        }
    }

    let uses_local_detuning =
        object
            .get("hamiltonian")
            .and_then(Value::as_object)
            .and_then(|hamiltonian| {
                hamiltonian.get("localDetuning")
            })
            .and_then(Value::as_array)
            .map(|values| !values.is_empty())
            .unwrap_or(false);

    if uses_local_detuning
        && !config.allow_experimental_local_detuning
    {
        return Err(
            QuEraAdapterError::ExperimentalCapabilityDisabled(
                EXPERIMENTAL_LOCAL_DETUNING,
            ),
        );
    }

    let hamiltonian =
        object
            .get("hamiltonian")
            .and_then(Value::as_object)
            .ok_or(
                QuEraAdapterError::MissingField(
                    "hamiltonian",
                ),
            )?;

    let driving_fields =
        hamiltonian
            .get("drivingFields")
            .and_then(Value::as_array)
            .ok_or(
                QuEraAdapterError::MissingField(
                    "hamiltonian.drivingFields",
                ),
            )?;

    if driving_fields.is_empty() {
        return Err(
            QuEraAdapterError::InvalidFieldType(
                "hamiltonian.drivingFields",
            ),
        );
    }

    Ok(AhsProgramInfo {
        site_count: sites.len(),
        fully_filled: filling
            .map(|values| {
                values
                    .iter()
                    .all(|value| value.as_u64() == Some(1))
            })
            .unwrap_or(true),
        uses_local_detuning,
        schema_version:
            schema_version.to_owned(),
    })
}

/// Normalizes a completed QuEra AHS result.
fn normalize_ahs_result(
    bytes: &[u8],
    backend_id: &str,
    shots: usize,
    task_arn: &str,
    artifact_store_id: &str,
) -> Result<ExecutionResult, QuEraAdapterError> {
    if bytes.len() > MAX_QUERA_RESULT_BYTES {
        return Err(
            QuEraAdapterError::TooManyMeasurements,
        );
    }

    let root: Value =
        serde_json::from_slice(bytes)
            .map_err(|_| {
                QuEraAdapterError::InvalidResultJson
            })?;

    let object =
        root.as_object().ok_or(
            QuEraAdapterError::UnsupportedResultShape,
        )?;

    let measurements =
        object
            .get("measurements")
            .and_then(Value::as_array)
            .ok_or(
                QuEraAdapterError::UnsupportedResultShape,
            )?;

    if measurements.len()
        > MAX_RESULT_MEASUREMENTS
    {
        return Err(
            QuEraAdapterError::TooManyMeasurements,
        );
    }

    if measurements.len() != shots {
        return Err(
            QuEraAdapterError::ResultShotMismatch {
                requested: shots,
                successful: measurements.len(),
            },
        );
    }

    let mut result =
        ExecutionResult::empty(
            backend_id.to_owned(),
            shots,
        )
        .map_err(|_| {
            QuEraAdapterError::UnsupportedResultShape
        })?;

    for measurement in measurements {
        let measurement =
            measurement.as_object().ok_or(
                QuEraAdapterError::InvalidMeasurement,
            )?;

        let shot_metadata =
            measurement
                .get("shotMetadata")
                .and_then(Value::as_object)
                .ok_or(
                    QuEraAdapterError::InvalidMeasurement,
                )?;

        let shot_status =
            shot_metadata
                .get("shotStatus")
                .and_then(Value::as_str)
                .ok_or(
                    QuEraAdapterError::InvalidMeasurement,
                )?;

        if !shot_status.eq_ignore_ascii_case("Success")
        {
            return Err(
                QuEraAdapterError::InvalidMeasurement,
            );
        }

        let post_sequence =
            measurement
                .get("post_sequence")
                .and_then(Value::as_array)
                .ok_or(
                    QuEraAdapterError::InvalidMeasurement,
                )?;

        if post_sequence.is_empty()
            || post_sequence.len()
                > MAX_RESULT_BITSTRING_LENGTH
        {
            return Err(
                QuEraAdapterError::InvalidMeasurement,
            );
        }

        let mut bitstring =
            String::with_capacity(
                post_sequence.len(),
            );

        for value in post_sequence {
            match value.as_u64() {
                Some(0) => bitstring.push('0'),
                Some(1) => bitstring.push('1'),
                _ => {
                    return Err(
                        QuEraAdapterError::InvalidMeasurement,
                    );
                }
            }
        }

        result
            .insert_count(
                bitstring,
                1,
            )
            .map_err(|_| {
                QuEraAdapterError::UnsupportedResultShape
            })?;
    }

    result.metadata.insert(
        "provider".to_owned(),
        QUERA_PROVIDER_ID.to_owned(),
    );

    result.metadata.insert(
        "backend".to_owned(),
        QUERA_AQUILA_BACKEND_ID.to_owned(),
    );

    result.metadata.insert(
        "device_arn".to_owned(),
        QUERA_AQUILA_DEVICE_ARN.to_owned(),
    );

    result.metadata.insert(
        "task_arn".to_owned(),
        task_arn.to_owned(),
    );

    result.metadata.insert(
        "result_format".to_owned(),
        "braket.task_result.analog_hamiltonian_simulation_task_result"
            .to_owned(),
    );

    result.metadata.insert(
        "artifact_store".to_owned(),
        artifact_store_id.to_owned(),
    );

    result.metadata.insert(
        "execution_model".to_owned(),
        "analog_hamiltonian_simulation".to_owned(),
    );

    result
        .validate()
        .map_err(|_| {
            QuEraAdapterError::UnsupportedResultShape
        })?;

    Ok(result)
}

/// Maps Amazon Braket task state to Zamani lifecycle state.
fn map_braket_state(
    state: &str,
) -> Result<BackendJobState, BackendError> {
    match state {
        BRAKET_CREATED => Ok(
            BackendJobState::Created,
        ),

        BRAKET_QUEUED => Ok(
            BackendJobState::Queued,
        ),

        BRAKET_RUNNING => Ok(
            BackendJobState::Running,
        ),

        BRAKET_CANCELLING => Ok(
            BackendJobState::Cancelling,
        ),

        BRAKET_CANCELLED => Ok(
            BackendJobState::Cancelled,
        ),

        BRAKET_COMPLETED => Ok(
            BackendJobState::Completed,
        ),

        BRAKET_FAILED => Ok(
            BackendJobState::Failed,
        ),

        "EXPIRED" => Ok(
            BackendJobState::Expired,
        ),

        "TIMED_OUT" => Ok(
            BackendJobState::TimedOut,
        ),

        other => Err(
            to_backend_error(
                QuEraAdapterError::UnknownTaskState(
                    other.to_owned(),
                ),
            ),
        ),
    }
}

/// Parses a provider JSON payload with bounded semantics.
fn parse_json(
    bytes: &[u8],
) -> Result<Value, BackendError> {
    serde_json::from_slice(bytes)
        .map_err(|error| {
            BackendError::ExecutionUnavailable(
                format!(
                    "invalid Amazon Braket JSON response: {}",
                    sanitize_provider_message(
                        &error.to_string(),
                    )
                ),
            )
        })
}

/// Converts an adapter-local error into the canonical hardware error.
fn to_backend_error(
    error: QuEraAdapterError,
) -> BackendError {
    BackendError::ExecutionRejected(
        sanitize_provider_message(
            &error.to_string(),
        ),
    )
}

/// Converts a Braket HTTP response into a canonical error.
///
/// Provider error bodies are intentionally bounded and sanitized.
fn map_provider_http_error(
    status: u16,
    body: &[u8],
) -> BackendError {
    let safe_body =
        if body.len()
            > MAX_PROVIDER_FAILURE_REASON_LENGTH
        {
            &body[..MAX_PROVIDER_FAILURE_REASON_LENGTH]
        } else {
            body
        };

    let message =
        serde_json::from_slice::<Value>(
            safe_body,
        )
        .ok()
        .and_then(|value| {
            value
                .get("message")
                .and_then(Value::as_str)
                .or_else(|| {
                    value
                        .get("Message")
                        .and_then(Value::as_str)
                })
        })
        .unwrap_or("Amazon Braket request failed");

    let message =
        sanitize_provider_message(message);

    match status {
        400 => BackendError::ExecutionRejected(
            message,
        ),

        401 | 403 => BackendError::ExecutionRejected(
            "Amazon Braket authentication or authorization failed"
                .to_owned(),
        ),

        402 => BackendError::ExecutionUnavailable(
            message,
        ),

        404 => BackendError::ExecutionRejected(
            "Amazon Braket resource was not found"
                .to_owned(),
        ),

        409 => BackendError::ExecutionRejected(
            message,
        ),

        410 => BackendError::BackendUnavailable {
            backend_id:
                QUERA_AQUILA_BACKEND_ID.to_owned(),
            status: BackendStatus::Retired,
        },

        424 => BackendError::BackendUnavailable {
            backend_id:
                QUERA_AQUILA_BACKEND_ID.to_owned(),
            status: BackendStatus::Offline,
        },

        429 => BackendError::ExecutionUnavailable(
            message,
        ),

        500..=599 => {
            BackendError::ExecutionUnavailable(
                message,
            )
        }

        _ => BackendError::ExecutionUnavailable(
            format!(
                "Amazon Braket returned HTTP status {status}"
            ),
        ),
    }
}

/// Builds a deterministic client token from a caller request.
///
/// A caller-provided request ID is preferred because it permits application
/// level idempotency across process restarts.
fn client_token_for_request(
    request: &ExecutionRequest,
) -> String {
    let source =
        request.request_id.as_deref().unwrap_or(
            "zamani-quera-request",
        );

    truncate_ascii_identifier(
        &format!(
            "zamani-quera-{}",
            sanitize_request_identifier(source)
        ),
        MAX_BRAKET_CLIENT_TOKEN_LENGTH,
    )
}

/// Builds a deterministic cancellation client token.
fn client_token_for_job(
    job: &BackendJobId,
) -> String {
    truncate_ascii_identifier(
        &format!(
            "zamani-quera-cancel-{}",
            sanitize_request_identifier(
                job.as_str(),
            )
        ),
        MAX_BRAKET_CLIENT_TOKEN_LENGTH,
    )
}

/// Sanitizes an identifier for use in request correlation metadata.
fn sanitize_request_identifier(
    value: &str,
) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric()
                || matches!(
                    character,
                    '-' | '_' | '.' 
                )
            {
                character
            } else {
                '-'
            }
        })
        .collect()
}

/// Truncates an ASCII-safe identifier without splitting UTF-8.
fn truncate_ascii_identifier(
    value: &str,
    maximum: usize,
) -> String {
    value
        .chars()
        .take(maximum)
        .collect()
}

/// Validates an Amazon Braket device ARN.
///
/// This deliberately validates the structure needed by this adapter rather
/// than accepting an arbitrary ARN and calling it an Aquila target.
fn validate_device_arn(
    value: &str,
) -> Result<(), BackendError> {
    if value.is_empty()
        || value.len()
            > MAX_QUERA_DEVICE_ARN_LENGTH
        || value.chars().any(char::is_control)
    {
        return Err(BackendError::InvalidIdentifier {
            field: "device_arn",
        });
    }

    if value != QUERA_AQUILA_DEVICE_ARN {
        return Err(BackendError::ExecutionRejected(
            format!(
                "QuEra adapter currently targets Aquila only; expected '{}'",
                QUERA_AQUILA_DEVICE_ARN
            ),
        ));
    }

    Ok(())
}

/// Validates a submitted task ARN.
fn validate_task_arn(
    value: &str,
) -> Result<(), QuEraAdapterError> {
    if value.is_empty()
        || value.len()
            > MAX_QUERA_TASK_ARN_LENGTH
        || value.chars().any(char::is_control)
        || !value.starts_with(
            "arn:aws:braket:",
        )
        || !value.contains(":quantum-task/")
    {
        return Err(
            QuEraAdapterError::InvalidProviderResponse(
                "invalid quantumTaskArn".to_owned(),
            ),
        );
    }

    Ok(())
}

/// Parses a Braket queue position.
///
/// Braket documents the position as a string, so non-numeric values are
/// deliberately treated as unknown rather than guessed.
fn parse_queue_position(
    position: &str,
) -> Option<usize> {
    position.parse::<usize>().ok()
}

/// Bounded optional provider status.
fn bounded_optional_string(
    value: &str,
    maximum: usize,
) -> Result<Option<String>, BackendError> {
    if value.is_empty() {
        return Ok(None);
    }

    if value.len() > maximum
        || value.chars().any(char::is_control)
    {
        return Err(BackendError::ExecutionRejected(
            "provider status is invalid".to_owned(),
        ));
    }

    Ok(Some(value.to_owned()))
}

/// Validates a generic bounded string.
fn validate_safe_string(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), BackendError> {
    if value.is_empty()
        || value.len() > maximum
        || value.chars().any(char::is_control)
    {
        return Err(BackendError::ExecutionRejected(
            format!("invalid {field}"),
        ));
    }

    Ok(())
}

/// Validates an Amazon S3 key prefix according to Braket restrictions.
fn validate_s3_key_prefix(
    prefix: &str,
) -> Result<(), BackendError> {
    if prefix.is_empty()
        || prefix.len()
            > MAX_BRAKET_S3_KEY_PREFIX_LENGTH
        || prefix.contains("../")
        || prefix.contains("./")
        || prefix.contains('{')
        || prefix.contains('}')
        || prefix.contains('[')
        || prefix.contains(']')
        || prefix.contains('<')
        || prefix.contains('>')
        || prefix.contains('\\')
        || prefix.contains('|')
        || prefix.contains('^')
        || prefix.contains('~')
        || prefix.contains('`')
        || prefix.contains('%')
        || prefix.contains('#')
        || prefix.contains('"')
        || prefix
            .chars()
            .any(|character| {
                character.is_control()
                    || !character.is_ascii()
                        && character.is_whitespace()
            })
    {
        return Err(BackendError::ExecutionRejected(
            "invalid Amazon Braket S3 output key prefix"
                .to_owned(),
        ));
    }

    Ok(())
}

/// Detects secret-like field names/values.
fn contains_secret_marker(
    value: &str,
) -> bool {
    let normalized =
        value.to_ascii_lowercase();

    [
        "api_key",
        "apikey",
        "access_token",
        "refresh_token",
        "authorization",
        "password",
        "private_key",
        "secret",
        "session_token",
        "cookie",
    ]
    .iter()
    .any(|marker| normalized.contains(marker))
}

/// Provider error sanitization.
///
/// Provider messages are diagnostic data and must never be allowed to inject
/// control characters or credential-looking fields into canonical metadata.
fn sanitize_provider_message(
    message: &str,
) -> String {
    let sanitized: String =
        message
            .chars()
            .filter(|character| {
                !character.is_control()
                    || *character == ' '
            })
            .collect();

    let sanitized =
        if sanitized.len()
            > MAX_PROVIDER_FAILURE_REASON_LENGTH
        {
            sanitized
                .chars()
                .take(
                    MAX_PROVIDER_FAILURE_REASON_LENGTH,
                )
                .collect()
        } else {
            sanitized
        };

    if contains_secret_marker(&sanitized) {
        "provider returned a redacted diagnostic"
            .to_owned()
    } else {
        sanitized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_ahs_program() -> Vec<u8> {
        serde_json::to_vec(&json!({
            "braketSchemaHeader": {
                "name": "braket.ir.ahs.program",
                "version": "1"
            },
            "setup": {
                "ahs_register": {
                    "sites": [
                        [0.0, 0.0],
                        [0.0, 0.000004],
                        [0.000004, 0.0]
                    ],
                    "filling": [1, 1, 1]
                }
            },
            "hamiltonian": {
                "drivingFields": [
                    {
                        "amplitude": {
                            "time_series": {
                                "values": [
                                    0.0,
                                    15700000.0,
                                    0.0
                                ],
                                "times": [
                                    0.0,
                                    0.000001,
                                    0.000002
                                ]
                            },
                            "pattern": "uniform"
                        },
                        "phase": {
                            "time_series": {
                                "values": [
                                    0.0,
                                    0.0,
                                    0.0
                                ],
                                "times": [
                                    0.0,
                                    0.000001,
                                    0.000002
                                ]
                            },
                            "pattern": "uniform"
                        },
                        "detuning": {
                            "time_series": {
                                "values": [
                                    0.0,
                                    0.0,
                                    0.0
                                ],
                                "times": [
                                    0.0,
                                    0.000001,
                                    0.000002
                                ]
                            },
                            "pattern": "uniform"
                        }
                    }
                ],
                "localDetuning": []
            }
        }))
        .expect("test AHS JSON must serialize")
    }

    #[test]
    fn valid_aquila_arn_is_accepted() {
        assert!(
            validate_device_arn(
                QUERA_AQUILA_DEVICE_ARN
            )
            .is_ok()
        );
    }

    #[test]
    fn non_aquila_arn_is_rejected() {
        assert!(
            validate_device_arn(
                "arn:aws:braket:us-east-1::device/qpu/quera/Other"
            )
            .is_err()
        );
    }

    #[test]
    fn valid_ahs_program_is_accepted() {
        let config =
            QuEraAquilaConfig::new(
                "example-bucket",
                "zamani/quera",
            )
            .expect("valid config");

        let bytes =
            valid_ahs_program();

        let program =
            BackendProgram::new(
                QUERA_AHS_PROGRAM_FORMAT,
                bytes,
            )
            .expect("valid program");

        let info =
            validate_ahs_program(
                &program,
                100,
                &config,
            )
            .expect("AHS program must validate");

        assert_eq!(
            info.site_count,
            3
        );
        assert!(info.fully_filled);
        assert!(
            !info.uses_local_detuning
        );
        assert_eq!(
            info.schema_version,
            "1"
        );
    }

    #[test]
    fn local_detuning_is_rejected_without_explicit_enablement() {
        let mut value: Value =
            serde_json::from_slice(
                &valid_ahs_program(),
            )
            .expect("valid JSON");

        value["hamiltonian"]["localDetuning"] =
            json!([{
                "magnitude": {
                    "time_series": {
                        "values": [0.0],
                        "times": [0.0]
                    },
                    "pattern": [1.0, 1.0, 1.0]
                }
            }]);

        let program =
            BackendProgram::new(
                QUERA_AHS_PROGRAM_FORMAT,
                serde_json::to_vec(&value)
                    .expect("serialize"),
            )
            .expect("valid program");

        let config =
            QuEraAquilaConfig::new(
                "example-bucket",
                "zamani/quera",
            )
            .expect("valid config");

        let error =
            validate_ahs_program(
                &program,
                100,
                &config,
            )
            .expect_err(
                "experimental local detuning must be disabled by default",
            );

        assert_eq!(
            error,
            QuEraAdapterError::ExperimentalCapabilityDisabled(
                EXPERIMENTAL_LOCAL_DETUNING
            )
        );
    }

    #[test]
    fn local_detuning_is_allowed_when_explicitly_enabled() {
        let mut value: Value =
            serde_json::from_slice(
                &valid_ahs_program(),
            )
            .expect("valid JSON");

        value["hamiltonian"]["localDetuning"] =
            json!([{
                "magnitude": {
                    "time_series": {
                        "values": [0.0],
                        "times": [0.0]
                    },
                    "pattern": [1.0, 1.0, 1.0]
                }
            }]);

        let program =
            BackendProgram::new(
                QUERA_AHS_PROGRAM_FORMAT,
                serde_json::to_vec(&value)
                    .expect("serialize"),
            )
            .expect("valid program");

        let config =
            QuEraAquilaConfig::new(
                "example-bucket",
                "zamani/quera",
            )
            .expect("valid config")
            .allow_experimental_local_detuning(
                true,
            );

        let info =
            validate_ahs_program(
                &program,
                100,
                &config,
            )
            .expect(
                "enabled experimental capability must validate",
            );

        assert!(
            info.uses_local_detuning
        );
    }

    #[test]
    fn too_many_atoms_are_rejected() {
        let sites: Vec<Value> =
            (0..=QUERA_AQUILA_MAX_QUBITS)
                .map(|index| {
                    json!([
                        index as f64 * 0.000004,
                        0.0
                    ])
                })
                .collect();

        let filling: Vec<Value> =
            sites.iter().map(|_| json!(1)).collect();

        let value = json!({
            "braketSchemaHeader": {
                "name": "braket.ir.ahs.program",
                "version": "1"
            },
            "setup": {
                "ahs_register": {
                    "sites": sites,
                    "filling": filling
                }
            },
            "hamiltonian": {
                "drivingFields": [{}]
            }
        });

        let program =
            BackendProgram::new(
                QUERA_AHS_PROGRAM_FORMAT,
                serde_json::to_vec(&value)
                    .expect("serialize"),
            )
            .expect("payload");

        let config =
            QuEraAquilaConfig::new(
                "example-bucket",
                "zamani/quera",
            )
            .expect("config");

        let error =
            validate_ahs_program(
                &program,
                1,
                &config,
            )
            .expect_err(
                "too many atoms must be rejected",
            );

        assert!(matches!(
            error,
            QuEraAdapterError::AtomCountExceeded {
                ..
            }
        ));
    }

    #[test]
    fn too_many_shots_are_rejected() {
        let config =
            QuEraAquilaConfig::new(
                "example-bucket",
                "zamani/quera",
            )
            .expect("config");

        let program =
            BackendProgram::new(
                QUERA_AHS_PROGRAM_FORMAT,
                valid_ahs_program(),
            )
            .expect("program");

        let error =
            validate_ahs_program(
                &program,
                QUERA_AQUILA_MAX_SHOTS + 1,
                &config,
            )
            .expect_err(
                "shot limit must be enforced",
            );

        assert!(matches!(
            error,
            QuEraAdapterError::ShotsExceeded {
                ..
            }
        ));
    }

    #[test]
    fn result_is_normalized_into_counts() {
        let result = json!({
            "taskMetadata": {
                "shots": 2
            },
            "measurements": [
                {
                    "shotMetadata": {
                        "shotStatus": "Success"
                    },
                    "pre_sequence": [1, 1, 1],
                    "post_sequence": [0, 1, 1]
                },
                {
                    "shotMetadata": {
                        "shotStatus": "Success"
                    },
                    "pre_sequence": [1, 1, 1],
                    "post_sequence": [0, 1, 1]
                }
            ]
        });

        let bytes =
            serde_json::to_vec(&result)
                .expect("serialize");

        let normalized =
            normalize_ahs_result(
                &bytes,
                QUERA_AQUILA_BACKEND_ID,
                2,
                "arn:aws:braket:us-east-1:123456789012:quantum-task/test",
                "test-store",
            )
            .expect("result must normalize");

        assert_eq!(
            normalized.counts.get("011"),
            Some(&2)
        );

        assert_eq!(
            normalized.counted_shots(),
            2
        );

        assert!(
            normalized.counts_match_shots()
        );
    }

    #[test]
    fn failed_measurement_is_not_silently_counted() {
        let result = json!({
            "measurements": [
                {
                    "shotMetadata": {
                        "shotStatus": "Failure"
                    },
                    "post_sequence": [0, 1]
                }
            ]
        });

        let bytes =
            serde_json::to_vec(&result)
                .expect("serialize");

        let error =
            normalize_ahs_result(
                &bytes,
                QUERA_AQUILA_BACKEND_ID,
                1,
                "arn:aws:braket:us-east-1:123456789012:quantum-task/test",
                "test-store",
            )
            .expect_err(
                "failed shots must not be normalized as successful",
            );

        assert_eq!(
            error,
            QuEraAdapterError::InvalidMeasurement
        );
    }

    #[test]
    fn unknown_provider_state_is_rejected() {
        assert!(
            map_braket_state(
                "FUTURE_PROVIDER_STATE"
            )
            .is_err()
        );
    }

    #[test]
    fn known_provider_states_are_mapped() {
        assert_eq!(
            map_braket_state("CREATED")
                .expect("created"),
            BackendJobState::Created
        );

        assert_eq!(
            map_braket_state("QUEUED")
                .expect("queued"),
            BackendJobState::Queued
        );

        assert_eq!(
            map_braket_state("RUNNING")
                .expect("running"),
            BackendJobState::Running
        );

        assert_eq!(
            map_braket_state("COMPLETED")
                .expect("completed"),
            BackendJobState::Completed
        );

        assert_eq!(
            map_braket_state("CANCELLED")
                .expect("cancelled"),
            BackendJobState::Cancelled
        );
    }

    #[test]
    fn secret_markers_are_detected() {
        assert!(
            contains_secret_marker(
                "access_token"
            )
        );

        assert!(
            contains_secret_marker(
                "authorization"
            )
        );

        assert!(
            !contains_secret_marker(
                "device_arn"
            )
        );
    }

    #[test]
    fn request_identifier_is_deterministic() {
        let request =
            ExecutionRequest::new(
                Default::default(),
            );

        assert_eq!(
            client_token_for_request(
                &request
            ),
            client_token_for_request(
                &request
            )
        );
    }

    #[test]
    fn s3_prefix_traversal_is_rejected() {
        assert!(
            validate_s3_key_prefix(
                "../secret"
            )
            .is_err()
        );

        assert!(
            validate_s3_key_prefix(
                "./secret"
            )
            .is_err()
        );
    }
}