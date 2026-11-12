
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Configuration
//!
//! Path: src/zur/config.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//!
//! PURPOSE
//!
//! Defines immutable, deterministic configuration for the Zamani
//! Universal Runtime (ZUR).
//!
//! ZUR configuration controls operational runtime behavior without
//! redefining program semantics, compilation requirements, target
//! capabilities, physical resource allocation, or domain-specific
//! execution planning.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Runtime configuration.
//! - Runtime operational policies.
//! - Optional operational budgets.
//! - Extensible budget identifiers.
//! - Runtime service activation preferences.
//! - JIT activation policy.
//! - Validated alternative-execution policy.
//! - Configuration construction and validation.
//!
//! THIS MODULE DOES NOT OWN
//!
//! - Source-language semantics.
//! - AST, types, effects, or semantic requirements.
//! - Capability definitions or capability evidence.
//! - Physical resource discovery.
//! - Physical resource allocation or reservation.
//! - Backend registration or selection.
//! - Artifact manifests.
//! - Compiler optimization or lowering.
//! - Quantum routing or error correction.
//! - Distributed execution planning.
//! - Hybrid execution planning.
//! - Platform discovery.
//! - Actual budget accounting or enforcement.
//! - Service registration or service implementation.
//! - JIT compilation implementation.
//!
//! These responsibilities belong to their respective subsystems.
//!
//! SCALABILITY
//!
//! No arbitrary hardware, computational-domain, provider, service,
//! device, processor, memory, thread, node, qubit, or task ceilings
//! are imposed.
//!
//! Budget quantities use canonical decimal strings rather than
//! fixed-width integers. This avoids introducing an arbitrary numeric
//! ceiling into the configuration representation.
//!
//! An absent budget means that this configuration imposes no explicit
//! budget for that dimension.
//!
//! It does not promise infinite physical resources.
//!
//! DETERMINISM
//!
//! Configuration uses ordered collections.
//!
//! It does not read environment variables, inspect hardware, access
//! the filesystem, access the network, read the clock, generate random
//! values, or initialize global state.
//!
//! SAFETY
//!
//! Safe Rust only.
//!
//! INTEGRATION
//!
//! - error.rs owns ZurError and ZurResult.
//! - version.rs owns runtime contract versions.
//! - ids.rs owns runtime-specific identities.
//! - core/context.rs consumes RuntimeConfig.
//! - core/builder.rs validates and installs configuration.
//! - core/service_manager.rs applies service activation policies.
//! - memory/, concurrency/, scheduling/, and other services enforce
//!   their applicable operational budgets.
//! - jit/ consumes JitPolicy.
//! - execution/ consumes ExecutionFallbackPolicy.
//!
//! The configuration is immutable after construction and contains
//! no invocation-local accounting state.

use core::cmp::Ordering;
use core::fmt;
use std::collections::BTreeMap;

use crate::zur::error::{ZurError, ZurResult};

// =============================================================================
// VALIDATION
// =============================================================================

/// Validates a canonical namespaced configuration identifier.
///
/// Format:
///
///     namespace.segment.identifier
///
/// Permitted characters:
///
/// - ASCII lowercase letters.
/// - ASCII digits.
/// - Hyphens.
/// - Underscores.
/// - Dots separating nonempty segments.
///
/// Identifiers must contain at least two segments.
///
/// There is no arbitrary identifier-length ceiling.
///
/// This validation is shared by budget and service identifiers.
fn validate_namespaced_identifier(
    value: &str,
    description: &str,
) -> ZurResult<()> {
    if value.is_empty() {
        return Err(configuration_error(
            format!("{description} must not be empty"),
        ));
    }

    if value.trim() != value {
        return Err(configuration_error(
            format!("{description} must not contain surrounding whitespace"),
        ));
    }

    let mut segments = value.split('.');

    let first = segments.next().ok_or_else(|| {
        configuration_error(
            format!("{description} must be namespaced"),
        )
    })?;

    let mut segment_count = 0usize;

    for segment in std::iter::once(first).chain(segments) {
        if segment.is_empty() {
            return Err(configuration_error(
                format!("{description} contains an empty namespace segment"),
            ));
        }

        if !segment.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'-'
                || byte == b'_'
        }) {
            return Err(configuration_error(
                format!("{description} contains an invalid character"),
            ));
        }

        if segment.starts_with('-')
            || segment.starts_with('_')
            || segment.ends_with('-')
            || segment.ends_with('_')
        {
            return Err(configuration_error(
                format!("{description} has an invalid segment boundary"),
            ));
        }

        segment_count += 1;
    }

    if segment_count < 2 {
        return Err(configuration_error(
            format!("{description} must contain a namespace and identifier"),
        ));
    }

    Ok(())
}

/// Constructs a structured ZUR configuration error.
fn configuration_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::configuration(message)
        .with_component("zur.config")
}

// =============================================================================
// EXTENSIBLE BUDGET IDENTIFIERS
// =============================================================================

/// Identifies one operational budget dimension.
///
/// Budget identifiers are open-ended namespaced strings.
///
/// Examples:
///
/// - `core.memory_bytes`
/// - `core.execution_time_ns`
/// - `core.concurrent_tasks`
/// - `core.io_bytes`
/// - `core.provider_sessions`
/// - `vendor.extension.custom_work`
///
/// These are identifiers, not a closed list of supported resources.
///
/// A subsystem may introduce new dimensions without changing this type.
///
/// The unit is part of the identifier's contract. For example,
/// `core.memory_bytes` and `core.execution_time_ns` are different
/// dimensions and must not be compared as though they shared units.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BudgetKey(String);

impl BudgetKey {
    /// Constructs a validated budget identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_namespaced_identifier(
            &value,
            "budget identifier",
        )?;

        Ok(Self(value))
    }

    /// Returns the canonical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for BudgetKey {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// EXTENSIBLE SERVICE IDENTIFIERS
// =============================================================================

/// Identifies a runtime service contract.
///
/// This identifies a service category, not an instantiated service.
///
/// It must not be confused with:
///
/// - ServiceInstanceId.
/// - Provider identity.
/// - Backend identity.
/// - Physical device identity.
///
/// Those identities remain owned by their respective subsystems.
///
/// Service identifiers are extensible and namespaced.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceKey(String);

impl ServiceKey {
    /// Constructs a validated service identifier.
    pub fn new(
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_namespaced_identifier(
            &value,
            "service identifier",
        )?;

        Ok(Self(value))
    }

    /// Returns the canonical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for ServiceKey {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// ARBITRARY-LENGTH DECIMAL BUDGET QUANTITIES
// =============================================================================

/// An optional operational budget.
///
/// A bounded quantity is represented as a canonical unsigned decimal
/// string rather than u64, u128, or another fixed-width integer.
///
/// This permits configuration to represent quantities larger than any
/// particular machine integer type.
///
/// The responsible runtime service still needs to measure actual work
/// and compare it against the configured budget.
///
/// `None` means no explicit configuration-level budget.
///
/// `Some("0")` is a valid budget permitting zero consumption.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct BudgetLimit {
    limit: Option<String>,
}

impl BudgetLimit {
    /// Creates a budget without an explicit limit.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            limit: None,
        }
    }

    /// Creates a budget with an explicit decimal limit.
    ///
    /// Accepted values:
    ///
    /// - `0`
    /// - `1`
    /// - `100`
    /// - `184467440737095516160000000000000000000`
    ///
    /// Leading zeroes are rejected except for the value `0`.
    ///
    /// No arbitrary numeric-width ceiling is imposed.
    pub fn bounded(
        value: impl Into<String>,
    ) -> ZurResult<Self> {
        let value = value.into();

        validate_decimal_quantity(&value)?;

        Ok(Self {
            limit: Some(value),
        })
    }

    /// Returns the configured decimal limit.
    ///
    /// Returns `None` when the budget is unlimited.
    #[must_use]
    pub fn limit(&self) -> Option<&str> {
        self.limit.as_deref()
    }

    /// Whether this budget imposes no explicit limit.
    #[must_use]
    pub const fn is_unlimited(&self) -> bool {
        self.limit.is_none()
    }

    /// Whether a measured decimal quantity is within this budget.
    ///
    /// This is a pure comparison.
    ///
    /// It does not:
    ///
    /// - Reserve a resource.
    /// - Accumulate work.
    /// - Track concurrent consumers.
    /// - Enforce the budget.
    /// - Guarantee that a resource is available.
    pub fn permits(
        &self,
        consumed: &str,
    ) -> ZurResult<bool> {
        validate_decimal_quantity(consumed)?;

        let Some(limit) = self.limit.as_deref() else {
            return Ok(true);
        };

        Ok(compare_decimal(consumed, limit) != Ordering::Greater)
    }

    /// Checks consumption and returns an error if the budget is exceeded.
    ///
    /// The caller is responsible for measuring consumption correctly.
    pub fn check(
        &self,
        consumed: &str,
        key: &BudgetKey,
    ) -> ZurResult<()> {
        if self.permits(consumed)? {
            return Ok(());
        }

        Err(
            ZurError::budget_exhausted(
                format!(
                    "operational budget `{key}` exceeded"
                ),
            )
            .with_component("zur.config")
            .with_operation("check_budget"),
        )
    }
}

/// Validates an unsigned canonical decimal quantity.
fn validate_decimal_quantity(
    value: &str,
) -> ZurResult<()> {
    if value.is_empty() {
        return Err(configuration_error(
            "budget quantity must not be empty",
        ));
    }

    if !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(configuration_error(
            "budget quantity must contain decimal digits only",
        ));
    }

    if value.len() > 1 && value.starts_with('0') {
        return Err(configuration_error(
            "budget quantity must not contain leading zeroes",
        ));
    }

    Ok(())
}

/// Compares two validated canonical unsigned decimal quantities.
///
/// This comparison does not parse either quantity into a fixed-width
/// integer.
///
/// Decimal strings with fewer digits are smaller. Strings with the
/// same number of digits can be compared lexicographically.
fn compare_decimal(
    left: &str,
    right: &str,
) -> Ordering {
    match left.len().cmp(&right.len()) {
        Ordering::Equal => left.cmp(right),
        ordering => ordering,
    }
}

// =============================================================================
// SERVICE ACTIVATION POLICY
// =============================================================================

/// Controls when a registered runtime service may be activated.
///
/// This policy does not register, instantiate, or implement a service.
///
/// It is consumed by the runtime service manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum ServiceActivation {
    /// The service must not be activated through this configuration.
    ///
    /// If an execution artifact independently requires this service,
    /// the runtime must report that the required service is unavailable.
    Disabled,

    /// Activate the service when a validated execution requires it.
    ///
    /// This is the default for services without an explicit override.
    #[default]
    OnDemand,

    /// Activate the service during runtime initialization.
    ///
    /// The service manager must still validate registration, version,
    /// dependencies, permissions and initialization success.
    Eager,
}

impl ServiceActivation {
    /// Returns the stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::OnDemand => "on_demand",
            Self::Eager => "eager",
        }
    }
}

impl fmt::Display for ServiceActivation {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// JIT POLICY
// =============================================================================

/// Controls whether ZUR may use just-in-time compilation.
///
/// JIT implementation, compiler integration, executable memory,
/// verification and code-cache ownership belong to `zur::jit`.
///
/// This policy cannot authorize unsafe code or bypass security checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum JitPolicy {
    /// Do not use JIT through this runtime configuration.
    ///
    /// AOT and other independently supported execution paths remain
    /// available when compatible.
    #[default]
    Disabled,

    /// Permit JIT when a compatible implementation is available
    /// and all applicable policies and contracts are satisfied.
    ///
    /// This does not require JIT compilation.
    Allow,
}

impl JitPolicy {
    /// Returns whether JIT may be considered.
    #[must_use]
    pub const fn is_allowed(self) -> bool {
        matches!(self, Self::Allow)
    }
}

// =============================================================================
// EXECUTION FALLBACK POLICY
// =============================================================================

/// Controls fallback to alternative execution realizations.
///
/// Fallback must never silently change program meaning.
///
/// A fallback is valid only when the compiler, execution plan and
/// relevant domain contracts establish that the alternative preserves
/// the required semantics, effects, constraints and guarantees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum ExecutionFallbackPolicy {
    /// Do not select an alternative realization automatically.
    ///
    /// Report unavailable execution paths explicitly.
    Strict,

    /// Permit alternatives that have been independently validated
    /// as semantics-preserving and compliant with the original
    /// requirements and constraints.
    ///
    /// This is the default.
    #[default]
    ValidatedAlternativesOnly,
}

impl ExecutionFallbackPolicy {
    /// Returns whether validated alternatives may be considered.
    #[must_use]
    pub const fn allows_validated_alternatives(self) -> bool {
        matches!(
            self,
            Self::ValidatedAlternativesOnly
        )
    }
}

// =============================================================================
// CANONICAL ZUR CONFIGURATION
// =============================================================================

/// Immutable configuration for one ZUR runtime instance.
///
/// This structure contains operational policies and explicit budgets.
///
/// It does not contain:
///
/// - Mutable resource counters.
/// - Hardware inventories.
/// - Device selections.
/// - Backend registrations.
/// - Physical resource reservations.
/// - Compiler optimization state.
/// - Program semantic requirements.
/// - Execution results.
///
/// Its configuration collections are deterministic.
///
/// All unspecified budget dimensions are unlimited at the configuration
/// policy level. This does not mean that physical resources are unlimited.
///
/// Services default to on-demand activation.
///
/// JIT defaults to disabled.
///
/// Execution fallback defaults to validated alternatives only.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    budgets: BTreeMap<BudgetKey, BudgetLimit>,

    services: BTreeMap<ServiceKey, ServiceActivation>,

    jit: JitPolicy,

    fallback: ExecutionFallbackPolicy,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            budgets: BTreeMap::new(),

            services: BTreeMap::new(),

            jit: JitPolicy::Disabled,

            fallback: ExecutionFallbackPolicy::ValidatedAlternativesOnly,
        }
    }
}

impl RuntimeConfig {
    /// Creates the default runtime configuration.
    ///
    /// Defaults:
    ///
    /// - No explicit operational budgets.
    /// - Runtime services activated on demand.
    /// - JIT disabled.
    /// - Validated semantics-preserving alternatives permitted.
    ///
    /// No hardware is discovered or selected.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // -------------------------------------------------------------------------
    // Budget configuration
    // -------------------------------------------------------------------------

    /// Adds a new operational budget.
    ///
    /// Duplicate keys are rejected to prevent accidental order-dependent
    /// configuration.
    ///
    /// Use `replace_budget` for an intentional replacement.
    pub fn with_budget(
        mut self,
        key: BudgetKey,
        limit: BudgetLimit,
    ) -> ZurResult<Self> {
        if self.budgets.contains_key(&key) {
            return Err(configuration_error(
                format!(
                    "duplicate operational budget `{key}`"
                ),
            ));
        }

        self.budgets.insert(key, limit);

        Ok(self)
    }

    /// Explicitly inserts or replaces an operational budget.
    ///
    /// Returns the previous value when one existed.
    pub fn replace_budget(
        &mut self,
        key: BudgetKey,
        limit: BudgetLimit,
    ) -> Option<BudgetLimit> {
        self.budgets.insert(key, limit)
    }

    /// Returns the configured budget for a dimension.
    ///
    /// An absent entry means that this configuration imposes no
    /// explicit budget for that dimension.
    #[must_use]
    pub fn budget(
        &self,
        key: &BudgetKey,
    ) -> Option<&BudgetLimit> {
        self.budgets.get(key)
    }

    /// Returns all configured budgets in deterministic order.
    #[must_use]
    pub fn budgets(
        &self,
    ) -> &BTreeMap<BudgetKey, BudgetLimit> {
        &self.budgets
    }

    // -------------------------------------------------------------------------
    // Service activation
    // -------------------------------------------------------------------------

    /// Sets the activation policy for a service.
    ///
    /// An explicit policy overrides the default on-demand behavior.
    ///
    /// Service registration and dependency resolution remain the
    /// responsibility of the service registry and service manager.
    ///
    /// Calling this method again for the same service explicitly
    /// replaces the previous policy.
    pub fn with_service(
        mut self,
        key: ServiceKey,
        activation: ServiceActivation,
    ) -> Self {
        self.services.insert(key, activation);

        self
    }

    /// Explicitly replaces a service activation policy.
    ///
    /// Returns the previous policy when one existed.
    pub fn replace_service(
        &mut self,
        key: ServiceKey,
        activation: ServiceActivation,
    ) -> Option<ServiceActivation> {
        self.services.insert(key, activation)
    }

    /// Returns the explicit activation policy for a service.
    ///
    /// Returns `None` when no override exists.
    #[must_use]
    pub fn explicit_service_policy(
        &self,
        key: &ServiceKey,
    ) -> Option<ServiceActivation> {
        self.services.get(key).copied()
    }

    /// Returns the effective activation policy for a service.
    ///
    /// Services without an explicit override default to on-demand.
    #[must_use]
    pub fn service_policy(
        &self,
        key: &ServiceKey,
    ) -> ServiceActivation {
        self.explicit_service_policy(key)
            .unwrap_or_default()
    }

    /// Returns all explicit service activation policies.
    #[must_use]
    pub fn services(
        &self,
    ) -> &BTreeMap<ServiceKey, ServiceActivation> {
        &self.services
    }

    // -------------------------------------------------------------------------
    // JIT policy
    // -------------------------------------------------------------------------

    /// Returns the JIT activation policy.
    #[must_use]
    pub const fn jit_policy(&self) -> JitPolicy {
        self.jit
    }

    /// Sets the JIT activation policy.
    #[must_use]
    pub fn with_jit_policy(
        mut self,
        policy: JitPolicy,
    ) -> Self {
        self.jit = policy;

        self
    }

    // -------------------------------------------------------------------------
    // Execution fallback
    // -------------------------------------------------------------------------

    /// Returns the execution fallback policy.
    #[must_use]
    pub const fn fallback_policy(
        &self,
    ) -> ExecutionFallbackPolicy {
        self.fallback
    }

    /// Sets the execution fallback policy.
    #[must_use]
    pub fn with_fallback_policy(
        mut self,
        policy: ExecutionFallbackPolicy,
    ) -> Self {
        self.fallback = policy;

        self
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Validates configuration invariants.
    ///
    /// The public constructors validate identifiers and budget quantities
    /// before they can enter this structure.
    ///
    /// This method remains an explicit validation boundary for runtime
    /// construction and future configuration extensions.
    ///
    /// It does not check hardware availability, provider registration,
    /// artifact compatibility, resource feasibility or permissions.
    pub fn validate(&self) -> ZurResult<()> {
        for key in self.budgets.keys() {
            validate_namespaced_identifier(
                key.as_str(),
                "budget identifier",
            )?;
        }

        for key in self.services.keys() {
            validate_namespaced_identifier(
                key.as_str(),
                "service identifier",
            )?;
        }

        for limit in self.budgets.values() {
            if let Some(value) = limit.limit() {
                validate_decimal_quantity(value)?;
            }
        }

        Ok(())
    }

    /// Returns whether no explicit operational budgets or service
    /// activation overrides have been configured.
    #[must_use]
    pub fn is_default_configuration(&self) -> bool {
        self.budgets.is_empty()
            && self.services.is_empty()
            && self.jit == JitPolicy::Disabled
            && self.fallback
                == ExecutionFallbackPolicy::ValidatedAlternativesOnly
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn budget_key(value: &str) -> BudgetKey {
        BudgetKey::new(value).expect("valid budget key")
    }

    fn service_key(value: &str) -> ServiceKey {
        ServiceKey::new(value).expect("valid service key")
    }

    // -------------------------------------------------------------------------
    // Identifier validation
    // -------------------------------------------------------------------------

    #[test]
    fn accepts_namespaced_budget_identifiers() {
        assert!(
            BudgetKey::new("core.memory_bytes").is_ok()
        );

        assert!(
            BudgetKey::new("vendor.extension.work_units").is_ok()
        );

        assert!(
            BudgetKey::new("future.domain.custom_resource").is_ok()
        );
    }

    #[test]
    fn rejects_invalid_budget_identifiers() {
        assert!(BudgetKey::new("").is_err());

        assert!(BudgetKey::new("memory").is_err());

        assert!(BudgetKey::new(".memory").is_err());

        assert!(BudgetKey::new("core.").is_err());

        assert!(BudgetKey::new("core..memory").is_err());

        assert!(BudgetKey::new("Core.memory").is_err());

        assert!(BudgetKey::new("core/memory").is_err());

        assert!(BudgetKey::new(" core.memory").is_err());

        assert!(BudgetKey::new("core.memory ").is_err());
    }

    #[test]
    fn accepts_extensible_service_identifiers() {
        assert!(
            ServiceKey::new("core.memory").is_ok()
        );

        assert!(
            ServiceKey::new("vendor.quantum.execution").is_ok()
        );

        assert!(
            ServiceKey::new("future.domain.service").is_ok()
        );
    }

    #[test]
    fn rejects_invalid_service_identifiers() {
        assert!(ServiceKey::new("").is_err());

        assert!(ServiceKey::new("memory").is_err());

        assert!(ServiceKey::new("core..memory").is_err());

        assert!(ServiceKey::new("Core.memory").is_err());
    }

    // -------------------------------------------------------------------------
    // Budget quantities
    // -------------------------------------------------------------------------

    #[test]
    fn creates_unlimited_budget() {
        let budget = BudgetLimit::unlimited();

        assert!(budget.is_unlimited());

        assert_eq!(budget.limit(), None);

        assert!(
            budget.permits("999999999999999999999999").unwrap()
        );
    }

    #[test]
    fn accepts_zero_budget() {
        let budget = BudgetLimit::bounded("0").unwrap();

        assert_eq!(budget.limit(), Some("0"));

        assert!(budget.permits("0").unwrap());

        assert!(!budget.permits("1").unwrap());
    }

    #[test]
    fn validates_decimal_budget_quantities() {
        assert!(BudgetLimit::bounded("1").is_ok());

        assert!(BudgetLimit::bounded("100").is_ok());

        assert!(
            BudgetLimit::bounded(
                "184467440737095516160000000000000000000"
            ).is_ok()
        );

        assert!(BudgetLimit::bounded("").is_err());

        assert!(BudgetLimit::bounded("-1").is_err());

        assert!(BudgetLimit::bounded("+1").is_err());

        assert!(BudgetLimit::bounded("01").is_err());

        assert!(BudgetLimit::bounded("1.5").is_err());

        assert!(BudgetLimit::bounded("1a").is_err());
    }

    #[test]
    fn compares_large_decimal_quantities_without_integer_overflow() {
        let budget = BudgetLimit::bounded(
            "184467440737095516160000000000000000000"
        ).unwrap();

        assert!(
            budget.permits(
                "184467440737095516160000000000000000000"
            ).unwrap()
        );

        assert!(
            budget.permits(
                "184467440737095516159999999999999999999"
            ).unwrap()
        );

        assert!(
            !budget.permits(
                "184467440737095516160000000000000000001"
            ).unwrap()
        );
    }

    #[test]
    fn rejects_invalid_consumption_values() {
        let budget = BudgetLimit::unlimited();

        assert!(budget.permits("").is_err());

        assert!(budget.permits("-1").is_err());

        assert!(budget.permits("01").is_err());
    }

    #[test]
    fn budget_check_reports_exhaustion() {
        let key = budget_key("core.memory_bytes");

        let budget = BudgetLimit::bounded("100").unwrap();

        assert!(budget.check("100", &key).is_ok());

        let error = budget.check("101", &key).unwrap_err();

        assert_eq!(
            error.component(),
            Some("zur.config")
        );

        assert_eq!(
            error.operation(),
            Some("check_budget")
        );
    }

    #[test]
    fn budget_check_accepts_unlimited_consumption() {
        let key = budget_key("core.memory_bytes");

        let budget = BudgetLimit::unlimited();

        assert!(
            budget.check(
                "999999999999999999999999999999",
                &key
            ).is_ok()
        );
    }

    // -------------------------------------------------------------------------
    // Default configuration
    // -------------------------------------------------------------------------

    #[test]
    fn default_configuration_is_deterministic() {
        let first = RuntimeConfig::new();

        let second = RuntimeConfig::new();

        assert_eq!(first, second);

        assert!(first.budgets().is_empty());

        assert!(first.services().is_empty());

        assert_eq!(
            first.jit_policy(),
            JitPolicy::Disabled
        );

        assert_eq!(
            first.fallback_policy(),
            ExecutionFallbackPolicy::ValidatedAlternativesOnly
        );

        assert!(first.is_default_configuration());
    }

    #[test]
    fn unspecified_services_default_to_on_demand() {
        let config = RuntimeConfig::new();

        let service = service_key("core.memory");

        assert_eq!(
            config.service_policy(&service),
            ServiceActivation::OnDemand
        );
    }

    // -------------------------------------------------------------------------
    // Budget configuration
    // -------------------------------------------------------------------------

    #[test]
    fn inserts_and_retrieves_budget() {
        let key = budget_key("core.memory_bytes");

        let budget = BudgetLimit::bounded("4096").unwrap();

        let config = RuntimeConfig::new()
            .with_budget(key.clone(), budget.clone())
            .unwrap();

        assert_eq!(
            config.budget(&key),
            Some(&budget)
        );

        assert_eq!(
            config.budgets().len(),
            1
        );
    }

    #[test]
    fn rejects_duplicate_budget_keys() {
        let key = budget_key("core.memory_bytes");

        let config = RuntimeConfig::new()
            .with_budget(
                key.clone(),
                BudgetLimit::bounded("100").unwrap(),
            )
            .unwrap();

        assert!(
            config.with_budget(
                key,
                BudgetLimit::bounded("200").unwrap(),
            ).is_err()
        );
    }

    #[test]
    fn explicit_budget_replacement_is_supported() {
        let key = budget_key("core.memory_bytes");

        let mut config = RuntimeConfig::new();

        config.replace_budget(
            key.clone(),
            BudgetLimit::bounded("100").unwrap(),
        );

        let previous = config.replace_budget(
            key.clone(),
            BudgetLimit::bounded("200").unwrap(),
        );

        assert_eq!(
            previous.unwrap().limit(),
            Some("100")
        );

        assert_eq!(
            config.budget(&key).unwrap().limit(),
            Some("200")
        );
    }

    // -------------------------------------------------------------------------
    // Service configuration
    // -------------------------------------------------------------------------

    #[test]
    fn service_policy_can_be_disabled() {
        let service = service_key("core.memory");

        let config = RuntimeConfig::new()
            .with_service(
                service.clone(),
                ServiceActivation::Disabled,
            );

        assert_eq!(
            config.service_policy(&service),
            ServiceActivation::Disabled
        );
    }

    #[test]
    fn service_policy_can_be_eager() {
        let service = service_key("core.observability");

        let config = RuntimeConfig::new()
            .with_service(
                service.clone(),
                ServiceActivation::Eager,
            );

        assert_eq!(
            config.service_policy(&service),
            ServiceActivation::Eager
        );
    }

    #[test]
    fn service_policy_overrides_are_independent() {
        let memory = service_key("core.memory");

        let tracing = service_key("core.tracing");

        let config = RuntimeConfig::new()
            .with_service(
                memory.clone(),
                ServiceActivation::Eager,
            )
            .with_service(
                tracing.clone(),
                ServiceActivation::Disabled,
            );

        assert_eq!(
            config.service_policy(&memory),
            ServiceActivation::Eager
        );

        assert_eq!(
            config.service_policy(&tracing),
            ServiceActivation::Disabled
        );
    }

    // -------------------------------------------------------------------------
    // JIT and fallback
    // -------------------------------------------------------------------------

    #[test]
    fn jit_is_disabled_by_default() {
        assert!(
            !RuntimeConfig::new()
                .jit_policy()
                .is_allowed()
        );
    }

    #[test]
    fn jit_can_be_enabled_explicitly() {
        let config = RuntimeConfig::new()
            .with_jit_policy(JitPolicy::Allow);

        assert!(
            config.jit_policy().is_allowed()
        );
    }

    #[test]
    fn strict_fallback_is_available() {
        let config = RuntimeConfig::new()
            .with_fallback_policy(
                ExecutionFallbackPolicy::Strict,
            );

        assert_eq!(
            config.fallback_policy(),
            ExecutionFallbackPolicy::Strict
        );

        assert!(
            !config.fallback_policy()
                .allows_validated_alternatives()
        );
    }

    #[test]
    fn fallback_never_authorizes_semantic_degradation() {
        assert!(
            ExecutionFallbackPolicy::ValidatedAlternativesOnly
                .allows_validated_alternatives()
        );

        // The policy permits only alternatives independently proven
        // to preserve the original execution contract.
        //
        // It does not authorize dropping requirements, effects,
        // constraints, or correctness guarantees.
    }

    // -------------------------------------------------------------------------
    // Configuration validation
    // -------------------------------------------------------------------------

    #[test]
    fn default_configuration_validates() {
        assert!(
            RuntimeConfig::new().validate().is_ok()
        );
    }

    #[test]
    fn configured_runtime_validates() {
        let config = RuntimeConfig::new()
            .with_budget(
                budget_key("core.memory_bytes"),
                BudgetLimit::bounded("8192").unwrap(),
            )
            .unwrap()
            .with_service(
                service_key("core.memory"),
                ServiceActivation::OnDemand,
            )
            .with_jit_policy(JitPolicy::Allow);

        assert!(config.validate().is_ok());
    }

    // -------------------------------------------------------------------------
    // Cloning and equality
    // -------------------------------------------------------------------------

    #[test]
    fn configuration_cloning_preserves_values() {
        let config = RuntimeConfig::new()
            .with_budget(
                budget_key("core.memory_bytes"),
                BudgetLimit::bounded("4096").unwrap(),
            )
            .unwrap();

        let cloned = config.clone();

        assert_eq!(config, cloned);
    }

    #[test]
    fn configuration_equality_is_value_based() {
        let first = RuntimeConfig::new()
            .with_jit_policy(JitPolicy::Allow);

        let second = RuntimeConfig::new()
            .with_jit_policy(JitPolicy::Allow);

        assert_eq!(first, second);
    }
}
