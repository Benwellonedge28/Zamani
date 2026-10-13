
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Hybrid-Domain Semantic Validation
//!
//! Path: `src/frontend/semantic/domains/hybrid.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Validate cross-domain semantic relationships in hybrid Zamani programs.
//!
//! Hybrid programs may combine classical, quantum, HDL, AI/data, accelerator,
//! distributed, networking, and future computational domains. This module
//! checks the structural integrity of the resulting semantic composition
//! while delegating domain-specific meaning to the authoritative semantic
//! services.
//!
//! ## AUTHORITY
//!
//! - `grammar/Zamani.g4`
//! - `grammar/hybrid/hybrid.g4`
//! - `grammar/hybrid/classical-quantum.g4`
//! - `grammar/hybrid/quantum-classical-control.g4`
//! - `grammar/hybrid/accelerator-interoperability.g4`
//! - `grammar/hybrid/hybrid-resources.g4`
//! - `grammar/quantum/mid-circuit-control.g4`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/domains/classical.rs`
//! - `src/frontend/semantic/domains/quantum.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/frontend/semantic/domains/registry.rs`
//! - `src/frontend/semantic/types/`
//! - `src/frontend/semantic/effects/`
//! - `src/frontend/semantic/capabilities/`
//! - `src/frontend/semantic/resources/`
//! - `src/frontend/semantic/diagnostics/`
//!
//! The canonical grammar and normative specifications govern source meaning.
//! The canonical AST owns source structure and node identity. Shared semantic
//! services own name resolution, types, effects, capabilities, resources,
//! contracts, and provenance. This module owns hybrid composition checks.
//!
//! ## OWNS
//!
//! - The hybrid semantic input contract.
//! - Domain-qualified semantic identities used by this contract.
//! - Hybrid value-flow and dependency integrity checks.
//! - Cross-domain validation dispatch through shared services.
//! - Explicit synchronization validation dispatch.
//! - Deterministic hybrid issue classification and reporting.
//! - Analysis-completeness reporting.
//!
//! ## DOES_NOT_OWN
//!
//! - Lexing, parsing, grammar interpretation, or AST construction.
//! - A replacement AST or general-purpose symbol table.
//! - General type inference, unification, or generic substitution.
//! - Effect inference or effect authorization.
//! - Capability registration or target capability discovery.
//! - Resource feasibility, device allocation, or scheduling.
//! - Quantum operation registries or quantum IR.
//! - HDL elaboration or hardware IR.
//! - Classical IR generation or optimization.
//! - Backend selection, physical placement, routing, QEC, ZQN, HAL,
//!   or runtime execution.
//!
//! ## SCALABILITY
//!
//! No language-level maximum is imposed on domains, operations, values,
//! dependencies, crossings, or synchronization relationships.
//!
//! Standard dynamically sized collections are used. Operational budgets
//! belong to the semantic session. An incomplete analysis is never reported
//! as successful validation.
//!
//! Literal infinity is not a realizable input size. The design introduces no
//! arbitrary machine-capacity ceiling; actual processing is bounded by
//! representable values, available resources, and explicit operational policy.
//!
//! ## INTEGRATION CONTRACT
//!
//! 1. The AST adapter maps canonical AST nodes into `HybridProgram`.
//! 2. It preserves canonical source-node identities and qualified domains.
//! 3. It supplies resolved operation identities and typed value references.
//! 4. The shared semantic context checks types, operations, domain crossings,
//!    and synchronization.
//! 5. The pipeline invokes `validate_hybrid` after the required shared facts
//!    have been established.
//! 6. The diagnostic adapter maps `HybridIssue` into the repository's
//!    canonical diagnostic type.
//! 7. Finalized semantic facts feed the existing IR adapters. Quantum
//!    computation continues through the canonical `quantum::ir` boundary.
//!
//! This module is intentionally independent of unverified AST constructors
//! and diagnostic constructors. The integration adapter must bridge those
//! APIs explicitly rather than duplicating their ownership.
//!
//! ## FREEZE CRITERIA
//!
//! - The AST adapter is loss-preserving.
//! - Shared semantic services implement `HybridSemanticContext`.
//! - All supported hybrid constructs map to explicit input records.
//! - Unresolved inputs and incomplete analysis cannot be certified as valid.
//! - Value references and dependency identities are checked.
//! - Invalid dependency cycles are diagnosed deterministically.
//! - Domain-specific meaning is delegated rather than guessed.
//! - Unit, integration, determinism, fuzz, and stress tests pass.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::frontend::ast::node::node_id::NodeId;

// =============================================================================
// Public contract versions
// =============================================================================

/// Version of the hybrid semantic input and output contract.
///
/// This is independent of the language, AST, compiler, and IR schema versions.
pub const HYBRID_VALIDATION_SCHEMA_VERSION: u16 = 1;

/// Result type for hybrid input construction.
pub type HybridInputResult<T> = Result<T, HybridInputError>;

// =============================================================================
// Stable source-level identities
// =============================================================================

/// A qualified computational-domain identity.
///
/// This is a source-level identity, not a physical device, backend, or
/// execution-target identity.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HybridDomainId {
    namespace: String,
    name: String,
}

impl HybridDomainId {
    /// Construct a qualified domain identity.
    ///
    /// Empty components and control characters are rejected. No closed list
    /// of domain names is maintained here.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> HybridInputResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identity_component(&namespace, "domain namespace")?;
        validate_identity_component(&name, "domain name")?;

        Ok(Self { namespace, name })
    }

    /// Return the domain namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the domain name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the canonical qualified spelling.
    ///
    /// The returned string is for identity display and diagnostics. Semantic
    /// comparisons use the structured identity.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}.{}", self.namespace, self.name)
    }
}

impl fmt::Display for HybridDomainId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}", self.namespace, self.name)
    }
}

/// An identity for a semantic operation within a computational domain.
///
/// Operation identities are open-world: adding an operation does not require
/// changing this module.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HybridOperationId {
    domain: HybridDomainId,
    name: String,
}

impl HybridOperationId {
    /// Construct an operation identity.
    pub fn new(
        domain: HybridDomainId,
        name: impl Into<String>,
    ) -> HybridInputResult<Self> {
        let name = name.into();
        validate_identity_component(&name, "operation name")?;

        Ok(Self { domain, name })
    }

    /// Return the operation's domain.
    #[must_use]
    pub fn domain(&self) -> &HybridDomainId {
        &self.domain
    }

    /// Return the operation's local name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the canonical qualified operation spelling.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}::{}", self.domain, self.name)
    }
}

impl fmt::Display for HybridOperationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}::{}", self.domain, self.name)
    }
}

/// A stable identity within one hybrid semantic input.
///
/// The AST adapter should derive these identities deterministically from
/// canonical source identities. They are not hardware identifiers.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HybridEntityId(String);

impl HybridEntityId {
    /// Construct a non-empty entity identity.
    pub fn new(value: impl Into<String>) -> HybridInputResult<Self> {
        let value = value.into();
        validate_identity_component(&value, "hybrid entity identity")?;
        Ok(Self(value))
    }

    /// Return the underlying identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HybridEntityId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

fn validate_identity_component(
    value: &str,
    component: &'static str,
) -> HybridInputResult<()> {
    if value.trim().is_empty() {
        return Err(HybridInputError::EmptyIdentity { component });
    }

    if let Some(character) = value.chars().find(char::is_control) {
        return Err(HybridInputError::ControlCharacter {
            component,
            character,
        });
    }

    Ok(())
}

// =============================================================================
// Input records
// =============================================================================

/// The semantic role of a hybrid node.
///
/// These roles describe source-level composition. They do not select physical
/// devices or determine how computation is lowered.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum HybridNodeKind {
    /// An ordinary computation within a domain.
    Compute,

    /// An explicit source-level conversion between domain representations.
    Conversion,

    /// A synchronization operation or synchronization boundary.
    Synchronization,

    /// A structured control region, including conditional feedback.
    ControlRegion,

    /// An explicit domain-crossing boundary.
    DomainBoundary,

    /// A resource or execution-intent boundary whose rules are owned by
    /// the shared resource and capability subsystems.
    RequirementBoundary,

    /// A registered extension-defined semantic construct.
    Extension {
        /// Namespace-qualified extension identity.
        qualified_name: String,
    },
}

/// A semantic value produced by a hybrid operation.
///
/// `type_identity` is a reference to a type already resolved by the shared
/// type system. This module never implements type compatibility itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HybridValue {
    id: HybridEntityId,
    domain: HybridDomainId,
    type_identity: String,
    source_node: Option<NodeId>,
}

impl HybridValue {
    /// Construct a typed semantic value.
    pub fn new(
        id: HybridEntityId,
        domain: HybridDomainId,
        type_identity: impl Into<String>,
        source_node: Option<NodeId>,
    ) -> HybridInputResult<Self> {
        let type_identity = type_identity.into();
        validate_identity_component(&type_identity, "type identity")?;

        Ok(Self {
            id,
            domain,
            type_identity,
            source_node,
        })
    }

    /// Return this value's identity.
    #[must_use]
    pub fn id(&self) -> &HybridEntityId {
        &self.id
    }

    /// Return the domain that owns this logical value.
    #[must_use]
    pub fn domain(&self) -> &HybridDomainId {
        &self.domain
    }

    /// Return the shared semantic type identity.
    #[must_use]
    pub fn type_identity(&self) -> &str {
        &self.type_identity
    }

    /// Return the originating AST node, if available.
    #[must_use]
    pub fn source_node(&self) -> Option<&NodeId> {
        self.source_node.as_ref()
    }
}

/// A reference to an input value consumed by a hybrid node.
///
/// The optional expected type is a constraint supplied by the AST adapter
/// after the relevant declaration or operation contract has been resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HybridValueInput {
    value: HybridEntityId,
    expected_type: Option<String>,
}

impl HybridValueInput {
    /// Construct a value input.
    pub fn new(
        value: HybridEntityId,
        expected_type: Option<String>,
    ) -> HybridInputResult<Self> {
        if let Some(expected) = expected_type.as_deref() {
            validate_identity_component(expected, "expected type identity")?;
        }

        Ok(Self {
            value,
            expected_type,
        })
    }

    /// Return the referenced value identity.
    #[must_use]
    pub fn value(&self) -> &HybridEntityId {
        &self.value
    }

    /// Return the expected type identity, if specified.
    #[must_use]
    pub fn expected_type(&self) -> Option<&str> {
        self.expected_type.as_deref()
    }
}

/// A node participating in a hybrid semantic composition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HybridNode {
    id: HybridEntityId,
    source_node: Option<NodeId>,
    operation: HybridOperationId,
    kind: HybridNodeKind,
    inputs: Vec<HybridValueInput>,
    outputs: Vec<HybridValue>,
    synchronization_group: Option<HybridEntityId>,
}

impl HybridNode {
    /// Construct a hybrid semantic node.
    ///
    /// Duplicate node IDs and duplicate output IDs are checked when the
    /// containing program is validated, because uniqueness is a property of
    /// the whole program rather than this individual record.
    pub fn new(
        id: HybridEntityId,
        source_node: Option<NodeId>,
        operation: HybridOperationId,
        kind: HybridNodeKind,
        inputs: Vec<HybridValueInput>,
        outputs: Vec<HybridValue>,
        synchronization_group: Option<HybridEntityId>,
    ) -> Self {
        Self {
            id,
            source_node,
            operation,
            kind,
            inputs,
            outputs,
            synchronization_group,
        }
    }

    /// Return the node identity.
    #[must_use]
    pub fn id(&self) -> &HybridEntityId {
        &self.id
    }

    /// Return the canonical AST node identity, if available.
    #[must_use]
    pub fn source_node(&self) -> Option<&NodeId> {
        self.source_node.as_ref()
    }

    /// Return the resolved operation identity.
    #[must_use]
    pub fn operation(&self) -> &HybridOperationId {
        &self.operation
    }

    /// Return the node role.
    #[must_use]
    pub fn kind(&self) -> &HybridNodeKind {
        &self.kind
    }

    /// Return consumed value references.
    #[must_use]
    pub fn inputs(&self) -> &[HybridValueInput] {
        &self.inputs
    }

    /// Return produced values.
    #[must_use]
    pub fn outputs(&self) -> &[HybridValue] {
        &self.outputs
    }

    /// Return the synchronization group, if present.
    #[must_use]
    pub fn synchronization_group(&self) -> Option<&HybridEntityId> {
        self.synchronization_group.as_ref()
    }
}

/// The meaning of an explicit dependency edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HybridDependencyKind {
    /// A value-flow dependency.
    Data,

    /// A control-flow dependency.
    Control,

    /// An ordering constraint.
    Ordering,

    /// An explicitly represented feedback relationship.
    ///
    /// Feedback edges are retained in the semantic graph but excluded from
    /// the ordinary acyclic scheduling-dependency check. The domain-specific
    /// control validator must establish whether the feedback is legal.
    Feedback,
}

/// A directed dependency between hybrid semantic nodes.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HybridDependency {
    from: HybridEntityId,
    to: HybridEntityId,
    kind: HybridDependencyKind,
}

impl HybridDependency {
    /// Construct a dependency edge.
    #[must_use]
    pub fn new(
        from: HybridEntityId,
        to: HybridEntityId,
        kind: HybridDependencyKind,
    ) -> Self {
        Self { from, to, kind }
    }

    /// Return the dependency source.
    #[must_use]
    pub fn from(&self) -> &HybridEntityId {
        &self.from
    }

    /// Return the dependency destination.
    #[must_use]
    pub fn to(&self) -> &HybridEntityId {
        &self.to
    }

    /// Return the dependency kind.
    #[must_use]
    pub const fn kind(&self) -> HybridDependencyKind {
        self.kind
    }
}

/// One hybrid semantic program.
///
/// This is a semantic input model, not an AST replacement or intermediate
/// representation. The AST adapter remains responsible for preserving
/// source constructs and provenance.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HybridProgram {
    nodes: Vec<HybridNode>,
    dependencies: Vec<HybridDependency>,
}

impl HybridProgram {
    /// Create an empty hybrid program.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a node.
    pub fn add_node(&mut self, node: HybridNode) {
        self.nodes.push(node);
    }

    /// Append a dependency.
    pub fn add_dependency(&mut self, dependency: HybridDependency) {
        self.dependencies.push(dependency);
    }

    /// Return nodes in source-adapter insertion order.
    #[must_use]
    pub fn nodes(&self) -> &[HybridNode] {
        &self.nodes
    }

    /// Return explicit dependencies in insertion order.
    #[must_use]
    pub fn dependencies(&self) -> &[HybridDependency] {
        &self.dependencies
    }

    /// Return whether the input has no nodes.
    ///
    /// An empty graph is not automatically considered a valid source program;
    /// the enclosing language and pipeline decide whether it is permitted.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

// =============================================================================
// Shared semantic-service boundary
// =============================================================================

/// Classification of a failure reported by a shared semantic service.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HybridContextFailureKind {
    /// The service established that the construct violates a semantic rule.
    Invalid,

    /// The service could not establish validity because information or
    /// supported analysis was unavailable.
    Incomplete,
}

/// A failure returned by a shared semantic service.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HybridContextFailure {
    kind: HybridContextFailureKind,
    code: String,
    message: String,
}

impl HybridContextFailure {
    /// Construct an invalidity result.
    pub fn invalid(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> HybridInputResult<Self> {
        Self::new(HybridContextFailureKind::Invalid, code, message)
    }

    /// Construct an incomplete-analysis result.
    pub fn incomplete(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> HybridInputResult<Self> {
        Self::new(HybridContextFailureKind::Incomplete, code, message)
    }

    fn new(
        kind: HybridContextFailureKind,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> HybridInputResult<Self> {
        let code = code.into();
        let message = message.into();

        validate_identity_component(&code, "context diagnostic code")?;

        if message.trim().is_empty() {
            return Err(HybridInputError::EmptyIdentity {
                component: "context diagnostic message",
            });
        }

        Ok(Self {
            kind,
            code,
            message,
        })
    }
}

/// Shared semantic services required by hybrid validation.
///
/// Implementations must delegate to the canonical type, symbol, effect,
/// capability, resource, contract, and domain-validation services.
///
/// Methods return explicit failures rather than treating missing information
/// as successful validation. Implementations should be deterministic for a
/// fixed semantic context and extension-registry revision.
pub trait HybridSemanticContext {
    /// Validate a resolved operation and its node-level semantic contract.
    fn validate_operation(
        &self,
        node: &HybridNode,
    ) -> Result<(), HybridContextFailure>;

    /// Check whether a value type can satisfy an input type constraint.
    ///
    /// This must use the canonical type compatibility rules. Returning `Ok`
    /// asserts that the compatibility obligation has been discharged.
    fn check_type_compatibility(
        &self,
        value_type: &str,
        expected_type: &str,
        source_node: Option<&NodeId>,
        consumer_node: Option<&NodeId>,
    ) -> Result<(), HybridContextFailure>;

    /// Validate a value crossing from its producer domain to its consumer
    /// domain.
    ///
    /// This hook owns domain-specific restrictions, permitted conversions,
    /// ownership/lifetime rules, and required effects or capabilities as
    /// appropriate. It must not infer target availability from source syntax.
    fn validate_domain_transition(
        &self,
        producer: &HybridNode,
        consumer: &HybridNode,
        value: &HybridValue,
    ) -> Result<(), HybridContextFailure>;

    /// Validate synchronization semantics for the supplied group.
    ///
    /// This hook checks the language-defined synchronization contract.
    /// It must not perform runtime synchronization or hardware scheduling.
    fn validate_synchronization(
        &self,
        group: &HybridEntityId,
        members: &[&HybridNode],
    ) -> Result<(), HybridContextFailure>;
}

// =============================================================================
// Diagnostics and results
// =============================================================================

/// Severity of a hybrid validation issue.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HybridIssueSeverity {
    /// The analysis found a semantic violation.
    Error,

    /// Analysis could not establish a semantic fact.
    Incomplete,
}

/// Stable hybrid validation issue categories.
///
/// Final public diagnostic codes remain owned by the repository's canonical
/// diagnostic registry. These rule identifiers are local to this module.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HybridIssueKind {
    /// A node identity is duplicated.
    DuplicateNode,

    /// A produced value identity is duplicated.
    DuplicateValue,

    /// A node references an unknown input value.
    UnresolvedValue,

    /// A value has no valid producer.
    MissingProducer,

    /// A node references a dependency endpoint that does not exist.
    UnknownDependencyEndpoint,

    /// A self-dependency or ordinary dependency cycle exists.
    DependencyCycle,

    /// The operation's domain conflicts with its value domain.
    DomainIdentityMismatch,

    /// A value's type or a domain crossing failed shared validation.
    TypeOrBoundaryViolation,

    /// An operation failed shared semantic validation.
    OperationViolation,

    /// Synchronization failed shared semantic validation.
    SynchronizationViolation,

    /// A synchronization group is malformed.
    InvalidSynchronizationGroup,

    /// Shared semantic information is missing or analysis is unsupported.
    IncompleteAnalysis,

    /// A dependency is duplicated.
    DuplicateDependency,
}

/// One structured hybrid semantic issue.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HybridIssue {
    severity: HybridIssueSeverity,
    kind: HybridIssueKind,
    rule: String,
    message: String,
    entity: Option<HybridEntityId>,
    source_node: Option<NodeId>,
}

impl HybridIssue {
    fn new(
        severity: HybridIssueSeverity,
        kind: HybridIssueKind,
        rule: impl Into<String>,
        message: impl Into<String>,
        entity: Option<HybridEntityId>,
        source_node: Option<NodeId>,
    ) -> Self {
        Self {
            severity,
            kind,
            rule: rule.into(),
            message: message.into(),
            entity,
            source_node,
        }
    }

    /// Return issue severity.
    #[must_use]
    pub const fn severity(&self) -> HybridIssueSeverity {
        self.severity
    }

    /// Return the issue category.
    #[must_use]
    pub const fn kind(&self) -> HybridIssueKind {
        self.kind
    }

    /// Return the stable local rule identifier.
    #[must_use]
    pub fn rule(&self) -> &str {
        &self.rule
    }

    /// Return the human-readable message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the relevant hybrid entity, if any.
    #[must_use]
    pub fn entity(&self) -> Option<&HybridEntityId> {
        self.entity.as_ref()
    }

    /// Return the relevant canonical AST node, if available.
    #[must_use]
    pub fn source_node(&self) -> Option<&NodeId> {
        self.source_node.as_ref()
    }
}

/// Completion state of hybrid validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HybridCompletion {
    /// All required checks ran to completion.
    Complete,

    /// At least one required check could not establish a result.
    Incomplete,
}

/// The report returned by hybrid semantic validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HybridValidation {
    completion: HybridCompletion,
    issues: Vec<HybridIssue>,
    validated_nodes: usize,
    validated_values: usize,
    validated_dependencies: usize,
}

impl HybridValidation {
    /// Return whether all required checks completed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(self.completion, HybridCompletion::Complete)
    }

    /// Return whether analysis is complete and no semantic errors were found.
    ///
    /// An incomplete report is never valid, even if no error issue was emitted.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.is_complete()
            && !self
                .issues
                .iter()
                .any(|issue| issue.severity == HybridIssueSeverity::Error)
    }

    /// Return the completion state.
    #[must_use]
    pub const fn completion(&self) -> HybridCompletion {
        self.completion
    }

    /// Return issues in deterministic validation order.
    #[must_use]
    pub fn issues(&self) -> &[HybridIssue] {
        &self.issues
    }

    /// Return the number of nodes checked.
    #[must_use]
    pub const fn validated_nodes(&self) -> usize {
        self.validated_nodes
    }

    /// Return the number of values checked.
    #[must_use]
    pub const fn validated_values(&self) -> usize {
        self.validated_values
    }

    /// Return the number of dependencies checked.
    #[must_use]
    pub const fn validated_dependencies(&self) -> usize {
        self.validated_dependencies
    }
}

/// Errors caused by malformed construction input rather than invalid source
/// semantics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HybridInputError {
    /// An identity component was empty.
    EmptyIdentity {
        /// Name of the identity component.
        component: &'static str,
    },

    /// An identity component contained a control character.
    ControlCharacter {
        /// Name of the identity component.
        component: &'static str,

        /// Invalid character.
        character: char,
    },
}

impl fmt::Display for HybridInputError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIdentity { component } => {
                write!(formatter, "{component} must not be empty")
            }
            Self::ControlCharacter {
                component,
                character,
            } => {
                write!(
                    formatter,
                    "{component} contains control character U+{:04X}",
                    *character as u32
                )
            }
        }
    }
}

impl std::error::Error for HybridInputError {}

// =============================================================================
// Validation
// =============================================================================

/// Validate one hybrid semantic program.
///
/// The validator:
///
/// - checks node and value identity uniqueness;
/// - verifies input references and dependency endpoints;
/// - checks that operation and value domain identities agree;
/// - delegates operation, type, transition, and synchronization semantics;
/// - derives data dependencies from resolved value flow;
/// - rejects cycles in ordinary dependencies while preserving explicitly
///   marked feedback relationships for domain-specific validation;
/// - returns an incomplete result whenever a required shared check cannot
///   establish validity.
///
/// Diagnostics are emitted in deterministic passes. Node validation follows
/// input order; dependency-cycle analysis uses ordered identities.
pub fn validate_hybrid<C: HybridSemanticContext + ?Sized>(
    program: &HybridProgram,
    context: &C,
) -> HybridValidation {
    let mut state = ValidationState::default();

    let mut nodes_by_id: BTreeMap<HybridEntityId, &HybridNode> = BTreeMap::new();
    let mut values_by_id: BTreeMap<HybridEntityId, (&HybridValue, &HybridNode)> =
        BTreeMap::new();

    // Pass 1: collect identities without overwriting the first definition.
    for node in &program.nodes {
        if nodes_by_id.contains_key(node.id()) {
            state.error(
                HybridIssueKind::DuplicateNode,
                "HYB0001",
                format!("duplicate hybrid node identity `{}`", node.id()),
                Some(node.id().clone()),
                node.source_node().cloned(),
            );
        } else {
            nodes_by_id.insert(node.id().clone(), node);
        }

        if node.operation().domain() != &node.operation.domain {
            // This comparison is intentionally retained as a compile-time
            // structural invariant should the operation representation evolve.
            state.error(
                HybridIssueKind::DomainIdentityMismatch,
                "HYB0002",
                "operation domain identity is inconsistent",
                Some(node.id().clone()),
                node.source_node().cloned(),
            );
        }

        for value in node.outputs() {
            if &value.domain != node.operation().domain() {
                state.error(
                    HybridIssueKind::DomainIdentityMismatch,
                    "HYB0003",
                    format!(
                        "output `{}` belongs to domain `{}`, but its producer belongs to `{}`",
                        value.id(),
                        value.domain(),
                        node.operation().domain()
                    ),
                    Some(node.id().clone()),
                    value.source_node().cloned().or_else(|| node.source_node().cloned()),
                );
            }

            if values_by_id.contains_key(value.id()) {
                state.error(
                    HybridIssueKind::DuplicateValue,
                    "HYB0004",
                    format!("duplicate hybrid value identity `{}`", value.id()),
                    Some(value.id().clone()),
                    value.source_node().cloned(),
                );
            } else {
                values_by_id.insert(value.id().clone(), (value, node));
            }
        }
    }

    // Pass 2: verify explicit dependency endpoints and duplicates.
    let mut unique_dependencies = BTreeSet::new();

    for dependency in program.dependencies() {
        if !unique_dependencies.insert(dependency.clone()) {
            state.error(
                HybridIssueKind::DuplicateDependency,
                "HYB0005",
                format!(
                    "duplicate dependency from `{}` to `{}`",
                    dependency.from(),
                    dependency.to()
                ),
                Some(dependency.to().clone()),
                nodes_by_id
                    .get(dependency.to())
                    .and_then(|node| node.source_node().cloned()),
            );
            continue;
        }

        if !nodes_by_id.contains_key(dependency.from()) {
            state.error(
                HybridIssueKind::UnknownDependencyEndpoint,
                "HYB0006",
                format!(
                    "dependency source `{}` does not identify a hybrid node",
                    dependency.from()
                ),
                Some(dependency.from().clone()),
                None,
            );
        }

        if !nodes_by_id.contains_key(dependency.to()) {
            state.error(
                HybridIssueKind::UnknownDependencyEndpoint,
                "HYB0007",
                format!(
                    "dependency destination `{}` does not identify a hybrid node",
                    dependency.to()
                ),
                Some(dependency.to().clone()),
                None,
            );
        }

        if dependency.from() == dependency.to()
            && dependency.kind() != HybridDependencyKind::Feedback
        {
            state.error(
                HybridIssueKind::DependencyCycle,
                "HYB0008",
                format!(
                    "ordinary dependency for node `{}` refers to itself",
                    dependency.from()
                ),
                Some(dependency.from().clone()),
                nodes_by_id
                    .get(dependency.from())
                    .and_then(|node| node.source_node().cloned()),
            );
        }
    }

    // Pass 3: validate operations and value flow.
    for node in &program.nodes {
        match context.validate_operation(node) {
            Ok(()) => {}
            Err(failure) => state.context_failure(
                failure,
                HybridIssueKind::OperationViolation,
                Some(node.id().clone()),
                node.source_node().cloned(),
            ),
        }

        for input in node.inputs() {
            let Some((value, producer)) = values_by_id.get(input.value()) else {
                state.error(
                    HybridIssueKind::UnresolvedValue,
                    "HYB0010",
                    format!(
                        "node `{}` consumes unresolved value `{}`",
                        node.id(),
                        input.value()
                    ),
                    Some(node.id().clone()),
                    node.source_node().cloned(),
                );
                continue;
            };

            // Every ordinary value use establishes an implicit data dependency.
            // A source adapter may also supply an explicit edge; duplicate
            // semantic edges are coalesced for cycle analysis.
            if producer.id() == node.id() {
                state.error(
                    HybridIssueKind::DependencyCycle,
                    "HYB0011",
                    format!(
                        "node `{}` consumes its own output `{}`",
                        node.id(),
                        input.value()
                    ),
                    Some(node.id().clone()),
                    node.source_node().cloned(),
                );
            }

            if let Some(expected_type) = input.expected_type() {
                match context.check_type_compatibility(
                    value.type_identity(),
                    expected_type,
                    value.source_node(),
                    node.source_node(),
                ) {
                    Ok(()) => {}
                    Err(failure) => state.context_failure(
                        failure,
                        HybridIssueKind::TypeOrBoundaryViolation,
                        Some(node.id().clone()),
                        node.source_node().cloned(),
                    ),
                }
            }

            if value.domain() != node.operation().domain() {
                match context.validate_domain_transition(producer, node, value) {
                    Ok(()) => {}
                    Err(failure) => state.context_failure(
                        failure,
                        HybridIssueKind::TypeOrBoundaryViolation,
                        Some(node.id().clone()),
                        node.source_node().cloned(),
                    ),
                }
            }
        }
    }

    // Pass 4: validate synchronization groups.
    let mut synchronization_groups: BTreeMap<
        HybridEntityId,
        Vec<&HybridNode>,
    > = BTreeMap::new();

    for node in &program.nodes {
        if let Some(group) = node.synchronization_group() {
            synchronization_groups
                .entry(group.clone())
                .or_default()
                .push(node);
        }

        if matches!(node.kind(), HybridNodeKind::Synchronization)
            && node.synchronization_group().is_none()
        {
            state.error(
                HybridIssueKind::InvalidSynchronizationGroup,
                "HYB0012",
                format!(
                    "synchronization node `{}` has no synchronization group",
                    node.id()
                ),
                Some(node.id().clone()),
                node.source_node().cloned(),
            );
        }
    }

    for (group, members) in &synchronization_groups {
        match context.validate_synchronization(group, members) {
            Ok(()) => {}
            Err(failure) => state.context_failure(
                failure,
                HybridIssueKind::SynchronizationViolation,
                Some(group.clone()),
                members.first().and_then(|node| node.source_node().cloned()),
            ),
        }
    }

    // Pass 5: cycle detection over ordinary dependencies and value flow.
    //
    // Explicit feedback is kept out of the ordinary dependency DAG. It is not
    // automatically considered semantically valid: its control/effect rules
    // must be checked by the relevant domain context.
    let mut adjacency: BTreeMap<HybridEntityId, BTreeSet<HybridEntityId>> =
        BTreeMap::new();

    let mut indegree: BTreeMap<HybridEntityId, usize> = nodes_by_id
        .keys()
        .cloned()
        .map(|id| (id, 0))
        .collect();

    for dependency in program.dependencies() {
        if dependency.kind() == HybridDependencyKind::Feedback {
            continue;
        }

        add_graph_edge(
            dependency.from(),
            dependency.to(),
            &mut adjacency,
            &mut indegree,
        );
    }

    for node in &program.nodes {
        for input in node.inputs() {
            let Some((_, producer)) = values_by_id.get(input.value()) else {
                continue;
            };

            if producer.id() == node.id() {
                continue;
            }

            add_graph_edge(
                producer.id(),
                node.id(),
                &mut adjacency,
                &mut indegree,
            );
        }
    }

    let mut ready: BTreeSet<HybridEntityId> = indegree
        .iter()
        .filter_map(|(id, degree)| (*degree == 0).then_some(id.clone()))
        .collect();

    let mut visited = 0usize;

    while let Some(current) = ready.pop_first() {
        visited = visited.saturating_add(1);

        if let Some(neighbors) = adjacency.get(&current) {
            for neighbor in neighbors {
                let Some(degree) = indegree.get_mut(neighbor) else {
                    continue;
                };

                *degree = degree.saturating_sub(1);

                if *degree == 0 {
                    ready.insert(neighbor.clone());
                }
            }
        }
    }

    if visited < indegree.len() {
        let cyclic: Vec<String> = indegree
            .iter()
            .filter_map(|(id, degree)| {
                (*degree > 0).then(|| id.to_string())
            })
            .collect();

        state.error(
            HybridIssueKind::DependencyCycle,
            "HYB0013",
            format!(
                "ordinary hybrid dependency graph contains a cycle involving: {}",
                cyclic.join(", ")
            ),
            None,
            None,
        );
    }

    // The report is deterministic: source-order passes are preserved, and
    // cycle participants are collected from ordered maps.
    HybridValidation {
        completion: if state.incomplete {
            HybridCompletion::Incomplete
        } else {
            HybridCompletion::Complete
        },
        issues: state.issues,
        validated_nodes: program.nodes.len(),
        validated_values: values_by_id.len(),
        validated_dependencies: program.dependencies.len(),
    }
}

fn add_graph_edge(
    from: &HybridEntityId,
    to: &HybridEntityId,
    adjacency: &mut BTreeMap<HybridEntityId, BTreeSet<HybridEntityId>>,
    indegree: &mut BTreeMap<HybridEntityId, usize>,
) {
    if !indegree.contains_key(from) || !indegree.contains_key(to) {
        return;
    }

    let neighbors = adjacency.entry(from.clone()).or_default();

    // A repeated edge must not inflate the indegree.
    if neighbors.insert(to.clone()) {
        if let Some(degree) = indegree.get_mut(to) {
            *degree = degree.saturating_add(1);
        }
    }
}

#[derive(Default)]
struct ValidationState {
    issues: Vec<HybridIssue>,
    incomplete: bool,
}

impl ValidationState {
    fn error(
        &mut self,
        kind: HybridIssueKind,
        rule: &'static str,
        message: impl Into<String>,
        entity: Option<HybridEntityId>,
        source_node: Option<NodeId>,
    ) {
        self.issues.push(HybridIssue::new(
            HybridIssueSeverity::Error,
            kind,
            rule,
            message,
            entity,
            source_node,
        ));
    }

    fn context_failure(
        &mut self,
        failure: HybridContextFailure,
        default_kind: HybridIssueKind,
        entity: Option<HybridEntityId>,
        source_node: Option<NodeId>,
    ) {
        let (severity, kind) = match failure.kind {
            HybridContextFailureKind::Invalid => {
                (HybridIssueSeverity::Error, default_kind)
            }
            HybridContextFailureKind::Incomplete => {
                self.incomplete = true;
                (
                    HybridIssueSeverity::Incomplete,
                    HybridIssueKind::IncompleteAnalysis,
                )
            }
        };

        self.issues.push(HybridIssue::new(
            severity,
            kind,
            failure.code,
            failure.message,
            entity,
            source_node,
        ));
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestContext {
        invalid_operations: BTreeSet<String>,
        invalid_types: BTreeSet<(String, String)>,
        invalid_transitions: BTreeSet<(HybridDomainId, HybridDomainId)>,
        incomplete_operations: BTreeSet<String>,
        invalid_sync_groups: BTreeSet<HybridEntityId>,
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
                        "TEST_INCOMPLETE",
                        format!("operation `{operation}` is unresolved"),
                    )
                    .expect("test diagnostic is valid"),
                );
            }

            if self.invalid_operations.contains(&operation) {
                return Err(
                    HybridContextFailure::invalid(
                        "TEST_OPERATION",
                        format!("operation `{operation}` is invalid"),
                    )
                    .expect("test diagnostic is valid"),
                );
            }

            Ok(())
        }

        fn check_type_compatibility(
            &self,
            value_type: &str,
            expected_type: &str,
            _source_node: Option<&NodeId>,
            _consumer_node: Option<&NodeId>,
        ) -> Result<(), HybridContextFailure> {
            if self
                .invalid_types
                .contains(&(value_type.to_owned(), expected_type.to_owned()))
            {
                return Err(
                    HybridContextFailure::invalid(
                        "TEST_TYPE",
                        format!(
                            "type `{value_type}` is not compatible with `{expected_type}`"
                        ),
                    )
                    .expect("test diagnostic is valid"),
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

            if self.invalid_transitions.contains(&transition) {
                return Err(
                    HybridContextFailure::invalid(
                        "TEST_TRANSITION",
                        "domain transition is not permitted",
                    )
                    .expect("test diagnostic is valid"),
                );
            }

            Ok(())
        }

        fn validate_synchronization(
            &self,
            group: &HybridEntityId,
            _members: &[&HybridNode],
        ) -> Result<(), HybridContextFailure> {
            if self.invalid_sync_groups.contains(group) {
                return Err(
                    HybridContextFailure::invalid(
                        "TEST_SYNC",
                        "synchronization group is invalid",
                    )
                    .expect("test diagnostic is valid"),
                );
            }

            Ok(())
        }
    }

    fn entity(value: &str) -> HybridEntityId {
        HybridEntityId::new(value).expect("test identity is valid")
    }

    fn domain(namespace: &str, name: &str) -> HybridDomainId {
        HybridDomainId::new(namespace, name).expect("test domain is valid")
    }

    fn operation(domain: HybridDomainId, name: &str) -> HybridOperationId {
        HybridOperationId::new(domain, name).expect("test operation is valid")
    }

    fn value(
        id: &str,
        domain: HybridDomainId,
        ty: &str,
    ) -> HybridValue {
        HybridValue::new(entity(id), domain, ty, None)
            .expect("test value is valid")
    }

    fn node(
        id: &str,
        operation: HybridOperationId,
        inputs: Vec<HybridValueInput>,
        outputs: Vec<HybridValue>,
    ) -> HybridNode {
        HybridNode::new(
            entity(id),
            None,
            operation,
            HybridNodeKind::Compute,
            inputs,
            outputs,
            None,
        )
    }

    fn input(id: &str, expected_type: Option<&str>) -> HybridValueInput {
        HybridValueInput::new(
            entity(id),
            expected_type.map(str::to_owned),
        )
        .expect("test input is valid")
    }

    #[test]
    fn empty_program_is_structurally_valid() {
        let report = validate_hybrid(&HybridProgram::new(), &TestContext::default());

        assert!(report.is_valid());
        assert!(report.is_complete());
    }

    #[test]
    fn validates_classical_to_quantum_value_flow_through_context() {
        let classical = domain("zamani", "classical");
        let quantum = domain("zamani", "quantum");

        let producer = node(
            "classical-producer",
            operation(classical.clone(), "prepare_input"),
            Vec::new(),
            vec![value("input-value", classical, "Scalar")],
        );

        let consumer = node(
            "quantum-consumer",
            operation(quantum, "consume_input"),
            vec![input("input-value", Some("Scalar"))],
            Vec::new(),
        );

        let mut program = HybridProgram::new();
        program.add_node(producer);
        program.add_node(consumer);

        let report = validate_hybrid(&program, &TestContext::default());

        assert!(report.is_valid(), "{:?}", report.issues());
        assert_eq!(report.validated_nodes(), 2);
        assert_eq!(report.validated_values(), 1);
    }

    #[test]
    fn unresolved_input_is_not_certified_valid() {
        let domain = domain("zamani", "classical");

        let consumer = node(
            "consumer",
            operation(domain, "consume"),
            vec![input("missing-value", None)],
            Vec::new(),
        );

        let mut program = HybridProgram::new();
        program.add_node(consumer);

        let report = validate_hybrid(&program, &TestContext::default());

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.kind() == HybridIssueKind::UnresolvedValue
        }));
    }

    #[test]
    fn rejects_duplicate_node_identities() {
        let domain = domain("zamani", "classical");
        let op = operation(domain, "work");

        let mut program = HybridProgram::new();
        program.add_node(node("duplicate", op.clone(), Vec::new(), Vec::new()));
        program.add_node(node("duplicate", op, Vec::new(), Vec::new()));

        let report = validate_hybrid(&program, &TestContext::default());

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.kind() == HybridIssueKind::DuplicateNode
        }));
    }

    #[test]
    fn rejects_duplicate_value_identities() {
        let domain = domain("zamani", "classical");

        let mut program = HybridProgram::new();
        program.add_node(node(
            "first",
            operation(domain.clone(), "first"),
            Vec::new(),
            vec![value("same-value", domain.clone(), "Scalar")],
        ));
        program.add_node(node(
            "second",
            operation(domain.clone(), "second"),
            Vec::new(),
            vec![value("same-value", domain, "Scalar")],
        ));

        let report = validate_hybrid(&program, &TestContext::default());

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.kind() == HybridIssueKind::DuplicateValue
        }));
    }

    #[test]
    fn reports_invalid_type_compatibility_from_shared_service() {
        let domain = domain("zamani", "classical");

        let producer = node(
            "producer",
            operation(domain.clone(), "produce"),
            Vec::new(),
            vec![value("result", domain.clone(), "Text")],
        );
        let consumer = node(
            "consumer",
            operation(domain, "consume"),
            vec![input("result", Some("Scalar"))],
            Vec::new(),
        );

        let mut program = HybridProgram::new();
        program.add_node(producer);
        program.add_node(consumer);

        let context = TestContext {
            invalid_types: BTreeSet::from([("Text".into(), "Scalar".into())]),
            ..TestContext::default()
        };

        let report = validate_hybrid(&program, &context);

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.rule() == "TEST_TYPE"
        }));
    }

    #[test]
    fn reports_incomplete_analysis_separately_from_invalidity() {
        let domain = domain("zamani", "classical");
        let op = operation(domain, "unresolved_operation");

        let mut program = HybridProgram::new();
        program.add_node(node("node", op.clone(), Vec::new(), Vec::new()));

        let context = TestContext {
            incomplete_operations: BTreeSet::from([op.qualified_name()]),
            ..TestContext::default()
        };

        let report = validate_hybrid(&program, &context);

        assert!(!report.is_valid());
        assert!(!report.is_complete());
        assert!(report.issues().iter().any(|issue| {
            issue.severity() == HybridIssueSeverity::Incomplete
        }));
    }

    #[test]
    fn detects_ordinary_dependency_cycles() {
        let domain = domain("zamani", "classical");

        let mut program = HybridProgram::new();
        program.add_node(node(
            "a",
            operation(domain.clone(), "a"),
            Vec::new(),
            Vec::new(),
        ));
        program.add_node(node(
            "b",
            operation(domain, "b"),
            Vec::new(),
            Vec::new(),
        ));
        program.add_dependency(HybridDependency::new(
            entity("a"),
            entity("b"),
            HybridDependencyKind::Ordering,
        ));
        program.add_dependency(HybridDependency::new(
            entity("b"),
            entity("a"),
            HybridDependencyKind::Control,
        ));

        let report = validate_hybrid(&program, &TestContext::default());

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.kind() == HybridIssueKind::DependencyCycle
        }));
    }

    #[test]
    fn explicit_feedback_is_not_misclassified_as_an_ordinary_cycle() {
        let domain = domain("zamani", "classical");

        let mut program = HybridProgram::new();
        program.add_node(node(
            "a",
            operation(domain.clone(), "a"),
            Vec::new(),
            Vec::new(),
        ));
        program.add_node(node(
            "b",
            operation(domain, "b"),
            Vec::new(),
            Vec::new(),
        ));
        program.add_dependency(HybridDependency::new(
            entity("a"),
            entity("b"),
            HybridDependencyKind::Ordering,
        ));
        program.add_dependency(HybridDependency::new(
            entity("b"),
            entity("a"),
            HybridDependencyKind::Feedback,
        ));

        let report = validate_hybrid(&program, &TestContext::default());

        assert!(report.is_valid(), "{:?}", report.issues());
    }

    #[test]
    fn detects_invalid_domain_transition_via_shared_service() {
        let classical = domain("zamani", "classical");
        let quantum = domain("zamani", "quantum");

        let producer = node(
            "producer",
            operation(classical.clone(), "produce"),
            Vec::new(),
            vec![value("result", classical.clone(), "Scalar")],
        );
        let consumer = node(
            "consumer",
            operation(quantum.clone(), "consume"),
            vec![input("result", None)],
            Vec::new(),
        );

        let mut program = HybridProgram::new();
        program.add_node(producer);
        program.add_node(consumer);

        let context = TestContext {
            invalid_transitions: BTreeSet::from([(classical, quantum)]),
            ..TestContext::default()
        };

        let report = validate_hybrid(&program, &context);

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.rule() == "TEST_TRANSITION"
        }));
    }

    #[test]
    fn synchronization_requires_a_group_identity() {
        let domain = domain("zamani", "classical");
        let node = HybridNode::new(
            entity("sync"),
            None,
            operation(domain, "synchronize"),
            HybridNodeKind::Synchronization,
            Vec::new(),
            Vec::new(),
            None,
        );

        let mut program = HybridProgram::new();
        program.add_node(node);

        let report = validate_hybrid(&program, &TestContext::default());

        assert!(!report.is_valid());
        assert!(report.issues().iter().any(|issue| {
            issue.kind() == HybridIssueKind::InvalidSynchronizationGroup
        }));
    }

    #[test]
    fn domain_identity_supports_future_domains() {
        let domain = domain(
            "future.example",
            "photonic_computing",
        );

        assert_eq!(
            domain.qualified_name(),
            "future.example.photonic_computing"
        );
    }

    #[test]
    fn identity_validation_rejects_control_characters() {
        let result = HybridDomainId::new("zamani\ninvalid", "classical");

        assert!(matches!(
            result,
            Err(HybridInputError::ControlCharacter { .. })
        ));
    }
}
