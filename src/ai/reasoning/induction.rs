
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Inductive Reasoning
//!
//! Path: `src/ai/reasoning/induction.rs`
//!
//! Rust edition: 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the AI-domain representation and structural validation
//! interfaces for inductive reasoning.
//!
//! Induction expresses the derivation of a generalized conclusion
//! from observations, evidence, examples, or other canonical
//! computational expressions.
//!
//! This module preserves references to the canonical frontend AST.
//! It does not construct another AST or implement an induction engine.
//!
//! # Architectural authorities
//!
//! Source grammar:
//! `grammar/ai/induction.g4`
//!
//! AI grammar composition:
//! `grammar/ai/ai.g4`
//!
//! Universal reasoning grammar:
//! `grammar/statements/reason.g4`
//!
//! Reasoning composition:
//! `grammar/ai/reasoning.g4`
//!
//! Canonical AST:
//! `src/frontend/ast/`
//!
//! Canonical AST identity:
//! `src/frontend/ast/node/node_id.rs`
//!
//! AI semantic authority:
//! `src/frontend/semantic/domains/ai.rs`
//!
//! Shared AI errors:
//! `src/ai/error.rs`
//!
//! Related reasoning domains:
//! `src/ai/reasoning/inference.rs`
//! `src/ai/reasoning/deduction.rs`
//! `src/ai/reasoning/symbolic.rs`
//!
//! # Ownership
//!
//! This module owns:
//!
//! - inductive reasoning request descriptions;
//! - canonical reasoning statement references;
//! - canonical expression references;
//! - inductive request collections;
//! - structural reference validation;
//! - inductive semantic-classification verification;
//! - deterministic validation reports.
//!
//! # Does not own
//!
//! This module does not own:
//!
//! - source parsing;
//! - lexer tokens;
//! - grammar definitions;
//! - AST construction;
//! - general expression representations;
//! - general type representations;
//! - name resolution;
//! - reasoning operator syntax;
//! - inference algorithms;
//! - deduction algorithms;
//! - learning algorithms;
//! - model training;
//! - model adaptation;
//! - evidence storage;
//! - knowledge storage;
//! - uncertainty representation;
//! - probability calculations;
//! - provenance storage;
//! - tensor storage;
//! - numerical computation;
//! - hardware discovery;
//! - resource allocation;
//! - scheduling;
//! - distributed transport;
//! - ZUIR construction;
//! - runtime execution.
//!
//! # Induction versus other reasoning domains
//!
//! Induction is distinct from:
//!
//! - inference;
//! - deduction;
//! - abduction;
//! - learning;
//! - adaptation;
//! - probabilistic computation.
//!
//! These domains may compose with induction, but must retain
//! their independently defined semantics.
//!
//! # POCO-REAF
//!
//! This module introduces no language-level maximum for:
//!
//! - inductive requests;
//! - observations;
//! - evidence references;
//! - contextual expressions;
//! - target expressions;
//! - source expressions;
//! - models;
//! - datasets;
//! - reasoning operations;
//! - execution targets.
//!
//! Collections grow according to supplied input and available resources.
//!
//! Rust's finite address space and allocation behavior remain
//! implementation constraints.
//!
//! Literal infinite computation is not promised.
//!
//! Operational limits belong to the responsible compiler,
//! configuration, resource, and execution infrastructure.
//!
//! # Important semantic boundary
//!
//! Structural validation does not establish that a generalization
//! is mathematically justified.
//!
//! It also does not establish that the available evidence is sufficient.
//!
//! These decisions belong to canonical semantic analysis and the
//! registered reasoning/evidence implementations.
//!
//! # Integration pathway
//!
//! ```text
//! grammar/ai/induction.g4
//!          |
//!          v
//! aiInductionConstruct
//!          |
//!          v
//! reasonStatement
//!          |
//!          v
//! canonical frontend AST
//!          |
//!          v
//! semantic induction classification
//!          |
//!          v
//! InductionProgram
//!          |
//!          v
//! canonical semantic validation
//!          |
//!          v
//! evidence / provenance / uncertainty
//!          |
//!          v
//! ZUIR AI dialect
//!          |
//!          v
//! optimization and lowering
//!          |
//!          v
//! target realization
//! ```
//!
//! The AST adapter must preserve canonical NodeId values.
//!
//! The semantic resolver must obtain induction classification
//! from the authoritative semantic analysis result.
//!
//! This module must never infer induction classification merely
//! from the presence of an expression or a valid AST node.
//!
//! # Determinism
//!
//! - Requests preserve insertion/source order.
//! - Context expressions preserve source order.
//! - Duplicate source constructs are rejected.
//! - Validation is deterministic.
//! - No global mutable state is introduced.
//! - No hardware-specific behavior is introduced.
//!
//! # Safety
//!
//! This implementation uses safe Rust only.

use std::collections::BTreeSet;

use crate::ai::error::{
    AiError,
    AiErrorKind,
    AiErrorPhase,
    AiResult,
};

use crate::frontend::ast::node::node_id::NodeId;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn induction_error(
    kind: AiErrorKind,
    message: impl Into<String>,
) -> AiError {
    AiError::new(kind, message)
        .with_phase(AiErrorPhase::Reasoning)
}

// ============================================================================
// CANONICAL AST REFERENCES
// ============================================================================

/// A reference to a canonical reasoning statement.
///
/// This reference identifies the original `reasonStatement`
/// in the frontend AST.
///
/// It does not contain or recreate a statement tree.
///
/// The referenced statement must be checked against the
/// canonical AST and semantic-analysis session.
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
pub struct InductionStatementRef {
    node_id: NodeId,
}

impl InductionStatementRef {
    /// Construct a reference to a canonical reasoning statement.
    ///
    /// Construction does not establish that the referenced node
    /// exists or represents an inductive operation.
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self { node_id }
    }

    /// Return the canonical AST node identity.
    #[must_use]
    pub const fn node_id(self) -> NodeId {
        self.node_id
    }
}

/// A reference to an expression in the canonical frontend AST.
///
/// Expressions are never reconstructed inside this module.
///
/// Their structure, typing and semantics remain owned by
/// the canonical frontend.
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
pub struct InductionExpressionRef {
    node_id: NodeId,
}

impl InductionExpressionRef {
    /// Construct a reference to a canonical expression.
    ///
    /// The reference is checked during structural validation.
    #[must_use]
    pub const fn new(node_id: NodeId) -> Self {
        Self { node_id }
    }

    /// Return the canonical AST node identity.
    #[must_use]
    pub const fn node_id(self) -> NodeId {
        self.node_id
    }
}

// ============================================================================
// INDUCTIVE REQUEST
// ============================================================================

/// A single inductive reasoning request.
///
/// This is a domain-level projection of a canonical reasoning
/// statement, not an independent AST or intermediate representation.
///
/// The fields correspond to the universal reasoning grammar:
///
/// ```text
/// reason TARGET;
///
/// reason TARGET from SOURCE;
///
/// reason TARGET with (CONTEXT);
///
/// reason TARGET from SOURCE with (CONTEXT);
/// ```
///
/// Whether the original reasoning statement is actually inductive
/// is determined by semantic analysis.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InductionRequest {
    statement: InductionStatementRef,

    target: InductionExpressionRef,

    source: Option<InductionExpressionRef>,

    context: Vec<InductionExpressionRef>,
}

impl InductionRequest {
    /// Construct an inductive reasoning request.
    ///
    /// `statement` identifies the canonical reasoning statement.
    ///
    /// `target` identifies the expression receiving the generalized
    /// result.
    ///
    /// `source` optionally identifies the expression supplying the
    /// primary observations or source data.
    ///
    /// `context` contains additional expressions in source order.
    ///
    /// This constructor performs no AST or semantic validation.
    #[must_use]
    pub fn new(
        statement: InductionStatementRef,
        target: InductionExpressionRef,
        source: Option<InductionExpressionRef>,
        context: Vec<InductionExpressionRef>,
    ) -> Self {
        Self {
            statement,
            target,
            source,
            context,
        }
    }

    /// Return the canonical reasoning statement reference.
    #[must_use]
    pub const fn statement(&self) -> InductionStatementRef {
        self.statement
    }

    /// Return the target expression.
    #[must_use]
    pub const fn target(&self) -> InductionExpressionRef {
        self.target
    }

    /// Return the optional source expression.
    #[must_use]
    pub const fn source(
        &self,
    ) -> Option<InductionExpressionRef> {
        self.source
    }

    /// Return additional contextual expressions in source order.
    #[must_use]
    pub fn context(&self) -> &[InductionExpressionRef] {
        &self.context
    }
}

// ============================================================================
// SEMANTIC RESOLUTION CONTRACT
// ============================================================================

/// Resolves induction references against the canonical frontend
/// AST and the authoritative semantic-analysis result.
///
/// Implementations must operate on the AST and semantic session
/// belonging to the current compilation.
///
/// They must not accept an identifier merely because it is
/// numerically valid.
///
/// IMPORTANT:
///
/// `is_inductive_reasoning_statement` must consult the canonical
/// semantic classification.
///
/// It must not classify every `reasonStatement` as induction.
///
/// The semantic subsystem remains responsible for deciding
/// whether a reasoning operation represents inductive reasoning.
///
/// This trait provides read-only validation access. It does not
/// construct or modify AST nodes.
pub trait InductionNodeResolver {
    /// Return true if the node exists and represents a canonical
    /// universal reasoning statement.
    fn is_reason_statement_node(
        &self,
        node_id: NodeId,
    ) -> bool;

    /// Return true if the canonical semantic model has classified
    /// this reasoning statement as an inductive operation.
    ///
    /// The implementation must use authoritative semantic facts,
    /// not source-name heuristics.
    fn is_inductive_reasoning_statement(
        &self,
        node_id: NodeId,
    ) -> bool;

    /// Return true if the node exists and represents a canonical
    /// frontend expression.
    fn is_expression_node(
        &self,
        node_id: NodeId,
    ) -> bool;
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Deterministic structural validation statistics.
///
/// These counters describe the references inspected by this module.
///
/// They do not certify mathematical correctness, evidence sufficiency,
/// probabilistic confidence, or successful execution.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
)]
pub struct InductionValidationReport {
    requests: usize,

    statement_references: usize,

    expression_references: usize,
}

impl InductionValidationReport {
    /// Return the number of validated inductive requests.
    #[must_use]
    pub const fn requests(&self) -> usize {
        self.requests
    }

    /// Return the number of validated reasoning statement references.
    #[must_use]
    pub const fn statement_references(&self) -> usize {
        self.statement_references
    }

    /// Return the number of validated expression references.
    #[must_use]
    pub const fn expression_references(&self) -> usize {
        self.expression_references
    }

    fn increment(
        counter: &mut usize,
        description: &str,
    ) -> AiResult<()> {
        *counter = counter.checked_add(1).ok_or_else(|| {
            induction_error(
                AiErrorKind::ResourceExhausted,
                format!(
                    "induction validation counter overflow: {description}"
                ),
            )
        })?;

        Ok(())
    }

    fn record_request(&mut self) -> AiResult<()> {
        Self::increment(
            &mut self.requests,
            "requests",
        )
    }

    fn record_statement_reference(&mut self) -> AiResult<()> {
        Self::increment(
            &mut self.statement_references,
            "statement references",
        )
    }

    fn record_expression_reference(&mut self) -> AiResult<()> {
        Self::increment(
            &mut self.expression_references,
            "expression references",
        )
    }
}

// ============================================================================
// INDUCTION PROGRAM
// ============================================================================

/// A collection of inductive reasoning requests.
///
/// This is an AI-domain collection, not an AST or executable IR.
///
/// Requests preserve their original insertion/source order.
///
/// No language-level maximum is imposed on the number of requests.
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
)]
pub struct InductionProgram {
    requests: Vec<InductionRequest>,
}

impl InductionProgram {
    /// Construct an empty induction program.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return all requests in insertion/source order.
    #[must_use]
    pub fn requests(&self) -> &[InductionRequest] {
        &self.requests
    }

    /// Return whether the program contains no requests.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.requests.is_empty()
    }

    /// Return the number of registered requests.
    #[must_use]
    pub fn len(&self) -> usize {
        self.requests.len()
    }

    /// Add an inductive reasoning request.
    ///
    /// A canonical source statement may only be registered once
    /// within this program.
    ///
    /// Duplicate detection occurs before mutation, so a failed
    /// registration leaves the program unchanged.
    pub fn add_request(
        &mut self,
        request: InductionRequest,
    ) -> AiResult<()> {
        let source_id = request.statement().node_id();

        if self.requests.iter().any(|existing| {
            existing.statement().node_id() == source_id
        }) {
            return Err(induction_error(
                AiErrorKind::Conflict,
                format!(
                    "induction source statement {source_id} is already registered"
                ),
            ));
        }

        self.requests.push(request);

        Ok(())
    }

    /// Validate all requests against the canonical AST and
    /// authoritative semantic classification.
    ///
    /// Validation establishes:
    ///
    /// - statement references identify canonical reasoning statements;
    /// - statements have been classified as inductive;
    /// - targets identify canonical expressions;
    /// - optional sources identify canonical expressions;
    /// - contextual expressions identify canonical expressions;
    /// - source statement identities are unique.
    ///
    /// Validation does not establish:
    ///
    /// - mathematical correctness;
    /// - evidence sufficiency;
    /// - evidence authenticity;
    /// - confidence or probability;
    /// - uncertainty bounds;
    /// - knowledge consistency;
    /// - type correctness;
    /// - name resolution;
    /// - authorization;
    /// - capability availability;
    /// - resource feasibility;
    /// - execution success.
    ///
    /// These responsibilities remain with the authoritative
    /// semantic and downstream execution systems.
    pub fn validate<R: InductionNodeResolver>(
        &self,
        resolver: &R,
    ) -> AiResult<InductionValidationReport> {
        let mut report = InductionValidationReport::default();

        let mut registered_statements = BTreeSet::new();

        for request in &self.requests {
            let statement = request.statement();

            validate_statement(
                resolver,
                statement,
                &mut registered_statements,
                &mut report,
            )?;

            validate_expression(
                resolver,
                request.target(),
                &mut report,
            )?;

            if let Some(source) = request.source() {
                validate_expression(
                    resolver,
                    source,
                    &mut report,
                )?;
            }

            for expression in request.context() {
                validate_expression(
                    resolver,
                    *expression,
                    &mut report,
                )?;
            }

            report.record_request()?;
        }

        Ok(report)
    }
}

// ============================================================================
// STRUCTURAL VALIDATION
// ============================================================================

fn validate_statement<R: InductionNodeResolver>(
    resolver: &R,
    statement: InductionStatementRef,
    registered: &mut BTreeSet<NodeId>,
    report: &mut InductionValidationReport,
) -> AiResult<()> {
    let node_id = statement.node_id();

    if !registered.insert(node_id) {
        return Err(induction_error(
            AiErrorKind::Conflict,
            format!(
                "duplicate induction statement reference {node_id}"
            ),
        ));
    }

    if !resolver.is_reason_statement_node(node_id) {
        return Err(induction_error(
            AiErrorKind::InvalidInput,
            format!(
                "AST node {node_id} is not a valid reasoning statement"
            ),
        ));
    }

    if !resolver.is_inductive_reasoning_statement(node_id) {
        return Err(induction_error(
            AiErrorKind::InvalidOperation,
            format!(
                "reasoning statement {node_id} has not been classified as inductive by semantic analysis"
            ),
        ));
    }

    report.record_statement_reference()
}

fn validate_expression<R: InductionNodeResolver>(
    resolver: &R,
    expression: InductionExpressionRef,
    report: &mut InductionValidationReport,
) -> AiResult<()> {
    let node_id = expression.node_id();

    if !resolver.is_expression_node(node_id) {
        return Err(induction_error(
            AiErrorKind::InvalidInput,
            format!(
                "AST node {node_id} is not a valid canonical expression"
            ),
        ));
    }

    report.record_expression_reference()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    #[derive(Default)]
    struct TestResolver {
        reasoning_statements: BTreeSet<NodeId>,

        inductive_statements: BTreeSet<NodeId>,

        expressions: BTreeSet<NodeId>,
    }

    impl TestResolver {
        fn with_reasoning_statement(
            mut self,
            id: u64,
        ) -> Self {
            self.reasoning_statements.insert(node(id));
            self
        }

        fn with_inductive_statement(
            mut self,
            id: u64,
        ) -> Self {
            self.inductive_statements.insert(node(id));
            self
        }

        fn with_expression(
            mut self,
            id: u64,
        ) -> Self {
            self.expressions.insert(node(id));
            self
        }
    }

    impl InductionNodeResolver for TestResolver {
        fn is_reason_statement_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.reasoning_statements.contains(&node_id)
        }

        fn is_inductive_reasoning_statement(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.inductive_statements.contains(&node_id)
        }

        fn is_expression_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.expressions.contains(&node_id)
        }
    }

    fn node(id: u64) -> NodeId {
        NodeId::new(id).expect("valid node identity")
    }

    fn statement(id: u64) -> InductionStatementRef {
        InductionStatementRef::new(node(id))
    }

    fn expression(id: u64) -> InductionExpressionRef {
        InductionExpressionRef::new(node(id))
    }

    fn request(
        statement_id: u64,
        target_id: u64,
        source_id: Option<u64>,
        context_ids: &[u64],
    ) -> InductionRequest {
        InductionRequest::new(
            statement(statement_id),
            expression(target_id),
            source_id.map(expression),
            context_ids
                .iter()
                .copied()
                .map(expression)
                .collect(),
        )
    }

    fn resolver_for(
        expression_ids: &[u64],
        statement_ids: &[u64],
    ) -> TestResolver {
        let mut resolver = TestResolver::default();

        for id in expression_ids {
            resolver.expressions.insert(node(*id));
        }

        for id in statement_ids {
            resolver.reasoning_statements.insert(node(*id));
            resolver.inductive_statements.insert(node(*id));
        }

        resolver
    }

    // ------------------------------------------------------------------------
    // Reference tests
    // ------------------------------------------------------------------------

    #[test]
    fn statement_reference_preserves_canonical_identity() {
        let reference = statement(42);

        assert_eq!(reference.node_id(), node(42));
    }

    #[test]
    fn expression_reference_preserves_canonical_identity() {
        let reference = expression(42);

        assert_eq!(reference.node_id(), node(42));
    }

    #[test]
    fn request_preserves_all_expression_references() {
        let item = request(
            1,
            2,
            Some(3),
            &[4, 5, 6],
        );

        assert_eq!(item.statement(), statement(1));
        assert_eq!(item.target(), expression(2));
        assert_eq!(item.source(), Some(expression(3)));

        assert_eq!(
            item.context(),
            &[
                expression(4),
                expression(5),
                expression(6),
            ]
        );
    }

    #[test]
    fn request_without_source_or_context_is_supported() {
        let item = request(1, 2, None, &[]);

        assert_eq!(item.source(), None);
        assert!(item.context().is_empty());
    }

    // ------------------------------------------------------------------------
    // Registration tests
    // ------------------------------------------------------------------------

    #[test]
    fn program_accepts_distinct_inductive_requests() {
        let mut program = InductionProgram::new();

        assert!(program.add_request(
            request(1, 2, Some(3), &[4]),
        ).is_ok());

        assert!(program.add_request(
            request(5, 6, Some(7), &[8]),
        ).is_ok());

        assert_eq!(program.len(), 2);
        assert!(!program.is_empty());
    }

    #[test]
    fn program_rejects_duplicate_source_statements() {
        let mut program = InductionProgram::new();

        program.add_request(
            request(1, 2, None, &[]),
        ).expect("first registration");

        let result = program.add_request(
            request(1, 3, None, &[]),
        );

        assert!(result.is_err());
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn registration_preserves_source_order() {
        let mut program = InductionProgram::new();

        program.add_request(
            request(5, 6, None, &[]),
        ).expect("registration");

        program.add_request(
            request(1, 2, None, &[]),
        ).expect("registration");

        assert_eq!(
            program.requests()[0].statement(),
            statement(5),
        );

        assert_eq!(
            program.requests()[1].statement(),
            statement(1),
        );
    }

    // ------------------------------------------------------------------------
    // Structural validation tests
    // ------------------------------------------------------------------------

    #[test]
    fn validates_complete_inductive_request() {
        let resolver = resolver_for(
            &[2, 3, 4, 5],
            &[1],
        );

        let mut program = InductionProgram::new();

        program.add_request(
            request(
                1,
                2,
                Some(3),
                &[4, 5],
            ),
        ).expect("registration");

        let report = program
            .validate(&resolver)
            .expect("validation");

        assert_eq!(report.requests(), 1);
        assert_eq!(report.statement_references(), 1);
        assert_eq!(report.expression_references(), 4);
    }

    #[test]
    fn rejects_nonexistent_reasoning_statement() {
        let resolver = TestResolver::default()
            .with_expression(2);

        let mut program = InductionProgram::new();

        program.add_request(
            request(1, 2, None, &[]),
        ).expect("registration");

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn rejects_reasoning_statement_not_classified_as_induction() {
        let resolver = TestResolver::default()
            .with_reasoning_statement(1)
            .with_expression(2);

        let mut program = InductionProgram::new();

        program.add_request(
            request(1, 2, None, &[]),
        ).expect("registration");

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn rejects_invalid_target_expression() {
        let resolver = resolver_for(
            &[],
            &[1],
        );

        let mut program = InductionProgram::new();

        program.add_request(
            request(1, 2, None, &[]),
        ).expect("registration");

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn rejects_invalid_source_expression() {
        let resolver = resolver_for(
            &[2],
            &[1],
        );

        let mut program = InductionProgram::new();

        program.add_request(
            request(1, 2, Some(3), &[]),
        ).expect("registration");

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn rejects_invalid_context_expression() {
        let resolver = resolver_for(
            &[2, 3],
            &[1],
        );

        let mut program = InductionProgram::new();

        program.add_request(
            request(1, 2, Some(3), &[4]),
        ).expect("registration");

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn empty_program_is_valid() {
        let resolver = TestResolver::default();

        let program = InductionProgram::new();

        let report = program
            .validate(&resolver)
            .expect("empty program validation");

        assert_eq!(report.requests(), 0);
        assert_eq!(report.statement_references(), 0);
        assert_eq!(report.expression_references(), 0);
    }

    #[test]
    fn validation_is_deterministic() {
        let resolver = resolver_for(
            &[2, 3, 4],
            &[1],
        );

        let mut program = InductionProgram::new();

        program.add_request(
            request(1, 2, Some(3), &[4]),
        ).expect("registration");

        let first = program
            .validate(&resolver)
            .expect("first validation");

        let second = program
            .validate(&resolver)
            .expect("second validation");

        assert_eq!(first, second);
    }

    // ------------------------------------------------------------------------
    // Scalability and architectural tests
    // ------------------------------------------------------------------------

    #[test]
    fn preserves_arbitrary_context_collection_length() {
        let context: Vec<InductionExpressionRef> =
            (2..=1025)
                .map(expression)
                .collect();

        let item = InductionRequest::new(
            statement(1),
            expression(1026),
            Some(expression(1027)),
            context,
        );

        assert_eq!(item.context().len(), 1024);
    }

    #[test]
    fn program_has_no_language_level_request_ceiling() {
        let mut program = InductionProgram::new();

        for id in 1_u64..=1024 {
            program.add_request(
                request(
                    id,
                    id + 10_000,
                    None,
                    &[],
                ),
            ).expect("registration");
        }

        assert_eq!(program.len(), 1024);
    }

    #[test]
    fn repeated_context_expressions_are_preserved() {
        let item = request(
            1,
            2,
            None,
            &[3, 3, 3],
        );

        assert_eq!(
            item.context(),
            &[
                expression(3),
                expression(3),
                expression(3),
            ],
        );
    }
}
