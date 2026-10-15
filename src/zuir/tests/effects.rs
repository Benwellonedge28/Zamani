#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! ZUIR effect-model conformance tests.
//!
//! Path: src/zuir/tests/effects.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Exercise the existing canonical effect model and its integration with
//! ZuirOperation and the effect-analysis API.
//!
//! These tests cover:
//! - open-world, qualified effect identities;
//! - deterministic effect sets and set algebra;
//! - explicit effect-summary completeness;
//! - effect policy enforcement and registry-backed validation;
//! - operation-level effect declarations;
//! - deterministic aggregation and declared upper-bound checks;
//! - serialization round trips;
//! - dynamically sized collections;
//! - malformed names and duplicate operation identities.
//!
//! OWNERSHIP
//!
//! The production implementation remains in:
//! - crate::zuir::ir::effects
//! - crate::zuir::ir::operation
//! - crate::zuir::analysis::effects
//! - crate::zuir::schema::types
//!
//! This file owns tests only. It must not introduce replacement effect types,
//! duplicate effect semantics, or change production APIs to satisfy tests.
//!
//! INTEGRATION
//!
//! This file is intended to be included by the ZUIR unit-test module tree.
//! It is not a standalone Cargo integration test: `crate::zuir` refers to
//! the current library crate, so the ZUIR module must be reachable from the
//! library root.
//!
//! COMPLETENESS CONTRACT
//!
//! A set of observed or declared effects does not establish that all effects
//! have been discovered. An empty or partial summary must not prove purity.
//! Complete summaries may be used to prove purity only when a trusted
//! semantic analysis establishes that their completeness claim is justified.
//!
//! SCALABILITY
//!
//! Tests use dynamically sized collections and do not encode a universal
//! hardware, operation, effect, qubit, node, or tensor-rank limit. Stress
//! cases are deliberately finite and verify behavior rather than promising
//! literal infinite capacity.

use std::collections::BTreeSet;

use crate::zuir::analysis::effects::{
    EffectAnalysis,
    EffectAnalysisError,
    analyze_effects,
};
use crate::zuir::ids::OperationId;
use crate::zuir::ir::effects::{
    EffectClass,
    EffectCompleteness,
    EffectDefinition,
    EffectError,
    EffectPolicy,
    EffectProperties,
    EffectSet,
    EffectSummary,
    declared_operation_effects,
    validate_operation_effects,
};
use crate::zuir::ir::operation::ZuirOperation;
use crate::zuir::schema::types::QualifiedName;

// -----------------------------------------------------------------------------
// Test helpers
// -----------------------------------------------------------------------------

fn qname(parts: &[&str]) -> QualifiedName {
    QualifiedName::new(parts.iter().copied())
        .expect("test qualified name must be valid")
}

fn effect(name: &str) -> QualifiedName {
    QualifiedName::parse(name)
        .expect("test effect name must be valid")
}

fn effect_set(names: &[&str]) -> EffectSet {
    EffectSet::from_names(names.iter().map(|name| effect(name)))
        .expect("all test effect names must be valid")
}

fn operation(id: u128) -> ZuirOperation {
    ZuirOperation::new(
        OperationId::new(id).expect("test operation ID must be nonzero"),
        qname(&["test", "operation"]),
    )
    .expect("test operation must be valid")
}

fn operation_with_effects(id: u128, names: &[&str]) -> ZuirOperation {
    let mut operation = operation(id);

    for name in names {
        operation
            .add_effect(effect(name))
            .expect("test effect declaration must be valid");
    }

    operation
}

// -----------------------------------------------------------------------------
// Qualified effect identities
// -----------------------------------------------------------------------------

#[test]
fn qualified_effect_names_preserve_their_namespace() {
    let measurement = effect("quantum::measurement");
    let communication = effect("distributed::communication");

    assert_ne!(measurement, communication);
    assert_eq!(measurement.to_string(), "quantum::measurement");
    assert_eq!(
        communication.to_string(),
        "distributed::communication"
    );
}

#[test]
fn arbitrary_future_effect_names_do_not_require_a_core_enum_change() {
    let future = effect("future_domain::new_effect");
    let vendor = effect("vendor_example::accelerator::special_operation");

    let set = EffectSet::from_names([future.clone(), vendor.clone()])
        .expect("open-world effect names must be accepted");

    assert!(set.contains(&future));
    assert!(set.contains(&vendor));
    assert_eq!(set.len(), 2);
}

#[test]
fn qualified_name_parser_rejects_empty_or_malformed_names() {
    assert!(QualifiedName::parse("").is_err());
    assert!(QualifiedName::parse("::measurement").is_err());
    assert!(QualifiedName::parse("quantum::").is_err());
    assert!(QualifiedName::parse("quantum::::measurement").is_err());
}

#[test]
fn effect_definition_rejects_invalid_names() {
    assert!(QualifiedName::new(Vec::<String>::new()).is_err());

    let definition = EffectDefinition::new(
        effect("quantum::measurement"),
        EffectClass::Measurement,
        EffectProperties::externally_observable(),
    )
    .expect("valid effect definition must be accepted");

    assert_eq!(
        definition.name().to_string(),
        "quantum::measurement"
    );
    assert_eq!(definition.class(), EffectClass::Measurement);
    assert!(definition.validate().is_ok());
}

// -----------------------------------------------------------------------------
// Effect-set invariants
// -----------------------------------------------------------------------------

#[test]
fn effect_set_deduplicates_names() {
    let read = effect("core::memory_read");
    let io = effect("core::io");

    let set = EffectSet::from_names([
        read.clone(),
        io.clone(),
        read.clone(),
    ])
    .expect("valid effect names must be accepted");

    assert_eq!(set.len(), 2);
    assert!(set.contains(&read));
    assert!(set.contains(&io));
}

#[test]
fn effect_set_iteration_is_deterministic() {
    let first = effect_set(&[
        "network::request",
        "core::memory_read",
        "quantum::measurement",
        "distributed::message",
    ]);

    let second = effect_set(&[
        "distributed::message",
        "quantum::measurement",
        "network::request",
        "core::memory_read",
    ]);

    let first_order: Vec<String> =
        first.iter().map(ToString::to_string).collect();
    let second_order: Vec<String> =
        second.iter().map(ToString::to_string).collect();

    assert_eq!(first_order, second_order);
}

#[test]
fn effect_set_union_is_idempotent() {
    let left = effect_set(&[
        "core::memory_read",
        "core::io",
    ]);

    let union = left.union(&left);

    assert_eq!(union, left);
}

#[test]
fn effect_set_union_is_associative() {
    let a = effect_set(&["core::memory_read"]);
    let b = effect_set(&["core::io"]);
    let c = effect_set(&["quantum::measurement"]);

    assert_eq!(
        a.union(&b).union(&c),
        a.union(&b.union(&c))
    );
}

#[test]
fn effect_set_union_deduplicates_overlapping_members() {
    let left = effect_set(&[
        "core::memory_read",
        "core::io",
    ]);

    let right = effect_set(&[
        "core::io",
        "network::request",
    ]);

    let combined = left.union(&right);

    assert_eq!(combined.len(), 3);
    assert!(combined.contains(&effect("core::memory_read")));
    assert!(combined.contains(&effect("core::io")));
    assert!(combined.contains(&effect("network::request")));
}

#[test]
fn effect_set_subset_and_contains_all_are_consistent() {
    let smaller = effect_set(&["core::memory_read"]);
    let larger = effect_set(&[
        "core::memory_read",
        "core::io",
    ]);

    assert!(smaller.is_subset_of(&larger));
    assert!(larger.contains_all(&smaller));

    assert!(!larger.is_subset_of(&smaller));
    assert!(!smaller.contains_all(&larger));
}

#[test]
fn removing_an_effect_reports_whether_the_set_changed() {
    let mut set = effect_set(&["core::io"]);
    let io = effect("core::io");
    let absent = effect("core::memory_read");

    assert!(set.remove(&io));
    assert!(!set.contains(&io));

    assert!(!set.remove(&io));
    assert!(!set.remove(&absent));
}

#[test]
fn empty_effect_set_is_a_valid_collection() {
    let set = EffectSet::new();

    assert!(set.is_empty());
    assert_eq!(set.len(), 0);
    assert!(set.validate().is_ok());
}

// -----------------------------------------------------------------------------
// Completeness and purity
// -----------------------------------------------------------------------------

#[test]
fn unknown_empty_summary_does_not_prove_purity() {
    let summary = EffectSummary::unknown();

    assert_eq!(
        summary.completeness(),
        EffectCompleteness::Unknown
    );
    assert!(
        summary.is_proven_pure().is_err(),
        "unknown effect information must not prove purity"
    );
}

#[test]
fn partial_empty_summary_does_not_prove_purity() {
    let summary = EffectSummary::new(
        EffectSet::new(),
        EffectCompleteness::Partial,
    );

    assert!(
        summary.is_proven_pure().is_err(),
        "an empty partial summary must not prove purity"
    );
}

#[test]
fn complete_empty_summary_can_prove_purity() {
    // This test exercises the summary API's contract. In production, the
    // caller must only construct a Complete summary when a trusted analysis
    // establishes that the relevant scope is complete.
    let summary = EffectSummary::complete(EffectSet::new());

    assert_eq!(summary.is_proven_pure(), Ok(true));
}

#[test]
fn complete_nonempty_summary_is_not_pure() {
    let summary = EffectSummary::complete(
        effect_set(&["core::io"]),
    );

    assert_eq!(summary.is_proven_pure(), Ok(false));
}

#[test]
fn summary_validation_checks_the_effect_set() {
    let summary = EffectSummary::new(
        effect_set(&[
            "core::io",
            "quantum::measurement",
        ]),
        EffectCompleteness::Partial,
    );

    assert!(summary.validate().is_ok());
}

// -----------------------------------------------------------------------------
// Effect definitions and policies
// -----------------------------------------------------------------------------

#[test]
fn permissive_policy_accepts_well_formed_open_world_names() {
    let policy = EffectPolicy::permissive();
    let declared = effect_set(&[
        "future_domain::operation",
        "quantum::measurement",
        "distributed::message",
    ]);

    assert!(policy.validate(&declared).is_ok());
}

#[test]
fn allow_list_rejects_an_effect_not_in_the_list() {
    let policy = EffectPolicy::allow_only(
        effect_set(&["core::memory_read"]),
    );

    let declared = effect_set(&["core::io"]);

    assert!(matches!(
        policy.validate(&declared),
        Err(EffectError::EffectNotAllowed { .. })
    ));
}

#[test]
fn allow_list_accepts_a_subset_of_allowed_effects() {
    let policy = EffectPolicy::allow_only(effect_set(&[
        "core::memory_read",
        "core::io",
        "network::request",
    ]));

    let declared = effect_set(&[
        "core::memory_read",
        "network::request",
    ]);

    assert!(policy.validate(&declared).is_ok());
}

#[test]
fn forbidden_effect_is_rejected_even_when_allow_list_contains_it() {
    let io = effect("core::io");

    let mut policy = EffectPolicy::allow_only(
        effect_set(&["core::io", "core::memory_read"]),
    );

    policy
        .forbid(io.clone())
        .expect("valid forbidden effect must be accepted");

    assert!(matches!(
        policy.validate(&effect_set(&["core::io"])),
        Err(EffectError::ForbiddenEffect { effect })
            if effect == io.to_string()
    ));
}

#[test]
fn required_effect_must_be_present() {
    let mut policy = EffectPolicy::permissive();

    policy
        .require(effect("security::authorize"))
        .expect("valid required effect must be accepted");

    assert!(matches!(
        policy.validate(&EffectSet::new()),
        Err(EffectError::MissingRequiredEffect { .. })
    ));
}

#[test]
fn required_effect_is_satisfied_when_present() {
    let mut policy = EffectPolicy::permissive();

    policy
        .require(effect("security::authorize"))
        .expect("valid required effect must be accepted");

    assert!(
        policy
            .validate(&effect_set(&[
                "security::authorize",
                "core::io",
            ]))
            .is_ok()
    );
}

#[test]
fn requiring_registered_definitions_needs_registry_backed_validation() {
    let declared = effect_set(&["custom_domain::effect"]);
    let mut policy = EffectPolicy::permissive();

    policy.require_registered_definitions(true);

    assert!(policy.needs_registered_definitions());

    assert!(matches!(
        policy.validate(&declared),
        Err(EffectError::IncompleteEffectInformation { .. })
    ));
}

#[test]
fn registry_backed_validation_rejects_unknown_effects() {
    let declared = effect_set(&["custom_domain::effect"]);
    let policy = EffectPolicy::permissive();

    assert!(matches!(
        policy.validate_with_registry(&declared, |_| None),
        Err(EffectError::UnknownEffect { .. })
    ));
}

#[test]
fn registry_backed_validation_accepts_a_matching_definition() {
    let name = effect("extension_example::effect");

    let definition = EffectDefinition::new(
        name.clone(),
        EffectClass::Extension,
        EffectProperties::externally_observable(),
    )
    .expect("extension effect definition must be valid");

    let declared = EffectSet::from_names([name.clone()])
        .expect("declared effect must be valid");

    let policy = EffectPolicy::permissive();

    let result = policy.validate_with_registry(
        &declared,
        |requested| {
            if requested == definition.name() {
                Some(definition.clone())
            } else {
                None
            }
        },
    );

    assert!(result.is_ok());
}

#[test]
fn registry_backed_validation_rejects_a_mismatched_definition() {
    let requested = effect("extension_example::requested");
    let different = EffectDefinition::new(
        effect("extension_example::different"),
        EffectClass::Extension,
        EffectProperties::externally_observable(),
    )
    .expect("test definition must be valid");

    let declared = EffectSet::from_names([requested])
        .expect("declared effect must be valid");

    let result = EffectPolicy::permissive().validate_with_registry(
        &declared,
        |_| Some(different.clone()),
    );

    assert!(matches!(
        result,
        Err(EffectError::UnknownEffect { .. })
    ));
}

// -----------------------------------------------------------------------------
// Operation-level integration
// -----------------------------------------------------------------------------

#[test]
fn operation_effect_declarations_are_validated() {
    let operation = operation_with_effects(
        1,
        &[
            "core::io",
            "quantum::measurement",
        ],
    );

    assert!(operation.validate().is_ok());
    assert!(validate_operation_effects(&operation).is_ok());
}

#[test]
fn operation_effects_are_deduplicated() {
    let mut operation = operation(2);

    assert!(
        operation
            .add_effect(effect("core::io"))
            .expect("effect must be valid")
    );

    assert!(
        !operation
            .add_effect(effect("core::io"))
            .expect("duplicate effect name is valid")
    );

    assert_eq!(operation.effects().len(), 1);
}

#[test]
fn declared_operation_effects_are_not_claimed_complete() {
    let operation = operation_with_effects(
        3,
        &["core::io"],
    );

    let summary = declared_operation_effects(&operation)
        .expect("valid operation effects must be convertible");

    assert_eq!(
        summary.completeness(),
        EffectCompleteness::Partial
    );
    assert!(
        summary.is_proven_pure().is_err(),
        "an operation's declarations alone must not prove completeness"
    );
}

#[test]
fn operation_effect_set_replacement_is_supported() {
    let mut operation = operation_with_effects(
        4,
        &["core::io"],
    );

    let replacement = BTreeSet::from([
        effect("quantum::measurement"),
        effect("distributed::message"),
    ]);

    operation
        .set_effects(replacement)
        .expect("valid replacement effect set must be accepted");

    assert_eq!(operation.effects().len(), 2);
    assert!(!operation.effects().contains(&effect("core::io")));
    assert!(
        operation
            .effects()
            .contains(&effect("quantum::measurement"))
    );
}

// -----------------------------------------------------------------------------
// Effect analysis integration
// -----------------------------------------------------------------------------

#[test]
fn analysis_aggregates_operation_effects_deterministically() {
    let first = operation_with_effects(
        10,
        &[
            "core::io",
            "core::memory_read",
        ],
    );

    let second = operation_with_effects(
        11,
        &[
            "core::io",
            "quantum::measurement",
        ],
    );

    let analysis = EffectAnalysis::analyze([&second, &first])
        .expect("unique operations with valid effects must analyze");

    assert_eq!(analysis.operation_count(), 2);

    let aggregate = analysis.aggregate();

    assert_eq!(
        aggregate.completeness(),
        EffectCompleteness::Partial
    );

    assert_eq!(
        aggregate.effects().len(),
        3,
        "the aggregate must union effects and remove duplicates"
    );

    assert!(
        aggregate
            .effects()
            .contains(&effect("core::io"))
    );
    assert!(
        aggregate
            .effects()
            .contains(&effect("core::memory_read"))
    );
    assert!(
        aggregate
            .effects()
            .contains(&effect("quantum::measurement"))
    );
}

#[test]
fn analysis_does_not_promote_an_empty_scope_to_complete() {
    let analysis = EffectAnalysis::analyze(
        std::iter::empty::<&ZuirOperation>(),
    )
    .expect("empty analysis scope is valid");

    assert!(analysis.is_empty());
    assert_eq!(analysis.operation_count(), 0);

    assert_eq!(
        analysis.aggregate().completeness(),
        EffectCompleteness::Partial
    );

    assert!(
        analysis.aggregate().is_proven_pure().is_err(),
        "empty analysis input must not establish completeness"
    );
}

#[test]
fn analysis_rejects_duplicate_operation_ids() {
    let first = operation_with_effects(20, &["core::io"]);
    let second = operation_with_effects(
        20,
        &["quantum::measurement"],
    );

    assert!(matches!(
        EffectAnalysis::analyze([&first, &second]),
        Err(EffectAnalysisError::DuplicateOperation { .. })
    ));
}

#[test]
fn analysis_accepts_an_empty_declared_upper_bound_for_an_empty_scope() {
    let analysis = EffectAnalysis::analyze(
        std::iter::empty::<&ZuirOperation>(),
    )
    .expect("empty analysis scope is valid");

    assert!(
        analysis
            .check_declared_upper_bound(&EffectSet::new())
            .is_ok()
    );
}

#[test]
fn analysis_rejects_effects_missing_from_the_declared_upper_bound() {
    let operation = operation_with_effects(
        21,
        &[
            "core::io",
            "quantum::measurement",
        ],
    );

    let analysis = EffectAnalysis::analyze([&operation])
        .expect("valid operation effects must analyze");

    let declared = effect_set(&["core::io"]);

    assert!(matches!(
        analysis.check_declared_upper_bound(&declared),
        Err(EffectAnalysisError::UndeclaredEffect { .. })
    ));
}

#[test]
fn analysis_accepts_a_declared_upper_bound_containing_all_observed_effects() {
    let operation = operation_with_effects(
        22,
        &[
            "core::io",
            "quantum::measurement",
        ],
    );

    let analysis = EffectAnalysis::analyze([&operation])
        .expect("valid operation effects must analyze");

    let declared = effect_set(&[
        "core::io",
        "quantum::measurement",
        "distributed::message",
    ]);

    assert!(
        analysis
            .check_declared_upper_bound(&declared)
            .is_ok()
    );
}

#[test]
fn convenience_analysis_preserves_partial_completeness() {
    let operation = operation_with_effects(
        23,
        &["core::io"],
    );

    let summary = analyze_effects([&operation])
        .expect("valid operation effects must analyze");

    assert_eq!(
        summary.completeness(),
        EffectCompleteness::Partial
    );
}

// -----------------------------------------------------------------------------
// Serialization compatibility
// -----------------------------------------------------------------------------

#[test]
fn effect_set_json_round_trip_preserves_members() {
    let original = effect_set(&[
        "core::io",
        "quantum::measurement",
        "distributed::message",
    ]);

    let encoded = serde_json::to_string(&original)
        .expect("effect set must serialize");

    let decoded: EffectSet = serde_json::from_str(&encoded)
        .expect("serialized effect set must deserialize");

    assert_eq!(decoded, original);
    assert!(decoded.validate().is_ok());
}

#[test]
fn effect_summary_json_round_trip_preserves_completeness() {
    let original = EffectSummary::new(
        effect_set(&[
            "core::io",
            "quantum::measurement",
        ]),
        EffectCompleteness::Partial,
    );

    let encoded = serde_json::to_string(&original)
        .expect("effect summary must serialize");

    let decoded: EffectSummary = serde_json::from_str(&encoded)
        .expect("serialized effect summary must deserialize");

    assert_eq!(decoded, original);
    assert_eq!(
        decoded.completeness(),
        EffectCompleteness::Partial
    );
}

#[test]
fn effect_definition_json_round_trip_preserves_metadata() {
    let original = EffectDefinition::new(
        effect("quantum::measurement"),
        EffectClass::Measurement,
        EffectProperties::externally_observable(),
    )
    .expect("definition must be valid");

    let encoded = serde_json::to_string(&original)
        .expect("effect definition must serialize");

    let decoded: EffectDefinition = serde_json::from_str(&encoded)
        .expect("effect definition must deserialize");

    assert_eq!(decoded, original);
    assert!(decoded.validate().is_ok());
}

#[test]
fn effect_policy_json_round_trip_preserves_rules() {
    let mut original = EffectPolicy::allow_only(effect_set(&[
        "core::io",
        "core::memory_read",
    ]));

    original
        .forbid(effect("core::io"))
        .expect("forbidden effect must be valid");

    original
        .require(effect("core::memory_read"))
        .expect("required effect must be valid");

    let encoded = serde_json::to_string(&original)
        .expect("effect policy must serialize");

    let decoded: EffectPolicy = serde_json::from_str(&encoded)
        .expect("effect policy must deserialize");

    assert_eq!(decoded, original);
}

// -----------------------------------------------------------------------------
// Dynamic collection and determinism checks
// -----------------------------------------------------------------------------

#[test]
fn effect_set_handles_a_dynamically_sized_collection() {
    // This is a regression test for accidental small fixed-size assumptions,
    // not a claim that any finite process has infinite resources.
    let count = 2_048_usize;

    let names = (0..count).map(|index| {
        QualifiedName::new(vec![
            String::from("stress"),
            format!("effect_{index}"),
        ])
        .expect("generated effect name must be valid")
    });

    let set = EffectSet::from_names(names)
        .expect("dynamic effect collection must be accepted");

    assert_eq!(set.len(), count);
    assert!(set.validate().is_ok());
}

#[test]
fn effect_aggregation_is_independent_of_input_order() {
    let first = operation_with_effects(
        30,
        &["core::io", "core::memory_read"],
    );

    let second = operation_with_effects(
        31,
        &["quantum::measurement"],
    );

    let forward = EffectAnalysis::analyze([&first, &second])
        .expect("forward analysis must succeed");

    let reverse = EffectAnalysis::analyze([&second, &first])
        .expect("reverse analysis must succeed");

    assert_eq!(
        forward.aggregate().effects(),
        reverse.aggregate().effects()
    );

    assert_eq!(
        forward.aggregate().completeness(),
        reverse.aggregate().completeness()
    );
}