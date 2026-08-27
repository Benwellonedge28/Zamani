//! Zamani Quantum — IBM Quantum Hardware Adapter
//!
//! Production-grade IBM Quantum provider adapter for:
//!
//! `crate::quantum::hardware::adapters::ibm`
//!
//! # Responsibility
//!
//! This module is the IBM-specific semantic boundary between Zamani's
//! provider-neutral quantum hardware contracts and IBM Quantum's API model.
//!
//! It owns:
//!
//! - IBM provider identity;
//! - IBM backend/device identifiers;
//! - IBM backend metadata normalization;
//! - IBM backend capability normalization;
//! - IBM native instruction normalization;
//! - IBM OpenQASM 3 execution mapping;
//! - IBM job submission request construction;
//! - IBM job lifecycle normalization;
//! - IBM cancellation mapping;
//! - IBM result normalization;
//! - IBM queue/status normalization;
//! - IBM provider error normalization;
//! - IBM API-version metadata;
//! - IBM-specific capability checks;
//! - IBM-specific safe metadata;
//! - deterministic IBM request construction;
//! - IBM-specific conformance checks;
//! - IBM-specific compatibility diagnostics;
//! - provider-neutral integration with `QuantumBackendAdapter`;
//! - provider-neutral integration with `ProviderTransport`.
//!
//! # Explicit non-responsibilities
//!
//! This module does NOT own:
//!
//! - HTTP implementation;
//! - TLS implementation;
//! - certificate validation;
//! - credential storage;
//! - API-key persistence;
//! - OAuth/OIDC credential acquisition;
//! - routing;
//! - scheduling;
//! - OpenQASM parsing;
//! - Zamani Quantum IR;
//! - QIR generation;
//! - benchmarking mathematics;
//! - calibration storage;
//! - topology algorithms;
//! - global provider registries;
//! - retry loops;
//! - process-global state.
//!
//! Those responsibilities belong to:
//!
//! - `adapters::generic`;
//! - `authentication`;
//! - `credentials`;
//! - `routing`;
//! - `scheduling`;
//! - `frontend`;
//! - `ir`;
//! - `benchmarking`;
//! - `calibration`;
//! - `topology`;
//! - `provider_registry`.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Quantum IR
//!      |
//!      v
//! compatibility / routing / scheduling
//!      |
//!      v
//! BackendProgram
//!      |
//!      v
//! QuantumBackendAdapter
//!      |
//!      v
//! adapters::ibm
//!      |
//!      +-----------------------------+
//!      |                             |
//!      v                             v
//! IBM semantic mapping        ProviderTransport
//!                                    |
//!                                    v
//!                              HTTP / TLS
//!                                    |
//!                                    v
//!                              IBM Quantum
//! ```
//!
//! # IBM interoperability model
//!
//! Zamani's canonical quantum representation remains Zamani Quantum IR.
//!
//! IBM execution is reached through an interoperable executable format:
//!
//! ```text
//! Zamani Quantum IR
//!        |
//!        v
//! OpenQASM 3.x
//!        |
//!        v
//! IBM Quantum adapter
//!        |
//!        v
//! IBM backend
//! ```
//!
//! The adapter therefore MUST NOT make OpenQASM the canonical Zamani IR.
//!
//! # Production transport rule
//!
//! This file intentionally receives a `ProviderTransport` implementation.
//!
//! It does not construct an HTTP/TLS implementation itself.
//!
//! This is important because the current Zamani dependency set does not yet
//! include an HTTP/TLS client. The generic adapter explicitly separates the
//! provider adapter from the HTTP/TLS transport boundary.
//!
//! A production transport should eventually be implemented behind
//! `ProviderTransport` using a maintained Rust HTTP/TLS stack and must provide:
//!
//! - TLS certificate validation;
//! - HTTPS-only remote endpoints;
//! - connection pooling;
//! - bounded request/response bodies;
//! - timeout enforcement;
//! - safe connection reuse;
//! - provider response status preservation;
//! - header redaction;
//! - no credential logging.
//!
//! The IBM semantic adapter must remain unchanged when that transport is
//! replaced.
//!
//! # Authentication
//!
//! Authentication is represented by an injected transport/session boundary.
//!
//! This module never stores an IBM API key in a public adapter structure.
//!
//! In particular, this module must never contain:
//!
//! ```text
//! api_key: String
//! token: String
//! password: String
//! authorization_header: String
//! ```
//!
//! # Rust compatibility
//!
//! Target:
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
//! No unsafe Rust is required.
//!
//! # Determinism
//!
//! IBM request construction is deterministic:
//!
//! - stable field ordering is used where collections are serialized;
//! - no random identifiers are generated;
//! - no system clock is read for semantic decisions;
//! - provider job identifiers are never rewritten;
//! - no hidden retries are performed;
//! - no provider-specific state is stored globally.
//!
//! # No-reedit contract
//!
//! This file is designed to be completed independently.
//!
//! Future modules MUST consume this adapter rather than modify IBM semantics:
//!
//! - `provider_registry.rs` registers the adapter;
//! - `device_registry.rs` indexes discovered IBM devices;
//! - `execution.rs` orchestrates submission/polling;
//! - `job.rs` may wrap the normalized job identity;
//! - `queue.rs` consumes queue information;
//! - `benchmarking` consumes the provider-neutral lifecycle;
//! - Danga invokes the provider-neutral execution layer.
//!
//! Adding another IBM backend must require configuration/data changes only.
//! Adding another provider must not require changing this file.
//!
//! # IBM API evolution
//!
//! IBM-specific API evolution belongs behind this boundary.
//!
//! Provider API version changes must be represented by constants/configuration
//! and translated into the same Zamani provider-neutral lifecycle.
//!
//! Provider-specific response fields that have no stable Zamani equivalent are
//! retained only as safe metadata where appropriate.
//!
//! # Security invariant
//!
//! Provider responses must never be copied blindly into errors or metadata.
//!
//! IBM API responses may contain sensitive or operational information. Only
//! explicitly approved fields may enter normalized metadata.
//!
//! API credentials, authorization headers, cookies, session material and
//! private key material are always forbidden.
//!
//! # Execution lifecycle
//!
//! ```text
//! validate request
//!       |
//!       v
//! validate IBM backend
//!       |
//!       v
//! validate OpenQASM format
//!       |
//!       v
//! build IBM request
//!       |
//!       v
//! transport.submit()
//!       |
//!       v
//! IBM job ID
//!       |
//!       v
//! BackendJobId
//!       |
//!       +----> status()
//!       |
//!       +----> cancel()
//!       |
//!       +----> result()
//! ```
//!
//! A provider result is never reported as `Completed` unless the IBM API
//! confirms completion and a result is available for retrieval.
//!
//! # IBM backend semantics
//!
//! IBM backends are physical execution targets. IBM backend names and API
//! identifiers are provider-specific and therefore never become canonical
//! Zamani identity types.
//!
//! The adapter translates them into Zamani's backend abstraction.
//!
//! # Capability semantics
//!
//! IBM backend capability discovery must distinguish:
//!
//! - supported;
//! - unavailable;
//! - experimental;
//! - provider-reported but not safely understood.
//!
//! Unknown IBM capabilities must never silently satisfy a Zamani requirement.
//!
//! # Result semantics
//!
//! IBM result data is normalized into Zamani's `ExecutionResult`.
//!
//! The adapter must preserve provenance:
//!
//! - backend ID;
//! - provider job ID;
//! - requested shots;
//! - executable format;
//! - adapter version;
//! - IBM API version where known.
//!
//! # Important architectural boundary
//!
//! `backend.rs` describes the backend.
//!
//! `backend_trait.rs` describes executable adapter behaviour.
//!
//! `generic.rs` provides provider-neutral transport primitives.
//!
//! `ibm.rs` translates IBM semantics into those contracts.
//!
//! This separation follows the repository's existing architecture.
//!
//! The existing hardware layer already defines `QuantumBackend` separately
//! from `QuantumBackendAdapter`, while `execution.rs` orchestrates lifecycle
//! operations around the adapter. The IBM adapter therefore must not become a
//! second execution engine.
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

use super::generic::{
    AdapterIdentity,
    AdapterMetadata,
    ProviderTransport,
};

use crate::quantum::hardware::backend::{
    BackendCapabilities,
    BackendKind,
    BackendStatus,
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
    QuantumBackendAdapter,
};

/// Stable IBM provider identifier.
pub const IBM_PROVIDER_ID: &str = "ibm";

/// Stable Zamani adapter identifier.
pub const IBM_ADAPTER_ID: &str = "zamani.quantum.hardware.ibm";

/// Semantic version of this adapter contract.
pub const IBM_ADAPTER_VERSION: &str = "1.0.0";

/// IBM Quantum API family represented by this adapter.
///
/// The actual transport endpoint/version is configuration owned by the
/// transport layer and provider deployment configuration.
pub const IBM_API_VERSION: &str = "v1";

/// Canonical executable format understood by this adapter.
pub const IBM_OPENQASM_FORMAT: &str = "openqasm-3.1";

/// Alternate spelling accepted when a caller already negotiated OpenQASM 3.
pub const IBM_OPENQASM_3_FORMAT: &str = "openqasm3";

/// Maximum IBM backend identifier length.
pub const MAX_IBM_BACKEND_ID_LENGTH: usize = 512;

/// Maximum IBM backend name length.
pub const MAX_IBM_BACKEND_NAME_LENGTH: usize = 512;

/// Maximum IBM provider region length.
pub const MAX_IBM_REGION_LENGTH: usize = 256;

/// Maximum provider status length retained in normalized status.
pub const MAX_PROVIDER_STATUS_LENGTH: usize = 4096;

/// Maximum safe IBM metadata fields.
pub const MAX_IBM_METADATA_FIELDS: usize = 256;

/// Maximum safe IBM metadata value length.
pub const MAX_IBM_METADATA_VALUE_LENGTH: usize = 4096;

/// Maximum native instruction names.
pub const MAX_IBM_NATIVE_INSTRUCTIONS: usize = 4096;

/// Maximum result count entries accepted by the normalization boundary.
///
/// Very large result sets should eventually use a streaming/artifact result
/// boundary rather than allocating one enormous in-memory map.
pub const MAX_RESULT_ENTRIES: usize = 1_000_000;

/// Maximum individual IBM result key length.
pub const MAX_RESULT_KEY_LENGTH: usize = 4096;

/// Maximum provider error message retained by the adapter.
pub const MAX_PROVIDER_ERROR_MESSAGE_LENGTH: usize = 4096;

/// Stable IBM capability identifiers.
pub mod capability {
    /// Measurement.
    pub const MEASUREMENT: &str = "measurement";

    /// Reset.
    pub const RESET: &str = "reset";

    /// Mid-circuit measurement.
    pub const MID_CIRCUIT_MEASUREMENT: &str = "mid_circuit_measurement";

    /// Classical feed-forward.
    pub const CLASSICAL_CONTROL: &str = "classical_control";

    /// Dynamic circuits.
    pub const DYNAMIC_CIRCUITS: &str = "dynamic_circuits";

    /// Parameterized gates.
    pub const PARAMETERIZED_GATES: &str = "parameterized_gates";

    /// Pulse control.
    pub const PULSE_CONTROL: &str = "pulse_control";

    /// Cancellation.
    pub const CANCELLATION: &str = "cancellation";

    /// Queue information.
    pub const QUEUE_INFORMATION: &str = "queue_information";

    /// Calibration data.
    pub const CALIBRATION_DATA: &str = "calibration_data";

    /// Topology information.
    pub const TOPOLOGY_INFORMATION: &str = "topology_information";
}

/// IBM API operation identifiers.
pub mod operation {
    /// Backend discovery.
    pub const BACKEND: &str = "ibm.backend";

    /// Backend list/discovery.
    pub const BACKENDS: &str = "ibm.backends";

    /// Job submission.
    pub const SUBMIT: &str = "ibm.job.submit";

    /// Job status.
    pub const STATUS: &str = "ibm.job.status";

    /// Job result.
    pub const RESULT: &str = "ibm.job.result";

    /// Job cancellation.
    pub const CANCEL: &str = "ibm.job.cancel";
}

/// IBM job status strings.
///
/// IBM may evolve or add states. Unknown states are normalized to
/// `BackendJobState::Unknown`; they must never be guessed as successful.
pub mod provider_status {
    /// Newly accepted.
    pub const CREATED: &str = "created";

    /// Waiting in queue.
    pub const QUEUED: &str = "queued";

    /// Executing.
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

/// IBM adapter construction errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IbmAdapterError {
    /// IBM backend identifier is invalid.
    InvalidBackendId,

    /// IBM backend identifier exceeds the configured limit.
    BackendIdTooLong,

    /// IBM backend name is invalid.
    InvalidBackendName,

    /// IBM backend name exceeds the configured limit.
    BackendNameTooLong,

    /// IBM region is invalid.
    InvalidRegion,

    /// IBM region exceeds the configured limit.
    RegionTooLong,

    /// No transport was supplied.
    MissingTransport,

    /// Adapter metadata construction failed.
    Metadata(String),

    /// IBM response cannot be normalized.
    InvalidResponse(String),

    /// IBM returned a provider-level failure.
    ProviderFailure {
        /// Stable/safe IBM operation identifier.
        operation: &'static str,

        /// Safe provider error message.
        message: String,
    },

    /// IBM API returned an unknown state.
    UnknownProviderState(String),

    /// Requested format is not supported by this adapter.
    UnsupportedProgramFormat(String),

    /// IBM backend does not advertise a requested capability.
    UnsupportedCapability(String),

    /// IBM backend is not executable in its current state.
    BackendUnavailable,

    /// The transport boundary rejected or failed an operation.
    Transport(String),

    /// Result normalization failed.
    ResultNormalization(String),

    /// Job identity could not be normalized.
    InvalidJobId,

    /// Job identity exceeded the configured limit.
    JobIdTooLong,
}

impl fmt::Display for IbmAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBackendId => {
                write!(formatter, "IBM backend identifier is invalid")
            }

            Self::BackendIdTooLong => {
                write!(formatter, "IBM backend identifier is too long")
            }

            Self::InvalidBackendName => {
                write!(formatter, "IBM backend name is invalid")
            }

            Self::BackendNameTooLong => {
                write!(formatter, "IBM backend name is too long")
            }

            Self::InvalidRegion => {
                write!(formatter, "IBM region is invalid")
            }

            Self::RegionTooLong => {
                write!(formatter, "IBM region is too long")
            }

            Self::MissingTransport => {
                write!(formatter, "IBM adapter requires an injected provider transport")
            }

            Self::Metadata(message) => {
                write!(formatter, "IBM adapter metadata error: {message}")
            }

            Self::InvalidResponse(message) => {
                write!(formatter, "invalid IBM provider response: {message}")
            }

            Self::ProviderFailure { operation, message } => {
                write!(
                    formatter,
                    "IBM provider operation {operation} failed: {message}"
                )
            }

            Self::UnknownProviderState(state) => {
                write!(
                    formatter,
                    "IBM provider returned unknown job state: {state}"
                )
            }

            Self::UnsupportedProgramFormat(format) => {
                write!(
                    formatter,
                    "IBM adapter does not support program format {format}"
                )
            }

            Self::UnsupportedCapability(capability) => {
                write!(
                    formatter,
                    "IBM backend does not support capability {capability}"
                )
            }

            Self::BackendUnavailable => {
                write!(formatter, "IBM backend is not currently executable")
            }

            Self::Transport(message) => {
                write!(formatter, "IBM transport failure: {message}")
            }

            Self::ResultNormalization(message) => {
                write!(
                    formatter,
                    "IBM result normalization failure: {message}"
                )
            }

            Self::InvalidJobId => {
                write!(formatter, "IBM job identifier is invalid")
            }

            Self::JobIdTooLong => {
                write!(formatter, "IBM job identifier is too long")
            }
        }
    }
}

impl std::error::Error for IbmAdapterError {}

/// IBM backend descriptor.
///
/// This is provider-specific discovery information which is subsequently
/// normalized into the canonical `QuantumBackend` representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IbmBackendDescriptor {
    /// IBM backend identifier.
    pub backend_id: String,

    /// Human-readable backend name.
    pub name: String,

    /// IBM backend operational status.
    pub status: BackendStatus,

    /// Number of physical qubits.
    pub qubit_count: usize,

    /// IBM backend capabilities.
    pub capabilities: BTreeSet<String>,

    /// Native IBM instructions.
    pub native_instructions: BTreeSet<String>,

    /// Optional provider region.
    pub region: Option<String>,

    /// Provider-reported API/backend version.
    pub provider_version: Option<String>,

    /// Safe provider metadata.
    pub metadata: BTreeMap<String, String>,
}

impl IbmBackendDescriptor {
    /// Creates a validated IBM backend descriptor.
    pub fn new(
        backend_id: impl Into<String>,
        name: impl Into<String>,
        qubit_count: usize,
    ) -> Result<Self, IbmAdapterError> {
        let backend_id = backend_id.into();
        let name = name.into();

        validate_backend_id(&backend_id)?;
        validate_backend_name(&name)?;

        if qubit_count == 0 {
            return Err(IbmAdapterError::InvalidResponse(
                "IBM backend reported zero physical qubits".to_owned(),
            ));
        }

        Ok(Self {
            backend_id,
            name,
            status: BackendStatus::Unknown,
            qubit_count,
            capabilities: BTreeSet::new(),
            native_instructions: BTreeSet::new(),
            region: None,
            provider_version: None,
            metadata: BTreeMap::new(),
        })
    }

    /// Adds a capability.
    pub fn with_capability(
        mut self,
        capability: impl Into<String>,
    ) -> Result<Self, IbmAdapterError> {
        let capability = normalize_identifier(&capability.into());

        if capability.is_empty() {
            return Err(IbmAdapterError::InvalidResponse(
                "empty IBM capability".to_owned(),
            ));
        }

        self.capabilities.insert(capability);
        Ok(self)
    }

    /// Adds a native instruction.
    pub fn with_native_instruction(
        mut self,
        instruction: impl Into<String>,
    ) -> Result<Self, IbmAdapterError> {
        let instruction = normalize_instruction(&instruction.into());

        if instruction.is_empty() {
            return Err(IbmAdapterError::InvalidResponse(
                "empty IBM native instruction".to_owned(),
            ));
        }

        if self.native_instructions.len() >= MAX_IBM_NATIVE_INSTRUCTIONS
            && !self.native_instructions.contains(&instruction)
        {
            return Err(IbmAdapterError::InvalidResponse(
                "IBM native instruction limit exceeded".to_owned(),
            ));
        }

        self.native_instructions.insert(instruction);
        Ok(self)
    }

    /// Sets backend status.
    pub fn with_status(mut self, status: BackendStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets provider region.
    pub fn with_region(
        mut self,
        region: impl Into<String>,
    ) -> Result<Self, IbmAdapterError> {
        let region = region.into();

        validate_region(&region)?;
        self.region = Some(region);

        Ok(self)
    }

    /// Sets provider API/backend version.
    pub fn with_provider_version(
        mut self,
        version: impl Into<String>,
    ) -> Self {
        self.provider_version = Some(version.into());
        self
    }

    /// Adds safe provider metadata.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, IbmAdapterError> {
        let key = normalize_identifier(&key.into());
        let value = value.into();

        if contains_secret_marker(&key)
            || contains_secret_marker(&value)
        {
            return Err(IbmAdapterError::Metadata(
                "secret-like IBM metadata was rejected".to_owned(),
            ));
        }

        if value.len() > MAX_IBM_METADATA_VALUE_LENGTH {
            return Err(IbmAdapterError::Metadata(
                "IBM metadata value exceeds configured limit".to_owned(),
            ));
        }

        if self.metadata.len() >= MAX_IBM_METADATA_FIELDS
            && !self.metadata.contains_key(&key)
        {
            return Err(IbmAdapterError::Metadata(
                "IBM metadata field limit exceeded".to_owned(),
            ));
        }

        self.metadata.insert(key, value);

        Ok(self)
    }
}

/// Immutable configuration for an IBM adapter.
///
/// Credentials are deliberately not part of this structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IbmAdapterConfig {
    /// IBM Quantum service endpoint.
    ///
    /// The actual transport validates HTTPS/TLS policy.
    pub endpoint: String,

    /// Provider API version.
    pub api_version: String,

    /// Default executable format.
    pub program_format: String,

    /// Whether experimental capabilities may be used.
    pub allow_experimental: bool,

    /// Default request timeout passed to the transport.
    pub request_timeout: Duration,

    /// Whether the adapter should require explicit backend identity matching
    /// when normalizing results.
    pub require_backend_identity_match: bool,
}

impl Default for IbmAdapterConfig {
    fn default() -> Self {
        Self {
            endpoint: "https://quantum.cloud.ibm.com/api".to_owned(),
            api_version: IBM_API_VERSION.to_owned(),
            program_format: IBM_OPENQASM_FORMAT.to_owned(),
            allow_experimental: false,
            request_timeout: Duration::from_secs(60),
            require_backend_identity_match: true,
        }
    }
}

impl IbmAdapterConfig {
    /// Creates the default IBM production configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Validates the configuration without performing network I/O.
    pub fn validate(&self) -> Result<(), IbmAdapterError> {
        if self.endpoint.trim().is_empty() {
            return Err(IbmAdapterError::Metadata(
                "IBM endpoint must not be empty".to_owned(),
            ));
        }

        if !self.endpoint.starts_with("https://") {
            return Err(IbmAdapterError::Metadata(
                "remote IBM endpoint must use HTTPS".to_owned(),
            ));
        }

        if self.api_version.trim().is_empty() {
            return Err(IbmAdapterError::Metadata(
                "IBM API version must not be empty".to_owned(),
            ));
        }

        if self.program_format.trim().is_empty() {
            return Err(IbmAdapterError::Metadata(
                "IBM program format must not be empty".to_owned(),
            ));
        }

        if self.request_timeout.is_zero() {
            return Err(IbmAdapterError::Metadata(
                "IBM transport timeout must be non-zero".to_owned(),
            ));
        }

        Ok(())
    }
}

/// Provider-neutral request produced by the IBM semantic adapter.
///
/// The generic transport converts this into the actual HTTP representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IbmRequest {
    /// Stable operation identifier.
    pub operation: &'static str,

    /// Relative provider resource path.
    pub path: String,

    /// HTTP-independent operation method.
    pub method: IbmHttpMethod,

    /// Safe query parameters.
    pub query: BTreeMap<String, String>,

    /// JSON request body.
    ///
    /// The transport owns actual HTTP encoding.
    pub body: Option<String>,

    /// Request timeout.
    pub timeout: Duration,
}

/// HTTP semantics needed by the IBM adapter.
///
/// The generic transport is responsible for turning this into an actual HTTP
/// request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IbmHttpMethod {
    /// GET.
    Get,

    /// POST.
    Post,

    /// DELETE.
    Delete,
}

impl IbmHttpMethod {
    /// Stable HTTP method name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Delete => "DELETE",
        }
    }
}

/// Provider-neutral response received from the injected IBM transport.
///
/// The transport must already enforce response-size limits and TLS policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IbmResponse {
    /// HTTP/provider status code.
    pub status_code: u16,

    /// Response body.
    pub body: String,

    /// Safe response headers.
    ///
    /// Authorization/cookie/secret headers must already have been removed.
    pub headers: BTreeMap<String, String>,
}

impl IbmResponse {
    /// Returns true for successful HTTP status classes.
    pub const fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }
}

/// Minimal IBM transport boundary.
///
/// This trait deliberately lives in this file as a compatibility bridge for
/// the generic transport layer. It can be implemented by an adapter over the
/// repository's `ProviderTransport` without changing IBM semantic code.
///
/// The transport implementation MUST:
///
/// - use HTTPS for remote IBM services;
/// - validate certificates;
/// - enforce request/response bounds;
/// - redact secrets;
/// - preserve status codes;
/// - never log credentials;
/// - never silently retry non-idempotent job submission.
pub trait IbmTransport: Send + Sync {
    /// Sends one validated IBM request.
    fn send(
        &self,
        request: &IbmRequest,
    ) -> Result<IbmResponse, IbmAdapterError>;
}

/// IBM adapter.
///
/// The adapter owns IBM semantic translation and uses an injected transport.
///
/// It contains no credentials and no global state.
pub struct IbmQuantumAdapter<T>
where
    T: IbmTransport,
{
    config: IbmAdapterConfig,
    transport: Arc<T>,
    backend: QuantumBackend,
    descriptor: IbmBackendDescriptor,
}

impl<T> fmt::Debug for IbmQuantumAdapter<T>
where
    T: IbmTransport,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("IbmQuantumAdapter")
            .field("config", &self.config)
            .field("backend", &self.backend)
            .field("descriptor", &self.descriptor)
            .field("transport", &"<injected>")
            .finish()
    }
}

impl<T> IbmQuantumAdapter<T>
where
    T: IbmTransport,
{
    /// Creates a validated IBM adapter.
    ///
    /// No network call is made.
    pub fn new(
        config: IbmAdapterConfig,
        transport: Arc<T>,
        descriptor: IbmBackendDescriptor,
        backend: QuantumBackend,
    ) -> Result<Self, IbmAdapterError> {
        config.validate()?;

        if descriptor.backend_id != backend.metadata.backend_id {
            return Err(IbmAdapterError::InvalidResponse(
                "IBM descriptor/backend identity mismatch".to_owned(),
            ));
        }

        if descriptor.qubit_count != backend.limits.max_qubits {
            return Err(IbmAdapterError::InvalidResponse(
                "IBM descriptor/backend qubit-count mismatch".to_owned(),
            ));
        }

        Ok(Self {
            config,
            transport,
            backend,
            descriptor,
        })
    }

    /// Returns the IBM adapter configuration.
    pub fn config(&self) -> &IbmAdapterConfig {
        &self.config
    }

    /// Returns the canonical Zamani backend.
    pub fn backend(&self) -> &QuantumBackend {
        &self.backend
    }

    /// Returns the provider-specific descriptor.
    pub fn descriptor(&self) -> &IbmBackendDescriptor {
        &self.descriptor
    }

    /// Returns stable IBM adapter identity.
    pub fn identity() -> AdapterIdentity {
        AdapterIdentity::new(
            IBM_ADAPTER_ID,
            IBM_PROVIDER_ID,
            IBM_ADAPTER_VERSION,
            Some(IBM_API_VERSION.to_owned()),
        )
        .expect("static IBM adapter identity must be valid")
    }

    /// Returns provider-neutral adapter metadata.
    pub fn metadata() -> AdapterMetadata {
        let mut metadata =
            AdapterMetadata::new(
                Self::identity(),
                "Zamani IBM Quantum Adapter",
            )
            .expect("static IBM adapter metadata must be valid");

        metadata = metadata
            .with_format(IBM_OPENQASM_FORMAT)
            .expect("static IBM format must be valid");

        metadata = metadata
            .with_format(IBM_OPENQASM_3_FORMAT)
            .expect("static IBM format must be valid");

        metadata = metadata
            .with_capability(capability::MEASUREMENT)
            .expect("static IBM capability must be valid");

        metadata = metadata
            .with_capability(capability::RESET)
            .expect("static IBM capability must be valid");

        metadata = metadata
            .with_capability(capability::MID_CIRCUIT_MEASUREMENT)
            .expect("static IBM capability must be valid");

        metadata = metadata
            .with_capability(capability::DYNAMIC_CIRCUITS)
            .expect("static IBM capability must be valid");

        metadata = metadata
            .with_capability(capability::CLASSICAL_CONTROL)
            .expect("static IBM capability must be valid");

        metadata = metadata
            .with_capability(capability::PARAMETERIZED_GATES)
            .expect("static IBM capability must be valid");

        metadata = metadata
            .with_capability(capability::CANCELLATION)
            .expect("static IBM capability must be valid");

        metadata = metadata
            .with_capability(capability::QUEUE_INFORMATION)
            .expect("static IBM capability must be valid");

        metadata = metadata
            .with_capability(capability::CALIBRATION_DATA)
            .expect("static IBM capability must be valid");

        metadata = metadata
            .with_capability(capability::TOPOLOGY_INFORMATION)
            .expect("static IBM capability must be valid");

        metadata
    }

    /// Converts an IBM program format into its canonical identifier.
    pub fn normalize_program_format(
        format: &str,
    ) -> Result<&'static str, IbmAdapterError> {
        match normalize_identifier(format).as_str() {
            IBM_OPENQASM_FORMAT => Ok(IBM_OPENQASM_FORMAT),
            IBM_OPENQASM_3_FORMAT => Ok(IBM_OPENQASM_FORMAT),
            "openqasm-3" => Ok(IBM_OPENQASM_FORMAT),
            _ => Err(IbmAdapterError::UnsupportedProgramFormat(
                format.to_owned(),
            )),
        }
    }

    /// Validates an execution request against IBM-specific invariants.
    pub fn validate_execution_request(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<(), IbmAdapterError> {
        if !self.descriptor.status.is_usable() {
            return Err(IbmAdapterError::BackendUnavailable);
        }

        Self::normalize_program_format(program.format())?;

        if request.shots == 0 {
            return Err(IbmAdapterError::InvalidResponse(
                "IBM execution requires at least one shot".to_owned(),
            ));
        }

        if request.shots > self.backend.limits.max_shots {
            return Err(IbmAdapterError::InvalidResponse(
                "requested IBM shot count exceeds backend limit".to_owned(),
            ));
        }

        Ok(())
    }

    /// Builds the IBM job-submission request.
    ///
    /// The request is deterministic and contains no credentials.
    pub fn build_submit_request(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<IbmRequest, IbmAdapterError> {
        self.validate_execution_request(request, program)?;

        let backend_id =
            percent_encode_path_segment(&self.descriptor.backend_id)?;

        let body = build_submission_json(
            program,
            request.shots,
        )?;

        Ok(IbmRequest {
            operation: operation::SUBMIT,
            path: format!(
                "/{}/jobs",
                backend_id
            ),
            method: IbmHttpMethod::Post,
            query: BTreeMap::new(),
            body: Some(body),
            timeout: self.config.request_timeout,
        })
    }

    /// Builds an IBM status request.
    pub fn build_status_request(
        &self,
        job_id: &BackendJobId,
    ) -> Result<IbmRequest, IbmAdapterError> {
        let encoded_job_id =
            percent_encode_path_segment(job_id.as_str())?;

        Ok(IbmRequest {
            operation: operation::STATUS,
            path: format!(
                "/jobs/{encoded_job_id}"
            ),
            method: IbmHttpMethod::Get,
            query: BTreeMap::new(),
            body: None,
            timeout: self.config.request_timeout,
        })
    }

    /// Builds an IBM result request.
    pub fn build_result_request(
        &self,
        job_id: &BackendJobId,
    ) -> Result<IbmRequest, IbmAdapterError> {
        let encoded_job_id =
            percent_encode_path_segment(job_id.as_str())?;

        Ok(IbmRequest {
            operation: operation::RESULT,
            path: format!(
                "/jobs/{encoded_job_id}/results"
            ),
            method: IbmHttpMethod::Get,
            query: BTreeMap::new(),
            body: None,
            timeout: self.config.request_timeout,
        })
    }

    /// Builds an IBM cancellation request.
    pub fn build_cancel_request(
        &self,
        job_id: &BackendJobId,
    ) -> Result<IbmRequest, IbmAdapterError> {
        let encoded_job_id =
            percent_encode_path_segment(job_id.as_str())?;

        Ok(IbmRequest {
            operation: operation::CANCEL,
            path: format!(
                "/jobs/{encoded_job_id}"
            ),
            method: IbmHttpMethod::Delete,
            query: BTreeMap::new(),
            body: None,
            timeout: self.config.request_timeout,
        })
    }

    /// Normalizes an IBM provider job identifier.
    pub fn normalize_job_id(
        value: &str,
    ) -> Result<BackendJobId, IbmAdapterError> {
        let value = value.trim();

        if value.is_empty() {
            return Err(IbmAdapterError::InvalidJobId);
        }

        if value.len() > super::super::backend_trait::MAX_JOB_ID_LENGTH {
            return Err(IbmAdapterError::JobIdTooLong);
        }

        BackendJobId::new(value.to_owned())
            .map_err(|_| IbmAdapterError::InvalidJobId)
    }

    /// Converts an IBM status string into Zamani's lifecycle state.
    pub fn normalize_job_state(
        state: &str,
    ) -> Result<BackendJobState, IbmAdapterError> {
        let normalized = normalize_identifier(state);

        match normalized.as_str() {
            provider_status::CREATED => Ok(BackendJobState::Created),

            provider_status::QUEUED => Ok(BackendJobState::Queued),

            provider_status::RUNNING => Ok(BackendJobState::Running),

            provider_status::COMPLETED => {
                Ok(BackendJobState::Completed)
            }

            provider_status::FAILED => Ok(BackendJobState::Failed),

            provider_status::CANCELLED => {
                Ok(BackendJobState::Cancelled)
            }

            provider_status::CANCELLING => {
                Ok(BackendJobState::Cancelling)
            }

            provider_status::EXPIRED => {
                Ok(BackendJobState::Expired)
            }

            provider_status::TIMED_OUT => {
                Ok(BackendJobState::TimedOut)
            }

            _ => Err(IbmAdapterError::UnknownProviderState(
                state.to_owned(),
            )),
        }
    }

    /// Normalizes a provider status response.
    ///
    /// The IBM transport is responsible for JSON decoding. This method accepts
    /// the already-extracted stable provider fields so that this module does
    /// not become coupled to a JSON implementation.
    pub fn normalize_job_status(
        &self,
        job_id: BackendJobId,
        state: &str,
        provider_status: Option<String>,
        queue_position: Option<usize>,
        estimated_wait: Option<Duration>,
        result_available: bool,
    ) -> Result<BackendJobStatus, IbmAdapterError> {
        let state = Self::normalize_job_state(state)?;

        let provider_status =
            provider_status.map(|status| {
                truncate_safe_status(&status)
            });

        let job = BackendJob::new(
            job_id,
            self.descriptor.backend_id.clone(),
            None,
            state,
        )
        .map_err(|_| IbmAdapterError::InvalidJobId)?;

        Ok(BackendJobStatus {
            job,
            provider_status,
            queue_position,
            estimated_wait,
            result_available,
        })
    }

    /// Normalizes IBM counts into a provider-neutral execution result.
    ///
    /// `counts` is expected to contain canonical bitstring/count pairs.
    ///
    /// The adapter does not invent probabilities or amplitudes.
    /// Those are derived values and belong to higher-level result analysis.
    pub fn normalize_counts_result(
        &self,
        job_id: &BackendJobId,
        shots: u64,
        counts: BTreeMap<String, u64>,
    ) -> Result<ExecutionResult, IbmAdapterError> {
        if shots == 0 {
            return Err(IbmAdapterError::ResultNormalization(
                "IBM result contains zero requested shots".to_owned(),
            ));
        }

        if counts.len() > MAX_RESULT_ENTRIES {
            return Err(IbmAdapterError::ResultNormalization(
                "IBM result contains too many count entries".to_owned(),
            ));
        }

        let total: u64 = counts
            .values()
            .try_fold(0_u64, |accumulator, value| {
                accumulator.checked_add(*value)
            })
            .ok_or_else(|| {
                IbmAdapterError::ResultNormalization(
                    "IBM result count overflow".to_owned(),
                )
            })?;

        if total > shots {
            return Err(IbmAdapterError::ResultNormalization(
                "IBM result contains more samples than requested shots"
                    .to_owned(),
            ));
        }

        for key in counts.keys() {
            if key.is_empty() || key.len() > MAX_RESULT_KEY_LENGTH {
                return Err(IbmAdapterError::ResultNormalization(
                    "IBM result contains an invalid bitstring key".to_owned(),
                ));
            }
        }

        /*
         * The current repository's ExecutionResult remains the authoritative
         * result type. We intentionally construct it through its existing
         * provider-neutral API rather than creating an IBM result type.
         *
         * If the current backend.rs evolves its result constructor, this
         * adapter should consume that stable constructor rather than defining
         * an IBM-specific result model.
         */
        ExecutionResult::from_counts(
            self.descriptor.backend_id.clone(),
            job_id.as_str().to_owned(),
            shots,
            counts,
        )
        .map_err(|error| {
            IbmAdapterError::ResultNormalization(error.to_string())
        })
    }

    /// Maps an IBM HTTP/provider response into a normalized provider failure.
    pub fn provider_failure(
        operation: &'static str,
        response: &IbmResponse,
    ) -> IbmAdapterError {
        let message =
            sanitize_provider_message(&response.body);

        IbmAdapterError::ProviderFailure {
            operation,
            message,
        }
    }

    /// Validates that a provider response is successful.
    pub fn require_success(
        operation: &'static str,
        response: &IbmResponse,
    ) -> Result<(), IbmAdapterError> {
        if response.is_success() {
            Ok(())
        } else {
            Err(Self::provider_failure(operation, response))
        }
    }

    /// Performs IBM job submission through the injected transport.
    pub fn submit(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<BackendJob, IbmAdapterError> {
        let provider_request =
            self.build_submit_request(request, program)?;

        let response =
            self.transport.send(&provider_request)?;

        Self::require_success(
            operation::SUBMIT,
            &response,
        )?;

        let provider_job_id =
            extract_string_field(
                &response.body,
                "id",
            )
            .ok_or_else(|| {
                IbmAdapterError::InvalidResponse(
                    "IBM submission response did not contain a job id"
                        .to_owned(),
                )
            })?;

        let job_id =
            Self::normalize_job_id(&provider_job_id)?;

        BackendJob::new(
            job_id,
            self.descriptor.backend_id.clone(),
            request.request_id.clone(),
            BackendJobState::Created,
        )
        .map_err(|_| IbmAdapterError::InvalidJobId)
    }

    /// Retrieves normalized IBM job status.
    pub fn status(
        &self,
        job_id: &BackendJobId,
    ) -> Result<BackendJobStatus, IbmAdapterError> {
        let request =
            self.build_status_request(job_id)?;

        let response =
            self.transport.send(&request)?;

        Self::require_success(
            operation::STATUS,
            &response,
        )?;

        let state =
            extract_string_field(
                &response.body,
                "status",
            )
            .ok_or_else(|| {
                IbmAdapterError::InvalidResponse(
                    "IBM status response did not contain status"
                        .to_owned(),
                )
            })?;

        let result_available =
            extract_bool_field(
                &response.body,
                "result_available",
            )
            .unwrap_or(matches!(
                Self::normalize_job_state(&state)?,
                BackendJobState::Completed
            ));

        let queue_position =
            extract_u64_field(
                &response.body,
                "queue_position",
            )
            .and_then(|value| usize::try_from(value).ok());

        self.normalize_job_status(
            job_id.clone(),
            &state,
            None,
            queue_position,
            None,
            result_available,
        )
    }

    /// Cancels an IBM job.
    pub fn cancel(
        &self,
        job_id: &BackendJobId,
    ) -> Result<BackendCancellation, IbmAdapterError> {
        let request =
            self.build_cancel_request(job_id)?;

        let response =
            self.transport.send(&request)?;

        if response.is_success() {
            return Ok(BackendCancellation::Cancelled);
        }

        /*
         * Cancellation can legitimately race with provider completion.
         * A provider response indicating that the job is already terminal
         * should be interpreted by the transport/parser layer rather than
         * blindly converted to success here.
         */
        Err(Self::provider_failure(
            operation::CANCEL,
            &response,
        ))
    }

    /// Retrieves and normalizes IBM results.
    ///
    /// The transport layer must extract the provider result into a canonical
    /// counts map before calling this method.
    pub fn result_from_counts(
        &self,
        job_id: &BackendJobId,
        shots: u64,
        counts: BTreeMap<String, u64>,
    ) -> Result<ExecutionResult, IbmAdapterError> {
        self.normalize_counts_result(
            job_id,
            shots,
            counts,
        )
    }

    /// Returns whether this adapter can execute the supplied program format.
    pub fn supports_program_format(
        format: &str,
    ) -> bool {
        Self::normalize_program_format(format).is_ok()
    }

    /// Returns whether a capability is advertised by the IBM descriptor.
    pub fn supports_capability(
        &self,
        capability: &str,
    ) -> bool {
        self.descriptor
            .capabilities
            .contains(
                &normalize_identifier(capability),
            )
    }
}

/// Generic helper for constructing an IBM backend capability set.
///
/// Provider discovery code can use this when translating IBM backend
/// properties into Zamani capability identifiers.
pub fn normalize_ibm_capabilities(
    measurement: bool,
    reset: bool,
    mid_circuit_measurement: bool,
    classical_control: bool,
    dynamic_circuits: bool,
    parameterized_gates: bool,
    cancellation: bool,
    queue_information: bool,
    calibration_data: bool,
    topology_information: bool,
) -> BTreeSet<String> {
    let mut capabilities = BTreeSet::new();

    if measurement {
        capabilities.insert(capability::MEASUREMENT.to_owned());
    }

    if reset {
        capabilities.insert(capability::RESET.to_owned());
    }

    if mid_circuit_measurement {
        capabilities.insert(
            capability::MID_CIRCUIT_MEASUREMENT.to_owned(),
        );
    }

    if classical_control {
        capabilities.insert(
            capability::CLASSICAL_CONTROL.to_owned(),
        );
    }

    if dynamic_circuits {
        capabilities.insert(
            capability::DYNAMIC_CIRCUITS.to_owned(),
        );
    }

    if parameterized_gates {
        capabilities.insert(
            capability::PARAMETERIZED_GATES.to_owned(),
        );
    }

    if cancellation {
        capabilities.insert(
            capability::CANCELLATION.to_owned(),
        );
    }

    if queue_information {
        capabilities.insert(
            capability::QUEUE_INFORMATION.to_owned(),
        );
    }

    if calibration_data {
        capabilities.insert(
            capability::CALIBRATION_DATA.to_owned(),
        );
    }

    if topology_information {
        capabilities.insert(
            capability::TOPOLOGY_INFORMATION.to_owned(),
        );
    }

    capabilities
}

/// Normalizes IBM native instruction names.
///
/// IBM provider naming is kept provider-local; the resulting strings are
/// stable canonical identifiers only within the adapter boundary.
pub fn normalize_ibm_instruction(
    instruction: &str,
) -> String {
    normalize_instruction(instruction)
}

/// Validates an IBM backend identifier.
pub fn validate_backend_id(
    backend_id: &str,
) -> Result<(), IbmAdapterError> {
    let value = backend_id.trim();

    if value.is_empty() {
        return Err(IbmAdapterError::InvalidBackendId);
    }

    if value.len() > MAX_IBM_BACKEND_ID_LENGTH {
        return Err(IbmAdapterError::BackendIdTooLong);
    }

    if value.contains('/') || value.contains('\\') {
        return Err(IbmAdapterError::InvalidBackendId);
    }

    if value.chars().any(char::is_control) {
        return Err(IbmAdapterError::InvalidBackendId);
    }

    Ok(())
}

/// Validates an IBM backend display name.
pub fn validate_backend_name(
    name: &str,
) -> Result<(), IbmAdapterError> {
    let value = name.trim();

    if value.is_empty() {
        return Err(IbmAdapterError::InvalidBackendName);
    }

    if value.len() > MAX_IBM_BACKEND_NAME_LENGTH {
        return Err(IbmAdapterError::BackendNameTooLong);
    }

    if value.chars().any(char::is_control) {
        return Err(IbmAdapterError::InvalidBackendName);
    }

    Ok(())
}

/// Validates an IBM region.
pub fn validate_region(
    region: &str,
) -> Result<(), IbmAdapterError> {
    let value = region.trim();

    if value.is_empty() {
        return Err(IbmAdapterError::InvalidRegion);
    }

    if value.len() > MAX_IBM_REGION_LENGTH {
        return Err(IbmAdapterError::RegionTooLong);
    }

    if value.chars().any(char::is_control) {
        return Err(IbmAdapterError::InvalidRegion);
    }

    Ok(())
}

/// Builds a minimal IBM submission JSON document.
///
/// The exact provider submission schema is deliberately isolated here so that
/// provider API changes do not leak into the rest of the hardware subsystem.
///
/// The transport sends this JSON as `application/json`.
fn build_submission_json(
    program: &BackendProgram,
    shots: u64,
) -> Result<String, IbmAdapterError> {
    if program.is_empty() {
        return Err(IbmAdapterError::InvalidResponse(
            "cannot submit an empty IBM program".to_owned(),
        ));
    }

    /*
     * JSON is intentionally constructed with serde_json here because it is
     * already a repository dependency and prevents malformed escaping of
     * OpenQASM source.
     *
     * The IBM transport must not log this body.
     */
    let program_text =
        std::str::from_utf8(program.bytes())
            .map_err(|_| {
                IbmAdapterError::InvalidResponse(
                    "IBM OpenQASM program must be UTF-8".to_owned(),
                )
            })?;

    let value =
        serde_json::json!({
            "program": program_text,
            "format": IBM_OPENQASM_FORMAT,
            "shots": shots,
        });

    serde_json::to_string(&value)
        .map_err(|error| {
            IbmAdapterError::InvalidResponse(
                format!(
                    "failed to encode IBM submission JSON: {error}"
                ),
            )
        })
}

/// Extracts a simple JSON string field.
///
/// This deliberately uses `serde_json` rather than a hand-written JSON parser.
fn extract_string_field(
    body: &str,
    field: &str,
) -> Option<String> {
    let value: serde_json::Value =
        serde_json::from_str(body).ok()?;

    value
        .get(field)?
        .as_str()
        .map(ToOwned::to_owned)
}

/// Extracts a boolean JSON field.
fn extract_bool_field(
    body: &str,
    field: &str,
) -> Option<bool> {
    let value: serde_json::Value =
        serde_json::from_str(body).ok()?;

    value.get(field)?.as_bool()
}

/// Extracts a non-negative integer JSON field.
fn extract_u64_field(
    body: &str,
    field: &str,
) -> Option<u64> {
    let value: serde_json::Value =
        serde_json::from_str(body).ok()?;

    value.get(field)?.as_u64()
}

/// Normalizes an identifier into a stable lowercase representation.
fn normalize_identifier(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
}

/// Normalizes an IBM instruction.
fn normalize_instruction(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
}

/// Returns true if a string resembles credential/secret material.
///
/// This is deliberately conservative. False positives are preferable to
/// accidentally logging credentials.
fn contains_secret_marker(value: &str) -> bool {
    let value = value.to_ascii_lowercase();

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
        "secret",
        "cookie",
        "bearer",
    ];

    MARKERS.iter().any(|marker| value.contains(marker))
}

/// Sanitizes a provider response body before putting it into an error.
///
/// Provider responses are treated as untrusted input.
fn sanitize_provider_message(
    body: &str,
) -> String {
    let mut message = body
        .chars()
        .filter(|character| !character.is_control() || *character == '\n')
        .collect::<String>();

    if contains_secret_marker(&message) {
        return "IBM provider returned an error containing sensitive-looking data; response body was redacted"
            .to_owned();
    }

    if message.len() > MAX_PROVIDER_ERROR_MESSAGE_LENGTH {
        message.truncate(MAX_PROVIDER_ERROR_MESSAGE_LENGTH);
    }

    message
}

/// Truncates provider status safely.
fn truncate_safe_status(
    status: &str,
) -> String {
    let mut status = status
        .chars()
        .filter(|character| !character.is_control())
        .collect::<String>();

    if status.len() > MAX_PROVIDER_STATUS_LENGTH {
        status.truncate(MAX_PROVIDER_STATUS_LENGTH);
    }

    status
}

/// Percent-encodes a backend/job path segment.
///
/// Only unreserved URI characters are emitted unchanged.
fn percent_encode_path_segment(
    value: &str,
) -> Result<String, IbmAdapterError> {
    if value.is_empty() {
        return Err(IbmAdapterError::InvalidBackendId);
    }

    let mut encoded = String::with_capacity(value.len());

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
            encoded.push(*byte as char);
        } else {
            encoded.push('%');

            let high = byte >> 4;
            let low = byte & 0x0f;

            encoded.push(hex_digit(high));
            encoded.push(hex_digit(low));
        }
    }

    Ok(encoded)
}

/// Converts a nibble to uppercase hexadecimal.
fn hex_digit(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=15 => (b'A' + (value - 10)) as char,
        _ => unreachable!("nibble must be <= 15"),
    }
}

// =============================================================================
// Provider-neutral adapter integration
// =============================================================================
//
// The exact `QuantumBackendAdapter` surface is deliberately implemented in
// terms of the repository's canonical trait rather than inventing a second
// execution abstraction.
//
// The provider adapter should remain the only location containing IBM-specific
// translation logic.
//
// NOTE:
// The repository's current trait is the authoritative contract. The following
// implementation block is intentionally kept isolated so that future trait
// evolution does not contaminate IBM-specific data structures.
//
// =============================================================================

impl<T> QuantumBackendAdapter for IbmQuantumAdapter<T>
where
    T: IbmTransport + 'static,
{
    /*
     * The repository's canonical adapter trait owns the complete executable
     * backend lifecycle.
     *
     * IBM-specific methods above provide the semantic primitives consumed by
     * the trait implementation.
     *
     * The actual trait method surface is intentionally delegated to the
     * existing backend_trait.rs contract rather than duplicating lifecycle
     * semantics here.
     *
     * This block is the integration seam for:
     *
     *     provider_registry
     *          |
     *          v
     *     Box/Arc<dyn QuantumBackendAdapter>
     *          |
     *          v
     *     execution.rs
     *          |
     *          v
     *     IbmQuantumAdapter
     *
     * If the current repository trait exposes additional required methods,
     * those methods belong here and must call the IBM primitives above.
     */

    fn backend(&self) -> &QuantumBackend {
        &self.backend
    }

    fn metadata(&self) -> AdapterMetadata {
        Self::metadata()
    }

    fn submit(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<BackendJob, crate::quantum::hardware::backend::BackendError> {
        self.submit(request, program)
            .map_err(map_ibm_error)
    }

    fn status(
        &self,
        job_id: &BackendJobId,
    ) -> Result<BackendJobStatus, crate::quantum::hardware::backend::BackendError> {
        self.status(job_id)
            .map_err(map_ibm_error)
    }

    fn result(
        &self,
        job_id: &BackendJobId,
    ) -> Result<ExecutionResult, crate::quantum::hardware::backend::BackendError> {
        let request =
            self.build_result_request(job_id)
                .map_err(map_ibm_error)?;

        let response =
            self.transport
                .send(&request)
                .map_err(map_ibm_error)?;

        Self::require_success(
            operation::RESULT,
            &response,
        )
        .map_err(map_ibm_error)?;

        let shots =
            extract_u64_field(
                &response.body,
                "shots",
            )
            .ok_or_else(|| {
                crate::quantum::hardware::backend::BackendError::ExecutionUnavailable
            })?;

        let counts =
            extract_counts(
                &response.body,
            )
            .map_err(map_ibm_error)?;

        self.normalize_counts_result(
            job_id,
            shots,
            counts,
        )
        .map_err(map_ibm_error)
    }

    fn cancel(
        &self,
        job_id: &BackendJobId,
    ) -> Result<BackendCancellation, crate::quantum::hardware::backend::BackendError> {
        self.cancel(job_id)
            .map_err(map_ibm_error)
    }
}

// =============================================================================
// IBM result extraction
// =============================================================================

/// Extracts IBM result counts from the provider response.
///
/// Accepted normalized forms:
///
/// ```json
/// {
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
///     "counts": {
///       "00": 500,
///       "11": 500
///     }
///   }
/// }
/// ```
///
/// Provider-specific response decoding remains isolated here.
fn extract_counts(
    body: &str,
) -> Result<BTreeMap<String, u64>, IbmAdapterError> {
    let value: serde_json::Value =
        serde_json::from_str(body)
            .map_err(|error| {
                IbmAdapterError::ResultNormalization(
                    format!(
                        "invalid IBM JSON result: {error}"
                    ),
                )
            })?;

    let counts_value =
        value
            .get("counts")
            .or_else(|| {
                value
                    .get("result")
                    .and_then(|result| result.get("counts"))
            })
            .ok_or_else(|| {
                IbmAdapterError::ResultNormalization(
                    "IBM result does not contain counts"
                        .to_owned(),
                )
            })?;

    let object =
        counts_value
            .as_object()
            .ok_or_else(|| {
                IbmAdapterError::ResultNormalization(
                    "IBM counts field is not an object"
                        .to_owned(),
                )
            })?;

    if object.len() > MAX_RESULT_ENTRIES {
        return Err(IbmAdapterError::ResultNormalization(
            "IBM result contains too many count entries"
                .to_owned(),
        ));
    }

    let mut counts = BTreeMap::new();

    for (key, value) in object {
        if key.is_empty() || key.len() > MAX_RESULT_KEY_LENGTH {
            return Err(IbmAdapterError::ResultNormalization(
                "IBM result contains an invalid count key"
                    .to_owned(),
            ));
        }

        let count =
            value.as_u64().ok_or_else(|| {
                IbmAdapterError::ResultNormalization(
                    "IBM count value is not an unsigned integer"
                        .to_owned(),
                )
            })?;

        counts.insert(key.clone(), count);
    }

    Ok(counts)
}

// =============================================================================
// Backend error mapping
// =============================================================================

/// Converts an IBM adapter error into the repository's existing provider
/// neutral backend error boundary.
///
/// Provider-specific IBM information is kept only in the human-readable
/// diagnostic where the current backend error API permits it.
///
/// No IBM-specific error type leaks outside this module.
fn map_ibm_error(
    error: IbmAdapterError,
) -> crate::quantum::hardware::backend::BackendError {
    use crate::quantum::hardware::backend::BackendError;

    match error {
        IbmAdapterError::BackendUnavailable => {
            BackendError::ExecutionUnavailable
        }

        IbmAdapterError::UnsupportedProgramFormat(_)
        | IbmAdapterError::UnsupportedCapability(_) => {
            BackendError::ExecutionUnavailable
        }

        IbmAdapterError::InvalidResponse(_)
        | IbmAdapterError::ResultNormalization(_)
        | IbmAdapterError::InvalidJobId
        | IbmAdapterError::JobIdTooLong
        | IbmAdapterError::InvalidBackendId
        | IbmAdapterError::BackendIdTooLong
        | IbmAdapterError::InvalidBackendName
        | IbmAdapterError::BackendNameTooLong
        | IbmAdapterError::InvalidRegion
        | IbmAdapterError::RegionTooLong
        | IbmAdapterError::MissingTransport
        | IbmAdapterError::Metadata(_) => {
            BackendError::ExecutionUnavailable
        }

        IbmAdapterError::ProviderFailure {
            operation: _,
            message: _,
        }
        | IbmAdapterError::UnknownProviderState(_)
        | IbmAdapterError::Transport(_) => {
            BackendError::ExecutionUnavailable
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct MockTransport {
        requests: std::sync::Mutex<Vec<IbmRequest>>,
    }

    impl IbmTransport for MockTransport {
        fn send(
            &self,
            request: &IbmRequest,
        ) -> Result<IbmResponse, IbmAdapterError> {
            self.requests
                .lock()
                .expect("mock mutex must not be poisoned")
                .push(request.clone());

            match request.operation {
                operation::SUBMIT => Ok(IbmResponse {
                    status_code: 200,
                    body: r#"{"id":"ibm-job-001"}"#.to_owned(),
                    headers: BTreeMap::new(),
                }),

                operation::STATUS => Ok(IbmResponse {
                    status_code: 200,
                    body: r#"{
                        "status":"completed",
                        "result_available":true,
                        "queue_position":0
                    }"#
                    .to_owned(),
                    headers: BTreeMap::new(),
                }),

                operation::RESULT => Ok(IbmResponse {
                    status_code: 200,
                    body: r#"{
                        "shots":1000,
                        "counts":{
                            "00":500,
                            "11":500
                        }
                    }"#
                    .to_owned(),
                    headers: BTreeMap::new(),
                }),

                operation::CANCEL => Ok(IbmResponse {
                    status_code: 200,
                    body: "{}".to_owned(),
                    headers: BTreeMap::new(),
                }),

                _ => Err(IbmAdapterError::InvalidResponse(
                    "unknown mock operation".to_owned(),
                )),
            }
        }
    }

    #[test]
    fn adapter_identity_is_stable() {
        let identity =
            IbmQuantumAdapter::<MockTransport>::identity();

        assert_eq!(
            identity.provider_id(),
            IBM_PROVIDER_ID
        );

        assert_eq!(
            identity.adapter_id(),
            IBM_ADAPTER_ID
        );

        assert_eq!(
            identity.adapter_version(),
            IBM_ADAPTER_VERSION
        );
    }

    #[test]
    fn metadata_contains_ibm_formats() {
        let metadata =
            IbmQuantumAdapter::<MockTransport>::metadata();

        assert!(
            metadata
                .supported_formats
                .contains(IBM_OPENQASM_FORMAT)
        );
    }

    #[test]
    fn openqasm_format_is_normalized() {
        assert_eq!(
            IbmQuantumAdapter::<MockTransport>::normalize_program_format(
                "openqasm3"
            )
            .expect("openqasm3 must be supported"),
            IBM_OPENQASM_FORMAT
        );
    }

    #[test]
    fn unsupported_format_is_rejected() {
        assert!(
            IbmQuantumAdapter::<MockTransport>::normalize_program_format(
                "qir"
            )
            .is_err()
        );
    }

    #[test]
    fn backend_identifier_validation_rejects_path_traversal() {
        assert!(
            validate_backend_id("../backend").is_err()
        );

        assert!(
            validate_backend_id("foo/bar").is_err()
        );
    }

    #[test]
    fn provider_state_is_normalized() {
        assert_eq!(
            IbmQuantumAdapter::<MockTransport>::normalize_job_state(
                "completed"
            )
            .expect("completed must normalize"),
            BackendJobState::Completed
        );

        assert_eq!(
            IbmQuantumAdapter::<MockTransport>::normalize_job_state(
                "running"
            )
            .expect("running must normalize"),
            BackendJobState::Running
        );
    }

    #[test]
    fn unknown_provider_state_is_not_assumed_successful() {
        assert!(
            IbmQuantumAdapter::<MockTransport>::normalize_job_state(
                "some_future_ibm_state"
            )
            .is_err()
        );
    }

    #[test]
    fn path_segments_are_encoded() {
        assert_eq!(
            percent_encode_path_segment("job-01"),
            Ok("job-01".to_owned())
        );

        assert_eq!(
            percent_encode_path_segment("job/01"),
            Ok("job%2F01".to_owned())
        );
    }

    #[test]
    fn secret_like_metadata_is_rejected() {
        assert!(
            IbmBackendDescriptor::new(
                "ibm_test",
                "IBM Test",
                5
            )
            .expect("descriptor must be valid")
            .with_metadata(
                "api_key",
                "secret-value"
            )
            .is_err()
        );
    }

    #[test]
    fn result_counts_are_deterministic() {
        let transport =
            Arc::new(MockTransport::default());

        let descriptor =
            IbmBackendDescriptor::new(
                "ibm_test",
                "IBM Test",
                5,
            )
            .expect("descriptor must be valid")
            .with_status(BackendStatus::Available);

        let backend =
            test_backend("ibm_test", 5);

        let adapter =
            IbmQuantumAdapter::new(
                IbmAdapterConfig::default(),
                transport,
                descriptor,
                backend,
            )
            .expect("adapter must be valid");

        let job =
            BackendJobId::new("ibm-job-001")
                .expect("job id must be valid");

        let mut counts = BTreeMap::new();

        counts.insert("11".to_owned(), 500);
        counts.insert("00".to_owned(), 500);

        let result =
            adapter
                .normalize_counts_result(
                    &job,
                    1000,
                    counts,
                )
                .expect("counts must normalize");

        assert_eq!(
            result.backend_id(),
            "ibm_test"
        );
    }

    #[test]
    fn submission_request_contains_no_credentials() {
        let transport =
            Arc::new(MockTransport::default());

        let descriptor =
            IbmBackendDescriptor::new(
                "ibm_test",
                "IBM Test",
                5,
            )
            .expect("descriptor must be valid")
            .with_status(BackendStatus::Available);

        let backend =
            test_backend("ibm_test", 5);

        let adapter =
            IbmQuantumAdapter::new(
                IbmAdapterConfig::default(),
                transport,
                descriptor,
                backend,
            )
            .expect("adapter must be valid");

        let program =
            BackendProgram::new(
                IBM_OPENQASM_FORMAT,
                b"OPENQASM 3.0;\nqubit[1] q;\nbit[1] c;\nmeasure q -> c;"
                    .to_vec(),
            )
            .expect("program must be valid");

        let request =
            ExecutionRequest::new(
                "request-001",
                "ibm_test",
                100,
            )
            .expect("execution request must be valid");

        let ibm_request =
            adapter
                .build_submit_request(
                    &request,
                    &program,
                )
                .expect("IBM request must be valid");

        assert_eq!(
            ibm_request.operation,
            operation::SUBMIT
        );

        let body =
            ibm_request
                .body
                .expect("submission must contain a body");

        assert!(
            !body.contains("api_key")
        );

        assert!(
            !body.contains("authorization")
        );
    }

    #[test]
    fn mock_submission_and_status_are_normalized() {
        let transport =
            Arc::new(MockTransport::default());

        let descriptor =
            IbmBackendDescriptor::new(
                "ibm_test",
                "IBM Test",
                5,
            )
            .expect("descriptor must be valid")
            .with_status(BackendStatus::Available);

        let backend =
            test_backend("ibm_test", 5);

        let adapter =
            IbmQuantumAdapter::new(
                IbmAdapterConfig::default(),
                transport,
                descriptor,
                backend,
            )
            .expect("adapter must be valid");

        let program =
            BackendProgram::new(
                IBM_OPENQASM_FORMAT,
                b"OPENQASM 3.0;".to_vec(),
            )
            .expect("program must be valid");

        let request =
            ExecutionRequest::new(
                "request-001",
                "ibm_test",
                100,
            )
            .expect("request must be valid");

        let job =
            adapter
                .submit(
                    &request,
                    &program,
                )
                .expect("submission must succeed");

        assert_eq!(
            job.id.as_str(),
            "ibm-job-001"
        );

        let status =
            adapter
                .status(&job.id)
                .expect("status must succeed");

        assert_eq!(
            status.job.state,
            BackendJobState::Completed
        );

        assert!(
            status.result_available
        );
    }

    fn test_backend(
        backend_id: &str,
        qubits: usize,
    ) -> QuantumBackend {
        /*
         * This helper intentionally constructs the existing repository
         * backend contract. The IBM adapter itself does not redefine the
         * canonical backend aggregate.
         *
         * If backend.rs exposes a dedicated production builder, this helper
         * should use that builder instead of reconstructing the aggregate.
         */
        QuantumBackend::test_backend(
            backend_id,
            BackendKind::Qpu,
            qubits,
        )
    }
}