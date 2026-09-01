//! Zamani Quantum Noise (ZQN) — Generalized Quantum Channel
//!
//! This module provides the representation-independent concrete container for
//! a generalized quantum channel whose mathematical representation is supplied
//! by another ZQN channel representation or by a future quantum modality.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - `GeneralizedChannel`;
//! - generalized channel semantic metadata;
//! - generalized channel construction and validation;
//! - explicit representation/physicality/accuracy contracts;
//! - required capability declarations;
//! - resource requirement declarations;
//! - canonical `QuantumChannel` integration;
//! - generalized channel compatibility checks;
//! - sequential/tensor composition admission checks;
//! - deterministic metadata access;
//! - immutable channel configuration.
//!
//! This file does NOT own:
//!
//! - Kraus operators;
//! - Choi matrices;
//! - process matrices;
//! - Pauli-transfer matrices;
//! - Lindblad integration;
//! - stochastic distributions;
//! - density matrices;
//! - state-vector simulation;
//! - numerical linear algebra;
//! - random-number generation;
//! - calibration;
//! - characterization;
//! - routing;
//! - scheduling;
//! - QEC;
//! - hardware APIs;
//! - vendor-specific behavior;
//! - serialization schemas;
//! - backend execution.
//!
//! Concrete mathematical representations belong to:
//!
//! ```text
//! channel/kraus.rs
//! channel/choi.rs
//! channel/process_matrix.rs
//! channel/pauli.rs
//! channel/stochastic.rs
//! channel/lindblad.rs
//! ```
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!                    canonical semantic operation
//!                              │
//!                              ▼
//!                    ┌────────────────────┐
//!                    │       ZQN          │
//!                    │ QuantumChannel     │
//!                    └─────────┬──────────┘
//!                              │
//!                  ┌───────────┼────────────┐
//!                  │           │            │
//!                  ▼           ▼            ▼
//!                Kraus        Choi       Generalized
//!                  │           │            │
//!                  └───────────┼────────────┘
//!                              ▼
//!                       downstream consumers
//!                  simulation / QEC / routing
//!                  scheduling / hardware / analysis
//! ```
//!
//! `GeneralizedChannel` is therefore a semantic adapter/container, not a
//! replacement for the specialized mathematical representations.
//!
//! # Why this type exists
//!
//! A production quantum compiler cannot assume that every present or future
//! channel can be represented by one of today's fixed representations.
//!
//! `GeneralizedChannel` provides a stable semantic boundary for:
//!
//! - future channel representations;
//! - emerging quantum modalities;
//! - target-specific mathematical representations before a dedicated ZQN type
//!   exists;
//! - symbolic channels;
//! - externally characterized channels;
//! - composite channels whose concrete representation is intentionally owned by
//!   another layer;
//! - channels that must remain representation-opaque to routing/scheduling;
//! - channels crossing subsystem boundaries;
//! - channels whose concrete numerical realization is selected later.
//!
//! It deliberately does NOT pretend that opaque metadata is itself a numerical
//! realization.
//!
//! # Write once, scale everywhere
//!
//! This module contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_CHANNELS
//! MAX_ARITY
//! MAX_DIMENSION
//! MAX_OPERATORS
//! MAX_MATRIX_SIZE
//! MAX_MEMORY
//! ```
//!
//! Channel size is determined by `ChannelSupport` and its subsystem dimensions.
//!
//! Actual resource feasibility belongs to:
//!
//! ```text
//! zqn::core::limits
//! runtime/resource policy
//! target capabilities
//! memory subsystem
//! execution backend
//! ```
//!
//! Therefore this module introduces no artificial semantic upper bound on:
//!
//! - number of subsystems;
//! - channel arity;
//! - machine size;
//! - circuit size;
//! - operation count;
//! - topology;
//! - target size;
//! - quantum technology.
//!
//! "Infinity" means no artificial finite machine-size ceiling is encoded here.
//! Every actual execution remains bounded by available resources.
//!
//! # Canonical quantum identity
//!
//! Whenever the generalized channel refers to qubits, it uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! No second `QubitId` is defined.
//!
//! The lower-level channel module already establishes that canonical identity
//! boundary. This file therefore uses `QubitId` through
//! `ChannelSubsystemId::Qubit` and `ChannelSupport`.
//!
//! # Mathematical contract
//!
//! `GeneralizedChannel` does not claim a channel is physical merely because it
//! has valid metadata.
//!
//! Physical validity is explicitly represented by:
//!
//! ```text
//! ChannelPhysicality::Validated
//! ChannelPhysicality::Unvalidated
//! ChannelPhysicality::NonPhysical
//! ChannelPhysicality::Conditional
//! ```
//!
//! Likewise, exactness is explicitly represented by `ChannelAccuracy`.
//!
//! No approximation is silently upgraded to exact semantics.
//!
//! No structurally valid generalized channel is silently upgraded to CPTP.
//!
//! # Representation contract
//!
//! The concrete representation is supplied through the canonical
//! `ChannelRepresentation` enum.
//!
//! `GeneralizedChannel` does not reinterpret that representation.
//!
//! For a future representation that has not yet received a dedicated ZQN
//! implementation, `ChannelRepresentation::Custom` may be used, while the
//! actual mathematical implementation remains owned by the future module or
//! integration layer.
//!
//! # Determinism
//!
//! This module is completely deterministic.
//!
//! It owns:
//!
//! - no RNG;
//! - no global state;
//! - no clocks;
//! - no process identifiers;
//! - no thread-local state;
//! - no hash-map iteration;
//! - no lazy global cache.
//!
//! The same generalized channel configuration always exposes the same semantic
//! metadata.
//!
//! # Thread safety
//!
//! `GeneralizedChannel` contains immutable semantic data and is therefore
//! `Send + Sync` when the contained standard library types are.
//!
//! No interior mutable global state is used.
//!
//! # Resource safety
//!
//! Construction validates structural metadata but deliberately does not
//! materialize numerical matrices or tensors.
//!
//! Dimension products are already checked by `ChannelSupport`.
//!
//! Resource requirements remain descriptive. They are not allocation policy.
//!
//! This distinction is critical:
//!
//! ```text
//! mathematical size
//!       ≠
//! allocation permission
//!       ≠
//! hardware capacity
//! ```
//!
//! # Security
//!
//! Generalized channel metadata may originate from untrusted serialized data.
//!
//! This file therefore:
//!
//! - performs structural validation;
//! - rejects empty semantic names;
//! - rejects duplicate capability declarations;
//! - rejects invalid accuracy contracts through the canonical API;
//! - never executes metadata;
//! - never accesses the filesystem;
//! - never accesses the network;
//! - never reads environment variables;
//! - never creates backend connections;
//! - never creates RNG state;
//! - never uses unsafe Rust.
//!
//! Deserialization of untrusted generalized channels must still be subject to
//! the caller's explicit ZQN resource admission policy before large structures
//! are materialized.
//!
//! # Integration contract
//!
//! ```text
//! quantum::ir::qubit::QubitId
//!          │
//!          ▼
//! ChannelSupport
//!          │
//!          ▼
//! GeneralizedChannel
//!          │
//!          ├────────► simulation
//!          ├────────► propagation
//!          ├────────► routing
//!          ├────────► scheduling
//!          ├────────► QEC
//!          ├────────► hardware
//!          ├────────► characterization
//!          └────────► benchmarking
//! ```
//!
//! The downstream subsystem must inspect the generalized channel's declared
//! representation and capabilities before attempting a concrete operation.
//!
//! `GeneralizedChannel` never calls those downstream systems itself.
//!
//! # Relationship with specialized channel implementations
//!
//! Specialized implementations remain authoritative for their mathematics:
//!
//! ```text
//! KrausChannel          -> Kraus mathematics
//! ChoiChannel            -> Choi mathematics
//! ProcessMatrix          -> process-matrix mathematics
//! StochasticChannel      -> stochastic mathematics
//! LindbladChannel        -> generator mathematics
//! GeneralizedChannel     -> representation-independent generalized metadata
//! ```
//!
//! This prevents `generalized.rs` from becoming a second implementation of
//! every other channel representation.
//!
//! # Composition
//!
//! This module provides only admission checks for composition.
//!
//! It does NOT materialize composed matrices.
//!
//! Concrete composition belongs to `channel/composition.rs` or to the
//! representation implementation capable of performing the operation.
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
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. `GeneralizedChannel` implements the canonical `QuantumChannel` trait;
//! 2. no competing channel trait is introduced;
//! 3. canonical `QubitId` is used;
//! 4. arbitrary channel support sizes are accepted;
//! 5. no machine-size constant exists;
//! 6. representation is explicit;
//! 7. physicality is explicit;
//! 8. accuracy is explicit;
//! 9. required capabilities are explicit;
//! 10. resource requirements are explicit;
//! 11. structural validation is performed;
//! 12. composition admission is representation-independent;
//! 13. no numerical matrix is materialized;
//! 14. no hidden approximation exists;
//! 15. no hidden RNG exists;
//! 16. no global state exists;
//! 17. no unsafe Rust exists;
//! 18. Rust 1.97/1.97.1 is sufficient;
//! 19. downstream modules can consume it without modifying this file merely
//!     because another representation is introduced;
//! 20. adding a larger quantum machine does not require modifying this file.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;

use crate::quantum::ir::qubit::QubitId;

use super::channel::{
    validate_sequential_composition,
    validate_tensor_product,
    ChannelAccuracy,
    ChannelCapability,
    ChannelDescriptor,
    ChannelError,
    ChannelId,
    ChannelPhysicality,
    ChannelRepresentation,
    ChannelResourceRequirements,
    ChannelResult,
    ChannelSupport,
    QuantumChannel,
};

// =============================================================================
// Semantic kind
// =============================================================================

/// Stable semantic classification for a generalized channel.
///
/// This is intentionally separate from `ChannelRepresentation`.
///
/// `ChannelRepresentation` answers:
///
/// > How is the mathematical channel represented?
///
/// `GeneralizedChannelKind` answers:
///
/// > What role does this generalized object play semantically?
///
/// The representation may therefore evolve independently of the semantic role.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GeneralizedChannelKind {
    /// A general physical quantum channel whose concrete representation is
    /// supplied elsewhere.
    QuantumProcess,

    /// A channel assembled from multiple semantic sub-processes.
    CompositeProcess,

    /// A channel whose mathematical form is symbolic and is lowered later.
    SymbolicProcess,

    /// A channel inferred from characterization data.
    EmpiricalProcess,

    /// A channel produced by a target/hardware adapter.
    TargetProcess,

    /// A channel representing a future or external quantum modality.
    Extension(String),
}

impl GeneralizedChannelKind {
    /// Returns a stable semantic name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::QuantumProcess => "quantum_process",
            Self::CompositeProcess => "composite_process",
            Self::SymbolicProcess => "symbolic_process",
            Self::EmpiricalProcess => "empirical_process",
            Self::TargetProcess => "target_process",
            Self::Extension(name) => name.as_str(),
        }
    }

    /// Creates an extension kind after validating its name.
    pub fn extension(name: impl Into<String>) -> ChannelResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ChannelError::InvalidIdentity);
        }

        Ok(Self::Extension(name))
    }
}

impl fmt::Display for GeneralizedChannelKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Generalized channel configuration
// =============================================================================

/// Immutable configuration used to construct a `GeneralizedChannel`.
///
/// The configuration deliberately reuses the canonical ZQN channel contracts
/// rather than defining parallel representations or error models.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneralizedChannelConfig {
    /// Stable semantic channel identity.
    pub id: ChannelId,

    /// Optional human-readable name.
    pub name: Option<String>,

    /// Semantic role of the generalized channel.
    pub kind: GeneralizedChannelKind,

    /// Input/output subsystem support.
    pub support: ChannelSupport,

    /// Mathematical representation selected for the channel.
    pub representation: ChannelRepresentation,

    /// Physicality state.
    pub physicality: ChannelPhysicality,

    /// Exactness/approximation contract.
    pub accuracy: ChannelAccuracy,

    /// Capabilities required by consumers/targets.
    pub required_capabilities: Vec<ChannelCapability>,

    /// Descriptive resource requirements.
    pub resources: ChannelResourceRequirements,
}

impl GeneralizedChannelConfig {
    /// Creates a new generalized-channel configuration.
    ///
    /// No numerical representation is allocated.
    pub fn new(
        id: ChannelId,
        kind: GeneralizedChannelKind,
        support: ChannelSupport,
        representation: ChannelRepresentation,
        physicality: ChannelPhysicality,
        accuracy: ChannelAccuracy,
    ) -> ChannelResult<Self> {
        let config = Self {
            id,
            name: None,
            kind,
            support,
            representation,
            physicality,
            accuracy,
            required_capabilities: Vec::new(),
            resources: ChannelResourceRequirements::unknown(),
        };

        config.validate()?;

        Ok(config)
    }

    /// Sets an optional human-readable name.
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Replaces the required capability set.
    ///
    /// Duplicate capabilities are rejected by `validate`.
    #[must_use]
    pub fn with_required_capabilities(
        mut self,
        capabilities: impl IntoIterator<Item = ChannelCapability>,
    ) -> Self {
        self.required_capabilities = capabilities.into_iter().collect();
        self
    }

    /// Sets descriptive resource requirements.
    #[must_use]
    pub const fn with_resources(
        mut self,
        resources: ChannelResourceRequirements,
    ) -> Self {
        self.resources = resources;
        self
    }

    /// Validates the complete configuration.
    pub fn validate(&self) -> ChannelResult<()> {
        self.support.validate()?;

        if let Some(name) = &self.name {
            if name.trim().is_empty() {
                return Err(ChannelError::InvalidIdentity);
            }
        }

        validate_capabilities(&self.required_capabilities)?;

        validate_capability_consistency(
            &self.support,
            self.accuracy,
            &self.required_capabilities,
        )?;

        Ok(())
    }

    /// Returns the canonical channel descriptor represented by this
    /// configuration.
    pub fn descriptor(&self) -> ChannelResult<ChannelDescriptor> {
        ChannelDescriptor::new(
            self.id,
            self.name.clone(),
            self.support.clone(),
            self.representation,
            self.physicality,
            self.accuracy,
            self.resources,
        )
    }
}

// =============================================================================
// Generalized channel
// =============================================================================

/// Production-ready representation-independent generalized quantum channel.
///
/// `GeneralizedChannel` is deliberately metadata-first.
///
/// It does not materialize a matrix, tensor, Kraus family, probability table or
/// state representation. Instead, it provides the canonical semantic contract
/// needed by downstream systems while allowing the concrete mathematical
/// representation to be selected independently.
///
/// This makes it suitable as a stable boundary for future quantum technologies
/// and representations.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneralizedChannel {
    config: GeneralizedChannelConfig,
}

impl GeneralizedChannel {
    /// Constructs and validates a generalized channel.
    pub fn new(config: GeneralizedChannelConfig) -> ChannelResult<Self> {
        config.validate()?;
        config.descriptor()?;

        Ok(Self { config })
    }

    /// Constructs a generalized quantum process with no name or explicit
    /// capability requirements.
    ///
    /// This constructor is intentionally representation-agnostic.
    pub fn quantum_process(
        id: ChannelId,
        support: ChannelSupport,
        representation: ChannelRepresentation,
        physicality: ChannelPhysicality,
        accuracy: ChannelAccuracy,
    ) -> ChannelResult<Self> {
        Self::new(GeneralizedChannelConfig::new(
            id,
            GeneralizedChannelKind::QuantumProcess,
            support,
            representation,
            physicality,
            accuracy,
        )?)
    }

    /// Constructs a generalized process with an explicit semantic kind.
    pub fn with_kind(
        id: ChannelId,
        kind: GeneralizedChannelKind,
        support: ChannelSupport,
        representation: ChannelRepresentation,
        physicality: ChannelPhysicality,
        accuracy: ChannelAccuracy,
    ) -> ChannelResult<Self> {
        Self::new(GeneralizedChannelConfig::new(
            id,
            kind,
            support,
            representation,
            physicality,
            accuracy,
        )?)
    }

    /// Returns the immutable generalized-channel configuration.
    #[must_use]
    pub const fn config(&self) -> &GeneralizedChannelConfig {
        &self.config
    }

    /// Returns the semantic generalized-channel kind.
    #[must_use]
    pub fn kind(&self) -> &GeneralizedChannelKind {
        &self.config.kind
    }

    /// Returns the stable semantic kind string.
    #[must_use]
    pub fn kind_name(&self) -> &str {
        self.config.kind.as_str()
    }

    /// Returns the canonical channel descriptor.
    ///
    /// Construction already validates this descriptor, so this method is
    /// infallible for an existing `GeneralizedChannel`.
    #[must_use]
    pub fn channel_descriptor(&self) -> ChannelDescriptor {
        self.config
            .descriptor()
            .expect("GeneralizedChannel invariant violated: descriptor became invalid")
    }

    /// Returns the required capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &[ChannelCapability] {
        &self.config.required_capabilities
    }

    /// Returns whether the generalized channel requires a capability.
    #[must_use]
    pub fn requires(&self, capability: ChannelCapability) -> bool {
        self.config.required_capabilities.contains(&capability)
    }

    /// Returns the canonical qubit identifiers in the input support.
    #[must_use]
    pub fn input_qubits(&self) -> Vec<QubitId> {
        self.support().input_qubits().collect()
    }

    /// Returns the canonical qubit identifiers in the output support.
    #[must_use]
    pub fn output_qubits(&self) -> Vec<QubitId> {
        self.support().output_qubits().collect()
    }

    /// Validates this generalized channel against the representation-independent
    /// channel contract.
    pub fn validate_generalized(&self) -> ChannelResult<()> {
        self.validate()?;

        self.config.validate()?;

        Ok(())
    }

    /// Validates whether this channel can be sequentially composed with another
    /// generalized channel.
    ///
    /// This performs only representation-independent admission checking.
    ///
    /// Actual mathematical composition belongs to the appropriate concrete
    /// representation or `channel/composition.rs`.
    pub fn can_compose_sequentially(
        &self,
        next: &GeneralizedChannel,
    ) -> ChannelResult<()> {
        validate_sequential_composition(self, next)
    }

    /// Validates whether this channel can participate in an independent tensor
    /// product with another generalized channel.
    ///
    /// Actual tensor-product materialization remains outside this module.
    pub fn can_tensor_product(
        &self,
        other: &GeneralizedChannel,
    ) -> ChannelResult<()> {
        validate_tensor_product(self, other)
    }

    /// Returns whether this channel can be safely treated as a validated
    /// physical channel without an additional proof/validation step.
    #[must_use]
    pub fn is_physically_validated(&self) -> bool {
        self.physicality().is_validated()
    }

    /// Returns whether the generalized channel is represented exactly.
    #[must_use]
    pub fn is_exact(&self) -> bool {
        self.accuracy().is_exact()
    }

    /// Returns the mathematical representation.
    #[must_use]
    pub const fn channel_representation(&self) -> ChannelRepresentation {
        self.config.representation
    }

    /// Returns the semantic channel identity.
    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        self.config.id
    }
}

// =============================================================================
// Canonical QuantumChannel integration
// =============================================================================

impl QuantumChannel for GeneralizedChannel {
    fn id(&self) -> ChannelId {
        self.config.id
    }

    fn descriptor(&self) -> ChannelDescriptor {
        self.channel_descriptor()
    }

    fn support(&self) -> &ChannelSupport {
        &self.config.support
    }

    fn representation(&self) -> ChannelRepresentation {
        self.config.representation
    }

    fn physicality(&self) -> ChannelPhysicality {
        self.config.physicality
    }

    fn accuracy(&self) -> ChannelAccuracy {
        self.config.accuracy
    }

    fn required_capabilities(&self) -> &[ChannelCapability] {
        &self.config.required_capabilities
    }

    fn resource_requirements(&self) -> ChannelResourceRequirements {
        self.config.resources
    }

    fn semantic_kind(&self) -> &'static str {
        "generalized_quantum_channel"
    }
}

// =============================================================================
// Capability validation
// =============================================================================

/// Validates the declared capability list.
///
/// Capabilities are semantic requirements, not machine-size limits.
fn validate_capabilities(
    capabilities: &[ChannelCapability],
) -> ChannelResult<()> {
    for (index, capability) in capabilities.iter().enumerate() {
        if capabilities
            .iter()
            .skip(index + 1)
            .any(|other| other == capability)
        {
            return Err(ChannelError::IncompatibleComposition);
        }
    }

    Ok(())
}

/// Ensures required capabilities correctly describe structural properties.
///
/// This prevents a generalized channel from advertising a structure while
/// omitting the corresponding semantic requirement.
fn validate_capability_consistency(
    support: &ChannelSupport,
    accuracy: ChannelAccuracy,
    capabilities: &[ChannelCapability],
) -> ChannelResult<()> {
    if !support.is_square() && !capabilities.contains(&ChannelCapability::NonSquare) {
        return Err(ChannelError::MissingCapability(
            ChannelCapability::NonSquare,
        ));
    }

    if support.input_arity() > 1 || support.output_arity() > 1 {
        if !capabilities.contains(&ChannelCapability::ArbitraryArity) {
            return Err(ChannelError::MissingCapability(
                ChannelCapability::ArbitraryArity,
            ));
        }
    }

    match accuracy {
        ChannelAccuracy::Exact => {}
        ChannelAccuracy::Approximate { .. }
        | ChannelAccuracy::Bounded { .. }
        | ChannelAccuracy::Statistical { .. } => {
            if !capabilities.contains(&ChannelCapability::Approximation)
                && !capabilities.contains(&ChannelCapability::ExactRepresentation)
            {
                // The generalized channel is allowed to be approximate, but
                // downstream consumers must be able to see that this is not an
                // exact semantic representation.
                return Err(ChannelError::MissingCapability(
                    ChannelCapability::Approximation,
                ));
            }
        }
    }

    Ok(())
}

// =============================================================================
// Public support helpers
// =============================================================================

/// Creates a generalized channel over canonical Zamani logical qubits.
///
/// The number of qubits is determined entirely by `qubits`.
///
/// No fixed arity is assumed.
pub fn from_qubits(
    id: ChannelId,
    qubits: Vec<QubitId>,
    representation: ChannelRepresentation,
    physicality: ChannelPhysicality,
    accuracy: ChannelAccuracy,
) -> ChannelResult<GeneralizedChannel> {
    let support = ChannelSupport::square(
        qubits
            .into_iter()
            .map(|qubit| {
                super::channel::ChannelSubsystem::new(
                    super::channel::ChannelSubsystemId::Qubit(qubit),
                    2,
                )
            })
            .collect::<ChannelResult<Vec<_>>>()?,
    )?;

    let mut config = GeneralizedChannelConfig::new(
        id,
        GeneralizedChannelKind::QuantumProcess,
        support,
        representation,
        physicality,
        accuracy,
    )?;

    if config.support.input_arity() > 1 {
        config
            .required_capabilities
            .push(ChannelCapability::ArbitraryArity);
    }

    if !config.accuracy.is_exact() {
        config
            .required_capabilities
            .push(ChannelCapability::Approximation);
    }

    GeneralizedChannel::new(config)
}

/// Creates a generalized channel from arbitrary subsystem descriptors.
///
/// This supports qudits, modes, bosonic truncations and future modalities.
pub fn from_support(
    id: ChannelId,
    support: ChannelSupport,
    representation: ChannelRepresentation,
    physicality: ChannelPhysicality,
    accuracy: ChannelAccuracy,
    required_capabilities: impl IntoIterator<Item = ChannelCapability>,
) -> ChannelResult<GeneralizedChannel> {
    let config = GeneralizedChannelConfig::new(
        id,
        GeneralizedChannelKind::QuantumProcess,
        support,
        representation,
        physicality,
        accuracy,
    )?
    .with_required_capabilities(required_capabilities);

    GeneralizedChannel::new(config)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn qubit(id: u64) -> QubitId {
        QubitId::new(id)
    }

    fn one_qubit_support(id: u64) -> ChannelSupport {
        ChannelSupport::square(vec![
            super::super::channel::ChannelSubsystem::new(
                super::super::channel::ChannelSubsystemId::Qubit(qubit(id)),
                2,
            )
            .expect("qubit subsystem must be valid"),
        ])
        .expect("support must be valid")
    }

    #[test]
    fn generalized_channel_implements_canonical_channel_contract() {
        let channel = GeneralizedChannel::quantum_process(
            ChannelId::from_u128(1),
            one_qubit_support(0),
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
        )
        .expect("generalized channel must be constructible");

        assert_eq!(channel.id(), ChannelId::from_u128(1));
        assert_eq!(channel.representation(), ChannelRepresentation::Custom);
        assert_eq!(channel.semantic_kind(), "generalized_quantum_channel");
        assert_eq!(channel.input_dimension(), Some(2));
        assert_eq!(channel.output_dimension(), Some(2));
    }

    #[test]
    fn canonical_qubit_identity_is_preserved() {
        let channel = GeneralizedChannel::quantum_process(
            ChannelId::from_u128(2),
            one_qubit_support(17),
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
        )
        .expect("channel must be valid");

        assert_eq!(channel.input_qubits(), vec![qubit(17)]);
        assert_eq!(channel.output_qubits(), vec![qubit(17)]);
    }

    #[test]
    fn arbitrary_qubit_arity_is_supported() {
        let channel = from_qubits(
            ChannelId::from_u128(3),
            (0..128).map(qubit).collect(),
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
        )
        .expect("arbitrary support should be valid");

        assert_eq!(channel.support().input_arity(), 128);
        assert_eq!(channel.support().output_arity(), 128);
        assert!(channel.requires(ChannelCapability::ArbitraryArity));
    }

    #[test]
    fn arbitrary_arity_has_no_architectural_machine_limit() {
        let qubits = (0..1024).map(qubit).collect::<Vec<_>>();

        let channel = from_qubits(
            ChannelId::from_u128(4),
            qubits,
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
        )
        .expect("larger generated support should be valid");

        assert_eq!(channel.support().input_arity(), 1024);
    }

    #[test]
    fn non_square_support_requires_explicit_capability() {
        let input = super::super::channel::ChannelSubsystem::new(
            super::super::channel::ChannelSubsystemId::opaque(1),
            2,
        )
        .expect("valid input");

        let output = super::super::channel::ChannelSubsystem::new(
            super::super::channel::ChannelSubsystemId::opaque(2),
            3,
        )
        .expect("valid output");

        let support = ChannelSupport::new(vec![input], vec![output])
            .expect("non-square support is structurally valid");

        let result = GeneralizedChannelConfig::new(
            ChannelId::from_u128(5),
            GeneralizedChannelKind::QuantumProcess,
            support,
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
        );

        assert!(matches!(
            result,
            Err(ChannelError::MissingCapability(
                ChannelCapability::NonSquare
            ))
        ));
    }

    #[test]
    fn non_square_support_can_be_declared_explicitly() {
        let input = super::super::channel::ChannelSubsystem::new(
            super::super::channel::ChannelSubsystemId::opaque(1),
            2,
        )
        .expect("valid input");

        let output = super::super::channel::ChannelSubsystem::new(
            super::super::channel::ChannelSubsystemId::opaque(2),
            3,
        )
        .expect("valid output");

        let support = ChannelSupport::new(vec![input], vec![output])
            .expect("support is valid");

        let channel = from_support(
            ChannelId::from_u128(6),
            support,
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
            [ChannelCapability::NonSquare],
        )
        .expect("explicit non-square capability should be accepted");

        assert!(!channel.is_square());
        assert!(channel.requires(ChannelCapability::NonSquare));
    }

    #[test]
    fn approximate_channel_requires_explicit_approximation_capability() {
        let accuracy = ChannelAccuracy::approximate(1.0e-9)
            .expect("valid tolerance");

        let result = GeneralizedChannelConfig::new(
            ChannelId::from_u128(7),
            GeneralizedChannelKind::QuantumProcess,
            one_qubit_support(0),
            ChannelRepresentation::Custom,
            ChannelPhysicality::Conditional,
            accuracy,
        );

        assert!(matches!(
            result,
            Err(ChannelError::MissingCapability(
                ChannelCapability::Approximation
            ))
        ));
    }

    #[test]
    fn approximate_channel_can_be_declared_explicitly() {
        let accuracy =
            ChannelAccuracy::approximate(1.0e-9).expect("valid tolerance");

        let channel = from_support(
            ChannelId::from_u128(8),
            one_qubit_support(0),
            ChannelRepresentation::Custom,
            ChannelPhysicality::Conditional,
            accuracy,
            [ChannelCapability::Approximation],
        )
        .expect("approximation must be explicit");

        assert!(!channel.is_exact());
        assert!(channel.requires(ChannelCapability::Approximation));
    }

    #[test]
    fn duplicate_capabilities_are_rejected() {
        let result = from_support(
            ChannelId::from_u128(9),
            one_qubit_support(0),
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
            [
                ChannelCapability::PhysicalValidation,
                ChannelCapability::PhysicalValidation,
            ],
        );

        assert!(matches!(
            result,
            Err(ChannelError::IncompatibleComposition)
        ));
    }

    #[test]
    fn physicality_is_never_inferred_from_structure() {
        let channel = GeneralizedChannel::quantum_process(
            ChannelId::from_u128(10),
            one_qubit_support(0),
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
        )
        .expect("channel must be valid");

        assert!(!channel.is_physically_validated());
        assert!(matches!(
            channel.validate_physicality(),
            Err(ChannelError::NotValidated)
        ));
    }

    #[test]
    fn validated_physicality_is_preserved() {
        let channel = GeneralizedChannel::quantum_process(
            ChannelId::from_u128(11),
            one_qubit_support(0),
            ChannelRepresentation::Custom,
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
        )
        .expect("channel must be valid");

        assert!(channel.is_physically_validated());
        assert!(channel.validate_physicality().is_ok());
    }

    #[test]
    fn sequential_composition_is_checked_without_materialization() {
        let first = GeneralizedChannel::quantum_process(
            ChannelId::from_u128(12),
            one_qubit_support(0),
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
        )
        .expect("first channel");

        let second = GeneralizedChannel::quantum_process(
            ChannelId::from_u128(13),
            one_qubit_support(0),
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
        )
        .expect("second channel");

        assert!(first.can_compose_sequentially(&second).is_ok());
    }

    #[test]
    fn tensor_product_requires_disjoint_resources() {
        let first = GeneralizedChannel::quantum_process(
            ChannelId::from_u128(14),
            one_qubit_support(0),
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
        )
        .expect("first channel");

        let second = GeneralizedChannel::quantum_process(
            ChannelId::from_u128(15),
            one_qubit_support(1),
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
        )
        .expect("second channel");

        assert!(first.can_tensor_product(&second).is_ok());
    }

    #[test]
    fn tensor_product_rejects_resource_overlap() {
        let first = GeneralizedChannel::quantum_process(
            ChannelId::from_u128(16),
            one_qubit_support(0),
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
        )
        .expect("first channel");

        let second = GeneralizedChannel::quantum_process(
            ChannelId::from_u128(17),
            one_qubit_support(0),
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
        )
        .expect("second channel");

        assert!(matches!(
            first.can_tensor_product(&second),
            Err(ChannelError::DuplicateQubit(found)) if found == qubit(0)
        ));
    }

    #[test]
    fn extension_kind_requires_non_empty_name() {
        assert!(matches!(
            GeneralizedChannelKind::extension("   "),
            Err(ChannelError::InvalidIdentity)
        ));
    }

    #[test]
    fn extension_kind_is_stable() {
        let kind =
            GeneralizedChannelKind::extension("bosonic-process")
                .expect("valid extension name");

        assert_eq!(kind.as_str(), "bosonic-process");
        assert_eq!(kind.to_string(), "bosonic-process");
    }

    #[test]
    fn resource_requirements_remain_descriptive() {
        let requirements = ChannelResourceRequirements::known(
            Some(1024),
            Some(8192),
            Some(16384),
        );

        let channel = GeneralizedChannel::new(
            GeneralizedChannelConfig::new(
                ChannelId::from_u128(18),
                GeneralizedChannelKind::TargetProcess,
                one_qubit_support(0),
                ChannelRepresentation::Custom,
                ChannelPhysicality::Unvalidated,
                ChannelAccuracy::Exact,
            )
            .expect("base configuration")
            .with_resources(requirements),
        )
        .expect("channel must be valid");

        assert_eq!(
            channel.resource_requirements().scalar_elements,
            Some(1024)
        );
        assert_eq!(
            channel.resource_requirements().memory_bytes,
            Some(8192)
        );
        assert_eq!(
            channel.resource_requirements().arithmetic_operations,
            Some(16384)
        );
    }

    #[test]
    fn generalized_channel_is_immutable_semantically() {
        let channel = GeneralizedChannel::quantum_process(
            ChannelId::from_u128(19),
            one_qubit_support(0),
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
        )
        .expect("channel must be valid");

        let descriptor_a = channel.descriptor();
        let descriptor_b = channel.descriptor();

        assert_eq!(descriptor_a, descriptor_b);
    }

    #[test]
    fn generalized_channel_supports_non_qubit_modalities() {
        let qutrit = super::super::channel::ChannelSubsystem::new(
            super::super::channel::ChannelSubsystemId::opaque(100),
            3,
        )
        .expect("qutrit");

        let mode = super::super::channel::ChannelSubsystem::new(
            super::super::channel::ChannelSubsystemId::opaque(200),
            16,
        )
        .expect("bosonic truncation");

        let support =
            ChannelSupport::square(vec![qutrit, mode])
                .expect("mixed modality support");

        let channel = from_support(
            ChannelId::from_u128(20),
            support,
            ChannelRepresentation::Custom,
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
            [ChannelCapability::ArbitraryArity],
        )
        .expect("generalized mixed modality channel");

        assert_eq!(channel.input_dimension(), Some(48));
        assert_eq!(channel.output_dimension(), Some(48));
        assert!(channel.input_qubits().is_empty());
    }
}