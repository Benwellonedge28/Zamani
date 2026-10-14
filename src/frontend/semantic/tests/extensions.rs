
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Production contract tests for semantic extensions.
//!
//! Path: `src/frontend/semantic/tests/extensions.rs`
//!
//! PURPOSE
//!
//! Verify extension identity, version ranges, manifest construction,
//! dependency validation and ordering, policy enforcement, conflict
//! detection, contribution ownership, deterministic reporting, scalability,
//! and the separation between grammar and semantic validation.
//!
//! AUTHORITY
//!
//! - `src/frontend/semantic/domains/extensions.rs`
//! - `src/frontend/semantic/config.rs`
//! - `grammar/Zamani.g4`
//! - `grammar/dialects/extension-points.g4`
//! - `grammar/dialects/vendor.g4`
//! - `grammar/effects/custom-effects.g4`
//! - `grammar/specification/extensibility.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! OWNERSHIP
//!
//! This file owns tests and test fixtures only. It does not implement
//! extension registration, parsing, semantic validation, code loading,
//! capability authorization, target selection, or execution.
//!
//! SCALABILITY
//!
//! Test collection sizes are workloads, not language or architecture limits.
//! The extension system must not introduce arbitrary limits on extension
//! counts, dependencies, contributions, namespaces, or computational domains.
//!
//! INTEGRATION
//!
//! Register this file from `src/frontend/semantic/tests/mod.rs` and ensure
//! the semantic module tree exposes `domains::extensions`.
//!
//! FREEZE CRITERIA
//!
//! - Every test passes against the authoritative extension API.
//! - Invalid manifests or dependency graphs cannot be accepted.
//! - Equivalent inputs yield equivalent ordered results.
//! - The grammar and semantic specifications retain separate responsibilities.
//! - The test suite uses no unsafe Rust.

use std::collections::BTreeSet;

use crate::frontend::semantic::config::ExtensionPolicy;
use crate::frontend::semantic::domains::extensions::{
    validate_extensions, ExtensionContribution, ExtensionDependency, ExtensionId,
    ExtensionInputError, ExtensionIssueKind, ExtensionKind, ExtensionManifest,
    ExtensionStability, ExtensionValidationReport, ExtensionVersion, QualifiedName,
    VersionRange,
};

// -----------------------------------------------------------------------------
// Shared fixtures
// -----------------------------------------------------------------------------

fn id(namespace: &str, name: impl Into<String>) -> ExtensionId {
    QualifiedName::new(namespace, name)
        .expect("test extension identity must be valid")
}

fn version(major: u64, minor: u64, patch: u64) -> ExtensionVersion {
    ExtensionVersion::new(major, minor, patch)
}

fn language_version() -> ExtensionVersion {
    version(1, 0, 0)
}

fn manifest(namespace: &str, name: &str) -> ExtensionManifest {
    ExtensionManifest::new(
        id(namespace, name),
        version(1, 0, 0),
        ExtensionKind::Domain,
        ExtensionStability::Stable,
        format!("Test extension {namespace}::{name}"),
    )
    .expect("test manifest must be valid")
}

fn validate(manifests: &[ExtensionManifest]) -> ExtensionValidationReport {
    validate_extensions(
        manifests,
        language_version(),
        ExtensionPolicy::StableOnly,
    )
}

fn has_issue(report: &ExtensionValidationReport, expected: ExtensionIssueKind) -> bool {
    report.issues().iter().any(|issue| issue.kind == expected)
}

// -----------------------------------------------------------------------------
// Identity and manifest construction
// -----------------------------------------------------------------------------

#[test]
fn qualified_identity_preserves_namespace_and_name() {
    let identity = id("vendor.quantum", "operation");

    assert_eq!(identity.namespace(), "vendor.quantum");
    assert_eq!(identity.name(), "operation");
    assert_eq!(identity.qualified_name(), "vendor.quantum::operation");
    assert_eq!(identity.to_string(), "vendor.quantum::operation");
}

#[test]
fn qualified_identity_components_do_not_collapse_into_one_string() {
    let left = id("a::b", "c");
    let right = id("a", "b::c");

    assert_ne!(left, right);
    assert_ne!(left.namespace(), right.namespace());
    assert_ne!(left.name(), right.name());
}

#[test]
fn identity_construction_rejects_empty_components() {
    assert!(matches!(
        QualifiedName::new("", "operation"),
        Err(ExtensionInputError::EmptyIdentityComponent { .. })
    ));

    assert!(matches!(
        QualifiedName::new("vendor", ""),
        Err(ExtensionInputError::EmptyIdentityComponent { .. })
    ));

    assert!(matches!(
        QualifiedName::new("vendor", "   "),
        Err(ExtensionInputError::EmptyIdentityComponent { .. })
    ));
}

#[test]
fn identity_construction_rejects_whitespace_and_control_characters() {
    assert!(matches!(
        QualifiedName::new("vendor namespace", "operation"),
        Err(ExtensionInputError::WhitespaceInIdentity { .. })
    ));

    assert!(matches!(
        QualifiedName::new("vendor", "operation name"),
        Err(ExtensionInputError::WhitespaceInIdentity { .. })
    ));

    assert!(matches!(
        QualifiedName::new("vendor\nnamespace", "operation"),
        Err(ExtensionInputError::ControlCharacterInIdentity { .. })
    ));

    assert!(matches!(
        QualifiedName::new("vendor", "operation\tname"),
        Err(ExtensionInputError::ControlCharacterInIdentity { .. })
    ));
}

#[test]
fn manifest_rejects_empty_description() {
    let result = ExtensionManifest::new(
        id("example", "extension"),
        version(1, 0, 0),
        ExtensionKind::Domain,
        ExtensionStability::Stable,
        "   ",
    );

    assert!(matches!(result, Err(ExtensionInputError::EmptyDescription)));
}

#[test]
fn manifest_builder_preserves_declared_metadata() {
    let dependency_id = id("example", "base");
    let capability = id("quantum", "measurement");
    let contribution_id = id("quantum.operations", "custom_operation");

    let manifest = manifest("example", "consumer")
        .with_dependency(ExtensionDependency::new(
            dependency_id.clone(),
            VersionRange::any(),
            false,
        ))
        .with_contribution(ExtensionContribution::new(
            contribution_id.clone(),
            version(2, 1, 0),
        ))
        .requiring_capability(capability.clone());

    assert_eq!(manifest.id(), &id("example", "consumer"));
    assert_eq!(manifest.version(), version(1, 0, 0));
    assert_eq!(manifest.kind(), ExtensionKind::Domain);
    assert_eq!(manifest.stability(), ExtensionStability::Stable);
    assert!(!manifest.description().trim().is_empty());

    assert_eq!(manifest.dependencies().len(), 1);
    assert_eq!(manifest.dependencies()[0].id(), &dependency_id);
    assert!(!manifest.dependencies()[0].is_optional());

    assert_eq!(manifest.contributions().len(), 1);
    assert_eq!(manifest.contributions()[0].id(), &contribution_id);
    assert_eq!(
        manifest.contributions()[0].contract_version(),
        version(2, 1, 0)
    );

    let capabilities: BTreeSet<_> = manifest.required_capabilities().clone();
    assert!(capabilities.contains(&capability));
    assert_eq!(manifest.source_node(), None);
}

// -----------------------------------------------------------------------------
// Version-range semantics
// -----------------------------------------------------------------------------

#[test]
fn version_range_uses_inclusive_lower_and_exclusive_upper_bounds() {
    let range = VersionRange::new(
        Some(version(1, 2, 0)),
        Some(version(2, 0, 0)),
    )
    .expect("ordered bounds must be valid");

    assert_eq!(range.minimum_inclusive(), Some(version(1, 2, 0)));
    assert_eq!(range.maximum_exclusive(), Some(version(2, 0, 0)));

    assert!(range.contains(version(1, 2, 0)));
    assert!(range.contains(version(1, 9, 9)));
    assert!(!range.contains(version(1, 1, 9)));
    assert!(!range.contains(version(2, 0, 0)));
}

#[test]
fn unbounded_version_range_accepts_representable_versions() {
    let range = VersionRange::any();

    assert!(range.contains(version(0, 0, 0)));
    assert!(range.contains(version(1, 0, 0)));
    assert!(range.contains(version(u64::MAX, u64::MAX, u64::MAX)));
}

#[test]
fn version_range_rejects_equal_or_reversed_bounds() {
    assert!(matches!(
        VersionRange::new(
            Some(version(2, 0, 0)),
            Some(version(2, 0, 0)),
        ),
        Err(ExtensionInputError::EmptyVersionRange { .. })
    ));

    assert!(matches!(
        VersionRange::new(
            Some(version(3, 0, 0)),
            Some(version(2, 0, 0)),
        ),
        Err(ExtensionInputError::EmptyVersionRange { .. })
    ));
}

#[test]
fn one_sided_version_ranges_are_supported() {
    let lower_bounded =
        VersionRange::new(Some(version(2, 0, 0)), None).expect("valid range");
    let upper_bounded =
        VersionRange::new(None, Some(version(2, 0, 0))).expect("valid range");

    assert!(lower_bounded.contains(version(u64::MAX, 0, 0)));
    assert!(!lower_bounded.contains(version(1, u64::MAX, u64::MAX)));

    assert!(upper_bounded.contains(version(1, u64::MAX, u64::MAX)));
    assert!(!upper_bounded.contains(version(2, 0, 0)));
}

// -----------------------------------------------------------------------------
// Successful validation and dependency ordering
// -----------------------------------------------------------------------------

#[test]
fn empty_manifest_set_is_valid() {
    let report = validate(&[]);

    assert!(report.is_valid());
    assert!(!report.has_issues());
    assert!(report.issues().is_empty());
    assert!(report.dependency_order().is_empty());
}

#[test]
fn independent_extensions_are_accepted() {
    let report = validate(&[
        manifest("classical", "types"),
        manifest("quantum", "operations"),
        manifest("hdl", "signals"),
        manifest("future.domain", "feature"),
    ]);

    assert!(report.is_valid(), "issues: {:?}", report.issues());
    assert!(!report.has_issues());
    assert_eq!(report.dependency_order().len(), 4);
}

#[test]
fn dependencies_are_ordered_before_their_consumers() {
    let consumer = manifest("example", "consumer").with_dependency(
        ExtensionDependency::new(
            id("example", "base"),
            VersionRange::any(),
            false,
        ),
    );

    let base = manifest("example", "base");

    let report = validate(&[consumer, base]);

    assert!(report.is_valid(), "issues: {:?}", report.issues());
    assert_eq!(
        report.dependency_order(),
        &[id("example", "base"), id("example", "consumer")]
    );
}

#[test]
fn independent_manifest_input_order_does_not_change_results() {
    let first = manifest("example", "first");
    let second = manifest("example", "second");
    let third = manifest("example", "third");

    let forward = validate(&[first.clone(), second.clone(), third.clone()]);
    let reverse = validate(&[third, second, first]);

    assert_eq!(forward, reverse);
    assert!(forward.is_valid());
}

#[test]
fn optional_missing_dependency_does_not_block_validation() {
    let consumer = manifest("example", "consumer").with_dependency(
        ExtensionDependency::new(
            id("example", "optional"),
            VersionRange::any(),
            true,
        ),
    );

    let report = validate(&[consumer]);

    assert!(report.is_valid(), "issues: {:?}", report.issues());
    assert_eq!(report.dependency_order(), &[id("example", "consumer")]);
}

#[test]
fn dependency_versions_accept_a_matching_registered_version() {
    let base = ExtensionManifest::new(
        id("example", "base"),
        version(1, 5, 0),
        ExtensionKind::Domain,
        ExtensionStability::Stable,
        "Versioned base extension",
    )
    .expect("manifest must be valid");

    let consumer = manifest("example", "consumer").with_dependency(
        ExtensionDependency::new(
            id("example", "base"),
            VersionRange::new(
                Some(version(1, 0, 0)),
                Some(version(2, 0, 0)),
            )
            .expect("valid range"),
            false,
        ),
    );

    let report = validate(&[consumer, base]);

    assert!(report.is_valid(), "issues: {:?}", report.issues());
}

// -----------------------------------------------------------------------------
// Invalid dependency graphs and identity conflicts
// -----------------------------------------------------------------------------

#[test]
fn required_missing_dependency_is_rejected() {
    let consumer = manifest("example", "consumer").with_dependency(
        ExtensionDependency::new(
            id("example", "missing"),
            VersionRange::any(),
            false,
        ),
    );

    let report = validate(&[consumer]);

    assert!(!report.is_valid());
    assert!(has_issue(&report, ExtensionIssueKind::MissingDependency));
}

#[test]
fn incompatible_dependency_version_is_rejected() {
    let base = manifest("example", "base");

    let consumer = manifest("example", "consumer").with_dependency(
        ExtensionDependency::new(
            id("example", "base"),
            VersionRange::new(
                Some(version(2, 0, 0)),
                Some(version(3, 0, 0)),
            )
            .expect("valid range"),
            false,
        ),
    );

    let report = validate(&[consumer, base]);

    assert!(!report.is_valid());
    assert!(has_issue(
        &report,
        ExtensionIssueKind::DependencyVersionIncompatible
    ));
}

#[test]
fn self_dependency_is_rejected() {
    let extension = manifest("example", "self").with_dependency(
        ExtensionDependency::new(
            id("example", "self"),
            VersionRange::any(),
            false,
        ),
    );

    let report = validate(&[extension]);

    assert!(!report.is_valid());
    assert!(has_issue(&report, ExtensionIssueKind::SelfDependency));
}

#[test]
fn duplicate_dependency_declarations_are_reported() {
    let dependency = || {
        ExtensionDependency::new(
            id("example", "base"),
            VersionRange::any(),
            false,
        )
    };

    let consumer = manifest("example", "consumer")
        .with_dependency(dependency())
        .with_dependency(dependency());

    let report = validate(&[manifest("example", "base"), consumer]);

    assert!(!report.is_valid());
    assert!(has_issue(&report, ExtensionIssueKind::DuplicateDependency));
}

#[test]
fn dependency_cycles_are_rejected_without_a_partial_activation_order() {
    let first = manifest("example", "first").with_dependency(
        ExtensionDependency::new(
            id("example", "second"),
            VersionRange::any(),
            false,
        ),
    );

    let second = manifest("example", "second").with_dependency(
        ExtensionDependency::new(
            id("example", "first"),
            VersionRange::any(),
            false,
        ),
    );

    let report = validate(&[first, second]);

    assert!(!report.is_valid());
    assert!(report.dependency_order().is_empty());
    assert!(has_issue(&report, ExtensionIssueKind::DependencyCycle));
}

#[test]
fn duplicate_extension_identities_are_rejected() {
    let first = ExtensionManifest::new(
        id("example", "duplicate"),
        version(1, 0, 0),
        ExtensionKind::Domain,
        ExtensionStability::Stable,
        "First definition",
    )
    .expect("manifest must be valid");

    let second = ExtensionManifest::new(
        id("example", "duplicate"),
        version(2, 0, 0),
        ExtensionKind::Domain,
        ExtensionStability::Stable,
        "Second definition",
    )
    .expect("manifest must be valid");

    let report = validate(&[first, second]);

    assert!(!report.is_valid());
    assert!(has_issue(&report, ExtensionIssueKind::DuplicateExtension));
}

#[test]
fn conflicts_between_registered_extensions_are_rejected() {
    let first = manifest("example", "first")
        .with_conflict(id("example", "second"));

    let second = manifest("example", "second");

    let report = validate(&[first, second]);

    assert!(!report.is_valid());
    assert!(has_issue(&report, ExtensionIssueKind::ExtensionConflict));
}

#[test]
fn duplicate_contribution_identity_across_extensions_is_rejected() {
    let contribution = || {
        ExtensionContribution::new(
            id("quantum.operations", "custom_operation"),
            version(1, 0, 0),
        )
    };

    let first = manifest("example", "first").with_contribution(contribution());
    let second = manifest("example", "second").with_contribution(contribution());

    let report = validate(&[first, second]);

    assert!(!report.is_valid());
    assert!(has_issue(
        &report,
        ExtensionIssueKind::DuplicateContribution
    ));
}

// -----------------------------------------------------------------------------
// Language-version and extension-policy checks
// -----------------------------------------------------------------------------

#[test]
fn language_version_incompatibility_is_reported() {
    let extension = manifest("example", "future")
        .with_language_versions(
            VersionRange::new(
                Some(version(2, 0, 0)),
                Some(version(3, 0, 0)),
            )
            .expect("valid range"),
        );

    let report = validate(&[extension]);

    assert!(!report.is_valid());
    assert!(has_issue(
        &report,
        ExtensionIssueKind::LanguageVersionIncompatible
    ));
}

#[test]
fn stable_only_policy_rejects_experimental_extensions() {
    let experimental = ExtensionManifest::new(
        id("example", "preview"),
        version(0, 1, 0),
        ExtensionKind::Experimental,
        ExtensionStability::Experimental,
        "Experimental test extension",
    )
    .expect("manifest must be valid");

    let report = validate(&[experimental]);

    assert!(!report.is_valid());
    assert!(has_issue(&report, ExtensionIssueKind::PolicyViolation));
}

#[test]
fn allow_experimental_policy_accepts_explicit_experimental_metadata() {
    let experimental = ExtensionManifest::new(
        id("example", "preview"),
        version(0, 1, 0),
        ExtensionKind::Experimental,
        ExtensionStability::Experimental,
        "Experimental test extension",
    )
    .expect("manifest must be valid");

    let report = validate_extensions(
        &[experimental],
        language_version(),
        ExtensionPolicy::AllowExperimental,
    );

    assert!(report.is_valid(), "issues: {:?}", report.issues());
}

#[test]
fn disabled_policy_rejects_non_core_extensions() {
    let external = ExtensionManifest::new(
        id("vendor", "feature"),
        version(1, 0, 0),
        ExtensionKind::External,
        ExtensionStability::Stable,
        "External test extension",
    )
    .expect("manifest must be valid");

    let report = validate_extensions(
        &[external],
        language_version(),
        ExtensionPolicy::Disabled,
    );

    assert!(!report.is_valid());
    assert!(has_issue(&report, ExtensionIssueKind::PolicyViolation));
}

#[test]
fn disabled_policy_preserves_core_definitions() {
    let core = ExtensionManifest::new(
        id("zamani.core", "standard"),
        version(1, 0, 0),
        ExtensionKind::Core,
        ExtensionStability::Stable,
        "Core test definition",
    )
    .expect("manifest must be valid");

    let report = validate_extensions(
        &[core],
        language_version(),
        ExtensionPolicy::Disabled,
    );

    assert!(report.is_valid(), "issues: {:?}", report.issues());
}

// -----------------------------------------------------------------------------
// Determinism, diagnostics, and large dependency graphs
// -----------------------------------------------------------------------------

#[test]
fn validation_issues_have_stable_ordering() {
    let first = manifest("example", "first").with_dependency(
        ExtensionDependency::new(
            id("missing", "alpha"),
            VersionRange::any(),
            false,
        ),
    );

    let second = manifest("example", "second").with_dependency(
        ExtensionDependency::new(
            id("missing", "beta"),
            VersionRange::any(),
            false,
        ),
    );

    let third = manifest("example", "third").with_dependency(
        ExtensionDependency::new(
            id("missing", "gamma"),
            VersionRange::any(),
            false,
        ),
    );

    let forward = validate(&[first.clone(), second.clone(), third.clone()]);
    let reverse = validate(&[third, second, first]);

    assert_eq!(forward.issues(), reverse.issues());
    assert_eq!(forward.dependency_order(), reverse.dependency_order());
}

#[test]
fn issue_records_expose_machine_readable_categories() {
    let extension = manifest("example", "consumer").with_dependency(
        ExtensionDependency::new(
            id("example", "missing"),
            VersionRange::any(),
            false,
        ),
    );

    let report = validate(&[extension]);
    let issue = report
        .issues()
        .iter()
        .find(|issue| issue.kind == ExtensionIssueKind::MissingDependency)
        .expect("missing dependency must have a structured issue");

    assert_eq!(issue.extension, Some(id("example", "consumer")));
    assert_eq!(issue.related_extension, Some(id("example", "missing")));
    assert!(!issue.message.trim().is_empty());
}

#[test]
fn large_dependency_chain_is_valid_and_dependency_first() {
    // This is a representative stress workload, not a language limit.
    // The implementation must not interpret this test size as a maximum.
    let workload_size = 1_024_usize;
    let mut manifests = Vec::with_capacity(workload_size);

    for index in 0..workload_size {
        let current_name = format!("node-{index}");
        let current = if index == 0 {
            manifest("stress", &current_name)
        } else {
            manifest("stress", &current_name).with_dependency(
                ExtensionDependency::new(
                    id("stress", format!("node-{}", index - 1)),
                    VersionRange::any(),
                    false,
                ),
            )
        };

        manifests.push(current);
    }

    // Reverse declaration order to ensure the algorithm uses dependencies,
    // not source or registration order, to construct its result.
    manifests.reverse();

    let report = validate(&manifests);

    assert!(report.is_valid(), "issues: {:?}", report.issues());
    assert_eq!(report.dependency_order().len(), workload_size);

    for index in 0..workload_size {
        assert_eq!(
            report.dependency_order()[index],
            id("stress", format!("node-{index}"))
        );
    }
}

// -----------------------------------------------------------------------------
// Grammar and specification boundary checks
// -----------------------------------------------------------------------------

#[test]
fn canonical_root_grammar_exists_and_describes_composition() {
    let grammar = include_str!("../../../../grammar/Zamani.g4");

    assert!(!grammar.trim().is_empty());
    assert!(grammar.contains("Zamani"));
    assert!(
        grammar.to_ascii_lowercase().contains("grammar"),
        "root grammar should document its grammar composition role"
    );
}

#[test]
fn dialect_extension_grammar_retains_its_own_declaration_rule() {
    let extension_points =
        include_str!("../../../../grammar/dialects/extension-points.g4");
    let vendor_grammar = include_str!("../../../../grammar/dialects/vendor.g4");

    assert!(!extension_points.trim().is_empty());
    assert!(!vendor_grammar.trim().is_empty());

    assert!(
        vendor_grammar.contains("dialectVendorExtensionDeclaration"),
        "vendor extension syntax must remain owned by its grammar"
    );
}

#[test]
fn custom_effect_extension_syntax_remains_separately_owned() {
    let grammar =
        include_str!("../../../../grammar/effects/custom-effects.g4");

    assert!(!grammar.trim().is_empty());
    assert!(
        grammar.contains("customEffectExtensionDeclaration"),
        "custom effect extension syntax must remain in its canonical grammar"
    );
}

#[test]
fn normative_specifications_preserve_semantic_extension_boundaries() {
    let extensibility =
        include_str!("../../../../grammar/specification/extensibility.md");
    let semantic_model =
        include_str!("../../../../grammar/specification/semantic-model.md");
    let boundaries =
        include_str!("../../../../grammar/validation/semantic-boundaries.md");

    assert!(!extensibility.trim().is_empty());
    assert!(!semantic_model.trim().is_empty());
    assert!(!boundaries.trim().is_empty());

    assert!(
        extensibility.contains("Extensions may add capability"),
        "the extensibility specification must preserve the portable core"
    );

    assert!(
        semantic_model.contains("independently of the machine"),
        "semantic meaning must not depend on a particular target"
    );

    assert!(
        boundaries.contains("POCO-REAF"),
        "semantic boundaries must document the portability contract"
    );
}
