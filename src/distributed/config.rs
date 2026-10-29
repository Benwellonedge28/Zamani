
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Distributed Runtime Configuration
//!
//! Path: `src/distributed/config.rs`
//!
//! ## Purpose
//!
//! Defines validated, target-independent operational configuration for
//! distributed planning and execution.
//!
//! ## Authority
//!
//! - `grammar/spec/distributed.md`
//! - `grammar/specification/scalability-model.md`
//! - `src/distributed/requirements.rs`
//! - The canonical semantic resource and capability models.
//!
//! ## Ownership
//!
//! This module owns operational configuration only:
//!
//! - optional execution budgets;
//! - optional operation and shutdown timeouts;
//! - retry and backoff policy;
//! - configuration validation;
//! - open-world extension settings.
//!
//! It does not own semantic requirements, capability identities, resource
//! quantities, resource feasibility, scheduling, placement, networking,
//! authentication implementations, hardware allocation, or execution.
//!
//! ## Scalability
//!
//! No universal maximum is imposed by this module. An unset budget means
//! that this configuration does not impose that particular operational cap.
//! The operating environment, runtime, transport, or provider may still have
//! genuine resource or representation limits.
//!
//! Configuration values are policies, not guarantees of available capacity.
//! A configured budget does not prove that resources exist.
//!
//! ## Integration
//!
//! The distributed planner may read planning budgets from this configuration.
//! The executor may read concurrency and operation budgets.
//! Transport adapters may read message and in-flight byte budgets.
//! Lifecycle management may read shutdown timeouts.
//! Retry implementations may read retry policy.
//!
//! These consumers must enforce the relevant policy and return typed errors
//! when a budget is exceeded. Merely storing a configuration value does not
//! enforce it.
//!
//! This module intentionally uses only the Rust standard library.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;
use std::num::NonZeroUsize;
use std::time::Duration;

/// Version of this in-memory configuration API.
///
/// This is not a language version, wire-protocol version, or persisted
/// configuration-file schema version.
pub const DISTRIBUTED_CONFIG_API_VERSION: u16 = 1;

/// A validated, namespaced key for an operational extension setting.
///
/// Namespaces permit independently developed domains and providers to add
/// configuration without modifying a closed Rust enum. These keys do not
/// represent semantic capabilities or resource kinds.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExtensionKey {
    namespace: String,
    name: String,
}

impl ExtensionKey {
    /// Creates a validated extension key.
    ///
    /// Components must be non-empty after trimming and contain no control
    /// characters. Their original spelling is preserved.
    ///
    /// # Errors
    ///
    /// Returns an error if either component is invalid.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, ConfigError> {
        let namespace = namespace.into();
        let name = name.into();

        if !is_valid_identifier_component(&namespace)
            || !is_valid_identifier_component(&name)
        {
            return Err(ConfigError::InvalidExtensionKey {
                namespace,
                name,
            });
        }

        Ok(Self { namespace, name })
    }

    /// Returns the extension namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the extension name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the stable namespaced spelling `namespace:name`.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}:{}", self.namespace, self.name)
    }
}

impl fmt::Display for ExtensionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.name)
    }
}

fn is_valid_identifier_component(value: &str) -> bool {
    !value.trim().is_empty() && !value.chars().any(char::is_control)
}

/// An open-world scalar value for a provider-specific setting.
///
/// The owner of the namespace defines the meaning and validation rules of
/// each setting. Unknown values must not be silently interpreted as enforced
/// policies.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ExtensionValue {
    /// A textual value.
    Text(String),

    /// A boolean value.
    Boolean(bool),

    /// An unsigned integer represented as decimal text.
    ///
    /// A string representation avoids imposing a machine-integer ceiling
    /// on the value's textual representation. The extension owner must
    /// validate the syntax and numeric meaning required by its setting.
    UnsignedDecimal(String),
}

impl ExtensionValue {
    /// Constructs a textual extension value.
    #[must_use]
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    /// Constructs a boolean extension value.
    #[must_use]
    pub const fn boolean(value: bool) -> Self {
        Self::Boolean(value)
    }

    /// Constructs an unsigned-decimal extension value.
    ///
    /// This preserves the supplied text. It does not validate or normalize
    /// the decimal representation; that is the namespace owner's job.
    #[must_use]
    pub fn unsigned_decimal(value: impl Into<String>) -> Self {
        Self::UnsignedDecimal(value.into())
    }
}

/// Retry policy for operations that the execution layer has independently
/// established are safe to retry.
///
/// A retry policy alone does not make a non-idempotent operation safe.
/// The executor must account for side effects, delivery uncertainty,
/// deadlines, cancellation, and the provider's idempotency guarantees.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetryPolicy {
    max_attempts: NonZeroUsize,
    initial_backoff: Duration,
    max_backoff: Duration,
    multiplier: u32,
}

impl RetryPolicy {
    /// Creates a retry policy.
    ///
    /// `max_attempts` includes the initial attempt. A value of one disables
    /// retries. The backoff multiplier must be positive, and the maximum
    /// backoff must not be smaller than the initial backoff.
    ///
    /// # Errors
    ///
    /// Returns an error if the backoff configuration is invalid.
    pub fn new(
        max_attempts: NonZeroUsize,
        initial_backoff: Duration,
        max_backoff: Duration,
        multiplier: u32,
    ) -> Result<Self, ConfigError> {
        if multiplier == 0 {
            return Err(ConfigError::ZeroBackoffMultiplier);
        }

        if max_backoff < initial_backoff {
            return Err(ConfigError::MaximumBackoffBelowInitial);
        }

        Ok(Self {
            max_attempts,
            initial_backoff,
            max_backoff,
            multiplier,
        })
    }

    /// Returns the maximum number of attempts, including the initial attempt.
    #[must_use]
    pub const fn max_attempts(&self) -> NonZeroUsize {
        self.max_attempts
    }

    /// Returns the initial retry delay.
    #[must_use]
    pub const fn initial_backoff(&self) -> Duration {
        self.initial_backoff
    }

    /// Returns the maximum permitted retry delay.
    #[must_use]
    pub const fn max_backoff(&self) -> Duration {
        self.max_backoff
    }

    /// Returns the exponential backoff multiplier.
    #[must_use]
    pub const fn multiplier(&self) -> u32 {
        self.multiplier
    }

    /// Returns whether this policy permits additional attempts.
    #[must_use]
    pub const fn retries_enabled(&self) -> bool {
        self.max_attempts.get() > 1
    }

    /// Computes the capped delay before a retry.
    ///
    /// `retry_index` is zero-based: zero returns the initial backoff,
    /// one applies the multiplier once, and so on.
    ///
    /// Arithmetic saturates and the result is capped at `max_backoff`.
    /// This function does not sleep, retry an operation, or establish
    /// whether retrying that operation is semantically safe.
    #[must_use]
    pub fn delay_before_retry(&self, retry_index: usize) -> Duration {
        let mut delay = self.initial_backoff;

        for _ in 0..retry_index {
            if delay >= self.max_backoff {
                return self.max_backoff;
            }

            let next_nanos = delay
                .as_nanos()
                .saturating_mul(u128::from(self.multiplier));

            let cap_nanos = self.max_backoff.as_nanos();
            let bounded_nanos = next_nanos.min(cap_nanos);

            delay = duration_from_nanos_saturating(bounded_nanos);

            if delay >= self.max_backoff {
                return self.max_backoff;
            }
        }

        delay.min(self.max_backoff)
    }
}

fn duration_from_nanos_saturating(nanos: u128) -> Duration {
    let seconds = nanos / 1_000_000_000;
    let subsecond_nanos = (nanos % 1_000_000_000) as u32;

    if seconds > u128::from(u64::MAX) {
        Duration::MAX
    } else {
        Duration::new(seconds as u64, subsecond_nanos)
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: NonZeroUsize::MIN,
            initial_backoff: Duration::ZERO,
            max_backoff: Duration::ZERO,
            multiplier: 1,
        }
    }
}

/// Optional execution and transport budgets.
///
/// Every field is an operational policy, not a semantic restriction on
/// programs expressed in Zamani. `None` means this budget is not set by
/// this configuration.
///
/// A downstream implementation may have additional real limits. Such
/// limits must be reported honestly rather than represented as if this
/// configuration had established a universal maximum.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResourceBudgets {
    max_concurrent_tasks: Option<NonZeroUsize>,
    max_in_flight_operations: Option<NonZeroUsize>,
    max_message_bytes: Option<NonZeroUsize>,
    max_in_flight_bytes: Option<NonZeroUsize>,
}

impl ResourceBudgets {
    /// Creates an empty budget policy with no configured caps.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_concurrent_tasks: None,
            max_in_flight_operations: None,
            max_message_bytes: None,
            max_in_flight_bytes: None,
        }
    }

    /// Sets the optional maximum number of concurrently executing tasks.
    #[must_use]
    pub fn with_max_concurrent_tasks(
        mut self,
        limit: Option<NonZeroUsize>,
    ) -> Self {
        self.max_concurrent_tasks = limit;
        self
    }

    /// Sets the optional maximum number of in-flight operations.
    #[must_use]
    pub fn with_max_in_flight_operations(
        mut self,
        limit: Option<NonZeroUsize>,
    ) -> Self {
        self.max_in_flight_operations = limit;
        self
    }

    /// Sets the optional maximum encoded message size in bytes.
    #[must_use]
    pub fn with_max_message_bytes(
        mut self,
        limit: Option<NonZeroUsize>,
    ) -> Self {
        self.max_message_bytes = limit;
        self
    }

    /// Sets the optional maximum aggregate in-flight byte budget.
    #[must_use]
    pub fn with_max_in_flight_bytes(
        mut self,
        limit: Option<NonZeroUsize>,
    ) -> Self {
        self.max_in_flight_bytes = limit;
        self
    }

    /// Returns the configured concurrent-task limit, if any.
    #[must_use]
    pub const fn max_concurrent_tasks(&self) -> Option<NonZeroUsize> {
        self.max_concurrent_tasks
    }

    /// Returns the configured in-flight operation limit, if any.
    #[must_use]
    pub const fn max_in_flight_operations(&self) -> Option<NonZeroUsize> {
        self.max_in_flight_operations
    }

    /// Returns the configured message-size limit, if any.
    #[must_use]
    pub const fn max_message_bytes(&self) -> Option<NonZeroUsize> {
        self.max_message_bytes
    }

    /// Returns the configured aggregate in-flight byte limit, if any.
    #[must_use]
    pub const fn max_in_flight_bytes(&self) -> Option<NonZeroUsize> {
        self.max_in_flight_bytes
    }
}

/// Operational configuration for distributed execution.
///
/// This type is deliberately independent of the executor, scheduler,
/// transport, backend, and runtime implementations. It can be constructed
/// and validated before those components are initialized.
///
/// The defaults impose no resource-count or byte-count ceilings, no operation
/// timeout, no shutdown timeout, and no automatic retries. Deployments should
/// provide explicit budgets and deadlines appropriate to their environment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DistributedConfig {
    operation_timeout: Option<Duration>,
    shutdown_timeout: Option<Duration>,
    resource_budgets: ResourceBudgets,
    retry_policy: RetryPolicy,
    extensions: BTreeMap<ExtensionKey, ExtensionValue>,
}

impl Default for DistributedConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl DistributedConfig {
    /// Creates a configuration without arbitrary operational ceilings.
    ///
    /// This does not imply unlimited physical resources or unlimited
    /// execution time. It means only that this configuration has not set
    /// optional caps or deadlines.
    #[must_use]
    pub fn new() -> Self {
        Self {
            operation_timeout: None,
            shutdown_timeout: None,
            resource_budgets: ResourceBudgets::new(),
            retry_policy: RetryPolicy::default(),
            extensions: BTreeMap::new(),
        }
    }

    /// Sets the optional timeout for an individual operation.
    ///
    /// `None` means this configuration imposes no operation timeout.
    /// A runtime may still have a deadline from a caller or external policy.
    #[must_use]
    pub fn with_operation_timeout(
        mut self,
        timeout: Option<Duration>,
    ) -> Self {
        self.operation_timeout = timeout;
        self
    }

    /// Sets the optional graceful-shutdown timeout.
    ///
    /// `None` means this configuration does not set a shutdown deadline.
    #[must_use]
    pub fn with_shutdown_timeout(
        mut self,
        timeout: Option<Duration>,
    ) -> Self {
        self.shutdown_timeout = timeout;
        self
    }

    /// Sets the resource-budget policy.
    #[must_use]
    pub fn with_resource_budgets(
        mut self,
        budgets: ResourceBudgets,
    ) -> Self {
        self.resource_budgets = budgets;
        self
    }

    /// Sets the retry policy.
    #[must_use]
    pub fn with_retry_policy(
        mut self,
        policy: RetryPolicy,
    ) -> Self {
        self.retry_policy = policy;
        self
    }

    /// Inserts or replaces a namespaced extension setting.
    ///
    /// Replacement is explicit at the call site. Extension settings are
    /// ordered deterministically by their key.
    #[must_use]
    pub fn with_extension(
        mut self,
        key: ExtensionKey,
        value: ExtensionValue,
    ) -> Self {
        self.extensions.insert(key, value);
        self
    }

    /// Returns the optional operation timeout.
    #[must_use]
    pub const fn operation_timeout(&self) -> Option<Duration> {
        self.operation_timeout
    }

    /// Returns the optional shutdown timeout.
    #[must_use]
    pub const fn shutdown_timeout(&self) -> Option<Duration> {
        self.shutdown_timeout
    }

    /// Returns the configured resource budgets.
    #[must_use]
    pub const fn resource_budgets(&self) -> &ResourceBudgets {
        &self.resource_budgets
    }

    /// Returns the retry policy.
    #[must_use]
    pub const fn retry_policy(&self) -> &RetryPolicy {
        &self.retry_policy
    }

    /// Returns all extension settings in deterministic key order.
    #[must_use]
    pub fn extensions(&self) -> &BTreeMap<ExtensionKey, ExtensionValue> {
        &self.extensions
    }

    /// Returns one extension value, if configured.
    #[must_use]
    pub fn extension(
        &self,
        key: &ExtensionKey,
    ) -> Option<&ExtensionValue> {
        self.extensions.get(key)
    }

    /// Validates invariants that apply to the whole configuration.
    ///
    /// The current configuration uses validated value types and therefore
    /// has no additional cross-field invariants. This method gives runtime
    /// initialization a stable validation boundary for future compatible
    /// extensions.
    ///
    /// # Errors
    ///
    /// Returns an error if a future or externally constructed configuration
    /// violates a configuration invariant.
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.retry_policy.multiplier == 0 {
            return Err(ConfigError::ZeroBackoffMultiplier);
        }

        if self.retry_policy.max_backoff
            < self.retry_policy.initial_backoff
        {
            return Err(ConfigError::MaximumBackoffBelowInitial);
        }

        Ok(())
    }
}

/// Errors from construction or validation of distributed configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConfigError {
    /// An extension key has an empty or invalid component.
    InvalidExtensionKey {
        /// The supplied namespace.
        namespace: String,
        /// The supplied setting name.
        name: String,
    },

    /// A retry policy has a zero multiplier.
    ZeroBackoffMultiplier,

    /// The maximum backoff is smaller than the initial backoff.
    MaximumBackoffBelowInitial,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidExtensionKey { namespace, name } => write!(
                f,
                "invalid distributed extension key: namespace {:?}, \
                 name {:?}; components must be non-empty and contain \
                 no control characters",
                namespace, name
            ),
            Self::ZeroBackoffMultiplier => {
                f.write_str("retry backoff multiplier must be positive")
            }
            Self::MaximumBackoffBelowInitial => f.write_str(
                "maximum retry backoff must be greater than or equal \
                 to the initial retry backoff",
            ),
        }
    }
}

impl Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_do_not_impose_resource_caps_or_deadlines() {
        let config = DistributedConfig::default();

        assert_eq!(config.operation_timeout(), None);
        assert_eq!(config.shutdown_timeout(), None);
        assert_eq!(
            config.resource_budgets().max_concurrent_tasks(),
            None
        );
        assert_eq!(
            config.resource_budgets().max_in_flight_operations(),
            None
        );
        assert_eq!(
            config.resource_budgets().max_message_bytes(),
            None
        );
        assert_eq!(
            config.resource_budgets().max_in_flight_bytes(),
            None
        );
        assert!(!config.retry_policy().retries_enabled());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn validates_retry_policy() {
        let attempts = NonZeroUsize::new(4).expect("four is nonzero");

        assert!(matches!(
            RetryPolicy::new(
                attempts,
                Duration::from_millis(10),
                Duration::from_millis(5),
                2,
            ),
            Err(ConfigError::MaximumBackoffBelowInitial)
        ));

        assert!(matches!(
            RetryPolicy::new(
                attempts,
                Duration::ZERO,
                Duration::from_secs(1),
                0,
            ),
            Err(ConfigError::ZeroBackoffMultiplier)
        ));
    }

    #[test]
    fn computes_capped_backoff_without_overflow() {
        let policy = RetryPolicy::new(
            NonZeroUsize::new(8).expect("eight is nonzero"),
            Duration::from_millis(10),
            Duration::from_millis(35),
            3,
        )
        .expect("policy is valid");

        assert_eq!(
            policy.delay_before_retry(0),
            Duration::from_millis(10)
        );
        assert_eq!(
            policy.delay_before_retry(1),
            Duration::from_millis(30)
        );
        assert_eq!(
            policy.delay_before_retry(2),
            Duration::from_millis(35)
        );
        assert_eq!(
            policy.delay_before_retry(usize::MAX),
            Duration::from_millis(35)
        );
    }

    #[test]
    fn accepts_optional_budgets_without_changing_semantics() {
        let budgets = ResourceBudgets::new()
            .with_max_concurrent_tasks(NonZeroUsize::new(128))
            .with_max_in_flight_operations(NonZeroUsize::new(4096));

        let config = DistributedConfig::new()
            .with_resource_budgets(budgets)
            .with_operation_timeout(Some(Duration::from_secs(30)));

        assert_eq!(
            config.resource_budgets().max_concurrent_tasks(),
            NonZeroUsize::new(128)
        );
        assert_eq!(
            config.resource_budgets().max_in_flight_operations(),
            NonZeroUsize::new(4096)
        );
        assert_eq!(
            config.operation_timeout(),
            Some(Duration::from_secs(30))
        );
        assert!(config.validate().is_ok());
    }

    #[test]
    fn extension_keys_are_validated_and_deterministic() {
        assert!(ExtensionKey::new("   ", "setting").is_err());
        assert!(ExtensionKey::new("provider", "bad\nname").is_err());

        let key = ExtensionKey::new("transport.example", "compression")
            .expect("key is valid");

        let config = DistributedConfig::new().with_extension(
            key.clone(),
            ExtensionValue::text("provider-defined"),
        );

        assert_eq!(
            config.extension(&key),
            Some(&ExtensionValue::text("provider-defined"))
        );
        assert_eq!(
            key.qualified_name(),
            "transport.example:compression"
        );
    }

    #[test]
    fn extension_replacement_is_explicit_and_stable() {
        let key = ExtensionKey::new("runtime.example", "mode")
            .expect("key is valid");

        let config = DistributedConfig::new()
            .with_extension(key.clone(), ExtensionValue::text("first"))
            .with_extension(key.clone(), ExtensionValue::text("second"));

        assert_eq!(
            config.extension(&key),
            Some(&ExtensionValue::text("second"))
        );
        assert_eq!(config.extensions().len(), 1);
    }

    #[test]
    fn maximum_duration_backoff_is_handled_safely() {
        let policy = RetryPolicy::new(
            NonZeroUsize::new(3).expect("three is nonzero"),
            Duration::MAX,
            Duration::MAX,
            u32::MAX,
        )
        .expect("policy is valid");

        assert_eq!(policy.delay_before_retry(usize::MAX), Duration::MAX);
    }
}
