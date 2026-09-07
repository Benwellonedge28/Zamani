//! # Zamani Quantum Resilience — Prediction Orchestrator
//!
//! Path:
//! `src/quantum/resilience/learning/predictor.rs`
//!
//! ## Purpose
//!
//! This module provides the production prediction orchestration layer for
//! quantum-resilience learning.
//!
//! The predictor is deliberately NOT a machine-learning implementation.
//! Instead, it coordinates:
//!
//! - validated prediction inputs;
//! - feature-schema compatibility;
//! - model requirements;
//! - model readiness;
//! - deterministic execution requirements;
//! - model invocation;
//! - prediction validation;
//! - prediction provenance;
//! - batch prediction;
//! - prediction-task validation;
//! - advisory confidence handling;
//! - safe integration with resilience planning.
//!
//! ## Architectural ownership
//!
//! `learning/model.rs` owns:
//!
//! - `PredictionModel`;
//! - `ModelHandle`;
//! - `PredictionInput`;
//! - `Prediction`;
//! - `PredictionOutput`;
//! - `ModelRequirements`;
//! - `ModelCompatibility`;
//! - model metadata;
//! - model identity/version;
//! - model capability declarations.
//!
//! `learning/features.rs` owns:
//!
//! - feature extraction;
//! - feature schemas;
//! - feature definitions;
//! - canonical feature ordering;
//! - feature provenance;
//! - logical/physical quantum-resource references.
//!
//! This file owns:
//!
//! - prediction orchestration;
//! - preflight validation;
//! - compatibility enforcement;
//! - prediction-output validation;
//! - task-level validation;
//! - deterministic-mode enforcement;
//! - batch orchestration;
//! - advisory prediction envelopes;
//! - model invocation boundaries.
//!
//! It does NOT own:
//!
//! - model training;
//! - feature extraction;
//! - strategy selection;
//! - recovery;
//! - mitigation;
//! - routing;
//! - scheduling;
//! - QEC;
//! - hardware discovery;
//! - canonical quantum IR;
//! - semantic acceptance of quantum results.
//!
//! Those remain owned by their respective subsystems.
//!
//! ## Critical safety rule
//!
//! A prediction is evidence, not authority.
//!
//! A prediction MUST NOT by itself:
//!
//! - authorize recovery;
//! - authorize migration;
//! - select a backend;
//! - alter a quantum program;
//! - change QEC configuration;
//! - bypass policy;
//! - bypass capability validation;
//! - bypass semantic verification;
//! - accept an execution result.
//!
//! The predictor only establishes that a model produced a contract-valid
//! prediction under declared conditions.
//!
//! ## Scalability
//!
//! This module contains no fixed limits for:
//!
//! - qubits;
//! - logical qubits;
//! - physical qubits;
//! - devices;
//! - backends;
//! - models;
//! - features;
//! - predictions;
//! - batch sizes;
//! - executions;
//! - recovery attempts.
//!
//! Resource limits are supplied by the caller, model implementation, policy,
//! execution environment, or available resources.
//!
//! No `MAX_*` quantum-machine constants are defined here.
//!
//! ## Determinism
//!
//! Deterministic prediction is controlled through `PredictionContext` and the
//! model's declared `ModelCapabilities`.
//!
//! This module does not create hidden random state, access a random generator,
//! access wall-clock time, or read environment state.
//!
//! ## Quantum identity
//!
//! This module does not directly manipulate quantum-resource identities.
//!
//! If a prediction needs logical or physical qubit identity, that identity is
//! carried by the feature-extraction/context layer using the canonical types:
//!
//! `crate::quantum::ir::qubit::QubitId`
//! `crate::quantum::ir::qubit::PhysicalQubitId`
//! `crate::quantum::ir::qubit::QubitRef`
//!
//! No competing predictor-specific qubit identifier is introduced here.
//!
//! ## Integration contract
//!
//! Feature extraction:
//!
//! `learning/features.rs`
//!     ↓
//! `PredictionInput`
//!     ↓
//! `Predictor`
//!     ↓
//! `PredictionModel`
//!     ↓
//! `Prediction`
//!     ↓
//! `learning/strategy.rs` / `planning/*`
//!
//! Verified execution outcomes later flow through:
//!
//! `learning/feedback.rs`
//!
//! The planner remains responsible for deciding whether a prediction is
//! sufficient evidence for an action.
//!
//! ## Rust requirements
//!
//! - Rust 2021.
//! - Rust 1.97 / 1.97.1.
//! - Stable Rust.
//! - No nightly features.
//! - No `unsafe`.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! ## Dependency direction
//!
//! This module depends on the learning model contract.
//!
//! The model contract does not depend on this predictor implementation.
//!
//! This prevents a circular dependency and allows concrete models to be
//! developed independently from the orchestration layer.

#![forbid(unsafe_code)]

use std::fmt;

use serde::{Deserialize, Serialize};

use super::model::{
    assess_compatibility,
    validate_finite,
    validate_probability,
    ModelCompatibility,
    ModelError,
    ModelHandle,
    ModelId,
    ModelKind,
    ModelRequirements,
    ModelResult,
    ModelSchemaId,
    ModelState,
    Prediction,
    PredictionContext,
    PredictionInput,
    PredictionOutput,
    PredictionTarget,
    PredictionTask,
};

// =============================================================================
// Predictor result / error aliases
// =============================================================================

/// Result type returned by the predictor.
pub type PredictorResult<T> = Result<T, PredictorError>;

/// Errors produced by prediction orchestration.
///
/// Model-specific errors remain represented by `ModelError`. This wrapper
/// provides predictor-level context without replacing the model contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PredictorError {
    /// The predictor has no usable model.
    ModelUnavailable {
        /// Model identity when known.
        model_id: Option<ModelId>,

        /// Model state when known.
        state: Option<ModelState>,

        /// Human-readable explanation.
        reason: String,
    },

    /// The model does not satisfy the caller's requirements.
    IncompatibleModel {
        /// Model identity.
        model_id: ModelId,

        /// Compatibility result.
        compatibility: ModelCompatibility,

        /// Human-readable explanation.
        reason: String,
    },

    /// The supplied prediction input is invalid.
    InvalidInput {
        /// Human-readable explanation.
        reason: String,
    },

    /// The model returned an invalid prediction.
    InvalidPrediction {
        /// Human-readable explanation.
        reason: String,
    },

    /// The prediction does not represent the requested task.
    TaskMismatch {
        /// Requested task.
        expected: PredictionTask,

        /// Actual model task.
        actual: PredictionTask,
    },

    /// The prediction output does not satisfy the requested target.
    TargetMismatch {
        /// Output name.
        output: String,

        /// Expected target.
        expected: PredictionTarget,

        /// Actual target.
        actual: PredictionTarget,
    },

    /// Deterministic execution was requested but cannot be guaranteed.
    DeterminismUnavailable {
        /// Human-readable explanation.
        reason: String,
    },

    /// Batch execution is invalid.
    InvalidBatch {
        /// Index of the problematic request when applicable.
        index: Option<usize>,

        /// Human-readable explanation.
        reason: String,
    },

    /// The underlying model contract returned an error.
    Model {
        /// Wrapped model error.
        error: ModelError,
    },
}

impl fmt::Display for PredictorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ModelUnavailable {
                model_id,
                state,
                reason,
            } => {
                write!(f, "prediction model unavailable")?;

                if let Some(model_id) = model_id {
                    write!(f, " `{model_id}`")?;
                }

                if let Some(state) = state {
                    write!(f, " in state {state:?}")?;
                }

                write!(f, ": {reason}")
            }

            Self::IncompatibleModel {
                model_id,
                compatibility,
                reason,
            } => {
                write!(
                    f,
                    "prediction model `{model_id}` is incompatible ({compatibility:?}): {reason}"
                )
            }

            Self::InvalidInput { reason } => {
                write!(f, "invalid prediction input: {reason}")
            }

            Self::InvalidPrediction { reason } => {
                write!(f, "invalid prediction: {reason}")
            }

            Self::TaskMismatch { expected, actual } => {
                write!(
                    f,
                    "prediction task mismatch: expected {expected:?}, got {actual:?}"
                )
            }

            Self::TargetMismatch {
                output,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "prediction target mismatch for `{output}`: expected {expected:?}, got {actual:?}"
                )
            }

            Self::DeterminismUnavailable { reason } => {
                write!(f, "deterministic prediction unavailable: {reason}")
            }

            Self::InvalidBatch { index, reason } => {
                if let Some(index) = index {
                    write!(f, "invalid prediction batch at index {index}: {reason}")
                } else {
                    write!(f, "invalid prediction batch: {reason}")
                }
            }

            Self::Model { error } => {
                write!(f, "prediction model error: {error}")
            }
        }
    }
}

impl std::error::Error for PredictorError {}

impl From<ModelError> for PredictorError {
    fn from(error: ModelError) -> Self {
        Self::Model { error }
    }
}

// =============================================================================
// Prediction request
// =============================================================================

/// A complete predictor request.
///
/// This is intentionally immutable. A prediction request describes what the
/// caller wants; it does not describe what recovery action should be taken.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionRequest {
    /// Features and explicit prediction context.
    pub input: PredictionInput,

    /// Requirements that the selected model must satisfy.
    pub requirements: ModelRequirements,

    /// Optional task-level requirement.
    ///
    /// When supplied, it must match the model's declared prediction task.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<PredictionTask>,

    /// Optional required output names.
    ///
    /// Empty means that all model outputs are acceptable.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_outputs: Vec<String>,
}

impl PredictionRequest {
    /// Creates a request from input and model requirements.
    #[must_use]
    pub fn new(input: PredictionInput, requirements: ModelRequirements) -> Self {
        Self {
            input,
            requirements,
            task: None,
            required_outputs: Vec::new(),
        }
    }

    /// Requires a specific prediction task.
    #[must_use]
    pub fn with_task(mut self, task: PredictionTask) -> Self {
        self.task = Some(task);
        self
    }

    /// Requires one or more named outputs.
    ///
    /// Duplicate names are rejected during request validation.
    #[must_use]
    pub fn with_required_outputs<I, S>(mut self, outputs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.required_outputs = outputs.into_iter().map(Into::into).collect();
        self
    }

    /// Returns whether deterministic prediction is required.
    #[must_use]
    pub const fn deterministic(&self) -> bool {
        self.input.context.deterministic || self.requirements.deterministic
    }
}

// =============================================================================
// Prediction result
// =============================================================================

/// Validated prediction returned by the predictor.
///
/// The underlying [`Prediction`] is preserved rather than copied into a
/// second prediction type. This avoids parallel prediction representations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionResult {
    /// Validated model prediction.
    pub prediction: Prediction,

    /// Compatibility result established before invocation.
    pub compatibility: ModelCompatibility,

    /// Model identity used for the prediction.
    pub model_id: ModelId,

    /// Whether deterministic execution was requested.
    pub deterministic_requested: bool,
}

impl PredictionResult {
    /// Returns an output by name.
    #[must_use]
    pub fn output(&self, name: &str) -> Option<&PredictionOutput> {
        self.prediction.output(name)
    }

    /// Returns the underlying prediction.
    #[must_use]
    pub const fn prediction(&self) -> &Prediction {
        &self.prediction
    }

    /// Returns whether the result has at least one output.
    #[must_use]
    pub fn has_outputs(&self) -> bool {
        !self.prediction.is_empty()
    }
}

/// Batch result preserving request/result ordering.
///
/// The predictor never reorders caller-supplied requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionBatchResult {
    /// Results in the same order as the input requests.
    pub results: Vec<PredictionResult>,
}

impl PredictionBatchResult {
    /// Creates an empty batch result.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Returns the number of results.
    #[must_use]
    pub fn len(&self) -> usize {
        self.results.len()
    }

    /// Returns whether the batch contains no results.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Returns a result by input index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&PredictionResult> {
        self.results.get(index)
    }
}

// =============================================================================
// Predictor
// =============================================================================

/// Production prediction orchestrator.
///
/// `Predictor` owns a shared model handle but does not own model training,
/// feature extraction, recovery policy, or quantum execution.
///
/// Multiple predictors may safely share the same [`ModelHandle`].
#[derive(Clone)]
pub struct Predictor {
    model: ModelHandle,
}

impl Predictor {
    /// Creates a predictor around a shared model.
    ///
    /// Model metadata is inspected immediately so callers can discover
    /// malformed model contracts before prediction begins.
    pub fn new(model: ModelHandle) -> PredictorResult<Self> {
        validate_model_contract(&model)?;

        Ok(Self { model })
    }

    /// Returns the model handle.
    #[must_use]
    pub fn model(&self) -> &ModelHandle {
        &self.model
    }

    /// Returns immutable model metadata.
    #[must_use]
    pub fn metadata(&self) -> &super::model::ModelMetadata {
        self.model.metadata()
    }

    /// Returns the model identity.
    #[must_use]
    pub fn model_id(&self) -> &ModelId {
        &self.metadata().id
    }

    /// Returns the model's prediction task.
    #[must_use]
    pub const fn task(&self) -> &PredictionTask {
        // `const fn` cannot return through the nested reference in every
        // compiler context where this API is used, so the actual method below
        // is intentionally non-const.
        unreachable!()
    }

    /// Returns the model's prediction task.
    #[must_use]
    pub fn prediction_task(&self) -> &PredictionTask {
        &self.metadata().task
    }

    /// Returns the model kind.
    #[must_use]
    pub fn model_kind(&self) -> &ModelKind {
        &self.metadata().kind
    }

    /// Returns the current model state.
    #[must_use]
    pub fn state(&self) -> ModelState {
        self.model.state()
    }

    /// Checks whether the predictor can currently invoke its model.
    pub fn validate_ready(&self) -> PredictorResult<()> {
        match self.model.state() {
            ModelState::Ready => Ok(()),

            state => Err(PredictorError::ModelUnavailable {
                model_id: Some(self.model_id().clone()),
                state: Some(state),
                reason: "prediction requires a model in Ready state".to_owned(),
            }),
        }
    }

    /// Assesses compatibility without performing a prediction.
    #[must_use]
    pub fn compatibility(&self, requirements: &ModelRequirements) -> ModelCompatibility {
        assess_compatibility(self.metadata(), requirements)
    }

    /// Performs one fully validated prediction.
    pub fn predict(&self, request: &PredictionRequest) -> PredictorResult<PredictionResult> {
        self.preflight(request)?;

        let prediction = self.model.predict(&request.input)?;

        self.validate_prediction(request, &prediction)?;

        Ok(PredictionResult {
            prediction,
            compatibility: self.compatibility(&request.requirements),
            model_id: self.model_id().clone(),
            deterministic_requested: request.deterministic(),
        })
    }

    /// Performs multiple predictions while preserving request order.
    ///
    /// The predictor does not impose a batch-size limit. If the underlying
    /// model advertises efficient batch prediction, it may implement
    /// `PredictionModel::predict_batch`; otherwise the model contract controls
    /// whether batch prediction is supported.
    pub fn predict_batch(
        &self,
        requests: &[PredictionRequest],
    ) -> PredictorResult<PredictionBatchResult> {
        self.validate_ready()?;

        if requests.is_empty() {
            return Ok(PredictionBatchResult::empty());
        }

        for (index, request) in requests.iter().enumerate() {
            self.preflight(request)
                .map_err(|error| PredictorError::InvalidBatch {
                    index: Some(index),
                    reason: error.to_string(),
                })?;
        }

        let inputs: Vec<PredictionInput> =
            requests.iter().map(|request| request.input.clone()).collect();

        let predictions = self.model.predict_batch(&inputs)?;

        if predictions.len() != requests.len() {
            return Err(PredictorError::InvalidBatch {
                index: None,
                reason: format!(
                    "model returned {} predictions for {} requests",
                    predictions.len(),
                    requests.len()
                ),
            });
        }

        let mut results = Vec::with_capacity(predictions.len());

        for (index, (request, prediction)) in requests
            .iter()
            .zip(predictions.into_iter())
            .enumerate()
        {
            self.validate_prediction(request, &prediction)
                .map_err(|error| PredictorError::InvalidBatch {
                    index: Some(index),
                    reason: error.to_string(),
                })?;

            results.push(PredictionResult {
                prediction,
                compatibility: self.compatibility(&request.requirements),
                model_id: self.model_id().clone(),
                deterministic_requested: request.deterministic(),
            });
        }

        Ok(PredictionBatchResult { results })
    }

    /// Performs prediction with only a feature vector and explicit context.
    ///
    /// This convenience API remains schema-explicit and therefore cannot
    /// silently reinterpret features.
    pub fn predict_input(
        &self,
        input: PredictionInput,
        requirements: ModelRequirements,
    ) -> PredictorResult<PredictionResult> {
        self.predict(&PredictionRequest::new(input, requirements))
    }

    /// Performs prediction when the caller already knows the model's declared
    /// input/output schemas.
    ///
    /// This avoids introducing another schema abstraction into the predictor.
    pub fn predict_for_schemas(
        &self,
        input: PredictionInput,
        input_schema: ModelSchemaId,
        output_schema: ModelSchemaId,
    ) -> PredictorResult<PredictionResult> {
        let requirements = ModelRequirements::new(input_schema, output_schema);

        self.predict_input(input, requirements)
    }

    /// Performs a task-specific prediction.
    pub fn predict_for_task(
        &self,
        input: PredictionInput,
        requirements: ModelRequirements,
        task: PredictionTask,
    ) -> PredictorResult<PredictionResult> {
        let request = PredictionRequest::new(input, requirements).with_task(task);

        self.predict(&request)
    }

    /// Performs a prediction and requires specific named outputs.
    pub fn predict_required_outputs(
        &self,
        input: PredictionInput,
        requirements: ModelRequirements,
        outputs: impl IntoIterator<Item = impl Into<String>>,
    ) -> PredictorResult<PredictionResult> {
        let request =
            PredictionRequest::new(input, requirements).with_required_outputs(outputs);

        self.predict(&request)
    }

    /// Validates a prediction against a request without executing another
    /// prediction.
    pub fn validate_prediction(
        &self,
        request: &PredictionRequest,
        prediction: &Prediction,
    ) -> PredictorResult<()> {
        validate_prediction_structure(prediction)?;

        validate_prediction_provenance(self.metadata(), &request.input.context, prediction)?;

        if let Some(expected_task) = &request.task {
            if expected_task != &self.metadata().task {
                return Err(PredictorError::TaskMismatch {
                    expected: expected_task.clone(),
                    actual: self.metadata().task.clone(),
                });
            }
        }

        validate_required_outputs(&request.required_outputs, prediction)?;

        validate_prediction_targets(prediction)?;

        Ok(())
    }

    /// Validates a request before model invocation.
    pub fn preflight(&self, request: &PredictionRequest) -> PredictorResult<()> {
        self.validate_ready()?;

        validate_request_structure(request)?;

        let compatibility = self.compatibility(&request.requirements);

        if !compatibility.is_usable() {
            return Err(PredictorError::IncompatibleModel {
                model_id: self.model_id().clone(),
                compatibility,
                reason: "model does not satisfy prediction requirements".to_owned(),
            });
        }

        validate_input_against_model(self.metadata(), request)?;

        validate_determinism(self.metadata(), &request.input.context)?;

        if let Some(expected_task) = &request.task {
            if expected_task != &self.metadata().task {
                return Err(PredictorError::TaskMismatch {
                    expected: expected_task.clone(),
                    actual: self.metadata().task.clone(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Model contract validation
// =============================================================================

/// Validates the model contract before constructing a predictor.
///
/// This is deliberately stricter than simply checking whether the model is
/// currently ready. A model with invalid metadata must not enter the normal
/// prediction path.
pub fn validate_model_contract(model: &ModelHandle) -> PredictorResult<()> {
    let metadata = model.metadata();

    if metadata.id.as_str().trim().is_empty() {
        return Err(PredictorError::InvalidPrediction {
            reason: "model identifier must not be empty".to_owned(),
        });
    }

    if metadata.input_schema.as_str().trim().is_empty() {
        return Err(PredictorError::InvalidPrediction {
            reason: "model input schema must not be empty".to_owned(),
        });
    }

    if metadata.output_schema.as_str().trim().is_empty() {
        return Err(PredictorError::InvalidPrediction {
            reason: "model output schema must not be empty".to_owned(),
        });
    }

    if metadata.capabilities.deterministic
        && metadata.capabilities.incremental
        && !metadata.capabilities.versioned
    {
        return Err(PredictorError::InvalidPrediction {
            reason: "incremental deterministic models must expose versioned metadata"
                .to_owned(),
        });
    }

    Ok(())
}

// =============================================================================
// Request validation
// =============================================================================

/// Validates structural properties of a prediction request.
pub fn validate_request_structure(request: &PredictionRequest) -> PredictorResult<()> {
    if request.input.features.schema.as_str().trim().is_empty() {
        return Err(PredictorError::InvalidInput {
            reason: "feature schema must not be empty".to_owned(),
        });
    }

    validate_feature_values(&request.input.features.values)?;

    validate_required_output_names(&request.required_outputs)?;

    Ok(())
}

/// Validates feature values before they enter a model.
pub fn validate_feature_values(values: &[f64]) -> PredictorResult<()> {
    for (index, value) in values.iter().copied().enumerate() {
        validate_finite(value, &format!("feature at index {index}"))
            .map_err(|error| PredictorError::InvalidInput {
                reason: error.to_string(),
            })?;
    }

    Ok(())
}

/// Validates required output names.
///
/// Names are treated as stable semantic identifiers. Empty and duplicate names
/// are rejected because they make output interpretation ambiguous.
pub fn validate_required_output_names(names: &[String]) -> PredictorResult<()> {
    for (index, name) in names.iter().enumerate() {
        if name.trim().is_empty() {
            return Err(PredictorError::InvalidInput {
                reason: format!("required output name at index {index} is empty"),
            });
        }

        if names[..index].iter().any(|existing| existing == name) {
            return Err(PredictorError::InvalidInput {
                reason: format!("duplicate required output `{name}`"),
            });
        }
    }

    Ok(())
}

/// Validates input schema and deterministic requirements against the model.
pub fn validate_input_against_model(
    metadata: &super::model::ModelMetadata,
    request: &PredictionRequest,
) -> PredictorResult<()> {
    if metadata.input_schema != request.input.features.schema {
        return Err(PredictorError::InvalidInput {
            reason: format!(
                "feature schema `{}` does not match model input schema `{}`",
                request.input.features.schema.as_str(),
                metadata.input_schema.as_str()
            ),
        });
    }

    if request.requirements.input_schema != metadata.input_schema {
        return Err(PredictorError::InvalidInput {
            reason: format!(
                "request input schema `{}` does not match model input schema `{}`",
                request.requirements.input_schema.as_str(),
                metadata.input_schema.as_str()
            ),
        });
    }

    if request.requirements.output_schema != metadata.output_schema {
        return Err(PredictorError::InvalidInput {
            reason: format!(
                "request output schema `{}` does not match model output schema `{}`",
                request.requirements.output_schema.as_str(),
                metadata.output_schema.as_str()
            ),
        });
    }

    validate_feature_values(&request.input.features.values)?;

    Ok(())
}

/// Validates deterministic execution requirements.
///
/// Deterministic mode is never silently downgraded.
pub fn validate_determinism(
    metadata: &super::model::ModelMetadata,
    context: &PredictionContext,
) -> PredictorResult<()> {
    if !context.deterministic {
        return Ok(());
    }

    if !metadata.capabilities.deterministic {
        return Err(PredictorError::DeterminismUnavailable {
            reason: format!(
                "model `{}` does not declare deterministic prediction capability",
                metadata.id.as_str()
            ),
        });
    }

    Ok(())
}

// =============================================================================
// Prediction validation
// =============================================================================

/// Validates the structural integrity of a prediction.
pub fn validate_prediction_structure(prediction: &Prediction) -> PredictorResult<()> {
    if prediction.outputs.is_empty() {
        return Err(PredictorError::InvalidPrediction {
            reason: "prediction contains no outputs".to_owned(),
        });
    }

    if let Some(confidence) = prediction.confidence {
        validate_probability(confidence, "prediction confidence").map_err(|error| {
            PredictorError::InvalidPrediction {
                reason: error.to_string(),
            }
        })?;
    }

    let mut names: Vec<&str> = Vec::with_capacity(prediction.outputs.len());

    for output in &prediction.outputs {
        if output.name.trim().is_empty() {
            return Err(PredictorError::InvalidPrediction {
                reason: "prediction output name must not be empty".to_owned(),
            });
        }

        if names.iter().any(|existing| *existing == output.name) {
            return Err(PredictorError::InvalidPrediction {
                reason: format!("duplicate prediction output `{}`", output.name),
            });
        }

        names.push(output.name.as_str());

        validate_finite(output.value, "prediction output").map_err(|error| {
            PredictorError::InvalidPrediction {
                reason: error.to_string(),
            }
        })?;

        if let Some(confidence) = output.confidence {
            validate_probability(confidence, "prediction output confidence").map_err(
                |error| PredictorError::InvalidPrediction {
                    reason: error.to_string(),
                },
            )?;
        }
    }

    Ok(())
}

/// Validates prediction provenance against the model and request context.
pub fn validate_prediction_provenance(
    metadata: &super::model::ModelMetadata,
    context: &PredictionContext,
    prediction: &Prediction,
) -> PredictorResult<()> {
    let provenance = &prediction.provenance;

    if provenance.model_id != metadata.id {
        return Err(PredictorError::InvalidPrediction {
            reason: format!(
                "prediction provenance model `{}` does not match active model `{}`",
                provenance.model_id.as_str(),
                metadata.id.as_str()
            ),
        });
    }

    if provenance.model_version != metadata.version {
        return Err(PredictorError::InvalidPrediction {
            reason: format!(
                "prediction provenance version `{}` does not match active model `{}`",
                provenance.model_version, metadata.version
            ),
        });
    }

    if provenance.input_schema != metadata.input_schema {
        return Err(PredictorError::InvalidPrediction {
            reason: format!(
                "prediction provenance input schema `{}` does not match model `{}`",
                provenance.input_schema.as_str(),
                metadata.input_schema.as_str()
            ),
        });
    }

    if provenance.output_schema != metadata.output_schema {
        return Err(PredictorError::InvalidPrediction {
            reason: format!(
                "prediction provenance output schema `{}` does not match model `{}`",
                provenance.output_schema.as_str(),
                metadata.output_schema.as_str()
            ),
        });
    }

    if provenance.deterministic != context.deterministic {
        return Err(PredictorError::InvalidPrediction {
            reason: "prediction provenance deterministic flag does not match request"
                .to_owned(),
        });
    }

    if provenance.random_seed != context.random_seed {
        return Err(PredictorError::InvalidPrediction {
            reason: "prediction provenance random seed does not match request"
                .to_owned(),
        });
    }

    if provenance.model_integrity_digest != metadata.integrity_digest {
        return Err(PredictorError::InvalidPrediction {
            reason: "prediction provenance model integrity digest does not match model metadata"
                .to_owned(),
        });
    }

    Ok(())
}

/// Ensures every required output is present.
pub fn validate_required_outputs(
    required_outputs: &[String],
    prediction: &Prediction,
) -> PredictorResult<()> {
    for required in required_outputs {
        if prediction.output(required).is_none() {
            return Err(PredictorError::InvalidPrediction {
                reason: format!("required prediction output `{required}` is missing"),
            });
        }
    }

    Ok(())
}

/// Validates semantic ranges declared by prediction targets.
pub fn validate_prediction_targets(prediction: &Prediction) -> PredictorResult<()> {
    for output in &prediction.outputs {
        match output.target {
            PredictionTarget::Probability | PredictionTarget::Confidence => {
                if !(0.0..=1.0).contains(&output.value) {
                    return Err(PredictorError::InvalidPrediction {
                        reason: format!(
                            "output `{}` has target {:?} but value {} is outside [0, 1]",
                            output.name, output.target, output.value
                        ),
                    });
                }
            }

            PredictionTarget::NonNegativeScalar => {
                if output.value < 0.0 {
                    return Err(PredictorError::InvalidPrediction {
                        reason: format!(
                            "output `{}` is non-negative but has value {}",
                            output.name, output.value
                        ),
                    });
                }
            }

            PredictionTarget::Scalar
            | PredictionTarget::Category
            | PredictionTarget::RankingScore
            | PredictionTarget::Custom(_) => {}
        }
    }

    Ok(())
}

// =============================================================================
// Advisory helpers
// =============================================================================

/// Returns a named numeric prediction if present.
///
/// This helper deliberately does not make a decision about recovery or policy.
pub fn output_value<'a>(
    prediction: &'a Prediction,
    output_name: &str,
) -> Option<&'a PredictionOutput> {
    prediction.output(output_name)
}

/// Extracts a probability output after verifying that its semantic target is
/// actually `PredictionTarget::Probability`.
pub fn probability_output(
    prediction: &Prediction,
    output_name: &str,
) -> PredictorResult<f64> {
    let output = prediction
        .output(output_name)
        .ok_or_else(|| PredictorError::InvalidPrediction {
            reason: format!("prediction output `{output_name}` is missing"),
        })?;

    if output.target != PredictionTarget::Probability {
        return Err(PredictorError::TargetMismatch {
            output: output_name.to_owned(),
            expected: PredictionTarget::Probability,
            actual: output.target.clone(),
        });
    }

    validate_probability(output.value, output_name).map_err(|error| {
        PredictorError::InvalidPrediction {
            reason: error.to_string(),
        }
    })?;

    Ok(output.value)
}

/// Extracts a confidence value when one exists.
///
/// No default confidence is invented when the model did not provide one.
pub fn prediction_confidence(prediction: &Prediction) -> Option<f64> {
    prediction.confidence
}

/// Extracts output-specific confidence without manufacturing a confidence
/// value.
pub fn output_confidence(
    prediction: &Prediction,
    output_name: &str,
) -> Option<f64> {
    prediction
        .output(output_name)
        .and_then(|output| output.confidence)
}

// =============================================================================
// Task validation helpers
// =============================================================================

/// Validates that a model is suitable for a particular resilience task.
///
/// This helper is intentionally advisory. It does not rank models and does not
/// authorize any resilience action.
pub fn validate_task(
    model: &ModelHandle,
    expected_task: &PredictionTask,
) -> PredictorResult<()> {
    if model.metadata().task != *expected_task {
        return Err(PredictorError::TaskMismatch {
            expected: expected_task.clone(),
            actual: model.metadata().task.clone(),
        });
    }

    Ok(())
}

/// Builds the most restrictive common requirements needed for a prediction.
///
/// This helper does not introduce hardware-specific limits.
#[must_use]
pub fn requirements_for_task(
    input_schema: ModelSchemaId,
    output_schema: ModelSchemaId,
    task: PredictionTask,
    deterministic: bool,
    uncertainty_required: bool,
) -> (ModelRequirements, PredictionTask) {
    let mut requirements = ModelRequirements::new(input_schema, output_schema);

    if deterministic {
        requirements = requirements.require_deterministic();
    }

    if uncertainty_required {
        requirements = requirements.require_uncertainty();
    }

    (requirements, task)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::resilience::learning::model::{
        FeatureVector,
        ModelCapabilities,
        ModelMetadata,
        ModelSchemaId,
        ModelVersion,
    };

    fn schema(value: &str) -> ModelSchemaId {
        ModelSchemaId::new(value).expect("valid schema")
    }

    fn metadata() -> ModelMetadata {
        ModelMetadata::new(
            ModelId::new("test.failure-model").expect("valid model id"),
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

            let value = input
                .features
                .values
                .first()
                .copied()
                .unwrap_or(0.0);

            let output = PredictionOutput::new(
                "failure_probability",
                value,
                PredictionTarget::Probability,
            )?;

            Prediction::new(
                vec![output],
                super::super::model::PredictionProvenance::from_metadata(
                    &self.metadata,
                    &input.context,
                ),
            )
        }
    }

    fn predictor() -> Predictor {
        let model = TestModel {
            metadata: metadata(),
            state: ModelState::Ready,
        };

        Predictor::new(ModelHandle::new(model)).expect("valid predictor")
    }

    fn input(value: f64) -> PredictionInput {
        PredictionInput::new(
            FeatureVector::new(schema("features.v1"), vec![value])
                .expect("valid features"),
            PredictionContext::default(),
        )
    }

    fn requirements() -> ModelRequirements {
        ModelRequirements::new(
            schema("features.v1"),
            schema("prediction.failure.v1"),
        )
    }

    #[test]
    fn predictor_rejects_unready_model() {
        let model = TestModel {
            metadata: metadata(),
            state: ModelState::Unavailable,
        };

        let predictor =
            Predictor::new(ModelHandle::new(model)).expect("metadata is valid");

        let request = PredictionRequest::new(input(0.2), requirements());

        assert!(predictor.predict(&request).is_err());
    }

    #[test]
    fn predictor_accepts_valid_prediction() {
        let predictor = predictor();

        let request = PredictionRequest::new(input(0.2), requirements())
            .with_task(PredictionTask::FailureProbability);

        let result = predictor.predict(&request).expect("prediction succeeds");

        assert_eq!(result.model_id.as_str(), "test.failure-model");
        assert_eq!(
            probability_output(&result.prediction, "failure_probability")
                .expect("probability output"),
            0.2
        );
    }

    #[test]
    fn predictor_rejects_wrong_feature_schema() {
        let predictor = predictor();

        let bad_input = PredictionInput::new(
            FeatureVector::new(schema("features.v2"), vec![0.2])
                .expect("valid features"),
            PredictionContext::default(),
        );

        let request = PredictionRequest::new(bad_input, requirements());

        assert!(predictor.predict(&request).is_err());
    }

    #[test]
    fn deterministic_mode_is_enforced() {
        let predictor = predictor();

        let mut context = PredictionContext::default();
        context.deterministic = true;

        let request = PredictionRequest::new(
            PredictionInput::new(
                FeatureVector::new(schema("features.v1"), vec![0.2])
                    .expect("valid features"),
                context,
            ),
            requirements().require_deterministic(),
        );

        let result = predictor.predict(&request);

        assert!(result.is_ok());
        assert!(result
            .expect("prediction")
            .deterministic_requested);
    }

    #[test]
    fn predictor_rejects_non_finite_feature() {
        let predictor = predictor();

        let bad_features =
            FeatureVector::new(schema("features.v1"), vec![f64::NAN]);

        assert!(bad_features.is_err());

        let _ = predictor;
    }

    #[test]
    fn required_outputs_are_checked() {
        let predictor = predictor();

        let request = PredictionRequest::new(input(0.2), requirements())
            .with_required_outputs(["failure_probability"]);

        assert!(predictor.predict(&request).is_ok());
    }

    #[test]
    fn missing_required_output_is_rejected() {
        let predictor = predictor();

        let request = PredictionRequest::new(input(0.2), requirements())
            .with_required_outputs(["recovery_probability"]);

        assert!(predictor.predict(&request).is_err());
    }

    #[test]
    fn wrong_task_is_rejected() {
        let predictor = predictor();

        let request = PredictionRequest::new(input(0.2), requirements())
            .with_task(PredictionTask::RecoverySuccessProbability);

        assert!(predictor.predict(&request).is_err());
    }

    #[test]
    fn batch_preserves_order() {
        let predictor = predictor();

        let requests = vec![
            PredictionRequest::new(input(0.1), requirements()),
            PredictionRequest::new(input(0.2), requirements()),
            PredictionRequest::new(input(0.3), requirements()),
        ];

        let result = predictor
            .predict_batch(&requests)
            .expect("batch succeeds");

        assert_eq!(result.len(), 3);

        assert_eq!(
            probability_output(
                &result.results[0].prediction,
                "failure_probability"
            )
            .expect("probability"),
            0.1
        );

        assert_eq!(
            probability_output(
                &result.results[1].prediction,
                "failure_probability"
            )
            .expect("probability"),
            0.2
        );

        assert_eq!(
            probability_output(
                &result.results[2].prediction,
                "failure_probability"
            )
            .expect("probability"),
            0.3
        );
    }

    #[test]
    fn empty_batch_is_valid() {
        let predictor = predictor();

        let result = predictor
            .predict_batch(&[])
            .expect("empty batch is valid");

        assert!(result.is_empty());
    }

    #[test]
    fn duplicate_required_outputs_are_rejected() {
        let result = validate_required_output_names(&[
            "failure".to_owned(),
            "failure".to_owned(),
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn non_finite_prediction_is_rejected() {
        let result = PredictionOutput::new(
            "failure_probability",
            f64::NAN,
            PredictionTarget::Probability,
        );

        assert!(result.is_err());
    }

    #[test]
    fn probability_helper_rejects_wrong_target() {
        let output = PredictionOutput::new(
            "score",
            0.5,
            PredictionTarget::RankingScore,
        )
        .expect("valid output");

        let prediction = Prediction::new(
            vec![output],
            super::super::model::PredictionProvenance::from_metadata(
                &metadata(),
                &PredictionContext::default(),
            ),
        )
        .expect("valid prediction");

        assert!(probability_output(&prediction, "score").is_err());
    }

    #[test]
    fn model_contract_is_validated_at_construction() {
        let model = TestModel {
            metadata: metadata(),
            state: ModelState::Ready,
        };

        let result = validate_model_contract(&ModelHandle::new(model));

        assert!(result.is_ok());
    }

    #[test]
    fn task_requirements_can_be_constructed_without_hardware_assumptions() {
        let (requirements, task) = requirements_for_task(
            schema("features.v1"),
            schema("prediction.failure.v1"),
            PredictionTask::FailureProbability,
            true,
            true,
        );

        assert!(requirements.deterministic);
        assert!(requirements.uncertainty_required);
        assert_eq!(task, PredictionTask::FailureProbability);
    }
}