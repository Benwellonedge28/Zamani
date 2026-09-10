//! Zamani Native AST — Import Declarations
//!
//! Canonical source-level representation of Zamani imports.
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
//! Native AST
//!     │
//!     └── Import
//!             │
//!             ▼
//!     structural validation
//!             │
//!             ▼
//!     semantic name/import resolution
//!             │
//!             ▼
//!     semantic model
//!             │
//!             ▼
//!     ZUIR
//!             │
//!             ▼
//!     domain / target lowering
//! ```
//!
//! # Responsibility
//!
//! This module owns the source-level representation of an import declaration.
//!
//! It preserves programmer intent and source structure without resolving the
//! imported entity.
//!
//! This module does NOT:
//!
//! - access the filesystem;
//! - read package manifests;
//! - contact registries;
//! - resolve modules;
//! - resolve symbols;
//! - assign symbol IDs;
//! - perform package dependency resolution;
//! - load external files;
//! - inspect hardware;
//! - select a backend;
//! - perform quantum compilation;
//! - perform routing;
//! - perform scheduling;
//! - perform calibration;
//! - perform QEC;
//! - lower to ZUIR;
//! - execute imported code.
//!
//! Those responsibilities belong to later compiler layers.
//!
//! # POCO-REAF
//!
//! Imports are represented entirely in terms of source-level names and paths.
//!
//! There is no:
//!
//! - fixed module count;
//! - fixed import count;
//! - fixed namespace depth;
//! - fixed package count;
//! - fixed resource count;
//! - fixed machine size;
//! - fixed qubit count;
//! - fixed backend;
//! - fixed vendor;
//! - fixed topology.
//!
//! Collection sizes grow according to available compiler resources.
//!
//! # Grammar boundary
//!
//! The current Zamani grammar provides these forms:
//!
//! ```text
//! import IDENTIFIER;
//! import IDENTIFIER as IDENTIFIER;
//! import { IDENTIFIER (, IDENTIFIER)* } from STRING;
//! import * as IDENTIFIER from STRING;
//! ```
//!
//! The AST is intentionally slightly more general than the present grammar:
//! paths are represented by the canonical `Path` type rather than by a single
//! `String`. This allows qualified paths to be introduced by the grammar later
//! without redesigning this AST node.
//!
//! # Important architectural distinction
//!
//! ```text
//! Import
//!     = source-level import syntax
//!
//! ImportPath / Path
//!     = source-level name/path structure
//!
//! Semantic import
//!     = resolved module/package/symbol meaning
//!
//! ZUIR
//!     = universal computational representation
//! ```
//!
//! `Import` must never contain resolved symbols.
//!
//! Do not add fields such as:
//!
//! ```text
//! resolved_module
//! resolved_symbol
//! package_id
//! file_path
//! filesystem_path
//! registry_id
//! symbol_id
//! backend_id
//! ```
//!
//! Those belong to semantic/compiler side tables.
//!
//! # Domain neutrality
//!
//! An import can refer to:
//!
//! - classical code;
//! - quantum code;
//! - hybrid code;
//! - HDL extensions;
//! - AI libraries;
//! - accelerator libraries;
//! - distributed modules;
//! - user libraries;
//! - standard libraries;
//! - future computational domains.
//!
//! This module does not distinguish those domains.
//!
//! A path such as:
//!
//! ```text
//! quantum::algorithms::grover
//! ```
//!
//! remains merely a source-level path until semantic analysis assigns meaning.
//!
//! # External formats
//!
//! OpenQASM, QIR, Q#, Quil, Cirq and vendor-specific formats must not become
//! variants of this AST node.
//!
//! Their import mechanisms belong to their respective frontend/extension
//! layers. If an external format is imported into Zamani, its importer lowers
//! the external representation into this source-level AST structure.
//!
//! # Rust requirements
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Dependency contract
//!
//! This module may depend only on foundational native-AST infrastructure and
//! the canonical path representation:
//!
//! - `Node`;
//! - `AstNode`;
//! - `NodeId`;
//! - `NodeKind`;
//! - `CoreNodeKind`;
//! - `NodeMetadata`;
//! - `Span`;
//! - `Path`;
//! - `serde`;
//! - the Rust standard library.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - compiler driver;
//! - ZUIR;
//! - quantum IR;
//! - hardware;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime execution;
//! - package registries;
//! - filesystem APIs;
//! - network APIs;
//! - vendor SDKs;
//! - LLVM;
//! - QIR implementation types;
//! - MLIR implementation types.
//!
//! # Integration contract
//!
//! ## Parser
//!
//! The parser creates `Import` from syntactically recognized import forms.
//!
//! The parser owns token recognition.
//!
//! This node owns the resulting source-level structure.
//!
//! ## Structural validation
//!
//! Validation checks:
//!
//! - canonical node kind;
//! - non-empty required paths;
//! - non-empty path segments;
//! - non-empty aliases where present;
//! - non-empty selective import lists;
//! - non-empty external source literals;
//! - structurally valid import items.
//!
//! Validation does NOT check whether an imported name actually exists.
//!
//! ## Semantic analysis
//!
//! Semantic analysis consumes validated imports and determines:
//!
//! - which module/package/source is referenced;
//! - which names are exported;
//! - which aliases are introduced;
//! - whether imports conflict;
//! - whether visibility rules permit the import;
//! - whether dependencies form a valid module graph.
//!
//! ## ZUIR
//!
//! Imports normally disappear during semantic/lowering phases because they
//! establish name-resolution context rather than runtime computation.
//!
//! Any resulting semantic dependency information belongs to the semantic
//! model/compiler graph, not this AST node.
//!
//! ## Visitors
//!
//! Visitors must visit:
//!
//! 1. the import node;
//! 2. the import path;
//! 3. every selective import item;
//! 4. aliases/source-level names where the visitor API exposes them.
//!
//! Source order must be preserved.
//!
//! ## Serialization
//!
//! Serialization preserves:
//!
//! - import kind;
//! - path segment order;
//! - selective item order;
//! - aliases;
//! - external source spelling;
//! - node identity;
//! - source span;
//! - metadata.
//!
//! Resolved semantic identities are never serialized here.
//!
//! ## Determinism
//!
//! All ordered collections use `Vec`.
//!
//! The AST never uses hash-map iteration to establish source order.
//!
//! ## Scalability
//!
//! There is no AST-level maximum for:
//!
//! - import declarations;
//! - path depth;
//! - path segment length;
//! - selective import count;
//! - alias length;
//! - source literal length.
//!
//! Configurable compiler resource policies may impose operational limits
//! outside this representation.
//!
//! ## Security
//!
//! This module performs no I/O and no code execution.
//!
//! Import source strings are data only.
//!
//! A malicious import such as:
//!
//! ```text
//! import * as x from "...";
//! ```
//!
//! cannot cause filesystem or network access merely by existing in the AST.
//!
//! ## Thread safety
//!
//! The node contains ordinary owned source data and no mutable global state.
//!
//! Immutable AST instances can therefore be shared between compiler phases
//! when their constituent types satisfy the corresponding `Send + Sync`
//! requirements.
//!
//! # Canonical design rule
//!
//! This file owns the *syntax* of imports.
//!
//! `Path` owns path structure.
//!
//! Semantic analysis owns resolution.
//!
//! The compiler owns dependency loading.
//!
//! Package tooling owns package discovery.
//!
//! Runtime owns execution.
//!
//! None of those responsibilities may be moved into this module merely for
//! convenience.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::path::Path;
use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Local schema version for [`Import`].
///
/// This is deliberately independent of the Zamani language version, compiler
/// version, and serialized AST schema version.
pub const IMPORT_SCHEMA_VERSION: u16 = 1;

/// Canonical AST kind used by [`Import`].
#[must_use]
pub fn import_node_kind() -> NodeKind {
    NodeKind::core(CoreNodeKind::Import)
}

/// A single explicitly imported source-level item.
///
/// The current grammar permits a single identifier in an import list.
/// Representing it as a `Path` keeps the AST future-proof if the grammar later
/// permits qualified names without changing this data model.
///
/// An alias is optional because the same structure can represent both a
/// direct name and a future aliased selective import.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImportItem {
    path: Path,
    alias: Option<String>,
}

impl ImportItem {
    /// Creates an import item without an alias.
    ///
    /// Structural validity is checked separately by [`Self::validate`].
    #[must_use]
    pub fn new(path: Path) -> Self {
        Self { path, alias: None }
    }

    /// Creates an aliased import item.
    ///
    /// The alias is source-level data and is not resolved here.
    #[must_use]
    pub fn with_alias(path: Path, alias: impl Into<String>) -> Self {
        Self {
            path,
            alias: Some(alias.into()),
        }
    }

    /// Returns the imported source-level path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the optional source-level alias.
    #[must_use]
    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }

    /// Returns whether this item has an explicit alias.
    #[must_use]
    pub fn has_alias(&self) -> bool {
        self.alias.is_some()
    }

    /// Validates the local structure.
    ///
    /// This does not perform name resolution.
    pub fn validate(&self) -> Result<(), ImportValidationError> {
        validate_path(&self.path, "import item path")?;

        if let Some(alias) = &self.alias {
            if alias.is_empty() {
                return Err(ImportValidationError::EmptyAlias);
            }
        }

        Ok(())
    }
}

impl fmt::Display for ImportItem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.path)?;

        if let Some(alias) = &self.alias {
            write!(formatter, " as {alias}")?;
        }

        Ok(())
    }
}

/// Source-level form of an import declaration.
///
/// This is deliberately a closed set of *syntactic forms*, not a list of
/// semantic domains or backends. New semantic domains do not require changes
/// here.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImportSpecifier {
    /// Direct import:
    ///
    /// ```text
    /// import module;
    /// ```
    ///
    /// or:
    ///
    /// ```text
    /// import module as alias;
    /// ```
    Direct {
        /// Source-level path being imported.
        path: Path,

        /// Optional local alias.
        alias: Option<String>,
    },

    /// Selective import:
    ///
    /// ```text
    /// import { A, B } from "source";
    /// ```
    Selective {
        /// Explicitly imported items in source order.
        items: Vec<ImportItem>,

        /// Source literal exactly as represented by the parser.
        ///
        /// It is intentionally not interpreted as a filesystem path, URL,
        /// package name, or registry identifier in this layer.
        source: String,
    },

    /// Glob import:
    ///
    /// ```text
    /// import * as alias from "source";
    /// ```
    Glob {
        /// Local alias introduced by the source syntax.
        alias: String,

        /// Source literal exactly as represented by the parser.
        source: String,
    },
}

impl ImportSpecifier {
    /// Creates a direct import.
    #[must_use]
    pub fn direct(path: Path) -> Self {
        Self::Direct { path, alias: None }
    }

    /// Creates an aliased direct import.
    #[must_use]
    pub fn direct_as(path: Path, alias: impl Into<String>) -> Self {
        Self::Direct {
            path,
            alias: Some(alias.into()),
        }
    }

    /// Creates a selective import.
    #[must_use]
    pub fn selective(items: Vec<ImportItem>, source: impl Into<String>) -> Self {
        Self::Selective {
            items,
            source: source.into(),
        }
    }

    /// Creates a glob import.
    #[must_use]
    pub fn glob(alias: impl Into<String>, source: impl Into<String>) -> Self {
        Self::Glob {
            alias: alias.into(),
            source: source.into(),
        }
    }

    /// Returns whether this is a direct import.
    #[must_use]
    pub fn is_direct(&self) -> bool {
        matches!(self, Self::Direct { .. })
    }

    /// Returns whether this is a aliased direct import.
    #[must_use]
    pub fn is_aliased_direct(&self) -> bool {
        matches!(
            self,
            Self::Direct {
                alias: Some(_),
                ..
            }
        )
    }

    /// Returns whether this is a selective import.
    #[must_use]
    pub fn is_selective(&self) -> bool {
        matches!(self, Self::Selective { .. })
    }

    /// Returns whether this is a glob import.
    #[must_use]
    pub fn is_glob(&self) -> bool {
        matches!(self, Self::Glob { .. })
    }

    /// Returns the direct-import path, if applicable.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::Direct { path, .. } => Some(path),
            Self::Selective { .. } | Self::Glob { .. } => None,
        }
    }

    /// Returns the direct-import alias, if applicable.
    #[must_use]
    pub fn alias(&self) -> Option<&str> {
        match self {
            Self::Direct { alias, .. } => alias.as_deref(),
            Self::Selective { .. } => None,
            Self::Glob { alias, .. } => Some(alias.as_str()),
        }
    }

    /// Returns selective items, if this is a selective import.
    #[must_use]
    pub fn items(&self) -> Option<&[ImportItem]> {
        match self {
            Self::Selective { items, .. } => Some(items),
            Self::Direct { .. } | Self::Glob { .. } => None,
        }
    }

    /// Returns the external source literal, if applicable.
    #[must_use]
    pub fn source(&self) -> Option<&str> {
        match self {
            Self::Selective { source, .. } | Self::Glob { source, .. } => {
                Some(source.as_str())
            }
            Self::Direct { .. } => None,
        }
    }

    /// Returns the number of source-level items represented by this
    /// specifier.
    ///
    /// This is a collection cardinality, not a language-level limit.
    #[must_use]
    pub fn item_count(&self) -> usize {
        match self {
            Self::Direct { .. } | Self::Glob { .. } => 1,
            Self::Selective { items, .. } => items.len(),
        }
    }

    /// Validates this import specifier structurally.
    ///
    /// No module/package/symbol lookup occurs.
    pub fn validate(&self) -> Result<(), ImportValidationError> {
        match self {
            Self::Direct { path, alias } => {
                validate_path(path, "direct import path")?;

                if let Some(alias) = alias {
                    if alias.is_empty() {
                        return Err(ImportValidationError::EmptyAlias);
                    }
                }
            }

            Self::Selective { items, source } => {
                if items.is_empty() {
                    return Err(ImportValidationError::EmptySelectiveImport);
                }

                if source.is_empty() {
                    return Err(ImportValidationError::EmptySource);
                }

                for (index, item) in items.iter().enumerate() {
                    item.validate().map_err(|source_error| {
                        ImportValidationError::InvalidItem {
                            index,
                            source: Box::new(source_error),
                        }
                    })?;
                }
            }

            Self::Glob { alias, source } => {
                if alias.is_empty() {
                    return Err(ImportValidationError::EmptyAlias);
                }

                if source.is_empty() {
                    return Err(ImportValidationError::EmptySource);
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for ImportSpecifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Direct { path, alias } => {
                write!(formatter, "import {path}")?;

                if let Some(alias) = alias {
                    write!(formatter, " as {alias}")?;
                }

                formatter.write_str(";")
            }

            Self::Selective { items, source } => {
                formatter.write_str("import { ")?;

                for (index, item) in items.iter().enumerate() {
                    if index != 0 {
                        formatter.write_str(", ")?;
                    }

                    write!(formatter, "{item}")?;
                }

                write!(formatter, " }} from \"{source}\";")
            }

            Self::Glob { alias, source } => {
                write!(formatter, "import * as {alias} from \"{source}\";")
            }
        }
    }
}

/// Canonical source-level import declaration.
///
/// `Import` owns the source syntax and AST identity. It does not resolve the
/// import.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Import {
    /// Common source-level AST identity, classification, span and metadata.
    node: Node,

    /// Source-level import specification.
    specifier: ImportSpecifier,
}

impl Import {
    /// Creates a structurally validated import node.
    ///
    /// The node kind is always forced to `CoreNodeKind::Import`; callers cannot
    /// accidentally create an import carrying a different core node kind.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        specifier: ImportSpecifier,
    ) -> Result<Self, ImportValidationError> {
        specifier.validate()?;

        Ok(Self {
            node: Node::new(
                id,
                import_node_kind(),
                span,
                metadata,
            ),
            specifier,
        })
    }

    /// Constructs an import while deferring structural validation.
    ///
    /// This exists specifically for parser-recovery and deserialization
    /// pipelines where malformed source structures must be representable before
    /// the canonical validation pass runs.
    ///
    /// It performs no unsafe operation and does not bypass semantic validation;
    /// callers must invoke [`Self::validate`] before semantic analysis.
    #[must_use]
    pub fn from_raw(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        specifier: ImportSpecifier,
    ) -> Self {
        Self {
            node: Node::new(
                id,
                import_node_kind(),
                span,
                metadata,
            ),
            specifier,
        }
    }

    /// Returns the import specification.
    #[must_use]
    pub fn specifier(&self) -> &ImportSpecifier {
        &self.specifier
    }

    /// Returns mutable access to the import specification.
    ///
    /// Mutation is source-level only. Call [`Self::validate`] after modifying
    /// the specification and before passing the AST to semantic analysis.
    pub fn specifier_mut(&mut self) -> &mut ImportSpecifier {
        &mut self.specifier
    }

    /// Replaces the source-level import specification.
    ///
    /// The replacement is validated before mutation, so an existing valid
    /// `Import` remains unchanged if validation fails.
    pub fn replace_specifier(
        &mut self,
        specifier: ImportSpecifier,
    ) -> Result<(), ImportValidationError> {
        specifier.validate()?;
        self.specifier = specifier;
        Ok(())
    }

    /// Returns whether this is a direct import.
    #[must_use]
    pub fn is_direct(&self) -> bool {
        self.specifier.is_direct()
    }

    /// Returns whether this is a selective import.
    #[must_use]
    pub fn is_selective(&self) -> bool {
        self.specifier.is_selective()
    }

    /// Returns whether this is a glob import.
    #[must_use]
    pub fn is_glob(&self) -> bool {
        self.specifier.is_glob()
    }

    /// Returns the direct import path, if applicable.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.specifier.path()
    }

    /// Returns the import alias, if applicable.
    #[must_use]
    pub fn alias(&self) -> Option<&str> {
        self.specifier.alias()
    }

    /// Returns selective import items, if applicable.
    #[must_use]
    pub fn items(&self) -> Option<&[ImportItem]> {
        self.specifier.items()
    }

    /// Returns the external source literal, if applicable.
    #[must_use]
    pub fn source(&self) -> Option<&str> {
        self.specifier.source()
    }

    /// Returns the number of immediate imported items represented by this
    /// declaration.
    #[must_use]
    pub fn item_count(&self) -> usize {
        self.specifier.item_count()
    }

    /// Validates the complete local AST structure.
    ///
    /// This method intentionally performs no semantic name resolution.
    pub fn validate(&self) -> Result<(), ImportValidationError> {
        self.validate_node_kind()?;
        self.specifier.validate()
    }

    /// Returns the local AST schema version.
    #[must_use]
    pub const fn schema_version() -> u16 {
        IMPORT_SCHEMA_VERSION
    }

    /// Returns the complete source-level representation.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        self.specifier.to_string()
    }
}

impl AstNode for Import {
    /// Returns the canonical common AST node.
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }
}

impl fmt::Display for Import {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.specifier.fmt(formatter)
    }
}

// =============================================================================
// Validation
// =============================================================================

/// Structural validation failures for [`Import`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportValidationError {
    /// The embedded node does not carry the canonical import node kind.
    InvalidNodeKind,

    /// A path contains no segments.
    EmptyPath {
        /// Human-readable location within the import structure.
        context: &'static str,
    },

    /// A path contains an empty segment.
    EmptyPathSegment {
        /// Human-readable location within the import structure.
        context: &'static str,

        /// Zero-based path segment index.
        index: usize,
    },

    /// An alias exists syntactically but contains no identifier text.
    EmptyAlias,

    /// A selective import contains no items.
    EmptySelectiveImport,

    /// A `from` source literal is empty.
    EmptySource,

    /// A selective import item is invalid.
    InvalidItem {
        /// Zero-based selective-item index.
        index: usize,

        /// Underlying validation failure.
        source: Box<ImportValidationError>,
    },
}

impl fmt::Display for ImportValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind => {
                formatter.write_str("import AST node has an invalid node kind")
            }

            Self::EmptyPath { context } => {
                write!(formatter, "{context} cannot be empty")
            }

            Self::EmptyPathSegment { context, index } => {
                write!(
                    formatter,
                    "{context} contains an empty segment at index {index}"
                )
            }

            Self::EmptyAlias => {
                formatter.write_str("import alias cannot be empty")
            }

            Self::EmptySelectiveImport => {
                formatter.write_str("selective import must contain at least one item")
            }

            Self::EmptySource => {
                formatter.write_str("import source cannot be empty")
            }

            Self::InvalidItem { index, source } => {
                write!(
                    formatter,
                    "invalid selective import item at index {index}: {source}"
                )
            }
        }
    }
}

impl std::error::Error for ImportValidationError {}

/// Validates a source-level path without performing name resolution.
///
/// The exact segment representation is owned by `paths/path.rs`; this module
/// only requires the canonical path inspection contract.
fn validate_path(
    path: &Path,
    context: &'static str,
) -> Result<(), ImportValidationError> {
    if path.is_empty() {
        return Err(ImportValidationError::EmptyPath { context });
    }

    for (index, segment) in path.segments().iter().enumerate() {
        if segment.is_empty() {
            return Err(ImportValidationError::EmptyPathSegment {
                context,
                index,
            });
        }
    }

    Ok(())
}

// =============================================================================
// Source-level constructors
// =============================================================================

/// Creates a direct import using the canonical source-level path.
pub fn direct_import(
    id: NodeId,
    span: Span,
    metadata: NodeMetadata,
    path: Path,
) -> Result<Import, ImportValidationError> {
    Import::new(
        id,
        span,
        metadata,
        ImportSpecifier::direct(path),
    )
}

/// Creates an aliased direct import.
pub fn aliased_import(
    id: NodeId,
    span: Span,
    metadata: NodeMetadata,
    path: Path,
    alias: impl Into<String>,
) -> Result<Import, ImportValidationError> {
    Import::new(
        id,
        span,
        metadata,
        ImportSpecifier::direct_as(path, alias),
    )
}

/// Creates a selective import.
pub fn selective_import(
    id: NodeId,
    span: Span,
    metadata: NodeMetadata,
    items: Vec<ImportItem>,
    source: impl Into<String>,
) -> Result<Import, ImportValidationError> {
    Import::new(
        id,
        span,
        metadata,
        ImportSpecifier::selective(items, source),
    )
}

/// Creates a glob import.
pub fn glob_import(
    id: NodeId,
    span: Span,
    metadata: NodeMetadata,
    alias: impl Into<String>,
    source: impl Into<String>,
) -> Result<Import, ImportValidationError> {
    Import::new(
        id,
        span,
        metadata,
        ImportSpecifier::glob(alias, source),
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value)
    }

    fn metadata() -> NodeMetadata {
        NodeMetadata::default()
    }

    fn span() -> Span {
        Span::default()
    }

    #[test]
    fn direct_import_is_represented_without_alias() {
        let import = direct_import(
            node_id(1),
            span(),
            metadata(),
            Path::single("math"),
        )
        .expect("valid direct import");

        assert!(import.is_direct());
        assert!(!import.is_selective());
        assert!(!import.is_glob());
        assert_eq!(import.alias(), None);
        assert_eq!(import.path().map(ToString::to_string), Some("math".into()));
    }

    #[test]
    fn aliased_direct_import_preserves_alias() {
        let import = aliased_import(
            node_id(2),
            span(),
            metadata(),
            Path::single("math"),
            "m",
        )
        .expect("valid aliased import");

        assert!(import.is_direct());
        assert_eq!(import.alias(), Some("m"));
        assert_eq!(import.to_source_string(), "import math as m;");
    }

    #[test]
    fn selective_import_preserves_source_order() {
        let import = selective_import(
            node_id(3),
            span(),
            metadata(),
            vec![
                ImportItem::new(Path::single("first")),
                ImportItem::new(Path::single("second")),
                ImportItem::new(Path::single("third")),
            ],
            "library",
        )
        .expect("valid selective import");

        let items = import.items().expect("selective import items");

        assert_eq!(items.len(), 3);
        assert_eq!(items[0].path().to_string(), "first");
        assert_eq!(items[1].path().to_string(), "second");
        assert_eq!(items[2].path().to_string(), "third");
    }

    #[test]
    fn glob_import_preserves_alias_and_source() {
        let import = glob_import(
            node_id(4),
            span(),
            metadata(),
            "library",
            "source",
        )
        .expect("valid glob import");

        assert!(import.is_glob());
        assert_eq!(import.alias(), Some("library"));
        assert_eq!(import.source(), Some("source"));
        assert_eq!(
            import.to_source_string(),
            "import * as library from \"source\";"
        );
    }

    #[test]
    fn qualified_paths_are_supported_without_changing_import_schema() {
        let path = Path::from_segments([
            "quantum",
            "algorithms",
            "grover",
        ]);

        let import = direct_import(
            node_id(5),
            span(),
            metadata(),
            path,
        )
        .expect("valid qualified import");

        assert_eq!(
            import.path().expect("path").to_string(),
            "quantum::algorithms::grover"
        );
    }

    #[test]
    fn empty_direct_path_is_rejected() {
        let result = direct_import(
            node_id(6),
            span(),
            metadata(),
            Path::new(Vec::new()),
        );

        assert_eq!(
            result,
            Err(ImportValidationError::EmptyPath {
                context: "direct import path",
            })
        );
    }

    #[test]
    fn empty_alias_is_rejected() {
        let result = aliased_import(
            node_id(7),
            span(),
            metadata(),
            Path::single("module"),
            "",
        );

        assert_eq!(
            result,
            Err(ImportValidationError::EmptyAlias)
        );
    }

    #[test]
    fn empty_selective_import_is_rejected() {
        let result = selective_import(
            node_id(8),
            span(),
            metadata(),
            Vec::new(),
            "source",
        );

        assert_eq!(
            result,
            Err(ImportValidationError::EmptySelectiveImport)
        );
    }

    #[test]
    fn empty_glob_alias_is_rejected() {
        let result = glob_import(
            node_id(9),
            span(),
            metadata(),
            "",
            "source",
        );

        assert_eq!(
            result,
            Err(ImportValidationError::EmptyAlias)
        );
    }

    #[test]
    fn empty_source_is_rejected() {
        let result = glob_import(
            node_id(10),
            span(),
            metadata(),
            "source",
            "",
        );

        assert_eq!(
            result,
            Err(ImportValidationError::EmptySource)
        );
    }

    #[test]
    fn node_kind_is_canonical_import() {
        let import = direct_import(
            node_id(11),
            span(),
            metadata(),
            Path::single("module"),
        )
        .expect("valid import");

        assert_eq!(
            import.node().kind().as_core(),
            Some(&CoreNodeKind::Import)
        );
    }

    #[test]
    fn from_raw_allows_parser_recovery() {
        let import = Import::from_raw(
            node_id(12),
            span(),
            metadata(),
            ImportSpecifier::Selective {
                items: Vec::new(),
                source: String::new(),
            },
        );

        assert!(import.validate().is_err());
        assert!(import.is_selective());
    }

    #[test]
    fn replacement_is_transactional() {
        let mut import = direct_import(
            node_id(13),
            span(),
            metadata(),
            Path::single("valid"),
        )
        .expect("valid import");

        let result = import.replace_specifier(
            ImportSpecifier::Direct {
                path: Path::new(Vec::new()),
                alias: None,
            },
        );

        assert!(result.is_err());
        assert_eq!(
            import.path().expect("original path").to_string(),
            "valid"
        );
    }

    #[test]
    fn display_matches_source_level_forms() {
        let direct = ImportSpecifier::direct(Path::single("module"));
        assert_eq!(direct.to_string(), "import module;");

        let aliased = ImportSpecifier::direct_as(
            Path::single("module"),
            "m",
        );
        assert_eq!(aliased.to_string(), "import module as m;");

        let selective = ImportSpecifier::selective(
            vec![
                ImportItem::new(Path::single("A")),
                ImportItem::new(Path::single("B")),
            ],
            "library",
        );
        assert_eq!(
            selective.to_string(),
            "import { A, B } from \"library\";"
        );

        let glob = ImportSpecifier::glob("lib", "library");
        assert_eq!(
            glob.to_string(),
            "import * as lib from \"library\";"
        );
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(Import::schema_version(), IMPORT_SCHEMA_VERSION);
        assert_eq!(IMPORT_SCHEMA_VERSION, 1);
    }
}