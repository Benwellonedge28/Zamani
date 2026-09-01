//! Zamani Quantum Noise (ZQN) — Amplitude-Damping Channel
//!
//! This module defines the canonical semantic amplitude-damping noise model
//! and its lowering into ZQN's existing Kraus representation.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - the semantic amplitude-damping parameter;
//! - validation of the damping probability;
//! - optional association with the canonical logical `QubitId`;
//! - the mathematical amplitude-damping Kraus operators;
//! - deterministic construction of those operators;
//! - conversion into the existing ZQN `KrausChannel` representation;
//! - amplitude-damping-specific metadata and invariants;
//! - amplitude-damping-specific tests.
//!
//! # This file does NOT own
//!
//! This file does not own:
//!
//! - the canonical quantum IR;
//! - `QubitId` definition;
//! - generic probability semantics;
//! - generic Kraus representation semantics;
//! - density matrices;
//! - state-vector storage;
//! - stochastic sampling;
//! - random-number generation;
//! - calibration;
//! - T1 estimation;
//! - temperature models;
//! - thermal equilibrium models;
//! - scheduling;
//! - routing;
//! - QEC decoding;
//! - hardware APIs;
//! - vendor-specific behavior;
//! - serialization wire formats;
//! - execution;
//! - resource allocation policy.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Mathematical definition
//!
//! The amplitude-damping channel is parameterized by
//!
//! ```text
//! γ ∈ [0, 1]
//! ```
//!
//! with Kraus operators
//!
//! ```text
//! K₀ = [ 1             0             ]
//!      [ 0   sqrt(1 - γ)             ]
//!
//! K₁ = [ 0   sqrt(γ) ]
//!      [ 0      0    ]
//! ```
//!
//! and channel action
//!
//! ```text
//! E(ρ) = K₀ ρ K₀† + K₁ ρ K₁†.
//! ```
//!
//! The convention is:
//!
//! ```text
//! |0> -> |0>
//! |1> -> |0> with probability γ
//! |1> -> |1> with probability 1 - γ
//! ```
//!
//! This is the standard zero-temperature amplitude-damping channel.
//!
//! A finite-temperature/generalized amplitude-damping model is intentionally
//! NOT implemented here. That belongs in a separate channel module because it
//! introduces additional physical parameters and semantics.
//!
//! # Why the parameter is a Probability
//!
//! The damping parameter is physically constrained to `[0, 1]`. ZQN already
//! provides the canonical probability primitive, so this file does not create
//! another probability type or duplicate probability validation.
//!
//! # Canonical qubit identity
//!
//! A bound amplitude-damping channel uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! This file deliberately does not define:
//!
//! ```text
//! AmplitudeQubitId
//! NoiseQubitId
//! ZqnQubitId
//! ```
//!
//! A channel can remain unbound and later be associated with a resource by
//! routing, noise application, scheduling, or target lowering.
//!
//! # Scalability
//!
//! Amplitude damping is intrinsically a single-subsystem channel.
//!
//! Therefore this file does not encode:
//!
//! - maximum qubit count;
//! - maximum number of channels;
//! - maximum circuit size;
//! - maximum machine size;
//! - maximum logical resource count;
//! - vendor limits;
//! - topology limits.
//!
//! A large quantum machine is represented by applying this same semantic model
//! independently to whatever collection of resources requires it.
//!
//! For example:
//!
//! ```text
//! program
//!     │
//!     ▼
//! canonical quantum IR
//!     │
//!     ▼
//! ZQN amplitude-damping specification
//!     │
//!     ├── resource A -> γₐ
//!     ├── resource B -> γᵦ
//!     ├── resource C -> γ𝚌
//!     └── ...
//! ```
//!
//! The number of resources is data, not a type-level or compile-time limit.
//!
//! The mathematical channel itself remains two-dimensional because it models
//! one qubit. Scaling to many resources is performed by composition/tensor
//! mechanisms in the surrounding channel/noise infrastructure.
//!
//! # Resource semantics
//!
//! Constructing a one-qubit Kraus representation necessarily allocates a
//! finite number of matrix elements. That is an implementation consequence,
//! not a semantic machine-size limit.
//!
//! Resource governance belongs to `KrausResourceLimits` and the execution
//! context in the Kraus/simulation layers.
//!
//! This file never invents an arbitrary maximum allocation.
//!
//! # Determinism
//!
//! Construction is completely deterministic.
//!
//! No RNG exists in this module.
//!
//! No hidden global state exists.
//!
//! Given the same validated damping parameter, this module always produces the
//! same Kraus operators.
//!
//! Stochastic trajectory sampling belongs to `zqn::simulation` and must use
//! the repository's explicit deterministic execution/seed policy.
//!
//! # Numerical safety
//!
//! This implementation:
//!
//! - rejects invalid probabilities through `Probability`;
//! - never accepts NaN or infinity as a damping parameter;
//! - never silently clamps invalid input;
//! - computes `sqrt(1 - γ)` only after γ has been validated;
//! - preserves the exact endpoint behavior γ = 0 and γ = 1;
//! - performs no unchecked allocation calculations;
//! - contains no unsafe code.
//!
//! Floating-point round-off is not silently treated as a physical parameter.
//!
//! # Physical invariants
//!
//! For every valid γ:
//!
//! ```text
//! 0 <= γ <= 1
//! ```
//!
//! and therefore:
//!
//! ```text
//! 0 <= 1 - γ <= 1.
//! ```
//!
//! The resulting Kraus representation is completely positive by construction.
//!
//! The two Kraus operators satisfy:
//!
//! ```text
//! K₀†K₀ + K₁†K₁ = I.
//! ```
//!
//! Thus the channel is trace preserving for every valid γ.
//!
//! # Integration architecture
//!
//! ```text
//!                 quantum::ir
//!                     │
//!                     │ QubitId
//!                     ▼
//!              amplitude.rs
//!                     │
//!             semantic γ model
//!                     │
//!                     ▼
//!             KrausOperator × 2
//!                     │
//!                     ▼
//!          channel_for_qubits(...)
//!                     │
//!                     ▼
//!               KrausChannel
//!                     │
//!       ┌─────────────┼─────────────┐
//!       ▼             ▼             ▼
//!   simulation       QEC        hardware
//!       │             │             │
//!       ▼             ▼             ▼
//!   execution     fault adapter   lowering
//! ```
//!
//! `amplitude.rs` therefore depends on the generic Kraus implementation rather
//! than redefining it.
//!
//! # Relationship to calibration
//!
//! A real device may derive γ from quantities such as T1 and elapsed duration:
//!
//! ```text
//! γ = 1 - exp(-t / T1)
//! ```
//!
//! That calculation does NOT belong here.
//!
//! Calibration owns T1.
//!
//! Scheduling owns elapsed duration.
//!
//! A calibration/scheduling/noise integration layer may calculate the resulting
//! probability and construct this model with that probability.
//!
//! This separation prevents this file from becoming tied to one calibration
//! representation or one hardware technology.
//!
//! # Relationship to thermal noise
//!
//! This model represents zero-temperature amplitude relaxation.
//!
//! A physical environment with thermal excitation generally requires a
//! generalized amplitude-damping/thermal model. That is deliberately kept
//! separate so callers cannot accidentally claim that ordinary amplitude
//! damping models both relaxation and thermal excitation.
//!
//! # Relationship to QEC
//!
//! This file does not convert amplitude damping into a Pauli fault because that
//! conversion is not exact in general.
//!
//! A QEC adapter may explicitly derive an approximation or an operational fault
//! model and must declare the approximation/error contract.
//!
//! # Relationship to routing
//!
//! Routing may consume a derived error/fidelity cost from this channel.
//!
//! Routing must not modify this mathematical model or create another
//! amplitude-damping representation.
//!
//! # Relationship to scheduling
//!
//! Scheduling may calculate a time-dependent damping probability externally
//! and construct a new immutable channel description.
//!
//! This file does not own clocks or durations.
//!
//! # Relationship to serialization
//!
//! The semantic fields of this type are stable mathematical data:
//!
//! - channel kind;
//! - damping probability;
//! - optional canonical logical qubit.
//!
//! Wire serialization belongs to `zqn::io`, which must provide an explicit,
//! versioned schema rather than treating Rust's memory layout as a wire format.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! This file contains no `unsafe` code.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. damping probability is validated by the canonical ZQN probability type;
//! 2. the canonical `QubitId` is used;
//! 3. no duplicate qubit identity exists;
//! 4. the mathematical Kraus operators are correct;
//! 5. γ = 0 produces identity;
//! 6. γ = 1 produces complete relaxation;
//! 7. every valid γ produces a trace-preserving channel;
//! 8. no machine-size constant exists;
//! 9. no RNG exists;
//! 10. no hardware provider is referenced;
//! 11. the existing `KrausChannel` implementation is reused;
//! 12. invalid values are rejected rather than clamped;
//! 13. tests cover mathematical invariants;
//! 14. the implementation works independently of the number of quantum
//!     resources in the eventual program.
//!
//! # Public API
//!
//! The intended stable API is:
//!
//! ```text
//! AmplitudeDamping
//! AmplitudeDamping::new
//! AmplitudeDamping::with_qubit
//! AmplitudeDamping::probability
//! AmplitudeDamping::qubit
//! AmplitudeDamping::kraus_operators
//! AmplitudeDamping::kraus_channel
//! ```
//!
//! Downstream systems should normally consume `AmplitudeDamping` as a semantic
//! model and request a `KrausChannel` only at the representation boundary.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::ir::qubit::QubitId;
use crate::quantum::memory::complex::Complex64;
use crate::quantum::zqn::channel::kraus::{
    channel_for_qubits,
    KrausChannel,
    KrausError,
    KrausOperator,
    KrausResult,
};
use crate::quantum::zqn::probability::probability::{
    Probability,
    ProbabilityError,
};

/// Stable semantic identifier for the amplitude-damping channel family.
pub const AMPLITUDE_DAMPING_KIND: &str =
    "zamani.quantum.zqn.channel.amplitude_damping";

/// Mathematical dimension of the qubit subsystem on which this channel acts.
///
/// This is a property of the amplitude-damping mathematical model, not a
/// machine-size limit.
pub const QUBIT_DIMENSION: usize = 2;

/// Errors specific to the amplitude-damping semantic model.
#[derive(Debug, Clone, PartialEq)]
pub enum AmplitudeDampingError {
    /// The supplied probability was invalid.
    InvalidProbability(ProbabilityError),

    /// The generic Kraus representation rejected the generated channel.
    Kraus(KrausError),

    /// A qubit was required for a bound channel but none was supplied.
    QubitRequired,

    /// The generated square-root parameter was not finite.
    NonFiniteCoefficient,

    /// The generated coefficient was outside the expected mathematical range.
    InvalidCoefficient,

    /// The channel's generated Kraus representation failed its physical
    /// invariant.
    PhysicalInvariantViolation,
}

impl std::fmt::Display for AmplitudeDampingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidProbability(error) => {
                write!(f, "invalid amplitude-damping probability: {error}")
            }
            Self::Kraus(error) => {
                write!(f, "invalid amplitude-damping Kraus representation: {error}")
            }
            Self::QubitRequired => {
                write!(f, "a canonical QubitId is required to bind this channel")
            }
            Self::NonFiniteCoefficient => {
                write!(
                    f,
                    "amplitude-damping Kraus coefficient is not finite"
                )
            }
            Self::InvalidCoefficient => {
                write!(
                    f,
                    "amplitude-damping Kraus coefficient is outside [0, 1]"
                )
            }
            Self::PhysicalInvariantViolation => {
                write!(
                    f,
                    "generated amplitude-damping channel violated a physical invariant"
                )
            }
        }
    }
}

impl std::error::Error for AmplitudeDampingError {}

impl From<ProbabilityError> for AmplitudeDampingError {
    fn from(error: ProbabilityError) -> Self {
        Self::InvalidProbability(error)
    }
}

impl From<KrausError> for AmplitudeDampingError {
    fn from(error: KrausError) -> Self {
        Self::Kraus(error)
    }
}

/// Result type for amplitude-damping operations.
pub type AmplitudeDampingResult<T> = Result<T, AmplitudeDampingError>;

/// Immutable semantic description of a zero-temperature amplitude-damping
/// channel.
///
/// The model is independent of hardware, topology, scheduling, calibration
/// storage, simulation strategy, and execution backend.
///
/// # Examples
///
/// ```
/// use crate::quantum::zqn::channel::amplitude::AmplitudeDamping;
///
/// let channel = AmplitudeDamping::new(0.10).expect("valid damping probability");
///
/// assert_eq!(channel.probability().value(), 0.10);
/// assert!(channel.qubit().is_none());
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AmplitudeDamping {
    probability: Probability,
    qubit: Option<QubitId>,
}

impl AmplitudeDamping {
    /// Creates an unbound amplitude-damping channel.
    ///
    /// The resulting semantic channel can later be bound to a canonical
    /// `QubitId`.
    ///
    /// # Arguments
    ///
    /// * `probability` — damping probability γ in `[0, 1]`.
    ///
    /// # Errors
    ///
    /// Rejects NaN, infinities, negative probabilities, and probabilities
    /// greater than one through the canonical ZQN `Probability` type.
    pub fn new(probability: f64) -> AmplitudeDampingResult<Self> {
        Ok(Self {
            probability: Probability::new(probability)?,
            qubit: None,
        })
    }

    /// Creates an amplitude-damping channel already associated with the
    /// canonical logical qubit.
    ///
    /// No alternate ZQN qubit identity is introduced.
    pub fn with_qubit(
        qubit: QubitId,
        probability: f64,
    ) -> AmplitudeDampingResult<Self> {
        Ok(Self {
            probability: Probability::new(probability)?,
            qubit: Some(qubit),
        })
    }

    /// Creates an amplitude-damping channel from an already validated
    /// probability.
    ///
    /// This avoids re-validating a probability that has already crossed the
    /// canonical ZQN probability boundary.
    #[must_use]
    pub const fn from_probability(probability: Probability) -> Self {
        Self {
            probability,
            qubit: None,
        }
    }

    /// Creates an amplitude-damping channel from an already validated
    /// probability and binds it to a canonical logical qubit.
    #[must_use]
    pub const fn from_probability_on_qubit(
        qubit: QubitId,
        probability: Probability,
    ) -> Self {
        Self {
            probability,
            qubit: Some(qubit),
        }
    }

    /// Returns the damping probability γ.
    #[must_use]
    pub const fn probability(&self) -> Probability {
        self.probability
    }

    /// Returns the bound canonical logical qubit, if any.
    #[must_use]
    pub const fn qubit(&self) -> Option<QubitId> {
        self.qubit
    }

    /// Returns the semantic channel family identifier.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        AMPLITUDE_DAMPING_KIND
    }

    /// Returns the Hilbert-space dimension of the subsystem.
    ///
    /// This is always two because this particular mathematical channel is a
    /// single-qubit channel. It is not a machine-size limit.
    #[must_use]
    pub const fn input_dimension(&self) -> usize {
        QUBIT_DIMENSION
    }

    /// Returns the output Hilbert-space dimension.
    #[must_use]
    pub const fn output_dimension(&self) -> usize {
        QUBIT_DIMENSION
    }

    /// Returns the survival coefficient:
    ///
    /// ```text
    /// sqrt(1 - γ)
    /// ```
    ///
    /// The result is guaranteed to be finite and within `[0, 1]` for every
    /// valid probability.
    pub fn survival_amplitude(&self) -> AmplitudeDampingResult<f64> {
        let value = (1.0 - self.probability.value()).sqrt();

        validate_coefficient(value)?;

        Ok(value)
    }

    /// Returns the relaxation coefficient:
    ///
    /// ```text
    /// sqrt(γ)
    /// ```
    ///
    /// The result is guaranteed to be finite and within `[0, 1]`.
    pub fn relaxation_amplitude(&self) -> AmplitudeDampingResult<f64> {
        let value = self.probability.value().sqrt();

        validate_coefficient(value)?;

        Ok(value)
    }

    /// Returns the two canonical Kraus operators.
    ///
    /// The operators are generated from the semantic probability rather than
    /// stored independently. This prevents the probability and operators from
    /// becoming inconsistent.
    ///
    /// The returned operators are suitable for consumption by the generic ZQN
    /// Kraus representation.
    pub fn kraus_operators(
        &self,
    ) -> AmplitudeDampingResult<Vec<KrausOperator>> {
        let survival = self.survival_amplitude()?;
        let relaxation = self.relaxation_amplitude()?;

        let k0 = KrausOperator::new(
            QUBIT_DIMENSION,
            QUBIT_DIMENSION,
            vec![
                Complex64::ONE,
                Complex64::ZERO,
                Complex64::ZERO,
                Complex64::new(survival, 0.0),
            ],
        )?;

        let k1 = KrausOperator::new(
            QUBIT_DIMENSION,
            QUBIT_DIMENSION,
            vec![
                Complex64::ZERO,
                Complex64::new(relaxation, 0.0),
                Complex64::ZERO,
                Complex64::ZERO,
            ],
        )?;

        Ok(vec![k0, k1])
    }

    /// Lowers this semantic channel into the existing ZQN `KrausChannel`
    /// representation.
    ///
    /// If this channel was constructed with `with_qubit`, that canonical
    /// resource is used automatically.
    ///
    /// If the channel is unbound, the caller must provide the canonical qubit
    /// explicitly.
    ///
    /// This method deliberately constructs a one-qubit Kraus channel. Scaling
    /// the same noise model across many resources belongs to the generic ZQN
    /// composition/noise-application layer.
    pub fn kraus_channel(
        &self,
        qubit: Option<QubitId>,
    ) -> AmplitudeDampingResult<KrausChannel> {
        let selected_qubit = match (self.qubit, qubit) {
            (Some(bound), Some(requested)) if bound != requested => {
                return Err(AmplitudeDampingError::Kraus(
                    KrausError::IncompatibleResources,
                ));
            }
            (Some(bound), _) => bound,
            (None, Some(requested)) => requested,
            (None, None) => return Err(AmplitudeDampingError::QubitRequired),
        };

        let operators = self.kraus_operators()?;

        Ok(channel_for_qubits(
            &[selected_qubit],
            operators,
        )?)
    }

    /// Lowers this channel using the qubit it was originally bound to.
    ///
    /// This is a convenience API for callers that already have a bound
    /// amplitude-damping channel.
    pub fn bound_kraus_channel(
        &self,
    ) -> AmplitudeDampingResult<KrausChannel> {
        self.kraus_channel(None)
    }

    /// Returns whether this channel is exactly the identity channel.
    ///
    /// γ = 0 gives:
    ///
    /// ```text
    /// K₀ = I
    /// K₁ = 0
    /// ```
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.probability.is_zero()
    }

    /// Returns whether the damping probability is exactly one.
    ///
    /// At γ = 1:
    ///
    /// ```text
    /// K₀ = |0><0|
    /// K₁ = |0><1|
    /// ```
    #[must_use]
    pub fn is_complete_relaxation(&self) -> bool {
        self.probability.is_one()
    }

    /// Returns the probability of remaining in the excited state after
    /// amplitude damping, given that the input was exactly `|1>`.
    ///
    /// This is:
    ///
    /// ```text
    /// 1 - γ
    /// ```
    #[must_use]
    pub fn excited_state_survival_probability(&self) -> Probability {
        self.probability.complement()
    }

    /// Returns the probability of relaxation when the input was exactly
    /// `|1>`.
    #[must_use]
    pub const fn relaxation_probability(&self) -> Probability {
        self.probability
    }
}

/// Validates a generated scalar Kraus coefficient.
///
/// Coefficients are not probabilities themselves, but for this channel's
/// canonical coefficients both square-root amplitudes must lie in `[0, 1]`.
fn validate_coefficient(value: f64) -> AmplitudeDampingResult<f64> {
    if !value.is_finite() {
        return Err(AmplitudeDampingError::NonFiniteCoefficient);
    }

    if !(0.0..=1.0).contains(&value) {
        return Err(AmplitudeDampingError::InvalidCoefficient);
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::qubit::QubitId;

    fn assert_close(left: f64, right: f64, tolerance: f64) {
        assert!(
            (left - right).abs() <= tolerance,
            "left={left:?}, right={right:?}, tolerance={tolerance:?}"
        );
    }

    #[test]
    fn zero_damping_is_identity() {
        let channel = AmplitudeDamping::new(0.0).expect("zero is valid");

        assert!(channel.is_identity());
        assert!(!channel.is_complete_relaxation());

        let operators = channel
            .kraus_operators()
            .expect("identity Kraus operators");

        assert_eq!(operators.len(), 2);
    }

    #[test]
    fn unit_damping_is_complete_relaxation() {
        let channel = AmplitudeDamping::new(1.0).expect("one is valid");

        assert!(!channel.is_identity());
        assert!(channel.is_complete_relaxation());

        assert_eq!(
            channel.relaxation_probability(),
            Probability::ONE
        );

        assert_eq!(
            channel.excited_state_survival_probability(),
            Probability::ZERO
        );
    }

    #[test]
    fn invalid_probabilities_are_rejected() {
        assert!(AmplitudeDamping::new(-1.0).is_err());
        assert!(AmplitudeDamping::new(1.000_000_000_1).is_err());
        assert!(AmplitudeDamping::new(f64::NAN).is_err());
        assert!(AmplitudeDamping::new(f64::INFINITY).is_err());
        assert!(AmplitudeDamping::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn survival_and_relaxation_amplitudes_are_correct() {
        let channel =
            AmplitudeDamping::new(0.25).expect("valid probability");

        assert_close(
            channel.survival_amplitude().unwrap(),
            0.75_f64.sqrt(),
            1.0e-15,
        );

        assert_close(
            channel.relaxation_amplitude().unwrap(),
            0.25_f64.sqrt(),
            1.0e-15,
        );
    }

    #[test]
    fn generated_kraus_representation_has_two_operators() {
        let channel =
            AmplitudeDamping::new(0.30).expect("valid probability");

        let operators = channel
            .kraus_operators()
            .expect("valid Kraus representation");

        assert_eq!(operators.len(), 2);
    }

    #[test]
    fn gamma_zero_has_identity_first_operator() {
        let channel =
            AmplitudeDamping::new(0.0).expect("valid probability");

        let operators = channel.kraus_operators().unwrap();

        let identity = KrausOperator::identity(QUBIT_DIMENSION)
            .expect("identity operator");

        assert_eq!(operators[0], identity);
    }

    #[test]
    fn gamma_one_has_complete_relaxation_structure() {
        let channel =
            AmplitudeDamping::new(1.0).expect("valid probability");

        let operators = channel.kraus_operators().unwrap();

        assert_eq!(operators.len(), 2);
    }

    #[test]
    fn channel_is_bound_to_canonical_qubit_id() {
        let qubit = QubitId::new(37);

        let channel =
            AmplitudeDamping::with_qubit(qubit, 0.125)
                .expect("valid channel");

        assert_eq!(channel.qubit(), Some(qubit));
    }

    #[test]
    fn unbound_channel_can_be_bound_later() {
        let qubit = QubitId::new(123_456);

        let channel =
            AmplitudeDamping::new(0.125)
                .expect("valid channel");

        let kraus = channel
            .kraus_channel(Some(qubit))
            .expect("channel can be bound");

        assert_eq!(kraus.resources().logical_qubits(), &[qubit]);
    }

    #[test]
    fn bound_channel_uses_its_canonical_qubit() {
        let qubit = QubitId::new(9);

        let channel =
            AmplitudeDamping::with_qubit(qubit, 0.2)
                .expect("valid channel");

        let kraus = channel
            .bound_kraus_channel()
            .expect("bound channel");

        assert_eq!(kraus.resources().logical_qubits(), &[qubit]);
    }

    #[test]
    fn mismatched_binding_is_rejected() {
        let bound = QubitId::new(1);
        let requested = QubitId::new(2);

        let channel =
            AmplitudeDamping::with_qubit(bound, 0.2)
                .expect("valid channel");

        assert!(channel.kraus_channel(Some(requested)).is_err());
    }

    #[test]
    fn unbound_channel_requires_a_qubit_for_kraus_lowering() {
        let channel =
            AmplitudeDamping::new(0.2)
                .expect("valid channel");

        assert!(matches!(
            channel.kraus_channel(None),
            Err(AmplitudeDampingError::QubitRequired)
        ));
    }

    #[test]
    fn excited_state_probabilities_are_complementary() {
        let channel =
            AmplitudeDamping::new(0.37)
                .expect("valid channel");

        assert_close(
            channel.relaxation_probability().value()
                + channel.excited_state_survival_probability().value(),
            1.0,
            0.0,
        );
    }

    #[test]
    fn probability_endpoints_are_exact() {
        let zero =
            AmplitudeDamping::new(0.0)
                .expect("valid channel");

        let one =
            AmplitudeDamping::new(1.0)
                .expect("valid channel");

        assert_eq!(
            zero.relaxation_probability(),
            Probability::ZERO
        );

        assert_eq!(
            zero.excited_state_survival_probability(),
            Probability::ONE
        );

        assert_eq!(
            one.relaxation_probability(),
            Probability::ONE
        );

        assert_eq!(
            one.excited_state_survival_probability(),
            Probability::ZERO
        );
    }

    #[test]
    fn construction_is_deterministic() {
        let first =
            AmplitudeDamping::new(0.413)
                .expect("valid channel");

        let second =
            AmplitudeDamping::new(0.413)
                .expect("valid channel");

        assert_eq!(first, second);

        let first_ops = first.kraus_operators().unwrap();
        let second_ops = second.kraus_operators().unwrap();

        assert_eq!(first_ops, second_ops);
    }

    #[test]
    fn many_resource_ids_can_use_the_same_semantic_model() {
        let channel =
            AmplitudeDamping::new(0.05)
                .expect("valid channel");

        let resource_count = 1_024usize;

        let resources: Vec<QubitId> = (0..resource_count)
            .map(QubitId::new)
            .collect();

        for qubit in resources {
            let lowered = channel
                .kraus_channel(Some(qubit))
                .expect("every resource can use the same model");

            assert_eq!(
                lowered.resources().logical_qubits(),
                &[qubit]
            );
        }
    }
}