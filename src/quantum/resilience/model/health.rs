//! Zamani Quantum Resilience — Health Model.
//!
//! Path:
//!     src/quantum/resilience/model/health.rs
//!
//! Purpose:
//!     Defines the provider-independent health state vocabulary used by the
//!     quantum resilience subsystem.
//!
//! Architectural role:
//!
//!     Health answers:
//!
//!         "What is the current operational condition of a resource or
//!          execution capability?"
//!
//!     Health does NOT answer:
//!
//!         "How severe is an incident?"
//!         "What caused the condition?"
//!         "What recovery action should be performed?"
//!         "How confident are we in the observation?"
//!
//!     Those concerns belong to:
//!
//!         model/severity.rs
//!         model/fault.rs
//!         model/incident.rs
//!         model/confidence.rs
//!         diagnosis/
//!         policy/
//!         planning/
//!         recovery/
//!
//! # Ownership
//!
//! This module owns:
//!
//! - canonical resilience health states;
//! - health-state ordering;
//! - health-state predicates;
//! - deterministic health-state transitions;
//! - stable textual representations;
//! - health-state aggregation;
//! - health observations/conditions;
//! - structural validation of health values.
//!
//! This module does NOT own:
//!
//! - physical hardware status;
//! - backend/provider status;
//! - quantum fault semantics;
//! - fault probabilities;
//! - confidence models;
//! - resource identities;
//! - topology;
//! - calibration;
//! - routing;
//! - scheduling;
//! - QEC;
//! - mitigation;
//! - recovery;
//! - authorization;
//! - persistence;
//! - telemetry transport;
//! - machine-size limits.
//!
//! # Relationship to the rest of Zamani
//!
//! ```text
//! quantum::ir::qubit
//!     │
//!     └── canonical logical/physical identities
//!
//! quantum::hardware
//!     │
//!     └── provider/device/backend observations
//!
//! quantum::zqn
//!     │
//!     └── canonical fault/noise semantics
//!
//!             │
//!             ▼
//! quantum::resilience::model::health
//!             │
//!             ├── diagnosis
//!             ├── policy
//!             ├── planning
//!             ├── adaptation
//!             ├── recovery
//!             ├── telemetry
//!             └── verification
//! ```
//!
//! The health model is deliberately below those decision-making layers.
//!
//! # Health versus severity
//!
//! Health and severity are different dimensions.
//!
//! ```text
//! Health
//!     = current operational condition of a resource.
//!
//! Severity
//!     = operational consequence of a condition/incident.
//! ```
//!
//! Examples:
//!
//! ```text
//! resource health = Degraded
//! incident severity = Informational
//! ```
//!
//! is possible when the degradation is known and harmless to the current
//! workload.
//!
//! Conversely:
//!
//! ```text
//! resource health = Healthy
//! incident severity = Critical
//! ```
//!
//! can occur transiently when a critical incident has just been observed but
//! the affected resource has already recovered.
//!
//! Therefore this module MUST NOT contain or derive the resilience
//! `Severity` value automatically.
//!
//! # Health versus hardware backend status
//!
//! `quantum::hardware::backend_status` already owns backend operational state.
//!
//! That module answers questions such as:
//!
//! ```text
//! Is the backend available?
//! Is it busy?
//! Is it offline?
//! Is it under maintenance?
//! ```
//!
//! This module answers the resilience question:
//!
//! ```text
//! What health condition should the resilience engine associate with the
//! resource/capability after interpreting available observations?
//! ```
//!
//! Hardware adapters may therefore translate provider/backend observations
//! into resilience observations without creating a second backend-status
//! vocabulary here.
//!
//! # Why health has no `Error` state
//!
//! `Error` is an event/fault concept, not a sufficiently precise health state.
//!
//! A resource can experience an error and subsequently be:
//!
//! - Healthy;
//! - Degraded;
//! - Unstable;
//! - Recovering;
//! - Unavailable;
//! - Quarantined.
//!
//! The actual error belongs to the fault/incident model.
//!
//! # Health states
//!
//! The canonical resilience health states are:
//!
//! ```text
//! Unknown
//! Healthy
//! Degraded
//! Unstable
//! Unavailable
//! Recovering
//! Quarantined
//! Retired
//! ```
//!
//! They intentionally represent condition rather than action.
//!
//! `Recovering` means that recovery is currently represented as part of the
//! observed condition. It does NOT authorize or initiate recovery.
//!
//! `Quarantined` means the resource has been isolated from normal use by a
//! higher-level control plane. This type records the state; it does not grant
//! or revoke authorization itself.
//!
//! `Retired` is terminal in the resilience health lifecycle.
//!
//! # Write once, scale everywhere
//!
//! No machine-size assumption exists in this module.
//!
//! There is no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_RESOURCES
//! MAX_DEVICES
//! MAX_BACKENDS
//! MAX_CONDITIONS
//! ```
//!
//! Health is a constant-size semantic state.
//!
//! A single physical qubit, an arbitrarily large logical register, an entire
//! QPU, a backend fleet, or a distributed quantum execution domain can all be
//! represented by the same health vocabulary.
//!
//! The identity and collection size are owned by the resource/container
//! layers.
//!
//! "Infinity" therefore means that this model introduces no artificial
//! machine-size ceiling. Actual execution remains bounded by available
//! computational, memory, storage, network, and hardware resources.
//!
//! # Determinism
//!
//! This module:
//!
//! - does not read the system clock;
//! - does not generate randomness;
//! - does not access environment variables;
//! - does not access global mutable state;
//! - does not perform I/O;
//! - does not inspect hardware;
//! - does not query providers;
//! - does not use memory addresses as identity.
//!
//! Equal health values produce equal results.
//!
//! # Security
//!
//! Health is descriptive data.
//!
//! A health value MUST NOT:
//!
//! - grant hardware access;
//! - grant credentials;
//! - authorize recovery;
//! - authorize migration;
//! - bypass verification;
//! - disable security controls;
//! - establish trust in an observation source.
//!
//! A malicious component reporting `Retired` or `Healthy` does not acquire
//! authority merely by producing that value.
//!
//! Trust, authentication, authorization and provenance belong to higher-level
//! security and telemetry contracts.
//!
//! # Integration contract
//!
//! `health.rs` intentionally has no dependency on `quantum::ir::qubit`.
//!
//! This is deliberate.
//!
//! `QubitId` and `PhysicalQubitId` belong to the canonical IR and must be
//! supplied by `model/resource.rs` or the appropriate integration boundary
//! when a health state is associated with a quantum resource.
//!
//! Do NOT duplicate `QubitId` or `PhysicalQubitId` here.
//!
//! Later integration should look conceptually like:
//!
//! ```text
//! model/resource.rs
//!       │
//!       ├── quantum::ir::qubit::QubitId
//!       ├── quantum::ir::qubit::PhysicalQubitId
//!       └── HealthState / HealthObservation
//!                    │
//!                    ▼
//!              diagnosis/policy
//! ```
//!
//! This keeps identity ownership in `quantum::ir::qubit` and health
//! semantics in this file.
//!
//! # Integration with severity
//!
//! `model/severity.rs` is intentionally not imported here.
//!
//! A caller may associate a severity with a health observation at a higher
//! layer:
//!
//! ```text
//! HealthObservation
//!     │
//!     ├── health state
//!     ├── evidence
//!     ├── source
//!     ├── confidence
//!     └── interpreted severity
//! ```
//!
//! Health itself must remain reusable without the severity subsystem.
//!
//! # Integration with hardware
//!
//! Hardware adapters should map provider/backend state into health
//! observations.
//!
//! They must not add provider-specific variants to `HealthState`.
//!
//! For example:
//!
//! ```text
//! provider says "maintenance"
//!             │
//!             ▼
//! hardware adapter
//!             │
//!             ▼
//! resilience health observation
//!             │
//!             ▼
//! Maintenance-like condition is represented by the appropriate health
//! state/condition without exposing provider terminology.
//! ```
//!
//! Backend operational status remains owned by
//! `quantum::hardware::backend_status`.
//!
//! # Integration with ZQN
//!
//! ZQN remains authoritative for quantum fault semantics.
//!
//! A ZQN fault may contribute evidence that a resource is:
//!
//! ```text
//! Healthy
//! Degraded
//! Unstable
//! Unavailable
//! ```
//!
//! But this module must not reinterpret or replace ZQN fault taxonomy.
//!
//! # Integration with diagnosis
//!
//! Diagnosis may consume:
//!
//! - current health;
//! - previous health;
//! - fault observations;
//! - telemetry;
//! - calibration;
//! - topology;
//! - QEC observations;
//! - execution results.
//!
//! Diagnosis determines causal meaning.
//!
//! Health does not perform causal inference.
//!
//! # Integration with policy
//!
//! Policy may use health as a constraint:
//!
//! ```text
//! if resource health is Quarantined
//!     resource cannot be selected
//! ```
//!
//! The policy engine owns that decision.
//!
//! Health itself never selects a recovery action.
//!
//! # Integration with planning
//!
//! Planning may compare candidate resources according to health state.
//!
//! Health does not rank recovery plans.
//!
//! # Integration with verification
//!
//! Verification may require that a result was produced using resources whose
//! health satisfied the execution policy.
//!
//! Health alone never proves result correctness.
//!
//! # Integration with telemetry
//!
//! Telemetry produces observations.
//!
//! Health stores normalized state/condition data but does not own telemetry
//! transport or timestamps.
//!
//! # State transition philosophy
//!
//! Health transitions are deliberately permissive enough for distributed and
//! heterogeneous systems while preventing clearly invalid resurrection of a
//! retired resource.
//!
//! A higher-level policy may impose stricter transition rules.
//!
//! This module therefore distinguishes:
//!
//! ```text
//! structurally valid transition
//! ```
//!
//! from:
//!
//! ```text
//! policy-authorized transition
//! ```
//!
//! Only the former belongs here.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use core::str::FromStr;

// ============================================================================
// Health state
// ============================================================================

/// Canonical resilience health state.
///
/// Health describes the current operational condition of a resource or
/// capability. It does not identify the resource and does not authorize an
/// action.
///
/// The ordering is from least usable/known to most terminally constrained;
/// it must NOT be interpreted as severity or probability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HealthState {
    /// No sufficiently trustworthy health state has been established.
    Unknown,

    /// Resource is operating within the applicable health expectations.
    Healthy,

    /// Resource remains usable but one or more health properties are below
    /// their expected operating condition.
    Degraded,

    /// Resource health is fluctuating, unstable, or otherwise unsuitable for
    /// assuming sustained normal operation.
    Unstable,

    /// Resource is known not to be usable for the relevant operation.
    Unavailable,

    /// Resource is currently undergoing a recovery process.
    ///
    /// This is descriptive state, not recovery authorization.
    Recovering,

    /// Resource has been deliberately isolated from normal use.
    ///
    /// This is descriptive state, not an authorization primitive.
    Quarantined,

    /// Resource has permanently left the usable resource pool.
    Retired,
}

impl HealthState {
    /// Returns the least informative/usable canonical state.
    #[must_use]
    pub const fn minimum() -> Self {
        Self::Unknown
    }

    /// Returns the terminal canonical state.
    #[must_use]
    pub const fn maximum() -> Self {
        Self::Retired
    }

    /// Returns a stable semantic rank.
    ///
    /// The rank is useful for deterministic ordering only.
    ///
    /// It is NOT:
    ///
    /// - a probability;
    /// - a percentage;
    /// - a fidelity;
    /// - an error rate;
    /// - a severity score.
    #[must_use]
    pub const fn rank(self) -> u8 {
        match self {
            Self::Unknown => 0,
            Self::Healthy => 1,
            Self::Degraded => 2,
            Self::Unstable => 3,
            Self::Unavailable => 4,
            Self::Recovering => 5,
            Self::Quarantined => 6,
            Self::Retired => 7,
        }
    }

    /// Constructs a health state from its stable semantic rank.
    ///
    /// Unknown future values are rejected rather than silently mapped.
    #[must_use]
    pub const fn from_rank(rank: u8) -> Option<Self> {
        match rank {
            0 => Some(Self::Unknown),
            1 => Some(Self::Healthy),
            2 => Some(Self::Degraded),
            3 => Some(Self::Unstable),
            4 => Some(Self::Unavailable),
            5 => Some(Self::Recovering),
            6 => Some(Self::Quarantined),
            7 => Some(Self::Retired),
            _ => None,
        }
    }

    /// Returns the stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Unstable => "unstable",
            Self::Unavailable => "unavailable",
            Self::Recovering => "recovering",
            Self::Quarantined => "quarantined",
            Self::Retired => "retired",
        }
    }

    /// Returns whether the state is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns whether the state is healthy.
    #[must_use]
    pub const fn is_healthy(self) -> bool {
        matches!(self, Self::Healthy)
    }

    /// Returns whether the state is degraded.
    #[must_use]
    pub const fn is_degraded(self) -> bool {
        matches!(self, Self::Degraded)
    }

    /// Returns whether the state is unstable.
    #[must_use]
    pub const fn is_unstable(self) -> bool {
        matches!(self, Self::Unstable)
    }

    /// Returns whether the resource is currently unavailable.
    #[must_use]
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }

    /// Returns whether the resource is recovering.
    #[must_use]
    pub const fn is_recovering(self) -> bool {
        matches!(self, Self::Recovering)
    }

    /// Returns whether the resource is quarantined.
    #[must_use]
    pub const fn is_quarantined(self) -> bool {
        matches!(self, Self::Quarantined)
    }

    /// Returns whether the resource is retired.
    #[must_use]
    pub const fn is_retired(self) -> bool {
        matches!(self, Self::Retired)
    }

    /// Returns whether normal workload use is compatible with this state.
    ///
    /// This is deliberately conservative:
    ///
    /// - `Healthy` is normally usable;
    /// - `Degraded` may be usable subject to policy;
    /// - all other states require additional policy interpretation.
    ///
    /// This method does NOT authorize execution.
    #[must_use]
    pub const fn is_normally_usable(self) -> bool {
        matches!(self, Self::Healthy | Self::Degraded)
    }

    /// Returns whether the state indicates a condition requiring attention.
    #[must_use]
    pub const fn requires_attention(self) -> bool {
        matches!(
            self,
            Self::Degraded
                | Self::Unstable
                | Self::Unavailable
                | Self::Recovering
                | Self::Quarantined
                | Self::Retired
        )
    }

    /// Returns whether the state represents a terminal resource condition.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Retired)
    }

    /// Returns whether the state is more degraded than the supplied state
    /// according to the canonical health ordering.
    ///
    /// This ordering is only a deterministic health ordering. It is not a
    /// resilience severity ordering.
    #[must_use]
    pub const fn is_at_least_as_degraded_as(self, other: Self) -> bool {
        self.rank() >= other.rank()
    }

    /// Returns whether the state is less degraded than the supplied state.
    #[must_use]
    pub const fn is_less_degraded_than(self, other: Self) -> bool {
        self.rank() < other.rank()
    }

    /// Returns the more degraded state according to the canonical ordering.
    ///
    /// This is useful for conservative aggregation only.
    #[must_use]
    pub const fn max(self, other: Self) -> Self {
        if self.rank() >= other.rank() {
            self
        } else {
            other
        }
    }

    /// Returns the less degraded state according to the canonical ordering.
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        if self.rank() <= other.rank() {
            self
        } else {
            other
        }
    }

    /// Determines whether a structural health transition is valid.
    ///
    /// This method does not perform the transition.
    ///
    /// `Retired` is terminal.
    ///
    /// The rules intentionally permit recovery and degradation paths required
    /// by distributed and heterogeneous quantum systems.
    #[must_use]
    pub const fn can_transition_to(self, target: Self) -> bool {
        if self == target {
            return true;
        }

        match self {
            Self::Unknown => true,

            Self::Healthy => matches!(
                target,
                Self::Degraded
                    | Self::Unstable
                    | Self::Unavailable
                    | Self::Recovering
                    | Self::Quarantined
                    | Self::Retired
            ),

            Self::Degraded => matches!(
                target,
                Self::Healthy
                    | Self::Unstable
                    | Self::Unavailable
                    | Self::Recovering
                    | Self::Quarantined
                    | Self::Retired
            ),

            Self::Unstable => matches!(
                target,
                Self::Healthy
                    | Self::Degraded
                    | Self::Unavailable
                    | Self::Recovering
                    | Self::Quarantined
                    | Self::Retired
            ),

            Self::Unavailable => matches!(
                target,
                Self::Unknown
                    | Self::Healthy
                    | Self::Degraded
                    | Self::Unstable
                    | Self::Recovering
                    | Self::Quarantined
                    | Self::Retired
            ),

            Self::Recovering => matches!(
                target,
                Self::Healthy
                    | Self::Degraded
                    | Self::Unstable
                    | Self::Unavailable
                    | Self::Quarantined
                    | Self::Retired
            ),

            Self::Quarantined => matches!(
                target,
                Self::Recovering
                    | Self::Unavailable
                    | Self::Retired
                    | Self::Healthy
                    | Self::Degraded
                    | Self::Unstable
            ),

            Self::Retired => false,
        }
    }
}

impl Default for HealthState {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for HealthState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Parsing
// ============================================================================

/// Error returned when parsing an invalid [`HealthState`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidHealthState {
    input: String,
}

impl InvalidHealthState {
    /// Returns the rejected input.
    ///
    /// The parser stores the supplied string so callers can produce useful
    /// diagnostics. Callers processing untrusted arbitrarily large input
    /// should enforce their own input/resource policy before parsing.
    #[must_use]
    pub fn input(&self) -> &str {
        &self.input
    }
}

impl fmt::Display for InvalidHealthState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid health state: {}", self.input)
    }
}

impl std::error::Error for InvalidHealthState {}

impl FromStr for HealthState {
    type Err = InvalidHealthState;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "unknown" => Ok(Self::Unknown),
            "healthy" => Ok(Self::Healthy),
            "degraded" => Ok(Self::Degraded),
            "unstable" => Ok(Self::Unstable),
            "unavailable" => Ok(Self::Unavailable),
            "recovering" => Ok(Self::Recovering),
            "quarantined" => Ok(Self::Quarantined),
            "retired" => Ok(Self::Retired),
            _ => Err(InvalidHealthState {
                input: value.to_owned(),
            }),
        }
    }
}

impl TryFrom<u8> for HealthState {
    type Error = InvalidHealthState;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_rank(value).ok_or_else(|| InvalidHealthState {
            input: value.to_string(),
        })
    }
}

impl From<HealthState> for u8 {
    fn from(value: HealthState) -> Self {
        value.rank()
    }
}

// ============================================================================
// Health condition kind
// ============================================================================

/// Canonical category of a health condition.
///
/// A condition explains an aspect of health without defining a fault taxonomy
/// or recovery action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HealthConditionKind {
    /// General operational condition.
    Operational,

    /// Availability-related condition.
    Availability,

    /// Stability-related condition.
    Stability,

    /// Performance degradation.
    Performance,

    /// Calibration-related health condition.
    Calibration,

    /// Connectivity/topology-related health condition.
    Connectivity,

    /// Execution-path health condition.
    Execution,

    /// Readout/measurement health condition.
    Readout,

    /// Control-path health condition.
    Control,

    /// Memory/storage-related execution health condition.
    Memory,

    /// Thermal/environmental health condition.
    Environment,

    /// Quantum error-correction related health condition.
    ///
    /// This is a classification of the observation, not QEC semantics.
    ErrorCorrection,

    /// Provider/backend observation.
    Backend,

    /// Network/transport observation.
    Network,

    /// Security-related health observation.
    ///
    /// This does not itself establish a security incident.
    Security,

    /// Unknown or uncategorized condition.
    Unknown,
}

impl HealthConditionKind {
    /// Returns the stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Operational => "operational",
            Self::Availability => "availability",
            Self::Stability => "stability",
            Self::Performance => "performance",
            Self::Calibration => "calibration",
            Self::Connectivity => "connectivity",
            Self::Execution => "execution",
            Self::Readout => "readout",
            Self::Control => "control",
            Self::Memory => "memory",
            Self::Environment => "environment",
            Self::ErrorCorrection => "error_correction",
            Self::Backend => "backend",
            Self::Network => "network",
            Self::Security => "security",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for HealthConditionKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Health condition
// ============================================================================

/// Immutable health condition attached to a health observation.
///
/// The condition is deliberately descriptive. It does not contain:
///
/// - recovery commands;
/// - credentials;
/// - resource identity;
/// - policy decisions;
/// - fault taxonomy;
/// - timestamps;
/// - confidence.
///
/// Those dimensions belong to their respective models.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HealthCondition {
    kind: HealthConditionKind,
    active: bool,
}

impl HealthCondition {
    /// Creates a health condition.
    #[must_use]
    pub const fn new(kind: HealthConditionKind, active: bool) -> Self {
        Self { kind, active }
    }

    /// Returns the condition category.
    #[must_use]
    pub const fn kind(&self) -> HealthConditionKind {
        self.kind
    }

    /// Returns whether the condition is currently active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.active
    }

    /// Returns whether this condition is informational only because it is
    /// currently inactive.
    #[must_use]
    pub const fn is_inactive(&self) -> bool {
        !self.active
    }

    /// Returns a copy with a different active state.
    #[must_use]
    pub const fn with_active(&self, active: bool) -> Self {
        Self {
            kind: self.kind,
            active,
        }
    }
}

// ============================================================================
// Health observation
// ============================================================================

/// Immutable, provider-independent health observation.
///
/// `HealthObservation` is intentionally small and composable. Resource
/// identity belongs to `model/resource.rs`.
///
/// This type can therefore be used for:
///
/// - logical qubits;
/// - physical qubits;
/// - couplings;
/// - devices;
/// - backends;
/// - execution channels;
/// - distributed resources;
/// - future quantum architectures.
///
/// No fixed resource cardinality is encoded.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HealthObservation {
    state: HealthState,
    condition: Option<HealthCondition>,
}

impl HealthObservation {
    /// Creates an observation with only a health state.
    #[must_use]
    pub const fn new(state: HealthState) -> Self {
        Self {
            state,
            condition: None,
        }
    }

    /// Creates an observation with a health state and condition.
    #[must_use]
    pub const fn with_condition(
        state: HealthState,
        condition: HealthCondition,
    ) -> Self {
        Self {
            state,
            condition: Some(condition),
        }
    }

    /// Returns the health state.
    #[must_use]
    pub const fn state(&self) -> HealthState {
        self.state
    }

    /// Returns the optional condition.
    #[must_use]
    pub const fn condition(&self) -> Option<&HealthCondition> {
        self.condition.as_ref()
    }

    /// Returns whether the observation is healthy.
    #[must_use]
    pub const fn is_healthy(&self) -> bool {
        self.state.is_healthy()
    }

    /// Returns whether the observation is degraded.
    #[must_use]
    pub const fn is_degraded(&self) -> bool {
        self.state.is_degraded()
    }

    /// Returns whether the observation requires attention.
    #[must_use]
    pub const fn requires_attention(&self) -> bool {
        self.state.requires_attention()
    }

    /// Returns whether the observation is structurally valid.
    ///
    /// An observation is structurally valid when:
    ///
    /// - its health state is a valid enum value;
    /// - any attached condition has a valid category.
    ///
    /// Since both are strongly typed, this primarily exists as a stable
    /// validation boundary for future schema evolution.
    #[must_use]
    pub const fn is_structurally_valid(&self) -> bool {
        true
    }

    /// Returns a copy with the supplied health state.
    #[must_use]
    pub const fn with_state(&self, state: HealthState) -> Self {
        Self {
            state,
            condition: self.condition,
        }
    }

    /// Returns a copy with a condition.
    #[must_use]
    pub const fn with_condition_value(
        &self,
        condition: HealthCondition,
    ) -> Self {
        Self {
            state: self.state,
            condition: Some(condition),
        }
    }

    /// Returns a copy without a condition.
    #[must_use]
    pub const fn without_condition(&self) -> Self {
        Self {
            state: self.state,
            condition: None,
        }
    }
}

impl Default for HealthObservation {
    fn default() -> Self {
        Self::new(HealthState::Unknown)
    }
}

// ============================================================================
// Health aggregation
// ============================================================================

/// Deterministically combines two health observations.
///
/// The more degraded health state is selected according to
/// [`HealthState::rank`].
///
/// Conditions are intentionally NOT automatically merged because combining
/// conditions requires domain knowledge and may produce misleading results.
///
/// Higher-level incident/diagnosis logic owns multi-condition correlation.
#[must_use]
pub const fn aggregate(
    left: HealthObservation,
    right: HealthObservation,
) -> HealthObservation {
    let state = left.state.max(right.state);

    if state == left.state {
        left
    } else {
        right
    }
}

/// Returns the most degraded health state in an iterator.
///
/// An empty iterator returns `None`.
///
/// This function performs no allocation and imposes no artificial collection
/// size limit.
#[must_use]
pub fn aggregate_states<I>(states: I) -> Option<HealthState>
where
    I: IntoIterator<Item = HealthState>,
{
    let mut result = None;

    for state in states {
        result = Some(match result {
            Some(current) => current.max(state),
            None => state,
        });
    }

    result
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_order_is_stable() {
        assert_eq!(HealthState::Unknown.rank(), 0);
        assert_eq!(HealthState::Healthy.rank(), 1);
        assert_eq!(HealthState::Degraded.rank(), 2);
        assert_eq!(HealthState::Unstable.rank(), 3);
        assert_eq!(HealthState::Unavailable.rank(), 4);
        assert_eq!(HealthState::Recovering.rank(), 5);
        assert_eq!(HealthState::Quarantined.rank(), 6);
        assert_eq!(HealthState::Retired.rank(), 7);
    }

    #[test]
    fn rank_round_trip_is_lossless() {
        let states = [
            HealthState::Unknown,
            HealthState::Healthy,
            HealthState::Degraded,
            HealthState::Unstable,
            HealthState::Unavailable,
            HealthState::Recovering,
            HealthState::Quarantined,
            HealthState::Retired,
        ];

        for state in states {
            assert_eq!(HealthState::from_rank(state.rank()), Some(state));
        }
    }

    #[test]
    fn unknown_rank_is_rejected() {
        assert_eq!(HealthState::from_rank(8), None);
        assert_eq!(HealthState::from_rank(u8::MAX), None);
    }

    #[test]
    fn stable_strings_are_correct() {
        assert_eq!(HealthState::Unknown.as_str(), "unknown");
        assert_eq!(HealthState::Healthy.as_str(), "healthy");
        assert_eq!(HealthState::Degraded.as_str(), "degraded");
        assert_eq!(HealthState::Unstable.as_str(), "unstable");
        assert_eq!(HealthState::Unavailable.as_str(), "unavailable");
        assert_eq!(HealthState::Recovering.as_str(), "recovering");
        assert_eq!(HealthState::Quarantined.as_str(), "quarantined");
        assert_eq!(HealthState::Retired.as_str(), "retired");
    }

    #[test]
    fn parsing_is_strict_and_deterministic() {
        assert_eq!(
            "unknown".parse::<HealthState>().unwrap(),
            HealthState::Unknown
        );

        assert_eq!(
            "healthy".parse::<HealthState>().unwrap(),
            HealthState::Healthy
        );

        assert!("Healthy".parse::<HealthState>().is_err());
        assert!("ERROR".parse::<HealthState>().is_err());
        assert!("".parse::<HealthState>().is_err());
    }

    #[test]
    fn display_matches_stable_representation() {
        assert_eq!(HealthState::Healthy.to_string(), "healthy");
        assert_eq!(HealthState::Quarantined.to_string(), "quarantined");
    }

    #[test]
    fn predicates_are_consistent() {
        assert!(HealthState::Healthy.is_healthy());
        assert!(HealthState::Degraded.is_degraded());
        assert!(HealthState::Unstable.is_unstable());
        assert!(HealthState::Unavailable.is_unavailable());
        assert!(HealthState::Recovering.is_recovering());
        assert!(HealthState::Quarantined.is_quarantined());
        assert!(HealthState::Retired.is_retired());
        assert!(HealthState::Unknown.is_unknown());
    }

    #[test]
    fn normal_usability_is_conservative() {
        assert!(HealthState::Healthy.is_normally_usable());
        assert!(HealthState::Degraded.is_normally_usable());

        assert!(!HealthState::Unknown.is_normally_usable());
        assert!(!HealthState::Unstable.is_normally_usable());
        assert!(!HealthState::Unavailable.is_normally_usable());
        assert!(!HealthState::Recovering.is_normally_usable());
        assert!(!HealthState::Quarantined.is_normally_usable());
        assert!(!HealthState::Retired.is_normally_usable());
    }

    #[test]
    fn retired_is_terminal() {
        assert!(HealthState::Retired.is_terminal());
        assert!(!HealthState::Healthy.is_terminal());
        assert!(!HealthState::Retired.can_transition_to(HealthState::Healthy));
        assert!(HealthState::Retired.can_transition_to(HealthState::Retired));
    }

    #[test]
    fn recovery_paths_are_supported() {
        assert!(HealthState::Unavailable.can_transition_to(HealthState::Recovering));
        assert!(HealthState::Recovering.can_transition_to(HealthState::Healthy));
        assert!(HealthState::Recovering.can_transition_to(HealthState::Degraded));
        assert!(HealthState::Quarantined.can_transition_to(HealthState::Recovering));
    }

    #[test]
    fn degradation_paths_are_supported() {
        assert!(HealthState::Healthy.can_transition_to(HealthState::Degraded));
        assert!(HealthState::Degraded.can_transition_to(HealthState::Unstable));
        assert!(HealthState::Unstable.can_transition_to(HealthState::Unavailable));
    }

    #[test]
    fn aggregation_is_conservative() {
        assert_eq!(
            HealthState::Healthy.max(HealthState::Degraded),
            HealthState::Degraded
        );

        assert_eq!(
            HealthState::Degraded.max(HealthState::Unavailable),
            HealthState::Unavailable
        );

        assert_eq!(
            HealthState::Healthy.min(HealthState::Degraded),
            HealthState::Healthy
        );
    }

    #[test]
    fn aggregate_states_handles_empty_input() {
        let states: [HealthState; 0] = [];

        assert_eq!(aggregate_states(states), None);
    }

    #[test]
    fn aggregate_states_handles_arbitrary_input_size() {
        let states = [
            HealthState::Healthy,
            HealthState::Degraded,
            HealthState::Healthy,
            HealthState::Unstable,
            HealthState::Degraded,
        ];

        assert_eq!(
            aggregate_states(states),
            Some(HealthState::Unstable)
        );
    }

    #[test]
    fn condition_is_immutable_value_data() {
        let condition =
            HealthCondition::new(HealthConditionKind::Calibration, true);

        assert_eq!(
            condition.kind(),
            HealthConditionKind::Calibration
        );
        assert!(condition.is_active());
        assert!(!condition.is_inactive());

        let inactive = condition.with_active(false);

        assert!(!inactive.is_active());
        assert!(inactive.is_inactive());
    }

    #[test]
    fn observation_can_be_created_without_condition() {
        let observation = HealthObservation::new(HealthState::Healthy);

        assert_eq!(observation.state(), HealthState::Healthy);
        assert!(observation.condition().is_none());
        assert!(observation.is_healthy());
        assert!(!observation.requires_attention());
        assert!(observation.is_structurally_valid());
    }

    #[test]
    fn observation_can_carry_condition() {
        let condition =
            HealthCondition::new(HealthConditionKind::Performance, true);

        let observation = HealthObservation::with_condition(
            HealthState::Degraded,
            condition,
        );

        assert_eq!(observation.state(), HealthState::Degraded);
        assert!(observation.condition().is_some());
        assert!(observation.is_degraded());
        assert!(observation.requires_attention());
    }

    #[test]
    fn observation_is_persistent() {
        let original = HealthObservation::new(HealthState::Healthy);
        let changed = original.with_state(HealthState::Degraded);

        assert_eq!(original.state(), HealthState::Healthy);
        assert_eq!(changed.state(), HealthState::Degraded);
    }

    #[test]
    fn aggregate_preserves_the_more_degraded_observation() {
        let healthy = HealthObservation::new(HealthState::Healthy);

        let degraded = HealthObservation::with_condition(
            HealthState::Degraded,
            HealthCondition::new(
                HealthConditionKind::Calibration,
                true,
            ),
        );

        let result = aggregate(healthy, degraded.clone());

        assert_eq!(result, degraded);
    }

    #[test]
    fn numeric_conversion_is_lossless() {
        let states = [
            HealthState::Unknown,
            HealthState::Healthy,
            HealthState::Degraded,
            HealthState::Unstable,
            HealthState::Unavailable,
            HealthState::Recovering,
            HealthState::Quarantined,
            HealthState::Retired,
        ];

        for state in states {
            let encoded: u8 = state.into();
            let decoded =
                HealthState::try_from(encoded).expect("valid health rank");

            assert_eq!(decoded, state);
        }
    }

    #[test]
    fn invalid_numeric_conversion_is_rejected() {
        assert!(HealthState::try_from(8_u8).is_err());
        assert!(HealthState::try_from(u8::MAX).is_err());
    }

    #[test]
    fn condition_kind_strings_are_stable() {
        assert_eq!(
            HealthConditionKind::Operational.as_str(),
            "operational"
        );

        assert_eq!(
            HealthConditionKind::ErrorCorrection.as_str(),
            "error_correction"
        );

        assert_eq!(
            HealthConditionKind::Unknown.as_str(),
            "unknown"
        );
    }

    #[test]
    fn default_is_unknown() {
        assert_eq!(HealthState::default(), HealthState::Unknown);
        assert_eq!(
            HealthObservation::default().state(),
            HealthState::Unknown
        );
    }

    #[test]
    fn no_health_state_is_authority() {
        // This test intentionally documents the semantic contract:
        // state predicates describe data and never authorize actions.
        assert!(HealthState::Healthy.is_normally_usable());
        assert!(HealthState::Retired.is_terminal());
    }
}