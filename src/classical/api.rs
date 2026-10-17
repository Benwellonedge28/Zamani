
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Public API Boundary
//!
//! Path: `src/classical/api.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Exposes a stable, target-independent API for classical configuration
//! validation and artifact compatibility assessment.
//!
//! This module is a facade over existing canonical subsystem owners. It does
//! not implement a compiler, interpreter, execution engine, second IR, type
//! system, resource allocator, hardware abstraction layer, or backend.
//!
//! # Authority and ownership
//!
//! This module owns:
//! - the public entry points documented below;
//! - orchestration of configuration validation;
//! - orchestration of compatibility assessment;
//! - the strict compatibility acceptance policy at this API boundary.
//!
//! Canonical ownership remains with:
//! - `config.rs`: classical configuration and its validation rules;
//! - `error.rs`: classical error taxonomy and `ClassicalResult<T>`;
//! - `compatibility.rs`: compatibility descriptors, profiles, issues,
//!   reports, and compatibility assessment rules;
//! - `diagnostics.rs` and `source_map`: structured diagnostics and source data;
//! - the frontend and semantic subsystem: syntax and program meaning;
//! - the canonical classical IR: classical computation representation;
//! - `quantum::ir`: quantum and hybrid IR contracts;
//! - `zuir`: shared universal IR and identity contracts;
//! - execution and target subsystems: actual resource feasibility,
//!   placement, scheduling, lowering, and execution.
//!
//! # POCO-REAF and scalability
//!
//! This API imposes no universal maximum on source size, operation count,
//! collection size, tensor rank, processor count, device count, or node count.
//!
//! It does not promise physically infinite resources or claim that every
//! target can execute every program. Representational constraints, configured
//! operational budgets, target capabilities, and resource exhaustion must
//! be handled by their respective owners.
//!
//! An unsupported target capability is a feasibility or compilation issue,
//! not automatically a source-language semantic error.
//!
//! # Integration contract
//!
//! Dependency direction:
//!
//! ```text
//! frontend / AST / semantic model
//!               |
//!               v
//! classical API facade
//!       |               |
//!       v               v
//! configuration    compatibility
//!       |               |
//!       +-------+-------+
//!               |
//!               v
//! validation / canonical IR adapters
//!               |
//!               v
//! optimization / lowering / resource planning
//!               |
//!               v
//! target execution and runtime
//! ```
//!
//! This facade must not create circular dependencies or replace any canonical
//! representation. Callers requiring program compilation or execution must
//! use the appropriate pipeline owner after those contracts are established.
//!
//! # Error policy
//!
//! Configuration failures are converted to `ClassicalError` while retaining
//! the underlying validation explanation in the error message.
//!
//! Compatibility assessment preserves the complete report. Strict assessment
//! rejects blocking compatibility issues but accepts non-blocking warnings.
//! It never migrates artifacts, weakens requirements, or silently changes
//! program semantics.
//!
//! # Stability
//!
//! Public signatures and error guarantees are compatibility contracts.
//! Changes require review of existing callers and downstream integrations.

use crate::classical::compatibility::{
    assess_compatibility as assess_compatibility_contract,
    CompatibilityDescriptor,
    CompatibilityProfile,
    CompatibilityReport,
};
use crate::classical::config::ClassicalConfig;
use crate::classical::error::{ClassicalError, ClassicalResult};

/// Validates classical configuration through its canonical owner.
///
/// This function validates configuration only. It does not discover
/// hardware, allocate resources, compile a program, or execute computation.
///
/// # Errors
///
/// Returns [`ClassicalError`] with the invalid-configuration classification
/// if the configuration is internally invalid or violates its declared
/// configuration policy.
///
/// The configuration owner remains responsible for the precise validation
/// rules and for distinguishing operational budgets from language semantics.
pub fn validate_configuration(config: &ClassicalConfig) -> ClassicalResult<()> {
    config
        .validate()
        .map_err(|error| ClassicalError::invalid_configuration(error.to_string()))
}

/// Assesses an artifact's declared compatibility with a consumer.
///
/// Returns the complete report, including blocking issues and non-blocking
/// warnings. The result describes compatibility only; it does not establish
/// artifact authenticity, integrity, safety, or semantic validity.
///
/// Callers must validate artifact contents and canonical IR invariants
/// separately using the appropriate subsystem.
///
/// # Determinism
///
/// For identical descriptors and profiles, the result is expected to be
/// identical, as guaranteed by the compatibility subsystem.
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
/// On success, returns the complete compatibility report. Non-blocking
/// warnings are preserved for callers to inspect and report.
///
/// # Errors
///
/// Returns a version-mismatch error when the compatibility report contains
/// blocking issues. The error identifies the compatibility domain, expected
/// contract and semantic versions, actual versions, and blocking issue codes.
///
/// Use [`assess_artifact_compatibility`] when the caller needs the complete
/// report even when the artifact is incompatible.
///
/// # Important
///
/// This function does not perform a migration or modify either argument.
/// An explicit migration system, if introduced, must have its own contract,
/// semantic-preservation rules, and failure reporting.
pub fn require_artifact_compatibility(
    artifact: &CompatibilityDescriptor,
    consumer: &CompatibilityProfile,
) -> ClassicalResult<CompatibilityReport> {
    let report = assess_artifact_compatibility(artifact, consumer);

    if report.has_blocking_issues() {
        // Compatibility reports are owned by the compatibility subsystem.
        // Preserve its issue ordering when constructing the error summary.
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
        CompatibilityDescriptor,
        CompatibilityProfile,
        CompatibilityStatus,
        CompatibilityVersion,
    };
    use crate::classical::error::ClassicalErrorKind;

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
            required_features.iter().map(|feature| (*feature).to_owned()),
            optional_features.iter().map(|feature| (*feature).to_owned()),
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
        let artifact = artifact(
            &["classical.numeric.checked_arithmetic"],
            &[],
        );
        let consumer = consumer(
            &["classical.numeric.checked_arithmetic"],
        );

        let report = require_artifact_compatibility(&artifact, &consumer)
            .expect("compatible artifact must pass");

        assert_eq!(report.status(), CompatibilityStatus::Compatible);
        assert!(!report.has_blocking_issues());
        assert!(report.issues().is_empty());
    }

    #[test]
    fn missing_required_feature_fails_strict_gate() {
        let artifact = artifact(
            &["classical.numeric.arbitrary_precision"],
            &[],
        );
        let consumer = consumer(&[]);

        let error = require_artifact_compatibility(&artifact, &consumer)
            .expect_err("missing required feature must fail");

        assert_eq!(error.kind(), ClassicalErrorKind::VersionMismatch);
        assert_eq!(error.code(), "classical.version_mismatch");
    }

    #[test]
    fn optional_feature_warning_does_not_fail_strict_gate() {
        let artifact = artifact(
            &[],
            &["classical.numeric.fast_math"],
        );
        let consumer = consumer(&[]);

        let report = require_artifact_compatibility(&artifact, &consumer)
            .expect("a non-blocking optional-feature warning must not fail");

        assert_eq!(
            report.status(),
            CompatibilityStatus::CompatibleWithWarnings,
        );
        assert!(!report.has_blocking_issues());
        assert_eq!(report.issues().len(), 1);
    }

    #[test]
    fn assessment_preserves_all_report_details() {
        let artifact = artifact(
            &[
                "classical.required.alpha",
                "classical.required.beta",
            ],
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

        let _report = assess_artifact_compatibility(&artifact, &consumer);

        assert_eq!(artifact, artifact_before);
        assert_eq!(consumer, consumer_before);
    }

    #[test]
    fn assessment_is_deterministic_for_identical_inputs() {
        let artifact = artifact(
            &[
                "classical.required.z",
                "classical.required.a",
            ],
            &["classical.optional.b"],
        );
        let consumer = consumer(&[]);

        let first = assess_artifact_compatibility(&artifact, &consumer);
        let second = assess_artifact_compatibility(&artifact, &consumer);

        assert_eq!(first, second);
    }

    #[test]
    fn incompatible_artifacts_are_not_silently_migrated() {
        let artifact = artifact(&["classical.required"], &[]);
        let consumer = consumer(&[]);

        assert!(
            require_artifact_compatibility(&artifact, &consumer).is_err(),
            "incompatible artifacts must be rejected, not silently migrated",
        );
    }

    #[test]
    fn compatibility_assessment_does_not_change_the_consumer_profile() {
        let artifact = artifact(&[], &[]);
        let consumer = consumer(&["classical.numeric.checked_arithmetic"]);
        let before = consumer.clone();

        let _report = assess_artifact_compatibility(&artifact, &consumer);

        assert_eq!(consumer, before);
    }

    #[test]
    fn feature_identifiers_remain_extensible() {
        let feature = "classical.future_domain.new_capability";

        let artifact = artifact(&[feature], &[]);
        let consumer = consumer(&[feature]);

        let report = require_artifact_compatibility(&artifact, &consumer)
            .expect("a registered feature identifier must not require a code change");

        assert_eq!(report.status(), CompatibilityStatus::Compatible);
    }
}
