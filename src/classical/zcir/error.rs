
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Error Integration
//!
//! Path: `src/classical/zcir/error.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Provide the error-construction boundary for Zamani's Classical
//! Intermediate Representation (ZCIR).
//!
//! ZCIR reuses the canonical ZUIR error taxonomy and result contract.
//! This module supplies classical-IR-specific convenience constructors
//! without introducing a competing error type, error-code registry,
//! diagnostic system, source-location model, or recovery policy.
//!
//! ## AUTHORITY
//!
//! This module conforms to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/diagnostics.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/errors.rs`
//! - `src/zuir/diagnostics.rs`
//! - `src/zuir/source_map.rs`
//!
//! The existing ZUIR error contract is authoritative for error categories,
//! stable codes, structured context, typed entity references, version
//! context, and standard error chaining.
//!
//! ## OWNS
//!
//! - The ZCIR-facing error-construction helpers.
//! - The `ZcirError` and `ZcirResult<T>` public aliases.
//! - Documentation of how classical IR failures map to canonical errors.
//! - Tests for this module's mapping and construction behavior.
//!
//! ## DOES NOT OWN
//!
//! - A second error taxonomy or error-code registry.
//! - ZUIR error identity, implementation, or compatibility policy.
//! - Source spans, source maps, diagnostic severity, rendering, or fix-its.
//! - Type-checking or structural-verification algorithms.
//! - Semantic analysis or source-language validity.
//! - Resource discovery, allocation, scheduling, or target selection.
//! - Serialization formats, schema migration, or lowering implementations.
//! - Whether compilation can recover after a failure.
//!
//! ## ERROR CLASSIFICATION
//!
//! Error category and processing phase are separate concepts.
//! A malformed representation is a representation failure; exhaustion of
//! an explicitly configured work budget is an operational failure.
//! A missing target capability is not automatically an invalid source
//! program. Callers must select the category that describes the actual
//! failure rather than using one generic error for every situation.
//!
//! ## SCALABILITY
//!
//! This module imposes no universal limits on operation counts, graph sizes,
//! collection lengths, integer widths, processor counts, memory capacity,
//! tensor rank, or other hardware populations.
//!
//! Error messages are dynamically sized strings. ZUIR context collections
//! use the canonical error implementation and grow dynamically.
//!
//! Actual allocation and execution remain subject to available resources.
//! Callers may enforce explicit operational budgets, but exhaustion of a
//! budget must not be reported as proof that a program is semantically
//! invalid.
//!
//! ## DEPENDENCY DIRECTION
//!
//! ```text
//! src/zuir/errors.rs
//!          |
//!          v
//! src/classical/zcir/error.rs
//!          |
//!          +--> builders
//!          +--> verifier
//!          +--> analysis
//!          +--> transformations
//!          +--> serialization adapters
//!          +--> lowering adapters
//! ```
//!
//! This module may depend on canonical ZUIR errors. ZUIR's foundational
//! error module must not depend on ZCIR.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `zcir/mod.rs` must declare `pub mod error;` after this file exists.
//! - ZCIR consumers should import `ZcirError`, `ZcirResult`, and the
//!   re-exported canonical error context types from this module.
//! - Existing ZUIR error categories and codes remain canonical.
//! - Verification and transformation code must attach the relevant
//!   `ErrorSubject` when a canonical IR entity is available.
//! - Version-aware code must attach canonical `VersionErrorContext`
//!   rather than embedding all version details only in prose.
//! - Source locations and diagnostic severity must be attached through
//!   the established diagnostic/source-map APIs, not added to this error.
//! - Resource-budget failures must use an operational category.
//! - Unsupported target capabilities must not be mislabeled as invalid
//!   source semantics unless a separate semantic rule was actually violated.
//! - This module must not be registered from `src/lib.rs` independently;
//!   the intended path is through the classical and ZCIR module roots.
//!
//! ## FREEZE CRITERIA
//!
//! - Canonical error types are reused rather than duplicated.
//! - Error categories and phase mappings are covered by tests.
//! - Error sources remain available through standard error chaining.
//! - Callers can add subjects, version context, artifact paths, and notes
//!   using the canonical ZUIR builder methods.
//! - No unsafe code or universal scalability ceiling is introduced.
//! - The module compiles against the actual ZUIR API and module graph.

use crate::zuir::errors::{
    ErrorPhase, ErrorSubject, VersionErrorContext, ZuirError, ZuirErrorKind,
};

/// Canonical error type used by ZCIR.
///
/// This is an alias, not a second error implementation. It preserves
/// compatibility with ZUIR consumers and retains the canonical structured
/// context and error-source chaining behavior.
pub use crate::zuir::errors::{
    ErrorPhase as ZcirErrorPhase,
    ErrorSubject as ZcirErrorSubject,
    VersionErrorContext as ZcirVersionErrorContext,
};

/// Result type for fallible ZCIR operations.
///
/// The error value is the canonical ZUIR error. ZCIR-specific errors must
/// not create an incompatible result type at subsystem boundaries.
pub type ZcirResult<T> = Result<T, ZcirError>;

/// Canonical error alias for callers that prefer the ZCIR-facing name.
pub type ZcirError = ZuirError;

/// Constructs a canonical error from an existing ZUIR error category.
///
/// Use this when a more specialized helper would misrepresent the failure.
/// The caller may subsequently attach canonical context with methods such as
/// `with_subject`, `with_phase`, `with_version`, `with_artifact_path`,
/// `with_related_subject`, `with_note`, and `with_source`.
#[must_use]
pub fn error(kind: ZuirErrorKind, message: impl Into<String>) -> ZcirError {
    ZcirError::new(kind, message)
}

/// Constructs an invalid-input error.
///
/// Use for a rejected API argument or caller-supplied input that violates
/// the API's explicit input contract. Do not use this as a substitute for
/// a verifier's diagnosis of an invalid completed representation.
#[must_use]
pub fn invalid_input(message: impl Into<String>) -> ZcirError {
    ZcirError::new(ZuirErrorKind::InvalidInput, message)
        .with_phase(ErrorPhase::Construction)
}

/// Constructs an error for a malformed or internally inconsistent
/// representation.
///
/// This helper describes the failure category; it does not perform
/// structural verification.
#[must_use]
pub fn malformed_representation(message: impl Into<String>) -> ZcirError {
    ZcirError::new(ZuirErrorKind::MalformedRepresentation, message)
        .with_phase(ErrorPhase::Verification)
}

/// Constructs an error for an invalid structural invariant.
///
/// Examples include an invalid graph relationship or a required structural
/// element missing from a representation. The responsible verifier must
/// establish the actual invariant violation.
#[must_use]
pub fn invalid_structure(message: impl Into<String>) -> ZcirError {
    malformed_representation(message)
}

/// Constructs an error for an invalid type relationship discovered while
/// validating the classical representation.
#[must_use]
pub fn invalid_type(message: impl Into<String>) -> ZcirError {
    ZcirError::new(ZuirErrorKind::InvalidType, message)
        .with_phase(ErrorPhase::Verification)
}

/// Constructs an error when required verification invariants could not
/// be established.
///
/// Use this for a failed verification result, not for operational failures
/// such as cancellation or an exhausted work budget.
#[must_use]
pub fn verification_failed(message: impl Into<String>) -> ZcirError {
    ZcirError::new(ZuirErrorKind::VerificationFailed, message)
        .with_phase(ErrorPhase::Verification)
}

/// Constructs an error for a transformation that failed to satisfy its
/// declared transformation contract.
#[must_use]
pub fn transformation_failed(message: impl Into<String>) -> ZcirError {
    ZcirError::new(ZuirErrorKind::TransformationFailed, message)
        .with_phase(ErrorPhase::Transformation)
}

/// Constructs an error when an operation is unsupported in the current
/// transformation context.
///
/// This does not imply that the operation is invalid in the source language
/// or unsupported by every possible target.
#[must_use]
pub fn unsupported_operation(message: impl Into<String>) -> ZcirError {
    ZuirError::new(ZuirErrorKind::UnsupportedOperation, message)
        .with_phase(ErrorPhase::Transformation)
}

/// Constructs an error when lowering cannot preserve or realize a required
/// semantic property.
#[must_use]
pub fn lowering_unsupported(message: impl Into<String>) -> ZcirError {
    ZuirError::new(ZuirErrorKind::LoweringUnsupported, message)
        .with_phase(ErrorPhase::Lowering)
}

/// Constructs an error when a required capability is unavailable.
///
/// Capability absence is a feasibility or realization failure; it must not
/// automatically be treated as a source-language semantic error.
#[must_use]
pub fn capability_unavailable(message: impl Into<String>) -> ZcirError {
    ZuirError::new(ZuirErrorKind::CapabilityUnavailable, message)
}

/// Constructs an error when a version is unsupported by the selected
/// compatibility policy.
///
/// Callers with structured version information should additionally attach
/// `ZcirVersionErrorContext` through `with_version`.
#[must_use]
pub fn unsupported_version(message: impl Into<String>) -> ZcirError {
    ZuirError::new(ZuirErrorKind::UnsupportedVersion, message)
        .with_phase(ErrorPhase::Compatibility)
}

/// Constructs an error when explicit migration is required before the
/// supplied representation may be interpreted.
#[must_use]
pub fn migration_required(message: impl Into<String>) -> ZcirError {
    ZuirError::new(ZuirErrorKind::MigrationRequired, message)
        .with_phase(ErrorPhase::Migration)
}

/// Constructs an error when an explicit operational resource budget or
/// available process resource has been exhausted.
///
/// This does not mean that the source program is invalid or that the
/// language imposes a universal capacity limit.
#[must_use]
pub fn resource_exhausted(message: impl Into<String>) -> ZcirError {
    ZuirError::new(ZuirErrorKind::ResourceExhausted, message)
}

/// Constructs an error when an explicitly configured operational limit
/// has been reached.
///
/// Callers should identify the applicable budget in structured context or
/// an explanatory note. Do not turn an implementation budget into a
/// universal language-level maximum.
#[must_use]
pub fn operational_limit_exceeded(message: impl Into<String>) -> ZcirError {
    ZuirError::new(ZuirErrorKind::LimitExceeded, message)
}

/// Constructs an error for invalid or contradictory ZCIR configuration.
#[must_use]
pub fn configuration_invalid(message: impl Into<String>) -> ZcirError {
    ZuirError::new(ZuirErrorKind::ConfigurationInvalid, message)
        .with_phase(ErrorPhase::Configuration)
}

/// Constructs an internal integration-contract failure.
///
/// Reserve this for implementation invariants or subsystem contract failures.
/// Do not use it to report invalid user input or expected target infeasibility.
#[must_use]
pub fn internal_failure(message: impl Into<String>) -> ZcirError {
    ZuirError::new(ZuirErrorKind::InternalFailure, message)
}

/// Adds a canonical entity subject to an error.
///
/// This helper is useful when an error is produced by a generic adapter and
/// the affected entity becomes known only at the integration boundary.
#[must_use]
pub fn with_subject(error: ZcirError, subject: ErrorSubject) -> ZcirError {
    error.with_subject(subject)
}

/// Adds canonical version context to an error.
///
/// Schema, format, dialect, and language versions remain distinct through
/// the canonical `VersionErrorContext` type.
#[must_use]
pub fn with_version_context(
    error: ZcirError,
    context: VersionErrorContext,
) -> ZcirError {
    error.with_version(context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;
    use std::io;

    #[test]
    fn aliases_use_the_canonical_zuiR_error_contract() {
        let result: ZcirResult<()> = Err(invalid_input("invalid builder argument"));
        let error = result.expect_err("the supplied argument is invalid");

        assert_eq!(error.kind(), ZuirErrorKind::InvalidInput);
        assert_eq!(error.code().as_str(), "ZMN-IR-INVALID-INPUT");
        assert_eq!(error.context().phase(), Some(ErrorPhase::Construction));
    }

    #[test]
    fn malformed_structure_is_not_reported_as_resource_exhaustion() {
        let error = invalid_structure("a required control-flow edge is missing");

        assert_eq!(error.kind(), ZuirErrorKind::MalformedRepresentation);
        assert_eq!(error.context().phase(), Some(ErrorPhase::Verification));
        assert!(!error.kind().is_operational_failure());
    }

    #[test]
    fn resource_exhaustion_is_an_operational_failure() {
        let error = resource_exhausted("the configured verification work budget was exhausted");

        assert_eq!(error.kind(), ZuirErrorKind::ResourceExhausted);
        assert!(error.kind().is_operational_failure());
        assert!(!error.kind().is_representation_failure());
    }

    #[test]
    fn target_capability_failure_is_not_automatically_source_invalidity() {
        let error = capability_unavailable(
            "the selected target does not provide the required operation capability",
        );

        assert_eq!(error.kind(), ZuirErrorKind::CapabilityUnavailable);
        assert!(error.kind().is_compatibility_or_realization_failure());
    }

    #[test]
    fn lowering_and_migration_errors_have_explicit_phases() {
        let lowering = lowering_unsupported("the target cannot preserve a required effect");
        let migration = migration_required("the artifact must be migrated before interpretation");

        assert_eq!(lowering.context().phase(), Some(ErrorPhase::Lowering));
        assert_eq!(migration.context().phase(), Some(ErrorPhase::Migration));
    }

    #[test]
    fn canonical_subject_and_version_context_are_preserved() {
        let error = invalid_type("the operation result type is incompatible")
            .with_note("the type verifier detected the mismatch");

        assert_eq!(error.kind(), ZuirErrorKind::InvalidType);
        assert_eq!(error.context().notes().len(), 1);

        let supplied = crate::zuir::version::ZuirSchemaVersion::new(1, 0, 0);
        let expected = crate::zuir::version::ZuirSchemaVersion::new(2, 0, 0);

        let error = with_version_context(
            error,
            VersionErrorContext::Schema { supplied, expected },
        );

        assert!(matches!(
            error.context().version(),
            Some(VersionErrorContext::Schema {
                supplied: actual_supplied,
                expected: actual_expected,
            }) if *actual_supplied == supplied && *actual_expected == expected
        ));
    }

    #[test]
    fn underlying_error_chains_are_preserved() {
        let error = ZcirError::from_source(
            ZuirErrorKind::IoFailure,
            "failed to read a ZCIR artifact",
            io::Error::other("simulated I/O failure"),
        );

        assert!(error.source_error().is_some());
        assert!(StdError::source(&error).is_some());
    }

    #[test]
    fn error_codes_remain_canonical_and_machine_readable() {
        let cases = [
            (
                invalid_input("bad argument"),
                "ZMN-IR-INVALID-INPUT",
            ),
            (
                verification_failed("verification did not succeed"),
                "ZMN-IR-VERIFICATION-FAILED",
            ),
            (
                transformation_failed("transformation did not complete"),
                "ZMN-IR-TRANSFORMATION-FAILED",
            ),
            (
                unsupported_version("unsupported artifact version"),
                "ZMN-IR-VERSION-UNSUPPORTED",
            ),
        ];

        for (error, expected_code) in cases {
            assert_eq!(error.code().as_str(), expected_code);
        }
    }
}
