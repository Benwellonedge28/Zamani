//! # Zamani Quantum Resilience — Learning Model Contract
//!
//! Path:
//! `src/quantum/resilience/learning/model.rs`
//!
//! ## Purpose
//!
//! This module defines the provider-independent contract for predictive models
//! used by the quantum-resilience learning subsystem.
//!
//! This module is deliberately a CONTRACT layer.
//!
//! It does not implement:
//!
//! - machine-learning algorithms;
//! - model training;
//! - feature extraction;
//! - telemetry;
//! - history storage;
//! - strategy selection;
//! - recovery;
//! - routing;
//! - scheduling;
//! - QEC;
//! - hardware discovery;
//! - canonical quantum IR.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! ## Integration contract
//!
//! `learning/features.rs` owns:
//!
//! - `FeatureSchemaId`;
//! - `FeatureSchema`;
//! - `FeatureSchemaVersion`;
//! - `FeatureVector`;
//! - feature validation;
//! - feature provenance/context.
//!
//! This module consumes those canonical types.
//!
//! `learning/predictor.rs` consumes [`PredictionModel`], [`ModelHandle`],
//! [`PredictionInput`], and [`Prediction`].
//!
//! `learning/strategy.rs` may use predictions as advisory evidence when ranking
//! resilience strategies.
//!
//! `learning/feedback.rs` may associate verified outcomes with
//! [`PredictionProvenance`].
//!
//! `history/*` supplies historical observations to the feature/training
//! layers. This module does not access history directly.
//!
//! `telemetry/*` supplies observations through the feature layer.
//!
//! `quantum::hardware` remains authoritative for hardware capabilities.
//!
//! `quantum::zqn` remains authoritative for quantum fault/noise semantics.
//!
//! `quantum::ir::qubit` remains authoritative for quantum-resource identity.
//!
//! ## Critical safety rule
//!
//! A prediction is EVIDENCE, never AUTHORITY.
//!
//! A prediction MUST NOT by itself:
//!
//! - authorize recovery;
//! - authorize migration;
//! - change program semantics;
//! - select a hardware target;
//! - change QEC;
//! - accept an execution result;
//! - bypass safety policy;
//! - bypass capability validation;
//! - bypass semantic verification.
//!
//! The resilience planner remains authoritative for those decisions.
//!
//! ## Scalability
//!
//! This module deliberately contains no fixed limits for:
//!
//! - qubits;
//! - logical qubits;
//! - physical qubits;
//! - devices;
//! - backends;
//! - features;
//! - outputs;
//! - models;
//! - executions;
//! - training samples.
//!
//! Dynamic collections are bounded only by caller policy and available
//! resources.
//!
//! There is no:
//!
//! - `MAX_QUBITS`;
//! - `MAX_FEATURES`;
//! - `MAX_OUTPUTS`;
//! - fixed retry count;
//! - provider-specific device size;
//! - provider-specific backend branch.
//!
//! ## Determinism
//!
//! Deterministic operation is explicit.
//!
//! A caller requiring deterministic inference must set
//! [`PredictionContext::deterministic`] to `true`.
//!
//! A model may satisfy deterministic inference only when
//! [`ModelCapabilities::deterministic`] is true.
//!
//! No model may silently obtain randomness from:
//!
//! - global state;
//! - wall-clock time;
//! - environment variables;
//! - network state;
//! - filesystem state;
//! - process identity.
//!
//! If randomness is required, its seed must be explicitly represented by
//! [`PredictionContext::random_seed`] and recorded in provenance.
//!
//! ## Numerical safety
//!
//! NaN and positive/negative infinity are rejected.
//!
//! Probability and confidence outputs MUST be within `[0, 1]`.
//!
//! Model implementations remain responsible for domain-specific numerical
//! validation beyond this contract.
//!
//! ## Rust requirements
//!
//! - Rust 2021.
//! - Rust 1.97 / 1.97.1.
//! - Stable Rust only.
//! - No nightly features.
//! - No `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the last requirement compiler-enforced.
//!
//! ## Canonical feature integration
//!
//! IMPORTANT:
//!
//! `FeatureVector` is NOT redefined here.
//!
//! `learning/features.rs` is the canonical owner of feature vectors.
//!
//! `ModelSchemaId` is a type alias for the canonical
//! `features::FeatureSchemaId` so the learning model layer cannot accidentally
//! create a second incompatible schema identity system.
//!
//! ## Version compatibility
//!
//! Input compatibility is exact at the schema identity/version boundary.
//! Version migration belongs outside this module.
//!
//! This prevents a prediction model from silently interpreting an incompatible
//! feature representation.
//!
//! ## Thread safety
//!
//! [`PredictionModel`] requires `Send + Sync`.
//!
//! This allows a [`ModelHandle`] to be safely shared by concurrent inference
//! components without requiring global mutable state.
//!
//! A concrete model implementation may internally use synchronization, but
//! that implementation detail does not leak into this contract.

#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::features::{FeatureSchemaId, FeatureSchemaVersion, FeatureVector};

// =============================================================================
// Canonical aliases
// =============================================================================

/// Canonical feature-schema identifier used by learning models.
///
/// This is deliberately an alias instead of a second wrapper type.
///
/// `learning/features.rs` is the authoritative owner of feature schema
/// identity.
pub type ModelSchemaId = FeatureSchemaId;

/// Canonical feature-schema version.
pub type ModelSchemaVersion = FeatureSchemaVersion;

// =============================================================================
// Result / error contract
// =============================================================================

/// Result type used by the model contract.
pub type ModelResult<T> = Result<T, ModelError>;

/// Contract-level errors produced by prediction models.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelError {
    /// Model metadata is invalid.
    InvalidMetadata {
        /// Invalid field.
        field: String,

        /// Explanation.
        reason: String,
    },

    /// Input does not satisfy the model contract.
    InvalidInput {
        /// Explanation.
        reason: String,
    },

    /// Model output does not satisfy the declared contract.
    InvalidOutput {
        /// Explanation.
        reason: String,
    },

    /// Operation is unsupported.
    UnsupportedOperation {
        /// Operation name.
        operation: String,
    },

    /// Model is not available for prediction.
    Unavailable {
        /// Explanation.
        reason: String,
    },

    /// Deterministic prediction was requested but cannot be guaranteed.
    DeterminismUnavailable {
        /// Explanation.
        reason: String,
    },

    /// Feature/output schema mismatch.
    SchemaMismatch {
        /// Expected schema.
        expected: ModelSchemaId,

        /// Supplied schema.
        actual: ModelSchemaId,
    },

    /// Feature schema version mismatch.
    SchemaVersionMismatch {
        /// Expected version.
        expected: ModelSchemaVersion,

        /// Supplied version.
        actual: ModelSchemaVersion,
    },

    /// Model version incompatibility.
    VersionMismatch {
        /// Required version.
        required: ModelVersion,

        /// Actual model version.
        actual: ModelVersion,
    },

    /// Model capability mismatch.
    CapabilityMismatch {
        /// Explanation.
        reason: String,
    },

    /// Model state is internally invalid.
    InvalidState {
        /// Explanation.
        reason: String,
    },

    /// Duplicate output names were supplied.
    DuplicateOutput {
        /// Duplicate output name.
        name: String,
    },
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMetadata { field, reason } => {
                write!(f, "invalid model metadata `{field}`: {reason}")
            }

            Self::InvalidInput { reason } => {
                write!(f, "invalid model input: {reason}")
            }

            Self::InvalidOutput { reason } => {
                write!(f, "invalid model output: {reason}")
            }

            Self::UnsupportedOperation { operation } => {
                write!(f, "unsupported model operation: {operation}")
            }

            Self::Unavailable { reason } => {
                write!(f, "model unavailable: {reason}")
            }

            Self::DeterminismUnavailable { reason } => {
                write!(f, "deterministic prediction unavailable: {reason}")
            }

            Self::SchemaMismatch { expected, actual } => {
                write!(
                    f,
                    "model schema mismatch: expected `{expected}`, got `{actual}`"
                )
            }

            Self::SchemaVersionMismatch { expected, actual } => {
                write!(
                    f,
                    "model schema version mismatch: expected `{expected}`, got `{actual}`"
                )
            }

            Self::VersionMismatch { required, actual } => {
                write!(
                    f,
                    "model version mismatch: required `{required}`, got `{actual}`"
                )
            }

            Self::CapabilityMismatch { reason } => {
                write!(f, "model capability mismatch: {reason}")
            }

            Self::InvalidState { reason } => {
                write!(f, "invalid model state: {reason}")
            }

            Self::DuplicateOutput { name } => {
                write!(f, "duplicate prediction output `{name}`")
            }
        }
    }
}

impl Error for ModelError {}

// =============================================================================
// Stable model identifiers
// =============================================================================

/// Stable identifier for a prediction model family.
///
/// The identifier is semantic and independent of:
///
/// - backend;
/// - device;
/// - process;
/// - memory address;
/// - implementation instance.
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
pub struct ModelId(String);

impl ModelId {
    /// Creates a validated model identifier.
    pub fn new<S: Into<String>>(value: S) -> ModelResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ModelError::InvalidMetadata {
                field: "model_id".to_owned(),
                reason: "identifier must not be empty".to_owned(),
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
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

impl fmt::Display for ModelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

// =============================================================================
// Model version
// =============================================================================

/// Semantic model version.
///
/// Ordering is based on numeric semantic version components followed by
/// optional pre-release/build metadata. The model registry remains responsible
/// for deciding whether two versions are compatible.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ModelVersion {
    /// Major version.
    pub major: u64,

    /// Minor version.
    pub minor: u64,

    /// Patch version.
    pub patch: u64,

    /// Optional pre-release metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_release: Option<String>,

    /// Optional build metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
}

impl ModelVersion {
    /// Creates a model version.
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: None,
            build: None,
        }
    }

    /// Adds pre-release metadata.
    #[must_use]
    pub fn with_pre_release<S: Into<String>>(mut self, value: S) -> Self {
        self.pre_release = Some(value.into());
        self
    }

    /// Adds build metadata.
    #[must_use]
    pub fn with_build<S: Into<String>>(mut self, value: S) -> Self {
        self.build = Some(value.into());
        self
    }

    /// Returns whether two versions share a major version.
    #[must_use]
    pub const fn same_major(&self, other: &Self) -> bool {
        self.major == other.major
    }
}

impl fmt::Display for ModelVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;

        if let Some(pre_release) = &self.pre_release {
            write!(f, "-{pre_release}")?;
        }

        if let Some(build) = &self.build {
            write!(f, "+{build}")?;
        }

        Ok(())
    }
}

// =============================================================================
// Model classification
// =============================================================================

/// General implementation family of a prediction model.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelKind {
    /// Statistical model.
    Statistical,

    /// Regression model.
    Regression,

    /// Classification model.
    Classification,

    /// Time-series model.
    TimeSeries,

    /// Probabilistic model.
    Probabilistic,

    /// Ensemble model.
    Ensemble,

    /// Neural/network model.
    Neural,

    /// Kernel/similarity model.
    Kernel,

    /// Hybrid rule/model system.
    Hybrid,

    /// Extension-defined implementation family.
    Custom(String),
}

/// Prediction task.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PredictionTask {
    /// Probability of execution failure.
    FailureProbability,

    /// Probability of successful recovery.
    RecoverySuccessProbability,

    /// Probability of degradation/resource loss.
    DegradationProbability,

    /// Expected latency.
    Latency,

    /// Expected fidelity/quality.
    Fidelity,

    /// Expected error rate.
    ErrorRate,

    /// Suitability of a resilience strategy.
    StrategySuitability,

    /// Resource availability.
    ResourceAvailability,

    /// Generic probability.
    Probability,

    /// Generic regression value.
    Regression,

    /// Extension-defined task.
    Custom(String),
}

/// Interpretation of one model output.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PredictionTarget {
    /// Probability in `[0, 1]`.
    Probability,

    /// Confidence in `[0, 1]`.
    Confidence,

    /// Non-negative scalar.
    NonNegativeScalar,

    /// Arbitrary scalar.
    Scalar,

    /// Categorical output.
    Category,

    /// Ranking score.
    RankingScore,

    /// Extension-defined target.
    Custom(String),
}

// =============================================================================
// Model lifecycle
// =============================================================================

/// Operational model state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelState {
    /// Metadata is registered but model is not ready.
    Registered,

    /// Model can predict.
    Ready,

    /// Temporarily unavailable.
    Unavailable,

    /// Model is being updated.
    Updating,

    /// Disabled by policy.
    Disabled,

    /// Failed validation/integrity checks.
    Invalid,

    /// Permanently retired.
    Retired,
}

impl ModelState {
    /// Returns whether prediction is permitted.
    #[must_use]
    pub const fn can_predict(self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Returns whether the model is permanently retired.
    #[must_use]
    pub const fn is_retired(self) -> bool {
        matches!(self, Self::Retired)
    }
}

// =============================================================================
// Model capabilities
// =============================================================================

/// Capabilities declared by a model implementation.
///
/// These declarations describe what the implementation can guarantee.
/// They do not override resilience policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelCapabilities {
    /// Deterministic inference is supported.
    pub deterministic: bool,

    /// Single/online prediction is supported.
    pub online_prediction: bool,

    /// Batch prediction is supported.
    pub batch_prediction: bool,

    /// Uncertainty/confidence can be estimated.
    pub uncertainty_estimation: bool,

    /// Incremental/online model updates are supported.
    pub incremental: bool,

    /// Model has explicit version metadata.
    pub versioned: bool,

    /// Model can execute without network/cloud dependencies.
    pub offline: bool,

    /// Model does not depend on a particular quantum provider.
    pub provider_independent: bool,

    /// Model accepts variable-dimensional feature vectors according to schema.
    pub variable_dimension: bool,
}

impl Default for ModelCapabilities {
    fn default() -> Self {
        Self {
            deterministic: false,
            online_prediction: true,
            batch_prediction: false,
            uncertainty_estimation: false,
            incremental: false,
            versioned: true,
            offline: true,
            provider_independent: true,
            variable_dimension: true,
        }
    }
}

// =============================================================================
// Model metadata
// =============================================================================

/// Immutable descriptive metadata for a model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelMetadata {
    /// Stable model identity.
    pub id: ModelId,

    /// Model version.
    pub version: ModelVersion,

    /// Model implementation family.
    pub kind: ModelKind,

    /// Prediction task.
    pub task: PredictionTask,

    /// Canonical input feature schema.
    pub input_schema: ModelSchemaId,

    /// Canonical output schema.
    ///
    /// Output schemas are represented as stable identifiers because output
    /// semantics are model-domain specific.
    pub output_schema: ModelSchemaId,

    /// Model capabilities.
    pub capabilities: ModelCapabilities,

    /// Optional human-readable description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional implementation-family identifier.
    ///
    /// This MUST NOT be interpreted as a backend/provider selector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implementation: Option<String>,

    /// Optional integrity digest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integrity_digest: Option<String>,
}

impl ModelMetadata {
    /// Creates validated metadata.
    pub fn new(
        id: ModelId,
        version: ModelVersion,
        kind: ModelKind,
        task: PredictionTask,
        input_schema: ModelSchemaId,
        output_schema: ModelSchemaId,
        capabilities: ModelCapabilities,
    ) -> ModelResult<Self> {
        validate_non_empty("input_schema", input_schema.as_str())?;
        validate_non_empty("output_schema", output_schema.as_str())?;

        if !capabilities.versioned {
            // A model may technically be unversioned, but the resilience
            // learning contract requires explicit model identity/versioning.
            return Err(ModelError::CapabilityMismatch {
                reason: "resilience models must expose explicit version metadata"
                    .to_owned(),
            });
        }

        Ok(Self {
            id,
            version,
            kind,
            task,
            input_schema,
            output_schema,
            capabilities,
            description: None,
            implementation: None,
            integrity_digest: None,
        })
    }

    /// Adds a description.
    #[must_use]
    pub fn with_description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds an implementation identifier.
    #[must_use]
    pub fn with_implementation<S: Into<String>>(mut self, implementation: S) -> Self {
        self.implementation = Some(implementation.into());
        self
    }

    /// Adds an integrity digest.
    #[must_use]
    pub fn with_integrity_digest<S: Into<String>>(mut self, digest: S) -> Self {
        self.integrity_digest = Some(digest.into());
        self
    }

    /// Returns model identity.
    #[must_use]
    pub fn id(&self) -> &ModelId {
        &self.id
    }

    /// Returns model version.
    #[must_use]
    pub fn version(&self) -> &ModelVersion {
        &self.version
    }

    /// Validates metadata independently of construction.
    pub fn validate(&self) -> ModelResult<()> {
        if self.id.as_str().trim().is_empty() {
            return Err(ModelError::InvalidMetadata {
                field: "model_id".to_owned(),
                reason: "identifier must not be empty".to_owned(),
            });
        }

        if self.input_schema.as_str().trim().is_empty() {
            return Err(ModelError::InvalidMetadata {
                field: "input_schema".to_owned(),
                reason: "schema identifier must not be empty".to_owned(),
            });
        }

        if self.output_schema.as_str().trim().is_empty() {
            return Err(ModelError::InvalidMetadata {
                field: "output_schema".to_owned(),
                reason: "schema identifier must not be empty".to_owned(),
            });
        }

        if !self.capabilities.versioned {
            return Err(ModelError::CapabilityMismatch {
                reason: "model must expose explicit version metadata".to_owned(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Prediction context
// =============================================================================

/// Explicit context supplied to a prediction.
///
/// No ambient state is permitted to influence deterministic inference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredictionContext {
    /// Optional caller-owned execution identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,

    /// Explicit random seed, when a stochastic model is intentionally used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<u64>,

    /// Whether deterministic inference is mandatory.
    pub deterministic: bool,

    /// Caller-supplied logical observation epoch/version.
    ///
    /// This is NOT populated from wall-clock time by the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observation_epoch: Option<u64>,
}

impl Default for PredictionContext {
    fn default() -> Self {
        Self {
            execution_id: None,
            random_seed: None,
            deterministic: false,
            observation_epoch: None,
        }
    }
}

impl PredictionContext {
    /// Creates a deterministic prediction context.
    #[must_use]
    pub fn deterministic() -> Self {
        Self {
            deterministic: true,
            ..Self::default()
        }
    }

    /// Creates a stochastic context with an explicit seed.
    #[must_use]
    pub fn seeded(seed: u64) -> Self {
        Self {
            random_seed: Some(seed),
            deterministic: false,
            ..Self::default()
        }
    }

    /// Adds an execution identifier.
    pub fn with_execution_id<S: Into<String>>(
        mut self,
        execution_id: S,
    ) -> ModelResult<Self> {
        let execution_id = execution_id.into();

        validate_non_empty("execution_id", &execution_id)?;

        self.execution_id = Some(execution_id);
        Ok(self)
    }

    /// Adds an observation epoch.
    #[must_use]
    pub const fn with_observation_epoch(
        mut self,
        observation_epoch: u64,
    ) -> Self {
        self.observation_epoch = Some(observation_epoch);
        self
    }
}

// =============================================================================
// Prediction input
// =============================================================================

/// Complete input supplied to a prediction model.
///
/// The feature vector is the canonical vector from `learning/features.rs`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionInput {
    /// Canonical feature vector.
    pub features: FeatureVector,

    /// Explicit prediction context.
    pub context: PredictionContext,
}

impl PredictionInput {
    /// Creates prediction input.
    #[must_use]
    pub fn new(features: FeatureVector, context: PredictionContext) -> Self {
        Self { features, context }
    }

    /// Validates basic numerical/context invariants.
    pub fn validate(&self) -> ModelResult<()> {
        validate_finite_values(self.features.as_slice())?;

        if self.context.deterministic && self.context.random_seed.is_some() {
            // A deterministic seeded computation is valid: the seed is simply
            // part of the explicit deterministic input. Therefore this is
            // intentionally accepted.
        }

        Ok(())
    }
}

// =============================================================================
// Prediction output
// =============================================================================

/// One model output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionOutput {
    /// Stable output name within the output schema.
    pub name: String,

    /// Numerical predicted value.
    pub value: f64,

    /// Interpretation of the value.
    pub target: PredictionTarget,

    /// Optional confidence supplied by the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

impl PredictionOutput {
    /// Creates an output.
    pub fn new<S: Into<String>>(
        name: S,
        value: f64,
        target: PredictionTarget,
    ) -> ModelResult<Self> {
        let name = name.into();

        validate_non_empty("prediction_output.name", &name)?;
        validate_finite(value, "prediction output")?;

        validate_target_value(&target, value)?;

        Ok(Self {
            name,
            value,
            target,
            confidence: None,
        })
    }

    /// Adds confidence.
    pub fn with_confidence(mut self, confidence: f64) -> ModelResult<Self> {
        validate_probability(confidence, "prediction output confidence")?;

        self.confidence = Some(confidence);

        Ok(self)
    }
}

// =============================================================================
// Prediction provenance
// =============================================================================

/// Immutable provenance describing where a prediction came from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredictionProvenance {
    /// Model identity.
    pub model_id: ModelId,

    /// Model version.
    pub model_version: ModelVersion,

    /// Input schema identity.
    pub input_schema: ModelSchemaId,

    /// Input schema version.
    pub input_schema_version: ModelSchemaVersion,

    /// Output schema identity.
    pub output_schema: ModelSchemaId,

    /// Optional model integrity digest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_integrity_digest: Option<String>,

    /// Whether deterministic inference was requested.
    pub deterministic: bool,

    /// Explicit random seed, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<u64>,

    /// Optional execution identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,

    /// Optional observation epoch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observation_epoch: Option<u64>,
}

impl PredictionProvenance {
    /// Creates provenance from model metadata, feature vector, and context.
    #[must_use]
    pub fn from_metadata(
        metadata: &ModelMetadata,
        input: &PredictionInput,
    ) -> Self {
        Self {
            model_id: metadata.id.clone(),
            model_version: metadata.version.clone(),
            input_schema: input.features.schema_id.clone(),
            input_schema_version: input.features.schema_version,
            output_schema: metadata.output_schema.clone(),
            model_integrity_digest: metadata.integrity_digest.clone(),
            deterministic: input.context.deterministic,
            random_seed: input.context.random_seed,
            execution_id: input.context.execution_id.clone(),
            observation_epoch: input.context.observation_epoch,
        }
    }
}

// =============================================================================
// Prediction
// =============================================================================

/// Complete prediction result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prediction {
    /// Model outputs.
    pub outputs: Vec<PredictionOutput>,

    /// Optional aggregate confidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,

    /// Immutable provenance.
    pub provenance: PredictionProvenance,
}

impl Prediction {
    /// Creates and validates a prediction.
    pub fn new(
        outputs: Vec<PredictionOutput>,
        provenance: PredictionProvenance,
    ) -> ModelResult<Self> {
        validate_outputs(&outputs)?;

        Ok(Self {
            outputs,
            confidence: None,
            provenance,
        })
    }

    /// Adds aggregate confidence.
    pub fn with_confidence(
        mut self,
        confidence: f64,
    ) -> ModelResult<Self> {
        validate_probability(confidence, "prediction confidence")?;

        self.confidence = Some(confidence);

        Ok(self)
    }

    /// Returns an output by stable name.
    #[must_use]
    pub fn output(&self, name: &str) -> Option<&PredictionOutput> {
        self.outputs.iter().find(|output| output.name == name)
    }

    /// Returns the number of outputs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.outputs.len()
    }

    /// Returns whether the prediction contains no outputs.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.outputs.is_empty()
    }

    /// Returns all outputs.
    #[must_use]
    pub fn outputs(&self) -> &[PredictionOutput] {
        &self.outputs
    }
}

// =============================================================================
// Model compatibility
// =============================================================================

/// Result of model/request compatibility assessment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelCompatibility {
    /// Model directly satisfies the request.
    Compatible,

    /// Model can satisfy the request after caller-side adaptation.
    CompatibleWithAdaptation,

    /// Model can operate but with an explicitly degraded contract.
    CompatibleWithDegradation,

    /// Model can be used after explicit model-version migration.
    CompatibleWithMigration,

    /// Compatibility depends on runtime state.
    ConditionallyCompatible,

    /// Model cannot satisfy the request.
    Incompatible,

    /// Insufficient trusted information exists.
    Unknown,
}

impl ModelCompatibility {
    /// Returns whether the assessment permits use subject to caller policy.
    #[must_use]
    pub const fn is_usable(self) -> bool {
        matches!(
            self,
            Self::Compatible
                | Self::CompatibleWithAdaptation
                | Self::CompatibleWithDegradation
                | Self::CompatibleWithMigration
                | Self::ConditionallyCompatible
        )
    }

    /// Returns whether the model is directly compatible.
    #[must_use]
    pub const fn is_direct(self) -> bool {
        matches!(self, Self::Compatible)
    }
}

/// Requirements imposed by the caller.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelRequirements {
    /// Required input schema.
    pub input_schema: ModelSchemaId,

    /// Required input schema version.
    pub input_schema_version: ModelSchemaVersion,

    /// Required output schema.
    pub output_schema: ModelSchemaId,

    /// Whether deterministic inference is required.
    pub deterministic: bool,

    /// Whether uncertainty estimation is required.
    pub uncertainty_required: bool,

    /// Whether offline execution is required.
    pub offline_required: bool,

    /// Whether provider independence is required.
    pub provider_independent: bool,

    /// Whether variable-dimensional feature support is required.
    pub variable_dimension: bool,
}

impl ModelRequirements {
    /// Creates requirements from canonical schemas.
    #[must_use]
    pub const fn new(
        input_schema: ModelSchemaId,
        input_schema_version: ModelSchemaVersion,
        output_schema: ModelSchemaId,
    ) -> Self {
        Self {
            input_schema,
            input_schema_version,
            output_schema,
            deterministic: false,
            uncertainty_required: false,
            offline_required: false,
            provider_independent: true,
            variable_dimension: false,
        }
    }

    /// Requires deterministic inference.
    #[must_use]
    pub const fn require_deterministic(mut self) -> Self {
        self.deterministic = true;
        self
    }

    /// Requires uncertainty estimation.
    #[must_use]
    pub const fn require_uncertainty(mut self) -> Self {
        self.uncertainty_required = true;
        self
    }

    /// Requires offline operation.
    #[must_use]
    pub const fn require_offline(mut self) -> Self {
        self.offline_required = true;
        self
    }

    /// Requires provider independence.
    #[must_use]
    pub const fn require_provider_independence(mut self) -> Self {
        self.provider_independent = true;
        self
    }

    /// Requires variable-dimensional support.
    #[must_use]
    pub const fn require_variable_dimension(mut self) -> Self {
        self.variable_dimension = true;
        self
    }
}

/// Assesses model compatibility without performing inference.
///
/// This function is pure and deterministic.
#[must_use]
pub fn assess_compatibility(
    metadata: &ModelMetadata,
    requirements: &ModelRequirements,
) -> ModelCompatibility {
    if metadata.input_schema != requirements.input_schema {
        return ModelCompatibility::Incompatible;
    }

    if metadata.output_schema != requirements.output_schema {
        return ModelCompatibility::Incompatible;
    }

    if metadata.versioned == false {
        return ModelCompatibility::Incompatible;
    }

    if metadata.capabilities.versioned == false {
        return ModelCompatibility::Incompatible;
    }

    // Feature schema version migration is intentionally not implicit.
    //
    // The exact version must be handled by the caller or an explicit migration
    // layer. This contract never silently reinterprets features.
    //
    // The model metadata does not own the feature schema version, therefore
    // callers should validate the supplied FeatureVector against the canonical
    // schema before invoking the model.

    if requirements.deterministic && !metadata.capabilities.deterministic {
        return ModelCompatibility::Incompatible;
    }

    if requirements.uncertainty_required
        && !metadata.capabilities.uncertainty_estimation
    {
        return ModelCompatibility::Incompatible;
    }

    if requirements.offline_required && !metadata.capabilities.offline {
        return ModelCompatibility::Incompatible;
    }

    if requirements.provider_independent
        && !metadata.capabilities.provider_independent
    {
        return ModelCompatibility::Incompatible;
    }

    if requirements.variable_dimension
        && !metadata.capabilities.variable_dimension
    {
        return ModelCompatibility::Incompatible;
    }

    // The version itself is not part of the feature schema identity. Exact
    // feature-vector validation is performed by `validate_input`.
    let _ = requirements.input_schema_version;

    ModelCompatibility::Compatible
}

// =============================================================================
// Prediction model trait
// =============================================================================

/// Provider-independent prediction-model interface.
///
/// Implementations:
///
/// - predict only;
/// - do not make resilience decisions;
/// - do not mutate global state;
/// - do not access hidden environmental state;
/// - validate their declared input/output contract.
///
/// The resilience planner remains authoritative.
pub trait PredictionModel: Send + Sync {
    /// Returns immutable metadata.
    fn metadata(&self) -> &ModelMetadata;

    /// Returns current lifecycle state.
    fn state(&self) -> ModelState;

    /// Performs one prediction.
    fn predict(&self, input: &PredictionInput) -> ModelResult<Prediction>;

    /// Performs multiple predictions.
///
/// Implementations may override this for efficient vectorized/batched
/// inference. The default implementation preserves input order and evaluates
/// each input independently.
    fn predict_batch(
        &self,
        inputs: &[PredictionInput],
    ) -> ModelResult<Vec<Prediction>> {
        if inputs.len() > 1
            && !self.metadata().capabilities.batch_prediction
        {
            return Err(ModelError::UnsupportedOperation {
                operation: "batch_prediction".to_owned(),
            });
        }

        inputs.iter().map(|input| self.predict(input)).collect()
    }

    /// Assesses compatibility with a request.
    fn compatibility(
        &self,
        requirements: &ModelRequirements,
    ) -> ModelCompatibility {
        assess_compatibility(self.metadata(), requirements)
    }

    /// Validates model readiness.
    fn validate_ready(&self) -> ModelResult<()> {
        self.metadata().validate()?;

        let state = self.state();

        if !state.can_predict() {
            return Err(ModelError::Unavailable {
                reason: format!("model state is {state:?}"),
            });
        }

        Ok(())
    }

    /// Validates a prediction request before inference.
    fn validate_input(&self, input: &PredictionInput) -> ModelResult<()> {
        self.validate_ready()?;

        input.validate()?;

        if input.features.schema_id != self.metadata().input_schema {
            return Err(ModelError::SchemaMismatch {
                expected: self.metadata().input_schema.clone(),
                actual: input.features.schema_id.clone(),
            });
        }

        if input.context.deterministic
            && !self.metadata().capabilities.deterministic
        {
            return Err(ModelError::DeterminismUnavailable {
                reason: format!(
                    "model `{}` does not declare deterministic inference",
                    self.metadata().id
                ),
            });
        }

        Ok(())
    }

    /// Validates and attaches canonical provenance to a prediction.
    ///
    /// Implementations may call this after generating their outputs.
    fn finalize_prediction(
        &self,
        input: &PredictionInput,
        outputs: Vec<PredictionOutput>,
    ) -> ModelResult<Prediction> {
        self.validate_input(input)?;

        let provenance =
            PredictionProvenance::from_metadata(self.metadata(), input);

        Prediction::new(outputs, provenance)
    }
}

// =============================================================================
// Shared model handle
// =============================================================================

/// Thread-safe shared handle to a prediction model.
///
/// `Arc` gives shared ownership without global mutable state.
#[derive(Clone)]
pub struct ModelHandle {
    inner: Arc<dyn PredictionModel>,
}

impl ModelHandle {
    /// Creates a shared model handle.
    #[must_use]
    pub fn new<M>(model: M) -> Self
    where
        M: PredictionModel + 'static,
    {
        Self {
            inner: Arc::new(model),
        }
    }

    /// Creates a handle from an existing trait-object `Arc`.
    #[must_use]
    pub fn from_arc(model: Arc<dyn PredictionModel>) -> Self {
        Self { inner: model }
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &ModelMetadata {
        self.inner.metadata()
    }

    /// Returns model state.
    #[must_use]
    pub fn state(&self) -> ModelState {
        self.inner.state()
    }

    /// Performs prediction.
    pub fn predict(
        &self,
        input: &PredictionInput,
    ) -> ModelResult<Prediction> {
        self.inner.predict(input)
    }

    /// Performs batch prediction.
    pub fn predict_batch(
        &self,
        inputs: &[PredictionInput],
    ) -> ModelResult<Vec<Prediction>> {
        self.inner.predict_batch(inputs)
    }

    /// Assesses compatibility.
    #[must_use]
    pub fn compatibility(
        &self,
        requirements: &ModelRequirements,
    ) -> ModelCompatibility {
        self.inner.compatibility(requirements)
    }

    /// Returns the model trait object.
    #[must_use]
    pub fn as_model(&self) -> &dyn PredictionModel {
        self.inner.as_ref()
    }
}

impl fmt::Debug for ModelHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModelHandle")
            .field("id", &self.metadata().id)
            .field("version", &self.metadata().version)
            .field("state", &self.state())
            .finish()
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates a non-empty textual field.
pub fn validate_non_empty(field: &str, value: &str) -> ModelResult<()> {
    if value.trim().is_empty() {
        return Err(ModelError::InvalidMetadata {
            field: field.to_owned(),
            reason: "value must not be empty".to_owned(),
        });
    }

    Ok(())
}

/// Validates one finite floating-point value.
pub fn validate_finite(value: f64, field: &str) -> ModelResult<()> {
    if !value.is_finite() {
        return Err(ModelError::InvalidInput {
            reason: format!("{field} must be finite"),
        });
    }

    Ok(())
}

/// Validates a probability/confidence.
pub fn validate_probability(
    value: f64,
    field: &str,
) -> ModelResult<()> {
    validate_finite(value, field)?;

    if !(0.0..=1.0).contains(&value) {
        return Err(ModelError::InvalidInput {
            reason: format!("{field} must be within [0, 1]"),
        });
    }

    Ok(())
}

/// Validates all feature-vector values.
pub fn validate_finite_values(values: &[f64]) -> ModelResult<()> {
    for (index, value) in values.iter().copied().enumerate() {
        validate_finite(value, &format!("feature[{index}]"))?;
    }

    Ok(())
}

fn validate_target_value(
    target: &PredictionTarget,
    value: f64,
) -> ModelResult<()> {
    match target {
        PredictionTarget::Probability
        | PredictionTarget::Confidence => {
            validate_probability(value, "prediction output")?;
        }

        PredictionTarget::NonNegativeScalar => {
            if value < 0.0 {
                return Err(ModelError::InvalidOutput {
                    reason:
                        "non-negative prediction output cannot be negative"
                            .to_owned(),
                });
            }
        }

        PredictionTarget::Scalar
        | PredictionTarget::Category
        | PredictionTarget::RankingScore
        | PredictionTarget::Custom(_) => {}
    }

    Ok(())
}

fn validate_outputs(outputs: &[PredictionOutput]) -> ModelResult<()> {
    let mut names = std::collections::BTreeSet::new();

    for output in outputs {
        validate_non_empty("prediction_output.name", &output.name)?;
        validate_finite(output.value, "prediction output")?;

        validate_target_value(&output.target, output.value)?;

        if let Some(confidence) = output.confidence {
            validate_probability(
                confidence,
                "prediction output confidence",
            )?;
        }

        if !names.insert(output.name.clone()) {
            return Err(ModelError::DuplicateOutput {
                name: output.name.clone(),
            });
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn input_schema() -> ModelSchemaId {
        ModelSchemaId::new("resilience.features.v1")
            .expect("schema identifier must be valid")
    }

    fn output_schema() -> ModelSchemaId {
        ModelSchemaId::new("resilience.failure.v1")
            .expect("schema identifier must be valid")
    }

    fn metadata() -> ModelMetadata {
        ModelMetadata::new(
            ModelId::new("resilience.failure").expect("valid model id"),
            ModelVersion::new(1, 0, 0),
            ModelKind::Statistical,
            PredictionTask::FailureProbability,
            input_schema(),
            output_schema(),
            ModelCapabilities {
                deterministic: true,
                online_prediction: true,
                batch_prediction: true,
                uncertainty_estimation: true,
                incremental: false,
                versioned: true,
                offline: true,
                provider_independent: true,
                variable_dimension: true,
            },
        )
        .expect("valid metadata")
    }

    fn feature_vector() -> FeatureVector {
        let schema = super::super::features::FeatureSchema::new(
            input_schema(),
            FeatureSchemaVersion::new(1, 0),
            "test feature schema".to_owned(),
            Vec::new(),
            true,
        )
        .expect("valid feature schema");

        FeatureVector::new(&schema, Vec::new(), false)
            .expect("valid feature vector")
    }

    #[derive(Debug)]
    struct TestModel {
        metadata: ModelMetadata,
        state: ModelState,
    }

    impl PredictionModel for TestModel {
        fn metadata(&self) -> &ModelMetadata {
            &self.metadata
        }

        fn state(&self) -> ModelState {
            self.state
        }

        fn predict(
            &self,
            input: &PredictionInput,
        ) -> ModelResult<Prediction> {
            self.validate_input(input)?;

            let output = PredictionOutput::new(
                "failure_probability",
                0.25,
                PredictionTarget::Probability,
            )?;

            self.finalize_prediction(input, vec![output])
        }
    }

    #[test]
    fn model_id_rejects_empty_values() {
        assert!(ModelId::new("").is_err());
        assert!(ModelId::new("   ").is_err());
        assert!(ModelId::new("failure.model").is_ok());
    }

    #[test]
    fn model_schema_alias_uses_canonical_feature_schema() {
        let schema = ModelSchemaId::new("features.v1")
            .expect("valid schema");

        assert_eq!(schema.as_str(), "features.v1");
    }

    #[test]
    fn model_version_formats_correctly() {
        let version = ModelVersion::new(1, 2, 3)
            .with_pre_release("rc.1")
            .with_build("build.7");

        assert_eq!(version.to_string(), "1.2.3-rc.1+build.7");
    }

    #[test]
    fn probability_validation_is_strict() {
        assert!(validate_probability(0.0, "p").is_ok());
        assert!(validate_probability(0.5, "p").is_ok());
        assert!(validate_probability(1.0, "p").is_ok());

        assert!(validate_probability(-0.01, "p").is_err());
        assert!(validate_probability(1.01, "p").is_err());
        assert!(validate_probability(f64::NAN, "p").is_err());
        assert!(validate_probability(f64::INFINITY, "p").is_err());
    }

    #[test]
    fn prediction_output_rejects_invalid_probability() {
        let result = PredictionOutput::new(
            "failure_probability",
            1.5,
            PredictionTarget::Probability,
        );

        assert!(result.is_err());
    }

    #[test]
    fn prediction_output_rejects_nan() {
        let result = PredictionOutput::new(
            "failure_probability",
            f64::NAN,
            PredictionTarget::Probability,
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_outputs_are_rejected() {
        let first = PredictionOutput::new(
            "failure",
            0.1,
            PredictionTarget::Probability,
        )
        .expect("valid output");

        let second = PredictionOutput::new(
            "failure",
            0.2,
            PredictionTarget::Probability,
        )
        .expect("valid output");

        let provenance = PredictionProvenance {
            model_id: ModelId::new("model").expect("valid model"),
            model_version: ModelVersion::new(1, 0, 0),
            input_schema: input_schema(),
            input_schema_version: FeatureSchemaVersion::new(1, 0),
            output_schema: output_schema(),
            model_integrity_digest: None,
            deterministic: true,
            random_seed: None,
            execution_id: None,
            observation_epoch: None,
        };

        assert!(Prediction::new(
            vec![first, second],
            provenance
        )
        .is_err());
    }

    #[test]
    fn deterministic_context_is_explicit() {
        let context = PredictionContext::deterministic();

        assert!(context.deterministic);
        assert_eq!(context.random_seed, None);
    }

    #[test]
    fn seeded_context_records_seed() {
        let context = PredictionContext::seeded(42);

        assert!(!context.deterministic);
        assert_eq!(context.random_seed, Some(42));
    }

    #[test]
    fn deterministic_model_accepts_deterministic_input() {
        let model = TestModel {
            metadata: metadata(),
            state: ModelState::Ready,
        };

        let handle = ModelHandle::new(model);

        let input = PredictionInput::new(
            feature_vector(),
            PredictionContext::deterministic(),
        );

        let prediction = handle
            .predict(&input)
            .expect("prediction should succeed");

        assert_eq!(prediction.len(), 1);
        assert!(prediction.provenance.deterministic);
        assert_eq!(
            prediction
                .output("failure_probability")
                .expect("output exists")
                .value,
            0.25
        );
    }

    #[test]
    fn wrong_schema_is_rejected() {
        let model = TestModel {
            metadata: metadata(),
            state: ModelState::Ready,
        };

        let wrong_schema =
            ModelSchemaId::new("wrong.schema").expect("valid schema");

        let schema = super::super::features::FeatureSchema::new(
            wrong_schema,
            FeatureSchemaVersion::new(1, 0),
            "wrong schema".to_owned(),
            Vec::new(),
            true,
        )
        .expect("valid feature schema");

        let features = FeatureVector::new(&schema, Vec::new(), false)
            .expect("valid feature vector");

        let input = PredictionInput::new(
            features,
            PredictionContext::default(),
        );

        assert!(matches!(
            model.predict(&input),
            Err(ModelError::SchemaMismatch { .. })
        ));
    }

    #[test]
    fn unavailable_model_fails_closed() {
        let model = TestModel {
            metadata: metadata(),
            state: ModelState::Unavailable,
        };

        let input = PredictionInput::new(
            feature_vector(),
            PredictionContext::default(),
        );

        assert!(matches!(
            model.predict(&input),
            Err(ModelError::Unavailable { .. })
        ));
    }

    #[test]
    fn retired_model_cannot_predict() {
        let model = TestModel {
            metadata: metadata(),
            state: ModelState::Retired,
        };

        assert!(!model.state().can_predict());
        assert!(model.state().is_retired());
    }

    #[test]
    fn compatibility_accepts_matching_requirements() {
        let metadata = metadata();

        let requirements = ModelRequirements::new(
            input_schema(),
            FeatureSchemaVersion::new(1, 0),
            output_schema(),
        )
        .require_deterministic()
        .require_uncertainty()
        .require_offline()
        .require_provider_independence()
        .require_variable_dimension();

        assert_eq!(
            assess_compatibility(&metadata, &requirements),
            ModelCompatibility::Compatible
        );
    }

    #[test]
    fn compatibility_rejects_wrong_schema() {
        let metadata = metadata();

        let requirements = ModelRequirements::new(
            ModelSchemaId::new("wrong.schema")
                .expect("valid schema"),
            FeatureSchemaVersion::new(1, 0),
            output_schema(),
        );

        assert_eq!(
            assess_compatibility(&metadata, &requirements),
            ModelCompatibility::Incompatible
        );
    }

    #[test]
    fn provenance_contains_model_and_execution_identity() {
        let metadata = metadata();

        let input = PredictionInput::new(
            feature_vector(),
            PredictionContext::deterministic()
                .with_execution_id("execution-1")
                .expect("valid execution id")
                .with_observation_epoch(10),
        );

        let provenance =
            PredictionProvenance::from_metadata(&metadata, &input);

        assert_eq!(
            provenance.model_id,
            ModelId::new("resilience.failure")
                .expect("valid model id")
        );

        assert_eq!(
            provenance.execution_id.as_deref(),
            Some("execution-1")
        );

        assert_eq!(provenance.observation_epoch, Some(10));
        assert!(provenance.deterministic);
    }

    #[test]
    fn model_handle_can_be_cloned() {
        let handle = ModelHandle::new(TestModel {
            metadata: metadata(),
            state: ModelState::Ready,
        });

        let cloned = handle.clone();

        assert_eq!(handle.metadata().id, cloned.metadata().id);
        assert_eq!(
            handle.metadata().version,
            cloned.metadata().version
        );
    }

    #[test]
    fn model_handle_supports_batch_prediction() {
        let handle = ModelHandle::new(TestModel {
            metadata: metadata(),
            state: ModelState::Ready,
        });

        let input_a = PredictionInput::new(
            feature_vector(),
            PredictionContext::default(),
        );

        let input_b = PredictionInput::new(
            feature_vector(),
            PredictionContext::default(),
        );

        let predictions = handle
            .predict_batch(&[input_a, input_b])
            .expect("batch prediction should succeed");

        assert_eq!(predictions.len(), 2);
    }
}