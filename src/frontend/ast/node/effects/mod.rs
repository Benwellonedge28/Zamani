//! # Zamani Frontend AST — Source-Level Effects
//!
//! Canonical source-level representation of effect declarations/references.
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
//!      └── Effect
//!             │
//!             ├── identity
//!             ├── optional value
//!             └── source metadata
//!      │
//!      ▼
//! structural validation
//!      │
//!      ▼
//! semantic analysis
//!      │
//!      ├── effect resolution
//!      ├── effect compatibility
//!      ├── effect composition
//!      └── effect propagation
//!      │
//!      ▼
//! semantic model
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ▼
//! domain / target lowering
//! ```
//!
//! ## Purpose
//!
//! This module owns the native Zamani AST representation of a **source-level
//! effect**.
//!
//! An effect describes an observable semantic property or computational
//! consequence of source code. The AST records the programmer's declaration or
//! reference; it does not decide how that effect is implemented.
//!
//! ## Domain neutrality
//!
//! This module deliberately does **not** define a closed list such as:
//!
//! ```text
//! enum EffectKind {
//!     IO,
//!     Quantum,
//!     Hardware,
//!     Mutation,
//!     ...
//! }
//! ```
//!
//! Such an enum would make the native AST a closed world and would require
//! modifying this file whenever Zamani gains a new effect.
//!
//! Instead, effects use a namespaced identity:
//!
//! ```text
//! zamani:io
//! zamani:mutation
//! quantum:measurement
//! quantum:decoherence
//! hardware:interaction
//! custom-domain:effect
//! ```
//!
//! The namespace and name are opaque to the AST. Semantic analysis and the
//! registered effect system determine their meaning.
//!
//! This allows new effects to be introduced without redesigning the native AST.
//!
//! ## POCO-REAF
//!
//! Effects contain no:
//!
//! - machine size;
//! - processor count;
//! - GPU count;
//! - QPU count;
//! - qubit count;
//! - register width;
//! - topology;
//! - vendor;
//! - backend;
//! - instruction set;
//! - calibration;
//! - physical qubit mapping;
//! - scheduler state;
//! - runtime state.
//!
//! Therefore an effect-bearing program remains source-level and portable.
//!
//! ```text
//! Program Once
//!      │
//!      ▼
//! Effect AST
//!      │
//!      ▼
//! Semantic effect model
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ▼
//! target capability/resource analysis
//!      │
//!      ▼
//! target realization
//! ```
//!
//! ## Quantum compatibility
//!
//! The representation is capable of expressing quantum effects without making
//! quantum computing the definition of the effect system.
//!
//! Examples include:
//!
//! ```text
//! quantum:measurement
//! quantum:reset
//! quantum:entanglement
//! quantum:decoherence
//! quantum:non_unitary
//! quantum:adaptive_control
//! ```
//!
//! These are merely identifiers at the AST layer.
//!
//! Their semantic meaning is resolved downstream.
//!
//! In particular, this module does **not** implement:
//!
//! - quantum noise;
//! - ZQN;
//! - quantum error correction;
//! - routing;
//! - scheduling;
//! - pulse generation;
//! - calibration;
//! - physical qubit assignment;
//! - backend execution.
//!
//! ## Existing repository integration
//!
//! The repository already classifies effects with:
//!
//! ```text
//! CoreNodeKind::Effect
//! ```
//!
//! This module uses that existing canonical node kind rather than introducing
//! another classification system.
//!
//! Existing declaration nodes such as `Function` already retain effect
//! references as `NodeId`s. The canonical integration therefore remains:
//!
//! ```text
//! Function
//!     │
//!     └── Vec<NodeId>
//!             │
//!             ▼
//!          Effect
//! ```
//!
//! The canonical AST store owns the actual nodes.
//!
//! This module never owns another copy of those nodes.
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational AST infrastructure:
//!
//! - [`super::super::node::Node`];
//! - [`super::super::node::AstNode`];
//! - [`super::super::node_id::NodeId`];
//! - [`super::super::node_kind::{CoreNodeKind, NodeKind}`];
//! - [`super::super::metadata::NodeMetadata`];
//! - [`super::super::source::Span`];
//! - Rust standard-library facilities;
//! - `serde` already used by the AST.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - quantum hardware;
//! - routing;
//! - scheduling;
//! - optimization;
//! - QEC;
//! - ZQN;
//! - calibration;
//! - runtime;
//! - backend providers;
//! - vendor SDKs.
//!
//! ## Integration contract
//!
//! ### Parser
//!
//! The parser:
//!
//! 1. allocates a `NodeId`;
//! 2. determines the source span;
//! 3. parses the namespaced effect identity;
//! 4. optionally parses an opaque source-level value;
//! 5. constructs `Effect`;
//! 6. stores it in the canonical AST store;
//! 7. gives the resulting `NodeId` to its owning declaration/expression.
//!
//! The parser must not resolve the semantic meaning of the effect.
//!
//! ### Structural validation
//!
//! Validation verifies:
//!
//! - valid node identity;
//! - `CoreNodeKind::Effect` classification;
//! - valid namespace;
//! - valid name;
//! - valid optional value representation;
//! - valid source span.
//!
//! It does not determine whether an effect is legal for a particular target.
//!
//! ### Semantic analysis
//!
//! Semantic analysis resolves the effect identifier through the effect registry.
//!
//! It may determine:
//!
//! - effect identity;
//! - effect hierarchy;
//! - effect compatibility;
//! - effect composition;
//! - effect propagation;
//! - effect requirements;
//! - effect conflicts;
//! - domain semantics.
//!
//! Those properties do not belong in this AST node.
//!
//! ### ZUIR
//!
//! The AST effect is lowered through semantic analysis:
//!
//! ```text
//! Effect AST
//!     │
//!     ▼
//! semantic effect
//!     │
//!     ▼
//! ZUIR effect semantics
//! ```
//!
//! This module does not import or depend on ZUIR.
//!
//! ### Quantum integration
//!
//! Quantum effects are represented by ordinary namespaced effect identities.
//!
//! No quantum-specific branch is required in this module.
//!
//! Consequently, a new quantum effect can be introduced without modifying
//! this file.
//!
//! ### Future domains
//!
//! The same representation supports effects introduced by future domains such
//! as:
//!
//! - classical accelerators;
//! - AI computation;
//! - neuromorphic systems;
//! - photonic systems;
//! - analog computation;
//! - distributed computation;
//! - HDL;
//! - future computational models.
//!
//! ## Scalability
//!
//! There is no language-level limit in this type on:
//!
//! - the number of effects;
//! - the number of effect references;
//! - the number of namespaces;
//! - the number of effect names;
//! - the number of declarations containing effects;
//! - the number of computational domains.
//!
//! Collection cardinality is controlled by the owning AST structures and
//! configurable compiler resource policies.
//!
//! Safety limits must never become language semantics.
//!
//! ## Determinism
//!
//! This module:
//!
//! - uses no global mutable state;
//! - uses no randomness;
//! - uses no wall-clock time;
//! - accesses no hardware;
//! - allocates no node IDs;
//! - depends on no hash-map iteration order.
//!
//! Equality and hashing are determined solely by stored source-level data.
//!
//! ## Security
//!
//! Effect identities originate from source code and therefore are untrusted.
//!
//! Constructors reject structurally invalid identifiers and prevent accidental
//! acceptance of empty namespace/name components.
//!
//! The implementation:
//!
//! - contains no `unsafe`;
//! - performs no pointer operations;
//! - performs no unchecked indexing;
//! - does not execute effect values;
//! - does not perform filesystem/network access.
//!
//! Identifier-size protection is structural input protection, not a machine or
//! program-size limit.
//!
//! ## Serialization
//!
//! `Effect` and its supporting types implement `Serialize` and `Deserialize`
//! using the same serde infrastructure as the surrounding AST.
//!
//! Serialization contains only source-level information.
//!
//! No target/backend state is serialized.
//!
//! ## Versioning
//!
//! This schema version is independent of:
//!
//! - Zamani language version;
//! - compiler version;
//! - serialized AST format version;
//! - ZUIR version;
//! - quantum IR version;
//! - extension versions;
//! - backend versions.
//!
//! ## Rust compatibility
//!
//! Target:
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

/// Schema version for the source-level effect AST contract.
///
/// This is deliberately independent of the language, compiler, serialization,
/// semantic-model, ZUIR, and backend version spaces.
pub const EFFECT_AST_SCHEMA_VERSION: u16 = 1;

/// Maximum permitted size of one effect namespace or name.
///
/// This is a malformed-input protection bound. It is not a limit on:
///
/// - the number of effects;
/// - the number of effect namespaces;
/// - program size;
/// - machine size;
/// - resource size.
///
/// Keeping this bound local prevents pathological identifier allocations while
/// preserving POCO-REAF scalability.
const MAX_EFFECT_IDENTIFIER_BYTES: usize = 16 * 1024;

/// Canonical native AST node kind for an effect.
#[inline]
#[must_use]
pub const fn effect_node_kind() -> NodeKind {
    NodeKind::Core(CoreNodeKind::Effect)
}

/// Source-level namespaced identity of an effect.
///
/// The AST deliberately does not interpret the identity.
///
/// # Examples
///
/// ```text
/// zamani:io
/// zamani:mutation
/// quantum:measurement
/// quantum:decoherence
/// hardware:interaction
/// custom-domain:effect
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EffectIdentity {
    namespace: String,
    name: String,
}

impl EffectIdentity {
    /// Creates a namespaced effect identity.
    ///
    /// # Errors
    ///
    /// Returns [`EffectIdentityError`] when:
    ///
    /// - the namespace is empty;
    /// - the name is empty;
    /// - either component contains structural whitespace;
    /// - either component exceeds the structural identifier bound.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, EffectIdentityError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_component(&namespace, "namespace")?;
        validate_component(&name, "name")?;

        Ok(Self { namespace, name })
    }

    /// Returns the namespace.
    #[inline]
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the effect name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the stable `namespace:name` representation.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        let mut result =
            String::with_capacity(self.namespace.len() + 1 + self.name.len());

        result.push_str(&self.namespace);
        result.push(':');
        result.push_str(&self.name);

        result
    }

    /// Returns whether this identity belongs to the native Zamani namespace.
    #[inline]
    #[must_use]
    pub fn is_zamani(&self) -> bool {
        self.namespace == "zamani"
    }

    /// Returns whether this identity belongs to the quantum namespace.
    ///
    /// This is only an identity query. It does not make the AST quantum-aware.
    #[inline]
    #[must_use]
    pub fn is_namespace(&self, namespace: &str) -> bool {
        self.namespace == namespace
    }
}

impl fmt::Display for EffectIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.qualified_name())
    }
}

/// Errors produced while constructing an [`EffectIdentity`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EffectIdentityError {
    /// A namespace or name was empty.
    EmptyComponent {
        /// Component that was empty.
        component: &'static str,
    },

    /// A namespace or name contained whitespace.
    Whitespace {
        /// Component containing whitespace.
        component: &'static str,
    },

    /// A namespace or name exceeded the structural safety bound.
    ComponentTooLarge {
        /// Component that exceeded the bound.
        component: &'static str,

        /// Number of bytes supplied.
        bytes: usize,

        /// Maximum accepted number of bytes.
        maximum: usize,
    },
}

impl fmt::Display for EffectIdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyComponent { component } => {
                write!(formatter, "effect {component} cannot be empty")
            }

            Self::Whitespace { component } => {
                write!(
                    formatter,
                    "effect {component} cannot contain whitespace"
                )
            }

            Self::ComponentTooLarge {
                component,
                bytes,
                maximum,
            } => {
                write!(
                    formatter,
                    "effect {component} is {bytes} bytes; maximum is {maximum}"
                )
            }
        }
    }
}

impl std::error::Error for EffectIdentityError {}

/// Source-level effect declaration/reference.
///
/// `Effect` contains only information that belongs in the native AST.
///
/// The optional `value` is intentionally opaque. This allows an effect
/// extension to attach source-level information without forcing the core AST
/// to understand every future effect schema.
///
/// Semantic analysis is responsible for interpreting the value.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Effect {
    /// Common source-level AST identity and metadata.
    node: Node,

    /// Namespaced source-level effect identity.
    identity: EffectIdentity,

    /// Optional opaque source-level effect value.
    ///
    /// This is not a backend parameter object and must not contain resolved
    /// hardware/runtime state.
    value: Option<String>,
}

impl Effect {
    /// Creates a source-level effect.
    ///
    /// The supplied `Node` must have `CoreNodeKind::Effect`.
    ///
    /// This constructor deliberately checks the node kind immediately because
    /// allowing an effect to carry a different structural kind would make
    /// downstream visitor and validation behavior ambiguous.
    ///
    /// # Errors
    ///
    /// Returns [`EffectError::InvalidNodeKind`] when the node does not use the
    /// canonical effect node kind.
    pub fn new(
        node: Node,
        identity: EffectIdentity,
    ) -> Result<Self, EffectError> {
        if node.kind() != &effect_node_kind() {
            return Err(EffectError::InvalidNodeKind {
                actual: node.kind_owned(),
            });
        }

        Ok(Self {
            node,
            identity,
            value: None,
        })
    }

    /// Creates an effect with an opaque source-level value.
    ///
    /// The value is preserved by the AST and interpreted later.
    pub fn with_value(
        node: Node,
        identity: EffectIdentity,
        value: impl Into<String>,
    ) -> Result<Self, EffectError> {
        let mut effect = Self::new(node, identity)?;
        let value = value.into();

        if value.is_empty() {
            return Err(EffectError::EmptyValue);
        }

        effect.value = Some(value);

        Ok(effect)
    }

    /// Creates an effect directly from its foundational AST components.
    ///
    /// This convenience constructor is useful to parsers and builders because
    /// it prevents them from having to manually construct a `Node` followed by
    /// a separate effect construction step.
    pub fn from_parts(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        identity: EffectIdentity,
    ) -> Self {
        let node = Node::new(
            id,
            effect_node_kind(),
            span,
            metadata,
        );

        Self {
            node,
            identity,
            value: None,
        }
    }

    /// Creates an effect directly from its foundational AST components with an
    /// opaque source-level value.
    pub fn from_parts_with_value(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        identity: EffectIdentity,
        value: impl Into<String>,
    ) -> Result<Self, EffectError> {
        let node = Node::new(
            id,
            effect_node_kind(),
            span,
            metadata,
        );

        Self::with_value(node, identity, value)
    }

    /// Returns the common AST node.
    #[inline]
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    #[inline]
    #[must_use]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST node identity.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the canonical effect node kind.
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

    /// Returns source metadata.
    #[inline]
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns mutable source metadata.
    #[inline]
    #[must_use]
    pub fn metadata_mut(&mut self) -> &mut NodeMetadata {
        self.node.metadata_mut()
    }

    /// Returns the effect identity.
    #[inline]
    #[must_use]
    pub fn identity(&self) -> &EffectIdentity {
        &self.identity
    }

    /// Returns the effect namespace.
    #[inline]
    #[must_use]
    pub fn namespace(&self) -> &str {
        self.identity.namespace()
    }

    /// Returns the effect name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        self.identity.name()
    }

    /// Returns the stable qualified effect identity.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        self.identity.qualified_name()
    }

    /// Returns the optional opaque source-level value.
    #[inline]
    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Returns whether the effect has an explicit source-level value.
    #[inline]
    #[must_use]
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Replaces the opaque source-level value.
    ///
    /// Returns the previous value.
    ///
    /// An empty value is rejected because an empty value is indistinguishable
    /// from an absent value and therefore creates unnecessary ambiguity in the
    /// source representation.
    pub fn replace_value(
        &mut self,
        value: Option<String>,
    ) -> Result<Option<String>, EffectError> {
        if let Some(ref value) = value {
            if value.is_empty() {
                return Err(EffectError::EmptyValue);
            }
        }

        Ok(core::mem::replace(&mut self.value, value))
    }

    /// Returns the AST schema version.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        EFFECT_AST_SCHEMA_VERSION
    }

    /// Returns a compact representation suitable for diagnostics.
    ///
    /// Arbitrary effect values are intentionally excluded so diagnostics cannot
    /// accidentally dump large extension payloads.
    #[must_use]
    pub fn diagnostic_summary(&self) -> EffectDiagnosticSummary {
        EffectDiagnosticSummary {
            id: self.id(),
            identity: self.identity.clone(),
            span: self.span().clone(),
        }
    }

    /// Returns whether this is a source-level quantum namespace effect.
    ///
    /// This is only a namespace query. No quantum implementation is imported
    /// by this module.
    #[inline]
    #[must_use]
    pub fn is_quantum_namespace(&self) -> bool {
        self.identity.is_namespace("quantum")
    }
}

impl AstNode for Effect {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.qualified_name())?;

        if let Some(value) = self.value() {
            formatter.write_str("=")?;
            formatter.write_str(value)?;
        }

        Ok(())
    }
}

/// Errors produced while constructing or modifying an [`Effect`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EffectError {
    /// The supplied node was not classified as an effect.
    InvalidNodeKind {
        /// Actual node kind.
        actual: NodeKind,
    },

    /// An explicit effect value was empty.
    EmptyValue,
}

impl fmt::Display for EffectError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "effect node must use {}, found {actual}",
                    effect_node_kind()
                )
            }

            Self::EmptyValue => {
                formatter.write_str("effect value cannot be empty")
            }
        }
    }
}

impl std::error::Error for EffectError {}

/// Lightweight diagnostic representation of an effect.
///
/// The arbitrary extension value is deliberately excluded.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EffectDiagnosticSummary {
    /// Stable AST node identity.
    pub id: NodeId,

    /// Source-level effect identity.
    pub identity: EffectIdentity,

    /// Source location.
    pub span: Span,
}

impl fmt::Display for EffectDiagnosticSummary {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} at {}",
            self.identity,
            self.span
        )
    }
}

/// Validates one effect identifier component.
///
/// This is intentionally stricter than the semantic identifier system needs
/// to be: it only protects the structural AST representation from obviously
/// malformed components.
///
/// Semantic naming rules remain the responsibility of the language/extension
/// registry.
fn validate_component(
    value: &str,
    component: &'static str,
) -> Result<(), EffectIdentityError> {
    if value.is_empty() {
        return Err(EffectIdentityError::EmptyComponent { component });
    }

    if value.len() > MAX_EFFECT_IDENTIFIER_BYTES {
        return Err(EffectIdentityError::ComponentTooLarge {
            component,
            bytes: value.len(),
            maximum: MAX_EFFECT_IDENTIFIER_BYTES,
        });
    }

    if value.chars().any(char::is_whitespace) {
        return Err(EffectIdentityError::Whitespace { component });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_identity() -> EffectIdentity {
        EffectIdentity::new("zamani", "io")
            .expect("test effect identity must be valid")
    }

    #[test]
    fn identity_preserves_namespace_and_name() {
        let identity = test_identity();

        assert_eq!(identity.namespace(), "zamani");
        assert_eq!(identity.name(), "io");
        assert_eq!(identity.qualified_name(), "zamani:io");
    }

    #[test]
    fn identity_rejects_empty_namespace() {
        let result = EffectIdentity::new("", "io");

        assert_eq!(
            result,
            Err(EffectIdentityError::EmptyComponent {
                component: "namespace"
            })
        );
    }

    #[test]
    fn identity_rejects_empty_name() {
        let result = EffectIdentity::new("zamani", "");

        assert_eq!(
            result,
            Err(EffectIdentityError::EmptyComponent {
                component: "name"
            })
        );
    }

    #[test]
    fn identity_rejects_whitespace() {
        let result = EffectIdentity::new("zamani core", "io");

        assert_eq!(
            result,
            Err(EffectIdentityError::Whitespace {
                component: "namespace"
            })
        );
    }

    #[test]
    fn identity_is_namespaced_and_extensible() {
        let quantum =
            EffectIdentity::new("quantum", "measurement")
                .expect("quantum identity must be valid");

        let future =
            EffectIdentity::new("future-domain", "new-effect")
                .expect("future identity must be valid");

        assert_eq!(
            quantum.qualified_name(),
            "quantum:measurement"
        );

        assert_eq!(
            future.qualified_name(),
            "future-domain:new-effect"
        );
    }

    #[test]
    fn effect_uses_canonical_node_kind() {
        assert_eq!(
            effect_node_kind(),
            NodeKind::Core(CoreNodeKind::Effect)
        );
    }

    #[test]
    fn effect_from_parts_has_expected_identity() {
        let id = NodeId::default();
        let span = Span::default();
        let metadata = NodeMetadata::default();

        let effect = Effect::from_parts(
            id,
            span,
            metadata,
            test_identity(),
        );

        assert_eq!(effect.id(), id);
        assert_eq!(
            effect.kind(),
            &NodeKind::Core(CoreNodeKind::Effect)
        );
        assert_eq!(effect.qualified_name(), "zamani:io");
        assert_eq!(effect.value(), None);
    }

    #[test]
    fn effect_value_is_preserved() {
        let id = NodeId::default();
        let span = Span::default();
        let metadata = NodeMetadata::default();

        let effect = Effect::from_parts_with_value(
            id,
            span,
            metadata,
            test_identity(),
            "source-level-value",
        )
        .expect("effect value must be accepted");

        assert_eq!(
            effect.value(),
            Some("source-level-value")
        );
    }

    #[test]
    fn empty_effect_value_is_rejected() {
        let id = NodeId::default();
        let span = Span::default();
        let metadata = NodeMetadata::default();

        let result = Effect::from_parts_with_value(
            id,
            span,
            metadata,
            test_identity(),
            "",
        );

        assert_eq!(result, Err(EffectError::EmptyValue));
    }

    #[test]
    fn wrong_node_kind_is_rejected() {
        let node = Node::new(
            NodeId::default(),
            NodeKind::Core(CoreNodeKind::Function),
            Span::default(),
            NodeMetadata::default(),
        );

        let result = Effect::new(
            node,
            test_identity(),
        );

        assert!(matches!(
            result,
            Err(EffectError::InvalidNodeKind { .. })
        ));
    }

    #[test]
    fn diagnostic_summary_excludes_value() {
        let effect = Effect::from_parts_with_value(
            NodeId::default(),
            Span::default(),
            NodeMetadata::default(),
            test_identity(),
            "large-extension-payload",
        )
        .expect("effect must be valid");

        let summary = effect.diagnostic_summary();

        assert_eq!(
            summary.identity.qualified_name(),
            "zamani:io"
        );
    }

    #[test]
    fn quantum_namespace_is_only_an_identity_query() {
        let effect = Effect::from_parts(
            NodeId::default(),
            Span::default(),
            NodeMetadata::default(),
            EffectIdentity::new(
                "quantum",
                "measurement",
            )
            .expect("quantum effect must be valid"),
        );

        assert!(effect.is_quantum_namespace());
        assert!(!effect.identity().is_namespace("hardware"));
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            Effect::schema_version(),
            EFFECT_AST_SCHEMA_VERSION
        );
    }

    #[test]
    fn equality_is_source_data_based() {
        let identity = test_identity();

        let first = Effect::from_parts(
            NodeId::default(),
            Span::default(),
            NodeMetadata::default(),
            identity.clone(),
        );

        let second = Effect::from_parts(
            NodeId::default(),
            Span::default(),
            NodeMetadata::default(),
            identity,
        );

        assert_eq!(first, second);
    }
}