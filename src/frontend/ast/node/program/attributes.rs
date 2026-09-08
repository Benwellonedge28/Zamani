//! # Zamani Frontend AST — Program Attributes
//!
//! Production-grade representation of source-level attributes attached to
//! programs, modules, items, declarations, and other AST constructs.
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
//!      ├── Program
//!      │     ├── attributes
//!      │     └── items
//!      │
//!      ├── Module
//!      │     └── attributes
//!      │
//!      └── Item
//!            └── attributes
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
//! ```
//!
//! ## Purpose
//!
//! This module defines the canonical **source-level attribute model** used by
//! the native Zamani AST.
//!
//! An attribute is metadata expressed by the Zamani source language and
//! associated with another AST construct.
//!
//! Attributes may communicate source-level intent, declarations, compiler
//! directives, documentation/tooling information, constraints, extension
//! information, or other explicitly defined language-level metadata.
//!
//! This module deliberately does **not** decide what an attribute means for a
//! particular machine, processor, quantum computer, backend, scheduler, or
//! runtime.
//!
//! Semantic interpretation belongs to later compiler phases.
//!
//! ## POCO-REAF
//!
//! The attribute representation contains no fixed computational-resource
//! assumptions.
//!
//! It does not encode:
//!
//! - a maximum number of qubits;
//! - a maximum number of CPUs;
//! - a fixed GPU count;
//! - a fixed FPGA count;
//! - a fixed register size;
//! - a fixed machine topology;
//! - a fixed gate set;
//! - a fixed quantum technology;
//! - a hardware vendor;
//! - a backend;
//! - a scheduler;
//! - a calibration system;
//! - a routing implementation;
//! - a QEC implementation;
//! - a resilience implementation;
//! - a QIR value;
//! - an LLVM value;
//! - an MLIR operation.
//!
//! Consequently, an attribute can describe source-level intent for a program
//! that is eventually compiled to a tiny machine, a very large machine, a
//! heterogeneous system, a distributed system, a quantum system, or a future
//! computational architecture.
//!
//! ```text
//! Program written once
//!        │
//!        ▼
//! Source-level attributes
//!        │
//!        ▼
//! Native AST
//!        │
//!        ▼
//! Semantic interpretation
//!        │
//!        ▼
//! Capability/resource matching
//!        │
//!        ▼
//! Target-specific realization
//! ```
//!
//! ## Critical architectural boundary
//!
//! An attribute is **not** automatically a semantic fact.
//!
//! For example, an attribute expressing a resource requirement does not itself
//! allocate that resource.
//!
//! Likewise, an attribute naming a capability does not prove that a target
//! provides that capability.
//!
//! The correct separation is:
//!
//! ```text
//! source attribute
//!       │
//!       ▼
//! AST representation
//!       │
//!       ▼
//! semantic interpretation
//!       │
//!       ▼
//! validated requirement/constraint
//!       │
//!       ▼
//! target capability/resource matching
//! ```
//!
//! ## Extensibility
//!
//! Attributes are intentionally represented using:
//!
//! - a namespace;
//! - a name;
//! - optional arguments;
//! - source information;
//! - structured values.
//!
//! The core AST therefore does not need a new Rust enum variant every time a
//! new language feature, computational domain, quantum technology, tool, or
//! future extension introduces an attribute.
//!
//! Examples of namespaces might include:
//!
//! ```text
//! zamani
//! zamani.compiler
//! zamani.tooling
//! extension.example
//! vendor.example
//! ```
//!
//! The AST treats these as identifiers. Their semantic meaning belongs to the
//! appropriate registered extension or later compiler phase.
//!
//! ## Attribute versus annotation versus directive
//!
//! These concepts must not be conflated:
//!
//! - **Attribute** — structured metadata attached to an AST construct.
//! - **Annotation** — source-level metadata/intent whose exact semantics may be
//!   defined by a language extension.
//! - **Directive** — an instruction affecting compiler/source processing.
//!
//! This file owns the generic attribute data model. Specialized annotation or
//! directive semantics belong elsewhere.
//!
//! ## Dependency contract
//!
//! This module may depend only on:
//!
//! - Rust standard-library types;
//! - `serde` for serialization;
//! - foundational source-span infrastructure when source locations are stored.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - compiler orchestration;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - quantum hardware;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - optimization;
//! - error correction;
//! - resilience;
//! - runtime;
//! - backend providers;
//! - external quantum-language implementations.
//!
//! ## Integration contract
//!
//! ```text
//! parser
//!   │
//!   ▼
//! Attribute
//!   │
//!   ▼
//! AttributeSet
//!   │
//!   ├───────────────┐
//!   ▼               ▼
//! Program          Module / Item
//!   │               │
//!   └───────┬───────┘
//!           ▼
//! structural validation
//!           │
//!           ▼
//! semantic analysis
//!           │
//!           ▼
//! semantic model
//!           │
//!           ▼
//! ZUIR
//! ```
//!
//! `Program`, `Module`, and `Item` may own an `AttributeSet` or another
//! equivalent ordered attribute collection.
//!
//! The attribute module must not depend on those parent nodes. This keeps the
//! dependency direction one-way and prevents circular dependencies.
//!
//! ## Parser contract
//!
//! The parser is responsible for:
//!
//! 1. recognizing attribute syntax;
//! 2. preserving namespace and name spelling;
//! 3. constructing structured attribute values;
//! 4. attaching the correct source span;
//! 5. preserving source order;
//! 6. preserving duplicate attributes rather than silently discarding them;
//! 7. constructing an `AttributeSet` without interpreting target semantics.
//!
//! The parser must not:
//!
//! - resolve capabilities;
//! - select hardware;
//! - allocate resources;
//! - perform quantum routing;
//! - schedule execution;
//! - select a backend.
//!
//! ## Semantic contract
//!
//! Semantic analysis consumes attributes after structural validation.
//!
//! It determines whether a particular attribute is:
//!
//! - recognized;
//! - ignored;
//! - deprecated;
//! - invalid;
//! - extension-defined;
//! - semantically meaningful for the current construct;
//! - relevant to the requested compilation mode.
//!
//! Semantic analysis may convert attributes into semantic constraints or other
//! side-table information.
//!
//! It must not mutate the source-level attribute representation merely to
//! record semantic resolution.
//!
//! ## ZUIR contract
//!
//! Attributes do not lower directly to ZUIR merely because they exist in the
//! AST.
//!
//! The normal path is:
//!
//! ```text
//! Attribute
//!     │
//!     ▼
//! semantic interpretation
//!     │
//!     ▼
//! semantic constraint / property / directive
//!     │
//!     ▼
//! ZUIR, when applicable
//! ```
//!
//! Attributes that are purely tooling metadata may never enter ZUIR.
//!
//! Attributes that affect computation may become semantic information during
//! lowering.
//!
//! ## Source preservation
//!
//! Attribute spelling and source ordering must be preserved.
//!
//! This is important for:
//!
//! - diagnostics;
//! - IDE tooling;
//! - source-to-source transformations;
//! - formatting;
//! - documentation generation;
//! - deterministic compilation;
//! - macro expansion;
//! - imported source formats.
//!
//! ## Determinism
//!
//! Attribute collections preserve source order.
//!
//! This module deliberately does not use a hash-based collection for the
//! canonical attribute sequence.
//!
//! Two attributes with identical namespace/name are not automatically merged.
//!
//! This avoids making semantic assumptions at the AST layer and preserves the
//! source faithfully.
//!
//! ## Duplicate attributes
//!
//! Duplicate attributes are legal at the representation level.
//!
//! Whether duplicates are:
//!
//! - valid;
//! - invalid;
//! - equivalent;
//! - overriding;
//! - cumulative;
//! - extension-specific
//!
//! is a semantic question.
//!
//! Therefore this module does not silently deduplicate attributes.
//!
//! ## Unknown attributes
//!
//! Unknown attributes are representable.
//!
//! The AST must preserve them so that:
//!
//! - newer compilers can consume ASTs produced by older tooling;
//! - extensions can be loaded later;
//! - source-preserving tools do not destroy information;
//! - forward compatibility is possible.
//!
//! Whether an unknown attribute is accepted for compilation is decided by the
//! validation/semantic policy.
//!
//! ## Serialization
//!
//! Serialization must preserve:
//!
//! - namespace;
//! - name;
//! - arguments;
//! - source span;
//! - source order;
//! - duplicate attributes;
//! - unknown attributes;
//! - structured values.
//!
//! No pointer, memory address, backend object, or runtime state is serialized.
//!
//! The enclosing AST serialization layer is responsible for the global AST
//! schema version.
//!
//! ## Scalability
//!
//! There is no fixed number of attributes.
//!
//! The same representation supports:
//!
//! ```text
//! 0 attributes
//! 1 attribute
//! 10 attributes
//! 10,000 attributes
//! generated attributes
//! extension-defined attributes
//! large structured attribute values
//! ```
//!
//! Any operational limits for hostile/untrusted input must be supplied by
//! configurable compiler policy.
//!
//! They must not become hidden language semantics in this module.
//!
//! ## Security
//!
//! Attribute data originates from potentially untrusted source.
//!
//! This implementation therefore:
//!
//! - performs checked conversions;
//! - avoids unchecked indexing;
//! - does not dereference pointers;
//! - does not execute attribute values;
//! - does not invoke external processes;
//! - does not access hardware;
//! - does not perform filesystem operations;
//! - does not use global mutable state;
//! - does not use `unsafe`.
//!
//! ## Thread safety
//!
//! The types in this module use ordinary owned Rust values and have no global
//! mutable state.
//!
//! Immutable attribute collections can therefore be shared between compiler
//! phases when the enclosing AST is shared immutably.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1.
//!
//! The implementation uses stable language/library facilities only.
//!
//! No `unsafe` code is used.

use core::fmt;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::super::source::Span;

/// Schema version for the program-attribute data model.
///
/// This is independent from:
///
/// - Zamani language version;
/// - compiler version;
/// - AST schema version;
/// - serialization format version;
/// - extension version.
pub const PROGRAM_ATTRIBUTES_SCHEMA_VERSION: u16 = 1;

/// Namespace reserved for attributes defined by the native Zamani language.
pub const ZAMANI_ATTRIBUTE_NAMESPACE: &str = "zamani";

/// Attribute name identifying compiler-oriented metadata.
///
/// This is only an identifier convention. Interpretation belongs to later
/// compiler layers.
pub const COMPILER_ATTRIBUTE_NAMESPACE: &str = "zamani.compiler";

/// Attribute name identifying tooling-oriented metadata.
pub const TOOLING_ATTRIBUTE_NAMESPACE: &str = "zamani.tooling";

/// A source-level attribute attached to an AST construct.
///
/// `Attribute` intentionally contains no semantic-resolution state.
///
/// # Identity
///
/// Attributes do not have a separate `NodeId` in this module because they are
/// owned source metadata of their containing AST node. If a future AST design
/// requires every attribute to become an independently addressable AST node,
/// that identity can be introduced at the higher AST-node layer without
/// changing the semantic representation of the attribute itself.
///
/// # Duplicate policy
///
/// Duplicate attributes are representable. The containing `AttributeSet`
/// preserves their order.
///
/// # Extension policy
///
/// Namespace and name are open-ended strings. A new extension therefore does
/// not require changing this type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attribute {
    /// Attribute namespace.
    namespace: String,

    /// Attribute name.
    name: String,

    /// Optional structured arguments.
    ///
    /// `None` means the attribute has no argument payload.
    arguments: Option<AttributeArguments>,

    /// Source span of the complete attribute.
    span: Span,
}

impl Attribute {
    /// Creates a new argument-free attribute.
    ///
    /// # Errors
    ///
    /// Returns [`AttributeError`] when the namespace or name is invalid.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        span: Span,
    ) -> Result<Self, AttributeError> {
        Self::with_arguments(namespace, name, None, span)
    }

    /// Creates a new attribute with optional structured arguments.
    ///
    /// No semantic validation is performed.
    pub fn with_arguments(
        namespace: impl Into<String>,
        name: impl Into<String>,
        arguments: Option<AttributeArguments>,
        span: Span,
    ) -> Result<Self, AttributeError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_identifier_component("namespace", &namespace)?;
        validate_identifier_component("name", &name)?;

        Ok(Self {
            namespace,
            name,
            arguments,
            span,
        })
    }

    /// Creates an attribute using a structured argument map.
    ///
    /// This is a convenience constructor for the common named-argument form.
    pub fn with_named_arguments(
        namespace: impl Into<String>,
        name: impl Into<String>,
        arguments: BTreeMap<String, AttributeValue>,
        span: Span,
    ) -> Result<Self, AttributeError> {
        Self::with_arguments(
            namespace,
            name,
            Some(AttributeArguments::Named(arguments)),
            span,
        )
    }

    /// Returns the namespace.
    #[inline]
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the attribute name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the optional argument payload.
    #[inline]
    #[must_use]
    pub fn arguments(&self) -> Option<&AttributeArguments> {
        self.arguments.as_ref()
    }

    /// Returns mutable access to the argument payload.
    ///
    /// Semantic interpretation must not be performed through this API.
    #[inline]
    pub fn arguments_mut(&mut self) -> Option<&mut AttributeArguments> {
        self.arguments.as_mut()
    }

    /// Returns the source span of the complete attribute.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &Span {
        &self.span
    }

    /// Returns the canonical namespace/name identifier.
    ///
    /// The format is:
    ///
    /// ```text
    /// namespace:name
    /// ```
    #[must_use]
    pub fn qualified_name(&self) -> String {
        let mut result = String::with_capacity(
            self.namespace
                .len()
                .saturating_add(1)
                .saturating_add(self.name.len()),
        );

        result.push_str(&self.namespace);
        result.push(':');
        result.push_str(&self.name);

        result
    }

    /// Returns whether this attribute belongs to the native Zamani namespace.
    #[inline]
    #[must_use]
    pub fn is_zamani(&self) -> bool {
        self.namespace == ZAMANI_ATTRIBUTE_NAMESPACE
            || self.namespace.starts_with("zamani.")
    }

    /// Returns whether this attribute belongs to the compiler namespace.
    #[inline]
    #[must_use]
    pub fn is_compiler_attribute(&self) -> bool {
        self.namespace == COMPILER_ATTRIBUTE_NAMESPACE
    }

    /// Returns whether this attribute belongs to the tooling namespace.
    #[inline]
    #[must_use]
    pub fn is_tooling_attribute(&self) -> bool {
        self.namespace == TOOLING_ATTRIBUTE_NAMESPACE
    }

    /// Returns whether the attribute has arguments.
    #[inline]
    #[must_use]
    pub fn has_arguments(&self) -> bool {
        self.arguments.is_some()
    }

    /// Returns the number of positional/named argument values.
    ///
    /// For named arguments, this is the number of named entries.
    #[inline]
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments
            .as_ref()
            .map(AttributeArguments::len)
            .unwrap_or(0)
    }

    /// Validates the local structural invariants of this attribute.
    ///
    /// This method deliberately does not determine whether the attribute is
    /// legal on a particular AST node.
    pub fn validate(&self) -> Result<(), AttributeError> {
        validate_identifier_component("namespace", &self.namespace)?;
        validate_identifier_component("name", &self.name)?;

        if let Some(arguments) = &self.arguments {
            arguments.validate()?;
        }

        Ok(())
    }
}

/// Structured arguments associated with an attribute.
///
/// Two forms are supported:
///
/// - positional arguments;
/// - named arguments.
///
/// The representation is intentionally open-ended and does not prescribe a
/// specific semantic schema.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttributeArguments {
    /// Ordered positional arguments.
    Positional(Vec<AttributeValue>),

    /// Deterministically ordered named arguments.
    Named(BTreeMap<String, AttributeValue>),
}

impl AttributeArguments {
    /// Creates positional arguments.
    #[inline]
    #[must_use]
    pub fn positional(values: Vec<AttributeValue>) -> Self {
        Self::Positional(values)
    }

    /// Creates named arguments.
    #[inline]
    #[must_use]
    pub fn named(values: BTreeMap<String, AttributeValue>) -> Self {
        Self::Named(values)
    }

    /// Returns the number of arguments.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Positional(values) => values.len(),
            Self::Named(values) => values.len(),
        }
    }

    /// Returns whether no arguments are present.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns positional values when applicable.
    #[inline]
    #[must_use]
    pub fn as_positional(&self) -> Option<&[AttributeValue]> {
        match self {
            Self::Positional(values) => Some(values),
            Self::Named(_) => None,
        }
    }

    /// Returns named values when applicable.
    #[inline]
    #[must_use]
    pub fn as_named(&self) -> Option<&BTreeMap<String, AttributeValue>> {
        match self {
            Self::Positional(_) => None,
            Self::Named(values) => Some(values),
        }
    }

    /// Returns the named value for `name`, if present.
    #[inline]
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&AttributeValue> {
        match self {
            Self::Positional(_) => None,
            Self::Named(values) => values.get(name),
        }
    }

    /// Validates argument structure.
    pub fn validate(&self) -> Result<(), AttributeError> {
        match self {
            Self::Positional(values) => {
                for value in values {
                    value.validate()?;
                }
            }

            Self::Named(values) => {
                for (name, value) in values {
                    validate_argument_name(name)?;
                    value.validate()?;
                }
            }
        }

        Ok(())
    }
}

/// Source-level structured value used by attributes.
///
/// This type intentionally describes syntax/data rather than semantic types.
///
/// It can therefore represent values introduced by future extensions without
/// requiring the core AST to know their meaning.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttributeValue {
    /// No value.
    Null,

    /// Boolean value.
    Boolean(bool),

    /// Signed integer value.
    Integer(i64),

    /// Arbitrary unsigned integer value that fits in `u64`.
    Unsigned(u64),

    /// Exact source-preserved textual value.
    String(String),

    /// Identifier-like value.
    Identifier(String),

    /// Qualified identifier represented as ordered segments.
    Path(Vec<String>),

    /// Ordered sequence.
    List(Vec<AttributeValue>),

    /// Deterministically ordered object/map.
    Map(BTreeMap<String, AttributeValue>),
}

impl AttributeValue {
    /// Creates a string value.
    #[inline]
    #[must_use]
    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }

    /// Creates an identifier value.
    #[inline]
    #[must_use]
    pub fn identifier(value: impl Into<String>) -> Self {
        Self::Identifier(value.into())
    }

    /// Creates a path value.
    #[inline]
    #[must_use]
    pub fn path(values: Vec<String>) -> Self {
        Self::Path(values)
    }

    /// Creates a list value.
    #[inline]
    #[must_use]
    pub fn list(values: Vec<Self>) -> Self {
        Self::List(values)
    }

    /// Creates a map value.
    #[inline]
    #[must_use]
    pub fn map(values: BTreeMap<String, Self>) -> Self {
        Self::Map(values)
    }

    /// Returns the string value, if applicable.
    #[inline]
    #[must_use]
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the identifier value, if applicable.
    #[inline]
    #[must_use]
    pub fn as_identifier(&self) -> Option<&str> {
        match self {
            Self::Identifier(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the signed integer value, if applicable.
    #[inline]
    #[must_use]
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the unsigned integer value, if applicable.
    #[inline]
    #[must_use]
    pub fn as_unsigned(&self) -> Option<u64> {
        match self {
            Self::Unsigned(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the boolean value, if applicable.
    #[inline]
    #[must_use]
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the list value, if applicable.
    #[inline]
    #[must_use]
    pub fn as_list(&self) -> Option<&[Self]> {
        match self {
            Self::List(values) => Some(values),
            _ => None,
        }
    }

    /// Returns the map value, if applicable.
    #[inline]
    #[must_use]
    pub fn as_map(&self) -> Option<&BTreeMap<String, Self>> {
        match self {
            Self::Map(values) => Some(values),
            _ => None,
        }
    }

    /// Validates structural invariants recursively.
    pub fn validate(&self) -> Result<(), AttributeError> {
        match self {
            Self::Null
            | Self::Boolean(_)
            | Self::Integer(_)
            | Self::Unsigned(_)
            | Self::String(_) => {}

            Self::Identifier(value) => {
                validate_identifier_component("identifier", value)?;
            }

            Self::Path(segments) => {
                if segments.is_empty() {
                    return Err(AttributeError::EmptyPath);
                }

                for segment in segments {
                    validate_identifier_component("path segment", segment)?;
                }
            }

            Self::List(values) => {
                for value in values {
                    value.validate()?;
                }
            }

            Self::Map(values) => {
                for (key, value) in values {
                    validate_argument_name(key)?;
                    value.validate()?;
                }
            }
        }

        Ok(())
    }
}

/// Ordered collection of attributes belonging to one AST construct.
///
/// `AttributeSet` intentionally preserves insertion/source order.
///
/// Despite the name, this is not a mathematical set because duplicate
/// attributes are intentionally preserved.
///
/// The type exists to provide a stable collection API without exposing the
/// underlying storage representation.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttributeSet {
    attributes: Vec<Attribute>,
}

impl AttributeSet {
    /// Creates an empty attribute collection.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            attributes: Vec::new(),
        }
    }

    /// Creates an attribute collection from an existing ordered sequence.
    ///
    /// The sequence is preserved exactly.
    #[inline]
    #[must_use]
    pub fn from_vec(attributes: Vec<Attribute>) -> Self {
        Self { attributes }
    }

    /// Returns the number of attributes.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    /// Returns whether there are no attributes.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }

    /// Appends an attribute while preserving source order.
    ///
    /// Duplicate names are intentionally accepted.
    #[inline]
    pub fn push(&mut self, attribute: Attribute) {
        self.attributes.push(attribute);
    }

    /// Extends the collection in iterator order.
    #[inline]
    pub fn extend<I>(&mut self, attributes: I)
    where
        I: IntoIterator<Item = Attribute>,
    {
        self.attributes.extend(attributes);
    }

    /// Returns all attributes in source order.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[Attribute] {
        &self.attributes
    }

    /// Returns a mutable slice of attributes.
    ///
    /// This is intended for controlled AST transformations.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [Attribute] {
        &mut self.attributes
    }

    /// Returns an iterator over attributes.
    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, Attribute> {
        self.attributes.iter()
    }

    /// Returns a mutable iterator over attributes.
    #[inline]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Attribute> {
        self.attributes.iter_mut()
    }

    /// Returns the first attribute with the supplied namespace and name.
    #[must_use]
    pub fn find(&self, namespace: &str, name: &str) -> Option<&Attribute> {
        self.attributes
            .iter()
            .find(|attribute| {
                attribute.namespace() == namespace && attribute.name() == name
            })
    }

    /// Returns all attributes with the supplied namespace and name.
    ///
    /// The returned references preserve source order.
    #[must_use]
    pub fn find_all(
        &self,
        namespace: &str,
        name: &str,
    ) -> Vec<&Attribute> {
        self.attributes
            .iter()
            .filter(|attribute| {
                attribute.namespace() == namespace && attribute.name() == name
            })
            .collect()
    }

    /// Returns whether at least one attribute with the supplied identity exists.
    #[inline]
    #[must_use]
    pub fn contains(&self, namespace: &str, name: &str) -> bool {
        self.find(namespace, name).is_some()
    }

    /// Returns attributes belonging to a namespace.
    ///
    /// Source order is preserved.
    #[must_use]
    pub fn in_namespace(&self, namespace: &str) -> Vec<&Attribute> {
        self.attributes
            .iter()
            .filter(|attribute| attribute.namespace() == namespace)
            .collect()
    }

    /// Removes and returns the attribute at `index`.
    ///
    /// This operation is checked and returns `None` when the index is outside
    /// the collection.
    pub fn remove(&mut self, index: usize) -> Option<Attribute> {
        if index < self.attributes.len() {
            Some(self.attributes.remove(index))
        } else {
            None
        }
    }

    /// Clears the collection.
    ///
    /// This is intended for AST transformation infrastructure.
    #[inline]
    pub fn clear(&mut self) {
        self.attributes.clear();
    }

    /// Validates all contained attributes.
    ///
    /// This performs only local structural validation.
    pub fn validate(&self) -> Result<(), AttributeError> {
        for attribute in &self.attributes {
            attribute.validate()?;
        }

        Ok(())
    }

    /// Returns the schema version of this attribute model.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        PROGRAM_ATTRIBUTES_SCHEMA_VERSION
    }
}

impl<'a> IntoIterator for &'a AttributeSet {
    type Item = &'a Attribute;
    type IntoIter = core::slice::Iter<'a, Attribute>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.attributes.iter()
    }
}

impl<'a> IntoIterator for &'a mut AttributeSet {
    type Item = &'a mut Attribute;
    type IntoIter = core::slice::IterMut<'a, Attribute>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.attributes.iter_mut()
    }
}

impl IntoIterator for AttributeSet {
    type Item = Attribute;
    type IntoIter = std::vec::IntoIter<Attribute>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.attributes.into_iter()
    }
}

/// Errors produced by local attribute construction/validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttributeError {
    /// Namespace is empty.
    EmptyNamespace,

    /// Attribute name is empty.
    EmptyName,

    /// An identifier component contains an invalid character.
    InvalidCharacter {
        /// Component that failed validation.
        component: &'static str,

        /// Invalid character.
        character: char,
    },

    /// An attribute argument name is empty.
    EmptyArgumentName,

    /// An argument name contains an invalid character.
    InvalidArgumentName {
        /// Invalid character.
        character: char,
    },

    /// A path contains no segments.
    EmptyPath,
}

impl fmt::Display for AttributeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNamespace => {
                formatter.write_str("attribute namespace must not be empty")
            }

            Self::EmptyName => {
                formatter.write_str("attribute name must not be empty")
            }

            Self::InvalidCharacter {
                component,
                character,
            } => {
                write!(
                    formatter,
                    "attribute {component} contains invalid character {character:?}"
                )
            }

            Self::EmptyArgumentName => {
                formatter.write_str("attribute argument name must not be empty")
            }

            Self::InvalidArgumentName { character } => {
                write!(
                    formatter,
                    "attribute argument name contains invalid character {character:?}"
                )
            }

            Self::EmptyPath => {
                formatter.write_str("attribute path must contain at least one segment")
            }
        }
    }
}

impl std::error::Error for AttributeError {}

/// Validates a namespace/name/identifier component.
///
/// This is deliberately a syntactic validation function.
///
/// It does not impose:
///
/// - a finite namespace list;
/// - a finite attribute list;
/// - a hardware-specific naming scheme;
/// - a domain-specific naming scheme.
///
/// Names remain extensible.
fn validate_identifier_component(
    component: &'static str,
    value: &str,
) -> Result<(), AttributeError> {
    if value.is_empty() {
        return Err(match component {
            "namespace" => AttributeError::EmptyNamespace,
            "name" => AttributeError::EmptyName,
            _ => AttributeError::InvalidCharacter {
                component,
                character: '\0',
            },
        });
    }

    for character in value.chars() {
        if character.is_control()
            || character.is_whitespace()
            || matches!(character, '/' | '\\' | ':')
        {
            return Err(AttributeError::InvalidCharacter {
                component,
                character,
            });
        }
    }

    Ok(())
}

/// Validates an attribute argument name.
///
/// Argument names use the same conservative structural rules as attribute
/// identifiers while keeping the error type specific to argument names.
fn validate_argument_name(value: &str) -> Result<(), AttributeError> {
    if value.is_empty() {
        return Err(AttributeError::EmptyArgumentName);
    }

    for character in value.chars() {
        if character.is_control()
            || character.is_whitespace()
            || matches!(character, '/' | '\\' | ':')
        {
            return Err(AttributeError::InvalidArgumentName { character });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span() -> Span {
        Span::default()
    }

    #[test]
    fn creates_argument_free_attribute() {
        let attribute =
            Attribute::new("zamani", "entry", span()).expect("valid attribute");

        assert_eq!(attribute.namespace(), "zamani");
        assert_eq!(attribute.name(), "entry");
        assert!(!attribute.has_arguments());
        assert_eq!(attribute.argument_count(), 0);
        assert_eq!(attribute.qualified_name(), "zamani:entry");
    }

    #[test]
    fn creates_named_attribute() {
        let mut arguments = BTreeMap::new();

        arguments.insert(
            "value".to_owned(),
            AttributeValue::String("example".to_owned()),
        );

        let attribute = Attribute::with_named_arguments(
            "zamani.compiler",
            "mode",
            arguments,
            span(),
        )
        .expect("valid attribute");

        assert_eq!(attribute.argument_count(), 1);
        assert!(attribute.is_compiler_attribute());
    }

    #[test]
    fn preserves_duplicate_attributes() {
        let first =
            Attribute::new("example", "flag", span()).expect("valid attribute");

        let second =
            Attribute::new("example", "flag", span()).expect("valid attribute");

        let mut attributes = AttributeSet::new();

        attributes.push(first);
        attributes.push(second);

        assert_eq!(attributes.len(), 2);
        assert_eq!(attributes.find_all("example", "flag").len(), 2);
    }

    #[test]
    fn preserves_source_order() {
        let first =
            Attribute::new("example", "first", span()).expect("valid attribute");

        let second =
            Attribute::new("example", "second", span()).expect("valid attribute");

        let third =
            Attribute::new("example", "third", span()).expect("valid attribute");

        let mut attributes = AttributeSet::new();

        attributes.push(first);
        attributes.push(second);
        attributes.push(third);

        let names: Vec<&str> =
            attributes.iter().map(Attribute::name).collect();

        assert_eq!(names, vec!["first", "second", "third"]);
    }

    #[test]
    fn validates_invalid_namespace() {
        let result = Attribute::new("bad namespace", "name", span());

        assert!(matches!(
            result,
            Err(AttributeError::InvalidCharacter { .. })
        ));
    }

    #[test]
    fn validates_invalid_name() {
        let result = Attribute::new("namespace", "bad:name", span());

        assert!(matches!(
            result,
            Err(AttributeError::InvalidCharacter { .. })
        ));
    }

    #[test]
    fn validates_nested_values() {
        let value = AttributeValue::List(vec![
            AttributeValue::Boolean(true),
            AttributeValue::Map({
                let mut map = BTreeMap::new();
                map.insert(
                    "resource".to_owned(),
                    AttributeValue::String("generic".to_owned()),
                );
                map
            }),
        ]);

        assert!(value.validate().is_ok());
    }

    #[test]
    fn supports_extension_defined_values() {
        let value = AttributeValue::Map({
            let mut map = BTreeMap::new();

            map.insert(
                "future-domain".to_owned(),
                AttributeValue::Identifier("example".to_owned()),
            );

            map
        });

        assert!(value.validate().is_ok());
    }

    #[test]
    fn empty_attribute_set_is_valid() {
        let attributes = AttributeSet::new();

        assert!(attributes.is_empty());
        assert_eq!(attributes.len(), 0);
        assert!(attributes.validate().is_ok());
    }

    #[test]
    fn remove_is_checked() {
        let mut attributes = AttributeSet::new();

        assert!(attributes.remove(0).is_none());
    }

    #[test]
    fn schema_version_is_explicit() {
        assert_eq!(AttributeSet::schema_version(), 1);
    }
}