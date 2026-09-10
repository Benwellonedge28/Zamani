//! # Zamani Native AST — Path Segment
//!
//! Canonical source-level representation of one segment of a Zamani path.
//!
//! ## Architectural position
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
//! frontend::ast::node::paths
//!     │
//!     ├── Path
//!     │    └── PathSegment  ← this module
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ├── name resolution
//!     ├── module resolution
//!     ├── import resolution
//!     ├── namespace resolution
//!     └── external-reference resolution
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! ## Purpose
//!
//! `PathSegment` represents exactly one source-level component of a qualified
//! path.
//!
//! Examples:
//!
//! ```text
//! math
//! linear
//! transform
//! ```
//!
//! together form:
//!
//! ```text
//! math::linear::transform
//! ```
//!
//! A path segment is deliberately a **component**, not an independently
//! addressable AST node. The enclosing `Path` owns the AST-level identity.
//!
//! This distinction is important because the current Zamani `CoreNodeKind`
//! classification does not define a dedicated `PathSegment` node kind.
//! Creating a synthetic node identity merely for a path component would make
//! the AST hierarchy less coherent and would require unrelated changes to the
//! canonical node-kind system.
//!
//! ## Ownership
//!
//! This file owns:
//!
//! - one path segment's source spelling;
//! - one path segment's source span;
//! - local structural validation;
//! - deterministic equality and hashing;
//! - source-level formatting;
//! - cheap read-only accessors;
//! - construction from parser/source data.
//!
//! This file does NOT own:
//!
//! - complete paths;
//! - imports;
//! - aliases;
//! - namespaces;
//! - symbol resolution;
//! - module resolution;
//! - generic argument resolution;
//! - type resolution;
//! - semantic identities;
//! - resource identities;
//! - quantum identities;
//! - physical qubit identifiers;
//! - hardware mappings;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - backend selection;
//! - execution.
//!
//! Those responsibilities belong to their respective AST or downstream
//! compilation layers.
//!
//! ## POCO-REAF
//!
//! A path segment contains no information about computational scale.
//!
//! It does not encode:
//!
//! - machine size;
//! - CPU count;
//! - GPU count;
//! - FPGA size;
//! - QPU size;
//! - qubit count;
//! - register width;
//! - topology;
//! - vendor;
//! - backend;
//! - instruction set;
//! - gate set;
//! - quantum technology.
//!
//! Therefore the same source-level segment representation can participate in
//! programs ranging from tiny programs to programs whose eventual realization
//! uses arbitrarily large resources supported by the compiler and target.
//!
//! "Infinity" here means that this type introduces no artificial semantic
//! machine-size bound. Actual execution remains bounded by available memory,
//! compiler resources, target capabilities and representable values.
//!
//! ## Source/semantic boundary
//!
//! Consider:
//!
//! ```text
//! quantum::State
//! ```
//!
//! This module represents:
//!
//! ```text
//! quantum
//! State
//! ```
//!
//! It does NOT determine that:
//!
//! - `quantum` is a quantum namespace;
//! - `State` is a quantum state;
//! - either name exists;
//! - either name is imported;
//! - either name denotes a qubit;
//! - either name denotes a physical resource.
//!
//! Such interpretation belongs to semantic analysis.
//!
//! ## Identifier validation boundary
//!
//! The lexer/parser owns the complete Zamani lexical grammar.
//!
//! `PathSegment` intentionally does not duplicate that grammar.
//!
//! It performs only the structural invariant that a path segment cannot have
//! an empty source spelling.
//!
//! This permits future lexical evolution without requiring this foundational
//! AST component to be rewritten.
//!
//! Unicode is preserved because the source spelling is stored as UTF-8.
//!
//! ## Why this is not `TypeName`
//!
//! The current repository already contains `types::type_expr::TypeName` and
//! `TypePath`. Those are currently used by the type system, including
//! `NamedType`. They are type-oriented abstractions.
//!
//! A general path segment must remain usable by:
//!
//! - expressions;
//! - declarations;
//! - imports;
//! - namespaces;
//! - annotations;
//! - capabilities;
//! - resources;
//! - domains;
//! - external references;
//! - types;
//! - future language constructs.
//!
//! Therefore this module deliberately does not depend on `TypeName`.
//!
//! The eventual canonical path migration can make `TypePath` consume
//! `PathSegment` rather than maintain a competing representation.
//!
//! ## Dependency contract
//!
//! This module may depend on:
//!
//! - `frontend::ast::node::source::Span`;
//! - Rust standard-library facilities;
//! - Serde, using the dependency already present in the repository.
//!
//! It must NOT depend on:
//!
//! - semantic analysis;
//! - compiler orchestration;
//! - ZUIR;
//! - quantum IR;
//! - QEC;
//! - routing;
//! - scheduling;
//! - calibration;
//! - runtime;
//! - hardware;
//! - backend providers;
//! - LLVM;
//! - QIR implementation types;
//! - MLIR implementation types;
//! - vendor SDKs;
//! - filesystem state;
//! - global mutable state.
//!
//! ## Integration contract
//!
//! ```text
//! source/span.rs
//!       │
//!       ▼
//! PathSegment
//!       │
//!       ▼
//! paths/path.rs
//!       │
//!       ├── paths/import.rs
//!       ├── expressions
//!       ├── declarations
//!       ├── types
//!       └── annotations
//!       │
//!       ▼
//! structural validation
//!       │
//!       ▼
//! semantic name/module resolution
//!       │
//!       ▼
//! semantic model
//!       │
//!       ▼
//! ZUIR
//! ```
//!
//! `PathSegment` must remain below all semantic and target-specific layers.
//!
//! ## Determinism
//!
//! Equality, ordering, hashing and serialization depend only on:
//!
//! - segment spelling;
//! - source span.
//!
//! No:
//!
//! - memory address;
//! - timestamp;
//! - randomness;
//! - thread identity;
//! - process identity;
//! - backend state
//!
//! is used.
//!
//! ## Serialization
//!
//! Serde serialization preserves:
//!
//! - source spelling;
//! - source span.
//!
//! AST-wide serialization versioning belongs to the AST serialization layer.
//! This file intentionally does not introduce a competing global serialization
//! version.
//!
//! ## Security
//!
//! This type performs no I/O and no code execution.
//!
//! It contains no `unsafe` code.
//!
//! An empty spelling is represented safely and rejected by structural
//! validation.
//!
//! The type deliberately does not impose a hard-coded identifier-length
//! limit. If hostile-input limits are required, those belong to configurable
//! compiler policy rather than language semantics.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! ## Important integration note
//!
//! `PathSegment` is intentionally not a `Node`.
//!
//! The enclosing `Path` should own the `NodeId`, `NodeKind`, complete source
//! span and metadata for the path expression/declaration construct.
//!
//! This prevents every `::`-separated identifier from becoming an independent
//! semantic AST node while still preserving precise source locations for
//! diagnostics.
//!
//! Generic arguments, if the Zamani grammar associates them with individual
//! path segments in the future, should be represented by the canonical path
//! layer rather than added here. Keeping that responsibility in `Path` or a
//! dedicated path-argument abstraction avoids coupling this foundational
//! component to the type-expression graph and prevents cyclic dependencies.
//!
//! This file is therefore complete for the current repository contract and
//! does not need to be reopened merely because path resolution, generic
//! handling, quantum extensions or ZUIR changes later.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::source::Span;

/// Local schema version for [`PathSegment`].
///
/// This is deliberately independent of:
///
/// - Zamani language version;
/// - AST serialization version;
/// - compiler version;
/// - semantic-model version;
/// - ZUIR version.
pub const PATH_SEGMENT_SCHEMA_VERSION: u16 = 1;

/// A single source-level component of a Zamani path.
///
/// # Representation
///
/// A segment consists of:
///
/// - its source spelling;
/// - its source span.
///
/// Semantic meaning is intentionally absent.
///
/// # Examples
///
/// ```text
/// foo
/// foo::bar
/// foo::bar::baz
/// ```
///
/// In `foo::bar::baz`, the individual segment values are:
///
/// ```text
/// foo
/// bar
/// baz
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PathSegment {
    /// Source spelling of the segment.
    ///
    /// The spelling is stored exactly as supplied to the AST layer.
    ///
    /// Lexical validity belongs to the lexer/parser.
    name: String,

    /// Exact source range occupied by this segment.
    span: Span,
}

impl PathSegment {
    /// Creates a path segment from source spelling and source span.
    ///
    /// This constructor does not perform lexical validation.
    ///
    /// An empty spelling is rejected because an empty path component is not a
    /// structurally valid path segment.
    ///
    /// # Errors
    ///
    /// Returns [`PathSegmentError::EmptyName`] when `name` is empty.
    ///
    /// # Determinism
    ///
    /// Construction depends only on the supplied arguments.
    pub fn new<N>(name: N, span: Span) -> Result<Self, PathSegmentError>
    where
        N: Into<String>,
    {
        let name = name.into();

        if name.is_empty() {
            return Err(PathSegmentError::EmptyName);
        }

        Ok(Self { name, span })
    }

    /// Creates a segment from an already validated source spelling.
    ///
    /// This is intentionally separate from [`Self::new`] so parser/builders
    /// can make the validation boundary explicit.
    ///
    /// # Panics
    ///
    /// This method does not panic. Invalid input is returned as an error.
    pub fn try_from_name<N>(
        name: N,
        span: Span,
    ) -> Result<Self, PathSegmentError>
    where
        N: Into<String>,
    {
        Self::new(name, span)
    }

    /// Returns the source spelling.
    #[must_use]
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the source spelling as bytes.
    ///
    /// The returned bytes are the UTF-8 representation of [`Self::name`].
    #[must_use]
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.name.as_bytes()
    }

    /// Returns the source span.
    #[must_use]
    #[inline]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Returns the byte length of the source spelling.
    ///
    /// This is a storage/source-property query, not a semantic identifier
    /// limit.
    #[must_use]
    #[inline]
    pub fn byte_len(&self) -> usize {
        self.name.len()
    }

    /// Returns the Unicode scalar-value count of the source spelling.
    ///
    /// This is intentionally derived on demand rather than stored separately,
    /// preventing duplicated state.
    #[must_use]
    #[inline]
    pub fn char_len(&self) -> usize {
        self.name.chars().count()
    }

    /// Returns whether the source spelling is empty.
    ///
    /// A normally constructed `PathSegment` is never empty. This method is
    /// still provided because it is useful to generic structural validators
    /// and makes the invariant directly observable.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }

    /// Returns whether the segment has a non-empty source spelling.
    #[must_use]
    #[inline]
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
    }

    /// Performs local structural validation.
    ///
    /// This intentionally does not validate:
    ///
    /// - lexical identifier grammar;
    /// - symbol existence;
    /// - namespace existence;
    /// - imports;
    /// - aliases;
    /// - types;
    /// - capabilities;
    /// - resources;
    /// - quantum semantics;
    /// - target support.
    ///
    /// Those checks belong to the appropriate later phase.
    pub fn validate_structure(&self) -> Result<(), PathSegmentError> {
        if self.name.is_empty() {
            return Err(PathSegmentError::EmptyName);
        }

        Ok(())
    }

    /// Returns the local schema version.
    #[must_use]
    #[inline]
    pub const fn schema_version() -> u16 {
        PATH_SEGMENT_SCHEMA_VERSION
    }

    /// Creates a new segment with the same source spelling and a different
    /// source span.
    ///
    /// This is useful for parser recovery or source-map transformations while
    /// preserving the source-level identity of the segment value.
    ///
    /// No semantic information is introduced.
    #[must_use]
    pub fn with_span(self, span: Span) -> Self {
        Self {
            name: self.name,
            span,
        }
    }

    /// Creates a new segment with a different source spelling and the same
    /// source span.
    ///
    /// This method validates the replacement spelling and therefore cannot
    /// create an invalid segment.
    pub fn with_name<N>(
        self,
        name: N,
    ) -> Result<Self, PathSegmentError>
    where
        N: Into<String>,
    {
        Self::new(name, self.span)
    }

    /// Consumes the segment and returns its source spelling.
    #[must_use]
    #[inline]
    pub fn into_name(self) -> String {
        self.name
    }

    /// Consumes the segment and returns `(name, span)`.
    #[must_use]
    #[inline]
    pub fn into_parts(self) -> (String, Span) {
        (self.name, self.span)
    }

    /// Returns whether this segment's source spelling equals `other`.
    ///
    /// This is a source-level comparison only.
    ///
    /// It does not perform:
    ///
    /// - Unicode normalization;
    /// - case folding;
    /// - symbol resolution;
    /// - namespace resolution.
    #[must_use]
    #[inline]
    pub fn name_is(&self, other: &str) -> bool {
        self.name == other
    }

    /// Returns a borrowed view of the source spelling.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

impl AsRef<str> for PathSegment {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl fmt::Display for PathSegment {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)
    }
}

/// Errors produced by local [`PathSegment`] construction or validation.
///
/// This error type intentionally contains no semantic or backend-specific
/// variants.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PathSegmentError {
    /// The segment spelling was empty.
    EmptyName,
}

impl fmt::Display for PathSegmentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => {
                formatter.write_str("path segment name cannot be empty")
            }
        }
    }
}

impl std::error::Error for PathSegmentError {}

/// Converts an owned source string into a validated path segment.
///
/// This implementation cannot infer a source span, so it is intentionally
/// not provided through `From<String>`. Callers must explicitly provide the
/// span through [`PathSegment::new`].
#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::source::{
        SourceId,
        SourceOffset,
    };

    fn span() -> Span {
        Span::new(
            SourceId::from_raw(0),
            SourceOffset::from_raw(0),
            SourceOffset::from_raw(3),
        )
        .expect("test span must be valid")
    }

    #[test]
    fn constructs_segment() {
        let segment =
            PathSegment::new("quantum", span())
                .expect("non-empty segment must construct");

        assert_eq!(segment.name(), "quantum");
        assert_eq!(segment.as_str(), "quantum");
        assert_eq!(segment.byte_len(), 7);
        assert_eq!(segment.char_len(), 7);
        assert!(!segment.is_empty());
        assert!(segment.is_valid());
        assert_eq!(segment.span(), span());
    }

    #[test]
    fn supports_unicode_source_names() {
        let segment =
            PathSegment::new("量子", span())
                .expect("Unicode source spelling must be preserved");

        assert_eq!(segment.name(), "量子");
        assert_eq!(segment.char_len(), 2);
        assert!(segment.byte_len() > segment.char_len());
    }

    #[test]
    fn rejects_empty_segment() {
        let result = PathSegment::new("", span());

        assert_eq!(
            result,
            Err(PathSegmentError::EmptyName)
        );
    }

    #[test]
    fn try_from_name_matches_new() {
        let first =
            PathSegment::new("module", span())
                .expect("segment must construct");

        let second =
            PathSegment::try_from_name("module", span())
                .expect("segment must construct");

        assert_eq!(first, second);
    }

    #[test]
    fn validates_structure() {
        let segment =
            PathSegment::new("module", span())
                .expect("segment must construct");

        assert_eq!(segment.validate_structure(), Ok(()));
    }

    #[test]
    fn display_preserves_source_spelling() {
        let segment =
            PathSegment::new("transform", span())
                .expect("segment must construct");

        assert_eq!(segment.to_string(), "transform");
    }

    #[test]
    fn as_ref_exposes_source_name() {
        let segment =
            PathSegment::new("State", span())
                .expect("segment must construct");

        let value: &str = segment.as_ref();

        assert_eq!(value, "State");
    }

    #[test]
    fn name_comparison_is_exact() {
        let segment =
            PathSegment::new("State", span())
                .expect("segment must construct");

        assert!(segment.name_is("State"));
        assert!(!segment.name_is("state"));
    }

    #[test]
    fn replacement_name_preserves_span() {
        let original =
            PathSegment::new("old", span())
                .expect("segment must construct");

        let replacement = original
            .with_name("new")
            .expect("replacement name must construct");

        assert_eq!(replacement.name(), "new");
        assert_eq!(replacement.span(), span());
    }

    #[test]
    fn replacement_span_preserves_name() {
        let original =
            PathSegment::new("module", span())
                .expect("segment must construct");

        let replacement =
            original.with_span(span());

        assert_eq!(replacement.name(), "module");
    }

    #[test]
    fn into_parts_preserves_data() {
        let original =
            PathSegment::new("module", span())
                .expect("segment must construct");

        let (name, segment_span) = original.into_parts();

        assert_eq!(name, "module");
        assert_eq!(segment_span, span());
    }

    #[test]
    fn into_name_consumes_only_the_spelling() {
        let original =
            PathSegment::new("module", span())
                .expect("segment must construct");

        assert_eq!(original.into_name(), "module");
    }

    #[test]
    fn schema_version_is_explicit() {
        assert_eq!(
            PathSegment::schema_version(),
            PATH_SEGMENT_SCHEMA_VERSION
        );
    }

    #[test]
    fn equality_is_source_structural() {
        let first =
            PathSegment::new("module", span())
                .expect("segment must construct");

        let second =
            PathSegment::new("module", span())
                .expect("segment must construct");

        assert_eq!(first, second);
    }

    #[test]
    fn different_names_are_not_equal() {
        let first =
            PathSegment::new("module", span())
                .expect("segment must construct");

        let second =
            PathSegment::new("other", span())
                .expect("segment must construct");

        assert_ne!(first, second);
    }

    #[test]
    fn different_spans_are_structurally_distinguishable() {
        let first_span = Span::new(
            SourceId::from_raw(0),
            SourceOffset::from_raw(0),
            SourceOffset::from_raw(3),
        )
        .expect("test span must be valid");

        let second_span = Span::new(
            SourceId::from_raw(0),
            SourceOffset::from_raw(4),
            SourceOffset::from_raw(7),
        )
        .expect("test span must be valid");

        let first =
            PathSegment::new("foo", first_span)
                .expect("segment must construct");

        let second =
            PathSegment::new("foo", second_span)
                .expect("segment must construct");

        assert_ne!(first, second);
    }
}