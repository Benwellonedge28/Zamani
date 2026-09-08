//! Source-origin and provenance infrastructure for the native Zamani AST.
//!
//! # Architectural role
//!
//! `SourceOrigin` describes the provenance of an AST node independently from
//! the node's source [`Span`](super::span::Span).
//!
//! A `Span` answers:
//!
//!     "Where is this node located in a source coordinate space?"
//!
//! `SourceOrigin` answers:
//!
//!     "Where did this node come from, and how was that origin produced?"
//!
//! These are deliberately separate concepts.
//!
//! # Compilation boundary
//!
//! ```text
//! source text
//!     │
//!     ▼
//! lexer / parser
//!     │
//!     ▼
//! native Zamani AST
//!     │
//!     ├── NodeId
//!     ├── Span
//!     ├── SourceOrigin  ← this module
//!     └── metadata
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
//! domain / target / hardware lowering
//! ```
//!
//! # Domain neutrality
//!
//! This module intentionally knows nothing about:
//!
//! - quantum hardware;
//! - qubits;
//! - quantum topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - backends;
//! - vendors;
//! - CPU/GPU/FPGA instructions;
//! - LLVM;
//! - QIR;
//! - MLIR;
//! - runtime execution.
//!
//! Source provenance is applicable equally to classical, quantum, hybrid,
//! HDL, distributed, accelerator, AI, and future Zamani constructs.
//!
//! # POCO-REAF
//!
//! Source provenance contains no machine-size assumptions.
//!
//! It does not contain:
//!
//! - a machine identifier;
//! - a device identifier;
//! - a qubit count;
//! - a hardware topology;
//! - a backend identifier;
//! - a fixed resource array;
//! - a fixed list of computational domains.
//!
//! Therefore the same provenance model remains valid regardless of the size
//! or technology of the eventual execution target.
//!
//! # Coordinate relationship
//!
//! `SourceOrigin` may contain one or more [`Span`] values, but it never
//! reimplements source coordinates.
//!
//! `Span` remains authoritative for source coordinates.
//!
//! # Provenance model
//!
//! The model distinguishes between:
//!
//! * `Source` — directly written source text.
//! * `Generated` — compiler/tool-generated source or AST.
//! * `MacroExpansion` — produced through macro expansion.
//! * `Imported` — originating in an imported external representation.
//! * `Desugared` — produced while lowering syntactic sugar.
//! * `Transformed` — produced by an AST transformation.
//! * `Synthetic` — compiler-created with no direct source spelling.
//! * `Unknown` — provenance unavailable or intentionally unspecified.
//!
//! These categories describe *provenance*, not semantics.
//!
//! # Important invariant
//!
//! Provenance must never be used as a substitute for semantic identity.
//!
//! For example, two generated nodes can have the same origin while remaining
//! completely different AST nodes.
//!
//! # Scalability
//!
//! This type contains no language-defined finite limit on:
//!
//! - source files;
//! - generated files;
//! - macro expansions;
//! - transformation depth;
//! - imported formats;
//! - provenance records.
//!
//! Operational limits belong to configurable compiler policies.
//!
//! The coordinate representation is inherited from `Span` and therefore uses
//! the repository's canonical `u64` source-coordinate model.
//!
//! "Infinity" in POCO-REAF means that the AST introduces no artificial
//! machine-size limit; actual execution remains bounded by representable
//! coordinates, process resources, storage, and explicitly configured
//! compiler policies.
//!
//! # Determinism
//!
//! This module contains no:
//!
//! - memory addresses;
//! - timestamps;
//! - random identifiers;
//! - thread-local state;
//! - global mutable state;
//! - filesystem-dependent identity.
//!
//! Provenance values are therefore deterministic when their supplied inputs
//! are deterministic.
//!
//! # Serialization
//!
//! All public provenance structures derive `Serialize` and `Deserialize`.
//!
//! Serialization represents logical provenance only. No Rust pointer,
//! filesystem handle, process identifier, or machine-specific state is
//! serialized.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.
//!
//! # Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - edition 2021
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! The module may be consumed by:
//!
//! - AST nodes;
//! - AST metadata;
//! - source maps;
//! - parser;
//! - macro infrastructure;
//! - generated-code infrastructure;
//! - AST transformations;
//! - structural validation;
//! - diagnostics;
//! - visitors;
//! - traversal;
//! - serialization;
//! - semantic provenance.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - hardware;
//! - optimization;
//! - routing;
//! - scheduling;
//! - execution;
//! - runtime;
//! - backend implementations.

// -----------------------------------------------------------------------------
// Imports
// -----------------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use std::fmt;

use super::span::{SourceId, Span};

// -----------------------------------------------------------------------------
// Schema version
// -----------------------------------------------------------------------------

/// Version of the in-memory source-origin contract.
///
/// This is deliberately independent from:
///
/// - the Zamani language version;
/// - the AST schema version;
/// - the serialization schema version;
/// - the compiler version;
/// - extension versions.
pub const SOURCE_ORIGIN_SCHEMA_VERSION: u16 = 1;

// -----------------------------------------------------------------------------
// Origin identifiers
// -----------------------------------------------------------------------------

/// Stable identifier for a provenance record.
///
/// The identifier is an opaque logical identifier and does not encode:
///
/// - memory addresses;
/// - filesystem addresses;
/// - process IDs;
/// - backend IDs;
/// - machine IDs.
///
/// `u64` provides a stable, platform-independent serialized representation.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[repr(transparent)]
pub struct OriginId(u64);

impl OriginId {
    /// Creates an origin identifier from its stable raw representation.
    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    /// Returns the stable raw representation.
    #[must_use]
    pub const fn as_raw(self) -> u64 {
        self.0
    }

    /// Returns whether this is the zero/default identifier.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl From<u64> for OriginId {
    fn from(value: u64) -> Self {
        Self::from_raw(value)
    }
}

impl From<OriginId> for u64 {
    fn from(value: OriginId) -> Self {
        value.as_raw()
    }
}

impl fmt::Display for OriginId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "origin#{}", self.0)
    }
}

// -----------------------------------------------------------------------------
// Origin kinds
// -----------------------------------------------------------------------------

/// Classification of AST/source provenance.
///
/// This is intentionally a closed set of **provenance categories**, not a
/// closed set of computational domains.
///
/// New computational domains do not require modification of this enum.
///
/// Domain identity belongs elsewhere in the AST extension/domain system.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum SourceOriginKind {
    /// Directly represented in user/source input.
    #[default]
    Source,

    /// Created by a compiler, generator, or tool.
    Generated,

    /// Produced by macro expansion.
    MacroExpansion,

    /// Imported from another source representation or external format.
    Imported,

    /// Produced while eliminating source-language syntactic sugar.
    Desugared,

    /// Produced by an AST transformation.
    Transformed,

    /// Created by the compiler without a direct source spelling.
    Synthetic,

    /// Provenance is unavailable or intentionally unspecified.
    Unknown,
}

impl SourceOriginKind {
    /// Returns whether this origin represents direct source input.
    #[must_use]
    pub const fn is_source(self) -> bool {
        matches!(self, Self::Source)
    }

    /// Returns whether this origin was generated or transformed.
    #[must_use]
    pub const fn is_derived(self) -> bool {
        matches!(
            self,
            Self::Generated
                | Self::MacroExpansion
                | Self::Desugared
                | Self::Transformed
                | Self::Synthetic
        )
    }

    /// Returns whether this origin represents an imported representation.
    #[must_use]
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::Imported)
    }

    /// Returns whether this origin is explicitly synthetic.
    #[must_use]
    pub const fn is_synthetic(self) -> bool {
        matches!(self, Self::Synthetic)
    }

    /// Returns a stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Generated => "generated",
            Self::MacroExpansion => "macro_expansion",
            Self::Imported => "imported",
            Self::Desugared => "desugared",
            Self::Transformed => "transformed",
            Self::Synthetic => "synthetic",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for SourceOriginKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// -----------------------------------------------------------------------------
// External/import identity
// -----------------------------------------------------------------------------

/// Stable identity for an external provenance namespace.
///
/// This deliberately stores a namespace string rather than an enum of known
/// formats.
///
/// Consequently, adding:
///
/// - OpenQASM;
/// - QIR;
/// - Quil;
/// - future quantum formats;
/// - HDL formats;
/// - future computational representations
///
/// does not require changing the core AST provenance model.
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct OriginNamespace {
    value: String,
}

impl OriginNamespace {
    /// Creates a non-normalized provenance namespace.
    ///
    /// The value is kept exactly as supplied. Validation of namespace syntax
    /// belongs to the extension/import subsystem because the core AST must not
    /// impose a domain-specific namespace grammar.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the namespace.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Consumes the wrapper and returns the owned namespace.
    #[must_use]
    pub fn into_string(self) -> String {
        self.value
    }

    /// Returns whether the namespace is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl From<String> for OriginNamespace {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for OriginNamespace {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl AsRef<str> for OriginNamespace {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for OriginNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.value)
    }
}

// -----------------------------------------------------------------------------
// Provenance reference
// -----------------------------------------------------------------------------

/// A reference to an earlier provenance record.
///
/// Provenance forms a logical chain:
///
/// ```text
/// source
///   │
///   ▼
/// macro expansion
///   │
///   ▼
/// desugaring
///   │
///   ▼
/// transformation
///   │
///   ▼
/// generated AST node
/// ```
///
/// The reference is optional because not every generated/synthetic node has a
/// meaningful parent provenance record.
///
/// Cycles are not prevented by this low-level value because the type itself
/// does not own a provenance graph. Cycle detection belongs to the provenance
/// registry/validator.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct OriginRef {
    id: OriginId,
}

impl OriginRef {
    /// Creates a provenance reference.
    #[must_use]
    pub const fn new(id: OriginId) -> Self {
        Self { id }
    }

    /// Returns the referenced provenance identifier.
    #[must_use]
    pub const fn id(self) -> OriginId {
        self.id
    }
}

impl From<OriginId> for OriginRef {
    fn from(value: OriginId) -> Self {
        Self::new(value)
    }
}

// -----------------------------------------------------------------------------
// Source-origin descriptor
// -----------------------------------------------------------------------------

/// Immutable description of where an AST construct originated.
///
/// `SourceOrigin` is intentionally a value object. It does not own:
///
/// - source text;
/// - a source-map;
/// - a filesystem handle;
/// - an AST node;
/// - a semantic model;
/// - a compiler session.
///
/// This keeps it cheap to embed in AST metadata and safe to serialize.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SourceOrigin {
    /// Classification of the provenance.
    kind: SourceOriginKind,

    /// Stable provenance identifier.
    ///
    /// `OriginId::default()` may be used when the surrounding infrastructure
    /// does not allocate explicit provenance IDs.
    id: OriginId,

    /// Source span from which the construct originated, when one exists.
    ///
    /// A generated node may legitimately have no direct source span.
    span: Option<Span>,

    /// Previous provenance record, when this origin was derived from another
    /// origin.
    parent: Option<OriginRef>,

    /// Optional namespace identifying the external producer/representation.
    ///
    /// Examples might include a future external quantum format or a generated
    /// toolchain component. The native AST does not interpret the namespace.
    namespace: Option<OriginNamespace>,

    /// Optional logical name supplied by the producer.
    ///
    /// This is not a filesystem path and must not be interpreted as one.
    name: Option<String>,
}

impl SourceOrigin {
    /// Creates a direct source origin.
    ///
    /// The supplied span should normally correspond to the source spelling of
    /// the node.
    #[must_use]
    pub const fn source(id: OriginId, span: Span) -> Self {
        Self {
            kind: SourceOriginKind::Source,
            id,
            span: Some(span),
            parent: None,
            namespace: None,
            name: None,
        }
    }

    /// Creates a generated origin with an optional parent provenance.
    #[must_use]
    pub const fn generated(
        id: OriginId,
        span: Option<Span>,
        parent: Option<OriginRef>,
    ) -> Self {
        Self {
            kind: SourceOriginKind::Generated,
            id,
            span,
            parent,
            namespace: None,
            name: None,
        }
    }

    /// Creates a macro-expansion origin.
    #[must_use]
    pub const fn macro_expansion(
        id: OriginId,
        span: Option<Span>,
        parent: Option<OriginRef>,
    ) -> Self {
        Self {
            kind: SourceOriginKind::MacroExpansion,
            id,
            span,
            parent,
            namespace: None,
            name: None,
        }
    }

    /// Creates an imported origin.
    ///
    /// The namespace can identify an external representation without coupling
    /// the native AST to that representation.
    #[must_use]
    pub fn imported(
        id: OriginId,
        span: Option<Span>,
        namespace: Option<OriginNamespace>,
        name: Option<String>,
    ) -> Self {
        Self {
            kind: SourceOriginKind::Imported,
            id,
            span,
            parent: None,
            namespace,
            name,
        }
    }

    /// Creates a desugared origin.
    #[must_use]
    pub const fn desugared(
        id: OriginId,
        span: Option<Span>,
        parent: Option<OriginRef>,
    ) -> Self {
        Self {
            kind: SourceOriginKind::Desugared,
            id,
            span,
            parent,
            namespace: None,
            name: None,
        }
    }

    /// Creates a transformed origin.
    #[must_use]
    pub const fn transformed(
        id: OriginId,
        span: Option<Span>,
        parent: Option<OriginRef>,
    ) -> Self {
        Self {
            kind: SourceOriginKind::Transformed,
            id,
            span,
            parent,
            namespace: None,
            name: None,
        }
    }

    /// Creates a synthetic origin.
    #[must_use]
    pub const fn synthetic(id: OriginId) -> Self {
        Self {
            kind: SourceOriginKind::Synthetic,
            id,
            span: None,
            parent: None,
            namespace: None,
            name: None,
        }
    }

    /// Creates an unknown origin.
    #[must_use]
    pub const fn unknown(id: OriginId) -> Self {
        Self {
            kind: SourceOriginKind::Unknown,
            id,
            span: None,
            parent: None,
            namespace: None,
            name: None,
        }
    }

    /// Creates an origin using all available fields.
    ///
    /// This is the primary constructor for infrastructure that needs to
    /// deserialize or construct provenance without choosing a convenience
    /// constructor.
    #[must_use]
    pub fn new(
        kind: SourceOriginKind,
        id: OriginId,
        span: Option<Span>,
        parent: Option<OriginRef>,
        namespace: Option<OriginNamespace>,
        name: Option<String>,
    ) -> Self {
        Self {
            kind,
            id,
            span,
            parent,
            namespace,
            name,
        }
    }

    /// Returns the provenance category.
    #[must_use]
    pub const fn kind(&self) -> SourceOriginKind {
        self.kind
    }

    /// Returns the stable provenance identifier.
    #[must_use]
    pub const fn id(&self) -> OriginId {
        self.id
    }

    /// Returns the originating span, if available.
    #[must_use]
    pub const fn span(&self) -> Option<Span> {
        self.span
    }

    /// Returns the parent provenance reference, if any.
    #[must_use]
    pub const fn parent(&self) -> Option<OriginRef> {
        self.parent
    }

    /// Returns the external namespace, if present.
    #[must_use]
    pub fn namespace(&self) -> Option<&OriginNamespace> {
        self.namespace.as_ref()
    }

    /// Returns the optional logical producer/name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns whether this is direct source provenance.
    #[must_use]
    pub const fn is_source(&self) -> bool {
        self.kind.is_source()
    }

    /// Returns whether this is derived provenance.
    #[must_use]
    pub const fn is_derived(&self) -> bool {
        self.kind.is_derived()
    }

    /// Returns whether this is imported provenance.
    #[must_use]
    pub const fn is_imported(&self) -> bool {
        self.kind.is_imported()
    }

    /// Returns whether this origin has an associated source span.
    #[must_use]
    pub const fn has_span(&self) -> bool {
        self.span.is_some()
    }

    /// Returns whether this origin has a parent provenance record.
    #[must_use]
    pub const fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    /// Returns the schema version of this source-origin contract.
    #[must_use]
    pub const fn schema_version() -> u16 {
        SOURCE_ORIGIN_SCHEMA_VERSION
    }

    /// Returns a new origin with a different parent.
    ///
    /// The original value is not modified.
    #[must_use]
    pub const fn with_parent(mut self, parent: Option<OriginRef>) -> Self {
        self.parent = parent;
        self
    }

    /// Returns a new origin with a different span.
    ///
    /// The original value is not modified.
    #[must_use]
    pub const fn with_span(mut self, span: Option<Span>) -> Self {
        self.span = span;
        self
    }

    /// Returns a new origin with a namespace.
    ///
    /// The original value is not modified.
    #[must_use]
    pub fn with_namespace(
        mut self,
        namespace: Option<OriginNamespace>,
    ) -> Self {
        self.namespace = namespace;
        self
    }

    /// Returns a new origin with a logical producer/name.
    ///
    /// The original value is not modified.
    #[must_use]
    pub fn with_name(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }

    /// Returns the source identifier when a span exists.
    #[must_use]
    pub const fn source_id(&self) -> Option<SourceId> {
        match self.span {
            Some(span) => Some(span.source()),
            None => None,
        }
    }

    /// Returns a human-readable compact description.
    ///
    /// This deliberately does not include arbitrary names or namespaces, which
    /// may be extremely large extension payloads.
    #[must_use]
    pub fn diagnostic_label(&self) -> String {
        match self.kind {
            SourceOriginKind::Source => String::from("source"),
            SourceOriginKind::Generated => String::from("generated"),
            SourceOriginKind::MacroExpansion => String::from("macro expansion"),
            SourceOriginKind::Imported => String::from("imported"),
            SourceOriginKind::Desugared => String::from("desugared"),
            SourceOriginKind::Transformed => String::from("transformed"),
            SourceOriginKind::Synthetic => String::from("synthetic"),
            SourceOriginKind::Unknown => String::from("unknown"),
        }
    }
}

impl Default for SourceOrigin {
    fn default() -> Self {
        Self::unknown(OriginId::default())
    }
}

impl fmt::Display for SourceOrigin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.kind, self.id)?;

        if let Some(span) = self.span {
            write!(formatter, " at {}", span)?;
        }

        if let Some(namespace) = &self.namespace {
            write!(formatter, " ({})", namespace)?;
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Provenance chain
// -----------------------------------------------------------------------------

/// A lightweight immutable provenance chain.
///
/// This type is useful when a diagnostic or compiler phase needs to retain the
/// sequence of provenance records without coupling the AST node itself to a
/// mutable provenance registry.
///
/// The chain is represented from the most immediate origin backward:
///
/// ```text
/// [current, parent, grandparent, ...]
/// ```
///
/// The vector is intentionally owned by the caller. No global registry is
/// required.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct SourceOriginChain {
    origins: Vec<SourceOrigin>,
}

impl SourceOriginChain {
    /// Creates an empty provenance chain.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            origins: Vec::new(),
        }
    }

    /// Creates a chain from an existing vector.
    #[must_use]
    pub fn from_vec(origins: Vec<SourceOrigin>) -> Self {
        Self { origins }
    }

    /// Returns the number of provenance records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.origins.len()
    }

    /// Returns whether the chain is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.origins.is_empty()
    }

    /// Returns the records in immediate-to-oldest order.
    #[must_use]
    pub fn as_slice(&self) -> &[SourceOrigin] {
        &self.origins
    }

    /// Returns an iterator over the provenance records.
    pub fn iter(&self) -> std::slice::Iter<'_, SourceOrigin> {
        self.origins.iter()
    }

    /// Appends a provenance record.
    ///
    /// This operation is intentionally explicit. Provenance chain growth is
    /// controlled by the caller/compiler policy rather than hidden global
    /// limits.
    pub fn push(&mut self, origin: SourceOrigin) {
        self.origins.push(origin);
    }

    /// Returns the most recent provenance record.
    #[must_use]
    pub fn current(&self) -> Option<&SourceOrigin> {
        self.origins.first()
    }

    /// Returns the oldest provenance record.
    #[must_use]
    pub fn root(&self) -> Option<&SourceOrigin> {
        self.origins.last()
    }

    /// Consumes the chain and returns its records.
    #[must_use]
    pub fn into_vec(self) -> Vec<SourceOrigin> {
        self.origins
    }
}

impl IntoIterator for SourceOriginChain {
    type Item = SourceOrigin;
    type IntoIter = std::vec::IntoIter<SourceOrigin>;

    fn into_iter(self) -> Self::IntoIter {
        self.origins.into_iter()
    }
}

impl<'a> IntoIterator for &'a SourceOriginChain {
    type Item = &'a SourceOrigin;
    type IntoIter = std::slice::Iter<'a, SourceOrigin>;

    fn into_iter(self) -> Self::IntoIter {
        self.origins.iter()
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn source_id() -> SourceId {
        SourceId::from_raw(7)
    }

    fn span() -> Span {
        Span::new(
            source_id(),
            10_u64.into(),
            20_u64.into(),
        )
        .expect("test span must be valid")
    }

    #[test]
    fn source_origin_preserves_source_provenance() {
        let origin = SourceOrigin::source(
            OriginId::from_raw(1),
            span(),
        );

        assert_eq!(origin.kind(), SourceOriginKind::Source);
        assert_eq!(origin.id(), OriginId::from_raw(1));
        assert_eq!(origin.span(), Some(span()));
        assert!(origin.is_source());
        assert!(!origin.is_derived());
        assert_eq!(origin.source_id(), Some(source_id()));
    }

    #[test]
    fn generated_origin_can_reference_parent() {
        let parent = OriginRef::new(OriginId::from_raw(10));

        let origin = SourceOrigin::generated(
            OriginId::from_raw(11),
            Some(span()),
            Some(parent),
        );

        assert!(origin.is_derived());
        assert_eq!(origin.parent(), Some(parent));
        assert_eq!(origin.span(), Some(span()));
    }

    #[test]
    fn imported_origin_is_namespace_extensible() {
        let namespace = OriginNamespace::new("quantum.example.format");

        let origin = SourceOrigin::imported(
            OriginId::from_raw(2),
            Some(span()),
            Some(namespace.clone()),
            Some(String::from("operation")),
        );

        assert!(origin.is_imported());
        assert_eq!(origin.namespace(), Some(&namespace));
        assert_eq!(origin.name(), Some("operation"));
    }

    #[test]
    fn synthetic_origin_does_not_require_a_span() {
        let origin = SourceOrigin::synthetic(
            OriginId::from_raw(3),
        );

        assert!(origin.span().is_none());
        assert!(origin.source_id().is_none());
        assert!(!origin.is_source());
        assert!(origin.is_derived());
    }

    #[test]
    fn provenance_kind_is_not_a_domain_registry() {
        assert_eq!(
            SourceOriginKind::Source.as_str(),
            "source"
        );
        assert_eq!(
            SourceOriginKind::Imported.as_str(),
            "imported"
        );
    }

    #[test]
    fn origin_ids_are_stable_value_objects() {
        let a = OriginId::from_raw(42);
        let b = OriginId::from_raw(42);

        assert_eq!(a, b);
        assert_eq!(a.as_raw(), 42);
    }

    #[test]
    fn namespace_preserves_external_identity_without_hard_coding_formats() {
        let namespace = OriginNamespace::new(
            "future.quantum.representation.v1",
        );

        assert_eq!(
            namespace.as_str(),
            "future.quantum.representation.v1"
        );
    }

    #[test]
    fn chain_preserves_order() {
        let first = SourceOrigin::source(
            OriginId::from_raw(1),
            span(),
        );

        let second = SourceOrigin::desugared(
            OriginId::from_raw(2),
            Some(span()),
            Some(OriginRef::from(OriginId::from_raw(1))),
        );

        let mut chain = SourceOriginChain::new();
        chain.push(second.clone());
        chain.push(first.clone());

        assert_eq!(chain.len(), 2);
        assert_eq!(chain.current(), Some(&second));
        assert_eq!(chain.root(), Some(&first));
    }

    #[test]
    fn chain_round_trip_shape_is_preserved() {
        let origin = SourceOrigin::source(
            OriginId::from_raw(1),
            span(),
        );

        let chain = SourceOriginChain::from_vec(
            vec![origin.clone()],
        );

        let restored = SourceOriginChain::from_vec(
            chain.as_slice().to_vec(),
        );

        assert_eq!(chain, restored);
    }

    #[test]
    fn schema_version_is_independent() {
        assert_eq!(
            SourceOrigin::schema_version(),
            SOURCE_ORIGIN_SCHEMA_VERSION
        );
    }
}