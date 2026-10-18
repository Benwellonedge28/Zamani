
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Classical Intermediate Representation — Public API
//!
//! Path: `src/classical/zcir/api.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Expose the canonical ZUIR public API through the ZCIR namespace without
//! creating a second API implementation, configuration authority, error
//! taxonomy, or compilation state.
//!
//! ZCIR is the classical-computation integration boundary over the
//! repository's canonical universal IR architecture. This module provides
//! convenient public access to the existing ZUIR API and its stable contracts.
//!
//! ## Authority
//!
//! The implementation must remain consistent with:
//!
//! - `src/zuir/api.rs`
//! - `src/zuir/config.rs`
//! - `src/zuir/errors.rs`
//! - `src/zuir/ids.rs`
//! - `src/zuir/source_map.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/classical/api.rs`
//! - `src/classical/config.rs`
//! - `src/classical/error.rs`
//! - `src/classical/zcir/ids.rs`
//! - `src/classical/zcir/source_map.rs`
//! - `grammar/Zamani.g4`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! The grammar owns source syntax. The semantic subsystem establishes
//! program meaning. ZUIR owns the canonical shared IR contracts. Classical
//! and domain-specific adapters must preserve these ownership boundaries.
//!
//! ## Owns
//!
//! This module owns:
//!
//! - The ZCIR-facing public API namespace.
//! - Public aliases and re-exports of canonical ZUIR API contracts.
//! - Tests that ensure the facade does not create competing API types.
//!
//! ## Does not own
//!
//! This module does not implement:
//!
//! - A second configuration system or independent policy defaults.
//! - A competing error taxonomy or diagnostic registry.
//! - Classical IR nodes, types, values, IDs, or source maps.
//! - Parsing, AST construction, name resolution, or semantic analysis.
//! - IR construction, verification, analysis, optimization, or lowering.
//! - Serialization, migration, or canonical fingerprints.
//! - Target discovery, hardware allocation, scheduling, or execution.
//! - A compiler session, global registry, cache, or mutable global state.
//!
//! These responsibilities remain with their canonical owning subsystems.
//!
//! ## API contract
//!
//! `ZcirApi` is a public alias for `ZuirApi`. It is not a new wrapper type.
//! Consequently, both names use the same constructor, configuration,
//! validation behavior, error semantics, and delegated-operation contract.
//!
//! This deliberate identity prevents ZCIR from drifting away from ZUIR's
//! public API or requiring callers to migrate between duplicate facades.
//!
//! The exported configuration and error types are also the canonical ZUIR
//! definitions. They are not copied or redefined here.
//!
//! ## Dependency direction
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! frontend and semantic model
//!      |
//!      v
//! canonical ZUIR API and IR
//!      |
//!      +----> ZCIR public API facade
//!      |              |
//!      |              v
//!      |       classical adapters
//!      |
//!      +----> other domain adapters
//!
//! verified IR -> legal transformations -> lowering -> target execution
//! ```
//!
//! The ZUIR implementation must not depend on this ZCIR facade. ZCIR-specific
//! implementations should import the owning subsystem directly when working
//! inside that subsystem; this facade is intended for public API consumers.
//!
//! ## Configuration and resource policy
//!
//! Configuration and operational budgets are defined by the canonical ZUIR
//! configuration contract. An absent budget does not promise infinite
//! resources. It means only that the configuration does not impose an
//! explicit budget for that dimension.
//!
//! This module introduces no universal maximum for program size, operation
//! count, processor count, memory, register width, collection length,
//! tensor rank, or device count.
//!
//! Actual resource exhaustion, configured-budget exhaustion, unsupported
//! target capabilities, and representation limits must be reported by the
//! responsible subsystem. They must not silently change program meaning.
//!
//! ## Error contract
//!
//! `ZcirApi` preserves the canonical ZUIR API's structured error behavior.
//! This module does not parse human-readable error messages to make policy
//! decisions and does not replace canonical error kinds with ZCIR-specific
//! equivalents.
//!
//! ## Integration contract
//!
//! - `src/classical/zcir/mod.rs` must declare `pub mod api;`.
//! - `src/zuir/mod.rs` must declare the existing ZUIR API and configuration
//!   modules.
//! - `src/lib.rs` must expose `pub mod zuir;` and `pub mod classical;`
//!   exactly once when their module trees are ready to compile.
//! - ZCIR builders and verifiers must use canonical ZUIR contracts.
//! - Classical configuration and compatibility operations must continue to
//!   use their existing owners under `src/classical/`.
//! - Source locations and provenance must use the canonical source-map
//!   contracts; this facade does not create source-map state.
//! - Domain-specific APIs must not introduce dependency cycles back into ZUIR.
//!
//! If a future ZCIR operation requires behavior not provided by ZUIR, add
//! that operation only after defining its ownership, semantics, errors,
//! configuration, and integration contracts. Do not silently extend this
//! facade into a second compiler or IR implementation.
//!
//! ## Compatibility
//!
//! Because `ZcirApi` is an alias rather than a distinct struct, the ZUIR API's
//! source-compatibility changes also affect the ZCIR alias. Changes to the
//! canonical API therefore require reviewing downstream ZCIR callers.
//!
//! The alias does not establish a serialized-artifact version or guarantee
//! binary compatibility across independently compiled library versions.
//!
//! ## Freeze criteria
//!
//! - Every re-export resolves to the actual canonical ZUIR public API.
//! - The ZCIR root declares this module without duplicate declarations.
//! - The crate root exposes the required modules exactly once.
//! - Tests demonstrate API identity and delegated behavior.
//! - No duplicate API state, configuration authority, or error taxonomy is
//!   introduced.
//! - Formatting, compilation, tests, and lint gates pass on Rust 1.97 or later.

/// Canonical ZUIR configuration, policy, and operational-budget contracts.
pub use crate::zuir::api::{
    BudgetKind,
    CompatibilityPolicy,
    DiagnosticLimitPolicy,
    ExtensionPolicy,
    VerificationPolicy,
    WorkBudget,
    ZuirConfig,
};

/// Canonical ZUIR structured errors and result type.
pub use crate::zuir::api::{
    ErrorPhase,
    ErrorSubject,
    ZuirError,
    ZuirErrorCode,
    ZuirErrorKind,
    ZuirResult,
};

/// Canonical ZUIR public API, exposed under the ZCIR-facing name.
///
/// This is a re-export of the same type, not an independent wrapper or
/// implementation.
pub use crate::zuir::api::ZuirApi as ZcirApi;

/// Validate a ZUIR configuration using the canonical API contract.
///
/// This function is re-exported to keep configuration validation consistent
/// for callers using either the ZUIR or ZCIR namespace.
pub use crate::zuir::api::validate_config;

/// Validate a public API operation label using the canonical API contract.
///
/// The label is diagnostic context only. It does not select a backend,
/// register a dialect, authorize an extension, or execute an operation.
pub use crate::zuir::api::validate_operation_label;

#[cfg(test)]
mod tests {
    use super::*;

    /// The ZCIR-facing name must remain the same API type as ZUIR's.
    #[test]
    fn zcir_api_is_the_canonical_zuir_api_type() {
        fn accepts_zuir_api(_: &crate::zuir::api::ZuirApi) {}

        let api = ZcirApi::new_default();

        accepts_zuir_api(&api);

        assert!(api.validate_config().is_ok());
    }

    /// The facade must preserve the canonical default configuration.
    #[test]
    fn default_api_uses_canonical_configuration() {
        let api = ZcirApi::default();

        assert!(api.config().deterministic());
        assert!(validate_config(api.config()).is_ok());
    }

    /// Delegated work must receive the canonical immutable configuration.
    #[test]
    fn api_delegates_work_without_creating_independent_state() {
        let api = ZcirApi::new_default();

        let result = api.run("zcir.api.test", |config| {
            assert!(config.deterministic());
            Ok(42_u64)
        });

        assert_eq!(result.expect("delegated operation should succeed"), 42);
    }

    /// Invalid operation labels must be rejected before delegated work runs.
    #[test]
    fn api_rejects_empty_operation_labels() {
        let api = ZcirApi::new_default();
        let mut called = false;

        let result = api.run("  ", |_config| {
            called = true;
            Ok(())
        });

        assert!(result.is_err());
        assert!(!called);
    }

    /// Public validation must use the canonical ZUIR implementation.
    #[test]
    fn operation_label_validation_is_shared() {
        assert!(validate_operation_label("zcir.verify").is_ok());
        assert!(validate_operation_label("  ").is_err());
    }

    /// Error and result types must remain canonical rather than duplicated.
    #[test]
    fn error_contract_is_the_canonical_zuir_contract() {
        fn accepts_canonical_error(_: ZuirError) {}

        accepts_canonical_error(ZuirError::new(
            ZuirErrorKind::InvalidInput,
            "test input",
        ));
    }
}
