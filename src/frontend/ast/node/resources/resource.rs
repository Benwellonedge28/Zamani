//! # Zamani Native AST — Resource
//!
//! Canonical source-level representation of a Zamani resource declaration or
//! resource reference.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer
//!     │
//!     ▼
//! parser
//!     │
//!     ▼
//! Resource AST  ← this module
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ├── name resolution
//!     ├── type/resource semantics
//!     ├── capability analysis
//!     ├── ownership/lifetime analysis
//!     ├── quantum-resource analysis
//!     └── domain analysis
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! domain IR
//!     │
//!     ▼
//! target/resource discovery
//!     │
//!     ▼
//! allocation / mapping / scheduling / execution
//! ```
//!
//! ## Critical architectural boundary
//!
//! This type belongs to the **native Zamani AST**.
//!
//! It therefore represents programmer intent and source structure only.
//!
//! It does NOT represent:
//!
//! - a physical qubit;
//! - a physical memory address;
//! - a hardware allocation;
//! - a backend resource;
//! - a QPU device;
//! - a device topology;
//! - a routing assignment;
//! - a scheduler reservation;
//! - a calibration record;
//! - a pulse channel;
//! - a QIR value;
//! - an LLVM value;
//! - an MLIR operation;
//! - a quantum-IR `ResourceRequirement`;
//! - a runtime resource handle.
//!
//! Those representations belong to later compiler/runtime layers.
//!
//! ## POCO-REAF
//!
//! A resource declaration is source-level intent.
//!
//! The same AST representation must remain valid whether a program is later
//! lowered to:
//!
//! - one CPU;
//! - many CPUs;
//! - one GPU;
//! - many GPUs;
//! - one FPGA;
//! - many FPGAs;
//! - one QPU;
//! - many QPUs;
//! - a simulator;
//! - a distributed machine;
//! - a heterogeneous system;
//! - a future computational substrate.
//!
//! Consequently this file contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_RESOURCES
//! MAX_DEVICES
//! MAX_MACHINE_SIZE
//! MAX_REGISTER_SIZE
//! ```
//!
//! A compiler may impose explicit resource-safety policies elsewhere. Such
//! policies are not language semantics and must not be encoded in this node.
//!
//! ## Domain neutrality
//!
//! Resource syntax is deliberately generic.
//!
//! A resource can eventually denote a:
//!
//! - classical resource;
//! - quantum resource;
//! - logical quantum resource;
//! - accelerator resource;
//! - memory resource;
//! - communication resource;
//! - distributed resource;
//! - timing resource;
//! - computational capability;
//! - future resource class.
//!
//! This file does not contain a closed enumeration such as:
//!
//! ```text
//! QuantumResource
//! CpuResource
//! GpuResource
//! ...
//! ```
//!
//! Such a design would require the AST to change whenever a new technology is
//! introduced.
//!
//! ## Child-node ownership
//!
//! Child AST nodes are referenced by `NodeId`.
//!
//! This file does not own the global AST graph.
//!
//! This is important because a later change to the representation of:
//!
//! - names;
//! - types;
//! - expressions;
//! - generic arguments;
//! - attributes;
//! - annotations;
//! - constraints;
//!
//! does not require this file to own or duplicate those structures.
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational AST infrastructure:
//!
//! - `Node`;
//! - `AstNode`;
//! - `NodeId`;
//! - `NodeKind`;
//! - `CoreNodeKind`;
//! - `NodeMetadata`;
//! - `Span`;
//! - Rust standard-library facilities;
//! - Serde.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - QEC;
//! - ZQN;
//! - routing;
//! - scheduling;
//! - calibration;
//! - runtime;
//! - backend SDKs;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - vendor-specific APIs.
//!
//! ## Integration contract
//!
//! Parser:
//!
//! ```text
//! parser
//!   │
//!   ├── allocate NodeId
//!   ├── parse resource name/reference
//!   ├── parse optional type
//!   ├── parse optional cardinality expression
//!   ├── parse optional scope
//!   ├── parse optional initializer
//!   ├── parse optional attributes
//!   └── construct Resource
//! ```
//!
//! Structural validation:
//!
//! ```text
//! Resource
//!   │
//!   ├── validate node kind
//!   ├── validate local identifiers
//!   ├── validate child IDs
//!   ├── validate collection structure
//!   └── validate source-level invariants
//! ```
//!
//! Semantic analysis:
//!
//! ```text
//! Resource
//!   │
//!   ├── resolve name
//!   ├── resolve type
//!   ├── resolve cardinality
//!   ├── determine resource class
//!   ├── determine ownership/lifetime
//!   ├── determine capabilities
//!   └── produce semantic resource model
//! ```
//!
//! ZUIR:
//!
//! ```text
//! Resource AST
//!     ↓
//! semantic resource model
//!     ↓
//! ZUIR resource semantics
//!     ↓
//! domain-specific IR
//! ```
//!
//! There is intentionally no direct ZUIR dependency here.
//!
//! ## Determinism
//!
//! This representation is deterministic because it contains no:
//!
//! - memory addresses;
//! - timestamps;
//! - random values;
//! - thread-local state;
//! - backend handles;
//! - unordered semantic state.
//!
//! Ordered source constructs remain ordered.
//!
//! ## Serialization
//!
//! Serde derives preserve the complete source-level state of this node.
//!
//! Global AST serialization/versioning remains owned by the AST serialization
//! subsystem. The local schema version below identifies this node contract and
//! is not a replacement for the global AST schema version.
//!
//! ## Security
//!
//! This module:
//!
//! - performs no I/O;
//! - performs no network access;
//! - executes no source code;
//! - uses no raw pointers;
//! - uses no `unsafe`;
//! - does not interpret `NodeId` as an address;
//! - performs checked collection validation;
//! - does not introduce fixed computational limits.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Logical schema version of the native resource AST node.
///
/// This is independent from:
///
/// - Zamani language version;
/// - compiler version;
/// - global AST serialization version;
/// - semantic-model version;
/// - ZUIR version;
/// - backend version.
pub const RESOURCE_AST_SCHEMA_VERSION: u16 = 1;

/// Stable source-level identity of this AST construct.
pub const RESOURCE_AST_KIND_NAME: &str = "zamani:resource";

/// Returns the canonical native node kind for a resource.
#[inline]
#[must_use]
pub const fn resource_node_kind() -> NodeKind {
    NodeKind::Core(CoreNodeKind::Resource)
}

// =============================================================================
// Resource name
// =============================================================================

/// Source-level resource name.
///
/// This is intentionally an opaque source identifier rather than a hardware
/// identifier.
///
/// Examples:
///
/// ```text
/// q
/// qubits
/// memory
/// accelerator
/// workspace
/// photons
/// atoms
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ResourceName(String);

impl ResourceName {
    /// Creates a resource name.
    ///
    /// Empty or whitespace-only names are rejected.
    pub fn new(name: impl Into<String>) -> Result<Self, ResourceError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ResourceError::EmptyName);
        }

        Ok(Self(name))
    }

    /// Returns the source-level name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the wrapper and returns the owned source name.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Returns the number of bytes in the source name.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for ResourceName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Resource declaration mode
// =============================================================================

/// Source-level role of a resource node.
///
/// The distinction is syntactic/source-semantic intent only.
///
/// It does not imply physical allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ResourceMode {
    /// Declares a resource.
    Declaration,

    /// References an already declared resource.
    Reference,
}

impl Default for ResourceMode {
    fn default() -> Self {
        Self::Declaration
    }
}

impl fmt::Display for ResourceMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Declaration => formatter.write_str("declaration"),
            Self::Reference => formatter.write_str("reference"),
        }
    }
}

// =============================================================================
// Resource cardinality
// =============================================================================

/// Source-level cardinality of a resource.
///
/// This representation deliberately does not use a fixed-size machine
/// integer as the only representation of resource cardinality.
///
/// A cardinality can therefore be:
///
/// - omitted;
/// - an arbitrary source expression;
/// - a statically known value after semantic analysis;
/// - a symbolic value;
/// - a runtime-derived value where the language permits it.
///
/// The AST stores the expression's `NodeId` rather than prematurely evaluating
/// it.
///
/// This is critical for POCO-REAF because:
///
/// ```text
/// resource q[N]
/// ```
///
/// must not be turned into a machine-specific value while parsing.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceCardinality {
    /// No explicit cardinality was supplied.
    Unspecified,

    /// Cardinality is represented by another AST expression.
    Expression(NodeId),
}

impl Default for ResourceCardinality {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl ResourceCardinality {
    /// Creates an unspecified cardinality.
    #[must_use]
    pub const fn unspecified() -> Self {
        Self::Unspecified
    }

    /// Creates an expression-backed cardinality.
    #[must_use]
    pub const fn expression(node: NodeId) -> Self {
        Self::Expression(node)
    }

    /// Returns the expression node when one exists.
    #[must_use]
    pub const fn expression_id(&self) -> Option<NodeId> {
        match self {
            Self::Unspecified => None,
            Self::Expression(id) => Some(*id),
        }
    }

    /// Returns whether a cardinality was explicitly supplied.
    #[must_use]
    pub const fn is_specified(&self) -> bool {
        !matches!(self, Self::Unspecified)
    }
}

// =============================================================================
// Resource scope
// =============================================================================

/// Source-level scope of a resource.
///
/// This is intentionally represented using AST identities rather than
/// filesystem, hardware, device, or physical-location identifiers.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceScope {
    /// Resource has the surrounding source scope.
    Implicit,

    /// Resource is scoped to a source-level AST declaration/module.
    Node(NodeId),

    /// Resource uses a source-level symbolic scope expression.
    Expression(NodeId),

    /// Extensible source-level named scope.
    Named(String),
}

impl Default for ResourceScope {
    fn default() -> Self {
        Self::Implicit
    }
}

impl ResourceScope {
    /// Creates an implicit scope.
    #[must_use]
    pub const fn implicit() -> Self {
        Self::Implicit
    }

    /// Creates a node-based scope.
    #[must_use]
    pub const fn node(id: NodeId) -> Self {
        Self::Node(id)
    }

    /// Creates an expression-backed scope.
    #[must_use]
    pub const fn expression(id: NodeId) -> Self {
        Self::Expression(id)
    }

    /// Creates a named semantic scope.
    pub fn named(name: impl Into<String>) -> Result<Self, ResourceError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ResourceError::EmptyScope);
        }

        Ok(Self::Named(name))
    }

    /// Returns whether this is the implicit scope.
    #[must_use]
    pub const fn is_implicit(&self) -> bool {
        matches!(self, Self::Implicit)
    }

    /// Returns a referenced node when applicable.
    #[must_use]
    pub const fn node_id(&self) -> Option<NodeId> {
        match self {
            Self::Node(id) | Self::Expression(id) => Some(*id),
            Self::Implicit | Self::Named(_) => None,
        }
    }

    /// Returns a named scope when applicable.
    #[must_use]
    pub fn named_value(&self) -> Option<&str> {
        match self {
            Self::Named(value) => Some(value.as_str()),
            _ => None,
        }
    }
}

// =============================================================================
// Resource type reference
// =============================================================================

/// Source-level resource type reference.
///
/// The AST deliberately stores a `NodeId` instead of a resolved type.
///
/// This prevents the resource node from depending on the semantic type system
/// and allows generic resource types to evolve independently.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    /// No explicit resource type was supplied.
    Unspecified,

    /// Resource type is represented by another AST node.
    Node(NodeId),
}

impl Default for ResourceType {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl ResourceType {
    /// Creates an unspecified resource type.
    #[must_use]
    pub const fn unspecified() -> Self {
        Self::Unspecified
    }

    /// Creates a resource type reference.
    #[must_use]
    pub const fn node(id: NodeId) -> Self {
        Self::Node(id)
    }

    /// Returns the type node if present.
    #[must_use]
    pub const fn node_id(self) -> Option<NodeId> {
        match self {
            Self::Unspecified => None,
            Self::Node(id) => Some(id),
        }
    }

    /// Returns whether an explicit resource type exists.
    #[must_use]
    pub const fn is_specified(self) -> bool {
        !matches!(self, Self::Unspecified)
    }
}

// =============================================================================
// Resource attributes
// =============================================================================

/// Source-level resource attribute reference.
///
/// Attributes remain ordinary AST nodes. This avoids creating a second
/// attribute representation solely for resources.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceAttributes {
    attributes: Vec<NodeId>,
}

impl ResourceAttributes {
    /// Creates an empty attribute collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            attributes: Vec::new(),
        }
    }

    /// Creates attributes from an existing ordered vector.
    #[must_use]
    pub fn from_vec(attributes: Vec<NodeId>) -> Self {
        Self { attributes }
    }

    /// Appends an attribute reference.
    pub fn push(&mut self, id: NodeId) {
        self.attributes.push(id);
    }

    /// Returns the attributes in source order.
    #[must_use]
    pub fn as_slice(&self) -> &[NodeId] {
        &self.attributes
    }

    /// Returns the number of attributes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    /// Returns whether no attributes exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }

    /// Returns an iterator over attributes.
    pub fn iter(&self) -> impl Iterator<Item = &NodeId> {
        self.attributes.iter()
    }
}

// =============================================================================
// Resource node
// =============================================================================

/// Canonical native Zamani AST resource node.
///
/// This node represents source-level resource intent.
///
/// It does not allocate or identify physical resources.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Resource {
    /// Common source-level node identity and metadata.
    node: Node,

    /// Whether this node declares or references a resource.
    mode: ResourceMode,

    /// Source-level resource name.
    name: ResourceName,

    /// Optional source-level resource type.
    resource_type: ResourceType,

    /// Optional symbolic cardinality.
    cardinality: ResourceCardinality,

    /// Source-level scope.
    scope: ResourceScope,

    /// Optional initializer/value expression.
    initializer: Option<NodeId>,

    /// Source-level attributes.
    attributes: ResourceAttributes,
}

impl Resource {
    /// Creates a resource declaration/reference.
    ///
    /// This constructor performs only local structural checks:
    ///
    /// - the supplied node must have the resource node kind;
    /// - the resource name must already be valid.
    ///
    /// Cross-node semantic checks remain downstream.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        mode: ResourceMode,
        name: ResourceName,
        resource_type: ResourceType,
        cardinality: ResourceCardinality,
        scope: ResourceScope,
        initializer: Option<NodeId>,
        attributes: ResourceAttributes,
    ) -> Result<Self, ResourceError> {
        let node = Node::new(
            id,
            resource_node_kind(),
            span,
            metadata,
        );

        let resource = Self {
            node,
            mode,
            name,
            resource_type,
            cardinality,
            scope,
            initializer,
            attributes,
        };

        resource.validate_local()?;

        Ok(resource)
    }

    /// Creates a resource declaration with no optional components.
    pub fn declaration(
        id: NodeId,
        span: Span,
        name: ResourceName,
    ) -> Result<Self, ResourceError> {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            ResourceMode::Declaration,
            name,
            ResourceType::Unspecified,
            ResourceCardinality::Unspecified,
            ResourceScope::Implicit,
            None,
            ResourceAttributes::new(),
        )
    }

    /// Creates a resource reference with no optional components.
    pub fn reference(
        id: NodeId,
        span: Span,
        name: ResourceName,
    ) -> Result<Self, ResourceError> {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            ResourceMode::Reference,
            name,
            ResourceType::Unspecified,
            ResourceCardinality::Unspecified,
            ResourceScope::Implicit,
            None,
            ResourceAttributes::new(),
        )
    }

    /// Returns the local AST node.
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns the declaration/reference mode.
    #[must_use]
    pub const fn mode(&self) -> ResourceMode {
        self.mode
    }

    /// Returns whether this node declares a resource.
    #[must_use]
    pub const fn is_declaration(&self) -> bool {
        matches!(self.mode, ResourceMode::Declaration)
    }

    /// Returns whether this node references a resource.
    #[must_use]
    pub const fn is_reference(&self) -> bool {
        matches!(self.mode, ResourceMode::Reference)
    }

    /// Returns the source-level resource name.
    #[must_use]
    pub fn name(&self) -> &ResourceName {
        &self.name
    }

    /// Returns the source-level resource type.
    #[must_use]
    pub const fn resource_type(&self) -> ResourceType {
        self.resource_type
    }

    /// Returns the resource type node, when present.
    #[must_use]
    pub const fn resource_type_id(&self) -> Option<NodeId> {
        self.resource_type.node_id()
    }

    /// Returns the symbolic cardinality.
    #[must_use]
    pub const fn cardinality(&self) -> &ResourceCardinality {
        &self.cardinality
    }

    /// Returns the cardinality expression node, when present.
    #[must_use]
    pub const fn cardinality_id(&self) -> Option<NodeId> {
        self.cardinality.expression_id()
    }

    /// Returns the source-level scope.
    #[must_use]
    pub const fn scope(&self) -> &ResourceScope {
        &self.scope
    }

    /// Returns the initializer expression node, when present.
    #[must_use]
    pub const fn initializer_id(&self) -> Option<NodeId> {
        self.initializer
    }

    /// Returns the resource attributes.
    #[must_use]
    pub const fn attributes(&self) -> &ResourceAttributes {
        &self.attributes
    }

    /// Returns mutable access to the attributes.
    #[must_use]
    pub fn attributes_mut(&mut self) -> &mut ResourceAttributes {
        &mut self.attributes
    }

    /// Replaces the initializer.
    ///
    /// The caller remains responsible for ensuring that the referenced node
    /// belongs to the same AST graph.
    pub fn set_initializer(&mut self, initializer: Option<NodeId>) {
        self.initializer = initializer;
    }

    /// Replaces the resource type.
    pub fn set_resource_type(&mut self, resource_type: ResourceType) {
        self.resource_type = resource_type;
    }

    /// Replaces the cardinality.
    pub fn set_cardinality(&mut self, cardinality: ResourceCardinality) {
        self.cardinality = cardinality;
    }

    /// Replaces the scope.
    pub fn set_scope(&mut self, scope: ResourceScope) {
        self.scope = scope;
    }

    /// Performs local structural validation.
    ///
    /// This method does not resolve names or types.
    pub fn validate_local(&self) -> Result<(), ResourceError> {
        if !self.node.is_kind(&resource_node_kind()) {
            return Err(ResourceError::InvalidNodeKind);
        }

        if self.name.as_str().trim().is_empty() {
            return Err(ResourceError::EmptyName);
        }

        if let ResourceScope::Named(name) = &self.scope {
            if name.trim().is_empty() {
                return Err(ResourceError::EmptyScope);
            }
        }

        self.validate_child_ids()?;

        Ok(())
    }

    /// Validates local child-reference invariants.
    ///
    /// A child may not reference this resource node itself.
    ///
    /// Cross-node graph validity belongs to the AST graph validator.
    pub fn validate_child_ids(&self) -> Result<(), ResourceError> {
        let resource_id = self.id();

        if self.resource_type_id() == Some(resource_id) {
            return Err(ResourceError::SelfReference);
        }

        if self.cardinality_id() == Some(resource_id) {
            return Err(ResourceError::SelfReference);
        }

        if self.initializer_id() == Some(resource_id) {
            return Err(ResourceError::SelfReference);
        }

        if self.scope.node_id() == Some(resource_id) {
            return Err(ResourceError::SelfReference);
        }

        for attribute in self.attributes.iter() {
            if *attribute == resource_id {
                return Err(ResourceError::SelfReference);
            }
        }

        Ok(())
    }

    /// Returns the local schema version.
    #[must_use]
    pub const fn schema_version() -> u16 {
        RESOURCE_AST_SCHEMA_VERSION
    }

    /// Returns the stable AST kind name.
    #[must_use]
    pub const fn kind_name() -> &'static str {
        RESOURCE_AST_KIND_NAME
    }

    /// Returns child node IDs in canonical traversal order.
    ///
    /// Traversal order:
    ///
    /// 1. resource type;
    /// 2. cardinality expression;
    /// 3. scope expression/node;
    /// 4. initializer;
    /// 5. attributes in source order.
    ///
    /// The returned vector is intentionally produced on demand. The AST node
    /// does not maintain a second mutable child index that could become
    /// inconsistent with its canonical fields.
    #[must_use]
    pub fn child_ids(&self) -> Vec<NodeId> {
        let mut children = Vec::new();

        if let Some(id) = self.resource_type_id() {
            children.push(id);
        }

        if let Some(id) = self.cardinality_id() {
            children.push(id);
        }

        if let Some(id) = self.scope.node_id() {
            children.push(id);
        }

        if let Some(id) = self.initializer_id() {
            children.push(id);
        }

        children.extend(self.attributes.iter().copied());

        children
    }

    /// Validates that no child reference occurs more than once.
    ///
    /// Duplicate references are rejected only when they occur in the
    /// resource's child-reference sequence. This is a structural sanity check,
    /// not a semantic rule about source reuse elsewhere in the AST.
    pub fn validate_child_uniqueness(&self) -> Result<(), ResourceError> {
        let children = self.child_ids();

        let mut seen = std::collections::HashSet::with_capacity(children.len());

        for child in children {
            if !seen.insert(child) {
                return Err(ResourceError::DuplicateChildReference(child));
            }
        }

        Ok(())
    }

    /// Performs all validation that can be completed without the global AST
    /// graph or semantic environment.
    pub fn validate(&self) -> Result<(), ResourceError> {
        self.validate_local()?;
        self.validate_child_uniqueness()?;
        Ok(())
    }

    /// Returns a compact diagnostic representation.
    ///
    /// This avoids copying potentially large metadata/attribute payloads.
    #[must_use]
    pub fn diagnostic_summary(&self) -> ResourceDiagnosticSummary {
        ResourceDiagnosticSummary {
            id: self.id(),
            mode: self.mode,
            name: self.name.clone(),
            span: self.span_owned(),
        }
    }
}

impl AstNode for Resource {
    fn node(&self) -> &Node {
        &self.node
    }

    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

// =============================================================================
// Diagnostic summary
// =============================================================================

/// Compact diagnostic representation of a resource node.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceDiagnosticSummary {
    /// Stable AST node identity.
    pub id: NodeId,

    /// Declaration/reference mode.
    pub mode: ResourceMode,

    /// Source-level resource name.
    pub name: ResourceName,

    /// Source location.
    pub span: Span,
}

impl fmt::Display for ResourceDiagnosticSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} resource `{}` at {}",
            self.mode,
            self.name,
            self.span
        )
    }
}

// =============================================================================
// Validation policy
// =============================================================================

/// Optional compiler-policy limits for local resource validation.
///
/// These are **not language limits**.
///
/// `None` means that this layer does not impose that particular limit.
///
/// This exists so hostile or pathological source input can be bounded by a
/// compiler/session policy without hard-coding a language-level maximum into
/// the AST node.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ResourceValidationPolicy {
    /// Optional maximum source-name byte length.
    pub max_name_bytes: Option<usize>,

    /// Optional maximum number of resource attributes.
    pub max_attributes: Option<usize>,
}

impl ResourceValidationPolicy {
    /// Creates an unrestricted validation policy.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_name_bytes: None,
            max_attributes: None,
        }
    }

    /// Validates a resource under this policy.
    pub fn validate(&self, resource: &Resource) -> Result<(), ResourceError> {
        resource.validate()?;

        if let Some(max_name_bytes) = self.max_name_bytes {
            if resource.name().byte_len() > max_name_bytes {
                return Err(ResourceError::NameTooLong {
                    actual: resource.name().byte_len(),
                    maximum: max_name_bytes,
                });
            }
        }

        if let Some(max_attributes) = self.max_attributes {
            if resource.attributes().len() > max_attributes {
                return Err(ResourceError::TooManyAttributes {
                    actual: resource.attributes().len(),
                    maximum: max_attributes,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by local resource-node construction or validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourceError {
    /// Resource name is empty.
    EmptyName,

    /// Resource scope name is empty.
    EmptyScope,

    /// Resource node contains an unexpected node kind.
    InvalidNodeKind,

    /// Resource node directly references itself.
    SelfReference,

    /// A child reference occurs more than once in the local child sequence.
    DuplicateChildReference(NodeId),

    /// Resource name exceeds an explicitly configured compiler-policy limit.
    NameTooLong {
        /// Actual byte length.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// Attribute count exceeds an explicitly configured compiler-policy limit.
    TooManyAttributes {
        /// Actual attribute count.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },
}

impl fmt::Display for ResourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str("resource name cannot be empty")
            }

            Self::EmptyScope => {
                formatter.write_str("resource scope cannot be empty")
            }

            Self::InvalidNodeKind => {
                formatter.write_str(
                    "resource node has an invalid AST node kind",
                )
            }

            Self::SelfReference => {
                formatter.write_str(
                    "resource node cannot directly reference itself",
                )
            }

            Self::DuplicateChildReference(id) => {
                write!(
                    formatter,
                    "resource contains duplicate child reference `{id:?}`"
                )
            }

            Self::NameTooLong { actual, maximum } => {
                write!(
                    formatter,
                    "resource name is {actual} bytes but the configured \
                     compiler limit is {maximum} bytes"
                )
            }

            Self::TooManyAttributes { actual, maximum } => {
                write!(
                    formatter,
                    "resource contains {actual} attributes but the \
                     configured compiler limit is {maximum}"
                )
            }
        }
    }
}

impl std::error::Error for ResourceError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_span() -> Span {
        Span::default()
    }

    fn test_node_id(value: u64) -> NodeId {
        NodeId::new(value)
    }

    #[test]
    fn resource_kind_is_canonical() {
        assert_eq!(
            resource_node_kind(),
            NodeKind::Core(CoreNodeKind::Resource)
        );
    }

    #[test]
    fn resource_kind_name_is_stable() {
        assert_eq!(
            Resource::kind_name(),
            "zamani:resource"
        );
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            Resource::schema_version(),
            RESOURCE_AST_SCHEMA_VERSION
        );
    }

    #[test]
    fn resource_name_rejects_empty_values() {
        assert_eq!(
            ResourceName::new(""),
            Err(ResourceError::EmptyName)
        );

        assert_eq!(
            ResourceName::new("   "),
            Err(ResourceError::EmptyName)
        );
    }

    #[test]
    fn resource_name_preserves_source_text() {
        let name =
            ResourceName::new("logical_qubits")
                .expect("name should be valid");

        assert_eq!(name.as_str(), "logical_qubits");
        assert_eq!(name.byte_len(), "logical_qubits".len());
    }

    #[test]
    fn declaration_can_be_created_without_hardware_information() {
        let resource = Resource::declaration(
            test_node_id(1),
            test_span(),
            ResourceName::new("q")
                .expect("resource name should be valid"),
        )
        .expect("resource should be valid");

        assert!(resource.is_declaration());
        assert!(!resource.is_reference());
        assert_eq!(resource.name().as_str(), "q");
        assert!(resource.validate().is_ok());
    }

    #[test]
    fn reference_can_be_created() {
        let resource = Resource::reference(
            test_node_id(2),
            test_span(),
            ResourceName::new("q")
                .expect("resource name should be valid"),
        )
        .expect("resource should be valid");

        assert!(resource.is_reference());
        assert!(!resource.is_declaration());
    }

    #[test]
    fn cardinality_preserves_symbolic_expression() {
        let expression_id = test_node_id(10);

        let cardinality =
            ResourceCardinality::expression(expression_id);

        assert_eq!(
            cardinality.expression_id(),
            Some(expression_id)
        );
        assert!(cardinality.is_specified());
    }

    #[test]
    fn unspecified_cardinality_has_no_machine_size_assumption() {
        let cardinality =
            ResourceCardinality::unspecified();

        assert!(!cardinality.is_specified());
        assert_eq!(
            cardinality.expression_id(),
            None
        );
    }

    #[test]
    fn resource_type_is_ast_based() {
        let type_id = test_node_id(20);
        let resource_type = ResourceType::node(type_id);

        assert!(resource_type.is_specified());
        assert_eq!(
            resource_type.node_id(),
            Some(type_id)
        );
    }

    #[test]
    fn named_scope_rejects_empty_values() {
        assert_eq!(
            ResourceScope::named(""),
            Err(ResourceError::EmptyScope)
        );

        assert_eq!(
            ResourceScope::named("   "),
            Err(ResourceError::EmptyScope)
        );
    }

    #[test]
    fn named_scope_is_source_level() {
        let scope =
            ResourceScope::named("module_scope")
                .expect("scope should be valid");

        assert_eq!(
            scope.named_value(),
            Some("module_scope")
        );
    }

    #[test]
    fn resource_child_ids_are_deterministic() {
        let type_id = test_node_id(10);
        let cardinality_id = test_node_id(11);
        let scope_id = test_node_id(12);
        let initializer_id = test_node_id(13);
        let attribute_id = test_node_id(14);

        let mut attributes = ResourceAttributes::new();
        attributes.push(attribute_id);

        let resource = Resource::new(
            test_node_id(1),
            test_span(),
            NodeMetadata::default(),
            ResourceMode::Declaration,
            ResourceName::new("q")
                .expect("name should be valid"),
            ResourceType::node(type_id),
            ResourceCardinality::expression(cardinality_id),
            ResourceScope::node(scope_id),
            Some(initializer_id),
            attributes,
        )
        .expect("resource should be valid");

        assert_eq!(
            resource.child_ids(),
            vec![
                type_id,
                cardinality_id,
                scope_id,
                initializer_id,
                attribute_id,
            ]
        );
    }

    #[test]
    fn self_reference_is_rejected() {
        let id = test_node_id(100);

        let result = Resource::new(
            id,
            test_span(),
            NodeMetadata::default(),
            ResourceMode::Declaration,
            ResourceName::new("q")
                .expect("name should be valid"),
            ResourceType::node(id),
            ResourceCardinality::Unspecified,
            ResourceScope::Implicit,
            None,
            ResourceAttributes::new(),
        );

        assert_eq!(
            result,
            Err(ResourceError::SelfReference)
        );
    }

    #[test]
    fn duplicate_child_references_are_rejected() {
        let id = test_node_id(1);
        let child = test_node_id(2);

        let mut attributes = ResourceAttributes::new();
        attributes.push(child);

        let resource = Resource::new(
            id,
            test_span(),
            NodeMetadata::default(),
            ResourceMode::Declaration,
            ResourceName::new("q")
                .expect("name should be valid"),
            ResourceType::node(child),
            ResourceCardinality::Unspecified,
            ResourceScope::Implicit,
            None,
            attributes,
        )
        .expect("resource construction should succeed");

        assert_eq!(
            resource.validate_child_uniqueness(),
            Err(ResourceError::DuplicateChildReference(child))
        );
    }

    #[test]
    fn attributes_preserve_source_order() {
        let mut attributes = ResourceAttributes::new();

        let first = test_node_id(1);
        let second = test_node_id(2);
        let third = test_node_id(3);

        attributes.push(first);
        attributes.push(second);
        attributes.push(third);

        assert_eq!(
            attributes.as_slice(),
            &[first, second, third]
        );
    }

    #[test]
    fn validation_policy_can_be_unrestricted() {
        let resource = Resource::declaration(
            test_node_id(1),
            test_span(),
            ResourceName::new("arbitrarily_named_resource")
                .expect("name should be valid"),
        )
        .expect("resource should be valid");

        let policy =
            ResourceValidationPolicy::unrestricted();

        assert!(policy.validate(&resource).is_ok());
    }

    #[test]
    fn validation_policy_is_not_a_language_limit() {
        let resource = Resource::declaration(
            test_node_id(1),
            test_span(),
            ResourceName::new("resource")
                .expect("name should be valid"),
        )
        .expect("resource should be valid");

        let policy = ResourceValidationPolicy {
            max_name_bytes: Some(4),
            max_attributes: None,
        };

        assert_eq!(
            policy.validate(&resource),
            Err(ResourceError::NameTooLong {
                actual: 8,
                maximum: 4,
            })
        );
    }

    #[test]
    fn resource_implements_ast_node_contract() {
        let resource = Resource::declaration(
            test_node_id(1),
            test_span(),
            ResourceName::new("q")
                .expect("name should be valid"),
        )
        .expect("resource should be valid");

        assert_eq!(
            resource.kind(),
            &NodeKind::Core(CoreNodeKind::Resource)
        );

        assert_eq!(
            resource.id(),
            test_node_id(1)
        );
    }

    #[test]
    fn resource_is_backend_independent() {
        let resource = Resource::declaration(
            test_node_id(1),
            test_span(),
            ResourceName::new("q")
                .expect("name should be valid"),
        )
        .expect("resource should be valid");

        let serialized =
            serde_json::to_string(&resource)
                .expect("resource should serialize");

        assert!(serialized.contains("q"));
        assert!(!serialized.contains("IBM"));
        assert!(!serialized.contains("IonQ"));
        assert!(!serialized.contains("QIR"));
        assert!(!serialized.contains("LLVM"));
    }

    #[test]
    fn large_symbolic_resource_has_no_fixed_machine_limit() {
        let expression_id = test_node_id(u64::MAX - 1);

        let resource = Resource::new(
            test_node_id(u64::MAX),
            test_span(),
            NodeMetadata::default(),
            ResourceMode::Declaration,
            ResourceName::new("q")
                .expect("name should be valid"),
            ResourceType::Unspecified,
            ResourceCardinality::expression(expression_id),
            ResourceScope::Implicit,
            None,
            ResourceAttributes::new(),
        )
        .expect("resource should remain representable");

        assert_eq!(
            resource.cardinality_id(),
            Some(expression_id)
        );
    }

    #[test]
    fn diagnostic_summary_excludes_large_metadata() {
        let resource = Resource::declaration(
            test_node_id(1),
            test_span(),
            ResourceName::new("q")
                .expect("resource name should be valid"),
        )
        .expect("resource should be valid");

        let summary = resource.diagnostic_summary();

        assert_eq!(summary.id, test_node_id(1));
        assert_eq!(summary.name.as_str(), "q");
        assert_eq!(
            summary.mode,
            ResourceMode::Declaration
        );
    }
}