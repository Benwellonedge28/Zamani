//! Zamani Quantum Benchmarking — Volumetric Performance Surface
//!
//! This module represents the measured performance surface of a volumetric
//! quantum benchmark over circuit width and circuit depth.
//!
//! # Purpose
//!
//! A volumetric benchmark evaluates quantum-computing performance over
//! rectangular circuit shapes:
//!
//! ```text
//!                    circuit depth
//!             1      2      3      4      5
//!          +------+------+------+------+------+
//! width 1  |  p   |  p   |  p   |  p   |  p   |
//!          +------+------+------+------+------+
//! width 2  |  p   |  p   |  p   |  p   |  p   |
//!          +------+------+------+------+------+
//! width 3  |  p   |  p   |  p   |  p   |  p   |
//!          +------+------+------+------+------+
//! width 4  |  p   |  p   |  p   |  p   |  p   |
//!          +------+------+------+------+------+
//! ```
//!
//! where `p` is the measured benchmark quality for that `(width, depth)`
//! point.
//!
//! This module owns the **data model and deterministic validation/access** of
//! that surface. It does NOT:
//!
//! - generate quantum circuits;
//! - execute circuits;
//! - select a backend;
//! - compile or transpile circuits;
//! - perform routing;
//! - perform scheduling;
//! - calculate Quantum Volume;
//! - fit statistical models;
//! - invent missing measurements;
//! - interpolate measurements;
//! - extrapolate measurements;
//! - draw charts;
//! - communicate with hardware;
//! - depend on a simulator;
//! - depend on the Quantum IR;
//! - depend on a hardware provider;
//! - maintain process-global state;
//! - silently discard invalid observations.
//!
//! Those responsibilities belong to the surrounding benchmarking architecture.
//!
//! # Architectural role
//!
//! The intended dependency direction is:
//!
//! ```text
//! Zamani Quantum IR
//!        │
//!        ▼
//! benchmark workload / circuit generation
//!        │
//!        ▼
//! benchmark execution
//!        │
//!        ▼
//! statistical analysis / metrics
//!        │
//!        ▼
//! volumetric::surface
//!        │
//!        ├──────────────► volumetric::frontier
//!        │
//!        ├──────────────► volumetric::positioning
//!        │
//!        └──────────────► reporting
//! ```
//!
//! `surface.rs` is therefore intentionally a leaf-level volumetric data
//! structure. It can be implemented and tested before the future volumetric
//! modules exist.
//!
//! # Relationship to Quantum Volume
//!
//! Quantum Volume is a square-circuit benchmark. Volumetric benchmarking
//! generalizes this idea to rectangular shapes where width and depth are
//! independently varied.
//!
//! Therefore:
//!
//! ```text
//! Quantum Volume:
//!     width == depth
//!
//! Volumetric surface:
//!     width and depth are independent
//! ```
//!
//! This module must never enforce `width == depth`.
//!
//! # Scientific semantics
//!
//! A surface point represents a measured or analyzed quality value for exactly
//! one `(width, depth)` shape.
//!
//! The quality value is deliberately represented as an opaque finite `f64`
//! rather than being restricted to fidelity or probability. Different
//! volumetric benchmarks may use:
//!
//! - fidelity;
//! - success probability;
//! - approximation ratio;
//! - normalized application quality;
//! - error rate;
//! - expectation-value accuracy;
//! - throughput;
//! - another explicitly documented scalar metric.
//!
//! The caller must define the meaning and unit of the quality value.
//!
//! `surface.rs` therefore does not assume that "higher is better" unless the
//! caller explicitly uses the supplied threshold-classification helpers.
//!
//! # Missing points
//!
//! A volumetric surface may legitimately be sparse. For example:
//!
//! ```text
//! width/depth      1   2   3   4   5
//!
//!       1          x   x   x   -   -
//!       2          x   x   x   x   -
//!       3          x   x   -   -   -
//!       4          x   -   -   -   -
//! ```
//!
//! `-` means "not measured", not zero and not failure.
//!
//! This distinction is critical. Missing observations must never be converted
//! into numerical zeroes.
//!
//! # Duplicate points
//!
//! The surface is a canonical representation of one analyzed value per
//! `(width, depth)` coordinate. Duplicate coordinates are rejected.
//!
//! If a benchmark has multiple raw observations at the same shape, aggregation
//! belongs in the statistical layer before constructing the final surface.
//!
//! # Determinism
//!
//! Points are returned in deterministic lexicographic order:
//!
//! ```text
//! width ascending
//! then depth ascending
//! ```
//!
//! No hash-map iteration order is exposed through the public API.
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
//! No external crates are required.
//!
//! # Integration contract
//!
//! Future modules may consume this type without changing this file:
//!
//! ```text
//! volumetric::volume
//!     -> VolumetricSurface::insert()
//!
//! statistics
//!     -> aggregated quality
//!     -> SurfacePoint
//!
//! volumetric::frontier
//!     -> Surface::points()
//!     -> Surface::classify()
//!
//! volumetric::positioning
//!     -> Surface::bounds()
//!     -> Surface::points()
//!
//! reporting
//!     -> Surface::points()
//!     -> Surface::dimensions()
//! ```
//!
//! The future `core::metric::Metric` type may wrap or reference a
//! `SurfacePoint`, but this module deliberately does not depend on that future
//! type. This prevents circular module dependencies and allows the file to be
//! completed independently.
//!
//! # Important non-responsibility
//!
//! This file does not calculate a Pareto frontier. The frontier is a derived
//! analytical concept and belongs in `volumetric/frontier.rs`.
//!
//! It also does not interpolate between measured points. Any interpolation or
//! statistical surface reconstruction must be explicit and owned by the
//! analysis layer so that measured and inferred values cannot be confused.

use std::error::Error;
use std::fmt;

// ============================================================================
// Public constants
// ============================================================================

/// Stable identifier for this representation.
pub const VOLUMETRIC_SURFACE_ID: &str = "volumetric_surface";

/// Version of the surface data contract.
///
/// This is independent from the Zamani compiler version and independent from
/// individual benchmark protocol versions.
pub const VOLUMETRIC_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Minimum valid circuit width.
pub const MIN_CIRCUIT_WIDTH: usize = 1;

/// Minimum valid circuit depth.
pub const MIN_CIRCUIT_DEPTH: usize = 1;

/// Numerical tolerance used only when checking values that should be within
/// the unit interval.
///
/// The surface itself does not require quality to be a probability. This
/// constant is used by `UnitIntervalQuality`.
pub const UNIT_INTERVAL_EPSILON: f64 = 1.0e-12;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the volumetric surface model.
#[derive(Debug, Clone, PartialEq)]
pub enum VolumetricSurfaceError {
    /// Circuit width was zero.
    InvalidWidth {
        width: usize,
    },

    /// Circuit depth was zero.
    InvalidDepth {
        depth: usize,
    },

    /// A quality value was NaN or infinite.
    NonFiniteQuality {
        value: f64,
    },

    /// A quality uncertainty was NaN or infinite.
    NonFiniteUncertainty {
        value: f64,
    },

    /// An uncertainty was negative.
    NegativeUncertainty {
        value: f64,
    },

    /// A lower quality bound was greater than the upper bound.
    InvalidQualityBounds {
        lower: f64,
        upper: f64,
    },

    /// A quality lower bound was non-finite.
    NonFiniteLowerBound {
        value: f64,
    },

    /// A quality upper bound was non-finite.
    NonFiniteUpperBound {
        value: f64,
    },

    /// A quality value was outside the unit interval where a unit-interval
    /// metric was explicitly requested.
    QualityOutsideUnitInterval {
        value: f64,
    },

    /// An uncertainty was supplied without a meaningful measurement.
    InvalidUncertaintyWithoutMeasurement,

    /// A duplicate `(width, depth)` point was inserted.
    DuplicatePoint {
        width: usize,
        depth: usize,
    },

    /// A requested point does not exist.
    PointNotFound {
        width: usize,
        depth: usize,
    },

    /// The supplied threshold was not finite.
    NonFiniteThreshold {
        threshold: f64,
    },

    /// A maximum width/depth limit was exceeded.
    DimensionLimitExceeded {
        dimension: SurfaceDimension,
        value: usize,
        maximum: usize,
    },

    /// A requested surface capacity would overflow.
    CapacityOverflow {
        width: usize,
        depth: usize,
    },
}

impl fmt::Display for VolumetricSurfaceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidWidth { width } => {
                write!(
                    formatter,
                    "volumetric circuit width must be at least {}, got {}",
                    MIN_CIRCUIT_WIDTH, width
                )
            }

            Self::InvalidDepth { depth } => {
                write!(
                    formatter,
                    "volumetric circuit depth must be at least {}, got {}",
                    MIN_CIRCUIT_DEPTH, depth
                )
            }

            Self::NonFiniteQuality { value } => {
                write!(
                    formatter,
                    "volumetric quality must be finite, got {}",
                    value
                )
            }

            Self::NonFiniteUncertainty { value } => {
                write!(
                    formatter,
                    "volumetric quality uncertainty must be finite, got {}",
                    value
                )
            }

            Self::NegativeUncertainty { value } => {
                write!(
                    formatter,
                    "volumetric quality uncertainty cannot be negative, got {}",
                    value
                )
            }

            Self::InvalidQualityBounds { lower, upper } => {
                write!(
                    formatter,
                    "volumetric quality bounds are invalid: lower {} > upper {}",
                    lower, upper
                )
            }

            Self::NonFiniteLowerBound { value } => {
                write!(
                    formatter,
                    "volumetric lower quality bound must be finite, got {}",
                    value
                )
            }

            Self::NonFiniteUpperBound { value } => {
                write!(
                    formatter,
                    "volumetric upper quality bound must be finite, got {}",
                    value
                )
            }

            Self::QualityOutsideUnitInterval { value } => {
                write!(
                    formatter,
                    "unit-interval quality must be in [0, 1], got {}",
                    value
                )
            }

            Self::InvalidUncertaintyWithoutMeasurement => {
                write!(
                    formatter,
                    "quality uncertainty requires a measured quality value"
                )
            }

            Self::DuplicatePoint { width, depth } => {
                write!(
                    formatter,
                    "volumetric surface already contains point ({}, {})",
                    width, depth
                )
            }

            Self::PointNotFound { width, depth } => {
                write!(
                    formatter,
                    "volumetric surface does not contain point ({}, {})",
                    width, depth
                )
            }

            Self::NonFiniteThreshold { threshold } => {
                write!(
                    formatter,
                    "volumetric classification threshold must be finite, got {}",
                    threshold
                )
            }

            Self::DimensionLimitExceeded {
                dimension,
                value,
                maximum,
            } => {
                write!(
                    formatter,
                    "{} value {} exceeds configured maximum {}",
                    dimension.as_str(),
                    value,
                    maximum
                )
            }

            Self::CapacityOverflow { width, depth } => {
                write!(
                    formatter,
                    "surface capacity for {} x {} dimensions overflows usize",
                    width, depth
                )
            }
        }
    }
}

impl Error for VolumetricSurfaceError {}

// ============================================================================
// Dimensions
// ============================================================================

/// Dimension represented by a volumetric surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceDimension {
    /// Circuit width / number of qubits.
    Width,

    /// Circuit depth.
    Depth,
}

impl SurfaceDimension {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Width => "width",
            Self::Depth => "depth",
        }
    }
}

impl fmt::Display for SurfaceDimension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Quality semantics
// ============================================================================

/// Direction in which a quality metric improves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityDirection {
    /// Larger values represent better performance.
    HigherIsBetter,

    /// Smaller values represent better performance.
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

    /// Determines whether a value meets or exceeds a threshold according to
    /// this metric's direction.
    pub fn meets_threshold(self, value: f64, threshold: f64) -> bool {
        match self {
            Self::HigherIsBetter => value >= threshold,
            Self::LowerIsBetter => value <= threshold,
        }
    }

    /// Returns the signed difference from a threshold.
    ///
    /// Positive means the point is on the acceptable side of the threshold.
    pub fn margin(self, value: f64, threshold: f64) -> f64 {
        match self {
            Self::HigherIsBetter => value - threshold,
            Self::LowerIsBetter => threshold - value,
        }
    }
}

// ============================================================================
// Optional quality bounds
// ============================================================================

/// Optional uncertainty/bounds attached to a measured quality value.
///
/// This structure deliberately stores the analyzed result rather than raw
/// observations. Statistical calculations belong to `statistics/*`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QualityBounds {
    /// Lower confidence/uncertainty bound.
    pub lower: f64,

    /// Upper confidence/uncertainty bound.
    pub upper: f64,
}

impl QualityBounds {
    /// Creates validated bounds.
    pub fn new(
        lower: f64,
        upper: f64,
    ) -> Result<Self, VolumetricSurfaceError> {
        if !lower.is_finite() {
            return Err(
                VolumetricSurfaceError::NonFiniteLowerBound {
                    value: lower,
                },
            );
        }

        if !upper.is_finite() {
            return Err(
                VolumetricSurfaceError::NonFiniteUpperBound {
                    value: upper,
                },
            );
        }

        if lower > upper {
            return Err(
                VolumetricSurfaceError::InvalidQualityBounds {
                    lower,
                    upper,
                },
            );
        }

        Ok(Self { lower, upper })
    }

    /// Width of the uncertainty interval.
    pub fn width(&self) -> f64 {
        self.upper - self.lower
    }

    /// Returns whether the interval is entirely on the acceptable side of a
    /// threshold.
    pub fn meets_threshold(
        &self,
        direction: QualityDirection,
        threshold: f64,
    ) -> Result<bool, VolumetricSurfaceError> {
        if !threshold.is_finite() {
            return Err(
                VolumetricSurfaceError::NonFiniteThreshold {
                    threshold,
                },
            );
        }

        Ok(match direction {
            QualityDirection::HigherIsBetter => self.lower >= threshold,
            QualityDirection::LowerIsBetter => self.upper <= threshold,
        })
    }
}

// ============================================================================
// Surface point
// ============================================================================

/// One analyzed observation on a volumetric performance surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfacePoint {
    /// Circuit width / number of qubits.
    pub width: usize,

    /// Circuit depth.
    pub depth: usize,

    /// Measured/analyzed quality.
    pub quality: f64,

    /// Optional uncertainty interval.
    ///
    /// This is normally a confidence interval produced by the statistical
    /// layer. The surface does not infer it.
    pub bounds: Option<QualityBounds>,

    /// Number of raw observations contributing to this point, when known.
    pub sample_count: Option<usize>,

    /// Number of circuits contributing to this point, when known.
    pub circuit_count: Option<usize>,
}

impl SurfacePoint {
    /// Creates a point with no uncertainty metadata.
    pub fn new(
        width: usize,
        depth: usize,
        quality: f64,
    ) -> Result<Self, VolumetricSurfaceError> {
        validate_dimensions(width, depth)?;
        validate_quality(quality)?;

        Ok(Self {
            width,
            depth,
            quality,
            bounds: None,
            sample_count: None,
            circuit_count: None,
        })
    }

    /// Creates a point with an explicit uncertainty interval.
    pub fn with_bounds(
        width: usize,
        depth: usize,
        quality: f64,
        bounds: QualityBounds,
    ) -> Result<Self, VolumetricSurfaceError> {
        validate_dimensions(width, depth)?;
        validate_quality(quality)?;

        if quality < bounds.lower || quality > bounds.upper {
            return Err(
                VolumetricSurfaceError::InvalidQualityBounds {
                    lower: bounds.lower,
                    upper: bounds.upper,
                },
            );
        }

        Ok(Self {
            width,
            depth,
            quality,
            bounds: Some(bounds),
            sample_count: None,
            circuit_count: None,
        })
    }

    /// Adds sample-count metadata.
    pub fn with_sample_count(
        mut self,
        sample_count: usize,
    ) -> Self {
        self.sample_count = Some(sample_count);
        self
    }

    /// Adds circuit-count metadata.
    pub fn with_circuit_count(
        mut self,
        circuit_count: usize,
    ) -> Self {
        self.circuit_count = Some(circuit_count);
        self
    }

    /// Returns the circuit shape as `(width, depth)`.
    pub const fn shape(&self) -> (usize, usize) {
        (self.width, self.depth)
    }

    /// Returns the total logical circuit-volume coordinate.
    ///
    /// This is only the rectangular coordinate `width * depth`; it is not a
    /// universal physical resource count.
    pub fn rectangular_volume(&self) -> usize {
        self.width.saturating_mul(self.depth)
    }

    /// Returns the uncertainty interval width when available.
    pub fn uncertainty_width(&self) -> Option<f64> {
        self.bounds.map(|bounds| bounds.width())
    }

    /// Classifies this point against a threshold.
    ///
    /// This uses the point estimate. For statistically conservative decisions,
    /// prefer `classifies_conservatively()`.
    pub fn classifies(
        &self,
        direction: QualityDirection,
        threshold: f64,
    ) -> Result<bool, VolumetricSurfaceError> {
        if !threshold.is_finite() {
            return Err(
                VolumetricSurfaceError::NonFiniteThreshold {
                    threshold,
                },
            );
        }

        Ok(direction.meets_threshold(
            self.quality,
            threshold,
        ))
    }

    /// Classifies this point using the entire uncertainty interval when one is
    /// available.
    ///
    /// If no bounds are available, this falls back to the point estimate.
    pub fn classifies_conservatively(
        &self,
        direction: QualityDirection,
        threshold: f64,
    ) -> Result<bool, VolumetricSurfaceError> {
        if !threshold.is_finite() {
            return Err(
                VolumetricSurfaceError::NonFiniteThreshold {
                    threshold,
                },
            );
        }

        match self.bounds {
            Some(bounds) => {
                bounds.meets_threshold(
                    direction,
                    threshold,
                )
            }

            None => Ok(direction.meets_threshold(
                self.quality,
                threshold,
            )),
        }
    }
}

// ============================================================================
// Unit-interval helper
// ============================================================================

/// A quality value explicitly known to be a probability/fidelity-like value in
/// [0, 1].
///
/// This is intentionally separate from `SurfacePoint`, because not every
/// volumetric benchmark metric is a probability.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnitIntervalQuality {
    value: f64,
}

impl UnitIntervalQuality {
    /// Creates a validated unit-interval quality.
    pub fn new(
        value: f64,
    ) -> Result<Self, VolumetricSurfaceError> {
        if !value.is_finite()
            || value < -UNIT_INTERVAL_EPSILON
            || value > 1.0 + UNIT_INTERVAL_EPSILON
        {
            return Err(
                VolumetricSurfaceError::QualityOutsideUnitInterval {
                    value,
                },
            );
        }

        Ok(Self {
            value: value.clamp(0.0, 1.0),
        })
    }

    /// Returns the validated value.
    pub const fn value(self) -> f64 {
        self.value
    }
}

// ============================================================================
// Surface limits
// ============================================================================

/// Resource limits for a surface.
///
/// These limits are intentionally independent from the future global
/// `benchmarking::core::limits` module. That module can translate its global
/// limits into this structure without requiring changes here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SurfaceLimits {
    /// Maximum allowed width.
    pub max_width: usize,

    /// Maximum allowed depth.
    pub max_depth: usize,

    /// Maximum number of stored points.
    pub max_points: usize,
}

impl SurfaceLimits {
    /// Creates validated limits.
    pub fn new(
        max_width: usize,
        max_depth: usize,
        max_points: usize,
    ) -> Result<Self, VolumetricSurfaceError> {
        if max_width == 0 {
            return Err(
                VolumetricSurfaceError::InvalidWidth {
                    width: max_width,
                },
            );
        }

        if max_depth == 0 {
            return Err(
                VolumetricSurfaceError::InvalidDepth {
                    depth: max_depth,
                },
            );
        }

        Ok(Self {
            max_width,
            max_depth,
            max_points,
        })
    }

    /// Production-oriented conservative default.
    ///
    /// This is deliberately finite so malformed or untrusted benchmark
    /// requests cannot force an unbounded allocation.
    pub const fn production() -> Self {
        Self {
            max_width: 1_000_000,
            max_depth: 1_000_000,
            max_points: 10_000_000,
        }
    }

    fn validate_point(
        &self,
        point: &SurfacePoint,
    ) -> Result<(), VolumetricSurfaceError> {
        if point.width > self.max_width {
            return Err(
                VolumetricSurfaceError::DimensionLimitExceeded {
                    dimension: SurfaceDimension::Width,
                    value: point.width,
                    maximum: self.max_width,
                },
            );
        }

        if point.depth > self.max_depth {
            return Err(
                VolumetricSurfaceError::DimensionLimitExceeded {
                    dimension: SurfaceDimension::Depth,
                    value: point.depth,
                    maximum: self.max_depth,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Surface
// ============================================================================

/// Sparse deterministic volumetric performance surface.
///
/// Internally points are kept in a `Vec` rather than a hash map. This gives:
///
/// - deterministic iteration;
/// - no hash-seed dependence;
/// - no hidden hashing behavior;
/// - straightforward serialization;
/// - stable scientific output;
/// - no external dependencies.
///
/// Insertions are checked for duplicate coordinates.
///
/// For the very large surfaces produced by future high-throughput benchmark
/// systems, a specialized indexed representation can be introduced behind
/// this API without changing the public semantics.
#[derive(Debug, Clone, PartialEq)]
pub struct VolumetricSurface {
    /// Surface schema version.
    schema_version: u32,

    /// Stable identifier.
    benchmark_surface_id: &'static str,

    /// Meaning/direction of the quality metric.
    quality_direction: QualityDirection,

    /// Optional human/machine-readable quality metric identifier.
    metric_id: Option<String>,

    /// Optional metric unit.
    metric_unit: Option<String>,

    /// Resource limits.
    limits: SurfaceLimits,

    /// Canonically ordered points.
    points: Vec<SurfacePoint>,
}

impl VolumetricSurface {
    /// Creates an empty surface using production limits.
    pub fn new(
        quality_direction: QualityDirection,
    ) -> Self {
        Self::with_limits(
            quality_direction,
            SurfaceLimits::production(),
        )
    }

    /// Creates an empty surface with explicit limits.
    pub fn with_limits(
        quality_direction: QualityDirection,
        limits: SurfaceLimits,
    ) -> Self {
        Self {
            schema_version: VOLUMETRIC_SURFACE_SCHEMA_VERSION,
            benchmark_surface_id: VOLUMETRIC_SURFACE_ID,
            quality_direction,
            metric_id: None,
            metric_unit: None,
            limits,
            points: Vec::new(),
        }
    }

    /// Creates an empty surface with explicit metric metadata.
    pub fn with_metric(
        quality_direction: QualityDirection,
        metric_id: impl Into<String>,
        metric_unit: Option<impl Into<String>>,
    ) -> Self {
        let mut surface = Self::new(quality_direction);

        surface.metric_id = Some(metric_id.into());
        surface.metric_unit =
            metric_unit.map(Into::into);

        surface
    }

    /// Returns the schema version.
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the stable surface identifier.
    pub const fn benchmark_surface_id(&self) -> &'static str {
        self.benchmark_surface_id
    }

    /// Returns the quality direction.
    pub const fn quality_direction(
        &self,
    ) -> QualityDirection {
        self.quality_direction
    }

    /// Returns the metric identifier.
    pub fn metric_id(&self) -> Option<&str> {
        self.metric_id.as_deref()
    }

    /// Returns the metric unit.
    pub fn metric_unit(&self) -> Option<&str> {
        self.metric_unit.as_deref()
    }

    /// Returns the configured limits.
    pub const fn limits(&self) -> SurfaceLimits {
        self.limits
    }

    /// Returns the number of measured points.
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Returns whether the surface contains no points.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Returns all points in deterministic `(width, depth)` order.
    pub fn points(&self) -> &[SurfacePoint] {
        &self.points
    }

    /// Inserts a point.
    ///
    /// The point becomes part of the canonical surface only if:
    ///
    /// - dimensions are valid;
    /// - quality is finite;
    /// - uncertainty metadata is valid;
    /// - dimensions are within limits;
    /// - no point with the same `(width, depth)` already exists;
    /// - the point-count limit is not exceeded.
    pub fn insert(
        &mut self,
        point: SurfacePoint,
    ) -> Result<(), VolumetricSurfaceError> {
        self.limits.validate_point(&point)?;

        if self.points.len() >= self.limits.max_points {
            return Err(
                VolumetricSurfaceError::DimensionLimitExceeded {
                    dimension: SurfaceDimension::Width,
                    value: self.points.len().saturating_add(1),
                    maximum: self.limits.max_points,
                },
            );
        }

        if self.contains(point.width, point.depth) {
            return Err(
                VolumetricSurfaceError::DuplicatePoint {
                    width: point.width,
                    depth: point.depth,
                },
            );
        }

        let insertion_index =
            self.insertion_index(point.width, point.depth);

        self.points
            .insert(insertion_index, point);

        Ok(())
    }

    /// Inserts a point from primitive values.
    pub fn insert_value(
        &mut self,
        width: usize,
        depth: usize,
        quality: f64,
    ) -> Result<(), VolumetricSurfaceError> {
        self.insert(SurfacePoint::new(
            width,
            depth,
            quality,
        )?)
    }

    /// Inserts a point with uncertainty bounds.
    pub fn insert_with_bounds(
        &mut self,
        width: usize,
        depth: usize,
        quality: f64,
        bounds: QualityBounds,
    ) -> Result<(), VolumetricSurfaceError> {
        self.insert(SurfacePoint::with_bounds(
            width,
            depth,
            quality,
            bounds,
        )?)
    }

    /// Returns whether a coordinate has been measured.
    pub fn contains(
        &self,
        width: usize,
        depth: usize,
    ) -> bool {
        self.find_index(width, depth).is_some()
    }

    /// Returns a point if it exists.
    pub fn get(
        &self,
        width: usize,
        depth: usize,
    ) -> Option<&SurfacePoint> {
        self.find_index(width, depth)
            .map(|index| &self.points[index])
    }

    /// Returns a point or a structured error.
    pub fn require(
        &self,
        width: usize,
        depth: usize,
    ) -> Result<&SurfacePoint, VolumetricSurfaceError> {
        self.get(width, depth).ok_or(
            VolumetricSurfaceError::PointNotFound {
                width,
                depth,
            },
        )
    }

    /// Returns the smallest measured width.
    pub fn min_width(&self) -> Option<usize> {
        self.points.first().map(|point| point.width)
    }

    /// Returns the largest measured width.
    pub fn max_width(&self) -> Option<usize> {
        self.points
            .iter()
            .map(|point| point.width)
            .max()
    }

    /// Returns the smallest measured depth.
    pub fn min_depth(&self) -> Option<usize> {
        self.points
            .iter()
            .map(|point| point.depth)
            .min()
    }

    /// Returns the largest measured depth.
    pub fn max_depth(&self) -> Option<usize> {
        self.points
            .iter()
            .map(|point| point.depth)
            .max()
    }

    /// Returns `(min_width, max_width, min_depth, max_depth)`.
    pub fn bounds(
        &self,
    ) -> Option<(usize, usize, usize, usize)> {
        if self.is_empty() {
            return None;
        }

        Some((
            self.min_width()?,
            self.max_width()?,
            self.min_depth()?,
            self.max_depth()?,
        ))
    }

    /// Returns all points at one circuit width.
    ///
    /// Points are returned in ascending depth order because the surface itself
    /// is maintained in canonical order.
    pub fn points_at_width(
        &self,
        width: usize,
    ) -> Vec<&SurfacePoint> {
        self.points
            .iter()
            .filter(|point| point.width == width)
            .collect()
    }

    /// Returns all points at one circuit depth.
    ///
    /// Points are returned in ascending width order.
    pub fn points_at_depth(
        &self,
        depth: usize,
    ) -> Vec<&SurfacePoint> {
        self.points
            .iter()
            .filter(|point| point.depth == depth)
            .collect()
    }

    /// Returns all points satisfying the threshold according to the point
    /// estimate.
    ///
    /// This does not perform interpolation.
    pub fn passing_points(
        &self,
        threshold: f64,
    ) -> Result<Vec<&SurfacePoint>, VolumetricSurfaceError> {
        if !threshold.is_finite() {
            return Err(
                VolumetricSurfaceError::NonFiniteThreshold {
                    threshold,
                },
            );
        }

        Ok(self
            .points
            .iter()
            .filter(|point| {
                self.quality_direction
                    .meets_threshold(
                        point.quality,
                        threshold,
                    )
            })
            .collect())
    }

    /// Returns all points satisfying the threshold conservatively.
    ///
    /// When a confidence interval is present, the entire interval must be on
    /// the acceptable side of the threshold.
    ///
    /// When no interval is present, the point estimate is used.
    pub fn conservatively_passing_points(
        &self,
        threshold: f64,
    ) -> Result<Vec<&SurfacePoint>, VolumetricSurfaceError> {
        if !threshold.is_finite() {
            return Err(
                VolumetricSurfaceError::NonFiniteThreshold {
                    threshold,
                },
            );
        }

        self.points
            .iter()
            .map(|point| {
                point
                    .classifies_conservatively(
                        self.quality_direction,
                        threshold,
                    )
                    .map(|passes| {
                        if passes {
                            Some(point)
                        } else {
                            None
                        }
                    })
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|points| {
                points
                    .into_iter()
                    .flatten()
                    .collect()
            })
    }

    /// Returns the best measured point according to the configured quality
    /// direction.
    ///
    /// Ties are resolved deterministically by:
    ///
    /// 1. better quality;
    /// 2. smaller rectangular volume;
    /// 3. smaller width;
    /// 4. smaller depth.
    pub fn best_point(&self) -> Option<&SurfacePoint> {
        let mut best: Option<&SurfacePoint> = None;

        for candidate in &self.points {
            best = match best {
                None => Some(candidate),

                Some(current) => {
                    if is_better(
                        self.quality_direction,
                        candidate,
                        current,
                    ) {
                        Some(candidate)
                    } else {
                        Some(current)
                    }
                }
            };
        }

        best
    }

    /// Returns the point with the greatest width that passes a threshold.
    ///
    /// This is intentionally a simple surface query. It is NOT the complete
    /// volumetric Pareto frontier.
    pub fn widest_passing_point(
        &self,
        threshold: f64,
    ) -> Result<Option<&SurfacePoint>, VolumetricSurfaceError> {
        let passing =
            self.passing_points(threshold)?;

        Ok(passing.into_iter().max_by(|left, right| {
            left.width
                .cmp(&right.width)
                .then_with(|| {
                    left.depth.cmp(&right.depth)
                })
                .then_with(|| {
                    compare_quality(
                        self.quality_direction,
                        left.quality,
                        right.quality,
                    )
                })
        }))
    }

    /// Returns the deepest measured point that passes a threshold.
    pub fn deepest_passing_point(
        &self,
        threshold: f64,
    ) -> Result<Option<&SurfacePoint>, VolumetricSurfaceError> {
        let passing =
            self.passing_points(threshold)?;

        Ok(passing.into_iter().max_by(|left, right| {
            left.depth
                .cmp(&right.depth)
                .then_with(|| {
                    left.width.cmp(&right.width)
                })
                .then_with(|| {
                    compare_quality(
                        self.quality_direction,
                        left.quality,
                        right.quality,
                    )
                })
        }))
    }

    /// Returns whether every coordinate in the supplied rectangle has been
    /// measured.
    ///
    /// This method does not require the rectangle to start at `(1, 1)`.
    ///
    /// Missing points remain missing; this method never fabricates values.
    pub fn is_complete_rectangle(
        &self,
        min_width: usize,
        max_width: usize,
        min_depth: usize,
        max_depth: usize,
    ) -> Result<bool, VolumetricSurfaceError> {
        validate_dimensions(
            min_width,
            min_depth,
        )?;
        validate_dimensions(
            max_width,
            max_depth,
        )?;

        if min_width > max_width
            || min_depth > max_depth
        {
            return Ok(false);
        }

        for width in min_width..=max_width {
            for depth in min_depth..=max_depth {
                if !self.contains(width, depth) {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    /// Returns the number of missing cells in a rectangle.
    ///
    /// This is useful for reporting surface coverage without confusing
    /// missing cells with failed benchmark results.
    pub fn missing_cells(
        &self,
        min_width: usize,
        max_width: usize,
        min_depth: usize,
        max_depth: usize,
    ) -> Result<usize, VolumetricSurfaceError> {
        validate_rectangle(
            min_width,
            max_width,
            min_depth,
            max_depth,
        )?;

        let width_count =
            max_width - min_width + 1;

        let depth_count =
            max_depth - min_depth + 1;

        let total =
            width_count
                .checked_mul(depth_count)
                .ok_or(
                    VolumetricSurfaceError::CapacityOverflow {
                        width: width_count,
                        depth: depth_count,
                    },
                )?;

        let measured = self
            .points
            .iter()
            .filter(|point| {
                point.width >= min_width
                    && point.width <= max_width
                    && point.depth >= min_depth
                    && point.depth <= max_depth
            })
            .count();

        Ok(total.saturating_sub(measured))
    }

    /// Returns the number of points in the supplied rectangle.
    pub fn measured_cells(
        &self,
        min_width: usize,
        max_width: usize,
        min_depth: usize,
        max_depth: usize,
    ) -> Result<usize, VolumetricSurfaceError> {
        validate_rectangle(
            min_width,
            max_width,
            min_depth,
            max_depth,
        )?;

        Ok(self
            .points
            .iter()
            .filter(|point| {
                point.width >= min_width
                    && point.width <= max_width
                    && point.depth >= min_depth
                    && point.depth <= max_depth
            })
            .count())
    }

    /// Returns the fraction of measured cells in a rectangle.
    ///
    /// No inference is performed.
    pub fn coverage(
        &self,
        min_width: usize,
        max_width: usize,
        min_depth: usize,
        max_depth: usize,
    ) -> Result<f64, VolumetricSurfaceError> {
        validate_rectangle(
            min_width,
            max_width,
            min_depth,
            max_depth,
        )?;

        let width_count =
            max_width - min_width + 1;

        let depth_count =
            max_depth - min_depth + 1;

        let total =
            width_count
                .checked_mul(depth_count)
                .ok_or(
                    VolumetricSurfaceError::CapacityOverflow {
                        width: width_count,
                        depth: depth_count,
                    },
                )?;

        let measured = self.measured_cells(
            min_width,
            max_width,
            min_depth,
            max_depth,
        )?;

        Ok(measured as f64 / total as f64)
    }

    /// Returns a deterministic copy of the surface points.
    ///
    /// This is useful for serializers that need owned data.
    pub fn to_points(&self) -> Vec<SurfacePoint> {
        self.points.clone()
    }

    /// Validates the complete surface.
    ///
    /// This method is useful at integration boundaries before a result is
    /// serialized or passed to another subsystem.
    pub fn validate(&self) -> Result<(), VolumetricSurfaceError> {
        if self.schema_version
            != VOLUMETRIC_SURFACE_SCHEMA_VERSION
        {
            // The current type cannot be constructed with another schema
            // version through its public API, so this branch intentionally
            // does not exist as a separate error.
        }

        if self.points.len() > self.limits.max_points {
            return Err(
                VolumetricSurfaceError::DimensionLimitExceeded {
                    dimension: SurfaceDimension::Width,
                    value: self.points.len(),
                    maximum: self.limits.max_points,
                },
            );
        }

        for pair in self.points.windows(2) {
            let previous = pair[0];
            let current = pair[1];

            if compare_coordinates(
                previous.width,
                previous.depth,
                current.width,
                current.depth,
            ) == std::cmp::Ordering::Greater
            {
                return Err(
                    VolumetricSurfaceError::DuplicatePoint {
                        width: current.width,
                        depth: current.depth,
                    },
                );
            }

            if previous.width == current.width
                && previous.depth == current.depth
            {
                return Err(
                    VolumetricSurfaceError::DuplicatePoint {
                        width: current.width,
                        depth: current.depth,
                    },
                );
            }
        }

        for point in &self.points {
            validate_dimensions(
                point.width,
                point.depth,
            )?;
            validate_quality(point.quality)?;
            self.limits.validate_point(point)?;

            if let Some(bounds) = point.bounds {
                QualityBounds::new(
                    bounds.lower,
                    bounds.upper,
                )?;

                if point.quality < bounds.lower
                    || point.quality > bounds.upper
                {
                    return Err(
                        VolumetricSurfaceError::InvalidQualityBounds {
                            lower: bounds.lower,
                            upper: bounds.upper,
                        },
                    );
                }
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Internal indexing
    // ------------------------------------------------------------------------

    fn find_index(
        &self,
        width: usize,
        depth: usize,
    ) -> Option<usize> {
        self.points
            .binary_search_by(|point| {
                compare_coordinates(
                    point.width,
                    point.depth,
                    width,
                    depth,
                )
            })
            .ok()
    }

    fn insertion_index(
        &self,
        width: usize,
        depth: usize,
    ) -> usize {
        match self.points.binary_search_by(|point| {
            compare_coordinates(
                point.width,
                point.depth,
                width,
                depth,
            )
        }) {
            Ok(index) => index,
            Err(index) => index,
        }
    }
}

// ============================================================================
// Free validation helpers
// ============================================================================

fn validate_dimensions(
    width: usize,
    depth: usize,
) -> Result<(), VolumetricSurfaceError> {
    if width < MIN_CIRCUIT_WIDTH {
        return Err(
            VolumetricSurfaceError::InvalidWidth {
                width,
            },
        );
    }

    if depth < MIN_CIRCUIT_DEPTH {
        return Err(
            VolumetricSurfaceError::InvalidDepth {
                depth,
            },
        );
    }

    Ok(())
}

fn validate_quality(
    quality: f64,
) -> Result<(), VolumetricSurfaceError> {
    if !quality.is_finite() {
        return Err(
            VolumetricSurfaceError::NonFiniteQuality {
                value: quality,
            },
        );
    }

    Ok(())
}

fn validate_rectangle(
    min_width: usize,
    max_width: usize,
    min_depth: usize,
    max_depth: usize,
) -> Result<(), VolumetricSurfaceError> {
    validate_dimensions(
        min_width,
        min_depth,
    )?;

    validate_dimensions(
        max_width,
        max_depth,
    )?;

    Ok(())
}

fn compare_coordinates(
    left_width: usize,
    left_depth: usize,
    right_width: usize,
    right_depth: usize,
) -> std::cmp::Ordering {
    left_width
        .cmp(&right_width)
        .then_with(|| {
            left_depth.cmp(&right_depth)
        })
}

fn compare_quality(
    direction: QualityDirection,
    left: f64,
    right: f64,
) -> std::cmp::Ordering {
    match direction {
        QualityDirection::HigherIsBetter => {
            left.partial_cmp(&right)
                .unwrap_or(std::cmp::Ordering::Equal)
        }

        QualityDirection::LowerIsBetter => {
            right.partial_cmp(&left)
                .unwrap_or(std::cmp::Ordering::Equal)
        }
    }
}

fn is_better(
    direction: QualityDirection,
    candidate: &SurfacePoint,
    current: &SurfacePoint,
) -> bool {
    match compare_quality(
        direction,
        candidate.quality,
        current.quality,
    ) {
        std::cmp::Ordering::Greater => true,

        std::cmp::Ordering::Less => false,

        std::cmp::Ordering::Equal => {
            candidate
                .rectangular_volume()
                .cmp(&current.rectangular_volume())
                == std::cmp::Ordering::Less
                || (
                    candidate.rectangular_volume()
                        == current.rectangular_volume()
                        && compare_coordinates(
                            candidate.width,
                            candidate.depth,
                            current.width,
                            current.depth,
                        ) == std::cmp::Ordering::Less
                )
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_requires_positive_dimensions() {
        assert!(matches!(
            SurfacePoint::new(0, 1, 0.5),
            Err(
                VolumetricSurfaceError::InvalidWidth {
                    width: 0
                }
            )
        ));

        assert!(matches!(
            SurfacePoint::new(1, 0, 0.5),
            Err(
                VolumetricSurfaceError::InvalidDepth {
                    depth: 0
                }
            )
        ));
    }

    #[test]
    fn point_rejects_non_finite_quality() {
        assert!(matches!(
            SurfacePoint::new(1, 1, f64::NAN),
            Err(
                VolumetricSurfaceError::NonFiniteQuality {
                    ..
                }
            )
        ));

        assert!(matches!(
            SurfacePoint::new(1, 1, f64::INFINITY),
            Err(
                VolumetricSurfaceError::NonFiniteQuality {
                    ..
                }
            )
        ));
    }

    #[test]
    fn bounds_are_validated() {
        assert!(QualityBounds::new(
            0.8,
            0.9
        )
        .is_ok());

        assert!(matches!(
            QualityBounds::new(0.9, 0.8),
            Err(
                VolumetricSurfaceError::InvalidQualityBounds {
                    ..
                }
            )
        ));
    }

    #[test]
    fn point_quality_must_be_inside_bounds() {
        let bounds =
            QualityBounds::new(0.8, 0.9)
                .expect("valid bounds");

        assert!(SurfacePoint::with_bounds(
            2,
            3,
            0.85,
            bounds
        )
        .is_ok());

        assert!(matches!(
            SurfacePoint::with_bounds(
                2,
                3,
                0.95,
                bounds
            ),
            Err(
                VolumetricSurfaceError::InvalidQualityBounds {
                    ..
                }
            )
        ));
    }

    #[test]
    fn unit_interval_quality_is_validated() {
        assert_eq!(
            UnitIntervalQuality::new(0.5)
                .expect("valid probability")
                .value(),
            0.5
        );

        assert!(UnitIntervalQuality::new(-0.1).is_err());
        assert!(UnitIntervalQuality::new(1.1).is_err());
        assert!(UnitIntervalQuality::new(f64::NAN).is_err());
    }

    #[test]
    fn surface_inserts_points_in_deterministic_order() {
        let mut surface =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        surface
            .insert_value(3, 2, 0.8)
            .expect("insert");

        surface
            .insert_value(1, 4, 0.7)
            .expect("insert");

        surface
            .insert_value(1, 2, 0.9)
            .expect("insert");

        surface
            .insert_value(2, 1, 0.6)
            .expect("insert");

        let shapes: Vec<(usize, usize)> =
            surface
                .points()
                .iter()
                .map(|point| point.shape())
                .collect();

        assert_eq!(
            shapes,
            vec![
                (1, 2),
                (1, 4),
                (2, 1),
                (3, 2),
            ]
        );
    }

    #[test]
    fn duplicate_points_are_rejected() {
        let mut surface =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        surface
            .insert_value(2, 2, 0.8)
            .expect("first insertion");

        assert!(matches!(
            surface.insert_value(
                2,
                2,
                0.9
            ),
            Err(
                VolumetricSurfaceError::DuplicatePoint {
                    width: 2,
                    depth: 2
                }
            )
        ));
    }

    #[test]
    fn lookup_is_deterministic() {
        let mut surface =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        surface
            .insert_value(4, 7, 0.91)
            .expect("insert");

        assert_eq!(
            surface
                .get(4, 7)
                .expect("point")
                .quality,
            0.91
        );

        assert!(surface.get(1, 1).is_none());
    }

    #[test]
    fn bounds_are_reported() {
        let mut surface =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        surface
            .insert_value(2, 7, 0.8)
            .expect("insert");

        surface
            .insert_value(5, 3, 0.9)
            .expect("insert");

        assert_eq!(
            surface.bounds(),
            Some((2, 5, 3, 7))
        );
    }

    #[test]
    fn points_at_width_are_depth_sorted() {
        let mut surface =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        surface
            .insert_value(3, 8, 0.8)
            .expect("insert");

        surface
            .insert_value(3, 2, 0.9)
            .expect("insert");

        surface
            .insert_value(3, 5, 0.7)
            .expect("insert");

        let depths: Vec<usize> =
            surface
                .points_at_width(3)
                .iter()
                .map(|point| point.depth)
                .collect();

        assert_eq!(depths, vec![2, 5, 8]);
    }

    #[test]
    fn points_at_depth_are_width_sorted() {
        let mut surface =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        surface
            .insert_value(5, 3, 0.8)
            .expect("insert");

        surface
            .insert_value(2, 3, 0.9)
            .expect("insert");

        surface
            .insert_value(4, 3, 0.7)
            .expect("insert");

        let widths: Vec<usize> =
            surface
                .points_at_depth(3)
                .iter()
                .map(|point| point.width)
                .collect();

        assert_eq!(widths, vec![2, 4, 5]);
    }

    #[test]
    fn higher_is_better_threshold_works() {
        let mut surface =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        surface
            .insert_value(2, 2, 0.8)
            .expect("insert");

        surface
            .insert_value(3, 3, 0.6)
            .expect("insert");

        let passing =
            surface
                .passing_points(0.7)
                .expect("threshold");

        assert_eq!(passing.len(), 1);
        assert_eq!(passing[0].shape(), (2, 2));
    }

    #[test]
    fn lower_is_better_threshold_works() {
        let mut surface =
            VolumetricSurface::new(
                QualityDirection::LowerIsBetter,
            );

        surface
            .insert_value(2, 2, 0.1)
            .expect("insert");

        surface
            .insert_value(3, 3, 0.4)
            .expect("insert");

        let passing =
            surface
                .passing_points(0.2)
                .expect("threshold");

        assert_eq!(passing.len(), 1);
        assert_eq!(passing[0].shape(), (2, 2));
    }

    #[test]
    fn conservative_threshold_uses_lower_bound_for_higher_is_better() {
        let mut surface =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        surface
            .insert_with_bounds(
                2,
                2,
                0.85,
                QualityBounds::new(
                    0.80,
                    0.90,
                )
                .expect("bounds"),
            )
            .expect("insert");

        surface
            .insert_with_bounds(
                3,
                3,
                0.85,
                QualityBounds::new(
                    0.60,
                    0.95,
                )
                .expect("bounds"),
            )
            .expect("insert");

        let passing = surface
            .conservatively_passing_points(0.75)
            .expect("threshold");

        assert_eq!(passing.len(), 1);
        assert_eq!(passing[0].shape(), (2, 2));
    }

    #[test]
    fn conservative_threshold_uses_upper_bound_for_lower_is_better() {
        let mut surface =
            VolumetricSurface::new(
                QualityDirection::LowerIsBetter,
            );

        surface
            .insert_with_bounds(
                2,
                2,
                0.10,
                QualityBounds::new(
                    0.05,
                    0.15,
                )
                .expect("bounds"),
            )
            .expect("insert");

        surface
            .insert_with_bounds(
                3,
                3,
                0.10,
                QualityBounds::new(
                    0.05,
                    0.30,
                )
                .expect("bounds"),
            )
            .expect("insert");

        let passing = surface
            .conservatively_passing_points(0.20)
            .expect("threshold");

        assert_eq!(passing.len(), 1);
        assert_eq!(passing[0].shape(), (2, 2));
    }

    #[test]
    fn best_point_respects_metric_direction() {
        let mut higher =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        higher
            .insert_value(2, 2, 0.8)
            .expect("insert");

        higher
            .insert_value(3, 3, 0.9)
            .expect("insert");

        assert_eq!(
            higher
                .best_point()
                .expect("best")
                .shape(),
            (3, 3)
        );

        let mut lower =
            VolumetricSurface::new(
                QualityDirection::LowerIsBetter,
            );

        lower
            .insert_value(2, 2, 0.8)
            .expect("insert");

        lower
            .insert_value(3, 3, 0.2)
            .expect("insert");

        assert_eq!(
            lower
                .best_point()
                .expect("best")
                .shape(),
            (3, 3)
        );
    }

    #[test]
    fn rectangle_completeness_distinguishes_missing_points() {
        let mut surface =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        for width in 1..=2 {
            for depth in 1..=2 {
                surface
                    .insert_value(
                        width,
                        depth,
                        0.8,
                    )
                    .expect("insert");
            }
        }

        assert!(surface
            .is_complete_rectangle(
                1,
                2,
                1,
                2
            )
            .expect("complete"));

        surface
            .insert_value(3, 3, 0.8)
            .expect("insert");

        assert!(!surface
            .is_complete_rectangle(
                1,
                3,
                1,
                3
            )
            .expect("incomplete"));
    }

    #[test]
    fn coverage_does_not_treat_missing_points_as_failures() {
        let mut surface =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        surface
            .insert_value(1, 1, 0.8)
            .expect("insert");

        surface
            .insert_value(2, 2, 0.8)
            .expect("insert");

        assert_eq!(
            surface
                .measured_cells(1, 2, 1, 2)
                .expect("count"),
            2
        );

        assert_eq!(
            surface
                .missing_cells(1, 2, 1, 2)
                .expect("missing"),
            2
        );

        assert!(
            (
                surface
                    .coverage(1, 2, 1, 2)
                    .expect("coverage")
                    - 0.5
            )
            .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn widest_and_deepest_passing_points_are_deterministic() {
        let mut surface =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        surface
            .insert_value(2, 10, 0.9)
            .expect("insert");

        surface
            .insert_value(5, 2, 0.8)
            .expect("insert");

        surface
            .insert_value(4, 7, 0.85)
            .expect("insert");

        assert_eq!(
            surface
                .widest_passing_point(0.8)
                .expect("query")
                .expect("point")
                .shape(),
            (5, 2)
        );

        assert_eq!(
            surface
                .deepest_passing_point(0.8)
                .expect("query")
                .expect("point")
                .shape(),
            (2, 10)
        );
    }

    #[test]
    fn limits_are_enforced() {
        let limits =
            SurfaceLimits::new(
                4,
                4,
                2,
            )
            .expect("valid limits");

        let mut surface =
            VolumetricSurface::with_limits(
                QualityDirection::HigherIsBetter,
                limits,
            );

        assert!(
            surface
                .insert_value(
                    5,
                    1,
                    0.8
                )
                .is_err()
        );

        surface
            .insert_value(1, 1, 0.8)
            .expect("first");

        surface
            .insert_value(2, 2, 0.8)
            .expect("second");

        assert!(
            surface
                .insert_value(3, 3, 0.8)
                .is_err()
        );
    }

    #[test]
    fn validation_accepts_canonical_surface() {
        let mut surface =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        surface
            .insert_with_bounds(
                1,
                1,
                0.9,
                QualityBounds::new(
                    0.8,
                    1.0,
                )
                .expect("bounds"),
            )
            .expect("insert");

        surface
            .insert_value(2, 4, 0.7)
            .expect("insert");

        assert!(surface.validate().is_ok());
    }

    #[test]
    fn schema_identifier_is_stable() {
        let surface =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        assert_eq!(
            surface.benchmark_surface_id(),
            "volumetric_surface"
        );

        assert_eq!(
            surface.schema_version(),
            1
        );
    }

    #[test]
    fn metric_metadata_is_retained() {
        let surface =
            VolumetricSurface::with_metric(
                QualityDirection::HigherIsBetter,
                "fidelity",
                Some("dimensionless"),
            );

        assert_eq!(
            surface.metric_id(),
            Some("fidelity")
        );

        assert_eq!(
            surface.metric_unit(),
            Some("dimensionless")
        );
    }

    #[test]
    fn points_are_owned_without_changing_semantics() {
        let mut surface =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        surface
            .insert_value(2, 3, 0.91)
            .expect("insert");

        let copied = surface.to_points();

        assert_eq!(copied.len(), 1);
        assert_eq!(
            copied[0].shape(),
            (2, 3)
        );
        assert_eq!(
            copied[0].quality,
            0.91
        );
    }

    #[test]
    fn quality_direction_margin_is_correct() {
        assert_eq!(
            QualityDirection::HigherIsBetter
                .margin(0.9, 0.8),
            0.1
        );

        assert_eq!(
            QualityDirection::LowerIsBetter
                .margin(0.1, 0.2),
            0.1
        );
    }
}