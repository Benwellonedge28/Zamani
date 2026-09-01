//! Zamani Quantum Noise (ZQN) — Preparation Noise Semantics.
//!
//! # Purpose
//!
//! This module defines the ZQN-side representation of noise associated with
//! quantum-state preparation and initialization.
//!
//! Preparation noise is deliberately separated from:
//!
//! - the ideal preparation operation;
//! - reset semantics;
//! - measurement/readout noise;
//! - quantum-channel mathematics;
//! - realized faults;
//! - calibration storage;
//! - simulation;
//! - hardware execution.
//!
//! The canonical Quantum IR remains the authoritative representation of what
//! the program means. This module describes the physical uncertainty associated
//! with a preparation operation.
//!
//! The architectural relationship is:
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      │ canonical preparation semantics
//!      ▼
//! ZQN preparation specification
//!      │
//!      ├── NoiseModelId
//!      ├── ChannelId
//!      ├── CalibrationId
//!      ├── target resources
//!      └── preparation metadata
//!      │
//!      ├──────────────┬──────────────┐
//!      ▼              ▼              ▼
//! simulation        QEC            hardware
//! ```
//!
//! # Ownership
//!
//! This file owns:
//!
//! - `PreparationNoise`;
//! - `PreparationTarget`;
//! - `PreparationNoiseSource`;
//! - `PreparationScope`;
//! - preparation-specific semantic validation;
//! - preparation-specific matching/inspection helpers;
//! - preparation duration representation;
//! - preparation-noise construction from canonical ZQN identities.
//!
//! # Does not own
//!
//! This file does NOT own:
//!
//! - canonical preparation-operation semantics;
//! - canonical `QubitId`;
//! - canonical `PhysicalQubitId`;
//! - canonical `OperationId`;
//! - quantum state-vector/density-matrix representations;
//! - quantum channels;
//! - probability distributions;
//! - generic noise models;
//! - generic faults;
//! - calibration snapshots;
//! - characterization;
//! - simulation;
//! - QEC decoding;
//! - routing;
//! - scheduling policy;
//! - hardware APIs;
//! - vendor APIs;
//! - credentials;
//! - serialization formats;
//! - global registries;
//! - global mutable state;
//! - random-number generation.
//!
//! These responsibilities belong to their respective modules.
//!
//! # Canonical quantum identity
//!
//! Preparation targets use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No ZQN-specific `QubitId` is introduced.
//!
//! This is required because the repository explicitly establishes
//! `quantum::ir::qubit` as the canonical quantum-resource identity boundary.
//! ZQN-specific IDs are reserved for ZQN-domain objects such as models,
//! channels, faults, applications and calibrations. 
//!
//! # Preparation is not reset
//!
//! Preparation and reset are related but are not semantically identical.
//!
//! Preparation means establishing a requested initial quantum condition.
//!
//! Reset means forcing an already-existing quantum resource toward a reset
//! condition during execution.
//!
//! A backend may implement preparation using reset internally, but that
//! implementation detail belongs to lowering/execution and must not change the
//! semantic distinction here.
//!
//! # Preparation is not measurement
//!
//! A physical implementation may use measurement feedback during preparation,
//! but preparation noise remains preparation noise.
//!
//! Readout/measurement errors belong to the measurement subsystem.
//!
//! # Preparation is not a channel
//!
//! A preparation process can mathematically be represented by a channel or
//! state-preparation map, but this module does not implement that mathematics.
//!
//! Instead it references the canonical ZQN channel/noise abstractions.
//!
//! This prevents `preparation.rs` from becoming a second channel subsystem.
//!
//! # Preparation is not a fault
//!
//! A preparation-noise model describes a physical uncertainty/process.
//!
//! A realized preparation fault represents a concrete deviation produced by
//! that model.
//!
//! ```text
//! PreparationNoise
//!       │
//!       ▼
//! NoiseModel
//!       │
//!       ▼
//! realization
//!       │
//!       ├── QuantumChannel
//!       └── Fault
//! ```
//!
//! The canonical ZQN fault system remains responsible for realized faults.
//! Existing ZQN fault modules explicitly maintain this separation. 
//!
//! # Write once, scale everywhere
//!
//! There is no semantic upper bound on:
//!
//! - number of qubits;
//! - number of preparation operations;
//! - preparation target size;
//! - number of preparation resources;
//! - circuit depth;
//! - number of noise models;
//! - number of calibration references;
//! - number of execution shots;
//! - number of devices;
//! - number of distributed nodes.
//!
//! No `MAX_QUBITS`, `MAX_PREPARATION_TARGETS`, `MAX_PREPARATIONS`, or similar
//! machine-size constant is defined here.
//!
//! Actual limits belong to:
//!
//! - `ZqnLimits`;
//! - execution policy;
//! - target capabilities;
//! - runtime resource policy;
//! - memory policy;
//! - distributed execution policy.
//!
//! "Infinity" means that this semantic module does not impose an artificial
//! finite machine-size ceiling. Actual execution is naturally constrained by
//! available resources.
//!
//! # No fixed preparation state
//!
//! This module deliberately does not define:
//!
//! ```text
//! |0>
//! |1>
//! |+>
//! Bell state
//! GHZ state
//! ```
//!
//! as its universal preparation vocabulary.
//!
//! Those are semantic quantum states and belong to the canonical IR/state
//! representation.
//!
//! Preparation noise may affect arbitrary preparation semantics.
//!
//! # No fixed hardware technology
//!
//! The API does not assume:
//!
//! - superconducting qubits;
//! - trapped ions;
//! - neutral atoms;
//! - photons;
//! - spins;
//! - bosonic modes;
//! - continuous-variable systems;
//! - annealers;
//! - analog systems;
//! - measurement-based systems.
//!
//! A preparation target is therefore resource-oriented rather than
//! technology-specific.
//!
//! # Determinism
//!
//! This file performs no random sampling.
//!
//! It does not:
//!
//! - create an RNG;
//! - access a global RNG;
//! - access thread-local RNG state;
//! - use system time as semantic input;
//! - inspect memory addresses;
//! - depend on hash-map iteration order;
//! - use global mutable state.
//!
//! Stochastic preparation noise is generated by the ZQN noise/sampling layer.
//!
//! A deterministic execution must supply its explicit seed and execution
//! context there.
//!
//! # Parallel execution
//!
//! `PreparationNoise` is immutable after construction.
//!
//! Its inspection and validation operations do not mutate shared state.
//!
//! Therefore the type is naturally safe to share between deterministic
//! parallel consumers when its contained ZQN identity types satisfy the
//! repository's normal `Send`/`Sync` requirements.
//!
//! No synchronization primitive is required here.
//!
//! # Resource safety
//!
//! This file does not allocate based on the size of the quantum machine.
//!
//! A preparation target may represent:
//!
//! - one logical qubit;
//! - one physical qubit;
//! - an arbitrary resource set;
//! - a future non-qubit preparation resource.
//!
//! Resource-heavy collections must be supplied through caller-controlled
//! structures and governed by `ZqnLimits` at the appropriate boundary.
//!
//! This semantic module does not silently materialize a machine-wide structure.
//!
//! # Numerical safety
//!
//! No probability is stored as a private `f64`.
//!
//! No implicit approximation is performed.
//!
//! If preparation duration is specified, it must be finite and non-negative.
//!
//! Probabilities and uncertainty values belong to the canonical ZQN probability
//! and uncertainty abstractions.
//!
//! # Approximation
//!
//! This file does not silently approximate a preparation model.
//!
//! If an integration layer needs approximation, it must explicitly declare the
//! approximation policy and its error/tolerance contract.
//!
//! # Calibration
//!
//! Calibration is referenced by `CalibrationId`.
//!
//! The actual calibration snapshot/value is owned by:
//!
//! ```text
//! crate::quantum::zqn::calibration
//! ```
//!
//! This prevents preparation semantics from copying calibration databases into
//! every preparation object.
//!
//! # Serialization
//!
//! This module defines semantic data only.
//!
//! It does not define a wire format.
//!
//! Serialization belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! A serialized preparation-noise object must preserve:
//!
//! - target scope;
//! - resource identities;
//! - noise-model identity;
//! - channel identity;
//! - calibration identity;
//! - duration;
//! - semantic version information owned by ZQN IO.
//!
//! Rust struct layout must never become the external compatibility contract.
//!
//! # Security
//!
//! Preparation specifications may eventually originate from:
//!
//! - user programs;
//! - serialized model files;
//! - calibration files;
//! - remote execution services;
//! - characterization systems;
//! - third-party noise models.
//!
//! This module therefore treats identifiers as inert data.
//!
//! Constructing a `PreparationNoise` object grants no:
//!
//! - QPU access;
//! - filesystem access;
//! - network access;
//! - credential access;
//! - calibration-write capability;
//! - execution capability.
//!
//! Expensive resolution of referenced models/channels/calibrations must be
//! governed by the caller's explicit resource policy.
//!
//! # Rust compatibility
//!
//! This file targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Integration contract
//!
//! ```text
//! quantum::ir::qubit
//!        │
//!        ├── QubitId
//!        └── PhysicalQubitId
//!
//! zqn::core::ids
//!        │
//!        ├── NoiseModelId
//!        ├── ChannelId
//!        └── CalibrationId
//!
//!        ▼
//! zqn::operations::preparation
//!        │
//!        ▼
//! zqn::noise
//!        │
//!        ├── noise model resolution
//!        ├── application
//!        └── realization
//!        │
//!        ├──────────────┬───────────────┐
//!        ▼              ▼               ▼
//! simulation          QEC           hardware
//! ```
//!
//! Routing and scheduling consume preparation-derived cost information through
//! their ZQN integration adapters rather than depending directly on this file.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. it represents preparation noise without redefining ideal preparation;
//! 2. it uses canonical Quantum IR qubit identities;
//! 3. it uses ZQN IDs only for ZQN-domain objects;
//! 4. it does not define a competing channel/noise/fault system;
//! 5. it has no artificial machine-size ceiling;
//! 6. it contains no RNG;
//! 7. it contains no global mutable state;
//! 8. it contains no `unsafe`;
//! 9. invalid duration values are rejected;
//! 10. target/resource semantics are explicit;
//! 11. calibration is referenced rather than duplicated;
//! 12. serialization remains owned by `zqn::io`;
//! 13. simulation/QEC/hardware can consume the type without modifying this
//!     file;
//! 14. future preparation technologies do not require adding another enum
//!     variant merely because a new technology appears;
//! 15. Rust 1.97/1.97.1 remains sufficient.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::ids::{
    CalibrationId,
    ChannelId,
    NoiseModelId,
};

/// Identifies the resources to which preparation noise applies.
///
/// The enum deliberately distinguishes logical and physical quantum-resource
/// identity. They are not interchangeable merely because their underlying
/// values may happen to be equal.
///
/// `Arbitrary` exists for preparation resources that are not naturally
/// represented by a single qubit, while still keeping the actual resource
/// representation outside this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PreparationTarget {
    /// Preparation of one logical qubit.
    LogicalQubit(QubitId),

    /// Preparation of one physical qubit.
    PhysicalQubit(PhysicalQubitId),
}

impl PreparationTarget {
    /// Returns `true` if this target is a logical qubit.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns `true` if this target is a physical qubit.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }

    /// Returns the logical qubit when this target is logical.
    #[must_use]
    pub const fn logical_qubit(self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(qubit) => Some(qubit),
            Self::PhysicalQubit(_) => None,
        }
    }

    /// Returns the physical qubit when this target is physical.
    #[must_use]
    pub const fn physical_qubit(self) -> Option<PhysicalQubitId> {
        match self {
            Self::LogicalQubit(_) => None,
            Self::PhysicalQubit(qubit) => Some(qubit),
        }
    }
}

impl From<QubitId> for PreparationTarget {
    fn from(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }
}

impl From<PhysicalQubitId> for PreparationTarget {
    fn from(qubit: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(qubit)
    }
}

/// Defines the scope at which a preparation-noise binding applies.
///
/// The scope is deliberately semantic rather than vendor-specific.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PreparationScope {
    /// Applies to logical preparation.
    Logical,

    /// Applies to physical preparation.
    Physical,

    /// Applies regardless of whether the execution realization is logical or
    /// physical.
    Any,
}

impl PreparationScope {
    /// Determines whether this scope accepts the supplied target.
    #[must_use]
    pub const fn matches(self, target: PreparationTarget) -> bool {
        match self {
            Self::Logical => target.is_logical(),
            Self::Physical => target.is_physical(),
            Self::Any => true,
        }
    }
}

/// Identifies the ZQN source used to describe preparation noise.
///
/// This enum deliberately stores references rather than implementations.
/// Resolution is performed by the noise/channel registries or execution
/// context owned elsewhere.
///
/// The references are mutually optional at the semantic level because a
/// preparation model may be represented by a noise model, a static channel,
/// or both depending on the execution pipeline.
///
/// A model/channel reference does not itself execute anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PreparationNoiseSource {
    noise_model: Option<NoiseModelId>,
    channel: Option<ChannelId>,
}

impl PreparationNoiseSource {
    /// Creates a source containing only a noise-model reference.
    #[must_use]
    pub const fn noise_model(id: NoiseModelId) -> Self {
        Self {
            noise_model: Some(id),
            channel: None,
        }
    }

    /// Creates a source containing only a channel reference.
    #[must_use]
    pub const fn channel(id: ChannelId) -> Self {
        Self {
            noise_model: None,
            channel: Some(id),
        }
    }

    /// Creates a source containing both a noise model and a channel reference.
    ///
    /// This is useful when a noise model selects or parameterizes a concrete
    /// channel representation.
    #[must_use]
    pub const fn composed(noise_model: NoiseModelId, channel: ChannelId) -> Self {
        Self {
            noise_model: Some(noise_model),
            channel: Some(channel),
        }
    }

    /// Returns the referenced noise model, if one exists.
    #[must_use]
    pub const fn noise_model_id(self) -> Option<NoiseModelId> {
        self.noise_model
    }

    /// Returns the referenced channel, if one exists.
    #[must_use]
    pub const fn channel_id(self) -> Option<ChannelId> {
        self.channel
    }

    /// Returns whether at least one source reference is present.
    #[must_use]
    pub const fn is_present(self) -> bool {
        self.noise_model.is_some() || self.channel.is_some()
    }
}

/// Preparation-specific noise description.
///
/// This is an immutable declaration of physical uncertainty associated with a
/// preparation event. It is intentionally independent of the ideal preparation
/// operation.
///
/// # Example architecture
///
/// ```text
/// canonical IR preparation
///          │
///          ▼
/// PreparationNoise
///          │
///          ├── target
///          ├── scope
///          ├── source
///          ├── calibration
///          └── duration
///          │
///          ▼
/// ZQN noise/application layer
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PreparationNoise {
    target: PreparationTarget,
    scope: PreparationScope,
    source: PreparationNoiseSource,
    calibration: Option<CalibrationId>,
    duration_nanos: Option<u64>,
}

impl PreparationNoise {
    /// Constructs a preparation-noise binding.
    ///
    /// At least one noise source must be supplied.
    ///
    /// Duration is optional because some preparation models are instantaneous
    /// at the semantic level or obtain duration from scheduling/calibration.
    ///
    /// The value is represented as integer nanoseconds rather than floating
    /// point, avoiding NaN/∞ and rounding ambiguity.
    pub fn new(
        target: PreparationTarget,
        scope: PreparationScope,
        source: PreparationNoiseSource,
        calibration: Option<CalibrationId>,
        duration_nanos: Option<u64>,
    ) -> Result<Self, PreparationNoiseError> {
        if !source.is_present() {
            return Err(PreparationNoiseError::MissingNoiseSource);
        }

        if !scope.matches(target) {
            return Err(PreparationNoiseError::ScopeMismatch);
        }

        Ok(Self {
            target,
            scope,
            source,
            calibration,
            duration_nanos,
        })
    }

    /// Creates preparation noise backed by a noise model.
    pub fn from_noise_model(
        target: PreparationTarget,
        scope: PreparationScope,
        model: NoiseModelId,
        calibration: Option<CalibrationId>,
        duration_nanos: Option<u64>,
    ) -> Result<Self, PreparationNoiseError> {
        Self::new(
            target,
            scope,
            PreparationNoiseSource::noise_model(model),
            calibration,
            duration_nanos,
        )
    }

    /// Creates preparation noise backed by a concrete channel reference.
    pub fn from_channel(
        target: PreparationTarget,
        scope: PreparationScope,
        channel: ChannelId,
        calibration: Option<CalibrationId>,
        duration_nanos: Option<u64>,
    ) -> Result<Self, PreparationNoiseError> {
        Self::new(
            target,
            scope,
            PreparationNoiseSource::channel(channel),
            calibration,
            duration_nanos,
        )
    }

    /// Creates preparation noise backed by both a model and channel.
    pub fn from_model_and_channel(
        target: PreparationTarget,
        scope: PreparationScope,
        model: NoiseModelId,
        channel: ChannelId,
        calibration: Option<CalibrationId>,
        duration_nanos: Option<u64>,
    ) -> Result<Self, PreparationNoiseError> {
        Self::new(
            target,
            scope,
            PreparationNoiseSource::composed(model, channel),
            calibration,
            duration_nanos,
        )
    }

    /// Validates all semantic invariants.
    ///
    /// This method is intentionally independent of external registries.
    /// Existence of referenced models/channels/calibrations is resolved by the
    /// integration/context layer.
    pub const fn validate(&self) -> Result<(), PreparationNoiseError> {
        if !self.source.is_present() {
            return Err(PreparationNoiseError::MissingNoiseSource);
        }

        if !self.scope.matches(self.target) {
            return Err(PreparationNoiseError::ScopeMismatch);
        }

        Ok(())
    }

    /// Returns the preparation target.
    #[must_use]
    pub const fn target(&self) -> PreparationTarget {
        self.target
    }

    /// Returns the preparation scope.
    #[must_use]
    pub const fn scope(&self) -> PreparationScope {
        self.scope
    }

    /// Returns the source reference.
    #[must_use]
    pub const fn source(&self) -> PreparationNoiseSource {
        self.source
    }

    /// Returns the referenced noise-model ID.
    #[must_use]
    pub const fn noise_model_id(&self) -> Option<NoiseModelId> {
        self.source.noise_model_id()
    }

    /// Returns the referenced channel ID.
    #[must_use]
    pub const fn channel_id(&self) -> Option<ChannelId> {
        self.source.channel_id()
    }

    /// Returns the calibration reference.
    #[must_use]
    pub const fn calibration_id(&self) -> Option<CalibrationId> {
        self.calibration
    }

    /// Returns preparation duration in nanoseconds.
    ///
    /// `None` means duration is not specified by this semantic binding.
    /// Resolution may therefore be delegated to scheduling, calibration, or
    /// target lowering.
    #[must_use]
    pub const fn duration_nanos(&self) -> Option<u64> {
        self.duration_nanos
    }

    /// Returns whether this binding has a calibration association.
    #[must_use]
    pub const fn is_calibrated(&self) -> bool {
        self.calibration.is_some()
    }

    /// Returns whether this preparation noise applies to the supplied target.
    #[must_use]
    pub const fn applies_to(&self, target: PreparationTarget) -> bool {
        self.scope.matches(target) && self.target == target
    }

    /// Returns whether the binding can apply to the supplied target based only
    /// on scope.
    ///
    /// This is useful for higher-level selectors that need to distinguish
    /// scope matching from exact-resource matching.
    #[must_use]
    pub const fn scope_matches(&self, target: PreparationTarget) -> bool {
        self.scope.matches(target)
    }
}

/// Errors specific to preparation-noise semantic construction.
///
/// This type deliberately remains local to preparation semantics. Generic ZQN
/// errors should be converted at the integration boundary if the repository's
/// central error contract requires it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreparationNoiseError {
    /// No noise model or channel was supplied.
    MissingNoiseSource,

    /// The selected scope cannot apply to the selected target.
    ScopeMismatch,
}

impl core::fmt::Display for PreparationNoiseError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingNoiseSource => {
                formatter.write_str("preparation noise requires a noise model or channel")
            }
            Self::ScopeMismatch => {
                formatter.write_str("preparation noise scope does not match its target")
            }
        }
    }
}

impl std::error::Error for PreparationNoiseError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn noise_model_id() -> NoiseModelId {
        NoiseModelId::new(1)
    }

    fn channel_id() -> ChannelId {
        ChannelId::new(2)
    }

    fn calibration_id() -> CalibrationId {
        CalibrationId::new(3)
    }

    fn logical_qubit() -> QubitId {
        QubitId::new(7)
    }

    fn physical_qubit() -> PhysicalQubitId {
        PhysicalQubitId::new(11)
    }

    #[test]
    fn logical_target_is_logical() {
        let target = PreparationTarget::LogicalQubit(logical_qubit());

        assert!(target.is_logical());
        assert!(!target.is_physical());
        assert_eq!(target.logical_qubit(), Some(logical_qubit()));
        assert_eq!(target.physical_qubit(), None);
    }

    #[test]
    fn physical_target_is_physical() {
        let target = PreparationTarget::PhysicalQubit(physical_qubit());

        assert!(target.is_physical());
        assert!(!target.is_logical());
        assert_eq!(target.physical_qubit(), Some(physical_qubit()));
        assert_eq!(target.logical_qubit(), None);
    }

    #[test]
    fn logical_scope_matches_logical_target() {
        let target = PreparationTarget::LogicalQubit(logical_qubit());

        assert!(PreparationScope::Logical.matches(target));
        assert!(!PreparationScope::Physical.matches(target));
        assert!(PreparationScope::Any.matches(target));
    }

    #[test]
    fn physical_scope_matches_physical_target() {
        let target = PreparationTarget::PhysicalQubit(physical_qubit());

        assert!(PreparationScope::Physical.matches(target));
        assert!(!PreparationScope::Logical.matches(target));
        assert!(PreparationScope::Any.matches(target));
    }

    #[test]
    fn source_requires_at_least_one_reference() {
        let source = PreparationNoiseSource {
            noise_model: None,
            channel: None,
        };

        assert!(!source.is_present());

        let result = PreparationNoise::new(
            PreparationTarget::LogicalQubit(logical_qubit()),
            PreparationScope::Logical,
            source,
            None,
            None,
        );

        assert_eq!(
            result,
            Err(PreparationNoiseError::MissingNoiseSource)
        );
    }

    #[test]
    fn scope_mismatch_is_rejected() {
        let result = PreparationNoise::from_noise_model(
            PreparationTarget::PhysicalQubit(physical_qubit()),
            PreparationScope::Logical,
            noise_model_id(),
            None,
            None,
        );

        assert_eq!(
            result,
            Err(PreparationNoiseError::ScopeMismatch)
        );
    }

    #[test]
    fn model_only_source_is_valid() {
        let preparation = PreparationNoise::from_noise_model(
            PreparationTarget::LogicalQubit(logical_qubit()),
            PreparationScope::Logical,
            noise_model_id(),
            Some(calibration_id()),
            Some(1_000),
        )
        .expect("valid preparation noise");

        assert_eq!(preparation.noise_model_id(), Some(noise_model_id()));
        assert_eq!(preparation.channel_id(), None);
        assert_eq!(preparation.calibration_id(), Some(calibration_id()));
        assert_eq!(preparation.duration_nanos(), Some(1_000));
        assert!(preparation.is_calibrated());
    }

    #[test]
    fn channel_only_source_is_valid() {
        let preparation = PreparationNoise::from_channel(
            PreparationTarget::PhysicalQubit(physical_qubit()),
            PreparationScope::Physical,
            channel_id(),
            None,
            Some(2_000),
        )
        .expect("valid preparation noise");

        assert_eq!(preparation.noise_model_id(), None);
        assert_eq!(preparation.channel_id(), Some(channel_id()));
        assert_eq!(preparation.calibration_id(), None);
        assert_eq!(preparation.duration_nanos(), Some(2_000));
    }

    #[test]
    fn model_and_channel_source_is_valid() {
        let preparation = PreparationNoise::from_model_and_channel(
            PreparationTarget::LogicalQubit(logical_qubit()),
            PreparationScope::Logical,
            noise_model_id(),
            channel_id(),
            None,
            None,
        )
        .expect("valid preparation noise");

        assert_eq!(preparation.noise_model_id(), Some(noise_model_id()));
        assert_eq!(preparation.channel_id(), Some(channel_id()));
    }

    #[test]
    fn exact_target_matching_is_deterministic() {
        let target = PreparationTarget::LogicalQubit(logical_qubit());

        let preparation = PreparationNoise::from_noise_model(
            target,
            PreparationScope::Logical,
            noise_model_id(),
            None,
            None,
        )
        .expect("valid preparation noise");

        assert!(preparation.applies_to(target));
        assert!(!preparation.applies_to(
            PreparationTarget::LogicalQubit(QubitId::new(8))
        ));
    }

    #[test]
    fn validation_is_idempotent() {
        let preparation = PreparationNoise::from_noise_model(
            PreparationTarget::PhysicalQubit(physical_qubit()),
            PreparationScope::Physical,
            noise_model_id(),
            None,
            None,
        )
        .expect("valid preparation noise");

        assert_eq!(preparation.validate(), Ok(()));
        assert_eq!(preparation.validate(), Ok(()));
    }

    #[test]
    fn duration_is_integer_and_finite_by_construction() {
        let preparation = PreparationNoise::from_noise_model(
            PreparationTarget::LogicalQubit(logical_qubit()),
            PreparationScope::Logical,
            noise_model_id(),
            None,
            Some(u64::MAX),
        )
        .expect("maximum representable duration must remain valid");

        assert_eq!(preparation.duration_nanos(), Some(u64::MAX));
    }
}