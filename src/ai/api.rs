"src/ai/api.rs"

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI — Stable Public API
//!
//! Path: `src/ai/api.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides the stable public programming interface for Zamani's
//! AI computational domain.
//!
//! This module is the public façade over the AI subsystem.
//!
//! It composes existing AI infrastructure rather than implementing
//! competing versions of existing repository facilities.
//!
//! ## ARCHITECTURAL AUTHORITIES
//!
//! Normative specifications:
//!
//! - grammar/DESIGN.md
//! - grammar/spec/ai.md
//! - grammar/ai/README.md
//! - grammar/ai/ai.g4
//!
//! Existing implementation authorities:
//!
//! - crate::ai::config
//! - crate::ai::context
//! - crate::ai::registry
//! - crate::ai::version
//! - crate::ai::error
//! - crate::ai::capabilities
//! - crate::ai::requirements
//! - crate::ai::compatibility
//! - crate::ai::provenance
//! - crate::ai::validation
//! - crate::frontend::semantic::domains::ai
//!
//! ## OWNS
//!
//! This module owns:
//!
//! - The stable public AI API boundary.
//! - Public AI subsystem construction.
//! - Explicit AI dependency injection.
//! - Public operation registration workflows.
//! - Public operation resolution workflows.
//! - Public configuration access.
//! - Public version-contract access.
//! - Public AI context access.
//! - Public AI error translation.
//! - Stable re-exports of approved AI-domain contracts.
//!
//! ## DOES NOT OWN
//!
//! This module does not own:
//!
//! - Source syntax.
//! - Lexer or parser implementation.
//! - AST construction.
//! - Semantic analysis.
//! - Type checking.
//! - Effect checking.
//! - Capability discovery.
//! - Resource feasibility.
//! - Model storage.
//! - Tensor storage.
//! - Training execution.
//! - Inference execution.
//! - Reasoning algorithms.
//! - Automatic differentiation implementation.
//! - Agent execution.
//! - Pipeline scheduling.
//! - Universal intermediate representations.
//! - Hardware discovery.
//! - Device allocation.
//! - Distributed transport.
//! - Runtime execution.
//!
//! These responsibilities remain with their authoritative modules.
//!
//! ## POCO-REAF
//!
//! The public AI API is independent of:
//!
//! - Processor architecture.
//! - Processor count.
//! - Accelerator count.
//! - Memory capacity.
//! - Tensor rank.
//! - Model size.
//! - Dataset size.
//! - Agent population.
//! - Distributed node count.
//! - Hardware vendor.
//! - Physical topology.
//!
//! No language-level computational capacity ceiling is introduced.
//!
//! Operational budgets remain explicit configuration policies.
//!
//! Actual feasibility is determined by downstream compiler,
//! capability, resource, backend, distributed and runtime systems.
//!
//! Literal infinite computation is not promised.
//!
//! ## DEPENDENCY DIRECTION
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Canonical frontend AST
//!      |
//!      v
//! Canonical semantic analysis
//!      |
//!      v
//! AI semantic domain
//!      |
//!      v
//! THIS PUBLIC API
//!      |
//!      +---- AI model interfaces
//!      +---- AI tensor interfaces
//!      +---- AI learning interfaces
//!      +---- AI reasoning interfaces
//!      +---- AI agent interfaces
//!      +---- AI pipeline interfaces
//!      +---- AI differentiation interfaces
//!      |
//!      v
//! ZUIR and canonical domain IRs
//!      |
//!      v
//! Compiler / optimization / lowering
//!      |
//!      v
//! Hybrid / distributed / hardware
//!      |
//!      v
//! Runtime realization
//! ```
//!
//! The public API must not reverse this dependency direction.
//!
//! ## INTEGRATION
//!
//! `AiApi` composes `AiContext`.
//!
//! `AiApiBuilder` composes `AiContextBuilder` and
//! `AiOperationRegistry`.
//!
//! The frontend semantic analyzer remains responsible for
//! producing canonical semantic results.
//!
//! AI operation descriptors remain descriptive declarations.
//! Registration does not establish executable implementation,
//! hardware availability, authorization or resource feasibility.
//!
//! ## ERROR CONTRACT
//!
//! Public API operations preserve their original failure categories.
//!
//! Invalid configuration, invalid operation declarations,
//! duplicate registrations and unresolved operations are
//! distinguishable.
//!
//! Errors are never silently converted into successful results.
//!
//! ## CONCURRENCY
//!
//! An initialized AiApi owns an immutable AiContext.
//!
//! Context cloning shares immutable configuration and registry
//! snapshots through Arc.
//!
//! Registry modifications are performed through a builder before
//! publishing a new API context.
//!
//! No hidden global mutable state is introduced.
//!
//! ## FREEZE CRITERIA
//!
//! - Uses existing AI context infrastructure.
//! - Uses the canonical AI operation registry.
//! - Uses canonical frontend semantic types.
//! - Preserves exact operation version resolution.
//! - Supports atomic batch registration.
//! - Preserves explicit semantic-service lifetimes.
//! - Preserves structured errors.
//! - Introduces no competing AST, IR, tensor or runtime.
//! - Introduces no arbitrary hardware ceilings.
//! - Contains no unsafe code.
//! - Passes public API and integration tests.

use std::error::Error;
use std::fmt;
use std::sync::Arc;

// ============================================================================
// EXISTING AI INFRASTRUCTURE
// ============================================================================

use crate::ai::config::AiConfig;

use crate::ai::context::{
    AiContext,
    AiContextBuilder,
    AiContextError,
    AiVersionSet,
};

use crate::ai::registry::{
    AiOperationDescriptor,
    AiOperationRegistry,
    AiRegistryError,
};

use crate::ai::version::AiOperationSetVersion;

use crate::frontend::semantic::domains::ai::{
    AiOperationKind,
    AiSemanticContext,
    QualifiedName,
};

// ============================================================================
// PUBLIC CONTRACT RE-EXPORTS
// ============================================================================

/// Canonical AI-domain error.
pub use crate::ai::error::{
    AiError,
    AiResult,
};

/// Canonical AI-domain identifiers.
pub use crate::ai::ids::{
    AiAgentId,
    AiArtifactId,
    AiDatasetId,
    AiDecisionId,
    AiEvidenceId,
    AiKnowledgeId,
    AiModelId,
    AiParameterId,
    AiPipelineId,
    AiTrainingRunId,
};

/// Canonical AI configuration.
pub use crate::ai::config::{
    AiExtensionPolicy,
    AiInferencePolicy,
    AiNumericalPolicy,
    AiOperationalBudgets,
    AiReproducibilityPolicy,
    AiSecurityPolicy,
    AiTrainingPolicy,
    AiValidationPolicy,
    ModelCompatibilityPolicy,
};

/// Canonical AI version contracts.
pub use crate::ai::version::{
    AiApiVersion,
    AiArtifactFormatVersion,
    AiModelSchemaVersion,
    AiOperationSetVersion,
    AiSemanticVersion,
};

/// Canonical AI capability integration.
pub use crate::ai::capabilities::{
    ai_capability_id,
    is_ai_capability,
    AiCapabilityEvidence,
    AiCapabilityEvidenceSet,
    AiCapabilityId,
    AiCapabilityRequirement,
    AiCapabilityVersion,
};

/// Canonical AI compatibility contracts.
pub use crate::ai::compatibility::AiFeatureId;

/// Canonical AI requirement interfaces.
pub use crate::ai::requirements::{
    RequirementKey,
    RequirementKind,
    RequirementValue,
};

/// Canonical AI provenance integration.
pub use crate::ai::provenance::{
    AiEntityKind,
    AiProvenanceRecord,
};

// ============================================================================
// PUBLIC SEMANTIC TYPES
// ============================================================================

/// Canonical frontend AI operation classification.
///
/// This is a re-export of the semantic authority, not a new AI enum.
pub use crate::frontend::semantic::domains::ai::AiOperationKind as OperationKind;

/// Canonical frontend AI qualified name.
///
/// This is the existing semantic representation.
pub use crate::frontend::semantic::domains::ai::QualifiedName as OperationName;

/// Canonical frontend AI semantic service interface.
///
/// Implementations remain owned by the frontend semantic subsystem.
pub use crate::frontend::semantic::domains::ai::AiSemanticContext as SemanticServices;

// ============================================================================
// PUBLIC OPERATION CONTRACT
// ============================================================================

/// Immutable, validated declaration of an AI computational operation.
///
/// This is descriptive metadata.
///
/// It does not contain executable code or hardware-specific handles.
pub type OperationDescriptor = AiOperationDescriptor;

/// Exact versioned AI operation registry.
///
/// This is the existing canonical AI registry.
pub type OperationRegistry = AiOperationRegistry;

/// AI operation registry failure.
pub type OperationRegistryError = AiRegistryError;

/// Exact AI operation-set version.
pub type OperationVersion = AiOperationSetVersion;

// ============================================================================
// PUBLIC CONTEXT CONTRACT
// ============================================================================

/// Immutable AI subsystem dependency context.
///
/// The context shares configuration and registry snapshots.
///
/// It does not execute AI models.
pub type Context<'services> = AiContext<'services>;

/// Builder for an immutable AI dependency context.
pub type ContextBuilder<'services> = AiContextBuilder<'services>;

/// AI context construction failure.
pub type ContextError = AiContextError;

/// Independently versioned AI subsystem contracts.
pub type VersionSet = AiVersionSet;

// ============================================================================
// API ERROR
// ============================================================================

/// Errors returned by the stable public AI API.
///
/// The original subsystem error is retained in every variant.
#[derive(Debug)]
#[non_exhaustive]
pub enum ApiError {
    /// AI configuration or context construction failed.
    Context(AiContextError),

    /// An operation declaration could not be registered.
    Registry(AiRegistryError),
}

impl fmt::Display for ApiError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Context(error) => {
                write!(
                    formatter,
                    "Zamani AI API context error: {error}"
                )
            }

            Self::Registry(error) => {
                write!(
                    formatter,
                    "Zamani AI API registry error: {error}"
                )
            }
        }
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Context(error) => Some(error),
            Self::Registry(error) => Some(error),
        }
    }
}

impl From<AiContextError> for ApiError {
    fn from(error: AiContextError) -> Self {
        Self::Context(error)
    }
}

impl From<AiRegistryError> for ApiError {
    fn from(error: AiRegistryError) -> Self {
        Self::Registry(error)
    }
}

/// Result type for public AI API construction and management.
pub type ApiResult<T> = Result<T, ApiError>;

// ============================================================================
// STABLE PUBLIC API
// ============================================================================

/// Stable public entry point to Zamani AI infrastructure.
///
/// `AiApi` is a lightweight façade over the canonical AI context.
///
/// It does not introduce another execution engine.
///
/// # Construction
///
/// Use [`AiApi::new`] for default configuration or
/// [`AiApi::builder`] for explicit dependency injection.
///
/// # Portability
///
/// An API instance contains no device-specific assumptions.
///
/// The same logical AI API contract can be consumed by different
/// compiler and execution realizations.
pub struct AiApi<'services> {
    context: AiContext<'services>,
}

impl<'services> AiApi<'services> {
    /// Construct the public AI API using validated default configuration.
    ///
    /// The operation registry starts empty.
    ///
    /// No hardware discovery or execution is performed.
    ///
    /// # Errors
    ///
    /// Returns an error if the default configuration is invalid.
    pub fn new() -> ApiResult<Self> {
        Self::builder().build()
    }

    /// Construct the public AI API with explicit configuration.
    ///
    /// The operation registry starts empty.
    ///
    /// # Errors
    ///
    /// Returns an error if configuration validation fails.
    pub fn with_config(
        config: AiConfig,
    ) -> ApiResult<Self> {
        Self::builder()
            .config(config)
            .build()
    }

    /// Construct an AI API from an existing validated context.
    ///
    /// This is useful when an application already manages AI
    /// dependency injection and context lifecycle.
    #[must_use]
    pub fn from_context(
        context: AiContext<'services>,
    ) -> Self {
        Self { context }
    }

    /// Create an explicit AI API builder.
    ///
    /// The builder begins with:
    ///
    /// - validated default configuration at build time;
    /// - an empty operation registry;
    /// - initial AI contract versions;
    /// - no implicit semantic service.
    #[must_use]
    pub fn builder() -> AiApiBuilder<'services> {
        AiApiBuilder::new()
    }

    /// Return the immutable AI context.
    #[must_use]
    pub fn context(&self) -> &AiContext<'services> {
        &self.context
    }

    /// Return the immutable AI configuration.
    #[must_use]
    pub fn config(&self) -> &AiConfig {
        self.context.config()
    }

    /// Return the AI version set.
    #[must_use]
    pub const fn versions(&self) -> AiVersionSet {
        self.context.versions()
    }

    /// Return the immutable operation registry.
    #[must_use]
    pub fn operations(&self) -> &AiOperationRegistry {
        self.context.registry()
    }

    /// Return the number of registered operation declarations.
    ///
    /// This is a count of registry entries, not executable implementations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.context.registered_operation_count()
    }

    /// Return whether the operation registry is empty.
    #[must_use]
    pub fn operations_are_empty(&self) -> bool {
        self.context.has_no_registered_operations()
    }

    /// Resolve an exact AI operation declaration.
    ///
    /// No implicit version substitution occurs.
    ///
    /// The requested operation must be registered under the exact
    /// qualified name and operation-set version.
    ///
    /// # Errors
    ///
    /// Returns `ApiError::Registry` if the operation is not registered.
    pub fn resolve_operation(
        &self,
        name: &QualifiedName,
        version: AiOperationSetVersion,
    ) -> ApiResult<Arc<AiOperationDescriptor>> {
        self.context
            .registry()
            .resolve(name, version)
            .map_err(ApiError::Registry)
    }

    /// Check whether an exact operation declaration is registered.
    #[must_use]
    pub fn has_operation(
        &self,
        name: &QualifiedName,
        version: AiOperationSetVersion,
    ) -> bool {
        self.context
            .registry()
            .contains(name, version)
    }

    /// Return an independent builder based on this API's configuration.
    ///
    /// The existing registry is copied as an immutable declaration
    /// snapshot.
    ///
    /// Changes made through the returned builder cannot mutate
    /// the existing API instance.
    #[must_use]
    pub fn to_builder(&self) -> AiApiBuilder<'services> {
        AiApiBuilder {
            config: self.context.config().clone(),
            registry: self.context.registry().snapshot(),
            versions: self.context.versions(),
            semantic_services: self.context.semantic_services(),
        }
    }
}

// ============================================================================
// API BUILDER
// ============================================================================

/// Builder for the stable Zamani AI API.
///
/// All dependencies are explicit.
///
/// The builder does not load extensions, execute code, discover hardware,
/// allocate resources or initialize runtime services.
///
/// Registry changes are staged before the API context is published.
pub struct AiApiBuilder<'services> {
    config: AiConfig,

    registry: AiOperationRegistry,

    versions: AiVersionSet,

    semantic_services:
        Option<&'services dyn AiSemanticContext>,
}

impl<'services> AiApiBuilder<'services> {
    /// Create a builder with the canonical initial AI configuration.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: AiConfig::default(),
            registry: AiOperationRegistry::new(),
            versions: AiVersionSet::initial(),
            semantic_services: None,
        }
    }

    /// Replace the AI configuration.
    ///
    /// Configuration is validated when `build` is called.
    #[must_use]
    pub fn config(
        mut self,
        config: AiConfig,
    ) -> Self {
        self.config = config;
        self
    }

    /// Replace the complete AI operation registry.
    ///
    /// The supplied registry is transferred into the builder.
    #[must_use]
    pub fn registry(
        mut self,
        registry: AiOperationRegistry,
    ) -> Self {
        self.registry = registry;
        self
    }

    /// Replace the AI version set.
    ///
    /// Each version belongs to its independently governed
    /// compatibility domain.
    #[must_use]
    pub fn versions(
        mut self,
        versions: AiVersionSet,
    ) -> Self {
        self.versions = versions;
        self
    }

    /// Attach the canonical frontend AI semantic services.
    ///
    /// The reference is borrowed and its lifetime is preserved
    /// by the resulting API instance.
    ///
    /// This does not replace semantic analysis.
    #[must_use]
    pub fn semantic_services(
        mut self,
        services: &'services dyn AiSemanticContext,
    ) -> Self {
        self.semantic_services = Some(services);
        self
    }

    /// Register one AI operation declaration.
    ///
    /// The declaration is validated before registration.
    ///
    /// Duplicate exact name/version pairs are rejected.
    ///
    /// Registration does not establish implementation availability
    /// or target feasibility.
    ///
    /// # Errors
    ///
    /// Returns the canonical registry error.
    pub fn register_operation(
        &mut self,
        descriptor: AiOperationDescriptor,
    ) -> Result<Arc<AiOperationDescriptor>, AiRegistryError> {
        self.registry.register(descriptor)
    }

    /// Atomically register multiple AI operation declarations.
    ///
    /// The existing registry remains unchanged if any declaration
    /// is invalid or duplicates an existing registration.
    ///
    /// Multiple versions of the same operation may coexist.
    ///
    /// # Errors
    ///
    /// Returns the first canonical registry error.
    pub fn register_operations<I>(
        &mut self,
        descriptors: I,
    ) -> Result<(), AiRegistryError>
    where
        I: IntoIterator<Item = AiOperationDescriptor>,
    {
        self.registry.register_batch(descriptors)
    }

    /// Remove one exact operation registration.
    ///
    /// Previously returned immutable declaration snapshots remain valid.
    pub fn remove_operation(
        &mut self,
        name: &QualifiedName,
        version: AiOperationSetVersion,
    ) -> Option<Arc<AiOperationDescriptor>> {
        self.registry.remove(name, version)
    }

    /// Return the number of registered declarations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.registry.len()
    }

    /// Return whether no operations are registered.
    #[must_use]
    pub fn operations_are_empty(&self) -> bool {
        self.registry.is_empty()
    }

    /// Build the validated public AI API.
    ///
    /// All configuration checks complete before the context is
    /// published.
    ///
    /// The operation registry is transferred into an immutable context.
    ///
    /// # Errors
    ///
    /// Returns an error if configuration validation fails.
    pub fn build(self) -> ApiResult<AiApi<'services>> {
        let Self {
            config,
            registry,
            versions,
            semantic_services,
        } = self;

        let mut context_builder = AiContextBuilder::new()
            .config(config)
            .registry(registry)
            .versions(versions);

        if let Some(services) = semantic_services {
            context_builder =
                context_builder.semantic_services(services);
        }

        let context = context_builder.build()?;

        Ok(AiApi { context })
    }
}

impl Default for AiApiBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation_name() -> QualifiedName {
        QualifiedName::new(
            "zamani.ai",
            "inference",
        )
        .expect("valid operation name")
    }

    fn operation_version() -> AiOperationSetVersion {
        AiOperationSetVersion::new(1, 0, 0)
    }

    fn operation_descriptor() -> AiOperationDescriptor {
        AiOperationDescriptor::new(
            operation_name(),
            AiOperationKind::Inference,
            operation_version(),
        )
        .expect("valid operation descriptor")
    }

    #[test]
    fn creates_public_api_with_default_configuration() {
        let api = AiApi::new()
            .expect("default API construction succeeds");

        assert!(api.operations_are_empty());
        assert_eq!(api.operation_count(), 0);
    }

    #[test]
    fn creates_public_api_with_explicit_configuration() {
        let config = AiConfig::default();

        let api = AiApi::with_config(config.clone())
            .expect("valid configuration");

        assert_eq!(api.config(), &config);
    }

    #[test]
    fn builder_registers_an_operation() {
        let mut builder = AiApiBuilder::new();

        builder
            .register_operation(operation_descriptor())
            .expect("operation registration succeeds");

        let api = builder
            .build()
            .expect("API construction succeeds");

        assert_eq!(api.operation_count(), 1);

        assert!(api.has_operation(
            &operation_name(),
            operation_version(),
        ));
    }

    #[test]
    fn resolves_exact_registered_operation() {
        let mut builder = AiApiBuilder::new();

        builder
            .register_operation(operation_descriptor())
            .expect("registration succeeds");

        let api = builder
            .build()
            .expect("construction succeeds");

        let descriptor = api
            .resolve_operation(
                &operation_name(),
                operation_version(),
            )
            .expect("exact operation exists");

        assert_eq!(
            descriptor.kind(),
            &AiOperationKind::Inference
        );
    }

    #[test]
    fn unknown_operation_returns_an_error() {
        let api = AiApi::new()
            .expect("valid API");

        let result = api.resolve_operation(
            &operation_name(),
            operation_version(),
        );

        assert!(matches!(
            result,
            Err(ApiError::Registry(
                AiRegistryError::UnknownOperation { .. }
            ))
        ));
    }

    #[test]
    fn duplicate_registration_is_rejected() {
        let mut builder = AiApiBuilder::new();

        builder
            .register_operation(operation_descriptor())
            .expect("first registration");

        let result = builder.register_operation(
            operation_descriptor()
        );

        assert!(matches!(
            result,
            Err(AiRegistryError::DuplicateOperation { .. })
        ));

        assert_eq!(builder.operation_count(), 1);
    }

    #[test]
    fn batch_registration_is_atomic() {
        let mut builder = AiApiBuilder::new();

        builder
            .register_operation(operation_descriptor())
            .expect("initial registration");

        let second_version = AiOperationSetVersion::new(
            2,
            0,
            0,
        );

        let second = AiOperationDescriptor::new(
            operation_name(),
            AiOperationKind::Inference,
            second_version,
        )
        .expect("valid second operation");

        let result = builder.register_operations(vec![
            second,
            operation_descriptor(),
        ]);

        assert!(result.is_err());

        assert_eq!(builder.operation_count(), 1);

        assert!(!builder.registry.contains(
            &operation_name(),
            second_version,
        ));
    }

    #[test]
    fn multiple_versions_can_coexist() {
        let mut builder = AiApiBuilder::new();

        let first = operation_descriptor();

        let second_version = AiOperationSetVersion::new(
            2,
            0,
            0,
        );

        let second = AiOperationDescriptor::new(
            operation_name(),
            AiOperationKind::Inference,
            second_version,
        )
        .expect("valid second operation");

        builder
            .register_operations(vec![first, second])
            .expect("distinct versions");

        assert_eq!(builder.operation_count(), 2);
    }

    #[test]
    fn api_builder_preserves_existing_configuration() {
        let config = AiConfig::default();

        let api = AiApi::with_config(config.clone())
            .expect("valid API");

        let rebuilt = api
            .to_builder()
            .build()
            .expect("rebuilt API");

        assert_eq!(rebuilt.config(), &config);
    }

    #[test]
    fn api_clones_share_immutable_context_dependencies() {
        let api = AiApi::new()
            .expect("valid API");

        let clone = api.context().clone();

        assert!(Arc::ptr_eq(
            &api.context().shared_config(),
            &clone.shared_config(),
        ));

        assert!(Arc::ptr_eq(
            &api.context().shared_registry(),
            &clone.shared_registry(),
        ));
    }

    #[test]
    fn api_does_not_require_hardware_discovery() {
        let api = AiApi::new()
            .expect("valid API");

        assert!(api.operations_are_empty());

        // Construction requires no physical device information.
        // Execution feasibility is assessed downstream.
    }

    #[test]
    fn version_set_is_preserved() {
        let versions = AiVersionSet::initial();

        let api = AiApiBuilder::new()
            .versions(versions)
            .build()
            .expect("valid API");

        assert_eq!(api.versions(), versions);
    }

    #[test]
    fn registry_removal_does_not_mutate_existing_api() {
        let mut builder = AiApiBuilder::new();

        builder
            .register_operation(operation_descriptor())
            .expect("registration succeeds");

        let api = builder
            .build()
            .expect("API construction");

        let mut next_builder = api.to_builder();

        next_builder.remove_operation(
            &operation_name(),
            operation_version(),
        );

        let next_api = next_builder
            .build()
            .expect("next API construction");

        assert_eq!(api.operation_count(), 1);
        assert_eq!(next_api.operation_count(), 0);
    }
}