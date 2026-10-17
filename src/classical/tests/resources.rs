
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Classical Resource Integration Tests
//!
//! File: `src/classical/tests/resources.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verify that classical execution requirements correctly integrate with
//! Zamani's canonical frontend semantic resource and capability models.
//!
//! ## Authority
//!
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/execution-model.md`
//! - `grammar/spec/resources.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/frontend/semantic/resources/model.rs`
//! - `src/frontend/semantic/capabilities/model.rs`
//! - `src/classical/execution/requirements.rs`
//!
//! The grammar specifies syntax; semantic models define resource and
//! capability meaning. These tests verify the integration boundary without
//! introducing a duplicate resource representation.
//!
//! ## Ownership
//!
//! This file owns tests only. It does not define production resource types,
//! resource inventories, hardware limits, target feasibility, allocation,
//! placement, scheduling, or runtime execution.
//!
//! ## Scalability contract
//!
//! Test workloads are generated from their input parameters. Test iteration
//! counts are test workloads, not language or implementation ceilings.
//! The implementation must not impose a universal maximum resource count,
//! capability count, quantity, machine size, or program size.
//!
//! Large decimal quantities must not be narrowed to machine-sized integers.
//! Symbolic and runtime-derived quantities must retain their semantic
//! identities until their designated evaluation service handles them.
//!
//! ## Integration
//!
//! Register this file through `src/classical/tests/mod.rs` and the
//! classical module's test-only module declaration. See the integration
//! instructions accompanying this file.
//!
//! These tests require the canonical frontend semantic models and the
//! classical execution-requirements module to be exposed by the crate.

use crate::classical::execution::requirements::{
    CapabilityRequirementEntry,
    ExecutionRequirements,
    ExecutionRequirementsError,
    RequirementIssue,
    RequirementIssueKind,
    RequirementsCompleteness,
};
use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::resources::resource_kind::ResourceKind;
use crate::frontend::semantic::capabilities::model::{
    CapabilityId,
    CapabilityRequirement,
};
use crate::frontend::semantic::resources::model::{
    ConstraintOperand,
    ConstraintOperator,
    ExactQuantity,
    ResourceConstraint,
    ResourceIntent,
    ResourceQuantity,
    ResourceRequirement,
    ResourceUnit,
};

/// Construct a valid extensible resource-kind identity.
///
/// The helper intentionally uses an open namespaced identity rather than
/// relying on a closed list of built-in resource kinds.
fn resource_kind(name: &str) -> ResourceKind {
    ResourceKind::new("zamani.classical", name)
        .expect("test resource-kind identity must be valid")
}

/// Construct an exact, known resource requirement.
fn known_requirement(
    name: &str,
    quantity: &str,
) -> ResourceRequirement {
    let kind = resource_kind(name);
    let quantity = ResourceQuantity::known(quantity)
        .expect("test quantity must be a valid exact decimal");

    ResourceRequirement::new(
        kind,
        quantity,
        None,
        ResourceIntent::Required,
    )
    .expect("test resource requirement must be valid")
}

/// Construct a validated, target-independent capability requirement.
fn capability_requirement(name: &str) -> CapabilityRequirement {
    let id = CapabilityId::new("zamani.classical", name)
        .expect("test capability identity must be valid");

    CapabilityRequirement::new(id)
}

/// Construct a valid AST node identity for provenance tests.
fn source_node(value: u64) -> NodeId {
    NodeId::new(value).expect("test node identity must be nonzero")
}

// ---------------------------------------------------------------------------
// Empty state and completeness
// ---------------------------------------------------------------------------

#[test]
fn empty_requirements_are_complete_without_claiming_feasibility() {
    let requirements = ExecutionRequirements::new();

    assert_eq!(
        requirements.completeness(),
        RequirementsCompleteness::Complete
    );
    assert!(requirements.is_complete());
    assert!(requirements.ensure_complete().is_ok());

    assert_eq!(requirements.resource_requirement_count(), 0);
    assert_eq!(requirements.resource_constraint_count(), 0);
    assert_eq!(requirements.capability_requirement_count(), 0);
    assert_eq!(requirements.issue_count(), 0);

    assert!(requirements.resource_requirements().is_empty());
    assert!(requirements.resource_constraints().is_empty());
    assert!(requirements.capability_requirements().is_empty());
    assert!(requirements.issues().is_empty());
}

// ---------------------------------------------------------------------------
// Canonical resource requirements
// ---------------------------------------------------------------------------

#[test]
fn resource_requirements_preserve_insertion_order_and_duplicates() {
    let first = known_requirement("memory", "1024");
    let second = known_requirement("workers", "8");
    let duplicate = known_requirement("memory", "1024");

    let mut requirements = ExecutionRequirements::new();

    requirements.add_resource_requirement(first.clone());
    requirements.add_resource_requirement(second.clone());
    requirements.add_resource_requirement(duplicate.clone());

    assert_eq!(requirements.resource_requirement_count(), 3);
    assert_eq!(
        requirements.resource_requirements(),
        &[first, second, duplicate]
    );

    // Identical declarations are not silently deduplicated.
    assert!(requirements.is_complete());
}

#[test]
fn resource_quantities_support_values_beyond_machine_integer_widths() {
    // This is test data, not a language-level quantity ceiling.
    let large_quantity = format!("1{}", "0".repeat(300));

    let exact = ExactQuantity::new(&large_quantity)
        .expect("a valid large decimal quantity must be representable");

    assert_eq!(exact.as_str(), large_quantity);
    assert!(!exact.is_zero());

    let requirement = ResourceRequirement::new(
        resource_kind("memory"),
        ResourceQuantity::Known(exact),
        None,
        ResourceIntent::Required,
    )
    .expect("a large exact quantity must remain valid");

    let mut requirements = ExecutionRequirements::new();
    requirements.add_resource_requirement(requirement);

    assert_eq!(requirements.resource_requirement_count(), 1);
    assert!(requirements.ensure_complete().is_ok());
}

#[test]
fn exact_decimal_quantities_are_not_rounded_through_floating_point() {
    let left = ExactQuantity::new("0.1000000000000000000000000000000000001")
        .expect("left quantity must be valid");

    let right = ExactQuantity::new("0.1000000000000000000000000000000000002")
        .expect("right quantity must be valid");

    assert!(left.numeric_cmp(&right).is_lt());
    assert_eq!(
        ExactQuantity::new("00012.3400")
            .expect("valid decimal")
            .as_str(),
        "12.34"
    );
}

// ---------------------------------------------------------------------------
// Resource constraints
// ---------------------------------------------------------------------------

#[test]
fn resource_constraints_preserve_their_semantic_operator_and_operand() {
    let quantity = ExactQuantity::new("4096")
        .expect("constraint quantity must be valid");

    let unit = ResourceUnit::new("si", "byte")
        .expect("resource unit identity must be valid");

    let constraint = ResourceConstraint::new(
        resource_kind("memory"),
        ConstraintOperator::GreaterThanOrEqual,
        ConstraintOperand::Known(quantity),
        Some(unit),
        ResourceIntent::Required,
        Some(source_node(7)),
    )
    .expect("resource constraint must be valid");

    let mut requirements = ExecutionRequirements::new();
    requirements.add_resource_constraint(constraint.clone());

    assert_eq!(requirements.resource_constraint_count(), 1);
    assert_eq!(requirements.resource_constraints(), &[constraint]);
    assert!(requirements.ensure_complete().is_ok());

    // A valid semantic constraint does not prove that a target has enough
    // memory. Feasibility belongs to downstream execution planning.
}

// ---------------------------------------------------------------------------
// Symbolic and runtime-derived quantities
// ---------------------------------------------------------------------------

#[test]
fn symbolic_resource_quantities_are_preserved_without_host_sized_conversion() {
    let node = source_node(41);

    let symbolic_requirement = ResourceRequirement::new(
        resource_kind("workers"),
        ResourceQuantity::Symbolic(node),
        None,
        ResourceIntent::Required,
    )
    .expect("symbolic requirement must be valid");

    let runtime_requirement = ResourceRequirement::new(
        resource_kind("memory"),
        ResourceQuantity::Runtime(source_node(42)),
        None,
        ResourceIntent::Budget,
    )
    .expect("runtime-derived requirement must be valid");

    let mut requirements = ExecutionRequirements::new();
    requirements.add_resource_requirement(symbolic_requirement);
    requirements.add_resource_requirement(runtime_requirement);

    assert_eq!(requirements.resource_requirement_count(), 2);
    assert!(requirements.is_complete());
    assert!(requirements.ensure_complete().is_ok());
}

#[test]
fn symbolic_requirements_are_not_incomplete_merely_because_they_are_unresolved() {
    let requirement = ResourceRequirement::new(
        resource_kind("workers"),
        ResourceQuantity::Symbolic(source_node(51)),
        None,
        ResourceIntent::Required,
    )
    .expect("symbolic requirement must be valid");

    let mut requirements = ExecutionRequirements::new();
    requirements.add_resource_requirement(requirement);

    assert_eq!(
        requirements.completeness(),
        RequirementsCompleteness::Complete
    );
    assert!(requirements.ensure_complete().is_ok());

    // Completeness records preparation failures, not whether a symbolic
    // expression has already been evaluated or a target has been selected.
}

// ---------------------------------------------------------------------------
// Capability requirements and provenance
// ---------------------------------------------------------------------------

#[test]
fn capability_requirements_preserve_source_provenance() {
    let capability = capability_requirement("parallel_execution");
    let node = source_node(73);

    let entry = CapabilityRequirementEntry::new(
        capability.clone(),
        Some(node),
    );

    assert_eq!(entry.requirement(), &capability);
    assert_eq!(entry.source_node(), Some(node));

    let mut requirements = ExecutionRequirements::new();
    requirements.add_capability_requirement(capability, Some(node));

    assert_eq!(requirements.capability_requirement_count(), 1);

    let stored = &requirements.capability_requirements()[0];
    assert_eq!(stored.source_node(), Some(node));
    assert_eq!(
        stored.requirement(),
        requirements.capability_requirements()[0].requirement()
    );
}

#[test]
fn capability_requirements_can_be_added_without_source_node_provenance() {
    let capability = capability_requirement("portable_execution");

    let mut requirements = ExecutionRequirements::new();
    requirements.add_capability_requirement(capability, None);

    assert_eq!(requirements.capability_requirement_count(), 1);
    assert_eq!(
        requirements.capability_requirements()[0].source_node(),
        None
    );
    assert!(requirements.ensure_complete().is_ok());
}

#[test]
fn duplicate_capability_requirements_are_retained() {
    let capability = capability_requirement("parallel_execution");

    let mut requirements = ExecutionRequirements::new();
    requirements.add_capability_requirement(
        capability.clone(),
        Some(source_node(80)),
    );
    requirements.add_capability_requirement(
        capability,
        Some(source_node(81)),
    );

    assert_eq!(requirements.capability_requirement_count(), 2);
    assert_ne!(
        requirements.capability_requirements()[0].source_node(),
        requirements.capability_requirements()[1].source_node()
    );
}

// ---------------------------------------------------------------------------
// Incomplete analysis and diagnostics
// ---------------------------------------------------------------------------

#[test]
fn recording_an_issue_marks_requirements_incomplete() {
    let mut requirements = ExecutionRequirements::new();

    requirements.record_issue(RequirementIssue::new(
        RequirementIssueKind::UnresolvedReference,
        Some(source_node(91)),
        "A resource reference could not be resolved.",
    ));

    assert_eq!(
        requirements.completeness(),
        RequirementsCompleteness::Incomplete
    );
    assert!(!requirements.is_complete());
    assert_eq!(requirements.issue_count(), 1);

    assert_eq!(
        requirements.issues()[0].kind(),
        RequirementIssueKind::UnresolvedReference
    );
    assert_eq!(
        requirements.issues()[0].source_node(),
        Some(source_node(91))
    );
    assert_eq!(
        requirements.issues()[0].code(),
        "classical.requirements.unresolved_reference"
    );

    assert!(matches!(
        requirements.ensure_complete(),
        Err(ExecutionRequirementsError::Incomplete { issue_count: 1 })
    ));
}

#[test]
fn issue_classifications_have_stable_machine_readable_codes() {
    let cases = [
        (
            RequirementIssueKind::InvalidDeclaration,
            "classical.requirements.invalid_declaration",
        ),
        (
            RequirementIssueKind::UnresolvedReference,
            "classical.requirements.unresolved_reference",
        ),
        (
            RequirementIssueKind::AnalysisFailure,
            "classical.requirements.analysis_failure",
        ),
        (
            RequirementIssueKind::BudgetExhausted,
            "classical.requirements.budget_exhausted",
        ),
        (
            RequirementIssueKind::MissingInput,
            "classical.requirements.missing_input",
        ),
        (
            RequirementIssueKind::UnsupportedFeature,
            "classical.requirements.unsupported_feature",
        ),
    ];

    for (kind, expected_code) in cases {
        let issue = RequirementIssue::new(
            kind,
            None,
            "Test diagnostic.",
        );

        assert_eq!(issue.kind(), kind);
        assert_eq!(issue.code(), expected_code);
        assert!(!issue.message().is_empty());
    }
}

// ---------------------------------------------------------------------------
// Merge and deterministic behavior
// ---------------------------------------------------------------------------

#[test]
fn merging_requirement_sets_preserves_entries_and_source_order() {
    let first = known_requirement("memory", "1024");
    let second = known_requirement("workers", "4");
    let third = known_requirement("memory", "2048");

    let mut left = ExecutionRequirements::new();
    left.add_resource_requirement(first.clone());

    let mut right = ExecutionRequirements::new();
    right.add_resource_requirement(second.clone());
    right.add_resource_requirement(third.clone());

    left.append(right);

    assert_eq!(
        left.resource_requirements(),
        &[first, second, third]
    );
    assert_eq!(left.resource_requirement_count(), 3);
    assert!(left.ensure_complete().is_ok());
}

#[test]
fn incompleteness_is_contagious_when_requirement_sets_are_merged() {
    let mut complete = ExecutionRequirements::new();
    complete.add_resource_requirement(known_requirement("memory", "512"));

    let mut incomplete = ExecutionRequirements::new();
    incomplete.record_issue(RequirementIssue::new(
        RequirementIssueKind::AnalysisFailure,
        None,
        "Resource analysis did not finish.",
    ));

    complete.append(incomplete);

    assert!(!complete.is_complete());
    assert_eq!(complete.issue_count(), 1);
    assert!(matches!(
        complete.ensure_complete(),
        Err(ExecutionRequirementsError::Incomplete { issue_count: 1 })
    ));
}

#[test]
fn merging_an_empty_complete_set_does_not_change_existing_requirements() {
    let requirement = known_requirement("memory", "256");
    let mut original = ExecutionRequirements::new();

    original.add_resource_requirement(requirement.clone());
    original.append(ExecutionRequirements::new());

    assert_eq!(original.resource_requirements(), &[requirement]);
    assert!(original.ensure_complete().is_ok());
}

#[test]
fn canonical_resource_order_is_deterministic_and_does_not_mutate_source_order() {
    let first = known_requirement("memory", "1024");
    let second = known_requirement("workers", "8");
    let third = known_requirement("memory", "2048");

    let mut requirements = ExecutionRequirements::new();
    requirements.add_resource_requirement(first.clone());
    requirements.add_resource_requirement(second.clone());
    requirements.add_resource_requirement(third.clone());

    let original = requirements.resource_requirements().to_vec();

    let ordered_a = requirements.resources_in_canonical_order();
    let ordered_b = requirements.resources_in_canonical_order();

    assert_eq!(ordered_a, ordered_b);
    assert_eq!(requirements.resource_requirements(), original);
}

// ---------------------------------------------------------------------------
// Configurable scaling tests
// ---------------------------------------------------------------------------

/// Exercise aggregation with a workload selected by the caller.
///
/// The workload size is not a language limit. Larger values may be used by
/// stress-test configurations when the machine has sufficient resources.
fn exercise_requirement_scaling(workload_size: usize) {
    let mut requirements = ExecutionRequirements::new();

    for index in 0..workload_size {
        let quantity = index.to_string();
        requirements.add_resource_requirement(
            known_requirement("workers", &quantity),
        );
    }

    assert_eq!(
        requirements.resource_requirement_count(),
        workload_size
    );
    assert!(requirements.ensure_complete().is_ok());
}

#[test]
fn aggregation_handles_empty_and_small_workloads() {
    exercise_requirement_scaling(0);
    exercise_requirement_scaling(1);
    exercise_requirement_scaling(2);
}

#[test]
fn aggregation_handles_a_larger_generated_workload() {
    // This is a practical regression-test workload, not a maximum.
    exercise_requirement_scaling(4096);
}

#[test]
fn aggregation_preserves_each_entry_for_mixed_resource_and_capability_workloads() {
    let workload_size = 128;
    let mut requirements = ExecutionRequirements::new();

    for index in 0..workload_size {
        requirements.add_resource_requirement(
            known_requirement("workers", &index.to_string()),
        );

        requirements.add_capability_requirement(
            capability_requirement("portable_execution"),
            Some(source_node(index as u64 + 1)),
        );
    }

    assert_eq!(
        requirements.resource_requirement_count(),
        workload_size
    );
    assert_eq!(
        requirements.capability_requirement_count(),
        workload_size
    );
    assert_eq!(requirements.issue_count(), 0);
    assert!(requirements.ensure_complete().is_ok());
}
