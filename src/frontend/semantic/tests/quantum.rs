
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Quantum semantic conformance tests for Zamani's native frontend.
//!
//! PURPOSE
//!
//! Exercise the public quantum-domain validator against operation contracts,
//! invocation facts, shared semantic services, incomplete analysis, deterministic
//! diagnostics, and open-world extension identities.
//!
//! AUTHORITY
//!
//! - `grammar/quantum/operations.g4`
//! - `grammar/quantum/parameters.g4`
//! - `grammar/quantum/measurement.g4`
//! - `grammar/quantum/controls.g4`
//! - `grammar/quantum/quantum-resources.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/domains.md`
//! - `src/frontend/ast/node/node_id.rs`
//! - `src/frontend/semantic/domains/quantum.rs`
//!
//! OWNERSHIP
//!
//! These tests do not define quantum semantics. The production validator,
//! shared type/effect/capability systems, canonical AST, and normative grammar
//! remain authoritative.
//!
//! SCALABILITY
//!
//! Tests assert that operation identity and cardinality contracts are open
//! ended. Test collection sizes are finite test inputs, not language limits.
//! No test introduces a hardware-capacity ceiling.
//!
//! INTEGRATION
//!
//! Declare this file as `mod quantum;` in
//! `src/frontend/semantic/tests/mod.rs`. The parent semantic module must expose
//! the `domains::quantum` API. Grammar paths below are relative to this file.
//!
//! FREEZE CRITERIA
//!
//! This file is frozen only after the parent modules are wired, the tests
//! compile against the actual repository, and the focused and workspace test
//! suites pass on the supported Rust toolchain.

use std::collections::BTreeSet;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::semantic::domains::quantum::{
    validate_quantum_invocations, Cardinality, ClassicalControlFacts,
    OperationContract, OperationIdentity, OperationModifier, ParameterContract,
    QuantumAnalysisFailure, QuantumBoundaryDirection, QuantumCompletion,
    QuantumIssueKind, QuantumSemanticContext, QuantumValidationOptions,
    QuantumValidator, ResolvedArgument, ResolvedControl, ResolvedTarget,
    ResultBinding, SemanticTypeIdentity,
};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn node(raw: u64) -> NodeId {
    NodeId::new(raw).expect("test node IDs must be non-zero")
}

fn operation(namespace: &str, name: &str) -> OperationIdentity {
    OperationIdentity::new(namespace, name).expect("valid operation identity")
}

fn ty(name: &str) -> SemanticTypeIdentity {
    SemanticTypeIdentity::new("test.types", name).expect("valid semantic type")
}

fn contract(name: &str) -> OperationContract {
    OperationContract {
        identity: operation("test.quantum", name),
        parameters: Vec::new(),
        targets: Cardinality::one_or_more(),
        controls: Cardinality::any(),
        allowed_modifiers: BTreeSet::new(),
        results: Vec::new(),
        performs_measurement: false,
        permits_classical_control: false,
        permits_repeated_targets: false,
        required_capabilities: BTreeSet::new(),
        declared_effects: BTreeSet::new(),
    }
}

fn target(raw_node: u64, identity: &str) -> ResolvedTarget {
    ResolvedTarget {
        node: node(raw_node),
        identity: identity.to_owned(),
        ty: ty("logical-qubit"),
    }
}

fn control(raw_node: u64, identity: &str) -> ResolvedControl {
    ResolvedControl {
        node: node(raw_node),
        identity: identity.to_owned(),
        ty: ty("logical-qubit"),
    }
}

fn invocation(name: &str) -> crate::frontend::semantic::domains::quantum::QuantumInvocation {
    crate::frontend::semantic::domains::quantum::QuantumInvocation {
        node: node(1),
        operation: operation("test.quantum", name),
        arguments: Vec::new(),
        targets: vec![target(2, "logical:q0")],
        controls: Vec::new(),
        modifiers: Vec::new(),
        results: Vec::new(),
        classical_control: ClassicalControlFacts::default(),
        resolution_complete: true,
    }
}

#[derive(Clone, Debug, Default)]
struct TestContext {
    incompatible_type_pairs: BTreeSet<(SemanticTypeIdentity, SemanticTypeIdentity)>,
    missing_capabilities: BTreeSet<String>,
    denied_effects: BTreeSet<String>,
    denied_boundaries: BTreeSet<QuantumBoundaryDirection>,
}

impl QuantumSemanticContext for TestContext {
    fn types_compatible(
        &self,
        expected: &SemanticTypeIdentity,
        actual: &SemanticTypeIdentity,
    ) -> bool {
        !self
            .incompatible_type_pairs
            .contains(&(expected.clone(), actual.clone()))
    }

    fn capability_established(&self, capability: &str, _at: NodeId) -> bool {
        !self.missing_capabilities.contains(capability)
    }

    fn effect_permitted(&self, effect: &str, _at: NodeId) -> bool {
        !self.denied_effects.contains(effect)
    }

    fn boundary_permitted(
        &self,
        _at: NodeId,
        direction: QuantumBoundaryDirection,
    ) -> bool {
        !self.denied_boundaries.contains(&direction)
    }
}

fn validator(
    contracts: impl IntoIterator<Item = OperationContract>,
) -> Result<QuantumValidator, QuantumAnalysisFailure> {
    QuantumValidator::new(contracts)
}

fn has_rule(
    report: &crate::frontend::semantic::domains::quantum::QuantumValidationReport,
    rule: &str,
) -> bool {
    report.issues.iter().any(|issue| issue.rule == rule)
}

#[test]
fn accepts_an_open_world_operation_identity() -> TestResult {
    let id = OperationIdentity::new(
        "future.domain.provider.extension",
        "operation_with_new_semantics",
    )?;

    assert_eq!(id.namespace(), "future.domain.provider.extension");
    assert_eq!(id.name(), "operation_with_new_semantics");
    assert_eq!(
        id.qualified_name(),
        "future.domain.provider.extension::operation_with_new_semantics"
    );
    Ok(())
}

#[test]
fn rejects_empty_or_whitespace_operation_identity_components() {
    assert!(OperationIdentity::new("", "measure").is_err());
    assert!(OperationIdentity::new("quantum", "").is_err());
    assert!(OperationIdentity::new("   ", "measure").is_err());
    assert!(OperationIdentity::new("quantum", "   ").is_err());
    assert!(OperationIdentity::new(" quantum", "measure").is_err());
    assert!(OperationIdentity::new("quantum", "measure ").is_err());
}

#[test]
fn accepts_unbounded_cardinality_without_a_hardware_ceiling() -> TestResult {
    let any = Cardinality::any();
    let one_or_more = Cardinality::one_or_more();

    assert!(any.accepts(0));
    assert!(any.accepts(1));
    assert!(one_or_more.accepts(1));
    assert!(!one_or_more.accepts(0));

    // This checks the largest count representable by this host's `usize`.
    // It does not allocate a collection of that size.
    assert!(any.accepts(usize::MAX));

    let unbounded = Cardinality::new(7, None)?;
    assert!(!unbounded.accepts(6));
    assert!(unbounded.accepts(7));
    assert!(unbounded.accepts(usize::MAX));

    Ok(())
}

#[test]
fn rejects_a_cardinality_whose_maximum_is_below_its_minimum() {
    assert!(Cardinality::new(3, Some(2)).is_err());
    assert!(Cardinality::new(0, Some(0)).is_ok());
    assert!(Cardinality::new(u64::MAX, Some(u64::MAX)).is_ok());
}

#[test]
fn accepts_a_valid_operation_and_invocation() -> TestResult {
    let validator = validator([contract("user_defined_operation")])?;
    let report = validator.validate(
        &[invocation("user_defined_operation")],
        &TestContext::default(),
    );

    assert!(report.is_complete());
    assert!(report.is_valid());
    assert_eq!(report.completion, QuantumCompletion::Complete);
    assert_eq!(report.invocations.len(), 1);
    assert!(report.invocations[0].is_valid());
    assert!(report.issues.is_empty());
    Ok(())
}

#[test]
fn rejects_duplicate_operation_definitions_instead_of_overwriting() {
    let result = validator([contract("custom"), contract("custom")]);

    assert!(matches!(
        result,
        Err(QuantumAnalysisFailure::InvalidContext { .. })
    ));
}

#[test]
fn rejects_an_unregistered_operation_without_certifying_it() -> TestResult {
    let validator = validator([])?;
    let report =
        validator.validate(&[invocation("not_registered")], &TestContext::default());

    assert_eq!(report.completion, QuantumCompletion::CompleteWithIssues);
    assert!(!report.is_valid());
    assert!(has_rule(&report, "QSEM0002"));
    assert_eq!(
        report.issues[0].kind,
        QuantumIssueKind::UnresolvedOperation
    );
    Ok(())
}

#[test]
fn unresolved_invocation_facts_make_analysis_incomplete() -> TestResult {
    let validator = validator([contract("operation")])?;
    let mut call = invocation("operation");
    call.resolution_complete = false;

    let report = validator.validate(&[call], &TestContext::default());

    assert_eq!(report.completion, QuantumCompletion::Incomplete);
    assert!(!report.is_complete());
    assert!(!report.is_valid());
    assert!(has_rule(&report, "QSEM0001"));
    Ok(())
}

#[test]
fn validates_required_and_optional_parameters() -> TestResult {
    let mut operation = contract("parameterized");
    operation.parameters = vec![
        ParameterContract {
            name: Some("angle".to_owned()),
            expected_type: ty("angle"),
            optional: false,
        },
        ParameterContract {
            name: Some("precision".to_owned()),
            expected_type: ty("precision"),
            optional: true,
        },
    ];

    let validator = validator([operation])?;

    let mut valid = invocation("parameterized");
    valid.arguments.push(ResolvedArgument {
        node: node(3),
        ty: ty("angle"),
        name: Some("angle".to_owned()),
    });

    let valid_report = validator.validate(&[valid], &TestContext::default());
    assert!(valid_report.is_valid());

    let mut missing_required = invocation("parameterized");
    missing_required.arguments.push(ResolvedArgument {
        node: node(4),
        ty: ty("precision"),
        name: Some("precision".to_owned()),
    });

    let invalid_report =
        validator.validate(&[missing_required], &TestContext::default());
    assert!(has_rule(&invalid_report, "QSEM0101"));
    assert!(!invalid_report.is_valid());

    Ok(())
}

#[test]
fn parameter_type_compatibility_is_delegated_to_the_shared_type_service(
) -> TestResult {
    let mut operation = contract("typed");
    operation.parameters.push(ParameterContract {
        name: Some("angle".to_owned()),
        expected_type: ty("angle"),
        optional: false,
    });

    let validator = validator([operation])?;
    let mut call = invocation("typed");
    call.arguments.push(ResolvedArgument {
        node: node(3),
        ty: ty("text"),
        name: Some("angle".to_owned()),
    });

    let report = validator.validate(&[call], &TestContext::default());
    assert!(has_rule(&report, "QSEM0102"));
    Ok(())
}

#[test]
fn reports_mismatched_named_argument_identity() -> TestResult {
    let mut operation = contract("named");
    operation.parameters.push(ParameterContract {
        name: Some("angle".to_owned()),
        expected_type: ty("angle"),
        optional: false,
    });

    let validator = validator([operation])?;
    let mut call = invocation("named");
    call.arguments.push(ResolvedArgument {
        node: node(3),
        ty: ty("angle"),
        name: Some("duration".to_owned()),
    });

    let report = validator.validate(&[call], &TestContext::default());
    assert!(has_rule(&report, "QSEM0103"));
    Ok(())
}

#[test]
fn validates_target_cardinality_and_duplicate_target_identity() -> TestResult {
    let mut operation = contract("two_targets");
    operation.targets = Cardinality::new(2, Some(2))?;
    let validator = validator([operation])?;

    let mut too_few = invocation("two_targets");
    too_few.targets.clear();
    let report = validator.validate(&[too_few], &TestContext::default());
    assert!(has_rule(&report, "QSEM0201"));

    let mut repeated = invocation("two_targets");
    repeated.targets.push(target(3, "logical:q0"));
    let report = validator.validate(&[repeated], &TestContext::default());
    assert!(has_rule(&report, "QSEM0203"));

    Ok(())
}

#[test]
fn permits_repeated_targets_only_when_the_operation_contract_allows_them(
) -> TestResult {
    let mut operation = contract("repeatable");
    operation.targets = Cardinality::new(2, Some(2))?;
    operation.permits_repeated_targets = true;

    let validator = validator([operation])?;
    let mut call = invocation("repeatable");
    call.targets.push(target(3, "logical:q0"));

    let report = validator.validate(&[call], &TestContext::default());
    assert!(!has_rule(&report, "QSEM0203"));
    Ok(())
}

#[test]
fn rejects_a_control_that_is_also_a_target() -> TestResult {
    let validator = validator([contract("controlled")])?;
    let mut call = invocation("controlled");
    call.controls.push(control(4, "logical:q0"));

    let report = validator.validate(&[call], &TestContext::default());
    assert!(has_rule(&report, "QSEM0303"));
    Ok(())
}

#[test]
fn rejects_duplicate_control_identities() -> TestResult {
    let validator = validator([contract("controlled")])?;
    let mut call = invocation("controlled");
    call.controls.push(control(4, "logical:control"));
    call.controls.push(control(5, "logical:control"));

    let report = validator.validate(&[call], &TestContext::default());
    assert!(has_rule(&report, "QSEM0304"));
    Ok(())
}

#[test]
fn rejects_unsupported_modifiers_and_accepts_registered_modifiers() -> TestResult {
    let mut operation = contract("modifiable");
    operation
        .allowed_modifiers
        .insert("extension.adjoint".to_owned());

    let validator = validator([operation])?;
    let mut call = invocation("modifiable");
    call.modifiers.push(OperationModifier {
        node: node(3),
        identity: "extension.adjoint".to_owned(),
    });

    assert!(
        validator
            .validate(&[call.clone()], &TestContext::default())
            .is_valid()
    );

    call.modifiers.push(OperationModifier {
        node: node(4),
        identity: "extension.unregistered_modifier".to_owned(),
    });

    let report = validator.validate(&[call], &TestContext::default());
    assert!(has_rule(&report, "QSEM0402"));
    Ok(())
}

#[test]
fn rejects_duplicate_modifier_applications() -> TestResult {
    let mut operation = contract("modifiable");
    operation
        .allowed_modifiers
        .insert("extension.adjoint".to_owned());

    let validator = validator([operation])?;
    let mut call = invocation("modifiable");
    call.modifiers = vec![
        OperationModifier {
            node: node(3),
            identity: "extension.adjoint".to_owned(),
        },
        OperationModifier {
            node: node(4),
            identity: "extension.adjoint".to_owned(),
        },
    ];

    let report = validator.validate(&[call], &TestContext::default());
    assert!(has_rule(&report, "QSEM0403"));
    Ok(())
}

#[test]
fn validates_result_count_and_result_types() -> TestResult {
    let mut operation = contract("producing");
    operation.results.push(ty("measurement-result"));
    let validator = validator([operation])?;

    let missing = invocation("producing");
    let report = validator.validate(&[missing], &TestContext::default());
    assert!(has_rule(&report, "QSEM0501"));

    let mut mismatched = invocation("producing");
    mismatched.results.push(ResultBinding {
        node: node(3),
        ty: ty("text"),
    });

    let report = validator.validate(&[mismatched], &TestContext::default());
    assert!(has_rule(&report, "QSEM0502"));
    Ok(())
}

#[test]
fn measurement_contract_requires_a_result_binding() -> TestResult {
    let mut operation = contract("observe");
    operation.performs_measurement = true;
    operation.results.push(ty("measurement-result"));

    let validator = validator([operation])?;
    let report =
        validator.validate(&[invocation("observe")], &TestContext::default());

    assert!(has_rule(&report, "QSEM0601"));
    assert!(!report.is_valid());
    Ok(())
}

#[test]
fn measurement_respects_quantum_to_classical_boundary_policy() -> TestResult {
    let mut operation = contract("observe");
    operation.performs_measurement = true;
    operation.results.push(ty("measurement-result"));

    let validator = validator([operation])?;
    let mut call = invocation("observe");
    call.results.push(ResultBinding {
        node: node(3),
        ty: ty("measurement-result"),
    });

    let mut context = TestContext::default();
    context
        .denied_boundaries
        .insert(QuantumBoundaryDirection::QuantumToClassical);

    let report = validator.validate(&[call], &context);
    assert!(has_rule(&report, "QSEM0602"));
    Ok(())
}

#[test]
fn classical_feed_forward_requires_explicit_contract_permission() -> TestResult {
    let validator = validator([contract("no_feed_forward")])?;
    let mut call = invocation("no_feed_forward");
    call.classical_control = ClassicalControlFacts {
        conditional: true,
        uses_feed_forward: true,
        control_node: Some(node(6)),
    };

    let report = validator.validate(&[call], &TestContext::default());
    assert!(has_rule(&report, "QSEM0603"));
    Ok(())
}

#[test]
fn permitted_classical_feed_forward_still_obeys_boundary_policy() -> TestResult {
    let mut operation = contract("feed_forward");
    operation.permits_classical_control = true;
    let validator = validator([operation])?;

    let mut call = invocation("feed_forward");
    call.classical_control = ClassicalControlFacts {
        conditional: false,
        uses_feed_forward: true,
        control_node: Some(node(6)),
    };

    let mut context = TestContext::default();
    context
        .denied_boundaries
        .insert(QuantumBoundaryDirection::ClassicalToQuantum);

    let report = validator.validate(&[call], &context);
    assert!(has_rule(&report, "QSEM0604"));
    Ok(())
}

#[test]
fn reports_missing_capabilities_without_claiming_target_feasibility() -> TestResult {
    let mut operation = contract("requires_capability");
    operation
        .required_capabilities
        .insert("quantum.measurement".to_owned());

    let validator = validator([operation])?;
    let mut context = TestContext::default();
    context
        .missing_capabilities
        .insert("quantum.measurement".to_owned());

    let report = validator.validate(
        &[invocation("requires_capability")],
        &context,
    );

    assert!(has_rule(&report, "QSEM0702"));
    assert!(!report.is_valid());
    Ok(())
}

#[test]
fn reports_denied_effects_through_the_shared_context() -> TestResult {
    let mut operation = contract("effectful");
    operation
        .declared_effects
        .insert("quantum.execute".to_owned());

    let validator = validator([operation])?;
    let mut context = TestContext::default();
    context.denied_effects.insert("quantum.execute".to_owned());

    let report =
        validator.validate(&[invocation("effectful")], &context);

    assert!(has_rule(&report, "QSEM0701"));
    Ok(())
}

#[test]
fn rejects_malformed_operation_contract_metadata() {
    let mut operation = contract("bad_metadata");
    operation.allowed_modifiers.insert("".to_owned());

    assert!(matches!(
        validator([operation]),
        Err(QuantumAnalysisFailure::InvalidContext { .. })
    ));
}

#[test]
fn reports_broken_shared_type_service_as_incomplete_analysis() -> TestResult {
    let validator = validator([contract("type_service")])?;
    let mut context = TestContext::default();
    context
        .incompatible_type_pairs
        .insert((ty("logical-qubit"), ty("logical-qubit")));

    let report =
        validator.validate(&[invocation("type_service")], &context);

    assert_eq!(report.completion, QuantumCompletion::Incomplete);
    assert!(has_rule(&report, "QSEM0202"));
    assert!(!report.is_valid());
    Ok(())
}

#[test]
fn preserves_input_order_and_diagnostic_order_deterministically() -> TestResult {
    let validator = validator([contract("first"), contract("second")])?;
    let calls = [invocation("second"), invocation("first")];
    let context = TestContext::default();

    let first_run = validate_quantum_invocations(&validator, &calls, &context);
    let second_run = validate_quantum_invocations(&validator, &calls, &context);

    assert_eq!(first_run, second_run);
    assert_eq!(first_run.invocations[0].operation.name(), "second");
    assert_eq!(first_run.invocations[1].operation.name(), "first");
    Ok(())
}

#[test]
fn empty_input_is_a_complete_valid_analysis() -> TestResult {
    let validator = validator([])?;
    let report = validator.validate(&[], &TestContext::default());

    assert_eq!(report.completion, QuantumCompletion::Complete);
    assert!(report.is_complete());
    assert!(report.is_valid());
    assert!(report.invocations.is_empty());
    assert!(report.issues.is_empty());
    Ok(())
}

#[test]
fn configuration_can_disable_only_the_checks_it_owns() -> TestResult {
    let mut operation = contract("measurement");
    operation.performs_measurement = true;
    operation.results.push(ty("measurement-result"));

    let options = QuantumValidationOptions {
        check_effects: false,
        check_capabilities: false,
        check_boundaries: false,
        check_target_aliasing: true,
    };

    let validator = QuantumValidator::with_options([operation], options)?;
    let mut call = invocation("measurement");
    call.results.push(ResultBinding {
        node: node(3),
        ty: ty("measurement-result"),
    });

    let mut context = TestContext::default();
    context
        .denied_boundaries
        .insert(QuantumBoundaryDirection::QuantumToClassical);

    let report = validator.validate(&[call], &context);
    assert!(!has_rule(&report, "QSEM0602"));
    Ok(())
}

#[test]
fn normative_quantum_grammar_contracts_remain_present() {
    const OPERATIONS: &str =
        include_str!("../../../../grammar/quantum/operations.g4");
    const PARAMETERS: &str =
        include_str!("../../../../grammar/quantum/parameters.g4");
    const MEASUREMENT: &str =
        include_str!("../../../../grammar/quantum/measurement.g4");
    const CONTROLS: &str =
        include_str!("../../../../grammar/quantum/controls.g4");
    const RESOURCES: &str =
        include_str!("../../../../grammar/quantum/quantum-resources.g4");
    const SEMANTIC_MODEL: &str =
        include_str!("../../../../grammar/specification/semantic-model.md");
    const DOMAINS: &str =
        include_str!("../../../../grammar/specification/domains.md");

    assert!(OPERATIONS.contains("quantum::ir"));
    assert!(PARAMETERS.contains("quantum::ir"));
    assert!(MEASUREMENT.contains("quantum::ir"));
    assert!(CONTROLS.contains("quantum::ir"));
    assert!(RESOURCES.contains("quantum::ir"));
    assert!(SEMANTIC_MODEL.contains("semantic"));
    assert!(DOMAINS.contains("quantum"));
}

#[test]
fn grammar_resource_contract_does_not_turn_capacity_into_source_validity() {
    const RESOURCES: &str =
        include_str!("../../../../grammar/quantum/quantum-resources.g4");

    // The grammar contract explicitly separates resource intent from physical
    // realization. These assertions guard that architectural statement; they
    // do not substitute for ANTLR or parser conformance tests.
    assert!(RESOURCES.contains("WHAT THE PROGRAM REQUIRES"));
    assert!(RESOURCES.contains("Actual feasibility is determined downstream"));
    assert!(RESOURCES.contains("MAX_QUBITS"));
}
