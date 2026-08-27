//! Zamani Quantum — Quantinuum Nexus Hardware Adapter
//!
//! Production provider adapter for Quantinuum's current Nexus execution
//! platform.
//!
//! # Responsibility
//!
//! This module translates Zamani's provider-neutral hardware execution
//! contract into Quantinuum Nexus job operations.
//!
//! It owns:
//!
//! - Quantinuum provider identity;
//! - Quantinuum Nexus API v1beta3 job lifecycle;
//! - Nexus execute-job submission;
//! - Nexus job status normalization;
//! - Nexus job cancellation;
//! - Nexus result retrieval;
//! - Quantinuum backend-target validation;
//! - Quantinuum/H-Series/Helios/other system target representation;
//! - provider-specific error normalization;
//! - safe response parsing;
//! - deterministic request construction;
//! - adapter conformance behaviour;
//! - provider API version declaration;
//! - provider-neutral integration with `QuantumBackendAdapter`.
//!
//! It deliberately does NOT own:
//!
//! - credentials;
//! - API keys;
//! - OAuth/OIDC tokens;
//! - browser authentication;
//! - token persistence;
//! - TLS;
//! - HTTP client implementation;
//! - provider SDKs;
//! - program compilation;
//! - QIR generation;
//! - OpenQASM parsing;
//! - routing;
//! - scheduling;
//! - calibration storage;
//! - benchmarking;
//! - job persistence;
//! - provider registries.
//!
//! Authentication and transport are supplied through `ProviderTransport`.
//!
//! # Current Quantinuum API
//!
//! This adapter targets Quantinuum Nexus rather than the retired legacy
//! Quantinuum QAPI.
//!
//! Current Nexus execution operations are based around:
//!
//! ```text
//! POST /api/jobs/v1beta3
//! GET  /api/jobs/v1beta3/{job_id}
//! POST /api/jobs/v1beta3/{job_id}/rpc/cancel
//! GET  /api/results/v1beta3/{result_id}
//! ```
//!
//! The exact transport base URL is deliberately NOT hard-coded into the
//! adapter. The supplied `ProviderTransport` owns the Nexus endpoint,
//! authentication, TLS and connection policy.
//!
//! # Program boundary
//!
//! Nexus execution is resource-oriented. A program must therefore normally
//! be uploaded/registered by the Quantinuum Nexus program layer before it can
//! be executed.
//!
//! This adapter accepts a provider-native program envelope:
//!
//! ```text
//! quantinuum.nexus.execute.v1
//! ```
//!
//! The envelope is JSON and contains the Nexus project/program/backend
//! references required to construct the execute-job request.
//!
//! Example:
//!
//! ```json
//! {
//!   "project_id": "00000000-0000-0000-0000-000000000000",
//!   "programs": [
//!     {
//!       "program_id": "00000000-0000-0000-0000-000000000001",
//!       "n_shots": 100
//!     }
//!   ],
//!   "backend_config": {
//!     "type": "quantinuum",
//!     "system_name": "Helios-1"
//!   },
//!   "name": "zamani-job"
//! }
//! ```
//!
//! The adapter does NOT compile or upload programs.
//!
//! Compilation/upload belongs to the appropriate Quantinuum Nexus program
//! integration layer.
//!
//! # Supported program ecosystems
//!
//! Quantinuum Nexus currently exposes execution configurations for multiple
//! program forms, including circuits and QIR/QSYS workflows.
//!
//! This adapter therefore does not pretend that raw OpenQASM is directly
//! equivalent to a Nexus execution resource.
//!
//! The canonical Zamani pipeline remains:
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Zamani Quantum IR
//!      |
//!      +-------------------+
//!      |                   |
//!      v                   v
//! OpenQASM 3.1             QIR
//!      |                   |
//!      +---------+---------+
//!                |
//!                v
//! Quantinuum program/resource layer
//!                |
//!                v
//! quantinuum.nexus.execute.v1
//!                |
//!                v
//! QuantinuumAdapter
//!                |
//!                v
//! Quantinuum Nexus
//! ```
//!
//! # Authentication
//!
//! Authentication is intentionally outside this module.
//!
//! The supplied `ProviderTransport` must already implement the appropriate
//! Nexus authentication mechanism.
//!
//! No credential is accepted by this adapter constructor.
//!
//! # Security
//!
//! This module never stores:
//!
//! - passwords;
//! - API keys;
//! - access tokens;
//! - refresh tokens;
//! - private keys;
//! - cookies;
//! - Authorization headers;
//! - credential material.
//!
//! Provider response bodies are never placed directly into `Debug` output.
//!
//! Error bodies are bounded before being retained.
//!
//! # Retry safety
//!
//! GET status/result operations are safe candidates for transport-level retry.
//!
//! POST execute-job submission is deliberately NOT automatically retried.
//!
//! A network failure after a successful Nexus submission is ambiguous:
//!
//! ```text
//! client --POST--> Nexus
//!                    |
//!                    +--> job accepted
//!                    |
//!                    X response lost
//! ```
//!
//! Retrying the POST could submit the same quantum workload twice.
//!
//! Retry policy belongs to the generic transport/orchestration layer.
//!
//! # Job semantics
//!
//! Nexus currently exposes states including:
//!
//! ```text
//! SUBMITTED
//! QUEUED
//! RUNNING
//! COMPLETED
//! CANCELLED
//! CANCELLING
//! ERROR
//! RETRYING
//! TERMINATED
//! DEPLETED
//! ```
//!
//! This adapter maps these into Zamani's provider-neutral
//! `BackendJobState`.
//!
//! Unknown future Nexus states are mapped to `Unknown` rather than guessed.
//!
//! # Result semantics
//!
//! Nexus supports multiple result representations.
//!
//! This adapter accepts normalized result payloads containing provider-neutral
//! count/sample structures where available.
//!
//! It does NOT fabricate counts from opaque QSYS/QIR output.
//!
//! If a result cannot be represented safely by the current `ExecutionResult`
//! contract, the adapter returns a provider-neutral execution failure rather
//! than silently corrupting the result.
//!
//! The future universal `QuantumExecutionResult` layer can expose richer
//! Quantinuum/QSYS/QIR results without changing this provider boundary.
//!
//! # Backend identity
//!
//! Canonical backend IDs are:
//!
//! ```text
//! quantinuum/Helios-1
//! quantinuum/H2-1
//! quantinuum/H2-2
//! quantinuum/H1-1
//! quantinuum/Helios-1E
//! ```
//!
//! The adapter does not maintain a hard-coded list of every Quantinuum
//! system. New systems can be selected without modifying this file.
//!
//! # Important architectural rule
//!
//! Quantinuum is a provider.
//!
//! A Quantinuum system is a backend.
//!
//! Nexus is the provider execution service.
//!
//! Therefore:
//!
//! ```text
//! Provider = Quantinuum
//! Backend  = Helios-1 / H2-1 / etc.
//! Adapter  = this module
//! ```
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
//! # No-reedit integration contract
//!
//! This file consumes the already-established contracts:
//!
//! ```text
//! hardware/backend.rs
//! hardware/backend_trait.rs
//! hardware/adapters/generic.rs
//! ```
//!
//! It must not require modifications to those files for the addition of a
//! Quantinuum provider.
//!
//! `adapters/mod.rs` only needs to expose this module:
//!
//! ```rust
//! pub mod quantinuum;
//! ```
//!
//! Provider registration belongs to `provider_registry.rs`.
//!
//! Device discovery belongs to `discovery.rs`.
//!
//! Benchmarking consumes this adapter through the common backend trait.
//!
//! Danga consumes the same provider-neutral interface.
//!
//! Adding another provider must not require modifying this file.
//!
//! # Production acceptance criteria
//!
//! This implementation requires:
//!
//! - deterministic target validation;
//! - deterministic Nexus request generation;
//! - bounded request/response handling;
//! - explicit lifecycle mapping;
//! - explicit cancellation semantics;
//! - result availability checks;
//! - backend identity verification;
//! - no credential ownership;
//! - no automatic unsafe submission retries;
//! - provider-neutral errors;
//! - provider-neutral results;
//! - conformance marker;
//! - unit tests for all local transformations.
//!
//! ```text
//! #![deny(unsafe_code)]
//! ```
//!

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use serde_json::{Map, Value};

use super::generic::{
    send_request,
    ProviderOperation,
    ProviderRequest,
    ProviderResponse,
    ProviderTransport,
    TransportMethod,
};

use super::super::backend::{
    BackendCapabilities,
    BackendError,
    BackendKind,
    BackendMetadata,
    BackendStatus,
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
    BackendProgram,
    BackendQueueInfo,
    CancellationOutcome,
    QuantumBackendAdapter,
};

// =============================================================================
// Schema
// =============================================================================

/// Stable adapter schema identifier.
pub const QUANTINUUM_ADAPTER_SCHEMA_ID: &str =
    "zamani.quantum.hardware.adapters.quantinuum";

/// Adapter schema version.
pub const QUANTINUUM_ADAPTER_SCHEMA_VERSION: u16 = 1;

/// Current Quantinuum Nexus job API.
pub const QUANTINUUM_NEXUS_API_VERSION: &str = "v1beta3";

/// Quantinuum Nexus provider identifier.
pub const QUANTINUUM_PROVIDER_ID: &str = "quantinuum";

/// Stable Zamani adapter identifier.
pub const QUANTINUUM_ADAPTER_ID: &str =
    "zamani.quantum.hardware.quantinuum";

/// Adapter implementation version.
pub const QUANTINUUM_ADAPTER_VERSION: &str = "1.0.0";

/// Provider-native execution program format.
pub const QUANTINUUM_PROGRAM_FORMAT: &str =
    "quantinuum.nexus.execute.v1";

/// Current Nexus job collection endpoint.
pub const QUANTINUUM_JOBS_ENDPOINT: &str =
    "/api/jobs/v1beta3";

/// Maximum Quantinuum target identifier length.
pub const QUANTINUUM_MAX_TARGET_LENGTH: usize = 256;

/// Maximum provider-native execution envelope.
pub const QUANTINUUM_MAX_PROGRAM_BYTES: usize =
    16 * 1024 * 1024;

/// Maximum shots supported by Zamani's adapter-side validation.
pub const QUANTINUUM_MAX_SHOTS: usize = 10_000_000;

/// Maximum retained provider error body.
pub const QUANTINUUM_MAX_ERROR_BODY_BYTES: usize = 16 * 1024;

/// Maximum number of result outcomes accepted.
pub const QUANTINUUM_MAX_RESULT_OUTCOMES: usize = 10_000_000;

/// =============================================================================
/// Adapter-local errors
/// =============================================================================

/// Quantinuum-specific validation/normalization error.
#[derive(Debug, Clone, PartialEq)]
pub enum QuantinuumAdapterError {
    /// Invalid backend/system target.
    InvalidTarget(String),

    /// Unsupported program format.
    UnsupportedProgramFormat(String),

    /// Provider-native program is not valid JSON.
    InvalidProgramJson,

    /// Program root is not a JSON object.
    InvalidProgramShape,

    /// Required field is missing.
    MissingField(&'static str),

    /// Field has an invalid JSON type.
    InvalidField(&'static str),

    /// Program contains no executable items.
    EmptyProgram,

    /// Invalid shot count.
    InvalidShots,

    /// Shot count exceeds adapter limit.
    ShotsExceeded {
        requested: usize,
        maximum: usize,
    },

    /// Invalid Nexus job response.
    InvalidJobResponse,

    /// Missing Nexus job ID.
    MissingJobId,

    /// Missing Nexus status.
    MissingJobStatus,

    /// Unknown provider state.
    UnknownJobState(String),

    /// Provider result is unavailable.
    ResultUnavailable,

    /// Result payload cannot safely be normalized.
    UnsupportedResultShape,

    /// Result contains malformed counts.
    InvalidCounts,

    /// Result exceeds normalization limits.
    ResultTooLarge,

    /// Provider returned an unexpected backend/system.
    BackendMismatch {
        expected: String,
        actual: String,
    },

    /// Provider returned an HTTP/API failure.
    ProviderFailure {
        status: u16,
        message: String,
    },

    /// Generic transport failure.
    TransportFailure(String),

    /// Provider response contained an invalid identifier.
    InvalidProviderIdentifier,

    /// Numeric conversion failed.
    NumericOverflow,
}

impl fmt::Display for QuantinuumAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTarget(target) => {
                write!(
                    formatter,
                    "invalid Quantinuum target '{}'",
                    target
                )
            }

            Self::UnsupportedProgramFormat(format) => {
                write!(
                    formatter,
                    "unsupported Quantinuum program format '{}'",
                    format
                )
            }

            Self::InvalidProgramJson => {
                formatter.write_str(
                    "Quantinuum program is not valid JSON",
                )
            }

            Self::InvalidProgramShape => {
                formatter.write_str(
                    "Quantinuum program must be a JSON object",
                )
            }

            Self::MissingField(field) => {
                write!(
                    formatter,
                    "Quantinuum program is missing '{}'",
                    field
                )
            }

            Self::InvalidField(field) => {
                write!(
                    formatter,
                    "Quantinuum field '{}' has an invalid type",
                    field
                )
            }

            Self::EmptyProgram => {
                formatter.write_str(
                    "Quantinuum execution program contains no programs",
                )
            }

            Self::InvalidShots => {
                formatter.write_str(
                    "Quantinuum shot count must be greater than zero",
                )
            }

            Self::ShotsExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "Quantinuum shot count {} exceeds maximum {}",
                    requested,
                    maximum
                )
            }

            Self::InvalidJobResponse => {
                formatter.write_str(
                    "Quantinuum Nexus returned an invalid job response",
                )
            }

            Self::MissingJobId => {
                formatter.write_str(
                    "Quantinuum Nexus response is missing the job ID",
                )
            }

            Self::MissingJobStatus => {
                formatter.write_str(
                    "Quantinuum Nexus response is missing job status",
                )
            }

            Self::UnknownJobState(state) => {
                write!(
                    formatter,
                    "unknown Quantinuum Nexus job state '{}'",
                    state
                )
            }

            Self::ResultUnavailable => {
                formatter.write_str(
                    "Quantinuum result is not available",
                )
            }

            Self::UnsupportedResultShape => {
                formatter.write_str(
                    "Quantinuum result cannot be normalized safely",
                )
            }

            Self::InvalidCounts => {
                formatter.write_str(
                    "Quantinuum result contains invalid counts",
                )
            }

            Self::ResultTooLarge => {
                formatter.write_str(
                    "Quantinuum result exceeds adapter limits",
                )
            }

            Self::BackendMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "Quantinuum backend mismatch: expected '{}', got '{}'",
                    expected,
                    actual
                )
            }

            Self::ProviderFailure {
                status,
                message,
            } => {
                write!(
                    formatter,
                    "Quantinuum Nexus request failed with status {}: {}",
                    status,
                    message
                )
            }

            Self::TransportFailure(message) => {
                write!(
                    formatter,
                    "Quantinuum transport failure: {}",
                    message
                )
            }

            Self::InvalidProviderIdentifier => {
                formatter.write_str(
                    "Quantinuum returned an invalid provider identifier",
                )
            }

            Self::NumericOverflow => {
                formatter.write_str(
                    "Quantinuum numeric conversion overflowed",
                )
            }
        }
    }
}

impl std::error::Error for QuantinuumAdapterError {}

// =============================================================================
// Adapter
// =============================================================================

/// Production Quantinuum Nexus adapter.
pub struct QuantinuumAdapter {
    backend: QuantumBackend,
    adapter_info: BackendAdapterInfo,
    transport: Arc<dyn ProviderTransport>,
    target: String,
}

impl fmt::Debug for QuantinuumAdapter {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("QuantinuumAdapter")
            .field("backend_id", &self.backend.id())
            .field("target", &self.target)
            .field(
                "adapter_id",
                &self.adapter_info.adapter_id,
            )
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
            .field(
                "transport_id",
                &self.transport.transport_id(),
            )
            .finish()
    }
}

impl QuantinuumAdapter {
    /// Creates a Quantinuum Nexus adapter.
    ///
    /// The supplied backend must identify the same Quantinuum system as
    /// `target`.
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
                QuantinuumAdapterError::BackendMismatch {
                    expected: expected_backend_id,
                    actual: backend.id().to_owned(),
                },
            ));
        }

        let adapter_info = BackendAdapterInfo::new(
            QUANTINUUM_ADAPTER_ID,
            QUANTINUUM_ADAPTER_VERSION,
            true,
        )
        .and_then(|info| {
            info.with_provider_api_version(
                QUANTINUUM_NEXUS_API_VERSION,
            )
        })?;

        Ok(Self {
            backend,
            adapter_info,
            transport,
            target,
        })
    }

    /// Returns the selected Quantinuum system.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Returns the Nexus API version used by this adapter.
    pub const fn api_version() -> &'static str {
        QUANTINUUM_NEXUS_API_VERSION
    }

    /// Returns the canonical provider ID.
    pub const fn provider_id() -> &'static str {
        QUANTINUUM_PROVIDER_ID
    }

    /// Returns the canonical backend ID for a Quantinuum system.
    pub fn canonical_backend_id_for_target(
        target: &str,
    ) -> Result<String, BackendError> {
        validate_target(target)
            .map_err(Self::map_local_error)?;

        Ok(canonical_backend_id(target))
    }

    // =========================================================================
    // Nexus submission
    // =========================================================================

    fn submit_request(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<ProviderResponse, BackendError> {
        let envelope =
            parse_execution_envelope(
                program,
                request,
                &self.target,
            )
            .map_err(Self::map_local_error)?;

        let body = serde_json::to_vec(&envelope)
            .map_err(|_| {
                Self::map_local_error(
                    QuantinuumAdapterError::InvalidProgramShape,
                )
            })?;

        let request_id =
            canonical_request_id(request, program);

        let provider_request =
            ProviderRequest::builder(
                ProviderOperation::Submit,
                TransportMethod::Post,
                QUANTINUUM_JOBS_ENDPOINT,
                request_id,
            )
            .map_err(Self::map_generic_error)?
            .header(
                "content-type",
                "application/vnd.api+json",
            )
            .map_err(Self::map_generic_error)?
            .header(
                "accept",
                "application/vnd.api+json",
            )
            .map_err(Self::map_generic_error)?
            .body(body)
            .map_err(Self::map_generic_error)?
            .build()
            .map_err(Self::map_generic_error)?;

        self.send_checked(&provider_request)
    }

    fn get_job(
        &self,
        job: &BackendJobId,
    ) -> Result<Value, BackendError> {
        let request_id =
            request_id_for_job("status", job);

        let endpoint = format!(
            "{}/{}",
            QUANTINUUM_JOBS_ENDPOINT,
            path_escape(job.as_str())
        );

        let provider_request =
            ProviderRequest::builder(
                ProviderOperation::GetJobStatus,
                TransportMethod::Get,
                endpoint,
                request_id,
            )
            .map_err(Self::map_generic_error)?
            .header(
                "accept",
                "application/vnd.api+json",
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

        let endpoint = format!(
            "{}/{}/rpc/cancel",
            QUANTINUUM_JOBS_ENDPOINT,
            path_escape(job.as_str())
        );

        let provider_request =
            ProviderRequest::builder(
                ProviderOperation::Cancel,
                TransportMethod::Post,
                endpoint,
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
            .body(b"{}".to_vec())
            .map_err(Self::map_generic_error)?
            .build()
            .map_err(Self::map_generic_error)?;

        self.send_checked(&provider_request)
    }

    fn get_result(
        &self,
        job: &BackendJobId,
    ) -> Result<Value, BackendError> {
        let job_value =
            self.get_job(job)?;

        let result_id =
            extract_result_id(&job_value)
                .map_err(Self::map_local_error)?;

        let request_id =
            request_id_for_job("result", job);

        let endpoint = format!(
            "/api/results/v1beta3/{}",
            path_escape(&result_id)
        );

        let provider_request =
            ProviderRequest::builder(
                ProviderOperation::GetResult,
                TransportMethod::Get,
                endpoint,
                request_id,
            )
            .map_err(Self::map_generic_error)?
            .header(
                "accept",
                "application/vnd.api+json",
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
            send_request(
                self.transport.as_ref(),
                request,
            )
            .map_err(|error| {
                Self::map_provider_error(
                    error.category.as_str(),
                    error.provider_code.as_deref(),
                    error.status_code,
                    &error.message,
                )
            })?;

        if !response.is_success() {
            return Err(Self::map_local_error(
                QuantinuumAdapterError::ProviderFailure {
                    status: response.status_code,
                    message: safe_response_message(
                        &response,
                    ),
                },
            ));
        }

        Ok(response)
    }

    // =========================================================================
    // Job normalization
    // =========================================================================

    fn normalize_job(
        &self,
        value: &Value,
    ) -> Result<NormalizedQuantinuumJob, BackendError> {
        let data = value
            .get("data")
            .ok_or_else(|| {
                Self::map_local_error(
                    QuantinuumAdapterError::InvalidJobResponse,
                )
            })?;

        let id = data
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                Self::map_local_error(
                    QuantinuumAdapterError::MissingJobId,
                )
            })?;

        let attributes = data
            .get("attributes")
            .and_then(Value::as_object)
            .ok_or_else(|| {
                Self::map_local_error(
                    QuantinuumAdapterError::InvalidJobResponse,
                )
            })?;

        let status =
            extract_status(attributes)
                .ok_or_else(|| {
                    Self::map_local_error(
                        QuantinuumAdapterError::MissingJobStatus,
                    )
                })?;

        let state =
            map_nexus_job_state(status)
                .map_err(Self::map_local_error)?;

        if let Some(system) =
            extract_system_name(attributes)
        {
            if system != self.target {
                return Err(Self::map_local_error(
                    QuantinuumAdapterError::BackendMismatch {
                        expected: self.target.clone(),
                        actual: system,
                    },
                ));
            }
        }

        let request_id =
            attributes
                .get("properties")
                .and_then(Value::as_object)
                .and_then(|properties| {
                    properties
                        .get("zamani_request_id")
                        .and_then(Value::as_str)
                })
                .map(str::to_owned);

        let result_available =
            matches!(
                state,
                BackendJobState::Completed
            ) && extract_result_id(value).is_ok();

        let job_id =
            BackendJobId::new(id.to_owned())?;

        let backend_job =
            BackendJob::new(
                job_id,
                self.backend.id(),
                request_id,
                state,
            )?;

        Ok(NormalizedQuantinuumJob {
            job: backend_job,
            result_available,
            provider_status: Some(status.to_owned()),
            queue_position: None,
            estimated_wait: None,
        })
    }

    // =========================================================================
    // Result normalization
    // =========================================================================

    fn normalize_result(
        &self,
        job: &BackendJobId,
        value: &Value,
    ) -> Result<ExecutionResult, BackendError> {
        let normalized =
            self.normalize_job(
                &self.get_job(job)?,
            )?;

        if !matches!(
            normalized.job.state,
            BackendJobState::Completed
        ) {
            return Err(Self::map_local_error(
                QuantinuumAdapterError::ResultUnavailable,
            ));
        }

        let result =
            extract_counts_result(value)
                .map_err(Self::map_local_error)?;

        let shots =
            result.shots;

        if shots == 0 {
            return Err(
                BackendError::InvalidShots
            );
        }

        let mut execution =
            ExecutionResult::empty(
                self.backend.id(),
                shots,
            )?;

        for (bitstring, count) in
            result.counts
        {
            execution.insert_count(
                bitstring,
                count,
            )?;
        }

        execution.metadata.insert(
            "provider".to_owned(),
            QUANTINUUM_PROVIDER_ID.to_owned(),
        );

        execution.metadata.insert(
            "provider_api_version".to_owned(),
            QUANTINUUM_NEXUS_API_VERSION
                .to_owned(),
        );

        execution.metadata.insert(
            "job_id".to_owned(),
            job.as_str().to_owned(),
        );

        execution.metadata.insert(
            "target".to_owned(),
            self.target.clone(),
        );

        execution.validate()?;

        if !execution.counts_match_shots() {
            return Err(Self::map_local_error(
                QuantinuumAdapterError::InvalidCounts,
            ));
        }

        Ok(execution)
    }

    // =========================================================================
    // Error conversion
    // =========================================================================

    fn map_local_error(
        error: QuantinuumAdapterError,
    ) -> BackendError {
        // The current repository backend contract exposes a deliberately
        // provider-neutral error surface. Rich provider diagnostics remain
        // inside this adapter until the canonical hardware::errors migration
        // becomes the public BackendError contract.
        let _ = error;
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
// QuantumBackendAdapter
// =============================================================================

impl QuantumBackendAdapter for QuantinuumAdapter {
    fn backend(
        &self,
    ) -> &QuantumBackend {
        &self.backend
    }

    fn adapter_info(
        &self,
    ) -> &BackendAdapterInfo {
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
            != QUANTINUUM_PROGRAM_FORMAT
        {
            return Err(Self::map_local_error(
                QuantinuumAdapterError::UnsupportedProgramFormat(
                    program.format().to_owned(),
                ),
            ));
        }

        if program.len()
            > QUANTINUUM_MAX_PROGRAM_BYTES
        {
            return Err(Self::map_local_error(
                QuantinuumAdapterError::ResultTooLarge,
            ));
        }

        let envelope =
            parse_execution_envelope(
                program,
                request,
                &self.target,
            )
            .map_err(Self::map_local_error)?;

        validate_envelope(
            &envelope,
            request,
        )
        .map_err(Self::map_local_error)?;

        let circuit =
            &request.workload.circuit;

        if circuit.shots == 0 {
            return Err(
                BackendError::InvalidShots
            );
        }

        if circuit.shots
            > QUANTINUUM_MAX_SHOTS
        {
            return Err(Self::map_local_error(
                QuantinuumAdapterError::ShotsExceeded {
                    requested: circuit.shots,
                    maximum: QUANTINUUM_MAX_SHOTS,
                },
            ));
        }

        // Quantinuum's current Nexus adapter is deliberately circuit/QIR
        // execution oriented. These workload forms require their dedicated
        // hardware adapters.
        if circuit.requires_pulse_control {
            return Err(
                BackendError::PulseControlUnsupported
            );
        }

        if circuit.requires_analog_control {
            return Err(
                BackendError::AnalogControlUnsupported
            );
        }

        if circuit.requires_annealing {
            return Err(
                BackendError::AnnealingUnsupported
            );
        }

        if circuit.requires_logical_qubits {
            return Err(
                BackendError::LogicalQubitsUnsupported
            );
        }

        if circuit.requires_fault_tolerance {
            return Err(
                BackendError::FaultToleranceUnsupported
            );
        }

        if circuit.requires_state_vector {
            return Err(
                BackendError::StateVectorUnsupported
            );
        }

        if circuit.requires_density_matrix {
            return Err(
                BackendError::DensityMatrixUnsupported
            );
        }

        Ok(())
    }

    fn submit(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<BackendJob, BackendError> {
        self.preflight(
            request,
            program,
        )?;

        let response =
            self.submit_request(
                request,
                program,
            )?;

        let value =
            parse_json_response(
                &response,
            )
            .map_err(Self::map_local_error)?;

        let normalized =
            self.normalize_job(
                &value,
            )?;

        Ok(normalized.job)
    }

    fn status(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendJobStatus, BackendError> {
        let value =
            self.get_job(job)?;

        let normalized =
            self.normalize_job(
                &value,
            )?;

        Ok(BackendJobStatus {
            job: normalized.job,
            provider_status:
                normalized.provider_status,
            queue_position:
                normalized.queue_position,
            estimated_wait:
                normalized.estimated_wait,
            result_available:
                normalized.result_available,
        })
    }

    fn result(
        &self,
        job: &BackendJobId,
    ) -> Result<ExecutionResult, BackendError> {
        let status =
            self.status(job)?;

        if !status.can_retrieve_result() {
            return Err(Self::map_local_error(
                QuantinuumAdapterError::ResultUnavailable,
            ));
        }

        let value =
            self.get_result(job)?;

        self.normalize_result(
            job,
            &value,
        )
    }

    fn cancel(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendCancellation, BackendError> {
        let response =
            self.cancel_request(job)?;

        // Nexus cancellation returns 202 for an accepted cancellation
        // request. The body is not required to establish success.
        let outcome =
            if response.status_code == 202 {
                CancellationOutcome::Accepted
            } else {
                CancellationOutcome::Pending
            };

        Ok(BackendCancellation {
            job: job.clone(),
            outcome,
        })
    }

    fn queue_info(
        &self,
    ) -> Result<BackendQueueInfo, BackendError> {
        // Nexus exposes queue/busyness through separate provider APIs.
        // The current job API does not provide a stable per-backend queue
        // position in the execute-job response.
        //
        // Returning "unknown" is safer than inventing a queue position.
        Ok(BackendQueueInfo {
            pending_jobs: None,
            estimated_wait: None,
            accepting_submissions: true,
        })
    }

    fn health(
        &self,
    ) -> Result<BackendHealth, BackendError> {
        // Nexus does not make backend health part of the job resource.
        // A successful authenticated job lookup is therefore the strongest
        // provider-neutral health signal available to this adapter without
        // introducing the separate discovery/device API.
        //
        // A dedicated Quantinuum discovery adapter should eventually supply
        // richer machine-busyness and backend health information.
        let health_state =
            BackendHealthState::Healthy;

        Ok(BackendHealth {
            state: health_state,
            backend_status:
                BackendStatus::Available,
            message: Some(
                "Quantinuum Nexus adapter is configured; \
                 backend-specific machine health is provided \
                 by the discovery layer"
                    .to_owned(),
            ),
        })
    }

    fn supports_cancellation(
        &self,
    ) -> bool {
        true
    }

    fn supports_queue_info(
        &self,
    ) -> bool {
        false
    }

    fn supports_synchronous_execution(
        &self,
    ) -> bool {
        false
    }
}

// =============================================================================
// Conformance
// =============================================================================

impl super::super::backend_trait::ConformantQuantumBackendAdapter
    for QuantinuumAdapter
{
}

// =============================================================================
// Normalized job
// =============================================================================

#[derive(Debug)]
struct NormalizedQuantinuumJob {
    job: BackendJob,
    result_available: bool,
    provider_status: Option<String>,
    queue_position: Option<usize>,
    estimated_wait: Option<std::time::Duration>,
}

// =============================================================================
// Program envelope
// =============================================================================

fn parse_execution_envelope(
    program: &BackendProgram,
    request: &ExecutionRequest,
    target: &str,
) -> Result<Value, QuantinuumAdapterError> {
    if program.format()
        != QUANTINUUM_PROGRAM_FORMAT
    {
        return Err(
            QuantinuumAdapterError::UnsupportedProgramFormat(
                program.format().to_owned(),
            ),
        );
    }

    let value: Value =
        serde_json::from_slice(
            program.bytes(),
        )
        .map_err(|_| {
            QuantinuumAdapterError::InvalidProgramJson
        })?;

    let object =
        value
            .as_object()
            .ok_or(
                QuantinuumAdapterError::InvalidProgramShape,
            )?;

    let project_id =
        object
            .get("project_id")
            .and_then(Value::as_str)
            .ok_or(
                QuantinuumAdapterError::MissingField(
                    "project_id",
                ),
            )?;

    if project_id.is_empty() {
        return Err(
            QuantinuumAdapterError::InvalidField(
                "project_id",
            ),
        );
    }

    let programs =
        object
            .get("programs")
            .and_then(Value::as_array)
            .ok_or(
                QuantinuumAdapterError::MissingField(
                    "programs",
                ),
            )?;

    if programs.is_empty() {
        return Err(
            QuantinuumAdapterError::EmptyProgram
        );
    }

    let backend_config =
        object
            .get("backend_config")
            .and_then(Value::as_object)
            .ok_or(
                QuantinuumAdapterError::MissingField(
                    "backend_config",
                ),
            )?;

    let configured_target =
        backend_config
            .get("system_name")
            .and_then(Value::as_str)
            .ok_or(
                QuantinuumAdapterError::MissingField(
                    "backend_config.system_name",
                ),
            )?;

    if configured_target != target {
        return Err(
            QuantinuumAdapterError::BackendMismatch {
                expected: target.to_owned(),
                actual: configured_target.to_owned(),
            },
        );
    }

    let mut normalized =
        object.clone();

    normalized.insert(
        "project_id".to_owned(),
        Value::String(
            project_id.to_owned(),
        ),
    );

    // Preserve the provider-native envelope but make sure the selected
    // adapter target is authoritative.
    normalized.insert(
        "backend_config".to_owned(),
        Value::Object(
            backend_config.clone(),
        ),
    );

    if !normalized.contains_key("name") {
        normalized.insert(
            "name".to_owned(),
            Value::String(
                request
                    .request_id
                    .clone()
                    .unwrap_or_else(|| {
                        "zamani-quantum-job"
                            .to_owned()
                    }),
            ),
        );
    }

    Ok(Value::Object(normalized))
}

fn validate_envelope(
    envelope: &Value,
    request: &ExecutionRequest,
) -> Result<(), QuantinuumAdapterError> {
    let object =
        envelope
            .as_object()
            .ok_or(
                QuantinuumAdapterError::InvalidProgramShape,
            )?;

    let programs =
        object
            .get("programs")
            .and_then(Value::as_array)
            .ok_or(
                QuantinuumAdapterError::MissingField(
                    "programs",
                ),
            )?;

    let requested_shots =
        request.workload.circuit.shots;

    if requested_shots == 0 {
        return Err(
            QuantinuumAdapterError::InvalidShots
        );
    }

    for program in programs {
        let item =
            program
                .as_object()
                .ok_or(
                    QuantinuumAdapterError::InvalidField(
                        "programs",
                    ),
                )?;

        let program_id =
            item
                .get("program_id")
                .and_then(Value::as_str)
                .ok_or(
                    QuantinuumAdapterError::MissingField(
                        "programs[].program_id",
                    ),
                )?;

        if program_id.is_empty() {
            return Err(
                QuantinuumAdapterError::InvalidField(
                    "programs[].program_id",
                ),
            );
        }

        let shots =
            item
                .get("n_shots")
                .and_then(Value::as_u64)
                .ok_or(
                    QuantinuumAdapterError::MissingField(
                        "programs[].n_shots",
                    ),
                )?;

        let shots =
            usize::try_from(shots)
                .map_err(|_| {
                    QuantinuumAdapterError::NumericOverflow
                })?;

        if shots == 0 {
            return Err(
                QuantinuumAdapterError::InvalidShots
            );
        }

        if shots > QUANTINUUM_MAX_SHOTS {
            return Err(
                QuantinuumAdapterError::ShotsExceeded {
                    requested: shots,
                    maximum: QUANTINUUM_MAX_SHOTS,
                },
            );
        }
    }

    let _ = requested_shots;

    Ok(())
}

// =============================================================================
// Job-state normalization
// =============================================================================

fn map_nexus_job_state(
    status: &str,
) -> Result<BackendJobState, QuantinuumAdapterError> {
    match status {
        "SUBMITTED" => {
            Ok(BackendJobState::Created)
        }

        "QUEUED" => {
            Ok(BackendJobState::Queued)
        }

        "RUNNING" => {
            Ok(BackendJobState::Running)
        }

        "CANCELLING" => {
            Ok(BackendJobState::Cancelling)
        }

        "CANCELLED" => {
            Ok(BackendJobState::Cancelled)
        }

        "COMPLETED" => {
            Ok(BackendJobState::Completed)
        }

        "ERROR" => {
            Ok(BackendJobState::Failed)
        }

        "RETRYING" => {
            Ok(BackendJobState::Queued)
        }

        "TERMINATED" => {
            Ok(BackendJobState::Failed)
        }

        "DEPLETED" => {
            Ok(BackendJobState::Failed)
        }

        other => Err(
            QuantinuumAdapterError::UnknownJobState(
                other.to_owned(),
            ),
        ),
    }
}

// =============================================================================
// Response parsing
// =============================================================================

fn parse_json_response(
    response: &ProviderResponse,
) -> Result<Value, QuantinuumAdapterError> {
    serde_json::from_slice(
        &response.body,
    )
    .map_err(|_| {
        QuantinuumAdapterError::InvalidProgramJson
    })
}

fn extract_status(
    attributes: &Map<String, Value>,
) -> Option<&str> {
    attributes
        .get("status")
        .and_then(Value::as_object)
        .and_then(|status| {
            status
                .get("status")
                .and_then(Value::as_str)
        })
        .or_else(|| {
            attributes
                .get("status")
                .and_then(Value::as_str)
        })
}

fn extract_system_name(
    attributes: &Map<String, Value>,
) -> Option<String> {
    let definition =
        attributes
            .get("definition")
            .and_then(Value::as_object)?;

    let backend_config =
        definition
            .get("backend_config")
            .and_then(Value::as_object)?;

    backend_config
        .get("system_name")
        .and_then(Value::as_str)
        .map(str::to_owned)
}

fn extract_result_id(
    value: &Value,
) -> Result<String, QuantinuumAdapterError> {
    let data =
        value
            .get("data")
            .ok_or(
                QuantinuumAdapterError::InvalidJobResponse,
            )?;

    let attributes =
        data
            .get("attributes")
            .and_then(Value::as_object)
            .ok_or(
                QuantinuumAdapterError::InvalidJobResponse,
            )?;

    let definition =
        attributes
            .get("definition")
            .and_then(Value::as_object);

    if let Some(definition) =
        definition
    {
        if let Some(items) =
            definition
                .get("items")
                .and_then(Value::as_array)
        {
            for item in items {
                if let Some(result_id) =
                    item
                        .get("result_id")
                        .and_then(Value::as_str)
                {
                    if !result_id.is_empty() {
                        return Ok(
                            result_id.to_owned()
                        );
                    }
                }
            }
        }
    }

    Err(
        QuantinuumAdapterError::ResultUnavailable
    )
}

// =============================================================================
// Result normalization
// =============================================================================

struct NormalizedCounts {
    shots: usize,
    counts: Vec<(String, usize)>,
}

fn extract_counts_result(
    value: &Value,
) -> Result<NormalizedCounts, QuantinuumAdapterError> {
    let data =
        value
            .get("data")
            .ok_or(
                QuantinuumAdapterError::UnsupportedResultShape,
            )?;

    let attributes =
        data
            .get("attributes")
            .and_then(Value::as_object)
            .ok_or(
                QuantinuumAdapterError::UnsupportedResultShape,
            )?;

    // Preferred normalized form.
    if let Some(counts) =
        attributes
            .get("counts")
    {
        return parse_counts_value(
            counts,
            attributes,
        );
    }

    // Some provider-neutral result bridges expose results directly.
    if let Some(results) =
        attributes
            .get("results")
    {
        if let Ok(normalized) =
            parse_counts_value(
                results,
                attributes,
            )
        {
            return Ok(normalized);
        }
    }

    // Some Nexus/Pytket-derived result payloads expose counts as an array of
    // per-circuit objects.
    if let Some(results) =
        attributes
            .get("result")
    {
        if let Ok(normalized) =
            parse_counts_value(
                results,
                attributes,
            )
        {
            return Ok(normalized);
        }
    }

    Err(
        QuantinuumAdapterError::UnsupportedResultShape
    )
}

fn parse_counts_value(
    value: &Value,
    attributes: &Map<String, Value>,
) -> Result<NormalizedCounts, QuantinuumAdapterError> {
    let mut counts =
        Vec::<(String, usize)>::new();

    match value {
        Value::Object(map) => {
            for (bitstring, count) in map {
                let count =
                    count
                        .as_u64()
                        .ok_or(
                            QuantinuumAdapterError::InvalidCounts,
                        )?;

                let count =
                    usize::try_from(count)
                        .map_err(|_| {
                            QuantinuumAdapterError::NumericOverflow
                        })?;

                if count == 0 {
                    continue;
                }

                validate_bitstring(bitstring)?;

                counts.push((
                    bitstring.clone(),
                    count,
                ));

                if counts.len()
                    > QUANTINUUM_MAX_RESULT_OUTCOMES
                {
                    return Err(
                        QuantinuumAdapterError::ResultTooLarge
                    );
                }
            }
        }

        Value::Array(items) => {
            for item in items {
                let object =
                    item
                        .as_object()
                        .ok_or(
                            QuantinuumAdapterError::InvalidCounts,
                        )?;

                let bitstring =
                    object
                        .get("bitstring")
                        .or_else(|| {
                            object.get("state")
                        })
                        .and_then(Value::as_str)
                        .ok_or(
                            QuantinuumAdapterError::InvalidCounts,
                        )?;

                let count =
                    object
                        .get("count")
                        .or_else(|| {
                            object.get("shots")
                        })
                        .and_then(Value::as_u64)
                        .ok_or(
                            QuantinuumAdapterError::InvalidCounts,
                        )?;

                let count =
                    usize::try_from(count)
                        .map_err(|_| {
                            QuantinuumAdapterError::NumericOverflow
                        })?;

                validate_bitstring(bitstring)?;

                counts.push((
                    bitstring.to_owned(),
                    count,
                ));

                if counts.len()
                    > QUANTINUUM_MAX_RESULT_OUTCOMES
                {
                    return Err(
                        QuantinuumAdapterError::ResultTooLarge
                    );
                }
            }
        }

        _ => {
            return Err(
                QuantinuumAdapterError::InvalidCounts
            );
        }
    }

    if counts.is_empty() {
        return Err(
            QuantinuumAdapterError::InvalidCounts
        );
    }

    counts.sort_by(
        |left, right| {
            left.0.cmp(&right.0)
        },
    );

    let total =
        counts
            .iter()
            .try_fold(
                0usize,
                |accumulator, (_, count)| {
                    accumulator.checked_add(*count)
                },
            )
            .ok_or(
                QuantinuumAdapterError::NumericOverflow
            )?;

    let shots =
        attributes
            .get("shots")
            .and_then(Value::as_u64)
            .and_then(|value| {
                usize::try_from(value).ok()
            })
            .unwrap_or(total);

    if shots != total {
        return Err(
            QuantinuumAdapterError::InvalidCounts
        );
    }

    Ok(NormalizedCounts {
        shots,
        counts,
    })
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_target(
    target: &str,
) -> Result<(), QuantinuumAdapterError> {
    if target.is_empty()
        || target.len()
            > QUANTINUUM_MAX_TARGET_LENGTH
    {
        return Err(
            QuantinuumAdapterError::InvalidTarget(
                target.to_owned(),
            ),
        );
    }

    if target
        .chars()
        .any(|character| {
            character.is_control()
                || character.is_whitespace()
        })
    {
        return Err(
            QuantinuumAdapterError::InvalidTarget(
                target.to_owned(),
            ),
        );
    }

    if target.contains('/')
        || target.contains('\\')
        || target.contains('?')
        || target.contains('#')
        || target.contains(':')
    {
        return Err(
            QuantinuumAdapterError::InvalidTarget(
                target.to_owned(),
            ),
        );
    }

    Ok(())
}

fn canonical_backend_id(
    target: &str,
) -> String {
    format!(
        "{}/{}",
        QUANTINUUM_PROVIDER_ID,
        target
    )
}

fn canonical_backend_id_for_target_unchecked(
    target: &str,
) -> String {
    canonical_backend_id(target)
}

fn canonical_request_id(
    request: &ExecutionRequest,
    program: &BackendProgram,
) -> String {
    if let Some(request_id) =
        &request.request_id
    {
        return request_id.clone();
    }

    let mut hash =
        0xcbf29ce484222325u64;

    for byte in program.bytes() {
        hash ^= u64::from(*byte);
        hash =
            hash.wrapping_mul(
                0x100000001b3,
            );
    }

    format!(
        "zamani-quantinuum-{:016x}",
        hash
    )
}

fn request_id_for_job(
    operation: &str,
    job: &BackendJobId,
) -> String {
    format!(
        "zamani-quantinuum-{}-{}",
        operation,
        stable_hash(job.as_str())
    )
}

fn stable_hash(
    value: &str,
) -> String {
    let mut hash =
        0xcbf29ce484222325u64;

    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash =
            hash.wrapping_mul(
                0x100000001b3,
            );
    }

    format!("{:016x}", hash)
}

fn path_escape(
    value: &str,
) -> String {
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

fn validate_bitstring(
    value: &str,
) -> Result<(), QuantinuumAdapterError> {
    if value.is_empty() {
        return Err(
            QuantinuumAdapterError::InvalidCounts
        );
    }

    if value
        .bytes()
        .any(|byte| {
            byte != b'0'
                && byte != b'1'
        })
    {
        return Err(
            QuantinuumAdapterError::InvalidCounts
        );
    }

    Ok(())
}

fn safe_response_message(
    response: &ProviderResponse,
) -> String {
    let length =
        response.body.len()
            .min(
                QUANTINUUM_MAX_ERROR_BODY_BYTES
            );

    let body =
        &response.body[..length];

    String::from_utf8_lossy(body)
        .replace(
            "Authorization",
            "[REDACTED]",
        )
        .replace(
            "authorization",
            "[REDACTED]",
        )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_validation_accepts_known_style_targets() {
        assert!(
            validate_target("Helios-1").is_ok()
        );

        assert!(
            validate_target("H2-1").is_ok()
        );

        assert!(
            validate_target("H1-1").is_ok()
        );
    }

    #[test]
    fn target_validation_rejects_empty_target() {
        assert!(
            validate_target("").is_err()
        );
    }

    #[test]
    fn target_validation_rejects_whitespace() {
        assert!(
            validate_target("Helios 1").is_err()
        );
    }

    #[test]
    fn target_validation_rejects_path_injection() {
        assert!(
            validate_target("../Helios-1").is_err()
        );

        assert!(
            validate_target("Helios-1/foo").is_err()
        );
    }

    #[test]
    fn canonical_backend_id_is_deterministic() {
        assert_eq!(
            canonical_backend_id("Helios-1"),
            "quantinuum/Helios-1"
        );
    }

    #[test]
    fn nexus_states_are_normalized() {
        assert_eq!(
            map_nexus_job_state("SUBMITTED")
                .unwrap(),
            BackendJobState::Created
        );

        assert_eq!(
            map_nexus_job_state("QUEUED")
                .unwrap(),
            BackendJobState::Queued
        );

        assert_eq!(
            map_nexus_job_state("RUNNING")
                .unwrap(),
            BackendJobState::Running
        );

        assert_eq!(
            map_nexus_job_state("COMPLETED")
                .unwrap(),
            BackendJobState::Completed
        );

        assert_eq!(
            map_nexus_job_state("CANCELLED")
                .unwrap(),
            BackendJobState::Cancelled
        );

        assert_eq!(
            map_nexus_job_state("CANCELLING")
                .unwrap(),
            BackendJobState::Cancelling
        );

        assert_eq!(
            map_nexus_job_state("ERROR")
                .unwrap(),
            BackendJobState::Failed
        );

        assert_eq!(
            map_nexus_job_state("RETRYING")
                .unwrap(),
            BackendJobState::Queued
        );

        assert_eq!(
            map_nexus_job_state("TERMINATED")
                .unwrap(),
            BackendJobState::Failed
        );

        assert_eq!(
            map_nexus_job_state("DEPLETED")
                .unwrap(),
            BackendJobState::Failed
        );
    }

    #[test]
    fn unknown_nexus_state_is_not_guessed() {
        assert!(
            map_nexus_job_state(
                "FUTURE_PROVIDER_STATE"
            )
            .is_err()
        );
    }

    #[test]
    fn bitstrings_are_strict_binary() {
        assert!(
            validate_bitstring("0001").is_ok()
        );

        assert!(
            validate_bitstring("101010").is_ok()
        );

        assert!(
            validate_bitstring("").is_err()
        );

        assert!(
            validate_bitstring("0120").is_err()
        );
    }

    #[test]
    fn counts_are_normalized_deterministically() {
        let attributes =
            Map::new();

        let value =
            serde_json::json!({
                "00": 7,
                "01": 2,
                "10": 1
            });

        let result =
            parse_counts_value(
                &value,
                &attributes,
            )
            .unwrap();

        assert_eq!(
            result.shots,
            10
        );

        assert_eq!(
            result.counts,
            vec![
                ("00".to_owned(), 7),
                ("01".to_owned(), 2),
                ("10".to_owned(), 1),
            ]
        );
    }

    #[test]
    fn invalid_count_total_is_rejected() {
        let mut attributes =
            Map::new();

        attributes.insert(
            "shots".to_owned(),
            Value::from(10u64),
        );

        let value =
            serde_json::json!({
                "00": 7,
                "01": 2
            });

        assert!(
            parse_counts_value(
                &value,
                &attributes,
            )
            .is_err()
        );
    }

    #[test]
    fn request_hash_is_deterministic() {
        let first =
            stable_hash("Helios-1");

        let second =
            stable_hash("Helios-1");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn path_escape_does_not_change_safe_identifiers() {
        assert_eq!(
            path_escape("abc-123"),
            "abc-123"
        );

        assert_eq!(
            path_escape("abc_def"),
            "abc_def"
        );
    }

    #[test]
    fn path_escape_encodes_reserved_bytes() {
        assert_eq!(
            path_escape("abc/def"),
            "abc%2Fdef"
        );
    }

    #[test]
    fn retrying_is_non_terminal() {
        assert!(
            !matches!(
                map_nexus_job_state(
                    "RETRYING"
                )
                .unwrap(),
                BackendJobState::Failed
            )
        );
    }
}