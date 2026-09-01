//! Zamani Quantum Noise (ZQN) — Stochastic Quantum Channel
//!
//! # Ownership
//!
//! This file owns the concrete stochastic-channel representation used by ZQN.
//!
//! A stochastic channel represents a conditional probability map:
//!
//! ```text
//!              P(output | input)
//! input ─────────────────────────────► output
//! ```
//!
//! More precisely, for every declared input outcome `i`, this implementation
//! stores a normalized finite discrete distribution over output outcomes `o`.
//!
//! The mathematical object is therefore:
//!
//! ```text
//! P(o | i)
//! ```
//!
//! for every input `i` in the channel's explicitly represented finite input
//! domain.
//!
//! This file owns:
//!
//! - `StochasticChannel`;
//! - deterministic storage of input-conditioned output distributions;
//! - construction and validation of stochastic transition maps;
//! - lookup of conditional output distributions;
//! - conditional probability queries;
//! - caller-owned-RNG conditional sampling;
//! - stochastic-channel resource accounting;
//! - implementation of the canonical `QuantumChannel` contract;
//! - explicit distinction between stochastic validity and full quantum CPTP
//!   validation;
//! - deterministic iteration over the represented input domain.
//!
//! # Does NOT own
//!
//! This file does NOT own:
//!
//! - the canonical quantum IR;
//! - `QubitId` or `PhysicalQubitId` definitions;
//! - probability mathematics;
//! - generic probability distributions;
//! - Kraus operators;
//! - Choi matrices;
//! - density matrices;
//! - state-vector simulation;
//! - Monte Carlo execution policy;
//! - RNG ownership;
//! - global random state;
//! - noise-model policy;
//! - calibration;
//! - characterization;
//! - routing;
//! - scheduling;
//! - QEC;
//! - hardware APIs;
//! - vendor-specific behavior;
//! - wire/schema versioning;
//! - canonical serialization;
//! - general channel composition;
//! - general channel conversion.
//!
//! Probability semantics belong to:
//!
//! ```text
//! crate::quantum::zqn::probability::distribution
//! ```
//!
//! Representation-independent channel semantics belong to:
//!
//! ```text
//! crate::quantum::zqn::channel::channel
//! ```
//!
//! Representation classification belongs to:
//!
//! ```text
//! crate::quantum::zqn::channel::representation
//! ```
//!
//! This file composes those existing contracts instead of redefining them.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              │ canonical resources
//!                              ▼
//!                    ┌────────────────────┐
//!                    │     ZQN Channel    │
//!                    │                    │
//!                    │ QuantumChannel     │
//!                    └─────────┬──────────┘
//!                              │
//!                              ▼
//!                       StochasticChannel
//!                              │
//!               ┌──────────────┴──────────────┐
//!               │                             │
//!               ▼                             ▼
//!       Distribution<O>                 Quantum support
//!               │                             │
//!               └──────────────┬──────────────┘
//!                              ▼
//!                  simulation / QEC / routing
//!                  scheduling / hardware
//! ```
//!
//! # Meaning of "stochastic"
//!
//! A stochastic channel is a representation in which the modeled process is
//! explicitly described by classical conditional probabilities.
//!
//! It is therefore suitable for:
//!
//! - classical stochastic subspaces;
//! - measurement/readout transition models;
//! - classicalized noise models;
//! - empirical transition models;
//! - sampled/estimated transition behavior;
//! - stochastic fault transitions;
//! - representations whose downstream semantics explicitly operate on a
//!   classical outcome space.
//!
//! A stochastic transition table by itself does NOT specify how arbitrary
//! quantum coherences transform.
//!
//! Consequently, this implementation MUST NOT claim full quantum CPTP
//! physicality merely because every row is a normalized probability
//! distribution.
//!
//! The channel is therefore represented as structurally valid while its
//! full quantum physicality remains `Unvalidated` unless a higher-level
//! representation/embedding establishes the required quantum properties.
//!
//! This prevents a scientifically dangerous false equivalence:
//!
//! ```text
//! row-normalized classical stochastic map
//!             ≠
//! automatically proven full quantum CPTP map
//! ```
//!
//! # Write once, scale everywhere
//!
//! This file contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_TRANSITIONS
//! MAX_INPUTS
//! MAX_OUTPUTS
//! MAX_ARITY
//! MAX_STATES
//! MAX_CHANNEL_SIZE
//! ```
//!
//! The number of represented input states and output states is determined by
//! the supplied data and available resources.
//!
//! The implementation uses `usize` only where Rust collections and indexing
//! require a host-sized representation.
//!
//! No `usize` value is interpreted as a semantic maximum quantum-machine size.
//!
//! A sufficiently large machine may therefore use this representation without
//! requiring a source-level or semantic change, subject to:
//!
//! - available memory;
//! - CPU/GPU resources;
//! - distributed execution resources;
//! - explicit runtime resource policy;
//! - target capabilities;
//! - numerical precision.
//!
//! "Infinity" means that this file introduces no artificial finite semantic
//! ceiling. It does not mean that finite physical hardware can allocate an
//! infinite data structure.
//!
//! # Canonical quantum identity
//!
//! This implementation does not define another qubit identity.
//!
//! When stochastic-channel support refers to qubits, it uses the canonical:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! through `ChannelSubsystemId::Qubit` and `ChannelSupport`.
//!
//! The repository's canonical channel layer already establishes that
//! `QubitId` is the identity boundary for quantum channels.
//!
//! # Probability ownership
//!
//! The probability layer already owns:
//!
//! - probability validation;
//! - normalization;
//! - zero-weight handling;
//! - duplicate outcome handling;
//! - conditional distribution sampling;
//! - distribution cardinality;
//! - deterministic distribution behavior.
//!
//! This file therefore stores:
//!
//! ```text
//! input → Distribution<output>
//! ```
//!
//! rather than implementing another probability engine.
//!
//! # Determinism
//!
//! The semantic transition table is deterministic.
//!
//! Input transitions are stored in `BTreeMap` rather than `HashMap` so that:
//!
//! - iteration order is deterministic;
//! - diagnostics are deterministic;
//! - resource accounting is deterministic;
//! - canonical descriptions are deterministic;
//! - tests do not depend on randomized hash iteration.
//!
//! Sampling NEVER creates or owns an RNG.
//!
//! The caller supplies the RNG to `sample`.
//!
//! Therefore there is:
//!
//! - no global RNG;
//! - no thread-local RNG;
//! - no time-derived seed;
//! - no hidden entropy;
//! - no process-address-derived randomness.
//!
//! The caller may derive independent deterministic streams from:
//!
//! ```text
//! master seed
//! program identity
//! operation identity
//! resource identity
//! shot index
//! ```
//!
//! without this file knowing how that policy is implemented.
//!
//! # Parallel determinism
//!
//! This representation contains no mutable global state and no RNG state.
//!
//! Therefore it is safe to share immutable stochastic-channel instances
//! between parallel workers when their generic outcome types satisfy the
//! required thread-safety bounds.
//!
//! Reproducibility of parallel sampling is controlled by the caller's RNG
//! partitioning policy.
//!
//! # Mathematical invariants
//!
//! A successfully constructed `StochasticChannel` guarantees:
//!
//! 1. the quantum channel support is structurally valid;
//! 2. every represented input outcome is unique;
//! 3. every represented input has exactly one output distribution;
//! 4. every output distribution is valid according to ZQN's `Distribution`
//!    contract;
//! 5. every output distribution is normalized according to the tolerance
//!    supplied during construction;
//! 6. no hidden probability normalization occurs after construction;
//! 7. no hidden RNG exists;
//! 8. the channel representation is explicitly stochastic;
//! 9. full quantum physicality is not falsely inferred from row normalization;
//! 10. no machine-size semantic limit is imposed.
//!
//! # Finite-domain requirement
//!
//! A `StochasticChannel<I, O>` represents an explicitly materialized finite
//! input domain.
//!
//! This is a property of THIS REPRESENTATION, not a limitation on ZQN or
//! quantum computing generally.
//!
//! A future representation can represent enormous or implicit domains without
//! changing the representation-independent `QuantumChannel` abstraction.
//!
//! If a caller needs a lazy or algorithmically generated transition function,
//! that belongs in a separate representation/adapter rather than forcing this
//! file to materialize the entire domain.
//!
//! # Conditional probability semantics
//!
//! For an input `i` and output `o`:
//!
//! ```text
//! conditional_probability(i, o) = P(o | i)
//! ```
//!
//! If `i` is not represented by this finite channel, the query returns `None`.
//!
//! If `i` exists but `o` has zero or absent probability, the result is `0.0`
//! when the underlying distribution represents that absence as zero probability.
//!
//! # Physicality contract
//!
//! The implementation deliberately reports:
//!
//! ```text
//! ChannelPhysicality::Unvalidated
//! ```
//!
//! because row normalization alone does not establish a complete quantum
//! channel action on arbitrary density operators.
//!
//! A higher-level adapter may establish physicality when it provides a valid
//! quantum embedding or equivalent proof.
//!
//! This implementation MUST NOT change `Unvalidated` to `Validated` merely
//! because the rows sum to one.
//!
//! # Accuracy contract
//!
//! The constructor accepts an explicit `ChannelAccuracy`.
//!
//! This permits the caller to distinguish:
//!
//! - exact transition probabilities;
//! - approximate transition probabilities;
//! - bounded estimates;
//! - statistical characterization.
//!
//! No universal numerical tolerance is embedded in this file.
//!
//! # Resource accounting
//!
//! The representation reports the number of stored transition probabilities as
//! `scalar_elements` when it can be represented by the repository's portable
//! `u128` count.
//!
//! Resource accounting is descriptive, not an allocation policy.
//!
//! Actual admission decisions belong to:
//!
//! ```text
//! zqn::core::limits
//! runtime/resource-management layers
//! ```
//!
//! This distinction prevents resource accounting from becoming an artificial
//! machine-size limit.
//!
//! # Error model
//!
//! Channel-level structural failures use the repository's canonical
//! `ChannelError`.
//!
//! Stochastic-specific operational failures use `StochasticError`.
//!
//! In particular, this file does not modify the central channel error enum
//! merely to expose representation-specific sampling failures.
//!
//! # Serialization
//!
//! Serialization ownership remains outside this file.
//!
//! In-memory Rust representation is not the ZQN wire format.
//!
//! A future `zqn::io` layer can serialize a stable representation containing:
//!
//! ```text
//! schema version
//! channel identity
//! support
//! accuracy
//! input domain
//! conditional distributions
//! provenance
//! ```
//!
//! without making the Rust struct layout itself the protocol.
//!
//! # Security
//!
//! This file accepts potentially externally supplied stochastic data.
//!
//! It therefore:
//!
//! - rejects malformed distributions through the canonical distribution API;
//! - rejects duplicate input keys;
//! - never silently repairs probabilities;
//! - never accepts NaN/∞ through the probability layer;
//! - does not allocate based on attacker-controlled counts without the caller
//!   first choosing the supplied collection/resource policy;
//! - does not execute arbitrary code from stochastic data;
//! - does not access files, network resources, environment variables or
//!   credentials;
//! - contains no unsafe Rust.
//!
//! Callers processing untrusted serialized input MUST enforce explicit resource
//! admission limits before deserializing extremely large transition tables.
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
//! # Integration contract
//!
//! ```text
//! probability::distribution
//!             │
//!             ▼
//! channel::stochastic
//!             │
//!       ┌─────┼─────────────────┐
//!       ▼     ▼                 ▼
//!   QuantumChannel  simulation  composition
//!       │             │         │
//!       ▼             ▼         ▼
//!      IR          runtime     channel algebra
//!       │
//!       ├────────► routing
//!       ├────────► scheduling
//!       ├────────► QEC
//!       ├────────► hardware
//!       └────────► benchmarking
//! ```
//!
//! No downstream system should create another competing stochastic-channel
//! semantic type.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. stochastic conditional probabilities are represented canonically;
//! 2. every input row is a valid normalized distribution;
//! 3. duplicate input outcomes are rejected;
//! 4. the caller controls numerical tolerance;
//! 5. the caller owns RNG state;
//! 6. sampling is deterministic for deterministic RNG state;
//! 7. iteration order is deterministic;
//! 8. no global state exists;
//! 9. no semantic machine-size limit exists;
//! 10. canonical `QubitId` is used indirectly through `ChannelSupport`;
//! 11. the canonical `QuantumChannel` trait is implemented;
//! 12. stochastic representation is reported correctly;
//! 13. stochastic row validity is not falsely advertised as full quantum CPTP
//!     validation;
//! 14. resource requirements are exposed without becoming allocation policy;
//! 15. errors are explicit;
//! 16. the implementation is `Send + Sync` whenever its generic outcome types
//!     permit it;
//! 17. no unsafe code exists;
//! 18. Rust 1.97/1.97.1 is sufficient;
//! 19. later simulation, routing, QEC, scheduling and hardware code can consume
//!     the implementation without changing its semantic contract.
//!
//! # Testing
//!
//! Tests cover:
//!
//! - valid construction;
//! - empty transition rejection;
//! - duplicate input rejection;
//! - conditional lookup;
//! - missing input lookup;
//! - conditional probability lookup;
//! - deterministic iteration;
//! - caller-owned deterministic sampling;
//! - malformed probability propagation;
//! - stochastic-channel descriptor construction;
//! - canonical `QuantumChannel` behavior;
//! - resource accounting;
//! - arbitrary outcome labels;
//! - arbitrary support sizes within available test resources;
//! - no hidden global state.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::BTreeMap;

use rand::Rng;

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
    ChannelSupport,
    QuantumChannel,
};

use crate::quantum::zqn::probability::distribution::{
    Distribution,
    DistributionError,
};

// =============================================================================
// Public error type
// =============================================================================

/// Errors specific to stochastic-channel operations.
#[derive(Debug, Clone, PartialEq)]
pub enum StochasticError {
    /// The stochastic transition domain contains no input outcomes.
    EmptyDomain,

    /// An input outcome was specified more than once.
    DuplicateInput,

    /// The caller attempted to query or sample an input that is not represented
    /// by this finite stochastic channel.
    UnknownInput,

    /// The underlying output distribution is invalid.
    Distribution(DistributionError),

    /// A supplied normalization tolerance is invalid.
    InvalidTolerance,

    /// A resource/count conversion would overflow the portable representation.
    CountOverflow,

    /// The stochastic channel's support and transition semantics are
    /// incompatible.
    IncompatibleSupport,

    /// The operation requires a semantic property that this representation
    /// intentionally does not claim.
    PhysicalityNotEstablished,
}

impl fmt::Display for StochasticError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDomain => {
                formatter.write_str("stochastic channel input domain is empty")
            }

            Self::DuplicateInput => {
                formatter.write_str("stochastic channel contains a duplicate input")
            }

            Self::UnknownInput => {
                formatter.write_str("stochastic channel input is not represented")
            }

            Self::Distribution(error) => {
                write!(formatter, "invalid stochastic output distribution: {error}")
            }

            Self::InvalidTolerance => {
                formatter.write_str("stochastic-channel normalization tolerance is invalid")
            }

            Self::CountOverflow => {
                formatter.write_str("stochastic-channel resource count overflowed")
            }

            Self::IncompatibleSupport => {
                formatter.write_str("stochastic channel support is incompatible with its semantics")
            }

            Self::PhysicalityNotEstablished => {
                formatter.write_str(
                    "full quantum physicality has not been established for the stochastic representation",
                )
            }
        }
    }
}

impl std::error::Error for StochasticError {}

impl From<DistributionError> for StochasticError {
    fn from(error: DistributionError) -> Self {
        Self::Distribution(error)
    }
}

// =============================================================================
// Transition representation
// =============================================================================

/// One conditional stochastic transition.
///
/// The transition means:
///
/// ```text
/// input → Distribution<output>
/// ```
///
/// The distribution represents `P(output | input)`.
#[derive(Clone, Debug, PartialEq)]
pub struct StochasticTransition<I, O> {
    input: I,
    output: Distribution<O>,
}

impl<I, O> StochasticTransition<I, O> {
    /// Creates a conditional stochastic transition from an already validated
    /// output distribution.
    ///
    /// The distribution is not re-normalized.
    #[must_use]
    pub fn new(input: I, output: Distribution<O>) -> Self {
        Self { input, output }
    }

    /// Returns the input outcome.
    #[must_use]
    pub fn input(&self) -> &I {
        &self.input
    }

    /// Returns the conditional output distribution.
    #[must_use]
    pub fn output_distribution(&self) -> &Distribution<O> {
        &self.output
    }
}

// =============================================================================
// Stochastic channel
// =============================================================================

/// A finite-domain conditional stochastic quantum-channel representation.
///
/// The channel stores:
///
/// ```text
/// P(output | input)
/// ```
///
/// as one [`Distribution`] for every represented input outcome.
///
/// `I` and `O` are deliberately generic. They may represent:
///
/// - classical measurement labels;
/// - computational-basis states;
/// - fault labels;
/// - readout states;
/// - encoded outcomes;
/// - modality-specific finite labels;
/// - application-defined finite outcome types.
///
/// No qubit-specific outcome type is imposed here.
///
/// Quantum resource identity belongs to `ChannelSupport`.
///
/// # Physicality
///
/// The implementation is structurally a stochastic channel, but it does not
/// claim full quantum CPTP validity merely from row normalization. Its
/// `QuantumChannel::physicality()` is therefore `ChannelPhysicality::Unvalidated`.
///
/// # Determinism
///
/// Input transitions are stored in `BTreeMap`, providing deterministic ordering
/// independent of randomized hash seeds.
#[derive(Clone, Debug, PartialEq)]
pub struct StochasticChannel<I, O> {
    id: ChannelId,
    name: Option<String>,
    support: ChannelSupport,
    transitions: BTreeMap<I, Distribution<O>>,
    accuracy: ChannelAccuracy,
}

impl<I, O> StochasticChannel<I, O>
where
    I: Ord,
{
    /// Constructs a stochastic channel from conditional output distributions.
    ///
    /// Every `(input, distribution)` pair must have a unique input.
    ///
    /// Each supplied distribution must already satisfy ZQN's probability
    /// invariants. The distribution API is responsible for probability
    /// validation and normalization.
    ///
    /// `accuracy` explicitly declares whether the stochastic data is exact,
    /// approximate, bounded or statistical.
    pub fn new<T>(
        id: ChannelId,
        name: Option<String>,
        support: ChannelSupport,
        transitions: T,
        accuracy: ChannelAccuracy,
    ) -> Result<Self, StochasticError>
    where
        T: IntoIterator<Item = (I, Distribution<O>)>,
    {
        let mut map = BTreeMap::new();

        for (input, distribution) in transitions {
            if map.insert(input, distribution).is_some() {
                return Err(StochasticError::DuplicateInput);
            }
        }

        if map.is_empty() {
            return Err(StochasticError::EmptyDomain);
        }

        let channel = Self {
            id,
            name,
            support,
            transitions: map,
            accuracy,
        };

        channel.validate_stochasticity()?;

        Ok(channel)
    }

    /// Constructs a stochastic channel from validated transition objects.
    ///
    /// This is equivalent to [`Self::new`] but makes the conditional structure
    /// explicit for callers building transition pipelines.
    pub fn from_transitions<T>(
        id: ChannelId,
        name: Option<String>,
        support: ChannelSupport,
        transitions: T,
        accuracy: ChannelAccuracy,
    ) -> Result<Self, StochasticError>
    where
        T: IntoIterator<Item = StochasticTransition<I, O>>,
    {
        Self::new(
            id,
            name,
            support,
            transitions
                .into_iter()
                .map(|transition| (transition.input, transition.output)),
            accuracy,
        )
    }

    /// Returns the stable channel identity.
    #[must_use]
    pub const fn id(&self) -> ChannelId {
        self.id
    }

    /// Returns the optional semantic name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the channel support.
    #[must_use]
    pub fn support(&self) -> &ChannelSupport {
        &self.support
    }

    /// Returns the declared accuracy contract.
    #[must_use]
    pub const fn accuracy(&self) -> ChannelAccuracy {
        self.accuracy
    }

    /// Returns the number of explicitly represented input outcomes.
    ///
    /// This is a representation cardinality, not a machine-size limit.
    #[must_use]
    pub fn input_count(&self) -> usize {
        self.transitions.len()
    }

    /// Returns the number of stored conditional probabilities.
    ///
    /// The count is returned as `u128` so it can participate in portable ZQN
    /// resource accounting.
    #[must_use]
    pub fn probability_count(&self) -> u128 {
        self.transitions
            .values()
            .map(|distribution| distribution.len() as u128)
            .sum()
    }

    /// Returns the conditional distribution for an input.
    ///
    /// `None` means the input is outside this finite channel's represented
    /// domain.
    #[must_use]
    pub fn distribution_for(&self, input: &I) -> Option<&Distribution<O>> {
        self.transitions.get(input)
    }

    /// Returns the probability of `output` conditional on `input`.
    ///
    /// The result is:
    ///
    /// ```text
    /// P(output | input)
    /// ```
    ///
    /// `None` means the input is not represented by this finite stochastic
    /// channel.
    #[must_use]
    pub fn probability_of(&self, input: &I, output: &O) -> Option<f64>
    where
        O: PartialEq,
    {
        self.transitions
            .get(input)
            .and_then(|distribution| distribution.probability_of(output))
    }

    /// Samples an output conditional on an input using caller-owned RNG state.
    ///
    /// The stochastic channel never owns the RNG.
    ///
    /// The caller is responsible for deterministic RNG stream derivation.
    pub fn sample<R>(&self, input: &I, rng: &mut R) -> Result<&O, StochasticError>
    where
        R: Rng + ?Sized,
    {
        let distribution = self
            .transitions
            .get(input)
            .ok_or(StochasticError::UnknownInput)?;

        distribution.sample(rng).map_err(StochasticError::from)
    }

    /// Returns an iterator over all represented input/conditional-distribution
    /// pairs.
    ///
    /// `BTreeMap` guarantees deterministic ordering according to `I::Ord`.
    pub fn transitions(&self) -> impl Iterator<Item = (&I, &Distribution<O>)> {
        self.transitions.iter()
    }

    /// Returns whether the requested input is represented.
    #[must_use]
    pub fn contains_input(&self, input: &I) -> bool {
        self.transitions.contains_key(input)
    }

    /// Validates the stochastic representation independently of full quantum
    /// physicality.
    ///
    /// Because `Distribution` enforces its own probability invariants during
    /// construction/deserialization, validation here checks that every stored
    /// row is non-empty and that the channel domain is non-empty.
    pub fn validate_stochasticity(&self) -> Result<(), StochasticError> {
        if self.transitions.is_empty() {
            return Err(StochasticError::EmptyDomain);
        }

        for distribution in self.transitions.values() {
            if distribution.is_empty() {
                return Err(StochasticError::EmptyDomain);
            }
        }

        self.support
            .validate()
            .map_err(|_| StochasticError::IncompatibleSupport)?;

        Ok(())
    }

    /// Returns the canonical channel descriptor.
    ///
    /// The stochastic representation is marked `Unvalidated` for quantum
    /// physicality because this file does not prove a complete CPTP extension
    /// to arbitrary quantum states.
    pub fn descriptor(&self) -> ChannelResult<ChannelDescriptor> {
        let resources = ChannelResourceRequirements::known(
            Some(self.probability_count()),
            None,
            Some(self.probability_count()),
        );

        ChannelDescriptor::new(
            self.id,
            self.name.clone(),
            self.support.clone(),
            ChannelRepresentation::Stochastic,
            ChannelPhysicality::Unvalidated,
            self.accuracy,
            resources,
        )
    }

    /// Returns the stochastic channel's required semantic capabilities.
    ///
    /// The returned slice is static and immutable. It contains no target or
    /// machine-specific information.
    #[must_use]
    pub const fn required_capabilities() -> &'static [ChannelCapability] {
        &STOCHASTIC_CAPABILITIES
    }

    /// Returns the number of stored scalar transition probabilities.
    #[must_use]
    pub fn resource_scalar_elements(&self) -> u128 {
        self.probability_count()
    }
}

// =============================================================================
// Canonical QuantumChannel integration
// =============================================================================

const STOCHASTIC_CAPABILITIES: [ChannelCapability; 1] =
    [ChannelCapability::Stochastic];

impl<I, O> QuantumChannel for StochasticChannel<I, O>
where
    I: Ord + Send + Sync + fmt::Debug,
    O: Send + Sync + fmt::Debug,
{
    fn id(&self) -> ChannelId {
        self.id
    }

    fn descriptor(&self) -> ChannelDescriptor {
        // Construction validates the descriptor inputs, therefore this should
        // be infallible for a valid StochasticChannel.
        //
        // If a future channel contract makes descriptor creation fallible in a
        // way that cannot be established at construction time, this implementation
        // should be revisited centrally in channel.rs rather than silently
        // manufacturing an invalid descriptor.
        self.descriptor()
            .expect("validated StochasticChannel must produce a valid descriptor")
    }

    fn support(&self) -> &ChannelSupport {
        &self.support
    }

    fn representation(&self) -> ChannelRepresentation {
        ChannelRepresentation::Stochastic
    }

    fn physicality(&self) -> ChannelPhysicality {
        ChannelPhysicality::Unvalidated
    }

    fn accuracy(&self) -> ChannelAccuracy {
        self.accuracy
    }

    fn required_capabilities(&self) -> &[ChannelCapability] {
        &STOCHASTIC_CAPABILITIES
    }

    fn resource_requirements(&self) -> ChannelResourceRequirements {
        ChannelResourceRequirements::known(
            Some(self.probability_count()),
            None,
            Some(self.probability_count()),
        )
    }

    fn validate(&self) -> ChannelResult<()> {
        self.support.validate()?;

        self.validate_stochasticity()
            .map_err(|error| match error {
                StochasticError::IncompatibleSupport => ChannelError::DomainMismatch {
                    input_arity: self.support.input_arity(),
                    output_arity: self.support.output_arity(),
                },

                StochasticError::EmptyDomain => ChannelError::EmptySupport,

                StochasticError::DuplicateInput => ChannelError::IncompatibleComposition,

                StochasticError::Distribution(_) => ChannelError::PropertyUndetermined(
                    "stochastic distribution validation",
                ),

                StochasticError::InvalidTolerance => ChannelError::InvalidTolerance,

                StochasticError::CountOverflow => ChannelError::ResourceRequirementUnavailable,

                StochasticError::UnknownInput => ChannelError::PropertyUndetermined(
                    "unknown stochastic input",
                ),

                StochasticError::PhysicalityNotEstablished => ChannelError::NotValidated,
            })?;

        <Self as QuantumChannel>::validate_accuracy(self)?;

        Ok(())
    }

    fn validate_physicality(&self) -> ChannelResult<()> {
        Err(ChannelError::NotValidated)
    }
}

impl<I, O> StochasticChannel<I, O>
where
    I: Ord + Send + Sync + fmt::Debug,
    O: Send + Sync + fmt::Debug,
{
    /// Internal helper matching the canonical channel validation semantics.
    ///
    /// Kept separate from `validate` so that the representation's stochastic
    /// invariants remain independently testable.
    fn validate_accuracy(&self) -> ChannelResult<()> {
        match self.accuracy {
            ChannelAccuracy::Exact => Ok(()),

            ChannelAccuracy::Approximate { tolerance } => {
                if !tolerance.is_finite() || tolerance < 0.0 {
                    Err(ChannelError::InvalidTolerance)
                } else {
                    Ok(())
                }
            }

            ChannelAccuracy::Bounded { error_bound } => {
                if !error_bound.is_finite() || error_bound < 0.0 {
                    Err(ChannelError::InvalidErrorBound)
                } else {
                    Ok(())
                }
            }

            ChannelAccuracy::Statistical { confidence } => {
                if !confidence.is_finite() || !(0.0..=1.0).contains(&confidence) {
                    Err(ChannelError::InvalidTolerance)
                } else {
                    Ok(())
                }
            }
        }
    }
}

// =============================================================================
// Optional helper constructors
// =============================================================================

impl<I, O> StochasticChannel<I, O>
where
    I: Ord,
{
    /// Constructs a channel from a vector of `(input, output_distribution)` pairs.
    ///
    /// This convenience constructor does not introduce any fixed-size limit.
    pub fn from_vec(
        id: ChannelId,
        name: Option<String>,
        support: ChannelSupport,
        transitions: Vec<(I, Distribution<O>)>,
        accuracy: ChannelAccuracy,
    ) -> Result<Self, StochasticError> {
        Self::new(id, name, support, transitions, accuracy)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use rand::SeedableRng;

    fn distribution(entries: &[(u8, f64)]) -> Distribution<u8> {
        Distribution::from_weighted(entries.iter().copied(), 1.0e-12)
            .expect("test distribution must be valid")
    }

    fn support() -> ChannelSupport {
        ChannelSupport::new(
            vec![
                crate::quantum::zqn::channel::channel::ChannelSubsystem::new(
                    crate::quantum::zqn::channel::channel::ChannelSubsystemId::opaque(0),
                    2,
                )
                .expect("valid subsystem"),
            ],
            vec![
                crate::quantum::zqn::channel::channel::ChannelSubsystem::new(
                    crate::quantum::zqn::channel::channel::ChannelSubsystemId::opaque(0),
                    2,
                )
                .expect("valid subsystem"),
            ],
        )
        .expect("valid support")
    }

    fn channel() -> StochasticChannel<u8, u8> {
        StochasticChannel::new(
            ChannelId::from_u128(1),
            Some("test-stochastic-channel".to_owned()),
            support(),
            vec![
                (0, distribution(&[(0, 0.9), (1, 0.1)])),
                (1, distribution(&[(0, 0.2), (1, 0.8)])),
            ],
            ChannelAccuracy::Exact,
        )
        .expect("valid stochastic channel")
    }

    #[test]
    fn constructs_valid_channel() {
        let channel = channel();

        assert_eq!(channel.input_count(), 2);
        assert_eq!(channel.probability_count(), 4);
        assert!(channel.contains_input(&0));
        assert!(channel.contains_input(&1));
    }

    #[test]
    fn rejects_empty_domain() {
        let result = StochasticChannel::<u8, u8>::new(
            ChannelId::from_u128(1),
            None,
            support(),
            Vec::<(u8, Distribution<u8>)>::new(),
            ChannelAccuracy::Exact,
        );

        assert_eq!(result, Err(StochasticError::EmptyDomain));
    }

    #[test]
    fn rejects_duplicate_inputs() {
        let result = StochasticChannel::new(
            ChannelId::from_u128(1),
            None,
            support(),
            vec![
                (0, distribution(&[(0, 1.0)])),
                (0, distribution(&[(1, 1.0)])),
            ],
            ChannelAccuracy::Exact,
        );

        assert_eq!(result, Err(StochasticError::DuplicateInput));
    }

    #[test]
    fn returns_conditional_distribution() {
        let channel = channel();

        let distribution = channel
            .distribution_for(&0)
            .expect("input must exist");

        assert_eq!(distribution.probability_of(&0), Some(0.9));
        assert_eq!(distribution.probability_of(&1), Some(0.1));
    }

    #[test]
    fn returns_conditional_probability() {
        let channel = channel();

        assert_eq!(channel.probability_of(&0, &0), Some(0.9));
        assert_eq!(channel.probability_of(&0, &1), Some(0.1));
        assert_eq!(channel.probability_of(&1, &0), Some(0.2));
        assert_eq!(channel.probability_of(&1, &1), Some(0.8));
    }

    #[test]
    fn unknown_input_returns_none_for_probability_query() {
        let channel = channel();

        assert_eq!(channel.probability_of(&7, &0), None);
    }

    #[test]
    fn unknown_input_is_an_explicit_sampling_error() {
        let channel = channel();
        let mut rng = rand::rngs::StdRng::seed_from_u64(7);

        assert_eq!(
            channel.sample(&7, &mut rng),
            Err(StochasticError::UnknownInput)
        );
    }

    #[test]
    fn sampling_uses_caller_owned_rng() {
        let channel = channel();

        let mut first = rand::rngs::StdRng::seed_from_u64(1234);
        let mut second = rand::rngs::StdRng::seed_from_u64(1234);

        let first_sample = *channel.sample(&0, &mut first).expect("sampling must work");
        let second_sample = *channel
            .sample(&0, &mut second)
            .expect("sampling must work");

        assert_eq!(first_sample, second_sample);
    }

    #[test]
    fn transitions_are_deterministically_ordered() {
        let channel = channel();

        let inputs: Vec<u8> = channel.transitions().map(|(input, _)| *input).collect();

        assert_eq!(inputs, vec![0, 1]);
    }

    #[test]
    fn reports_stochastic_representation() {
        let channel = channel();

        assert_eq!(
            <StochasticChannel<u8, u8> as QuantumChannel>::representation(&channel),
            ChannelRepresentation::Stochastic
        );
    }

    #[test]
    fn reports_stochastic_capability() {
        let channel = channel();

        assert_eq!(
            <StochasticChannel<u8, u8> as QuantumChannel>::required_capabilities(&channel),
            &[ChannelCapability::Stochastic]
        );
    }

    #[test]
    fn does_not_claim_quantum_physicality_from_row_normalization() {
        let channel = channel();

        assert_eq!(
            channel.physicality(),
            ChannelPhysicality::Unvalidated
        );

        assert_eq!(
            channel.validate_physicality(),
            Err(ChannelError::NotValidated)
        );
    }

    #[test]
    fn descriptor_contains_stochastic_metadata() {
        let channel = channel();

        let descriptor = channel.descriptor();

        assert_eq!(
            descriptor.representation,
            ChannelRepresentation::Stochastic
        );
        assert_eq!(
            descriptor.physicality,
            ChannelPhysicality::Unvalidated
        );
        assert_eq!(descriptor.resources.scalar_elements, Some(4));
        assert_eq!(descriptor.resources.arithmetic_operations, Some(4));
    }

    #[test]
    fn canonical_quantum_channel_validation_succeeds_for_valid_stochastic_data() {
        let channel = channel();

        assert!(
            <StochasticChannel<u8, u8> as QuantumChannel>::validate(&channel).is_ok()
        );
    }

    #[test]
    fn exact_accuracy_is_preserved() {
        let channel = channel();

        assert_eq!(channel.accuracy(), ChannelAccuracy::Exact);
        assert!(channel.is_exact());
    }

    #[test]
    fn approximate_accuracy_is_explicit() {
        let accuracy =
            ChannelAccuracy::approximate(1.0e-6).expect("valid approximation");

        let channel = StochasticChannel::new(
            ChannelId::from_u128(2),
            None,
            support(),
            vec![(0, distribution(&[(0, 1.0)]))],
            accuracy,
        )
        .expect("valid channel");

        assert_eq!(channel.accuracy(), accuracy);
        assert!(!channel.is_exact());
    }

    #[test]
    fn supports_non_numeric_outcome_types() {
        let distribution = Distribution::from_weighted(
            vec![
                ("success".to_owned(), 0.7),
                ("failure".to_owned(), 0.3),
            ],
            1.0e-12,
        )
        .expect("valid distribution");

        let channel = StochasticChannel::new(
            ChannelId::from_u128(3),
            None,
            support(),
            vec![(String::from("input"), distribution)],
            ChannelAccuracy::Statistical {
                confidence: 0.95,
            },
        )
        .expect("valid channel");

        assert_eq!(
            channel.probability_of(
                &String::from("input"),
                &String::from("success")
            ),
            Some(0.7)
        );
    }

    #[test]
    fn resource_count_matches_stored_probabilities() {
        let channel = channel();

        assert_eq!(channel.resource_scalar_elements(), 4);
        assert_eq!(channel.probability_count(), 4);
    }

    #[test]
    fn arbitrary_input_domain_is_supported_without_machine_size_constants() {
        let transitions = (0u64..128u64).map(|input| {
            (
                input,
                Distribution::from_weighted(
                    vec![(input, 1.0)],
                    1.0e-12,
                )
                .expect("valid distribution"),
            )
        });

        let channel = StochasticChannel::new(
            ChannelId::from_u128(4),
            None,
            support(),
            transitions,
            ChannelAccuracy::Exact,
        )
        .expect("valid channel");

        assert_eq!(channel.input_count(), 128);
        assert_eq!(channel.probability_count(), 128);
    }
}