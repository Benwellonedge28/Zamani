
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Stress and Scalability Tests
//!
//! Path: src/classical/tests/stress.rs
//! Rust edition: 2021
//! Intended minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Exercise the existing classical compatibility contract under increasing
//! input sizes and adversarial-but-valid boundary cases.
//!
//! These tests are not a benchmark suite and do not prove infinite capacity.
//! They verify that the implementation does not introduce arbitrary semantic
//! ceilings, lose feature identifiers, become order-dependent, overflow
//! version comparisons, or silently accept missing required features.
//!
//! # Ownership
//!
//! This file owns test workloads and test assertions only.
//!
//! It does not own:
//! - classical language semantics;
//! - compatibility policy or production error types;
//! - canonical classical or quantum IR;
//! - memory allocation policy or runtime resource discovery;
//! - hardware selection, placement, or scheduling.
//!
//! # Integration
//!
//! Register this file through `src/classical/tests/mod.rs` using:
//!
//!     mod stress;
//!
//! The parent test module must itself be registered by the classical module
//! root. Import paths below assume the root exposes `classical::compatibility`.
//!
//! The current implementation of `CompatibilityIssueCode` must also support
//! the ordering used by `assess_compatibility`. Its `.cmp()` call requires
//! `Ord`/`PartialOrd` derives or an equivalent explicit sort key.
//!
//! # Scalability policy
//!
//! Workload sizes in ordinary tests are test inputs, not language limits.
//! The extended stress test is ignored by default and can be run explicitly.
//! It accepts an operator-selected workload size from the environment; no
//! fixed maximum is imposed by this test module.
//!
//! A test that exhausts host resources is not evidence of a language-level
//! maximum. Production code must report operational failures safely.

use crate::classical::compatibility::{
    assess_compatibility, CompatibilityDescriptor, CompatibilityIssueCode,
    CompatibilityProfile, CompatibilityStatus, CompatibilityVersion,
};

const TEST_DOMAIN: &str = "classical.ir";

fn version(major: u64, minor: u64, patch: u64) -> CompatibilityVersion {
    CompatibilityVersion::new(major, minor, patch)
}

fn descriptor(
    contract: CompatibilityVersion,
    semantics: CompatibilityVersion,
    required: impl IntoIterator<Item = String>,
    optional: impl IntoIterator<Item = String>,
) -> CompatibilityDescriptor {
    CompatibilityDescriptor::new(
        TEST_DOMAIN,
        contract,
        semantics,
        required,
        optional,
    )
    .expect("stress-test descriptor must be valid")
}

fn profile(
    contract: CompatibilityVersion,
    semantics: CompatibilityVersion,
    supported: impl IntoIterator<Item = String>,
) -> CompatibilityProfile {
    CompatibilityProfile::new(
        TEST_DOMAIN,
        contract,
        semantics,
        supported,
    )
    .expect("stress-test profile must be valid")
}

fn feature_name(index: usize) -> String {
    format!("classical.stress.feature_{index}")
}

fn all_features(count: usize) -> Vec<String> {
    (0..count).map(feature_name).collect()
}

fn compatible_pair(
    features: Vec<String>,
) -> (CompatibilityDescriptor, CompatibilityProfile) {
    let artifact = descriptor(
        version(1, 4, 0),
        version(2, 3, 0),
        features.clone(),
        std::iter::empty(),
    );

    let consumer = profile(
        version(1, 4, 9),
        version(2, 3, 7),
        features,
    );

    (artifact, consumer)
}

#[test]
fn empty_feature_sets_are_compatible() {
    let (artifact, consumer) = compatible_pair(Vec::new());

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Compatible);
    assert!(report.is_compatible());
    assert!(!report.has_blocking_issues());
    assert!(report.issues().is_empty());
}

#[test]
fn thousands_of_supported_features_remain_compatible() {
    let features = all_features(4_096);
    let (artifact, consumer) = compatible_pair(features.clone());

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(artifact.required_features().len(), features.len());
    assert_eq!(consumer.supported_features().len(), features.len());
    assert_eq!(report.status(), CompatibilityStatus::Compatible);
    assert!(report.issues().is_empty());
}

#[test]
fn a_missing_required_feature_is_not_silently_accepted() {
    let mut required = all_features(1_024);
    let missing = "classical.stress.required_missing".to_owned();
    required.push(missing.clone());

    let supported = all_features(1_024);
    let artifact = descriptor(
        version(1, 0, 0),
        version(1, 0, 0),
        required,
        std::iter::empty(),
    );
    let consumer = profile(
        version(1, 0, 0),
        version(1, 0, 0),
        supported,
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(!report.is_compatible());
    assert!(report.has_blocking_issues());

    assert!(report.issues().iter().any(|issue| {
        issue.code() == CompatibilityIssueCode::MissingRequiredFeature
            && issue.subject() == missing
    }));
}

#[test]
fn many_missing_required_features_are_all_reported() {
    let count = 2_048;
    let required = all_features(count);

    let artifact = descriptor(
        version(1, 0, 0),
        version(1, 0, 0),
        required,
        std::iter::empty(),
    );
    let consumer = profile(
        version(1, 0, 0),
        version(1, 0, 0),
        std::iter::empty(),
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert_eq!(report.issues().len(), count);
    assert!(report.issues().iter().all(|issue| {
        issue.code() == CompatibilityIssueCode::MissingRequiredFeature
    }));
}

#[test]
fn many_missing_optional_features_produce_warnings_not_blocking_errors() {
    let count = 2_048;
    let optional = all_features(count);

    let artifact = descriptor(
        version(1, 0, 0),
        version(1, 0, 0),
        std::iter::empty(),
        optional,
    );
    let consumer = profile(
        version(1, 0, 0),
        version(1, 0, 0),
        std::iter::empty(),
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(
        report.status(),
        CompatibilityStatus::CompatibleWithWarnings
    );
    assert!(report.is_compatible());
    assert!(!report.has_blocking_issues());
    assert_eq!(report.issues().len(), count);
    assert!(report.issues().iter().all(|issue| {
        issue.code() == CompatibilityIssueCode::MissingOptionalFeature
    }));
}

#[test]
fn required_and_optional_feature_sets_scale_independently() {
    let required_count = 1_024;
    let optional_count = 1_024;

    let required = (0..required_count)
        .map(|index| format!("classical.required.feature_{index}"));
    let optional = (0..optional_count)
        .map(|index| format!("classical.optional.feature_{index}"));

    let artifact = descriptor(
        version(1, 0, 0),
        version(1, 0, 0),
        required,
        optional,
    );

    let consumer = profile(
        version(1, 0, 0),
        version(1, 0, 0),
        (0..required_count)
            .map(|index| format!("classical.required.feature_{index}")),
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::CompatibleWithWarnings);
    assert_eq!(report.issues().len(), optional_count);
    assert!(report.issues().iter().all(|issue| {
        issue.code() == CompatibilityIssueCode::MissingOptionalFeature
    }));
}

#[test]
fn duplicate_input_features_do_not_duplicate_semantic_requirements() {
    let unique_count = 256;
    let mut repeated = Vec::new();

    for _ in 0..8 {
        repeated.extend(all_features(unique_count));
    }

    let artifact = descriptor(
        version(1, 0, 0),
        version(1, 0, 0),
        repeated,
        std::iter::empty(),
    );

    let consumer = profile(
        version(1, 0, 0),
        version(1, 0, 0),
        all_features(unique_count),
    );

    assert_eq!(artifact.required_features().len(), unique_count);
    assert_eq!(
        assess_compatibility(&artifact, &consumer).status(),
        CompatibilityStatus::Compatible
    );
}

#[test]
fn feature_input_order_does_not_change_the_report() {
    let count = 1_500;
    let ascending = all_features(count);
    let descending: Vec<String> = ascending.iter().rev().cloned().collect();

    let first_artifact = descriptor(
        version(1, 2, 0),
        version(3, 4, 0),
        ascending.clone(),
        std::iter::empty(),
    );
    let second_artifact = descriptor(
        version(1, 2, 0),
        version(3, 4, 0),
        descending,
        std::iter::empty(),
    );

    let supported: Vec<String> = ascending
        .iter()
        .filter(|feature| !feature.ends_with("_1499"))
        .cloned()
        .collect();

    let consumer = profile(
        version(1, 2, 1),
        version(3, 4, 1),
        supported,
    );

    let first_report = assess_compatibility(&first_artifact, &consumer);
    let second_report = assess_compatibility(&second_artifact, &consumer);

    assert_eq!(first_report, second_report);
    assert_eq!(first_report.status(), CompatibilityStatus::Incompatible);
    assert_eq!(first_report.issues().len(), 1_500 - 1_499 + 1);
}

#[test]
fn repeated_assessment_is_deterministic_for_large_inputs() {
    let features = all_features(3_000);
    let (artifact, consumer) = compatible_pair(features);

    let expected = assess_compatibility(&artifact, &consumer);

    for _ in 0..8 {
        assert_eq!(assess_compatibility(&artifact, &consumer), expected);
    }
}

#[test]
fn very_long_valid_feature_identifiers_are_not_truncated() {
    let long_component = "x".repeat(16_384);
    let long_feature = format!("classical.extension.{long_component}");

    let artifact = descriptor(
        version(1, 0, 0),
        version(1, 0, 0),
        [long_feature.clone()],
        std::iter::empty(),
    );
    let consumer = profile(
        version(1, 0, 0),
        version(1, 0, 0),
        [long_feature.clone()],
    );

    assert!(artifact.required_features().contains(&long_feature));
    assert!(consumer.supported_features().contains(&long_feature));

    let report = assess_compatibility(&artifact, &consumer);
    assert_eq!(report.status(), CompatibilityStatus::Compatible);
    assert!(report.issues().is_empty());
}

#[test]
fn version_comparison_handles_numeric_boundaries_without_overflow() {
    let minimum = version(0, 0, 0);
    let maximum = version(u64::MAX, u64::MAX, u64::MAX);
    let same_major_newer_minor = version(u64::MAX, u64::MAX, 0);
    let different_major = version(u64::MAX - 1, u64::MAX, u64::MAX);

    assert!(minimum.is_supported_by(maximum));
    assert!(same_major_newer_minor.is_supported_by(maximum));
    assert!(!maximum.is_supported_by(different_major));
    assert_eq!(maximum.compare(maximum), core::cmp::Ordering::Equal);
}

#[test]
fn patch_version_differences_do_not_create_false_incompatibility() {
    let artifact = descriptor(
        version(7, 12, u64::MAX),
        version(9, 5, u64::MAX),
        std::iter::empty(),
        std::iter::empty(),
    );

    let consumer = profile(
        version(7, 12, 0),
        version(9, 5, 0),
        std::iter::empty(),
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Compatible);
    assert!(report.issues().is_empty());
}

#[test]
fn newer_minor_versions_are_reported_instead_of_wrapping() {
    let artifact = descriptor(
        version(1, u64::MAX, 0),
        version(2, u64::MAX, 0),
        std::iter::empty(),
        std::iter::empty(),
    );

    let consumer = profile(
        version(1, 0, u64::MAX),
        version(2, 0, u64::MAX),
        std::iter::empty(),
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(report.issues().iter().any(|issue| {
        issue.code() == CompatibilityIssueCode::ContractVersionTooNew
    }));
    assert!(report.issues().iter().any(|issue| {
        issue.code() == CompatibilityIssueCode::SemanticVersionTooNew
    }));
}

#[test]
fn issue_codes_remain_machine_readable_under_stress() {
    let artifact = descriptor(
        version(1, 0, 0),
        version(1, 0, 0),
        ["classical.required.absent".to_owned()],
        std::iter::empty(),
    );
    let consumer = profile(
        version(1, 0, 0),
        version(1, 0, 0),
        std::iter::empty(),
    );

    let report = assess_compatibility(&artifact, &consumer);
    let issue = report
        .issues()
        .first()
        .expect("one required feature should be missing");

    assert_eq!(
        issue.code().as_str(),
        "classical.compatibility.missing_required_feature"
    );
    assert_eq!(
        issue.code().to_string(),
        issue.code().as_str()
    );
    assert_eq!(issue.subject(), "classical.required.absent");
    assert!(!issue.message().is_empty());
}

#[test]
#[ignore = "extended allocation-heavy stress test; run explicitly with a suitable resource budget"]
fn extended_feature_scale_is_operator_configurable() {
    // Example:
    // ZAMANI_STRESS_FEATURE_COUNT=500000 cargo test extended_feature_scale_is_operator_configurable -- --ignored
    //
    // This value is a workload size, not a language or production limit.
    // Choose it according to the machine's available memory and test budget.
    let count = std::env::var("ZAMANI_STRESS_FEATURE_COUNT")
        .ok()
        .map(|value| {
            value
                .parse::<usize>()
                .expect("ZAMANI_STRESS_FEATURE_COUNT must be a valid usize")
        })
        .unwrap_or(50_000);

    let features = all_features(count);
    let (artifact, consumer) = compatible_pair(features.clone());

    assert_eq!(artifact.required_features().len(), count);
    assert_eq!(consumer.supported_features().len(), count);

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Compatible);
    assert!(report.issues().is_empty());
}
