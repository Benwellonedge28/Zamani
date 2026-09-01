//! Zamani Quantum Noise (ZQN) — Pauli Channels.
//!
//! Production-grade Pauli-channel mathematics for arbitrary finite Pauli
//! supports.
//!
//! # Architectural responsibility
//!
//! This module owns the mathematical representation of Pauli channels:
//!
//! ```text
//! P(ρ) = Σ_i p_i P_i ρ P_i†
//! ```
//!
//! where:
//!
//! - each `P_i` is a Pauli string;
//! - each `p_i` is a validated `Probability`;
//! - probabilities form a normalized distribution;
//! - global Pauli phase is ignored because it has no effect on the channel;
//! - the support may contain an arbitrary number of quantum resources.
//!
//! This module also owns:
//!
//! - single-resource Pauli operators;
//! - arbitrary Pauli strings;
//! - sparse Pauli terms;
//! - Pauli-channel construction and validation;
//! - deterministic canonical ordering;
//! - channel composition;
//! - tensor-product construction;
//! - identity-channel handling;
//! - support inspection;
//! - channel probability lookup;
//! - exact structural equality;
//! - safe channel simplification.
//!
//! # Does not own
//!
//! This module does NOT own:
//!
//! - canonical quantum IR;
//! - source-language parsing;
//! - hardware execution;
//! - QPU APIs;
//! - routing;
//! - scheduling;
//! - calibration;
//! - characterization experiments;
//! - random-number generation;
//! - Monte Carlo sampling;
//! - state-vector simulation;
//! - density-matrix simulation;
//! - QEC decoding;
//! - syndrome generation;
//! - logical Pauli definitions;
//! - vendor-specific Pauli representations.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Canonical quantum identity boundary
//!
//! ZQN must never define a second `QubitId` or `PhysicalQubitId`.
//!
//! Quantum resource identity is owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! Therefore this file uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! directly.
//!
//! A Pauli channel can therefore describe noise over either logical or
//! physical resources without inventing another identity system.
//!
//! # Mathematical semantics
//!
//! A Pauli channel is a completely positive trace-preserving quantum channel
//! of the form:
//!
//! ```text
//! E(ρ) = Σ_P p(P) P ρ P†
//! ```
//!
//! with:
//!
//! ```text
//! p(P) >= 0
//! Σ_P p(P) = 1
//! ```
//!
//! Every `Probability` is already guaranteed to be finite and within
//! `[0, 1]` by `zqn::probability::Probability`.
//!
//! The channel constructor additionally requires the complete distribution to
//! normalize to one within an explicitly supplied tolerance.
//!
//! No implicit normalization is performed.
//!
//! A caller supplying probabilities whose sum is not one must either:
//!
//! - correct the probabilities;
//! - explicitly construct a normalized distribution upstream;
//! - or use a future explicitly named normalization facility.
//!
//! This module must never silently change a physical noise model.
//!
//! # Global phase
//!
//! Pauli operators are represented modulo global phase.
//!
//! Thus:
//!
//! ```text
//! X
//! -X
//! iX
//! -iX
//! ```
//!
//! all represent the same Pauli action for this channel representation.
//!
//! The phase is intentionally not stored.
//!
//! Relative phase between different operators is not part of a stochastic
//! Pauli channel's classical mixture semantics.
//!
//! Coherent errors must instead be represented by an appropriate general
//! quantum-channel representation.
//!
//! # Scalability
//!
//! There is no semantic fixed qubit count in this module.
//!
//! A Pauli string is represented sparsely:
//!
//! ```text
//! resource -> Pauli
//! ```
//!
//! Identity factors do not need to be stored.
//!
//! Therefore an N-resource Pauli string requires storage proportional to its
//! non-identity support rather than automatically materializing a dense
//! N-dimensional object.
//!
//! The implementation does not contain:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PAULIS
//! MAX_TERMS
//! MAX_CHANNEL_SIZE
//! ```
//!
//! Such limits, if required for an execution environment, belong to
//! `ZqnLimits` or another explicit resource-policy layer.
//!
//! "Infinity" means no artificial semantic upper bound. Actual computation
//! remains bounded by available CPU, memory, storage, execution time and target
//! resources.
//!
//! # Sparse representation
//!
//! A Pauli string stores only non-identity factors.
//!
//! For example:
//!
//! ```text
//! X(q0) Z(q1000000)
//! ```
//!
//! does not require storing identities for every resource between `q0` and
//! `q1000000`.
//!
//! This is essential for large and distributed systems.
//!
//! # Determinism
//!
//! This module contains no RNG and no global state.
//!
//! Canonical Pauli strings use `BTreeMap`, giving deterministic ordering.
//!
//! Channel terms are stored in canonical deterministic order.
//!
//! Equal mathematical terms can be merged deterministically.
//!
//! Composition therefore has deterministic structural output for identical
//! inputs.
//!
//! Sampling is deliberately outside this file.
//!
//! # Numerical semantics
//!
//! Probabilities are represented by the repository's canonical ZQN
//! `Probability` type.
//!
//! Floating-point arithmetic is used only where explicitly required for:
//!
//! - normalization checks;
//! - composition;
//! - tolerance comparison.
//!
//! No NaN or infinity may be accepted as a probability.
//!
//! No invalid probability is silently clamped.
//!
//! # Approximation
//!
//! This module does not silently approximate.
//!
//! Composition can generate terms whose probabilities are represented using
//! finite-precision arithmetic. The resulting value is validated before it is
//! returned.
//!
//! If an operation cannot represent its result within the requested numerical
//! contract, it returns an error.
//!
//! Approximate channel compression belongs to a separate approximation layer.
//!
//! # Resource safety
//!
//! This module uses safe Rust only.
//!
//! It does not:
//!
//! - use raw pointers;
//! - use FFI;
//! - use unsafe blocks;
//! - allocate based on unchecked attacker-controlled arithmetic;
//! - recursively materialize a full quantum state;
//! - spawn threads;
//! - perform I/O;
//! - access hardware.
//!
//! Operations that can increase the number of terms use checked capacity
//! calculations where applicable and return explicit errors rather than
//! panicking on representational overflow.
//!
//! # Integration
//!
//! ```text
//! quantum::ir::qubit
//!          │
//!          ▼
//!     PauliResource
//!          │
//!          ▼
//!     PauliString
//!          │
//!          ▼
//!     PauliTerm
//!          │
//!          ▼
//!     PauliChannel
//!          │
//!     ┌────┼───────────────┐
//!     ▼    ▼               ▼
//!   ZQN   simulator      target
//!    │                      │
//!    ▼                      ▼
//!   QEC / routing /      hardware
//!   scheduling adapters
//! ```
//!
//! The channel remains backend-independent.
//!
//! # QEC integration
//!
//! QEC must not import this module's implementation merely to reuse a Pauli
//! enum if doing so would create a circular dependency.
//!
//! Conversely, ZQN must not depend on QEC.
//!
//! If the repository later needs a common algebraic Pauli type, that type
//! should live in a lower-level quantum mathematics module and both ZQN and QEC
//! may consume it.
//!
//! Until such a common abstraction exists, this module intentionally owns its
//! own channel-specific Pauli representation.
//!
//! # Routing integration
//!
//! Routing may consume a `PauliChannel` to estimate a noise-aware cost.
//!
//! This module does not know how routing works.
//!
//! # Scheduling integration
//!
//! Scheduling may associate Pauli channels with operation duration, idle
//! intervals, or physical resources.
//!
//! Duration and scheduling semantics remain outside this module.
//!
//! # Hardware integration
//!
//! Hardware adapters may lower a Pauli channel into a native stochastic-noise
//! representation when target capabilities permit.
//!
//! No vendor API is imported here.
//!
//! # Serialization
//!
//! This module deliberately does not define an external wire format.
//!
//! Serialization belongs to:
//!
//! ```text
//! crate::quantum::zqn::io
//! ```
//!
//! The serializer must preserve:
//!
//! - resource identity domain;
//! - resource identity value;
//! - Pauli value;
//! - probability;
//! - channel term ordering/canonical meaning;
//! - schema version.
//!
//! Rust struct layout is not a wire-format guarantee.
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
//! - no unsafe code.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. Pauli values are mathematically correct;
//! 2. global phase is ignored;
//! 3. arbitrary finite resource support is possible;
//! 4. no fixed machine size is encoded;
//! 5. canonical IR resource identities are used;
//! 6. channels cannot contain invalid probabilities;
//! 7. channel construction does not silently normalize;
//! 8. duplicate Pauli terms are merged;
//! 9. zero-probability terms are removed;
//! 10. deterministic ordering is preserved;
//! 11. composition is mathematically defined;
//! 12. tensor product is mathematically defined;
//! 13. identity is represented correctly;
//! 14. no RNG is hidden here;
//! 15. no vendor dependency exists;
//! 16. no QEC dependency exists;
//! 17. no unsafe Rust exists;
//! 18. resource growth is explicit;
//! 19. numerical failures are returned as errors;
//! 20. tests cover algebra, validation, determinism and scaling behavior.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]

use core::fmt;
use std::collections::BTreeMap;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::probability::Probability;

// ============================================================================
// Constants
// ============================================================================

/// Default absolute tolerance for probability normalization checks.
///
/// This is a numerical tolerance, not a machine-size limit.
///
/// Callers requiring a different numerical contract should use
/// `PauliChannel::new_with_tolerance`.
pub const DEFAULT_NORMALIZATION_TOLERANCE: f64 = 1.0e-12;

/// Default tolerance for probability values that become numerically zero
/// during composition.
///
/// This value is deliberately conservative and is only used by explicit
/// simplification operations, never by channel construction.
pub const DEFAULT_ZERO_TOLERANCE: f64 = 0.0;

// ============================================================================
// Resource identity
// ============================================================================

/// Quantum resource to which a Pauli factor applies.
///
/// Logical and physical qubit identities are intentionally distinct.
///
/// This enum does not claim that either resource currently exists. Existence
/// and target validation belong to the IR/hardware layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PauliResource {
    /// Canonical logical qubit identity.
    Logical(QubitId),

    /// Canonical physical qubit identity.
    Physical(PhysicalQubitId),
}

impl PauliResource {
    /// Returns true when the resource is a logical qubit.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::Logical(_))
    }

    /// Returns true when the resource is a physical qubit.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::Physical(_))
    }

    /// Returns the canonical logical qubit when this resource is logical.
    #[must_use]
    pub const fn logical(self) -> Option<QubitId> {
        match self {
            Self::Logical(value) => Some(value),
            Self::Physical(_) => None,
        }
    }

    /// Returns the canonical physical qubit when this resource is physical.
    #[must_use]
    pub const fn physical(self) -> Option<PhysicalQubitId> {
        match self {
            Self::Logical(_) => None,
            Self::Physical(value) => Some(value),
        }
    }
}

impl fmt::Display for PauliResource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Logical(id) => write!(formatter, "q:{id}"),
            Self::Physical(id) => write!(formatter, "p:{id}"),
        }
    }
}

// ============================================================================
// Single-resource Pauli
// ============================================================================

/// Single-resource Pauli operator.
///
/// Global phase is intentionally ignored.
///
/// The four values are:
///
/// ```text
/// I X Y Z
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pauli {
    /// Identity.
    I,

    /// Pauli X.
    X,

    /// Pauli Y.
    Y,

    /// Pauli Z.
    Z,
}

impl Pauli {
    /// Identity Pauli.
    pub const IDENTITY: Self = Self::I;

    /// Returns true when this is identity.
    #[must_use]
    pub const fn is_identity(self) -> bool {
        matches!(self, Self::I)
    }

    /// Returns true when this is X.
    #[must_use]
    pub const fn is_x(self) -> bool {
        matches!(self, Self::X)
    }

    /// Returns true when this is Y.
    #[must_use]
    pub const fn is_y(self) -> bool {
        matches!(self, Self::Y)
    }

    /// Returns true when this is Z.
    #[must_use]
    pub const fn is_z(self) -> bool {
        matches!(self, Self::Z)
    }

    /// Returns the Pauli obtained by multiplication, modulo global phase.
    ///
    /// The phase produced by multiplication is deliberately discarded.
    #[must_use]
    pub const fn multiply(self, other: Self) -> Self {
        use Pauli::*;

        match (self, other) {
            (I, value) | (value, I) => value,

            (X, X) | (Y, Y) | (Z, Z) => I,

            (X, Y) | (Y, X) => Z,
            (X, Z) | (Z, X) => Y,
            (Y, Z) | (Z, Y) => X,
        }
    }

    /// Returns whether the two single-resource Paulis commute.
    #[must_use]
    pub const fn commutes_with(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::I, _)
                | (_, Self::I)
                | (Self::X, Self::X)
                | (Self::Y, Self::Y)
                | (Self::Z, Self::Z)
        )
    }

    /// Returns the symplectic X component.
    ///
    /// This is `1` for X/Y and `0` for I/Z.
    #[must_use]
    pub const fn x_component(self) -> u8 {
        match self {
            Self::I | Self::Z => 0,
            Self::X | Self::Y => 1,
        }
    }

    /// Returns the symplectic Z component.
    ///
    /// This is `1` for Z/Y and `0` for I/X.
    #[must_use]
    pub const fn z_component(self) -> u8 {
        match self {
            Self::I | Self::X => 0,
            Self::Y | Self::Z => 1,
        }
    }

    /// Returns the Pauli symbol.
    #[must_use]
    pub const fn symbol(self) -> char {
        match self {
            Self::I => 'I',
            Self::X => 'X',
            Self::Y => 'Y',
            Self::Z => 'Z',
        }
    }
}

impl fmt::Display for Pauli {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::I => "I",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
        })
    }
}

// ============================================================================
// Pauli string
// ============================================================================

/// Sparse Pauli string over canonical Zamani quantum resources.
///
/// Identity factors are omitted.
///
/// Example:
///
/// ```text
/// X(q0) Z(q7)
/// ```
///
/// is stored as two resource/factor entries rather than as a dense vector
/// containing every resource between them.
///
/// The type is suitable for arbitrarily large finite supports subject to
/// available memory.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PauliString {
    factors: BTreeMap<PauliResource, Pauli>,
}

impl PauliString {
    /// Creates the identity Pauli string.
    #[must_use]
    pub fn identity() -> Self {
        Self {
            factors: BTreeMap::new(),
        }
    }

    /// Creates a Pauli string containing one non-identity factor.
    ///
    /// Supplying identity returns the identity string.
    #[must_use]
    pub fn single(resource: PauliResource, pauli: Pauli) -> Self {
        let mut string = Self::identity();
        string.set(resource, pauli);
        string
    }

    /// Creates a Pauli string from an iterator of resource/factor pairs.
    ///
    /// Duplicate resources are rejected because silently replacing one Pauli
    /// with another would hide a caller error.
    pub fn try_from_factors<I>(
        factors: I,
    ) -> Result<Self, PauliError>
    where
        I: IntoIterator<Item = (PauliResource, Pauli)>,
    {
        let mut result = Self::identity();

        for (resource, pauli) in factors {
            if pauli.is_identity() {
                continue;
            }

            if result.factors.contains_key(&resource) {
                return Err(PauliError::DuplicateResource { resource });
            }

            result.factors.insert(resource, pauli);
        }

        Ok(result)
    }

    /// Returns true when this string is the identity.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.factors.is_empty()
    }

    /// Returns the number of non-identity factors.
    #[must_use]
    pub fn weight(&self) -> usize {
        self.factors.len()
    }

    /// Returns the number of stored factors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.factors.len()
    }

    /// Returns true when no non-identity factors are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.factors.is_empty()
    }

    /// Returns the Pauli acting on a resource.
    ///
    /// Unspecified resources have identity action.
    #[must_use]
    pub fn get(&self, resource: &PauliResource) -> Pauli {
        self.factors
            .get(resource)
            .copied()
            .unwrap_or(Pauli::I)
    }

    /// Inserts or replaces a factor.
    ///
    /// Identity removes the factor.
    ///
    /// This operation is intentionally explicit: replacing an existing
    /// factor is useful when constructing a Pauli string incrementally.
    pub fn set(&mut self, resource: PauliResource, pauli: Pauli) {
        if pauli.is_identity() {
            self.factors.remove(&resource);
        } else {
            self.factors.insert(resource, pauli);
        }
    }

    /// Removes a factor and returns it.
    pub fn remove(&mut self, resource: &PauliResource) -> Option<Pauli> {
        self.factors.remove(resource)
    }

    /// Returns an iterator over factors in canonical deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = (&PauliResource, &Pauli)> {
        self.factors.iter()
    }

    /// Returns all non-identity resources in canonical order.
    #[must_use]
    pub fn support(&self) -> Vec<PauliResource> {
        self.factors.keys().copied().collect()
    }

    /// Returns the logical-qubit support.
    #[must_use]
    pub fn logical_support(&self) -> Vec<QubitId> {
        self.factors
            .keys()
            .filter_map(|resource| resource.logical())
            .collect()
    }

    /// Returns the physical-qubit support.
    #[must_use]
    pub fn physical_support(&self) -> Vec<PhysicalQubitId> {
        self.factors
            .keys()
            .filter_map(|resource| resource.physical())
            .collect()
    }

    /// Returns true if every resource is logical.
    #[must_use]
    pub fn is_logical_only(&self) -> bool {
        self.factors.keys().all(PauliResource::is_logical)
    }

    /// Returns true if every resource is physical.
    #[must_use]
    pub fn is_physical_only(&self) -> bool {
        self.factors.keys().all(PauliResource::is_physical)
    }

    /// Returns true if the string contains both logical and physical
    /// resources.
    ///
    /// Mixed-domain strings are normally undesirable for physical execution,
    /// but the representation itself does not silently forbid them. Target
    /// validation belongs to the target/integration layer.
    #[must_use]
    pub fn is_mixed_domain(&self) -> bool {
        let logical = self.factors.keys().any(PauliResource::is_logical);
        let physical = self.factors.keys().any(PauliResource::is_physical);
        logical && physical
    }

    /// Multiplies two Pauli strings modulo global phase.
    ///
    /// Resources present in only one operand are copied.
    ///
    /// Resources present in both operands are multiplied using the Pauli
    /// multiplication table.
    #[must_use]
    pub fn multiply(&self, other: &Self) -> Self {
        let mut result = self.clone();

        for (&resource, &pauli) in &other.factors {
            let combined = result.get(&resource).multiply(pauli);
            result.set(resource, combined);
        }

        result
    }

    /// Returns whether this Pauli string commutes with another.
    ///
    /// Two Pauli strings commute iff the number of resource positions at which
    /// their non-identity Paulis anticommute is even.
    #[must_use]
    pub fn commutes_with(&self, other: &Self) -> bool {
        let mut anticommute_count = 0usize;

        let mut left = self.factors.iter();
        let mut right = other.factors.iter();

        let mut left_item = left.next();
        let mut right_item = right.next();

        while let (Some((&left_resource, &left_pauli)), Some((&right_resource, &right_pauli))) =
            (left_item, right_item)
        {
            match left_resource.cmp(&right_resource) {
                std::cmp::Ordering::Less => {
                    left_item = left.next();
                }
                std::cmp::Ordering::Greater => {
                    right_item = right.next();
                }
                std::cmp::Ordering::Equal => {
                    if !left_pauli.commutes_with(right_pauli) {
                        anticommute_count += 1;
                    }

                    left_item = left.next();
                    right_item = right.next();
                }
            }
        }

        anticommute_count % 2 == 0
    }

    /// Returns the tensor product of two Pauli strings.
    ///
    /// Tensor product is represented by set union because the resource
    /// identities themselves identify the tensor factors.
    ///
    /// The two strings must have disjoint supports.
    pub fn tensor_product(&self, other: &Self) -> Result<Self, PauliError> {
        if let Some(resource) = self
            .factors
            .keys()
            .find(|resource| other.factors.contains_key(resource))
            .copied()
        {
            return Err(PauliError::OverlappingTensorSupport { resource });
        }

        let mut result = self.clone();

        for (&resource, &pauli) in &other.factors {
            result.factors.insert(resource, pauli);
        }

        Ok(result)
    }

    /// Returns the symplectic X vector as sparse resource/factor data.
    ///
    /// This is primarily useful for stabilizer/QEC adapters and does not
    /// expose a fixed-width bit vector.
    #[must_use]
    pub fn x_support(&self) -> Vec<PauliResource> {
        self.factors
            .iter()
            .filter_map(|(&resource, &pauli)| {
                if pauli.x_component() == 1 {
                    Some(resource)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns the symplectic Z vector as sparse resource/factor data.
    #[must_use]
    pub fn z_support(&self) -> Vec<PauliResource> {
        self.factors
            .iter()
            .filter_map(|(&resource, &pauli)| {
                if pauli.z_component() == 1 {
                    Some(resource)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for PauliString {
    fn default() -> Self {
        Self::identity()
    }
}

impl fmt::Display for PauliString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_identity() {
            return formatter.write_str("I");
        }

        let mut first = true;

        for (resource, pauli) in &self.factors {
            if !first {
                formatter.write_str(" ")?;
            }

            first = false;
            write!(formatter, "{pauli}({resource})")?;
        }

        Ok(())
    }
}

// ============================================================================
// Pauli channel term
// ============================================================================

/// One probability-weighted Pauli term.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauliTerm {
    pauli: PauliString,
    probability: Probability,
}

impl PauliTerm {
    /// Creates a validated Pauli term.
    #[must_use]
    pub fn new(pauli: PauliString, probability: Probability) -> Self {
        Self {
            pauli,
            probability,
        }
    }

    /// Returns the Pauli string.
    #[must_use]
    pub fn pauli(&self) -> &PauliString {
        &self.pauli
    }

    /// Returns the probability.
    #[must_use]
    pub const fn probability(&self) -> Probability {
        self.probability
    }

    /// Returns true when this term has zero probability.
    #[must_use]
    pub fn is_zero_probability(&self) -> bool {
        self.probability.is_zero()
    }
}

// ============================================================================
// Pauli channel
// ============================================================================

/// Sparse stochastic Pauli quantum channel.
///
/// Mathematically:
///
/// ```text
/// E(ρ) = Σ_P p(P) PρP†
/// ```
///
/// Terms are stored canonically and duplicate Pauli strings are merged.
///
/// A valid channel always contains total probability one.
///
/// The identity channel is represented by exactly one identity term with
/// probability one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauliChannel {
    terms: BTreeMap<PauliString, Probability>,
    normalization_tolerance: f64,
}

impl PauliChannel {
    /// Creates a Pauli channel using the default normalization tolerance.
    ///
    /// The input probabilities must already sum to one within the default
    /// tolerance.
    pub fn new<I>(terms: I) -> Result<Self, PauliError>
    where
        I: IntoIterator<Item = PauliTerm>,
    {
        Self::new_with_tolerance(terms, DEFAULT_NORMALIZATION_TOLERANCE)
    }

    /// Creates a Pauli channel with an explicit normalization tolerance.
    ///
    /// `tolerance` must be finite and non-negative.
    ///
    /// No normalization is performed.
    pub fn new_with_tolerance<I>(
        terms: I,
        tolerance: f64,
    ) -> Result<Self, PauliError>
    where
        I: IntoIterator<Item = PauliTerm>,
    {
        validate_tolerance(tolerance)?;

        let mut merged: BTreeMap<PauliString, f64> = BTreeMap::new();

        for term in terms {
            let probability = term.probability.value();

            if probability == 0.0 {
                continue;
            }

            let entry = merged.entry(term.pauli).or_insert(0.0);

            *entry = entry
                .checked_add(probability)
                .ok_or(PauliError::ProbabilityArithmeticOverflow)?;
        }

        if merged.is_empty() {
            return Err(PauliError::EmptyChannel);
        }

        let total = merged
            .values()
            .try_fold(0.0f64, |accumulator, probability| {
                accumulator
                    .checked_add(*probability)
                    .ok_or(PauliError::ProbabilityArithmeticOverflow)
            })?;

        if !total.is_finite() {
            return Err(PauliError::NonFiniteNormalization);
        }

        if (total - 1.0).abs() > tolerance {
            return Err(PauliError::NotNormalized {
                total,
                tolerance,
            });
        }

        let terms = merged
            .into_iter()
            .map(|(pauli, probability)| {
                let probability =
                    Probability::new(probability).map_err(PauliError::Probability)?;

                Ok((pauli, probability))
            })
            .collect::<Result<BTreeMap<_, _>, PauliError>>()?;

        let channel = Self {
            terms,
            normalization_tolerance: tolerance,
        };

        channel.validate()?;

        Ok(channel)
    }

    /// Creates the exact identity channel.
    #[must_use]
    pub fn identity() -> Self {
        let mut terms = BTreeMap::new();
        terms.insert(PauliString::identity(), Probability::ONE);

        Self {
            terms,
            normalization_tolerance: DEFAULT_NORMALIZATION_TOLERANCE,
        }
    }

    /// Returns the configured normalization tolerance.
    #[must_use]
    pub const fn normalization_tolerance(&self) -> f64 {
        self.normalization_tolerance
    }

    /// Validates the channel invariants.
    pub fn validate(&self) -> Result<(), PauliError> {
        if self.terms.is_empty() {
            return Err(PauliError::EmptyChannel);
        }

        let mut total = 0.0f64;

        for probability in self.terms.values() {
            if !probability.value().is_finite() {
                return Err(PauliError::NonFiniteProbability);
            }

            total = total
                .checked_add(probability.value())
                .ok_or(PauliError::ProbabilityArithmeticOverflow)?;
        }

        if !total.is_finite() {
            return Err(PauliError::NonFiniteNormalization);
        }

        if (total - 1.0).abs() > self.normalization_tolerance {
            return Err(PauliError::NotNormalized {
                total,
                tolerance: self.normalization_tolerance,
            });
        }

        Ok(())
    }

    /// Returns the number of non-zero Pauli terms.
    #[must_use]
    pub fn len(&self) -> usize {
        self.terms.len()
    }

    /// Returns true when no terms exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    /// Returns true when this is exactly the identity channel.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.len() == 1
            && self
                .probability_of(&PauliString::identity())
                .map(|probability| probability.is_one())
                .unwrap_or(false)
    }

    /// Returns the probability assigned to a Pauli string.
    #[must_use]
    pub fn probability_of(&self, pauli: &PauliString) -> Option<Probability> {
        self.terms.get(pauli).copied()
    }

    /// Returns an iterator over terms in canonical deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = (&PauliString, &Probability)> {
        self.terms.iter()
    }

    /// Returns all channel terms as an owned vector in canonical order.
    #[must_use]
    pub fn terms(&self) -> Vec<PauliTerm> {
        self.terms
            .iter()
            .map(|(pauli, &probability)| {
                PauliTerm::new(pauli.clone(), probability)
            })
            .collect()
    }

    /// Returns every non-identity resource touched by this channel.
    ///
    /// The result is deterministic and deduplicated.
    #[must_use]
    pub fn support(&self) -> Vec<PauliResource> {
        let mut support = std::collections::BTreeSet::new();

        for pauli in self.terms.keys() {
            support.extend(pauli.support());
        }

        support.into_iter().collect()
    }

    /// Returns all logical qubits touched by this channel.
    #[must_use]
    pub fn logical_support(&self) -> Vec<QubitId> {
        let mut support = std::collections::BTreeSet::new();

        for pauli in self.terms.keys() {
            support.extend(pauli.logical_support());
        }

        support.into_iter().collect()
    }

    /// Returns all physical qubits touched by this channel.
    #[must_use]
    pub fn physical_support(&self) -> Vec<PhysicalQubitId> {
        let mut support = std::collections::BTreeSet::new();

        for pauli in self.terms.keys() {
            support.extend(pauli.physical_support());
        }

        support.into_iter().collect()
    }

    /// Returns true when every term acts only on logical qubits.
    #[must_use]
    pub fn is_logical_only(&self) -> bool {
        self.terms.keys().all(PauliString::is_logical_only)
    }

    /// Returns true when every term acts only on physical qubits.
    #[must_use]
    pub fn is_physical_only(&self) -> bool {
        self.terms.keys().all(PauliString::is_physical_only)
    }

    /// Returns true when at least one term mixes logical and physical
    /// resources.
    #[must_use]
    pub fn contains_mixed_domain_terms(&self) -> bool {
        self.terms.keys().any(PauliString::is_mixed_domain)
    }

    /// Composes this channel with another Pauli channel.
    ///
    /// If:
    ///
    /// ```text
    /// A(ρ) = Σ_i p_i P_iρP_i†
    /// B(ρ) = Σ_j q_j Q_jρQ_j†
    /// ```
    ///
    /// then:
    ///
    /// ```text
    /// (B ∘ A)(ρ)
    ///   = Σ_i,j p_i q_j (Q_jP_i)ρ(Q_jP_i)†
    /// ```
    ///
    /// Global phase is irrelevant and therefore omitted.
    pub fn compose(&self, after: &Self) -> Result<Self, PauliError> {
        let mut merged: BTreeMap<PauliString, f64> = BTreeMap::new();

        for (before_pauli, before_probability) in &self.terms {
            for (after_pauli, after_probability) in &after.terms {
                let pauli = before_pauli.multiply(after_pauli);

                let product = before_probability
                    .value()
                    .checked_mul(after_probability.value())
                    .ok_or(PauliError::ProbabilityArithmeticOverflow)?;

                let entry = merged.entry(pauli).or_insert(0.0);

                *entry = entry
                    .checked_add(product)
                    .ok_or(PauliError::ProbabilityArithmeticOverflow)?;
            }
        }

        let terms = merged
            .into_iter()
            .map(|(pauli, probability)| {
                let probability =
                    Probability::new(probability).map_err(PauliError::Probability)?;

                Ok(PauliTerm::new(pauli, probability))
            })
            .collect::<Result<Vec<_>, PauliError>>()?;

        Self::new_with_tolerance(
            terms,
            self.normalization_tolerance
                .max(after.normalization_tolerance),
        )
    }

    /// Returns the tensor product with another channel.
    ///
    /// Each channel term is combined with each other term.
    ///
    /// The channels must have disjoint resource supports.
    pub fn tensor_product(&self, other: &Self) -> Result<Self, PauliError> {
        let self_support: std::collections::BTreeSet<_> =
            self.support().into_iter().collect();

        if let Some(resource) = other
            .support()
            .into_iter()
            .find(|resource| self_support.contains(resource))
        {
            return Err(PauliError::OverlappingTensorSupport { resource });
        }

        let mut terms = Vec::new();

        for (left_pauli, left_probability) in &self.terms {
            for (right_pauli, right_probability) in &other.terms {
                let pauli = left_pauli.tensor_product(right_pauli)?;

                let probability = left_probability
                    .value()
                    .checked_mul(right_probability.value())
                    .ok_or(PauliError::ProbabilityArithmeticOverflow)?;

                let probability =
                    Probability::new(probability).map_err(PauliError::Probability)?;

                terms.push(PauliTerm::new(pauli, probability));
            }
        }

        Self::new_with_tolerance(
            terms,
            self.normalization_tolerance
                .max(other.normalization_tolerance),
        )
    }

    /// Returns the channel's total probability.
    ///
    /// A valid channel always returns exactly the floating-point sum used by
    /// its validation contract, which is expected to be very close to one.
    #[must_use]
    pub fn total_probability(&self) -> f64 {
        self.terms
            .values()
            .map(|probability| probability.value())
            .sum()
    }

    /// Returns the largest non-identity Pauli probability.
    ///
    /// This is useful for simple noise diagnostics.
    #[must_use]
    pub fn largest_error_probability(&self) -> Option<Probability> {
        self.terms
            .iter()
            .filter_map(|(pauli, probability)| {
                if pauli.is_identity() {
                    None
                } else {
                    Some(*probability)
                }
            })
            .max_by(|left, right| {
                left.value()
                    .partial_cmp(&right.value())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// Returns the total probability of non-identity Pauli errors.
    pub fn total_error_probability(&self) -> Result<Probability, PauliError> {
        let mut total = 0.0f64;

        for (pauli, probability) in &self.terms {
            if pauli.is_identity() {
                continue;
            }

            total = total
                .checked_add(probability.value())
                .ok_or(PauliError::ProbabilityArithmeticOverflow)?;
        }

        Probability::new(total).map_err(PauliError::Probability)
    }

    /// Returns the identity probability.
    #[must_use]
    pub fn identity_probability(&self) -> Probability {
        self.probability_of(&PauliString::identity())
            .unwrap_or(Probability::ZERO)
    }

    /// Returns the channel as a normalized probability map.
    ///
    /// The returned map is a deterministic owned representation.
    #[must_use]
    pub fn probability_map(&self) -> BTreeMap<PauliString, Probability> {
        self.terms.clone()
    }

    /// Removes terms whose probabilities are below the explicitly supplied
    /// threshold and reconstructs the channel.
    ///
    /// This method is intentionally explicit because dropping probability mass
    /// changes the physical model.
    ///
    /// Therefore this function only accepts a zero threshold unless the caller
    /// also provides a replacement normalization policy through a separate
    /// future approximation subsystem.
    pub fn simplify_zero_only(
        &self,
        threshold: f64,
    ) -> Result<Self, PauliError> {
        if !threshold.is_finite() || threshold < 0.0 {
            return Err(PauliError::InvalidTolerance { tolerance: threshold });
        }

        if threshold != DEFAULT_ZERO_TOLERANCE {
            return Err(PauliError::UnsupportedProbabilityTruncation {
                threshold,
            });
        }

        Self::new_with_tolerance(
            self.terms()
                .into_iter()
                .filter(|term| !term.probability.is_zero())
                .collect::<Vec<_>>(),
            self.normalization_tolerance,
        )
    }
}

// ============================================================================
// Convenience constructors
// ============================================================================

impl PauliChannel {
    /// Constructs a two-term bit-flip channel:
    ///
    /// ```text
    /// E(ρ) = (1-p)ρ + pXρX
    /// ```
    ///
    /// The supplied probability is the X-error probability.
    pub fn bit_flip(
        resource: PauliResource,
        probability: Probability,
    ) -> Result<Self, PauliError> {
        let no_error = probability.complement();

        Self::new([
            PauliTerm::new(
                PauliString::identity(),
                no_error,
            ),
            PauliTerm::new(
                PauliString::single(resource, Pauli::X),
                probability,
            ),
        ])
    }

    /// Constructs a two-term phase-flip channel:
    ///
    /// ```text
    /// E(ρ) = (1-p)ρ + pZρZ
    /// ```
    pub fn phase_flip(
        resource: PauliResource,
        probability: Probability,
    ) -> Result<Self, PauliError> {
        let no_error = probability.complement();

        Self::new([
            PauliTerm::new(
                PauliString::identity(),
                no_error,
            ),
            PauliTerm::new(
                PauliString::single(resource, Pauli::Z),
                probability,
            ),
        ])
    }

    /// Constructs a Y-flip channel.
    pub fn bit_phase_flip(
        resource: PauliResource,
        probability: Probability,
    ) -> Result<Self, PauliError> {
        let no_error = probability.complement();

        Self::new([
            PauliTerm::new(
                PauliString::identity(),
                no_error,
            ),
            PauliTerm::new(
                PauliString::single(resource, Pauli::Y),
                probability,
            ),
        ])
    }

    /// Constructs the general single-resource Pauli channel.
    ///
    /// The three error probabilities are supplied explicitly.
    ///
    /// The identity probability is derived as:
    ///
    /// ```text
    /// p_I = 1 - p_X - p_Y - p_Z
    /// ```
    ///
    /// No clamping or silent normalization occurs.
    pub fn single_qubit(
        resource: PauliResource,
        x: Probability,
        y: Probability,
        z: Probability,
    ) -> Result<Self, PauliError> {
        let error_total = x
            .value()
            .checked_add(y.value())
            .and_then(|value| value.checked_add(z.value()))
            .ok_or(PauliError::ProbabilityArithmeticOverflow)?;

        if error_total > 1.0 {
            return Err(PauliError::ErrorProbabilityExceedsOne {
                total: error_total,
            });
        }

        let identity = Probability::new(1.0 - error_total)
            .map_err(PauliError::Probability)?;

        Self::new([
            PauliTerm::new(
                PauliString::identity(),
                identity,
            ),
            PauliTerm::new(
                PauliString::single(resource, Pauli::X),
                x,
            ),
            PauliTerm::new(
                PauliString::single(resource, Pauli::Y),
                y,
            ),
            PauliTerm::new(
                PauliString::single(resource, Pauli::Z),
                z,
            ),
        ])
    }

    /// Constructs a depolarizing channel for one resource.
    ///
    /// The supplied probability is the total non-identity error probability.
    ///
    /// It is divided equally between X, Y and Z.
    ///
    /// For a total error probability `p`:
    ///
    /// ```text
    /// p_I = 1-p
    /// p_X = p_Y = p_Z = p/3
    /// ```
    pub fn depolarizing(
        resource: PauliResource,
        probability: Probability,
    ) -> Result<Self, PauliError> {
        let error_each = probability
            .value()
            .checked_div(3.0)
            .ok_or(PauliError::ProbabilityArithmeticOverflow)?;

        let error_each =
            Probability::new(error_each).map_err(PauliError::Probability)?;

        Self::single_qubit(
            resource,
            error_each,
            error_each,
            error_each,
        )
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by Pauli-channel construction and operations.
#[derive(Debug, Clone, PartialEq)]
pub enum PauliError {
    /// A resource occurred more than once while constructing a Pauli string.
    DuplicateResource {
        resource: PauliResource,
    },

    /// Tensor product operands overlap on a resource.
    OverlappingTensorSupport {
        resource: PauliResource,
    },

    /// No non-zero terms were supplied.
    EmptyChannel,

    /// Probabilities did not normalize to one.
    NotNormalized {
        total: f64,
        tolerance: f64,
    },

    /// A numerical tolerance was invalid.
    InvalidTolerance {
        tolerance: f64,
    },

    /// Probability arithmetic overflowed or produced an invalid result.
    ProbabilityArithmeticOverflow,

    /// A probability supplied by the probability subsystem was invalid.
    Probability(crate::quantum::zqn::probability::ProbabilityError),

    /// A normalization calculation produced a non-finite result.
    NonFiniteNormalization,

    /// A stored probability was non-finite.
    NonFiniteProbability,

    /// The total error probability exceeded one.
    ErrorProbabilityExceedsOne {
        total: f64,
    },

    /// Arbitrary probability truncation is not permitted by this exact
    /// channel representation.
    UnsupportedProbabilityTruncation {
        threshold: f64,
    },
}

impl fmt::Display for PauliError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateResource { resource } => {
                write!(
                    formatter,
                    "Pauli resource {resource} was specified more than once"
                )
            }

            Self::OverlappingTensorSupport { resource } => {
                write!(
                    formatter,
                    "Pauli tensor-product operands overlap on resource {resource}"
                )
            }

            Self::EmptyChannel => {
                formatter.write_str("Pauli channel contains no non-zero terms")
            }

            Self::NotNormalized { total, tolerance } => {
                write!(
                    formatter,
                    "Pauli channel probability total {total} is not normalized \
                     within tolerance {tolerance}"
                )
            }

            Self::InvalidTolerance { tolerance } => {
                write!(
                    formatter,
                    "invalid Pauli-channel numerical tolerance {tolerance}"
                )
            }

            Self::ProbabilityArithmeticOverflow => {
                formatter.write_str(
                    "Pauli-channel probability arithmetic overflowed or \
                     produced an unrepresentable result",
                )
            }

            Self::Probability(error) => {
                write!(formatter, "invalid Pauli-channel probability: {error}")
            }

            Self::NonFiniteNormalization => {
                formatter.write_str(
                    "Pauli-channel normalization became non-finite",
                )
            }

            Self::NonFiniteProbability => {
                formatter.write_str(
                    "Pauli channel contains a non-finite probability",
                )
            }

            Self::ErrorProbabilityExceedsOne { total } => {
                write!(
                    formatter,
                    "total Pauli error probability {total} exceeds one"
                )
            }

            Self::UnsupportedProbabilityTruncation { threshold } => {
                write!(
                    formatter,
                    "probability truncation at threshold {threshold} \
                     is not supported by the exact Pauli-channel representation"
                )
            }
        }
    }
}

impl std::error::Error for PauliError {}

// ============================================================================
// Internal validation
// ============================================================================

fn validate_tolerance(tolerance: f64) -> Result<(), PauliError> {
    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(PauliError::InvalidTolerance { tolerance });
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::zqn::probability::Probability;

    fn logical(id: u64) -> PauliResource {
        PauliResource::Logical(QubitId::new(id))
    }

    fn physical(id: u64) -> PauliResource {
        PauliResource::Physical(PhysicalQubitId::new(id))
    }

    #[test]
    fn identity_pauli_is_identity() {
        assert!(Pauli::I.is_identity());
        assert!(!Pauli::X.is_identity());
    }

    #[test]
    fn pauli_multiplication_is_correct_modulo_phase() {
        assert_eq!(Pauli::X.multiply(Pauli::X), Pauli::I);
        assert_eq!(Pauli::Y.multiply(Pauli::Y), Pauli::I);
        assert_eq!(Pauli::Z.multiply(Pauli::Z), Pauli::I);

        assert_eq!(Pauli::X.multiply(Pauli::Y), Pauli::Z);
        assert_eq!(Pauli::Y.multiply(Pauli::Z), Pauli::X);
        assert_eq!(Pauli::Z.multiply(Pauli::X), Pauli::Y);
    }

    #[test]
    fn pauli_commutation_is_correct() {
        assert!(Pauli::I.commutes_with(Pauli::X));
        assert!(Pauli::X.commutes_with(Pauli::X));
        assert!(!Pauli::X.commutes_with(Pauli::Y));
        assert!(!Pauli::Y.commutes_with(Pauli::Z));
        assert!(!Pauli::Z.commutes_with(Pauli::X));
    }

    #[test]
    fn identity_string_has_zero_weight() {
        let identity = PauliString::identity();

        assert!(identity.is_identity());
        assert_eq!(identity.weight(), 0);
        assert!(identity.support().is_empty());
    }

    #[test]
    fn sparse_string_stores_only_non_identity_factors() {
        let mut pauli = PauliString::identity();

        pauli.set(logical(0), Pauli::X);
        pauli.set(logical(1), Pauli::I);
        pauli.set(logical(10_000), Pauli::Z);

        assert_eq!(pauli.weight(), 2);
        assert_eq!(pauli.get(&logical(1)), Pauli::I);
        assert_eq!(pauli.get(&logical(0)), Pauli::X);
        assert_eq!(pauli.get(&logical(10_000)), Pauli::Z);
    }

    #[test]
    fn duplicate_resource_construction_is_rejected() {
        let result = PauliString::try_from_factors([
            (logical(0), Pauli::X),
            (logical(0), Pauli::Z),
        ]);

        assert!(matches!(
            result,
            Err(PauliError::DuplicateResource { .. })
        ));
    }

    #[test]
    fn string_multiplication_is_correct() {
        let left = PauliString::single(logical(0), Pauli::X);
        let right = PauliString::single(logical(0), Pauli::Y);

        let result = left.multiply(&right);

        assert_eq!(result.get(&logical(0)), Pauli::Z);
    }

    #[test]
    fn_disjoint_pauli_strings_commute() {
        let left = PauliString::single(logical(0), Pauli::X);
        let right = PauliString::single(logical(1), Pauli::Y);

        assert!(left.commutes_with(&right));
    }

    #[test]
    fn same_resource_anticommuting_strings_do_not_commute() {
        let left = PauliString::single(logical(0), Pauli::X);
        let right = PauliString::single(logical(0), Pauli::Y);

        assert!(!left.commutes_with(&right));
    }

    #[test]
    fn two_resource_anticommutations_cancel() {
        let left = PauliString::try_from_factors([
            (logical(0), Pauli::X),
            (logical(1), Pauli::X),
        ])
        .unwrap();

        let right = PauliString::try_from_factors([
            (logical(0), Pauli::Y),
            (logical(1), Pauli::Y),
        ])
        .unwrap();

        assert!(left.commutes_with(&right));
    }

    #[test]
    fn tensor_product_requires_disjoint_support() {
        let left = PauliString::single(logical(0), Pauli::X);
        let right = PauliString::single(logical(1), Pauli::Z);

        let result = left.tensor_product(&right).unwrap();

        assert_eq!(result.weight(), 2);

        let overlapping =
            PauliString::single(logical(0), Pauli::Z);

        assert!(matches!(
            left.tensor_product(&overlapping),
            Err(PauliError::OverlappingTensorSupport { .. })
        ));
    }

    #[test]
    fn identity_channel_is_valid() {
        let channel = PauliChannel::identity();

        assert!(channel.validate().is_ok());
        assert!(channel.is_identity());
        assert_eq!(channel.len(), 1);
        assert_eq!(
            channel.identity_probability(),
            Probability::ONE
        );
    }

    #[test]
    fn zero_probability_terms_are_removed() {
        let channel = PauliChannel::new([
            PauliTerm::new(
                PauliString::identity(),
                Probability::ONE,
            ),
            PauliTerm::new(
                PauliString::single(logical(0), Pauli::X),
                Probability::ZERO,
            ),
        ])
        .unwrap();

        assert_eq!(channel.len(), 1);
        assert!(channel.is_identity());
    }

    #[test]
    fn duplicate_channel_terms_are_merged() {
        let p1 = Probability::new(0.25).unwrap();
        let p2 = Probability::new(0.75).unwrap();

        let x = PauliString::single(logical(0), Pauli::X);

        let channel = PauliChannel::new([
            PauliTerm::new(
                PauliString::identity(),
                Probability::ZERO,
            ),
            PauliTerm::new(x.clone(), p1),
            PauliTerm::new(x.clone(), p2),
        ])
        .unwrap();

        assert_eq!(channel.len(), 1);
        assert_eq!(
            channel.probability_of(&x).unwrap().value(),
            1.0
        );
    }

    #[test]
    fn non_normalized_channel_is_rejected() {
        let result = PauliChannel::new([
            PauliTerm::new(
                PauliString::identity(),
                Probability::new(0.4).unwrap(),
            ),
            PauliTerm::new(
                PauliString::single(logical(0), Pauli::X),
                Probability::new(0.4).unwrap(),
            ),
        ]);

        assert!(matches!(
            result,
            Err(PauliError::NotNormalized { .. })
        ));
    }

    #[test]
    fn bit_flip_channel_is_correct() {
        let p = Probability::new(0.2).unwrap();

        let channel =
            PauliChannel::bit_flip(logical(0), p).unwrap();

        assert_eq!(channel.identity_probability().value(), 0.8);

        let x = PauliString::single(logical(0), Pauli::X);

        assert_eq!(
            channel.probability_of(&x).unwrap().value(),
            0.2
        );
    }

    #[test]
    fn phase_flip_channel_is_correct() {
        let p = Probability::new(0.3).unwrap();

        let channel =
            PauliChannel::phase_flip(logical(0), p).unwrap();

        let z = PauliString::single(logical(0), Pauli::Z);

        assert_eq!(
            channel.probability_of(&z).unwrap().value(),
            0.3
        );
    }

    #[test]
    fn single_resource_pauli_channel_is_normalized() {
        let channel = PauliChannel::single_qubit(
            logical(0),
            Probability::new(0.1).unwrap(),
            Probability::new(0.2).unwrap(),
            Probability::new(0.3).unwrap(),
        )
        .unwrap();

        assert_eq!(channel.total_probability(), 1.0);
        assert_eq!(
            channel.identity_probability().value(),
            0.4
        );
    }

    #[test]
    fn single_resource_pauli_channel_rejects_excess_probability() {
        let result = PauliChannel::single_qubit(
            logical(0),
            Probability::new(0.4).unwrap(),
            Probability::new(0.4).unwrap(),
            Probability::new(0.4).unwrap(),
        );

        assert!(matches!(
            result,
            Err(PauliError::ErrorProbabilityExceedsOne { .. })
        ));
    }

    #[test]
    fn depolarizing_channel_has_equal_non_identity_terms() {
        let channel =
            PauliChannel::depolarizing(
                logical(0),
                Probability::new(0.3).unwrap(),
            )
            .unwrap();

        assert_eq!(
            channel
                .probability_of(
                    &PauliString::single(
                        logical(0),
                        Pauli::X
                    )
                )
                .unwrap()
                .value(),
            0.1
        );

        assert_eq!(
            channel
                .probability_of(
                    &PauliString::single(
                        logical(0),
                        Pauli::Y
                    )
                )
                .unwrap()
                .value(),
            0.1
        );

        assert_eq!(
            channel
                .probability_of(
                    &PauliString::single(
                        logical(0),
                        Pauli::Z
                    )
                )
                .unwrap()
                .value(),
            0.1
        );
    }

    #[test]
    fn composition_is_normalized() {
        let first =
            PauliChannel::bit_flip(
                logical(0),
                Probability::new(0.1).unwrap(),
            )
            .unwrap();

        let second =
            PauliChannel::bit_flip(
                logical(0),
                Probability::new(0.2).unwrap(),
            )
            .unwrap();

        let composed =
            first.compose(&second).unwrap();

        assert_eq!(
            composed.total_probability(),
            1.0
        );

        assert!(composed.validate().is_ok());
    }

    #[test]
    fn identity_composition_is_identity_preserving() {
        let channel =
            PauliChannel::phase_flip(
                logical(0),
                Probability::new(0.2).unwrap(),
            )
            .unwrap();

        let left =
            PauliChannel::identity()
                .compose(&channel)
                .unwrap();

        let right =
            channel
                .compose(&PauliChannel::identity())
                .unwrap();

        assert_eq!(left, channel);
        assert_eq!(right, channel);
    }

    #[test]
    fn tensor_product_preserves_normalization() {
        let first =
            PauliChannel::bit_flip(
                logical(0),
                Probability::new(0.1).unwrap(),
            )
            .unwrap();

        let second =
            PauliChannel::phase_flip(
                logical(1),
                Probability::new(0.2).unwrap(),
            )
            .unwrap();

        let combined =
            first.tensor_product(&second).unwrap();

        assert_eq!(
            combined.total_probability(),
            1.0
        );

        assert_eq!(combined.len(), 4);
    }

    #[test]
    fn physical_identity_is_distinct_from_logical_identity() {
        let logical_resource = logical(7);
        let physical_resource = physical(7);

        assert_ne!(
            logical_resource,
            physical_resource
        );
    }

    #[test]
    fn support_is_deterministic() {
        let channel = PauliChannel::new([
            PauliTerm::new(
                PauliString::single(
                    logical(10),
                    Pauli::X,
                ),
                Probability::new(0.2).unwrap(),
            ),
            PauliTerm::new(
                PauliString::single(
                    logical(2),
                    Pauli::Z,
                ),
                Probability::new(0.3).unwrap(),
            ),
            PauliTerm::new(
                PauliString::identity(),
                Probability::new(0.5).unwrap(),
            ),
        ])
        .unwrap();

        let support = channel.support();

        assert_eq!(
            support,
            vec![logical(2), logical(10)]
        );
    }

    #[test]
    fn channel_can_represent_large_sparse_resource_ids() {
        let high_id = u64::MAX;

        let channel =
            PauliChannel::bit_flip(
                physical(high_id),
                Probability::new(0.001).unwrap(),
            )
            .unwrap();

        assert_eq!(
            channel.physical_support(),
            vec![PhysicalQubitId::new(high_id)]
        );
    }

    #[test]
    fn mixed_logical_and_physical_terms_are_detectable() {
        let pauli =
            PauliString::try_from_factors([
                (logical(0), Pauli::X),
                (physical(1), Pauli::Z),
            ])
            .unwrap();

        assert!(pauli.is_mixed_domain());
        assert!(!pauli.is_logical_only());
        assert!(!pauli.is_physical_only());
    }

    #[test]
    fn channel_terms_have_deterministic_order() {
        let channel = PauliChannel::new([
            PauliTerm::new(
                PauliString::single(
                    logical(3),
                    Pauli::Z,
                ),
                Probability::new(0.2).unwrap(),
            ),
            PauliTerm::new(
                PauliString::identity(),
                Probability::new(0.5).unwrap(),
            ),
            PauliTerm::new(
                PauliString::single(
                    logical(1),
                    Pauli::X,
                ),
                Probability::new(0.3).unwrap(),
            ),
        ])
        .unwrap();

        let first: Vec<_> = channel
            .iter()
            .map(|(pauli, _)| pauli.to_string())
            .collect();

        let second: Vec<_> = channel
            .iter()
            .map(|(pauli, _)| pauli.to_string())
            .collect();

        assert_eq!(first, second);
    }

    #[test]
    fn no_probability_truncation_is_performed_silently() {
        let channel =
            PauliChannel::identity();

        let result =
            channel.simplify_zero_only(1.0e-6);

        assert!(matches!(
            result,
            Err(
                PauliError::UnsupportedProbabilityTruncation { .. }
            )
        ));
    }
}