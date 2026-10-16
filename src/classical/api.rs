
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Public API Boundary
//!
//! Path: `src/classical/api.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide a stable, target-independent entry point for classical subsystem
//! configuration validation and compatibility checks.
//!
//! This API is deliberately a façade. It does not implement a competing
//! compiler pipeline, IR, type system, runtime, resource manager, or backend.
//!
//! # Ownership
//!
//! This module owns the public classical API boundary and the composition
//! rules implemented by its functions.
//!
//! It does not own:
//! - classical configuration semantics (`config.rs`);
//! - classical error classification (`error.rs`);
//! - compatibility rules or report structures (`compatibility.rs`);
//! - diagnostics rendering or source spans;
//! - canonical AST, semantic types, identities, or IR;
//! - hardware discovery, resource negotiation, scheduling, or execution.
//!
//! # POCO-REAF
//!
//! The public API does not impose universal limits on program size, hardware
//! counts, tensor dimensions, collection sizes, or execution scale.
//!
//! Configuration budgets, target capabilities, representation limits, and
//! actual resource availability remain explicit responsibilities of their
//! owning subsystems. This API does not claim that finite resources are
//! infinite or that every target can execute every program.
//!
//! # Integration contract
//!
//! - `config.rs` validates configuration using its canonical validator.
//! - `error.rs` supplies `ClassicalError` and `ClassicalResult<T>`.
//! - `compatibility.rs` supplies compatibility descriptors, profiles,
//!   reports, and assessment rules.
//! - `validation/` may use this module as a public entry point but retains
//!   ownership of structural, type, effect, resource, and IR validation.
//! - `lowering/` must preserve canonical IR ownership and semantics.
//! - `execution/` must check real capabilities and resource availability.
//! - `src/zuir/ids.rs` and `src/source_map.rs` retain their canonical owners.
//! - `grammar/specification/` remains the normative language authority.
//!
//! # Failure behavior
//!
//! Configuration errors are translated into the canonical classical error
//! boundary without discarding their human-readable explanation.
//!
//! Compatibility assessment returns the complete report, including warnings.
//! The strict compatibility gate returns an error if any blocking issue is
//! present. It never executes migrations or silently changes semantics.
//!
//! # Freeze rule
//!
//! Changes to exported function signatures or their documented guarantees
//! require a compatibility review. New compiler and execution entry points
//! should be added only after their underlying pipeline contracts are frozen.

use crate::classical::compatibility::{
    assess_compatibility as assess_compatibility_contract,
    CompatibilityDescriptor,
    CompatibilityProfile,
    CompatibilityReport,
};
use crate::classical::config::ClassicalConfig;
use crate::classical::error::{ClassicalError, ClassicalResult};

/// Validates classical configuration through the canonical configuration
/// owner.
///
/// # Errors
///
/// Returns `ClassicalError::InvalidConfiguration` if the configuration is
/// internally invalid or violates a configured policy.
///
/// This function does not probe hardware, allocate resources, compile code,
/// or execute the program.
pub fn validate_configuration(config: &ClassicalConfig) -> ClassicalResult<()> {
    config
        .validate()
        .map_err(|error| ClassicalError::invalid_configuration(error.to_string()))
}

/// Assesses an artifact's declared compatibility with a consumer.
///
/// This function returns the complete compatibility report rather than
/// collapsing warnings and blocking issues into a boolean.
///
/// A compatible report is not proof that an artifact is well-formed, safe,
/// authentic, or semantically valid. Those checks belong to serialization,
/// verification, security, and semantic-validation owners.
#[must_use]
pub fn assess_artifact_compatibility(
    artifact: &CompatibilityDescriptor,
    consumer: &CompatibilityProfile,
) -> CompatibilityReport {
    assess_compatibility_contract(artifact, consumer)
}

/// Requires an artifact to satisfy the consumer's blocking compatibility
/// requirements.
///
/// Returns the complete report on success, preserving any non-blocking
/// warnings for the caller.
///
/// # Errors
///
/// Returns `ClassicalError::VersionMismatch` when the report contains blocking
/// issues. The error includes the compatibility domain and a deterministic
/// summary of the reported issue codes.
///
/// The detailed report remains available from [`assess_artifact_compatibility`]
/// when callers need per-feature diagnostics or remediation information.
///
/// This function never migrates an artifact or weakens a required feature.
pub fn require_artifact_compatibility(
    artifact: &CompatibilityDescriptor,
    consumer: &CompatibilityProfile,
) -> ClassicalResult<CompatibilityReport> {
    let report = assess_artifact_compatibility(artifact, consumer);

    if report.has_blocking_issues() {
        let issue_codes = report
            .issues()
            .iter()
            .filter(|issue| issue.code().is_blocking())
            .map(|issue| issue.code().as_str())
            .collect::<Vec<_>>()
            .join(", ");

        return Err(ClassicalError::VersionMismatch {
            subject: artifact.domain().to_owned(),
            expected: format!(
                "compatible with consumer contract {} and semantic {}",
                consumer.contract_version(),
                consumer.semantic_version(),
            ),
            actual: format!(
                "artifact contract {} and semantic {}; blocking issues: {}",
                artifact.contract_version(),
                artifact.semantic_version(),
                issue_codes,
            ),
        });
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classical::compatibility::{
        CompatibilityDescriptor, CompatibilityProfile, CompatibilityStatus,
        CompatibilityVersion,
    };

    fn version(major: u64, minor: u64, patch: u64) -> CompatibilityVersion {
        CompatibilityVersion::new(major, minor, patch)
    }

    fn artifact(
        required_features: &[&str],
        optional_features: &[&str],
    ) -> CompatibilityDescriptor {
        CompatibilityDescriptor::new(
            "classical.ir",
            version(1, 0, 0),
            version(1, 0, 0),
            required_features
                .iter()
                .map(|feature| (*feature).to_owned()),
            optional_features
                .iter()
                .map(|feature| (*feature).to_owned()),
        )
        .expect("test artifact must be valid")
    }

    fn consumer(features: &[&str]) -> CompatibilityProfile {
        CompatibilityProfile::new(
            "classical.ir",
            version(1, 0, 0),
            version(1, 0, 0),
            features.iter().map(|feature| (*feature).to_owned()),
        )
        .expect("test consumer must be valid")
    }

    #[test]
    fn compatible_artifact_passes_strict_gate() {
        let artifact = artifact(&["classical.numeric.checked_arithmetic"], &[]);
        let consumer = consumer(&["classical.numeric.checked_arithmetic"]);

        let report = require_artifact_compatibility(&artifact, &consumer)
            .expect("compatible artifact must pass");

        assert_eq!(report.status(), CompatibilityStatus::Compatible);
        assert!(report.issues().is_empty());
    }

    #[test]
    fn missing_required_feature_fails_strict_gate() {
        let artifact = artifact(&["classical.numeric.arbitrary_precision"], &[]);
        let consumer = consumer(&[]);

        let error = require_artifact_compatibility(&artifact, &consumer)
            .expect_err("missing required feature must fail");

        assert_eq!(
            error.kind(),
            crate::classical::error::ClassicalErrorKind::VersionMismatch
        );
        assert_eq!(error.code(), "classical.version_mismatch");
    }

    #[test]
    fn optional_feature_warning_does_not_fail_strict_gate() {
        let artifact = artifact(&[], &["classical.numeric.fast_math"]);
        let consumer = consumer(&[]);

        let report = require_artifact_compatibility(&artifact, &consumer)
            .expect("optional unsupported feature is non-blocking");

        assert_eq!(
            report.status(),
            CompatibilityStatus::CompatibleWithWarnings
        );
        assert_eq!(report.issues().len(), 1);
    }

    #[test]
    fn assessment_preserves_all_report_details() {
        let artifact = artifact(
            &["classical.required.alpha", "classical.required.beta"],
            &["classical.optional.gamma"],
        );
        let consumer = consumer(&[]);

        let report = assess_artifact_compatibility(&artifact, &consumer);

        assert_eq!(report.status(), CompatibilityStatus::Incompatible);
        assert_eq!(report.issues().len(), 3);
        assert!(report.has_blocking_issues());
    }

    #[test]
    fn compatibility_checks_do_not_mutate_descriptors() {
        let artifact = artifact(&["classical.required"], &[]);
        let consumer = consumer(&[]);
        let artifact_before = artifact.clone();
        let consumer_before = consumer.clone();

        let _ = assess_artifact_compatibility(&artifact, &consumer);

        assert_eq!(artifact, artifact_before);
        assert_eq!(consumer, consumer_before);
    }

    #[test]
    fn report_is_deterministic_for_identical_inputs() {
        let artifact = artifact(
            &["classical.required.z", "classical.required.a"],
            &["classical.optional.b"],
        );
        let consumer = consumer(&[]);

        let first = assess_artifact_compatibility(&artifact, &consumer);
        let second = assess_artifact_compatibility(&artifact, &consumer);

        assert_eq!(first, second);
    }

    #[test]
    fn public_api_has_no_implicit_migration_behavior() {
        let artifact = artifact(&["classical.required"], &[]);
        let consumer = consumer(&[]);

        assert!(
            require_artifact_compatibility(&artifact, &consumer).is_err(),
            "incompatible artifacts must be rejected, not silently migrated"
        );
    }
}
