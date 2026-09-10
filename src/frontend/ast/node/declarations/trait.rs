//! # Zamani Frontend AST — Trait Declaration
//!
//! Canonical source-level representation of a Zamani trait declaration.
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
//! ┌────────────────────────────────────────────┐
//! │          Native Zamani AST                 │
//! │                                            │
//! │ Trait                                      │
//! │   ├── Node                                 │
//! │   ├── name                                 │
//! │   ├── visibility                           │
//! │   ├── generic parameters                   │
//! │   ├── supertraits                          │
//! │   ├── members                              │
//! │   ├── modifiers                            │
//! │   ├── attributes                           │
//! │   ├── annotations                          │
//! │   ├── effects                              │
//! │   └── capabilities                         │
//! └──────────────────┬─────────────────────────┘
//!                    │
//!                    ▼
//!          structural validation
//!                    │
//!                    ▼
//!             semantic analysis
//!                    │
//!                    ▼
//!              semantic model
//!                    │
//!                    ▼
//!                   ZUIR
//!                    │
//!          ┌─────────┼─────────┐
//!          ▼         ▼         ▼
//!      Classical   Quantum     HDL
//!         IR         IR        IR
//! ```
//!
//! ## Purpose
//!
//! [`Trait`] represents the source-level declaration of an extensible Zamani
//! interface/contract.
//!
//! A trait is deliberately represented as source structure rather than as a
//! resolved semantic object.
//!
//! This file therefore does **not** perform:
//!
//! - name resolution;
//! - type checking;
//! - generic substitution;
//! - trait coherence checking;
//! - method dispatch resolution;
//! - implementation selection;
//! - capability satisfaction;
//! - resource allocation;
//! - quantum compilation;
//! - quantum gate decomposition;
//! - qubit allocation;
//! - physical mapping;
//! - routing;
//! - scheduling;
//! - calibration;
//! - quantum error correction;
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
//! A trait is a source-level abstraction and therefore contains no assumptions
//! about the machine on which an implementation eventually executes.
//!
//! This representation does not encode:
//!
//! - CPU count;
//! - GPU count;
//! - FPGA count;
//! - QPU count;
//! - qubit count;
//! - register width;
//! - memory size;
//! - hardware topology;
//! - gate set;
//! - instruction set;
//! - vendor;
//! - backend;
//! - scheduler;
//! - calibration;
//! - QEC implementation;
//! - runtime state.
//!
//! Consequently, the same trait declaration can participate in programs that
//! target tiny systems, large systems, heterogeneous systems, distributed
//! systems, quantum systems, classical systems, or future computational
//! architectures.
//!
//! The downstream compiler determines how the trait's semantic requirements
//! are realized.
//!
//! ## Domain neutrality
//!
//! Traits are intentionally not quantum-specific.
//!
//! The same construct can describe contracts for:
//!
//! - ordinary classical types;
//! - quantum abstractions;
//! - hybrid computation;
//! - distributed computation;
//! - accelerator abstractions;
//! - AI/ML abstractions;
//! - HDL-facing abstractions;
//! - future computational domains.
//!
//! A quantum extension may define additional source constructs, but this core
//! trait representation must remain independent of any particular quantum
//! technology.
//!
//! ## Child-node ownership
//!
//! Child syntax is represented using [`NodeId`] references.
//!
//! The enclosing AST graph/store owns the canonical child nodes.
//!
//! This provides a single authoritative representation for every AST node and
//! permits the AST store to use:
//!
//! - indexed storage;
//! - arenas;
//! - persistent structures;
//! - immutable stores;
//! - incremental stores;
//! - generated-node stores;
//! - compact graph representations;
//! - future storage strategies.
//!
//! No child node is duplicated inside [`Trait`].
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational native-AST infrastructure:
//!
//! - [`super::super::metadata::NodeMetadata`];
//! - [`super::super::node::{AstNode, Node}`];
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
//! The parser should:
//!
//! 1. allocate a unique [`NodeId`] for the trait declaration;
//! 2. create the declaration [`Span`];
//! 3. construct the source name;
//! 4. construct child nodes for generic parameters;
//! 5. construct child nodes for supertrait references;
//! 6. construct child nodes for trait members;
//! 7. construct optional visibility syntax;
//! 8. construct modifiers, attributes, annotations, effects and capabilities;
//! 9. store all child nodes in the canonical AST graph;
//! 10. preserve source order;
//! 11. construct this [`Trait`] value;
//! 12. defer semantic interpretation to later phases.
//!
//! The parser must not:
//!
//! - resolve supertraits;
//! - determine implementation coherence;
//! - resolve associated types;
//! - select implementations;
//! - determine hardware realization.
//!
//! ### Structural validation
//!
//! [`Trait::validate_structure`] validates only information locally available
//! to this declaration.
//!
//! It verifies:
//!
//! - the name is non-empty;
//! - the node has the native `zamani:trait` kind.
//!
//! It does not dereference child IDs.
//!
//! Cross-node validation belongs to the AST validation layer.
//!
//! ### Semantic analysis
//!
//! Semantic analysis consumes the declaration and resolves:
//!
//! - the trait symbol;
//! - namespace/scope;
//! - visibility;
//! - generic parameters;
//! - supertrait relationships;
//! - associated types;
//! - associated constants;
//! - associated functions/methods;
//! - required implementations;
//! - default implementations;
//! - effects;
//! - capabilities;
//! - resource requirements;
//! - domain semantics.
//!
//! None of those resolved semantic objects are stored here.
//!
//! ### ZUIR
//!
//! A trait does not lower directly to ZUIR.
//!
//! The intended path is:
//!
//! ```text
//! Trait AST
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic trait model
//!     │
//!     ▼
//! ZUIR / domain semantic representation
//! ```
//!
//! Whether a trait is erased, monomorphized, represented as a contract,
//! lowered to dispatch structures, or otherwise transformed is a downstream
//! decision.
//!
//! ## Quantum integration
//!
//! This type deliberately contains no fields such as:
//!
//! ```text
//! qubit_count
//! gate_set
//! hardware
//! topology
//! backend
//! qpu
//! ```
//!
//! A trait may nevertheless describe quantum abstractions.
//!
//! For example, a future semantic layer may use a trait to express a generic
//! computational contract implemented by multiple quantum technologies.
//!
//! The trait AST remains unaware of the physical realization.
//!
//! ## Scalability
//!
//! There is no fixed limit on:
//!
//! - traits per program;
//! - generic parameters;
//! - supertraits;
//! - members;
//! - modifiers;
//! - attributes;
//! - annotations;
//! - effects;
//! - capabilities.
//!
//! All collections are dynamically sized.
//!
//! No language-level machine-size limit is encoded here.
//!
//! Compiler safety limits, if required, must be supplied through configurable
//! compiler policies outside this AST node.
//!
//! ## Determinism
//!
//! Collections preserve source order using [`Vec`].
//!
//! No unordered collection is used.
//!
//! This representation does not depend on:
//!
//! - wall-clock time;
//! - randomness;
//! - memory addresses;
//! - pointer identity;
//! - process-global state;
//! - thread scheduling;
//! - hash-map iteration order.
//!
//! Therefore deterministic AST construction produces deterministic structural
//! ordering.
//!
//! ## Serialization
//!
//! [`Trait`] derives `Serialize` and `Deserialize`.
//!
//! Serialization preserves source-level information:
//!
//! - node identity;
//! - node kind;
//! - source span;
//! - metadata;
//! - source name;
//! - child node references;
//! - source ordering.
//!
//! It must never contain:
//!
//! - pointers;
//! - memory addresses;
//! - resolved symbols;
//! - backend handles;
//! - hardware state;
//! - runtime state;
//! - QIR/LLVM/MLIR values.
//!
//! Overall serialization versioning belongs to the AST serialization layer.
//!
//! ## Source mapping
//!
//! The embedded [`Node`] owns the complete trait declaration span.
//!
//! Every referenced child node owns its own span.
//!
//! This allows diagnostics to distinguish the declaration span from spans of:
//!
//! - the trait name;
//! - generic parameters;
//! - supertraits;
//! - members;
//! - visibility;
//! - modifiers;
//! - attributes;
//! - annotations;
//! - effects;
//! - capabilities.
//!
//! ## Visitor integration
//!
//! Visitor and traversal infrastructure should:
//!
//! 1. visit the trait node;
//! 2. traverse [`Trait::child_node_ids`] in deterministic source order;
//! 3. resolve those IDs through the canonical AST store;
//! 4. visit the referenced child nodes.
//!
//! This file deliberately does not depend on visitor implementations.
//!
//! ## Error model
//!
//! Errors are restricted to local source-structure invariants.
//!
//! Semantic failures are not represented by [`TraitError`].
//!
//! Examples of failures belonging elsewhere:
//!
//! - unresolved supertrait;
//! - cyclic trait semantics;
//! - conflicting associated members;
//! - invalid generic bounds;
//! - unsatisfied capability;
//! - unavailable resource;
//! - invalid quantum realization.
//!
//! ## Security
//!
//! This type may be constructed from untrusted source input.
//!
//! It therefore:
//!
//! - performs no unchecked indexing;
//! - performs no pointer dereferencing;
//! - performs no recursive traversal;
//! - performs no code execution;
//! - performs no filesystem access;
//! - performs no network access;
//! - uses no global mutable state;
//! - uses no `unsafe` code.
//!
//! Child references are not dereferenced by this type.
//!
//! ## Thread safety
//!
//! [`Trait`] contains ordinary owned/value-based fields.
//!
//! Its `Send` and `Sync` properties follow those of its constituent types.
//!
//! Immutable AST stores may be consumed concurrently by later compiler phases
//! when the enclosing storage layer supports concurrent immutable access.
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

// -----------------------------------------------------------------------------
// Imports
// -----------------------------------------------------------------------------

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

// -----------------------------------------------------------------------------
// Schema
// -----------------------------------------------------------------------------

/// In-memory schema version for the native Zamani trait declaration.
///
/// This version is independent from:
///
/// - the Zamani language version;
/// - compiler version;
/// - serialized AST format version;
/// - extension versions.
pub const TRAIT_SCHEMA_VERSION: u16 = 1;

// -----------------------------------------------------------------------------
// AST node
// -----------------------------------------------------------------------------

/// Canonical source-level Zamani trait declaration.
///
/// A [`Trait`] stores source structure only. Semantic resolution is performed
/// later and is not embedded into this node.
///
/// # Structure
///
/// A trait consists of:
///
/// - one source name;
/// - optional visibility;
/// - generic child nodes;
/// - zero or more supertrait references;
/// - zero or more member references;
/// - optional source-level modifiers;
/// - source-level attributes;
/// - source-level annotations;
/// - source-level effects;
/// - source-level capabilities.
///
/// All child syntax is referenced through [`NodeId`].
///
/// # Scalability
///
/// There is no fixed number of members, supertraits, generic parameters, or
/// annotations.
///
/// # Domain independence
///
/// Nothing in this structure identifies a processor, quantum technology,
/// backend, vendor, instruction set, or hardware topology.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Trait {
    /// Common AST identity, kind, source span and metadata.
    node: Node,

    /// Source spelling of the trait name.
    ///
    /// Name resolution is intentionally deferred.
    name: String,

    /// Optional source-level visibility node.
    ///
    /// Visibility semantics are resolved downstream.
    visibility: Option<NodeId>,

    /// Generic parameter/argument-related child nodes.
    ///
    /// Source order is preserved.
    generics: Vec<NodeId>,

    /// References to source-level supertrait expressions.
    ///
    /// These are unresolved child-node references. Semantic analysis determines
    /// their meaning and validates their relationships.
    supertraits: Vec<NodeId>,

    /// References to trait member declarations.
    ///
    /// Members may include functions, associated types, associated constants,
    /// nested declarations, or future language-level constructs.
    ///
    /// The AST does not impose a closed member enumeration.
    members: Vec<NodeId>,

    /// Source-level modifier nodes.
    modifiers: Vec<NodeId>,

    /// Source-level attribute nodes.
    attributes: Vec<NodeId>,

    /// Source-level annotation nodes.
    annotations: Vec<NodeId>,

    /// Source-level effect nodes.
    effects: Vec<NodeId>,

    /// Source-level capability nodes.
    capabilities: Vec<NodeId>,
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Errors produced while constructing or locally validating a [`Trait`].
///
/// These errors intentionally describe only local structural violations.
///
/// Semantic, type-system, capability, resource, domain, and hardware errors
/// belong to later compiler phases.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TraitError {
    /// A trait must have a non-empty source name.
    EmptyName,

    /// The embedded node does not have the native trait node kind.
    InvalidNodeKind {
        /// Actual node kind encountered.
        actual: NodeKind,
    },
}

impl std::fmt::Display for TraitError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName => {
                write!(formatter, "trait declaration name must not be empty")
            }

            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "expected native trait node kind, found {actual}"
                )
            }
        }
    }
}

impl std::error::Error for TraitError {}

// -----------------------------------------------------------------------------
// Constructors
// -----------------------------------------------------------------------------

impl Trait {
    /// Creates a new native Zamani trait declaration.
    ///
    /// The constructor establishes the canonical:
    ///
    /// ```text
    /// zamani:trait
    /// ```
    ///
    /// node kind.
    ///
    /// # Arguments
    ///
    /// * `id` - unique AST identity for the declaration;
    /// * `span` - complete source span;
    /// * `metadata` - source-level metadata;
    /// * `name` - source spelling of the trait name.
    ///
    /// # Errors
    ///
    /// Returns [`TraitError::EmptyName`] if the supplied name is empty.
    ///
    /// Identifier grammar is intentionally not checked here. Lexical and
    /// syntactic identifier validation belongs to the lexer/parser.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        name: impl Into<String>,
    ) -> Result<Self, TraitError> {
        let name = name.into();

        if name.is_empty() {
            return Err(TraitError::EmptyName);
        }

        Ok(Self {
            node: Node::new(
                id,
                NodeKind::core(CoreNodeKind::Trait),
                span,
                metadata,
            ),
            name,
            visibility: None,
            generics: Vec::new(),
            supertraits: Vec::new(),
            members: Vec::new(),
            modifiers: Vec::new(),
            attributes: Vec::new(),
            annotations: Vec::new(),
            effects: Vec::new(),
            capabilities: Vec::new(),
        })
    }

    /// Creates a trait declaration using default node metadata.
    ///
    /// The node identity and source span remain explicit inputs.
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        name: impl Into<String>,
    ) -> Result<Self, TraitError> {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            name,
        )
    }

    /// Returns the schema version of this declaration contract.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        TRAIT_SCHEMA_VERSION
    }
}

// -----------------------------------------------------------------------------
// Common AST access
// -----------------------------------------------------------------------------

impl Trait {
    /// Returns the source-level trait name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Replaces the source-level trait name.
    ///
    /// Only the local non-empty-name invariant is checked here.
    ///
    /// Lexical identifier validity remains the responsibility of the
    /// lexer/parser.
    pub fn set_name(
        &mut self,
        name: impl Into<String>,
    ) -> Result<(), TraitError> {
        let name = name.into();

        if name.is_empty() {
            return Err(TraitError::EmptyName);
        }

        self.name = name;
        Ok(())
    }

    /// Returns the canonical node identity.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the underlying canonical AST node.
    #[inline]
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the underlying canonical AST node.
    ///
    /// Structural transformations should be performed through controlled AST
    /// transformation infrastructure.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the trait's source-level node kind.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the complete declaration source span.
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

    /// Returns the stable qualified node-kind name.
    ///
    /// For a native trait this is:
    ///
    /// ```text
    /// zamani:trait
    /// ```
    #[must_use]
    pub fn qualified_kind_name(&self) -> String {
        self.node.kind().qualified_name()
    }
}

// -----------------------------------------------------------------------------
// Visibility
// -----------------------------------------------------------------------------

impl Trait {
    /// Returns the optional source-level visibility node.
    #[inline]
    #[must_use]
    pub fn visibility(&self) -> Option<NodeId> {
        self.visibility
    }

    /// Sets or clears the source-level visibility node.
    ///
    /// The supplied ID is not dereferenced or semantically interpreted.
    #[inline]
    pub fn set_visibility(&mut self, visibility: Option<NodeId>) {
        self.visibility = visibility;
    }

    /// Returns whether explicit source-level visibility is represented.
    #[inline]
    #[must_use]
    pub fn has_visibility(&self) -> bool {
        self.visibility.is_some()
    }
}

// -----------------------------------------------------------------------------
// Generic parameters
// -----------------------------------------------------------------------------

impl Trait {
    /// Returns immutable access to generic child references.
    #[inline]
    #[must_use]
    pub fn generics(&self) -> &[NodeId] {
        &self.generics
    }

    /// Returns the number of generic child references.
    #[inline]
    #[must_use]
    pub fn generic_count(&self) -> usize {
        self.generics.len()
    }

    /// Returns whether this trait has generic child nodes.
    #[inline]
    #[must_use]
    pub fn is_generic(&self) -> bool {
        !self.generics.is_empty()
    }

    /// Appends a generic child reference while preserving source order.
    #[inline]
    pub fn push_generic(&mut self, node_id: NodeId) {
        self.generics.push(node_id);
    }

    /// Removes all generic child references.
    #[inline]
    pub fn clear_generics(&mut self) {
        self.generics.clear();
    }
}

// -----------------------------------------------------------------------------
// Supertraits
// -----------------------------------------------------------------------------

impl Trait {
    /// Returns immutable access to supertrait child references.
    #[inline]
    #[must_use]
    pub fn supertraits(&self) -> &[NodeId] {
        &self.supertraits
    }

    /// Returns the number of declared supertrait references.
    #[inline]
    #[must_use]
    pub fn supertrait_count(&self) -> usize {
        self.supertraits.len()
    }

    /// Appends a supertrait child reference while preserving source order.
    ///
    /// The reference is not resolved here.
    #[inline]
    pub fn push_supertrait(&mut self, node_id: NodeId) {
        self.supertraits.push(node_id);
    }

    /// Removes all supertrait references.
    #[inline]
    pub fn clear_supertraits(&mut self) {
        self.supertraits.clear();
    }
}

// -----------------------------------------------------------------------------
// Members
// -----------------------------------------------------------------------------

impl Trait {
    /// Returns immutable access to trait member references.
    ///
    /// Members remain generic node references so the trait declaration does not
    /// need to know every future declaration kind.
    #[inline]
    #[must_use]
    pub fn members(&self) -> &[NodeId] {
        &self.members
    }

    /// Returns the number of direct member references.
    #[inline]
    #[must_use]
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Returns whether the trait contains no declared members.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Appends a member reference while preserving source order.
    #[inline]
    pub fn push_member(&mut self, node_id: NodeId) {
        self.members.push(node_id);
    }

    /// Removes all member references.
    #[inline]
    pub fn clear_members(&mut self) {
        self.members.clear();
    }
}

// -----------------------------------------------------------------------------
// Modifiers
// -----------------------------------------------------------------------------

impl Trait {
    /// Returns immutable access to modifier references.
    #[inline]
    #[must_use]
    pub fn modifiers(&self) -> &[NodeId] {
        &self.modifiers
    }

    /// Returns the number of modifier references.
    #[inline]
    #[must_use]
    pub fn modifier_count(&self) -> usize {
        self.modifiers.len()
    }

    /// Appends a modifier reference.
    #[inline]
    pub fn push_modifier(&mut self, node_id: NodeId) {
        self.modifiers.push(node_id);
    }

    /// Removes all modifier references.
    #[inline]
    pub fn clear_modifiers(&mut self) {
        self.modifiers.clear();
    }
}

// -----------------------------------------------------------------------------
// Attributes
// -----------------------------------------------------------------------------

impl Trait {
    /// Returns immutable access to source-level attributes.
    #[inline]
    #[must_use]
    pub fn attributes(&self) -> &[NodeId] {
        &self.attributes
    }

    /// Returns the number of attributes.
    #[inline]
    #[must_use]
    pub fn attribute_count(&self) -> usize {
        self.attributes.len()
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
}

// -----------------------------------------------------------------------------
// Annotations
// -----------------------------------------------------------------------------

impl Trait {
    /// Returns immutable access to source-level annotations.
    #[inline]
    #[must_use]
    pub fn annotations(&self) -> &[NodeId] {
        &self.annotations
    }

    /// Returns the number of annotations.
    #[inline]
    #[must_use]
    pub fn annotation_count(&self) -> usize {
        self.annotations.len()
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
}

// -----------------------------------------------------------------------------
// Effects
// -----------------------------------------------------------------------------

impl Trait {
    /// Returns immutable access to source-level effects.
    #[inline]
    #[must_use]
    pub fn effects(&self) -> &[NodeId] {
        &self.effects
    }

    /// Returns the number of effects.
    #[inline]
    #[must_use]
    pub fn effect_count(&self) -> usize {
        self.effects.len()
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
}

// -----------------------------------------------------------------------------
// Capabilities
// -----------------------------------------------------------------------------

impl Trait {
    /// Returns immutable access to source-level capabilities.
    #[inline]
    #[must_use]
    pub fn capabilities(&self) -> &[NodeId] {
        &self.capabilities
    }

    /// Returns the number of capability references.
    #[inline]
    #[must_use]
    pub fn capability_count(&self) -> usize {
        self.capabilities.len()
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
}

// -----------------------------------------------------------------------------
// Child traversal
// -----------------------------------------------------------------------------

impl Trait {
    /// Returns all direct child node IDs in deterministic source-structure
    /// order.
    ///
    /// The ordering is:
    ///
    /// 1. visibility;
    /// 2. generic parameters/arguments;
    /// 3. supertraits;
    /// 4. members;
    /// 5. modifiers;
    /// 6. attributes;
    /// 7. annotations;
    /// 8. effects;
    /// 9. capabilities.
    ///
    /// The trait's own node ID is never returned.
    ///
    /// This method is deliberately non-recursive.
    pub fn child_node_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.visibility
            .into_iter()
            .chain(self.generics.iter().copied())
            .chain(self.supertraits.iter().copied())
            .chain(self.members.iter().copied())
            .chain(self.modifiers.iter().copied())
            .chain(self.attributes.iter().copied())
            .chain(self.annotations.iter().copied())
            .chain(self.effects.iter().copied())
            .chain(self.capabilities.iter().copied())
    }

    /// Returns the number of direct child references.
    ///
    /// This does not recursively inspect referenced nodes.
    #[inline]
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.visibility.is_some() as usize
            + self.generics.len()
            + self.supertraits.len()
            + self.members.len()
            + self.modifiers.len()
            + self.attributes.len()
            + self.annotations.len()
            + self.effects.len()
            + self.capabilities.len()
    }

    /// Calls `callback` for every direct child reference in deterministic
    /// source-structure order.
    ///
    /// This method does not dereference child IDs and does not recursively
    /// traverse the AST.
    pub fn for_each_child<F>(&self, mut callback: F)
    where
        F: FnMut(NodeId),
    {
        for node_id in self.child_node_ids() {
            callback(node_id);
        }
    }
}

// -----------------------------------------------------------------------------
// Structural validation
// -----------------------------------------------------------------------------

impl Trait {
    /// Performs local structural validation.
    ///
    /// This function intentionally does not dereference child node IDs.
    ///
    /// Cross-node checks belong to the AST validation layer.
    pub fn validate_structure(&self) -> Result<(), TraitError> {
        if self.name.is_empty() {
            return Err(TraitError::EmptyName);
        }

        match self.node.kind() {
            NodeKind::Core(CoreNodeKind::Trait) => Ok(()),
            actual => Err(TraitError::InvalidNodeKind {
                actual: actual.clone(),
            }),
        }
    }

    /// Returns `true` when the declaration satisfies local structural
    /// invariants.
    #[inline]
    #[must_use]
    pub fn is_structurally_valid(&self) -> bool {
        self.validate_structure().is_ok()
    }
}

// -----------------------------------------------------------------------------
// AST integration
// -----------------------------------------------------------------------------

impl AstNode for Trait {
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

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

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
    fn constructs_trait() {
        let trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "Computable",
        )
        .expect("trait should be constructible");

        assert_eq!(trait_decl.name(), "Computable");
        assert_eq!(
            trait_decl.kind(),
            &NodeKind::core(CoreNodeKind::Trait)
        );
    }

    #[test]
    fn rejects_empty_name() {
        let result = Trait::without_metadata(
            test_id(1),
            test_span(),
            "",
        );

        assert_eq!(result, Err(TraitError::EmptyName));
    }

    #[test]
    fn supports_visibility() {
        let mut trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "Computable",
        )
        .expect("trait should be constructible");

        assert!(!trait_decl.has_visibility());

        trait_decl.set_visibility(Some(test_id(2)));

        assert!(trait_decl.has_visibility());
        assert_eq!(
            trait_decl.visibility(),
            Some(test_id(2))
        );
    }

    #[test]
    fn supports_generics() {
        let mut trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "Computable",
        )
        .expect("trait should be constructible");

        trait_decl.push_generic(test_id(2));
        trait_decl.push_generic(test_id(3));

        assert!(trait_decl.is_generic());
        assert_eq!(trait_decl.generic_count(), 2);
        assert_eq!(
            trait_decl.generics(),
            &[test_id(2), test_id(3)]
        );
    }

    #[test]
    fn supports_supertraits() {
        let mut trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "QuantumOperation",
        )
        .expect("trait should be constructible");

        trait_decl.push_supertrait(test_id(2));
        trait_decl.push_supertrait(test_id(3));

        assert_eq!(trait_decl.supertrait_count(), 2);
        assert_eq!(
            trait_decl.supertraits(),
            &[test_id(2), test_id(3)]
        );
    }

    #[test]
    fn supports_members_without_closed_member_type() {
        let mut trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "Computable",
        )
        .expect("trait should be constructible");

        trait_decl.push_member(test_id(2));
        trait_decl.push_member(test_id(3));
        trait_decl.push_member(test_id(4));

        assert_eq!(trait_decl.member_count(), 3);
        assert_eq!(
            trait_decl.members(),
            &[test_id(2), test_id(3), test_id(4)]
        );
    }

    #[test]
    fn supports_source_level_metadata_collections() {
        let mut trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "Computable",
        )
        .expect("trait should be constructible");

        trait_decl.push_modifier(test_id(2));
        trait_decl.push_attribute(test_id(3));
        trait_decl.push_annotation(test_id(4));
        trait_decl.push_effect(test_id(5));
        trait_decl.push_capability(test_id(6));

        assert_eq!(trait_decl.modifier_count(), 1);
        assert_eq!(trait_decl.attribute_count(), 1);
        assert_eq!(trait_decl.annotation_count(), 1);
        assert_eq!(trait_decl.effect_count(), 1);
        assert_eq!(trait_decl.capability_count(), 1);
    }

    #[test]
    fn child_order_is_deterministic() {
        let mut trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "Computable",
        )
        .expect("trait should be constructible");

        trait_decl.set_visibility(Some(test_id(2)));

        trait_decl.push_generic(test_id(3));
        trait_decl.push_generic(test_id(4));

        trait_decl.push_supertrait(test_id(5));

        trait_decl.push_member(test_id(6));
        trait_decl.push_member(test_id(7));

        trait_decl.push_modifier(test_id(8));
        trait_decl.push_attribute(test_id(9));
        trait_decl.push_annotation(test_id(10));
        trait_decl.push_effect(test_id(11));
        trait_decl.push_capability(test_id(12));

        let children: Vec<NodeId> =
            trait_decl.child_node_ids().collect();

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
                test_id(11),
                test_id(12),
            ]
        );
    }

    #[test]
    fn child_count_matches_child_iterator() {
        let mut trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "Computable",
        )
        .expect("trait should be constructible");

        trait_decl.set_visibility(Some(test_id(2)));
        trait_decl.push_generic(test_id(3));
        trait_decl.push_supertrait(test_id(4));
        trait_decl.push_member(test_id(5));
        trait_decl.push_modifier(test_id(6));
        trait_decl.push_attribute(test_id(7));
        trait_decl.push_annotation(test_id(8));
        trait_decl.push_effect(test_id(9));
        trait_decl.push_capability(test_id(10));

        assert_eq!(
            trait_decl.child_count(),
            trait_decl.child_node_ids().count()
        );
    }

    #[test]
    fn supports_large_source_ordered_member_sets() {
        let mut trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "LargeContract",
        )
        .expect("trait should be constructible");

        for value in 2..=1002 {
            trait_decl.push_member(test_id(value));
        }

        assert_eq!(trait_decl.member_count(), 1001);
        assert_eq!(
            trait_decl.members().first(),
            Some(&test_id(2))
        );
        assert_eq!(
            trait_decl.members().last(),
            Some(&test_id(1002))
        );
    }

    #[test]
    fn empty_trait_is_structurally_valid() {
        let trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "Marker",
        )
        .expect("trait should be constructible");

        assert!(trait_decl.is_empty());
        assert!(trait_decl.validate_structure().is_ok());
    }

    #[test]
    fn structural_validation_accepts_valid_trait() {
        let trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "Computable",
        )
        .expect("trait should be constructible");

        assert!(trait_decl.validate_structure().is_ok());
    }

    #[test]
    fn changing_name_preserves_other_structure() {
        let mut trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "OldName",
        )
        .expect("trait should be constructible");

        trait_decl.push_member(test_id(2));

        trait_decl
            .set_name("NewName")
            .expect("non-empty name should be accepted");

        assert_eq!(trait_decl.name(), "NewName");
        assert_eq!(
            trait_decl.members(),
            &[test_id(2)]
        );
        assert_eq!(
            trait_decl.kind(),
            &NodeKind::core(CoreNodeKind::Trait)
        );
    }

    #[test]
    fn failed_name_change_does_not_corrupt_trait() {
        let mut trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "Computable",
        )
        .expect("trait should be constructible");

        let result = trait_decl.set_name("");

        assert_eq!(result, Err(TraitError::EmptyName));
        assert_eq!(trait_decl.name(), "Computable");
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            Trait::schema_version(),
            TRAIT_SCHEMA_VERSION
        );
    }

    #[test]
    fn qualified_kind_name_is_stable() {
        let trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "Computable",
        )
        .expect("trait should be constructible");

        assert_eq!(
            trait_decl.qualified_kind_name(),
            "zamani:trait"
        );
    }

    #[test]
    fn implements_ast_node_contract() {
        let trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "Computable",
        )
        .expect("trait should be constructible");

        let node_id = AstNode::id(&trait_decl);

        assert_eq!(node_id, test_id(1));
        assert_eq!(
            AstNode::kind(&trait_decl),
            &NodeKind::core(CoreNodeKind::Trait)
        );
    }

    #[test]
    fn clone_preserves_ast_identity() {
        let trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "Computable",
        )
        .expect("trait should be constructible");

        let cloned = trait_decl.clone();

        assert_eq!(trait_decl, cloned);
        assert_eq!(trait_decl.id(), cloned.id());
    }

    #[test]
    fn serialization_round_trip_preserves_structure() {
        let mut trait_decl = Trait::without_metadata(
            test_id(1),
            test_span(),
            "Computable",
        )
        .expect("trait should be constructible");

        trait_decl.set_visibility(Some(test_id(2)));
        trait_decl.push_generic(test_id(3));
        trait_decl.push_supertrait(test_id(4));
        trait_decl.push_member(test_id(5));
        trait_decl.push_modifier(test_id(6));
        trait_decl.push_attribute(test_id(7));
        trait_decl.push_annotation(test_id(8));
        trait_decl.push_effect(test_id(9));
        trait_decl.push_capability(test_id(10));

        let serialized =
            serde_json::to_string(&trait_decl)
                .expect("trait should serialize");

        let restored: Trait =
            serde_json::from_str(&serialized)
                .expect("trait should deserialize");

        assert_eq!(trait_decl, restored);
    }
}