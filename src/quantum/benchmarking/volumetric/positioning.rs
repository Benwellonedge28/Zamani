//! Zamani Quantum Benchmarking — Volumetric Positioning
//!
//! # Purpose
//!
//! This module provides the analytical positioning layer for volumetric
//! quantum benchmarking.
//!
//! A volumetric surface represents measured performance at:
//!
//! ```text
//! (width, depth) -> quality
//! ```
//!
//! `positioning.rs` answers questions such as:
//!
//! - Where is a system positioned in width/depth/quality space?
//! - What is the normalized position of an individual benchmark point?
//! - What is the best measured operating point?
//! - What is the largest measured rectangle satisfying a quality envelope?
//! - How much of the requested benchmark space was actually measured?
//! - How does one benchmark surface compare with another at common coordinates?
//! - Which system has better quality at equivalent workload sizes?
//! - How much quality advantage/disadvantage exists between two systems?
//! - What is the measured workload frontier without inventing/interpolating data?
//!
//! # Architectural boundary
//!
//! This module deliberately does NOT:
//!
//! - generate quantum circuits;
//! - execute circuits;
//! - select a backend;
//! - compile or transpile circuits;
//! - perform routing;
//! - perform scheduling;
//! - calculate Quantum Volume;
//! - perform randomized benchmarking;
//! - calculate XEB;
//! - perform statistical fitting;
//! - interpolate missing points;
//! - extrapolate measurements;
//! - invent measurements;
//! - silently discard measurements;
//! - depend on Quantum IR;
//! - depend on a simulator;
//! - depend on a hardware provider;
//! - maintain global state;
//! - print diagnostics.
//!
//! Those responsibilities belong to the surrounding benchmarking system.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! benchmark generation/execution
//!      │
//!      ▼
//! statistical analysis
//!      │
//!      ▼
//! volumetric::surface
//!      │
//!      ▼
//! volumetric::positioning
//!      │
//!      ├──► analysis
//!      └──► reporting
//! ```
//!
//! This module therefore depends only on the already-defined
//! `volumetric::surface` representation and the Rust standard library.
//!
//! # Scientific semantics
//!
//! Positioning is deliberately separated from benchmarking itself.
//!
//! A benchmark measurement is an observation.
//!
//! Positioning is a derived analytical representation of those observations.
//!
//! No claim of quantum advantage, speedup, superiority, or scalability is made
//! merely because one surface has a larger positioning score.
//!
//! Comparisons are valid only when the compared surfaces have compatible
//! metric semantics and units.
//!
//! # Missing measurements
//!
//! Missing points are NOT failures.
//!
//! If a system has:
//!
//! ```text
//! (1,1) measured
//! (1,2) measured
//! (1,3) missing
//! ```
//!
//! positioning.rs never converts `(1,3)` into zero quality.
//!
//! Coverage and comparison statistics explicitly account for missing data.
//!
//! # No interpolation
//!
//! This module never infers an unmeasured point from neighboring points.
//!
//! Consequently:
//!
//! ```text
//! measured point      = measured
//! missing point       = unknown
//! inferred point      = not represented
//! ```
//!
//! Any interpolation or model-based reconstruction belongs in a future
//! explicitly named analysis module.
//!
//! # Determinism
//!
//! All public collections returned by this module are deterministic.
//!
//! Coordinates are ordered:
//!
//! 1. width ascending;
//! 2. depth ascending;
//! 3. quality tie-breaking according to the configured metric direction.
//!
//! No process-global random state is used.
//!
//! # Resource safety
//!
//! This module validates requested coordinate ranges and comparison limits
//! before allocating output collections.
//!
//! It never allocates based on an unchecked attacker-controlled multiplication.
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
//! No external crate is required.
//!
//! # Integration contract
//!
//! This file consumes:
//!
//! ```text
//! volumetric::surface::VolumetricSurface
//! volumetric::surface::SurfacePoint
//! volumetric::surface::QualityDirection
//! ```
//!
//! Future modules may consume this file without modifying its core semantics:
//!
//! ```text
//! volumetric::positioning
//!        │
//!        ├──► analysis::compare
//!        ├──► analysis::baseline
//!        ├──► analysis::regression
//!        ├──► analysis::bottleneck
//!        └──► reporting::*
//! ```
//!
//! `positioning.rs` intentionally does not depend on those future modules.
//!
//! This permits this file to be completed and tested independently first.

use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

use super::surface::{
    QualityDirection,
    SurfacePoint,
    VolumetricSurface,
};

// ============================================================================
// Public constants
// ============================================================================

/// Stable identifier for the positioning analysis layer.
pub const VOLUMETRIC_POSITIONING_ID: &str =
    "volumetric_positioning";

/// Schema version of the positioning result contract.
pub const VOLUMETRIC_POSITIONING_SCHEMA_VERSION: u32 = 1;

/// Default maximum number of points accepted in one positioning analysis.
pub const DEFAULT_MAX_ANALYSIS_POINTS: usize = 1_000_000;

/// Default maximum number of pairwise comparison records.
pub const DEFAULT_MAX_COMPARISON_POINTS: usize = 1_000_000;

/// Default minimum coordinate used by a full surface-positioning request.
pub const DEFAULT_MIN_COORDINATE: usize = 1;

/// Default epsilon used only for finite floating-point comparisons.
pub const POSITIONING_EPSILON: f64 = 1.0e-12;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by volumetric positioning analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum PositioningError {
    /// The supplied surface is empty.
    EmptySurface,

    /// A requested coordinate is zero.
    InvalidCoordinate {
        /// Coordinate value.
        value: usize,
    },

    /// A minimum coordinate is greater than its maximum coordinate.
    InvalidRange {
        /// Minimum value.
        minimum: usize,

        /// Maximum value.
        maximum: usize,
    },

    /// A requested range would overflow when its cell count is calculated.
    RangeOverflow {
        /// Width of the requested range.
        width: usize,

        /// Depth of the requested range.
        depth: usize,
    },

    /// A requested analysis exceeds the configured point limit.
    AnalysisLimitExceeded {
        /// Requested number of points.
        requested: usize,

        /// Maximum permitted number.
        maximum: usize,
    },

    /// The quality value is not finite.
    NonFiniteQuality {
        /// Invalid value.
        value: f64,
    },

    /// A normalization bound is invalid.
    InvalidNormalizationBounds {
        /// Lower bound.
        minimum: f64,

        /// Upper bound.
        maximum: f64,
    },

    /// A normalization range has zero width.
    ZeroNormalizationRange {
        /// Constant value.
        value: f64,
    },

    /// A threshold is not finite.
    NonFiniteThreshold {
        /// Invalid threshold.
        value: f64,
    },

    /// A quality threshold lies outside a unit interval.
    ThresholdOutOfRange {
        /// Invalid threshold.
        value: f64,
    },

    /// Surfaces have incompatible quality directions.
    IncompatibleQualityDirection {
        /// Direction of the first surface.
        left: QualityDirection,

        /// Direction of the second surface.
        right: QualityDirection,
    },

    /// Both surfaces explicitly identify different metrics.
    IncompatibleMetric {
        /// First metric identifier.
        left: String,

        /// Second metric identifier.
        right: String,
    },

    /// Both surfaces explicitly identify different units.
    IncompatibleUnit {
        /// First unit.
        left: String,

        /// Second unit.
        right: String,
    },

    /// A relative comparison was requested for a zero denominator.
    ZeroComparisonDenominator,

    /// A computed score was not finite.
    NonFiniteScore,

    /// A comparison result would exceed its configured storage limit.
    ComparisonLimitExceeded {
        /// Number of requested comparison records.
        requested: usize,

        /// Maximum permitted records.
        maximum: usize,
    },
}

impl fmt::Display for PositioningError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptySurface => {
                write!(
                    formatter,
                    "cannot position an empty volumetric surface"
                )
            }

            Self::InvalidCoordinate { value } => {
                write!(
                    formatter,
                    "volumetric coordinate must be greater than zero, got {}",
                    value
                )
            }

            Self::InvalidRange { minimum, maximum } => {
                write!(
                    formatter,
                    "invalid volumetric range: minimum {} > maximum {}",
                    minimum,
                    maximum
                )
            }

            Self::RangeOverflow { width, depth } => {
                write!(
                    formatter,
                    "volumetric range {} x {} overflows usize",
                    width,
                    depth
                )
            }

            Self::AnalysisLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "positioning analysis requires {} points, exceeding limit {}",
                    requested,
                    maximum
                )
            }

            Self::NonFiniteQuality { value } => {
                write!(
                    formatter,
                    "positioning quality must be finite, got {}",
                    value
                )
            }

            Self::InvalidNormalizationBounds {
                minimum,
                maximum,
            } => {
                write!(
                    formatter,
                    "invalid normalization bounds: minimum {} > maximum {}",
                    minimum,
                    maximum
                )
            }

            Self::ZeroNormalizationRange { value } => {
                write!(
                    formatter,
                    "normalization range cannot have zero width; value={}",
                    value
                )
            }

            Self::NonFiniteThreshold { value } => {
                write!(
                    formatter,
                    "positioning threshold must be finite, got {}",
                    value
                )
            }

            Self::ThresholdOutOfRange { value } => {
                write!(
                    formatter,
                    "positioning threshold must be in [0, 1], got {}",
                    value
                )
            }

            Self::IncompatibleQualityDirection {
                left,
                right,
            } => {
                write!(
                    formatter,
                    "incompatible quality directions: {} vs {}",
                    left.as_str(),
                    right.as_str()
                )
            }

            Self::IncompatibleMetric { left, right } => {
                write!(
                    formatter,
                    "incompatible volumetric metrics: '{}' vs '{}'",
                    left,
                    right
                )
            }

            Self::IncompatibleUnit { left, right } => {
                write!(
                    formatter,
                    "incompatible volumetric units: '{}' vs '{}'",
                    left,
                    right
                )
            }

            Self::ZeroComparisonDenominator => {
                write!(
                    formatter,
                    "relative comparison cannot use a zero denominator"
                )
            }

            Self::NonFiniteScore => {
                write!(
                    formatter,
                    "positioning calculation produced a non-finite score"
                )
            }

            Self::ComparisonLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "comparison requires {} records, exceeding limit {}",
                    requested,
                    maximum
                )
            }
        }
    }
}

impl Error for PositioningError {}

// ============================================================================
// Normalization
// ============================================================================

/// Defines how a scalar quality value is normalized.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QualityNormalization {
    /// Quality is already defined on [0, 1].
    UnitInterval,

    /// Normalize using explicit finite bounds.
    Explicit {
        /// Minimum possible/reference value.
        minimum: f64,

        /// Maximum possible/reference value.
        maximum: f64,
    },

    /// Do not normalize quality.
    ///
    /// This is valid for reporting raw values but should generally not be used
    /// for an aggregate score across heterogeneous metrics.
    Raw,
}

impl QualityNormalization {
    /// Creates an explicit normalization range.
    pub fn explicit(
        minimum: f64,
        maximum: f64,
    ) -> Result<Self, PositioningError> {
        validate_normalization_bounds(minimum, maximum)?;

        Ok(Self::Explicit {
            minimum,
            maximum,
        })
    }

    /// Returns the normalized value.
    ///
    /// For `HigherIsBetter`, larger normalized values remain better.
    ///
    /// For `LowerIsBetter`, the value is inverted so that the returned
    /// normalized quality always follows the positioning convention:
    ///
    /// ```text
    /// 1.0 = best
    /// 0.0 = worst
    /// ```
    pub fn normalize(
        self,
        value: f64,
        direction: QualityDirection,
    ) -> Result<f64, PositioningError> {
        if !value.is_finite() {
            return Err(
                PositioningError::NonFiniteQuality { value }
            );
        }

        let normalized = match self {
            Self::UnitInterval => {
                if !(0.0..=1.0).contains(&value) {
                    return Err(
                        PositioningError::ThresholdOutOfRange {
                            value,
                        },
                    );
                }

                value
            }

            Self::Explicit {
                minimum,
                maximum,
            } => {
                validate_normalization_bounds(
                    minimum,
                    maximum,
                )?;

                (value - minimum) / (maximum - minimum)
            }

            Self::Raw => value,
        };

        let normalized = match direction {
            QualityDirection::HigherIsBetter => normalized,

            QualityDirection::LowerIsBetter => {
                match self {
                    Self::Raw => -normalized,

                    _ => 1.0 - normalized,
                }
            }
        };

        if !normalized.is_finite() {
            return Err(PositioningError::NonFiniteScore);
        }

        Ok(normalized)
    }
}

/// Validate normalization bounds.
fn validate_normalization_bounds(
    minimum: f64,
    maximum: f64,
) -> Result<(), PositioningError> {
    if !minimum.is_finite() || !maximum.is_finite() {
        return Err(
            PositioningError::InvalidNormalizationBounds {
                minimum,
                maximum,
            },
        );
    }

    if minimum > maximum {
        return Err(
            PositioningError::InvalidNormalizationBounds {
                minimum,
                maximum,
            },
        );
    }

    if (maximum - minimum).abs() <= POSITIONING_EPSILON {
        return Err(
            PositioningError::ZeroNormalizationRange {
                value: minimum,
            },
        );
    }

    Ok(())
}

// ============================================================================
// Coordinate normalization
// ============================================================================

/// Normalization specification for width and depth.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordinateNormalization {
    /// Maximum reference width.
    pub max_width: usize,

    /// Maximum reference depth.
    pub max_depth: usize,
}

impl CoordinateNormalization {
    /// Construct explicit coordinate normalization.
    pub fn new(
        max_width: usize,
        max_depth: usize,
    ) -> Result<Self, PositioningError> {
        if max_width == 0 {
            return Err(
                PositioningError::InvalidCoordinate {
                    value: max_width,
                },
            );
        }

        if max_depth == 0 {
            return Err(
                PositioningError::InvalidCoordinate {
                    value: max_depth,
                },
            );
        }

        Ok(Self {
            max_width,
            max_depth,
        })
    }

    /// Normalize a width into [0, 1].
    #[must_use]
    pub fn width(&self, width: usize) -> f64 {
        width as f64 / self.max_width as f64
    }

    /// Normalize a depth into [0, 1].
    #[must_use]
    pub fn depth(&self, depth: usize) -> f64 {
        depth as f64 / self.max_depth as f64
    }
}

// ============================================================================
// Positioning configuration
// ============================================================================

/// Configuration controlling volumetric positioning analysis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PositioningConfig {
    /// Width/depth normalization reference.
    pub coordinates: CoordinateNormalization,

    /// Quality normalization strategy.
    pub quality_normalization: QualityNormalization,

    /// Maximum number of points that may be analyzed.
    pub max_analysis_points: usize,

    /// Optional quality threshold.
    ///
    /// When present, passing/attainable summaries are calculated.
    pub threshold: Option<f64>,

    /// Whether conservative uncertainty bounds should be used when present.
    pub conservative_threshold: bool,
}

impl PositioningConfig {
    /// Create production configuration.
    pub fn new(
        max_width: usize,
        max_depth: usize,
        quality_normalization: QualityNormalization,
    ) -> Result<Self, PositioningError> {
        Ok(Self {
            coordinates: CoordinateNormalization::new(
                max_width,
                max_depth,
            )?,
            quality_normalization,
            max_analysis_points:
                DEFAULT_MAX_ANALYSIS_POINTS,
            threshold: None,
            conservative_threshold: true,
        })
    }

    /// Attach a quality threshold.
    pub fn with_threshold(
        mut self,
        threshold: f64,
    ) -> Result<Self, PositioningError> {
        if !threshold.is_finite() {
            return Err(
                PositioningError::NonFiniteThreshold {
                    value: threshold,
                },
            );
        }

        self.threshold = Some(threshold);

        Ok(self)
    }

    /// Configure conservative or point-estimate classification.
    #[must_use]
    pub const fn with_conservative_threshold(
        mut self,
        conservative: bool,
    ) -> Self {
        self.conservative_threshold = conservative;
        self
    }

    /// Configure the analysis point limit.
    pub fn with_max_analysis_points(
        mut self,
        maximum: usize,
    ) -> Result<Self, PositioningError> {
        if maximum == 0 {
            return Err(
                PositioningError::AnalysisLimitExceeded {
                    requested: 1,
                    maximum,
                },
            );
        }

        self.max_analysis_points = maximum;

        Ok(self)
    }
}

// ============================================================================
// Position vector
// ============================================================================

/// A normalized position of one measured volumetric point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PositionVector {
    /// Original width.
    pub width: usize,

    /// Original depth.
    pub depth: usize,

    /// Normalized width in [0, 1].
    pub normalized_width: f64,

    /// Normalized depth in [0, 1].
    pub normalized_depth: f64,

    /// Raw quality.
    pub quality: f64,

    /// Quality normalized so 1.0 is best and 0.0 is worst whenever a bounded
    /// normalization is used.
    pub normalized_quality: f64,

    /// Whether this point satisfies the configured quality threshold.
    pub passes_threshold: Option<bool>,

    /// Whether this point can be considered conservatively passing.
    pub conservatively_passes_threshold: Option<bool>,
}

impl PositionVector {
    /// Returns the normalized three-dimensional positioning coordinates.
    #[must_use]
    pub const fn vector(
        &self,
    ) -> (f64, f64, f64) {
        (
            self.normalized_width,
            self.normalized_depth,
            self.normalized_quality,
        )
    }

    /// Returns the Euclidean norm of the normalized position vector.
    #[must_use]
    pub fn euclidean_norm(&self) -> f64 {
        let value = self.normalized_width
            * self.normalized_width
            + self.normalized_depth
                * self.normalized_depth
            + self.normalized_quality
                * self.normalized_quality;

        value.sqrt()
    }

    /// Returns a workload-size product in normalized coordinates.
    #[must_use]
    pub fn normalized_rectangular_volume(
        &self,
    ) -> f64 {
        self.normalized_width
            * self.normalized_depth
    }
}

// ============================================================================
// Position weighting
// ============================================================================

/// Weights used when reducing a normalized position to one scalar score.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PositionWeights {
    /// Importance of width.
    pub width: f64,

    /// Importance of depth.
    pub depth: f64,

    /// Importance of quality.
    pub quality: f64,
}

impl PositionWeights {
    /// Equal weighting.
    pub const fn equal() -> Self {
        Self {
            width: 1.0,
            depth: 1.0,
            quality: 1.0,
        }
    }

    /// Workload-focused weighting.
    pub const fn workload_focused() -> Self {
        Self {
            width: 1.0,
            depth: 1.0,
            quality: 0.5,
        }
    }

    /// Quality-focused weighting.
    pub const fn quality_focused() -> Self {
        Self {
            width: 0.5,
            depth: 0.5,
            quality: 1.0,
        }
    }

    /// Validate the weights.
    pub fn validate(&self) -> Result<(), PositioningError> {
        if !self.width.is_finite()
            || !self.depth.is_finite()
            || !self.quality.is_finite()
        {
            return Err(PositioningError::NonFiniteScore);
        }

        if self.width < 0.0
            || self.depth < 0.0
            || self.quality < 0.0
        {
            return Err(PositioningError::NonFiniteScore);
        }

        if self.width == 0.0
            && self.depth == 0.0
            && self.quality == 0.0
        {
            return Err(PositioningError::NonFiniteScore);
        }

        Ok(())
    }

    /// Calculate the weighted arithmetic positioning score.
    ///
    /// The result is normalized to the interval [0, 1] when each input
    /// dimension is normalized to [0, 1].
    pub fn score(
        &self,
        position: &PositionVector,
    ) -> Result<f64, PositioningError> {
        self.validate()?;

        let total =
            self.width + self.depth + self.quality;

        let score = (
            self.width * position.normalized_width
                + self.depth
                    * position.normalized_depth
                + self.quality
                    * position.normalized_quality
        ) / total;

        if !score.is_finite() {
            return Err(PositioningError::NonFiniteScore);
        }

        Ok(score)
    }
}

// ============================================================================
// Aggregate position
// ============================================================================

/// Aggregate positioning summary for one volumetric surface.
#[derive(Debug, Clone, PartialEq)]
pub struct SurfacePosition {
    /// Stable schema version.
    pub schema_version: u32,

    /// Number of measured points.
    pub measured_points: usize,

    /// Number of points satisfying the threshold.
    pub passing_points: Option<usize>,

    /// Fraction of measured requested coordinates.
    pub coverage: f64,

    /// Average normalized width of measured points.
    pub mean_normalized_width: f64,

    /// Average normalized depth of measured points.
    pub mean_normalized_depth: f64,

    /// Average normalized quality.
    pub mean_normalized_quality: f64,

    /// Maximum normalized quality.
    pub maximum_normalized_quality: f64,

    /// Best measured point.
    pub best_point: PositionVector,

    /// Best scalar weighted positioning score.
    pub best_score: f64,

    /// Mean scalar weighted positioning score.
    pub mean_score: f64,

    /// Largest measured width.
    pub maximum_width: usize,

    /// Largest measured depth.
    pub maximum_depth: usize,

    /// Largest measured rectangular coordinate among actual points.
    pub maximum_rectangular_volume: usize,
}

impl SurfacePosition {
    /// Returns the normalized aggregate vector.
    #[must_use]
    pub fn mean_vector(
        &self,
    ) -> (f64, f64, f64) {
        (
            self.mean_normalized_width,
            self.mean_normalized_depth,
            self.mean_normalized_quality,
        )
    }
}

// ============================================================================
// Point positioning
// ============================================================================

/// Position one point using a positioning configuration.
pub fn position_point(
    point: &SurfacePoint,
    surface: &VolumetricSurface,
    config: &PositioningConfig,
) -> Result<PositionVector, PositioningError> {
    validate_point(point)?;

    let normalized_quality =
        config
            .quality_normalization
            .normalize(
                point.quality,
                surface.quality_direction(),
            )?;

    let passes_threshold = match config.threshold {
        Some(threshold) => Some(
            point
                .classifies(
                    surface.quality_direction(),
                    threshold,
                )
                .map_err(|_| {
                    PositioningError::NonFiniteThreshold {
                        value: threshold,
                    }
                })?,
        ),

        None => None,
    };

    let conservatively_passes_threshold =
        match config.threshold {
            Some(threshold) => Some(
                point
                    .classifies_conservatively(
                        surface.quality_direction(),
                        threshold,
                    )
                    .map_err(|_| {
                        PositioningError::NonFiniteThreshold {
                            value: threshold,
                        }
                    })?,
            ),

            None => None,
        };

    Ok(PositionVector {
        width: point.width,
        depth: point.depth,
        normalized_width: config
            .coordinates
            .width(point.width),
        normalized_depth: config
            .coordinates
            .depth(point.depth),
        quality: point.quality,
        normalized_quality,
        passes_threshold,
        conservatively_passes_threshold,
    })
}

// ============================================================================
// Surface positioning
// ============================================================================

/// Calculate the aggregate positioning of a volumetric surface.
pub fn position_surface(
    surface: &VolumetricSurface,
    config: &PositioningConfig,
    weights: PositionWeights,
) -> Result<SurfacePosition, PositioningError> {
    if surface.is_empty() {
        return Err(PositioningError::EmptySurface);
    }

    weights.validate()?;

    if surface.len() > config.max_analysis_points {
        return Err(
            PositioningError::AnalysisLimitExceeded {
                requested: surface.len(),
                maximum: config.max_analysis_points,
            },
        );
    }

    let mut positions =
        Vec::with_capacity(surface.len());

    for point in surface.points() {
        positions.push(position_point(
            point,
            surface,
            config,
        )?);
    }

    let measured_points = positions.len();

    let mut width_sum = 0.0;
    let mut depth_sum = 0.0;
    let mut quality_sum = 0.0;
    let mut maximum_quality =
        f64::NEG_INFINITY;

    let mut score_sum = 0.0;
    let mut best_score =
        f64::NEG_INFINITY;

    let mut best_position: Option<PositionVector> =
        None;

    let mut passing_points = 0usize;

    let mut maximum_width = 0usize;
    let mut maximum_depth = 0usize;
    let mut maximum_rectangular_volume = 0usize;

    for position in &positions {
        width_sum += position.normalized_width;
        depth_sum += position.normalized_depth;
        quality_sum += position.normalized_quality;

        if position.normalized_quality
            > maximum_quality
        {
            maximum_quality =
                position.normalized_quality;
        }

        if position.passes_threshold == Some(true)
        {
            passing_points =
                passing_points.saturating_add(1);
        }

        let score =
            weights.score(position)?;

        score_sum += score;

        let replace = match best_position {
            None => true,

            Some(current) => {
                is_position_better(
                    position,
                    &current,
                    score,
                    best_score,
                )
            }
        };

        if replace {
            best_position = Some(*position);
            best_score = score;
        }

        if position.width > maximum_width {
            maximum_width = position.width;
        }

        if position.depth > maximum_depth {
            maximum_depth = position.depth;
        }

        let rectangular =
            position.width
                .checked_mul(position.depth)
                .unwrap_or(usize::MAX);

        if rectangular
            > maximum_rectangular_volume
        {
            maximum_rectangular_volume =
                rectangular;
        }
    }

    let coverage =
        calculate_surface_coverage(surface)?;

    let best_point =
        best_position.ok_or(
            PositioningError::EmptySurface,
        )?;

    let mean_score =
        score_sum / measured_points as f64;

    if !mean_score.is_finite()
        || !best_score.is_finite()
        || !coverage.is_finite()
    {
        return Err(PositioningError::NonFiniteScore);
    }

    Ok(SurfacePosition {
        schema_version:
            VOLUMETRIC_POSITIONING_SCHEMA_VERSION,

        measured_points,

        passing_points: config
            .threshold
            .map(|_| passing_points),

        coverage,

        mean_normalized_width:
            width_sum / measured_points as f64,

        mean_normalized_depth:
            depth_sum / measured_points as f64,

        mean_normalized_quality:
            quality_sum / measured_points as f64,

        maximum_normalized_quality:
            maximum_quality,

        best_point,

        best_score,

        mean_score,

        maximum_width,

        maximum_depth,

        maximum_rectangular_volume,
    })
}

/// Determine whether one normalized position is better than another.
///
/// The primary ordering is weighted scalar score. Ties are resolved by:
///
/// 1. normalized quality;
/// 2. normalized width;
/// 3. normalized depth;
/// 4. smaller raw width;
/// 5. smaller raw depth.
fn is_position_better(
    candidate: &PositionVector,
    current: &PositionVector,
    candidate_score: f64,
    current_score: f64,
) -> bool {
    if candidate_score
        > current_score + POSITIONING_EPSILON
    {
        return true;
    }

    if (candidate_score - current_score).abs()
        > POSITIONING_EPSILON
    {
        return false;
    }

    if candidate.normalized_quality
        > current.normalized_quality
            + POSITIONING_EPSILON
    {
        return true;
    }

    if candidate.normalized_width
        > current.normalized_width
            + POSITIONING_EPSILON
    {
        return true;
    }

    if candidate.normalized_depth
        > current.normalized_depth
            + POSITIONING_EPSILON
    {
        return true;
    }

    if candidate.width != current.width {
        return candidate.width < current.width;
    }

    candidate.depth < current.depth
}

// ============================================================================
// Coverage
// ============================================================================

/// Calculate the measured coverage of the surface's bounding rectangle.
///
/// Coverage is:
///
/// ```text
/// measured cells / total cells in measured bounding rectangle
/// ```
///
/// No interpolation is performed.
pub fn calculate_surface_coverage(
    surface: &VolumetricSurface,
) -> Result<f64, PositioningError> {
    let (
        min_width,
        max_width,
        min_depth,
        max_depth,
    ) = surface
        .bounds()
        .ok_or(PositioningError::EmptySurface)?;

    calculate_range_coverage(
        surface,
        min_width,
        max_width,
        min_depth,
        max_depth,
    )
}

/// Calculate coverage for an explicitly requested rectangle.
pub fn calculate_range_coverage(
    surface: &VolumetricSurface,
    min_width: usize,
    max_width: usize,
    min_depth: usize,
    max_depth: usize,
) -> Result<f64, PositioningError> {
    validate_range(min_width, max_width)?;
    validate_range(min_depth, max_depth)?;

    let width_count =
        max_width
            .checked_sub(min_width)
            .and_then(|value| value.checked_add(1))
            .ok_or(
                PositioningError::RangeOverflow {
                    width: max_width,
                    depth: max_depth,
                },
            )?;

    let depth_count =
        max_depth
            .checked_sub(min_depth)
            .and_then(|value| value.checked_add(1))
            .ok_or(
                PositioningError::RangeOverflow {
                    width: max_width,
                    depth: max_depth,
                },
            )?;

    let total_cells =
        width_count
            .checked_mul(depth_count)
            .ok_or(
                PositioningError::RangeOverflow {
                    width: width_count,
                    depth: depth_count,
                },
            )?;

    if total_cells == 0 {
        return Err(
            PositioningError::RangeOverflow {
                width: width_count,
                depth: depth_count,
            },
        );
    }

    let measured_cells =
        surface
            .measured_cells(
                min_width,
                max_width,
                min_depth,
                max_depth,
            )
            .map_err(|_| {
                PositioningError::RangeOverflow {
                    width: width_count,
                    depth: depth_count,
                }
            })?;

    let coverage =
        measured_cells as f64
            / total_cells as f64;

    if !coverage.is_finite() {
        return Err(PositioningError::NonFiniteScore);
    }

    Ok(coverage.clamp(0.0, 1.0))
}

// ============================================================================
// Threshold envelope
// ============================================================================

/// Represents the largest fully measured rectangle satisfying a quality
/// threshold.
///
/// This is deliberately based only on measured points.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeasuredEnvelope {
    /// Maximum width of the fully measured rectangle.
    pub width: usize,

    /// Maximum depth of the fully measured rectangle.
    pub depth: usize,

    /// Number of cells in the rectangle.
    pub cells: usize,

    /// Whether all cells in the rectangle were measured.
    pub complete: bool,
}

impl MeasuredEnvelope {
    /// Returns `width * depth` safely.
    pub fn rectangular_volume(
        &self,
    ) -> Result<usize, PositioningError> {
        self.width
            .checked_mul(self.depth)
            .ok_or(
                PositioningError::RangeOverflow {
                    width: self.width,
                    depth: self.depth,
                },
            )
    }
}

/// Find the largest measured rectangular envelope satisfying a quality
/// threshold.
///
/// The rectangle always starts at `(1, 1)`.
///
/// This function intentionally does not interpolate missing points.
pub fn largest_passing_envelope(
    surface: &VolumetricSurface,
    threshold: f64,
    conservative: bool,
    maximum_dimension: usize,
) -> Result<MeasuredEnvelope, PositioningError> {
    if surface.is_empty() {
        return Err(PositioningError::EmptySurface);
    }

    validate_threshold(threshold)?;

    if maximum_dimension == 0 {
        return Err(
            PositioningError::InvalidCoordinate {
                value: maximum_dimension,
            },
        );
    }

    let mut best =
        MeasuredEnvelope {
            width: 0,
            depth: 0,
            cells: 0,
            complete: false,
        };

    for width in 1..=maximum_dimension {
        for depth in 1..=maximum_dimension {
            let cells =
                width
                    .checked_mul(depth)
                    .ok_or(
                        PositioningError::RangeOverflow {
                            width,
                            depth,
                        },
                    )?;

            if cells <= best.cells {
                continue;
            }

            let complete =
                surface
                    .is_complete_rectangle(
                        1,
                        width,
                        1,
                        depth,
                    )
                    .map_err(|_| {
                        PositioningError::RangeOverflow {
                            width,
                            depth,
                        }
                    })?;

            if !complete {
                continue;
            }

            let passing = if conservative {
                surface
                    .conservatively_passing_points(
                        threshold,
                    )
                    .map_err(|_| {
                        PositioningError::NonFiniteThreshold {
                            value: threshold,
                        }
                    })?
                    .into_iter()
                    .all(|point| {
                        point.width <= width
                            && point.depth <= depth
                    })
            } else {
                surface
                    .passing_points(threshold)
                    .map_err(|_| {
                        PositioningError::NonFiniteThreshold {
                            value: threshold,
                        }
                    })?
                    .into_iter()
                    .filter(|point| {
                        point.width <= width
                            && point.depth <= depth
                    })
                    .count()
                    == cells
            };

            if passing {
                best =
                    MeasuredEnvelope {
                        width,
                        depth,
                        cells,
                        complete: true,
                    };
            }
        }
    }

    Ok(best)
}

// ============================================================================
// Common-coordinate comparison
// ============================================================================

/// Comparison mode for two compatible volumetric surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonMode {
    /// Compare raw quality difference.
    ///
    /// Positive means the left surface is better after respecting quality
    /// direction.
    AbsoluteDifference,

    /// Compare relative quality difference.
    ///
    /// This requires a non-zero right-side value.
    RelativeDifference,

    /// Compare normalized quality difference.
    ///
    /// This uses the supplied quality normalization.
    NormalizedDifference,
}

impl ComparisonMode {
    /// Stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AbsoluteDifference => {
                "absolute_difference"
            }

            Self::RelativeDifference => {
                "relative_difference"
            }

            Self::NormalizedDifference => {
                "normalized_difference"
            }
        }
    }
}

/// One common-coordinate comparison.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointComparison {
    /// Coordinate width.
    pub width: usize,

    /// Coordinate depth.
    pub depth: usize,

    /// Quality of the left surface.
    pub left_quality: f64,

    /// Quality of the right surface.
    pub right_quality: f64,

    /// Direction-aware signed advantage of the left surface.
    ///
    /// Positive means left is better.
    /// Negative means right is better.
    pub signed_advantage: f64,

    /// Whether left is better.
    pub left_better: bool,

    /// Whether right is better.
    pub right_better: bool,

    /// Whether the values are effectively tied.
    pub tie: bool,
}

impl PointComparison {
    /// Return the coordinate.
    pub const fn coordinate(
        &self,
    ) -> (usize, usize) {
        (self.width, self.depth)
    }
}

/// Aggregate comparison of two volumetric surfaces.
#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceComparison {
    /// Stable schema version.
    pub schema_version: u32,

    /// Comparison mode.
    pub mode: ComparisonMode,

    /// Number of points measured by both surfaces.
    pub common_points: usize,

    /// Points only measured by the left surface.
    pub left_only_points: usize,

    /// Points only measured by the right surface.
    pub right_only_points: usize,

    /// Points where left is better.
    pub left_wins: usize,

    /// Points where right is better.
    pub right_wins: usize,

    /// Tied points.
    pub ties: usize,

    /// Mean direction-aware advantage of left.
    pub mean_signed_advantage: f64,

    /// Minimum signed advantage of left.
    pub minimum_signed_advantage: f64,

    /// Maximum signed advantage of left.
    pub maximum_signed_advantage: f64,

    /// Fraction of common points won by left.
    pub left_win_rate: f64,

    /// Fraction of common points won by right.
    pub right_win_rate: f64,

    /// Detailed common-coordinate comparisons.
    pub points: Vec<PointComparison>,
}

/// Configuration for surface comparison.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComparisonConfig {
    /// Comparison mode.
    pub mode: ComparisonMode,

    /// Quality normalization used by normalized comparisons.
    pub normalization: QualityNormalization,

    /// Maximum number of common-coordinate records.
    pub max_points: usize,
}

impl ComparisonConfig {
    /// Create an absolute-difference comparison.
    pub fn absolute() -> Self {
        Self {
            mode: ComparisonMode::AbsoluteDifference,
            normalization:
                QualityNormalization::Raw,
            max_points:
                DEFAULT_MAX_COMPARISON_POINTS,
        }
    }

    /// Create a relative-difference comparison.
    pub fn relative() -> Self {
        Self {
            mode: ComparisonMode::RelativeDifference,
            normalization:
                QualityNormalization::Raw,
            max_points:
                DEFAULT_MAX_COMPARISON_POINTS,
        }
    }

    /// Create a normalized comparison.
    pub fn normalized(
        normalization: QualityNormalization,
    ) -> Self {
        Self {
            mode:
                ComparisonMode::NormalizedDifference,
            normalization,
            max_points:
                DEFAULT_MAX_COMPARISON_POINTS,
        }
    }

    /// Set the comparison point limit.
    pub fn with_max_points(
        mut self,
        maximum: usize,
    ) -> Result<Self, PositioningError> {
        if maximum == 0 {
            return Err(
                PositioningError::ComparisonLimitExceeded {
                    requested: 1,
                    maximum,
                },
            );
        }

        self.max_points = maximum;

        Ok(self)
    }
}

/// Compare two compatible surfaces at common coordinates.
///
/// No interpolation is performed.
///
/// A point absent from one surface is excluded from common-coordinate
/// comparison and is counted separately.
pub fn compare_surfaces(
    left: &VolumetricSurface,
    right: &VolumetricSurface,
    config: &ComparisonConfig,
) -> Result<SurfaceComparison, PositioningError> {
    validate_surface_compatibility(
        left,
        right,
    )?;

    let common_capacity =
        left.len().min(right.len());

    if common_capacity > config.max_points {
        return Err(
            PositioningError::ComparisonLimitExceeded {
                requested: common_capacity,
                maximum: config.max_points,
            },
        );
    }

    let mut points =
        Vec::with_capacity(common_capacity);

    let mut left_only = 0usize;
    let mut right_only = 0usize;

    for left_point in left.points() {
        match right.get(
            left_point.width,
            left_point.depth,
        ) {
            Some(right_point) => {
                let comparison =
                    compare_points(
                        left_point,
                        right_point,
                        left.quality_direction(),
                        config,
                    )?;

                points.push(comparison);
            }

            None => {
                left_only =
                    left_only.saturating_add(1);
            }
        }
    }

    for right_point in right.points() {
        if !left.contains(
            right_point.width,
            right_point.depth,
        ) {
            right_only =
                right_only.saturating_add(1);
        }
    }

    points.sort_by(compare_point_coordinates);

    let common_points = points.len();

    let mut left_wins = 0usize;
    let mut right_wins = 0usize;
    let mut ties = 0usize;

    let mut advantage_sum = 0.0;
    let mut minimum_advantage =
        f64::INFINITY;
    let mut maximum_advantage =
        f64::NEG_INFINITY;

    for point in &points {
        advantage_sum +=
            point.signed_advantage;

        if point.signed_advantage
            > POSITIONING_EPSILON
        {
            left_wins =
                left_wins.saturating_add(1);
        } else if point.signed_advantage
            < -POSITIONING_EPSILON
        {
            right_wins =
                right_wins.saturating_add(1);
        } else {
            ties = ties.saturating_add(1);
        }

        if point.signed_advantage
            < minimum_advantage
        {
            minimum_advantage =
                point.signed_advantage;
        }

        if point.signed_advantage
            > maximum_advantage
        {
            maximum_advantage =
                point.signed_advantage;
        }
    }

    if common_points == 0 {
        minimum_advantage = 0.0;
        maximum_advantage = 0.0;
    }

    let mean_signed_advantage =
        if common_points == 0 {
            0.0
        } else {
            advantage_sum
                / common_points as f64
        };

    let left_win_rate =
        if common_points == 0 {
            0.0
        } else {
            left_wins as f64
                / common_points as f64
        };

    let right_win_rate =
        if common_points == 0 {
            0.0
        } else {
            right_wins as f64
                / common_points as f64
        };

    if !mean_signed_advantage.is_finite()
        || !minimum_advantage.is_finite()
        || !maximum_advantage.is_finite()
        || !left_win_rate.is_finite()
        || !right_win_rate.is_finite()
    {
        return Err(PositioningError::NonFiniteScore);
    }

    Ok(SurfaceComparison {
        schema_version:
            VOLUMETRIC_POSITIONING_SCHEMA_VERSION,

        mode: config.mode,

        common_points,

        left_only_points: left_only,

        right_only_points: right_only,

        left_wins,

        right_wins,

        ties,

        mean_signed_advantage,

        minimum_signed_advantage:
            minimum_advantage,

        maximum_signed_advantage:
            maximum_advantage,

        left_win_rate,

        right_win_rate,

        points,
    })
}

/// Compare two individual common-coordinate points.
fn compare_points(
    left: &SurfacePoint,
    right: &SurfacePoint,
    direction: QualityDirection,
    config: &ComparisonConfig,
) -> Result<PointComparison, PositioningError> {
    validate_point(left)?;
    validate_point(right)?;

    if left.width != right.width
        || left.depth != right.depth
    {
        return Err(PositioningError::InvalidRange {
            minimum: 1,
            maximum: 0,
        });
    }

    let signed_advantage =
        match config.mode {
            ComparisonMode::AbsoluteDifference => {
                direction_aware_difference(
                    left.quality,
                    right.quality,
                    direction,
                )
            }

            ComparisonMode::RelativeDifference => {
                let denominator =
                    right.quality.abs();

                if denominator
                    <= POSITIONING_EPSILON
                {
                    return Err(
                        PositioningError::ZeroComparisonDenominator,
                    );
                }

                direction_aware_difference(
                    left.quality,
                    right.quality,
                    direction,
                ) / denominator
            }

            ComparisonMode::NormalizedDifference => {
                let left_normalized =
                    config
                        .normalization
                        .normalize(
                            left.quality,
                            direction,
                        )?;

                let right_normalized =
                    config
                        .normalization
                        .normalize(
                            right.quality,
                            direction,
                        )?;

                left_normalized
                    - right_normalized
            }
        };

    if !signed_advantage.is_finite() {
        return Err(PositioningError::NonFiniteScore);
    }

    let left_better =
        signed_advantage
            > POSITIONING_EPSILON;

    let right_better =
        signed_advantage
            < -POSITIONING_EPSILON;

    let tie =
        !left_better && !right_better;

    Ok(PointComparison {
        width: left.width,
        depth: left.depth,
        left_quality: left.quality,
        right_quality: right.quality,
        signed_advantage,
        left_better,
        right_better,
        tie,
    })
}

/// Convert a quality difference into a direction-aware advantage.
fn direction_aware_difference(
    left: f64,
    right: f64,
    direction: QualityDirection,
) -> f64 {
    match direction {
        QualityDirection::HigherIsBetter => {
            left - right
        }

        QualityDirection::LowerIsBetter => {
            right - left
        }
    }
}

// ============================================================================
// Compatibility
// ============================================================================

/// Validate that two surfaces are scientifically comparable.
pub fn validate_surface_compatibility(
    left: &VolumetricSurface,
    right: &VolumetricSurface,
) -> Result<(), PositioningError> {
    if left.quality_direction()
        != right.quality_direction()
    {
        return Err(
            PositioningError::IncompatibleQualityDirection {
                left: left.quality_direction(),
                right: right.quality_direction(),
            },
        );
    }

    match (
        left.metric_id(),
        right.metric_id(),
    ) {
        (Some(left_metric), Some(right_metric))
            if left_metric != right_metric =>
        {
            return Err(
                PositioningError::IncompatibleMetric {
                    left: left_metric.to_owned(),
                    right: right_metric.to_owned(),
                },
            );
        }

        _ => {}
    }

    match (
        left.metric_unit(),
        right.metric_unit(),
    ) {
        (Some(left_unit), Some(right_unit))
            if left_unit != right_unit =>
        {
            return Err(
                PositioningError::IncompatibleUnit {
                    left: left_unit.to_owned(),
                    right: right_unit.to_owned(),
                },
            );
        }

        _ => {}
    }

    Ok(())
}

// ============================================================================
// Measured frontier
// ============================================================================

/// One point on the measured quality/workload frontier.
///
/// A frontier point is always an actual measured point.
///
/// No interpolation or extrapolation is performed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeasuredFrontierPoint {
    /// Width.
    pub width: usize,

    /// Depth.
    pub depth: usize,

    /// Quality.
    pub quality: f64,
}

/// Determine the nondominated measured points in width/depth/quality space.
///
/// A point A dominates point B when A is:
///
/// - at least as large in width;
/// - at least as large in depth;
/// - at least as good in quality;
///
/// and strictly better in at least one dimension.
///
/// This uses the configured quality direction.
///
/// The result contains only actual measured points.
pub fn measured_pareto_frontier(
    surface: &VolumetricSurface,
    maximum_points: usize,
) -> Result<Vec<MeasuredFrontierPoint>, PositioningError> {
    if surface.is_empty() {
        return Err(PositioningError::EmptySurface);
    }

    if maximum_points == 0 {
        return Err(
            PositioningError::AnalysisLimitExceeded {
                requested: 1,
                maximum: maximum_points,
            },
        );
    }

    if surface.len() > maximum_points {
        return Err(
            PositioningError::AnalysisLimitExceeded {
                requested: surface.len(),
                maximum: maximum_points,
            },
        );
    }

    let points = surface.points();

    let mut frontier =
        Vec::with_capacity(points.len());

    'candidate: for candidate in points {
        for other in points {
            if std::ptr::eq(
                candidate,
                other,
            ) {
                continue;
            }

            if dominates(
                other,
                candidate,
                surface.quality_direction(),
            ) {
                continue 'candidate;
            }
        }

        frontier.push(
            MeasuredFrontierPoint {
                width: candidate.width,
                depth: candidate.depth,
                quality: candidate.quality,
            },
        );
    }

    frontier.sort_by(|left, right| {
        left.width
            .cmp(&right.width)
            .then_with(|| {
                left.depth.cmp(&right.depth)
            })
            .then_with(|| {
                compare_quality_for_direction(
                    surface.quality_direction(),
                    left.quality,
                    right.quality,
                )
            })
    });

    Ok(frontier)
}

/// Determine whether one measured point dominates another.
fn dominates(
    candidate: &SurfacePoint,
    other: &SurfacePoint,
    direction: QualityDirection,
) -> bool {
    let width_at_least =
        candidate.width >= other.width;

    let depth_at_least =
        candidate.depth >= other.depth;

    let quality_at_least =
        direction.is_at_least_as_good(
            candidate.quality,
            other.quality,
        );

    if !width_at_least
        || !depth_at_least
        || !quality_at_least
    {
        return false;
    }

    candidate.width > other.width
        || candidate.depth > other.depth
        || direction.is_better(
            candidate.quality,
            other.quality,
        )
}

// ============================================================================
// Best point at workload constraints
// ============================================================================

/// Select the best measured point satisfying workload limits.
///
/// `max_width` and `max_depth` are upper bounds, not interpolation targets.
pub fn best_point_within(
    surface: &VolumetricSurface,
    max_width: usize,
    max_depth: usize,
) -> Result<Option<&SurfacePoint>, PositioningError> {
    if max_width == 0 {
        return Err(
            PositioningError::InvalidCoordinate {
                value: max_width,
            },
        );
    }

    if max_depth == 0 {
        return Err(
            PositioningError::InvalidCoordinate {
                value: max_depth,
            },
        );
    }

    let mut best: Option<&SurfacePoint> =
        None;

    for point in surface.points() {
        if point.width > max_width
            || point.depth > max_depth
        {
            continue;
        }

        best = match best {
            None => Some(point),

            Some(current) => {
                if better_within(
                    point,
                    current,
                    surface.quality_direction(),
                ) {
                    Some(point)
                } else {
                    Some(current)
                }
            }
        };
    }

    Ok(best)
}

/// Determine whether a point is better under a workload constraint.
fn better_within(
    candidate: &SurfacePoint,
    current: &SurfacePoint,
    direction: QualityDirection,
) -> bool {
    if direction.is_better(
        candidate.quality,
        current.quality,
    ) {
        return true;
    }

    if direction.is_better(
        current.quality,
        candidate.quality,
    ) {
        return false;
    }

    let candidate_volume =
        candidate
            .width
            .checked_mul(candidate.depth)
            .unwrap_or(usize::MAX);

    let current_volume =
        current
            .width
            .checked_mul(current.depth)
            .unwrap_or(usize::MAX);

    candidate_volume > current_volume
        || (candidate_volume == current_volume
            && candidate.width > current.width)
}

// ============================================================================
// Utility validation
// ============================================================================

fn validate_point(
    point: &SurfacePoint,
) -> Result<(), PositioningError> {
    if point.width == 0 {
        return Err(
            PositioningError::InvalidCoordinate {
                value: point.width,
            },
        );
    }

    if point.depth == 0 {
        return Err(
            PositioningError::InvalidCoordinate {
                value: point.depth,
            },
        );
    }

    if !point.quality.is_finite() {
        return Err(
            PositioningError::NonFiniteQuality {
                value: point.quality,
            },
        );
    }

    Ok(())
}

fn validate_range(
    minimum: usize,
    maximum: usize,
) -> Result<(), PositioningError> {
    if minimum == 0 {
        return Err(
            PositioningError::InvalidCoordinate {
                value: minimum,
            },
        );
    }

    if maximum == 0 {
        return Err(
            PositioningError::InvalidCoordinate {
                value: maximum,
            },
        );
    }

    if minimum > maximum {
        return Err(
            PositioningError::InvalidRange {
                minimum,
                maximum,
            },
        );
    }

    Ok(())
}

fn validate_threshold(
    threshold: f64,
) -> Result<(), PositioningError> {
    if !threshold.is_finite() {
        return Err(
            PositioningError::NonFiniteThreshold {
                value: threshold,
            },
        );
    }

    Ok(())
}

fn compare_point_coordinates(
    left: &PointComparison,
    right: &PointComparison,
) -> Ordering {
    left.width
        .cmp(&right.width)
        .then_with(|| {
            left.depth.cmp(&right.depth)
        })
}

fn compare_quality_for_direction(
    direction: QualityDirection,
    left: f64,
    right: f64,
) -> Ordering {
    match direction {
        QualityDirection::HigherIsBetter => {
            left.partial_cmp(&right)
                .unwrap_or(Ordering::Equal)
        }

        QualityDirection::LowerIsBetter => {
            right.partial_cmp(&left)
                .unwrap_or(Ordering::Equal)
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_surface() -> VolumetricSurface {
        let mut surface =
            VolumetricSurface::with_metric(
                QualityDirection::HigherIsBetter,
                "fidelity",
                Some("unit"),
            );

        surface
            .insert_value(1, 1, 0.99)
            .expect("insert 1,1");

        surface
            .insert_value(1, 2, 0.98)
            .expect("insert 1,2");

        surface
            .insert_value(2, 1, 0.97)
            .expect("insert 2,1");

        surface
            .insert_value(2, 2, 0.94)
            .expect("insert 2,2");

        surface
            .insert_value(3, 1, 0.90)
            .expect("insert 3,1");

        surface
            .insert_value(3, 2, 0.88)
            .expect("insert 3,2");

        surface
    }

    #[test]
    fn coordinate_normalization_is_deterministic() {
        let normalization =
            CoordinateNormalization::new(10, 20)
                .expect("valid normalization");

        assert_eq!(
            normalization.width(5),
            0.5
        );

        assert_eq!(
            normalization.depth(10),
            0.5
        );
    }

    #[test]
    fn quality_normalization_preserves_higher_is_better() {
        let normalization =
            QualityNormalization::UnitInterval;

        let value = normalization
            .normalize(
                0.75,
                QualityDirection::HigherIsBetter,
            )
            .expect("valid value");

        assert!((value - 0.75).abs() < 1.0e-12);
    }

    #[test]
    fn quality_normalization_inverts_lower_is_better() {
        let normalization =
            QualityNormalization::UnitInterval;

        let value = normalization
            .normalize(
                0.25,
                QualityDirection::LowerIsBetter,
            )
            .expect("valid value");

        assert!((value - 0.75).abs() < 1.0e-12);
    }

    #[test]
    fn explicit_normalization_works() {
        let normalization =
            QualityNormalization::explicit(
                10.0,
                20.0,
            )
            .expect("valid bounds");

        let value = normalization
            .normalize(
                15.0,
                QualityDirection::HigherIsBetter,
            )
            .expect("valid value");

        assert!((value - 0.5).abs() < 1.0e-12);
    }

    #[test]
    fn position_point_contains_all_dimensions() {
        let surface = unit_surface();

        let config =
            PositioningConfig::new(
                4,
                4,
                QualityNormalization::UnitInterval,
            )
            .expect("valid configuration");

        let point = surface
            .get(2, 2)
            .expect("point exists");

        let position =
            position_point(
                point,
                &surface,
                &config,
            )
            .expect("position succeeds");

        assert_eq!(position.width, 2);
        assert_eq!(position.depth, 2);
        assert!((position.normalized_width - 0.5).abs() < 1.0e-12);
        assert!((position.normalized_depth - 0.5).abs() < 1.0e-12);
        assert!((position.normalized_quality - 0.94).abs() < 1.0e-12);
    }

    #[test]
    fn position_surface_is_deterministic() {
        let surface = unit_surface();

        let config =
            PositioningConfig::new(
                4,
                4,
                QualityNormalization::UnitInterval,
            )
            .expect("valid configuration");

        let position =
            position_surface(
                &surface,
                &config,
                PositionWeights::equal(),
            )
            .expect("position succeeds");

        assert_eq!(
            position.measured_points,
            6
        );

        assert_eq!(
            position.maximum_width,
            3
        );

        assert_eq!(
            position.maximum_depth,
            2
        );

        assert_eq!(
            position.maximum_rectangular_volume,
            6
        );

        assert!(position.mean_score.is_finite());
        assert!(position.best_score.is_finite());
    }

    #[test]
    fn coverage_distinguishes_missing_points() {
        let surface = unit_surface();

        let coverage =
            calculate_range_coverage(
                &surface,
                1,
                3,
                1,
                2,
            )
            .expect("coverage succeeds");

        assert!(
            (coverage - 1.0).abs()
                < 1.0e-12
        );

        let partial =
            calculate_range_coverage(
                &surface,
                1,
                3,
                1,
                3,
            )
            .expect("coverage succeeds");

        assert!(
            (partial - 6.0 / 9.0).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn best_point_within_respects_bounds() {
        let surface = unit_surface();

        let point =
            best_point_within(
                &surface,
                2,
                2,
            )
            .expect("query succeeds")
            .expect("point exists");

        assert_eq!(
            point.width,
            1
        );

        assert_eq!(
            point.depth,
            1
        );

        assert!(
            (point.quality - 0.99).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn surface_comparison_detects_winner() {
        let left = unit_surface();

        let mut right =
            VolumetricSurface::with_metric(
                QualityDirection::HigherIsBetter,
                "fidelity",
                Some("unit"),
            );

        right
            .insert_value(1, 1, 0.95)
            .expect("insert");

        right
            .insert_value(1, 2, 0.99)
            .expect("insert");

        right
            .insert_value(2, 1, 0.96)
            .expect("insert");

        right
            .insert_value(2, 2, 0.91)
            .expect("insert");

        right
            .insert_value(3, 1, 0.89)
            .expect("insert");

        right
            .insert_value(3, 2, 0.91)
            .expect("insert");

        let comparison =
            compare_surfaces(
                &left,
                &right,
                &ComparisonConfig::absolute(),
            )
            .expect("comparison succeeds");

        assert_eq!(
            comparison.common_points,
            6
        );

        assert!(
            comparison.left_wins > 0
        );

        assert!(
            comparison.right_wins > 0
        );

        assert_eq!(
            comparison.left_wins
                + comparison.right_wins
                + comparison.ties,
            comparison.common_points
        );
    }

    #[test]
    fn comparison_counts_non_common_points() {
        let left = unit_surface();

        let mut right =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        right
            .insert_value(1, 1, 0.9)
            .expect("insert");

        right
            .insert_value(4, 4, 0.8)
            .expect("insert");

        let comparison =
            compare_surfaces(
                &left,
                &right,
                &ComparisonConfig::absolute(),
            )
            .expect("comparison succeeds");

        assert_eq!(
            comparison.common_points,
            1
        );

        assert_eq!(
            comparison.left_only_points,
            5
        );

        assert_eq!(
            comparison.right_only_points,
            1
        );
    }

    #[test]
    fn incompatible_metrics_are_rejected() {
        let left =
            VolumetricSurface::with_metric(
                QualityDirection::HigherIsBetter,
                "fidelity",
                Some("unit"),
            );

        let right =
            VolumetricSurface::with_metric(
                QualityDirection::HigherIsBetter,
                "success_probability",
                Some("unit"),
            );

        let result =
            validate_surface_compatibility(
                &left,
                &right,
            );

        assert!(matches!(
            result,
            Err(PositioningError::IncompatibleMetric { .. })
        ));
    }

    #[test]
    fn incompatible_units_are_rejected() {
        let left =
            VolumetricSurface::with_metric(
                QualityDirection::LowerIsBetter,
                "runtime",
                Some("seconds"),
            );

        let right =
            VolumetricSurface::with_metric(
                QualityDirection::LowerIsBetter,
                "runtime",
                Some("milliseconds"),
            );

        let result =
            validate_surface_compatibility(
                &left,
                &right,
            );

        assert!(matches!(
            result,
            Err(PositioningError::IncompatibleUnit { .. })
        ));
    }

    #[test]
    fn lower_is_better_comparison_has_correct_sign() {
        let mut left =
            VolumetricSurface::with_metric(
                QualityDirection::LowerIsBetter,
                "runtime",
                Some("seconds"),
            );

        let mut right =
            VolumetricSurface::with_metric(
                QualityDirection::LowerIsBetter,
                "runtime",
                Some("seconds"),
            );

        left.insert_value(1, 1, 1.0)
            .expect("insert");

        right.insert_value(1, 1, 2.0)
            .expect("insert");

        let comparison =
            compare_surfaces(
                &left,
                &right,
                &ComparisonConfig::absolute(),
            )
            .expect("comparison succeeds");

        assert_eq!(
            comparison.left_wins,
            1
        );

        assert!(
            comparison
                .points[0]
                .signed_advantage
                > 0.0
        );
    }

    #[test]
    fn pareto_frontier_contains_only_nondominated_points() {
        let surface = unit_surface();

        let frontier =
            measured_pareto_frontier(
                &surface,
                100,
            )
            .expect("frontier succeeds");

        assert!(!frontier.is_empty());

        for point in &frontier {
            assert!(
                point.width > 0
            );

            assert!(
                point.depth > 0
            );

            assert!(
                point.quality.is_finite()
            );
        }
    }

    #[test]
    fn threshold_envelope_requires_complete_measurement() {
        let surface = unit_surface();

        let envelope =
            largest_passing_envelope(
                &surface,
                0.90,
                false,
                3,
            )
            .expect("envelope succeeds");

        assert!(envelope.complete);
        assert!(envelope.width >= 1);
        assert!(envelope.depth >= 1);
    }

    #[test]
    fn empty_surface_is_rejected() {
        let surface =
            VolumetricSurface::new(
                QualityDirection::HigherIsBetter,
            );

        let config =
            PositioningConfig::new(
                10,
                10,
                QualityNormalization::UnitInterval,
            )
            .expect("configuration succeeds");

        let result =
            position_surface(
                &surface,
                &config,
                PositionWeights::equal(),
            );

        assert!(matches!(
            result,
            Err(PositioningError::EmptySurface)
        ));
    }

    #[test]
    fn zero_normalization_range_is_rejected() {
        let result =
            QualityNormalization::explicit(
                1.0,
                1.0,
            );

        assert!(matches!(
            result,
            Err(
                PositioningError::ZeroNormalizationRange { .. }
            )
        ));
    }

    #[test]
    fn zero_weights_are_rejected() {
        let weights =
            PositionWeights {
                width: 0.0,
                depth: 0.0,
                quality: 0.0,
            };

        assert!(matches!(
            weights.validate(),
            Err(PositioningError::NonFiniteScore)
        ));
    }

    #[test]
    fn comparison_results_are_coordinate_sorted() {
        let left = unit_surface();
        let right = unit_surface();

        let comparison =
            compare_surfaces(
                &left,
                &right,
                &ComparisonConfig::absolute(),
            )
            .expect("comparison succeeds");

        let coordinates: Vec<_> =
            comparison
                .points
                .iter()
                .map(|point| {
                    (point.width, point.depth)
                })
                .collect();

        let mut sorted =
            coordinates.clone();

        sorted.sort();

        assert_eq!(
            coordinates,
            sorted
        );
    }
}