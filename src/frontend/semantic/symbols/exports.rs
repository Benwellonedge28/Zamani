
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani semantic export plans
//!
//! Path: `src/frontend/semantic/symbols/exports.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Preserve validated, syntax-neutral export intent between canonical AST
//! extraction and semantic module/export resolution.
//!
//! This module is deliberately not an AST, symbol table, module resolver, or
//! package loader. The canonical AST remains the source of syntax and source
//! locations. `symbols/imports.rs` remains responsible for registering
//! resolved exports and imports against canonical symbol identities.
//!
//! ## Supported export forms
//!
//! - Direct export, with an optional alias.
//! - Named export lists, with per-name aliases.
//! - Wildcard exports.
//! - Namespace-style wildcard re-exports.
//! - Optional source references represented as qualified paths or opaque
//!   string-literal values.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - normalized export paths and source references;
//! - validated export targets and named specifiers;
//! - export directives and deterministic per-unit collection order;
//! - structural validation that does not require symbol resolution.
//!
//! It does not own:
//!
//! - grammar, parsing, canonical AST nodes, or source spans;
//! - declaration identities, symbol storage, or lexical scopes;
//! - visibility policy, namespace policy, or name resolution;
//! - wildcard expansion, package lookup, filesystem access, or networking;
//! - import/export dependency-cycle analysis;
//! - type checking, effects, capabilities, resources, or contracts;
//! - target selection, hardware realization, or IR generation.
//!
//! ## Integration contract
//!
//! 1. The parser builds the canonical AST according to
//!    `grammar/modules/exports.g4`.
//! 2. A declaration-collection pass converts the AST export node into these
//!    validated values, retaining source spans on the AST node itself.
//! 3. The semantic export/import pass resolves paths, aliases, visibility,
//!    wildcard expansion, and re-export sources.
//! 4. Resolved direct exports are registered through
//!    `ImportResolver::add_export` in `symbols/imports.rs`; import bindings
//!    are registered through `ImportResolver::add_import`.
//! 5. `ImportResolver::validate_against` validates the resulting registry
//!    against `ScopeTree` and `SymbolTable` after collection is complete.
//!
//! This module intentionally does not guess symbol namespaces or declaration
//! identities. Those must come from the canonical resolver and symbol table.
//!
//! ## Scalability and determinism
//!
//! There is no language-defined limit on the number of directives, path
//! segments, exported declarations, or computational domains. Collection
//! grows with available memory. Rust allocation and representational limits
//! remain implementation constraints, not source-language restrictions.
//!
//! Directive order is preserved because source order can matter to diagnostics
//! and tooling. No global mutable state, hash-map iteration, or unsafe code
//! is used.
//!
//! ## Completion boundary
//!
//! An `ExportPlan` represents collected intent, not a certified export set.
//! Successful construction does not prove that a referenced symbol exists,
//! is visible, has a unique public name, or can be imported. Those checks
//! require the canonical symbol table, module registry, and language policy.

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error;

/// A qualified source-level name represented by already-parsed segments.
///
/// Segments are preserved exactly. Identifier normalization and interpretation
/// are owned by the canonical lexer, parser, and language specification.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExportPath {
    segments: Vec<String>,
}

impl ExportPath {
    /// Construct a qualified path from parsed name segments.
    ///
    /// Empty paths and empty or whitespace-only segments are rejected.
    /// Segment spelling is otherwise preserved.
    pub fn new<I, S>(segments: I) -> Result<Self, ExportError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let segments: Vec<String> = segments.into_iter().map(Into::into).collect();

        if segments.is_empty() {
            return Err(ExportError::EmptyPath);
        }

        for (index, segment) in segments.iter().enumerate() {
            if segment.trim().is_empty() {
                return Err(ExportError::EmptyPathSegment { index });
            }
        }

        Ok(Self { segments })
    }

    /// Return the exact parsed path segments.
    #[must_use]
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    /// Return the final segment of this non-empty path.
    #[must_use]
    pub fn name(&self) -> &str {
        // `new` guarantees at least one segment.
        self.segments.last().map(String::as_str).unwrap_or("")
    }

    /// Return the number of path segments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Return whether this path contains no segments.
    ///
    /// Valid constructed paths are never empty. This method is provided for
    /// conventional collection-style inspection.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
}

impl fmt::Display for ExportPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, segment) in self.segments.iter().enumerate() {
            if index != 0 {
                formatter.write_str("::")?;
            }
            formatter.write_str(segment)?;
        }
        Ok(())
    }
}

/// An export's optional source reference.
///
/// A qualified source remains a logical source-level path. A literal source
/// remains opaque text supplied by the canonical AST; it is not interpreted
/// as a filesystem path or network location here.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ExportSource {
    /// A source expressed as a qualified name.
    Qualified(ExportPath),

    /// A decoded string-literal value from the canonical AST.
    ///
    /// The value is opaque until package/module resolution. This type does not
    /// perform string-literal decoding or escaping.
    Literal(String),
}

impl ExportSource {
    /// Construct an opaque literal source value.
    ///
    /// Empty and whitespace-only values are rejected. The supplied value is
    /// otherwise preserved exactly.
    pub fn literal(value: impl Into<String>) -> Result<Self, ExportError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ExportError::EmptySource);
        }

        Ok(Self::Literal(value))
    }

    /// Construct a qualified-name source.
    #[must_use]
    pub fn qualified(path: ExportPath) -> Self {
        Self::Qualified(path)
    }
}

/// One named entry in an export list.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExportSpecifier {
    path: ExportPath,
    alias: Option<String>,
}

impl ExportSpecifier {
    /// Construct a named export specifier.
    ///
    /// `alias` is the public name introduced by this specifier. If absent,
    /// the final segment of `path` is the requested public name.
    pub fn new(
        path: ExportPath,
        alias: Option<String>,
    ) -> Result<Self, ExportError> {
        if alias.as_deref().is_some_and(|name| name.trim().is_empty()) {
            return Err(ExportError::EmptyAlias);
        }

        Ok(Self { path, alias })
    }

    /// Return the referenced source-level path.
    #[must_use]
    pub fn path(&self) -> &ExportPath {
        &self.path
    }

    /// Return the optional public alias.
    #[must_use]
    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }

    /// Return the requested public name.
    #[must_use]
    pub fn exported_name(&self) -> &str {
        self.alias.as_deref().unwrap_or_else(|| self.path.name())
    }
}

/// A direct path export with an optional public alias.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PathExport {
    path: ExportPath,
    alias: Option<String>,
}

impl PathExport {
    /// Construct a direct export target.
    pub fn new(
        path: ExportPath,
        alias: Option<String>,
    ) -> Result<Self, ExportError> {
        if alias.as_deref().is_some_and(|name| name.trim().is_empty()) {
            return Err(ExportError::EmptyAlias);
        }

        Ok(Self { path, alias })
    }

    /// Return the referenced path.
    #[must_use]
    pub fn path(&self) -> &ExportPath {
        &self.path
    }

    /// Return the optional public alias.
    #[must_use]
    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }

    /// Return the requested public name.
    #[must_use]
    pub fn exported_name(&self) -> &str {
        self.alias.as_deref().unwrap_or_else(|| self.path.name())
    }
}

/// A non-empty named export list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NamedExports {
    specifiers: Vec<ExportSpecifier>,
}

impl NamedExports {
    /// Construct a named export list.
    ///
    /// Duplicate public names within the same list are rejected. Cross-module
    /// and cross-declaration collisions must still be checked after name and
    /// namespace resolution.
    pub fn new<I>(specifiers: I) -> Result<Self, ExportError>
    where
        I: IntoIterator<Item = ExportSpecifier>,
    {
        let specifiers: Vec<ExportSpecifier> = specifiers.into_iter().collect();

        if specifiers.is_empty() {
            return Err(ExportError::EmptyNamedExportList);
        }

        let mut names = BTreeSet::new();
        for specifier in &specifiers {
            let name = specifier.exported_name();

            if !names.insert(name.to_owned()) {
                return Err(ExportError::DuplicateNameInList {
                    name: name.to_owned(),
                });
            }
        }

        Ok(Self { specifiers })
    }

    /// Return the specifiers in source order.
    #[must_use]
    pub fn specifiers(&self) -> &[ExportSpecifier] {
        &self.specifiers
    }

    /// Return the number of specifiers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.specifiers.len()
    }

    /// Return whether the list is empty.
    ///
    /// A valid `NamedExports` value is never empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.specifiers.is_empty()
    }
}

/// A wildcard export with an optional namespace alias.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WildcardExport {
    alias: Option<String>,
}

impl WildcardExport {
    /// Construct a wildcard export.
    ///
    /// A namespace alias is only meaningful for a source-qualified wildcard
    /// re-export. `ExportDirective::new` enforces that relationship.
    pub fn new(alias: Option<String>) -> Result<Self, ExportError> {
        if alias.as_deref().is_some_and(|name| name.trim().is_empty()) {
            return Err(ExportError::EmptyAlias);
        }

        Ok(Self { alias })
    }

    /// Return the optional namespace alias.
    #[must_use]
    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }
}

/// The target shape of an export directive.
///
/// These are semantic input records, not a replacement for the canonical AST.
/// Source spans and exact syntax are retained by the AST node that produced
/// the record.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ExportTarget {
    /// Export one path, optionally under an alias.
    Path(PathExport),

    /// Export a non-empty list of named paths.
    Named(NamedExports),

    /// Export all eligible names, optionally under a namespace alias.
    Wildcard(WildcardExport),
}

impl ExportTarget {
    /// Construct a direct path export.
    pub fn path(
        path: ExportPath,
        alias: Option<String>,
    ) -> Result<Self, ExportError> {
        Ok(Self::Path(PathExport::new(path, alias)?))
    }

    /// Construct a named export list.
    pub fn named<I>(specifiers: I) -> Result<Self, ExportError>
    where
        I: IntoIterator<Item = ExportSpecifier>,
    {
        Ok(Self::Named(NamedExports::new(specifiers)?))
    }

    /// Construct a wildcard export.
    pub fn wildcard(alias: Option<String>) -> Result<Self, ExportError> {
        Ok(Self::Wildcard(WildcardExport::new(alias)?))
    }
}

/// One validated export directive extracted from the canonical AST.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportDirective {
    target: ExportTarget,
    source: Option<ExportSource>,
}

impl ExportDirective {
    /// Construct an export directive from a validated target and optional
    /// source reference.
    ///
    /// A wildcard namespace alias requires a source-qualified re-export.
    /// Whether the source exists and whether the export is legal remain
    /// semantic-resolution responsibilities.
    pub fn new(
        target: ExportTarget,
        source: Option<ExportSource>,
    ) -> Result<Self, ExportError> {
        if matches!(
            &target,
            ExportTarget::Wildcard(WildcardExport { alias: Some(_) })
        ) && source.is_none()
        {
            return Err(ExportError::WildcardAliasRequiresSource);
        }

        Ok(Self { target, source })
    }

    /// Return the export target.
    #[must_use]
    pub fn target(&self) -> &ExportTarget {
        &self.target
    }

    /// Return the optional source reference.
    #[must_use]
    pub fn source(&self) -> Option<&ExportSource> {
        self.source.as_ref()
    }

    /// Return whether this directive is a source-qualified re-export.
    #[must_use]
    pub fn is_reexport(&self) -> bool {
        self.source.is_some()
    }
}

/// An ordered collection of export directives for one semantic analysis unit.
///
/// This plan records intent only. It does not replace `ImportResolver`'s
/// canonical resolved-export registry and must not be used as proof that a
/// program's exports have been resolved or validated.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExportPlan {
    directives: Vec<ExportDirective>,
}

impl ExportPlan {
    /// Create an empty export plan.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            directives: Vec::new(),
        }
    }

    /// Append a validated directive, preserving source collection order.
    pub fn push(&mut self, directive: ExportDirective) {
        self.directives.push(directive);
    }

    /// Return the directives in insertion/source order.
    pub fn directives(&self) -> &[ExportDirective] {
        &self.directives
    }

    /// Return the number of collected directives.
    #[must_use]
    pub fn len(&self) -> usize {
        self.directives.len()
    }

    /// Return whether no directives have been collected.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.directives.is_empty()
    }

    /// Consume the plan and return its directives in source order.
    #[must_use]
    pub fn into_directives(self) -> Vec<ExportDirective> {
        self.directives
    }
}

/// Structural failures detected before symbol/module resolution.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ExportError {
    /// A qualified path has no segments.
    EmptyPath,
    /// A path segment is empty or whitespace-only.
    EmptyPathSegment {
        /// Zero-based position of the invalid segment.
        index: usize,
    },
    /// A source reference is empty or whitespace-only.
    EmptySource,
    /// An alias is empty or whitespace-only.
    EmptyAlias,
    /// A named export list has no entries.
    EmptyNamedExportList,
    /// A public name is repeated in one named export list.
    DuplicateNameInList {
        /// The repeated public name.
        name: String,
    },
    /// A wildcard namespace alias was specified without a source.
    WildcardAliasRequiresSource,
}

impl fmt::Display for ExportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPath => formatter.write_str("export path must not be empty"),
            Self::EmptyPathSegment { index } => {
                write!(formatter, "export path segment {index} must not be empty")
            }
            Self::EmptySource => {
                formatter.write_str("export source must not be empty")
            }
            Self::EmptyAlias => formatter.write_str("export alias must not be empty"),
            Self::EmptyNamedExportList => {
                formatter.write_str("named export list must contain at least one entry")
            }
            Self::DuplicateNameInList { name } => {
                write!(formatter, "export name `{name}` is repeated in the same list")
            }
            Self::WildcardAliasRequiresSource => formatter.write_str(
                "a wildcard namespace alias requires a source-qualified re-export",
            ),
        }
    }
}

impl Error for ExportError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn path(parts: &[&str]) -> ExportPath {
        ExportPath::new(parts.iter().copied()).expect("valid export path")
    }

    #[test]
    fn paths_preserve_segments_and_format_deterministically() {
        let value = path(&["math", "linear", "solve"]);

        assert_eq!(value.segments(), ["math", "linear", "solve"]);
        assert_eq!(value.name(), "solve");
        assert_eq!(value.to_string(), "math::linear::solve");
    }

    #[test]
    fn empty_paths_and_segments_are_rejected() {
        assert_eq!(
            ExportPath::new(Vec::<String>::new()),
            Err(ExportError::EmptyPath)
        );

        assert_eq!(
            ExportPath::new(["math", " "]),
            Err(ExportError::EmptyPathSegment { index: 1 })
        );
    }

    #[test]
    fn aliases_determine_requested_public_names() {
        let direct =
            PathExport::new(path(&["internal", "compute"]), Some("compute_public".into()))
                .expect("valid direct export");

        assert_eq!(direct.exported_name(), "compute_public");

        let item = ExportSpecifier::new(
            path(&["internal", "value"]),
            Some("public_value".into()),
        )
        .expect("valid specifier");

        assert_eq!(item.exported_name(), "public_value");
    }

    #[test]
    fn named_export_lists_reject_duplicate_public_names() {
        let first =
            ExportSpecifier::new(path(&["module_a", "item"]), Some("shared".into()))
                .expect("valid first item");
        let second =
            ExportSpecifier::new(path(&["module_b", "item"]), Some("shared".into()))
                .expect("valid second item");

        assert_eq!(
            NamedExports::new([first, second]),
            Err(ExportError::DuplicateNameInList {
                name: "shared".into()
            })
        );
    }

    #[test]
    fn named_export_lists_must_not_be_empty() {
        assert_eq!(
            NamedExports::new(Vec::<ExportSpecifier>::new()),
            Err(ExportError::EmptyNamedExportList)
        );
    }

    #[test]
    fn wildcard_namespace_alias_requires_a_source() {
        let target = ExportTarget::wildcard(Some("library".into()))
            .expect("valid wildcard target");

        assert_eq!(
            ExportDirective::new(target, None),
            Err(ExportError::WildcardAliasRequiresSource)
        );
    }

    #[test]
    fn qualified_and_literal_sources_remain_distinct() {
        let qualified =
            ExportSource::qualified(path(&["math", "linear"]));
        let literal =
            ExportSource::literal("math-library").expect("valid literal source");

        assert!(matches!(qualified, ExportSource::Qualified(_)));
        assert_eq!(
            literal,
            ExportSource::Literal("math-library".into())
        );
    }

    #[test]
    fn plan_preserves_directive_order() {
        let first = ExportDirective::new(
            ExportTarget::path(path(&["first"]), None).expect("valid target"),
            None,
        )
        .expect("valid directive");

        let second = ExportDirective::new(
            ExportTarget::wildcard(None).expect("valid target"),
            Some(ExportSource::literal("library").expect("valid source")),
        )
        .expect("valid directive");

        let mut plan = ExportPlan::new();
        plan.push(first.clone());
        plan.push(second.clone());

        assert_eq!(plan.len(), 2);
        assert_eq!(plan.directives(), &[first, second]);
    }

    #[test]
    fn literal_sources_must_not_be_empty() {
        assert_eq!(
            ExportSource::literal("  "),
            Err(ExportError::EmptySource)
        );
    }
}
