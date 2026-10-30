
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Domain Context
//!
//! Path: src/ai/context.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provides an immutable, explicitly constructed dependency context for
//! Zamani's AI computational domain.
//!
//! The context connects existing AI-domain infrastructure without
//! implementing competing semantic, registry, configuration,
//! provenance, capability, resource, IR or execution systems.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - AI context construction;
//! - dependency composition;
//! - immutable configuration access;
//! - immutable operation-registry snapshots;
//! - AI version-set selection;
//! - optional shared semantic-service access;
//! - extension-policy access;
//! - provenance-policy access;
//! - context-level configuration validation.
//!
//! ## Does not own
//!
//! - source syntax, parsing or AST construction;
//! - semantic analysis or type checking;
//! - capability discovery or verification;
//! - resource allocation or feasibility;
//! - model execution;
//! - tensor storage;
//! - ZUIR or domain-specific IR definitions;
//! - compiler optimization or lowering;
//! - hardware discovery or device selection;
//! - distributed scheduling or transport;
//! - provenance persistence;
//! - security enforcement;
//! - global mutable state.
//!
//! ## Canonical integration
//!
//! Configuration:
//!     crate::ai::config::AiConfig
//!
//! Operation registry:
//!     crate::ai::registry::AiOperationRegistry
//!
//! Version contracts:
//!     crate::ai::version
//!
//! Frontend semantic services:
//!     crate::frontend::semantic::domains::ai::AiSemanticContext
//!
//! Provenance:
//!     crate::ai::provenance
//!     crate::frontend::semantic::model::provenance
//!
//! Capabilities:
//!     crate::ai::capabilities
//!     crate::frontend::semantic::capabilities
//!
//! Requirements:
//!     crate::ai::requirements
//!
//! Universal IR:
//!     crate::zuir
//!
//! Hybrid composition:
//!     crate::hybrid
//!
//! Distributed execution:
//!     crate::distributed
//!
//! ## POCO-REAF
//!
//! The context contains no hardware-specific assumptions.
//!
//! It imposes no universal limits on models, parameters, tensors,
//! agents, pipelines, workers, accelerators, devices or memory.
//!
//! Operational budgets remain explicit configuration values.
//!
//! The context does not promise literal infinite computation.
//! Execution remains dependent on implementation capabilities,
//! available resources and physical feasibility.
//!
//! ## Concurrency
//!
//! Context configuration and operation registries are shared through
//! immutable Arc references.
//!
//! Semantic services are borrowed explicitly.
//!
//! No global singleton or implicit mutable state is introduced.
//!
//! ## Integration principle
//!
//! The context composes existing authorities. It does not replace them.

use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::ai::config::{
    AiConfig,
    AiConfigError,
    AiExtensionPolicy,
};

use crate::ai::registry::AiOperationRegistry;

use crate::ai::version::{
    AiApiVersion,
    AiArtifactFormatVersion,
    AiModelSchemaVersion,
    AiOperationSetVersion,
    AiSemanticVersion,
    INITIAL_AI_API_VERSION,
    INITIAL_AI_ARTIFACT_FORMAT_VERSION,
    INITIAL_AI_MODEL_SCHEMA_VERSION,
    INITIAL_AI_OPERATION_SET_VERSION,
    INITIAL_AI_SEMANTIC_VERSION,
};

use crate::frontend::semantic::domains::ai::AiSemanticContext;

// ============================================================================
// VERSION SET
// ============================================================================

/// Independently versioned AI subsystem contracts.
///
/// These versions identify different compatibility domains.
///
/// They must not be interpreted as compiler, backend, hardware or
/// runtime versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AiVersionSet {
    /// Stable public AI API version.
    pub api: AiApiVersion,

    /// AI semantic contract version.
    pub semantic: AiSemanticVersion,

    /// AI model schema version.
    pub model_schema: AiModelSchemaVersion,

    /// AI operation-set version.
    pub operation_set: AiOperationSetVersion,

    /// AI artifact format version.
    pub artifact_format: AiArtifactFormatVersion,
}

impl AiVersionSet {
    /// Construct a version set from explicitly supplied versions.
    ///
    /// Each version belongs to its own compatibility domain.
    #[must_use]
    pub const fn new(
        api: AiApiVersion,
        semantic: AiSemanticVersion,
        model_schema: AiModelSchemaVersion,
        operation_set: AiOperationSetVersion,
        artifact_format: AiArtifactFormatVersion,
    ) -> Self {
        Self {
            api,
            semantic,
            model_schema,
            operation_set,
            artifact_format,
        }
    }

    /// Return the initial versions defined by the AI version authority.
    ///
    /// These are contract versions, not assumptions about hardware.
    #[must_use]
    pub const fn initial() -> Self {
        Self {
            api: INITIAL_AI_API_VERSION,
            semantic: INITIAL_AI_SEMANTIC_VERSION,
            model_schema: INITIAL_AI_MODEL_SCHEMA_VERSION,
            operation_set: INITIAL_AI_OPERATION_SET_VERSION,
            artifact_format: INITIAL_AI_ARTIFACT_FORMAT_VERSION,
        }
    }
}

impl Default for AiVersionSet {
    fn default() -> Self {
        Self::initial()
    }
}

// ============================================================================
// CONTEXT ERROR
// ============================================================================

/// Errors encountered while constructing an AI context.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AiContextError {
    /// The supplied AI configuration is invalid.
    InvalidConfiguration(AiConfigError),
}

impl fmt::Display for AiContextError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(error) => {
                write!(
                    formatter,
                    "invalid AI context configuration: {error}"
                )
            }
        }
    }
}

impl Error for AiContextError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidConfiguration(error) => Some(error),
        }
    }
}

impl From<AiConfigError> for AiContextError {
    fn from(error: AiConfigError) -> Self {
        Self::InvalidConfiguration(error)
    }
}

// ============================================================================
// AI DOMAIN CONTEXT
// ============================================================================

/// Immutable dependency context for AI-domain operations.
///
/// An `AiContext` provides access to established AI infrastructure.
///
/// It does not perform compilation, semantic analysis, model execution,
/// hardware discovery or resource allocation.
///
/// The context is safe to clone. Cloned contexts share the same immutable
/// configuration and operation-registry snapshot.
///
/// Semantic services are borrowed from their canonical owner and retain
/// their original lifetime.
pub struct AiContext<'services> {
    config: Arc<AiConfig>,

    registry: Arc<AiOperationRegistry>,

    versions: AiVersionSet,

    semantic_services:
        Option<&'services dyn AiSemanticContext>,
}

impl<'services> AiContext<'services> {
    /// Construct an AI context using the default configuration.
    ///
    /// The default configuration is validated before the context
    /// becomes available.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn new() -> Result<Self, AiContextError> {
        AiContextBuilder::new().build()
    }

    /// Construct a context using an explicitly supplied configuration.
    ///
    /// The operation registry starts empty. No operation is implicitly
    /// registered.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn with_config(
        config: AiConfig,
    ) -> Result<Self, AiContextError> {
        AiContextBuilder::new()
            .config(config)
            .build()
    }

    /// Return the immutable AI configuration.
    #[must_use]
    pub fn config(&self) -> &AiConfig {
        &self.config
    }

    /// Return the shared configuration handle.
    ///
    /// This allows other AI subsystems to retain the same immutable
    /// configuration without copying its contents.
    #[must_use]
    pub fn shared_config(&self) -> Arc<AiConfig> {
        Arc::clone(&self.config)
    }

    /// Return the immutable AI operation registry.
    #[must_use]
    pub fn registry(&self) -> &AiOperationRegistry {
        &self.registry
    }

    /// Return the shared immutable operation registry.
    ///
    /// Registry modifications require constructing a new context
    /// with a newly prepared registry.
    #[must_use]
    pub fn shared_registry(
        &self,
    ) -> Arc<AiOperationRegistry> {
        Arc::clone(&self.registry)
    }

    /// Return the AI version set.
    #[must_use]
    pub const fn versions(&self) -> AiVersionSet {
        self.versions
    }

    /// Return the optional canonical semantic services.
    ///
    /// Semantic services are not owned by the AI context.
    ///
    /// They remain under the authority of the frontend semantic subsystem.
    #[must_use]
    pub fn semantic_services(
        &self,
    ) -> Option<&'services dyn AiSemanticContext> {
        self.semantic_services
    }

    /// Return the configured AI extension policy.
    ///
    /// This exposes policy only.
    ///
    /// It does not load, register, authorize or execute extensions.
    #[must_use]
    pub fn extension_policy(&self) -> &AiExtensionPolicy {
        &self.config.extensions
    }

    /// Return whether reproducibility policy requires provenance.
    ///
    /// This reports the configured requirement. It does not establish
    /// that provenance has been collected or persisted.
    #[must_use]
    pub const fn provenance_required(&self) -> bool {
        self.config.reproducibility.require_provenance
    }

    /// Return whether the configured AI validation policy requires
    /// complete validation.
    #[must_use]
    pub const fn requires_complete_validation(&self) -> bool {
        self.config.validation.require_complete_validation
    }

    /// Return whether the context's configuration has no explicit
    /// operational budgets.
    ///
    /// This does not imply that the execution environment has unlimited
    /// resources.
    #[must_use]
    pub const fn has_no_explicit_budgets(&self) -> bool {
        self.config.budgets.has_no_explicit_budgets()
    }

    /// Return the number of registered AI operation declarations.
    #[must_use]
    pub fn registered_operation_count(&self) -> usize {
        self.registry.len()
    }

    /// Return whether the operation registry is empty.
    #[must_use]
    pub fn has_no_registered_operations(&self) -> bool {
        self.registry.is_empty()
    }

    /// Construct an independent builder using this context's settings.
    ///
    /// The builder receives a snapshot of the existing registry.
    ///
    /// This permits deliberate context replacement without modifying
    /// contexts that are already in use.
    #[must_use]
    pub fn to_builder(&self) -> AiContextBuilder<'services> {
        AiContextBuilder {
            config: self.config.as_ref().clone(),
            registry: self.registry.snapshot(),
            versions: self.versions,
            semantic_services: self.semantic_services,
        }
    }
}

// ============================================================================
// CONTEXT CLONING
// ============================================================================

impl<'services> Clone for AiContext<'services> {
    fn clone(&self) -> Self {
        Self {
            config: Arc::clone(&self.config),
            registry: Arc::clone(&self.registry),
            versions: self.versions,
            semantic_services: self.semantic_services,
        }
    }
}

// ============================================================================
// CONTEXT BUILDER
// ============================================================================

/// Builder for explicitly constructing an AI context.
///
/// The builder provides dependency injection without introducing
/// global state, implicit initialization or hardware discovery.
pub struct AiContextBuilder<'services> {
    config: AiConfig,

    registry: AiOperationRegistry,

    versions: AiVersionSet,

    semantic_services:
        Option<&'services dyn AiSemanticContext>,
}

impl<'services> AiContextBuilder<'services> {
    /// Construct a builder with default AI dependencies.
    ///
    /// The default operation registry is empty.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: AiConfig::default(),
            registry: AiOperationRegistry::new(),
            versions: AiVersionSet::initial(),
            semantic_services: None,
        }
    }

    /// Supply an AI configuration.
    ///
    /// The configuration is validated during `build`.
    #[must_use]
    pub fn config(
        mut self,
        config: AiConfig,
    ) -> Self {
        self.config = config;
        self
    }

    /// Supply an existing AI operation registry.
    ///
    /// The registry is transferred into the context and subsequently
    /// shared immutably.
    ///
    /// No additional operation is registered automatically.
    #[must_use]
    pub fn registry(
        mut self,
        registry: AiOperationRegistry,
    ) -> Self {
        self.registry = registry;
        self
    }

    /// Supply an explicit AI version set.
    #[must_use]
    pub fn versions(
        mut self,
        versions: AiVersionSet,
    ) -> Self {
        self.versions = versions;
        self
    }

    /// Supply canonical frontend AI semantic services.
    ///
    /// The reference is borrowed. The context cannot outlive the
    /// supplied semantic service.
    ///
    /// This does not replace the frontend semantic analyzer.
    #[must_use]
    pub fn semantic_services(
        mut self,
        services: &'services dyn AiSemanticContext,
    ) -> Self {
        self.semantic_services = Some(services);
        self
    }

    /// Build and validate the AI context.
    ///
    /// The context is created only after its configuration passes
    /// the validation rules owned by `AiConfig`.
    ///
    /// # Errors
    ///
    /// Returns an error when configuration validation fails.
    pub fn build(
        self,
    ) -> Result<AiContext<'services>, AiContextError> {
        self.config.validate()?;

        Ok(AiContext {
            config: Arc::new(self.config),

            registry: Arc::new(self.registry),

            versions: self.versions,

            semantic_services: self.semantic_services,
        })
    }
}

impl Default for AiContextBuilder<'_> {
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

    use crate::ai::config::{
        AiOperationalBudgets,
        AiValidationPolicy,
    };

    // ------------------------------------------------------------------------
    // Semantic service test implementation
    // ------------------------------------------------------------------------

    struct TestSemanticServices;

    impl AiSemanticContext for TestSemanticServices {
        fn validate_operation(
            &self,
            _operation: &crate::frontend::semantic::domains::ai::AiOperation,
        ) -> crate::frontend::semantic::domains::ai::ContextCheck {
            crate::frontend::semantic::domains::ai::ContextCheck::valid()
        }

        fn validate_value_type(
            &self,
            _value: &crate::frontend::semantic::domains::ai::AiValue,
        ) -> crate::frontend::semantic::domains::ai::ContextCheck {
            crate::frontend::semantic::domains::ai::ContextCheck::valid()
        }

        fn validate_dimension(
            &self,
            _dimension: &crate::frontend::semantic::domains::ai::DimensionExpr,
        ) -> crate::frontend::semantic::domains::ai::ContextCheck {
            crate::frontend::semantic::domains::ai::ContextCheck::valid()
        }

        fn validate_flow(
            &self,
            _flow: &crate::frontend::semantic::domains::ai::AiValueFlow,
            _value: &crate::frontend::semantic::domains::ai::AiValue,
            _producer: &crate::frontend::semantic::domains::ai::AiOperation,
            _consumer: &crate::frontend::semantic::domains::ai::AiOperation,
        ) -> crate::frontend::semantic::domains::ai::ContextCheck {
            crate::frontend::semantic::domains::ai::ContextCheck::valid()
        }

        fn validate_obligations(
            &self,
            _operation: &crate::frontend::semantic::domains::ai::AiOperation,
        ) -> crate::frontend::semantic::domains::ai::ContextCheck {
            crate::frontend::semantic::domains::ai::ContextCheck::valid()
        }
    }

    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    #[test]
    fn default_context_is_constructible() {
        let context = AiContext::new()
            .expect("default configuration must be valid");

        assert!(context.registry().is_empty());
    }

    #[test]
    fn explicit_configuration_is_preserved() {
        let config = AiConfig::default()
            .with_validation(AiValidationPolicy::strict());

        let context = AiContext::with_config(config.clone())
            .expect("valid configuration");

        assert_eq!(context.config(), &config);
    }

    #[test]
    fn default_builder_has_initial_versions() {
        let context = AiContextBuilder::new()
            .build()
            .expect("valid context");

        assert_eq!(
            context.versions(),
            AiVersionSet::initial()
        );
    }

    #[test]
    fn custom_versions_are_preserved() {
        let versions = AiVersionSet::new(
            AiApiVersion::new(2, 0, 0),
            AiSemanticVersion::new(2, 0, 0),
            AiModelSchemaVersion::new(3, 0, 0),
            AiOperationSetVersion::new(2, 1, 0),
            AiArtifactFormatVersion::new(4, 0, 0),
        );

        let context = AiContextBuilder::new()
            .versions(versions)
            .build()
            .expect("valid context");

        assert_eq!(context.versions(), versions);
    }

    // ------------------------------------------------------------------------
    // Configuration
    // ------------------------------------------------------------------------

    #[test]
    fn invalid_configuration_is_rejected() {
        let config = AiConfig::default()
            .with_extensions(
                AiExtensionPolicy::explicit()
                    .with_extension(" "),
            );

        let result = AiContext::with_config(config);

        assert!(matches!(
            result,
            Err(AiContextError::InvalidConfiguration(_))
        ));
    }

    #[test]
    fn operational_budgets_are_not_changed() {
        let budgets = AiOperationalBudgets::unlimited();

        let config = AiConfig::default()
            .with_budgets(budgets);

        let context = AiContext::with_config(config)
            .expect("valid configuration");

        assert!(context.has_no_explicit_budgets());
    }

    #[test]
    fn context_preserves_explicit_budget_configuration() {
        let budgets = AiOperationalBudgets::default()
            .with_operation_time(
                Some(std::time::Duration::from_secs(60))
            );

        let context = AiContext::with_config(
            AiConfig::default().with_budgets(budgets)
        )
        .expect("valid configuration");

        assert!(!context.has_no_explicit_budgets());
    }

    #[test]
    fn provenance_policy_is_exposed_without_claiming_collection() {
        let config = AiConfig::default()
            .with_reproducibility(
                crate::ai::config::AiReproducibilityPolicy {
                    require_provenance: true,
                    ..Default::default()
                }
            );

        let context = AiContext::with_config(config)
            .expect("valid configuration");

        assert!(context.provenance_required());
    }

    // ------------------------------------------------------------------------
    // Registry
    // ------------------------------------------------------------------------

    #[test]
    fn context_starts_with_an_empty_registry() {
        let context = AiContext::new()
            .expect("valid context");

        assert_eq!(context.registered_operation_count(), 0);
        assert!(context.has_no_registered_operations());
    }

    #[test]
    fn registry_can_be_injected() {
        let registry = AiOperationRegistry::new();

        let context = AiContextBuilder::new()
            .registry(registry)
            .build()
            .expect("valid context");

        assert!(context.registry().is_empty());
    }

    #[test]
    fn cloned_contexts_share_immutable_dependencies() {
        let context = AiContext::new()
            .expect("valid context");

        let clone = context.clone();

        assert!(Arc::ptr_eq(
            &context.shared_config(),
            &clone.shared_config(),
        ));

        assert!(Arc::ptr_eq(
            &context.shared_registry(),
            &clone.shared_registry(),
        ));
    }

    #[test]
    fn builder_from_context_preserves_configuration() {
        let config = AiConfig::default()
            .with_validation(AiValidationPolicy::strict());

        let context = AiContext::with_config(config.clone())
            .expect("valid context");

        let rebuilt = context.to_builder()
            .build()
            .expect("valid rebuilt context");

        assert_eq!(rebuilt.config(), &config);
        assert_eq!(rebuilt.versions(), context.versions());
    }

    // ------------------------------------------------------------------------
    // Semantic integration
    // ------------------------------------------------------------------------

    #[test]
    fn semantic_services_are_optional() {
        let context = AiContext::new()
            .expect("valid context");

        assert!(context.semantic_services().is_none());
    }

    #[test]
    fn canonical_semantic_services_can_be_injected() {
        let services = TestSemanticServices;

        let context = AiContextBuilder::new()
            .semantic_services(&services)
            .build()
            .expect("valid context");

        assert!(context.semantic_services().is_some());
    }

    // ------------------------------------------------------------------------
    // Scalability and portability
    // ------------------------------------------------------------------------

    #[test]
    fn context_does_not_require_hardware_information() {
        let context = AiContext::new()
            .expect("valid context");

        assert!(context.registry().is_empty());
        assert_eq!(
            context.versions(),
            AiVersionSet::initial()
        );
    }

    #[test]
    fn context_supports_large_version_components() {
        let versions = AiVersionSet::new(
            AiApiVersion::new(u32::MAX, 0, 0),
            AiSemanticVersion::new(1, u32::MAX, 0),
            AiModelSchemaVersion::new(1, 0, u32::MAX),
            AiOperationSetVersion::new(2, 0, 0),
            AiArtifactFormatVersion::new(3, 0, 0),
        );

        let context = AiContextBuilder::new()
            .versions(versions)
            .build()
            .expect("valid context");

        assert_eq!(context.versions(), versions);
    }
}
