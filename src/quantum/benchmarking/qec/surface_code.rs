//! Zamani Quantum Benchmarking — Surface-Code Benchmark Model.
//!
//! Production-grade, backend-independent benchmarking of the canonical
//! rotated planar surface code already implemented by
//! `quantum::error_correction::surface_code`.
//!
//! # Responsibility
//!
//! This module owns the *benchmark-side* surface-code contract. It does not
//! reimplement the surface-code topology, stabilizer algebra, decoding, QPU
//! execution, or statistical fitting. Instead it consumes the canonical QEC
//! implementation and turns its verified topology into deterministic
//! benchmark observations.
//!
//! The benchmark covers properties that can be established without executing
//! a noisy QEC experiment:
//!
//! - code-distance policy;
//! - physical-data-qubit count;
//! - stabilizer count;
//! - logical-qubit count;
//! - stabilizer-weight distribution;
//! - X/Z stabilizer distribution;
//! - boundary/bulk topology counts;
//! - logical X/Z weights;
//! - stabilizer commutation;
//! - logical X/Z anti-commutation;
//! - topology/resource consistency;
//! - deterministic benchmark fingerprint.
//!
//! This module deliberately does NOT manufacture:
//!
//! - physical error rates;
//! - logical error rates;
//! - decoder failure rates;
//! - threshold estimates;
//! - hardware latency;
//! - syndrome-extraction fidelity.
//!
//! Those require an actual QEC experiment and belong to the QEC protocol,
//! execution, decoder, statistics, and universal benchmarking layers.
//!
//! # Architecture
//!
//! ```text
//! quantum::error_correction::surface_code
//!                  │
//!                  ▼
//!       benchmarking::qec::surface_code
//!                  │
//!        ┌─────────┼──────────┐
//!        ▼         ▼          ▼
//!     protocols  metrics   reporting
//!        │         │          │
//!        └─────────┼──────────┘
//!                  ▼
//!            BenchmarkResult
//! ```
//!
//! Dependency direction:
//!
//! ```text
//! benchmarking::qec::surface_code
//!             │
//!             └──> quantum::error_correction::surface_code
//! ```
//!
//! The canonical QEC surface-code implementation remains the sole owner of
//! surface-code topology. This module must never create a second incompatible
//! lattice representation.
//!
//! # Integration contract
//!
//! Once `benchmarking/qec/mod.rs` exists:
//!
//! ```text
//! pub mod surface_code;
//! ```
//!
//! The future QEC protocol layer should:
//!
//! 1. create `SurfaceCodeBenchmarkConfig`;
//! 2. construct `SurfaceCodeBenchmark`;
//! 3. run the structural preflight;
//! 4. use the canonical `SurfaceCode` topology for syndrome/circuit generation;
//! 5. execute the experiment;
//! 6. pass measured observations to `metrics::logical`;
//! 7. put this structural result and measured metrics into the universal
//!    `BenchmarkResult`.
//!
//! No change to the canonical QEC surface-code implementation is required by
//! this file.
//!
//! # Rust compatibility
//!
//! Rust 1.97 / 1.97.1, edition 2021, stable standard-library facilities only.

use core::fmt;
use std::collections::BTreeMap;
use std::fmt::Write as _;

use crate::quantum::error_correction::surface_code::{
    Boundary,
    SurfaceCode,
    SurfaceCodeError,
    StabilizerKind,
};

/// Stable benchmark identifier.
pub const BENCHMARK_ID: &str = "qec.surface_code";

/// Version of this benchmark result schema.
pub const SCHEMA_VERSION: u32 = 1;

/// Minimum supported rotated-planar distance.
pub const MIN_DISTANCE: usize = 3;

/// Default benchmark-side maximum distance.
///
/// The canonical QEC resource policy remains authoritative for actual
/// allocation. This bound prevents accidental unbounded benchmark requests.
pub const DEFAULT_MAX_DISTANCE: usize = 255;

/// Configuration for a deterministic surface-code structural benchmark.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceCodeBenchmarkConfig {
    distance: usize,
    max_distance: usize,
}

impl SurfaceCodeBenchmarkConfig {
    /// Creates a configuration using the default benchmark policy.
    pub fn new(distance: usize) -> Result<Self, SurfaceCodeBenchmarkError> {
        Self::with_max_distance(distance, DEFAULT_MAX_DISTANCE)
    }

    /// Creates a configuration with an explicit maximum distance.
    pub fn with_max_distance(
        distance: usize,
        max_distance: usize,
    ) -> Result<Self, SurfaceCodeBenchmarkError> {
        validate_distance(distance)?;
        validate_max_distance(max_distance)?;

        if distance > max_distance {
            return Err(SurfaceCodeBenchmarkError::DistanceExceedsPolicy {
                distance,
                maximum: max_distance,
            });
        }

        Ok(Self {
            distance,
            max_distance,
        })
    }

    /// Returns the requested code distance.
    #[must_use]
    pub const fn distance(self) -> usize {
        self.distance
    }

    /// Returns the benchmark policy maximum.
    #[must_use]
    pub const fn max_distance(self) -> usize {
        self.max_distance
    }
}

impl Default for SurfaceCodeBenchmarkConfig {
    fn default() -> Self {
        Self {
            distance: MIN_DISTANCE,
            max_distance: DEFAULT_MAX_DISTANCE,
        }
    }
}

/// Deterministic distribution of stabilizer weights.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabilizerWeightDistribution {
    counts: BTreeMap<usize, usize>,
}

impl StabilizerWeightDistribution {
    fn from_code(code: &SurfaceCode) -> Self {
        let mut counts = BTreeMap::new();

        for stabilizer in code.stabilizers() {
            let weight = stabilizer.weight();

            let entry = counts.entry(weight).or_insert(0);
            *entry += 1;
        }

        Self { counts }
    }

    /// Returns the number of stabilizers having `weight`.
    #[must_use]
    pub fn count(&self, weight: usize) -> usize {
        self.counts.get(&weight).copied().unwrap_or(0)
    }

    /// Returns deterministic ascending `(weight, count)` entries.
    #[must_use]
    pub fn entries(&self) -> Vec<(usize, usize)> {
        self.counts
            .iter()
            .map(|(&weight, &count)| (weight, count))
            .collect()
    }

    /// Returns the total number of represented stabilizers.
    #[must_use]
    pub fn total(&self) -> usize {
        self.counts.values().copied().sum()
    }
}

/// Deterministic structural benchmark result.
///
/// This is intentionally not a logical-error-rate result. A logical error
/// rate requires noisy execution, syndrome observations, and decoder output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceCodeBenchmarkResult {
    schema_version: u32,
    benchmark_id: &'static str,

    distance: usize,
    max_distance_policy: usize,

    physical_data_qubits: usize,
    stabilizers: usize,
    logical_qubits: usize,

    x_stabilizers: usize,
    z_stabilizers: usize,

    boundary_stabilizers: usize,
    bulk_stabilizers: usize,

    weight_distribution: StabilizerWeightDistribution,

    logical_x_weight: usize,
    logical_z_weight: usize,

    topology_valid: bool,
    stabilizers_commute: bool,
    logical_operators_valid: bool,
    resource_formula_valid: bool,
    logical_distance_matches_requested: bool,

    fingerprint: String,
}

impl SurfaceCodeBenchmarkResult {
    /// Returns the result schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the stable benchmark identifier.
    #[must_use]
    pub const fn benchmark_id(&self) -> &'static str {
        self.benchmark_id
    }

    /// Returns the code distance.
    #[must_use]
    pub const fn distance(&self) -> usize {
        self.distance
    }

    /// Returns the configured benchmark maximum.
    #[must_use]
    pub const fn max_distance_policy(&self) -> usize {
        self.max_distance_policy
    }

    /// Returns the number of physical data qubits.
    #[must_use]
    pub const fn physical_data_qubits(&self) -> usize {
        self.physical_data_qubits
    }

    /// Returns the number of stabilizers.
    #[must_use]
    pub const fn stabilizers(&self) -> usize {
        self.stabilizers
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub const fn logical_qubits(&self) -> usize {
        self.logical_qubits
    }

    /// Returns the number of X stabilizers.
    #[must_use]
    pub const fn x_stabilizers(&self) -> usize {
        self.x_stabilizers
    }

    /// Returns the number of Z stabilizers.
    #[must_use]
    pub const fn z_stabilizers(&self) -> usize {
        self.z_stabilizers
    }

    /// Returns the number of boundary stabilizers.
    #[must_use]
    pub const fn boundary_stabilizers(&self) -> usize {
        self.boundary_stabilizers
    }

    /// Returns the number of bulk stabilizers.
    #[must_use]
    pub const fn bulk_stabilizers(&self) -> usize {
        self.bulk_stabilizers
    }

    /// Returns the stabilizer-weight distribution.
    #[must_use]
    pub fn weight_distribution(&self) -> &StabilizerWeightDistribution {
        &self.weight_distribution
    }

    /// Returns the logical-X operator weight.
    #[must_use]
    pub const fn logical_x_weight(&self) -> usize {
        self.logical_x_weight
    }

    /// Returns the logical-Z operator weight.
    #[must_use]
    pub const fn logical_z_weight(&self) -> usize {
        self.logical_z_weight
    }

    /// Returns whether canonical topology validation passed.
    #[must_use]
    pub const fn topology_valid(&self) -> bool {
        self.topology_valid
    }

    /// Returns whether all stabilizers commute.
    #[must_use]
    pub const fn stabilizers_commute(&self) -> bool {
        self.stabilizers_commute
    }

    /// Returns whether the logical X/Z invariants passed.
    #[must_use]
    pub const fn logical_operators_valid(&self) -> bool {
        self.logical_operators_valid
    }

    /// Returns whether canonical resource formulas were satisfied.
    #[must_use]
    pub const fn resource_formula_valid(&self) -> bool {
        self.resource_formula_valid
    }

    /// Returns whether both logical strings have weight `d`.
    #[must_use]
    pub const fn logical_distance_matches_requested(&self) -> bool {
        self.logical_distance_matches_requested
    }

    /// Returns the deterministic topology fingerprint.
    ///
    /// This fingerprint is for deterministic identity/grouping only. It is
    /// not a cryptographic authenticity mechanism.
    #[must_use]
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }

    /// Returns true only when all structural invariants passed.
    #[must_use]
    pub const fn passed(&self) -> bool {
        self.topology_valid
            && self.stabilizers_commute
            && self.logical_operators_valid
            && self.resource_formula_valid
            && self.logical_distance_matches_requested
    }
}

/// Errors produced by the benchmark-side surface-code layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceCodeBenchmarkError {
    /// Requested distance is not a valid rotated-planar distance.
    InvalidDistance {
        distance: usize,
    },

    /// Requested distance exceeds benchmark policy.
    DistanceExceedsPolicy {
        distance: usize,
        maximum: usize,
    },

    /// Configured maximum itself is invalid.
    InvalidMaximumDistance {
        maximum: usize,
    },

    /// Canonical QEC construction or validation failed.
    CanonicalQec {
        message: String,
    },

    /// Canonical QEC model violated the benchmark contract.
    InvariantViolation {
        message: String,
    },

    /// Deterministic fingerprint construction failed.
    FingerprintFailure,
}

impl fmt::Display for SurfaceCodeBenchmarkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDistance { distance } => write!(
                formatter,
                "invalid surface-code benchmark distance {distance}; \
                 distance must be odd and >= 3"
            ),

            Self::DistanceExceedsPolicy {
                distance,
                maximum,
            } => write!(
                formatter,
                "surface-code benchmark distance {distance} exceeds \
                 configured maximum {maximum}"
            ),

            Self::InvalidMaximumDistance { maximum } => write!(
                formatter,
                "invalid surface-code benchmark maximum distance {maximum}; \
                 maximum must be odd and >= 3"
            ),

            Self::CanonicalQec { message } => write!(
                formatter,
                "canonical QEC surface-code failure: {message}"
            ),

            Self::InvariantViolation { message } => write!(
                formatter,
                "surface-code benchmark invariant violation: {message}"
            ),

            Self::FingerprintFailure => write!(
                formatter,
                "surface-code benchmark fingerprint construction failed"
            ),
        }
    }
}

impl std::error::Error for SurfaceCodeBenchmarkError {}

impl From<SurfaceCodeError> for SurfaceCodeBenchmarkError {
    fn from(error: SurfaceCodeError) -> Self {
        Self::CanonicalQec {
            message: error.to_string(),
        }
    }
}

/// Deterministic structural surface-code benchmark.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceCodeBenchmark {
    config: SurfaceCodeBenchmarkConfig,
}

impl SurfaceCodeBenchmark {
    /// Creates a validated benchmark.
    pub fn new(
        config: SurfaceCodeBenchmarkConfig,
    ) -> Result<Self, SurfaceCodeBenchmarkError> {
        validate_distance(config.distance)?;
        validate_max_distance(config.max_distance)?;

        if config.distance > config.max_distance {
            return Err(SurfaceCodeBenchmarkError::DistanceExceedsPolicy {
                distance: config.distance,
                maximum: config.max_distance,
            });
        }

        Ok(Self { config })
    }

    /// Returns the immutable configuration.
    #[must_use]
    pub const fn config(self) -> SurfaceCodeBenchmarkConfig {
        self.config
    }

    /// Executes the structural benchmark.
    ///
    /// No hardware, simulator, RNG, filesystem, network, or clock is used.
    pub fn run(self) -> Result<SurfaceCodeBenchmarkResult, SurfaceCodeBenchmarkError> {
        let code = SurfaceCode::new(self.config.distance)?;

        // Canonical validation remains authoritative, but the benchmark
        // explicitly invokes the relevant invariants so that its acceptance
        // contract is visible and testable.
        code.validate_logical_operators()?;
        code.validate()?;

        let physical_data_qubits = code.num_data_qubits();
        let stabilizers = code.num_stabilizers();
        let logical_qubits = code.num_logical_qubits();

        let expected_qubits = checked_square(self.config.distance)?;

        let expected_stabilizers = expected_qubits
            .checked_sub(1)
            .ok_or_else(|| SurfaceCodeBenchmarkError::InvariantViolation {
                message: "distance squared must be at least one".to_string(),
            })?;

        let resource_formula_valid =
            physical_data_qubits == expected_qubits
                && stabilizers == expected_stabilizers
                && logical_qubits == 1;

        if !resource_formula_valid {
            return Err(SurfaceCodeBenchmarkError::InvariantViolation {
                message: format!(
                    "expected d²={} data qubits, d²-1={} stabilizers, \
                     and 1 logical qubit; observed {}, {}, {}",
                    expected_qubits,
                    expected_stabilizers,
                    physical_data_qubits,
                    stabilizers,
                    logical_qubits
                ),
            });
        }

        let weight_distribution =
            StabilizerWeightDistribution::from_code(&code);

        if weight_distribution.total() != stabilizers {
            return Err(SurfaceCodeBenchmarkError::InvariantViolation {
                message:
                    "stabilizer weight distribution does not account for \
                     every stabilizer"
                        .to_string(),
            });
        }

        let (
            x_stabilizers,
            z_stabilizers,
            boundary_stabilizers,
            bulk_stabilizers,
        ) = classify_stabilizers(&code);

        if x_stabilizers
            .checked_add(z_stabilizers)
            != Some(stabilizers)
        {
            return Err(SurfaceCodeBenchmarkError::InvariantViolation {
                message:
                    "X/Z stabilizer partition does not account for every \
                     stabilizer"
                        .to_string(),
            });
        }

        if boundary_stabilizers
            .checked_add(bulk_stabilizers)
            != Some(stabilizers)
        {
            return Err(SurfaceCodeBenchmarkError::InvariantViolation {
                message:
                    "boundary/bulk stabilizer partition does not account \
                     for every stabilizer"
                        .to_string(),
            });
        }

        let stabilizers_commute = stabilizers_commute(&code)?;

        if !stabilizers_commute {
            return Err(SurfaceCodeBenchmarkError::InvariantViolation {
                message:
                    "canonical surface-code stabilizers do not mutually \
                     commute"
                        .to_string(),
            });
        }

        let logical_x_weight = code.logical_x().weight();
        let logical_z_weight = code.logical_z().weight();

        let logical_distance_matches_requested =
            logical_x_weight == self.config.distance
                && logical_z_weight == self.config.distance;

        if !logical_distance_matches_requested {
            return Err(SurfaceCodeBenchmarkError::InvariantViolation {
                message: format!(
                    "logical X/Z weights are {logical_x_weight}/{logical_z_weight}; \
                     expected both to equal d={}",
                    self.config.distance
                ),
            });
        }

        let logical_operators_valid =
            validate_logical_pair(&code)?;

        if !logical_operators_valid {
            return Err(SurfaceCodeBenchmarkError::InvariantViolation {
                message:
                    "canonical logical X/Z operators failed logical \
                     commutation invariants"
                        .to_string(),
            });
        }

        let fingerprint =
            fingerprint(&code, self.config.max_distance)
                .ok_or(SurfaceCodeBenchmarkError::FingerprintFailure)?;

        Ok(SurfaceCodeBenchmarkResult {
            schema_version: SCHEMA_VERSION,
            benchmark_id: BENCHMARK_ID,

            distance: self.config.distance,
            max_distance_policy: self.config.max_distance,

            physical_data_qubits,
            stabilizers,
            logical_qubits,

            x_stabilizers,
            z_stabilizers,

            boundary_stabilizers,
            bulk_stabilizers,

            weight_distribution,

            logical_x_weight,
            logical_z_weight,

            topology_valid: true,
            stabilizers_commute: true,
            logical_operators_valid,
            resource_formula_valid,
            logical_distance_matches_requested,

            fingerprint,
        })
    }
}

fn validate_distance(
    distance: usize,
) -> Result<(), SurfaceCodeBenchmarkError> {
    if distance < MIN_DISTANCE || distance % 2 == 0 {
        return Err(
            SurfaceCodeBenchmarkError::InvalidDistance {
                distance,
            },
        );
    }

    Ok(())
}

fn validate_max_distance(
    maximum: usize,
) -> Result<(), SurfaceCodeBenchmarkError> {
    if maximum < MIN_DISTANCE || maximum % 2 == 0 {
        return Err(
            SurfaceCodeBenchmarkError::InvalidMaximumDistance {
                maximum,
            },
        );
    }

    Ok(())
}

fn checked_square(
    value: usize,
) -> Result<usize, SurfaceCodeBenchmarkError> {
    value
        .checked_mul(value)
        .ok_or_else(|| {
            SurfaceCodeBenchmarkError::InvariantViolation {
                message:
                    "surface-code distance squared overflowed usize"
                        .to_string(),
            }
        })
}

fn classify_stabilizers(
    code: &SurfaceCode,
) -> (usize, usize, usize, usize) {
    let mut x_stabilizers = 0usize;
    let mut z_stabilizers = 0usize;
    let mut boundary_stabilizers = 0usize;

    for stabilizer in code.stabilizers() {
        match stabilizer.kind() {
            StabilizerKind::X => {
                x_stabilizers += 1;
            }

            StabilizerKind::Z => {
                z_stabilizers += 1;
            }
        }

        if stabilizer.boundary().is_some() {
            boundary_stabilizers += 1;
        }
    }

    let bulk_stabilizers = code
        .num_stabilizers()
        .saturating_sub(boundary_stabilizers);

    (
        x_stabilizers,
        z_stabilizers,
        boundary_stabilizers,
        bulk_stabilizers,
    )
}

fn stabilizers_commute(
    code: &SurfaceCode,
) -> Result<bool, SurfaceCodeBenchmarkError> {
    let stabilizers = code.stabilizers();

    for first_index in 0..stabilizers.len() {
        let first = &stabilizers[first_index];

        let first_operator =
            first.pauli_string(code.num_data_qubits())?;

        for second_index in
            (first_index + 1)..stabilizers.len()
        {
            let second = &stabilizers[second_index];

            let second_operator =
                second.pauli_string(code.num_data_qubits())?;

            if !first_operator
                .commutes_with(&second_operator)?
            {
                return Ok(false);
            }
        }
    }

    Ok(true)
}

fn validate_logical_pair(
    code: &SurfaceCode,
) -> Result<bool, SurfaceCodeBenchmarkError> {
    let logical_x = code.logical_x().operator();
    let logical_z = code.logical_z().operator();

    if logical_x.is_identity()
        || logical_z.is_identity()
    {
        return Ok(false);
    }

    if !logical_x.anticommutes_with(logical_z)? {
        return Ok(false);
    }

    for stabilizer in code.stabilizers() {
        let operator =
            stabilizer.pauli_string(code.num_data_qubits())?;

        if !logical_x.commutes_with(&operator)? {
            return Ok(false);
        }

        if !logical_z.commutes_with(&operator)? {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Builds a deterministic, non-cryptographic topology fingerprint.
///
/// This is deliberately local rather than depending on a hashing crate.
/// The result is suitable for benchmark identity/grouping, but NOT for
/// authenticity, signatures, or security-sensitive integrity verification.
fn fingerprint(
    code: &SurfaceCode,
    max_distance: usize,
) -> Option<String> {
    let mut hash = 0xcbf29ce484222325u64;

    fn feed(
        hash: &mut u64,
        byte: u8,
    ) {
        *hash ^= u64::from(byte);
        *hash = hash.wrapping_mul(0x100000001b3);
    }

    fn feed_usize(
        hash: &mut u64,
        value: usize,
    ) {
        for byte in value.to_le_bytes() {
            feed(hash, byte);
        }
    }

    fn feed_str(
        hash: &mut u64,
        value: &str,
    ) {
        for byte in value.as_bytes() {
            feed(hash, *byte);
        }

        feed(hash, 0xff);
    }

    feed_str(&mut hash, BENCHMARK_ID);
    feed_usize(&mut hash, SCHEMA_VERSION as usize);
    feed_usize(&mut hash, code.distance());
    feed_usize(&mut hash, max_distance);

    for qubit in code.data_qubits() {
        feed_usize(
            &mut hash,
            qubit.index().index(),
        );

        feed_usize(
            &mut hash,
            qubit.coordinate().row(),
        );

        feed_usize(
            &mut hash,
            qubit.coordinate().column(),
        );
    }

    for stabilizer in code.stabilizers() {
        feed_usize(
            &mut hash,
            stabilizer.id(),
        );

        let kind = stabilizer.kind().to_string();
        feed_str(&mut hash, &kind);

        match stabilizer.boundary() {
            Some(boundary) => {
                let boundary_string =
                    boundary.to_string();

                feed_str(
                    &mut hash,
                    &boundary_string,
                );
            }

            None => {
                feed_str(&mut hash, "bulk");
            }
        }

        for qubit in stabilizer.support() {
            feed_usize(
                &mut hash,
                qubit.index(),
            );
        }
    }

    feed_str(
        &mut hash,
        &code.logical_x()
            .operator()
            .to_string(),
    );

    feed_str(
        &mut hash,
        &code.logical_z()
            .operator()
            .to_string(),
    );

    let mut result = String::new();

    write!(
        &mut result,
        "{hash:016x}"
    )
    .ok()?;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_even_distance() {
        let result =
            SurfaceCodeBenchmarkConfig::new(4);

        assert!(matches!(
            result,
            Err(
                SurfaceCodeBenchmarkError::InvalidDistance {
                    distance: 4
                }
            )
        ));
    }

    #[test]
    fn rejects_distance_below_three() {
        let result =
            SurfaceCodeBenchmarkConfig::new(1);

        assert!(matches!(
            result,
            Err(
                SurfaceCodeBenchmarkError::InvalidDistance {
                    distance: 1
                }
            )
        ));
    }

    #[test]
    fn rejects_distance_above_policy() {
        let result =
            SurfaceCodeBenchmarkConfig::with_max_distance(
                7,
                5,
            );

        assert!(matches!(
            result,
            Err(
                SurfaceCodeBenchmarkError::DistanceExceedsPolicy {
                    distance: 7,
                    maximum: 5
                }
            )
        ));
    }

    #[test]
    fn rejects_even_maximum_distance() {
        let result =
            SurfaceCodeBenchmarkConfig::with_max_distance(
                3,
                4,
            );

        assert!(matches!(
            result,
            Err(
                SurfaceCodeBenchmarkError::InvalidMaximumDistance {
                    maximum: 4
                }
            )
        ));
    }

    #[test]
    fn distance_three_has_canonical_resources() {
        let config =
            SurfaceCodeBenchmarkConfig::new(3)
                .expect("distance 3 is valid");

        let benchmark =
            SurfaceCodeBenchmark::new(config)
                .expect("benchmark configuration is valid");

        let result =
            benchmark.run()
                .expect("canonical distance-3 code");

        assert!(result.passed());

        assert_eq!(
            result.physical_data_qubits(),
            9
        );

        assert_eq!(
            result.stabilizers(),
            8
        );

        assert_eq!(
            result.logical_qubits(),
            1
        );

        assert_eq!(
            result.logical_x_weight(),
            3
        );

        assert_eq!(
            result.logical_z_weight(),
            3
        );

        assert_eq!(
            result.weight_distribution().total(),
            8
        );

        assert!(!result.fingerprint().is_empty());
    }

    #[test]
    fn distance_five_has_canonical_resources() {
        let config =
            SurfaceCodeBenchmarkConfig::new(5)
                .expect("distance 5 is valid");

        let benchmark =
            SurfaceCodeBenchmark::new(config)
                .expect("benchmark configuration is valid");

        let result =
            benchmark.run()
                .expect("canonical distance-5 code");

        assert!(result.passed());

        assert_eq!(
            result.physical_data_qubits(),
            25
        );

        assert_eq!(
            result.stabilizers(),
            24
        );

        assert_eq!(
            result.logical_qubits(),
            1
        );

        assert_eq!(
            result.logical_x_weight(),
            5
        );

        assert_eq!(
            result.logical_z_weight(),
            5
        );

        assert_eq!(
            result.weight_distribution().total(),
            24
        );
    }

    #[test]
    fn benchmark_is_deterministic() {
        let config =
            SurfaceCodeBenchmarkConfig::new(7)
                .expect("distance 7 is valid");

        let benchmark =
            SurfaceCodeBenchmark::new(config)
                .expect("benchmark configuration is valid");

        let first =
            benchmark.run()
                .expect("first run");

        let second =
            benchmark.run()
                .expect("second run");

        assert_eq!(first, second);
        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn stabilizer_distribution_accounts_for_all_generators() {
        let config =
            SurfaceCodeBenchmarkConfig::new(3)
                .expect("distance 3 is valid");

        let benchmark =
            SurfaceCodeBenchmark::new(config)
                .expect("benchmark configuration is valid");

        let result =
            benchmark.run()
                .expect("canonical code");

        let entries =
            result.weight_distribution().entries();

        let total: usize =
            entries
                .iter()
                .map(|(_, count)| *count)
                .sum();

        assert_eq!(
            total,
            result.stabilizers()
        );

        assert_eq!(
            result.weight_distribution().count(0),
            0
        );
    }

    #[test]
    fn boundary_and_bulk_partition_is_complete() {
        let config =
            SurfaceCodeBenchmarkConfig::new(5)
                .expect("distance 5 is valid");

        let benchmark =
            SurfaceCodeBenchmark::new(config)
                .expect("benchmark configuration is valid");

        let result =
            benchmark.run()
                .expect("canonical code");

        assert!(
            result.boundary_stabilizers()
                <= result.stabilizers()
        );

        assert!(
            result.bulk_stabilizers()
                <= result.stabilizers()
        );

        assert_eq!(
            result.boundary_stabilizers()
                + result.bulk_stabilizers(),
            result.stabilizers()
        );
    }

    #[test]
    fn x_and_z_partition_is_complete() {
        let config =
            SurfaceCodeBenchmarkConfig::new(5)
                .expect("distance 5 is valid");

        let benchmark =
            SurfaceCodeBenchmark::new(config)
                .expect("benchmark configuration is valid");

        let result =
            benchmark.run()
                .expect("canonical code");

        assert_eq!(
            result.x_stabilizers()
                + result.z_stabilizers(),
            result.stabilizers()
        );
    }

    #[test]
    fn benchmark_constants_are_sane() {
        assert_eq!(
            BENCHMARK_ID,
            "qec.surface_code"
        );

        assert_eq!(
            MIN_DISTANCE,
            3
        );

        assert_eq!(
            DEFAULT_MAX_DISTANCE % 2,
            1
        );

        assert!(
            DEFAULT_MAX_DISTANCE
                >= MIN_DISTANCE
        );
    }

    #[test]
    fn all_boundary_variants_remain_available() {
        let boundaries = [
            Boundary::Top,
            Boundary::Bottom,
            Boundary::Left,
            Boundary::Right,
        ];

        assert_eq!(
            boundaries.len(),
            4
        );
    }
}