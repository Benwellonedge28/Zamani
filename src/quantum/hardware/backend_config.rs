//! Zamani Quantum — Backend Configuration
//!
//! Production-grade, provider-independent configuration for quantum execution
//! backends.
//!
//! # Responsibility
//!
//! This module defines configuration that determines HOW a backend is intended
//! to be used. It does not define what a backend is capable of and it does not
//! perform backend execution.
//!
//! This module owns:
//!
//! - backend configuration;
//! - backend/provider references;
//! - endpoint references;
//! - execution-model preference;
//! - timeout policy;
//! - retry policy;
//! - queue policy;
//! - calibration policy;
//! - validation policy;
//! - serialization-format preference;
//! - cost-estimation policy;
//! - region selection;
//! - account/project/workspace references;
//! - configuration validation;
//! - deterministic configuration normalization;
//! - security validation;
//! - configuration invariants.
//!
//! This module deliberately does NOT own:
//!
//! - credentials;
//! - API keys;
//! - access tokens;
//! - passwords;
//! - private keys;
//! - authentication sessions;
//! - provider SDKs;
//! - network communication;
//! - backend discovery;
//! - backend capabilities;
//! - backend topology;
//! - backend calibration data;
//! - quantum execution;
//! - job lifecycle;
//! - result handling;
//! - routing algorithms;
//! - scheduling algorithms;
//! - benchmarking;
//! - provider-specific protocol implementation;
//! - serialization implementation.
//!
//! Those responsibilities belong to other hardware modules.
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
//!       backend
//!        |
//!        +-------------------------+
//!        |                         |
//!        v                         v
//! backend_config             backend capabilities
//!        |                         |
//!        +-------------+-----------+
//!                      |
//!                      v
//!                execution layer
//!                      |
//!                      v
//!                provider adapter
//!                      |
//!                      v
//!                    QPU
//! ```
//!
//! `BackendConfig` is configuration, not backend state.
//!
//! # Dependency policy
//!
//! This file intentionally depends only on the Rust standard library.
//!
//! This makes it possible to complete and freeze this file before the
//! following modules are implemented:
//!
//! - `identity.rs`;
//! - `capabilities.rs`;
//! - `backend_status.rs`;
//! - `backend_trait.rs`;
//! - `backend.rs`;
//! - `execution.rs`;
//! - `provider.rs`;
//! - `credentials.rs`;
//! - `authentication.rs`;
//! - `serialization.rs`.
//!
//! Those modules consume this contract instead of changing it.
//!
//! # Security
//!
//! Configuration is not a secret store.
//!
//! The following are forbidden in configuration values:
//!
//! - API keys;
//! - bearer tokens;
//! - authorization headers;
//! - passwords;
//! - private keys;
//! - cookies;
//! - PEM private-key material;
//! - credential query parameters;
//! - embedded user/password URL authorities.
//!
//! Credentials must be represented by references owned by
//! `credentials.rs`/`authentication.rs`.
//!
//! # Determinism
//!
//! Configuration validation and normalization are deterministic.
//!
//! Runtime retry implementations may add bounded random jitter outside this
//! module, but the configuration itself contains no randomness.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Stability
//!
//! This file is intended to become the stable configuration contract for the
//! Zamani Quantum Hardware Abstraction Layer.
//!
//! Adding provider-specific fields must NOT require changing this file.
//! Provider-specific configuration belongs in provider adapters or provider
//! extension configuration.
//!
//! # Integration contract
//!
//! `backend.rs` consumes `BackendConfig` as backend policy.
//!
//! `backend_trait.rs` consumes the policy when defining backend operations.
//!
//! `execution.rs` consumes timeout, retry, queue and validation policies.
//!
//! `provider.rs` consumes provider/backend references.
//!
//! `credentials.rs` resolves credential references separately.
//!
//! `authentication.rs` consumes credential references without storing secrets
//! in this structure.
//!
//! `serialization.rs` owns external serialization.
//!
//! `discovery.rs` may construct configurations from discovered metadata.
//!
//! Danga may eventually load this structure from project configuration, but
//! Danga must not become a dependency of this module.
//!
//! Benchmarking may select or override configuration through its execution
//! orchestration layer, but this module must never depend on benchmarking.
//!
//! # No-re-edit rule
//!
//! This module intentionally uses stable primitive representations for
//! cross-module references instead of depending on types that are scheduled
//! to be created later. Later modules can convert their canonical IDs into
//! these validated references without requiring this file to change.
//!
//! The file is complete when:
//!
//! - every field has an explicit invariant;
//! - invalid values are rejected;
//! - secrets are rejected;
//! - defaults are deterministic;
//! - policy semantics are explicit;
//! - runtime-only behaviour is not encoded here;
//! - tests cover boundary and security cases;
//! - downstream modules can consume the public contract without modifying it.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::error::Error;
use std::fmt;
use std::time::Duration;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for backend configuration.
pub const BACKEND_CONFIG_SCHEMA_ID: &str =
    "zamani.quantum.hardware.backend_config";

/// Semantic schema version.
///
/// Increment only when serialized/configuration semantics become incompatible.
pub const BACKEND_CONFIG_SCHEMA_VERSION: u16 = 1;

/// Maximum backend-reference length.
pub const MAX_BACKEND_REFERENCE_LENGTH: usize = 512;

/// Maximum provider-reference length.
pub const MAX_PROVIDER_REFERENCE_LENGTH: usize = 512;

/// Maximum endpoint-reference length.
pub const MAX_ENDPOINT_REFERENCE_LENGTH: usize = 2048;

/// Maximum region length.
pub const MAX_REGION_LENGTH: usize = 256;

/// Maximum account/project/workspace reference length.
pub const MAX_SCOPE_REFERENCE_LENGTH: usize = 512;

/// Maximum configuration label length.
pub const MAX_LABEL_LENGTH: usize = 256;

/// Maximum number of retry attempts.
///
/// An attempt count of zero means that the initial operation is attempted once
/// and no retry occurs.
pub const MAX_RETRY_ATTEMPTS: u32 = 100;

/// Maximum timeout in seconds accepted by configuration.
pub const MAX_TIMEOUT_SECONDS: u64 = 7 * 24 * 60 * 60;

/// Maximum retry backoff in seconds.
pub const MAX_RETRY_BACKOFF_SECONDS: u64 = 24 * 60 * 60;

/// Maximum queue wait in seconds.
pub const MAX_QUEUE_WAIT_SECONDS: u64 = 7 * 24 * 60 * 60;

/// Maximum metadata-free configuration label count.
pub const MAX_LABEL_COUNT: usize = 64;

// =============================================================================
// Execution model
// =============================================================================

/// Preferred quantum execution model.
///
/// This is deliberately independent of physical technology and backend kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionModel {
    /// Conventional gate-model quantum circuit execution.
    GateModel,

    /// Gate-model circuits that may contain measurement-dependent control flow.
    DynamicCircuit,

    /// Pulse-level quantum execution.
    Pulse,

    /// Analog Hamiltonian/control execution.
    Analog,

    /// Quantum annealing / Ising / QUBO execution.
    Annealing,

    /// Logical/fault-tolerant quantum execution.
    Logical,

    /// General sampling workload.
    Sampling,

    /// Provider-defined execution model.
    Custom,
}

impl ExecutionModel {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GateModel => "gate_model",
            Self::DynamicCircuit => "dynamic_circuit",
            Self::Pulse => "pulse",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Logical => "logical",
            Self::Sampling => "sampling",
            Self::Custom => "custom",
        }
    }
}

impl Default for ExecutionModel {
    fn default() -> Self {
        Self::GateModel
    }
}

impl fmt::Display for ExecutionModel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Endpoint reference
// =============================================================================

/// Provider endpoint reference.
///
/// This is intentionally NOT a credential-bearing URL type.
///
/// Accepted forms are:
///
/// - `https://host/path`
/// - `http://host/path` for explicitly local/private development scenarios;
/// - `local://name`;
/// - `unix://path`;
/// - `provider://name`;
/// - `env://NAME`;
/// - `config://name`.
///
/// Production remote providers should normally use HTTPS.
///
/// Credentials embedded in URL authorities or query strings are rejected.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EndpointReference(String);

impl EndpointReference {
    /// Creates and validates an endpoint reference.
    pub fn new(value: impl Into<String>) -> Result<Self, BackendConfigError> {
        let value = value.into();
        validate_endpoint_reference(&value)?;

        Ok(Self(value))
    }

    /// Returns the canonical endpoint reference.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the wrapper and returns its string.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Returns whether this endpoint uses HTTPS.
    pub fn is_https(&self) -> bool {
        self.0
            .get(..8)
            .map(|prefix| prefix.eq_ignore_ascii_case("https://"))
            .unwrap_or(false)
    }

    /// Returns whether this endpoint is explicitly local.
    pub fn is_local(&self) -> bool {
        self.0.starts_with("local://")
            || self.0.starts_with("unix://")
            || self.0.starts_with("config://")
            || self.0.starts_with("env://")
    }
}

impl fmt::Display for EndpointReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Timeout policy
// =============================================================================

/// Timeout policy for backend operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeoutPolicy {
    /// Maximum time allowed for submission.
    pub submission: Duration,

    /// Maximum time allowed while polling a queued/running job.
    pub execution: Duration,

    /// Maximum time allowed to retrieve final results.
    pub result: Duration,

    /// Maximum time allowed for cancellation.
    pub cancellation: Duration,

    /// Maximum time allowed for health/discovery requests.
    pub control: Duration,
}

impl Default for TimeoutPolicy {
    fn default() -> Self {
        Self {
            submission: Duration::from_secs(60),
            execution: Duration::from_secs(24 * 60 * 60),
            result: Duration::from_secs(10 * 60),
            cancellation: Duration::from_secs(60),
            control: Duration::from_secs(30),
        }
    }
}

impl TimeoutPolicy {
    /// Creates a timeout policy using one timeout for every operation.
    pub fn uniform(timeout: Duration) -> Result<Self, BackendConfigError> {
        validate_duration(
            "uniform timeout",
            timeout,
            Duration::from_secs(1),
            max_timeout(),
        )?;

        Ok(Self {
            submission: timeout,
            execution: timeout,
            result: timeout,
            cancellation: timeout,
            control: timeout,
        })
    }

    /// Validates all timeout values.
    pub fn validate(&self) -> Result<(), BackendConfigError> {
        validate_duration(
            "submission timeout",
            self.submission,
            Duration::from_millis(1),
            max_timeout(),
        )?;

        validate_duration(
            "execution timeout",
            self.execution,
            Duration::from_millis(1),
            max_timeout(),
        )?;

        validate_duration(
            "result timeout",
            self.result,
            Duration::from_millis(1),
            max_timeout(),
        )?;

        validate_duration(
            "cancellation timeout",
            self.cancellation,
            Duration::from_millis(1),
            max_timeout(),
        )?;

        validate_duration(
            "control timeout",
            self.control,
            Duration::from_millis(1),
            max_timeout(),
        )
    }
}

// =============================================================================
// Retry policy
// =============================================================================

/// Retry policy.
///
/// Retry classification is intentionally NOT performed here.
///
/// A provider adapter/execution layer decides whether a concrete failure is
/// retryable. This structure only describes the permitted retry envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RetryPolicy {
    /// Maximum number of retries after the initial attempt.
    pub max_retries: u32,

    /// Initial backoff.
    pub initial_backoff: Duration,

    /// Maximum backoff.
    pub max_backoff: Duration,

    /// Multiplicative backoff factor represented as a fixed-point value.
///
/// For example:
///
/// `2_000` = 2.0x
///
/// `1_500` = 1.5x
///
/// This avoids floating-point values in configuration.
    pub backoff_multiplier_milli: u32,

    /// Whether retries are allowed for submission operations.
    pub retry_submission: bool,

    /// Whether retries are allowed for result retrieval.
    pub retry_result_retrieval: bool,

    /// Whether retries are allowed for control operations.
    pub retry_control: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff: Duration::from_millis(250),
            max_backoff: Duration::from_secs(30),
            backoff_multiplier_milli: 2_000,
            retry_submission: false,
            retry_result_retrieval: true,
            retry_control: true,
        }
    }
}

impl RetryPolicy {
    /// Creates a policy with no retries.
    pub const fn disabled() -> Self {
        Self {
            max_retries: 0,
            initial_backoff: Duration::from_secs(0),
            max_backoff: Duration::from_secs(0),
            backoff_multiplier_milli: 1_000,
            retry_submission: false,
            retry_result_retrieval: false,
            retry_control: false,
        }
    }

    /// Validates the retry policy.
    pub fn validate(&self) -> Result<(), BackendConfigError> {
        if self.max_retries > MAX_RETRY_ATTEMPTS {
            return Err(BackendConfigError::OutOfRange {
                field: "retry.max_retries",
                message: format!(
                    "maximum is {}",
                    MAX_RETRY_ATTEMPTS
                ),
            });
        }

        if self.backoff_multiplier_milli == 0 {
            return Err(BackendConfigError::InvalidValue {
                field: "retry.backoff_multiplier_milli",
                message: "must be greater than zero".to_string(),
            });
        }

        if self.backoff_multiplier_milli > 100_000 {
            return Err(BackendConfigError::OutOfRange {
                field: "retry.backoff_multiplier_milli",
                message: "maximum is 100000 (100x)".to_string(),
            });
        }

        validate_duration(
            "retry.initial_backoff",
            self.initial_backoff,
            Duration::from_millis(0),
            max_retry_backoff(),
        )?;

        validate_duration(
            "retry.max_backoff",
            self.max_backoff,
            Duration::from_millis(0),
            max_retry_backoff(),
        )?;

        if self.max_backoff < self.initial_backoff {
            return Err(BackendConfigError::InvalidValue {
                field: "retry.max_backoff",
                message: "must be greater than or equal to initial_backoff"
                    .to_string(),
            });
        }

        Ok(())
    }

    /// Calculates a deterministic exponential backoff for an attempt.
    ///
    /// This function does not add jitter.
    ///
    /// Runtime layers may apply bounded jitter externally when appropriate.
    pub fn backoff_for_retry(
        &self,
        retry_number: u32,
    ) -> Duration {
        if retry_number == 0 || self.max_retries == 0 {
            return Duration::from_secs(0);
        }

        let mut current = self.initial_backoff;

        for _ in 1..retry_number {
            let millis = current.as_millis();

            let multiplied = millis
                .saturating_mul(self.backoff_multiplier_milli as u128)
                / 1_000;

            let max_millis = self.max_backoff.as_millis();

            let bounded = multiplied.min(max_millis);

            current = Duration::from_millis(
                bounded.min(u64::MAX as u128) as u64,
            );

            if current >= self.max_backoff {
                break;
            }
        }

        current.min(self.max_backoff)
    }
}

// =============================================================================
// Queue policy
// =============================================================================

/// Queue handling policy.
///
/// This describes client behaviour, not provider queue implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QueuePolicy {
    /// Whether queue information should be requested when available.
    pub observe_queue: bool,

    /// Whether execution may wait for a queued job.
    pub wait_for_queue: bool,

    /// Maximum allowed queue wait.
    pub max_queue_wait: Duration,

    /// Requested priority.
///
/// Provider adapters may reject unsupported priority values.
    pub priority: QueuePriority,

    /// Whether client-side queue polling is enabled.
    pub poll_queue: bool,
}

impl Default for QueuePolicy {
    fn default() -> Self {
        Self {
            observe_queue: true,
            wait_for_queue: true,
            max_queue_wait: Duration::from_secs(24 * 60 * 60),
            priority: QueuePriority::Normal,
            poll_queue: true,
        }
    }
}

impl QueuePolicy {
    /// Validates queue policy.
    pub fn validate(&self) -> Result<(), BackendConfigError> {
        validate_duration(
            "queue.max_queue_wait",
            self.max_queue_wait,
            Duration::from_secs(0),
            max_queue_wait(),
        )
    }
}

/// Requested job priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QueuePriority {
    /// Lowest normal priority.
    Low,

    /// Normal priority.
    Normal,

    /// Elevated priority.
    High,

    /// Provider-specific urgent priority.
    Urgent,
}

impl QueuePriority {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::High => "high",
            Self::Urgent => "urgent",
        }
    }
}

impl Default for QueuePriority {
    fn default() -> Self {
        Self::Normal
    }
}

// =============================================================================
// Calibration policy
// =============================================================================

/// Policy controlling use of hardware calibration information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CalibrationPolicy {
    /// Whether calibration data should be required before execution.
    pub require_calibration: bool,

    /// Whether stale calibration may be used.
    ///
    /// Production configurations should normally keep this false.
    pub allow_stale: bool,

    /// Maximum acceptable calibration age.
    pub max_age: Duration,

    /// Whether calibration provenance must be recorded in execution metadata.
    pub require_provenance: bool,
}

impl Default for CalibrationPolicy {
    fn default() -> Self {
        Self {
            require_calibration: true,
            allow_stale: false,
            max_age: Duration::from_secs(60 * 60),
            require_provenance: true,
        }
    }
}

impl CalibrationPolicy {
    /// Strict production calibration policy.
    pub const fn strict() -> Self {
        Self {
            require_calibration: true,
            allow_stale: false,
            max_age: Duration::from_secs(60 * 60),
            require_provenance: true,
        }
    }

    /// Validates the calibration policy.
    pub fn validate(&self) -> Result<(), BackendConfigError> {
        if self.require_calibration && self.max_age.is_zero() {
            return Err(BackendConfigError::InvalidValue {
                field: "calibration.max_age",
                message:
                    "must be greater than zero when calibration is required"
                        .to_string(),
            });
        }

        if self.allow_stale && self.require_calibration {
            return Err(BackendConfigError::InvalidValue {
                field: "calibration.allow_stale",
                message:
                    "stale calibration cannot be enabled in strict required mode"
                        .to_string(),
            });
        }

        validate_duration(
            "calibration.max_age",
            self.max_age,
            Duration::from_secs(0),
            max_timeout(),
        )
    }
}

// =============================================================================
// Validation policy
// =============================================================================

/// Backend validation policy.
///
/// This controls how much pre-execution validation the execution pipeline
/// requires. It does not perform the validation itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ValidationPolicy {
    /// Validate backend identity/configuration before submission.
    pub validate_backend: bool,

    /// Validate workload capabilities before submission.
    pub validate_capabilities: bool,

    /// Validate topology before submission.
    pub validate_topology: bool,

    /// Validate timing constraints before submission.
    pub validate_timing: bool,

    /// Validate calibration freshness before submission.
    pub validate_calibration: bool,

    /// Reject experimental-only capabilities unless explicitly allowed.
    pub allow_experimental_capabilities: bool,

    /// Reject unknown capabilities instead of silently ignoring them.
    pub reject_unknown_requirements: bool,
}

impl Default for ValidationPolicy {
    fn default() -> Self {
        Self {
            validate_backend: true,
            validate_capabilities: true,
            validate_topology: true,
            validate_timing: true,
            validate_calibration: true,
            allow_experimental_capabilities: false,
            reject_unknown_requirements: true,
        }
    }
}

impl ValidationPolicy {
    /// Maximum-safety production policy.
    pub const fn strict() -> Self {
        Self {
            validate_backend: true,
            validate_capabilities: true,
            validate_topology: true,
            validate_timing: true,
            validate_calibration: true,
            allow_experimental_capabilities: false,
            reject_unknown_requirements: true,
        }
    }
}

// =============================================================================
// Serialization policy
// =============================================================================

/// Preferred representation for provider interoperability.
///
/// This does NOT perform serialization. `serialization.rs` owns that work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SerializationFormat {
    /// Zamani's canonical internal representation.
    Zamani,

    /// OpenQASM 3 interoperability format.
    OpenQasm3,

    /// QIR interoperability format.
    Qir,

    /// Provider-native representation.
    ProviderNative,

    /// Provider-selected format.
    Auto,
}

impl SerializationFormat {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Zamani => "zamani",
            Self::OpenQasm3 => "openqasm3",
            Self::Qir => "qir",
            Self::ProviderNative => "provider_native",
            Self::Auto => "auto",
        }
    }
}

impl Default for SerializationFormat {
    fn default() -> Self {
        Self::Auto
    }
}

/// Serialization preference policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SerializationPolicy {
    /// Preferred workload representation.
    pub preferred_format: SerializationFormat,

    /// Whether provider-native conversion is allowed.
    pub allow_provider_native: bool,

    /// Whether implicit format conversion is allowed.
    pub allow_implicit_conversion: bool,

    /// Whether a conversion must be recorded in provenance metadata.
    pub require_conversion_provenance: bool,
}

impl Default for SerializationPolicy {
    fn default() -> Self {
        Self {
            preferred_format: SerializationFormat::Auto,
            allow_provider_native: true,
            allow_implicit_conversion: true,
            require_conversion_provenance: true,
        }
    }
}

// =============================================================================
// Cost policy
// =============================================================================

/// Cost-estimation behaviour.
///
/// Pricing data itself never belongs in this structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CostPolicy {
    /// Whether cost estimation should be requested before submission when
    /// supported.
    pub estimate_before_submission: bool,

    /// Whether execution should be rejected when a required estimate cannot
    /// be obtained.
    pub require_estimate: bool,

    /// Whether provider pricing metadata may be used.
    pub allow_provider_pricing: bool,
}

impl Default for CostPolicy {
    fn default() -> Self {
        Self {
            estimate_before_submission: true,
            require_estimate: false,
            allow_provider_pricing: true,
        }
    }
}

// =============================================================================
// Configuration references
// =============================================================================

/// Validated backend identifier/reference.
///
/// This is intentionally a provider-neutral string reference. The canonical
/// typed backend identity is owned by `identity.rs`; that type can convert into
/// or from this representation without making configuration depend on it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BackendReference(String);

impl BackendReference {
    /// Creates a validated backend reference.
    pub fn new(value: impl Into<String>) -> Result<Self, BackendConfigError> {
        let value = value.into();

        validate_identifier(
            "backend_id",
            &value,
            MAX_BACKEND_REFERENCE_LENGTH,
        )?;

        Ok(Self(value))
    }

    /// Returns the reference.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the reference.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for BackendReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Validated provider identifier/reference.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProviderReference(String);

impl ProviderReference {
    /// Creates a validated provider reference.
    pub fn new(value: impl Into<String>) -> Result<Self, BackendConfigError> {
        let value = value.into();

        validate_identifier(
            "provider_id",
            &value,
            MAX_PROVIDER_REFERENCE_LENGTH,
        )?;

        Ok(Self(value))
    }

    /// Returns the reference.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the reference.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ProviderReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Backend configuration
// =============================================================================

/// Complete provider-neutral backend configuration.
///
/// # Important distinction
///
/// `BackendConfig` describes desired client/execution behaviour.
///
/// It does NOT claim that the backend actually supports those settings.
///
/// Actual support must be established through `BackendCapabilities` and
/// compatibility validation.
///
/// # Invariants
///
/// A valid configuration guarantees:
///
/// 1. backend ID is non-empty and bounded;
/// 2. provider ID is non-empty and bounded;
/// 3. endpoint is credential-free;
/// 4. remote endpoints use a valid URI scheme;
/// 5. timeout policy is valid;
/// 6. retry policy is valid;
/// 7. queue policy is valid;
/// 8. calibration policy is valid;
/// 9. configuration scope references are valid;
/// 10. labels are bounded and secret-free;
/// 11. production configuration does not contain credentials;
/// 12. provider-independent semantics remain independent of provider SDKs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendConfig {
    /// Stable backend identifier.
    pub backend_id: BackendReference,

    /// Provider owning the backend.
    pub provider_id: ProviderReference,

    /// Endpoint used by the provider adapter.
    pub endpoint: EndpointReference,

    /// Preferred execution model.
    pub execution_model: ExecutionModel,

    /// Operation timeout policy.
    pub timeout: TimeoutPolicy,

    /// Retry policy.
    pub retry: RetryPolicy,

    /// Queue handling policy.
    pub queue: QueuePolicy,

    /// Calibration requirements.
    pub calibration: CalibrationPolicy,

    /// Pre-execution validation policy.
    pub validation: ValidationPolicy,

    /// Serialization/interoperability preference.
    pub serialization: SerializationPolicy,

    /// Cost-estimation policy.
    pub cost: CostPolicy,

    /// Optional provider region.
    pub region: Option<String>,

    /// Optional account identifier.
    pub account: Option<String>,

    /// Optional project/workspace identifier.
    pub project: Option<String>,

    /// Optional human-readable configuration label.
    pub label: Option<String>,

    /// Whether this configuration is intended for production use.
    ///
    /// This is an explicit policy marker. It does not itself prove that the
    /// backend is production-ready.
    pub production: bool,
}

impl BackendConfig {
    /// Creates a production-oriented configuration with strict defaults.
    pub fn new(
        backend_id: BackendReference,
        provider_id: ProviderReference,
        endpoint: EndpointReference,
    ) -> Result<Self, BackendConfigError> {
        let config = Self {
            backend_id,
            provider_id,
            endpoint,
            execution_model: ExecutionModel::GateModel,
            timeout: TimeoutPolicy::default(),
            retry: RetryPolicy::default(),
            queue: QueuePolicy::default(),
            calibration: CalibrationPolicy::strict(),
            validation: ValidationPolicy::strict(),
            serialization: SerializationPolicy::default(),
            cost: CostPolicy::default(),
            region: None,
            account: None,
            project: None,
            label: None,
            production: true,
        };

        config.validate()?;

        Ok(config)
    }

    /// Returns a local-development configuration.
    ///
    /// Local configuration remains fully validated but does not require
    /// provider credentials.
    pub fn local(
        backend_id: BackendReference,
        endpoint: EndpointReference,
    ) -> Result<Self, BackendConfigError> {
        let provider_id =
            ProviderReference::new("local")?;

        let mut config =
            Self::new(backend_id, provider_id, endpoint)?;

        config.production = false;
        config.calibration.require_calibration = false;
        config.calibration.require_provenance = false;
        config.validation.validate_calibration = false;

        config.validate()?;

        Ok(config)
    }

    /// Validates the complete configuration.
    pub fn validate(&self) -> Result<(), BackendConfigError> {
        validate_identifier(
            "backend_id",
            self.backend_id.as_str(),
            MAX_BACKEND_REFERENCE_LENGTH,
        )?;

        validate_identifier(
            "provider_id",
            self.provider_id.as_str(),
            MAX_PROVIDER_REFERENCE_LENGTH,
        )?;

        validate_endpoint_reference(self.endpoint.as_str())?;

        self.timeout.validate()?;
        self.retry.validate()?;
        self.queue.validate()?;
        self.calibration.validate()?;

        if let Some(region) = &self.region {
            validate_scope_reference(
                "region",
                region,
                MAX_REGION_LENGTH,
            )?;
        }

        if let Some(account) = &self.account {
            validate_scope_reference(
                "account",
                account,
                MAX_SCOPE_REFERENCE_LENGTH,
            )?;
        }

        if let Some(project) = &self.project {
            validate_scope_reference(
                "project",
                project,
                MAX_SCOPE_REFERENCE_LENGTH,
            )?;
        }

        if let Some(label) = &self.label {
            validate_scope_reference(
                "label",
                label,
                MAX_LABEL_LENGTH,
            )?;
        }

        if self.production {
            if self.endpoint.as_str().starts_with("http://")
                && !self.endpoint.is_local()
            {
                return Err(BackendConfigError::InsecureRemoteEndpoint);
            }

            if self.calibration.allow_stale {
                return Err(BackendConfigError::ProductionPolicyViolation {
                    field: "calibration.allow_stale",
                    message:
                        "production configuration cannot permit stale calibration"
                            .to_string(),
                });
            }

            if !self.validation.validate_backend {
                return Err(BackendConfigError::ProductionPolicyViolation {
                    field: "validation.validate_backend",
                    message:
                        "production configuration must validate the backend"
                            .to_string(),
                });
            }

            if !self.validation.validate_capabilities {
                return Err(BackendConfigError::ProductionPolicyViolation {
                    field: "validation.validate_capabilities",
                    message:
                        "production configuration must validate capabilities"
                            .to_string(),
                });
            }

            if self.validation.allow_experimental_capabilities {
                return Err(BackendConfigError::ProductionPolicyViolation {
                    field: "validation.allow_experimental_capabilities",
                    message:
                        "experimental capabilities must be explicitly selected outside the strict production profile"
                            .to_string(),
                });
            }
        }

        Ok(())
    }

    /// Returns a canonical deterministic representation.
    ///
    /// This is intentionally not JSON and is not the responsibility of the
    /// serialization subsystem. It exists for deterministic comparisons,
    /// cache keys and tests.
    pub fn canonical_key(&self) -> String {
        let region = self.region.as_deref().unwrap_or("");
        let account = self.account.as_deref().unwrap_or("");
        let project = self.project.as_deref().unwrap_or("");
        let label = self.label.as_deref().unwrap_or("");

        format!(
            concat!(
                "schema={};version={};",
                "backend={};provider={};endpoint={};",
                "execution_model={};",
                "timeout_submission_ms={};",
                "timeout_execution_ms={};",
                "timeout_result_ms={};",
                "timeout_cancellation_ms={};",
                "timeout_control_ms={};",
                "retry_max={};",
                "retry_initial_ms={};",
                "retry_max_backoff_ms={};",
                "retry_multiplier_milli={};",
                "retry_submission={};",
                "retry_result={};",
                "retry_control={};",
                "queue_observe={};",
                "queue_wait={};",
                "queue_max_wait_ms={};",
                "queue_priority={};",
                "queue_poll={};",
                "calibration_required={};",
                "calibration_stale={};",
                "calibration_max_age_ms={};",
                "calibration_provenance={};",
                "validation_backend={};",
                "validation_capabilities={};",
                "validation_topology={};",
                "validation_timing={};",
                "validation_calibration={};",
                "validation_experimental={};",
                "validation_unknown={};",
                "serialization_format={};",
                "serialization_native={};",
                "serialization_implicit={};",
                "serialization_provenance={};",
                "cost_estimate={};",
                "cost_required={};",
                "cost_provider_pricing={};",
                "region={};account={};project={};label={};",
                "production={}"
            ),
            BACKEND_CONFIG_SCHEMA_ID,
            BACKEND_CONFIG_SCHEMA_VERSION,
            self.backend_id,
            self.provider_id,
            self.endpoint,
            self.execution_model,
            self.timeout.submission.as_millis(),
            self.timeout.execution.as_millis(),
            self.timeout.result.as_millis(),
            self.timeout.cancellation.as_millis(),
            self.timeout.control.as_millis(),
            self.retry.max_retries,
            self.retry.initial_backoff.as_millis(),
            self.retry.max_backoff.as_millis(),
            self.retry.backoff_multiplier_milli,
            self.retry.retry_submission,
            self.retry.retry_result_retrieval,
            self.retry.retry_control,
            self.queue.observe_queue,
            self.queue.wait_for_queue,
            self.queue.max_queue_wait.as_millis(),
            self.queue.priority.as_str(),
            self.queue.poll_queue,
            self.calibration.require_calibration,
            self.calibration.allow_stale,
            self.calibration.max_age.as_millis(),
            self.calibration.require_provenance,
            self.validation.validate_backend,
            self.validation.validate_capabilities,
            self.validation.validate_topology,
            self.validation.validate_timing,
            self.validation.validate_calibration,
            self.validation.allow_experimental_capabilities,
            self.validation.reject_unknown_requirements,
            self.serialization.preferred_format.as_str(),
            self.serialization.allow_provider_native,
            self.serialization.allow_implicit_conversion,
            self.serialization.require_conversion_provenance,
            self.cost.estimate_before_submission,
            self.cost.require_estimate,
            self.cost.allow_provider_pricing,
            region,
            account,
            project,
            label,
            self.production
        )
    }

    /// Returns whether the endpoint is safe for production transport.
    pub fn is_production_transport(&self) -> bool {
        self.endpoint.is_https()
            || self.endpoint.is_local()
    }

    /// Returns whether this configuration references a remote endpoint.
    pub fn is_remote(&self) -> bool {
        !self.endpoint.is_local()
    }

    /// Returns whether stale calibration is explicitly permitted.
    pub fn permits_stale_calibration(&self) -> bool {
        self.calibration.allow_stale
    }

    /// Returns the configured execution model.
    pub const fn execution_model(&self) -> ExecutionModel {
        self.execution_model
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for `BackendConfig`.
///
/// The builder contains no provider-specific behaviour. It exists to make
/// configuration construction explicit and validated.
#[derive(Debug, Default)]
pub struct BackendConfigBuilder {
    backend_id: Option<BackendReference>,
    provider_id: Option<ProviderReference>,
    endpoint: Option<EndpointReference>,
    execution_model: ExecutionModel,
    timeout: TimeoutPolicy,
    retry: RetryPolicy,
    queue: QueuePolicy,
    calibration: CalibrationPolicy,
    validation: ValidationPolicy,
    serialization: SerializationPolicy,
    cost: CostPolicy,
    region: Option<String>,
    account: Option<String>,
    project: Option<String>,
    label: Option<String>,
    production: bool,
}

impl BackendConfigBuilder {
    /// Creates a builder with strict production defaults.
    pub fn new() -> Self {
        Self {
            execution_model: ExecutionModel::GateModel,
            timeout: TimeoutPolicy::default(),
            retry: RetryPolicy::default(),
            queue: QueuePolicy::default(),
            calibration: CalibrationPolicy::strict(),
            validation: ValidationPolicy::strict(),
            serialization: SerializationPolicy::default(),
            cost: CostPolicy::default(),
            production: true,
            ..Self::default()
        }
    }

    /// Sets the backend identifier.
    pub fn backend_id(
        mut self,
        value: impl Into<String>,
    ) -> Result<Self, BackendConfigError> {
        self.backend_id =
            Some(BackendReference::new(value)?);
        Ok(self)
    }

    /// Sets the provider identifier.
    pub fn provider_id(
        mut self,
        value: impl Into<String>,
    ) -> Result<Self, BackendConfigError> {
        self.provider_id =
            Some(ProviderReference::new(value)?);
        Ok(self)
    }

    /// Sets the endpoint.
    pub fn endpoint(
        mut self,
        value: impl Into<String>,
    ) -> Result<Self, BackendConfigError> {
        self.endpoint =
            Some(EndpointReference::new(value)?);
        Ok(self)
    }

    /// Sets the execution model.
    pub fn execution_model(
        mut self,
        model: ExecutionModel,
    ) -> Self {
        self.execution_model = model;
        self
    }

    /// Sets the timeout policy.
    pub fn timeout(
        mut self,
        policy: TimeoutPolicy,
    ) -> Self {
        self.timeout = policy;
        self
    }

    /// Sets the retry policy.
    pub fn retry(
        mut self,
        policy: RetryPolicy,
    ) -> Self {
        self.retry = policy;
        self
    }

    /// Sets the queue policy.
    pub fn queue(
        mut self,
        policy: QueuePolicy,
    ) -> Self {
        self.queue = policy;
        self
    }

    /// Sets the calibration policy.
    pub fn calibration(
        mut self,
        policy: CalibrationPolicy,
    ) -> Self {
        self.calibration = policy;
        self
    }

    /// Sets the validation policy.
    pub fn validation(
        mut self,
        policy: ValidationPolicy,
    ) -> Self {
        self.validation = policy;
        self
    }

    /// Sets the serialization policy.
    pub fn serialization(
        mut self,
        policy: SerializationPolicy,
    ) -> Self {
        self.serialization = policy;
        self
    }

    /// Sets the cost policy.
    pub fn cost(
        mut self,
        policy: CostPolicy,
    ) -> Self {
        self.cost = policy;
        self
    }

    /// Sets the region.
    pub fn region(
        mut self,
        value: impl Into<String>,
    ) -> Result<Self, BackendConfigError> {
        let value = value.into();

        validate_scope_reference(
            "region",
            &value,
            MAX_REGION_LENGTH,
        )?;

        self.region = Some(value);
        Ok(self)
    }

    /// Sets the account reference.
    pub fn account(
        mut self,
        value: impl Into<String>,
    ) -> Result<Self, BackendConfigError> {
        let value = value.into();

        validate_scope_reference(
            "account",
            &value,
            MAX_SCOPE_REFERENCE_LENGTH,
        )?;

        self.account = Some(value);
        Ok(self)
    }

    /// Sets the project/workspace reference.
    pub fn project(
        mut self,
        value: impl Into<String>,
    ) -> Result<Self, BackendConfigError> {
        let value = value.into();

        validate_scope_reference(
            "project",
            &value,
            MAX_SCOPE_REFERENCE_LENGTH,
        )?;

        self.project = Some(value);
        Ok(self)
    }

    /// Sets a human-readable label.
    pub fn label(
        mut self,
        value: impl Into<String>,
    ) -> Result<Self, BackendConfigError> {
        let value = value.into();

        validate_scope_reference(
            "label",
            &value,
            MAX_LABEL_LENGTH,
        )?;

        self.label = Some(value);
        Ok(self)
    }

    /// Marks the configuration as production.
    pub fn production(
        mut self,
        value: bool,
    ) -> Self {
        self.production = value;
        self
    }

    /// Builds and validates the configuration.
    pub fn build(self) -> Result<BackendConfig, BackendConfigError> {
        let backend_id = self.backend_id.ok_or(
            BackendConfigError::MissingField {
                field: "backend_id",
            },
        )?;

        let provider_id = self.provider_id.ok_or(
            BackendConfigError::MissingField {
                field: "provider_id",
            },
        )?;

        let endpoint = self.endpoint.ok_or(
            BackendConfigError::MissingField {
                field: "endpoint",
            },
        )?;

        let config = BackendConfig {
            backend_id,
            provider_id,
            endpoint,
            execution_model: self.execution_model,
            timeout: self.timeout,
            retry: self.retry,
            queue: self.queue,
            calibration: self.calibration,
            validation: self.validation,
            serialization: self.serialization,
            cost: self.cost,
            region: self.region,
            account: self.account,
            project: self.project,
            label: self.label,
            production: self.production,
        };

        config.validate()?;

        Ok(config)
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while creating or validating backend configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackendConfigError {
    /// Required configuration field is missing.
    MissingField {
        /// Missing field name.
        field: &'static str,
    },

    /// Configuration field is empty.
    EmptyValue {
        /// Field name.
        field: &'static str,
    },

    /// Configuration value is too long.
    TooLong {
        /// Field name.
        field: &'static str,

        /// Maximum allowed length.
        max: usize,
    },

    /// Configuration value has invalid syntax.
    InvalidValue {
        /// Field name.
        field: &'static str,

        /// Explanation.
        message: String,
    },

    /// Configuration value exceeds a numeric bound.
    OutOfRange {
        /// Field name.
        field: &'static str,

        /// Explanation.
        message: String,
    },

    /// Endpoint contains credentials.
    CredentialMaterialInEndpoint,

    /// Endpoint contains a credential-bearing URL component.
    CredentialBearingUrl,

    /// Remote endpoint uses insecure HTTP.
    InsecureRemoteEndpoint,

    /// Production policy was violated.
    ProductionPolicyViolation {
        /// Field violating policy.
        field: &'static str,

        /// Explanation.
        message: String,
    },
}

impl fmt::Display for BackendConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField { field } => {
                write!(
                    formatter,
                    "missing required backend configuration field `{field}`"
                )
            }

            Self::EmptyValue { field } => {
                write!(
                    formatter,
                    "backend configuration field `{field}` cannot be empty"
                )
            }

            Self::TooLong { field, max } => {
                write!(
                    formatter,
                    "backend configuration field `{field}` exceeds maximum length {max}"
                )
            }

            Self::InvalidValue {
                field,
                message,
            } => {
                write!(
                    formatter,
                    "invalid backend configuration field `{field}`: {message}"
                )
            }

            Self::OutOfRange {
                field,
                message,
            } => {
                write!(
                    formatter,
                    "backend configuration field `{field}` is out of range: {message}"
                )
            }

            Self::CredentialMaterialInEndpoint => {
                formatter.write_str(
                    "backend endpoint contains forbidden credential material"
                )
            }

            Self::CredentialBearingUrl => {
                formatter.write_str(
                    "backend endpoint contains a credential-bearing URL component"
                )
            }

            Self::InsecureRemoteEndpoint => {
                formatter.write_str(
                    "production backend configuration requires HTTPS for remote endpoints"
                )
            }

            Self::ProductionPolicyViolation {
                field,
                message,
            } => {
                write!(
                    formatter,
                    "production backend policy violation in `{field}`: {message}"
                )
            }
        }
    }
}

impl Error for BackendConfigError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn max_timeout() -> Duration {
    Duration::from_secs(MAX_TIMEOUT_SECONDS)
}

fn max_retry_backoff() -> Duration {
    Duration::from_secs(MAX_RETRY_BACKOFF_SECONDS)
}

fn max_queue_wait() -> Duration {
    Duration::from_secs(MAX_QUEUE_WAIT_SECONDS)
}

fn validate_duration(
    field: &'static str,
    value: Duration,
    minimum: Duration,
    maximum: Duration,
) -> Result<(), BackendConfigError> {
    if value < minimum {
        return Err(BackendConfigError::OutOfRange {
            field,
            message: format!(
                "minimum is {} milliseconds",
                minimum.as_millis()
            ),
        });
    }

    if value > maximum {
        return Err(BackendConfigError::OutOfRange {
            field,
            message: format!(
                "maximum is {} seconds",
                maximum.as_secs()
            ),
        });
    }

    Ok(())
}

fn validate_identifier(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), BackendConfigError> {
    if value.is_empty() {
        return Err(BackendConfigError::EmptyValue { field });
    }

    if value.len() > maximum {
        return Err(BackendConfigError::TooLong {
            field,
            max: maximum,
        });
    }

    if value.trim() != value {
        return Err(BackendConfigError::InvalidValue {
            field,
            message: "leading or trailing whitespace is forbidden"
                .to_string(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(BackendConfigError::InvalidValue {
            field,
            message: "control characters are forbidden".to_string(),
        });
    }

    if contains_secret_marker(value) {
        return Err(BackendConfigError::InvalidValue {
            field,
            message:
                "credential or secret material is forbidden".to_string(),
        });
    }

    Ok(())
}

fn validate_scope_reference(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), BackendConfigError> {
    if value.is_empty() {
        return Err(BackendConfigError::EmptyValue { field });
    }

    if value.len() > maximum {
        return Err(BackendConfigError::TooLong {
            field,
            max: maximum,
        });
    }

    if value.trim() != value {
        return Err(BackendConfigError::InvalidValue {
            field,
            message: "leading or trailing whitespace is forbidden"
                .to_string(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(BackendConfigError::InvalidValue {
            field,
            message: "control characters are forbidden".to_string(),
        });
    }

    if contains_secret_marker(value) {
        return Err(BackendConfigError::InvalidValue {
            field,
            message:
                "credential or secret material is forbidden".to_string(),
        });
    }

    Ok(())
}

fn validate_endpoint_reference(
    value: &str,
) -> Result<(), BackendConfigError> {
    if value.is_empty() {
        return Err(BackendConfigError::EmptyValue {
            field: "endpoint",
        });
    }

    if value.len() > MAX_ENDPOINT_REFERENCE_LENGTH {
        return Err(BackendConfigError::TooLong {
            field: "endpoint",
            max: MAX_ENDPOINT_REFERENCE_LENGTH,
        });
    }

    if value.trim() != value {
        return Err(BackendConfigError::InvalidValue {
            field: "endpoint",
            message:
                "leading or trailing whitespace is forbidden".to_string(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(BackendConfigError::InvalidValue {
            field: "endpoint",
            message: "control characters are forbidden".to_string(),
        });
    }

    if contains_secret_marker(value) {
        return Err(BackendConfigError::CredentialMaterialInEndpoint);
    }

    let lower = value.to_ascii_lowercase();

    if lower.starts_with("https://")
        || lower.starts_with("http://")
    {
        validate_http_endpoint(value)?;
    } else if lower.starts_with("local://")
        || lower.starts_with("unix://")
        || lower.starts_with("provider://")
        || lower.starts_with("env://")
        || lower.starts_with("config://")
    {
        validate_reference_endpoint(value)?;
    } else {
        return Err(BackendConfigError::InvalidValue {
            field: "endpoint",
            message:
                "endpoint must use https://, http://, local://, unix://, provider://, env://, or config://"
                    .to_string(),
        });
    }

    Ok(())
}

fn validate_http_endpoint(
    value: &str,
) -> Result<(), BackendConfigError> {
    let lower = value.to_ascii_lowercase();

    if lower.contains("@") {
        return Err(BackendConfigError::CredentialBearingUrl);
    }

    if lower.contains("authorization=")
        || lower.contains("access_token=")
        || lower.contains("api_key=")
        || lower.contains("apikey=")
        || lower.contains("token=")
        || lower.contains("password=")
        || lower.contains("secret=")
        || lower.contains("private_key=")
    {
        return Err(BackendConfigError::CredentialBearingUrl);
    }

    let after_scheme = value
        .split_once("://")
        .map(|(_, rest)| rest)
        .unwrap_or("");

    if after_scheme.is_empty() {
        return Err(BackendConfigError::InvalidValue {
            field: "endpoint",
            message: "HTTP endpoint must contain a host".to_string(),
        });
    }

    let authority = after_scheme
        .split('/')
        .next()
        .unwrap_or("");

    if authority.is_empty() {
        return Err(BackendConfigError::InvalidValue {
            field: "endpoint",
            message: "HTTP endpoint must contain a host".to_string(),
        });
    }

    if authority.contains('?')
        || authority.contains('#')
    {
        return Err(BackendConfigError::InvalidValue {
            field: "endpoint",
            message:
                "HTTP endpoint authority contains invalid delimiters"
                    .to_string(),
        });
    }

    Ok(())
}

fn validate_reference_endpoint(
    value: &str,
) -> Result<(), BackendConfigError> {
    let (_, rest) = value
        .split_once("://")
        .ok_or_else(|| BackendConfigError::InvalidValue {
            field: "endpoint",
            message: "reference endpoint must contain ://".to_string(),
        })?;

    if rest.is_empty() {
        return Err(BackendConfigError::InvalidValue {
            field: "endpoint",
            message:
                "reference endpoint must contain a non-empty target"
                    .to_string(),
        });
    }

    Ok(())
}

fn contains_secret_marker(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();

    const MARKERS: &[&str] = &[
        "api_key",
        "apikey",
        "api-key",
        "access_token",
        "accesstoken",
        "access-token",
        "bearer ",
        "authorization:",
        "authorization=",
        "password=",
        "passwd=",
        "secret=",
        "private_key",
        "private-key",
        "-----begin private key-----",
        "-----begin rsa private key-----",
        "-----begin openssh private key-----",
        "cookie=",
        "session=",
    ];

    MARKERS.iter().any(|marker| lower.contains(marker))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn backend_id() -> BackendReference {
        BackendReference::new("test-backend")
            .expect("test backend ID must be valid")
    }

    fn provider_id() -> ProviderReference {
        ProviderReference::new("test-provider")
            .expect("test provider ID must be valid")
    }

    fn https_endpoint() -> EndpointReference {
        EndpointReference::new("https://quantum.example.invalid/api")
            .expect("test endpoint must be valid")
    }

    fn production_config() -> BackendConfig {
        BackendConfig::new(
            backend_id(),
            provider_id(),
            https_endpoint(),
        )
        .expect("production config must be valid")
    }

    #[test]
    fn production_defaults_are_valid() {
        let config = production_config();

        assert!(config.production);
        assert!(config.validation.validate_backend);
        assert!(config.validation.validate_capabilities);
        assert!(config.validation.validate_topology);
        assert!(config.validation.validate_timing);
        assert!(config.validation.validate_calibration);
        assert!(!config.validation.allow_experimental_capabilities);
        assert!(!config.calibration.allow_stale);
        assert!(config.calibration.require_provenance);
        assert!(config.is_production_transport());
    }

    #[test]
    fn backend_reference_rejects_empty() {
        let result = BackendReference::new("");

        assert!(matches!(
            result,
            Err(BackendConfigError::EmptyValue {
                field: "backend_id"
            })
        ));
    }

    #[test]
    fn provider_reference_rejects_whitespace() {
        let result = ProviderReference::new(" provider ");

        assert!(matches!(
            result,
            Err(BackendConfigError::InvalidValue {
                field: "provider_id",
                ..
            })
        ));
    }

    #[test]
    fn endpoint_requires_supported_scheme() {
        let result =
            EndpointReference::new("ftp://example.invalid");

        assert!(matches!(
            result,
            Err(BackendConfigError::InvalidValue {
                field: "endpoint",
                ..
            })
        ));
    }

    #[test]
    fn endpoint_rejects_embedded_credentials() {
        let result =
            EndpointReference::new(
                "https://user:password@example.invalid/api",
            );

        assert_eq!(
            result,
            Err(BackendConfigError::CredentialBearingUrl)
        );
    }

    #[test]
    fn endpoint_rejects_api_key_query_parameter() {
        let result =
            EndpointReference::new(
                "https://example.invalid/api?api_key=secret",
            );

        assert_eq!(
            result,
            Err(BackendConfigError::CredentialMaterialInEndpoint)
        );
    }

    #[test]
    fn endpoint_rejects_bearer_tokens() {
        let result =
            EndpointReference::new(
                "provider://Authorization: Bearer abc",
            );

        assert_eq!(
            result,
            Err(BackendConfigError::CredentialMaterialInEndpoint)
        );
    }

    #[test]
    fn local_endpoint_is_supported() {
        let endpoint =
            EndpointReference::new("local://statevector");

        assert!(endpoint.is_ok());

        let endpoint = endpoint.expect("local endpoint must be valid");

        assert!(endpoint.is_local());
        assert!(!endpoint.is_https());
    }

    #[test]
    fn production_http_endpoint_is_rejected() {
        let mut config = production_config();

        config.endpoint =
            EndpointReference::new(
                "http://quantum.example.invalid/api",
            )
            .expect("HTTP endpoint itself is syntactically valid");

        assert_eq!(
            config.validate(),
            Err(BackendConfigError::InsecureRemoteEndpoint)
        );
    }

    #[test]
    fn local_http_like_development_is_not_implicitly_allowed() {
        let mut config = production_config();

        config.production = false;
        config.endpoint =
            EndpointReference::new(
                "http://localhost:8080",
            )
            .expect("HTTP endpoint should be syntactically valid");

        assert!(config.validate().is_ok());
    }

    #[test]
    fn timeout_policy_rejects_zero_submission_timeout() {
        let policy = TimeoutPolicy {
            submission: Duration::from_secs(0),
            ..TimeoutPolicy::default()
        };

        assert!(policy.validate().is_err());
    }

    #[test]
    fn timeout_policy_rejects_excessive_timeout() {
        let policy = TimeoutPolicy {
            execution: Duration::from_secs(
                MAX_TIMEOUT_SECONDS + 1,
            ),
            ..TimeoutPolicy::default()
        };

        assert!(policy.validate().is_err());
    }

    #[test]
    fn retry_policy_disabled_is_valid() {
        assert!(RetryPolicy::disabled().validate().is_ok());
    }

    #[test]
    fn retry_policy_rejects_zero_multiplier() {
        let policy = RetryPolicy {
            backoff_multiplier_milli: 0,
            ..RetryPolicy::default()
        };

        assert!(policy.validate().is_err());
    }

    #[test]
    fn retry_backoff_is_deterministic() {
        let policy = RetryPolicy {
            max_retries: 10,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(10),
            backoff_multiplier_milli: 2_000,
            ..RetryPolicy::default()
        };

        assert_eq!(
            policy.backoff_for_retry(1),
            Duration::from_millis(100)
        );

        assert_eq!(
            policy.backoff_for_retry(2),
            Duration::from_millis(200)
        );

        assert_eq!(
            policy.backoff_for_retry(3),
            Duration::from_millis(400)
        );

        assert_eq!(
            policy.backoff_for_retry(4),
            Duration::from_millis(800)
        );
    }

    #[test]
    fn retry_backoff_is_capped() {
        let policy = RetryPolicy {
            max_retries: 10,
            initial_backoff: Duration::from_secs(8),
            max_backoff: Duration::from_secs(10),
            backoff_multiplier_milli: 2_000,
            ..RetryPolicy::default()
        };

        assert_eq!(
            policy.backoff_for_retry(2),
            Duration::from_secs(10)
        );

        assert_eq!(
            policy.backoff_for_retry(10),
            Duration::from_secs(10)
        );
    }

    #[test]
    fn queue_policy_accepts_zero_wait() {
        let policy = QueuePolicy {
            max_queue_wait: Duration::from_secs(0),
            ..QueuePolicy::default()
        };

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn queue_policy_rejects_excessive_wait() {
        let policy = QueuePolicy {
            max_queue_wait: Duration::from_secs(
                MAX_QUEUE_WAIT_SECONDS + 1,
            ),
            ..QueuePolicy::default()
        };

        assert!(policy.validate().is_err());
    }

    #[test]
    fn strict_calibration_rejects_stale_configuration() {
        let policy = CalibrationPolicy {
            allow_stale: true,
            ..CalibrationPolicy::strict()
        };

        assert!(policy.validate().is_err());
    }

    #[test]
    fn local_configuration_disables_hardware_calibration_requirement() {
        let config = BackendConfig::local(
            BackendReference::new("local-simulator")
                .expect("backend ID must be valid"),
            EndpointReference::new("local://statevector")
                .expect("local endpoint must be valid"),
        )
        .expect("local configuration must be valid");

        assert!(!config.production);
        assert!(!config.calibration.require_calibration);
        assert!(!config.validation.validate_calibration);
    }

    #[test]
    fn builder_requires_backend_id() {
        let result = BackendConfigBuilder::new()
            .provider_id("provider")
            .expect("provider must be valid")
            .endpoint("https://example.invalid")
            .expect("endpoint must be valid")
            .build();

        assert_eq!(
            result,
            Err(BackendConfigError::MissingField {
                field: "backend_id"
            })
        );
    }

    #[test]
    fn builder_requires_provider_id() {
        let result = BackendConfigBuilder::new()
            .backend_id("backend")
            .expect("backend must be valid")
            .endpoint("https://example.invalid")
            .expect("endpoint must be valid")
            .build();

        assert_eq!(
            result,
            Err(BackendConfigError::MissingField {
                field: "provider_id"
            })
        );
    }

    #[test]
    fn builder_requires_endpoint() {
        let result = BackendConfigBuilder::new()
            .backend_id("backend")
            .expect("backend must be valid")
            .provider_id("provider")
            .expect("provider must be valid")
            .build();

        assert_eq!(
            result,
            Err(BackendConfigError::MissingField {
                field: "endpoint"
            })
        );
    }

    #[test]
    fn builder_creates_valid_configuration() {
        let config = BackendConfigBuilder::new()
            .backend_id("backend")
            .expect("backend must be valid")
            .provider_id("provider")
            .expect("provider must be valid")
            .endpoint("https://example.invalid/api")
            .expect("endpoint must be valid")
            .execution_model(ExecutionModel::DynamicCircuit)
            .region("us-east")
            .expect("region must be valid")
            .project("project-a")
            .expect("project must be valid")
            .build()
            .expect("configuration must be valid");

        assert_eq!(
            config.execution_model,
            ExecutionModel::DynamicCircuit
        );

        assert_eq!(
            config.region.as_deref(),
            Some("us-east")
        );

        assert_eq!(
            config.project.as_deref(),
            Some("project-a")
        );
    }

    #[test]
    fn production_configuration_rejects_disabled_backend_validation() {
        let mut config = production_config();

        config.validation.validate_backend = false;

        assert!(matches!(
            config.validate(),
            Err(
                BackendConfigError::ProductionPolicyViolation {
                    field: "validation.validate_backend",
                    ..
                }
            )
        ));
    }

    #[test]
    fn production_configuration_rejects_disabled_capability_validation() {
        let mut config = production_config();

        config.validation.validate_capabilities = false;

        assert!(matches!(
            config.validate(),
            Err(
                BackendConfigError::ProductionPolicyViolation {
                    field: "validation.validate_capabilities",
                    ..
                }
            )
        ));
    }

    #[test]
    fn production_configuration_rejects_experimental_capabilities() {
        let mut config = production_config();

        config.validation.allow_experimental_capabilities = true;

        assert!(matches!(
            config.validate(),
            Err(
                BackendConfigError::ProductionPolicyViolation {
                    field: "validation.allow_experimental_capabilities",
                    ..
                }
            )
        ));
    }

    #[test]
    fn canonical_key_is_deterministic() {
        let first = production_config().canonical_key();
        let second = production_config().canonical_key();

        assert_eq!(first, second);
    }

    #[test]
    fn canonical_key_changes_when_configuration_changes() {
        let first = production_config().canonical_key();

        let mut second_config = production_config();
        second_config.execution_model =
            ExecutionModel::DynamicCircuit;

        let second = second_config.canonical_key();

        assert_ne!(first, second);
    }

    #[test]
    fn serialization_formats_have_stable_identifiers() {
        assert_eq!(
            SerializationFormat::OpenQasm3.as_str(),
            "openqasm3"
        );

        assert_eq!(
            SerializationFormat::Qir.as_str(),
            "qir"
        );
    }

    #[test]
    fn execution_models_have_stable_identifiers() {
        assert_eq!(
            ExecutionModel::GateModel.as_str(),
            "gate_model"
        );

        assert_eq!(
            ExecutionModel::DynamicCircuit.as_str(),
            "dynamic_circuit"
        );

        assert_eq!(
            ExecutionModel::Analog.as_str(),
            "analog"
        );

        assert_eq!(
            ExecutionModel::Annealing.as_str(),
            "annealing"
        );

        assert_eq!(
            ExecutionModel::Logical.as_str(),
            "logical"
        );
    }

    #[test]
    fn configuration_does_not_store_credential_fields() {
        let config = production_config();

        let debug = format!("{config:?}");

        assert!(!debug.contains("api_key"));
        assert!(!debug.contains("access_token"));
        assert!(!debug.contains("password"));
        assert!(!debug.contains("private_key"));
    }

    #[test]
    fn secret_markers_are_rejected_in_scope_references() {
        assert!(BackendReference::new(
            "backend?api_key=secret"
        )
        .is_err());

        assert!(ProviderReference::new(
            "provider?access_token=secret"
        )
        .is_err());
    }

    #[test]
    fn configuration_schema_is_stable() {
        assert_eq!(
            BACKEND_CONFIG_SCHEMA_ID,
            "zamani.quantum.hardware.backend_config"
        );

        assert_eq!(
            BACKEND_CONFIG_SCHEMA_VERSION,
            1
        );
    }
}