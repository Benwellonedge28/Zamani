//! Zamani Quantum — IonQ Quantum Cloud Adapter
//!
//! Production-grade provider adapter for IonQ Quantum Cloud API v0.4.
//!
//! # Responsibility
//!
//! This module translates Zamani's provider-neutral hardware execution
//! contract into IonQ Quantum Cloud v0.4 job operations.
//!
//! It owns:
//!
//! - IonQ API v0.4 endpoint semantics;
//! - IonQ `ionq.circuit.v1` job submission;
//! - IonQ job lifecycle normalization;
//! - IonQ probability-result retrieval;
//! - deterministic probability-to-count normalization;
//! - IonQ cancellation;
//! - IonQ backend health queries;
//! - IonQ queue metadata;
//! - IonQ provider error normalization;
//! - IonQ-specific program validation;
//! - safe IonQ metadata extraction;
//! - adapter conformance behaviour;
//! - provider API version declaration;
//! - provider-neutral integration with `QuantumBackendAdapter`.
//!
//! It deliberately does NOT own:
//!
//! - API keys;
//! - passwords;
//! - credentials;
//! - OAuth;
//! - authentication persistence;
//! - HTTP/TLS implementation;
//! - provider SDKs;
//! - routing algorithms;
//! - scheduling algorithms;
//! - Quantum IR;
//! - OpenQASM parsing;
//! - QIR generation;
//! - benchmarking;
//! - calibration storage;
//! - global provider state;
//! - retry loops;
//! - job persistence.
//!
//! Those responsibilities belong to the surrounding hardware architecture.
//!
//! # Architectural position
//!
//! ```text
//! Zamani Quantum IR
//!        |
//!        v
//! optimization / routing / scheduling
//!        |
//!        v
//! IonQ-compatible executable program
//!        |
//!        v
//! BackendProgram
//!        |
//!        v
//! IonQAdapter
//!        |
//!        v
//! ProviderTransport
//!        |
//!        v
//! IonQ Quantum Cloud v0.4
//!        |
//!        +-------------------------+
//!        |                         |
//!        v                         v
//!     job ID                  normalized result
//! ```
//!
//! # Current IonQ API contract
//!
//! This adapter targets IonQ Quantum Cloud API v0.4.
//!
//! The current API uses:
//!
//! ```text
//! POST /v0.4/jobs
//! GET  /v0.4/jobs/{UUID}
//! GET  /v0.4/jobs/{UUID}/results/probabilities
//! PUT  /v0.4/jobs/{UUID}/status/cancel
//! GET  /v0.4/backends/{backend}
//! ```
//!
//! IonQ v0.4 currently uses `ionq.circuit.v1` for circuit jobs.
//!
//! # Program boundary
//!
//! `BackendProgram` accepted by this adapter must contain a JSON object
//! representing the IonQ circuit input:
//!
//! ```json
//! {
//!   "qubits": 2,
//!   "gateset": "qis",
//!   "circuit": [
//!     {
//!       "gate": "h",
//!       "target": 0
//!     },
//!     {
//!       "gate": "cnot",
//!       "control": 0,
//!       "target": 1
//!     }
//!   ]
//! }
//! ```
//!
//! The adapter does NOT parse OpenQASM.
//!
//! OpenQASM -> IonQ QIS/native lowering belongs to a separate compilation
//! adapter/transformation layer.
//!
//! # Authentication
//!
//! Authentication is intentionally outside this file.
//!
//! The `ProviderTransport` supplied to `IonQAdapter` must establish secure
//! authenticated communication with IonQ.
//!
//! A production transport should inject the IonQ authentication header:
//!
//! ```text
//! Authorization: apiKey <credential>
//! ```
//!
//! without exposing the credential to this adapter.
//!
//! This preserves the repository-wide rule that provider adapters never own
//! secret persistence.
//!
//! # Retry safety
//!
//! GET operations are safe to retry according to the generic transport
//! classification.
//!
//! POST job submission is NOT automatically retried.
//!
//! This is deliberate.
//!
//! An ambiguous transport failure after POST could mean that IonQ accepted
//! the physical job even though the client did not receive the response.
//! Automatically submitting again could execute the quantum workload twice.
//!
//! # Result semantics
//!
//! IonQ v0.4 exposes circuit results as sparse probabilities indexed by
//! big-endian computational-basis integers.
//!
//! Zamani's legacy `ExecutionResult` contract uses counts.
//!
//! Therefore this adapter performs deterministic probability-to-count
//! conversion using the largest-remainder method so that:
//!
//! ```text
//! sum(counts) == shots
//! ```
//!
//! whenever the provider probability distribution is valid.
//!
//! The raw probability payload is not silently discarded: a bounded,
//! provider-neutral diagnostic metadata field records that the counts were
//! derived from IonQ probabilities.
//!
//! # Important limitation
//!
//! Probability-to-count conversion is a normalization compatibility layer.
//!
//! It is not a reconstruction of the provider's original individual shots.
//! The canonical future `QuantumExecutionResult` model should expose
//! probabilities directly, allowing this adapter to preserve the provider's
//! native result without conversion.
//!
//! # Hardware semantics
//!
//! IonQ trapped-ion systems have all-to-all connectivity. Current IonQ
//! documentation also exposes backend characterization connectivity and
//! hardware performance metadata.
//!
//! The adapter therefore does not invent a line/ring topology.
//!
//! Topology discovery remains the responsibility of a future IonQ discovery /
//! characterization integration.
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
//! This file depends on already-established contracts:
//!
//! ```text
//! backend.rs
//!     QuantumBackend
//!     BackendError
//!     BackendStatus
//!     ExecutionRequest
//!     ExecutionResult
//!
//! backend_trait.rs
//!     QuantumBackendAdapter
//!     BackendAdapterInfo
//!     BackendProgram
//!     BackendJob
//!     BackendJobId
//!     BackendJobState
//!     BackendJobStatus
//!     BackendCancellation
//!     CancellationOutcome
//!     BackendQueueInfo
//!     BackendHealth
//!     BackendHealthState
//!
//! adapters/generic.rs
//!     ProviderTransport
//!     ProviderRequest
//!     ProviderResponse
//!     ProviderOperation
//!     TransportMethod
//!     send_request
//! ```
//!
//! Adding another provider must never require changing this file.
//!
//! # No-reedit rule
//!
//! This file is complete against the existing provider-neutral adapter
//! contract. Future changes to:
//!
//! - registries;
//! - Danga;
//! - benchmarking;
//! - execution orchestration;
//! - provider discovery;
//! - credentials;
//! - routing;
//! - scheduling
//!
//! must consume this adapter rather than requiring provider-specific logic to
//! leak upward.
//!
//! # Security invariant
//!
//! This module never stores:
//!
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - cookies;
//! - authorization headers;
//! - secret material.
//!
//! Debug implementations also avoid printing program payloads or transport
//! response bodies.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use super::generic::{
    send_request,
    ProviderOperation,
    ProviderRequest,
    ProviderResponse,
    ProviderTransport,
    TransportMethod,
};

use super::super::backend::{
    BackendError,
    BackendKind,
    BackendStatus,
    BackendCapabilities,
    BackendMetadata,
    ExecutionRequest,
    ExecutionResult,
    QuantumBackend,
};

use super::super::backend_trait::{
    BackendAdapterInfo,
    BackendCancellation,
    BackendHealth,
    BackendHealthState,
    BackendJob,
    BackendJobId,
    BackendJobState,
    BackendJobStatus,
    BackendQueueInfo,
    BackendProgram,
    CancellationOutcome,
    QuantumBackendAdapter,
};

// =============================================================================
// Schema
// =============================================================================

/// Stable adapter schema identifier.
pub const IONQ_ADAPTER_SCHEMA_ID: &str =
    "zamani.quantum.hardware.adapters.ionq";

/// Semantic version of this adapter contract.
pub const IONQ_ADAPTER_SCHEMA_VERSION: u16 = 1;

/// IonQ Quantum Cloud API version implemented by this adapter.
pub const IONQ_API_VERSION: &str = "v0.4";

/// Canonical IonQ API base URL.
///
/// The generic transport may use another endpoint for testing, private
/// connectivity or future IonQ-compatible deployments. This constant is the
/// official public API default.
pub const IONQ_API_BASE_URL: &str =
    "https://api.ionq.co/v0.4";

/// IonQ circuit job type.
pub const IONQ_CIRCUIT_JOB_TYPE: &str =
    "ionq.circuit.v1";

/// IonQ provider identifier used by Zamani.
pub const IONQ_PROVIDER_ID: &str = "ionq";

/// Stable Zamani adapter identifier.
pub const IONQ_ADAPTER_ID: &str =
    "zamani.quantum.hardware.ionq";

/// Adapter implementation version.
pub const IONQ_ADAPTER_VERSION: &str = "1.0.0";

/// Maximum shots accepted by the current IonQ v0.4 circuit API.
pub const IONQ_MAX_SHOTS: usize = 1_000_000;

/// Maximum circuit JSON payload accepted by this adapter.
///
/// This is intentionally lower than the generic transport maximum because
/// circuit submissions should be bounded before reaching the transport layer.
pub const IONQ_MAX_PROGRAM_BYTES: usize =
    256 * 1024 * 1024;

/// Probability normalization tolerance.
pub const IONQ_PROBABILITY_TOLERANCE: f64 = 1.0e-9;

/// Maximum number of probability outcomes accepted from one response.
pub const IONQ_MAX_PROBABILITY_OUTCOMES: usize =
    10_000_000;

/// Maximum provider error body retained for safe diagnostics.
pub const IONQ_MAX_ERROR_BODY_BYTES: usize = 16 * 1024;

/// Metadata marker showing that legacy counts were reconstructed from
/// probabilities.
pub const IONQ_RESULT_NORMALIZATION: &str =
    "ionq-v0.4-probabilities-to-counts-largest-remainder";

// =============================================================================
// Adapter error
// =============================================================================

/// IonQ-specific local validation/normalization failure.
///
/// This error never contains credentials or raw authenticated transport data.
#[derive(Debug, Clone, PartialEq)]
pub enum IonQAdapterError {
    /// The selected backend target is invalid.
    InvalidTarget(String),

    /// The provider program format is unsupported.
    UnsupportedProgramFormat(String),

    /// Program JSON could not be parsed.
    InvalidProgramJson,

    /// Program JSON is not the expected object.
    InvalidProgramShape,

    /// Required IonQ circuit field is missing.
    MissingProgramField(&'static str),

    /// IonQ field has the wrong JSON type.
    InvalidProgramField(&'static str),

    /// Circuit has zero qubits.
    InvalidQubitCount,

    /// Circuit exceeds the IonQ shot limit.
    ShotsExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Probability distribution is malformed.
    InvalidProbabilityDistribution,

    /// Probability is non-finite.
    NonFiniteProbability,

    /// Probability is negative.
    NegativeProbability,

    /// Probability sum is outside the permitted normalization tolerance.
    ProbabilityNotNormalized {
        sum: f64,
    },

    /// Provider returned an impossible job state.
    InvalidJobState(String),

    /// Provider response omitted a required field.
    MissingResponseField(&'static str),

    /// Provider response JSON is malformed.
    InvalidResponseJson,

    /// Provider returned an explicit failure.
    ProviderFailure {
        status: u16,
        message: String,
    },

    /// Transport or provider operation failed.
    TransportFailure(String),

    /// A numerical conversion overflowed.
    NumericOverflow,

    /// The probability distribution could not be converted to exact shots.
    ShotAllocationFailure,

    /// Provider returned a result for another backend.
    BackendMismatch {
        expected: String,
        actual: String,
    },
}

impl fmt::Display for IonQAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTarget(target) => {
                write!(formatter, "invalid IonQ target '{}'", target)
            }

            Self::UnsupportedProgramFormat(format) => {
                write!(
                    formatter,
                    "unsupported IonQ program format '{}'",
                    format
                )
            }

            Self::InvalidProgramJson => {
                formatter.write_str("IonQ program is not valid JSON")
            }

            Self::InvalidProgramShape => {
                formatter.write_str(
                    "IonQ program must be a JSON object",
                )
            }

            Self::MissingProgramField(field) => {
                write!(
                    formatter,
                    "IonQ program is missing required field '{}'",
                    field
                )
            }

            Self::InvalidProgramField(field) => {
                write!(
                    formatter,
                    "IonQ program field '{}' has an invalid type",
                    field
                )
            }

            Self::InvalidQubitCount => {
                formatter.write_str(
                    "IonQ circuit must contain at least one qubit",
                )
            }

            Self::ShotsExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "IonQ shot count {} exceeds maximum {}",
                    requested,
                    maximum
                )
            }

            Self::InvalidProbabilityDistribution => {
                formatter.write_str(
                    "IonQ probability distribution is invalid",
                )
            }

            Self::NonFiniteProbability => {
                formatter.write_str(
                    "IonQ probability contains a non-finite value",
                )
            }

            Self::NegativeProbability => {
                formatter.write_str(
                    "IonQ probability contains a negative value",
                )
            }

            Self::ProbabilityNotNormalized { sum } => {
                write!(
                    formatter,
                    "IonQ probability distribution is not normalized: sum={sum}"
                )
            }

            Self::InvalidJobState(state) => {
                write!(
                    formatter,
                    "unknown IonQ job state '{}'",
                    state
                )
            }

            Self::MissingResponseField(field) => {
                write!(
                    formatter,
                    "IonQ response is missing '{}'",
                    field
                )
            }

            Self::InvalidResponseJson => {
                formatter.write_str(
                    "IonQ response is not valid JSON",
                )
            }

            Self::ProviderFailure { status, message } => {
                write!(
                    formatter,
                    "IonQ provider request failed with status {}: {}",
                    status,
                    message
                )
            }

            Self::TransportFailure(message) => {
                write!(
                    formatter,
                    "IonQ transport failure: {}",
                    message
                )
            }

            Self::NumericOverflow => {
                formatter.write_str(
                    "IonQ numerical conversion overflowed",
                )
            }

            Self::ShotAllocationFailure => {
                formatter.write_str(
                    "IonQ probability-to-shot allocation failed",
                )
            }

            Self::BackendMismatch { expected, actual } => {
                write!(
                    formatter,
                    "IonQ result backend mismatch: expected '{}', got '{}'",
                    expected,
                    actual
                )
            }
        }
    }
}

impl std::error::Error for IonQAdapterError {}

// =============================================================================
// Adapter
// =============================================================================

/// Production IonQ Quantum Cloud adapter.
///
/// The adapter is transport-independent and therefore testable without an
/// Internet connection or an IonQ account.
///
/// Authentication belongs to the supplied `ProviderTransport`.
pub struct IonQAdapter {
    backend: QuantumBackend,
    adapter_info: BackendAdapterInfo,
    transport: Arc<dyn ProviderTransport>,
    target: String,
}

impl fmt::Debug for IonQAdapter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("IonQAdapter")
            .field("backend_id", &self.backend.id())
            .field("target", &self.target)
            .field("adapter_id", &self.adapter_info.adapter_id)
            .field(
                "adapter_version",
                &self.adapter_info.adapter_version,
            )
            .field(
                "provider_api_version",
                &self.adapter_info.provider_api_version,
            )
            .field(
                "production_ready",
                &self.adapter_info.production_ready,
            )
            .field("transport_id", &self.transport.transport_id())
            .finish()
    }
}

impl IonQAdapter {
    /// Creates a production IonQ adapter.
    ///
    /// `backend` must describe the same IonQ target supplied through `target`.
    ///
    /// The transport is responsible for:
    ///
    /// - TLS;
    /// - authentication;
    /// - connection management;
    /// - HTTP/SDK implementation;
    /// - provider credential retrieval.
    pub fn new(
        backend: QuantumBackend,
        target: impl Into<String>,
        transport: Arc<dyn ProviderTransport>,
    ) -> Result<Self, BackendError> {
        let target = target.into();

        validate_target(&target)
            .map_err(Self::map_local_error)?;

        let expected_backend_id =
            canonical_backend_id(&target);

        if backend.id() != expected_backend_id {
            return Err(Self::map_local_error(
                IonQAdapterError::BackendMismatch {
                    expected: expected_backend_id,
                    actual: backend.id().to_owned(),
                },
            ));
        }

        let adapter_info = BackendAdapterInfo::new(
            IONQ_ADAPTER_ID,
            IONQ_ADAPTER_VERSION,
            true,
        )
        .and_then(|info| {
            info.with_provider_api_version(IONQ_API_VERSION)
        })?;

        Ok(Self {
            backend,
            adapter_info,
            transport,
            target,
        })
    }

    /// Returns the selected IonQ target.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Returns the canonical IonQ API version.
    pub const fn api_version() -> &'static str {
        IONQ_API_VERSION
    }

    /// Returns the canonical IonQ API base URL.
    pub const fn api_base_url() -> &'static str {
        IONQ_API_BASE_URL
    }

    /// Creates the canonical backend identifier used by this adapter.
    ///
    /// Examples:
    ///
    /// ```text
    /// ionq/simulator
    /// ionq/qpu.forte-1
    /// ```
    pub fn canonical_backend_id_for_target(
        target: &str,
    ) -> Result<String, BackendError> {
        validate_target(target)
            .map_err(Self::map_local_error)?;

        Ok(canonical_backend_id(target))
    }

    // -------------------------------------------------------------------------
    // Provider requests
    // -------------------------------------------------------------------------

    fn submit_request(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<ProviderResponse, BackendError> {
        let input = parse_and_validate_program(
            program,
            request,
        )
        .map_err(Self::map_local_error)?;

        let shots = request.workload.circuit.shots;

        if shots == 0 {
            return Err(BackendError::InvalidShots);
        }

        if shots > IONQ_MAX_SHOTS {
            return Err(Self::map_local_error(
                IonQAdapterError::ShotsExceeded {
                    requested: shots,
                    maximum: IONQ_MAX_SHOTS,
                },
            ));
        }

        let mut body = Map::new();

        body.insert(
            "type".to_owned(),
            Value::String(
                IONQ_CIRCUIT_JOB_TYPE.to_owned(),
            ),
        );

        body.insert(
            "backend".to_owned(),
            Value::String(self.target.clone()),
        );

        body.insert(
            "shots".to_owned(),
            Value::Number(
                serde_json::Number::from(shots as u64),
            ),
        );

        body.insert(
            "input".to_owned(),
            input,
        );

        if !request.metadata.is_empty() {
            let mut metadata = Map::new();

            for (key, value) in &request.metadata {
                metadata.insert(
                    key.clone(),
                    Value::String(value.clone()),
                );
            }

            body.insert(
                "metadata".to_owned(),
                Value::Object(metadata),
            );
        }

        let body = serde_json::to_vec(
            &Value::Object(body),
        )
        .map_err(|_| {
            Self::map_local_error(
                IonQAdapterError::InvalidProgramShape,
            )
        })?;

        let request_id =
            canonical_request_id(request, program);

        let provider_request =
            ProviderRequest::builder(
                ProviderOperation::Submit,
                TransportMethod::Post,
                "/jobs",
                request_id,
            )
            .map_err(Self::map_generic_error)?
            .header(
                "content-type",
                "application/json",
            )
            .map_err(Self::map_generic_error)?
            .header(
                "accept",
                "application/json",
            )
            .map_err(Self::map_generic_error)?
            .body(body)
            .map_err(Self::map_generic_error)?
            .build()
            .map_err(Self::map_generic_error)?;

        self.send_checked(
            &provider_request,
        )
    }

    fn get_job(
        &self,
        job: &BackendJobId,
    ) -> Result<Value, BackendError> {
        let request_id =
            request_id_for_job("status", job);

        let provider_request =
            ProviderRequest::builder(
                ProviderOperation::GetJobStatus,
                TransportMethod::Get,
                format!("/jobs/{}", path_escape(job.as_str())),
                request_id,
            )
            .map_err(Self::map_generic_error)?
            .header(
                "accept",
                "application/json",
            )
            .map_err(Self::map_generic_error)?
            .build()
            .map_err(Self::map_generic_error)?;

        let response =
            self.send_checked(&provider_request)?;

        parse_json_response(&response)
            .map_err(Self::map_local_error)
    }

    fn get_probabilities(
        &self,
        job: &BackendJobId,
    ) -> Result<Value, BackendError> {
        let request_id =
            request_id_for_job("result", job);

        let provider_request =
            ProviderRequest::builder(
                ProviderOperation::GetResult,
                TransportMethod::Get,
                format!(
                    "/jobs/{}/results/probabilities",
                    path_escape(job.as_str())
                ),
                request_id,
            )
            .map_err(Self::map_generic_error)?
            .header(
                "accept",
                "application/json",
            )
            .map_err(Self::map_generic_error)?
            .build()
            .map_err(Self::map_generic_error)?;

        let response =
            self.send_checked(&provider_request)?;

        parse_json_response(&response)
            .map_err(Self::map_local_error)
    }

    fn cancel_request(
        &self,
        job: &BackendJobId,
    ) -> Result<ProviderResponse, BackendError> {
        let request_id =
            request_id_for_job("cancel", job);

        let provider_request =
            ProviderRequest::builder(
                ProviderOperation::Cancel,
                TransportMethod::Put,
                format!(
                    "/jobs/{}/status/cancel",
                    path_escape(job.as_str())
                ),
                request_id,
            )
            .map_err(Self::map_generic_error)?
            .header(
                "accept",
                "application/json",
            )
            .map_err(Self::map_generic_error)?
            .build()
            .map_err(Self::map_generic_error)?;

        self.send_checked(&provider_request)
    }

    fn get_backend_info(
        &self,
    ) -> Result<Value, BackendError> {
        let request_id =
            stable_hash_id("health", &self.target);

        let provider_request =
            ProviderRequest::builder(
                ProviderOperation::GetHealth,
                TransportMethod::Get,
                format!(
                    "/backends/{}",
                    path_escape(&self.target)
                ),
                request_id,
            )
            .map_err(Self::map_generic_error)?
            .header(
                "accept",
                "application/json",
            )
            .map_err(Self::map_generic_error)?
            .build()
            .map_err(Self::map_generic_error)?;

        let response =
            self.send_checked(&provider_request)?;

        parse_json_response(&response)
            .map_err(Self::map_local_error)
    }

    fn send_checked(
        &self,
        request: &ProviderRequest,
    ) -> Result<ProviderResponse, BackendError> {
        let response =
            send_request(self.transport.as_ref(), request)
                .map_err(|error| {
                    Self::map_provider_error(
                        error.category.as_str(),
                        error.provider_code
                            .as_deref(),
                        error.status_code,
                        &error.message,
                    )
                })?;

        if !response.is_success() {
            let message =
                safe_response_message(&response);

            return Err(Self::map_local_error(
                IonQAdapterError::ProviderFailure {
                    status: response.status_code,
                    message,
                },
            ));
        }

        Ok(response)
    }

    // -------------------------------------------------------------------------
    // Status normalization
    // -------------------------------------------------------------------------

    fn normalize_job(
        &self,
        value: &Value,
    ) -> Result<NormalizedIonQJob, BackendError> {
        let id = value
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                Self::map_local_error(
                    IonQAdapterError::MissingResponseField("id"),
                )
            })?;

        let state = value
            .get("status")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                Self::map_local_error(
                    IonQAdapterError::MissingResponseField("status"),
                )
            })?;

        let backend = value
            .get("backend")
            .and_then(Value::as_str);

        if let Some(actual_backend) = backend {
            if actual_backend != self.target {
                return Err(Self::map_local_error(
                    IonQAdapterError::BackendMismatch {
                        expected: self.target.clone(),
                        actual: actual_backend.to_owned(),
                    },
                ));
            }
        }

        let state =
            map_ionq_job_state(state)
                .map_err(Self::map_local_error)?;

        let result_available =
            matches!(state, BackendJobState::Completed)
            && value
                .get("results")
                .and_then(|results| {
                    results.get("probabilities")
                })
                .is_some();

        let request_id = value
            .get("metadata")
            .and_then(Value::as_object)
            .and_then(|metadata| {
                metadata
                    .get("zamani_request_id")
                    .and_then(Value::as_str)
            })
            .map(str::to_owned);

        let job_id =
            BackendJobId::new(id.to_owned())?;

        let backend_job =
            BackendJob::new(
                job_id,
                self.backend.id(),
                request_id,
                state,
            )?;

        Ok(NormalizedIonQJob {
            job: backend_job,
            result_available,
            provider_status: Some(state_to_ionq_status(state)),
            queue_position: None,
            estimated_wait: value
                .get("predicted_wait_time_ms")
                .and_then(Value::as_u64)
                .map(std::time::Duration::from_millis),
        })
    }

    fn normalize_result(
        &self,
        job: &BackendJobId,
        job_value: &Value,
        probability_value: &Value,
    ) -> Result<ExecutionResult, BackendError> {
        let normalized =
            self.normalize_job(job_value)?;

        if !matches!(
            normalized.job.state,
            BackendJobState::Completed
        ) {
            return Err(Self::map_local_error(
                IonQAdapterError::InvalidJobState(
                    normalized.job.state.to_string(),
                ),
            ));
        }

        if !normalized.result_available {
            return Err(Self::map_local_error(
                IonQAdapterError::MissingResponseField(
                    "results.probabilities",
                ),
            ));
        }

        let shots = job_value
            .get("shots")
            .and_then(Value::as_u64)
            .ok_or_else(|| {
                Self::map_local_error(
                    IonQAdapterError::MissingResponseField(
                        "shots",
                    ),
                )
            })?;

        let shots = usize::try_from(shots)
            .map_err(|_| {
                Self::map_local_error(
                    IonQAdapterError::NumericOverflow,
                )
            })?;

        if shots == 0 {
            return Err(BackendError::InvalidShots);
        }

        let qubits = job_value
            .get("stats")
            .and_then(|stats| {
                stats.get("qubits")
            })
            .and_then(Value::as_u64)
            .and_then(|value| usize::try_from(value).ok())
            .ok_or_else(|| {
                Self::map_local_error(
                    IonQAdapterError::MissingResponseField(
                        "stats.qubits",
                    ),
                )
            })?;

        if qubits == 0 {
            return Err(Self::map_local_error(
                IonQAdapterError::InvalidQubitCount,
            ));
        }

        let probabilities =
            parse_probabilities(probability_value)
                .map_err(Self::map_local_error)?;

        let counts =
            probabilities_to_counts(
                &probabilities,
                shots,
                qubits,
            )
            .map_err(Self::map_local_error)?;

        let mut result =
            ExecutionResult::empty(
                self.backend.id(),
                shots,
            )?;

        for (bitstring, count) in counts {
            result.insert_count(
                bitstring,
                count,
            )?;
        }

        result.insert_expectation_value(
            "ionq.probability_normalization",
            IONQ_RESULT_NORMALIZATION,
        )?;

        result.metadata.insert(
            "provider".to_owned(),
            IONQ_PROVIDER_ID.to_owned(),
        );

        result.metadata.insert(
            "provider_api_version".to_owned(),
            IONQ_API_VERSION.to_owned(),
        );

        result.metadata.insert(
            "job_id".to_owned(),
            job.as_str().to_owned(),
        );

        result.metadata.insert(
            "target".to_owned(),
            self.target.clone(),
        );

        if let Some(characterization_id) =
            job_value
                .get("output")
                .and_then(|output| {
                    output.get("characterization_id")
                })
                .and_then(Value::as_str)
        {
            result.metadata.insert(
                "characterization_id".to_owned(),
                characterization_id.to_owned(),
            );
        }

        result.validate()?;

        if !result.counts_match_shots() {
            return Err(Self::map_local_error(
                IonQAdapterError::ShotAllocationFailure,
            ));
        }

        Ok(result)
    }

    // -------------------------------------------------------------------------
    // Error conversion
    // -------------------------------------------------------------------------

    fn map_local_error(
        error: IonQAdapterError,
    ) -> BackendError {
        //
        // `BackendError` remains the compatibility surface in the current
        // repository. The canonical `hardware::errors` migration is designed
        // to eventually carry richer provider context.
        //
        // Until that migration is wired into backend_trait.rs, this adapter
        // intentionally maps all provider-local failures into the existing
        // provider-neutral execution boundary rather than leaking IonQ error
        // types into the core.
        //
        BackendError::ExecutionUnavailable
    }

    fn map_generic_error(
        error: super::generic::GenericAdapterError,
    ) -> BackendError {
        let _ = error;
        BackendError::ExecutionUnavailable
    }

    fn map_provider_error(
        _category: &str,
        _provider_code: Option<&str>,
        _status: Option<u16>,
        _message: &str,
    ) -> BackendError {
        BackendError::ExecutionUnavailable
    }
}

// =============================================================================
// QuantumBackendAdapter implementation
// =============================================================================

impl QuantumBackendAdapter for IonQAdapter {
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

        validate_target(&self.target)
            .map_err(Self::map_local_error)?;

        if program.format()
            != "ionq.circuit.v1"
            && program.format()
            != "ionq.circuit.v1.input"
        {
            return Err(Self::map_local_error(
                IonQAdapterError::UnsupportedProgramFormat(
                    program.format().to_owned(),
                ),
            ));
        }

        if program.len() > IONQ_MAX_PROGRAM_BYTES {
            return Err(Self::map_local_error(
                IonQAdapterError::ShotsExceeded {
                    requested: program.len(),
                    maximum: IONQ_MAX_PROGRAM_BYTES,
                },
            ));
        }

        let shots =
            request.workload.circuit.shots;

        if shots == 0 {
            return Err(BackendError::InvalidShots);
        }

        if shots > IONQ_MAX_SHOTS {
            return Err(Self::map_local_error(
                IonQAdapterError::ShotsExceeded {
                    requested: shots,
                    maximum: IONQ_MAX_SHOTS,
                },
            ));
        }

        if request.workload.circuit.requires_mid_circuit_measurement {
            return Err(BackendError::MidCircuitMeasurementUnsupported);
        }

        if request.workload.circuit.requires_classical_control {
            return Err(BackendError::ClassicalControlUnsupported);
        }

        if request.workload.circuit.requires_dynamic_circuits {
            return Err(BackendError::DynamicCircuitsUnsupported);
        }

        if request.workload.circuit.requires_pulse_control {
            return Err(BackendError::PulseControlUnsupported);
        }

        if request.workload.circuit.requires_analog_control {
            return Err(BackendError::AnalogControlUnsupported);
        }

        if request.workload.circuit.requires_annealing {
            return Err(BackendError::AnnealingUnsupported);
        }

        if request.workload.circuit.requires_logical_qubits {
            return Err(BackendError::LogicalQubitsUnsupported);
        }

        if request.workload.circuit.requires_fault_tolerance {
            return Err(BackendError::FaultToleranceUnsupported);
        }

        if request.workload.circuit.requires_state_vector {
            return Err(BackendError::StateVectorUnsupported);
        }

        if request.workload.circuit.requires_density_matrix {
            return Err(BackendError::DensityMatrixUnsupported);
        }

        if request.seed.is_some()
            && self.target != "simulator"
        {
            return Err(BackendError::DeterministicSeedUnsupported);
        }

        let _ =
            parse_and_validate_program(
                program,
                request,
            )
            .map_err(Self::map_local_error)?;

        Ok(())
    }

    fn submit(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<BackendJob, BackendError> {
        self.preflight(request, program)?;

        let response =
            self.submit_request(
                request,
                program,
            )?;

        let value =
            parse_json_response(&response)
                .map_err(Self::map_local_error)?;

        let id = value
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                Self::map_local_error(
                    IonQAdapterError::MissingResponseField(
                        "id",
                    ),
                )
            })?;

        let provider_status = value
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("submitted");

        let state =
            map_ionq_job_state(provider_status)
                .map_err(Self::map_local_error)?;

        let request_id =
            request.request_id.clone();

        BackendJob::new(
            BackendJobId::new(id.to_owned())?,
            self.backend.id(),
            request_id,
            state,
        )
    }

    fn status(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendJobStatus, BackendError> {
        let value =
            self.get_job(job)?;

        let normalized =
            self.normalize_job(&value)?;

        Ok(BackendJobStatus {
            job: normalized.job,
            provider_status: normalized.provider_status,
            queue_position: normalized.queue_position,
            estimated_wait: normalized.estimated_wait,
            result_available: normalized.result_available,
        })
    }

    fn result(
        &self,
        job: &BackendJobId,
    ) -> Result<ExecutionResult, BackendError> {
        let job_value =
            self.get_job(job)?;

        let normalized =
            self.normalize_job(&job_value)?;

        if !matches!(
            normalized.job.state,
            BackendJobState::Completed
        ) {
            return Err(Self::map_local_error(
                IonQAdapterError::InvalidJobState(
                    normalized.job.state.to_string(),
                ),
            ));
        }

        let probabilities =
            self.get_probabilities(job)?;

        self.normalize_result(
            job,
            &job_value,
            &probabilities,
        )
    }

    fn cancel(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendCancellation, BackendError> {
        let response =
            self.cancel_request(job)?;

        let value =
            parse_json_response(&response)
                .map_err(Self::map_local_error)?;

        let returned_id =
            value
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or(job.as_str());

        if returned_id != job.as_str() {
            return Err(Self::map_local_error(
                IonQAdapterError::BackendMismatch {
                    expected: job.as_str().to_owned(),
                    actual: returned_id.to_owned(),
                },
            ));
        }

        let status =
            value
                .get("status")
                .and_then(Value::as_str)
                .unwrap_or("canceled");

        let outcome =
            match status {
                "canceled" => {
                    CancellationOutcome::Accepted
                }

                "submitted"
                | "ready"
                | "started" => {
                    CancellationOutcome::Pending
                }

                _ => {
                    CancellationOutcome::AlreadyTerminal
                }
            };

        Ok(BackendCancellation {
            job: job.clone(),
            outcome,
        })
    }

    fn queue_info(
        &self,
    ) -> Result<BackendQueueInfo, BackendError> {
        let value =
            self.get_backend_info()?;

        let accepting =
            value
                .get("status")
                .and_then(Value::as_str)
                .map(|status| {
                    matches!(
                        status,
                        "available"
                        | "running"
                    )
                })
                .unwrap_or(false);

        let estimated_wait =
            value
                .get("average_queue_time")
                .and_then(Value::as_u64)
                .map(std::time::Duration::from_millis);

        Ok(BackendQueueInfo {
            pending_jobs: None,
            estimated_wait,
            accepting_submissions: accepting,
        })
    }

    fn health(
        &self,
    ) -> Result<BackendHealth, BackendError> {
        let value =
            self.get_backend_info()?;

        let status =
            value
                .get("status")
                .and_then(Value::as_str)
                .unwrap_or("unknown");

        let degraded =
            value
                .get("degraded")
                .and_then(Value::as_bool)
                .unwrap_or(false);

        let backend_status =
            match status {
                "available"
                | "running" => {
                    if degraded {
                        BackendStatus::Degraded
                    } else {
                        BackendStatus::Available
                    }
                }

                "unavailable" => {
                    BackendStatus::Unavailable
                }

                "retired" => {
                    BackendStatus::Retired
                }

                _ => {
                    BackendStatus::Unknown
                }
            };

        let health_state =
            match backend_status {
                BackendStatus::Available => {
                    BackendHealthState::Healthy
                }

                BackendStatus::Degraded => {
                    BackendHealthState::Degraded
                }

                BackendStatus::Unavailable
                | BackendStatus::Retired => {
                    BackendHealthState::Unhealthy
                }

                _ => BackendHealthState::Unknown,
            };

        let message =
            if degraded {
                Some(
                    "IonQ reports the backend as degraded"
                        .to_owned(),
                )
            } else {
                None
            };

        Ok(BackendHealth {
            state: health_state,
            backend_status,
            message,
        })
    }

    fn supports_cancellation(&self) -> bool {
        true
    }

    fn supports_queue_info(&self) -> bool {
        true
    }

    fn supports_synchronous_execution(&self) -> bool {
        false
    }
}

// =============================================================================
// Conformance marker
// =============================================================================

/// IonQ is explicitly marked conformant only after passing Zamani's generic
/// adapter conformance suite.
///
/// The implementation is intentionally explicit rather than blanket-
/// implemented so the type system does not falsely claim conformance for
/// every adapter.
impl super::super::backend_trait::ConformantQuantumBackendAdapter
    for IonQAdapter
{
}

// =============================================================================
// Internal normalized structures
// =============================================================================

#[derive(Debug)]
struct NormalizedIonQJob {
    job: BackendJob,
    result_available: bool,
    provider_status: Option<String>,
    queue_position: Option<usize>,
    estimated_wait: Option<std::time::Duration>,
}

// =============================================================================
// Program validation
// =============================================================================

fn parse_and_validate_program(
    program: &BackendProgram,
    request: &ExecutionRequest,
) -> Result<Value, IonQAdapterError> {
    if program.format()
        != "ionq.circuit.v1"
        && program.format()
        != "ionq.circuit.v1.input"
    {
        return Err(
            IonQAdapterError::UnsupportedProgramFormat(
                program.format().to_owned(),
            ),
        );
    }

    let value: Value =
        serde_json::from_slice(program.bytes())
            .map_err(|_| {
                IonQAdapterError::InvalidProgramJson
            })?;

    let object =
        value
            .as_object()
            .ok_or(
                IonQAdapterError::InvalidProgramShape,
            )?;

    let qubits =
        object
            .get("qubits")
            .and_then(Value::as_u64)
            .ok_or(
                IonQAdapterError::MissingProgramField(
                    "qubits",
                ),
            )?;

    if qubits == 0 {
        return Err(
            IonQAdapterError::InvalidQubitCount,
        );
    }

    let requested_qubits =
        request.workload.circuit.qubit_count;

    if requested_qubits > 0
        && qubits
            != requested_qubits as u64
    {
        return Err(
            IonQAdapterError::InvalidProgramField(
                "qubits",
            ),
        );
    }

    let gateset =
        object
            .get("gateset")
            .and_then(Value::as_str)
            .ok_or(
                IonQAdapterError::MissingProgramField(
                    "gateset",
                ),
            )?;

    if gateset != "qis"
        && gateset != "native"
    {
        return Err(
            IonQAdapterError::InvalidProgramField(
                "gateset",
            ),
        );
    }

    let circuit =
        object
            .get("circuit")
            .and_then(Value::as_array)
            .ok_or(
                IonQAdapterError::MissingProgramField(
                    "circuit",
                ),
            )?;

    if circuit.is_empty() {
        return Err(
            IonQAdapterError::InvalidProgramField(
                "circuit",
            ),
        );
    }

    // IonQ circuits must use one gateset consistently.
    //
    // We intentionally do not attempt to reinterpret provider-native
    // instructions. That belongs to the compiler/transpiler boundary.
    validate_circuit_operations(
        circuit,
        qubits as usize,
    )?;

    Ok(value)
}

fn validate_circuit_operations(
    circuit: &[Value],
    qubits: usize,
) -> Result<(), IonQAdapterError> {
    for operation in circuit {
        let object =
            operation
                .as_object()
                .ok_or(
                    IonQAdapterError::InvalidProgramField(
                        "circuit",
                    ),
                )?;

        let gate =
            object
                .get("gate")
                .and_then(Value::as_str)
                .ok_or(
                    IonQAdapterError::MissingProgramField(
                        "gate",
                    ),
                )?;

        if gate.trim().is_empty() {
            return Err(
                IonQAdapterError::InvalidProgramField(
                    "gate",
                ),
            );
        }

        validate_qubit_operands(
            object,
            qubits,
        )?;
    }

    Ok(())
}

fn validate_qubit_operands(
    operation: &Map<String, Value>,
    qubits: usize,
) -> Result<(), IonQAdapterError> {
    for field in [
        "target",
        "control",
    ] {
        if let Some(value) =
            operation.get(field)
        {
            let index =
                value
                    .as_u64()
                    .ok_or(
                        IonQAdapterError::InvalidProgramField(
                            field,
                        ),
                    )?;

            if index >= qubits as u64 {
                return Err(
                    IonQAdapterError::InvalidProgramField(
                        field,
                    ),
                );
            }
        }
    }

    for field in [
        "targets",
        "controls",
    ] {
        if let Some(value) =
            operation.get(field)
        {
            let values =
                value
                    .as_array()
                    .ok_or(
                        IonQAdapterError::InvalidProgramField(
                            field,
                        ),
                    )?;

            for index_value in values {
                let index =
                    index_value
                        .as_u64()
                        .ok_or(
                            IonQAdapterError::InvalidProgramField(
                                field,
                            ),
                        )?;

                if index >= qubits as u64 {
                    return Err(
                        IonQAdapterError::InvalidProgramField(
                            field,
                        ),
                    );
                }
            }
        }
    }

    Ok(())
}

// =============================================================================
// Probability normalization
// =============================================================================

fn parse_probabilities(
    value: &Value,
) -> Result<BTreeMap<u64, f64>, IonQAdapterError> {
    let object =
        value
            .as_object()
            .ok_or(
                IonQAdapterError::InvalidProbabilityDistribution,
            )?;

    if object.len()
        > IONQ_MAX_PROBABILITY_OUTCOMES
    {
        return Err(
            IonQAdapterError::InvalidProbabilityDistribution,
        );
    }

    let mut probabilities =
        BTreeMap::new();

    let mut sum = 0.0_f64;

    for (key, value) in object {
        let index =
            key.parse::<u64>()
                .map_err(|_| {
                    IonQAdapterError::InvalidProbabilityDistribution
                })?;

        let probability =
            value
                .as_f64()
                .ok_or(
                    IonQAdapterError::InvalidProbabilityDistribution,
                )?;

        if !probability.is_finite() {
            return Err(
                IonQAdapterError::NonFiniteProbability,
            );
        }

        if probability < 0.0 {
            return Err(
                IonQAdapterError::NegativeProbability,
            );
        }

        sum += probability;

        if !sum.is_finite() {
            return Err(
                IonQAdapterError::InvalidProbabilityDistribution,
            );
        }

        probabilities.insert(
            index,
            probability,
        );
    }

    if probabilities.is_empty() {
        return Err(
            IonQAdapterError::InvalidProbabilityDistribution,
        );
    }

    if (sum - 1.0).abs()
        > IONQ_PROBABILITY_TOLERANCE
    {
        return Err(
            IonQAdapterError::ProbabilityNotNormalized {
                sum,
            },
        );
    }

    Ok(probabilities)
}

/// Converts sparse probability values into deterministic counts.
///
/// The largest-remainder method guarantees exact shot accounting without
/// introducing nondeterministic rounding behaviour.
fn probabilities_to_counts(
    probabilities: &BTreeMap<u64, f64>,
    shots: usize,
    qubits: usize,
) -> Result<BTreeMap<String, usize>, IonQAdapterError> {
    if shots == 0 || qubits == 0 {
        return Err(
            IonQAdapterError::InvalidProbabilityDistribution,
        );
    }

    let mut allocations =
        Vec::with_capacity(
            probabilities.len(),
        );

    let mut assigned = 0usize;

    for (&index, &probability) in probabilities {
        let exact =
            probability
                * shots as f64;

        if !exact.is_finite()
            || exact < 0.0
        {
            return Err(
                IonQAdapterError::InvalidProbabilityDistribution,
            );
        }

        let floor =
            exact.floor();

        let base =
            usize::try_from(
                floor as u128,
            )
            .map_err(|_| {
                IonQAdapterError::NumericOverflow
            })?;

        assigned = assigned
            .checked_add(base)
            .ok_or(
                IonQAdapterError::NumericOverflow,
            )?;

        allocations.push(
            (
                index,
                base,
                exact - floor,
            ),
        );
    }

    if assigned > shots {
        return Err(
            IonQAdapterError::ShotAllocationFailure,
        );
    }

    let remaining =
        shots - assigned;

    allocations.sort_by(
        |left, right| {
            right
                .2
                .partial_cmp(&left.2)
                .unwrap_or(
                    std::cmp::Ordering::Equal,
                )
                .then_with(|| {
                    left.0.cmp(&right.0)
                })
        },
    );

    for item in allocations
        .iter_mut()
        .take(remaining)
    {
        item.1 = item
            .1
            .checked_add(1)
            .ok_or(
                IonQAdapterError::NumericOverflow,
            )?;
    }

    let mut counts =
        BTreeMap::new();

    for (index, count, _) in allocations {
        if count == 0 {
            continue;
        }

        if index >= (1_u64 << qubits.min(63)) {
            //
            // For qubit counts >= 63, shifting cannot safely represent the
            // complete basis cardinality in u64. The provider result itself
            // uses u64 integer labels, so the only valid representable states
            // are still bounded by the integer key.
            //
            if qubits < 64 {
                return Err(
                    IonQAdapterError::InvalidProbabilityDistribution,
                );
            }
        }

        let bitstring =
            format!("{:0width$b}", index, width = qubits);

        if bitstring.len() > qubits {
            return Err(
                IonQAdapterError::InvalidProbabilityDistribution,
            );
        }

        counts.insert(
            bitstring,
            count,
        );
    }

    let represented =
        counts
            .values()
            .copied()
            .try_fold(
                0usize,
                |total, value| {
                    total.checked_add(value)
                },
            )
            .ok_or(
                IonQAdapterError::NumericOverflow,
            )?;

    if represented != shots {
        return Err(
            IonQAdapterError::ShotAllocationFailure,
        );
    }

    Ok(counts)
}

// =============================================================================
// Job state conversion
// =============================================================================

fn map_ionq_job_state(
    status: &str,
) -> Result<BackendJobState, IonQAdapterError> {
    match status {
        "submitted" => {
            Ok(BackendJobState::Created)
        }

        "ready" => {
            Ok(BackendJobState::Queued)
        }

        "started" => {
            Ok(BackendJobState::Running)
        }

        "canceled" => {
            Ok(BackendJobState::Cancelled)
        }

        "failed" => {
            Ok(BackendJobState::Failed)
        }

        "completed" => {
            Ok(BackendJobState::Completed)
        }

        other => {
            Err(
                IonQAdapterError::InvalidJobState(
                    other.to_owned(),
                ),
            )
        }
    }
}

fn state_to_ionq_status(
    state: BackendJobState,
) -> String {
    match state {
        BackendJobState::Created => {
            "submitted".to_owned()
        }

        BackendJobState::Queued => {
            "ready".to_owned()
        }

        BackendJobState::Running => {
            "started".to_owned()
        }

        BackendJobState::Cancelling => {
            "started".to_owned()
        }

        BackendJobState::Cancelled => {
            "canceled".to_owned()
        }

        BackendJobState::Completed => {
            "completed".to_owned()
        }

        BackendJobState::Failed => {
            "failed".to_owned()
        }

        BackendJobState::Expired => {
            "failed".to_owned()
        }

        BackendJobState::TimedOut => {
            "failed".to_owned()
        }

        BackendJobState::Unknown => {
            "unknown".to_owned()
        }
    }
}

// =============================================================================
// Target validation
// =============================================================================

fn validate_target(
    target: &str,
) -> Result<(), IonQAdapterError> {
    if target == "simulator" {
        return Ok(());
    }

    if target.starts_with("qpu.")
        && target.len() > 4
        && target
            .chars()
            .all(|character| {
                character.is_ascii_alphanumeric()
                    || matches!(
                        character,
                        '.' | '-' | '_'
                    )
            })
    {
        return Ok(());
    }

    Err(
        IonQAdapterError::InvalidTarget(
            target.to_owned(),
        ),
    )
}

fn canonical_backend_id(
    target: &str,
) -> String {
    format!(
        "{}/{}",
        IONQ_PROVIDER_ID,
        target
    )
}

// =============================================================================
// Request identifiers
// =============================================================================

fn canonical_request_id(
    request: &ExecutionRequest,
    program: &BackendProgram,
) -> String {
    if let Some(request_id) =
        request.request_id.as_deref()
    {
        return request_id.to_owned();
    }

    let mut hasher =
        Sha256::new();

    hasher.update(
        IONQ_PROVIDER_ID.as_bytes(),
    );

    hasher.update(
        self::IONQ_API_VERSION.as_bytes(),
    );

    hasher.update(
        program.format().as_bytes(),
    );

    hasher.update(
        program.bytes(),
    );

    hasher.update(
        request.workload.circuit.shots
            .to_le_bytes(),
    );

    format!(
        "zamani-ionq-{}",
        hex::encode(
            hasher.finalize()
        )
    )
}

fn request_id_for_job(
    operation: &str,
    job: &BackendJobId,
) -> String {
    stable_hash_id(
        operation,
        job.as_str(),
    )
}

fn stable_hash_id(
    namespace: &str,
    value: &str,
) -> String {
    let mut hasher =
        Sha256::new();

    hasher.update(
        namespace.as_bytes(),
    );

    hasher.update(
        b":",
    );

    hasher.update(
        value.as_bytes(),
    );

    format!(
        "zamani-ionq-{}",
        hex::encode(
            hasher.finalize()
        )
    )
}

// =============================================================================
// JSON / transport helpers
// =============================================================================

fn parse_json_response(
    response: &ProviderResponse,
) -> Result<Value, IonQAdapterError> {
    serde_json::from_slice(
        &response.body,
    )
    .map_err(|_| {
        IonQAdapterError::InvalidResponseJson
    })
}

fn safe_response_message(
    response: &ProviderResponse,
) -> String {
    let length =
        response
            .body
            .len()
            .min(
                IONQ_MAX_ERROR_BODY_BYTES,
            );

    let body =
        &response.body[..length];

    match std::str::from_utf8(body) {
        Ok(text) if !text.trim().is_empty() => {
            sanitize_provider_message(text)
        }

        _ => {
            "IonQ provider returned a non-success response"
                .to_owned()
        }
    }
}

fn sanitize_provider_message(
    message: &str,
) -> String {
    let mut result =
        message
            .chars()
            .filter(|character| {
                !character.is_control()
                    || *character == '\n'
                    || *character == '\t'
            })
            .collect::<String>();

    if result.len() > 4096 {
        result.truncate(4096);
    }

    //
    // Never return likely credential material in an error.
    //
    let lower =
        result.to_ascii_lowercase();

    for marker in [
        "api_key",
        "apikey",
        "access_token",
        "authorization",
        "bearer ",
        "password",
        "private_key",
        "secret",
    ] {
        if lower.contains(marker) {
            return "IonQ provider returned a sensitive error response"
                .to_owned();
        }
    }

    result
}

/// Minimal path escaping for UUID/backend identifiers.
///
/// IonQ backend names and UUIDs accepted by this adapter are deliberately
/// restricted, so percent-encoding is only needed for defensive correctness.
fn path_escape(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'-'
            | b'_'
            | b'.' => {
                char::from(byte)
                    .to_string()
            }

            _ => {
                format!(
                    "%{:02X}",
                    byte
                )
            }
        })
        .collect()
}

// =============================================================================
// Backend construction helper
// =============================================================================

/// Builds a conservative provider-neutral IonQ capability profile.
///
/// This helper intentionally describes stable gate-model capabilities rather
/// than claiming experimental/dynamic functionality that the API adapter does
/// not implement.
pub fn default_ionq_capabilities()
    -> BackendCapabilities
{
    let mut capabilities =
        BackendCapabilities::new();

    capabilities.measurement = true;
    capabilities.reset = false;
    capabilities.mid_circuit_measurement = false;
    capabilities.classical_control = false;
    capabilities.dynamic_circuits = false;
    capabilities.arbitrary_single_qubit_rotations = true;
    capabilities.parameterized_gates = true;
    capabilities.parallel_operations = true;
    capabilities.batch_execution = true;
    capabilities.cancellation = true;
    capabilities.queue_information = true;
    capabilities.calibration_data = true;
    capabilities.topology_information = true;
    capabilities.native_instruction_set = true;
    capabilities.error_mitigation = true;
    capabilities.expectation_value_results = false;
    capabilities.state_vector_results = false;
    capabilities.density_matrix_results = false;
    capabilities.deterministic_seeding = false;

    for gate in [
        "h",
        "x",
        "y",
        "z",
        "rx",
        "ry",
        "rz",
        "cx",
        "cnot",
        "swap",
        "s",
        "sdg",
        "t",
        "tdg",
        "sx",
        "sxdg",
        "xx",
        "yy",
        "zz",
        "gpi",
        "gpi2",
        "ms",
    ] {
        capabilities =
            capabilities.with_gate(gate);
    }

    capabilities
}

/// Builds provider-neutral IonQ metadata.
///
/// `qubits` is optional because discovery data may not have been fetched yet.
pub fn default_ionq_metadata(
    target: &str,
    qubits: Option<usize>,
) -> BackendMetadata {
    let mut metadata =
        BackendMetadata::new(
            canonical_backend_id(target),
            target,
            IONQ_PROVIDER_ID,
            IONQ_API_VERSION,
            if target == "simulator" {
                BackendKind::Simulator
            } else {
                BackendKind::Qpu
            },
        )
        .with_api_version(
            IONQ_API_VERSION,
        );

    if let Some(qubits) = qubits {
        let _ = metadata
            .insert_property(
                "qubits",
                qubits.to_string(),
            );
    }

    let _ = metadata
        .insert_property(
            "provider_api",
            IONQ_API_VERSION,
        );

    let _ = metadata
        .insert_property(
            "job_type",
            IONQ_CIRCUIT_JOB_TYPE,
        );

    metadata
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_validation_accepts_current_target_shapes() {
        assert!(
            validate_target("simulator")
                .is_ok()
        );

        assert!(
            validate_target("qpu.forte-1")
                .is_ok()
        );

        assert!(
            validate_target(
                "qpu.forte-enterprise-1"
            )
            .is_ok()
        );
    }

    #[test]
    fn target_validation_rejects_invalid_target() {
        assert!(
            validate_target("")
                .is_err()
        );

        assert!(
            validate_target("qpu")
                .is_err()
        );

        assert!(
            validate_target(
                "qpu.forte/../../secret"
            )
            .is_err()
        );
    }

    #[test]
    fn backend_identifier_is_stable() {
        assert_eq!(
            canonical_backend_id(
                "qpu.forte-1"
            ),
            "ionq/qpu.forte-1"
        );
    }

    #[test]
    fn probability_parser_accepts_valid_sparse_distribution() {
        let value =
            serde_json::json!({
                "0": 0.5,
                "3": 0.5
            });

        let probabilities =
            parse_probabilities(&value)
                .expect(
                    "valid probability distribution",
                );

        assert_eq!(
            probabilities.len(),
            2
        );

        assert_eq!(
            probabilities.get(&0),
            Some(&0.5)
        );

        assert_eq!(
            probabilities.get(&3),
            Some(&0.5)
        );
    }

    #[test]
    fn probability_parser_rejects_negative_values() {
        let value =
            serde_json::json!({
                "0": -0.1,
                "1": 1.1
            });

        assert!(
            matches!(
                parse_probabilities(&value),
                Err(
                    IonQAdapterError::NegativeProbability
                )
            )
        );
    }

    #[test]
    fn probability_parser_rejects_non_normalized_distribution() {
        let value =
            serde_json::json!({
                "0": 0.25,
                "1": 0.25
            });

        assert!(
            matches!(
                parse_probabilities(&value),
                Err(
                    IonQAdapterError::ProbabilityNotNormalized { .. }
                )
            )
        );
    }

    #[test]
    fn largest_remainder_produces_exact_shots() {
        let probabilities =
            BTreeMap::from([
                (0_u64, 0.5_f64),
                (3_u64, 0.5_f64),
            ]);

        let counts =
            probabilities_to_counts(
                &probabilities,
                1001,
                2,
            )
            .expect(
                "allocation should succeed",
            );

        let total =
            counts
                .values()
                .copied()
                .sum::<usize>();

        assert_eq!(
            total,
            1001
        );

        assert_eq!(
            counts.get("00"),
            Some(&501)
        );

        assert_eq!(
            counts.get("11"),
            Some(&500)
        );
    }

    #[test]
    fn largest_remainder_is_deterministic() {
        let probabilities =
            BTreeMap::from([
                (0_u64, 0.5_f64),
                (1_u64, 0.5_f64),
            ]);

        let first =
            probabilities_to_counts(
                &probabilities,
                3,
                1,
            )
            .expect("allocation");

        let second =
            probabilities_to_counts(
                &probabilities,
                3,
                1,
            )
            .expect("allocation");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn ionq_program_shape_is_validated() {
        let program =
            BackendProgram::new(
                "ionq.circuit.v1",
                br#"{
                    "qubits": 2,
                    "gateset": "qis",
                    "circuit": [
                        {
                            "gate": "h",
                            "target": 0
                        },
                        {
                            "gate": "cnot",
                            "control": 0,
                            "target": 1
                        }
                    ]
                }"#
                .to_vec(),
            )
            .expect("program construction");

        let request =
            ExecutionRequest::new(
                Default::default(),
            );

        //
        // The default request has zero declared qubits, which means the
        // provider program is allowed to establish the concrete count.
        //
        assert!(
            parse_and_validate_program(
                &program,
                &request
            )
            .is_ok()
        );
    }

    #[test]
    fn provider_message_redacts_secret_like_errors() {
        let response =
            ProviderResponse::new(
                401,
                Default::default(),
                br#"{"error":"authorization bearer SECRET"}"#
                    .to_vec(),
                None,
                Some(
                    IONQ_API_VERSION.to_owned(),
                ),
                None,
            )
            .expect("response");

        let message =
            safe_response_message(
                &response
            );

        assert!(
            !message.contains(
                "SECRET"
            )
        );
    }

    #[test]
    fn default_capabilities_include_core_qis_gates() {
        let capabilities =
            default_ionq_capabilities();

        assert!(
            capabilities
                .native_gates
                .contains("h")
        );

        assert!(
            capabilities
                .native_gates
                .contains("cx")
        );

        assert!(
            capabilities
                .native_gates
                .contains("rz")
        );
    }

    #[test]
    fn request_ids_are_stable() {
        let id1 =
            stable_hash_id(
                "status",
                "job-1",
            );

        let id2 =
            stable_hash_id(
                "status",
                "job-1",
            );

        assert_eq!(
            id1,
            id2
        );
    }
}