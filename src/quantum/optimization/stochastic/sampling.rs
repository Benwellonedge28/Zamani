//! Zamani Quantum Optimization — Bounded Statistical Sampling
//!
//! Backend-independent sampling primitives for stochastic optimization and
//! randomized verification.
//!
//! This module never executes a quantum circuit. It only selects indexed
//! observations/operations and computes statistics over caller-supplied data.
//! The canonical representation remains `crate::quantum::ir::QuantumCircuit`.
//!
//! Sampling is bounded by the requested sample size rather than by circuit
//! size. Uniform sampling without replacement uses Floyd's algorithm and is
//! O(K) additional memory for K sampled elements. No circuit-sized clone is
//! created for a small sample.
//!
//! Statistical estimates are estimates, never proofs of semantic equivalence.
//! Exact/exhaustive verification belongs to the verification subsystem.
//!
//! Deterministic sampling uses a local SplitMix64 generator so no RNG crate is
//! required. It is reproducible but not cryptographically secure.
//!
//! Rust 1.97 / 1.97.1, edition 2021. No unsafe code.

use std::collections::HashSet;
use std::fmt;

use crate::quantum::ir::QuantumCircuit;

use super::super::circuit::OperationId;

/// Result type for sampling operations.
pub type SamplingResult<T> = Result<T, SamplingError>;

/// Errors produced by sampling and statistical estimation.
#[derive(Debug, Clone, PartialEq)]
pub enum SamplingError {
    /// Requested sample is larger than its population.
    SampleLargerThanPopulation {
        requested: usize,
        population: usize,
    },

    /// A non-empty observation is required.
    EmptyPopulation,

    /// Sample-size configuration is invalid.
    InvalidSampleSize {
        requested: usize,
        reason: &'static str,
    },

    /// Confidence must satisfy 0 < confidence < 1.
    InvalidConfidenceLevel {
        confidence: f64,
    },

    /// An observation must be finite.
    NonFiniteObservation {
        index: usize,
        value: f64,
    },

    /// Supplied counts are inconsistent.
    InvalidObservationCount {
        population: usize,
        observations: usize,
    },

    /// Checked integer arithmetic detected overflow.
    ArithmeticOverflow {
        operation: &'static str,
    },
}

impl fmt::Display for SamplingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SampleLargerThanPopulation {
                requested,
                population,
            } => write!(
                f,
                "sample size {requested} exceeds population size {population}"
            ),

            Self::EmptyPopulation => {
                f.write_str("population is empty")
            }

            Self::InvalidSampleSize {
                requested,
                reason,
            } => {
                write!(
                    f,
                    "invalid sample size {requested}: {reason}"
                )
            }

            Self::InvalidConfidenceLevel {
                confidence,
            } => {
                write!(
                    f,
                    "invalid confidence level {confidence}; \
                     expected 0 < confidence < 1"
                )
            }

            Self::NonFiniteObservation {
                index,
                value,
            } => {
                write!(
                    f,
                    "observation {index} is not finite: {value}"
                )
            }

            Self::InvalidObservationCount {
                population,
                observations,
            } => {
                write!(
                    f,
                    "observation count {observations} is \
                     inconsistent with population {population}"
                )
            }

            Self::ArithmeticOverflow {
                operation,
            } => {
                write!(
                    f,
                    "integer overflow while calculating {operation}"
                )
            }
        }
    }
}

impl std::error::Error for SamplingError {}

/// Deterministic, non-cryptographic random generator.
///
/// SplitMix64 is used because it is compact, fast and requires no external
/// dependency. It is appropriate for compiler heuristics, sampling and
/// reproducible tests, but MUST NOT be used for cryptography.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    /// Creates a deterministic generator from `seed`.
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Returns the current generator state.
    ///
    /// This can be recorded in optimization provenance.
    #[must_use]
    pub const fn state(self) -> u64 {
        self.state
    }

    /// Produces the next 64-bit value.
    #[must_use]
    pub fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_add(0x9E37_79B9_7F4A_7C15);

        let mut z = self.state;

        z = (z ^ (z >> 30))
            .wrapping_mul(0xBF58_476D_1CE4_E5B9);

        z = (z ^ (z >> 27))
            .wrapping_mul(0x94D0_49BB_1331_11EB);

        z ^ (z >> 31)
    }

    /// Returns a uniformly distributed value in `[0, upper)`.
    ///
    /// Rejection sampling prevents modulo bias.
    pub fn gen_below(
        &mut self,
        upper: usize,
    ) -> SamplingResult<usize> {
        if upper == 0 {
            return Err(SamplingError::InvalidSampleSize {
                requested: 0,
                reason: "random upper bound must be greater than zero",
            });
        }

        let upper = u64::try_from(upper).map_err(|_| {
            SamplingError::ArithmeticOverflow {
                operation: "converting random upper bound to u64",
            }
        })?;

        let threshold = upper.wrapping_neg() % upper;

        loop {
            let value = self.next_u64();

            if value >= threshold {
                return usize::try_from(value % upper).map_err(|_| {
                    SamplingError::ArithmeticOverflow {
                        operation: "converting random index to usize",
                    }
                });
            }
        }
    }
}

/// Configuration for reproducible uniform sampling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SamplingConfig {
    sample_size: usize,
    seed: u64,
}

impl SamplingConfig {
    /// Creates a sampling configuration.
    pub const fn new(
        sample_size: usize,
        seed: u64,
    ) -> Self {
        Self {
            sample_size,
            seed,
        }
    }

    /// Returns the requested sample size.
    #[must_use]
    pub const fn sample_size(self) -> usize {
        self.sample_size
    }

    /// Returns the deterministic seed.
    #[must_use]
    pub const fn seed(self) -> u64 {
        self.seed
    }
}

impl Default for SamplingConfig {
    fn default() -> Self {
        Self::new(
            1024,
            0x5A4D_4E49_5341_4D50,
        )
    }
}

/// A sampled operation position in the canonical circuit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SampledOperation {
    index: usize,
}

impl SampledOperation {
    /// Creates an operation sample reference.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self { index }
    }

    /// Returns its operation index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.index
    }

    /// Converts the position to the optimizer-local operation ID.
    #[must_use]
    pub const fn operation_id(self) -> OperationId {
        OperationId::new(self.index)
    }
}

/// Result of uniform sampling from a finite indexed population.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexSample {
    indices: Vec<usize>,
    population_size: usize,
    seed: u64,
}

impl IndexSample {
    fn new(
        mut indices: Vec<usize>,
        population_size: usize,
        seed: u64,
    ) -> Self {
        indices.sort_unstable();

        Self {
            indices,
            population_size,
            seed,
        }
    }

    /// Returns selected indices in ascending order.
    #[must_use]
    pub fn indices(&self) -> &[usize] {
        &self.indices
    }

    /// Returns the number of selected observations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.indices.len()
    }

    /// Returns whether no observations were selected.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    /// Returns the source population size.
    #[must_use]
    pub const fn population_size(&self) -> usize {
        self.population_size
    }

    /// Returns the seed used to produce the sample.
    #[must_use]
    pub const fn seed(&self) -> u64 {
        self.seed
    }

    /// Iterates over optimizer operation references.
    pub fn operations(
        &self,
    ) -> impl Iterator<Item = SampledOperation> + '_ {
        self.indices
            .iter()
            .copied()
            .map(SampledOperation::new)
    }
}

/// Uniform backend-independent circuit/index sampler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CircuitSampler {
    config: SamplingConfig,
}

impl CircuitSampler {
    /// Creates a sampler.
    #[must_use]
    pub const fn new(
        config: SamplingConfig,
    ) -> Self {
        Self { config }
    }

    /// Creates a sampler with the stable module defaults.
    #[must_use]
    pub fn default_sampler() -> Self {
        Self::new(SamplingConfig::default())
    }

    /// Returns this sampler's configuration.
    #[must_use]
    pub const fn config(self) -> SamplingConfig {
        self.config
    }

    /// Samples operation positions from a canonical circuit.
    ///
    /// The circuit itself is never cloned.
    pub fn sample_circuit(
        &self,
        circuit: &QuantumCircuit,
    ) -> SamplingResult<IndexSample> {
        let population_size =
            circuit.operations().len();

        self.sample_indices(population_size)
    }

    /// Samples unique indices uniformly from
    /// `[0, population_size)`.
    ///
    /// Floyd's algorithm requires O(K) additional memory for a K-element
    /// sample. When almost the entire population is requested, the complement
    /// is sampled to avoid unnecessarily large hash-set storage.
    pub fn sample_indices(
        &self,
        population_size: usize,
    ) -> SamplingResult<IndexSample> {
        let k = self.config.sample_size;

        if k > population_size {
            return Err(
                SamplingError::SampleLargerThanPopulation {
                    requested: k,
                    population: population_size,
                },
            );
        }

        if k == 0 {
            return Ok(IndexSample::new(
                Vec::new(),
                population_size,
                self.config.seed,
            ));
        }

        if population_size == 0 {
            return Err(SamplingError::EmptyPopulation);
        }

        if k == population_size {
            return Ok(IndexSample::new(
                (0..population_size).collect(),
                population_size,
                self.config.seed,
            ));
        }

        /*
         * If more than half of the population is requested, sampling the
         * complement is usually cheaper in temporary memory.
         *
         * The final output is necessarily O(N) because the caller explicitly
         * requested almost every population element.
         */
        if k > population_size / 2 {
            let excluded_count =
                population_size - k;

            let excluded = sample_unique(
                excluded_count,
                population_size,
                self.config.seed,
            )?;

            let excluded_set: HashSet<usize> =
                excluded.into_iter().collect();

            let mut indices =
                Vec::with_capacity(k);

            for index in 0..population_size {
                if !excluded_set.contains(&index) {
                    indices.push(index);
                }
            }

            return Ok(IndexSample::new(
                indices,
                population_size,
                self.config.seed,
            ));
        }

        let indices = sample_unique(
            k,
            population_size,
            self.config.seed,
        )?;

        Ok(IndexSample::new(
            indices,
            population_size,
            self.config.seed,
        ))
    }
}

/// Uniformly selects `k` distinct values from `[0, n)`.
///
/// Floyd's algorithm avoids constructing a population-sized permutation.
fn sample_unique(
    k: usize,
    n: usize,
    seed: u64,
) -> SamplingResult<Vec<usize>> {
    if k == 0 {
        return Ok(Vec::new());
    }

    if n == 0 {
        return Err(SamplingError::EmptyPopulation);
    }

    if k > n {
        return Err(
            SamplingError::SampleLargerThanPopulation {
                requested: k,
                population: n,
            },
        );
    }

    let mut rng =
        DeterministicRng::new(seed);

    let mut selected =
        HashSet::with_capacity(k);

    let mut output =
        Vec::with_capacity(k);

    let start =
        n.checked_sub(k)
            .ok_or(
                SamplingError::ArithmeticOverflow {
                    operation:
                        "population_size - sample_size",
                },
            )?;

    /*
     * Floyd's algorithm:
     *
     * for j in [N-K, N):
     *     choose t uniformly from [0, j]
     *     if t already selected:
     *         select j
     *     else:
     *         select t
     */
    for j in start..n {
        let upper =
            j.checked_add(1)
                .ok_or(
                    SamplingError::ArithmeticOverflow {
                        operation:
                            "sampling upper bound j + 1",
                    },
                )?;

        let t =
            rng.gen_below(upper)?;

        let chosen =
            if selected.contains(&t) {
                j
            } else {
                t
            };

        selected.insert(chosen);
        output.push(chosen);
    }

    Ok(output)
}

/// Mean estimate with a two-sided confidence interval.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeanEstimate {
    mean: f64,
    standard_error: f64,
    lower: f64,
    upper: f64,
    confidence: f64,
    sample_size: usize,
    population_size: Option<usize>,
}

impl MeanEstimate {
    /// Returns the estimated mean.
    #[must_use]
    pub const fn mean(self) -> f64 {
        self.mean
    }

    /// Returns the estimated standard error.
    #[must_use]
    pub const fn standard_error(self) -> f64 {
        self.standard_error
    }

    /// Returns the lower confidence bound.
    #[must_use]
    pub const fn lower(self) -> f64 {
        self.lower
    }

    /// Returns the upper confidence bound.
    #[must_use]
    pub const fn upper(self) -> f64 {
        self.upper
    }

    /// Returns the requested confidence level.
    #[must_use]
    pub const fn confidence(self) -> f64 {
        self.confidence
    }

    /// Returns the number of observations.
    #[must_use]
    pub const fn sample_size(self) -> usize {
        self.sample_size
    }

    /// Returns the known finite population size.
    #[must_use]
    pub const fn population_size(self) -> Option<usize> {
        self.population_size
    }

    /// Returns the confidence interval half-width.
    #[must_use]
    pub const fn margin_of_error(self) -> f64 {
        self.upper - self.mean
    }
}

/// Estimates a mean using a normal-approximation confidence interval.
///
/// If `population_size` is supplied, observations are assumed to have been
/// selected without replacement and a finite-population correction is applied.
///
/// If the complete finite population was observed, uncertainty is exactly zero.
pub fn estimate_mean(
    observations: &[f64],
    population_size: Option<usize>,
    confidence: f64,
) -> SamplingResult<MeanEstimate> {
    validate_confidence(confidence)?;

    if observations.is_empty() {
        return Err(SamplingError::EmptyPopulation);
    }

    if let Some(population) = population_size {
        if population == 0 {
            return Err(SamplingError::EmptyPopulation);
        }

        if observations.len() > population {
            return Err(
                SamplingError::InvalidObservationCount {
                    population,
                    observations: observations.len(),
                },
            );
        }
    }

    for (index, value) in
        observations.iter().copied().enumerate()
    {
        if !value.is_finite() {
            return Err(
                SamplingError::NonFiniteObservation {
                    index,
                    value,
                },
            );
        }
    }

    let count = observations.len();

    let mean =
        observations.iter().copied().sum::<f64>()
            / count as f64;

    /*
     * Sample variance with Bessel correction.
     *
     * This intentionally uses the stable two-pass form rather than a
     * one-pass accumulation so ordinary compiler-sized data does not suffer
     * avoidable catastrophic cancellation.
     */
    let variance = if count > 1 {
        let mut sum_squared = 0.0;

        for value in observations {
            let delta = *value - mean;
            sum_squared += delta * delta;
        }

        sum_squared / (count - 1) as f64
    } else {
        0.0
    };

    let mut standard_error = if count > 1 {
        (variance / count as f64).sqrt()
    } else {
        0.0
    };

    if let Some(population) = population_size {
        if count == population {
            standard_error = 0.0;
        } else if count < population && population > 1 {
            let finite_population_correction =
                ((population - count) as f64
                    / (population - 1) as f64)
                    .sqrt();

            standard_error *=
                finite_population_correction;
        }
    }

    let critical_value =
        normal_critical_value(confidence);

    let margin =
        critical_value * standard_error;

    Ok(MeanEstimate {
        mean,
        standard_error,
        lower: mean - margin,
        upper: mean + margin,
        confidence,
        sample_size: count,
        population_size,
    })
}

/// Bernoulli/proportion estimate using Wilson's confidence interval.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RateEstimate {
    rate: f64,
    lower: f64,
    upper: f64,
    confidence: f64,
    successes: usize,
    sample_size: usize,
    population_size: Option<usize>,
}

impl RateEstimate {
    /// Returns the estimated rate.
    #[must_use]
    pub const fn rate(self) -> f64 {
        self.rate
    }

    /// Returns the lower confidence bound.
    #[must_use]
    pub const fn lower(self) -> f64 {
        self.lower
    }

    /// Returns the upper confidence bound.
    #[must_use]
    pub const fn upper(self) -> f64 {
        self.upper
    }

    /// Returns the confidence level.
    #[must_use]
    pub const fn confidence(self) -> f64 {
        self.confidence
    }

    /// Returns the number of successes.
    #[must_use]
    pub const fn successes(self) -> usize {
        self.successes
    }

    /// Returns the number of observations.
    #[must_use]
    pub const fn sample_size(self) -> usize {
        self.sample_size
    }

    /// Returns the known finite population size.
    #[must_use]
    pub const fn population_size(self) -> Option<usize> {
        self.population_size
    }

    /// Returns the confidence interval width.
    #[must_use]
    pub const fn interval_width(self) -> f64 {
        self.upper - self.lower
    }
}

/// Estimates a Bernoulli/proportion rate.
///
/// Wilson's interval is used instead of the simple Wald interval because it
/// behaves substantially better for small samples and rates near zero/one.
///
/// If the complete finite population is observed, the interval collapses to
/// the exact observed rate.
pub fn estimate_rate(
    successes: usize,
    sample_size: usize,
    population_size: Option<usize>,
    confidence: f64,
) -> SamplingResult<RateEstimate> {
    validate_confidence(confidence)?;

    if sample_size == 0 {
        return Err(SamplingError::InvalidSampleSize {
            requested: 0,
            reason:
                "rate estimation requires at least one observation",
        });
    }

    if successes > sample_size {
        return Err(
            SamplingError::InvalidObservationCount {
                population: sample_size,
                observations: successes,
            },
        );
    }

    if let Some(population) = population_size {
        if population == 0 {
            return Err(SamplingError::EmptyPopulation);
        }

        if sample_size > population {
            return Err(
                SamplingError::InvalidObservationCount {
                    population,
                    observations: sample_size,
                },
            );
        }
    }

    let n = sample_size as f64;

    let rate =
        successes as f64 / n;

    let z =
        normal_critical_value(confidence);

    let z2 = z * z;

    let denominator =
        1.0 + z2 / n;

    let center =
        (rate + z2 / (2.0 * n))
            / denominator;

    let half =
        z
            * (
                rate * (1.0 - rate) / n
                    + z2 / (4.0 * n * n)
            )
            .sqrt()
            / denominator;

    let mut lower =
        (center - half).max(0.0);

    let mut upper =
        (center + half).min(1.0);

    if let Some(population) = population_size {
        if sample_size == population {
            lower = rate;
            upper = rate;
        } else if population > 1 {
            let finite_population_correction =
                (
                    (population - sample_size) as f64
                        / (population - 1) as f64
                )
                .sqrt();

            let half =
                ((upper - lower) / 2.0)
                    * finite_population_correction;

            lower =
                (rate - half).max(0.0);

            upper =
                (rate + half).min(1.0);
        }
    }

    Ok(RateEstimate {
        rate,
        lower,
        upper,
        confidence,
        successes,
        sample_size,
        population_size,
    })
}

fn validate_confidence(
    confidence: f64,
) -> SamplingResult<()> {
    if !confidence.is_finite()
        || !(0.0 < confidence && confidence < 1.0)
    {
        return Err(
            SamplingError::InvalidConfidenceLevel {
                confidence,
            },
        );
    }

    Ok(())
}

/// Returns an approximate two-sided normal critical value.
///
/// The implementation is dependency-free and deterministic across platforms.
fn normal_critical_value(
    confidence: f64,
) -> f64 {
    inverse_standard_normal(
        (1.0 + confidence) / 2.0
    )
}

/// Acklam-style inverse standard-normal CDF approximation.
fn inverse_standard_normal(
    p: f64,
) -> f64 {
    const A: [f64; 6] = [
        -3.969_683_028_665_376e1,
        2.209_460_984_245_205e2,
        -2.759_285_104_469_687e2,
        1.383_577_518_672_69e2,
        -3.066_479_806_614_716e1,
        2.506_628_277_459_239,
    ];

    const B: [f64; 5] = [
        -5.447_609_879_822_406e1,
        1.615_858_368_580_409e2,
        -1.556_989_798_598_866e2,
        6.680_131_188_771_972e1,
        -1.328_068_155_288_572e1,
    ];

    const C: [f64; 6] = [
        -7.784_894_002_430_293e-3,
        -3.223_964_580_411_365e-1,
        -2.400_758_277_161_838,
        -2.549_732_539_343_734,
        4.374_664_141_464_968,
        2.938_163_982_698_783,
    ];

    const D: [f64; 4] = [
        7.784_695_709_041_462e-3,
        3.224_671_290_700_398e-1,
        2.445_134_137_142_996,
        3.754_408_661_907_416,
    ];

    const LOW: f64 = 0.02425;
    const HIGH: f64 = 1.0 - LOW;

    if p < LOW {
        let q =
            (-2.0 * p.ln()).sqrt();

        return (((((C[0] * q + C[1]) * q + C[2])
            * q
            + C[3])
            * q
            + C[4])
            * q)
            + C[5])
            / ((((D[0] * q + D[1]) * q + D[2])
                * q
                + D[3])
                * q
                + 1.0);
    }

    if p > HIGH {
        let q =
            (-2.0 * (1.0 - p).ln()).sqrt();

        return -(((((C[0] * q + C[1]) * q + C[2])
            * q
            + C[3])
            * q
            + C[4])
            * q)
            + C[5])
            / ((((D[0] * q + D[1]) * q + D[2])
                * q
                + D[3])
                * q
                + 1.0);
    }

    let q =
        p - 0.5;

    let r =
        q * q;

    (((((A[0] * r + A[1]) * r + A[2])
        * r
        + A[3])
        * r
        + A[4])
        * r
        + A[5])
        * q)
        / (((((B[0] * r + B[1]) * r + B[2])
            * r
            + B[3])
            * r
            + B[4])
            * r)
            + 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_rng_is_reproducible() {
        let mut a =
            DeterministicRng::new(1234);

        let mut b =
            DeterministicRng::new(1234);

        for _ in 0..128 {
            assert_eq!(
                a.next_u64(),
                b.next_u64()
            );
        }
    }

    #[test]
    fn different_seeds_differ() {
        let mut a =
            DeterministicRng::new(1);

        let mut b =
            DeterministicRng::new(2);

        assert_ne!(
            a.next_u64(),
            b.next_u64()
        );
    }

    #[test]
    fn sample_is_unique_sorted_and_bounded() {
        let sampler =
            CircuitSampler::new(
                SamplingConfig::new(
                    100,
                    7,
                ),
            );

        let sample =
            sampler
                .sample_indices(10_000)
                .expect(
                    "sampling must succeed",
                );

        assert_eq!(
            sample.len(),
            100
        );

        assert!(
            sample
                .indices()
                .windows(2)
                .all(|w| w[0] < w[1])
        );

        assert!(
            sample
                .indices()
                .iter()
                .all(|&i| i < 10_000)
        );
    }

    #[test]
    fn sampling_is_reproducible() {
        let config =
            SamplingConfig::new(
                64,
                0x1234_5678,
            );

        let a =
            CircuitSampler::new(config)
                .sample_indices(1_000)
                .expect(
                    "sampling must succeed",
                );

        let b =
            CircuitSampler::new(config)
                .sample_indices(1_000)
                .expect(
                    "sampling must succeed",
                );

        assert_eq!(a, b);
    }

    #[test]
    fn sampling_changes_with_seed() {
        let a =
            CircuitSampler::new(
                SamplingConfig::new(64, 1),
            )
            .sample_indices(1_000)
            .expect(
                "sampling must succeed",
            );

        let b =
            CircuitSampler::new(
                SamplingConfig::new(64, 2),
            )
            .sample_indices(1_000)
            .expect(
                "sampling must succeed",
            );

        assert_ne!(
            a.indices(),
            b.indices()
        );
    }

    #[test]
    fn full_population_is_exact() {
        let sample =
            CircuitSampler::new(
                SamplingConfig::new(5, 99),
            )
            .sample_indices(5)
            .expect(
                "sampling must succeed",
            );

        assert_eq!(
            sample.indices(),
            &[0, 1, 2, 3, 4]
        );
    }

    #[test]
    fn zero_sample_is_allowed() {
        let sample =
            CircuitSampler::new(
                SamplingConfig::new(0, 99),
            )
            .sample_indices(100)
            .expect(
                "sampling must succeed",
            );

        assert!(sample.is_empty());
    }

    #[test]
    fn oversized_sample_is_rejected() {
        let error =
            CircuitSampler::new(
                SamplingConfig::new(11, 1),
            )
            .sample_indices(10)
            .expect_err(
                "oversized sample must fail",
            );

        assert!(matches!(
            error,
            SamplingError::SampleLargerThanPopulation { .. }
        ));
    }

    #[test]
    fn mean_estimate_is_centered() {
        let estimate =
            estimate_mean(
                &[1.0, 2.0, 3.0, 4.0, 5.0],
                None,
                0.95,
            )
            .expect(
                "mean estimate must succeed",
            );

        assert!(
            (estimate.mean() - 3.0).abs()
                < 1.0e-12
        );

        assert!(
            estimate.lower() < 3.0
        );

        assert!(
            estimate.upper() > 3.0
        );
    }

    #[test]
    fn full_population_mean_is_exact() {
        let estimate =
            estimate_mean(
                &[1.0, 2.0, 3.0],
                Some(3),
                0.95,
            )
            .expect(
                "mean estimate must succeed",
            );

        assert_eq!(
            estimate.standard_error(),
            0.0
        );

        assert_eq!(
            estimate.lower(),
            estimate.mean()
        );

        assert_eq!(
            estimate.upper(),
            estimate.mean()
        );
    }

    #[test]
    fn rate_estimate_is_bounded() {
        let estimate =
            estimate_rate(
                50,
                100,
                None,
                0.95,
            )
            .expect(
                "rate estimate must succeed",
            );

        assert_eq!(
            estimate.rate(),
            0.5
        );

        assert!(
            estimate.lower() >= 0.0
        );

        assert!(
            estimate.upper() <= 1.0
        );

        assert!(
            estimate.lower() < 0.5
        );

        assert!(
            estimate.upper() > 0.5
        );
    }

    #[test]
    fn full_population_rate_is_exact() {
        let estimate =
            estimate_rate(
                2,
                4,
                Some(4),
                0.95,
            )
            .expect(
                "rate estimate must succeed",
            );

        assert_eq!(
            estimate.lower(),
            0.5
        );

        assert_eq!(
            estimate.upper(),
            0.5
        );
    }

    #[test]
    fn non_finite_mean_is_rejected() {
        let error =
            estimate_mean(
                &[1.0, f64::NAN],
                None,
                0.95,
            )
            .expect_err(
                "NaN must be rejected",
            );

        assert!(matches!(
            error,
            SamplingError::NonFiniteObservation {
                index: 1,
                ..
            }
        ));
    }

    #[test]
    fn infinite_mean_is_rejected() {
        let error =
            estimate_mean(
                &[1.0, f64::INFINITY],
                None,
                0.95,
            )
            .expect_err(
                "infinity must be rejected",
            );

        assert!(matches!(
            error,
            SamplingError::NonFiniteObservation {
                index: 1,
                ..
            }
        ));
    }

    #[test]
    fn invalid_confidence_is_rejected() {
        assert!(matches!(
            estimate_mean(
                &[1.0],
                None,
                1.0
            ),
            Err(
                SamplingError::InvalidConfidenceLevel { .. }
            )
        ));

        assert!(matches!(
            estimate_rate(
                1,
                1,
                None,
                0.0
            ),
            Err(
                SamplingError::InvalidConfidenceLevel { .. }
            )
        ));
    }

    #[test]
    fn operation_reference_is_stable() {
        let operation =
            SampledOperation::new(42);

        assert_eq!(
            operation.index(),
            42
        );

        assert_eq!(
            operation.operation_id(),
            OperationId::new(42)
        );
    }
}