
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI — Abductive Reasoning
//!
//! Path: src/ai/reasoning/abduction.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Provides the AI-domain representation and structural validation
//! interfaces for abductive reasoning.
//!
//! Abduction is the process of proposing a candidate explanation or
//! hypothesis from observations or evidence.
//!
//! Conceptually:
//!
//!     observations / evidence
//!              |
//!              v
//!       abductive operation
//!              |
//!              v
//!       candidate hypothesis
//!
//! This module represents computational intent.
//!
//! It does not execute an abductive algorithm or establish that a
//! hypothesis is true.
//!
//! ============================================================================
//! ARCHITECTURAL AUTHORITY
//! ============================================================================
//!
//! Canonical source syntax:
//!
//!     grammar/statements/abduction.g4
//!
//! AI grammar adapter:
//!
//!     grammar/ai/abduction.g4
//!
//! AI grammar composition:
//!
//!     grammar/ai/ai.g4
//!
//! Universal grammar:
//!
//!     grammar/statements/abduction.g4
//!
//! Canonical AST:
//!
//!     src/frontend/ast/
//!
//! Canonical AST identity:
//!
//!     src/frontend/ast/node/node_id.rs
//!
//! AI semantic authority:
//!
//!     src/frontend/semantic/domains/ai.rs
//!
//! Shared semantic infrastructure:
//!
//!     src/frontend/semantic/types/
//!     src/frontend/semantic/effects/
//!     src/frontend/semantic/capabilities/
//!     src/frontend/semantic/resources/
//!     src/frontend/semantic/model/provenance.rs
//!
//! AI error authority:
//!
//!     src/ai/error.rs
//!
//! AI provenance adapter:
//!
//!     src/ai/provenance.rs
//!
//! Universal evidence syntax:
//!
//!     grammar/expressions/evidence.g4
//!
//! Universal uncertainty syntax:
//!
//!     grammar/expressions/uncertainty.g4
//!
//! Knowledge authority:
//!
//!     Existing knowledge infrastructure.
//!
//! Universal intermediate representation:
//!
//!     src/zuir/
//!
//! AI intermediate representation:
//!
//!     src/zuir/dialects/ai/
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! THIS FILE OWNS:
//!
//! - Abductive statement descriptions.
//! - Hypothesis expression references.
//! - Evidence expression references.
//! - Optional abductive context references.
//! - Abductive program composition.
//! - Canonical AST reference validation.
//! - Duplicate abductive construct detection.
//! - Deterministic structural validation reports.
//!
//! THIS FILE DOES NOT OWN:
//!
//! - Source syntax.
//! - Lexical tokens.
//! - Parsing.
//! - Canonical AST construction.
//! - A competing expression tree.
//! - General semantic type checking.
//! - General name resolution.
//! - Bayesian inference algorithms.
//! - Probabilistic computation.
//! - Theorem proving.
//! - SAT/SMT solving.
//! - Hypothesis scoring.
//! - Confidence calculation.
//! - Evidence storage.
//! - Evidence authentication.
//! - Knowledge graph storage.
//! - Knowledge persistence.
//! - Uncertainty propagation.
//! - Explanation storage.
//! - Automatic differentiation.
//! - Tensor storage.
//! - Universal IR.
//! - Hardware discovery.
//! - Resource allocation.
//! - Scheduling.
//! - Routing.
//! - Distributed transport.
//! - Runtime execution.
//!
//! ============================================================================
//! POCO-REAF / SCALABILITY
//! ============================================================================
//!
//! This module introduces no language-level maximum for:
//!
//! - abductive statements;
//! - hypotheses;
//! - observations;
//! - evidence sources;
//! - context expressions;
//! - reasoning operations;
//! - reasoning participants;
//! - models;
//! - datasets;
//! - computational targets;
//! - hardware resources.
//!
//! Collections grow according to supplied input and available resources.
//!
//! No fixed hardware capacity, vendor, processor count, accelerator count,
//! evidence count, hypothesis count, or reasoning depth is assumed.
//!
//! Operational budgets belong to the responsible compiler, semantic
//! configuration, or execution infrastructure.
//!
//! Literal infinite computation is not promised. The objective is to
//! eliminate artificial language-level scalability ceilings.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! - Canonical AST identities are preserved.
//! - No global mutable state is introduced.
//! - No randomness is introduced.
//! - No wall-clock dependency is introduced.
//! - Validation follows deterministic collection order.
//! - Validation does not execute abductive reasoning.
//!
//! ============================================================================
//! INTEGRATION
//! ============================================================================
//!
//! Source:
//!
//!     abduce hypothesis from observations;
//!
//!          |
//!          v
//!
//! Universal AbductionStatements grammar
//!
//!          |
//!          v
//!
//! Canonical frontend AST
//!
//!          |
//!          v
//!
//! Frontend abductive adapter
//!
//!          |
//!          v
//!
//! AbductiveProgram
//!
//!          |
//!          v
//!
//! Canonical AST structural validation
//!
//!          |
//!          v
//!
//! frontend/semantic/domains/ai.rs
//!
//!          |
//!          v
//!
//! Canonical semantic model
//!
//!          |
//!          v
//!
//! ZUIR / appropriate domain IR
//!
//!          |
//!          v
//!
//! Optimization / lowering
//!
//!          |
//!          v
//!
//! Appropriate computational realization
//!
//! ============================================================================

use std::collections::BTreeSet;

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
};

use crate::frontend::ast::node::node_id::NodeId;

// ============================================================================
// CANONICAL EXPRESSION REFERENCES
// ============================================================================

/// A reference to an expression in the canonical Zamani frontend AST.
///
/// This type does not contain or construct another expression tree.
///
/// The frontend adapter is responsible for obtaining the NodeId from
/// the actual parsed AST.
///
/// Existence and expression classification are checked by
/// AbductionNodeResolver during validation.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct AbductionExpressionRef {
    node_id: NodeId,
}

impl AbductionExpressionRef {
    /// Construct a reference to a canonical AST expression.
    ///
    /// This does not establish that the node exists or is an expression.
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self {
            node_id,
        }
    }

    /// Return the canonical AST node identity.
    #[must_use]
    pub const fn node_id(self) -> NodeId {
        self.node_id
    }
}

// ============================================================================
// ABDUCTIVE STATEMENT
// ============================================================================

/// A universal abductive reasoning statement represented at the AI-domain
/// boundary.
///
/// Its structure corresponds to the canonical grammar:
///
///     abduce HYPOTHESIS from OBSERVATIONS;
///
///     abduce HYPOTHESIS from OBSERVATIONS with (CONTEXT);
///
/// The hypothesis, observations and context are references to expressions
/// in the canonical frontend AST.
///
/// Their actual semantic types and meanings belong to the frontend
/// semantic subsystem.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbductionStatement {
    source_node: NodeId,

    hypothesis: AbductionExpressionRef,

    evidence: AbductionExpressionRef,

    context: Vec<AbductionExpressionRef>,
}

impl AbductionStatement {
    /// Construct an abductive statement.
    ///
    /// The hypothesis and evidence references are mandatory because
    /// the canonical grammar requires both.
    ///
    /// The source node must identify the canonical abductive construct.
    /// This is checked during structural validation.
    #[must_use]
    pub const fn new(
        source_node: NodeId,
        hypothesis: AbductionExpressionRef,
        evidence: AbductionExpressionRef,
    ) -> Self {
        Self {
            source_node,
            hypothesis,
            evidence,
            context: Vec::new(),
        }
    }

    /// Construct an abductive statement with explicit context.
    ///
    /// An empty context is rejected because the canonical grammar
    /// requires at least one expression inside the context clause.
    pub fn with_context(
        mut self,
        context: Vec<AbductionExpressionRef>,
    ) -> AiResult<Self> {
        if context.is_empty() {
            return Err(abduction_error(
                AiErrorKind::InvalidInput,
                "an explicit abductive context must contain at least one expression",
            ));
        }

        self.context = context;

        Ok(self)
    }

    /// Return the canonical AST node identifying this statement.
    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    /// Return the candidate hypothesis expression.
    #[must_use]
    pub const fn hypothesis(&self) -> AbductionExpressionRef {
        self.hypothesis
    }

    /// Return the evidence or observation expression.
    #[must_use]
    pub const fn evidence(&self) -> AbductionExpressionRef {
        self.evidence
    }

    /// Return the optional context expressions.
    #[must_use]
    pub fn context(&self) -> &[AbductionExpressionRef] {
        &self.context
    }

    /// Return whether an explicit context clause exists.
    #[must_use]
    pub fn has_context(&self) -> bool {
        !self.context.is_empty()
    }
}

// ============================================================================
// CANONICAL AST RESOLUTION
// ============================================================================

/// Canonical frontend AST lookup interface.
///
/// The actual frontend integration implements this interface using
/// the AST belonging to the current compilation.
///
/// This module does not create:
///
/// - an AST registry;
/// - an AST allocator;
/// - an expression parser;
/// - an expression hierarchy;
/// - a competing AST storage system.
///
/// Implementations must answer consistently for the lifetime of one
/// validation operation.
pub trait AbductionNodeResolver {
    /// Return true only when the node exists in the current canonical AST
    /// and represents an expression.
    fn is_expression_node(
        &self,
        node_id: NodeId,
    ) -> bool;

    /// Return true only when the node exists in the current canonical AST
    /// and represents an abductive reasoning construct.
    fn is_abduction_construct_node(
        &self,
        node_id: NodeId,
    ) -> bool;
}

// ============================================================================
// ABDUCTIVE PROGRAM
// ============================================================================

/// A collection of abductive reasoning intentions.
///
/// This is not:
///
/// - a frontend AST;
/// - a semantic type system;
/// - a knowledge database;
/// - an executable reasoning engine;
/// - a universal intermediate representation.
///
/// Every expression reference points to the canonical frontend AST.
///
/// The collection has no language-level statement limit.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AbductiveProgram {
    statements: Vec<AbductionStatement>,
}

impl AbductiveProgram {
    /// Construct an empty abductive program.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return all abductive statements in insertion order.
    #[must_use]
    pub fn statements(&self) -> &[AbductionStatement] {
        &self.statements
    }

    /// Add an abductive statement.
    ///
    /// Structural correctness is established by validate().
    pub fn add_statement(
        &mut self,
        statement: AbductionStatement,
    ) {
        self.statements.push(statement);
    }

    /// Validate abductive structure and canonical AST references.
    ///
    /// Checks:
    ///
    /// - abductive construct identities;
    /// - canonical construct existence;
    /// - hypothesis expression references;
    /// - evidence expression references;
    /// - optional context expression references;
    /// - duplicate construct identities;
    /// - valid explicit context structure.
    ///
    /// This function does not establish:
    ///
    /// - whether the hypothesis is logically possible;
    /// - whether evidence supports the hypothesis;
    /// - whether evidence is trustworthy;
    /// - whether the hypothesis is true;
    /// - whether a reasoning algorithm exists;
    /// - whether a target can execute the operation.
    ///
    /// Those responsibilities belong to semantic analysis and
    /// downstream reasoning implementations.
    pub fn validate<R: AbductionNodeResolver>(
        &self,
        resolver: &R,
    ) -> AiResult<AbductionValidationReport> {
        let mut report = AbductionValidationReport::default();

        let mut construct_ids = BTreeSet::new();

        for statement in &self.statements {
            // ------------------------------------------------------------
            // Construct identity
            // ------------------------------------------------------------

            if !construct_ids.insert(statement.source_node) {
                return Err(abduction_error(
                    AiErrorKind::Conflict,
                    "a canonical AST node is registered as multiple abductive constructs",
                ));
            }

            // ------------------------------------------------------------
            // Canonical abductive construct
            // ------------------------------------------------------------

            if !resolver.is_abduction_construct_node(
                statement.source_node,
            ) {
                return Err(abduction_error(
                    AiErrorKind::InvalidInput,
                    "abductive statement does not reference a valid canonical AST construct",
                ));
            }

            report.record_statement()?;

            // ------------------------------------------------------------
            // Hypothesis
            // ------------------------------------------------------------

            validate_expression(
                resolver,
                statement.hypothesis,
                &mut report,
            )?;

            // ------------------------------------------------------------
            // Evidence / observations
            // ------------------------------------------------------------

            validate_expression(
                resolver,
                statement.evidence,
                &mut report,
            )?;

            // ------------------------------------------------------------
            // Optional context
            // ------------------------------------------------------------

            if statement.has_context() {
                if statement.context.is_empty() {
                    return Err(abduction_error(
                        AiErrorKind::InvalidInput,
                        "an explicit abductive context cannot be empty",
                    ));
                }

                for expression in &statement.context {
                    validate_expression(
                        resolver,
                        *expression,
                        &mut report,
                    )?;
                }
            }
        }

        report.complete = true;

        Ok(report)
    }
}

// ============================================================================
// STRUCTURAL VALIDATION
// ============================================================================

/// Validate one expression reference against the canonical frontend AST.
fn validate_expression<R: AbductionNodeResolver>(
    resolver: &R,
    expression: AbductionExpressionRef,
    report: &mut AbductionValidationReport,
) -> AiResult<()> {
    if !resolver.is_expression_node(expression.node_id) {
        return Err(abduction_error(
            AiErrorKind::InvalidInput,
            "abductive expression reference does not identify a valid canonical AST expression",
        ));
    }

    report.record_expression_reference()?;

    Ok(())
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Deterministic structural-validation report.
///
/// A complete report means that all structural checks implemented by
/// this module succeeded.
///
/// It does not certify mathematical correctness, semantic validity,
/// evidence reliability, or execution feasibility.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AbductionValidationReport {
    statements_checked: usize,

    expression_references_checked: usize,

    complete: bool,
}

impl AbductionValidationReport {
    /// Record a successfully validated abductive statement.
    fn record_statement(&mut self) -> AiResult<()> {
        self.statements_checked = self
            .statements_checked
            .checked_add(1)
            .ok_or_else(|| {
                abduction_error(
                    AiErrorKind::ResourceExhausted,
                    "abductive statement validation counter exhausted its representation",
                )
            })?;

        Ok(())
    }

    /// Record a successfully validated expression reference.
    fn record_expression_reference(&mut self) -> AiResult<()> {
        self.expression_references_checked = self
            .expression_references_checked
            .checked_add(1)
            .ok_or_else(|| {
                abduction_error(
                    AiErrorKind::ResourceExhausted,
                    "abductive expression validation counter exhausted its representation",
                )
            })?;

        Ok(())
    }

    /// Return the number of successfully validated statements.
    #[must_use]
    pub const fn statements_checked(&self) -> usize {
        self.statements_checked
    }

    /// Return the number of successfully validated expression references.
    #[must_use]
    pub const fn expression_references_checked(&self) -> usize {
        self.expression_references_checked
    }

    /// Return whether structural validation completed successfully.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

/// Construct a structured abductive reasoning error.
///
/// The shared AI error infrastructure remains authoritative.
fn abduction_error(
    kind: AiErrorKind,
    message: impl Into<String>,
) -> AiError {
    AiError::new(kind, message)
        .with_phase(AiErrorPhase::Reasoning)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    /// Test-only implementation of canonical AST resolution.
    struct TestResolver {
        expressions: BTreeSet<NodeId>,

        constructs: BTreeSet<NodeId>,
    }

    impl TestResolver {
        fn new() -> Self {
            Self {
                expressions: BTreeSet::new(),
                constructs: BTreeSet::new(),
            }
        }

        fn expression(&mut self, id: u64) -> NodeId {
            let id = NodeId::new(id)
                .expect("test expression identity must be nonzero");

            self.expressions.insert(id);

            id
        }

        fn construct(&mut self, id: u64) -> NodeId {
            let id = NodeId::new(id)
                .expect("test construct identity must be nonzero");

            self.constructs.insert(id);

            id
        }
    }

    impl AbductionNodeResolver for TestResolver {
        fn is_expression_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.expressions.contains(&node_id)
        }

        fn is_abduction_construct_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.constructs.contains(&node_id)
        }
    }

    fn expression(
        node_id: NodeId,
    ) -> AbductionExpressionRef {
        AbductionExpressionRef::new(node_id)
    }

    fn statement(
        resolver: &mut TestResolver,
        construct_id: u64,
        hypothesis_id: u64,
        evidence_id: u64,
    ) -> AbductionStatement {
        let construct = resolver.construct(construct_id);

        let hypothesis = resolver.expression(hypothesis_id);

        let evidence = resolver.expression(evidence_id);

        AbductionStatement::new(
            construct,
            expression(hypothesis),
            expression(evidence),
        )
    }

    // ------------------------------------------------------------------------
    // Construction
    // ------------------------------------------------------------------------

    #[test]
    fn constructs_abductive_statement() {
        let mut resolver = TestResolver::new();

        let statement = statement(
            &mut resolver,
            1,
            2,
            3,
        );

        assert_eq!(
            statement.source_node(),
            NodeId::new(1).unwrap()
        );

        assert_eq!(
            statement.hypothesis().node_id(),
            NodeId::new(2).unwrap()
        );

        assert_eq!(
            statement.evidence().node_id(),
            NodeId::new(3).unwrap()
        );

        assert!(!statement.has_context());
    }

    #[test]
    fn accepts_nonempty_abductive_context() {
        let mut resolver = TestResolver::new();

        let mut statement = statement(
            &mut resolver,
            1,
            2,
            3,
        );

        let first = resolver.expression(4);
        let second = resolver.expression(5);

        statement = statement
            .with_context(vec![
                expression(first),
                expression(second),
            ])
            .unwrap();

        assert!(statement.has_context());

        assert_eq!(statement.context().len(), 2);
    }

    #[test]
    fn rejects_explicit_empty_context() {
        let mut resolver = TestResolver::new();

        let statement = statement(
            &mut resolver,
            1,
            2,
            3,
        );

        let result = statement.with_context(Vec::new());

        assert!(result.is_err());

        let error = result.unwrap_err();

        assert_eq!(
            error.kind(),
            AiErrorKind::InvalidInput
        );

        assert_eq!(
            error.context().phase(),
            Some(AiErrorPhase::Reasoning)
        );
    }

    // ------------------------------------------------------------------------
    // Canonical AST validation
    // ------------------------------------------------------------------------

    #[test]
    fn validates_abductive_statement_without_context() {
        let mut resolver = TestResolver::new();

        let statement = statement(
            &mut resolver,
            1,
            2,
            3,
        );

        let mut program = AbductiveProgram::new();

        program.add_statement(statement);

        let report = program.validate(&resolver).unwrap();

        assert!(report.is_complete());

        assert_eq!(
            report.statements_checked(),
            1
        );

        assert_eq!(
            report.expression_references_checked(),
            2
        );
    }

    #[test]
    fn validates_abductive_statement_with_context() {
        let mut resolver = TestResolver::new();

        let mut statement = statement(
            &mut resolver,
            1,
            2,
            3,
        );

        let context = resolver.expression(4);

        statement = statement
            .with_context(vec![
                expression(context),
            ])
            .unwrap();

        let mut program = AbductiveProgram::new();

        program.add_statement(statement);

        let report = program.validate(&resolver).unwrap();

        assert!(report.is_complete());

        assert_eq!(
            report.expression_references_checked(),
            3
        );
    }

    #[test]
    fn rejects_unknown_abductive_construct() {
        let resolver = TestResolver::new();

        let statement = AbductionStatement::new(
            NodeId::new(1).unwrap(),
            expression(NodeId::new(2).unwrap()),
            expression(NodeId::new(3).unwrap()),
        );

        let mut program = AbductiveProgram::new();

        program.add_statement(statement);

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn rejects_unknown_hypothesis_expression() {
        let mut resolver = TestResolver::new();

        let construct = resolver.construct(1);

        let evidence = resolver.expression(3);

        let statement = AbductionStatement::new(
            construct,
            expression(NodeId::new(2).unwrap()),
            expression(evidence),
        );

        let mut program = AbductiveProgram::new();

        program.add_statement(statement);

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn rejects_unknown_evidence_expression() {
        let mut resolver = TestResolver::new();

        let construct = resolver.construct(1);

        let hypothesis = resolver.expression(2);

        let statement = AbductionStatement::new(
            construct,
            expression(hypothesis),
            expression(NodeId::new(3).unwrap()),
        );

        let mut program = AbductiveProgram::new();

        program.add_statement(statement);

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn rejects_unknown_context_expression() {
        let mut resolver = TestResolver::new();

        let mut statement = statement(
            &mut resolver,
            1,
            2,
            3,
        );

        statement = statement
            .with_context(vec![
                expression(NodeId::new(99).unwrap()),
            ])
            .unwrap();

        let mut program = AbductiveProgram::new();

        program.add_statement(statement);

        assert!(program.validate(&resolver).is_err());
    }

    // ------------------------------------------------------------------------
    // Identity and structural invariants
    // ------------------------------------------------------------------------

    #[test]
    fn rejects_duplicate_abductive_construct_identity() {
        let mut resolver = TestResolver::new();

        let first = statement(
            &mut resolver,
            1,
            2,
            3,
        );

        let second = statement(
            &mut resolver,
            1,
            4,
            5,
        );

        let mut program = AbductiveProgram::new();

        program.add_statement(first);

        program.add_statement(second);

        let result = program.validate(&resolver);

        assert!(result.is_err());

        assert_eq!(
            result.unwrap_err().kind(),
            AiErrorKind::Conflict
        );
    }

    #[test]
    fn permits_reusing_evidence_across_distinct_statements() {
        let mut resolver = TestResolver::new();

        let shared_evidence = resolver.expression(10);

        let first_construct = resolver.construct(1);
        let first_hypothesis = resolver.expression(2);

        let second_construct = resolver.construct(3);
        let second_hypothesis = resolver.expression(4);

        let first = AbductionStatement::new(
            first_construct,
            expression(first_hypothesis),
            expression(shared_evidence),
        );

        let second = AbductionStatement::new(
            second_construct,
            expression(second_hypothesis),
            expression(shared_evidence),
        );

        let mut program = AbductiveProgram::new();

        program.add_statement(first);
        program.add_statement(second);

        let report = program.validate(&resolver).unwrap();

        assert!(report.is_complete());

        assert_eq!(
            report.statements_checked(),
            2
        );

        assert_eq!(
            report.expression_references_checked(),
            4
        );
    }

    #[test]
    fn accepts_empty_abductive_program() {
        let resolver = TestResolver::new();

        let program = AbductiveProgram::new();

        let report = program.validate(&resolver).unwrap();

        assert!(report.is_complete());

        assert_eq!(
            report.statements_checked(),
            0
        );

        assert_eq!(
            report.expression_references_checked(),
            0
        );
    }

    // ------------------------------------------------------------------------
    // Scalability
    // ------------------------------------------------------------------------

    #[test]
    fn supports_growing_abductive_statement_collections() {
        let mut resolver = TestResolver::new();

        let mut program = AbductiveProgram::new();

        for index in 0_u64..1024 {
            let base = index
                .checked_mul(3)
                .and_then(|value| value.checked_add(1))
                .unwrap();

            let construct_id = base;
            let hypothesis_id = base + 1;
            let evidence_id = base + 2;

            let statement = statement(
                &mut resolver,
                construct_id,
                hypothesis_id,
                evidence_id,
            );

            program.add_statement(statement);
        }

        let report = program.validate(&resolver).unwrap();

        assert!(report.is_complete());

        assert_eq!(
            report.statements_checked(),
            1024
        );

        assert_eq!(
            report.expression_references_checked(),
            2048
        );
    }

    #[test]
    fn does_not_require_unique_evidence_or_hypothesis_identity() {
        let mut resolver = TestResolver::new();

        let shared = resolver.expression(10);

        let construct = resolver.construct(1);

        let statement = AbductionStatement::new(
            construct,
            expression(shared),
            expression(shared),
        );

        let mut program = AbductiveProgram::new();

        program.add_statement(statement);

        assert!(
            program.validate(&resolver).is_ok()
        );
    }

    // ------------------------------------------------------------------------
    // Error classification
    // ------------------------------------------------------------------------

    #[test]
    fn invalid_ast_references_are_reasoning_errors() {
        let resolver = TestResolver::new();

        let statement = AbductionStatement::new(
            NodeId::new(1).unwrap(),
            expression(NodeId::new(2).unwrap()),
            expression(NodeId::new(3).unwrap()),
        );

        let mut program = AbductiveProgram::new();

        program.add_statement(statement);

        let error = program.validate(&resolver).unwrap_err();

        assert_eq!(
            error.kind(),
            AiErrorKind::InvalidInput
        );

        assert_eq!(
            error.context().phase(),
            Some(AiErrorPhase::Reasoning)
        );
    }

    #[test]
    fn structural_validation_does_not_claim_hypothesis_truth() {
        let mut resolver = TestResolver::new();

        let statement = statement(
            &mut resolver,
            1,
            2,
            3,
        );

        let mut program = AbductiveProgram::new();

        program.add_statement(statement);

        let report = program.validate(&resolver).unwrap();

        // Structural completeness is not a mathematical proof.
        assert!(report.is_complete());
    }
}
