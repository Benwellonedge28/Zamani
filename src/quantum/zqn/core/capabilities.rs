//! Zamani Quantum Noise (ZQN) — Capability Model
//!
//! This module defines the provider-neutral capability vocabulary used by ZQN
//! to answer:
//!
//! > "Can a target, simulator, characterization system, or execution
//! > environment represent and/or realize this ZQN semantic requirement?"
//!
//! =============================================================================
//! Architectural ownership
//! =============================================================================
//!
//! This file owns:
//!
//! - capability identifiers;
//! - capability namespaces;
//! - capability scopes;
//! - capability support levels;
//! - capability requirements;
//! - capability sets;
//! - capability matching;
//! - capability diagnostics;
//! - capability identity and deterministic ordering;
//! - capability validation;
//! - capability-independent qubit scoping.
//!
//! This file does NOT own:
//!
//! - quantum channels;
//! - noise models;
//! - faults;
//! - calibration data;
//! - hardware discovery;
//! - vendor APIs;
//! - target transport;
//! - routing;
//! - scheduling;
//! - simulation;
//! - QEC algorithms;
//! - benchmarking;
//! - source-language parsing;
//! - execution;
//! - resource allocation.
//!
//! Those concerns belong to their respective subsystems.
//!
//! =============================================================================
//! Canonical identity rule
//! =============================================================================
//!
//! ZQN MUST NOT define another QubitId or PhysicalQubitId.
//!
//! The canonical identities are:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! The surrounding repository explicitly establishes `quantum::ir::qubit` as
//! the authoritative identity boundary.
//!
//! Capability scopes therefore use those canonical types directly.
//!
//! =============================================================================
//! Write once, scale everywhere
//! =============================================================================
//!
//! This module deliberately contains no:
//!
//! - MAX_QUBITS;
//! - MAX_CAPABILITIES;
//! - MAX_RESOURCES;
//! - MAX_OPERATION_ARITY;
//! - MAX_CORRELATION_SIZE;
//! - vendor-specific device count;
//! - fixed topology;
//! - fixed qubit count;
//! - hardware-specific assumptions.
//!
//! A capability set is finite because the program representation is finite,
//! not because ZQN imposes a semantic upper bound.
//!
//! Concrete resource limits belong to the caller's resource policy.
//!
//! =============================================================================
//! Extensibility rule
//! =============================================================================
//!
//! Capability identifiers are namespaced strings rather than an exhaustive
//! Rust enum. This is intentional.
//!
//! A closed enum would require this file to be modified whenever a new quantum
//! technology introduces a capability. That would violate ZQN's extensibility
//! and "write once, scale everywhere" objective.
//!
//! Built-in capability constructors are provided for standardized concepts,
//! while arbitrary future capabilities may use `CapabilityId::new(...)`.
//!
//! Example:
//!
//!     CapabilityId::new("zqn.noise.my_future_capability")
//!
//! Vendor-specific identifiers should normally live in an adapter namespace,
//! for example:
//!
//!     provider.example.capability.some_feature
//!
//! rather than being added to the ZQN core vocabulary.
//!
//! =============================================================================
//! Capability semantics
//! =============================================================================
//!
//! A capability answers what a target CAN do.
//!
//! It does not answer:
//!
//! - whether the compiler SHOULD use it;
//! - whether a particular circuit needs it;
//! - whether a model is physically accurate;
//! - whether a calibration is currently valid;
//! - whether an approximation is acceptable.
//!
//! Those decisions belong to higher-level policies.
//!
//! =============================================================================
//! Support semantics
//! =============================================================================
//!
//! Capability support is intentionally richer than a boolean.
//!
//! A target may:
//!
//! - provide a capability natively;
//! - provide it through an exact emulation;
//! - provide only an approximation;
//! - explicitly not provide it.
//!
//! ZQN must never silently turn an approximation into exact support.
//!
//! =============================================================================
//! Determinism
//! =============================================================================
//!
//! Capability values are immutable value objects.
//!
//! They contain no:
//!
//! - global state;
//! - random state;
//! - clocks;
//! - network connections;
//! - device handles.
//!
//! Equality, ordering and hashing therefore remain deterministic.
//!
//! =============================================================================
//! Safety
//! =============================================================================
//!
//! This module is safe Rust.
//!
//! No unsafe code is permitted.
//!
//! =============================================================================
//! Rust compatibility
//! =============================================================================
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! No external dependency is required by this file.
//!
//! =============================================================================
//! Integration contract
//! =============================================================================
//!
//! Producers:
//!
//! - hardware target descriptions;
//! - simulator descriptions;
//! - emulators;
//! - characterization systems;
//! - execution environments;
//! - future quantum technologies.
//!
//! Consumers:
//!
//! - `zqn::target`;
//! - `zqn::noise`;
//! - `zqn::channel`;
//! - `zqn::simulation`;
//! - `zqn::calibration`;
//! - `zqn::integration::ir`;
//! - `zqn::integration::routing`;
//! - `zqn::integration::scheduling`;
//! - `zqn::integration::qec`;
//! - `zqn::integration::hardware`;
//! - `zqn::integration::runtime`;
//! - `zqn::integration::benchmarking`.
//!
//! Expected direction:
//!
//!     semantic requirement
//!              │
//!              ▼
//!     ZQN capability requirement
//!              │
//!              ▼
//!     target capability set
//!              │
//!              ▼
//!     compatibility evaluation
//!              │
//!       ┌──────┴──────┐
//!       ▼             ▼
//!    compatible   incompatible
//!
//! This file does not invoke any of those consumers directly.
//!
//! =============================================================================

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Capability identifier
// =============================================================================

/// A globally namespaced ZQN capability identifier.
///
/// Capability identifiers are deliberately extensible.
///
/// The identifier consists of a non-empty namespace-like string. ZQN reserves
/// the `zqn.` namespace for standardized capability vocabulary.
///
/// Other namespaces may be used by:
///
/// - hardware adapters;
/// - simulators;
/// - research implementations;
/// - future quantum technologies.
///
/// No capability identifier implies a particular vendor or machine size.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CapabilityId(String);

impl CapabilityId {
    /// Namespace prefix reserved for standardized ZQN capabilities.
    pub const ZQN_NAMESPACE: &'static str = "zqn.";

    /// Creates a capability identifier.
    ///
    /// The identifier must:
    ///
    /// - be non-empty;
    /// - not contain ASCII control characters;
    /// - not contain leading/trailing whitespace.
    ///
    /// The method does not restrict the namespace so that future providers and
    /// experimental implementations can define their own capabilities.
    pub fn new<S>(value: S) -> Result<Self, CapabilityIdError>
    where
        S: Into<String>,
    {
        let value = value.into();

        validate_capability_identifier(&value)?;

        Ok(Self(value))
    }

    /// Creates a standardized ZQN capability identifier.
    ///
    /// This constructor is intentionally stricter than `new`: the resulting
    /// identifier must live in the reserved `zqn.` namespace.
    pub fn zqn<S>(name: S) -> Result<Self, CapabilityIdError>
    where
        S: AsRef<str>,
    {
        let name = name.as_ref();

        if name.is_empty() {
            return Err(CapabilityIdError::Empty);
        }

        let value = if name.starts_with(Self::ZQN_NAMESPACE) {
            name.to_owned()
        } else {
            format!("{}{}", Self::ZQN_NAMESPACE, name)
        };

        Self::new(value)
    }

    /// Returns the canonical identifier string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether this capability belongs to the standardized ZQN
    /// namespace.
    #[must_use]
    pub fn is_zqn(&self) -> bool {
        self.0.starts_with(Self::ZQN_NAMESPACE)
    }

    /// Returns the namespace portion.
    ///
    /// For `zqn.noise.readout`, this returns `zqn`.
    #[must_use]
    pub fn namespace(&self) -> &str {
        match self.0.find('.') {
            Some(index) => &self.0[..index],
            None => &self.0,
        }
    }

    // -------------------------------------------------------------------------
    // Standard ZQN capability constructors
    // -------------------------------------------------------------------------

    /// Exact/general quantum-channel support.
    #[must_use]
    pub fn quantum_channels() -> Self {
        Self::standard("channel")
    }

    /// Kraus representation support.
    #[must_use]
    pub fn channel_kraus() -> Self {
        Self::standard("channel.kraus")
    }

    /// Choi representation support.
    #[must_use]
    pub fn channel_choi() -> Self {
        Self::standard("channel.choi")
    }

    /// Superoperator/Liouville representation support.
    #[must_use]
    pub fn channel_superoperator() -> Self {
        Self::standard("channel.superoperator")
    }

    /// Pauli-transfer representation support.
    #[must_use]
    pub fn channel_pauli_transfer() -> Self {
        Self::standard("channel.pauli_transfer")
    }

    /// Stochastic-channel support.
    #[must_use]
    pub fn channel_stochastic() -> Self {
        Self::standard("channel.stochastic")
    }

    /// Lindblad/continuous-time model support.
    #[must_use]
    pub fn channel_lindblad() -> Self {
        Self::standard("channel.lindblad")
    }

    /// Thermal-noise support.
    #[must_use]
    pub fn noise_thermal() -> Self {
        Self::standard("noise.thermal")
    }

    /// Amplitude-damping support.
    #[must_use]
    pub fn noise_amplitude_damping() -> Self {
        Self::standard("noise.amplitude_damping")
    }

    /// Phase-damping/dephasing support.
    #[must_use]
    pub fn noise_phase_damping() -> Self {
        Self::standard("noise.phase_damping")
    }

    /// Depolarizing-noise support.
    #[must_use]
    pub fn noise_depolarizing() -> Self {
        Self::standard("noise.depolarizing")
    }

    /// Gate-noise support.
    #[must_use]
    pub fn noise_gate() -> Self {
        Self::standard("noise.gate")
    }

    /// Preparation-noise support.
    #[must_use]
    pub fn noise_preparation() -> Self {
        Self::standard("noise.preparation")
    }

    /// Reset-noise support.
    #[must_use]
    pub fn noise_reset() -> Self {
        Self::standard("noise.reset")
    }

    /// Measurement-noise support.
    #[must_use]
    pub fn noise_measurement() -> Self {
        Self::standard("noise.measurement")
    }

    /// Readout-error support.
    #[must_use]
    pub fn noise_readout() -> Self {
        Self::standard("noise.readout")
    }

    /// Idle-noise support.
    #[must_use]
    pub fn noise_idle() -> Self {
        Self::standard("noise.idle")
    }

    /// Pulse-noise support.
    #[must_use]
    pub fn noise_pulse() -> Self {
        Self::standard("noise.pulse")
    }

    /// Leakage support.
    #[must_use]
    pub fn noise_leakage() -> Self {
        Self::standard("noise.leakage")
    }

    /// Erasure support.
    #[must_use]
    pub fn noise_erasure() -> Self {
        Self::standard("noise.erasure")
    }

    /// Loss support.
    #[must_use]
    pub fn noise_loss() -> Self {
        Self::standard("noise.loss")
    }

    /// Correlated-noise support.
    #[must_use]
    pub fn noise_correlated() -> Self {
        Self::standard("noise.correlated")
    }

    /// Spatial-correlation support.
    #[must_use]
    pub fn noise_spatial_correlation() -> Self {
        Self::standard("noise.correlation.spatial")
    }

    /// Temporal-correlation support.
    #[must_use]
    pub fn noise_temporal_correlation() -> Self {
        Self::standard("noise.correlation.temporal")
    }

    /// Crosstalk support.
    #[must_use]
    pub fn noise_crosstalk() -> Self {
        Self::standard("noise.crosstalk")
    }

    /// Non-Markovian-noise support.
    #[must_use]
    pub fn noise_non_markovian() -> Self {
        Self::standard("noise.non_markovian")
    }

    /// Conditional/dynamic noise support.
    #[must_use]
    pub fn noise_conditional() -> Self {
        Self::standard("noise.conditional")
    }

    /// Dynamic/time-dependent noise support.
    #[must_use]
    pub fn noise_time_dependent() -> Self {
        Self::standard("noise.time_dependent")
    }

    /// Calibration-aware noise support.
    #[must_use]
    pub fn calibration_aware() -> Self {
        Self::standard("calibration.aware")
    }

    /// Time-varying calibration support.
    #[must_use]
    pub fn calibration_time_dependent() -> Self {
        Self::standard("calibration.time_dependent")
    }

    /// Characterization support.
    #[must_use]
    pub fn characterization() -> Self {
        Self::standard("characterization")
    }

    /// Process tomography support.
    #[must_use]
    pub fn characterization_process_tomography() -> Self {
        Self::standard("characterization.process_tomography")
    }

    /// Randomized-benchmarking support.
    #[must_use]
    pub fn characterization_randomized_benchmarking() -> Self {
        Self::standard("characterization.randomized_benchmarking")
    }

    /// Exact deterministic simulation support.
    #[must_use]
    pub fn simulation_deterministic() -> Self {
        Self::standard("simulation.deterministic")
    }

    /// Monte-Carlo simulation support.
    #[must_use]
    pub fn simulation_monte_carlo() -> Self {
        Self::standard("simulation.monte_carlo")
    }

    /// Quantum-trajectory simulation support.
    #[must_use]
    pub fn simulation_trajectory() -> Self {
        Self::standard("simulation.trajectory")
    }

    /// Reproducible stochastic execution support.
    #[must_use]
    pub fn reproducible_sampling() -> Self {
        Self::standard("execution.reproducible_sampling")
    }

    /// Deterministic parallel sampling support.
    #[must_use]
    pub fn deterministic_parallel_sampling() -> Self {
        Self::standard("execution.deterministic_parallel_sampling")
    }

    /// Runtime cancellation support.
    #[must_use]
    pub fn cancellation() -> Self {
        Self::standard("execution.cancellation")
    }

    /// Provenance support.
    #[must_use]
    pub fn provenance() -> Self {
        Self::standard("provenance")
    }

    /// Uncertainty representation support.
    #[must_use]
    pub fn uncertainty() -> Self {
        Self::standard("uncertainty")
    }

    /// Error-budget support.
    #[must_use]
    pub fn error_budget() -> Self {
        Self::standard("analysis.error_budget")
    }

    /// Returns an arbitrary standardized ZQN identifier without exposing the
    /// constructor's validation error.
    ///
    /// All strings supplied here are compile-time constants in this module.
    fn standard(name: &str) -> Self {
        debug_assert!(name.starts_with("zqn.") || !name.is_empty());

        if name.starts_with(Self::ZQN_NAMESPACE) {
            // All internal standard names are statically valid.
            Self(name.to_owned())
        } else {
            Self(format!("{}{}", Self::ZQN_NAMESPACE, name))
        }
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl AsRef<str> for CapabilityId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for CapabilityId {
    type Err = CapabilityIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

// =============================================================================
// Capability identifier errors
// =============================================================================

/// Validation errors for capability identifiers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CapabilityIdError {
    /// Identifier is empty.
    Empty,

    /// Identifier begins or ends with whitespace.
    BoundaryWhitespace,

    /// Identifier contains an ASCII control character.
    ControlCharacter,
}

impl fmt::Display for CapabilityIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("capability identifier must not be empty"),
            Self::BoundaryWhitespace => {
                formatter.write_str("capability identifier must not have boundary whitespace")
            }
            Self::ControlCharacter => {
                formatter.write_str("capability identifier must not contain control characters")
            }
        }
    }
}

impl std::error::Error for CapabilityIdError {}

fn validate_capability_identifier(value: &str) -> Result<(), CapabilityIdError> {
    if value.is_empty() {
        return Err(CapabilityIdError::Empty);
    }

    if value.trim() != value {
        return Err(CapabilityIdError::BoundaryWhitespace);
    }

    if value.chars().any(char::is_control) {
        return Err(CapabilityIdError::ControlCharacter);
    }

    Ok(())
}

// =============================================================================
// Capability scope
// =============================================================================

/// Scope at which a capability applies.
///
/// The logical and physical qubit variants use the canonical IR identity types.
/// No duplicate ZQN qubit identity is introduced.
///
/// `Resource` exists for quantum resources that are not naturally represented
/// as a qubit, such as:
///
/// - modes;
/// - links;
/// - measurement resources;
/// - pulse channels;
/// - logical blocks;
/// - future quantum technologies.
///
/// The resource identifier is descriptive only; existence and allocation belong
/// to the owning subsystem.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CapabilityScope {
    /// Capability applies globally to the target or execution environment.
    Global,

    /// Capability applies to one canonical logical qubit.
    LogicalQubit(QubitId),

    /// Capability applies to one canonical physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Capability applies to an explicitly named non-qubit resource.
    Resource(String),
}

impl CapabilityScope {
    /// Creates a resource-scoped capability.
    ///
    /// Empty identifiers are rejected because an empty resource name cannot
    /// provide a deterministic integration identity.
    pub fn resource<S>(identifier: S) -> Result<Self, ScopeError>
    where
        S: Into<String>,
    {
        let identifier = identifier.into();

        if identifier.is_empty() {
            return Err(ScopeError::EmptyResourceIdentifier);
        }

        if identifier.trim() != identifier {
            return Err(ScopeError::BoundaryWhitespace);
        }

        if identifier.chars().any(char::is_control) {
            return Err(ScopeError::ControlCharacter);
        }

        Ok(Self::Resource(identifier))
    }

    /// Returns whether this is a global scope.
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self, Self::Global)
    }

    /// Returns the logical qubit when scoped to one.
    #[must_use]
    pub const fn logical_qubit(&self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns the physical qubit when scoped to one.
    #[must_use]
    pub const fn physical_qubit(&self) -> Option<PhysicalQubitId> {
        match self {
            Self::PhysicalQubit(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns the arbitrary resource identifier when applicable.
    #[must_use]
    pub fn resource_identifier(&self) -> Option<&str> {
        match self {
            Self::Resource(identifier) => Some(identifier.as_str()),
            _ => None,
        }
    }
}

impl fmt::Display for CapabilityScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Global => formatter.write_str("global"),
            Self::LogicalQubit(id) => write!(formatter, "logical:{id}"),
            Self::PhysicalQubit(id) => write!(formatter, "physical:{id}"),
            Self::Resource(identifier) => write!(formatter, "resource:{identifier}"),
        }
    }
}

/// Errors produced while constructing a capability scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScopeError {
    /// Resource identifier was empty.
    EmptyResourceIdentifier,

    /// Resource identifier contains boundary whitespace.
    BoundaryWhitespace,

    /// Resource identifier contains an ASCII control character.
    ControlCharacter,
}

impl fmt::Display for ScopeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyResourceIdentifier => {
                formatter.write_str("resource identifier must not be empty")
            }
            Self::BoundaryWhitespace => {
                formatter.write_str("resource identifier must not have boundary whitespace")
            }
            Self::ControlCharacter => {
                formatter.write_str("resource identifier must not contain control characters")
            }
        }
    }
}

impl std::error::Error for ScopeError {}

// =============================================================================
// Support level
// =============================================================================

/// How a capability is realized by a target.
///
/// This distinction is essential for scientifically correct execution.
///
/// `Native` and `Emulated` may both represent the requested semantics exactly.
/// `Approximate` explicitly communicates that the requested semantics are not
/// realized exactly.
///
/// Higher-level policy decides whether an approximation is acceptable.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum SupportLevel {
    /// Capability is directly provided by the target.
    Native,

    /// Capability is provided exactly through a software or other emulation
    /// layer rather than natively by the target.
    Emulated,

    /// Capability is available only through an explicitly approximate
    /// realization.
    Approximate,

    /// Capability is not supported.
    Unsupported,
}

impl SupportLevel {
    /// Returns whether the capability can be realized exactly.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Native | Self::Emulated)
    }

    /// Returns whether this level represents any usable realization.
    ///
    /// `Approximate` is considered usable at this low-level classification;
    /// policy must decide whether the approximation may actually be selected.
    #[must_use]
    pub const fn is_supported(self) -> bool {
        !matches!(self, Self::Unsupported)
    }

    /// Returns whether the realization is native.
    #[must_use]
    pub const fn is_native(self) -> bool {
        matches!(self, Self::Native)
    }

    /// Returns whether the realization is emulated.
    #[must_use]
    pub const fn is_emulated(self) -> bool {
        matches!(self, Self::Emulated)
    }

    /// Returns whether the realization is approximate.
    #[must_use]
    pub const fn is_approximate(self) -> bool {
        matches!(self, Self::Approximate)
    }

    /// Returns whether the capability is unsupported.
    #[must_use]
    pub const fn is_unsupported(self) -> bool {
        matches!(self, Self::Unsupported)
    }
}

impl Default for SupportLevel {
    fn default() -> Self {
        Self::Unsupported
    }
}

impl fmt::Display for SupportLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native => formatter.write_str("native"),
            Self::Emulated => formatter.write_str("emulated"),
            Self::Approximate => formatter.write_str("approximate"),
            Self::Unsupported => formatter.write_str("unsupported"),
        }
    }
}

// =============================================================================
// Capability value
// =============================================================================

/// A single capability declaration.
///
/// This is the atomic unit of a target capability set.
///
/// A declaration says:
///
///     capability X
///     at scope Y
///     has realization Z
///
/// It does not contain hardware capacity, calibration values or execution
/// state.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Capability {
    id: CapabilityId,
    scope: CapabilityScope,
    support: SupportLevel,
}

impl Capability {
    /// Creates a capability declaration.
    #[must_use]
    pub const fn new(
        id: CapabilityId,
        scope: CapabilityScope,
        support: SupportLevel,
    ) -> Self {
        Self {
            id,
            scope,
            support,
        }
    }

    /// Creates an exact native capability.
    #[must_use]
    pub const fn native(id: CapabilityId, scope: CapabilityScope) -> Self {
        Self::new(id, scope, SupportLevel::Native)
    }

    /// Creates an exact emulated capability.
    #[must_use]
    pub const fn emulated(id: CapabilityId, scope: CapabilityScope) -> Self {
        Self::new(id, scope, SupportLevel::Emulated)
    }

    /// Creates an explicitly approximate capability.
    #[must_use]
    pub const fn approximate(id: CapabilityId, scope: CapabilityScope) -> Self {
        Self::new(id, scope, SupportLevel::Approximate)
    }

    /// Returns the capability identifier.
    #[must_use]
    pub const fn id(&self) -> &CapabilityId {
        &self.id
    }

    /// Returns the capability scope.
    #[must_use]
    pub const fn scope(&self) -> &CapabilityScope {
        &self.scope
    }

    /// Returns the support level.
    #[must_use]
    pub const fn support(&self) -> SupportLevel {
        self.support
    }

    /// Returns whether the capability is exactly supported.
    #[must_use]
    pub const fn is_exact(&self) -> bool {
        self.support.is_exact()
    }

    /// Returns whether the capability is usable at all.
    #[must_use]
    pub const fn is_supported(&self) -> bool {
        self.support.is_supported()
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} @ {} ({})",
            self.id, self.scope, self.support
        )
    }
}

// =============================================================================
// Requirement policy
// =============================================================================

/// Minimum realization accepted for a capability requirement.
///
/// This is deliberately separate from `SupportLevel` so a requirement can
/// express policy independently of what a target happens to provide.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum SupportRequirement {
    /// Native implementation is required.
    Native,

    /// Any exact realization is acceptable, including emulation.
    Exact,

    /// Approximate realization is acceptable.
    Approximate,

    /// Presence of the capability is sufficient, regardless of realization
    /// classification.
    Any,
}

impl SupportRequirement {
    /// Tests whether a supplied support level satisfies this requirement.
    #[must_use]
    pub const fn accepts(self, actual: SupportLevel) -> bool {
        match self {
            Self::Native => matches!(actual, SupportLevel::Native),
            Self::Exact => actual.is_exact(),
            Self::Approximate => actual.is_supported(),
            Self::Any => actual.is_supported(),
        }
    }
}

impl Default for SupportRequirement {
    fn default() -> Self {
        Self::Exact
    }
}

impl fmt::Display for SupportRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native => formatter.write_str("native"),
            Self::Exact => formatter.write_str("exact"),
            Self::Approximate => formatter.write_str("approximate"),
            Self::Any => formatter.write_str("any"),
        }
    }
}

// =============================================================================
// Capability requirement
// =============================================================================

/// A requirement imposed by a ZQN consumer.
///
/// Requirements are immutable descriptions of what must be available.
///
/// A requirement may be:
///
/// - global;
/// - logical-qubit scoped;
/// - physical-qubit scoped;
/// - scoped to another resource.
///
/// The requirement never performs target discovery itself.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CapabilityRequirement {
    id: CapabilityId,
    scope: CapabilityScope,
    support: SupportRequirement,
}

impl CapabilityRequirement {
    /// Creates a capability requirement.
    #[must_use]
    pub const fn new(
        id: CapabilityId,
        scope: CapabilityScope,
        support: SupportRequirement,
    ) -> Self {
        Self {
            id,
            scope,
            support,
        }
    }

    /// Requires native support.
    #[must_use]
    pub const fn native(id: CapabilityId, scope: CapabilityScope) -> Self {
        Self::new(id, scope, SupportRequirement::Native)
    }

    /// Requires exact support.
    #[must_use]
    pub const fn exact(id: CapabilityId, scope: CapabilityScope) -> Self {
        Self::new(id, scope, SupportRequirement::Exact)
    }

    /// Allows an explicit approximation.
    #[must_use]
    pub const fn approximate(id: CapabilityId, scope: CapabilityScope) -> Self {
        Self::new(id, scope, SupportRequirement::Approximate)
    }

    /// Accepts any supported realization.
    #[must_use]
    pub const fn any(id: CapabilityId, scope: CapabilityScope) -> Self {
        Self::new(id, scope, SupportRequirement::Any)
    }

    /// Returns the identifier.
    #[must_use]
    pub const fn id(&self) -> &CapabilityId {
        &self.id
    }

    /// Returns the required scope.
    #[must_use]
    pub const fn scope(&self) -> &CapabilityScope {
        &self.scope
    }

    /// Returns the minimum accepted realization.
    #[must_use]
    pub const fn support(&self) -> SupportRequirement {
        self.support
    }
}

impl fmt::Display for CapabilityRequirement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} @ {} ({})",
            self.id, self.scope, self.support
        )
    }
}

// =============================================================================
// Capability sets
// =============================================================================

/// Deterministically ordered collection of capability declarations.
///
/// `BTreeSet` is used deliberately:
///
/// - deterministic iteration;
/// - no duplicate declarations;
/// - stable equality;
/// - stable ordering;
/// - no hidden mutable global state.
///
/// The set itself imposes no semantic limit on the number of capabilities.
///
/// Any actual memory limit comes from the host and the caller's resource
/// policy.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CapabilitySet {
    entries: BTreeSet<Capability>,
}

impl CapabilitySet {
    /// Creates an empty capability set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a capability set from an iterator.
    pub fn from_iter<I>(capabilities: I) -> Self
    where
        I: IntoIterator<Item = Capability>,
    {
        Self {
            entries: capabilities.into_iter().collect(),
        }
    }

    /// Inserts a capability.
    ///
    /// Returns `true` if the set changed.
    pub fn insert(&mut self, capability: Capability) -> bool {
        self.entries.insert(capability)
    }

    /// Removes an exact capability declaration.
    ///
    /// Returns whether an entry was removed.
    pub fn remove(&mut self, capability: &Capability) -> bool {
        self.entries.remove(capability)
    }

    /// Returns whether the exact declaration exists.
    #[must_use]
    pub fn contains(&self, capability: &Capability) -> bool {
        self.entries.contains(capability)
    }

    /// Returns the number of declarations.
    ///
    /// This is a representation size, not a hardware-size limit.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether no declarations are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterates deterministically over declarations.
    pub fn iter(&self) -> impl Iterator<Item = &Capability> {
        self.entries.iter()
    }

    /// Finds all declarations matching an identifier.
    pub fn by_id<'a>(
        &'a self,
        id: &'a CapabilityId,
    ) -> impl Iterator<Item = &'a Capability> + 'a {
        self.entries.iter().filter(move |capability| capability.id() == id)
    }

    /// Returns whether the exact capability is supported at the required
    /// level.
    #[must_use]
    pub fn satisfies(&self, requirement: &CapabilityRequirement) -> bool {
        self.match_requirement(requirement).is_some()
    }

    /// Finds a declaration satisfying the requirement.
    ///
    /// The result is deterministic because the underlying set is ordered.
    #[must_use]
    pub fn match_requirement(
        &self,
        requirement: &CapabilityRequirement,
    ) -> Option<&Capability> {
        self.entries.iter().find(|capability| {
            capability.id() == requirement.id()
                && capability.scope() == requirement.scope()
                && requirement.support().accepts(capability.support())
        })
    }

    /// Checks all requirements.
    ///
    /// The returned vector contains only unsatisfied requirements and therefore
    /// provides deterministic diagnostics.
    #[must_use]
    pub fn missing(
        &self,
        requirements: &[CapabilityRequirement],
    ) -> Vec<CapabilityRequirement> {
        requirements
            .iter()
            .filter(|requirement| !self.satisfies(requirement))
            .cloned()
            .collect()
    }

    /// Returns whether all requirements are satisfied.
    #[must_use]
    pub fn satisfies_all(&self, requirements: &[CapabilityRequirement]) -> bool {
        requirements.iter().all(|requirement| self.satisfies(requirement))
    }

    /// Returns a cloned vector in deterministic order.
    #[must_use]
    pub fn to_vec(&self) -> Vec<Capability> {
        self.entries.iter().cloned().collect()
    }
}

impl IntoIterator for CapabilitySet {
    type Item = Capability;
    type IntoIter = std::collections::btree_set::IntoIter<Capability>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<'a> IntoIterator for &'a CapabilitySet {
    type Item = &'a Capability;
    type IntoIter = std::collections::btree_set::Iter<'a, Capability>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

// =============================================================================
// Compatibility result
// =============================================================================

/// Result of matching one requirement against a capability set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CapabilityMatch {
    /// Requirement is satisfied by exact/native/emulated support.
    Satisfied {
        /// The declaration that satisfied the requirement.
        capability: Capability,
    },

    /// Requirement exists only through an approximation.
    Approximate {
        /// The declaration providing the approximation.
        capability: Capability,
    },

    /// No matching capability exists at the required scope.
    Missing {
        /// The original requirement.
        requirement: CapabilityRequirement,
    },
}

impl CapabilityMatch {
    /// Returns whether the requirement is fully satisfied exactly.
    #[must_use]
    pub fn is_exact(&self) -> bool {
        matches!(self, Self::Satisfied { .. })
    }

    /// Returns whether the result is an explicit approximation.
    #[must_use]
    pub fn is_approximate(&self) -> bool {
        matches!(self, Self::Approximate { .. })
    }

    /// Returns whether the capability is missing.
    #[must_use]
    pub fn is_missing(&self) -> bool {
        matches!(self, Self::Missing { .. })
    }
}

// =============================================================================
// Capability evaluation
// =============================================================================

/// Evaluates one capability requirement without applying policy outside the
/// requirement's declared support level.
///
/// This function is useful when callers need to preserve the distinction
/// between exact and approximate realization.
#[must_use]
pub fn evaluate_requirement(
    capabilities: &CapabilitySet,
    requirement: &CapabilityRequirement,
) -> CapabilityMatch {
    if let Some(capability) = capabilities.match_requirement(requirement) {
        if capability.support().is_approximate() {
            CapabilityMatch::Approximate {
                capability: capability.clone(),
            }
        } else {
            CapabilityMatch::Satisfied {
                capability: capability.clone(),
            }
        }
    } else {
        CapabilityMatch::Missing {
            requirement: requirement.clone(),
        }
    }
}

// =============================================================================
// Capability identity helpers
// =============================================================================

/// Stable capability-set fingerprint input.
///
/// This type intentionally does not calculate a cryptographic digest.
///
/// Hashing belongs to the repository's canonical hashing subsystem once ZQN
/// identity is integrated with it.
///
/// The method exists so integration layers can obtain deterministic capability
/// ordering without depending on internal storage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalCapabilityEntry {
    /// Capability identifier.
    pub id: CapabilityId,

    /// Capability scope.
    pub scope: CapabilityScope,

    /// Support level.
    pub support: SupportLevel,
}

impl From<&Capability> for CanonicalCapabilityEntry {
    fn from(value: &Capability) -> Self {
        Self {
            id: value.id().clone(),
            scope: value.scope().clone(),
            support: value.support(),
        }
    }
}

// =============================================================================
// Standard capability profiles
// =============================================================================

/// Creates the minimum generic capability profile for a system that can
/// represent ordinary quantum channels and reproducibly sample stochastic
/// behavior.
///
/// This is a convenience constructor, not a hardware assumption.
///
/// It deliberately does not claim support for:
///
/// - a specific number of qubits;
/// - a specific gate set;
/// - a specific technology;
/// - a specific noise channel family.
#[must_use]
pub fn generic_noise_profile() -> CapabilitySet {
    let global = CapabilityScope::Global;

    CapabilitySet::from_iter([
        Capability::native(CapabilityId::quantum_channels(), global.clone()),
        Capability::native(
            CapabilityId::reproducible_sampling(),
            global.clone(),
        ),
        Capability::native(CapabilityId::provenance(), global),
    ])
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_identifier_rejects_empty_values() {
        assert_eq!(
            CapabilityId::new(""),
            Err(CapabilityIdError::Empty)
        );
    }

    #[test]
    fn capability_identifier_rejects_boundary_whitespace() {
        assert_eq!(
            CapabilityId::new(" zqn.noise"),
            Err(CapabilityIdError::BoundaryWhitespace)
        );

        assert_eq!(
            CapabilityId::new("zqn.noise "),
            Err(CapabilityIdError::BoundaryWhitespace)
        );
    }

    #[test]
    fn capability_identifier_rejects_control_characters() {
        assert_eq!(
            CapabilityId::new("zqn.noise\nreadout"),
            Err(CapabilityIdError::ControlCharacter)
        );
    }

    #[test]
    fn zqn_identifier_adds_namespace() {
        let id = CapabilityId::zqn("noise.readout").expect("valid identifier");

        assert_eq!(id.as_str(), "zqn.noise.readout");
        assert!(id.is_zqn());
        assert_eq!(id.namespace(), "zqn");
    }

    #[test]
    fn zqn_identifier_preserves_existing_namespace() {
        let id = CapabilityId::zqn("zqn.noise.readout").expect("valid identifier");

        assert_eq!(id.as_str(), "zqn.noise.readout");
    }

    #[test]
    fn canonical_qubit_types_are_used_for_scope() {
        let logical = CapabilityScope::LogicalQubit(QubitId::new(7));
        let physical = CapabilityScope::PhysicalQubit(PhysicalQubitId::new(11));

        assert_eq!(
            logical.logical_qubit(),
            Some(QubitId::new(7))
        );

        assert_eq!(
            physical.physical_qubit(),
            Some(PhysicalQubitId::new(11))
        );
    }

    #[test]
    fn resource_scope_rejects_empty_identifier() {
        assert_eq!(
            CapabilityScope::resource(""),
            Err(ScopeError::EmptyResourceIdentifier)
        );
    }

    #[test]
    fn native_requirement_accepts_native_support() {
        let capability = Capability::native(
            CapabilityId::noise_readout(),
            CapabilityScope::Global,
        );

        let requirement = CapabilityRequirement::native(
            CapabilityId::noise_readout(),
            CapabilityScope::Global,
        );

        assert!(requirement.support().accepts(capability.support()));
    }

    #[test]
    fn exact_requirement_accepts_emulation() {
        let capability = Capability::emulated(
            CapabilityId::channel_choi(),
            CapabilityScope::Global,
        );

        let requirement = CapabilityRequirement::exact(
            CapabilityId::channel_choi(),
            CapabilityScope::Global,
        );

        assert!(requirement.support().accepts(capability.support()));
    }

    #[test]
    fn exact_requirement_rejects_approximation() {
        let capability = Capability::approximate(
            CapabilityId::channel_choi(),
            CapabilityScope::Global,
        );

        let requirement = CapabilityRequirement::exact(
            CapabilityId::channel_choi(),
            CapabilityScope::Global,
        );

        assert!(!requirement.support().accepts(capability.support()));
    }

    #[test]
    fn approximate_requirement_explicitly_accepts_approximation() {
        let capability = Capability::approximate(
            CapabilityId::channel_choi(),
            CapabilityScope::Global,
        );

        let requirement = CapabilityRequirement::approximate(
            CapabilityId::channel_choi(),
            CapabilityScope::Global,
        );

        assert!(requirement.support().accepts(capability.support()));
    }

    #[test]
    fn capability_set_deduplicates_exact_entries() {
        let capability = Capability::native(
            CapabilityId::noise_readout(),
            CapabilityScope::Global,
        );

        let mut set = CapabilitySet::new();

        assert!(set.insert(capability.clone()));
        assert!(!set.insert(capability));

        assert_eq!(set.len(), 1);
    }

    #[test]
    fn capability_set_matches_requirement() {
        let capability = Capability::native(
            CapabilityId::noise_readout(),
            CapabilityScope::Global,
        );

        let mut set = CapabilitySet::new();
        set.insert(capability.clone());

        let requirement = CapabilityRequirement::exact(
            CapabilityId::noise_readout(),
            CapabilityScope::Global,
        );

        assert_eq!(
            set.match_requirement(&requirement),
            Some(&capability)
        );
    }

    #[test]
    fn capability_set_respects_scope() {
        let capability = Capability::native(
            CapabilityId::noise_readout(),
            CapabilityScope::PhysicalQubit(PhysicalQubitId::new(0)),
        );

        let mut set = CapabilitySet::new();
        set.insert(capability);

        let wrong_scope = CapabilityRequirement::exact(
            CapabilityId::noise_readout(),
            CapabilityScope::PhysicalQubit(PhysicalQubitId::new(1)),
        );

        assert!(!set.satisfies(&wrong_scope));
    }

    #[test]
    fn missing_requirements_are_deterministic() {
        let mut set = CapabilitySet::new();

        set.insert(Capability::native(
            CapabilityId::noise_readout(),
            CapabilityScope::Global,
        ));

        let requirements = vec![
            CapabilityRequirement::exact(
                CapabilityId::noise_readout(),
                CapabilityScope::Global,
            ),
            CapabilityRequirement::exact(
                CapabilityId::noise_crosstalk(),
                CapabilityScope::Global,
            ),
        ];

        let missing = set.missing(&requirements);

        assert_eq!(missing.len(), 1);
        assert_eq!(
            missing[0].id(),
            &CapabilityId::noise_crosstalk()
        );
    }

    #[test]
    fn approximate_match_is_distinguished() {
        let mut set = CapabilitySet::new();

        set.insert(Capability::approximate(
            CapabilityId::channel_lindblad(),
            CapabilityScope::Global,
        ));

        let requirement = CapabilityRequirement::approximate(
            CapabilityId::channel_lindblad(),
            CapabilityScope::Global,
        );

        let result = evaluate_requirement(&set, &requirement);

        assert!(result.is_approximate());
        assert!(!result.is_missing());
        assert!(!result.is_exact());
    }

    #[test]
    fn generic_profile_is_small_and_provider_neutral() {
        let profile = generic_noise_profile();

        assert!(profile.satisfies(&CapabilityRequirement::exact(
            CapabilityId::quantum_channels(),
            CapabilityScope::Global,
        )));

        assert!(profile.satisfies(&CapabilityRequirement::exact(
            CapabilityId::reproducible_sampling(),
            CapabilityScope::Global,
        )));

        assert!(profile.satisfies(&CapabilityRequirement::exact(
            CapabilityId::provenance(),
            CapabilityScope::Global,
        )));
    }

    #[test]
    fn capability_iteration_is_deterministic() {
        let mut first = CapabilitySet::new();
        let mut second = CapabilitySet::new();

        let entries = [
            Capability::native(
                CapabilityId::noise_readout(),
                CapabilityScope::Global,
            ),
            Capability::native(
                CapabilityId::noise_gate(),
                CapabilityScope::Global,
            ),
            Capability::native(
                CapabilityId::noise_idle(),
                CapabilityScope::Global,
            ),
        ];

        for entry in entries.iter().cloned() {
            first.insert(entry);
        }

        for entry in entries.iter().rev().cloned() {
            second.insert(entry);
        }

        assert_eq!(first.to_vec(), second.to_vec());
    }

    #[test]
    fn arbitrary_future_capabilities_do_not_require_core_changes() {
        let id = CapabilityId::new(
            "future.quantum.technology.non_markovian_environment",
        )
        .expect("future capability identifier should be valid");

        let capability = Capability::native(
            id.clone(),
            CapabilityScope::Global,
        );

        let requirement = CapabilityRequirement::exact(
            id,
            CapabilityScope::Global,
        );

        let mut set = CapabilitySet::new();
        set.insert(capability);

        assert!(set.satisfies(&requirement));
    }

    #[test]
    fn support_level_semantics_are_explicit() {
        assert!(SupportRequirement::Native.accepts(SupportLevel::Native));
        assert!(!SupportRequirement::Native.accepts(SupportLevel::Emulated));

        assert!(SupportRequirement::Exact.accepts(SupportLevel::Native));
        assert!(SupportRequirement::Exact.accepts(SupportLevel::Emulated));
        assert!(!SupportRequirement::Exact.accepts(SupportLevel::Approximate));

        assert!(SupportRequirement::Approximate.accepts(SupportLevel::Approximate));
        assert!(SupportRequirement::Approximate.accepts(SupportLevel::Emulated));

        assert!(SupportRequirement::Any.accepts(SupportLevel::Native));
        assert!(SupportRequirement::Any.accepts(SupportLevel::Emulated));
        assert!(SupportRequirement::Any.accepts(SupportLevel::Approximate));
        assert!(!SupportRequirement::Any.accepts(SupportLevel::Unsupported));
    }
}