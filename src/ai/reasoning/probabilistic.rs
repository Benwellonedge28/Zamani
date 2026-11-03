
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI — Probabilistic Reasoning
//!
//! Path: `src/ai/reasoning/probabilistic.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Defines target-independent descriptions of probabilistic computational
//! intent in Zamani.
//!
//! This module provides:
//!
//! - probabilistic declarations;
//! - random-variable declarations;
//! - distribution declarations;
//! - sampling requests;
//! - observations;
//! - conditioning requests;
//! - prior and posterior relationships;
//! - likelihood descriptions;
//! - probabilistic inference requests;
//! - probabilistic regions;
//! - probabilistic contracts;
//! - extensible probabilistic operation descriptions;
//! - evidence and uncertainty references;
//! - structural validation against the canonical frontend AST.
//!
//! # Architectural ownership
//!
//! Source syntax is owned by:
//!
//!     grammar/ai/probabilistic.g4
//!
//! AI grammar composition is owned by:
//!
//!     grammar/ai/ai.g4
//!
//! Canonical source structure and expression identity are owned by:
//!
//!     src/frontend/ast/
//!
//! AI semantic validation is owned by:
//!
//!     src/frontend/semantic/domains/ai.rs
//!
//! Universal AI operation representation belongs to:
//!
//!     src/zuir/dialects/ai/
//!
//! Tensor storage belongs to:
//!
//!     src/classical/collections/tensor.rs
//!
//! Evidence and uncertainty have separate reasoning-module ownership.
//!
//! # This module owns
//!
//! - Probabilistic computational-intent descriptors.
//! - References to canonical probabilistic AST constructs.
//! - References to canonical AST expressions.
//! - Probabilistic construct relationships.
//! - Structural validation of those references.
//! - Probabilistic-domain composition metadata.
//!
//! # This module does not own
//!
//! - Parsing or lexer tokens.
//! - AST construction or AST storage.
//! - General semantic validation.
//! - General type checking or name resolution.
//! - Numerical probability evaluation.
//! - Probability arithmetic.
//! - Distribution implementations.
//! - Random-number generation.
//! - Entropy-source selection.
//! - Bayesian inference algorithms.
//! - MCMC or variational inference implementations.
//! - Particle filtering or sequential Monte Carlo.
//! - Numerical integration.
//! - Statistical estimation.
//! - Tensor storage.
//! - Evidence persistence.
//! - Knowledge persistence.
//! - Uncertainty execution.
//! - Universal intermediate representation.
//! - Hardware discovery or selection.
//! - Resource allocation.
//! - Scheduling, routing or placement.
//! - Runtime execution.
//!
//! # POCO-REAF and scalability
//!
//! This module imposes no language-level maximum on:
//!
//! - probabilistic constructs;
//! - random variables;
//! - distributions;
//! - distribution parameters;
//! - observations;
//! - samples;
//! - conditions;
//! - evidence references;
//! - latent variables;
//! - inference variables;
//! - model relationships;
//! - region members;
//! - probabilistic operations;
//! - computational targets.
//!
//! Collections grow according to supplied input and available resources.
//!
//! Rust's finite address space, integer representation and allocation
//! behavior remain implementation constraints. Literal infinite computation
//! is not promised.
//!
//! Operational budgets belong to the compiler, resource-analysis and
//! execution layers. They must not become universal language-level limits.
//!
//! # Integration pathway
//!
//! ```text
//! grammar/ai/probabilistic.g4
//!                 |
//!                 v
//!          canonical parser
//!                 |
//!                 v
//!          canonical frontend AST
//!                 |
//!                 v
//!        probabilistic AST adapter
//!                 |
//!                 v
//!        ProbabilisticProgram
//!                 |
//!                 v
//!    frontend/semantic/domains/ai.rs
//!                 |
//!                 v
//!          canonical semantics
//!                 |
//!                 v
//!           ZUIR AI dialect
//!                 |
//!                 v
//!       optimization and lowering
//!                 |
//!                 v
//!       appropriate execution layer
//! ```
//!
//! This module preserves canonical AST identities. It does not translate
//! source expressions into a competing expression tree.
//!
//! # Important semantic boundary
//!
//! Structural validation in this module does not establish:
//!
//! - probability axioms;
//! - distribution validity;
//! - normalization;
//! - independence;
//! - conditional independence;
//! - posterior correctness;
//! - statistical significance;
//! - model identifiability;
//! - numerical stability;
//! - inference convergence;
//! - semantic type correctness;
//! - operation availability;
//! - target feasibility.
//!
//! Those properties require their respective semantic, mathematical,
//! numerical, registry or execution authorities.
//!
//! A structurally valid probabilistic program is not automatically a
//! mathematically valid or executable probabilistic program.

use std::collections::BTreeSet;
use std::fmt;

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

fn probabilistic_error(
    kind: AiErrorKind,
    message: impl Into<String>,
) -> AiError {
    AiError::new(kind, message)
        .with_phase(AiErrorPhase::Reasoning)
}

// ============================================================================
// PROBABILISTIC NAMES
// ============================================================================

/// A qualified, extensible probabilistic-domain name.
///
/// Names identify semantic vocabulary, registered operations, distributions
/// or extension-defined concepts.
///
/// A name is not an implementation and does not imply that an operation or
/// distribution is available.
///
/// Examples:
///
/// - `probability.normal`
/// - `probability.bernoulli`
/// - `probability.posterior`
/// - `statistics.likelihood`
/// - `extension.custom_distribution`
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct ProbabilisticName {
    namespace: String,
    name: String,
}

impl ProbabilisticName {
    /// Construct a qualified probabilistic name.
    ///
    /// Components must be nonempty and must not contain control characters.
    /// Whitespace is not silently normalized.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> AiResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        validate_name_component("namespace", &namespace)?;
        validate_name_component("name", &name)?;

        Ok(Self {
            namespace,
            name,
        })
    }

    /// Return the namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the local name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the qualified textual representation.
    #[must_use]
    pub fn qualified(&self) -> String {
        format!("{}.{}", self.namespace, self.name)
    }
}

impl fmt::Display for ProbabilisticName {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}",
            self.namespace,
            self.name
        )
    }
}

fn validate_name_component(
    component: &'static str,
    value: &str,
) -> AiResult<()> {
    if value.is_empty() || value.chars().any(char::is_control) {
        return Err(probabilistic_error(
            AiErrorKind::InvalidInput,
            format!(
                "probabilistic {component} must be nonempty \
                 and contain no control characters"
            ),
        ));
    }

    Ok(())
}

// ============================================================================
// CANONICAL AST REFERENCES
// ============================================================================

/// A reference to an expression in the canonical frontend AST.
///
/// This wrapper does not contain or reconstruct an expression.
///
/// Existence and expression-node classification must be verified against the
/// AST associated with the current compilation.
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
pub struct ProbabilisticExpressionRef {
    node_id: NodeId,
}

impl ProbabilisticExpressionRef {
    /// Construct a reference to a canonical AST expression node.
    ///
    /// The referenced node is checked during structural validation.
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

/// A reference to a canonical probabilistic construct node.
///
/// The referenced node must belong to the current AST and must represent a
/// probabilistic construct.
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
pub struct ProbabilisticConstructRef {
    node_id: NodeId,
}

impl ProbabilisticConstructRef {
    /// Construct a reference to a canonical probabilistic construct.
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
// GRAMMAR-ALIGNED CONSTRUCT KINDS
// ============================================================================

/// The source-level probabilistic construct represented by a descriptor.
///
/// These variants correspond to the construct families in
/// `grammar/ai/probabilistic.g4`.
///
/// This enum describes source intent. It does not implement the
/// mathematical or numerical operation.
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
#[non_exhaustive]
pub enum ProbabilisticConstructKind {
    /// General probabilistic declaration.
    Declaration,

    /// Random-variable declaration.
    RandomVariable,

    /// Distribution declaration.
    Distribution,

    /// Sampling construct.
    Sample,

    /// Observation construct.
    Observation,

    /// Conditioning construct.
    Conditioning,

    /// Prior relationship.
    Prior,

    /// Posterior relationship.
    Posterior,

    /// Likelihood construct.
    Likelihood,

    /// Probabilistic inference request.
    Inference,

    /// Probabilistic computation region.
    Region,

    /// Probabilistic resource/capability contract.
    Contract,

    /// Extension-defined probabilistic construct.
    Extension,
}

// ============================================================================
// EXPRESSION ROLES
// ============================================================================

/// The declared role of a canonical expression reference.
///
/// These roles provide structural context for adapters and diagnostics.
/// They do not replace semantic type checking.
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
#[non_exhaustive]
pub enum ProbabilisticExpressionRole {
    /// General probabilistic declaration initializer.
    Initializer,

    /// Random-variable definition.
    RandomVariableDefinition,

    /// Distribution parameter.
    DistributionParameter,

    /// Distribution initializer.
    DistributionInitializer,

    /// Sample subject.
    SampleSubject,

    /// Sample argument.
    SampleArgument,

    /// Observed value or observation expression.
    Observation,

    /// Conditioning expression.
    Condition,

    /// Prior expression.
    Prior,

    /// Posterior expression.
    Posterior,

    /// Likelihood expression.
    Likelihood,

    /// Inference request payload.
    InferenceRequest,

    /// Probabilistic operation payload.
    OperationPayload,

    /// Extension-defined expression role.
    Extension,
}

/// A role-labelled canonical expression reference.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProbabilisticExpression {
    role: ProbabilisticExpressionRole,
    reference: ProbabilisticExpressionRef,
}

impl ProbabilisticExpression {
    /// Construct a role-labelled expression reference.
    #[must_use]
    pub const fn new(
        role: ProbabilisticExpressionRole,
        reference: ProbabilisticExpressionRef,
    ) -> Self {
        Self { role, reference }
    }

    /// Return the expression role.
    #[must_use]
    pub const fn role(&self) -> ProbabilisticExpressionRole {
        self.role
    }

    /// Return the canonical expression reference.
    #[must_use]
    pub const fn reference(&self) -> ProbabilisticExpressionRef {
        self.reference
    }
}

// ============================================================================
// PROBABILISTIC OPERATION IDENTITY
// ============================================================================

/// An extensible probabilistic operation identity.
///
/// This is intentionally independent of a closed list of probability
/// algorithms.
///
/// Examples include:
///
/// - exact inference;
/// - approximate inference;
/// - MCMC;
/// - variational inference;
/// - particle filtering;
/// - sequential Monte Carlo;
/// - custom registered inference procedures.
///
/// Naming an operation does not implement it or establish its availability.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbabilisticOperation {
    name: ProbabilisticName,
}

impl ProbabilisticOperation {
    /// Construct an operation identity.
    #[must_use]
    pub fn new(name: ProbabilisticName) -> Self {
        Self { name }
    }

    /// Return the operation identity.
    #[must_use]
    pub fn name(&self) -> &ProbabilisticName {
        &self.name
    }
}

// ============================================================================
// EVIDENCE AND UNCERTAINTY REFERENCES
// ============================================================================

/// A reference to an evidence construct in the canonical frontend AST.
///
/// Evidence ownership and evidence validation belong to the dedicated
/// reasoning evidence subsystem. This wrapper is only an integration
/// reference; it does not implement evidence storage.
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
pub struct ProbabilisticEvidenceRef {
    node_id: NodeId,
}

impl ProbabilisticEvidenceRef {
    /// Construct an evidence reference.
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

/// A reference to an uncertainty construct in the canonical frontend AST.
///
/// Uncertainty representation and propagation belong to the dedicated
/// uncertainty subsystem. This wrapper does not implement either.
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
pub struct ProbabilisticUncertaintyRef {
    node_id: NodeId,
}

impl ProbabilisticUncertaintyRef {
    /// Construct an uncertainty reference.
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
// PROBABILISTIC CONSTRUCT
// ============================================================================

/// A target-independent probabilistic construct descriptor.
///
/// `expressions` refer to ordinary expressions in the canonical frontend
/// AST. `members` refer to nested probabilistic constructs, such as members
/// of a probabilistic region.
///
/// Evidence and uncertainty are represented by references to their
/// respective canonical AST constructs.
///
/// This structure is a domain descriptor, not an AST and not an IR.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbabilisticConstruct {
    source_node: NodeId,
    kind: ProbabilisticConstructKind,
    expressions: Vec<ProbabilisticExpression>,
    members: Vec<ProbabilisticConstructRef>,
    evidence: Vec<ProbabilisticEvidenceRef>,
    uncertainty: Vec<ProbabilisticUncertaintyRef>,
    operation: Option<ProbabilisticOperation>,
}

impl ProbabilisticConstruct {
    /// Construct a probabilistic descriptor.
    ///
    /// Structural correctness is checked by `ProbabilisticProgram::validate`.
    pub fn new(
        source_node: NodeId,
        kind: ProbabilisticConstructKind,
    ) -> Self {
        Self {
            source_node,
            kind,
            expressions: Vec::new(),
            members: Vec::new(),
            evidence: Vec::new(),
            uncertainty: Vec::new(),
            operation: None,
        }
    }

    /// Add an expression reference with its declared role.
    #[must_use]
    pub fn with_expression(
        mut self,
        expression: ProbabilisticExpression,
    ) -> Self {
        self.expressions.push(expression);
        self
    }

    /// Add a nested probabilistic construct reference.
    #[must_use]
    pub fn with_member(
        mut self,
        member: ProbabilisticConstructRef,
    ) -> Self {
        self.members.push(member);
        self
    }

    /// Add an evidence reference.
    #[must_use]
    pub fn with_evidence(
        mut self,
        evidence: ProbabilisticEvidenceRef,
    ) -> Self {
        self.evidence.push(evidence);
        self
    }

    /// Add an uncertainty reference.
    #[must_use]
    pub fn with_uncertainty(
        mut self,
        uncertainty: ProbabilisticUncertaintyRef,
    ) -> Self {
        self.uncertainty.push(uncertainty);
        self
    }

    /// Associate an extensible probabilistic operation.
    #[must_use]
    pub fn with_operation(
        mut self,
        operation: ProbabilisticOperation,
    ) -> Self {
        self.operation = Some(operation);
        self
    }

    /// Return the source construct identity.
    #[must_use]
    pub const fn source_node(&self) -> NodeId {
        self.source_node
    }

    /// Return the construct kind.
    #[must_use]
    pub const fn kind(&self) -> ProbabilisticConstructKind {
        self.kind
    }

    /// Return expression references.
    #[must_use]
    pub fn expressions(&self) -> &[ProbabilisticExpression] {
        &self.expressions
    }

    /// Return nested construct references.
    #[must_use]
    pub fn members(&self) -> &[ProbabilisticConstructRef] {
        &self.members
    }

    /// Return evidence references.
    #[must_use]
    pub fn evidence(&self) -> &[ProbabilisticEvidenceRef] {
        &self.evidence
    }

    /// Return uncertainty references.
    #[must_use]
    pub fn uncertainty(&self) -> &[ProbabilisticUncertaintyRef] {
        &self.uncertainty
    }

    /// Return the optional operation identity.
    #[must_use]
    pub fn operation(&self) -> Option<&ProbabilisticOperation> {
        self.operation.as_ref()
    }
}

// ============================================================================
// PROBABILISTIC PROGRAM
// ============================================================================

/// A collection of probabilistic computational-intent descriptors.
///
/// This is not an AST, semantic model or executable IR.
///
/// All source identities and expression references point to the canonical
/// frontend AST.
///
/// No language-level construct-count limit is imposed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProbabilisticProgram {
    constructs: Vec<ProbabilisticConstruct>,
}

impl ProbabilisticProgram {
    /// Construct an empty probabilistic program.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            constructs: Vec::new(),
        }
    }

    /// Construct a program from supplied descriptors.
    #[must_use]
    pub fn from_constructs(
        constructs: Vec<ProbabilisticConstruct>,
    ) -> Self {
        Self { constructs }
    }

    /// Append a probabilistic construct.
    ///
    /// This method does not perform semantic validation. Call `validate`
    /// against the canonical AST before semantic lowering.
    pub fn add_construct(
        &mut self,
        construct: ProbabilisticConstruct,
    ) {
        self.constructs.push(construct);
    }

    /// Return all probabilistic constructs in insertion order.
    #[must_use]
    pub fn constructs(&self) -> &[ProbabilisticConstruct] {
        &self.constructs
    }

    /// Return the number of descriptors in this collection.
    #[must_use]
    pub fn len(&self) -> usize {
        self.constructs.len()
    }

    /// Return whether this collection contains no descriptors.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.constructs.is_empty()
    }

    /// Validate canonical AST references and structural invariants.
    ///
    /// This operation:
    ///
    /// - verifies that every source node exists and is probabilistic;
    /// - rejects duplicate source construct identities;
    /// - verifies every expression reference;
    /// - verifies every nested construct reference;
    /// - verifies every evidence reference;
    /// - verifies every uncertainty reference;
    /// - rejects direct self-membership;
    /// - checks validation-counter overflow.
    ///
    /// It does not perform probability arithmetic, type checking, semantic
    /// validation, theorem proving, inference or execution.
    pub fn validate<R: ProbabilisticNodeResolver>(
        &self,
        resolver: &R,
    ) -> AiResult<ProbabilisticValidationReport> {
        let mut report = ProbabilisticValidationReport::default();

        let mut construct_ids = BTreeSet::new();

        for construct in &self.constructs {
            register_construct(
                &mut construct_ids,
                construct.source_node,
            )?;

            if !resolver.is_probabilistic_construct_node(
                construct.source_node,
                construct.kind,
            ) {
                return Err(probabilistic_error(
                    AiErrorKind::InvalidInput,
                    format!(
                        "AST node {} is not a valid probabilistic construct \
                         of kind {:?}",
                        construct.source_node,
                        construct.kind
                    ),
                ));
            }

            report.record_construct()?;

            for expression in &construct.expressions {
                validate_expression(
                    resolver,
                    expression.reference,
                    &mut report,
                )?;
            }

            for member in &construct.members {
                if member.node_id == construct.source_node {
                    return Err(probabilistic_error(
                        AiErrorKind::InvalidInput,
                        format!(
                            "probabilistic construct {} cannot directly \
                             contain itself",
                            construct.source_node
                        ),
                    ));
                }

                if !resolver.is_probabilistic_construct_node(
                    member.node_id,
                    ProbabilisticConstructKind::Region,
                ) && !resolver.is_any_probabilistic_construct_node(
                    member.node_id,
                ) {
                    return Err(probabilistic_error(
                        AiErrorKind::InvalidInput,
                        format!(
                            "AST node {} is not a probabilistic construct \
                             referenced by construct {}",
                            member.node_id,
                            construct.source_node
                        ),
                    ));
                }

                report.record_construct_reference()?;
            }

            for evidence in &construct.evidence {
                if !resolver.is_evidence_node(evidence.node_id) {
                    return Err(probabilistic_error(
                        AiErrorKind::InvalidInput,
                        format!(
                            "AST node {} is not a valid evidence construct",
                            evidence.node_id
                        ),
                    ));
                }

                report.record_evidence_reference()?;
            }

            for uncertainty in &construct.uncertainty {
                if !resolver.is_uncertainty_node(uncertainty.node_id) {
                    return Err(probabilistic_error(
                        AiErrorKind::InvalidInput,
                        format!(
                            "AST node {} is not a valid uncertainty construct",
                            uncertainty.node_id
                        ),
                    ));
                }

                report.record_uncertainty_reference()?;
            }
        }

        report.complete = true;
        Ok(report)
    }
}

// ============================================================================
// CANONICAL AST ADAPTER CONTRACT
// ============================================================================

/// Resolves probabilistic references against the canonical frontend AST.
///
/// The frontend adapter implements this trait using the actual AST associated
/// with the current compilation.
///
/// This module does not create an AST registry, parser or storage system.
///
/// The default implementation of `is_any_probabilistic_construct_node`
/// delegates to the general probabilistic construct predicate.
pub trait ProbabilisticNodeResolver {
    /// Return true only if this node exists in the current AST and represents
    /// the specified probabilistic construct kind.
    fn is_probabilistic_construct_node(
        &self,
        node_id: NodeId,
        kind: ProbabilisticConstructKind,
    ) -> bool;

    /// Return true only if this node exists and represents some probabilistic
    /// construct recognized by the canonical AST adapter.
    ///
    /// Implementations should override this method when the AST distinguishes
    /// construct categories independently of the requested kind.
    fn is_any_probabilistic_construct_node(
        &self,
        node_id: NodeId,
    ) -> bool {
        self.is_probabilistic_construct_node(
            node_id,
            ProbabilisticConstructKind::Declaration,
        ) || self.is_probabilistic_construct_node(
            node_id,
            ProbabilisticConstructKind::RandomVariable,
        ) || self.is_probabilistic_construct_node(
            node_id,
            ProbabilisticConstructKind::Distribution,
        ) || self.is_probabilistic_construct_node(
            node_id,
            ProbabilisticConstructKind::Sample,
        ) || self.is_probabilistic_construct_node(
            node_id,
            ProbabilisticConstructKind::Observation,
        ) || self.is_probabilistic_construct_node(
            node_id,
            ProbabilisticConstructKind::Conditioning,
        ) || self.is_probabilistic_construct_node(
            node_id,
            ProbabilisticConstructKind::Prior,
        ) || self.is_probabilistic_construct_node(
            node_id,
            ProbabilisticConstructKind::Posterior,
        ) || self.is_probabilistic_construct_node(
            node_id,
            ProbabilisticConstructKind::Likelihood,
        ) || self.is_probabilistic_construct_node(
            node_id,
            ProbabilisticConstructKind::Inference,
        ) || self.is_probabilistic_construct_node(
            node_id,
            ProbabilisticConstructKind::Region,
        ) || self.is_probabilistic_construct_node(
            node_id,
            ProbabilisticConstructKind::Contract,
        ) || self.is_probabilistic_construct_node(
            node_id,
            ProbabilisticConstructKind::Extension,
        )
    }

    /// Return true only if this node exists and represents a canonical
    /// expression node.
    fn is_expression_node(
        &self,
        node_id: NodeId,
    ) -> bool;

    /// Return true only if this node exists and represents an evidence
    /// construct recognized by the canonical AST adapter.
    ///
    /// This is an AST classification contract, not evidence validation.
    fn is_evidence_node(
        &self,
        node_id: NodeId,
    ) -> bool;

    /// Return true only if this node exists and represents an uncertainty
    /// construct recognized by the canonical AST adapter.
    ///
    /// This is an AST classification contract, not uncertainty validation.
    fn is_uncertainty_node(
        &self,
        node_id: NodeId,
    ) -> bool;
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Result of probabilistic structural validation.
///
/// A successful report describes only the checks performed by this module.
/// It is not a mathematical proof, semantic validation report or execution
/// feasibility result.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProbabilisticValidationReport {
    constructs_checked: usize,
    expressions_checked: usize,
    construct_references_checked: usize,
    evidence_references_checked: usize,
    uncertainty_references_checked: usize,
    complete: bool,
}

impl ProbabilisticValidationReport {
    fn increment(
        counter: &mut usize,
        description: &'static str,
    ) -> AiResult<()> {
        *counter = counter.checked_add(1).ok_or_else(|| {
            probabilistic_error(
                AiErrorKind::ResourceExhausted,
                format!(
                    "probabilistic validation {description} counter \
                     exhausted its representation"
                ),
            )
        })?;

        Ok(())
    }

    fn record_construct(&mut self) -> AiResult<()> {
        Self::increment(
            &mut self.constructs_checked,
            "construct",
        )
    }

    fn record_expression(&mut self) -> AiResult<()> {
        Self::increment(
            &mut self.expressions_checked,
            "expression",
        )
    }

    fn record_construct_reference(&mut self) -> AiResult<()> {
        Self::increment(
            &mut self.construct_references_checked,
            "construct-reference",
        )
    }

    fn record_evidence_reference(&mut self) -> AiResult<()> {
        Self::increment(
            &mut self.evidence_references_checked,
            "evidence-reference",
        )
    }

    fn record_uncertainty_reference(&mut self) -> AiResult<()> {
        Self::increment(
            &mut self.uncertainty_references_checked,
            "uncertainty-reference",
        )
    }

    /// Return the number of probabilistic constructs checked.
    #[must_use]
    pub const fn constructs_checked(&self) -> usize {
        self.constructs_checked
    }

    /// Return the number of expression references checked.
    #[must_use]
    pub const fn expressions_checked(&self) -> usize {
        self.expressions_checked
    }

    /// Return the number of nested construct references checked.
    #[must_use]
    pub const fn construct_references_checked(&self) -> usize {
        self.construct_references_checked
    }

    /// Return the number of evidence references checked.
    #[must_use]
    pub const fn evidence_references_checked(&self) -> usize {
        self.evidence_references_checked
    }

    /// Return the number of uncertainty references checked.
    #[must_use]
    pub const fn uncertainty_references_checked(&self) -> usize {
        self.uncertainty_references_checked
    }

    /// Return whether all structural checks completed successfully.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }
}

// ============================================================================
// STRUCTURAL VALIDATION HELPERS
// ============================================================================

fn register_construct(
    registered: &mut BTreeSet<NodeId>,
    source_node: NodeId,
) -> AiResult<()> {
    if !registered.insert(source_node) {
        return Err(probabilistic_error(
            AiErrorKind::InvalidInput,
            format!(
                "probabilistic AST construct {} is registered more than once",
                source_node
            ),
        ));
    }

    Ok(())
}

fn validate_expression<R: ProbabilisticNodeResolver>(
    resolver: &R,
    expression: ProbabilisticExpressionRef,
    report: &mut ProbabilisticValidationReport,
) -> AiResult<()> {
    let node_id = expression.node_id;

    if !resolver.is_expression_node(node_id) {
        return Err(probabilistic_error(
            AiErrorKind::InvalidInput,
            format!(
                "AST node {} is not a valid canonical expression",
                node_id
            ),
        ));
    }

    report.record_expression()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct MockResolver {
        constructs: BTreeSet<NodeId>,
        expressions: BTreeSet<NodeId>,
        evidence: BTreeSet<NodeId>,
        uncertainty: BTreeSet<NodeId>,
    }

    impl MockResolver {
        fn new() -> Self {
            Self {
                constructs: BTreeSet::new(),
                expressions: BTreeSet::new(),
                evidence: BTreeSet::new(),
                uncertainty: BTreeSet::new(),
            }
        }

        fn with_construct(mut self, id: NodeId) -> Self {
            self.constructs.insert(id);
            self
        }

        fn with_expression(mut self, id: NodeId) -> Self {
            self.expressions.insert(id);
            self
        }

        fn with_evidence(mut self, id: NodeId) -> Self {
            self.evidence.insert(id);
            self
        }

        fn with_uncertainty(mut self, id: NodeId) -> Self {
            self.uncertainty.insert(id);
            self
        }
    }

    impl ProbabilisticNodeResolver for MockResolver {
        fn is_probabilistic_construct_node(
            &self,
            node_id: NodeId,
            _kind: ProbabilisticConstructKind,
        ) -> bool {
            self.constructs.contains(&node_id)
        }

        fn is_any_probabilistic_construct_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.constructs.contains(&node_id)
        }

        fn is_expression_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.expressions.contains(&node_id)
        }

        fn is_evidence_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.evidence.contains(&node_id)
        }

        fn is_uncertainty_node(
            &self,
            node_id: NodeId,
        ) -> bool {
            self.uncertainty.contains(&node_id)
        }
    }

    fn node(value: u64) -> NodeId {
        NodeId::new(value).expect("test node identity must be nonzero")
    }

    fn expression(
        value: u64,
        role: ProbabilisticExpressionRole,
    ) -> ProbabilisticExpression {
        ProbabilisticExpression::new(
            role,
            ProbabilisticExpressionRef::new(node(value)),
        )
    }

    #[test]
    fn creates_empty_program() {
        let program = ProbabilisticProgram::new();

        assert!(program.is_empty());
        assert_eq!(program.len(), 0);
    }

    #[test]
    fn qualified_names_are_validated() {
        let name = ProbabilisticName::new(
            "probability",
            "normal",
        )
        .unwrap();

        assert_eq!(name.namespace(), "probability");
        assert_eq!(name.name(), "normal");
        assert_eq!(name.qualified(), "probability.normal");
    }

    #[test]
    fn rejects_empty_name_components() {
        assert!(
            ProbabilisticName::new("", "normal").is_err()
        );

        assert!(
            ProbabilisticName::new("probability", "").is_err()
        );
    }

    #[test]
    fn rejects_control_characters_in_names() {
        assert!(
            ProbabilisticName::new(
                "probability\n",
                "normal"
            )
            .is_err()
        );
    }

    #[test]
    fn validates_a_probabilistic_declaration() {
        let source = node(1);
        let initializer = node(2);

        let program = ProbabilisticProgram::from_constructs(
            vec![
                ProbabilisticConstruct::new(
                    source,
                    ProbabilisticConstructKind::Declaration,
                )
                .with_expression(expression(
                    initializer.get(),
                    ProbabilisticExpressionRole::Initializer,
                )),
            ],
        );

        let resolver = MockResolver::new()
            .with_construct(source)
            .with_expression(initializer);

        let report = program.validate(&resolver).unwrap();

        assert!(report.is_complete());
        assert_eq!(report.constructs_checked(), 1);
        assert_eq!(report.expressions_checked(), 1);
    }

    #[test]
    fn rejects_unknown_source_construct() {
        let source = node(1);

        let program = ProbabilisticProgram::from_constructs(
            vec![
                ProbabilisticConstruct::new(
                    source,
                    ProbabilisticConstructKind::RandomVariable,
                ),
            ],
        );

        let resolver = MockResolver::new();

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn rejects_unknown_expression_reference() {
        let source = node(1);
        let missing_expression = node(2);

        let program = ProbabilisticProgram::from_constructs(
            vec![
                ProbabilisticConstruct::new(
                    source,
                    ProbabilisticConstructKind::Sample,
                )
                .with_expression(expression(
                    missing_expression.get(),
                    ProbabilisticExpressionRole::SampleSubject,
                )),
            ],
        );

        let resolver = MockResolver::new()
            .with_construct(source);

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn rejects_duplicate_construct_source_identities() {
        let source = node(1);

        let program = ProbabilisticProgram::from_constructs(
            vec![
                ProbabilisticConstruct::new(
                    source,
                    ProbabilisticConstructKind::Declaration,
                ),
                ProbabilisticConstruct::new(
                    source,
                    ProbabilisticConstructKind::Observation,
                ),
            ],
        );

        let resolver = MockResolver::new()
            .with_construct(source);

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn validates_evidence_and_uncertainty_references() {
        let source = node(1);
        let evidence = node(2);
        let uncertainty = node(3);

        let program = ProbabilisticProgram::from_constructs(
            vec![
                ProbabilisticConstruct::new(
                    source,
                    ProbabilisticConstructKind::Inference,
                )
                .with_evidence(
                    ProbabilisticEvidenceRef::new(evidence),
                )
                .with_uncertainty(
                    ProbabilisticUncertaintyRef::new(uncertainty),
                ),
            ],
        );

        let resolver = MockResolver::new()
            .with_construct(source)
            .with_evidence(evidence)
            .with_uncertainty(uncertainty);

        let report = program.validate(&resolver).unwrap();

        assert_eq!(report.evidence_references_checked(), 1);
        assert_eq!(report.uncertainty_references_checked(), 1);
    }

    #[test]
    fn rejects_unresolved_evidence_reference() {
        let source = node(1);
        let evidence = node(2);

        let program = ProbabilisticProgram::from_constructs(
            vec![
                ProbabilisticConstruct::new(
                    source,
                    ProbabilisticConstructKind::Observation,
                )
                .with_evidence(
                    ProbabilisticEvidenceRef::new(evidence),
                ),
            ],
        );

        let resolver = MockResolver::new()
            .with_construct(source);

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn rejects_unresolved_uncertainty_reference() {
        let source = node(1);
        let uncertainty = node(2);

        let program = ProbabilisticProgram::from_constructs(
            vec![
                ProbabilisticConstruct::new(
                    source,
                    ProbabilisticConstructKind::Inference,
                )
                .with_uncertainty(
                    ProbabilisticUncertaintyRef::new(uncertainty),
                ),
            ],
        );

        let resolver = MockResolver::new()
            .with_construct(source);

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn rejects_direct_self_membership() {
        let source = node(1);

        let program = ProbabilisticProgram::from_constructs(
            vec![
                ProbabilisticConstruct::new(
                    source,
                    ProbabilisticConstructKind::Region,
                )
                .with_member(
                    ProbabilisticConstructRef::new(source),
                ),
            ],
        );

        let resolver = MockResolver::new()
            .with_construct(source);

        assert!(program.validate(&resolver).is_err());
    }

    #[test]
    fn validates_nested_construct_references() {
        let region = node(1);
        let member = node(2);

        let program = ProbabilisticProgram::from_constructs(
            vec![
                ProbabilisticConstruct::new(
                    region,
                    ProbabilisticConstructKind::Region,
                )
                .with_member(
                    ProbabilisticConstructRef::new(member),
                ),
                ProbabilisticConstruct::new(
                    member,
                    ProbabilisticConstructKind::RandomVariable,
                ),
            ],
        );

        let resolver = MockResolver::new()
            .with_construct(region)
            .with_construct(member);

        let report = program.validate(&resolver).unwrap();

        assert_eq!(report.constructs_checked(), 2);
        assert_eq!(report.construct_references_checked(), 1);
    }

    #[test]
    fn supports_extension_defined_operations() {
        let name = ProbabilisticName::new(
            "extension",
            "custom_inference",
        )
        .unwrap();

        let operation = ProbabilisticOperation::new(name);

        let construct = ProbabilisticConstruct::new(
            node(1),
            ProbabilisticConstructKind::Extension,
        )
        .with_operation(operation);

        assert_eq!(
            construct.operation().unwrap().name().qualified(),
            "extension.custom_inference"
        );
    }

    #[test]
    fn preserves_insertion_order() {
        let first = node(1);
        let second = node(2);

        let program = ProbabilisticProgram::from_constructs(
            vec![
                ProbabilisticConstruct::new(
                    first,
                    ProbabilisticConstructKind::Prior,
                ),
                ProbabilisticConstruct::new(
                    second,
                    ProbabilisticConstructKind::Posterior,
                ),
            ],
        );

        assert_eq!(program.constructs()[0].source_node(), first);
        assert_eq!(program.constructs()[1].source_node(), second);
    }

    #[test]
    fn empty_program_validates_as_complete() {
        let program = ProbabilisticProgram::new();
        let resolver = MockResolver::new();

        let report = program.validate(&resolver).unwrap();

        assert!(report.is_complete());
        assert_eq!(report.constructs_checked(), 0);
        assert_eq!(report.expressions_checked(), 0);
    }
}
