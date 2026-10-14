#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani semantic capability conformance tests.
//!
//! Path: src/frontend/semantic/tests/capabilities.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97+
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Verify that the semantic capability subsystem correctly integrates:
//!
//! - canonical native AST capability nodes;
//! - semantic capability identities and version constraints;
//! - AST-to-semantic requirement conversion;
//! - source-node and source-span provenance;
//! - scoped capability evidence;
//! - capability version and property compatibility;
//! - missing, unknown, contradictory, and incompatible evidence;
//! - deterministic requirement and evidence processing;
//! - open-world capability names and namespaces;
//! - scalable collections without language-level hardware ceilings.
//!
//! AUTHORITY
//!
//! Syntax and AST structure:
//!   grammar/Zamani.g4
//!   grammar/antlr/ZamaniParser.g4
//!   grammar/core/capabilities.g4
//!   grammar/resources/capabilities.g4
//!
//! Semantic contracts:
//!   grammar/specification/semantic-model.md
//!   grammar/specification/grammar-authority.md
//!   grammar/specification/scalability-model.md
//!   grammar/specification/poco-reaf.md
//!
//! Implementation contracts:
//!   src/frontend/ast/node/capabilities/capability.rs
//!   src/frontend/ast/node/node_id.rs
//!   src/frontend/ast/node/source/span.rs
//!   src/frontend/semantic/capabilities/model.rs
//!   src/frontend/semantic/capabilities/requirements.rs
//!   src/frontend/semantic/capabilities/evidence.rs
//!   src/frontend/semantic/capabilities/compatibility.rs
//!
//! OWNERSHIP
//!
//! This file owns tests and test fixtures only. It must not define a second
//! semantic capability model, duplicate production algorithms, or modify
//! production state.
//!
//! IMPORTANT DISTINCTIONS
//!
//! 1. A source-level requirement does not prove that a target supports it.
//! 2. Missing evidence is not proof of incompatibility.
//! 3. Unknown or incomplete evidence cannot establish satisfaction of a
//!    requirement that needs information which is unavailable.
//! 4. Evidence applies only to its explicitly identified subject.
//! 5. Conflicting claims must not silently overwrite one another.
//! 6. Capability compatibility is not source-language validity.
//! 7. Capability support does not establish resource sufficiency.
//! 8. No test assumes that a finite test collection defines a language limit.
//!
//! INTEGRATION
//!
//! Include this module from the semantic test module tree. These tests call
//! the existing public capability APIs and intentionally do not depend on
//! private implementation details.
//!
//! These tests exercise the semantic capability API. They do not claim that
//! the complete ANTLR grammar, parser, or full frontend pipeline has been
//! executed by these tests.
//!
//! SCALABILITY
//!
//! No test establishes a maximum legal number of capabilities. The collection
//! sizes used below are workloads for exercising behavior, not language
//! limits. Real compilation remains bounded by available memory, processing
//! time, representational limits, and explicit operational policies.

use std::collections::BTreeMap;

use crate::frontend::ast::node::capabilities::capability::{
    Capability as AstCapability,
    CapabilityVersion as AstCapabilityVersion,
    CapabilityVersionConstraint as AstVersionConstraint,
};
use crate::frontend::ast::node::metadata::NodeMetadata;
use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::source::span::{SourceId, Span};

use crate::frontend::semantic::capabilities::compatibility::{
    evaluate_compatibility,
    CapabilityCompatibilityStatus,
};
use crate::frontend::semantic::capabilities::evidence::{
    CapabilityEvidence,
    CapabilityEvidenceResult,
    CapabilityEvidenceSet,
    CapabilityEvidenceStatus,
    EvidenceOrigin,
    EvidenceSubject,
};
use crate::frontend::semantic::capabilities::model::{
    CapabilityId,
    CapabilityPropertyValue,
    CapabilityRequirement,
    CapabilityVersion,
    CapabilityVersionConstraint,
};
use crate::frontend::semantic::capabilities::requirements::{
    CapabilityRequirementError,
    CapabilityRequirements,
};

/// Stable test scope. This identifies a logical environment profile, not a
/// particular physical device.
const TEST_SUBJECT: &str = "test.execution_profile";

fn subject() -> EvidenceSubject {
    EvidenceSubject::new(TEST_SUBJECT).expect("test subject must be valid")
}

fn other_subject() -> EvidenceSubject {
    EvidenceSubject::new("test.other_execution_profile")
        .expect("other test subject must be valid")
}

fn origin(issuer: &str) -> EvidenceOrigin {
    EvidenceOrigin::new("test.fixture", issuer, None)
        .expect("test evidence origin must be valid")
}

fn capability_id(namespace: &str, name: &str) -> CapabilityId {
    CapabilityId::new(namespace, name)
        .expect("test capability identity must be valid")
}

fn quantum_capability_id() -> CapabilityId {
    capability_id("zamani.quantum", "dynamic_control")
}

fn ast_capability(
    node_id: u64,
    namespace: &str,
    name: &str,
    version: AstVersionConstraint,
) -> AstCapability {
    let id = NodeId::new(node_id).expect("test node ID must be nonzero");

    let start = usize::try_from(node_id)
        .expect("test node ID must fit source coordinates");

    let end = start
        .checked_add(1)
        .expect("test source-span end must not overflow");

    let span = Span::from_usize(SourceId::from_raw(1), start, end)
        .expect("test source span must be valid");

    AstCapability::with_version(
        id,
        span,
        NodeMetadata::default(),
        namespace,
        name,
        version,
    )
    .expect("test AST capability must be valid")
}

fn any_version_capability(
    node_id: u64,
    namespace: &str,
    name: &str,
) -> AstCapability {
    ast_capability(node_id, namespace, name, AstVersionConstraint::Any)
}

fn supported_evidence(
    identity: CapabilityId,
    scope: EvidenceSubject,
    version: Option<CapabilityVersion>,
    issuer: &str,
) -> CapabilityEvidence {
    CapabilityEvidence::new(
        identity,
        scope,
        CapabilityEvidenceStatus::Supported,
        version,
        BTreeMap::new(),
        origin(issuer),
    )
    .expect("test support evidence must be valid")
}

fn evidence_set(
    records: impl IntoIterator<Item = CapabilityEvidence>,
) -> CapabilityEvidenceSet {
    let mut set = CapabilityEvidenceSet::new();
    set.extend(records)
        .expect("test evidence records must be valid");
    set
}

fn requirements_from(
    capabilities: &[AstCapability],
) -> CapabilityRequirements {
    CapabilityRequirements::from_ast(capabilities.iter())
        .expect("test AST capabilities must convert successfully")
}

// =============================================================================
// Identity and extensibility
// =============================================================================

#[test]
fn capability_identity_is_namespace_qualified() {
    let quantum = capability_id("zamani.quantum", "compute");
    let classical = capability_id("zamani.classical", "compute");

    assert_ne!(quantum, classical);
    assert_eq!(quantum.namespace(), "zamani.quantum");
    assert_eq!(quantum.name(), "compute");
    assert_eq!(quantum.qualified_name(), "zamani.quantum.compute");
    assert!(quantum.matches("zamani.quantum", "compute"));
    assert!(!quantum.matches("zamani.classical", "compute"));
}

#[test]
fn future_domains_and_unicode_names_do_not_require_a_closed_enum() {
    let future = capability_id("future.computing.domain", "new_capability");
    let unicode = capability_id("future.计算", "能力");

    assert_eq!(
        future.qualified_name(),
        "future.computing.domain.new_capability"
    );
    assert_eq!(unicode.namespace(), "future.计算");
    assert_eq!(unicode.name(), "能力");
}

#[test]
fn invalid_capability_identity_components_are_rejected() {
    assert!(CapabilityId::new("", "compute").is_err());
    assert!(CapabilityId::new("namespace", "").is_err());
    assert!(CapabilityId::new("namespace\ninvalid", "compute").is_err());
    assert!(CapabilityId::new("namespace", "compute\ninvalid").is_err());
}

// =============================================================================
// Version constraints
// =============================================================================

#[test]
fn semantic_version_constraints_obey_inclusive_boundaries() {
    let v1 = CapabilityVersion::new(1, 0, 0);
    let v2 = CapabilityVersion::new(2, 0, 0);
    let v3 = CapabilityVersion::new(3, 0, 0);

    assert!(CapabilityVersionConstraint::Any.matches(v2));

    assert!(CapabilityVersionConstraint::Exact(v2).matches(v2));
    assert!(!CapabilityVersionConstraint::Exact(v2).matches(v1));

    assert!(CapabilityVersionConstraint::AtLeast(v2).matches(v2));
    assert!(CapabilityVersionConstraint::AtLeast(v2).matches(v3));
    assert!(!CapabilityVersionConstraint::AtLeast(v2).matches(v1));

    assert!(CapabilityVersionConstraint::AtMost(v2).matches(v1));
    assert!(CapabilityVersionConstraint::AtMost(v2).matches(v2));
    assert!(!CapabilityVersionConstraint::AtMost(v2).matches(v3));

    let interval = CapabilityVersionConstraint::between(v1, v3)
        .expect("ordered interval must be valid");

    assert!(interval.matches(v1));
    assert!(interval.matches(v2));
    assert!(interval.matches(v3));
}

#[test]
fn reversed_version_intervals_are_rejected() {
    let minimum = CapabilityVersion::new(3, 0, 0);
    let maximum = CapabilityVersion::new(1, 0, 0);

    assert!(CapabilityVersionConstraint::between(minimum, maximum).is_err());
}

// =============================================================================
// AST-to-semantic integration and provenance
// =============================================================================

#[test]
fn ast_conversion_preserves_identity_version_and_source_span() {
    let version = AstCapabilityVersion::new(2, 3, 4);
    let ast = ast_capability(
        7,
        "zamani.quantum",
        "dynamic_control",
        AstVersionConstraint::AtLeast(version),
    );

    let requirements = CapabilityRequirements::from_ast([&ast])
        .expect("valid AST capability must convert");

    let node_id = NodeId::new(7).expect("valid node ID");
    let record = requirements
        .get(node_id)
        .expect("converted requirement must retain its source node");

    assert_eq!(
        record.requirement().capability(),
        &quantum_capability_id()
    );

    assert_eq!(
        record.requirement().version_constraint(),
        CapabilityVersionConstraint::AtLeast(CapabilityVersion::new(2, 3, 4))
    );

    assert_eq!(record.source_node(), ast.id());
    assert_eq!(record.source_span(), ast.node().span());
}

#[test]
fn every_ast_version_constraint_form_converts_correctly() {
    let v1 = AstCapabilityVersion::new(1, 0, 0);
    let v2 = AstCapabilityVersion::new(2, 0, 0);
    let v3 = AstCapabilityVersion::new(3, 0, 0);

    let cases = [
        (
            AstVersionConstraint::Any,
            CapabilityVersionConstraint::Any,
        ),
        (
            AstVersionConstraint::Exact(v2),
            CapabilityVersionConstraint::Exact(CapabilityVersion::new(
                2, 0, 0,
            )),
        ),
        (
            AstVersionConstraint::AtLeast(v2),
            CapabilityVersionConstraint::AtLeast(CapabilityVersion::new(
                2, 0, 0,
            )),
        ),
        (
            AstVersionConstraint::AtMost(v2),
            CapabilityVersionConstraint::AtMost(CapabilityVersion::new(
                2, 0, 0,
            )),
        ),
        (
            AstVersionConstraint::Between {
                minimum: v1,
                maximum: v3,
            },
            CapabilityVersionConstraint::Between {
                minimum: CapabilityVersion::new(1, 0, 0),
                maximum: CapabilityVersion::new(3, 0, 0),
            },
        ),
    ];

    for (index, (source_constraint, expected)) in cases.into_iter().enumerate() {
        let node_id =
            u64::try_from(index + 1).expect("small test index must fit u64");

        let ast = ast_capability(
            node_id,
            "zamani.quantum",
            "dynamic_control",
            source_constraint,
        );

        let requirements = CapabilityRequirements::from_ast([&ast])
            .expect("valid version constraint must convert");

        let record = requirements
            .get(NodeId::new(node_id).expect("valid node ID"))
            .expect("requirement must exist");

        assert_eq!(record.requirement().version_constraint(), expected);
    }
}

#[test]
fn repeated_identical_source_node_insertion_is_idempotent() {
    let ast = any_version_capability(11, "zamani.quantum", "dynamic_control");
    let mut requirements = CapabilityRequirements::new();

    requirements
        .insert_ast(&ast)
        .expect("initial insertion must succeed");

    requirements
        .insert_ast(&ast)
        .expect("identical insertion must be idempotent");

    assert_eq!(requirements.len(), 1);
}

#[test]
fn conflicting_reuse_of_a_source_node_is_rejected_transactionally() {
    let original =
        any_version_capability(21, "zamani.quantum", "dynamic_control");

    let additional =
        any_version_capability(22, "zamani.classical", "parallel_execution");

    let conflicting = ast_capability(
        21,
        "zamani.quantum",
        "dynamic_control",
        AstVersionConstraint::Exact(AstCapabilityVersion::new(2, 0, 0)),
    );

    let mut requirements = CapabilityRequirements::new();

    requirements
        .insert_ast(&original)
        .expect("initial insertion must succeed");

    let before = requirements.clone();

    let result = requirements.extend_ast([&additional, &conflicting]);

    assert!(matches!(
        result,
        Err(CapabilityRequirementError::ConflictingSourceNode {
            source_node
        }) if source_node.get() == 21
    ));

    assert_eq!(
        requirements, before,
        "failed batch insertion must not partially mutate requirements"
    );
}

#[test]
fn requirements_for_the_same_capability_keep_distinct_source_nodes() {
    let first =
        any_version_capability(31, "zamani.quantum", "dynamic_control");

    let second =
        any_version_capability(32, "zamani.quantum", "dynamic_control");

    let requirements = requirements_from(&[first, second]);

    assert_eq!(requirements.len(), 2);
    assert_eq!(
        requirements
            .for_capability(&quantum_capability_id())
            .count(),
        2
    );
}

#[test]
fn requirement_iteration_is_deterministic_by_source_node_identity() {
    let first =
        any_version_capability(90, "zamani.quantum", "dynamic_control");

    let second =
        any_version_capability(20, "zamani.classical", "parallel_execution");

    let third =
        any_version_capability(50, "zamani.tensor", "compute");

    let forward = requirements_from(&[first.clone(), second.clone(), third.clone()]);
    let reverse = requirements_from(&[third, second, first]);

    let forward_ids: Vec<u64> = forward
        .iter()
        .map(|record| record.source_node().get())
        .collect();

    let reverse_ids: Vec<u64> = reverse
        .iter()
        .map(|record| record.source_node().get())
        .collect();

    assert_eq!(forward_ids, vec![20, 50, 90]);
    assert_eq!(forward_ids, reverse_ids);
}

// =============================================================================
// Evidence scope, missing evidence, and conservative evaluation
// =============================================================================

#[test]
fn missing_evidence_never_proves_compatibility() {
    let ast =
        any_version_capability(41, "zamani.quantum", "dynamic_control");

    let requirements = CapabilityRequirements::from_ast([&ast])
        .expect("requirements must convert");

    let evidence = CapabilityEvidenceSet::new();

    let report = evaluate_compatibility(&requirements, &subject(), &evidence)
        .expect("empty evidence is a valid input");

    assert_eq!(report.len(), 1);
    assert_eq!(
        report.iter().next().expect("one result").status(),
        CapabilityCompatibilityStatus::NoEvidence
    );
    assert!(!report.all_satisfied());
    assert!(report.summary().has_unresolved_requirements());
}

#[test]
fn evidence_from_another_scope_does_not_satisfy_a_requirement() {
    let ast =
        any_version_capability(42, "zamani.quantum", "dynamic_control");

    let requirements = CapabilityRequirements::from_ast([&ast])
        .expect("requirements must convert");

    let evidence = evidence_set([supported_evidence(
        quantum_capability_id(),
        other_subject(),
        None,
        "other-profile-provider",
    )]);

    let report = evaluate_compatibility(&requirements, &subject(), &evidence)
        .expect("valid evidence must be evaluated");

    assert_eq!(
        report.iter().next().expect("one result").status(),
        CapabilityCompatibilityStatus::NoEvidence
    );
    assert!(!report.all_satisfied());
}

#[test]
fn unknown_evidence_remains_indeterminate() {
    let identity = quantum_capability_id();
    let scope = subject();

    let record = CapabilityEvidence::new(
        identity.clone(),
        scope.clone(),
        CapabilityEvidenceStatus::Unknown,
        None,
        BTreeMap::new(),
        origin("unknown-provider"),
    )
    .expect("unknown evidence is a valid claim");

    let mut evidence = CapabilityEvidenceSet::new();
    evidence.insert(record).expect("evidence insertion must succeed");

    let requirement = CapabilityRequirement::new(identity);

    assert_eq!(
        evidence.evaluate(&scope, &requirement),
        CapabilityEvidenceResult::Indeterminate
    );
}

#[test]
fn contradictory_evidence_is_not_silently_overwritten() {
    let identity = quantum_capability_id();
    let scope = subject();

    let supported = supported_evidence(
        identity.clone(),
        scope.clone(),
        None,
        "support-provider",
    );

    let unsupported = CapabilityEvidence::new(
        identity.clone(),
        scope.clone(),
        CapabilityEvidenceStatus::Unsupported,
        None,
        BTreeMap::new(),
        origin("non-support-provider"),
    )
    .expect("non-support evidence must be valid");

    let mut evidence = CapabilityEvidenceSet::new();

    evidence
        .insert(supported)
        .expect("support evidence must be accepted");

    evidence
        .insert(unsupported)
        .expect("contradictory evidence must remain representable");

    assert_eq!(evidence.len(), 2);
    assert_eq!(
        evidence.evaluate(&scope, &CapabilityRequirement::new(identity)),
        CapabilityEvidenceResult::ConflictingEvidence
    );
}

// =============================================================================
// Version and property compatibility
// =============================================================================

#[test]
fn incompatible_version_evidence_is_reported_as_incompatible() {
    let ast = ast_capability(
        51,
        "zamani.quantum",
        "dynamic_control",
        AstVersionConstraint::AtLeast(AstCapabilityVersion::new(2, 0, 0)),
    );

    let requirements = CapabilityRequirements::from_ast([&ast])
        .expect("requirements must convert");

    let evidence = evidence_set([supported_evidence(
        quantum_capability_id(),
        subject(),
        Some(CapabilityVersion::new(1, 9, 9)),
        "version-provider",
    )]);

    let report = evaluate_compatibility(&requirements, &subject(), &evidence)
        .expect("valid evidence must be evaluated");

    assert_eq!(
        report.iter().next().expect("one result").status(),
        CapabilityCompatibilityStatus::Incompatible
    );

    assert!(report.summary().has_incompatibilities());
    assert!(!report.all_satisfied());
}

#[test]
fn missing_required_version_does_not_assume_a_version() {
    let identity = quantum_capability_id();
    let scope = subject();

    let evidence = evidence_set([supported_evidence(
        identity.clone(),
        scope.clone(),
        None,
        "versionless-provider",
    )]);

    let requirement = CapabilityRequirement::with_version(
        identity,
        CapabilityVersionConstraint::AtLeast(CapabilityVersion::new(
            2, 0, 0,
        )),
    );

    assert_eq!(
        evidence.evaluate(&scope, &requirement),
        crate::frontend::semantic::capabilities::evidence::CapabilityEvidenceResult::Indeterminate
    );
}

#[test]
fn matching_property_evidence_satisfies_a_property_requirement() {
    let identity = quantum_capability_id();
    let scope = subject();

    let mut properties = BTreeMap::new();

    properties.insert(
        "measurement.mode".to_owned(),
        CapabilityPropertyValue::Text("mid_circuit".to_owned()),
    );

    let record = CapabilityEvidence::new(
        identity.clone(),
        scope.clone(),
        CapabilityEvidenceStatus::Supported,
        None,
        properties,
        origin("property-provider"),
    )
    .expect("valid property evidence must be constructible");

    let mut evidence = CapabilityEvidenceSet::new();
    evidence.insert(record).expect("evidence insertion must succeed");

    let mut requirement =
        CapabilityRequirement::new(identity);

    requirement
        .add_property(
            "measurement.mode",
            CapabilityPropertyValue::Text("mid_circuit".to_owned()),
        )
        .expect("valid property requirement must be accepted");

    assert_eq!(
        evidence.evaluate(&scope, &requirement),
        CapabilityEvidenceResult::Satisfied
    );
}

#[test]
fn missing_required_property_is_indeterminate() {
    let identity = quantum_capability_id();
    let scope = subject();

    let evidence = evidence_set([supported_evidence(
        identity.clone(),
        scope.clone(),
        None,
        "propertyless-provider",
    )]);

    let mut requirement = CapabilityRequirement::new(identity);

    requirement
        .add_property(
            "measurement.mode",
            CapabilityPropertyValue::Text("mid_circuit".to_owned()),
        )
        .expect("valid property requirement must be accepted");

    assert_eq!(
        evidence.evaluate(&scope, &requirement),
        CapabilityEvidenceResult::Indeterminate
    );
}

// =============================================================================
// Determinism and scalable collection behavior
// =============================================================================

#[test]
fn evidence_collection_is_deterministic_regardless_of_insertion_order() {
    let first = supported_evidence(
        quantum_capability_id(),
        subject(),
        Some(CapabilityVersion::new(1, 0, 0)),
        "provider-a",
    );

    let second = supported_evidence(
        quantum_capability_id(),
        subject(),
        Some(CapabilityVersion::new(2, 0, 0)),
        "provider-b",
    );

    let left = evidence_set([first.clone(), second.clone()]);
    let right = evidence_set([second, first]);

    assert_eq!(
        left.iter().cloned().collect::<Vec<_>>(),
        right.iter().cloned().collect::<Vec<_>>()
    );
}

#[test]
fn capability_collection_scales_with_input_without_hardware_limits() {
    // This is a test workload, not a language-level maximum.
    // No machine, device, qubit, register, or processor count is implied.
    let workload_size: u64 = 512;

    let mut capabilities = Vec::new();

    for node_id in 1..=workload_size {
        let name = format!("capability_{node_id}");

        capabilities.push(any_version_capability(
            node_id,
            "future.extensible.domain",
            &name,
        ));
    }

    let requirements = CapabilityRequirements::from_ast(capabilities.iter())
        .expect("the configured test workload must be accepted");

    assert_eq!(
        requirements.len(),
        usize::try_from(workload_size).expect("test size fits usize")
    );

    for node_id in 1..=workload_size {
        let id = NodeId::new(node_id).expect("nonzero node ID");

        assert!(
            requirements.get(id).is_some(),
            "requirement for source node {node_id} must be retained"
        );
    }
}

#[test]
fn compatibility_report_preserves_source_provenance() {
    let ast =
        any_version_capability(61, "zamani.quantum", "dynamic_control");

    let expected_span = *ast.node().span();
    let expected_node = ast.id();

    let requirements = CapabilityRequirements::from_ast([&ast])
        .expect("requirements must convert");

    let evidence = evidence_set([supported_evidence(
        quantum_capability_id(),
        subject(),
        None,
        "provenance-provider",
    )]);

    let report = evaluate_compatibility(&requirements, &subject(), &evidence)
        .expect("valid evidence must be evaluated");

    let result = report
        .get(expected_node)
        .expect("report must index the source node");

    assert_eq!(result.source_node(), expected_node);
    assert_eq!(result.source_span(), &expected_span);
    assert_eq!(
        result.status(),
        CapabilityCompatibilityStatus::Satisfied
    );
}

#[test]
fn empty_requirement_report_does_not_claim_target_support() {
    let requirements = CapabilityRequirements::new();
    let evidence = CapabilityEvidenceSet::new();

    let report = evaluate_compatibility(&requirements, &subject(), &evidence)
        .expect("empty inputs are valid");

    assert!(report.is_empty());
    assert_eq!(report.summary().total(), 0);
    assert!(!report.all_satisfied());
}