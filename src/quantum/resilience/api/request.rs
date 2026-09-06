//! Zamani Quantum Resilience — Request Contract
//!
//! Path:
//!     src/quantum/resilience/api/request.rs
//!
//! Purpose:
//!     Defines the immutable, provider-independent request submitted to the
//!     quantum resilience orchestration layer.
//!
//! Architectural role:
//!     `ResilienceRequest` describes WHAT the caller wants protected.
//!     It does not describe HOW the computation is mapped, scheduled,
//!     optimized, error-corrected, mitigated, or executed.
//!
//! The request therefore deliberately does NOT own:
//!
//! - quantum IR semantics;
//! - quantum gates;
//! - quantum operations;
//! - physical hardware;
//! - backend/provider identity;
//! - routing algorithms;
//! - scheduling algorithms;
//! - optimization algorithms;
//! - QEC algorithms;
//! - fault ontologies;
//! - detector implementations;
//! - recovery implementations;
//! - mitigation implementations;
//! - credentials;
//! - network clients;
//! - filesystem state;
//! - global mutable state.
//!
//! Those responsibilities belong to their authoritative subsystems.
//!
//! # Core lifecycle
//!
//! ```text
//! Zamani Program
//!       |
//!       v
//! ResilienceRequest
//!       |
//!       v
//! ResilienceController
//!       |
//!       +--> Detection
//!       +--> Diagnosis
//!       +--> Policy
//!       +--> Planning
//!       +--> Adaptation
//!       +--> Recovery
//!       +--> Mitigation
//!       +--> Verification
//!       |
//!       v
//! ResilienceResponse
//! ```
//!
//! # Write once, scale everywhere
//!
//! A request MUST NOT contain assumptions such as:
//!
//! ```text
//! exactly 127 qubits
//! exactly 1000 qubits
//! backend X
//! physical qubit 7
//! retry three times
//! fidelity < 0.99
//! gate duration = 100 ns
//! ```
//!
//! Such values are target capabilities, execution policies, or runtime
//! observations and belong to the appropriate subsystem.
//!
//! The request may contain caller-declared requirements and budgets, but those
//! must always be explicit values supplied for the current invocation rather
//! than architectural constants.
//!
//! # Canonical quantum identity
//!
//! Logical qubit scoping uses:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! This module MUST NOT define another logical or physical qubit identifier.
//!
//! Physical qubit identities are intentionally not normally accepted by the
//! public request. Physical placement belongs to routing and hardware.
//!
//! # Determinism
//!
//! Strict deterministic mode is explicit.
//!
//! The request never reads:
//!
//! - wall-clock time;
//! - environment variables;
//! - process IDs;
//! - thread IDs;
//! - filesystem ordering;
//! - network state;
//! - global mutable state.
//!
//! If deterministic randomness is needed, the caller supplies an explicit
//! seed. The controller/context is responsible for incorporating all other
//! deterministic inputs such as hardware snapshots, policy versions, and
//! strategy versions into the reproducibility boundary.
//!
//! # Security
//!
//! Credentials, authentication material, provider tokens, private keys and
//! secrets MUST NOT be stored in this request.
//!
//! Authentication and authorization belong to the hardware/runtime/security
//! integration contracts.
//!
//! # Rust contract
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust only
//! - no `unsafe`
//! - no hard-coded machine-size limits
//! - no hidden I/O
//! - no hidden concurrency
//! - no hidden retry loops
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::num::NonZeroU64;
use std::sync::Arc;
use std::time::Duration;

use crate::quantum::ir::program::QuantumProgram;
use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Public schema
// =============================================================================

/// Stable schema identifier for the resilience request contract.
pub const RESILIENCE_REQUEST_SCHEMA_ID: &str =
    "zamani.quantum.resilience.api.request";

/// Semantic version of the request contract.
///
/// This value changes when the externally observable request schema changes.
/// It is deliberately independent of the Zamani IR version.
pub const RESILIENCE_REQUEST_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Request mode
// =============================================================================

/// Controls how strongly reproducibility and environmental determinism are
/// enforced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ResilienceExecutionMode {
    /// Every deterministic input required for the requested decision must be
    /// explicitly bound by the execution context.
    ///
    /// Missing deterministic inputs must cause a deterministic failure rather
    /// than an implicit downgrade.
    StrictDeterministic,

    /// Deterministic inputs are recorded and replayable, while explicitly
    /// declared environmental differences are allowed.
    Reproducible,

    /// The resilience system may use the supplied environment normally, while
    /// still preserving provenance and avoiding hidden state.
    #[default]
    BestEffort,
}

impl ResilienceExecutionMode {
    /// Returns the stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StrictDeterministic => "strict_deterministic",
            Self::Reproducible => "reproducible",
            Self::BestEffort => "best_effort",
        }
    }

    /// Returns whether all deterministic inputs must be explicitly bound.
    pub const fn requires_complete_deterministic_inputs(self) -> bool {
        matches!(self, Self::StrictDeterministic)
    }

    /// Returns whether the execution must produce replayable decision inputs.
    pub const fn requires_replayable_inputs(self) -> bool {
        matches!(
            self,
            Self::StrictDeterministic | Self::Reproducible
        )
    }
}

impl fmt::Display for ResilienceExecutionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Semantic acceptance
// =============================================================================

/// Defines the minimum semantic acceptance guarantee requested by the caller.
///
/// This is intentionally a requirement, not a verification algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SemanticGuarantee {
    /// A result may only be accepted when the applicable semantic verification
    /// contract succeeds.
    #[default]
    Strict,

    /// A degraded result may be accepted when the verification subsystem
    /// explicitly establishes that the declared degradation remains within the
    /// requested contract.
    AllowVerifiedDegradation,
}

impl SemanticGuarantee {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::AllowVerifiedDegradation => "allow_verified_degradation",
        }
    }

    /// Returns whether degraded acceptance is permitted.
    pub const fn permits_degraded_acceptance(self) -> bool {
        matches!(self, Self::AllowVerifiedDegradation)
    }
}

impl fmt::Display for SemanticGuarantee {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Resilience scope
// =============================================================================

/// Defines the logical portion of a program for which resilience is being
/// requested.
///
/// The default is the complete program.
///
/// This is deliberately expressed in terms of canonical logical qubit
/// identities only. Physical placement remains the responsibility of routing
/// and hardware.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ResilienceScope {
    /// Protect the complete submitted quantum program.
    #[default]
    Program,

    /// Protect the program execution associated with an explicitly supplied
    /// set of logical qubits.
    ///
    /// The identities are canonical `quantum::ir::qubit::QubitId` values.
    LogicalQubits(Arc<[QubitId]>),
}

impl ResilienceScope {
    /// Creates a whole-program scope.
    pub const fn program() -> Self {
        Self::Program
    }

    /// Creates a logical-qubit scope.
    ///
    /// The caller is responsible for supplying canonical logical qubit IDs.
    /// No fixed number of qubits is assumed.
    pub fn logical_qubits<I>(qubits: I) -> Self
    where
        I: IntoIterator<Item = QubitId>,
    {
        Self::LogicalQubits(qubits.into_iter().collect())
    }

    /// Returns whether this scope covers the complete program.
    pub const fn is_program(self: &Self) -> bool {
        matches!(self, Self::Program)
    }

    /// Returns the explicitly scoped logical qubits, when present.
    pub fn logical_qubits(&self) -> Option<&[QubitId]> {
        match self {
            Self::Program => None,
            Self::LogicalQubits(qubits) => Some(qubits.as_ref()),
        }
    }

    /// Returns the number of explicitly scoped logical qubits.
    ///
    /// This is an observation of the request, not an architectural limit.
    pub fn logical_qubit_count(&self) -> Option<usize> {
        self.logical_qubits().map(<[QubitId]>::len)
    }
}

// =============================================================================
// Resource preference
// =============================================================================

/// Describes how the caller wants resilience to treat resource overhead.
///
/// This is intentionally qualitative. Actual resource availability and
/// capability negotiation are owned by the execution context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ResourcePreference {
    /// Prefer semantic correctness even when additional resources are needed.
    #[default]
    CorrectnessFirst,

    /// Prefer execution completion when the semantic contract remains valid.
    AvailabilityFirst,

    /// Prefer minimizing additional resources while preserving the semantic
    /// contract.
    ResourceEfficient,

    /// Require the policy layer to make the trade-off explicitly.
    PolicyDriven,
}

impl ResourcePreference {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorrectnessFirst => "correctness_first",
            Self::AvailabilityFirst => "availability_first",
            Self::ResourceEfficient => "resource_efficient",
            Self::PolicyDriven => "policy_driven",
        }
    }
}

impl fmt::Display for ResourcePreference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Adaptation permissions
// =============================================================================

/// Declares which classes of physical execution adaptation the caller permits.
///
/// These are permissions/requirements for the policy layer. They do not invoke
/// the underlying mechanisms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdaptationPermissions {
    /// Whether logical-to-physical remapping may be requested.
    remapping: bool,

    /// Whether routing may be recomputed.
    rerouting: bool,

    /// Whether scheduling may be recomputed.
    rescheduling: bool,

    /// Whether the program may be recompiled.
    recompilation: bool,

    /// Whether optimization may be rerun for a changed target.
    reoptimization: bool,

    /// Whether execution may migrate to another compatible target.
    migration: bool,
}

impl Default for AdaptationPermissions {
    fn default() -> Self {
        Self {
            remapping: true,
            rerouting: true,
            rescheduling: true,
            recompilation: true,
            reoptimization: true,
            migration: true,
        }
    }
}

impl AdaptationPermissions {
    /// Creates permissions with all adaptation mechanisms disabled.
    pub const fn deny_all() -> Self {
        Self {
            remapping: false,
            rerouting: false,
            rescheduling: false,
            recompilation: false,
            reoptimization: false,
            migration: false,
        }
    }

    /// Creates permissions with all supported adaptation classes enabled.
    pub const fn allow_all() -> Self {
        Self {
            remapping: true,
            rerouting: true,
            rescheduling: true,
            recompilation: true,
            reoptimization: true,
            migration: true,
        }
    }

    /// Enables or disables logical-to-physical remapping.
    pub const fn with_remapping(mut self, allowed: bool) -> Self {
        self.remapping = allowed;
        self
    }

    /// Enables or disables rerouting.
    pub const fn with_rerouting(mut self, allowed: bool) -> Self {
        self.rerouting = allowed;
        self
    }

    /// Enables or disables rescheduling.
    pub const fn with_rescheduling(mut self, allowed: bool) -> Self {
        self.rescheduling = allowed;
        self
    }

    /// Enables or disables recompilation.
    pub const fn with_recompilation(mut self, allowed: bool) -> Self {
        self.recompilation = allowed;
        self
    }

    /// Enables or disables reoptimization.
    pub const fn with_reoptimization(mut self, allowed: bool) -> Self {
        self.reoptimization = allowed;
        self
    }

    /// Enables or disables target migration.
    pub const fn with_migration(mut self, allowed: bool) -> Self {
        self.migration = allowed;
        self
    }

    /// Returns whether remapping is allowed.
    pub const fn remapping_allowed(self) -> bool {
        self.remapping
    }

    /// Returns whether rerouting is allowed.
    pub const fn rerouting_allowed(self) -> bool {
        self.rerouting
    }

    /// Returns whether rescheduling is allowed.
    pub const fn rescheduling_allowed(self) -> bool {
        self.rescheduling
    }

    /// Returns whether recompilation is allowed.
    pub const fn recompilation_allowed(self) -> bool {
        self.recompilation
    }

    /// Returns whether reoptimization is allowed.
    pub const fn reoptimization_allowed(self) -> bool {
        self.reoptimization
    }

    /// Returns whether migration is allowed.
    pub const fn migration_allowed(self) -> bool {
        self.migration
    }
}

// =============================================================================
// Recovery permissions
// =============================================================================

/// Declares which recovery classes may be considered by the policy/planner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecoveryPermissions {
    /// Retry execution when the planner establishes that retry is valid.
    retry: bool,

    /// Restart from a valid execution boundary.
    restart: bool,

    /// Resume from a compatible checkpoint.
    resume: bool,

    /// Roll back to a valid checkpoint/state.
    rollback: bool,

    /// Migrate to another compatible target.
    migration: bool,

    /// Use a mathematically/semantically valid compensating action.
    compensation: bool,
}

impl Default for RecoveryPermissions {
    fn default() -> Self {
        Self {
            retry: true,
            restart: true,
            resume: true,
            rollback: true,
            migration: true,
            compensation: true,
        }
    }
}

impl RecoveryPermissions {
    /// Denies all recovery mechanisms.
    pub const fn deny_all() -> Self {
        Self {
            retry: false,
            restart: false,
            resume: false,
            rollback: false,
            migration: false,
            compensation: false,
        }
    }

    /// Allows all recovery mechanisms.
    pub const fn allow_all() -> Self {
        Self {
            retry: true,
            restart: true,
            resume: true,
            rollback: true,
            migration: true,
            compensation: true,
        }
    }

    /// Enables/disables retry.
    pub const fn with_retry(mut self, allowed: bool) -> Self {
        self.retry = allowed;
        self
    }

    /// Enables/disables restart.
    pub const fn with_restart(mut self, allowed: bool) -> Self {
        self.restart = allowed;
        self
    }

    /// Enables/disables checkpoint resume.
    pub const fn with_resume(mut self, allowed: bool) -> Self {
        self.resume = allowed;
        self
    }

    /// Enables/disables rollback.
    pub const fn with_rollback(mut self, allowed: bool) -> Self {
        self.rollback = allowed;
        self
    }

    /// Enables/disables migration.
    pub const fn with_migration(mut self, allowed: bool) -> Self {
        self.migration = allowed;
        self
    }

    /// Enables/disables compensation.
    pub const fn with_compensation(mut self, allowed: bool) -> Self {
        self.compensation = allowed;
        self
    }

    /// Returns whether retry is permitted.
    pub const fn retry_allowed(self) -> bool {
        self.retry
    }

    /// Returns whether restart is permitted.
    pub const fn restart_allowed(self) -> bool {
        self.restart
    }

    /// Returns whether resume is permitted.
    pub const fn resume_allowed(self) -> bool {
        self.resume
    }

    /// Returns whether rollback is permitted.
    pub const fn rollback_allowed(self) -> bool {
        self.rollback
    }

    /// Returns whether migration is permitted.
    pub const fn migration_allowed(self) -> bool {
        self.migration
    }

    /// Returns whether compensation is permitted.
    pub const fn compensation_allowed(self) -> bool {
        self.compensation
    }
}

// =============================================================================
// Mitigation permissions
// =============================================================================

/// Declares whether error-mitigation mechanisms may be considered.
///
/// This does not select a specific mitigation algorithm. Selection remains
/// owned by `quantum::resilience::mitigation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MitigationPermission {
    /// Do not apply mitigation.
    Disabled,

    /// Permit the policy/selector to choose an applicable mitigation strategy.
    Allowed,

    /// Require mitigation when the applicable policy establishes that it is
    /// necessary and safe.
    Required,
}

impl Default for MitigationPermission {
    fn default() -> Self {
        Self::Allowed
    }
}

impl MitigationPermission {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Allowed => "allowed",
            Self::Required => "required",
        }
    }
}

impl fmt::Display for MitigationPermission {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// QEC adaptation permission
// =============================================================================

/// Declares whether resilience may request a change to the applicable QEC
/// configuration.
///
/// Resilience never implements QEC itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum QecAdaptationPermission {
    /// QEC configuration must remain unchanged.
    Disabled,

    /// QEC adaptation may be selected when supported and policy-approved.
    #[default]
    Allowed,

    /// A QEC adaptation may be required by policy when verification establishes
    /// that the current configuration is insufficient.
    Required,
}

impl QecAdaptationPermission {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Allowed => "allowed",
            Self::Required => "required",
        }
    }
}

impl fmt::Display for QecAdaptationPermission {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Retry budget
// =============================================================================

/// Explicit caller-supplied recovery budget.
///
/// `None` means the request itself does not impose a retry-count ceiling.
/// This does NOT mean infinite retries: the policy, target capabilities,
/// deadline, security controls, and planner state still govern execution.
///
/// The important property is that the architecture never embeds a fixed
/// retry count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RetryBudget(Option<NonZeroU64>);

impl RetryBudget {
    /// Creates a budget with no request-local retry-count ceiling.
    pub const fn unlimited() -> Self {
        Self(None)
    }

    /// Creates an explicit retry-count ceiling.
    pub const fn limited(max_retries: NonZeroU64) -> Self {
        Self(Some(max_retries))
    }

    /// Creates an explicit retry-count ceiling from a positive integer.
    pub const fn try_limited(max_retries: u64) -> Option<Self> {
        match NonZeroU64::new(max_retries) {
            Some(value) => Some(Self::limited(value)),
            None => None,
        }
    }

    /// Returns the caller-supplied retry-count ceiling.
    pub const fn max_retries(self) -> Option<NonZeroU64> {
        self.0
    }

    /// Returns whether the request has no retry-count ceiling.
    pub const fn is_unlimited(self) -> bool {
        self.0.is_none()
    }
}

// =============================================================================
// Time budget
// =============================================================================

/// Optional caller-supplied wall-clock budget.
///
/// The request does not start or inspect a clock. The runtime/context owns
/// measurement and enforcement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TimeBudget(Option<Duration>);

impl TimeBudget {
    /// Creates an unbounded request-local time budget.
    pub const fn unlimited() -> Self {
        Self(None)
    }

    /// Creates a bounded time budget.
    pub const fn limited(duration: Duration) -> Self {
        Self(Some(duration))
    }

    /// Returns the optional time budget.
    pub const fn duration(self) -> Option<Duration> {
        self.0
    }

    /// Returns whether no request-local deadline is declared.
    pub const fn is_unlimited(self) -> bool {
        self.0.is_none()
    }
}

// =============================================================================
// Shot budget
// =============================================================================

/// Optional caller-supplied execution-shot budget.
///
/// The resilience layer does not assume that every quantum computational model
/// is shot-based. Therefore this remains optional and is interpreted only by
/// an execution context that supports the concept.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ShotBudget(Option<NonZeroU64>);

impl ShotBudget {
    /// Creates a request with no shot budget.
    pub const fn unlimited() -> Self {
        Self(None)
    }

    /// Creates an explicit shot budget.
    pub const fn limited(shots: NonZeroU64) -> Self {
        Self(Some(shots))
    }

    /// Returns the optional shot budget.
    pub const fn shots(self) -> Option<NonZeroU64> {
        self.0
    }

    /// Returns whether no shot budget is declared.
    pub const fn is_unlimited(self) -> bool {
        self.0.is_none()
    }
}

// =============================================================================
// Request identifier
// =============================================================================

/// Stable caller-supplied request identifier.
///
/// The request ID is an observability/provenance identity, not a quantum
/// identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResilienceRequestId(Arc<str>);

impl ResilienceRequestId {
    /// Creates a request ID after validating that it is non-empty.
    ///
    /// Whitespace is not accepted because an identifier containing only
    /// whitespace is not useful for deterministic provenance.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, RequestValidationError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(RequestValidationError::EmptyRequestId);
        }

        Ok(Self(value))
    }

    /// Returns the stable identifier.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ResilienceRequestId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Deterministic seed
// =============================================================================

/// Explicit deterministic seed supplied by the caller.
///
/// The request does not generate randomness. A seed is merely part of the
/// deterministic/reproducibility boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeterministicSeed(u64);

impl DeterministicSeed {
    /// Creates a deterministic seed.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the seed.
    pub const fn get(self) -> u64 {
        self.0
    }
}

// =============================================================================
// Request options
// =============================================================================

/// Immutable resilience request options.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResilienceRequestOptions {
    execution_mode: ResilienceExecutionMode,
    semantic_guarantee: SemanticGuarantee,
    resource_preference: ResourcePreference,
    adaptation: AdaptationPermissions,
    recovery: RecoveryPermissions,
    mitigation: MitigationPermission,
    qec_adaptation: QecAdaptationPermission,
    retry_budget: RetryBudget,
    time_budget: TimeBudget,
    shot_budget: ShotBudget,
    deterministic_seed: Option<DeterministicSeed>,
    allow_escalation: bool,
}

impl Default for ResilienceRequestOptions {
    fn default() -> Self {
        Self {
            execution_mode: ResilienceExecutionMode::BestEffort,
            semantic_guarantee: SemanticGuarantee::Strict,
            resource_preference: ResourcePreference::CorrectnessFirst,
            adaptation: AdaptationPermissions::default(),
            recovery: RecoveryPermissions::default(),
            mitigation: MitigationPermission::Allowed,
            qec_adaptation: QecAdaptationPermission::Allowed,
            retry_budget: RetryBudget::unlimited(),
            time_budget: TimeBudget::unlimited(),
            shot_budget: ShotBudget::unlimited(),
            deterministic_seed: None,
            allow_escalation: true,
        }
    }
}

impl ResilienceRequestOptions {
    /// Creates default production-safe options.
    pub const fn new() -> Self {
        Self {
            execution_mode: ResilienceExecutionMode::BestEffort,
            semantic_guarantee: SemanticGuarantee::Strict,
            resource_preference: ResourcePreference::CorrectnessFirst,
            adaptation: AdaptationPermissions::allow_all(),
            recovery: RecoveryPermissions::allow_all(),
            mitigation: MitigationPermission::Allowed,
            qec_adaptation: QecAdaptationPermission::Allowed,
            retry_budget: RetryBudget::unlimited(),
            time_budget: TimeBudget::unlimited(),
            shot_budget: ShotBudget::unlimited(),
            deterministic_seed: None,
            allow_escalation: true,
        }
    }

    /// Selects the execution mode.
    pub const fn with_execution_mode(
        mut self,
        mode: ResilienceExecutionMode,
    ) -> Self {
        self.execution_mode = mode;
        self
    }

    /// Selects the semantic guarantee.
    pub const fn with_semantic_guarantee(
        mut self,
        guarantee: SemanticGuarantee,
    ) -> Self {
        self.semantic_guarantee = guarantee;
        self
    }

    /// Selects the resource preference.
    pub const fn with_resource_preference(
        mut self,
        preference: ResourcePreference,
    ) -> Self {
        self.resource_preference = preference;
        self
    }

    /// Sets adaptation permissions.
    pub const fn with_adaptation(
        mut self,
        adaptation: AdaptationPermissions,
    ) -> Self {
        self.adaptation = adaptation;
        self
    }

    /// Sets recovery permissions.
    pub const fn with_recovery(
        mut self,
        recovery: RecoveryPermissions,
    ) -> Self {
        self.recovery = recovery;
        self
    }

    /// Sets mitigation permission.
    pub const fn with_mitigation(
        mut self,
        mitigation: MitigationPermission,
    ) -> Self {
        self.mitigation = mitigation;
        self
    }

    /// Sets QEC adaptation permission.
    pub const fn with_qec_adaptation(
        mut self,
        permission: QecAdaptationPermission,
    ) -> Self {
        self.qec_adaptation = permission;
        self
    }

    /// Sets the caller-supplied retry budget.
    pub const fn with_retry_budget(mut self, budget: RetryBudget) -> Self {
        self.retry_budget = budget;
        self
    }

    /// Sets the caller-supplied time budget.
    pub const fn with_time_budget(mut self, budget: TimeBudget) -> Self {
        self.time_budget = budget;
        self
    }

    /// Sets the caller-supplied shot budget.
    pub const fn with_shot_budget(mut self, budget: ShotBudget) -> Self {
        self.shot_budget = budget;
        self
    }

    /// Supplies an explicit deterministic seed.
    pub const fn with_deterministic_seed(
        mut self,
        seed: Option<DeterministicSeed>,
    ) -> Self {
        self.deterministic_seed = seed;
        self
    }

    /// Controls whether escalation is permitted.
    pub const fn with_escalation(mut self, allowed: bool) -> Self {
        self.allow_escalation = allowed;
        self
    }

    /// Returns the execution mode.
    pub const fn execution_mode(&self) -> ResilienceExecutionMode {
        self.execution_mode
    }

    /// Returns the semantic guarantee.
    pub const fn semantic_guarantee(&self) -> SemanticGuarantee {
        self.semantic_guarantee
    }

    /// Returns the resource preference.
    pub const fn resource_preference(&self) -> ResourcePreference {
        self.resource_preference
    }

    /// Returns adaptation permissions.
    pub const fn adaptation(&self) -> AdaptationPermissions {
        self.adaptation
    }

    /// Returns recovery permissions.
    pub const fn recovery(&self) -> RecoveryPermissions {
        self.recovery
    }

    /// Returns mitigation permission.
    pub const fn mitigation(&self) -> MitigationPermission {
        self.mitigation
    }

    /// Returns QEC adaptation permission.
    pub const fn qec_adaptation(&self) -> QecAdaptationPermission {
        self.qec_adaptation
    }

    /// Returns retry budget.
    pub const fn retry_budget(&self) -> RetryBudget {
        self.retry_budget
    }

    /// Returns time budget.
    pub const fn time_budget(&self) -> TimeBudget {
        self.time_budget
    }

    /// Returns shot budget.
    pub const fn shot_budget(&self) -> ShotBudget {
        self.shot_budget
    }

    /// Returns the deterministic seed, when explicitly supplied.
    pub const fn deterministic_seed(&self) -> Option<DeterministicSeed> {
        self.deterministic_seed
    }

    /// Returns whether escalation is allowed.
    pub const fn escalation_allowed(&self) -> bool {
        self.allow_escalation
    }
}

// =============================================================================
// Request validation error
// =============================================================================

/// Validation failures local to the request contract.
///
/// Cross-subsystem validation belongs to the controller/context because it
/// requires capabilities, policy, IR validation and execution state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestValidationError {
    /// The request identifier was empty or whitespace-only.
    EmptyRequestId,

    /// Strict deterministic mode was requested with a missing deterministic
    /// seed while the request explicitly requires caller-provided randomness.
    MissingDeterministicSeed,

    /// A logical-qubit scope contained a duplicate canonical qubit identity.
    DuplicateLogicalQubit(QubitId),
}

impl fmt::Display for RequestValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRequestId => {
                formatter.write_str("resilience request ID must not be empty")
            }
            Self::MissingDeterministicSeed => {
                formatter.write_str(
                    "strict deterministic mode requires an explicit deterministic seed",
                )
            }
            Self::DuplicateLogicalQubit(qubit) => {
                write!(
                    formatter,
                    "logical-qubit resilience scope contains duplicate qubit: {qubit:?}"
                )
            }
        }
    }
}

impl std::error::Error for RequestValidationError {}

// =============================================================================
// Resilience request
// =============================================================================

/// Immutable, provider-independent request submitted to the resilience
/// controller.
///
/// The request owns an `Arc<QuantumProgram>` so a large program is not copied
/// merely because resilience orchestration begins. This also permits multiple
/// resilience components to share the same immutable semantic program.
///
/// The program itself remains authoritative under `quantum::ir`.
#[derive(Debug, Clone)]
pub struct ResilienceRequest {
    request_id: ResilienceRequestId,

    /// Canonical Zamani semantic program.
    program: Arc<QuantumProgram>,

    /// Logical scope for this resilience operation.
    scope: ResilienceScope,

    /// Caller-declared resilience options.
    options: ResilienceRequestOptions,
}

impl ResilienceRequest {
    /// Constructs a request.
    ///
    /// The constructor performs only validation that can be completed without
    /// consulting hardware, policy, routing, scheduling, QEC or execution
    /// state.
    pub fn new(
        request_id: ResilienceRequestId,
        program: Arc<QuantumProgram>,
    ) -> Result<Self, RequestValidationError> {
        Self::from_parts(
            request_id,
            program,
            ResilienceScope::Program,
            ResilienceRequestOptions::default(),
        )
    }

    /// Constructs a request with explicit scope and options.
    pub fn from_parts(
        request_id: ResilienceRequestId,
        program: Arc<QuantumProgram>,
        scope: ResilienceScope,
        options: ResilienceRequestOptions,
    ) -> Result<Self, RequestValidationError> {
        validate_scope(&scope)?;

        validate_options(&options)?;

        Ok(Self {
            request_id,
            program,
            scope,
            options,
        })
    }

    /// Returns the request ID.
    pub fn request_id(&self) -> &ResilienceRequestId {
        &self.request_id
    }

    /// Returns the canonical immutable Zamani program.
    pub fn program(&self) -> &QuantumProgram {
        self.program.as_ref()
    }

    /// Returns the shared program handle.
    pub fn program_arc(&self) -> Arc<QuantumProgram> {
        Arc::clone(&self.program)
    }

    /// Returns the resilience scope.
    pub fn scope(&self) -> &ResilienceScope {
        &self.scope
    }

    /// Returns request options.
    pub const fn options(&self) -> &ResilienceRequestOptions {
        &self.options
    }

    /// Returns the selected execution mode.
    pub const fn execution_mode(&self) -> ResilienceExecutionMode {
        self.options.execution_mode()
    }

    /// Returns the semantic guarantee.
    pub const fn semantic_guarantee(&self) -> SemanticGuarantee {
        self.options.semantic_guarantee()
    }

    /// Returns whether degraded acceptance is permitted.
    pub const fn degraded_acceptance_allowed(&self) -> bool {
        self.semantic_guarantee().permits_degraded_acceptance()
    }

    /// Returns the logical qubit scope when the request is explicitly scoped.
    pub fn logical_qubits(&self) -> Option<&[QubitId]> {
        self.scope.logical_qubits()
    }

    /// Returns whether the complete program is protected.
    pub const fn protects_complete_program(&self) -> bool {
        self.scope.is_program()
    }

    /// Returns whether strict deterministic execution was requested.
    pub const fn strict_determinism(&self) -> bool {
        self.execution_mode()
            .requires_complete_deterministic_inputs()
    }

    /// Returns the explicit deterministic seed, if any.
    pub const fn deterministic_seed(&self) -> Option<DeterministicSeed> {
        self.options.deterministic_seed()
    }

    /// Returns the caller's resource preference.
    pub const fn resource_preference(&self) -> ResourcePreference {
        self.options.resource_preference()
    }

    /// Returns adaptation permissions.
    pub const fn adaptation_permissions(&self) -> AdaptationPermissions {
        self.options.adaptation()
    }

    /// Returns recovery permissions.
    pub const fn recovery_permissions(&self) -> RecoveryPermissions {
        self.options.recovery()
    }

    /// Returns mitigation permission.
    pub const fn mitigation_permission(&self) -> MitigationPermission {
        self.options.mitigation()
    }

    /// Returns QEC adaptation permission.
    pub const fn qec_adaptation_permission(&self) -> QecAdaptationPermission {
        self.options.qec_adaptation()
    }

    /// Returns retry budget.
    pub const fn retry_budget(&self) -> RetryBudget {
        self.options.retry_budget()
    }

    /// Returns time budget.
    pub const fn time_budget(&self) -> TimeBudget {
        self.options.time_budget()
    }

    /// Returns shot budget.
    pub const fn shot_budget(&self) -> ShotBudget {
        self.options.shot_budget()
    }

    /// Returns whether external escalation is permitted.
    pub const fn escalation_allowed(&self) -> bool {
        self.options.escalation_allowed()
    }

    /// Validates request-local invariants.
    ///
    /// Cross-system validation remains deliberately outside this method.
    pub fn validate(&self) -> Result<(), RequestValidationError> {
        validate_scope(&self.scope)?;
        validate_options(&self.options)?;
        Ok(())
    }

    /// Creates a builder for the request.
    pub fn builder(
        request_id: ResilienceRequestId,
        program: Arc<QuantumProgram>,
    ) -> ResilienceRequestBuilder {
        ResilienceRequestBuilder::new(request_id, program)
    }
}

// =============================================================================
// Builder
// =============================================================================

/// Builder for `ResilienceRequest`.
///
/// The builder exists to keep construction readable as the request contract
/// evolves without requiring callers to depend on field layout.
#[derive(Debug, Clone)]
pub struct ResilienceRequestBuilder {
    request_id: ResilienceRequestId,
    program: Arc<QuantumProgram>,
    scope: ResilienceScope,
    options: ResilienceRequestOptions,
}

impl ResilienceRequestBuilder {
    /// Creates a builder.
    pub fn new(
        request_id: ResilienceRequestId,
        program: Arc<QuantumProgram>,
    ) -> Self {
        Self {
            request_id,
            program,
            scope: ResilienceScope::Program,
            options: ResilienceRequestOptions::default(),
        }
    }

    /// Sets the resilience scope.
    pub fn scope(mut self, scope: ResilienceScope) -> Self {
        self.scope = scope;
        self
    }

    /// Sets complete request options.
    pub fn options(mut self, options: ResilienceRequestOptions) -> Self {
        self.options = options;
        self
    }

    /// Sets execution mode.
    pub fn execution_mode(
        mut self,
        mode: ResilienceExecutionMode,
    ) -> Self {
        self.options = self.options.with_execution_mode(mode);
        self
    }

    /// Sets semantic guarantee.
    pub fn semantic_guarantee(
        mut self,
        guarantee: SemanticGuarantee,
    ) -> Self {
        self.options = self.options.with_semantic_guarantee(guarantee);
        self
    }

    /// Sets resource preference.
    pub fn resource_preference(
        mut self,
        preference: ResourcePreference,
    ) -> Self {
        self.options = self.options.with_resource_preference(preference);
        self
    }

    /// Sets adaptation permissions.
    pub fn adaptation(
        mut self,
        adaptation: AdaptationPermissions,
    ) -> Self {
        self.options = self.options.with_adaptation(adaptation);
        self
    }

    /// Sets recovery permissions.
    pub fn recovery(
        mut self,
        recovery: RecoveryPermissions,
    ) -> Self {
        self.options = self.options.with_recovery(recovery);
        self
    }

    /// Sets mitigation permission.
    pub fn mitigation(
        mut self,
        mitigation: MitigationPermission,
    ) -> Self {
        self.options = self.options.with_mitigation(mitigation);
        self
    }

    /// Sets QEC adaptation permission.
    pub fn qec_adaptation(
        mut self,
        permission: QecAdaptationPermission,
    ) -> Self {
        self.options = self.options.with_qec_adaptation(permission);
        self
    }

    /// Sets retry budget.
    pub fn retry_budget(mut self, budget: RetryBudget) -> Self {
        self.options = self.options.with_retry_budget(budget);
        self
    }

    /// Sets time budget.
    pub fn time_budget(mut self, budget: TimeBudget) -> Self {
        self.options = self.options.with_time_budget(budget);
        self
    }

    /// Sets shot budget.
    pub fn shot_budget(mut self, budget: ShotBudget) -> Self {
        self.options = self.options.with_shot_budget(budget);
        self
    }

    /// Sets the deterministic seed.
    pub fn deterministic_seed(
        mut self,
        seed: Option<DeterministicSeed>,
    ) -> Self {
        self.options = self.options.with_deterministic_seed(seed);
        self
    }

    /// Sets escalation permission.
    pub fn allow_escalation(mut self, allowed: bool) -> Self {
        self.options = self.options.with_escalation(allowed);
        self
    }

    /// Builds and validates the request.
    pub fn build(self) -> Result<ResilienceRequest, RequestValidationError> {
        ResilienceRequest::from_parts(
            self.request_id,
            self.program,
            self.scope,
            self.options,
        )
    }
}

// =============================================================================
// Internal validation
// =============================================================================

fn validate_scope(scope: &ResilienceScope) -> Result<(), RequestValidationError> {
    let Some(qubits) = scope.logical_qubits() else {
        return Ok(());
    };

    // This is intentionally O(n²) only for the explicit caller-supplied
    // scoped list. It avoids introducing a second qubit-ID representation or
    // requiring Hash/Eq assumptions beyond the canonical type.
    //
    // The controller/context may apply more scalable validation if the
    // canonical IR provides stronger indexing facilities.
    for (index, qubit) in qubits.iter().enumerate() {
        if qubits[index + 1..].iter().any(|other| other == qubit) {
            return Err(RequestValidationError::DuplicateLogicalQubit(
                qubit.clone(),
            ));
        }
    }

    Ok(())
}

fn validate_options(
    options: &ResilienceRequestOptions,
) -> Result<(), RequestValidationError> {
    if options.execution_mode().requires_complete_deterministic_inputs()
        && options.deterministic_seed().is_none()
    {
        return Err(RequestValidationError::MissingDeterministicSeed);
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn request_id() -> ResilienceRequestId {
        ResilienceRequestId::new("test-request")
            .expect("test request ID must be valid")
    }

    fn program() -> Arc<QuantumProgram> {
        // Construction of a canonical QuantumProgram is intentionally not
        // duplicated here. These tests should be connected to the repository's
        // canonical program fixture/constructor when the program test utility
        // becomes available.
        //
        // This helper is therefore left as an integration seam rather than
        // inventing a second program representation.
        panic!("use canonical QuantumProgram fixture in integration tests")
    }

    #[test]
    fn request_id_rejects_empty_value() {
        let result = ResilienceRequestId::new("   ");

        assert_eq!(
            result,
            Err(RequestValidationError::EmptyRequestId)
        );
    }

    #[test]
    fn retry_budget_has_no_architectural_default_limit() {
        let budget = RetryBudget::unlimited();

        assert!(budget.is_unlimited());
        assert_eq!(budget.max_retries(), None);
    }

    #[test]
    fn time_budget_can_be_unbounded() {
        let budget = TimeBudget::unlimited();

        assert!(budget.is_unlimited());
        assert_eq!(budget.duration(), None);
    }

    #[test]
    fn shot_budget_can_be_unbounded() {
        let budget = ShotBudget::unlimited();

        assert!(budget.is_unlimited());
        assert_eq!(budget.shots(), None);
    }

    #[test]
    fn default_adaptation_is_provider_independent() {
        let permissions = AdaptationPermissions::default();

        assert!(permissions.remapping_allowed());
        assert!(permissions.rerouting_allowed());
        assert!(permissions.rescheduling_allowed());
        assert!(permissions.recompilation_allowed());
        assert!(permissions.reoptimization_allowed());
        assert!(permissions.migration_allowed());
    }

    #[test]
    fn deny_all_adaptation_is_explicit() {
        let permissions = AdaptationPermissions::deny_all();

        assert!(!permissions.remapping_allowed());
        assert!(!permissions.rerouting_allowed());
        assert!(!permissions.rescheduling_allowed());
        assert!(!permissions.recompilation_allowed());
        assert!(!permissions.reoptimization_allowed());
        assert!(!permissions.migration_allowed());
    }

    #[test]
    fn semantic_guarantee_is_strict_by_default() {
        let options = ResilienceRequestOptions::default();

        assert_eq!(
            options.semantic_guarantee(),
            SemanticGuarantee::Strict
        );
        assert!(!options.semantic_guarantee().permits_degraded_acceptance());
    }

    #[test]
    fn scope_defaults_to_complete_program() {
        let scope = ResilienceScope::default();

        assert!(scope.is_program());
        assert_eq!(scope.logical_qubits(), None);
    }

    #[test]
    fn deterministic_mode_requires_explicit_seed() {
        let options = ResilienceRequestOptions::default()
            .with_execution_mode(
                ResilienceExecutionMode::StrictDeterministic,
            );

        let result = validate_options(&options);

        assert_eq!(
            result,
            Err(RequestValidationError::MissingDeterministicSeed)
        );
    }

    #[test]
    fn deterministic_mode_accepts_explicit_seed() {
        let options = ResilienceRequestOptions::default()
            .with_execution_mode(
                ResilienceExecutionMode::StrictDeterministic,
            )
            .with_deterministic_seed(Some(DeterministicSeed::new(42)));

        assert!(validate_options(&options).is_ok());
    }

    #[test]
    fn seed_is_explicit_and_deterministic() {
        let seed = DeterministicSeed::new(42);

        assert_eq!(seed.get(), 42);
    }

    // `program()` is intentionally not invoked by these contract tests.
    //
    // The canonical QuantumProgram construction belongs to the canonical IR
    // subsystem and should be supplied by its fixtures rather than duplicated
    // in resilience tests.
    #[allow(dead_code)]
    fn _canonical_program_fixture_seam() -> Arc<QuantumProgram> {
        program()
    }
}