
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Result Contracts
//!
//! Path: `src/toolchain/result.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the canonical result-facing API for toolchain operations.
//!
//! This module reuses the error authority in `errors.rs`. It does not
//! introduce a second error representation, error taxonomy, or result
//! implementation.
//!
//! The module also provides composable helpers for enriching failed
//! operations with contextual information while preserving their original
//! error classification and causal source.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Public re-exports of the canonical toolchain result aliases.
//! - Public re-exports of the canonical toolchain error types.
//! - Result-level contextual enrichment.
//!
//! This module does not own:
//!
//! - Toolchain error definitions or error codes.
//! - Compiler, frontend, backend, runtime, or provider errors.
//! - Diagnostic rendering or logging.
//! - Retry, recovery, fallback, or cancellation policy.
//! - Build, package, dependency, registry, or deployment outcomes.
//! - Artifact, target, or hardware resource models.
//!
//! `errors.rs` remains the sole owner of `ToolchainError` and its
//! classification.
//!
//! ## Integration
//!
//! The following modules consume this contract:
//!
//! - `api/`
//! - `workspace/`
//! - `manifest/`
//! - `configuration/`
//! - `build/`
//! - `dependencies/`
//! - `packages/`
//! - `registries/`
//! - `targets/`
//! - `external_tools/`
//! - `tasks/`
//! - `commands/`
//! - `environments/`
//! - `deployment/`
//! - `security/`
//! - `diagnostics/`
//! - `agents/`
//!
//! Other repository subsystems retain ownership of their own errors.
//! Toolchain adapters should preserve those errors as causal sources when
//! converting them into a toolchain-boundary error.
//!
//! ## Result semantics
//!
//! `Ok(T)` means that the operation represented by the function completed
//! successfully according to that function's documented contract.
//!
//! `Err(ToolchainError)` means that the operation did not complete
//! successfully.
//!
//! This module does not reinterpret an error as success, silently discard
//! errors, or infer that a partially completed operation succeeded.
//!
//! Cancellation, resource exhaustion, validation failure, incompatibility,
//! and ordinary operation failure remain distinct classifications owned
//! by `ToolchainErrorKind`.
//!
//! ## POCO-REAF and scalability
//!
//! This module introduces no fixed limits on:
//!
//! - Number of projects or workspaces.
//! - Number of dependencies or packages.
//! - Number of targets or providers.
//! - Number of build tasks.
//! - Number of computing domains.
//! - Number of hardware resources.
//! - Number of distributed participants.
//! - Memory, tensor dimensions, or computational capacity.
//!
//! Results are subject only to Rust's representational constraints and the
//! resources available to the executing process.
//!
//! Literal infinite storage or execution is not promised.
//!
//! Operational limits belong to the relevant configuration and execution
//! contracts, not to this result abstraction.

// ============================================================================
// CANONICAL RESULT AND ERROR RE-EXPORTS
// ============================================================================

/// Canonical result type for toolchain operations.
///
/// This is the same type alias defined by `errors.rs`:
///
/// `core::result::Result<T, ToolchainError>`
///
/// Re-exporting it here provides a stable result-focused import path without
/// creating a competing result contract.
pub use super::errors::Result;

/// Explicitly named canonical result type for APIs that prefer an
/// unambiguous toolchain-specific name.
pub use super::errors::ToolchainResult;

/// Canonical toolchain error type.
pub use super::errors::ToolchainError;

/// Canonical toolchain error classification.
pub use super::errors::ToolchainErrorKind;

// ============================================================================
// RESULT CONTEXT EXTENSION
// ============================================================================

/// Extension methods for enriching failed toolchain results.
///
/// These methods leave successful values unchanged.
///
/// When a result contains an error, the methods attach additional context
/// through the canonical `ToolchainError` API.
///
/// Existing error classification and causal source information are
/// preserved.
///
/// # Example
///
/// ```
/// use zamani_compiler::toolchain::result::{
///     Result,
///     ToolchainError,
///     ToolchainErrorExt,
/// };
///
/// fn resolve_dependency() -> Result<()> {
///     Err(ToolchainError::not_found("dependency not found"))
/// }
///
/// let result = resolve_dependency()
///     .with_toolchain_operation("dependency.resolve")
///     .with_toolchain_subject("package:example")
///     .with_toolchain_context("workspace dependency resolution");
///
/// assert!(result.is_err());
/// ```
pub trait ToolchainErrorExt<T> {
    /// Attach an operation identifier to an error result.
    ///
    /// Successful results are returned unchanged.
    ///
    /// If the error already contains an operation identifier, the canonical
    /// `ToolchainError::with_operation` behavior applies.
    fn with_toolchain_operation(
        self,
        operation: impl Into<String>,
    ) -> Self;

    /// Attach a subject identifier or descriptive subject to an error.
    ///
    /// Subjects may describe projects, workspaces, packages, tasks,
    /// registries, providers, environments, deployments, or other
    /// toolchain-owned entities.
    ///
    /// This method does not create or replace canonical typed identities.
    fn with_toolchain_subject(
        self,
        subject: impl Into<String>,
    ) -> Self;

    /// Append contextual information to an error.
    ///
    /// Context is appended in the order in which this method is called.
    /// Existing context and causal source information are preserved.
    ///
    /// Callers must not include credentials, access tokens, private keys,
    /// unredacted secrets, or unnecessarily sensitive source contents.
    fn with_toolchain_context(
        self,
        context: impl Into<String>,
    ) -> Self;
}

impl<T> ToolchainErrorExt<T> for Result<T> {
    fn with_toolchain_operation(
        self,
        operation: impl Into<String>,
    ) -> Self {
        self.map_err(|error| error.with_operation(operation))
    }

    fn with_toolchain_subject(
        self,
        subject: impl Into<String>,
    ) -> Self {
        self.map_err(|error| error.with_subject(subject))
    }

    fn with_toolchain_context(
        self,
        context: impl Into<String>,
    ) -> Self {
        self.map_err(|error| error.with_context(context))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::{
        Result,
        ToolchainError,
        ToolchainErrorExt,
        ToolchainErrorKind,
        ToolchainResult,
    };

    use std::error::Error;
    use std::io;

    #[test]
    fn canonical_result_aliases_are_compatible() {
        fn accepts_result(value: Result<()>) -> ToolchainResult<()> {
            value
        }

        let success: Result<()> = Ok(());

        assert!(accepts_result(success).is_ok());

        let failure: ToolchainResult<()> =
            Err(ToolchainError::cancelled("operation cancelled"));

        let failure = accepts_result(failure);

        assert_eq!(
            failure
                .expect_err("cancellation must remain an error")
                .kind(),
            ToolchainErrorKind::Cancelled
        );
    }

    #[test]
    fn successful_result_is_not_changed_by_context_methods() {
        let result: Result<u64> = Ok(42);

        let result = result
            .with_toolchain_operation("build.execute")
            .with_toolchain_subject("project:sample")
            .with_toolchain_context("compilation stage");

        assert_eq!(result.expect("successful result"), 42);
    }

    #[test]
    fn operation_context_and_subject_are_attached_to_errors() {
        let result: Result<()> =
            Err(ToolchainError::build_failed("compilation failed"));

        let error = result
            .with_toolchain_operation("build.execute")
            .with_toolchain_subject("project:sample")
            .with_toolchain_context("compiler invocation")
            .with_toolchain_context("target realization")
            .expect_err("the build must remain failed");

        assert_eq!(error.kind(), ToolchainErrorKind::BuildFailed);
        assert_eq!(error.code(), "ZT0009");
        assert_eq!(error.message(), "compilation failed");
        assert_eq!(error.operation(), Some("build.execute"));
        assert_eq!(error.subject(), Some("project:sample"));

        assert_eq!(
            error.context(),
            &[
                "compiler invocation".to_owned(),
                "target realization".to_owned(),
            ]
        );
    }

    #[test]
    fn contextual_enrichment_preserves_error_classification() {
        let original =
            ToolchainError::incompatible_version("unsupported package format");

        let original_kind = original.kind();
        let original_code = original.code();
        let original_message = original.message().to_owned();

        let result: Result<()> = Err(original);

        let error = result
            .with_toolchain_operation("package.install")
            .with_toolchain_context("package compatibility check")
            .expect_err("incompatible package must remain an error");

        assert_eq!(error.kind(), original_kind);
        assert_eq!(error.code(), original_code);
        assert_eq!(error.message(), original_message);
    }

    #[test]
    fn contextual_enrichment_preserves_causal_source() {
        let source = io::Error::new(
            io::ErrorKind::NotFound,
            "underlying manifest file does not exist",
        );

        let original = ToolchainError::from(source);

        assert!(original.has_source());

        let result: Result<()> = Err(original);

        let error = result
            .with_toolchain_operation("manifest.load")
            .with_toolchain_subject("project:sample")
            .with_toolchain_context("workspace discovery")
            .expect_err("missing manifest must remain an error");

        assert!(error.has_source());

        let cause = Error::source(&error)
            .expect("the original I/O cause must be preserved");

        assert!(
            cause
                .to_string()
                .contains("underlying manifest file does not exist")
        );
    }

    #[test]
    fn resource_exhaustion_remains_distinct() {
        let result: Result<()> = Err(
            ToolchainError::resource_exhausted(
                "configured build memory budget exhausted",
            ),
        );

        let error = result
            .with_toolchain_operation("build.execute")
            .expect_err("resource exhaustion must remain an error");

        assert_eq!(
            error.kind(),
            ToolchainErrorKind::ResourceExhausted
        );

        assert_ne!(
            error.kind(),
            ToolchainErrorKind::InvalidInput
        );
    }

    #[test]
    fn cancellation_remains_distinct_from_build_failure() {
        let cancelled: Result<()> =
            Err(ToolchainError::cancelled("operation cancelled"));

        let failed: Result<()> =
            Err(ToolchainError::build_failed("build failed"));

        let cancelled = cancelled
            .with_toolchain_operation("task.execute")
            .expect_err("cancelled task must remain an error");

        let failed = failed
            .with_toolchain_operation("task.execute")
            .expect_err("failed task must remain an error");

        assert_eq!(
            cancelled.kind(),
            ToolchainErrorKind::Cancelled
        );

        assert_eq!(
            failed.kind(),
            ToolchainErrorKind::BuildFailed
        );

        assert_ne!(cancelled.code(), failed.code());
    }

    #[test]
    fn error_context_is_appended_in_call_order() {
        let result: Result<()> =
            Err(ToolchainError::provider_failed("provider failed"));

        let error = result
            .with_toolchain_context("provider lookup")
            .with_toolchain_context("capability validation")
            .with_toolchain_context("provider invocation")
            .expect_err("provider failure must remain an error");

        assert_eq!(
            error.context(),
            &[
                "provider lookup".to_owned(),
                "capability validation".to_owned(),
                "provider invocation".to_owned(),
            ]
        );
    }

    #[test]
    fn result_helpers_do_not_change_successful_values() {
        let values = vec![1_u64, 2, 3, 5, 8, 13];

        let result: Result<Vec<u64>> = Ok(values.clone());

        let actual = result
            .with_toolchain_operation("test.collect")
            .with_toolchain_subject("fixture:sequence")
            .with_toolchain_context("successful collection")
            .expect("successful collection");

        assert_eq!(actual, values);
    }
}
