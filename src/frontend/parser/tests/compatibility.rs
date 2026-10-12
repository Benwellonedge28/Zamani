#![forbid(unsafe_code)]

//! Integration tests for Zamani parser compatibility policy.
//!
//! File: src/frontend/parser/tests/compatibility.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Verify the compatibility engine's public contract for language versions,
//! feature lifecycle metadata, version boundaries, deprecated syntax,
//! unknown features, source ranges, deterministic reporting, and extensibility.
//!
//! # Ownership
//!
//! `parser/compatibility.rs` owns compatibility rules and policy evaluation.
//! This test module owns no compatibility implementation or feature catalogue.
//! The normative contracts are defined by:
//!
//! - `grammar/spec/compatibility.md`
//! - `grammar/specification/compatibility.md`
//! - `grammar/specification/language-version.md`
//! - `grammar/spec/syntax.md`
//!
//! # Integration
//!
//! Include this file from `src/frontend/parser/tests/mod.rs` and expose that
//! test module from the parser module using the repository's chosen test
//! wiring. The imports below assume `parser::compatibility` is reachable from
//! the crate's frontend module.
//!
//! These tests exercise the compatibility policy API. They do not claim that
//! every grammar feature has already been implemented or that a complete
//! source parser is available.
//!
//! # Scalability
//!
//! Tests use dynamically registered feature IDs and generated test cases.
//! No hardware-capacity ceilings or fixed language-wide feature catalogue are
//! introduced. Test sizes are finite by necessity; they are not language limits.

use std::str::FromStr;

use crate::frontend::parser::compatibility::{
    CompatibilityCode, CompatibilityPolicy, CompatibilityRegistry,
    CompatibilityRegistryError, CompatibilitySeverity, DeprecationPolicy,
    FeatureDisposition, FeatureId, FeatureIdError, FeatureRule, FeatureStatus,
    FeatureUse, LanguageVersion, UnknownFeaturePolicy, VersionParseError,
};

fn version(value: &str) -> LanguageVersion {
    LanguageVersion::from_str(value).expect("test version must be valid")
}

fn feature(value: &str) -> FeatureId {
    FeatureId::new(value).expect("test feature ID must be valid")
}

fn stable_rule(value: &str) -> FeatureRule {
    FeatureRule::new(feature(value), FeatureStatus::Stable)
}

fn conservative(value: &str) -> CompatibilityPolicy {
    CompatibilityPolicy::conservative(version(value))
}

fn register(
    registry: &mut CompatibilityRegistry,
    rule: FeatureRule,
) {
    registry
        .register(rule)
        .expect("test compatibility rule must be valid and unique");
}

#[test]
fn language_versions_parse_display_and_compare_numerically() {
    let v_1_9 = version("1.9.0");
    let v_1_10 = version("1.10.0");
    let v_2_0 = version("2.0.0");

    assert!(v_1_10 > v_1_9);
    assert!(v_2_0 > v_1_10);
    assert_eq!(v_1_10.to_string(), "1.10.0");
    assert_eq!(version("0.0.0").to_string(), "0.0.0");
}

#[test]
fn language_version_components_are_available_and_ordered() {
    let value = LanguageVersion::new(12, 34, 56);

    assert_eq!(value.major(), 12);
    assert_eq!(value.minor(), 34);
    assert_eq!(value.patch(), 56);
    assert_eq!(value, version("12.34.56"));
}

#[test]
fn malformed_language_versions_are_rejected() {
    let invalid = [
        "",
        "1",
        "1.2",
        "1.2.3.4",
        " 1.2.3",
        "1.2.3 ",
        "1..3",
        ".1.2",
        "1.2.",
        "1.-2.3",
        "+1.2.3",
        "01.2.3",
        "1.02.3",
        "1.2.03",
        "1.2.3-alpha",
        "1.2.3+build",
        "18446744073709551616.0.0",
    ];

    for input in invalid {
        assert!(
            input.parse::<LanguageVersion>().is_err(),
            "invalid version {input:?} must be rejected"
        );
    }
}

#[test]
fn version_parser_returns_structured_errors() {
    assert!(matches!(
        "".parse::<LanguageVersion>(),
        Err(VersionParseError::InvalidFormat(_))
    ));

    assert!(matches!(
        "1.2".parse::<LanguageVersion>(),
        Err(VersionParseError::InvalidFormat(_))
    ));

    assert!(matches!(
        "1.2.3.4".parse::<LanguageVersion>(),
        Err(VersionParseError::InvalidFormat(_))
    ));

    assert!(matches!(
        "1.02.3".parse::<LanguageVersion>(),
        Err(VersionParseError::LeadingZero(_))
    ));

    assert!(matches!(
        "1.x.3".parse::<LanguageVersion>(),
        Err(VersionParseError::InvalidComponent(_))
    ));
}

#[test]
fn feature_ids_preserve_namespaced_extension_identifiers() {
    let identifiers = [
        "core.expressions",
        "quantum.operation_syntax",
        "hdl.concurrent_process",
        "vendor.example.operation",
        "research.experimental_feature",
        "future.domain.construct",
    ];

    for value in identifiers {
        let id = feature(value);
        assert_eq!(id.as_str(), value);
        assert_eq!(id.to_string(), value);
    }
}

#[test]
fn malformed_feature_ids_are_rejected_without_normalization() {
    assert!(matches!(FeatureId::new(""), Err(FeatureIdError::Empty)));
    assert!(matches!(FeatureId::new("   "), Err(FeatureIdError::Empty)));

    assert!(matches!(
        FeatureId::new(" core.feature"),
        Err(FeatureIdError::SurroundingWhitespace)
    ));

    assert!(matches!(
        FeatureId::new("core.feature "),
        Err(FeatureIdError::SurroundingWhitespace)
    ));

    assert!(matches!(
        FeatureId::new("core\nfeature"),
        Err(FeatureIdError::ControlCharacter)
    ));
}

#[test]
fn registry_starts_empty_and_reports_registered_rules() {
    let mut registry = CompatibilityRegistry::new();

    assert!(registry.is_empty());
    assert_eq!(registry.len(), 0);
    assert!(registry.get(&feature("core.expressions")).is_none());

    register(&mut registry, stable_rule("core.expressions"));

    assert!(!registry.is_empty());
    assert_eq!(registry.len(), 1);
    assert_eq!(
        registry
            .get(&feature("core.expressions"))
            .expect("registered rule must be found")
            .status(),
        FeatureStatus::Stable
    );
}

#[test]
fn duplicate_feature_registration_is_rejected_atomically() {
    let mut registry = CompatibilityRegistry::new();

    register(&mut registry, stable_rule("core.expressions"));

    let duplicate = registry.register(FeatureRule::new(
        feature("core.expressions"),
        FeatureStatus::Experimental,
    ));

    assert!(matches!(
        duplicate,
        Err(CompatibilityRegistryError::DuplicateFeature(_))
    ));

    assert_eq!(registry.len(), 1);
    assert_eq!(
        registry
            .get(&feature("core.expressions"))
            .expect("original registration must remain")
            .status(),
        FeatureStatus::Stable
    );
}

#[test]
fn feature_rules_preserve_their_declared_metadata() {
    let rule = FeatureRule::new(
        feature("vendor.operation"),
        FeatureStatus::Experimental,
    )
    .introduced_in(version("1.2.0"))
    .deprecated_in(version("1.8.0"))
    .removed_in(version("2.0.0"))
    .replacement("vendor.replacement")
    .description("An example extension feature.");

    assert_eq!(rule.id().as_str(), "vendor.operation");
    assert_eq!(rule.status(), FeatureStatus::Experimental);
    assert_eq!(rule.introduced_version(), Some(version("1.2.0")));
    assert_eq!(rule.deprecated_version(), Some(version("1.8.0")));
    assert_eq!(rule.removed_version(), Some(version("2.0.0")));
    assert_eq!(rule.replacement_id(), Some("vendor.replacement"));
    assert_eq!(
        rule.description_text(),
        Some("An example extension feature.")
    );
}

#[test]
fn introduction_version_is_inclusive() {
    let mut registry = CompatibilityRegistry::new();

    register(
        &mut registry,
        stable_rule("core.new_construct").introduced_in(version("1.2.0")),
    );

    let before = registry.check(
        &FeatureUse::new(feature("core.new_construct")),
        &conservative("1.1.99"),
    );
    let at = registry.check(
        &FeatureUse::new(feature("core.new_construct")),
        &conservative("1.2.0"),
    );
    let after = registry.check(
        &FeatureUse::new(feature("core.new_construct")),
        &conservative("1.2.1"),
    );

    assert!(before.has_errors());
    assert_eq!(before.issues()[0].code(), CompatibilityCode::FeatureNotIntroduced);
    assert!(at.is_compatible());
    assert!(after.is_compatible());
}

#[test]
fn removal_version_is_exclusive_for_availability() {
    let mut registry = CompatibilityRegistry::new();

    register(
        &mut registry,
        stable_rule("core.legacy_construct")
            .introduced_in(version("1.0.0"))
            .removed_in(version("2.0.0")),
    );

    let before = registry.check(
        &FeatureUse::new(feature("core.legacy_construct")),
        &conservative("1.99.99"),
    );
    let at = registry.check(
        &FeatureUse::new(feature("core.legacy_construct")),
        &conservative("2.0.0"),
    );
    let after = registry.check(
        &FeatureUse::new(feature("core.legacy_construct")),
        &conservative("2.0.1"),
    );

    assert!(before.is_compatible());
    assert!(at.has_errors());
    assert_eq!(at.issues()[0].code(), CompatibilityCode::FeatureRemoved);
    assert!(after.has_errors());
}

#[test]
fn introduction_and_removal_boundaries_are_not_lexicographic() {
    let mut registry = CompatibilityRegistry::new();

    register(
        &mut registry,
        stable_rule("core.version_boundary")
            .introduced_in(version("1.10.0"))
            .removed_in(version("2.0.0")),
    );

    assert!(
        registry
            .check(
                &FeatureUse::new(feature("core.version_boundary")),
                &conservative("1.9.99"),
            )
            .has_errors()
    );

    assert!(
        registry
            .check(
                &FeatureUse::new(feature("core.version_boundary")),
                &conservative("1.10.0"),
            )
            .is_compatible()
    );
}

#[test]
fn unknown_features_are_rejected_by_conservative_policy() {
    let registry = CompatibilityRegistry::new();

    let report = registry.check(
        &FeatureUse::new(feature("extension.not_registered")),
        &conservative("1.0.0"),
    );

    assert!(report.has_errors());
    assert_eq!(report.error_count(), 1);
    assert_eq!(report.issues()[0].code(), CompatibilityCode::UnknownFeature);
    assert_eq!(
        report.issues()[0].severity(),
        CompatibilitySeverity::Error
    );
}

#[test]
fn unknown_feature_policy_can_warn_or_allow_explicitly() {
    let registry = CompatibilityRegistry::new();
    let use_site = FeatureUse::new(feature("extension.external_metadata"));

    let warning = registry.check(
        &use_site,
        &conservative("1.0.0")
            .with_unknown_feature_policy(UnknownFeaturePolicy::Warn),
    );

    assert!(!warning.has_errors());
    assert_eq!(warning.warning_count(), 1);
    assert_eq!(warning.issues()[0].code(), CompatibilityCode::UnknownFeature);

    let allowed = registry.check(
        &use_site,
        &conservative("1.0.0")
            .with_unknown_feature_policy(UnknownFeaturePolicy::Allow),
    );

    assert!(allowed.is_compatible());
    assert!(allowed.issues().is_empty());
}

#[test]
fn deprecation_warning_preserves_source_range_and_replacement() {
    let mut registry = CompatibilityRegistry::new();

    register(
        &mut registry,
        FeatureRule::new(feature("legacy.construct"), FeatureStatus::Deprecated)
            .deprecated_in(version("1.1.0"))
            .replacement("core.replacement"),
    );

    let use_site = FeatureUse::at(feature("legacy.construct"), 12..27);
    let report = registry.check(&use_site, &conservative("1.1.0"));

    assert!(report.is_compatible());
    assert_eq!(report.warning_count(), 1);

    let issue = &report.issues()[0];
    assert_eq!(issue.code(), CompatibilityCode::DeprecatedSyntaxWarning);
    assert_eq!(issue.severity(), CompatibilitySeverity::Warning);
    assert_eq!(issue.feature().as_str(), "legacy.construct");
    assert_eq!(issue.language_version(), version("1.1.0"));
    assert_eq!(issue.source_range(), Some(12..27));
    assert_eq!(issue.replacement(), Some("core.replacement"));
    assert!(issue.message().contains("deprecated"));
}

#[test]
fn deprecation_policy_can_allow_warn_or_reject() {
    let mut registry = CompatibilityRegistry::new();

    register(
        &mut registry,
        FeatureRule::new(feature("legacy.syntax"), FeatureStatus::Deprecated)
            .deprecated_in(version("1.0.0")),
    );

    let use_site = FeatureUse::new(feature("legacy.syntax"));

    let allowed = registry.check(
        &use_site,
        &conservative("1.0.0").with_deprecation_policy(DeprecationPolicy::Allow),
    );
    assert!(allowed.is_compatible());
    assert!(allowed.issues().is_empty());

    let warned = registry.check(
        &use_site,
        &conservative("1.0.0").with_deprecation_policy(DeprecationPolicy::Warn),
    );
    assert!(!warned.has_errors());
    assert_eq!(warned.warning_count(), 1);

    let rejected = registry.check(
        &use_site,
        &conservative("1.0.0").with_deprecation_policy(DeprecationPolicy::Reject),
    );
    assert!(rejected.has_errors());
    assert!(
        rejected
            .issues()
            .iter()
            .any(|issue| issue.code() == CompatibilityCode::DeprecatedSyntaxRejected)
    );
}

#[test]
fn deprecation_does_not_apply_before_its_declared_version() {
    let mut registry = CompatibilityRegistry::new();

    register(
        &mut registry,
        FeatureRule::new(feature("legacy.syntax"), FeatureStatus::Deprecated)
            .deprecated_in(version("1.5.0")),
    );

    let before = registry.check(
        &FeatureUse::new(feature("legacy.syntax")),
        &conservative("1.4.99"),
    );

    assert!(before.is_compatible());
    assert!(before.issues().is_empty());

    let at = registry.check(
        &FeatureUse::new(feature("legacy.syntax")),
        &conservative("1.5.0"),
    );

    assert_eq!(at.warning_count(), 1);
}

#[test]
fn rejected_lifecycle_states_fail_under_conservative_policy() {
    let mut registry = CompatibilityRegistry::new();

    for status in [
        FeatureStatus::Proposed,
        FeatureStatus::Planned,
        FeatureStatus::Historical,
        FeatureStatus::Rejected,
    ] {
        let id = format!("test.lifecycle.{status:?}").to_lowercase();
        register(&mut registry, FeatureRule::new(feature(id), status));
    }

    let uses = [
        FeatureUse::new(feature("test.lifecycle.proposed")),
        FeatureUse::new(feature("test.lifecycle.planned")),
        FeatureUse::new(feature("test.lifecycle.historical")),
        FeatureUse::new(feature("test.lifecycle.rejected")),
    ];

    let report = registry.check_all(uses.iter(), &conservative("1.0.0"));

    assert_eq!(report.error_count(), uses.len());
    assert!(report.has_errors());
}

#[test]
fn lifecycle_disposition_can_be_overridden_by_explicit_policy() {
    let mut registry = CompatibilityRegistry::new();

    register(
        &mut registry,
        FeatureRule::new(feature("extension.preview"), FeatureStatus::Experimental),
    );

    let policy = conservative("1.0.0").with_status_disposition(
        FeatureStatus::Experimental,
        FeatureDisposition::Allow,
    );

    let report = registry.check(
        &FeatureUse::new(feature("extension.preview")),
        &policy,
    );

    assert!(report.is_compatible());
    assert!(report.issues().is_empty());
}

#[test]
fn invalid_lifecycle_version_order_is_rejected() {
    let mut registry = CompatibilityRegistry::new();

    let invalid = stable_rule("core.invalid_order")
        .introduced_in(version("2.0.0"))
        .deprecated_in(version("1.9.0"));

    assert!(matches!(
        registry.register(invalid),
        Err(CompatibilityRegistryError::InvalidVersionOrder { .. })
    ));

    assert!(registry.is_empty());
}

#[test]
fn empty_replacement_metadata_is_rejected() {
    let mut registry = CompatibilityRegistry::new();

    let invalid = FeatureRule::new(feature("legacy.syntax"), FeatureStatus::Deprecated)
        .replacement(" ");

    assert!(matches!(
        registry.register(invalid),
        Err(CompatibilityRegistryError::EmptyReplacement { .. })
    ));

    assert!(registry.is_empty());
}

#[test]
fn source_ranges_are_half_open_and_preserved_exactly() {
    let mut registry = CompatibilityRegistry::new();

    register(
        &mut registry,
        FeatureRule::new(feature("extension.experimental"), FeatureStatus::Experimental),
    );

    for range in [0..0, 0..1, 7..19, 1_000..1_001] {
        let report = registry.check(
            &FeatureUse::at(feature("extension.experimental"), range.clone()),
            &conservative("1.0.0"),
        );

        assert_eq!(report.issues().len(), 1);
        assert_eq!(report.issues()[0].source_range(), Some(range));
    }
}

#[test]
fn missing_source_ranges_are_supported_for_non_source_checks() {
    let mut registry = CompatibilityRegistry::new();

    register(
        &mut registry,
        FeatureRule::new(feature("extension.experimental"), FeatureStatus::Experimental),
    );

    let report = registry.check(
        &FeatureUse::new(feature("extension.experimental")),
        &conservative("1.0.0"),
    );

    assert_eq!(report.issues().len(), 1);
    assert_eq!(report.issues()[0].source_range(), None);
}

#[test]
fn multiple_feature_findings_are_sorted_deterministically() {
    let mut registry = CompatibilityRegistry::new();

    register(
        &mut registry,
        FeatureRule::new(feature("extension.z"), FeatureStatus::Experimental),
    );
    register(
        &mut registry,
        FeatureRule::new(feature("extension.a"), FeatureStatus::Experimental),
    );

    let uses = [
        FeatureUse::at(feature("extension.z"), 40..45),
        FeatureUse::at(feature("extension.a"), 2..8),
        FeatureUse::at(feature("extension.z"), 15..20),
    ];

    let policy = conservative("1.0.0");
    let first = registry.check_all(uses.iter(), &policy);
    let second = registry.check_all(uses.iter(), &policy);

    assert_eq!(first, second);
    assert_eq!(
        first
            .issues()
            .iter()
            .map(|issue| issue.source_range())
            .collect::<Vec<_>>(),
        vec![Some(2..8), Some(15..20), Some(40..45)]
    );
}

#[test]
fn repeated_feature_uses_remain_separate_findings() {
    let mut registry = CompatibilityRegistry::new();

    register(
        &mut registry,
        FeatureRule::new(feature("extension.preview"), FeatureStatus::Experimental),
    );

    let uses = [
        FeatureUse::at(feature("extension.preview"), 1..4),
        FeatureUse::at(feature("extension.preview"), 10..13),
        FeatureUse::at(feature("extension.preview"), 20..23),
    ];

    let report = registry.check_all(uses.iter(), &conservative("1.0.0"));

    assert_eq!(report.issues().len(), uses.len());
    assert_eq!(
        report
            .issues()
            .iter()
            .map(|issue| issue.source_range())
            .collect::<Vec<_>>(),
        vec![Some(1..4), Some(10..13), Some(20..23)]
    );
}

#[test]
fn empty_feature_use_list_is_compatible() {
    let registry = CompatibilityRegistry::new();
    let report = registry.check_all(
        std::iter::empty::<&FeatureUse>(),
        &conservative("1.0.0"),
    );

    assert!(report.is_compatible());
    assert!(!report.has_errors());
    assert_eq!(report.error_count(), 0);
    assert_eq!(report.warning_count(), 0);
    assert!(report.issues().is_empty());
}

#[test]
fn arbitrary_registered_extension_ids_do_not_require_core_dispatch_changes() {
    let mut registry = CompatibilityRegistry::new();

    let identifiers = [
        "vendor.alpha.operation",
        "vendor.beta.primitive",
        "research.tensor.feature",
        "future.distributed.construct",
    ];

    for id in identifiers {
        register(
            &mut registry,
            FeatureRule::new(feature(id), FeatureStatus::Stable),
        );
    }

    assert_eq!(registry.len(), identifiers.len());

    let policy = conservative("1.0.0");
    for id in identifiers {
        let report = registry.check(&FeatureUse::new(feature(id)), &policy);
        assert!(
            report.is_compatible(),
            "registered stable feature {id} should be accepted"
        );
    }
}

#[test]
fn registry_and_policy_clones_are_independent_values() {
    let mut original = CompatibilityRegistry::new();
    register(&mut original, stable_rule("core.original"));

    let mut cloned = original.clone();
    register(&mut cloned, stable_rule("core.cloned"));

    assert_eq!(original.len(), 1);
    assert_eq!(cloned.len(), 2);
    assert!(original.get(&feature("core.cloned")).is_none());
    assert!(cloned.get(&feature("core.cloned")).is_some());

    let policy = conservative("1.2.3");
    let modified = policy
        .clone()
        .with_status_disposition(FeatureStatus::Experimental, FeatureDisposition::Allow);

    assert_eq!(policy.language_version(), version("1.2.3"));
    assert_eq!(
        policy.status_disposition(FeatureStatus::Experimental),
        FeatureDisposition::Warn
    );
    assert_eq!(
        modified.status_disposition(FeatureStatus::Experimental),
        FeatureDisposition::Allow
    );
}

#[test]
fn compatibility_findings_are_repeatable_for_identical_inputs() {
    let mut registry = CompatibilityRegistry::new();

    register(
        &mut registry,
        FeatureRule::new(feature("legacy.syntax"), FeatureStatus::Deprecated)
            .deprecated_in(version("1.1.0"))
            .replacement("core.replacement"),
    );

    let uses = [
        FeatureUse::at(feature("legacy.syntax"), 8..14),
        FeatureUse::at(feature("missing.feature"), 30..36),
    ];
    let policy = conservative("1.2.0");

    let first = registry.check_all(uses.iter(), &policy);
    let second = registry.check_all(uses.iter(), &policy);

    assert_eq!(first, second);
    assert_eq!(first.issues().len(), second.issues().len());
    assert_eq!(first.error_count(), second.error_count());
    assert_eq!(first.warning_count(), second.warning_count());
}

#[test]
fn large_dynamic_rule_sets_do_not_encode_a_fixed_feature_catalogue() {
    let mut registry = CompatibilityRegistry::new();
    let count = 2_048usize;

    for index in 0..count {
        let id = format!("generated.extension.feature_{index}");
        register(&mut registry, stable_rule(&id));
    }

    assert_eq!(registry.len(), count);

    let policy = conservative("1.0.0");
    for index in [0, count / 2, count - 1] {
        let id = format!("generated.extension.feature_{index}");
        let report = registry.check(&FeatureUse::new(feature(&id)), &policy);
        assert!(report.is_compatible(), "feature {id} should be accepted");
    }
}

#[test]
fn compatibility_is_independent_of_target_capacity_and_device_identity() {
    let mut registry = CompatibilityRegistry::new();

    // These are ordinary, caller-defined compatibility metadata identifiers.
    // The registry does not interpret them as physical limits or discover
    // hardware. Resource negotiation belongs to downstream compiler layers.
    for id in [
        "program.resource_contract",
        "program.capability_requirement",
        "program.topology_requirement",
        "extension.operation_metadata",
    ] {
        register(&mut registry, stable_rule(id));
    }

    let uses = [
        FeatureUse::new(feature("program.resource_contract")),
        FeatureUse::new(feature("program.capability_requirement")),
        FeatureUse::new(feature("program.topology_requirement")),
        FeatureUse::new(feature("extension.operation_metadata")),
    ];

    let report = registry.check_all(uses.iter(), &conservative("1.0.0"));

    assert!(report.is_compatible());
    assert!(report.issues().is_empty());
}