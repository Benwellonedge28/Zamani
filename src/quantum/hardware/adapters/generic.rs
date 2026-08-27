//! Zamani Quantum — Generic Provider Adapter Foundation
//!
//! Production-grade, provider-neutral foundation for:
//!
//! `crate::quantum::hardware::adapters`
//!
//! # Responsibility
//!
//! This module defines the reusable primitives that every concrete quantum
//! hardware adapter can build upon without modifying the canonical hardware
//! layer.
//!
//! It owns:
//!
//! - provider-neutral adapter identity;
//! - provider-neutral transport requests and responses;
//! - safe request/response metadata;
//! - transport-independent provider error representation;
//! - deterministic request construction;
//! - request validation;
//! - response validation;
//! - safe secret redaction;
//! - retry classification primitives;
//! - provider operation classification;
//! - stable provider operation identifiers;
//! - generic adapter conformance helpers;
//! - generic capability/format negotiation primitives;
//! - bounded payload handling;
//! - pagination primitives;
//! - rate-limit metadata;
//! - provider API version metadata;
//! - idempotency-key support;
//! - correlation/request identifiers;
//! - deterministic adapter metadata;
//! - provider-neutral adapter configuration;
//! - transport abstraction.
//!
//! It deliberately does NOT own:
//!
//! - provider credentials;
//! - credential persistence;
//! - authentication;
//! - OAuth/OIDC;
//! - API-key acquisition;
//! - provider SDKs;
//! - HTTP clients;
//! - TLS implementation;
//! - OpenQASM parsing;
//! - QIR generation;
//! - Zamani Quantum IR;
//! - transpilation;
//! - routing;
//! - scheduling;
//! - calibration storage;
//! - topology;
//! - job management;
//! - provider registries;
//! - benchmarking;
//! - simulator implementation;
//! - emulator implementation.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Architectural position
//!
//! ```text
//! Zamani Quantum IR
//!        |
//!        v
//! compatibility / routing / scheduling
//!        |
//!        v
//! BackendProgram
//!        |
//!        v
//! QuantumBackendAdapter
//!        |
//!        v
//! adapters::generic
//!        |
//!        +-----------------------+
//!        |                       |
//!        v                       v
//! ProviderTransport        Provider API mapping
//!        |                       |
//!        v                       v
//! HTTP / SDK / local       IBM / IonQ / Braket / ...
//!        |
//!        v
//! Quantum backend
//! ```
//!
//! `generic.rs` is intentionally below provider-specific adapters and above
//! the actual transport implementation.
//!
//! # Dependency direction
//!
//! The intended dependency graph is:
//!
//! ```text
//! backend.rs
//!     ^
//!     |
//! backend_trait.rs
//!     ^
//!     |
//! adapters/generic.rs
//!     ^
//!     +-----------+-----------+-----------+
//!     |           |           |           |
//!   ibm.rs     ionq.rs   aws_braket.rs  local.rs
//! ```
//!
//! Concrete adapters MUST depend on this module.
//!
//! This module MUST NOT depend on concrete adapters.
//!
//! # Integration with `backend_trait.rs`
//!
//! The existing repository already defines `QuantumBackendAdapter` in
//! `hardware/backend_trait.rs`. That trait owns execution lifecycle semantics.
//!
//! This module supplies the reusable provider/transport primitives that
//! concrete implementations of `QuantumBackendAdapter` can use.
//!
//! The separation is intentional:
//!
//! ```text
//! QuantumBackend
//!     = what the backend is
//!
//! QuantumBackendAdapter
//!     = how Zamani executes against it
//!
//! GenericAdapter primitives
//!     = reusable provider/transport machinery
//!
//! Concrete adapter
//!     = provider-specific translation
//! ```
//!
//! # Interoperability
//!
//! This module deliberately treats program formats as opaque identifiers.
//! Examples include:
//!
//! - `zamani-ir`;
//! - `openqasm-3.1`;
//! - `qir`;
//! - `pulse`;
//! - `analog`;
//! - `annealing`;
//! - `logical`;
//! - provider-native formats.
//!
//! QIR is an interoperability layer rather than the canonical Zamani IR.
//! OpenQASM is likewise handled by its dedicated adapter.
//!
//! # Security
//!
//! This module follows a strict no-secret invariant.
//!
//! It MUST NOT store:
//!
//! - passwords;
//! - API keys;
//! - access tokens;
//! - private keys;
//! - refresh tokens;
//! - cookies;
//! - authorization headers;
//! - credential bodies.
//!
//! Authentication-specific modules may construct authenticated transport
//! requests, but this generic layer never obtains credentials itself.
//!
//! Debug formatting for requests and responses deliberately redacts header
//! values and body contents.
//!
//! # Determinism
//!
//! Generic adapter state is deterministic:
//!
//! - identifiers are explicitly supplied;
//! - maps use `BTreeMap`;
//! - capabilities use `BTreeSet`;
//! - no global state is used;
//! - no random numbers are generated;
//! - no clock is read;
//! - no environment variables are read;
//! - no network operation is performed by this module.
//!
//! # Thread safety
//!
//! The core types are designed to be `Send`/`Sync` whenever their contained
//! provider-neutral values are `Send`/`Sync`.
//!
//! `ProviderTransport` is intentionally object-safe and requires `Send + Sync`
//! so registries can store it behind `Arc<dyn ProviderTransport>`.
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
//! # Stability rule
//!
//! This file is a foundational adapter contract.
//!
//! Adding a new provider MUST NOT require changing this file.
//!
//! If a provider requires an unusual feature, the provider adapter must model
//! that feature locally using these generic primitives.
//!
//! -----------------------------------------------------------------------------
//! Schema
//! -----------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::time::Duration;

use super::super::backend::BackendError;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the generic adapter layer.
pub const GENERIC_ADAPTER_SCHEMA_ID: &str =
    "zamani.quantum.hardware.adapters.generic";

/// Semantic schema version.
///
/// Increment only when serialized or externally observable semantics change
/// incompatibly.
pub const GENERIC_ADAPTER_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Limits
// =============================================================================

/// Maximum adapter identifier length.
pub const MAX_ADAPTER_ID_LENGTH: usize = 256;

/// Maximum provider identifier length.
pub const MAX_PROVIDER_ID_LENGTH: usize = 256;

/// Maximum adapter version length.
pub const MAX_ADAPTER_VERSION_LENGTH: usize = 128;

/// Maximum provider API version length.
pub const MAX_PROVIDER_API_VERSION_LENGTH: usize = 128;

/// Maximum operation identifier length.
pub const MAX_OPERATION_ID_LENGTH: usize = 256;

/// Maximum request identifier length.
pub const MAX_REQUEST_ID_LENGTH: usize = 512;

/// Maximum idempotency-key length.
pub const MAX_IDEMPOTENCY_KEY_LENGTH: usize = 512;

/// Maximum endpoint/path length.
///
/// The generic adapter never interprets an endpoint as a secret.
pub const MAX_ENDPOINT_LENGTH: usize = 4096;

/// Maximum header name length.
pub const MAX_HEADER_NAME_LENGTH: usize = 256;

/// Maximum header value length.
pub const MAX_HEADER_VALUE_LENGTH: usize = 8192;

/// Maximum number of headers.
pub const MAX_HEADERS: usize = 256;

/// Maximum query parameter name length.
pub const MAX_QUERY_NAME_LENGTH: usize = 256;

/// Maximum query parameter value length.
pub const MAX_QUERY_VALUE_LENGTH: usize = 4096;

/// Maximum query parameter count.
pub const MAX_QUERY_PARAMETERS: usize = 256;

/// Maximum transport request payload.
///
/// Larger artifacts should eventually use an artifact/streaming subsystem.
pub const MAX_REQUEST_BODY_BYTES: usize = 256 * 1024 * 1024;

/// Maximum transport response payload.
pub const MAX_RESPONSE_BODY_BYTES: usize = 256 * 1024 * 1024;

/// Maximum provider error code length.
pub const MAX_PROVIDER_ERROR_CODE_LENGTH: usize = 256;

/// Maximum provider error message length.
pub const MAX_PROVIDER_ERROR_MESSAGE_LENGTH: usize = 4096;

/// Maximum provider error context fields.
pub const MAX_PROVIDER_ERROR_CONTEXT_FIELDS: usize = 64;

/// Maximum provider error context key length.
pub const MAX_PROVIDER_ERROR_CONTEXT_KEY_LENGTH: usize = 256;

/// Maximum provider error context value length.
pub const MAX_PROVIDER_ERROR_CONTEXT_VALUE_LENGTH: usize = 4096;

/// Maximum number of supported formats.
pub const MAX_SUPPORTED_FORMATS: usize = 256;

/// Maximum format identifier length.
pub const MAX_FORMAT_IDENTIFIER_LENGTH: usize = 128;

/// Maximum number of capabilities.
pub const MAX_CAPABILITIES: usize = 4096;

/// Maximum capability identifier length.
pub const MAX_CAPABILITY_IDENTIFIER_LENGTH: usize = 256;

/// Maximum pagination token length.
pub const MAX_PAGE_TOKEN_LENGTH: usize = 4096;

// =============================================================================
// Adapter identity
// =============================================================================

/// Immutable identity/version information for a concrete adapter.
///
/// This structure identifies the adapter implementation, not the quantum
/// device itself.
///
/// Device/backend identity belongs to `backend.rs` and future identity
/// modules.
///
/// # Example
///
/// ```text
/// adapter_id          = "zamani.hardware.ibm"
/// provider_id         = "ibm"
/// adapter_version     = "1.0.0"
/// provider_api_version = "v1"
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdapterIdentity {
    adapter_id: String,
    provider_id: String,
    adapter_version: String,
    provider_api_version: Option<String>,
}

impl AdapterIdentity {
    /// Creates a validated adapter identity.
    pub fn new(
        adapter_id: impl Into<String>,
        provider_id: impl Into<String>,
        adapter_version: impl Into<String>,
        provider_api_version: Option<String>,
    ) -> Result<Self, GenericAdapterError> {
        let adapter_id = adapter_id.into();
        let provider_id = provider_id.into();
        let adapter_version = adapter_version.into();

        validate_identifier(
            "adapter_id",
            &adapter_id,
            MAX_ADAPTER_ID_LENGTH,
        )?;

        validate_identifier(
            "provider_id",
            &provider_id,
            MAX_PROVIDER_ID_LENGTH,
        )?;

        validate_identifier(
            "adapter_version",
            &adapter_version,
            MAX_ADAPTER_VERSION_LENGTH,
        )?;

        if let Some(version) = &provider_api_version {
            validate_identifier(
                "provider_api_version",
                version,
                MAX_PROVIDER_API_VERSION_LENGTH,
            )?;
        }

        Ok(Self {
            adapter_id,
            provider_id,
            adapter_version,
            provider_api_version,
        })
    }

    /// Returns the adapter identifier.
    pub fn adapter_id(&self) -> &str {
        &self.adapter_id
    }

    /// Returns the provider identifier.
    pub fn provider_id(&self) -> &str {
        &self.provider_id
    }

    /// Returns the adapter implementation version.
    pub fn adapter_version(&self) -> &str {
        &self.adapter_version
    }

    /// Returns the provider API version, if known.
    pub fn provider_api_version(&self) -> Option<&str> {
        self.provider_api_version.as_deref()
    }
}

// =============================================================================
// Adapter metadata
// =============================================================================

/// Immutable descriptive metadata for a generic adapter.
///
/// Metadata is informational and must never be used as a substitute for
/// authenticated provider state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterMetadata {
    /// Stable adapter identity.
    pub identity: AdapterIdentity,

    /// Human-readable display name.
    pub display_name: String,

    /// Supported provider-independent program formats.
    pub supported_formats: BTreeSet<String>,

    /// Stable provider-independent capability identifiers.
    pub capabilities: BTreeSet<String>,

    /// Optional additional metadata.
    pub metadata: BTreeMap<String, String>,
}

impl AdapterMetadata {
    /// Creates validated adapter metadata.
    pub fn new(
        identity: AdapterIdentity,
        display_name: impl Into<String>,
    ) -> Result<Self, GenericAdapterError> {
        let display_name = display_name.into();

        validate_identifier(
            "display_name",
            &display_name,
            MAX_ADAPTER_ID_LENGTH,
        )?;

        Ok(Self {
            identity,
            display_name,
            supported_formats: BTreeSet::new(),
            capabilities: BTreeSet::new(),
            metadata: BTreeMap::new(),
        })
    }

    /// Adds a supported program format.
    pub fn with_format(
        mut self,
        format: impl Into<String>,
    ) -> Result<Self, GenericAdapterError> {
        let format = normalize_identifier(format.into());

        validate_identifier(
            "format",
            &format,
            MAX_FORMAT_IDENTIFIER_LENGTH,
        )?;

        if self.supported_formats.len() >= MAX_SUPPORTED_FORMATS
            && !self.supported_formats.contains(&format)
        {
            return Err(GenericAdapterError::LimitExceeded {
                field: "supported_formats",
                maximum: MAX_SUPPORTED_FORMATS,
            });
        }

        self.supported_formats.insert(format);

        Ok(self)
    }

    /// Adds a stable capability identifier.
    pub fn with_capability(
        mut self,
        capability: impl Into<String>,
    ) -> Result<Self, GenericAdapterError> {
        let capability = normalize_identifier(capability.into());

        validate_identifier(
            "capability",
            &capability,
            MAX_CAPABILITY_IDENTIFIER_LENGTH,
        )?;

        if self.capabilities.len() >= MAX_CAPABILITIES
            && !self.capabilities.contains(&capability)
        {
            return Err(GenericAdapterError::LimitExceeded {
                field: "capabilities",
                maximum: MAX_CAPABILITIES,
            });
        }

        self.capabilities.insert(capability);

        Ok(self)
    }

    /// Adds safe metadata.
    ///
    /// Keys that indicate secret material are rejected.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, GenericAdapterError> {
        let key = key.into();
        let value = value.into();

        validate_metadata_key(&key)?;
        validate_metadata_value(&value)?;

        if contains_secret_marker(&key)
            || contains_secret_marker(&value)
        {
            return Err(GenericAdapterError::SecretMaterialRejected);
        }

        if self.metadata.len() >= MAX_HEADERS
            && !self.metadata.contains_key(&key)
        {
            return Err(GenericAdapterError::LimitExceeded {
                field: "metadata",
                maximum: MAX_HEADERS,
            });
        }

        self.metadata.insert(key, value);

        Ok(self)
    }
}

// =============================================================================
// HTTP-independent method abstraction
// =============================================================================

/// Generic transport operation.
///
/// HTTP adapters may map these to GET/POST/PUT/PATCH/DELETE.
///
/// Non-HTTP transports may map the same semantics to SDK/RPC operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransportMethod {
    /// Read a resource.
    Get,

    /// Create a resource.
    Post,

    /// Replace a resource.
    Put,

    /// Partially update a resource.
    Patch,

    /// Delete a resource.
    Delete,

    /// Provider-defined operation that does not fit CRUD semantics.
    Custom,
}

impl TransportMethod {
    /// Stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
            Self::Custom => "CUSTOM",
        }
    }

    /// Returns whether the method is normally idempotent.
    ///
    /// Provider semantics may override this through an explicit operation
    /// policy. `Post` is intentionally not treated as idempotent.
    pub const fn is_normally_idempotent(self) -> bool {
        matches!(
            self,
            Self::Get | Self::Put | Self::Delete
        )
    }
}

impl fmt::Display for TransportMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Provider operation
// =============================================================================

/// Provider-neutral operation category.
///
/// Concrete adapters translate these operations into provider APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProviderOperation {
    /// Discover provider/device information.
    Discovery,

    /// Retrieve backend metadata.
    DescribeBackend,

    /// Retrieve capabilities.
    GetCapabilities,

    /// Retrieve calibration information.
    GetCalibration,

    /// Retrieve health/status.
    GetHealth,

    /// Retrieve queue information.
    GetQueue,

    /// Submit a quantum workload.
    Submit,

    /// Retrieve job status.
    GetJobStatus,

    /// Retrieve a completed result.
    GetResult,

    /// Cancel a job.
    Cancel,

    /// Provider-specific operation.
    Custom,
}

impl ProviderOperation {
    /// Stable operation identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Discovery => "discovery",
            Self::DescribeBackend => "describe_backend",
            Self::GetCapabilities => "get_capabilities",
            Self::GetCalibration => "get_calibration",
            Self::GetHealth => "get_health",
            Self::GetQueue => "get_queue",
            Self::Submit => "submit",
            Self::GetJobStatus => "get_job_status",
            Self::GetResult => "get_result",
            Self::Cancel => "cancel",
            Self::Custom => "custom",
        }
    }

    /// Returns whether retrying is normally safe from a semantic perspective.
    ///
    /// This does not mean a retry is always safe. Provider-specific
    /// idempotency and execution state must still be considered.
    pub const fn is_read_only(self) -> bool {
        matches!(
            self,
            Self::Discovery
                | Self::DescribeBackend
                | Self::GetCapabilities
                | Self::GetCalibration
                | Self::GetHealth
                | Self::GetQueue
                | Self::GetJobStatus
                | Self::GetResult
        )
    }

    /// Returns whether this operation may create or mutate remote state.
    pub const fn is_mutating(self) -> bool {
        matches!(
            self,
            Self::Submit | Self::Cancel | Self::Custom
        )
    }
}

// =============================================================================
// Header handling
// =============================================================================

/// A deterministic collection of transport headers.
///
/// Header values are deliberately redacted in `Debug`.
#[derive(Clone, PartialEq, Eq, Default)]
pub struct ProviderHeaders {
    values: BTreeMap<String, String>,
}

impl ProviderHeaders {
    /// Creates an empty header set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a header after validation.
    ///
    /// Headers whose names indicate secret material are rejected by the
    /// generic layer. Authentication modules should own authenticated
    /// transport construction rather than placing secrets in this layer.
    pub fn insert(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), GenericAdapterError> {
        let name = normalize_header_name(name.into());
        let value = value.into();

        validate_header_name(&name)?;
        validate_header_value(&value)?;

        if contains_secret_marker(&name) {
            return Err(GenericAdapterError::SecretMaterialRejected);
        }

        if self.values.len() >= MAX_HEADERS
            && !self.values.contains_key(&name)
        {
            return Err(GenericAdapterError::LimitExceeded {
                field: "headers",
                maximum: MAX_HEADERS,
            });
        }

        self.values.insert(name, value);

        Ok(())
    }

    /// Retrieves a header value.
    ///
    /// This accessor is intended for non-sensitive headers only.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.values
            .get(&normalize_header_name(name.to_owned()))
            .map(String::as_str)
    }

    /// Returns the number of headers.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether there are no headers.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns deterministic header names.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.values.keys().map(String::as_str)
    }

    /// Returns a safe iterator for diagnostics.
    ///
    /// Values are replaced with `<redacted>`.
    pub fn redacted(&self) -> impl Iterator<Item = (&str, &'static str)> {
        self.values
            .keys()
            .map(|name| (name.as_str(), "<redacted>"))
    }
}

impl fmt::Debug for ProviderHeaders {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = formatter.debug_map();

        for name in self.values.keys() {
            debug.entry(&name, &"<redacted>");
        }

        debug.finish()
    }
}

// =============================================================================
// Query parameters
// =============================================================================

/// Deterministic provider query parameters.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProviderQuery {
    values: BTreeMap<String, String>,
}

impl ProviderQuery {
    /// Creates an empty query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a validated query parameter.
    pub fn insert(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), GenericAdapterError> {
        let name = name.into();
        let value = value.into();

        validate_bounded_string(
            "query_name",
            &name,
            MAX_QUERY_NAME_LENGTH,
        )?;

        validate_bounded_string(
            "query_value",
            &value,
            MAX_QUERY_VALUE_LENGTH,
        )?;

        if self.values.len() >= MAX_QUERY_PARAMETERS
            && !self.values.contains_key(&name)
        {
            return Err(GenericAdapterError::LimitExceeded {
                field: "query_parameters",
                maximum: MAX_QUERY_PARAMETERS,
            });
        }

        self.values.insert(name, value);

        Ok(())
    }

    /// Returns a query parameter.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.values.get(name).map(String::as_str)
    }

    /// Returns whether no query parameters exist.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the number of parameters.
    pub fn len(&self) -> usize {
        self.values.len()
    }
}

// =============================================================================
// Request
// =============================================================================

/// Generic provider transport request.
///
/// The request is intentionally independent of HTTP.
///
/// A concrete transport can translate it to:
//!
// - HTTP;
// - REST;
// - GraphQL;
// - gRPC;
// - SDK calls;
// - local IPC;
// - another provider protocol.
#[derive(Clone, PartialEq, Eq)]
pub struct ProviderRequest {
    /// Semantic operation being requested.
    pub operation: ProviderOperation,

    /// Transport method.
    pub method: TransportMethod,

    /// Provider-specific path or operation target.
    pub target: String,

    /// Query parameters.
    pub query: ProviderQuery,

    /// Non-secret headers.
    pub headers: ProviderHeaders,

    /// Optional request body.
    pub body: Option<Vec<u8>>,

    /// Caller correlation identifier.
    pub request_id: String,

    /// Optional idempotency key.
    pub idempotency_key: Option<String>,
}

impl ProviderRequest {
    /// Creates a request builder.
    pub fn builder(
        operation: ProviderOperation,
        method: TransportMethod,
        target: impl Into<String>,
        request_id: impl Into<String>,
    ) -> ProviderRequestBuilder {
        ProviderRequestBuilder {
            operation,
            method,
            target: target.into(),
            query: ProviderQuery::new(),
            headers: ProviderHeaders::new(),
            body: None,
            request_id: request_id.into(),
            idempotency_key: None,
        }
    }

    /// Validates the complete request.
    pub fn validate(&self) -> Result<(), GenericAdapterError> {
        validate_bounded_string(
            "target",
            &self.target,
            MAX_ENDPOINT_LENGTH,
        )?;

        validate_identifier(
            "request_id",
            &self.request_id,
            MAX_REQUEST_ID_LENGTH,
        )?;

        if let Some(key) = &self.idempotency_key {
            validate_identifier(
                "idempotency_key",
                key,
                MAX_IDEMPOTENCY_KEY_LENGTH,
            )?;
        }

        if let Some(body) = &self.body {
            if body.len() > MAX_REQUEST_BODY_BYTES {
                return Err(GenericAdapterError::LimitExceeded {
                    field: "request_body",
                    maximum: MAX_REQUEST_BODY_BYTES,
                });
            }
        }

        Ok(())
    }

    /// Returns whether the request has a body.
    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    /// Returns the request payload size.
    pub fn body_len(&self) -> usize {
        self.body.as_ref().map_or(0, Vec::len)
    }
}

impl fmt::Debug for ProviderRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderRequest")
            .field("operation", &self.operation)
            .field("method", &self.method)
            .field("target", &self.target)
            .field("query", &self.query)
            .field("headers", &self.headers)
            .field("body_len", &self.body_len())
            .field("request_id", &self.request_id)
            .field(
                "idempotency_key",
                &self.idempotency_key.as_ref().map(|_| "<redacted>"),
            )
            .finish()
    }
}

/// Builder for `ProviderRequest`.
#[derive(Debug, Clone)]
pub struct ProviderRequestBuilder {
    operation: ProviderOperation,
    method: TransportMethod,
    target: String,
    query: ProviderQuery,
    headers: ProviderHeaders,
    body: Option<Vec<u8>>,
    request_id: String,
    idempotency_key: Option<String>,
}

impl ProviderRequestBuilder {
    /// Adds a query parameter.
    pub fn query(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, GenericAdapterError> {
        self.query.insert(name, value)?;
        Ok(self)
    }

    /// Adds a safe transport header.
    pub fn header(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, GenericAdapterError> {
        self.headers.insert(name, value)?;
        Ok(self)
    }

    /// Sets the request body.
    pub fn body(
        mut self,
        body: impl Into<Vec<u8>>,
    ) -> Result<Self, GenericAdapterError> {
        let body = body.into();

        if body.len() > MAX_REQUEST_BODY_BYTES {
            return Err(GenericAdapterError::LimitExceeded {
                field: "request_body",
                maximum: MAX_REQUEST_BODY_BYTES,
            });
        }

        if body.is_empty() {
            return Err(GenericAdapterError::EmptyPayload {
                field: "request_body",
            });
        }

        self.body = Some(body);

        Ok(self)
    }

    /// Sets an idempotency key.
    pub fn idempotency_key(
        mut self,
        key: impl Into<String>,
    ) -> Result<Self, GenericAdapterError> {
        let key = key.into();

        validate_identifier(
            "idempotency_key",
            &key,
            MAX_IDEMPOTENCY_KEY_LENGTH,
        )?;

        self.idempotency_key = Some(key);

        Ok(self)
    }

    /// Finalizes and validates the request.
    pub fn build(self) -> Result<ProviderRequest, GenericAdapterError> {
        let request = ProviderRequest {
            operation: self.operation,
            method: self.method,
            target: self.target,
            query: self.query,
            headers: self.headers,
            body: self.body,
            request_id: self.request_id,
            idempotency_key: self.idempotency_key,
        };

        request.validate()?;

        Ok(request)
    }
}

// =============================================================================
// Response
// =============================================================================

/// Normalized transport response.
///
/// Provider adapters translate this into backend-specific semantics and then
/// into the canonical `QuantumBackendAdapter` result model.
#[derive(Clone, PartialEq, Eq)]
pub struct ProviderResponse {
    /// Transport status code.
///
/// For non-HTTP transports this may be a provider-defined normalized code.
    pub status_code: u16,

    /// Response headers.
    pub headers: ProviderHeaders,

    /// Response payload.
    pub body: Vec<u8>,

    /// Correlation/request identifier returned by the transport, when present.
    pub request_id: Option<String>,

    /// Provider API version observed in the response, when available.
    pub provider_api_version: Option<String>,

    /// Rate-limit information.
    pub rate_limit: Option<RateLimitInfo>,
}

impl ProviderResponse {
    /// Creates a validated response.
    pub fn new(
        status_code: u16,
        headers: ProviderHeaders,
        body: Vec<u8>,
        request_id: Option<String>,
        provider_api_version: Option<String>,
        rate_limit: Option<RateLimitInfo>,
    ) -> Result<Self, GenericAdapterError> {
        if body.len() > MAX_RESPONSE_BODY_BYTES {
            return Err(GenericAdapterError::LimitExceeded {
                field: "response_body",
                maximum: MAX_RESPONSE_BODY_BYTES,
            });
        }

        if let Some(request_id) = &request_id {
            validate_identifier(
                "response_request_id",
                request_id,
                MAX_REQUEST_ID_LENGTH,
            )?;
        }

        if let Some(version) = &provider_api_version {
            validate_identifier(
                "provider_api_version",
                version,
                MAX_PROVIDER_API_VERSION_LENGTH,
            )?;
        }

        Ok(Self {
            status_code,
            headers,
            body,
            request_id,
            provider_api_version,
            rate_limit,
        })
    }

    /// Returns whether the normalized transport status indicates success.
    pub const fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }

    /// Returns whether the response indicates a client-side failure.
    pub const fn is_client_error(&self) -> bool {
        self.status_code >= 400 && self.status_code < 500
    }

    /// Returns whether the response indicates a server/provider failure.
    pub const fn is_server_error(&self) -> bool {
        self.status_code >= 500 && self.status_code < 600
    }

    /// Returns the payload length.
    pub fn body_len(&self) -> usize {
        self.body.len()
    }
}

impl fmt::Debug for ProviderResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderResponse")
            .field("status_code", &self.status_code)
            .field("headers", &self.headers)
            .field("body_len", &self.body.len())
            .field("request_id", &self.request_id)
            .field("provider_api_version", &self.provider_api_version)
            .field("rate_limit", &self.rate_limit)
            .finish()
    }
}

// =============================================================================
// Rate limiting
// =============================================================================

/// Provider rate-limit metadata.
///
/// The generic layer records provider information but does not implement retry
/// loops. Retry policy belongs to the execution/provider layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimitInfo {
    /// Remaining operations, when known.
    pub remaining: Option<u64>,

    /// Maximum operations in the current window, when known.
    pub limit: Option<u64>,

    /// Duration until the rate-limit window resets, when known.
    pub reset_after: Option<Duration>,
}

impl RateLimitInfo {
    /// Creates empty rate-limit metadata.
    pub const fn unknown() -> Self {
        Self {
            remaining: None,
            limit: None,
            reset_after: None,
        }
    }

    /// Returns true when the provider indicates that the caller is exhausted.
    pub const fn exhausted(self) -> bool {
        matches!(self.remaining, Some(0))
    }
}

// =============================================================================
// Pagination
// =============================================================================

/// Generic pagination state.
///
/// Provider adapters may map this to page numbers, opaque cursors, continuation
/// tokens, or other mechanisms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pagination {
    /// Optional opaque continuation token.
    pub next_token: Option<String>,

    /// Whether another page is available.
    pub has_more: bool,
}

impl Pagination {
    /// Creates a terminal page.
    pub const fn complete() -> Self {
        Self {
            next_token: None,
            has_more: false,
        }
    }

    /// Creates a page with a validated continuation token.
    pub fn next(
        token: impl Into<String>,
    ) -> Result<Self, GenericAdapterError> {
        let token = token.into();

        validate_bounded_string(
            "page_token",
            &token,
            MAX_PAGE_TOKEN_LENGTH,
        )?;

        if token.is_empty() {
            return Err(GenericAdapterError::EmptyPayload {
                field: "page_token",
            });
        }

        Ok(Self {
            next_token: Some(token),
            has_more: true,
        })
    }
}

// =============================================================================
// Retry classification
// =============================================================================

/// Classification of whether an operation may be retried.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RetryClass {
    /// Retry is explicitly safe.
    Safe,

    /// Retry may be safe after checking idempotency/execution state.
    Conditional,

    /// Retrying is unsafe or meaningless.
    DoNotRetry,
}

impl RetryClass {
    /// Returns true when an automated retry may be attempted without additional
    /// semantic inspection.
    pub const fn is_automatically_retryable(self) -> bool {
        matches!(self, Self::Safe)
    }
}

// =============================================================================
// Provider failure category
// =============================================================================

/// Provider-neutral provider failure category.
///
/// Provider-specific errors are mapped into this enum. Provider-specific
/// codes remain data, not variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProviderFailureCategory {
    /// Invalid caller request.
    InvalidRequest,

    /// Authentication failed.
    Authentication,

    /// Authorization failed.
    Authorization,

    /// Resource was not found.
    NotFound,

    /// Request was rejected due to provider rate limiting.
    RateLimited,

    /// Provider currently lacks capacity.
    Capacity,

    /// Provider/device is temporarily unavailable.
    Unavailable,

    /// Provider rejected an unsupported capability/format.
    Unsupported,

    /// Provider rejected the request because it is invalid for the current
    /// backend state.
    Conflict,

    /// Provider-side execution failure.
    Execution,

    /// Provider returned an invalid/malformed response.
    InvalidResponse,

    /// Transport failure before a provider response was obtained.
    Transport,

    /// Provider timeout.
    Timeout,

    /// Unknown provider category.
    Unknown,
}

impl ProviderFailureCategory {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidRequest => "invalid_request",
            Self::Authentication => "authentication",
            Self::Authorization => "authorization",
            Self::NotFound => "not_found",
            Self::RateLimited => "rate_limited",
            Self::Capacity => "capacity",
            Self::Unavailable => "unavailable",
            Self::Unsupported => "unsupported",
            Self::Conflict => "conflict",
            Self::Execution => "execution",
            Self::InvalidResponse => "invalid_response",
            Self::Transport => "transport",
            Self::Timeout => "timeout",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for ProviderFailureCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Provider error
// =============================================================================

/// Provider-neutral error returned by a generic adapter/transport.
///
/// This type intentionally does not contain provider SDK error types.
///
/// A concrete adapter may retain its native error internally and translate it
/// into this representation before returning it to the canonical hardware
/// layer.
#[derive(Clone, PartialEq, Eq)]
pub struct ProviderError {
    /// Stable generic category.
    pub category: ProviderFailureCategory,

    /// Safe provider-native error code, when known.
    pub provider_code: Option<String>,

    /// Safe human-readable provider message.
    pub message: String,

    /// Whether retrying may be appropriate.
    pub retry: RetryClass,

    /// HTTP/transport status code, when available.
    pub status_code: Option<u16>,

    /// Provider request/correlation identifier.
    pub request_id: Option<String>,

    /// Safe structured context.
    pub context: BTreeMap<String, String>,
}

impl ProviderError {
    /// Creates a validated provider error.
    pub fn new(
        category: ProviderFailureCategory,
        message: impl Into<String>,
        retry: RetryClass,
    ) -> Result<Self, GenericAdapterError> {
        let message = message.into();

        validate_bounded_string(
            "provider_error_message",
            &message,
            MAX_PROVIDER_ERROR_MESSAGE_LENGTH,
        )?;

        if message.is_empty() {
            return Err(GenericAdapterError::EmptyPayload {
                field: "provider_error_message",
            });
        }

        if contains_secret_marker(&message) {
            return Err(GenericAdapterError::SecretMaterialRejected);
        }

        Ok(Self {
            category,
            provider_code: None,
            message,
            retry,
            status_code: None,
            request_id: None,
            context: BTreeMap::new(),
        })
    }

    /// Adds a safe provider-native code.
    pub fn with_provider_code(
        mut self,
        code: impl Into<String>,
    ) -> Result<Self, GenericAdapterError> {
        let code = code.into();

        validate_bounded_string(
            "provider_error_code",
            &code,
            MAX_PROVIDER_ERROR_CODE_LENGTH,
        )?;

        if contains_secret_marker(&code) {
            return Err(GenericAdapterError::SecretMaterialRejected);
        }

        self.provider_code = Some(code);

        Ok(self)
    }

    /// Adds a safe provider request ID.
    pub fn with_request_id(
        mut self,
        request_id: impl Into<String>,
    ) -> Result<Self, GenericAdapterError> {
        let request_id = request_id.into();

        validate_identifier(
            "provider_error_request_id",
            &request_id,
            MAX_REQUEST_ID_LENGTH,
        )?;

        self.request_id = Some(request_id);

        Ok(self)
    }

    /// Adds safe structured context.
    pub fn with_context(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, GenericAdapterError> {
        let key = key.into();
        let value = value.into();

        validate_bounded_string(
            "provider_error_context_key",
            &key,
            MAX_PROVIDER_ERROR_CONTEXT_KEY_LENGTH,
        )?;

        validate_bounded_string(
            "provider_error_context_value",
            &value,
            MAX_PROVIDER_ERROR_CONTEXT_VALUE_LENGTH,
        )?;

        if contains_secret_marker(&key)
            || contains_secret_marker(&value)
        {
            return Err(GenericAdapterError::SecretMaterialRejected);
        }

        if self.context.len() >= MAX_PROVIDER_ERROR_CONTEXT_FIELDS
            && !self.context.contains_key(&key)
        {
            return Err(GenericAdapterError::LimitExceeded {
                field: "provider_error_context",
                maximum: MAX_PROVIDER_ERROR_CONTEXT_FIELDS,
            });
        }

        self.context.insert(key, value);

        Ok(self)
    }
}

impl fmt::Debug for ProviderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderError")
            .field("category", &self.category)
            .field("provider_code", &self.provider_code)
            .field("message", &self.message)
            .field("retry", &self.retry)
            .field("status_code", &self.status_code)
            .field("request_id", &self.request_id)
            .field("context", &self.context)
            .finish()
    }
}

impl fmt::Display for ProviderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {}",
            self.category,
            self.message
        )?;

        if let Some(code) = &self.provider_code {
            write!(formatter, " (provider_code={})", code)?;
        }

        Ok(())
    }
}

impl Error for ProviderError {}

// =============================================================================
// Generic adapter errors
// =============================================================================

/// Errors produced by the generic adapter layer itself.
///
/// These are construction/validation errors, not provider execution errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericAdapterError {
    /// Required value was empty.
    EmptyPayload {
        /// Field that was empty.
        field: &'static str,
    },

    /// A field exceeded its maximum allowed length/size.
    LimitExceeded {
        /// Field being limited.
        field: &'static str,

        /// Maximum allowed value.
        maximum: usize,
    },

    /// An identifier failed validation.
    InvalidIdentifier {
        /// Identifier field.
        field: &'static str,

        /// Reason for rejection.
        reason: &'static str,
    },

    /// A value contains material that resembles credentials/secrets.
    SecretMaterialRejected,

    /// Unsupported operation/method combination.
    InvalidOperation,

    /// Request cannot be safely retried.
    NonRetryableRequest,

    /// Provider response was structurally invalid.
    InvalidResponse {
        /// Reason.
        reason: &'static str,
    },
}

impl fmt::Display for GenericAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPayload { field } => {
                write!(formatter, "{} must not be empty", field)
            }

            Self::LimitExceeded { field, maximum } => {
                write!(
                    formatter,
                    "{} exceeds maximum allowed size {}",
                    field,
                    maximum
                )
            }

            Self::InvalidIdentifier { field, reason } => {
                write!(
                    formatter,
                    "invalid {}: {}",
                    field,
                    reason
                )
            }

            Self::SecretMaterialRejected => {
                formatter.write_str(
                    "value rejected because it appears to contain secret material",
                )
            }

            Self::InvalidOperation => {
                formatter.write_str(
                    "transport operation is invalid for the requested operation",
                )
            }

            Self::NonRetryableRequest => {
                formatter.write_str(
                    "request is not safely retryable",
                )
            }

            Self::InvalidResponse { reason } => {
                write!(
                    formatter,
                    "invalid provider response: {}",
                    reason
                )
            }
        }
    }
}

impl Error for GenericAdapterError {}

// =============================================================================
// Transport error
// =============================================================================

/// Errors produced by the underlying transport implementation.
///
/// The generic adapter does not implement the transport itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportError {
    /// The transport could not establish communication.
    Unavailable {
        /// Safe diagnostic message.
        message: String,
    },

    /// Transport-level timeout.
    Timeout,

    /// Transport-level cancellation.
    Cancelled,

    /// TLS/security failure.
    Security {
        /// Safe diagnostic message.
        message: String,
    },

    /// Invalid transport response.
    InvalidResponse {
        /// Safe diagnostic message.
        message: String,
    },

    /// Transport-specific failure.
    Other {
        /// Safe diagnostic message.
        message: String,
    },
}

impl fmt::Display for TransportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unavailable { message } => {
                write!(formatter, "transport unavailable: {}", message)
            }

            Self::Timeout => {
                formatter.write_str("transport timeout")
            }

            Self::Cancelled => {
                formatter.write_str("transport cancelled")
            }

            Self::Security { message } => {
                write!(formatter, "transport security failure: {}", message)
            }

            Self::InvalidResponse { message } => {
                write!(
                    formatter,
                    "transport returned invalid response: {}",
                    message
                )
            }

            Self::Other { message } => {
                write!(formatter, "transport failure: {}", message)
            }
        }
    }
}

impl Error for TransportError {}

// =============================================================================
// Transport trait
// =============================================================================

/// Provider-neutral transport interface.
///
/// This is intentionally synchronous and object-safe so it remains compatible
/// with Rust 1.97 without requiring an async-trait dependency.
///
/// A concrete implementation may internally use:
///
/// - a blocking HTTP client;
/// - an SDK;
/// - a local simulator;
/// - an RPC client;
/// - another execution mechanism.
///
/// Asynchronous provider execution is represented by the quantum job lifecycle,
/// not by forcing this low-level trait to require an async runtime.
///
/// # Security
///
/// Implementations are responsible for authentication and secure transport.
///
/// The generic layer never reads credentials.
pub trait ProviderTransport: Send + Sync {
    /// Sends one validated provider request.
    fn send(
        &self,
        request: &ProviderRequest,
    ) -> Result<ProviderResponse, TransportError>;

    /// Returns a stable transport implementation identifier.
    fn transport_id(&self) -> &str;

    /// Returns the transport implementation version.
    fn transport_version(&self) -> &str;
}

// =============================================================================
// Transport invocation helper
// =============================================================================

/// Executes a validated request through a transport.
///
/// This helper centralizes the generic boundary between transport failures and
/// provider-neutral failures.
///
/// Provider adapters remain responsible for mapping provider-specific HTTP/SDK
/// status semantics into `ProviderError`.
pub fn send_request<T>(
    transport: &T,
    request: &ProviderRequest,
) -> Result<ProviderResponse, ProviderError>
where
    T: ProviderTransport + ?Sized,
{
    request
        .validate()
        .map_err(|error| {
            ProviderError::new(
                ProviderFailureCategory::InvalidRequest,
                error.to_string(),
                RetryClass::DoNotRetry,
            )
            .expect("static generic error message is valid")
        })?;

    match transport.send(request) {
        Ok(response) => {
            if response.status_code == 0 {
                return Err(
                    ProviderError::new(
                        ProviderFailureCategory::InvalidResponse,
                        "transport returned status code zero",
                        RetryClass::DoNotRetry,
                    )
                    .expect("static provider error is valid"),
                );
            }

            Ok(response)
        }

        Err(TransportError::Timeout) => Err(
            ProviderError::new(
                ProviderFailureCategory::Timeout,
                "provider transport timed out",
                retry_class_for_operation(request.operation),
            )
            .expect("static provider error is valid"),
        ),

        Err(TransportError::Cancelled) => Err(
            ProviderError::new(
                ProviderFailureCategory::Transport,
                "provider transport was cancelled",
                RetryClass::Conditional,
            )
            .expect("static provider error is valid"),
        ),

        Err(TransportError::Unavailable { message }) => Err(
            ProviderError::new(
                ProviderFailureCategory::Unavailable,
                message,
                RetryClass::Conditional,
            )
            .expect("validated transport message should be valid"),
        ),

        Err(TransportError::Security { message }) => Err(
            ProviderError::new(
                ProviderFailureCategory::Transport,
                message,
                RetryClass::DoNotRetry,
            )
            .expect("validated transport message should be valid"),
        ),

        Err(TransportError::InvalidResponse { message }) => Err(
            ProviderError::new(
                ProviderFailureCategory::InvalidResponse,
                message,
                RetryClass::DoNotRetry,
            )
            .expect("validated transport message should be valid"),
        ),

        Err(TransportError::Other { message }) => Err(
            ProviderError::new(
                ProviderFailureCategory::Transport,
                message,
                retry_class_for_operation(request.operation),
            )
            .expect("validated transport message should be valid"),
        ),
    }
}

// =============================================================================
// Response classification
// =============================================================================

/// Classifies a normalized transport response into a provider-neutral error.
///
/// A successful response returns `Ok(())`.
///
/// A non-success response returns `ProviderError`.
///
/// The body is deliberately NOT parsed here because provider-specific error
/// schemas belong to concrete adapters.
pub fn classify_response(
    operation: ProviderOperation,
    response: &ProviderResponse,
) -> Result<(), ProviderError> {
    if response.is_success() {
        return Ok(());
    }

    let category = match response.status_code {
        400 => ProviderFailureCategory::InvalidRequest,
        401 => ProviderFailureCategory::Authentication,
        403 => ProviderFailureCategory::Authorization,
        404 => ProviderFailureCategory::NotFound,
        409 => ProviderFailureCategory::Conflict,
        408 | 504 => ProviderFailureCategory::Timeout,
        429 => ProviderFailureCategory::RateLimited,
        500..=599 => ProviderFailureCategory::Unavailable,
        _ => ProviderFailureCategory::Unknown,
    };

    let retry = retry_class_for_status(
        operation,
        response.status_code,
    );

    let message = format!(
        "provider operation '{}' failed with transport status {}",
        operation.as_str(),
        response.status_code
    );

    let mut error = ProviderError::new(
        category,
        message,
        retry,
    )
    .expect("generated provider error is valid");

    error.status_code = Some(response.status_code);

    if let Some(request_id) = &response.request_id {
        error.request_id = Some(request_id.clone());
    }

    Ok(())
}

// =============================================================================
// Retry helpers
// =============================================================================

/// Determines the conservative retry classification for an operation.
pub const fn retry_class_for_operation(
    operation: ProviderOperation,
) -> RetryClass {
    match operation {
        ProviderOperation::Discovery
        | ProviderOperation::DescribeBackend
        | ProviderOperation::GetCapabilities
        | ProviderOperation::GetCalibration
        | ProviderOperation::GetHealth
        | ProviderOperation::GetQueue
        | ProviderOperation::GetJobStatus
        | ProviderOperation::GetResult => RetryClass::Safe,

        ProviderOperation::Submit => RetryClass::Conditional,

        ProviderOperation::Cancel => RetryClass::Conditional,

        ProviderOperation::Custom => RetryClass::Conditional,
    }
}

/// Determines retry classification from a transport status.
pub const fn retry_class_for_status(
    operation: ProviderOperation,
    status_code: u16,
) -> RetryClass {
    match status_code {
        408 | 429 | 500 | 502 | 503 | 504 => {
            match operation {
                ProviderOperation::Submit
                | ProviderOperation::Cancel
                | ProviderOperation::Custom => {
                    RetryClass::Conditional
                }

                _ => RetryClass::Safe,
            }
        }

        400 | 401 | 403 | 404 | 409 => RetryClass::DoNotRetry,

        _ if status_code >= 500 => {
            retry_class_for_operation(operation)
        }

        _ => RetryClass::DoNotRetry,
    }
}

// =============================================================================
// Format negotiation
// =============================================================================

/// Result of generic program-format negotiation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatNegotiation {
    /// The requested format is directly supported.
    Direct,

    /// The requested format can be transformed by another subsystem.
    ///
    /// The generic adapter does not perform that transformation.
    RequiresTranslation {
        /// Supported target format to which another subsystem may translate.
        target_format: String,
    },

    /// The provider cannot execute the requested format.
    Unsupported,
}

impl FormatNegotiation {
    /// Returns true when direct execution is possible.
    pub const fn is_direct(&self) -> bool {
        matches!(self, Self::Direct)
    }
}

/// Negotiates a format against an adapter's supported format set.
pub fn negotiate_format(
    requested: &str,
    supported: &BTreeSet<String>,
) -> Result<FormatNegotiation, GenericAdapterError> {
    let requested = normalize_identifier(requested.to_owned());

    validate_identifier(
        "requested_format",
        &requested,
        MAX_FORMAT_IDENTIFIER_LENGTH,
    )?;

    if supported.contains(&requested) {
        return Ok(FormatNegotiation::Direct);
    }

    Ok(FormatNegotiation::Unsupported)
}

// =============================================================================
// Capability negotiation
// =============================================================================

/// Result of generic capability negotiation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityNegotiation {
    /// Capabilities that were present.
    pub satisfied: BTreeSet<String>,

    /// Capabilities that were missing.
    pub missing: BTreeSet<String>,
}

impl CapabilityNegotiation {
    /// Returns whether all requested capabilities are satisfied.
    pub fn is_compatible(&self) -> bool {
        self.missing.is_empty()
    }
}

/// Negotiates requested capabilities against supported capabilities.
///
/// This function performs exact stable identifier matching. More sophisticated
/// semantic capability decomposition belongs in `compatibility.rs`.
pub fn negotiate_capabilities(
    requested: &BTreeSet<String>,
    supported: &BTreeSet<String>,
) -> Result<CapabilityNegotiation, GenericAdapterError> {
    if requested.len() > MAX_CAPABILITIES {
        return Err(GenericAdapterError::LimitExceeded {
            field: "requested_capabilities",
            maximum: MAX_CAPABILITIES,
        });
    }

    let mut satisfied = BTreeSet::new();
    let mut missing = BTreeSet::new();

    for capability in requested {
        let normalized = normalize_identifier(capability.clone());

        validate_identifier(
            "capability",
            &normalized,
            MAX_CAPABILITY_IDENTIFIER_LENGTH,
        )?;

        if supported.contains(&normalized) {
            satisfied.insert(normalized);
        } else {
            missing.insert(normalized);
        }
    }

    Ok(CapabilityNegotiation {
        satisfied,
        missing,
    })
}

// =============================================================================
// Idempotency
// =============================================================================

/// Determines whether a request has sufficient semantics for safe retry.
///
/// `Submit` requires an explicit idempotency key because blindly repeating a
/// quantum submission could create duplicate physical executions.
pub fn validate_retry_safety(
    request: &ProviderRequest,
) -> Result<(), GenericAdapterError> {
    if retry_class_for_operation(request.operation)
        == RetryClass::Safe
    {
        return Ok(());
    }

    if request.method.is_normally_idempotent() {
        return Ok(());
    }

    if request.idempotency_key.is_some() {
        return Ok(());
    }

    Err(GenericAdapterError::NonRetryableRequest)
}

// =============================================================================
// Provider adapter trait
// =============================================================================

/// Minimal provider-neutral adapter information contract.
///
/// This trait intentionally does not replace `QuantumBackendAdapter` in
/// `backend_trait.rs`.
///
/// It supplies reusable metadata/transport behavior for concrete adapters.
///
/// A concrete adapter normally has the form:
///
/// ```text
/// struct IbmAdapter<T> {
///     metadata: AdapterMetadata,
///     transport: T,
/// }
/// ```
///
/// and implements both:
///
/// ```text
/// GenericAdapter
/// QuantumBackendAdapter
/// ```
pub trait GenericAdapter: Send + Sync {
    /// Returns immutable adapter metadata.
    fn metadata(&self) -> &AdapterMetadata;

    /// Returns the underlying provider-neutral transport.
    fn transport(&self) -> &dyn ProviderTransport;

    /// Performs one generic provider request.
    fn request(
        &self,
        request: &ProviderRequest,
    ) -> Result<ProviderResponse, ProviderError> {
        send_request(self.transport(), request)
    }
}

// =============================================================================
// Backend error conversion
// =============================================================================

/// Converts a generic provider error into the legacy backend error surface.
///
/// The canonical `hardware/errors.rs` system is the intended long-term owner
/// of the complete error taxonomy. Until that migration is complete,
/// `backend.rs::BackendError` remains the compatibility boundary.
///
/// This function deliberately maps only to semantics that are already exposed
/// by the repository's current backend contract.
pub fn provider_error_to_backend_error(
    error: &ProviderError,
) -> BackendError {
    match error.category {
        ProviderFailureCategory::Unsupported
        | ProviderFailureCategory::InvalidRequest
        | ProviderFailureCategory::Conflict
        | ProviderFailureCategory::InvalidResponse => {
            BackendError::ExecutionUnavailable
        }

        ProviderFailureCategory::Authentication
        | ProviderFailureCategory::Authorization
        | ProviderFailureCategory::NotFound
        | ProviderFailureCategory::RateLimited
        | ProviderFailureCategory::Capacity
        | ProviderFailureCategory::Unavailable
        | ProviderFailureCategory::Execution
        | ProviderFailureCategory::Transport
        | ProviderFailureCategory::Timeout
        | ProviderFailureCategory::Unknown => {
            BackendError::ExecutionUnavailable
        }
    }
}

// =============================================================================
// Secret detection
// =============================================================================

/// Returns true if a field name/value contains a known secret marker.
///
/// This is intentionally conservative.
///
/// False positives are preferable to allowing credential material into the
/// generic metadata/error/request surface.
fn contains_secret_marker(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();

    const MARKERS: &[&str] = &[
        "authorization",
        "authenticate",
        "authentication",
        "api_key",
        "apikey",
        "access_token",
        "accesstoken",
        "refresh_token",
        "refreshtoken",
        "bearer",
        "password",
        "passwd",
        "private_key",
        "privatekey",
        "secret",
        "credential",
        "credentials",
        "cookie",
        "session_token",
        "sessiontoken",
    ];

    MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), GenericAdapterError> {
    if value.is_empty() {
        return Err(GenericAdapterError::EmptyPayload { field });
    }

    if value.len() > maximum {
        return Err(GenericAdapterError::LimitExceeded {
            field,
            maximum,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(GenericAdapterError::InvalidIdentifier {
            field,
            reason: "control characters are forbidden",
        });
    }

    if value.trim() != value {
        return Err(GenericAdapterError::InvalidIdentifier {
            field,
            reason: "leading or trailing whitespace is forbidden",
        });
    }

    if contains_secret_marker(value) {
        return Err(GenericAdapterError::SecretMaterialRejected);
    }

    Ok(())
}

fn validate_bounded_string(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), GenericAdapterError> {
    if value.len() > maximum {
        return Err(GenericAdapterError::LimitExceeded {
            field,
            maximum,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(GenericAdapterError::InvalidIdentifier {
            field,
            reason: "control characters are forbidden",
        });
    }

    Ok(())
}

fn validate_header_name(
    name: &str,
) -> Result<(), GenericAdapterError> {
    validate_bounded_string(
        "header_name",
        name,
        MAX_HEADER_NAME_LENGTH,
    )?;

    if name.is_empty() {
        return Err(GenericAdapterError::EmptyPayload {
            field: "header_name",
        });
    }

    Ok(())
}

fn validate_header_value(
    value: &str,
) -> Result<(), GenericAdapterError> {
    validate_bounded_string(
        "header_value",
        value,
        MAX_HEADER_VALUE_LENGTH,
    )
}

fn validate_metadata_key(
    key: &str,
) -> Result<(), GenericAdapterError> {
    validate_bounded_string(
        "metadata_key",
        key,
        MAX_HEADER_NAME_LENGTH,
    )?;

    if key.is_empty() {
        return Err(GenericAdapterError::EmptyPayload {
            field: "metadata_key",
        });
    }

    Ok(())
}

fn validate_metadata_value(
    value: &str,
) -> Result<(), GenericAdapterError> {
    validate_bounded_string(
        "metadata_value",
        value,
        MAX_PROVIDER_ERROR_MESSAGE_LENGTH,
    )
}

fn normalize_identifier(value: String) -> String {
    value.trim().to_ascii_lowercase()
}

fn normalize_header_name(value: String) -> String {
    value.trim().to_ascii_lowercase()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestTransport;

    impl ProviderTransport for TestTransport {
        fn send(
            &self,
            request: &ProviderRequest,
        ) -> Result<ProviderResponse, TransportError> {
            assert_eq!(
                request.operation,
                ProviderOperation::Discovery
            );

            ProviderResponse::new(
                200,
                ProviderHeaders::new(),
                b"{}".to_vec(),
                Some(request.request_id.clone()),
                Some("test-v1".to_owned()),
                None,
            )
            .map_err(|error| TransportError::InvalidResponse {
                message: error.to_string(),
            })
        }

        fn transport_id(&self) -> &str {
            "test"
        }

        fn transport_version(&self) -> &str {
            "1.0.0"
        }
    }

    struct FailingTransport;

    impl ProviderTransport for FailingTransport {
        fn send(
            &self,
            _request: &ProviderRequest,
        ) -> Result<ProviderResponse, TransportError> {
            Err(TransportError::Timeout)
        }

        fn transport_id(&self) -> &str {
            "failing-test"
        }

        fn transport_version(&self) -> &str {
            "1.0.0"
        }
    }

    #[test]
    fn adapter_identity_is_deterministic() {
        let identity = AdapterIdentity::new(
            "zamani.hardware.test",
            "test",
            "1.0.0",
            Some("v1".to_owned()),
        )
        .expect("identity must be valid");

        assert_eq!(
            identity.adapter_id(),
            "zamani.hardware.test"
        );
        assert_eq!(identity.provider_id(), "test");
        assert_eq!(identity.adapter_version(), "1.0.0");
        assert_eq!(
            identity.provider_api_version(),
            Some("v1")
        );
    }

    #[test]
    fn adapter_metadata_is_deterministic() {
        let identity = AdapterIdentity::new(
            "zamani.hardware.test",
            "test",
            "1.0.0",
            None,
        )
        .expect("identity must be valid");

        let metadata = AdapterMetadata::new(
            identity,
            "Test Adapter",
        )
        .expect("metadata must be valid")
        .with_format("openqasm-3.1")
        .expect("format must be valid")
        .with_format("qir")
        .expect("format must be valid")
        .with_capability("dynamic_circuits")
        .expect("capability must be valid");

        assert!(metadata.supported_formats.contains("qir"));
        assert!(
            metadata
                .supported_formats
                .contains("openqasm-3.1")
        );
        assert!(
            metadata
                .capabilities
                .contains("dynamic_circuits")
        );
    }

    #[test]
    fn request_builder_validates_and_builds() {
        let request = ProviderRequest::builder(
            ProviderOperation::Submit,
            TransportMethod::Post,
            "/jobs",
            "request-001",
        )
        .expect("builder construction should not fail")
        .header("content-type", "application/json")
        .expect("header should be valid")
        .body(b"{}".to_vec())
        .expect("body should be valid")
        .idempotency_key("idem-001")
        .expect("idempotency key should be valid")
        .build()
        .expect("request should be valid");

        assert_eq!(
            request.operation,
            ProviderOperation::Submit
        );
        assert_eq!(request.method, TransportMethod::Post);
        assert_eq!(request.body_len(), 2);
        assert_eq!(
            request.headers.get("content-type"),
            Some("application/json")
        );
    }

    #[test]
    fn request_debug_does_not_expose_body() {
        let request = ProviderRequest::builder(
            ProviderOperation::Submit,
            TransportMethod::Post,
            "/jobs",
            "request-001",
        )
        .body(b"very-sensitive-program".to_vec())
        .expect("body should be valid")
        .build()
        .expect("request should be valid");

        let debug = format!("{:?}", request);

        assert!(debug.contains("body_len"));
        assert!(!debug.contains("very-sensitive-program"));
    }

    #[test]
    fn secret_headers_are_rejected() {
        let mut headers = ProviderHeaders::new();

        let result = headers.insert(
            "Authorization",
            "Bearer secret",
        );

        assert_eq!(
            result,
            Err(GenericAdapterError::SecretMaterialRejected)
        );
    }

    #[test]
    fn secret_metadata_is_rejected() {
        let identity = AdapterIdentity::new(
            "zamani.hardware.test",
            "test",
            "1.0.0",
            None,
        )
        .expect("identity must be valid");

        let metadata = AdapterMetadata::new(
            identity,
            "Test",
        )
        .expect("metadata must be valid")
        .with_metadata(
            "api_key",
            "do-not-store",
        );

        assert_eq!(
            metadata,
            Err(GenericAdapterError::SecretMaterialRejected)
        );
    }

    #[test]
    fn format_negotiation_is_exact() {
        let mut supported = BTreeSet::new();
        supported.insert("qir".to_owned());

        assert_eq!(
            negotiate_format("qir", &supported)
                .expect("negotiation must succeed"),
            FormatNegotiation::Direct
        );

        assert_eq!(
            negotiate_format("openqasm-3.1", &supported)
                .expect("negotiation must succeed"),
            FormatNegotiation::Unsupported
        );
    }

    #[test]
    fn capability_negotiation_reports_missing_capabilities() {
        let requested = BTreeSet::from([
            "measurement".to_owned(),
            "dynamic_circuits".to_owned(),
        ]);

        let supported = BTreeSet::from([
            "measurement".to_owned(),
        ]);

        let result = negotiate_capabilities(
            &requested,
            &supported,
        )
        .expect("negotiation must succeed");

        assert!(
            result
                .satisfied
                .contains("measurement")
        );

        assert!(
            result
                .missing
                .contains("dynamic_circuits")
        );

        assert!(!result.is_compatible());
    }

    #[test]
    fn read_operations_are_safe_to_retry() {
        assert_eq!(
            retry_class_for_operation(
                ProviderOperation::GetHealth
            ),
            RetryClass::Safe
        );

        assert_eq!(
            retry_class_for_operation(
                ProviderOperation::GetResult
            ),
            RetryClass::Safe
        );
    }

    #[test]
    fn submission_requires_idempotency_for_retry() {
        let request = ProviderRequest::builder(
            ProviderOperation::Submit,
            TransportMethod::Post,
            "/jobs",
            "request-001",
        )
        .body(b"program".to_vec())
        .expect("body should be valid")
        .build()
        .expect("request should be valid");

        assert_eq!(
            validate_retry_safety(&request),
            Err(GenericAdapterError::NonRetryableRequest)
        );
    }

    #[test]
    fn submission_with_idempotency_is_retryable() {
        let request = ProviderRequest::builder(
            ProviderOperation::Submit,
            TransportMethod::Post,
            "/jobs",
            "request-001",
        )
        .body(b"program".to_vec())
        .expect("body should be valid")
        .idempotency_key("idem-001")
        .expect("idempotency key should be valid")
        .build()
        .expect("request should be valid");

        assert_eq!(
            validate_retry_safety(&request),
            Ok(())
        );
    }

    #[test]
    fn generic_transport_executes_request() {
        let request = ProviderRequest::builder(
            ProviderOperation::Discovery,
            TransportMethod::Get,
            "/devices",
            "request-001",
        )
        .build()
        .expect("request should be valid");

        let response = send_request(
            &TestTransport,
            &request,
        )
        .expect("transport should succeed");

        assert_eq!(response.status_code, 200);
        assert!(response.is_success());
    }

    #[test]
    fn transport_timeout_becomes_provider_error() {
        let request = ProviderRequest::builder(
            ProviderOperation::GetHealth,
            TransportMethod::Get,
            "/health",
            "request-001",
        )
        .build()
        .expect("request should be valid");

        let error = send_request(
            &FailingTransport,
            &request,
        )
        .expect_err("transport must fail");

        assert_eq!(
            error.category,
            ProviderFailureCategory::Timeout
        );

        assert_eq!(
            error.retry,
            RetryClass::Safe
        );
    }

    #[test]
    fn response_status_classification_is_provider_neutral() {
        let response = ProviderResponse::new(
            429,
            ProviderHeaders::new(),
            Vec::new(),
            Some("request-429".to_owned()),
            None,
            Some(RateLimitInfo {
                remaining: Some(0),
                limit: Some(100),
                reset_after: Some(Duration::from_secs(30)),
            }),
        )
        .expect("response should be valid");

        assert!(response.is_client_error());
        assert!(!response.is_success());

        let error = classify_response(
            ProviderOperation::GetHealth,
            &response,
        );

        assert!(error.is_err());
    }

    #[test]
    fn pagination_is_deterministic() {
        let page = Pagination::next("cursor-001")
            .expect("cursor should be valid");

        assert!(page.has_more);
        assert_eq!(
            page.next_token.as_deref(),
            Some("cursor-001")
        );

        assert_eq!(
            Pagination::complete(),
            Pagination {
                next_token: None,
                has_more: false,
            }
        );
    }

    #[test]
    fn provider_error_never_accepts_secret_context() {
        let error = ProviderError::new(
            ProviderFailureCategory::Execution,
            "execution failed",
            RetryClass::DoNotRetry,
        )
        .expect("error should be valid");

        let result = error.with_context(
            "access_token",
            "secret",
        );

        assert_eq!(
            result,
            Err(GenericAdapterError::SecretMaterialRejected)
        );
    }

    #[test]
    fn generic_error_conversion_has_no_provider_specific_variants() {
        let error = ProviderError::new(
            ProviderFailureCategory::RateLimited,
            "provider rate limit",
            RetryClass::Safe,
        )
        .expect("error should be valid");

        let backend_error =
            provider_error_to_backend_error(&error);

        assert_eq!(
            backend_error,
            BackendError::ExecutionUnavailable
        );
    }

    #[test]
    fn transport_is_object_safe() {
        let transport: Box<dyn ProviderTransport> =
            Box::new(TestTransport);

        assert_eq!(
            transport.transport_id(),
            "test"
        );
    }
}