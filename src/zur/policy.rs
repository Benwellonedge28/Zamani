
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Operational Policies
//!
//! Path: src/zur/policy.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97+
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Defines the runtime operational policy model used by the Zamani
//! Universal Runtime (ZUR).
//!
//! This module establishes consistent operational behavior for:
//!
//! - Classical execution.
//! - Quantum execution.
//! - HDL and hardware execution.
//! - AI and tensor execution.
//! - Hybrid execution.
//! - Distributed execution.
//! - Accelerators.
//! - Future computational domains.
//!
//! It provides policy descriptions and policy validation.
//! It does not execute policy actions.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Execution strategy preferences.
//! - Retry policy.
//! - Recovery policy.
//! - Cancellation policy.
//! - Resource exhaustion policy.
//! - Security enforcement policy.
//! - Observability policy.
//! - Runtime adaptation policy.
//! - Extensible operational policy identifiers.
//! - Runtime policy composition.
//!
//! EXISTING AUTHORITIES
//!
//! The following remain authoritative:
//!
//! - config.rs: RuntimeConfig, budgets, JitPolicy,
//!   ExecutionFallbackPolicy and ServiceActivation.
//! - error.rs: ZurError and ZurResult.
//! - ids.rs: Runtime-specific identities.
//! - frontend::semantic: Source-language policy semantics.
//! - frontend::semantic::capabilities: Capability semantics.
//! - frontend::semantic::resources: Resource semantics.
//! - compiler::planning: Planning and feasibility.
//! - backend: Backend and artifact compatibility.
//! - zuir: Universal intermediate representation.
//! - hybrid: Hybrid execution planning.
//! - distributed: Distributed execution.
//! - quantum: Quantum execution and resilience.
//!
//! THIS MODULE DOES NOT OWN
//!
//! - Source grammar or AST.
//! - Semantic requirements.
//! - Capability definitions or evidence.
//! - Resource requirements or quantities.
//! - Constraint evaluation.
//! - Preference scoring during target selection.
//! - Hint interpretation by compilers.
//! - Target feasibility.
//! - Hardware discovery.
//! - Physical resource reservation.
//! - Backend selection.
//! - Quantum routing or QEC.
//! - Distributed scheduling.
//! - Hybrid planning.
//! - Actual resource accounting.
//! - Security authentication or authorization.
//! - Retry execution.
//! - Recovery implementation.
//! - Cancellation implementation.
//! - Logging or metrics implementation.
//! - JIT compilation.
//!
//! These responsibilities belong to their authoritative subsystems.
//!
//! IMPORTANT CONCEPTUAL SEPARATION
//!
//! Requirement:
//!     A mandatory condition of program meaning or realization.
//!
//! Capability:
//!     An ability that an environment may provide.
//!
//! Preference:
//!     A non-mandatory choice among otherwise acceptable alternatives.
//!
//! Constraint:
//!     A restriction on admissible realizations.
//!
//! Hint:
//!     Optional guidance that cannot change program meaning.
//!
//! Feasibility:
//!     An evidence-based assessment of a proposed realization.
//!
//! Resource reservation:
//!     An actual acquisition of resources by the responsible subsystem.
//!
//! Execution result:
//!     An actual outcome produced by execution.
//!
//! Runtime policies must never merge these concepts.
//!
//! SCALABILITY
//!
//! This module imposes no arbitrary limits on:
//!
//! - Computational domains.
//! - Providers.
//! - Devices.
//! - Processes.
//! - Tasks.
//! - Threads.
//! - CPUs.
//! - GPUs.
//! - QPUs.
//! - Memory.
//! - Resource quantities.
//! - Policy extensions.
//! - Policy identifier length.
//! - Namespace depth.
//!
//! Collections grow according to available resources.
//!
//! No finite machine guarantees literal infinite computation.
//!
//! SAFE EXECUTION
//!
//! Policy declarations do not prove that a requested operation is
//! possible, permitted, compatible, or feasible.
//!
//! Every operational action requires the responsible subsystem
//! to validate its own inputs and evidence.
//!
//! An unknown policy extension must never be interpreted as permission.
//!
//! SECURITY
//!
//! Security enforcement is fail-closed.
//!
//! No policy in this module can bypass authentication,
//! authorization, artifact verification, isolation, or provider
//! security requirements.
//!
//! DETERMINISM
//!
//! Ordered collections are used for deterministic policy extension
//! representation.
//!
//! This module does not inspect hardware, access the network,
//! access the filesystem, read environment variables, read the clock,
//! generate random values, or initialize global mutable state.
//!
//! INTEGRATION
//!
//! - core/context.rs consumes RuntimePolicy.
//! - core/builder.rs validates and installs RuntimePolicy.
//! - execution/dispatch.rs consumes execution strategy preferences.
//! - execution/cancellation.rs consumes cancellation policy.
//! - reliability/ consumes recovery policy.
//! - resources/ consumes resource exhaustion policy.
//! - security/ consumes security enforcement policy.
//! - observability/ consumes observability policy.
//! - jit/ consumes strategy preferences together with config::JitPolicy.
//! - config.rs remains the owner of service activation, JIT permission,
//!   execution fallback and operational budgets.
//!
//! Policy enforcement remains in the owning subsystem.

use core::fmt;
use std::collections::BTreeMap;

use crate::zur::config::{
    ExecutionFallbackPolicy,
    JitPolicy,
    RuntimeConfig,
    ServiceActivation,
    ServiceKey,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

// =============================================================================
// IDENTIFIER VALIDATION
// =============================================================================

/// Validates an extensible operational policy identifier.
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
/// There is no arbitrary identifier-length or namespace-depth ceiling.
fn validate_policy_identifier(value: &str) -> ZurResult<()> {
    if value.is_empty() {
        return Err(policy_error(
            "policy identifier must not be empty",
        ));
    }

    if value.trim() != value {
        return Err(policy_error(
            "policy identifier must not contain surrounding whitespace",
        ));
    }

    let mut segment_count = 0usize;

    for segment in value.split('.') {
        if segment.is_empty() {
            return Err(policy_error(
                "policy identifier contains an empty namespace segment",
            ));
        }

        if !segment.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || byte == b'-'
                || byte == b'_'
        }) {
            return Err(policy_error(
                "policy identifier contains an invalid character",
            ));
        }

        if segment.starts_with('-')
            || segment.starts_with('_')
            || segment.ends_with('-')
            || segment.ends_with('_')
        {
            return Err(policy_error(
                "policy identifier has an invalid segment boundary",
            ));
        }

        segment_count += 1;
    }

    if segment_count < 2 {
        return Err(policy_error(
            "policy identifier must contain a namespace and identifier",
        ));
    }

    Ok(())
}

/// Creates a structured operational policy error.
fn policy_error(message: impl Into<String>) -> ZurError {
    ZurError::configuration(message)
        .with_component("zur.policy")
}

// =============================================================================
// EXECUTION STRATEGY PREFERENCE
// =============================================================================

/// Expresses a preference for an execution strategy.
///
/// This is a preference, not a requirement or capability declaration.
///
/// Actual execution strategy selection remains the responsibility
/// of the compiler, validated execution plan and execution dispatcher.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum ExecutionStrategyPreference {
    /// Prefer ahead-of-time compiled execution.
    ///
    /// This does not require a particular native target or backend.
    #[default]
    PreferAot,

    /// Prefer just-in-time execution when permitted and supported.
    ///
    /// This does not enable JIT by itself.
    PreferJit,

    /// Prefer adaptive execution when a compatible implementation
    /// and the required supporting services are available.
    PreferAdaptive,

    /// Accept any compatible execution strategy.
    ///
    /// This does not authorize semantic degradation.
    AnyCompatible,
}

impl ExecutionStrategyPreference {
    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreferAot => "prefer_aot",
            Self::PreferJit => "prefer_jit",
            Self::PreferAdaptive => "prefer_adaptive",
            Self::AnyCompatible => "any_compatible",
        }
    }
}

impl fmt::Display for ExecutionStrategyPreference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// RETRY POLICY
// =============================================================================

/// Controls whether automatic retry may be considered.
///
/// A retry policy does not execute retries.
///
/// Retry execution must additionally consider:
///
/// - Operation idempotency.
/// - Side effects.
/// - Cancellation.
/// - Deadlines.
/// - Resource budgets.
/// - Provider guarantees.
/// - Distributed execution contracts.
///
/// A policy must never turn an unsafe retry into a safe retry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum RetryPolicy {
    /// Do not retry failed operations automatically.
    #[default]
    Never,

    /// Permit retry only when the operation's authoritative
    /// execution contract establishes retry safety.
    WhenIdempotent,

    /// Permit retry when an independent execution contract
    /// explicitly authorizes it.
    ExplicitlyAuthorized,
}

impl RetryPolicy {
    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Never => "never",
            Self::WhenIdempotent => "when_idempotent",
            Self::ExplicitlyAuthorized => "explicitly_authorized",
        }
    }

    /// Whether retry may be considered.
    ///
    /// This does not establish that an individual retry is safe.
    #[must_use]
    pub const fn permits_consideration(self) -> bool {
        !matches!(self, Self::Never)
    }
}

// =============================================================================
// RECOVERY POLICY
// =============================================================================

/// Controls the permitted recovery approach.
///
/// Actual recovery belongs to ZUR reliability and the applicable
/// domain-specific resilience implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum RecoveryPolicy {
    /// Report failure without initiating automatic recovery.
    #[default]
    ReportFailure,

    /// Permit recovery when the responsible provider declares
    /// a compatible recovery operation.
    ProviderDeclared,

    /// Permit recovery using a validated checkpoint when available.
    CheckpointWhenAvailable,

    /// Require a valid checkpoint before checkpoint-based recovery.
    RequireCheckpoint,
}

impl RecoveryPolicy {
    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReportFailure => "report_failure",
            Self::ProviderDeclared => "provider_declared",
            Self::CheckpointWhenAvailable => "checkpoint_when_available",
            Self::RequireCheckpoint => "require_checkpoint",
        }
    }
}

// =============================================================================
// CANCELLATION POLICY
// =============================================================================

/// Controls cancellation expectations.
///
/// Cancellation must not be reported as completed until the
/// execution subsystem establishes the corresponding outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum CancellationPolicy {
    /// Request cooperative cancellation and allow the execution
    /// component to release its resources safely.
    #[default]
    Cooperative,

    /// Request cancellation and require the responsible execution
    /// component to acknowledge the request.
    RequireAcknowledgement,

    /// Do not initiate cancellation automatically.
    ///
    /// Explicit external cancellation requests must still follow
    /// the execution and security contracts.
    ExternalControl,
}

impl CancellationPolicy {
    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cooperative => "cooperative",
            Self::RequireAcknowledgement => "require_acknowledgement",
            Self::ExternalControl => "external_control",
        }
    }
}

// =============================================================================
// RESOURCE EXHAUSTION POLICY
// =============================================================================

/// Defines the operational response to resource exhaustion.
///
/// This policy does not determine resource feasibility or reserve resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum ResourceExhaustionPolicy {
    /// Report exhaustion without silently changing execution semantics.
    #[default]
    Fail,

    /// Permit waiting for resources to become available.
///
/// The caller must supply applicable cancellation and deadline
/// conditions. This policy does not authorize indefinite blocking.
    WaitWithinDeadline,

    /// Report that a new independently validated execution plan
    /// may be required.
    ///
    /// Actual replanning belongs to compiler planning or the
    /// appropriate domain-specific planning subsystem.
    RequestReplanning,
}

impl ResourceExhaustionPolicy {
    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fail => "fail",
            Self::WaitWithinDeadline => "wait_within_deadline",
            Self::RequestReplanning => "request_replanning",
        }
    }
}

// =============================================================================
// SECURITY POLICY
// =============================================================================

/// Controls the minimum runtime security enforcement behavior.
///
/// Security enforcement cannot be disabled through this policy.
///
/// Authentication, authorization, isolation, integrity verification
/// and provider trust remain owned by their respective security systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SecurityEnforcement {
    /// Require applicable security controls before execution.
    ///
    /// This is the only baseline enforcement mode.
    #[default]
    Mandatory,
}

impl SecurityEnforcement {
    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mandatory => "mandatory",
        }
    }
}

/// Defines how unresolved security decisions are handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SecurityUncertaintyPolicy {
    /// Deny operations when required security evidence is missing,
    /// incomplete or contradictory.
    #[default]
    Deny,

    /// Require an independent authorized security decision.
    RequireExternalDecision,
}

impl SecurityUncertaintyPolicy {
    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deny => "deny",
            Self::RequireExternalDecision => "require_external_decision",
        }
    }
}

/// Runtime security policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SecurityPolicy {
    enforcement: SecurityEnforcement,
    uncertainty: SecurityUncertaintyPolicy,
}

impl SecurityPolicy {
    /// Creates the mandatory, fail-closed security policy.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            enforcement: SecurityEnforcement::Mandatory,
            uncertainty: SecurityUncertaintyPolicy::Deny,
        }
    }

    /// Returns the enforcement mode.
    #[must_use]
    pub const fn enforcement(self) -> SecurityEnforcement {
        self.enforcement
    }

    /// Returns the uncertainty policy.
    #[must_use]
    pub const fn uncertainty(self) -> SecurityUncertaintyPolicy {
        self.uncertainty
    }

    /// Requires an independent security decision when evidence
    /// is insufficient.
    #[must_use]
    pub const fn with_external_decision(mut self) -> Self {
        self.uncertainty =
            SecurityUncertaintyPolicy::RequireExternalDecision;

        self
    }

    /// Validates the security invariants.
    pub fn validate(self) -> ZurResult<()> {
        if self.enforcement != SecurityEnforcement::Mandatory {
            return Err(policy_error(
                "runtime security enforcement cannot be disabled",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// OBSERVABILITY POLICY
// =============================================================================

/// Controls runtime diagnostic detail.
///
/// This policy describes the requested diagnostic level.
///
/// It does not implement logging, metrics, tracing or profiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum ObservabilityPolicy {
    /// Emit only required operational errors and failures.
    #[default]
    ErrorsOnly,

    /// Permit standard operational diagnostics.
    Standard,

    /// Permit detailed diagnostic collection.
    ///
    /// Sensitive values must still follow security and privacy rules.
    Detailed,

    /// Disable optional diagnostic collection.
    ///
    /// Mandatory security and correctness events cannot be suppressed.
    Disabled,
}

impl ObservabilityPolicy {
    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ErrorsOnly => "errors_only",
            Self::Standard => "standard",
            Self::Detailed => "detailed",
            Self::Disabled => "disabled",
        }
    }
}

// =============================================================================
// ADAPTATION POLICY
// =============================================================================

/// Controls whether runtime adaptation may be considered.
///
/// Adaptation may include compatible specialization, execution strategy
/// changes and validated operational adjustments.
///
/// It must never change established program meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum AdaptationPolicy {
    /// Do not initiate optional runtime adaptation.
    #[default]
    Disabled,

    /// Permit only independently validated adaptations that preserve
    /// semantic requirements, constraints and execution guarantees.
    ValidatedAlternativesOnly,
}

impl AdaptationPolicy {
    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::ValidatedAlternativesOnly => "validated_alternatives_only",
        }
    }

    /// Whether validated adaptation may be considered.
    #[must_use]
    pub const fn permits_validated_adaptation(self) -> bool {
        matches!(
            self,
            Self::ValidatedAlternativesOnly
        )
    }
}

// =============================================================================
// EXTENSIBLE POLICY IDENTIFIERS
// =============================================================================

/// Identifies an extension-defined runtime operational policy.
///
/// The identifier is namespaced and open-ended.
///
/// Examples:
///
/// - `vendor.runtime.custom_execution`
/// - `future.domain.adaptation`
/// - `extension.security.additional_validation`
///
/// The identifier alone does not define policy semantics.
///
/// An extension must provide its own validated interpretation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyKey(String);

impl PolicyKey {
    /// Constructs a validated namespaced policy identifier.
    pub fn new(value: impl Into<String>) -> ZurResult<Self> {
        let value = value.into();

        validate_policy_identifier(&value)?;

        Ok(Self(value))
    }

    /// Returns the canonical identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for PolicyKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// An opaque extension-defined policy value.
///
/// ZUR does not interpret arbitrary extension values.
///
/// The registered extension must validate and interpret its own value.
/// Unknown values must never grant permissions or bypass mandatory checks.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PolicyValue(String);

impl PolicyValue {
    /// Creates an opaque, non-empty policy value.
    pub fn new(value: impl Into<String>) -> ZurResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(policy_error(
                "extension policy value must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the opaque value.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the opaque value.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

// =============================================================================
// CANONICAL RUNTIME POLICY
// =============================================================================

/// Immutable operational policies for one ZUR runtime instance.
///
/// RuntimePolicy is deliberately separate from RuntimeConfig.
///
/// RuntimeConfig owns:
///
/// - Operational budgets.
/// - Service activation.
/// - JIT permission.
/// - Execution fallback configuration.
///
/// RuntimePolicy owns:
///
/// - Execution strategy preference.
/// - Retry behavior.
/// - Recovery behavior.
/// - Cancellation behavior.
/// - Resource exhaustion response.
/// - Security enforcement expectations.
/// - Observability preferences.
/// - Runtime adaptation permission.
/// - Extension-defined policy metadata.
///
/// This separation prevents configuration and operational policy
/// from becoming competing authorities.
///
/// Defaults are deliberately conservative:
///
/// - Prefer AOT.
/// - Never automatically retry.
/// - Report failure rather than automatically recover.
/// - Use cooperative cancellation.
/// - Fail on resource exhaustion.
/// - Enforce mandatory security.
/// - Deny unresolved security decisions.
/// - Collect errors only.
/// - Disable optional adaptation.
///
/// No hardware is discovered or selected by this structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimePolicy {
    execution: ExecutionStrategyPreference,

    retry: RetryPolicy,

    recovery: RecoveryPolicy,

    cancellation: CancellationPolicy,

    resource_exhaustion: ResourceExhaustionPolicy,

    security: SecurityPolicy,

    observability: ObservabilityPolicy,

    adaptation: AdaptationPolicy,

    extensions: BTreeMap<PolicyKey, PolicyValue>,
}

impl Default for RuntimePolicy {
    fn default() -> Self {
        Self {
            execution: ExecutionStrategyPreference::PreferAot,

            retry: RetryPolicy::Never,

            recovery: RecoveryPolicy::ReportFailure,

            cancellation: CancellationPolicy::Cooperative,

            resource_exhaustion: ResourceExhaustionPolicy::Fail,

            security: SecurityPolicy::new(),

            observability: ObservabilityPolicy::ErrorsOnly,

            adaptation: AdaptationPolicy::Disabled,

            extensions: BTreeMap::new(),
        }
    }
}

impl RuntimePolicy {
    /// Creates the default runtime policy.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // -------------------------------------------------------------------------
    // Execution strategy
    // -------------------------------------------------------------------------

    /// Returns the preferred execution strategy.
    #[must_use]
    pub const fn execution_preference(
        &self,
    ) -> ExecutionStrategyPreference {
        self.execution
    }

    /// Sets the preferred execution strategy.
    ///
    /// This does not enable JIT, select a backend or establish
    /// execution compatibility.
    #[must_use]
    pub fn with_execution_preference(
        mut self,
        preference: ExecutionStrategyPreference,
    ) -> Self {
        self.execution = preference;

        self
    }

    // -------------------------------------------------------------------------
    // Retry
    // -------------------------------------------------------------------------

    /// Returns the retry policy.
    #[must_use]
    pub const fn retry_policy(&self) -> RetryPolicy {
        self.retry
    }

    /// Sets the retry policy.
    #[must_use]
    pub fn with_retry_policy(
        mut self,
        policy: RetryPolicy,
    ) -> Self {
        self.retry = policy;

        self
    }

    // -------------------------------------------------------------------------
    // Recovery
    // -------------------------------------------------------------------------

    /// Returns the recovery policy.
    #[must_use]
    pub const fn recovery_policy(&self) -> RecoveryPolicy {
        self.recovery
    }

    /// Sets the recovery policy.
    #[must_use]
    pub fn with_recovery_policy(
        mut self,
        policy: RecoveryPolicy,
    ) -> Self {
        self.recovery = policy;

        self
    }

    // -------------------------------------------------------------------------
    // Cancellation
    // -------------------------------------------------------------------------

    /// Returns the cancellation policy.
    #[must_use]
    pub const fn cancellation_policy(&self) -> CancellationPolicy {
        self.cancellation
    }

    /// Sets the cancellation policy.
    #[must_use]
    pub fn with_cancellation_policy(
        mut self,
        policy: CancellationPolicy,
    ) -> Self {
        self.cancellation = policy;

        self
    }

    // -------------------------------------------------------------------------
    // Resource exhaustion
    // -------------------------------------------------------------------------

    /// Returns the resource exhaustion policy.
    #[must_use]
    pub const fn resource_exhaustion_policy(
        &self,
    ) -> ResourceExhaustionPolicy {
        self.resource_exhaustion
    }

    /// Sets the resource exhaustion policy.
    #[must_use]
    pub fn with_resource_exhaustion_policy(
        mut self,
        policy: ResourceExhaustionPolicy,
    ) -> Self {
        self.resource_exhaustion = policy;

        self
    }

    // -------------------------------------------------------------------------
    // Security
    // -------------------------------------------------------------------------

    /// Returns the security policy.
    #[must_use]
    pub const fn security_policy(&self) -> SecurityPolicy {
        self.security
    }

    /// Sets the security policy.
    #[must_use]
    pub fn with_security_policy(
        mut self,
        policy: SecurityPolicy,
    ) -> Self {
        self.security = policy;

        self
    }

    // -------------------------------------------------------------------------
    // Observability
    // -------------------------------------------------------------------------

    /// Returns the observability policy.
    #[must_use]
    pub const fn observability_policy(
        &self,
    ) -> ObservabilityPolicy {
        self.observability
    }

    /// Sets the observability policy.
    #[must_use]
    pub fn with_observability_policy(
        mut self,
        policy: ObservabilityPolicy,
    ) -> Self {
        self.observability = policy;

        self
    }

    // -------------------------------------------------------------------------
    // Adaptation
    // -------------------------------------------------------------------------

    /// Returns the adaptation policy.
    #[must_use]
    pub const fn adaptation_policy(
        &self,
    ) -> AdaptationPolicy {
        self.adaptation
    }

    /// Sets the adaptation policy.
    #[must_use]
    pub fn with_adaptation_policy(
        mut self,
        policy: AdaptationPolicy,
    ) -> Self {
        self.adaptation = policy;

        self
    }

    // -------------------------------------------------------------------------
    // Extension policies
    // -------------------------------------------------------------------------

    /// Inserts an extension-defined policy.
    ///
    /// Duplicate keys are rejected to prevent accidental,
    /// order-dependent configuration.
    pub fn with_extension(
        mut self,
        key: PolicyKey,
        value: PolicyValue,
    ) -> ZurResult<Self> {
        if self.extensions.contains_key(&key) {
            return Err(policy_error(
                format!("duplicate extension policy `{key}`"),
            ));
        }

        self.extensions.insert(key, value);

        Ok(self)
    }

    /// Explicitly replaces an extension-defined policy.
    ///
    /// Returns the previous value when one existed.
    pub fn replace_extension(
        &mut self,
        key: PolicyKey,
        value: PolicyValue,
    ) -> Option<PolicyValue> {
        self.extensions.insert(key, value)
    }

    /// Returns an extension policy by identity.
    #[must_use]
    pub fn extension(
        &self,
        key: &PolicyKey,
    ) -> Option<&PolicyValue> {
        self.extensions.get(key)
    }

    /// Returns all extension policies in deterministic order.
    #[must_use]
    pub fn extensions(
        &self,
    ) -> &BTreeMap<PolicyKey, PolicyValue> {
        &self.extensions
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Validates all runtime policy invariants.
    ///
    /// This does not evaluate source-language policy expressions,
    /// determine feasibility, inspect hardware or authorize operations.
    pub fn validate(&self) -> ZurResult<()> {
        self.security.validate()?;

        for key in self.extensions.keys() {
            validate_policy_identifier(key.as_str())?;
        }

        for value in self.extensions.values() {
            if value.as_str().trim().is_empty() {
                return Err(policy_error(
                    "extension policy contains an empty value",
                ));
            }
        }

        Ok(())
    }

    /// Returns whether this is the default runtime policy.
    #[must_use]
    pub fn is_default_policy(&self) -> bool {
        self == &Self::default()
    }
}

// =============================================================================
// CONFIGURATION INTEGRATION
// =============================================================================

/// Provides a validated view of the combined runtime configuration.
///
/// This is an integration boundary, not another configuration owner.
///
/// RuntimeConfig remains authoritative for budgets, service activation,
/// JIT permission and fallback behavior.
///
/// RuntimePolicy remains authoritative for operational policies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveRuntimePolicy {
    runtime: RuntimePolicy,
    configuration: RuntimeConfig,
}

impl EffectiveRuntimePolicy {
    /// Creates a combined policy after validating both inputs.
    ///
    /// No hardware inspection, resource reservation, service startup
    /// or execution occurs here.
    pub fn new(
        runtime: RuntimePolicy,
        configuration: RuntimeConfig,
    ) -> ZurResult<Self> {
        runtime.validate()?;
        configuration.validate()?;

        Ok(Self {
            runtime,
            configuration,
        })
    }

    /// Returns the operational policy.
    #[must_use]
    pub fn runtime_policy(&self) -> &RuntimePolicy {
        &self.runtime
    }

    /// Returns the canonical runtime configuration.
    #[must_use]
    pub fn configuration(&self) -> &RuntimeConfig {
        &self.configuration
    }

    /// Returns the configured JIT permission.
    ///
    /// JIT permission and JIT preference are different.
    ///
    /// A preference for JIT cannot override a disabled JIT configuration.
    #[must_use]
    pub const fn jit_policy(&self) -> JitPolicy {
        self.configuration.jit_policy()
    }

    /// Returns the configured fallback policy.
    #[must_use]
    pub const fn fallback_policy(&self) -> ExecutionFallbackPolicy {
        self.configuration.fallback_policy()
    }

    /// Returns the effective service activation policy.
    ///
    /// An unspecified service remains on-demand according to
    /// RuntimeConfig's existing contract.
    #[must_use]
    pub fn service_activation(
        &self,
        service: &ServiceKey,
    ) -> ServiceActivation {
        self.configuration.service_policy(service)
    }

    /// Returns the combined runtime execution preference.
    ///
    /// This is only a policy description.
    ///
    /// The execution dispatcher must still validate:
    ///
    /// - Artifact compatibility.
    /// - JIT permission.
    /// - Available implementations.
    /// - Capability evidence.
    /// - Semantic requirements.
    /// - Constraints.
    /// - Security authorization.
    #[must_use]
    pub const fn execution_preference(
        &self,
    ) -> ExecutionStrategyPreference {
        self.runtime.execution_preference()
    }

    /// Validates the complete policy composition.
    pub fn validate(&self) -> ZurResult<()> {
        self.runtime.validate()?;
        self.configuration.validate()?;

        Ok(())
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn extension_key(value: &str) -> PolicyKey {
        PolicyKey::new(value).expect("valid policy key")
    }

    fn extension_value(value: &str) -> PolicyValue {
        PolicyValue::new(value).expect("valid policy value")
    }

    // -------------------------------------------------------------------------
    // Defaults
    // -------------------------------------------------------------------------

    #[test]
    fn default_policy_is_conservative() {
        let policy = RuntimePolicy::new();

        assert_eq!(
            policy.execution_preference(),
            ExecutionStrategyPreference::PreferAot
        );

        assert_eq!(
            policy.retry_policy(),
            RetryPolicy::Never
        );

        assert_eq!(
            policy.recovery_policy(),
            RecoveryPolicy::ReportFailure
        );

        assert_eq!(
            policy.cancellation_policy(),
            CancellationPolicy::Cooperative
        );

        assert_eq!(
            policy.resource_exhaustion_policy(),
            ResourceExhaustionPolicy::Fail
        );

        assert_eq!(
            policy.security_policy(),
            SecurityPolicy::new()
        );

        assert_eq!(
            policy.observability_policy(),
            ObservabilityPolicy::ErrorsOnly
        );

        assert_eq!(
            policy.adaptation_policy(),
            AdaptationPolicy::Disabled
        );

        assert!(policy.is_default_policy());
    }

    // -------------------------------------------------------------------------
    // Execution strategy
    // -------------------------------------------------------------------------

    #[test]
    fn execution_strategy_preference_is_configurable() {
        let policy = RuntimePolicy::new()
            .with_execution_preference(
                ExecutionStrategyPreference::PreferJit
            );

        assert_eq!(
            policy.execution_preference(),
            ExecutionStrategyPreference::PreferJit
        );
    }

    #[test]
    fn strategy_preference_does_not_enable_jit() {
        let runtime = RuntimePolicy::new()
            .with_execution_preference(
                ExecutionStrategyPreference::PreferJit
            );

        let effective = EffectiveRuntimePolicy::new(
            runtime,
            RuntimeConfig::new(),
        ).unwrap();

        assert_eq!(
            effective.execution_preference(),
            ExecutionStrategyPreference::PreferJit
        );

        assert_eq!(
            effective.jit_policy(),
            JitPolicy::Disabled
        );
    }

    // -------------------------------------------------------------------------
    // Retry and recovery
    // -------------------------------------------------------------------------

    #[test]
    fn retry_defaults_to_never() {
        assert_eq!(
            RuntimePolicy::new().retry_policy(),
            RetryPolicy::Never
        );
    }

    #[test]
    fn retry_requires_explicit_policy() {
        let policy = RuntimePolicy::new()
            .with_retry_policy(
                RetryPolicy::WhenIdempotent
            );

        assert_eq!(
            policy.retry_policy(),
            RetryPolicy::WhenIdempotent
        );
    }

    #[test]
    fn retry_permission_is_not_retry_execution() {
        assert!(
            RetryPolicy::WhenIdempotent
                .permits_consideration()
        );

        assert!(
            !RetryPolicy::Never
                .permits_consideration()
        );
    }

    #[test]
    fn recovery_defaults_to_reporting_failure() {
        assert_eq!(
            RuntimePolicy::new().recovery_policy(),
            RecoveryPolicy::ReportFailure
        );
    }

    // -------------------------------------------------------------------------
    // Cancellation and resource exhaustion
    // -------------------------------------------------------------------------

    #[test]
    fn cancellation_defaults_to_cooperative() {
        assert_eq!(
            RuntimePolicy::new().cancellation_policy(),
            CancellationPolicy::Cooperative
        );
    }

    #[test]
    fn resource_exhaustion_defaults_to_failure() {
        assert_eq!(
            RuntimePolicy::new().resource_exhaustion_policy(),
            ResourceExhaustionPolicy::Fail
        );
    }

    #[test]
    fn resource_exhaustion_policy_is_configurable() {
        let policy = RuntimePolicy::new()
            .with_resource_exhaustion_policy(
                ResourceExhaustionPolicy::RequestReplanning
            );

        assert_eq!(
            policy.resource_exhaustion_policy(),
            ResourceExhaustionPolicy::RequestReplanning
        );
    }

    // -------------------------------------------------------------------------
    // Security
    // -------------------------------------------------------------------------

    #[test]
    fn security_is_mandatory_by_default() {
        let security = SecurityPolicy::new();

        assert_eq!(
            security.enforcement(),
            SecurityEnforcement::Mandatory
        );

        assert_eq!(
            security.uncertainty(),
            SecurityUncertaintyPolicy::Deny
        );

        assert!(security.validate().is_ok());
    }

    #[test]
    fn security_can_require_external_decision() {
        let security = SecurityPolicy::new()
            .with_external_decision();

        assert_eq!(
            security.uncertainty(),
            SecurityUncertaintyPolicy::RequireExternalDecision
        );

        assert!(security.validate().is_ok());
    }

    // -------------------------------------------------------------------------
    // Observability and adaptation
    // -------------------------------------------------------------------------

    #[test]
    fn observability_defaults_to_errors_only() {
        assert_eq!(
            RuntimePolicy::new().observability_policy(),
            ObservabilityPolicy::ErrorsOnly
        );
    }

    #[test]
    fn adaptation_is_disabled_by_default() {
        assert_eq!(
            RuntimePolicy::new().adaptation_policy(),
            AdaptationPolicy::Disabled
        );

        assert!(
            !AdaptationPolicy::Disabled
                .permits_validated_adaptation()
        );
    }

    #[test]
    fn validated_adaptation_can_be_permitted() {
        let policy = RuntimePolicy::new()
            .with_adaptation_policy(
                AdaptationPolicy::ValidatedAlternativesOnly
            );

        assert!(
            policy.adaptation_policy()
                .permits_validated_adaptation()
        );
    }

    // -------------------------------------------------------------------------
    // Extension identifiers
    // -------------------------------------------------------------------------

    #[test]
    fn accepts_extensible_policy_identifiers() {
        assert!(PolicyKey::new(
            "vendor.runtime.custom_execution"
        ).is_ok());

        assert!(PolicyKey::new(
            "future.domain.adaptation"
        ).is_ok());

        assert!(PolicyKey::new(
            "extension.security.additional_validation"
        ).is_ok());
    }

    #[test]
    fn rejects_invalid_policy_identifiers() {
        assert!(PolicyKey::new("").is_err());

        assert!(PolicyKey::new("policy").is_err());

        assert!(PolicyKey::new(".runtime.policy").is_err());

        assert!(PolicyKey::new("runtime.").is_err());

        assert!(PolicyKey::new("runtime..policy").is_err());

        assert!(PolicyKey::new("Runtime.policy").is_err());

        assert!(PolicyKey::new("runtime/policy").is_err());

        assert!(PolicyKey::new(" runtime.policy").is_err());
    }

    #[test]
    fn accepts_arbitrarily_deep_namespace_structure() {
        let identifier = format!(
            "{}.policy",
            vec!["extension"; 128].join(".")
        );

        assert!(PolicyKey::new(identifier).is_ok());
    }

    #[test]
    fn rejects_empty_extension_values() {
        assert!(PolicyValue::new("").is_err());

        assert!(PolicyValue::new("   ").is_err());
    }

    #[test]
    fn extension_registration_is_deterministic() {
        let first = RuntimePolicy::new()
            .with_extension(
                extension_key("vendor.runtime.alpha"),
                extension_value("enabled"),
            )
            .unwrap()
            .with_extension(
                extension_key("vendor.runtime.beta"),
                extension_value("validated"),
            )
            .unwrap();

        let second = RuntimePolicy::new()
            .with_extension(
                extension_key("vendor.runtime.beta"),
                extension_value("validated"),
            )
            .unwrap()
            .with_extension(
                extension_key("vendor.runtime.alpha"),
                extension_value("enabled"),
            )
            .unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn duplicate_extensions_are_rejected() {
        let key = extension_key("vendor.runtime.policy");

        let policy = RuntimePolicy::new()
            .with_extension(
                key.clone(),
                extension_value("enabled"),
            )
            .unwrap();

        assert!(
            policy.with_extension(
                key,
                extension_value("disabled"),
            ).is_err()
        );
    }

    #[test]
    fn explicit_extension_replacement_is_supported() {
        let key = extension_key("vendor.runtime.policy");

        let mut policy = RuntimePolicy::new();

        policy.replace_extension(
            key.clone(),
            extension_value("enabled"),
        );

        let previous = policy.replace_extension(
            key.clone(),
            extension_value("validated"),
        );

        assert_eq!(
            previous.unwrap().as_str(),
            "enabled"
        );

        assert_eq!(
            policy.extension(&key).unwrap().as_str(),
            "validated"
        );
    }

    // -------------------------------------------------------------------------
    // Configuration integration
    // -------------------------------------------------------------------------

    #[test]
    fn effective_policy_validates_both_inputs() {
        let effective = EffectiveRuntimePolicy::new(
            RuntimePolicy::new(),
            RuntimeConfig::new(),
        );

        assert!(effective.is_ok());
    }

    #[test]
    fn effective_policy_preserves_configuration_ownership() {
        let effective = EffectiveRuntimePolicy::new(
            RuntimePolicy::new(),
            RuntimeConfig::new()
                .with_jit_policy(JitPolicy::Allow),
        ).unwrap();

        assert_eq!(
            effective.jit_policy(),
            JitPolicy::Allow
        );
    }

    #[test]
    fn effective_policy_preserves_fallback_configuration() {
        let effective = EffectiveRuntimePolicy::new(
            RuntimePolicy::new(),
            RuntimeConfig::new()
                .with_fallback_policy(
                    ExecutionFallbackPolicy::Strict
                ),
        ).unwrap();

        assert_eq!(
            effective.fallback_policy(),
            ExecutionFallbackPolicy::Strict
        );
    }

    #[test]
    fn effective_policy_preserves_service_activation() {
        let service = ServiceKey::new("core.memory").unwrap();

        let effective = EffectiveRuntimePolicy::new(
            RuntimePolicy::new(),
            RuntimeConfig::new()
                .with_service(
                    service.clone(),
                    ServiceActivation::Disabled,
                ),
        ).unwrap();

        assert_eq!(
            effective.service_activation(&service),
            ServiceActivation::Disabled
        );
    }

    #[test]
    fn unspecified_service_remains_on_demand() {
        let service = ServiceKey::new("core.memory").unwrap();

        let effective = EffectiveRuntimePolicy::new(
            RuntimePolicy::new(),
            RuntimeConfig::new(),
        ).unwrap();

        assert_eq!(
            effective.service_activation(&service),
            ServiceActivation::OnDemand
        );
    }

    // -------------------------------------------------------------------------
    // Validation and cloning
    // -------------------------------------------------------------------------

    #[test]
    fn default_policy_validates() {
        assert!(RuntimePolicy::new().validate().is_ok());
    }

    #[test]
    fn configured_policy_validates() {
        let policy = RuntimePolicy::new()
            .with_execution_preference(
                ExecutionStrategyPreference::PreferAdaptive
            )
            .with_retry_policy(
                RetryPolicy::WhenIdempotent
            )
            .with_recovery_policy(
                RecoveryPolicy::CheckpointWhenAvailable
            )
            .with_cancellation_policy(
                CancellationPolicy::RequireAcknowledgement
            )
            .with_resource_exhaustion_policy(
                ResourceExhaustionPolicy::WaitWithinDeadline
            )
            .with_observability_policy(
                ObservabilityPolicy::Standard
            )
            .with_adaptation_policy(
                AdaptationPolicy::ValidatedAlternativesOnly
            );

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn policy_cloning_preserves_all_values() {
        let policy = RuntimePolicy::new()
            .with_execution_preference(
                ExecutionStrategyPreference::AnyCompatible
            )
            .with_extension(
                extension_key("vendor.runtime.custom"),
                extension_value("active"),
            )
            .unwrap();

        assert_eq!(policy, policy.clone());
    }

    #[test]
    fn policy_equality_is_value_based() {
        let first = RuntimePolicy::new()
            .with_retry_policy(RetryPolicy::WhenIdempotent);

        let second = RuntimePolicy::new()
            .with_retry_policy(RetryPolicy::WhenIdempotent);

        assert_eq!(first, second);
    }
}
