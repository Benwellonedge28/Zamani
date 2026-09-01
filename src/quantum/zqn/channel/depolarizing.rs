//! Zamani Quantum Noise (ZQN) — Depolarizing Quantum Channel.
//!
//! This module provides the canonical semantic representation of a
//! depolarizing quantum channel.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - `DepolarizingChannel`;
//! - the standard dimension-independent depolarizing semantics;
//! - validated depolarizing probability parameters;
//! - analytical dimension/resource calculations;
//! - conversion from average fidelity to depolarizing probability;
//! - conversion from depolarizing probability to average fidelity;
//! - the equivalent uniform non-identity Pauli probability for qubit systems;
//! - deterministic semantic inspection;
//! - resource-safe dimension calculations;
//! - implementation of the representation-independent `QuantumChannel`
//!   contract.
//!
//! # Does NOT own
//!
//! This file does NOT own:
//!
//! - density matrices;
//! - state-vector simulation;
//! - Kraus matrix materialization;
//! - Choi matrix materialization;
//! - Pauli-string enumeration;
//! - random-number generation;
//! - Monte Carlo sampling;
//! - execution;
//! - hardware APIs;
//! - calibration;
//! - characterization experiments;
//! - routing;
//! - scheduling;
//! - QEC decoding;
//! - serialization schemas;
//! - vendor-specific behavior.
//!
//! Those responsibilities belong to their respective ZQN/quantum subsystems.
//!
//! # Mathematical definition
//!
//! For a finite-dimensional Hilbert space of dimension `d`, the standard
//! depolarizing channel used here is:
//!
//! ```text
//! D_p(rho) = (1 - p) rho + p I/d
//! ```
//!
//! where:
//!
//! ```text
//! 0 <= p <= 1
//! d >= 2
//! ```
//!
//! This is the **state-mixing probability convention**.
//!
//! It is important not to confuse this `p` with the Pauli-transfer shrinking
//! factor commonly also called a "depolarizing parameter", or with a
//! randomized-benchmarking error parameter.
//!
//! For this convention the traceless component is scaled by:
//!
//! ```text
//! lambda = 1 - p
//! ```
//!
//! For a `d`-dimensional system:
//!
//! ```text
//! F_avg = 1 - p * (d - 1) / d
//! ```
//!
//! and therefore:
//!
//! ```text
//! p = d/(d - 1) * (1 - F_avg)
//! ```
//!
//! # Qubit/Pauli equivalence
//!
//! For an `n`-qubit system:
//!
//! ```text
//! d = 2^n
//! ```
//!
//! and the same channel can be expressed as a uniform Pauli channel:
//!
//! ```text
//! P(I) = 1 - p
//! P(P_nonidentity) = p / (4^n - 1)
//! ```
//!
//! The implementation deliberately does NOT enumerate those `4^n - 1` terms.
//! The probability is computed analytically when requested.
//!
//! This is essential for scalability.
//!
//! # Scalability
//!
//! There is no semantic maximum for:
//!
//! - number of subsystems;
//! - subsystem dimension;
//! - channel arity;
//! - machine size;
//! - number of qubits;
//! - number of logical resources;
//! - number of physical resources.
//!
//! The implementation does NOT contain:
//!
//! ```text
//! MAX_QUBITS
//! MAX_SUBSYSTEMS
//! MAX_DIMENSION
//! MAX_PAULIS
//! MAX_TERMS
//! ```
//!
//! Dimension calculations use checked arithmetic. An arithmetic overflow is a
//! representational/resource error, not an artificial quantum-computing limit.
//!
//! A very large depolarizing channel therefore remains a small semantic object:
//!
//! ```text
//! DepolarizingChannel
//!     ├── support
//!     └── probability
//! ```
//!
//! It does not materialize an exponentially large matrix or Pauli table.
//!
//! "Infinity" means no artificial finite semantic ceiling. Actual execution
//! remains bounded by available resources and the representation selected by
//! downstream simulation/hardware layers.
//!
//! # Canonical quantum identity
//!
//! ZQN does not define another `QubitId` or `PhysicalQubitId`.
//!
//! Qubit-based construction uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! through the canonical channel support types.
//!
//! Hardware identity remains owned by the quantum IR/hardware subsystem.
//!
//! # Representation strategy
//!
//! The depolarizing channel is represented analytically rather than as:
//!
//! - a dense matrix;
//! - a dense tensor;
//! - an enumerated Pauli distribution;
//! - a state vector.
//!
//! This means that constructing a depolarizing channel over one million
//! resources does not itself require constructing a one-million-resource
//! quantum state or an exponentially large error table.
//!
//! The downstream representation/simulation layer chooses the appropriate
//! realization for the available target resources.
//!
//! # Determinism
//!
//! This file contains:
//!
//! - no RNG;
//! - no global mutable state;
//! - no hidden sampling;
//! - no time-dependent behavior;
//! - no hardware discovery.
//!
//! Identical inputs produce identical semantic results.
//!
//! Sampling belongs to ZQN simulation and must use an explicit execution
//! context and seed policy.
//!
//! # Numerical policy
//!
//! `Probability` is the repository's canonical `[0, 1]` scalar.
//!
//! This file does not silently clamp invalid values.
//!
//! Floating-point operations are used only for derived numerical quantities
//! such as:
//!
//! - average fidelity;
//! - Pauli-term probability;
//! - shrinking factor.
//!
//! Derived values are checked before being returned as `Probability`.
//!
//! # Physicality
//!
//! For every finite support with valid subsystem dimensions and `0 <= p <= 1`,
//! the standard depolarizing map is CPTP.
//!
//! The implementation therefore reports validated physicality for the
//! mathematical channel itself.
//!
//! This does NOT mean that a particular hardware target can execute the model
//! exactly. Target compatibility is determined by the target/capability layer.
//!
//! # Integration
//!
//! ```text
//! quantum::ir::qubit\n//!         │\n//!         ▼\n//! ChannelSubsystem\n//!         │\n//!         ▼\n//! ChannelSupport\n//!         │\n//!         ▼\n//! DepolarizingChannel\n//!         │\n//!   ┌─────┼──────────────┐\n//!   ▼     ▼              ▼\n//! simulator  routing     scheduling\n//!   │                       │\n//!   ▼                       ▼\n//! QEC / benchmarking     hardware\n//! ```
//!
//! The implementation is independent of all downstream consumers.
//!
//! # QuantumChannel integration
//!
//! `DepolarizingChannel` implements the repository's representation-independent
//! `QuantumChannel` abstraction.
//!
//! Consequently downstream systems can consume it without knowing that its
//! concrete mathematical storage is analytical.
//!
//! No second channel trait is introduced.
//!
//! # Composition
//!
//! Two depolarizing channels acting on the same dimension compose as:
//!
//! ```text
//! D_p2 o D_p1 = D_(1 - (1-p1)(1-p2))
//! ```
//!
//! This is exposed analytically and does not require matrix multiplication.
//!
//! # Tensor products
//!
//! The tensor product of two independent depolarizing channels is generally
//! NOT itself the same global depolarizing channel on the combined Hilbert
//! space.
//!
//! Therefore this module deliberately does not provide an incorrectly named
//! `tensor_product()` that silently changes the noise model.
//!
//! Independent tensor-product composition belongs to the general channel
//! composition subsystem.
//!
//! # Average fidelity
//!
//! For dimension `d`:
//!
//! ```text
//! F_avg = 1 - p (d - 1) / d
//! ```
//!
//! The inverse is:
//!
//! ```text
//! p = d/(d - 1) (1 - F_avg)
//! ```
//!
//! These conversions use the standard state-mixing depolarizing convention.
//!
//! # Randomized benchmarking integration
//!
//! Randomized benchmarking commonly uses a depolarizing/twirled model, but the
//! fitted parameter must not automatically be treated as this module's `p`
//! without applying the correct convention and assumptions.
//!
//! This module therefore exposes explicit conversion methods rather than
//! accepting an ambiguously named "error rate".
//!
//! # Serialization
//!
//! This file does not define a wire format.
//!
//! A future `zqn::io` layer should serialize:
//!
//! - channel identity;
//! - channel name;
//! - support;
//! - probability;
//! - schema version;
//! - provenance where applicable.
//!
//! Rust memory layout is not an external serialization contract.
//!
//! # Security/resource safety
//!
//! This file:
//!
//! - uses safe Rust only;
//! - performs no I/O;
//! - performs no FFI;
//! - performs no dynamic code execution;
//! - does not allocate exponentially-sized structures;
//! - does not enumerate Pauli operators;
//! - uses checked arithmetic for derived resource counts;
//! - rejects invalid numerical parameters;
//! - contains no global state.
//!
//! Callers handling untrusted serialized supports must apply their explicit
//! resource policy before constructing enormous support collections.
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
//! 1. the standard depolarizing map is represented exactly;
//! 2. `p` is explicitly the state-mixing probability;
//! 3. probabilities are validated;
//! 4. canonical channel support is used;
//! 5. canonical `QubitId` is used for qubit construction;
//! 6. no machine-size limit exists;
//! 7. no exponential Pauli materialization occurs;
//! 8. dimension arithmetic is checked;
//! 9. average-fidelity conversion is mathematically explicit;
//! 10. Pauli-equivalent probabilities are derived analytically;
//! 11. composition is exact for compatible depolarizing channels;
//! 12. sampling is absent;
//! 13. hardware APIs are absent;
//! 14. QEC dependencies are absent;
//! 15. no unsafe code exists;
//! 16. the canonical `QuantumChannel` contract is implemented;
//! 17. tests cover validation, algebra, scaling and numerical edge cases.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use crate::quantum::ir::qubit::QubitId;

use crate::quantum::zqn::channel::channel::{
    ChannelAccuracy,
    ChannelDescriptor,
    ChannelId,
    ChannelPhysicality,
    ChannelRepresentation,
    ChannelResult,
    ChannelSupport,
    QuantumChannel,
};

use crate::quantum::zqn::probability::Probability;

/// Errors specific to depolarizing-channel construction and derived
/// calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum DepolarizingError {
    /// A derived dimension/count cannot be represented by the chosen portable
    /// integer representation.
    DimensionOverflow,

    /// A derived probability could not be represented as a valid ZQN
    /// probability.
    DerivedProbabilityOutOfRange,

    /// Average fidelity was invalid.
    InvalidAverageFidelity,

    /// The two channels do not act on compatible dimensions/supports.
    IncompatibleChannels,

    /// The requested operation requires a representation not provided by this
    /// analytical channel.
    UnsupportedOperation(&'static str),
}

impl fmt::Display for DepolarizingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionOverflow => {
                write!(formatter, "depolarizing channel dimension overflows u128")
            }

            Self::DerivedProbabilityOutOfRange => {
                write!(
                    formatter,
                    "derived depolarizing probability is outside [0, 1]"
                )
            }

            Self::InvalidAverageFidelity => {
                write!(formatter, "average fidelity must be finite and in [0, 1]")
            }

            Self::IncompatibleChannels => {
                write!(
                    formatter,
                    "depolarizing channels have incompatible supports or dimensions"
                )
            }

            Self::UnsupportedOperation(operation) => {
                write!(formatter, "unsupported depolarizing operation: {operation}")
            }
        }
    }
}

impl std::error::Error for DepolarizingError {}

/// Production representation of the standard depolarizing channel.
///
/// The channel is stored analytically:
///
/// ```text
/// D_p(rho) = (1-p) rho + p I/d
/// ```
///
/// This avoids exponential allocation for large multi-qubit systems.
#[derive(Debug, Clone, PartialEq)]
pub struct DepolarizingChannel {
    /// Stable semantic channel identity.
    id: ChannelId,

    /// Human-readable semantic name.
    name: String,

    /// Canonical channel support.
    support: ChannelSupport,

    /// State-mixing depolarizing probability.
    probability: Probability,
}

impl DepolarizingChannel {
    /// Constructs a depolarizing channel from canonical channel support.
    ///
    /// The support's input and output domains must be compatible. A
    /// depolarizing channel is dimension preserving.
    pub fn new(
        id: ChannelId,
        name: impl Into<String>,
        support: ChannelSupport,
        probability: Probability,
    ) -> ChannelResult<Self> {
        support.validate()?;

        if support.input().len() != support.output().len() {
            return Err(crate::quantum::zqn::channel::channel::ChannelError::DomainMismatch {
                input_arity: support.input().len(),
                output_arity: support.output().len(),
            });
        }

        for (input, output) in support.input().iter().zip(support.output().iter()) {
            if input.dimension() != output.dimension() {
                return Err(
                    crate::quantum::zqn::channel::channel::ChannelError::DomainMismatch {
                        input_arity: support.input().len(),
                        output_arity: support.output().len(),
                    },
                );
            }
        }

        Ok(Self {
            id,
            name: name.into(),
            support,
            probability,
        })
    }

    /// Constructs a depolarizing channel over logical qubits.
    ///
    /// Every supplied `QubitId` becomes a dimension-2 subsystem.
    ///
    /// The qubit IDs are canonical IDs owned by `quantum::ir::qubit`.
    pub fn from_logical_qubits(
        id: ChannelId,
        name: impl Into<String>,
        qubits: Vec<QubitId>,
        probability: Probability,
    ) -> ChannelResult<Self> {
        use crate::quantum::zqn::channel::channel::{
            ChannelSubsystem,
            ChannelSubsystemId,
        };

        let mut input = Vec::with_capacity(qubits.len());
        let mut output = Vec::with_capacity(qubits.len());

        for qubit in qubits {
            let subsystem = ChannelSubsystem::new(
                ChannelSubsystemId::qubit(qubit),
                2,
            )?;

            input.push(subsystem);
            output.push(subsystem);
        }

        let support = ChannelSupport::new(input, output)?;

        Self::new(id, name, support, probability)
    }

    /// Returns the channel identity.
    #[must_use]
    pub const fn id(&self) -> ChannelId {
        self.id
    }

    /// Returns the channel name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the canonical channel support.
    #[must_use]
    pub const fn support(&self) -> &ChannelSupport {
        &self.support
    }

    /// Returns the depolarizing probability `p`.
    #[must_use]
    pub const fn probability(&self) -> Probability {
        self.probability
    }

    /// Returns the identity/shrinking coefficient:
    ///
    /// ```text
    /// lambda = 1 - p
    /// ```
    ///
    /// On traceless operators, the channel acts as multiplication by
    /// `lambda`.
    #[must_use]
    pub fn shrinking_factor(&self) -> Probability {
        self.probability.complement()
    }

    /// Returns the total Hilbert-space dimension of the channel support.
    ///
    /// For an `n`-qubit support this is `2^n`.
    ///
    /// The calculation is checked and never wraps.
    pub fn dimension(&self) -> Result<u128, DepolarizingError> {
        checked_support_dimension(&self.support)
    }

    /// Returns the number of non-identity generalized operator-basis elements:
    ///
    /// ```text
    /// d^2 - 1
    /// ```
    ///
    /// This is a count only. No elements are materialized.
    pub fn non_identity_operator_count(&self) -> Result<u128, DepolarizingError> {
        let dimension = self.dimension()?;
        let square = dimension
            .checked_mul(dimension)
            .ok_or(DepolarizingError::DimensionOverflow)?;

        square
            .checked_sub(1)
            .ok_or(DepolarizingError::DimensionOverflow)
    }

    /// Returns the probability assigned to each non-identity generalized
    /// operator in the uniform operator-basis realization:
    ///
    /// ```text
    /// p / (d^2 - 1)
    /// ```
    ///
    /// This method computes the value analytically and does not enumerate the
    /// operator basis.
    pub fn non_identity_operator_probability(
        &self,
    ) -> Result<Probability, DepolarizingError> {
        let count = self.non_identity_operator_count()?;

        if count == 0 {
            return Err(DepolarizingError::DerivedProbabilityOutOfRange);
        }

        Probability::new(self.probability.value() / count as f64)
            .map_err(|_| DepolarizingError::DerivedProbabilityOutOfRange)
    }

    /// Returns the probability of the identity component in the equivalent
    /// uniform operator-basis realization.
    ///
    /// ```text
    /// P(I) = 1 - p
    /// ```
    #[must_use]
    pub const fn identity_probability(&self) -> Probability {
        self.probability.complement()
    }

    /// Returns the average fidelity of this depolarizing channel.
    ///
    /// For dimension `d`:
    ///
    /// ```text
    /// F_avg = 1 - p (d - 1) / d
    /// ```
    pub fn average_fidelity(&self) -> Result<f64, DepolarizingError> {
        let dimension = self.dimension()?;

        if dimension < 2 {
            return Err(DepolarizingError::DimensionOverflow);
        }

        let d = dimension as f64;
        let fidelity =
            1.0 - self.probability.value() * ((d - 1.0) / d);

        if !fidelity.is_finite() || !(0.0..=1.0).contains(&fidelity) {
            return Err(DepolarizingError::DerivedProbabilityOutOfRange);
        }

        Ok(fidelity)
    }

    /// Constructs the depolarizing probability corresponding to a supplied
    /// average fidelity.
    ///
    /// For dimension `d`:
    ///
    /// ```text
    /// p = d/(d - 1) * (1 - F_avg)
    /// ```
    ///
    /// This constructor uses the state-mixing convention defined by this file.
    pub fn probability_from_average_fidelity(
        dimension: u128,
        average_fidelity: f64,
    ) -> Result<Probability, DepolarizingError> {
        if dimension < 2
            || !average_fidelity.is_finite()
            || !(0.0..=1.0).contains(&average_fidelity)
        {
            return Err(DepolarizingError::InvalidAverageFidelity);
        }

        let d = dimension as f64;

        let probability =
            d / (d - 1.0) * (1.0 - average_fidelity);

        Probability::new(probability)
            .map_err(|_| DepolarizingError::DerivedProbabilityOutOfRange)
    }

    /// Constructs a depolarizing channel from an average-fidelity estimate.
    pub fn from_average_fidelity(
        id: ChannelId,
        name: impl Into<String>,
        support: ChannelSupport,
        average_fidelity: f64,
    ) -> Result<Self, DepolarizingError> {
        let dimension = checked_support_dimension(&support)?;
        let probability =
            Self::probability_from_average_fidelity(
                dimension,
                average_fidelity,
            )?;

        Self::new(id, name, support, probability)
            .map_err(|_| DepolarizingError::IncompatibleChannels)
    }

    /// Composes this depolarizing channel after another compatible
    /// depolarizing channel.
    ///
    /// ```text
    /// D_p2 o D_p1
    ///
    /// = D_(1 - (1-p1)(1-p2))
    /// ```
    pub fn compose_after(
        &self,
        first: &Self,
        id: ChannelId,
        name: impl Into<String>,
    ) -> Result<Self, DepolarizingError> {
        if self.support != first.support {
            return Err(DepolarizingError::IncompatibleChannels);
        }

        let p1 = first.probability.value();
        let p2 = self.probability.value();

        let composed = 1.0 - (1.0 - p1) * (1.0 - p2);

        let probability = Probability::new(composed)
            .map_err(|_| DepolarizingError::DerivedProbabilityOutOfRange)?;

        Self::new(id, name, self.support.clone(), probability)
            .map_err(|_| DepolarizingError::IncompatibleChannels)
    }

    /// Returns the standard depolarizing representation formula as text.
    ///
    /// This is intended for diagnostics/documentation and does not perform
    /// execution.
    #[must_use]
    pub fn semantic_formula(&self) -> &'static str {
        "D_p(rho) = (1-p) rho + p I/d"
    }

    /// Returns whether the channel has zero noise.
    #[must_use]
    pub const fn is_identity(&self) -> bool {
        self.probability.is_zero()
    }

    /// Returns whether the channel is fully depolarizing.
    #[must_use]
    pub const fn is_fully_depolarizing(&self) -> bool {
        self.probability.is_one()
    }
}

impl QuantumChannel for DepolarizingChannel {
    fn id(&self) -> ChannelId {
        self.id
    }

    fn support(&self) -> &ChannelSupport {
        &self.support
    }

    fn representation(&self) -> ChannelRepresentation {
        /*
         * The semantic object is analytically depolarizing. The repository's
         * channel representation enum intentionally identifies mathematical
         * storage classes such as Kraus/Choi/Pauli/Stochastic. A depolarizing
         * channel is not itself a separate storage representation, so it is
         * represented through the general channel abstraction until a dedicated
         * analytical representation is introduced centrally.
         */
        ChannelRepresentation::Pauli
    }

    fn physicality(&self) -> ChannelPhysicality {
        ChannelPhysicality::Validated
    }

    fn accuracy(&self) -> ChannelAccuracy {
        ChannelAccuracy::exact()
    }

    fn descriptor(&self) -> ChannelDescriptor {
        /*
         * The descriptor is constructed from the already validated support and
         * semantic channel data. This object is deterministic and contains no
         * execution state.
         */
        ChannelDescriptor::new(
            self.id,
            self.name.clone(),
            self.support.clone(),
            self.representation(),
            self.physicality(),
            self.accuracy(),
            self.resource_requirements(),
        )
        .expect("DepolarizingChannel invariants guarantee a valid descriptor")
    }

    fn resource_requirements(
        &self,
    ) -> crate::quantum::zqn::channel::channel::ChannelResourceRequirements {
        /*
         * The analytical representation requires no exponentially-sized
         * operator table. The support itself remains the authoritative resource
         * description.
         *
         * The repository's resource-requirement abstraction owns the exact
         * accounting policy; this channel therefore reports the support-derived
         * scalar representation without introducing a machine-size limit.
         */
        crate::quantum::zqn::channel::channel::ChannelResourceRequirements::known(
            1,
            0,
        )
    }
}

/// Computes the tensor-product Hilbert-space dimension of a channel support.
///
/// This is intentionally independent of the number of physical machine qubits.
/// It only describes the supplied semantic support.
fn checked_support_dimension(
    support: &ChannelSupport,
) -> Result<u128, DepolarizingError> {
    let mut dimension = 1_u128;

    for subsystem in support.input() {
        dimension = dimension
            .checked_mul(subsystem.dimension() as u128)
            .ok_or(DepolarizingError::DimensionOverflow)?;
    }

    if dimension < 2 {
        return Err(DepolarizingError::DimensionOverflow);
    }

    Ok(dimension)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::zqn::channel::channel::{
        ChannelSubsystem,
        ChannelSubsystemId,
        ChannelSupport,
    };

    fn logical_support(qubit: QubitId) -> ChannelSupport {
        let subsystem = ChannelSubsystem::new(
            ChannelSubsystemId::qubit(qubit),
            2,
        )
        .expect("qubit subsystem must be valid");

        ChannelSupport::new(
            vec![subsystem],
            vec![subsystem],
        )
        .expect("support must be valid")
    }

    #[test]
    fn zero_probability_is_identity() {
        let support =
            logical_support(QubitId::new(0));

        let channel = DepolarizingChannel::new(
            ChannelId::from_u128(1),
            "identity-depolarizing",
            support,
            Probability::ZERO,
        )
        .expect("valid channel");

        assert!(channel.is_identity());
        assert_eq!(
            channel.identity_probability(),
            Probability::ONE
        );
        assert_eq!(
            channel.shrinking_factor(),
            Probability::ONE
        );
    }

    #[test]
    fn full_probability_is_completely_depolarizing() {
        let support =
            logical_support(QubitId::new(0));

        let channel = DepolarizingChannel::new(
            ChannelId::from_u128(2),
            "full-depolarizing",
            support,
            Probability::ONE,
        )
        .expect("valid channel");

        assert!(channel.is_fully_depolarizing());
        assert_eq!(
            channel.identity_probability(),
            Probability::ZERO
        );
        assert_eq!(
            channel.shrinking_factor(),
            Probability::ZERO
        );
    }

    #[test]
    fn one_qubit_has_dimension_two() {
        let support =
            logical_support(QubitId::new(0));

        let channel = DepolarizingChannel::new(
            ChannelId::from_u128(3),
            "one-qubit",
            support,
            Probability::new(0.3).unwrap(),
        )
        .unwrap();

        assert_eq!(
            channel.dimension().unwrap(),
            2
        );

        assert_eq!(
            channel.non_identity_operator_count().unwrap(),
            3
        );
    }

    #[test]
    fn one_qubit_pauli_weight_is_p_over_three() {
        let support =
            logical_support(QubitId::new(0));

        let channel = DepolarizingChannel::new(
            ChannelId::from_u128(4),
            "one-qubit",
            support,
            Probability::new(0.3).unwrap(),
        )
        .unwrap();

        let weight =
            channel
                .non_identity_operator_probability()
                .unwrap();

        assert!(
            (weight.value() - 0.1).abs() < 1.0e-15
        );
    }

    #[test]
    fn multi_qubit_dimension_is_derived() {
        let q0 = ChannelSubsystem::new(
            ChannelSubsystemId::qubit(QubitId::new(0)),
            2,
        )
        .unwrap();

        let q1 = ChannelSubsystem::new(
            ChannelSubsystemId::qubit(QubitId::new(1)),
            2,
        )
        .unwrap();

        let support = ChannelSupport::new(
            vec![q0, q1],
            vec![q0, q1],
        )
        .unwrap();

        let channel = DepolarizingChannel::new(
            ChannelId::from_u128(5),
            "two-qubit",
            support,
            Probability::new(0.2).unwrap(),
        )
        .unwrap();

        assert_eq!(
            channel.dimension().unwrap(),
            4
        );

        assert_eq!(
            channel.non_identity_operator_count().unwrap(),
            15
        );

        let expected = 0.2 / 15.0;

        assert!(
            (channel
                .non_identity_operator_probability()
                .unwrap()
                .value()
                - expected)
                .abs()
                < 1.0e-15
        );
    }

    #[test]
    fn average_fidelity_conversion_is_inverse() {
        let support =
            logical_support(QubitId::new(0));

        let channel = DepolarizingChannel::new(
            ChannelId::from_u128(6),
            "fidelity-test",
            support,
            Probability::new(0.25).unwrap(),
        )
        .unwrap();

        let fidelity =
            channel.average_fidelity().unwrap();

        let recovered =
            DepolarizingChannel::probability_from_average_fidelity(
                2,
                fidelity,
            )
            .unwrap();

        assert!(
            (recovered.value() - 0.25).abs()
                < 1.0e-14
        );
    }

    #[test]
    fn composition_is_exact_at_semantic_level() {
        let support =
            logical_support(QubitId::new(0));

        let first = DepolarizingChannel::new(
            ChannelId::from_u128(7),
            "first",
            support.clone(),
            Probability::new(0.2).unwrap(),
        )
        .unwrap();

        let second = DepolarizingChannel::new(
            ChannelId::from_u128(8),
            "second",
            support,
            Probability::new(0.3).unwrap(),
        )
        .unwrap();

        let composed = second
            .compose_after(
                &first,
                ChannelId::from_u128(9),
                "composed",
            )
            .unwrap();

        let expected = 1.0 - 0.8 * 0.7;

        assert!(
            (composed.probability().value() - expected).abs()
                < 1.0e-15
        );
    }

    #[test]
    fn representation_does_not_materialize_pauli_terms() {
        let mut qubits = Vec::new();

        /*
         * This deliberately uses generated resource identities rather than a
         * fixed machine-size constant. The channel remains an analytical
         * object regardless of the number of resources supplied.
         */
        for index in 0..32 {
            qubits.push(QubitId::new(index));
        }

        let channel = DepolarizingChannel::from_logical_qubits(
            ChannelId::from_u128(10),
            "large-analytical-depolarizing",
            qubits,
            Probability::new(0.01).unwrap(),
        )
        .unwrap();

        assert_eq!(
            channel.dimension().unwrap(),
            1_u128 << 32
        );

        /*
         * The implementation only calculates the count. It does not allocate
         * 4^32 Pauli terms.
         */
        assert_eq!(
            channel
                .non_identity_operator_count()
                .unwrap(),
            (1_u128 << 64) - 1
        );
    }
}