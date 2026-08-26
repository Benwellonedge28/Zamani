//! Zamani Quantum Benchmarking — Volumetric Benchmark Core
//!
//! This module defines the backend-independent mathematical/data-model layer
//! for volumetric quantum benchmarking.
//!
//! # Purpose
//!
//! A volumetric benchmark measures a workload over multiple dimensions rather
//! than reducing a quantum system to one scalar benchmark number.
//!
//! The canonical two-dimensional representation is:
//
//! ```text
//!                 depth
//!                   ↑
//!                   │
//!                   │        ●
//!                   │     ●  ●
//!                   │  ●  ●  ●
//!                   │● ●  ●
//!                   └────────────────→ width
//! ```
//!
//! Each point represents an experimentally measured workload:
//
//! ```text
//! (width, depth) -> quality
//! ```
//!
//! This module deliberately does NOT:
//!
//! - generate quantum circuits;
//! - execute circuits;
//! - select hardware;
//! - select simulators;
//! - compile or transpile circuits;
//! - perform routing;
//! - perform scheduling;
//! - implement Quantum Volume;
//! - implement XEB;
//! - implement randomized benchmarking;
//! - implement application algorithms;
//! - depend on a quantum backend;
//! - depend on a simulator;
//! - depend on Quantum IR;
//! - perform statistical fitting;
//! - silently discard invalid measurements.
//!
//! Those responsibilities belong to the surrounding benchmarking architecture.
//!
//! # Architectural position
//!
//! ```text
//!                     Benchmark protocol
//!                            │
//!                            ▼
//!                     execution/results
//!                            │
//!                            ▼
//!                volumetric::volume
//!                            │
//!                ┌───────────┼───────────┐
//!                ▼           ▼           ▼
//!             surface     frontier   positioning
//! ```
//!
//! The intended dependency direction is therefore:
//
//! ```text
//! core::*
//!     ▲
//!     │
//! volumetric::volume
//!     ▲
//!     │
//! protocols / applications / qec
//! ```
//!
//! This file intentionally avoids importing future volumetric siblings. It can
//! therefore be completed and tested before `surface.rs`, `frontier.rs`, and
//! `positioning.rs` are implemented.
//!
//! # Why this abstraction exists
//!
//! Quantum Volume provides one scalar value, but a real quantum system can
//! simultaneously have different limits in:
//!
//! - width;
//! - circuit depth;
//! - fidelity;
//! - success probability;
//! - error rate;
//! - execution time;
//! - resource usage;
//! - logical quality;
//! - application quality.
//!
//! A volumetric surface preserves this multidimensional information.
//!
//! # Production requirements
//!
//! This implementation provides:
//!
//! - bounded input sizes;
//! - deterministic ordering;
//! - duplicate detection;
//! - finite-number validation;
//! - explicit quality semantics;
//! - configurable pass/fail predicates;
//! - exact integer arithmetic for coordinates;
//! - overflow-safe point counting;
//! - immutable-after-construction surfaces;
//! - deterministic frontier extraction;
//! - rectangular and sparse surfaces;
//! - missing-point detection;
//! - dominance queries;
//! - reproducible fingerprints;
//! - no global state;
//! - no logging/printing;
//! - no panics on normal invalid input;
//! - Rust 1.97 / 1.97.1 compatibility.
//!
//! # Integration contract
//!
//! `volumetric::volume` is intended to be consumed by:
//!
//! - `volumetric::surface`
//! - `volumetric::frontier`
//! - `volumetric::positioning`
//! - `protocols::quantum_volume`
//! - `protocols::xeb`
//! - application benchmarks
//! - QEC benchmarks
//! - `analysis::*`
//! - `reporting::*`
//!
//! The higher-level benchmark protocol should create a `VolumePoint` for each
//! completed experiment and add it to a `VolumeSurface`.
//!
//! No later module needs to modify the semantics of this file merely because
//! it is integrated with the rest of the benchmarking framework.
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

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::hash::{Hash, Hasher};

// ============================================================================
// Public constants
// ============================================================================

/// Stable benchmark-family identifier.
pub const VOLUMETRIC_BENCHMARK_ID: &str = "volumetric";

/// Stable schema version for this mathematical/data-model contract.
pub const VOLUMETRIC_VOLUME_SCHEMA_VERSION: u32 = 1;

/// Maximum number of points accepted by one surface.
///
/// This is deliberately bounded to prevent malformed input from forcing
/// unbounded memory growth.
pub const DEFAULT_MAX_POINTS: usize = 1_000_000;

/// Maximum supported width coordinate.
pub const DEFAULT_MAX_WIDTH: usize = 1_000_000;

/// Maximum supported depth coordinate.
pub const DEFAULT_MAX_DEPTH: usize = 1_000_000;

/// Maximum number of dimensions represented by one volumetric surface.
///
/// The current implementation is two-dimensional:
///
/// - width
/// - depth
pub const VOLUMETRIC_DIMENSION_COUNT: usize = 2;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the volumetric benchmark data model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VolumeError {
    /// Width is zero.
    InvalidWidth,

    /// Depth is zero.
    InvalidDepth,

    /// Width exceeds the configured maximum.
    WidthExceedsLimit {
        /// Supplied width.
        width: usize,

        /// Maximum accepted width.
        maximum: usize,
    },

    /// Depth exceeds the configured maximum.
    DepthExceedsLimit {
        /// Supplied depth.
        depth: usize,

        /// Maximum accepted depth.
        maximum: usize,
    },

    /// Quality is NaN or infinity.
    NonFiniteQuality,

    /// The quality value is outside the allowed range.
    QualityOutOfRange {
        /// Supplied quality.
        value_bits: u64,
    },

    /// The same `(width, depth)` coordinate was inserted twice.
    DuplicatePoint {
        /// Width of the duplicated coordinate.
        width: usize,

        /// Depth of the duplicated coordinate.
        depth: usize,
    },

    /// The surface would exceed its configured point limit.
    PointLimitExceeded {
        /// Current point count.
        current: usize,

        /// Maximum permitted point count.
        maximum: usize,
    },

    /// A configured limit is invalid.
    InvalidPointLimit,

    /// A quality threshold is not finite.
    NonFiniteThreshold,

    /// A quality threshold is outside the configured quality domain.
    ThresholdOutOfRange {
        /// Supplied threshold.
        value_bits: u64,
    },

    /// The quality semantics are inconsistent.
    InvalidQualitySemantics,

    /// A coordinate range is invalid.
    InvalidRange,

    /// A computed point count overflowed `usize`.
    CoordinateCountOverflow,

    /// A fingerprint operation could not produce a valid result.
    FingerprintFailure,
}

impl fmt::Display for VolumeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidWidth => {
                write!(formatter, "volumetric width must be greater than zero")
            }

            Self::InvalidDepth => {
                write!(formatter, "volumetric depth must be greater than zero")
            }

            Self::WidthExceedsLimit { width, maximum } => {
                write!(
                    formatter,
                    "volumetric width {} exceeds maximum {}",
                    width, maximum
                )
            }

            Self::DepthExceedsLimit { depth, maximum } => {
                write!(
                    formatter,
                    "volumetric depth {} exceeds maximum {}",
                    depth, maximum
                )
            }

            Self::NonFiniteQuality => {
                write!(formatter, "volumetric quality must be finite")
            }

            Self::QualityOutOfRange { .. } => {
                write!(
                    formatter,
                    "volumetric quality is outside the configured domain"
                )
            }

            Self::DuplicatePoint { width, depth } => {
                write!(
                    formatter,
                    "volumetric point ({}, {}) already exists",
                    width, depth
                )
            }

            Self::PointLimitExceeded { current, maximum } => {
                write!(
                    formatter,
                    "volumetric point limit exceeded: current={}, maximum={}",
                    current, maximum
                )
            }

            Self::InvalidPointLimit => {
                write!(
                    formatter,
                    "volumetric point limit must be greater than zero"
                )
            }

            Self::NonFiniteThreshold => {
                write!(
                    formatter,
                    "volumetric quality threshold must be finite"
                )
            }

            Self::ThresholdOutOfRange { .. } => {
                write!(
                    formatter,
                    "volumetric quality threshold is outside the configured domain"
                )
            }

            Self::InvalidQualitySemantics => {
                write!(
                    formatter,
                    "volumetric quality semantics are inconsistent"
                )
            }

            Self::InvalidRange => {
                write!(
                    formatter,
                    "volumetric coordinate range is invalid"
                )
            }

            Self::CoordinateCountOverflow => {
                write!(
                    formatter,
                    "volumetric coordinate count overflowed usize"
                )
            }

            Self::FingerprintFailure => {
                write!(
                    formatter,
                    "volumetric fingerprint calculation failed"
                )
            }
        }
    }
}

impl Error for VolumeError {}

// ============================================================================
// Quality semantics
// ============================================================================

/// Semantic direction of a volumetric quality metric.
///
/// The direction is critical because different benchmarks have opposite
/// meanings of "better":
///
/// - fidelity: higher is better
/// - success probability: higher is better
/// - error rate: lower is better
/// - runtime: lower is better
/// - energy: lower is better
///
/// A volumetric surface must never infer this from the numeric value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QualityDirection {
    /// Larger values are better.
    HigherIsBetter,

    /// Smaller values are better.
    LowerIsBetter,
}

impl QualityDirection {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HigherIsBetter => "higher_is_better",
            Self::LowerIsBetter => "lower_is_better",
        }
    }

    /// Returns whether a value satisfies a threshold.
    ///
    /// The comparison is inclusive:
    ///
    /// - higher-is-better: `value >= threshold`
    /// - lower-is-better: `value <= threshold`
    #[must_use]
    pub fn satisfies(self, value: f64, threshold: f64) -> bool {
        match self {
            Self::HigherIsBetter => value >= threshold,
            Self::LowerIsBetter => value <= threshold,
        }
    }

    /// Returns whether `left` is better than `right`.
    #[must_use]
    pub fn is_better(self, left: f64, right: f64) -> bool {
        match self {
            Self::HigherIsBetter => left > right,
            Self::LowerIsBetter => left < right,
        }
    }

    /// Returns whether `left` is at least as good as `right`.
    #[must_use]
    pub fn is_at_least_as_good(self, left: f64, right: f64) -> bool {
        match self {
            Self::HigherIsBetter => left >= right,
            Self::LowerIsBetter => left <= right,
        }
    }
}

// ============================================================================
// Quality domain
// ============================================================================

/// Mathematical domain of volumetric quality values.
///
/// A generic finite real domain is provided because not every benchmark metric
/// is a probability.
///
/// For probability/fidelity/error-rate benchmarks, use `UnitInterval`.
///
/// For runtime, energy, objective values, or other quantities, use `Finite`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QualityDomain {
    /// Quality must be in [0, 1].
    UnitInterval,

    /// Quality may be any finite `f64`.
    Finite,
}

impl QualityDomain {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnitInterval => "unit_interval",
            Self::Finite => "finite",
        }
    }

    fn validate(self, value: f64) -> Result<(), VolumeError> {
        if !value.is_finite() {
            return Err(VolumeError::NonFiniteQuality);
        }

        match self {
            Self::UnitInterval => {
                if !(0.0..=1.0).contains(&value) {
                    return Err(VolumeError::QualityOutOfRange {
                        value_bits: value.to_bits(),
                    });
                }
            }

            Self::Finite => {}
        }

        Ok(())
    }

    fn validate_threshold(self, threshold: f64) -> Result<(), VolumeError> {
        if !threshold.is_finite() {
            return Err(VolumeError::NonFiniteThreshold);
        }

        match self {
            Self::UnitInterval => {
                if !(0.0..=1.0).contains(&threshold) {
                    return Err(VolumeError::ThresholdOutOfRange {
                        value_bits: threshold.to_bits(),
                    });
                }
            }

            Self::Finite => {}
        }

        Ok(())
    }
}

// ============================================================================
// Quality specification
// ============================================================================

/// Complete semantic definition of the quality dimension of a volumetric
/// benchmark.
///
/// This prevents downstream modules from guessing what a scalar means.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QualitySpec {
    /// Stable metric identifier.
    ///
    /// Examples:
    ///
    /// - `fidelity`
    /// - `success_probability`
    /// - `error_rate`
    /// - `execution_time`
    pub metric_id: &'static str,

    /// Direction in which quality improves.
    pub direction: QualityDirection,

    /// Mathematical domain.
    pub domain: QualityDomain,

    /// Minimum acceptable quality.
    ///
    /// Interpretation depends on `direction`.
    pub threshold: f64,
}

impl QualitySpec {
    /// Construct a quality specification.
    pub fn new(
        metric_id: &'static str,
        direction: QualityDirection,
        domain: QualityDomain,
        threshold: f64,
    ) -> Result<Self, VolumeError> {
        if metric_id.is_empty() {
            return Err(VolumeError::InvalidQualitySemantics);
        }

        domain.validate_threshold(threshold)?;

        Ok(Self {
            metric_id,
            direction,
            domain,
            threshold,
        })
    }

    /// Standard fidelity specification.
    pub fn fidelity(threshold: f64) -> Result<Self, VolumeError> {
        Self::new(
            "fidelity",
            QualityDirection::HigherIsBetter,
            QualityDomain::UnitInterval,
            threshold,
        )
    }

    /// Standard success-probability specification.
    pub fn success_probability(threshold: f64) -> Result<Self, VolumeError> {
        Self::new(
            "success_probability",
            QualityDirection::HigherIsBetter,
            QualityDomain::UnitInterval,
            threshold,
        )
    }

    /// Standard error-rate specification.
    pub fn error_rate(threshold: f64) -> Result<Self, VolumeError> {
        Self::new(
            "error_rate",
            QualityDirection::LowerIsBetter,
            QualityDomain::UnitInterval,
            threshold,
        )
    }

    /// Standard runtime specification.
    pub fn runtime(threshold: f64) -> Result<Self, VolumeError> {
        Self::new(
            "runtime",
            QualityDirection::LowerIsBetter,
            QualityDomain::Finite,
            threshold,
        )
    }

    /// Validate a quality value.
    pub fn validate_value(&self, value: f64) -> Result<(), VolumeError> {
        self.domain.validate(value)
    }

    /// Determine whether a value passes the benchmark quality requirement.
    #[must_use]
    pub fn passes(&self, value: f64) -> bool {
        value.is_finite() && self.direction.satisfies(value, self.threshold)
    }
}

// ============================================================================
// Volume coordinate
// ============================================================================

/// Two-dimensional volumetric benchmark coordinate.
///
/// The dimensions are intentionally named `width` and `depth` rather than
/// `qubits` and `layers` because higher-level protocols may map them to
/// different physical or logical concepts.
///
/// For a gate-model circuit benchmark:
///
/// - width commonly maps to active qubits
/// - depth commonly maps to circuit depth
///
/// Other quantum technologies may define their own mapping while retaining the
/// same volumetric surface abstraction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VolumeCoordinate {
    /// Workload width.
    pub width: usize,

    /// Workload depth.
    pub depth: usize,
}

impl VolumeCoordinate {
    /// Creates a coordinate with the default safety limits.
    pub fn new(width: usize, depth: usize) -> Result<Self, VolumeError> {
        Self::with_limits(
            width,
            depth,
            DEFAULT_MAX_WIDTH,
            DEFAULT_MAX_DEPTH,
        )
    }

    /// Creates a coordinate with explicit limits.
    pub fn with_limits(
        width: usize,
        depth: usize,
        max_width: usize,
        max_depth: usize,
    ) -> Result<Self, VolumeError> {
        if width == 0 {
            return Err(VolumeError::InvalidWidth);
        }

        if depth == 0 {
            return Err(VolumeError::InvalidDepth);
        }

        if max_width == 0 {
            return Err(VolumeError::WidthExceedsLimit {
                width,
                maximum: max_width,
            });
        }

        if max_depth == 0 {
            return Err(VolumeError::DepthExceedsLimit {
                depth,
                maximum: max_depth,
            });
        }

        if width > max_width {
            return Err(VolumeError::WidthExceedsLimit {
                width,
                maximum: max_width,
            });
        }

        if depth > max_depth {
            return Err(VolumeError::DepthExceedsLimit {
                depth,
                maximum: max_depth,
            });
        }

        Ok(Self { width, depth })
    }

    /// Returns the square dimension associated with this coordinate.
    ///
    /// This is useful for Quantum Volume, where:
    ///
    /// `m = min(width, depth)`.
    #[must_use]
    pub const fn square_dimension(self) -> usize {
        if self.width < self.depth {
            self.width
        } else {
            self.depth
        }
    }

    /// Returns whether this coordinate lies on the diagonal.
    #[must_use]
    pub const fn is_square(self) -> bool {
        self.width == self.depth
    }

    /// Returns the coordinate as a `(width, depth)` tuple.
    #[must_use]
    pub const fn tuple(self) -> (usize, usize) {
        (self.width, self.depth)
    }
}

// ============================================================================
// Volume point
// ============================================================================

/// One measured point in a volumetric benchmark surface.
///
/// A point contains exactly one scalar quality value. Statistical uncertainty
/// belongs to the higher-level metric/result layer and should be attached to
/// the observation there; this structure intentionally does not manufacture
/// uncertainty that it does not have.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VolumePoint {
    /// Workload coordinate.
    pub coordinate: VolumeCoordinate,

    /// Measured quality value.
    pub quality: f64,

    /// Whether the measurement passed the configured quality envelope.
    ///
    /// This is stored explicitly so the point remains auditable even if a
    /// consumer later uses a different display threshold.
    pub passed: bool,
}

impl VolumePoint {
    /// Create a point using a quality specification.
    pub fn new(
        coordinate: VolumeCoordinate,
        quality: f64,
        quality_spec: QualitySpec,
    ) -> Result<Self, VolumeError> {
        quality_spec.validate_value(quality)?;

        Ok(Self {
            coordinate,
            quality,
            passed: quality_spec.passes(quality),
        })
    }

    /// Create a point with explicit pass/fail state.
    ///
    /// This is useful when the higher-level statistical layer has already made
    /// the benchmark decision, for example when QV uses a confidence-bound
    /// decision rather than a raw probability threshold.
    pub fn with_decision(
        coordinate: VolumeCoordinate,
        quality: f64,
        passed: bool,
    ) -> Result<Self, VolumeError> {
        if !quality.is_finite() {
            return Err(VolumeError::NonFiniteQuality);
        }

        Ok(Self {
            coordinate,
            quality,
            passed,
        })
    }

    /// Returns the width.
    #[must_use]
    pub const fn width(self) -> usize {
        self.coordinate.width
    }

    /// Returns the depth.
    #[must_use]
    pub const fn depth(self) -> usize {
        self.coordinate.depth
    }

    /// Returns the coordinate.
    #[must_use]
    pub const fn coordinate(self) -> VolumeCoordinate {
        self.coordinate
    }
}

// ============================================================================
// Surface configuration
// ============================================================================

/// Configuration controlling volumetric surface construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VolumeSurfaceConfig {
    /// Maximum width accepted by the surface.
    pub max_width: usize,

    /// Maximum depth accepted by the surface.
    pub max_depth: usize,

    /// Maximum number of points accepted.
    pub max_points: usize,

    /// Whether duplicate coordinates are rejected.
    ///
    /// Production mode should keep this true. It is exposed so specialized
    /// offline aggregation layers can intentionally choose another policy
    /// without changing the data structure.
    pub reject_duplicates: bool,
}

impl Default for VolumeSurfaceConfig {
    fn default() -> Self {
        Self {
            max_width: DEFAULT_MAX_WIDTH,
            max_depth: DEFAULT_MAX_DEPTH,
            max_points: DEFAULT_MAX_POINTS,
            reject_duplicates: true,
        }
    }
}

impl VolumeSurfaceConfig {
    /// Validate the surface configuration.
    pub fn validate(&self) -> Result<(), VolumeError> {
        if self.max_width == 0 {
            return Err(VolumeError::WidthExceedsLimit {
                width: 0,
                maximum: self.max_width,
            });
        }

        if self.max_depth == 0 {
            return Err(VolumeError::DepthExceedsLimit {
                depth: 0,
                maximum: self.max_depth,
            });
        }

        if self.max_points == 0 {
            return Err(VolumeError::InvalidPointLimit);
        }

        Ok(())
    }
}

// ============================================================================
// Volume surface
// ============================================================================

/// Deterministic sparse volumetric benchmark surface.
///
/// Internally this uses `BTreeMap` so iteration order is stable:
///
/// 1. width ascending
/// 2. depth ascending
///
/// Stable ordering is important for:
///
/// - reports;
/// - regression comparisons;
/// - serialization;
/// - fingerprints;
/// - reproducibility.
///
/// The structure is intentionally sparse. A volumetric benchmark does not have
/// to execute every possible `(width, depth)` combination.
#[derive(Debug, Clone, PartialEq)]
pub struct VolumeSurface {
    /// Surface configuration.
    config: VolumeSurfaceConfig,

    /// Quality specification.
    quality_spec: QualitySpec,

    /// Measured points.
    points: BTreeMap<VolumeCoordinate, VolumePoint>,
}

impl VolumeSurface {
    /// Creates an empty surface using default safety limits.
    pub fn new(quality_spec: QualitySpec) -> Result<Self, VolumeError> {
        Self::with_config(quality_spec, VolumeSurfaceConfig::default())
    }

    /// Creates an empty surface using explicit safety limits.
    pub fn with_config(
        quality_spec: QualitySpec,
        config: VolumeSurfaceConfig,
    ) -> Result<Self, VolumeError> {
        config.validate()?;

        Ok(Self {
            config,
            quality_spec,
            points: BTreeMap::new(),
        })
    }

    /// Returns the immutable surface configuration.
    #[must_use]
    pub const fn config(&self) -> &VolumeSurfaceConfig {
        &self.config
    }

    /// Returns the quality specification.
    #[must_use]
    pub const fn quality_spec(&self) -> &QualitySpec {
        &self.quality_spec
    }

    /// Returns the number of measured points.
    #[must_use]
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Returns whether the surface contains no points.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Insert a measured point.
    ///
    /// The quality is revalidated against the surface's quality domain.
    pub fn insert(&mut self, point: VolumePoint) -> Result<(), VolumeError> {
        VolumeCoordinate::with_limits(
            point.width(),
            point.depth(),
            self.config.max_width,
            self.config.max_depth,
        )?;

        self.quality_spec.validate_value(point.quality)?;

        if self.points.len() >= self.config.max_points
            && !self.points.contains_key(&point.coordinate)
        {
            return Err(VolumeError::PointLimitExceeded {
                current: self.points.len(),
                maximum: self.config.max_points,
            });
        }

        if self.config.reject_duplicates
            && self.points.contains_key(&point.coordinate)
        {
            return Err(VolumeError::DuplicatePoint {
                width: point.width(),
                depth: point.depth(),
            });
        }

        self.points.insert(point.coordinate, point);

        Ok(())
    }

    /// Insert a point directly from coordinate and quality.
    pub fn insert_measurement(
        &mut self,
        width: usize,
        depth: usize,
        quality: f64,
    ) -> Result<(), VolumeError> {
        let coordinate = VolumeCoordinate::with_limits(
            width,
            depth,
            self.config.max_width,
            self.config.max_depth,
        )?;

        let point = VolumePoint::new(
            coordinate,
            quality,
            self.quality_spec,
        )?;

        self.insert(point)
    }

    /// Replace an existing point or insert a new one.
    ///
    /// This is deliberately separate from `insert()` because accidental
    /// overwrites are dangerous in scientific benchmarking.
    pub fn upsert(&mut self, point: VolumePoint) -> Result<(), VolumeError> {
        VolumeCoordinate::with_limits(
            point.width(),
            point.depth(),
            self.config.max_width,
            self.config.max_depth,
        )?;

        self.quality_spec.validate_value(point.quality)?;

        if !self.points.contains_key(&point.coordinate)
            && self.points.len() >= self.config.max_points
        {
            return Err(VolumeError::PointLimitExceeded {
                current: self.points.len(),
                maximum: self.config.max_points,
            });
        }

        self.points.insert(point.coordinate, point);

        Ok(())
    }

    /// Retrieve a point by coordinate.
    #[must_use]
    pub fn get(
        &self,
        coordinate: VolumeCoordinate,
    ) -> Option<&VolumePoint> {
        self.points.get(&coordinate)
    }

    /// Retrieve a point by width and depth.
    #[must_use]
    pub fn get_at(
        &self,
        width: usize,
        depth: usize,
    ) -> Option<&VolumePoint> {
        self.points.get(&VolumeCoordinate { width, depth })
    }

    /// Returns all points in deterministic coordinate order.
    pub fn points(
        &self,
    ) -> impl Iterator<Item = &VolumePoint> {
        self.points.values()
    }

    /// Returns all points as a deterministic vector.
    #[must_use]
    pub fn to_vec(&self) -> Vec<VolumePoint> {
        self.points.values().copied().collect()
    }

    /// Returns all coordinates in deterministic order.
    #[must_use]
    pub fn coordinates(&self) -> Vec<VolumeCoordinate> {
        self.points.keys().copied().collect()
    }

    /// Returns the largest observed width.
    #[must_use]
    pub fn max_observed_width(&self) -> Option<usize> {
        self.points.keys().map(|coordinate| coordinate.width).max()
    }

    /// Returns the largest observed depth.
    #[must_use]
    pub fn max_observed_depth(&self) -> Option<usize> {
        self.points.keys().map(|coordinate| coordinate.depth).max()
    }

    /// Returns the largest observed square dimension.
    #[must_use]
    pub fn max_observed_square_dimension(&self) -> Option<usize> {
        self.points
            .keys()
            .map(VolumeCoordinate::square_dimension)
            .max()
    }

    /// Returns all points satisfying the quality envelope.
    #[must_use]
    pub fn passing_points(&self) -> Vec<VolumePoint> {
        self.points
            .values()
            .copied()
            .filter(|point| point.passed)
            .collect()
    }

    /// Returns all points failing the quality envelope.
    #[must_use]
    pub fn failing_points(&self) -> Vec<VolumePoint> {
        self.points
            .values()
            .copied()
            .filter(|point| !point.passed)
            .collect()
    }

    /// Returns the number of passing points.
    #[must_use]
    pub fn passing_count(&self) -> usize {
        self.points.values().filter(|point| point.passed).count()
    }

    /// Returns the number of failing points.
    #[must_use]
    pub fn failing_count(&self) -> usize {
        self.points.values().filter(|point| !point.passed).count()
    }

    /// Returns whether a coordinate exists.
    #[must_use]
    pub fn contains(&self, coordinate: VolumeCoordinate) -> bool {
        self.points.contains_key(&coordinate)
    }

    /// Determine whether every coordinate in a rectangular range exists.
    ///
    /// The ranges are inclusive.
    pub fn is_complete_rectangle(
        &self,
        min_width: usize,
        max_width: usize,
        min_depth: usize,
        max_depth: usize,
    ) -> Result<bool, VolumeError> {
        validate_range(min_width, max_width)?;
        validate_range(min_depth, max_depth)?;

        let expected = checked_rectangle_size(
            min_width,
            max_width,
            min_depth,
            max_depth,
        )?;

        if expected > self.config.max_points {
            return Err(VolumeError::PointLimitExceeded {
                current: expected,
                maximum: self.config.max_points,
            });
        }

        for width in min_width..=max_width {
            for depth in min_depth..=max_depth {
                if !self.contains(VolumeCoordinate { width, depth }) {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    /// Return missing coordinates from an inclusive rectangular range.
    ///
    /// The result is deterministic.
    pub fn missing_coordinates(
        &self,
        min_width: usize,
        max_width: usize,
        min_depth: usize,
        max_depth: usize,
    ) -> Result<Vec<VolumeCoordinate>, VolumeError> {
        validate_range(min_width, max_width)?;
        validate_range(min_depth, max_depth)?;

        let expected = checked_rectangle_size(
            min_width,
            max_width,
            min_depth,
            max_depth,
        )?;

        if expected > self.config.max_points {
            return Err(VolumeError::PointLimitExceeded {
                current: expected,
                maximum: self.config.max_points,
            });
        }

        let mut missing = Vec::new();

        for width in min_width..=max_width {
            for depth in min_depth..=max_depth {
                let coordinate = VolumeCoordinate { width, depth };

                if !self.contains(coordinate) {
                    missing.push(coordinate);
                }
            }
        }

        Ok(missing)
    }

    /// Return the best point according to the configured quality direction.
    ///
    /// Ties are resolved deterministically by:
    ///
    /// 1. better quality;
    /// 2. larger square dimension;
    /// 3. larger width;
    /// 4. larger depth.
    #[must_use]
    pub fn best_point(&self) -> Option<VolumePoint> {
        self.points
            .values()
            .copied()
            .max_by(|left, right| compare_points(
                *left,
                *right,
                self.quality_spec.direction,
            ))
    }

    /// Return the best passing point.
    ///
    /// This is often the most useful query for a volumetric benchmark.
    #[must_use]
    pub fn best_passing_point(&self) -> Option<VolumePoint> {
        self.points
            .values()
            .copied()
            .filter(|point| point.passed)
            .max_by(|left, right| compare_points(
                *left,
                *right,
                self.quality_spec.direction,
            ))
    }

    /// Return the largest passing square dimension.
    ///
    /// This is deliberately generic and does not calculate Quantum Volume.
    ///
    /// For Quantum Volume, the higher-level QV protocol may transform the
    /// resulting dimension `m` into `2^m` using its own checked arithmetic and
    /// statistical decision semantics.
    #[must_use]
    pub fn largest_passing_square_dimension(&self) -> Option<usize> {
        self.points
            .values()
            .filter(|point| point.passed)
            .map(|point| point.coordinate.square_dimension())
            .max()
    }

    /// Return the point with the largest square dimension that passes.
    ///
    /// If several points have the same square dimension, the quality direction
    /// determines the preferred point, followed by deterministic coordinates.
    #[must_use]
    pub fn largest_passing_square_point(&self) -> Option<VolumePoint> {
        self.points
            .values()
            .copied()
            .filter(|point| point.passed)
            .max_by(|left, right| {
                let left_square =
                    left.coordinate.square_dimension();

                let right_square =
                    right.coordinate.square_dimension();

                match left_square.cmp(&right_square) {
                    Ordering::Equal => compare_points(
                        *left,
                        *right,
                        self.quality_spec.direction,
                    ),

                    ordering => ordering,
                }
            })
    }

    /// Returns the Pareto frontier of measured points.
    ///
    /// A point is dominated if another measured point is at least as large in
    /// both width and depth and at least as good in quality, with at least one
    /// strict improvement.
    ///
    /// This is useful for volumetric positioning and system comparison.
    #[must_use]
    pub fn pareto_frontier(&self) -> Vec<VolumePoint> {
        let points = self.to_vec();

        let mut frontier = Vec::new();

        for candidate in points.iter().copied() {
            let dominated = points.iter().copied().any(|other| {
                other.coordinate.width >= candidate.coordinate.width
                    && other.coordinate.depth >= candidate.coordinate.depth
                    && self
                        .quality_spec
                        .direction
                        .is_at_least_as_good(
                            other.quality,
                            candidate.quality,
                        )
                    && (
                        other.coordinate.width > candidate.coordinate.width
                            || other.coordinate.depth > candidate.coordinate.depth
                            || self
                                .quality_spec
                                .direction
                                .is_better(
                                    other.quality,
                                    candidate.quality,
                                )
                    )
            });

            if !dominated {
                frontier.push(candidate);
            }
        }

        frontier.sort_by(|left, right| {
            left.coordinate
                .cmp(&right.coordinate)
        });

        frontier
    }

    /// Return all measured points whose square dimension is at least `minimum`.
    #[must_use]
    pub fn points_at_or_above_square_dimension(
        &self,
        minimum: usize,
    ) -> Vec<VolumePoint> {
        self.points
            .values()
            .copied()
            .filter(|point| {
                point.coordinate.square_dimension() >= minimum
            })
            .collect()
    }

    /// Count points in an inclusive rectangle.
    pub fn count_in_rectangle(
        &self,
        min_width: usize,
        max_width: usize,
        min_depth: usize,
        max_depth: usize,
    ) -> Result<usize, VolumeError> {
        validate_range(min_width, max_width)?;
        validate_range(min_depth, max_depth)?;

        Ok(self
            .points
            .keys()
            .filter(|coordinate| {
                coordinate.width >= min_width
                    && coordinate.width <= max_width
                    && coordinate.depth >= min_depth
                    && coordinate.depth <= max_depth
            })
            .count())
    }

    /// Calculate the fraction of expected points that are present in an
    /// inclusive rectangle.
    pub fn coverage(
        &self,
        min_width: usize,
        max_width: usize,
        min_depth: usize,
        max_depth: usize,
    ) -> Result<f64, VolumeError> {
        let expected = checked_rectangle_size(
            min_width,
            max_width,
            min_depth,
            max_depth,
        )?;

        let observed = self.count_in_rectangle(
            min_width,
            max_width,
            min_depth,
            max_depth,
        )?;

        if expected == 0 {
            return Err(VolumeError::CoordinateCountOverflow);
        }

        Ok(observed as f64 / expected as f64)
    }

    /// Return points grouped by width.
    ///
    /// The returned map has deterministic key and value ordering.
    #[must_use]
    pub fn by_width(
        &self,
    ) -> BTreeMap<usize, Vec<VolumePoint>> {
        let mut result: BTreeMap<usize, Vec<VolumePoint>> =
            BTreeMap::new();

        for point in self.points.values().copied() {
            result
                .entry(point.width())
                .or_default()
                .push(point);
        }

        result
    }

    /// Return points grouped by depth.
    #[must_use]
    pub fn by_depth(
        &self,
    ) -> BTreeMap<usize, Vec<VolumePoint>> {
        let mut result: BTreeMap<usize, Vec<VolumePoint>> =
            BTreeMap::new();

        for point in self.points.values().copied() {
            result
                .entry(point.depth())
                .or_default()
                .push(point);
        }

        result
    }

    /// Return the unique observed widths.
    #[must_use]
    pub fn widths(&self) -> Vec<usize> {
        self.points
            .keys()
            .map(|coordinate| coordinate.width)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    /// Return the unique observed depths.
    #[must_use]
    pub fn depths(&self) -> Vec<usize> {
        self.points
            .keys()
            .map(|coordinate| coordinate.depth)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    /// Produce a deterministic 64-bit fingerprint of the surface.
    ///
    /// This fingerprint is intended for:
    ///
    /// - reproducibility checks;
    /// - regression fixtures;
    /// - cache keys;
    /// - result identity.
    ///
    /// It is NOT intended to be a cryptographic hash.
    #[must_use]
    pub fn fingerprint(&self) -> u64 {
        let mut hasher = DeterministicHasher::new();

        self.quality_spec.metric_id.hash(&mut hasher);
        self.quality_spec.direction.hash(&mut hasher);
        self.quality_spec.domain.hash(&mut hasher);
        self.quality_spec.threshold.to_bits().hash(&mut hasher);

        self.config.max_width.hash(&mut hasher);
        self.config.max_depth.hash(&mut hasher);
        self.config.max_points.hash(&mut hasher);
        self.config.reject_duplicates.hash(&mut hasher);

        for point in self.points.values() {
            point.coordinate.width.hash(&mut hasher);
            point.coordinate.depth.hash(&mut hasher);
            point.quality.to_bits().hash(&mut hasher);
            point.passed.hash(&mut hasher);
        }

        hasher.finish()
    }
}

// ============================================================================
// Surface constructors
// ============================================================================

impl VolumeSurface {
    /// Construct a surface from an iterator of points.
    ///
    /// Duplicate coordinates are rejected according to the supplied
    /// configuration.
    pub fn from_points<I>(
        quality_spec: QualitySpec,
        config: VolumeSurfaceConfig,
        points: I,
    ) -> Result<Self, VolumeError>
    where
        I: IntoIterator<Item = VolumePoint>,
    {
        let mut surface =
            Self::with_config(quality_spec, config)?;

        for point in points {
            surface.insert(point)?;
        }

        Ok(surface)
    }

    /// Construct a surface from `(width, depth, quality)` tuples.
    pub fn from_measurements<I>(
        quality_spec: QualitySpec,
        config: VolumeSurfaceConfig,
        measurements: I,
    ) -> Result<Self, VolumeError>
    where
        I: IntoIterator<Item = (usize, usize, f64)>,
    {
        let mut surface =
            Self::with_config(quality_spec, config)?;

        for (width, depth, quality) in measurements {
            surface.insert_measurement(
                width,
                depth,
                quality,
            )?;
        }

        Ok(surface)
    }
}

// ============================================================================
// Coordinate ranges
// ============================================================================

/// Inclusive coordinate range for volumetric experiments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoordinateRange {
    /// Inclusive minimum.
    pub min: usize,

    /// Inclusive maximum.
    pub max: usize,
}

impl CoordinateRange {
    /// Creates an inclusive coordinate range.
    pub fn new(
        min: usize,
        max: usize,
    ) -> Result<Self, VolumeError> {
        if min == 0 || max == 0 || min > max {
            return Err(VolumeError::InvalidRange);
        }

        Ok(Self { min, max })
    }

    /// Number of integer coordinates represented by this range.
    pub fn len(&self) -> Result<usize, VolumeError> {
        self.max
            .checked_sub(self.min)
            .and_then(|difference| difference.checked_add(1))
            .ok_or(VolumeError::CoordinateCountOverflow)
    }

    /// Returns whether the range is empty.
    ///
    /// Construction rejects empty ranges, therefore this always returns false
    /// for a valid instance.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Returns whether the range contains a value.
    #[must_use]
    pub const fn contains(&self, value: usize) -> bool {
        value >= self.min && value <= self.max
    }
}

// ============================================================================
// Volumetric workload grid
// ============================================================================

/// A planned set of volumetric coordinates.
///
/// This type represents the *requested experiment space*, not measured
/// results. It is useful to the protocol/execution layer when planning a
/// benchmark.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VolumeGrid {
    /// Width range.
    pub width: CoordinateRange,

    /// Depth range.
    pub depth: CoordinateRange,
}

impl VolumeGrid {
    /// Create a grid.
    pub fn new(
        width: CoordinateRange,
        depth: CoordinateRange,
    ) -> Result<Self, VolumeError> {
        let count = checked_rectangle_size(
            width.min,
            width.max,
            depth.min,
            depth.max,
        )?;

        if count == 0 {
            return Err(VolumeError::CoordinateCountOverflow);
        }

        Ok(Self { width, depth })
    }

    /// Number of coordinates in the grid.
    pub fn len(&self) -> Result<usize, VolumeError> {
        checked_rectangle_size(
            self.width.min,
            self.width.max,
            self.depth.min,
            self.depth.max,
        )
    }

    /// Returns whether the grid contains a coordinate.
    #[must_use]
    pub const fn contains(
        &self,
        coordinate: VolumeCoordinate,
    ) -> bool {
        self.width.contains(coordinate.width)
            && self.depth.contains(coordinate.depth)
    }

    /// Iterate over coordinates in deterministic order.
    pub fn coordinates(
        &self,
    ) -> VolumeGridIter {
        VolumeGridIter {
            grid: self.clone(),
            current_width: self.width.min,
            current_depth: self.depth.min,
            finished: false,
        }
    }
}

/// Deterministic iterator over a volumetric workload grid.
#[derive(Debug, Clone)]
pub struct VolumeGridIter {
    grid: VolumeGrid,
    current_width: usize,
    current_depth: usize,
    finished: bool,
}

impl Iterator for VolumeGridIter {
    type Item = VolumeCoordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let coordinate = VolumeCoordinate {
            width: self.current_width,
            depth: self.current_depth,
        };

        if self.current_depth == self.grid.depth.max {
            if self.current_width == self.grid.width.max {
                self.finished = true;
            } else {
                self.current_width += 1;
                self.current_depth = self.grid.depth.min;
            }
        } else {
            self.current_depth += 1;
        }

        Some(coordinate)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = if self.finished {
            0
        } else {
            let width_remaining = self
                .grid
                .width
                .max
                .saturating_sub(self.current_width)
                .saturating_add(1);

            let depth_remaining = self
                .grid
                .depth
                .max
                .saturating_sub(self.current_depth)
                .saturating_add(1);

            width_remaining
                .saturating_mul(depth_remaining)
        };

        (remaining, Some(remaining))
    }
}

// ============================================================================
// Summary
// ============================================================================

/// Summary statistics for a volumetric surface.
///
/// This is intentionally descriptive rather than protocol-specific.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VolumeSummary {
    /// Number of measured points.
    pub point_count: usize,

    /// Number of passing points.
    pub passing_count: usize,

    /// Number of failing points.
    pub failing_count: usize,

    /// Maximum observed width.
    pub max_width: Option<usize>,

    /// Maximum observed depth.
    pub max_depth: Option<usize>,

    /// Maximum observed square dimension.
    pub max_square_dimension: Option<usize>,

    /// Best measured quality.
    pub best_quality: Option<f64>,

    /// Quality of the best passing point.
    pub best_passing_quality: Option<f64>,

    /// Deterministic surface fingerprint.
    pub fingerprint: u64,
}

impl VolumeSurface {
    /// Produce a summary of the surface.
    #[must_use]
    pub fn summary(&self) -> VolumeSummary {
        let best = self.best_point();

        let best_passing = self.best_passing_point();

        VolumeSummary {
            point_count: self.len(),
            passing_count: self.passing_count(),
            failing_count: self.failing_count(),
            max_width: self.max_observed_width(),
            max_depth: self.max_observed_depth(),
            max_square_dimension:
                self.max_observed_square_dimension(),
            best_quality: best.map(|point| point.quality),
            best_passing_quality:
                best_passing.map(|point| point.quality),
            fingerprint: self.fingerprint(),
        }
    }
}

// ============================================================================
// Helper functions
// ============================================================================

fn validate_range(
    minimum: usize,
    maximum: usize,
) -> Result<(), VolumeError> {
    if minimum == 0
        || maximum == 0
        || minimum > maximum
    {
        return Err(VolumeError::InvalidRange);
    }

    Ok(())
}

fn checked_rectangle_size(
    min_width: usize,
    max_width: usize,
    min_depth: usize,
    max_depth: usize,
) -> Result<usize, VolumeError> {
    validate_range(min_width, max_width)?;
    validate_range(min_depth, max_depth)?;

    let width_count = max_width
        .checked_sub(min_width)
        .and_then(|difference| difference.checked_add(1))
        .ok_or(VolumeError::CoordinateCountOverflow)?;

    let depth_count = max_depth
        .checked_sub(min_depth)
        .and_then(|difference| difference.checked_add(1))
        .ok_or(VolumeError::CoordinateCountOverflow)?;

    width_count
        .checked_mul(depth_count)
        .ok_or(VolumeError::CoordinateCountOverflow)
}

fn compare_points(
    left: VolumePoint,
    right: VolumePoint,
    direction: QualityDirection,
) -> Ordering {
    let quality_order = match direction {
        QualityDirection::HigherIsBetter => {
            left.quality
                .partial_cmp(&right.quality)
                .unwrap_or(Ordering::Equal)
        }

        QualityDirection::LowerIsBetter => {
            right
                .quality
                .partial_cmp(&left.quality)
                .unwrap_or(Ordering::Equal)
        }
    };

    match quality_order {
        Ordering::Equal => {
            left.coordinate
                .square_dimension()
                .cmp(&right.coordinate.square_dimension())
                .then_with(|| {
                    left.coordinate.width.cmp(
                        &right.coordinate.width
                    )
                })
                .then_with(|| {
                    left.coordinate.depth.cmp(
                        &right.coordinate.depth
                    )
                })
        }

        ordering => ordering,
    }
}

// ============================================================================
// Deterministic non-cryptographic hasher
// ============================================================================

/// Small deterministic hasher used only for reproducibility fingerprints.
///
/// `DefaultHasher` is intentionally avoided because its implementation details
/// are not a suitable long-term reproducibility contract.
#[derive(Debug, Clone, Copy)]
struct DeterministicHasher {
    state: u64,
}

impl DeterministicHasher {
    fn new() -> Self {
        Self {
            state: 0xcbf29ce484222325,
        }
    }
}

impl Hasher for DeterministicHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        // FNV-1a style mixing with an additional avalanche step.
        for byte in bytes {
            self.state ^= u64::from(*byte);
            self.state =
                self.state.wrapping_mul(0x100000001b3);
        }

        self.state ^= self.state >> 33;
        self.state = self
            .state
            .wrapping_mul(0xff51afd7ed558ccd);
        self.state ^= self.state >> 33;
        self.state = self
            .state
            .wrapping_mul(0xc4ceb9fe1a85ec53);
        self.state ^= self.state >> 33;
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn fidelity_spec() -> QualitySpec {
        QualitySpec::fidelity(0.9)
            .expect("valid fidelity specification")
    }

    fn error_spec() -> QualitySpec {
        QualitySpec::error_rate(0.1)
            .expect("valid error-rate specification")
    }

    #[test]
    fn coordinate_creation_is_valid() {
        let coordinate =
            VolumeCoordinate::new(8, 16)
                .expect("coordinate must be valid");

        assert_eq!(coordinate.width, 8);
        assert_eq!(coordinate.depth, 16);
        assert_eq!(coordinate.square_dimension(), 8);
        assert!(!coordinate.is_square());
    }

    #[test]
    fn zero_width_is_rejected() {
        assert_eq!(
            VolumeCoordinate::new(0, 4),
            Err(VolumeError::InvalidWidth)
        );
    }

    #[test]
    fn zero_depth_is_rejected() {
        assert_eq!(
            VolumeCoordinate::new(4, 0),
            Err(VolumeError::InvalidDepth)
        );
    }

    #[test]
    fn coordinate_limits_are_enforced() {
        assert!(matches!(
            VolumeCoordinate::with_limits(
                101,
                4,
                100,
                100,
            ),
            Err(VolumeError::WidthExceedsLimit {
                width: 101,
                maximum: 100,
            })
        ));

        assert!(matches!(
            VolumeCoordinate::with_limits(
                4,
                101,
                100,
                100,
            ),
            Err(VolumeError::DepthExceedsLimit {
                depth: 101,
                maximum: 100,
            })
        ));
    }

    #[test]
    fn quality_spec_validates_unit_interval() {
        assert!(QualitySpec::fidelity(0.9).is_ok());

        assert!(matches!(
            QualitySpec::fidelity(1.1),
            Err(VolumeError::ThresholdOutOfRange { .. })
        ));

        assert!(matches!(
            QualitySpec::fidelity(-0.1),
            Err(VolumeError::ThresholdOutOfRange { .. })
        ));

        assert_eq!(
            QualitySpec::fidelity(f64::NAN),
            Err(VolumeError::NonFiniteThreshold)
        );
    }

    #[test]
    fn finite_quality_domain_accepts_non_probability_values() {
        let spec =
            QualitySpec::runtime(10.0)
                .expect("runtime specification must be valid");

        assert!(
            spec.validate_value(5.0).is_ok()
        );

        assert!(
            spec.validate_value(f64::INFINITY).is_err()
        );

        assert!(
            spec.validate_value(f64::NEG_INFINITY).is_err()
        );
    }

    #[test]
    fn quality_direction_is_respected() {
        let higher =
            QualitySpec::fidelity(0.9)
                .expect("valid specification");

        assert!(higher.passes(0.9));
        assert!(higher.passes(0.95));
        assert!(!higher.passes(0.89));

        let lower =
            QualitySpec::error_rate(0.1)
                .expect("valid specification");

        assert!(lower.passes(0.1));
        assert!(lower.passes(0.05));
        assert!(!lower.passes(0.11));
    }

    #[test]
    fn point_records_decision() {
        let coordinate =
            VolumeCoordinate::new(8, 8)
                .expect("coordinate must be valid");

        let passing =
            VolumePoint::new(
                coordinate,
                0.95,
                fidelity_spec(),
            )
            .expect("point must be valid");

        assert!(passing.passed);

        let failing =
            VolumePoint::new(
                coordinate,
                0.80,
                fidelity_spec(),
            )
            .expect("point must be valid");

        assert!(!failing.passed);
    }

    #[test]
    fn explicit_decision_supports_statistical_protocols() {
        let coordinate =
            VolumeCoordinate::new(8, 8)
                .expect("coordinate must be valid");

        let point =
            VolumePoint::with_decision(
                coordinate,
                0.65,
                true,
            )
            .expect("point must be valid");

        assert!(point.passed);
        assert_eq!(point.quality, 0.65);
    }

    #[test]
    fn surface_accepts_unique_points() {
        let mut surface =
            VolumeSurface::new(fidelity_spec())
                .expect("surface must be valid");

        surface
            .insert_measurement(2, 2, 0.95)
            .expect("point must insert");

        surface
            .insert_measurement(4, 4, 0.92)
            .expect("point must insert");

        assert_eq!(surface.len(), 2);
    }

    #[test]
    fn duplicate_points_are_rejected() {
        let mut surface =
            VolumeSurface::new(fidelity_spec())
                .expect("surface must be valid");

        surface
            .insert_measurement(2, 2, 0.95)
            .expect("first point must insert");

        assert_eq!(
            surface.insert_measurement(
                2,
                2,
                0.96
            ),
            Err(VolumeError::DuplicatePoint {
                width: 2,
                depth: 2,
            })
        );
    }

    #[test]
    fn upsert_is_explicitly_available() {
        let mut surface =
            VolumeSurface::new(fidelity_spec())
                .expect("surface must be valid");

        surface
            .insert_measurement(2, 2, 0.95)
            .expect("first point must insert");

        surface
            .upsert(
                VolumePoint::new(
                    VolumeCoordinate::new(2, 2)
                        .expect("coordinate"),
                    0.99,
                    fidelity_spec(),
                )
                .expect("point"),
            )
            .expect("upsert must succeed");

        assert_eq!(
            surface
                .get_at(2, 2)
                .expect("point must exist")
                .quality,
            0.99
        );

        assert_eq!(surface.len(), 1);
    }

    #[test]
    fn point_limit_is_enforced() {
        let config = VolumeSurfaceConfig {
            max_width: 100,
            max_depth: 100,
            max_points: 2,
            reject_duplicates: true,
        };

        let mut surface =
            VolumeSurface::with_config(
                fidelity_spec(),
                config,
            )
            .expect("surface must be valid");

        surface
            .insert_measurement(1, 1, 0.95)
            .expect("point must insert");

        surface
            .insert_measurement(1, 2, 0.95)
            .expect("point must insert");

        assert!(matches!(
            surface.insert_measurement(
                1,
                3,
                0.95
            ),
            Err(VolumeError::PointLimitExceeded {
                current: 2,
                maximum: 2,
            })
        ));
    }

    #[test]
    fn best_point_uses_quality_direction() {
        let mut surface =
            VolumeSurface::new(fidelity_spec())
                .expect("surface must be valid");

        surface
            .insert_measurement(2, 2, 0.95)
            .expect("point");

        surface
            .insert_measurement(4, 4, 0.90)
            .expect("point");

        let best =
            surface.best_point()
                .expect("best point");

        assert_eq!(best.width(), 2);
        assert_eq!(best.depth(), 2);
        assert_eq!(best.quality, 0.95);
    }

    #[test]
    fn best_point_supports_lower_is_better_metrics() {
        let mut surface =
            VolumeSurface::new(error_spec())
                .expect("surface must be valid");

        surface
            .insert_measurement(2, 2, 0.05)
            .expect("point");

        surface
            .insert_measurement(4, 4, 0.08)
            .expect("point");

        let best =
            surface.best_point()
                .expect("best point");

        assert_eq!(best.width(), 2);
        assert_eq!(best.depth(), 2);
        assert_eq!(best.quality, 0.05);
    }

    #[test]
    fn largest_passing_square_dimension_is_correct() {
        let mut surface =
            VolumeSurface::new(fidelity_spec())
                .expect("surface must be valid");

        surface
            .insert_measurement(2, 2, 0.95)
            .expect("point");

        surface
            .insert_measurement(4, 4, 0.92)
            .expect("point");

        surface
            .insert_measurement(8, 8, 0.85)
            .expect("point");

        assert_eq!(
            surface.largest_passing_square_dimension(),
            Some(4)
        );
    }

    #[test]
    fn failing_points_are_not_counted_as_passing() {
        let mut surface =
            VolumeSurface::new(fidelity_spec())
                .expect("surface must be valid");

        surface
            .insert_measurement(2, 2, 0.95)
            .expect("point");

        surface
            .insert_measurement(4, 4, 0.85)
            .expect("point");

        assert_eq!(surface.passing_count(), 1);
        assert_eq!(surface.failing_count(), 1);
    }

    #[test]
    fn rectangular_completeness_is_detected() {
        let mut surface =
            VolumeSurface::new(fidelity_spec())
                .expect("surface must be valid");

        for width in 1..=2 {
            for depth in 1..=2 {
                surface
                    .insert_measurement(
                        width,
                        depth,
                        0.95,
                    )
                    .expect("point");
            }
        }

        assert!(
            surface
                .is_complete_rectangle(
                    1,
                    2,
                    1,
                    2,
                )
                .expect("query must succeed")
        );

        assert_eq!(
            surface
                .missing_coordinates(
                    1,
                    2,
                    1,
                    2,
                )
                .expect("query must succeed"),
            Vec::<VolumeCoordinate>::new()
        );
    }

    #[test]
    fn missing_coordinates_are_deterministic() {
        let mut surface =
            VolumeSurface::new(fidelity_spec())
                .expect("surface must be valid");

        surface
            .insert_measurement(1, 1, 0.95)
            .expect("point");

        let missing =
            surface
                .missing_coordinates(
                    1,
                    2,
                    1,
                    2,
                )
                .expect("query");

        assert_eq!(
            missing,
            vec![
                VolumeCoordinate {
                    width: 1,
                    depth: 2,
                },
                VolumeCoordinate {
                    width: 2,
                    depth: 1,
                },
                VolumeCoordinate {
                    width: 2,
                    depth: 2,
                },
            ]
        );
    }

    #[test]
    fn coverage_is_correct() {
        let mut surface =
            VolumeSurface::new(fidelity_spec())
                .expect("surface");

        surface
            .insert_measurement(1, 1, 0.95)
            .expect("point");

        surface
            .insert_measurement(1, 2, 0.95)
            .expect("point");

        assert_eq!(
            surface
                .coverage(1, 2, 1, 2)
                .expect("coverage"),
            0.5
        );
    }

    #[test]
    fn grid_size_is_correct() {
        let width =
            CoordinateRange::new(1, 4)
                .expect("range");

        let depth =
            CoordinateRange::new(1, 8)
                .expect("range");

        let grid =
            VolumeGrid::new(width, depth)
                .expect("grid");

        assert_eq!(
            grid.len().expect("length"),
            32
        );

        assert_eq!(
            grid.coordinates().count(),
            32
        );
    }

    #[test]
    fn grid_contains_coordinates() {
        let grid =
            VolumeGrid::new(
                CoordinateRange::new(2, 4)
                    .expect("range"),
                CoordinateRange::new(3, 5)
                    .expect("range"),
            )
            .expect("grid");

        assert!(
            grid.contains(
                VolumeCoordinate {
                    width: 3,
                    depth: 4,
                }
            )
        );

        assert!(
            !grid.contains(
                VolumeCoordinate {
                    width: 1,
                    depth: 4,
                }
            )
        );
    }

    #[test]
    fn grid_iteration_order_is_deterministic() {
        let grid =
            VolumeGrid::new(
                CoordinateRange::new(1, 2)
                    .expect("range"),
                CoordinateRange::new(1, 2)
                    .expect("range"),
            )
            .expect("grid");

        let coordinates: Vec<_> =
            grid.coordinates().collect();

        assert_eq!(
            coordinates,
            vec![
                VolumeCoordinate {
                    width: 1,
                    depth: 1,
                },
                VolumeCoordinate {
                    width: 1,
                    depth: 2,
                },
                VolumeCoordinate {
                    width: 2,
                    depth: 1,
                },
                VolumeCoordinate {
                    width: 2,
                    depth: 2,
                },
            ]
        );
    }

    #[test]
    fn pareto_frontier_removes_dominated_points() {
        let mut surface =
            VolumeSurface::new(fidelity_spec())
                .expect("surface");

        surface
            .insert_measurement(2, 2, 0.95)
            .expect("point");

        surface
            .insert_measurement(4, 4, 0.90)
            .expect("point");

        surface
            .insert_measurement(8, 8, 0.80)
            .expect("point");

        surface
            .insert_measurement(4, 8, 0.85)
            .expect("point");

        let frontier =
            surface.pareto_frontier();

        assert_eq!(frontier.len(), 3);

        assert!(
            frontier.iter().any(|point| {
                point.width() == 2
                    && point.depth() == 2
            })
        );

        assert!(
            frontier.iter().any(|point| {
                point.width() == 4
                    && point.depth() == 8
            })
        );

        assert!(
            frontier.iter().any(|point| {
                point.width() == 8
                    && point.depth() == 8
            })
        );
    }

    #[test]
    fn fingerprint_is_reproducible() {
        let mut first =
            VolumeSurface::new(fidelity_spec())
                .expect("surface");

        first
            .insert_measurement(4, 4, 0.95)
            .expect("point");

        first
            .insert_measurement(8, 8, 0.90)
            .expect("point");

        let mut second =
            VolumeSurface::new(fidelity_spec())
                .expect("surface");

        second
            .insert_measurement(8, 8, 0.90)
            .expect("point");

        second
            .insert_measurement(4, 4, 0.95)
            .expect("point");

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn changing_a_measurement_changes_fingerprint() {
        let mut first =
            VolumeSurface::new(fidelity_spec())
                .expect("surface");

        first
            .insert_measurement(4, 4, 0.95)
            .expect("point");

        let mut second =
            VolumeSurface::new(fidelity_spec())
                .expect("surface");

        second
            .insert_measurement(4, 4, 0.94)
            .expect("point");

        assert_ne!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn non_finite_measurements_are_rejected() {
        let mut surface =
            VolumeSurface::new(fidelity_spec())
                .expect("surface");

        assert_eq!(
            surface.insert_measurement(
                4,
                4,
                f64::NAN,
            ),
            Err(VolumeError::NonFiniteQuality)
        );

        assert_eq!(
            surface.insert_measurement(
                4,
                4,
                f64::INFINITY,
            ),
            Err(VolumeError::NonFiniteQuality)
        );
    }

    #[test]
    fn summary_contains_expected_values() {
        let mut surface =
            VolumeSurface::new(fidelity_spec())
                .expect("surface");

        surface
            .insert_measurement(2, 2, 0.95)
            .expect("point");

        surface
            .insert_measurement(8, 8, 0.85)
            .expect("point");

        let summary =
            surface.summary();

        assert_eq!(summary.point_count, 2);
        assert_eq!(summary.passing_count, 1);
        assert_eq!(summary.failing_count, 1);
        assert_eq!(summary.max_width, Some(8));
        assert_eq!(summary.max_depth, Some(8));
        assert_eq!(
            summary.max_square_dimension,
            Some(8)
        );
        assert_eq!(
            summary.best_quality,
            Some(0.95)
        );
        assert_eq!(
            summary.best_passing_quality,
            Some(0.95)
        );
        assert_ne!(summary.fingerprint, 0);
    }

    #[test]
    fn dimensions_are_stable() {
        assert_eq!(
            VOLUMETRIC_DIMENSION_COUNT,
            2
        );
    }

    #[test]
    fn quality_direction_identifiers_are_stable() {
        assert_eq!(
            QualityDirection::HigherIsBetter.as_str(),
            "higher_is_better"
        );

        assert_eq!(
            QualityDirection::LowerIsBetter.as_str(),
            "lower_is_better"
        );
    }

    #[test]
    fn quality_domain_identifiers_are_stable() {
        assert_eq!(
            QualityDomain::UnitInterval.as_str(),
            "unit_interval"
        );

        assert_eq!(
            QualityDomain::Finite.as_str(),
            "finite"
        );
    }

    #[test]
    fn volume_coordinate_order_is_deterministic() {
        let first =
            VolumeCoordinate {
                width: 2,
                depth: 4,
            };

        let second =
            VolumeCoordinate {
                width: 4,
                depth: 2,
            };

        assert!(first < second);
    }

    #[test]
    fn square_coordinate_is_detected() {
        let coordinate =
            VolumeCoordinate::new(8, 8)
                .expect("coordinate");

        assert!(coordinate.is_square());
        assert_eq!(
            coordinate.square_dimension(),
            8
        );
    }
}