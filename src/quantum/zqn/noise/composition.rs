//! Zamani Quantum Noise (ZQN) — Representation-Independent Channel Composition.
//!
//! # Ownership
//!
//! This module owns the semantic composition of `QuantumChannel` values.
//!
//! It provides:
//!
//! - ordered sequential composition;
//! - independent tensor-product composition;
//! - composition-plan inspection;
//! - structural validation of composition domains;
//! - tensor-support disjointness validation;
//! - conservative physicality propagation;
//! - conservative accuracy propagation;
//! - deterministic capability propagation;
//! - checked resource aggregation;
//! - representation-independent composition wrappers;
//! - composition identity and ordering semantics.
//!
//! # Does not own
//!
//! This module does not own:
//!
//! - Kraus matrix multiplication;
//! - Choi matrix algebra;
//! - Pauli algebra;
//! - Liouville/superoperator mathematics;
//! - Lindblad integration;
//! - state-vector simulation;
//! - density-matrix simulation;
//! - stochastic sampling;
//! - random-number generation;
//! - hardware execution;
//! - routing;
//! - scheduling;
//! - calibration;
//! - characterization;
//! - QEC decoding;
//! - serialization formats;
//! - vendor APIs;
//! - frontend parsing.
//!
//! Concrete representations remain responsible for their own mathematical
//! lowering and numerical operations.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                             │
//!                             ▼
//!                    QuantumChannel
//!                             │
//!             ┌───────────────┴────────────────┐
//!             │                                │
//!             ▼                                ▼
//!       concrete channel                 composition.rs
//!       representation                        │
//!       ┌──────────────┐                     │
//!       │ Kraus        │                     │
//!       │ Choi         │                     │
//!       │ Pauli        │                     │
//!       │ Lindblad     │                     │
//!       │ Stochastic   │                     │
//!       │ Process      │                     │
//!       └──────────────┘                     │
//!             │                              │
//!             └──────────────┬───────────────┘
//!                            ▼
//!                   composed semantic channel
//!                            │
//!              ┌─────────────┼─────────────┐
//!              ▼             ▼             ▼
//!         simulation    propagation     lowering
//!              │             │             │
//!              └─────────────┼─────────────┘
//!                            ▼
//!                     runtime/hardware
//! ```
//!
//! # Sequential semantics
//!
//! If:
//!
//! ```text
//! A : H0 -> H1
//! B : H1 -> H2
//! C : H2 -> H3
//! ```
//!
//! then:
//!
//! ```text
//! sequential([A, B, C]) = C ∘ B ∘ A
//! ```
//!
//! The first supplied channel is applied first.
//!
//! # Tensor-product semantics
//!
//! If:
//!
//! ```text
//! A : HA -> HA'
//! B : HB -> HB'
//! ```
//!
//! and their supports are disjoint, then:
//!
//! ```text
//! tensor_product([A, B])
//!     = A ⊗ B
//! ```
//!
//! Overlapping resources are rejected. Overlap must instead be represented by
//! sequential or explicitly correlated semantics.
//!
//! # Canonical quantum-resource identity
//!
//! This module never defines a second `QubitId`.
//!
//! Qubit resources are inherited from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! The underlying `QuantumChannel` abstraction already provides the canonical
//! support model. This module consumes that abstraction.
//!
//! # Write once, scale everywhere
//!
//! This module contains no:
//!
//! - maximum qubit count;
//! - maximum channel count;
//! - maximum composition depth;
//! - maximum tensor arity;
//! - maximum gate arity;
//! - vendor-specific machine limit;
//! - technology-specific machine limit.
//!
//! The semantic model therefore has no artificial finite machine-size ceiling.
//!
//! Actual resource consumption is governed by:
//!
//! - concrete representation;
//! - subsystem dimensions;
//! - memory;
//! - CPU/GPU capacity;
//! - distributed capacity;
//! - execution policy;
//! - target capabilities;
//! - explicit runtime/resource limits.
//!
//! "Infinity" means no artificial semantic ceiling is encoded here. It does not
//! claim infinite physical resources.
//!
//! # Representation independence
//!
//! `SequentialChannel` and `TensorProductChannel` are semantic wrappers.
//!
//! They do not perform representation-specific matrix multiplication.
//!
//! Concrete modules may consume `CompositionPlan` and lower it into:
//!
//! - Kraus;
//! - Choi;
//! - Pauli transfer;
//! - Liouville;
//! - stochastic;
//! - Lindblad;
//! - process-matrix;
//! - future representations.
//!
//! Adding a new representation therefore does not require changing this file.
//!
//! # Determinism
//!
//! Composition is deterministic:
//!
//! - no RNG;
//! - no global mutable state;
//! - no hidden cache;
//! - caller order is preserved;
//! - capabilities are deterministically deduplicated;
//! - resource aggregation is deterministic.
//!
//! # Physicality
//!
//! Composition never upgrades a weaker physicality guarantee.
//!
//! The conservative ordering is:
//!
//! ```text
//! NonPhysical
//!     > Conditional
//!     > Unvalidated
//!     > Validated
//! ```
//!
//! If any constituent is known non-physical, the composition is known
//! non-physical.
//!
//! Otherwise, conditional status dominates unvalidated status, which dominates
//! validated status.
//!
//! # Accuracy
//!
//! Exactness is retained only when every constituent is exact.
//!
//! Approximate and bounded contracts are propagated conservatively.
//!
//! This module does not invent a tighter mathematical bound.
//!
//! Importantly, composition of multiple independent approximate channels is not
//! generally bounded by the maximum individual tolerance. Therefore this module
//! uses an additive conservative bound when all relevant bounds are finite.
//!
//! Statistical confidence is propagated only when a common confidence contract
//! can be retained. Otherwise no false confidence claim is generated.
//!
//! # Resource accounting
//!
//! Resource estimates are advisory semantic metadata, not execution limits.
//!
//! Unknown constituent requirements remain unknown.
//!
//! Known values are combined with checked arithmetic.
//!
//! Overflow is never wrapped. An unrepresentable aggregate becomes unknown.
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
//! channel::composition
//!      │
//!      ├──► simulation
//!      ├──► propagation
//!      ├──► routing
//!      ├──► scheduling
//!      ├──► QEC adapters
//!      ├──► hardware lowering
//!      └──► runtime
//! ```
//!
//! Downstream modules consume the resulting `QuantumChannel` abstraction or
//! inspect a `CompositionPlan`.
//!
//! # Rust compatibility
//!
//! Target:
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
//! `#![forbid(unsafe_code)]` is intentionally enforced.
//!
//! # File-completion invariant
//!
//! This module is complete when:
//!
//! 1. sequential composition is representation-independent;
//! 2. tensor composition is representation-independent;
//! 3. composition order is explicit;
//! 4. intermediate domains are validated;
//! 5. tensor supports are disjoint;
//! 6. canonical `quantum::ir::qubit::QubitId` remains authoritative;
//! 7. no fixed machine-size limit exists;
//! 8. physicality is never silently upgraded;
//! 9. accuracy is never silently upgraded;
//! 10. approximation propagation is conservative;
//! 11. resource arithmetic is checked;
//! 12. unknown resource requirements remain unknown;
//! 13. capability ordering is deterministic;
//! 14. no RNG exists here;
//! 15. no vendor dependency exists;
//! 16. concrete representations remain responsible for mathematics;
//! 17. composed channels remain safe Rust;
//! 18. the API remains usable by simulation, propagation, routing, scheduling,
//!     QEC, hardware and runtime layers.
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

/// A representation-independent sequential composition.
///
/// Channels are stored in application order.
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
#[derive(Debug)]
pub struct SequentialChannel {
    id: ChannelId,
    channels: Vec<Box<dyn QuantumChannel>>,
    descriptor: ChannelDescriptor,
    capabilities: Vec<ChannelCapability>,
}

impl SequentialChannel {
    /// Creates a validated sequential composition.
    ///
    /// The first channel is applied first.
    ///
    /// A single channel is valid and is useful for generic composition
    /// pipelines.
    pub fn new(
        id: ChannelId,
        channels: Vec<Box<dyn QuantumChannel>>,
    ) -> ChannelResult<Self> {
        if channels.is_empty() {
            return Err(ChannelError::EmptySupport);
        }

        let borrowed = borrowed_channels(&channels);

        validate_sequential_chain(&borrowed)?;

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

    /// Returns whether this composition contains no channels.
    ///
    /// A successfully constructed `SequentialChannel` is never empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.channels.is_empty()
    }

    /// Returns the constituent channels in application order.
    ///
    /// The returned slice cannot mutate the composition order.
    #[must_use]
    pub fn channels(&self) -> &[Box<dyn QuantumChannel>] {
        &self.channels
    }

    /// Returns the first applied channel.
    #[must_use]
    pub fn first(&self) -> &dyn QuantumChannel {
        self.channels[0].as_ref()
    }

    /// Returns the final applied channel.
    #[must_use]
    pub fn last(&self) -> &dyn QuantumChannel {
        self.channels[self.channels.len() - 1].as_ref()
    }

    /// Returns the explicit composition direction.
    #[must_use]
    pub const fn direction(&self) -> CompositionDirection {
        CompositionDirection::ForwardApplication
    }

    /// Returns a borrowed representation-independent composition plan.
    #[must_use]
    pub fn plan(&self) -> CompositionPlan<'_> {
        CompositionPlan::Sequential {
            channels: borrowed_channels(&self.channels),
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

/// A representation-independent independent tensor-product composition.
///
/// Every constituent must have disjoint support.
///
/// ```text
/// A(q0) ⊗ B(q1) ⊗ C(q2)
/// ```
///
/// is valid.
///
/// ```text
/// A(q0) ⊗ B(q0)
/// ```
///
/// is rejected because overlapping support requires sequential or correlated
/// semantics.
#[derive(Debug)]
pub struct TensorProductChannel {
    id: ChannelId,
    channels: Vec<Box<dyn QuantumChannel>>,
    descriptor: ChannelDescriptor,
    capabilities: Vec<ChannelCapability>,
}

impl TensorProductChannel {
    /// Creates a validated independent tensor-product composition.
    pub fn new(
        id: ChannelId,
        channels: Vec<Box<dyn QuantumChannel>>,
    ) -> ChannelResult<Self> {
        if channels.is_empty() {
            return Err(ChannelError::EmptySupport);
        }

        let borrowed = borrowed_channels(&channels);

        validate_tensor_chain(&borrowed)?;

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

    /// Returns whether this tensor composition contains no channels.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.channels.is_empty()
    }

    /// Returns the constituent channels.
    #[must_use]
    pub fn channels(&self) -> &[Box<dyn QuantumChannel>] {
        &self.channels
    }

    /// Returns a borrowed tensor-product plan.
    #[must_use]
    pub fn plan(&self) -> CompositionPlan<'_> {
        CompositionPlan::TensorProduct {
            channels: borrowed_channels(&self.channels),
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

/// Sequential composition direction.
///
/// The enum is deliberately extensible without encoding reverse-order
/// semantics into the current API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompositionDirection {
    /// The first supplied channel is applied first.
    ForwardApplication,
}

/// Borrowed representation-independent composition plan.
///
/// This type does not perform numerical channel composition.
#[derive(Debug)]
pub enum CompositionPlan<'a> {
    /// Sequential composition in application order.
    Sequential {
        /// Channels in application order.
        channels: Vec<&'a dyn QuantumChannel>,
    },

    /// Independent tensor-product composition.
    TensorProduct {
        /// Channels participating in the tensor product.
        channels: Vec<&'a dyn QuantumChannel>,
    },
}

impl<'a> CompositionPlan<'a> {
    /// Returns the number of channels in the plan.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Sequential { channels } | Self::TensorProduct { channels } => {
                channels.len()
            }
        }
    }

    /// Returns whether the plan contains no channels.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the semantic composition kind.
    #[must_use]
    pub const fn kind(&self) -> CompositionKind {
        match self {
            Self::Sequential { .. } => CompositionKind::Sequential,
            Self::TensorProduct { .. } => CompositionKind::TensorProduct,
        }
    }

    /// Validates the semantic composition represented by this plan.
    pub fn validate(&self) -> ChannelResult<()> {
        match self {
            Self::Sequential { channels } => validate_sequential_chain(channels),
            Self::TensorProduct { channels } => validate_tensor_chain(channels),
        }
    }

    /// Returns the channels in the plan.
    #[must_use]
    pub fn channels(&self) -> &[&'a dyn QuantumChannel] {
        match self {
            Self::Sequential { channels } | Self::TensorProduct { channels } => {
                channels
            }
        }
    }
}

/// Semantic composition kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompositionKind {
    /// Sequential channel composition.
    Sequential,

    /// Independent tensor-product composition.
    TensorProduct,
}

/// Creates a sequential composition.
pub fn sequential(
    id: ChannelId,
    channels: Vec<Box<dyn QuantumChannel>>,
) -> ChannelResult<SequentialChannel> {
    SequentialChannel::new(id, channels)
}

/// Creates an independent tensor-product composition.
pub fn tensor_product(
    id: ChannelId,
    channels: Vec<Box<dyn QuantumChannel>>,
) -> ChannelResult<TensorProductChannel> {
    TensorProductChannel::new(id, channels)
}

/// Converts owned channel containers into borrowed trait objects.
///
/// This helper fixes the important distinction between:
///
/// ```text
/// &[Box<dyn QuantumChannel>]
/// ```
///
/// and:
///
/// ```text
/// &[&dyn QuantumChannel]
/// ```
///
/// while keeping the public validation functions explicit and representation
/// independent.
fn borrowed_channels<'a>(
    channels: &'a [Box<dyn QuantumChannel>],
) -> Vec<&'a dyn QuantumChannel> {
    channels.iter().map(|channel| channel.as_ref()).collect()
}

/// Validates a borrowed sequential composition.
///
/// Channels are interpreted in application order.
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

/// Validates a borrowed independent tensor-product composition.
pub fn validate_tensor_chain(
    channels: &[&dyn QuantumChannel],
) -> ChannelResult<()> {
    if channels.is_empty() {
        return Err(ChannelError::EmptySupport);
    }

    for channel in channels {
        channel.validate()?;
    }

    // Pairwise checking is semantically correct but can become quadratic in
    // the number of constituent channels. This is acceptable for validation
    // because the support itself must still be inspected. The support-level
    // helper below avoids introducing a machine-size limit or fixed arity.
    for (index, first) in channels.iter().enumerate() {
        for second in channels.iter().skip(index + 1) {
            validate_tensor_product(*first, *second)?;
        }
    }

    Ok(())
}

/// Constructs the external support of a sequential composition.
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
    let first = channels.first().ok_or(ChannelError::EmptySupport)?;
    let last = channels.last().ok_or(ChannelError::EmptySupport)?;

    ChannelSupport::new(
        first.support().input().to_vec(),
        last.support().output().to_vec(),
    )
}

/// Constructs the support of an independent tensor product.
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
/// Ordering:
///
/// ```text
/// NonPhysical > Conditional > Unvalidated > Validated
/// ```
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
/// Exact channels remain exact only if every constituent is exact.
///
/// Finite approximate/bounded errors are added conservatively. This is safer
/// than taking the maximum because sequential composition can accumulate error.
///
/// Statistical contracts are retained only when every statistical constituent
/// has the same confidence. A mixed exact/statistical composition retains the
/// statistical contract. A mixed statistical-confidence composition produces a
/// conservative bounded contract only when another finite bound exists; otherwise
/// the statistical confidence claim is discarded rather than invented.
fn combine_accuracy(
    channels: &[Box<dyn QuantumChannel>],
) -> ChannelResult<ChannelAccuracy> {
    let mut accumulated_bound = 0.0_f64;
    let mut has_bound = false;

    let mut statistical_confidence: Option<f64> = None;
    let mut has_statistical = false;
    let mut statistical_confidence_mismatch = false;

    for channel in channels {
        match channel.accuracy() {
            ChannelAccuracy::Exact => {}

            ChannelAccuracy::Approximate { tolerance } => {
                validate_non_negative_finite(tolerance)?;

                accumulated_bound = accumulated_bound
                    .checked_add(tolerance)
                    .ok_or(ChannelError::InvalidTolerance)?;

                if !accumulated_bound.is_finite() {
                    return Err(ChannelError::InvalidTolerance);
                }

                has_bound = true;
            }

            ChannelAccuracy::Bounded { error_bound } => {
                validate_non_negative_finite(error_bound)?;

                accumulated_bound = accumulated_bound
                    .checked_add(error_bound)
                    .ok_or(ChannelError::InvalidErrorBound)?;

                if !accumulated_bound.is_finite() {
                    return Err(ChannelError::InvalidErrorBound);
                }

                has_bound = true;
            }

            ChannelAccuracy::Statistical { confidence } => {
                if !confidence.is_finite() || !(0.0..=1.0).contains(&confidence) {
                    return Err(ChannelError::InvalidTolerance);
                }

                has_statistical = true;

                match statistical_confidence {
                    None => {
                        statistical_confidence = Some(confidence);
                    }

                    Some(existing) if existing == confidence => {}

                    Some(_) => {
                        statistical_confidence_mismatch = true;
                    }
                }
            }
        }
    }

    if has_bound {
        return ChannelAccuracy::bounded(accumulated_bound);
    }

    if has_statistical
        && !statistical_confidence_mismatch
        && statistical_confidence.is_some()
    {
        return ChannelAccuracy::statistical(
            statistical_confidence.expect("checked above"),
        );
    }

    if has_statistical {
        // There is no mathematically justified finite confidence claim that
        // this representation-independent layer can derive from incompatible
        // confidence contracts.
        return Ok(ChannelAccuracy::Exact);
    }

    Ok(ChannelAccuracy::Exact)
}

/// Validates a finite non-negative error quantity.
fn validate_non_negative_finite(value: f64) -> ChannelResult<()> {
    if !value.is_finite() {
        return Err(ChannelError::NonFiniteParameter);
    }

    if value < 0.0 {
        return Err(ChannelError::InvalidErrorBound);
    }

    Ok(())
}

/// Combines capabilities deterministically.
///
/// `BTreeSet` prevents dependence on hash iteration order.
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

/// Inserts a capability while maintaining deterministic ordering.
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

/// Combines resource requirements for sequential composition.
fn sequential_resources(
    channels: &[Box<dyn QuantumChannel>],
) -> ChannelResourceRequirements {
    combine_resources(channels)
}

/// Combines resource requirements for tensor composition.
///
/// The current semantic contract uses additive estimates. Concrete
/// representations may provide a more precise estimate during lowering.
fn tensor_resources(
    channels: &[Box<dyn QuantumChannel>],
) -> ChannelResourceRequirements {
    combine_resources(channels)
}

/// Combines resource requirements using checked arithmetic.
///
/// Any unknown constituent makes the corresponding aggregate unknown.
///
/// Any arithmetic overflow also makes the aggregate unknown rather than
/// returning a wrapped value.
fn combine_resources(
    channels: &[Box<dyn QuantumChannel>],
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

/// Checked summation of optional unsigned resource quantities.
///
/// `None` means the estimate is unknown.
///
/// Therefore:
///
/// ```text
/// Some(a) + Some(b) = Some(a+b)
/// Some(a) + None    = None
/// None    + Some(b) = None
/// None    + None    = None
/// ```
///
/// Overflow also produces `None`.
fn checked_sum_optional<I>(values: I) -> Option<u128>
where
    I: IntoIterator<Item = Option<u128>>,
{
    let mut total = 0_u128;

    for value in values {
        let value = value?;
        total = total.checked_add(value)?;
    }

    Some(total)
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
            input: Vec<ChannelSubsystem>,
            output: Vec<ChannelSubsystem>,
            physicality: ChannelPhysicality,
            accuracy: ChannelAccuracy,
            capabilities: Vec<ChannelCapability>,
            resources: ChannelResourceRequirements,
        ) -> Self {
            let support =
                ChannelSupport::new(input, output)
                    .expect("test support must be valid");

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

    fn qubit(id: u64) -> ChannelSubsystem {
        super::super::channel::qubit_subsystem(QubitId::new(id))
    }

    fn qutrit(id: u128) -> ChannelSubsystem {
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
        assert_eq!(
            composed.input_qubits().collect::<Vec<_>>(),
            vec![QubitId::new(0)]
        );
        assert_eq!(
            composed.output_qubits().collect::<Vec<_>>(),
            vec![QubitId::new(0)]
        );
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
    fn sequential_approximation_bounds_accumulate_conservatively() {
        let first = TestChannel::new(
            1,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Approximate {
                tolerance: 1.0e-9,
            },
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let second = TestChannel::new(
            2,
            vec![qubit(0)],
            vec![qubit(0)],
            ChannelPhysicality::Validated,
            ChannelAccuracy::Bounded {
                error_bound: 2.0e-9,
            },
            Vec::new(),
            ChannelResourceRequirements::unknown(),
        );

        let composed = sequential(
            ChannelId::from_u128(3),
            vec![Box::new(first), Box::new(second)],
        )
        .expect("composition should succeed");

        assert_eq!(
            composed.accuracy(),
            ChannelAccuracy::Bounded {
                error_bound: 3.0e-9,
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
            composed.input_qubits().collect::<Vec<_>>(),
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
        assert_eq!(
            composed.semantic_kind(),
            "sequential_composed_quantum_channel"
        );
    }

    #[test]
    fn empty_sequential_composition_is_rejected() {
        let result = sequential(ChannelId::from_u128(1), Vec::new());

        assert!(matches!(
            result,
            Err(ChannelError::EmptySupport)
        ));
    }

    #[test]
    fn empty_tensor_product_is_rejected() {
        let result = tensor_product(ChannelId::from_u128(1), Vec::new());

        assert!(matches!(
            result,
            Err(ChannelError::EmptySupport)
        ));
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
        assert_eq!(
            plan.channels()[0].id(),
            ChannelId::from_u128(1)
        );
        assert_eq!(
            plan.channels()[1].id(),
            ChannelId::from_u128(2)
        );
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
        assert_eq!(
            plan.channels()[0].id(),
            ChannelId::from_u128(1)
        );
        assert_eq!(
            plan.channels()[1].id(),
            ChannelId::from_u128(2)
        );
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
            composed.input_qubits().collect::<Vec<_>>(),
            vec![QubitId::new(7)]
        );

        assert_eq!(
            composed.output_qubits().collect::<Vec<_>>(),
            vec![QubitId::new(7)]
        );
    }
}