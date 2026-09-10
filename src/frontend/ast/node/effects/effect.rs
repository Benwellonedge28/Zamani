//! # Zamani Native AST — Effect Declaration
//!
//! Canonical source-level representation of a Zamani algebraic-effect
//! declaration.
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
//! Effect
//!     │
//!     ├── Node
//!     ├── name
//!     ├── generic parameter NodeIds
//!     ├── parameter NodeIds
//!     └── optional return-type NodeId
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ├── name resolution
//!     ├── generic resolution
//!     ├── parameter/type checking
//!     ├── effect identity
//!     ├── effect-row/handler semantics
//!     └── capability/resource analysis
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! domain / target lowering
//! ```
//!
//! ## Responsibility
//!
//! This file owns the canonical **source-level declaration** of one effect.
//!
//! An effect declaration describes a programmer-defined computational effect.
//! It does not describe how that effect is implemented.
//!
//! The representation is deliberately independent of:
//!
//! - CPU architecture;
//! - GPU architecture;
//! - FPGA architecture;
//! - ASIC architecture;
//! - QPU architecture;
//! - physical qubits;
//! - logical qubits;
//! - quantum topology;
//! - quantum gates;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - QEC;
//! - resilience implementation;
//! - noise models;
//! - ZQN;
//! - runtime dispatch;
//! - backend queues;
//! - vendor SDKs;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - hardware instruction sets.
//!
//! Those concerns belong to later compiler/runtime layers.
//!
//! ## POCO-REAF
//!
//! An effect is source-level intent.
//!
//! It must remain valid regardless of whether the eventual computation is
//! executed on:
//!
//! - a tiny machine;
//! - a large machine;
//! - a distributed system;
//! - a simulator;
//! - classical hardware;
//! - quantum hardware;
//! - heterogeneous hardware;
//! - a future computational substrate.
//!
//! This file therefore contains no:
//!
//! ```text
//! MAX_EFFECTS
//! MAX_EFFECT_PARAMETERS
//! MAX_EFFECT_GENERICS
//! MAX_MACHINE_SIZE
//! MAX_QUBITS
//! ```
//!
//! Such limits, when necessary for compiler resource protection, belong to
//! explicit compiler policies.
//!
//! "Infinity" in the POCO-REAF architecture means that this representation
//! introduces no artificial machine-size or effect-cardinality limit beyond
//! the representable Rust data model and resources actually available to the
//! compiler.
//!
//! ## Grammar integration
//!
//! The current Zamani grammar defines:
//!
//! ```text
//! effectDecl:
//!     'effect'
//!     IDENTIFIER
//!     genericParameters?
//!     '(' parameterList? ')'
//!     ('->' typeExpr)?
//!     ';'
//! ```
//!
//! Therefore this node represents:
//!
//! ```text
//! Effect
//! ├── name
//! ├── generic parameters
//! ├── parameters
//! └── optional return type
//! ```
//!
//! Child AST nodes are referenced by `NodeId` rather than embedded directly.
//!
//! This keeps this file independent of the concrete implementation of:
//!
//! - generic parameters;
//! - parameters;
//! - type expressions;
//! - declarations;
//! - future AST extensions.
//!
//! ## Effect declaration vs effect use
//!
//! This type represents an **effect declaration**.
//!
//! It must not be confused with an effect invocation/use.
//!
//! Conceptually:
//!
//! ```text
//! effect Read<T>(...) -> ...;
//!          │
//!          └── Effect
//!
//! perform Read(...)
//!          │
//!          └── future effect-use / perform AST node
//! ```
//!
//! A future `perform` node should reference the declaration through normal
//! source-level name/path semantics and must not duplicate the declaration
//! representation.
//!
//! ## Effect semantics
//!
//! This node deliberately does not decide whether an effect is:
//!
//! - pure;
//! - impure;
//! - I/O-related;
//! - quantum-related;
//! - resource-related;
//! - asynchronous;
//! - nondeterministic;
//! - distributed;
//! - security-sensitive;
//! - resumable;
//! - non-resumable;
//! - hardware-backed.
//!
//! Those are semantic properties.
//!
//! The AST preserves the declaration's source structure; semantic analysis
//! determines its meaning.
//!
//! ## Quantum integration
//!
//! Effects may describe quantum-related programmer intent without making the
//! AST quantum-specific.
//!
//! For example:
//!
//! ```text
//! effect QuantumDecoherence;
//! effect Measure<Q>(Q) -> Result;
//! ```
//!
//! The AST does not know whether an effect is ultimately implemented using:
//!
//! - a simulator;
//! - superconducting hardware;
//! - trapped ions;
//! - neutral atoms;
//! - photonics;
//! - a future quantum technology.
//!
//! Quantum implementation remains downstream.
//!
//! In particular, this file must never contain a closed enum such as:
//!
//! ```text
//! enum QuantumEffect {
//!     Decoherence,
//!     Measurement,
//!     Reset,
//!     ...
//! }
//! ```
//!
//! Such a design would prevent extensibility and violate the domain-neutral
//! architecture.
//!
//! ## Capability boundary
//!
//! An effect declaration is not itself a hardware capability.
//!
//! Semantic analysis may later determine that an effect requires particular
//! capabilities, but this node does not contain backend identifiers.
//!
//! Therefore a new backend or computational technology does not require this
//! file to change.
//!
//! ## Resource boundary
//!
//! An effect parameter may eventually have a semantic type corresponding to:
//!
//! - a value;
//! - memory;
//! - a resource;
//! - a quantum resource;
//! - a distributed resource;
//! - another future computational resource.
//!
//! This file only stores the source-level child-node relationships.
//!
//! Resource ownership, lifetime, allocation and availability belong downstream.
//!
//! ## Dependency contract
//!
//! This module may depend only on:
//!
//! - `Node`;
//! - `AstNode`;
//! - `NodeId`;
//! - `NodeKind`;
//! - `CoreNodeKind`;
//! - `NodeMetadata`;
//! - `Span`;
//! - Serde;
//! - Rust standard-library facilities.
//!
//! It must never depend on:
//!
//! - parser implementation;
//! - lexer implementation;
//! - semantic analysis;
//! - compiler driver;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - quantum routing;
//! - quantum scheduling;
//! - quantum optimization;
//! - QEC;
//! - resilience;
//! - calibration;
//! - runtime;
//! - backend providers;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - vendor SDKs;
//! - filesystem APIs;
//! - network APIs.
//!
//! ## Child ownership
//!
//! This node does not own the child AST nodes themselves.
//!
//! It owns only their stable `NodeId` references.
//!
//! The canonical AST store owns the actual child nodes.
//!
//! This is important because it means a future change to the concrete
//! representation of a parameter or generic parameter does not require this
//! file to change.
//!
//! ## Structural validation
//!
//! Validation performed here is deliberately local.
//!
//! This file validates:
//!
//! - correct node kind;
//! - non-empty effect name;
//! - optional configured name-size policy;
//! - configurable collection limits;
//! - duplicate child references within each declaration category;
//! - the effect node not referencing itself;
//! - return-type presence only through its structural `NodeId`.
//!
//! It does **not** validate:
//!
//! - whether the name resolves;
//! - whether generic parameters exist;
//! - whether parameters have valid types;
//! - whether the return type is semantically valid;
//! - whether generic parameters conflict;
//! - whether parameter names are unique;
//! - whether the effect is legal in a particular domain;
//! - whether an effect can execute on hardware;
//! - whether an effect requires quantum resources.
//!
//! Those checks belong to semantic analysis.
//!
//! ## Duplicate references
//!
//! Duplicate child `NodeId`s are rejected within each homogeneous declaration
//! collection because they usually indicate malformed AST construction.
//!
//! This check does not impose a language-level cardinality limit.
//!
//! Cross-category reuse is not rejected here because the same semantic node may
//! be referenced from more than one source construct depending on the AST
//! store's ownership model.
//!
//! ## Determinism
//!
//! The representation is deterministic because:
//!
//! - declaration order is preserved;
//! - generic parameter order is preserved;
//! - parameter order is preserved;
//! - there is no unordered collection;
//! - there is no randomness;
//! - there is no timestamp;
//! - there is no memory address;
//! - there is no backend state.
//!
//! ## Serialization
//!
//! Serde serialization preserves all source-level state:
//!
//! - node identity;
//! - node kind;
//! - source span;
//! - metadata;
//! - effect name;
//! - generic parameter references;
//! - parameter references;
//! - optional return-type reference.
//!
//! Global AST schema versioning remains owned by the AST serialization layer.
//!
//! This file exposes a local schema version solely to allow tooling and tests
//! to identify the logical contract of this node.
//!
//! ## Traversal
//!
//! The canonical child order is:
//!
//! ```text
//! generic_parameters
//! parameters
//! return_type
//! ```
//!
//! Each collection retains source order.
//!
//! The central traversal subsystem should use this ordering.
//!
//! This file intentionally does not implement a second traversal framework.
//!
//! ## Semantic integration
//!
//! Semantic analysis consumes:
//!
//! ```text
//! Effect
//!   │
//!   ├── name
//!   ├── generic_parameters
//!   ├── parameters
//!   └── return_type
//! ```
//!
//! and resolves:
//!
//! - effect identity;
//! - scope;
//! - generic bindings;
//! - parameter types;
//! - return type;
//! - effect algebra;
//! - handler compatibility;
//! - effect composition;
//! - capabilities;
//! - resource requirements;
//! - domain semantics.
//!
//! Semantic side tables should associate resolved information with the
//! `NodeId` rather than mutating this source AST node with backend information.
//!
//! ## ZUIR integration
//!
//! This file has no direct ZUIR dependency.
//!
//! The required lowering boundary is:
//!
//! ```text
//! Effect AST
//!     │
//!     ▼
//! semantic effect model
//!     │
//!     ▼
//! ZUIR effect semantics
//!     │
//!     ▼
//! domain / target realization
//! ```
//!
//! The AST must never become a disguised ZUIR node.
//!
//! ## Function integration
//!
//! Existing function declarations represent effect requirements as:
//!
//! ```text
//! Vec<NodeId>
//! ```
//!
//! This node is therefore suitable as the canonical target of those references.
//!
//! The function does not need to know the internal representation of `Effect`.
//!
//! Conceptually:
//!
//! ```text
//! Function.effects()
//!       │
//!       ▼
//! NodeId
//!       │
//!       ▼
//! canonical AST store
//!       │
//!       ▼
//! Effect
//! ```
//!
//! This preserves the dependency direction:
//!
//! ```text
//! Effect
//!   ▲
//!   │
//! Function
//! ```
//!
//! rather than creating a dependency from `Effect` back to `Function`.
//!
//! ## Parser integration contract
//!
//! The parser should:
//!
//! 1. allocate a `NodeId` for the effect;
//! 2. parse the identifier;
//! 3. parse optional generic parameters;
//! 4. parse parameters;
//! 5. parse the optional return type;
//! 6. preserve source order;
//! 7. construct this node;
//! 8. insert the node into the canonical AST store;
//! 9. attach the resulting `NodeId` to the containing program/module.
//!
//! The parser must not perform:
//!
//! - name resolution;
//! - type inference;
//! - capability resolution;
//! - resource allocation;
//! - backend selection;
//! - quantum mapping;
//! - scheduling.
//!
//! ## Compatibility with current grammar
//!
//! The current grammar explicitly permits generic parameters and parameters
//! on effect declarations. 
//!
//! Therefore this representation deliberately does not reduce an effect to
//! only a string name.
//!
//! A name-only representation would lose source-level declaration structure.
//!
//! ## Scalable child representation
//!
//! `Vec<NodeId>` is used for ordered child references.
//!
//! There is no fixed-size array.
//!
//! The representation therefore scales with available compiler memory rather
//! than an artificial language constant.
//!
//! A compiler may later impose explicit resource-policy limits before or during
//! parsing/validation.
//!
//! Such limits must not be encoded in this node.
//!
//! ## Thread safety
//!
//! This type has no global mutable state and contains only owned values and
//! stable identifiers.
//!
//! Read-only instances can therefore participate in parallel compiler phases
//! where the containing AST store provides the required `Send`/`Sync` guarantees.
//!
//! ## Security
//!
//! This implementation:
//!
//! - performs no I/O;
//! - performs no network access;
//! - executes no source code;
//! - uses no raw pointers;
//! - uses no `unsafe`;
//! - does not perform unchecked indexing;
//! - does not trust external `NodeId`s as memory addresses.
//!
//! Malformed structures are reported through validation rather than panics.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
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
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Logical schema version of the source-level `Effect` AST node.
///
/// This is intentionally independent of:
///
/// - Zamani language version;
/// - compiler version;
/// - global serialized AST version;
/// - semantic-model version;
/// - ZUIR version;
/// - target/backend version.
pub const EFFECT_AST_SCHEMA_VERSION: u16 = 1;

/// Stable source-level identity for this AST construct.
pub const EFFECT_AST_KIND_NAME: &str = "zamani:effect";

/// Creates the canonical native AST node kind for an effect declaration.
#[inline]
#[must_use]
pub const fn effect_node_kind() -> NodeKind {
    NodeKind::Core(CoreNodeKind::Effect)
}

/// Compiler resource policy for structural effect validation.
///
/// `None` means that this local validation layer imposes no limit.
///
/// These are deliberately compiler-policy controls rather than language
/// semantics. They therefore do not limit the source language's conceptual
/// scalability.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EffectValidationPolicy {
    /// Optional maximum byte length of the effect's source name.
    pub max_name_bytes: Option<usize>,

    /// Optional maximum number of generic-parameter references.
    pub max_generic_parameters: Option<usize>,

    /// Optional maximum number of parameter references.
    pub max_parameters: Option<usize>,
}

impl EffectValidationPolicy {
    /// Creates an unrestricted validation policy.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_name_bytes: None,
            max_generic_parameters: None,
            max_parameters: None,
        }
    }
}

/// Structural errors produced by [`Effect::validate_structure_with_policy`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum EffectValidationError {
    /// The effect's source name is empty.
    EmptyName,

    /// The effect name exceeds a configured compiler-policy limit.
    NameTooLarge {
        /// Actual UTF-8 byte length.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The generic-parameter collection exceeds a configured policy limit.
    TooManyGenericParameters {
        /// Actual number of references.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The parameter collection exceeds a configured policy limit.
    TooManyParameters {
        /// Actual number of references.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The common node has the wrong AST kind.
    WrongNodeKind {
        /// Expected native node kind.
        expected: NodeKind,

        /// Actual node kind.
        actual: NodeKind,
    },

    /// The effect references itself as a generic parameter.
    SelfReferenceAsGenericParameter,

    /// The effect references itself as a parameter.
    SelfReferenceAsParameter,

    /// The effect references itself as its return type.
    SelfReferenceAsReturnType,

    /// A generic-parameter reference appears more than once.
    DuplicateGenericParameter {
        /// Duplicated child node identity.
        node_id: NodeId,
    },

    /// A parameter reference appears more than once.
    DuplicateParameter {
        /// Duplicated child node identity.
        node_id: NodeId,
    },
}

impl fmt::Display for EffectValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str("effect name cannot be empty")
            }

            Self::NameTooLarge { actual, maximum } => {
                write!(
                    formatter,
                    "effect name exceeds configured limit: {actual} > {maximum}"
                )
            }

            Self::TooManyGenericParameters { actual, maximum } => {
                write!(
                    formatter,
                    "effect generic-parameter count exceeds configured limit: \
                     {actual} > {maximum}"
                )
            }

            Self::TooManyParameters { actual, maximum } => {
                write!(
                    formatter,
                    "effect parameter count exceeds configured limit: \
                     {actual} > {maximum}"
                )
            }

            Self::WrongNodeKind { expected, actual } => {
                write!(
                    formatter,
                    "effect has wrong AST node kind: expected {expected:?}, \
                     found {actual:?}"
                )
            }

            Self::SelfReferenceAsGenericParameter => {
                formatter.write_str(
                    "effect cannot reference itself as a generic parameter node",
                )
            }

            Self::SelfReferenceAsParameter => {
                formatter.write_str(
                    "effect cannot reference itself as a parameter node",
                )
            }

            Self::SelfReferenceAsReturnType => {
                formatter.write_str(
                    "effect cannot reference itself as its return-type node",
                )
            }

            Self::DuplicateGenericParameter { node_id } => {
                write!(
                    formatter,
                    "effect contains duplicate generic-parameter node: {node_id:?}"
                )
            }

            Self::DuplicateParameter { node_id } => {
                write!(
                    formatter,
                    "effect contains duplicate parameter node: {node_id:?}"
                )
            }
        }
    }
}

impl std::error::Error for EffectValidationError {}

/// Canonical source-level Zamani effect declaration.
///
/// An `Effect` contains source structure only. Child AST nodes are owned by
/// the canonical AST store and referenced here through `NodeId`.
///
/// # Source shape
///
/// ```text
/// effect Name<T>(parameter...) -> ReturnType;
/// ```
///
/// maps structurally to:
///
/// ```text
/// Effect {
///     node,
///     name,
///     generic_parameters,
///     parameters,
///     return_type,
/// }
/// ```
///
/// The actual semantic meaning of the effect is resolved later.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Effect {
    /// Common native AST node data.
    node: Node,

    /// Original source spelling of the effect name.
    ///
    /// This is deliberately not a resolved symbol.
    name: String,

    /// Generic-parameter AST node references in source order.
    generic_parameters: Vec<NodeId>,

    /// Parameter AST node references in source order.
    parameters: Vec<NodeId>,

    /// Optional return-type AST node reference.
    return_type: Option<NodeId>,
}

impl Effect {
    /// Constructs an effect declaration from an existing common AST node.
    ///
    /// The supplied `Node` is preserved exactly. Structural validation reports
    /// a wrong node kind rather than silently repairing malformed input.
    #[must_use]
    pub fn from_node(
        node: Node,
        name: impl Into<String>,
        generic_parameters: Vec<NodeId>,
        parameters: Vec<NodeId>,
        return_type: Option<NodeId>,
    ) -> Self {
        Self {
            node,
            name: name.into(),
            generic_parameters,
            parameters,
            return_type,
        }
    }

    /// Constructs a canonical effect declaration.
    ///
    /// The node is assigned `CoreNodeKind::Effect`.
    ///
    /// This constructor does not perform semantic validation.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        name: impl Into<String>,
        generic_parameters: Vec<NodeId>,
        parameters: Vec<NodeId>,
        return_type: Option<NodeId>,
    ) -> Self {
        let node = Node::new(
            id,
            effect_node_kind(),
            span,
            metadata,
        );

        Self::from_node(
            node,
            name,
            generic_parameters,
            parameters,
            return_type,
        )
    }

    /// Constructs an effect with empty metadata.
    #[must_use]
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        name: impl Into<String>,
        generic_parameters: Vec<NodeId>,
        parameters: Vec<NodeId>,
        return_type: Option<NodeId>,
    ) -> Self {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            name,
            generic_parameters,
            parameters,
            return_type,
        )
    }

    /// Returns the common AST node.
    #[inline]
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns the mutable common AST node.
    #[inline]
    #[must_use]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns this effect's stable AST identity.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id
    }

    /// Returns this effect's source span.
    #[inline]
    #[must_use]
    pub fn span(&self) -> Span {
        self.node.span
    }

    /// Returns the effect's source name.
    ///
    /// This does not perform name resolution.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the effect name's UTF-8 byte length.
    #[inline]
    #[must_use]
    pub fn name_byte_len(&self) -> usize {
        self.name.len()
    }

    /// Returns whether the source name is empty.
    #[inline]
    #[must_use]
    pub fn is_name_empty(&self) -> bool {
        self.name.is_empty()
    }

    /// Returns the generic-parameter references in source order.
    #[inline]
    #[must_use]
    pub fn generic_parameters(&self) -> &[NodeId] {
        &self.generic_parameters
    }

    /// Returns the number of generic-parameter references.
    #[inline]
    #[must_use]
    pub fn generic_parameter_count(&self) -> usize {
        self.generic_parameters.len()
    }

    /// Returns whether this effect has generic parameters.
    #[inline]
    #[must_use]
    pub fn has_generic_parameters(&self) -> bool {
        !self.generic_parameters.is_empty()
    }

    /// Returns the generic parameter at `index`, if present.
    #[inline]
    #[must_use]
    pub fn generic_parameter(&self, index: usize) -> Option<NodeId> {
        self.generic_parameters.get(index).copied()
    }

    /// Returns the ordinary parameter references in source order.
    #[inline]
    #[must_use]
    pub fn parameters(&self) -> &[NodeId] {
        &self.parameters
    }

    /// Returns the number of parameter references.
    #[inline]
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Returns whether this effect has parameters.
    #[inline]
    #[must_use]
    pub fn has_parameters(&self) -> bool {
        !self.parameters.is_empty()
    }

    /// Returns the parameter at `index`, if present.
    #[inline]
    #[must_use]
    pub fn parameter(&self, index: usize) -> Option<NodeId> {
        self.parameters.get(index).copied()
    }

    /// Returns the optional return-type node reference.
    #[inline]
    #[must_use]
    pub fn return_type(&self) -> Option<NodeId> {
        self.return_type
    }

    /// Returns whether an explicit return type was declared.
    #[inline]
    #[must_use]
    pub const fn has_return_type(&self) -> bool {
        self.return_type.is_some()
    }

    /// Returns the number of direct child-node references.
    ///
    /// The order is:
    ///
    /// ```text
    /// generic parameters
    /// parameters
    /// optional return type
    /// ```
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.generic_parameters
            .len()
            .saturating_add(self.parameters.len())
            .saturating_add(usize::from(self.return_type.is_some()))
    }

    /// Returns all direct child-node references in canonical traversal order.
    ///
    /// This allocates because the optional return type must be appended after
    /// the two independently stored ordered collections.
    ///
    /// Central AST traversal should preferably use
    /// [`Self::for_each_child_node_id`] to remain allocation-free.
    #[must_use]
    pub fn child_node_ids(&self) -> Vec<NodeId> {
        let mut children = Vec::with_capacity(self.child_count());

        children.extend_from_slice(&self.generic_parameters);
        children.extend_from_slice(&self.parameters);

        if let Some(return_type) = self.return_type {
            children.push(return_type);
        }

        children
    }

    /// Visits every direct child `NodeId` in canonical source/declaration
    /// order without allocating a temporary vector.
    ///
    /// This is the preferred primitive for large-AST traversal.
    pub fn for_each_child_node_id<F>(&self, mut visit: F)
    where
        F: FnMut(NodeId),
    {
        for &node_id in &self.generic_parameters {
            visit(node_id);
        }

        for &node_id in &self.parameters {
            visit(node_id);
        }

        if let Some(node_id) = self.return_type {
            visit(node_id);
        }
    }

    /// Appends one generic-parameter reference.
    ///
    /// The AST does not resolve the referenced node.
    pub fn push_generic_parameter(&mut self, node_id: NodeId) {
        self.generic_parameters.push(node_id);
    }

    /// Appends generic-parameter references while preserving iterator order.
    pub fn extend_generic_parameters<I>(&mut self, node_ids: I)
    where
        I: IntoIterator<Item = NodeId>,
    {
        self.generic_parameters.extend(node_ids);
    }

    /// Appends one parameter reference.
    pub fn push_parameter(&mut self, node_id: NodeId) {
        self.parameters.push(node_id);
    }

    /// Appends parameter references while preserving iterator order.
    pub fn extend_parameters<I>(&mut self, node_ids: I)
    where
        I: IntoIterator<Item = NodeId>,
    {
        self.parameters.extend(node_ids);
    }

    /// Sets the optional return-type reference.
    ///
    /// Semantic interpretation of that node belongs to semantic analysis.
    pub fn set_return_type(&mut self, return_type: Option<NodeId>) {
        self.return_type = return_type;
    }

    /// Removes all generic-parameter references.
    pub fn clear_generic_parameters(&mut self) {
        self.generic_parameters.clear();
    }

    /// Removes all parameter references.
    pub fn clear_parameters(&mut self) {
        self.parameters.clear();
    }

    /// Removes the explicit return-type reference.
    pub fn clear_return_type(&mut self) {
        self.return_type = None;
    }

    /// Returns the stable AST construct name.
    #[inline]
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        EFFECT_AST_KIND_NAME
    }

    /// Returns this node contract's schema version.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        EFFECT_AST_SCHEMA_VERSION
    }

    /// Validates local structural invariants without imposing compiler
    /// resource limits.
    pub fn validate_structure(
        &self,
    ) -> Result<(), EffectValidationError> {
        self.validate_structure_with_policy(
            &EffectValidationPolicy::unrestricted(),
        )
    }

    /// Validates local structural invariants using an explicit compiler
    /// resource policy.
    ///
    /// This function deliberately does not resolve any child `NodeId`.
    ///
    /// Resolution requires the canonical AST store and belongs to the
    /// structural-validation/semantic-analysis orchestration layer.
    pub fn validate_structure_with_policy(
        &self,
        policy: &EffectValidationPolicy,
    ) -> Result<(), EffectValidationError> {
        let actual_kind = self.node.kind;

        if actual_kind != effect_node_kind() {
            return Err(EffectValidationError::WrongNodeKind {
                expected: effect_node_kind(),
                actual: actual_kind,
            });
        }

        if self.name.is_empty() {
            return Err(EffectValidationError::EmptyName);
        }

        if let Some(maximum) = policy.max_name_bytes {
            if self.name.len() > maximum {
                return Err(EffectValidationError::NameTooLarge {
                    actual: self.name.len(),
                    maximum,
                });
            }
        }

        if let Some(maximum) = policy.max_generic_parameters {
            if self.generic_parameters.len() > maximum {
                return Err(
                    EffectValidationError::TooManyGenericParameters {
                        actual: self.generic_parameters.len(),
                        maximum,
                    },
                );
            }
        }

        if let Some(maximum) = policy.max_parameters {
            if self.parameters.len() > maximum {
                return Err(EffectValidationError::TooManyParameters {
                    actual: self.parameters.len(),
                    maximum,
                });
            }
        }

        let self_id = self.node.id;

        if self
            .generic_parameters
            .iter()
            .any(|&node_id| node_id == self_id)
        {
            return Err(
                EffectValidationError::SelfReferenceAsGenericParameter,
            );
        }

        if self
            .parameters
            .iter()
            .any(|&node_id| node_id == self_id)
        {
            return Err(EffectValidationError::SelfReferenceAsParameter);
        }

        if self.return_type == Some(self_id) {
            return Err(
                EffectValidationError::SelfReferenceAsReturnType,
            );
        }

        validate_unique_node_ids(
            &self.generic_parameters,
            |node_id| EffectValidationError::DuplicateGenericParameter {
                node_id,
            },
        )?;

        validate_unique_node_ids(
            &self.parameters,
            |node_id| EffectValidationError::DuplicateParameter {
                node_id,
            },
        )?;

        Ok(())
    }

    /// Returns a deterministic source-level declaration header.
    ///
    /// This method does not dereference child nodes because the concrete
    /// source spelling of those nodes belongs to the canonical AST store.
    ///
    /// Consequently, the result is intentionally structural:
    ///
    /// ```text
    /// effect Name
    /// effect Name<...>
    /// ```
    ///
    /// Use the AST printer/pretty-printer with the canonical store when a
    /// complete source reconstruction is required.
    #[must_use]
    pub fn declaration_prefix(&self) -> String {
        let mut output =
            String::with_capacity("effect ".len() + self.name.len());

        output.push_str("effect ");
        output.push_str(&self.name);

        if !self.generic_parameters.is_empty() {
            output.push('<');

            for (index, node_id) in
                self.generic_parameters.iter().enumerate()
            {
                if index != 0 {
                    output.push_str(", ");
                }

                output.push_str(&format!("{node_id:?}"));
            }

            output.push('>');
        }

        output
    }

    /// Returns the optional return-type node identity without resolving it.
    ///
    /// This explicit method exists to make semantic/lowering code clear about
    /// the fact that it is working with an unresolved AST reference.
    #[inline]
    #[must_use]
    pub const fn unresolved_return_type(&self) -> Option<NodeId> {
        self.return_type
    }

    /// Consumes the effect and returns all source-level components.
    ///
    /// This avoids cloning potentially large child-reference collections.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        Node,
        String,
        Vec<NodeId>,
        Vec<NodeId>,
        Option<NodeId>,
    ) {
        (
            self.node,
            self.name,
            self.generic_parameters,
            self.parameters,
            self.return_type,
        )
    }
}

/// Validates that an ordered `NodeId` collection contains no duplicate IDs.
///
/// The temporary set is local to validation and is never serialized.
///
/// This function has no language-level cardinality limit.
fn validate_unique_node_ids<F>(
    node_ids: &[NodeId],
    mut duplicate_error: F,
) -> Result<(), EffectValidationError>
where
    F: FnMut(NodeId) -> EffectValidationError,
{
    let mut seen = HashSet::with_capacity(node_ids.len());

    for &node_id in node_ids {
        if !seen.insert(node_id) {
            return Err(duplicate_error(node_id));
        }
    }

    Ok(())
}

impl AstNode for Effect {
    /// Returns the common AST node.
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    /// Returns the mutable common AST node.
    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("effect ")?;
        formatter.write_str(&self.name)?;

        if !self.generic_parameters.is_empty() {
            formatter.write_str("<")?;

            for (index, node_id) in
                self.generic_parameters.iter().enumerate()
            {
                if index != 0 {
                    formatter.write_str(", ")?;
                }

                write!(formatter, "{node_id:?}")?;
            }

            formatter.write_str(">")?;
        }

        formatter.write_str("(")?;

        for (index, node_id) in self.parameters.iter().enumerate() {
            if index != 0 {
                formatter.write_str(", ")?;
            }

            write!(formatter, "{node_id:?}")?;
        }

        formatter.write_str(")")?;

        if let Some(return_type) = self.return_type {
            formatter.write_str(" -> ")?;
            write!(formatter, "{return_type:?}")?;
        }

        formatter.write_str(";")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::node_id::NodeId;
    use super::super::super::source::SourceId;

    fn test_span() -> Span {
        Span::point(SourceId::from_raw(1), Default::default())
    }

    fn test_id(raw: u64) -> NodeId {
        NodeId::from_raw(raw)
    }

    #[test]
    fn constructs_canonical_effect() {
        let effect = Effect::without_metadata(
            test_id(1),
            test_span(),
            "Read",
            vec![test_id(2)],
            vec![test_id(3)],
            Some(test_id(4)),
        );

        assert_eq!(effect.name(), "Read");
        assert_eq!(effect.generic_parameter_count(), 1);
        assert_eq!(effect.parameter_count(), 1);
        assert_eq!(effect.return_type(), Some(test_id(4)));
        assert_eq!(effect.node().kind, effect_node_kind());
    }

    #[test]
    fn preserves_source_order() {
        let effect = Effect::without_metadata(
            test_id(1),
            test_span(),
            "Effect",
            vec![test_id(2), test_id(3), test_id(4)],
            vec![test_id(5), test_id(6)],
            Some(test_id(7)),
        );

        assert_eq!(
            effect.generic_parameters(),
            &[test_id(2), test_id(3), test_id(4)]
        );

        assert_eq!(
            effect.parameters(),
            &[test_id(5), test_id(6)]
        );

        assert_eq!(
            effect.child_node_ids(),
            vec![
                test_id(2),
                test_id(3),
                test_id(4),
                test_id(5),
                test_id(6),
                test_id(7),
            ]
        );
    }

    #[test]
    fn validates_empty_name() {
        let effect = Effect::without_metadata(
            test_id(1),
            test_span(),
            "",
            Vec::new(),
            Vec::new(),
            None,
        );

        assert_eq!(
            effect.validate_structure(),
            Err(EffectValidationError::EmptyName)
        );
    }

    #[test]
    fn validates_wrong_node_kind() {
        let node = Node::new(
            test_id(1),
            NodeKind::Core(CoreNodeKind::Function),
            test_span(),
            NodeMetadata::default(),
        );

        let effect = Effect::from_node(
            node,
            "Read",
            Vec::new(),
            Vec::new(),
            None,
        );

        assert!(matches!(
            effect.validate_structure(),
            Err(EffectValidationError::WrongNodeKind { .. })
        ));
    }

    #[test]
    fn rejects_duplicate_generic_parameter_reference() {
        let id = test_id(2);

        let effect = Effect::without_metadata(
            test_id(1),
            test_span(),
            "Read",
            vec![id, id],
            Vec::new(),
            None,
        );

        assert_eq!(
            effect.validate_structure(),
            Err(
                EffectValidationError::DuplicateGenericParameter {
                    node_id: id,
                }
            )
        );
    }

    #[test]
    fn rejects_duplicate_parameter_reference() {
        let id = test_id(2);

        let effect = Effect::without_metadata(
            test_id(1),
            test_span(),
            "Read",
            Vec::new(),
            vec![id, id],
            None,
        );

        assert_eq!(
            effect.validate_structure(),
            Err(
                EffectValidationError::DuplicateParameter {
                    node_id: id,
                }
            )
        );
    }

    #[test]
    fn rejects_self_reference() {
        let effect_id = test_id(1);

        let effect = Effect::without_metadata(
            effect_id,
            test_span(),
            "Read",
            Vec::new(),
            Vec::new(),
            Some(effect_id),
        );

        assert_eq!(
            effect.validate_structure(),
            Err(EffectValidationError::SelfReferenceAsReturnType)
        );
    }

    #[test]
    fn unrestricted_validation_has_no_artificial_cardinality_limit() {
        let generic_parameters =
            (2_u64..=1025).map(test_id).collect::<Vec<_>>();

        let parameters =
            (1026_u64..=2049).map(test_id).collect::<Vec<_>>();

        let effect = Effect::without_metadata(
            test_id(1),
            test_span(),
            "LargeEffect",
            generic_parameters,
            parameters,
            None,
        );

        assert!(effect.validate_structure().is_ok());
    }

    #[test]
    fn explicit_policy_limits_are_not_language_semantics() {
        let effect = Effect::without_metadata(
            test_id(1),
            test_span(),
            "Read",
            vec![test_id(2), test_id(3)],
            Vec::new(),
            None,
        );

        let policy = EffectValidationPolicy {
            max_name_bytes: None,
            max_generic_parameters: Some(1),
            max_parameters: None,
        };

        assert_eq!(
            effect.validate_structure_with_policy(&policy),
            Err(
                EffectValidationError::TooManyGenericParameters {
                    actual: 2,
                    maximum: 1,
                }
            )
        );
    }

    #[test]
    fn child_iteration_does_not_require_allocation() {
        let effect = Effect::without_metadata(
            test_id(1),
            test_span(),
            "Read",
            vec![test_id(2), test_id(3)],
            vec![test_id(4)],
            Some(test_id(5)),
        );

        let mut visited = Vec::new();

        effect.for_each_child_node_id(|node_id| {
            visited.push(node_id);
        });

        assert_eq!(
            visited,
            vec![
                test_id(2),
                test_id(3),
                test_id(4),
                test_id(5),
            ]
        );
    }

    #[test]
    fn serialization_round_trip_preserves_structure() {
        let effect = Effect::without_metadata(
            test_id(1),
            test_span(),
            "Measure",
            vec![test_id(2)],
            vec![test_id(3)],
            Some(test_id(4)),
        );

        let serialized =
            serde_json::to_string(&effect).expect("serialization must succeed");

        let decoded: Effect =
            serde_json::from_str(&serialized)
                .expect("deserialization must succeed");

        assert_eq!(decoded, effect);
    }
}