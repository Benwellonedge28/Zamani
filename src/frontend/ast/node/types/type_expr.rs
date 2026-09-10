//! Zamani Native AST — Type Expressions
//!
//! Canonical source-level representation of Zamani type expressions.
//!
//! # Architectural position
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
//! TypeExpr
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic type model
//!     │
//!     ▼
//! ZUIR / domain IR
//!     │
//!     ├── classical
//!     ├── quantum
//!     ├── HDL
//!     ├── accelerator
//!     ├── distributed
//!     └── future domains
//! ```
//!
//! # Responsibility
//!
//! This file owns the source-level representation of a Zamani type expression.
//!
//! It describes what a programmer wrote or what the parser structurally
//! constructed. It does not resolve the type, determine its runtime layout,
//! allocate resources, select hardware, or lower to an implementation.
//!
//! In particular, this module does not know about:
//!
//! - CPUs;
//! - GPUs;
//! - FPGAs;
//! - ASICs;
//! - QPUs;
//! - quantum vendors;
//! - physical qubits;
//! - logical-qubit allocation;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - QEC implementation;
//! - resilience implementation;
//! - backend queues;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - ZUIR implementation details.
//!
//! Those concerns belong to later compiler layers.
//!
//! # POCO-REAF
//!
//! The type system must not encode a finite machine size.
//!
//! A type such as:
//!
//! ```text
//! Quantum<Q>
//! ```
//!
//! describes a source-level abstraction. It does not mean a fixed number of
//! physical qubits.
//!
//! Similarly:
//!
//! ```text
//! Array<T, N>
//! ```
//!
//! permits `N` to remain symbolic until later compilation stages.
//!
//! The AST therefore introduces no artificial limits on:
//!
//! - type nesting;
//! - generic arity;
//! - tuple arity;
//! - function parameters;
//! - array dimensions;
//! - symbolic resource sizes;
//! - quantum-resource cardinality.
//!
//! Actual limits are compiler resource-policy concerns.
//!
//! # Important architectural distinction
//!
//! ```text
//! TypeExpr
//!     = source syntax / source intent
//!
//! SemanticType
//!     = resolved type meaning
//!
//! ZUIR
//!     = universal computational representation
//!
//! Domain IR
//!     = domain-specific implementation representation
//!
//! Target IR
//!     = machine/backend representation
//! ```
//!
//! `TypeExpr` must never become a disguised semantic type or hardware IR.
//!
//! # External language independence
//!
//! OpenQASM, QIR, Q#, Quil, Cirq representations, vendor languages and future
//! external formats must not be embedded into this type.
//!
//! External frontends should lower their source representation into Zamani
//! source-level types.
//!
//! # Rust requirements
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Dependencies
//!
//! This module intentionally depends only on:
//!
//! - Rust's standard library;
//! - `serde` already present in the repository.
//!
//! It does not depend on the legacy `crate::ast` module.
//!
//! This is important because the new frontend AST must become independent of
//! the legacy AST rather than introducing another dependency cycle.
//!
//! # File contract
//!
//! ## Inputs
//!
//! - parser-produced type structure;
//! - source-level names;
//! - generic arguments;
//! - type-expression children;
//! - source-level modifiers and bounds.
//!
//! ## Outputs
//!
//! - `TypeExpr`;
//! - deterministic structural inspection;
//! - structural validation;
//! - source-level type formatting;
//! - child enumeration;
//! - target-independent type predicates.
//!
//! ## Does not perform
//!
//! - name resolution;
//! - type inference;
//! - type unification;
//! - trait resolution;
//! - overload resolution;
//! - generic substitution;
//! - ownership checking;
//! - borrow checking;
//! - resource allocation;
//! - quantum legality checking;
//! - topology checking;
//! - backend selection.
//!
//! # Integration contract
//!
//! `type_alias.rs`, declarations, parameters, casts, type ascriptions and
//! function signatures consume `TypeExpr`.
//!
//! The parser constructs `TypeExpr`.
//!
//! Structural validation validates `TypeExpr`.
//!
//! Semantic analysis consumes validated `TypeExpr` and produces semantic types.
//!
//! ZUIR lowering consumes semantic types rather than importing this file
//! directly for target-specific decisions.
//!
//! # No hard-coded machine assumptions
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_REGISTER_SIZE
//! MAX_TYPE_DEPTH
//! MAX_GENERIC_PARAMETERS
//! MAX_ARRAY_LENGTH
//! MAX_FUNCTION_PARAMETERS
//! ```
//!
//! in this module.
//!
//! Safety limits, where required, are supplied explicitly through
//! `TypeValidationPolicy`.
//!
//! # Determinism
//!
//! Ordered source collections use `Vec`.
//!
//! Named path components retain source order.
//!
//! No hash-map iteration is used to define semantic or serialization order.
//!
//! # Security
//!
//! This file performs no I/O, no execution, no pointer arithmetic and no raw
//! memory operations.
//!
//! Invalid untrusted structures return errors instead of panicking.
//!
//! # Compatibility note
//!
//! The legacy AST previously represented type expressions with variants such
//! as:
//!
//! ```text
//! Identifier
//! Generic
//! Tuple
//! Array
//! Slice
//! Function
//! Reference
//! Pointer
//! Optional
//! Result
//! Never
//! Unit
//! SelfType
//! Quantum
//! Linear
//! Affine
//! Temporal
//! Pi
//! Sigma
//! Identity
//! Hkt
//! ```
//!
//! The canonical frontend representation retains these semantic source-level
//! capabilities while replacing implementation-specific dependencies with
//! target-independent structures.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::borrow::Borrow;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

/// Schema version for this local type-expression representation.
///
/// This is deliberately separate from the overall Zamani language version,
/// compiler version, and serialized AST schema version.
pub const TYPE_EXPR_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Shared string representation
// =============================================================================

/// Interning-friendly immutable source identifier.
///
/// `Arc<str>` avoids repeatedly copying large identifiers while retaining
/// ordinary owned lifetime semantics and `Send + Sync` compatibility.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TypeName(Arc<str>);

impl TypeName {
    /// Creates a type name.
    ///
    /// Empty names are structurally permitted at construction time so parser
    /// error recovery can preserve malformed source. Complete AST validation
    /// rejects them.
    #[must_use]
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self(Arc::<str>::from(name.into()))
    }

    /// Creates a type name from a string slice.
    #[must_use]
    pub fn from_str(name: &str) -> Self {
        Self(Arc::<str>::from(name))
    }

    /// Returns the textual name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    /// Returns whether the name is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the byte length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Default for TypeName {
    fn default() -> Self {
        Self::from_str("")
    }
}

impl AsRef<str> for TypeName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for TypeName {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for TypeName {
    fn from(value: &str) -> Self {
        Self::from_str(value)
    }
}

impl From<String> for TypeName {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for TypeName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Type paths
// =============================================================================

/// A source-level qualified type path.
///
/// Examples:
///
/// ```text
/// Int
/// std::collections::Map
/// quantum::State
/// my::module::Type
/// ```
///
/// The path contains no resolved symbol identity. Resolution belongs to
/// semantic analysis.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypePath {
    segments: Vec<TypeName>,
}

impl TypePath {
    /// Creates a path from ordered segments.
    #[must_use]
    pub fn new(segments: Vec<TypeName>) -> Self {
        Self { segments }
    }

    /// Creates a single-segment path.
    #[must_use]
    pub fn single<S>(name: S) -> Self
    where
        S: Into<TypeName>,
    {
        Self {
            segments: vec![name.into()],
        }
    }

    /// Creates a path from an iterator.
    #[must_use]
    pub fn from_names<I, S>(names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<TypeName>,
    {
        Self {
            segments: names.into_iter().map(Into::into).collect(),
        }
    }

    /// Returns the ordered path segments.
    #[must_use]
    pub fn segments(&self) -> &[TypeName] {
        &self.segments
    }

    /// Returns the number of path segments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Returns whether the path has no segments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Returns the final path component.
    #[must_use]
    pub fn last(&self) -> Option<&TypeName> {
        self.segments.last()
    }

    /// Returns the textual path using Zamani's source-level `::` separator.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        self.segments
            .iter()
            .map(TypeName::as_str)
            .collect::<Vec<_>>()
            .join("::")
    }
}

impl Default for TypePath {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl fmt::Display for TypePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;

        for segment in &self.segments {
            if !first {
                formatter.write_str("::")?;
            }

            formatter.write_str(segment.as_str())?;
            first = false;
        }

        Ok(())
    }
}

// =============================================================================
// Type parameters and lifetimes
// =============================================================================

/// Source-level generic type parameter reference.
///
/// This is intentionally a name rather than a resolved symbol.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TypeParameterName(TypeName);

impl TypeParameterName {
    /// Creates a type-parameter name.
    #[must_use]
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self(TypeName::new(name))
    }

    /// Returns the parameter name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for TypeParameterName {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for TypeParameterName {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for TypeParameterName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Source-level lifetime name.
///
/// Lifetime semantics are resolved later.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LifetimeName(TypeName);

impl LifetimeName {
    /// Creates a lifetime name without requiring the leading apostrophe.
    #[must_use]
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self(TypeName::new(name))
    }

    /// Returns the lifetime name without syntactic decoration.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for LifetimeName {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for LifetimeName {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for LifetimeName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.as_str() == "_" {
            formatter.write_str("'_")
        } else {
            write!(formatter, "'{}", self.as_str())
        }
    }
}

// =============================================================================
// Type expression
// =============================================================================

/// Canonical source-level Zamani type expression.
///
/// The representation deliberately remains source-oriented and target-neutral.
///
/// Recursive type expressions use `Box<TypeExpr>` rather than references into
/// a global AST store. This makes `TypeExpr` independently usable by parsers,
/// declaration nodes, transformations and compatibility adapters while still
/// imposing no finite semantic size limit.
///
/// Compiler-wide AST graph interning can later wrap or intern these values
/// without changing their semantic vocabulary.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeExpr {
    /// A named or qualified type.
    Identifier(TypePath),

    /// A generic/parameterized type application.
    ///
    /// Example:
    ///
    /// ```text
    /// Vec<Int>
    /// Map<String, Int>
    /// Quantum<Q>
    /// ```
    Generic {
        /// Generic constructor/base type.
        base: Box<TypeExpr>,

        /// Ordered generic arguments.
        arguments: Vec<TypeExpr>,
    },

    /// Tuple type.
    Tuple(Vec<TypeExpr>),

    /// Fixed or symbolic-size array.
    ///
    /// `length = None` represents a source form whose cardinality is not
    /// explicitly supplied by the syntax.
    Array {
        /// Element type.
        element: Box<TypeExpr>,

        /// Optional source-level length expression.
        ///
        /// The expression is kept as source text rather than being converted
        /// to a machine-sized integer. Semantic analysis determines whether it
        /// is a valid compile-time or dependent cardinality.
        length: Option<TypeValueExpr>,
    },

    /// Dynamically sized/slice type.
    Slice(Box<TypeExpr>),

    /// Function type.
    Function {
        /// Ordered parameter types.
        parameters: Vec<TypeExpr>,

        /// Return type.
        return_type: Box<TypeExpr>,
    },

    /// Borrow/reference type.
    Reference {
        /// Whether the reference is mutable.
        mutable: bool,

        /// Optional source-level lifetime.
        lifetime: Option<LifetimeName>,

        /// Referenced type.
        inner: Box<TypeExpr>,
    },

    /// Raw pointer type.
    ///
    /// Pointer legality is a semantic/compiler concern.
    Pointer {
        /// Whether the pointer is mutable.
        mutable: bool,

        /// Pointed-to type.
        inner: Box<TypeExpr>,
    },

    /// Nullable/optional type.
    Optional(Box<TypeExpr>),

    /// Result type.
    Result {
        /// Success type.
        ok: Box<TypeExpr>,

        /// Error type.
        error: Box<TypeExpr>,
    },

    /// Never type.
    Never,

    /// Unit type.
    Unit,

    /// `Self` type.
    SelfType,

    /// Compiler/parser inference placeholder `_`.
    ///
    /// It represents source syntax only. It must be resolved or rejected by
    /// semantic analysis where inference is not permitted.
    Infer,

    /// Generic parameter reference.
    GenericParameter(TypeParameterName),

    /// Union type.
    ///
    /// This is source-level only. Canonicalization and subtype semantics belong
    /// to semantic analysis.
    Union(Vec<TypeExpr>),

    /// Intersection type.
    ///
    /// This is source-level only. Canonicalization and trait semantics belong
    /// to semantic analysis.
    Intersection(Vec<TypeExpr>),

    /// Associated/member type projection.
    ///
    /// Example:
    ///
    /// ```text
    /// Iterator::Item
    /// ```
    Associated {
        /// Base type.
        base: Box<TypeExpr>,

        /// Associated type name.
        member: TypeName,
    },

    /// Type-level function application.
    ///
    /// This is distinct from `Generic` because a higher-kinded constructor may
    /// itself be the result of another type-level computation.
    TypeApplication {
        /// Constructor.
        constructor: Box<TypeExpr>,

        /// Ordered arguments.
        arguments: Vec<TypeExpr>,
    },

    /// Quantum resource abstraction.
    ///
    /// This is intentionally a language-level type constructor rather than a
    /// physical qubit representation.
    Quantum(Box<TypeExpr>),

    /// Linear resource type.
    Linear(Box<TypeExpr>),

    /// Affine resource type.
    Affine(Box<TypeExpr>),

    /// Temporal/resource-lifetime type.
    Temporal(Box<TypeExpr>),

    /// Dependent Pi type.
    ///
    /// Conceptually:
    ///
    /// ```text
    /// Π(parameter : parameter_type). body
    /// ```
    Pi {
        /// Bound parameter.
        parameter: TypeParameterName,

        /// Parameter type.
        parameter_type: Box<TypeExpr>,

        /// Body type.
        body: Box<TypeExpr>,
    },

    /// Dependent Sigma type.
    ///
    /// Conceptually:
    ///
    /// ```text
    /// Σ(parameter : parameter_type). body
    /// ```
    Sigma {
        /// Bound parameter.
        parameter: TypeParameterName,

        /// Parameter type.
        parameter_type: Box<TypeExpr>,

        /// Body type.
        body: Box<TypeExpr>,
    },

    /// Type identity/equality proposition.
    Identity {
        /// Left-hand type.
        left: Box<TypeExpr>,

        /// Right-hand type.
        right: Box<TypeExpr>,
    },

    /// Higher-kinded type application compatibility form.
    ///
    /// This retains the existing Zamani conceptual `Hkt(name, type)` model
    /// without tying the representation to a closed set of constructors.
    Hkt {
        /// Higher-kinded constructor name.
        constructor: TypePath,

        /// Applied type.
        argument: Box<TypeExpr>,
    },

    /// Extensible source-level type construct.
    ///
    /// The core compiler therefore does not need to be modified whenever a
    /// future computational domain introduces a new type-level abstraction.
    Extension(TypeExtension),
}

// =============================================================================
// Type-level values
// =============================================================================

/// Source-level expression used where a type requires a symbolic value.
///
/// This is deliberately not an expression AST dependency. It preserves the
/// source-level value structure necessary for type cardinalities while keeping
/// type expressions independent of the full expression subsystem.
///
/// A later parser integration may replace or wrap this with the canonical
/// expression-node representation without changing the type vocabulary.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeValueExpr {
    /// An identifier used as a symbolic type-level value.
    Identifier(TypePath),

    /// A literal integer written in source.
    ///
    /// The textual representation is preserved to avoid forcing machine-width
    /// assumptions into the AST.
    Integer(Arc<str>),

    /// A generic symbolic value.
    Generic(TypeParameterName),

    /// A namespaced extension value.
    Extension(TypeValueExtension),
}

/// Extensible type-level value.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeValueExtension {
    /// Extension namespace.
    namespace: TypeName,

    /// Extension-defined operation/name.
    name: TypeName,

    /// Extension payload.
    payload: Vec<TypeValueExpr>,
}

impl TypeValueExtension {
    /// Creates a type-level value extension.
    #[must_use]
    pub fn new<N, S>(namespace: N, name: S, payload: Vec<TypeValueExpr>) -> Self
    where
        N: Into<TypeName>,
        S: Into<TypeName>,
    {
        Self {
            namespace: namespace.into(),
            name: name.into(),
            payload,
        }
    }

    /// Returns the extension namespace.
    #[must_use]
    pub fn namespace(&self) -> &TypeName {
        &self.namespace
    }

    /// Returns the extension name.
    #[must_use]
    pub fn name(&self) -> &TypeName {
        &self.name
    }

    /// Returns the extension payload.
    #[must_use]
    pub fn payload(&self) -> &[TypeValueExpr] {
        &self.payload
    }
}

// =============================================================================
// Extensions
// =============================================================================

/// Extensible source-level type construct.
///
/// The namespace/name pair prevents the native type system from becoming a
/// closed enumeration of every possible future domain.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeExtension {
    /// Extension namespace.
    namespace: TypeName,

    /// Extension-defined type name.
    name: TypeName,

    /// Ordered type arguments.
    arguments: Vec<TypeExpr>,

    /// Extension-defined attributes.
    attributes: Vec<TypeAttribute>,
}

impl TypeExtension {
    /// Creates an extension type.
    #[must_use]
    pub fn new<N, S>(
        namespace: N,
        name: S,
        arguments: Vec<TypeExpr>,
        attributes: Vec<TypeAttribute>,
    ) -> Self
    where
        N: Into<TypeName>,
        S: Into<TypeName>,
    {
        Self {
            namespace: namespace.into(),
            name: name.into(),
            arguments,
            attributes,
        }
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &TypeName {
        &self.namespace
    }

    /// Returns the extension-defined type name.
    #[must_use]
    pub fn name(&self) -> &TypeName {
        &self.name
    }

    /// Returns type arguments.
    #[must_use]
    pub fn arguments(&self) -> &[TypeExpr] {
        &self.arguments
    }

    /// Returns extension attributes.
    #[must_use]
    pub fn attributes(&self) -> &[TypeAttribute] {
        &self.attributes
    }
}

/// Namespaced source-level type attribute.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeAttribute {
    /// Attribute namespace.
    namespace: TypeName,

    /// Attribute name.
    name: TypeName,

    /// Optional textual value.
    value: Option<Arc<str>>,
}

impl TypeAttribute {
    /// Creates a type attribute.
    #[must_use]
    pub fn new<N, S>(
        namespace: N,
        name: S,
        value: Option<String>,
    ) -> Self
    where
        N: Into<TypeName>,
        S: Into<TypeName>,
    {
        Self {
            namespace: namespace.into(),
            name: name.into(),
            value: value.map(Arc::<str>::from),
        }
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &TypeName {
        &self.namespace
    }

    /// Returns the name.
    #[must_use]
    pub fn name(&self) -> &TypeName {
        &self.name
    }

    /// Returns the optional value.
    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}

// =============================================================================
// Validation
// =============================================================================

/// Structural type-expression validation policy.
///
/// These limits belong to the compiler invocation/security policy and are not
/// part of Zamani's language semantics.
///
/// `None` means "do not impose this policy limit here".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeValidationPolicy {
    /// Maximum number of direct type children in a collection.
    pub max_collection_items: Option<usize>,

    /// Maximum byte length of a single identifier.
    pub max_identifier_bytes: Option<usize>,

    /// Maximum byte length of an extension namespace.
    pub max_extension_namespace_bytes: Option<usize>,

    /// Maximum byte length of an extension name.
    pub max_extension_name_bytes: Option<usize>,

    /// Maximum byte length of an extension attribute value.
    pub max_attribute_value_bytes: Option<usize>,
}

impl Default for TypeValidationPolicy {
    fn default() -> Self {
        Self {
            max_collection_items: None,
            max_identifier_bytes: None,
            max_extension_namespace_bytes: None,
            max_extension_name_bytes: None,
            max_attribute_value_bytes: None,
        }
    }
}

/// Structural validation errors.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TypeExprError {
    /// A required name was empty.
    EmptyName,

    /// A path contained no components.
    EmptyPath,

    /// A configured identifier limit was exceeded.
    IdentifierTooLarge {
        /// Observed byte count.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// A configured collection limit was exceeded.
    CollectionTooLarge {
        /// Observed item count.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// A configured extension namespace limit was exceeded.
    ExtensionNamespaceTooLarge {
        /// Observed byte count.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// A configured extension name limit was exceeded.
    ExtensionNameTooLarge {
        /// Observed byte count.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// A configured attribute value limit was exceeded.
    AttributeValueTooLarge {
        /// Observed byte count.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// An extension was structurally invalid.
    InvalidExtension,

    /// An extension attribute was structurally invalid.
    InvalidAttribute,
}

impl fmt::Display for TypeExprError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("type name cannot be empty"),

            Self::EmptyPath => formatter.write_str("type path cannot be empty"),

            Self::IdentifierTooLarge { actual, maximum } => {
                write!(
                    formatter,
                    "type identifier exceeds configured limit: {actual} > {maximum}"
                )
            }

            Self::CollectionTooLarge { actual, maximum } => {
                write!(
                    formatter,
                    "type collection exceeds configured limit: {actual} > {maximum}"
                )
            }

            Self::ExtensionNamespaceTooLarge { actual, maximum } => {
                write!(
                    formatter,
                    "type extension namespace exceeds configured limit: {actual} > {maximum}"
                )
            }

            Self::ExtensionNameTooLarge { actual, maximum } => {
                write!(
                    formatter,
                    "type extension name exceeds configured limit: {actual} > {maximum}"
                )
            }

            Self::AttributeValueTooLarge { actual, maximum } => {
                write!(
                    formatter,
                    "type attribute value exceeds configured limit: {actual} > {maximum}"
                )
            }

            Self::InvalidExtension => {
                formatter.write_str("type extension is structurally invalid")
            }

            Self::InvalidAttribute => {
                formatter.write_str("type attribute is structurally invalid")
            }
        }
    }
}

impl std::error::Error for TypeExprError {}

// =============================================================================
// TypeExpr implementation
// =============================================================================

impl TypeExpr {
    /// Creates a named type.
    #[must_use]
    pub fn named(path: TypePath) -> Self {
        Self::Identifier(path)
    }

    /// Creates a single-segment named type.
    #[must_use]
    pub fn name<S>(name: S) -> Self
    where
        S: Into<TypeName>,
    {
        Self::Identifier(TypePath::single(name))
    }

    /// Creates a generic type.
    #[must_use]
    pub fn generic(base: TypeExpr, arguments: Vec<TypeExpr>) -> Self {
        Self::Generic {
            base: Box::new(base),
            arguments,
        }
    }

    /// Creates a tuple type.
    #[must_use]
    pub fn tuple(elements: Vec<TypeExpr>) -> Self {
        Self::Tuple(elements)
    }

    /// Creates an array type.
    #[must_use]
    pub fn array(element: TypeExpr, length: Option<TypeValueExpr>) -> Self {
        Self::Array {
            element: Box::new(element),
            length,
        }
    }

    /// Creates a slice type.
    #[must_use]
    pub fn slice(element: TypeExpr) -> Self {
        Self::Slice(Box::new(element))
    }

    /// Creates a function type.
    #[must_use]
    pub fn function(parameters: Vec<TypeExpr>, return_type: TypeExpr) -> Self {
        Self::Function {
            parameters,
            return_type: Box::new(return_type),
        }
    }

    /// Creates a reference type.
    #[must_use]
    pub fn reference(
        mutable: bool,
        lifetime: Option<LifetimeName>,
        inner: TypeExpr,
    ) -> Self {
        Self::Reference {
            mutable,
            lifetime,
            inner: Box::new(inner),
        }
    }

    /// Creates a pointer type.
    #[must_use]
    pub fn pointer(mutable: bool, inner: TypeExpr) -> Self {
        Self::Pointer {
            mutable,
            inner: Box::new(inner),
        }
    }

    /// Creates an optional type.
    #[must_use]
    pub fn optional(inner: TypeExpr) -> Self {
        Self::Optional(Box::new(inner))
    }

    /// Creates a result type.
    #[must_use]
    pub fn result(ok: TypeExpr, error: TypeExpr) -> Self {
        Self::Result {
            ok: Box::new(ok),
            error: Box::new(error),
        }
    }

    /// Creates a quantum type.
    ///
    /// This is a source-level abstraction. It does not identify physical
    /// qubits or a quantum technology.
    #[must_use]
    pub fn quantum(inner: TypeExpr) -> Self {
        Self::Quantum(Box::new(inner))
    }

    /// Creates a linear type.
    #[must_use]
    pub fn linear(inner: TypeExpr) -> Self {
        Self::Linear(Box::new(inner))
    }

    /// Creates an affine type.
    #[must_use]
    pub fn affine(inner: TypeExpr) -> Self {
        Self::Affine(Box::new(inner))
    }

    /// Creates a temporal type.
    #[must_use]
    pub fn temporal(inner: TypeExpr) -> Self {
        Self::Temporal(Box::new(inner))
    }

    /// Creates a higher-kinded type application.
    #[must_use]
    pub fn hkt(constructor: TypePath, argument: TypeExpr) -> Self {
        Self::Hkt {
            constructor,
            argument: Box::new(argument),
        }
    }

    /// Creates an extension type.
    #[must_use]
    pub fn extension(extension: TypeExtension) -> Self {
        Self::Extension(extension)
    }

    /// Returns the number of immediate child type expressions.
    #[must_use]
    pub fn child_count(&self) -> usize {
        match self {
            Self::Identifier(_)
            | Self::Never
            | Self::Unit
            | Self::SelfType
            | Self::Infer
            | Self::GenericParameter(_) => 0,

            Self::Generic { arguments, .. }
            | Self::Tuple(arguments)
            | Self::Union(arguments)
            | Self::Intersection(arguments)
            | Self::TypeApplication { arguments, .. } => {
                arguments.len()
                    + match self {
                        Self::Generic { .. } | Self::TypeApplication { .. } => 1,
                        _ => 0,
                    }
            }

            Self::Array { .. } => 1,

            Self::Slice(inner)
            | Self::Optional(inner)
            | Self::Quantum(inner)
            | Self::Linear(inner)
            | Self::Affine(inner)
            | Self::Temporal(inner)
            | Self::Hkt {
                argument: inner, ..
            } => 1,

            Self::Function {
                parameters,
                return_type: _,
            } => parameters.len() + 1,

            Self::Reference { .. }
            | Self::Pointer { .. } => 1,

            Self::Result { .. }
            | Self::Identity { .. }
            | Self::Associated { .. } => 2,

            Self::Pi {
                parameter_type: _,
                body: _,
                ..
            }
            | Self::Sigma {
                parameter_type: _,
                body: _,
                ..
            } => 2,

            Self::Extension(extension) => extension.arguments.len(),
        }
    }

    /// Returns whether the expression has no nested type expressions.
    #[must_use]
    pub fn is_leaf(&self) -> bool {
        self.child_count() == 0
    }

    /// Returns whether this type is syntactically generic/parameterized.
    #[must_use]
    pub fn is_generic(&self) -> bool {
        matches!(
            self,
            Self::Generic { .. }
                | Self::TypeApplication { .. }
                | Self::Hkt { .. }
                | Self::Pi { .. }
                | Self::Sigma { .. }
        )
    }

    /// Returns whether this type is a source-level quantum abstraction.
    #[must_use]
    pub fn is_quantum(&self) -> bool {
        matches!(self, Self::Quantum(_))
    }

    /// Returns whether this type carries source-level linear-resource
    /// semantics.
    #[must_use]
    pub fn is_linear(&self) -> bool {
        matches!(self, Self::Linear(_))
    }

    /// Returns whether this type carries source-level affine-resource
    /// semantics.
    #[must_use]
    pub fn is_affine(&self) -> bool {
        matches!(self, Self::Affine(_))
    }

    /// Returns whether this type is source-level temporal/resource-aware.
    #[must_use]
    pub fn is_temporal(&self) -> bool {
        matches!(self, Self::Temporal(_))
    }

    /// Returns whether this type requires semantic inference.
    #[must_use]
    pub fn requires_inference(&self) -> bool {
        match self {
            Self::Infer => true,

            Self::Generic { base, arguments } => {
                base.requires_inference()
                    || arguments.iter().any(TypeExpr::requires_inference)
            }

            Self::Tuple(types)
            | Self::Union(types)
            | Self::Intersection(types)
            | Self::TypeApplication { arguments: types, .. } => {
                types.iter().any(TypeExpr::requires_inference)
                    || matches!(self, Self::TypeApplication { constructor: _, .. })
            }

            Self::Array { element, .. } => element.requires_inference(),

            Self::Slice(inner)
            | Self::Optional(inner)
            | Self::Quantum(inner)
            | Self::Linear(inner)
            | Self::Affine(inner)
            | Self::Temporal(inner)
            | Self::Hkt {
                argument: inner, ..
            } => inner.requires_inference(),

            Self::Function {
                parameters,
                return_type,
            } => {
                parameters.iter().any(TypeExpr::requires_inference)
                    || return_type.requires_inference()
            }

            Self::Reference { inner, .. } | Self::Pointer { inner, .. } => {
                inner.requires_inference()
            }

            Self::Result { ok, error } | Self::Identity { left: ok, right: error } => {
                ok.requires_inference() || error.requires_inference()
            }

            Self::Associated { base, .. } => base.requires_inference(),

            Self::Pi {
                parameter_type,
                body,
                ..
            }
            | Self::Sigma {
                parameter_type,
                body,
                ..
            } => {
                parameter_type.requires_inference() || body.requires_inference()
            }

            Self::Extension(extension) => extension
                .arguments
                .iter()
                .any(TypeExpr::requires_inference),

            Self::Identifier(_)
            | Self::Never
            | Self::Unit
            | Self::SelfType
            | Self::GenericParameter(_) => false,
        }
    }

    /// Returns the top-level named path, if the type is directly named.
    #[must_use]
    pub fn path(&self) -> Option<&TypePath> {
        match self {
            Self::Identifier(path) => Some(path),
            _ => None,
        }
    }

    /// Returns the top-level type name, if it is a single named identifier.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.path()
            .and_then(TypePath::last)
            .map(TypeName::as_str)
    }

    /// Performs local structural validation using the default policy.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.validate_with_policy(&TypeValidationPolicy::default())
    }

    /// Performs local structural validation using an explicit caller policy.
    ///
    /// The validation is iterative rather than recursively calling itself.
    /// This prevents arbitrarily deep source input from exhausting the Rust
    /// call stack during validation.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        let mut stack = vec![self];

        while let Some(current) = stack.pop() {
            match current {
                Self::Identifier(path) => {
                    validate_path(path, policy)?;
                }

                Self::Generic { base, arguments } => {
                    validate_collection(arguments.len(), policy)?;
                    stack.push(base);

                    for argument in arguments.iter().rev() {
                        stack.push(argument);
                    }
                }

                Self::Tuple(elements)
                | Self::Union(elements)
                | Self::Intersection(elements) => {
                    validate_collection(elements.len(), policy)?;

                    for element in elements.iter().rev() {
                        stack.push(element);
                    }
                }

                Self::Array { element, length } => {
                    stack.push(element);

                    if let Some(value) = length {
                        validate_value_expr(value, policy)?;
                    }
                }

                Self::Slice(inner)
                | Self::Optional(inner)
                | Self::Quantum(inner)
                | Self::Linear(inner)
                | Self::Affine(inner)
                | Self::Temporal(inner) => {
                    stack.push(inner);
                }

                Self::Function {
                    parameters,
                    return_type,
                } => {
                    validate_collection(parameters.len(), policy)?;

                    stack.push(return_type);

                    for parameter in parameters.iter().rev() {
                        stack.push(parameter);
                    }
                }

                Self::Reference {
                    lifetime,
                    inner,
                    ..
                } => {
                    if let Some(lifetime) = lifetime {
                        validate_name(lifetime.as_str(), policy)?;
                    }

                    stack.push(inner);
                }

                Self::Pointer { inner, .. } => {
                    stack.push(inner);
                }

                Self::Result { ok, error } => {
                    stack.push(error);
                    stack.push(ok);
                }

                Self::Never | Self::Unit | Self::SelfType | Self::Infer => {}

                Self::GenericParameter(parameter) => {
                    validate_name(parameter.as_str(), policy)?;
                }

                Self::Associated { base, member } => {
                    validate_name(member.as_str(), policy)?;
                    stack.push(base);
                }

                Self::TypeApplication {
                    constructor,
                    arguments,
                } => {
                    validate_collection(arguments.len(), policy)?;

                    stack.push(constructor);

                    for argument in arguments.iter().rev() {
                        stack.push(argument);
                    }
                }

                Self::Pi {
                    parameter,
                    parameter_type,
                    body,
                }
                | Self::Sigma {
                    parameter,
                    parameter_type,
                    body,
                } => {
                    validate_name(parameter.as_str(), policy)?;
                    stack.push(body);
                    stack.push(parameter_type);
                }

                Self::Identity { left, right } => {
                    stack.push(right);
                    stack.push(left);
                }

                Self::Hkt {
                    constructor,
                    argument,
                } => {
                    validate_path(constructor, policy)?;
                    stack.push(argument);
                }

                Self::Extension(extension) => {
                    validate_extension(extension, policy)?;

                    for argument in extension.arguments.iter().rev() {
                        stack.push(argument);
                    }
                }
            }
        }

        Ok(())
    }

    /// Returns the source-level spelling of the type.
    ///
    /// This is a deterministic structural formatter, not a semantic pretty
    /// printer. It does not resolve aliases or canonicalize equivalent types.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        match self {
            Self::Identifier(path) => path.to_source_string(),

            Self::Generic { base, arguments } => {
                format!(
                    "{}<{}>",
                    base.to_source_string(),
                    join_types(arguments)
                )
            }

            Self::Tuple(elements) => {
                if elements.is_empty() {
                    "()".to_owned()
                } else {
                    format!("({})", join_types(elements))
                }
            }

            Self::Array { element, length } => match length {
                Some(length) => {
                    format!("[{}; {}]", element.to_source_string(), length.to_source_string())
                }
                None => format!("[{}]", element.to_source_string()),
            },

            Self::Slice(inner) => {
                format!("[{}]", inner.to_source_string())
            }

            Self::Function {
                parameters,
                return_type,
            } => {
                format!(
                    "fn({}) -> {}",
                    join_types(parameters),
                    return_type.to_source_string()
                )
            }

            Self::Reference {
                mutable,
                lifetime,
                inner,
            } => {
                let mut result = String::from("&");

                if let Some(lifetime) = lifetime {
                    result.push_str(&lifetime.to_string());
                    result.push(' ');
                }

                if *mutable {
                    result.push_str("mut ");
                }

                result.push_str(&inner.to_source_string());
                result
            }

            Self::Pointer { mutable, inner } => {
                if *mutable {
                    format!("*mut {}", inner.to_source_string())
                } else {
                    format!("*const {}", inner.to_source_string())
                }
            }

            Self::Optional(inner) => {
                format!("?{}", inner.to_source_string())
            }

            Self::Result { ok, error } => {
                format!(
                    "Result<{}, {}>",
                    ok.to_source_string(),
                    error.to_source_string()
                )
            }

            Self::Never => "!".to_owned(),

            Self::Unit => "()".to_owned(),

            Self::SelfType => "Self".to_owned(),

            Self::Infer => "_".to_owned(),

            Self::GenericParameter(parameter) => parameter.to_string(),

            Self::Union(types) => join_types_with(types, " | "),

            Self::Intersection(types) => join_types_with(types, " & "),

            Self::Associated { base, member } => {
                format!("{}::{}", base.to_source_string(), member)
            }

            Self::TypeApplication {
                constructor,
                arguments,
            } => {
                format!(
                    "{}<{}>",
                    constructor.to_source_string(),
                    join_types(arguments)
                )
            }

            Self::Quantum(inner) => {
                format!("Quantum<{}>", inner.to_source_string())
            }

            Self::Linear(inner) => {
                format!("Linear<{}>", inner.to_source_string())
            }

            Self::Affine(inner) => {
                format!("Affine<{}>", inner.to_source_string())
            }

            Self::Temporal(inner) => {
                format!("Temporal<{}>", inner.to_source_string())
            }

            Self::Pi {
                parameter,
                parameter_type,
                body,
            } => {
                format!(
                    "Pi<{}: {}, {}>",
                    parameter,
                    parameter_type.to_source_string(),
                    body.to_source_string()
                )
            }

            Self::Sigma {
                parameter,
                parameter_type,
                body,
            } => {
                format!(
                    "Sigma<{}: {}, {}>",
                    parameter,
                    parameter_type.to_source_string(),
                    body.to_source_string()
                )
            }

            Self::Identity { left, right } => {
                format!(
                    "Identity<{}, {}>",
                    left.to_source_string(),
                    right.to_source_string()
                )
            }

            Self::Hkt {
                constructor,
                argument,
            } => {
                format!(
                    "hkt<{}, {}>",
                    constructor,
                    argument.to_source_string()
                )
            }

            Self::Extension(extension) => {
                let mut result = format!(
                    "{}::{}",
                    extension.namespace,
                    extension.name
                );

                if !extension.arguments.is_empty() {
                    result.push('<');
                    result.push_str(&join_types(&extension.arguments));
                    result.push('>');
                }

                result
            }
        }
    }

    /// Returns the canonical stable variant identifier.
    ///
    /// This identifier is source-AST schema metadata and must not be confused
    /// with a backend instruction identifier.
    #[must_use]
    pub fn variant_name(&self) -> &'static str {
        match self {
            Self::Identifier(_) => "identifier",
            Self::Generic { .. } => "generic",
            Self::Tuple(_) => "tuple",
            Self::Array { .. } => "array",
            Self::Slice(_) => "slice",
            Self::Function { .. } => "function",
            Self::Reference { .. } => "reference",
            Self::Pointer { .. } => "pointer",
            Self::Optional(_) => "optional",
            Self::Result { .. } => "result",
            Self::Never => "never",
            Self::Unit => "unit",
            Self::SelfType => "self",
            Self::Infer => "infer",
            Self::GenericParameter(_) => "generic_parameter",
            Self::Union(_) => "union",
            Self::Intersection(_) => "intersection",
            Self::Associated { .. } => "associated",
            Self::TypeApplication { .. } => "type_application",
            Self::Quantum(_) => "quantum",
            Self::Linear(_) => "linear",
            Self::Affine(_) => "affine",
            Self::Temporal(_) => "temporal",
            Self::Pi { .. } => "pi",
            Self::Sigma { .. } => "sigma",
            Self::Identity { .. } => "identity",
            Self::Hkt { .. } => "hkt",
            Self::Extension(_) => "extension",
        }
    }
}

// =============================================================================
// Type-level value helpers
// =============================================================================

impl TypeValueExpr {
    /// Returns the deterministic source representation.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        match self {
            Self::Identifier(path) => path.to_source_string(),

            Self::Integer(value) => value.as_ref().to_owned(),

            Self::Generic(parameter) => parameter.to_string(),

            Self::Extension(extension) => {
                let mut result = format!(
                    "{}::{}",
                    extension.namespace,
                    extension.name
                );

                if !extension.payload.is_empty() {
                    result.push('(');

                    for (index, value) in extension.payload.iter().enumerate() {
                        if index != 0 {
                            result.push_str(", ");
                        }

                        result.push_str(&value.to_source_string());
                    }

                    result.push(')');
                }

                result
            }
        }
    }
}

// =============================================================================
// Internal validation helpers
// =============================================================================

fn validate_collection(
    actual: usize,
    policy: &TypeValidationPolicy,
) -> Result<(), TypeExprError> {
    if let Some(maximum) = policy.max_collection_items {
        if actual > maximum {
            return Err(TypeExprError::CollectionTooLarge { actual, maximum });
        }
    }

    Ok(())
}

fn validate_name(
    name: &str,
    policy: &TypeValidationPolicy,
) -> Result<(), TypeExprError> {
    if name.is_empty() {
        return Err(TypeExprError::EmptyName);
    }

    if let Some(maximum) = policy.max_identifier_bytes {
        if name.len() > maximum {
            return Err(TypeExprError::IdentifierTooLarge {
                actual: name.len(),
                maximum,
            });
        }
    }

    Ok(())
}

fn validate_path(
    path: &TypePath,
    policy: &TypeValidationPolicy,
) -> Result<(), TypeExprError> {
    if path.is_empty() {
        return Err(TypeExprError::EmptyPath);
    }

    for segment in path.segments() {
        validate_name(segment.as_str(), policy)?;
    }

    Ok(())
}

fn validate_value_expr(
    value: &TypeValueExpr,
    policy: &TypeValidationPolicy,
) -> Result<(), TypeExprError> {
    match value {
        TypeValueExpr::Identifier(path) => validate_path(path, policy),

        TypeValueExpr::Integer(value) => {
            if let Some(maximum) = policy.max_identifier_bytes {
                if value.len() > maximum {
                    return Err(TypeExprError::IdentifierTooLarge {
                        actual: value.len(),
                        maximum,
                    });
                }
            }

            Ok(())
        }

        TypeValueExpr::Generic(parameter) => {
            validate_name(parameter.as_str(), policy)
        }

        TypeValueExpr::Extension(extension) => {
            validate_name(extension.namespace.as_str(), policy)?;
            validate_name(extension.name.as_str(), policy)?;

            if let Some(maximum) = policy.max_collection_items {
                if extension.payload.len() > maximum {
                    return Err(TypeExprError::CollectionTooLarge {
                        actual: extension.payload.len(),
                        maximum,
                    });
                }
            }

            for payload in &extension.payload {
                validate_value_expr(payload, policy)?;
            }

            Ok(())
        }
    }
}

fn validate_extension(
    extension: &TypeExtension,
    policy: &TypeValidationPolicy,
) -> Result<(), TypeExprError> {
    if extension.namespace.is_empty() || extension.name.is_empty() {
        return Err(TypeExprError::InvalidExtension);
    }

    if let Some(maximum) = policy.max_extension_namespace_bytes {
        if extension.namespace.len() > maximum {
            return Err(TypeExprError::ExtensionNamespaceTooLarge {
                actual: extension.namespace.len(),
                maximum,
            });
        }
    }

    if let Some(maximum) = policy.max_extension_name_bytes {
        if extension.name.len() > maximum {
            return Err(TypeExprError::ExtensionNameTooLarge {
                actual: extension.name.len(),
                maximum,
            });
        }
    }

    validate_collection(extension.arguments.len(), policy)?;

    for attribute in &extension.attributes {
        validate_attribute(attribute, policy)?;
    }

    Ok(())
}

fn validate_attribute(
    attribute: &TypeAttribute,
    policy: &TypeValidationPolicy,
) -> Result<(), TypeExprError> {
    if attribute.namespace.is_empty() || attribute.name.is_empty() {
        return Err(TypeExprError::InvalidAttribute);
    }

    validate_name(attribute.namespace.as_str(), policy)?;
    validate_name(attribute.name.as_str(), policy)?;

    if let Some(value) = attribute.value.as_deref() {
        if let Some(maximum) = policy.max_attribute_value_bytes {
            if value.len() > maximum {
                return Err(TypeExprError::AttributeValueTooLarge {
                    actual: value.len(),
                    maximum,
                });
            }
        }
    }

    Ok(())
}

fn join_types(types: &[TypeExpr]) -> String {
    join_types_with(types, ", ")
}

fn join_types_with(types: &[TypeExpr], separator: &str) -> String {
    let mut result = String::new();

    for (index, ty) in types.iter().enumerate() {
        if index != 0 {
            result.push_str(separator);
        }

        result.push_str(&ty.to_source_string());
    }

    result
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_type_round_trips() {
        let ty = TypeExpr::name("Int");

        assert_eq!(ty.name(), Some("Int"));
        assert_eq!(ty.to_source_string(), "Int");
        assert_eq!(ty.variant_name(), "identifier");
        assert!(ty.validate().is_ok());
    }

    #[test]
    fn qualified_type_is_preserved() {
        let ty = TypeExpr::named(TypePath::from_names([
            "std",
            "collections",
            "Map",
        ]));

        assert_eq!(ty.to_source_string(), "std::collections::Map");
        assert!(ty.validate().is_ok());
    }

    #[test]
    fn generic_type_supports_arbitrary_arity() {
        let ty = TypeExpr::generic(
            TypeExpr::name("Map"),
            vec![
                TypeExpr::name("String"),
                TypeExpr::name("Int"),
                TypeExpr::name("Bool"),
            ],
        );

        assert_eq!(ty.to_source_string(), "Map<String, Int, Bool>");
        assert_eq!(ty.child_count(), 4);
        assert!(ty.validate().is_ok());
    }

    #[test]
    fn quantum_type_is_hardware_neutral() {
        let ty = TypeExpr::quantum(TypeExpr::name("Q"));

        assert!(ty.is_quantum());
        assert_eq!(ty.to_source_string(), "Quantum<Q>");
        assert!(ty.validate().is_ok());
    }

    #[test]
    fn quantum_type_does_not_encode_qubit_count() {
        let ty = TypeExpr::quantum(TypeExpr::generic(
            TypeExpr::name("Register"),
            vec![TypeExpr::name("N")],
        ));

        assert_eq!(ty.to_source_string(), "Quantum<Register<N>>");
    }

    #[test]
    fn function_type_supports_dynamic_parameter_count() {
        let ty = TypeExpr::function(
            vec![
                TypeExpr::name("A"),
                TypeExpr::name("B"),
                TypeExpr::name("C"),
            ],
            TypeExpr::name("Result"),
        );

        assert_eq!(ty.to_source_string(), "fn(A, B, C) -> Result");
        assert!(ty.validate().is_ok());
    }

    #[test]
    fn symbolic_array_length_is_not_machine_sized() {
        let ty = TypeExpr::array(
            TypeExpr::name("Qubit"),
            Some(TypeValueExpr::Generic(TypeParameterName::from("N"))),
        );

        assert_eq!(ty.to_source_string(), "[Qubit; N]");
        assert!(ty.validate().is_ok());
    }

    #[test]
    fn reference_preserves_lifetime_and_mutability() {
        let ty = TypeExpr::reference(
            true,
            Some(LifetimeName::from("a")),
            TypeExpr::name("T"),
        );

        assert_eq!(ty.to_source_string(), "&'a mut T");
        assert!(ty.validate().is_ok());
    }

    #[test]
    fn dependent_types_are_source_level_only() {
        let ty = TypeExpr::Pi {
            parameter: TypeParameterName::from("N"),
            parameter_type: Box::new(TypeExpr::name("Nat")),
            body: Box::new(TypeExpr::array(
                TypeExpr::name("Qubit"),
                Some(TypeValueExpr::Generic(TypeParameterName::from("N"))),
            )),
        };

        assert_eq!(ty.to_source_string(), "Pi<N: Nat, [Qubit; N]>");
        assert!(ty.validate().is_ok());
    }

    #[test]
    fn extension_type_is_open_ended() {
        let extension = TypeExtension::new(
            "zamani.quantum",
            "logical_register",
            vec![TypeExpr::name("N")],
            vec![],
        );

        let ty = TypeExpr::extension(extension);

        assert_eq!(
            ty.to_source_string(),
            "zamani.quantum::logical_register<N>"
        );
        assert!(ty.validate().is_ok());
    }

    #[test]
    fn malformed_empty_path_is_rejected() {
        let ty = TypeExpr::Identifier(TypePath::default());

        assert_eq!(
            ty.validate(),
            Err(TypeExprError::EmptyPath)
        );
    }

    #[test]
    fn configurable_limits_are_policy_not_language_constants() {
        let ty = TypeExpr::tuple(vec![
            TypeExpr::name("A"),
            TypeExpr::name("B"),
            TypeExpr::name("C"),
        ]);

        let policy = TypeValidationPolicy {
            max_collection_items: Some(2),
            ..TypeValidationPolicy::default()
        };

        assert_eq!(
            ty.validate_with_policy(&policy),
            Err(TypeExprError::CollectionTooLarge {
                actual: 3,
                maximum: 2,
            })
        );
    }

    #[test]
    fn deep_type_validation_is_iterative() {
        let mut ty = TypeExpr::name("Leaf");

        for _ in 0..4096 {
            ty = TypeExpr::optional(ty);
        }

        assert!(ty.validate().is_ok());
    }

    #[test]
    fn serialization_round_trip_preserves_structure() {
        let ty = TypeExpr::generic(
            TypeExpr::name("Quantum"),
            vec![
                TypeExpr::array(
                    TypeExpr::name("Qubit"),
                    Some(TypeValueExpr::Integer(Arc::<str>::from("N"))),
                ),
            ],
        );

        let encoded =
            serde_json::to_string(&ty).expect("serialization must succeed");

        let decoded: TypeExpr =
            serde_json::from_str(&encoded).expect("deserialization must succeed");

        assert_eq!(decoded, ty);
    }

    #[test]
    fn schema_version_is_explicit() {
        assert_eq!(TYPE_EXPR_SCHEMA_VERSION, 1);
    }

    #[test]
    fn special_source_types_are_stable() {
        assert_eq!(TypeExpr::Never.to_source_string(), "!");
        assert_eq!(TypeExpr::Unit.to_source_string(), "()");
        assert_eq!(TypeExpr::SelfType.to_source_string(), "Self");
        assert_eq!(TypeExpr::Infer.to_source_string(), "_");
    }
}