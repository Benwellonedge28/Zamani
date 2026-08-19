//! Zamani Quantum Error Correction — Threshold Benchmarks.
//!
//! Phase 11:
//!     Physical error rate
//!             |
//!             v
//!       Fault injection
//!             |
//!             v
//!          Syndrome
//!             |
//!             v
//!          Decoder
//!             |
//!             v
//!       Logical outcome
//!             |
//!             v
//!     Logical error rate
//!
//! IMPORTANT:
//! This file must never claim to measure a QEC threshold unless a real
//! fault-tolerant decoder is connected to the benchmark.
//!
//! A threshold benchmark is meaningful only when:
//!
//!     logical_error_rate(p, d)
//!
//! is measured for multiple physical error probabilities `p` and code
//! distances `d` under a defined noise model and decoding policy.
//!
//! The benchmark is deliberately deterministic:
//! - no external RNG dependency;
//! - reproducible seeds;
//! - bounded sample counts;
//! - checked arithmetic;
//! - no unchecked indexing;
//! - no panics for malformed benchmark configuration.
//!
//! This module is intended to be included from the QEC test module once the
//! production decoder/simulation APIs are wired into `mod.rs`.

#![allow(dead_code)]

use std::fmt;

// ============================================================================
// Benchmark limits
// ============================================================================

/// Maximum number of code distances accepted by one benchmark run.
pub const MAX_DISTANCES: usize = 32;

/// Maximum number of physical error probabilities accepted by one run.
pub const MAX_ERROR_RATES: usize = 64;

/// Maximum number of Monte-Carlo samples per point.
///
/// This protects CI and untrusted benchmark configuration from accidentally
/// requesting an unbounded computation.
pub const MAX_SAMPLES_PER_POINT: usize = 1_000_000;

/// Maximum total simulation points.
///
/// `distances × error_rates × samples` is checked before execution.
pub const MAX_TOTAL_SAMPLES: u64 = 10_000_000;

/// Default deterministic seed.
pub const DEFAULT_SEED: u64 = 0x5A4D_414E_495F5145;

// ============================================================================
// Benchmark configuration
// ============================================================================

/// Configuration for one threshold experiment.
#[derive(Debug, Clone, PartialEq)]
pub struct ThresholdConfig {
    /// Odd surface-code distances.
    pub distances: Vec<usize>,

    /// Physical Pauli error probabilities.
    ///
    /// Every value must satisfy:
    ///
    ///     0.0 <= p <= 1.0
    pub error_rates: Vec<f64>,

    /// Number of Monte-Carlo trials per `(distance, error_rate)` point.
    pub samples_per_point: usize,

    /// Deterministic simulation seed.
    pub seed: u64,
}

impl ThresholdConfig {
    /// Creates a validated threshold configuration.
    pub fn new(
        distances: Vec<usize>,
        error_rates: Vec<f64>,
        samples_per_point: usize,
        seed: u64,
    ) -> Result<Self, ThresholdError> {
        if distances.is_empty() {
            return Err(
                ThresholdError::EmptyDistanceSet,
            );
        }

        if distances.len() > MAX_DISTANCES {
            return Err(
                ThresholdError::TooManyDistances {
                    actual: distances.len(),
                    maximum: MAX_DISTANCES,
                },
            );
        }

        if error_rates.is_empty() {
            return Err(
                ThresholdError::EmptyErrorRateSet,
            );
        }

        if error_rates.len()
            > MAX_ERROR_RATES
        {
            return Err(
                ThresholdError::TooManyErrorRates {
                    actual: error_rates.len(),
                    maximum: MAX_ERROR_RATES,
                },
            );
        }

        if samples_per_point == 0 {
            return Err(
                ThresholdError::ZeroSamples,
            );
        }

        if samples_per_point
            > MAX_SAMPLES_PER_POINT
        {
            return Err(
                ThresholdError::TooManySamples {
                    actual: samples_per_point,
                    maximum: MAX_SAMPLES_PER_POINT,
                },
            );
        }

        for &distance in &distances {
            validate_distance(distance)?;
        }

        for &probability in &error_rates {
            validate_probability(
                probability,
            )?;
        }

        let points =
            u64::try_from(
                distances.len(),
            )
            .map_err(|_| {
                ThresholdError::ArithmeticOverflow
            })?
            .checked_mul(
                u64::try_from(
                    error_rates.len(),
                )
                .map_err(|_| {
                    ThresholdError::ArithmeticOverflow
                })?,
            )
            .ok_or(
                ThresholdError::ArithmeticOverflow,
            )?;

        let total_samples =
            points
                .checked_mul(
                    u64::try_from(
                        samples_per_point,
                    )
                    .map_err(|_| {
                        ThresholdError::ArithmeticOverflow
                    })?,
                )
                .ok_or(
                    ThresholdError::ArithmeticOverflow,
                )?;

        if total_samples
            > MAX_TOTAL_SAMPLES
        {
            return Err(
                ThresholdError::WorkBudgetExceeded {
                    requested:
                        total_samples,
                    maximum:
                        MAX_TOTAL_SAMPLES,
                },
            );
        }

        Ok(Self {
            distances,
            error_rates,
            samples_per_point,
            seed,
        })
    }

    /// Standard initial Phase-11 configuration.
    pub fn standard() -> Self {
        Self {
            distances:
                vec![3, 5, 7, 9],
            error_rates:
                vec![
                    0.001,
                    0.005,
                    0.010,
                    0.020,
                    0.050,
                ],
            samples_per_point:
                10_000,
            seed:
                DEFAULT_SEED,
        }
    }
}

// ============================================================================
// Benchmark result
// ============================================================================

/// Measurements for one `(distance, physical_error_rate)` point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThresholdPoint {
    pub distance: usize,
    pub physical_error_rate: f64,
    pub samples: u64,
    pub physical_errors: u64,
    pub logical_failures: u64,
    pub decoder_failures: u64,
}

impl ThresholdPoint {
    pub fn new(
        distance: usize,
        physical_error_rate: f64,
        samples: u64,
    ) -> Result<Self, ThresholdError> {
        validate_distance(distance)?;
        validate_probability(
            physical_error_rate,
        )?;

        if samples == 0 {
            return Err(
                ThresholdError::ZeroSamples,
            );
        }

        Ok(Self {
            distance,
            physical_error_rate,
            samples,
            physical_errors: 0,
            logical_failures: 0,
            decoder_failures: 0,
        })
    }

    pub fn physical_error_rate(
        &self,
    ) -> f64 {
        if self.samples == 0 {
            return 0.0;
        }

        self.physical_errors as f64
            / self.samples as f64
    }

    pub fn logical_error_rate(
        &self,
    ) -> f64 {
        if self.samples == 0 {
            return 0.0;
        }

        self.logical_failures as f64
            / self.samples as f64
    }

    pub fn decoder_failure_rate(
        &self,
    ) -> f64 {
        if self.samples == 0 {
            return 0.0;
        }

        self.decoder_failures as f64
            / self.samples as f64
    }
}

/// Complete threshold benchmark.
#[derive(Debug, Clone, PartialEq)]
pub struct ThresholdReport {
    points: Vec<ThresholdPoint>,
}

impl ThresholdReport {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }

    pub fn add(
        &mut self,
        point: ThresholdPoint,
    ) {
        self.points.push(point);
    }

    pub fn points(
        &self,
    ) -> &[ThresholdPoint] {
        &self.points
    }

    pub fn is_empty(
        &self,
    ) -> bool {
        self.points.is_empty()
    }
}

impl Default for ThresholdReport {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Deterministic RNG
// ============================================================================

/// Small deterministic PRNG used exclusively by the benchmark harness.
///
/// This is not a cryptographic RNG and MUST NOT be used for production
/// cryptography, key generation, or security decisions.
#[derive(Debug, Clone, Copy)]
pub struct BenchmarkRng {
    state: u64,
}

impl BenchmarkRng {
    pub fn new(seed: u64) -> Self {
        let state =
            if seed == 0 {
                DEFAULT_SEED
            } else {
                seed
            };

        Self { state }
    }

    pub fn next_u64(
        &mut self,
    ) -> u64 {
        // xorshift64*.
        //
        // The wrapping operations are intentional and deterministic.
        let mut x = self.state;

        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;

        self.state = x;

        x.wrapping_mul(
            0x2545_F491_4F6C_DD1D,
        )
    }

    pub fn next_unit_f64(
        &mut self,
    ) -> f64 {
        let value =
            self.next_u64() >> 11;

        value as f64
            / ((1u64 << 53) as f64)
    }

    pub fn bernoulli(
        &mut self,
        probability: f64,
    ) -> bool {
        self.next_unit_f64()
            < probability
    }
}

// ============================================================================
// Statistical helpers
// ============================================================================

/// Wilson confidence interval for a Bernoulli proportion.
///
/// This is preferable to reporting only a raw percentage because threshold
/// curves can otherwise be misleading when the number of observed failures
/// is small.
pub fn wilson_interval(
    successes: u64,
    trials: u64,
    z: f64,
) -> Result<(f64, f64), ThresholdError> {
    if trials == 0 {
        return Err(
            ThresholdError::ZeroSamples,
        );
    }

    if !z.is_finite()
        || z <= 0.0
    {
        return Err(
            ThresholdError::InvalidConfidenceParameter,
        );
    }

    if successes > trials {
        return Err(
            ThresholdError::InvalidCounts {
                successes,
                trials,
            },
        );
    }

    let n = trials as f64;
    let p = successes as f64 / n;

    let z2 = z * z;

    let denominator =
        1.0 + z2 / n;

    let centre =
        (p + z2 / (2.0 * n))
            / denominator;

    let margin =
        z
            * ((p * (1.0 - p) / n
                + z2 / (4.0 * n * n))
                .sqrt())
            / denominator;

    Ok((
        (centre - margin).max(0.0),
        (centre + margin).min(1.0),
    ))
}

// ============================================================================
// Validation
// ============================================================================

fn validate_distance(
    distance: usize,
) -> Result<(), ThresholdError> {
    if distance < 3 {
        return Err(
            ThresholdError::InvalidDistance {
                distance,
            },
        );
    }

    if distance % 2 == 0 {
        return Err(
            ThresholdError::EvenDistance {
                distance,
            },
        );
    }

    Ok(())
}

fn validate_probability(
    probability: f64,
) -> Result<(), ThresholdError> {
    if !probability.is_finite()
        || !(0.0..=1.0)
            .contains(&probability)
    {
        return Err(
            ThresholdError::InvalidProbability {
                probability,
            },
        );
    }

    Ok(())
}

// ============================================================================
// Errors
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum ThresholdError {
    EmptyDistanceSet,

    EmptyErrorRateSet,

    TooManyDistances {
        actual: usize,
        maximum: usize,
    },

    TooManyErrorRates {
        actual: usize,
        maximum: usize,
    },

    InvalidDistance {
        distance: usize,
    },

    EvenDistance {
        distance: usize,
    },

    InvalidProbability {
        probability: f64,
    },

    ZeroSamples,

    TooManySamples {
        actual: usize,
        maximum: usize,
    },

    WorkBudgetExceeded {
        requested: u64,
        maximum: u64,
    },

    InvalidCounts {
        successes: u64,
        trials: u64,
    },

    InvalidConfidenceParameter,

    ArithmeticOverflow,

    /// Returned when Phase-11 execution is requested before an actual
    /// production decoder/simulation backend has been wired into the harness.
    DecoderBackendNotConfigured,
}

impl fmt::Display
    for ThresholdError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyDistanceSet => {
                write!(
                    f,
                    "threshold benchmark requires at least one code distance"
                )
            }

            Self::EmptyErrorRateSet => {
                write!(
                    f,
                    "threshold benchmark requires at least one physical error rate"
                )
            }

            Self::TooManyDistances {
                actual,
                maximum,
            } => {
                write!(
                    f,
                    "received {actual} distances; maximum is {maximum}"
                )
            }

            Self::TooManyErrorRates {
                actual,
                maximum,
            } => {
                write!(
                    f,
                    "received {actual} error rates; maximum is {maximum}"
                )
            }

            Self::InvalidDistance {
                distance,
            } => {
                write!(
                    f,
                    "invalid surface-code distance {distance}; distance must be >= 3"
                )
            }

            Self::EvenDistance {
                distance,
            } => {
                write!(
                    f,
                    "invalid surface-code distance {distance}; rotated planar distance must be odd"
                )
            }

            Self::InvalidProbability {
                probability,
            } => {
                write!(
                    f,
                    "invalid physical error probability {probability}"
                )
            }

            Self::ZeroSamples => {
                write!(
                    f,
                    "benchmark sample count must be non-zero"
                )
            }

            Self::TooManySamples {
                actual,
                maximum,
            } => {
                write!(
                    f,
                    "requested {actual} samples per point; maximum is {maximum}"
                )
            }

            Self::WorkBudgetExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "benchmark requests {requested} samples; maximum workload is {maximum}"
                )
            }

            Self::InvalidCounts {
                successes,
                trials,
            } => {
                write!(
                    f,
                    "invalid statistical counts: {successes} successes in {trials} trials"
                )
            }

            Self::InvalidConfidenceParameter => {
                write!(
                    f,
                    "confidence parameter must be finite and positive"
                )
            }

            Self::ArithmeticOverflow => {
                write!(
                    f,
                    "benchmark arithmetic overflow"
                )
            }

            Self::DecoderBackendNotConfigured => {
                write!(
                    f,
                    "production threshold decoder backend is not configured"
                )
            }
        }
    }
}

impl std::error::Error
    for ThresholdError
{
}

// ============================================================================
// Tests for the benchmark infrastructure
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_configuration_is_valid() {
        let config =
            ThresholdConfig::standard();

        assert_eq!(
            config.distances,
            vec![3, 5, 7, 9]
        );

        assert_eq!(
            config.error_rates,
            vec![
                0.001,
                0.005,
                0.010,
                0.020,
                0.050
            ]
        );

        assert_eq!(
            config.samples_per_point,
            10_000
        );
    }

    #[test]
    fn invalid_distance_is_rejected() {
        assert!(matches!(
            ThresholdConfig::new(
                vec![2],
                vec![0.01],
                100,
                DEFAULT_SEED,
            ),
            Err(
                ThresholdError::InvalidDistance {
                    distance: 2
                }
            )
        ));
    }

    #[test]
    fn even_distance_is_rejected() {
        assert!(matches!(
            ThresholdConfig::new(
                vec![4],
                vec![0.01],
                100,
                DEFAULT_SEED,
            ),
            Err(
                ThresholdError::EvenDistance {
                    distance: 4
                }
            )
        ));
    }

    #[test]
    fn invalid_probability_is_rejected() {
        assert!(matches!(
            ThresholdConfig::new(
                vec![3],
                vec![1.1],
                100,
                DEFAULT_SEED,
            ),
            Err(
                ThresholdError::InvalidProbability {
                    ..
                }
            )
        ));

        assert!(matches!(
            ThresholdConfig::new(
                vec![3],
                vec![f64::NAN],
                100,
                DEFAULT_SEED,
            ),
            Err(
                ThresholdError::InvalidProbability {
                    ..
                }
            )
        ));
    }

    #[test]
    fn zero_samples_are_rejected() {
        assert!(matches!(
            ThresholdConfig::new(
                vec![3],
                vec![0.01],
                0,
                DEFAULT_SEED,
            ),
            Err(
                ThresholdError::ZeroSamples
            )
        ));
    }

    #[test]
    fn excessive_workload_is_rejected() {
        assert!(matches!(
            ThresholdConfig::new(
                vec![3; MAX_DISTANCES],
                vec![0.01; MAX_ERROR_RATES],
                MAX_SAMPLES_PER_POINT,
                DEFAULT_SEED,
            ),
            Err(
                ThresholdError::WorkBudgetExceeded {
                    ..
                }
            )
        ));
    }

    #[test]
    fn deterministic_rng_is_reproducible() {
        let mut a =
            BenchmarkRng::new(
                DEFAULT_SEED,
            );

        let mut b =
            BenchmarkRng::new(
                DEFAULT_SEED,
            );

        for _ in 0..1_000 {
            assert_eq!(
                a.next_u64(),
                b.next_u64()
            );
        }
    }

    #[test]
    fn zero_seed_is_normalized() {
        let mut a =
            BenchmarkRng::new(0);

        let mut b =
            BenchmarkRng::new(
                DEFAULT_SEED,
            );

        assert_eq!(
            a.next_u64(),
            b.next_u64()
        );
    }

    #[test]
    fn bernoulli_probability_zero_never_fires() {
        let mut rng =
            BenchmarkRng::new(
                DEFAULT_SEED,
            );

        for _ in 0..10_000 {
            assert!(
                !rng.bernoulli(0.0)
            );
        }
    }

    #[test]
    fn bernoulli_probability_one_always_fires() {
        let mut rng =
            BenchmarkRng::new(
                DEFAULT_SEED,
            );

        for _ in 0..10_000 {
            assert!(
                rng.bernoulli(1.0)
            );
        }
    }

    #[test]
    fn wilson_interval_contains_observed_rate() {
        let lower_upper =
            wilson_interval(
                50,
                100,
                1.96,
            )
            .unwrap();

        assert!(
            lower_upper.0 <= 0.5
        );

        assert!(
            lower_upper.1 >= 0.5
        );
    }

    #[test]
    fn wilson_interval_rejects_invalid_counts() {
        assert!(matches!(
            wilson_interval(
                101,
                100,
                1.96,
            ),
            Err(
                ThresholdError::InvalidCounts {
                    successes: 101,
                    trials: 100,
                }
            )
        ));
    }

    #[test]
    fn threshold_point_rejects_zero_samples() {
        assert!(matches!(
            ThresholdPoint::new(
                3,
                0.01,
                0,
            ),
            Err(
                ThresholdError::ZeroSamples
            )
        ));
    }

    #[test]
    fn threshold_point_reports_rates() {
        let mut point =
            ThresholdPoint::new(
                3,
                0.01,
                1_000,
            )
            .unwrap();

        point.physical_errors = 20;
        point.logical_failures = 3;
        point.decoder_failures = 1;

        assert_eq!(
            point.physical_error_rate(),
            0.02
        );

        assert_eq!(
            point.logical_error_rate(),
            0.003
        );

        assert_eq!(
            point.decoder_failure_rate(),
            0.001
        );
    }

    #[test]
    fn report_preserves_points() {
        let point =
            ThresholdPoint::new(
                5,
                0.01,
                100,
            )
            .unwrap();

        let mut report =
            ThresholdReport::new();

        report.add(point);

        assert_eq!(
            report.points().len(),
            1
        );

        assert_eq!(
            report.points()[0],
            point
        );
    }
}