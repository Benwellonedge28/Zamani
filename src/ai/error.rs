
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI — Structured Error Contract
//!
//! Path: src/ai/error.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Defines the structured error contract for Zamani's AI computational
//! domain.
//!
//! This module provides errors that can be propagated across:
//!
//! - AI model construction;
//! - model composition;
//! - AI operation registration;
//! - tensor interoperability;
//! - learning and training;
//! - inference;
//! - reasoning;
//! - knowledge operations;
//! - probabilistic computation;
//! - automatic differentiation;
//! - multi-agent computation;
//! - AI pipelines;
//! - AI optimization;
//! - datasets and preprocessing;
//! - model serialization and deployment;
//! - AI security and authorization;
//! - capability negotiation;
//! - resource feasibility;
//! - compiler integration;
//! - distributed execution;
//! - runtime integration.
//!
//! ============================================================================
//! AUTHORITY
//! ============================================================================
//!
//! Normative references:
//!
//! - grammar/DESIGN.md
//! - grammar/spec/ai.md
//! - grammar/spec/diagnostics.md
//! - grammar/ai/README.md
//! - grammar/ai/ai.g4
//! - src/frontend/semantic/domains/ai.rs
//! - src/frontend/semantic/api.rs
//! - src/zuir/errors.rs
//!
//! The grammar defines language syntax.
//! The frontend semantic subsystem defines source-level meaning.
//! ZUIR owns universal intermediate-representation errors.
//! This module owns AI-domain operational and integration failures.
//!
//! ============================================================================
//! OWNS
//! ============================================================================
//!
//! - AI-specific error categories;
//! - stable AI error codes;
//! - structured AI error context;
//! - underlying error chaining;
//! - retry classifications;
//! - execution outcome uncertainty;
//! - serializable error records;
//! - the AiResult<T> convenience alias.
//!
//! ============================================================================
//! DOES NOT OWN
//! ============================================================================
//!
//! - Source syntax diagnostics;
//! - AST construction errors;
//! - canonical semantic diagnostics;
//! - canonical source spans;
//! - canonical type-system errors;
//! - universal ZUIR errors;
//! - resource allocation;
//! - capability discovery;
//! - hardware selection;
//! - scheduling;
//! - routing;
//! - model execution;
//! - distributed transport;
//! - runtime recovery;
//! - security enforcement.
//!
//! ============================================================================
//! ERROR-CLASSIFICATION INVARIANTS
//! ============================================================================
//!
//! The following failures MUST remain distinguishable:
//!
//! 1. Invalid AI input.
//! 2. Invalid AI model structure.
//! 3. Unsupported AI operation.
//! 4. Missing execution capability.
//! 5. Unsatisfied resource requirement.
//! 6. Temporary resource exhaustion.
//! 7. Numerical computation failure.
//! 8. Training failure.
//! 9. Inference failure.
//! 10. Reasoning failure.
//! 11. Differentiation failure.
//! 12. Security and authorization failure.
//! 13. Serialization failure.
//! 14. Artifact incompatibility.
//! 15. Cancellation.
//! 16. Deadline expiration.
//! 17. Distributed execution failure.
//! 18. Internal integration failure.
//!
//! A target limitation MUST NOT automatically become a source-language
//! semantic error.
//!
//! A resource shortage MUST NOT silently change program meaning.
//!
//! Unsupported execution MUST NOT be reported as successful execution.
//!
//! An uncertain execution outcome MUST NOT be represented as definitely
//! successful or definitely unexecuted.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! This module introduces no universal ceilings for:
//!
//! - models;
//! - model parameters;
//! - tensor rank;
//! - tensor dimensions;
//! - operations;
//! - datasets;
//! - agents;
//! - training steps;
//! - pipeline stages;
//! - accelerators;
//! - workers;
//! - devices;
//! - memory;
//! - distributed nodes.
//!
//! Actual process-resource exhaustion and configured operational budgets
//! remain possible and must be reported explicitly.
//!
//! Literal infinite computation is not promised. The objective is to impose
//! no arbitrary language-level hardware ceiling.
//!
//! ============================================================================
//! DEPENDENCIES
//! ============================================================================
//!
//! Uses:
//!
//! - std::error::Error;
//! - std::collections::BTreeMap;
//! - serde.
//!
//! Does not depend on AI models, tensors, execution engines, registries,
//! hardware, runtime, or frontend implementation modules.
//!
//! This allows it to be implemented and frozen independently.
//!
//! ============================================================================
//! INTEGRATION
//! ============================================================================
//!
//! - model/* uses AiResult<T> for AI model-domain operations.
//! - tensor/* uses AiResult<T> for AI tensor-domain operations.
//! - learning/* uses AiResult<T> for learning failures.
//! - reasoning/* uses AiResult<T> for reasoning failures.
//! - differentiation/* uses AiResult<T> for differentiation failures.
//! - agents/* uses AiResult<T> for agent-domain failures.
//! - pipeline/* uses AiResult<T> for pipeline failures.
//! - optimization/* uses AiResult<T> for AI optimization failures.
//! - data/* uses AiResult<T> for AI data operations.
//! - deployment/* uses AiResult<T> for artifact operations.
//! - execution.rs translates AI errors at the runtime boundary.
//! - interoperability.rs translates errors between AI and other domains.
//!
//! Universal ZUIR errors may be retained as underlying sources using
//! AiError::with_source without introducing a dependency on ZUIR.
//!
//! The canonical frontend diagnostic system remains authoritative.
//!
//! ============================================================================
//! FREEZE CRITERIA
//! ============================================================================
//!
//! - Error categories are structured and extensible.
//! - Error codes are stable and machine-readable.
//! - Error context is deterministic.
//! - Underlying causes are preserved.
//! - Retry advice is explicit.
//! - Uncertain execution outcomes are represented.
//! - Serialization excludes opaque error objects.
//! - Target incompatibility differs from semantic invalidity.
//! - Resource exhaustion differs from resource infeasibility.
//! - No unsafe code or universal hardware ceiling is introduced.

use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error as StdError;

// ============================================================================
// RESULT ALIAS
// ============================================================================

/// Standard result type for fallible AI-domain operations.
///
/// Use this alias for errors owned by the AI subsystem.
///
/// Other subsystems retain their own error types and translate them at
/// explicitly documented integration boundaries.
pub type AiResult<T> = Result<T, AiError>;

// ============================================================================
// ERROR CATEGORIES
// ============================================================================

/// Broad category of an AI-domain failure.
///
/// This enumeration is non-exhaustive so new AI computational paradigms
/// can introduce additional error categories without breaking consumers.
///
/// Consumers should branch on categories and stable error codes rather
/// than parsing human-readable messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum AiErrorKind {
    /// Invalid argument or malformed AI-domain input.
    InvalidInput,

    /// Invalid model definition or structure.
    InvalidModel,

    /// Invalid model graph or dependency relationship.
    InvalidModelGraph,

    /// Invalid AI operation definition.
    InvalidOperation,

    /// Unsupported AI operation semantics.
    UnsupportedOperation,

    /// Missing or unverified computational capability.
    CapabilityUnavailable,

    /// A declared computational requirement cannot be satisfied.
    RequirementUnsatisfied,

    /// A resource requirement is infeasible for the selected realization.
    ResourceInfeasible,

    /// An operational resource or execution budget was exhausted.
    ResourceExhausted,

    /// Tensor shape or dimensional compatibility failure.
    ShapeIncompatible,

    /// Tensor data type or numerical representation incompatibility.
    DataTypeIncompatible,

    /// Invalid tensor layout or representation.
    LayoutIncompatible,

    /// Numerical computation failed.
    NumericalFailure,

    /// Numerical precision or stability requirements were violated.
    NumericalInstability,

    /// Learning or training operation failed.
    TrainingFailed,

    /// Model inference failed.
    InferenceFailed,

    /// Model evaluation failed.
    EvaluationFailed,

    /// Learning algorithm or optimizer failed.
    OptimizationFailed,

    /// Reasoning operation failed.
    ReasoningFailed,

    /// Knowledge representation or knowledge operation failed.
    KnowledgeFailure,

    /// Probabilistic computation failed.
    ProbabilisticFailure,

    /// Uncertainty evaluation or propagation failed.
    UncertaintyFailure,

    /// Automatic differentiation failed.
    DifferentiationFailed,

    /// Requested operation is not differentiable under its contract.
    NonDifferentiableOperation,

    /// AI pipeline is structurally or operationally invalid.
    PipelineFailure,

    /// AI agent operation failed.
    AgentFailure,

    /// Agent coordination or communication failed.
    AgentCoordinationFailure,

    /// Dataset or data source operation failed.
    DataFailure,

    /// Dataset schema or integrity validation failed.
    DataValidationFailure,

    /// Model or artifact serialization failed.
    SerializationFailure,

    /// Model or artifact deserialization failed.
    DeserializationFailure,

    /// Model or artifact version is incompatible.
    VersionIncompatible,

    /// Artifact integrity verification failed.
    IntegrityFailure,

    /// Security requirements were not satisfied.
    SecurityViolation,

    /// Authentication failed.
    AuthenticationFailed,

    /// Authorization was denied.
    AuthorizationDenied,

    /// A requested execution was cancelled.
    Cancelled,

    /// An execution deadline expired.
    DeadlineExceeded,

    /// An execution target cannot preserve the required semantics.
    TargetIncompatible,

    /// A distributed AI operation failed.
    DistributedExecutionFailed,

    /// An external integration or provider failed.
    IntegrationFailure,

    /// A requested AI entity was not found.
    NotFound,

    /// An operation conflicts with existing state.
    Conflict,

    /// An internal AI subsystem invariant failed.
    InternalFailure,
}

impl AiErrorKind {
    /// Returns the stable default error code for this category.
    ///
    /// Once released, a code's meaning MUST NOT be silently changed.
    #[must_use]
    pub const fn default_code(self) -> &'static str {
        match self {
            Self::InvalidInput => "ZMN-AI-INVALID-INPUT",
            Self::InvalidModel => "ZMN-AI-INVALID-MODEL",
            Self::InvalidModelGraph => "ZMN-AI-INVALID-MODEL-GRAPH",
            Self::InvalidOperation => "ZMN-AI-INVALID-OPERATION",
            Self::UnsupportedOperation => "ZMN-AI-UNSUPPORTED-OPERATION",
            Self::CapabilityUnavailable => "ZMN-AI-CAPABILITY-UNAVAILABLE",
            Self::RequirementUnsatisfied => "ZMN-AI-REQUIREMENT-UNSATISFIED",
            Self::ResourceInfeasible => "ZMN-AI-RESOURCE-INFEASIBLE",
            Self::ResourceExhausted => "ZMN-AI-RESOURCE-EXHAUSTED",
            Self::ShapeIncompatible => "ZMN-AI-SHAPE-INCOMPATIBLE",
            Self::DataTypeIncompatible => "ZMN-AI-DATATYPE-INCOMPATIBLE",
            Self::LayoutIncompatible => "ZMN-AI-LAYOUT-INCOMPATIBLE",
            Self::NumericalFailure => "ZMN-AI-NUMERICAL-FAILURE",
            Self::NumericalInstability => "ZMN-AI-NUMERICAL-INSTABILITY",
            Self::TrainingFailed => "ZMN-AI-TRAINING-FAILED",
            Self::InferenceFailed => "ZMN-AI-INFERENCE-FAILED",
            Self::EvaluationFailed => "ZMN-AI-EVALUATION-FAILED",
            Self::OptimizationFailed => "ZMN-AI-OPTIMIZATION-FAILED",
            Self::ReasoningFailed => "ZMN-AI-REASONING-FAILED",
            Self::KnowledgeFailure => "ZMN-AI-KNOWLEDGE-FAILURE",
            Self::ProbabilisticFailure => "ZMN-AI-PROBABILISTIC-FAILURE",
            Self::UncertaintyFailure => "ZMN-AI-UNCERTAINTY-FAILURE",
            Self::DifferentiationFailed => "ZMN-AI-DIFFERENTIATION-FAILED",
            Self::NonDifferentiableOperation => "ZMN-AI-NON-DIFFERENTIABLE",
            Self::PipelineFailure => "ZMN-AI-PIPELINE-FAILURE",
            Self::AgentFailure => "ZMN-AI-AGENT-FAILURE",
            Self::AgentCoordinationFailure => "ZMN-AI-AGENT-COORDINATION-FAILED",
            Self::DataFailure => "ZMN-AI-DATA-FAILURE",
            Self::DataValidationFailure => "ZMN-AI-DATA-VALIDATION-FAILED",
            Self::SerializationFailure => "ZMN-AI-SERIALIZATION-FAILED",
            Self::DeserializationFailure => "ZMN-AI-DESERIALIZATION-FAILED",
            Self::VersionIncompatible => "ZMN-AI-VERSION-INCOMPATIBLE",
            Self::IntegrityFailure => "ZMN-AI-INTEGRITY-FAILED",
            Self::SecurityViolation => "ZMN-AI-SECURITY-VIOLATION",
            Self::AuthenticationFailed => "ZMN-AI-AUTHENTICATION-FAILED",
            Self::AuthorizationDenied => "ZMN-AI-AUTHORIZATION-DENIED",
            Self::Cancelled => "ZMN-AI-CANCELLED",
            Self::DeadlineExceeded => "ZMN-AI-DEADLINE-EXCEEDED",
            Self::TargetIncompatible => "ZMN-AI-TARGET-INCOMPATIBLE",
            Self::DistributedExecutionFailed => "ZMN-AI-DISTRIBUTED-EXECUTION-FAILED",
            Self::IntegrationFailure => "ZMN-AI-INTEGRATION-FAILED",
            Self::NotFound => "ZMN-AI-NOT-FOUND",
            Self::Conflict => "ZMN-AI-CONFLICT",
            Self::InternalFailure => "ZMN-AI-INTERNAL-FAILURE",
        }
    }

    /// Whether this category represents invalid AI-domain input or structure.
    ///
    /// This does not establish that a source program is semantically invalid.
    #[must_use]
    pub const fn is_domain_validation_failure(self) -> bool {
        matches!(
            self,
            Self::InvalidInput
                | Self::InvalidModel
                | Self::InvalidModelGraph
                | Self::InvalidOperation
                | Self::ShapeIncompatible
                | Self::DataTypeIncompatible
                | Self::LayoutIncompatible
                | Self::DataValidationFailure
                | Self::Conflict
        )
    }

    /// Whether this category represents a compatibility or realization
    /// problem rather than necessarily invalid source semantics.
    #[must_use]
    pub const fn is_compatibility_failure(self) -> bool {
        matches!(
            self,
            Self::UnsupportedOperation
                | Self::CapabilityUnavailable
                | Self::RequirementUnsatisfied
                | Self::ResourceInfeasible
                | Self::VersionIncompatible
                | Self::TargetIncompatible
        )
    }

    /// Whether this category commonly indicates an operational failure.
    #[must_use]
    pub const fn is_operational_failure(self) -> bool {
        matches!(
            self,
            Self::ResourceExhausted
                | Self::TrainingFailed
                | Self::InferenceFailed
                | Self::EvaluationFailed
                | Self::OptimizationFailed
                | Self::NumericalFailure
                | Self::DistributedExecutionFailed
                | Self::Cancelled
                | Self::DeadlineExceeded
        )
    }
}

impl fmt::Display for AiErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidInput => "invalid input",
            Self::InvalidModel => "invalid model",
            Self::InvalidModelGraph => "invalid model graph",
            Self::InvalidOperation => "invalid operation",
            Self::UnsupportedOperation => "unsupported operation",
            Self::CapabilityUnavailable => "capability unavailable",
            Self::RequirementUnsatisfied => "requirement unsatisfied",
            Self::ResourceInfeasible => "resource infeasible",
            Self::ResourceExhausted => "resource exhausted",
            Self::ShapeIncompatible => "shape incompatible",
            Self::DataTypeIncompatible => "data type incompatible",
            Self::LayoutIncompatible => "layout incompatible",
            Self::NumericalFailure => "numerical failure",
            Self::NumericalInstability => "numerical instability",
            Self::TrainingFailed => "training failed",
            Self::InferenceFailed => "inference failed",
            Self::EvaluationFailed => "evaluation failed",
            Self::OptimizationFailed => "optimization failed",
            Self::ReasoningFailed => "reasoning failed",
            Self::KnowledgeFailure => "knowledge failure",
            Self::ProbabilisticFailure => "probabilistic failure",
            Self::UncertaintyFailure => "uncertainty failure",
            Self::DifferentiationFailed => "differentiation failed",
            Self::NonDifferentiableOperation => "non-differentiable operation",
            Self::PipelineFailure => "pipeline failure",
            Self::AgentFailure => "agent failure",
            Self::AgentCoordinationFailure => "agent coordination failure",
            Self::DataFailure => "data failure",
            Self::DataValidationFailure => "data validation failure",
            Self::SerializationFailure => "serialization failure",
            Self::DeserializationFailure => "deserialization failure",
            Self::VersionIncompatible => "version incompatible",
            Self::IntegrityFailure => "integrity failure",
            Self::SecurityViolation => "security violation",
            Self::AuthenticationFailed => "authentication failed",
            Self::AuthorizationDenied => "authorization denied",
            Self::Cancelled => "cancelled",
            Self::DeadlineExceeded => "deadline exceeded",
            Self::TargetIncompatible => "target incompatible",
            Self::DistributedExecutionFailed => "distributed execution failed",
            Self::IntegrationFailure => "integration failure",
            Self::NotFound => "not found",
            Self::Conflict => "conflict",
            Self::InternalFailure => "internal failure",
        })
    }
}

// ============================================================================
// STABLE ERROR CODES
// ============================================================================

/// A stable, machine-readable AI error code.
///
/// Standard AI codes use:
///
///     ZMN-AI-...
///
/// Registered extension codes may use:
///
///     ZMN-EXT-...
///
/// Syntax validation does not establish extension registration.
///
/// Consumers must use codes rather than parsing human-readable messages.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AiErrorCode(String);

/// Invalid AI error code syntax.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvalidAiErrorCode;

impl fmt::Display for InvalidAiErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            "AI error code must use ZMN-AI- or ZMN-EXT- followed by nonempty uppercase ASCII segments",
        )
    }
}

impl StdError for InvalidAiErrorCode {}

impl AiErrorCode {
    /// Construct and validate an AI or extension error code.
    pub fn new(code: impl Into<String>) -> Result<Self, InvalidAiErrorCode> {
        let code = code.into();

        let valid_namespace =
            code.starts_with("ZMN-AI-") || code.starts_with("ZMN-EXT-");

        let valid_segments = code
            .split('-')
            .count() >= 4
            && code.split('-').all(|segment| {
                !segment.is_empty()
                    && segment
                        .bytes()
                        .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
            });

        if valid_namespace && valid_segments {
            Ok(Self(code))
        } else {
            Err(InvalidAiErrorCode)
        }
    }

    /// Return the underlying stable code.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_standard(code: &'static str) -> Self {
        Self(code.to_owned())
    }
}

impl fmt::Display for AiErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// ============================================================================
// RETRY CLASSIFICATION
// ============================================================================

/// Retry guidance associated with an AI operation failure.
///
/// Retry guidance is advisory. The responsible execution layer must still
/// consider idempotency, side effects, authorization, state and delivery
/// guarantees before retrying.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum RetryDisposition {
    /// Retrying without changing conditions is not appropriate.
    Never,

    /// Retry only after the relevant state or configuration changes.
    AfterConditionChange,

    /// Retry may be appropriate after backoff and revalidation.
    RetryWithBackoff,

    /// The operation is explicitly known to be safe to retry.
    SafeToRetry,

    /// The responsible layer has not established retry safety.
    Unknown,
}

// ============================================================================
// EXECUTION OUTCOME
// ============================================================================

/// What is known about an operation when an error is reported.
///
/// This distinction is important for distributed execution, training,
/// model updates, external tools and other operations with side effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum OperationOutcome {
    /// Execution is known not to have started.
    NotStarted,

    /// Execution completed successfully before the reported failure.
    Completed,

    /// Execution started but is known not to have committed its effects.
    NotCommitted,

    /// It is unknown whether execution or its effects were committed.
    Unknown,
}

// ============================================================================
// ERROR CONTEXT
// ============================================================================

/// AI-specific processing phase.
///
/// These values identify the AI-domain boundary where the failure occurred.
/// They do not replace compiler diagnostic phases or runtime tracing spans.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum AiErrorPhase {
    Construction,
    Registration,
    Validation,
    Composition,
    TensorProcessing,
    Training,
    Inference,
    Evaluation,
    Reasoning,
    Differentiation,
    Optimization,
    DataProcessing,
    Pipeline,
    AgentCoordination,
    Serialization,
    Deserialization,
    Deployment,
    Security,
    CapabilityNegotiation,
    ResourceAnalysis,
    Compilation,
    Lowering,
    Execution,
    DistributedExecution,
    Recovery,
    Interoperability,
}

/// Structured contextual information associated with an AI error.
///
/// Entity identifiers are opaque references supplied by the owning subsystem.
/// This module does not create a competing model-ID, operation-ID or
/// artifact-ID authority.
///
/// Source spans remain owned by the canonical frontend source-map and
/// diagnostic infrastructure.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiErrorContext {
    phase: Option<AiErrorPhase>,

    model_id: Option<String>,

    operation_id: Option<String>,

    agent_id: Option<String>,

    dataset_id: Option<String>,

    pipeline_id: Option<String>,

    artifact_reference: Option<String>,

    target_reference: Option<String>,

    retry: Option<RetryDisposition>,

    outcome: Option<OperationOutcome>,

    details: BTreeMap<String, String>,
}

impl AiErrorContext {
    /// Create empty error context.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the processing phase.
    #[must_use]
    pub const fn phase(&self) -> Option<AiErrorPhase> {
        self.phase
    }

    /// Return the model reference.
    #[must_use]
    pub fn model_id(&self) -> Option<&str> {
        self.model_id.as_deref()
    }

    /// Return the operation reference.
    #[must_use]
    pub fn operation_id(&self) -> Option<&str> {
        self.operation_id.as_deref()
    }

    /// Return the agent reference.
    #[must_use]
    pub fn agent_id(&self) -> Option<&str> {
        self.agent_id.as_deref()
    }

    /// Return the dataset reference.
    #[must_use]
    pub fn dataset_id(&self) -> Option<&str> {
        self.dataset_id.as_deref()
    }

    /// Return the pipeline reference.
    #[must_use]
    pub fn pipeline_id(&self) -> Option<&str> {
        self.pipeline_id.as_deref()
    }

    /// Return the artifact reference.
    #[must_use]
    pub fn artifact_reference(&self) -> Option<&str> {
        self.artifact_reference.as_deref()
    }

    /// Return the target reference.
    #[must_use]
    pub fn target_reference(&self) -> Option<&str> {
        self.target_reference.as_deref()
    }

    /// Return retry guidance.
    #[must_use]
    pub const fn retry(&self) -> Option<RetryDisposition> {
        self.retry
    }

    /// Return the known execution outcome.
    #[must_use]
    pub const fn outcome(&self) -> Option<OperationOutcome> {
        self.outcome
    }

    /// Return deterministic additional contextual details.
    #[must_use]
    pub fn details(&self) -> &BTreeMap<String, String> {
        &self.details
    }

    /// Set the processing phase.
    #[must_use]
    pub fn with_phase(mut self, phase: AiErrorPhase) -> Self {
        self.phase = Some(phase);
        self
    }

    /// Associate a model reference.
    #[must_use]
    pub fn with_model(mut self, id: impl Into<String>) -> Self {
        self.model_id = Some(id.into());
        self
    }

    /// Associate an operation reference.
    #[must_use]
    pub fn with_operation(mut self, id: impl Into<String>) -> Self {
        self.operation_id = Some(id.into());
        self
    }

    /// Associate an agent reference.
    #[must_use]
    pub fn with_agent(mut self, id: impl Into<String>) -> Self {
        self.agent_id = Some(id.into());
        self
    }

    /// Associate a dataset reference.
    #[must_use]
    pub fn with_dataset(mut self, id: impl Into<String>) -> Self {
        self.dataset_id = Some(id.into());
        self
    }

    /// Associate a pipeline reference.
    #[must_use]
    pub fn with_pipeline(mut self, id: impl Into<String>) -> Self {
        self.pipeline_id = Some(id.into());
        self
    }

    /// Associate an artifact reference.
    #[must_use]
    pub fn with_artifact(mut self, reference: impl Into<String>) -> Self {
        self.artifact_reference = Some(reference.into());
        self
    }

    /// Associate a target reference.
    ///
    /// The reference may identify a target or capability provider. This
    /// context does not select or discover hardware.
    #[must_use]
    pub fn with_target(mut self, reference: impl Into<String>) -> Self {
        self.target_reference = Some(reference.into());
        self
    }

    /// Explicitly state retry guidance.
    #[must_use]
    pub fn with_retry(mut self, retry: RetryDisposition) -> Self {
        self.retry = Some(retry);
        self
    }

    /// Explicitly state the known operation outcome.
    #[must_use]
    pub fn with_outcome(mut self, outcome: OperationOutcome) -> Self {
        self.outcome = Some(outcome);
        self
    }

    /// Add structured contextual information.
    ///
    /// Reusing a key replaces its previous value. BTreeMap provides
    /// deterministic iteration and serialization ordering.
    #[must_use]
    pub fn with_detail(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.details.insert(key.into(), value.into());
        self
    }
}

// ============================================================================
// SERIALIZABLE ERROR RECORD
// ============================================================================

/// Serializable representation of an AI error.
///
/// The underlying Rust error object is deliberately excluded because
/// arbitrary standard errors cannot be reliably or safely serialized.
///
/// The record preserves the stable category, code, message and context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiErrorRecord {
    pub kind: AiErrorKind,
    pub code: AiErrorCode,
    pub message: String,
    pub context: AiErrorContext,
}

impl AiErrorRecord {
    /// Return whether the record represents a target compatibility problem.
    #[must_use]
    pub const fn is_compatibility_failure(&self) -> bool {
        self.kind.is_compatibility_failure()
    }
}

// ============================================================================
// CANONICAL AI ERROR
// ============================================================================

/// A structured AI-domain failure.
///
/// The error preserves:
///
/// - broad error category;
/// - stable error code;
/// - explanatory message;
/// - contextual references;
/// - retry classification;
/// - execution outcome;
/// - underlying standard error.
///
/// This type deliberately does not implement Clone because its underlying
/// error chain may contain non-cloneable errors.
///
/// Use `record()` when a serializable representation is required.
#[derive(Debug)]
pub struct AiError {
    kind: AiErrorKind,

    code: AiErrorCode,

    message: String,

    context: AiErrorContext,

    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl AiError {
    /// Construct an error using the standard code for its category.
    #[must_use]
    pub fn new(
        kind: AiErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            code: AiErrorCode::from_standard(kind.default_code()),
            message: message.into(),
            context: AiErrorContext::new(),
            source: None,
        }
    }

    /// Construct an error using a validated custom error code.
    ///
    /// Extension codes must still be registered and approved by the
    /// repository's extension governance before being considered stable.
    pub fn with_code(
        kind: AiErrorKind,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<Self, InvalidAiErrorCode> {
        Ok(Self {
            kind,
            code: AiErrorCode::new(code)?,
            message: message.into(),
            context: AiErrorContext::new(),
            source: None,
        })
    }

    /// Construct an AI error while preserving an underlying cause.
    pub fn from_source<E>(
        kind: AiErrorKind,
        message: impl Into<String>,
        source: E,
    ) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::new(kind, message).with_source(source)
    }

    /// Return the broad error category.
    #[must_use]
    pub const fn kind(&self) -> AiErrorKind {
        self.kind
    }

    /// Return the stable machine-readable error code.
    #[must_use]
    pub fn code(&self) -> &AiErrorCode {
        &self.code
    }

    /// Return the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the structured error context.
    #[must_use]
    pub fn context(&self) -> &AiErrorContext {
        &self.context
    }

    /// Return the underlying source error.
    #[must_use]
    pub fn source_error(
        &self,
    ) -> Option<&(dyn StdError + Send + Sync + 'static)> {
        self.source.as_deref()
    }

    /// Return the error as a serializable record.
    ///
    /// The underlying error chain is intentionally not copied.
    #[must_use]
    pub fn record(&self) -> AiErrorRecord {
        AiErrorRecord {
            kind: self.kind,
            code: self.code.clone(),
            message: self.message.clone(),
            context: self.context.clone(),
        }
    }

    /// Replace the current structured context.
    #[must_use]
    pub fn with_context(mut self, context: AiErrorContext) -> Self {
        self.context = context;
        self
    }

    /// Associate a processing phase.
    #[must_use]
    pub fn with_phase(mut self, phase: AiErrorPhase) -> Self {
        self.context.phase = Some(phase);
        self
    }

    /// Associate a model reference.
    #[must_use]
    pub fn with_model(mut self, id: impl Into<String>) -> Self {
        self.context.model_id = Some(id.into());
        self
    }

    /// Associate an operation reference.
    #[must_use]
    pub fn with_operation(mut self, id: impl Into<String>) -> Self {
        self.context.operation_id = Some(id.into());
        self
    }

    /// Associate an agent reference.
    #[must_use]
    pub fn with_agent(mut self, id: impl Into<String>) -> Self {
        self.context.agent_id = Some(id.into());
        self
    }

    /// Associate a dataset reference.
    #[must_use]
    pub fn with_dataset(mut self, id: impl Into<String>) -> Self {
        self.context.dataset_id = Some(id.into());
        self
    }

    /// Associate a pipeline reference.
    #[must_use]
    pub fn with_pipeline(mut self, id: impl Into<String>) -> Self {
        self.context.pipeline_id = Some(id.into());
        self
    }

    /// Associate an artifact reference.
    #[must_use]
    pub fn with_artifact(mut self, reference: impl Into<String>) -> Self {
        self.context.artifact_reference = Some(reference.into());
        self
    }

    /// Associate a target reference.
    #[must_use]
    pub fn with_target(mut self, reference: impl Into<String>) -> Self {
        self.context.target_reference = Some(reference.into());
        self
    }

    /// Specify retry guidance.
    #[must_use]
    pub fn with_retry(mut self, retry: RetryDisposition) -> Self {
        self.context.retry = Some(retry);
        self
    }

    /// Specify the known execution outcome.
    #[must_use]
    pub fn with_outcome(mut self, outcome: OperationOutcome) -> Self {
        self.context.outcome = Some(outcome);
        self
    }

    /// Add deterministic structured contextual information.
    #[must_use]
    pub fn with_detail(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.context.details.insert(key.into(), value.into());
        self
    }

    /// Attach an underlying standard Rust error.
    ///
    /// This preserves the original error for diagnostics, logging and
    /// integration without flattening it into an unstructured string.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Convert the error into a serializable record.
    ///
    /// This method is useful for distributed transport and artifact
    /// diagnostics where the original Rust error object cannot cross
    /// process or machine boundaries.
    #[must_use]
    pub fn into_record(self) -> AiErrorRecord {
        AiErrorRecord {
            kind: self.kind,
            code: self.code,
            message: self.message,
            context: self.context,
        }
    }
}

impl fmt::Display for AiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}",
            self.code,
            self.message
        )?;

        if let Some(phase) = self.context.phase {
            write!(f, " (phase: {phase:?}")?;

            if let Some(model) = &self.context.model_id {
                write!(f, ", model: {model}")?;
            }

            if let Some(operation) = &self.context.operation_id {
                write!(f, ", operation: {operation}")?;
            }

            f.write_str(")")?;
        } else {
            if let Some(model) = &self.context.model_id {
                write!(f, " (model: {model}")?;

                if let Some(operation) = &self.context.operation_id {
                    write!(f, ", operation: {operation}")?;
                }

                f.write_str(")")?;
            } else if let Some(operation) = &self.context.operation_id {
                write!(f, " (operation: {operation})")?;
            }
        }

        Ok(())
    }
}

impl StdError for AiError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn StdError + 'static))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;

    #[test]
    fn standard_error_codes_are_stable() {
        let error = AiError::new(
            AiErrorKind::TrainingFailed,
            "training operation failed",
        );

        assert_eq!(
            error.code().as_str(),
            "ZMN-AI-TRAINING-FAILED"
        );

        assert_eq!(
            error.kind(),
            AiErrorKind::TrainingFailed
        );

        assert!(error.to_string().contains(
            "ZMN-AI-TRAINING-FAILED"
        ));
    }

    #[test]
    fn all_standard_error_codes_have_valid_syntax() {
        let kinds = [
            AiErrorKind::InvalidInput,
            AiErrorKind::InvalidModel,
            AiErrorKind::InvalidModelGraph,
            AiErrorKind::InvalidOperation,
            AiErrorKind::UnsupportedOperation,
            AiErrorKind::CapabilityUnavailable,
            AiErrorKind::RequirementUnsatisfied,
            AiErrorKind::ResourceInfeasible,
            AiErrorKind::ResourceExhausted,
            AiErrorKind::ShapeIncompatible,
            AiErrorKind::DataTypeIncompatible,
            AiErrorKind::LayoutIncompatible,
            AiErrorKind::NumericalFailure,
            AiErrorKind::NumericalInstability,
            AiErrorKind::TrainingFailed,
            AiErrorKind::InferenceFailed,
            AiErrorKind::EvaluationFailed,
            AiErrorKind::OptimizationFailed,
            AiErrorKind::ReasoningFailed,
            AiErrorKind::KnowledgeFailure,
            AiErrorKind::ProbabilisticFailure,
            AiErrorKind::UncertaintyFailure,
            AiErrorKind::DifferentiationFailed,
            AiErrorKind::NonDifferentiableOperation,
            AiErrorKind::PipelineFailure,
            AiErrorKind::AgentFailure,
            AiErrorKind::AgentCoordinationFailure,
            AiErrorKind::DataFailure,
            AiErrorKind::DataValidationFailure,
            AiErrorKind::SerializationFailure,
            AiErrorKind::DeserializationFailure,
            AiErrorKind::VersionIncompatible,
            AiErrorKind::IntegrityFailure,
            AiErrorKind::SecurityViolation,
            AiErrorKind::AuthenticationFailed,
            AiErrorKind::AuthorizationDenied,
            AiErrorKind::Cancelled,
            AiErrorKind::DeadlineExceeded,
            AiErrorKind::TargetIncompatible,
            AiErrorKind::DistributedExecutionFailed,
            AiErrorKind::IntegrationFailure,
            AiErrorKind::NotFound,
            AiErrorKind::Conflict,
            AiErrorKind::InternalFailure,
        ];

        for kind in kinds {
            assert!(
                AiErrorCode::new(kind.default_code()).is_ok(),
                "invalid standard code: {}",
                kind.default_code()
            );
        }
    }

    #[test]
    fn custom_codes_are_validated() {
        assert!(
            AiErrorCode::new("ZMN-AI-CUSTOM-FAILURE").is_ok()
        );

        assert!(
            AiErrorCode::new("ZMN-EXT-EXAMPLE-AI-FAILURE").is_ok()
        );

        assert!(AiErrorCode::new("invalid-code").is_err());
        assert!(AiErrorCode::new("ZMN-AI-").is_err());
        assert!(AiErrorCode::new("ZMN-AI-invalid").is_err());
        assert!(AiErrorCode::new("ZMN-AI-EMPTY-").is_err());
        assert!(AiErrorCode::new("ZMN-IR-INVALID-TYPE").is_err());
    }

    #[test]
    fn custom_error_code_can_be_used() {
        let error = AiError::with_code(
            AiErrorKind::IntegrationFailure,
            "ZMN-EXT-EXAMPLE-AI-PROVIDER-FAILED",
            "an external AI provider failed",
        )
        .expect("valid custom code");

        assert_eq!(
            error.code().as_str(),
            "ZMN-EXT-EXAMPLE-AI-PROVIDER-FAILED"
        );
    }

    #[test]
    fn error_context_preserves_domain_references() {
        let error = AiError::new(
            AiErrorKind::InferenceFailed,
            "inference failed",
        )
        .with_phase(AiErrorPhase::Inference)
        .with_model("model-reference")
        .with_operation("operation-reference")
        .with_detail("provider", "registered-provider");

        assert_eq!(
            error.context().phase(),
            Some(AiErrorPhase::Inference)
        );

        assert_eq!(
            error.context().model_id(),
            Some("model-reference")
        );

        assert_eq!(
            error.context().operation_id(),
            Some("operation-reference")
        );

        assert_eq!(
            error.context().details().get("provider"),
            Some(&"registered-provider".to_owned())
        );
    }

    #[test]
    fn execution_outcome_preserves_uncertainty() {
        let error = AiError::new(
            AiErrorKind::DistributedExecutionFailed,
            "remote execution response was lost",
        )
        .with_outcome(OperationOutcome::Unknown)
        .with_retry(RetryDisposition::Unknown);

        assert_eq!(
            error.context().outcome(),
            Some(OperationOutcome::Unknown)
        );

        assert_eq!(
            error.context().retry(),
            Some(RetryDisposition::Unknown)
        );
    }

    #[test]
    fn resource_infeasibility_differs_from_resource_exhaustion() {
        let infeasible = AiError::new(
            AiErrorKind::ResourceInfeasible,
            "target cannot satisfy the declared requirement",
        );

        let exhausted = AiError::new(
            AiErrorKind::ResourceExhausted,
            "execution budget was exhausted",
        );

        assert!(infeasible.kind().is_compatibility_failure());
        assert!(exhausted.kind().is_operational_failure());

        assert_ne!(
            infeasible.code().as_str(),
            exhausted.code().as_str()
        );
    }

    #[test]
    fn target_incompatibility_is_not_source_semantic_invalidity() {
        let error = AiError::new(
            AiErrorKind::TargetIncompatible,
            "selected target cannot implement the required operation",
        );

        assert!(error.kind().is_compatibility_failure());

        assert!(
            !error.kind().is_domain_validation_failure()
        );
    }

    #[test]
    fn error_chaining_preserves_underlying_cause() {
        let underlying = std::io::Error::other(
            "simulated external provider failure",
        );

        let error = AiError::from_source(
            AiErrorKind::IntegrationFailure,
            "AI provider integration failed",
            underlying,
        );

        assert!(StdError::source(&error).is_some());
        assert!(error.source_error().is_some());
    }

    #[test]
    fn error_records_serialize_and_round_trip() {
        let error = AiError::new(
            AiErrorKind::TrainingFailed,
            "training failed",
        )
        .with_phase(AiErrorPhase::Training)
        .with_model("model-1")
        .with_retry(RetryDisposition::AfterConditionChange)
        .with_outcome(OperationOutcome::NotCommitted);

        let record = error.record();

        let encoded = serde_json::to_string(&record)
            .expect("serialization succeeds");

        let decoded: AiErrorRecord =
            serde_json::from_str(&encoded)
                .expect("deserialization succeeds");

        assert_eq!(decoded, record);
    }

    #[test]
    fn structured_error_record_excludes_opaque_source_objects() {
        let error = AiError::from_source(
            AiErrorKind::SerializationFailure,
            "serialization failed",
            std::io::Error::other("underlying failure"),
        );

        assert!(error.source_error().is_some());

        let record = error.record();

        assert_eq!(
            record.kind,
            AiErrorKind::SerializationFailure
        );

        assert_eq!(
            record.code.as_str(),
            "ZMN-AI-SERIALIZATION-FAILED"
        );
    }

    #[test]
    fn error_details_have_deterministic_ordering() {
        let context = AiErrorContext::new()
            .with_detail("zeta", "last")
            .with_detail("alpha", "first")
            .with_detail("middle", "between");

        let keys: Vec<&String> =
            context.details().keys().collect();

        assert_eq!(
            keys,
            vec![
                &"alpha".to_owned(),
                &"middle".to_owned(),
                &"zeta".to_owned(),
            ]
        );
    }

    #[test]
    fn display_contains_machine_code_and_explanation() {
        let error = AiError::new(
            AiErrorKind::CapabilityUnavailable,
            "required tensor operation is unavailable",
        )
        .with_phase(AiErrorPhase::CapabilityNegotiation)
        .with_model("model-42");

        let rendered = error.to_string();

        assert!(rendered.contains("ZMN-AI-CAPABILITY-UNAVAILABLE"));
        assert!(rendered.contains("required tensor operation"));
        assert!(rendered.contains("model-42"));
    }

    #[test]
    fn error_categories_are_open_to_future_extensions() {
        let error = AiError::with_code(
            AiErrorKind::IntegrationFailure,
            "ZMN-EXT-FUTURE-AI-COMPUTATION-FAILED",
            "future computational extension failed",
        )
        .expect("valid extension code");

        assert_eq!(
            error.kind(),
            AiErrorKind::IntegrationFailure
        );
    }

    #[test]
    fn no_retry_policy_is_assumed_implicitly() {
        let error = AiError::new(
            AiErrorKind::TrainingFailed,
            "training failed",
        );

        assert_eq!(error.context().retry(), None);
    }
}
