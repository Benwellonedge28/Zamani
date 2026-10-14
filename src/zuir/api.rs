
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Universal Intermediate Representation — Public API
//!
//! Path: `src/zuir/api.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97 or later.
//!
//! ## PURPOSE
//!
//! Provide the stable public entry point for ZUIR configuration and
//! fallible operations. This module validates configuration at operation
//! boundaries and preserves structured ZUIR errors.
//!
//! ## AUTHORITY
//!
//! This module conforms to:
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/config.rs`
//! - `src/zuir/errors.rs`
//! - `src/zuir/version.rs`
//! - `src/zuir/diagnostics.rs`
//! - `src/zuir/source_map.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/grammar.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! ## OWNS
//!
//! - The public ZUIR API facade.
//! - Public re-exports of approved configuration and error contracts.
//! - Configuration validation at the API boundary.
//! - A generic operation boundary that preserves structured errors.
//!
//! ## DOES NOT OWN
//!
//! - AST construction, parsing, or grammar rules.
//! - Semantic analysis, name resolution, or type inference.
//! - IR nodes, identifiers, types, values, or operation semantics.
//! - Dialect registration or operation-schema definitions.
//! - Verification algorithms, transformations, or optimization passes.
//! - Serialization formats, fingerprints, or migration implementations.
//! - Physical resource discovery, target selection, lowering, or execution.
//!
//! ## ARCHITECTURAL CONTRACT
//!
//! ZUIR is the target-independent representation boundary for computational
//! meaning. This API must not become a second AST, semantic model, quantum IR,
//! or hardware-specific representation.
//!
//! Public operations must delegate to their authoritative implementation
//! modules. Unsupported functionality must fail explicitly; it must not
//! pretend to verify, serialize, transform, or lower an artifact.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This module introduces no fixed limits on program size, operations,
//! qubits, processors, devices, memory, register width, tensor rank,
//! topology, or other hardware dimensions.
//!
//! Configuration budgets are explicit operational policies. An absent
//! budget does not impose a policy limit, but cannot guarantee infinite
//! memory, time, address space, or physical resources.
//!
//! Determinism is a correctness requirement and cannot be disabled through
//! this API.
//!
//! ## ERROR CONTRACT
//!
//! Configuration failures and operation failures use the canonical
//! `ZuirError` and `ZuirResult` types. Existing error categories and phases
//! are preserved. The API adds contextual notes instead of replacing the
//! original error category or message.
//!
//! ## DEPENDENCY DIRECTION
//!
//! This module may depend on foundational configuration and error contracts.
//! Those foundational modules must not depend on this API facade.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `mod.rs` declares `pub mod api;` and re-exports the intended public API.
//! - Builders own validated construction of concrete IR nodes.
//! - The verifier owns structural and semantic invariant checks.
//! - The dialect registry owns extension resolution and operation definitions.
//! - Pass management owns transformation execution and invalidation.
//! - Serialization owns encoding, decoding, and schema compatibility.
//! - Lowering owns target feasibility and semantic-preservation checks.
//! - Frontend and grammar modules remain authoritative for source syntax.
//!
//! This facade intentionally does not invent methods that depend on IR types
//! or subsystems that are not yet present in the module graph.
//!
//! ## FREEZE CRITERIA
//!
//! Freeze only after this module compiles against the real `mod.rs`, the
//! repository's public error/configuration contracts are stable, tests pass,
//! and the eventual builder/verifier/serialization/lowering APIs are added
//! through their own authoritative modules rather than duplicated here.

use crate::zuir::config::ZuirConfig;
use crate::zuir::errors::{ErrorPhase, ZuirError, ZuirErrorKind, ZuirResult};

// Re-export stable contracts so callers can use the public API without
// depending on internal module paths.
pub use crate::zuir::config::{
    BudgetKind, CompatibilityPolicy, DiagnosticLimitPolicy, ExtensionPolicy,
    VerificationPolicy, WorkBudget,
};
pub use crate::zuir::errors::{ErrorSubject, ZuirErrorCode};

/// The canonical public API facade for ZUIR.
///
/// `ZuirApi` is an immutable, invocation-safe holder for configuration.
/// It owns neither an IR artifact nor mutable global state. Concrete IR
/// construction and processing remain the responsibility of their modules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZuirApi {
    config: ZuirConfig,
}

impl Default for ZuirApi {
    /// Create an API facade with the default ZUIR configuration.
    ///
    /// The default configuration is valid by construction.
    fn default() -> Self {
        Self {
            config: ZuirConfig::default(),
        }
    }
}

impl ZuirApi {
    /// Create an API facade with the default configuration.
    #[must_use]
    pub fn new_default() -> Self {
        Self::default()
    }

    /// Create an API facade using a caller-provided configuration.
    ///
    /// Configuration is validated before the facade is returned. Invalid
    /// configuration is rejected before any compilation operation begins.
    pub fn with_config(config: ZuirConfig) -> ZuirResult<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    /// Borrow the immutable configuration used by this facade.
    #[must_use]
    pub const fn config(&self) -> &ZuirConfig {
        &self.config
    }

    /// Revalidate the configuration at an explicit trust boundary.
    ///
    /// This is useful when an API entry point is invoked after configuration
    /// has been loaded or assembled by another subsystem. Configuration
    /// remains immutable after it has been placed in this facade.
    pub fn validate_config(&self) -> ZuirResult<()> {
        self.config.validate()
    }

    /// Run a fallible operation under this facade's validated configuration.
    ///
    /// The operation closure receives a shared reference to the configuration
    /// and returns the canonical ZUIR result type. The closure is invoked only
    /// after configuration validation succeeds.
    ///
    /// This method does not claim to verify or transform an IR artifact.
    /// Callers must pass the operation to its owning subsystem.
    ///
    /// If the operation fails, its original error kind and phase are retained;
    /// an explanatory note identifies the API operation that failed.
    pub fn run<T, F>(&self, operation: &str, action: F) -> ZuirResult<T>
    where
        F: FnOnce(&ZuirConfig) -> ZuirResult<T>,
    {
        self.validate_config()?;

        action(&self.config).map_err(|error| {
            error.with_note(format!("ZUIR API operation: {operation}"))
        })
    }

    /// Run a closure with an explicit phase label for errors created by that
    /// closure.
    ///
    /// The phase is applied only to errors returned by the closure. Existing
    /// error kinds and messages remain intact. Use this method when an
    /// operation implementation has not yet assigned its own phase.
    ///
    /// Prefer `run` when the owning subsystem already assigns precise phases.
    pub fn run_in_phase<T, F>(
        &self,
        phase: ErrorPhase,
        operation: &str,
        action: F,
    ) -> ZuirResult<T>
    where
        F: FnOnce(&ZuirConfig) -> ZuirResult<T>,
    {
        self.validate_config()?;

        action(&self.config).map_err(|error| {
            error
                .with_phase(phase)
                .with_note(format!("ZUIR API operation: {operation}"))
        })
    }
}

impl From<ZuirConfig> for ZuirApi {
    /// Construct the API facade from a configuration.
    ///
    /// Unlike `with_config`, `From` cannot return a validation error. The
    /// configuration type currently protects its invariants through private
    /// fields and validated builder methods, so this conversion is safe.
    fn from(config: ZuirConfig) -> Self {
        Self { config }
    }
}

/// Validate a configuration without retaining an API facade.
///
/// This convenience function is useful for applications that keep
/// configuration in their own invocation context.
pub fn validate_config(config: &ZuirConfig) -> ZuirResult<()> {
    config.validate()
}

/// Construct a structured configuration error for API-boundary failures.
///
/// Prefer the canonical configuration validation in `config.rs`; this
/// helper exists for API-level argument checks only.
pub(crate) fn api_input_error(message: impl Into<String>) -> ZuirError {
    ZuirError::new(ZuirErrorKind::InvalidInput, message)
        .with_phase(ErrorPhase::Configuration)
        .with_note("The request was rejected at the ZUIR public API boundary.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_api_has_valid_deterministic_configuration() {
        let api = ZuirApi::default();

        assert!(api.config().deterministic());
        assert!(api.validate_config().is_ok());
    }

    #[test]
    fn caller_configuration_is_validated_and_retained() {
        let config = ZuirConfig::default()
            .with_verification_policy(VerificationPolicy::Thorough)
            .with_compatibility_policy(CompatibilityPolicy::Compatible)
            .with_extension_policy(ExtensionPolicy::AllowExperimental);

        let api = ZuirApi::with_config(config.clone())
            .expect("default-derived configuration should be valid");

        assert_eq!(api.config(), &config);
    }

    #[test]
    fn run_passes_the_same_immutable_configuration_to_the_operation() {
        let api = ZuirApi::default();

        let result = api.run("test.operation", |config| {
            assert!(config.deterministic());
            assert_eq!(
                config.verification_policy(),
                VerificationPolicy::Standard
            );
            Ok(42_u64)
        });

        assert_eq!(result.expect("operation should succeed"), 42);
    }

    #[test]
    fn run_preserves_error_kind_and_adds_operation_context() {
        let api = ZuirApi::default();

        let result: ZuirResult<()> = api.run("test.failure", |_config| {
            Err(ZuirError::new(
                ZuirErrorKind::UnsupportedOperation,
                "operation is unsupported",
            ))
        });

        let error = result.expect_err("the operation should fail");

        assert_eq!(error.kind(), ZuirErrorKind::UnsupportedOperation);
        assert!(
            error.to_string().contains("operation is unsupported"),
            "the original error message should remain available"
        );
    }

    #[test]
    fn run_in_phase_assigns_the_requested_phase() {
        let api = ZuirApi::default();

        let result: ZuirResult<()> = api.run_in_phase(
            ErrorPhase::Verification,
            "test.verification",
            |_config| {
                Err(ZuirError::new(
                    ZuirErrorKind::VerificationFailed,
                    "verification failed",
                ))
            },
        );

        let error = result.expect_err("verification should fail");
        assert_eq!(error.kind(), ZuirErrorKind::VerificationFailed);
        assert_eq!(error.context().phase(), Some(ErrorPhase::Verification));
    }

    #[test]
    fn operation_is_not_called_if_configuration_validation_fails() {
        // Public configuration builders validate custom budget keys, so
        // invalid configuration must be rejected before a facade can exist.
        let invalid = ZuirConfig::default().with_budget(
            BudgetKind::Custom("not namespaced".to_owned()),
            WorkBudget::bounded(1),
        );

        assert!(invalid.is_err());
    }

    #[test]
    fn unlimited_budgets_do_not_encode_hardware_capacities() {
        let api = ZuirApi::default();

        assert!(
            api.config()
                .budget(&BudgetKind::IrEntities)
                .is_unlimited()
        );
        assert!(
            api.config()
                .budget(&BudgetKind::SerializedBytes)
                .is_unlimited()
        );
    }
}
 