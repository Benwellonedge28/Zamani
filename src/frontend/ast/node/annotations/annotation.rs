//! # Zamani Frontend AST — Generic Annotation
//!
//! Production-ready source-level representation of a Zamani annotation.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//!    lexer
//!      │
//!      ▼
//!    parser
//!      │
//!      ▼
//! ┌──────────────────────────────┐
//! │ Native Zamani AST            │
//! │                              │
//! │ Generic Annotation           │
//! └──────────────┬───────────────┘
//!                │
//!                ▼
//!       structural validation
//!                │
//!                ▼
//!        semantic analysis
//!                │
//!                ▼
//!         Semantic Model
//!                │
//!                ▼
//!               ZUIR
//!                │
//!        ┌───────┼────────┐
//!        ▼       ▼        ▼
//!     classical quantum   HDL
//!       IR       IR       IR
//!                │
//!                ▼
//!       target / backend
//! ```
//!
//! ## Purpose
//!
//! `Annotation` represents source-level programmer intent attached to another
//! AST construct.
//!
//! An annotation is intentionally open-ended:
//!
//! - identity is namespaced;
//! - the annotation name is not a closed Rust enum;
//! - values remain source-level data;
//! - expression/AST references are represented by `NodeId`;
//! - semantic interpretation belongs to later compiler phases.
//!
//! This is essential for POCO-REAF:
//!
//! ```text
//! Program Once
//!      │
//!      ▼
//! Native AST
//!      │
//!      ▼
//! Semantic meaning
//!      │
//!      ▼
//! ZUIR
//!      │
//!      ▼
//! target/resource discovery
//!      │
//!      ▼
//! mapping / optimization / scheduling / execution
//! ```
//!
//! The annotation itself never contains assumptions about the eventual
//! machine, processor, QPU, accelerator, topology, vendor, instruction set,
//! register count, qubit count, or execution environment.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - annotation identity;
//! - annotation value;
//! - annotation argument references;
//! - source span;
//! - AST metadata;
//! - AST node identity;
//! - source-level provenance.
//!
//! It does NOT own:
//!
//! - semantic meaning;
//! - resolved symbols;
//! - resolved types;
//! - resource allocation;
//! - physical resources;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - backend execution;
//! - runtime state;
//! - credentials;
//! - vendor-specific behavior.
//!
//! ## Domain neutrality
//!
//! The representation deliberately does not contain variants such as:
//!
//! ```text
//! QuantumAnnotation
//! IBMAnnotation
//! QIRAnnotation
//! GPUAnnotation
//! HDLAnnotation
//! ```
//!
//! A namespace identifies an annotation family instead:
//!
//! ```text
//! zamani::inline
//! quantum::measurement
//! quantum::adaptive
//! hardware::constraint
//! user::custom_annotation
//! ```
//!
//! These are identifiers only at this AST layer. Their meaning is established
//! by the semantic/extension registry.
//!
//! ## Integration contract
//!
//! ```text
//! parser
//!   │
//!   ├── NodeId
//!   ├── Span
//!   ├── namespace
//!   ├── name
//!   ├── source-level value
//!   └── NodeId argument references
//!          │
//!          ▼
//!      Annotation
//!          │
//!          ▼
//! structural validation
//!          │
//!          ▼
//! semantic annotation registry
//!          │
//!          ▼
//! resolved annotation semantics
//!          │
//!          ▼
//! Semantic Model
//!          │
//!          ▼
//! ZUIR / domain lowering
//! ```
//!
//! The parser must not resolve annotation semantics.
//!
//! Structural validation must not perform hardware validation.
//!
//! Semantic analysis determines whether an annotation is known, legal,
//! repeatable, well-typed, and meaningful for its target.
//!
//! ## Dependency contract
//!
//! This module may depend only on:
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
//! It must NOT depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - OpenQASM;
//! - MLIR;
//! - LLVM;
//! - hardware;
//! - backends;
//! - routing;
//! - scheduling;
//! - calibration;
//! - optimization;
//! - execution.
//!
//! ## Scalability
//!
//! This type contains no language-level limit for:
//!
//! - annotation count;
//! - annotation name length;
//! - namespace length;
//! - argument count;
//! - value size;
//! - AST size;
//! - machine size;
//! - resource count;
//! - qubit count.
//!
//! Collections are dynamically sized.
//!
//! Any operational limit required for hostile-input protection belongs to the
//! configurable AST/compiler validation policy, not to this representation.
//!
//! ## Determinism
//!
//! Annotation arguments retain source order.
//!
//! No hash map is used for source-order data.
//!
//! The type contains no:
//!
//! - global mutable state;
//! - timestamps;
//! - randomness;
//! - pointers;
//! - memory addresses;
//! - backend handles.
//!
//! ## Serialization
//!
//! `serde` serialization is structural. The enclosing AST serialization layer
//! owns the global serialization schema/version/compatibility policy.
//!
//! This module never serializes:
//!
//! - pointers;
//! - memory addresses;
//! - runtime handles;
//! - hardware state;
//! - backend credentials.
//!
//! ## Safety
//!
//! This file contains no `unsafe` code.
//!
//! No unchecked indexing is used.
//!
//! No raw pointers are used.
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

use core::fmt;
use std::error::Error;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// In-memory schema version for the generic annotation AST node.
///
/// This is intentionally independent of:
///
/// - Zamani language version;
/// - compiler version;
/// - serialized AST version;
/// - extension version;
/// - ZUIR version.
pub const ANNOTATION_AST_SCHEMA_VERSION: u16 = 1;

/// Returns the canonical native AST node kind for annotations.
#[inline]
#[must_use]
pub const fn annotation_node_kind() -> NodeKind {
    NodeKind::Core(CoreNodeKind::Annotation)
}

/// A source-level, namespaced annotation identity.
///
/// The identity is deliberately open-ended rather than represented by a
/// closed enum. This allows new language and domain extensions without
/// modifying the native AST.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AnnotationId {
    namespace: String,
    name: String,
}

impl AnnotationId {
    /// Creates a validated annotation identity.
    ///
    /// Namespace and name must both be non-empty and must not contain
    /// whitespace at their boundaries or control characters.
    pub fn new<N, S>(
        namespace: N,
        name: S,
    ) -> Result<Self, AnnotationIdError>
    where
        N: Into<String>,
        S: Into<String>,
    {
        let namespace = namespace.into();
        let name = name.into();

        validate_identifier_component("namespace", &namespace)?;
        validate_identifier_component("name", &name)?;

        Ok(Self { namespace, name })
    }

    /// Returns the namespace.
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

    /// Returns the canonical `namespace::name` identity.
    #[must_use]
    pub fn canonical_name(&self) -> String {
        let capacity = self
            .namespace
            .len()
            .saturating_add(2)
            .saturating_add(self.name.len());

        let mut result = String::with_capacity(capacity);
        result.push_str(&self.namespace);
        result.push_str("::");
        result.push_str(&self.name);
        result
    }

    /// Consumes the identity and returns its components.
    #[must_use]
    pub fn into_parts(self) -> (String, String) {
        (self.namespace, self.name)
    }
}

impl fmt::Display for AnnotationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}::{}",
            self.namespace,
            self.name
        )
    }
}

/// Source-level value carried by an annotation.
///
/// Values deliberately preserve lexical information where selecting a
/// machine representation too early would reduce portability.
///
/// In particular, integers and reals are represented as strings rather than
/// `i64`, `u64`, or `f64`. This avoids imposing a host-dependent numeric
/// representation on the source AST.
///
/// Expression values use `NodeId`, keeping this module independent from the
/// concrete expression AST implementation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnotationValue {
    /// No explicit value.
    Unit,

    /// Boolean source value.
    Bool(bool),

    /// Integer whose lexical representation is preserved.
    Integer(String),

    /// Real/decimal value whose lexical representation is preserved.
    Real(String),

    /// String source value.
    String(String),

    /// Symbolic source-level identifier/value.
    Symbol(String),

    /// Reference to another AST node.
    Expression(NodeId),

    /// Ordered annotation value sequence.
    List(Vec<AnnotationValue>),

    /// Ordered annotation object.
    ///
    /// A vector is intentional: annotation object field order is preserved
    /// deterministically and duplicate fields remain representable for
    /// diagnostics. Semantic analysis decides whether duplicates are legal.
    Object(Vec<AnnotationField>),
}

impl AnnotationValue {
    /// Creates a source-preserving integer value.
    #[inline]
    pub fn integer<S: Into<String>>(value: S) -> Self {
        Self::Integer(value.into())
    }

    /// Creates a source-preserving real value.
    #[inline]
    pub fn real<S: Into<String>>(value: S) -> Self {
        Self::Real(value.into())
    }

    /// Creates a symbolic value.
    #[inline]
    pub fn symbol<S: Into<String>>(value: S) -> Self {
        Self::Symbol(value.into())
    }

    /// Creates an AST-expression reference.
    #[inline]
    #[must_use]
    pub const fn expression(node: NodeId) -> Self {
        Self::Expression(node)
    }

    /// Returns whether this is the unit/no-value form.
    #[inline]
    #[must_use]
    pub const fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }

    /// Returns the number of immediate children.
    ///
    /// This operation does not recursively traverse the value and therefore
    /// remains predictable for very large annotation values.
    #[must_use]
    pub fn child_count(&self) -> usize {
        match self {
            Self::Unit
            | Self::Bool(_)
            | Self::Integer(_)
            | Self::Real(_)
            | Self::String(_)
            | Self::Symbol(_)
            | Self::Expression(_) => 0,

            Self::List(values) => values.len(),
            Self::Object(fields) => fields.len(),
        }
    }
}

/// A named field within an annotation object value.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnnotationField {
    name: String,
    value: AnnotationValue,
}

impl AnnotationField {
    /// Creates a validated annotation object field.
    pub fn new<N>(
        name: N,
        value: AnnotationValue,
    ) -> Result<Self, AnnotationFieldError>
    where
        N: Into<String>,
    {
        let name = name.into();

        validate_identifier_component("field", &name)
            .map_err(AnnotationFieldError::InvalidName)?;

        Ok(Self { name, value })
    }

    /// Returns the field name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the field value.
    #[inline]
    #[must_use]
    pub fn value(&self) -> &AnnotationValue {
        &self.value
    }

    /// Returns mutable access to the field value.
    #[inline]
    pub fn value_mut(&mut self) -> &mut AnnotationValue {
        &mut self.value
    }

    /// Consumes the field and returns `(name, value)`.
    #[must_use]
    pub fn into_parts(self) -> (String, AnnotationValue) {
        (self.name, self.value)
    }
}

/// Source/AST provenance of an annotation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnotationOrigin {
    /// Written directly in Zamani source.
    Source,

    /// Created by a compiler transformation.
    Generated,

    /// Imported from another source representation.
    Imported,

    /// Created by macro/source expansion.
    Expanded,
}

/// A generic source-level Zamani annotation.
///
/// This is the canonical generic annotation representation.
///
/// It can be attached to:
///
/// - declarations;
/// - functions;
/// - variables;
/// - types;
/// - expressions;
/// - statements;
/// - resources;
/// - effects;
/// - capabilities;
/// - domains;
/// - extension-defined constructs.
///
/// It does not decide what the annotation means.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Annotation {
    /// Common source-level AST identity, kind, span, and metadata.
    node: Node,

    /// Namespaced annotation identity.
    id: AnnotationId,

    /// Optional source-level annotation value.
    value: AnnotationValue,

    /// Ordered references to child AST nodes used by the annotation.
    ///
    /// The canonical AST store owns the referenced nodes.
    arguments: Vec<NodeId>,

    /// Source/provenance classification.
    origin: AnnotationOrigin,
}

impl Annotation {
    /// Creates an annotation without arguments.
    pub fn new(
        node_id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        id: AnnotationId,
    ) -> Self {
        Self {
            node: Node::new(
                node_id,
                annotation_node_kind(),
                span,
                metadata,
            ),
            id,
            value: AnnotationValue::Unit,
            arguments: Vec::new(),
            origin: AnnotationOrigin::Source,
        }
    }

    /// Creates an annotation with a value.
    #[must_use]
    pub fn with_value(
        node_id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        id: AnnotationId,
        value: AnnotationValue,
    ) -> Self {
        Self {
            node: Node::new(
                node_id,
                annotation_node_kind(),
                span,
                metadata,
            ),
            id,
            value,
            arguments: Vec::new(),
            origin: AnnotationOrigin::Source,
        }
    }

    /// Creates a fully specified annotation.
    #[must_use]
    pub fn with_arguments(
        node_id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        id: AnnotationId,
        value: AnnotationValue,
        arguments: Vec<NodeId>,
        origin: AnnotationOrigin,
    ) -> Self {
        Self {
            node: Node::new(
                node_id,
                annotation_node_kind(),
                span,
                metadata,
            ),
            id,
            value,
            arguments,
            origin,
        }
    }

    /// Returns the annotation identity.
    #[inline]
    #[must_use]
    pub fn annotation_id(&self) -> &AnnotationId {
        &self.id
    }

    /// Returns the namespace.
    #[inline]
    #[must_use]
    pub fn namespace(&self) -> &str {
        self.id.namespace()
    }

    /// Returns the annotation name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        self.id.name()
    }

    /// Returns the canonical `namespace::name` identity.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        self.id.canonical_name()
    }

    /// Returns the source-level value.
    #[inline]
    #[must_use]
    pub fn value(&self) -> &AnnotationValue {
        &self.value
    }

    /// Returns mutable access to the source-level value.
    #[inline]
    pub fn value_mut(&mut self) -> &mut AnnotationValue {
        &mut self.value
    }

    /// Replaces the value.
    #[inline]
    pub fn set_value(&mut self, value: AnnotationValue) {
        self.value = value;
    }

    /// Returns annotation arguments in source order.
    #[inline]
    #[must_use]
    pub fn arguments(&self) -> &[NodeId] {
        &self.arguments
    }

    /// Returns mutable annotation arguments in source order.
    #[inline]
    pub fn arguments_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.arguments
    }

    /// Returns the number of annotation arguments.
    #[inline]
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Returns whether this annotation has no argument references.
    #[inline]
    #[must_use]
    pub fn has_no_arguments(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Appends one argument reference.
    ///
    /// The referenced node is not dereferenced here. The canonical AST
    /// validation layer is responsible for checking that the ID exists and
    /// that the referenced node is legal for this annotation.
    #[inline]
    pub fn push_argument(&mut self, argument: NodeId) {
        self.arguments.push(argument);
    }

    /// Appends multiple argument references while preserving source order.
    pub fn extend_arguments<I>(&mut self, arguments: I)
    where
        I: IntoIterator<Item = NodeId>,
    {
        self.arguments.extend(arguments);
    }

    /// Replaces the complete argument list.
    ///
    /// Returns the previous list.
    pub fn replace_arguments(
        &mut self,
        arguments: Vec<NodeId>,
    ) -> Vec<NodeId> {
        std::mem::replace(&mut self.arguments, arguments)
    }

    /// Returns the annotation origin.
    #[inline]
    #[must_use]
    pub const fn origin(&self) -> AnnotationOrigin {
        self.origin
    }

    /// Changes the annotation origin.
    #[inline]
    pub const fn set_origin(&mut self, origin: AnnotationOrigin) {
        self.origin = origin;
    }

    /// Returns this annotation's schema version.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        ANNOTATION_AST_SCHEMA_VERSION
    }

    /// Validates invariants owned directly by this node.
    ///
    /// This intentionally does not resolve child `NodeId`s. Cross-node
    /// validation belongs to the canonical AST validator.
    pub fn validate(&self) -> Result<(), AnnotationValidationError> {
        self.id
            .validate()
            .map_err(AnnotationValidationError::InvalidIdentity)?;

        if self.node.kind() != &annotation_node_kind() {
            return Err(AnnotationValidationError::InvalidNodeKind);
        }

        validate_annotation_value(&self.value)?;

        Ok(())
    }

    /// Returns a deterministic source-level identity/value summary.
    ///
    /// This is suitable for diagnostics without exposing the internal `Node`.
    #[must_use]
    pub fn identity(&self) -> AnnotationIdentity {
        AnnotationIdentity {
            namespace: self.namespace().to_owned(),
            name: self.name().to_owned(),
        }
    }
}

impl AstNode for Annotation {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl fmt::Display for Annotation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.qualified_name())?;

        if !self.value.is_unit() {
            formatter.write_str("=")?;

            match &self.value {
                AnnotationValue::Unit => {}

                AnnotationValue::Bool(value) => {
                    write!(formatter, "{value}")?;
                }

                AnnotationValue::Integer(value)
                | AnnotationValue::Real(value)
                | AnnotationValue::String(value)
                | AnnotationValue::Symbol(value) => {
                    formatter.write_str(value)?;
                }

                AnnotationValue::Expression(node_id) => {
                    write!(formatter, "${node_id}")?;
                }

                AnnotationValue::List(values) => {
                    formatter.write_str("[")?;

                    for (index, value) in values.iter().enumerate() {
                        if index != 0 {
                            formatter.write_str(", ")?;
                        }

                        fmt_value(formatter, value)?;
                    }

                    formatter.write_str("]")?;
                }

                AnnotationValue::Object(fields) => {
                    formatter.write_str("{")?;

                    for (index, field) in fields.iter().enumerate() {
                        if index != 0 {
                            formatter.write_str(", ")?;
                        }

                        formatter.write_str(field.name())?;
                        formatter.write_str("=")?;
                        fmt_value(formatter, field.value())?;
                    }

                    formatter.write_str("}")?;
                }
            }
        }

        Ok(())
    }
}

/// Lightweight annotation identity used by diagnostics and semantic lookup.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnnotationIdentity {
    /// Namespace component.
    pub namespace: String,

    /// Name component.
    pub name: String,
}

impl fmt::Display for AnnotationIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}::{}",
            self.namespace,
            self.name
        )
    }
}

/// Ordered annotation collection.
///
/// This collection deliberately preserves source order.
///
/// Duplicate annotations are permitted at the AST layer. Whether an
/// annotation is repeatable is a semantic/extension-registry property, not a
/// structural AST property.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Annotations {
    entries: Vec<Annotation>,
}

impl Annotations {
    /// Creates an empty annotation collection.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Returns the number of annotations.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether no annotations are present.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Appends an annotation in source order.
    #[inline]
    pub fn push(&mut self, annotation: Annotation) {
        self.entries.push(annotation);
    }

    /// Extends the collection in iterator order.
    pub fn extend<I>(&mut self, annotations: I)
    where
        I: IntoIterator<Item = Annotation>,
    {
        self.entries.extend(annotations);
    }

    /// Returns immutable annotations in source order.
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Annotation> {
        self.entries.iter()
    }

    /// Returns mutable annotations in source order.
    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Annotation> {
        self.entries.iter_mut()
    }

    /// Returns the first annotation with the supplied identity.
    #[must_use]
    pub fn get(
        &self,
        id: &AnnotationId,
    ) -> Option<&Annotation> {
        self.entries
            .iter()
            .find(|annotation| annotation.annotation_id() == id)
    }

    /// Returns all annotations with the supplied identity.
    pub fn get_all<'a>(
        &'a self,
        id: &'a AnnotationId,
    ) -> impl Iterator<Item = &'a Annotation> {
        self.entries
            .iter()
            .filter(move |annotation| annotation.annotation_id() == id)
    }

    /// Removes all annotations.
    #[inline]
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Validates every annotation's local invariants.
    ///
    /// Duplicate-policy checking deliberately remains outside this method.
    pub fn validate(&self) -> Result<(), AnnotationValidationError> {
        for annotation in &self.entries {
            annotation.validate()?;
        }

        Ok(())
    }

    /// Returns the underlying annotations as a slice.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[Annotation] {
        &self.entries
    }

    /// Consumes the collection and returns its annotations.
    #[inline]
    #[must_use]
    pub fn into_vec(self) -> Vec<Annotation> {
        self.entries
    }
}

impl<'a> IntoIterator for &'a Annotations {
    type Item = &'a Annotation;
    type IntoIter = std::slice::Iter<'a, Annotation>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

impl<'a> IntoIterator for &'a mut Annotations {
    type Item = &'a mut Annotation;
    type IntoIter = std::slice::IterMut<'a, Annotation>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter_mut()
    }
}

impl IntoIterator for Annotations {
    type Item = Annotation;
    type IntoIter = std::vec::IntoIter<Annotation>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

/// Validates an annotation identifier component.
///
/// The AST does not impose a programming-language-specific identifier grammar
/// here because annotation namespaces may belong to extensions. It only
/// enforces the structural invariants necessary for stable identity.
fn validate_identifier_component(
    field: &'static str,
    value: &str,
) -> Result<(), AnnotationIdError> {
    if value.is_empty() {
        return Err(AnnotationIdError::EmptyComponent { field });
    }

    if value.trim() != value {
        return Err(AnnotationIdError::Whitespace {
            field,
            value: value.to_owned(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(AnnotationIdError::ControlCharacter {
            field,
            value: value.to_owned(),
        });
    }

    Ok(())
}

/// Validates an annotation value structurally.
///
/// This function intentionally does not recursively enforce an arbitrary
/// nesting limit. Resource limits belong to configurable compiler policy.
///
/// The traversal is iterative to avoid making annotation correctness depend on
/// the host call stack for deeply nested values.
fn validate_annotation_value(
    root: &AnnotationValue,
) -> Result<(), AnnotationValidationError> {
    let mut pending = vec![root];

    while let Some(value) = pending.pop() {
        match value {
            AnnotationValue::Unit
            | AnnotationValue::Bool(_)
            | AnnotationValue::Expression(_) => {}

            AnnotationValue::Integer(value) => {
                if value.is_empty() {
                    return Err(
                        AnnotationValidationError::EmptyNumericValue,
                    );
                }

                if value.chars().any(char::is_control) {
                    return Err(
                        AnnotationValidationError::ControlCharacterValue,
                    );
                }
            }

            AnnotationValue::Real(value) => {
                if value.is_empty() {
                    return Err(
                        AnnotationValidationError::EmptyNumericValue,
                    );
                }

                if value.chars().any(char::is_control) {
                    return Err(
                        AnnotationValidationError::ControlCharacterValue,
                    );
                }
            }

            AnnotationValue::String(value)
            | AnnotationValue::Symbol(value) => {
                if value.chars().any(char::is_control) {
                    return Err(
                        AnnotationValidationError::ControlCharacterValue,
                    );
                }
            }

            AnnotationValue::List(values) => {
                pending.extend(values.iter().rev());
            }

            AnnotationValue::Object(fields) => {
                for field in fields.iter().rev() {
                    validate_identifier_component(
                        "field",
                        field.name(),
                    )
                    .map_err(
                        AnnotationValidationError::InvalidFieldName,
                    )?;

                    pending.push(field.value());
                }
            }
        }
    }

    Ok(())
}

/// Formats an annotation value without imposing semantic interpretation.
fn fmt_value(
    formatter: &mut fmt::Formatter<'_>,
    value: &AnnotationValue,
) -> fmt::Result {
    match value {
        AnnotationValue::Unit => Ok(()),

        AnnotationValue::Bool(value) => {
            write!(formatter, "{value}")
        }

        AnnotationValue::Integer(value)
        | AnnotationValue::Real(value)
        | AnnotationValue::String(value)
        | AnnotationValue::Symbol(value) => {
            formatter.write_str(value)
        }

        AnnotationValue::Expression(node_id) => {
            write!(formatter, "${node_id}")
        }

        AnnotationValue::List(values) => {
            formatter.write_str("[")?;

            for (index, value) in values.iter().enumerate() {
                if index != 0 {
                    formatter.write_str(", ")?;
                }

                fmt_value(formatter, value)?;
            }

            formatter.write_str("]")
        }

        AnnotationValue::Object(fields) => {
            formatter.write_str("{")?;

            for (index, field) in fields.iter().enumerate() {
                if index != 0 {
                    formatter.write_str(", ")?;
                }

                formatter.write_str(field.name())?;
                formatter.write_str("=")?;
                fmt_value(formatter, field.value())?;
            }

            formatter.write_str("}")
        }
    }
}

/// Errors produced while creating an annotation identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnnotationIdError {
    /// Namespace or name was empty.
    EmptyComponent {
        /// Component name.
        field: &'static str,
    },

    /// Namespace or name contained surrounding whitespace.
    Whitespace {
        /// Component name.
        field: &'static str,

        /// Invalid value.
        value: String,
    },

    /// Namespace or name contained a control character.
    ControlCharacter {
        /// Component name.
        field: &'static str,

        /// Invalid value.
        value: String,
    },
}

impl fmt::Display for AnnotationIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyComponent { field } => {
                write!(formatter, "annotation {field} cannot be empty")
            }

            Self::Whitespace { field, value } => {
                write!(
                    formatter,
                    "annotation {field} cannot have surrounding whitespace: {value:?}"
                )
            }

            Self::ControlCharacter { field, .. } => {
                write!(
                    formatter,
                    "annotation {field} cannot contain control characters"
                )
            }
        }
    }
}

impl Error for AnnotationIdError {}

/// Errors produced while creating annotation object fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnnotationFieldError {
    /// The field name is structurally invalid.
    InvalidName(AnnotationIdError),
}

impl fmt::Display for AnnotationFieldError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(error) => {
                write!(formatter, "invalid annotation field name: {error}")
            }
        }
    }
}

impl Error for AnnotationFieldError {}

/// Errors produced by local annotation validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnnotationValidationError {
    /// Annotation identity is invalid.
    InvalidIdentity(AnnotationIdError),

    /// The embedded node does not have the annotation node kind.
    InvalidNodeKind,

    /// An integer or real value has an empty lexical representation.
    EmptyNumericValue,

    /// A source-level value contains a control character.
    ControlCharacterValue,

    /// An object field name is invalid.
    InvalidFieldName(AnnotationIdError),
}

impl fmt::Display for AnnotationValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity(error) => {
                write!(formatter, "invalid annotation identity: {error}")
            }

            Self::InvalidNodeKind => {
                write!(
                    formatter,
                    "annotation node has an invalid AST node kind"
                )
            }

            Self::EmptyNumericValue => {
                write!(
                    formatter,
                    "annotation numeric value cannot be empty"
                )
            }

            Self::ControlCharacterValue => {
                write!(
                    formatter,
                    "annotation value cannot contain control characters"
                )
            }

            Self::InvalidFieldName(error) => {
                write!(
                    formatter,
                    "invalid annotation object field name: {error}"
                )
            }
        }
    }
}

impl Error for AnnotationValidationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::node_id::NodeIdAllocator;

    fn test_node_id() -> NodeId {
        let mut allocator = NodeIdAllocator::new();
        allocator
            .allocate()
            .expect("deterministic test NodeId allocation")
    }

    #[test]
    fn annotation_identity_is_namespaced() {
        let id = AnnotationId::new(
            "quantum",
            "measurement",
        )
        .expect("valid annotation identity");

        assert_eq!(id.namespace(), "quantum");
        assert_eq!(id.name(), "measurement");
        assert_eq!(
            id.canonical_name(),
            "quantum::measurement"
        );
    }

    #[test]
    fn empty_namespace_is_rejected() {
        let result = AnnotationId::new("", "test");

        assert!(matches!(
            result,
            Err(AnnotationIdError::EmptyComponent {
                field: "namespace"
            })
        ));
    }

    #[test]
    fn empty_name_is_rejected() {
        let result = AnnotationId::new("quantum", "");

        assert!(matches!(
            result,
            Err(AnnotationIdError::EmptyComponent {
                field: "name"
            })
        ));
    }

    #[test]
    fn control_characters_are_rejected() {
        let result = AnnotationId::new(
            "quantum",
            "measurement\n",
        );

        assert!(matches!(
            result,
            Err(AnnotationIdError::ControlCharacter {
                field: "name",
                ..
            })
        ));
    }

    #[test]
    fn annotation_uses_canonical_annotation_node_kind() {
        let annotation = Annotation::new(
            test_node_id(),
            Span::default(),
            NodeMetadata::default(),
            AnnotationId::new("quantum", "adaptive")
                .expect("valid identity"),
        );

        assert_eq!(
            annotation.kind(),
            &annotation_node_kind()
        );
    }

    #[test]
    fn annotation_arguments_preserve_source_order() {
        let mut allocator = NodeIdAllocator::new();

        let annotation_id = allocator
            .allocate()
            .expect("annotation ID");

        let first = allocator
            .allocate()
            .expect("first argument ID");

        let second = allocator
            .allocate()
            .expect("second argument ID");

        let mut annotation = Annotation::new(
            annotation_id,
            Span::default(),
            NodeMetadata::default(),
            AnnotationId::new(
                "user",
                "example",
            )
            .expect("valid identity"),
        );

        annotation.push_argument(first);
        annotation.push_argument(second);

        assert_eq!(
            annotation.arguments(),
            &[first, second]
        );
    }

    #[test]
    fn duplicate_annotations_are_allowed_at_ast_level() {
        let mut allocator = NodeIdAllocator::new();

        let first = Annotation::new(
            allocator.allocate().expect("first"),
            Span::default(),
            NodeMetadata::default(),
            AnnotationId::new(
                "user",
                "repeatable",
            )
            .expect("valid identity"),
        );

        let second = Annotation::new(
            allocator.allocate().expect("second"),
            Span::default(),
            NodeMetadata::default(),
            AnnotationId::new(
                "user",
                "repeatable",
            )
            .expect("valid identity"),
        );

        let mut annotations = Annotations::new();
        annotations.push(first);
        annotations.push(second);

        assert_eq!(annotations.len(), 2);
    }

    #[test]
    fn lexical_integer_is_not_truncated_to_machine_width() {
        let value = AnnotationValue::integer(
            "184467440737095516160000000000000000000000000000000",
        );

        assert_eq!(
            value,
            AnnotationValue::Integer(
                "184467440737095516160000000000000000000000000000000"
                    .to_owned(),
            )
        );
    }

    #[test]
    fn deeply_nested_values_validate_without_recursive_calls() {
        let mut value = AnnotationValue::Unit;

        for _ in 0..10_000 {
            value = AnnotationValue::List(vec![value]);
        }

        assert!(
            validate_annotation_value(&value).is_ok()
        );
    }

    #[test]
    fn expression_values_reference_ast_nodes() {
        let node_id = test_node_id();

        let value = AnnotationValue::expression(node_id);

        assert_eq!(
            value,
            AnnotationValue::Expression(node_id)
        );
    }

    #[test]
    fn object_fields_preserve_order() {
        let first = AnnotationField::new(
            "first",
            AnnotationValue::Bool(true),
        )
        .expect("valid field");

        let second = AnnotationField::new(
            "second",
            AnnotationValue::Bool(false),
        )
        .expect("valid field");

        let value = AnnotationValue::Object(
            vec![first, second],
        );

        match value {
            AnnotationValue::Object(fields) => {
                assert_eq!(fields[0].name(), "first");
                assert_eq!(fields[1].name(), "second");
            }

            _ => panic!("expected object"),
        }
    }

    #[test]
    fn annotation_validates() {
        let annotation = Annotation::new(
            test_node_id(),
            Span::default(),
            NodeMetadata::default(),
            AnnotationId::new(
                "quantum",
                "adaptive",
            )
            .expect("valid identity"),
        );

        assert!(annotation.validate().is_ok());
    }

    #[test]
    fn display_is_deterministic() {
        let annotation = Annotation::with_value(
            test_node_id(),
            Span::default(),
            NodeMetadata::default(),
            AnnotationId::new(
                "quantum",
                "precision",
            )
            .expect("valid identity"),
            AnnotationValue::Integer(
                "1024".to_owned(),
            ),
        );

        assert_eq!(
            annotation.to_string(),
            "quantum::precision=1024"
        );
    }

    #[test]
    fn schema_version_is_explicit() {
        assert_eq!(
            Annotation::schema_version(),
            ANNOTATION_AST_SCHEMA_VERSION
        );
    }
}