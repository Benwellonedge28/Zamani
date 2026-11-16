
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Configuration Validation
//!
//! Path: src/toolchain/configuration/validation.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Coordinates complete validation of effective toolchain configuration.
//!
//! This module integrates:
//!
//! - Canonical ToolchainConfig.
//! - Configuration profiles and inheritance.
//! - Configuration policies.
//! - Canonical compiler operational budgets.
//! - Project-manifest validation participants.
//! - Provider configuration validation participants.
//! - Environment configuration validation participants.
//! - Third-party configuration extensions.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Configuration validation orchestration.
//! - Validation participant contracts.
//! - Validation participant registration.
//! - Validation scopes and ordering.
//! - Structured validation diagnostics.
//! - Validation reports.
//! - Effective configuration validation.
//!
//! ## Non-ownership
//!
//! This module does not own:
//!
//! - Configuration storage.
//! - Configuration serialization.
//! - Profile definitions.
//! - Profile inheritance algorithms.
//! - Policy definitions or policy composition.
//! - Source-language grammar.
//! - Semantic requirement evaluation.
//! - Capability discovery.
//! - Constraint solving.
//! - Preference scoring.
//! - Hint interpretation.
//! - Physical hardware discovery.
//! - Target selection.
//! - Feasibility determination.
//! - Resource allocation.
//! - Runtime scheduling.
//! - Backend generation.
//! - Security authorization.
//! - Filesystem access.
//! - Network access.
//! - Process execution.
//!
//! These responsibilities remain with their authoritative subsystems.
//!
//! ## Authoritative integrations
//!
//! Toolchain configuration:
//!     crate::toolchain::configuration::config
//!
//! Configuration profiles:
//!     crate::toolchain::configuration::profiles
//!
//! Configuration policies:
//!     crate::toolchain::configuration::policies
//!
//! Operational budgets:
//!     crate::toolchain::configuration::budget
//!
//! Canonical compiler budgets:
//!     crate::compiler::configuration::budget
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! Project manifest:
//!     crate::zamani_project_config
//!
//! Semantic authority:
//!     crate::frontend::semantic
//!
//! Backend and external tools:
//!     crate::backend
//!
//! Physical hardware:
//!     crate::zhal
//!
//! Runtime:
//!     crate::zur
//!
//! ## Grammar integration
//!
//! This module respects the following grammar authorities:
//!
//! - grammar/core/requirements.g4
//! - grammar/core/capabilities.g4
//! - grammar/core/constraints.g4
//! - grammar/core/hints.g4
//! - grammar/resources/requirements.g4
//! - grammar/resources/budgets.g4
//! - grammar/resources/preferences.g4
//! - grammar/resources/hints.g4
//! - grammar/policies/policy.g4
//! - grammar/policies/requirements.g4
//! - grammar/policies/constraints.g4
//!
//! Grammar files own source syntax.
//!
//! Frontend semantic analysis owns the interpretation of source constructs.
//!
//! This module validates toolchain configuration only.
//!
//! ## POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! No arbitrary universal ceilings are imposed on:
//!
//! - Computational domains.
//! - Providers.
//! - Targets.
//! - Processors.
//! - Accelerators.
//! - Quantum resources.
//! - Hardware resources.
//! - Memory.
//! - Storage.
//! - Distributed participants.
//! - Tensor dimensions.
//! - Configuration sections.
//! - Configuration properties.
//! - Validation participants.
//!
//! Practical processing and storage remain bounded by available
//! implementation resources and explicitly configured operational budgets.
//!
//! Literal infinite memory or computation is not promised.
//!
//! ## Fundamental semantic separation
//!
//! Requirement:
//!     What a program needs.
//!
//! Capability:
//!     What an environment can provide.
//!
//! Preference:
//!     Which compatible realization is desirable.
//!
//! Constraint:
//!     What a valid realization must satisfy.
//!
//! Hint:
//!     Advisory implementation information.
//!
//! Budget:
//!     An operational allowance.
//!
//! Feasibility:
//!     Whether a proposed realization satisfies the applicable conditions.
//!
//! These concepts MUST NOT be interchanged.
//!
//! ## Validation guarantees
//!
//! A successful report establishes that the registered validation
//! participants accepted the configuration they were given.
//!
//! It does not establish:
//!
//! - Universal target compatibility.
//! - Physical resource availability.
//! - Hardware reservation.
//! - Provider operational health beyond the evidence checked.
//! - Successful compilation.
//! - Successful deployment.
//! - Successful execution.
//!
//! Unknown capabilities must never be interpreted as confirmed capabilities.
//!
//! ## Concurrency
//!
//! The registry is immutable during validation through shared borrowing.
//!
//! Registered validators must implement Send + Sync.
//!
//! Implementations must not mutate shared configuration during validation.
//!
//! ## Security
//!
//! Validators must treat configuration values as untrusted input.
//!
//! Credentials and private keys must not be embedded in ordinary
//! configuration sections.
//!
//! Validation participants must not bypass their owning subsystem's
//! authorization or security boundaries.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use crate::toolchain::configuration::budget::ToolchainBudget;

use crate::toolchain::configuration::config::ToolchainConfig;

use crate::toolchain::configuration::policies::PolicySet;

use crate::toolchain::configuration::profiles::ProfileRegistry;

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainResult,
};

// ============================================================================
// VALIDATION SCOPE
// ============================================================================

/// Identifies the authority responsible for one validation operation.
///
/// Ordering is intentional and provides a deterministic validation pipeline.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ValidationScope {
    /// Canonical toolchain configuration and internal structure.
    Core,

    /// Project and manifest configuration.
    Manifest,

    /// Registered compiler, backend and domain providers.
    Provider,

    /// Environment and execution-platform configuration.
    Environment,

    /// Third-party and future configuration extensions.
    Extension,
}

impl ValidationScope {
    /// Return the stable machine-readable scope identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Core => "core",
            Self::Manifest => "manifest",
            Self::Provider => "provider",
            Self::Environment => "environment",
            Self::Extension => "extension",
        }
    }
}

impl fmt::Display for ValidationScope {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// VALIDATION PARTICIPANT CONTRACT
// ============================================================================

/// Extensible contract for configuration validation.
///
/// Implementations may adapt existing authoritative repository
/// subsystems or third-party provider contracts.
///
/// A participant must validate only the responsibilities it owns.
///
/// For example:
///
/// - A manifest participant validates project-manifest compatibility.
/// - A backend participant validates backend configuration.
/// - A hardware-provider participant validates provider configuration.
/// - An environment participant validates environment requirements.
///
/// A participant must not claim that a resource is physically available
/// merely because a configuration entry exists.
pub trait ConfigurationValidator: Send + Sync {
    /// Return a stable, namespaced validator identity.
    fn id(&self) -> &str;

    /// Return the validation authority represented by this participant.
    fn scope(&self) -> ValidationScope;

    /// Validate the effective toolchain configuration.
    ///
    /// The configuration is immutable for the duration of this call.
    ///
    /// Implementations must return an error when validation cannot
    /// establish that their owned configuration is acceptable.
    ///
    /// Unknown or unsupported configuration must not be accepted
    /// as confirmed feasibility.
    fn validate(
        &self,
        config: &ToolchainConfig,
    ) -> ToolchainResult<()>;
}

// ============================================================================
// VALIDATION PARTICIPANT REGISTRY
// ============================================================================

/// Registry of extensible configuration validators.
///
/// The registry uses stable identities and deterministic ordering.
///
/// It supports future providers and computing domains without requiring
/// modifications to this module.
///
/// Duplicate identities are rejected.
#[derive(Default)]
pub struct ConfigurationValidatorRegistry {
    validators: BTreeMap<
        String,
        Arc<dyn ConfigurationValidator>,
    >,
}

impl fmt::Debug for ConfigurationValidatorRegistry {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ConfigurationValidatorRegistry")
            .field(
                "validator_ids",
                &self.validators.keys().collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl ConfigurationValidatorRegistry {
    /// Construct an empty validator registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of registered validators.
    #[must_use]
    pub fn len(&self) -> usize {
        self.validators.len()
    }

    /// Return whether the registry contains no validators.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.validators.is_empty()
    }

    /// Register a validator.
    ///
    /// The validator is stored behind an Arc so it can be shared
    /// without duplicating its implementation.
    ///
    /// Registration is transactional.
    ///
    /// A duplicate identity is rejected without replacing the
    /// previously registered validator.
    pub fn register(
        &mut self,
        validator: Arc<dyn ConfigurationValidator>,
    ) -> ToolchainResult<()> {
        let id = validator.id();

        validate_validator_id(id)?;

        if self.validators.contains_key(id) {
            return Err(
                ToolchainError::invalid_configuration(
                    format!(
                        "configuration validator `{id}` is already registered"
                    ),
                ),
            );
        }

        self.validators.insert(
            id.to_owned(),
            validator,
        );

        Ok(())
    }

    /// Register a concrete validator implementation.
    pub fn register_validator<V>(
        &mut self,
        validator: V,
    ) -> ToolchainResult<()>
    where
        V: ConfigurationValidator + 'static,
    {
        self.register(Arc::new(validator))
    }

    /// Remove a validator by identity.
    ///
    /// Returns whether an entry was removed.
    pub fn remove(
        &mut self,
        id: &str,
    ) -> bool {
        self.validators.remove(id).is_some()
    }

    /// Check whether a validator is registered.
    #[must_use]
    pub fn contains(
        &self,
        id: &str,
    ) -> bool {
        self.validators.contains_key(id)
    }

    /// Return registered validator identities in deterministic order.
    #[must_use]
    pub fn validator_ids(
        &self,
    ) -> impl Iterator<Item = &str> {
        self.validators.keys().map(String::as_str)
    }

    fn validators_for(
        &self,
        scope: ValidationScope,
    ) -> impl Iterator<Item = &dyn ConfigurationValidator> {
        self.validators
            .values()
            .filter(move |validator| validator.scope() == scope)
            .map(Arc::as_ref)
    }
}

fn validate_validator_id(
    id: &str,
) -> ToolchainResult<()> {
    if id.is_empty() {
        return Err(
            ToolchainError::invalid_configuration(
                "configuration validator identity cannot be empty",
            ),
        );
    }

    if id.trim() != id {
        return Err(
            ToolchainError::invalid_configuration(
                "configuration validator identity cannot contain surrounding whitespace",
            ),
        );
    }

    if !id.chars().all(|character| {
        character.is_ascii_alphanumeric()
            || matches!(
                character,
                '.' | '_' | '-' | ':' | '/'
            )
    }) {
        return Err(
            ToolchainError::invalid_configuration(
                "configuration validator identity contains unsupported characters",
            ),
        );
    }

    Ok(())
}

// ============================================================================
// VALIDATION CONTEXT
// ============================================================================

/// Additional authoritative inputs for configuration validation.
///
/// The budget is borrowed from the canonical compiler budget implementation
/// through its toolchain-facing alias.
///
/// Validators may capture manifest, provider or environment references
/// in their own implementations. This avoids inventing replacement
/// manifest, hardware or environment data models.
///
/// No physical resource is discovered or allocated by this context.
#[derive(Default)]
pub struct ConfigurationValidationContext<'a> {
    budget: Option<&'a ToolchainBudget>,

    validators: Option<&'a ConfigurationValidatorRegistry>,
}

impl fmt::Debug for ConfigurationValidationContext<'_> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ConfigurationValidationContext")
            .field(
                "has_budget",
                &self.budget.is_some(),
            )
            .field(
                "has_validator_registry",
                &self.validators.is_some(),
            )
            .finish()
    }
}

impl<'a> ConfigurationValidationContext<'a> {
    /// Construct an empty validation context.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            budget: None,
            validators: None,
        }
    }

    /// Supply the canonical operational budget.
    ///
    /// This validates configuration compatibility only.
    ///
    /// It does not consume or reserve any budget.
    #[must_use]
    pub const fn with_budget(
        mut self,
        budget: &'a ToolchainBudget,
    ) -> Self {
        self.budget = Some(budget);
        self
    }

    /// Supply the registered manifest, provider, environment
    /// and extension validation participants.
    #[must_use]
    pub const fn with_validators(
        mut self,
        validators: &'a ConfigurationValidatorRegistry,
    ) -> Self {
        self.validators = Some(validators);
        self
    }

    /// Return the supplied operational budget, if any.
    #[must_use]
    pub const fn budget(
        &self,
    ) -> Option<&'a ToolchainBudget> {
        self.budget
    }

    /// Return the supplied validator registry, if any.
    #[must_use]
    pub const fn validators(
        &self,
    ) -> Option<&'a ConfigurationValidatorRegistry> {
        self.validators
    }
}

// ============================================================================
// VALIDATION DIAGNOSTICS
// ============================================================================

/// Structured information about one failed validation operation.
///
/// The original error code and context are preserved.
///
/// Sensitive error source objects are not serialized into reports.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationDiagnostic {
    scope: ValidationScope,

    validator_id: String,

    code: String,

    message: String,

    context: Vec<String>,
}

impl ValidationDiagnostic {
    fn from_error(
        scope: ValidationScope,
        validator_id: impl Into<String>,
        error: &ToolchainError,
    ) -> Self {
        Self {
            scope,
            validator_id: validator_id.into(),
            code: error.code().to_owned(),
            message: error.message().to_owned(),
            context: error.context().to_vec(),
        }
    }

    /// Return the responsible validation scope.
    #[must_use]
    pub const fn scope(&self) -> ValidationScope {
        self.scope
    }

    /// Return the validator identity.
    #[must_use]
    pub fn validator_id(&self) -> &str {
        &self.validator_id
    }

    /// Return the original stable error code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Return the diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return additional diagnostic context.
    #[must_use]
    pub fn context(&self) -> &[String] {
        &self.context
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Complete result of configuration validation.
///
/// An empty diagnostic collection means that all applicable validation
/// operations completed successfully.
///
/// A nonempty collection means that at least one validation operation failed.
///
/// A report does not certify physical feasibility.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConfigurationValidationReport {
    diagnostics: Vec<ValidationDiagnostic>,
}

impl ConfigurationValidationReport {
    /// Construct an empty validation report.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return all diagnostics in deterministic validation order.
    #[must_use]
    pub fn diagnostics(
        &self,
    ) -> &[ValidationDiagnostic] {
        &self.diagnostics
    }

    /// Return the number of validation failures.
    #[must_use]
    pub fn failure_count(&self) -> usize {
        self.diagnostics.len()
    }

    /// Return whether all applicable validation operations succeeded.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Return whether any validation operation failed.
    #[must_use]
    pub fn has_failures(&self) -> bool {
        !self.is_valid()
    }

    /// Consume the report and return an error if validation failed.
    ///
    /// All recorded failures are included in the resulting message.
    ///
    /// The original diagnostic codes remain available in the report
    /// before this conversion.
    pub fn into_result(
        self,
    ) -> ToolchainResult<Self> {
        if self.is_valid() {
            return Ok(self);
        }

        let summary = self
            .diagnostics
            .iter()
            .map(|diagnostic| {
                format!(
                    "[{}] {}: {}",
                    diagnostic.code(),
                    diagnostic.validator_id(),
                    diagnostic.message(),
                )
            })
            .collect::<Vec<_>>()
            .join("; ");

        Err(
            ToolchainError::validation_failed(
                format!(
                    "toolchain configuration validation failed: {summary}"
                ),
            ),
        )
    }

    fn record(
        &mut self,
        scope: ValidationScope,
        validator_id: impl Into<String>,
        error: ToolchainError,
    ) {
        self.diagnostics.push(
            ValidationDiagnostic::from_error(
                scope,
                validator_id,
                &error,
            ),
        );
    }

    fn record_result(
        &mut self,
        scope: ValidationScope,
        validator_id: impl Into<String>,
        result: ToolchainResult<()>,
    ) {
        if let Err(error) = result {
            self.record(
                scope,
                validator_id,
                error,
            );
        }
    }
}

// ============================================================================
// BUILT-IN VALIDATION
// ============================================================================

/// Validate the canonical structure of a toolchain configuration.
///
/// This delegates to the existing ToolchainConfig implementation.
fn validate_core(
    config: &ToolchainConfig,
    report: &mut ConfigurationValidationReport,
) {
    report.record_result(
        ValidationScope::Core,
        "toolchain.configuration.structure",
        config.validate(),
    );
}

/// Resolve and validate the selected configuration profile.
///
/// The profile registry is loaded from the canonical ToolchainConfig.
///
/// Profile inheritance, cycle detection, name-index integrity and
/// disabled-profile checks remain owned by ProfileRegistry.
///
/// The selected profile is applied to a temporary effective configuration.
///
/// The caller's original configuration is never modified.
fn validate_profiles(
    config: &ToolchainConfig,
    report: &mut ConfigurationValidationReport,
) -> ToolchainConfig {
    let mut effective = config.clone();

    let registry = match ProfileRegistry::load_from_config(config) {
        Ok(registry) => registry,

        Err(error) => {
            report.record(
                ValidationScope::Core,
                "toolchain.configuration.profiles.registry",
                error,
            );

            return effective;
        }
    };

    if let Err(error) = registry.validate() {
        report.record(
            ValidationScope::Core,
            "toolchain.configuration.profiles.registry",
            error,
        );

        return effective;
    }

    let Some(profile_id) = config.profile() else {
        return effective;
    };

    match registry.resolve(profile_id) {
        Ok(resolved) => {
            if let Err(error) = resolved.apply_to(&mut effective) {
                report.record(
                    ValidationScope::Core,
                    "toolchain.configuration.profiles.application",
                    error,
                );
            }
        }

        Err(error) => {
            report.record(
                ValidationScope::Core,
                "toolchain.configuration.profiles.resolution",
                error,
            );
        }
    }

    effective
}

/// Validate configuration policy definitions and composition.
///
/// Policy validation remains delegated to PolicySet.
///
/// Resolving global policies checks their composition without claiming
/// that a target-specific policy has been applied to a target.
///
/// Target-specific policy resolution remains the responsibility of the
/// operation that has an explicit target context.
fn validate_policies(
    config: &ToolchainConfig,
    report: &mut ConfigurationValidationReport,
) {
    let policies = match PolicySet::from_config(config) {
        Ok(policies) => policies,

        Err(error) => {
            report.record(
                ValidationScope::Core,
                "toolchain.configuration.policies",
                error,
            );

            return;
        }
    };

    if let Err(error) = policies.validate() {
        report.record(
            ValidationScope::Core,
            "toolchain.configuration.policies.structure",
            error,
        );

        return;
    }

    report.record_result(
        ValidationScope::Core,
        "toolchain.configuration.policies.composition",
        policies.resolve(None).map(|_| ()),
    );
}

/// Validate the supplied canonical operational budget.
///
/// CompilationBudget owns dimension identity and limit validity.
///
/// This coordinator does not reinterpret budgets as hardware capabilities
/// or semantic requirements.
///
/// An explicitly configured zero budget is valid configuration.
fn validate_budget(
    budget: &ToolchainBudget,
    report: &mut ConfigurationValidationReport,
) {
    // Budget dimensions and limits are validated by their constructors.
    // Their internal representation is private.
    //
    // Inspecting the configured limits ensures the supplied budget is
    // connected to this validation operation without imposing additional
    // universal limits or changing its accounting semantics.

    for (dimension, limit) in budget.limits() {
        if dimension.name().trim().is_empty()
            || dimension.unit().trim().is_empty()
        {
            report.record(
                ValidationScope::Core,
                "toolchain.configuration.budget",
                ToolchainError::invalid_configuration(
                    "operational budget contains an invalid dimension",
                ),
            );

            return;
        }

        // A zero value is a valid explicit operational limit.
        // None represents an unlimited budget for that dimension.
        let _configured_limit = limit.value();
    }
}

// ============================================================================
// EXTENSIBLE VALIDATION
// ============================================================================

/// Execute all registered validators for one scope.
///
/// The registry uses ordered identities, so validator execution order
/// is deterministic within each scope.
///
/// Each validator receives the effective configuration.
///
/// A validator failure is recorded without converting it into success.
fn validate_scope(
    config: &ToolchainConfig,
    registry: &ConfigurationValidatorRegistry,
    scope: ValidationScope,
    report: &mut ConfigurationValidationReport,
) {
    for validator in registry.validators_for(scope) {
        let id = validator.id();

        let result = validator.validate(config);

        report.record_result(
            scope,
            id.to_owned(),
            result,
        );
    }
}

// ============================================================================
// PUBLIC VALIDATION API
// ============================================================================

/// Validate a complete toolchain configuration.
///
/// Validation order:
///
/// 1. Canonical configuration structure.
/// 2. Profile registry and selected-profile resolution.
/// 3. Effective configuration structure.
/// 4. Policy definitions and global policy composition.
/// 5. Supplied operational budget.
/// 6. Manifest validation participants.
/// 7. Provider validation participants.
/// 8. Environment validation participants.
/// 9. Third-party extension validation participants.
///
/// All applicable validations are attempted even if an earlier
/// independent validation fails.
///
/// The resulting report preserves every recorded failure.
///
/// This function does not modify the supplied configuration.
///
/// It does not perform filesystem, network, process or hardware operations
/// unless a registered participant explicitly owns and performs such
/// an operation. Participants should normally validate supplied snapshots
/// rather than initiate external discovery.
pub fn validate_configuration(
    config: &ToolchainConfig,
    context: &ConfigurationValidationContext<'_>,
) -> ConfigurationValidationReport {
    let mut report = ConfigurationValidationReport::new();

    validate_core(
        config,
        &mut report,
    );

    let effective = validate_profiles(
        config,
        &mut report,
    );

    report.record_result(
        ValidationScope::Core,
        "toolchain.configuration.effective",
        effective.validate(),
    );

    validate_policies(
        &effective,
        &mut report,
    );

    if let Some(budget) = context.budget() {
        validate_budget(
            budget,
            &mut report,
        );
    }

    if let Some(registry) = context.validators() {
        validate_scope(
            &effective,
            registry,
            ValidationScope::Manifest,
            &mut report,
        );

        validate_scope(
            &effective,
            registry,
            ValidationScope::Provider,
            &mut report,
        );

        validate_scope(
            &effective,
            registry,
            ValidationScope::Environment,
            &mut report,
        );

        validate_scope(
            &effective,
            registry,
            ValidationScope::Extension,
            &mut report,
        );
    }

    report
}

/// Validate configuration and require a successful result.
///
/// Unlike [`validate_configuration`], this function returns an error
/// when one or more validation operations fail.
///
/// Use the report-returning function when callers need every diagnostic.
pub fn validate_configuration_or_error(
    config: &ToolchainConfig,
    context: &ConfigurationValidationContext<'_>,
) -> ToolchainResult<ConfigurationValidationReport> {
    validate_configuration(
        config,
        context,
    )
    .into_result()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct AcceptingValidator {
        id: String,
        scope: ValidationScope,
    }

    impl ConfigurationValidator for AcceptingValidator {
        fn id(&self) -> &str {
            &self.id
        }

        fn scope(&self) -> ValidationScope {
            self.scope
        }

        fn validate(
            &self,
            _config: &ToolchainConfig,
        ) -> ToolchainResult<()> {
            Ok(())
        }
    }

    struct RejectingValidator;

    impl ConfigurationValidator for RejectingValidator {
        fn id(&self) -> &str {
            "test.provider.rejecting"
        }

        fn scope(&self) -> ValidationScope {
            ValidationScope::Provider
        }

        fn validate(
            &self,
            _config: &ToolchainConfig,
        ) -> ToolchainResult<()> {
            Err(
                ToolchainError::provider_failed(
                    "test provider rejected its configuration",
                ),
            )
        }
    }

    #[test]
    fn empty_configuration_is_valid() {
        let config = ToolchainConfig::new();

        let context = ConfigurationValidationContext::new();

        let report = validate_configuration(
            &config,
            &context,
        );

        assert!(report.is_valid());
        assert_eq!(report.failure_count(), 0);
    }

    #[test]
    fn validator_registry_starts_empty() {
        let registry = ConfigurationValidatorRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn registers_open_world_validator_identities() {
        let mut registry =
            ConfigurationValidatorRegistry::new();

        registry
            .register_validator(AcceptingValidator {
                id: "future.domain.provider.validator".to_owned(),
                scope: ValidationScope::Provider,
            })
            .expect("registration should succeed");

        assert_eq!(registry.len(), 1);

        assert!(
            registry.contains(
                "future.domain.provider.validator"
            )
        );
    }

    #[test]
    fn rejects_duplicate_validator_identities() {
        let mut registry =
            ConfigurationValidatorRegistry::new();

        let first = AcceptingValidator {
            id: "provider.validation".to_owned(),
            scope: ValidationScope::Provider,
        };

        let second = AcceptingValidator {
            id: "provider.validation".to_owned(),
            scope: ValidationScope::Environment,
        };

        registry
            .register_validator(first)
            .expect("first registration should succeed");

        assert!(
            registry.register_validator(second).is_err()
        );

        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn rejects_invalid_validator_identities() {
        let mut registry =
            ConfigurationValidatorRegistry::new();

        let validator = AcceptingValidator {
            id: " invalid identity ".to_owned(),
            scope: ValidationScope::Extension,
        };

        assert!(
            registry.register_validator(validator).is_err()
        );

        assert!(registry.is_empty());
    }

    #[test]
    fn removes_registered_validators() {
        let mut registry =
            ConfigurationValidatorRegistry::new();

        registry
            .register_validator(AcceptingValidator {
                id: "provider.one".to_owned(),
                scope: ValidationScope::Provider,
            })
            .expect("registration should succeed");

        assert!(registry.remove("provider.one"));
        assert!(!registry.remove("provider.one"));
        assert!(registry.is_empty());
    }

    #[test]
    fn accepting_validator_preserves_validity() {
        let mut registry =
            ConfigurationValidatorRegistry::new();

        registry
            .register_validator(AcceptingValidator {
                id: "environment.host".to_owned(),
                scope: ValidationScope::Environment,
            })
            .expect("registration should succeed");

        let config = ToolchainConfig::new();

        let context =
            ConfigurationValidationContext::new()
                .with_validators(&registry);

        let report = validate_configuration(
            &config,
            &context,
        );

        assert!(report.is_valid());
    }

    #[test]
    fn rejecting_validator_produces_structured_diagnostic() {
        let mut registry =
            ConfigurationValidatorRegistry::new();

        registry
            .register_validator(RejectingValidator)
            .expect("registration should succeed");

        let config = ToolchainConfig::new();

        let context =
            ConfigurationValidationContext::new()
                .with_validators(&registry);

        let report = validate_configuration(
            &config,
            &context,
        );

        assert!(!report.is_valid());
        assert_eq!(report.failure_count(), 1);

        let diagnostic = &report.diagnostics()[0];

        assert_eq!(
            diagnostic.scope(),
            ValidationScope::Provider,
        );

        assert_eq!(
            diagnostic.validator_id(),
            "test.provider.rejecting",
        );

        assert_eq!(
            diagnostic.code(),
            "ZT0014",
        );
    }

    #[test]
    fn failed_report_cannot_be_converted_into_success() {
        let mut registry =
            ConfigurationValidatorRegistry::new();

        registry
            .register_validator(RejectingValidator)
            .expect("registration should succeed");

        let config = ToolchainConfig::new();

        let context =
            ConfigurationValidationContext::new()
                .with_validators(&registry);

        let report = validate_configuration(
            &config,
            &context,
        );

        assert!(report.into_result().is_err());
    }

    #[test]
    fn validation_does_not_mutate_original_configuration() {
        let config = ToolchainConfig::new();

        let original = config.clone();

        let context = ConfigurationValidationContext::new();

        let _report = validate_configuration(
            &config,
            &context,
        );

        assert_eq!(config, original);
    }

    #[test]
    fn validation_scopes_have_stable_order() {
        assert!(
            ValidationScope::Core
                < ValidationScope::Manifest
        );

        assert!(
            ValidationScope::Manifest
                < ValidationScope::Provider
        );

        assert!(
            ValidationScope::Provider
                < ValidationScope::Environment
        );

        assert!(
            ValidationScope::Environment
                < ValidationScope::Extension
        );
    }

    #[test]
    fn validation_report_starts_valid() {
        let report = ConfigurationValidationReport::new();

        assert!(report.is_valid());
        assert!(!report.has_failures());
        assert_eq!(report.failure_count(), 0);
    }

    #[test]
    fn successful_report_converts_to_result() {
        let report = ConfigurationValidationReport::new();

        assert!(report.into_result().is_ok());
    }

    #[test]
    fn validator_iteration_is_deterministic() {
        let mut registry =
            ConfigurationValidatorRegistry::new();

        for id in [
            "provider.z",
            "provider.a",
            "provider.m",
        ] {
            registry
                .register_validator(AcceptingValidator {
                    id: id.to_owned(),
                    scope: ValidationScope::Provider,
                })
                .expect("registration should succeed");
        }

        let ids = registry
            .validator_ids()
            .collect::<Vec<_>>();

        assert_eq!(
            ids,
            vec![
                "provider.a",
                "provider.m",
                "provider.z",
            ],
        );
    }
}
