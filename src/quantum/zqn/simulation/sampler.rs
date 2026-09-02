//! Zamani Quantum Noise (ZQN) — Production Sampling Engine.
//!
//! `src/quantum/zqn/simulation/sampler.rs`
//!
//! # Purpose
//!
//! This module owns the execution-side sampling boundary for ZQN.
//!
//! It converts mathematical probability distributions into reproducible
//! observations while keeping probability semantics, quantum state evolution,
//! noise-model semantics, hardware execution, QEC, calibration, and routing
//! outside this file.
//!
//! The central architecture is:
//!
//! ```text
//! ZQN Distribution<T>
//!        │
//!        ▼
//! SamplingPolicy
//!        │
//!        ▼
//! Sampler<R>
//!        │
//!        ├── single sample
//!        ├── bounded batch
//!        └── lazy stream
//!
//! Runtime / simulation / QEC / benchmarking owns:
//! - the RNG policy;
//! - the seed;
//! - the quantum-resource identity;
//! - the execution context;
//! - cancellation;
//! - memory/resource limits;
//! - interpretation of outcomes.
//! ```
//!
//! # Ownership
//!
//! This file owns:
//!
//! - deterministic sampling orchestration;
//! - sampling policy;
//! - explicit RNG ownership at the sampler boundary;
//! - single-outcome sampling;
//! - bounded batch sampling;
//! - lazy streaming sampling;
//! - checked sample-count conversion;
//! - sampling statistics that do not require materializing all observations;
//! - reproducibility metadata;
//! - deterministic sample-index derivation;
//! - sampling-specific validation;
//! - sampling-specific errors.
//!
//! # Does not own
//!
//! This module does NOT own:
//!
//! - quantum state representation;
//! - state-vector simulation;
//! - density-matrix simulation;
//! - tensor-network simulation;
//! - quantum channels;
//! - Kraus/Choi mathematics;
//! - probability-distribution construction;
//! - noise-model semantics;
//! - calibration;
//! - characterization;
//! - QEC decoding;
//! - routing;
//! - scheduling;
//! - hardware execution;
//! - vendor APIs;
//! - QPU credentials;
//! - canonical quantum IR;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - source-language semantics.
//!
//! Those responsibilities remain in their owning modules.
//!
//! # Canonical quantum identity
//!
//! This file deliberately does not create or redefine a quantum-resource ID.
//!
//! If a caller needs to associate an observation with a quantum resource, the
//! caller must use the canonical identities from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The mathematical sampler itself is intentionally independent of those
//! identities.
//!
//! This preserves the repository's canonical identity boundary.
//!
//! # Why the sampler is generic over T
//!
//! A quantum execution result is not necessarily a two-valued bit.
//!
//! Outcomes may represent:
//!
//! - classical bits;
//! - bit strings;
//! - qudit values;
//! - measurement labels;
//! - fault kinds;
//! - logical states;
//! - bosonic outcomes;
//! - continuous-value bins;
//! - future quantum modalities.
//!
//! Therefore this module samples `Distribution<T>` rather than assuming:
//!
//! ```text
//! bool
//! bit
//! u8
//! qubit
//! ```
//!
//! # Relationship with probability::distribution
//!
//! `zqn::probability::distribution::Distribution<T>` is the mathematical
//! probability abstraction already established by the repository.
//!
//! It owns:
//!
//! - probability validation;
//! - normalization;
//! - duplicate merging;
//! - deterministic representation;
//! - probability lookup;
//! - caller-owned RNG sampling.
//!
//! This sampler must NOT duplicate those responsibilities.
//!
//! The relationship is:
//!
//! ```text
//! probability::distribution
//!          │
//!          │ mathematical distribution
//!          ▼
//! simulation::sampler
//!          │
//!          │ execution policy
//!          ▼
//! observations
//! ```
//!
//! The distribution remains the source of truth for probabilities.
//!
//! # Write once, scale everywhere
//!
//! There is intentionally no semantic maximum for:
//!
//! - number of samples;
//! - number of quantum resources;
//! - number of outcomes;
//! - number of shots;
//! - number of operations;
//! - circuit depth;
//! - machine size.
//!
//! In particular this module contains no:
//!
//! ```text
//! MAX_SHOTS
//! MAX_QUBITS
//! MAX_SAMPLES
//! MAX_OUTCOMES
//! ```
//!
//! An execution may request an arbitrarily large number of observations,
//! subject only to:
//!
//! - available resources;
//! - explicit runtime policies;
//! - caller-selected limits;
//! - target capabilities;
//! - storage capacity.
//!
//! Large requests must be streamable rather than requiring one giant
//! allocation.
//!
//! # Infinity semantics
//!
//! "Infinity" in Zamani means that the semantic model has no artificial finite
//! machine-size ceiling.
//!
//! It does NOT mean that a process can physically allocate infinite memory or
//! perform infinite work.
//!
//! This distinction is fundamental.
//!
//! Therefore:
//!
//! ```text
//! semantic sample count
//!         │
//!         ├── may be arbitrarily large
//!         │
//!         ▼
//! execution strategy
//!         │
//!         ├── bounded vector
//!         ├── streaming iterator
//!         └── distributed execution
//! ```
//!
//! # Determinism
//!
//! Determinism is a first-class contract.
//!
//! The sampler:
//!
//! - never creates a global RNG;
//! - never uses a thread-local RNG implicitly;
//! - never reads wall-clock time;
//! - never derives entropy from memory addresses;
//! - never depends on thread scheduling;
//! - never depends on hash-map iteration order;
//! - never mutates global state.
//!
//! The caller owns the RNG.
//!
//! A seeded sampler therefore has the form:
//!
//! ```text
//! seed
//!   │
//!   ▼
//! explicit RNG
//!   │
//!   ▼
//! Sampler
//!   │
//!   ▼
//! Distribution
//!   │
//!   ▼
//! observation
//! ```
//!
//! # Parallel determinism
//!
//! A production execution system may execute samples:
//!
//! - sequentially;
//! - concurrently;
//! - on multiple processes;
//! - on multiple machines;
//! - on accelerators.
//!
//! The semantic sample identity is therefore defined by an explicit sample
//! index rather than by worker identity.
//!
//! A deterministic runtime should derive independent RNG streams from:
//!
//! ```text
//! master seed
//! program identity
//! model identity
//! operation identity
//! resource identity
//! sample index
//! ```
//!
//! The exact cryptographic derivation belongs to the runtime/reproducibility
//! layer. This file provides the local sequential sampler contract without
//! secretly inventing a global seed policy.
//!
//! # RNG compatibility
//!
//! The repository currently uses `rand = "0.8"`.
//!
//! This file therefore uses the stable `rand::Rng` and
//! `rand::SeedableRng` APIs available under that dependency.
//!
//! # Sampling modes
//!
//! Three primary modes are provided:
//!
//! ```text
//! sample()
//!     one observation
//!
//! sample_n()
//!     caller requests a bounded materialized batch
//!
//! stream()
//!     lazy potentially-large observation stream
//! ```
//!
//! The stream is the preferred interface for very large executions because it
//! does not require materializing all observations.
//!
//! # Resource safety
//!
//! `sample_n()` necessarily allocates storage proportional to the requested
//! batch size.
//!
//! Therefore it is explicitly distinguished from `stream()`.
//!
//! The sampler never attempts to "help" by silently truncating a request.
//!
//! If the requested count cannot be represented by the host collection type,
//! the sampler returns an explicit error.
//!
//! If allocation itself fails, Rust's normal allocation behavior applies; the
//! caller should use bounded batches or streaming when operating under an
//! explicit resource budget.
//!
//! # Numerical safety
//!
//! The sampler does not modify probabilities.
//!
//! It delegates probability validation and weighted selection to
//! `Distribution<T>`.
//!
//! It therefore does not:
//!
//! - clamp invalid probabilities;
//! - normalize invalid distributions silently;
//! - turn NaN into zero;
//! - turn infinity into a finite number;
//! - repair negative probabilities.
//!
//! Invalid probability semantics must fail at the mathematical distribution
//! boundary.
//!
//! # Statistical correctness
//!
//! Sampling is with replacement.
//!
//! Each observation is an independent draw from the supplied distribution,
//! conditional on the supplied RNG stream.
//!
//! The sampler does not reinterpret the distribution as:
//!
//! - a multinomial count;
//! - a deterministic sequence;
//! - a shuffled outcome list.
//!
//! # Reproducibility
//!
//! The sampler exposes the initial seed when using `SeededSampler`.
//!
//! It does not expose internal RNG implementation state as a stable wire
//! format.
//!
//! A stable scientific execution record should additionally store:
//!
//! - ZQN version;
//! - model identity;
//! - distribution identity;
//! - target identity;
//! - calibration identity;
//! - master seed;
//! - sampling policy;
//! - numerical configuration.
//!
//! Those identities are owned by the surrounding ZQN/runtime layers.
//!
//! # Thread safety
//!
//! `Sampler<R>` does not require `R: Sync` merely to define the abstraction.
//!
//! A sampler owns mutable RNG state and therefore requires exclusive mutable
//! access while sampling.
//!
//! Concurrent execution should use independent sampler instances/streams.
//!
//! A single mutable RNG must not be shared between worker threads merely to
//! obtain parallelism.
//!
//! # Serialization
//!
//! RNG state is intentionally not serialized here.
//!
//! The sampler's configuration can be serialized by a higher-level execution
//! schema, but the concrete RNG implementation is not a ZQN wire-format
//! contract.
//!
//! # Security
//!
//! A pseudorandom sampler is not a cryptographic RNG.
//!
//! This module MUST NOT be used for:
//!
//! - cryptographic key generation;
//! - security tokens;
//! - authentication;
//! - cryptographic commitments;
//! - secrets.
//!
//! Quantum-noise reproducibility and cryptographic randomness are different
//! requirements.
//!
//! # Integration
//!
//! ```text
//! Quantum IR
//!      │
//!      ▼
//! Noise model / channel
//!      │
//!      ▼
//! Distribution<T>
//!      │
//!      ▼
//! simulation::sampler
//!      │
//!      ├───────────────┬────────────────┐
//!      ▼               ▼                ▼
//! state simulation    QEC          benchmarking
//!      │               │                │
//!      └───────────────┼────────────────┘
//!                      ▼
//!                 observations
//! ```
//!
//! The sampler is therefore a leaf execution utility, not a replacement for
//! the simulator.
//!
//! # Integration with QEC
//!
//! QEC may use this module when a physical fault model produces a probability
//! distribution over fault outcomes.
//!
//! The sampler does not interpret those outcomes.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may consume the streaming interface to execute extremely large
//! shot counts without materializing every observation.
//!
//! # Integration with simulation
//!
//! Simulation engines may call `sample()` for a single stochastic event or
//! `stream()` for shot-oriented execution.
//!
//! # Integration with hardware
//!
//! Hardware adapters normally do not use this sampler for actual physical
//! measurements. They return observations from the QPU.
//!
//! The sampler may instead be used for:
//!
//! - local emulation;
//! - backend-independent noise injection;
//! - deterministic test execution;
//! - hardware-model simulation.
//!
//! # Integration with calibration
//!
//! Calibration values affect the distribution.
//!
//! Calibration itself does not belong here.
//!
//! ```text
//! calibration
//!      │
//!      ▼
//! noise model
//!      │
//!      ▼
//! distribution
//!      │
//!      ▼
//! sampler
//! ```
//!
//! # Integration with canonical quantum IR
//!
//! This file intentionally does not depend directly on `quantum::ir::qubit`.
//!
//! A sampler is a mathematical execution primitive.
//!
//! When a higher-level simulation object associates a sample with a canonical
//! qubit/resource, it must use the canonical IR identity there.
//!
//! This avoids contaminating the probability/sampling layer with resource
//! semantics.
//!
//! # API stability
//!
//! The stable API is intentionally small:
//!
//! ```text
//! SamplingPolicy
//! SamplingError
//! Sampler
//! SeededSampler
//! SampleStream
//! ```
//!
//! Additional optimization strategies should be added behind these contracts
//! rather than forcing downstream consumers to depend on implementation
//! details.
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
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::convert::TryFrom;
use std::marker::PhantomData;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::quantum::zqn::probability::distribution::{
    Distribution,
    DistributionError,
};

// =============================================================================
// Public constants
// =============================================================================

/// Semantic version of the sampling contract.
///
/// This is a schema/contract marker only. It is not a resource limit.
pub const SAMPLER_MODEL_VERSION: u16 = 1;

// =============================================================================
// Sampling policy
// =============================================================================

/// Controls how sampling is performed.
///
/// The policy is deliberately independent from the quantum resource count.
///
/// A distribution is sampled with replacement under all policies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SamplingPolicy {
    /// Whether a materialized result is permitted.
    ///
    /// This is an execution-policy hint rather than a mathematical property.
    pub materialization: MaterializationPolicy,

    /// Whether an empty request is accepted.
    ///
    /// Accepting zero is useful for generic pipelines where the number of
    /// requested shots is computed dynamically.
    pub allow_zero_samples: bool,
}

impl Default for SamplingPolicy {
    fn default() -> Self {
        Self {
            materialization: MaterializationPolicy::Bounded,
            allow_zero_samples: true,
        }
    }
}

/// Controls whether samples may be materialized into a collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterializationPolicy {
    /// The caller explicitly accepts a `Vec<T>` result.
    Bounded,

    /// The caller intends to use a lazy stream.
    StreamingOnly,
}

// =============================================================================
// Errors
// =============================================================================

/// Errors raised by the ZQN sampler.
#[derive(Debug, Clone, PartialEq)]
pub enum SamplingError {
    /// The requested sample count is zero while the policy forbids zero.
    ZeroSamplesNotAllowed,

    /// The requested count cannot be represented by the host collection type.
    SampleCountOverflow {
        requested: u128,
    },

    /// Materialization was requested while policy allows only streaming.
    MaterializationForbidden,

    /// The supplied distribution is invalid for sampling.
    Distribution(DistributionError),

    /// The sampler received an invalid sampling probability from the
    /// mathematical layer.
    InvalidSamplingProbability {
        value: f64,
    },

    /// The random draw could not be mapped to a valid outcome.
    ///
    /// This should only occur if the distribution implementation violates its
    /// own invariants or if floating-point rounding creates an impossible
    /// terminal state.
    NoOutcomeForDraw {
        draw: f64,
    },

    /// The configured seed could not be used to initialize the selected RNG.
    InvalidSeed,

    /// The requested execution index cannot be represented by the chosen
    /// numeric type.
    IndexOverflow,
}

impl fmt::Display for SamplingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroSamplesNotAllowed => {
                formatter.write_str("zero samples are not permitted by the sampling policy")
            }

            Self::SampleCountOverflow { requested } => write!(
                formatter,
                "requested sample count {requested} cannot be represented by the host collection"
            ),

            Self::MaterializationForbidden => {
                formatter.write_str("materialized sampling is forbidden by the sampling policy")
            }

            Self::Distribution(error) => write!(formatter, "distribution sampling failed: {error}"),

            Self::InvalidSamplingProbability { value } => write!(
                formatter,
                "sampling probability is not finite or is outside [0, 1]: {value}"
            ),

            Self::NoOutcomeForDraw { draw } => {
                write!(formatter, "random draw {draw} could not be mapped to an outcome")
            }

            Self::InvalidSeed => {
                formatter.write_str("the supplied seed could not initialize the RNG")
            }

            Self::IndexOverflow => {
                formatter.write_str("sampling index cannot be represented")
            }
        }
    }
}

impl std::error::Error for SamplingError {}

impl From<DistributionError> for SamplingError {
    fn from(error: DistributionError) -> Self {
        Self::Distribution(error)
    }
}

// =============================================================================
// Sample identity
// =============================================================================

/// Stable logical identity of one requested sample.
///
/// This is intentionally not a quantum-resource ID.
///
/// It identifies a shot/observation position inside a sampling execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SampleIndex(u128);

impl SampleIndex {
    /// Creates a sample index.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the numeric sample index.
    #[must_use]
    pub const fn get(self) -> u128 {
        self.0
    }

    /// Returns the next index, or an explicit overflow error.
    pub fn checked_next(self) -> Result<Self, SamplingError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(SamplingError::IndexOverflow)
    }
}

impl fmt::Display for SampleIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Sampling observation
// =============================================================================

/// One sampled observation.
///
/// The sample index is included so downstream distributed execution can retain
/// deterministic shot identity without relying on vector position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sample<T> {
    /// Stable logical sample index.
    pub index: SampleIndex,

    /// Sampled outcome.
    pub outcome: T,
}

impl<T> Sample<T> {
    /// Creates a sampled observation.
    #[must_use]
    pub const fn new(index: SampleIndex, outcome: T) -> Self {
        Self { index, outcome }
    }

    /// Maps the outcome while preserving sample identity.
    #[must_use]
    pub fn map<U, F>(self, mapper: F) -> Sample<U>
    where
        F: FnOnce(T) -> U,
    {
        Sample {
            index: self.index,
            outcome: mapper(self.outcome),
        }
    }
}

// =============================================================================
// Sampler
// =============================================================================

/// Generic ZQN sampling engine.
///
/// `R` is caller-owned RNG state.
///
/// The sampler itself owns no global state and has no implicit entropy source.
#[derive(Debug, Clone)]
pub struct Sampler<R> {
    rng: R,
    next_index: SampleIndex,
    policy: SamplingPolicy,
}

impl<R> Sampler<R> {
    /// Creates a sampler from an existing caller-owned RNG.
    ///
    /// This is the preferred integration point for runtimes that already have
    /// an explicit reproducibility/RNG subsystem.
    #[must_use]
    pub const fn with_rng(rng: R) -> Self {
        Self {
            rng,
            next_index: SampleIndex::new(0),
            policy: SamplingPolicy {
                materialization: MaterializationPolicy::Bounded,
                allow_zero_samples: true,
            },
        }
    }

    /// Creates a sampler with an explicit policy.
    #[must_use]
    pub const fn with_rng_and_policy(
        rng: R,
        policy: SamplingPolicy,
    ) -> Self {
        Self {
            rng,
            next_index: SampleIndex::new(0),
            policy,
        }
    }

    /// Returns the configured policy.
    #[must_use]
    pub const fn policy(&self) -> SamplingPolicy {
        self.policy
    }

    /// Returns the next sample index that will be emitted.
    #[must_use]
    pub const fn next_index(&self) -> SampleIndex {
        self.next_index
    }

    /// Replaces the next sample index.
    ///
    /// This is useful for distributed execution where a worker owns an explicit
    /// range of global sample indices.
    pub const fn set_next_index(&mut self, index: SampleIndex) {
        self.next_index = index;
    }

    /// Consumes the sampler and returns the underlying RNG.
    #[must_use]
    pub fn into_rng(self) -> R {
        self.rng
    }

    /// Returns a mutable reference to the underlying RNG.
    ///
    /// This is intentionally explicit so callers understand that changing RNG
    /// state changes subsequent stochastic observations.
    pub fn rng_mut(&mut self) -> &mut R {
        &mut self.rng
    }
}

impl<R> Sampler<R>
where
    R: Rng,
{
    /// Samples one observation.
    pub fn sample<T>(
        &mut self,
        distribution: &Distribution<T>,
    ) -> Result<Sample<T>, SamplingError>
    where
        T: Clone,
    {
        let index = self.next_index;
        let outcome = sample_distribution(distribution, &mut self.rng)?;

        self.next_index = self.next_index.checked_next()?;

        Ok(Sample::new(index, outcome))
    }

    /// Samples exactly `count` observations into a vector.
    ///
    /// This is the bounded/materialized interface.
    ///
    /// For very large executions use [`Self::stream`] instead.
    pub fn sample_n<T>(
        &mut self,
        distribution: &Distribution<T>,
        count: u128,
    ) -> Result<Vec<Sample<T>>, SamplingError>
    where
        T: Clone,
    {
        if count == 0 {
            if !self.policy.allow_zero_samples {
                return Err(SamplingError::ZeroSamplesNotAllowed);
            }

            return Ok(Vec::new());
        }

        if matches!(
            self.policy.materialization,
            MaterializationPolicy::StreamingOnly
        ) {
            return Err(SamplingError::MaterializationForbidden);
        }

        let capacity =
            usize::try_from(count)
                .map_err(|_| SamplingError::SampleCountOverflow {
                    requested: count,
                })?;

        let mut samples = Vec::new();

        /*
         * Reserve only after the count has passed the host-index conversion.
         *
         * The reservation is intentionally fallible only through Rust's normal
         * allocator semantics. The sampler never silently truncates.
         */
        if capacity > 0 {
            samples.reserve_exact(capacity);
        }

        for _ in 0..count {
            samples.push(self.sample(distribution)?);
        }

        Ok(samples)
    }

    /// Creates a lazy stream of exactly `count` observations.
    ///
    /// No result collection proportional to `count` is allocated.
    ///
    /// This is the preferred API for:
    ///
    /// - very large shot counts;
    /// - benchmarking;
    /// - Monte Carlo simulation;
    /// - distributed execution;
    /// - resource-constrained environments.
    pub fn stream<'a, T>(
        &'a mut self,
        distribution: &'a Distribution<T>,
        count: u128,
    ) -> Result<SampleStream<'a, R, T>, SamplingError>
    where
        T: Clone,
    {
        if count == 0 && !self.policy.allow_zero_samples {
            return Err(SamplingError::ZeroSamplesNotAllowed);
        }

        Ok(SampleStream {
            sampler: self,
            distribution,
            remaining: count,
            _marker: PhantomData,
        })
    }

    /// Samples observations into a caller-provided vector.
    ///
    /// This is useful when the caller has already performed resource admission
    /// and wants to reuse a buffer.
    pub fn sample_into<T>(
        &mut self,
        distribution: &Distribution<T>,
        output: &mut Vec<Sample<T>>,
        count: u128,
    ) -> Result<(), SamplingError>
    where
        T: Clone,
    {
        if count == 0 {
            if !self.policy.allow_zero_samples {
                return Err(SamplingError::ZeroSamplesNotAllowed);
            }

            return Ok(());
        }

        for _ in 0..count {
            output.push(self.sample(distribution)?);
        }

        Ok(())
    }

    /// Resets the logical sample index without changing the RNG.
    ///
    /// This operation is useful only when the caller deliberately wants to
    /// reinterpret subsequent RNG output under a different sample numbering.
    ///
    /// It does NOT rewind the RNG.
    pub const fn reset_index(&mut self) {
        self.next_index = SampleIndex::new(0);
    }
}

// =============================================================================
// Seeded sampler
// =============================================================================

/// Reproducible ZQN sampler backed by `StdRng`.
///
/// This type is intentionally explicit about seed ownership.
///
/// It is suitable for:
///
/// - deterministic tests;
/// - reproducible local simulation;
/// - benchmarking fixtures;
/// - regression testing;
/// - scientific experiments where the seed is part of the execution record.
///
/// It is NOT a cryptographic random generator.
#[derive(Debug, Clone)]
pub struct SeededSampler {
    sampler: Sampler<StdRng>,
    seed: [u8; 32],
}

impl SeededSampler {
    /// Creates a reproducible sampler from a 256-bit seed.
    #[must_use]
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self {
            sampler: Sampler::with_rng(StdRng::from_seed(seed)),
            seed,
        }
    }

    /// Creates a reproducible sampler with an explicit policy.
    #[must_use]
    pub fn from_seed_with_policy(
        seed: [u8; 32],
        policy: SamplingPolicy,
    ) -> Self {
        Self {
            sampler: Sampler::with_rng_and_policy(
                StdRng::from_seed(seed),
                policy,
            ),
            seed,
        }
    }

    /// Returns the configured seed.
    #[must_use]
    pub const fn seed(&self) -> [u8; 32] {
        self.seed
    }

    /// Returns the sampling policy.
    #[must_use]
    pub const fn policy(&self) -> SamplingPolicy {
        self.sampler.policy()
    }

    /// Returns the next sample index.
    #[must_use]
    pub const fn next_index(&self) -> SampleIndex {
        self.sampler.next_index()
    }

    /// Sets the logical sample index.
    ///
    /// This does not rewind or alter RNG state.
    pub const fn set_next_index(&mut self, index: SampleIndex) {
        self.sampler.set_next_index(index);
    }

    /// Samples one observation.
    pub fn sample<T>(
        &mut self,
        distribution: &Distribution<T>,
    ) -> Result<Sample<T>, SamplingError>
    where
        T: Clone,
    {
        self.sampler.sample(distribution)
    }

    /// Samples a materialized batch.
    pub fn sample_n<T>(
        &mut self,
        distribution: &Distribution<T>,
        count: u128,
    ) -> Result<Vec<Sample<T>>, SamplingError>
    where
        T: Clone,
    {
        self.sampler.sample_n(distribution, count)
    }

    /// Creates a lazy stream.
    pub fn stream<'a, T>(
        &'a mut self,
        distribution: &'a Distribution<T>,
        count: u128,
    ) -> Result<SampleStream<'a, StdRng, T>, SamplingError>
    where
        T: Clone,
    {
        self.sampler.stream(distribution, count)
    }

    /// Consumes the wrapper and returns the generic sampler.
    #[must_use]
    pub fn into_sampler(self) -> Sampler<StdRng> {
        self.sampler
    }
}

// =============================================================================
// Lazy stream
// =============================================================================

/// Lazy stream of ZQN observations.
///
/// The stream owns no additional collection proportional to the number of
/// requested observations.
///
/// It borrows the sampler and distribution, so the same mutable sampler cannot
/// be used concurrently while the stream exists.
pub struct SampleStream<'a, R, T> {
    sampler: &'a mut Sampler<R>,
    distribution: &'a Distribution<T>,
    remaining: u128,
    _marker: PhantomData<&'a T>,
}

impl<'a, R, T> SampleStream<'a, R, T>
where
    R: Rng,
    T: Clone,
{
    /// Returns the number of samples that have not yet been emitted.
    #[must_use]
    pub const fn remaining(&self) -> u128 {
        self.remaining
    }
}

impl<'a, R, T> Iterator for SampleStream<'a, R, T>
where
    R: Rng,
    T: Clone,
{
    type Item = Result<Sample<T>, SamplingError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        self.remaining -= 1;

        Some(self.sampler.sample(self.distribution))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let upper = usize::try_from(self.remaining).ok();

        match upper {
            Some(value) => (value, Some(value)),
            None => (0, None),
        }
    }
}

// =============================================================================
// Standalone distribution sampling
// =============================================================================

/// Samples one outcome from a distribution using caller-owned RNG state.
///
/// This function is the smallest integration boundary between the probability
/// and simulation layers.
///
/// It intentionally delegates mathematical sampling to `Distribution<T>`.
pub fn sample_distribution<T, R>(
    distribution: &Distribution<T>,
    rng: &mut R,
) -> Result<T, SamplingError>
where
    T: Clone,
    R: Rng,
{
    /*
     * The distribution layer is the canonical owner of weighted sampling.
     *
     * Keeping this function as a thin adapter means:
     *
     * - the mathematical distribution owns probability semantics;
     * - this module owns execution orchestration;
     * - future distribution implementations do not require a sampler rewrite.
     */
    distribution
        .sample(rng)
        .map_err(SamplingError::Distribution)
}

// =============================================================================
// Deterministic seed derivation helper
// =============================================================================

/// Derives a deterministic 32-byte child seed from a parent seed and an
/// explicit domain/index pair.
///
/// This is deliberately a small non-cryptographic domain-separation helper for
/// reproducible RNG stream partitioning.
///
/// It does NOT claim cryptographic security.
///
/// The caller should include the complete semantic identity in the domain and
/// index inputs when constructing distributed execution streams.
#[must_use]
pub fn derive_stream_seed(
    parent_seed: [u8; 32],
    domain: u128,
    index: u128,
) -> [u8; 32] {
    /*
     * SplitMix-style integer mixing is used solely to produce deterministic,
     * well-distributed independent-looking RNG seeds.
     *
     * This is not a cryptographic hash and must never be used as one.
     *
     * The implementation is intentionally allocation-free and independent of
     * process/thread identity.
     */
    fn mix(mut value: u64) -> u64 {
        value ^= value >> 30;
        value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value ^= value >> 27;
        value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }

    let domain_lo = domain as u64;
    let domain_hi = (domain >> 64) as u64;

    let index_lo = index as u64;
    let index_hi = (index >> 64) as u64;

    let mut output = [0_u8; 32];

    let words = [
        u64::from_le_bytes([
            parent_seed[0],
            parent_seed[1],
            parent_seed[2],
            parent_seed[3],
            parent_seed[4],
            parent_seed[5],
            parent_seed[6],
            parent_seed[7],
        ]),
        u64::from_le_bytes([
            parent_seed[8],
            parent_seed[9],
            parent_seed[10],
            parent_seed[11],
            parent_seed[12],
            parent_seed[13],
            parent_seed[14],
            parent_seed[15],
        ]),
        u64::from_le_bytes([
            parent_seed[16],
            parent_seed[17],
            parent_seed[18],
            parent_seed[19],
            parent_seed[20],
            parent_seed[21],
            parent_seed[22],
            parent_seed[23],
        ]),
        u64::from_le_bytes([
            parent_seed[24],
            parent_seed[25],
            parent_seed[26],
            parent_seed[27],
            parent_seed[28],
            parent_seed[29],
            parent_seed[30],
            parent_seed[31],
        ]),
    ];

    let mixed = [
        mix(words[0] ^ domain_lo ^ index_lo),
        mix(words[1].rotate_left(17) ^ domain_hi ^ index_hi),
        mix(words[2].rotate_left(31) ^ domain_lo ^ index_hi),
        mix(words[3].rotate_left(47) ^ domain_hi ^ index_lo),
    ];

    output[0..8].copy_from_slice(&mixed[0].to_le_bytes());
    output[8..16].copy_from_slice(&mixed[1].to_le_bytes());
    output[16..24].copy_from_slice(&mixed[2].to_le_bytes());
    output[24..32].copy_from_slice(&mixed[3].to_le_bytes());

    output
}

/// Creates a deterministic sampler for one explicitly identified stream.
///
/// This function does not use:
///
/// - wall-clock time;
/// - process IDs;
/// - thread IDs;
/// - memory addresses;
/// - operating-system entropy.
///
/// Therefore identical inputs produce identical initial sampler state.
#[must_use]
pub fn deterministic_stream_sampler(
    parent_seed: [u8; 32],
    domain: u128,
    stream_index: u128,
) -> SeededSampler {
    SeededSampler::from_seed(derive_stream_seed(
        parent_seed,
        domain,
        stream_index,
    ))
}

// =============================================================================
// Streaming statistics
// =============================================================================

/// Online categorical counting result.
///
/// This type intentionally stores counts by outcome and therefore should only
/// be used when the outcome domain itself is known to be resource-bounded.
///
/// For enormous outcome domains, callers should consume `SampleStream` directly
/// into an external aggregation system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleCount {
    /// Number of observations processed.
    pub observations: u128,

    /// Number of observations matching the requested outcome.
    pub matches: u128,
}

impl SampleCount {
    /// Creates an empty counter.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            observations: 0,
            matches: 0,
        }
    }

    /// Records one observation.
    pub fn observe<T>(&mut self, sample: &Sample<T>, expected: &T)
    where
        T: PartialEq,
    {
        self.observations = self.observations.saturating_add(1);

        if &sample.outcome == expected {
            self.matches = self.matches.saturating_add(1);
        }
    }

    /// Returns the observed frequency.
    ///
    /// `None` means that no observations have been recorded.
    #[must_use]
    pub fn frequency(&self) -> Option<f64> {
        if self.observations == 0 {
            return None;
        }

        Some(self.matches as f64 / self.observations as f64)
    }
}

impl Default for SampleCount {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::zqn::probability::distribution::Distribution;

    fn binary_distribution() -> Distribution<bool> {
        Distribution::from_weighted(
            vec![
                (false, 0.25),
                (true, 0.75),
            ],
            1.0e-12,
        )
        .expect("valid binary distribution")
    }

    #[test]
    fn seeded_sampler_is_reproducible() {
        let distribution = binary_distribution();

        let seed = [7_u8; 32];

        let mut first = SeededSampler::from_seed(seed);
        let mut second = SeededSampler::from_seed(seed);

        let first_samples = first
            .sample_n(&distribution, 1_000)
            .expect("sampling should succeed");

        let second_samples = second
            .sample_n(&distribution, 1_000)
            .expect("sampling should succeed");

        assert_eq!(first_samples, second_samples);
    }

    #[test]
    fn sample_indices_are_monotonic() {
        let distribution = binary_distribution();

        let mut sampler = SeededSampler::from_seed([1_u8; 32]);

        let samples = sampler
            .sample_n(&distribution, 32)
            .expect("sampling should succeed");

        for (expected, sample) in samples.iter().enumerate() {
            assert_eq!(
                sample.index,
                SampleIndex::new(expected as u128)
            );
        }
    }

    #[test]
    fn zero_samples_can_be_valid() {
        let distribution = binary_distribution();

        let mut sampler = SeededSampler::from_seed([2_u8; 32]);

        let samples = sampler
            .sample_n(&distribution, 0)
            .expect("zero samples are allowed by default");

        assert!(samples.is_empty());
        assert_eq!(sampler.next_index(), SampleIndex::new(0));
    }

    #[test]
    fn zero_samples_can_be_forbidden() {
        let distribution = binary_distribution();

        let policy = SamplingPolicy {
            materialization: MaterializationPolicy::Bounded,
            allow_zero_samples: false,
        };

        let mut sampler =
            SeededSampler::from_seed_with_policy([3_u8; 32], policy);

        let result = sampler.sample_n(&distribution, 0);

        assert_eq!(
            result,
            Err(SamplingError::ZeroSamplesNotAllowed)
        );
    }

    #[test]
    fn streaming_does_not_require_materialized_batch() {
        let distribution = binary_distribution();

        let mut sampler = SeededSampler::from_seed([4_u8; 32]);

        let mut stream = sampler
            .stream(&distribution, 10_000)
            .expect("stream should construct");

        assert_eq!(stream.remaining(), 10_000);

        let first = stream
            .next()
            .expect("one sample should exist")
            .expect("sample should succeed");

        assert_eq!(first.index, SampleIndex::new(0));
        assert_eq!(stream.remaining(), 9_999);

        let consumed = stream.count();

        assert_eq!(consumed, 9_999);
        assert_eq!(sampler.next_index(), SampleIndex::new(10_000));
    }

    #[test]
    fn streaming_and_materialized_execution_are_equivalent_for_same_seed() {
        let distribution = binary_distribution();
        let seed = [5_u8; 32];

        let mut materialized = SeededSampler::from_seed(seed);
        let materialized_samples = materialized
            .sample_n(&distribution, 256)
            .expect("materialized sampling should succeed");

        let mut streaming = SeededSampler::from_seed(seed);

        let streamed_samples: Vec<_> = streaming
            .stream(&distribution, 256)
            .expect("stream should construct")
            .collect::<Result<Vec<_>, _>>()
            .expect("streamed sampling should succeed");

        assert_eq!(materialized_samples, streamed_samples);
    }

    #[test]
    fn stream_seed_derivation_is_deterministic() {
        let parent = [9_u8; 32];

        let first = derive_stream_seed(parent, 10, 20);
        let second = derive_stream_seed(parent, 10, 20);

        assert_eq!(first, second);
    }

    #[test]
    fn different_stream_indices_produce_different_seed_material() {
        let parent = [10_u8; 32];

        let first = derive_stream_seed(parent, 0, 0);
        let second = derive_stream_seed(parent, 0, 1);

        assert_ne!(first, second);
    }

    #[test]
    fn different_domains_produce_different_seed_material() {
        let parent = [11_u8; 32];

        let first = derive_stream_seed(parent, 0, 0);
        let second = derive_stream_seed(parent, 1, 0);

        assert_ne!(first, second);
    }

    #[test]
    fn deterministic_streams_are_reproducible() {
        let distribution = binary_distribution();

        let parent = [12_u8; 32];

        let mut first =
            deterministic_stream_sampler(parent, 42, 100);

        let mut second =
            deterministic_stream_sampler(parent, 42, 100);

        let a = first
            .sample_n(&distribution, 512)
            .expect("sampling should succeed");

        let b = second
            .sample_n(&distribution, 512)
            .expect("sampling should succeed");

        assert_eq!(a, b);
    }

    #[test]
    fn independent_stream_indices_do_not_share_initial_rng_state() {
        let distribution = binary_distribution();

        let parent = [13_u8; 32];

        let mut first =
            deterministic_stream_sampler(parent, 42, 0);

        let mut second =
            deterministic_stream_sampler(parent, 42, 1);

        let a = first
            .sample_n(&distribution, 64)
            .expect("sampling should succeed");

        let b = second
            .sample_n(&distribution, 64)
            .expect("sampling should succeed");

        assert_ne!(a, b);
    }

    #[test]
    fn sample_count_tracks_matches() {
        let distribution = binary_distribution();

        let mut sampler = SeededSampler::from_seed([14_u8; 32]);

        let samples = sampler
            .sample_n(&distribution, 1_000)
            .expect("sampling should succeed");

        let mut count = SampleCount::new();

        for sample in &samples {
            count.observe(sample, &true);
        }

        assert_eq!(count.observations, 1_000);
        assert!(count.matches <= count.observations);

        let frequency = count.frequency().expect("observations exist");

        assert!((0.0..=1.0).contains(&frequency));
    }

    #[test]
    fn empty_counter_has_no_frequency() {
        let counter = SampleCount::new();

        assert_eq!(counter.frequency(), None);
    }

    #[test]
    fn sample_index_checked_increment() {
        let index = SampleIndex::new(u128::MAX);

        assert_eq!(
            index.checked_next(),
            Err(SamplingError::IndexOverflow)
        );
    }

    #[test]
    fn stream_size_hint_is_exact_when_representable() {
        let distribution = binary_distribution();

        let mut sampler = SeededSampler::from_seed([15_u8; 32]);

        let stream = sampler
            .stream(&distribution, 123)
            .expect("stream should construct");

        assert_eq!(stream.size_hint(), (123, Some(123)));
    }

    #[test]
    fn streaming_only_policy_rejects_materialization() {
        let distribution = binary_distribution();

        let policy = SamplingPolicy {
            materialization: MaterializationPolicy::StreamingOnly,
            allow_zero_samples: true,
        };

        let mut sampler =
            SeededSampler::from_seed_with_policy([16_u8; 32], policy);

        assert_eq!(
            sampler.sample_n(&distribution, 1),
            Err(SamplingError::MaterializationForbidden)
        );
    }

    #[test]
    fn streaming_only_policy_still_allows_streaming() {
        let distribution = binary_distribution();

        let policy = SamplingPolicy {
            materialization: MaterializationPolicy::StreamingOnly,
            allow_zero_samples: true,
        };

        let mut sampler =
            SeededSampler::from_seed_with_policy([17_u8; 32], policy);

        let mut stream = sampler
            .stream(&distribution, 4)
            .expect("stream should be permitted");

        assert_eq!(
            stream
                .next()
                .expect("sample should exist")
                .expect("sample should succeed")
                .index,
            SampleIndex::new(0)
        );
    }

    #[test]
    fn explicit_sample_start_index_is_preserved() {
        let distribution = binary_distribution();

        let mut sampler = SeededSampler::from_seed([18_u8; 32]);

        sampler.set_next_index(SampleIndex::new(10_000));

        let sample = sampler
            .sample(&distribution)
            .expect("sampling should succeed");

        assert_eq!(sample.index, SampleIndex::new(10_000));
        assert_eq!(sampler.next_index(), SampleIndex::new(10_001));
    }

    #[test]
    fn reset_index_does_not_reset_rng() {
        let distribution = binary_distribution();

        let seed = [19_u8; 32];

        let mut first = SeededSampler::from_seed(seed);
        let mut second = SeededSampler::from_seed(seed);

        let first_sample = first
            .sample(&distribution)
            .expect("sampling should succeed");

        let second_sample = second
            .sample(&distribution)
            .expect("sampling should succeed");

        assert_eq!(first_sample, second_sample);

        first.reset_index();

        let repeated = first
            .sample(&distribution)
            .expect("sampling should succeed");

        let continued = second
            .sample(&distribution)
            .expect("sampling should succeed");

        /*
         * The logical index was reset, but the RNG was intentionally not.
         * Therefore the stochastic state continues rather than restarting.
         */
        assert_eq!(repeated.outcome, continued.outcome);
        assert_eq!(repeated.index, SampleIndex::new(0));
        assert_eq!(continued.index, SampleIndex::new(1));
    }
}