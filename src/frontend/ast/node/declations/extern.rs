//! # Zamani Frontend AST — External Declaration
//!
//! Source-level representation of an external/foreign interface declaration.
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
//! Native Zamani AST
//!     │
//!     ├── ExternalDeclaration
//!     │       ├── Node
//!     │       ├── optional external source/path
//!     │       └── ordered declaration NodeIds
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
//! domain / target / runtime integration
//! ```
//!
//! ## Purpose
//!
//! `ExternalDeclaration` represents source-level declarations whose callable
//! implementations are supplied outside the current Zamani source program.
//!
//! The declaration is intentionally an interface description rather than an
//! execution mechanism.
//!
//! It may therefore describe an external interface implemented by:
//!
//! - a native library;
//! - another programming language;
//! - a runtime service;
//! - a classical processor;
//! - a GPU/accelerator;
//! - a quantum runtime;
//! - an HDL environment;
//! - a distributed service;
//! - a future computational system.
//!
//! The AST does not determine which of these ultimately supplies the
//! implementation.
//!
//! ## Grammar integration
//!
//! The repository grammar currently defines the native `extern` construct
//! conceptually as:
//!
//! ```text
//! foreignFunctionCall
//!     : 'extern' STRING '{' externDecl* '}'
//!     | 'foreign' IDENTIFIER '::' IDENTIFIER '(' argumentList? ')' ';'
//!     ;
//!
//! externDecl
//!     : visibilityModifier?
//!       'fn'
//!       IDENTIFIER
//!       '(' parameterList? ')'
//!       ('->' typeExpr)?
//!       ';'
//!     ;
//! ```
//!
//! Therefore this node represents the enclosing `extern STRING { ... }`
//! declaration. Individual external functions remain canonical child AST
//! nodes, referenced by [`NodeId`].
//!
//! The grammar is the source-language authority. If the grammar evolves,
//! parser integration must evolve while preserving the architectural contract
//! described by this file.
//!
//! ## Domain neutrality
//!
//! This type deliberately does **not** contain:
//!
//! - ABI-specific enums;
//! - CPU handles;
//! - GPU handles;
//! - QPU handles;
//! - LLVM types;
//! - LLVM values;
//! - QIR values;
//! - MLIR operations;
//! - OpenQASM nodes;
//! - vendor identifiers;
//! - hardware topology;
//! - physical qubit identifiers;
//! - device queues;
//! - runtime handles;
//! - backend credentials;
//! - scheduling information;
//! - routing information;
//! - calibration data;
//! - error-correction implementation;
//! - resilience implementation.
//!
//! Those concerns belong to later compiler/runtime layers.
//!
//! ## POCO-REAF
//!
//! External declarations do not encode a fixed machine size, processor,
//! register size, qubit count, hardware topology, vendor, backend or execution
//! environment.
//!
//! A single source-level declaration can consequently participate in
//! compilation for different environments:
//!
//! ```text
//!                    ExternalDeclaration
//!                            │
//!                            ▼
//!                     semantic analysis
//!                            │
//!                            ▼
//!                      capability match
//!                            │
//!              ┌─────────────┼─────────────┐
//!              ▼             ▼             ▼
//!          native lib     runtime       quantum/other
//!              │             │             │
//!              └─────────────┼─────────────┘
//!                            ▼
//!                         target
//! ```
//!
//! The AST describes the programmer's interface intent. The implementation
//! mechanism is selected downstream.
//!
//! ## Canonical child ownership
//!
//! `ExternalDeclaration` does not embed concrete child declarations.
//!
//! Instead it stores their canonical [`NodeId`] values.
//!
//! This preserves the repository invariant that every concrete AST node has
//! one authoritative representation.
//!
//! ```text
//! ExternalDeclaration
//!        │
//!        ├── NodeId ──► external function declaration
//!        ├── NodeId ──► external function declaration
//!        └── NodeId ──► ...
//! ```
//!
//! The enclosing AST graph/store owns the actual child nodes.
//!
//! This permits the storage implementation to evolve independently toward:
//!
//! - indexed storage;
//! - arenas;
//! - persistent stores;
//! - incremental stores;
//! - other safe representations.
//!
//! ## Dependency contract
//!
//! This file may depend only on:
//!
//! - foundational native AST infrastructure;
//! - [`Node`];
//! - [`NodeId`];
//! - [`NodeKind`];
//! - [`CoreNodeKind`];
//! - [`NodeMetadata`];
//! - [`Span`];
//! - Rust standard-library facilities;
//! - `serde`.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime;
//! - backend providers;
//! - LLVM;
//! - MLIR;
//! - QIR;
//! - OpenQASM;
//! - external-language AST implementations.
//!
//! ## Structural versus semantic validation
//!
//! This file performs only declaration-local structural validation.
//!
//! It validates:
//!
//! - the canonical node kind;
//! - the external source/path when supplied;
//! - direct child identity constraints;
//! - duplicate direct child IDs;
//! - self-reference.
//!
//! It does **not** determine:
//!
//! - whether the external library exists;
//! - whether the path resolves;
//! - whether a foreign function exists;
//! - whether the function signature is compatible;
//! - whether an ABI is valid;
//! - whether an implementation is available;
//! - whether a target supports the declaration;
//! - whether an external call is safe to execute.
//!
//! Those decisions belong to semantic analysis, capability resolution,
//! linking, and/or runtime policy.
//!
//! ## Source ordering
//!
//! External declarations preserve the source ordering of their child
//! declarations.
//!
//! This is required for:
//!
//! - deterministic traversal;
//! - deterministic serialization;
//! - diagnostics;
//! - tooling;
//! - source reconstruction;
//! - reproducible compilation.
//!
//! ## Scalability
//!
//! There is no language-level maximum for:
//!
//! - external declarations;
//! - external functions;
//! - declaration nesting;
//! - source size;
//! - resource size;
//! - machine size;
//! - computational domain;
//! - number of supported implementations.
//!
//! `Vec<NodeId>` grows according to available resources.
//!
//! Hostile-input limits must be supplied by configurable compiler/resource
//! policies rather than hidden constants in this AST node.
//!
//! ## Determinism
//!
//! This type does not generate:
//!
//! - timestamps;
//! - random values;
//! - pointer identities;
//! - memory-address-derived IDs;
//! - process-global state.
//!
//! Node identities are supplied by the AST construction layer.
//!
//! Child order is preserved exactly.
//!
//! ## Serialization
//!
//! The declaration derives Serde serialization and contains only source-level
//! information and canonical node references.
//!
//! Overall serialization compatibility is governed by the AST serialization
//! layer. This local schema version is intentionally independent of:
//!
//! - Zamani language version;
//! - compiler version;
//! - serialization format version;
//! - extension versions.
//!
//! ## Security
//!
//! This type uses no:
//!
//! - `unsafe`;
//! - raw pointers;
//! - unchecked indexing;
//! - global mutable state;
//! - filesystem access;
//! - process execution;
//! - dynamic library loading.
//!
//! An external declaration is therefore inert AST data. Merely constructing or
//! deserializing this node must never execute external code.
//!
//! ## Thread safety
//!
//! The type contains ordinary owned Rust data and no global mutable state.
//! Immutable instances may therefore be shared safely when the containing AST
//! graph is shared safely.
//!
//! ## Integration contract
//!
//! ### Parser
//!
//! The parser:
//!
//! 1. recognizes `extern`;
//! 2. parses the source string/path expression according to the grammar;
//! 3. parses each `externDecl`;
//! 4. constructs canonical child AST nodes;
//! 5. obtains their [`NodeId`] values;
//! 6. constructs this `ExternalDeclaration`;
//! 7. inserts the resulting node into the AST store.
//!
//! The parser must not resolve or load the external implementation.
//!
//! ### Structural validation
//!
//! The validator checks this node locally. AST-graph validation must additionally
//! verify that every stored child ID resolves to a valid canonical declaration.
//!
//! ### Semantic analysis
//!
//! Semantic analysis resolves:
//!
//! - external source identity;
//! - external function symbols;
//! - parameter types;
//! - result types;
//! - visibility;
//! - capabilities;
//! - effects;
//! - resource requirements;
//! - implementation/linking constraints.
//!
//! These must remain outside this AST structure.
//!
//! ### ZUIR
//!
//! The external declaration itself normally does not become an executable ZUIR
//! operation. Its child declarations contribute callable/interface metadata
//! used by semantic analysis and later lowering.
//!
//! An actual call to an external function is represented by the appropriate
//! source expression/call node and lowered according to its resolved semantic
//! identity.
//!
//! ### Quantum integration
//!
//! This node may describe an external callable used by quantum or hybrid code,
//! but it must never contain quantum hardware objects.
//!
//! For example, an external function could ultimately resolve to a quantum
//! service, simulator, accelerator, or runtime. That resolution is downstream.
//!
//! ### External-language integration
//!
//! External formats such as OpenQASM must retain their own ASTs under their
//! respective format frontend directories. They must not be embedded into this
//! native node.
//!
//! ## Migration contract
//!
//! This file is the canonical destination for the native `extern` declaration.
//!
//! There must be exactly one authoritative native `ExternalDeclaration` type.
//!
//! Legacy AST representations must be mapped into this type or explicitly
//! retired during AST migration.
//!
//! The older misspelled `declations/` directory must not become a second
//! authoritative declaration hierarchy.
//!
//! ## Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021.
//!
//! No nightly features are required.
//! No `unsafe` code is used.
//!
//! ## Definition of done
//!
//! This file is complete when:
//!
//! - the canonical type exists;
//! - the canonical node kind is used;
//! - child nodes use canonical `NodeId`s;
//! - local invariants are enforced;
//! - source order is preserved;
//! - serialization is available;
//! - parser integration is wired;
//! - structural validation is wired;
//! - semantic integration is defined;
//! - ZUIR integration is defined;
//! - visitors/traversal can discover the node;
//! - tests cover construction and invariants;
//! - no backend/hardware dependency exists;
//! - no hidden computational-size limit exists;
//! - no `unsafe` code exists.
//!
//! The file must not need to be reopened merely because an unrelated AST node
//! is implemented later, provided the public contract above remains unchanged.

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Schema version for the native source-level external declaration.
///
/// This is deliberately independent from the Zamani language version,
/// compiler version, serialization format version, and extension versions.
pub const EXTERNAL_DECLARATION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level identifier for this AST construct.
pub const EXTERNAL_DECLARATION_KIND_NAME: &str = "zamani:external-declaration";

/// Source-level external interface declaration.
///
/// An external declaration identifies an optional external source/path and
/// maintains an ordered collection of canonical child declaration IDs.
///
/// The external implementation itself is intentionally not represented here.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExternalDeclaration {
    /// Common AST identity, classification, source span and metadata.
    node: Node,

    /// Optional source-level external implementation identifier.
    ///
    /// This is the string appearing in the source-level declaration. It is
    /// intentionally not interpreted as a filesystem path, URL, library name,
    /// package name, ABI, or backend identifier by this AST node.
    source: Option<String>,

    /// Ordered child external declarations.
    ///
    /// Each ID refers to an authoritative AST node in the enclosing AST graph.
    declarations: Vec<NodeId>,
}

impl ExternalDeclaration {
    /// Creates a new external declaration.
    ///
    /// This constructor creates the canonical `ExternalDeclaration` node and
    /// validates declaration-local structural invariants.
    ///
    /// No filesystem, network, dynamic-library, or runtime operation occurs.
    ///
    /// # Errors
    ///
    /// Returns [`ExternalDeclarationError`] when:
    ///
    /// - the source identifier is empty after validation;
    /// - the source identifier contains an invalid NUL character;
    /// - the node identity is inconsistent;
    /// - the declaration directly contains itself;
    /// - a direct child ID is duplicated.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        source: Option<String>,
        declarations: Vec<NodeId>,
    ) -> Result<Self, ExternalDeclarationError> {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::ExternalDeclaration),
            span,
            metadata,
        );

        Self::try_from_node(node, source, declarations)
    }

    /// Creates an external declaration from an already constructed canonical
    /// [`Node`].
    ///
    /// This constructor is useful to AST builders that centralize node
    /// construction.
    pub fn try_from_node(
        node: Node,
        source: Option<String>,
        declarations: Vec<NodeId>,
    ) -> Result<Self, ExternalDeclarationError> {
        validate_source(source.as_deref())?;

        let declaration = Self {
            node,
            source,
            declarations,
        };

        declaration.validate_local()?;

        Ok(declaration)
    }

    /// Creates an external declaration without an external source identifier.
    ///
    /// This is useful for parser recovery and programmatic construction where
    /// the external implementation is supplied by a later semantic phase.
    pub fn anonymous(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        declarations: Vec<NodeId>,
    ) -> Result<Self, ExternalDeclarationError> {
        Self::new(id, span, metadata, None, declarations)
    }

    /// Creates an empty external declaration with a source identifier.
    ///
    /// This is useful when parsing the enclosing `extern "..." { ... }`
    /// construct before its child declarations have been parsed.
    pub fn empty(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        source: impl Into<String>,
    ) -> Result<Self, ExternalDeclarationError> {
        Self::new(
            id,
            span,
            metadata,
            Some(source.into()),
            Vec::new(),
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
    /// This is intended for controlled AST transformations.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST node ID.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the canonical node kind.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the source span.
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
    #[inline]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the optional external source identifier.
    ///
    /// The value is source data only. No interpretation or resolution is
    /// performed by this AST node.
    #[inline]
    #[must_use]
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    /// Returns mutable access to the optional source identifier.
    ///
    /// Call [`Self::validate_local`] after mutation.
    #[inline]
    pub fn source_mut(&mut self) -> &mut Option<String> {
        &mut self.source
    }

    /// Replaces the external source identifier.
    ///
    /// Returns the previous value.
    ///
    /// # Errors
    ///
    /// Returns [`ExternalDeclarationError`] if the new source identifier is
    /// structurally invalid.
    pub fn replace_source(
        &mut self,
        source: Option<String>,
    ) -> Result<Option<String>, ExternalDeclarationError> {
        validate_source(source.as_deref())?;

        Ok(core::mem::replace(&mut self.source, source))
    }

    /// Returns the ordered canonical child declaration IDs.
    #[inline]
    #[must_use]
    pub fn declarations(&self) -> &[NodeId] {
        &self.declarations
    }

    /// Returns mutable access to the ordered child declaration IDs.
    ///
    /// Structural validation should be performed after mutation.
    #[inline]
    pub fn declarations_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.declarations
    }

    /// Replaces the complete child declaration sequence.
    ///
    /// Returns the previous sequence.
    ///
    /// The replacement is checked for local invariants before being installed.
    pub fn replace_declarations(
        &mut self,
        declarations: Vec<NodeId>,
    ) -> Result<Vec<NodeId>, ExternalDeclarationError> {
        validate_declarations(self.id(), &declarations)?;

        Ok(core::mem::replace(
            &mut self.declarations,
            declarations,
        ))
    }

    /// Returns the number of direct external declarations.
    #[inline]
    #[must_use]
    pub fn declaration_count(&self) -> usize {
        self.declarations.len()
    }

    /// Returns whether this external declaration contains no direct
    /// declarations.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.declarations.is_empty()
    }

    /// Appends one canonical child declaration ID.
    ///
    /// Source order is preserved.
    ///
    /// # Errors
    ///
    /// Returns an error if `declaration` equals this node's own ID or is already
    /// present as a direct child.
    pub fn push_declaration(
        &mut self,
        declaration: NodeId,
    ) -> Result<(), ExternalDeclarationError> {
        if declaration == self.id() {
            return Err(ExternalDeclarationError::SelfReference {
                id: declaration,
            });
        }

        if self.declarations.contains(&declaration) {
            return Err(ExternalDeclarationError::DuplicateDeclaration {
                id: declaration,
            });
        }

        self.declarations.push(declaration);

        Ok(())
    }

    /// Inserts one canonical child declaration ID at `index`.
    ///
    /// Source order remains explicit and deterministic.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - `declaration` equals this node's ID;
    /// - `declaration` already exists;
    /// - `index` is greater than the current number of declarations.
    pub fn insert_declaration(
        &mut self,
        index: usize,
        declaration: NodeId,
    ) -> Result<(), ExternalDeclarationError> {
        if declaration == self.id() {
            return Err(ExternalDeclarationError::SelfReference {
                id: declaration,
            });
        }

        if self.declarations.contains(&declaration) {
            return Err(ExternalDeclarationError::DuplicateDeclaration {
                id: declaration,
            });
        }

        if index > self.declarations.len() {
            return Err(ExternalDeclarationError::IndexOutOfBounds {
                index,
                len: self.declarations.len(),
            });
        }

        self.declarations.insert(index, declaration);

        Ok(())
    }

    /// Removes a child declaration by position.
    ///
    /// Returns the removed ID when the position exists.
    #[inline]
    pub fn remove_declaration(&mut self, index: usize) -> Option<NodeId> {
        if index < self.declarations.len() {
            Some(self.declarations.remove(index))
        } else {
            None
        }
    }

    /// Removes the first occurrence of a child declaration ID.
    ///
    /// Returns `true` when an entry was removed.
    pub fn remove_declaration_id(&mut self, declaration: NodeId) -> bool {
        if let Some(index) = self
            .declarations
            .iter()
            .position(|candidate| *candidate == declaration)
        {
            self.declarations.remove(index);
            true
        } else {
            false
        }
    }

    /// Clears all child declaration IDs.
    ///
    /// This operation cannot violate the local child-ID invariants.
    pub fn clear_declarations(&mut self) {
        self.declarations.clear();
    }

    /// Returns the child declaration at `index`.
    #[inline]
    #[must_use]
    pub fn declaration(&self, index: usize) -> Option<NodeId> {
        self.declarations.get(index).copied()
    }

    /// Returns whether the supplied child ID is a direct child.
    #[inline]
    #[must_use]
    pub fn contains_declaration(&self, declaration: NodeId) -> bool {
        self.declarations.contains(&declaration)
    }

    /// Validates all invariants that can be checked without access to the
    /// enclosing AST graph.
    ///
    /// Cross-node validation, such as verifying that a child ID resolves to an
    /// actual declaration node, belongs to the AST graph validator.
    pub fn validate_local(&self) -> Result<(), ExternalDeclarationError> {
        match self.node.kind().as_core() {
            Some(CoreNodeKind::ExternalDeclaration) => {}
            Some(actual) => {
                return Err(ExternalDeclarationError::InvalidNodeKind {
                    expected: CoreNodeKind::ExternalDeclaration,
                    actual,
                });
            }
            None => {
                return Err(ExternalDeclarationError::InvalidNodeKind {
                    expected: CoreNodeKind::ExternalDeclaration,
                    actual: CoreNodeKind::Opaque,
                });
            }
        }

        if self.id().is_zero() {
            return Err(ExternalDeclarationError::ZeroNodeId);
        }

        validate_source(self.source.as_deref())?;
        validate_declarations(self.id(), &self.declarations)?;

        Ok(())
    }
}

/// Validates the optional source identifier.
///
/// The AST intentionally performs only representation-level checks.
///
/// It does not interpret the value as a path, URL, package, library,
/// executable, backend, device, or ABI.
fn validate_source(source: Option<&str>) -> Result<(), ExternalDeclarationError> {
    let Some(source) = source else {
        return Ok(());
    };

    if source.is_empty() {
        return Err(ExternalDeclarationError::EmptySource);
    }

    if source.contains('\0') {
        return Err(ExternalDeclarationError::NulInSource);
    }

    Ok(())
}

/// Validates direct child IDs without requiring the complete AST graph.
fn validate_declarations(
    owner: NodeId,
    declarations: &[NodeId],
) -> Result<(), ExternalDeclarationError> {
    for declaration in declarations {
        if declaration.is_zero() {
            return Err(ExternalDeclarationError::ZeroChildNodeId);
        }

        if *declaration == owner {
            return Err(ExternalDeclarationError::SelfReference {
                id: *declaration,
            });
        }
    }

    for (index, declaration) in declarations.iter().enumerate() {
        if declarations[..index].contains(declaration) {
            return Err(ExternalDeclarationError::DuplicateDeclaration {
                id: *declaration,
            });
        }
    }

    Ok(())
}

/// Errors produced by declaration-local validation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ExternalDeclarationError {
    /// The external source identifier was present but empty.
    EmptySource,

    /// The external source identifier contained a NUL character.
    NulInSource,

    /// The node was assigned an invalid zero ID.
    ZeroNodeId,

    /// A child declaration ID was zero.
    ZeroChildNodeId,

    /// The node did not have the required canonical node kind.
    InvalidNodeKind {
        /// Required canonical node kind.
        expected: CoreNodeKind,

        /// Actual core node kind when one exists.
        ///
        /// `CoreNodeKind::Opaque` is used when the actual node kind is an
        /// extension kind because the error type intentionally remains compact
        /// and independent of extension internals.
        actual: CoreNodeKind,
    },

    /// The declaration directly contains its own node ID.
    SelfReference {
        /// The invalid child ID.
        id: NodeId,
    },

    /// The same direct child ID occurred more than once.
    DuplicateDeclaration {
        /// The duplicated child ID.
        id: NodeId,
    },

    /// An insertion index was outside the current child sequence.
    IndexOutOfBounds {
        /// Requested insertion index.
        index: usize,

        /// Current sequence length.
        len: usize,
    },
}

impl fmt::Display for ExternalDeclarationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySource => {
                formatter.write_str("external declaration source cannot be empty")
            }

            Self::NulInSource => {
                formatter.write_str(
                    "external declaration source cannot contain a NUL character",
                )
            }

            Self::ZeroNodeId => {
                formatter.write_str("external declaration cannot use a zero NodeId")
            }

            Self::ZeroChildNodeId => {
                formatter.write_str(
                    "external declaration cannot contain a zero child NodeId",
                )
            }

            Self::InvalidNodeKind { expected, actual } => {
                write!(
                    formatter,
                    "invalid external declaration node kind: expected \
                     {expected:?}, found {actual:?}"
                )
            }

            Self::SelfReference { id } => {
                write!(
                    formatter,
                    "external declaration cannot directly contain itself: {id:?}"
                )
            }

            Self::DuplicateDeclaration { id } => {
                write!(
                    formatter,
                    "external declaration contains duplicate child NodeId: {id:?}"
                )
            }

            Self::IndexOutOfBounds { index, len } => {
                write!(
                    formatter,
                    "external declaration insertion index {index} \
                     exceeds child count {len}"
                )
            }
        }
    }
}

impl std::error::Error for ExternalDeclarationError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_node_id(value: u64) -> NodeId {
        NodeId::new(value)
    }

    fn test_span() -> Span {
        Span::default()
    }

    fn test_metadata() -> NodeMetadata {
        NodeMetadata::default()
    }

    #[test]
    fn creates_named_external_declaration() {
        let declaration = ExternalDeclaration::empty(
            test_node_id(1),
            test_span(),
            test_metadata(),
            "example",
        )
        .expect("valid external declaration");

        assert_eq!(
            declaration.kind().as_core(),
            Some(CoreNodeKind::ExternalDeclaration)
        );
        assert_eq!(declaration.source(), Some("example"));
        assert!(declaration.is_empty());
    }

    #[test]
    fn anonymous_external_declaration_is_valid() {
        let declaration = ExternalDeclaration::anonymous(
            test_node_id(1),
            test_span(),
            test_metadata(),
            Vec::new(),
        )
        .expect("anonymous external declaration should be valid");

        assert_eq!(declaration.source(), None);
        assert!(declaration.is_empty());
    }

    #[test]
    fn source_cannot_be_empty() {
        let result = ExternalDeclaration::empty(
            test_node_id(1),
            test_span(),
            test_metadata(),
            "",
        );

        assert_eq!(
            result,
            Err(ExternalDeclarationError::EmptySource)
        );
    }

    #[test]
    fn source_cannot_contain_nul() {
        let result = ExternalDeclaration::empty(
            test_node_id(1),
            test_span(),
            test_metadata(),
            "lib\0name",
        );

        assert_eq!(
            result,
            Err(ExternalDeclarationError::NulInSource)
        );
    }

    #[test]
    fn child_order_is_preserved() {
        let mut declaration = ExternalDeclaration::empty(
            test_node_id(1),
            test_span(),
            test_metadata(),
            "example",
        )
        .expect("valid external declaration");

        declaration
            .push_declaration(test_node_id(2))
            .expect("first child should be accepted");

        declaration
            .push_declaration(test_node_id(3))
            .expect("second child should be accepted");

        declaration
            .push_declaration(test_node_id(4))
            .expect("third child should be accepted");

        assert_eq!(
            declaration.declarations(),
            &[
                test_node_id(2),
                test_node_id(3),
                test_node_id(4),
            ]
        );
    }

    #[test]
    fn duplicate_child_is_rejected() {
        let mut declaration = ExternalDeclaration::empty(
            test_node_id(1),
            test_span(),
            test_metadata(),
            "example",
        )
        .expect("valid external declaration");

        declaration
            .push_declaration(test_node_id(2))
            .expect("first child should be accepted");

        assert_eq!(
            declaration.push_declaration(test_node_id(2)),
            Err(ExternalDeclarationError::DuplicateDeclaration {
                id: test_node_id(2),
            })
        );
    }

    #[test]
    fn self_reference_is_rejected() {
        let mut declaration = ExternalDeclaration::empty(
            test_node_id(1),
            test_span(),
            test_metadata(),
            "example",
        )
        .expect("valid external declaration");

        assert_eq!(
            declaration.push_declaration(test_node_id(1)),
            Err(ExternalDeclarationError::SelfReference {
                id: test_node_id(1),
            })
        );
    }

    #[test]
    fn insertion_preserves_requested_order() {
        let mut declaration = ExternalDeclaration::empty(
            test_node_id(1),
            test_span(),
            test_metadata(),
            "example",
        )
        .expect("valid external declaration");

        declaration
            .push_declaration(test_node_id(2))
            .expect("first child should be accepted");

        declaration
            .push_declaration(test_node_id(4))
            .expect("second child should be accepted");

        declaration
            .insert_declaration(1, test_node_id(3))
            .expect("middle insertion should succeed");

        assert_eq!(
            declaration.declarations(),
            &[
                test_node_id(2),
                test_node_id(3),
                test_node_id(4),
            ]
        );
    }

    #[test]
    fn removal_is_safe_for_invalid_index() {
        let mut declaration = ExternalDeclaration::empty(
            test_node_id(1),
            test_span(),
            test_metadata(),
            "example",
        )
        .expect("valid external declaration");

        assert_eq!(declaration.remove_declaration(0), None);
    }

    #[test]
    fn serialization_round_trip_preserves_structure() {
        let mut declaration = ExternalDeclaration::empty(
            test_node_id(1),
            test_span(),
            test_metadata(),
            "example",
        )
        .expect("valid external declaration");

        declaration
            .push_declaration(test_node_id(2))
            .expect("child should be accepted");

        declaration
            .push_declaration(test_node_id(3))
            .expect("child should be accepted");

        let encoded =
            serde_json::to_string(&declaration).expect("serialization should succeed");

        let decoded: ExternalDeclaration =
            serde_json::from_str(&encoded).expect("deserialization should succeed");

        assert_eq!(decoded, declaration);
    }
}