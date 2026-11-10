
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler Extensions
//!
//! Canonical path: `src/compiler/extensions/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//!
//! ## Purpose
//!
//! This module is the public composition root for the Zamani compiler
//! extension subsystem.
//!
//! It exposes the existing provider, manifest, compatibility,
//! discovery, validation, isolation, and provider-registry
//! implementations through one stable compiler module boundary.
//!
//! It does not introduce another provider implementation,
//! registration mechanism, discovery engine, compatibility evaluator,
//! or lifecycle manager.
//!
//! ## POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! The extension architecture is open-world.
//!
//! Independently developed providers must be able to contribute
//! implementations for existing and future computing environments
//! without requiring modifications to this composition root.
//!
//! Examples of possible contributions include:
//!
//! - compiler backends;
//! - compiler passes;
//! - target discovery providers;
//! - domain integrations;
//! - grammar extension adapters;
//! - analysis providers;
//! - verification providers;
//! - future compiler extension points.
//!
//! These are contribution examples, not a closed list of supported
//! computing technologies.
//!
//! ## Canonical ownership
//!
//! | Responsibility | Authority |
//! |---|---|
//! | Provider identity and lifecycle | `provider` |
//! | Extension manifest | `manifest` |
//! | Provider compatibility | `compatibility` |
//! | Extension discovery | `discovery` |
//! | Extension admission validation | `validation` |
//! | Declarative isolation | `isolation` |
//! | Provider registration | `registry` |
//! | Backend registration | `crate::backend::registry` |
//! | Pass registration | `crate::zuir::pass::registry` |
//! | Target discovery | `crate::compiler::planning::target_discovery` |
//! | Security admission | `crate::compiler::security` |
//! | Source-level semantics | `crate::frontend::semantic` |
//! | Source grammar | `grammar/` |
//! | Runtime enforcement | `crate::runtime` |
//!
//! This module composes these authorities. It does not replace them.
//!
//! ## Responsibility
//!
//! This module owns:
//!
//! - Extension subsystem module composition.
//! - Stable public extension module paths.
//! - Provider-neutral public API re-exports.
//! - Extension subsystem identity and composition version.
//! - Composition-level architectural invariants.
//!
//! This module does not own:
//!
//! - Provider registration storage.
//! - Provider identity or version definitions.
//! - Manifest parsing or serialization.
//! - Filesystem traversal.
//! - Extension discovery algorithms.
//! - Compatibility algorithms.
//! - Provider admission decisions.
//! - Publisher authentication.
//! - Cryptographic verification.
//! - Provider authorization.
//! - Dynamic library loading.
//! - Provider initialization or shutdown.
//! - Sandbox implementation.
//! - Backend registration.
//! - Pass registration.
//! - Dialect registration.
//! - Grammar parsing or modification.
//! - Semantic analysis.
//! - Capability definitions.
//! - Intermediate representations.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Compilation scheduling.
//! - Code generation.
//! - Runtime execution.
//!
//! ## Extension admission boundary
//!
//! The intended provider admission sequence is:
//!
//! 1. Discover a declarative extension candidate.
//! 2. Parse and structurally validate its manifest.
//! 3. Construct its canonical provider descriptor.
//! 4. Evaluate provider API and dependency compatibility.
//! 5. Perform extension admission validation.
//! 6. Verify publisher, package and artifact trust through the
//!    canonical security and supply-chain authorities.
//! 7. Evaluate the applicable isolation requirements.
//! 8. Register the admitted provider through `registry`.
//! 9. Resolve provider dependencies against an immutable registry
//!    snapshot.
//! 10. Delegate provider initialization and shutdown to the
//!     appropriate lifecycle owner.
//! 11. Delegate specialized contributions to their canonical
//!     subsystem registries.
//!
//! This module does not execute this sequence.
//!
//! In particular, successful discovery, parsing, compatibility,
//! validation, or registration is not proof that executable provider
//! code is trusted, isolated, initialized, or safe to execute.
//!
//! ## Specialized registry invariant
//!
//! The provider registry is not a replacement for specialized
//! implementation registries.
//!
//! A provider contributing a backend must register its backend
//! implementation through:
//!
//! `crate::backend::registry`
//!
//! A provider contributing a compiler pass must integrate with:
//!
//! `crate::zuir::pass::registry`
//!
//! A provider contributing target discovery must integrate with:
//!
//! `crate::compiler::planning::target_discovery`
//!
//! A provider contributing source-language extensions must integrate
//! with the canonical frontend and semantic extension interfaces.
//!
//! Provider registration alone must never be interpreted as
//! registration of every declared contribution.
//!
//! ## Grammar boundary
//!
//! The authoritative grammar composition boundary is:
//!
//! `grammar/Zamani.g4`
//!
//! The canonical lexer boundary is:
//!
//! `grammar/antlr/ZamaniLexer.g4`
//!
//! The lexical composition authority is:
//!
//! `grammar/lexer/tokens.g4`
//!
//! The semantic authority is specified by:
//!
//! `grammar/specification/semantic-model.md`
//!
//! These authorities establish source syntax and source meaning.
//!
//! This module is host-side compiler extension infrastructure.
//!
//! It does not add provider-registration syntax, define new tokens,
//! modify parser productions, or create another grammar registry.
//!
//! Source-language extension syntax and semantic interpretation
//! remain the responsibility of the frontend and its canonical
//! extension interfaces.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary universal maximum on:
//!
//! - provider count;
//! - provider versions;
//! - provider dependencies;
//! - extension manifests;
//! - contributions;
//! - extension points;
//! - computing domains;
//! - targets;
//! - backend implementations;
//! - compiler passes;
//! - hardware providers;
//! - distributed participants;
//! - future computing technologies.
//!
//! It introduces no fixed hardware assumptions or universal
//! MAX_PROVIDERS, MAX_BACKENDS, MAX_TARGETS, MAX_CPUS, MAX_GPUS,
//! MAX_QUBITS, MAX_NODES, or equivalent semantic ceilings.
//!
//! Actual operational limits are determined by available memory,
//! address space, processing capacity, operating-system constraints,
//! provider implementation limits, and caller-configured budgets.
//!
//! Operational budgets are not semantic capacity limits.
//!
//! No finite implementation promises literal infinite physical
//! computation or unlimited physical resources.
//!
//! ## Determinism
//!
//! This composition root introduces no mutable global state.
//!
//! It does not establish provider registration order as semantic
//! priority.
//!
//! Deterministic ordering and dependency resolution remain owned
//! by `registry`.
//!
//! Manifest ordering and serialization remain owned by `manifest`.
//!
//! Compatibility findings remain owned by `compatibility`.
//!
//! ## Concurrency
//!
//! Synchronization and shared-state behavior are inherited from
//! the individual subsystem implementations.
//!
//! This module creates no additional locks, registries, caches,
//! thread pools, or provider lifecycle state.
//!
//! ## Security
//!
//! Unsafe Rust is forbidden throughout this module.
//!
//! This module does not treat provider metadata as trusted code.
//!
//! It does not claim that declarative isolation requirements
//! establish enforced isolation.
//!
//! It does not claim that successful validation establishes
//! publisher identity or artifact integrity.
//!
//! Untrusted executable providers must not be activated unless
//! the applicable security, admission, and isolation authorities
//! establish the required conditions.
//!
//! ## Compatibility
//!
//! The public child-module paths are stable composition paths.
//!
//! Individual subsystem APIs remain owned by their respective
//! modules.
//!
//! The composition API version is independent of:
//!
//! - Zamani language version;
//! - grammar version;
//! - semantic-model version;
//! - provider implementation version;
//! - manifest format version;
//! - backend API version;
//! - ZUIR schema version;
//! - runtime API version.
//!
//! Adding a child module or a backward-compatible re-export does
//! not automatically require an incompatible API version change.
//!
//! An incompatible change to this public composition contract
//! requires an explicit versioning decision.

// ============================================================================
// 1. EXTENSION SUBSYSTEM MODULES
// ============================================================================

/// Canonical provider identity, descriptors, dependencies,
/// API compatibility, and provider lifecycle contracts.
///
/// This module owns provider contract definitions, not registration.
pub mod provider;

/// Versioned extension manifest declarations, structural
/// validation, serialization, and descriptor conversion.
///
/// Manifests are untrusted input until independently admitted.
pub mod manifest;

/// Provider API, dependency, contribution, and extension-point
/// compatibility evaluation.
///
/// Compatibility does not establish trust or authorization.
pub mod compatibility;

/// Declarative extension candidate discovery.
///
/// Discovery does not register, load, initialize, or execute providers.
pub mod discovery;

/// Provider manifest and compatibility validation coordination.
///
/// Validation does not establish supply-chain trust or runtime safety.
pub mod validation;

/// Declarative provider and provider-instance isolation requirements.
///
/// This module describes isolation requirements; enforcement belongs
/// to the applicable security and runtime authorities.
pub mod isolation;

/// Canonical registry of provider instances.
///
/// This is the provider registry only. Backend, pass, target,
/// and dialect implementation registries retain their own authority.
pub mod registry;

// ============================================================================
// 2. MODULE IDENTITY AND COMPOSITION VERSION
// ============================================================================

/// Stable architectural identity of the compiler extension subsystem.
pub const EXTENSIONS_MODULE_NAME: &str =
    "zamani.compiler.extensions";

/// Public composition API version.
///
/// This version identifies the module composition boundary.
/// It is not the provider API version, manifest format version,
/// backend API version, or runtime API version.
pub const EXTENSIONS_API_VERSION: u16 = 1;

/// Returns the stable extension subsystem identity.
#[must_use]
pub const fn module_name() -> &'static str {
    EXTENSIONS_MODULE_NAME
}

/// Returns the extension composition API version.
#[must_use]
pub const fn api_version() -> u16 {
    EXTENSIONS_API_VERSION
}

// ============================================================================
// 3. PROVIDER CONTRACT RE-EXPORTS
// ============================================================================

// These are aliases to the canonical provider definitions.
// No competing provider identity or version types are introduced.

pub use self::provider::{
    ContributionKind,
    ExtensionPointId,
    ExtensionProvider,
    ProviderApiCompatibility,
    ProviderDependency,
    ProviderDescriptor,
    ProviderError,
    ProviderId,
    ProviderVersion,
    ProviderVersionRequirement,
};

// ============================================================================
// 4. PROVIDER REGISTRY RE-EXPORTS
// ============================================================================

// These are the existing provider registry types.
// This module does not implement another registry.

pub use self::registry::{
    ProviderRegistry,
    RegistryError,
    RegisteredProvider,
    RegistrySnapshot,
};

// ============================================================================
// 5. COMPOSITION CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_module_has_stable_identity() {
        assert_eq!(
            module_name(),
            "zamani.compiler.extensions",
        );
    }

    #[test]
    fn extension_composition_api_version_is_available() {
        assert_eq!(
            api_version(),
            EXTENSIONS_API_VERSION,
        );

        assert_ne!(api_version(), 0);
    }

    #[test]
    fn all_extension_subsystems_are_exposed() {
        let _ = std::any::type_name::<ProviderId>();
        let _ = std::any::type_name::<ProviderDescriptor>();
        let _ = std::any::type_name::<ExtensionProvider>();
        let _ = std::any::type_name::<ProviderRegistry>();
        let _ = std::any::type_name::<RegistrySnapshot>();

        // Referencing each module ensures that the composition
        // root exposes the complete extension subsystem.
        let _ = provider::PROVIDER_MODULE_NAME;
        let _ = manifest::MANIFEST_MODULE_NAME;
        let _ = compatibility::EXTENSION_COMPATIBILITY_MODULE_NAME;
        let _ = discovery::EXTENSION_DISCOVERY_MODULE_NAME;
        let _ = validation::EXTENSION_VALIDATION_MODULE_NAME;
    }

    #[test]
    fn provider_registry_is_distinct_from_backend_registry() {
        // The provider registry manages provider instances.
        // The backend registry remains owned by crate::backend.
        let _ = std::any::type_name::<ProviderRegistry>();
        let _ = std::any::type_name::<crate::backend::registry::BackendRegistry>();
    }

    #[test]
    fn compiler_pass_registry_remains_a_separate_authority() {
        // Pass registration remains owned by the canonical ZUIR
        // pass registry and its compiler-facing integration.
        let _ = std::any::type_name::<
            crate::compiler::passes::registry::CompilerPassRegistry
        >();
    }
}
