//! # Zamani Quantum Resilience — Learning Model Contract
//!
//! Path:
//! `src/quantum/resilience/learning/model.rs`
//!
//! ## Purpose
//!
//! This module defines the provider-independent contract for prediction models
//! used by the quantum-resilience learning subsystem.
//!
//! A model may predict quantities such as:
//!
//! - execution-failure probability;
//! - hardware/resource degradation probability;
//! - recovery success probability;
//! - expected execution latency;
//! - expected fidelity/error characteristics;
//! - mitigation effectiveness;
//! - strategy suitability;
//! - resource availability;
//! - other explicitly versioned resilience quantities.
//!
//! This module intentionally does NOT implement a machine-learning algorithm.
//! Concrete implementations may live elsewhere and implement [`PredictionModel`].
//!
//! ## Architectural ownership
//!
//! `learning/model.rs` owns:
//!
//! - model identity;
//! - model version;
//! - model metadata;
//! - model capabilities;
//! - model lifecycle state;
//! - model input/output contract;
//! - prediction confidence;
//! - prediction provenance;
//! - deterministic-model metadata;
//! - model compatibility;
//! - model trait/object boundary.
//!
//! It does NOT own:
//!
//! - feature extraction;
//! - telemetry collection;
//! - history storage;
//! - model training;
//! - strategy selection;
//! - planning;
//! - recovery;
//! - hardware discovery;
//! - QEC;
//! - routing;
//! - scheduling;
//! - canonical quantum IR.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! ## Integration
//!
//! `learning/features.rs` produces feature vectors that can be supplied to
//! [`PredictionModel::predict`].
//!
//! `learning/predictor.rs` consumes [`PredictionModel`] implementations and
//! [`Prediction`] values.
//!
//! `learning/strategy.rs` may use predictions as advisory inputs when ranking
//! strategies.
//!
//! `learning/feedback.rs` may use [`ModelId`], [`ModelVersion`], and prediction
//! provenance when associating verified outcomes with a model.
//!
//! `history/*` supplies historical observations used by the feature layer and
//! training systems; this file does not access history directly.
//!
//! The planner MUST treat model predictions as advisory evidence. A prediction
//! MUST NOT override safety policy, semantic verification, capability
//! validation, or explicit execution constraints.
//!
//! ## Scalability
//!
//! No fixed number of models, features, outputs, resources, qubits, devices,
//! executions, predictions, or training samples is imposed here.
//!
//! Runtime limits may be imposed by:
//!
//! - caller policy;
//! - available memory;
//! - execution budgets;
//! - model implementation requirements;
//! - resource availability;
//! - security policy.
//!
//! This module MUST NOT introduce artificial quantum-machine-size limits.
//!
//! ## Safety
//!
//! - Rust 2021.
//! - Rust 1.97 / 1.97.1.
//! - No `unsafe`.
//! - No global mutable state.
//! - No provider-specific assumptions.
//! - No hard-coded hardware sizes.
//! - No hard-coded retry counts.
//! - No hidden randomness.
//! - No hidden wall-clock dependency in deterministic prediction.
//!
//! ## Determinism
//!
//! A deterministic model MUST declare that capability through
//! [`ModelCapabilities::deterministic`] and MUST produce reproducible results
//! for identical declared inputs.
//!
//! Randomized models remain valid, but their randomness MUST be represented by
//! the model's input/context or implementation-level provenance rather than
//! being hidden from the resilience system.
//!
//! ## Canonical quantum identity
//!
//! This module does not define a quantum-resource identity.
//!
//! If a concrete model needs logical or physical qubit identity, it MUST use:
//!
//! `crate::quantum::ir::qubit::QubitId`
//! `crate::quantum::ir::qubit::PhysicalQubitId`
//! `crate::quantum::ir::qubit::QubitRef`
//!
//! where appropriate.
//!
//! It MUST NOT define `ResilienceQubitId`, `LearningQubitId`, or another
//! competing quantum identity type.

use std::error::Error;
use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

// =============================================================================
// Result / error contract
// =============================================================================

/// Result type used by the learning-model contract.
pub type ModelResult<T> = Result<T, ModelError>;

/// Errors produced by the model contract.
///
/// The variants intentionally describe contract-level failures rather than
/// implementation-specific ML framework errors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelError {
    /// The model identifier or other model metadata is invalid.
    InvalidMetadata {
        field: String,
        reason: String,
    },

    /// The supplied feature vector is incompatible with the model.
    InvalidInput {
        reason: String,
    },

    /// The model's output violates its declared output contract.
    InvalidOutput {
        reason: String,
    },

    /// The requested operation is not supported by this model.
    UnsupportedOperation {
        operation: String,
    },

    /// The model is not currently usable.
    Unavailable {
        reason: String,
    },

    /// The model cannot safely operate under the requested deterministic mode.
    DeterminismUnavailable {
        reason: String,
    },

    /// The model and requested schema are incompatible.
    SchemaMismatch {
        expected: ModelSchemaId,
        actual: ModelSchemaId,
    },

    /// The model version is incompatible with the caller.
    VersionMismatch {
        required: ModelVersion,
        actual: ModelVersion,
    },

    /// The model's declared capabilities do not satisfy a requirement.
    CapabilityMismatch {
        reason: String,
    },

    /// A prediction cannot be trusted because the model's state is invalid.
    InvalidState {
        reason: String,
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
                    "model schema mismatch: expected {}, got {}",
                    expected.as_str(),
                    actual.as_str()
                )
            }
            Self::VersionMismatch { required, actual } => {
                write!(
                    f,
                    "model version mismatch: required {}, got {}",
                    required,
                    actual
                )
            }
            Self::CapabilityMismatch { reason } => {
                write!(f, "model capability mismatch: {reason}")
            }
            Self::InvalidState { reason } => {
                write!(f, "invalid model state: {reason}")
            }
        }
    }
}

impl Error for ModelError {}

// =============================================================================
// Stable identifiers
// =============================================================================

/// Stable identifier for a prediction model.
///
/// This identifies the model family, not a particular execution instance.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
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

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its string representation.
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

/// Version of a model.
///
/// This is intentionally represented without an external semver dependency so
/// the resilience core does not acquire another dependency merely to identify
/// model versions.
///
/// `major.minor.patch` follows conventional semantic-version ordering.
/// `pre_release` and `build` are preserved as metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ModelVersion {
    /// Major version.
    pub major: u64,

    /// Minor version.
    pub minor: u64,

    /// Patch version.
    pub patch: u64,

    /// Optional pre-release identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_release: Option<String>,

    /// Optional build metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
}

impl ModelVersion {
    /// Creates a stable model version.
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

    /// Returns whether this version has the same semantic major version.
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

/// Stable identifier for the feature schema consumed by a model.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ModelSchemaId(String);

impl ModelSchemaId {
    /// Creates a validated schema identifier.
    pub fn new<S: Into<String>>(value: S) -> ModelResult<Self> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ModelError::InvalidMetadata {
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

impl fmt::Display for ModelSchemaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

// =============================================================================
// Model classification
// =============================================================================

/// General class of prediction model.
///
/// `Custom` is intentionally available so adding a future model family does
/// not require changing this core enum.
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

    /// Kernel or similarity model.
    Kernel,

    /// Rule/model hybrid.
    Hybrid,

    /// Model supplied by an extension.
    Custom(String),
}

/// Prediction task performed by a model.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PredictionTask {
    /// Predict the probability of an execution failure.
    FailureProbability,

    /// Predict the probability that a recovery strategy succeeds.
    RecoverySuccessProbability,

    /// Predict degradation or resource loss.
    DegradationProbability,

    /// Predict expected latency or duration.
    Latency,

    /// Predict expected fidelity or quality.
    Fidelity,

    /// Predict expected error characteristics.
    ErrorRate,

    /// Predict suitability/ranking information.
    StrategySuitability,

    /// Predict resource availability/capacity.
    ResourceAvailability,

    /// Generic probabilistic prediction.
    Probability,

    /// Generic scalar prediction.
    Regression,

    /// Extension-defined task.
    Custom(String),
}

/// Unit/interpretation of a model output.
///
/// The model itself does not decide how the planner should act on a value.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PredictionTarget {
    /// A probability in the closed interval [0, 1].
    Probability,

    /// A confidence value in the closed interval [0, 1].
    Confidence,

    /// A non-negative quantity with caller-defined units.
    NonNegativeScalar,

    /// A scalar with caller-defined semantics.
    Scalar,

    /// A categorical label.
    Category,

    /// A ranking score whose absolute scale has no prescribed meaning.
    RankingScore,

    /// Extension-defined target.
    Custom(String),
}

// =============================================================================
// Model lifecycle
// =============================================================================

/// Operational state of a model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelState {
    /// Model metadata exists but the model is not loaded.
    Registered,

    /// Model is loaded and usable.
    Ready,

    /// Model is temporarily unavailable.
    Unavailable,

    /// Model is being replaced or refreshed.
    Updating,

    /// Model is disabled by policy.
    Disabled,

    /// Model failed validation or integrity checks.
    Invalid,

    /// Model has been retired and must no longer be selected.
    Retired,
}

impl ModelState {
    /// Returns whether prediction is normally permitted in this state.
    #[must_use]
    pub const fn can_predict(self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Returns whether the model is terminally retired.
    #[must_use]
    pub const fn is_retired(self) -> bool {
        matches!(self, Self::Retired)
    }
}

// =============================================================================
// Model capabilities
// =============================================================================

/// Explicit capabilities of a prediction model.
///
/// Capabilities are declarations, not permissions. Safety policy remains
/// authoritative.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelCapabilities {
    /// Whether predictions are deterministic for identical declared inputs.
    pub deterministic: bool,

    /// Whether the model supports online prediction.
    pub online_prediction: bool,

    /// Whether the model supports batch prediction.
    pub batch_prediction: bool,

    /// Whether the model can expose uncertainty/confidence.
    pub uncertainty_estimation: bool,

    /// Whether the model can operate incrementally.
    pub incremental: bool,

    /// Whether the model supports model-version compatibility checks.
    pub versioned: bool,

    /// Whether the model can operate without external network services.
    pub offline: bool,

    /// Whether the model can be evaluated independently of target-provider
    /// identity.
    pub provider_independent: bool,

    /// Whether the model can consume arbitrarily sized feature vectors within
    /// the caller's available resources.
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
    /// Stable model identifier.
    pub id: ModelId,

    /// Model version.
    pub version: ModelVersion,

    /// Model family.
    pub kind: ModelKind,

    /// Prediction task.
    pub task: PredictionTask,

    /// Stable feature-schema identifier.
    pub input_schema: ModelSchemaId,

    /// Stable output-schema identifier.
    pub output_schema: ModelSchemaId,

    /// Capability declaration.
    pub capabilities: ModelCapabilities,

    /// Optional human-readable description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional implementation identifier.
    ///
    /// This identifies the implementation family, not a provider/backend.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub implementation: Option<String>,

    /// Optional content/integrity digest.
    ///
    /// The digest is opaque to this module. A repository-level integrity
    /// subsystem may populate it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integrity_digest: Option<String>,
}

impl ModelMetadata {
    /// Creates validated model metadata.
    pub fn new(
        id: ModelId,
        version: ModelVersion,
        kind: ModelKind,
        task: PredictionTask,
        input_schema: ModelSchemaId,
        output_schema: ModelSchemaId,
        capabilities: ModelCapabilities,
    ) -> ModelResult<Self> {
        if let Some(description) = None::<String> {
            let _ = description;
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

    /// Sets a human-readable description.
    #[must_use]
    pub fn with_description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets an implementation identifier.
    #[must_use]
    pub fn with_implementation<S: Into<String>>(mut self, implementation: S) -> Self {
        self.implementation = Some(implementation.into());
        self
    }

    /// Sets an integrity digest.
    #[must_use]
    pub fn with_integrity_digest<S: Into<String>>(mut self, digest: S) -> Self {
        self.integrity_digest = Some(digest.into());
        self
    }

    /// Returns the model's stable identifier.
    #[must_use]
    pub fn id(&self) -> &ModelId {
        &self.id
    }

    /// Returns the model version.
    #[must_use]
    pub const fn version(&self) -> &ModelVersion {
        &self.version
    }
}

// =============================================================================
// Prediction input
// =============================================================================

/// Explicit execution context supplied to a prediction.
///
/// The model receives only the values represented here; it must not inspect
/// ambient global state to influence a deterministic prediction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionContext {
    /// Optional caller-provided execution identity.
    ///
    /// This is provenance only and MUST NOT be used as an implicit source of
    /// randomness.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,

    /// Optional deterministic seed supplied by the caller.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<u64>,

    /// Whether the caller requires deterministic prediction.
    pub deterministic: bool,

    /// Optional logical timestamp/version supplied by the caller.
    ///
    /// This is deliberately not populated by the model from wall-clock time.
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

/// Feature vector supplied to a model.
///
/// `values` are intentionally stored as a contiguous vector rather than a
/// fixed-size array. This avoids imposing a maximum feature count.
///
/// The feature schema is identified independently so the model can reject
/// incompatible feature representations before evaluation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureVector {
    /// Feature-schema identity.
    pub schema: ModelSchemaId,

    /// Ordered numeric feature values.
    pub values: Vec<f64>,
}

impl FeatureVector {
    /// Creates a feature vector after validating numeric values.
    pub fn new(schema: ModelSchemaId, values: Vec<f64>) -> ModelResult<Self> {
        validate_finite_values(&values)?;

        Ok(Self { schema, values })
    }

    /// Creates an empty feature vector for a valid schema.
    pub fn empty(schema: ModelSchemaId) -> Self {
        Self {
            schema,
            values: Vec::new(),
        }
    }

    /// Returns the number of features.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether no feature values are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the feature values.
    #[must_use]
    pub fn as_slice(&self) -> &[f64] {
        &self.values
    }
}

/// Complete input supplied to a prediction model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionInput {
    /// Feature vector.
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
}

// =============================================================================
// Prediction output
// =============================================================================

/// A single model output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionOutput {
    /// Stable name within the output schema.
    pub name: String,

    /// Predicted value.
    pub value: f64,

    /// Interpretation of the value.
    pub target: PredictionTarget,

    /// Optional model-provided confidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

impl PredictionOutput {
    /// Creates a scalar output.
    pub fn new<S: Into<String>>(
        name: S,
        value: f64,
        target: PredictionTarget,
    ) -> ModelResult<Self> {
        validate_finite(value, "prediction value")?;

        Ok(Self {
            name: name.into(),
            value,
            target,
            confidence: None,
        })
    }

    /// Adds confidence to an output.
    pub fn with_confidence(mut self, confidence: f64) -> ModelResult<Self> {
        validate_probability(confidence, "prediction confidence")?;
        self.confidence = Some(confidence);
        Ok(self)
    }
}

/// Provenance associated with a prediction.
///
/// Provenance is deliberately explicit so learned information cannot become
/// untraceable input to resilience planning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PredictionProvenance {
    /// Model identity.
    pub model_id: ModelId,

    /// Model version.
    pub model_version: ModelVersion,

    /// Input schema identity.
    pub input_schema: ModelSchemaId,

    /// Output schema identity.
    pub output_schema: ModelSchemaId,

    /// Optional model integrity digest.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_integrity_digest: Option<String>,

    /// Whether the prediction was made under deterministic requirements.
    pub deterministic: bool,

    /// Optional seed identity. This is metadata, not hidden randomness.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<u64>,
}

impl PredictionProvenance {
    /// Builds provenance from model metadata and prediction context.
    #[must_use]
    pub fn from_metadata(metadata: &ModelMetadata, context: &PredictionContext) -> Self {
        Self {
            model_id: metadata.id.clone(),
            model_version: metadata.version.clone(),
            input_schema: metadata.input_schema.clone(),
            output_schema: metadata.output_schema.clone(),
            model_integrity_digest: metadata.integrity_digest.clone(),
            deterministic: context.deterministic,
            random_seed: context.random_seed,
        }
    }
}

/// Complete prediction returned by a model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prediction {
    /// Model outputs.
    pub outputs: Vec<PredictionOutput>,

    /// Aggregate prediction confidence when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,

    /// Provenance of the prediction.
    pub provenance: PredictionProvenance,
}

impl Prediction {
    /// Creates a prediction and validates its outputs.
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

    /// Sets aggregate confidence.
    pub fn with_confidence(mut self, confidence: f64) -> ModelResult<Self> {
        validate_probability(confidence, "prediction confidence")?;
        self.confidence = Some(confidence);
        Ok(self)
    }

    /// Finds an output by its stable name.
    #[must_use]
    pub fn output(&self, name: &str) -> Option<&PredictionOutput> {
        self.outputs.iter().find(|output| output.name == name)
    }

    /// Returns the number of outputs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.outputs.len()
    }

    /// Returns whether the prediction has no outputs.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.outputs.is_empty()
    }
}

// =============================================================================
// Compatibility
// =============================================================================

/// Compatibility assessment between a model and a prediction request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelCompatibility {
    /// Model is directly usable.
    Compatible,

    /// Model is usable after caller-side adaptation.
    CompatibleWithAdaptation,

    /// Model is usable only with degraded confidence/capability.
    CompatibleWithDegradation,

    /// Model is usable only after a model-version migration.
    CompatibleWithMigration,

    /// Compatibility depends on runtime conditions.
    ConditionallyCompatible,

    /// Model cannot satisfy the request.
    Incompatible,

    /// There is insufficient trusted information.
    Unknown,
}

impl ModelCompatibility {
    /// Returns whether the result is directly usable.
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
}

/// Requirements used to assess model compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelRequirements {
    /// Required input feature schema.
    pub input_schema: ModelSchemaId,

    /// Required output schema.
    pub output_schema: ModelSchemaId,

    /// Whether deterministic prediction is required.
    pub deterministic: bool,

    /// Whether uncertainty information is required.
    pub uncertainty_required: bool,

    /// Whether offline execution is required.
    pub offline_required: bool,

    /// Whether provider independence is required.
    pub provider_independent: bool,
}

impl ModelRequirements {
    /// Creates requirements.
    #[must_use]
    pub fn new(input_schema: ModelSchemaId, output_schema: ModelSchemaId) -> Self {
        Self {
            input_schema,
            output_schema,
            deterministic: false,
            uncertainty_required: false,
            offline_required: false,
            provider_independent: true,
        }
    }

    /// Requires deterministic prediction.
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

    /// Requires offline execution.
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
}

/// Checks model metadata against caller requirements.
///
/// This is intentionally pure and deterministic.
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

    if requirements.deterministic && !metadata.capabilities.deterministic {
        return ModelCompatibility::Incompatible;
    }

    if requirements.uncertainty_required && !metadata.capabilities.uncertainty_estimation {
        return ModelCompatibility::Incompatible;
    }

    if requirements.offline_required && !metadata.capabilities.offline {
        return ModelCompatibility::Incompatible;
    }

    if requirements.provider_independent && !metadata.capabilities.provider_independent {
        return ModelCompatibility::Incompatible;
    }

    ModelCompatibility::Compatible
}

// =============================================================================
// Prediction model trait
// =============================================================================

/// Generic prediction-model interface.
///
/// Implementations must be:
///
/// - thread-safe when registered as shared models;
/// - explicit about deterministic behavior;
/// - explicit about input/output schemas;
/// - free from hidden resilience decisions.
///
/// The model predicts; the resilience planner decides.
pub trait PredictionModel: Send + Sync {
    /// Returns immutable model metadata.
    fn metadata(&self) -> &ModelMetadata;

    /// Returns the current operational state.
    fn state(&self) -> ModelState;

    /// Performs one prediction.
    fn predict(&self, input: &PredictionInput) -> ModelResult<Prediction>;

    /// Performs multiple predictions.
///
/// Implementations may override this for efficient batching. The default
/// implementation evaluates inputs independently in input order.
    fn predict_batch(&self, inputs: &[PredictionInput]) -> ModelResult<Vec<Prediction>> {
        if !self.metadata().capabilities.batch_prediction && inputs.len() > 1 {
            return Err(ModelError::UnsupportedOperation {
                operation: "batch_prediction".to_owned(),
            });
        }

        inputs.iter().map(|input| self.predict(input)).collect()
    }

    /// Validates compatibility with the requested requirements.
    fn compatibility(&self, requirements: &ModelRequirements) -> ModelCompatibility {
        assess_compatibility(self.metadata(), requirements)
    }

    /// Validates that the model is ready for prediction.
    fn validate_ready(&self) -> ModelResult<()> {
        if !self.state().can_predict() {
            return Err(ModelError::Unavailable {
                reason: format!("model state is {:?}", self.state()),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Shared model handle
// =============================================================================

/// Shared, immutable model handle.
///
/// `Arc` provides scalable shared ownership without global mutable state.
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

    /// Creates a handle from an existing shared model.
    #[must_use]
    pub fn from_arc(model: Arc<dyn PredictionModel>) -> Self {
        Self { inner: model }
    }

    /// Returns the model metadata.
    #[must_use]
    pub fn metadata(&self) -> &ModelMetadata {
        self.inner.metadata()
    }

    /// Returns the model state.
    #[must_use]
    pub fn state(&self) -> ModelState {
        self.inner.state()
    }

    /// Performs a prediction.
    pub fn predict(&self, input: &PredictionInput) -> ModelResult<Prediction> {
        self.inner.predict(input)
    }

    /// Performs batch prediction.
    pub fn predict_batch(&self, inputs: &[PredictionInput]) -> ModelResult<Vec<Prediction>> {
        self.inner.predict_batch(inputs)
    }

    /// Checks compatibility.
    #[must_use]
    pub fn compatibility(&self, requirements: &ModelRequirements) -> ModelCompatibility {
        self.inner.compatibility(requirements)
    }

    /// Returns the underlying model as a trait object.
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

/// Validates a finite floating-point value.
pub fn validate_finite(value: f64, field: &str) -> ModelResult<()> {
    if !value.is_finite() {
        return Err(ModelError::InvalidInput {
            reason: format!("{field} must be finite"),
        });
    }

    Ok(())
}

/// Validates a probability/confidence value.
///
/// Probability values are required to be within [0, 1].
pub fn validate_probability(value: f64, field: &str) -> ModelResult<()> {
    validate_finite(value, field)?;

    if !(0.0..=1.0).contains(&value) {
        return Err(ModelError::InvalidInput {
            reason: format!("{field} must be within [0, 1]"),
        });
    }

    Ok(())
}

/// Validates an entire numeric feature vector.
pub fn validate_finite_values(values: &[f64]) -> ModelResult<()> {
    for (index, value) in values.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(ModelError::InvalidInput {
                reason: format!("feature at index {index} must be finite"),
            });
        }
    }

    Ok(())
}

fn validate_outputs(outputs: &[PredictionOutput]) -> ModelResult<()> {
    for output in outputs {
        if output.name.trim().is_empty() {
            return Err(ModelError::InvalidOutput {
                reason: "prediction output name must not be empty".to_owned(),
            });
        }

        validate_finite(output.value, "prediction output")?;

        if let Some(confidence) = output.confidence {
            validate_probability(confidence, "prediction output confidence")?;
        }

        if matches!(output.target, PredictionTarget::Probability)
            && !(0.0..=1.0).contains(&output.value)
        {
            return Err(ModelError::InvalidOutput {
                reason: format!(
                    "probability output `{}` must be within [0, 1]",
                    output.name
                ),
            });
        }

        if matches!(output.target, PredictionTarget::Confidence)
            && !(0.0..=1.0).contains(&output.value)
        {
            return Err(ModelError::InvalidOutput {
                reason: format!(
                    "confidence output `{}` must be within [0, 1]",
                    output.name
                ),
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

    fn schema(name: &str) -> ModelSchemaId {
        ModelSchemaId::new(name).expect("valid schema")
    }

    fn model_metadata() -> ModelMetadata {
        ModelMetadata::new(
            ModelId::new("test.model").expect("valid model id"),
            ModelVersion::new(1, 0, 0),
            ModelKind::Statistical,
            PredictionTask::FailureProbability,
            schema("features.v1"),
            schema("prediction.failure.v1"),
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

        fn predict(&self, input: &PredictionInput) -> ModelResult<Prediction> {
            self.validate_ready()?;

            if input.features.schema != self.metadata.input_schema {
                return Err(ModelError::SchemaMismatch {
                    expected: self.metadata.input_schema.clone(),
                    actual: input.features.schema.clone(),
                });
            }

            let value = if input.features.values.is_empty() {
                0.0
            } else {
                input.features.values[0]
            };

            let output = PredictionOutput::new(
                "failure_probability",
                value,
                PredictionTarget::Probability,
            )?;

            Prediction::new(
                vec![output],
                PredictionProvenance::from_metadata(&self.metadata, &input.context),
            )
        }
    }

    #[test]
    fn model_id_rejects_empty_identifier() {
        assert!(ModelId::new("").is_err());
        assert!(ModelId::new("   ").is_err());
        assert!(ModelId::new("resilience.failure").is_ok());
    }

    #[test]
    fn schema_id_rejects_empty_identifier() {
        assert!(ModelSchemaId::new("").is_err());
        assert!(ModelSchemaId::new("features.v1").is_ok());
    }

    #[test]
    fn model_version_formats_stably() {
        let version = ModelVersion::new(1, 2, 3)
            .with_pre_release("rc.1")
            .with_build("build.42");

        assert_eq!(version.to_string(), "1.2.3-rc.1+build.42");
    }

    #[test]
    fn feature_vector_rejects_nan() {
        let result = FeatureVector::new(schema("features.v1"), vec![f64::NAN]);
        assert!(result.is_err());
    }

    #[test]
    fn feature_vector_rejects_infinity() {
        let result = FeatureVector::new(schema("features.v1"), vec![f64::INFINITY]);
        assert!(result.is_err());
    }

    #[test]
    fn probability_validation_is_strict() {
        assert!(validate_probability(0.0, "p").is_ok());
        assert!(validate_probability(0.5, "p").is_ok());
        assert!(validate_probability(1.0, "p").is_ok());
        assert!(validate_probability(-0.01, "p").is_err());
        assert!(validate_probability(1.01, "p").is_err());
    }

    #[test]
    fn probability_output_is_range_checked() {
        let result = PredictionOutput::new(
            "failure_probability",
            2.0,
            PredictionTarget::Probability,
        );

        assert!(result.is_err());
    }

    #[test]
    fn confidence_is_range_checked() {
        let output =
            PredictionOutput::new("failure_probability", 0.5, PredictionTarget::Probability)
                .expect("valid output");

        assert!(output.with_confidence(0.9).is_ok());
        assert!(output.with_confidence(-0.1).is_err());
        assert!(output.with_confidence(1.1).is_err());
    }

    #[test]
    fn compatibility_rejects_wrong_input_schema() {
        let metadata = model_metadata();

        let requirements =
            ModelRequirements::new(schema("features.v2"), schema("prediction.failure.v1"));

        assert_eq!(
            assess_compatibility(&metadata, &requirements),
            ModelCompatibility::Incompatible
        );
    }

    #[test]
    fn compatibility_accepts_matching_requirements() {
        let metadata = model_metadata();

        let requirements =
            ModelRequirements::new(schema("features.v1"), schema("prediction.failure.v1"))
                .require_deterministic()
                .require_uncertainty()
                .require_offline()
                .require_provider_independence();

        assert_eq!(
            assess_compatibility(&metadata, &requirements),
            ModelCompatibility::Compatible
        );
    }

    #[test]
    fn shared_model_handle_is_usable() {
        let model = TestModel {
            metadata: model_metadata(),
            state: ModelState::Ready,
        };

        let handle = ModelHandle::new(model);

        let features =
            FeatureVector::new(schema("features.v1"), vec![0.25]).expect("valid features");

        let input = PredictionInput::new(
            features,
            PredictionContext {
                deterministic: true,
                ..PredictionContext::default()
            },
        );

        let prediction = handle.predict(&input).expect("prediction succeeds");

        assert_eq!(prediction.len(), 1);
        assert_eq!(
            prediction
                .output("failure_probability")
                .expect("output exists")
                .value,
            0.25
        );
    }

    #[test]
    fn unavailable_model_fails_closed() {
        let model = TestModel {
            metadata: model_metadata(),
            state: ModelState::Unavailable,
        };

        let handle = ModelHandle::new(model);

        let features =
            FeatureVector::new(schema("features.v1"), vec![0.25]).expect("valid features");

        let input = PredictionInput::new(features, PredictionContext::default());

        assert!(matches!(
            handle.predict(&input),
            Err(ModelError::Unavailable { .. })
        ));
    }

    #[test]
    fn deterministic_context_is_preserved_in_provenance() {
        let metadata = model_metadata();

        let context = PredictionContext {
            execution_id: Some("execution.example".to_owned()),
            random_seed: Some(42),
            deterministic: true,
            observation_epoch: Some(7),
        };

        let provenance = PredictionProvenance::from_metadata(&metadata, &context);

        assert!(provenance.deterministic);
        assert_eq!(provenance.random_seed, Some(42));
        assert_eq!(
            provenance.model_id,
            ModelId::new("test.model").expect("valid model id")
        );
    }

    #[test]
    fn model_state_only_allows_ready_for_prediction() {
        assert!(!ModelState::Registered.can_predict());
        assert!(ModelState::Ready.can_predict());
        assert!(!ModelState::Unavailable.can_predict());
        assert!(!ModelState::Updating.can_predict());
        assert!(!ModelState::Disabled.can_predict());
        assert!(!ModelState::Invalid.can_predict());
        assert!(!ModelState::Retired.can_predict());
    }

    #[test]
    fn prediction_output_lookup_is_stable() {
        let metadata = model_metadata();

        let first =
            PredictionOutput::new("failure_probability", 0.1, PredictionTarget::Probability)
                .expect("valid output");

        let second =
            PredictionOutput::new("latency", 10.0, PredictionTarget::NonNegativeScalar)
                .expect("valid output");

        let prediction = Prediction::new(
            vec![first, second],
            PredictionProvenance::from_metadata(&metadata, &PredictionContext::default()),
        )
        .expect("valid prediction");

        assert_eq!(
            prediction
                .output("failure_probability")
                .expect("output exists")
                .value,
            0.1
        );

        assert_eq!(
            prediction.output("missing"),
            None
        );
    }

    #[test]
    fn model_handle_can_be_cloned() {
        let model = TestModel {
            metadata: model_metadata(),
            state: ModelState::Ready,
        };

        let first = ModelHandle::new(model);
        let second = first.clone();

        assert_eq!(first.metadata().id, second.metadata().id);
        assert_eq!(first.metadata().version, second.metadata().version);
    }

    #[test]
    fn model_schema_is_serializable() {
        let metadata = model_metadata();

        let encoded = serde_json::to_string(&metadata).expect("serialize");
        let decoded: ModelMetadata = serde_json::from_str(&encoded).expect("deserialize");

        assert_eq!(metadata, decoded);
    }
}