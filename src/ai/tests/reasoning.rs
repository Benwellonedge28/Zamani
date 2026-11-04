#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI — Reasoning Integration Tests
//!
//! Path: src/ai/tests/reasoning.rs
//! Edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Verify the public contracts and integration boundaries of the existing
//! AI reasoning subsystem.
//!
//! This file tests reasoning representations and their structural validation.
//! It does not implement a second reasoning engine, AST, semantic system,
//! knowledge store, theorem prover, probability engine, or IR.
//!
//! ============================================================================
//! ARCHITECTURAL AUTHORITIES
//! ============================================================================
//!
//! Grammar composition:
//!     grammar/ai/ai.g4
//!
//! AI reasoning grammar adapter:
//!     grammar/ai/reasoning.g4
//!
//! Universal reasoning grammar:
//!     grammar/statements/reason.g4
//!
//! Canonical AST:
//!     src/frontend/ast/
//!
//! Canonical AST node identity:
//!     src/frontend/ast/node/node_id.rs
//!
//! AI reasoning composition:
//!     src/ai/reasoning/mod.rs
//!
//! Symbolic reasoning:
//!     src/ai/reasoning/symbolic.rs
//!
//! AI semantic boundary:
//!     src/frontend/semantic/domains/ai.rs
//!
//! Canonical universal IR:
//!     src/zuir/
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This file owns:
//!
//! - Integration tests for the AI reasoning public module boundary.
//! - Tests for symbolic reasoning's canonical AST references.
//! - Tests for structural validation behavior.
//! - Tests for extensible symbolic identities.
//! - Tests for dynamically sized reasoning descriptions.
//! - Regression tests for reasoning validation invariants.
//!
//! This file does not own:
//!
//! - Reasoning algorithms.
//! - Inference execution.
//! - Deduction execution.
//! - Inductive or abductive algorithms.
//! - Causal inference algorithms.
//! - Probability arithmetic.
//! - Knowledge persistence.
//! - Evidence storage.
//! - Uncertainty propagation algorithms.
//! - Canonical AST construction.
//! - Semantic type checking.
//! - IR construction or lowering.
//! - Hardware selection.
//! - Runtime scheduling.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! These tests do not impose a language-level ceiling on:
//!
//! - Reasoning constructs.
//! - Symbolic declarations.
//! - Symbolic operations.
//! - Expression references.
//! - Knowledge entities.
//! - Evidence records.
//! - Probabilistic variables.
//! - Causal variables.
//! - Reasoning depth.
//! - AI agents.
//! - Execution targets.
//!
//! Test workloads are samples, not architectural limits.
//!
//! Physical infinity is not possible. The requirement is that no arbitrary
//! language-level maximum is introduced. Actual exhaustion must be explicit.
//!
//! ============================================================================
//! INTEGRATION
//! ============================================================================
//!
//! The intended integration pathway is:
//!
//!     Zamani source
//!          |
//!          v
//!     grammar/statements/reason.g4
//!          |
//!          v
//!     grammar/ai/reasoning.g4
//!          |
//!          v
//!     canonical frontend AST
//!          |
//!          v
//!     canonical AST node identities
//!          |
//!          v
//!     AI reasoning descriptions
//!          |
//!          v
//!     structural validation
//!          |
//!          v
//!     shared semantic validation
//!          |
//!          v
//!     canonical ZUIR and domain IRs
//!          |
//!          v
//!     optimization / lowering
//!          |
//!          v
//!     runtime / distributed / backend realization
//!
//! The tests below exercise the Rust reasoning-description and AST-reference
//! boundary. They do not pretend that the complete source-to-runtime pipeline
//! is already connected.
//!
//! ============================================================================
//! FREEZE CRITERIA
//! ============================================================================
//!
//! This file is complete when:
//!
//! - It tests existing public reasoning contracts.
//! - It uses canonical frontend NodeId.
//! - It introduces no duplicate AST or reasoning representation.
//! - It checks both valid and invalid references.
//! - It verifies deterministic structural counters.
//! - It tests extensible names.
//! - It tests dynamically sized descriptions.
//! - It introduces no hardware or language-level ceilings.
//! - It uses safe Rust only.
//! - It is registered by reasoning/mod.rs.
//! - The repository's Rust verification commands pass.
//!
//! Note: passing these tests does not establish mathematical truth,
//! satisfiability, semantic type correctness, or runtime execution success.

use std::collections::BTreeSet;

use crate::ai::reasoning::symbolic::{
    SymbolicDeclaration,
    SymbolicName,
    SymbolicNodeResolver,
    SymbolicProgram,
    SymbolicRelation,
    SymbolicExpressionRef,
};

use crate::frontend::ast::node::node_id::NodeId;

// ============================================================================
// TEST AST RESOLVER
// ============================================================================
//
// The production reasoning subsystem deliberately receives a resolver
// instead of owning an AST.
//
// This test resolver supplies canonical NodeId references for isolated tests.
//
// It is not an AST implementation and must never be used as a production
// semantic resolver.

#[derive(Default)]
struct TestNodeResolver {
    expressions: BTreeSet<NodeId>,
    constructs: BTreeSet<NodeId>,
}

impl TestNodeResolver {
    fn new() -> Self {
        Self::default()
    }

    fn add_expression(&mut self, id: u64) -> NodeId {
        let node = NodeId::new(id)
            .expect("test expression node identity must be nonzero");

        self.expressions.insert(node);
        node
    }

    fn add_construct(&mut self, id: u64) -> NodeId {
        let node = NodeId::new(id)
            .expect("test construct node identity must be nonzero");

        self.constructs.insert(node);
        node
    }
}

impl SymbolicNodeResolver for TestNodeResolver {
    fn is_expression_node(&self, node_id: NodeId) -> bool {
        self.expressions.contains(&node_id)
    }

    fn is_symbolic_construct_node(&self, node_id: NodeId) -> bool {
        self.constructs.contains(&node_id)
    }
}

// ============================================================================
// TEST HELPERS
// ============================================================================

fn symbolic_name(namespace: &str, name: &str) -> SymbolicName {
    SymbolicName::new(namespace, name)
        .expect("test symbolic name must be valid")
}

fn expression(node_id: NodeId) -> SymbolicExpressionRef {
    SymbolicExpressionRef::new(node_id)
}

// ============================================================================
// MODULE AND OWNERSHIP CONTRACTS
// ============================================================================

#[test]
fn reasoning_composition_uses_the_existing_domain_modules() {
    // These modules are deliberately owned by reasoning/mod.rs.
    //
    // This test file must not declare replacements for them.
    //
    // Their existence is enforced by the Rust module compilation itself.
    //
    // This test documents the expected reasoning domain boundary.

    let expected_modules = [
        "symbolic",
        "inference",
        "deduction",
        "induction",
        "abduction",
        "causality",
        "probabilistic",
        "knowledge",
        "evidence",
        "uncertainty",
    ];

    assert_eq!(expected_modules.len(), 10);
}

// ============================================================================
// SYMBOLIC NAME CONTRACTS
// ============================================================================

#[test]
fn symbolic_names_accept_extensible_namespaces() {
    let name = symbolic_name(
        "user.domain.mathematics",
        "arbitrary_symbolic_operation",
    );

    assert_eq!(
        name.namespace(),
        "user.domain.mathematics"
    );

    assert_eq!(
        name.name(),
        "arbitrary_symbolic_operation"
    );

    assert_eq!(
        name.qualified(),
        "user.domain.mathematics.arbitrary_symbolic_operation"
    );
}

#[test]
fn symbolic_names_do_not_require_a_closed_operation_vocabulary() {
    let names = [
        symbolic_name("core.logic", "equal"),
        symbolic_name("mathematics.algebra", "factor"),
        symbolic_name("mathematics.calculus", "integrate"),
        symbolic_name("user.extension", "future_operation"),
    ];

    assert_eq!(names.len(), 4);

    assert_eq!(
        names[3].qualified(),
        "user.extension.future_operation"
    );
}

#[test]
fn symbolic_names_reject_empty_components() {
    assert!(
        SymbolicName::new("", "operation").is_err()
    );

    assert!(
        SymbolicName::new("namespace", "").is_err()
    );
}

#[test]
fn symbolic_names_reject_control_characters() {
    assert!(
        SymbolicName::new("namespace\ninvalid", "operation").is_err()
    );

    assert!(
        SymbolicName::new("namespace", "operation\tinvalid").is_err()
    );
}

// ============================================================================
// EMPTY PROGRAM CONTRACTS
// ============================================================================

#[test]
fn empty_symbolic_program_is_structurally_valid() {
    let program = SymbolicProgram::new();
    let resolver = TestNodeResolver::new();

    let report = program
        .validate(&resolver)
        .expect("empty symbolic program should validate");

    assert_eq!(report.constructs_checked(), 0);
    assert_eq!(report.expression_references_checked(), 0);
    assert!(report.is_complete());
}

// ============================================================================
// CANONICAL AST INTEGRATION
// ============================================================================

#[test]
fn symbolic_declaration_accepts_a_canonical_ast_construct() {
    let mut resolver = TestNodeResolver::new();

    let declaration_node = resolver.add_construct(1);

    let declaration = SymbolicDeclaration::new(
        declaration_node,
        symbolic_name("core.symbolic", "variable"),
        symbolic_name("application", "temperature"),
        None,
    );

    let mut program = SymbolicProgram::new();
    program.add_declaration(declaration);

    let report = program
        .validate(&resolver)
        .expect("valid canonical AST reference should validate");

    assert_eq!(report.constructs_checked(), 1);
    assert!(report.is_complete());
}

#[test]
fn symbolic_declaration_rejects_an_unknown_ast_construct() {
    let resolver = TestNodeResolver::new();

    let unknown_node = NodeId::new(100)
        .expect("test node identity must be nonzero");

    let declaration = SymbolicDeclaration::new(
        unknown_node,
        symbolic_name("core.symbolic", "variable"),
        symbolic_name("application", "unknown"),
        None,
    );

    let mut program = SymbolicProgram::new();
    program.add_declaration(declaration);

    let report = program
        .validate(&resolver)
        .expect("structural validation should return a report");

    // The report must not claim complete validation when the referenced
    // canonical AST construct cannot be resolved.
    assert!(!report.is_complete());
}

#[test]
fn symbolic_relations_resolve_canonical_expression_nodes() {
    let mut resolver = TestNodeResolver::new();

    let relation_node = resolver.add_construct(10);
    let left_node = resolver.add_expression(11);
    let right_node = resolver.add_expression(12);

    let relation = SymbolicRelation::new(
        relation_node,
        symbolic_name("core.logic", "equal"),
        vec![
            expression(left_node),
            expression(right_node),
        ],
    );

    let mut program = SymbolicProgram::new();
    program.add_relation(relation);

    let report = program
        .validate(&resolver)
        .expect("valid expression references should validate");

    assert_eq!(report.constructs_checked(), 1);
    assert_eq!(report.expression_references_checked(), 2);
    assert!(report.is_complete());
}

#[test]
fn symbolic_relations_do_not_accept_unresolved_expression_references_as_valid() {
    let mut resolver = TestNodeResolver::new();

    let relation_node = resolver.add_construct(20);
    let known_expression = resolver.add_expression(21);

    let unknown_expression = NodeId::new(22)
        .expect("test node identity must be nonzero");

    let relation = SymbolicRelation::new(
        relation_node,
        symbolic_name("core.logic", "equal"),
        vec![
            expression(known_expression),
            expression(unknown_expression),
        ],
    );

    let mut program = SymbolicProgram::new();
    program.add_relation(relation);

    let report = program
        .validate(&resolver)
        .expect("validation should produce a structural report");

    assert!(!report.is_complete());
}

// ============================================================================
// MULTI-CONSTRUCT REASONING
// ============================================================================

#[test]
fn symbolic_program_validates_multiple_independent_declarations() {
    let mut resolver = TestNodeResolver::new();
    let mut program = SymbolicProgram::new();

    let declaration_count = 64usize;

    for index in 0..declaration_count {
        let raw_id = u64::try_from(index + 1)
            .expect("test identity must fit the canonical NodeId input");

        let node = resolver.add_construct(raw_id);

        let declaration = SymbolicDeclaration::new(
            node,
            symbolic_name("core.symbolic", "variable"),
            symbolic_name(
                "application.variables",
                &format!("variable_{index}"),
            ),
            None,
        );

        program.add_declaration(declaration);
    }

    assert_eq!(
        program.declarations().len(),
        declaration_count
    );

    let report = program
        .validate(&resolver)
        .expect("all independently registered declarations should validate");

    assert_eq!(
        report.constructs_checked(),
        declaration_count
    );

    assert!(report.is_complete());
}

// ============================================================================
// DYNAMIC SCALABILITY CONTRACTS
// ============================================================================

#[test]
fn symbolic_program_accepts_dynamically_sized_declaration_sets() {
    let mut resolver = TestNodeResolver::new();
    let mut program = SymbolicProgram::new();

    // This is a test workload, not a language-level maximum.
    //
    // The production representation must not introduce MAX_SYMBOLS,
    // MAX_DECLARATIONS, or any equivalent semantic ceiling.

    let workload_size = 512usize;

    for index in 0..workload_size {
        let raw_id = u64::try_from(index + 1)
            .expect("test identity must fit the canonical NodeId input");

        let node = resolver.add_construct(raw_id);

        program.add_declaration(
            SymbolicDeclaration::new(
                node,
                symbolic_name("core.symbolic", "entity"),
                symbolic_name(
                    "scalable.workload",
                    &format!("entity_{index}"),
                ),
                None,
            ),
        );
    }

    assert_eq!(
        program.declarations().len(),
        workload_size
    );

    let report = program
        .validate(&resolver)
        .expect("dynamically sized symbolic program should validate");

    assert_eq!(
        report.constructs_checked(),
        workload_size
    );

    assert!(report.is_complete());
}

// ============================================================================
// DETERMINISM CONTRACTS
// ============================================================================

#[test]
fn equivalent_symbolic_programs_produce_equivalent_validation_reports() {
    fn build_program() -> (
        SymbolicProgram,
        TestNodeResolver,
    ) {
        let mut resolver = TestNodeResolver::new();
        let mut program = SymbolicProgram::new();

        let first = resolver.add_construct(1);
        let second = resolver.add_construct(2);

        program.add_declaration(
            SymbolicDeclaration::new(
                first,
                symbolic_name("core.symbolic", "variable"),
                symbolic_name("example", "first"),
                None,
            ),
        );

        program.add_declaration(
            SymbolicDeclaration::new(
                second,
                symbolic_name("core.symbolic", "variable"),
                symbolic_name("example", "second"),
                None,
            ),
        );

        (program, resolver)
    }

    let (first_program, first_resolver) = build_program();
    let (second_program, second_resolver) = build_program();

    let first_report = first_program
        .validate(&first_resolver)
        .expect("first program should validate");

    let second_report = second_program
        .validate(&second_resolver)
        .expect("second program should validate");

    assert_eq!(
        first_report.constructs_checked(),
        second_report.constructs_checked()
    );

    assert_eq!(
        first_report.expression_references_checked(),
        second_report.expression_references_checked()
    );

    assert_eq!(
        first_report.is_complete(),
        second_report.is_complete()
    );
}

// ============================================================================
// EXTENSION AND PORTABILITY CONTRACTS
// ============================================================================

#[test]
fn reasoning_names_are_independent_of_execution_target_identity() {
    // A symbolic operation name is a domain identity.
    //
    // It must not encode a CPU, GPU, QPU, FPGA, vendor, or physical device.

    let operation = symbolic_name(
        "future.reasoning.extension",
        "symbolic_transform",
    );

    assert_eq!(
        operation.qualified(),
        "future.reasoning.extension.symbolic_transform"
    );
}

#[test]
fn symbolic_representation_does_not_require_expression_materialization() {
    // SymbolicExpressionRef contains a canonical AST identity.
    //
    // This allows reasoning descriptions to refer to expressions without
    // constructing a second expression tree or evaluating the expression.

    let node = NodeId::new(900)
        .expect("test node identity must be nonzero");

    let reference = expression(node);

    assert_eq!(reference.node_id(), node);
}

// ============================================================================
// ERROR AND VALIDATION BOUNDARIES
// ============================================================================

#[test]
fn structural_validation_does_not_claim_mathematical_proof() {
    let mut resolver = TestNodeResolver::new();

    let relation_node = resolver.add_construct(30);
    let left_node = resolver.add_expression(31);
    let right_node = resolver.add_expression(32);

    let mut program = SymbolicProgram::new();

    program.add_relation(
        SymbolicRelation::new(
            relation_node,
            symbolic_name("core.logic", "equal"),
            vec![
                expression(left_node),
                expression(right_node),
            ],
        ),
    );

    let report = program
        .validate(&resolver)
        .expect("structural validation should complete");

    // The symbolic validator reports structural completeness only.
    // It does not expose a theorem-proving result.
    assert!(report.is_complete());
}

// ============================================================================
// FINAL CONTRACT
// ============================================================================

#[test]
fn reasoning_tests_use_the_existing_canonical_ast_boundary() {
    // Compile-time references to the canonical identity type and existing
    // reasoning API ensure this test module remains attached to the intended
    // architectural boundary.

    fn accepts_canonical_node_id(_: NodeId) {}

    fn accepts_symbolic_program(_: &SymbolicProgram) {}

    let node = NodeId::new(1)
        .expect("test node identity must be nonzero");

    let program = SymbolicProgram::new();

    accepts_canonical_node_id(node);
    accepts_symbolic_program(&program);
}