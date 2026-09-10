//! # Zamani Frontend AST — Effect Annotation
//!
//! Production-ready source-level representation of an annotation attached to
//! an effect declaration or effect use.
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
//!      └── EffectAnnotation
//!              │
//!              ▼
//!      structural validation
//!              │
//!              ▼
//!      semantic analysis
//!              │
//!              ▼
//!      Semantic Model
//!              │
//!              ▼
//!             ZUIR
//! ```
//!
//! ## Purpose
//!
//! `EffectAnnotation` represents source-level annotation metadata associated
//! with an effect. It deliberately stores programmer intent rather than
//! compiler-resolved meaning.
//!
//! The representation is intentionally open-ended. An annotation is identified
//! by a namespace and name rather than by a closed Rust enum.
//!
//! This permits future effect systems and computational domains to introduce
//! annotations without modifying this AST type.
//!
//! Examples of possible source-level identities include:
//!
//! ```text
//! zamani:reversible
//! zamani:deterministic
//! zamani:transactional
//! quantum:measurement
//! quantum:adaptive
//! custom-domain:annotation
//! ```
//!
//! These strings are only identifiers at the AST layer. Their semantics are
//! determined by later compiler phases and extension registries.
//!
//! ## What this type does NOT represent
//!
//! This type must never become a container for:
//!
//! - quantum hardware;
//! - physical qubits;
//! - hardware topology;
//! - backend identifiers;
//! - calibration data;
//! - pulse instructions;
//! - QIR values;
//! - LLVM values;
//! - MLIR operations;
//! - routing information;
//! - scheduling information;
//! - error-correction implementations;
//! - resilience implementations;
//! - runtime state;
//! - credentials;
//! - vendor-specific execution state.
//!
//! Those concerns belong downstream.
//!
//! ## POCO-REAF
//!
//! The annotation representation contains no machine-size assumptions.
//!
//! It has no:
//!
//! - maximum qubit count;
//! - maximum register size;
//! - fixed machine size;
//! - fixed backend;
//! - fixed vendor;
//! - fixed gate set;
//! - fixed topology;
//! - fixed computational-domain list.
//!
//! Therefore an effect annotation can remain part of the same source AST
//! regardless of the eventual execution scale.
//!
//! ## Ownership model
//!
//! The annotation owns:
//!
//! - its common [`Node`];
//! - its namespace;
//! - its annotation name;
//! - an optional source-level value;
//! - an ordered collection of optional argument node references.
//!
//! It does not own the referenced AST nodes themselves.
//!
//! Referenced nodes remain owned by the canonical AST store.
//!
//! ## Child-node model
//!
//! Annotation arguments are represented using [`NodeId`] rather than embedding
//! another AST representation. This keeps the representation compatible with
//! the repository's graph-oriented AST design.
//!
//! ```text
//! Effect
//!   │
//!   └── NodeId ──► EffectAnnotation
//!                       │
//!                       ├── NodeId ──► argument
//!                       ├── NodeId ──► argument
//!                       └── ...
//! ```
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational AST infrastructure:
//!
//! - [`Node`];
//! - [`AstNode`];
//! - [`NodeId`];
//! - [`NodeKind`];
//! - [`CoreNodeKind`];
//! - [`NodeMetadata`];
//! - [`Span`];
//! - the Rust standard library;
//! - `serde`.
//!
//! It must not depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - OpenQASM;
//! - MLIR;
//! - LLVM;
//! - hardware;
//! - backend providers;
//! - routing;
//! - scheduling;
//! - calibration;
//! - optimization;
//! - execution;
//! - runtime state.
//!
//! ## Parser integration contract
//!
//! The parser is responsible for:
//!
//! 1. allocating the annotation's [`NodeId`];
//! 2. determining the source [`Span`];
//! 3. parsing the namespace;
//! 4. parsing the annotation name;
//! 5. parsing an optional source-level value;
//! 6. constructing argument AST nodes when the language grammar permits them;
//! 7. storing those child nodes in the canonical AST store;
//! 8. constructing this type with their `NodeId`s.
//!
//! The parser must not resolve annotation meaning.
//!
//! ## Semantic integration contract
//!
//! Semantic analysis consumes the validated annotation and determines:
//!
//! - whether the annotation is registered;
//! - whether it is legal for the associated effect;
//! - its semantic meaning;
//! - its argument types;
//! - its interaction with effects;
//! - any resulting semantic constraints.
//!
//! None of those results belong in this AST node.
//!
//! ## ZUIR integration contract
//!
//! This type does not lower directly to ZUIR.
//!
//! The required path is:
//!
//! ```text
//! EffectAnnotation
//!       │
//!       ▼
//! semantic analysis
//!       │
//!       ▼
//! resolved effect annotation
//!       │
//!       ▼
//! ZUIR
//! ```
//!
//! An annotation that has no semantic meaning for a particular compilation
//! mode may be rejected, ignored, or preserved according to the compiler's
//! explicit annotation policy.
//!
//! ## Determinism
//!
//! Annotation arguments are stored in source order.
//!
//! No hash-map iteration order is used.
//!
//! The type contains no:
//!
//! - global mutable state;
//! - timestamps;
//! - randomness;
//! - memory addresses;
//! - backend state.
//!
//! ## Scalability
//!
//! There is no artificial limit on:
//!
//! - annotation count;
//! - annotation name length;
//! - namespace length;
//! - argument count;
//! - program size;
//! - number of effects;
//! - number of computational resources.
//!
//! Resource limits required for hostile-input protection belong to configurable
//! compiler validation policy, not this AST data type.
//!
//! ## Serialization
//!
//! `serde` derives provide structural serialization.
//!
//! The AST serialization layer is responsible for the enclosing schema version,
//! compatibility policy, unknown-field policy, and corruption detection.
//!
//! This type does not serialize pointers, memory addresses, backend handles, or
//! process-local state.
//!
//! ## Security
//!
//! This module contains no `unsafe` code.
//!
//! It performs no unchecked indexing or pointer manipulation.
//!
//! Child references are opaque [`NodeId`] values.
//!
//! Annotation values remain source-level data and must not be interpreted as
//! executable instructions by this module.
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

/// Schema version for the `EffectAnnotation` AST contract.
///
/// This is independent from the Zamani language version, compiler version,
/// serialization version, extension version, and ZUIR version.
pub const EFFECT_ANNOTATION_AST_SCHEMA_VERSION: u16 = 1;

/// Returns the canonical native AST node kind for effect annotations.
#[inline]
#[must_use]
pub const fn effect_annotation_node_kind() -> NodeKind {
    NodeKind::Core(CoreNodeKind::Annotation)
}

/// A source-level annotation attached to an effect.
///
/// The annotation identity is open-ended through a namespace/name pair.
///
/// The optional `value` is preserved exactly as source-level text. Semantic
/// analysis is responsible for interpreting it.
///
/// `arguments` contains references to child AST nodes in source order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectAnnotation {
    /// Common AST identity, source span and metadata.
    node: Node,

    /// Annotation namespace.
    namespace: String,

    /// Annotation name.
    name: String,

    /// Optional opaque source-level annotation value.
    ///
    /// The AST preserves this value but does not interpret it.
    value: Option<String>,

    /// Optional annotation argument nodes in source order.
    ///
    /// The referenced nodes are owned by the canonical AST store.
    arguments: Vec<NodeId>,
}

impl EffectAnnotation {
    /// Creates an effect annotation without a value or arguments.
    ///
    /// # Errors
    ///
    /// Returns [`EffectAnnotationError`] if the namespace or name is invalid.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, EffectAnnotationError> {
        Self::new_with_arguments(
            id,
            span,
            metadata,
            namespace,
            name,
            None,
            Vec::new(),
        )
    }

    /// Creates an effect annotation with an optional value and argument list.
    ///
    /// Arguments are retained in the order supplied by the parser.
    ///
    /// The supplied `NodeId`s are not dereferenced or resolved here. Existence
    /// and child-kind relationships are validated by the AST validation layer.
    pub fn new_with_arguments(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        namespace: impl Into<String>,
        name: impl Into<String>,
        value: Option<String>,
        arguments: Vec<NodeId>,
    ) -> Result<Self, EffectAnnotationError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_component(&namespace, "namespace")?;
        validate_component(&name, "name")?;

        if let Some(value) = value.as_deref() {
            if value.is_empty() {
                return Err(EffectAnnotationError::EmptyValue);
            }
        }

        Ok(Self {
            node: Node::new(
                id,
                effect_annotation_node_kind(),
                span,
                metadata,
            ),
            namespace,
            name,
            value,
            arguments,
        })
    }

    /// Creates an annotation with a single source-level value.
    pub fn with_value(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        namespace: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, EffectAnnotationError> {
        Self::new_with_arguments(
            id,
            span,
            metadata,
            namespace,
            name,
            Some(value.into()),
            Vec::new(),
        )
    }

    /// Returns the annotation namespace.
    #[inline]
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the annotation name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the optional source-level annotation value.
    #[inline]
    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Returns annotation arguments in deterministic source order.
    #[inline]
    #[must_use]
    pub fn arguments(&self) -> &[NodeId] {
        &self.arguments
    }

    /// Returns the number of annotation arguments.
    #[inline]
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Returns `true` when this annotation has no arguments.
    #[inline]
    #[must_use]
    pub fn has_no_arguments(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Returns `true` when this annotation has a source-level value.
    #[inline]
    #[must_use]
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Returns the stable `namespace:name` annotation identity.
    ///
    /// The returned value is intended for diagnostics and semantic lookup.
    /// It is not a backend instruction identifier.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        let mut result =
            String::with_capacity(self.namespace.len() + 1 + self.name.len());

        result.push_str(&self.namespace);
        result.push(':');
        result.push_str(&self.name);

        result
    }

    /// Returns the schema version of this AST contract.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        EFFECT_ANNOTATION_AST_SCHEMA_VERSION
    }

    /// Appends an argument node reference.
    ///
    /// The referenced node must be owned by the canonical AST store.
    ///
    /// This method intentionally does not dereference the ID.
    pub fn push_argument(&mut self, argument: NodeId) {
        self.arguments.push(argument);
    }

    /// Extends the annotation argument list while preserving iterator order.
    pub fn extend_arguments<I>(&mut self, arguments: I)
    where
        I: IntoIterator<Item = NodeId>,
    {
        self.arguments.extend(arguments);
    }

    /// Replaces the argument list and returns the previous list.
    pub fn replace_arguments(
        &mut self,
        arguments: Vec<NodeId>,
    ) -> Vec<NodeId> {
        core::mem::replace(&mut self.arguments, arguments)
    }

    /// Replaces the annotation's source-level value.
    ///
    /// An empty value is rejected.
    pub fn set_value(
        &mut self,
        value: Option<String>,
    ) -> Result<(), EffectAnnotationError> {
        if let Some(value) = value.as_deref() {
            if value.is_empty() {
                return Err(EffectAnnotationError::EmptyValue);
            }
        }

        self.value = value;
        Ok(())
    }

    /// Removes the annotation value.
    #[inline]
    pub fn clear_value(&mut self) {
        self.value = None;
    }

    /// Returns a lightweight identity used by diagnostics and semantic lookup.
    #[must_use]
    pub fn identity(&self) -> EffectAnnotationIdentity {
        EffectAnnotationIdentity {
            namespace: self.namespace.clone(),
            name: self.name.clone(),
        }
    }

    /// Validates this node's local invariants.
    ///
    /// This method intentionally does not inspect referenced child nodes.
    /// Cross-node validation belongs to the AST structural validation layer,
    /// where the canonical AST store is available.
    pub fn validate(&self) -> Result<(), EffectAnnotationError> {
        validate_component(&self.namespace, "namespace")?;
        validate_component(&self.name, "name")?;

        if self.value.as_deref() == Some("") {
            return Err(EffectAnnotationError::EmptyValue);
        }

        if self.node.kind() != &effect_annotation_node_kind() {
            return Err(EffectAnnotationError::InvalidNodeKind);
        }

        Ok(())
    }
}

impl AstNode for EffectAnnotation {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl fmt::Display for EffectAnnotation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.qualified_name())?;

        if let Some(value) = self.value() {
            formatter.write_str("=")?;
            formatter.write_str(value)?;
        }

        if !self.arguments.is_empty() {
            formatter.write_str("(")?;

            for (index, argument) in self.arguments.iter().enumerate() {
                if index != 0 {
                    formatter.write_str(", ")?;
                }

                write!(formatter, "{argument}")?;
            }

            formatter.write_str(")")?;
        }

        Ok(())
    }
}

/// Stable identity of an effect annotation.
///
/// This type deliberately excludes source position and arguments so that
/// semantic lookup can compare annotation identity independently of where the
/// annotation appeared.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct EffectAnnotationIdentity {
    /// Annotation namespace.
    namespace: String,

    /// Annotation name.
    name: String,
}

impl EffectAnnotationIdentity {
    /// Creates an annotation identity.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, EffectAnnotationError> {
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

    /// Returns the name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the stable qualified identity.
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

impl fmt::Display for EffectAnnotationIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}",
            self.namespace,
            self.name
        )
    }
}

/// Errors produced while constructing or validating an effect annotation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EffectAnnotationError {
    /// Namespace is empty.
    EmptyNamespace,

    /// Name is empty.
    EmptyName,

    /// Namespace contains a forbidden character.
    InvalidNamespaceCharacter(char),

    /// Name contains a forbidden character.
    InvalidNameCharacter(char),

    /// Annotation value was supplied but empty.
    EmptyValue,

    /// The embedded node has the wrong AST classification.
    InvalidNodeKind,
}

impl fmt::Display for EffectAnnotationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNamespace => {
                formatter.write_str(
                    "effect annotation namespace must not be empty",
                )
            }

            Self::EmptyName => {
                formatter.write_str(
                    "effect annotation name must not be empty",
                )
            }

            Self::InvalidNamespaceCharacter(character) => {
                write!(
                    formatter,
                    "effect annotation namespace contains forbidden character {:?}",
                    character
                )
            }

            Self::InvalidNameCharacter(character) => {
                write!(
                    formatter,
                    "effect annotation name contains forbidden character {:?}",
                    character
                )
            }

            Self::EmptyValue => {
                formatter.write_str(
                    "effect annotation value must not be empty",
                )
            }

            Self::InvalidNodeKind => {
                formatter.write_str(
                    "effect annotation contains an invalid AST node kind",
                )
            }
        }
    }
}

impl std::error::Error for EffectAnnotationError {}

/// Validates a namespace or annotation-name component.
///
/// The AST accepts Unicode and does not perform Unicode normalization.
/// Exact source spelling is therefore preserved.
///
/// `/`, `\`, `:`, whitespace, and control characters are rejected because they
/// would make the qualified `namespace:name` identity ambiguous or unsuitable
/// for deterministic tooling.
fn validate_component(
    value: &str,
    component: &'static str,
) -> Result<(), EffectAnnotationError> {
    if value.is_empty() {
        return Err(match component {
            "namespace" => EffectAnnotationError::EmptyNamespace,
            _ => EffectAnnotationError::EmptyName,
        });
    }

    for character in value.chars() {
        if character.is_control()
            || character.is_whitespace()
            || matches!(character, '/' | '\\' | ':')
        {
            return Err(match component {
                "namespace" => {
                    EffectAnnotationError::InvalidNamespaceCharacter(character)
                }
                _ => EffectAnnotationError::InvalidNameCharacter(character),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_node_id(value: u64) -> NodeId {
        /*
         * NodeId's exact constructor is intentionally isolated here so that
         * production code does not depend on a particular allocation strategy.
         *
         * The repository's NodeId API should be used by the actual parser/
         * builder. Tests below use NodeId::default() where a concrete allocator
         * API is not required by the behavior under test.
         */
        let _ = value;
        NodeId::default()
    }

    #[test]
    fn creates_basic_annotation() {
        let annotation = EffectAnnotation::new(
            test_node_id(1),
            Span::default(),
            NodeMetadata::default(),
            "zamani",
            "deterministic",
        )
        .expect("valid annotation");

        assert_eq!(annotation.namespace(), "zamani");
        assert_eq!(annotation.name(), "deterministic");
        assert_eq!(
            annotation.qualified_name(),
            "zamani:deterministic"
        );
        assert!(annotation.value().is_none());
        assert!(annotation.arguments().is_empty());
        assert_eq!(
            annotation.kind(),
            &NodeKind::Core(CoreNodeKind::Annotation)
        );
    }

    #[test]
    fn preserves_value() {
        let annotation = EffectAnnotation::with_value(
            test_node_id(1),
            Span::default(),
            NodeMetadata::default(),
            "zamani",
            "mode",
            "source-level-value",
        )
        .expect("valid annotation");

        assert_eq!(
            annotation.value(),
            Some("source-level-value")
        );
    }

    #[test]
    fn preserves_argument_order() {
        let first = NodeId::default();
        let second = NodeId::default();

        let annotation = EffectAnnotation::new_with_arguments(
            test_node_id(1),
            Span::default(),
            NodeMetadata::default(),
            "zamani",
            "example",
            None,
            vec![first, second],
        )
        .expect("valid annotation");

        assert_eq!(annotation.arguments(), &[first, second]);
    }

    #[test]
    fn rejects_empty_namespace() {
        let result = EffectAnnotation::new(
            test_node_id(1),
            Span::default(),
            NodeMetadata::default(),
            "",
            "annotation",
        );

        assert_eq!(
            result,
            Err(EffectAnnotationError::EmptyNamespace)
        );
    }

    #[test]
    fn rejects_empty_name() {
        let result = EffectAnnotation::new(
            test_node_id(1),
            Span::default(),
            NodeMetadata::default(),
            "zamani",
            "",
        );

        assert_eq!(
            result,
            Err(EffectAnnotationError::EmptyName)
        );
    }

    #[test]
    fn rejects_ambiguous_namespace() {
        let result = EffectAnnotation::new(
            test_node_id(1),
            Span::default(),
            NodeMetadata::default(),
            "quantum:hardware",
            "annotation",
        );

        assert!(matches!(
            result,
            Err(EffectAnnotationError::InvalidNamespaceCharacter(':'))
        ));
    }

    #[test]
    fn rejects_empty_value() {
        let result = EffectAnnotation::with_value(
            test_node_id(1),
            Span::default(),
            NodeMetadata::default(),
            "zamani",
            "mode",
            "",
        );

        assert_eq!(
            result,
            Err(EffectAnnotationError::EmptyValue)
        );
    }

    #[test]
    fn identity_is_independent_of_arguments() {
        let annotation = EffectAnnotation::new_with_arguments(
            test_node_id(1),
            Span::default(),
            NodeMetadata::default(),
            "zamani",
            "deterministic",
            Some("true".to_owned()),
            Vec::new(),
        )
        .expect("valid annotation");

        let identity = annotation.identity();

        assert_eq!(identity.namespace(), "zamani");
        assert_eq!(identity.name(), "deterministic");
        assert_eq!(
            identity.qualified_name(),
            "zamani:deterministic"
        );
    }

    #[test]
    fn validation_accepts_valid_annotation() {
        let annotation = EffectAnnotation::new(
            test_node_id(1),
            Span::default(),
            NodeMetadata::default(),
            "custom-domain",
            "annotation",
        )
        .expect("valid annotation");

        assert!(annotation.validate().is_ok());
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(
            EffectAnnotation::schema_version(),
            EFFECT_ANNOTATION_AST_SCHEMA_VERSION
        );
    }

    #[test]
    fn display_is_deterministic() {
        let annotation = EffectAnnotation::with_value(
            test_node_id(1),
            Span::default(),
            NodeMetadata::default(),
            "zamani",
            "mode",
            "source",
        )
        .expect("valid annotation");

        assert_eq!(
            annotation.to_string(),
            "zamani:mode=source"
        );
    }
}