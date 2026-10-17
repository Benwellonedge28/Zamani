
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical compatibility conformance tests.
//!
//! Path: src/classical/tests/compatibility.rs
//! Edition: Rust 2021
//! Minimum intended Rust version: Rust 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Verify the existing compatibility API's versioning, extensible feature
//! identifiers, deterministic reports, conservative rejection behavior,
//! and separation of required and optional features.
//!
//! AUTHORITY
//!
//! - grammar/specification/compatibility.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/validation/semantic-boundaries.md
//! - src/classical/compatibility.rs
//!
//! OWNERSHIP
//!
//! `src/classical/compatibility.rs` owns compatibility behavior.
//! This file owns only conformance tests.
//!
//! The tests do not define language-version policy, duplicate canonical
//! IR types, perform migrations, or assume that a compatible artifact is
//! necessarily executable on a particular target.
//!
//! INTEGRATION
//!
//! Register this module in src/classical/tests/mod.rs and register the
//! tests module from src/classical/mod.rs according to the project's
//! chosen test configuration.
//!
//! These tests use the public compatibility API. They must not depend on
//! private fields or implementation-specific data structures.
//!
//! SCALABILITY
//!
//! No fixed hardware, collection, feature-count, or target-count ceiling
//! is introduced. Test workloads are examples, not language limits.
//!
//! COMPLETION
//!
//! This file is complete only after it compiles against the actual
//! compatibility implementation and all assertions pass.

use crate::classical::compatibility::{
    assess_compatibility,
    CompatibilityDescriptor,
    CompatibilityDescriptorError,
    CompatibilityIssueCode,
    CompatibilityProfile,
    CompatibilityStatus,
    CompatibilityVersion,
};

fn version(major: u64, minor: u64, patch: u64) -> CompatibilityVersion {
    CompatibilityVersion::new(major, minor, patch)
}

fn descriptor(
    domain: &str,
    contract: CompatibilityVersion,
    semantics: CompatibilityVersion,
    required: &[&str],
    optional: &[&str],
) -> CompatibilityDescriptor {
    CompatibilityDescriptor::new(
        domain,
        contract,
        semantics,
        required.iter().map(|feature| (*feature).to_owned()),
        optional.iter().map(|feature| (*feature).to_owned()),
    )
    .expect("test descriptor must be valid")
}

fn profile(
    domain: &str,
    contract: CompatibilityVersion,
    semantics: CompatibilityVersion,
    supported: &[&str],
) -> CompatibilityProfile {
    CompatibilityProfile::new(
        domain,
        contract,
        semantics,
        supported.iter().map(|feature| (*feature).to_owned()),
    )
    .expect("test profile must be valid")
}

#[test]
fn equal_contracts_and_supported_features_are_compatible() {
    let artifact = descriptor(
        "classical.ir",
        version(1, 2, 0),
        version(3, 4, 5),
        &["classical.numeric.checked-arithmetic"],
        &[],
    );

    let consumer = profile(
        "classical.ir",
        version(1, 2, 9),
        version(3, 4, 0),
        &["classical.numeric.checked-arithmetic"],
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Compatible);
    assert!(report.is_compatible());
    assert!(!report.has_blocking_issues());
    assert!(report.issues().is_empty());
}

#[test]
fn older_minor_versions_are_supported_by_newer_minor_versions() {
    let artifact = descriptor(
        "classical.ir",
        version(2, 3, 9),
        version(4, 1, 7),
        &[],
        &[],
    );

    let consumer = profile(
        "classical.ir",
        version(2, 5, 0),
        version(4, 2, 0),
        &[],
    );

    assert!(assess_compatibility(&artifact, &consumer).is_compatible());
}

#[test]
fn patch_versions_do_not_change_compatibility_under_the_declared_policy() {
    let artifact = descriptor(
        "classical.ir",
        version(1, 3, u64::MAX),
        version(2, 4, u64::MAX),
        &[],
        &[],
    );

    let consumer = profile(
        "classical.ir",
        version(1, 3, 0),
        version(2, 4, 0),
        &[],
    );

    assert!(assess_compatibility(&artifact, &consumer).is_compatible());
}

#[test]
fn contract_major_mismatch_is_blocking() {
    let artifact = descriptor(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
        &[],
    );

    let consumer = profile(
        "classical.ir",
        version(2, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(report.has_blocking_issues());
    assert!(report.issues().iter().any(|issue| {
        issue.code() == CompatibilityIssueCode::ContractMajorMismatch
    }));
}

#[test]
fn newer_contract_minor_version_is_rejected() {
    let artifact = descriptor(
        "classical.ir",
        version(1, 8, 0),
        version(1, 0, 0),
        &[],
        &[],
    );

    let consumer = profile(
        "classical.ir",
        version(1, 7, u64::MAX),
        version(1, 0, 0),
        &[],
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert!(!report.is_compatible());
    assert!(report.issues().iter().any(|issue| {
        issue.code() == CompatibilityIssueCode::ContractVersionTooNew
    }));
}

#[test]
fn semantic_major_mismatch_is_blocking() {
    let artifact = descriptor(
        "classical.ir",
        version(1, 0, 0),
        version(2, 0, 0),
        &[],
        &[],
    );

    let consumer = profile(
        "classical.ir",
        version(1, 0, 0),
        version(3, 0, 0),
        &[],
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(report.issues().iter().any(|issue| {
        issue.code() == CompatibilityIssueCode::SemanticMajorMismatch
    }));
}

#[test]
fn newer_semantic_minor_version_is_rejected() {
    let artifact = descriptor(
        "classical.ir",
        version(1, 0, 0),
        version(1, 9, 0),
        &[],
        &[],
    );

    let consumer = profile(
        "classical.ir",
        version(1, 0, 0),
        version(1, 8, 0),
        &[],
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert!(!report.is_compatible());
    assert!(report.issues().iter().any(|issue| {
        issue.code() == CompatibilityIssueCode::SemanticVersionTooNew
    }));
}

#[test]
fn different_domains_are_incompatible() {
    let artifact = descriptor(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
        &[],
    );

    let consumer = profile(
        "classical.runtime",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(report.issues().iter().any(|issue| {
        issue.code() == CompatibilityIssueCode::DomainMismatch
    }));
}

#[test]
fn missing_required_feature_is_never_silently_accepted() {
    let artifact = descriptor(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &["classical.numeric.arbitrary_precision"],
        &[],
    );

    let consumer = profile(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(!report.is_compatible());
    assert!(report.has_blocking_issues());

    let issue = report
        .issues()
        .iter()
        .find(|issue| {
            issue.code() == CompatibilityIssueCode::MissingRequiredFeature
        })
        .expect("missing required feature must be reported");

    assert_eq!(
        issue.code().as_str(),
        "classical.compatibility.missing_required_feature"
    );
    assert_eq!(
        issue.subject(),
        "classical.numeric.arbitrary_precision"
    );
}

#[test]
fn missing_optional_feature_produces_a_warning() {
    let artifact = descriptor(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
        &["classical.numeric.fast-math"],
    );

    let consumer = profile(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(
        report.status(),
        CompatibilityStatus::CompatibleWithWarnings
    );
    assert!(report.is_compatible());
    assert!(!report.has_blocking_issues());
    assert_eq!(report.issues().len(), 1);
    assert_eq!(
        report.issues()[0].code(),
        CompatibilityIssueCode::MissingOptionalFeature
    );
}

#[test]
fn missing_required_and_optional_features_have_distinct_outcomes() {
    let artifact = descriptor(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &["classical.memory.checked-allocation"],
        &["classical.numeric.vectorized"],
    );

    let consumer = profile(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
    assert!(report.has_blocking_issues());

    assert!(report.issues().iter().any(|issue| {
        issue.code() == CompatibilityIssueCode::MissingRequiredFeature
    }));

    assert!(report.issues().iter().any(|issue| {
        issue.code() == CompatibilityIssueCode::MissingOptionalFeature
    }));
}

#[test]
fn required_optional_feature_overlap_is_rejected() {
    let result = CompatibilityDescriptor::new(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        vec!["classical.numeric.checked".to_owned()],
        vec!["classical.numeric.checked".to_owned()],
    );

    assert!(matches!(
        result,
        Err(
            CompatibilityDescriptorError::FeatureDeclaredAsRequiredAndOptional(
                feature
            )
        ) if feature == "classical.numeric.checked"
    ));
}

#[test]
fn invalid_domains_are_rejected() {
    let invalid_domains = ["", ".classical", "classical.", "classical..ir"];

    for domain in invalid_domains {
        let result = CompatibilityProfile::new(
            domain,
            version(1, 0, 0),
            version(1, 0, 0),
            Vec::<String>::new(),
        );

        assert!(
            matches!(result, Err(CompatibilityDescriptorError::InvalidDomain(_))),
            "domain {domain:?} should be rejected"
        );
    }
}

#[test]
fn invalid_feature_identifiers_are_rejected() {
    let invalid_features = [
        "",
        ".feature",
        "feature.",
        "classical..numeric",
        "classical.numeric feature",
        "classical.numeric/feature",
    ];

    for feature in invalid_features {
        let result = CompatibilityProfile::new(
            "classical.ir",
            version(1, 0, 0),
            version(1, 0, 0),
            vec![feature.to_owned()],
        );

        assert!(
            matches!(
                result,
                Err(CompatibilityDescriptorError::InvalidFeatureIdentifier(_))
            ),
            "feature identifier {feature:?} should be rejected"
        );
    }
}

#[test]
fn namespaced_and_future_feature_identifiers_are_extensible() {
    let features = [
        "classical.numeric.arbitrary_precision",
        "classical.parallel.data-parallel",
        "vendor_extension.feature_v2",
        "future_domain.capability",
    ];

    let artifact = descriptor(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &features,
        &[],
    );

    let consumer = profile(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &features,
    );

    assert!(assess_compatibility(&artifact, &consumer).is_compatible());
}

#[test]
fn duplicate_feature_declarations_are_set_like_and_deterministic() {
    let artifact = CompatibilityDescriptor::new(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        vec![
            "classical.numeric.checked".to_owned(),
            "classical.numeric.checked".to_owned(),
        ],
        Vec::<String>::new(),
    )
    .expect("repeated required feature declarations should deduplicate");

    assert_eq!(artifact.required_features().len(), 1);
}

#[test]
fn feature_input_order_does_not_change_the_compatibility_report() {
    let artifact_a = descriptor(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[
            "classical.zeta",
            "classical.alpha",
            "classical.beta",
        ],
        &["classical.optional.zeta", "classical.optional.alpha"],
    );

    let artifact_b = descriptor(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[
            "classical.beta",
            "classical.zeta",
            "classical.alpha",
        ],
        &["classical.optional.alpha", "classical.optional.zeta"],
    );

    let consumer = profile(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &[],
    );

    let report_a = assess_compatibility(&artifact_a, &consumer);
    let report_b = assess_compatibility(&artifact_b, &consumer);

    assert_eq!(report_a, report_b);
}

#[test]
fn issue_codes_are_stable_machine_readable_identifiers() {
    let cases = [
        (
            CompatibilityIssueCode::DomainMismatch,
            "classical.compatibility.domain_mismatch",
        ),
        (
            CompatibilityIssueCode::ContractMajorMismatch,
            "classical.compatibility.contract_major_mismatch",
        ),
        (
            CompatibilityIssueCode::ContractVersionTooNew,
            "classical.compatibility.contract_version_too_new",
        ),
        (
            CompatibilityIssueCode::SemanticMajorMismatch,
            "classical.compatibility.semantic_major_mismatch",
        ),
        (
            CompatibilityIssueCode::SemanticVersionTooNew,
            "classical.compatibility.semantic_version_too_new",
        ),
        (
            CompatibilityIssueCode::MissingRequiredFeature,
            "classical.compatibility.missing_required_feature",
        ),
        (
            CompatibilityIssueCode::MissingOptionalFeature,
            "classical.compatibility.missing_optional_feature",
        ),
    ];

    for (code, expected) in cases {
        assert_eq!(code.as_str(), expected);
        assert!(!code.as_str().is_empty());
    }
}

#[test]
fn compatibility_version_ordering_is_numeric_and_transitive() {
    let versions = [
        version(0, 0, 0),
        version(0, 0, 1),
        version(0, 1, 0),
        version(1, 0, 0),
        version(u64::MAX, u64::MAX, u64::MAX),
    ];

    for pair in versions.windows(2) {
        assert!(pair[0] < pair[1]);
    }

    assert_eq!(
        version(u64::MAX, u64::MAX, u64::MAX).to_string(),
        format!("{}.{}.{}", u64::MAX, u64::MAX, u64::MAX)
    );
}

#[test]
fn version_support_policy_does_not_overflow_at_numeric_boundaries() {
    assert!(
        version(u64::MAX, u64::MAX, u64::MAX)
            .is_supported_by(version(u64::MAX, u64::MAX, 0))
    );

    assert!(
        !version(u64::MAX, u64::MAX, 0)
            .is_supported_by(version(u64::MAX - 1, u64::MAX, u64::MAX))
    );

    assert!(
        !version(1, u64::MAX, 0)
            .is_supported_by(version(1, u64::MAX - 1, u64::MAX))
    );
}

#[test]
fn compatibility_assessment_is_repeatable() {
    let artifact = descriptor(
        "classical.ir",
        version(1, 2, 0),
        version(2, 1, 0),
        &["classical.numeric.required"],
        &["classical.numeric.optional"],
    );

    let consumer = profile(
        "classical.ir",
        version(1, 2, 0),
        version(2, 1, 0),
        &[],
    );

    let first = assess_compatibility(&artifact, &consumer);
    let second = assess_compatibility(&artifact, &consumer);
    let third = assess_compatibility(&artifact, &consumer);

    assert_eq!(first, second);
    assert_eq!(second, third);
}

#[test]
fn compatibility_does_not_claim_target_resource_feasibility() {
    // A compatibility profile declares a contract and supported features.
    // It is not a report about physical memory, processor counts, or
    // whether a particular execution target can satisfy program demands.
    let artifact = descriptor(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &["classical.numeric.checked-arithmetic"],
        &[],
    );

    let consumer = profile(
        "classical.ir",
        version(1, 0, 0),
        version(1, 0, 0),
        &["classical.numeric.checked-arithmetic"],
    );

    let report = assess_compatibility(&artifact, &consumer);

    assert!(report.is_compatible());
    assert!(report.issues().is_empty());
}
