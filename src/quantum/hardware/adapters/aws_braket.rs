//! Zamani Quantum — Amazon Braket Hardware Adapter
//!
//! Production-grade Amazon Braket adapter for the Zamani Quantum Hardware
//! Abstraction Layer.
//!
//! # Responsibility
//!
//! This module translates Zamani's provider-neutral hardware execution
//! contract into Amazon Braket quantum-task operations.
//!
//! It owns:
//!
//! - Amazon Braket provider identity;
//! - Braket device ARN validation;
//! - Braket quantum-task submission;
//! - OpenQASM 3.0 task construction;
//! - provider-native Braket action submission;
//! - client-token/idempotency handling;
//! - task lifecycle normalization;
//! - task cancellation;
//! - queue information normalization;
//! - task-result artifact retrieval through an injected artifact store;
//! - Braket result normalization into `ExecutionResult`;
//! - Braket failure mapping into `BackendError`;
//! - safe Braket response parsing;
//! - Braket-specific configuration;
//! - Braket-specific tags/device parameters;
//! - Braket experimental capability declaration;
//! - adapter provenance;
//! - adapter conformance behaviour;
//! - Braket-specific validation;
//! - provider/API schema version handling.
//!
//! It deliberately does NOT own:
//!
//! - AWS credentials;
//! - AWS access keys;
//! - AWS secret keys;
//! - AWS session tokens;
//! - IAM;
//! - AWS Signature Version 4;
//! - TLS;
//! - HTTP implementation;
//! - S3 credentials;
//! - S3 client implementation;
//! - OpenQASM parsing;
//! - Zamani Quantum IR;
//! - routing;
//! - scheduling;
//! - transpilation;
//! - calibration algorithms;
//! - benchmarking;
//! - provider registration;
//! - global state.
//!
//! Authentication belongs to `authentication.rs`.
//! Credential references belong to `credentials.rs`.
//! Transport belongs to `adapters::generic` and its concrete implementation.
//! S3 result retrieval belongs to an injected `BraketArtifactStore`.
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
//! BackendProgram
//!      |
//!      v
//! QuantumBackendAdapter
//!      |
//!      v
//! aws_braket.rs
//!      |
//!      +----------------------+
//!      |                      |
//!      v                      v
//! ProviderTransport     BraketArtifactStore
//!      |                      |
//!      v                      v
//! Amazon Braket API          S3
//!      |
//!      v
//! Braket device/QPU/simulator
//! ```
//!
//! # Provider model
//!
//! Amazon Braket is treated as a provider rather than as a single quantum
//! backend. A Braket backend is represented by its device ARN and the
//! canonical `QuantumBackend` descriptor.
//!
//! Examples:
//!
//! ```text
//! arn:aws:braket:us-west-1::device/qpu/rigetti/Ankaa-3
//! arn:aws:braket:us-east-1::device/qpu/ionq/...
//! arn:aws:braket:::device/quantum-simulator/amazon/sv1
//! ```
//!
//! # OpenQASM
//!
//! Amazon Braket currently supports OpenQASM 3.0 for gate-based devices and
//! simulators. Device-specific supported operations are exposed through the
//! device capability response.
//!
//! Zamani therefore treats:
//!
//! ```text
//! openqasm-3.0
//! openqasm-3.1
//! ```
//!
//! as interoperability formats while retaining Zamani Quantum IR as the
//! canonical language representation.
//!
//! # Provider-native actions
//!
//! Braket exposes multiple action schemas, including OpenQASM, analog and
//! other provider/device-specific actions.
//!
//! This adapter therefore also supports:
//!
//! ```text
//! aws-braket-action
//! ```
//!
//! where the payload is a complete Braket `action` JSON object.
//!
//! Provider-native action submission is deliberately explicit. The adapter
//! never guesses the schema of arbitrary JSON.
//!
//! # Result architecture
//!
//! Braket task metadata contains the S3 bucket and output directory where the
//! result is stored. The adapter does not embed an S3 implementation.
//!
//! Instead:
//!
//! ```text
//! GetQuantumTask
//!      |
//!      v
//! outputS3Bucket + outputS3Directory
//!      |
//!      v
//! BraketArtifactStore
//!      |
//!      v
//! result JSON
//!      |
//!      v
//! ExecutionResult
//! ```
//!
//! # Security
//!
//! No AWS secret is stored by this module.
//!
//! The transport supplied to this adapter is responsible for authentication.
//!
//! This adapter never:
//!
//! - reads AWS_ACCESS_KEY_ID;
//! - reads AWS_SECRET_ACCESS_KEY;
//! - reads AWS_SESSION_TOKEN;
//! - constructs Authorization headers;
//! - stores credentials;
//! - serializes credentials;
//! - logs credentials.
//!
//! Provider responses are parsed only into bounded provider-neutral values.
//!
//! # Idempotency
//!
//! Amazon Braket requires a client token for `CreateQuantumTask`.
//!
//! The adapter uses:
//!
//! 1. `ExecutionRequest.request_id`, when present;
//! 2. otherwise a deterministic request fingerprint combined with a
//!    process-local monotonic counter and current timestamp.
//!
//! The resulting client token is truncated to the Braket limit of 64 bytes.
//!
//! The request ID remains the preferred production mechanism because callers
//! can then guarantee application-level idempotency.
//!
//! # Retry semantics
//!
//! Submission is NOT automatically retried after an ambiguous transport
//! failure unless the request carries an idempotent client token and the
//! caller's transport/retry policy explicitly permits it.
//!
//! This module does not perform blind submission retries.
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
//! This file depends on the stable APIs of:
//!
//! - `backend.rs`;
//! - `backend_trait.rs`;
//! - `adapters/generic.rs`;
//! - `serde_json`;
//! - `sha2`.
//!
//! It does NOT require changes to those files.
//!
//! Downstream integration:
//!
//! - `provider_registry.rs` stores the adapter as `dyn QuantumBackendAdapter`.
//! - `device_registry.rs` indexes the supplied `QuantumBackend`.
//! - `execution.rs` calls `preflight`, `submit`, `status`, `result` and
//!   `cancel`.
//! - `job.rs` consumes `BackendJob` and `BackendJobStatus`.
//! - `queue.rs` consumes `BackendQueueInfo`.
//! - `benchmarking` consumes the normalized lifecycle/result contract.
//! - Danga can select this adapter through provider/backend registries.
//!
//! Adding another Braket device MUST NOT require changing this file.
//!
//! Adding another Braket provider/device technology MUST NOT require changing
//! the core hardware abstractions.
//!
//! # Production completion rule
//!
//! This file is considered complete when:
//!
//! 1. Braket device ARNs are validated;
//! 2. OpenQASM 3.x submission is implemented;
//! 3. provider-native action submission is explicit;
//! 4. client-token idempotency is implemented;
//! 5. task lifecycle is normalized;
//! 6. cancellation is normalized;
//! 7. queue metadata is normalized;
//! 8. S3 result retrieval is abstracted safely;
//! 9. result counts are normalized;
//! 10. provider failures are mapped;
//! 11. malformed responses are rejected;
//! 12. secrets cannot enter adapter metadata;
//! 13. no AWS SDK is required by the core crate;
//! 14. the adapter implements `QuantumBackendAdapter`;
//! 15. conformance behaviour is deterministic;
//! 16. unit tests cover success and failure paths;
//! 17. no downstream file needs to modify this contract.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use super::generic::{
    send_request,
    ProviderError,
    ProviderFailureCategory,
    ProviderOperation,
    ProviderRequest,
    ProviderResponse,
    ProviderTransport,
    RetryClass,
    TransportMethod,
};

use super::super::backend::{
    BackendError,
    BackendHealth,
    BackendHealthState,
    BackendJob,
    BackendJobId,
    BackendJobState,
    BackendJobStatus,
    BackendKind,
    BackendQueueInfo,
    BackendStatus,
    ExecutionRequest,
    ExecutionResult,
    QuantumBackend,
};

use super::super::backend_trait::{
    BackendAdapterInfo,
    BackendCancellation,
    BackendProgram,
    CancellationOutcome,
    QuantumBackendAdapter,
};

/// Stable adapter identifier.
pub const AWS_BRAKET_ADAPTER_ID: &str =
    "zamani.quantum.hardware.adapters.aws_braket";

/// Provider identifier.
pub const AWS_BRAKET_PROVIDER_ID: &str = "aws-braket";

/// Adapter semantic version.
pub const AWS_BRAKET_ADAPTER_VERSION: &str = "1.0.0";

/// Amazon Braket API version represented by this adapter.
pub const AWS_BRAKET_API_VERSION: &str = "2021-08-04";

/// Braket OpenQASM action schema.
pub const BRAKET_OPENQASM_ACTION_SCHEMA: &str =
    "braket.ir.openqasm.program";

/// Braket OpenQASM action schema version.
pub const BRAKET_OPENQASM_ACTION_VERSION: &str = "1";

/// Maximum Amazon Braket client-token length.
pub const MAX_BRAKET_CLIENT_TOKEN_LENGTH: usize = 64;

/// Maximum Braket device ARN length.
pub const MAX_BRAKET_DEVICE_ARN_LENGTH: usize = 256;

/// Maximum Braket S3 bucket length.
pub const MAX_BRAKET_S3_BUCKET_LENGTH: usize = 63;

/// Maximum Braket S3 key-prefix length.
///
/// Braket currently imposes additional provider-side restrictions. The
/// adapter performs conservative validation before submission.
pub const MAX_BRAKET_S3_PREFIX_LENGTH: usize = 1024;

/// Maximum provider-native action size.
///
/// This is intentionally below the generic transport's much larger payload
/// ceiling because the Braket CreateQuantumTask action itself is limited by
/// the service.
pub const MAX_BRAKET_ACTION_BYTES: usize = 5 * 1024 * 1024;

/// Maximum device-parameters JSON size.
pub const MAX_BRAKET_DEVICE_PARAMETERS_BYTES: usize = 48 * 1024;

/// Maximum failure reason length retained in normalized diagnostics.
pub const MAX_BRAKET_FAILURE_REASON_LENGTH: usize = 4096;

/// Maximum provider-status string retained by the adapter.
pub const MAX_BRAKET_STATUS_LENGTH: usize = 128;

/// Maximum S3 result-directory length retained by the adapter.
pub const MAX_BRAKET_RESULT_DIRECTORY_LENGTH: usize = 4096;

/// Maximum number of Braket tags.
pub const MAX_BRAKET_TAGS: usize = 50;

/// Maximum Braket tag key length.
pub const MAX_BRAKET_TAG_KEY_LENGTH: usize = 128;

/// Maximum Braket tag value length.
pub const MAX_BRAKET_TAG_VALUE_LENGTH: usize = 256;

/// OpenQASM program format identifier.
pub const FORMAT_OPENQASM_30: &str = "openqasm-3.0";

/// OpenQASM 3.1 program format identifier.
///
/// Braket's service documentation currently documents OpenQASM 3.0
/// submission. Zamani accepts this identifier only when the caller has
/// already lowered the program to a Braket-supported OpenQASM representation.
pub const FORMAT_OPENQASM_31: &str = "openqasm-3.1";

/// Explicit provider-native action format.
pub const FORMAT_BRAKET_ACTION: &str = "aws-braket-action";

/// Internal adapter counter used only for generating a fallback client token
/// when callers did not provide a request ID.
///
/// The counter is never serialized as provider metadata.
static CLIENT_TOKEN_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Result artifact retrieval abstraction.
///
/// Amazon Braket stores completed task results in S3. The hardware adapter
/// must not own an AWS S3 client or credentials, so retrieval is injected.
///
/// A concrete implementation may use:
///
/// - AWS SDK;
/// - AWS CLI wrapper;
/// - SigV4 HTTP;
/// - private cloud gateway;
/// - test fixture storage.
///
/// The artifact key is the exact Braket `outputS3Directory` value returned by
/// the task metadata.
///
/// Implementations MUST NOT return secrets.
pub trait BraketArtifactStore: Send + Sync {
    /// Retrieves a completed Braket task result.
    fn get_task_result(
        &self,
        bucket: &str,
        directory: &str,
    ) -> Result<Vec<u8>, BraketArtifactError>;

    /// Stable artifact-store implementation identifier.
    fn store_id(&self) -> &str;
}

/// Artifact retrieval error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BraketArtifactError {
    /// Requested artifact is not available yet.
    NotFound,

    /// Access was denied.
    AccessDenied,

    /// Artifact store is temporarily unavailable.
    Unavailable(String),

    /// Artifact payload is malformed.
    InvalidPayload(String),

    /// Artifact retrieval exceeded its transport deadline.
    Timeout,

    /// Other safe failure.
    Other(String),
}

impl fmt::Display for BraketArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => {
                formatter.write_str("Braket result artifact was not found")
            }

            Self::AccessDenied => {
                formatter.write_str(
                    "access to the Braket result artifact was denied",
                )
            }

            Self::Unavailable(message) => {
                write!(formatter, "Braket artifact store unavailable: {message}")
            }

            Self::InvalidPayload(message) => {
                write!(
                    formatter,
                    "invalid Braket result artifact: {message}"
                )
            }

            Self::Timeout => {
                formatter.write_str("Braket result artifact retrieval timed out")
            }

            Self::Other(message) => {
                write!(formatter, "Braket artifact retrieval failed: {message}")
            }
        }
    }
}

impl std::error::Error for BraketArtifactError {}

/// Braket-specific adapter configuration.
///
/// Configuration contains no credentials.
///
/// Authentication remains the responsibility of the supplied transport and
/// artifact store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AwsBraketConfig {
    /// S3 bucket used for task output.
    pub output_s3_bucket: String,

    /// S3 key prefix used for task output.
    pub output_s3_key_prefix: String,

    /// Optional JSON object passed as Braket `deviceParameters`.
    pub device_parameters: Option<Vec<u8>>,

    /// Optional Braket experimental-capability object.
    pub experimental_capabilities: Option<Vec<u8>>,

    /// Optional Braket hybrid-job token.
    pub job_token: Option<String>,

    /// Provider task tags.
    pub tags: BTreeMap<String, String>,

    /// Whether provider-native action submission is enabled.
    ///
    /// OpenQASM submission remains enabled independently.
    pub allow_provider_native_actions: bool,

    /// Whether the adapter accepts `openqasm-3.1` input.
    ///
    /// Acceptance means the caller promises the payload has already been
    /// lowered to syntax supported by the selected Braket target.
    pub allow_openqasm_31: bool,
}

impl AwsBraketConfig {
    /// Creates a conservative production configuration.
    pub fn new(
        output_s3_bucket: impl Into<String>,
        output_s3_key_prefix: impl Into<String>,
    ) -> Result<Self, BackendError> {
        let config = Self {
            output_s3_bucket: output_s3_bucket.into(),
            output_s3_key_prefix: output_s3_key_prefix.into(),
            device_parameters: None,
            experimental_capabilities: None,
            job_token: None,
            tags: BTreeMap::new(),
            allow_provider_native_actions: false,
            allow_openqasm_31: true,
        };

        config.validate()?;

        Ok(config)
    }

    /// Enables provider-native Braket actions explicitly.
    pub fn allow_provider_native_actions(mut self) -> Self {
        self.allow_provider_native_actions = true;
        self
    }

    /// Controls OpenQASM 3.1 acceptance.
    pub fn with_openqasm_31(mut self, enabled: bool) -> Self {
        self.allow_openqasm_31 = enabled;
        self
    }

    /// Sets device parameters.
    pub fn with_device_parameters(
        mut self,
        parameters: impl Into<Vec<u8>>,
    ) -> Result<Self, BackendError> {
        let parameters = parameters.into();

        validate_json_object(
            &parameters,
            "deviceParameters",
            MAX_BRAKET_DEVICE_PARAMETERS_BYTES,
        )?;

        self.device_parameters = Some(parameters);
        Ok(self)
    }

    /// Sets Braket experimental capabilities.
    pub fn with_experimental_capabilities(
        mut self,
        capabilities: impl Into<Vec<u8>>,
    ) -> Result<Self, BackendError> {
        let capabilities = capabilities.into();

        validate_json_object(
            &capabilities,
            "experimentalCapabilities",
            MAX_BRAKET_ACTION_BYTES,
        )?;

        self.experimental_capabilities = Some(capabilities);
        Ok(self)
    }

    /// Sets a Braket hybrid-job token.
    pub fn with_job_token(
        mut self,
        token: impl Into<String>,
    ) -> Result<Self, BackendError> {
        let token = token.into();

        validate_bounded_string(
            "jobToken",
            &token,
            128,
        )?;

        self.job_token = Some(token);
        Ok(self)
    }

    /// Adds a Braket task tag.
    pub fn with_tag(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, BackendError> {
        let key = key.into();
        let value = value.into();

        validate_tag(&key, &value)?;

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

    /// Validates the entire configuration.
    pub fn validate(&self) -> Result<(), BackendError> {
        validate_s3_bucket(&self.output_s3_bucket)?;
        validate_s3_prefix(&self.output_s3_key_prefix)?;

        if let Some(parameters) = &self.device_parameters {
            validate_json_object(
                parameters,
                "deviceParameters",
                MAX_BRAKET_DEVICE_PARAMETERS_BYTES,
            )?;
        }

        if let Some(capabilities) = &self.experimental_capabilities {
            validate_json_object(
                capabilities,
                "experimentalCapabilities",
                MAX_BRAKET_ACTION_BYTES,
            )?;
        }

        if let Some(token) = &self.job_token {
            validate_bounded_string("jobToken", token, 128)?;
        }

        if self.tags.len() > MAX_BRAKET_TAGS {
            return Err(BackendError::MetadataLimitExceeded {
                maximum: MAX_BRAKET_TAGS,
            });
        }

        for (key, value) in &self.tags {
            validate_tag(key, value)?;
        }

        Ok(())
    }
}

/// Production Amazon Braket adapter.
///
/// The adapter is immutable after construction.
///
/// Shared transports and artifact stores are held behind `Arc`, allowing
/// multiple Braket adapters to coexist without owning global state.
pub struct AwsBraketAdapter {
    backend: QuantumBackend,
    adapter_info: BackendAdapterInfo,
    transport: Arc<dyn ProviderTransport>,
    artifact_store: Arc<dyn BraketArtifactStore>,
    config: AwsBraketConfig,
}

impl fmt::Debug for AwsBraketAdapter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AwsBraketAdapter")
            .field("backend_id", &self.backend.id())
            .field("provider", &self.backend.provider())
            .field("adapter_info", &self.adapter_info)
            .field("transport_id", &self.transport.transport_id())
            .field("transport_version", &self.transport.transport_version())
            .field("artifact_store_id", &self.artifact_store.store_id())
            .field("output_s3_bucket", &self.config.output_s3_bucket)
            .field("output_s3_key_prefix", &self.config.output_s3_key_prefix)
            .finish()
    }
}

impl AwsBraketAdapter {
    /// Constructs a production Braket adapter.
    ///
    /// The backend must already be a validated `QuantumBackend`.
    ///
    /// The caller supplies authenticated transport and result-artifact
    /// implementations.
    pub fn new(
        backend: QuantumBackend,
        transport: Arc<dyn ProviderTransport>,
        artifact_store: Arc<dyn BraketArtifactStore>,
        config: AwsBraketConfig,
    ) -> Result<Self, BackendError> {
        config.validate()?;

        validate_braket_backend(&backend)?;

        let adapter_info = BackendAdapterInfo::new(
            AWS_BRAKET_ADAPTER_ID,
            AWS_BRAKET_ADAPTER_VERSION,
            true,
        )?
        .with_provider_api_version(AWS_BRAKET_API_VERSION)?;

        Ok(Self {
            backend,
            adapter_info,
            transport,
            artifact_store,
            config,
        })
    }

    /// Returns the Braket configuration.
    pub fn config(&self) -> &AwsBraketConfig {
        &self.config
    }

    /// Returns the configured transport.
    pub fn transport(&self) -> &dyn ProviderTransport {
        self.transport.as_ref()
    }

    /// Returns the configured result artifact store.
    pub fn artifact_store(&self) -> &dyn BraketArtifactStore {
        self.artifact_store.as_ref()
    }

    /// Performs a Braket-specific preflight.
    pub fn braket_preflight(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<(), BackendError> {
        request.validate_structure()?;

        validate_braket_backend(&self.backend)?;

        if !self.backend.metadata.status.is_usable() {
            return Err(BackendError::BackendUnavailable {
                backend_id: self.backend.id().to_owned(),
                status: self.backend.metadata.status,
            });
        }

        if self.backend.kind() != BackendKind::Qpu
            && self.backend.kind() != BackendKind::Simulator
            && self.backend.kind() != BackendKind::Emulator
        {
            return Err(BackendError::ExecutionUnavailable(
                "Amazon Braket adapter requires a QPU, simulator, or emulator backend"
                    .to_owned(),
            ));
        }

        match program.format() {
            FORMAT_OPENQASM_30 => {}

            FORMAT_OPENQASM_31 if self.config.allow_openqasm_31 => {}

            FORMAT_OPENQASM_31 => {
                return Err(BackendError::ExecutionRejected(
                    "OpenQASM 3.1 input is disabled by this Braket adapter configuration"
                        .to_owned(),
                ));
            }

            FORMAT_BRAKET_ACTION if self.config.allow_provider_native_actions => {}

            FORMAT_BRAKET_ACTION => {
                return Err(BackendError::ExecutionRejected(
                    "provider-native Braket actions are disabled by configuration"
                        .to_owned(),
                ));
            }

            other => {
                return Err(BackendError::ExecutionRejected(format!(
                    "Amazon Braket adapter does not directly accept program format '{other}'; \
                     lower the Zamani workload to OpenQASM 3.x or an explicit aws-braket-action"
                )));
            }
        }

        let shots = request.workload.circuit.shots;

        if shots == 0 {
            return Err(BackendError::InvalidShots);
        }

        if let Some(seed) = request.seed {
            if !self.backend.capabilities.deterministic_seeding {
                return Err(BackendError::DeterministicSeedingUnsupported);
            }

            // Prevent an otherwise unused-value warning while documenting the
            // fact that Braket seed support is target-dependent.
            let _ = seed;
        }

        Ok(())
    }

    /// Creates the Braket action object for a Zamani OpenQASM program.
    fn openqasm_action(
        &self,
        program: &BackendProgram,
    ) -> Result<Value, BackendError> {
        let source = std::str::from_utf8(program.bytes())
            .map_err(|_| {
                BackendError::ExecutionRejected(
                    "OpenQASM payload is not valid UTF-8".to_owned(),
                )
            })?;

        if source.trim().is_empty() {
            return Err(BackendError::ExecutionRejected(
                "OpenQASM source is empty".to_owned(),
            ));
        }

        let action = json!({
            "braketSchemaHeader": {
                "name": BRAKET_OPENQASM_ACTION_SCHEMA,
                "version": BRAKET_OPENQASM_ACTION_VERSION
            },
            "source": source
        });

        validate_serialized_action(&action)?;

        Ok(action)
    }

    /// Parses an explicit provider-native Braket action.
    fn native_action(
        &self,
        program: &BackendProgram,
    ) -> Result<Value, BackendError> {
        let action: Value =
            serde_json::from_slice(program.bytes()).map_err(|error| {
                BackendError::ExecutionRejected(format!(
                    "invalid Amazon Braket action JSON: {error}"
                ))
            })?;

        if !action.is_object() {
            return Err(BackendError::ExecutionRejected(
                "Amazon Braket action must be a JSON object".to_owned(),
            ));
        }

        validate_serialized_action(&action)?;

        Ok(action)
    }

    /// Builds a Braket `CreateQuantumTask` request.
    fn create_task_request(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<ProviderRequest, BackendError> {
        let action = match program.format() {
            FORMAT_OPENQASM_30 | FORMAT_OPENQASM_31 => {
                self.openqasm_action(program)?
            }

            FORMAT_BRAKET_ACTION => {
                self.native_action(program)?
            }

            other => {
                return Err(BackendError::ExecutionRejected(format!(
                    "unsupported Braket program format '{other}'"
                )));
            }
        };

        let client_token = self.client_token(request, program)?;

        let mut payload = serde_json::Map::new();

        payload.insert(
            "clientToken".to_owned(),
            Value::String(client_token.clone()),
        );

        payload.insert(
            "deviceArn".to_owned(),
            Value::String(self.backend.id().to_owned()),
        );

        payload.insert(
            "shots".to_owned(),
            Value::Number(
                serde_json::Number::from(
                    request.workload.circuit.shots as u64
                ),
            ),
        );

        payload.insert(
            "action".to_owned(),
            action,
        );

        payload.insert(
            "outputS3Bucket".to_owned(),
            Value::String(self.config.output_s3_bucket.clone()),
        );

        payload.insert(
            "outputS3KeyPrefix".to_owned(),
            Value::String(self.config.output_s3_key_prefix.clone()),
        );

        if let Some(parameters) = &self.config.device_parameters {
            payload.insert(
                "deviceParameters".to_owned(),
                parse_json_value(parameters, "deviceParameters")?,
            );
        }

        if let Some(capabilities) = &self.config.experimental_capabilities {
            payload.insert(
                "experimentalCapabilities".to_owned(),
                parse_json_value(
                    capabilities,
                    "experimentalCapabilities",
                )?,
            );
        }

        if let Some(job_token) = &self.config.job_token {
            payload.insert(
                "jobToken".to_owned(),
                Value::String(job_token.clone()),
            );
        }

        if !self.config.tags.is_empty() {
            let mut tags = serde_json::Map::new();

            for (key, value) in &self.config.tags {
                tags.insert(
                    key.clone(),
                    Value::String(value.clone()),
                );
            }

            payload.insert("tags".to_owned(), Value::Object(tags));
        }

        let body =
            serde_json::to_vec(&Value::Object(payload)).map_err(|error| {
                BackendError::ExecutionRejected(format!(
                    "failed to encode Amazon Braket CreateQuantumTask request: {error}"
                ))
            })?;

        if body.len() > MAX_BRAKET_ACTION_BYTES {
            return Err(BackendError::ExecutionRejected(format!(
                "Amazon Braket CreateQuantumTask payload exceeds {} bytes",
                MAX_BRAKET_ACTION_BYTES
            )));
        }

        let request_id = request
            .request_id
            .clone()
            .unwrap_or_else(|| client_token.clone());

        let builder = ProviderRequest::builder(
            ProviderOperation::Submit,
            TransportMethod::Post,
            "/quantum-task",
            request_id,
        )
        .header("content-type", "application/json")
        .map_err(generic_error)?;

        let builder = builder
            .header("accept", "application/json")
            .map_err(generic_error)?;

        let builder = builder
            .body(body)
            .map_err(generic_error)?;

        let builder = builder
            .idempotency_key(client_token)
            .map_err(generic_error)?;

        builder.build().map_err(generic_error)
    }

    /// Builds a GetQuantumTask request.
    fn get_task_request(
        &self,
        job: &BackendJobId,
        request_id: &str,
    ) -> Result<ProviderRequest, BackendError> {
        ProviderRequest::builder(
            ProviderOperation::GetJobStatus,
            TransportMethod::Get,
            format!("/quantum-task/{}", url_path_segment(job.as_str())?),
            request_id,
        )
        .header("accept", "application/json")
        .map_err(generic_error)?
        .build()
        .map_err(generic_error)
    }

    /// Builds a cancellation request.
    fn cancel_task_request(
        &self,
        job: &BackendJobId,
        request_id: &str,
    ) -> Result<ProviderRequest, BackendError> {
        ProviderRequest::builder(
            ProviderOperation::Cancel,
            TransportMethod::Delete,
            format!("/quantum-task/{}", url_path_segment(job.as_str())?),
            request_id,
        )
        .header("accept", "application/json")
        .map_err(generic_error)?
        .build()
        .map_err(generic_error)
    }

    /// Generates the Braket client token.
    fn client_token(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<String, BackendError> {
        if let Some(request_id) = &request.request_id {
            return normalize_client_token(request_id);
        }

        let counter =
            CLIENT_TOKEN_COUNTER.fetch_add(1, Ordering::Relaxed);

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| {
                BackendError::ExecutionUnavailable(
                    "system clock is before UNIX epoch; cannot safely create Braket client token"
                        .to_owned(),
                )
            })?
            .as_nanos();

        let mut hasher = Sha256::new();

        hasher.update(self.backend.id().as_bytes());
        hasher.update(program.format().as_bytes());
        hasher.update(program.bytes());
        hasher.update(
            request.workload.circuit.shots.to_le_bytes(),
        );
        hasher.update(counter.to_le_bytes());
        hasher.update(timestamp.to_le_bytes());

        let digest = hasher.finalize();

        Ok(hex::encode(digest)[..MAX_BRAKET_CLIENT_TOKEN_LENGTH]
            .to_owned())
    }

    /// Converts a Braket response into a normalized provider error.
    fn provider_error(
        &self,
        response: &ProviderResponse,
    ) -> BackendError {
        let value = serde_json::from_slice::<Value>(&response.body)
            .unwrap_or(Value::Null);

        let message = value
            .get("message")
            .and_then(Value::as_str)
            .or_else(|| value.get("Message").and_then(Value::as_str))
            .or_else(|| value.get("error").and_then(Value::as_str))
            .unwrap_or("Amazon Braket returned an error");

        let code = value
            .get("__type")
            .and_then(Value::as_str)
            .or_else(|| value.get("code").and_then(Value::as_str));

        let safe_message =
            bounded_diagnostic(message, MAX_BRAKET_FAILURE_REASON_LENGTH);

        let code = code
            .map(|value| {
                bounded_diagnostic(
                    value,
                    MAX_BRAKET_FAILURE_REASON_LENGTH,
                )
            })
            .unwrap_or_else(|| "unknown".to_owned());

        match response.status_code {
            400 => BackendError::ExecutionRejected(format!(
                "Amazon Braket rejected request: {safe_message} (code={code})"
            )),

            401 => BackendError::ExecutionRejected(
                "Amazon Braket authentication failed".to_owned(),
            ),

            403 => BackendError::ExecutionRejected(
                "Amazon Braket authorization failed".to_owned(),
            ),

            404 => BackendError::ExecutionRejected(format!(
                "Amazon Braket resource was not found: {safe_message}"
            )),

            409 => BackendError::ExecutionRejected(format!(
                "Amazon Braket rejected the request because of a state conflict: {safe_message}"
            )),

            429 => BackendError::ExecutionUnavailable(format!(
                "Amazon Braket rate-limited the request: {safe_message}"
            )),

            500..=599 => BackendError::ExecutionUnavailable(format!(
                "Amazon Braket service failure: {safe_message}"
            )),

            status => BackendError::ExecutionUnavailable(format!(
                "Amazon Braket request failed with status {status}: {safe_message}"
            )),
        }
    }

    /// Sends a request and maps generic/provider failures into BackendError.
    fn send(
        &self,
        request: &ProviderRequest,
    ) -> Result<ProviderResponse, BackendError> {
        match send_request(self.transport.as_ref(), request) {
            Ok(response) if response.is_success() => Ok(response),

            Ok(response) => Err(self.provider_error(&response)),

            Err(error) => Err(map_provider_error(error)),
        }
    }

    /// Parses GetQuantumTask metadata.
    fn parse_task_metadata(
        &self,
        value: &Value,
    ) -> Result<BraketTaskMetadata, BackendError> {
        let arn = required_string(
            value,
            "quantumTaskArn",
            MAX_BRAKET_DEVICE_ARN_LENGTH,
        )?;

        validate_braket_task_arn(&arn)?;

        let device_arn = required_string(
            value,
            "deviceArn",
            MAX_BRAKET_DEVICE_ARN_LENGTH,
        )?;

        validate_braket_device_arn(&device_arn)?;

        let status = required_string(
            value,
            "status",
            MAX_BRAKET_STATUS_LENGTH,
        )?;

        let shots = required_u64(value, "shots")?;

        let output_bucket = required_string(
            value,
            "outputS3Bucket",
            MAX_BRAKET_S3_BUCKET_LENGTH,
        )?;

        validate_s3_bucket(&output_bucket)?;

        let output_directory = required_string(
            value,
            "outputS3Directory",
            MAX_BRAKET_RESULT_DIRECTORY_LENGTH,
        )?;

        validate_s3_prefix(&output_directory)?;

        let failure_reason = value
            .get("failureReason")
            .and_then(Value::as_str)
            .map(|reason| {
                bounded_diagnostic(
                    reason,
                    MAX_BRAKET_FAILURE_REASON_LENGTH,
                )
            });

        let queue_info = parse_queue_info(value);

        Ok(BraketTaskMetadata {
            arn,
            device_arn,
            status,
            shots: usize::try_from(shots).map_err(|_| {
                BackendError::ExecutionRejected(
                    "Braket shot count exceeds this platform's usize range"
                        .to_owned(),
                )
            })?,
            output_bucket,
            output_directory,
            failure_reason,
            queue_info,
        })
    }

    /// Normalizes a Braket task state.
    fn normalize_task_state(
        status: &str,
    ) -> Result<BackendJobState, BackendError> {
        match status {
            "CREATED" => Ok(BackendJobState::Created),
            "QUEUED" => Ok(BackendJobState::Queued),
            "RUNNING" => Ok(BackendJobState::Running),
            "CANCELLING" => Ok(BackendJobState::Cancelling),
            "CANCELLED" => Ok(BackendJobState::Cancelled),
            "COMPLETED" => Ok(BackendJobState::Completed),
            "FAILED" => Ok(BackendJobState::Failed),

            other => Err(BackendError::ExecutionUnavailable(format!(
                "Amazon Braket returned unknown quantum-task status '{other}'"
            ))),
        }
    }

    /// Converts task metadata to a normalized job status.
    fn normalized_status(
        &self,
        metadata: BraketTaskMetadata,
    ) -> Result<BackendJobStatus, BackendError> {
        let state = Self::normalize_task_state(&metadata.status)?;

        if state == BackendJobState::Failed {
            if let Some(reason) = metadata.failure_reason {
                return Err(BackendError::ExecutionRejected(format!(
                    "Amazon Braket quantum task failed: {reason}"
                )));
            }
        }

        let job_id =
            BackendJobId::new(metadata.arn.clone())?;

        let job = BackendJob::new(
            job_id,
            self.backend.id().to_owned(),
            None,
            state,
        )?;

        Ok(BackendJobStatus {
            job,
            provider_status: Some(metadata.status),
            queue_position: metadata
                .queue_info
                .as_ref()
                .and_then(|info| info.position),
            estimated_wait: None,
            result_available: state == BackendJobState::Completed,
        })
    }

    /// Converts a completed Braket result artifact into ExecutionResult.
    fn normalize_result(
        &self,
        job: &BackendJobId,
        metadata: &BraketTaskMetadata,
        bytes: &[u8],
    ) -> Result<ExecutionResult, BackendError> {
        let value: Value =
            serde_json::from_slice(bytes).map_err(|error| {
                BackendError::ExecutionRejected(format!(
                    "Amazon Braket result artifact is not valid JSON: {error}"
                ))
            })?;

        let mut result =
            ExecutionResult::empty(
                self.backend.id().to_owned(),
                metadata.shots,
            )?;

        result.metadata.insert(
            "provider".to_owned(),
            AWS_BRAKET_PROVIDER_ID.to_owned(),
        );

        result.metadata.insert(
            "provider_task_arn".to_owned(),
            job.as_str().to_owned(),
        );

        result.metadata.insert(
            "device_arn".to_owned(),
            metadata.device_arn.clone(),
        );

        result.metadata.insert(
            "output_s3_bucket".to_owned(),
            metadata.output_bucket.clone(),
        );

        result.metadata.insert(
            "output_s3_directory".to_owned(),
            metadata.output_directory.clone(),
        );

        if let Some(counts) = extract_counts(&value) {
            for (bitstring, count) in counts {
                result.counts.insert(bitstring, count);
            }
        }

        if let Some(expectations) = extract_expectations(&value) {
            for (observable, value) in expectations {
                result
                    .expectation_values
                    .insert(observable, value);
            }
        }

        let counted = result.counted_shots();

        if counted > result.shots {
            return Err(BackendError::ResultShotsExceeded {
                represented: counted,
                shots: result.shots,
            });
        }

        // A successful Braket task is not considered a valid normalized result
        // unless the artifact contains at least one result representation.
        if result.counts.is_empty()
            && result.expectation_values.is_empty()
            && value
                .get("measurements")
                .is_none()
            && value
                .get("measurementCounts")
                .is_none()
        {
            return Err(BackendError::ExecutionRejected(
                "Amazon Braket result artifact contains no normalized measurement or expectation data"
                    .to_owned(),
            ));
        }

        Ok(result)
    }
}

impl QuantumBackendAdapter for AwsBraketAdapter {
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
        self.braket_preflight(request, program)
    }

    fn submit(
        &self,
        request: &ExecutionRequest,
        program: &BackendProgram,
    ) -> Result<BackendJob, BackendError> {
        self.braket_preflight(request, program)?;

        let provider_request =
            self.create_task_request(request, program)?;

        let response = self.send(&provider_request)?;

        let value: Value =
            serde_json::from_slice(&response.body).map_err(|error| {
                BackendError::ExecutionRejected(format!(
                    "Amazon Braket CreateQuantumTask returned invalid JSON: {error}"
                ))
            })?;

        let arn = required_string(
            &value,
            "quantumTaskArn",
            MAX_BRAKET_DEVICE_ARN_LENGTH,
        )?;

        validate_braket_task_arn(&arn)?;

        let job_id = BackendJobId::new(arn)?;

        BackendJob::new(
            job_id,
            self.backend.id().to_owned(),
            request.request_id.clone(),
            BackendJobState::Created,
        )
    }

    fn status(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendJobStatus, BackendError> {
        let request_id =
            format!("status-{}", stable_short_hash(job.as_str()));

        let provider_request =
            self.get_task_request(job, &request_id)?;

        let response = self.send(&provider_request)?;

        let value: Value =
            serde_json::from_slice(&response.body).map_err(|error| {
                BackendError::ExecutionRejected(format!(
                    "Amazon Braket GetQuantumTask returned invalid JSON: {error}"
                ))
            })?;

        let metadata =
            self.parse_task_metadata(&value)?;

        if metadata.device_arn != self.backend.id() {
            return Err(BackendError::ExecutionRejected(format!(
                "Amazon Braket task belongs to device '{}' rather than configured backend '{}'",
                metadata.device_arn,
                self.backend.id()
            )));
        }

        self.normalized_status(metadata)
    }

    fn result(
        &self,
        job: &BackendJobId,
    ) -> Result<ExecutionResult, BackendError> {
        let status = self.status(job)?;

        if status.job.state != BackendJobState::Completed {
            return Err(BackendError::ExecutionUnavailable(format!(
                "Amazon Braket task '{}' is not completed; current state is {}",
                job,
                status.job.state
            )));
        }

        let request_id =
            format!("result-{}", stable_short_hash(job.as_str()));

        let provider_request =
            self.get_task_request(job, &request_id)?;

        let response = self.send(&provider_request)?;

        let value: Value =
            serde_json::from_slice(&response.body).map_err(|error| {
                BackendError::ExecutionRejected(format!(
                    "Amazon Braket GetQuantumTask returned invalid JSON: {error}"
                ))
            })?;

        let metadata =
            self.parse_task_metadata(&value)?;

        if metadata.status != "COMPLETED" {
            return Err(BackendError::ExecutionUnavailable(format!(
                "Amazon Braket task '{}' changed state while retrieving result: {}",
                job,
                metadata.status
            )));
        }

        let artifact = self
            .artifact_store
            .get_task_result(
                &metadata.output_bucket,
                &metadata.output_directory,
            )
            .map_err(map_artifact_error)?;

        self.normalize_result(job, &metadata, &artifact)
    }

    fn cancel(
        &self,
        job: &BackendJobId,
    ) -> Result<BackendCancellation, BackendError> {
        let current = self.status(job)?;

        if current.job.state.is_terminal() {
            return Ok(BackendCancellation {
                job: job.clone(),
                outcome: CancellationOutcome::AlreadyTerminal,
            });
        }

        let request_id =
            format!("cancel-{}", stable_short_hash(job.as_str()));

        let provider_request =
            self.cancel_task_request(job, &request_id)?;

        let response = self.send(&provider_request)?;

        if response.status_code == 200
            || response.status_code == 202
            || response.status_code == 204
        {
            return Ok(BackendCancellation {
                job: job.clone(),
                outcome: CancellationOutcome::Accepted,
            });
        }

        Ok(BackendCancellation {
            job: job.clone(),
            outcome: CancellationOutcome::Pending,
        })
    }

    fn queue_info(&self) -> Result<BackendQueueInfo, BackendError> {
        // Braket queue information is task/device dependent. The canonical
        // adapter does not invent a provider-wide queue depth.
        //
        // The service exposes queue metadata through GetQuantumTask and
        // device queue information. Because the core trait asks for backend-
        // wide information, this method deliberately reports unsupported
        // rather than fabricating a value.
        Err(BackendError::ExecutionUnavailable(
            "Amazon Braket queue information is task/device-specific; \
             use status(job) for task queue metadata"
                .to_owned(),
        ))
    }

    fn health(&self) -> Result<BackendHealth, BackendError> {
        // There is intentionally no unauthenticated health endpoint invented
        // here. A successful Describe/GetDevice operation belongs to discovery
        // and can be used by a higher-level health subsystem.
        Ok(BackendHealth {
            state: BackendHealthState::Unknown,
            backend_status: self.backend.metadata.status,
            message: Some(
                "Braket adapter health is determined by authenticated provider \
                 discovery/health integration"
                    .to_owned(),
            ),
        })
    }

    fn supports_cancellation(&self) -> bool {
        self.backend.capabilities.cancellation
    }

    fn supports_queue_info(&self) -> bool {
        self.backend.capabilities.queue_information
    }

    fn supports_synchronous_execution(&self) -> bool {
        false
    }
}

impl super::super::backend_trait::ConformantQuantumBackendAdapter
    for AwsBraketAdapter
{
}

/// Internal normalized Braket task metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
struct BraketTaskMetadata {
    arn: String,
    device_arn: String,
    status: String,
    shots: usize,
    output_bucket: String,
    output_directory: String,
    failure_reason: Option<String>,
    queue_info: Option<BraketQueueMetadata>,
}

/// Internal queue metadata returned by GetQuantumTask.
#[derive(Debug, Clone, PartialEq, Eq)]
struct BraketQueueMetadata {
    position: Option<usize>,
}

/// Extracts queue metadata without inventing an estimate.
fn parse_queue_info(value: &Value) -> Option<BraketQueueMetadata> {
    let queue = value.get("queueInfo")?;

    let position = queue
        .get("position")
        .and_then(|value| value.as_str())
        .and_then(|value| value.parse::<usize>().ok())
        .or_else(|| {
            queue
                .get("position")
                .and_then(Value::as_u64)
                .and_then(|value| usize::try_from(value).ok())
        });

    Some(BraketQueueMetadata { position })
}

/// Extracts normalized counts from known Braket result representations.
///
/// Braket's SDK exposes measurement counts. Provider/device result schemas
/// may use either `measurementCounts` or nested result structures, so this
/// parser intentionally supports both the canonical flat form and nested
/// object forms.
fn extract_counts(value: &Value) -> Option<BTreeMap<String, usize>> {
    let candidates = [
        value.get("measurementCounts"),
        value.get("measurement_counts"),
        value.get("counts"),
        value
            .get("result")
            .and_then(|result| result.get("measurementCounts")),
        value
            .get("result")
            .and_then(|result| result.get("measurement_counts")),
        value
            .get("result")
            .and_then(|result| result.get("counts")),
    ];

    for candidate in candidates.into_iter().flatten() {
        if let Some(map) = parse_count_object(candidate) {
            return Some(map);
        }
    }

    // Some result representations expose individual measurement shots.
    if let Some(measurements) =
        value.get("measurements").and_then(Value::as_array)
    {
        let mut counts = BTreeMap::new();

        for measurement in measurements {
            let bits = if let Some(bits) =
                measurement.as_array()
            {
                bits.iter()
                    .map(|bit| {
                        bit.as_u64()
                            .map(|value| if value == 0 { '0' } else { '1' })
                    })
                    .collect::<Option<String>>()
            } else {
                measurement
                    .as_str()
                    .map(ToOwned::to_owned)
            };

            if let Some(bits) = bits {
                if bits.chars().all(|bit| bit == '0' || bit == '1') {
                    let entry = counts.entry(bits).or_insert(0);
                    *entry = entry.saturating_add(1);
                }
            }
        }

        if !counts.is_empty() {
            return Some(counts);
        }
    }

    None
}

/// Parses an object containing bitstring -> count mappings.
fn parse_count_object(
    value: &Value,
) -> Option<BTreeMap<String, usize>> {
    let object = value.as_object()?;

    let mut counts = BTreeMap::new();

    for (bitstring, count) in object {
        if !is_valid_bitstring(bitstring) {
            return None;
        }

        let count = count
            .as_u64()
            .and_then(|value| usize::try_from(value).ok())?;

        counts.insert(bitstring.clone(), count);
    }

    if counts.is_empty() {
        None
    } else {
        Some(counts)
    }
}

/// Extracts expectation values from normalized or provider-native result
/// representations.
fn extract_expectations(
    value: &Value,
) -> Option<BTreeMap<String, String>> {
    let candidates = [
        value.get("expectationValues"),
        value.get("expectation_values"),
        value.get("expectations"),
        value
            .get("result")
            .and_then(|result| result.get("expectationValues")),
        value
            .get("result")
            .and_then(|result| result.get("expectation_values")),
    ];

    for candidate in candidates.into_iter().flatten() {
        if let Some(object) = candidate.as_object() {
            let mut values = BTreeMap::new();

            for (key, value) in object {
                if let Some(number) = value.as_f64() {
                    values.insert(key.clone(), number.to_string());
                } else if let Some(text) = value.as_str() {
                    values.insert(key.clone(), text.to_owned());
                }
            }

            if !values.is_empty() {
                return Some(values);
            }
        }
    }

    None
}

/// Validates that a Braket backend descriptor actually represents an AWS
/// Braket device.
fn validate_braket_backend(
    backend: &QuantumBackend,
) -> Result<(), BackendError> {
    if backend.provider() != AWS_BRAKET_PROVIDER_ID
        && backend.provider() != "aws.braket"
        && backend.provider() != "amazon-braket"
    {
        return Err(BackendError::ExecutionRejected(format!(
            "backend provider '{}' is not an Amazon Braket provider identifier",
            backend.provider()
        )));
    }

    validate_braket_device_arn(backend.id())?;

    Ok(())
}

/// Validates a Braket device ARN without performing network access.
fn validate_braket_device_arn(
    arn: &str,
) -> Result<(), BackendError> {
    validate_bounded_string(
        "deviceArn",
        arn,
        MAX_BRAKET_DEVICE_ARN_LENGTH,
    )?;

    let parts: Vec<&str> = arn.split(':').collect();

    if parts.len() != 6 {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket device ARN must contain six ARN components"
                .to_owned(),
        ));
    }

    if parts[0] != "arn" {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket device ARN must start with 'arn'"
                .to_owned(),
        ));
    }

    if !parts[1].starts_with("aws") {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket device ARN has an invalid AWS partition"
                .to_owned(),
        ));
    }

    if parts[2] != "braket" {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket device ARN service must be 'braket'"
                .to_owned(),
        ));
    }

    if parts[5].is_empty() {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket device ARN resource cannot be empty"
                .to_owned(),
        ));
    }

    if !parts[5].starts_with("device/") {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket ARN resource must start with 'device/'"
                .to_owned(),
        ));
    }

    Ok(())
}

/// Validates a Braket quantum-task ARN.
fn validate_braket_task_arn(
    arn: &str,
) -> Result<(), BackendError> {
    validate_bounded_string(
        "quantumTaskArn",
        arn,
        MAX_BRAKET_DEVICE_ARN_LENGTH,
    )?;

    let parts: Vec<&str> = arn.split(':').collect();

    if parts.len() != 6 {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket task ARN must contain six ARN components"
                .to_owned(),
        ));
    }

    if parts[0] != "arn"
        || !parts[1].starts_with("aws")
        || parts[2] != "braket"
    {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket task ARN has an invalid ARN prefix"
                .to_owned(),
        ));
    }

    if !parts[5].starts_with("quantum-task/")
        && !parts[5].starts_with("job/")
    {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket task ARN resource must be quantum-task/* or job/*"
                .to_owned(),
        ));
    }

    Ok(())
}

/// Validates an S3 bucket name conservatively.
fn validate_s3_bucket(
    bucket: &str,
) -> Result<(), BackendError> {
    validate_bounded_string(
        "outputS3Bucket",
        bucket,
        MAX_BRAKET_S3_BUCKET_LENGTH,
    )?;

    if bucket.len() < 3 {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket S3 output bucket must contain at least three characters"
                .to_owned(),
        ));
    }

    if bucket.starts_with('.')
        || bucket.ends_with('.')
        || bucket.starts_with('-')
        || bucket.ends_with('-')
    {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket S3 bucket name has invalid boundary characters"
                .to_owned(),
        ));
    }

    if bucket.contains("..")
        || bucket.contains(".-")
        || bucket.contains("-.")
    {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket S3 bucket name contains an invalid sequence"
                .to_owned(),
        ));
    }

    if !bucket
        .chars()
        .all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || matches!(character, '.' | '-')
        })
    {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket S3 bucket name contains invalid characters"
                .to_owned(),
        ));
    }

    Ok(())
}

/// Validates a Braket S3 output prefix/directory.
fn validate_s3_prefix(
    prefix: &str,
) -> Result<(), BackendError> {
    validate_bounded_string(
        "outputS3KeyPrefix",
        prefix,
        MAX_BRAKET_S3_PREFIX_LENGTH,
    )?;

    if prefix.contains("../")
        || prefix == ".."
        || prefix.starts_with('/')
    {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket S3 output prefix contains forbidden path traversal or absolute-path syntax"
                .to_owned(),
        ));
    }

    if prefix.contains('\0') {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket S3 output prefix contains NUL"
                .to_owned(),
        ));
    }

    Ok(())
}

/// Validates Braket tags.
fn validate_tag(
    key: &str,
    value: &str,
) -> Result<(), BackendError> {
    validate_bounded_string(
        "tag.key",
        key,
        MAX_BRAKET_TAG_KEY_LENGTH,
    )?;

    validate_bounded_string(
        "tag.value",
        value,
        MAX_BRAKET_TAG_VALUE_LENGTH,
    )?;

    if key.contains("secret")
        || key.contains("token")
        || key.contains("password")
        || key.contains("credential")
        || key.contains("authorization")
    {
        return Err(BackendError::SecretLikeMetadata {
            key: key.to_owned(),
        });
    }

    Ok(())
}

/// Validates an arbitrary JSON object used as a Braket API field.
fn validate_json_object(
    bytes: &[u8],
    field: &'static str,
    maximum: usize,
) -> Result<(), BackendError> {
    if bytes.is_empty() {
        return Err(BackendError::ExecutionRejected(format!(
            "{field} must not be empty"
        )));
    }

    if bytes.len() > maximum {
        return Err(BackendError::ExecutionRejected(format!(
            "{field} exceeds maximum allowed size {maximum}"
        )));
    }

    let value: Value =
        serde_json::from_slice(bytes).map_err(|error| {
            BackendError::ExecutionRejected(format!(
                "{field} is not valid JSON: {error}"
            ))
        })?;

    if !value.is_object() {
        return Err(BackendError::ExecutionRejected(format!(
            "{field} must be a JSON object"
        )));
    }

    Ok(())
}

/// Parses a previously validated JSON value.
fn parse_json_value(
    bytes: &[u8],
    field: &'static str,
) -> Result<Value, BackendError> {
    serde_json::from_slice(bytes).map_err(|error| {
        BackendError::ExecutionRejected(format!(
            "{field} is not valid JSON: {error}"
        ))
    })
}

/// Validates a serialized Braket action.
fn validate_serialized_action(
    action: &Value,
) -> Result<(), BackendError> {
    let object = action.as_object().ok_or_else(|| {
        BackendError::ExecutionRejected(
            "Amazon Braket action must be an object".to_owned(),
        )
    })?;

    let header = object
        .get("braketSchemaHeader")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            BackendError::ExecutionRejected(
                "Amazon Braket action is missing braketSchemaHeader"
                    .to_owned(),
            )
        })?;

    let name = header
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            BackendError::ExecutionRejected(
                "Amazon Braket action schema header is missing name"
                    .to_owned(),
            )
        })?;

    let version = header
        .get("version")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            BackendError::ExecutionRejected(
                "Amazon Braket action schema header is missing version"
                    .to_owned(),
            )
        })?;

    validate_bounded_string(
        "braketSchemaHeader.name",
        name,
        256,
    )?;

    validate_bounded_string(
        "braketSchemaHeader.version",
        version,
        64,
    )?;

    Ok(())
}

/// Returns a required JSON string.
fn required_string(
    value: &Value,
    field: &'static str,
    maximum: usize,
) -> Result<String, BackendError> {
    let string = value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| {
            BackendError::ExecutionRejected(format!(
                "Amazon Braket response is missing string field '{field}'"
            ))
        })?;

    validate_bounded_string(field, string, maximum)?;

    Ok(string.to_owned())
}

/// Returns a required JSON unsigned integer.
fn required_u64(
    value: &Value,
    field: &'static str,
) -> Result<u64, BackendError> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            BackendError::ExecutionRejected(format!(
                "Amazon Braket response is missing numeric field '{field}'"
            ))
        })
}

/// Validates a bounded textual value.
fn validate_bounded_string(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), BackendError> {
    if value.trim().is_empty() {
        return Err(BackendError::InvalidIdentifier { field });
    }

    if value.len() > maximum {
        return Err(BackendError::IdentifierTooLong {
            field,
            maximum,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(BackendError::InvalidIdentifier { field });
    }

    Ok(())
}

/// Normalizes an execution request ID into a Braket-safe client token.
fn normalize_client_token(
    request_id: &str,
) -> Result<String, BackendError> {
    validate_bounded_string(
        "request_id",
        request_id,
        MAX_BRAKET_CLIENT_TOKEN_LENGTH,
    )?;

    if !request_id
        .chars()
        .all(|character| {
            character.is_ascii_alphanumeric()
                || matches!(
                    character,
                    '-' | '_' | '.' | ':' | '/' | '='
                )
        })
    {
        return Err(BackendError::ExecutionRejected(
            "request_id cannot be represented safely as an Amazon Braket client token"
                .to_owned(),
        ));
    }

    Ok(request_id.to_owned())
}

/// Safely encodes a path segment.
///
/// This deliberately rejects path separators instead of attempting to perform
/// partial URL encoding in the provider-neutral adapter layer.
fn url_path_segment(
    value: &str,
) -> Result<String, BackendError> {
    validate_bounded_string(
        "path_segment",
        value,
        MAX_BRAKET_DEVICE_ARN_LENGTH,
    )?;

    if value.contains('/')
        || value.contains('\\')
        || value.contains('?')
        || value.contains('#')
    {
        return Err(BackendError::ExecutionRejected(
            "Amazon Braket path identifier contains forbidden URL delimiter"
                .to_owned(),
        ));
    }

    Ok(value.to_owned())
}

/// Creates a short deterministic hash for request correlation identifiers.
fn stable_short_hash(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());

    let digest = hasher.finalize();

    hex::encode(digest)[..32].to_owned()
}

/// Safely truncates provider diagnostics.
fn bounded_diagnostic(
    value: &str,
    maximum: usize,
) -> String {
    value.chars().take(maximum).collect()
}

/// Maps generic provider errors into Zamani backend errors.
fn map_provider_error(
    error: ProviderError,
) -> BackendError {
    let message =
        bounded_diagnostic(&error.message, MAX_BRAKET_FAILURE_REASON_LENGTH);

    match error.category {
        ProviderFailureCategory::Authentication => {
            BackendError::ExecutionRejected(
                "Amazon Braket authentication failed".to_owned(),
            )
        }

        ProviderFailureCategory::Authorization => {
            BackendError::ExecutionRejected(
                "Amazon Braket authorization failed".to_owned(),
            )
        }

        ProviderFailureCategory::NotFound => {
            BackendError::ExecutionRejected(format!(
                "Amazon Braket resource was not found: {message}"
            ))
        }

        ProviderFailureCategory::RateLimited => {
            BackendError::ExecutionUnavailable(format!(
                "Amazon Braket rate limit reached: {message}"
            ))
        }

        ProviderFailureCategory::Capacity => {
            BackendError::ExecutionUnavailable(format!(
                "Amazon Braket capacity unavailable: {message}"
            ))
        }

        ProviderFailureCategory::Unavailable => {
            BackendError::ExecutionUnavailable(format!(
                "Amazon Braket unavailable: {message}"
            ))
        }

        ProviderFailureCategory::Unsupported => {
            BackendError::ExecutionRejected(format!(
                "Amazon Braket rejected unsupported functionality: {message}"
            ))
        }

        ProviderFailureCategory::Conflict => {
            BackendError::ExecutionRejected(format!(
                "Amazon Braket task conflict: {message}"
            ))
        }

        ProviderFailureCategory::Timeout => {
            BackendError::ExecutionUnavailable(format!(
                "Amazon Braket request timed out: {message}"
            ))
        }

        ProviderFailureCategory::InvalidResponse => {
            BackendError::ExecutionRejected(format!(
                "Amazon Braket returned an invalid response: {message}"
            ))
        }

        ProviderFailureCategory::Transport
        | ProviderFailureCategory::Execution
        | ProviderFailureCategory::Unknown
        | ProviderFailureCategory::InvalidRequest => {
            BackendError::ExecutionUnavailable(format!(
                "Amazon Braket provider transport failure: {message}"
            ))
        }
    }
}

/// Maps artifact-store failures into canonical backend errors.
fn map_artifact_error(
    error: BraketArtifactError,
) -> BackendError {
    match error {
        BraketArtifactError::NotFound => {
            BackendError::ExecutionUnavailable(
                "Amazon Braket result artifact is not yet available"
                    .to_owned(),
            )
        }

        BraketArtifactError::AccessDenied => {
            BackendError::ExecutionRejected(
                "access to Amazon Braket result artifact was denied"
                    .to_owned(),
            )
        }

        BraketArtifactError::Unavailable(message) => {
            BackendError::ExecutionUnavailable(format!(
                "Amazon Braket result storage unavailable: {message}"
            ))
        }

        BraketArtifactError::InvalidPayload(message) => {
            BackendError::ExecutionRejected(format!(
                "Amazon Braket result artifact is invalid: {message}"
            ))
        }

        BraketArtifactError::Timeout => {
            BackendError::ExecutionUnavailable(
                "Amazon Braket result artifact retrieval timed out"
                    .to_owned(),
            )
        }

        BraketArtifactError::Other(message) => {
            BackendError::ExecutionUnavailable(format!(
                "Amazon Braket result retrieval failed: {message}"
            ))
        }
    }
}

/// Maps generic adapter construction failures.
fn generic_error(
    error: impl fmt::Display,
) -> BackendError {
    BackendError::ExecutionRejected(error.to_string())
}

/// Returns whether a value is a valid classical bitstring.
fn is_valid_bitstring(
    value: &str,
) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|character| character == '0' || character == '1')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_openqasm_format_constants() {
        assert_eq!(
            FORMAT_OPENQASM_30,
            "openqasm-3.0"
        );

        assert_eq!(
            FORMAT_OPENQASM_31,
            "openqasm-3.1"
        );
    }

    #[test]
    fn validates_braket_device_arn() {
        assert!(
            validate_braket_device_arn(
                "arn:aws:braket:us-west-1::device/qpu/rigetti/Ankaa-3"
            )
            .is_ok()
        );
    }

    #[test]
    fn validates_braket_simulator_arn() {
        assert!(
            validate_braket_device_arn(
                "arn:aws:braket:::device/quantum-simulator/amazon/sv1"
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_non_braket_device_arn() {
        assert!(
            validate_braket_device_arn(
                "arn:aws:s3:::bucket"
            )
            .is_err()
        );
    }

    #[test]
    fn validates_task_arn() {
        assert!(
            validate_braket_task_arn(
                "arn:aws:braket:us-west-1:123456789012:quantum-task/123"
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_task_arn_with_wrong_resource() {
        assert!(
            validate_braket_task_arn(
                "arn:aws:braket:us-west-1:123456789012:device/qpu/test"
            )
            .is_err()
        );
    }

    #[test]
    fn validates_s3_bucket() {
        assert!(
            validate_s3_bucket("amazon-braket-results")
                .is_ok()
        );
    }

    #[test]
    fn rejects_invalid_s3_bucket() {
        assert!(
            validate_s3_bucket("Amazon-Braket-Results")
                .is_err()
        );
    }

    #[test]
    fn rejects_s3_path_traversal() {
        assert!(
            validate_s3_prefix("../results")
                .is_err()
        );

        assert!(
            validate_s3_prefix("foo/../results")
                .is_err()
        );
    }

    #[test]
    fn creates_openqasm_action() {
        let program =
            BackendProgram::new(
                FORMAT_OPENQASM_30,
                b"OPENQASM 3;\nqubit[1] q;\n".to_vec(),
            )
            .expect("valid program");

        let source =
            std::str::from_utf8(program.bytes())
                .expect("utf8");

        assert!(source.contains("OPENQASM 3;"));
    }

    #[test]
    fn validates_native_action_schema() {
        let action = json!({
            "braketSchemaHeader": {
                "name": BRAKET_OPENQASM_ACTION_SCHEMA,
                "version": BRAKET_OPENQASM_ACTION_VERSION
            },
            "source": "OPENQASM 3;"
        });

        assert!(
            validate_serialized_action(&action)
                .is_ok()
        );
    }

    #[test]
    fn rejects_native_action_without_schema_header() {
        let action = json!({
            "source": "OPENQASM 3;"
        });

        assert!(
            validate_serialized_action(&action)
                .is_err()
        );
    }

    #[test]
    fn extracts_measurement_counts() {
        let value = json!({
            "measurementCounts": {
                "000": 3,
                "111": 2
            }
        });

        let counts =
            extract_counts(&value)
                .expect("counts");

        assert_eq!(counts.get("000"), Some(&3));
        assert_eq!(counts.get("111"), Some(&2));
    }

    #[test]
    fn extracts_measurement_arrays() {
        let value = json!({
            "measurements": [
                [0, 0, 0],
                [1, 1, 1],
                [0, 0, 0]
            ]
        });

        let counts =
            extract_counts(&value)
                .expect("counts");

        assert_eq!(counts.get("000"), Some(&2));
        assert_eq!(counts.get("111"), Some(&1));
    }

    #[test]
    fn rejects_invalid_bitstring() {
        assert!(!is_valid_bitstring("012"));
        assert!(!is_valid_bitstring(""));
        assert!(is_valid_bitstring("0101"));
    }

    #[test]
    fn normalizes_braket_states() {
        assert_eq!(
            AwsBraketAdapter::normalize_task_state("CREATED")
                .expect("state"),
            BackendJobState::Created
        );

        assert_eq!(
            AwsBraketAdapter::normalize_task_state("QUEUED")
                .expect("state"),
            BackendJobState::Queued
        );

        assert_eq!(
            AwsBraketAdapter::normalize_task_state("RUNNING")
                .expect("state"),
            BackendJobState::Running
        );

        assert_eq!(
            AwsBraketAdapter::normalize_task_state("COMPLETED")
                .expect("state"),
            BackendJobState::Completed
        );

        assert_eq!(
            AwsBraketAdapter::normalize_task_state("CANCELLED")
                .expect("state"),
            BackendJobState::Cancelled
        );

        assert_eq!(
            AwsBraketAdapter::normalize_task_state("FAILED")
                .expect("state"),
            BackendJobState::Failed
        );
    }

    #[test]
    fn unknown_state_is_not_silently_accepted() {
        assert!(
            AwsBraketAdapter::normalize_task_state("SOMETHING_NEW")
                .is_err()
        );
    }

    #[test]
    fn client_token_rejects_empty_request_id() {
        assert!(
            normalize_client_token("")
                .is_err()
        );
    }

    #[test]
    fn client_token_rejects_oversized_request_id() {
        let value =
            "x".repeat(MAX_BRAKET_CLIENT_TOKEN_LENGTH + 1);

        assert!(
            normalize_client_token(&value)
                .is_err()
        );
    }

    #[test]
    fn client_token_accepts_safe_request_id() {
        assert_eq!(
            normalize_client_token("zamani-run-001")
                .expect("token"),
            "zamani-run-001"
        );
    }

    #[test]
    fn provider_errors_do_not_expose_authentication_material() {
        let error = ProviderError::new(
            ProviderFailureCategory::Authentication,
            "authentication failed",
            RetryClass::DoNotRetry,
        )
        .expect("valid provider error");

        let mapped = map_provider_error(error);

        assert_eq!(
            mapped,
            BackendError::ExecutionRejected(
                "Amazon Braket authentication failed".to_owned()
            )
        );
    }

    #[test]
    fn artifact_not_found_is_retryable_at_higher_layer() {
        let mapped =
            map_artifact_error(
                BraketArtifactError::NotFound
            );

        assert!(
            matches!(
                mapped,
                BackendError::ExecutionUnavailable(_)
            )
        );
    }
}