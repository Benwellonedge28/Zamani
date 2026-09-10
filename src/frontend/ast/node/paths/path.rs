//! Zamani Frontend AST — Canonical Source Path
//!
//! This module defines the canonical, domain-neutral representation of a
//! source-level path used by the native Zamani AST.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer / parser
//!     │
//!     ▼
//! frontend::ast::node::paths::path::Path
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ├── name resolution
//!     ├── symbol resolution
//!     ├── type resolution
//!     ├── capability resolution
//!     └── resource/domain resolution
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! domain / target lowering
//! ```
//!
//! # Responsibility
//!
//! `Path` represents what the programmer wrote as a source-level qualified
//! name/path. It preserves ordered path components without deciding what those
//! components mean.
//!
//! A path may eventually refer to:
//!
//! - a module;
//! - a namespace;
//! - a type;
//! - a function;
//! - a value;
//! - a resource;
//! - an operation;
//! - a capability;
//! - an extension;
//! - another source-level entity.
//!
//! This module does not resolve any of those meanings.
//!
//! # Domain neutrality
//!
//! This type deliberately knows nothing about:
//!
//! - quantum computing;
//! - qubits;
//! - quantum gates;
//! - QPU topology;
//! - quantum vendors;
//! - classical CPUs;
//! - GPUs;
//! - FPGAs;
//! - ASICs;
//! - HDL;
//! - AI accelerators;
//! - routing;
//! - scheduling;
//! - calibration;
//! - error correction;
//! - resilience;
//! - runtime execution;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - ZUIR;
//! - backend identifiers;
//! - operating-system paths;
//! - network URLs.
//!
//! A path such as:
//!
//! ```text
//! quantum::algorithm::QFT
//! ```
//!
//! is only a source-level sequence of names. Semantic analysis determines
//! whether it actually refers to a quantum operation, type, module, or
//! something else.
//!
//! # POCO-REAF
//!
//! A source path must not encode computational scale.
//!
//! Nothing in this representation assumes:
//!
//! - a maximum number of path components;
//! - a maximum identifier length;
//! - a maximum module depth;
//! - a maximum number of namespaces;
//! - a maximum number of computational resources;
//! - a maximum number of qubits;
//! - a particular machine;
//! - a particular backend.
//!
//! Therefore the same representation can be used by a tiny program or a
//! substantially larger program. Practical limits are imposed only by the
//! compiler's explicitly configured resource policies and the resources
//! available to the compilation process.
//!
//! # Source representation
//!
//! The canonical textual separator is `::`.
//!
//! Examples:
//!
//! ```text
//! foo
//! foo::bar
//! std::collections::Map
//! quantum::algorithm::QFT
//! my::module::Type
//! ```
//!
//! The path itself does not assume that every source language context uses the
//! same syntactic root semantics. Root semantics, imports, aliases and name
//! resolution belong to their consuming AST/semantic layer.
//!
//! # Important separation from filesystem paths
//!
//! `Path` is a programming-language path, not an operating-system filesystem
//! path.
//!
//! It must never perform filesystem operations or implicitly interpret:
//!
//! - `/`;
//! - `\\`;
//! - drive letters;
//! - filesystem roots;
//! - URL schemes;
//! - network locations.
//!
//! If the compiler needs a filesystem path, URL, package locator, registry
//! address, or source-unit locator, that belongs to a separate source/package
//! infrastructure type.
//!
//! # Important separation from `TypePath`
//!
//! The repository already contains a type-specific `TypePath` under the type
//! expression subsystem.
//!
//! This module must not become another type system.
//!
//! `Path` is the generic source-level path primitive.
//!
//! A type-specific layer may adapt `Path` into `TypePath` or eventually
//! canonicalize the repository around this representation. Such migration is
//! deliberately downstream from this foundational file.
//!
//! # Semantic boundary
//!
//! ```text
//! Path
//!     = source syntax / unresolved source intent
//!
//! ResolvedPath
//!     = semantic name-resolution result
//!
//! Semantic entity
//!     = resolved meaning
//!
//! ZUIR
//!     = universal computational representation
//! ```
//!
//! `Path` must never contain a resolved symbol ID, resource ID, qubit ID,
//! backend ID, or hardware identity.
//!
//! # Determinism
//!
//! Path components are stored in source order using `Vec`.
//!
//! No hash-map iteration, global state, timestamps, randomness, process IDs,
//! thread IDs, memory addresses, or filesystem discovery participate in path
//! construction.
//!
//! Consequently, constructing the same sequence of source components produces
//! the same path value.
//!
//! # Ownership
//!
//! `Path` owns its ordered components.
//!
//! Each `PathSegment` owns an immutable string representation through
//! `Arc<str>`, avoiding unnecessary repeated copies when path components are
//! cloned while retaining ordinary Rust ownership semantics.
//!
//! # Thread safety
//!
//! The representation contains no mutable global state and uses immutable
//! reference-counted strings. It is therefore suitable for immutable sharing
//! between compiler phases when used with thread-safe surrounding structures.
//!
//! # Serialization
//!
//! Serialization is intentionally provided through Serde because Serde is
//! already an existing repository dependency.
//!
//! The path schema version is local to this representation and must remain
//! distinct from:
//!
//! - Zamani language version;
//! - compiler version;
//! - overall AST schema version;
//! - semantic-model version;
//! - ZUIR version;
//! - package version.
//!
//! # Error model
//!
//! Constructors preserve parser information and therefore permit structurally
//! empty paths. Complete AST validation is responsible for rejecting an empty
//! path where the surrounding grammar requires a non-empty path.
//!
//! This allows parser recovery to retain malformed source without requiring
//! low-level constructors to invent diagnostics or panic.
//!
//! # Security
//!
//! This module:
//!
//! - performs no I/O;
//! - performs no filesystem access;
//! - performs no network access;
//! - performs no execution;
//! - performs no pointer arithmetic;
//! - contains no `unsafe` code;
//! - performs checked length/index operations;
//! - does not interpret path components as filesystem locations.
//!
//! Untrusted source can therefore be represented without executing or resolving
//! it.
//!
//! # Rust compatibility
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
//! # Integration contract
//!
//! This file is foundational and intentionally has no dependency on later AST
//! modules.
//!
//! ```text
//! path.rs
//!    │
//!    ├──► paths/mod.rs
//!    │
//!    ├──► parser
//!    │
//!    ├──► declarations
//!    │
//!    ├──► expressions
//!    │
//!    ├──► imports
//!    │
//!    ├──► type/path adapters
//!    │
//!    ├──► structural validation
//!    │
//!    ├──► visitors / traversal
//!    │
//!    ├──► serialization
//!    │
//!    └──► semantic analysis
//!              │
//!              ▼
//!        name resolution
//!              │
//!              ▼
//!        semantic model
//!              │
//!              ▼
//!             ZUIR
//! ```
//!
//! It must never depend in the reverse direction on:
//!
//! - semantic analysis;
//! - symbol tables;
//! - ZUIR;
//! - quantum IR;
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware;
//! - runtime;
//! - backend implementations.
//!
//! # File contract
//!
//! ## Inputs
//!
//! - source-level path components;
//! - parser-produced identifiers;
//! - programmatically constructed source names;
//! - generated source-level names.
//!
//! ## Outputs
//!
//! - `Path`;
//! - `PathSegment`;
//! - deterministic path inspection;
//! - source rendering;
//! - structural predicates;
//! - checked component access.
//!
//! ## This file does not perform
//!
//! - name resolution;
//! - import resolution;
//! - type resolution;
//! - overload resolution;
//! - symbol lookup;
//! - capability lookup;
//! - resource allocation;
//! - quantum mapping;
//! - hardware mapping;
//! - routing;
//! - scheduling;
//! - backend selection.
//!
//! # No hidden scalability limit
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_PATH_SEGMENTS
//! MAX_PATH_LENGTH
//! MAX_IDENTIFIER_LENGTH
//! ```
//!
//! in this file.
//!
//! The `Vec` and `Arc<str>` representations grow according to available
//! resources. Any operational limits must be supplied by an explicit compiler
//! policy rather than encoded into source-language semantics.
//!
//! # Extension policy
//!
//! New computational domains do not require modifying this file.
//!
//! A path can name an entity in any namespace because this representation is
//! intentionally unaware of the entity's domain.
//!
//! For example, all of the following are ordinary paths at this layer:
//!
//! ```text
//! classical::math::sqrt
//! quantum::algorithms::qft
//! hdl::module::alu
//! ai::model::transformer
//! future::domain::operation
//! ```
//!
//! Whether those names are legal or meaningful is determined later.
//!
//! # Implementation
//!
//! This file contains the complete foundational path contract. The containing
//! `paths/mod.rs` should re-export the public API without duplicating the data
//! model.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::borrow::Borrow;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

/// Schema version for the canonical source-level path representation.
///
/// This is intentionally independent of the overall AST schema and Zamani
/// language version.
pub const PATH_SCHEMA_VERSION: u16 = 1;

/// Canonical separator used when rendering a Zamani source path.
pub const PATH_SEPARATOR: &str = "::";

/// An immutable source-level path component.
///
/// `PathSegment` contains only source spelling. It does not contain a resolved
/// symbol, semantic type, domain identity, resource identity, or backend data.
///
/// Empty segments are permitted during low-level construction so parser error
/// recovery can preserve malformed source. Complete AST validation should
/// reject empty segments where the grammar requires valid identifiers.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PathSegment(Arc<str>);

impl PathSegment {
    /// Creates a path segment from an owned string.
    ///
    /// Empty strings are preserved intentionally for parser recovery.
    #[must_use]
    pub fn new<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        Self(Arc::<str>::from(value.into()))
    }

    /// Creates a path segment directly from a string slice.
    #[must_use]
    pub fn from_str(value: &str) -> Self {
        Self(Arc::<str>::from(value))
    }

    /// Returns the source spelling of this segment.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    /// Returns the UTF-8 byte length of this segment.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the segment is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Consumes the segment and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0.to_string()
    }
}

impl Default for PathSegment {
    fn default() -> Self {
        Self::from_str("")
    }
}

impl AsRef<str> for PathSegment {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for PathSegment {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for PathSegment {
    fn from(value: &str) -> Self {
        Self::from_str(value)
    }
}

impl From<String> for PathSegment {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for PathSegment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A canonical source-level qualified path.
///
/// `Path` is an ordered sequence of source-level [`PathSegment`] values.
///
/// The type intentionally does not distinguish semantic categories. The same
/// representation may be used for module paths, names, type paths, operation
/// names, resource names, capabilities, extensions, and future language
/// constructs.
///
/// # Examples
///
/// ```text
/// foo
/// foo::bar
/// std::collections::Map
/// quantum::algorithm::QFT
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Path {
    segments: Vec<PathSegment>,
}

impl Path {
    /// Creates a path from an ordered collection of source segments.
    ///
    /// The input order is preserved exactly.
    #[must_use]
    pub fn new(segments: Vec<PathSegment>) -> Self {
        Self { segments }
    }

    /// Creates an empty path.
    ///
    /// Empty paths are useful for parser recovery and intermediate
    /// construction. Structural validation must decide whether an empty path
    /// is legal in the surrounding grammar.
    #[must_use]
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    /// Creates a path containing exactly one segment.
    #[must_use]
    pub fn single<S>(segment: S) -> Self
    where
        S: Into<PathSegment>,
    {
        Self::new(vec![segment.into()])
    }

    /// Creates a path from any ordered iterable of path-like values.
    ///
    /// No machine-size or component-count assumption is introduced.
    #[must_use]
    pub fn from_segments<I, S>(segments: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<PathSegment>,
    {
        Self {
            segments: segments.into_iter().map(Into::into).collect(),
        }
    }

    /// Returns all path segments in source order.
    #[must_use]
    pub fn segments(&self) -> &[PathSegment] {
        &self.segments
    }

    /// Returns the number of path segments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Returns whether the path contains no segments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Returns the first path segment.
    #[must_use]
    pub fn first(&self) -> Option<&PathSegment> {
        self.segments.first()
    }

    /// Returns the final path segment.
    #[must_use]
    pub fn last(&self) -> Option<&PathSegment> {
        self.segments.last()
    }

    /// Returns a path segment by zero-based source-order index.
    ///
    /// This operation is checked and returns `None` for an out-of-range index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&PathSegment> {
        self.segments.get(index)
    }

    /// Returns an iterator over path segments in source order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &PathSegment> + DoubleEndedIterator {
        self.segments.iter()
    }

    /// Returns whether every segment has non-empty source spelling.
    ///
    /// This is a structural predicate only. It does not determine whether the
    /// individual names are valid identifiers according to the language grammar.
    #[must_use]
    pub fn has_no_empty_segments(&self) -> bool {
        self.segments.iter().all(|segment| !segment.is_empty())
    }

    /// Returns whether the path has exactly one segment.
    #[must_use]
    pub fn is_single(&self) -> bool {
        self.segments.len() == 1
    }

    /// Returns the source-level textual representation using `::`.
    ///
    /// This method performs no semantic resolution.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        self.segments
            .iter()
            .map(PathSegment::as_str)
            .collect::<Vec<_>>()
            .join(PATH_SEPARATOR)
    }

    /// Appends one source-level segment.
    ///
    /// This mutates only this path value. It does not modify any global
    /// compiler state.
    pub fn push<S>(&mut self, segment: S)
    where
        S: Into<PathSegment>,
    {
        self.segments.push(segment.into());
    }

    /// Appends all segments from another path while preserving source order.
    ///
    /// This is useful for parser and AST builders constructing qualified names
    /// incrementally.
    pub fn extend(&mut self, other: &Path) {
        self.segments.extend(other.segments.iter().cloned());
    }

    /// Removes and returns the final segment, if present.
    ///
    /// This is useful for source-level transformations such as separating a
    /// namespace prefix from its terminal name. Semantic interpretation still
    /// belongs downstream.
    pub fn pop(&mut self) -> Option<PathSegment> {
        self.segments.pop()
    }

    /// Returns a borrowed prefix containing the first `count` segments.
    ///
    /// The returned slice is source ordered and does not allocate.
    #[must_use]
    pub fn prefix(&self, count: usize) -> Option<&[PathSegment]> {
        if count <= self.segments.len() {
            Some(&self.segments[..count])
        } else {
            None
        }
    }

    /// Returns a borrowed suffix containing the final `count` segments.
    ///
    /// The returned slice is source ordered and does not allocate.
    #[must_use]
    pub fn suffix(&self, count: usize) -> Option<&[PathSegment]> {
        if count <= self.segments.len() {
            Some(&self.segments[self.segments.len() - count..])
        } else {
            None
        }
    }

    /// Creates a new path from the first `count` segments.
    ///
    /// Returns `None` if `count` exceeds the path length.
    #[must_use]
    pub fn to_prefix(&self, count: usize) -> Option<Self> {
        self.prefix(count)
            .map(|segments| Self::new(segments.to_vec()))
    }

    /// Creates a new path from the final `count` segments.
    ///
    /// Returns `None` if `count` exceeds the path length.
    #[must_use]
    pub fn to_suffix(&self, count: usize) -> Option<Self> {
        self.suffix(count)
            .map(|segments| Self::new(segments.to_vec()))
    }

    /// Returns a path containing a clone of this path's segments followed by
    /// one additional segment.
    ///
    /// The original path remains unchanged.
    #[must_use]
    pub fn joined<S>(&self, segment: S) -> Self
    where
        S: Into<PathSegment>,
    {
        let mut result = self.clone();
        result.push(segment);
        result
    }

    /// Returns whether this path starts with the supplied path.
    ///
    /// Empty `prefix` is considered a prefix of every path.
    #[must_use]
    pub fn starts_with(&self, prefix: &Path) -> bool {
        self.segments.len() >= prefix.segments.len()
            && self
                .segments
                .iter()
                .zip(prefix.segments.iter())
                .all(|(left, right)| left == right)
    }

    /// Returns whether this path ends with the supplied path.
    ///
    /// Empty `suffix` is considered a suffix of every path.
    #[must_use]
    pub fn ends_with(&self, suffix: &Path) -> bool {
        self.segments.len() >= suffix.segments.len()
            && self.segments[self.segments.len() - suffix.segments.len()..]
                .iter()
                .zip(suffix.segments.iter())
                .all(|(left, right)| left == right)
    }

    /// Returns the first position at which this path differs from another
    /// path, using source-order segment comparison.
    ///
    /// If one path is a prefix of the other, the returned index is the length
    /// of the shorter path.
    #[must_use]
    pub fn first_difference(&self, other: &Path) -> usize {
        let common = self.segments.len().min(other.segments.len());

        for index in 0..common {
            if self.segments[index] != other.segments[index] {
                return index;
            }
        }

        common
    }

    /// Returns a structural error when the path is empty or contains an empty
    /// component.
    ///
    /// This deliberately does not validate identifier syntax because identifier
    /// grammar belongs to the parser/validation layer.
    pub fn validate_structure(&self) -> Result<(), PathStructureError> {
        if self.is_empty() {
            return Err(PathStructureError::EmptyPath);
        }

        for (index, segment) in self.segments.iter().enumerate() {
            if segment.is_empty() {
                return Err(PathStructureError::EmptySegment { index });
            }
        }

        Ok(())
    }

    /// Returns an owned vector of source segment spellings.
    ///
    /// This is useful at integration boundaries that need ordinary `String`
    /// values. It performs no semantic resolution.
    #[must_use]
    pub fn to_strings(&self) -> Vec<String> {
        self.segments
            .iter()
            .map(PathSegment::into_string)
            .collect()
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<PathSegment> for Path {
    fn from(value: PathSegment) -> Self {
        Self::single(value)
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        Self::single(value)
    }
}

impl From<String> for Path {
    fn from(value: String) -> Self {
        Self::single(value)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, segment) in self.segments.iter().enumerate() {
            if index != 0 {
                formatter.write_str(PATH_SEPARATOR)?;
            }

            formatter.write_str(segment.as_str())?;
        }

        Ok(())
    }
}

/// Structural validation errors for a source-level [`Path`].
///
/// These errors are intentionally limited to properties owned by this file.
/// Identifier grammar, name resolution, visibility, imports, overloads, type
/// correctness and domain legality belong to later layers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathStructureError {
    /// The path contains no segments.
    EmptyPath,

    /// The path contains an empty segment at the given zero-based index.
    EmptySegment {
        /// Position of the invalid segment in source order.
        index: usize,
    },
}

impl fmt::Display for PathStructureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPath => {
                formatter.write_str("source path must contain at least one segment")
            }
            Self::EmptySegment { index } => {
                write!(formatter, "source path contains an empty segment at index {index}")
            }
        }
    }
}

impl std::error::Error for PathStructureError {}

/// Returns the schema version for the canonical source-level path model.
#[must_use]
pub const fn schema_version() -> u16 {
    PATH_SCHEMA_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_version_is_nonzero() {
        assert!(schema_version() > 0);
    }

    #[test]
    fn segment_preserves_source_spelling() {
        let segment = PathSegment::from_str("Quantum");

        assert_eq!(segment.as_str(), "Quantum");
        assert_eq!(segment.len(), 7);
        assert!(!segment.is_empty());
    }

    #[test]
    fn segment_supports_unicode() {
        let segment = PathSegment::from_str("量子");

        assert_eq!(segment.as_str(), "量子");
        assert!(!segment.is_empty());
    }

    #[test]
    fn single_path_is_constructed_without_resolution() {
        let path = Path::single("Type");

        assert_eq!(path.len(), 1);
        assert_eq!(path.first().map(PathSegment::as_str), Some("Type"));
        assert_eq!(path.to_source_string(), "Type");
    }

    #[test]
    fn qualified_path_preserves_source_order() {
        let path = Path::from_segments(["std", "collections", "Map"]);

        assert_eq!(path.len(), 3);
        assert_eq!(path.get(0).map(PathSegment::as_str), Some("std"));
        assert_eq!(path.get(1).map(PathSegment::as_str), Some("collections"));
        assert_eq!(path.get(2).map(PathSegment::as_str), Some("Map"));
        assert_eq!(path.to_source_string(), "std::collections::Map");
    }

    #[test]
    fn path_iteration_is_source_ordered() {
        let path = Path::from_segments(["a", "b", "c"]);

        let names: Vec<&str> = path.iter().map(PathSegment::as_str).collect();

        assert_eq!(names, vec!["a", "b", "c"]);
    }

    #[test]
    fn empty_path_is_structurally_detectable() {
        let path = Path::empty();

        assert!(path.is_empty());
        assert_eq!(
            path.validate_structure(),
            Err(PathStructureError::EmptyPath)
        );
    }

    #[test]
    fn empty_segments_are_structurally_detectable() {
        let path = Path::from_segments(["a", "", "c"]);

        assert_eq!(
            path.validate_structure(),
            Err(PathStructureError::EmptySegment { index: 1 })
        );
    }

    #[test]
    fn nonempty_path_with_nonempty_segments_is_valid() {
        let path = Path::from_segments(["quantum", "algorithm", "QFT"]);

        assert_eq!(path.validate_structure(), Ok(()));
        assert!(path.has_no_empty_segments());
    }

    #[test]
    fn push_preserves_existing_order() {
        let mut path = Path::single("a");

        path.push("b");
        path.push("c");

        assert_eq!(path.to_source_string(), "a::b::c");
    }

    #[test]
    fn extend_preserves_both_sequences() {
        let mut left = Path::from_segments(["a", "b"]);
        let right = Path::from_segments(["c", "d"]);

        left.extend(&right);

        assert_eq!(left.to_source_string(), "a::b::c::d");
        assert_eq!(right.to_source_string(), "c::d");
    }

    #[test]
    fn pop_removes_only_the_final_segment() {
        let mut path = Path::from_segments(["a", "b", "c"]);

        assert_eq!(path.pop().map(PathSegment::as_str), Some("c"));
        assert_eq!(path.to_source_string(), "a::b");
    }

    #[test]
    fn prefix_and_suffix_are_checked() {
        let path = Path::from_segments(["a", "b", "c", "d"]);

        assert_eq!(
            path.prefix(2)
                .expect("valid prefix")
                .iter()
                .map(PathSegment::as_str)
                .collect::<Vec<_>>(),
            vec!["a", "b"]
        );

        assert_eq!(
            path.suffix(2)
                .expect("valid suffix")
                .iter()
                .map(PathSegment::as_str)
                .collect::<Vec<_>>(),
            vec!["c", "d"]
        );

        assert!(path.prefix(5).is_none());
        assert!(path.suffix(5).is_none());
    }

    #[test]
    fn zero_length_prefix_and_suffix_are_supported() {
        let path = Path::from_segments(["a", "b"]);

        assert_eq!(path.prefix(0).expect("empty prefix").len(), 0);
        assert_eq!(path.suffix(0).expect("empty suffix").len(), 0);
    }

    #[test]
    fn prefix_and_suffix_paths_preserve_order() {
        let path = Path::from_segments(["a", "b", "c"]);

        assert_eq!(
            path.to_prefix(2)
                .expect("prefix")
                .to_source_string(),
            "a::b"
        );

        assert_eq!(
            path.to_suffix(2)
                .expect("suffix")
                .to_source_string(),
            "b::c"
        );
    }

    #[test]
    fn joined_does_not_mutate_original() {
        let original = Path::from_segments(["a", "b"]);
        let joined = original.joined("c");

        assert_eq!(original.to_source_string(), "a::b");
        assert_eq!(joined.to_source_string(), "a::b::c");
    }

    #[test]
    fn starts_with_and_ends_with_are_segment_based() {
        let path = Path::from_segments(["a", "b", "c"]);

        assert!(path.starts_with(&Path::from_segments(["a"])));
        assert!(path.starts_with(&Path::from_segments(["a", "b"])));
        assert!(path.ends_with(&Path::from_segments(["c"])));
        assert!(path.ends_with(&Path::from_segments(["b", "c"])));

        assert!(!path.starts_with(&Path::from_segments(["b"])));
        assert!(!path.ends_with(&Path::from_segments(["b"])));
    }

    #[test]
    fn empty_path_is_prefix_and_suffix() {
        let path = Path::from_segments(["a", "b"]);
        let empty = Path::empty();

        assert!(path.starts_with(&empty));
        assert!(path.ends_with(&empty));
    }

    #[test]
    fn first_difference_is_deterministic() {
        let left = Path::from_segments(["a", "b", "c"]);
        let same = Path::from_segments(["a", "b", "c"]);
        let different = Path::from_segments(["a", "x", "c"]);
        let prefix = Path::from_segments(["a", "b"]);

        assert_eq!(left.first_difference(&same), 3);
        assert_eq!(left.first_difference(&different), 1);
        assert_eq!(left.first_difference(&prefix), 2);
    }

    #[test]
    fn path_display_uses_canonical_separator() {
        let path = Path::from_segments(["my", "module", "Type"]);

        assert_eq!(path.to_string(), "my::module::Type");
    }

    #[test]
    fn source_path_is_not_a_filesystem_path() {
        let path = Path::from_segments(["module", "Type"]);

        assert_eq!(path.to_source_string(), "module::Type");

        // The source path representation performs no filesystem interpretation.
        assert_ne!(path.to_source_string(), "module/Type");
    }

    #[test]
    fn large_component_count_has_no_ast_defined_limit() {
        let components: Vec<String> = (0..10_000)
            .map(|index| format!("component_{index}"))
            .collect();

        let path = Path::from_segments(components);

        assert_eq!(path.len(), 10_000);
        assert!(path.has_no_empty_segments());
    }

    #[test]
    fn long_component_is_preserved() {
        let component = "x".repeat(100_000);
        let path = Path::single(component.clone());

        assert_eq!(path.len(), 1);
        assert_eq!(path.first().expect("segment").len(), 100_000);
        assert_eq!(path.first().expect("segment").as_str(), component);
    }

    #[test]
    fn serialization_round_trip_preserves_path() {
        let path = Path::from_segments([
            "quantum",
            "algorithm",
            "phase_estimation",
        ]);

        let encoded = serde_json::to_string(&path).expect("serialize");
        let decoded: Path = serde_json::from_str(&encoded).expect("deserialize");

        assert_eq!(decoded, path);
        assert_eq!(
            decoded.to_source_string(),
            "quantum::algorithm::phase_estimation"
        );
    }

    #[test]
    fn cloned_segments_remain_equal() {
        let original = PathSegment::from_str("module");
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.as_str(), cloned.as_str());
    }

    #[test]
    fn strings_can_be_exported_without_semantic_resolution() {
        let path = Path::from_segments(["a", "b", "c"]);

        assert_eq!(
            path.to_strings(),
            vec![
                "a".to_owned(),
                "b".to_owned(),
                "c".to_owned()
            ]
        );
    }

    #[test]
    fn schema_is_independent_of_language_version() {
        // This value is deliberately local to this source representation.
        // Language/compiler versioning belongs to higher-level infrastructure.
        assert_eq!(PATH_SCHEMA_VERSION, 1);
    }
}