//! Zamani Quantum Benchmarking — Volumetric Performance Frontier
//!
//! # Purpose
//!
//! This module computes the production volumetric performance frontier from an
//! already aggregated set of benchmark measurements.
//!
//! It deliberately does NOT:
//!
//! - generate quantum circuits;
//! - execute circuits;
//! - communicate with quantum hardware;
//! - select a backend;
//! - perform compilation;
//! - perform routing;
//! - perform scheduling;
//! - depend on Quantum IR;
//! - depend on a simulator;
//! - depend on a hardware provider;
//! - perform statistical fitting;
//! - silently discard measurements;
//! - maintain process-global state;
//! - print diagnostics.
//!
//! The intended dependency direction is:
//!
//! ```text
//! benchmark protocol
//!       │
//!       ▼
//! volumetric measurements
//!       │
//!       ▼
//! volumetric::frontier
//!       │
//!       ├── passing boundary
//!       ├── Pareto frontier
//!       └── maximum supported point
//! ```
//!
//! # What is a volumetric frontier?
//!
//! A volumetric benchmark evaluates quantum-system performance across at least
//! two resource dimensions, normally:
//!
//! - circuit width / number of qubits;
//! - circuit depth.
//!
//! Each `(width, depth)` point has a measured quality value.
//!
//! A frontier identifies the boundary of the region that satisfies a required
//! quality criterion.
//!
//! For the default `HigherIsBetter` policy:
//!
//! ```text
//! quality >= required_quality
//! ```
//!
//! is considered a passing point.
//!
//! The frontier is not the same thing as the complete performance surface.
//! `surface.rs` is expected to retain the complete grid/surface, while this
//! module reduces that already validated surface to its useful boundary.
//!
//! # Important statistical rule
//!
//! This module does not calculate confidence intervals.
//!
//! A higher-level statistical layer should calculate conservative bounds first
//! and pass the appropriate value here. For example, if a benchmark requires:
//!
//! ```text
//! lower_confidence_bound >= required_quality
//! ```
//!
//! then the caller should pass the lower confidence bound as `quality`.
//!
//! This keeps statistical methodology separate from geometric frontier
//! extraction.
//!
//! # Frontier semantics
//!
//! The module produces two complementary views:
//!
//! 1. `width_frontier`
//!
//!    For every tested width, it records the deepest passing point.
//!
//! 2. `pareto_frontier`
//!
//!    It records non-dominated passing points across width and depth.
//!
//! A point `(w1, d1)` dominates `(w2, d2)` when:
//!
//! ```text
//! w1 >= w2
//! d1 >= d2
//! ```
//!
//! with at least one strict inequality.
//!
//! The quality requirement has already been applied before dominance is
//! evaluated. This prevents a very high-quality shallow point from incorrectly
//! hiding a deeper point that also satisfies the required quality.
//!
//! # Determinism
//!
//! Frontier extraction is deterministic.
//!
//! Measurements are sorted by:
//!
//! 1. width ascending;
//! 2. depth ascending;
//! 3. quality according to the configured objective;
//!
//! Output ordering is therefore stable and suitable for:
//!
//! - reproducibility;
//! - golden tests;
//! - CI regression tests;
//! - JSON/CSV serialization;
//! - benchmark comparison;
//! - result hashing.
//!
//! # Duplicate measurements
//!
//! This module intentionally rejects duplicate `(width, depth)` coordinates.
//!
//! Repeated hardware observations must be aggregated by the statistical layer
//! before frontier extraction. Silently choosing one duplicate measurement
//! would make the benchmark result dependent on input ordering and could hide
//! experimental problems.
//!
//! # Overflow and resource safety
//!
//! The module does not allocate based on a benchmark-declared maximum width or
//! depth. It only allocates in proportion to the number of supplied validated
//! measurements.
//!
//! This prevents a malformed configuration such as `width = usize::MAX` from
//! causing a giant allocation.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This file is intentionally independent of the rest of the future
//! benchmarking tree.
//!
//! Future integration should be:
//!
//! ```text
//! volumetric::surface
//!        │
//!        ▼
//! VolumetricMeasurement
//!        │
//!        ▼
//! volumetric::frontier
//!        │
//!        ▼
//! FrontierResult
//!        │
//!        ├── reporting
//!        ├── analysis
//!        └── core::result
//! ```
//!
//! The QV protocol may also use it:
//!
//! ```text
//! generators::qv
//!      │
//!      ▼
//! protocols::quantum_volume
//!      │
//!      ▼
//! volume_estimator
//!      │
//!      ▼
//! volumetric::frontier
//! ```
//!
//! `volume_estimator.rs` remains responsible for QV-specific statistical
//! mathematics. This file is responsible only for extracting the volumetric
//! boundary.
//!
//! # Public API stability
//!
//! The following types form the stable API:
//!
//! - `FrontierError`
//! - `FrontierObjective`
//! - `VolumetricMeasurement`
//! - `FrontierPolicy`
//! - `FrontierPoint`
//! - `FrontierResult`
//! - `compute_frontier()`
//!
//! Future benchmarking modules should consume these types rather than
//! duplicating frontier logic.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

// ============================================================================
// Public constants
// ============================================================================

/// Stable benchmark-analysis identifier for this module.
pub const VOLUMETRIC_FRONTIER_ID: &str = "volumetric_frontier";

/// Version of the serialized frontier-analysis contract.
///
/// This is independent of the Zamani compiler version.
pub const VOLUMETRIC_FRONTIER_SCHEMA_VERSION: u32 = 1;

/// Small numerical tolerance used only when validating a value that should
/// lie within the closed unit interval.
///
/// Frontier extraction itself never uses this tolerance to alter measurements.
const UNIT_INTERVAL_EPSILON: f64 = 1.0e-12;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced while constructing a volumetric performance frontier.
#[derive(Debug, Clone, PartialEq)]
pub enum FrontierError {
    /// No measurements were supplied.
    EmptyMeasurements,

    /// Width is zero.
    InvalidWidth {
        index: usize,
    },

    /// Depth is zero.
    InvalidDepth {
        index: usize,
    },

    /// Quality is NaN or infinite.
    NonFiniteQuality {
        index: usize,
        value: f64,
    },

    /// The required quality threshold is NaN or infinite.
    NonFiniteThreshold {
        value: f64,
    },

    /// A unit-interval quality threshold was outside [0, 1].
    InvalidUnitIntervalThreshold {
        value: f64,
    },

    /// The same `(width, depth)` coordinate occurred more than once.
    DuplicateCoordinate {
        width: usize,
        depth: usize,
    },

    /// A caller supplied a measurement that violates the selected objective's
    /// finite-value requirements.
    InvalidMeasurement {
        index: usize,
    },
}

impl fmt::Display for FrontierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMeasurements => {
                write!(formatter, "volumetric frontier requires at least one measurement")
            }

            Self::InvalidWidth { index } => {
                write!(
                    formatter,
                    "volumetric measurement {} has width zero",
                    index
                )
            }

            Self::InvalidDepth { index } => {
                write!(
                    formatter,
                    "volumetric measurement {} has depth zero",
                    index
                )
            }

            Self::NonFiniteQuality { index, value } => {
                write!(
                    formatter,
                    "volumetric measurement {} has non-finite quality {}",
                    index,
                    value
                )
            }

            Self::NonFiniteThreshold { value } => {
                write!(
                    formatter,
                    "volumetric frontier threshold must be finite, got {}",
                    value
                )
            }

            Self::InvalidUnitIntervalThreshold { value } => {
                write!(
                    formatter,
                    "volumetric frontier threshold must be in [0, 1], got {}",
                    value
                )
            }

            Self::DuplicateCoordinate { width, depth } => {
                write!(
                    formatter,
                    "duplicate volumetric measurement coordinate ({}, {})",
                    width,
                    depth
                )
            }

            Self::InvalidMeasurement { index } => {
                write!(
                    formatter,
                    "volumetric measurement {} is invalid for the selected frontier objective",
                    index
                )
            }
        }
    }
}

impl Error for FrontierError {}

// ============================================================================
// Frontier objective
// ============================================================================

/// Direction in which a quality metric is considered better.
///
/// The default for fidelity/success-probability style metrics is
/// `HigherIsBetter`.
///
/// `LowerIsBetter` allows the same geometric frontier engine to be used for
/// metrics such as error rate, latency, or resource cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontierObjective {
    /// Larger values are better.
    HigherIsBetter,

    /// Smaller values are better.
    LowerIsBetter,
}

impl FrontierObjective {
    /// Returns whether a measured value satisfies the supplied threshold.
    #[inline]
    pub fn passes(self, value: f64, threshold: f64) -> bool {
        match self {
            Self::HigherIsBetter => value >= threshold,
            Self::LowerIsBetter => value <= threshold,
        }
    }

    /// Returns whether `candidate` is better than `current`.
    #[inline]
    pub fn is_better(self, candidate: f64, current: f64) -> bool {
        match self {
            Self::HigherIsBetter => candidate > current,
            Self::LowerIsBetter => candidate < current,
        }
    }

    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HigherIsBetter => "higher_is_better",
            Self::LowerIsBetter => "lower_is_better",
        }
    }
}

// ============================================================================
// Volumetric measurement
// ============================================================================

/// One already-aggregated point on a volumetric benchmark surface.
///
/// A measurement represents exactly one `(width, depth)` coordinate.
///
/// Repeated shots, circuits, bootstrap samples, calibration runs, or hardware
/// repetitions must be aggregated by the caller before constructing this
/// value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VolumetricMeasurement {
    /// Number of qubits/resources represented by the horizontal dimension.
    pub width: usize,

    /// Circuit depth or equivalent vertical resource dimension.
    pub depth: usize,

    /// Quality value associated with this point.
    ///
    /// For a fidelity/success-probability benchmark this is normally in [0, 1].
    /// Frontier extraction itself does not force the value into [0, 1], because
    /// the engine also supports metrics such as throughput and latency.
    pub quality: f64,
}

impl VolumetricMeasurement {
    /// Creates a validated measurement.
    pub fn new(
        width: usize,
        depth: usize,
        quality: f64,
    ) -> Result<Self, FrontierError> {
        let measurement = Self {
            width,
            depth,
            quality,
        };

        measurement.validate(0)?;

        Ok(measurement)
    }

    /// Validates the measurement.
    pub fn validate(&self, index: usize) -> Result<(), FrontierError> {
        if self.width == 0 {
            return Err(FrontierError::InvalidWidth { index });
        }

        if self.depth == 0 {
            return Err(FrontierError::InvalidDepth { index });
        }

        if !self.quality.is_finite() {
            return Err(FrontierError::NonFiniteQuality {
                index,
                value: self.quality,
            });
        }

        Ok(())
    }

    /// Returns the coordinate `(width, depth)`.
    #[inline]
    pub const fn coordinate(&self) -> (usize, usize) {
        (self.width, self.depth)
    }
}

// ============================================================================
// Frontier policy
// ============================================================================

/// Configuration controlling frontier extraction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrontierPolicy {
    /// Determines whether higher or lower quality is better.
    pub objective: FrontierObjective,

    /// Minimum acceptable quality for `HigherIsBetter`, or maximum acceptable
    /// quality for `LowerIsBetter`.
    pub required_quality: f64,

    /// Whether the caller asserts that `required_quality` is a probability-like
    /// unit in [0, 1].
    ///
    /// This is useful for validation without forcing all volumetric metrics to
    /// be probabilities.
    pub unit_interval: bool,
}

impl FrontierPolicy {
    /// Creates the standard quality policy:
    ///
    /// ```text
    /// quality >= required_quality
    /// ```
    ///
    /// The threshold is validated as a probability in [0, 1].
    pub fn quality(required_quality: f64) -> Result<Self, FrontierError> {
        Self {
            objective: FrontierObjective::HigherIsBetter,
            required_quality,
            unit_interval: true,
        }
        .validate()
    }

    /// Creates a generic higher-is-better policy.
    pub fn higher_is_better(required_quality: f64) -> Result<Self, FrontierError> {
        Self {
            objective: FrontierObjective::HigherIsBetter,
            required_quality,
            unit_interval: false,
        }
        .validate()
    }

    /// Creates a generic lower-is-better policy.
    pub fn lower_is_better(required_quality: f64) -> Result<Self, FrontierError> {
        Self {
            objective: FrontierObjective::LowerIsBetter,
            required_quality,
            unit_interval: false,
        }
        .validate()
    }

    /// Validates the policy.
    pub fn validate(&self) -> Result<(), FrontierError> {
        if !self.required_quality.is_finite() {
            return Err(FrontierError::NonFiniteThreshold {
                value: self.required_quality,
            });
        }

        if self.unit_interval
            && (self.required_quality < -UNIT_INTERVAL_EPSILON
                || self.required_quality > 1.0 + UNIT_INTERVAL_EPSILON)
        {
            return Err(FrontierError::InvalidUnitIntervalThreshold {
                value: self.required_quality,
            });
        }

        Ok(())
    }

    /// Returns whether a measurement passes the required quality criterion.
    #[inline]
    pub fn passes(&self, quality: f64) -> bool {
        self.objective
            .passes(quality, self.required_quality)
    }
}

// ============================================================================
// Frontier point
// ============================================================================

/// A point belonging to the extracted volumetric frontier.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrontierPoint {
    /// Width/resource size.
    pub width: usize,

    /// Maximum passing depth represented by this frontier point.
    pub depth: usize,

    /// Measured quality at this exact coordinate.
    pub quality: f64,
}

impl FrontierPoint {
    /// Returns the coordinate.
    #[inline]
    pub const fn coordinate(&self) -> (usize, usize) {
        (self.width, self.depth)
    }
}

// ============================================================================
// Frontier result
// ============================================================================

/// Complete result of volumetric frontier extraction.
///
/// The result deliberately retains both the width frontier and Pareto frontier.
///
/// This avoids forcing downstream reporting code to reconstruct one from the
/// other.
#[derive(Debug, Clone, PartialEq)]
pub struct FrontierResult {
    /// Stable analysis identifier.
    pub analysis_id: &'static str,

    /// Schema version.
    pub schema_version: u32,

    /// Frontier policy used for classification.
    pub policy: FrontierPolicy,

    /// Number of unique input measurements.
    pub measurement_count: usize,

    /// Number of measurements satisfying the quality requirement.
    pub passing_measurement_count: usize,

    /// Number of distinct widths represented by the input.
    pub width_count: usize,

    /// Number of distinct depths represented by the input.
    pub depth_count: usize,

    /// Largest width represented by a passing measurement.
    ///
    /// `None` means no measurement passed.
    pub maximum_passing_width: Option<usize>,

    /// Largest depth represented by a passing measurement.
    ///
    /// `None` means no measurement passed.
    pub maximum_passing_depth: Option<usize>,

    /// Largest square dimension for which both width and depth are at least
    /// the same value and the point itself passes.
    ///
    /// This is useful for relating a volumetric frontier back to Quantum
    /// Volume-style square dimensions without making this module QV-specific.
    pub maximum_passing_square_dimension: Option<usize>,

    /// For each tested width, the deepest passing point.
    ///
    /// Sorted by ascending width.
    pub width_frontier: Vec<FrontierPoint>,

    /// Non-dominated passing points.
    ///
    /// Sorted by ascending width and then ascending depth.
    pub pareto_frontier: Vec<FrontierPoint>,
}

impl FrontierResult {
    /// Returns true when at least one measurement satisfies the policy.
    #[inline]
    pub const fn has_passing_point(&self) -> bool {
        self.passing_measurement_count != 0
    }

    /// Returns the number of points on the Pareto frontier.
    #[inline]
    pub fn pareto_point_count(&self) -> usize {
        self.pareto_frontier.len()
    }

    /// Returns the deepest passing point for a specific width.
    pub fn deepest_passing_at_width(
        &self,
        width: usize,
    ) -> Option<FrontierPoint> {
        self.width_frontier
            .binary_search_by_key(&width, |point| point.width)
            .ok()
            .map(|index| self.width_frontier[index])
    }

    /// Returns the deepest passing point among all widths.
    pub fn deepest_passing_point(&self) -> Option<FrontierPoint> {
        self.width_frontier
            .iter()
            .copied()
            .max_by_key(|point| (point.depth, point.width))
    }

    /// Returns the widest passing point, breaking ties by depth.
    pub fn widest_passing_point(&self) -> Option<FrontierPoint> {
        self.width_frontier
            .iter()
            .copied()
            .max_by_key(|point| (point.width, point.depth))
    }

    /// Returns the largest square dimension represented by the result.
    ///
    /// This is equivalent to:
    ///
    /// ```text
    /// max(min(width, depth))
    /// ```
    ///
    /// over passing points.
    pub fn maximum_square_dimension(&self) -> Option<usize> {
        self.maximum_passing_square_dimension
    }
}

// ============================================================================
// Public computation API
// ============================================================================

/// Computes a volumetric performance frontier.
///
/// # Inputs
///
/// `measurements` must contain one already-aggregated measurement for every
/// `(width, depth)` coordinate.
///
/// `policy` defines what quality is considered acceptable.
///
/// # Errors
///
/// The function returns an error if:
///
/// - no measurements are supplied;
/// - width or depth is zero;
/// - quality is non-finite;
/// - threshold is invalid;
/// - duplicate coordinates exist.
///
/// # Example
///
/// ```
/// use zamani::quantum::benchmarking::volumetric::frontier::{
///     compute_frontier,
///     FrontierPolicy,
///     VolumetricMeasurement,
/// };
///
/// let measurements = vec![
///     VolumetricMeasurement::new(2, 2, 0.99).unwrap(),
///     VolumetricMeasurement::new(2, 4, 0.96).unwrap(),
///     VolumetricMeasurement::new(2, 6, 0.80).unwrap(),
///     VolumetricMeasurement::new(4, 2, 0.98).unwrap(),
///     VolumetricMeasurement::new(4, 4, 0.95).unwrap(),
///     VolumetricMeasurement::new(4, 6, 0.70).unwrap(),
/// ];
///
/// let policy = FrontierPolicy::quality(0.95).unwrap();
/// let result = compute_frontier(&measurements, policy).unwrap();
///
/// assert_eq!(
///     result.deepest_passing_at_width(2).unwrap().depth,
///     4
/// );
/// assert_eq!(
///     result.deepest_passing_at_width(4).unwrap().depth,
///     4
/// );
/// ```
///
/// The function never mutates the caller's input.
pub fn compute_frontier(
    measurements: &[VolumetricMeasurement],
    policy: FrontierPolicy,
) -> Result<FrontierResult, FrontierError> {
    policy.validate()?;

    if measurements.is_empty() {
        return Err(FrontierError::EmptyMeasurements);
    }

    // ------------------------------------------------------------------------
    // Validate and detect duplicate coordinates.
    // ------------------------------------------------------------------------

    let mut coordinates = BTreeSet::new();

    for (index, measurement) in measurements.iter().enumerate() {
        measurement.validate(index)?;

        if !coordinates.insert(measurement.coordinate()) {
            return Err(FrontierError::DuplicateCoordinate {
                width: measurement.width,
                depth: measurement.depth,
            });
        }
    }

    // ------------------------------------------------------------------------
    // Organize the surface by width.
    //
    // BTreeMap gives deterministic width ordering without depending on the
    // caller's input ordering.
    // ------------------------------------------------------------------------

    let mut by_width: BTreeMap<usize, Vec<VolumetricMeasurement>> =
        BTreeMap::new();

    for measurement in measurements.iter().copied() {
        by_width
            .entry(measurement.width)
            .or_default()
            .push(measurement);
    }

    // ------------------------------------------------------------------------
    // Extract the deepest passing point for every width.
    // ------------------------------------------------------------------------

    let mut width_frontier = Vec::with_capacity(by_width.len());
    let mut passing_measurement_count = 0usize;

    let mut maximum_passing_width = None;
    let mut maximum_passing_depth = None;
    let mut maximum_passing_square_dimension = None;

    for (width, mut width_measurements) in by_width {
        width_measurements.sort_by(|left, right| {
            left.depth.cmp(&right.depth).then_with(|| {
                compare_quality(
                    left.quality,
                    right.quality,
                    policy.objective,
                )
            })
        });

        let mut deepest_passing: Option<FrontierPoint> = None;

        for measurement in width_measurements {
            if policy.passes(measurement.quality) {
                passing_measurement_count += 1;

                let candidate = FrontierPoint {
                    width: measurement.width,
                    depth: measurement.depth,
                    quality: measurement.quality,
                };

                let should_replace = deepest_passing
                    .map(|current| {
                        candidate.depth > current.depth
                            || (candidate.depth == current.depth
                                && policy.objective.is_better(
                                    candidate.quality,
                                    current.quality,
                                ))
                    })
                    .unwrap_or(true);

                if should_replace {
                    deepest_passing = Some(candidate);
                }

                maximum_passing_width = Some(
                    maximum_passing_width
                        .map_or(measurement.width, |current| {
                            current.max(measurement.width)
                        }),
                );

                maximum_passing_depth = Some(
                    maximum_passing_depth
                        .map_or(measurement.depth, |current| {
                            current.max(measurement.depth)
                        }),
                );

                let square_dimension =
                    measurement.width.min(measurement.depth);

                maximum_passing_square_dimension = Some(
                    maximum_passing_square_dimension
                        .map_or(square_dimension, |current| {
                            current.max(square_dimension)
                        }),
                );
            }
        }

        if let Some(point) = deepest_passing {
            width_frontier.push(point);
        }
    }

    // ------------------------------------------------------------------------
    // Extract the true two-dimensional Pareto frontier.
    // ------------------------------------------------------------------------
    //
    // A passing point is dominated if another passing point is at least as
    // wide and at least as deep, with one dimension strictly greater.
    //
    // Because width_frontier contains the deepest passing point for each width,
    // any point omitted here is already represented by an equal-width point
    // with greater depth or by another width/depth combination.
    // ------------------------------------------------------------------------

    let pareto_frontier =
        extract_pareto_frontier(&width_frontier);

    Ok(FrontierResult {
        analysis_id: VOLUMETRIC_FRONTIER_ID,
        schema_version: VOLUMETRIC_FRONTIER_SCHEMA_VERSION,
        policy,
        measurement_count: measurements.len(),
        passing_measurement_count,
        width_count: coordinates
            .iter()
            .map(|coordinate| coordinate.0)
            .collect::<BTreeSet<_>>()
            .len(),
        depth_count: coordinates
            .iter()
            .map(|coordinate| coordinate.1)
            .collect::<BTreeSet<_>>()
            .len(),
        maximum_passing_width,
        maximum_passing_depth,
        maximum_passing_square_dimension,
        width_frontier,
        pareto_frontier,
    })
}

// ============================================================================
// Pareto extraction
// ============================================================================

/// Extracts the non-dominated points from a width frontier.
///
/// The input is expected to contain at most one point for every width.
fn extract_pareto_frontier(
    width_frontier: &[FrontierPoint],
) -> Vec<FrontierPoint> {
    if width_frontier.is_empty() {
        return Vec::new();
    }

    // Sort by width descending, then depth descending.
    //
    // For each point, `maximum_depth_seen` represents the greatest depth of a
    // point having a strictly greater width. A point is non-dominated when its
    // depth is strictly greater than that value.
    let mut sorted = width_frontier.to_vec();

    sorted.sort_by(|left, right| {
        right
            .width
            .cmp(&left.width)
            .then_with(|| right.depth.cmp(&left.depth))
    });

    let mut result = Vec::with_capacity(sorted.len());
    let mut maximum_depth_seen = 0usize;

    for point in sorted {
        if point.depth > maximum_depth_seen {
            result.push(point);
            maximum_depth_seen = point.depth;
        }
    }

    // Public result order is deterministic and easier for reports to consume.
    result.sort_by_key(|point| (point.width, point.depth));

    result
}

// ============================================================================
// Numerical helpers
// ============================================================================

/// Orders quality values according to the selected objective.
///
/// This helper is used only for deterministic ordering. It never treats NaN as
/// a valid value because all measurements have already been validated.
fn compare_quality(
    left: f64,
    right: f64,
    objective: FrontierObjective,
) -> std::cmp::Ordering {
    match objective {
        FrontierObjective::HigherIsBetter => {
            right.partial_cmp(&left).unwrap_or(std::cmp::Ordering::Equal)
        }

        FrontierObjective::LowerIsBetter => {
            left.partial_cmp(&right).unwrap_or(std::cmp::Ordering::Equal)
        }
    }
}

// ============================================================================
// Unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn measurement(
        width: usize,
        depth: usize,
        quality: f64,
    ) -> VolumetricMeasurement {
        VolumetricMeasurement::new(width, depth, quality).unwrap()
    }

    #[test]
    fn standard_quality_policy_uses_higher_is_better() {
        let policy = FrontierPolicy::quality(2.0 / 3.0).unwrap();

        assert_eq!(
            policy.objective,
            FrontierObjective::HigherIsBetter
        );

        assert!(policy.passes(0.8));
        assert!(!policy.passes(0.6));
    }

    #[test]
    fn rejects_empty_measurements() {
        let policy = FrontierPolicy::quality(0.95).unwrap();

        let result = compute_frontier(&[], policy);

        assert_eq!(result, Err(FrontierError::EmptyMeasurements));
    }

    #[test]
    fn rejects_zero_width() {
        let measurement = VolumetricMeasurement {
            width: 0,
            depth: 2,
            quality: 0.99,
        };

        assert_eq!(
            compute_frontier(
                &[measurement],
                FrontierPolicy::quality(0.95).unwrap()
            ),
            Err(FrontierError::InvalidWidth { index: 0 })
        );
    }

    #[test]
    fn rejects_zero_depth() {
        let measurement = VolumetricMeasurement {
            width: 2,
            depth: 0,
            quality: 0.99,
        };

        assert_eq!(
            compute_frontier(
                &[measurement],
                FrontierPolicy::quality(0.95).unwrap()
            ),
            Err(FrontierError::InvalidDepth { index: 0 })
        );
    }

    #[test]
    fn rejects_nan_quality() {
        let measurement = VolumetricMeasurement {
            width: 2,
            depth: 2,
            quality: f64::NAN,
        };

        match compute_frontier(
            &[measurement],
            FrontierPolicy::quality(0.95).unwrap(),
        ) {
            Err(FrontierError::NonFiniteQuality { index: 0, .. }) => {}
            other => panic!("unexpected result: {:?}", other),
        }
    }

    #[test]
    fn rejects_infinite_quality() {
        let measurement = VolumetricMeasurement {
            width: 2,
            depth: 2,
            quality: f64::INFINITY,
        };

        match compute_frontier(
            &[measurement],
            FrontierPolicy::quality(0.95).unwrap(),
        ) {
            Err(FrontierError::NonFiniteQuality { index: 0, .. }) => {}
            other => panic!("unexpected result: {:?}", other),
        }
    }

    #[test]
    fn rejects_non_finite_threshold() {
        let result = FrontierPolicy::quality(f64::NAN);

        match result {
            Err(FrontierError::NonFiniteThreshold { .. }) => {}
            other => panic!("unexpected result: {:?}", other),
        }
    }

    #[test]
    fn rejects_invalid_probability_threshold() {
        assert!(matches!(
            FrontierPolicy::quality(-0.1),
            Err(FrontierError::InvalidUnitIntervalThreshold { .. })
        ));

        assert!(matches!(
            FrontierPolicy::quality(1.1),
            Err(FrontierError::InvalidUnitIntervalThreshold { .. })
        ));
    }

    #[test]
    fn accepts_probability_threshold_boundaries() {
        assert!(FrontierPolicy::quality(0.0).is_ok());
        assert!(FrontierPolicy::quality(1.0).is_ok());
    }

    #[test]
    fn rejects_duplicate_coordinates() {
        let measurements = vec![
            measurement(2, 2, 0.99),
            measurement(2, 2, 0.98),
        ];

        assert_eq!(
            compute_frontier(
                &measurements,
                FrontierPolicy::quality(0.95).unwrap()
            ),
            Err(FrontierError::DuplicateCoordinate {
                width: 2,
                depth: 2,
            })
        );
    }

    #[test]
    fn extracts_deepest_passing_point_per_width() {
        let measurements = vec![
            measurement(2, 2, 0.99),
            measurement(2, 4, 0.97),
            measurement(2, 6, 0.80),
            measurement(4, 2, 0.98),
            measurement(4, 4, 0.95),
            measurement(4, 6, 0.70),
        ];

        let result = compute_frontier(
            &measurements,
            FrontierPolicy::quality(0.95).unwrap(),
        )
        .unwrap();

        assert_eq!(result.width_frontier.len(), 2);

        assert_eq!(
            result.deepest_passing_at_width(2).unwrap(),
            FrontierPoint {
                width: 2,
                depth: 4,
                quality: 0.97,
            }
        );

        assert_eq!(
            result.deepest_passing_at_width(4).unwrap(),
            FrontierPoint {
                width: 4,
                depth: 4,
                quality: 0.95,
            }
        );
    }

    #[test]
    fn extracts_pareto_frontier() {
        let measurements = vec![
            measurement(2, 8, 0.99),
            measurement(4, 6, 0.99),
            measurement(6, 4, 0.99),
            measurement(8, 2, 0.99),
        ];

        let result = compute_frontier(
            &measurements,
            FrontierPolicy::quality(0.95).unwrap(),
        )
        .unwrap();

        assert_eq!(
            result.pareto_frontier,
            vec![
                FrontierPoint {
                    width: 2,
                    depth: 8,
                    quality: 0.99,
                },
                FrontierPoint {
                    width: 4,
                    depth: 6,
                    quality: 0.99,
                },
                FrontierPoint {
                    width: 6,
                    depth: 4,
                    quality: 0.99,
                },
                FrontierPoint {
                    width: 8,
                    depth: 2,
                    quality: 0.99,
                },
            ]
        );
    }

    #[test]
    fn dominated_points_are_removed_from_pareto_frontier() {
        let measurements = vec![
            measurement(2, 2, 0.99),
            measurement(2, 8, 0.99),
            measurement(4, 4, 0.99),
            measurement(6, 2, 0.99),
        ];

        let result = compute_frontier(
            &measurements,
            FrontierPolicy::quality(0.95).unwrap(),
        )
        .unwrap();

        assert_eq!(
            result.pareto_frontier,
            vec![
                FrontierPoint {
                    width: 2,
                    depth: 8,
                    quality: 0.99,
                },
                FrontierPoint {
                    width: 4,
                    depth: 4,
                    quality: 0.99,
                },
                FrontierPoint {
                    width: 6,
                    depth: 2,
                    quality: 0.99,
                },
            ]
        );
    }

    #[test]
    fn no_measurement_passing_is_valid() {
        let measurements = vec![
            measurement(2, 2, 0.80),
            measurement(4, 4, 0.70),
        ];

        let result = compute_frontier(
            &measurements,
            FrontierPolicy::quality(0.95).unwrap(),
        )
        .unwrap();

        assert!(!result.has_passing_point());
        assert_eq!(result.maximum_passing_width, None);
        assert_eq!(result.maximum_passing_depth, None);
        assert_eq!(result.maximum_passing_square_dimension, None);
        assert!(result.width_frontier.is_empty());
        assert!(result.pareto_frontier.is_empty());
    }

    #[test]
    fn computes_maximum_square_dimension() {
        let measurements = vec![
            measurement(2, 8, 0.99),
            measurement(4, 6, 0.99),
            measurement(8, 4, 0.99),
        ];

        let result = compute_frontier(
            &measurements,
            FrontierPolicy::quality(0.95).unwrap(),
        )
        .unwrap();

        assert_eq!(
            result.maximum_passing_square_dimension,
            Some(4)
        );
    }

    #[test]
    fn deepest_passing_point_is_deterministic() {
        let measurements = vec![
            measurement(8, 4, 0.99),
            measurement(2, 8, 0.99),
            measurement(4, 6, 0.99),
        ];

        let result = compute_frontier(
            &measurements,
            FrontierPolicy::quality(0.95).unwrap(),
        )
        .unwrap();

        assert_eq!(
            result.deepest_passing_point(),
            Some(FrontierPoint {
                width: 2,
                depth: 8,
                quality: 0.99,
            })
        );
    }

    #[test]
    fn widest_passing_point_is_deterministic() {
        let measurements = vec![
            measurement(8, 4, 0.99),
            measurement(2, 8, 0.99),
            measurement(4, 6, 0.99),
        ];

        let result = compute_frontier(
            &measurements,
            FrontierPolicy::quality(0.95).unwrap(),
        )
        .unwrap();

        assert_eq!(
            result.widest_passing_point(),
            Some(FrontierPoint {
                width: 8,
                depth: 4,
                quality: 0.99,
            })
        );
    }

    #[test]
    fn lower_is_better_policy_is_supported() {
        let policy = FrontierPolicy::lower_is_better(0.05).unwrap();

        let measurements = vec![
            measurement(2, 2, 0.01),
            measurement(2, 4, 0.03),
            measurement(2, 6, 0.08),
        ];

        let result = compute_frontier(&measurements, policy).unwrap();

        assert_eq!(
            result.deepest_passing_at_width(2).unwrap().depth,
            4
        );
    }

    #[test]
    fn generic_higher_is_better_policy_accepts_non_probability_metrics() {
        let policy = FrontierPolicy::higher_is_better(100.0).unwrap();

        let measurements = vec![
            measurement(2, 2, 150.0),
            measurement(2, 4, 120.0),
            measurement(2, 6, 80.0),
        ];

        let result = compute_frontier(&measurements, policy).unwrap();

        assert_eq!(
            result.deepest_passing_at_width(2).unwrap().depth,
            4
        );
    }

    #[test]
    fn input_order_does_not_change_result() {
        let first = vec![
            measurement(2, 2, 0.99),
            measurement(4, 4, 0.95),
            measurement(2, 4, 0.97),
            measurement(4, 2, 0.98),
        ];

        let second = vec![
            measurement(4, 2, 0.98),
            measurement(2, 4, 0.97),
            measurement(4, 4, 0.95),
            measurement(2, 2, 0.99),
        ];

        let policy = FrontierPolicy::quality(0.95).unwrap();

        let first_result = compute_frontier(&first, policy).unwrap();
        let second_result = compute_frontier(&second, policy).unwrap();

        assert_eq!(first_result, second_result);
    }

    #[test]
    fn threshold_is_inclusive() {
        let measurement = measurement(4, 4, 0.95);

        let result = compute_frontier(
            &[measurement],
            FrontierPolicy::quality(0.95).unwrap(),
        )
        .unwrap();

        assert!(result.has_passing_point());
        assert_eq!(
            result.maximum_passing_square_dimension,
            Some(4)
        );
    }

    #[test]
    fn result_contains_correct_counts() {
        let measurements = vec![
            measurement(2, 2, 0.99),
            measurement(2, 4, 0.80),
            measurement(4, 2, 0.98),
            measurement(4, 4, 0.70),
        ];

        let result = compute_frontier(
            &measurements,
            FrontierPolicy::quality(0.95).unwrap(),
        )
        .unwrap();

        assert_eq!(result.measurement_count, 4);
        assert_eq!(result.passing_measurement_count, 2);
        assert_eq!(result.width_count, 2);
        assert_eq!(result.depth_count, 2);
    }

    #[test]
    fn width_frontier_is_sorted() {
        let measurements = vec![
            measurement(8, 8, 0.99),
            measurement(2, 2, 0.99),
            measurement(6, 6, 0.99),
            measurement(4, 4, 0.99),
        ];

        let result = compute_frontier(
            &measurements,
            FrontierPolicy::quality(0.95).unwrap(),
        )
        .unwrap();

        let widths: Vec<usize> = result
            .width_frontier
            .iter()
            .map(|point| point.width)
            .collect();

        assert_eq!(widths, vec![2, 4, 6, 8]);
    }

    #[test]
    fn pareto_frontier_is_sorted() {
        let measurements = vec![
            measurement(8, 2, 0.99),
            measurement(2, 8, 0.99),
            measurement(6, 4, 0.99),
            measurement(4, 6, 0.99),
        ];

        let result = compute_frontier(
            &measurements,
            FrontierPolicy::quality(0.95).unwrap(),
        )
        .unwrap();

        let coordinates: Vec<(usize, usize)> = result
            .pareto_frontier
            .iter()
            .map(|point| point.coordinate())
            .collect();

        assert_eq!(
            coordinates,
            vec![(2, 8), (4, 6), (6, 4), (8, 2)]
        );
    }

    #[test]
    fn schema_identity_is_stable() {
        let result = compute_frontier(
            &[measurement(2, 2, 0.99)],
            FrontierPolicy::quality(0.95).unwrap(),
        )
        .unwrap();

        assert_eq!(
            result.analysis_id,
            "volumetric_frontier"
        );

        assert_eq!(
            result.schema_version,
            VOLUMETRIC_FRONTIER_SCHEMA_VERSION
        );
    }

    #[test]
    fn measurement_coordinate_is_stable() {
        let value = measurement(7, 13, 0.91);

        assert_eq!(value.coordinate(), (7, 13));
    }

    #[test]
    fn width_lookup_uses_sorted_frontier() {
        let measurements = vec![
            measurement(8, 4, 0.99),
            measurement(2, 8, 0.99),
            measurement(4, 6, 0.99),
        ];

        let result = compute_frontier(
            &measurements,
            FrontierPolicy::quality(0.95).unwrap(),
        )
        .unwrap();

        assert_eq!(
            result.deepest_passing_at_width(4),
            Some(FrontierPoint {
                width: 4,
                depth: 6,
                quality: 0.99,
            })
        );

        assert_eq!(
            result.deepest_passing_at_width(3),
            None
        );
    }

    #[test]
    fn lower_is_better_pareto_frontier_uses_only_pass_fail_geometry() {
        let policy = FrontierPolicy::lower_is_better(0.05).unwrap();

        let measurements = vec![
            measurement(2, 8, 0.04),
            measurement(4, 6, 0.03),
            measurement(6, 4, 0.02),
            measurement(8, 2, 0.01),
        ];

        let result = compute_frontier(&measurements, policy).unwrap();

        assert_eq!(
            result.pareto_frontier.len(),
            4
        );
    }
}