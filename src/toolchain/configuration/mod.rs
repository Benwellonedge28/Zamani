
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Configuration
//!
//! Path: `src/toolchain/configuration/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the public composition boundary for toolchain configuration.
//!
//! This module exposes the existing configuration authorities:
//!
//! - `budget`: canonical operational-budget integration.
//! - `config`: aggregate, namespaced toolchain configuration.
//! - `policies`: configuration policy definitions and resolution.
//! - `profiles`: configuration profiles and inheritance.
//! - `validation`: structural and extensible configuration validation.
//!
//! ## Ownership
//!
//! This module owns only:
//!
//! - Configuration module composition.
//! - Public configuration API exports.
//! - The configuration subsystem's public boundary.
//!
//! It does not implement a second configuration model, budget system,
//! policy engine, profile registry, semantic analyzer, capability
//! resolver, hardware discovery system, or resource allocator.
//!
//! ## Architectural boundaries
//!
//! `ToolchainConfig` stores configuration data. Domain-specific
//! configuration is interpreted by its authoritative subsystem.
//!
//! `ProfileRegistry` owns profile registration, inheritance and resolution.
//!
//! `PolicySet` owns policy definitions and deterministic policy composition.
//!
//! `ToolchainBudget` reuses the canonical compiler budget implementation.
//!
//! `ConfigurationValidatorRegistry` coordinates validation participants
//! supplied by authoritative subsystems and third-party extensions.
//!
//! ## Semantic separation
//!
//! The configuration subsystem preserves the distinctions between:
//!
//! - Requirements: what a program or operation needs.
//! - Capabilities: what an environment or provider can supply.
//! - Preferences: desirable compatible realizations.
//! - Constraints: conditions a valid realization must satisfy.
//! - Hints: advisory implementation information.
//! - Budgets: configurable operational allowances.
//! - Feasibility: whether a proposed realization satisfies applicable
//!   requirements, capabilities, constraints and budgets.
//!
//! This module does not convert one category into another.
//!
//! Source-language semantics remain owned by the frontend semantic layer.
//!
//! ## POCO-REAF and scalability
//!
//! This module introduces no fixed ceilings on:
//!
//! - Computing domains.
//! - Configuration sections.
//! - Configuration properties.
//! - Profiles or policies.
//! - Validation participants.
//! - Targets or providers.
//! - Processors, accelerators or quantum resources.
//! - Memory, storage or distributed participants.
//!
//! Collections grow according to available implementation resources.
//! Operational budgets remain explicit and configurable.
//!
//! Literal infinite computation, memory or storage is not promised.
//!
//! ## Determinism
//!
//! The underlying configuration, profile and policy implementations
//! use deterministic ordered collections where ordering is relevant.
//!
//! This module preserves those contracts and does not introduce
//! registration-order-dependent behavior.
//!
//! ## Security
//!
//! This module performs no filesystem access, network access, environment
//! lookup, process execution, hardware discovery or resource allocation.
//!
//! Configuration values and validation participants remain untrusted
//! until accepted by their respective authoritative validators.
//!
//! Credentials and private keys must use designated credential and
//! security providers rather than ordinary configuration sections.
//!
//! ## Integration
//!
//! Parent module:
//!     crate::toolchain
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! Toolchain identities:
//!     crate::toolchain::ids
//!
//! Compiler operational budgets:
//!     crate::compiler::configuration::budget
//!
//! Project manifests:
//!     crate::zamani_project_config
//!
//! Frontend semantics:
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
//! ## Compatibility
//!
//! Existing module paths remain available:
//!
//!     configuration::config
//!     configuration::budget
//!     configuration::policies
//!     configuration::profiles
//!     configuration::validation
//!
//! The re-exports below provide a stable convenience API without
//! removing access to the underlying modules.

/// Canonical operational-budget integration.
///
/// Reuses the compiler's budget implementation. This module does not
/// introduce an independent budget tracker or accounting system.
pub mod budget;

/// Aggregate, namespaced toolchain configuration.
///
/// Owns `ToolchainConfig`, configuration section identities and
/// configuration construction, serialization and structural validation.
pub mod config;

/// Configuration policy definitions and deterministic policy resolution.
///
/// Policy composition does not itself grant authorization or establish
/// target feasibility.
pub mod policies;

/// Configuration profile definitions, registration and inheritance.
///
/// Resolved profiles provide configuration overlays; they do not
/// allocate resources or select physical hardware.
pub mod profiles;

/// Configuration validation orchestration and extension participants.
///
/// Coordinates core, manifest, provider, environment and extension
/// validation without taking ownership of those subsystems' semantics.
pub mod validation;

// ============================================================================
// CANONICAL CONFIGURATION API
// ============================================================================

/// Namespaced configuration-section identity.
pub use config::ConfigSectionKey;

/// Aggregate toolchain configuration.
pub use config::ToolchainConfig;

/// Builder for aggregate toolchain configuration.
pub use config::ToolchainConfigBuilder;

// ============================================================================
// OPERATIONAL BUDGET API
// ============================================================================

/// Canonical compiler budget dimension.
pub use budget::BudgetDimension;

/// Canonical compiler budget limit.
pub use budget::BudgetLimit;

/// Canonical compiler budget configuration.
pub use budget::CompilationBudget;

/// Toolchain-facing budget configuration alias.
pub use budget::ToolchainBudget;

/// Toolchain-facing budget tracker alias.
pub use budget::ToolchainBudgetTracker;

/// Toolchain-facing budget request alias.
pub use budget::ToolchainBudgetRequest;

/// Toolchain-facing budget error alias.
pub use budget::ToolchainBudgetError;

/// Toolchain-facing budget result alias.
pub use budget::ToolchainBudgetResult;

/// Toolchain-facing budget snapshot alias.
pub use budget::ToolchainBudgetSnapshot;

/// Toolchain-facing budget usage alias.
pub use budget::ToolchainBudgetUsage;

/// Public version of the toolchain budget integration contract.
pub use budget::TOOLCHAIN_BUDGET_API_VERSION;

// ============================================================================
// POLICY API
// ============================================================================

/// Stable policy identity.
pub use policies::PolicyId;

/// Policy revision identity.
pub use policies::PolicyRevision;

/// Policy property identity.
pub use policies::PolicyPropertyKey;

/// Policy applicability scope.
pub use policies::PolicyScope;

/// A policy definition.
pub use policies::PolicyDefinition;

/// A collection of policy definitions.
pub use policies::PolicySet;

/// Provenance for a resolved policy property.
pub use policies::PolicyProvenance;

/// A resolved policy property.
pub use policies::PolicyProperty;

/// Resolved policy configuration.
pub use policies::ResolvedPolicy;

/// Canonical policy configuration section name.
pub use policies::TOOLCHAIN_POLICIES_SECTION;

/// Policy configuration API version.
pub use policies::POLICY_CONFIGURATION_API_VERSION;

// ============================================================================
// PROFILE API
// ============================================================================

/// Validated human-readable profile name.
pub use profiles::ProfileName;

/// A reusable profile definition.
pub use profiles::ProfileDefinition;

/// Registry of profile definitions.
pub use profiles::ProfileRegistry;

/// A profile resolved with its inherited configuration.
pub use profiles::ResolvedProfile;

// ============================================================================
// VALIDATION API
// ============================================================================

/// Authority responsible for a validation operation.
pub use validation::ValidationScope;

/// Contract implemented by configuration validation participants.
pub use validation::ConfigurationValidator;

/// Registry of configuration validation participants.
pub use validation::ConfigurationValidatorRegistry;

/// Immutable context supplied to configuration validation.
pub use validation::ConfigurationValidationContext;

/// One structured configuration-validation diagnostic.
pub use validation::ValidationDiagnostic;

/// Aggregate configuration-validation report.
pub use validation::ConfigurationValidationReport;

/// Validate configuration and collect all available diagnostics.
pub use validation::validate_configuration;

/// Validate configuration and return an error if validation fails.
pub use validation::validate_configuration_or_error;

// ============================================================================
// MODULE COMPOSITION CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::{
        validate_configuration,
        ConfigurationValidationContext,
        ToolchainConfig,
    };

    #[test]
    fn configuration_modules_are_composed() {
        let config = ToolchainConfig::new();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn empty_configuration_passes_core_validation() {
        let config = ToolchainConfig::default();
        let context = ConfigurationValidationContext::new();

        let report = validate_configuration(&config, &context);

        assert!(report.is_valid());
        assert_eq!(report.failure_count(), 0);
    }

    #[test]
    fn configuration_validation_does_not_require_a_target() {
        let config = ToolchainConfig::new();
        let context = ConfigurationValidationContext::new();

        let report = validate_configuration(&config, &context);

        assert!(report.is_valid());
    }
}
