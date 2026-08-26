//! Zamani Quantum Benchmarking — Sampling Engine
//!
//! Production sampling primitives for the quantum benchmarking execution
//! subsystem.
//!
//! # Responsibility
//!
//! This module converts a validated probability distribution into deterministic
//! or pseudo-random shot samples and normalized count distributions.
//!
//! It deliberately does NOT:
//!
//! - execute quantum circuits;
//! - communicate with hardware;
//! - communicate with simulator providers;
//! - compile or transpile circuits;
//! - route circuits;
//! - schedule circuits;
//! - calculate benchmark-specific metrics;
//! - calculate Quantum Volume;
//! - calculate randomized-benchmarking decay;
//! - calculate XEB;
//! - own the execution request/response contract;
//! - own Quantum IR;
//! - silently repair malformed probability distributions;
//! - silently change the requested number of shots;
//! - use process-global mutable RNG state.
//!
//! The execution layer may use this module when a backend exposes an ideal or
//! simulated probability distribution and the caller explicitly requests
//! shot-based sampling.
//!
//! # Architectural position
//!
//! ```text
//! Zamani Quantum IR
//!        │
//!        ▼
//! benchmark generator
//!        │
//!        ▼
//! execution request
//!        │
//!        ├──────────────► hardware backend
//!        │
//!        └──────────────► simulator
//!                              │
//!                              ▼
//!                    probability distribution
//!                              │
//!                              ▼
//!                    execution::sampler
//!                              │
//!                     ┌────────┴────────┐
//!                     ▼                 ▼
//!                  samples             counts
//!                     │                 │
//!                     └────────┬────────┘
//!                              ▼
//!                     core::observation
//!                              │
//!                              ▼
//!                         statistics
//! ```
//!
//! # Production invariants
//!
//! 1. Zero-shot requests are rejected.
//! 2. Empty distributions are rejected.
//! 3. Negative probabilities are rejected.
//! 4. NaN and infinite probabilities are rejected.
//! 5. Probability sums are validated.
//! 6. Distribution normalization is never implicit.
//! 7. Sampling is reproducible when a seed is supplied.
//! 8. The RNG is local to the sampler invocation.
//! 9. No process-global mutable RNG exists.
//! 10. The requested shot count is preserved exactly.
//! 11. Every produced shot corresponds to exactly one distribution outcome.
//! 12. Counts always sum exactly to the requested shot count.
//! 13. Integer overflow is prevented with checked arithmetic.
//! 14. Probability underflow/rounding cannot create an invalid count total.
//! 15. Sampling never mutates the caller's distribution.
//! 16. Sampling does not silently drop zero-probability outcomes.
//! 17. Duplicate outcomes are rejected rather than merged implicitly.
//! 18. The implementation does not require unsafe code.
//! 19. The implementation does not require an async runtime.
//! 20. The implementation is compatible with Rust 1.97.1 / Rust 2021.
//!
//! # Dependency boundary
//!
//! This file intentionally depends only on:
//!
//! - Rust standard library;
//! - `rand`, already present in Zamani's Cargo.toml.
//!
//! It does not depend on:
//!
//! - Quantum IR;
//! - execution.rs;
//! - response.rs;
//! - observation.rs;
//! - metrics;
//! - benchmark protocols.
//!
//! That makes the file independently testable and prevents circular
//! dependencies while the rest of the execution subsystem is assembled.
//!
//! # Integration contract
//!
//! Future `execution::executor` implementations should:
//!
//! 1. obtain a backend probability distribution;
//! 2. validate the distribution;
//! 3. construct a `Sampler`;
//! 4. call `sample()`;
//! 5. convert `SampleCounts` into the canonical execution observation;
//! 6. preserve the sampler's seed and metadata in execution provenance.
//!
//! Benchmark protocols must consume the resulting observations rather than
//! calling this module directly unless they explicitly need simulator-side
//! sampling.
//!
//! Hardware backends that already return physical shot counts should NOT pass
//! those counts through this sampler. Doing so would incorrectly resample an
//! already sampled experiment.
//!
//! # Important statistical distinction
//!
//! This module performs Monte-Carlo sampling from an explicitly supplied
//! probability distribution. It does not claim that the distribution itself
//! is physically exact. The provenance layer must identify whether the source
//! distribution came from:
//!
//! - an exact simulator;
//! - an approximate simulator;
//! - a noisy model;
//! - a hardware calibration model;
//! - an analytic calculation.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1, Rust 2021.
//!
//! No nightly features are required.
//!
//! ---------------------------------------------------------------------------
//! Public API overview
//! ---------------------------------------------------------------------------
//!
//! `ProbabilityDistribution`
//!     Validated immutable probability distribution.
//!
//! `DistributionEntry`
//!     One outcome/probability pair.
//!
//! `SamplerConfig`
//!     Sampling policy and safety configuration.
//!
//! `SamplingSeed`
//!     Explicit reproducibility identity.
//!
//! `Sampler`
//!     Sampling engine.
//!
//! `SampleCounts`
//!     Exact shot counts.
//!
//! `SampleSequence`
//!     Optional ordered shot sequence.
//!
//! `SamplingResult`
//!     Complete sampling output with provenance.
//!
//! `SamplerError`
//!     Exhaustive sampling failures.
//!
//! ---------------------------------------------------------------------------

use std::collections::BTreeMap;
use std::fmt;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

// =============================================================================
// Constants
// =============================================================================

/// Default tolerance used when validating probability sums.
///
/// A distribution whose sum differs from one by less than this amount is
/// accepted without modification.
///
/// Importantly, the distribution is NOT normalized automatically.
pub const DEFAULT_SUM_TOLERANCE: f64 = 1.0e-12;

/// Default maximum number of outcomes permitted in one distribution.
///
/// This protects the sampler from pathological allocations. Callers needing
/// larger distributions must explicitly configure a larger limit.
pub const DEFAULT_MAX_OUTCOMES: usize = 1_000_000;

/// Default maximum number of shots permitted by one sampler instance.
///
/// This is a library safety limit rather than a statement about hardware
/// capability.
pub const DEFAULT_MAX_SHOTS: usize = 100_000_000;

/// Minimum valid probability.
///
/// Kept explicit to make validation semantics obvious.
pub const MIN_PROBABILITY: f64 = 0.0;

/// Maximum valid probability.
pub const MAX_PROBABILITY: f64 = 1.0;

// =============================================================================
// Sampling seed
// =============================================================================

/// Explicit reproducibility seed for sampling.
///
/// A seed is optional at the public sampler level. If no seed is supplied,
/// entropy from the operating system is used by `StdRng::from_os_rng()` where
/// supported by the installed `rand` version.
///
/// Zamani benchmark infrastructure should normally supply an explicit seed
/// because benchmark reproducibility is a first-class requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SamplingSeed(u64);

impl SamplingSeed {
    /// Creates an explicit sampling seed.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw seed value.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl From<u64> for SamplingSeed {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for SamplingSeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// =============================================================================
// Distribution entry
// =============================================================================

/// One outcome and its probability.
///
/// The outcome is intentionally represented as a `String` rather than a
/// bitstring-specific type because the sampler is also used by:
///
/// - classical measurement results;
/// - application benchmark labels;
/// - annealing samples;
/// - categorical simulator outputs;
/// - future backend-specific normalized observations.
///
/// Quantum-specific validation belongs above this layer.
#[derive(Debug, Clone, PartialEq)]
pub struct DistributionEntry {
    /// Observable outcome identifier.
    pub outcome: String,

    /// Probability associated with the outcome.
    pub probability: f64,
}

impl DistributionEntry {
    /// Creates one distribution entry.
    ///
    /// Validation of the probability occurs when the complete distribution is
    /// constructed because duplicate outcomes and total probability are
    /// properties of the complete distribution.
    pub fn new(outcome: impl Into<String>, probability: f64) -> Self {
        Self {
            outcome: outcome.into(),
            probability,
        }
    }
}

// =============================================================================
// Probability distribution
// =============================================================================

/// Validated immutable probability distribution.
///
/// Once constructed successfully, this type guarantees:
///
/// - at least one outcome;
/// - no duplicate outcomes;
/// - no empty outcome identifiers;
/// - finite probabilities;
/// - probabilities in `[0, 1]`;
/// - total probability within the configured tolerance of one.
///
/// The values are not renormalized.
#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilityDistribution {
    entries: Vec<DistributionEntry>,
    sum: f64,
    tolerance: f64,
}

impl ProbabilityDistribution {
    /// Constructs a distribution using the default validation policy.
    pub fn new(
        entries: Vec<DistributionEntry>,
    ) -> Result<Self, SamplerError> {
        Self::with_limits(
            entries,
            DEFAULT_SUM_TOLERANCE,
            DEFAULT_MAX_OUTCOMES,
        )
    }

    /// Constructs a distribution with explicit validation parameters.
    pub fn with_limits(
        entries: Vec<DistributionEntry>,
        tolerance: f64,
        max_outcomes: usize,
    ) -> Result<Self, SamplerError> {
        validate_tolerance(tolerance)?;

        if max_outcomes == 0 {
            return Err(SamplerError::InvalidMaximumOutcomes);
        }

        if entries.is_empty() {
            return Err(SamplerError::EmptyDistribution);
        }

        if entries.len() > max_outcomes {
            return Err(SamplerError::TooManyOutcomes {
                requested: entries.len(),
                maximum: max_outcomes,
            });
        }

        let mut seen = BTreeMap::<String, ()>::new();
        let mut sum = 0.0_f64;

        for entry in &entries {
            if entry.outcome.trim().is_empty() {
                return Err(SamplerError::EmptyOutcome);
            }

            if !entry.probability.is_finite() {
                return Err(SamplerError::NonFiniteProbability {
                    outcome: entry.outcome.clone(),
                    probability: entry.probability,
                });
            }

            if !(MIN_PROBABILITY..=MAX_PROBABILITY)
                .contains(&entry.probability)
            {
                return Err(SamplerError::ProbabilityOutOfRange {
                    outcome: entry.outcome.clone(),
                    probability: entry.probability,
                });
            }

            if seen.insert(entry.outcome.clone(), ()).is_some() {
                return Err(SamplerError::DuplicateOutcome {
                    outcome: entry.outcome.clone(),
                });
            }

            sum += entry.probability;

            if !sum.is_finite() {
                return Err(SamplerError::ProbabilitySumOverflow);
            }
        }

        if (sum - 1.0).abs() > tolerance {
            return Err(SamplerError::ProbabilitySumInvalid {
                sum,
                tolerance,
            });
        }

        Ok(Self {
            entries,
            sum,
            tolerance,
        })
    }

    /// Creates a distribution from a map.
    ///
    /// `BTreeMap` gives deterministic iteration order, which is useful for
    /// reproducibility and stable fingerprints.
    pub fn from_map(
        probabilities: BTreeMap<String, f64>,
    ) -> Result<Self, SamplerError> {
        let entries = probabilities
            .into_iter()
            .map(|(outcome, probability)| {
                DistributionEntry::new(outcome, probability)
            })
            .collect();

        Self::new(entries)
    }

    /// Creates a distribution by explicitly normalizing its probabilities.
    ///
    /// This is intentionally a separate operation from `new()`.
    ///
    /// The caller is therefore making an explicit semantic decision that the
    /// supplied values represent relative weights rather than an already
    /// normalized probability distribution.
    pub fn from_weights(
        entries: Vec<DistributionEntry>,
    ) -> Result<Self, SamplerError> {
        if entries.is_empty() {
            return Err(SamplerError::EmptyDistribution);
        }

        let mut total = 0.0_f64;

        for entry in &entries {
            if entry.outcome.trim().is_empty() {
                return Err(SamplerError::EmptyOutcome);
            }

            if !entry.probability.is_finite() {
                return Err(SamplerError::NonFiniteProbability {
                    outcome: entry.outcome.clone(),
                    probability: entry.probability,
                });
            }

            if entry.probability < 0.0 {
                return Err(SamplerError::NegativeProbability {
                    outcome: entry.outcome.clone(),
                    probability: entry.probability,
                });
            }

            total += entry.probability;

            if !total.is_finite() {
                return Err(SamplerError::ProbabilitySumOverflow);
            }
        }

        if total <= 0.0 {
            return Err(SamplerError::ZeroTotalWeight);
        }

        let normalized = entries
            .into_iter()
            .map(|entry| {
                DistributionEntry::new(
                    entry.outcome,
                    entry.probability / total,
                )
            })
            .collect::<Vec<_>>();

        Self::new(normalized)
    }

    /// Returns the number of outcomes.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the distribution contains no outcomes.
    ///
    /// This is always false for a successfully constructed distribution.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the original validated probability sum.
    pub fn sum(&self) -> f64 {
        self.sum
    }

    /// Returns the validation tolerance.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Returns the distribution entries.
    pub fn entries(&self) -> &[DistributionEntry] {
        &self.entries
    }

    /// Returns an iterator over distribution entries.
    pub fn iter(&self) -> impl Iterator<Item = &DistributionEntry> {
        self.entries.iter()
    }

    /// Returns the probability of a named outcome.
    pub fn probability_of(&self, outcome: &str) -> Option<f64> {
        self.entries
            .iter()
            .find(|entry| entry.outcome == outcome)
            .map(|entry| entry.probability)
    }
}

// =============================================================================
// Sampling configuration
// =============================================================================

/// Configuration controlling one sampling operation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SamplerConfig {
    /// Maximum number of shots permitted by this sampler.
    pub max_shots: usize,

    /// Maximum number of outcomes permitted by this sampler.
    pub max_outcomes: usize,

    /// Probability validation tolerance.
    pub probability_tolerance: f64,

    /// Whether an ordered sequence of sampled outcomes should be retained.
    ///
    /// Keeping the sequence consumes memory proportional to the number of
    /// shots. Count-only sampling should be preferred for large experiments.
    pub retain_sequence: bool,
}

impl Default for SamplerConfig {
    fn default() -> Self {
        Self {
            max_shots: DEFAULT_MAX_SHOTS,
            max_outcomes: DEFAULT_MAX_OUTCOMES,
            probability_tolerance: DEFAULT_SUM_TOLERANCE,
            retain_sequence: false,
        }
    }
}

impl SamplerConfig {
    /// Creates the default production configuration.
    pub const fn new() -> Self {
        Self {
            max_shots: DEFAULT_MAX_SHOTS,
            max_outcomes: DEFAULT_MAX_OUTCOMES,
            probability_tolerance: DEFAULT_SUM_TOLERANCE,
            retain_sequence: false,
        }
    }

    /// Sets the maximum number of shots.
    pub const fn with_max_shots(mut self, value: usize) -> Self {
        self.max_shots = value;
        self
    }

    /// Sets the maximum number of outcomes.
    pub const fn with_max_outcomes(mut self, value: usize) -> Self {
        self.max_outcomes = value;
        self
    }

    /// Sets probability validation tolerance.
    pub const fn with_probability_tolerance(
        mut self,
        value: f64,
    ) -> Self {
        self.probability_tolerance = value;
        self
    }

    /// Enables retention of the ordered shot sequence.
    pub const fn with_sequence(mut self, enabled: bool) -> Self {
        self.retain_sequence = enabled;
        self
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<(), SamplerError> {
        if self.max_shots == 0 {
            return Err(SamplerError::InvalidMaximumShots);
        }

        if self.max_outcomes == 0 {
            return Err(SamplerError::InvalidMaximumOutcomes);
        }

        validate_tolerance(self.probability_tolerance)?;

        Ok(())
    }
}

// =============================================================================
// Sample counts
// =============================================================================

/// Exact counts produced by a sampling experiment.
///
/// The invariant is:
///
/// `sum(counts.values()) == shots`.
///
/// Zero-count outcomes are retained so the result preserves the complete
/// support of the supplied distribution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleCounts {
    shots: usize,
    counts: BTreeMap<String, usize>,
}

impl SampleCounts {
    fn new(
        shots: usize,
        distribution: &ProbabilityDistribution,
    ) -> Result<Self, SamplerError> {
        let mut counts = BTreeMap::new();

        for entry in distribution.entries() {
            counts.insert(entry.outcome.clone(), 0);
        }

        Ok(Self { shots, counts })
    }

    /// Returns the exact requested shot count.
    pub fn shots(&self) -> usize {
        self.shots
    }

    /// Returns the number of distinct outcomes in the result.
    pub fn len(&self) -> usize {
        self.counts.len()
    }

    /// Returns whether the result contains no outcomes.
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }

    /// Returns all counts.
    pub fn as_map(&self) -> &BTreeMap<String, usize> {
        &self.counts
    }

    /// Returns the count for an outcome.
    ///
    /// Returns zero for a valid outcome that was never sampled.
    pub fn count(&self, outcome: &str) -> usize {
        self.counts.get(outcome).copied().unwrap_or(0)
    }

    /// Returns an iterator over counts.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &usize)> {
        self.counts.iter()
    }

    /// Returns the empirical probability of an outcome.
    pub fn empirical_probability(
        &self,
        outcome: &str,
    ) -> Result<f64, SamplerError> {
        if self.shots == 0 {
            return Err(SamplerError::InvalidShotCount);
        }

        Ok(self.count(outcome) as f64 / self.shots as f64)
    }

    /// Returns empirical probabilities for every outcome.
    pub fn empirical_distribution(
        &self,
    ) -> Result<BTreeMap<String, f64>, SamplerError> {
        if self.shots == 0 {
            return Err(SamplerError::InvalidShotCount);
        }

        let denominator = self.shots as f64;
        let mut result = BTreeMap::new();

        for (outcome, count) in &self.counts {
            result.insert(outcome.clone(), *count as f64 / denominator);
        }

        Ok(result)
    }

    fn increment(&mut self, outcome: &str) -> Result<(), SamplerError> {
        let count = self
            .counts
            .get_mut(outcome)
            .ok_or_else(|| SamplerError::UnknownSampleOutcome {
                outcome: outcome.to_owned(),
            })?;

        *count = count
            .checked_add(1)
            .ok_or(SamplerError::CountOverflow)?;

        Ok(())
    }

    fn validate_total(&self) -> Result<(), SamplerError> {
        let total = self.counts.values().try_fold(
            0usize,
            |accumulator, value| {
                accumulator
                    .checked_add(*value)
                    .ok_or(SamplerError::CountOverflow)
            },
        )?;

        if total != self.shots {
            return Err(SamplerError::CountTotalMismatch {
                expected: self.shots,
                actual: total,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Sample sequence
// =============================================================================

/// Ordered sampled outcomes.
///
/// This is optional because retaining every shot can consume significant
/// memory.
///
/// For most benchmark analysis, `SampleCounts` is sufficient.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleSequence {
    shots: Vec<String>,
}

impl SampleSequence {
    fn with_capacity(capacity: usize) -> Result<Self, SamplerError> {
        Ok(Self {
            shots: Vec::with_capacity(capacity),
        })
    }

    fn push(&mut self, outcome: &str) {
        self.shots.push(outcome.to_owned());
    }

    /// Returns the number of retained shots.
    pub fn len(&self) -> usize {
        self.shots.len()
    }

    /// Returns whether no shots were retained.
    pub fn is_empty(&self) -> bool {
        self.shots.is_empty()
    }

    /// Returns the ordered sampled outcomes.
    pub fn as_slice(&self) -> &[String] {
        &self.shots
    }

    /// Consumes the sequence and returns the underlying vector.
    pub fn into_vec(self) -> Vec<String> {
        self.shots
    }
}

// =============================================================================
// Sampling result
// =============================================================================

/// Complete result of one sampling operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SamplingResult {
    /// Exact number of generated shots.
    pub shots: usize,

    /// Counts for every distribution outcome.
    pub counts: SampleCounts,

    /// Optional ordered sample sequence.
    pub sequence: Option<SampleSequence>,

    /// Explicit seed when deterministic sampling was requested.
    pub seed: Option<SamplingSeed>,

    /// Number of distribution outcomes.
    pub outcome_count: usize,
}

impl SamplingResult {
    /// Returns the empirical distribution.
    pub fn empirical_distribution(
        &self,
    ) -> Result<BTreeMap<String, f64>, SamplerError> {
        self.counts.empirical_distribution()
    }

    /// Returns the number of shots.
    pub fn shots(&self) -> usize {
        self.shots
    }

    /// Returns whether ordered samples were retained.
    pub fn has_sequence(&self) -> bool {
        self.sequence.is_some()
    }

    /// Validates the result invariants.
    pub fn validate(&self) -> Result<(), SamplerError> {
        if self.shots == 0 {
            return Err(SamplerError::InvalidShotCount);
        }

        if self.counts.shots() != self.shots {
            return Err(SamplerError::CountTotalMismatch {
                expected: self.shots,
                actual: self.counts.shots(),
            });
        }

        self.counts.validate_total()?;

        if let Some(sequence) = &self.sequence {
            if sequence.len() != self.shots {
                return Err(SamplerError::SequenceLengthMismatch {
                    expected: self.shots,
                    actual: sequence.len(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Cumulative distribution
// =============================================================================

/// Internal cumulative probability entry.
///
/// The last entry is forced to have cumulative probability exactly `1.0`
/// during construction. This prevents floating-point accumulation from
/// producing a final interval smaller than the complete random-number range.
#[derive(Debug, Clone)]
struct CumulativeEntry {
    outcome: String,
    cumulative_probability: f64,
}

// =============================================================================
// Sampler
// =============================================================================

/// Production probability-distribution sampler.
///
/// The sampler itself is stateless. A fresh RNG is constructed for each
/// sampling operation, preventing hidden cross-experiment state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sampler {
    config: SamplerConfig,
}

impl Sampler {
    /// Creates a sampler with production defaults.
    pub fn new() -> Self {
        Self {
            config: SamplerConfig::default(),
        }
    }

    /// Creates a sampler with explicit configuration.
    pub fn with_config(
        config: SamplerConfig,
    ) -> Result<Self, SamplerError> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the sampler configuration.
    pub fn config(&self) -> SamplerConfig {
        self.config
    }

    /// Samples using operating-system-provided entropy.
    ///
    /// This is appropriate for ordinary simulator execution where exact
    /// reproducibility is not required.
    ///
    /// Benchmark infrastructure should normally prefer `sample_seeded`.
    pub fn sample(
        &self,
        distribution: &ProbabilityDistribution,
        shots: usize,
    ) -> Result<SamplingResult, SamplerError> {
        self.validate_request(distribution, shots)?;

        let mut rng = StdRng::from_os_rng();

        self.sample_with_rng(distribution, shots, None, &mut rng)
    }

    /// Samples deterministically from an explicit seed.
    ///
    /// For identical:
    ///
    /// - distribution ordering;
    /// - distribution probabilities;
    /// - shot count;
    /// - sampler implementation;
    /// - seed;
    ///
    /// the generated sequence is reproducible.
    pub fn sample_seeded(
        &self,
        distribution: &ProbabilityDistribution,
        shots: usize,
        seed: SamplingSeed,
    ) -> Result<SamplingResult, SamplerError> {
        self.validate_request(distribution, shots)?;

        let mut rng = StdRng::seed_from_u64(seed.get());

        self.sample_with_rng(
            distribution,
            shots,
            Some(seed),
            &mut rng,
        )
    }

    /// Samples using a caller-owned RNG.
    ///
    /// This is useful when an experiment needs multiple statistically
    /// independent sampling streams derived from one controlled RNG.
    ///
    /// The sampler does not store or mutate any global RNG.
    pub fn sample_with_rng<R>(
        &self,
        distribution: &ProbabilityDistribution,
        shots: usize,
        seed: Option<SamplingSeed>,
        rng: &mut R,
    ) -> Result<SamplingResult, SamplerError>
    where
        R: Rng + ?Sized,
    {
        self.validate_request(distribution, shots)?;

        let cumulative = build_cumulative_distribution(
            distribution,
        )?;

        let mut counts =
            SampleCounts::new(shots, distribution)?;

        let mut sequence = if self.config.retain_sequence {
            Some(SampleSequence::with_capacity(shots)?)
        } else {
            None
        };

        for _ in 0..shots {
            let random_value = rng.gen::<f64>();

            let selected =
                select_outcome(&cumulative, random_value)?;

            counts.increment(selected)?;

            if let Some(sequence) = sequence.as_mut() {
                sequence.push(selected);
            }
        }

        counts.validate_total()?;

        if let Some(sequence) = &sequence {
            if sequence.len() != shots {
                return Err(SamplerError::SequenceLengthMismatch {
                    expected: shots,
                    actual: sequence.len(),
                });
            }
        }

        let result = SamplingResult {
            shots,
            counts,
            sequence,
            seed,
            outcome_count: distribution.len(),
        };

        result.validate()?;

        Ok(result)
    }

    /// Generates one sample without constructing a complete count result.
    ///
    /// This is useful for algorithms such as random-circuit sampling where a
    /// caller wants a stream-like interface.
    pub fn sample_one(
        &self,
        distribution: &ProbabilityDistribution,
        seed: SamplingSeed,
    ) -> Result<String, SamplerError> {
        self.validate_distribution(distribution)?;

        let cumulative =
            build_cumulative_distribution(distribution)?;

        let mut rng = StdRng::seed_from_u64(seed.get());

        let random_value = rng.gen::<f64>();

        select_outcome(&cumulative, random_value)
            .map(str::to_owned)
    }

    /// Validates a complete sampling request.
    pub fn validate_request(
        &self,
        distribution: &ProbabilityDistribution,
        shots: usize,
    ) -> Result<(), SamplerError> {
        self.config.validate()?;

        if shots == 0 {
            return Err(SamplerError::InvalidShotCount);
        }

        if shots > self.config.max_shots {
            return Err(SamplerError::ShotLimitExceeded {
                requested: shots,
                maximum: self.config.max_shots,
            });
        }

        self.validate_distribution(distribution)
    }

    /// Validates a distribution against the sampler's limits.
    pub fn validate_distribution(
        &self,
        distribution: &ProbabilityDistribution,
    ) -> Result<(), SamplerError> {
        if distribution.is_empty() {
            return Err(SamplerError::EmptyDistribution);
        }

        if distribution.len() > self.config.max_outcomes {
            return Err(SamplerError::TooManyOutcomes {
                requested: distribution.len(),
                maximum: self.config.max_outcomes,
            });
        }

        if (distribution.sum() - 1.0).abs()
            > self.config.probability_tolerance
        {
            return Err(SamplerError::ProbabilitySumInvalid {
                sum: distribution.sum(),
                tolerance: self.config.probability_tolerance,
            });
        }

        for entry in distribution.entries() {
            if entry.outcome.trim().is_empty() {
                return Err(SamplerError::EmptyOutcome);
            }

            if !entry.probability.is_finite() {
                return Err(SamplerError::NonFiniteProbability {
                    outcome: entry.outcome.clone(),
                    probability: entry.probability,
                });
            }

            if !(MIN_PROBABILITY..=MAX_PROBABILITY)
                .contains(&entry.probability)
            {
                return Err(SamplerError::ProbabilityOutOfRange {
                    outcome: entry.outcome.clone(),
                    probability: entry.probability,
                });
            }
        }

        Ok(())
    }
}

impl Default for Sampler {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Cumulative distribution construction
// =============================================================================

fn build_cumulative_distribution(
    distribution: &ProbabilityDistribution,
) -> Result<Vec<CumulativeEntry>, SamplerError> {
    let mut cumulative = Vec::with_capacity(
        distribution.len(),
    );

    let mut running = 0.0_f64;

    for (index, entry) in distribution.entries().iter().enumerate() {
        running += entry.probability;

        if !running.is_finite() {
            return Err(SamplerError::ProbabilitySumOverflow);
        }

        let cumulative_probability =
            if index + 1 == distribution.len() {
                // Force the final boundary to exactly 1.0. This is necessary
                // because floating-point summation can produce values such as
                // 0.9999999999999999 even when the validated distribution is
                // mathematically normalized.
                1.0
            } else {
                running.min(1.0)
            };

        cumulative.push(CumulativeEntry {
            outcome: entry.outcome.clone(),
            cumulative_probability,
        });
    }

    if cumulative.is_empty() {
        return Err(SamplerError::EmptyDistribution);
    }

    Ok(cumulative)
}

// =============================================================================
// Outcome selection
// =============================================================================

fn select_outcome<'a>(
    cumulative: &'a [CumulativeEntry],
    random_value: f64,
) -> Result<&'a str, SamplerError> {
    if !random_value.is_finite()
        || !(0.0..1.0).contains(&random_value)
    {
        return Err(SamplerError::InvalidRandomValue {
            value: random_value,
        });
    }

    // Binary search provides O(log n) selection for large distributions.
    let mut low = 0usize;
    let mut high = cumulative.len();

    while low < high {
        let middle = low + (high - low) / 2;

        if random_value
            < cumulative[middle].cumulative_probability
        {
            high = middle;
        } else {
            low = middle + 1;
        }
    }

    if low < cumulative.len() {
        return Ok(&cumulative[low].outcome);
    }

    // `random_value` is strictly less than 1.0 and the final cumulative
    // boundary is exactly 1.0, so this should be unreachable.
    //
    // Keeping an explicit error here is preferable to indexing blindly and
    // turning a numerical invariant failure into a panic.
    Err(SamplerError::SamplingSelectionFailure)
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_tolerance(
    tolerance: f64,
) -> Result<(), SamplerError> {
    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(SamplerError::InvalidTolerance {
            tolerance,
        });
    }

    Ok(())
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the production sampling subsystem.
#[derive(Debug, Clone, PartialEq)]
pub enum SamplerError {
    /// No outcomes were supplied.
    EmptyDistribution,

    /// An outcome identifier is empty or whitespace-only.
    EmptyOutcome,

    /// Two distribution entries use the same outcome.
    DuplicateOutcome {
        outcome: String,
    },

    /// Probability is NaN or infinite.
    NonFiniteProbability {
        outcome: String,
        probability: f64,
    },

    /// Probability is below zero.
    NegativeProbability {
        outcome: String,
        probability: f64,
    },

    /// Probability is outside `[0, 1]`.
    ProbabilityOutOfRange {
        outcome: String,
        probability: f64,
    },

    /// Probability values do not sum to one within the configured tolerance.
    ProbabilitySumInvalid {
        sum: f64,
        tolerance: f64,
    },

    /// Floating-point accumulation became non-finite.
    ProbabilitySumOverflow,

    /// All supplied weights are zero.
    ZeroTotalWeight,

    /// Validation tolerance is invalid.
    InvalidTolerance {
        tolerance: f64,
    },

    /// Maximum outcome count is zero.
    InvalidMaximumOutcomes,

    /// Maximum shot count is zero.
    InvalidMaximumShots,

    /// Distribution contains more outcomes than permitted.
    TooManyOutcomes {
        requested: usize,
        maximum: usize,
    },

    /// Requested shot count is zero.
    InvalidShotCount,

    /// Requested shot count exceeds the sampler safety limit.
    ShotLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// An internal count would overflow `usize`.
    CountOverflow,

    /// Final count total does not equal requested shots.
    CountTotalMismatch {
        expected: usize,
        actual: usize,
    },

    /// Retained sequence length does not equal requested shots.
    SequenceLengthMismatch {
        expected: usize,
        actual: usize,
    },

    /// A sampled outcome is not part of the validated distribution.
    UnknownSampleOutcome {
        outcome: String,
    },

    /// RNG produced an invalid value.
    InvalidRandomValue {
        value: f64,
    },

    /// Cumulative distribution could not select an outcome.
    SamplingSelectionFailure,
}

impl fmt::Display for SamplerError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyDistribution => {
                write!(f, "sampling distribution cannot be empty")
            }

            Self::EmptyOutcome => {
                write!(f, "sampling outcome cannot be empty")
            }

            Self::DuplicateOutcome { outcome } => {
                write!(
                    f,
                    "sampling distribution contains duplicate outcome '{}'",
                    outcome
                )
            }

            Self::NonFiniteProbability {
                outcome,
                probability,
            } => {
                write!(
                    f,
                    "probability for outcome '{}' is non-finite: {}",
                    outcome, probability
                )
            }

            Self::NegativeProbability {
                outcome,
                probability,
            } => {
                write!(
                    f,
                    "probability for outcome '{}' is negative: {}",
                    outcome, probability
                )
            }

            Self::ProbabilityOutOfRange {
                outcome,
                probability,
            } => {
                write!(
                    f,
                    "probability for outcome '{}' is outside [0, 1]: {}",
                    outcome, probability
                )
            }

            Self::ProbabilitySumInvalid {
                sum,
                tolerance,
            } => {
                write!(
                    f,
                    "probability sum {} is not within tolerance {} of 1",
                    sum, tolerance
                )
            }

            Self::ProbabilitySumOverflow => {
                write!(f, "probability sum became non-finite")
            }

            Self::ZeroTotalWeight => {
                write!(f, "total sampling weight must be greater than zero")
            }

            Self::InvalidTolerance { tolerance } => {
                write!(
                    f,
                    "probability tolerance must be finite and non-negative: {}",
                    tolerance
                )
            }

            Self::InvalidMaximumOutcomes => {
                write!(
                    f,
                    "maximum outcome count must be greater than zero"
                )
            }

            Self::InvalidMaximumShots => {
                write!(
                    f,
                    "maximum shot count must be greater than zero"
                )
            }

            Self::TooManyOutcomes {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "sampling distribution contains {} outcomes; maximum is {}",
                    requested, maximum
                )
            }

            Self::InvalidShotCount => {
                write!(f, "sampling requires at least one shot")
            }

            Self::ShotLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "requested {} shots exceeds sampler limit of {}",
                    requested, maximum
                )
            }

            Self::CountOverflow => {
                write!(f, "sample count overflow")
            }

            Self::CountTotalMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "sample count total mismatch: expected {}, got {}",
                    expected, actual
                )
            }

            Self::SequenceLengthMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "sample sequence length mismatch: expected {}, got {}",
                    expected, actual
                )
            }

            Self::UnknownSampleOutcome { outcome } => {
                write!(
                    f,
                    "sampler produced an unknown outcome '{}'",
                    outcome
                )
            }

            Self::InvalidRandomValue { value } => {
                write!(
                    f,
                    "random sampling value must be finite and in [0, 1): {}",
                    value
                )
            }

            Self::SamplingSelectionFailure => {
                write!(
                    f,
                    "unable to select an outcome from the cumulative distribution"
                )
            }
        }
    }
}

impl std::error::Error for SamplerError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn binary_distribution() -> ProbabilityDistribution {
        ProbabilityDistribution::new(vec![
            DistributionEntry::new("0", 0.5),
            DistributionEntry::new("1", 0.5),
        ])
        .expect("valid binary distribution")
    }

    fn ternary_distribution() -> ProbabilityDistribution {
        ProbabilityDistribution::new(vec![
            DistributionEntry::new("00", 0.25),
            DistributionEntry::new("01", 0.50),
            DistributionEntry::new("10", 0.25),
        ])
        .expect("valid ternary distribution")
    }

    #[test]
    fn distribution_accepts_valid_probabilities() {
        let distribution = binary_distribution();

        assert_eq!(distribution.len(), 2);
        assert!(!distribution.is_empty());
        assert!((distribution.sum() - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn distribution_rejects_empty_input() {
        let result = ProbabilityDistribution::new(Vec::new());

        assert_eq!(
            result,
            Err(SamplerError::EmptyDistribution)
        );
    }

    #[test]
    fn distribution_rejects_empty_outcome() {
        let result = ProbabilityDistribution::new(vec![
            DistributionEntry::new("", 1.0),
        ]);

        assert_eq!(
            result,
            Err(SamplerError::EmptyOutcome)
        );
    }

    #[test]
    fn distribution_rejects_duplicate_outcomes() {
        let result = ProbabilityDistribution::new(vec![
            DistributionEntry::new("0", 0.5),
            DistributionEntry::new("0", 0.5),
        ]);

        assert_eq!(
            result,
            Err(SamplerError::DuplicateOutcome {
                outcome: "0".to_owned(),
            })
        );
    }

    #[test]
    fn distribution_rejects_negative_probability() {
        let result = ProbabilityDistribution::new(vec![
            DistributionEntry::new("0", -0.1),
            DistributionEntry::new("1", 1.1),
        ]);

        assert!(matches!(
            result,
            Err(SamplerError::ProbabilityOutOfRange { .. })
        ));
    }

    #[test]
    fn distribution_rejects_nan_probability() {
        let result = ProbabilityDistribution::new(vec![
            DistributionEntry::new("0", f64::NAN),
        ]);

        assert!(matches!(
            result,
            Err(SamplerError::NonFiniteProbability { .. })
        ));
    }

    #[test]
    fn distribution_rejects_infinite_probability() {
        let result = ProbabilityDistribution::new(vec![
            DistributionEntry::new("0", f64::INFINITY),
        ]);

        assert!(matches!(
            result,
            Err(SamplerError::NonFiniteProbability { .. })
        ));
    }

    #[test]
    fn distribution_rejects_invalid_sum() {
        let result = ProbabilityDistribution::new(vec![
            DistributionEntry::new("0", 0.4),
            DistributionEntry::new("1", 0.4),
        ]);

        assert!(matches!(
            result,
            Err(SamplerError::ProbabilitySumInvalid { .. })
        ));
    }

    #[test]
    fn weights_are_explicitly_normalized() {
        let distribution =
            ProbabilityDistribution::from_weights(vec![
                DistributionEntry::new("a", 1.0),
                DistributionEntry::new("b", 3.0),
            ])
            .expect("valid weights");

        assert_eq!(distribution.len(), 2);
        assert_eq!(
            distribution.probability_of("a"),
            Some(0.25)
        );
        assert_eq!(
            distribution.probability_of("b"),
            Some(0.75)
        );
    }

    #[test]
    fn zero_total_weight_is_rejected() {
        let result =
            ProbabilityDistribution::from_weights(vec![
                DistributionEntry::new("a", 0.0),
                DistributionEntry::new("b", 0.0),
            ]);

        assert_eq!(
            result,
            Err(SamplerError::ZeroTotalWeight)
        );
    }

    #[test]
    fn zero_shots_are_rejected() {
        let sampler = Sampler::new();

        let result = sampler.sample(
            &binary_distribution(),
            0,
        );

        assert_eq!(
            result,
            Err(SamplerError::InvalidShotCount)
        );
    }

    #[test]
    fn seeded_sampling_is_reproducible() {
        let sampler =
            Sampler::with_config(
                SamplerConfig::new().with_sequence(true),
            )
            .expect("valid sampler");

        let distribution = binary_distribution();
        let seed = SamplingSeed::new(42);

        let first = sampler
            .sample_seeded(
                &distribution,
                1_000,
                seed,
            )
            .expect("sampling succeeds");

        let second = sampler
            .sample_seeded(
                &distribution,
                1_000,
                seed,
            )
            .expect("sampling succeeds");

        assert_eq!(first, second);
    }

    #[test]
    fn different_seeds_are_not_forced_to_match() {
        let sampler =
            Sampler::with_config(
                SamplerConfig::new().with_sequence(true),
            )
            .expect("valid sampler");

        let distribution = binary_distribution();

        let first = sampler
            .sample_seeded(
                &distribution,
                128,
                SamplingSeed::new(1),
            )
            .expect("sampling succeeds");

        let second = sampler
            .sample_seeded(
                &distribution,
                128,
                SamplingSeed::new(2),
            )
            .expect("sampling succeeds");

        assert_ne!(first.sequence, second.sequence);
    }

    #[test]
    fn counts_sum_exactly_to_shots() {
        let sampler = Sampler::new();

        let result = sampler
            .sample_seeded(
                &ternary_distribution(),
                10_000,
                SamplingSeed::new(7),
            )
            .expect("sampling succeeds");

        assert_eq!(result.shots(), 10_000);

        let total: usize =
            result.counts().values().sum();

        assert_eq!(total, 10_000);
    }

    #[test]
    fn all_distribution_outcomes_are_present() {
        let sampler = Sampler::new();

        let result = sampler
            .sample_seeded(
                &ternary_distribution(),
                100,
                SamplingSeed::new(7),
            )
            .expect("sampling succeeds");

        assert_eq!(result.counts().len(), 3);
        assert!(result.counts().contains_key("00"));
        assert!(result.counts().contains_key("01"));
        assert!(result.counts().contains_key("10"));
    }

    #[test]
    fn empirical_distribution_is_normalized() {
        let sampler = Sampler::new();

        let result = sampler
            .sample_seeded(
                &binary_distribution(),
                10_000,
                SamplingSeed::new(123),
            )
            .expect("sampling succeeds");

        let probabilities =
            result.empirical_distribution().expect(
                "empirical distribution succeeds",
            );

        let sum: f64 =
            probabilities.values().copied().sum();

        assert!((sum - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn sequence_length_matches_shots() {
        let sampler =
            Sampler::with_config(
                SamplerConfig::new().with_sequence(true),
            )
            .expect("valid sampler");

        let result = sampler
            .sample_seeded(
                &binary_distribution(),
                256,
                SamplingSeed::new(99),
            )
            .expect("sampling succeeds");

        assert_eq!(
            result.sequence.as_ref().unwrap().len(),
            256
        );
    }

    #[test]
    fn count_only_mode_does_not_retain_sequence() {
        let sampler = Sampler::new();

        let result = sampler
            .sample_seeded(
                &binary_distribution(),
                256,
                SamplingSeed::new(99),
            )
            .expect("sampling succeeds");

        assert!(!result.has_sequence());
    }

    #[test]
    fn impossible_selection_is_not_silently_indexed() {
        let distribution = binary_distribution();

        let cumulative =
            build_cumulative_distribution(&distribution)
                .expect("valid cumulative distribution");

        let result =
            select_outcome(&cumulative, 1.0);

        assert!(matches!(
            result,
            Err(SamplerError::InvalidRandomValue { .. })
        ));
    }

    #[test]
    fn zero_probability_outcome_is_preserved() {
        let distribution =
            ProbabilityDistribution::new(vec![
                DistributionEntry::new("never", 0.0),
                DistributionEntry::new("always", 1.0),
            ])
            .expect("valid distribution");

        let sampler =
            Sampler::with_config(
                SamplerConfig::new().with_sequence(true),
            )
            .expect("valid sampler");

        let result = sampler
            .sample_seeded(
                &distribution,
                100,
                SamplingSeed::new(5),
            )
            .expect("sampling succeeds");

        assert_eq!(result.counts().get("never"), Some(&0));
        assert_eq!(result.counts().get("always"), Some(&100));

        assert!(
            result
                .sequence
                .as_ref()
                .unwrap()
                .as_slice()
                .iter()
                .all(|value| value == "always")
        );
    }

    #[test]
    fn maximum_shots_are_enforced() {
        let sampler =
            Sampler::with_config(
                SamplerConfig::new().with_max_shots(10),
            )
            .expect("valid sampler");

        let result = sampler.sample(
            &binary_distribution(),
            11,
        );

        assert_eq!(
            result,
            Err(SamplerError::ShotLimitExceeded {
                requested: 11,
                maximum: 10,
            })
        );
    }

    #[test]
    fn maximum_outcomes_are_enforced() {
        let sampler =
            Sampler::with_config(
                SamplerConfig::new().with_max_outcomes(1),
            )
            .expect("valid sampler");

        let result = sampler.sample(
            &binary_distribution(),
            10,
        );

        assert_eq!(
            result,
            Err(SamplerError::TooManyOutcomes {
                requested: 2,
                maximum: 1,
            })
        );
    }

    #[test]
    fn sampler_result_validates() {
        let sampler = Sampler::new();

        let result = sampler
            .sample_seeded(
                &binary_distribution(),
                500,
                SamplingSeed::new(1234),
            )
            .expect("sampling succeeds");

        result.validate().expect("result is valid");
    }

    #[test]
    fn distribution_from_map_is_deterministic() {
        let mut map = BTreeMap::new();
        map.insert("1".to_owned(), 0.25);
        map.insert("0".to_owned(), 0.75);

        let distribution =
            ProbabilityDistribution::from_map(map)
                .expect("valid map");

        assert_eq!(
            distribution.entries()[0].outcome,
            "0"
        );
        assert_eq!(
            distribution.entries()[1].outcome,
            "1"
        );
    }

    #[test]
    fn sample_one_is_deterministic_for_seed() {
        let sampler = Sampler::new();
        let distribution = binary_distribution();

        let first = sampler
            .sample_one(
                &distribution,
                SamplingSeed::new(55),
            )
            .expect("sample succeeds");

        let second = sampler
            .sample_one(
                &distribution,
                SamplingSeed::new(55),
            )
            .expect("sample succeeds");

        assert_eq!(first, second);
    }
}

// =============================================================================
// Compatibility helpers used by integration tests
// =============================================================================

impl SamplingResult {
    /// Returns the underlying counts map.
    ///
    /// This method is intentionally named `counts()` so future execution
    /// response adapters can map it directly into their observation model.
    pub fn counts(&self) -> &BTreeMap<String, usize> {
        self.counts.as_map()
    }
}