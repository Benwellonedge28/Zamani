
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI Configuration
//!
//! File: `src/ai/config.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines immutable, injectable configuration for the Zamani AI domain.
//!
//! Configuration expresses operational policy for AI model processing,
//! validation, numerical behavior, reproducibility, model compatibility,
//! training, inference, security expectations, resource budgets and
//! extension activation.
//!
//! ## Architectural authority
//!
//! Normative references:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/ai/README.md`
//! - `grammar/ai/ai.g4`
//! - `grammar/spec/ai.md`
//! - `src/frontend/semantic/domains/ai.rs`
//! - `src/frontend/semantic/diagnostics.rs`
//! - `src/frontend/lexer/config.rs`
//! - `src/ai/error.rs`
//!
//! The AI grammar and specification define AI language semantics.
//! The frontend owns source-level semantic validation.
//! The shared frontend resource model owns resource-limit representation.
//! Distributed execution, runtime and security infrastructure own
//! realization and enforcement of their respective responsibilities.
//!
//! ## This module owns
//!
//! - AI-domain validation preferences;
//! - numerical transformation policy;
//! - reproducibility policy;
//! - model compatibility policy;
//! - training policy;
//! - inference policy;
//! - AI security requirements and restrictions;
//! - AI operational budgets;
//! - AI extension activation policy;
//! - configuration validation and construction.
//!
//! ## This module does not own
//!
//! - Source syntax, grammar, AST or semantic analysis;
//! - the universal type, effect, capability or resource systems;
//! - hardware capability discovery;
//! - resource allocation or feasibility analysis;
//! - model definitions, tensors or model storage;
//! - model training or inference implementations;
//! - scheduling, placement, routing or distributed transport;
//! - checkpoint storage or recovery implementation;
//! - authorization engines, cryptographic verification or sandboxing;
//! - compiler configuration or runtime configuration;
//! - global mutable configuration state.
//!
//! ## Scalability and POCO-REAF
//!
//! All AI operational budgets default to `ResourceLimit::Unlimited`.
//!
//! A finite budget is an explicitly supplied operational restriction. It is
//! not a language-level maximum and must never silently change program
//! semantics.
//!
//! No universal maximum is imposed for:
//!
//! - models or model parameters;
//! - model graph nodes or depth;
//! - tensor rank, dimensions or elements;
//! - dataset records;
//! - training steps;
//! - inference requests;
//! - pipeline operations;
//! - agents or agent actions;
//! - memory;
//! - accelerators, devices, workers or distributed nodes.
//!
//! `Unlimited` means that this configuration imposes no explicit budget.
//! It does not promise infinite memory, address space, execution time or
//! physical resources.
//!
//! ## Integration contract
//!
//! - AI contexts receive a validated `AiConfig` by value or shared reference.
//! - AI validation consumes `AiValidationPolicy` but delegates source-level
//!   semantic validation to the canonical frontend.
//! - Numerical transformations consume `AiNumericalPolicy` and must preserve
//!   the source program's declared numerical contract.
//! - Reproducibility services consume `AiReproducibilityPolicy`.
//! - Model compatibility services consume `ModelCompatibilityPolicy`.
//! - Training and inference components consume their respective policies.
//! - Resource feasibility and accounting components consume
//!   `AiOperationalBudgets` and enforce only the budgets relevant to them.
//! - Extension registries consume `AiExtensionPolicy`; this configuration
//!   does not load or register extensions.
//! - Security infrastructure consumes `AiSecurityPolicy` and remains
//!   authoritative for enforcement.
//! - Runtime and distributed execution retain ownership of deadlines,
//!   scheduling, retries, transport, cancellation and recovery.
//!
//! Configuration validation checks only invariants owned by this module.
//! It does not certify that a model, target, extension, policy, or execution
//! request is valid or feasible.
//!
//! ## Dependencies
//!
//! Uses the canonical frontend `ResourceLimit` type.
//! Uses only the Rust standard library otherwise.
//!
//! No AI model, backend, runtime, distributed or hardware dependency is
//! introduced.

use core::fmt;
use std::collections::BTreeSet;
use std::time::Duration;

use crate::frontend::lexer::config::ResourceLimit;

// ============================================================================
// VALIDATION POLICY
// ============================================================================

/// Determines how AI-domain validation work is performed.
///
/// This policy does not replace or override the canonical frontend semantic
/// analyzer. It applies only to validation performed by AI-domain services.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AiValidationMode {
    /// Stop the current AI validation operation when a fatal issue is found.
    FailFast,

    /// Continue collecting independent AI-domain validation issues where
    /// doing so is safe and meaningful.
    #[default]
    CollectIssues,
}

/// AI-domain validation policy.
///
/// A successful result must never be returned when required validation
/// remains incomplete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AiValidationPolicy {
    /// Controls whether validation stops at the first fatal issue or
    /// collects independent issues.
    pub mode: AiValidationMode,

    /// Require validation to complete before an operation is accepted.
    ///
    /// Disabling this setting does not authorize an incomplete result to be
    /// represented as fully valid. Consumers must preserve the incomplete
    /// status explicitly.
    pub require_complete_validation: bool,

    /// Whether AI-domain validation may accept registered experimental
    /// operations.
    ///
    /// This does not promote experimental functionality to stable language
    /// semantics and does not bypass compatibility or security checks.
    pub allow_experimental_operations: bool,
}

impl AiValidationPolicy {
    /// Strict production-oriented validation defaults.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            mode: AiValidationMode::CollectIssues,
            require_complete_validation: true,
            allow_experimental_operations: false,
        }
    }

    /// Creates a policy that stops after a fatal issue.
    #[must_use]
    pub const fn fail_fast() -> Self {
        Self {
            mode: AiValidationMode::FailFast,
            require_complete_validation: true,
            allow_experimental_operations: false,
        }
    }
}

impl Default for AiValidationPolicy {
    fn default() -> Self {
        Self::strict()
    }
}

// ============================================================================
// NUMERICAL POLICY
// ============================================================================

/// Controls whether numerical transformations may relax declared semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum NumericalTransformationPolicy {
    /// Preserve the numerical requirements declared by the program.
    ///
    /// This is the default. An implementation must not silently replace
    /// exact or strict numerical behavior with approximate behavior.
    #[default]
    PreserveDeclaredSemantics,

    /// Permit approximate transformations only where the program's
    /// numerical contract explicitly allows them.
    AllowContractPermittedApproximation,
}

/// Controls numerical behavior for AI-domain computation.
///
/// These are transformation permissions, not declarations of a particular
/// floating-point format or hardware register width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AiNumericalPolicy {
    /// Controls permission to perform approximate numerical transformations.
    pub transformations: NumericalTransformationPolicy,

    /// Whether reassociation is permitted when the declared numerical
    /// contract allows it.
    pub allow_reassociation: bool,

    /// Whether precision relaxation is permitted when the declared
    /// numerical contract allows it.
    pub allow_precision_relaxation: bool,

    /// Whether the implementation may use an algorithm with different
    /// rounding behavior when the declared numerical contract permits it.
    pub allow_alternative_rounding: bool,
}

impl AiNumericalPolicy {
    /// Preserve declared numerical semantics.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            transformations: NumericalTransformationPolicy::PreserveDeclaredSemantics,
            allow_reassociation: false,
            allow_precision_relaxation: false,
            allow_alternative_rounding: false,
        }
    }

    /// Permit only transformations that are explicitly allowed by the
    /// program's numerical contract.
    #[must_use]
    pub const fn contract_permitted() -> Self {
        Self {
            transformations:
                NumericalTransformationPolicy::AllowContractPermittedApproximation,
            allow_reassociation: true,
            allow_precision_relaxation: true,
            allow_alternative_rounding: true,
        }
    }
}

impl Default for AiNumericalPolicy {
    fn default() -> Self {
        Self::strict()
    }
}

// ============================================================================
// REPRODUCIBILITY POLICY
// ============================================================================

/// Reproducibility expectations for AI computations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ReproducibilityMode {
    /// Preserve the reproducibility contract declared by the program.
    ///
    /// This does not silently strengthen or weaken source-level requirements.
    #[default]
    PreserveDeclaredContract,

    /// Require deterministic behavior wherever the complete execution
    /// contract and available implementation can provide it.
    ///
    /// If deterministic execution cannot be established, the responsible
    /// compiler or runtime must report that limitation.
    RequireDeterministic,

    /// Permit nondeterministic behavior only where the program's declared
    /// semantics permit it.
    PermitDeclaredNondeterminism,
}

/// Configuration for reproducibility of AI computations.
///
/// Reproducibility may depend on source identity, model and dataset versions,
/// numerical behavior, random-state information, compiler transformations,
/// external dependencies and execution realization.
///
/// This policy does not implement random-number generation or guarantee
/// cross-target bit-for-bit identity.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AiReproducibilityPolicy {
    /// Reproducibility requirement.
    pub mode: ReproducibilityMode,

    /// Optional caller-supplied random seed.
    ///
    /// A seed alone does not guarantee reproducibility across algorithms,
    /// implementations, numerical formats or hardware.
    pub seed: Option<u128>,

    /// Require provenance sufficient to identify the inputs and declared
    /// execution conditions relevant to reproducibility.
    pub require_provenance: bool,

    /// Require relevant numerical behavior to be recorded.
    pub record_numerical_profile: bool,
}

impl AiReproducibilityPolicy {
    /// Preserve the program's declared reproducibility contract.
    #[must_use]
    pub const fn preserve_declared_contract() -> Self {
        Self {
            mode: ReproducibilityMode::PreserveDeclaredContract,
            seed: None,
            require_provenance: true,
            record_numerical_profile: true,
        }
    }

    /// Require deterministic execution where supported.
    #[must_use]
    pub const fn deterministic() -> Self {
        Self {
            mode: ReproducibilityMode::RequireDeterministic,
            seed: None,
            require_provenance: true,
            record_numerical_profile: true,
        }
    }

    /// Supply an explicit random seed.
    #[must_use]
    pub fn with_seed(mut self, seed: u128) -> Self {
        self.seed = Some(seed);
        self
    }
}

// ============================================================================
// MODEL COMPATIBILITY POLICY
// ============================================================================

/// Controls compatibility handling for models and model artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ModelCompatibilityMode {
    /// Require exact compatibility with the declared model and artifact
    /// contracts unless an explicitly compatible representation is defined.
    #[default]
    RequireCompatible,

    /// Permit registered, validated compatibility migrations.
    ///
    /// The migration registry owns migration discovery and execution.
    AllowRegisteredMigrations,

    /// Reject deprecated model or artifact contracts even if a compatibility
    /// migration is available.
    RejectDeprecated,
}

/// Model and artifact compatibility policy.
///
/// This policy does not define model versions, artifact formats, migration
/// implementations or compiler compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ModelCompatibilityPolicy {
    /// Compatibility behavior.
    pub mode: ModelCompatibilityMode,

    /// Whether explicitly registered, semantics-preserving migrations may
    /// be considered.
    pub permit_semantics_preserving_migrations: bool,

    /// Whether lossy migrations are permitted.
    ///
    /// This defaults to false. A lossy migration must never be silently
    /// treated as semantics-preserving.
    pub permit_lossy_migrations: bool,
}

impl ModelCompatibilityPolicy {
    /// Conservative compatibility defaults.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            mode: ModelCompatibilityMode::RequireCompatible,
            permit_semantics_preserving_migrations: true,
            permit_lossy_migrations: false,
        }
    }
}

impl Default for ModelCompatibilityPolicy {
    fn default() -> Self {
        Self::strict()
    }
}

// ============================================================================
// TRAINING POLICY
// ============================================================================

/// Controls the treatment of nondeterministic training algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TrainingDeterminismPolicy {
    /// Follow the determinism requirements declared by the program.
    #[default]
    FollowProgramContract,

    /// Require deterministic training when supported.
    RequireDeterministic,

    /// Permit nondeterministic training only where the program contract
    /// explicitly permits it.
    PermitDeclaredNondeterminism,
}

/// Controls whether incomplete training results may be exposed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PartialTrainingResultPolicy {
    /// Do not present incomplete training as a completed training result.
    #[default]
    RejectAsComplete,

    /// Permit an explicitly marked partial result to be returned.
    ///
    /// Consumers must retain its incomplete status and must not represent
    /// it as a completed training run.
    ReturnMarkedPartial,
}

/// AI training policy.
///
/// This type describes policy only. It does not implement optimizers,
/// gradient computation, checkpoint storage, scheduling or training loops.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AiTrainingPolicy {
    /// Determinism expectations.
    pub determinism: TrainingDeterminismPolicy,

    /// Handling of incomplete training results.
    pub partial_results: PartialTrainingResultPolicy,

    /// Require training data provenance where the relevant data source
    /// supports provenance reporting.
    pub require_data_provenance: bool,

    /// Require optimizer and training-state compatibility checks before
    /// resuming a training operation.
    pub require_resume_compatibility: bool,
}

impl AiTrainingPolicy {
    /// Conservative training defaults.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            determinism: TrainingDeterminismPolicy::FollowProgramContract,
            partial_results: PartialTrainingResultPolicy::RejectAsComplete,
            require_data_provenance: true,
            require_resume_compatibility: true,
        }
    }
}

impl Default for AiTrainingPolicy {
    fn default() -> Self {
        Self::conservative()
    }
}

// ============================================================================
// INFERENCE POLICY
// ============================================================================

/// Controls whether inference may use approximate transformations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum InferenceApproximationPolicy {
    /// Preserve the numerical and behavioral contract declared by the model.
    #[default]
    FollowModelContract,

    /// Permit approximation only when explicitly allowed by the model
    /// and source-level numerical contract.
    PermitContractApprovedApproximation,
}

/// Controls fallback behavior during inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum InferenceFallbackPolicy {
    /// Do not select an undeclared fallback.
    ///
    /// A fallback explicitly declared by the program may still be used when
    /// its contract permits it.
    #[default]
    DeclaredFallbacksOnly,

    /// Permit registered fallback realizations only when their semantic
    /// compatibility has been established.
    AllowRegisteredCompatibleFallbacks,
}

/// AI inference policy.
///
/// This policy does not select devices, discover model providers, route
/// requests, or implement inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AiInferencePolicy {
    /// Approximation behavior.
    pub approximation: InferenceApproximationPolicy,

    /// Fallback behavior.
    pub fallback: InferenceFallbackPolicy,

    /// Require model compatibility verification before inference.
    pub require_model_compatibility: bool,

    /// Require model and inference provenance where available.
    pub require_provenance: bool,
}

impl AiInferencePolicy {
    /// Conservative inference defaults.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            approximation: InferenceApproximationPolicy::FollowModelContract,
            fallback: InferenceFallbackPolicy::DeclaredFallbacksOnly,
            require_model_compatibility: true,
            require_provenance: true,
        }
    }
}

impl Default for AiInferencePolicy {
    fn default() -> Self {
        Self::conservative()
    }
}

// ============================================================================
// SECURITY POLICY
// ============================================================================

/// Authorization expectations for AI operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AiAuthorizationPolicy {
    /// Require authorization to be established by the authoritative security
    /// subsystem before a protected operation proceeds.
    #[default]
    RequireExternalAuthorization,

    /// Defer the authorization decision to the caller's explicitly supplied
    /// security context.
    ///
    /// This is not permission to bypass authorization.
    DeferToSecurityContext,
}

/// Integrity expectations for models and artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AiArtifactIntegrityPolicy {
    /// Require integrity verification by the authoritative integrity service.
    #[default]
    RequireVerification,

    /// Follow the integrity requirements declared by the artifact and
    /// applicable security policy.
    FollowDeclaredRequirements,
}

/// Extension trust policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AiExtensionTrustPolicy {
    /// Reject extensions whose trust has not been established by the
    /// authoritative extension/security infrastructure.
    #[default]
    RequireTrustedExtensions,

    /// Follow the trust requirements supplied by the active security context.
    FollowSecurityContext,
}

/// AI security expectations.
///
/// This structure expresses requirements. It does not implement
/// authentication, authorization, signatures, cryptography, sandboxing,
/// isolation or audit storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AiSecurityPolicy {
    /// Authorization requirement.
    pub authorization: AiAuthorizationPolicy,

    /// Model and artifact integrity requirement.
    pub artifact_integrity: AiArtifactIntegrityPolicy,

    /// Extension trust requirement.
    pub extension_trust: AiExtensionTrustPolicy,

    /// Require auditable provenance for protected AI operations.
    pub require_audit_provenance: bool,
}

impl AiSecurityPolicy {
    /// Conservative security defaults.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            authorization: AiAuthorizationPolicy::RequireExternalAuthorization,
            artifact_integrity: AiArtifactIntegrityPolicy::RequireVerification,
            extension_trust: AiExtensionTrustPolicy::RequireTrustedExtensions,
            require_audit_provenance: true,
        }
    }
}

impl Default for AiSecurityPolicy {
    fn default() -> Self {
        Self::strict()
    }
}

// ============================================================================
// OPERATIONAL RESOURCE BUDGETS
// ============================================================================

/// Explicit operational budgets for AI-domain processing.
///
/// Every field uses the repository's canonical `ResourceLimit`.
///
/// A budget is enforced by the component that owns the corresponding work.
/// This structure does not allocate resources, estimate hardware capacity or
/// redefine semantic requirements.
///
/// `Unlimited` means that this configuration supplies no explicit ceiling
/// for that dimension. Real resource exhaustion remains possible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AiOperationalBudgets {
    /// Maximum model graph nodes processed by one configured operation.
    ///
    /// This is an operational processing budget, not a model-size language
    /// restriction.
    pub model_graph_nodes: ResourceLimit,

    /// Maximum model parameters processed by one configured operation.
    ///
    /// This is not a universal maximum parameter count.
    pub model_parameters: ResourceLimit,

    /// Maximum tensor elements processed by one configured operation.
    ///
    /// Tensor rank and dimension semantics remain owned by the canonical
    /// type and tensor systems.
    pub tensor_elements: ResourceLimit,

    /// Maximum dataset records processed by one configured operation.
    pub dataset_records: ResourceLimit,

    /// Maximum training steps performed by one configured training request.
    ///
    /// The language does not impose a maximum number of training steps.
    pub training_steps: ResourceLimit,

    /// Maximum inference requests processed by one configured request scope.
    pub inference_requests: ResourceLimit,

    /// Maximum AI pipeline operations processed by one configured request.
    pub pipeline_operations: ResourceLimit,

    /// Maximum agent actions processed by one configured request.
    pub agent_actions: ResourceLimit,

    /// Maximum memory bytes charged to the AI operation by the component
    /// responsible for memory accounting.
    ///
    /// This does not represent total physical memory or device memory.
    pub memory_bytes: ResourceLimit,

    /// Maximum implementation-defined AI work units.
    ///
    /// A consumer using this field must document the work-unit accounting
    /// convention and apply it consistently.
    pub work_units: ResourceLimit,

    /// Optional elapsed-time budget for one AI operation.
    ///
    /// The execution owner is responsible for measuring elapsed time and
    /// enforcing the deadline. This field does not implement cancellation.
    pub operation_time: Option<Duration>,
}

impl AiOperationalBudgets {
    /// Creates a budget configuration without explicit ceilings.
    ///
    /// This is the default POCO-REAF configuration. It does not promise
    /// infinite physical resources.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            model_graph_nodes: ResourceLimit::Unlimited,
            model_parameters: ResourceLimit::Unlimited,
            tensor_elements: ResourceLimit::Unlimited,
            dataset_records: ResourceLimit::Unlimited,
            training_steps: ResourceLimit::Unlimited,
            inference_requests: ResourceLimit::Unlimited,
            pipeline_operations: ResourceLimit::Unlimited,
            agent_actions: ResourceLimit::Unlimited,
            memory_bytes: ResourceLimit::Unlimited,
            work_units: ResourceLimit::Unlimited,
            operation_time: None,
        }
    }

    /// Returns true when no explicit count or memory budgets are configured
    /// and no operation-time budget is present.
    #[must_use]
    pub const fn has_no_explicit_budgets(&self) -> bool {
        !self.model_graph_nodes.is_bounded()
            && !self.model_parameters.is_bounded()
            && !self.tensor_elements.is_bounded()
            && !self.dataset_records.is_bounded()
            && !self.training_steps.is_bounded()
            && !self.inference_requests.is_bounded()
            && !self.pipeline_operations.is_bounded()
            && !self.agent_actions.is_bounded()
            && !self.memory_bytes.is_bounded()
            && !self.work_units.is_bounded()
            && self.operation_time.is_none()
    }
}

impl AiOperationalBudgets {
    /// Sets the operation-time budget.
    #[must_use]
    pub const fn with_operation_time(mut self, budget: Option<Duration>) -> Self {
        self.operation_time = budget;
        self
    }
}

// ============================================================================
// EXTENSION POLICY
// ============================================================================

/// Controls which registered AI extensions may be activated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AiExtensionActivationPolicy {
    /// Permit only extension identifiers explicitly enabled by the caller.
    ///
    /// An empty enabled set means that no non-core AI extension is enabled.
    #[default]
    Explicit,

    /// Permit all extensions available in the supplied registry, subject to
    /// trust, version, compatibility and conflict validation.
    ///
    /// This does not load code or authorize extension execution.
    AllowRegistered,
}

/// AI extension activation configuration.
///
/// Extension identifiers are opaque here. The extension registry owns
/// identifier resolution, registration, versioning and implementation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AiExtensionPolicy {
    /// Activation policy.
    pub activation: AiExtensionActivationPolicy,

    /// Explicitly enabled extension identifiers.
    pub enabled_extensions: BTreeSet<String>,

    /// Whether registered experimental AI extensions may be considered.
    ///
    /// Experimental extensions remain experimental and must still satisfy
    /// compatibility and security requirements.
    pub allow_experimental: bool,
}

impl AiExtensionPolicy {
    /// Explicit activation with no additional extensions enabled.
    #[must_use]
    pub fn explicit() -> Self {
        Self::default()
    }

    /// Enable one extension identifier.
    #[must_use]
    pub fn with_extension(mut self, extension_id: impl Into<String>) -> Self {
        self.enabled_extensions.insert(extension_id.into());
        self
    }

    /// Replace the explicitly enabled extension set.
    #[must_use]
    pub fn with_extensions(
        mut self,
        extension_ids: impl IntoIterator<Item = String>,
    ) -> Self {
        self.enabled_extensions = extension_ids.into_iter().collect();
        self
    }

    /// Select the activation policy.
    #[must_use]
    pub const fn with_activation(
        mut self,
        activation: AiExtensionActivationPolicy,
    ) -> Self {
        self.activation = activation;
        self
    }

    /// Enable or disable experimental extensions.
    #[must_use]
    pub const fn with_experimental(mut self, allow: bool) -> Self {
        self.allow_experimental = allow;
        self
    }
}

// ============================================================================
// CONFIGURATION
// ============================================================================

/// Complete AI-domain configuration.
///
/// `AiConfig` is a value object. It contains no global state, device handles,
/// runtime sessions, registries, model objects or mutable execution state.
///
/// Construct it once, validate it, and share it immutably with the AI
/// subsystem components that need it.
///
/// Fields are public to support explicit configuration construction and
/// external configuration adapters. Consumers must call `validate()` before
/// using externally constructed values.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AiConfig {
    /// AI-domain validation policy.
    pub validation: AiValidationPolicy,

    /// Numerical transformation policy.
    pub numerical: AiNumericalPolicy,

    /// Reproducibility policy.
    pub reproducibility: AiReproducibilityPolicy,

    /// Model and artifact compatibility policy.
    pub compatibility: ModelCompatibilityPolicy,

    /// Training policy.
    pub training: AiTrainingPolicy,

    /// Inference policy.
    pub inference: AiInferencePolicy,

    /// AI security requirements.
    pub security: AiSecurityPolicy,

    /// Operational budgets.
    pub budgets: AiOperationalBudgets,

    /// Extension activation and trust-related selection policy.
    pub extensions: AiExtensionPolicy,
}

impl AiConfig {
    /// Creates the default AI configuration.
    ///
    /// Defaults:
    ///
    /// - complete AI-domain validation;
    /// - preservation of declared numerical semantics;
    /// - preservation of declared reproducibility requirements;
    /// - conservative model compatibility;
    /// - conservative training and inference policies;
    /// - external authorization and artifact-integrity verification;
    /// - no explicit operational budgets;
    /// - explicit extension activation.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an incremental configuration builder.
    #[must_use]
    pub fn builder() -> AiConfigBuilder {
        AiConfigBuilder {
            config: Self::new(),
        }
    }

    /// Validates configuration invariants owned by the AI subsystem.
    ///
    /// This does not verify whether extensions exist, whether security
    /// services are available, whether a model is compatible, or whether a
    /// target can satisfy a resource requirement.
    pub fn validate(&self) -> Result<(), AiConfigError> {
        for extension_id in &self.extensions.enabled_extensions {
            if extension_id.trim().is_empty() {
                return Err(AiConfigError::EmptyExtensionId);
            }

            if extension_id.chars().any(char::is_whitespace) {
                return Err(AiConfigError::WhitespaceInExtensionId {
                    extension_id: extension_id.clone(),
                });
            }
        }

        if self.extensions.activation == AiExtensionActivationPolicy::AllowRegistered
            && !self.extensions.enabled_extensions.is_empty()
        {
            return Err(AiConfigError::ConflictingExtensionPolicies);
        }

        Ok(())
    }

    /// Replace the validation policy.
    #[must_use]
    pub fn with_validation(mut self, policy: AiValidationPolicy) -> Self {
        self.validation = policy;
        self
    }

    /// Replace the numerical policy.
    #[must_use]
    pub fn with_numerical(mut self, policy: AiNumericalPolicy) -> Self {
        self.numerical = policy;
        self
    }

    /// Replace the reproducibility policy.
    #[must_use]
    pub fn with_reproducibility(
        mut self,
        policy: AiReproducibilityPolicy,
    ) -> Self {
        self.reproducibility = policy;
        self
    }

    /// Replace the model compatibility policy.
    #[must_use]
    pub fn with_compatibility(
        mut self,
        policy: ModelCompatibilityPolicy,
    ) -> Self {
        self.compatibility = policy;
        self
    }

    /// Replace the training policy.
    #[must_use]
    pub fn with_training(mut self, policy: AiTrainingPolicy) -> Self {
        self.training = policy;
        self
    }

    /// Replace the inference policy.
    #[must_use]
    pub fn with_inference(mut self, policy: AiInferencePolicy) -> Self {
        self.inference = policy;
        self
    }

    /// Replace the security policy.
    #[must_use]
    pub fn with_security(mut self, policy: AiSecurityPolicy) -> Self {
        self.security = policy;
        self
    }

    /// Replace all AI operational budgets.
    #[must_use]
    pub const fn with_budgets(mut self, budgets: AiOperationalBudgets) -> Self {
        self.budgets = budgets;
        self
    }

    /// Replace the extension policy.
    #[must_use]
    pub fn with_extensions(mut self, policy: AiExtensionPolicy) -> Self {
        self.extensions = policy;
        self
    }
}

// ============================================================================
// CONFIGURATION BUILDER
// ============================================================================

/// Incremental builder for [`AiConfig`].
///
/// The builder performs no I/O, extension loading, hardware discovery,
/// resource allocation, model construction or runtime initialization.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AiConfigBuilder {
    config: AiConfig,
}

impl AiConfigBuilder {
    /// Set AI validation policy.
    #[must_use]
    pub fn validation(mut self, policy: AiValidationPolicy) -> Self {
        self.config.validation = policy;
        self
    }

    /// Set numerical policy.
    #[must_use]
    pub fn numerical(mut self, policy: AiNumericalPolicy) -> Self {
        self.config.numerical = policy;
        self
    }

    /// Set reproducibility policy.
    #[must_use]
    pub fn reproducibility(
        mut self,
        policy: AiReproducibilityPolicy,
    ) -> Self {
        self.config.reproducibility = policy;
        self
    }

    /// Set model compatibility policy.
    #[must_use]
    pub fn compatibility(mut self, policy: ModelCompatibilityPolicy) -> Self {
        self.config.compatibility = policy;
        self
    }

    /// Set training policy.
    #[must_use]
    pub fn training(mut self, policy: AiTrainingPolicy) -> Self {
        self.config.training = policy;
        self
    }

    /// Set inference policy.
    #[must_use]
    pub fn inference(mut self, policy: AiInferencePolicy) -> Self {
        self.config.inference = policy;
        self
    }

    /// Set AI security requirements.
    #[must_use]
    pub fn security(mut self, policy: AiSecurityPolicy) -> Self {
        self.config.security = policy;
        self
    }

    /// Set AI operational budgets.
    #[must_use]
    pub const fn budgets(mut self, budgets: AiOperationalBudgets) -> Self {
        self.config.budgets = budgets;
        self
    }

    /// Set extension activation policy.
    #[must_use]
    pub fn extensions(mut self, policy: AiExtensionPolicy) -> Self {
        self.config.extensions = policy;
        self
    }

    /// Validate and return the completed configuration.
    pub fn build(self) -> Result<AiConfig, AiConfigError> {
        self.config.validate()?;
        Ok(self.config)
    }
}

// ============================================================================
// CONFIGURATION ERRORS
// ============================================================================

/// AI configuration construction or validation failure.
///
/// These errors describe invalid configuration values or combinations.
/// They are not source-language semantic diagnostics or runtime failures.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AiConfigError {
    /// An explicitly enabled extension identifier is empty.
    EmptyExtensionId,

    /// An extension identifier contains whitespace.
    WhitespaceInExtensionId {
        /// Invalid identifier.
        extension_id: String,
    },

    /// The allow-all-registered policy conflicts with an explicit extension
    /// allow-list.
    ConflictingExtensionPolicies,
}

impl fmt::Display for AiConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyExtensionId => {
                formatter.write_str("AI extension identifier must not be empty")
            }
            Self::WhitespaceInExtensionId { extension_id } => {
                write!(
                    formatter,
                    "AI extension identifier contains whitespace: {extension_id:?}"
                )
            }
            Self::ConflictingExtensionPolicies => {
                formatter.write_str(
                    "AI extension policy cannot allow all registered extensions \
                     while also specifying an explicit extension allow-list",
                )
            }
        }
    }
}

impl std::error::Error for AiConfigError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;

    #[test]
    fn default_configuration_is_valid() {
        let config = AiConfig::default();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn default_budgets_do_not_impose_explicit_limits() {
        let budgets = AiOperationalBudgets::default();

        assert!(budgets.has_no_explicit_budgets());
    }

    #[test]
    fn unlimited_budgets_are_explicitly_unbounded() {
        let budgets = AiOperationalBudgets::unlimited();

        assert_eq!(budgets.model_graph_nodes, ResourceLimit::Unlimited);
        assert_eq!(budgets.model_parameters, ResourceLimit::Unlimited);
        assert_eq!(budgets.tensor_elements, ResourceLimit::Unlimited);
        assert_eq!(budgets.dataset_records, ResourceLimit::Unlimited);
        assert_eq!(budgets.training_steps, ResourceLimit::Unlimited);
        assert_eq!(budgets.inference_requests, ResourceLimit::Unlimited);
        assert_eq!(budgets.pipeline_operations, ResourceLimit::Unlimited);
        assert_eq!(budgets.agent_actions, ResourceLimit::Unlimited);
        assert_eq!(budgets.memory_bytes, ResourceLimit::Unlimited);
        assert_eq!(budgets.work_units, ResourceLimit::Unlimited);
        assert_eq!(budgets.operation_time, None);
    }

    #[test]
    fn caller_can_supply_finite_operational_budgets() {
        let limit = ResourceLimit::AtMost(
            NonZeroUsize::new(4096).expect("nonzero test value"),
        );

        let budgets = AiOperationalBudgets {
            tensor_elements: limit,
            ..AiOperationalBudgets::unlimited()
        };

        assert!(budgets.tensor_elements.is_bounded());
        assert!(!budgets.has_no_explicit_budgets());
    }

    #[test]
    fn large_representable_budgets_are_not_rejected_by_ai_configuration() {
        let limit = ResourceLimit::AtMost(
            NonZeroUsize::new(usize::MAX).expect("usize::MAX is nonzero"),
        );

        let budgets = AiOperationalBudgets {
            model_parameters: limit,
            ..AiOperationalBudgets::unlimited()
        };

        assert_eq!(budgets.model_parameters, limit);
    }

    #[test]
    fn operation_time_budget_is_optional() {
        let budgets = AiOperationalBudgets::unlimited()
            .with_operation_time(Some(Duration::from_secs(30)));

        assert_eq!(budgets.operation_time, Some(Duration::from_secs(30)));
        assert!(!budgets.has_no_explicit_budgets());
    }

    #[test]
    fn explicit_extension_registration_is_valid() {
        let policy = AiExtensionPolicy::explicit()
            .with_extension("example.extension");

        let config = AiConfig::default().with_extensions(policy);

        assert!(config.validate().is_ok());
    }

    #[test]
    fn empty_extension_identifier_is_rejected() {
        let policy = AiExtensionPolicy::explicit().with_extension(" ");

        let config = AiConfig::default().with_extensions(policy);

        assert_eq!(
            config.validate(),
            Err(AiConfigError::EmptyExtensionId)
        );
    }

    #[test]
    fn extension_identifier_with_whitespace_is_rejected() {
        let policy = AiExtensionPolicy::explicit()
            .with_extension("example extension");

        let config = AiConfig::default().with_extensions(policy);

        assert!(matches!(
            config.validate(),
            Err(AiConfigError::WhitespaceInExtensionId { .. })
        ));
    }

    #[test]
    fn allow_registered_conflicts_with_explicit_allow_list() {
        let policy = AiExtensionPolicy::explicit()
            .with_extension("example.extension")
            .with_activation(AiExtensionActivationPolicy::AllowRegistered);

        let config = AiConfig::default().with_extensions(policy);

        assert_eq!(
            config.validate(),
            Err(AiConfigError::ConflictingExtensionPolicies)
        );
    }

    #[test]
    fn builder_returns_validated_configuration() {
        let config = AiConfig::builder()
            .validation(AiValidationPolicy::strict())
            .numerical(AiNumericalPolicy::strict())
            .compatibility(ModelCompatibilityPolicy::strict())
            .training(AiTrainingPolicy::conservative())
            .inference(AiInferencePolicy::conservative())
            .security(AiSecurityPolicy::strict())
            .budgets(AiOperationalBudgets::unlimited())
            .extensions(AiExtensionPolicy::explicit())
            .build()
            .expect("default policies should be valid");

        assert!(config.validate().is_ok());
    }

    #[test]
    fn deterministic_reproducibility_policy_preserves_seed() {
        let policy = AiReproducibilityPolicy::deterministic()
            .with_seed(123456789);

        assert_eq!(policy.mode, ReproducibilityMode::RequireDeterministic);
        assert_eq!(policy.seed, Some(123456789));
        assert!(policy.require_provenance);
    }

    #[test]
    fn numerical_defaults_do_not_authorize_approximate_rewrites() {
        let policy = AiNumericalPolicy::default();

        assert_eq!(
            policy.transformations,
            NumericalTransformationPolicy::PreserveDeclaredSemantics
        );
        assert!(!policy.allow_reassociation);
        assert!(!policy.allow_precision_relaxation);
        assert!(!policy.allow_alternative_rounding);
    }

    #[test]
    fn security_defaults_require_external_authorization_and_integrity() {
        let policy = AiSecurityPolicy::default();

        assert_eq!(
            policy.authorization,
            AiAuthorizationPolicy::RequireExternalAuthorization
        );
        assert_eq!(
            policy.artifact_integrity,
            AiArtifactIntegrityPolicy::RequireVerification
        );
        assert_eq!(
            policy.extension_trust,
            AiExtensionTrustPolicy::RequireTrustedExtensions
        );
    }

    #[test]
    fn configuration_is_a_value_that_can_be_cloned() {
        let original = AiConfig::default();
        let copied = original.clone();

        assert_eq!(original, copied);
    }
}
