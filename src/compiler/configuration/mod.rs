
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Universal Meta-Compiler — Configuration Module
//!
//! Path:
//!     src/compiler/configuration/mod.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module is the public composition boundary for compiler configuration.
//!
//! It exposes the independently-owned configuration contracts required by
//! the Zamani compiler without introducing competing configuration models.
//!
//! The module composes:
//!
//!     budget.rs
//!         Operational compilation budgets and shared accounting.
//!
//!     config.rs
//!         Construction and validation of the existing CompilerConfig.
//!
//!     policy.rs
//!         Compiler-side policy representation and evaluation boundary.
//!
//!     profile.rs
//!         Named compilation profiles, registration, and resolution.
//!
//!     target.rs
//!         Extensible target descriptors and legacy-target adaptation.
//!
//!     validation.rs
//!         Aggregate validation across configuration contracts.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! The canonical ownership boundaries are:
//!
//!     crate::compiler_types
//!         CompilerConfig
//!         CompilationTarget
//!         OptimizationLevel
//!
//!     configuration::budget
//!         CompilationBudget
//!         BudgetDimension
//!         BudgetTracker
//!         BudgetUsage
//!         BudgetSnapshot
//!
//!     configuration::config
//!         CompilerConfigBuilder
//!         ConfigurationError
//!         ConfigurationValidationMode
//!         CompilerConfig validation
//!
//!     configuration::policy
//!         PolicyDocument
//!         PolicyRule
//!         PolicyDirective
//!         PolicyExpression
//!         Policy evaluation contracts
//!
//!     configuration::profile
//!         CompilationProfileId
//!         CompilationProfileSpec
//!         CompilationProfileRegistry
//!         ResolvedCompilationProfile
//!
//!     configuration::target
//!         TargetId
//!         TargetDescriptor
//!         TargetProperty
//!         Target validation
//!
//!     configuration::validation
//!         ConfigurationValidationRequest
//!         ConfigurationValidationReport
//!         ConfigurationValidationFailure
//!         Aggregate validation
//!
//! This module owns only:
//!
//!     - Public module composition.
//!     - Stable configuration API re-exports.
//!     - Configuration API version identification.
//!     - The configuration subsystem's documented integration boundary.
//!
//! It does not own:
//!
//!     - Source parsing or grammar implementation.
//!     - AST construction.
//!     - Semantic analysis.
//!     - Type inference.
//!     - Ownership or borrow checking.
//!     - Capability discovery.
//!     - Hardware discovery.
//!     - Backend selection.
//!     - Optimization implementation.
//!     - IR definitions.
//!     - Physical resource allocation.
//!     - Runtime scheduling.
//!     - Device execution.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Configuration describes compilation intent and operational policy.
//! It must not impose universal machine-capacity ceilings.
//!
//! In particular, this module defines no fixed maximum for:
//!
//!     - CPUs, cores, threads, or workers;
//!     - GPUs, FPGAs, QPUs, or accelerators;
//!     - nodes, devices, or processing elements;
//!     - memory, storage, or address-space capacity;
//!     - qubits or logical qubits;
//!     - tensor rank or tensor dimensions;
//!     - modules, functions, operations, or IR entities;
//!     - compilation stages or optimization passes;
//!     - profiles, targets, policies, or artifacts.
//!
//! Operational limits are represented by CompilationBudget.
//!
//! An absent budget means that no explicit compiler budget is configured
//! for that dimension. It does not promise infinite physical resources.
//!
//! Hardware feasibility belongs to target capabilities, resource planning,
//! backend compatibility, and runtime resource management.
//!
//! ============================================================================
//! GRAMMAR AND FRONTEND BOUNDARY
//! ============================================================================
//!
//! The source-level grammar remains authoritative for syntax:
//!
//!     grammar/compile/profiles.g4
//!         Named compilation-profile syntax.
//!
//!     grammar/compile/optimization.g4
//!         Optimization intent and directives.
//!
//!     grammar/compile/compile.g4
//!         Compilation syntax composition.
//!
//!     grammar/DESIGN.md
//!         Grammar architecture and ownership contract.
//!
//! The frontend owns parsing and AST construction.
//!
//! Semantic analysis owns:
//!
//!     - Name resolution.
//!     - Profile inheritance and composition semantics.
//!     - Requirement and constraint validation.
//!     - Capability meaning.
//!     - Policy authorization and enforcement semantics.
//!     - Source-level type and effect validation.
//!
//! This module exposes compiler-side configuration contracts. It does not
//! parse grammar files or implement a second semantic model.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! Parent module:
//!
//!     src/compiler/mod.rs
//!
//! Must declare:
//!
//!     pub mod configuration;
//!
//! Existing public crate entry:
//!
//!     src/lib.rs
//!
//! Already declares:
//!
//!     pub mod compiler;
//!
//! No additional declaration in src/lib.rs is required.
//!
//! Compiler orchestration should use:
//!
//!     crate::compiler::configuration::validation
//!
//! for aggregate validation before starting stages that depend on the
//! supplied configuration.
//!
//! Profile resolution should use:
//!
//!     crate::compiler::configuration::profile
//!
//! Target adaptation should use:
//!
//!     crate::compiler::configuration::target
//!
//! Budget accounting should use:
//!
//!     crate::compiler::configuration::budget
//!
//! Policy handling should use:
//!
//!     crate::compiler::configuration::policy
//!
//! Configuration construction and compatibility validation should use:
//!
//!     crate::compiler::configuration::config
//!
//! No configuration module may silently replace caller-supplied values.
//!
//! No configuration module may claim that a target is feasible merely
//! because a target descriptor is structurally valid.
//!
//! No profile may claim that a recommended pass is implemented merely
//! because its identifier is present in the profile.
//!
//! ============================================================================
//! DETERMINISM AND CONCURRENCY
//! ============================================================================
//!
//! This module performs no global mutation.
//!
//! Registries are instance-owned.
//!
//! Ordered collections and deterministic behavior are provided by the
//! authoritative child modules.
//!
//! This module does not:
//!
//!     - Spawn threads.
//!     - Discover hardware.
//!     - Read environment variables.
//!     - Read or write files.
//!     - Access the network.
//!     - Start compiler stages.
//!     - Allocate physical resources.
//!
//! Independent compiler sessions can therefore use independent configuration
//! objects and registries without relying on a global mutable configuration.
//!
//! ============================================================================
//! ERROR CONTRACT
//! ============================================================================
//!
//! Errors are defined by the module that owns the relevant contract.
//!
//! Examples:
//!
//!     ConfigurationError
//!         Invalid or unsupported CompilerConfig.
//!
//!     BudgetError
//!         Invalid budget operation or exhausted budget.
//!
//!     PolicyValidationError
//!         Structurally invalid policy document.
//!
//!     PolicyEvaluationError
//!         Policy expression evaluation failure.
//!
//!     ProfileError
//!         Invalid, unknown, or incompatible profile.
//!
//!     TargetError
//!         Invalid target descriptor.
//!
//!     ConfigurationValidationFailure
//!         Aggregate configuration validation failure.
//!
//! This module does not flatten these errors into strings or discard
//! their structured information.
//!
//! ============================================================================
//! COMPATIBILITY
//! ============================================================================
//!
//! The existing public types remain owned by their original modules.
//!
//! Existing callers can continue using:
//!
//!     crate::compiler_types::CompilerConfig
//!     crate::compiler_types::CompilationTarget
//!     crate::compiler_types::OptimizationLevel
//!
//! Existing module-qualified paths remain available:
//!
//!     crate::compiler::configuration::config
//!     crate::compiler::configuration::budget
//!     crate::compiler::configuration::policy
//!     crate::compiler::configuration::profile
//!     crate::compiler::configuration::target
//!     crate::compiler::configuration::validation
//!
//! Re-exports below provide a convenient public facade without removing
//! those original paths.
//!
//! ============================================================================
//! MODULES
//! ============================================================================

/// Operational compilation budgets and shared budget accounting.
pub mod budget;

/// Canonical CompilerConfig construction and validation.
pub mod config;

/// Compiler-side policy contracts and evaluation interfaces.
pub mod policy;

/// Named compilation profiles and deterministic profile resolution.
pub mod profile;

/// Extensible target descriptors and legacy target adaptation.
pub mod target;

/// Aggregate validation of compiler configuration contracts.
pub mod validation;

// ============================================================================
// API VERSION
// ============================================================================

/// Version of the public configuration-module composition contract.
///
/// This is independent of:
///
/// - The Zamani language version.
/// - The compiler release version.
/// - The grammar version.
/// - The semantic-model version.
/// - The IR version.
/// - The backend ABI version.
/// - The target descriptor API version.
///
/// Increment this version only when the public composition contract
/// changes incompatibly.
pub const CONFIGURATION_MODULE_API_VERSION: u16 = 1;

// ============================================================================
// BUDGET PUBLIC API
// ============================================================================

pub use self::budget::{
    BudgetDimension,
    BudgetError,
    BudgetLimit,
    BudgetResult,
    BudgetSnapshot,
    BudgetTracker,
    BudgetUsage,
    CompilationBudget,
};

// ============================================================================
// CONFIGURATION PUBLIC API
// ============================================================================

pub use self::config::{
    is_production_optimization_supported,
    validate_compiler_config,
    CompilerConfigBuilder,
    ConfigurationError,
    ConfigurationValidationMode,
    CONFIGURATION_API_VERSION,
};

// ============================================================================
// POLICY PUBLIC API
// ============================================================================

pub use self::policy::{
    PolicyAction,
    PolicyActionOrigin,
    PolicyClass,
    PolicyDirective,
    PolicyDirectiveKind,
    PolicyDocument,
    PolicyEvaluation,
    PolicyEvaluationError,
    PolicyExpression,
    PolicyExpressionEvaluator,
    PolicyId,
    PolicyMember,
    PolicyPropertyKey,
    PolicyRule,
    PolicyRuleClause,
    PolicyStrength,
    PolicyValidationError,
    TruthValue,
    POLICY_API_VERSION,
};

// ============================================================================
// PROFILE PUBLIC API
// ============================================================================

pub use self::profile::{
    is_builtin_profile,
    resolve_standard_profile,
    standard_profile_registry,
    validate_profile_compatibility,
    CompilationProfileId,
    CompilationProfileRegistry,
    CompilationProfileSpec,
    ProfileEffort,
    ProfileError,
    ProfileResult,
    ProfileSafety,
    ResolvedCompilationProfile,
    PROFILE_API_VERSION,
    PROFILE_SCHEMA_VERSION,
};

// ============================================================================
// TARGET PUBLIC API
// ============================================================================

pub use self::target::{
    ArtifactFormat,
    TargetDescriptor,
    TargetDescriptorBuilder,
    TargetError,
    TargetId,
    TargetProperty,
    TargetRole,
    TARGET_DESCRIPTOR_API_VERSION,
};

// ============================================================================
// VALIDATION PUBLIC API
// ============================================================================

pub use self::validation::{
    is_production_configuration_accepted,
    validate_compilation_configuration,
    validate_compilation_configuration_with_contracts,
    ConfigurationValidationFailure,
    ConfigurationValidationReport,
    ConfigurationValidationRequest,
    ValidationComponent,
    ValidationFinding,
    ValidationInputError,
    ValidationSeverity,
    CONFIGURATION_VALIDATION_API_VERSION,
};

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler_types::{
        CompilationTarget,
        CompilerConfig,
        OptimizationLevel,
    };

    #[test]
    fn configuration_module_api_version_is_nonzero() {
        assert_ne!(CONFIGURATION_MODULE_API_VERSION, 0);
    }

    #[test]
    fn all_configuration_contract_versions_are_nonzero() {
        assert_ne!(CONFIGURATION_API_VERSION, 0);
        assert_ne!(POLICY_API_VERSION, 0);
        assert_ne!(PROFILE_API_VERSION, 0);
        assert_ne!(PROFILE_SCHEMA_VERSION, 0);
        assert_ne!(TARGET_DESCRIPTOR_API_VERSION, 0);
        assert_ne!(CONFIGURATION_VALIDATION_API_VERSION, 0);
    }

    #[test]
    fn canonical_compiler_configuration_remains_usable() {
        let config = CompilerConfig {
            target: CompilationTarget::Wasm32,
            opt_level: OptimizationLevel::Basic,
            debug_info: true,
            verify: true,
            emit_ir: false,
            parallel: false,
        };

        assert!(
            validate_compiler_config(
                &config,
                ConfigurationValidationMode::Production,
            )
            .is_ok()
        );
    }

    #[test]
    fn configuration_builder_is_available_through_the_facade() {
        let config = CompilerConfigBuilder::for_target(
            CompilationTarget::Wasm32,
        )
        .optimization_level(OptimizationLevel::Aggressive)
        .verify(true)
        .build()
        .expect("explicit target configuration should build");

        assert_eq!(config.target, CompilationTarget::Wasm32);
        assert_eq!(config.opt_level, OptimizationLevel::Aggressive);
    }

    #[test]
    fn unlimited_budget_is_not_a_hardware_capacity_claim() {
        let budget = CompilationBudget::unlimited();

        assert!(budget.is_unlimited());
    }

    #[test]
    fn profile_registry_is_available_through_the_facade() {
        let registry = CompilationProfileRegistry::with_builtins();

        assert!(!registry.is_empty());
    }

    #[test]
    fn target_descriptor_is_available_through_the_facade() {
        let descriptor = TargetDescriptor::builder(
            TargetId::new("test:portable-target")
                .expect("target identifier should be valid"),
            "Portable test target",
        )
        .build()
        .expect("target descriptor should be valid");

        assert_eq!(
            descriptor.id().as_str(),
            "test:portable-target"
        );
    }

    #[test]
    fn policy_document_is_available_through_the_facade() {
        let identifier = PolicyId::new("test::portable")
            .expect("policy identifier should be valid");

        let policy = PolicyDocument::new(identifier);

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn aggregate_validation_is_available_through_the_facade() {
        let config = CompilerConfig {
            target: CompilationTarget::Wasm32,
            opt_level: OptimizationLevel::Basic,
            debug_info: true,
            verify: true,
            emit_ir: false,
            parallel: false,
        };

        let report = validate_compilation_configuration(
            &config,
            ConfigurationValidationMode::Production,
        )
        .expect("valid configuration should produce a report");

        assert!(report.is_valid());
    }

    #[test]
    fn aggregate_validation_preserves_legacy_configuration() {
        let config = CompilerConfig::default();
        let original_target = config.target.clone();
        let original_level = config.opt_level.clone();

        let _ = validate_compilation_configuration(
            &config,
            ConfigurationValidationMode::Standard,
        )
        .expect("standard validation should succeed");

        assert_eq!(config.target, original_target);
        assert_eq!(config.opt_level, original_level);
    }
}
