#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Compatibility Integration Tests
//!
//! Path: `src/zuir/tests/compatibility.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Exercise the public compatibility contracts used by ZUIR, including
//! version comparisons, multidimensional compatibility reports, migration
//! requirements, strict policy evaluation, deterministic ordering, and
//! serialization stability.
//!
//! AUTHORITY
//!
//! - src/zuir/ZUIR-SPEC.md
//! - src/zuir/version.rs
//! - src/zuir/spec/compatibility.rs
//! - src/zuir/spec/mod.rs
//! - grammar/specification/compatibility.md
//! - grammar/specification/grammar-authority.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/Zamani.g4
//!
//! OWNS
//!
//! Regression tests for the public compatibility contracts. This file does
//! not own compatibility policy, version algorithms, serialization
//! implementations, or migrations.
//!
//! DOES_NOT_OWN
//!
//! - Source grammar, AST construction, or semantic analysis.
//! - IR construction or verification.
//! - Migration execution.
//! - Hardware discovery, allocation, scheduling, or target feasibility.
//! - Proof that two programs have equivalent semantics.
//!
//! DEPENDENCIES
//!
//! Uses the canonical version and compatibility APIs. Does not import grammar,
//! AST, quantum IR, hardware, or backend implementation types.
//!
//! SCALABILITY
//!
//! Tests use dynamically sized collections and generated test cases. Any
//! finite test workload is a test fixture, not a universal implementation
//! ceiling. Real execution remains subject to available resources.
//!
//! POCO-REAF
//!
//! Compatibility decisions must be explicit and independent of a specific
//! target. Unknown required semantics, unsupported future versions, and
//! missing required evidence must not be silently accepted.
//!
//! FREEZE CRITERIA
//!
//! This file may be frozen when the module is wired into the crate, these
//! tests compile against the actual public APIs, and the relevant Cargo test
//! commands pass on the declared Rust baseline.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::zuir::spec::compatibility::{
    compare_version_triples, ChangeClass, ChangeImpact, CompatibilityDimension,
    CompatibilityPolicy as ReportPolicy, CompatibilityReason as ReportReason,
    CompatibilityReport, CompatibilityStatus, DimensionDecision, MigrationRequirement,
    REQUIRED_CONFORMANCE,
};

use crate::zuir::version::{
    check_format_compatibility, check_schema_compatibility, compare_versions,
    same_dialect_major, Compatibility as VersionCompatibility,
    CompatibilityPolicy as VersionPolicy, CompatibilityReason as VersionReason,
    DialectVersion, LanguageVersion, VersionIdentity, VersionTriple,
    ZuirFormatVersion, ZuirSchemaVersion, CURRENT_ZUIR_FORMAT_VERSION,
    CURRENT_ZUIR_SCHEMA_VERSION, INITIAL_DIALECT_VERSION,
};

fn compatible(dimension: CompatibilityDimension) -> DimensionDecision {
    DimensionDecision::new(
        dimension,
        CompatibilityStatus::Compatible,
        ReportReason::ExactMatch,
        "The test fixture provides matching compatibility evidence.",
    )
}

fn decision(
    dimension: CompatibilityDimension,
    status: CompatibilityStatus,
    reason: ReportReason,
) -> DimensionDecision {
    DimensionDecision::new(
        dimension,
        status,
        reason,
        "Compatibility decision supplied by the test fixture.",
    )
}

fn report_policy(
    required: impl IntoIterator<Item = CompatibilityDimension>,
    allow_migrations: bool,
) -> ReportPolicy {
    ReportPolicy {
        required_dimensions: required.into_iter().collect::<BTreeSet<_>>(),
        allow_migrations,
        allow_opaque_optional_extensions: false,
        allow_unknown_for_non_executable_artifacts: false,
    }
}

fn complete_report(
    dimensions: impl IntoIterator<Item = CompatibilityDimension>,
) -> CompatibilityReport {
    let mut report = CompatibilityReport::new();

    for dimension in dimensions {
        report.push(compatible(dimension));
    }

    report
}

fn all_known_dimensions() -> [CompatibilityDimension; 30] {
    [
        CompatibilityDimension::Lexical,
        CompatibilityDimension::Grammar,
        CompatibilityDimension::SourceLanguage,
        CompatibilityDimension::Ast,
        CompatibilityDimension::TypeSystem,
        CompatibilityDimension::Semantics,
        CompatibilityDimension::Effects,
        CompatibilityDimension::Capabilities,
        CompatibilityDimension::Resources,
        CompatibilityDimension::Ownership,
        CompatibilityDimension::ControlFlow,
        CompatibilityDimension::NumericalBehavior,
        CompatibilityDimension::Concurrency,
        CompatibilityDimension::QuantumSemantics,
        CompatibilityDimension::HdlSemantics,
        CompatibilityDimension::DialectSchema,
        CompatibilityDimension::OperationSemantics,
        CompatibilityDimension::ZuirSchema,
        CompatibilityDimension::SerializationFormat,
        CompatibilityDimension::PublicApi,
        CompatibilityDimension::SymbolLinkage,
        CompatibilityDimension::Provenance,
        CompatibilityDimension::Diagnostics,
        CompatibilityDimension::Transformations,
        CompatibilityDimension::Lowering,
        CompatibilityDimension::RuntimeInterface,
        CompatibilityDimension::TargetRealization,
        CompatibilityDimension::Security,
        CompatibilityDimension::Determinism,
        CompatibilityDimension::Extensions,
    ]
}

// ---------------------------------------------------------------------------
// Stable compatibility identifiers and change classification
// ---------------------------------------------------------------------------

#[test]
fn compatibility_dimension_identifiers_are_unique_and_non_empty() {
    let dimensions = all_known_dimensions();
    let identifiers: BTreeSet<&str> =
        dimensions.iter().map(|dimension| dimension.as_str()).collect();

    assert_eq!(
        identifiers.len(),
        dimensions.len(),
        "compatibility dimensions must not share stable identifiers"
    );

    assert!(
        identifiers.iter().all(|identifier| !identifier.is_empty()),
        "every compatibility dimension must have a stable non-empty identifier"
    );
}

#[test]
fn compatibility_dimension_identifiers_use_stable_serialized_names() {
    assert_eq!(
        serde_json::to_string(&CompatibilityDimension::QuantumSemantics)
            .expect("dimension serialization must succeed"),
        "\"quantum_semantics\""
    );

    assert_eq!(
        serde_json::to_string(&CompatibilityDimension::ZuirSchema)
            .expect("dimension serialization must succeed"),
        "\"zuir_schema\""
    );

    assert_eq!(
        serde_json::from_str::<CompatibilityDimension>("\"serialization_format\"")
            .expect("known serialized dimension must deserialize"),
        CompatibilityDimension::SerializationFormat
    );
}

#[test]
fn change_classes_have_conservative_default_impacts() {
    assert_eq!(
        ChangeClass::Additive.default_impact(),
        ChangeImpact::Preserved
    );
    assert_eq!(
        ChangeClass::Internal.default_impact(),
        ChangeImpact::Preserved
    );
    assert_eq!(
        ChangeClass::BugFix.default_impact(),
        ChangeImpact::Undetermined
    );
    assert_eq!(
        ChangeClass::Migratable.default_impact(),
        ChangeImpact::RequiresMigration
    );
    assert_eq!(
        ChangeClass::Breaking.default_impact(),
        ChangeImpact::Breaking
    );
    assert_eq!(
        ChangeClass::Unknown.default_impact(),
        ChangeImpact::Undetermined
    );
}

// ---------------------------------------------------------------------------
// Version comparison contracts
// ---------------------------------------------------------------------------

#[test]
fn exact_version_is_compatible_without_migration() {
    let version = VersionTriple::new(1, 2, 3);

    let result = compare_version_triples(version, version, version, false);

    assert_eq!(result.dimension, CompatibilityDimension::ZuirSchema);
    assert_eq!(result.status, CompatibilityStatus::Compatible);
    assert_eq!(result.reason, ReportReason::ExactMatch);
    assert!(result.is_compatible_without_migration());
    assert!(result.migration.is_none());
}

#[test]
fn older_same_major_version_requires_migration_when_not_explicitly_allowed() {
    let result = compare_version_triples(
        VersionTriple::new(1, 2, 0),
        VersionTriple::new(1, 4, 0),
        VersionTriple::new(1, 0, 0),
        false,
    );

    assert_eq!(result.status, CompatibilityStatus::RequiresMigration);
    assert_eq!(result.reason, ReportReason::MigrationRequired);
    assert!(!result.is_compatible_without_migration());
}

#[test]
fn explicitly_supported_older_version_can_be_accepted_without_migration() {
    let result = compare_version_triples(
        VersionTriple::new(1, 2, 0),
        VersionTriple::new(1, 4, 0),
        VersionTriple::new(1, 0, 0),
        true,
    );

    assert_eq!(result.status, CompatibilityStatus::Compatible);
    assert_eq!(result.reason, ReportReason::SupportedOlderVersion);
}

#[test]
fn future_version_is_rejected_even_when_older_versions_are_allowed() {
    let result = compare_version_triples(
        VersionTriple::new(2, 0, 0),
        VersionTriple::new(1, 9, 0),
        VersionTriple::new(1, 0, 0),
        true,
    );

    assert_eq!(result.status, CompatibilityStatus::Incompatible);
    assert_eq!(result.reason, ReportReason::FutureVersion);
}

#[test]
fn major_version_mismatch_is_rejected() {
    let result = compare_version_triples(
        VersionTriple::new(2, 0, 0),
        VersionTriple::new(1, 9, 9),
        VersionTriple::new(1, 0, 0),
        true,
    );

    assert_eq!(result.status, CompatibilityStatus::Incompatible);
    assert_eq!(result.reason, ReportReason::MajorVersionMismatch);
}

#[test]
fn version_below_minimum_supported_is_rejected() {
    let result = compare_version_triples(
        VersionTriple::new(1, 0, 0),
        VersionTriple::new(1, 4, 0),
        VersionTriple::new(1, 1, 0),
        true,
    );

    assert_eq!(result.status, CompatibilityStatus::Incompatible);
    assert_eq!(result.reason, ReportReason::BelowMinimumSupported);
}

#[test]
fn version_policy_rejects_an_invalid_supported_range() {
    let policy = VersionPolicy::new(
        VersionTriple::new(1, 2, 0),
        VersionTriple::new(1, 3, 0),
        true,
    );

    assert!(
        policy.is_none(),
        "the minimum supported version cannot exceed the current version"
    );
}

#[test]
fn exact_version_policy_rejects_older_versions_without_migration() {
    let current = VersionTriple::new(3, 0, 0);
    let policy = VersionPolicy::exact(current);

    let result = policy.check(VersionTriple::new(2, 9, 9));

    assert_eq!(
        result.compatibility(),
        VersionCompatibility::Incompatible
    );
    assert_eq!(result.reason(), VersionReason::MajorVersionMismatch);
    assert!(result.must_reject_without_migration());
}

#[test]
fn version_policy_distinguishes_future_versions() {
    let current = VersionTriple::new(3, 0, 0);
    let policy = VersionPolicy::exact(current);

    let result = policy.check(VersionTriple::new(4, 0, 0));

    assert_eq!(
        result.compatibility(),
        VersionCompatibility::Incompatible
    );
    assert_eq!(result.reason(), VersionReason::MajorVersionMismatch);
}

#[test]
fn schema_and_format_versions_are_checked_independently() {
    let schema_policy = VersionPolicy::exact(
        CURRENT_ZUIR_SCHEMA_VERSION.components(),
    );
    let format_policy = VersionPolicy::exact(
        CURRENT_ZUIR_FORMAT_VERSION.components(),
    );

    let schema_result =
        check_schema_compatibility(CURRENT_ZUIR_SCHEMA_VERSION, schema_policy);
    let format_result =
        check_format_compatibility(CURRENT_ZUIR_FORMAT_VERSION, format_policy);

    assert_eq!(
        schema_result.compatibility(),
        VersionCompatibility::Compatible
    );
    assert_eq!(
        format_result.compatibility(),
        VersionCompatibility::Compatible
    );

    assert_eq!(
        schema_result.source(),
        CURRENT_ZUIR_SCHEMA_VERSION.components()
    );
    assert_eq!(
        format_result.source(),
        CURRENT_ZUIR_FORMAT_VERSION.components()
    );
}

#[test]
fn different_version_categories_do_not_implicitly_share_identity() {
    let schema = ZuirSchemaVersion::new(1, 2, 3);
    let format = ZuirFormatVersion::new(1, 2, 3);

    // Numeric components may be compared when explicitly requested, but the
    // strongly typed wrappers remain distinct Rust types.
    assert_eq!(schema.components(), format.components());
    assert_eq!(compare_versions(schema, format), std::cmp::Ordering::Equal);

    let dialect = DialectVersion::new(1, 0, 0);
    let language = LanguageVersion::new(1, 0, 0);

    assert_eq!(dialect.components(), language.components());
}

#[test]
fn dialect_major_check_is_only_a_coarse_compatibility_check() {
    assert!(same_dialect_major(
        DialectVersion::new(2, 1, 0),
        DialectVersion::new(2, 9, 7),
    ));

    assert!(!same_dialect_major(
        DialectVersion::new(2, 0, 0),
        DialectVersion::new(3, 0, 0),
    ));

    assert!(same_dialect_major(
        INITIAL_DIALECT_VERSION,
        DialectVersion::new(1, 99, 99),
    ));
}

#[test]
fn version_component_increments_are_checked_at_numeric_boundaries() {
    let maximum = VersionTriple::new(u32::MAX, u32::MAX, u32::MAX);

    assert_eq!(maximum.checked_next_patch(), None);
    assert_eq!(maximum.checked_next_minor(), None);
    assert_eq!(maximum.checked_next_major(), None);

    assert_eq!(
        VersionTriple::new(1, 2, 3).checked_next_patch(),
        Some(VersionTriple::new(1, 2, 4))
    );
    assert_eq!(
        VersionTriple::new(1, 2, 3).checked_next_minor(),
        Some(VersionTriple::new(1, 3, 0))
    );
    assert_eq!(
        VersionTriple::new(1, 2, 3).checked_next_major(),
        Some(VersionTriple::new(2, 0, 0))
    );
}

#[test]
fn version_identity_display_and_components_are_consistent() {
    let version = ZuirSchemaVersion::new(2, 7, 11);

    assert_eq!(version.major(), 2);
    assert_eq!(version.minor(), 7);
    assert_eq!(version.patch(), 11);
    assert_eq!(version.components(), VersionTriple::new(2, 7, 11));
    assert_eq!(version.to_string(), "2.7.11");
    assert_eq!(version.components(), VersionIdentity::components(version));
}

// ---------------------------------------------------------------------------
// Compatibility report aggregation and strict policy
// ---------------------------------------------------------------------------

#[test]
fn empty_report_never_proves_compatibility() {
    let report = CompatibilityReport::new();

    assert_eq!(report.status(), CompatibilityStatus::Unknown);
    assert!(!report.is_compatible_without_migration());
    assert!(report.must_reject_without_resolution());
}

#[test]
fn report_requires_every_required_dimension() {
    let policy = report_policy(
        [
            CompatibilityDimension::Semantics,
            CompatibilityDimension::TypeSystem,
            CompatibilityDimension::Effects,
        ],
        false,
    );

    let report = complete_report([
        CompatibilityDimension::Semantics,
        CompatibilityDimension::TypeSystem,
    ]);

    assert_eq!(policy.evaluate(&report), CompatibilityStatus::Unknown);
}

#[test]
fn complete_required_evidence_satisfies_policy() {
    let required = [
        CompatibilityDimension::Semantics,
        CompatibilityDimension::TypeSystem,
        CompatibilityDimension::Effects,
        CompatibilityDimension::QuantumSemantics,
        CompatibilityDimension::HdlSemantics,
    ];

    let policy = report_policy(required, false);
    let report = complete_report(required);

    assert_eq!(policy.evaluate(&report), CompatibilityStatus::Compatible);
}

#[test]
fn empty_required_policy_does_not_prove_compatibility() {
    let policy = report_policy([], false);
    let report = CompatibilityReport::new();

    assert_eq!(policy.evaluate(&report), CompatibilityStatus::Unknown);
}

#[test]
fn incompatible_decision_takes_precedence_in_aggregate_report() {
    let mut report = CompatibilityReport::new();

    report.push(decision(
        CompatibilityDimension::Semantics,
        CompatibilityStatus::Unknown,
        ReportReason::InsufficientEvidence,
    ));
    report.push(decision(
        CompatibilityDimension::Security,
        CompatibilityStatus::Incompatible,
        ReportReason::ContractViolation,
    ));
    report.push(decision(
        CompatibilityDimension::Resources,
        CompatibilityStatus::ResourceExhausted,
        ReportReason::ResourceBudgetExceeded,
    ));

    assert_eq!(report.status(), CompatibilityStatus::Incompatible);
}

#[test]
fn resource_exhaustion_is_not_misreported_as_semantic_incompatibility() {
    let mut report = CompatibilityReport::new();

    report.push(decision(
        CompatibilityDimension::Semantics,
        CompatibilityStatus::ResourceExhausted,
        ReportReason::ResourceBudgetExceeded,
    ));

    assert_eq!(report.status(), CompatibilityStatus::ResourceExhausted);
    assert_ne!(report.status(), CompatibilityStatus::Incompatible);
}

#[test]
fn migration_is_rejected_when_policy_disallows_it() {
    let policy = report_policy([CompatibilityDimension::Semantics], false);
    let mut report = CompatibilityReport::new();

    report.push(decision(
        CompatibilityDimension::Semantics,
        CompatibilityStatus::RequiresMigration,
        ReportReason::MigrationRequired,
    ));

    assert_eq!(policy.evaluate(&report), CompatibilityStatus::Incompatible);
}

#[test]
fn migration_can_be_reported_when_policy_allows_it() {
    let policy = report_policy([CompatibilityDimension::ZuirSchema], true);
    let mut report = CompatibilityReport::new();

    report.push(decision(
        CompatibilityDimension::ZuirSchema,
        CompatibilityStatus::RequiresMigration,
        ReportReason::MigrationRequired,
    ));

    assert_eq!(
        policy.evaluate(&report),
        CompatibilityStatus::RequiresMigration
    );
}

#[test]
fn strict_policy_requires_semantic_and_execution_relevant_dimensions() {
    let policy = ReportPolicy::strict();

    for required in [
        CompatibilityDimension::Semantics,
        CompatibilityDimension::TypeSystem,
        CompatibilityDimension::Effects,
        CompatibilityDimension::Capabilities,
        CompatibilityDimension::Resources,
        CompatibilityDimension::Ownership,
        CompatibilityDimension::OperationSemantics,
        CompatibilityDimension::DialectSchema,
        CompatibilityDimension::ZuirSchema,
        CompatibilityDimension::SerializationFormat,
        CompatibilityDimension::Extensions,
        CompatibilityDimension::Lowering,
        CompatibilityDimension::Security,
    ] {
        assert!(
            policy.required_dimensions.contains(&required),
            "strict policy is missing required dimension {required}"
        );
    }

    assert!(!policy.allow_unknown_for_non_executable_artifacts);
}

#[test]
fn exact_policy_disallows_migrations_and_opaque_optional_extensions() {
    let policy = ReportPolicy::exact();

    assert!(!policy.allow_migrations);
    assert!(!policy.allow_opaque_optional_extensions);
    assert!(!policy.allow_unknown_for_non_executable_artifacts);
}

#[test]
fn report_lookup_returns_the_decision_for_a_dimension() {
    let mut report = CompatibilityReport::new();

    report.push(compatible(CompatibilityDimension::Semantics));

    assert!(report.decision(CompatibilityDimension::Semantics).is_some());
    assert!(report.decision(CompatibilityDimension::Security).is_none());
}

// ---------------------------------------------------------------------------
// Migration contract
// ---------------------------------------------------------------------------

#[test]
fn migration_requirement_is_explicit_and_requires_pre_interpretation_migration() {
    let migration = MigrationRequirement {
        migration_id: "zuir.schema.v1_to_v2".to_owned(),
        from: VersionTriple::new(1, 0, 0),
        to: VersionTriple::new(2, 0, 0),
        rationale: "Convert the old schema before interpreting its meaning."
            .to_owned(),
        required_before_interpretation: true,
    };

    let result = DimensionDecision::requiring_migration(
        CompatibilityDimension::ZuirSchema,
        "The source schema requires a defined conversion.",
        migration.clone(),
    );

    assert_eq!(result.status, CompatibilityStatus::RequiresMigration);
    assert_eq!(result.reason, ReportReason::MigrationRequired);
    assert_eq!(result.migration, Some(migration));
    assert!(!result.is_compatible_without_migration());
}

#[test]
fn report_exposes_migration_requirements_without_executing_them() {
    let migration = MigrationRequirement {
        migration_id: "zuir.dialect.example.v1_to_v2".to_owned(),
        from: VersionTriple::new(1, 0, 0),
        to: VersionTriple::new(2, 0, 0),
        rationale: "Update the declared dialect contract.".to_owned(),
        required_before_interpretation: true,
    };

    let mut report = CompatibilityReport::new();
    report.push(DimensionDecision::requiring_migration(
        CompatibilityDimension::DialectSchema,
        "A dialect migration is required.",
        migration.clone(),
    ));

    let collected: Vec<_> = report.migrations().cloned().collect();

    assert_eq!(collected, vec![migration]);
    assert_eq!(report.status(), CompatibilityStatus::RequiresMigration);
}

// ---------------------------------------------------------------------------
// Determinism and serialization stability
// ---------------------------------------------------------------------------

#[test]
fn report_order_can_be_canonicalized_deterministically() {
    let mut first = CompatibilityReport::new();
    first.push(compatible(CompatibilityDimension::Security));
    first.push(compatible(CompatibilityDimension::Semantics));
    first.push(compatible(CompatibilityDimension::Resources));

    let mut second = CompatibilityReport::new();
    second.push(compatible(CompatibilityDimension::Resources));
    second.push(compatible(CompatibilityDimension::Security));
    second.push(compatible(CompatibilityDimension::Semantics));

    first.sort_deterministically();
    second.sort_deterministically();

    assert_eq!(first, second);
    assert_eq!(
        first
            .decisions
            .iter()
            .map(|item| item.dimension)
            .collect::<Vec<_>>(),
        vec![
            CompatibilityDimension::Semantics,
            CompatibilityDimension::Resources,
            CompatibilityDimension::Security,
        ]
    );
}

#[test]
fn sorted_compatibility_reports_serialize_deterministically() {
    let mut first = CompatibilityReport::new();
    first.push(compatible(CompatibilityDimension::Extensions));
    first.push(compatible(CompatibilityDimension::Semantics));
    first.push(compatible(CompatibilityDimension::ZuirSchema));

    let mut second = CompatibilityReport::new();
    second.push(compatible(CompatibilityDimension::ZuirSchema));
    second.push(compatible(CompatibilityDimension::Extensions));
    second.push(compatible(CompatibilityDimension::Semantics));

    first.sort_deterministically();
    second.sort_deterministically();

    let first_bytes =
        serde_json::to_vec(&first).expect("first report must serialize");
    let second_bytes =
        serde_json::to_vec(&second).expect("second report must serialize");

    assert_eq!(first_bytes, second_bytes);
}

#[test]
fn serialized_report_round_trip_preserves_decisions_and_migrations() {
    let migration = MigrationRequirement {
        migration_id: "zuir.schema.v1_to_v2".to_owned(),
        from: VersionTriple::new(1, 0, 0),
        to: VersionTriple::new(2, 0, 0),
        rationale: "Explicit compatibility migration.".to_owned(),
        required_before_interpretation: true,
    };

    let mut report = CompatibilityReport::new();
    report.push(compatible(CompatibilityDimension::Semantics));
    report.push(DimensionDecision::requiring_migration(
        CompatibilityDimension::ZuirSchema,
        "Schema conversion is required.",
        migration,
    ));
    report.sort_deterministically();

    let encoded =
        serde_json::to_vec(&report).expect("report serialization must succeed");
    let decoded: CompatibilityReport =
        serde_json::from_slice(&encoded).expect("report deserialization must succeed");

    assert_eq!(decoded, report);
    assert_eq!(decoded.status(), CompatibilityStatus::RequiresMigration);
}

#[test]
fn serialized_version_wrappers_round_trip_without_changing_components() {
    let schema = ZuirSchemaVersion::new(7, 12, 4);
    let encoded =
        serde_json::to_vec(&schema).expect("schema version must serialize");
    let decoded: ZuirSchemaVersion =
        serde_json::from_slice(&encoded).expect("schema version must deserialize");

    assert_eq!(decoded, schema);
    assert_eq!(decoded.components(), VersionTriple::new(7, 12, 4));
}

#[test]
fn report_status_is_independent_of_insertion_order() {
    let statuses = [
        (
            CompatibilityStatus::Unknown,
            ReportReason::InsufficientEvidence,
        ),
        (
            CompatibilityStatus::ResourceExhausted,
            ReportReason::ResourceBudgetExceeded,
        ),
        (
            CompatibilityStatus::RequiresMigration,
            ReportReason::MigrationRequired,
        ),
    ];

    let mut forward = CompatibilityReport::new();
    for (status, reason) in statuses {
        forward.push(decision(
            CompatibilityDimension::Semantics,
            status,
            reason,
        ));
    }

    let mut reverse = CompatibilityReport::new();
    for (status, reason) in statuses.into_iter().rev() {
        reverse.push(decision(
            CompatibilityDimension::Semantics,
            status,
            reason,
        ));
    }

    assert_eq!(forward.status(), reverse.status());
    assert_eq!(
        forward.status(),
        CompatibilityStatus::ResourceExhausted
    );
}

// ---------------------------------------------------------------------------
// Conformance declaration and scalable dynamic collections
// ---------------------------------------------------------------------------

#[test]
fn required_conformance_contract_declares_critical_guarantees() {
    assert!(REQUIRED_CONFORMANCE.requires_semantic_preservation);
    assert!(REQUIRED_CONFORMANCE.requires_explicit_migrations);
    assert!(REQUIRED_CONFORMANCE.rejects_unknown_required_semantics);
    assert!(REQUIRED_CONFORMANCE.separates_resource_exhaustion);
    assert!(REQUIRED_CONFORMANCE.requires_deterministic_reports);
    assert!(REQUIRED_CONFORMANCE.forbids_unsafe_code);
}

#[test]
fn compatibility_report_handles_large_dynamic_decision_sets() {
    // This is a stress fixture, not a protocol limit or maximum supported
    // report size. Its size is selected to exercise allocation, iteration,
    // sorting, aggregation, and serialization without asserting infinity.
    let dimensions = all_known_dimensions();
    let repetitions = 128_usize;
    let expected_count = dimensions
        .len()
        .checked_mul(repetitions)
        .expect("test fixture size arithmetic must not overflow");

    let mut report = CompatibilityReport::new();

    for _ in 0..repetitions {
        for dimension in dimensions {
            report.push(compatible(dimension));
        }
    }

    assert_eq!(report.decisions.len(), expected_count);
    assert_eq!(report.status(), CompatibilityStatus::Compatible);

    report.sort_deterministically();

    assert_eq!(report.decisions.len(), expected_count);
    assert_eq!(report.status(), CompatibilityStatus::Compatible);

    let encoded =
        serde_json::to_vec(&report).expect("large report must serialize");
    let decoded: CompatibilityReport =
        serde_json::from_slice(&encoded).expect("large report must deserialize");

    assert_eq!(decoded.decisions.len(), expected_count);
    assert_eq!(decoded.status(), CompatibilityStatus::Compatible);
}

#[test]
fn resource_exhaustion_and_unknown_evidence_remain_distinct() {
    let mut exhausted = CompatibilityReport::new();
    exhausted.push(decision(
        CompatibilityDimension::Semantics,
        CompatibilityStatus::ResourceExhausted,
        ReportReason::ResourceBudgetExceeded,
    ));

    let mut unknown = CompatibilityReport::new();
    unknown.push(decision(
        CompatibilityDimension::Semantics,
        CompatibilityStatus::Unknown,
        ReportReason::InsufficientEvidence,
    ));

    assert_eq!(exhausted.status(), CompatibilityStatus::ResourceExhausted);
    assert_eq!(unknown.status(), CompatibilityStatus::Unknown);
    assert_ne!(exhausted.status(), unknown.status());
}