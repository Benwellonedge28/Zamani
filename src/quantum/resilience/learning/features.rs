//! # Zamani Quantum Resilience — Learning Features
//!
//! Path:
//! `src/quantum/resilience/learning/features.rs`
//!
//! ## Purpose
//!
//! This module defines the canonical, provider-independent feature layer for
//! the quantum-resilience learning subsystem.
//!
//! Features convert observations about quantum execution into deterministic,
//! validated, schema-versioned numerical representations suitable for:
//!
//! - prediction models;
//! - anomaly detection;
//! - recovery-success prediction;
//! - failure prediction;
//! - hardware/resource degradation prediction;
//! - mitigation effectiveness estimation;
//! - strategy ranking;
//! - historical analysis;
//! - offline training;
//! - online inference.
//!
//! This module does NOT implement a machine-learning algorithm.
//!
//! ## Architectural ownership
//!
//! This file owns:
//!
//! - feature identifiers;
//! - feature schemas;
//! - feature definitions;
//! - feature types;
//! - feature values;
//! - feature vectors;
//! - feature extraction contracts;
//! - missing-value policy;
//! - normalization contracts;
//! - feature validation;
//! - deterministic feature ordering;
//! - feature-set provenance;
//! - feature extraction context;
//! - canonical logical/physical quantum-resource references.
//!
//! It does NOT own:
//!
//! - model training;
//! - model inference;
//! - prediction;
//! - strategy selection;
//! - recovery;
//! - telemetry collection;
//! - history persistence;
//! - hardware discovery;
//! - routing;
//! - scheduling;
//! - QEC;
//! - noise modelling;
//! - canonical quantum IR.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! ## Integration contract
//!
//! `learning/model.rs`
//!     consumes the numeric feature vector produced by this module.
//!
//! `learning/predictor.rs`
//!     consumes feature vectors and passes them to prediction models.
//!
//! `learning/feedback.rs`
//!     may retain feature schema/version and feature provenance alongside
//!     verified outcomes.
//!
//! `learning/strategy.rs`
//!     may use feature vectors and model predictions when ranking strategies.
//!
//! `history/*`
//!     supplies historical observations from which feature extractors can
//!     construct feature values.
//!
//! `telemetry/*`
//!     supplies current execution/hardware observations.
//!
//! `quantum::ir::qubit`
//!     remains the authoritative source of logical/physical qubit identity.
//!
//! `quantum::hardware`
//!     remains authoritative for hardware capabilities and physical-resource
//!     observations.
//!
//! `quantum::zqn`
//!     remains authoritative for quantum fault/noise semantics.
//!
//! ## Critical safety rule
//!
//! Features are evidence, not authority.
//!
//! A feature-derived prediction MUST NOT directly authorize:
//!
//! - a recovery;
//! - a migration;
//! - a semantic change;
//! - a hardware selection;
//! - a QEC change;
//! - acceptance of an execution result.
//!
//! Those decisions remain subject to resilience policy, capability validation,
//! security checks, and semantic verification.
//!
//! ## Scalability
//!
//! This module contains no fixed:
//!
//! - qubit count;
//! - feature count;
//! - device count;
//! - backend count;
//! - execution count;
//! - model count;
//! - topology size;
//! - training-set size.
//!
//! Collections are dynamically sized and therefore limited only by the
//! configured/runtime resources available to the caller.
//!
//! No `MAX_FEATURES`, `MAX_QUBITS`, `MAX_DEVICES`, or similar architectural
//! constants are defined here.
//!
//! ## Determinism
//!
//! Feature extraction can be required to be deterministic.
//!
//! Deterministic extraction means that identical:
//!
//! - source observations;
//! - feature schema;
//! - feature configuration;
//! - extraction context;
//!
//! produce identical ordered feature values.
//!
//! Feature definitions therefore carry stable identifiers and explicit order.
//!
//! ## Missing data
//!
//! Missing data is never silently converted to zero.
//!
//! A schema explicitly chooses how missing values are handled.
//!
//! Supported policies include:
//!
//! - reject;
//! - constant imputation;
//! - mean;
//! - median;
//! - previous/last-known value;
//! - explicit missing indicator.
//!
//! Any imputation must be visible in the resulting provenance.
//!
//! ## Numerical safety
//!
//! Feature values must be finite unless an explicitly declared future feature
//! type provides a different representation.
//!
//! NaN and positive/negative infinity are rejected because silently passing
//! non-finite values into a prediction model can produce unsafe decisions.
//!
//! ## Rust compatibility
//!
//! - Rust 2021.
//! - Rust 1.97 / 1.97.1.
//! - Stable Rust.
//! - No nightly features.
//! - No `unsafe`.
//!
//! The `#![forbid(unsafe_code)]` attribute below makes the safety requirement
//! compiler-enforced for this module.
//!
//! ## Canonical quantum identity
//!
//! When a feature is associated with a quantum resource, this module uses:
//!
//! `crate::quantum::ir::qubit::QubitId`
//! `crate::quantum::ir::qubit::PhysicalQubitId`
//! `crate::quantum::ir::qubit::QubitRef`
//!
//! It MUST NOT define another qubit identity type.
//!
//! The current canonical IR deliberately keeps logical and physical identities
//! distinct. Feature extraction must preserve that distinction rather than
//! collapsing both into an integer.
//!
//! ## Serialization
//!
//! The portable feature vector/schema types are serializable through `serde`.
//!
//! `FeatureContext` deliberately does not require serialization because it may
//! contain canonical IR resource references whose ownership/serialization
//! belongs to the IR subsystem.
//!
//! A future resilience serialization layer can serialize context using the
//! canonical IR serialization contracts rather than inventing another quantum
//! identity format.
//!
//! ## No hidden runtime dependencies
//!
//! This module does not access:
//!
//! - wall-clock time;
//! - global mutable state;
//! - environment variables;
//! - network services;
//! - filesystem state;
//! - random generators.
//!
//! Any such information must be supplied explicitly by the caller.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId, QubitRef};

// =============================================================================
// Result / error contract
// =============================================================================

/// Result type used by the feature subsystem.
pub type FeatureResult<T> = Result<T, FeatureError>;

/// Errors produced by feature schema construction, extraction, validation,
/// encoding, and normalization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FeatureError {
    /// A feature identifier or schema identifier is invalid.
    InvalidIdentifier {
        /// Identifier field.
        field: String,

        /// Human-readable reason.
        reason: String,
    },

    /// A schema is invalid.
    InvalidSchema {
        /// Human-readable reason.
        reason: String,
    },

    /// A feature definition is invalid.
    InvalidDefinition {
        /// Feature identifier.
        feature: String,

        /// Human-readable reason.
        reason: String,
    },

    /// A feature value is invalid.
    InvalidValue {
        /// Feature identifier.
        feature: String,

        /// Human-readable reason.
        reason: String,
    },

    /// A feature value is missing and the schema does not permit it.
    MissingValue {
        /// Feature identifier.
        feature: String,
    },

    /// The supplied feature vector does not match its schema.
    SchemaMismatch {
        /// Expected schema identifier.
        expected: FeatureSchemaId,

        /// Actual schema identifier.
        actual: FeatureSchemaId,
    },

    /// Feature ordering is invalid.
    OrderingViolation {
        /// Human-readable reason.
        reason: String,
    },

    /// Duplicate feature identifier.
    DuplicateFeature {
        /// Duplicate feature identifier.
        feature: String,
    },

    /// Unknown feature identifier.
    UnknownFeature {
        /// Feature identifier.
        feature: String,
    },

    /// A feature value cannot be normalized using the requested transform.
    NormalizationFailure {
        /// Feature identifier.
        feature: String,

        /// Human-readable reason.
        reason: String,
    },

    /// An extractor failed.
    ExtractionFailure {
        /// Extractor identifier.
        extractor: String,

        /// Human-readable reason.
        reason: String,
    },

    /// The requested operation is unsupported.
    Unsupported {
        /// Operation.
        operation: String,
    },
}

impl fmt::Display for FeatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field, reason } => {
                write!(f, "invalid feature identifier `{field}`: {reason}")
            }
            Self::InvalidSchema { reason } => {
                write!(f, "invalid feature schema: {reason}")
            }
            Self::InvalidDefinition { feature, reason } => {
                write!(f, "invalid feature definition `{feature}`: {reason}")
            }
            Self::InvalidValue { feature, reason } => {
                write!(f, "invalid feature value `{feature}`: {reason}")
            }
            Self::MissingValue { feature } => {
                write!(f, "missing feature value `{feature}`")
            }
            Self::SchemaMismatch { expected, actual } => {
                write!(
                    f,
                    "feature schema mismatch: expected `{expected}`, got `{actual}`"
                )
            }
            Self::OrderingViolation { reason } => {
                write!(f, "feature ordering violation: {reason}")
            }
            Self::DuplicateFeature { feature } => {
                write!(f, "duplicate feature `{feature}`")
            }
            Self::UnknownFeature { feature } => {
                write!(f, "unknown feature `{feature}`")
            }
            Self::NormalizationFailure { feature, reason } => {
                write!(f, "normalization failed for `{feature}`: {reason}")
            }
            Self::ExtractionFailure {
                extractor,
                reason,
            } => {
                write!(f, "feature extractor `{extractor}` failed: {reason}")
            }
            Self::Unsupported { operation } => {
                write!(f, "unsupported feature operation: {operation}")
            }
        }
    }
}

impl Error for FeatureError {}

// =============================================================================
// Stable identifiers
// =============================================================================

/// Stable feature identifier.
///
/// A feature identifier is semantic, not positional. Position is determined
/// by the schema.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct FeatureId(String);

impl FeatureId {
    /// Creates a validated feature identifier.
    pub fn new<S: Into<String>>(value: S) -> FeatureResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(FeatureError::InvalidIdentifier {
                field: "feature_id".to_owned(),
                reason: "identifier must not be empty".to_owned(),
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for FeatureId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Stable feature-schema identifier.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct FeatureSchemaId(String);

impl FeatureSchemaId {
    /// Creates a validated schema identifier.
    pub fn new<S: Into<String>>(value: S) -> FeatureResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(FeatureError::InvalidIdentifier {
                field: "schema_id".to_owned(),
                reason: "identifier must not be empty".to_owned(),
            });
        }

        Ok(Self(value))
    }

    /// Returns the schema identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FeatureSchemaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Version of a feature schema.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct FeatureSchemaVersion {
    /// Major schema version.
    pub major: u64,

    /// Minor schema version.
    pub minor: u64,
}

impl FeatureSchemaVersion {
    /// Creates a schema version.
    #[must_use]
    pub const fn new(major: u64, minor: u64) -> Self {
        Self { major, minor }
    }
}

impl fmt::Display for FeatureSchemaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

// =============================================================================
// Feature type
// =============================================================================

/// Semantic type of a feature.
///
/// The representation remains numeric so the result can be passed to a
/// numerical prediction model without requiring the model layer to understand
/// domain-specific Rust types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeatureType {
    /// Arbitrary finite scalar.
    Scalar,

    /// Probability in [0, 1].
    Probability,

    /// Confidence in [0, 1].
    Confidence,

    /// Non-negative scalar.
    NonNegative,

    /// Integer/count represented exactly enough for the selected f64 model
    /// interface.
    Count,

    /// Binary value represented as 0 or 1.
    Binary,

    /// Ordered categorical value represented numerically.
    Ordinal,

    /// Duration in caller-declared units.
    Duration,

    /// Ratio or normalized quantity.
    Ratio,

    /// Extension-defined semantic type.
    Custom(String),
}

impl FeatureType {
    /// Validates a raw numerical value against the semantic feature type.
    pub fn validate(&self, value: f64) -> FeatureResult<()> {
        if !value.is_finite() {
            return Err(FeatureError::InvalidValue {
                feature: "<unknown>".to_owned(),
                reason: "value must be finite".to_owned(),
            });
        }

        match self {
            Self::Probability | Self::Confidence | Self::Ratio => {
                if !(0.0..=1.0).contains(&value) {
                    return Err(FeatureError::InvalidValue {
                        feature: "<unknown>".to_owned(),
                        reason: "value must be in [0, 1]".to_owned(),
                    });
                }
            }

            Self::NonNegative | Self::Count | Self::Duration => {
                if value < 0.0 {
                    return Err(FeatureError::InvalidValue {
                        feature: "<unknown>".to_owned(),
                        reason: "value must be non-negative".to_owned(),
                    });
                }
            }

            Self::Binary => {
                if value != 0.0 && value != 1.0 {
                    return Err(FeatureError::InvalidValue {
                        feature: "<unknown>".to_owned(),
                        reason: "binary value must be exactly 0 or 1".to_owned(),
                    });
                }
            }

            Self::Scalar | Self::Ordinal | Self::Custom(_) => {}
        }

        Ok(())
    }
}

// =============================================================================
// Feature normalization
// =============================================================================

/// Numerical normalization applied after extraction.
///
/// Parameters are part of the feature schema and therefore must never be
/// silently inferred differently by different model instances.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Normalization {
    /// No transformation.
    None,

    /// Clamp to an explicitly declared interval.
    Clamp {
        /// Lower bound.
        min: f64,

        /// Upper bound.
        max: f64,
    },

    /// Linear transformation into an explicitly declared interval.
    MinMax {
        /// Source minimum.
        source_min: f64,

        /// Source maximum.
        source_max: f64,

        /// Destination minimum.
        target_min: f64,

        /// Destination maximum.
        target_max: f64,
    },

    /// Standardization using explicitly declared mean and standard deviation.
    ZScore {
        /// Mean.
        mean: f64,

        /// Standard deviation.
        standard_deviation: f64,
    },

    /// Natural-log transform of 1 + value.
    Log1p,

    /// Extension-defined normalization.
    Custom {
        /// Stable transform identifier.
        id: String,
    },
}

impl Normalization {
    /// Validates normalization parameters.
    pub fn validate(&self) -> FeatureResult<()> {
        match self {
            Self::None | Self::Log1p => Ok(()),

            Self::Clamp { min, max } => {
                validate_finite(*min, "normalization.min")?;
                validate_finite(*max, "normalization.max")?;

                if min > max {
                    return Err(FeatureError::InvalidSchema {
                        reason: "normalization clamp minimum exceeds maximum".to_owned(),
                    });
                }

                Ok(())
            }

            Self::MinMax {
                source_min,
                source_max,
                target_min,
                target_max,
            } => {
                validate_finite(*source_min, "normalization.source_min")?;
                validate_finite(*source_max, "normalization.source_max")?;
                validate_finite(*target_min, "normalization.target_min")?;
                validate_finite(*target_max, "normalization.target_max")?;

                if source_min == source_max {
                    return Err(FeatureError::InvalidSchema {
                        reason: "min-max source range must be non-zero".to_owned(),
                    });
                }

                if target_min > target_max {
                    return Err(FeatureError::InvalidSchema {
                        reason: "min-max target minimum exceeds maximum".to_owned(),
                    });
                }

                Ok(())
            }

            Self::ZScore {
                mean,
                standard_deviation,
            } => {
                validate_finite(*mean, "normalization.mean")?;
                validate_finite(
                    *standard_deviation,
                    "normalization.standard_deviation",
                )?;

                if *standard_deviation <= 0.0 {
                    return Err(FeatureError::InvalidSchema {
                        reason: "z-score standard deviation must be positive"
                            .to_owned(),
                    });
                }

                Ok(())
            }

            Self::Custom { id } => {
                if id.trim().is_empty() {
                    return Err(FeatureError::InvalidSchema {
                        reason: "custom normalization identifier must not be empty"
                            .to_owned(),
                    });
                }

                Ok(())
            }
        }
    }

    /// Applies the normalization transform.
    pub fn apply(&self, value: f64) -> FeatureResult<f64> {
        validate_finite(value, "feature")?;
        self.validate()?;

        let result = match self {
            Self::None => value,

            Self::Clamp { min, max } => value.clamp(*min, *max),

            Self::MinMax {
                source_min,
                source_max,
                target_min,
                target_max,
            } => {
                let ratio = (value - source_min) / (source_max - source_min);
                target_min + ratio * (target_max - target_min)
            }

            Self::ZScore {
                mean,
                standard_deviation,
            } => (value - mean) / standard_deviation,

            Self::Log1p => {
                if value < -1.0 {
                    return Err(FeatureError::NormalizationFailure {
                        feature: "<unknown>".to_owned(),
                        reason: "log1p requires value >= -1".to_owned(),
                    });
                }

                value.ln_1p()
            }

            Self::Custom { id } => {
                return Err(FeatureError::Unsupported {
                    operation: format!("custom normalization `{id}`"),
                });
            }
        };

        validate_finite(result, "normalized feature")
    }
}

// =============================================================================
// Missing values
// =============================================================================

/// Policy for handling missing observations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MissingValuePolicy {
    /// Reject the sample.
    Reject,

    /// Replace with an explicitly configured constant.
    Constant(f64),

    /// Replace using a caller/model supplied mean.
    Mean,

    /// Replace using a caller/model supplied median.
    Median,

    /// Use the most recently valid observation supplied by the caller.
    Previous,

    /// Encode missingness as a separate indicator while using the supplied
    /// fallback value.
    Indicator {
        /// Numeric value used for the feature.
        fallback: f64,
    },
}

impl MissingValuePolicy {
    /// Validates policy configuration.
    pub fn validate(&self) -> FeatureResult<()> {
        match self {
            Self::Reject | Self::Mean | Self::Median | Self::Previous => Ok(()),

            Self::Constant(value) | Self::Indicator { fallback: value } => {
                validate_finite(*value, "missing-value fallback")
            }
        }
    }
}

// =============================================================================
// Feature definition
// =============================================================================

/// Definition of one feature in a schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureDefinition {
    /// Stable semantic identifier.
    pub id: FeatureId,

    /// Human-readable description.
    pub description: String,

    /// Semantic type.
    pub feature_type: FeatureType,

    /// Unit or unit-like semantic description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,

    /// Whether the feature must be present.
    pub required: bool,

    /// Missing-value policy.
    pub missing_value: MissingValuePolicy,

    /// Normalization.
    pub normalization: Normalization,

    /// Optional explicit lower bound before normalization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,

    /// Optional explicit upper bound before normalization.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,

    /// Optional extractor key.
    ///
    /// This identifies the observation source semantically without coupling
    /// the schema to a concrete telemetry/history implementation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_key: Option<String>,

    /// Whether the feature may be safely used for deterministic inference.
    pub deterministic: bool,
}

impl FeatureDefinition {
    /// Validates the definition.
    pub fn validate(&self) -> FeatureResult<()> {
        if self.description.trim().is_empty() {
            return Err(FeatureError::InvalidDefinition {
                feature: self.id.to_string(),
                reason: "description must not be empty".to_owned(),
            });
        }

        self.missing_value.validate()?;
        self.normalization.validate()?;

        if let Some(minimum) = self.minimum {
            validate_finite(minimum, "feature.minimum")?;
        }

        if let Some(maximum) = self.maximum {
            validate_finite(maximum, "feature.maximum")?;
        }

        if let (Some(minimum), Some(maximum)) = (self.minimum, self.maximum) {
            if minimum > maximum {
                return Err(FeatureError::InvalidDefinition {
                    feature: self.id.to_string(),
                    reason: "minimum exceeds maximum".to_owned(),
                });
            }
        }

        if let Some(source_key) = &self.source_key {
            if source_key.trim().is_empty() {
                return Err(FeatureError::InvalidDefinition {
                    feature: self.id.to_string(),
                    reason: "source key must not be empty".to_owned(),
                });
            }
        }

        Ok(())
    }

    /// Validates and normalizes a present value.
    pub fn process(&self, value: f64) -> FeatureResult<f64> {
        validate_finite_for_feature(&self.id, value)?;

        self.feature_type
            .validate(value)
            .map_err(|_| FeatureError::InvalidValue {
                feature: self.id.to_string(),
                reason: "value violates feature semantic type".to_owned(),
            })?;

        if let Some(minimum) = self.minimum {
            if value < minimum {
                return Err(FeatureError::InvalidValue {
                    feature: self.id.to_string(),
                    reason: format!("value is below declared minimum {minimum}"),
                });
            }
        }

        if let Some(maximum) = self.maximum {
            if value > maximum {
                return Err(FeatureError::InvalidValue {
                    feature: self.id.to_string(),
                    reason: format!("value exceeds declared maximum {maximum}"),
                });
            }
        }

        self.normalization
            .apply(value)
            .map_err(|error| match error {
                FeatureError::NormalizationFailure { .. } => {
                    FeatureError::NormalizationFailure {
                        feature: self.id.to_string(),
                        reason: "normalization failed".to_owned(),
                    }
                }
                other => other,
            })
    }
}

// =============================================================================
// Feature schema
// =============================================================================

/// Complete ordered feature schema.
///
/// Ordering is part of the contract because numerical model input is positional.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureSchema {
    /// Stable schema identity.
    pub id: FeatureSchemaId,

    /// Schema version.
    pub version: FeatureSchemaVersion,

    /// Human-readable description.
    pub description: String,

    /// Ordered feature definitions.
    pub features: Vec<FeatureDefinition>,

    /// Whether the schema itself is deterministic.
    pub deterministic: bool,
}

impl FeatureSchema {
    /// Creates and validates a schema.
    pub fn new(
        id: FeatureSchemaId,
        version: FeatureSchemaVersion,
        description: String,
        features: Vec<FeatureDefinition>,
        deterministic: bool,
    ) -> FeatureResult<Self> {
        let schema = Self {
            id,
            version,
            description,
            features,
            deterministic,
        };

        schema.validate()?;
        Ok(schema)
    }

    /// Validates the complete schema.
    pub fn validate(&self) -> FeatureResult<()> {
        if self.description.trim().is_empty() {
            return Err(FeatureError::InvalidSchema {
                reason: "description must not be empty".to_owned(),
            });
        }

        let mut ids = BTreeSet::new();

        for feature in &self.features {
            feature.validate()?;

            if !ids.insert(feature.id.clone()) {
                return Err(FeatureError::DuplicateFeature {
                    feature: feature.id.to_string(),
                });
            }
        }

        Ok(())
    }

    /// Returns the number of declared features.
    #[must_use]
    pub fn len(&self) -> usize {
        self.features.len()
    }

    /// Returns whether the schema contains no features.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.features.is_empty()
    }

    /// Finds a feature definition by identifier.
    #[must_use]
    pub fn definition(&self, id: &FeatureId) -> Option<&FeatureDefinition> {
        self.features.iter().find(|feature| feature.id == *id)
    }

    /// Returns the positional index of a feature.
    #[must_use]
    pub fn index_of(&self, id: &FeatureId) -> Option<usize> {
        self.features.iter().position(|feature| feature.id == *id)
    }
}

// =============================================================================
// Feature value
// =============================================================================

/// A raw feature observation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureValue {
    /// Feature identifier.
    pub feature: FeatureId,

    /// Numerical value.
    pub value: f64,

    /// Whether this value was imputed.
    pub imputed: bool,
}

impl FeatureValue {
    /// Creates a raw observed value.
    pub fn observed(feature: FeatureId, value: f64) -> FeatureResult<Self> {
        validate_finite_for_feature(&feature, value)?;

        Ok(Self {
            feature,
            value,
            imputed: false,
        })
    }

    /// Creates an explicitly imputed value.
    pub fn imputed(feature: FeatureId, value: f64) -> FeatureResult<Self> {
        validate_finite_for_feature(&feature, value)?;

        Ok(Self {
            feature,
            value,
            imputed: true,
        })
    }
}

// =============================================================================
// Feature vector
// =============================================================================

/// Ordered numerical feature vector.
///
/// This is the portable representation intended to cross the boundary into
/// `learning/model.rs`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureVector {
    /// Feature schema identity.
    pub schema_id: FeatureSchemaId,

    /// Feature schema version.
    pub schema_version: FeatureSchemaVersion,

    /// Ordered numerical values.
    pub values: Vec<f64>,

    /// Indicates whether any value was imputed.
    pub contains_imputed_values: bool,
}

impl FeatureVector {
    /// Creates a feature vector after validating it against a schema.
    pub fn new(
        schema: &FeatureSchema,
        values: Vec<f64>,
        contains_imputed_values: bool,
    ) -> FeatureResult<Self> {
        schema.validate()?;

        if values.len() != schema.features.len() {
            return Err(FeatureError::OrderingViolation {
                reason: format!(
                    "expected {} values for schema, received {}",
                    schema.features.len(),
                    values.len()
                ),
            });
        }

        for (definition, value) in schema.features.iter().zip(values.iter()) {
            validate_finite_for_feature(&definition.id, *value)?;
        }

        Ok(Self {
            schema_id: schema.id.clone(),
            schema_version: schema.version,
            values,
            contains_imputed_values,
        })
    }

    /// Returns the number of numerical features.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether the vector is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns an immutable view of the numerical values.
    #[must_use]
    pub fn as_slice(&self) -> &[f64] {
        &self.values
    }

    /// Returns a mutable view for controlled downstream processing.
    ///
    /// Callers must preserve the schema contract after mutation.
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [f64] {
        &mut self.values
    }

    /// Validates the vector against a schema.
    pub fn validate_against(&self, schema: &FeatureSchema) -> FeatureResult<()> {
        if self.schema_id != schema.id {
            return Err(FeatureError::SchemaMismatch {
                expected: schema.id.clone(),
                actual: self.schema_id.clone(),
            });
        }

        if self.schema_version != schema.version {
            return Err(FeatureError::InvalidSchema {
                reason: format!(
                    "schema version mismatch: expected {}, got {}",
                    schema.version, self.schema_version
                ),
            });
        }

        if self.values.len() != schema.features.len() {
            return Err(FeatureError::OrderingViolation {
                reason: format!(
                    "schema contains {} features but vector contains {} values",
                    schema.features.len(),
                    self.values.len()
                ),
            });
        }

        for (definition, value) in schema.features.iter().zip(self.values.iter()) {
            validate_finite_for_feature(&definition.id, *value)?;
        }

        Ok(())
    }
}

// =============================================================================
// Feature context
// =============================================================================

/// Scope of observations represented by a feature set.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeatureScope {
    /// Entire execution.
    Execution,

    /// Whole backend/device.
    Device,

    /// Logical quantum resource.
    LogicalQubit(QubitId),

    /// Physical quantum resource.
    PhysicalQubit(PhysicalQubitId),

    /// Explicit logical/physical reference.
    Qubit(QubitRef),

    /// A larger region of resources.
    Region(String),

    /// A user/extension-defined scope.
    Custom(String),
}

impl FeatureScope {
    /// Returns a stable semantic label.
    #[must_use]
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Execution => "execution",
            Self::Device => "device",
            Self::LogicalQubit(_) => "logical_qubit",
            Self::PhysicalQubit(_) => "physical_qubit",
            Self::Qubit(_) => "qubit",
            Self::Region(_) => "region",
            Self::Custom(_) => "custom",
        }
    }
}

/// Explicit context supplied to feature extraction.
///
/// The context is deliberately not coupled to telemetry or history
/// implementations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureContext {
    /// Scope represented by the feature sample.
    pub scope: FeatureScope,

    /// Stable execution identifier, when available.
    pub execution_id: Option<String>,

    /// Stable backend/device identifier, when available.
    pub resource_id: Option<String>,

    /// Additional deterministic metadata.
    pub attributes: BTreeMap<String, String>,
}

impl FeatureContext {
    /// Creates execution-scoped context.
    #[must_use]
    pub fn execution() -> Self {
        Self {
            scope: FeatureScope::Execution,
            execution_id: None,
            resource_id: None,
            attributes: BTreeMap::new(),
        }
    }

    /// Adds an execution identifier.
    pub fn with_execution_id<S: Into<String>>(
        mut self,
        id: S,
    ) -> FeatureResult<Self> {
        let id = id.into();

        if id.trim().is_empty() {
            return Err(FeatureError::InvalidIdentifier {
                field: "execution_id".to_owned(),
                reason: "identifier must not be empty".to_owned(),
            });
        }

        self.execution_id = Some(id);
        Ok(self)
    }

    /// Adds a resource identifier.
    pub fn with_resource_id<S: Into<String>>(
        mut self,
        id: S,
    ) -> FeatureResult<Self> {
        let id = id.into();

        if id.trim().is_empty() {
            return Err(FeatureError::InvalidIdentifier {
                field: "resource_id".to_owned(),
                reason: "identifier must not be empty".to_owned(),
            });
        }

        self.resource_id = Some(id);
        Ok(self)
    }

    /// Adds deterministic metadata.
    pub fn with_attribute<S1: Into<String>, S2: Into<String>>(
        mut self,
        key: S1,
        value: S2,
    ) -> FeatureResult<Self> {
        let key = key.into();

        if key.trim().is_empty() {
            return Err(FeatureError::InvalidIdentifier {
                field: "attribute_key".to_owned(),
                reason: "attribute key must not be empty".to_owned(),
            });
        }

        self.attributes.insert(key, value.into());
        Ok(self)
    }
}

// =============================================================================
// Feature provenance
// =============================================================================

/// Provenance for a feature vector.
///
/// This does not contain mutable runtime state. It records what feature
/// extraction contract produced the vector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeatureProvenance {
    /// Extractor identifier.
    pub extractor_id: String,

    /// Extractor version.
    pub extractor_version: String,

    /// Feature schema.
    pub schema_id: FeatureSchemaId,

    /// Feature schema version.
    pub schema_version: FeatureSchemaVersion,

    /// Whether extraction was deterministic.
    pub deterministic: bool,

    /// Whether one or more values were imputed.
    pub contains_imputed_values: bool,

    /// Optional source identifiers.
    #[serde(default)]
    pub source_ids: Vec<String>,
}

impl FeatureProvenance {
    /// Creates validated provenance.
    pub fn new<S1: Into<String>, S2: Into<String>>(
        extractor_id: S1,
        extractor_version: S2,
        schema: &FeatureSchema,
        deterministic: bool,
        contains_imputed_values: bool,
    ) -> FeatureResult<Self> {
        let extractor_id = extractor_id.into();
        let extractor_version = extractor_version.into();

        if extractor_id.trim().is_empty() {
            return Err(FeatureError::InvalidIdentifier {
                field: "extractor_id".to_owned(),
                reason: "identifier must not be empty".to_owned(),
            });
        }

        if extractor_version.trim().is_empty() {
            return Err(FeatureError::InvalidIdentifier {
                field: "extractor_version".to_owned(),
                reason: "version must not be empty".to_owned(),
            });
        }

        Ok(Self {
            extractor_id,
            extractor_version,
            schema_id: schema.id.clone(),
            schema_version: schema.version,
            deterministic,
            contains_imputed_values,
            source_ids: Vec::new(),
        })
    }

    /// Adds a source identifier.
    pub fn add_source_id<S: Into<String>>(
        &mut self,
        source_id: S,
    ) -> FeatureResult<()> {
        let source_id = source_id.into();

        if source_id.trim().is_empty() {
            return Err(FeatureError::InvalidIdentifier {
                field: "source_id".to_owned(),
                reason: "source identifier must not be empty".to_owned(),
            });
        }

        self.source_ids.push(source_id);
        Ok(())
    }
}

// =============================================================================
// Feature set
// =============================================================================

/// Complete feature sample ready for model integration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureSet {
    /// Numerical feature vector.
    pub vector: FeatureVector,

    /// Extraction provenance.
    pub provenance: FeatureProvenance,
}

impl FeatureSet {
    /// Creates a feature set after validating vector/schema/provenance
    /// consistency.
    pub fn new(
        vector: FeatureVector,
        provenance: FeatureProvenance,
    ) -> FeatureResult<Self> {
        if vector.schema_id != provenance.schema_id {
            return Err(FeatureError::SchemaMismatch {
                expected: provenance.schema_id.clone(),
                actual: vector.schema_id.clone(),
            });
        }

        if vector.schema_version != provenance.schema_version {
            return Err(FeatureError::InvalidSchema {
                reason: "feature vector and provenance schema versions differ"
                    .to_owned(),
            });
        }

        if vector.contains_imputed_values != provenance.contains_imputed_values {
            return Err(FeatureError::InvalidSchema {
                reason: "feature vector and provenance disagree about imputation"
                    .to_owned(),
            });
        }

        Ok(Self {
            vector,
            provenance,
        })
    }

    /// Returns the numerical model input.
    #[must_use]
    pub fn values(&self) -> &[f64] {
        self.vector.as_slice()
    }
}

// =============================================================================
// Feature extraction trait
// =============================================================================

/// Provider-independent feature extractor.
///
/// An extractor converts an explicitly supplied observation into a feature
/// set. It must not silently obtain state from global resources.
///
/// `Observation` is intentionally generic so telemetry, history, simulation,
/// replay, and hardware integrations can each supply their own observation
/// type.
pub trait FeatureExtractor<Observation>: Send + Sync {
    /// Returns the stable extractor identifier.
    fn id(&self) -> &str;

    /// Returns the extractor version.
    fn version(&self) -> &str;

    /// Returns the feature schema produced by this extractor.
    fn schema(&self) -> &FeatureSchema;

    /// Extracts a feature set from explicit input.
    fn extract(
        &self,
        observation: &Observation,
        context: &FeatureContext,
    ) -> FeatureResult<FeatureSet>;
}

// =============================================================================
// Generic feature assembler
// =============================================================================

/// Deterministic assembler for feature values.
///
/// This is useful when multiple independent feature providers contribute to
/// one schema. The assembler does not perform model inference.
#[derive(Debug, Clone)]
pub struct FeatureAssembler {
    schema: FeatureSchema,
    values: BTreeMap<FeatureId, FeatureValue>,
}

impl FeatureAssembler {
    /// Creates an empty assembler for a validated schema.
    pub fn new(schema: FeatureSchema) -> FeatureResult<Self> {
        schema.validate()?;

        Ok(Self {
            schema,
            values: BTreeMap::new(),
        })
    }

    /// Returns the schema.
    #[must_use]
    pub fn schema(&self) -> &FeatureSchema {
        &self.schema
    }

    /// Inserts an observed feature value.
    ///
    /// Duplicate insertion is rejected rather than silently replacing an
    /// existing value.
    pub fn insert(
        &mut self,
        value: FeatureValue,
    ) -> FeatureResult<()> {
        if self.values.contains_key(&value.feature) {
            return Err(FeatureError::DuplicateFeature {
                feature: value.feature.to_string(),
            });
        }

        let definition = self
            .schema
            .definition(&value.feature)
            .ok_or_else(|| FeatureError::UnknownFeature {
                feature: value.feature.to_string(),
            })?;

        definition.process(value.value)?;

        self.values.insert(value.feature.clone(), value);

        Ok(())
    }

    /// Inserts a missing feature according to its schema policy.
    pub fn insert_missing(
        &mut self,
        feature: &FeatureId,
        replacement: Option<f64>,
    ) -> FeatureResult<()> {
        if self.values.contains_key(feature) {
            return Err(FeatureError::DuplicateFeature {
                feature: feature.to_string(),
            });
        }

        let definition = self
            .schema
            .definition(feature)
            .ok_or_else(|| FeatureError::UnknownFeature {
                feature: feature.to_string(),
            })?;

        let value = match &definition.missing_value {
            MissingValuePolicy::Reject => {
                return Err(FeatureError::MissingValue {
                    feature: feature.to_string(),
                });
            }

            MissingValuePolicy::Constant(value) => *value,

            MissingValuePolicy::Mean | MissingValuePolicy::Median | MissingValuePolicy::Previous => {
                replacement.ok_or_else(|| FeatureError::MissingValue {
                    feature: feature.to_string(),
                })?
            }

            MissingValuePolicy::Indicator { fallback } => {
                replacement.unwrap_or(*fallback)
            }
        };

        let processed = definition.process(value)?;

        self.values.insert(
            feature.clone(),
            FeatureValue {
                feature: feature.clone(),
                value: processed,
                imputed: true,
            },
        );

        Ok(())
    }

    /// Finalizes the assembler into an ordered feature vector.
    pub fn build(self) -> FeatureResult<FeatureVector> {
        let mut values = Vec::with_capacity(self.schema.features.len());
        let mut contains_imputed_values = false;

        for definition in &self.schema.features {
            match self.values.get(&definition.id) {
                Some(value) => {
                    values.push(value.value);
                    contains_imputed_values |= value.imputed;
                }

                None if definition.required => {
                    return Err(FeatureError::MissingValue {
                        feature: definition.id.to_string(),
                    });
                }

                None => {
                    return Err(FeatureError::MissingValue {
                        feature: definition.id.to_string(),
                    });
                }
            }
        }

        FeatureVector::new(
            &self.schema,
            values,
            contains_imputed_values,
        )
    }
}

// =============================================================================
// Numeric feature helpers
// =============================================================================

/// Validates a finite number.
fn validate_finite(value: f64, field: &str) -> FeatureResult<()> {
    if !value.is_finite() {
        return Err(FeatureError::InvalidValue {
            feature: field.to_owned(),
            reason: "value must be finite".to_owned(),
        });
    }

    Ok(())
}

/// Validates a finite feature value with the feature identifier attached.
fn validate_finite_for_feature(
    feature: &FeatureId,
    value: f64,
) -> FeatureResult<()> {
    if !value.is_finite() {
        return Err(FeatureError::InvalidValue {
            feature: feature.to_string(),
            reason: "value must be finite".to_owned(),
        });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn schema() -> FeatureSchema {
        FeatureSchema::new(
            FeatureSchemaId::new("resilience.execution.v1").unwrap(),
            FeatureSchemaVersion::new(1, 0),
            "Execution resilience features".to_owned(),
            vec![
                FeatureDefinition {
                    id: FeatureId::new("failure_probability").unwrap(),
                    description: "Observed failure probability".to_owned(),
                    feature_type: FeatureType::Probability,
                    unit: None,
                    required: true,
                    missing_value: MissingValuePolicy::Reject,
                    normalization: Normalization::None,
                    minimum: None,
                    maximum: None,
                    source_key: Some("execution.failure_probability".to_owned()),
                    deterministic: true,
                },
                FeatureDefinition {
                    id: FeatureId::new("latency").unwrap(),
                    description: "Observed execution latency".to_owned(),
                    feature_type: FeatureType::NonNegative,
                    unit: Some("seconds".to_owned()),
                    required: true,
                    missing_value: MissingValuePolicy::Constant(0.0),
                    normalization: Normalization::Log1p,
                    minimum: Some(0.0),
                    maximum: None,
                    source_key: Some("execution.latency".to_owned()),
                    deterministic: true,
                },
            ],
            true,
        )
        .unwrap()
    }

    #[test]
    fn schema_is_valid() {
        let schema = schema();

        assert_eq!(schema.len(), 2);
        assert!(!schema.is_empty());
    }

    #[test]
    fn assembler_preserves_schema_order() {
        let schema = schema();
        let mut assembler = FeatureAssembler::new(schema.clone()).unwrap();

        assembler
            .insert(
                FeatureValue::observed(
                    FeatureId::new("latency").unwrap(),
                    9.0,
                )
                .unwrap(),
            )
            .unwrap();

        assembler
            .insert(
                FeatureValue::observed(
                    FeatureId::new("failure_probability").unwrap(),
                    0.25,
                )
                .unwrap(),
            )
            .unwrap();

        let vector = assembler.build().unwrap();

        assert_eq!(vector.len(), 2);

        // failure_probability is first in the schema even though latency
        // was inserted first.
        assert_eq!(vector.values[0], 0.25);
        assert!((vector.values[1] - 9.0_f64.ln_1p()).abs() < 1e-12);
    }

    #[test]
    fn probability_range_is_validated() {
        let schema = schema();
        let mut assembler = FeatureAssembler::new(schema).unwrap();

        let result = assembler.insert(
            FeatureValue::observed(
                FeatureId::new("failure_probability").unwrap(),
                1.5,
            )
            .unwrap(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn non_finite_values_are_rejected() {
        let schema = schema();
        let mut assembler = FeatureAssembler::new(schema).unwrap();

        let result = assembler.insert(
            FeatureValue::observed(
                FeatureId::new("failure_probability").unwrap(),
                f64::NAN,
            )
            .unwrap(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_features_are_rejected() {
        let schema = schema();
        let mut assembler = FeatureAssembler::new(schema).unwrap();

        let feature = FeatureId::new("failure_probability").unwrap();

        assembler
            .insert(FeatureValue::observed(feature.clone(), 0.1).unwrap())
            .unwrap();

        let result =
            assembler.insert(FeatureValue::observed(feature, 0.2).unwrap());

        assert!(matches!(
            result,
            Err(FeatureError::DuplicateFeature { .. })
        ));
    }

    #[test]
    fn missing_constant_can_be_applied() {
        let schema = schema();
        let mut assembler = FeatureAssembler::new(schema).unwrap();

        assembler
            .insert(
                FeatureValue::observed(
                    FeatureId::new("failure_probability").unwrap(),
                    0.2,
                )
                .unwrap(),
            )
            .unwrap();

        assembler
            .insert_missing(&FeatureId::new("latency").unwrap(), None)
            .unwrap();

        let vector = assembler.build().unwrap();

        assert!(vector.contains_imputed_values);
        assert_eq!(vector.len(), 2);
    }

    #[test]
    fn normalization_rejects_invalid_log_domain() {
        let result = Normalization::Log1p.apply(-2.0);

        assert!(result.is_err());
    }

    #[test]
    fn min_max_is_deterministic() {
        let normalization = Normalization::MinMax {
            source_min: 0.0,
            source_max: 10.0,
            target_min: 0.0,
            target_max: 1.0,
        };

        let first = normalization.apply(2.5).unwrap();
        let second = normalization.apply(2.5).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn canonical_qubit_context_preserves_logical_physical_separation() {
        let logical = QubitId::new(7);
        let physical = PhysicalQubitId::new(11);

        let logical_scope = FeatureScope::LogicalQubit(logical);
        let physical_scope = FeatureScope::PhysicalQubit(physical);

        assert_eq!(logical_scope.kind(), "logical_qubit");
        assert_eq!(physical_scope.kind(), "physical_qubit");
    }

    #[test]
    fn qubit_reference_context_is_supported() {
        let logical = QubitId::new(3);
        let scope = FeatureScope::Qubit(QubitRef::Logical(logical));

        assert_eq!(scope.kind(), "qubit");
    }

    #[test]
    fn feature_provenance_matches_schema() {
        let schema = schema();

        let provenance = FeatureProvenance::new(
            "execution-features",
            "1.0.0",
            &schema,
            true,
            false,
        )
        .unwrap();

        let vector = FeatureVector::new(
            &schema,
            vec![0.2, 1.0],
            false,
        )
        .unwrap();

        let set = FeatureSet::new(vector, provenance).unwrap();

        assert_eq!(set.vector.schema_id, schema.id);
        assert_eq!(set.vector.schema_version, schema.version);
    }
}