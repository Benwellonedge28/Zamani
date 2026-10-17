
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Public API Contract Tests
//!
//! Path: `src/classical/tests/api_contract.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Verify the observable public contract of `crate::classical::api`.
//! These tests are intentionally written against public interfaces rather
//! than private implementation details.
//!
//! # Authority
//!
//! - `src/classical/api.rs`: public facade and acceptance policy.
//! - `src/classical/config.rs`: configuration schema and validation.
//! - `src/classical/error.rs`: canonical error classifications and codes.
//! - `src/classical/compatibility.rs`: compatibility descriptors, profiles,
//!   issue codes, reports, and compatibility rules.
//! - `grammar/specification/semantic-model.md`: language-independent meaning.
//! - `grammar/validation/semantic-boundaries.md`: semantic ownership and
//!   target-feasibility boundaries.
//! - `grammar/Zamani.g4`: source grammar authority.
//!
//! # Non-goals
//!
//! These tests do not claim that a compatible artifact is authentic, safe,
//! semantically valid, executable, or feasible on every target.
//!
//! They do not test hardware discovery, runtime allocation, physical resource
//! availability, compiler optimization correctness, or backend execution.
//! Those contracts belong to their canonical subsystem owners.
//!
//! # Scalability
//!
//! Tests must not establish artificial semantic ceilings for the number of
//! processors, devices, values, operations, collection elements, or tensor
//! dimensions. Large-data and resource-exhaustion qualification belongs in
//! the dedicated scalability and stress test suites.
//!
//! # Integration
//!
//! Include this file as a unit-test module from `src/classical/mod.rs`:
//
//! ```ignore
//! #[cfg(test)]
//! #[path = "tests/api_contract.rs"]
//! mod api_contract;
//! ```
//!
//! If the parent module already has a test-module declaration, extend that
//! declaration instead of adding a duplicate. Do not declare a second
//! `classical` module or duplicate the crate-root module.
//!
//! # Completion
//!
//! This file is complete when it compiles against the frozen public API,
//! every assertion passes, and the repository's formatting, build, test,
//! and lint checks pass on the declared Rust toolchain.

use crate::classical::api::{
    assess_artifact_compatibility,
    require_artifact_compatibility,
    validate_configuration,
};
use crate::classical::compatibility::{
    CompatibilityDescriptor,
    CompatibilityIssueCode,
    CompatibilityProfile,
    CompatibilityStatus,
    CompatibilityVersion,
};
use crate::classical::config::{
    ClassicalConfig,
    CONFIG_SCHEMA_VERSION,
};
use crate::classical::error::ClassicalErrorKind;

/// Creates a version without depending on a target or host representation.
fn version(major: u64, minor: u64, patch: u64) -> CompatibilityVersion {
    CompatibilityVersion::new(major, minor, patch)
}

/// Builds a valid artifact descriptor for API contract tests.
fn make_artifact(
    domain: &str,
    contract: CompatibilityVersion,
    semantic: CompatibilityVersion,
    required: &[&str],
    optional: &[&str],
) -> CompatibilityDescriptor {
    CompatibilityDescriptor::new(
        domain,
        contract,
        semantic,
        required.iter().map(|value| (*value).to_owned()),
        optional.iter().map(|value| (*value).to_owned()),
    )
    .expect("test artifact descriptor must satisfy its constructor contract")
}

/// Builds a valid consumer profile for API contract tests.
fn make_consumer(
    domain: &str,
    contract: CompatibilityVersion,
    semantic: CompatibilityVersion,
    features: &[&str],
) -> CompatibilityProfile {
    CompatibilityProfile::new(
        domain,
        contract,
        semantic,
        features.iter().map(|value| (*value).to_owned()),
    )
    .expect("test consumer profile must satisfy its constructor contract")
}

/// Extracts stable public issue codes for deterministic assertions.
fn issue_codes(
    report: &crate::classical::compatibility::CompatibilityReport,
) -> Vec<&'static str> {
    report
        .issues()
        .iter()
        .map(|issue| issue.code().as_str())
        .collect()
}

#[test]
fn default_configuration_is_valid() {
    let config = ClassicalConfig::new();

    assert_eq!(config.schema_version, CONFIG_SCHEMA_VERSION);
    assert!(
        validate_configuration(&config).is_ok(),
        "the canonical default configuration must be valid",
    );
}

#[test]
fn configuration_without_optimizations_remains_valid() {
    let config = ClassicalConfig::without_optimizations();

    assert!(
        validate_configuration(&config).is_ok(),
        "disabling optional optimizations must not invalidate a configuration",
    );
}

#[test]
fn unsupported_configuration_schema_is_rejected() {
    let mut config = ClassicalConfig::new();

    // The next schema identifier is a test input, not a semantic size limit.
    config.schema_version = CONFIG_SCHEMA_VERSION.saturating_add(1);

    let error = validate_configuration(&config)
        .expect_err("an unsupported configuration schema must be rejected");

    assert_eq!(error.kind(), ClassicalErrorKind::InvalidConfiguration);
    assert_eq!(error.code(), "classical.invalid_configuration");
}

#[test]
fn validation_does_not_mutate_configuration() {
    let config = ClassicalConfig::new();
    let before = config.clone();

    validate_configuration(&config)
        .expect("the default configuration must validate");

    assert_eq!(
        config, before,
        "configuration validation must not mutate its input",
    );
}

#[test]
fn compatible_artifact_passes_strict_compatibility_gate() {
    let artifact = make_artifact(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &["classical.numeric.checked_arithmetic"],
        &[],
    );
    let consumer = make_consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &["classical.numeric.checked_arithmetic"],
    );

    let report = require_artifact_compatibility(&artifact, &consumer)
        .expect("all required compatibility conditions are satisfied");

    assert_eq!(report.status(), CompatibilityStatus::Compatible);
    assert!(report.is_compatible());
    assert!(!report.has_blocking_issues());
    assert!(report.issues().is_empty());
}

#[test]
fn missing_required_feature_is_reported_as_incompatible() {
    let artifact = make_artifact(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &["classical.numeric.arbitrary_precision"],
        &[],
    );
    let consumer = make_consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let report = assess_artifact_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(!report.is_compatible());
    assert!(report.has_blocking_issues());
    assert!(report.issues().iter().any(|issue| {
        issue.code() == CompatibilityIssueCode::MissingRequiredFeature
    }));
}

#[test]
fn strict_gate_rejects_missing_required_feature() {
    let artifact = make_artifact(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &["classical.numeric.arbitrary_precision"],
        &[],
    );
    let consumer = make_consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let error = require_artifact_compatibility(&artifact, &consumer)
        .expect_err("a missing required feature must fail the strict gate");

    assert_eq!(error.kind(), ClassicalErrorKind::VersionMismatch);
    assert_eq!(error.code(), "classical.version_mismatch");
}

#[test]
fn unavailable_optional_feature_is_a_warning_not_a_blocker() {
    let artifact = make_artifact(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
        &["classical.numeric.fast_math"],
    );
    let consumer = make_consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let report = assess_artifact_compatibility(&artifact, &consumer);

    assert_eq!(
        report.status(),
        CompatibilityStatus::CompatibleWithWarnings,
    );
    assert!(report.is_compatible());
    assert!(!report.has_blocking_issues());
    assert!(report.issues().iter().any(|issue| {
        issue.code() == CompatibilityIssueCode::MissingOptionalFeature
    }));

    let strict_report =
        require_artifact_compatibility(&artifact, &consumer)
            .expect("a warning without a blocker must pass the strict gate");

    assert_eq!(
        strict_report.status(),
        CompatibilityStatus::CompatibleWithWarnings,
    );
}

#[test]
fn different_contract_domains_are_incompatible() {
    let artifact = make_artifact(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
        &[],
    );
    let consumer = make_consumer(
        "classical.abi",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let report = assess_artifact_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(report.has_blocking_issues());
    assert!(report.issues().iter().any(|issue| {
        issue.code() == CompatibilityIssueCode::DomainMismatch
    }));
}

#[test]
fn incompatible_contract_major_versions_are_rejected() {
    let artifact = make_artifact(
        "classical.ir",
        version(2, 0, 0),
        version(1, 0, 0),
        &[],
        &[],
    );
    let consumer = make_consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let report = assess_artifact_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(report.issues().iter().any(|issue| {
        issue.code() == CompatibilityIssueCode::ContractMajorMismatch
    }));
}

#[test]
fn incompatible_semantic_major_versions_are_rejected() {
    let artifact = make_artifact(
        "classical.ir",
        version(1, 0, 0),
        version(2, 0, 0),
        &[],
        &[],
    );
    let consumer = make_consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let report = assess_artifact_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(report.issues().iter().any(|issue| {
        issue.code() == CompatibilityIssueCode::SemanticMajorMismatch
    }));
}

#[test]
fn assessment_preserves_multiple_blocking_findings() {
    let artifact = make_artifact(
        "classical.ir",
        version(2, 0, 0),
        version(2, 0, 0),
        &[
            "classical.required.alpha",
            "classical.required.beta",
        ],
        &[],
    );
    let consumer = make_consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let report = assess_artifact_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(report.has_blocking_issues());

    // The two version findings and both missing-feature findings must
    // remain visible. No single failure may hide the others.
    assert_eq!(report.issues().len(), 4);
}

#[test]
fn assessment_is_deterministic_for_identical_inputs() {
    let artifact = make_artifact(
        "classical.ir",
        version(2, 0, 0),
        version(2, 0, 0),
        &[
            "classical.required.z",
            "classical.required.a",
            "classical.required.m",
        ],
        &["classical.optional.b"],
    );
    let consumer = make_consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let first = assess_artifact_compatibility(&artifact, &consumer);
    let second = assess_artifact_compatibility(&artifact, &consumer);

    assert_eq!(first.status(), second.status());
    assert_eq!(first.has_blocking_issues(), second.has_blocking_issues());
    assert_eq!(issue_codes(&first), issue_codes(&second));
    assert_eq!(first.issues().len(), second.issues().len());

    for (left, right) in first.issues().iter().zip(second.issues()) {
        assert_eq!(left.code(), right.code());
        assert_eq!(left.subject(), right.subject());
        assert_eq!(left.message(), right.message());
    }
}

#[test]
fn compatibility_assessment_does_not_mutate_inputs() {
    let artifact = make_artifact(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &["classical.required.feature"],
        &["classical.optional.feature"],
    );
    let consumer = make_consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let artifact_before = artifact.clone();
    let consumer_before = consumer.clone();

    let _report = assess_artifact_compatibility(&artifact, &consumer);

    assert_eq!(artifact, artifact_before);
    assert_eq!(consumer, consumer_before);
}

#[test]
fn extensible_feature_identifiers_need_no_new_enum_variant() {
    let future_feature = "classical.future_domain.new_capability";

    let artifact = make_artifact(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[future_feature],
        &[],
    );
    let consumer = make_consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[future_feature],
    );

    let report = require_artifact_compatibility(&artifact, &consumer)
        .expect("a valid namespaced feature must be supported without enum changes");

    assert_eq!(report.status(), CompatibilityStatus::Compatible);
}

#[test]
fn empty_feature_sets_are_valid_when_versions_match() {
    let artifact = make_artifact(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
        &[],
    );
    let consumer = make_consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let report = require_artifact_compatibility(&artifact, &consumer)
        .expect("no declared feature requirements means no feature blocker");

    assert_eq!(report.status(), CompatibilityStatus::Compatible);
    assert!(report.issues().is_empty());
}

#[test]
fn repeated_assessment_does_not_accumulate_hidden_state() {
    let artifact = make_artifact(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &["classical.required.feature"],
        &[],
    );
    let consumer = make_consumer(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let first = assess_artifact_compatibility(&artifact, &consumer);
    let second = assess_artifact_compatibility(&artifact, &consumer);
    let third = assess_artifact_compatibility(&artifact, &consumer);

    assert_eq!(issue_codes(&first), issue_codes(&second));
    assert_eq!(issue_codes(&second), issue_codes(&third));
    assert_eq!(first.status(), third.status());
}
