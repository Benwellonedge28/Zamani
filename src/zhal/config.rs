
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## ZHAL Configuration
//!
//! Path: `src/zhal/config.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## 1. PURPOSE
//!
//! Defines immutable, validated, extensible configuration for the
//! Zamani Universal Hardware Abstraction Layer.
//!
//! ZHAL configuration controls how the hardware abstraction layer
//! participates in hardware discovery, provider initialization,
//! device observation, health observation, hardware access and
//! operational diagnostics.
//!
//! Configuration describes policy.
//!
//! It does not perform hardware operations.
//!
//! ## 2. ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - ZhalConfig.
//! - ZhalConfigBuilder.
//! - DiscoveryPolicy.
//! - ProviderInitializationPolicy.
//! - ObservationPolicy.
//! - HealthPolicy.
//! - TimeoutPolicy.
//! - RetryPolicy.
//! - ConcurrencyPolicy.
//! - DiagnosticPolicy.
//! - SecurityPolicyReferences.
//! - OptionalFeaturePolicy.
//! - PolicyReference.
//! - Configuration validation.
//! - Configuration defaults.
//!
//! This module does not own:
//!
//! - Provider identity or lifecycle.
//! - Capability identity or evidence.
//! - Device identity.
//! - Device inventory.
//! - Hardware discovery implementation.
//! - Resource allocation.
//! - Resource reservation.
//! - Runtime scheduling.
//! - Execution.
//! - Provider authentication.
//! - Authorization.
//! - Security enforcement.
//! - Quantum routing.
//! - HDL realization.
//! - Distributed execution.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ## 3. EXISTING REPOSITORY AUTHORITIES
//!
//! Standard-library provider contracts:
//!     crate::stdlib::provider
//!
//! Standard-library capabilities:
//!     crate::stdlib::capability
//!
//! Standard-library operation context:
//!     crate::stdlib::context
//!
//! Standard-library device I/O:
//!     crate::stdlib::io::devices
//!
//! Runtime configuration and budgets:
//!     crate::zur::config
//!
//! Runtime context:
//!     crate::zur::core::context
//!
//! Runtime security:
//!     crate::zur::security
//!
//! Quantum hardware:
//!     crate::quantum::hardware
//!
//! HDL realization:
//!     crate::hdl::realization
//!
//! Distributed execution:
//!     crate::distributed
//!
//! Hybrid execution:
//!     crate::hybrid
//!
//! ## 4. DEPENDENCY POLICY
//!
//! This file deliberately depends only on the Rust standard library.
//!
//! It must not depend on ZUR configuration because ZUR will consume
//! ZHAL integration contracts. This prevents a potential circular
//! dependency.
//!
//! Runtime budget policies are referenced through opaque identifiers.
//! ZUR remains responsible for resolving and enforcing those policies.
//!
//! ## 5. POCO-REAF
//!
//! No universal hardware ceilings are introduced.
//!
//! In particular, this module does not define:
//!
//! - MAX_CPUS.
//! - MAX_GPUS.
//! - MAX_QPUS.
//! - MAX_FPGAS.
//! - MAX_DEVICES.
//! - MAX_PROVIDERS.
//! - MAX_NODES.
//! - MAX_MEMORY.
//! - MAX_THREADS.
//! - MAX_QUbits.
//! - MAX_REGISTER_WIDTH.
//!
//! Optional operational limits are explicit configuration choices.
//! They are not language or hardware capability restrictions.
//!
//! An absent operational limit means that ZHAL itself imposes no
//! additional limit for that configured dimension.
//!
//! Actual operations remain subject to available resources,
//! provider capabilities, operating-system constraints and
//! externally owned runtime policies.
//!
//! Literal infinite physical computation is not promised.
//!
//! ## 6. DETERMINISM
//!
//! Configuration collections use BTreeMap.
//!
//! Configuration construction performs no:
//!
//! - Hardware discovery.
//! - Filesystem access.
//! - Network access.
//! - Environment-variable access.
//! - Clock access.
//! - Random-number generation.
//! - Background-thread creation.
//! - Global initialization.
//!
//! ## 7. THREAD SAFETY
//!
//! Configuration contains immutable, owned values.
//!
//! It is safe to share between threads.
//!
//! Mutable operational state belongs to the subsystem enforcing
//! the corresponding policy.
//!
//! ## 8. SECURITY
//!
//! Configuration may reference external security policies.
//!
//! It must never contain:
//!
//! - Passwords.
//! - Private keys.
//! - Access tokens.
//! - Bearer tokens.
//! - Authentication secrets.
//! - Credentials.
//! - Unrestricted secret environment values.
//!
//! Configuration references do not establish authorization.
//!
//! ## 9. ERROR HANDLING
//!
//! Invalid configuration returns ConfigError.
//!
//! No ordinary invalid input causes a panic.
//!
//! Validation does not perform hardware feasibility assessment.

use core::fmt;
use std::collections::BTreeMap;
use std::num::NonZeroUsize;
use std::time::Duration;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable architectural identifier for the ZHAL configuration module.
pub const ZHAL_CONFIG_MODULE_NAME: &str = "zamani.zhal.config";

/// Version of the configuration contract.
///
/// Independent of the Zamani package version, language version,
/// grammar version, hardware-provider API version and firmware version.
pub const ZHAL_CONFIG_SCHEMA_VERSION: u16 = 1;

/// Returns the configuration module identity.
#[must_use]
pub const fn module_name() -> &'static str {
    ZHAL_CONFIG_MODULE_NAME
}

/// Returns the configuration schema version.
#[must_use]
pub const fn schema_version() -> u16 {
    ZHAL_CONFIG_SCHEMA_VERSION
}

// ============================================================================
// CONFIGURATION ERRORS
// ============================================================================

/// Categories of configuration validation failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ConfigErrorKind {
    /// A required configuration value is invalid.
    InvalidValue,

    /// A configuration identifier is invalid.
    InvalidIdentifier,

    /// Two configuration settings contradict each other.
    ContradictorySettings,

    /// A required configuration dependency is absent.
    MissingDependency,

    /// A configuration value violates its contract.
    InvalidContract,
}

/// Structured configuration validation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError {
    kind: ConfigErrorKind,
    field: String,
    message: String,
}

impl ConfigError {
    fn new(
        kind: ConfigErrorKind,
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            field: field.into(),
            message: message.into(),
        }
    }

    /// Returns the error category.
    #[must_use]
    pub const fn kind(&self) -> ConfigErrorKind {
        self.kind
    }

    /// Returns the configuration field associated with the error.
    #[must_use]
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Returns the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ConfigError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "ZHAL configuration error in '{}': {}",
            self.field,
            self.message
        )
    }
}

impl std::error::Error for ConfigError {}

/// Result type used by ZHAL configuration validation.
pub type ConfigResult<T> = Result<T, ConfigError>;

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    field: &str,
) -> ConfigResult<()> {
    if value.is_empty() {
        return Err(ConfigError::new(
            ConfigErrorKind::InvalidIdentifier,
            field,
            "identifier must not be empty",
        ));
    }

    if value.trim() != value {
        return Err(ConfigError::new(
            ConfigErrorKind::InvalidIdentifier,
            field,
            "identifier must not contain surrounding whitespace",
        ));
    }

    let mut segments = value.split('.');
    let mut count = 0usize;

    for segment in &mut segments {
        if segment.is_empty() {
            return Err(ConfigError::new(
                ConfigErrorKind::InvalidIdentifier,
                field,
                "identifier contains an empty namespace segment",
            ));
        }

        if !segment.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'_'
                || byte == b'-'
        }) {
            return Err(ConfigError::new(
                ConfigErrorKind::InvalidIdentifier,
                field,
                "identifier contains an unsupported character",
            ));
        }

        if segment.starts_with(['-', '_'])
            || segment.ends_with(['-', '_'])
        {
            return Err(ConfigError::new(
                ConfigErrorKind::InvalidIdentifier,
                field,
                "identifier has an invalid segment boundary",
            ));
        }

        count += 1;
    }

    if count < 2 {
        return Err(ConfigError::new(
            ConfigErrorKind::InvalidIdentifier,
            field,
            "identifier must contain a namespace and a name",
        ));
    }

    Ok(())
}

// ============================================================================
// EXTERNAL POLICY REFERENCES
// ============================================================================

/// An opaque reference to a policy owned by another subsystem.
///
/// ZHAL stores the reference but does not implement, evaluate or
/// enforce the referenced policy.
///
/// Examples:
///
/// - `zur.resource_budget`
/// - `zur.hardware_access`
/// - `zur.provider_trust`
/// - `security.device_isolation`
/// - `vendor.extension.policy`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyReference(String);

impl PolicyReference {
    /// Constructs a validated policy reference.
    pub fn new(
        value: impl Into<String>,
    ) -> ConfigResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "policy_reference",
        )?;

        Ok(Self(value))
    }

    /// Returns the canonical policy reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the reference and returns its identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for PolicyReference {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// DISCOVERY POLICY
// ============================================================================

/// Controls when hardware discovery may be initiated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum DiscoveryActivation {
    /// Discovery occurs only when explicitly requested.
    ///
    /// This is the default.
    #[default]
    ExplicitOnly,

    /// Discovery may be initiated when a consumer requires it.
    OnDemand,

    /// Discovery may run periodically when an owning runtime
    /// or service manager explicitly provides the mechanism.
    Periodic,
}

/// Controls behavior when individual discovery providers fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum DiscoveryFailureMode {
    /// Preserve successful observations while reporting failures.
    ///
    /// This is the default.
    #[default]
    BestEffort,

    /// Fail the discovery operation when a required provider fails.
    FailFast,
}

/// Policy controlling hardware discovery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveryPolicy {
    activation: DiscoveryActivation,
    failure_mode: DiscoveryFailureMode,
    refresh_interval: Option<Duration>,
    maximum_results_per_request: Option<NonZeroUsize>,
    maximum_concurrent_providers: Option<NonZeroUsize>,
}

impl Default for DiscoveryPolicy {
    fn default() -> Self {
        Self {
            activation: DiscoveryActivation::ExplicitOnly,
            failure_mode: DiscoveryFailureMode::BestEffort,
            refresh_interval: None,
            maximum_results_per_request: None,
            maximum_concurrent_providers: None,
        }
    }
}

impl DiscoveryPolicy {
    /// Returns the discovery activation policy.
    #[must_use]
    pub const fn activation(&self) -> DiscoveryActivation {
        self.activation
    }

    /// Returns the provider failure policy.
    #[must_use]
    pub const fn failure_mode(&self) -> DiscoveryFailureMode {
        self.failure_mode
    }

    /// Returns the optional refresh interval.
    #[must_use]
    pub const fn refresh_interval(&self) -> Option<Duration> {
        self.refresh_interval
    }

    /// Returns the optional result budget per request.
    ///
    /// This is an operational limit, not a universal device limit.
    #[must_use]
    pub const fn maximum_results_per_request(
        &self,
    ) -> Option<NonZeroUsize> {
        self.maximum_results_per_request
    }

    /// Returns the optional concurrent-provider budget.
    #[must_use]
    pub const fn maximum_concurrent_providers(
        &self,
    ) -> Option<NonZeroUsize> {
        self.maximum_concurrent_providers
    }

    /// Sets discovery activation.
    #[must_use]
    pub fn with_activation(
        mut self,
        activation: DiscoveryActivation,
    ) -> Self {
        self.activation = activation;
        self
    }

    /// Sets discovery failure behavior.
    #[must_use]
    pub fn with_failure_mode(
        mut self,
        mode: DiscoveryFailureMode,
    ) -> Self {
        self.failure_mode = mode;
        self
    }

    /// Sets the periodic refresh interval.
    #[must_use]
    pub fn with_refresh_interval(
        mut self,
        interval: Option<Duration>,
    ) -> Self {
        self.refresh_interval = interval;
        self
    }

    /// Sets an optional result budget for an individual request.
    #[must_use]
    pub fn with_maximum_results_per_request(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.maximum_results_per_request = maximum;
        self
    }

    /// Sets an optional concurrent-provider budget.
    #[must_use]
    pub fn with_maximum_concurrent_providers(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.maximum_concurrent_providers = maximum;
        self
    }

    fn validate(&self) -> ConfigResult<()> {
        if self.activation == DiscoveryActivation::Periodic {
            match self.refresh_interval {
                Some(interval) if !interval.is_zero() => {}
                _ => {
                    return Err(ConfigError::new(
                        ConfigErrorKind::MissingDependency,
                        "discovery.refresh_interval",
                        "periodic discovery requires a nonzero refresh interval",
                    ));
                }
            }
        }

        if let Some(interval) = self.refresh_interval {
            if interval.is_zero() {
                return Err(ConfigError::new(
                    ConfigErrorKind::InvalidValue,
                    "discovery.refresh_interval",
                    "periodic refresh interval must be nonzero",
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// PROVIDER INITIALIZATION POLICY
// ============================================================================

/// Controls provider initialization behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum ProviderInitializationMode {
    /// Initialize providers only through explicit lifecycle requests.
    ///
    /// This is the default.
    #[default]
    Explicit,

    /// Initialize a provider when a validated operation requires it.
    OnDemand,

    /// Permit initialization during an explicitly initiated
    /// ZHAL or runtime startup procedure.
    Eager,
}

/// Controls provider initialization behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderInitializationPolicy {
    mode: ProviderInitializationMode,
    maximum_concurrent_initializations: Option<NonZeroUsize>,
    rollback_on_failure: bool,
}

impl Default for ProviderInitializationPolicy {
    fn default() -> Self {
        Self {
            mode: ProviderInitializationMode::Explicit,
            maximum_concurrent_initializations: None,
            rollback_on_failure: true,
        }
    }
}

impl ProviderInitializationPolicy {
    /// Returns the initialization mode.
    #[must_use]
    pub const fn mode(&self) -> ProviderInitializationMode {
        self.mode
    }

    /// Returns the optional concurrent initialization budget.
    #[must_use]
    pub const fn maximum_concurrent_initializations(
        &self,
    ) -> Option<NonZeroUsize> {
        self.maximum_concurrent_initializations
    }

    /// Returns whether partial initialization should be rolled back.
    #[must_use]
    pub const fn rollback_on_failure(&self) -> bool {
        self.rollback_on_failure
    }

    /// Sets the initialization mode.
    #[must_use]
    pub fn with_mode(
        mut self,
        mode: ProviderInitializationMode,
    ) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the concurrent initialization budget.
    #[must_use]
    pub fn with_maximum_concurrent_initializations(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.maximum_concurrent_initializations = maximum;
        self
    }

    /// Sets initialization rollback behavior.
    #[must_use]
    pub fn with_rollback_on_failure(
        mut self,
        enabled: bool,
    ) -> Self {
        self.rollback_on_failure = enabled;
        self
    }
}

// ============================================================================
// DEVICE OBSERVATION POLICY
// ============================================================================

/// Controls the treatment of stale hardware observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum StaleObservationPolicy {
    /// Preserve stale observations but identify them as stale.
    ///
    /// They must not be treated as fresh evidence.
    ///
    /// This is the default.
    #[default]
    ExposeAsStale,

    /// Reject stale observations when a consumer requires fresh evidence.
    RejectForFreshnessDependentDecisions,
}

/// Controls hardware observation freshness and refresh behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationPolicy {
    maximum_age: Option<Duration>,
    refresh_interval: Option<Duration>,
    stale_policy: StaleObservationPolicy,
}

impl Default for ObservationPolicy {
    fn default() -> Self {
        Self {
            maximum_age: None,
            refresh_interval: None,
            stale_policy: StaleObservationPolicy::ExposeAsStale,
        }
    }
}

impl ObservationPolicy {
    /// Returns the maximum acceptable observation age.
    #[must_use]
    pub const fn maximum_age(&self) -> Option<Duration> {
        self.maximum_age
    }

    /// Returns the optional observation refresh interval.
    #[must_use]
    pub const fn refresh_interval(&self) -> Option<Duration> {
        self.refresh_interval
    }

    /// Returns the stale observation policy.
    #[must_use]
    pub const fn stale_policy(&self) -> StaleObservationPolicy {
        self.stale_policy
    }

    /// Sets the maximum observation age.
    #[must_use]
    pub fn with_maximum_age(
        mut self,
        maximum_age: Option<Duration>,
    ) -> Self {
        self.maximum_age = maximum_age;
        self
    }

    /// Sets the observation refresh interval.
    #[must_use]
    pub fn with_refresh_interval(
        mut self,
        interval: Option<Duration>,
    ) -> Self {
        self.refresh_interval = interval;
        self
    }

    /// Sets the stale observation policy.
    #[must_use]
    pub fn with_stale_policy(
        mut self,
        policy: StaleObservationPolicy,
    ) -> Self {
        self.stale_policy = policy;
        self
    }

    fn validate(&self) -> ConfigResult<()> {
        if let Some(interval) = self.refresh_interval {
            if interval.is_zero() {
                return Err(ConfigError::new(
                    ConfigErrorKind::InvalidValue,
                    "observation.refresh_interval",
                    "refresh interval must be nonzero",
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// HARDWARE HEALTH POLICY
// ============================================================================

/// Controls hardware health observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthPolicy {
    enabled: bool,
    interval: Option<Duration>,
    maximum_concurrent_checks: Option<NonZeroUsize>,
    maximum_consecutive_failures: Option<NonZeroUsize>,
}

impl Default for HealthPolicy {
    fn default() -> Self {
        Self {
            enabled: false,
            interval: None,
            maximum_concurrent_checks: None,
            maximum_consecutive_failures: None,
        }
    }
}

impl HealthPolicy {
    /// Returns whether periodic health observation is enabled.
    #[must_use]
    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    /// Returns the optional health observation interval.
    #[must_use]
    pub const fn interval(&self) -> Option<Duration> {
        self.interval
    }

    /// Returns the optional concurrent health-check budget.
    #[must_use]
    pub const fn maximum_concurrent_checks(
        &self,
    ) -> Option<NonZeroUsize> {
        self.maximum_concurrent_checks
    }

    /// Returns the optional consecutive failure threshold.
    #[must_use]
    pub const fn maximum_consecutive_failures(
        &self,
    ) -> Option<NonZeroUsize> {
        self.maximum_consecutive_failures
    }

    /// Enables or disables health observation.
    #[must_use]
    pub fn with_enabled(
        mut self,
        enabled: bool,
    ) -> Self {
        self.enabled = enabled;
        self
    }

    /// Sets the health observation interval.
    #[must_use]
    pub fn with_interval(
        mut self,
        interval: Option<Duration>,
    ) -> Self {
        self.interval = interval;
        self
    }

    /// Sets the concurrent health-check budget.
    #[must_use]
    pub fn with_maximum_concurrent_checks(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.maximum_concurrent_checks = maximum;
        self
    }

    /// Sets the consecutive health failure threshold.
    #[must_use]
    pub fn with_maximum_consecutive_failures(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.maximum_consecutive_failures = maximum;
        self
    }

    fn validate(&self) -> ConfigResult<()> {
        if let Some(interval) = self.interval {
            if interval.is_zero() {
                return Err(ConfigError::new(
                    ConfigErrorKind::InvalidValue,
                    "health.interval",
                    "health interval must be nonzero",
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// TIMEOUT POLICY
// ============================================================================

/// Timeouts for individual ZHAL operation categories.
///
/// An absent timeout means that this configuration does not impose
/// a timeout for that category.
///
/// The caller's deadline, provider contract, operating system or
/// enclosing runtime may still impose other deadlines.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TimeoutPolicy {
    provider_call: Option<Duration>,
    discovery: Option<Duration>,
    initialization: Option<Duration>,
    observation: Option<Duration>,
    health_check: Option<Duration>,
    device_access: Option<Duration>,
    shutdown: Option<Duration>,
}

impl TimeoutPolicy {
    /// Returns the provider-call timeout.
    #[must_use]
    pub const fn provider_call(&self) -> Option<Duration> {
        self.provider_call
    }

    /// Returns the discovery timeout.
    #[must_use]
    pub const fn discovery(&self) -> Option<Duration> {
        self.discovery
    }

    /// Returns the provider-initialization timeout.
    #[must_use]
    pub const fn initialization(&self) -> Option<Duration> {
        self.initialization
    }

    /// Returns the observation timeout.
    #[must_use]
    pub const fn observation(&self) -> Option<Duration> {
        self.observation
    }

    /// Returns the health-check timeout.
    #[must_use]
    pub const fn health_check(&self) -> Option<Duration> {
        self.health_check
    }

    /// Returns the device-access timeout.
    #[must_use]
    pub const fn device_access(&self) -> Option<Duration> {
        self.device_access
    }

    /// Returns the shutdown timeout.
    #[must_use]
    pub const fn shutdown(&self) -> Option<Duration> {
        self.shutdown
    }

    /// Sets the provider-call timeout.
    #[must_use]
    pub fn with_provider_call(
        mut self,
        timeout: Option<Duration>,
    ) -> Self {
        self.provider_call = timeout;
        self
    }

    /// Sets the discovery timeout.
    #[must_use]
    pub fn with_discovery(
        mut self,
        timeout: Option<Duration>,
    ) -> Self {
        self.discovery = timeout;
        self
    }

    /// Sets the initialization timeout.
    #[must_use]
    pub fn with_initialization(
        mut self,
        timeout: Option<Duration>,
    ) -> Self {
        self.initialization = timeout;
        self
    }

    /// Sets the observation timeout.
    #[must_use]
    pub fn with_observation(
        mut self,
        timeout: Option<Duration>,
    ) -> Self {
        self.observation = timeout;
        self
    }

    /// Sets the health-check timeout.
    #[must_use]
    pub fn with_health_check(
        mut self,
        timeout: Option<Duration>,
    ) -> Self {
        self.health_check = timeout;
        self
    }

    /// Sets the device-access timeout.
    #[must_use]
    pub fn with_device_access(
        mut self,
        timeout: Option<Duration>,
    ) -> Self {
        self.device_access = timeout;
        self
    }

    /// Sets the shutdown timeout.
    #[must_use]
    pub fn with_shutdown(
        mut self,
        timeout: Option<Duration>,
    ) -> Self {
        self.shutdown = timeout;
        self
    }
}

// ============================================================================
// RETRY POLICY
// ============================================================================

/// Rational multiplier for exponential retry backoff.
///
/// The multiplier is numerator / denominator.
///
/// A multiplier smaller than one is permitted when deliberately
/// configured. Both values must be nonzero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BackoffMultiplier {
    numerator: u32,
    denominator: u32,
}

impl BackoffMultiplier {
    /// Creates a validated backoff multiplier.
    pub fn new(
        numerator: u32,
        denominator: u32,
    ) -> ConfigResult<Self> {
        if numerator == 0 || denominator == 0 {
            return Err(ConfigError::new(
                ConfigErrorKind::InvalidValue,
                "retry.backoff_multiplier",
                "numerator and denominator must both be nonzero",
            ));
        }

        Ok(Self {
            numerator,
            denominator,
        })
    }

    /// Returns the multiplier numerator.
    #[must_use]
    pub const fn numerator(self) -> u32 {
        self.numerator
    }

    /// Returns the multiplier denominator.
    #[must_use]
    pub const fn denominator(self) -> u32 {
        self.denominator
    }
}

impl Default for BackoffMultiplier {
    fn default() -> Self {
        Self {
            numerator: 2,
            denominator: 1,
        }
    }
}

/// Controls whether selected provider failures may be retried.
///
/// Retry safety is determined by the operation's idempotency and
/// domain-specific execution contract, not by this policy alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetryPolicy {
    maximum_retries: Option<NonZeroUsize>,
    initial_delay: Duration,
    maximum_delay: Option<Duration>,
    multiplier: BackoffMultiplier,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            maximum_retries: None,
            initial_delay: Duration::ZERO,
            maximum_delay: None,
            multiplier: BackoffMultiplier::default(),
        }
    }
}

impl RetryPolicy {
    /// Returns the optional retry count.
    ///
    /// The initial operation attempt is not counted as a retry.
    #[must_use]
    pub const fn maximum_retries(&self) -> Option<NonZeroUsize> {
        self.maximum_retries
    }

    /// Returns the initial backoff delay.
    #[must_use]
    pub const fn initial_delay(&self) -> Duration {
        self.initial_delay
    }

    /// Returns the optional maximum backoff delay.
    #[must_use]
    pub const fn maximum_delay(&self) -> Option<Duration> {
        self.maximum_delay
    }

    /// Returns the exponential backoff multiplier.
    #[must_use]
    pub const fn multiplier(&self) -> BackoffMultiplier {
        self.multiplier
    }

    /// Sets the maximum number of retries.
    #[must_use]
    pub fn with_maximum_retries(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.maximum_retries = maximum;
        self
    }

    /// Sets the initial retry delay.
    #[must_use]
    pub fn with_initial_delay(
        mut self,
        delay: Duration,
    ) -> Self {
        self.initial_delay = delay;
        self
    }

    /// Sets the maximum retry delay.
    #[must_use]
    pub fn with_maximum_delay(
        mut self,
        delay: Option<Duration>,
    ) -> Self {
        self.maximum_delay = delay;
        self
    }

    /// Sets the backoff multiplier.
    #[must_use]
    pub fn with_multiplier(
        mut self,
        multiplier: BackoffMultiplier,
    ) -> Self {
        self.multiplier = multiplier;
        self
    }

    fn validate(&self) -> ConfigResult<()> {
        if let Some(maximum) = self.maximum_delay {
            if maximum < self.initial_delay {
                return Err(ConfigError::new(
                    ConfigErrorKind::ContradictorySettings,
                    "retry.maximum_delay",
                    "maximum delay cannot be smaller than initial delay",
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// CONCURRENCY POLICY
// ============================================================================

/// Configurable operational concurrency budgets.
///
/// None means ZHAL does not impose an additional concurrency budget
/// for that category.
///
/// It does not promise unlimited operating-system threads, provider
/// connections, execution capacity or physical resources.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConcurrencyPolicy {
    provider_calls: Option<NonZeroUsize>,
    discovery_operations: Option<NonZeroUsize>,
    observations: Option<NonZeroUsize>,
    health_checks: Option<NonZeroUsize>,
}

impl ConcurrencyPolicy {
    /// Returns the provider-call concurrency budget.
    #[must_use]
    pub const fn provider_calls(&self) -> Option<NonZeroUsize> {
        self.provider_calls
    }

    /// Returns the discovery concurrency budget.
    #[must_use]
    pub const fn discovery_operations(&self) -> Option<NonZeroUsize> {
        self.discovery_operations
    }

    /// Returns the observation concurrency budget.
    #[must_use]
    pub const fn observations(&self) -> Option<NonZeroUsize> {
        self.observations
    }

    /// Returns the health-check concurrency budget.
    #[must_use]
    pub const fn health_checks(&self) -> Option<NonZeroUsize> {
        self.health_checks
    }

    /// Sets the provider-call concurrency budget.
    #[must_use]
    pub fn with_provider_calls(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.provider_calls = maximum;
        self
    }

    /// Sets the discovery concurrency budget.
    #[must_use]
    pub fn with_discovery_operations(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.discovery_operations = maximum;
        self
    }

    /// Sets the observation concurrency budget.
    #[must_use]
    pub fn with_observations(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.observations = maximum;
        self
    }

    /// Sets the health-check concurrency budget.
    #[must_use]
    pub fn with_health_checks(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.health_checks = maximum;
        self
    }
}

// ============================================================================
// DIAGNOSTIC POLICY
// ============================================================================

/// Diagnostic verbosity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum DiagnosticLevel {
    /// Only critical diagnostic information.
    Critical,

    /// Errors and failures.
    Error,

    /// Errors and warnings.
    Warn,

    /// General operational information.
    #[default]
    Info,

    /// Detailed diagnostic information.
    Debug,

    /// Extensive diagnostic information.
    Trace,
}

/// Controls diagnostic information emitted by ZHAL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticPolicy {
    level: DiagnosticLevel,
    include_provider_details: bool,
    include_device_identifiers: bool,
    maximum_events_per_operation: Option<NonZeroUsize>,
}

impl Default for DiagnosticPolicy {
    fn default() -> Self {
        Self {
            level: DiagnosticLevel::Info,
            include_provider_details: true,
            include_device_identifiers: false,
            maximum_events_per_operation: None,
        }
    }
}

impl DiagnosticPolicy {
    /// Returns the diagnostic verbosity.
    #[must_use]
    pub const fn level(&self) -> DiagnosticLevel {
        self.level
    }

    /// Returns whether provider details may be included.
    #[must_use]
    pub const fn include_provider_details(&self) -> bool {
        self.include_provider_details
    }

    /// Returns whether device identifiers may be included.
    #[must_use]
    pub const fn include_device_identifiers(&self) -> bool {
        self.include_device_identifiers
    }

    /// Returns the optional diagnostic event budget.
    #[must_use]
    pub const fn maximum_events_per_operation(
        &self,
    ) -> Option<NonZeroUsize> {
        self.maximum_events_per_operation
    }

    /// Sets diagnostic verbosity.
    #[must_use]
    pub fn with_level(
        mut self,
        level: DiagnosticLevel,
    ) -> Self {
        self.level = level;
        self
    }

    /// Controls provider diagnostic details.
    #[must_use]
    pub fn with_provider_details(
        mut self,
        enabled: bool,
    ) -> Self {
        self.include_provider_details = enabled;
        self
    }

    /// Controls device identifier disclosure.
    #[must_use]
    pub fn with_device_identifiers(
        mut self,
        enabled: bool,
    ) -> Self {
        self.include_device_identifiers = enabled;
        self
    }

    /// Sets the diagnostic event budget.
    #[must_use]
    pub fn with_maximum_events_per_operation(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.maximum_events_per_operation = maximum;
        self
    }
}

// ============================================================================
// SECURITY POLICY REFERENCES
// ============================================================================

/// References to externally owned security policies.
///
/// These references do not perform authentication, authorization,
/// trust verification, isolation or enforcement.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SecurityPolicyReferences {
    provider_trust: Option<PolicyReference>,
    device_access: Option<PolicyReference>,
    isolation: Option<PolicyReference>,
}

impl SecurityPolicyReferences {
    /// Returns the provider trust policy reference.
    #[must_use]
    pub fn provider_trust(&self) -> Option<&PolicyReference> {
        self.provider_trust.as_ref()
    }

    /// Returns the device access policy reference.
    #[must_use]
    pub fn device_access(&self) -> Option<&PolicyReference> {
        self.device_access.as_ref()
    }

    /// Returns the isolation policy reference.
    #[must_use]
    pub fn isolation(&self) -> Option<&PolicyReference> {
        self.isolation.as_ref()
    }

    /// Sets the provider trust policy reference.
    #[must_use]
    pub fn with_provider_trust(
        mut self,
        reference: Option<PolicyReference>,
    ) -> Self {
        self.provider_trust = reference;
        self
    }

    /// Sets the device access policy reference.
    #[must_use]
    pub fn with_device_access(
        mut self,
        reference: Option<PolicyReference>,
    ) -> Self {
        self.device_access = reference;
        self
    }

    /// Sets the isolation policy reference.
    #[must_use]
    pub fn with_isolation(
        mut self,
        reference: Option<PolicyReference>,
    ) -> Self {
        self.isolation = reference;
        self
    }
}

// ============================================================================
// OPTIONAL FEATURE POLICY
// ============================================================================

/// Controls an optional ZHAL feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum FeatureActivation {
    /// The feature must not be activated.
    Disabled,

    /// Activate when explicitly required and supported.
    #[default]
    OnDemand,

    /// Permit activation during explicit subsystem initialization.
    Enabled,
}

/// An extensible identity for an optional feature.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FeatureId(String);

impl FeatureId {
    /// Constructs a validated feature identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> ConfigResult<Self> {
        let value = value.into();

        validate_identifier(
            &value,
            "feature_id",
        )?;

        Ok(Self(value))
    }

    /// Returns the canonical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Configures optional features using open-ended identifiers.
///
/// New technologies and optional integrations do not require
/// modifications to this structure.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OptionalFeaturePolicy {
    features: BTreeMap<FeatureId, FeatureActivation>,
}

impl OptionalFeaturePolicy {
    /// Sets a feature activation policy.
    ///
    /// An existing entry is explicitly replaced.
    pub fn set(
        &mut self,
        feature: FeatureId,
        activation: FeatureActivation,
    ) -> Option<FeatureActivation> {
        self.features.insert(feature, activation)
    }

    /// Returns an explicit feature policy.
    #[must_use]
    pub fn get(
        &self,
        feature: &FeatureId,
    ) -> Option<FeatureActivation> {
        self.features.get(feature).copied()
    }

    /// Returns the effective policy.
    ///
    /// Features without an explicit override default to on-demand.
    #[must_use]
    pub fn effective(
        &self,
        feature: &FeatureId,
    ) -> FeatureActivation {
        self.get(feature).unwrap_or_default()
    }

    /// Returns all explicitly configured feature policies.
    #[must_use]
    pub fn features(
        &self,
    ) -> &BTreeMap<FeatureId, FeatureActivation> {
        &self.features
    }

    fn validate(&self) -> ConfigResult<()> {
        for feature in self.features.keys() {
            validate_identifier(
                feature.as_str(),
                "feature_id",
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// COMPLETE ZHAL CONFIGURATION
// ============================================================================

/// Immutable configuration for one ZHAL instance.
///
/// Configuration is independent of physical hardware inventory,
/// provider implementation and runtime resource allocation.
///
/// All operational limits are optional and explicitly configured.
///
/// The default configuration imposes no additional hardware
/// population ceilings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZhalConfig {
    discovery: DiscoveryPolicy,
    initialization: ProviderInitializationPolicy,
    observation: ObservationPolicy,
    health: HealthPolicy,
    timeouts: TimeoutPolicy,
    retries: RetryPolicy,
    concurrency: ConcurrencyPolicy,
    diagnostics: DiagnosticPolicy,
    security: SecurityPolicyReferences,
    optional_features: OptionalFeaturePolicy,
    resource_budget_policy: Option<PolicyReference>,
}

impl Default for ZhalConfig {
    fn default() -> Self {
        Self {
            discovery: DiscoveryPolicy::default(),
            initialization: ProviderInitializationPolicy::default(),
            observation: ObservationPolicy::default(),
            health: HealthPolicy::default(),
            timeouts: TimeoutPolicy::default(),
            retries: RetryPolicy::default(),
            concurrency: ConcurrencyPolicy::default(),
            diagnostics: DiagnosticPolicy::default(),
            security: SecurityPolicyReferences::default(),
            optional_features: OptionalFeaturePolicy::default(),
            resource_budget_policy: None,
        }
    }
}

impl ZhalConfig {
    /// Creates the default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a configuration builder.
    #[must_use]
    pub fn builder() -> ZhalConfigBuilder {
        ZhalConfigBuilder::new()
    }

    /// Returns the discovery policy.
    #[must_use]
    pub const fn discovery(&self) -> &DiscoveryPolicy {
        &self.discovery
    }

    /// Returns the provider initialization policy.
    #[must_use]
    pub const fn initialization(
        &self,
    ) -> &ProviderInitializationPolicy {
        &self.initialization
    }

    /// Returns the observation policy.
    #[must_use]
    pub const fn observation(&self) -> &ObservationPolicy {
        &self.observation
    }

    /// Returns the health policy.
    #[must_use]
    pub const fn health(&self) -> &HealthPolicy {
        &self.health
    }

    /// Returns the timeout policy.
    #[must_use]
    pub const fn timeouts(&self) -> &TimeoutPolicy {
        &self.timeouts
    }

    /// Returns the retry policy.
    #[must_use]
    pub const fn retries(&self) -> &RetryPolicy {
        &self.retries
    }

    /// Returns the concurrency policy.
    #[must_use]
    pub const fn concurrency(&self) -> &ConcurrencyPolicy {
        &self.concurrency
    }

    /// Returns the diagnostic policy.
    #[must_use]
    pub const fn diagnostics(&self) -> &DiagnosticPolicy {
        &self.diagnostics
    }

    /// Returns the security policy references.
    #[must_use]
    pub const fn security(&self) -> &SecurityPolicyReferences {
        &self.security
    }

    /// Returns optional feature policies.
    #[must_use]
    pub const fn optional_features(
        &self,
    ) -> &OptionalFeaturePolicy {
        &self.optional_features
    }

    /// Returns the optional externally owned resource-budget policy.
    #[must_use]
    pub fn resource_budget_policy(
        &self,
    ) -> Option<&PolicyReference> {
        self.resource_budget_policy.as_ref()
    }

    /// Validates the complete configuration.
    ///
    /// This method validates internal consistency only.
    ///
    /// It does not:
    ///
    /// - Discover hardware.
    /// - Inspect providers.
    /// - Establish hardware feasibility.
    /// - Authorize device access.
    /// - Reserve resources.
    /// - Initialize providers.
    pub fn validate(&self) -> ConfigResult<()> {
        self.discovery.validate()?;
        self.observation.validate()?;
        self.health.validate()?;
        self.retries.validate()?;
        self.optional_features.validate()?;

        Ok(())
    }

    /// Returns a builder initialized from this configuration.
    #[must_use]
    pub fn to_builder(&self) -> ZhalConfigBuilder {
        ZhalConfigBuilder {
            config: self.clone(),
        }
    }
}

// ============================================================================
// CONFIGURATION BUILDER
// ============================================================================

/// Builder for constructing validated ZHAL configuration.
///
/// Construction performs no hardware operations or initialization.
#[derive(Debug, Clone, Default)]
pub struct ZhalConfigBuilder {
    config: ZhalConfig,
}

impl ZhalConfigBuilder {
    /// Creates a builder with default policies.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets discovery policy.
    #[must_use]
    pub fn discovery(
        mut self,
        policy: DiscoveryPolicy,
    ) -> Self {
        self.config.discovery = policy;
        self
    }

    /// Sets provider initialization policy.
    #[must_use]
    pub fn initialization(
        mut self,
        policy: ProviderInitializationPolicy,
    ) -> Self {
        self.config.initialization = policy;
        self
    }

    /// Sets observation policy.
    #[must_use]
    pub fn observation(
        mut self,
        policy: ObservationPolicy,
    ) -> Self {
        self.config.observation = policy;
        self
    }

    /// Sets health policy.
    #[must_use]
    pub fn health(
        mut self,
        policy: HealthPolicy,
    ) -> Self {
        self.config.health = policy;
        self
    }

    /// Sets timeout policy.
    #[must_use]
    pub fn timeouts(
        mut self,
        policy: TimeoutPolicy,
    ) -> Self {
        self.config.timeouts = policy;
        self
    }

    /// Sets retry policy.
    #[must_use]
    pub fn retries(
        mut self,
        policy: RetryPolicy,
    ) -> Self {
        self.config.retries = policy;
        self
    }

    /// Sets concurrency policy.
    #[must_use]
    pub fn concurrency(
        mut self,
        policy: ConcurrencyPolicy,
    ) -> Self {
        self.config.concurrency = policy;
        self
    }

    /// Sets diagnostic policy.
    #[must_use]
    pub fn diagnostics(
        mut self,
        policy: DiagnosticPolicy,
    ) -> Self {
        self.config.diagnostics = policy;
        self
    }

    /// Sets security policy references.
    #[must_use]
    pub fn security(
        mut self,
        policy: SecurityPolicyReferences,
    ) -> Self {
        self.config.security = policy;
        self
    }

    /// Sets optional feature policies.
    #[must_use]
    pub fn optional_features(
        mut self,
        policy: OptionalFeaturePolicy,
    ) -> Self {
        self.config.optional_features = policy;
        self
    }

    /// References the externally owned runtime resource-budget policy.
    #[must_use]
    pub fn resource_budget_policy(
        mut self,
        reference: Option<PolicyReference>,
    ) -> Self {
        self.config.resource_budget_policy = reference;
        self
    }

    /// Validates and builds the immutable configuration.
    pub fn build(self) -> ConfigResult<ZhalConfig> {
        self.config.validate()?;
        Ok(self.config)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;

    fn nonzero(value: usize) -> NonZeroUsize {
        NonZeroUsize::new(value).expect("nonzero value")
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            module_name(),
            "zamani.zhal.config"
        );

        assert_eq!(schema_version(), 1);
    }

    #[test]
    fn default_configuration_is_valid() {
        let config = ZhalConfig::default();

        assert!(config.validate().is_ok());

        assert_eq!(
            config.discovery().activation(),
            DiscoveryActivation::ExplicitOnly
        );

        assert_eq!(
            config.initialization().mode(),
            ProviderInitializationMode::Explicit
        );

        assert!(!config.health().enabled());
    }

    #[test]
    fn default_configuration_has_no_artificial_limits() {
        let config = ZhalConfig::default();

        assert_eq!(
            config.discovery().maximum_results_per_request(),
            None
        );

        assert_eq!(
            config.discovery().maximum_concurrent_providers(),
            None
        );

        assert_eq!(
            config.concurrency().provider_calls(),
            None
        );

        assert_eq!(
            config.concurrency().observations(),
            None
        );
    }

    #[test]
    fn periodic_discovery_requires_refresh_interval() {
        let policy = DiscoveryPolicy::default()
            .with_activation(DiscoveryActivation::Periodic);

        assert!(policy.validate().is_err());
    }

    #[test]
    fn periodic_discovery_accepts_configured_interval() {
        let policy = DiscoveryPolicy::default()
            .with_activation(DiscoveryActivation::Periodic)
            .with_refresh_interval(Some(Duration::from_secs(30)));

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn zero_refresh_interval_is_rejected() {
        let policy = DiscoveryPolicy::default()
            .with_refresh_interval(Some(Duration::ZERO));

        assert!(policy.validate().is_err());
    }

    #[test]
    fn observation_refresh_interval_must_be_nonzero() {
        let policy = ObservationPolicy::default()
            .with_refresh_interval(Some(Duration::ZERO));

        assert!(policy.validate().is_err());
    }

    #[test]
    fn health_interval_must_be_nonzero() {
        let policy = HealthPolicy::default()
            .with_interval(Some(Duration::ZERO));

        assert!(policy.validate().is_err());
    }

    #[test]
    fn retries_are_disabled_by_default() {
        let policy = RetryPolicy::default();

        assert_eq!(policy.maximum_retries(), None);
    }

    #[test]
    fn retry_policy_accepts_valid_backoff() {
        let policy = RetryPolicy::default()
            .with_maximum_retries(Some(nonzero(5)))
            .with_initial_delay(Duration::from_millis(10))
            .with_maximum_delay(Some(Duration::from_secs(2)));

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn retry_policy_rejects_contradictory_delays() {
        let policy = RetryPolicy::default()
            .with_initial_delay(Duration::from_secs(5))
            .with_maximum_delay(Some(Duration::from_secs(2)));

        assert!(policy.validate().is_err());
    }

    #[test]
    fn backoff_multiplier_rejects_zero_components() {
        assert!(BackoffMultiplier::new(0, 1).is_err());
        assert!(BackoffMultiplier::new(1, 0).is_err());
    }

    #[test]
    fn backoff_multiplier_accepts_rational_values() {
        let multiplier = BackoffMultiplier::new(3, 2).unwrap();

        assert_eq!(multiplier.numerator(), 3);
        assert_eq!(multiplier.denominator(), 2);
    }

    #[test]
    fn identifiers_are_extensible() {
        assert!(
            PolicyReference::new(
                "future.photonic.hardware_policy"
            ).is_ok()
        );

        assert!(
            FeatureId::new(
                "future.neuromorphic.acceleration"
            ).is_ok()
        );
    }

    #[test]
    fn invalid_identifiers_are_rejected() {
        assert!(PolicyReference::new("").is_err());

        assert!(
            PolicyReference::new(
                "invalid identifier"
            ).is_err()
        );

        assert!(
            PolicyReference::new(
                "hardware."
            ).is_err()
        );

        assert!(
            FeatureId::new(
                "Future.Hardware"
            ).is_err()
        );
    }

    #[test]
    fn optional_features_are_extensible() {
        let feature = FeatureId::new(
            "future.photonic.execution"
        ).unwrap();

        let mut policy = OptionalFeaturePolicy::default();

        policy.set(
            feature.clone(),
            FeatureActivation::Enabled,
        );

        assert_eq!(
            policy.effective(&feature),
            FeatureActivation::Enabled
        );
    }

    #[test]
    fn security_references_do_not_require_credentials() {
        let policy = SecurityPolicyReferences::default()
            .with_provider_trust(Some(
                PolicyReference::new(
                    "zur.provider_trust"
                ).unwrap()
            ));

        assert_eq!(
            policy.provider_trust().unwrap().as_str(),
            "zur.provider_trust"
        );
    }

    #[test]
    fn builder_constructs_valid_configuration() {
        let config = ZhalConfig::builder()
            .discovery(
                DiscoveryPolicy::default()
                    .with_activation(
                        DiscoveryActivation::OnDemand
                    )
                    .with_maximum_concurrent_providers(
                        Some(nonzero(8))
                    )
            )
            .initialization(
                ProviderInitializationPolicy::default()
                    .with_mode(
                        ProviderInitializationMode::OnDemand
                    )
            )
            .health(
                HealthPolicy::default()
                    .with_enabled(true)
                    .with_interval(Some(
                        Duration::from_secs(60)
                    ))
            )
            .build()
            .unwrap();

        assert!(config.validate().is_ok());

        assert_eq!(
            config.discovery().activation(),
            DiscoveryActivation::OnDemand
        );

        assert!(config.health().enabled());
    }

    #[test]
    fn configuration_can_be_modified_without_losing_other_settings() {
        let original = ZhalConfig::builder()
            .discovery(
                DiscoveryPolicy::default()
                    .with_activation(
                        DiscoveryActivation::OnDemand
                    )
            )
            .build()
            .unwrap();

        let updated = original
            .to_builder()
            .health(
                HealthPolicy::default()
                    .with_enabled(true)
            )
            .build()
            .unwrap();

        assert_eq!(
            updated.discovery().activation(),
            DiscoveryActivation::OnDemand
        );

        assert!(updated.health().enabled());
    }

    #[test]
    fn timeout_zero_is_a_valid_immediate_timeout() {
        let policy = TimeoutPolicy::default()
            .with_provider_call(Some(Duration::ZERO));

        assert_eq!(
            policy.provider_call(),
            Some(Duration::ZERO)
        );
    }

    #[test]
    fn concurrency_limits_are_operational_not_hardware_limits() {
        let policy = ConcurrencyPolicy::default()
            .with_provider_calls(Some(nonzero(64)))
            .with_observations(Some(nonzero(128)));

        assert_eq!(
            policy.provider_calls().unwrap().get(),
            64
        );

        assert_eq!(
            policy.observations().unwrap().get(),
            128
        );
    }

    #[test]
    fn configuration_uses_deterministic_feature_ordering() {
        let mut features = OptionalFeaturePolicy::default();

        let second = FeatureId::new(
            "vendor.second.feature"
        ).unwrap();

        let first = FeatureId::new(
            "vendor.first.feature"
        ).unwrap();

        features.set(second, FeatureActivation::Enabled);
        features.set(first, FeatureActivation::Disabled);

        let keys: Vec<&str> = features
            .features()
            .keys()
            .map(FeatureId::as_str)
            .collect();

        assert_eq!(
            keys,
            vec![
                "vendor.first.feature",
                "vendor.second.feature"
            ]
        );
    }

    #[test]
    fn configuration_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<ZhalConfig>();
        assert_send_sync::<ZhalConfigBuilder>();
    }

    #[test]
    fn configuration_cloning_preserves_values() {
        let original = ZhalConfig::builder()
            .discovery(
                DiscoveryPolicy::default()
                    .with_activation(
                        DiscoveryActivation::OnDemand
                    )
            )
            .build()
            .unwrap();

        let cloned = original.clone();

        assert_eq!(original, cloned);
    }
}
