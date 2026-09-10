//! # Zamani Native AST — Capability
//!
//! Production-grade source-level representation of a Zamani capability.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! Lexer
//!     │
//!     ▼
//! Parser
//!     │
//!     ▼
//! Capability
//!     │
//!     ▼
//! Structural AST validation
//!     │
//!     ▼
//! Semantic analysis
//!     │
//!     ├── capability-name resolution
//!     ├── capability-kind interpretation
//!     ├── capability requirement analysis
//!     ├── resource analysis
//!     ├── domain analysis
//!     └── target compatibility analysis
//!     │
//!     ▼
//! Semantic Model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! Domain / target lowering
//! ```
//!
//! ## Scope
//!
//! This file owns the native Zamani AST representation of a capability
//! identity and its source-level version requirement.
//!
//! It deliberately does **not** own:
//!
//! - hardware capabilities;
//! - backend capabilities;
//! - quantum-device capabilities;
//! - QPU topology;
//! - physical qubits;
//! - QEC capabilities;
//! - scheduler capabilities;
//! - routing capabilities;
//! - calibration capabilities;
//! - runtime capability tokens;
//! - security credentials;
//! - capability authorization;
//! - capability negotiation;
//! - target compatibility;
//! - QIR capabilities;
//! - LLVM capabilities;
//! - MLIR capabilities.
//!
//! Those concerns belong to later compiler/runtime layers.
//!
//! ## Core architectural rule
//!
//! A capability in the native AST is **program intent**.
//!
//! It answers:
//!
//! > "What named computational property does the source program declare,
//! > require, provide, or refer to?"
//!
//! It does not answer:
//!
//! > "Which machine provides it?"
//!
//! or:
//!
//! > "How is it implemented?"
//!
//! or:
//!
//! > "Which physical resource satisfies it?"
//!
//! Those questions are resolved downstream.
//!
//! ## POCO-REAF
//!
//! The capability representation contains no:
//!
//! - machine size;
//! - qubit count;
//! - register width;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - processor architecture;
//! - instruction set;
//! - gate set;
//! - device identifier;
//! - queue identifier;
//! - calibration identifier;
//! - physical resource identifier;
//! - scheduler state.
//!
//! Consequently, a capability AST node remains unchanged when the same
//! Zamani source program is compiled for different computational scales,
//! architectures, quantum technologies, or execution environments.
//!
//! This is required for:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! ## Relationship with the Quantum IR
//!
//! The repository already contains a downstream Quantum IR capability model.
//! That model owns semantic capability identities such as `CapabilityId`.
//!
//! This file intentionally does **not** import that type.
//!
//! The boundary is:
//!
//! ```text
//! frontend::ast::node::capabilities::Capability
//!             │
//!             ▼
//! semantic capability resolution
//!             │
//!             ▼
//! quantum::ir::...::CapabilityId
//!             │
//!             ▼
//! ZUIR / Quantum IR
//! ```
//!
//! This prevents the frontend AST from depending on Quantum IR and keeps the
//! native AST domain-neutral.
//!
//! ## Namespaces
//!
//! Capability identities are namespace-qualified.
//!
//! Examples:
//!
//! ```text
//! zamani.quantum.mid_circuit_measurement
//! zamani.quantum.dynamic_control
//! zamani.quantum.logical_qubits
//! zamani.resource.distributed
//! zamani.compute.parallel
//! future.example.capability
//! ```
//!
//! The AST does not maintain a closed enumeration of capability names.
//!
//! A new capability must therefore not require changing this file.
//!
//! ## Scalability
//!
//! There is intentionally no maximum:
//!
//! - number of capabilities;
//! - number of capability declarations;
//! - namespace depth;
//! - namespace length;
//! - capability-name length;
//! - number of modules;
//! - number of machines;
//! - number of qubits;
//! - number of computational resources.
//!
//! Collections and source strings grow according to available resources and
//! configurable compiler resource policies.
//!
//! This file contains no language-level machine-size ceiling.
//!
//! ## Determinism
//!
//! Equality and hashing depend only on logical source-level state.
//!
//! The node contains no:
//!
//! - pointer address;
//! - timestamp;
//! - process identifier;
//! - random state;
//! - thread-local state;
//! - hardware state;
//! - runtime state.
//!
//! ## Dependency boundary
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
//! - the Rust standard library.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - Quantum IR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - quantum hardware;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime;
//! - backend providers;
//! - external quantum-language ASTs.
//!
//! ## Structural validation
//!
//! This file validates only local invariants:
//!
//! 1. the embedded node has `CoreNodeKind::Capability`;
//! 2. the namespace is non-empty;
//! 3. the capability name is non-empty;
//! 4. namespace/name components do not contain control characters;
//! 5. a version range has its lower bound no greater than its upper bound;
//! 6. the capability remains a source-level leaf node.
//!
//! It does not validate whether a capability exists or whether a target can
//! provide it.
//!
//! ## Semantic validation boundary
//!
//! The semantic layer determines:
//!
//! - whether the capability is known;
//! - whether it is declared by an extension;
//! - whether it is required or provided;
//! - whether it is compatible with another capability;
//! - whether the version requirement can be satisfied;
//! - whether the capability applies to a particular domain;
//! - whether the target provides it;
//! - whether the program can be lowered to ZUIR.
//!
//! None of that state is stored in this AST node.
//!
//! ## Versioning
//!
//! Capability identity and capability version are intentionally separate.
//!
//! ```text
//! capability:
//!     zamani.quantum.dynamic_control
//!
//! version:
//!     1.2.0
//! ```
//!
//! A new implementation/version therefore does not require a new capability
//! identity.
//!
//! The version constraint is source-level data. Target capability negotiation
//! belongs downstream.
//!
//! ## Serialization
//!
//! Serde serialization preserves the complete logical AST representation:
//!
//! - node identity;
//! - node kind;
//! - source span;
//! - metadata;
//! - namespace;
//! - capability name;
//! - version requirement.
//!
//! Global AST serialization/versioning remains owned by the AST serialization
//! subsystem. This file does not introduce a competing serialization format.
//!
//! ## Traversal
//!
//! Capability identity is a leaf source construct.
//!
//! It has zero child AST nodes.
//!
//! This keeps traversal:
//!
//! - deterministic;
//! - allocation-free;
//! - simple for visitors;
//! - safe for very large programs.
//!
//! ## Parser integration
//!
//! The parser should construct this node after recognizing the capability
//! syntax defined by the Zamani grammar.
//!
//! The parser owns lexical syntax.
//!
//! This file intentionally does not duplicate the lexer grammar.
//!
//! ## Semantic integration
//!
//! Semantic analysis consumes:
//!
//! ```text
//! capability.namespace()
//! capability.name()
//! capability.version_constraint()
//! capability.id()
//! ```
//!
//! The semantic layer may associate the AST `NodeId` with a resolved semantic
//! capability through a side table.
//!
//! No semantic information is added to this node.
//!
//! ## ZUIR integration
//!
//! This file does not import ZUIR.
//!
//! The semantic layer converts this source-level capability into the
//! appropriate semantic capability representation, which can subsequently
//! lower into ZUIR or a domain-specific IR.
//!
//! ## Security
//!
//! Capability declarations are source input and therefore untrusted.
//!
//! This implementation:
//!
//! - performs no unsafe operations;
//! - does not allocate hidden global state;
//! - does not perform filesystem access;
//! - does not perform network access;
//! - does not contact a backend;
//! - does not resolve credentials;
//! - does not interpret capability names as executable code.
//!
//! Namespace and name validation rejects control characters but intentionally
//! does not impose an arbitrary byte-length ceiling.
//!
//! Compiler-wide hostile-input limits belong to configurable compiler policy.
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
//! ## Integration contract
//!
//! ```text
//! src/frontend/ast/node/capabilities/mod.rs
//!     └── pub mod capability;
//!
//! src/frontend/ast/node/mod.rs
//!     └── capabilities module exposure
//!
//! parser
//!     └── source capability syntax → Capability
//!
//! structural validation
//!     └── Capability::validate_structure()
//!
//! visitors
//!     └── Capability is traversed as a leaf
//!
//! semantic analysis
//!     └── Capability::namespace()
//!     └── Capability::name()
//!     └── Capability::version_constraint()
//!
//! semantic model
//!     └── resolve AST capability into semantic capability identity
//!
//! ZUIR
//!     └── consume resolved semantic capability
//!
//! quantum/domain lowering
//!     └── determine actual implementation
//!
//! hardware/backend
//!     └── determine actual realization
//! ```
//!
//! Adding a new computational domain, quantum technology, machine size,
//! hardware backend, or capability must not require changing this file.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Stable contract version for [`Capability`].
///
/// This is the logical node contract version. It is intentionally independent
/// from the global AST serialization version and the Zamani language version.
pub const CAPABILITY_SCHEMA_VERSION: u16 = 1;

/// Stable source-level identifier for this AST node kind.
pub const CAPABILITY_KIND_NAME: &str = "zamani:capability";

/// Source-level capability version.
///
/// Capability identity and capability version are separate so that a capability
/// can evolve without changing its semantic identity.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CapabilityVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl CapabilityVersion {
    /// Creates a semantic capability version.
    #[must_use]
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major component.
    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    /// Returns the minor component.
    #[must_use]
    pub const fn minor(self) -> u64 {
        self.minor
    }

    /// Returns the patch component.
    #[must_use]
    pub const fn patch(self) -> u64 {
        self.patch
    }
}

impl fmt::Display for CapabilityVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

/// Source-level constraint on a capability version.
///
/// This is intentionally independent of target capability negotiation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityVersionConstraint {
    /// Any implementation version is acceptable.
    Any,

    /// The implementation must have exactly this version.
    Exact(CapabilityVersion),

    /// The implementation must have this version or newer.
    AtLeast(CapabilityVersion),

    /// The implementation must have this version or older.
    AtMost(CapabilityVersion),

    /// The implementation must fall inside an inclusive version interval.
    Between {
        /// Lowest accepted version.
        minimum: CapabilityVersion,

        /// Highest accepted version.
        maximum: CapabilityVersion,
    },
}

impl CapabilityVersionConstraint {
    /// Creates an inclusive version interval.
    ///
    /// # Errors
    ///
    /// Returns [`CapabilityError::InvalidVersionRange`] if `minimum` is greater
    /// than `maximum`.
    pub const fn between(
        minimum: CapabilityVersion,
        maximum: CapabilityVersion,
    ) -> Result<Self, CapabilityError> {
        if minimum > maximum {
            return Err(CapabilityError::InvalidVersionRange);
        }

        Ok(Self::Between { minimum, maximum })
    }

    /// Returns whether a concrete capability version satisfies this constraint.
    #[must_use]
    pub const fn matches(self, version: CapabilityVersion) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(required) => version == required,
            Self::AtLeast(required) => version >= required,
            Self::AtMost(required) => version <= required,
            Self::Between { minimum, maximum } => {
                version >= minimum && version <= maximum
            }
        }
    }
}

impl Default for CapabilityVersionConstraint {
    fn default() -> Self {
        Self::Any
    }
}

/// Errors detectable without consulting the complete AST graph.
///
/// These errors deliberately do not include semantic errors such as
/// "capability not supported by this machine". Such checks belong downstream.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CapabilityError {
    /// The namespace is empty.
    EmptyNamespace,

    /// The capability name is empty.
    EmptyName,

    /// A namespace/name component contains a control character.
    ControlCharacter {
        /// Component that contained the invalid character.
        component: CapabilityComponent,
    },

    /// A version range has its lower bound after its upper bound.
    InvalidVersionRange,

    /// The node has an unexpected AST node kind.
    InvalidNodeKind {
        /// Actual node classification.
        actual: NodeKind,
    },
}

impl fmt::Display for CapabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNamespace => {
                write!(formatter, "capability namespace must not be empty")
            }
            Self::EmptyName => {
                write!(formatter, "capability name must not be empty")
            }
            Self::ControlCharacter { component } => {
                write!(
                    formatter,
                    "capability {} contains a control character",
                    component
                )
            }
            Self::InvalidVersionRange => {
                write!(
                    formatter,
                    "capability version range minimum must not exceed maximum"
                )
            }
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "capability AST node has invalid node kind: {}",
                    actual
                )
            }
        }
    }
}

impl std::error::Error for CapabilityError {}

/// Identifies which source-level capability component failed validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapabilityComponent {
    /// Capability namespace.
    Namespace,

    /// Capability name.
    Name,
}

impl fmt::Display for CapabilityComponent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Namespace => write!(formatter, "namespace"),
            Self::Name => write!(formatter, "name"),
        }
    }
}

/// A source-level Zamani capability.
///
/// `Capability` represents capability intent, not capability realization.
///
/// Examples:
///
/// ```text
/// zamani.quantum.mid_circuit_measurement
/// zamani.quantum.dynamic_control
/// zamani.resource.distributed
/// future.example.new_capability
/// ```
///
/// The representation is deliberately open-ended. Adding a new capability does
/// not require changing the Rust type or recompiling this module around a
/// closed enumeration of capabilities.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Capability {
    /// Common source-level AST identity, source span, kind and metadata.
    node: Node,

    /// Namespace-qualified semantic namespace.
    ///
    /// The AST preserves this as source-level text. Resolution is performed
    /// later.
    namespace: String,

    /// Unqualified capability name.
    name: String,

    /// Optional source-level version requirement.
    version: CapabilityVersionConstraint,
}

impl Capability {
    /// Creates a capability after validating its source-level identity.
    ///
    /// This constructor creates the canonical native `Capability` node kind.
    ///
    /// It performs only local source-level validation. It does not determine
    /// whether the capability exists or whether any target provides it.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, CapabilityError> {
        Self::with_version(
            id,
            span,
            metadata,
            namespace,
            name,
            CapabilityVersionConstraint::Any,
        )
    }

    /// Creates a capability with an explicit version constraint.
    pub fn with_version(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        namespace: impl Into<String>,
        name: impl Into<String>,
        version: CapabilityVersionConstraint,
    ) -> Result<Self, CapabilityError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_component(&namespace, CapabilityComponent::Namespace)?;
        validate_component(&name, CapabilityComponent::Name)?;
        validate_version_constraint(version)?;

        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::Capability),
            span,
            metadata,
        );

        Ok(Self {
            node,
            namespace,
            name,
            version,
        })
    }

    /// Creates a capability with default metadata.
    pub fn without_metadata(
        id: NodeId,
        span: Span,
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, CapabilityError> {
        Self::new(
            id,
            span,
            NodeMetadata::default(),
            namespace,
            name,
        )
    }

    /// Creates a capability with default metadata and an explicit version
    /// constraint.
    pub fn without_metadata_with_version(
        id: NodeId,
        span: Span,
        namespace: impl Into<String>,
        name: impl Into<String>,
        version: CapabilityVersionConstraint,
    ) -> Result<Self, CapabilityError> {
        Self::with_version(
            id,
            span,
            NodeMetadata::default(),
            namespace,
            name,
            version,
        )
    }

    /// Creates a capability from an already-existing common AST node.
    ///
    /// This constructor is useful for controlled AST transformations and
    /// migration code.
    ///
    /// Unlike [`Self::new`], it does not silently rewrite the supplied node's
    /// classification. Structural validation reports a mismatched kind.
    pub fn from_node(
        node: Node,
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, CapabilityError> {
        Self::from_node_with_version(
            node,
            namespace,
            name,
            CapabilityVersionConstraint::Any,
        )
    }

    /// Creates a capability from an existing node and explicit version
    /// constraint.
    pub fn from_node_with_version(
        node: Node,
        namespace: impl Into<String>,
        name: impl Into<String>,
        version: CapabilityVersionConstraint,
    ) -> Result<Self, CapabilityError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_component(&namespace, CapabilityComponent::Namespace)?;
        validate_component(&name, CapabilityComponent::Name)?;
        validate_version_constraint(version)?;

        Ok(Self {
            node,
            namespace,
            name,
            version,
        })
    }

    /// Returns the capability namespace.
    #[must_use]
    #[inline]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the unqualified capability name.
    #[must_use]
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the source-level version constraint.
    #[must_use]
    #[inline]
    pub const fn version_constraint(&self) -> CapabilityVersionConstraint {
        self.version
    }

    /// Returns whether any version is accepted.
    #[must_use]
    #[inline]
    pub const fn accepts_any_version(&self) -> bool {
        matches!(self.version, CapabilityVersionConstraint::Any)
    }

    /// Returns a stable namespace-qualified source capability identifier.
    ///
    /// The returned string is source-level vocabulary only. It is not a
    /// backend identifier and must not be interpreted as one.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        let mut qualified =
            String::with_capacity(self.namespace.len() + 1 + self.name.len());

        qualified.push_str(&self.namespace);
        qualified.push('.');
        qualified.push_str(&self.name);

        qualified
    }

    /// Returns the byte length of the namespace.
    ///
    /// This is informational only and does not impose a language-level limit.
    #[must_use]
    #[inline]
    pub fn namespace_byte_len(&self) -> usize {
        self.namespace.len()
    }

    /// Returns the byte length of the capability name.
    ///
    /// This is informational only and does not impose a language-level limit.
    #[must_use]
    #[inline]
    pub fn name_byte_len(&self) -> usize {
        self.name.len()
    }

    /// Returns whether this capability has no AST children.
    #[must_use]
    #[inline]
    pub const fn is_leaf(&self) -> bool {
        true
    }

    /// Returns the number of child AST nodes.
    ///
    /// Capability identity and version are scalar source data and therefore
    /// have no child AST nodes.
    #[must_use]
    #[inline]
    pub const fn child_count(&self) -> usize {
        0
    }

    /// Returns an empty iterator over child AST node IDs.
    ///
    /// This is intentionally allocation-free.
    #[must_use]
    #[inline]
    pub fn child_node_ids(&self) -> std::slice::Iter<'_, NodeId> {
        static CHILDREN: [NodeId; 0] = [];
        CHILDREN.iter()
    }

    /// Validates this capability's local AST invariants.
    ///
    /// This function intentionally does not perform semantic capability
    /// resolution.
    pub fn validate_structure(&self) -> Result<(), CapabilityError> {
        match self.node.kind().as_core() {
            Some(CoreNodeKind::Capability) => {}
            _ => {
                return Err(CapabilityError::InvalidNodeKind {
                    actual: self.node.kind_owned(),
                });
            }
        }

        validate_component(
            &self.namespace,
            CapabilityComponent::Namespace,
        )?;

        validate_component(
            &self.name,
            CapabilityComponent::Name,
        )?;

        validate_version_constraint(self.version)?;

        Ok(())
    }

    /// Returns the common AST node.
    #[must_use]
    #[inline]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// Structural transformations remain responsible for preserving the
    /// node-kind invariant.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Replaces the capability version requirement.
    ///
    /// The previous requirement is returned.
    ///
    /// This is a source-level transformation and does not perform target
    /// capability negotiation.
    #[must_use]
    pub fn replace_version_constraint(
        &mut self,
        version: CapabilityVersionConstraint,
    ) -> Result<CapabilityVersionConstraint, CapabilityError> {
        validate_version_constraint(version)?;

        let previous = self.version;
        self.version = version;
        Ok(previous)
    }

    /// Replaces the namespace.
    ///
    /// The replacement is validated before the AST is modified.
    pub fn replace_namespace(
        &mut self,
        namespace: impl Into<String>,
    ) -> Result<String, CapabilityError> {
        let namespace = namespace.into();

        validate_component(
            &namespace,
            CapabilityComponent::Namespace,
        )?;

        Ok(std::mem::replace(
            &mut self.namespace,
            namespace,
        ))
    }

    /// Replaces the capability name.
    ///
    /// The replacement is validated before the AST is modified.
    pub fn replace_name(
        &mut self,
        name: impl Into<String>,
    ) -> Result<String, CapabilityError> {
        let name = name.into();

        validate_component(
            &name,
            CapabilityComponent::Name,
        )?;

        Ok(std::mem::replace(&mut self.name, name))
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.qualified_name())?;

        match self.version {
            CapabilityVersionConstraint::Any => Ok(()),

            CapabilityVersionConstraint::Exact(version) => {
                write!(formatter, " = {version}")
            }

            CapabilityVersionConstraint::AtLeast(version) => {
                write!(formatter, " >= {version}")
            }

            CapabilityVersionConstraint::AtMost(version) => {
                write!(formatter, " <= {version}")
            }

            CapabilityVersionConstraint::Between {
                minimum,
                maximum,
            } => {
                write!(formatter, " in [{minimum}, {maximum}]")
            }
        }
    }
}

impl AstNode for Capability {
    /// Returns the embedded canonical AST node.
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the embedded canonical AST node.
    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

/// Validates a capability namespace/name component.
///
/// The AST deliberately performs minimal lexical validation here. The lexer
/// remains authoritative for the complete Zamani identifier grammar.
///
/// Control characters are rejected because they cannot safely participate in
/// stable source-level capability identifiers.
///
/// Unicode letters, digits, punctuation and namespace conventions remain
/// available to the language grammar without an arbitrary ASCII-only
/// restriction.
fn validate_component(
    value: &str,
    component: CapabilityComponent,
) -> Result<(), CapabilityError> {
    if value.is_empty() {
        return Err(match component {
            CapabilityComponent::Namespace => {
                CapabilityError::EmptyNamespace
            }
            CapabilityComponent::Name => CapabilityError::EmptyName,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(CapabilityError::ControlCharacter { component });
    }

    Ok(())
}

/// Validates a version constraint.
fn validate_version_constraint(
    constraint: CapabilityVersionConstraint,
) -> Result<(), CapabilityError> {
    match constraint {
        CapabilityVersionConstraint::Between {
            minimum,
            maximum,
        } if minimum > maximum => {
            Err(CapabilityError::InvalidVersionRange)
        }
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn capability() -> Capability {
        Capability::without_metadata(
            NodeId::new(1),
            Span::default(),
            "zamani.quantum",
            "dynamic_control",
        )
        .expect("valid capability")
    }

    #[test]
    fn constructs_canonical_capability() {
        let capability = capability();

        assert_eq!(
            capability.kind().as_core(),
            Some(CoreNodeKind::Capability)
        );

        assert_eq!(
            capability.namespace(),
            "zamani.quantum"
        );

        assert_eq!(
            capability.name(),
            "dynamic_control"
        );

        assert_eq!(
            capability.qualified_name(),
            "zamani.quantum.dynamic_control"
        );

        assert!(capability.accepts_any_version());
    }

    #[test]
    fn capability_is_a_leaf() {
        let capability = capability();

        assert!(capability.is_leaf());
        assert_eq!(capability.child_count(), 0);
        assert_eq!(
            capability.child_node_ids().count(),
            0
        );
    }

    #[test]
    fn ast_node_identity_is_preserved() {
        let capability = capability();

        assert_eq!(
            capability.id(),
            NodeId::new(1)
        );

        assert_eq!(
            capability.node().kind().as_core(),
            Some(CoreNodeKind::Capability)
        );
    }

    #[test]
    fn rejects_empty_namespace() {
        let result = Capability::without_metadata(
            NodeId::new(1),
            Span::default(),
            "",
            "dynamic_control",
        );

        assert_eq!(
            result,
            Err(CapabilityError::EmptyNamespace)
        );
    }

    #[test]
    fn rejects_empty_name() {
        let result = Capability::without_metadata(
            NodeId::new(1),
            Span::default(),
            "zamani.quantum",
            "",
        );

        assert_eq!(
            result,
            Err(CapabilityError::EmptyName)
        );
    }

    #[test]
    fn rejects_control_characters() {
        let result = Capability::without_metadata(
            NodeId::new(1),
            Span::default(),
            "zamani\nquantum",
            "dynamic_control",
        );

        assert_eq!(
            result,
            Err(CapabilityError::ControlCharacter {
                component: CapabilityComponent::Namespace,
            })
        );
    }

    #[test]
    fn supports_unicode_capability_names() {
        let capability = Capability::without_metadata(
            NodeId::new(1),
            Span::default(),
            "future.计算",
            "能力",
        )
        .expect("unicode capability should be representable");

        assert_eq!(
            capability.qualified_name(),
            "future.计算.能力"
        );
    }

    #[test]
    fn supports_version_constraints() {
        let version = CapabilityVersion::new(2, 4, 1);

        let capability =
            Capability::without_metadata_with_version(
                NodeId::new(1),
                Span::default(),
                "zamani.quantum",
                "dynamic_control",
                CapabilityVersionConstraint::AtLeast(version),
            )
            .expect("valid capability");

        assert_eq!(
            capability.version_constraint(),
            CapabilityVersionConstraint::AtLeast(version)
        );

        assert!(
            CapabilityVersionConstraint::AtLeast(version)
                .matches(CapabilityVersion::new(3, 0, 0))
        );

        assert!(
            !CapabilityVersionConstraint::AtLeast(version)
                .matches(CapabilityVersion::new(2, 3, 9))
        );
    }

    #[test]
    fn rejects_invalid_version_ranges() {
        let minimum = CapabilityVersion::new(2, 0, 0);
        let maximum = CapabilityVersion::new(1, 9, 9);

        let result =
            CapabilityVersionConstraint::between(
                minimum,
                maximum,
            );

        assert_eq!(
            result,
            Err(CapabilityError::InvalidVersionRange)
        );
    }

    #[test]
    fn validates_canonical_node_kind() {
        let capability = capability();

        assert!(
            capability.validate_structure().is_ok()
        );
    }

    #[test]
    fn validation_detects_changed_node_kind() {
        let mut capability = capability();

        capability
            .node_mut()
            .replace_kind(
                NodeKind::core(CoreNodeKind::Identifier)
            );

        assert_eq!(
            capability.validate_structure(),
            Err(CapabilityError::InvalidNodeKind {
                actual: NodeKind::core(
                    CoreNodeKind::Identifier
                ),
            })
        );
    }

    #[test]
    fn namespace_replacement_is_atomic_on_error() {
        let mut capability = capability();

        let result =
            capability.replace_namespace("");

        assert_eq!(
            result,
            Err(CapabilityError::EmptyNamespace)
        );

        assert_eq!(
            capability.namespace(),
            "zamani.quantum"
        );
    }

    #[test]
    fn name_replacement_is_atomic_on_error() {
        let mut capability = capability();

        let result =
            capability.replace_name("");

        assert_eq!(
            result,
            Err(CapabilityError::EmptyName)
        );

        assert_eq!(
            capability.name(),
            "dynamic_control"
        );
    }

    #[test]
    fn version_replacement_is_atomic_on_error() {
        let mut capability = capability();

        let original =
            capability.version_constraint();

        let result =
            capability.replace_version_constraint(
                CapabilityVersionConstraint::Between {
                    minimum: CapabilityVersion::new(2, 0, 0),
                    maximum: CapabilityVersion::new(1, 0, 0),
                },
            );

        assert_eq!(
            result,
            Err(CapabilityError::InvalidVersionRange)
        );

        assert_eq!(
            capability.version_constraint(),
            original
        );
    }

    #[test]
    fn capability_is_deterministic() {
        let first = capability();
        let second = capability();

        assert_eq!(first, second);
        assert_eq!(
            first.qualified_name(),
            second.qualified_name()
        );
    }

    #[test]
    fn display_is_source_level() {
        let capability =
            Capability::without_metadata_with_version(
                NodeId::new(1),
                Span::default(),
                "zamani.quantum",
                "dynamic_control",
                CapabilityVersionConstraint::Exact(
                    CapabilityVersion::new(1, 2, 3),
                ),
            )
            .expect("valid capability");

        assert_eq!(
            capability.to_string(),
            "zamani.quantum.dynamic_control = 1.2.3"
        );
    }

    #[test]
    fn no_hardware_information_is_required() {
        let capability = capability();

        assert_eq!(
            capability.qualified_name(),
            "zamani.quantum.dynamic_control"
        );

        // The source representation contains no device, topology, qubit,
        // backend, vendor, scheduler or physical-resource identity.
    }
}