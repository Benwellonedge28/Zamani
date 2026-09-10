//! # Zamani Frontend AST — Constant Declaration
//!
//! Canonical source-level representation of a Zamani constant declaration.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! lexer
//!      │
//!      ▼
//! parser
//!      │
//!      ▼
//! ┌───────────────────────────────┐
//! │ Native Zamani AST             │
//! │                               │
//! │ Constant                      │
//! │   ├── Node                    │
//! │   ├── name                    │
//! │   ├── type annotation         │
//! │   ├── initializer             │
//! │   ├── visibility              │
//! │   ├── generics                │
//! │   ├── attributes              │
//! │   ├── annotations             │
//! │   ├── effects                 │
//! │   ├── capabilities            │
//! │   └── modifiers               │
//! └───────────────┬───────────────┘
//!                 │
//!                 ▼
//!       structural validation
//!                 │
//!                 ▼
//!          semantic analysis
//!                 │
//!                 ▼
//!           semantic model
//!                 │
//!                 ▼
//!                ZUIR
//!                 │
//!        ┌────────┼─────────┐
//!        ▼        ▼         ▼
//!    classical  quantum    HDL
//!       IR        IR        IR
//! ```
//!
//! ## Purpose
//!
//! [`Constant`] represents the **source-language declaration** of an immutable
//! named value.
//!
//! This type deliberately represents source structure rather than evaluating
//! the constant or determining how it will eventually be represented.
//!
//! Consequently, this file does not perform:
//!
//! - constant evaluation;
//! - type inference;
//! - type checking;
//! - name resolution;
//! - generic substitution;
//! - resource allocation;
//! - quantum compilation;
//! - quantum gate decomposition;
//! - qubit allocation;
//! - hardware mapping;
//! - routing;
//! - scheduling;
//! - calibration;
//! - error correction;
//! - resilience;
//! - backend selection;
//! - target selection;
//! - optimization;
//! - ZUIR construction.
//!
//! Those responsibilities belong to later compiler phases.
//!
//! ## POCO-REAF
//!
//! A constant declaration must remain independent of the machine on which its
//! containing program eventually executes.
//!
//! Nothing in this representation assumes:
//!
//! - a particular CPU;
//! - a particular GPU;
//! - a particular QPU;
//! - a particular number of qubits;
//! - a particular register size;
//! - a particular address width;
//! - a particular vendor;
//! - a particular instruction set;
//! - a particular quantum gate set;
//! - a particular topology;
//! - a particular runtime;
//! - a particular backend.
//!
//! A constant can therefore participate in a program that is compiled once and
//! subsequently realized at different scales, subject to the capabilities and
//! resources available to the downstream compiler.
//!
//! ## Source structure versus semantic meaning
//!
//! The AST preserves:
//!
//! ```text
//! const NAME [: TYPE] = VALUE
//! ```
//!
//! as source structure.
//!
//! The semantic layer determines whether:
//!
//! - `TYPE` is valid;
//! - `VALUE` has the required type;
//! - `VALUE` is actually compile-time evaluable;
//! - the declaration is visible in a particular scope;
//! - the initializer is legal;
//! - generic arguments are valid;
//! - effects/capabilities are satisfied.
//!
//! The AST must not answer those questions.
//!
//! ## Child-node ownership
//!
//! Child AST constructs are represented by [`NodeId`] rather than being
//! embedded directly in this declaration.
//!
//! This is intentional.
//!
//! It allows the canonical AST graph/store to choose its storage strategy
//! independently of this declaration:
//!
//! - indexed storage;
//! - arena storage;
//! - immutable storage;
//! - persistent structures;
//! - incremental storage;
//! - compact graph storage;
//! - generated-node stores;
//! - other future storage strategies.
//!
//! Every referenced child must have exactly one authoritative representation in
//! the enclosing AST graph.
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational native-AST infrastructure:
//!
//! - [`super::super::metadata::NodeMetadata`];
//! - [`super::super::node::AstNode`];
//! - [`super::super::node::Node`];
//! - [`super::super::node_id::NodeId`];
//! - [`super::super::node_kind::{CoreNodeKind, NodeKind}`];
//! - [`super::super::source::Span`];
//! - Rust standard-library facilities;
//! - `serde` serialization traits.
//!
//! This module must never depend on:
//!
//! - semantic analysis;
//! - compiler orchestration;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - quantum optimization;
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
//! - vendor SDKs.
//!
//! ## Integration contract
//!
//! ### Parser
//!
//! The parser must:
//!
//! 1. allocate a unique [`NodeId`];
//! 2. create the constant's source [`Span`];
//! 3. construct the constant name;
//! 4. construct child AST nodes for its optional type and required
//!    initializer;
//! 5. store those children in the canonical AST graph;
//! 6. reference those children from [`Constant`];
//! 7. preserve source ordering in the containing program/module;
//! 8. preserve source-level metadata;
//! 9. perform only syntactic/parser-level checks.
//!
//! The parser must not evaluate the initializer or resolve its type.
//!
//! ### Structural validation
//!
//! [`Constant::validate_structure`] checks only information available inside
//! this declaration itself.
//!
//! It verifies:
//!
//! - the declaration has a non-empty source name;
//! - the declaration is classified as `zamani:constant`;
//! - the initializer reference is present;
//! - collection structure is internally representable.
//!
//! Cross-node validation belongs to the AST validation layer because this file
//! intentionally does not own the complete AST graph.
//!
//! ### Semantic analysis
//!
//! Semantic analysis consumes the declaration and resolves:
//!
//! - the declaration's symbol;
//! - its scope;
//! - visibility;
//! - its declared/inferred type;
//! - constant-evaluation rules;
//! - generic arguments;
//! - effects;
//! - capabilities;
//! - resource semantics;
//! - domain semantics.
//!
//! None of those resolved values are stored in [`Constant`].
//!
//! ### ZUIR
//!
//! A constant does not lower directly from the AST into target instructions.
//!
//! The intended path is:
//!
//! ```text
//! Constant AST
//!      │
//!      ▼
//! semantic analysis
//!      │
//!      ▼
//! semantic constant/value
//!      │
//!      ▼
//! ZUIR constant/value representation
//! ```
//!
//! Whether a constant is folded, materialized, propagated, embedded, or
//! otherwise represented is a downstream compiler decision.
//!
//! ## Quantum integration
//!
//! This declaration intentionally contains no quantum-specific fields.
//!
//! A constant may nevertheless participate in quantum programs.
//!
//! For example, a quantum algorithm may use source constants for:
//!
//! - iteration counts;
//! - symbolic dimensions;
//! - angles;
//! - algorithm parameters;
//! - thresholds;
//! - compile-time configuration;
//! - generic arguments.
//!
//! Whether such a value can be represented as a quantum-domain parameter is
//! determined downstream.
//!
//! There must be no special-case field such as:
//!
//! ```text
//! qubit_count
//! quantum_angle
//! quantum_gate
//! backend_constant
//! ```
//!
//! in this type.
//!
//! ## Scalability
//!
//! This type contains no fixed limits on:
//!
//! - constants per program;
//! - name length beyond configurable compiler-input policy;
//! - generic parameter count;
//! - attribute count;
//! - annotation count;
//! - effect count;
//! - capability count;
//! - modifier count.
//!
//! Collections use dynamically sized vectors.
//!
//! A compiler may impose configurable operational limits elsewhere to defend
//! against hostile input or finite machine resources. Such limits must not be
//! encoded as language semantics in this type.
//!
//! ## Determinism
//!
//! The representation contains no unordered collections.
//!
//! Vector ordering is preserved exactly as supplied by the parser/builder.
//!
//! No value depends on:
//!
//! - wall-clock time;
//! - randomness;
//! - memory addresses;
//! - pointer identity;
//! - process-global mutable state;
//! - thread scheduling;
//! - hash-map iteration order.
//!
//! Therefore serialization and structural comparison can be deterministic when
//! the enclosing AST construction is deterministic.
//!
//! ## Serialization
//!
//! `Constant` derives `Serialize` and `Deserialize`.
//!
//! Serialization preserves source-level information:
//!
//! - node identity;
//! - node kind;
//! - source span;
//! - metadata;
//! - name;
//! - child references;
//! - declaration modifiers;
//! - declaration annotations;
//! - effects;
//! - capabilities;
//! - generic references;
//! - visibility reference.
//!
//! It must never contain:
//!
//! - pointers;
//! - memory addresses;
//! - resolved symbols;
//! - evaluated runtime values;
//! - backend handles;
//! - hardware identifiers;
//! - scheduler state;
//! - QIR/LLVM/MLIR values.
//!
//! The enclosing AST serialization layer owns the overall serialization schema
//! version.
//!
//! ## Source mapping
//!
//! The [`Node`] embedded in this declaration owns the declaration's source
//! span.
//!
//! Child nodes referenced through [`NodeId`] own their own source spans.
//!
//! This permits diagnostics to distinguish:
//!
//! ```text
//! whole declaration span
//! name span
//! type span
//! initializer span
//! attribute span
//! annotation span
//! ```
//!
//! without forcing `Constant` to duplicate source-location infrastructure.
//!
//! ## Visitor integration
//!
//! Visitor infrastructure should treat this type as one AST node and then
//! traverse [`Constant::child_node_ids`] to reach referenced child nodes.
//!
//! This file intentionally does not depend on the visitor implementation.
//!
//! This keeps dependency direction one-way:
//!
//! ```text
//! Constant
//!    │
//!    ▼
//! visitors / traversal
//! ```
//!
//! rather than:
//!
//! ```text
//! Constant → visitor implementation
//! ```
//!
//! ## Error model
//!
//! Errors in this module are limited to local structural construction rules.
//!
//! Semantic failures are deliberately not represented here.
//!
//! In particular, this file does not return errors for things such as:
//!
//! - "type is invalid";
//! - "initializer is not constant";
//! - "name is unresolved";
//! - "capability unavailable";
//! - "hardware lacks resources".
//!
//! Those errors belong to later phases.
//!
//! ## Security
//!
//! This type is intended to receive data originating from untrusted source
//! input.
//!
//! It therefore:
//!
//! - performs no unchecked indexing;
//! - performs no pointer dereferencing;
//! - performs no recursive traversal;
//! - performs no evaluation;
//! - performs no arbitrary code execution;
//! - performs no filesystem access;
//! - performs no network access;
//! - uses no global mutable state;
//! - uses no `unsafe` code.
//!
//! Large collections remain subject to allocation failure and to configurable
//! compiler resource policies outside the AST node itself.
//!
//! ## Thread safety
//!
//! `Constant` contains ordinary owned/value-based fields. Its `Send` and
//! `Sync` properties therefore follow those of its constituent types.
//!
//! Immutable ASTs may be consumed concurrently by later compiler phases when
//! the enclosing AST store supports concurrent immutable access.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1.
//!
//! This file uses stable Rust APIs only.
//!
//! No `unsafe` code is used.

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// In-memory schema version of the constant-declaration contract.
///
/// This is deliberately independent of:
///
/// - Zamani language version;
/// - compiler version;
/// - AST serialization version;
/// - extension versions.
pub const CONSTANT_SCHEMA_VERSION: u16 = 1;

/// A source-level Zamani constant declaration.
///
/// A constant has one authoritative AST identity represented by [`Node`].
/// References to its child syntax are represented by [`NodeId`].
///
/// # Required source structure
///
/// A constant requires:
///
/// - a non-empty name;
/// - an initializer expression.
///
/// Its type annotation is optional because Zamani may support inferred
/// constant types.
///
/// The declaration can additionally carry arbitrary source-level metadata,
/// annotations, effects, capabilities, modifiers, generic references and a
/// visibility reference without making any of those concepts hardware-specific.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Constant {
    /// Common AST identity, classification, source span and metadata.
    node: Node,

    /// Source spelling of the constant's declared name.
    ///
    /// Name resolution is intentionally not performed here.
    name: String,

    /// Optional explicit type-expression node.
    ///
    /// `None` means the source declaration does not provide an explicit type
    /// annotation. Semantic analysis may infer one.
    type_annotation: Option<NodeId>,

    /// Required initializer expression node.
    ///
    /// This is a source expression reference, not an already-evaluated value.
    initializer: NodeId,

    /// Optional reference to the canonical source-level visibility construct.
    ///
    /// The AST core treats this as an opaque node reference so visibility can
    /// evolve without coupling `Constant` to a closed visibility enum.
    ///
    /// The referenced node is responsible for representing its own source
    /// syntax. Semantic analysis interprets its meaning.
    visibility: Option<NodeId>,

    /// Generic parameter/argument-related child nodes associated with the
    /// declaration.
    ///
    /// The AST does not determine whether a particular generic relationship is
    /// semantically valid.
    generics: Vec<NodeId>,

    /// Source-level modifier nodes.
    ///
    /// This intentionally uses node references rather than a closed enum so
    /// future language constructs can be introduced without changing this
    /// declaration's representation.
    modifiers: Vec<NodeId>,

    /// Source-level attributes attached to the declaration.
    attributes: Vec<NodeId>,

    /// Source-level annotations attached to the declaration.
    annotations: Vec<NodeId>,

    /// Source-level effect declarations/annotations.
    effects: Vec<NodeId>,

    /// Source-level capability requirements/annotations.
    capabilities: Vec<NodeId>,
}

/// Errors produced while constructing or locally validating a [`Constant`].
///
/// This error type intentionally contains only source-structure failures.
/// Semantic/type/hardware errors belong to later compiler phases.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstantError {
    /// A constant must have a non-empty source name.
    EmptyName,

    /// The supplied node kind was not the native constant kind.
    InvalidNodeKind {
        /// Actual node kind supplied by the caller.
        actual: NodeKind,
    },

    /// A required initializer node was not supplied.
    MissingInitializer,
}

impl std::fmt::Display for ConstantError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName => {
                write!(formatter, "constant declaration name must not be empty")
            }
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "expected native constant node kind, found {actual}"
                )
            }
            Self::MissingInitializer => {
                write!(formatter, "constant declaration requires an initializer")
            }
        }
    }
}

impl std::error::Error for ConstantError {}

impl Constant {
    /// Creates a new constant declaration.
    ///
    /// # Arguments
    ///
    /// * `id` — unique AST identity for this declaration.
    /// * `span` — complete source span of the declaration.
    /// * `metadata` — source-level metadata.
    /// * `name` — source spelling of the declared name.
    /// * `type_annotation` — optional type-expression node.
    /// * `initializer` — required initializer expression node.
    ///
    /// The constructor creates the canonical `zamani:constant` node kind.
    ///
    /// # Errors
    ///
    /// Returns [`ConstantError::EmptyName`] if `name` is empty.
    ///
    /// This function deliberately does not validate identifier grammar because
    /// lexical/syntactic identifier rules belong to the lexer/parser.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        name: impl Into<String>,
        type_annotation: Option<NodeId>,
        initializer: NodeId,
    ) -> Result<Self, ConstantError> {
        let name = name.into();

        if name.is_empty() {
            return Err(ConstantError::EmptyName);
        }

        Ok(Self {
            node: Node::new(
                id,
                NodeKind::core(CoreNodeKind::Constant),
                span,
                metadata,
            ),
            name,
            type_annotation,
            initializer,
            visibility: None,
            generics: Vec::new(),
            modifiers: Vec::new(),
            attributes: Vec::new(),
            annotations: Vec::new(),
            effects: Vec::new(),
            capabilities: Vec::new(),
        })
    }

    /// Creates a constant using default metadata.
    ///
    /// The source span and node identity remain explicit inputs so the
    /// constructor never silently manufactures source information.
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        name: impl Into<String>,
        type_annotation: Option<NodeId>,
        initializer: NodeId,
    ) -> Result<Self, ConstantError> {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            name,
            type_annotation,
            initializer,
        )
    }

    /// Returns the schema version of this declaration contract.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        CONSTANT_SCHEMA_VERSION
    }

    /// Returns the constant's source name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Replaces the source name.
    ///
    /// This performs only the local non-empty-name invariant. Lexical
    /// identifier validity remains the parser's responsibility.
    pub fn set_name(
        &mut self,
        name: impl Into<String>,
    ) -> Result<(), ConstantError> {
        let name = name.into();

        if name.is_empty() {
            return Err(ConstantError::EmptyName);
        }

        self.name = name;
        Ok(())
    }

    /// Returns the optional explicit type-expression node.
    #[inline]
    #[must_use]
    pub fn type_annotation(&self) -> Option<NodeId> {
        self.type_annotation
    }

    /// Sets or replaces the explicit type-expression node.
    #[inline]
    pub fn set_type_annotation(&mut self, type_annotation: Option<NodeId>) {
        self.type_annotation = type_annotation;
    }

    /// Returns the initializer expression node.
    #[inline]
    #[must_use]
    pub fn initializer(&self) -> NodeId {
        self.initializer
    }

    /// Replaces the initializer expression node.
    ///
    /// This method does not evaluate or semantically validate the expression.
    #[inline]
    pub fn set_initializer(&mut self, initializer: NodeId) {
        self.initializer = initializer;
    }

    /// Returns the optional visibility node.
    #[inline]
    #[must_use]
    pub fn visibility(&self) -> Option<NodeId> {
        self.visibility
    }

    /// Sets or clears the source-level visibility node.
    #[inline]
    pub fn set_visibility(&mut self, visibility: Option<NodeId>) {
        self.visibility = visibility;
    }

    /// Returns immutable access to generic child references.
    #[inline]
    #[must_use]
    pub fn generics(&self) -> &[NodeId] {
        &self.generics
    }

    /// Appends a generic child reference while preserving source order.
    #[inline]
    pub fn push_generic(&mut self, node_id: NodeId) {
        self.generics.push(node_id);
    }

    /// Removes all generic references.
    ///
    /// This operation is intentionally explicit rather than exposing the
    /// underlying vector for arbitrary mutation.
    #[inline]
    pub fn clear_generics(&mut self) {
        self.generics.clear();
    }

    /// Returns immutable access to source-level modifier references.
    #[inline]
    #[must_use]
    pub fn modifiers(&self) -> &[NodeId] {
        &self.modifiers
    }

    /// Appends a source-level modifier.
    #[inline]
    pub fn push_modifier(&mut self, node_id: NodeId) {
        self.modifiers.push(node_id);
    }

    /// Removes all modifiers.
    #[inline]
    pub fn clear_modifiers(&mut self) {
        self.modifiers.clear();
    }

    /// Returns immutable access to declaration attributes.
    #[inline]
    #[must_use]
    pub fn attributes(&self) -> &[NodeId] {
        &self.attributes
    }

    /// Appends an attribute while preserving source order.
    #[inline]
    pub fn push_attribute(&mut self, node_id: NodeId) {
        self.attributes.push(node_id);
    }

    /// Removes all attributes.
    #[inline]
    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    /// Returns immutable access to declaration annotations.
    #[inline]
    #[must_use]
    pub fn annotations(&self) -> &[NodeId] {
        &self.annotations
    }

    /// Appends an annotation while preserving source order.
    #[inline]
    pub fn push_annotation(&mut self, node_id: NodeId) {
        self.annotations.push(node_id);
    }

    /// Removes all annotations.
    #[inline]
    pub fn clear_annotations(&mut self) {
        self.annotations.clear();
    }

    /// Returns immutable access to source-level effect references.
    #[inline]
    #[must_use]
    pub fn effects(&self) -> &[NodeId] {
        &self.effects
    }

    /// Appends an effect reference.
    #[inline]
    pub fn push_effect(&mut self, node_id: NodeId) {
        self.effects.push(node_id);
    }

    /// Removes all effect references.
    #[inline]
    pub fn clear_effects(&mut self) {
        self.effects.clear();
    }

    /// Returns immutable access to source-level capability references.
    #[inline]
    #[must_use]
    pub fn capabilities(&self) -> &[NodeId] {
        &self.capabilities
    }

    /// Appends a capability reference.
    #[inline]
    pub fn push_capability(&mut self, node_id: NodeId) {
        self.capabilities.push(node_id);
    }

    /// Removes all capability references.
    #[inline]
    pub fn clear_capabilities(&mut self) {
        self.capabilities.clear();
    }

    /// Returns all direct child references in deterministic source-structure
    /// order.
    ///
    /// The order is:
    ///
    /// 1. type annotation, when present;
    /// 2. initializer;
    /// 3. visibility;
    /// 4. generic references;
    /// 5. modifiers;
    /// 6. attributes;
    /// 7. annotations;
    /// 8. effects;
    /// 9. capabilities.
    ///
    /// The constant's own [`NodeId`] is deliberately not returned.
    ///
    /// This method performs no recursive traversal.
    pub fn child_node_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.type_annotation
            .into_iter()
            .chain(std::iter::once(self.initializer))
            .chain(self.visibility)
            .chain(self.generics.iter().copied())
            .chain(self.modifiers.iter().copied())
            .chain(self.attributes.iter().copied())
            .chain(self.annotations.iter().copied())
            .chain(self.effects.iter().copied())
            .chain(self.capabilities.iter().copied())
    }

    /// Returns the number of direct child references.
    ///
    /// This is a structural convenience and does not recursively inspect the
    /// referenced nodes.
    #[inline]
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.type_annotation.is_some() as usize
            + 1
            + self.visibility.is_some() as usize
            + self.generics.len()
            + self.modifiers.len()
            + self.attributes.len()
            + self.annotations.len()
            + self.effects.len()
            + self.capabilities.len()
    }

    /// Performs local structural validation.
    ///
    /// This function intentionally does not inspect referenced nodes because
    /// the complete AST graph/store is outside this type's ownership.
    ///
    /// Cross-node validation must be performed by the AST validation layer.
    pub fn validate_structure(&self) -> Result<(), ConstantError> {
        if self.name.is_empty() {
            return Err(ConstantError::EmptyName);
        }

        match self.node.kind() {
            NodeKind::Core(CoreNodeKind::Constant) => Ok(()),
            actual => Err(ConstantError::InvalidNodeKind {
                actual: actual.clone(),
            }),
        }
    }

    /// Returns `true` when this declaration has an explicit type annotation.
    #[inline]
    #[must_use]
    pub fn has_explicit_type(&self) -> bool {
        self.type_annotation.is_some()
    }

    /// Returns `true` when the declaration has source-level visibility
    /// information attached.
    #[inline]
    #[must_use]
    pub fn has_visibility(&self) -> bool {
        self.visibility.is_some()
    }

    /// Returns the stable qualified node-kind identifier.
    ///
    /// For a native constant this is:
    ///
    /// ```text
    /// zamani:constant
    /// ```
    #[must_use]
    pub fn qualified_kind_name(&self) -> String {
        self.node.kind().qualified_name()
    }
}

impl AstNode for Constant {
    /// Returns the common canonical AST node.
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common canonical AST node.
    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_id(value: u64) -> NodeId {
        NodeId::new(value).expect("test node ID must be non-zero")
    }

    fn test_span() -> Span {
        Span::default()
    }

    #[test]
    fn constructs_constant_with_required_initializer() {
        let constant = Constant::without_metadata(
            test_id(1),
            test_span(),
            "ANSWER",
            None,
            test_id(2),
        )
        .expect("constant should be structurally constructible");

        assert_eq!(constant.name(), "ANSWER");
        assert_eq!(constant.initializer(), test_id(2));
        assert!(!constant.has_explicit_type());
        assert_eq!(
            constant.kind(),
            &NodeKind::core(CoreNodeKind::Constant)
        );
    }

    #[test]
    fn rejects_empty_name() {
        let result = Constant::without_metadata(
            test_id(1),
            test_span(),
            "",
            None,
            test_id(2),
        );

        assert_eq!(result, Err(ConstantError::EmptyName));
    }

    #[test]
    fn supports_optional_type_annotation() {
        let constant = Constant::without_metadata(
            test_id(1),
            test_span(),
            "ANGLE",
            Some(test_id(2)),
            test_id(3),
        )
        .expect("constant should be constructible");

        assert_eq!(constant.type_annotation(), Some(test_id(2)));
        assert!(constant.has_explicit_type());
    }

    #[test]
    fn supports_unbounded_source_ordered_collections() {
        let mut constant = Constant::without_metadata(
            test_id(1),
            test_span(),
            "VALUE",
            None,
            test_id(2),
        )
        .expect("constant should be constructible");

        for value in 3..=102 {
            constant.push_attribute(test_id(value));
        }

        assert_eq!(constant.attributes().len(), 100);
        assert_eq!(constant.attributes().first(), Some(&test_id(3)));
        assert_eq!(constant.attributes().last(), Some(&test_id(102)));
    }

    #[test]
    fn child_order_is_deterministic() {
        let mut constant = Constant::without_metadata(
            test_id(1),
            test_span(),
            "VALUE",
            Some(test_id(2)),
            test_id(3),
        )
        .expect("constant should be constructible");

        constant.set_visibility(Some(test_id(4)));
        constant.push_generic(test_id(5));
        constant.push_modifier(test_id(6));
        constant.push_attribute(test_id(7));
        constant.push_annotation(test_id(8));
        constant.push_effect(test_id(9));
        constant.push_capability(test_id(10));

        let children: Vec<NodeId> = constant.child_node_ids().collect();

        assert_eq!(
            children,
            vec![
                test_id(2),
                test_id(3),
                test_id(4),
                test_id(5),
                test_id(6),
                test_id(7),
                test_id(8),
                test_id(9),
                test_id(10),
            ]
        );
    }

    #[test]
    fn structural_validation_accepts_valid_constant() {
        let constant = Constant::without_metadata(
            test_id(1),
            test_span(),
            "VALUE",
            None,
            test_id(2),
        )
        .expect("constant should be constructible");

        assert!(constant.validate_structure().is_ok());
    }

    #[test]
    fn replacing_name_preserves_structure() {
        let mut constant = Constant::without_metadata(
            test_id(1),
            test_span(),
            "OLD",
            None,
            test_id(2),
        )
        .expect("constant should be constructible");

        constant
            .set_name("NEW")
            .expect("non-empty replacement name");

        assert_eq!(constant.name(), "NEW");
        assert_eq!(constant.initializer(), test_id(2));
        assert_eq!(
            constant.kind(),
            &NodeKind::core(CoreNodeKind::Constant)
        );
    }

    #[test]
    fn replacing_name_rejects_empty_value_without_corrupting_node() {
        let mut constant = Constant::without_metadata(
            test_id(1),
            test_span(),
            "VALUE",
            None,
            test_id(2),
        )
        .expect("constant should be constructible");

        let result = constant.set_name("");

        assert_eq!(result, Err(ConstantError::EmptyName));
        assert_eq!(constant.name(), "VALUE");
    }

    #[test]
    fn supports_source_level_visibility_reference() {
        let mut constant = Constant::without_metadata(
            test_id(1),
            test_span(),
            "VALUE",
            None,
            test_id(2),
        )
        .expect("constant should be constructible");

        assert!(!constant.has_visibility());

        constant.set_visibility(Some(test_id(3)));

        assert!(constant.has_visibility());
        assert_eq!(constant.visibility(), Some(test_id(3)));
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(Constant::schema_version(), CONSTANT_SCHEMA_VERSION);
    }

    #[test]
    fn cloning_preserves_ast_identity() {
        let constant = Constant::without_metadata(
            test_id(1),
            test_span(),
            "VALUE",
            None,
            test_id(2),
        )
        .expect("constant should be constructible");

        let cloned = constant.clone();

        assert_eq!(constant, cloned);
        assert_eq!(constant.id(), cloned.id());
    }
}