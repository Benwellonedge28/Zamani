
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Environment Integration
//!
//! Path: src/toolchain/environments/mod.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//! Safety: Safe Rust only.
//!
//! This module is the composition root for environment integration.
//!
//! It coordinates existing environment configuration, platform discovery
//! and reproducibility infrastructure without introducing competing
//! configuration, hardware, process, compiler or runtime authorities.
//!
//! # Architectural ownership
//!
//! | Responsibility | Authority |
//! |---|---|
//! | Environment configuration | toolchain::configuration |
//! | Environment discovery | environments::discovery |
//! | Platform observations | zur::platforms |
//! | External tool discovery | backend::toolchain |
//! | External process execution | backend::process |
//! | Compilation environment identity | compiler::reproducibility |
//! | Backend fingerprints | backend::reproducibility |
//! | Host descriptions | zur::platforms::host |
//! | Target compatibility | backend and ZUR |
//! | Physical hardware | ZHAL and providers |
//! | Execution | ZUR |
//!
//! # Architectural invariants
//!
//! - No duplicate configuration authority.
//! - No duplicate environment identity authority.
//! - No duplicate fingerprint algorithm.
//! - No duplicate platform provider interface.
//! - No duplicate process execution implementation.
//! - No implicit hardware discovery.
//! - No implicit target selection.
//! - No fabricated capability or availability evidence.
//! - No arbitrary hardware or computational ceilings.
//! - No unsafe Rust.
//!
//! # POCO-REAF
//!
//! One logical Zamani project may be realized across multiple compatible
//! execution environments.
//!
//! Environment-specific realization must preserve canonical program
//! semantics.
//!
//! Incompatibility must be reported explicitly rather than silently
//! changing program meaning.
//!
//! # Scalability
//!
//! This module introduces no fixed maximum for environments, providers,
//! external tools, platforms, devices, computational domains, processors,
//! accelerators, distributed participants, memory or storage.
//!
//! Actual capacity depends on available resources, implementation
//! capabilities and explicitly configured operational budgets.
//!
//! Literal infinite physical computation is not promised.
//!
//! # Grammar boundary
//!
//! Source grammar and semantic analysis own computational intent,
//! requirements, capabilities, preferences and constraints.
//!
//! Environment integration consumes validated representations.
//!
//! This module does not parse grammar, inspect AST nodes or reinterpret
//! source-language semantics.
//!
//! # Module composition
//!
//! discovery.rs:
//!     Explicit platform observations and external-tool discovery.
//!
//! configuration.rs:
//!     Explicit immutable environment configuration.
//!
//! fingerprints.rs:
//!     Integration with compiler, backend and platform fingerprinting.
//!
//! Environment compatibility validation is deliberately kept separate
//! from these three existing implementations. Its public module is added
//! when the dedicated validation implementation is available.

// ============================================================================
// ENVIRONMENT IMPLEMENTATIONS
// ============================================================================

/// Explicit platform and external-tool discovery.
///
/// Reuses ZUR platform providers and the canonical backend toolchain manager.
///
/// Does not independently implement hardware discovery, process execution,
/// executable resolution or tool-version parsing.
pub mod discovery;

/// Explicit environment configuration.
///
/// Reuses the aggregate toolchain configuration and canonical provider
/// identities.
///
/// Does not discover hardware, execute processes or establish feasibility.
pub mod configuration;

/// Environment reproducibility and fingerprint integration.
///
/// Reuses compiler reproducibility, backend reproducibility and canonical
/// stable fingerprint infrastructure.
///
/// Fingerprints are identities, not signatures or compatibility proofs.
pub mod fingerprints;

// ============================================================================
// STABLE PUBLIC CONFIGURATION API
// ============================================================================

/// Environment configuration identity and descriptive attributes.
pub use configuration::{
    EnvironmentAttributeKey,
    EnvironmentConfiguration,
    EnvironmentConfigurationBuilder,
};

// ============================================================================
// STABLE PUBLIC DISCOVERY API
// ============================================================================

/// External toolchain discovery results and environment discovery service.
pub use discovery::{
    DiscoveredToolchains,
    EnvironmentDiscovery,
};

// ============================================================================
// STABLE PUBLIC FINGERPRINT API
// ============================================================================

/// Environment fingerprint contracts and platform observation identity.
pub use fingerprints::{
    EnvironmentFingerprint,
    EnvironmentFingerprintBuilder,
    PlatformObservationFingerprint,
    ENVIRONMENT_FINGERPRINT_API_VERSION,
};

/// Stable identity of the environment integration component.
///
/// This is descriptive metadata, not a platform or provider identity.
#[must_use]
pub const fn component_name() -> &'static str {
    "zamani.toolchain.environments"
}

/// Environment integration composition generation.
///
/// Independent of language, compiler, backend, runtime and artifact versions.
#[must_use]
pub const fn contract_generation() -> u32 {
    1
}

// ============================================================================
// CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_identity_is_stable() {
        assert_eq!(
            component_name(),
            "zamani.toolchain.environments"
        );
    }

    #[test]
    fn contract_generation_is_nonzero() {
        assert_ne!(contract_generation(), 0);
    }

    #[test]
    fn environment_modules_are_composed() {
        let configuration = EnvironmentConfiguration::new(
            crate::toolchain::ids::EnvironmentId::new(
                "org.zamani.test.environment"
            )
            .expect("valid environment identity"),
        );

        assert!(configuration.validate().is_ok());

        let discovery = EnvironmentDiscovery::new();

        let discovered = DiscoveredToolchains::new();

        assert!(discovered.is_empty());

        let _ = discovery;
    }

    #[test]
    fn fingerprint_api_has_a_version() {
        assert_ne!(ENVIRONMENT_FINGERPRINT_API_VERSION, 0);
    }
}
