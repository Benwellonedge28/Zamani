//! Zamani Quantum Benchmarking — Bottleneck Analysis
//!
//! Production-grade identification of the limiting dimensions of a quantum
//! computing workload.
//!
//! # Purpose
//!
//! This module answers:
//!
//! > "Which measured dimension is currently limiting the benchmarked system?"
//!
//! It deliberately does NOT:
//!
//! - execute quantum circuits;
//! - generate circuits;
//! - select a backend;
//! - compile or route circuits;
//! - mutate hardware state;
//! - perform benchmark protocol execution;
//! - define universal hardware thresholds;
//! - invent acceptable performance limits;
//! - compare two benchmark runs;
//! - store historical baselines;
//! - print diagnostics;
//! - maintain process-global state.
//!
//! Those responsibilities belong to:
//!
//! - `core::*` for canonical benchmark data;
//! - `execution::*` for execution;
//! - `protocols::*` for benchmark protocols;
//! - `analysis::baseline` for baseline comparison;
//! - `analysis::regression` for regression detection;
//! - `analysis::attribution` for causal attribution;
//! - `analysis::diagnosis` for higher-level diagnosis;
//! - `reporting::*` for presentation.
//!
//! # Architectural position
//!
//! ```text
//! benchmark execution
//!        │
//!        ▼
//! core::result::BenchmarkResult
//!        │
//!        ▼
//! core::metric::Metric
//!        │
//!        ▼
//! analysis::bottleneck
//!        │
//!        ├── limiting metric
//!        ├── severity
//!        ├── confidence
//!        ├── diagnostic evidence
//!        └── ranked bottleneck list
//! ```
//!
//! `bottleneck.rs` intentionally operates on `Metric` values rather than
//! depending directly on `BenchmarkResult`. This keeps the analyzer usable
//! for:
//!
//! - complete benchmark results;
//! - partial results;
//! - individual protocol analyses;
//! - synthetic test fixtures;
//! - streaming analysis;
//! - future result schemas;
//! - Zamani-language custom benchmarks.
//!
//! Higher-level integration can obtain metrics from `BenchmarkResult` and pass
//! them here without changing this file.
//!
//! # Critical semantic rule
//!
//! A metric is NOT automatically a bottleneck merely because its numeric value
//! is "large" or "small".
//!
//! For example:
//!
//! - a large qubit count is usually good;
//! - a large throughput is usually good;
//! - a large error rate is bad;
//! - a large latency is bad;
//! - a large fidelity is good;
//! - a large energy consumption may be bad;
//! - a large objective value may be good or bad depending on the workload.
//!
//! Therefore this module requires an explicit `BottleneckPolicy` containing
//! acceptable and critical limits.
//!
//! It never invents universal thresholds.
//!
//! # Severity model
//!
//! For a metric where lower values are better:
//!
//! ```text
//! value <= acceptable       => severity 0
//! value >= critical         => severity 1
//! acceptable < value < critical
//!                         => linear interpolation
//! ```
//!
//! For a metric where higher values are better:
//!
//! ```text
//! value >= acceptable       => severity 0
//! value <= critical         => severity 1
//! critical < value < acceptable
//!                         => linear interpolation
//! ```
//!
//! The resulting score is in `[0, 1]`.
//!
//! `0` means the metric is within the acceptable region.
//!
//! `1` means the metric has reached or exceeded the critical boundary.
//!
//! # Confidence handling
//!
//! A metric with an uncertainty interval that already crosses the acceptable
//! boundary must not be presented as an unqualified bottleneck.
//!
//! The analyzer therefore records:
//!
//! - point-estimate severity;
//! - worst-case severity from the confidence interval;
//! - best-case severity from the confidence interval;
//! - whether the bottleneck classification is statistically robust;
//! - whether the metric has confidence information.
//!
//! This avoids turning noisy measurements into false engineering conclusions.
//!
//! # Quality handling
//!
//! Metrics marked `Invalid` are rejected.
//!
//! Metrics marked `Uncertain` or `Approximate` may still be analyzed, but the
//! resulting finding is explicitly marked as uncertain.
//!
//! Metrics marked `Estimated` or `Fitted` remain valid inputs; their quality is
//! preserved in the finding.
//!
//! # Duplicate metrics
//!
//! Multiple measurements of the same metric kind are legal. They may represent:
//!
//! - different qubit groups;
//! - different circuit widths;
//! - different depths;
//! - different workload instances;
//! - different calibration windows;
//! - different execution phases.
//!
//! Therefore this module does NOT silently deduplicate metrics.
//!
//! Instead, every input metric produces an independently identifiable
//! candidate, and ranking is performed over candidates.
//!
//! A caller that wants aggregation must use the appropriate statistics/metric
//! layer before invoking this analyzer.
//!
//! # Security/resource safety
//!
//! This module is designed to process untrusted benchmark-result data safely.
//!
//! It enforces:
//!
//! - bounded metric count;
//! - bounded policy count;
//! - bounded identifier lengths;
//! - finite numeric values;
//! - valid threshold ordering;
//! - finite weights;
//! - finite severity values;
//! - deterministic ranking;
//! - no allocations proportional to untrusted strings beyond configured
//!   limits;
//! - no recursion;
//! - no unsafe code;
//! - no process-global state.
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
//! This file depends only on:
//!
//! ```text
//! core::metric
//! ```
//!
//! Specifically:
//!
//! - `Metric`
//! - `MetricDirection`
//! - `MetricKind`
//! - `MetricQuality`
//! - `MetricUnit`
//! - `FiniteF64`
//!
//! It does NOT depend on `BenchmarkResult`, `baseline`, `regression`,
//! `attribution`, `diagnosis`, reporting, execution, protocols, hardware, or
//! the Quantum IR.
//!
//! Consequently this file can be completed before those modules are finished.
//!
//! Later integration is one-way:
//!
//! ```text
//! core::result::BenchmarkResult
//!             │
//!             ▼
//! analysis::bottleneck::BottleneckAnalyzer
//!             │
//!             ▼
//! analysis::diagnosis
//!             │
//!             ▼
//! reporting
//! ```
//!
//! No modification of this file should be required when those modules are
//! integrated, provided they consume the public API defined here.
//!
//! # Scientific limitation
//!
//! Bottleneck analysis is a diagnostic ranking mechanism, not causal proof.
//!
//! If latency is the strongest limiting metric, this module may identify
//! latency as the bottleneck. It does NOT claim that a specific hardware,
//! compiler, routing or scheduling component caused that latency.
//!
//! Causal attribution belongs to `analysis::attribution`.
//!
//! ---------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::fmt;

use super::super::core::metric::{
    FiniteF64,
    Metric,
    MetricDirection,
    MetricKind,
    MetricQuality,
    MetricUnit,
};

// =============================================================================
// Public constants
// =============================================================================

/// Stable component identifier.
pub const BOTTLENECK_ANALYSIS_COMPONENT_ID: &str =
    "zamani.quantum.benchmark.analysis.bottleneck";

/// Stable API/schema version.
///
/// Increment the major version if the meaning of a serialized bottleneck
/// finding changes incompatibly.
pub const BOTTLENECK_ANALYSIS_VERSION: &str = "1.0.0";

/// Maximum number of metrics accepted by one analysis invocation.
///
/// This prevents accidental or malicious unbounded analysis workloads.
pub const MAX_BOTTLENECK_METRICS: usize = 16_384;

/// Maximum number of configured metric limits.
pub const MAX_BOTTLENECK_LIMITS: usize = 16_384;

/// Maximum metric identifier length in bytes.
pub const MAX_METRIC_ID_LENGTH: usize = 256;

/// Maximum diagnostic-code length.
pub const MAX_DIAGNOSTIC_CODE_LENGTH: usize = 128;

/// Maximum human-readable description length.
pub const MAX_DESCRIPTION_LENGTH: usize = 4_096;

/// Maximum number of ranked findings returned.
pub const MAX_RETURNED_FINDINGS: usize = 16_384;

/// Small tolerance used only for floating-point boundary comparisons.
const NUMERIC_EPSILON: f64 = 1.0e-12;

/// Minimum legal positive weight.
const MIN_WEIGHT: f64 = 0.0;

/// Maximum supported weight.
///
/// Weights are normalized during scoring, so extremely large values have no
/// scientific advantage and can amplify numerical instability.
const MAX_WEIGHT: f64 = 1.0e12;

// =============================================================================
// Error model
// =============================================================================

/// Errors produced by bottleneck analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum BottleneckError {
    /// No metrics were supplied.
    EmptyMetrics,

    /// Too many metrics were supplied.
    TooManyMetrics {
        /// Number supplied.
        count: usize,

        /// Maximum permitted.
        maximum: usize,
    },

    /// No bottleneck policy limits were supplied.
    EmptyPolicy,

    /// Too many limits were supplied.
    TooManyLimits {
        /// Number supplied.
        count: usize,

        /// Maximum permitted.
        maximum: usize,
    },

    /// A metric identifier is empty or too long.
    InvalidMetricIdentifier {
        /// Identifier field.
        identifier: String,
    },

    /// A limit has an invalid numeric value.
    InvalidLimitValue {
        /// Metric identifier.
        metric_id: String,

        /// Supplied value.
        value: f64,
    },

    /// Acceptable and critical thresholds are ordered incorrectly.
    InvalidThresholdOrder {
        /// Metric identifier.
        metric_id: String,

        /// Acceptable threshold.
        acceptable: f64,

        /// Critical threshold.
        critical: f64,
    },

    /// A weight is invalid.
    InvalidWeight {
        /// Metric identifier.
        metric_id: String,

        /// Supplied weight.
        weight: f64,
    },

    /// A metric value is non-finite.
    NonFiniteMetricValue {
        /// Metric identifier.
        metric_id: String,
    },

    /// A metric marked invalid cannot participate in bottleneck analysis.
    InvalidMetricQuality {
        /// Metric identifier.
        metric_id: String,
    },

    /// The configured unit does not match the metric.
    UnitMismatch {
        /// Metric identifier.
        metric_id: String,
    },

    /// An internal numerical result became non-finite.
    NonFiniteScore {
        /// Metric identifier.
        metric_id: String,
    },

    /// The sum of configured weights is not usable.
    InvalidTotalWeight,
}

impl fmt::Display for BottleneckError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMetrics => {
                formatter.write_str("bottleneck analysis requires at least one metric")
            }

            Self::TooManyMetrics { count, maximum } => {
                write!(
                    formatter,
                    "bottleneck analysis received {} metrics; maximum is {}",
                    count, maximum
                )
            }

            Self::EmptyPolicy => {
                formatter.write_str(
                    "bottleneck analysis requires at least one configured metric limit",
                )
            }

            Self::TooManyLimits { count, maximum } => {
                write!(
                    formatter,
                    "bottleneck policy contains {} limits; maximum is {}",
                    count, maximum
                )
            }

            Self::InvalidMetricIdentifier { identifier } => {
                write!(
                    formatter,
                    "invalid bottleneck metric identifier `{}`",
                    identifier
                )
            }

            Self::InvalidLimitValue { metric_id, value } => {
                write!(
                    formatter,
                    "bottleneck limit for `{}` contains non-finite value {}",
                    metric_id, value
                )
            }

            Self::InvalidThresholdOrder {
                metric_id,
                acceptable,
                critical,
            } => {
                write!(
                    formatter,
                    "invalid bottleneck thresholds for `{}`: acceptable={}, critical={}",
                    metric_id, acceptable, critical
                )
            }

            Self::InvalidWeight { metric_id, weight } => {
                write!(
                    formatter,
                    "invalid bottleneck weight for `{}`: {}",
                    metric_id, weight
                )
            }

            Self::NonFiniteMetricValue { metric_id } => {
                write!(
                    formatter,
                    "metric `{}` contains a non-finite value",
                    metric_id
                )
            }

            Self::InvalidMetricQuality { metric_id } => {
                write!(
                    formatter,
                    "metric `{}` is marked invalid and cannot be analyzed",
                    metric_id
                )
            }

            Self::UnitMismatch { metric_id } => {
                write!(
                    formatter,
                    "metric `{}` uses a unit incompatible with its bottleneck policy",
                    metric_id
                )
            }

            Self::NonFiniteScore { metric_id } => {
                write!(
                    formatter,
                    "bottleneck severity for `{}` became non-finite",
                    metric_id
                )
            }

            Self::InvalidTotalWeight => {
                formatter.write_str(
                    "bottleneck policy has no usable positive metric weight",
                )
            }
        }
    }
}

impl std::error::Error for BottleneckError {}

// =============================================================================
// Bottleneck policy
// =============================================================================

/// Explicit acceptable/critical limits for one metric.
///
/// The policy deliberately does not assume universal quantum-computing
/// thresholds.
///
/// A laboratory, hardware provider, CI system, benchmark protocol, or Zamani
/// application must provide limits appropriate to its workload.
///
/// # Direction semantics
///
/// For `LowerIsBetter`:
///
/// ```text
/// value <= acceptable -> severity 0
/// value >= critical   -> severity 1
/// ```
///
/// For `HigherIsBetter`:
///
/// ```text
/// value >= acceptable -> severity 0
/// value <= critical   -> severity 1
/// ```
///
/// `Neutral` metrics cannot be scored as bottlenecks and should normally not
/// have a limit configured.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricLimit {
    /// Stable metric identifier, normally `MetricKind::id()`.
    pub metric_id: String,

    /// Expected metric unit.
    pub unit: MetricUnit,

    /// Metric optimization direction.
    pub direction: MetricDirection,

    /// Value at or better than which the metric is acceptable.
    pub acceptable: FiniteF64,

    /// Value at or beyond which the metric is critical.
    pub critical: FiniteF64,

    /// Relative importance of this metric when calculating aggregate pressure.
    pub weight: FiniteF64,

    /// Optional diagnostic code.
    pub diagnostic_code: String,

    /// Human-readable description of why this metric matters.
    pub description: String,
}

impl MetricLimit {
    /// Creates a validated metric limit.
    pub fn new(
        metric_id: impl Into<String>,
        unit: MetricUnit,
        direction: MetricDirection,
        acceptable: f64,
        critical: f64,
    ) -> Result<Self, BottleneckError> {
        Self::with_weight(
            metric_id,
            unit,
            direction,
            acceptable,
            critical,
            1.0,
        )
    }

    /// Creates a validated metric limit with an explicit weight.
    pub fn with_weight(
        metric_id: impl Into<String>,
        unit: MetricUnit,
        direction: MetricDirection,
        acceptable: f64,
        critical: f64,
        weight: f64,
    ) -> Result<Self, BottleneckError> {
        let metric_id = metric_id.into();

        validate_identifier(&metric_id)?;

        if !acceptable.is_finite() {
            return Err(BottleneckError::InvalidLimitValue {
                metric_id: metric_id.clone(),
                value: acceptable,
            });
        }

        if !critical.is_finite() {
            return Err(BottleneckError::InvalidLimitValue {
                metric_id: metric_id.clone(),
                value: critical,
            });
        }

        if !weight.is_finite()
            || weight < MIN_WEIGHT
            || weight > MAX_WEIGHT
        {
            return Err(BottleneckError::InvalidWeight {
                metric_id: metric_id.clone(),
                weight,
            });
        }

        match direction {
            MetricDirection::LowerIsBetter => {
                if acceptable > critical {
                    return Err(BottleneckError::InvalidThresholdOrder {
                        metric_id,
                        acceptable,
                        critical,
                    });
                }
            }

            MetricDirection::HigherIsBetter => {
                if acceptable < critical {
                    return Err(BottleneckError::InvalidThresholdOrder {
                        metric_id,
                        acceptable,
                        critical,
                    });
                }
            }

            MetricDirection::Neutral => {
                // Neutral metrics are allowed in a policy so that the policy
                // can describe them, but they will not produce severity.
            }
        }

        let diagnostic_code =
            format!("bottleneck.{}", sanitize_diagnostic_component(&metric_id));

        let description =
            format!("Performance pressure for metric `{}`.", metric_id);

        Ok(Self {
            metric_id,
            unit,
            direction,
            acceptable: FiniteF64::new(acceptable)
                .expect("validated finite acceptable threshold"),
            critical: FiniteF64::new(critical)
                .expect("validated finite critical threshold"),
            weight: FiniteF64::new(weight)
                .expect("validated finite weight"),
            diagnostic_code,
            description,
        })
    }

    /// Sets a diagnostic code.
    pub fn with_diagnostic_code(
        mut self,
        diagnostic_code: impl Into<String>,
    ) -> Result<Self, BottleneckError> {
        let diagnostic_code = diagnostic_code.into();

        if diagnostic_code.trim().is_empty()
            || diagnostic_code.len() > MAX_DIAGNOSTIC_CODE_LENGTH
        {
            return Err(BottleneckError::InvalidMetricIdentifier {
                identifier: diagnostic_code,
            });
        }

        self.diagnostic_code = diagnostic_code;
        Ok(self)
    }

    /// Sets a human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, BottleneckError> {
        let description = description.into();

        if description.trim().is_empty()
            || description.len() > MAX_DESCRIPTION_LENGTH
        {
            return Err(BottleneckError::InvalidMetricIdentifier {
                identifier: description,
            });
        }

        self.description = description;
        Ok(self)
    }

    /// Returns the metric identifier.
    #[inline]
    pub fn metric_id(&self) -> &str {
        &self.metric_id
    }

    /// Returns the configured direction.
    #[inline]
    pub const fn direction(&self) -> MetricDirection {
        self.direction
    }

    /// Returns the configured weight.
    #[inline]
    pub fn weight(&self) -> f64 {
        self.weight.get()
    }

    /// Calculates normalized severity for a value.
    ///
    /// Returns a value in `[0, 1]`.
    pub fn severity(&self, value: f64) -> Result<f64, BottleneckError> {
        if !value.is_finite() {
            return Err(BottleneckError::InvalidLimitValue {
                metric_id: self.metric_id.clone(),
                value,
            });
        }

        let acceptable = self.acceptable.get();
        let critical = self.critical.get();

        let severity = match self.direction {
            MetricDirection::LowerIsBetter => {
                if value <= acceptable {
                    0.0
                } else if value >= critical {
                    1.0
                } else {
                    interpolate(value, acceptable, critical)
                }
            }

            MetricDirection::HigherIsBetter => {
                if value >= acceptable {
                    0.0
                } else if value <= critical {
                    1.0
                } else {
                    interpolate(value, acceptable, critical)
                }
            }

            MetricDirection::Neutral => 0.0,
        };

        if !severity.is_finite() {
            return Err(BottleneckError::NonFiniteScore {
                metric_id: self.metric_id.clone(),
            });
        }

        Ok(clamp_unit_interval(severity))
    }

    /// Returns whether a value is inside the acceptable region.
    pub fn is_acceptable(&self, value: f64) -> bool {
        match self.direction {
            MetricDirection::LowerIsBetter => {
                value <= self.acceptable.get()
            }

            MetricDirection::HigherIsBetter => {
                value >= self.acceptable.get()
            }

            MetricDirection::Neutral => true,
        }
    }

    /// Returns whether a value is at or beyond the critical region.
    pub fn is_critical(&self, value: f64) -> bool {
        match self.direction {
            MetricDirection::LowerIsBetter => {
                value >= self.critical.get()
            }

            MetricDirection::HigherIsBetter => {
                value <= self.critical.get()
            }

            MetricDirection::Neutral => false,
        }
    }
}

// =============================================================================
// Policy
// =============================================================================

/// Complete bottleneck-analysis policy.
///
/// The policy is intentionally explicit: the analyzer cannot claim that one
/// dimension is a bottleneck unless a caller has defined what "bad" means for
/// that dimension.
#[derive(Debug, Clone, PartialEq)]
pub struct BottleneckPolicy {
    /// Metric-specific limits.
    limits: Vec<MetricLimit>,

    /// Minimum severity required for a metric to be considered a bottleneck.
    bottleneck_threshold: f64,

    /// Maximum number of findings returned.
    max_findings: usize,

    /// Whether uncertain/approximate metrics may participate.
    include_uncertain_metrics: bool,
}

impl BottleneckPolicy {
    /// Creates a policy containing one limit.
    pub fn new(limit: MetricLimit) -> Result<Self, BottleneckError> {
        Self::with_limits(vec![limit])
    }

    /// Creates a policy from multiple metric limits.
    pub fn with_limits(
        limits: Vec<MetricLimit>,
    ) -> Result<Self, BottleneckError> {
        if limits.is_empty() {
            return Err(BottleneckError::EmptyPolicy);
        }

        if limits.len() > MAX_BOTTLENECK_LIMITS {
            return Err(BottleneckError::TooManyLimits {
                count: limits.len(),
                maximum: MAX_BOTTLENECK_LIMITS,
            });
        }

        for limit in &limits {
            validate_identifier(&limit.metric_id)?;
        }

        Ok(Self {
            limits,
            bottleneck_threshold: 0.5,
            max_findings: MAX_RETURNED_FINDINGS,
            include_uncertain_metrics: true,
        })
    }

    /// Sets the minimum severity at which a finding is classified as a
    /// bottleneck.
    ///
    /// The value must be in `[0, 1]`.
    pub fn with_bottleneck_threshold(
        mut self,
        threshold: f64,
    ) -> Result<Self, BottleneckError> {
        if !threshold.is_finite()
            || !(0.0..=1.0).contains(&threshold)
        {
            return Err(BottleneckError::InvalidLimitValue {
                metric_id: "bottleneck_threshold".to_owned(),
                value: threshold,
            });
        }

        self.bottleneck_threshold = threshold;
        Ok(self)
    }

    /// Sets the maximum number of findings returned.
    pub fn with_max_findings(
        mut self,
        max_findings: usize,
    ) -> Result<Self, BottleneckError> {
        if max_findings == 0
            || max_findings > MAX_RETURNED_FINDINGS
        {
            return Err(BottleneckError::TooManyLimits {
                count: max_findings,
                maximum: MAX_RETURNED_FINDINGS,
            });
        }

        self.max_findings = max_findings;
        Ok(self)
    }

    /// Controls whether uncertain/approximate metrics are included.
    #[must_use]
    pub fn include_uncertain_metrics(
        mut self,
        include: bool,
    ) -> Self {
        self.include_uncertain_metrics = include;
        self
    }

    /// Returns all configured limits.
    #[inline]
    pub fn limits(&self) -> &[MetricLimit] {
        &self.limits
    }

    /// Returns the bottleneck threshold.
    #[inline]
    pub const fn bottleneck_threshold(&self) -> f64 {
        self.bottleneck_threshold
    }

    /// Returns the maximum number of findings.
    #[inline]
    pub const fn max_findings(&self) -> usize {
        self.max_findings
    }

    /// Returns whether uncertain metrics are included.
    #[inline]
    pub const fn includes_uncertain_metrics(&self) -> bool {
        self.include_uncertain_metrics
    }

    /// Finds the configured limit for a metric.
    pub fn limit_for(&self, metric_id: &str) -> Option<&MetricLimit> {
        self.limits
            .iter()
            .find(|limit| limit.metric_id == metric_id)
    }

    /// Validates the complete policy.
    pub fn validate(&self) -> Result<(), BottleneckError> {
        if self.limits.is_empty() {
            return Err(BottleneckError::EmptyPolicy);
        }

        if self.limits.len() > MAX_BOTTLENECK_LIMITS {
            return Err(BottleneckError::TooManyLimits {
                count: self.limits.len(),
                maximum: MAX_BOTTLENECK_LIMITS,
            });
        }

        if !self.bottleneck_threshold.is_finite()
            || !(0.0..=1.0).contains(&self.bottleneck_threshold)
        {
            return Err(BottleneckError::InvalidLimitValue {
                metric_id: "bottleneck_threshold".to_owned(),
                value: self.bottleneck_threshold,
            });
        }

        if self.max_findings == 0
            || self.max_findings > MAX_RETURNED_FINDINGS
        {
            return Err(BottleneckError::TooManyLimits {
                count: self.max_findings,
                maximum: MAX_RETURNED_FINDINGS,
            });
        }

        for limit in &self.limits {
            validate_identifier(&limit.metric_id)?;

            if !limit.acceptable.get().is_finite()
                || !limit.critical.get().is_finite()
            {
                return Err(BottleneckError::InvalidLimitValue {
                    metric_id: limit.metric_id.clone(),
                    value: f64::NAN,
                });
            }

            if !limit.weight.get().is_finite()
                || limit.weight.get() < 0.0
                || limit.weight.get() > MAX_WEIGHT
            {
                return Err(BottleneckError::InvalidWeight {
                    metric_id: limit.metric_id.clone(),
                    weight: limit.weight.get(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Classification
// =============================================================================

/// Scientific classification of a bottleneck finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BottleneckClassification {
    /// Metric is within its acceptable region.
    Healthy,

    /// Metric shows measurable pressure but is below the bottleneck threshold.
    Watch,

    /// Metric is sufficiently degraded to be considered a bottleneck.
    Bottleneck,

    /// Metric has reached the configured critical region.
    Critical,

    /// Metric could not be classified reliably because its uncertainty spans
    /// materially different regions.
    Uncertain,

    /// No policy was supplied for the metric.
    Unconfigured,
}

impl BottleneckClassification {
    /// Returns a stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Watch => "watch",
            Self::Bottleneck => "bottleneck",
            Self::Critical => "critical",
            Self::Uncertain => "uncertain",
            Self::Unconfigured => "unconfigured",
        }
    }

    /// Returns whether this finding represents an actionable bottleneck.
    #[must_use]
    pub const fn is_bottleneck(self) -> bool {
        matches!(
            self,
            Self::Bottleneck | Self::Critical
        )
    }
}

impl fmt::Display for BottleneckClassification {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Confidence state
// =============================================================================

/// Statistical confidence state of a bottleneck finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BottleneckConfidence {
    /// Metric has no confidence interval.
    Unspecified,

    /// Confidence interval remains in the same broad classification.
    Robust,

    /// Confidence interval crosses an important classification boundary.
    BoundaryCrossing,

    /// Metric quality is explicitly uncertain or approximate.
    LowQuality,
}

impl BottleneckConfidence {
    /// Returns a stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unspecified => "unspecified",
            Self::Robust => "robust",
            Self::BoundaryCrossing => "boundary_crossing",
            Self::LowQuality => "low_quality",
        }
    }
}

// =============================================================================
// Finding
// =============================================================================

/// Complete bottleneck finding for one metric.
#[derive(Debug, Clone, PartialEq)]
pub struct BottleneckFinding {
    /// Stable metric identifier.
    pub metric_id: String,

    /// Original metric kind.
    pub kind: MetricKind,

    /// Unit of the metric.
    pub unit: MetricUnit,

    /// Measured point estimate.
    pub value: FiniteF64,

    /// Point-estimate severity in `[0, 1]`.
    pub severity: FiniteF64,

    /// Best-case severity from the confidence interval.
    pub best_case_severity: FiniteF64,

    /// Worst-case severity from the confidence interval.
    pub worst_case_severity: FiniteF64,

    /// Weighted contribution to aggregate bottleneck pressure.
    pub weighted_pressure: FiniteF64,

    /// Classification of this metric.
    pub classification: BottleneckClassification,

    /// Statistical confidence state.
    pub confidence: BottleneckConfidence,

    /// Metric quality inherited from the source metric.
    pub quality: MetricQuality,

    /// Optimization direction.
    pub direction: MetricDirection,

    /// Configured acceptable boundary.
    pub acceptable: FiniteF64,

    /// Configured critical boundary.
    pub critical: FiniteF64,

    /// Configured importance weight.
    pub weight: FiniteF64,

    /// Stable diagnostic code.
    pub diagnostic_code: String,

    /// Human-readable explanation.
    pub explanation: String,

    /// Original metric sample count.
    pub sample_count: Option<u64>,

    /// Original shot count.
    pub shot_count: Option<u64>,

    /// Original circuit count.
    pub circuit_count: Option<u64>,
}

impl BottleneckFinding {
    /// Returns whether this finding is an actionable bottleneck.
    #[must_use]
    pub const fn is_bottleneck(&self) -> bool {
        self.classification.is_bottleneck()
    }

    /// Returns whether this finding is critical.
    #[must_use]
    pub const fn is_critical(&self) -> bool {
        matches!(
            self.classification,
            BottleneckClassification::Critical
        )
    }
}

// =============================================================================
// Analysis summary
// =============================================================================

/// Summary of one bottleneck-analysis invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct BottleneckSummary {
    /// Total number of supplied metrics.
    pub metric_count: usize,

    /// Number of metrics with configured limits.
    pub configured_metric_count: usize,

    /// Number of metrics classified as bottlenecks.
    pub bottleneck_count: usize,

    /// Number of critical metrics.
    pub critical_count: usize,

    /// Number of healthy metrics.
    pub healthy_count: usize,

    /// Number of watch-level metrics.
    pub watch_count: usize,

    /// Number of uncertain metrics.
    pub uncertain_count: usize,

    /// Number of unconfigured metrics.
    pub unconfigured_count: usize,

    /// Aggregate weighted bottleneck pressure in `[0, 1]`.
    ///
    /// This is a portfolio pressure indicator, not a scientific benchmark
    /// score and must not be compared across policies without policy
    /// compatibility.
    pub aggregate_pressure: FiniteF64,

    /// Index into `findings` of the highest-pressure finding.
    pub primary_bottleneck_index: Option<usize>,
}

impl BottleneckSummary {
    /// Returns whether at least one actionable bottleneck exists.
    #[must_use]
    pub const fn has_bottleneck(&self) -> bool {
        self.bottleneck_count > 0
    }
}

/// Complete bottleneck-analysis result.
#[derive(Debug, Clone, PartialEq)]
pub struct BottleneckAnalysis {
    /// Analyzer component version.
    pub analyzer_version: &'static str,

    /// Ranked findings.
    ///
    /// The first finding is the highest-pressure finding.
    pub findings: Vec<BottleneckFinding>,

    /// Aggregate analysis summary.
    pub summary: BottleneckSummary,
}

impl BottleneckAnalysis {
    /// Returns the primary bottleneck, if one exists.
    pub fn primary_bottleneck(&self) -> Option<&BottleneckFinding> {
        self.summary
            .primary_bottleneck_index
            .and_then(|index| self.findings.get(index))
    }

    /// Returns only actionable bottlenecks.
    pub fn bottlenecks(&self) -> impl Iterator<Item = &BottleneckFinding> {
        self.findings.iter().filter(|finding| finding.is_bottleneck())
    }

    /// Returns whether at least one actionable bottleneck exists.
    #[must_use]
    pub const fn has_bottleneck(&self) -> bool {
        self.summary.has_bottleneck()
    }
}

// =============================================================================
// Analyzer
// =============================================================================

/// Stateless bottleneck analyzer.
///
/// It owns no mutable global state and can safely be reused across benchmark
/// runs.
#[derive(Debug, Clone, Copy, Default)]
pub struct BottleneckAnalyzer;

impl BottleneckAnalyzer {
    /// Creates a bottleneck analyzer.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Analyzes a metric collection against an explicit policy.
    ///
    /// This is the primary API for integration with `BenchmarkResult`.
    pub fn analyze(
        &self,
        metrics: &[Metric],
        policy: &BottleneckPolicy,
    ) -> Result<BottleneckAnalysis, BottleneckError> {
        if metrics.is_empty() {
            return Err(BottleneckError::EmptyMetrics);
        }

        if metrics.len() > MAX_BOTTLENECK_METRICS {
            return Err(BottleneckError::TooManyMetrics {
                count: metrics.len(),
                maximum: MAX_BOTTLENECK_METRICS,
            });
        }

        policy.validate()?;

        let total_weight = policy
            .limits
            .iter()
            .map(MetricLimit::weight)
            .filter(|weight| *weight > 0.0)
            .sum::<f64>();

        if !total_weight.is_finite() || total_weight <= 0.0 {
            return Err(BottleneckError::InvalidTotalWeight);
        }

        let mut findings = Vec::with_capacity(metrics.len());

        for metric in metrics {
            if metric.quality == MetricQuality::Invalid {
                return Err(BottleneckError::InvalidMetricQuality {
                    metric_id: metric.kind.id(),
                });
            }

            let metric_id = metric.kind.id();

            if metric_id.len() > MAX_METRIC_ID_LENGTH {
                return Err(BottleneckError::InvalidMetricIdentifier {
                    identifier: metric_id,
                });
            }

            let value = metric.value.get();

            if !value.is_finite() {
                return Err(BottleneckError::NonFiniteMetricValue {
                    metric_id,
                });
            }

            let Some(limit) = policy.limit_for(&metric_id) else {
                findings.push(self.unconfigured_finding(metric));
                continue;
            };

            if metric.unit != limit.unit {
                return Err(BottleneckError::UnitMismatch {
                    metric_id,
                });
            }

            if !policy.includes_uncertain_metrics()
                && matches!(
                    metric.quality,
                    MetricQuality::Uncertain
                        | MetricQuality::Approximate
                )
            {
                continue;
            }

            let finding =
                self.analyze_metric(metric, limit, total_weight, policy)?;

            findings.push(finding);
        }

        findings.sort_by(compare_findings);

        if findings.len() > policy.max_findings() {
            findings.truncate(policy.max_findings());
        }

        let summary = build_summary(
            metrics.len(),
            &findings,
            total_weight,
        )?;

        Ok(BottleneckAnalysis {
            analyzer_version: BOTTLENECK_ANALYSIS_VERSION,
            findings,
            summary,
        })
    }

    /// Analyzes a single metric.
    ///
    /// This is useful for streaming or incremental benchmark analysis.
    pub fn analyze_one(
        &self,
        metric: &Metric,
        policy: &BottleneckPolicy,
    ) -> Result<Option<BottleneckFinding>, BottleneckError> {
        policy.validate()?;

        if metric.quality == MetricQuality::Invalid {
            return Err(BottleneckError::InvalidMetricQuality {
                metric_id: metric.kind.id(),
            });
        }

        if !policy.includes_uncertain_metrics()
            && matches!(
                metric.quality,
                MetricQuality::Uncertain
                    | MetricQuality::Approximate
            )
        {
            return Ok(None);
        }

        let metric_id = metric.kind.id();

        let Some(limit) = policy.limit_for(&metric_id) else {
            return Ok(Some(self.unconfigured_finding(metric)));
        };

        if metric.unit != limit.unit {
            return Err(BottleneckError::UnitMismatch {
                metric_id,
            });
        }

        let total_weight = policy
            .limits
            .iter()
            .map(MetricLimit::weight)
            .filter(|weight| *weight > 0.0)
            .sum::<f64>();

        if !total_weight.is_finite() || total_weight <= 0.0 {
            return Err(BottleneckError::InvalidTotalWeight);
        }

        Ok(Some(self.analyze_metric(
            metric,
            limit,
            total_weight,
            policy,
        )?))
    }

    fn analyze_metric(
        &self,
        metric: &Metric,
        limit: &MetricLimit,
        total_weight: f64,
        policy: &BottleneckPolicy,
    ) -> Result<BottleneckFinding, BottleneckError> {
        let value = metric.value.get();

        let severity = limit.severity(value)?;

        let (best_case_severity, worst_case_severity, confidence) =
            confidence_severity(metric, limit)?;

        let weighted_pressure =
            severity * (limit.weight() / total_weight);

        if !weighted_pressure.is_finite() {
            return Err(BottleneckError::NonFiniteScore {
                metric_id: limit.metric_id.clone(),
            });
        }

        let classification = classify(
            severity,
            best_case_severity,
            worst_case_severity,
            metric,
            limit,
            policy.bottleneck_threshold(),
        );

        let explanation = build_explanation(
            metric,
            limit,
            severity,
            best_case_severity,
            worst_case_severity,
            classification,
            confidence,
        );

        Ok(BottleneckFinding {
            metric_id: limit.metric_id.clone(),
            kind: metric.kind.clone(),
            unit: metric.unit.clone(),
            value: metric.value,
            severity: finite_score(
                severity,
                &limit.metric_id,
            )?,
            best_case_severity: finite_score(
                best_case_severity,
                &limit.metric_id,
            )?,
            worst_case_severity: finite_score(
                worst_case_severity,
                &limit.metric_id,
            )?,
            weighted_pressure: finite_score(
                weighted_pressure,
                &limit.metric_id,
            )?,
            classification,
            confidence,
            quality: metric.quality,
            direction: limit.direction,
            acceptable: limit.acceptable,
            critical: limit.critical,
            weight: limit.weight,
            diagnostic_code: limit.diagnostic_code.clone(),
            explanation,
            sample_count: metric.sample_count,
            shot_count: metric.shot_count,
            circuit_count: metric.circuit_count,
        })
    }

    fn unconfigured_finding(
        &self,
        metric: &Metric,
    ) -> BottleneckFinding {
        let metric_id = metric.kind.id();

        BottleneckFinding {
            metric_id: metric_id.clone(),
            kind: metric.kind.clone(),
            unit: metric.unit.clone(),
            value: metric.value,
            severity: finite_zero(),
            best_case_severity: finite_zero(),
            worst_case_severity: finite_zero(),
            weighted_pressure: finite_zero(),
            classification: BottleneckClassification::Unconfigured,
            confidence: BottleneckConfidence::Unspecified,
            quality: metric.quality,
            direction: metric.direction,
            acceptable: finite_zero(),
            critical: finite_zero(),
            weight: finite_zero(),
            diagnostic_code: format!(
                "bottleneck.unconfigured.{}",
                sanitize_diagnostic_component(&metric_id)
            ),
            explanation: format!(
                "Metric `{}` was measured but no bottleneck limit was configured; \
                 no bottleneck conclusion was made.",
                metric_id
            ),
            sample_count: metric.sample_count,
            shot_count: metric.shot_count,
            circuit_count: metric.circuit_count,
        }
    }
}

// =============================================================================
// Confidence analysis
// =============================================================================

fn confidence_severity(
    metric: &Metric,
    limit: &MetricLimit,
) -> Result<(f64, f64, BottleneckConfidence), BottleneckError> {
    let Some(confidence) = &metric.confidence else {
        let severity = limit.severity(metric.value.get())?;

        return Ok((
            severity,
            severity,
            BottleneckConfidence::Unspecified,
        ));
    };

    let lower = confidence.lower.get();
    let upper = confidence.upper.get();

    if !lower.is_finite() || !upper.is_finite() {
        return Err(BottleneckError::NonFiniteMetricValue {
            metric_id: limit.metric_id.clone(),
        });
    }

    let low_severity = limit.severity(lower)?;
    let high_severity = limit.severity(upper)?;

    let best = low_severity.min(high_severity);
    let worst = low_severity.max(high_severity);

    let point = limit.severity(metric.value.get())?;

    let crosses_important_boundary =
        crosses_classification_boundary(
            point,
            best,
            worst,
        );

    let confidence = if matches!(
        metric.quality,
        MetricQuality::Uncertain | MetricQuality::Approximate
    ) {
        BottleneckConfidence::LowQuality
    } else if crosses_important_boundary {
        BottleneckConfidence::BoundaryCrossing
    } else {
        BottleneckConfidence::Robust
    };

    Ok((best, worst, confidence))
}

fn crosses_classification_boundary(
    point: f64,
    best: f64,
    worst: f64,
) -> bool {
    let threshold = 0.5;

    let point_side = point >= threshold;
    let best_side = best >= threshold;
    let worst_side = worst >= threshold;

    point_side != best_side || point_side != worst_side
}

// =============================================================================
// Classification
// =============================================================================

fn classify(
    severity: f64,
    best_case_severity: f64,
    worst_case_severity: f64,
    metric: &Metric,
    limit: &MetricLimit,
    bottleneck_threshold: f64,
) -> BottleneckClassification {
    if matches!(
        metric.quality,
        MetricQuality::Uncertain | MetricQuality::Approximate
    ) {
        return BottleneckClassification::Uncertain;
    }

    if worst_case_severity < bottleneck_threshold
        && severity < bottleneck_threshold
    {
        if limit.is_acceptable(metric.value.get()) {
            return BottleneckClassification::Healthy;
        }

        return BottleneckClassification::Watch;
    }

    if limit.is_critical(metric.value.get()) {
        if best_case_severity < 1.0 - NUMERIC_EPSILON {
            return BottleneckClassification::Uncertain;
        }

        return BottleneckClassification::Critical;
    }

    if severity >= bottleneck_threshold {
        if best_case_severity < bottleneck_threshold {
            return BottleneckClassification::Uncertain;
        }

        return BottleneckClassification::Bottleneck;
    }

    BottleneckClassification::Watch
}

// =============================================================================
// Explanation
// =============================================================================

fn build_explanation(
    metric: &Metric,
    limit: &MetricLimit,
    severity: f64,
    best_case_severity: f64,
    worst_case_severity: f64,
    classification: BottleneckClassification,
    confidence: BottleneckConfidence,
) -> String {
    let direction = match limit.direction {
        MetricDirection::LowerIsBetter => "lower",
        MetricDirection::HigherIsBetter => "higher",
        MetricDirection::Neutral => "neutral",
    };

    let quality = match metric.quality {
        MetricQuality::Observed => "observed",
        MetricQuality::Derived => "derived",
        MetricQuality::Estimated => "estimated",
        MetricQuality::Fitted => "fitted",
        MetricQuality::Approximate => "approximate",
        MetricQuality::Uncertain => "uncertain",
        MetricQuality::Invalid => "invalid",
    };

    format!(
        "Metric `{}` has value {} {} with {}-is-better semantics. \
         Acceptable boundary is {}, critical boundary is {}. \
         Point severity is {:.6}, confidence-range severity is \
         [{:.6}, {:.6}]. Classification: {}. Confidence state: {}. \
         Metric quality: {}. {}",
        limit.metric_id,
        metric.value.get(),
        limit.unit.id(),
        direction,
        limit.acceptable.get(),
        limit.critical.get(),
        severity,
        best_case_severity,
        worst_case_severity,
        classification.as_str(),
        confidence.as_str(),
        quality,
        limit.description,
    )
}

// =============================================================================
// Summary
// =============================================================================

fn build_summary(
    metric_count: usize,
    findings: &[BottleneckFinding],
    total_weight: f64,
) -> Result<BottleneckSummary, BottleneckError> {
    let mut bottleneck_count = 0usize;
    let mut critical_count = 0usize;
    let mut healthy_count = 0usize;
    let mut watch_count = 0usize;
    let mut uncertain_count = 0usize;
    let mut unconfigured_count = 0usize;

    let mut weighted_pressure = 0.0f64;

    for finding in findings {
        match finding.classification {
            BottleneckClassification::Healthy => {
                healthy_count += 1;
            }

            BottleneckClassification::Watch => {
                watch_count += 1;
            }

            BottleneckClassification::Bottleneck => {
                bottleneck_count += 1;
            }

            BottleneckClassification::Critical => {
                critical_count += 1;
                bottleneck_count += 1;
            }

            BottleneckClassification::Uncertain => {
                uncertain_count += 1;
            }

            BottleneckClassification::Unconfigured => {
                unconfigured_count += 1;
            }
        }

        weighted_pressure += finding.weighted_pressure.get();
    }

    let aggregate_pressure = if total_weight > 0.0 {
        // `weighted_pressure` is already normalized against total weight.
        weighted_pressure.min(1.0).max(0.0)
    } else {
        0.0
    };

    if !aggregate_pressure.is_finite() {
        return Err(BottleneckError::InvalidTotalWeight);
    }

    let primary_bottleneck_index = findings
        .iter()
        .enumerate()
        .filter(|(_, finding)| finding.is_bottleneck())
        .max_by(|(_, left), (_, right)| {
            compare_findings(left, right)
        })
        .map(|(index, _)| index);

    Ok(BottleneckSummary {
        metric_count,
        configured_metric_count: findings
            .iter()
            .filter(|finding| {
                finding.classification
                    != BottleneckClassification::Unconfigured
            })
            .count(),
        bottleneck_count,
        critical_count,
        healthy_count,
        watch_count,
        uncertain_count,
        unconfigured_count,
        aggregate_pressure: FiniteF64::new(aggregate_pressure)
            .expect("aggregate pressure was validated finite"),
        primary_bottleneck_index,
    })
}

// =============================================================================
// Ordering
// =============================================================================

fn compare_findings(
    left: &BottleneckFinding,
    right: &BottleneckFinding,
) -> Ordering {
    right
        .weighted_pressure
        .get()
        .partial_cmp(&left.weighted_pressure.get())
        .unwrap_or(Ordering::Equal)
        .then_with(|| {
            right
                .worst_case_severity
                .get()
                .partial_cmp(&left.worst_case_severity.get())
                .unwrap_or(Ordering::Equal)
        })
        .then_with(|| {
            classification_rank(right.classification)
                .cmp(&classification_rank(left.classification))
        })
        .then_with(|| left.metric_id.cmp(&right.metric_id))
}

fn classification_rank(
    classification: BottleneckClassification,
) -> u8 {
    match classification {
        BottleneckClassification::Critical => 5,
        BottleneckClassification::Bottleneck => 4,
        BottleneckClassification::Uncertain => 3,
        BottleneckClassification::Watch => 2,
        BottleneckClassification::Healthy => 1,
        BottleneckClassification::Unconfigured => 0,
    }
}

// =============================================================================
// Numeric helpers
// =============================================================================

fn interpolate(
    value: f64,
    acceptable: f64,
    critical: f64,
) -> f64 {
    let denominator = critical - acceptable;

    if denominator.abs() <= NUMERIC_EPSILON {
        return if value >= critical { 1.0 } else { 0.0 };
    }

    (value - acceptable) / denominator
}

fn clamp_unit_interval(value: f64) -> f64 {
    if value <= 0.0 {
        0.0
    } else if value >= 1.0 {
        1.0
    } else {
        value
    }
}

fn finite_score(
    value: f64,
    metric_id: &str,
) -> Result<FiniteF64, BottleneckError> {
    let value = clamp_unit_interval(value);

    if !value.is_finite() {
        return Err(BottleneckError::NonFiniteScore {
            metric_id: metric_id.to_owned(),
        });
    }

    Ok(FiniteF64::new(value)
        .expect("clamped score must be finite"))
}

fn finite_zero() -> FiniteF64 {
    FiniteF64::new(0.0)
        .expect("zero is finite")
}

// =============================================================================
// Identifier validation
// =============================================================================

fn validate_identifier(
    identifier: &str,
) -> Result<(), BottleneckError> {
    if identifier.trim().is_empty()
        || identifier.len() > MAX_METRIC_ID_LENGTH
    {
        return Err(BottleneckError::InvalidMetricIdentifier {
            identifier: identifier.to_owned(),
        });
    }

    Ok(())
}

fn sanitize_diagnostic_component(
    identifier: &str,
) -> String {
    let mut output = String::with_capacity(identifier.len().min(128));

    for character in identifier.chars().take(128) {
        if character.is_ascii_alphanumeric()
            || character == '_'
            || character == '-'
            || character == '.'
        {
            output.push(character);
        } else {
            output.push('_');
        }
    }

    if output.is_empty() {
        "metric".to_owned()
    } else {
        output
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn runtime_metric(value: f64) -> Metric {
        Metric::new(
            MetricKind::Runtime,
            MetricUnit::Seconds,
            value,
        )
        .expect("runtime metric should be valid")
    }

    fn throughput_metric(value: f64) -> Metric {
        Metric::new(
            MetricKind::Throughput,
            MetricUnit::Operations,
            value,
        )
        .expect("throughput metric should be valid")
    }

    fn fidelity_metric(value: f64) -> Metric {
        Metric::new(
            MetricKind::Fidelity,
            MetricUnit::Probability,
            value,
        )
        .expect("fidelity metric should be valid")
    }

    #[test]
    fn lower_is_better_severity_is_zero_at_acceptable_boundary() {
        let limit = MetricLimit::new(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
        )
        .expect("limit should be valid");

        assert_eq!(
            limit.severity(1.0).expect("severity"),
            0.0
        );
    }

    #[test]
    fn lower_is_better_severity_is_one_at_critical_boundary() {
        let limit = MetricLimit::new(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
        )
        .expect("limit should be valid");

        assert_eq!(
            limit.severity(2.0).expect("severity"),
            1.0
        );
    }

    #[test]
    fn lower_is_better_interpolates() {
        let limit = MetricLimit::new(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            3.0,
        )
        .expect("limit should be valid");

        let severity =
            limit.severity(2.0).expect("severity");

        assert!((severity - 0.5).abs() < NUMERIC_EPSILON);
    }

    #[test]
    fn higher_is_better_is_zero_above_acceptable() {
        let limit = MetricLimit::new(
            "fidelity",
            MetricUnit::Probability,
            MetricDirection::HigherIsBetter,
            0.99,
            0.90,
        )
        .expect("limit should be valid");

        assert_eq!(
            limit.severity(0.995).expect("severity"),
            0.0
        );
    }

    #[test]
    fn higher_is_better_is_one_below_critical() {
        let limit = MetricLimit::new(
            "fidelity",
            MetricUnit::Probability,
            MetricDirection::HigherIsBetter,
            0.99,
            0.90,
        )
        .expect("limit should be valid");

        assert_eq!(
            limit.severity(0.90).expect("severity"),
            1.0
        );
    }

    #[test]
    fn policy_rejects_reversed_lower_is_better_thresholds() {
        let result = MetricLimit::new(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            2.0,
            1.0,
        );

        assert!(matches!(
            result,
            Err(BottleneckError::InvalidThresholdOrder { .. })
        ));
    }

    #[test]
    fn policy_rejects_reversed_higher_is_better_thresholds() {
        let result = MetricLimit::new(
            "fidelity",
            MetricUnit::Probability,
            MetricDirection::HigherIsBetter,
            0.90,
            0.99,
        );

        assert!(matches!(
            result,
            Err(BottleneckError::InvalidThresholdOrder { .. })
        ));
    }

    #[test]
    fn analyzer_identifies_runtime_bottleneck() {
        let metric = runtime_metric(2.0);

        let limit = MetricLimit::new(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
        )
        .expect("limit");

        let policy =
            BottleneckPolicy::new(limit)
                .expect("policy");

        let analysis =
            BottleneckAnalyzer::new()
                .analyze(&[metric], &policy)
                .expect("analysis");

        assert!(analysis.has_bottleneck());
        assert_eq!(
            analysis.summary.bottleneck_count,
            1
        );
        assert_eq!(
            analysis.summary.critical_count,
            1
        );
    }

    #[test]
    fn analyzer_identifies_healthy_runtime() {
        let metric = runtime_metric(0.5);

        let limit = MetricLimit::new(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
        )
        .expect("limit");

        let policy =
            BottleneckPolicy::new(limit)
                .expect("policy");

        let analysis =
            BottleneckAnalyzer::new()
                .analyze(&[metric], &policy)
                .expect("analysis");

        assert!(!analysis.has_bottleneck());
        assert_eq!(
            analysis.summary.healthy_count,
            1
        );
    }

    #[test]
    fn analyzer_identifies_higher_is_better_bottleneck() {
        let metric = throughput_metric(10.0);

        let limit = MetricLimit::new(
            "throughput",
            MetricUnit::Operations,
            MetricDirection::HigherIsBetter,
            100.0,
            20.0,
        )
        .expect("limit");

        let policy =
            BottleneckPolicy::new(limit)
                .expect("policy");

        let analysis =
            BottleneckAnalyzer::new()
                .analyze(&[metric], &policy)
                .expect("analysis");

        assert!(analysis.has_bottleneck());
    }

    #[test]
    fn analyzer_does_not_invent_unconfigured_bottleneck() {
        let metric = runtime_metric(1000.0);

        let limit = MetricLimit::new(
            "throughput",
            MetricUnit::Operations,
            MetricDirection::HigherIsBetter,
            100.0,
            10.0,
        )
        .expect("limit");

        let policy =
            BottleneckPolicy::new(limit)
                .expect("policy");

        let analysis =
            BottleneckAnalyzer::new()
                .analyze(&[metric], &policy)
                .expect("analysis");

        assert_eq!(
            analysis.summary.unconfigured_count,
            1
        );
        assert!(!analysis.has_bottleneck());
    }

    #[test]
    fn analyzer_rejects_unit_mismatch() {
        let metric = runtime_metric(2.0);

        let limit = MetricLimit::new(
            "runtime",
            MetricUnit::Milliseconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
        )
        .expect("limit");

        let policy =
            BottleneckPolicy::new(limit)
                .expect("policy");

        let result =
            BottleneckAnalyzer::new()
                .analyze(&[metric], &policy);

        assert!(matches!(
            result,
            Err(BottleneckError::UnitMismatch { .. })
        ));
    }

    #[test]
    fn analyzer_rejects_invalid_metric_quality() {
        let mut metric = runtime_metric(1.0);
        metric.quality = MetricQuality::Invalid;

        let limit = MetricLimit::new(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
        )
        .expect("limit");

        let policy =
            BottleneckPolicy::new(limit)
                .expect("policy");

        let result =
            BottleneckAnalyzer::new()
                .analyze(&[metric], &policy);

        assert!(matches!(
            result,
            Err(BottleneckError::InvalidMetricQuality { .. })
        ));
    }

    #[test]
    fn findings_are_ranked_by_weighted_pressure() {
        let runtime = runtime_metric(2.0);
        let fidelity = fidelity_metric(0.90);

        let runtime_limit = MetricLimit::with_weight(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
            1.0,
        )
        .expect("runtime limit");

        let fidelity_limit = MetricLimit::with_weight(
            "fidelity",
            MetricUnit::Probability,
            MetricDirection::HigherIsBetter,
            0.99,
            0.80,
            3.0,
        )
        .expect("fidelity limit");

        let policy =
            BottleneckPolicy::with_limits(vec![
                runtime_limit,
                fidelity_limit,
            ])
            .expect("policy");

        let analysis =
            BottleneckAnalyzer::new()
                .analyze(
                    &[runtime, fidelity],
                    &policy,
                )
                .expect("analysis");

        assert_eq!(
            analysis.findings[0].metric_id,
            "fidelity"
        );
    }

    #[test]
    fn confidence_interval_can_make_classification_uncertain() {
        let mut metric = runtime_metric(1.5);

        let confidence = super::super::super::core::metric::MetricConfidence::new(
            0.95,
            0.5,
            2.5,
            super::super::super::core::metric::ConfidenceMethod::NormalApproximation,
        )
        .expect("confidence");

        metric = metric
            .with_confidence(confidence)
            .expect("metric confidence");

        let limit = MetricLimit::new(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
        )
        .expect("limit");

        let policy =
            BottleneckPolicy::new(limit)
                .expect("policy");

        let analysis =
            BottleneckAnalyzer::new()
                .analyze(&[metric], &policy)
                .expect("analysis");

        assert_eq!(
            analysis.findings[0].confidence,
            BottleneckConfidence::BoundaryCrossing
        );
    }

    #[test]
    fn uncertain_quality_is_explicitly_preserved() {
        let mut metric = runtime_metric(2.0);
        metric.quality = MetricQuality::Uncertain;

        let limit = MetricLimit::new(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
        )
        .expect("limit");

        let policy =
            BottleneckPolicy::new(limit)
                .expect("policy");

        let analysis =
            BottleneckAnalyzer::new()
                .analyze(&[metric], &policy)
                .expect("analysis");

        assert_eq!(
            analysis.findings[0].classification,
            BottleneckClassification::Uncertain
        );
        assert_eq!(
            analysis.findings[0].confidence,
            BottleneckConfidence::LowQuality
        );
    }

    #[test]
    fn policy_can_exclude_uncertain_metrics() {
        let mut metric = runtime_metric(2.0);
        metric.quality = MetricQuality::Uncertain;

        let limit = MetricLimit::new(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
        )
        .expect("limit");

        let policy =
            BottleneckPolicy::new(limit)
                .expect("policy")
                .include_uncertain_metrics(false);

        let analysis =
            BottleneckAnalyzer::new()
                .analyze(&[metric], &policy)
                .expect("analysis");

        assert!(analysis.findings.is_empty());
        assert_eq!(
            analysis.summary.metric_count,
            1
        );
    }

    #[test]
    fn aggregate_pressure_is_bounded() {
        let runtime = runtime_metric(2.0);

        let limit = MetricLimit::with_weight(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
            100.0,
        )
        .expect("limit");

        let policy =
            BottleneckPolicy::new(limit)
                .expect("policy");

        let analysis =
            BottleneckAnalyzer::new()
                .analyze(&[runtime], &policy)
                .expect("analysis");

        assert!(
            (0.0..=1.0)
                .contains(
                    &analysis
                        .summary
                        .aggregate_pressure
                        .get()
                )
        );
    }

    #[test]
    fn primary_bottleneck_is_available() {
        let metric = runtime_metric(2.0);

        let limit = MetricLimit::new(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
        )
        .expect("limit");

        let policy =
            BottleneckPolicy::new(limit)
                .expect("policy");

        let analysis =
            BottleneckAnalyzer::new()
                .analyze(&[metric], &policy)
                .expect("analysis");

        assert_eq!(
            analysis
                .primary_bottleneck()
                .expect("primary")
                .metric_id,
            "runtime"
        );
    }

    #[test]
    fn duplicate_metrics_are_not_silently_collapsed() {
        let first = runtime_metric(1.0);
        let second = runtime_metric(2.0);

        let limit = MetricLimit::new(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
        )
        .expect("limit");

        let policy =
            BottleneckPolicy::new(limit)
                .expect("policy");

        let analysis =
            BottleneckAnalyzer::new()
                .analyze(
                    &[first, second],
                    &policy,
                )
                .expect("analysis");

        assert_eq!(analysis.findings.len(), 2);
    }

    #[test]
    fn deterministic_ranking_is_used_for_equal_scores() {
        let runtime = runtime_metric(2.0);

        let mut second = runtime_metric(2.0);
        second.kind = MetricKind::Latency;

        let runtime_limit = MetricLimit::new(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
        )
        .expect("runtime limit");

        let latency_limit = MetricLimit::new(
            "latency",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
        )
        .expect("latency limit");

        let policy =
            BottleneckPolicy::with_limits(vec![
                runtime_limit,
                latency_limit,
            ])
            .expect("policy");

        let analysis =
            BottleneckAnalyzer::new()
                .analyze(
                    &[runtime, second],
                    &policy,
                )
                .expect("analysis");

        assert_eq!(
            analysis.findings[0].metric_id,
            "latency"
        );
        assert_eq!(
            analysis.findings[1].metric_id,
            "runtime"
        );
    }

    #[test]
    fn empty_metrics_are_rejected() {
        let limit = MetricLimit::new(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
        )
        .expect("limit");

        let policy =
            BottleneckPolicy::new(limit)
                .expect("policy");

        let result =
            BottleneckAnalyzer::new()
                .analyze(&[], &policy);

        assert!(matches!(
            result,
            Err(BottleneckError::EmptyMetrics)
        ));
    }

    #[test]
    fn empty_policy_is_rejected() {
        let result =
            BottleneckPolicy::with_limits(Vec::new());

        assert!(matches!(
            result,
            Err(BottleneckError::EmptyPolicy)
        ));
    }

    #[test]
    fn zero_weight_policy_is_rejected_at_analysis_time() {
        let limit = MetricLimit::with_weight(
            "runtime",
            MetricUnit::Seconds,
            MetricDirection::LowerIsBetter,
            1.0,
            2.0,
            0.0,
        )
        .expect("zero weight is structurally valid");

        let policy =
            BottleneckPolicy::new(limit)
                .expect("policy");

        let metric = runtime_metric(2.0);

        let result =
            BottleneckAnalyzer::new()
                .analyze(&[metric], &policy);

        assert!(matches!(
            result,
            Err(BottleneckError::InvalidTotalWeight)
        ));
    }

    #[test]
    fn neutral_metric_is_not_a_bottleneck() {
        let metric = Metric::new(
            MetricKind::Custom("neutral_test".to_owned()),
            MetricUnit::Dimensionless,
            100.0,
        )
        .expect("metric");

        let limit = MetricLimit::new(
            "neutral_test",
            MetricUnit::Dimensionless,
            MetricDirection::Neutral,
            0.0,
            100.0,
        )
        .expect("limit");

        let policy =
            BottleneckPolicy::new(limit)
                .expect("policy");

        let analysis =
            BottleneckAnalyzer::new()
                .analyze(&[metric], &policy)
                .expect("analysis");

        assert!(!analysis.has_bottleneck());
    }
}