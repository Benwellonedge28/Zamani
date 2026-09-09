//! # Zamani Frontend AST — Function Declaration
//!
//! Canonical source-level representation of a Zamani function declaration.
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
//! Native Zamani AST
//!      │
//!      └── Function
//!             │
//!             ├── parameters ───────► AST nodes
//!             ├── generic parameters ► AST nodes
//!             ├── return type ───────► AST node
//!             ├── body ──────────────► AST node
//!             ├── effects ───────────► AST nodes
//!             ├── capabilities ──────► AST nodes
//!             └── contracts ─────────► AST nodes
//!      │
//!      ▼
//! structural validation
//!      │
//!      ▼
//! semantic analysis
//!      │
//!      ▼
//! semantic model
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ▼
//! domain / target lowering
//!      │
//!      ▼
//! available execution resources
//! ```
//!
//! ## Purpose
//!
//! `Function` represents **source-level programmer intent**.
//!
//! It deliberately does not represent:
//!
//! - a machine function;
//! - a CPU function;
//! - a GPU kernel;
//! - a QPU kernel;
//! - a quantum circuit;
//! - a quantum gate;
//! - a physical qubit mapping;
//! - a hardware instruction sequence;
//! - a scheduler task;
//! - a backend job;
//! - a QIR function;
//! - an LLVM function;
//! - an MLIR operation;
//! - a vendor-specific function;
//! - a runtime execution object.
//!
//! The same `Function` can therefore participate in compilation for different
//! computational scales and technologies.
//!
//! ## POCO-REAF
//!
//! The function representation intentionally contains no:
//!
//! - fixed machine size;
//! - fixed CPU count;
//! - fixed GPU count;
//! - fixed QPU count;
//! - fixed qubit count;
//! - fixed register width;
//! - fixed topology;
//! - fixed gate set;
//! - fixed instruction set;
//! - fixed vendor;
//! - fixed backend.
//!
//! A function can therefore express an algorithm once while downstream
//! compilation determines how that algorithm can be realized on the available
//! resources.
//!
//! ```text
//!                    Function AST
//!                         │
//!                         ▼
//!                  semantic meaning
//!                         │
//!                         ▼
//!                         ZUIR
//!                         │
//!              ┌──────────┼──────────┐
//!              ▼          ▼          ▼
//!          classical    quantum      HDL
//!              │          │          │
//!              └──────────┼──────────┘
//!                         ▼
//!                target realization
//! ```
//!
//! ## Important architectural decision
//!
//! Child AST structures are referenced through [`NodeId`] rather than being
//! duplicated inside this declaration.
//!
//! This gives the AST a graph-oriented representation:
//!
//! ```text
//! Function
//!   │
//!   ├── NodeId ──► Parameter
//!   ├── NodeId ──► GenericParameter
//!   ├── NodeId ──► Type
//!   ├── NodeId ──► Statement/Block
//!   ├── NodeId ──► Effect
//!   ├── NodeId ──► Capability
//!   └── NodeId ──► Contract
//! ```
//!
//! The canonical AST store owns the actual nodes.
//!
//! `Function` therefore never needs to be modified merely because a new AST
//! node type is introduced.
//!
//! ## Extensibility
//!
//! Function modifiers are represented using namespaced source-level identifiers
//! rather than a closed enum containing every possible future modifier.
//!
//! Therefore a new modifier does not require modifying this file.
//!
//! For example, these are representable without changing `Function`:
//!
//! ```text
//! zamani:async
//! zamani:inline
//! zamani:const
//! zamani:unsafe
//! quantum:entry
//! quantum:adaptive
//! future-domain:distributed
//! future-domain:accelerated
//! ```
//!
//! Whether a modifier is legal is determined by semantic analysis and the
//! relevant language/extension registry.
//!
//! ## Dependency contract
//!
//! This file may depend only on:
//!
//! - [`super::super::node::Node`];
//! - [`super::super::node::AstNode`];
//! - [`super::super::node_id::NodeId`];
//! - [`super::super::node_kind::{CoreNodeKind, NodeKind}`];
//! - [`super::super::metadata::NodeMetadata`];
//! - [`super::super::source::Span`];
//! - Rust standard-library facilities;
//! - `serde` already used by the foundational AST node infrastructure.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - quantum routing;
//! - quantum scheduling;
//! - quantum optimization;
//! - quantum resilience;
//! - quantum error correction;
//! - calibration;
//! - runtime;
//! - backend providers;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - vendor SDKs.
//!
//! ## Integration contract
//!
//! ### Parser
//!
//! The parser:
//!
//! 1. allocates a `NodeId` for the function;
//! 2. constructs child parameter/generic/type/body nodes;
//! 3. stores those children in the canonical AST store;
//! 4. constructs this `Function` using the corresponding `NodeId`s;
//! 5. wraps the function in the program/module item relationship.
//!
//! The parser must not perform:
//!
//! - name resolution;
//! - type inference;
//! - resource allocation;
//! - hardware selection;
//! - quantum routing;
//! - scheduling.
//!
//! ### Semantic analysis
//!
//! Semantic analysis resolves every referenced `NodeId` through the canonical
//! AST store and derives:
//!
//! - symbol identity;
//! - resolved parameter types;
//! - generic substitutions;
//! - return type;
//! - effects;
//! - capabilities;
//! - resource requirements;
//! - domain semantics;
//! - callable signature;
//! - control-flow properties.
//!
//! None of that information belongs in this AST node.
//!
//! ### ZUIR
//!
//! `Function` does not lower directly to ZUIR.
//!
//! The required path is:
//!
//! ```text
//! Function
//!    │
//!    ▼
//! semantic analysis
//!    │
//!    ▼
//! semantic function model
//!    │
//!    ▼
//! ZUIR function
//! ```
//!
//! ### Quantum integration
//!
//! Quantum functions are ordinary functions at the native AST level.
//!
//! A function can refer to generic resource, operation, effect and capability
//! nodes without this file knowing whether the eventual implementation uses:
//!
//! - one qubit;
//! - many qubits;
//! - logical qubits;
//! - physical qubits;
//! - a simulator;
//! - superconducting hardware;
//! - trapped ions;
//! - neutral atoms;
//! - photonics;
//! - a future quantum technology.
//!
//! Physical realization remains downstream.
//!
//! ## Scalability
//!
//! There is no fixed limit in this representation on:
//!
//! - number of parameters;
//! - number of generic parameters;
//! - number of modifiers;
//! - number of effects;
//! - number of capabilities;
//! - number of contracts;
//! - function-body size;
//! - number of functions;
//! - quantum resource cardinality;
//! - computational target size.
//!
//! Collections grow according to available memory and configured compiler
//! resource policies.
//!
//! Safety limits must not be encoded as language semantics in this file.
//!
//! ## Determinism
//!
//! This type:
//!
//! - has no global mutable state;
//! - allocates no IDs;
//! - uses no randomness;
//! - accesses no clock;
//! - accesses no hardware;
//! - depends on no hash-map iteration order.
//!
//! Deterministic identity allocation is owned by the AST builder/parser.
//!
//! ## Security
//!
//! This file contains no `unsafe` code.
//!
//! It performs no unchecked indexing and no pointer manipulation.
//!
//! Collection validation is performed iteratively where practical and does not
//! dereference child nodes.
//!
//! Child relationships are represented by opaque `NodeId`s and therefore do
//! not permit arbitrary memory access.
//!
//! ## Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Edition 2021.
//!
//! No unstable features are required.
//!
//! No `unsafe` code is used.

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Schema version for the source-level function declaration contract.
///
/// This version is independent of:
///
/// - Zamani language version;
/// - compiler version;
/// - serialized AST version;
/// - extension versions;
/// - ZUIR version;
/// - target/backend versions.
pub const FUNCTION_AST_SCHEMA_VERSION: u16 = 1;

/// Creates the canonical native node kind for a function.
#[inline]
#[must_use]
pub const fn function_node_kind() -> NodeKind {
    NodeKind::Core(CoreNodeKind::Function)
}

/// Source-level visibility of a function.
///
/// Visibility is language semantics, not hardware semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum FunctionVisibility {
    /// Function is visible according to the public language visibility rules.
    Public,

    /// Function is visible only within its defining private scope.
    Private,

    /// Function is visible according to protected/inherited visibility rules.
    Protected,
}

impl Default for FunctionVisibility {
    fn default() -> Self {
        Self::Private
    }
}

impl fmt::Display for FunctionVisibility {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Public => formatter.write_str("public"),
            Self::Private => formatter.write_str("private"),
            Self::Protected => formatter.write_str("protected"),
        }
    }
}

/// Namespaced source-level function modifier.
///
/// A modifier is intentionally not represented by a closed enum.
///
/// This allows language extensions to add modifiers without changing the
/// canonical function representation.
///
/// # Examples
///
/// ```text
/// zamani:async
/// zamani:inline
/// zamani:const
/// quantum:adaptive
/// custom-domain:entry
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionModifier {
    namespace: String,
    name: String,
    value: Option<String>,
}

impl FunctionModifier {
    /// Creates a namespaced modifier.
    ///
    /// # Errors
    ///
    /// Returns an error when the namespace or name is empty or contains
    /// structural whitespace.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, FunctionModifierError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identifier_component(&namespace, "namespace")?;
        validate_identifier_component(&name, "name")?;

        Ok(Self {
            namespace,
            name,
            value: None,
        })
    }

    /// Creates a namespaced modifier with a source-level value.
    ///
    /// The value remains opaque to the AST.
    ///
    /// Semantic analysis determines its meaning.
    pub fn with_value(
        namespace: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, FunctionModifierError> {
        let mut modifier = Self::new(namespace, name)?;
        let value = value.into();

        if value.is_empty() {
            return Err(FunctionModifierError::EmptyValue);
        }

        modifier.value = Some(value);
        Ok(modifier)
    }

    /// Returns the modifier namespace.
    #[inline]
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the modifier name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the optional source-level modifier value.
    #[inline]
    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Returns the stable `namespace:name` identity.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        let mut result =
            String::with_capacity(self.namespace.len() + 1 + self.name.len());

        result.push_str(&self.namespace);
        result.push(':');
        result.push_str(&self.name);

        result
    }
}

impl fmt::Display for FunctionModifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.qualified_name())?;

        if let Some(value) = self.value() {
            formatter.write_str("=")?;
            formatter.write_str(value)?;
        }

        Ok(())
    }
}

/// Errors produced while constructing a [`FunctionModifier`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FunctionModifierError {
    /// Namespace or name is empty.
    EmptyComponent {
        /// Component name.
        component: &'static str,
    },

    /// Namespace or name contains whitespace.
    Whitespace {
        /// Component name.
        component: &'static str,
    },

    /// Modifier value is empty.
    EmptyValue,
}

impl fmt::Display for FunctionModifierError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyComponent { component } => {
                write!(formatter, "function modifier {component} cannot be empty")
            }
            Self::Whitespace { component } => {
                write!(
                    formatter,
                    "function modifier {component} cannot contain whitespace"
                )
            }
            Self::EmptyValue => {
                formatter.write_str("function modifier value cannot be empty")
            }
        }
    }
}

impl std::error::Error for FunctionModifierError {}

/// Source-level function declaration.
///
/// The function owns its declaration metadata but references child AST nodes
/// through `NodeId`.
///
/// # Invariants
///
/// A structurally valid `Function` satisfies:
///
/// 1. Its common node kind is `CoreNodeKind::Function`.
/// 2. Its function node identity is distinct from all child identities.
/// 3. Its name is non-empty.
/// 4. Its parameter IDs are unique within the declaration.
/// 5. Its generic parameter IDs are unique within the declaration.
/// 6. Its effect IDs are unique within the declaration.
/// 7. Its capability IDs are unique within the declaration.
/// 8. Its contract IDs are unique within the declaration.
/// 9. The optional return-type ID is not the function's own ID.
/// 10. The body ID is not the function's own ID.
/// 11. No target, machine, backend or hardware information is stored.
/// 12. Child nodes remain owned by the canonical AST store.
///
/// # Source structure
///
/// ```text
/// Function
/// ├── Node
/// ├── name
/// ├── visibility
/// ├── modifiers
/// ├── generic_parameters
/// ├── parameters
/// ├── return_type
/// ├── body
/// ├── effects
/// ├── capabilities
/// └── contracts
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
    /// Common AST identity, source span and metadata.
    node: Node,

    /// Source-level function name.
    name: String,

    /// Source-level visibility.
    visibility: FunctionVisibility,

    /// Extensible source-level modifiers.
    modifiers: Vec<FunctionModifier>,

    /// Generic parameter nodes.
    generic_parameters: Vec<NodeId>,

    /// Function parameter nodes in source order.
    parameters: Vec<NodeId>,

    /// Optional return-type node.
    return_type: Option<NodeId>,

    /// Function body node.
    ///
    /// The referenced node is normally a block/body AST node.
    ///
    /// The AST intentionally does not impose a concrete body type here.
    body: NodeId,

    /// Source-level effect declarations/annotations.
    effects: Vec<NodeId>,

    /// Source-level capability declarations/requirements.
    capabilities: Vec<NodeId>,

    /// Source-level contracts.
    contracts: Vec<NodeId>,
}

impl Function {
    /// Creates a new function declaration.
    ///
    /// The caller owns allocation of every `NodeId`.
    ///
    /// This constructor validates only invariants that can be checked without
    /// access to the AST store.
    ///
    /// Cross-node validation remains the responsibility of the AST validation
    /// layer.
    ///
    /// # Arguments
    ///
    /// * `id` - Function node identity.
    /// * `span` - Source span covering the declaration.
    /// * `metadata` - Source-level metadata.
    /// * `name` - Function identifier.
    /// * `visibility` - Source-level visibility.
    /// * `modifiers` - Extensible function modifiers.
    /// * `generic_parameters` - Generic parameter node IDs.
    /// * `parameters` - Function parameter node IDs.
    /// * `return_type` - Optional return-type node ID.
    /// * `body` - Function body node ID.
    /// * `effects` - Effect node IDs.
    /// * `capabilities` - Capability node IDs.
    /// * `contracts` - Contract node IDs.
    ///
    /// # Errors
    ///
    /// Returns [`FunctionError`] when the function's local structural
    /// invariants cannot be satisfied.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        name: impl Into<String>,
        visibility: FunctionVisibility,
        modifiers: Vec<FunctionModifier>,
        generic_parameters: Vec<NodeId>,
        parameters: Vec<NodeId>,
        return_type: Option<NodeId>,
        body: NodeId,
        effects: Vec<NodeId>,
        capabilities: Vec<NodeId>,
        contracts: Vec<NodeId>,
    ) -> Result<Self, FunctionError> {
        let name = name.into();

        validate_function_name(&name)?;

        validate_child_ids(id, "generic parameters", &generic_parameters)?;
        validate_child_ids(id, "parameters", &parameters)?;
        validate_optional_child_id(id, "return type", return_type)?;
        validate_child_id(id, "body", body)?;
        validate_child_ids(id, "effects", &effects)?;
        validate_child_ids(id, "capabilities", &capabilities)?;
        validate_child_ids(id, "contracts", &contracts)?;

        ensure_unique_ids("generic parameters", &generic_parameters)?;
        ensure_unique_ids("parameters", &parameters)?;
        ensure_unique_ids("effects", &effects)?;
        ensure_unique_ids("capabilities", &capabilities)?;
        ensure_unique_ids("contracts", &contracts)?;

        Ok(Self {
            node: Node::new(
                id,
                function_node_kind(),
                span,
                metadata,
            ),
            name,
            visibility,
            modifiers,
            generic_parameters,
            parameters,
            return_type,
            body,
            effects,
            capabilities,
            contracts,
        })
    }

    /// Creates a function using default source-level metadata and private
    /// visibility.
    pub fn simple(
        id: NodeId,
        span: Span,
        name: impl Into<String>,
        parameters: Vec<NodeId>,
        body: NodeId,
    ) -> Result<Self, FunctionError> {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            name,
            FunctionVisibility::Private,
            Vec::new(),
            Vec::new(),
            parameters,
            None,
            body,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    /// Returns the function's stable AST node identity.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the function's node kind.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the function's source span.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns immutable source-level metadata.
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

    /// Returns the function name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the function visibility.
    #[inline]
    #[must_use]
    pub const fn visibility(&self) -> FunctionVisibility {
        self.visibility
    }

    /// Changes function visibility.
    ///
    /// This modifies source-level AST information only.
    pub fn set_visibility(&mut self, visibility: FunctionVisibility) {
        self.visibility = visibility;
    }

    /// Returns all function modifiers in source order.
    #[inline]
    #[must_use]
    pub fn modifiers(&self) -> &[FunctionModifier] {
        &self.modifiers
    }

    /// Returns mutable access to function modifiers.
    ///
    /// Callers must preserve modifier validity.
    #[inline]
    pub fn modifiers_mut(&mut self) -> &mut Vec<FunctionModifier> {
        &mut self.modifiers
    }

    /// Adds a source-level modifier.
    ///
    /// Duplicate modifiers are allowed at this AST layer because some language
    /// extensions may define meaningful repeated annotations. Semantic
    /// validation determines whether repetition is legal.
    pub fn push_modifier(&mut self, modifier: FunctionModifier) {
        self.modifiers.push(modifier);
    }

    /// Returns generic parameter node IDs in source order.
    #[inline]
    #[must_use]
    pub fn generic_parameters(&self) -> &[NodeId] {
        &self.generic_parameters
    }

    /// Returns mutable access to generic parameter IDs.
    ///
    /// Structural validation must be run after mutation.
    #[inline]
    pub fn generic_parameters_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.generic_parameters
    }

    /// Adds one generic parameter.
    ///
    /// Returns an error when the ID duplicates an existing generic parameter or
    /// equals the function's own ID.
    pub fn push_generic_parameter(
        &mut self,
        parameter: NodeId,
    ) -> Result<(), FunctionError> {
        validate_child_id(self.id(), "generic parameter", parameter)?;

        if self.generic_parameters.contains(&parameter) {
            return Err(FunctionError::DuplicateChildId {
                category: "generic parameter",
                id: parameter,
            });
        }

        self.generic_parameters.push(parameter);
        Ok(())
    }

    /// Returns function parameter node IDs in source order.
    #[inline]
    #[must_use]
    pub fn parameters(&self) -> &[NodeId] {
        &self.parameters
    }

    /// Returns mutable access to function parameter IDs.
    #[inline]
    pub fn parameters_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.parameters
    }

    /// Adds one function parameter.
    ///
    /// Parameter ordering is significant and is therefore preserved.
    pub fn push_parameter(
        &mut self,
        parameter: NodeId,
    ) -> Result<(), FunctionError> {
        validate_child_id(self.id(), "parameter", parameter)?;

        if self.parameters.contains(&parameter) {
            return Err(FunctionError::DuplicateChildId {
                category: "parameter",
                id: parameter,
            });
        }

        self.parameters.push(parameter);
        Ok(())
    }

    /// Returns the optional return-type node.
    #[inline]
    #[must_use]
    pub fn return_type(&self) -> Option<NodeId> {
        self.return_type
    }

    /// Replaces the optional return-type node.
    ///
    /// Passing `None` removes the explicit return type.
    pub fn set_return_type(
        &mut self,
        return_type: Option<NodeId>,
    ) -> Result<(), FunctionError> {
        validate_optional_child_id(self.id(), "return type", return_type)?;
        self.return_type = return_type;
        Ok(())
    }

    /// Returns the function body node ID.
    #[inline]
    #[must_use]
    pub fn body(&self) -> NodeId {
        self.body
    }

    /// Replaces the function body node.
    pub fn set_body(
        &mut self,
        body: NodeId,
    ) -> Result<(), FunctionError> {
        validate_child_id(self.id(), "body", body)?;
        self.body = body;
        Ok(())
    }

    /// Returns effect node IDs in source order.
    #[inline]
    #[must_use]
    pub fn effects(&self) -> &[NodeId] {
        &self.effects
    }

    /// Returns mutable access to effect node IDs.
    #[inline]
    pub fn effects_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.effects
    }

    /// Adds an effect reference.
    pub fn push_effect(
        &mut self,
        effect: NodeId,
    ) -> Result<(), FunctionError> {
        validate_child_id(self.id(), "effect", effect)?;

        if self.effects.contains(&effect) {
            return Err(FunctionError::DuplicateChildId {
                category: "effect",
                id: effect,
            });
        }

        self.effects.push(effect);
        Ok(())
    }

    /// Returns capability node IDs in source order.
    #[inline]
    #[must_use]
    pub fn capabilities(&self) -> &[NodeId] {
        &self.capabilities
    }

    /// Returns mutable access to capability node IDs.
    #[inline]
    pub fn capabilities_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.capabilities
    }

    /// Adds a capability reference.
    pub fn push_capability(
        &mut self,
        capability: NodeId,
    ) -> Result<(), FunctionError> {
        validate_child_id(self.id(), "capability", capability)?;

        if self.capabilities.contains(&capability) {
            return Err(FunctionError::DuplicateChildId {
                category: "capability",
                id: capability,
            });
        }

        self.capabilities.push(capability);
        Ok(())
    }

    /// Returns contract node IDs in source order.
    #[inline]
    #[must_use]
    pub fn contracts(&self) -> &[NodeId] {
        &self.contracts
    }

    /// Returns mutable access to contract node IDs.
    #[inline]
    pub fn contracts_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.contracts
    }

    /// Adds a contract reference.
    pub fn push_contract(
        &mut self,
        contract: NodeId,
    ) -> Result<(), FunctionError> {
        validate_child_id(self.id(), "contract", contract)?;

        if self.contracts.contains(&contract) {
            return Err(FunctionError::DuplicateChildId {
                category: "contract",
                id: contract,
            });
        }

        self.contracts.push(contract);
        Ok(())
    }

    /// Returns the function AST schema version.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        FUNCTION_AST_SCHEMA_VERSION
    }

    /// Returns whether this function has an explicit return type.
    #[inline]
    #[must_use]
    pub const fn has_explicit_return_type(&self) -> bool {
        self.return_type.is_some()
    }

    /// Returns whether this function has generic parameters.
    #[inline]
    #[must_use]
    pub fn is_generic(&self) -> bool {
        !self.generic_parameters.is_empty()
    }

    /// Returns whether this function has effects.
    #[inline]
    #[must_use]
    pub fn has_effects(&self) -> bool {
        !self.effects.is_empty()
    }

    /// Returns whether this function has capability requirements.
    #[inline]
    #[must_use]
    pub fn has_capabilities(&self) -> bool {
        !self.capabilities.is_empty()
    }

    /// Returns whether this function has contracts.
    #[inline]
    #[must_use]
    pub fn has_contracts(&self) -> bool {
        !self.contracts.is_empty()
    }

    /// Returns the number of parameters.
    ///
    /// This is a collection length, not a language-level limit.
    #[inline]
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Returns the number of generic parameters.
    #[inline]
    #[must_use]
    pub fn generic_parameter_count(&self) -> usize {
        self.generic_parameters.len()
    }

    /// Returns a stable source-level signature summary.
    ///
    /// The summary intentionally contains no resolved types or target
    /// information.
    #[must_use]
    pub fn signature_summary(&self) -> FunctionSignatureSummary {
        FunctionSignatureSummary {
            name: self.name.clone(),
            generic_parameter_count: self.generic_parameters.len(),
            parameter_count: self.parameters.len(),
            has_explicit_return_type: self.return_type.is_some(),
        }
    }

    /// Validates local structural invariants.
    ///
    /// This method does not dereference child `NodeId`s.
    ///
    /// Therefore it cannot validate whether the IDs actually exist in the
    /// enclosing AST store. That is intentionally left to graph-level AST
    /// validation.
    pub fn validate(&self) -> Result<(), FunctionError> {
        if !self.kind().is_core()
            || self.kind().as_core() != Some(CoreNodeKind::Function)
        {
            return Err(FunctionError::InvalidNodeKind {
                actual: self.kind().clone(),
            });
        }

        validate_function_name(&self.name)?;

        validate_child_ids(
            self.id(),
            "generic parameters",
            &self.generic_parameters,
        )?;

        validate_child_ids(
            self.id(),
            "parameters",
            &self.parameters,
        )?;

        validate_optional_child_id(
            self.id(),
            "return type",
            self.return_type,
        )?;

        validate_child_id(self.id(), "body", self.body)?;

        validate_child_ids(
            self.id(),
            "effects",
            &self.effects,
        )?;

        validate_child_ids(
            self.id(),
            "capabilities",
            &self.capabilities,
        )?;

        validate_child_ids(
            self.id(),
            "contracts",
            &self.contracts,
        )?;

        ensure_unique_ids(
            "generic parameters",
            &self.generic_parameters,
        )?;

        ensure_unique_ids(
            "parameters",
            &self.parameters,
        )?;

        ensure_unique_ids(
            "effects",
            &self.effects,
        )?;

        ensure_unique_ids(
            "capabilities",
            &self.capabilities,
        )?;

        ensure_unique_ids(
            "contracts",
            &self.contracts,
        )?;

        Ok(())
    }

    /// Returns all child node IDs in deterministic source/semantic order.
    ///
    /// The returned order is:
    ///
    /// 1. generic parameters;
    /// 2. parameters;
    /// 3. return type, when present;
    /// 4. body;
    /// 5. effects;
    /// 6. capabilities;
    /// 7. contracts.
    ///
    /// This method allocates a new vector. Traversal engines that need
    /// allocation-free traversal should consume the individual accessors
    /// instead.
    #[must_use]
    pub fn child_ids(&self) -> Vec<NodeId> {
        let mut result = Vec::with_capacity(
            self.generic_parameters.len()
                + self.parameters.len()
                + usize::from(self.return_type.is_some())
                + 1
                + self.effects.len()
                + self.capabilities.len()
                + self.contracts.len(),
        );

        result.extend_from_slice(&self.generic_parameters);
        result.extend_from_slice(&self.parameters);

        if let Some(return_type) = self.return_type {
            result.push(return_type);
        }

        result.push(self.body);

        result.extend_from_slice(&self.effects);
        result.extend_from_slice(&self.capabilities);
        result.extend_from_slice(&self.contracts);

        result
    }
}

impl AstNode for Function {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

/// Compact source-level signature information.
///
/// This is useful for diagnostics, indexing and tooling.
///
/// It deliberately does not contain resolved types.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionSignatureSummary {
    /// Function name.
    pub name: String,

    /// Number of generic parameters.
    pub generic_parameter_count: usize,

    /// Number of ordinary parameters.
    pub parameter_count: usize,

    /// Whether an explicit return type exists.
    pub has_explicit_return_type: bool,
}

impl fmt::Display for FunctionSignatureSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)?;
        formatter.write_str("(")?;

        for index in 0..self.parameter_count {
            if index != 0 {
                formatter.write_str(", ")?;
            }

            formatter.write_str("_")?;
        }

        formatter.write_str(")")?;

        if self.has_explicit_return_type {
            formatter.write_str(" -> _")?;
        }

        Ok(())
    }
}

/// Errors produced by local function AST construction or validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FunctionError {
    /// Function name is empty.
    EmptyName,

    /// Function name contains structural whitespace.
    NameContainsWhitespace,

    /// A child ID refers to the function itself.
    SelfReference {
        /// Child category.
        category: &'static str,

        /// Function identity.
        function: NodeId,
    },

    /// A child ID was repeated.
    DuplicateChildId {
        /// Child category.
        category: &'static str,

        /// Repeated child identity.
        id: NodeId,
    },

    /// Function has an unexpected node kind.
    InvalidNodeKind {
        /// Actual node kind.
        actual: NodeKind,
    },

    /// Function modifier is structurally invalid.
    InvalidModifier(FunctionModifierError),
}

impl fmt::Display for FunctionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str("function name cannot be empty")
            }

            Self::NameContainsWhitespace => {
                formatter.write_str(
                    "function name cannot contain structural whitespace",
                )
            }

            Self::SelfReference { category, function } => {
                write!(
                    formatter,
                    "function {function} cannot reference itself as a {category}"
                )
            }

            Self::DuplicateChildId { category, id } => {
                write!(
                    formatter,
                    "duplicate {category} node identity: {id}"
                )
            }

            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "function node has invalid AST kind: {actual}"
                )
            }

            Self::InvalidModifier(error) => {
                write!(formatter, "invalid function modifier: {error}")
            }
        }
    }
}

impl std::error::Error for FunctionError {}

/// Validates a function identifier.
///
/// This intentionally does not impose an ASCII-only policy. Unicode identifiers
/// remain possible at the source-language level.
///
/// Lexical identifier rules are owned by the lexer/parser; this function only
/// enforces structural properties that must hold regardless of lexer policy.
fn validate_function_name(name: &str) -> Result<(), FunctionError> {
    if name.is_empty() {
        return Err(FunctionError::EmptyName);
    }

    if name.chars().any(char::is_whitespace) {
        return Err(FunctionError::NameContainsWhitespace);
    }

    Ok(())
}

/// Validates a single child node identity.
fn validate_child_id(
    function_id: NodeId,
    category: &'static str,
    child: NodeId,
) -> Result<(), FunctionError> {
    if child == function_id {
        return Err(FunctionError::SelfReference {
            category,
            function: function_id,
        });
    }

    Ok(())
}

/// Validates an optional child node identity.
fn validate_optional_child_id(
    function_id: NodeId,
    category: &'static str,
    child: Option<NodeId>,
) -> Result<(), FunctionError> {
    if let Some(child) = child {
        validate_child_id(function_id, category, child)?;
    }

    Ok(())
}

/// Validates a collection of child node identities.
fn validate_child_ids(
    function_id: NodeId,
    category: &'static str,
    children: &[NodeId],
) -> Result<(), FunctionError> {
    for &child in children {
        validate_child_id(function_id, category, child)?;
    }

    Ok(())
}

/// Ensures that a child-ID collection contains no duplicate identities.
///
/// The implementation intentionally uses a simple deterministic O(n²) check
/// rather than allocating a hash set in the AST node itself.
///
/// This validation is construction-time structural validation, not a hot
/// traversal path. Large-scale compiler validation should use the surrounding
/// AST validation subsystem when it can provide more appropriate indexed
/// storage.
///
/// This method introduces no maximum collection size.
fn ensure_unique_ids(
    category: &'static str,
    children: &[NodeId],
) -> Result<(), FunctionError> {
    let mut left = 0usize;

    while left < children.len() {
        let mut right = left + 1;

        while right < children.len() {
            if children[left] == children[right] {
                return Err(FunctionError::DuplicateChildId {
                    category,
                    id: children[left],
                });
            }

            right += 1;
        }

        left += 1;
    }

    Ok(())
}

/// Validates a namespaced modifier component.
fn validate_identifier_component(
    value: &str,
    component: &'static str,
) -> Result<(), FunctionModifierError> {
    if value.is_empty() {
        return Err(FunctionModifierError::EmptyComponent { component });
    }

    if value.chars().any(char::is_whitespace) {
        return Err(FunctionModifierError::Whitespace { component });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value).expect("test IDs must be non-zero")
    }

    fn span() -> Span {
        Span::point(
            super::super::super::source::SourceId::from_raw(0),
            super::super::super::source::SourceOffset::from_raw(0),
        )
    }

    #[test]
    fn function_has_canonical_function_kind() {
        assert_eq!(
            function_node_kind(),
            NodeKind::Core(CoreNodeKind::Function)
        );
    }

    #[test]
    fn function_can_be_constructed_without_hardware_information() {
        let function = Function::simple(
            node_id(1),
            span(),
            "compute",
            vec![node_id(2), node_id(3)],
            node_id(4),
        )
        .expect("function construction should succeed");

        assert_eq!(function.id(), node_id(1));
        assert_eq!(function.name(), "compute");
        assert_eq!(function.parameter_count(), 2);
        assert_eq!(function.body(), node_id(4));
        assert_eq!(
            function.kind(),
            &NodeKind::Core(CoreNodeKind::Function)
        );
    }

    #[test]
    fn function_rejects_empty_name() {
        let result = Function::simple(
            node_id(1),
            span(),
            "",
            Vec::new(),
            node_id(2),
        );

        assert_eq!(
            result,
            Err(FunctionError::EmptyName)
        );
    }

    #[test]
    fn function_rejects_whitespace_in_name() {
        let result = Function::simple(
            node_id(1),
            span(),
            "bad name",
            Vec::new(),
            node_id(2),
        );

        assert_eq!(
            result,
            Err(FunctionError::NameContainsWhitespace)
        );
    }

    #[test]
    fn function_rejects_self_referencing_body() {
        let result = Function::simple(
            node_id(1),
            span(),
            "compute",
            Vec::new(),
            node_id(1),
        );

        assert!(matches!(
            result,
            Err(FunctionError::SelfReference {
                category: "body",
                ..
            })
        ));
    }

    #[test]
    fn function_rejects_duplicate_parameters() {
        let result = Function::new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            "compute",
            FunctionVisibility::Private,
            Vec::new(),
            Vec::new(),
            vec![node_id(2), node_id(2)],
            None,
            node_id(3),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(FunctionError::DuplicateChildId {
                category: "parameters",
                id
            }) if id == node_id(2)
        ));
    }

    #[test]
    fn function_accepts_generic_parameters_and_return_type() {
        let function = Function::new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            "identity",
            FunctionVisibility::Public,
            Vec::new(),
            vec![node_id(2)],
            vec![node_id(3)],
            Some(node_id(4)),
            node_id(5),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .expect("generic function should construct");

        assert!(function.is_generic());
        assert_eq!(
            function.generic_parameter_count(),
            1
        );
        assert_eq!(
            function.return_type(),
            Some(node_id(4))
        );
    }

    #[test]
    fn modifiers_are_open_and_namespaced() {
        let modifier = FunctionModifier::new(
            "quantum",
            "adaptive",
        )
        .expect("modifier should construct");

        assert_eq!(modifier.namespace(), "quantum");
        assert_eq!(modifier.name(), "adaptive");
        assert_eq!(
            modifier.qualified_name(),
            "quantum:adaptive"
        );
    }

    #[test]
    fn modifiers_can_carry_opaque_source_values() {
        let modifier = FunctionModifier::with_value(
            "zamani",
            "mode",
            "adaptive",
        )
        .expect("modifier should construct");

        assert_eq!(
            modifier.value(),
            Some("adaptive")
        );
    }

    #[test]
    fn modifiers_reject_empty_namespace() {
        let result = FunctionModifier::new("", "async");

        assert_eq!(
            result,
            Err(FunctionModifierError::EmptyComponent {
                component: "namespace"
            })
        );
    }

    #[test]
    fn modifiers_reject_whitespace() {
        let result = FunctionModifier::new(
            "zamani",
            "not valid",
        );

        assert_eq!(
            result,
            Err(FunctionModifierError::Whitespace {
                component: "name"
            })
        );
    }

    #[test]
    fn child_ids_are_deterministic() {
        let function = Function::new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            "compute",
            FunctionVisibility::Private,
            Vec::new(),
            vec![node_id(2)],
            vec![node_id(3), node_id(4)],
            Some(node_id(5)),
            node_id(6),
            vec![node_id(7)],
            vec![node_id(8)],
            vec![node_id(9)],
        )
        .expect("function should construct");

        assert_eq!(
            function.child_ids(),
            vec![
                node_id(2),
                node_id(3),
                node_id(4),
                node_id(5),
                node_id(6),
                node_id(7),
                node_id(8),
                node_id(9),
            ]
        );
    }

    #[test]
    fn function_validation_succeeds_for_valid_structure() {
        let function = Function::new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            "compute",
            FunctionVisibility::Public,
            Vec::new(),
            vec![node_id(2)],
            vec![node_id(3)],
            Some(node_id(4)),
            node_id(5),
            vec![node_id(6)],
            vec![node_id(7)],
            vec![node_id(8)],
        )
        .expect("function should construct");

        assert!(function.validate().is_ok());
    }

    #[test]
    fn function_signature_summary_is_source_level_only() {
        let function = Function::new(
            node_id(1),
            span(),
            NodeMetadata::default(),
            "compute",
            FunctionVisibility::Public,
            Vec::new(),
            vec![node_id(2)],
            vec![node_id(3), node_id(4)],
            Some(node_id(5)),
            node_id(6),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
        .expect("function should construct");

        let summary = function.signature_summary();

        assert_eq!(summary.name, "compute");
        assert_eq!(summary.generic_parameter_count, 1);
        assert_eq!(summary.parameter_count, 2);
        assert!(summary.has_explicit_return_type);
        assert_eq!(
            summary.to_string(),
            "compute(_, _) -> _"
        );
    }

    #[test]
    fn function_is_ast_node() {
        let function = Function::simple(
            node_id(1),
            span(),
            "compute",
            Vec::new(),
            node_id(2),
        )
        .expect("function should construct");

        let node: &dyn AstNode = &function;

        assert_eq!(node.id(), node_id(1));
        assert_eq!(
            node.kind(),
            &NodeKind::Core(CoreNodeKind::Function)
        );
    }

    #[test]
    fn function_schema_version_is_stable() {
        assert_eq!(
            Function::schema_version(),
            FUNCTION_AST_SCHEMA_VERSION
        );
    }

    #[test]
    fn function_supports_unicode_names() {
        let function = Function::simple(
            node_id(1),
            span(),
            "compute_量子",
            Vec::new(),
            node_id(2),
        )
        .expect("Unicode function names should be accepted");

        assert_eq!(
            function.name(),
            "compute_量子"
        );
    }

    #[test]
    fn function_has_no_fixed_parameter_limit() {
        let parameters: Vec<NodeId> =
            (2..=10_001).map(node_id).collect();

        let function = Function::simple(
            node_id(1),
            span(),
            "large",
            parameters,
            node_id(10_002),
        )
        .expect("large parameter lists should be representable");

        assert_eq!(
            function.parameter_count(),
            10_000
        );
    }
}