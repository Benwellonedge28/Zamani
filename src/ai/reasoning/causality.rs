
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI — Causal Computation
//!
//! Path: `src/ai/reasoning/causality.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Provides the AI-domain representation and structural validation
//! boundary for causal computational intent.
//!
//! Causal computation concerns relationships between events, variables,
//! states, observations, interventions, outcomes, and other computational
//! entities.
//!
//! It is not restricted to machine learning. Causal operations may be used
//! with classical computation, scientific computation, datasets, models,
//! simulations, quantum-derived observations, hardware observations,
//! distributed state, and future computational domains.
//!
//! This module represents causal operation intent. It does not execute
//! causal algorithms or establish causal truth.
//!
//! ============================================================================
//! ARCHITECTURAL AUTHORITY
//! ============================================================================
//!
//! Canonical causal grammar:
//!
//!     grammar/ai/causality.g4
//!
//! AI grammar composition:
//!
//!     grammar/ai/ai.g4
//!
//! Canonical AST:
//!
//!     src/frontend/ast/
//!
//! Canonical AST identity:
//!
//!     src/frontend/ast/node/node_id.rs
//!
//! Canonical semantic infrastructure:
//!
//!     src/frontend/semantic/
//!
//! AI semantic authority:
//!
//!     src/frontend/semantic/domains/ai.rs
//!
//! Canonical provenance:
//!
//!     src/frontend/semantic/model/provenance.rs
//!
//! AI provenance adapter:
//!
//!     src/ai/provenance.rs
//!
//! Evidence and uncertainty:
//!
//!     Existing evidence and uncertainty authorities.
//!
//! Universal IR:
//!
//!     src/zuir/
//!
//! Quantum representation:
//!
//!     src/quantum/ir/
//!
//! AI error authority:
//!
//!     src/ai/error.rs
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! THIS FILE OWNS:
//!
//! - Causal operation descriptions.
//! - Causal operation names.
//! - Positional causal argument references.
//! - Named causal argument references.
//! - Causal program composition.
//! - Canonical AST reference validation.
//! - Duplicate causal construct detection.
//! - Structural validation reports.
//!
//! THIS FILE DOES NOT OWN:
//!
//! - Source syntax or lexer rules.
//! - Parsing or AST construction.
//! - A competing expression tree.
//! - General semantic type checking.
//! - General name resolution.
//! - Causal graph storage.
//! - Knowledge graph storage.
//! - Evidence storage or authentication.
//! - Causal discovery algorithms.
//! - Causal inference algorithms.
//! - Bayesian inference.
//! - Structural causal model solvers.
//! - Counterfactual solvers.
//! - Statistical estimation.
//! - Probability or uncertainty calculation.
//! - Temporal ordering.
//! - General provenance storage.
//! - Universal intermediate representation.
//! - Quantum intermediate representation.
//! - Hardware discovery.
//! - Device allocation.
//! - Optimization or lowering.
//! - Scheduling or routing.
//! - Distributed transport.
//! - Runtime execution.
//!
//! ============================================================================
//! GRAMMAR MAPPING
//! ============================================================================
//!
//! The canonical grammar defines:
//!
//!     aiCausalityConstruct
//!     causalStatement
//!     causalExpression
//!     causalOperation
//!     causalOperationName
//!     causalArgumentList
//!     causalArgument
//!     causalNamedArgument
//!     causalContext
//!     causalContextItem
//!
//! Its operation representation is generic:
//!
//!     qualified operation name
//!     ordered arguments
//!     optional named arguments
//!     ordinary expression operands
//!
//! This module preserves that generic representation.
//!
//! The canonical semantic operation registry determines whether an operation
//! represents a cause, effect, observation, intervention, counterfactual,
//! dependency, query, assertion, explanation, or registered extension.
//!
//! This module deliberately does not establish those meanings by parsing
//! operation-name strings.
//!
//! ============================================================================
//! POCO-REAF / SCALABILITY
//! ============================================================================
//!
//! This module introduces no language-level maximum for:
//!
//! - causal operations;
//! - causal relationships;
//! - observations;
//! - interventions;
//! - counterfactual requests;
//! - dependencies;
//! - variables;
//! - models;
//! - evidence references;
//! - context arguments;
//! - operation arguments;
//! - causal constructs;
//! - computational domains;
//! - execution targets.
//!
//! Dynamic collections grow according to supplied input and available
//! resources.
//!
//! No fixed graph depth, graph width, node count, edge count, processor
//! count, accelerator count, memory capacity, or machine size is assumed.
//!
//! Rust's finite address space and allocation behavior remain implementation
//! constraints. Literal infinite computation is not promised.
//!
//! Operational budgets belong to the appropriate compiler, semantic,
//! planning, or execution subsystem.
//!
//! ============================================================================
//! INTEGRATION
//! ============================================================================
//!
//! Source:
//!
//!     grammar/ai/causality.g4
//!
//! Parser:
//!
//!     canonical frontend parser
//!
//! AST:
//!
//!     canonical operation/expression nodes
//!
//! Adapter:
//!
//!     maps canonical AST node identities to CausalOperation values
//!
//! Structural validation:
//!
//!     CausalProgram::validate
//!
//! Semantic analysis:
//!
//!     resolves operation identity, arity, argument roles, types, effects,
//!     capabilities, resources, contracts, policies, evidence and provenance
//!
//! Canonical IR:
//!
//!     existing ZUIR and applicable domain IRs
//!
//! Execution:
//!
//!     existing runtime and execution providers
//!
//! The AST adapter must preserve NodeId identities. It must not construct
//! another expression tree or copy expression semantics into this module.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! - BTreeSet is used for deterministic duplicate detection.
//! - Argument order is preserved.
//! - Named argument order is preserved.
//! - No global mutable state is introduced.
//! - No randomness or wall-clock dependency is introduced.
//! - Validation does not execute causal operations.
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
// ERROR CONSTRUCTION
// ============================================================================

/// Construct a structured causal reasoning error.
///
/// The shared AI error infrastructure remains authoritative.
fn causal_error(
    kind: AiErrorKind,
    message: impl Into<String>,
) -> AiError {
    AiError::new(kind, message)
        .with_phase(AiErrorPhase::Reasoning)
}

// ============================================================================
// CANONICAL CAUSAL OPERATION NAMES
// ============================================================================

/// An extensible causal operation identity.
///
/// The namespace is optional because the canonical name grammar may
/// represent either a qualified or unqualified name. Semantic analysis
/// determines whether the resolved name belongs to the canonical causal
/// namespace or a registered extension namespace.
///
/// Examples:
///
///     causal::cause
///     causal::effect
///     causal::observe
///     causal::intervene
///     causal::counterfactual
///     causal::depends_on
///     causal::query
///     causal::assert
///     causal::explain
///     extension.science::discover
///
/// These examples are conventions, not a closed operation catalogue.
///
/// This type records identity only. It does not establish that an operation
/// exists, is authorized, or has an executable implementation.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct CausalOperationName {
    namespace: Option<String>,
    name: String,
}

impl CausalOperationName {
    /// Construct a qualified causal operation name.
    ///
    /// The namespace and name must be nonempty and contain no control
    /// characters. Whitespace is not silently normalized.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> AiResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        validate_name_component("namespace", &namespace)?;
        validate_name_component("operation name", &name)?;

        Ok(Self {
            namespace: Some(namespace),
            name,
        })
    }

    /// Construct an unqualified operation name.
    ///
    /// Whether an unqualified name resolves to a valid causal operation is
    /// determined by canonical semantic name resolution.
    pub fn unqualified(
        name: impl Into<String>,
    ) -> AiResult<Self> {
        let name = name.into();

        validate_name_component("operation name", &name)?;

        Ok(Self {
            namespace: None,
            name,
        })
    }

    /// Return the optional namespace.
    #[must_use]
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    /// Return the operation's unqualified name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the qualified textual identity.
    ///
    /// This is a display and interoperability representation. It must not
    /// replace canonical name resolution.
    #[must_use]
    pub fn qualified(&self) -> String {
        match &self.namespace {
            Some(namespace) => {
                format!("{namespace}::{}", self.name)
            }
            None => self.name.clone(),
        }
    }
}

fn validate_name_component(
    component: &'static str,
    value: &str,
) -> AiResult<()> {
    if value.is_empty() || value.chars().any(char::is_control) {
        return Err(causal_error(
            AiErrorKind::InvalidInput,
            format!(
                "causal {component} must be nonempty and contain no control characters"
            ),
        ));
    }

    Ok(())
}

// ============================================================================
// CANONICAL EXPRESSION REFERENCES
// ============================================================================

/// A reference to an expression in the canonical frontend AST.
///
/// This type does not contain an expression tree, expression text, inferred
/// type, resolved symbol, or execution value.
///
/// The canonical AST owns the referenced expression.
///
/// A NodeId alone does not prove that the referenced node exists or is an
/// expression. Those properties are checked through CausalNodeResolver.
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
pub struct CausalExpressionRef {
    node_id: NodeId,
}

impl CausalExpressionRef {
    /// Construct a reference to a canonical AST expression node.
    ///
    /// Existence and node classification are checked during validation.
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
// CAUSAL ARGUMENTS
// ============================================================================

/// An argument supplied to a causal operation.
///
/// Both positional and named arguments refer to canonical AST expressions.
///
/// Named arguments are intentionally open-world. Their meaning is resolved
/// by the canonical semantic operation registry.
///
/// Examples of named metadata include:
///
///     evidence
///     provenance
///     context
///     model
///     policy
///     confidence
///
/// This module does not maintain a closed catalogue of argument names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CausalArgument {
    /// An ordered positional argument.
    Positional {
        expression: CausalExpressionRef,
    },

    /// An ordered named argument.
    Named {
        name: String,
        expression: CausalExpressionRef,
    },
}

impl CausalArgument {
    /// Construct a positional causal argument.
    #[must_use]
    pub const fn positional(
        expression: CausalExpressionRef,
    ) -> Self {
        Self::Positional {
            expression,
        }
    }

    /// Construct a named causal argument.
    ///
    /// The name must be nonempty and contain no control characters.
    ///
    /// Duplicate named arguments are deliberately left to semantic
    /// validation, which knows the registered operation's argument contract.
    pub fn named(
        name: impl Into<String>,
        expression: CausalExpressionRef,
    ) -> AiResult<Self> {
        let name = name.into();

        validate_name_component("named argument", &name)?;

        Ok(Self::Named {
            name,
            expression,
        })
    }

    /// Return the referenced canonical expression.
    #[must_use]
    pub const fn expression(
        &self,
    ) -> CausalExpressionRef {
        match self {
            Self::Positional { expression }
            | Self::Named { expression, .. } => *expression,
        }
    }

    /// Return the named-argument identity, if this is a named argument.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Positional { .. } => None,
            Self::Named { name, .. } => Some(name),
        }
    }

    /// Return whether this is a named argument.
    #[must_use]
    pub const fn is_named(&self) -> bool {
        matches!(self, Self::Named { .. })
    }
}

// ============================================================================
// CAUSAL OPERATION
// ============================================================================

/// A causal operation request.
///
/// This is the domain representation of the generic operation structure
/// defined by `grammar/ai/causality.g4`.
///
/// It preserves:
///
/// - the canonical AST node identifying the operation;
/// - the qualified operation name;
/// - ordered positional arguments;
/// - ordered named arguments;
/// - canonical AST references for every argument expression.
///
/// The semantic operation registry determines the operation's category,
/// arity, argument roles, types, effects, capabilities, requirements,
/// contracts, policies, evidence requirements, and provenance requirements.
///
/// This type does not execute the operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CausalOperation {
    source_node: NodeId,
    name: CausalOperationName,
    arguments: Vec<CausalArgument>,
}

impl CausalOperation {
    /// Construct a causal operation description.
    ///
    /// The constructor preserves the supplied argument order.
    ///
    /// Structural validity is established by CausalProgram::validate.
    #[must_use]
    pub fn new(
        source_node: NodeId,
        name: CausalOperationName,
        arguments: Vec<CausalArgument>,
    ) -> Self {
        Self {
            source_node,
            name,
            arguments,
        }
    }

    /// Return the canonical AST node identifying this operation.
    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    /// Return the operation identity.
    #[must_use]
    pub fn name(&self) -> &CausalOperationName {
        &self.name
    }

    /// Return the ordered operation arguments.
    #[must_use]
    pub fn arguments(&self) -> &[CausalArgument] {
        &self.arguments
    }

    /// Return the number of arguments.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Return whether the operation has no arguments.
    #[must_use]
    pub fn has_no_arguments(&self) -> bool {
        self.arguments.is_empty()
    }
}

// ============================================================================
// CANONICAL AST RESOLUTION
// ============================================================================

/// Canonical frontend AST lookup interface.
///
/// The actual frontend integration implements this interface using the AST
/// belonging to the current compilation.
///
/// Implementations must answer consistently for the lifetime of one
/// validation operation.
///
/// This interface does not create:
///
/// - an AST registry;
/// - an AST allocator;
/// - an expression parser;
/// - an expression hierarchy;
/// - a competing AST storage system.
///
/// An adapter should use the canonical AST's node identities and node
/// classifications. It must not infer AST membership from identifier text.
pub trait CausalNodeResolver {
    /// Return true only when the node exists in the current canonical AST
    /// and represents an expression.
    fn is_expression_node(
        &self,
        node_id: NodeId,
    ) -> bool;

    /// Return true only when the node exists in the current canonical AST
    /// and represents a causal operation construct.
    ///
    /// A causal operation may occur in either expression or statement
    /// position. The adapter should recognize the canonical AST operation
    /// representation in both cases.
    fn is_causal_operation_node(
        &self,
        node_id: NodeId,
    ) -> bool;
}

// ============================================================================
// CAUSAL PROGRAM
// ============================================================================

/// A collection of causal operation descriptions.
///
/// This is not:
///
/// - a frontend AST;
/// - a causal graph database;
/// - a knowledge store;
/// - a causal inference engine;
/// - a semantic type system;
/// - a universal intermediate representation;
/// - an execution plan.
///
/// Every operation and expression reference points to the canonical
/// frontend AST.
///
/// The collection has no language-level operation limit.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CausalProgram {
    operations: Vec<CausalOperation>,
}

impl CausalProgram {
    /// Construct an empty causal program.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return causal operations in insertion/source order.
    #[must_use]
    pub fn operations(&self) -> &[CausalOperation] {
        &self.operations
    }

    /// Add a causal operation description.
    ///
    /// Structural correctness is established by validate().
    pub fn add_operation(
        &mut self,
        operation: CausalOperation,
    ) {
        self.operations.push(operation);
    }

    /// Return the number of causal operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Return whether the program contains no causal operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Validate causal structure and canonical AST references.
    ///
    /// Checks:
    ///
    /// - each operation refers to a canonical causal AST construct;
    /// - causal construct identities are not registered more than once;
    /// - each positional argument references a canonical AST expression;
    /// - each named argument references a canonical AST expression;
    /// - validation counters remain representable.
    ///
    /// This function does not establish:
    ///
    /// - whether an operation name resolves;
    /// - whether an operation belongs to an authorized causal namespace;
    /// - whether argument arity is valid;
    /// - whether argument names are valid for the operation;
    /// - whether argument types are compatible;
    /// - whether a relationship is causally valid;
    /// - whether an intervention is authorized;
    /// - whether evidence is reliable or authentic;
    /// - whether a counterfactual is identifiable;
    /// - whether a causal model is mathematically valid;
    /// - whether an implementation exists;
    /// - whether a target has sufficient resources.
    ///
    /// These responsibilities belong to semantic analysis, the relevant
    /// reasoning implementations, security, and downstream execution.
    pub fn validate<R: CausalNodeResolver>(
        &self,
        resolver: &R,
    ) -> AiResult<CausalValidationReport> {
        let mut report = CausalValidationReport::default();

        let mut construct_ids = BTreeSet::new();

        for operation in &self.operations {
            // ------------------------------------------------------------
            // Duplicate construct identity
            // ------------------------------------------------------------

            if !construct_ids.insert(operation.source_node) {
                return Err(causal_error(
                    AiErrorKind::Conflict,
                    "a canonical AST node is registered as multiple causal operations",
                ));
            }

            // ------------------------------------------------------------
            // Canonical causal construct
            // ------------------------------------------------------------

            if !resolver.is_causal_operation_node(
                operation.source_node,
            ) {
                return Err(causal_error(
                    AiErrorKind::InvalidInput,
                    "causal operation does not reference a valid canonical AST causal construct",
                ));
            }

            report.record_operation()?;

            // ------------------------------------------------------------
            // Canonical argument expressions
            // ------------------------------------------------------------

            for argument in &operation.arguments {
                validate_expression(
                    resolver,
                    argument.expression(),
                    &mut report,
                )?;
            }
        }

        report.complete = true;

        Ok(report)
    }
}

// ============================================================================
// STRUCTURAL VALIDATION HELPERS
// ============================================================================

/// Validate one expression reference against the canonical frontend AST.
fn validate_expression<R: CausalNodeResolver>(
    resolver: &R,
    expression: CausalExpressionRef,
    report: &mut CausalValidationReport,
) -> AiResult<()> {
    if !resolver.is_expression_node(expression.node_id) {
        return Err(causal_error(
            AiErrorKind::InvalidInput,
            "causal argument does not reference a valid canonical AST expression",
        ));
    }

    report.record_expression_reference()?;

    Ok(())
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Deterministic causal structural-validation report.
///
/// A complete report means that every structural check implemented by this
/// module succeeded.
///
/// It does not certify causal truth, causal identifiability, semantic
/// validity, evidence reliability, policy authorization, or execution
/// feasibility.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CausalValidationReport {
    operations_checked: usize,
    expression_references_checked: usize,
    complete: bool,
}

impl CausalValidationReport {
    /// Record a successfully validated causal operation.
    fn record_operation(&mut self) -> AiResult<()> {
        self.operations_checked = self
            .operations_checked
            .checked_add(1)
            .ok_or_else(|| {
                causal_error(
                    AiErrorKind::ResourceExhausted,
                    "causal operation validation counter exhausted its representation",
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
                causal_error(
                    AiErrorKind::ResourceExhausted,
                    "causal expression validation counter exhausted its representation",
                )
            })?;

        Ok(())
    }

    /// Return the number of successfully validated operations.
    #[must_use]
    pub const fn operations_checked(&self) -> usize {
        self.operations_checked
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
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    /// Test-only canonical AST resolver.
    ///
    /// Production code must implement CausalNodeResolver using the actual
    /// canonical frontend AST.
    #[derive(Default)]
    struct TestResolver {
        expressions: BTreeSet<NodeId>,
        causal_operations: BTreeSet<NodeId>,
    }

    impl TestResolver {
        fn new() -> Self {
            Self::default()
        }

        fn expression(&mut self, id: u64) -> NodeId {
            let id = NodeId::new(id)
                .expect("test AST node identity must be nonzero");

            self.expressions.insert(id);

            id
        }

        fn causal_operation(&mut self, id: u64) -> NodeId {
            let id = NodeId::new(id)
                .expect("test AST node identity must be nonzero");

            self.causal_operations.insert(id);

            id
        }
    }

    impl CausalNodeResolver for TestResolver {
        fn is_expression_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.expressions.contains(&node_id)
        }

        fn is_causal_operation_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.causal_operations.contains(&node_id)
        }
    }

    fn operation_name(
        namespace: &str,
        name: &str,
    ) -> CausalOperationName {
        CausalOperationName::new(namespace, name)
            .expect("test operation name must be valid")
    }

    fn expression(
        node_id: NodeId,
    ) -> CausalExpressionRef {
        CausalExpressionRef::new(node_id)
    }

    // ---------------------------------------------------------------------
    // Operation-name tests
    // ---------------------------------------------------------------------

    #[test]
    fn constructs_qualified_operation_names() {
        let name = operation_name(
            "causal",
            "counterfactual",
        );

        assert_eq!(
            name.namespace(),
            Some("causal")
        );

        assert_eq!(
            name.name(),
            "counterfactual"
        );

        assert_eq!(
            name.qualified(),
            "causal::counterfactual"
        );
    }

    #[test]
    fn constructs_unqualified_operation_names() {
        let name = CausalOperationName::unqualified(
            "observe",
        )
        .unwrap();

        assert_eq!(name.namespace(), None);
        assert_eq!(name.name(), "observe");
        assert_eq!(name.qualified(), "observe");
    }

    #[test]
    fn permits_extensible_operation_namespaces() {
        let name = operation_name(
            "extension.scientific.causality",
            "discover",
        );

        assert_eq!(
            name.qualified(),
            "extension.scientific.causality::discover"
        );
    }

    #[test]
    fn rejects_empty_operation_name_components() {
        assert!(
            CausalOperationName::new(
                "",
                "cause",
            )
            .is_err()
        );

        assert!(
            CausalOperationName::new(
                "causal",
                "",
            )
            .is_err()
        );

        assert!(
            CausalOperationName::unqualified("")
                .is_err()
        );
    }

    #[test]
    fn rejects_control_characters_in_operation_names() {
        assert!(
            CausalOperationName::new(
                "causal",
                "cause\ninvalid",
            )
            .is_err()
        );
    }

    // ---------------------------------------------------------------------
    // Argument tests
    // ---------------------------------------------------------------------

    #[test]
    fn constructs_positional_arguments() {
        let node_id = NodeId::new(1).unwrap();

        let argument = CausalArgument::positional(
            expression(node_id),
        );

        assert!(!argument.is_named());
        assert_eq!(argument.name(), None);
        assert_eq!(
            argument.expression().node_id(),
            node_id
        );
    }

    #[test]
    fn constructs_named_arguments() {
        let node_id = NodeId::new(1).unwrap();

        let argument = CausalArgument::named(
            "evidence",
            expression(node_id),
        )
        .unwrap();

        assert!(argument.is_named());
        assert_eq!(argument.name(), Some("evidence"));
        assert_eq!(
            argument.expression().node_id(),
            node_id
        );
    }

    #[test]
    fn rejects_invalid_named_argument_names() {
        let node_id = NodeId::new(1).unwrap();

        assert!(
            CausalArgument::named(
                "",
                expression(node_id),
            )
            .is_err()
        );

        assert!(
            CausalArgument::named(
                "invalid\nname",
                expression(node_id),
            )
            .is_err()
        );
    }

    // ---------------------------------------------------------------------
    // Structural validation tests
    // ---------------------------------------------------------------------

    #[test]
    fn validates_a_causal_operation() {
        let mut resolver = TestResolver::new();

        let source = resolver.causal_operation(1);
        let cause = resolver.expression(2);
        let effect = resolver.expression(3);

        let operation = CausalOperation::new(
            source,
            operation_name("causal", "cause"),
            vec![
                CausalArgument::positional(
                    expression(cause),
                ),
                CausalArgument::positional(
                    expression(effect),
                ),
            ],
        );

        let mut program = CausalProgram::new();

        program.add_operation(operation);

        let report = program
            .validate(&resolver)
            .unwrap();

        assert!(report.is_complete());
        assert_eq!(report.operations_checked(), 1);
        assert_eq!(
            report.expression_references_checked(),
            2
        );
    }

    #[test]
    fn validates_named_evidence_and_context_arguments() {
        let mut resolver = TestResolver::new();

        let source = resolver.causal_operation(1);
        let outcome = resolver.expression(2);
        let evidence = resolver.expression(3);
        let context = resolver.expression(4);

        let operation = CausalOperation::new(
            source,
            operation_name(
                "causal",
                "counterfactual",
            ),
            vec![
                CausalArgument::positional(
                    expression(outcome),
                ),
                CausalArgument::named(
                    "evidence",
                    expression(evidence),
                )
                .unwrap(),
                CausalArgument::named(
                    "context",
                    expression(context),
                )
                .unwrap(),
            ],
        );

        let mut program = CausalProgram::new();

        program.add_operation(operation);

        let report = program
            .validate(&resolver)
            .unwrap();

        assert!(report.is_complete());

        assert_eq!(
            report.expression_references_checked(),
            3
        );
    }

    #[test]
    fn rejects_a_missing_causal_construct() {
        let resolver = TestResolver::new();

        let source = NodeId::new(1).unwrap();

        let operation = CausalOperation::new(
            source,
            operation_name("causal", "observe"),
            Vec::new(),
        );

        let mut program = CausalProgram::new();

        program.add_operation(operation);

        assert!(
            program.validate(&resolver).is_err()
        );
    }

    #[test]
    fn rejects_a_missing_expression_reference() {
        let mut resolver = TestResolver::new();

        let source = resolver.causal_operation(1);

        let missing_expression = NodeId::new(2).unwrap();

        let operation = CausalOperation::new(
            source,
            operation_name("causal", "observe"),
            vec![
                CausalArgument::positional(
                    expression(missing_expression),
                ),
            ],
        );

        let mut program = CausalProgram::new();

        program.add_operation(operation);

        assert!(
            program.validate(&resolver).is_err()
        );
    }

    #[test]
    fn rejects_duplicate_causal_construct_identities() {
        let mut resolver = TestResolver::new();

        let source = resolver.causal_operation(1);
        let input = resolver.expression(2);

        let first = CausalOperation::new(
            source,
            operation_name("causal", "cause"),
            vec![
                CausalArgument::positional(
                    expression(input),
                ),
            ],
        );

        let second = CausalOperation::new(
            source,
            operation_name("causal", "effect"),
            vec![
                CausalArgument::positional(
                    expression(input),
                ),
            ],
        );

        let mut program = CausalProgram::new();

        program.add_operation(first);
        program.add_operation(second);

        let error = program
            .validate(&resolver)
            .unwrap_err();

        assert_eq!(
            error.kind(),
            AiErrorKind::Conflict
        );
    }

    #[test]
    fn preserves_operation_and_argument_order() {
        let mut resolver = TestResolver::new();

        let first_source = resolver.causal_operation(1);
        let second_source = resolver.causal_operation(2);

        let first_expression = resolver.expression(3);
        let second_expression = resolver.expression(4);

        let first = CausalOperation::new(
            first_source,
            operation_name("causal", "cause"),
            vec![
                CausalArgument::positional(
                    expression(first_expression),
                ),
                CausalArgument::positional(
                    expression(second_expression),
                ),
            ],
        );

        let second = CausalOperation::new(
            second_source,
            operation_name("causal", "effect"),
            Vec::new(),
        );

        let mut program = CausalProgram::new();

        program.add_operation(first);
        program.add_operation(second);

        assert_eq!(
            program.operations()[0].name().name(),
            "cause"
        );

        assert_eq!(
            program.operations()[0].arguments()[0]
                .expression()
                .node_id(),
            first_expression
        );

        assert_eq!(
            program.operations()[0].arguments()[1]
                .expression()
                .node_id(),
            second_expression
        );

        assert_eq!(
            program.operations()[1].name().name(),
            "effect"
        );

        let report = program
            .validate(&resolver)
            .unwrap();

        assert_eq!(report.operations_checked(), 2);
    }

    #[test]
    fn empty_causal_program_is_structurally_valid() {
        let resolver = TestResolver::new();
        let program = CausalProgram::new();

        let report = program
            .validate(&resolver)
            .unwrap();

        assert!(program.is_empty());
        assert!(report.is_complete());
        assert_eq!(report.operations_checked(), 0);
        assert_eq!(
            report.expression_references_checked(),
            0
        );
    }

    #[test]
    fn validation_report_does_not_claim_semantic_completion() {
        let mut resolver = TestResolver::new();

        let source = resolver.causal_operation(1);

        let operation = CausalOperation::new(
            source,
            operation_name(
                "unregistered.extension",
                "future_operation",
            ),
            Vec::new(),
        );

        let mut program = CausalProgram::new();

        program.add_operation(operation);

        let report = program
            .validate(&resolver)
            .unwrap();

        // This report certifies only the structural checks performed here.
        // Operation registration remains the semantic layer's responsibility.
        assert!(report.is_complete());
    }
}
