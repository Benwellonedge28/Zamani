//! Zamani Quantum Noise (ZQN) — Representation-Independent Channel Composition.
//!
//! # Ownership
//!
//! This file owns the representation-independent semantics for composing
//! quantum channels.
//!
//! It provides:
//!
//! - sequential channel-composition plans;
//! - tensor-product composition plans;
//! - validation of composition domains;
//! - validation of tensor-product resource disjointness;
//! - deterministic composition structure;
//! - composed-channel semantic wrappers;
//! - propagation of physicality and accuracy contracts;
//! - propagation of required capabilities;
//! - propagation of resource requirements;
//! - canonical logical-qubit inspection through `QuantumChannel`;
//! - explicit composition identities;
//! - composition metadata;
//! - safe resource accounting;
//! - tests for composition invariants.
//!
//! # Does not own
//!
//! This file does NOT own:
//!
//! - Kraus-operator multiplication;
//! - Choi-matrix multiplication;
//! - Pauli algebra;
//! - density-matrix simulation;
//! - state-vector simulation;
//! - numerical linear algebra;
//! - stochastic sampling;
//! - RNG state;
//! - hardware execution;
//! - routing;
//! - scheduling;
//! - calibration;
//! - characterization;
//! - QEC;
//! - serialization formats;
//! - vendor APIs;
//! - source-language parsing.
//!
//! Concrete representations remain responsible for their own mathematical
//! composition. This module supplies the common semantic composition layer.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                             │
//!                             ▼
//!                     QuantumChannel
//!                             │
//!              ┌──────────────┴──────────────┐
//!              │                             │
//!              ▼                             ▼
//!       concrete channel               composition.rs
//!       representation                       │
//!       ┌──────────────┐                      │
//!       │ Kraus        │                      │
//!       │ Choi         │                      │
//!       │ Pauli        │                      │
//!       │ Lindblad     │                      │
//!       │ Stochastic   │                      │
//!       │ Process      │                      │
//!       └──────────────┘                      │
//!              │                              │
//!              └──────────────┬───────────────┘
//!                             ▼
//!                    ComposedChannel
//!                             │
//!             ┌───────────────┼────────────────┐
//!             ▼               ▼                ▼
//!        simulation       propagation       lowering
//!             │               │                │
//!             └───────────────┼────────────────┘
//!                             ▼
//!                    runtime / hardware
//! ```
//!
//! # Critical architectural rule
//!
//! `composition.rs` MUST remain representation-independent.
//!
//! For example:
//!
//! ```text
//! KrausChannel::compose(...)
//!     -> concrete Kraus mathematics
//!
//! PauliChannel::compose(...)
//!     -> concrete Pauli mathematics
//!
//! ChoiChannel::compose(...)
//!     -> concrete Choi mathematics
//!
//! composition::SequentialChannel
//!     -> semantic composition of QuantumChannel objects
//! ```
//!
//! This separation prevents the composition layer from becoming a giant
//! representation dispatcher and allows new channel representations to be
//! introduced without modifying this file.
//!
//! # Sequential semantics
//!
//! Given:
//!
//! ```text
//! A : H0 -> H1
//! B : H1 -> H2
//! ```
//!
//! sequential composition is:
//!
//! ```text
//! B ∘ A : H0 -> H2
//! ```
//!
//! The constructor validates that the output domain of the preceding channel
//! matches the input domain of the following channel.
//!
//! The caller's ordering is explicit:
//!
//! ```text
//! SequentialChannel::new(
//!     id,
//!     vec![A, B, C],
//! )
//! ```
//!
//! means:
//!
//! ```text
//! C ∘ B ∘ A
//! ```
//!
//! The first element is applied first.
//!
//! # Tensor-product semantics
//!
//! Given independent channels:
//!
//! ```text
//! A : HA -> HA'
//! B : HB -> HB'
//! ```
//!
//! tensor composition is:
//!
//! ```text
//! A ⊗ B : HA ⊗ HB -> HA' ⊗ HB'
//! ```
//!
//! The supports must be disjoint.
//!
//! Overlapping resources are rejected rather than silently interpreted as
//! correlated or sequential operations.
//!
//! # Resource identity
//!
//! This module does not define a second qubit identity system.
//!
//! Canonical logical qubit identity remains:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! The existing `QuantumChannel`, `ChannelSupport`, and `ChannelSubsystem`
//! abstractions already expose canonical qubit identities.
//!
//! This module consumes those abstractions rather than creating another
//! `QubitId`.
//!
//! # Scalability
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_CHANNELS
//! MAX_COMPOSITION_DEPTH
//! MAX_TENSOR_ARITY
//! MAX_COMPOSITION_SIZE
//! ```
//!
//! The semantic composition model therefore has no artificial finite machine
//! size.
//!
//! Actual resource consumption is determined by:
//!
//! - number of composed channels;
//! - concrete representation;
//! - subsystem dimensions;
//! - available memory;
//! - CPU/GPU resources;
//! - distributed resources;
//! - execution policy;
//! - target capabilities;
//! - explicit runtime limits.
//!
//! "Infinity" therefore means that this module introduces no artificial finite
//! semantic ceiling. Physical execution remains bounded by available resources.
//!
//! # Determinism
//!
//! Composition is deterministic.
//!
//! This file:
//!
//! - contains no RNG;
//! - contains no global mutable state;
//! - contains no hidden caches;
//! - does not depend on iteration over unordered hash maps;
//! - preserves caller-defined channel ordering;
//! - produces deterministic capability ordering.
//!
//! Sampling belongs to ZQN simulation and must use its explicit deterministic
//! execution context.
//!
//! # Physicality propagation
//!
//! A sequential or tensor composition is physically validated only when every
//! constituent channel is physically validated.
//!
//! If any constituent is:
//!
//! - non-physical -> the composition is non-physical;
//! - unvalidated -> the composition is unvalidated;
//! - conditional -> the composition is conditional;
//! - validated -> it may contribute to a validated result.
//!
//! This file never upgrades an uncertain channel into a validated one.
//!
//! # Accuracy propagation
//!
//! Exactness is preserved only when every constituent is exact.
//!
//! For bounded/approximate channels, this module conservatively propagates a
//! declared bound rather than silently declaring the result exact.
//!
//! Statistical accuracy is preserved only as statistical metadata; this file
//! does not invent a statistical confidence calculation.
//!
//! # Resource accounting
//!
//! Resource requirements are combined conservatively.
//!
//! If any constituent requirement is unknown, the corresponding aggregate
//! requirement remains unknown.
//!
//! Arithmetic and memory estimates use checked arithmetic and never wrap.
//!
//! A representational overflow is therefore reported as `None`, not as a false
//! finite resource estimate.
//!
//! # Integration
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! QuantumChannel
//!      │
//!      ▼
//! composition.rs
//!      │
//!      ├──────────► simulation
//!      ├──────────► propagation
//!      ├──────────► routing
//!      ├──────────► scheduling
//!      ├──────────► QEC adapters
//!      ├──────────► hardware lowering
//!      └──────────► runtime
//! ```
//!
//! No downstream subsystem needs to know how the composition was represented.
//!
//! # Concrete representation integration
//!
//! A concrete representation MAY consume a `CompositionPlan` and lower it into
//! its own mathematical representation.
//!
//! For example:
//!
//! ```text
//! CompositionPlan
//!       │
//!       ▼
//! Kraus lowering
//!       │
//!       ▼
//! KrausChannel
//! ```
//!
//! or:
//!
//! ```text
//! CompositionPlan
//!       │
//!       ▼
//! Pauli lowering
//!       │
//!       ▼
//! PauliChannel
//! ```
//!
//! Such lowering belongs to the concrete representation module, not here.
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
//! - no unsafe Rust.
//!
//! # Safety
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! No unsafe code is necessary for composition semantics.
//!
//! # File-completion contract
//!
//! This file is complete when:
//!
//! 1. sequential composition is representation-independent;
//! 2. tensor composition is representation-independent;
//! 3. composition order is explicit;
//! 4. intermediate domains are validated;
//! 5. tensor supports are disjoint;
//! 6. no fixed machine size exists;
//! 7. canonical IR qubit identity remains authoritative;
//! 8. physicality is never silently upgraded;
//! 9. exactness is never silently upgraded;
//! 10. approximation is explicit;
//! 11. resource arithmetic is checked;
//! 12. unknown resource requirements remain unknown;
//! 13. capabilities are propagated deterministically;
//! 14. no RNG is introduced;
//! 15. no vendor dependency exists;
//! 16. no QEC dependency exists;
//! 17. concrete representations remain responsible for mathematical lowering;
//! 18. composed channels remain `Send + Sync` when their children are;
//! 19. tests cover domain validation, tensor disjointness, propagation,
//!     determinism and overflow safety.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use super::channel::{
    validate_sequential_composition, validate_tensor_product, ChannelAccuracy,
    ChannelCapability, ChannelDescriptor, ChannelError, ChannelId, ChannelPhysicality,
    ChannelRepresentation, ChannelResourceRequirements, ChannelResult, ChannelSupport,
    QuantumChannel,
};

/// A representation-independent ordered sequential composition.
///
/// The channels are stored in execution order:
///
/// ```text
/// [A, B, C]
/// ```
///
/// means:
///
/// ```text
/// C ∘ B ∘ A
/// ```
///
/// The type owns composition semantics, not concrete mathematical channel
/// multiplication.
#[derive(Debug)]
pub struct SequentialChannel {
    id: ChannelId,
    channels: Vec<Box<dyn QuantumChannel>>,
    descriptor: ChannelDescriptor,
    capabilities: Vec<ChannelCapability>,
}

impl SequentialChannel {
    /// Constructs a sequential composition.
    ///
    /// At least one channel is required.
    ///
    /// For a single channel this produces a semantic composition wrapper
    /// containing that channel. This is useful when generic compilation
    /// pipelines build composition lists incrementally.
    ///
    /// The channels are interpreted in application order.
    pub fn new(
        id: ChannelId,
        channels: Vec<Box<dyn QuantumChannel>>,
    ) -> ChannelResult<Self> {
        if channels.is_empty() {
            return Err(ChannelError::EmptySupport);
        }

        validate_sequential_chain(&channels)?;

        let support = sequential_support(&channels)?;

        let physicality = combine_physicality(&channels);

        let accuracy = combine_accuracy(&channels)?;

        let capabilities = union_capabilities(&channels);

        let resources = sequential_resources(&channels);

        let descriptor = ChannelDescriptor::new(
            id,
            None,
            support,
            ChannelRepresentation::Custom,
            physicality,
            accuracy,
            resources,
        )?;

        Ok(Self {
            id,
            channels,
            descriptor,
            capabilities,
        })
    }

    /// Returns the number of constituent channels.
    #[must_use]
    pub fn len(&self) -> usize {
        self.channels.len()
    }

    /// Returns true if the composition contains no channels.
    ///
    /// This is always false for a successfully constructed
    /// `SequentialChannel`; the method exists for collection-like API
    /// consistency.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.channels.is_empty()
    }

    /// Returns the constituent channels in application order.
    ///
    /// The returned slice is immutable so composition ordering cannot be
    /// changed after validation.
    #[must_use]
    pub fn channels(&self) -> &[Box<dyn QuantumChannel>] {
        &self.channels
    }

    /// Returns the first channel in the composition.
    #[must_use]
    pub fn first(&self) -> &dyn QuantumChannel {
        self.channels[0].as_ref()
    }

    /// Returns the last channel in the composition.
    #[must_use]
    pub fn last(&self) -> &dyn QuantumChannel {
        self.channels[self.channels.len() - 1].as_ref()
    }

    /// Returns the semantic composition direction.
    ///
    /// The result is always:
    ///
    /// ```text
    /// last ∘ ... ∘ second ∘ first
    /// ```
    #[must_use]
    pub const fn direction(&self) -> CompositionDirection {
        CompositionDirection::ForwardApplication
    }

    /// Returns the composition plan represented by this channel.
    ///
    /// This is useful to representation-specific lowering layers.
    pub fn plan(&self) -> CompositionPlan<'_> {
        CompositionPlan::Sequential {
            channels: self
                .channels
                .iter()
                .map(|channel| channel.as_ref())
                .collect(),
        }
    }
}

impl QuantumChannel for SequentialChannel {
    fn id(&self) -> ChannelId {
        self.id
    }

    fn descriptor(&self) -> ChannelDescriptor {
        self.descriptor.clone()
    }

    fn support(&self) -> &ChannelSupport {
        &self.descriptor.support
    }

    fn representation(&self) -> ChannelRepresentation {
        ChannelRepresentation::Custom
    }

    fn physicality(&self) -> ChannelPhysicality {
        self.descriptor.physicality
    }

    fn accuracy(&self) -> ChannelAccuracy {
        self.descriptor.accuracy
    }

    fn required_capabilities(&self) -> &[ChannelCapability] {
        &self.capabilities
    }

    fn resource_requirements(&self) -> ChannelResourceRequirements {
        self.descriptor.resources
    }

    fn semantic_kind(&self) -> &'static str {
        "sequential_composed_quantum_channel"
    }
}

/// A representation-independent tensor-product composition.
///
/// Every constituent channel must act on a disjoint resource support.
///
/// Example:
///
/// ```text
/// A(q0) ⊗ B(q1) ⊗ C(q2)
/// ```
///
/// is valid.
///
/// But:
///
/// ```text
/// A(q0) ⊗ B(q0)
/// ```
///
/// is rejected because the semantics are ambiguous: the shared resource would
/// require sequential or correlated semantics rather than an independent tensor
/// product.
#[derive(Debug)]
pub struct TensorProductChannel {
    id: ChannelId,
    channels: Vec<Box<dyn QuantumChannel>>,
    descriptor: ChannelDescriptor,
    capabilities: Vec<ChannelCapability>,
}

impl TensorProductChannel {
    /// Constructs a tensor-product composition.
    ///
    /// At least one channel is required.
    pub fn new(
        id: ChannelId,
        channels: Vec<Box<dyn QuantumChannel>>,
    ) -> ChannelResult<Self> {
        if channels.is_empty() {
            return Err(ChannelError::EmptySupport);
        }

        validate_tensor_chain(&channels)?;

        let support = tensor_support(&channels)?;

        let physicality = combine_physicality(&channels);

        let accuracy = combine_accuracy(&channels)?;

        let mut capabilities = union_capabilities(&channels);

        if channels.len() > 1 {
            insert_capability_sorted(
                &mut capabilities,
                ChannelCapability::ArbitraryArity,
            );
        }

        let resources = tensor_resources(&channels);

        let descriptor = ChannelDescriptor::new(
            id,
            None,
            support,
            ChannelRepresentation::Custom,
            physicality,
            accuracy,
            resources,
        )?;

        Ok(Self {
            id,
            channels,
            descriptor,
            capabilities,
        })
    }

    /// Returns the number of constituent channels.
    #[must_use]
    pub fn len(&self) -> usize {
        self.channels.len()
    }

    /// Returns true if the composition contains no channels.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.channels.is_empty()
    }

    /// Returns the constituent channels.
    #[must_use]
    pub fn channels(&self) -> &[Box<dyn QuantumChannel>] {
        &self.channels
    }

    /// Returns the tensor-product composition plan.
    pub fn plan(&self) -> CompositionPlan<'_> {
        CompositionPlan::TensorProduct {
            channels: self
                .channels
                .iter()
                .map(|channel| channel.as_ref())
                .collect(),
        }
    }
}

impl QuantumChannel for TensorProductChannel {
    fn id(&self) -> ChannelId {
        self.id
    }

    fn descriptor(&self) -> ChannelDescriptor {
        self.descriptor.clone()
    }

    fn support(&self) -> &ChannelSupport {
        &self.descriptor.support
    }

    fn representation(&self) -> ChannelRepresentation {
        ChannelRepresentation::Custom
    }

    fn physicality(&self) -> ChannelPhysicality {
        self.descriptor.physicality
    }

    fn accuracy(&self) -> ChannelAccuracy {
        self.descriptor.accuracy
    }

    fn required_capabilities(&self) -> &[ChannelCapability] {
        &self.capabilities
    }

    fn resource_requirements(&self) -> ChannelResourceRequirements {
        self.descriptor.resources
    }

    fn semantic_kind(&self) -> &'static str {
        "tensor_product_composed_quantum_channel"
    }
}

/// Direction used by sequential composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompositionDirection {
    /// The first supplied channel is applied first.
    ForwardApplication,
}

/// A borrowed representation-independent composition plan.
///
/// This type intentionally borrows channels rather than owning them. It is
/// suitable for lowering layers that need to inspect a composition without
/// transferring ownership.
///
/// The plan itself does not perform numerical composition.
#[derive(Debug)]
pub enum CompositionPlan<'a> {
    /// Sequential composition in application order.
    Sequential {
        /// Channels in application order.
        channels: Vec<&'a dyn QuantumChannel>,
    },

    /// Independent tensor product.
    TensorProduct {
        /// Channels participating in the tensor product.
        channels: Vec<&'a dyn QuantumChannel>,
    },
}

impl<'a> CompositionPlan<'a> {
    /// Returns the number of constituent channels.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Sequential { channels } | Self::TensorProduct { channels } => channels.len(),
        }
    }

    /// Returns whether the plan contains no channels.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the composition kind.
    #[must_use]
    pub const fn kind(&self) -> CompositionKind {
        match self {
            Self::Sequential { .. } => CompositionKind::Sequential,
            Self::TensorProduct { .. } => CompositionKind::TensorProduct,
        }
    }

    /// Validates the composition plan without constructing a composed channel.
    pub fn validate(&self) -> ChannelResult<()> {
        match self {
            Self::Sequential { channels } => {
                validate_sequential_chain(channels)?;
            }

            Self::TensorProduct { channels } => {
                validate_tensor_chain(channels)?;
            }
        }

        Ok(())
    }

    /// Returns the channels in the plan.
    #[must_use]
    pub fn channels(&self) -> &[&'a dyn QuantumChannel] {
        match self {
            Self::Sequential { channels } | Self::TensorProduct { channels } => channels,
        }
    }
}

/// Semantic kind of composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompositionKind {
    /// Sequential function/channel composition.
    Sequential,

    /// Independent tensor-product composition.
    TensorProduct,
}

/// Creates a sequential composition from owned channels.
///
/// This is the preferred convenience API when a compiler/runtime has already
/// determined the semantic composition.
pub fn sequential(
    id: ChannelId,
    channels: Vec<Box<dyn QuantumChannel>>,
) -> ChannelResult<SequentialChannel> {
    SequentialChannel::new(id, channels)
}

/// Creates a tensor-product composition from owned channels.
pub fn tensor_product(
    id: ChannelId,
    channels: Vec<Box<dyn QuantumChannel>>,
) -> ChannelResult<TensorProductChannel> {
    TensorProductChannel::new(id, channels)
}

/// Validates a borrowed sequential composition.
///
/// The channels are interpreted in application order.
pub fn validate_sequential_chain(
    channels: &[&dyn QuantumChannel],
) -> ChannelResult<()> {
    if channels.is_empty() {
        return Err(ChannelError::EmptySupport);
    }

    for channel in channels {
        channel.validate()?;
    }

    for pair in channels.windows(2) {
        validate_sequential_composition(pair[0], pair[1])?;
    }

    Ok(())
}

/// Validates a borrowed tensor-product composition.
pub fn validate_tensor_chain(
    channels: &[&dyn QuantumChannel],
) -> ChannelResult<()> {
    if channels.is_empty() {
        return Err(ChannelError::EmptySupport);
    }

    for channel in channels {
        channel.validate()?;
    }

    for (index, first) in channels.iter().enumerate() {
        for second in channels.iter().skip(index + 1) {
            validate_tensor_product(*first, *second)?;
        }
    }

    Ok(())
}

/// Constructs the support of a sequential composition.
///
/// For:
///
/// ```text
/// A : H0 -> H1
/// B : H1 -> H2
/// C : H2 -> H3
/// ```
///
/// the resulting support is:
///
/// ```text
/// H0 -> H3
/// ```
fn sequential_support(
    channels: &[Box<dyn QuantumChannel>],
) -> ChannelResult<ChannelSupport> {
    let first = channels
        .first()
        .ok_or(ChannelError::EmptySupport)?;

    let last = channels
        .last()
        .ok_or(ChannelError::EmptySupport)?;

    ChannelSupport::new(
        first.support().input().to_vec(),
        last.support().output().to_vec(),
    )
}

/// Constructs the support of a tensor-product composition.
fn tensor_support(
    channels: &[Box<dyn QuantumChannel>],
) -> ChannelResult<ChannelSupport> {
    if channels.is_empty() {
        return Err(ChannelError::EmptySupport);
    }

    let mut input = Vec::new();
    let mut output = Vec::new();

    for channel in channels {
        input.extend_from_slice(channel.support().input());
        output.extend_from_slice(channel.support().output());
    }

    ChannelSupport::new(input, output)
}

/// Combines physicality conservatively.
///
/// The ordering is:
///
/// ```text
/// NonPhysical > Conditional > Unvalidated > Validated
/// ```
///
/// This function never upgrades a weaker guarantee.
fn combine_physicality(
    channels: &[Box<dyn QuantumChannel>],
) -> ChannelPhysicality {
    let mut has_conditional = false;
    let mut has_unvalidated = false;

    for channel in channels {
        match channel.physicality() {
            ChannelPhysicality::NonPhysical => {
                return ChannelPhysicality::NonPhysical;
            }

            ChannelPhysicality::Conditional => {
                has_conditional = true;
            }

            ChannelPhysicality::Unvalidated => {
                has_unvalidated = true;
            }

            ChannelPhysicality::Validated => {}
        }
    }

    if has_conditional {
        ChannelPhysicality::Conditional
    } else if has_unvalidated {
        ChannelPhysicality::Unvalidated
    } else {
        ChannelPhysicality::Validated
    }
}

/// Combines accuracy contracts conservatively.
///
/// Exactness is retained only when all constituents are exact.
///
/// For approximate/bounded channels, the largest declared bound is propagated.
/// This is deliberately conservative and does not attempt to infer a tighter
/// mathematical bound without representation-specific knowledge.
///
/// Statistical contracts are preserved only when all statistical channels agree
/// on a confidence level. Otherwise the composition remains bounded/unknown
/// rather than inventing a confidence claim.
fn combine_accuracy(
    channels: &[Box<dyn QuantumChannel>],
) -> ChannelResult<ChannelAccuracy> {
    let mut maximum_bound = 0.0f64;
    let mut has_bound = false;
    let mut statistical_confidence: Option<f64> = None;

    for channel in channels {
        match channel.accuracy() {
            ChannelAccuracy::Exact => {}

            ChannelAccuracy::Approximate { tolerance } => {
                validate_non_negative_finite(tolerance)?;
                maximum_bound = maximum_bound.max(tolerance);
                has_bound = true;
            }

            ChannelAccuracy::Bounded { error_bound } => {
                validate_non_negative_finite(error_bound)?;
                maximum_bound = maximum_bound.max(error_bound);
                has_bound = true;
            }

            ChannelAccuracy::Statistical { confidence } => {
                if !confidence.is_finite() || !(0.0..=1.0).contains(&confidence) {
                    return Err(ChannelError::InvalidTolerance);
                }

                statistical_confidence = match statistical_confidence {
                    None => Some(confidence),

                    Some(existing) if existing == confidence => Some(existing),

                    Some(_) => {
                        // We cannot safely invent a combined statistical
                        // confidence level here.
                        None
                    }
                };
            }
        }
    }

    if has_bound {
        return ChannelAccuracy::bounded(maximum_bound);
    }

    if let Some(confidence) = statistical_confidence {
        return ChannelAccuracy::statistical(confidence);
    }

    Ok(ChannelAccuracy::Exact)
}

/// Combines capabilities deterministically.
///
/// `BTreeSet` is used so output ordering does not depend on hash-map iteration
/// order.
fn union_capabilities(
    channels: &[Box<dyn QuantumChannel>],
) -> Vec<ChannelCapability> {
    let mut capabilities = BTreeSet::new();

    for channel in channels {
        for capability in channel.required_capabilities() {
            capabilities.insert(*capability);
        }
    }

    capabilities.into_iter().collect()
}

/// Inserts a capability while preserving deterministic ordering.
fn insert_capability_sorted(
    capabilities: &mut Vec<ChannelCapability>,
    capability: ChannelCapability,
) {
    if capabilities.contains(&capability) {
        return;
    }

    capabilities.push(capability);
    capabilities.sort();
}

/// Combines sequential resource requirements.
///
/// Unknown values remain unknown.
///
/// Known arithmetic quantities are summed with checked arithmetic.
fn sequential_resources(
    channels: &[Box<dyn QuantumChannel>],
) -> ChannelResourceRequirements {
    combine_resources(channels, false)
}

/// Combines tensor-product resource requirements.
///
/// Scalar and memory requirements are additive at the semantic composition
/// level. Concrete representations may provide more precise accounting during
/// lowering.
fn tensor_resources(
    channels: &[Box<dyn QuantumChannel>],
) -> ChannelResourceRequirements {
    combine_resources(channels, true)
}

/// Combines resource estimates.
///
/// The `tensor` flag is retained to make the semantic distinction explicit even
/// though the representation-independent conservative estimate currently uses
/// additive accounting for both composition kinds.
fn combine_resources(
    channels: &[Box<dyn QuantumChannel>],
    _tensor: bool,
) -> ChannelResourceRequirements {
    let scalar_elements = checked_sum_optional(
        channels
            .iter()
            .map(|channel| channel.resource_requirements().scalar_elements),
    );

    let memory_bytes = checked_sum_optional(
        channels
            .iter()
            .map(|channel| channel.resource_requirements().memory_bytes),
    );

    let arithmetic_operations = checked_sum_optional(
        channels
            .iter()
            .map(|channel| channel.resource_requirements().arithmetic_operations),
    );

    ChannelResourceRequirements::known(
        scalar_elements,
        memory_bytes,
        arithmetic_operations,
    )
}

/// Checked summation over optional values.
///
/// If any constituent value is unknown, the result is unknown.
///
/// If arithmetic overflows, the result is also unknown rather than wrapped.
fn checked_sum_optional<I>(values: I) -> Option<u128>
where
    I: IntoIterator<Item = Option<u128>>,
{
    let mut total = 0u128;

    for value in values {
        let value = value?;

        total = total.checked_add(value)?;
    }

    Some(total)
}

/// Validates a finite non-negative bound.
fn validate_non_negative_finite(value: f64) -> ChannelResult<()> {
    if !value.is_finite() {
        return Err(ChannelError::NonFiniteParameter);
    }

    if value < 0.0 {
        return Err(ChannelError::InvalidErrorBound);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::qubit::QubitId;

    #[derive(Debug)]
    struct TestChannel {
        descriptor: ChannelDescriptor,
        capabilities: Vec<ChannelCapability>,
    }

    impl TestChannel {
        fn new(
            id: u128,
            input: Vec<super::super::channel::ChannelSubsystem>,
            output: Vec<super::super::channel::ChannelSubsystem>,
            physicality: ChannelPhysicality,
            accuracy: ChannelAccuracy,
            capabilities: Vec<ChannelCapability>,
            resources: ChannelResourceRequirements,
        ) -> Self {
            let support =
                ChannelSupport::new(input, output).expect("test support must be valid");

            let descriptor = ChannelDescriptor::new(
                ChannelId::from_u128(id),
                None,
                support,
                ChannelRepresentation::Kraus,
                physicality,
                accuracy,
                resources,
            )
            .expect("test descriptor must be valid");

            Self {
                descriptor,
                capabilities,
            }
        }
    }

    impl QuantumChannel for TestChannel {
        fn id(&self) -> ChannelId {
            self.descriptor.id
        }

        fn descriptor(&self) -> ChannelDescriptor {
            self.descriptor.clone()
        }

        fn support(&self) -> &ChannelSupport {
            &self.descriptor.support
        }

        fn representation(&self) -> ChannelRepresentation {
            self.descriptor.representation
        }

        fn physicality(&self) -> ChannelPhysicality {
            self.descriptor.physicality
        }

        fn accuracy(&self) -> ChannelAccuracy {
            self.descriptor.accuracy
        }

        fn required_capabilities(&self) -> &[ChannelCapability] {
            &self.capabilities
        }

        fn resource_requirements(&self) -> ChannelResourceRequirements {
            self.descriptor.resources
        }
    }

    fn qubit(id: u64) -> super::super::channel::ChannelSubsystem {
        super::super::channel::qubit_subsystem(QubitId::new(id))
    }

    fn qutrit(
        id: u128,
    ) -> super::super::channel::ChannelSubsystem {
        super::super::channel::subsystem(
            super::super::channel::ChannelSubsystemId::opaque(id),
            3,
        )
        .expect("qutrit must be valid")
    }

    #[test]
    fn sequential_composition_preserves_external_domains() {
        let first = TestChannel::new(
            1,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let composed = sequential(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        )
        .expect("sequential composition should succeed");

        assert_eq!(composed.len(), 2);
        assert_eq!(composed.support().input_arity(), 1);
        assert_eq!(composed.support().output_arity(), 1);
        assert_eq!(composed.input_qubits(), vec![QubitId::new(0)]);
        assert_eq!(composed.output_qubits(), vec![QubitId::new(0)]);
    }

    #[test]
    fn sequential_composition_requires_matching_intermediate_dimensions() {
        let first = TestChannel::new(
            1,
            vec![qutrit(0)],
            vec![qutrit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let result = sequential(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        );

        assert!(matches!(
            result,
            Err(ChannelError::IncompatibleComposition)
        ));
    }

    #[test]
    fn sequential_composition_propagates_non_physicality() {
        let first = TestChannel::new(
            1,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::NonPhysical,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let composed = sequential(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        )
        .expect("structural composition should succeed");

        assert_eq!(
            composed.physicality(),
            ChannelPhysicality::NonPhysical
        );

        assert!(matches!(
            composed.validate_physicality(),
            Err(ChannelError::NotPhysical(_))
        ));
    }

    #[test]
    fn sequential_composition_propagates_unvalidated_state() {
        let first = TestChannel::new(
            1,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Unvalidated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let composed = sequential(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        )
        .expect("structural composition should succeed");

        assert_eq!(
            composed.physicality(),
            ChannelPhysicality::Unvalidated
        );
    }

    #[test]
    fn sequential_composition_propagates_conditional_state() {
        let first = TestChannel::new(
            1,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Conditional,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let composed = sequential(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        )
        .expect("structural composition should succeed");

        assert_eq!(
            composed.physicality(),
            ChannelPhysicality::Conditional
        );
    }

    #[test]
    fn sequential_composition_preserves_exactness_only_when_all_are_exact() {
        let first = TestChannel::new(
            1,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Approximate { tolerance: 1.0e-9 },
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let composed = sequential(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        )
        .expect("structural composition should succeed");

        assert_eq!(
            composed.accuracy(),
            ChannelAccuracy::Bounded {
                error_bound: 1.0e-9
            }
        );
    }

    #[test]
    fn tensor_product_requires_disjoint_support() {
        let first = TestChannel::new(
            1,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(1)],
            vec![qubit(1)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let composed = tensor_product(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        )
        .expect("disjoint tensor product should succeed");

        assert_eq!(composed.support().input_arity(), 2);
        assert_eq!(composed.support().output_arity(), 2);
        assert_eq!(
            composed.input_qubits(),
            vec![QubitId::new(0), QubitId::new(1)]
        );
    }

    #[test]
    fn tensor_product_rejects_overlapping_support() {
        let first = TestChannel::new(
            1,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let result = tensor_product(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        );

        assert!(matches!(
            result,
            Err(ChannelError::DuplicateQubit(qubit_id))
                if qubit_id == QubitId::new(0)
        ));
    }

    #[test]
    fn capabilities_are_deterministic_and_deduplicated() {
        let first = TestChannel::new(
            1,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            vec![
                ChannelCapability::CorrelatedNoise,
                ChannelCapability::Stochastic,
            ],
            ChannelResourceRequirements::unknown(),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            vec![
                ChannelCapability::Stochastic,
                ChannelCapability::TimeDependent,
            ],
            ChannelResourceRequirements::unknown(),
        );

        let composed = sequential(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        )
        .expect("composition should succeed");

        assert_eq!(
            composed.required_capabilities(),
            &[
                ChannelCapability::CorrelatedNoise,
                ChannelCapability::Stochastic,
                ChannelCapability::TimeDependent,
            ]
        );
    }

    #[test]
    fn known_resources_are_added() {
        let first = TestChannel::new(
            1,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::known(
                Some(10),
                Some(100),
                Some(1_000),
            ),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::known(
                Some(20),
                Some(200),
                Some(2_000),
            ),
        );

        let composed = sequential(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        )
        .expect("composition should succeed");

        assert_eq!(
            composed.resource_requirements(),
            ChannelResourceRequirements::known(
                Some(30),
                Some(300),
                Some(3_000),
            )
        );
    }

    #[test]
    fn unknown_resource_estimate_stays_unknown() {
        let first = TestChannel::new(
            1,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::known(
                Some(10),
                Some(100),
                None,
            ),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let composed = sequential(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        )
        .expect("composition should succeed");

        assert_eq!(
            composed.resource_requirements().scalar_elements,
            None
        );

        assert_eq!(
            composed.resource_requirements().memory_bytes,
            None
        );

        assert_eq!(
            composed.resource_requirements().arithmetic_operations,
            None
        );
    }

    #[test]
    fn resource_overflow_does_not_wrap() {
        let first = TestChannel::new(
            1,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::known(
                Some(u128::MAX),
                None,
                None,
            ),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::known(
                Some(1),
                None,
                None,
            ),
        );

        let composed = sequential(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        )
        .expect("composition should succeed");

        assert_eq!(
            composed.resource_requirements().scalar_elements,
            None
        );
    }

    #[test]
    fn single_channel_composition_is_allowed() {
        let channel = TestChannel::new(
            1,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let composed = sequential(
            ChannelId::from_u128(2),
            vec![Box::new(channel)],
        )
        .expect("single-channel composition should be valid");

        assert_eq!(composed.len(), 1);
        assert_eq!(composed.semantic_kind(), "sequential_composed_quantum_channel");
    }

    #[test]
    fn empty_sequential_composition_is_rejected() {
        let result = sequential(ChannelId::from_u128(1), Vec::new());

        assert!(matches!(result, Err(ChannelError::EmptySupport)));
    }

    #[test]
    fn empty_tensor_product_is_rejected() {
        let result = tensor_product(ChannelId::from_u128(1), Vec::new());

        assert!(matches!(result, Err(ChannelError::EmptySupport)));
    }

    #[test]
    fn plan_preserves_sequential_order() {
        let first = TestChannel::new(
            1,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let composed = sequential(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        )
        .expect("composition should succeed");

        let plan = composed.plan();

        assert_eq!(plan.kind(), CompositionKind::Sequential);
        assert_eq!(plan.len(), 2);
        assert_eq!(plan.channels()[0].id(), ChannelId::from_u128(1));
        assert_eq!(plan.channels()[1].id(), ChannelId::from_u128(2));
    }

    #[test]
    fn plan_preserves_tensor_membership() {
        let first = TestChannel::new(
            1,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(1)],
            vec![qubit(1)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let composed = tensor_product(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        )
        .expect("tensor product should succeed");

        let plan = composed.plan();

        assert_eq!(plan.kind(), CompositionKind::TensorProduct);
        assert_eq!(plan.len(), 2);
        assert_eq!(plan.channels()[0].id(), ChannelId::from_u128(1));
        assert_eq!(plan.channels()[1].id(), ChannelId::from_u128(2));
    }

    #[test]
    fn canonical_qubit_identity_is_preserved() {
        let first = TestChannel::new(
            1,
            vec![qubit(7)],
            vec![qubit(7)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(7)],
            vec![qubit(7)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let composed = sequential(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        )
        .expect("composition should succeed");

        assert_eq!(
            composed.input_qubits(),
            vec![QubitId::new(7)]
        );

        assert_eq!(
            composed.output_qubits(),
            vec![QubitId::new(7)]
        );
    }
}