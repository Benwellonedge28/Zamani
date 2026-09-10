//! # Zamani Frontend AST — Implementation Declaration
//!
//! Production-grade source-level representation of a Zamani `impl`
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
//! native Zamani AST
//!     │
//!     ├── ImplementationDeclaration  ← this module
//!     │       ├── Node
//!     │       ├── generic parameters
//!     │       ├── optional trait reference
//!     │       ├── implemented/self type
//!     │       ├── constraints
//!     │       └── member references
//!     │
//!     ▼
//! structural validation
//!     │
//!     ▼
//! semantic analysis
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
//! target / runtime / hardware
//! ```
//!
//! ## Purpose
//!
//! [`ImplementationDeclaration`] represents the source-level structure of an
//! implementation declaration.
//!
//! It supports both:
//!
//! - inherent implementations;
//! - trait implementations;
//! - generic implementations;
//! - constrained implementations;
//! - implementations containing arbitrarily many members;
//! - implementations whose target type is itself generic or otherwise
//!   structurally complex.
//!
//! The declaration deliberately stores references to canonical AST nodes using
//! [`NodeId`] rather than duplicating child AST nodes.
//!
//! ## Source-level examples
//!
//! Conceptually, the representation can describe:
//!
//! ```text
//! impl Type {
//!     ...
//! }
//! ```
//!
//! and:
//!
//! ```text
//! impl Trait for Type {
//!     ...
//! }
//! ```
//!
//! as well as generic forms:
//!
//! ```text
//! impl<T> Trait<T> for Type<T>
//! where ...
//! {
//!     ...
//! }
//! ```
//!
//! The concrete syntax belongs to the Zamani parser. This module does not
//! prescribe parser tokens.
//!
//! ## Domain neutrality
//!
//! An implementation declaration is a native source-language construct.
//!
//! This module does not contain:
//!
//! - quantum gates;
//! - qubit IDs;
//! - quantum hardware;
//! - physical topology;
//! - backend IDs;
//! - QIR values;
//! - LLVM values;
//! - MLIR operations;
//! - CPU instructions;
//! - GPU instructions;
//! - FPGA instructions;
//! - scheduling information;
//! - routing information;
//! - calibration data;
//! - QEC implementations;
//! - runtime state.
//!
//! An implementation can therefore describe ordinary classical code,
//! resource-oriented abstractions, quantum-related source abstractions,
//! accelerator abstractions, HDL-facing abstractions, or future computational
//! constructs without changing this AST representation.
//!
//! ## POCO-REAF
//!
//! The implementation declaration contains no machine-size assumption and no
//! finite computational-resource limit.
//!
//! ```text
//! Program Once
//!      │
//!      ▼
//! ImplementationDeclaration
//!      │
//!      ▼
//! semantic resolution
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ├── tiny target
//!      ├── large target
//!      ├── heterogeneous target
//!      ├── distributed target
//!      ├── quantum target
//!      └── future target
//! ```
//!
//! A source-level implementation therefore remains independent of the
//! eventual realization of the program.
//!
//! ## Canonical AST graph
//!
//! Child constructs are represented by [`NodeId`] references.
//!
//! The actual child nodes are owned by the enclosing canonical AST graph/store.
//!
//! This provides the invariant:
//!
//! > Every AST node has one authoritative representation.
//!
//! This design also permits the enclosing AST store to use different safe
//! storage strategies without changing this declaration's contract:
//!
//! - indexed storage;
//! - arenas;
//! - persistent structures;
//! - incremental stores;
//! - compact graph storage;
//! - other safe representations.
//!
//! ## Identity model
//!
//! [`Self::id`] identifies this declaration node.
//!
//! The following IDs identify canonical child nodes:
//!
//! - generic parameters;
//! - optional implemented trait type/reference;
//! - implemented/self type;
//! - where-clause/constraint nodes;
//! - implementation members.
//!
//! A child ID must never be interpreted as:
//!
//! - a symbol ID;
//! - a type ID;
//! - a quantum-resource ID;
//! - a hardware ID;
//! - a backend ID;
//! - a runtime ID;
//! - a ZUIR ID.
//!
//! Those identities belong to later compiler layers.
//!
//! ## Inherent versus trait implementation
//!
//! The optional [`Self::trait_target`] field distinguishes the two source-level
//! forms:
//!
//! ```text
//! trait_target = None
//!     → inherent implementation
//!
//! trait_target = Some(NodeId)
//!     → trait implementation
//! ```
//!
//! This is intentionally a syntactic/source-level distinction.
//!
//! Whether the referenced trait actually exists, is a trait, is implemented
//! legally, satisfies generic constraints, or is coherent belongs to semantic
//! analysis.
//!
//! ## Structural validation boundary
//!
//! [`Self::validate_local`] validates only information owned by this object.
//!
//! It verifies:
//!
//! - the node kind is `CoreNodeKind::Implementation`;
//! - the declaration has a valid self-type reference;
//! - the declaration does not directly reference itself as its own child;
//! - generic/trait/type/constraint/member references are structurally distinct
//!   from the declaration itself.
//!
//! It does **not** verify:
//!
//! - that referenced nodes exist;
//! - that a trait reference resolves to a trait;
//! - that the self type resolves;
//! - that generic parameters are legal;
//! - that where clauses are satisfiable;
//! - that members are legal;
//! - coherence/orphan rules;
//! - method signature compatibility;
//! - type checking;
//! - capability checking;
//! - resource feasibility;
//! - quantum topology;
//! - hardware compatibility.
//!
//! Those checks belong to the enclosing AST validation and semantic-analysis
//! layers.
//!
//! ## Determinism
//!
//! Source order is preserved by `Vec<NodeId>` fields.
//!
//! No hash-map iteration order participates in this declaration's structural
//! representation.
//!
//! The constructor does not generate IDs, timestamps, random values, pointers,
//! or machine-dependent identities.
//!
//! ## Scalability
//!
//! There is no language-level fixed limit on:
//!
//! - implementation declarations;
//! - generic parameters;
//! - constraints;
//! - members;
//! - source files;
//! - type complexity;
//! - computational resources;
//! - quantum resources;
//! - target-machine size.
//!
//! Collections grow according to available resources.
//!
//! Operational limits for hostile-input protection belong to configurable
//! compiler policy infrastructure, not this AST node.
//!
//! ## Security
//!
//! This type:
//!
//! - uses no `unsafe`;
//! - performs no unchecked indexing;
//! - does not dereference pointers;
//! - does not recursively traverse referenced nodes;
//! - does not resolve untrusted references;
//! - does not execute source code;
//! - does not access hardware;
//! - does not access backend credentials.
//!
//! Deep graph traversal and resource limits are handled by dedicated AST
//! validation/traversal infrastructure.
//!
//! ## Serialization
//!
//! The declaration derives `Serialize` and `Deserialize` so the enclosing AST
//! serialization layer can preserve:
//!
//! - node identity;
//! - node kind;
//! - source span;
//! - metadata;
//! - generic parameter references;
//! - optional trait reference;
//! - self-type reference;
//! - constraint references;
//! - member references.
//!
//! No pointer, memory address, semantic side table, hardware state, or runtime
//! state is serialized.
//!
//! The enclosing serialization layer owns the complete AST schema version.
//!
//! ## Visitor/traversal contract
//!
//! This type intentionally exposes its child `NodeId` references in deterministic
//! source order.
//!
//! Generic traversal infrastructure should visit references in this order:
//!
//! ```text
//! generic parameters
//!       ↓
//! optional trait target
//!       ↓
//! self type
//!       ↓
//! constraints
//!       ↓
//! members
//! ```
//!
//! The declaration itself does not depend on the visitor implementation.
//!
//! ## Parser contract
//!
//! The parser is responsible for:
//!
//! 1. allocating the implementation declaration's [`NodeId`];
//! 2. constructing canonical child nodes;
//! 3. recording their IDs;
//! 4. preserving source order;
//! 5. constructing this declaration;
//! 6. inserting the declaration into the canonical AST graph.
//!
//! The parser must not perform semantic trait resolution or target selection.
//!
//! ## Semantic-analysis contract
//!
//! Semantic analysis consumes this declaration and resolves:
//!
//! - the self type;
//! - the optional trait;
//! - generic parameters;
//! - constraints;
//! - implementation members;
//! - symbols;
//! - types;
//! - method compatibility;
//! - coherence rules;
//! - capabilities;
//! - effects;
//! - resource semantics.
//!
//! Semantic information must not be written into this AST node.
//!
//! ## ZUIR contract
//!
//! This declaration does not lower directly to ZUIR.
//!
//! The intended path is:
//!
//! ```text
//! ImplementationDeclaration
//!       │
//!       ▼
//! semantic model
//!       │
//!       ▼
//! ZUIR
//!       │
//!       ▼
//! domain-specific IR
//! ```
//!
//! The exact ZUIR representation is determined by the semantic/lowering
//! architecture.
//!
//! ## Forbidden dependencies
//!
//! This file must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - backend providers;
//! - routing;
//! - scheduling;
//! - optimization;
//! - QEC;
//! - resilience;
//! - runtime;
//! - compiler execution state;
//! - external quantum-language ASTs.
//!
//! Allowed dependencies are limited to foundational native-AST infrastructure
//! and the serialization/standard-library facilities already used by the
//! repository.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1.
//!
//! No unstable APIs are required.
//!
//! No `unsafe` code is used.

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Schema version for the source-level implementation-declaration contract.
///
/// This version is independent of:
///
/// - Zamani language version;
/// - compiler version;
/// - serialized AST version;
/// - extension versions.
pub const IMPLEMENTATION_DECLARATION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level identifier for an implementation declaration.
pub const IMPLEMENTATION_DECLARATION_KIND_NAME: &str = "zamani:implementation";

/// A source-level Zamani implementation declaration.
///
/// An implementation can be:
///
/// - an inherent implementation when [`Self::trait_target`] is `None`;
/// - a trait implementation when [`Self::trait_target`] is `Some`.
///
/// All child constructs are represented by canonical [`NodeId`] references.
///
/// The actual child nodes are owned by the canonical AST graph/store.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImplementationDeclaration {
    /// Common source-level AST identity, classification, span and metadata.
    node: Node,

    /// Generic-parameter nodes in source order.
    ///
    /// These IDs refer to canonical AST nodes owned by the AST graph.
    generic_parameters: Vec<NodeId>,

    /// Optional trait/type reference being implemented.
    ///
    /// `None` means an inherent implementation.
    ///
    /// `Some(id)` means that the implementation has source-level trait-target
    /// syntax. Semantic analysis determines whether the referenced node is
    /// actually a valid trait.
    trait_target: Option<NodeId>,

    /// Type being implemented.
    ///
    /// This is always represented by a canonical AST node reference.
    ///
    /// The node may represent a named type, generic type expression,
    /// constructed type, extension-defined type, or another source-level type
    /// expression.
    self_type: NodeId,

    /// Constraint/where-clause nodes in source order.
    ///
    /// These remain unresolved until semantic analysis.
    constraints: Vec<NodeId>,

    /// Implementation member nodes in source order.
    ///
    /// Members are canonical AST nodes and may represent functions,
    /// associated constants, associated types, nested declarations, or
    /// extension-defined source constructs as permitted by the language.
    members: Vec<NodeId>,
}

/// Errors produced by [`ImplementationDeclaration`] construction or local
/// structural validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImplementationDeclarationError {
    /// The supplied AST node has the wrong core node kind.
    InvalidNodeKind {
        /// Required core node kind.
        expected: CoreNodeKind,

        /// Actual node kind.
        actual: NodeKind,
    },

    /// The implementation's self-type reference points to the implementation
    /// declaration itself.
    SelfTypeSelfReference {
        /// Implementation declaration ID.
        declaration: NodeId,

        /// Invalid referenced ID.
        referenced: NodeId,
    },

    /// The trait target directly points to the implementation declaration
    /// itself.
    TraitTargetSelfReference {
        /// Implementation declaration ID.
        declaration: NodeId,

        /// Invalid referenced ID.
        referenced: NodeId,
    },

    /// A generic parameter directly points to the implementation declaration
    /// itself.
    GenericParameterSelfReference {
        /// Implementation declaration ID.
        declaration: NodeId,

        /// Invalid referenced ID.
        referenced: NodeId,
    },

    /// A constraint directly points to the implementation declaration itself.
    ConstraintSelfReference {
        /// Implementation declaration ID.
        declaration: NodeId,

        /// Invalid referenced ID.
        referenced: NodeId,
    },

    /// A member directly points to the implementation declaration itself.
    MemberSelfReference {
        /// Implementation declaration ID.
        declaration: NodeId,

        /// Invalid referenced ID.
        referenced: NodeId,
    },
}

impl fmt::Display for ImplementationDeclarationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { expected, actual } => write!(
                formatter,
                "invalid implementation declaration node kind: expected \
                 zamani:{}, found {}",
                expected.name(),
                actual
            ),

            Self::SelfTypeSelfReference {
                declaration,
                referenced,
            } => write!(
                formatter,
                "implementation declaration {} cannot use itself ({}) as \
                 its self type",
                declaration,
                referenced
            ),

            Self::TraitTargetSelfReference {
                declaration,
                referenced,
            } => write!(
                formatter,
                "implementation declaration {} cannot use itself ({}) as \
                 its trait target",
                declaration,
                referenced
            ),

            Self::GenericParameterSelfReference {
                declaration,
                referenced,
            } => write!(
                formatter,
                "implementation declaration {} cannot use itself ({}) as a \
                 generic parameter",
                declaration,
                referenced
            ),

            Self::ConstraintSelfReference {
                declaration,
                referenced,
            } => write!(
                formatter,
                "implementation declaration {} cannot use itself ({}) as a \
                 constraint",
                declaration,
                referenced
            ),

            Self::MemberSelfReference {
                declaration,
                referenced,
            } => write!(
                formatter,
                "implementation declaration {} cannot use itself ({}) as a \
                 member",
                declaration,
                referenced
            ),
        }
    }
}

impl std::error::Error for ImplementationDeclarationError {}

impl ImplementationDeclaration {
    /// Creates a new implementation declaration.
    ///
    /// This constructor establishes the declaration's local structural
    /// representation.
    ///
    /// It does not:
    ///
    /// - resolve the self type;
    /// - resolve the trait;
    /// - resolve generic parameters;
    /// - validate constraints;
    /// - validate members;
    /// - perform type checking;
    /// - perform coherence checking;
    /// - inspect hardware;
    /// - select a backend.
    ///
    /// # Arguments
    ///
    /// * `id` — identity allocated by the AST construction layer.
    /// * `span` — complete source span of the implementation declaration.
    /// * `metadata` — source-level metadata.
    /// * `generic_parameters` — generic parameter nodes in source order.
    /// * `trait_target` — optional trait target.
    /// * `self_type` — canonical node representing the implemented type.
    /// * `constraints` — constraint/where-clause nodes in source order.
    /// * `members` — implementation members in source order.
    ///
    /// # Errors
    ///
    /// Returns an error when the declaration would directly reference itself.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        generic_parameters: Vec<NodeId>,
        trait_target: Option<NodeId>,
        self_type: NodeId,
        constraints: Vec<NodeId>,
        members: Vec<NodeId>,
    ) -> Result<Self, ImplementationDeclarationError> {
        let declaration = Self {
            node: Node::new(
                id,
                NodeKind::core(CoreNodeKind::Implementation),
                span,
                metadata,
            ),
            generic_parameters,
            trait_target,
            self_type,
            constraints,
            members,
        };

        declaration.validate_local()?;

        Ok(declaration)
    }

    /// Creates an implementation declaration with default metadata.
    ///
    /// The caller must still provide the node identity and source span.
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        generic_parameters: Vec<NodeId>,
        trait_target: Option<NodeId>,
        self_type: NodeId,
        constraints: Vec<NodeId>,
        members: Vec<NodeId>,
    ) -> Result<Self, ImplementationDeclarationError> {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            generic_parameters,
            trait_target,
            self_type,
            constraints,
            members,
        )
    }

    /// Returns the common AST node.
    #[inline]
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// This is intended for controlled source-level AST transformations.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the implementation declaration's stable AST node identity.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the declaration's canonical source-level node kind.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the declaration's source span.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns source-level metadata.
    #[inline]
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable source-level metadata.
    ///
    /// Semantic information must not be stored in metadata.
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns generic-parameter references in source order.
    #[inline]
    #[must_use]
    pub fn generic_parameters(&self) -> &[NodeId] {
        &self.generic_parameters
    }

    /// Returns mutable generic-parameter references.
    ///
    /// Structural validation should be performed after modifying the
    /// collection.
    #[inline]
    pub fn generic_parameters_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.generic_parameters
    }

    /// Replaces the generic-parameter references.
    ///
    /// Returns the previous collection.
    pub fn replace_generic_parameters(
        &mut self,
        generic_parameters: Vec<NodeId>,
    ) -> Vec<NodeId> {
        core::mem::replace(
            &mut self.generic_parameters,
            generic_parameters,
        )
    }

    /// Returns the number of generic parameters.
    #[inline]
    #[must_use]
    pub fn generic_parameter_count(&self) -> usize {
        self.generic_parameters.len()
    }

    /// Returns whether the implementation is generic.
    #[inline]
    #[must_use]
    pub fn is_generic(&self) -> bool {
        !self.generic_parameters.is_empty()
    }

    /// Returns the optional trait target.
    ///
    /// `None` means an inherent implementation.
    ///
    /// `Some(id)` means the source declares an implementation of the
    /// referenced trait/type construct. Semantic analysis resolves and
    /// validates its actual meaning.
    #[inline]
    #[must_use]
    pub fn trait_target(&self) -> Option<NodeId> {
        self.trait_target
    }

    /// Sets the optional trait target.
    ///
    /// Returns the previous target.
    ///
    /// The declaration cannot directly reference itself.
    pub fn replace_trait_target(
        &mut self,
        trait_target: Option<NodeId>,
    ) -> Result<Option<NodeId>, ImplementationDeclarationError> {
        if let Some(target) = trait_target {
            if target == self.id() {
                return Err(
                    ImplementationDeclarationError::TraitTargetSelfReference {
                        declaration: self.id(),
                        referenced: target,
                    },
                );
            }
        }

        Ok(core::mem::replace(
            &mut self.trait_target,
            trait_target,
        ))
    }

    /// Returns `true` when this is an inherent implementation.
    #[inline]
    #[must_use]
    pub fn is_inherent(&self) -> bool {
        self.trait_target.is_none()
    }

    /// Returns `true` when this is a trait implementation.
    #[inline]
    #[must_use]
    pub fn is_trait_implementation(&self) -> bool {
        self.trait_target.is_some()
    }

    /// Returns the canonical AST node representing the implemented/self type.
    #[inline]
    #[must_use]
    pub fn self_type(&self) -> NodeId {
        self.self_type
    }

    /// Replaces the implemented/self type.
    ///
    /// Returns the previous reference.
    ///
    /// The implementation declaration cannot directly use itself as its own
    /// self type.
    pub fn replace_self_type(
        &mut self,
        self_type: NodeId,
    ) -> Result<NodeId, ImplementationDeclarationError> {
        if self_type == self.id() {
            return Err(
                ImplementationDeclarationError::SelfTypeSelfReference {
                    declaration: self.id(),
                    referenced: self_type,
                },
            );
        }

        Ok(core::mem::replace(&mut self.self_type, self_type))
    }

    /// Returns constraint/where-clause references in source order.
    #[inline]
    #[must_use]
    pub fn constraints(&self) -> &[NodeId] {
        &self.constraints
    }

    /// Returns mutable constraint references.
    ///
    /// Structural validation should be performed after modifying the
    /// collection.
    #[inline]
    pub fn constraints_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.constraints
    }

    /// Replaces the constraint references.
    ///
    /// Returns the previous collection.
    pub fn replace_constraints(
        &mut self,
        constraints: Vec<NodeId>,
    ) -> Vec<NodeId> {
        core::mem::replace(&mut self.constraints, constraints)
    }

    /// Returns the number of constraints.
    #[inline]
    #[must_use]
    pub fn constraint_count(&self) -> usize {
        self.constraints.len()
    }

    /// Returns whether this implementation has explicit constraints.
    #[inline]
    #[must_use]
    pub fn is_constrained(&self) -> bool {
        !self.constraints.is_empty()
    }

    /// Returns implementation members in source order.
    #[inline]
    #[must_use]
    pub fn members(&self) -> &[NodeId] {
        &self.members
    }

    /// Returns mutable member references.
    ///
    /// Structural validation should be performed after modifying the
    /// collection.
    #[inline]
    pub fn members_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.members
    }

    /// Replaces the implementation members.
    ///
    /// Returns the previous collection.
    pub fn replace_members(&mut self, members: Vec<NodeId>) -> Vec<NodeId> {
        core::mem::replace(&mut self.members, members)
    }

    /// Returns the number of implementation members.
    #[inline]
    #[must_use]
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Returns whether the implementation contains no members.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Returns the implementation declaration schema version.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        IMPLEMENTATION_DECLARATION_SCHEMA_VERSION
    }

    /// Returns the stable source-level kind identifier.
    #[inline]
    #[must_use]
    pub const fn stable_kind_name() -> &'static str {
        IMPLEMENTATION_DECLARATION_KIND_NAME
    }

    /// Performs structural validation local to this declaration.
    ///
    /// This function intentionally does not inspect the canonical AST store.
    ///
    /// Consequently it cannot and should not verify that child IDs exist.
    /// That responsibility belongs to AST graph validation.
    pub fn validate_local(&self) -> Result<(), ImplementationDeclarationError> {
        if self.node.kind().as_core() != Some(CoreNodeKind::Implementation) {
            return Err(
                ImplementationDeclarationError::InvalidNodeKind {
                    expected: CoreNodeKind::Implementation,
                    actual: self.node.kind().clone(),
                },
            );
        }

        let declaration_id = self.id();

        if self.self_type == declaration_id {
            return Err(
                ImplementationDeclarationError::SelfTypeSelfReference {
                    declaration: declaration_id,
                    referenced: self.self_type,
                },
            );
        }

        if let Some(trait_target) = self.trait_target {
            if trait_target == declaration_id {
                return Err(
                    ImplementationDeclarationError::TraitTargetSelfReference {
                        declaration: declaration_id,
                        referenced: trait_target,
                    },
                );
            }
        }

        for &generic_parameter in &self.generic_parameters {
            if generic_parameter == declaration_id {
                return Err(
                    ImplementationDeclarationError::GenericParameterSelfReference {
                        declaration: declaration_id,
                        referenced: generic_parameter,
                    },
                );
            }
        }

        for &constraint in &self.constraints {
            if constraint == declaration_id {
                return Err(
                    ImplementationDeclarationError::ConstraintSelfReference {
                        declaration: declaration_id,
                        referenced: constraint,
                    },
                );
            }
        }

        for &member in &self.members {
            if member == declaration_id {
                return Err(
                    ImplementationDeclarationError::MemberSelfReference {
                        declaration: declaration_id,
                        referenced: member,
                    },
                );
            }
        }

        Ok(())
    }

    /// Returns whether the declaration satisfies its local structural
    /// invariants.
    #[inline]
    #[must_use]
    pub fn is_locally_valid(&self) -> bool {
        self.validate_local().is_ok()
    }

    /// Returns all direct child references in deterministic source-oriented
    /// traversal order.
    ///
    /// The order is:
    ///
    /// 1. generic parameters;
    /// 2. optional trait target;
    /// 3. self type;
    /// 4. constraints;
    /// 5. members.
    ///
    /// The returned vector is newly allocated. Use
    /// [`Self::for_each_reference`] for allocation-free traversal.
    #[must_use]
    pub fn referenced_nodes(&self) -> Vec<NodeId> {
        let additional = usize::from(self.trait_target.is_some())
            + 1
            + self.generic_parameters.len()
            + self.constraints.len()
            + self.members.len();

        let mut references = Vec::with_capacity(additional);

        references.extend(self.generic_parameters.iter().copied());

        if let Some(trait_target) = self.trait_target {
            references.push(trait_target);
        }

        references.push(self.self_type);

        references.extend(self.constraints.iter().copied());
        references.extend(self.members.iter().copied());

        references
    }

    /// Visits all direct child references without allocating a temporary
    /// collection.
    ///
    /// The callback is invoked in deterministic source-oriented order:
    ///
    /// 1. generic parameters;
    /// 2. optional trait target;
    /// 3. self type;
    /// 4. constraints;
    /// 5. members.
    pub fn for_each_reference<F>(&self, mut callback: F)
    where
        F: FnMut(NodeId),
    {
        for &generic_parameter in &self.generic_parameters {
            callback(generic_parameter);
        }

        if let Some(trait_target) = self.trait_target {
            callback(trait_target);
        }

        callback(self.self_type);

        for &constraint in &self.constraints {
            callback(constraint);
        }

        for &member in &self.members {
            callback(member);
        }
    }

    /// Returns the number of direct child references.
    ///
    /// This is a structural count only and does not impose a language-level
    /// maximum.
    #[inline]
    #[must_use]
    pub fn reference_count(&self) -> usize {
        self.generic_parameters.len()
            + usize::from(self.trait_target.is_some())
            + 1
            + self.constraints.len()
            + self.members.len()
    }

    /// Returns a compact diagnostic description.
    ///
    /// The description deliberately excludes metadata and complete child
    /// contents so diagnostics cannot accidentally expand arbitrarily large
    /// AST payloads.
    pub fn diagnostic_summary(&self) -> ImplementationDiagnosticSummary {
        ImplementationDiagnosticSummary {
            id: self.id(),
            self_type: self.self_type,
            trait_target: self.trait_target,
            generic_parameter_count: self.generic_parameter_count(),
            constraint_count: self.constraint_count(),
            member_count: self.member_count(),
            span: self.span().clone(),
        }
    }
}

impl fmt::Display for ImplementationDeclaration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.trait_target.is_some() {
            write!(formatter, "impl <trait> for <type>")
        } else {
            write!(formatter, "impl <type>")
        }
    }
}

/// Lightweight diagnostic information for an implementation declaration.
///
/// This type intentionally does not contain the complete AST graph or metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImplementationDiagnosticSummary {
    /// Stable AST identity.
    pub id: NodeId,

    /// Canonical AST node representing the implemented/self type.
    pub self_type: NodeId,

    /// Optional canonical AST node representing the trait target.
    pub trait_target: Option<NodeId>,

    /// Number of generic parameters.
    pub generic_parameter_count: usize,

    /// Number of constraints.
    pub constraint_count: usize,

    /// Number of implementation members.
    pub member_count: usize,

    /// Source span of the declaration.
    pub span: Span,
}

impl fmt::Display for ImplementationDiagnosticSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.trait_target.is_some() {
            write!(
                formatter,
                "implementation {}: trait target {}, self type {}, \
                 {} generic parameters, {} constraints, {} members at {}",
                self.id,
                self.trait_target
                    .expect("trait target checked above"),
                self.self_type,
                self.generic_parameter_count,
                self.constraint_count,
                self.member_count,
                self.span
            )
        } else {
            write!(
                formatter,
                "implementation {}: self type {}, {} generic parameters, \
                 {} constraints, {} members at {}",
                self.id,
                self.self_type,
                self.generic_parameter_count,
                self.constraint_count,
                self.member_count,
                self.span
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u64) -> NodeId {
        NodeId::new(value).expect("test IDs must be non-zero")
    }

    fn span() -> Span {
        Span::default()
    }

    fn metadata() -> NodeMetadata {
        NodeMetadata::default()
    }

    #[test]
    fn creates_inherent_implementation() {
        let implementation = ImplementationDeclaration::new(
            id(1),
            span(),
            metadata(),
            vec![id(2)],
            None,
            id(3),
            vec![id(4)],
            vec![id(5), id(6)],
        )
        .expect("valid implementation");

        assert_eq!(implementation.id(), id(1));
        assert_eq!(
            implementation.kind().as_core(),
            Some(CoreNodeKind::Implementation)
        );
        assert!(implementation.is_inherent());
        assert!(!implementation.is_trait_implementation());
        assert!(implementation.is_generic());
        assert!(implementation.is_constrained());
        assert_eq!(implementation.member_count(), 2);
        assert_eq!(implementation.self_type(), id(3));
    }

    #[test]
    fn creates_trait_implementation() {
        let implementation = ImplementationDeclaration::new(
            id(1),
            span(),
            metadata(),
            vec![id(2)],
            Some(id(3)),
            id(4),
            vec![id(5)],
            vec![id(6)],
        )
        .expect("valid implementation");

        assert!(implementation.is_trait_implementation());
        assert!(!implementation.is_inherent());
        assert_eq!(implementation.trait_target(), Some(id(3)));
        assert_eq!(implementation.self_type(), id(4));
    }

    #[test]
    fn rejects_self_type_self_reference() {
        let result = ImplementationDeclaration::new(
            id(1),
            span(),
            metadata(),
            Vec::new(),
            None,
            id(1),
            Vec::new(),
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(
                ImplementationDeclarationError::SelfTypeSelfReference {
                    declaration,
                    referenced
                }
            ) if declaration == id(1) && referenced == id(1)
        ));
    }

    #[test]
    fn rejects_trait_target_self_reference() {
        let result = ImplementationDeclaration::new(
            id(1),
            span(),
            metadata(),
            Vec::new(),
            Some(id(1)),
            id(2),
            Vec::new(),
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(
                ImplementationDeclarationError::TraitTargetSelfReference {
                    declaration,
                    referenced
                }
            ) if declaration == id(1) && referenced == id(1)
        ));
    }

    #[test]
    fn rejects_generic_parameter_self_reference() {
        let result = ImplementationDeclaration::new(
            id(1),
            span(),
            metadata(),
            vec![id(1)],
            None,
            id(2),
            Vec::new(),
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(
                ImplementationDeclarationError::GenericParameterSelfReference {
                    declaration,
                    referenced
                }
            ) if declaration == id(1) && referenced == id(1)
        ));
    }

    #[test]
    fn rejects_constraint_self_reference() {
        let result = ImplementationDeclaration::new(
            id(1),
            span(),
            metadata(),
            Vec::new(),
            None,
            id(2),
            vec![id(1)],
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(
                ImplementationDeclarationError::ConstraintSelfReference {
                    declaration,
                    referenced
                }
            ) if declaration == id(1) && referenced == id(1)
        ));
    }

    #[test]
    fn rejects_member_self_reference() {
        let result = ImplementationDeclaration::new(
            id(1),
            span(),
            metadata(),
            Vec::new(),
            None,
            id(2),
            Vec::new(),
            vec![id(1)],
        );

        assert!(matches!(
            result,
            Err(
                ImplementationDeclarationError::MemberSelfReference {
                    declaration,
                    referenced
                }
            ) if declaration == id(1) && referenced == id(1)
        ));
    }

    #[test]
    fn preserves_deterministic_reference_order() {
        let implementation = ImplementationDeclaration::new(
            id(1),
            span(),
            metadata(),
            vec![id(2), id(3)],
            Some(id(4)),
            id(5),
            vec![id(6), id(7)],
            vec![id(8), id(9)],
        )
        .expect("valid implementation");

        assert_eq!(
            implementation.referenced_nodes(),
            vec![
                id(2),
                id(3),
                id(4),
                id(5),
                id(6),
                id(7),
                id(8),
                id(9),
            ]
        );
    }

    #[test]
    fn allocation_free_reference_iteration_matches_reference_list() {
        let implementation = ImplementationDeclaration::new(
            id(1),
            span(),
            metadata(),
            vec![id(2), id(3)],
            Some(id(4)),
            id(5),
            vec![id(6)],
            vec![id(7), id(8)],
        )
        .expect("valid implementation");

        let mut visited = Vec::new();

        implementation.for_each_reference(|reference| {
            visited.push(reference);
        });

        assert_eq!(visited, implementation.referenced_nodes());
    }

    #[test]
    fn reports_empty_member_collection() {
        let implementation = ImplementationDeclaration::new(
            id(1),
            span(),
            metadata(),
            Vec::new(),
            None,
            id(2),
            Vec::new(),
            Vec::new(),
        )
        .expect("valid implementation");

        assert!(implementation.is_empty());
        assert_eq!(implementation.member_count(), 0);
    }

    #[test]
    fn supports_large_reference_collections_without_language_level_limit() {
        let generic_parameters: Vec<NodeId> = (2..=128)
            .map(id)
            .collect();

        let members: Vec<NodeId> = (129..=256)
            .map(id)
            .collect();

        let implementation = ImplementationDeclaration::new(
            id(1),
            span(),
            metadata(),
            generic_parameters,
            None,
            id(257),
            Vec::new(),
            members,
        )
        .expect("valid implementation");

        assert_eq!(implementation.generic_parameter_count(), 127);
        assert_eq!(implementation.member_count(), 128);
    }

    #[test]
    fn local_validation_accepts_recursive_type_graphs_outside_direct_self_reference() {
        // A type graph can legitimately be recursive through other nodes.
        // This declaration only rejects the direct impossible edge where the
        // implementation node is itself the referenced child.
        let implementation = ImplementationDeclaration::new(
            id(1),
            span(),
            metadata(),
            vec![id(2)],
            Some(id(3)),
            id(4),
            vec![id(5)],
            vec![id(6)],
        )
        .expect("valid implementation");

        assert!(implementation.is_locally_valid());
    }

    #[test]
    fn schema_contract_is_stable() {
        assert_eq!(
            ImplementationDeclaration::schema_version(),
            IMPLEMENTATION_DECLARATION_SCHEMA_VERSION
        );

        assert_eq!(
            ImplementationDeclaration::stable_kind_name(),
            "zamani:implementation"
        );
    }
}