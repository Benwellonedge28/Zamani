
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Standard-library integration with the canonical Zamani compiler.
//!
//! Path:
//!     src/stdlib/integration/compiler.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Provide a stable standard-library-facing integration boundary
//! for the existing canonical Zamani compiler.
//!
//! This module re-exports compiler-owned APIs. It does not
//! implement or replace the compiler.
//!
//! # Architectural ownership
//!
//! The following existing repository components remain authoritative:
//!
//! - `crate::frontend`:
//!   Lexing, parsing, AST construction and frontend integration.
//!
//! - `crate::frontend::semantic`:
//!   Source-level semantic analysis and validated program meaning.
//!
//! - `crate::compiler`:
//!   Compiler composition and compilation coordination.
//!
//! - `crate::compiler::configuration`:
//!   Compiler configuration, policies, profiles, targets,
//!   operational budgets and configuration validation.
//!
//! - `crate::compiler::analysis`:
//!   Compiler analysis, requirements, capabilities and feasibility.
//!
//! - `crate::compiler::ir`:
//!   Integration with canonical intermediate representations.
//!
//! - `crate::compiler::planning`:
//!   Target, capability, feasibility and resource planning.
//!
//! - `crate::compiler::lowering`:
//!   Domain-specific and target-specific lowering.
//!
//! - `crate::compiler::backends`:
//!   Compiler-facing backend integration.
//!
//! - `crate::compiler::artifacts`:
//!   Compilation artifacts, manifests and portability contracts.
//!
//! - `crate::backend`:
//!   Backend implementations and target-specific realization.
//!
//! - `crate::runtime`:
//!   Runtime execution, scheduling and physical resource use.
//!
//! This module owns only the standard-library integration boundary.
//!
//! # Dependency direction
//!
//! ```text
//!
//! Zamani standard library
//!          |
//!          v
//! stdlib::integration::compiler
//!          |
//!          v
//! compiler::api
//!          |
//!          v
//! canonical compiler composition
//!          |
//!          +--> frontend
//!          +--> semantic analysis
//!          +--> canonical IR
//!          +--> analysis
//!          +--> optimization
//!          +--> planning
//!          +--> lowering
//!          +--> backend integration
//!          +--> artifact validation
//!
//! Runtime execution remains a separate responsibility.
//!
//! ```
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This adapter introduces no universal hardware or computational
//! capacity limits.
//!
//! It does not impose fixed limits on:
//!
//! - processors, cores or threads;
//! - GPUs, FPGAs, ASICs or QPUs;
//! - accelerators or heterogeneous devices;
//! - distributed participants;
//! - memory or storage;
//! - logical or physical qubits;
//! - tensor dimensions or rank;
//! - register widths;
//! - modules, functions or operations;
//! - compiler stages, passes or providers.
//!
//! Explicit operational budgets remain the responsibility of
//! the canonical compiler configuration and planning systems.
//!
//! Actual feasibility depends on program requirements, available
//! capabilities, constraints, policies, implementation support,
//! operational budgets and physical resources.
//!
//! No finite implementation promises literal infinite computation.
//!
//! # Configuration contract
//!
//! Compilation configuration must be supplied through the existing
//! `crate::compiler_types::CompilerConfig` authority.
//!
//! This adapter does not:
//!
//! - invent a replacement configuration type;
//! - silently select the host machine as the target;
//! - silently replace an explicitly requested target;
//! - convert a requirement into a preference;
//! - treat an unknown capability as a supported capability;
//! - claim feasibility without appropriate evidence.
//!
//! In particular, callers should not assume that the existing
//! `CompilerApi::production_config()` is target-independent.
//! Its current implementation selects `X86_64Linux`.
//!
//! # Error contract
//!
//! Compiler failures retain the canonical `CompilerError`.
//!
//! This module does not flatten compiler errors into strings,
//! create another compiler error hierarchy, or convert failures
//! into successful compilation responses.
//!
//! # Compatibility
//!
//! Existing compiler APIs remain owned by `crate::compiler::api`.
//!
//! Legacy compiler functions are exposed only through explicitly
//! named compatibility exports.
//!
//! # Integration requirements
//!
//! Parent:
//!     `src/stdlib/integration/mod.rs`
//!
//! Standard-library root:
//!     `src/stdlib/mod.rs`
//!
//! Compiler authority:
//!     `src/compiler/mod.rs`
//!
//! No compiler initialization or shutdown is performed here.
//!
//! No compiler stage is executed merely by importing this module.

/// Canonical compiler API request, response, service and
/// configuration types.
///
/// These are re-exports. Their implementations and type identities
/// remain owned by `crate::compiler::api`.
pub use crate::compiler::api::{
    CompilationRequest,
    CompilationResponse,
    CompilationTarget,
    CompilerApi,
    CompilerConfig,
    CompilerError,
    CompilerOutput,
    CompilerResult,
    CompilerService,
    OptimizationLevel,
    SourceInput,
};

/// Canonical compiler API compatibility functions.
///
/// The aliases make their legacy status explicit at the
/// standard-library integration boundary.
pub use crate::compiler::api::{
    compile as compile_legacy,
    compile_source as compile_source_legacy,
    compile_with_config as compile_with_config_legacy,
};

/// Canonical compiler composition API version.
///
/// This is the compiler's version, not a separately maintained
/// standard-library version.
pub use crate::compiler::{
    COMPILER_API_VERSION as CANONICAL_COMPILER_API_VERSION,
};

/// Version of this standard-library-to-compiler integration
/// contract.
///
/// Increment this version only when this integration boundary
/// changes incompatibly.
///
/// This value is independent of:
///
/// - Cargo package version;
/// - Zamani language version;
/// - grammar version;
/// - semantic-model version;
/// - compiler API version;
/// - IR schema versions;
/// - artifact serialization versions;
/// - backend ABI versions;
/// - runtime versions.
pub const STDLIB_COMPILER_INTEGRATION_API_VERSION: u16 = 1;

/// Returns the canonical compiler composition API version.
///
/// The compiler remains the sole owner of this version.
///
/// # Examples
///
/// ```
/// let version =
///     zamani::stdlib::integration::compiler::compiler_api_version();
///
/// assert_ne!(version, 0);
/// ```
#[must_use]
pub const fn compiler_api_version() -> u16 {
    crate::compiler::api_version()
}

/// Returns the standard-library compiler integration API version.
///
/// This identifies the compatibility contract of this adapter,
/// not the version of the underlying compiler implementation.
#[must_use]
pub const fn integration_api_version() -> u16 {
    STDLIB_COMPILER_INTEGRATION_API_VERSION
}

// ============================================================================
// CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_api_version_is_nonzero() {
        assert_ne!(
            STDLIB_COMPILER_INTEGRATION_API_VERSION,
            0
        );
    }

    #[test]
    fn integration_version_matches_accessor() {
        assert_eq!(
            integration_api_version(),
            STDLIB_COMPILER_INTEGRATION_API_VERSION
        );
    }

    #[test]
    fn compiler_version_comes_from_canonical_authority() {
        assert_eq!(
            compiler_api_version(),
            crate::compiler::api_version()
        );

        assert_eq!(
            CANONICAL_COMPILER_API_VERSION,
            crate::compiler::COMPILER_API_VERSION
        );
    }

    #[test]
    fn compiler_api_is_available_without_global_initialization() {
        let api = CompilerApi::new();

        let _ = api;
    }

    #[test]
    fn compiler_service_contract_remains_object_safe() {
        fn accepts_service(_: &dyn CompilerService) {}

        let _ = accepts_service;
    }

    #[test]
    fn canonical_request_type_is_reused() {
        fn accepts_request(_: CompilationRequest) {}

        let _ = accepts_request;
    }

    #[test]
    fn canonical_response_type_is_reused() {
        fn accepts_response(_: CompilationResponse) {}

        let _ = accepts_response;
    }

    #[test]
    fn canonical_configuration_type_is_reused() {
        fn accepts_configuration(_: CompilerConfig) {}

        let _ = accepts_configuration;
    }

    #[test]
    fn canonical_error_type_is_reused() {
        fn accepts_error(_: CompilerError) {}

        let _ = accepts_error;
    }

    #[test]
    fn canonical_result_type_is_reused() {
        fn accepts_result(
            result: CompilerResult<()>,
        ) -> CompilerResult<()> {
            result
        }

        let _ = accepts_result;
    }

    #[test]
    fn canonical_source_input_type_is_reused() {
        fn accepts_input(_: SourceInput) {}

        let _ = accepts_input;
    }

    #[test]
    fn legacy_entry_points_remain_explicitly_available() {
        let _ = compile_legacy;
        let _ = compile_source_legacy;
        let _ = compile_with_config_legacy;
    }

    #[test]
    fn no_default_target_is_introduced_by_this_adapter() {
        // The integration layer intentionally exposes no
        // target-selection function and no implicit configuration
        // constructor. Callers supply the canonical configuration.
        //
        // Target compatibility and feasibility remain compiler
        // responsibilities.
        fn accepts_target(_: CompilationTarget) {}

        let _ = accepts_target;
    }
}
