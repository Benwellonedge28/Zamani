//! # Zamani Native AST — Export Declarations
//!
//! Canonical source-level representation of Zamani export declarations.
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
//! Native Zamani AST
//!     │
//!     └── Export
//!             │
//!             ▼
//!     structural validation
//!             │
//!             ▼
//!     semantic name/export resolution
//!             │
//!             ▼
//!     semantic model
//!             │
//!             ▼
//!            ZUIR
//!             │
//!             ▼
//!     domain / target lowering
//! ```
//!
//! ## Responsibility
//!
//! This module owns the source-level representation of an `export` declaration.
//!
//! It preserves programmer intent and source structure. It does not determine
//! what an exported name ultimately means, where it is stored, how a package is
//! loaded, or how the exported computation is executed.
//!
//! This module does NOT:
//!
//! - resolve symbols;
//! - resolve modules;
//! - resolve packages;
//! - inspect the filesystem;
//! - access a package registry;
//! - perform network I/O;
//! - assign semantic symbol IDs;
//! - calculate dependency graphs;
//! - select compilation targets;
//! - select hardware;
//! - perform quantum routing;
//! - perform scheduling;
//! - perform calibration;
//! - perform QEC;
//! - perform resilience;
//! - lower directly to quantum IR;
//! - lower directly to ZUIR;
//! - execute exported code.
//!
//! Those responsibilities belong to later compiler layers.
//!
//! ## POCO-REAF
//!
//! Export syntax contains no machine-size assumptions.
//!
//! There is no fixed:
//!
//! - number of exports;
//! - number of exported items;
//! - path depth;
//! - namespace depth;
//! - package count;
//! - module count;
//! - qubit count;
//! - register count;
//! - processor count;
//! - backend count;
//! - hardware topology;
//! - vendor;
//! - execution target.
//!
//! Ordered collections are represented using `Vec`, so their size is bounded
//! only by available compiler resources and explicit compiler resource policies.
//!
//! ## Current grammar
//!
//! The current Zamani grammar defines:
//!
//! ```text
//! export IDENTIFIER ('to' IDENTIFIER)? ';'
//! export '{' exportList '}' ';'
//! export '*' 'from' STRING ';'
//!
//! exportList:
//!     IDENTIFIER ('as' IDENTIFIER)?
//!     (',' IDENTIFIER ('as' IDENTIFIER)?)*
//! ```
//!
//! This AST represents the same three source-level forms while using the
//! canonical `Path` abstraction instead of a single identifier string.
//!
//! Using `Path` permits qualified export names to be supported later without
//! changing the fundamental AST schema.
//!
//! ## Architectural separation
//!
//! ```text
//! Export
//!     = source-level export syntax
//!
//! Path
//!     = source-level unresolved name/path
//!
//! Semantic export
//!     = resolved exported symbol/module/package meaning
//!
//! Dependency graph
//!     = compiler/package-system concern
//!
//! ZUIR
//!     = universal computational representation
//! ```
//!
//! `Export` must never contain resolved semantic identities.
//!
//! Do not add fields such as:
//!
//! ```text
//! resolved_symbol
//! resolved_module
//! package_id
//! registry_id
//! filesystem_path
//! backend_id
//! hardware_id
//! ```
//!
//! Such information belongs in semantic/compiler side tables.
//!
//! ## Domain neutrality
//!
//! An export may expose:
//!
//! - classical functionality;
//! - quantum functionality;
//! - hybrid functionality;
//! - HDL constructs;
//! - AI functionality;
//! - accelerator functionality;
//! - distributed functionality;
//! - mathematical functionality;
//! - future computational domains.
//!
//! This AST node deliberately does not distinguish between those domains.
//!
//! For example:
//!
//! ```text
//! export quantum::algorithms::grover;
//! export quantum::algorithms::qft as qft;
//! export ai::models::transformer;
//! export hdl::alu;
//! ```
//!
//! are all source-level export declarations. Their meanings are determined
//! downstream.
//!
//! ## External languages
//!
//! OpenQASM, QIR, Q#, Quil, Cirq and vendor-specific languages must not become
//! variants of this AST.
//!
//! If an external format is imported into Zamani, its own frontend represents
//! that format first and then lowers into Zamani's semantic/source structures.
//!
//! ## Dependency contract
//!
//! This module may depend only on foundational native-AST infrastructure:
//!
//! - `Path`;
//! - `Node`;
//! - `AstNode`;
//! - `NodeKind`;
//! - `CoreNodeKind`;
//! - `serde`;
//! - the Rust standard library.
//!
//! It must never depend on:
//!
//! - semantic analysis;
//! - symbol tables;
//! - compiler drivers;
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
//! ## Integration contract — parser
//!
//! The parser recognizes the `export` grammar and constructs this node.
//!
//! ```text
//! lexer
//!   │
//!   ▼
//! parser
//!   │
//!   ├── source span
//!   ├── NodeId
//!   ├── metadata
//!   └── ExportSpecifier
//!          │
//!          ▼
//!       Export
//! ```
//!
//! The parser owns token recognition and error recovery.
//!
//! This module owns only the resulting source structure.
//!
//! ## Integration contract — structural validation
//!
//! Structural validation must ensure:
//!
//! - the node kind is `CoreNodeKind::Export`;
//! - direct-export paths are non-empty;
//! - direct-export target paths are non-empty when present;
//! - export lists contain at least one item;
//! - export-item paths are non-empty;
//! - aliases are non-empty when present;
//! - wildcard source literals are structurally representable.
//!
//! An empty string is not rejected merely because it is an empty string literal.
//! String-literal interpretation belongs to the surrounding language semantics.
//!
//! Structural validation must NOT determine whether an exported symbol exists.
//!
//! ## Integration contract — semantic analysis
//!
//! Semantic analysis consumes validated exports and determines:
//!
//! - which source entity is exported;
//! - whether that entity exists;
//! - whether an alias is legal;
//! - whether exports conflict;
//! - whether visibility permits the export;
//! - what module/package graph relationships result;
//! - whether an exported entity is usable by downstream compilation.
//!
//! None of that state belongs inside this AST node.
//!
//! ## Integration contract — ZUIR
//!
//! Exports normally disappear during semantic/lowering stages because they are
//! source/module visibility constructs rather than runtime computation.
//!
//! Any resulting public-symbol or dependency information belongs to the
//! semantic/compiler graph.
//!
//! `Export` must not acquire a direct ZUIR representation merely to preserve
//! the source declaration.
//!
//! ## Integration contract — visitors
//!
//! Visitors must visit:
//!
//! 1. the `Export` node;
//! 2. its direct path where present;
//! 3. its target path where present;
//! 4. every `ExportItem` path;
//! 5. aliases/source-level names where the visitor API exposes them.
//!
//! Source order must always be preserved.
//!
//! ## Integration contract — serialization
//!
//! Serialization must preserve:
//!
//! - export variant;
//! - path segment order;
//! - export-item order;
//! - aliases;
//! - wildcard source spelling;
//! - node identity;
//! - source span;
//! - node metadata.
//!
//! Semantic resolution data must never be serialized into this node.
//!
//! ## Determinism
//!
//! This type contains no unordered collection whose iteration order contributes
//! to AST meaning.
//!
//! `Vec` preserves source order exactly.
//!
//! Construction depends only on supplied source-level data and therefore does
//! not depend on:
//!
//! - memory addresses;
//! - wall-clock time;
//! - randomness;
//! - process IDs;
//! - thread scheduling;
//! - filesystem discovery;
//! - backend discovery.
//!
//! ## Scalability
//!
//! There are no AST-level constants such as:
//!
//! ```text
//! MAX_EXPORTS
//! MAX_EXPORT_ITEMS
//! MAX_EXPORT_PATH_DEPTH
//! MAX_EXPORT_NAME_LENGTH
//! ```
//!
//! The AST therefore remains usable for tiny programs and very large programs,
//! subject only to available resources and explicit compiler safety policies.
//!
//! Compiler resource limits, if needed for hostile input protection, belong in
//! configurable validation/compiler-policy infrastructure.
//!
//! ## Security
//!
//! This module performs no I/O.
//!
//! A wildcard export such as:
//!
//! ```text
//! export * from "some-source";
//! ```
//!
//! stores `"some-source"` as source-level data. Merely constructing the AST
//! cannot open a file, contact a registry, load a package, or execute code.
//!
//! ## Rust requirements
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::path::Path;
use super::super::node::{AstNode, Node};
use super::super::node_kind::{CoreNodeKind, NodeKind};

/// Schema version for the native `Export` AST node.
///
/// This version is independent of:
///
/// - Zamani language version;
/// - compiler version;
/// - complete AST schema version;
/// - semantic-model version;
/// - ZUIR version;
/// - package version.
pub const EXPORT_SCHEMA_VERSION: u16 = 1;

/// Returns the canonical node kind for an [`Export`] node.
#[must_use]
pub fn export_node_kind() -> NodeKind {
    NodeKind::core(CoreNodeKind::Export)
}

/// One explicitly exported source-level item.
///
/// Current grammar:
///
/// ```text
/// IDENTIFIER
/// IDENTIFIER as IDENTIFIER
/// ```
///
/// The underlying path representation is intentionally more general so
/// qualified source names can be supported without redesigning this node.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExportItem {
    path: Path,
    alias: Option<String>,
}

impl ExportItem {
    /// Creates an unaliased export item.
    #[must_use]
    pub fn new(path: Path) -> Self {
        Self { path, alias: None }
    }

    /// Creates an aliased export item.
    #[must_use]
    pub fn with_alias(path: Path, alias: impl Into<String>) -> Self {
        Self {
            path,
            alias: Some(alias.into()),
        }
    }

    /// Returns the source-level path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the optional export alias.
    #[must_use]
    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }

    /// Returns whether the item has an alias.
    #[must_use]
    pub fn has_alias(&self) -> bool {
        self.alias.is_some()
    }

    /// Validates the item's local structural invariants.
    ///
    /// This method deliberately performs no symbol resolution.
    pub fn validate(&self) -> Result<(), ExportValidationError> {
        validate_path(&self.path, "export item path")?;

        if let Some(alias) = &self.alias {
            if alias.is_empty() {
                return Err(ExportValidationError::EmptyAlias);
            }
        }

        Ok(())
    }
}

impl fmt::Display for ExportItem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.path)?;

        if let Some(alias) = &self.alias {
            write!(formatter, " as {alias}")?;
        }

        Ok(())
    }
}

/// Source-level export syntax.
///
/// This enum models syntax forms, not semantic domains.
///
/// New computational domains do not require changes to this representation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExportSpecifier {
    /// Direct export:
    ///
    /// ```text
    /// export foo;
    /// ```
    ///
    /// or:
    ///
    /// ```text
    /// export foo to bar;
    /// ```
    Direct {
        /// Source-level entity being exported.
        path: Path,

        /// Optional source-level export destination/name.
        ///
        /// The current grammar calls this the `to` identifier.
        target: Option<Path>,
    },

    /// Named export list:
    ///
    /// ```text
    /// export { foo, bar as baz };
    /// ```
    List {
        /// Export items in exact source order.
        items: Vec<ExportItem>,
    },

    /// Wildcard export:
    ///
    /// ```text
    /// export * from "source";
    /// ```
    Glob {
        /// Source literal exactly as written by the parser.
        ///
        /// This value is not interpreted as a filesystem path, URL, package
        /// locator, registry identifier, or module ID at the AST layer.
        source: String,
    },
}

impl ExportSpecifier {
    /// Creates a direct export.
    #[must_use]
    pub fn direct(path: Path) -> Self {
        Self::Direct {
            path,
            target: None,
        }
    }

    /// Creates a direct export with a `to` target.
    #[must_use]
    pub fn direct_to(path: Path, target: Path) -> Self {
        Self::Direct {
            path,
            target: Some(target),
        }
    }

    /// Creates a named export list.
    #[must_use]
    pub fn list(items: Vec<ExportItem>) -> Self {
        Self::List { items }
    }

    /// Creates a wildcard export.
    #[must_use]
    pub fn glob(source: impl Into<String>) -> Self {
        Self::Glob {
            source: source.into(),
        }
    }

    /// Returns whether this is a direct export.
    #[must_use]
    pub fn is_direct(&self) -> bool {
        matches!(self, Self::Direct { .. })
    }

    /// Returns whether this is a direct export with a `to` target.
    #[must_use]
    pub fn is_direct_to(&self) -> bool {
        matches!(
            self,
            Self::Direct {
                target: Some(_),
                ..
            }
        )
    }

    /// Returns whether this is a named export list.
    #[must_use]
    pub fn is_list(&self) -> bool {
        matches!(self, Self::List { .. })
    }

    /// Returns whether this is a wildcard export.
    #[must_use]
    pub fn is_glob(&self) -> bool {
        matches!(self, Self::Glob { .. })
    }

    /// Returns the direct-export path, if applicable.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::Direct { path, .. } => Some(path),
            Self::List { .. } | Self::Glob { .. } => None,
        }
    }

    /// Returns the direct-export target, if applicable.
    #[must_use]
    pub fn target(&self) -> Option<&Path> {
        match self {
            Self::Direct { target, .. } => target.as_ref(),
            Self::List { .. } | Self::Glob { .. } => None,
        }
    }

    /// Returns the ordered named export items, if this is a list export.
    #[must_use]
    pub fn items(&self) -> Option<&[ExportItem]> {
        match self {
            Self::List { items } => Some(items.as_slice()),
            Self::Direct { .. } | Self::Glob { .. } => None,
        }
    }

    /// Returns the wildcard source literal, if this is a glob export.
    #[must_use]
    pub fn source(&self) -> Option<&str> {
        match self {
            Self::Glob { source } => Some(source.as_str()),
            Self::Direct { .. } | Self::List { .. } => None,
        }
    }

    /// Validates the local structural invariants.
    ///
    /// This method does not resolve names or inspect external sources.
    pub fn validate(&self) -> Result<(), ExportValidationError> {
        match self {
            Self::Direct { path, target } => {
                validate_path(path, "export path")?;

                if let Some(target) = target {
                    validate_path(target, "export target")?;
                }

                Ok(())
            }

            Self::List { items } => {
                if items.is_empty() {
                    return Err(ExportValidationError::EmptyExportList);
                }

                for item in items {
                    item.validate()?;
                }

                Ok(())
            }

            Self::Glob { .. } => {
                // Empty string literals are syntactically representable and
                // therefore are not rejected here. Semantic/package resolution
                // decides whether such a source is meaningful.
                Ok(())
            }
        }
    }
}

impl fmt::Display for ExportSpecifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Direct { path, target } => {
                write!(formatter, "export {path}")?;

                if let Some(target) = target {
                    write!(formatter, " to {target}")?;
                }

                Ok(())
            }

            Self::List { items } => {
                formatter.write_str("export { ")?;

                for (index, item) in items.iter().enumerate() {
                    if index != 0 {
                        formatter.write_str(", ")?;
                    }

                    write!(formatter, "{item}")?;
                }

                formatter.write_str(" }")
            }

            Self::Glob { source } => {
                write!(formatter, "export * from \"{source}\"")
            }
        }
    }
}

/// Canonical source-level Zamani export declaration.
///
/// `Export` owns the common AST [`Node`] plus the source-level export syntax.
///
/// It contains no resolved symbols or target-specific state.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Export {
    node: Node,
    specifier: ExportSpecifier,
}

impl Export {
    /// Constructs an export declaration from an already allocated canonical
    /// AST node.
    ///
    /// The supplied node must have `CoreNodeKind::Export`.
    ///
    /// No semantic resolution is performed.
    pub fn new(
        node: Node,
        specifier: ExportSpecifier,
    ) -> Result<Self, ExportValidationError> {
        let expected = export_node_kind();

        if !node.is_kind(&expected) {
            return Err(ExportValidationError::InvalidNodeKind {
                expected,
                actual: node.kind().clone(),
            });
        }

        specifier.validate()?;

        Ok(Self { node, specifier })
    }

    /// Constructs an export declaration without semantic resolution.
    ///
    /// This is an alias for [`Self::new`] intended for parser/builder code where
    /// the `try_` naming convention makes fallibility explicit.
    pub fn try_new(
        node: Node,
        specifier: ExportSpecifier,
    ) -> Result<Self, ExportValidationError> {
        Self::new(node, specifier)
    }

    /// Returns the canonical common AST node.
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    ///
    /// Callers that modify metadata or spans must preserve AST-wide structural
    /// invariants and run validation after transformations where appropriate.
    #[must_use]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the export specification.
    #[must_use]
    pub fn specifier(&self) -> &ExportSpecifier {
        &self.specifier
    }

    /// Returns mutable access to the export specification.
    ///
    /// Mutating the specification can invalidate its structural invariants.
    /// Callers must run [`Self::validate`] after arbitrary mutation.
    #[must_use]
    pub fn specifier_mut(&mut self) -> &mut ExportSpecifier {
        &mut self.specifier
    }

    /// Replaces the export specification after validating it.
    ///
    /// The existing specification is returned only after the replacement has
    /// been structurally validated.
    pub fn replace_specifier(
        &mut self,
        specifier: ExportSpecifier,
    ) -> Result<ExportSpecifier, ExportValidationError> {
        specifier.validate()?;

        Ok(core::mem::replace(&mut self.specifier, specifier))
    }

    /// Consumes the AST node and returns its export specification.
    #[must_use]
    pub fn into_specifier(self) -> ExportSpecifier {
        self.specifier
    }

    /// Returns whether this is a direct export.
    #[must_use]
    pub fn is_direct(&self) -> bool {
        self.specifier.is_direct()
    }

    /// Returns whether this is a `to` export.
    #[must_use]
    pub fn is_direct_to(&self) -> bool {
        self.specifier.is_direct_to()
    }

    /// Returns whether this is a named export list.
    #[must_use]
    pub fn is_list(&self) -> bool {
        self.specifier.is_list()
    }

    /// Returns whether this is a wildcard export.
    #[must_use]
    pub fn is_glob(&self) -> bool {
        self.specifier.is_glob()
    }

    /// Returns the direct-export path, if applicable.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.specifier.path()
    }

    /// Returns the direct-export target, if applicable.
    #[must_use]
    pub fn target(&self) -> Option<&Path> {
        self.specifier.target()
    }

    /// Returns ordered named export items, if applicable.
    #[must_use]
    pub fn items(&self) -> Option<&[ExportItem]> {
        self.specifier.items()
    }

    /// Returns the wildcard source, if applicable.
    #[must_use]
    pub fn source(&self) -> Option<&str> {
        self.specifier.source()
    }

    /// Validates the complete local export structure.
    ///
    /// The common `Node` classification is validated first, followed by the
    /// export-specific source structure.
    ///
    /// No name lookup, package lookup, filesystem access, or semantic analysis
    /// occurs here.
    pub fn validate(&self) -> Result<(), ExportValidationError> {
        let expected = export_node_kind();

        if !self.node.is_kind(&expected) {
            return Err(ExportValidationError::InvalidNodeKind {
                expected,
                actual: self.node.kind().clone(),
            });
        }

        self.specifier.validate()
    }

    /// Returns the independent schema version for this AST node.
    #[must_use]
    pub const fn schema_version() -> u16 {
        EXPORT_SCHEMA_VERSION
    }
}

impl AstNode for Export {
    fn node(&self) -> &Node {
        &self.node
    }

    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl fmt::Display for Export {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.specifier.fmt(formatter)
    }
}

/// Structural validation errors for an [`Export`].
///
/// These errors intentionally describe only local AST structure.
///
/// Semantic errors such as "symbol does not exist" or "export is not visible"
/// belong to semantic analysis.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExportValidationError {
    /// The export node has the wrong canonical AST node kind.
    InvalidNodeKind {
        /// Expected node kind.
        expected: NodeKind,

        /// Actual node kind.
        actual: NodeKind,
    },

    /// A required path has no segments.
    EmptyPath {
        /// Logical path role.
        context: &'static str,
    },

    /// A path contains an empty source segment.
    EmptyPathSegment {
        /// Logical path role.
        context: &'static str,

        /// Zero-based source-order segment index.
        index: usize,
    },

    /// An export alias is empty.
    EmptyAlias,

    /// A named export list contains no items.
    EmptyExportList,
}

impl fmt::Display for ExportValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { expected, actual } => {
                write!(
                    formatter,
                    "invalid export node kind: expected {expected}, found {actual}"
                )
            }

            Self::EmptyPath { context } => {
                write!(formatter, "{context} must contain at least one path segment")
            }

            Self::EmptyPathSegment { context, index } => {
                write!(
                    formatter,
                    "{context} contains an empty path segment at index {index}"
                )
            }

            Self::EmptyAlias => {
                formatter.write_str("export alias must not be empty")
            }

            Self::EmptyExportList => {
                formatter.write_str("export list must contain at least one item")
            }
        }
    }
}

impl std::error::Error for ExportValidationError {}

/// Validates a source-level path for use in an export declaration.
///
/// This intentionally validates only the generic `Path` structure. It does not
/// determine whether the path resolves to an actual declaration.
fn validate_path(
    path: &Path,
    context: &'static str,
) -> Result<(), ExportValidationError> {
    if path.is_empty() {
        return Err(ExportValidationError::EmptyPath { context });
    }

    for (index, segment) in path.segments().iter().enumerate() {
        if segment.is_empty() {
            return Err(ExportValidationError::EmptyPathSegment {
                context,
                index,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::path::PathSegment;

    fn path(parts: &[&str]) -> Path {
        Path::from_segments(parts.iter().copied().map(PathSegment::from_str))
    }

    #[test]
    fn direct_export_preserves_path() {
        let export = ExportSpecifier::direct(path(&["foo", "bar"]));

        assert!(export.is_direct());
        assert!(!export.is_direct_to());
        assert_eq!(
            export.path().map(ToString::to_string),
            Some("foo::bar".to_string())
        );
    }

    #[test]
    fn direct_export_to_preserves_target() {
        let export = ExportSpecifier::direct_to(
            path(&["foo"]),
            path(&["public", "foo"]),
        );

        assert!(export.is_direct_to());
        assert_eq!(
            export.path().map(ToString::to_string),
            Some("foo".to_string())
        );
        assert_eq!(
            export.target().map(ToString::to_string),
            Some("public::foo".to_string())
        );
    }

    #[test]
    fn aliased_export_item_preserves_alias() {
        let item = ExportItem::with_alias(path(&["foo"]), "bar");

        assert_eq!(item.alias(), Some("bar"));
        assert!(item.has_alias());
        assert!(item.validate().is_ok());
    }

    #[test]
    fn list_export_preserves_source_order() {
        let items = vec![
            ExportItem::new(path(&["first"])),
            ExportItem::with_alias(path(&["second"]), "renamed"),
            ExportItem::new(path(&["third"])),
        ];

        let export = ExportSpecifier::list(items);

        let names: Vec<String> = export
            .items()
            .expect("list export")
            .iter()
            .map(|item| item.path().to_string())
            .collect();

        assert_eq!(
            names,
            vec![
                "first".to_string(),
                "second".to_string(),
                "third".to_string()
            ]
        );
    }

    #[test]
    fn empty_list_is_rejected() {
        let export = ExportSpecifier::list(Vec::new());

        assert_eq!(
            export.validate(),
            Err(ExportValidationError::EmptyExportList)
        );
    }

    #[test]
    fn empty_path_is_rejected() {
        let export = ExportSpecifier::direct(Path::empty());

        assert_eq!(
            export.validate(),
            Err(ExportValidationError::EmptyPath {
                context: "export path"
            })
        );
    }

    #[test]
    fn empty_path_segment_is_rejected() {
        let export = ExportSpecifier::direct(path(&["foo", ""]));

        assert_eq!(
            export.validate(),
            Err(ExportValidationError::EmptyPathSegment {
                context: "export path",
                index: 1
            })
        );
    }

    #[test]
    fn empty_alias_is_rejected() {
        let item = ExportItem::with_alias(path(&["foo"]), "");

        assert_eq!(
            item.validate(),
            Err(ExportValidationError::EmptyAlias)
        );
    }

    #[test]
    fn empty_glob_source_is_structurally_representable() {
        // `""` can be a syntactically valid string literal. Whether it is a
        // meaningful module/package source is semantic/package resolution.
        let export = ExportSpecifier::glob("");

        assert!(export.validate().is_ok());
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(Export::schema_version(), EXPORT_SCHEMA_VERSION);
        assert_eq!(EXPORT_SCHEMA_VERSION, 1);
    }

    #[test]
    fn display_is_deterministic() {
        let direct = ExportSpecifier::direct_to(
            path(&["foo"]),
            path(&["bar"]),
        );

        assert_eq!(direct.to_string(), "export foo to bar");

        let list = ExportSpecifier::list(vec![
            ExportItem::new(path(&["foo"])),
            ExportItem::with_alias(path(&["bar"]), "baz"),
        ]);

        assert_eq!(list.to_string(), "export { foo, bar as baz }");

        let glob = ExportSpecifier::glob("library");

        assert_eq!(glob.to_string(), "export * from \"library\"");
    }
}