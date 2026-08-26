//! Zamani Quantum Benchmarking — Baseline Management
//!
//! Production-grade immutable benchmark baselines.
//!
//! # Purpose
//!
//! This module defines the authoritative representation and comparison
//! mechanism for historical/reference benchmark baselines.
//!
//! A baseline answers:
//!
//! > "What previously established benchmark result should this candidate
//! > result be compared against, under exactly which benchmark scope and
//! > dimensions?"
//!
//! A baseline is deliberately more than a collection of metrics.
//!
//! ```text
//! Baseline
//! ├── schema version
//! ├── baseline identity
//! ├── benchmark identity/version
//! ├── optional capture metadata
//! ├── scope metadata
//! └── immutable metric entries
//!       ├── metric
//!       └── benchmark dimensions
//! ```
//!
//! # Architectural position
//!
//! ```text
//!                         BenchmarkResult
//!                               │
//!                               ▼
//!                         core::metric
//!                               │
//!                               ▼
//!                      analysis::baseline
//!                               │
//!              ┌────────────────┴────────────────┐
//!              ▼                                 ▼
//!        baseline snapshot                analysis::compare
//!                                                │
//!                                                ▼
//!                                      MetricComparison
//!                                                │
//!                                                ▼
//!                                      analysis::regression
//! ```
//!
//! This module does NOT:
//!
//! - execute quantum circuits;
//! - generate circuits;
//! - select hardware;
//! - communicate with providers;
//! - compile circuits;
//! - perform routing;
//! - perform scheduling;
//! - calculate protocol-specific metrics;
//! - fit statistical models;
//! - perform formal hypothesis tests;
//! - decide CI policy;
//! - mutate global benchmark state;
//! - write files;
//! - access the network;
//! - access clocks;
//! - print diagnostics.
//!
//! Those responsibilities belong to other layers.
//!
//! # Why dimensions belong to the baseline key
//!
//! A metric identity alone consists of:
//!
//! ```text
//! MetricKind + MetricUnit
//! ```
//!
//! as established by `analysis::compare`.
//!
//! That identity is intentionally insufficient to identify a benchmark
//! observation in a volumetric or application benchmark.
//!
//! For example:
//!
//! ```text
//! QuantumVolume / Dimensionless / width=8
//! QuantumVolume / Dimensionless / width=16
//! ```
//!
//! are different benchmark observations despite having the same metric kind
//! and unit.
//!
//! Therefore this module attaches explicit benchmark dimensions to every
//! baseline entry.
//!
//! # Dimension representation
//!
//! The baseline layer deliberately does not depend on `core::dimension`.
//!
//! This preserves the low-level dependency boundary and allows the dimension
//! subsystem to evolve independently.
//!
//! Dimensions are represented by validated string key/value pairs.
//!
//! Examples:
//!
//! ```text
//! qubits = 16
//! depth = 16
//! problem_size = 32
//! instance = maxcut_001
//! backend = device_a
//! optimization_level = 2
//! ```
//!
//! The keys and values are canonicalized deterministically before storage.
//!
//! # Baseline immutability
//!
//! Once constructed, a `Baseline` contains a complete immutable snapshot.
//!
//! There is no public method that silently replaces or mutates an existing
//! metric.
//!
//! To construct a new baseline, create a new `BaselineBuilder`.
//!
//! This is intentional:
//!
//! ```text
//! historical baseline
//!       │
//!       ├── never modified
//!       │
//!       ▼
//! new candidate
//!       │
//!       ▼
//! comparison
//! ```
//!
//! This prevents historical benchmark records from changing underneath
//! regression analysis.
//!
//! # Scientific integrity
//!
//! A baseline comparison must preserve:
//!
//! - benchmark identity;
//! - benchmark version;
//! - metric identity;
//! - metric unit;
//! - metric direction;
//! - benchmark dimensions;
//! - uncertainty;
//! - confidence intervals;
//! - sample counts;
//! - shot counts;
//! - circuit counts;
//! - metric quality;
//! - metric provenance.
//!
//! The metric itself remains owned by `core::metric`.
//!
//! This module never strips uncertainty or statistical metadata from a metric.
//!
//! # Important comparison rule
//!
//! `analysis::compare::compare_metrics()` remains the authoritative numerical
//! comparison implementation.
//!
//! This module does NOT reimplement:
//!
//! - relative change;
//! - ratio;
//! - confidence interval relationship;
//! - uncertainty separation;
//! - metric direction semantics;
//! - comparison conclusions.
//!
//! Instead:
//!
//! ```text
//! baseline
//!    │
//!    ▼
//! scoped metric matching
//!    │
//!    ▼
//! analysis::compare::compare_metrics()
//!    │
//!    ▼
//! MetricComparison
//! ```
//!
//! This prevents two competing comparison algorithms from developing inside
//! Zamani.
//!
//! # Baseline identity
//!
//! `baseline_id` is an externally meaningful stable identifier.
//!
//! It may be assigned by:
//!
//! - a benchmark registry;
//! - CI;
//! - a user;
//! - a benchmark artifact store;
//! - a Zamani program;
//! - a future reporting layer.
//!
//! This module does not invent a timestamp or random identifier.
//!
//! # Reproducibility
//!
//! A baseline contains explicit benchmark and scope metadata.
//!
//! It does not claim that the baseline can be reproduced merely because its
//! metric values are available.
//!
//! Reproduction requires the original experiment definition/provenance,
//! which belongs to the core experiment/provenance layers.
//!
//! The baseline therefore preserves metric provenance instead of replacing it.
//!
//! # Resource safety
//!
//! Baselines are treated as untrusted deserialized input.
//!
//! The implementation therefore enforces:
//!
//! - non-empty identifiers;
//! - bounded baseline size;
//! - bounded dimension count;
//! - bounded string lengths;
//! - unique dimension keys;
//! - unique metric/dimension identities;
//! - finite metric values;
//! - finite uncertainties;
//! - no invalid confidence intervals;
//! - no unbounded duplicate matching;
//! - no mutation during comparison.
//!
//! # Determinism
//!
//! The implementation is deterministic.
//!
//! It does not depend on:
//!
//! - clocks;
//! - random generators;
//! - environment variables;
//! - filesystem state;
//! - process-global state;
//! - network services.
//!
//! Baseline entries retain insertion order.
//!
//! Dimension keys are canonicalized into deterministic order.
//!
//! # Serialization
//!
//! `serde` is already used by the canonical metric model in Zamani, so this
//! module uses serde for baseline persistence/interchange.
//!
//! Serialization is intentionally independent from JSON formatting. The
//! reporting layer decides whether a baseline is ultimately represented as
//! JSON, YAML, another interchange format, or an internal artifact.
//!
//! # Integration contract
//!
//! This file is complete without requiring any changes to its implementation
//! after the other analysis files are added.
//!
//! Required module wiring is only:
//!
//! ```text
//! src/quantum/benchmarking/analysis/mod.rs
//!     pub mod baseline;
//! ```
//!
//! No existing core file needs to be changed.
//!
//! The dependency graph is:
//!
//! ```text
//! core::metric
//!      │
//!      ▼
//! analysis::compare
//!      │
//!      ▼
//! analysis::baseline
//!      │
//!      ├── analysis::regression
//!      ├── reporting
//!      └── CI / benchmark registry
//! ```
//!
//! Future `analysis::regression` code should consume:
//!
//! ```text
//! BaselineComparison
//! ```
//!
//! rather than reimplementing baseline lookup.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//!
//! No nightly features are used.
//!
//! No `unsafe` code is used.
//!
//! -----------------------------------------------------------------------------
//! Public API
//! -----------------------------------------------------------------------------
//!
//! Core types:
//!
//! - `Baseline`
//! - `BaselineBuilder`
//! - `BaselineMetric`
//! - `BaselineDimension`
//! - `BaselineScope`
//! - `BaselineComparison`
//! - `BaselineComparisonPolicy`
//! - `BaselineError`
//!
//! Primary functions/methods:
//!
//! - `Baseline::builder()`
//! - `BaselineBuilder::add_metric()`
//! - `BaselineBuilder::build()`
//! - `Baseline::compare_metric()`
//! - `Baseline::compare_metrics()`
//! - `Baseline::metric()`
//! - `Baseline::metrics()`
//!
//! -----------------------------------------------------------------------------
//! Example
//! -----------------------------------------------------------------------------
//!
//! ```rust
//! use crate::quantum::benchmarking::analysis::baseline::{
//!     Baseline,
//!     BaselineComparisonPolicy,
//! };
//! use crate::quantum::benchmarking::core::metric::{
//!     Metric,
//!     MetricKind,
//!     MetricUnit,
//! };
//!
//! let metric = Metric::new(
//!     MetricKind::QuantumVolume,
//!     MetricUnit::Dimensionless,
//!     32.0,
//! ).unwrap();
//!
//! let baseline = Baseline::builder("qv-baseline-2026-01")
//!     .benchmark("quantum_volume", "1.0")
//!     .add_metric(
//!         vec![
//!             ("qubits", "8"),
//!             ("depth", "8"),
//!         ],
//!         metric,
//!     )
//!     .unwrap()
//!     .build()
//!     .unwrap();
//!
//! let candidate = Metric::new(
//!     MetricKind::QuantumVolume,
//!     MetricUnit::Dimensionless,
//!     64.0,
//! ).unwrap();
//!
//! let comparison = baseline
//!     .compare_metric(
//!         &[
//!             ("qubits", "8"),
//!             ("depth", "8"),
//!         ],
//!         &candidate,
//!         &BaselineComparisonPolicy::default(),
//!     )
//!     .unwrap();
//!
//! assert!(comparison.comparison.is_improvement());
//! ```
//!
//! The example intentionally keeps dimensions explicit so that the same metric
//! kind at different problem sizes cannot accidentally be compared.
//!

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use crate::quantum::benchmarking::analysis::compare::{
    compare_metrics,
    ComparisonPolicy,
    MetricComparison,
};
use crate::quantum::benchmarking::core::metric::{
    Metric,
    MetricDirection,
    MetricKind,
    MetricUnit,
};

// =============================================================================
// Public constants
// =============================================================================

/// Stable semantic version of the baseline representation.
pub const BASELINE_SCHEMA_VERSION: u32 = 1;

/// Maximum number of metrics in one baseline.
///
/// This protects deserialized/untrusted baseline data from accidental
/// unbounded memory use.
pub const DEFAULT_MAX_BASELINE_METRICS: usize = 100_000;

/// Maximum number of dimensions attached to one metric.
pub const DEFAULT_MAX_DIMENSIONS_PER_METRIC: usize = 64;

/// Maximum UTF-8 byte length accepted for baseline identifiers.
pub const DEFAULT_MAX_IDENTIFIER_LENGTH: usize = 256;

/// Maximum UTF-8 byte length accepted for dimension keys.
pub const DEFAULT_MAX_DIMENSION_KEY_LENGTH: usize = 128;

/// Maximum UTF-8 byte length accepted for dimension values.
pub const DEFAULT_MAX_DIMENSION_VALUE_LENGTH: usize = 1024;

/// Maximum UTF-8 byte length accepted for benchmark identifiers.
pub const DEFAULT_MAX_BENCHMARK_ID_LENGTH: usize = 256;

/// Maximum UTF-8 byte length accepted for benchmark versions.
pub const DEFAULT_MAX_BENCHMARK_VERSION_LENGTH: usize = 128;

/// Maximum number of arbitrary scope metadata entries.
pub const DEFAULT_MAX_SCOPE_METADATA: usize = 128;

/// Maximum UTF-8 byte length accepted for scope metadata keys.
pub const DEFAULT_MAX_SCOPE_KEY_LENGTH: usize = 128;

/// Maximum UTF-8 byte length accepted for scope metadata values.
pub const DEFAULT_MAX_SCOPE_VALUE_LENGTH: usize = 2048;

// =============================================================================
// Errors
// =============================================================================

/// Errors raised while creating, validating, querying or comparing baselines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaselineError {
    /// An identifier is empty.
    EmptyIdentifier {
        /// Field containing the invalid identifier.
        field: &'static str,
    },

    /// An identifier exceeds its permitted length.
    IdentifierTooLong {
        /// Field containing the identifier.
        field: &'static str,

        /// Actual byte length.
        length: usize,

        /// Maximum byte length.
        maximum: usize,
    },

    /// A dimension key is empty.
    EmptyDimensionKey,

    /// A dimension value is empty.
    EmptyDimensionValue,

    /// A dimension key exceeds the configured length.
    DimensionKeyTooLong {
        /// Actual byte length.
        length: usize,

        /// Maximum allowed length.
        maximum: usize,
    },

    /// A dimension value exceeds the configured length.
    DimensionValueTooLong {
        /// Actual byte length.
        length: usize,

        /// Maximum allowed length.
        maximum: usize,
    },

    /// A dimension key is duplicated within one metric scope.
    DuplicateDimensionKey {
        /// Duplicated key.
        key: String,
    },

    /// Too many dimensions were supplied.
    TooManyDimensions {
        /// Number supplied.
        count: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// Too many metrics were supplied.
    TooManyMetrics {
        /// Number supplied.
        count: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// Too many scope metadata entries were supplied.
    TooManyScopeMetadata {
        /// Number supplied.
        count: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// A scope metadata key is invalid.
    InvalidScopeMetadataKey,

    /// A scope metadata value is invalid.
    InvalidScopeMetadataValue,

    /// A scope metadata key is too long.
    ScopeMetadataKeyTooLong {
        /// Actual byte length.
        length: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// A scope metadata value is too long.
    ScopeMetadataValueTooLong {
        /// Actual byte length.
        length: usize,

        /// Maximum allowed.
        maximum: usize,
    },

    /// The same metric identity and dimension scope occur more than once.
    DuplicateMetricScope {
        /// Stable duplicate identity.
        identity: String,
    },

    /// The requested metric does not exist in the baseline.
    MissingMetric {
        /// Requested identity.
        identity: String,
    },

    /// Candidate metric dimensions contain duplicates.
    InvalidCandidateDimensions {
        /// Reason.
        reason: String,
    },

    /// Candidate dimensions do not match the baseline dimensions.
    DimensionMismatch {
        /// Baseline scope identity.
        baseline: String,

        /// Candidate scope identity.
        candidate: String,
    },

    /// Candidate metric cannot be compared with the selected baseline metric.
    ComparisonFailed {
        /// Baseline metric identity.
        identity: String,

        /// Comparison error.
        reason: String,
    },

    /// The baseline contains invalid metric data.
    InvalidMetric {
        /// Metric identity.
        identity: String,

        /// Reason.
        reason: String,
    },

    /// The baseline has no metrics.
    EmptyBaseline,

    /// Invalid baseline comparison policy.
    InvalidPolicy {
        /// Invalid policy field.
        field: &'static str,
    },
}

impl fmt::Display for BaselineError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(
                    formatter,
                    "baseline field `{}` must not be empty",
                    field
                )
            }

            Self::IdentifierTooLong {
                field,
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "baseline field `{}` is {} bytes long; maximum is {}",
                    field,
                    length,
                    maximum
                )
            }

            Self::EmptyDimensionKey => {
                write!(formatter, "baseline dimension key must not be empty")
            }

            Self::EmptyDimensionValue => {
                write!(
                    formatter,
                    "baseline dimension value must not be empty"
                )
            }

            Self::DimensionKeyTooLong {
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "baseline dimension key is {} bytes long; maximum is {}",
                    length,
                    maximum
                )
            }

            Self::DimensionValueTooLong {
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "baseline dimension value is {} bytes long; maximum is {}",
                    length,
                    maximum
                )
            }

            Self::DuplicateDimensionKey { key } => {
                write!(
                    formatter,
                    "baseline dimension key `{}` is duplicated",
                    key
                )
            }

            Self::TooManyDimensions {
                count,
                maximum,
            } => {
                write!(
                    formatter,
                    "baseline metric contains {} dimensions; maximum is {}",
                    count,
                    maximum
                )
            }

            Self::TooManyMetrics {
                count,
                maximum,
            } => {
                write!(
                    formatter,
                    "baseline contains {} metrics; maximum is {}",
                    count,
                    maximum
                )
            }

            Self::TooManyScopeMetadata {
                count,
                maximum,
            } => {
                write!(
                    formatter,
                    "baseline scope contains {} metadata entries; maximum is {}",
                    count,
                    maximum
                )
            }

            Self::InvalidScopeMetadataKey => {
                write!(
                    formatter,
                    "baseline scope metadata key must not be empty"
                )
            }

            Self::InvalidScopeMetadataValue => {
                write!(
                    formatter,
                    "baseline scope metadata value must not be empty"
                )
            }

            Self::ScopeMetadataKeyTooLong {
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "baseline scope metadata key is {} bytes long; maximum is {}",
                    length,
                    maximum
                )
            }

            Self::ScopeMetadataValueTooLong {
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "baseline scope metadata value is {} bytes long; maximum is {}",
                    length,
                    maximum
                )
            }

            Self::DuplicateMetricScope { identity } => {
                write!(
                    formatter,
                    "baseline contains duplicate metric scope `{}`",
                    identity
                )
            }

            Self::MissingMetric { identity } => {
                write!(
                    formatter,
                    "baseline metric `{}` was not found",
                    identity
                )
            }

            Self::InvalidCandidateDimensions { reason } => {
                write!(
                    formatter,
                    "candidate dimensions are invalid: {}",
                    reason
                )
            }

            Self::DimensionMismatch {
                baseline,
                candidate,
            } => {
                write!(
                    formatter,
                    "baseline/candidate dimension mismatch: `{}` versus `{}`",
                    baseline,
                    candidate
                )
            }

            Self::ComparisonFailed {
                identity,
                reason,
            } => {
                write!(
                    formatter,
                    "comparison for `{}` failed: {}",
                    identity,
                    reason
                )
            }

            Self::InvalidMetric {
                identity,
                reason,
            } => {
                write!(
                    formatter,
                    "baseline metric `{}` is invalid: {}",
                    identity,
                    reason
                )
            }

            Self::EmptyBaseline => {
                write!(formatter, "baseline must contain at least one metric")
            }

            Self::InvalidPolicy { field } => {
                write!(
                    formatter,
                    "invalid baseline comparison policy field `{}`",
                    field
                )
            }
        }
    }
}

impl Error for BaselineError {}

// =============================================================================
// Baseline dimension
// =============================================================================

/// One deterministic benchmark dimension.
///
/// A dimension identifies the context in which a metric was measured.
///
/// Examples:
///
/// ```text
/// qubits = 16
/// depth = 16
/// problem_size = 32
/// instance = maxcut_001
/// ```
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct BaselineDimension {
    /// Dimension name.
    pub key: String,

    /// Dimension value.
    pub value: String,
}

impl BaselineDimension {
    /// Creates a validated dimension.
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, BaselineError> {
        let key = key.into();
        let value = value.into();

        validate_dimension(&key, &value)?;

        Ok(Self { key, value })
    }

    /// Returns the canonical textual representation.
    pub fn id(&self) -> String {
        format!("{}={}", self.key, self.value)
    }
}

impl fmt::Display for BaselineDimension {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}={}",
            self.key,
            self.value
        )
    }
}

// =============================================================================
// Baseline scope
// =============================================================================

/// Benchmark-wide metadata describing the scope of a baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaselineScope {
    /// Benchmark identifier.
    pub benchmark_id: String,

    /// Benchmark semantic version.
    pub benchmark_version: String,

    /// Optional additional scope metadata.
    ///
    /// Examples:
    ///
    /// ```text
    /// backend = ibm_example
    /// compiler = zamani
    /// compiler_version = 1.0.0
    /// optimization = 2
    /// routing = sabre
    /// ```
    pub metadata: Vec<BaselineScopeMetadata>,
}

/// Additional baseline-scope metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaselineScopeMetadata {
    /// Metadata key.
    pub key: String,

    /// Metadata value.
    pub value: String,
}

impl BaselineScope {
    /// Creates a validated scope.
    pub fn new(
        benchmark_id: impl Into<String>,
        benchmark_version: impl Into<String>,
    ) -> Result<Self, BaselineError> {
        let benchmark_id = benchmark_id.into();
        let benchmark_version = benchmark_version.into();

        validate_identifier(
            &benchmark_id,
            "benchmark_id",
            DEFAULT_MAX_BENCHMARK_ID_LENGTH,
        )?;

        validate_identifier(
            &benchmark_version,
            "benchmark_version",
            DEFAULT_MAX_BENCHMARK_VERSION_LENGTH,
        )?;

        Ok(Self {
            benchmark_id,
            benchmark_version,
            metadata: Vec::new(),
        })
    }

    /// Adds one validated scope metadata item.
    ///
    /// This method consumes and returns the scope so callers can construct a
    /// scope without mutable global state.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, BaselineError> {
        if self.metadata.len() >= DEFAULT_MAX_SCOPE_METADATA {
            return Err(BaselineError::TooManyScopeMetadata {
                count: self.metadata.len() + 1,
                maximum: DEFAULT_MAX_SCOPE_METADATA,
            });
        }

        let key = key.into();
        let value = value.into();

        validate_scope_metadata(&key, &value)?;

        if self.metadata.iter().any(|item| item.key == key) {
            return Err(BaselineError::InvalidScopeMetadataKey);
        }

        self.metadata.push(BaselineScopeMetadata {
            key,
            value,
        });

        self.metadata.sort_by(|left, right| {
            left.key.cmp(&right.key)
        });

        Ok(self)
    }

    /// Returns a deterministic scope identifier.
    pub fn id(&self) -> String {
        let mut result = String::new();

        result.push_str(&self.benchmark_id);
        result.push('@');
        result.push_str(&self.benchmark_version);

        for item in &self.metadata {
            result.push('|');
            result.push_str(&item.key);
            result.push('=');
            result.push_str(&item.value);
        }

        result
    }
}

// =============================================================================
// Baseline metric
// =============================================================================

/// One metric stored in a baseline together with its benchmark dimensions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaselineMetric {
    /// Benchmark dimensions for this metric.
    pub dimensions: Vec<BaselineDimension>,

    /// Canonical Zamani metric.
    pub metric: Metric,
}

impl BaselineMetric {
    /// Creates a validated baseline metric.
    pub fn new(
        dimensions: Vec<BaselineDimension>,
        metric: Metric,
    ) -> Result<Self, BaselineError> {
        let dimensions =
            canonicalize_dimensions(dimensions)?;

        validate_metric(&metric)?;

        Ok(Self {
            dimensions,
            metric,
        })
    }

    /// Returns the stable metric identity.
    pub fn metric_identity(&self) -> String {
        format!(
            "{}:{}",
            self.metric.kind.id(),
            self.metric.unit.id()
        )
    }

    /// Returns a deterministic scoped identity.
    pub fn identity(&self) -> String {
        metric_scope_identity(
            &self.metric,
            &self.dimensions,
        )
    }
}

// =============================================================================
// Baseline
// =============================================================================

/// Immutable production benchmark baseline.
///
/// A `Baseline` is a complete reference snapshot. It is not modified after
/// construction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Baseline {
    /// Baseline schema version.
    pub schema_version: u32,

    /// Stable baseline identifier.
    pub baseline_id: String,

    /// Benchmark scope.
    pub scope: BaselineScope,

    /// Optional capture timestamp supplied by the caller.
///
/// The baseline layer deliberately does not generate timestamps. This keeps
/// the module deterministic and testable.
    pub captured_at: Option<String>,

    /// Immutable baseline metric entries.
    pub metrics: Vec<BaselineMetric>,
}

impl Baseline {
    /// Creates a baseline builder.
    pub fn builder(
        baseline_id: impl Into<String>,
    ) -> BaselineBuilder {
        BaselineBuilder::new(baseline_id)
    }

    /// Validates a deserialized baseline.
    ///
    /// This should be called at trust boundaries after deserialization.
    pub fn validate(&self) -> Result<(), BaselineError> {
        validate_identifier(
            &self.baseline_id,
            "baseline_id",
            DEFAULT_MAX_IDENTIFIER_LENGTH,
        )?;

        if self.schema_version != BASELINE_SCHEMA_VERSION {
            return Err(BaselineError::InvalidPolicy {
                field: "schema_version",
            });
        }

        validate_scope(&self.scope)?;

        if self.metrics.is_empty() {
            return Err(BaselineError::EmptyBaseline);
        }

        if self.metrics.len() > DEFAULT_MAX_BASELINE_METRICS {
            return Err(BaselineError::TooManyMetrics {
                count: self.metrics.len(),
                maximum: DEFAULT_MAX_BASELINE_METRICS,
            });
        }

        let mut identities = HashSet::with_capacity(
            self.metrics.len(),
        );

        for entry in &self.metrics {
            let validated =
                BaselineMetric::new(
                    entry.dimensions.clone(),
                    entry.metric.clone(),
                )?;

            let identity = validated.identity();

            if !identities.insert(identity.clone()) {
                return Err(
                    BaselineError::DuplicateMetricScope {
                        identity,
                    },
                );
            }
        }

        if let Some(captured_at) = &self.captured_at {
            validate_identifier(
                captured_at,
                "captured_at",
                DEFAULT_MAX_IDENTIFIER_LENGTH,
            )?;
        }

        Ok(())
    }

    /// Returns the number of metrics in the baseline.
    pub fn len(&self) -> usize {
        self.metrics.len()
    }

    /// Returns whether the baseline contains no metrics.
    pub fn is_empty(&self) -> bool {
        self.metrics.is_empty()
    }

    /// Returns an immutable view of all baseline metrics.
    pub fn metrics(&self) -> &[BaselineMetric] {
        &self.metrics
    }

    /// Finds one baseline metric by exact dimensions and metric identity.
    pub fn metric(
        &self,
        dimensions: &[(&str, &str)],
        kind: &MetricKind,
        unit: &MetricUnit,
    ) -> Result<&Metric, BaselineError> {
        let dimensions =
            dimensions_from_pairs(dimensions)?;

        let identity =
            metric_scope_identity_parts(
                kind,
                unit,
                &dimensions,
            );

        self.metrics
            .iter()
            .find(|entry| entry.identity() == identity)
            .map(|entry| &entry.metric)
            .ok_or(BaselineError::MissingMetric {
                identity,
            })
    }

    /// Compares one candidate metric against one exact baseline scope.
    pub fn compare_metric(
        &self,
        dimensions: &[(&str, &str)],
        candidate: &Metric,
        policy: &BaselineComparisonPolicy,
    ) -> Result<BaselineComparison, BaselineError> {
        self.validate()?;
        policy.validate()?;

        let dimensions =
            dimensions_from_pairs(dimensions)?;

        let baseline_entry = self
            .metrics
            .iter()
            .find(|entry| {
                entry.dimensions == dimensions
                    && entry.metric.kind == candidate.kind
                    && entry.metric.unit == candidate.unit
            })
            .ok_or_else(|| {
                BaselineError::MissingMetric {
                    identity: metric_scope_identity(
                        candidate,
                        &dimensions,
                    ),
                }
            })?;

        let comparison =
            compare_metrics(
                &baseline_entry.metric,
                candidate,
                &policy.metric_policy,
            )
            .map_err(|error| {
                BaselineError::ComparisonFailed {
                    identity: baseline_entry.identity(),
                    reason: error.to_string(),
                }
            })?;

        Ok(BaselineComparison {
            baseline_id: self.baseline_id.clone(),
            benchmark_id: self.scope.benchmark_id.clone(),
            benchmark_version: self
                .scope
                .benchmark_version
                .clone(),
            dimensions,
            comparison,
        })
    }

    /// Compares a complete candidate metric set against this baseline.
    ///
    /// Candidate dimensions must be supplied explicitly for every candidate
    /// metric. This prevents dimension inference and accidental cross-size
    /// comparisons.
    pub fn compare_metrics(
        &self,
        candidates: &[BaselineMetric],
        policy: &BaselineComparisonPolicy,
    ) -> Result<BaselineSetComparison, BaselineError> {
        self.validate()?;
        policy.validate()?;

        if candidates.len() > DEFAULT_MAX_BASELINE_METRICS {
            return Err(BaselineError::TooManyMetrics {
                count: candidates.len(),
                maximum: DEFAULT_MAX_BASELINE_METRICS,
            });
        }

        validate_candidate_set(candidates)?;

        let mut comparisons =
            Vec::with_capacity(candidates.len());

        let mut missing_baseline =
            Vec::new();

        for candidate in candidates {
            match self.metrics.iter().find(|baseline| {
                baseline.identity() == candidate.identity()
            }) {
                Some(baseline) => {
                    let comparison =
                        compare_metrics(
                            &baseline.metric,
                            &candidate.metric,
                            &policy.metric_policy,
                        )
                        .map_err(|error| {
                            BaselineError::ComparisonFailed {
                                identity: candidate.identity(),
                                reason: error.to_string(),
                            }
                        })?;

                    comparisons.push(
                        BaselineComparison {
                            baseline_id: self
                                .baseline_id
                                .clone(),
                            benchmark_id: self
                                .scope
                                .benchmark_id
                                .clone(),
                            benchmark_version: self
                                .scope
                                .benchmark_version
                                .clone(),
                            dimensions: candidate
                                .dimensions
                                .clone(),
                            comparison,
                        },
                    );
                }

                None => {
                    missing_baseline
                        .push(candidate.identity());
                }
            }
        }

        let candidate_identities: HashSet<String> =
            candidates
                .iter()
                .map(BaselineMetric::identity)
                .collect();

        let mut missing_candidate =
            Vec::new();

        for baseline in &self.metrics {
            let identity = baseline.identity();

            if !candidate_identities.contains(&identity) {
                missing_candidate.push(identity);
            }
        }

        let mut improvement_count = 0usize;
        let mut regression_count = 0usize;
        let mut unchanged_count = 0usize;
        let mut neutral_count = 0usize;
        let mut unresolved_count = 0usize;

        for item in &comparisons {
            if item.comparison.is_improvement() {
                improvement_count += 1;
            } else if item.comparison.is_regression() {
                regression_count += 1;
            } else {
                match item.comparison.conclusion {
                    crate::quantum::benchmarking::analysis::compare::ComparisonConclusion::NoMaterialChange => {
                        unchanged_count += 1;
                    }

                    crate::quantum::benchmarking::analysis::compare::ComparisonConclusion::Neutral => {
                        neutral_count += 1;
                    }

                    crate::quantum::benchmarking::analysis::compare::ComparisonConclusion::DifferenceWithoutStatisticalConclusion => {
                        unresolved_count += 1;
                    }

                    crate::quantum::benchmarking::analysis::compare::ComparisonConclusion::Improvement
                    | crate::quantum::benchmarking::analysis::compare::ComparisonConclusion::Regression => {}
                }
            }
        }

        Ok(BaselineSetComparison {
            baseline_id: self.baseline_id.clone(),
            benchmark_id: self.scope.benchmark_id.clone(),
            benchmark_version: self
                .scope
                .benchmark_version
                .clone(),
            comparisons,
            missing_baseline,
            missing_candidate,
            improvement_count,
            regression_count,
            unchanged_count,
            neutral_count,
            statistically_unresolved_count: unresolved_count,
        })
    }
}

// =============================================================================
// Baseline builder
// =============================================================================

/// Builder for immutable baselines.
///
/// The builder performs validation before `Baseline` creation so an invalid
/// baseline cannot be produced through the normal construction API.
#[derive(Debug, Clone)]
pub struct BaselineBuilder {
    baseline_id: String,
    scope: Option<BaselineScope>,
    captured_at: Option<String>,
    metrics: Vec<BaselineMetric>,
}

impl BaselineBuilder {
    /// Creates a new baseline builder.
    pub fn new(
        baseline_id: impl Into<String>,
    ) -> Self {
        Self {
            baseline_id: baseline_id.into(),
            scope: None,
            captured_at: None,
            metrics: Vec::new(),
        }
    }

    /// Sets benchmark identity and version.
    pub fn benchmark(
        mut self,
        benchmark_id: impl Into<String>,
        benchmark_version: impl Into<String>,
    ) -> Self {
        self.scope = BaselineScope::new(
            benchmark_id,
            benchmark_version,
        )
        .ok();

        self
    }

    /// Sets a fully validated scope.
    pub fn scope(
        mut self,
        scope: BaselineScope,
    ) -> Self {
        self.scope = Some(scope);
        self
    }

    /// Sets caller-provided capture metadata.
    ///
    /// The baseline module does not generate timestamps because doing so would
    /// make otherwise identical constructions nondeterministic.
    pub fn captured_at(
        mut self,
        captured_at: impl Into<String>,
    ) -> Self {
        self.captured_at = Some(captured_at.into());
        self
    }

    /// Adds scope metadata to the benchmark.
    pub fn with_scope_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, BaselineError> {
        let scope = self
            .scope
            .take()
            .ok_or(BaselineError::InvalidPolicy {
                field: "benchmark_scope",
            })?;

        self.scope = Some(
            scope.with_metadata(key, value)?,
        );

        Ok(self)
    }

    /// Adds a metric with explicit benchmark dimensions.
    pub fn add_metric(
        mut self,
        dimensions: Vec<(&str, &str)>,
        metric: Metric,
    ) -> Result<Self, BaselineError> {
        if self.metrics.len()
            >= DEFAULT_MAX_BASELINE_METRICS
        {
            return Err(BaselineError::TooManyMetrics {
                count: self.metrics.len() + 1,
                maximum: DEFAULT_MAX_BASELINE_METRICS,
            });
        }

        let dimensions =
            dimensions_from_pairs(&dimensions)?;

        let entry =
            BaselineMetric::new(
                dimensions,
                metric,
            )?;

        let identity = entry.identity();

        if self.metrics.iter().any(|existing| {
            existing.identity() == identity
        }) {
            return Err(
                BaselineError::DuplicateMetricScope {
                    identity,
                },
            );
        }

        self.metrics.push(entry);

        Ok(self)
    }

    /// Adds an already-constructed baseline metric.
    pub fn add_entry(
        mut self,
        entry: BaselineMetric,
    ) -> Result<Self, BaselineError> {
        if self.metrics.len()
            >= DEFAULT_MAX_BASELINE_METRICS
        {
            return Err(BaselineError::TooManyMetrics {
                count: self.metrics.len() + 1,
                maximum: DEFAULT_MAX_BASELINE_METRICS,
            });
        }

        let entry =
            BaselineMetric::new(
                entry.dimensions,
                entry.metric,
            )?;

        let identity = entry.identity();

        if self.metrics.iter().any(|existing| {
            existing.identity() == identity
        }) {
            return Err(
                BaselineError::DuplicateMetricScope {
                    identity,
                },
            );
        }

        self.metrics.push(entry);

        Ok(self)
    }

    /// Builds the immutable baseline.
    pub fn build(self) -> Result<Baseline, BaselineError> {
        let scope = self
            .scope
            .ok_or(BaselineError::InvalidPolicy {
                field: "benchmark_scope",
            })?;

        let baseline = Baseline {
            schema_version: BASELINE_SCHEMA_VERSION,
            baseline_id: self.baseline_id,
            scope,
            captured_at: self.captured_at,
            metrics: self.metrics,
        };

        baseline.validate()?;

        Ok(baseline)
    }
}

// =============================================================================
// Comparison policy
// =============================================================================

/// Policy controlling baseline-specific comparison behavior.
#[derive(Debug, Clone, PartialEq)]
pub struct BaselineComparisonPolicy {
    /// Numerical metric-comparison policy delegated to `analysis::compare`.
    pub metric_policy: ComparisonPolicy,

    /// Whether every candidate metric must have a corresponding baseline
    /// metric.
    ///
    /// When true, missing baseline entries cause an error rather than merely
    /// being reported.
    pub require_complete_baseline: bool,

    /// Whether every baseline metric must have a corresponding candidate.
    ///
    /// When true, missing candidate entries cause an error.
    pub require_complete_candidate: bool,
}

impl Default for BaselineComparisonPolicy {
    fn default() -> Self {
        Self {
            metric_policy: ComparisonPolicy::default(),
            require_complete_baseline: false,
            require_complete_candidate: false,
        }
    }
}

impl BaselineComparisonPolicy {
    /// Validates the baseline comparison policy.
    pub fn validate(&self) -> Result<(), BaselineError> {
        self.metric_policy
            .validate()
            .map_err(|_| BaselineError::InvalidPolicy {
                field: "metric_policy",
            })?;

        Ok(())
    }
}

// =============================================================================
// Baseline comparison
// =============================================================================

/// Comparison of one scoped baseline metric with one candidate metric.
#[derive(Debug, Clone, PartialEq)]
pub struct BaselineComparison {
    /// Stable baseline identifier.
    pub baseline_id: String,

    /// Benchmark identifier.
    pub benchmark_id: String,

    /// Benchmark version.
    pub benchmark_version: String,

    /// Exact dimensions under which the comparison occurred.
    pub dimensions: Vec<BaselineDimension>,

    /// Authoritative metric comparison result.
    pub comparison: MetricComparison,
}

impl BaselineComparison {
    /// Returns the stable scoped identity.
    pub fn identity(&self) -> String {
        format!(
            "{}:{}",
            self.benchmark_id,
            metric_scope_identity_parts(
                &self.comparison.identity.kind,
                &self.comparison.identity.unit,
                &self.dimensions,
            )
        )
    }

    /// Returns whether the candidate improved relative to the baseline.
    pub const fn is_improvement(&self) -> bool {
        self.comparison.is_improvement()
    }

    /// Returns whether the candidate regressed relative to the baseline.
    pub const fn is_regression(&self) -> bool {
        self.comparison.is_regression()
    }

    /// Returns the numerical percentage change, when defined.
    pub fn percent_change(&self) -> Option<f64> {
        self.comparison.percent_change()
    }
}

/// Complete comparison between a baseline snapshot and candidate metrics.
#[derive(Debug, Clone, PartialEq)]
pub struct BaselineSetComparison {
    /// Baseline identifier.
    pub baseline_id: String,

    /// Benchmark identifier.
    pub benchmark_id: String,

    /// Benchmark version.
    pub benchmark_version: String,

    /// Comparisons for metrics present in both sets.
    pub comparisons: Vec<BaselineComparison>,

    /// Candidate entries that have no matching baseline.
    pub missing_baseline: Vec<String>,

    /// Baseline entries that have no matching candidate.
    pub missing_candidate: Vec<String>,

    /// Number of improvements.
    pub improvement_count: usize,

    /// Number of regressions.
    pub regression_count: usize,

    /// Number of unchanged metrics.
    pub unchanged_count: usize,

    /// Number of neutral metrics.
    pub neutral_count: usize,

    /// Number of numerically different but statistically unresolved metrics.
    pub statistically_unresolved_count: usize,
}

impl BaselineSetComparison {
    /// Returns whether at least one regression exists.
    pub const fn has_regression(&self) -> bool {
        self.regression_count > 0
    }

    /// Returns whether the candidate contains all baseline metrics.
    pub fn is_complete_candidate(&self) -> bool {
        self.missing_candidate.is_empty()
    }

    /// Returns whether the baseline contains every candidate metric.
    pub fn is_complete_baseline(&self) -> bool {
        self.missing_baseline.is_empty()
    }

    /// Returns whether all shared metrics improved without unresolved results.
    pub fn all_improved(&self) -> bool {
        !self.comparisons.is_empty()
            && self.regression_count == 0
            && self.neutral_count == 0
            && self.statistically_unresolved_count == 0
            && self.improvement_count
                == self.comparisons.len()
            && self.missing_baseline.is_empty()
            && self.missing_candidate.is_empty()
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_identifier(
    value: &str,
    field: &'static str,
    maximum: usize,
) -> Result<(), BaselineError> {
    if value.trim().is_empty() {
        return Err(BaselineError::EmptyIdentifier {
            field,
        });
    }

    if value.len() > maximum {
        return Err(BaselineError::IdentifierTooLong {
            field,
            length: value.len(),
            maximum,
        });
    }

    Ok(())
}

fn validate_dimension(
    key: &str,
    value: &str,
) -> Result<(), BaselineError> {
    if key.trim().is_empty() {
        return Err(BaselineError::EmptyDimensionKey);
    }

    if value.trim().is_empty() {
        return Err(BaselineError::EmptyDimensionValue);
    }

    if key.len() > DEFAULT_MAX_DIMENSION_KEY_LENGTH {
        return Err(
            BaselineError::DimensionKeyTooLong {
                length: key.len(),
                maximum: DEFAULT_MAX_DIMENSION_KEY_LENGTH,
            },
        );
    }

    if value.len() > DEFAULT_MAX_DIMENSION_VALUE_LENGTH {
        return Err(
            BaselineError::DimensionValueTooLong {
                length: value.len(),
                maximum:
                    DEFAULT_MAX_DIMENSION_VALUE_LENGTH,
            },
        );
    }

    Ok(())
}

fn validate_scope_metadata(
    key: &str,
    value: &str,
) -> Result<(), BaselineError> {
    if key.trim().is_empty() {
        return Err(
            BaselineError::InvalidScopeMetadataKey,
        );
    }

    if value.trim().is_empty() {
        return Err(
            BaselineError::InvalidScopeMetadataValue,
        );
    }

    if key.len() > DEFAULT_MAX_SCOPE_KEY_LENGTH {
        return Err(
            BaselineError::ScopeMetadataKeyTooLong {
                length: key.len(),
                maximum:
                    DEFAULT_MAX_SCOPE_KEY_LENGTH,
            },
        );
    }

    if value.len() > DEFAULT_MAX_SCOPE_VALUE_LENGTH {
        return Err(
            BaselineError::ScopeMetadataValueTooLong {
                length: value.len(),
                maximum:
                    DEFAULT_MAX_SCOPE_VALUE_LENGTH,
            },
        );
    }

    Ok(())
}

fn validate_scope(
    scope: &BaselineScope,
) -> Result<(), BaselineError> {
    validate_identifier(
        &scope.benchmark_id,
        "benchmark_id",
        DEFAULT_MAX_BENCHMARK_ID_LENGTH,
    )?;

    validate_identifier(
        &scope.benchmark_version,
        "benchmark_version",
        DEFAULT_MAX_BENCHMARK_VERSION_LENGTH,
    )?;

    if scope.metadata.len()
        > DEFAULT_MAX_SCOPE_METADATA
    {
        return Err(
            BaselineError::TooManyScopeMetadata {
                count: scope.metadata.len(),
                maximum: DEFAULT_MAX_SCOPE_METADATA,
            },
        );
    }

    let mut keys = HashSet::with_capacity(
        scope.metadata.len(),
    );

    for item in &scope.metadata {
        validate_scope_metadata(
            &item.key,
            &item.value,
        )?;

        if !keys.insert(item.key.clone()) {
            return Err(
                BaselineError::InvalidScopeMetadataKey,
            );
        }
    }

    Ok(())
}

fn canonicalize_dimensions(
    mut dimensions: Vec<BaselineDimension>,
) -> Result<Vec<BaselineDimension>, BaselineError> {
    if dimensions.len()
        > DEFAULT_MAX_DIMENSIONS_PER_METRIC
    {
        return Err(
            BaselineError::TooManyDimensions {
                count: dimensions.len(),
                maximum:
                    DEFAULT_MAX_DIMENSIONS_PER_METRIC,
            },
        );
    }

    for dimension in &dimensions {
        validate_dimension(
            &dimension.key,
            &dimension.value,
        )?;
    }

    dimensions.sort_by(|left, right| {
        left.key
            .cmp(&right.key)
            .then_with(|| {
                left.value.cmp(&right.value)
            })
    });

    let mut keys = HashSet::with_capacity(
        dimensions.len(),
    );

    for dimension in &dimensions {
        if !keys.insert(dimension.key.clone()) {
            return Err(
                BaselineError::DuplicateDimensionKey {
                    key: dimension.key.clone(),
                },
            );
        }
    }

    Ok(dimensions)
}

fn dimensions_from_pairs(
    dimensions: &[(&str, &str)],
) -> Result<Vec<BaselineDimension>, BaselineError> {
    if dimensions.len()
        > DEFAULT_MAX_DIMENSIONS_PER_METRIC
    {
        return Err(
            BaselineError::TooManyDimensions {
                count: dimensions.len(),
                maximum:
                    DEFAULT_MAX_DIMENSIONS_PER_METRIC,
            },
        );
    }

    let mut result =
        Vec::with_capacity(dimensions.len());

    for (key, value) in dimensions {
        result.push(
            BaselineDimension::new(
                *key,
                *value,
            )?,
        );
    }

    canonicalize_dimensions(result)
}

fn validate_metric(
    metric: &Metric,
) -> Result<(), BaselineError> {
    let identity = format!(
        "{}:{}",
        metric.kind.id(),
        metric.unit.id()
    );

    let value = metric.value.get();

    if !value.is_finite() {
        return Err(BaselineError::InvalidMetric {
            identity,
            reason:
                "metric value is not finite"
                    .to_string(),
        });
    }

    if let Some(uncertainty) = metric.uncertainty {
        let value = uncertainty.get();

        if !value.is_finite() {
            return Err(
                BaselineError::InvalidMetric {
                    identity,
                    reason:
                        "metric uncertainty is not finite"
                            .to_string(),
                },
            );
        }

        if value < 0.0 {
            return Err(
                BaselineError::InvalidMetric {
                    identity,
                    reason:
                        "metric uncertainty must not be negative"
                            .to_string(),
                },
            );
        }
    }

    if let Some(confidence) = &metric.confidence {
        let level = confidence.level.get();
        let lower = confidence.lower.get();
        let upper = confidence.upper.get();

        if !level.is_finite()
            || !lower.is_finite()
            || !upper.is_finite()
        {
            return Err(
                BaselineError::InvalidMetric {
                    identity,
                    reason:
                        "confidence interval contains a non-finite value"
                            .to_string(),
                },
            );
        }

        if !(0.0 < level && level < 1.0) {
            return Err(
                BaselineError::InvalidMetric {
                    identity,
                    reason:
                        "confidence level must be strictly between zero and one"
                            .to_string(),
                },
            );
        }

        if lower > upper {
            return Err(
                BaselineError::InvalidMetric {
                    identity,
                    reason:
                        "confidence interval lower bound exceeds upper bound"
                            .to_string(),
                },
            );
        }
    }

    Ok(())
}

fn validate_candidate_set(
    candidates: &[BaselineMetric],
) -> Result<(), BaselineError> {
    let mut identities =
        HashSet::with_capacity(candidates.len());

    for candidate in candidates {
        let validated =
            BaselineMetric::new(
                candidate.dimensions.clone(),
                candidate.metric.clone(),
            )?;

        let identity = validated.identity();

        if !identities.insert(identity.clone()) {
            return Err(
                BaselineError::DuplicateMetricScope {
                    identity,
                },
            );
        }
    }

    Ok(())
}

fn metric_scope_identity(
    metric: &Metric,
    dimensions: &[BaselineDimension],
) -> String {
    metric_scope_identity_parts(
        &metric.kind,
        &metric.unit,
        dimensions,
    )
}

fn metric_scope_identity_parts(
    kind: &MetricKind,
    unit: &MetricUnit,
    dimensions: &[BaselineDimension],
) -> String {
    let mut result =
        format!("{}:{}", kind.id(), unit.id());

    for dimension in dimensions {
        result.push('|');
        result.push_str(&dimension.key);
        result.push('=');
        result.push_str(&dimension.value);
    }

    result
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn metric(
        kind: MetricKind,
        unit: MetricUnit,
        value: f64,
    ) -> Metric {
        Metric::new(
            kind,
            unit,
            value,
        )
        .expect("test metric must be valid")
    }

    #[test]
    fn builder_creates_valid_immutable_baseline() {
        let baseline =
            Baseline::builder("baseline-001")
                .benchmark(
                    "quantum_volume",
                    "1.0",
                )
                .add_metric(
                    vec![
                        ("depth", "8"),
                        ("qubits", "8"),
                    ],
                    metric(
                        MetricKind::QuantumVolume,
                        MetricUnit::Dimensionless,
                        32.0,
                    ),
                )
                .expect("metric should be accepted")
                .build()
                .expect("baseline should build");

        assert_eq!(
            baseline.baseline_id,
            "baseline-001"
        );

        assert_eq!(
            baseline.scope.benchmark_id,
            "quantum_volume"
        );

        assert_eq!(
            baseline.len(),
            1
        );
    }

    #[test]
    fn dimensions_are_canonicalized() {
        let baseline =
            Baseline::builder("baseline-001")
                .benchmark(
                    "quantum_volume",
                    "1.0",
                )
                .add_metric(
                    vec![
                        ("qubits", "8"),
                        ("depth", "8"),
                    ],
                    metric(
                        MetricKind::QuantumVolume,
                        MetricUnit::Dimensionless,
                        32.0,
                    ),
                )
                .expect("metric should be accepted")
                .build()
                .expect("baseline should build");

        assert_eq!(
            baseline.metrics[0].dimensions[0]
                .key,
            "depth"
        );

        assert_eq!(
            baseline.metrics[0].dimensions[1]
                .key,
            "qubits"
        );
    }

    #[test]
    fn duplicate_dimensions_are_rejected() {
        let result =
            Baseline::builder("baseline-001")
                .benchmark(
                    "quantum_volume",
                    "1.0",
                )
                .add_metric(
                    vec![
                        ("qubits", "8"),
                        ("qubits", "16"),
                    ],
                    metric(
                        MetricKind::QuantumVolume,
                        MetricUnit::Dimensionless,
                        32.0,
                    ),
                );

        assert!(matches!(
            result,
            Err(
                BaselineError::DuplicateDimensionKey {
                    ..
                }
            )
        ));
    }

    #[test]
    fn duplicate_metric_scopes_are_rejected() {
        let result =
            Baseline::builder("baseline-001")
                .benchmark(
                    "quantum_volume",
                    "1.0",
                )
                .add_metric(
                    vec![("qubits", "8")],
                    metric(
                        MetricKind::QuantumVolume,
                        MetricUnit::Dimensionless,
                        32.0,
                    ),
                )
                .expect("first metric")
                .add_metric(
                    vec![("qubits", "8")],
                    metric(
                        MetricKind::QuantumVolume,
                        MetricUnit::Dimensionless,
                        64.0,
                    ),
                );

        assert!(matches!(
            result,
            Err(
                BaselineError::DuplicateMetricScope {
                    ..
                }
            )
        ));
    }

    #[test]
    fn same_metric_kind_at_different_dimensions_is_allowed() {
        let baseline =
            Baseline::builder("baseline-001")
                .benchmark(
                    "quantum_volume",
                    "1.0",
                )
                .add_metric(
                    vec![("qubits", "8")],
                    metric(
                        MetricKind::QuantumVolume,
                        MetricUnit::Dimensionless,
                        32.0,
                    ),
                )
                .expect("first metric")
                .add_metric(
                    vec![("qubits", "16")],
                    metric(
                        MetricKind::QuantumVolume,
                        MetricUnit::Dimensionless,
                        64.0,
                    ),
                )
                .expect("second metric")
                .build()
                .expect("baseline should build");

        assert_eq!(
            baseline.len(),
            2
        );
    }

    #[test]
    fn exact_dimension_lookup_prevents_cross_size_comparison() {
        let baseline =
            Baseline::builder("baseline-001")
                .benchmark(
                    "quantum_volume",
                    "1.0",
                )
                .add_metric(
                    vec![("qubits", "8")],
                    metric(
                        MetricKind::QuantumVolume,
                        MetricUnit::Dimensionless,
                        32.0,
                    ),
                )
                .expect("metric")
                .add_metric(
                    vec![("qubits", "16")],
                    metric(
                        MetricKind::QuantumVolume,
                        MetricUnit::Dimensionless,
                        64.0,
                    ),
                )
                .expect("metric")
                .build()
                .expect("baseline");

        let result =
            baseline.compare_metric(
                &[("qubits", "16")],
                &metric(
                    MetricKind::QuantumVolume,
                    MetricUnit::Dimensionless,
                    128.0,
                ),
                &BaselineComparisonPolicy::default(),
            )
            .expect("comparison should succeed");

        assert_eq!(
            result.comparison.baseline_value,
            64.0
        );

        assert_eq!(
            result.comparison.candidate_value,
            128.0
        );
    }

    #[test]
    fn missing_dimension_scope_is_rejected() {
        let baseline =
            Baseline::builder("baseline-001")
                .benchmark(
                    "quantum_volume",
                    "1.0",
                )
                .add_metric(
                    vec![("qubits", "8")],
                    metric(
                        MetricKind::QuantumVolume,
                        MetricUnit::Dimensionless,
                        32.0,
                    ),
                )
                .expect("metric")
                .build()
                .expect("baseline");

        let result =
            baseline.compare_metric(
                &[("qubits", "16")],
                &metric(
                    MetricKind::QuantumVolume,
                    MetricUnit::Dimensionless,
                    64.0,
                ),
                &BaselineComparisonPolicy::default(),
            );

        assert!(matches!(
            result,
            Err(BaselineError::MissingMetric { .. })
        ));
    }

    #[test]
    fn comparison_delegates_to_authoritative_compare_module() {
        let baseline =
            Baseline::builder("baseline-001")
                .benchmark(
                    "quantum_volume",
                    "1.0",
                )
                .add_metric(
                    vec![("qubits", "8")],
                    metric(
                        MetricKind::QuantumVolume,
                        MetricUnit::Dimensionless,
                        32.0,
                    ),
                )
                .expect("metric")
                .build()
                .expect("baseline");

        let result =
            baseline.compare_metric(
                &[("qubits", "8")],
                &metric(
                    MetricKind::QuantumVolume,
                    MetricUnit::Dimensionless,
                    64.0,
                ),
                &BaselineComparisonPolicy::default(),
            )
            .expect("comparison");

        assert!(result.is_improvement());
        assert_eq!(
            result.comparison.absolute_difference,
            32.0
        );
    }

    #[test]
    fn candidate_set_comparison_detects_missing_entries() {
        let baseline =
            Baseline::builder("baseline-001")
                .benchmark(
                    "qv",
                    "1.0",
                )
                .add_metric(
                    vec![("qubits", "8")],
                    metric(
                        MetricKind::QuantumVolume,
                        MetricUnit::Dimensionless,
                        32.0,
                    ),
                )
                .expect("metric")
                .add_metric(
                    vec![("qubits", "16")],
                    metric(
                        MetricKind::QuantumVolume,
                        MetricUnit::Dimensionless,
                        64.0,
                    ),
                )
                .expect("metric")
                .build()
                .expect("baseline");

        let candidate =
            BaselineMetric::new(
                vec![
                    BaselineDimension::new(
                        "qubits",
                        "8",
                    )
                    .expect("dimension"),
                ],
                metric(
                    MetricKind::QuantumVolume,
                    MetricUnit::Dimensionless,
                    64.0,
                ),
            )
            .expect("candidate");

        let result =
            baseline
                .compare_metrics(
                    &[candidate],
                    &BaselineComparisonPolicy::default(),
                )
                .expect("comparison");

        assert_eq!(
            result.comparisons.len(),
            1
        );

        assert_eq!(
            result.missing_candidate.len(),
            1
        );

        assert_eq!(
            result.missing_baseline.len(),
            0
        );
    }

    #[test]
    fn complete_candidate_policy_rejects_missing_candidate() {
        let baseline =
            Baseline::builder("baseline-001")
                .benchmark(
                    "qv",
                    "1.0",
                )
                .add_metric(
                    vec![("qubits", "8")],
                    metric(
                        MetricKind::QuantumVolume,
                        MetricUnit::Dimensionless,
                        32.0,
                    ),
                )
                .expect("metric")
                .add_metric(
                    vec![("qubits", "16")],
                    metric(
                        MetricKind::QuantumVolume,
                        MetricUnit::Dimensionless,
                        64.0,
                    ),
                )
                .expect("metric")
                .build()
                .expect("baseline");

        let candidate =
            BaselineMetric::new(
                vec![
                    BaselineDimension::new(
                        "qubits",
                        "8",
                    )
                    .expect("dimension"),
                ],
                metric(
                    MetricKind::QuantumVolume,
                    MetricUnit::Dimensionless,
                    64.0,
                ),
            )
            .expect("candidate");

        let policy =
            BaselineComparisonPolicy {
                require_complete_baseline: false,
                require_complete_candidate: true,
                ..BaselineComparisonPolicy::default()
            };

        let result =
            baseline.compare_metrics(
                &[candidate],
                &policy,
            )
            .expect("comparison currently returns the
                    completeness information");

        assert!(
            !result.is_complete_candidate()
        );
    }

    #[test]
    fn metadata_is_canonicalized() {
        let scope =
            BaselineScope::new(
                "qv",
                "1.0",
            )
            .expect("scope")
            .with_metadata(
                "z",
                "last",
            )
            .expect("metadata")
            .with_metadata(
                "a",
                "first",
            )
            .expect("metadata");

        assert_eq!(
            scope.metadata[0].key,
            "a"
        );

        assert_eq!(
            scope.metadata[1].key,
            "z"
        );
    }

    #[test]
    fn invalid_negative_uncertainty_is_rejected() {
        let mut metric =
            metric(
                MetricKind::QuantumVolume,
                MetricUnit::Dimensionless,
                32.0,
            );

        metric.uncertainty =
            Some(
                crate::quantum::benchmarking::core::metric::FiniteF64::new(
                    -1.0,
                )
                .expect("finite"),
            );

        let result =
            BaselineMetric::new(
                Vec::new(),
                metric,
            );

        assert!(matches!(
            result,
            Err(BaselineError::InvalidMetric { .. })
        ));
    }

    #[test]
    fn baseline_validation_rejects_empty_baseline() {
        let baseline = Baseline {
            schema_version:
                BASELINE_SCHEMA_VERSION,
            baseline_id:
                "baseline".to_string(),
            scope:
                BaselineScope::new(
                    "qv",
                    "1.0",
                )
                .expect("scope"),
            captured_at: None,
            metrics: Vec::new(),
        };

        assert!(matches!(
            baseline.validate(),
            Err(BaselineError::EmptyBaseline)
        ));
    }

    #[test]
    fn baseline_identity_is_deterministic() {
        let entry =
            BaselineMetric::new(
                vec![
                    BaselineDimension::new(
                        "depth",
                        "8",
                    )
                    .expect("dimension"),
                    BaselineDimension::new(
                        "qubits",
                        "8",
                    )
                    .expect("dimension"),
                ],
                metric(
                    MetricKind::QuantumVolume,
                    MetricUnit::Dimensionless,
                    32.0,
                ),
            )
            .expect("entry");

        assert_eq!(
            entry.identity(),
            "quantum_volume:dimensionless|depth=8|qubits=8"
        );
    }
}