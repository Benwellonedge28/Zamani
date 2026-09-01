//! Zamani Quantum Noise (ZQN) — Phase-Damping Channels.
//!
//! # Purpose
//!
//! This module provides the production ZQN representation of exact
//! phase-damping/dephasing channels over an arbitrary finite set of canonical
//! Zamani logical qubits.
//!
//! The channel is parameterized by a coherence factor:
//!
//! ```text
//! 0 <= lambda <= 1
//! ```
//!
//! For one qubit the channel acts as:
//!
//! ```text
//! E(rho)
//!     = ((1 + lambda) / 2) rho
//!     + ((1 - lambda) / 2) Z rho Z
//! ```
//!
//! Consequently:
//!
//! ```text
//! rho_00 -> rho_00
//! rho_11 -> rho_11
//! rho_01 -> lambda * rho_01
//! rho_10 -> lambda * rho_10
//! ```
//!
//! The equivalent Kraus operators are:
//!
//! ```text
//! K_0 = sqrt((1 + lambda) / 2) I
//! K_1 = sqrt((1 - lambda) / 2) Z
//! ```
//!
//! This file stores the parameterized mathematical semantics rather than
//! materializing the corresponding matrices.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - `PhaseDampingChannel`;
//! - phase-damping coherence factors;
//! - exact phase-damping validation;
//! - per-qubit phase-damping parameters;
//! - exact phase-flip equivalent probabilities;
//! - exact coherence attenuation queries;
//! - exact phase-damping composition;
//! - exact tensor-product construction for disjoint supports;
//! - the `QuantumChannel` implementation for phase damping;
//! - deterministic factor ordering;
//! - phase-damping-specific resource descriptions.
//!
//! # Does NOT own
//!
//! This file does NOT own:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - canonical quantum IR;
//! - generic quantum-channel semantics;
//! - Kraus matrix storage;
//! - Choi matrix storage;
//! - density matrices;
//! - state-vector simulation;
//! - random sampling;
//! - Monte Carlo execution;
//! - hardware APIs;
//! - calibration storage;
//! - routing;
//! - scheduling;
//! - QEC;
//! - benchmarking;
//! - serialization formats;
//! - vendor-specific behavior;
//! - global RNG state.
//!
//! Those responsibilities belong to the appropriate ZQN or quantum subsystem.
//!
//! # Canonical quantum identity
//!
//! Logical qubits are identified exclusively through:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! This file intentionally does not define another qubit identifier.
//!
//! The current ZQN `ChannelSubsystemId` abstraction represents qubit channel
//! support through its canonical `QubitId` variant. Physical placement remains
//! the responsibility of the IR/routing/hardware layers.
//!
//! # Mathematical semantics
//!
//! For one qubit and coherence factor `lambda`:
//!
//! ```text
//! 0 <= lambda <= 1
//!
//! p_Z = (1 - lambda) / 2
//! p_I = (1 + lambda) / 2
//!
//! E(rho) = p_I rho + p_Z Z rho Z
//! ```
//!
//! The channel is therefore:
//!
//! - completely positive;
//! - trace preserving;
//! - unital;
//! - exact;
//! - Markovian as represented here;
//! - diagonal in the Pauli-transfer representation.
//!
//! Its Pauli-transfer eigenvalues are:
//!
//! ```text
//! I -> I
//! X -> lambda X
//! Y -> lambda Y
//! Z -> Z
//! ```
//!
//! # Multi-qubit semantics
//!
//! A `PhaseDampingChannel` may contain any finite number of qubits supported by
//! the available resources.
//!
//! Each supported qubit has its own coherence factor.
//!
//! For a computational-basis operator `|x><y|`, the attenuation is:
//!
//! ```text
//! product(lambda_q)
//! ```
//!
//! over supported qubits whose computational-basis values differ between `x`
//! and `y`.
//!
//! Therefore the representation does not need to materialize a `2^N x 2^N`
//! matrix.
//!
//! This is essential for the Zamani requirement:
//!
//! > A program is written once and scales to the available quantum machine.
//!
//! The representation size is proportional to the number of explicitly
//! parameterized resources, not to the exponentially large Hilbert-space
//! matrix.
//!
//! # Scalability
//!
//! This file contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_CHANNELS
//! MAX_MATRIX_DIMENSION
//! MAX_ARITY
//! MAX_TERMS
//! ```
//!
//! There is no semantic upper bound on the number of phase-damped resources.
//!
//! Actual limits arise only from:
//!
//! - available memory;
//! - CPU/GPU resources;
//! - distributed resources;
//! - execution policy;
//! - target capabilities;
//! - numerical representation.
//!
//! `usize` is used only where Rust collections require a host-sized value.
//!
//! It is never interpreted as a quantum-machine capacity.
//!
//! # Sparse parameterization
//!
//! A phase-damping channel stores only:
//!
//! ```text
//! QubitId -> coherence factor
//! ```
//!
//! No identity factors, dense Hilbert-space matrices, or exponentially large
//! channel tensors are stored.
//!
//! # Determinism
//!
//! The channel is completely deterministic.
//!
//! It contains:
//!
//! - no RNG;
//! - no global mutable state;
//! - no thread-local state;
//! - no time-derived state;
//! - no hidden calibration lookup;
//! - no hardware access.
//!
//! Factors are stored in `BTreeMap`, which gives deterministic canonical
//! ordering.
//!
//! Given identical inputs, the semantic channel is identical.
//!
//! # Numerical semantics
//!
//! Coherence factors use ZQN's canonical `Probability` type.
//!
//! Therefore every factor is:
//!
//! ```text
//! finite
//! and
//! 0 <= lambda <= 1
//! ```
//!
//! No clamping is performed.
//!
//! Invalid values are rejected at construction time.
//!
//! The phase-flip equivalent probability is:
//!
//! ```text
//! p_Z = (1 - lambda) / 2
//! ```
//!
//! which is guaranteed to lie in `[0, 1/2]` for a valid phase-damping factor.
//!
//! # Exactness
//!
//! Phase damping is represented exactly by its parameterized channel semantics.
//!
//! No numerical approximation is introduced by this representation.
//!
//! The corresponding Kraus coefficients may require square roots when a
//! numerical backend materializes Kraus matrices. That numerical realization
//! belongs to the representation/simulation layer and does not change the
//! semantic channel stored here.
//!
//! # Physicality
//!
//! Every successfully constructed phase-damping channel is physically valid.
//!
//! For each factor:
//!
//! ```text
//! p_I = (1 + lambda) / 2 >= 0
//! p_Z = (1 - lambda) / 2 >= 0
//! p_I + p_Z = 1
//! ```
//!
//! The channel is a convex mixture of two unitary channels, identity and Z.
//!
//! The tensor product of independently valid phase-damping channels is also
//! CPTP.
//!
//! Therefore this implementation reports:
//!
//! ```text
//! ChannelPhysicality::Validated
//! ```
//!
//! # Representation classification
//!
//! The canonical channel abstraction contains a `PauliTransfer` representation
//! category. Phase damping is diagonal in the Pauli-transfer basis, and this
//! implementation exposes its exact diagonal parameters without materializing
//! the full matrix.
//!
//! Therefore `representation()` returns:
//!
//! ```text
//! ChannelRepresentation::PauliTransfer
//! ```
//!
//! The actual dense Pauli-transfer matrix, when required by a simulator or
//! converter, must be materialized by the appropriate downstream subsystem.
//!
//! # Composition
//!
//! Sequential composition of two phase-damping channels over identical support
//! is exact:
//!
//! ```text
//! lambda_total = lambda_first * lambda_second
//! ```
//!
//! This follows directly from repeated attenuation of off-diagonal elements.
//!
//! Tensor-product composition is exact when supports are disjoint.
//!
//! Overlapping supports are rejected rather than silently interpreted as
//! correlated noise.
//!
//! Correlated phase noise belongs to the broader ZQN correlation/noise-model
//! subsystem.
//!
//! # Integration
//!
//! ```text
//! quantum::ir::qubit::QubitId
//!                 │
//!                 ▼
//!        ChannelSubsystemId::Qubit
//!                 │
//!                 ▼
//!          ChannelSupport
//!                 │
//!                 ▼
//!       PhaseDampingChannel
//!                 │
//!       ┌─────────┼───────────────┐
//!       ▼         ▼               ▼
//!   simulation  propagation   target lowering
//!       │         │               │
//!       ▼         ▼               ▼
//!     runtime   analysis       hardware
//!
//! PhaseDampingChannel may additionally be consumed by:
//!
//! - routing noise-cost analysis;
//! - scheduling/decoherence analysis;
//! - QEC physical-noise adapters;
//! - characterization;
//! - benchmarking;
//! - calibration-aware noise models.
//! ```
//!
//! This file does not depend on those downstream implementations.
//!
//! # Routing integration
//!
//! Routing may inspect the channel's coherence factors and derive a
//! noise-aware cost.
//!
//! Routing must not modify the phase-damping semantics.
//!
//! # Scheduling integration
//!
//! Scheduling may combine this channel with operation duration or idle time.
//!
//! A time-dependent phase-damping model belongs to the broader ZQN temporal
//! subsystem; this concrete channel remains time-independent.
//!
//! # QEC integration
//!
//! QEC may lower this channel to physical phase faults where an appropriate
//! approximation or exact equivalence is explicitly requested.
//!
//! This file does not import QEC and does not define QEC faults.
//!
//! # Hardware integration
//!
//! Hardware adapters may consume the channel's semantic descriptor and lower
//! it into target-supported representations.
//!
//! No vendor-specific API belongs here.
//!
//! # Calibration integration
//!
//! A calibration subsystem may produce coherence factors from measured
//! parameters such as dephasing time and elapsed operation time.
//!
//! The calibrated parameter must be supplied to this constructor; this file
//! does not read calibration storage.
//!
//! # Serialization
//!
//! This module deliberately does not define a wire format.
//!
//! `zqn::io` owns serialization.
//!
//! A serializer must preserve at minimum:
//!
//! - channel identity;
//! - canonical qubit identities;
//! - coherence factors;
//! - channel semantics;
//! - schema version.
//!
//! Rust struct layout is not a wire-format guarantee.
//!
//! # Security and resource safety
//!
//! This file:
//!
//! - uses safe Rust only;
//! - rejects invalid probabilities;
//! - does not perform I/O;
//! - does not access environment variables;
//! - does not access credentials;
//! - does not execute dynamic code;
//! - does not allocate exponentially sized matrices;
//! - does not create threads;
//! - does not own RNG state;
//! - does not contain global mutable state.
//!
//! Collection growth is determined by caller-provided data and available
//! resources. No artificial semantic machine-size limit is introduced.
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
//! # Definition of done
//!
//! This file is complete when:
//!
//! 1. phase damping has an explicit mathematical parameter;
//! 2. parameters use canonical ZQN probability validation;
//! 3. canonical `QubitId` is used;
//! 4. arbitrary finite support is supported;
//! 5. no machine-size limit is encoded;
//! 6. no dense exponential matrix is required;
//! 7. physical validity is explicit;
//! 8. exactness is explicit;
//! 9. deterministic ordering is guaranteed;
//! 10. exact sequential composition is available;
//! 11. exact tensor composition is available;
//! 12. overlapping tensor supports are rejected;
//! 13. phase-flip equivalence is available;
//! 14. coherence attenuation can be queried;
//! 15. Pauli-transfer parameters can be queried;
//! 16. `QuantumChannel` is implemented;
//! 17. no RNG is hidden here;
//! 18. no hardware dependency exists;
//! 19. no QEC dependency exists;
//! 20. no unsafe code exists;
//! 21. Rust 1.97/1.97.1 is sufficient.
//!
//! # External mathematical reference
//!
//! The representation is consistent with the standard Kraus-channel
//! formulation in which a quantum channel is expressed as a sum of Kraus
//! conjugations satisfying the trace-preservation condition.
//!
//! IBM Quantum's channel documentation describes this Kraus formulation and
//! its dephasing-channel construction.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::quantum::ir::qubit::QubitId;

use crate::quantum::zqn::channel::channel::{
    ChannelAccuracy,
    ChannelCapability,
    ChannelDescriptor,
    ChannelError,
    ChannelId,
    ChannelPhysicality,
    ChannelRepresentation,
    ChannelResourceRequirements,
    ChannelResult,
    ChannelSubsystem,
    ChannelSubsystemId,
    ChannelSupport,
    QuantumChannel,
};

use crate::quantum::zqn::probability::Probability;

/// Required capabilities of an exact phase-damping channel.
///
/// The array is immutable and contains no mutable global state.
const REQUIRED_CAPABILITIES: [ChannelCapability; 1] =
    [ChannelCapability::ExactRepresentation];

/// Exact parameterized phase-damping channel.
///
/// Each supported canonical logical qubit has one coherence factor.
///
/// For qubit `q`:
///
/// ```text
/// lambda_q = coherence_factor(q)
/// ```
///
/// The channel attenuates off-diagonal coherence involving that resource by
/// `lambda_q`.
///
/// The implementation is sparse in the number of explicitly supported
/// resources and does not construct a dense quantum-state or channel matrix.
#[derive(Debug, Clone, PartialEq)]
pub struct PhaseDampingChannel {
    descriptor: ChannelDescriptor,

    /// Deterministically ordered per-qubit coherence factors.
    factors: BTreeMap<QubitId, Probability>,
}

impl PhaseDampingChannel {
    /// Creates an exact phase-damping channel from per-qubit coherence
    /// factors.
    ///
    /// # Arguments
    ///
    /// `id`
    /// : Stable semantic channel identity supplied by the caller.
    ///
    /// `factors`
    /// : `(QubitId, coherence_factor)` pairs.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - no factors are supplied;
    /// - the same qubit occurs more than once;
    /// - the resulting channel support cannot be constructed.
    ///
    /// Probability validation is delegated to the canonical ZQN
    /// `Probability` type.
    pub fn new<I>(id: ChannelId, factors: I) -> ChannelResult<Self>
    where
        I: IntoIterator<Item = (QubitId, Probability)>,
    {
        let mut map = BTreeMap::new();

        for (qubit, factor) in factors {
            if map.insert(qubit, factor).is_some() {
                return Err(ChannelError::DuplicateQubit(qubit));
            }
        }

        if map.is_empty() {
            return Err(ChannelError::EmptySupport);
        }

        Self::from_factor_map(id, map)
    }

    /// Creates a one-qubit phase-damping channel.
    pub fn single(
        id: ChannelId,
        qubit: QubitId,
        coherence_factor: Probability,
    ) -> ChannelResult<Self> {
        Self::new(id, [(qubit, coherence_factor)])
    }

    /// Creates a phase-damping channel with one common coherence factor for
    /// every supplied qubit.
    ///
    /// The number of qubits is determined entirely by the supplied iterator.
    /// No fixed machine-size assumption is made.
    pub fn uniform<I>(
        id: ChannelId,
        qubits: I,
        coherence_factor: Probability,
    ) -> ChannelResult<Self>
    where
        I: IntoIterator<Item = QubitId>,
    {
        let mut map = BTreeMap::new();

        for qubit in qubits {
            if map.insert(qubit, coherence_factor).is_some() {
                return Err(ChannelError::DuplicateQubit(qubit));
            }
        }

        if map.is_empty() {
            return Err(ChannelError::EmptySupport);
        }

        Self::from_factor_map(id, map)
    }

    /// Creates a phase-damping channel from equivalent phase-flip
    /// probabilities.
    ///
    /// The relationship is:
    ///
    /// ```text
    /// p_Z = (1 - lambda) / 2
    ///
    /// lambda = 1 - 2 p_Z
    /// ```
    ///
    /// Therefore a phase-flip probability greater than `1/2` does not
    /// correspond to the conventional non-negative phase-damping coherence
    /// factor represented by this type and is rejected.
    pub fn from_phase_flip_probabilities<I>(
        id: ChannelId,
        probabilities: I,
    ) -> ChannelResult<Self>
    where
        I: IntoIterator<Item = (QubitId, Probability)>,
    {
        let factors = probabilities
            .into_iter()
            .map(|(qubit, probability)| {
                let value = probability.value();

                if value > 0.5 {
                    return Err(ChannelError::ParameterOutOfRange);
                }

                let lambda = 1.0 - (2.0 * value);

                let factor = Probability::new(lambda)
                    .map_err(|_| ChannelError::NonFiniteParameter)?;

                Ok((qubit, factor))
            })
            .collect::<ChannelResult<Vec<_>>>()?;

        Self::new(id, factors)
    }

    /// Returns the stable semantic channel identity.
    #[must_use]
    pub fn channel_id(&self) -> ChannelId {
        self.descriptor.id
    }

    /// Returns the number of explicitly parameterized qubits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.factors.len()
    }

    /// Returns whether the channel has no factors.
    ///
    /// A valid `PhaseDampingChannel` can never be empty, so this method is
    /// provided for collection-like API consistency and is always false for a
    /// successfully constructed value.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.factors.is_empty()
    }

    /// Returns the coherence factor for one qubit.
    ///
    /// `None` means that the channel does not model phase damping on that
    /// resource.
    #[must_use]
    pub fn coherence_factor(&self, qubit: QubitId) -> Option<Probability> {
        self.factors.get(&qubit).copied()
    }

    /// Returns all coherence factors in canonical deterministic order.
    ///
    /// The iterator does not expose mutable access, preserving channel
    /// invariants.
    pub fn factors(&self) -> impl Iterator<Item = (QubitId, Probability)> + '_ {
        self.factors.iter().map(|(&qubit, &factor)| (qubit, factor))
    }

    /// Returns the equivalent phase-flip probability for one qubit.
    ///
    /// For coherence factor `lambda`:
    ///
    /// ```text
    /// p_Z = (1 - lambda) / 2
    /// ```
    pub fn phase_flip_probability(
        &self,
        qubit: QubitId,
    ) -> ChannelResult<Option<Probability>> {
        let Some(lambda) = self.coherence_factor(qubit) else {
            return Ok(None);
        };

        let value = (1.0 - lambda.value()) / 2.0;

        Probability::new(value)
            .map(Some)
            .map_err(|_| ChannelError::NonFiniteParameter)
    }

    /// Returns the equivalent identity and Z probabilities for one qubit.
    ///
    /// The returned pair is:
    ///
    /// ```text
    /// (p_I, p_Z)
    /// ```
    ///
    /// with:
    ///
    /// ```text
    /// p_I = (1 + lambda) / 2
    /// p_Z = (1 - lambda) / 2
    /// ```
    pub fn pauli_probabilities(
        &self,
        qubit: QubitId,
    ) -> ChannelResult<Option<(Probability, Probability)>> {
        let Some(lambda) = self.coherence_factor(qubit) else {
            return Ok(None);
        };

        let p_identity = Probability::new((1.0 + lambda.value()) / 2.0)
            .map_err(|_| ChannelError::NonFiniteParameter)?;

        let p_z = Probability::new((1.0 - lambda.value()) / 2.0)
            .map_err(|_| ChannelError::NonFiniteParameter)?;

        Ok(Some((p_identity, p_z)))
    }

    /// Returns the Kraus coefficient magnitudes for one qubit.
    ///
    /// The returned pair is:
    ///
    /// ```text
    /// (
    ///     sqrt((1 + lambda) / 2),
    ///     sqrt((1 - lambda) / 2),
    /// )
    /// ```
    ///
    /// These are coefficient magnitudes for `I` and `Z`, respectively.
    ///
    /// This method does not materialize matrices.
    pub fn kraus_coefficients(
        &self,
        qubit: QubitId,
    ) -> ChannelResult<Option<(f64, f64)>> {
        let Some((p_identity, p_z)) = self.pauli_probabilities(qubit)? else {
            return Ok(None);
        };

        Ok(Some((
            p_identity.value().sqrt(),
            p_z.value().sqrt(),
        )))
    }

    /// Returns the exact Pauli-transfer diagonal for one supported qubit.
    ///
    /// The returned tuple is ordered as:
    ///
    /// ```text
    /// (I, X, Y, Z)
    /// ```
    ///
    /// and equals:
    ///
    /// ```text
    /// (1, lambda, lambda, 1)
    /// ```
    pub fn pauli_transfer_diagonal(
        &self,
        qubit: QubitId,
    ) -> Option<(f64, f64, f64, f64)> {
        self.coherence_factor(qubit).map(|factor| {
            let lambda = factor.value();
            (1.0, lambda, lambda, 1.0)
        })
    }

    /// Returns the exact attenuation applied to coherence involving the
    /// supplied set of differing qubits.
    ///
    /// If a qubit is not explicitly modeled by this channel, it contributes an
    /// attenuation factor of `1`.
    ///
    /// The input is interpreted as a set. Duplicate resource identities are
    /// rejected so that caller mistakes cannot silently alter the result.
    pub fn coherence_attenuation(
        &self,
        differing_qubits: &[QubitId],
    ) -> ChannelResult<f64> {
        let mut seen = BTreeSet::new();
        let mut attenuation = 1.0_f64;

        for &qubit in differing_qubits {
            if !seen.insert(qubit) {
                return Err(ChannelError::DuplicateQubit(qubit));
            }

            if let Some(factor) = self.factors.get(&qubit) {
                attenuation *= factor.value();
            }
        }

        if !attenuation.is_finite()
            || !(0.0..=1.0).contains(&attenuation)
        {
            return Err(ChannelError::NonFiniteParameter);
        }

        Ok(attenuation)
    }

    /// Returns true when every supported qubit has coherence factor `1`.
    ///
    /// Such a channel is exactly the identity channel.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.factors
            .values()
            .all(|factor| factor.is_one())
    }

    /// Returns true when at least one supported qubit is completely dephased.
    ///
    /// A zero coherence factor means that off-diagonal coherence involving
    /// that qubit is completely removed.
    #[must_use]
    pub fn contains_complete_dephasing(&self) -> bool {
        self.factors.values().any(|factor| factor.is_zero())
    }

    /// Performs exact sequential composition of two phase-damping channels
    /// over the same support.
    ///
    /// If:
    ///
    /// ```text
    /// E1(lambda_1)
    /// E2(lambda_2)
    /// ```
    ///
    /// are applied sequentially, the result is:
    ///
    /// ```text
    /// E(lambda_1 * lambda_2)
    /// ```
    ///
    /// The caller supplies the identity of the resulting channel.
    pub fn compose_sequential(
        &self,
        next: &Self,
        result_id: ChannelId,
    ) -> ChannelResult<Self> {
        if self.factors.len() != next.factors.len() {
            return Err(ChannelError::DomainMismatch {
                input_arity: self.factors.len(),
                output_arity: next.factors.len(),
            });
        }

        if self
            .factors
            .keys()
            .ne(next.factors.keys())
        {
            return Err(ChannelError::IncompatibleComposition);
        }

        let factors = self
            .factors
            .iter()
            .map(|(&qubit, &first)| {
                let second = next
                    .factors
                    .get(&qubit)
                    .copied()
                    .ok_or(ChannelError::IncompatibleComposition)?;

                let combined = first.multiply(second);

                Ok((qubit, combined))
            })
            .collect::<ChannelResult<Vec<_>>>()?;

        Self::new(result_id, factors)
    }

    /// Performs an exact tensor-product composition with another
    /// phase-damping channel.
    ///
    /// Tensor composition requires disjoint supports. Overlapping resources
    /// are rejected because overlap is not independent tensor-product
    /// semantics.
    pub fn tensor(
        &self,
        other: &Self,
        result_id: ChannelId,
    ) -> ChannelResult<Self> {
        let mut factors = self.factors.clone();

        for (&qubit, &factor) in &other.factors {
            if factors.insert(qubit, factor).is_some() {
                return Err(ChannelError::DuplicateQubit(qubit));
            }
        }

        Self::new(result_id, factors)
    }

    /// Returns a copy with one factor replaced.
    ///
    /// Replacing an existing factor preserves the support.
    ///
    /// This creates a new channel rather than mutating an existing channel,
    /// keeping channel values immutable after construction.
    pub fn with_factor(
        &self,
        result_id: ChannelId,
        qubit: QubitId,
        factor: Probability,
    ) -> ChannelResult<Self> {
        let mut factors = self.factors.clone();

        factors.insert(qubit, factor);

        Self::new(result_id, factors)
    }

    /// Validates the mathematical phase-damping invariants.
    ///
    /// This method is intentionally explicit even though the private
    /// representation and constructor already maintain the invariants.
    pub fn validate_physical_phase_damping(&self) -> ChannelResult<()> {
        self.validate()?;

        if self.factors.is_empty() {
            return Err(ChannelError::EmptySupport);
        }

        for factor in self.factors.values() {
            let lambda = factor.value();

            if !lambda.is_finite() {
                return Err(ChannelError::NonFiniteParameter);
            }

            if !(0.0..=1.0).contains(&lambda) {
                return Err(ChannelError::ParameterOutOfRange);
            }

            let p_identity = (1.0 + lambda) / 2.0;
            let p_z = (1.0 - lambda) / 2.0;

            if !p_identity.is_finite()
                || !p_z.is_finite()
                || p_identity < 0.0
                || p_z < 0.0
                || p_identity > 1.0
                || p_z > 1.0
            {
                return Err(ChannelError::ParameterOutOfRange);
            }

            let normalization = p_identity + p_z;

            if !normalization.is_finite()
                || (normalization - 1.0).abs() > f64::EPSILON
            {
                return Err(ChannelError::PropertyUndetermined(
                    "phase-damping Kraus weights do not normalize",
                ));
            }
        }

        Ok(())
    }

    /// Builds the channel's canonical representation-independent support and
    /// descriptor.
    fn from_factor_map(
        id: ChannelId,
        factors: BTreeMap<QubitId, Probability>,
    ) -> ChannelResult<Self> {
        if factors.is_empty() {
            return Err(ChannelError::EmptySupport);
        }

        let mut support_subsystems = Vec::with_capacity(factors.len());

        for &qubit in factors.keys() {
            support_subsystems.push(ChannelSubsystem::new(
                ChannelSubsystemId::qubit(qubit),
                2,
            )?);
        }

        let support = ChannelSupport::square(support_subsystems)?;

        let scalar_elements = Some(factors.len() as u128);

        let resources = ChannelResourceRequirements::known(
            scalar_elements,
            None,
            None,
        );

        let descriptor = ChannelDescriptor::new(
            id,
            Some(String::from("phase_damping")),
            support,
            ChannelRepresentation::PauliTransfer,
            ChannelPhysicality::Validated,
            ChannelAccuracy::Exact,
            resources,
        )?;

        let channel = Self {
            descriptor,
            factors,
        };

        channel.validate_physical_phase_damping()?;

        Ok(channel)
    }
}

impl QuantumChannel for PhaseDampingChannel {
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
        ChannelRepresentation::PauliTransfer
    }

    fn physicality(&self) -> ChannelPhysicality {
        ChannelPhysicality::Validated
    }

    fn accuracy(&self) -> ChannelAccuracy {
        ChannelAccuracy::Exact
    }

    fn required_capabilities(&self) -> &[ChannelCapability] {
        &REQUIRED_CAPABILITIES
    }

    fn resource_requirements(&self) -> ChannelResourceRequirements {
        self.descriptor.resources
    }

    fn validate(&self) -> ChannelResult<()> {
        self.descriptor.support.validate()?;

        if self.factors.is_empty() {
            return Err(ChannelError::EmptySupport);
        }

        if self.factors.len() != self.descriptor.support.input_arity()
            || self.factors.len() != self.descriptor.support.output_arity()
        {
            return Err(ChannelError::DomainMismatch {
                input_arity: self.descriptor.support.input_arity(),
                output_arity: self.descriptor.support.output_arity(),
            });
        }

        for (&qubit, factor) in &self.factors {
            if !factor.value().is_finite() {
                return Err(ChannelError::NonFiniteParameter);
            }

            if !(0.0..=1.0).contains(&factor.value()) {
                return Err(ChannelError::ParameterOutOfRange);
            }

            if !self
                .descriptor
                .support
                .input_qubits()
                .any(|candidate| candidate == qubit)
            {
                return Err(ChannelError::IncompatibleComposition);
            }
        }

        Ok(())
    }

    fn validate_physicality(&self) -> ChannelResult<()> {
        self.validate_physical_phase_damping()
    }
}

impl fmt::Display for PhaseDampingChannel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "PhaseDampingChannel(id={}, qubits={}, representation={})",
            self.id(),
            self.len(),
            self.representation()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    fn id(value: u128) -> ChannelId {
        ChannelId::from_u128(value)
    }

    fn probability(value: f64) -> Probability {
        Probability::new(value).expect("test probability must be valid")
    }

    #[test]
    fn single_qubit_channel_is_valid() {
        let channel =
            PhaseDampingChannel::single(id(1), q(0), probability(0.8))
                .expect("valid phase-damping channel");

        assert_eq!(channel.len(), 1);
        assert_eq!(
            channel.coherence_factor(q(0)).expect("factor").value(),
            0.8
        );

        assert!(channel.is_physical());
        assert!(channel.is_exact());
        assert_eq!(
            channel.representation(),
            ChannelRepresentation::PauliTransfer
        );
    }

    #[test]
    fn zero_coherence_factor_is_complete_dephasing() {
        let channel =
            PhaseDampingChannel::single(id(2), q(0), Probability::ZERO)
                .expect("valid channel");

        assert!(channel.contains_complete_dephasing());
        assert!(!channel.is_identity());

        let attenuation = channel
            .coherence_attenuation(&[q(0)])
            .expect("valid coherence query");

        assert_eq!(attenuation, 0.0);
    }

    #[test]
    fn unit_coherence_factor_is_identity() {
        let channel =
            PhaseDampingChannel::single(id(3), q(0), Probability::ONE)
                .expect("valid channel");

        assert!(channel.is_identity());

        let attenuation = channel
            .coherence_attenuation(&[q(0)])
            .expect("valid coherence query");

        assert_eq!(attenuation, 1.0);
    }

    #[test]
    fn equivalent_phase_flip_probability_is_correct() {
        let channel =
            PhaseDampingChannel::single(id(4), q(0), probability(0.8))
                .expect("valid channel");

        let p_z = channel
            .phase_flip_probability(q(0))
            .expect("query succeeds")
            .expect("qubit is modeled");

        assert!((p_z.value() - 0.1).abs() < f64::EPSILON);

        let probabilities = channel
            .pauli_probabilities(q(0))
            .expect("query succeeds")
            .expect("qubit is modeled");

        assert!((probabilities.0.value() - 0.9).abs() < f64::EPSILON);
        assert!((probabilities.1.value() - 0.1).abs() < f64::EPSILON);
    }

    #[test]
    fn kraus_coefficients_are_exactly_normalized() {
        let channel =
            PhaseDampingChannel::single(id(5), q(0), probability(0.8))
                .expect("valid channel");

        let (a, b) = channel
            .kraus_coefficients(q(0))
            .expect("query succeeds")
            .expect("qubit is modeled");

        let normalization = (a * a) + (b * b);

        assert!((normalization - 1.0).abs() <= f64::EPSILON);
    }

    #[test]
    fn pauli_transfer_diagonal_is_correct() {
        let channel =
            PhaseDampingChannel::single(id(6), q(0), probability(0.75))
                .expect("valid channel");

        assert_eq!(
            channel.pauli_transfer_diagonal(q(0)),
            Some((1.0, 0.75, 0.75, 1.0))
        );
    }

    #[test]
    fn multi_qubit_channel_is_sparse_and_deterministic() {
        let channel = PhaseDampingChannel::new(
            id(7),
            [
                (q(100), probability(0.8)),
                (q(2), probability(0.5)),
                (q(1000), probability(0.25)),
            ],
        )
        .expect("valid channel");

        let resources: Vec<_> = channel
            .factors()
            .map(|(qubit, _)| qubit)
            .collect();

        assert_eq!(resources, vec![q(2), q(100), q(1000)]);
    }

    #[test]
    fn coherence_attenuation_multiplies_independent_factors() {
        let channel = PhaseDampingChannel::new(
            id(8),
            [
                (q(0), probability(0.8)),
                (q(1), probability(0.5)),
                (q(2), probability(0.25)),
            ],
        )
        .expect("valid channel");

        let attenuation = channel
            .coherence_attenuation(&[q(0), q(1), q(2)])
            .expect("valid coherence query");

        assert!((attenuation - 0.1).abs() < 1.0e-15);
    }

    #[test]
    fn unmodeled_qubit_has_unit_attenuation() {
        let channel =
            PhaseDampingChannel::single(id(9), q(0), probability(0.5))
                .expect("valid channel");

        let attenuation = channel
            .coherence_attenuation(&[q(999)])
            .expect("valid coherence query");

        assert_eq!(attenuation, 1.0);
    }

    #[test]
    fn duplicate_qubits_are_rejected() {
        let result = PhaseDampingChannel::new(
            id(10),
            [
                (q(0), probability(0.8)),
                (q(0), probability(0.5)),
            ],
        );

        assert!(matches!(
            result,
            Err(ChannelError::DuplicateQubit(qubit)) if qubit == q(0)
        ));
    }

    #[test]
    fn empty_channel_is_rejected() {
        let result =
            PhaseDampingChannel::new(id(11), std::iter::empty());

        assert!(matches!(result, Err(ChannelError::EmptySupport)));
    }

    #[test]
    fn sequential_composition_multiplies_coherence_factors() {
        let first =
            PhaseDampingChannel::single(id(12), q(0), probability(0.8))
                .expect("valid channel");

        let second =
            PhaseDampingChannel::single(id(13), q(0), probability(0.5))
                .expect("valid channel");

        let composed = first
            .compose_sequential(&second, id(14))
            .expect("compatible channels");

        assert_eq!(
            composed.coherence_factor(q(0)).expect("factor").value(),
            0.4
        );
    }

    #[test]
    fn sequential_composition_requires_identical_support() {
        let first =
            PhaseDampingChannel::single(id(15), q(0), probability(0.8))
                .expect("valid channel");

        let second =
            PhaseDampingChannel::single(id(16), q(1), probability(0.5))
                .expect("valid channel");

        let result = first.compose_sequential(&second, id(17));

        assert!(matches!(
            result,
            Err(ChannelError::IncompatibleComposition)
        ));
    }

    #[test]
    fn tensor_product_requires_disjoint_support() {
        let first =
            PhaseDampingChannel::single(id(18), q(0), probability(0.8))
                .expect("valid channel");

        let second =
            PhaseDampingChannel::single(id(19), q(1), probability(0.5))
                .expect("valid channel");

        let tensor = first
            .tensor(&second, id(20))
            .expect("disjoint supports");

        assert_eq!(tensor.len(), 2);
        assert_eq!(
            tensor.coherence_factor(q(0)).expect("factor").value(),
            0.8
        );
        assert_eq!(
            tensor.coherence_factor(q(1)).expect("factor").value(),
            0.5
        );
    }

    #[test]
    fn tensor_product_rejects_overlapping_support() {
        let first =
            PhaseDampingChannel::single(id(21), q(0), probability(0.8))
                .expect("valid channel");

        let second =
            PhaseDampingChannel::single(id(22), q(0), probability(0.5))
                .expect("valid channel");

        let result = first.tensor(&second, id(23));

        assert!(matches!(
            result,
            Err(ChannelError::DuplicateQubit(qubit)) if qubit == q(0)
        ));
    }

    #[test]
    fn phase_flip_probability_constructor_is_inverse() {
        let channel = PhaseDampingChannel::from_phase_flip_probabilities(
            id(24),
            [(q(0), probability(0.1))],
        )
        .expect("valid phase-flip probability");

        let lambda = channel
            .coherence_factor(q(0))
            .expect("factor")
            .value();

        assert!((lambda - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    fn phase_flip_probability_above_half_is_rejected() {
        let result = PhaseDampingChannel::from_phase_flip_probabilities(
            id(25),
            [(q(0), probability(0.5000000001))],
        );

        assert!(matches!(
            result,
            Err(ChannelError::ParameterOutOfRange)
        ));
    }

    #[test]
    fn duplicate_coherence_query_is_rejected() {
        let channel =
            PhaseDampingChannel::single(id(26), q(0), probability(0.8))
                .expect("valid channel");

        let result = channel.coherence_attenuation(&[q(0), q(0)]);

        assert!(matches!(
            result,
            Err(ChannelError::DuplicateQubit(qubit)) if qubit == q(0)
        ));
    }

    #[test]
    fn physical_validation_succeeds_for_extreme_values() {
        let zero =
            PhaseDampingChannel::single(id(27), q(0), Probability::ZERO)
                .expect("valid channel");

        let one =
            PhaseDampingChannel::single(id(28), q(0), Probability::ONE)
                .expect("valid channel");

        zero.validate_physicality()
            .expect("complete dephasing is CPTP");

        one.validate_physicality()
            .expect("identity is CPTP");
    }

    #[test]
    fn required_capability_is_exact_representation() {
        let channel =
            PhaseDampingChannel::single(id(29), q(0), probability(0.7))
                .expect("valid channel");

        assert!(channel.requires_capability(
            ChannelCapability::ExactRepresentation
        ));

        assert!(!channel.requires_capability(
            ChannelCapability::CorrelatedNoise
        ));
    }

    #[test]
    fn descriptor_is_representation_independent() {
        let channel =
            PhaseDampingChannel::single(id(30), q(0), probability(0.7))
                .expect("valid channel");

        let descriptor = channel.descriptor();

        assert_eq!(descriptor.id, id(30));
        assert_eq!(
            descriptor.representation,
            ChannelRepresentation::PauliTransfer
        );
        assert_eq!(
            descriptor.physicality,
            ChannelPhysicality::Validated
        );
        assert!(descriptor.accuracy.is_exact());
        assert_eq!(
            descriptor.resources.scalar_elements,
            Some(1)
        );
    }

    #[test]
    fn canonical_qubit_identity_is_used() {
        let qubit = crate::quantum::ir::qubit::QubitId::new(41);

        let channel =
            PhaseDampingChannel::single(id(31), qubit, probability(0.9))
                .expect("valid channel");

        assert_eq!(
            channel.input_qubits(),
            vec![qubit]
        );
    }

    #[test]
    fn arbitrary_resource_indices_do_not_change_semantics() {
        let small =
            PhaseDampingChannel::single(id(32), q(0), probability(0.9))
                .expect("valid channel");

        let large =
            PhaseDampingChannel::single(
                id(33),
                q(usize::MAX),
                probability(0.9),
            )
            .expect("valid channel");

        assert_eq!(
            small.coherence_factor(q(0)),
            large.coherence_factor(q(usize::MAX))
        );
    }

    #[test]
    fn channel_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<PhaseDampingChannel>();
    }
}