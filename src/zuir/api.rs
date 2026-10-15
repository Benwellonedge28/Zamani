#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Universal Intermediate Representation — Public API
//!
//! Path: `src/zuir/api.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97 or later.
//!
//! STATUS
//!
//! Proposed replacement. Freeze only after parent-module integration,
//! repository compilation, and the required tests have passed.
//!
//! PURPOSE
//!
//! Provide the public entry point for ZUIR configuration and invocation
//! boundaries. Validate configuration and request labels before running
//! caller-supplied work, preserve structured errors, and expose approved
//! configuration and error contracts.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/config.rs`
//! - `src/zuir/errors.rs`
//! - `src/zuir/version.rs`
//! - `src/zuir/diagnostics.rs`
//! - `src/zuir/source_map.rs`
//! - `src/zuir/ir/`
//! - `src/zuir/builder/`
//! - `src/zuir/verify/`
//! - `src/zuir/serialization/`
//! - `src/zuir/lowering/`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/lexer/tokens.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! OWNS
//!
//! - The public ZUIR API facade.
//! - Deliberate re-exports of configuration and error contracts.
//! - Validation at this facade's invocation boundary.
//! - Contextual annotation of errors returned by delegated operations.
//!
//! DOES NOT OWN
//!
//! - Source parsing, AST construction, or semantic analysis.
//! - IR entities, identifiers, types, effects, or operation semantics.
//! - Dialect registration or extension execution.
//! - Verification, transformation, optimization, or lowering algorithms.
//! - Serialization, migration, fingerprints, or incremental cache algorithms.
//! - Target discovery, physical allocation, scheduling, or runtime execution.
//!
//! ARCHITECTURAL CONTRACT
//!
//! ZUIR represents established computational meaning independently of a
//! particular target. This facade is not a second AST, semantic model,
//! quantum IR, hardware IR, verifier, serializer, or backend.
//!
//! Every substantive operation must be delegated to its authoritative
//! subsystem. A missing or unsupported operation must fail explicitly;
//! this facade must never report success without doing the requested work.
//!
//! SCALABILITY AND POCO-REAF
//!
//! This module introduces no universal limit on program size, operation count,
//! qubits, processors, devices, memory, register width, tensor rank, topology,
//! or domain count.
//!
//! Explicit work budgets are operational policies enforced by the subsystem
//! performing the work. An absent budget does not promise infinite resources.
//! Resource exhaustion must not be misreported as semantic invalidity.
//!
//! Determinism is a correctness requirement and cannot be disabled here.
//!
//! ERROR CONTRACT
//!
//! Expected failures use `ZuirError` and `ZuirResult`. Contextual notes may
//! be added without replacing the original error kind or message. An existing
//! error phase must be preserved; a fallback phase is added only when the
//! delegated operation did not provide one.
//!
//! DEPENDENCY DIRECTION
//!
//! This facade may depend on foundational configuration and error contracts.
//! Those modules must not depend on this facade. Implementation modules must
//! use their owning module paths rather than depending on facade re-exports.
//!
//! FREEZE CRITERIA
//!
//! - The parent `src/zuir/mod.rs` declares this module correctly.
//! - Every re-export matches an existing public definition.
//! - Configuration and request-boundary validation are tested.
//! - Error kinds, messages, and existing phases are preserved.
//! - Public API integration tests exercise actual owning subsystems.
//! - Formatting, compilation, tests, and Clippy pass on the supported toolchain.

use crate::zuir::config::ZuirConfig;

// ---------------------------------------------------------------------------
// Public configuration and error contracts
// ---------------------------------------------------------------------------

pub use crate::zuir::config::{
    BudgetKind,
    CompatibilityPolicy,
    DiagnosticLimitPolicy,
    ExtensionPolicy,
    VerificationPolicy,
    WorkBudget,
    ZuirConfig,
};

pub use crate::zuir::errors::{
    ErrorPhase,
    ErrorSubject,
    ZuirError,
    ZuirErrorCode,
    ZuirErrorKind,
    ZuirResult,
};

// ---------------------------------------------------------------------------
// API facade
// ---------------------------------------------------------------------------

/// The public invocation facade for ZUIR.
///
/// `ZuirApi` owns an immutable configuration snapshot. It does not own an IR
/// artifact, mutable global registry, cache, compiler session, or target.
///
/// Cloning this value clones the configuration; it does not clone or share
/// any mutable compilation state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZuirApi {
    config: ZuirConfig,
}

impl Default for ZuirApi {
    /// Create an API facade using the default configuration.
    ///
    /// The default configuration is valid by construction.
    fn default() -> Self {
        Self {
            config: ZuirConfig::default(),
        }
    }
}

impl ZuirApi {
    /// Create a facade using the default configuration.
    #[must_use]
    pub fn new_default() -> Self {
        Self::default()
    }

    /// Create a facade from a caller-provided configuration.
    ///
    /// Configuration is validated before the facade is returned. No
    /// compilation or IR-processing work is started by this constructor.
    pub fn with_config(config: ZuirConfig) -> ZuirResult<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    /// Borrow the immutable configuration associated with this facade.
    #[must_use]
    pub const fn config(&self) -> &ZuirConfig {
        &self.config
    }

    /// Validate the configuration at an explicit trust boundary.
    ///
    /// This method is useful when a caller wants to validate its configuration
    /// immediately before handing it to another subsystem.
    pub fn validate_config(&self) -> ZuirResult<()> {
        self.config.validate()
    }

    /// Execute caller-supplied work under the validated configuration.
    ///
    /// The closure receives a shared reference to the configuration and must
    /// delegate substantive work to the appropriate owning subsystem.
    ///
    /// `operation` is a human-readable context label, not an operation
    /// definition, dialect identifier, permission, or executable command.
    ///
    /// The label must not be empty or consist only of whitespace. It is not
    /// silently trimmed or rewritten.
    ///
    /// If the closure returns an error, its kind, message, and existing
    /// context are retained; an API-operation note is added.
    pub fn run<T, F>(
        &self,
        operation: &str,
        action: F,
    ) -> ZuirResult<T>
    where
        F: FnOnce(&ZuirConfig) -> ZuirResult<T>,
    {
        self.validate_request(operation)?;

        action(&self.config).map_err(|error| {
            error.with_note(format!("ZUIR API operation: {operation}"))
        })
    }

    /// Execute work with a fallback error phase.
    ///
    /// The requested phase is applied only when the returned error has no
    /// phase already. A more precise phase assigned by the owning subsystem
    /// is preserved.
    ///
    /// Prefer [`Self::run`] when the delegated subsystem already reports
    /// precise error phases.
    pub fn run_in_phase<T, F>(
        &self,
        phase: ErrorPhase,
        operation: &str,
        action: F,
    ) -> ZuirResult<T>
    where
        F: FnOnce(&ZuirConfig) -> ZuirResult<T>,
    {
        self.validate_request(operation)?;

        action(&self.config).map_err(|error| {
            let error = if error.context().phase().is_none() {
                error.with_phase(phase)
            } else {
                error
            };

            error.with_note(format!("ZUIR API operation: {operation}"))
        })
    }

    /// Validate the configuration and request label before invoking work.
    ///
    /// This method is public so adapters can apply the same boundary checks
    /// before performing setup that precedes a call to [`Self::run`].
    pub fn validate_request(&self, operation: &str) -> ZuirResult<()> {
        validate_operation_label(operation)?;

        self.config.validate().map_err(|error| {
            error.with_note(format!(
                "Configuration validation failed before ZUIR API operation: {operation}"
            ))
        })
    }
}

impl From<ZuirConfig> for ZuirApi {
    /// Construct the facade from a configuration.
    ///
    /// `ZuirConfig` protects its invariants through its private fields and
    /// validated public construction methods. This infallible conversion is
    /// retained for compatibility with existing callers.
    ///
    /// APIs that need an explicit fallible boundary should use
    /// [`Self::with_config`] instead.
    fn from(config: ZuirConfig) -> Self {
        Self { config }
    }
}

// ---------------------------------------------------------------------------
// Standalone validation
// ---------------------------------------------------------------------------

/// Validate a configuration without retaining an API facade.
pub fn validate_config(config: &ZuirConfig) -> ZuirResult<()> {
    config.validate()
}

/// Validate a human-readable API operation label.
///
/// The label is diagnostic context only. It does not select a backend,
/// authorize an extension, or grant permission to execute an operation.
pub fn validate_operation_label(operation: &str) -> ZuirResult<()> {
    if operation.trim().is_empty() {
        return Err(api_input_error(
            "The ZUIR API operation label must not be empty.",
        ));
    }

    Ok(())
}

/// Construct a structured error for invalid API-boundary input.
///
/// Configuration invariants should instead be validated by `config.rs`.
pub(crate) fn api_input_error(message: impl Into<String>) -> ZuirError {
    ZuirError::new(ZuirErrorKind::InvalidInput, message)
        .with_phase(ErrorPhase::Configuration)
        .with_note("The request was rejected at the ZUIR public API boundary.")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_api_has_valid_deterministic_configuration() {
        let api = ZuirApi::default();

        assert!(api.config().deterministic());
        assert!(api.validate_config().is_ok());
        assert!(api.validate_request("test.operation").is_ok());
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
    fn run_rejects_an_empty_operation_label_before_calling_the_closure() {
        let api = ZuirApi::default();
        let mut called = false;

        let result = api.run("  ", |_config| {
            called = true;
            Ok(())
        });

        assert!(result.is_err());
        assert!(!called);
        assert_eq!(
            result.expect_err("empty label must fail").kind(),
            ZuirErrorKind::InvalidInput
        );
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
        assert!(
            error.to_string().contains("test.failure")
                || error
                    .context()
                    .notes()
                    .iter()
                    .any(|note| note.contains("test.failure")),
            "the operation label should be available as error context"
        );
    }

    #[test]
    fn run_in_phase_adds_a_phase_when_the_error_has_none() {
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
        assert_eq!(
            error.context().phase(),
            Some(ErrorPhase::Verification)
        );
    }

    #[test]
    fn run_in_phase_preserves_an_existing_precise_phase() {
        let api = ZuirApi::default();

        let result: ZuirResult<()> = api.run_in_phase(
            ErrorPhase::Verification,
            "test.serialization",
            |_config| {
                Err(
                    ZuirError::new(
                        ZuirErrorKind::UnsupportedVersion,
                        "unsupported serialized version",
                    )
                    .with_phase(ErrorPhase::Deserialization),
                )
            },
        );

        let error = result.expect_err("the unsupported version must fail");

        assert_eq!(
            error.context().phase(),
            Some(ErrorPhase::Deserialization)
        );
    }

    #[test]
    fn invalid_custom_budget_is_rejected_by_configuration_validation() {
        let result = ZuirConfig::default().with_budget(
            BudgetKind::Custom("not namespaced".to_owned()),
            WorkBudget::bounded(1),
        );

        assert!(result.is_err());
    }

    #[test]
    fn absent_budgets_do_not_impose_hardware_capacity_limits() {
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

    #[test]
    fn operation_label_validation_does_not_rewrite_valid_labels() {
        let label = "verify.program";
        assert!(validate_operation_label(label).is_ok());
        assert_eq!(label, "verify.program");
    }
}