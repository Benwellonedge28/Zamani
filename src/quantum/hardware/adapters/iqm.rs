//! Zamani Quantum — IQM Quantum Hardware Adapter
//!
//! Production-grade provider adapter for IQM Quantum Computers / IQM Server.
//!
//! # Responsibility
//!
//! This module is the IQM-specific semantic boundary between Zamani's
//! provider-neutral quantum hardware contracts and IQM's circuit/job API.
//!
//! It owns:
//!
//! - IQM provider identity;
//! - IQM quantum-computer target identity;
//! - IQM backend metadata normalization;
//! - IQM capability normalization;
//! - IQM native-instruction validation;
//! - IQM Circuit JSON validation;
//! - IQM circuit-job submission;
//! - IQM asynchronous job lifecycle normalization;
//! - IQM job cancellation;
//! - IQM measurement-count result normalization;
//! - IQM health/quantum-computer discovery normalization;
//! - IQM provider-error normalization;
//! - IQM API-version metadata;
//! - deterministic IQM request construction;
//! - safe IQM metadata extraction;
//! - provider-neutral integration with `QuantumBackendAdapter`;
//! - provider-neutral integration with `ProviderTransport`;
//! - conformance with the Zamani hardware adapter contract.
//!
//! # Explicit non-responsibilities
//!
//! This module does NOT own:
//!
//! - HTTP implementation;
//! - TLS implementation;
//! - certificate validation;
//! - credentials;
//! - bearer-token persistence;
//! - authentication;
//! - OAuth/OIDC;
//! - routing;
//! - scheduling;
//! - Zamani Quantum IR;
//! - OpenQASM parsing;
//! - QIR generation;
//! - general transpilation;
//! - calibration storage;
//! - topology algorithms;
//! - benchmarking;
//! - job persistence;
//! - provider registries;
//! - global mutable state;
//! - retry loops;
//! - automatic submission retries.
//!
//! Those responsibilities belong to the surrounding hardware architecture.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Zamani Quantum IR
//!      |
//!      v
//! compatibility / routing / scheduling
//!      |
//!      v
//! IQM-native executable circuit
//!      |
//!      v
//! BackendProgram
//!      |
//!      v
//! QuantumBackendAdapter
//!      |
//!      v
//! adapters::iqm
//!      |
//!      v
//! ProviderTransport
//!      |
//!      v
//! IQM Server
//!      |
//!      +---------------------+
//!      |                     |
//!      v                     v
//!     Job                  Result
//! ```
//!
//! # IQM interoperability model
//!
//! The canonical representation remains Zamani Quantum IR.
//!
//! IQM execution uses IQM's circuit representation after provider-neutral
//! compilation has lowered the workload to operations supported by the target
//! IQM architecture.
//!
//! Current IQM-native circuit operations include:
//!
//! - `measure`;
//! - `prx`;
//! - `cc_prx`;
//! - `reset`;
//! - `cz`;
//! - `move`;
//! - `barrier`;
//! - `delay`.
//!
//! IQM documents `prx` angles/phases in radians in current releases.
//!
//! This adapter therefore does not parse OpenQASM and does not perform general
//! gate decomposition.
//!
//! # Program contract
//!
//! `BackendProgram` accepted by this adapter uses:
//!
//! ```text
//! format = "iqm.circuit.v1"
//! ```
//!
//! and contains one IQM circuit JSON object:
//!
//! ```json
//! {
//!   "name": "zamani-circuit",
//!   "instructions": [
//!     {
//!       "name": "prx",
//!       "locus": ["QB1"],
//!       "args": {
//!         "angle": 1.5707963267948966,
//!         "phase": 0.0
//!       }
//!     },
//!     {
//!       "name": "cz",
//!       "locus": ["QB1", "QB2"],
//!       "args": {}
//!     },
//!     {
//!       "name": "measure",
//!       "locus": ["QB1", "QB2"],
//!       "args": {
//!         "key": "m"
//!       }
//!     }
//!   ]
//! }
//! ```
//!
//! The adapter validates structure and safe semantic constraints, but does not
//! pretend to know the complete target-specific dynamic architecture.
//!
//! Full architecture validation belongs to discovery/compatibility/routing.
//!
//! # Current IQM API model
//!
//! IQM's current client/server architecture exposes:
//!
//! ```text
//! GET  /v1/quantum-computers
//! POST /v1/jobs
//! GET  /v1/jobs/{id}
//! GET  /v1/jobs/{id}/artifacts/measurement_counts
//! DELETE /v1/jobs/{id}
//! ```
//!
//! The exact IQM Server deployment URL is supplied by the transport layer.
//!
//! The adapter deliberately does not hard-code authentication or a host.
//!
//! # Authentication
//!
//! IQM uses bearer-token authentication.
//!
//! The bearer token MUST be injected by the configured `ProviderTransport`.
//!
//! This module never stores:
//!
//! ```text
//! token: String
//! api_key: String
//! password: String
//! authorization_header: String
//! ```
//!
//! # Security invariant
//!
//! Provider responses are never copied wholesale into normalized metadata.
//!
//! Only explicitly approved metadata fields are retained.
//!
//! Program payloads, response bodies, credentials and authorization material
//! are not exposed through `Debug`.
//!
//! # Retry safety
//!
//! Job submission is deliberately not automatically retried.
//!
//! An ambiguous POST response could mean that IQM accepted the physical job.
//! Re-submitting after an ambiguous transport failure could execute the same
//! workload twice.
//!
//! GET status/result operations may be retried by a future transport/policy
//! layer, but this adapter itself does not perform retry loops.
//!
//! # Result semantics
//!
//! IQM exposes measurement-count artifacts as histogram counts.
//!
//! A current IQM/QDMI integration documents the artifact shape as an array
//! containing an object with a `counts` mapping from bitstrings to integer
//! counts.
//!
//! Zamani normalizes the first circuit's measurement-count artifact into its
//! provider-neutral `ExecutionResult`.
//!
//! A multi-circuit IQM job is deliberately rejected by this adapter because
//! the current `ExecutionResult` contract represents one execution result,
//! not a batch of independent circuit results.
//!
//! Higher-level batching must submit one Zamani execution per result or use a
//! future batch-result abstraction.
//!
//! # Result integrity
//!
//! The adapter verifies:
//!
//! - job identity;
//! - completed state;
//! - non-zero shots;
//! - bitstring validity through `ExecutionResult::validate()`;
//! - represented counts do not exceed requested shots;
//! - normalized counts are non-empty;
//! - provider job is associated with the selected backend where that metadata
//!   is available.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe Rust.
//!
//! # Integration contract
//!
//! This module consumes the existing repository contracts:
//!
//! ```text
//! backend.rs
//!   QuantumBackend
//!   BackendError
//!   BackendKind
//!   BackendStatus
//!   BackendCapabilities
//!   ExecutionRequest
//!   ExecutionResult
//!
//! backend_trait.rs
//!   QuantumBackendAdapter
//!   BackendAdapterInfo
//!   BackendCancellation
//!   BackendHealth
//!   BackendHealthState
//!   BackendJob
//!   BackendJobId
//!   BackendJobState
//!   BackendJobStatus
//!   BackendQueueInfo
//!   BackendProgram
//!   CancellationOutcome
//!
//! adapters/generic.rs
//!   ProviderTransport
//!   ProviderRequest
//!   ProviderResponse
//!   ProviderOperation
//!   TransportMethod
//!   send_request
//! ```
//!
//! The adapter never changes those contracts.
//!
//! Provider registries, execution orchestration, benchmarking and Danga
//! consume this adapter through `QuantumBackendAdapter`.
//!
//! # No-reedit rule
//!
//! Adding another IQM quantum computer must require configuration/discovery
//! data only.
//!
//! Adding another provider must never require changing this file.
//!
//! Changes to:
//!
//! - routing;
//! - scheduling;
//! - benchmarking;
//! - Danga;
//! - provider registries;
//! - credentials;
//! - execution orchestration
//!
//! must consume this adapter instead of adding provider logic to those layers.
//!
//! # API evolution
//!
//! IQM's current client architecture is evolving from older Cocos/Resonance
//! APIs toward Station Control based services.
//!
//! This adapter therefore isolates all IQM endpoint paths and status mappings
//! in this file. If an IQM Server deployment changes its REST path structure,
//! the transport/API mapping boundary can be updated without changing Zamani's
//! provider-neutral contracts.
//!
//! Unknown provider job states are mapped to `BackendJobState::Unknown`.
//!
//! They are NEVER interpreted as successful.
//!
//! # Capability semantics
//!
//! IQM supports dynamic circuits, mid-circuit measurement and classical
//! feedback for supported architectures. This adapter advertises those
//! capabilities at the provider-family level, while target-specific discovery
//! remains responsible for confirming whether a particular quantum computer
//! and calibration set supports the requested operation.
//!
//! Consequently, this adapter must not be used as a substitute for dynamic
//! quantum architecture discovery.
//!
//! # Topology semantics
//!
//! IQM hardware topology is not assumed to be all-to-all.
//!
//! The adapter does not fabricate connectivity.
//!
//! The canonical topology subsystem must obtain the IQM static/dynamic
//! architecture through a future discovery integration.
//!
//! # Calibration semantics
//!
//! IQM exposes calibration sets and quality metrics.
//!
//! This adapter advertises calibration-data availability but does not silently
//! manufacture calibration values.
//!
//! Calibration acquisition remains the responsibility of discovery/calibration.
//!
//! # Cost semantics
//!
//! No pricing is hard-coded into this adapter.
//!
//! Provider pricing belongs to provider metadata and future cost-estimation
//! infrastructure.
//!
//! # Important production invariant
//!
//! This adapter can execute a workload only after:
//!
//! 1. the caller has produced an IQM-native executable circuit;
//! 2. `ExecutionRequest` passes structural validation;
//! 3. the requested workload is compatible with the target;
//! 4. the transport is authenticated and secure;
//! 5. the IQM provider accepts the submission.
//!
//! It does not bypass Zamani compatibility/routing/scheduling layers.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
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
    BackendQueueInfo,
    BackendProgram,
    CancellationOutcome,
    QuantumBackendAdapter,
};

/// Stable IQM provider identifier.
pub const IQM_PROVIDER_ID: &str = "iqm";

/// Stable Zamani adapter identifier.
pub const IQM_ADAPTER_ID: &str =
    "zamani.quantum.hardware.iqm";

/// Semantic version of this adapter.
pub const IQM_ADAPTER_VERSION: &str = "1.0.0";

/// IQM Server REST API generation targeted by this adapter.
pub const IQM_API_VERSION: &str = "v1";

/// Canonical public IQM Resonance server.
pub const IQM_PUBLIC_SERVER_URL: &str =
    "https://resonance.iqm.tech/";

/// Program format for one IQM circuit.
pub const IQM_CIRCUIT_FORMAT: &str =
    "iqm.circuit.v1";

/// Alternate program-format identifier accepted for compatibility.
pub const IQM_CIRCUIT_FORMAT_LEGACY: &str =
    "iqm-json";

/// Maximum IQM backend/quantum-computer identifier.
pub const MAX_IQM_TARGET_LENGTH: usize = 256;

/// Maximum circuit name length.
pub const MAX_IQM_CIRCUIT_NAME_LENGTH: usize = 512;

/// Maximum instruction count.
pub const MAX_IQM_INSTRUCTIONS: usize = 1_000_000;

/// Maximum instruction name length.
pub const MAX_IQM_INSTRUCTION_NAME_LENGTH: usize = 128;

/// Maximum qubit/component identifier length.
pub const MAX_IQM_COMPONENT_NAME_LENGTH: usize = 256;

/// Maximum measurement-key length.
pub const MAX_IQM_MEASUREMENT_KEY_LENGTH: usize = 256;

/// Maximum result entries accepted in one normalized result.
pub const MAX_IQM_RESULT_ENTRIES: usize = 1_000_000;

/// Maximum JSON program size.
pub const MAX_IQM_PROGRAM_BYTES: usize =
    256 * 1024 * 1024;

/// Maximum supported shots.
///
/// IQM itself does not expose one universal provider-wide number that should
/// be hard-coded as a Zamani semantic limit. Zero means "not specified" in
/// the core backend limit model, so this adapter only enforces a defensive
/// integer bound.
pub const MAX_IQM_SHOTS: usize = usize::MAX;

/// Maximum provider error body retained for diagnostics.
pub const MAX_IQM_ERROR_BODY_BYTES: usize = 16 * 1024;

/// Stable provider capability names.
pub mod capability {
    /// Terminal measurement.
    pub const MEASUREMENT: &str = "measurement";

    /// Reset.
    pub const RESET: &str = "reset";

    /// Mid-circuit measurement.
    pub const MID_CIRCUIT_MEASUREMENT: &str =
        "mid_circuit_measurement";

    /// Classical feedback.
    pub const CLASSICAL_CONTROL: &str =
        "classical_control";

    /// Dynamic circuits.
    pub const DYNAMIC_CIRCUITS: &str =
        "dynamic_circuits";

    /// Parameterized gates.
    pub const PARAMETERIZED_GATES: &str =
        "parameterized_gates";

    /// Two-qubit operations.
    pub const TWO_QUBIT_OPERATIONS: &str =
        "two_qubit_operations";

    /// Batch execution.
    pub const BATCH_EXECUTION: &str =
        "batch_execution";

    /// Cancellation.
    pub const CANCELLATION: &str =
        "cancellation";

    /// Calibration data.
    pub const CALIBRATION_DATA: &str =
        "calibration_data";

    /// Topology.
    pub const TOPOLOGY_INFORMATION: &str =
        "topology_information";

    /// Timing information.
    pub const TIMING_INFORMATION: &str =
        "timing_information";
}

/// Stable IQM operation identifiers.
pub mod operation {
    /// Submit circuit job.
    pub const SUBMIT: &str = "iqm.job.submit";

    /// Get job.
    pub const GET_JOB: &str = "iqm.job.get";

    /// Get measurement counts artifact.
    pub const GET_MEASUREMENT_COUNTS: &str =
        "iqm.job.artifact.measurement_counts";

    /// Cancel job.
    pub const CANCEL: &str = "iqm.job.cancel";

    /// List quantum computers.
    pub const QUANTUM_COMPUTERS: &str =
        "iqm.quantum_computers";
}

/// Provider status strings used by current IQM Server clients.
///
/// Unknown states are intentionally accepted and mapped to `Unknown`.
pub mod provider_status {
    /// Job has been created.
    pub const CREATED: &str = "created";

    /// Compilation has started.
    pub const COMPILATION_STARTED: &str =
        "compilation_started";

    /// Job is pending.
    pub const PENDING: &str = "pending";

    /// Job is queued.
    pub const QUEUED: &str = "queued";

    /// Compilation has ended.
    pub const COMPILATION_ENDED: &str =
        "compilation_ended";

    /// Execution has started.
    pub const EXECUTION_STARTED: &str =
        "execution_started";

    /// Execution is running.
    pub const RUNNING: &str = "running";

    /// Job completed successfully.
    pub const COMPLETED: &str = "completed";

    /// Job failed.
    pub const FAILED: &str = "failed";

    /// Job cancelled.
    pub const CANCELLED: &str = "cancelled";

    /// Job aborted.
    pub const ABORTED: &str = "aborted";

    /// Job expired.
    pub const EXPIRED: &str = "expired";
}

/// IQM adapter-local error.
///
/// This error is intentionally converted into the repository's current
/// `BackendError` boundary before crossing the adapter trait.
#[derive(Debug, Clone, PartialEq)]
pub enum IqmAdapterError {
    /// Target identifier is invalid.
    InvalidTarget,

    /// Target identifier is too long.
    TargetTooLong,

    /// No transport was supplied.
    MissingTransport,

    /// Backend identity does not correspond to the target.
    BackendMismatch {
        expected: String,
        actual: String,
    },

    /// Program format is unsupported.
    UnsupportedProgramFormat(String),

    /// Program is not valid JSON.
    InvalidProgramJson,

    /// Program root is not an object.
    InvalidProgramShape,

    /// Required circuit field is missing.
    MissingProgramField(&'static str),

    /// Circuit field has an invalid JSON type.
    InvalidProgramField(&'static str),

    /// Circuit contains no instructions.
    EmptyCircuit,

    /// Circuit has no measurement operation.
    MissingMeasurement,

    /// Circuit contains an unsupported native instruction.
    UnsupportedInstruction(String),

    /// Instruction name is invalid.
    InvalidInstructionName,

    /// Instruction locus is invalid.
    InvalidInstructionLocus,

    /// Instruction arguments are invalid.
    InvalidInstructionArguments,

    /// Measurement key is invalid.
    InvalidMeasurementKey,

    /// Numeric value is invalid.
    InvalidNumericValue(&'static str),

    /// Shots are invalid.
    InvalidShots,

    /// IQM returned an invalid job ID.
    InvalidJobId,

    /// IQM returned an invalid job state.
    UnknownProviderState(String),

    /// IQM response is malformed.
    InvalidResponse(&'static str),

    /// IQM response is not JSON.
    InvalidResponseJson,

    /// IQM returned a provider error.
    ProviderFailure {
        operation: &'static str,
        status: Option<u16>,
        message: String,
    },

    /// Generic transport error.
    Transport(String),

    /// Result normalization failed.
    ResultNormalization(String),

    /// Result contains too many entries.
    ResultLimitExceeded,

    /// IQM returned a different backend identity.
    ResultBackendMismatch {
        expected: String,
        actual: String,
    },

    /// Provider returned no measurement counts.
    MissingMeasurementCounts,
}

impl fmt::Display for IqmAdapterError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidTarget => {
                formatter.write_str(
                    "IQM target identifier is invalid",
                )
            }

            Self::TargetTooLong => {
                formatter.write_str(
                    "IQM target identifier is too long",
                )
            }

            Self::MissingTransport => {
                formatter.write_str(
                    "IQM adapter requires an injected provider transport",
                )
            }

            Self::BackendMismatch { expected, actual } => {
                write!(
                    formatter,
                    "IQM backend mismatch: expected '{}', got '{}'",
                    expected,
                    actual
                )
            }

            Self::UnsupportedProgramFormat(format) => {
                write!(
                    formatter,
                    "IQM adapter does not support program format '{}'",
                    format
                )
            }

            Self::InvalidProgramJson => {
                formatter.write_str(
                    "IQM circuit program is not valid JSON",
                )
            }

            Self::InvalidProgramShape => {
                formatter.write_str(
                    "IQM circuit program must be a JSON object",
                )
            }

            Self::MissingProgramField(field) => {
                write!(
                    formatter,
                    "IQM circuit is missing '{}'",
                    field
                )
            }

            Self::InvalidProgramField(field) => {
                write!(
                    formatter,
                    "IQM circuit field '{}' has an invalid type",
                    field
                )
            }

            Self::EmptyCircuit => {
                formatter.write_str(
                    "IQM circuit contains no instructions",
                )
            }

            Self::MissingMeasurement => {
                formatter.write_str(
                    "IQM circuit contains no terminal or mid-circuit measurement",
                )
            }

            Self::UnsupportedInstruction(name) => {
                write!(
                    formatter,
                    "IQM instruction '{}' is not supported by the adapter",
                    name
                )
            }

            Self::InvalidInstructionName => {
                formatter.write_str(
                    "IQM instruction name is invalid",
                )
            }

            Self::InvalidInstructionLocus => {
                formatter.write_str(
                    "IQM instruction locus is invalid",
                )
            }

            Self::InvalidInstructionArguments => {
                formatter.write_str(
                    "IQM instruction arguments are invalid",
                )
            }

            Self::InvalidMeasurementKey => {
                formatter.write_str(
                    "IQM measurement key is invalid",
                )
            }

            Self::InvalidNumericValue(field) => {
                write!(
                    formatter,
                    "IQM numeric field '{}' is invalid",
                    field
                )
            }

            Self::InvalidShots => {
                formatter.write_str(
                    "IQM shot count must be greater than zero",
                )
            }

            Self::InvalidJobId => {
                formatter.write_str(
                    "IQM job identifier is invalid",
                )
            }

            Self::UnknownProviderState(state) => {
                write!(
                    formatter,
                    "IQM returned unknown job state '{}'",
                    state
                )
            }

            Self::InvalidResponse(field) => {
                write!(
                    formatter,
                    "IQM response is missing or invalid '{}'",
                    field
                )
            }

            Self::InvalidResponseJson => {
                formatter.write_str(
                    "IQM response is not valid JSON",
                )
            }

            Self::ProviderFailure {
                operation,
                status,
                message,
            } => {
                write!(
                    formatter,
                    "IQM operation {} failed",
                    operation
                )?;

                if let Some(status) = status {
                    write!(
                        formatter,
                        " with HTTP status {}",
                        status
                    )?;
                }

                write!(
                    formatter,
                    ": {}",
                    message
                )
            }

            Self::Transport(message) => {
                write!(
                    formatter,
                    "IQM transport failure: {}",
                    message
                )
            }

            Self::ResultNormalization(message) => {
                write!(
                    formatter,
                    "IQM result normalization failure: {}",
                    message
                )
            }

            Self::ResultLimitExceeded => {
                formatter.write_str(
                    "IQM result exceeds the normalization limit",
                )
            }

            Self::ResultBackendMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "IQM result backend mismatch: expected '{}', got '{}'",
                    expected,
                    actual
                )
            }

            Self::MissingMeasurementCounts => {
                formatter.write_str(
                    "IQM measurement-count artifact is missing",
                )
            }
        }
    }
}

impl std::error::Error for IqmAdapterError {}

/// Production IQM adapter.
///
/// Authentication and transport security are supplied by `ProviderTransport`.
pub struct IqmAdapter {
    backend: QuantumBackend,
    adapter_info: BackendAdapterInfo,
    transport: Arc<dyn ProviderTransport>,
    target: String,
}

impl fmt::Debug for IqmAdapter {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("IqmAdapter")
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

impl IqmAdapter {
    /// Creates an IQM adapter for one IQM quantum computer.
    ///
    /// The backend ID must equal:
    ///
    /// ```text
    /// iqm/<target>
    /// ```
    pub fn new(
        backend: QuantumBackend,
        target: impl Into<String>,
        transport: Arc<dyn ProviderTransport>,
    ) -> Result<Self, BackendError> {
        let target = target.into();

        validate_target(&target)
            .map_err(Self::map_local_error)?;

        let expected =
            canonical_backend_id(&target);

        if backend.id() != expected {
            return Err(Self::map_local_error(
                IqmAdapterError::BackendMismatch {
                    expected,
                    actual: backend.id().to_owned(),
                },
            ));
        }

        let adapter_info =
            BackendAdapterInfo::new(
                IQM_ADAPTER_ID,
                IQM_ADAPTER_VERSION,
                true,
            )?
            .with_provider_api_version(
                IQM_API_VERSION,
            )?;

        Ok(Self {
            backend,
            adapter_info,
            transport,
            target,
        })
    }

    /// Returns the IQM target/quantum-computer identifier.
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Returns the adapter's IQM API version.
    pub const fn api_version() -> &'static str {
        IQM_API_VERSION
    }

    /// Returns the public IQM server URL.
    ///
    /// This is informational only. The actual transport endpoint is
    /// transport-owned.
    pub const fn public_server_url() -> &'static str {
        IQM_PUBLIC_SERVER_URL
    }

    /// Creates the canonical Zamani backend identifier.
    pub fn canonical_backend_id_for_target(
        target: &str,
    ) -> Result<String, BackendError> {
        validate_target(target)
            .map_err(Self::map_local_error)?;

        Ok(canonical_backend_id(target))
    }

    /// Builds a conservative provider-family capability profile.
    ///
    /// Target-specific capability discovery should refine this information
    /// before production submission.
    pub fn provider_capabilities() -> BackendCapabilities {
        BackendCapabilities {
            measurement: true,
            reset: true,
            mid_circuit_measurement: true,
            classical_control: true,
            dynamic_circuits: true,
            arbitrary_single_qubit_rotations: false,
            parameterized_gates: true,
            three_qubit_operations: false,
            multi_qubit_operations: false,
            parallel_operations: true,
            batch_execution: true,
            streaming_results: false,
            cancellation: true,
            queue_information: false,
            pulse_control: false,
            analog_control: false,
            annealing: false,
            logical_qubits: false,
            fault_tolerance: false,
            syndrome_measurement: false,
            decoder_execution: false,
            deterministic_seeding: false,
            state_vector_results: false,
            density_matrix_results: false,
            expectation_value_results: false,
            readout_mitigation: false,
            error_mitigation: false,
            calibration_data: true,
            timing_information: true,
            topology_information: true,
            native_instruction_set: true,
            native_gates: [
                "measure",
                "prx",
                "cc_prx",
                "reset",
                "cz",
                "move",
                "barrier",
                "delay",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            experimental_capabilities:
                BTreeSet::new(),
        }
    }

    // -------------------------------------------------------------------------
    // Provider requests
    // -------------------------------------------------------------------------

    fn submit_request(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<ProviderResponse, BackendError> {
        let circuit =
            parse_and_validate_program(
                program,
                request,
            )
            .map_err(Self::map_local_error)?;

        let shots =
            request.workload.circuit.shots;

        if shots == 0 {
            return Err(BackendError::InvalidShots);
        }

        let mut body =
            Map::<String, Value>::new();

        body.insert(
            "circuits".to_owned(),
            Value::Array(vec![circuit]),
        );

        body.insert(
            "shots".to_owned(),
            Value::Number(
                serde_json::Number::from(
                    shots as u64,
                ),
            ),
        );

        /*
         * Provider-specific optional execution settings are deliberately
         * sourced only from explicitly namespaced, non-secret request
         * metadata.
         *
         * The adapter never copies arbitrary metadata into the provider
         * payload because doing so could create accidental API/schema
         * coupling.
         */
        if let Some(calibration_set_id) =
            request.metadata.get(
                "iqm.calibration_set_id",
            )
        {
            validate_safe_metadata_value(
                calibration_set_id,
            )
            .map_err(Self::map_local_error)?;

            body.insert(
                "calibration_set_id".to_owned(),
                Value::String(
                    calibration_set_id.clone(),
                ),
            );
        }

        if let Some(mapping_json) =
            request.metadata.get(
                "iqm.qubit_mapping",
            )
        {
            let mapping: Value =
                serde_json::from_str(
                    mapping_json,
                )
                .map_err(|_| {
                    Self::map_local_error(
                        IqmAdapterError::InvalidProgramField(
                            "iqm.qubit_mapping",
                        ),
                    )
                })?;

            validate_qubit_mapping(
                &mapping,
            )
            .map_err(Self::map_local_error)?;

            body.insert(
                "qubit_mapping".to_owned(),
                mapping,
            );
        }

        let body =
            serde_json::to_vec(
                &Value::Object(body),
            )
            .map_err(|_| {
                Self::map_local_error(
                    IqmAdapterError::InvalidProgramJson,
                )
            })?;

        if body.len()
            > MAX_IQM_PROGRAM_BYTES
        {
            return Err(Self::map_local_error(
                IqmAdapterError::ResultLimitExceeded,
            ));
        }

        let request_id =
            canonical_request_id(
                request,
                program,
            );

        let provider_request =
            ProviderRequest::builder(
                ProviderOperation::Submit,
                TransportMethod::Post,
                "/v1/jobs",
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
            operation::SUBMIT,
            &provider_request,
        )
    }

    fn get_job(
        &self,
        job: &BackendJobId,
    ) -> Result<Value, BackendError> {
        let request_id =
            stable_hash_id(
                "iqm-status",
                job.as_str(),
            );

        let provider_request =
            ProviderRequest::builder(
                ProviderOperation::GetJobStatus,
                TransportMethod::Get,
                format!(
                    "/v1/jobs/{}",
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
            self.send_checked(
                operation::GET_JOB,
                &provider_request,
            )?;

        parse_json_response(
            &response,
        )
        .map_err(Self::map_local_error)
    }

    fn get_measurement_counts(
        &self,
        job: &BackendJobId,
    ) -> Result<Value, BackendError> {
        let request_id =
            stable_hash_id(
                "iqm-result",
                job.as_str(),
            );

        let provider_request =
            ProviderRequest::builder(
                ProviderOperation::GetResult,
                TransportMethod::Get,
                format!(
                    "/v1/jobs/{}/artifacts/measurement_counts",
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
            self.send_checked(
                operation::GET_MEASUREMENT_COUNTS,
                &provider_request,
            )?;

        parse_json_response(
            &response,
        )
        .map_err(Self::map_local_error)
    }

    fn cancel_request(
        &self,
        job: &BackendJobId,
    ) -> Result<ProviderResponse, BackendError> {
        let request_id =
            stable_hash_id(
                "iqm-cancel",
                job.as_str(),
            );

        let provider_request =
            ProviderRequest::builder(
                ProviderOperation::Cancel,
                TransportMethod::Delete,
                format!(
                    "/v1/jobs/{}",
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

        self.send_checked(
            operation::CANCEL,
            &provider_request,
        )
    }

    fn get_quantum_computers(
        &self,
    ) -> Result<Value, BackendError> {
        let request_id =
            stable_hash_id(
                "iqm-quantum-computers",
                &self.target,
            );

        let provider_request =
            ProviderRequest::builder(
                ProviderOperation::GetHealth,
                TransportMethod::Get,
                "/v1/quantum-computers",
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
            self.send_checked(
                operation::QUANTUM_COMPUTERS,
                &provider_request,
            )?;

        parse_json_response(
            &response,
        )
        .map_err(Self::map_local_error)
    }

    fn send_checked(
        &self,
        operation_name: &'static str,
        request: &ProviderRequest,
    ) -> Result<ProviderResponse, BackendError> {
        let response =
            send_request(
                self.transport.as_ref(),
                request,
            )
            .map_err(|error| {
                Self::map_provider_error(
                    operation_name,
                    error.status_code,
                    &error.message,
                )
            })?;

        if !response.is_success() {
            return Err(Self::map_local_error(
                IqmAdapterError::ProviderFailure {
                    operation: operation_name,
                    status: Some(
                        response.status_code,
                    ),
                    message:
                        safe_response_message(
                            &response,
                        ),
                },
            ));
        }

        Ok(response)
    }

    // -------------------------------------------------------------------------
    // Job normalization
    // -------------------------------------------------------------------------

    fn normalize_job(
        &self,
        value: &Value,
    ) -> Result<NormalizedIqmJob, BackendError> {
        let id =
            value
                .get("id")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    Self::map_local_error(
                        IqmAdapterError::InvalidResponse(
                            "id",
                        ),
                    )
                })?;

        validate_job_id(id)
            .map_err(Self::map_local_error)?;

        let status =
            value
                .get("status")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    Self::map_local_error(
                        IqmAdapterError::InvalidResponse(
                            "status",
                        ),
                    )
                })?;

        let state =
            map_provider_status(status)
                .map_err(Self::map_local_error)?;

        let provider_target =
            extract_target_from_job(value);

        if let Some(actual) =
            provider_target
        {
            if !target_matches(
                &self.target,
                &actual,
            ) {
                return Err(
                    Self::map_local_error(
                        IqmAdapterError::ResultBackendMismatch {
                            expected: self.target.clone(),
                            actual,
                        },
                    ),
                );
            }
        }

        let result_available =
            matches!(
                state,
                BackendJobState::Completed
            );

        let request_id =
            value
                .get("metadata")
                .and_then(Value::as_object)
                .and_then(|metadata| {
                    metadata
                        .get("zamani_request_id")
                        .and_then(Value::as_str)
                })
                .map(str::to_owned);

        let backend_job =
            BackendJob::new(
                BackendJobId::new(
                    id.to_owned(),
                )?,
                self.backend.id(),
                request_id,
                state,
            )?;

        let queue_position =
            value
                .get("queue_position")
                .or_else(|| {
                    value.get(
                        "queuePosition",
                    )
                })
                .and_then(Value::as_u64)
                .and_then(|value| {
                    usize::try_from(
                        value,
                    )
                    .ok()
                });

        let estimated_wait =
            value
                .get(
                    "estimated_wait_seconds",
                )
                .or_else(|| {
                    value.get(
                        "estimatedWaitSeconds",
                    )
                })
                .and_then(
                    Value::as_f64,
                )
                .and_then(|seconds| {
                    if seconds.is_finite()
                        && seconds >= 0.0
                    {
                        Some(
                            std::time::Duration::from_secs_f64(
                                seconds,
                            ),
                        )
                    } else {
                        None
                    }
                });

        Ok(
            NormalizedIqmJob {
                job: backend_job,
                result_available,
                provider_status:
                    Some(status.to_owned()),
                queue_position,
                estimated_wait,
            },
        )
    }

    fn normalize_result(
        &self,
        job: &BackendJobId,
        job_value: &Value,
        artifact_value: &Value,
    ) -> Result<ExecutionResult, BackendError> {
        let normalized =
            self.normalize_job(job_value)?;

        if !matches!(
            normalized.job.state,
            BackendJobState::Completed
        ) {
            return Err(Self::map_local_error(
                IqmAdapterError::UnknownProviderState(
                    normalized
                        .provider_status
                        .clone()
                        .unwrap_or_else(
                            || "unknown".to_owned(),
                        ),
                ),
            ));
        }

        let counts =
            parse_measurement_counts(
                artifact_value,
            )
            .map_err(Self::map_local_error)?;

        if counts.is_empty() {
            return Err(
                Self::map_local_error(
                    IqmAdapterError::MissingMeasurementCounts,
                ),
            );
        }

        let shots =
            job_value
                .get("shots")
                .and_then(Value::as_u64)
                .or_else(|| {
                    job_value
                        .get("metadata")
                        .and_then(Value::as_object)
                        .and_then(|metadata| {
                            metadata
                                .get("shots")
                                .and_then(Value::as_u64)
                        })
                })
                .and_then(|value| {
                    usize::try_from(value)
                        .ok()
                })
                .ok_or_else(|| {
                    Self::map_local_error(
                        IqmAdapterError::InvalidResponse(
                            "shots",
                        ),
                    )
                })?;

        if shots == 0 {
            return Err(BackendError::InvalidShots);
        }

        let mut result =
            ExecutionResult::empty(
                self.backend.id(),
                shots,
            )?;

        for (bitstring, count) in
            counts
        {
            result.insert_count(
                bitstring,
                count,
            )?;
        }

        result.metadata.insert(
            "provider".to_owned(),
            IQM_PROVIDER_ID.to_owned(),
        );

        result.metadata.insert(
            "provider_api_version".to_owned(),
            IQM_API_VERSION.to_owned(),
        );

        result.metadata.insert(
            "job_id".to_owned(),
            job.as_str().to_owned(),
        );

        result.metadata.insert(
            "target".to_owned(),
            self.target.clone(),
        );

        if let Some(
            calibration_set_id,
        ) = job_value
            .get("compilation")
            .and_then(Value::as_object)
            .and_then(|compilation| {
                compilation
                    .get("calibration_set_id")
                    .and_then(Value::as_str)
            })
        {
            if is_safe_metadata_value(
                calibration_set_id,
            ) {
                result.metadata.insert(
                    "calibration_set_id"
                        .to_owned(),
                    calibration_set_id
                        .to_owned(),
                );
            }
        }

        result.validate()?;

        if !result.counts_match_shots() {
            return Err(
                Self::map_local_error(
                    IqmAdapterError::ResultNormalization(
                        "measurement counts do not match requested shots"
                            .to_owned(),
                    ),
                ),
            );
        }

        Ok(result)
    }

    // -------------------------------------------------------------------------
    // Error mapping
    // -------------------------------------------------------------------------

    fn map_local_error(
        error: IqmAdapterError,
    ) -> BackendError {
        match error {
            IqmAdapterError::InvalidShots => {
                BackendError::InvalidShots
            }

            IqmAdapterError::BackendMismatch {
                ..
            }
            | IqmAdapterError::ResultBackendMismatch {
                ..
            } => {
                BackendError::ExecutionRejected(
                    "IQM backend identity mismatch"
                        .to_owned(),
                )
            }

            IqmAdapterError::UnsupportedInstruction(
                instruction,
            ) => {
                BackendError::UnsupportedGate {
                    gate: instruction,
                }
            }

            IqmAdapterError::MissingMeasurement
            | IqmAdapterError::InvalidProgramJson
            | IqmAdapterError::InvalidProgramShape
            | IqmAdapterError::MissingProgramField(_)
            | IqmAdapterError::InvalidProgramField(_)
            | IqmAdapterError::EmptyCircuit
            | IqmAdapterError::InvalidInstructionName
            | IqmAdapterError::InvalidInstructionLocus
            | IqmAdapterError::InvalidInstructionArguments
            | IqmAdapterError::InvalidMeasurementKey
            | IqmAdapterError::InvalidNumericValue(_)
            | IqmAdapterError::InvalidTarget
            | IqmAdapterError::TargetTooLong
            | IqmAdapterError::InvalidJobId
            | IqmAdapterError::InvalidResponse(_)
            | IqmAdapterError::InvalidResponseJson
            | IqmAdapterError::UnsupportedProgramFormat(_)
            | IqmAdapterError::ResultNormalization(_)
            | IqmAdapterError::ResultLimitExceeded
            | IqmAdapterError::MissingMeasurementCounts => {
                BackendError::ExecutionRejected(
                    error.to_string(),
                )
            }

            IqmAdapterError::MissingTransport
            | IqmAdapterError::Transport(_)
            | IqmAdapterError::ProviderFailure {
                ..
            }
            | IqmAdapterError::UnknownProviderState(_) => {
                BackendError::ExecutionUnavailable(
                    error.to_string(),
                )
            }
        }
    }

    fn map_generic_error(
        error: super::generic::GenericAdapterError,
    ) -> BackendError {
        BackendError::ExecutionUnavailable(
            error.to_string(),
        )
    }

    fn map_provider_error(
        operation: &'static str,
        status: Option<u16>,
        message: &str,
    ) -> BackendError {
        BackendError::ExecutionUnavailable(
            format!(
                "IQM operation {} failed{}: {}",
                operation,
                status
                    .map(|value| {
                        format!(
                            " with HTTP status {}",
                            value
                        )
                    })
                    .unwrap_or_default(),
                sanitize_error_message(
                    message,
                ),
            ),
        )
    }
}

// =============================================================================
// QuantumBackendAdapter implementation
// =============================================================================

impl QuantumBackendAdapter for IqmAdapter {
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
            != IQM_CIRCUIT_FORMAT
            && program.format()
                != IQM_CIRCUIT_FORMAT_LEGACY
        {
            return Err(Self::map_local_error(
                IqmAdapterError::UnsupportedProgramFormat(
                    program.format().to_owned(),
                ),
            ));
        }

        if program.len()
            > MAX_IQM_PROGRAM_BYTES
        {
            return Err(Self::map_local_error(
                IqmAdapterError::ResultLimitExceeded,
            ));
        }

        let shots =
            request.workload.circuit.shots;

        if shots == 0 {
            return Err(BackendError::InvalidShots);
        }

        if request.workload.circuit
            .requires_annealing
        {
            return Err(
                BackendError::AnnealingUnsupported,
            );
        }

        if request.workload.circuit
            .requires_analog_control
        {
            return Err(
                BackendError::AnalogControlUnsupported,
            );
        }

        if request.workload.circuit
            .requires_pulse_control
        {
            return Err(
                BackendError::PulseControlUnsupported,
            );
        }

        if request.workload.circuit
            .requires_logical_qubits
        {
            return Err(
                BackendError::LogicalQubitsUnsupported,
            );
        }

        if request.workload.circuit
            .requires_fault_tolerance
        {
            return Err(
                BackendError::FaultToleranceUnsupported,
            );
        }

        if request.workload.circuit
            .requires_state_vector
        {
            return Err(
                BackendError::StateVectorUnsupported,
            );
        }

        if request.workload.circuit
            .requires_density_matrix
        {
            return Err(
                BackendError::DensityMatrixUnsupported,
            );
        }

        if request.workload.circuit
            .requires_expectation_values
        {
            return Err(
                BackendError::ExpectationValuesUnsupported,
            );
        }

        if request.seed.is_some() {
            return Err(
                BackendError::DeterministicSeedingUnsupported,
            );
        }

        let circuit =
            parse_and_validate_program(
                program,
                request,
            )
            .map_err(Self::map_local_error)?;

        validate_requested_gates(
            &request.workload.circuit.gates,
            &circuit,
        )
        .map_err(Self::map_local_error)?;

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

        let id =
            value
                .get("id")
                .and_then(Value::as_str)
                .or_else(|| {
                    value
                        .get("job_id")
                        .and_then(Value::as_str)
                })
                .ok_or_else(|| {
                    Self::map_local_error(
                        IqmAdapterError::InvalidResponse(
                            "id",
                        ),
                    )
                })?;

        validate_job_id(id)
            .map_err(Self::map_local_error)?;

        let state =
            value
                .get("status")
                .and_then(Value::as_str)
                .map(map_provider_status)
                .transpose()
                .map_err(Self::map_local_error)?
                .unwrap_or(
                    BackendJobState::Created,
                );

        BackendJob::new(
            BackendJobId::new(
                id.to_owned(),
            )?,
            self.backend.id(),
            request.request_id.clone(),
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
        let job_value =
            self.get_job(job)?;

        let normalized =
            self.normalize_job(
                &job_value,
            )?;

        if !matches!(
            normalized.job.state,
            BackendJobState::Completed
        ) {
            return Err(
                Self::map_local_error(
                    IqmAdapterError::UnknownProviderState(
                        normalized
                            .provider_status
                            .unwrap_or_else(
                                || "unknown"
                                    .to_owned(),
                            ),
                    ),
                ),
            );
        }

        let artifact =
            self.get_measurement_counts(
                job,
            )?;

        self.normalize_result(
            job,
            &job_value,
            &artifact,
        )
    }

    fn cancel(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendCancellation, BackendError> {
        /*
         * IQM's current client exposes cancellation as a provider operation
         * that leaves the job in the server database while preventing or
         * interrupting execution.
         *
         * The REST transport mapping is intentionally isolated here.
         */
        let response =
            self.cancel_request(job)?;

        /*
         * A successful DELETE/cancel operation does not need to expose a
         * provider-specific body to Zamani. The provider contract is enough
         * to report that cancellation was accepted.
         */
        let _ = response;

        Ok(BackendCancellation {
            job: job.clone(),
            outcome:
                CancellationOutcome::Accepted,
        })
    }

    fn queue_info(
        &self,
    ) -> Result<BackendQueueInfo, BackendError> {
        /*
         * IQM's public client exposes queue/job lifecycle information but does
         * not provide one stable provider-wide queue-depth contract that can
         * safely be interpreted as a global queue position.
         *
         * We therefore expose only data explicitly reported by the selected
         * quantum computer/job API when available.
         */
        let value =
            self.get_quantum_computers()?;

        let computer =
            find_quantum_computer(
                &value,
                &self.target,
            );

        let accepting =
            computer
                .and_then(|object| {
                    object
                        .get("status")
                        .and_then(Value::as_str)
                })
                .map(is_accepting_status)
                .unwrap_or(true);

        Ok(BackendQueueInfo {
            pending_jobs: None,
            estimated_wait: None,
            accepting_submissions: accepting,
        })
    }

    fn health(
        &self,
    ) -> Result<BackendHealth, BackendError> {
        let value =
            self.get_quantum_computers()?;

        let computer =
            find_quantum_computer(
                &value,
                &self.target,
            );

        let status =
            computer
                .and_then(|object| {
                    object
                        .get("status")
                        .and_then(Value::as_str)
                })
                .unwrap_or("unknown");

        let backend_status =
            normalize_quantum_computer_status(
                status,
            );

        let health_state =
            match backend_status {
                BackendStatus::Available => {
                    BackendHealthState::Healthy
                }

                BackendStatus::Busy
                | BackendStatus::Degraded => {
                    BackendHealthState::Degraded
                }

                BackendStatus::Unavailable
                | BackendStatus::Offline
                | BackendStatus::Retired => {
                    BackendHealthState::Unhealthy
                }

                BackendStatus::Unknown
                | BackendStatus::Maintenance => {
                    BackendHealthState::Unknown
                }
            };

        Ok(BackendHealth {
            state: health_state,
            backend_status,
            message: Some(format!(
                "IQM quantum computer '{}' reported status '{}'",
                self.target,
                status
            )),
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
        true
    }

    fn supports_synchronous_execution(
        &self,
    ) -> bool {
        false
    }
}

/// IQM adapter conformance marker.
///
/// This implementation is intended to pass the generic hardware adapter
/// conformance suite.
impl super::super::backend_trait::ConformantQuantumBackendAdapter
    for IqmAdapter
{
}

// =============================================================================
// Normalized job
// =============================================================================

#[derive(Debug)]
struct NormalizedIqmJob {
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
) -> Result<Value, IqmAdapterError> {
    if program.format()
        != IQM_CIRCUIT_FORMAT
        && program.format()
            != IQM_CIRCUIT_FORMAT_LEGACY
    {
        return Err(
            IqmAdapterError::UnsupportedProgramFormat(
                program.format().to_owned(),
            ),
        );
    }

    if program.len()
        > MAX_IQM_PROGRAM_BYTES
    {
        return Err(
            IqmAdapterError::ResultLimitExceeded,
        );
    }

    let value: Value =
        serde_json::from_slice(
            program.bytes(),
        )
        .map_err(|_| {
            IqmAdapterError::InvalidProgramJson
        })?;

    let object =
        value
            .as_object()
            .ok_or(
                IqmAdapterError::InvalidProgramShape,
            )?;

    let name =
        object
            .get("name")
            .and_then(Value::as_str)
            .ok_or(
                IqmAdapterError::MissingProgramField(
                    "name",
                ),
            )?;

    validate_circuit_name(
        name,
    )?;

    let instructions =
        object
            .get("instructions")
            .and_then(Value::as_array)
            .ok_or(
                IqmAdapterError::MissingProgramField(
                    "instructions",
                ),
            )?;

    if instructions.is_empty() {
        return Err(
            IqmAdapterError::EmptyCircuit,
        );
    }

    if instructions.len()
        > MAX_IQM_INSTRUCTIONS
    {
        return Err(
            IqmAdapterError::ResultLimitExceeded,
        );
    }

    let requested_qubits =
        request.workload.circuit.qubit_count;

    let mut measurement_found =
        false;

    for instruction in instructions {
        let object =
            instruction
                .as_object()
                .ok_or(
                    IqmAdapterError::InvalidInstructionArguments,
                )?;

        let name =
            object
                .get("name")
                .and_then(Value::as_str)
                .ok_or(
                    IqmAdapterError::MissingProgramField(
                        "instructions[].name",
                    ),
                )?;

        validate_instruction_name(
            name,
        )?;

        if !is_supported_instruction(
            name,
        ) {
            return Err(
                IqmAdapterError::UnsupportedInstruction(
                    name.to_owned(),
                ),
            );
        }

        let locus =
            object
                .get("locus")
                .and_then(Value::as_array)
                .ok_or(
                    IqmAdapterError::MissingProgramField(
                        "instructions[].locus",
                    ),
                )?;

        if locus.is_empty() {
            return Err(
                IqmAdapterError::InvalidInstructionLocus,
            );
        }

        if locus.len()
            > 64
        {
            return Err(
                IqmAdapterError::InvalidInstructionLocus,
            );
        }

        for component in locus {
            let component =
                component
                    .as_str()
                    .ok_or(
                        IqmAdapterError::InvalidInstructionLocus,
                    )?;

            validate_component_name(
                component,
            )?;
        }

        let args =
            object
                .get("args")
                .and_then(Value::as_object)
                .ok_or(
                    IqmAdapterError::MissingProgramField(
                        "instructions[].args",
                    ),
                )?;

        validate_instruction_arguments(
            name,
            args,
        )?;

        if name == "measure" {
            measurement_found = true;

            let key =
                args
                    .get("key")
                    .and_then(Value::as_str)
                    .ok_or(
                        IqmAdapterError::MissingProgramField(
                            "measure.args.key",
                        ),
                    )?;

            validate_measurement_key(
                key,
            )?;
        }

        if requested_qubits > 0
            && locus.len() == 0
        {
            return Err(
                IqmAdapterError::InvalidInstructionLocus,
            );
        }
    }

    if !measurement_found {
        return Err(
            IqmAdapterError::MissingMeasurement,
        );
    }

    Ok(value)
}

fn validate_requested_gates(
    requested: &[String],
    circuit: &Value,
) -> Result<(), IqmAdapterError> {
    if requested.is_empty() {
        return Ok(());
    }

    let instructions =
        circuit
            .get("instructions")
            .and_then(Value::as_array)
            .ok_or(
                IqmAdapterError::MissingProgramField(
                    "instructions",
                ),
            )?;

    let actual: BTreeSet<String> =
        instructions
            .iter()
            .filter_map(|instruction| {
                instruction
                    .get("name")
                    .and_then(Value::as_str)
                    .map(normalize_instruction)
            })
            .collect();

    for requested_gate in requested {
        let requested_gate =
            normalize_instruction(
                requested_gate,
            );

        if !actual.contains(
            &requested_gate,
        ) {
            return Err(
                IqmAdapterError::UnsupportedInstruction(
                    requested_gate,
                ),
            );
        }
    }

    Ok(())
}

fn validate_instruction_arguments(
    name: &str,
    args: &Map<String, Value>,
) -> Result<(), IqmAdapterError> {
    match name {
        "measure" => {
            let key =
                args
                    .get("key")
                    .and_then(Value::as_str)
                    .ok_or(
                        IqmAdapterError::InvalidInstructionArguments,
                    )?;

            validate_measurement_key(
                key,
            )?;

            if let Some(feedback_key) =
                args.get("feedback_key")
            {
                let feedback_key =
                    feedback_key
                        .as_str()
                        .ok_or(
                            IqmAdapterError::InvalidInstructionArguments,
                        )?;

                validate_measurement_key(
                    feedback_key,
                )?;
            }
        }

        "prx" => {
            let angle =
                args
                    .get("angle")
                    .and_then(Value::as_f64)
                    .ok_or(
                        IqmAdapterError::InvalidInstructionArguments,
                    )?;

            let phase =
                args
                    .get("phase")
                    .and_then(Value::as_f64)
                    .ok_or(
                        IqmAdapterError::InvalidInstructionArguments,
                    )?;

            validate_finite(
                "angle",
                angle,
            )?;

            validate_finite(
                "phase",
                phase,
            )?;
        }

        "cc_prx" => {
            let angle =
                args
                    .get("angle")
                    .and_then(Value::as_f64)
                    .ok_or(
                        IqmAdapterError::InvalidInstructionArguments,
                    )?;

            let phase =
                args
                    .get("phase")
                    .and_then(Value::as_f64)
                    .ok_or(
                        IqmAdapterError::InvalidInstructionArguments,
                    )?;

            let feedback_key =
                args
                    .get("feedback_key")
                    .and_then(Value::as_str)
                    .ok_or(
                        IqmAdapterError::InvalidInstructionArguments,
                    )?;

            let feedback_qubit =
                args
                    .get("feedback_qubit")
                    .and_then(Value::as_str)
                    .ok_or(
                        IqmAdapterError::InvalidInstructionArguments,
                    )?;

            validate_finite(
                "angle",
                angle,
            )?;

            validate_finite(
                "phase",
                phase,
            )?;

            validate_measurement_key(
                feedback_key,
            )?;

            validate_component_name(
                feedback_qubit,
            )?;
        }

        "delay" => {
            let duration =
                args
                    .get("duration")
                    .and_then(Value::as_f64)
                    .ok_or(
                        IqmAdapterError::InvalidInstructionArguments,
                    )?;

            if !duration.is_finite()
                || duration < 0.0
            {
                return Err(
                    IqmAdapterError::InvalidNumericValue(
                        "duration",
                    ),
                );
            }
        }

        "cz"
        | "reset"
        | "move"
        | "barrier" => {
            /*
             * Current IQM native operations do not require numeric arguments
             * for these instructions.
             *
             * Additional provider fields may be introduced by future IQM
             * versions, but the adapter deliberately does not silently
             * reinterpret unknown argument semantics.
             */
        }

        _ => {
            return Err(
                IqmAdapterError::UnsupportedInstruction(
                    name.to_owned(),
                ),
            );
        }
    }

    Ok(())
}

fn validate_qubit_mapping(
    value: &Value,
) -> Result<(), IqmAdapterError> {
    let array =
        value
            .as_array()
            .ok_or(
                IqmAdapterError::InvalidProgramField(
                    "iqm.qubit_mapping",
                ),
            )?;

    if array.len()
        > 1_000_000
    {
        return Err(
            IqmAdapterError::ResultLimitExceeded,
        );
    }

    for entry in array {
        let object =
            entry
                .as_object()
                .ok_or(
                    IqmAdapterError::InvalidProgramField(
                        "iqm.qubit_mapping",
                    ),
                )?;

        let logical =
            object
                .get("logical_name")
                .and_then(Value::as_str)
                .ok_or(
                    IqmAdapterError::InvalidProgramField(
                        "iqm.qubit_mapping.logical_name",
                    ),
                )?;

        let physical =
            object
                .get("physical_name")
                .and_then(Value::as_str)
                .ok_or(
                    IqmAdapterError::InvalidProgramField(
                        "iqm.qubit_mapping.physical_name",
                    ),
                )?;

        validate_component_name(
            logical,
        )?;

        validate_component_name(
            physical,
        )?;
    }

    Ok(())
}

// =============================================================================
// Result normalization
// =============================================================================

fn parse_measurement_counts(
    value: &Value,
) -> Result<BTreeMap<String, usize>, IqmAdapterError> {
    /*
     * Current IQM Server / QDMI integrations expose the measurement_counts
     * artifact as:
     *
     * [
     *   {
     *     "counts": {
     *       "00": 500,
     *       "11": 500
     *     }
     *   }
     * ]
     *
     * Accepting a direct object as well makes the adapter tolerant of
     * transport/proxy wrappers while retaining strict validation.
     */

    let object =
        if let Some(array) =
            value.as_array()
        {
            let first =
                array.first().ok_or(
                    IqmAdapterError::MissingMeasurementCounts,
                )?;

            first
                .as_object()
                .ok_or(
                    IqmAdapterError::InvalidResponse(
                        "measurement_counts[0]",
                    ),
                )?
        } else {
            value
                .as_object()
                .ok_or(
                    IqmAdapterError::InvalidResponse(
                        "measurement_counts",
                    ),
                )?
        };

    let counts =
        object
            .get("counts")
            .and_then(Value::as_object)
            .ok_or(
                IqmAdapterError::InvalidResponse(
                    "measurement_counts.counts",
                ),
            )?;

    if counts.len()
        > MAX_IQM_RESULT_ENTRIES
    {
        return Err(
            IqmAdapterError::ResultLimitExceeded,
        );
    }

    let mut normalized =
        BTreeMap::new();

    for (bitstring, value) in counts {
        validate_result_bitstring(
            bitstring,
        )?;

        let count =
            value
                .as_u64()
                .and_then(|value| {
                    usize::try_from(
                        value,
                    )
                    .ok()
                })
                .ok_or(
                    IqmAdapterError::InvalidResponse(
                        "measurement_counts.counts value",
                    ),
                )?;

        if count == 0 {
            /*
             * Zero-count outcomes are semantically redundant and should not
             * enter the normalized result.
             */
            continue;
        }

        normalized.insert(
            bitstring.clone(),
            count,
        );
    }

    Ok(normalized)
}

// =============================================================================
// Provider status normalization
// =============================================================================

fn map_provider_status(
    status: &str,
) -> Result<BackendJobState, IqmAdapterError> {
    let normalized =
        status
            .trim()
            .to_ascii_lowercase();

    let state =
        match normalized.as_str() {
            "created" => {
                BackendJobState::Created
            }

            "pending"
            | "queued"
            | "pending_compilation"
            | "pending_execution"
            | "compilation_started"
            | "compilation_ended" => {
                BackendJobState::Queued
            }

            "running"
            | "execution_started" => {
                BackendJobState::Running
            }

            "completed"
            | "ready" => {
                BackendJobState::Completed
            }

            "cancelled"
            | "canceled"
            | "aborted" => {
                BackendJobState::Cancelled
            }

            "failed"
            | "error" => {
                BackendJobState::Failed
            }

            "expired" => {
                BackendJobState::Expired
            }

            _ => {
                /*
                 * IQM client explicitly tolerates unrecognized provider
                 * statuses for forward compatibility. Zamani does the same,
                 * but safely maps them to Unknown rather than guessing.
                 */
                BackendJobState::Unknown
            }
        };

    Ok(state)
}

// =============================================================================
// Quantum-computer status normalization
// =============================================================================

fn normalize_quantum_computer_status(
    status: &str,
) -> BackendStatus {
    match status
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "available"
        | "online"
        | "ready"
        | "operational"
        | "up" => {
            BackendStatus::Available
        }

        "busy"
        | "running" => {
            BackendStatus::Busy
        }

        "maintenance"
        | "maintaining" => {
            BackendStatus::Maintenance
        }

        "degraded"
        | "warning" => {
            BackendStatus::Degraded
        }

        "offline"
        | "down" => {
            BackendStatus::Offline
        }

        "retired"
        | "decommissioned" => {
            BackendStatus::Retired
        }

        "unavailable"
        | "disabled" => {
            BackendStatus::Unavailable
        }

        _ => {
            BackendStatus::Unknown
        }
    }
}

fn is_accepting_status(
    status: &str,
) -> bool {
    matches!(
        status
            .trim()
            .to_ascii_lowercase()
            .as_str(),
        "available"
            | "online"
            | "ready"
            | "operational"
            | "up"
            | "busy"
            | "running"
    )
}

// =============================================================================
// Target discovery
// =============================================================================

fn find_quantum_computer<'a>(
    value: &'a Value,
    target: &str,
) -> Option<&'a Map<String, Value>> {
    let array =
        value.as_array()?;

    for item in array {
        let object =
            item.as_object()?;

        let id =
            object
                .get("id")
                .and_then(Value::as_str);

        let alias =
            object
                .get("alias")
                .and_then(Value::as_str);

        if id == Some(target)
            || alias == Some(target)
        {
            return Some(object);
        }
    }

    None
}

fn extract_target_from_job(
    value: &Value,
) -> Option<String> {
    value
        .get("quantum_computer")
        .and_then(Value::as_str)
        .or_else(|| {
            value
                .get("quantum_computer_id")
                .and_then(Value::as_str)
        })
        .or_else(|| {
            value
                .get("quantum_computer_alias")
                .and_then(Value::as_str)
        })
        .map(str::to_owned)
}

fn target_matches(
    expected: &str,
    actual: &str,
) -> bool {
    expected == actual
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_target(
    target: &str,
) -> Result<(), IqmAdapterError> {
    if target.trim().is_empty() {
        return Err(
            IqmAdapterError::InvalidTarget,
        );
    }

    if target.len()
        > MAX_IQM_TARGET_LENGTH
    {
        return Err(
            IqmAdapterError::TargetTooLong,
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
            IqmAdapterError::InvalidTarget,
        );
    }

    Ok(())
}

fn canonical_backend_id(
    target: &str,
) -> String {
    format!(
        "{}/{}",
        IQM_PROVIDER_ID,
        target
    )
}

fn validate_circuit_name(
    name: &str,
) -> Result<(), IqmAdapterError> {
    if name.trim().is_empty()
        || name.len()
            > MAX_IQM_CIRCUIT_NAME_LENGTH
        || name
            .chars()
            .any(char::is_control)
    {
        return Err(
            IqmAdapterError::InvalidProgramField(
                "name",
            ),
        );
    }

    Ok(())
}

fn validate_instruction_name(
    name: &str,
) -> Result<(), IqmAdapterError> {
    if name.trim().is_empty()
        || name.len()
            > MAX_IQM_INSTRUCTION_NAME_LENGTH
        || name
            .chars()
            .any(|character| {
                character.is_control()
                    || character.is_whitespace()
            })
    {
        return Err(
            IqmAdapterError::InvalidInstructionName,
        );
    }

    Ok(())
}

fn validate_component_name(
    component: &str,
) -> Result<(), IqmAdapterError> {
    if component.trim().is_empty()
        || component.len()
            > MAX_IQM_COMPONENT_NAME_LENGTH
        || component
            .chars()
            .any(char::is_control)
    {
        return Err(
            IqmAdapterError::InvalidInstructionLocus,
        );
    }

    Ok(())
}

fn validate_measurement_key(
    key: &str,
) -> Result<(), IqmAdapterError> {
    if key.trim().is_empty()
        || key.len()
            > MAX_IQM_MEASUREMENT_KEY_LENGTH
        || key
            .chars()
            .any(char::is_control)
    {
        return Err(
            IqmAdapterError::InvalidMeasurementKey,
        );
    }

    Ok(())
}

fn validate_finite(
    field: &'static str,
    value: f64,
) -> Result<(), IqmAdapterError> {
    if !value.is_finite() {
        return Err(
            IqmAdapterError::InvalidNumericValue(
                field,
            ),
        );
    }

    Ok(())
}

fn validate_job_id(
    id: &str,
) -> Result<(), IqmAdapterError> {
    if id.trim().is_empty()
        || id.len() > 1024
        || id.chars().any(char::is_control)
    {
        return Err(
            IqmAdapterError::InvalidJobId,
        );
    }

    Ok(())
}

fn validate_safe_metadata_value(
    value: &str,
) -> Result<(), IqmAdapterError> {
    if value.trim().is_empty()
        || value.len() > 4096
        || value.chars().any(char::is_control)
    {
        return Err(
            IqmAdapterError::InvalidProgramField(
                "metadata",
            ),
        );
    }

    if contains_secret_marker(
        value,
    ) {
        return Err(
            IqmAdapterError::InvalidProgramField(
                "metadata",
            ),
        );
    }

    Ok(())
}

fn is_safe_metadata_value(
    value: &str,
) -> bool {
    validate_safe_metadata_value(
        value,
    )
    .is_ok()
}

fn contains_secret_marker(
    value: &str,
) -> bool {
    let normalized =
        value.to_ascii_lowercase();

    [
        "authorization",
        "bearer ",
        "api_key",
        "apikey",
        "access_token",
        "refresh_token",
        "client_secret",
        "private_key",
        "password",
        "secret",
        "cookie",
    ]
    .iter()
    .any(|marker| {
        normalized.contains(marker)
    })
}

fn is_supported_instruction(
    name: &str,
) -> bool {
    matches!(
        normalize_instruction(name)
            .as_str(),
        "measure"
            | "prx"
            | "cc_prx"
            | "reset"
            | "cz"
            | "move"
            | "barrier"
            | "delay"
    )
}

fn normalize_instruction(
    value: &str,
) -> String {
    value
        .trim()
        .to_ascii_lowercase()
}

fn validate_result_bitstring(
    value: &str,
) -> Result<(), IqmAdapterError> {
    if value.is_empty()
        || value.len()
            > MAX_IQM_COMPONENT_NAME_LENGTH
        || !value
            .bytes()
            .all(|byte| {
                byte == b'0'
                    || byte == b'1'
            })
    {
        return Err(
            IqmAdapterError::ResultNormalization(
                format!(
                    "invalid measurement bitstring '{}'",
                    value
                ),
            ),
        );
    }

    Ok(())
}

// =============================================================================
// JSON / transport helpers
// =============================================================================

fn parse_json_response(
    response: &ProviderResponse,
) -> Result<Value, IqmAdapterError> {
    serde_json::from_slice(
        response.body.as_slice(),
    )
    .map_err(|_| {
        IqmAdapterError::InvalidResponseJson
    })
}

fn safe_response_message(
    response: &ProviderResponse,
) -> String {
    let body =
        &response.body;

    let bounded =
        &body[
            ..body
                .len()
                .min(
                    MAX_IQM_ERROR_BODY_BYTES,
                )
        ];

    let text =
        String::from_utf8_lossy(
            bounded,
        );

    sanitize_error_message(
        &text,
    )
}

fn sanitize_error_message(
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

    if result.len()
        > MAX_IQM_ERROR_BODY_BYTES
    {
        result.truncate(
            MAX_IQM_ERROR_BODY_BYTES,
        );
    }

    result
}

fn path_escape(
    value: &str,
) -> String {
    /*
     * IQM job IDs are UUID-like values in normal operation.
     *
     * We nevertheless percent-encode every byte outside the conservative
     * unreserved URI set rather than interpolating arbitrary user input.
     */
    let mut output =
        String::with_capacity(
            value.len(),
        );

    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'-'
            | b'_'
            | b'.'
            | b'~' => {
                output.push(
                    byte as char,
                );
            }

            _ => {
                output.push('%');

                output.push(
                    hex_digit(
                        byte >> 4,
                    ),
                );

                output.push(
                    hex_digit(
                        byte & 0x0f,
                    ),
                );
            }
        }
    }

    output
}

fn hex_digit(
    value: u8,
) -> char {
    match value {
        0..=9 => {
            (b'0' + value) as char
        }

        10..=15 => {
            (b'A' + (value - 10))
                as char
        }

        _ => unreachable!(),
    }
}

// =============================================================================
// Deterministic request IDs
// =============================================================================

fn canonical_request_id(
    request: &ExecutionRequest,
    program: &BackendProgram,
) -> String {
    if let Some(request_id) =
        &request.request_id
    {
        return request_id.clone();
    }

    let mut hasher =
        Sha256::new();

    hasher.update(
        IQM_ADAPTER_ID.as_bytes(),
    );

    hasher.update(
        [0u8],
    );

    hasher.update(
        program.format().as_bytes(),
    );

    hasher.update(
        [0u8],
    );

    hasher.update(
        program.bytes(),
    );

    hasher.update(
        [0u8],
    );

    hasher.update(
        request
            .workload
            .circuit
            .shots
            .to_le_bytes(),
    );

    let digest =
        hasher.finalize();

    format!(
        "zamani-iqm-{}",
        hex::encode(
            &digest[..16],
        )
    )
}

fn stable_hash_id(
    prefix: &str,
    value: &str,
) -> String {
    let mut hasher =
        Sha256::new();

    hasher.update(
        prefix.as_bytes(),
    );

    hasher.update(
        [0u8],
    );

    hasher.update(
        value.as_bytes(),
    );

    let digest =
        hasher.finalize();

    format!(
        "zamani-iqm-{}-{}",
        normalize_instruction(prefix),
        hex::encode(
            &digest[..16],
        )
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_backend_id_is_stable() {
        assert_eq!(
            canonical_backend_id(
                "garnet"
            ),
            "iqm/garnet"
        );
    }

    #[test]
    fn target_rejects_whitespace() {
        assert!(
            validate_target(
                "gar net"
            )
            .is_err()
        );
    }

    #[test]
    fn target_rejects_empty() {
        assert!(
            validate_target(
                ""
            )
            .is_err()
        );
    }

    #[test]
    fn supported_instructions_are_explicit() {
        assert!(
            is_supported_instruction(
                "measure"
            )
        );

        assert!(
            is_supported_instruction(
                "prx"
            )
        );

        assert!(
            is_supported_instruction(
                "cc_prx"
            )
        );

        assert!(
            is_supported_instruction(
                "cz"
            )
        );

        assert!(
            is_supported_instruction(
                "reset"
            )
        );

        assert!(
            is_supported_instruction(
                "move"
            )
        );

        assert!(
            is_supported_instruction(
                "barrier"
            )
        );

        assert!(
            is_supported_instruction(
                "delay"
            )
        );

        assert!(
            !is_supported_instruction(
                "cnot"
            )
        );
    }

    #[test]
    fn provider_capabilities_are_conservative() {
        let capabilities =
            IqmAdapter::provider_capabilities();

        assert!(
            capabilities.measurement
        );

        assert!(
            capabilities.reset
        );

        assert!(
            capabilities
                .mid_circuit_measurement
        );

        assert!(
            capabilities
                .classical_control
        );

        assert!(
            capabilities
                .dynamic_circuits
        );

        assert!(
            capabilities.cancellation
        );

        assert!(
            capabilities
                .topology_information
        );

        assert!(
            !capabilities
                .annealing
        );

        assert!(
            !capabilities
                .analog_control
        );
    }

    #[test]
    fn instruction_validation_accepts_prx() {
        let mut args =
            Map::new();

        args.insert(
            "angle".to_owned(),
            Value::from(
                std::f64::consts::PI,
            ),
        );

        args.insert(
            "phase".to_owned(),
            Value::from(0.0),
        );

        assert!(
            validate_instruction_arguments(
                "prx",
                &args,
            )
            .is_ok()
        );
    }

    #[test]
    fn instruction_validation_rejects_nan() {
        let mut args =
            Map::new();

        args.insert(
            "angle".to_owned(),
            Value::from(
                f64::NAN,
            ),
        );

        args.insert(
            "phase".to_owned(),
            Value::from(0.0),
        );

        assert!(
            validate_instruction_arguments(
                "prx",
                &args,
            )
            .is_err()
        );
    }

    #[test]
    fn measurement_requires_key() {
        let args =
            Map::new();

        assert!(
            validate_instruction_arguments(
                "measure",
                &args,
            )
            .is_err()
        );
    }

    #[test]
    fn result_bitstrings_are_binary() {
        assert!(
            validate_result_bitstring(
                "0101"
            )
            .is_ok()
        );

        assert!(
            validate_result_bitstring(
                "0121"
            )
            .is_err()
        );
    }

    #[test]
    fn result_counts_are_normalized() {
        let value =
            serde_json::json!([
                {
                    "counts": {
                        "00": 500,
                        "11": 500
                    }
                }
            ]);

        let counts =
            parse_measurement_counts(
                &value,
            )
            .expect(
                "valid IQM counts",
            );

        assert_eq!(
            counts.get("00"),
            Some(&500)
        );

        assert_eq!(
            counts.get("11"),
            Some(&500)
        );
    }

    #[test]
    fn result_counts_reject_non_binary_keys() {
        let value =
            serde_json::json!([
                {
                    "counts": {
                        "0x": 100
                    }
                }
            ]);

        assert!(
            parse_measurement_counts(
                &value,
            )
            .is_err()
        );
    }

    #[test]
    fn unknown_provider_state_is_not_success() {
        let state =
            map_provider_status(
                "future_state",
            )
            .expect(
                "status mapping must succeed",
            );

        assert_eq!(
            state,
            BackendJobState::Unknown
        );
    }

    #[test]
    fn provider_statuses_are_normalized() {
        assert_eq!(
            map_provider_status(
                "created"
            )
            .unwrap(),
            BackendJobState::Created
        );

        assert_eq!(
            map_provider_status(
                "running"
            )
            .unwrap(),
            BackendJobState::Running
        );

        assert_eq!(
            map_provider_status(
                "completed"
            )
            .unwrap(),
            BackendJobState::Completed
        );

        assert_eq!(
            map_provider_status(
                "failed"
            )
            .unwrap(),
            BackendJobState::Failed
        );

        assert_eq!(
            map_provider_status(
                "cancelled"
            )
            .unwrap(),
            BackendJobState::Cancelled
        );
    }

    #[test]
    fn request_id_is_deterministic() {
        let request =
            ExecutionRequest::new(
                Default::default(),
            );

        let program =
            BackendProgram::new(
                IQM_CIRCUIT_FORMAT,
                br#"{
                    "name":"bell",
                    "instructions":[
                        {
                            "name":"measure",
                            "locus":["QB1"],
                            "args":{"key":"m"}
                        }
                    ]
                }"#,
            )
            .expect(
                "program must be valid",
            );

        let first =
            canonical_request_id(
                &request,
                &program,
            );

        let second =
            canonical_request_id(
                &request,
                &program,
            );

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn path_escape_is_safe() {
        assert_eq!(
            path_escape(
                "abc-123"
            ),
            "abc-123"
        );

        assert_eq!(
            path_escape(
                "abc/123"
            ),
            "abc%2F123"
        );
    }

    #[test]
    fn secret_markers_are_rejected() {
        assert!(
            contains_secret_marker(
                "Authorization: Bearer x"
            )
        );

        assert!(
            contains_secret_marker(
                "client_secret"
            )
        );

        assert!(
            !contains_secret_marker(
                "calibration_set_id"
            )
        );
    }

    #[test]
    fn quantum_computer_status_mapping_is_conservative() {
        assert_eq!(
            normalize_quantum_computer_status(
                "available"
            ),
            BackendStatus::Available
        );

        assert_eq!(
            normalize_quantum_computer_status(
                "maintenance"
            ),
            BackendStatus::Maintenance
        );

        assert_eq!(
            normalize_quantum_computer_status(
                "future-status"
            ),
            BackendStatus::Unknown
        );
    }
}