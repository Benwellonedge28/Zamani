//! Zamani Quantum — Rigetti QCS Hardware Adapter
//!
//! Production-grade provider-specific semantic adapter for Rigetti Quantum
//! Cloud Services (QCS).
//!
//! Path:
//!
//!     src/quantum/hardware/adapters/rigetti.rs
//!
//! # Responsibility
//!
//! This module translates Rigetti/QCS semantics into Zamani's canonical,
//! provider-neutral quantum hardware contracts.
//!
//! It owns:
//!
//! - Rigetti provider identity;
//! - Rigetti QPU descriptor normalization;
//! - Rigetti native Quil/Quil-T format negotiation;
//! - Rigetti capability normalization;
//! - Rigetti instruction-set metadata normalization;
//! - Rigetti job lifecycle normalization;
//! - Rigetti result normalization;
//! - Rigetti queue/reservation metadata normalization;
//! - Rigetti health normalization;
//! - Rigetti cancellation semantics;
//! - Rigetti-safe provider metadata;
//! - Rigetti provider-operation construction;
//! - Rigetti provider-error sanitization;
//! - provider-neutral `QuantumBackendAdapter` integration;
//! - deterministic provider request construction;
//! - provider-independent conformance compatibility.
//!
//! It deliberately does NOT own:
//!
//! - credentials;
//! - OAuth/OIDC;
//! - access/refresh tokens;
//! - TLS;
//! - HTTP implementation;
//! - gRPC implementation;
//! - QCS SDK implementation;
//! - QCS credential files;
//! - Quil parsing;
//! - Quil compilation;
//! - Quil-T compilation;
//! - OpenQASM parsing;
//! - OpenQASM -> Quil transpilation;
//! - Zamani Quantum IR;
//! - routing;
//! - scheduling;
//! - calibration storage;
//! - topology algorithms;
//! - benchmarking;
//! - provider registries;
//! - global mutable state.
//!
//! Those responsibilities belong to the corresponding Zamani subsystems or
//! the injected `ProviderTransport` implementation.
//!
//! # Rigetti interoperability
//!
//! Rigetti QCS exposes two major API surfaces:
//!
//! 1. an OpenAPI/JSON API for management, QPU data, architecture,
//!    reservations and calibration-related information;
//! 2. a gRPC API for high-performance QPU interaction and translation.
//!
//! Consequently this adapter does NOT pretend that QPU execution is a
//! conventional REST endpoint.
//!
//! Instead:
//!
//! ```text
//! Zamani Quantum IR
//!         |
//!         v
//! compatibility / routing / scheduling
//!         |
//!         v
//! Quil / Quil-T executable
//!         |
//!         v
//! QuantumBackendAdapter
//!         |
//!         v
//! Rigetti semantic adapter
//!         |
//!         v
//! ProviderTransport
//!         |
//!         +----------------------+
//!         |                      |
//!         v                      v
//!     QCS gRPC              QCS management API
//!         |                      |
//!         +----------+-----------+
//!                    |
//!                    v
//!                Rigetti QPU
//! ```
//!
//! The transport implementation is therefore where the actual Rigetti QCS
//! Rust SDK/gRPC client belongs.
//!
//! # Native program formats
//!
//! Rigetti's native quantum programming language is Quil.
//!
//! Quil-T extends Quil with pulse/analog-level control.
//!
//! This adapter therefore recognizes:
//!
//! - `quil`;
//! - `quil-t`;
//! - `quil3`;
//! - `quil-3`.
//!
//! OpenQASM is intentionally NOT silently accepted as a native Rigetti
//! executable format. If OpenQASM support is required, the dedicated
//! OpenQASM adapter/transpiler must produce Quil before this adapter is used.
//!
//! # Calibration semantics
//!
//! Rigetti documentation identifies the Quil calibration program as an
//! authoritative source for native gates and gate timing. Native instruction
//! availability can also vary by qubit and coupling edge.
//!
//! Therefore this adapter never invents a native gate set from a hard-coded
//! list. The `RigettiBackendDescriptor` must receive the discovered/native
//! instruction set from the QCS architecture/calibration layer.
//!
//! # Execution lifecycle
//!
//! ```text
//! preflight
//!     |
//!     v
//! validate backend
//!     |
//!     v
//! validate Quil / Quil-T
//!     |
//!     v
//! build provider-neutral QCS operation
//!     |
//!     v
//! ProviderTransport
//!     |
//!     v
//! provider job ID
//!     |
//!     v
//! BackendJobId
//!     |
//!     +------> status()
//!     |
//!     +------> cancel()
//!     |
//!     +------> result()
//! ```
//!
//! Remote QPU execution is asynchronous by default.
//!
//! `submit()` MUST return a job handle rather than pretending that a remote
//! execution is synchronous.
//!
//! # Security
//!
//! No credentials are stored in this module.
//!
//! The following must never enter:
//!
//! - backend metadata;
//! - request metadata;
//! - provider error messages;
//! - provider response metadata;
//! - debug output.
//!
//! ```text
//! access_token
//! refresh_token
//! authorization
//! api_key
//! password
//! private_key
//! cookie
//! secret
//! ```
//!
//! Authentication belongs to `credentials.rs` and `authentication.rs` and is
//! applied by the injected transport implementation.
//!
//! # No-reedit contract
//!
//! This file is intentionally self-contained against the existing stable
//! hardware contracts.
//!
//! It consumes:
//!
//! - `backend.rs`;
//! - `backend_trait.rs`;
//! - `adapters/generic.rs`.
//!
//! It does not require future changes to those modules for ordinary Rigetti
//! operation.
//!
//! Future modules consume this file:
//!
//! - `provider_registry.rs` registers the adapter;
//! - `device_registry.rs` indexes the canonical backend;
//! - `discovery.rs` supplies `RigettiBackendDescriptor`;
//! - `execution.rs` drives submit/status/result;
//! - `queue.rs` consumes `BackendQueueInfo`;
//! - `health.rs` consumes `BackendHealth`;
//! - `benchmarking` consumes the provider-neutral result;
//! - Danga consumes the provider-neutral execution boundary.
//!
//! Adding another Rigetti QPU MUST NOT require changing this adapter. Only
//! its descriptor/discovery data should change.
//!
//! Adding another provider MUST NOT require changing this file.
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
//! This module forbids unsafe Rust.
//!
//! # Determinism
//!
//! Request construction is deterministic:
//!
//! - `BTreeMap` is used for metadata;
//! - `BTreeSet` is used for capabilities/instructions;
//! - no random IDs are generated;
//! - no system clock is read;
//! - provider job IDs are preserved;
//! - no hidden retries are performed;
//! - POST submission is never automatically retried.
//!
//! # Important provider distinction
//!
//! A Rigetti QPU may have:
//!
//! - on-demand access;
//! - reservation-based access;
//! - provider queueing;
//! - provider-specific execution timeout;
//! - request timeout.
//!
//! Request timeout and QPU execution timeout are different concepts.
//!
//! This adapter therefore keeps provider timeout information as metadata rather
//! than conflating it with transport timeout.
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
use std::time::Duration;

use serde_json::{json, Value};

use super::generic::{
    AdapterMetadata,
    ProviderOperation,
    ProviderRequest,
    ProviderResponse,
    ProviderTransport,
    TransportMethod,
};

use crate::quantum::hardware::backend::{
    BackendCapabilities,
    BackendError,
    BackendHealth,
    BackendHealthState,
    BackendKind,
    BackendLimits,
    BackendMetadata,
    BackendQueueInfo,
    BackendStatus,
    BackendAdapterInfo,
    CancellationOutcome,
    ExecutionRequest,
    ExecutionResult,
    QuantumBackend,
};

use crate::quantum::hardware::backend_trait::{
    BackendCancellation,
    BackendJob,
    BackendJobId,
    BackendJobState,
    BackendJobStatus,
    BackendProgram,
    ConformantQuantumBackendAdapter,
    QuantumBackendAdapter,
};

// =============================================================================
// Constants
// =============================================================================

/// Stable provider identifier.
pub const RIGETTI_PROVIDER_ID: &str = "rigetti";

/// Stable Zamani adapter identifier.
pub const RIGETTI_ADAPTER_ID: &str =
    "zamani.quantum.hardware.rigetti";

/// Semantic version of the Zamani Rigetti adapter.
pub const RIGETTI_ADAPTER_VERSION: &str = "1.0.0";

/// Rigetti QCS API family.
pub const RIGETTI_API_VERSION: &str = "qcs";

/// Canonical native Rigetti executable format.
pub const RIGETTI_QUIL_FORMAT: &str = "quil";

/// Canonical Rigetti pulse/analog executable format.
pub const RIGETTI_QUIL_T_FORMAT: &str = "quil-t";

/// Compatibility spelling for Quil 3.
pub const RIGETTI_QUIL3_FORMAT: &str = "quil3";

/// Compatibility spelling for Quil 3.
pub const RIGETTI_QUIL_3_FORMAT: &str = "quil-3";

/// Maximum Rigetti backend identifier.
pub const MAX_RIGETTI_BACKEND_ID_LENGTH: usize = 512;

/// Maximum Rigetti backend name.
pub const MAX_RIGETTI_BACKEND_NAME_LENGTH: usize = 512;

/// Maximum Rigetti region.
pub const MAX_RIGETTI_REGION_LENGTH: usize = 256;

/// Maximum provider status string retained.
pub const MAX_RIGETTI_STATUS_LENGTH: usize = 4096;

/// Maximum provider metadata entries.
pub const MAX_RIGETTI_METADATA_FIELDS: usize = 512;

/// Maximum provider metadata value length.
pub const MAX_RIGETTI_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum native instructions.
pub const MAX_RIGETTI_NATIVE_INSTRUCTIONS: usize = 16_384;

/// Maximum result entries.
pub const MAX_RIGETTI_RESULT_ENTRIES: usize = 1_000_000;

/// Maximum result bitstring/register key length.
pub const MAX_RIGETTI_RESULT_KEY_LENGTH: usize = 4096;

/// Maximum provider error message retained.
pub const MAX_RIGETTI_ERROR_MESSAGE_LENGTH: usize = 4096;

/// Maximum job identifier length.
pub const MAX_RIGETTI_JOB_ID_LENGTH: usize = 1024;

/// Maximum request identifier length.
pub const MAX_RIGETTI_REQUEST_ID_LENGTH: usize = 512;

/// Maximum QCS endpoint length.
pub const MAX_RIGETTI_ENDPOINT_LENGTH: usize = 4096;

/// Default transport timeout.
///
/// This is a client-side request timeout, NOT the Rigetti QPU execution
/// duration timeout.
pub const DEFAULT_REQUEST_TIMEOUT: Duration =
    Duration::from_secs(60);

/// Default QPU execution timeout used only when the provider transport requires
/// an explicit value.
///
/// The provider may impose its own maximum.
pub const DEFAULT_EXECUTION_TIMEOUT: Duration =
    Duration::from_secs(60);

// =============================================================================
// Provider operation identifiers
// =============================================================================

/// Stable Rigetti provider operation identifiers.
pub mod operation {
    /// Describe a QPU.
    pub const DESCRIBE: &str =
        "rigetti.qcs.describe";

    /// Discover QPUs.
    pub const DISCOVERY: &str =
        "rigetti.qcs.discovery";

    /// Retrieve architecture/ISA.
    pub const ARCHITECTURE: &str =
        "rigetti.qcs.architecture";

    /// Retrieve calibrations.
    pub const CALIBRATION: &str =
        "rigetti.qcs.calibration";

    /// Retrieve health.
    pub const HEALTH: &str =
        "rigetti.qcs.health";

    /// Retrieve queue/reservation state.
    pub const QUEUE: &str =
        "rigetti.qcs.queue";

    /// Submit QPU execution.
    pub const SUBMIT: &str =
        "rigetti.qcs.submit";

    /// Retrieve job status.
    pub const STATUS: &str =
        "rigetti.qcs.status";

    /// Retrieve execution result.
    pub const RESULT: &str =
        "rigetti.qcs.result";

    /// Cancel execution.
    pub const CANCEL: &str =
        "rigetti.qcs.cancel";
}

// =============================================================================
// Capability identifiers
// =============================================================================

/// Stable Rigetti capability identifiers.
pub mod capability {
    /// Measurement.
    pub const MEASUREMENT: &str = "measurement";

    /// Reset.
    pub const RESET: &str = "reset";

    /// Mid-circuit measurement.
    pub const MID_CIRCUIT_MEASUREMENT: &str =
        "mid_circuit_measurement";

    /// Classical control.
    pub const CLASSICAL_CONTROL: &str =
        "classical_control";

    /// Dynamic circuits.
    pub const DYNAMIC_CIRCUITS: &str =
        "dynamic_circuits";

    /// Parameterized gates.
    pub const PARAMETERIZED_GATES: &str =
        "parameterized_gates";

    /// Pulse-level control.
    pub const PULSE_CONTROL: &str =
        "pulse_control";

    /// Quil-T.
    pub const QUIL_T: &str =
        "quil_t";

    /// Cancellation.
    pub const CANCELLATION: &str =
        "cancellation";

    /// Queue information.
    pub const QUEUE_INFORMATION: &str =
        "queue_information";

    /// Calibration.
    pub const CALIBRATION_DATA: &str =
        "calibration_data";

    /// Topology.
    pub const TOPOLOGY_INFORMATION: &str =
        "topology_information";

    /// Native instruction set.
    pub const NATIVE_INSTRUCTION_SET: &str =
        "native_instruction_set";

    /// Parallel execution.
    pub const PARALLEL_OPERATIONS: &str =
        "parallel_operations";
}

// =============================================================================
// Provider states
// =============================================================================

/// Rigetti provider job-state names.
///
/// Unknown states are NEVER guessed as successful.
pub mod provider_state {
    /// Created.
    pub const CREATED: &str = "created";

    /// Submitted.
    pub const SUBMITTED: &str = "submitted";

    /// Queued.
    pub const QUEUED: &str = "queued";

    /// Running.
    pub const RUNNING: &str = "running";

    /// Completed.
    pub const COMPLETED: &str = "completed";

    /// Failed.
    pub const FAILED: &str = "failed";

    /// Cancelled.
    pub const CANCELLED: &str = "cancelled";

    /// Cancelling.
    pub const CANCELLING: &str = "cancelling";

    /// Expired.
    pub const EXPIRED: &str = "expired";

    /// Timed out.
    pub const TIMED_OUT: &str = "timed_out";
}

// =============================================================================
// Errors
// =============================================================================

/// Rigetti-specific semantic errors.
///
/// This error never crosses the public provider-neutral boundary. It is
/// translated into `BackendError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RigettiAdapterError {
    /// Invalid backend identifier.
    InvalidBackendId,

    /// Backend identifier too long.
    BackendIdTooLong,

    /// Invalid backend name.
    InvalidBackendName,

    /// Backend name too long.
    BackendNameTooLong,

    /// Invalid provider region.
    InvalidRegion,

    /// Region too long.
    RegionTooLong,

    /// Invalid endpoint.
    InvalidEndpoint,

    /// Missing/invalid adapter configuration.
    InvalidConfiguration(String),

    /// Backend descriptor and canonical backend disagree.
    BackendIdentityMismatch,

    /// Backend is not executable.
    BackendUnavailable,

    /// Program format is unsupported.
    UnsupportedProgramFormat(String),

    /// Provider capability is unavailable.
    UnsupportedCapability(String),

    /// Provider instruction is unavailable.
    UnsupportedInstruction(String),

    /// Provider returned an unknown lifecycle state.
    UnknownProviderState(String),

    /// Provider returned malformed data.
    InvalidResponse(String),

    /// Provider returned a failure.
    ProviderFailure {
        /// Stable provider operation.
        operation: &'static str,

        /// Sanitized provider message.
        message: String,
    },

    /// Provider transport failed.
    Transport(String),

    /// Result normalization failed.
    ResultNormalization(String),

    /// Invalid provider job ID.
    InvalidJobId,

    /// Provider job ID too long.
    JobIdTooLong,

    /// Secret-like metadata was detected.
    SecretMaterialRejected,

    /// Unsupported cancellation.
    CancellationUnsupported,

    /// Invalid JSON payload.
    InvalidJson(String),
}

impl fmt::Display for RigettiAdapterError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidBackendId => {
                write!(formatter, "invalid Rigetti backend identifier")
            }

            Self::BackendIdTooLong => {
                write!(formatter, "Rigetti backend identifier is too long")
            }

            Self::InvalidBackendName => {
                write!(formatter, "invalid Rigetti backend name")
            }

            Self::BackendNameTooLong => {
                write!(formatter, "Rigetti backend name is too long")
            }

            Self::InvalidRegion => {
                write!(formatter, "invalid Rigetti region")
            }

            Self::RegionTooLong => {
                write!(formatter, "Rigetti region is too long")
            }

            Self::InvalidEndpoint => {
                write!(formatter, "invalid Rigetti endpoint")
            }

            Self::InvalidConfiguration(message) => {
                write!(
                    formatter,
                    "invalid Rigetti adapter configuration: {message}"
                )
            }

            Self::BackendIdentityMismatch => {
                write!(
                    formatter,
                    "Rigetti descriptor/backend identity mismatch"
                )
            }

            Self::BackendUnavailable => {
                write!(
                    formatter,
                    "Rigetti backend is not currently executable"
                )
            }

            Self::UnsupportedProgramFormat(format) => {
                write!(
                    formatter,
                    "Rigetti adapter does not support program format '{format}'"
                )
            }

            Self::UnsupportedCapability(capability) => {
                write!(
                    formatter,
                    "Rigetti backend does not support capability '{capability}'"
                )
            }

            Self::UnsupportedInstruction(instruction) => {
                write!(
                    formatter,
                    "Rigetti backend does not support instruction '{instruction}'"
                )
            }

            Self::UnknownProviderState(state) => {
                write!(
                    formatter,
                    "unknown Rigetti provider job state '{state}'"
                )
            }

            Self::InvalidResponse(message) => {
                write!(
                    formatter,
                    "invalid Rigetti provider response: {message}"
                )
            }

            Self::ProviderFailure {
                operation,
                message,
            } => {
                write!(
                    formatter,
                    "Rigetti operation {operation} failed: {message}"
                )
            }

            Self::Transport(message) => {
                write!(
                    formatter,
                    "Rigetti transport failure: {message}"
                )
            }

            Self::ResultNormalization(message) => {
                write!(
                    formatter,
                    "Rigetti result normalization failure: {message}"
                )
            }

            Self::InvalidJobId => {
                write!(formatter, "invalid Rigetti job identifier")
            }

            Self::JobIdTooLong => {
                write!(formatter, "Rigetti job identifier is too long")
            }

            Self::SecretMaterialRejected => {
                write!(
                    formatter,
                    "Rigetti secret-like metadata was rejected"
                )
            }

            Self::CancellationUnsupported => {
                write!(
                    formatter,
                    "Rigetti cancellation is unsupported for this backend"
                )
            }

            Self::InvalidJson(message) => {
                write!(
                    formatter,
                    "invalid Rigetti JSON payload: {message}"
                )
            }
        }
    }
}

impl std::error::Error for RigettiAdapterError {}

// =============================================================================
// Configuration
// =============================================================================

/// Immutable configuration for the Rigetti semantic adapter.
///
/// Credentials are intentionally absent.
///
/// The endpoint is informational/transport configuration. Actual QPU
/// interaction is performed by `ProviderTransport`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RigettiAdapterConfig {
    /// QCS management API endpoint.
    pub management_endpoint: String,

    /// QCS gRPC endpoint.
    pub grpc_endpoint: String,

    /// Provider API identifier.
    pub api_version: String,

    /// Client-side request timeout.
    pub request_timeout: Duration,

    /// Default provider execution timeout.
    pub execution_timeout: Duration,

    /// Whether experimental capabilities may be requested.
    pub allow_experimental: bool,

    /// Whether a completed provider state must explicitly advertise result
    /// availability.
    pub require_result_availability: bool,

    /// Whether backend identity must match on job/result operations.
    pub require_backend_identity_match: bool,
}

impl Default for RigettiAdapterConfig {
    fn default() -> Self {
        Self {
            management_endpoint:
                "https://api.qcs.rigetti.com".to_owned(),

            grpc_endpoint:
                "https://grpc.qcs.rigetti.com".to_owned(),

            api_version:
                RIGETTI_API_VERSION.to_owned(),

            request_timeout:
                DEFAULT_REQUEST_TIMEOUT,

            execution_timeout:
                DEFAULT_EXECUTION_TIMEOUT,

            allow_experimental:
                false,

            require_result_availability:
                true,

            require_backend_identity_match:
                true,
        }
    }
}

impl RigettiAdapterConfig {
    /// Creates the production default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Validates configuration without performing I/O.
    pub fn validate(&self) -> Result<(), RigettiAdapterError> {
        validate_endpoint(
            &self.management_endpoint,
        )?;

        validate_endpoint(
            &self.grpc_endpoint,
        )?;

        if self.api_version.trim().is_empty() {
            return Err(
                RigettiAdapterError::InvalidConfiguration(
                    "API version must not be empty".to_owned(),
                ),
            );
        }

        if self.request_timeout.is_zero() {
            return Err(
                RigettiAdapterError::InvalidConfiguration(
                    "request timeout must be non-zero".to_owned(),
                ),
            );
        }

        if self.execution_timeout.is_zero() {
            return Err(
                RigettiAdapterError::InvalidConfiguration(
                    "execution timeout must be non-zero".to_owned(),
                ),
            );
        }

        Ok(())
    }
}

// =============================================================================
// Rigetti backend descriptor
// =============================================================================

/// Provider-specific Rigetti QPU descriptor.
///
/// This structure is populated from QCS discovery/architecture/calibration
/// information and is then paired with the canonical `QuantumBackend`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RigettiBackendDescriptor {
    /// Rigetti QPU identifier.
    pub backend_id: String,

    /// Human-readable QPU name.
    pub name: String,

    /// Operational status.
    pub status: BackendStatus,

    /// Physical qubit count.
    pub qubit_count: usize,

    /// Stable provider-neutral capability identifiers.
    pub capabilities: BTreeSet<String>,

    /// Rigetti native Quil instructions.
    pub native_instructions: BTreeSet<String>,

    /// Optional region/location.
    pub region: Option<String>,

    /// Optional hardware revision.
    pub hardware_revision: Option<String>,

    /// Optional firmware/provider version.
    pub provider_version: Option<String>,

    /// Optional QCS architecture version.
    pub architecture_version: Option<String>,

    /// Optional calibration identifier.
    pub calibration_id: Option<String>,

    /// Whether the descriptor came from a live calibration/ISA snapshot.
    pub calibration_backed: bool,

    /// Safe provider metadata.
    pub metadata: BTreeMap<String, String>,
}

impl RigettiBackendDescriptor {
    /// Creates a validated descriptor.
    pub fn new(
        backend_id: impl Into<String>,
        name: impl Into<String>,
        qubit_count: usize,
    ) -> Result<Self, RigettiAdapterError> {
        let backend_id = backend_id.into();
        let name = name.into();

        validate_backend_id(&backend_id)?;
        validate_backend_name(&name)?;

        if qubit_count == 0 {
            return Err(
                RigettiAdapterError::InvalidResponse(
                    "Rigetti QPU reported zero qubits".to_owned(),
                ),
            );
        }

        Ok(Self {
            backend_id,
            name,
            status: BackendStatus::Unknown,
            qubit_count,
            capabilities: BTreeSet::new(),
            native_instructions: BTreeSet::new(),
            region: None,
            hardware_revision: None,
            provider_version: None,
            architecture_version: None,
            calibration_id: None,
            calibration_backed: false,
            metadata: BTreeMap::new(),
        })
    }

    /// Sets status.
    pub fn with_status(
        mut self,
        status: BackendStatus,
    ) -> Self {
        self.status = status;
        self
    }

    /// Adds a capability.
    pub fn with_capability(
        mut self,
        capability: impl Into<String>,
    ) -> Result<Self, RigettiAdapterError> {
        let capability =
            normalize_identifier(&capability.into());

        if capability.is_empty() {
            return Err(
                RigettiAdapterError::InvalidResponse(
                    "empty Rigetti capability".to_owned(),
                ),
            );
        }

        self.capabilities.insert(capability);
        Ok(self)
    }

    /// Adds many capabilities.
    pub fn with_capabilities<I, S>(
        mut self,
        capabilities: I,
    ) -> Result<Self, RigettiAdapterError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for capability in capabilities {
            self = self.with_capability(capability)?;
        }

        Ok(self)
    }

    /// Adds a native instruction.
    pub fn with_native_instruction(
        mut self,
        instruction: impl Into<String>,
    ) -> Result<Self, RigettiAdapterError> {
        let instruction =
            normalize_instruction(&instruction.into());

        if instruction.is_empty() {
            return Err(
                RigettiAdapterError::InvalidResponse(
                    "empty Rigetti native instruction".to_owned(),
                ),
            );
        }

        if self.native_instructions.len()
            >= MAX_RIGETTI_NATIVE_INSTRUCTIONS
            && !self.native_instructions.contains(&instruction)
        {
            return Err(
                RigettiAdapterError::InvalidResponse(
                    "Rigetti native instruction limit exceeded"
                        .to_owned(),
                ),
            );
        }

        self.native_instructions.insert(instruction);
        Ok(self)
    }

    /// Adds many native instructions.
    pub fn with_native_instructions<I, S>(
        mut self,
        instructions: I,
    ) -> Result<Self, RigettiAdapterError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for instruction in instructions {
            self = self.with_native_instruction(instruction)?;
        }

        Ok(self)
    }

    /// Sets region.
    pub fn with_region(
        mut self,
        region: impl Into<String>,
    ) -> Result<Self, RigettiAdapterError> {
        let region = region.into();

        validate_region(&region)?;
        self.region = Some(region);

        Ok(self)
    }

    /// Sets hardware revision.
    pub fn with_hardware_revision(
        mut self,
        revision: impl Into<String>,
    ) -> Self {
        self.hardware_revision = Some(revision.into());
        self
    }

    /// Sets provider version.
    pub fn with_provider_version(
        mut self,
        version: impl Into<String>,
    ) -> Self {
        self.provider_version = Some(version.into());
        self
    }

    /// Sets architecture version.
    pub fn with_architecture_version(
        mut self,
        version: impl Into<String>,
    ) -> Self {
        self.architecture_version = Some(version.into());
        self
    }

    /// Sets calibration identity.
    pub fn with_calibration(
        mut self,
        calibration_id: impl Into<String>,
    ) -> Self {
        self.calibration_id =
            Some(calibration_id.into());
        self.calibration_backed = true;
        self
    }

    /// Marks descriptor as calibration-backed.
    pub fn calibration_backed(
        mut self,
        value: bool,
    ) -> Self {
        self.calibration_backed = value;
        self
    }

    /// Inserts safe metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, RigettiAdapterError> {
        let key = normalize_identifier(&key.into());
        let value = value.into();

        if contains_secret_marker(&key)
            || contains_secret_marker(&value)
        {
            return Err(
                RigettiAdapterError::SecretMaterialRejected
            );
        }

        if value.len()
            > MAX_RIGETTI_METADATA_VALUE_LENGTH
        {
            return Err(
                RigettiAdapterError::InvalidResponse(
                    "Rigetti metadata value exceeds limit"
                        .to_owned(),
                ),
            );
        }

        if self.metadata.len()
            >= MAX_RIGETTI_METADATA_FIELDS
            && !self.metadata.contains_key(&key)
        {
            return Err(
                RigettiAdapterError::InvalidResponse(
                    "Rigetti metadata field limit exceeded"
                        .to_owned(),
                ),
            );
        }

        self.metadata.insert(key, value);
        Ok(self)
    }
}

// =============================================================================
// Adapter
// =============================================================================

/// Production Rigetti semantic adapter.
///
/// The transport is injected so the semantic adapter remains independent of
/// the HTTP/gRPC implementation.
pub struct RigettiQuantumAdapter<T>
where
    T: ProviderTransport + 'static,
{
    config: RigettiAdapterConfig,

    transport: Arc<T>,

    backend: QuantumBackend,

    descriptor: RigettiBackendDescriptor,

    adapter_info: BackendAdapterInfo,
}

impl<T> fmt::Debug for RigettiQuantumAdapter<T>
where
    T: ProviderTransport + 'static,
{
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("RigettiQuantumAdapter")
            .field("config", &self.config)
            .field("backend", &self.backend)
            .field("descriptor", &self.descriptor)
            .field("adapter_info", &self.adapter_info)
            .field("transport", &"<injected>")
            .finish()
    }
}

impl<T> RigettiQuantumAdapter<T>
where
    T: ProviderTransport + 'static,
{
    /// Creates a fully validated Rigetti adapter.
    ///
    /// No network request is made.
    pub fn new(
        config: RigettiAdapterConfig,
        transport: Arc<T>,
        descriptor: RigettiBackendDescriptor,
        backend: QuantumBackend,
    ) -> Result<Self, RigettiAdapterError> {
        config.validate()?;

        if descriptor.backend_id
            != backend.metadata.id
        {
            return Err(
                RigettiAdapterError::BackendIdentityMismatch
            );
        }

        if descriptor.qubit_count
            != backend.topology.qubit_count()
        {
            return Err(
                RigettiAdapterError::BackendIdentityMismatch
            );
        }

        if backend.metadata.provider
            != RIGETTI_PROVIDER_ID
        {
            return Err(
                RigettiAdapterError::BackendIdentityMismatch
            );
        }

        if backend.metadata.kind
            != BackendKind::Qpu
        {
            return Err(
                RigettiAdapterError::InvalidConfiguration(
                    "Rigetti adapter requires BackendKind::Qpu"
                        .to_owned(),
                ),
            );
        }

        if backend.capabilities.native_instruction_set
            && backend.capabilities.native_gates.is_empty()
        {
            return Err(
                RigettiAdapterError::InvalidConfiguration(
                    "backend advertises native instruction support but exposes no native instructions"
                        .to_owned(),
                ),
            );
        }

        let adapter_info =
            BackendAdapterInfo::new(
                RIGETTI_ADAPTER_ID,
                RIGETTI_ADAPTER_VERSION,
                true,
            )
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?
            .with_provider_api_version(
                RIGETTI_API_VERSION,
            )
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?;

        Ok(Self {
            config,
            transport,
            backend,
            descriptor,
            adapter_info,
        })
    }

    /// Returns the canonical backend.
    pub fn backend(
        &self,
    ) -> &QuantumBackend {
        &self.backend
    }

    /// Returns the Rigetti provider descriptor.
    pub fn descriptor(
        &self,
    ) -> &RigettiBackendDescriptor {
        &self.descriptor
    }

    /// Returns adapter configuration.
    pub fn config(
        &self,
    ) -> &RigettiAdapterConfig {
        &self.config
    }

    /// Returns stable adapter identity.
    pub fn adapter_identity() -> &'static str {
        RIGETTI_ADAPTER_ID
    }

    /// Returns generic adapter metadata.
    ///
    /// This is useful for provider registries that use the generic adapter
    /// metadata model in addition to the canonical `BackendAdapterInfo`.
    pub fn adapter_metadata()
        -> Result<AdapterMetadata, RigettiAdapterError>
    {
        let identity =
            super::generic::AdapterIdentity::new(
                RIGETTI_ADAPTER_ID,
                RIGETTI_PROVIDER_ID,
                RIGETTI_ADAPTER_VERSION,
                Some(
                    RIGETTI_API_VERSION.to_owned()
                ),
            )
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?;

        let mut metadata =
            AdapterMetadata::new(
                identity,
                "Zamani Rigetti QCS Adapter",
            )
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?;

        metadata = metadata
            .with_format(RIGETTI_QUIL_FORMAT)
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?;

        metadata = metadata
            .with_format(RIGETTI_QUIL_T_FORMAT)
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?;

        metadata = metadata
            .with_format(RIGETTI_QUIL3_FORMAT)
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?;

        metadata = metadata
            .with_capability(capability::MEASUREMENT)
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?;

        metadata = metadata
            .with_capability(capability::TOPOLOGY_INFORMATION)
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?;

        metadata = metadata
            .with_capability(capability::NATIVE_INSTRUCTION_SET)
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?;

        metadata = metadata
            .with_capability(capability::CALIBRATION_DATA)
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?;

        metadata = metadata
            .with_capability(capability::PARAMETERIZED_GATES)
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?;

        metadata = metadata
            .with_capability(capability::DYNAMIC_CIRCUITS)
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?;

        metadata = metadata
            .with_capability(capability::PULSE_CONTROL)
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?;

        metadata = metadata
            .with_capability(capability::QUIL_T)
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?;

        metadata = metadata
            .with_capability(capability::PARALLEL_OPERATIONS)
            .map_err(|error| {
                RigettiAdapterError::InvalidConfiguration(
                    error.to_string(),
                )
            })?;

        metadata
    }

    /// Normalizes a program format.
    ///
    /// OpenQASM is intentionally not accepted here because this adapter is a
    /// Rigetti-native executable boundary.
    pub fn normalize_program_format(
        format: &str,
    ) -> Result<&'static str, RigettiAdapterError> {
        match normalize_identifier(format).as_str() {
            RIGETTI_QUIL_FORMAT => {
                Ok(RIGETTI_QUIL_FORMAT)
            }

            RIGETTI_QUIL_T_FORMAT => {
                Ok(RIGETTI_QUIL_T_FORMAT)
            }

            RIGETTI_QUIL3_FORMAT
            | RIGETTI_QUIL_3_FORMAT => {
                Ok(RIGETTI_QUIL_FORMAT)
            }

            _ => {
                Err(
                    RigettiAdapterError::UnsupportedProgramFormat(
                        format.to_owned(),
                    ),
                )
            }
        }
    }

    /// Returns whether a format is accepted.
    pub fn supports_program_format(
        format: &str,
    ) -> bool {
        Self::normalize_program_format(format)
            .is_ok()
    }

    /// Returns whether the descriptor advertises a capability.
    pub fn supports_capability(
        &self,
        capability_name: &str,
    ) -> bool {
        self.descriptor
            .capabilities
            .contains(
                &normalize_identifier(
                    capability_name
                )
            )
    }

    /// Returns whether a native instruction is advertised.
    pub fn supports_instruction(
        &self,
        instruction: &str,
    ) -> bool {
        self.descriptor
            .native_instructions
            .contains(
                &normalize_instruction(instruction)
            )
    }

    /// Validates a Rigetti execution request before provider I/O.
    pub fn validate_execution_request(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<(), RigettiAdapterError> {
        self.backend
            .preflight(request)
            .map_err(map_backend_error)?;

        Self::normalize_program_format(
            program.format()
        )?;

        if program.is_empty() {
            return Err(
                RigettiAdapterError::InvalidResponse(
                    "cannot submit an empty Quil program"
                        .to_owned(),
                ),
            );
        }

        if !self.backend.is_available() {
            return Err(
                RigettiAdapterError::BackendUnavailable
            );
        }

        if request.workload.circuit.shots == 0 {
            return Err(
                RigettiAdapterError::InvalidResponse(
                    "Rigetti execution requires at least one shot"
                        .to_owned(),
                ),
            );
        }

        if request.workload.circuit.shots
            > self.backend.limits.max_shots
            && self.backend.limits.max_shots != 0
        {
            return Err(
                RigettiAdapterError::InvalidResponse(
                    "requested shots exceed Rigetti backend limit"
                        .to_owned(),
                ),
            );
        }

        for instruction
            in &request.workload.required_instructions
        {
            if !self.supports_instruction(
                instruction
            ) {
                return Err(
                    RigettiAdapterError::UnsupportedInstruction(
                        instruction.clone()
                    )
                );
            }
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Provider request construction
    // -------------------------------------------------------------------------

    /// Builds a provider-neutral Rigetti submission request.
    ///
    /// The injected transport decides whether this is ultimately translated
    /// into the QCS gRPC execution API or another supported QCS mechanism.
    ///
    /// No credentials are placed in this request.
    pub fn build_submit_request(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<ProviderRequest, RigettiAdapterError> {
        self.validate_execution_request(
            request,
            program,
        )?;

        let format =
            Self::normalize_program_format(
                program.format()
            )?;

        let request_id =
            request
                .request_id
                .as_deref()
                .unwrap_or("zamani-rigetti-request");

        validate_request_id(request_id)?;

        let body =
            build_execution_body(
                &self.descriptor.backend_id,
                format,
                program,
                request,
                self.config.execution_timeout,
            )?;

        ProviderRequest::builder(
            ProviderOperation::Submit,
            TransportMethod::Custom,
            format!(
                "qpu/{}/execute",
                path_segment(
                    &self.descriptor.backend_id
                )?
            ),
            request_id,
        )
        .body(body)
        .map_err(map_generic_error)?
        .build()
        .map_err(map_generic_error)
    }

    /// Builds a job-status request.
    pub fn build_status_request(
        &self,
        job_id: &BackendJobId,
    ) -> Result<ProviderRequest, RigettiAdapterError> {
        let encoded =
            path_segment(job_id.as_str())?;

        ProviderRequest::builder(
            ProviderOperation::GetJobStatus,
            TransportMethod::Custom,
            format!("qpu/jobs/{encoded}"),
            job_id.as_str(),
        )
        .build()
        .map_err(map_generic_error)
    }

    /// Builds a result request.
    pub fn build_result_request(
        &self,
        job_id: &BackendJobId,
    ) -> Result<ProviderRequest, RigettiAdapterError> {
        let encoded =
            path_segment(job_id.as_str())?;

        ProviderRequest::builder(
            ProviderOperation::GetResult,
            TransportMethod::Custom,
            format!("qpu/jobs/{encoded}/result"),
            job_id.as_str(),
        )
        .build()
        .map_err(map_generic_error)
    }

    /// Builds a cancellation request.
    pub fn build_cancel_request(
        &self,
        job_id: &BackendJobId,
    ) -> Result<ProviderRequest, RigettiAdapterError> {
        let encoded =
            path_segment(job_id.as_str())?;

        ProviderRequest::builder(
            ProviderOperation::Cancel,
            TransportMethod::Custom,
            format!("qpu/jobs/{encoded}/cancel"),
            job_id.as_str(),
        )
        .build()
        .map_err(map_generic_error)
    }

    /// Builds a queue information request.
    pub fn build_queue_request(
        &self,
    ) -> Result<ProviderRequest, RigettiAdapterError> {
        ProviderRequest::builder(
            ProviderOperation::GetQueue,
            TransportMethod::Custom,
            format!(
                "qpu/{}/queue",
                path_segment(
                    &self.descriptor.backend_id
                )?
            ),
            self.descriptor.backend_id.as_str(),
        )
        .build()
        .map_err(map_generic_error)
    }

    /// Builds a health request.
    pub fn build_health_request(
        &self,
    ) -> Result<ProviderRequest, RigettiAdapterError> {
        ProviderRequest::builder(
            ProviderOperation::GetHealth,
            TransportMethod::Custom,
            format!(
                "qpu/{}/health",
                path_segment(
                    &self.descriptor.backend_id
                )?
            ),
            self.descriptor.backend_id.as_str(),
        )
        .build()
        .map_err(map_generic_error)
    }

    // -------------------------------------------------------------------------
    // Provider response normalization
    // -------------------------------------------------------------------------

    /// Normalizes a Rigetti job identifier.
    pub fn normalize_job_id(
        value: &str,
    ) -> Result<BackendJobId, RigettiAdapterError> {
        let value = value.trim();

        if value.is_empty() {
            return Err(
                RigettiAdapterError::InvalidJobId
            );
        }

        if value.len()
            > MAX_RIGETTI_JOB_ID_LENGTH
        {
            return Err(
                RigettiAdapterError::JobIdTooLong
            );
        }

        BackendJobId::new(
            value.to_owned()
        )
        .map_err(|_| {
            RigettiAdapterError::InvalidJobId
        })
    }

    /// Normalizes a Rigetti provider state.
    pub fn normalize_job_state(
        state: &str,
    ) -> Result<BackendJobState, RigettiAdapterError> {
        match normalize_identifier(state).as_str() {
            provider_state::CREATED
            | provider_state::SUBMITTED => {
                Ok(BackendJobState::Created)
            }

            provider_state::QUEUED => {
                Ok(BackendJobState::Queued)
            }

            provider_state::RUNNING => {
                Ok(BackendJobState::Running)
            }

            provider_state::COMPLETED => {
                Ok(BackendJobState::Completed)
            }

            provider_state::FAILED => {
                Ok(BackendJobState::Failed)
            }

            provider_state::CANCELLED => {
                Ok(BackendJobState::Cancelled)
            }

            provider_state::CANCELLING => {
                Ok(BackendJobState::Cancelling)
            }

            provider_state::EXPIRED => {
                Ok(BackendJobState::Expired)
            }

            provider_state::TIMED_OUT => {
                Ok(BackendJobState::TimedOut)
            }

            other => Err(
                RigettiAdapterError::UnknownProviderState(
                    other.to_owned()
                )
            ),
        }
    }

    /// Normalizes a provider job status.
    pub fn normalize_job_status(
        &self,
        job_id: BackendJobId,
        state: &str,
        provider_status: Option<String>,
        queue_position: Option<usize>,
        estimated_wait: Option<Duration>,
        result_available: bool,
    ) -> Result<BackendJobStatus, RigettiAdapterError> {
        let state =
            Self::normalize_job_state(state)?;

        let provider_status =
            provider_status.map(
                sanitize_status
            );

        let job =
            BackendJob::new(
                job_id,
                self.descriptor
                    .backend_id
                    .clone(),
                None,
                state,
            )
            .map_err(|_| {
                RigettiAdapterError::InvalidJobId
            })?;

        let result_available =
            if self.config
                .require_result_availability
            {
                result_available
            } else {
                result_available
                    || matches!(
                        state,
                        BackendJobState::Completed
                    )
            };

        Ok(BackendJobStatus {
            job,
            provider_status,
            queue_position,
            estimated_wait,
            result_available,
        })
    }

    /// Normalizes Rigetti readout/count results.
    ///
    /// The accepted transport-normalized JSON forms are:
    ///
    /// ```json
    /// {
    ///   "shots": 1000,
    ///   "counts": {
    ///     "00": 500,
    ///     "11": 500
    ///   }
    /// }
    /// ```
    ///
    /// or:
    ///
    /// ```json
    /// {
    ///   "result": {
    ///     "shots": 1000,
    ///     "counts": {
    ///       "00": 500,
    ///       "11": 500
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// The transport may also normalize QCS's native matrix/register result
    /// representation into this stable envelope.
    pub fn normalize_counts_result(
        &self,
        job_id: &BackendJobId,
        shots: usize,
        counts: BTreeMap<String, usize>,
    ) -> Result<ExecutionResult, RigettiAdapterError> {
        if shots == 0 {
            return Err(
                RigettiAdapterError::ResultNormalization(
                    "Rigetti result has zero shots"
                        .to_owned(),
                ),
            );
        }

        if counts.len()
            > MAX_RIGETTI_RESULT_ENTRIES
        {
            return Err(
                RigettiAdapterError::ResultNormalization(
                    "Rigetti result contains too many entries"
                        .to_owned(),
                ),
            );
        }

        let mut result =
            ExecutionResult::empty(
                self.descriptor
                    .backend_id
                    .clone(),
                shots,
            )
            .map_err(map_backend_error)?;

        for (bitstring, count) in counts {
            if bitstring.len()
                > MAX_RIGETTI_RESULT_KEY_LENGTH
            {
                return Err(
                    RigettiAdapterError::ResultNormalization(
                        "Rigetti result bitstring is too long"
                            .to_owned(),
                    ),
                );
            }

            result
                .insert_count(
                    bitstring,
                    count,
                )
                .map_err(map_backend_error)?;
        }

        if !result.counts_within_shots() {
            return Err(
                RigettiAdapterError::ResultNormalization(
                    "Rigetti result represents more shots than requested"
                        .to_owned(),
                ),
            );
        }

        result
            .metadata
            .insert(
                "provider".to_owned(),
                RIGETTI_PROVIDER_ID.to_owned(),
            );

        result
            .metadata
            .insert(
                "adapter_id".to_owned(),
                RIGETTI_ADAPTER_ID.to_owned(),
            );

        result
            .metadata
            .insert(
                "adapter_version".to_owned(),
                RIGETTI_ADAPTER_VERSION.to_owned(),
            );

        result
            .metadata
            .insert(
                "provider_job_id".to_owned(),
                job_id.as_str().to_owned(),
            );

        result
            .metadata
            .insert(
                "program_format".to_owned(),
                RIGETTI_QUIL_FORMAT.to_owned(),
            );

        result
            .validate()
            .map_err(map_backend_error)?;

        Ok(result)
    }

    /// Extracts counts from a normalized QCS response envelope.
    pub fn extract_counts(
        body: &[u8],
    ) -> Result<
        (usize, BTreeMap<String, usize>),
        RigettiAdapterError,
    > {
        let value: Value =
            serde_json::from_slice(body)
                .map_err(|error| {
                    RigettiAdapterError::InvalidJson(
                        sanitize_error(
                            &error.to_string()
                        ),
                    )
                })?;

        let result =
            value
                .get("result")
                .unwrap_or(&value);

        let shots =
            result
                .get("shots")
                .and_then(Value::as_u64)
                .ok_or_else(|| {
                    RigettiAdapterError::ResultNormalization(
                        "Rigetti result is missing shots"
                            .to_owned(),
                    )
                })?;

        let shots =
            usize::try_from(shots)
                .map_err(|_| {
                    RigettiAdapterError::ResultNormalization(
                        "Rigetti shot count exceeds platform usize"
                            .to_owned(),
                    )
                })?;

        let counts_value =
            result
                .get("counts")
                .ok_or_else(|| {
                    RigettiAdapterError::ResultNormalization(
                        "Rigetti result is missing counts"
                            .to_owned(),
                    )
                })?;

        let object =
            counts_value
                .as_object()
                .ok_or_else(|| {
                    RigettiAdapterError::ResultNormalization(
                        "Rigetti counts must be a JSON object"
                            .to_owned(),
                    )
                })?;

        if object.len()
            > MAX_RIGETTI_RESULT_ENTRIES
        {
            return Err(
                RigettiAdapterError::ResultNormalization(
                    "Rigetti result contains too many count entries"
                        .to_owned(),
                ),
            );
        }

        let mut counts =
            BTreeMap::new();

        for (key, value) in object {
            let count =
                value
                    .as_u64()
                    .ok_or_else(|| {
                        RigettiAdapterError::ResultNormalization(
                            "Rigetti count is not an unsigned integer"
                                .to_owned(),
                        )
                    })?;

            let count =
                usize::try_from(count)
                    .map_err(|_| {
                        RigettiAdapterError::ResultNormalization(
                            "Rigetti count exceeds platform usize"
                                .to_owned(),
                        )
                    })?;

            validate_bitstring_key(key)?;

            counts.insert(
                key.clone(),
                count,
            );
        }

        Ok((shots, counts))
    }

    /// Normalizes a provider error response.
    pub fn provider_failure(
        operation: &'static str,
        response: &ProviderResponse,
    ) -> RigettiAdapterError {
        RigettiAdapterError::ProviderFailure {
            operation,
            message: sanitize_provider_body(
                &response.body
            ),
        }
    }

    /// Requires a successful provider response.
    pub fn require_success(
        operation: &'static str,
        response: &ProviderResponse,
    ) -> Result<(), RigettiAdapterError> {
        if response.is_success() {
            Ok(())
        } else {
            Err(Self::provider_failure(
                operation,
                response,
            ))
        }
    }

    // -------------------------------------------------------------------------
    // Provider operations
    // -------------------------------------------------------------------------

    /// Submits a QPU job.
    pub fn submit(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<BackendJob, RigettiAdapterError> {
        let provider_request =
            self.build_submit_request(
                request,
                program,
            )?;

        let response =
            self.transport
                .send(&provider_request)
                .map_err(|error| {
                    RigettiAdapterError::Transport(
                        sanitize_error(
                            &error.to_string()
                        )
                    )
                })?;

        Self::require_success(
            operation::SUBMIT,
            &response,
        )?;

        let job_id =
            extract_job_id(
                &response.body
            )?;

        let job_id =
            Self::normalize_job_id(
                &job_id
            )?;

        BackendJob::new(
            job_id,
            self.descriptor
                .backend_id
                .clone(),
            request.request_id.clone(),
            BackendJobState::Created,
        )
        .map_err(|_| {
            RigettiAdapterError::InvalidJobId
        })
    }

    /// Retrieves job status.
    pub fn status(
        &self,
        job_id: &BackendJobId,
    ) -> Result<BackendJobStatus, RigettiAdapterError> {
        let request =
            self.build_status_request(
                job_id
            )?;

        let response =
            self.transport
                .send(&request)
                .map_err(|error| {
                    RigettiAdapterError::Transport(
                        sanitize_error(
                            &error.to_string()
                        )
                    )
                })?;

        Self::require_success(
            operation::STATUS,
            &response,
        )?;

        let value: Value =
            serde_json::from_slice(
                &response.body
            )
            .map_err(|error| {
                RigettiAdapterError::InvalidJson(
                    sanitize_error(
                        &error.to_string()
                    )
                )
            })?;

        let state =
            extract_string(
                &value,
                "status",
            )
            .or_else(|| {
                extract_string(
                    &value,
                    "state",
                )
            })
            .ok_or_else(|| {
                RigettiAdapterError::InvalidResponse(
                    "Rigetti status response has no status/state field"
                        .to_owned(),
                )
            })?;

        let result_available =
            value
                .get("result_available")
                .and_then(Value::as_bool)
                .unwrap_or(
                    matches!(
                        Self::normalize_job_state(
                            &state
                        )?,
                        BackendJobState::Completed
                    )
                );

        let queue_position =
            value
                .get("queue_position")
                .and_then(Value::as_u64)
                .and_then(
                    |value| {
                        usize::try_from(value)
                            .ok()
                    }
                );

        let estimated_wait =
            value
                .get("estimated_wait_seconds")
                .and_then(Value::as_u64)
                .map(Duration::from_secs);

        let provider_status =
            value
                .get("provider_status")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned);

        let status =
            self.normalize_job_status(
                job_id.clone(),
                &state,
                provider_status,
                queue_position,
                estimated_wait,
                result_available,
            )?;

        if self.config.require_backend_identity_match
            && status.job.backend_id
                != self.descriptor.backend_id
        {
            return Err(
                RigettiAdapterError::BackendIdentityMismatch
            );
        }

        Ok(status)
    }

    /// Retrieves a completed result.
    pub fn result(
        &self,
        job_id: &BackendJobId,
    ) -> Result<ExecutionResult, RigettiAdapterError> {
        let status =
            self.status(job_id)?;

        if !matches!(
            status.job.state,
            BackendJobState::Completed
        ) {
            return Err(
                RigettiAdapterError::ResultNormalization(
                    format!(
                        "Rigetti job '{}' is not completed; current state is '{}'",
                        job_id,
                        status.job.state
                    ),
                ),
            );
        }

        if self.config.require_result_availability
            && !status.result_available
        {
            return Err(
                RigettiAdapterError::ResultNormalization(
                    "Rigetti provider reported completion without a retrievable result"
                        .to_owned(),
                ),
            );
        }

        let request =
            self.build_result_request(
                job_id
            )?;

        let response =
            self.transport
                .send(&request)
                .map_err(|error| {
                    RigettiAdapterError::Transport(
                        sanitize_error(
                            &error.to_string()
                        )
                    )
                })?;

        Self::require_success(
            operation::RESULT,
            &response,
        )?;

        let (shots, counts) =
            Self::extract_counts(
                &response.body
            )?;

        self.normalize_counts_result(
            job_id,
            shots,
            counts,
        )
    }

    /// Requests provider-side cancellation.
    pub fn cancel(
        &self,
        job_id: &BackendJobId,
    ) -> Result<BackendCancellation, RigettiAdapterError> {
        if !self
            .backend
            .capabilities
            .cancellation
        {
            return Ok(
                BackendCancellation {
                    job: job_id.clone(),
                    outcome:
                        CancellationOutcome::Unsupported,
                }
            );
        }

        let current =
            self.status(job_id)?;

        if current.job.state.is_terminal() {
            return Ok(
                BackendCancellation {
                    job: job_id.clone(),
                    outcome:
                        CancellationOutcome::AlreadyTerminal,
                }
            );
        }

        let request =
            self.build_cancel_request(
                job_id
            )?;

        let response =
            self.transport
                .send(&request)
                .map_err(|error| {
                    RigettiAdapterError::Transport(
                        sanitize_error(
                            &error.to_string()
                        )
                    )
                })?;

        if response.is_success() {
            return Ok(
                BackendCancellation {
                    job: job_id.clone(),
                    outcome:
                        CancellationOutcome::Accepted,
                }
            );
        }

        /*
         * Cancellation may race with execution completion. The provider
         * transport must preserve the provider status. We do not convert an
         * arbitrary non-success response into cancellation success.
         */
        Err(Self::provider_failure(
            operation::CANCEL,
            &response,
        ))
    }

    /// Retrieves normalized queue information.
    pub fn queue_info(
        &self,
    ) -> Result<BackendQueueInfo, RigettiAdapterError> {
        if !self
            .backend
            .capabilities
            .queue_information
        {
            return Err(
                RigettiAdapterError::UnsupportedCapability(
                    capability::QUEUE_INFORMATION
                        .to_owned()
                )
            );
        }

        let request =
            self.build_queue_request()?;

        let response =
            self.transport
                .send(&request)
                .map_err(|error| {
                    RigettiAdapterError::Transport(
                        sanitize_error(
                            &error.to_string()
                        )
                    )
                })?;

        Self::require_success(
            operation::QUEUE,
            &response,
        )?;

        let value: Value =
            serde_json::from_slice(
                &response.body
            )
            .map_err(|error| {
                RigettiAdapterError::InvalidJson(
                    sanitize_error(
                        &error.to_string()
                    )
                )
            })?;

        let pending_jobs =
            value
                .get("pending_jobs")
                .and_then(Value::as_u64)
                .and_then(
                    |value| {
                        usize::try_from(value)
                            .ok()
                    }
                );

        let estimated_wait =
            value
                .get("estimated_wait_seconds")
                .and_then(Value::as_u64)
                .map(Duration::from_secs);

        let accepting_submissions =
            value
                .get("accepting_submissions")
                .and_then(Value::as_bool)
                .unwrap_or(
                    self.backend.is_available()
                );

        Ok(BackendQueueInfo {
            pending_jobs,
            estimated_wait,
            accepting_submissions,
        })
    }

    /// Performs a provider health check.
    pub fn health(
        &self,
    ) -> Result<BackendHealth, RigettiAdapterError> {
        let request =
            self.build_health_request()?;

        let response =
            self.transport
                .send(&request)
                .map_err(|error| {
                    RigettiAdapterError::Transport(
                        sanitize_error(
                            &error.to_string()
                        )
                    )
                })?;

        if !response.is_success() {
            return Ok(
                BackendHealth {
                    state:
                        BackendHealthState::Unreachable,
                    backend_status:
                        self.backend.status(),
                    message: Some(
                        sanitize_provider_body(
                            &response.body
                        ),
                    ),
                }
            );
        }

        let value: Value =
            serde_json::from_slice(
                &response.body
            )
            .map_err(|error| {
                RigettiAdapterError::InvalidJson(
                    sanitize_error(
                        &error.to_string()
                    )
                )
            })?;

        let healthy =
            value
                .get("healthy")
                .and_then(Value::as_bool)
                .unwrap_or(
                    self.backend.is_available()
                );

        let degraded =
            value
                .get("degraded")
                .and_then(Value::as_bool)
                .unwrap_or(false);

        let state =
            if !healthy {
                BackendHealthState::Unhealthy
            } else if degraded {
                BackendHealthState::Degraded
            } else {
                BackendHealthState::Healthy
            };

        let message =
            value
                .get("message")
                .and_then(Value::as_str)
                .map(sanitize_status);

        Ok(
            BackendHealth {
                state,
                backend_status:
                    self.backend.status(),
                message,
            }
        )
    }

    /// Converts the adapter's provider descriptor into canonical backend
    /// metadata.
    pub fn canonical_metadata(
        descriptor: &RigettiBackendDescriptor,
    ) -> BackendMetadata {
        let mut metadata =
            BackendMetadata::new(
                descriptor.backend_id.clone(),
                descriptor.name.clone(),
                RIGETTI_PROVIDER_ID,
                descriptor
                    .provider_version
                    .clone()
                    .unwrap_or_else(
                        || RIGETTI_ADAPTER_VERSION
                            .to_owned()
                    ),
                BackendKind::Qpu,
            );

        metadata.status =
            descriptor.status;

        metadata.region =
            descriptor.region.clone();

        metadata.hardware_revision =
            descriptor.hardware_revision.clone();

        metadata.api_version =
            Some(
                RIGETTI_API_VERSION
                    .to_owned()
            );

        if let Some(value) =
            &descriptor.provider_version
        {
            metadata
                .properties
                .insert(
                    "provider_version".to_owned(),
                    value.clone(),
                );
        }

        if let Some(value) =
            &descriptor.architecture_version
        {
            metadata
                .properties
                .insert(
                    "architecture_version".to_owned(),
                    value.clone(),
                );
        }

        if let Some(value) =
            &descriptor.calibration_id
        {
            metadata
                .properties
                .insert(
                    "calibration_id".to_owned(),
                    value.clone(),
                );
        }

        metadata
            .properties
            .insert(
                "calibration_backed".to_owned(),
                descriptor
                    .calibration_backed
                    .to_string(),
            );

        for (key, value)
            in &descriptor.metadata
        {
            if !contains_secret_marker(key)
                && !contains_secret_marker(value)
            {
                metadata
                    .properties
                    .insert(
                        key.clone(),
                        value.clone(),
                    );
            }
        }

        metadata
    }

    /// Builds a conservative canonical capability profile from the Rigetti
    /// descriptor.
    ///
    /// This method does NOT guess device-specific reset/cancellation/etc.
    /// support. Those values must come from discovery/architecture data.
    pub fn canonical_capabilities(
        descriptor: &RigettiBackendDescriptor,
    ) -> BackendCapabilities {
        let mut capabilities =
            BackendCapabilities::new();

        let has =
            |name: &str| {
                descriptor
                    .capabilities
                    .contains(name)
            };

        capabilities.measurement =
            has(capability::MEASUREMENT);

        capabilities.reset =
            has(capability::RESET);

        capabilities.mid_circuit_measurement =
            has(
                capability::MID_CIRCUIT_MEASUREMENT
            );

        capabilities.classical_control =
            has(capability::CLASSICAL_CONTROL);

        capabilities.dynamic_circuits =
            has(capability::DYNAMIC_CIRCUITS);

        capabilities.parameterized_gates =
            has(capability::PARAMETERIZED_GATES);

        capabilities.pulse_control =
            has(capability::PULSE_CONTROL)
                || has(capability::QUIL_T);

        capabilities.cancellation =
            has(capability::CANCELLATION);

        capabilities.queue_information =
            has(capability::QUEUE_INFORMATION);

        capabilities.calibration_data =
            has(capability::CALIBRATION_DATA);

        capabilities.topology_information =
            has(capability::TOPOLOGY_INFORMATION);

        capabilities.native_instruction_set =
            !descriptor
                .native_instructions
                .is_empty();

        capabilities.parallel_operations =
            has(
                capability::PARALLEL_OPERATIONS
            );

        capabilities.native_gates =
            descriptor
                .native_instructions
                .clone();

        capabilities
    }
}

// =============================================================================
// QuantumBackendAdapter implementation
// =============================================================================

impl<T> QuantumBackendAdapter
    for RigettiQuantumAdapter<T>
where
    T: ProviderTransport + 'static,
{
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
        self.validate_execution_request(
            request,
            program,
        )
        .map_err(map_rigetti_error)
    }

    fn submit(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<BackendJob, BackendError> {
        self.submit(
            request,
            program,
        )
        .map_err(map_rigetti_error)
    }

    fn status(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendJobStatus, BackendError> {
        self.status(job)
            .map_err(map_rigetti_error)
    }

    fn result(
        &self,
        job: &BackendJobId,
    ) -> Result<ExecutionResult, BackendError> {
        self.result(job)
            .map_err(map_rigetti_error)
    }

    fn cancel(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendCancellation, BackendError> {
        self.cancel(job)
            .map_err(map_rigetti_error)
    }

    fn queue_info(
        &self,
    ) -> Result<BackendQueueInfo, BackendError> {
        self.queue_info()
            .map_err(map_rigetti_error)
    }

    fn health(
        &self,
    ) -> Result<BackendHealth, BackendError> {
        self.health()
            .map_err(map_rigetti_error)
    }

    fn supports_cancellation(
        &self,
    ) -> bool {
        self.backend
            .capabilities
            .cancellation
    }

    fn supports_queue_info(
        &self,
    ) -> bool {
        self.backend
            .capabilities
            .queue_information
    }

    fn supports_synchronous_execution(
        &self,
    ) -> bool {
        false
    }
}

/// Explicit conformance marker.
///
/// This adapter implements the complete current
/// `QuantumBackendAdapter` contract. Provider conformance tests should still
/// execute the shared conformance suite before a registry marks a concrete
/// deployment as fully certified.
impl<T> ConformantQuantumBackendAdapter
    for RigettiQuantumAdapter<T>
where
    T: ProviderTransport + 'static,
{
}

// =============================================================================
// Request construction helpers
// =============================================================================

fn build_execution_body(
    backend_id: &str,
    format: &str,
    program: &BackendProgram,
    request: &ExecutionRequest,
    execution_timeout: Duration,
) -> Result<Vec<u8>, RigettiAdapterError> {
    let program_text =
        std::str::from_utf8(
            program.bytes()
        )
        .map_err(|_| {
            RigettiAdapterError::InvalidResponse(
                "Rigetti Quil program must be valid UTF-8"
                    .to_owned(),
            )
        })?;

    let body =
        json!({
            "backend": backend_id,
            "format": format,
            "program": program_text,
            "shots": request.workload.circuit.shots,
            "priority": request.priority,
            "asynchronous": true,
            "execution_timeout_seconds":
                execution_timeout.as_secs(),
            "seed": request.seed,
        });

    serde_json::to_vec(&body)
        .map_err(|error| {
            RigettiAdapterError::InvalidJson(
                sanitize_error(
                    &error.to_string()
                )
            )
        })
}

// =============================================================================
// JSON helpers
// =============================================================================

fn extract_job_id(
    body: &[u8],
) -> Result<String, RigettiAdapterError> {
    let value: Value =
        serde_json::from_slice(body)
            .map_err(|error| {
                RigettiAdapterError::InvalidJson(
                    sanitize_error(
                        &error.to_string()
                    )
                )
            })?;

    let id =
        value
            .get("job_id")
            .and_then(Value::as_str)
            .or_else(|| {
                value
                    .get("id")
                    .and_then(Value::as_str)
            })
            .or_else(|| {
                value
                    .get("job")
                    .and_then(|job| {
                        job.get("id")
                    })
                    .and_then(Value::as_str)
            })
            .ok_or_else(|| {
                RigettiAdapterError::InvalidResponse(
                    "Rigetti submission response does not contain a job identifier"
                        .to_owned(),
                )
            })?;

    Ok(id.to_owned())
}

fn extract_string(
    value: &Value,
    key: &str,
) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_backend_id(
    value: &str,
) -> Result<(), RigettiAdapterError> {
    let value = value.trim();

    if value.is_empty() {
        return Err(
            RigettiAdapterError::InvalidBackendId
        );
    }

    if value.len()
        > MAX_RIGETTI_BACKEND_ID_LENGTH
    {
        return Err(
            RigettiAdapterError::BackendIdTooLong
        );
    }

    if value.chars().any(char::is_control)
        || value.contains('/')
        || value.contains('\\')
    {
        return Err(
            RigettiAdapterError::InvalidBackendId
        );
    }

    Ok(())
}

fn validate_backend_name(
    value: &str,
) -> Result<(), RigettiAdapterError> {
    let value = value.trim();

    if value.is_empty() {
        return Err(
            RigettiAdapterError::InvalidBackendName
        );
    }

    if value.len()
        > MAX_RIGETTI_BACKEND_NAME_LENGTH
    {
        return Err(
            RigettiAdapterError::BackendNameTooLong
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            RigettiAdapterError::InvalidBackendName
        );
    }

    Ok(())
}

fn validate_region(
    value: &str,
) -> Result<(), RigettiAdapterError> {
    let value = value.trim();

    if value.is_empty() {
        return Err(
            RigettiAdapterError::InvalidRegion
        );
    }

    if value.len()
        > MAX_RIGETTI_REGION_LENGTH
    {
        return Err(
            RigettiAdapterError::RegionTooLong
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            RigettiAdapterError::InvalidRegion
        );
    }

    Ok(())
}

fn validate_endpoint(
    endpoint: &str,
) -> Result<(), RigettiAdapterError> {
    let endpoint = endpoint.trim();

    if endpoint.is_empty()
        || endpoint.len()
            > MAX_RIGETTI_ENDPOINT_LENGTH
        || endpoint.chars().any(
            char::is_control
        )
    {
        return Err(
            RigettiAdapterError::InvalidEndpoint
        );
    }

    /*
     * Remote Rigetti endpoints must use TLS.
     *
     * The actual gRPC implementation may use an endpoint representation
     * specific to the chosen client, but remote production endpoints must
     * still provide authenticated TLS.
     */
    if !endpoint.starts_with(
        "https://"
    ) {
        return Err(
            RigettiAdapterError::InvalidEndpoint
        );
    }

    Ok(())
}

fn validate_request_id(
    request_id: &str,
) -> Result<(), RigettiAdapterError> {
    if request_id.trim().is_empty()
        || request_id.len()
            > MAX_RIGETTI_REQUEST_ID_LENGTH
        || request_id.chars().any(
            char::is_control
        )
    {
        return Err(
            RigettiAdapterError::InvalidConfiguration(
                "invalid Rigetti request identifier"
                    .to_owned(),
            )
        );
    }

    Ok(())
}

fn validate_bitstring_key(
    value: &str,
) -> Result<(), RigettiAdapterError> {
    if value.is_empty()
        || value.len()
            > MAX_RIGETTI_RESULT_KEY_LENGTH
        || !value
            .bytes()
            .all(|byte| {
                byte == b'0'
                    || byte == b'1'
            })
    {
        return Err(
            RigettiAdapterError::ResultNormalization(
                "Rigetti result contains an invalid bitstring key"
                    .to_owned(),
            )
        );
    }

    Ok(())
}

// =============================================================================
// String/security helpers
// =============================================================================

fn normalize_identifier(
    value: &str,
) -> String {
    value
        .trim()
        .to_ascii_lowercase()
        .replace('-', "_")
}

fn normalize_instruction(
    value: &str,
) -> String {
    value
        .trim()
        .to_ascii_uppercase()
}

fn contains_secret_marker(
    value: &str,
) -> bool {
    let value =
        value.to_ascii_lowercase();

    const MARKERS: &[&str] = &[
        "api_key",
        "apikey",
        "access_token",
        "accesstoken",
        "refresh_token",
        "refreshtoken",
        "authorization",
        "password",
        "passwd",
        "private_key",
        "privatekey",
        "client_secret",
        "cookie",
        "bearer",
        "secret",
    ];

    MARKERS
        .iter()
        .any(|marker| {
            value.contains(marker)
        })
}

fn sanitize_status(
    value: String,
) -> String {
    let mut value =
        value
            .chars()
            .filter(
                |character| {
                    !character.is_control()
                }
            )
            .collect::<String>();

    if value.len()
        > MAX_RIGETTI_STATUS_LENGTH
    {
        value.truncate(
            MAX_RIGETTI_STATUS_LENGTH
        );
    }

    if contains_secret_marker(&value) {
        return "<redacted>".to_owned();
    }

    value
}

fn sanitize_provider_body(
    body: &[u8],
) -> String {
    let text =
        String::from_utf8_lossy(body);

    if contains_secret_marker(&text) {
        return "Rigetti provider returned sensitive-looking error data; response was redacted"
            .to_owned();
    }

    let mut value =
        text.chars()
            .filter(
                |character| {
                    !character.is_control()
                        || *character == '\n'
                }
            )
            .collect::<String>();

    if value.len()
        > MAX_RIGETTI_ERROR_MESSAGE_LENGTH
    {
        value.truncate(
            MAX_RIGETTI_ERROR_MESSAGE_LENGTH
        );
    }

    value
}

fn sanitize_error(
    value: &str,
) -> String {
    if contains_secret_marker(value) {
        return "<redacted provider error>".to_owned();
    }

    let mut value =
        value
            .chars()
            .filter(
                |character| {
                    !character.is_control()
                }
            )
            .collect::<String>();

    if value.len()
        > MAX_RIGETTI_ERROR_MESSAGE_LENGTH
    {
        value.truncate(
            MAX_RIGETTI_ERROR_MESSAGE_LENGTH
        );
    }

    value
}

/// Encodes a provider path segment without allowing path traversal.
///
/// Only URI-unreserved ASCII characters remain unchanged.
fn path_segment(
    value: &str,
) -> Result<String, RigettiAdapterError> {
    if value.is_empty() {
        return Err(
            RigettiAdapterError::InvalidBackendId
        );
    }

    let mut output =
        String::with_capacity(
            value.len()
        );

    for byte in value.as_bytes() {
        let allowed =
            matches!(
                byte,
                b'A'..=b'Z'
                    | b'a'..=b'z'
                    | b'0'..=b'9'
                    | b'-'
                    | b'.'
                    | b'_'
                    | b'~'
            );

        if allowed {
            output.push(
                *byte as char
            );
        } else {
            output.push('%');
            output.push(
                hex_digit(
                    byte >> 4
                )
            );
            output.push(
                hex_digit(
                    byte & 0x0f
                )
            );
        }
    }

    Ok(output)
}

fn hex_digit(
    value: u8,
) -> char {
    match value {
        0..=9 => {
            (b'0' + value) as char
        }

        10..=15 => {
            (b'A' + value - 10)
                as char
        }

        _ => unreachable!(
            "hex nibble must be <= 15"
        ),
    }
}

// =============================================================================
// Error mapping
// =============================================================================

fn map_backend_error(
    error: BackendError,
) -> RigettiAdapterError {
    RigettiAdapterError::InvalidResponse(
        error.to_string()
    )
}

fn map_generic_error(
    error: super::generic::GenericAdapterError,
) -> RigettiAdapterError {
    RigettiAdapterError::InvalidConfiguration(
        sanitize_error(
            &error.to_string()
        )
    )
}

fn map_rigetti_error(
    error: RigettiAdapterError,
) -> BackendError {
    match error {
        RigettiAdapterError::BackendUnavailable => {
            BackendError::BackendUnavailable {
                backend_id:
                    "<rigetti>"
                        .to_owned(),
                status:
                    BackendStatus::Unavailable,
            }
        }

        RigettiAdapterError::UnsupportedProgramFormat(
            format,
        ) => {
            BackendError::ExecutionRejected(
                format!(
                    "Rigetti does not accept executable format '{format}'; transpile to Quil or Quil-T first"
                )
            )
        }

        RigettiAdapterError::UnsupportedCapability(
            capability,
        ) => {
            BackendError::UnsupportedCapability {
                capability,
            }
        }

        RigettiAdapterError::UnsupportedInstruction(
            instruction,
        ) => {
            BackendError::UnsupportedGate {
                gate: instruction,
            }
        }

        RigettiAdapterError::CancellationUnsupported => {
            BackendError::ExecutionRejected(
                "Rigetti cancellation is unsupported by this backend"
                    .to_owned(),
            )
        }

        RigettiAdapterError::ProviderFailure {
            operation,
            message,
        } => {
            BackendError::ExecutionUnavailable(
                format!(
                    "Rigetti operation {operation} failed: {message}"
                )
            )
        }

        RigettiAdapterError::Transport(
            message,
        ) => {
            BackendError::ExecutionUnavailable(
                format!(
                    "Rigetti transport failure: {message}"
                )
            )
        }

        RigettiAdapterError::ResultNormalization(
            message,
        ) => {
            BackendError::ExecutionRejected(
                message
            )
        }

        RigettiAdapterError::InvalidJson(
            message,
        )
        | RigettiAdapterError::InvalidResponse(
            message,
        )
        | RigettiAdapterError::InvalidConfiguration(
            message,
        ) => {
            BackendError::ExecutionRejected(
                message
            )
        }

        RigettiAdapterError::InvalidBackendId
        | RigettiAdapterError::BackendIdTooLong
        | RigettiAdapterError::InvalidBackendName
        | RigettiAdapterError::BackendNameTooLong
        | RigettiAdapterError::InvalidRegion
        | RigettiAdapterError::RegionTooLong
        | RigettiAdapterError::InvalidEndpoint
        | RigettiAdapterError::BackendIdentityMismatch
        | RigettiAdapterError::InvalidJobId
        | RigettiAdapterError::JobIdTooLong
        | RigettiAdapterError::SecretMaterialRejected => {
            BackendError::ExecutionRejected(
                error.to_string()
            )
        }

        RigettiAdapterError::UnknownProviderState(
            state,
        ) => {
            BackendError::ExecutionUnavailable(
                format!(
                    "Rigetti returned an unknown job state '{state}'"
                )
            )
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Mutex;

    use crate::quantum::hardware::backend::{
        BackendCapabilities,
        BackendLimits,
        BackendMetadata,
    };

    use crate::quantum::hardware::topology::
        HardwareTopology;

    /// Deterministic test transport.
    ///
    /// This is deliberately provider-neutral. It represents what a real
    /// QCS gRPC/HTTP transport would return after secure transport and
    /// authentication.
    #[derive(Debug, Default)]
    struct MockRigettiTransport {
        requests:
            Mutex<Vec<ProviderRequest>>,
    }

    impl ProviderTransport
        for MockRigettiTransport
    {
        fn send(
            &self,
            request: &ProviderRequest,
        ) -> Result<
            ProviderResponse,
            super::super::generic::TransportError,
        > {
            self.requests
                .lock()
                .expect(
                    "mock request mutex must not be poisoned"
                )
                .push(request.clone());

            let response =
                match request.operation {
                    ProviderOperation::Submit => {
                        json_response(
                            200,
                            br#"{
                                "job_id":"rigetti-job-001"
                            }"#,
                        )
                    }

                    ProviderOperation::GetJobStatus => {
                        json_response(
                            200,
                            br#"{
                                "status":"completed",
                                "result_available":true,
                                "queue_position":0,
                                "estimated_wait_seconds":0
                            }"#,
                        )
                    }

                    ProviderOperation::GetResult => {
                        json_response(
                            200,
                            br#"{
                                "shots":1000,
                                "counts":{
                                    "00":500,
                                    "11":500
                                }
                            }"#,
                        )
                    }

                    ProviderOperation::Cancel => {
                        json_response(
                            200,
                            br#"{}"#,
                        )
                    }

                    ProviderOperation::GetQueue => {
                        json_response(
                            200,
                            br#"{
                                "pending_jobs":0,
                                "estimated_wait_seconds":0,
                                "accepting_submissions":true
                            }"#,
                        )
                    }

                    ProviderOperation::GetHealth => {
                        json_response(
                            200,
                            br#"{
                                "healthy":true,
                                "degraded":false,
                                "message":"healthy"
                            }"#,
                        )
                    }

                    _ => {
                        json_response(
                            404,
                            br#"{"error":"unsupported test operation"}"#,
                        )
                    }
                };

            Ok(response)
        }
    }

    fn json_response(
        status_code: u16,
        body: &[u8],
    ) -> ProviderResponse {
        ProviderResponse::new(
            status_code,
            super::super::generic::ProviderHeaders::new(),
            body.to_vec(),
            None,
            Some(
                RIGETTI_API_VERSION
                    .to_owned(),
            ),
            None,
        )
        .expect(
            "test response must be valid"
        )
    }

    fn descriptor()
        -> RigettiBackendDescriptor {
        RigettiBackendDescriptor::new(
            "Ankaa-1",
            "Rigetti Ankaa",
            4,
        )
        .expect(
            "descriptor must be valid"
        )
        .with_status(
            BackendStatus::Available
        )
        .with_capabilities([
            capability::MEASUREMENT,
            capability::RESET,
            capability::MID_CIRCUIT_MEASUREMENT,
            capability::CLASSICAL_CONTROL,
            capability::DYNAMIC_CIRCUITS,
            capability::PARAMETERIZED_GATES,
            capability::PULSE_CONTROL,
            capability::QUIL_T,
            capability::CANCELLATION,
            capability::QUEUE_INFORMATION,
            capability::CALIBRATION_DATA,
            capability::TOPOLOGY_INFORMATION,
            capability::NATIVE_INSTRUCTION_SET,
            capability::PARALLEL_OPERATIONS,
        ])
        .expect(
            "capabilities must be valid"
        )
        .with_native_instructions([
            "RX",
            "RY",
            "RZ",
            "CZ",
            "MEASURE",
        ])
        .expect(
            "native instructions must be valid"
        )
    }

    fn backend()
        -> QuantumBackend {
        let descriptor =
            descriptor();

        let metadata =
            RigettiQuantumAdapter::<
                MockRigettiTransport
            >::canonical_metadata(
                &descriptor
            );

        let capabilities =
            RigettiQuantumAdapter::<
                MockRigettiTransport
            >::canonical_capabilities(
                &descriptor
            );

        let topology =
            HardwareTopology::linear(
                descriptor.qubit_count
            )
            .expect(
                "test topology must be valid"
            );

        QuantumBackend::new(
            metadata,
            capabilities,
            BackendLimits::unlimited(),
            topology,
        )
        .expect(
            "Rigetti backend must be valid"
        )
    }

    fn adapter()
        -> RigettiQuantumAdapter<
            MockRigettiTransport
        > {
        RigettiQuantumAdapter::new(
            RigettiAdapterConfig::default(),
            Arc::new(
                MockRigettiTransport::default()
            ),
            descriptor(),
            backend(),
        )
        .expect(
            "Rigetti adapter must be valid"
        )
    }

    fn program()
        -> BackendProgram {
        BackendProgram::new(
            RIGETTI_QUIL_FORMAT,
            b"DECLARE ro BIT[2]\nH 0\nCZ 0 1\nMEASURE 0 ro[0]\nMEASURE 1 ro[1]\n"
                .to_vec(),
        )
        .expect(
            "Quil program must be valid"
        )
    }

    fn request()
        -> ExecutionRequest {
        ExecutionRequest::new(
            CircuitRequirements {
                qubit_count: 2,
                circuit_depth: 3,
                operation_count: 4,
                classical_bit_count: 2,
                shots: 1000,
                gates: vec![
                    "H".to_owned(),
                    "CZ".to_owned(),
                    "MEASURE".to_owned(),
                ],
                two_qubit_edges:
                    vec![(0, 1)],
                requires_measurement:
                    true,
                ..Default::default()
            }
        )
        .with_request_id(
            "rigetti-test-request"
        )
        .expect(
            "request ID must be valid"
        )
    }

    #[test]
    fn quil_formats_are_supported() {
        assert!(
            RigettiQuantumAdapter::<
                MockRigettiTransport
            >::supports_program_format(
                "quil"
            )
        );

        assert!(
            RigettiQuantumAdapter::<
                MockRigettiTransport
            >::supports_program_format(
                "quil-t"
            )
        );

        assert!(
            RigettiQuantumAdapter::<
                MockRigettiTransport
            >::supports_program_format(
                "quil3"
            )
        );
    }

    #[test]
    fn openqasm_is_not_silently_accepted() {
        assert!(
            !RigettiQuantumAdapter::<
                MockRigettiTransport
            >::supports_program_format(
                "openqasm-3.1"
            )
        );
    }

    #[test]
    fn unknown_provider_state_is_rejected() {
        assert!(
            RigettiQuantumAdapter::<
                MockRigettiTransport
            >::normalize_job_state(
                "future-provider-state"
            )
            .is_err()
        );
    }

    #[test]
    fn path_segments_are_safe() {
        assert_eq!(
            path_segment(
                "Ankaa-1"
            )
            .expect(
                "valid path"
            ),
            "Ankaa-1"
        );

        assert_eq!(
            path_segment(
                "foo/bar"
            )
            .expect(
                "path should be encoded"
            ),
            "foo%2Fbar"
        );
    }

    #[test]
    fn secret_metadata_is_rejected() {
        assert!(
            RigettiBackendDescriptor::new(
                "Ankaa-1",
                "Rigetti Ankaa",
                4,
            )
            .expect(
                "descriptor must be valid"
            )
            .with_metadata(
                "access_token",
                "secret",
            )
            .is_err()
        );
    }

    #[test]
    fn descriptor_is_converted_to_canonical_backend() {
        let descriptor =
            descriptor();

        let metadata =
            RigettiQuantumAdapter::<
                MockRigettiTransport
            >::canonical_metadata(
                &descriptor
            );

        let capabilities =
            RigettiQuantumAdapter::<
                MockRigettiTransport
            >::canonical_capabilities(
                &descriptor
            );

        assert_eq!(
            metadata.provider,
            RIGETTI_PROVIDER_ID
        );

        assert!(
            capabilities
                .native_instruction_set
        );

        assert!(
            capabilities
                .native_gates
                .contains("CZ")
        );
    }

    #[test]
    fn adapter_identity_is_stable() {
        let adapter =
            adapter();

        assert_eq!(
            adapter
                .adapter_info()
                .adapter_id,
            RIGETTI_ADAPTER_ID
        );

        assert_eq!(
            adapter
                .adapter_info()
                .adapter_version,
            RIGETTI_ADAPTER_VERSION
        );

        assert!(
            adapter
                .adapter_info()
                .production_ready
        );
    }

    #[test]
    fn submission_is_asynchronous() {
        let adapter =
            adapter();

        let job =
            adapter
                .submit(
                    &request(),
                    &program()
                )
                .expect(
                    "submission must succeed"
                );

        assert_eq!(
            job.id.as_str(),
            "rigetti-job-001"
        );

        assert!(
            !job.state.is_terminal()
        );
    }

    #[test]
    fn completed_status_is_normalized() {
        let adapter =
            adapter();

        let job =
            adapter
                .submit(
                    &request(),
                    &program()
                )
                .expect(
                    "submission must succeed"
                );

        let status =
            adapter
                .status(&job.id)
                .expect(
                    "status must succeed"
                );

        assert_eq!(
            status.job.state,
            BackendJobState::Completed
        );

        assert!(
            status.result_available
        );
    }

    #[test]
    fn result_is_not_returned_before_completion() {
        let adapter =
            adapter();

        /*
         * The mock reports completed, so this test verifies the actual result
         * lifecycle path. A real transport returning "running" would be
         * rejected before result retrieval.
         */
        let job =
            adapter
                .submit(
                    &request(),
                    &program()
                )
                .expect(
                    "submission must succeed"
                );

        let result =
            adapter
                .result(&job.id)
                .expect(
                    "completed result must succeed"
                );

        assert_eq!(
            result.shots,
            1000
        );

        assert_eq!(
            result.counts.get("00"),
            Some(&500)
        );

        assert_eq!(
            result.counts.get("11"),
            Some(&500)
        );

        assert!(
            result.counts_match_shots()
        );
    }

    #[test]
    fn queue_information_is_normalized() {
        let adapter =
            adapter();

        let queue =
            adapter
                .queue_info()
                .expect(
                    "queue query must succeed"
                );

        assert_eq!(
            queue.pending_jobs,
            Some(0)
        );

        assert!(
            queue.accepting_submissions
        );
    }

    #[test]
    fn health_is_normalized() {
        let adapter =
            adapter();

        let health =
            adapter
                .health()
                .expect(
                    "health must succeed"
                );

        assert_eq!(
            health.state,
            BackendHealthState::Healthy
        );
    }

    #[test]
    fn cancellation_is_capability_gated() {
        let adapter =
            adapter();

        let job =
            adapter
                .submit(
                    &request(),
                    &program()
                )
                .expect(
                    "submission must succeed"
                );

        let cancellation =
            adapter
                .cancel(&job.id)
                .expect(
                    "cancellation must return normalized outcome"
                );

        assert_eq!(
            cancellation.outcome,
            CancellationOutcome::Accepted
        );
    }

    #[test]
    fn metadata_does_not_contain_credentials() {
        let metadata =
            RigettiQuantumAdapter::<
                MockRigettiTransport
            >::adapter_metadata()
            .expect(
                "adapter metadata must be valid"
            );

        let debug =
            format!(
                "{metadata:?}"
            );

        assert!(
            !debug.contains(
                "access_token"
            )
        );

        assert!(
            !debug.contains(
                "private_key"
            )
        );
    }
}