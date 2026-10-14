
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Hybrid semantic integration and conformance tests.
//!
//! Path: `src/frontend/semantic/tests/hybrid.rs`
//!
//! PURPOSE
//!
//! Verify the public hybrid semantic contract across classical, quantum,
//! HDL, and extensible computational domains. These tests exercise structural
//! integrity, type-check delegation, domain transitions, synchronization,
//! dependency cycles, diagnostic determinism, incomplete analysis, and
//! resource-scalable input sizes.
//!
//! AUTHORITY
//!
//! - `grammar/hybrid/hybrid.g4`
//! - `grammar/hybrid/classical-quantum.g4`
//! - `grammar/hybrid/classical-quantum-boundary.g4`
//! - `grammar/hybrid/quantum-classical-control.g4`
//! - `grammar/hybrid/hybrid-resources.g4`
//! - `grammar/quantum/mid-circuit-control.g4`
//! - `src/frontend/semantic/domains/hybrid.rs`
//! - `src/frontend/semantic/domains/quantum.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/frontend/ast/node/`
//!
//! OWNERSHIP
//!
//! This file owns test fixtures and test assertions only. It does not define
//! production semantic rules, a competing AST, a type system, a quantum IR,
//! target planning, or hardware-specific limits.
//!
//! SCALABILITY
//!
//! The stress test uses a test-selected workload, not a language limit.
//! No maximum node, value, domain, or dependency count is specified by these
//! tests. Larger stress workloads belong in separately configured CI jobs.
//!
//! INTEGRATION
//!
//! Register this file as `mod hybrid;` in the semantic test module.
//! Production domain validation remains owned by `domains/hybrid.rs`.
//! Quantum, HDL, classical, type, effect, capability, resource, and diagnostic
//! services remain authoritative for their own semantics.

use std::collections::BTreeSet;

use crate::frontend::semantic::domains::hybrid::{
    validate_hybrid, HybridCompletion, HybridContextFailure, HybridDependency,
    HybridDependencyKind, HybridDomainId, HybridEntityId, HybridInputError,
    HybridIssueKind, HybridIssueSeverity, HybridNode, HybridNodeKind,
    HybridOperationId, HybridProgram, HybridSemanticContext, HybridValue,
    HybridValueInput,
};

/// A controllable fake of the shared semantic services.
///
/// This fixture deliberately makes no claims about real type-system or
/// domain rules. It lets each test prove that the hybrid validator delegates
/// those decisions and propagates the result correctly.
#[derive(Default)]
struct TestContext {
    invalid_operations: BTreeSet<String>,
    incomplete_operations: BTreeSet<String>,
    invalid_types: BTreeSet<(String, String)>,
    incomplete_types: BTreeSet<(String, String)>,
    invalid_transitions: BTreeSet<(HybridDomainId, HybridDomainId)>,
    incomplete_transitions: BTreeSet<(HybridDomainId, HybridDomainId)>,
    invalid_sync_groups: BTreeSet<HybridEntityId>,
    incomplete_sync_groups: BTreeSet<HybridEntityId>,
}

impl HybridSemanticContext for TestContext {
    fn validate_operation(
        &self,
        node: &HybridNode,
    ) -> Result<(), HybridContextFailure> {
        let operation = node.operation().qualified_name();

        if self.incomplete_operations.contains(&operation) {
            return Err(
                HybridContextFailure::incomplete(
                    "TEST_OPERATION_INCOMPLETE",
                    format!("operation `{operation}` could not be resolved"),
                )
                .expect("non-empty test diagnostic"),
            );
        }

        if self.invalid_operations.contains(&operation) {
            return Err(
                HybridContextFailure::invalid(
                    "TEST_OPERATION_INVALID",
                    format!("operation `{operation}` violates its contract"),
                )
                .expect("non-empty test diagnostic"),
            );
        }

        Ok(())
    }

    fn check_type_compatibility(
        &self,
        value_type: &str,
        expected_type: &str,
        _source_node: Option<&crate::frontend::ast::node::node_id::NodeId>,
        _consumer_node: Option<&crate::frontend::ast::node::node_id::NodeId>,
    ) -> Result<(), HybridContextFailure> {
        let pair = (value_type.to_owned(), expected_type.to_owned());

        if self.incomplete_types.contains(&pair) {
            return Err(
                HybridContextFailure::incomplete(
                    "TEST_TYPE_INCOMPLETE",
                    format!(
                        "compatibility of `{value_type}` with `{expected_type}` \
                         could not be established"
                    ),
                )
                .expect("non-empty test diagnostic"),
            );
        }

        if self.invalid_types.contains(&pair) {
            return Err(
                HybridContextFailure::invalid(
                    "TEST_TYPE_INVALID",
                    format!(
                        "`{value_type}` is not compatible with `{expected_type}`"
                    ),
                )
                .expect("non-empty test diagnostic"),
            );
        }

        Ok(())
    }

    fn validate_domain_transition(
        &self,
        producer: &HybridNode,
        consumer: &HybridNode,
        _value: &HybridValue,
    ) -> Result<(), HybridContextFailure> {
        let transition = (
            producer.operation().domain().clone(),
            consumer.operation().domain().clone(),
        );

        if self.incomplete_transitions.contains(&transition) {
            return Err(
                HybridContextFailure::incomplete(
                    "TEST_TRANSITION_INCOMPLETE",
                    "domain-transition validity could not be established",
                )
                .expect("non-empty test diagnostic"),
            );
        }

        if self.invalid_transitions.contains(&transition) {
            return Err(
                HybridContextFailure::invalid(
                    "TEST_TRANSITION_INVALID",
                    "the domain transition is not permitted",
                )
                .expect("non-empty test diagnostic"),
            );
        }

        Ok(())
    }

    fn validate_synchronization(
        &self,
        group: &HybridEntityId,
        _members: &[&HybridNode],
    ) -> Result<(), HybridContextFailure> {
        if self.incomplete_sync_groups.contains(group) {
            return Err(
                HybridContextFailure::incomplete(
                    "TEST_SYNC_INCOMPLETE",
                    "synchronization validity could not be established",
                )
                .expect("non-empty test diagnostic"),
            );
        }

        if self.invalid_sync_groups.contains(group) {
            return Err(
                HybridContextFailure::invalid(
                    "TEST_SYNC_INVALID",
                    "the synchronization group violates its contract",
                )
                .expect("non-empty test diagnostic"),
            );
        }

        Ok(())
    }
}

fn entity(name: impl Into<String>) -> HybridEntityId {
    HybridEntityId::new(name).expect("test entity identity must be valid")
}

fn domain(namespace: &str, name: &str) -> HybridDomainId {
    HybridDomainId::new(namespace, name).expect("test domain identity must be valid")
}

fn operation(domain_id: HybridDomainId, name: &str) -> HybridOperationId {
    HybridOperationId::new(domain_id, name)
        .expect("test operation identity must be valid")
}

fn value(id: &str, domain_id: HybridDomainId, type_id: &str) -> HybridValue {
    HybridValue::new(entity(id), domain_id, type_id, None)
        .expect("test value must be well formed")
}

fn input(id: &str, expected_type: Option<&str>) -> HybridValueInput {
    HybridValueInput::new(
        entity(id),
        expected_type.map(str::to_owned),
    )
    .expect("test input must be well formed")
}

fn node(
    id: &str,
    operation_id: HybridOperationId,
    inputs: Vec<HybridValueInput>,
    outputs: Vec<HybridValue>,
) -> HybridNode {
    HybridNode::new(
        entity(id),
        None,
        operation_id,
        HybridNodeKind::Compute,
        inputs,
        outputs,
        None,
    )
}

fn report(program: &HybridProgram) -> crate::frontend::semantic::domains::hybrid::HybridValidation {
    validate_hybrid(program, &TestContext::default())
}

fn has_issue(
    program: &HybridProgram,
    expected: HybridIssueKind,
) -> bool {
    report(program)
        .issues()
        .iter()
        .any(|issue| issue.kind() == expected)
}

fn add_simple_node(
    program: &mut HybridProgram,
    id: &str,
    domain_id: HybridDomainId,
) {
    program.add_node(node(
        id,
        operation(domain_id, "compute"),
        Vec::new(),
        Vec::new(),
    ));
}

#[test]
fn empty_input_is_complete_and_structurally_valid() {
    let result = report(&HybridProgram::new());

    assert!(result.is_complete());
    assert!(result.is_valid());
    assert_eq!(result.completion(), HybridCompletion::Complete);
    assert!(result.issues().is_empty());
    assert_eq!(result.validated_nodes(), 0);
    assert_eq!(result.validated_values(), 0);
    assert_eq!(result.validated_dependencies(), 0);
}

#[test]
fn classical_to_quantum_value_flow_delegates_boundary_validation() {
    let classical = domain("zamani", "classical");
    let quantum = domain("zamani", "quantum");

    let mut program = HybridProgram::new();
    program.add_node(node(
        "prepare-classical-input",
        operation(classical.clone(), "prepare"),
        Vec::new(),
        vec![value("measurement-parameter", classical, "Scalar")],
    ));
    program.add_node(node(
        "invoke-quantum-operation",
        operation(quantum, "consume"),
        vec![input("measurement-parameter", Some("Scalar"))],
        Vec::new(),
    ));

    let result = report(&program);

    assert!(result.is_valid(), "{:#?}", result.issues());
    assert_eq!(result.validated_nodes(), 2);
    assert_eq!(result.validated_values(), 1);
}

#[test]
fn quantum_to_classical_value_flow_delegates_boundary_validation() {
    let quantum = domain("zamani", "quantum");
    let classical = domain("zamani", "classical");

    let mut program = HybridProgram::new();
    program.add_node(node(
        "quantum-measurement",
        operation(quantum.clone(), "measure"),
        Vec::new(),
        vec![value("measurement-result", quantum, "MeasurementResult")],
    ));
    program.add_node(node(
        "classical-feedback",
        operation(classical, "branch"),
        vec![input("measurement-result", Some("MeasurementResult"))],
        Vec::new(),
    ));

    let result = report(&program);

    assert!(result.is_valid(), "{:#?}", result.issues());
    assert_eq!(result.validated_nodes(), 2);
}

#[test]
fn hdl_to_classical_flow_uses_the_shared_boundary_contract() {
    let hdl = domain("zamani", "hdl");
    let classical = domain("zamani", "classical");

    let mut program = HybridProgram::new();
    program.add_node(node(
        "signal-source",
        operation(hdl.clone(), "read_signal"),
        Vec::new(),
        vec![value("signal-value", hdl, "SignalValue")],
    ));
    program.add_node(node(
        "classical-consumer",
        operation(classical, "decode"),
        vec![input("signal-value", Some("SignalValue"))],
        Vec::new(),
    ));

    // This structural validator delegates domain-specific legality to the
    // context. An empty fake context allows transitions; production context
    // must apply the canonical language rules.
    assert!(report(&program).is_valid());
}

#[test]
fn unknown_input_value_is_an_error_not_a_success() {
    let mut program = HybridProgram::new();
    add_simple_node(
        &mut program,
        "consumer",
        domain("zamani", "classical"),
    );

    let consumer = node(
        "consumer-with-missing-input",
        operation(domain("zamani", "classical"), "consume"),
        vec![input("not-produced", None)],
        Vec::new(),
    );
    program.add_node(consumer);

    let result = report(&program);

    assert!(!result.is_valid());
    assert!(result.issues().iter().any(|issue| {
        issue.kind() == HybridIssueKind::UnresolvedValue
            && issue.severity() == HybridIssueSeverity::Error
    }));
}

#[test]
fn duplicate_node_identities_are_rejected() {
    let domain_id = domain("zamani", "classical");
    let operation_id = operation(domain_id, "compute");

    let mut program = HybridProgram::new();
    program.add_node(node(
        "duplicate",
        operation_id.clone(),
        Vec::new(),
        Vec::new(),
    ));
    program.add_node(node(
        "duplicate",
        operation_id,
        Vec::new(),
        Vec::new(),
    ));

    assert!(has_issue(&program, HybridIssueKind::DuplicateNode));
}

#[test]
fn duplicate_output_value_identities_are_rejected() {
    let domain_id = domain("zamani", "classical");

    let mut program = HybridProgram::new();
    program.add_node(node(
        "producer-a",
        operation(domain_id.clone(), "produce_a"),
        Vec::new(),
        vec![value("reused-value", domain_id.clone(), "Scalar")],
    ));
    program.add_node(node(
        "producer-b",
        operation(domain_id.clone(), "produce_b"),
        Vec::new(),
        vec![value("reused-value", domain_id, "Scalar")],
    ));

    assert!(has_issue(&program, HybridIssueKind::DuplicateValue));
}

#[test]
fn output_domain_must_match_its_producer_operation_domain() {
    let classical = domain("zamani", "classical");
    let quantum = domain("zamani", "quantum");

    let mut program = HybridProgram::new();
    program.add_node(node(
        "invalid-producer",
        operation(classical, "produce"),
        Vec::new(),
        vec![value("wrong-domain-output", quantum, "Scalar")],
    ));

    assert!(has_issue(&program, HybridIssueKind::DomainIdentityMismatch));
}

#[test]
fn incompatible_input_type_is_reported_by_shared_type_service() {
    let classical = domain("zamani", "classical");

    let mut program = HybridProgram::new();
    program.add_node(node(
        "producer",
        operation(classical.clone(), "produce"),
        Vec::new(),
        vec![value("text-value", classical.clone(), "Text")],
    ));
    program.add_node(node(
        "consumer",
        operation(classical, "consume"),
        vec![input("text-value", Some("Scalar"))],
        Vec::new(),
    ));

    let context = TestContext {
        invalid_types: BTreeSet::from([("Text".into(), "Scalar".into())]),
        ..TestContext::default()
    };
    let result = validate_hybrid(&program, &context);

    assert!(!result.is_valid());
    assert!(result.issues().iter().any(|issue| {
        issue.kind() == HybridIssueKind::TypeOrBoundaryViolation
            && issue.rule() == "TEST_TYPE_INVALID"
    }));
}

#[test]
fn incomplete_type_check_never_produces_valid_result() {
    let classical = domain("zamani", "classical");

    let mut program = HybridProgram::new();
    program.add_node(node(
        "producer",
        operation(classical.clone(), "produce"),
        Vec::new(),
        vec![value("v", classical.clone(), "UnresolvedType")],
    ));
    program.add_node(node(
        "consumer",
        operation(classical, "consume"),
        vec![input("v", Some("ExpectedType"))],
        Vec::new(),
    ));

    let context = TestContext {
        incomplete_types: BTreeSet::from([(
            "UnresolvedType".into(),
            "ExpectedType".into(),
        )]),
        ..TestContext::default()
    };
    let result = validate_hybrid(&program, &context);

    assert!(!result.is_valid());
    assert_eq!(result.completion(), HybridCompletion::Incomplete);
    assert!(result.issues().iter().any(|issue| {
        issue.kind() == HybridIssueKind::IncompleteAnalysis
            && issue.severity() == HybridIssueSeverity::Incomplete
    }));
}

#[test]
fn prohibited_domain_transition_is_reported_by_shared_context() {
    let classical = domain("zamani", "classical");
    let quantum = domain("zamani", "quantum");

    let mut program = HybridProgram::new();
    program.add_node(node(
        "classical-producer",
        operation(classical.clone(), "produce"),
        Vec::new(),
        vec![value("v", classical.clone(), "Scalar")],
    ));
    program.add_node(node(
        "quantum-consumer",
        operation(quantum.clone(), "consume"),
        vec![input("v", None)],
        Vec::new(),
    ));

    let context = TestContext {
        invalid_transitions: BTreeSet::from([(classical, quantum)]),
        ..TestContext::default()
    };
    let result = validate_hybrid(&program, &context);

    assert!(!result.is_valid());
    assert!(result.issues().iter().any(|issue| {
        issue.kind() == HybridIssueKind::TypeOrBoundaryViolation
            && issue.rule() == "TEST_TRANSITION_INVALID"
    }));
}

#[test]
fn incomplete_domain_transition_is_not_treated_as_permitted() {
    let classical = domain("zamani", "classical");
    let quantum = domain("zamani", "quantum");

    let mut program = HybridProgram::new();
    program.add_node(node(
        "producer",
        operation(classical.clone(), "produce"),
        Vec::new(),
        vec![value("v", classical.clone(), "Scalar")],
    ));
    program.add_node(node(
        "consumer",
        operation(quantum.clone(), "consume"),
        vec![input("v", None)],
        Vec::new(),
    ));

    let context = TestContext {
        incomplete_transitions: BTreeSet::from([(classical, quantum)]),
        ..TestContext::default()
    };
    let result = validate_hybrid(&program, &context);

    assert!(!result.is_valid());
    assert_eq!(result.completion(), HybridCompletion::Incomplete);
}

#[test]
fn operation_contract_violations_are_propagated() {
    let domain_id = domain("zamani", "quantum");
    let operation_id = operation(domain_id, "extension.operation");

    let mut program = HybridProgram::new();
    program.add_node(node(
        "operation-node",
        operation_id.clone(),
        Vec::new(),
        Vec::new(),
    ));

    let context = TestContext {
        invalid_operations: BTreeSet::from([operation_id.qualified_name()]),
        ..TestContext::default()
    };
    let result = validate_hybrid(&program, &context);

    assert!(!result.is_valid());
    assert!(result.issues().iter().any(|issue| {
        issue.kind() == HybridIssueKind::OperationViolation
            && issue.rule() == "TEST_OPERATION_INVALID"
    }));
}

#[test]
fn unresolved_operation_contract_marks_analysis_incomplete() {
    let domain_id = domain("vendor.example", "future_domain");
    let operation_id = operation(domain_id, "future.operation");

    let mut program = HybridProgram::new();
    program.add_node(node(
        "future-operation",
        operation_id.clone(),
        Vec::new(),
        Vec::new(),
    ));

    let context = TestContext {
        incomplete_operations: BTreeSet::from([operation_id.qualified_name()]),
        ..TestContext::default()
    };
    let result = validate_hybrid(&program, &context);

    assert!(!result.is_valid());
    assert_eq!(result.completion(), HybridCompletion::Incomplete);
}

#[test]
fn ordinary_dependency_cycle_is_rejected() {
    let domain_id = domain("zamani", "classical");

    let mut program = HybridProgram::new();
    add_simple_node(&mut program, "a", domain_id.clone());
    add_simple_node(&mut program, "b", domain_id);

    program.add_dependency(HybridDependency::new(
        entity("a"),
        entity("b"),
        HybridDependencyKind::Ordering,
    ));
    program.add_dependency(HybridDependency::new(
        entity("b"),
        entity("a"),
        HybridDependencyKind::Ordering,
    ));

    assert!(has_issue(&program, HybridIssueKind::DependencyCycle));
}

#[test]
fn unknown_dependency_endpoints_are_rejected() {
    let domain_id = domain("zamani", "classical");

    let mut program = HybridProgram::new();
    add_simple_node(&mut program, "known", domain_id);

    program.add_dependency(HybridDependency::new(
        entity("missing-source"),
        entity("known"),
        HybridDependencyKind::Ordering,
    ));

    assert!(has_issue(
        &program,
        HybridIssueKind::UnknownDependencyEndpoint
    ));
}

#[test]
fn duplicate_dependency_edges_are_reported() {
    let domain_id = domain("zamani", "classical");

    let mut program = HybridProgram::new();
    add_simple_node(&mut program, "a", domain_id.clone());
    add_simple_node(&mut program, "b", domain_id);

    let edge = HybridDependency::new(
        entity("a"),
        entity("b"),
        HybridDependencyKind::Ordering,
    );
    program.add_dependency(edge.clone());
    program.add_dependency(edge);

    assert!(has_issue(&program, HybridIssueKind::DuplicateDependency));
}

#[test]
fn synchronization_node_requires_a_group_identity() {
    let domain_id = domain("zamani", "hybrid");
    let sync_node = HybridNode::new(
        entity("sync"),
        None,
        operation(domain_id, "synchronize"),
        HybridNodeKind::Synchronization,
        Vec::new(),
        Vec::new(),
        None,
    );

    let mut program = HybridProgram::new();
    program.add_node(sync_node);

    assert!(has_issue(
        &program,
        HybridIssueKind::InvalidSynchronizationGroup
    ));
}

#[test]
fn synchronization_contract_failures_are_propagated() {
    let group = entity("phase-boundary");
    let domain_id = domain("zamani", "hybrid");
    let sync_node = HybridNode::new(
        entity("sync"),
        None,
        operation(domain_id, "synchronize"),
        HybridNodeKind::Synchronization,
        Vec::new(),
        Vec::new(),
        Some(group.clone()),
    );

    let mut program = HybridProgram::new();
    program.add_node(sync_node);

    let context = TestContext {
        invalid_sync_groups: BTreeSet::from([group]),
        ..TestContext::default()
    };
    let result = validate_hybrid(&program, &context);

    assert!(!result.is_valid());
    assert!(result.issues().iter().any(|issue| {
        issue.kind() == HybridIssueKind::SynchronizationViolation
            && issue.rule() == "TEST_SYNC_INVALID"
    }));
}

#[test]
fn incomplete_synchronization_contract_is_not_certified() {
    let group = entity("distributed-phase");
    let domain_id = domain("zamani", "hybrid");
    let sync_node = HybridNode::new(
        entity("sync"),
        None,
        operation(domain_id, "synchronize"),
        HybridNodeKind::Synchronization,
        Vec::new(),
        Vec::new(),
        Some(group.clone()),
    );

    let mut program = HybridProgram::new();
    program.add_node(sync_node);

    let context = TestContext {
        incomplete_sync_groups: BTreeSet::from([group]),
        ..TestContext::default()
    };
    let result = validate_hybrid(&program, &context);

    assert!(!result.is_valid());
    assert_eq!(result.completion(), HybridCompletion::Incomplete);
}

#[test]
fn feedback_edges_are_preserved_without_being_misclassified_as_ordinary_cycles() {
    let domain_id = domain("zamani", "classical");

    let mut program = HybridProgram::new();
    add_simple_node(&mut program, "feedback-controller", domain_id);

    program.add_dependency(HybridDependency::new(
        entity("feedback-controller"),
        entity("feedback-controller"),
        HybridDependencyKind::Feedback,
    ));

    let result = report(&program);

    assert!(
        !result.issues().iter().any(|issue| {
            issue.kind() == HybridIssueKind::DependencyCycle
        }),
        "explicit feedback is delegated to domain-specific validation"
    );
}

#[test]
fn future_domains_and_extension_operations_need_no_core_enum_change() {
    let future_domain = domain("future.example", "photonic_computing");
    let future_operation = operation(
        future_domain.clone(),
        "vendor_extension::adaptive_transform",
    );

    let mut program = HybridProgram::new();
    program.add_node(node(
        "future-node",
        future_operation,
        Vec::new(),
        vec![value("future-result", future_domain, "future.example::Value")],
    ));

    let result = report(&program);

    assert!(result.is_valid(), "{:#?}", result.issues());
}

#[test]
fn identities_reject_empty_components_and_control_characters() {
    assert!(matches!(
        HybridDomainId::new("", "classical"),
        Err(HybridInputError::EmptyIdentity { .. })
    ));

    assert!(matches!(
        HybridDomainId::new("zamani\ninvalid", "classical"),
        Err(HybridInputError::ControlCharacter { .. })
    ));

    assert!(matches!(
        HybridEntityId::new("node\u{0000}invalid"),
        Err(HybridInputError::ControlCharacter { .. })
    ));

    assert!(matches!(
        HybridOperationId::new(domain("zamani", "classical"), " "),
        Err(HybridInputError::EmptyIdentity { .. })
    ));
}

#[test]
fn large_symbolic_graph_is_checked_without_a_hardware_ceiling() {
    // This is a workload chosen for this test, not a production maximum.
    // Resource and graph sizes remain subject to available resources and the
    // enclosing analysis session's explicit operational budget.
    let workload = 2_000usize;
    let domain_id = domain("zamani", "classical");
    let mut program = HybridProgram::new();

    for index in 0..workload {
        let current = format!("node-{index}");
        let output = format!("value-{index}");

        let inputs = if index == 0 {
            Vec::new()
        } else {
            vec![input(&format!("value-{}", index - 1), Some("Scalar"))]
        };

        let outputs = vec![value(&output, domain_id.clone(), "Scalar")];

        program.add_node(node(
            &current,
            operation(domain_id.clone(), "transform"),
            inputs,
            outputs,
        ));
    }

    let result = report(&program);

    assert!(result.is_valid(), "{:#?}", result.issues());
    assert_eq!(result.validated_nodes(), workload);
    assert_eq!(result.validated_values(), workload);
}

#[test]
fn diagnostics_are_deterministic_for_identical_inputs() {
    let classical = domain("zamani", "classical");
    let mut program = HybridProgram::new();

    program.add_node(node(
        "consumer",
        operation(classical.clone(), "consume"),
        vec![input("missing-a", None), input("missing-b", None)],
        Vec::new(),
    ));
    program.add_dependency(HybridDependency::new(
        entity("absent"),
        entity("consumer"),
        HybridDependencyKind::Ordering,
    ));

    let first = report(&program);
    let second = report(&program);

    assert_eq!(first, second);
    assert_eq!(first.issues().len(), second.issues().len());

    for (left, right) in first.issues().iter().zip(second.issues()) {
        assert_eq!(left.kind(), right.kind());
        assert_eq!(left.severity(), right.severity());
        assert_eq!(left.rule(), right.rule());
        assert_eq!(left.message(), right.message());
        assert_eq!(left.entity(), right.entity());
    }
}

#[test]
fn grammar_contracts_for_hybrid_composition_exist_and_remain_nonempty() {
    // Paths are relative to this test file:
    // src/frontend/semantic/tests/hybrid.rs -> repository root.
    const HYBRID: &str =
        include_str!("../../../../grammar/hybrid/hybrid.g4");
    const CLASSICAL_QUANTUM: &str =
        include_str!("../../../../grammar/hybrid/classical-quantum.g4");
    const CLASSICAL_QUANTUM_BOUNDARY: &str =
        include_str!("../../../../grammar/hybrid/classical-quantum-boundary.g4");
    const QUANTUM_CLASSICAL_CONTROL: &str =
        include_str!("../../../../grammar/hybrid/quantum-classical-control.g4");
    const HYBRID_RESOURCES: &str =
        include_str!("../../../../grammar/hybrid/hybrid-resources.g4");
    const MID_CIRCUIT_CONTROL: &str =
        include_str!("../../../../grammar/quantum/mid-circuit-control.g4");

    for (name, source) in [
        ("hybrid", HYBRID),
        ("classical-quantum", CLASSICAL_QUANTUM),
        ("classical-quantum-boundary", CLASSICAL_QUANTUM_BOUNDARY),
        ("quantum-classical-control", QUANTUM_CLASSICAL_CONTROL),
        ("hybrid-resources", HYBRID_RESOURCES),
        ("mid-circuit-control", MID_CIRCUIT_CONTROL),
    ] {
        assert!(
            !source.trim().is_empty(),
            "authoritative grammar `{name}` must not be empty"
        );
    }

    assert!(
        HYBRID.contains("Hybrid"),
        "hybrid grammar should identify the hybrid composition domain"
    );
    assert!(
        CLASSICAL_QUANTUM.contains("classical")
            || CLASSICAL_QUANTUM.contains("Classical"),
        "classical/quantum grammar should document its domain boundary"
    );
    assert!(
        QUANTUM_CLASSICAL_CONTROL.contains("control")
            || QUANTUM_CLASSICAL_CONTROL.contains("Control"),
        "hybrid control grammar should identify its control boundary"
    );
    assert!(
        HYBRID_RESOURCES.contains("resource")
            || HYBRID_RESOURCES.contains("Resource"),
        "hybrid resource adapter should identify its resource boundary"
    );
}
