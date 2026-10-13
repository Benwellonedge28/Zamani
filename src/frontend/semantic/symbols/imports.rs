
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Analysis — Modules, Imports, and Exports
//!
//! Path: `src/frontend/semantic/symbols/imports.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Own the semantic import/export registry and qualified module-path lookup.
//! This module connects declarations collected by the semantic frontend to
//! the canonical symbol table and scope tree.
//!
//! ## Ownership boundaries
//!
//! This module owns:
//! - validated module paths and module registration;
//! - associations between module identities and canonical scope IDs;
//! - explicit exported-symbol registrations;
//! - import bindings, including aliases and symbol re-exports;
//! - import/export consistency validation;
//! - qualified-path lookup through the existing resolution interface.
//!
//! This module does not own:
//! - grammar productions or import-syntax parsing;
//! - AST nodes, AST identities, or source spans;
//! - symbol storage, declaration identity allocation, or lexical scopes;
//! - declaration collection or the language's visibility specification;
//! - dependency-cycle policy, type inference, or semantic diagnostics;
//! - target discovery, hardware allocation, or IR lowering.
//!
//! ## Integration contract
//!
//! - `scopes.rs` owns `ScopeId` and `ScopeTree`.
//! - `table.rs` owns `SymbolNamespace`, `SymbolRecord`, and `SymbolTable`.
//! - `resolution.rs` owns `NameReference`, `QualifiedLookup`, and
//!   `QualifiedPathResolver`.
//! - `declarations.rs` translates canonical AST declarations/imports/exports
//!   into calls to this module.
//! - `cycles.rs` owns dependency-cycle analysis.
//! - The semantic session owns one registry per analysis context.
//!
//! This module does not parse source text or create a second symbol table.
//!
//! ## Collection order
//!
//! Module declarations and exports may be collected before all imports have
//! been resolved. Call `validate_against` after module, declaration, import,
//! and export collection has completed. Do not treat a partially collected
//! registry as a certified complete result.
//!
//! ## Determinism and scalability
//!
//! Ordered maps and sets provide deterministic lookup and iteration.
//! There is no language-level cap on module count, import count, export count,
//! path depth, or computational-domain count. Actual use remains bounded by
//! available resources and Rust's representational limits.
//!
//! ## Visibility contract
//!
//! Register only declarations that the language's visibility rules permit
//! consumers to import. This registry does not infer public visibility from
//! naming conventions. An import marked `reexport` exposes an imported symbol
//! through the importing module, subject to the original export's visibility.
//!
//! The grammar and normative language specifications determine which source
//! forms map to these records. This module does not redefine that syntax.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use super::resolution::{QualifiedLookup, QualifiedPathResolver};
use super::scopes::{ScopeId, ScopeTree};
use super::table::{SymbolNamespace, SymbolRecord, SymbolTable};

/// A canonical module identity expressed as parsed path segments.
///
/// Segments are preserved exactly. Case folding, Unicode normalization,
/// escape handling, and source-syntax interpretation belong to the language
/// and parser contracts, not to this storage type.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModulePath(Vec<String>);

impl ModulePath {
    /// Construct a module path from already-parsed segments.
    ///
    /// Empty paths and empty or whitespace-only segments are rejected.
    /// Segment spelling is otherwise preserved.
    pub fn new<I, S>(segments: I) -> Result<Self, ImportError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let segments: Vec<String> = segments.into_iter().map(Into::into).collect();

        if segments.is_empty() {
            return Err(ImportError::EmptyModulePath);
        }

        for (index, segment) in segments.iter().enumerate() {
            if segment.trim().is_empty() {
                return Err(ImportError::EmptyPathSegment { index });
            }
        }

        Ok(Self(segments))
    }

    /// Return the exact parsed path segments.
    #[must_use]
    pub fn segments(&self) -> &[String] {
        &self.0
    }

    /// Return the final path segment.
    ///
    /// Construction guarantees that at least one segment exists.
    #[must_use]
    pub fn name(&self) -> &str {
        // This is safe because `new` rejects an empty segment list.
        self.0.last().map(String::as_str).unwrap_or("")
    }

    /// Return the parent module path, if this is not a top-level module.
    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        if self.0.len() <= 1 {
            return None;
        }

        Some(Self(self.0[..self.0.len() - 1].to_vec()))
    }

    /// Return a child module path.
    pub fn child(&self, segment: impl Into<String>) -> Result<Self, ImportError> {
        let segment = segment.into();

        if segment.trim().is_empty() {
            return Err(ImportError::EmptyPathSegment {
                index: self.0.len(),
            });
        }

        let mut segments = self.0.clone();
        segments.push(segment);
        Ok(Self(segments))
    }

    /// Return whether this path is a strict prefix of `other`.
    #[must_use]
    pub fn is_ancestor_of(&self, other: &Self) -> bool {
        self.0.len() < other.0.len()
            && other.0.starts_with(&self.0)
    }
}

impl fmt::Display for ModulePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, segment) in self.0.iter().enumerate() {
            if index != 0 {
                f.write_str("::")?;
            }
            f.write_str(segment)?;
        }
        Ok(())
    }
}

/// A registered source module and its canonical lexical scope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleRecord {
    path: ModulePath,
    scope: ScopeId,
}

impl ModuleRecord {
    /// Construct a module record.
    ///
    /// Registration with an `ImportResolver` performs uniqueness checks.
    #[must_use]
    pub const fn new(path: ModulePath, scope: ScopeId) -> Self {
        Self { path, scope }
    }

    /// Return the canonical module path.
    #[must_use]
    pub const fn path(&self) -> &ModulePath {
        &self.path
    }

    /// Return the canonical scope associated with this module.
    #[must_use]
    pub const fn scope(&self) -> ScopeId {
        self.scope
    }
}

/// The destination of an import binding.
///
/// A module import introduces a module alias. A symbol import introduces an
/// alias for one exported declaration in a specified namespace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportTarget {
    /// Import a module under a local name.
    Module(ModulePath),

    /// Import an exported declaration under a local name.
    Symbol {
        /// Module that exports the declaration.
        module: ModulePath,
        /// Exported declaration name, before local aliasing.
        name: String,
        /// Namespace in which the declaration is exported.
        namespace: SymbolNamespace,
    },
}

/// One import binding declared in a source module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportBinding {
    source: ModulePath,
    local_name: String,
    target: ImportTarget,
    reexport: bool,
}

impl ImportBinding {
    /// Construct an import binding.
    ///
    /// The source module must be registered when this binding is inserted.
    /// The target may be registered later, allowing forward module references
    /// during declaration collection. Target validity is checked by
    /// `validate` or `validate_against`.
    pub fn new(
        source: ModulePath,
        local_name: impl Into<String>,
        target: ImportTarget,
        reexport: bool,
    ) -> Result<Self, ImportError> {
        let local_name = local_name.into();

        if local_name.trim().is_empty() {
            return Err(ImportError::EmptyImportAlias);
        }

        if let ImportTarget::Symbol { name, .. } = &target {
            if name.trim().is_empty() {
                return Err(ImportError::EmptyImportedName);
            }
        }

        Ok(Self {
            source,
            local_name,
            target,
            reexport,
        })
    }

    /// Return the importing module.
    #[must_use]
    pub const fn source(&self) -> &ModulePath {
        &self.source
    }

    /// Return the local name introduced by the import.
    #[must_use]
    pub fn local_name(&self) -> &str {
        &self.local_name
    }

    /// Return the target described by this binding.
    #[must_use]
    pub const fn target(&self) -> &ImportTarget {
        &self.target
    }

    /// Return whether this import is intended to be re-exported.
    #[must_use]
    pub const fn is_reexport(&self) -> bool {
        self.reexport
    }
}

/// A registered exported declaration.
///
/// An export points to a declaration identity already owned by the canonical
/// symbol table; it does not duplicate the declaration or its semantic data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportRecord<Id> {
    module: ModulePath,
    name: String,
    namespace: SymbolNamespace,
    symbol_id: Id,
}

impl<Id> ExportRecord<Id> {
    /// Return the module exporting this declaration.
    #[must_use]
    pub const fn module(&self) -> &ModulePath {
        &self.module
    }

    /// Return the exported name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the export namespace.
    #[must_use]
    pub const fn namespace(&self) -> &SymbolNamespace {
        &self.namespace
    }

    /// Return the canonical declaration identity.
    #[must_use]
    pub const fn symbol_id(&self) -> &Id {
        &self.symbol_id
    }
}

/// Outcome of looking up an unqualified import alias.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ImportLookup<Id> {
    /// The alias resolves to one canonical declaration.
    Resolved(Id),
    /// No matching symbol import exists.
    NotFound,
    /// Multiple imported bindings match the requested alias and namespace.
    Ambiguous,
    /// A matching alias imports a module rather than a symbol.
    ModuleAlias,
}

/// Registry for module scopes, imports, and exported declaration identities.
///
/// `Id` is the semantic declaration identity already used by `SymbolTable`.
/// This registry does not allocate or redefine those identities.
#[derive(Clone, Debug)]
pub struct ImportResolver<Id> {
    modules: BTreeMap<ModulePath, ModuleRecord>,
    scope_to_module: BTreeMap<ScopeId, ModulePath>,
    imports: BTreeMap<(ModulePath, String), ImportBinding>,
    exports: BTreeMap<(ModulePath, SymbolNamespace, String), ExportRecord<Id>>,
}

impl<Id> Default for ImportResolver<Id> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Id> ImportResolver<Id> {
    /// Create an empty module/import registry for one analysis context.
    #[must_use]
    pub fn new() -> Self {
        Self {
            modules: BTreeMap::new(),
            scope_to_module: BTreeMap::new(),
            imports: BTreeMap::new(),
            exports: BTreeMap::new(),
        }
    }

    /// Return the number of registered modules.
    #[must_use]
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    /// Return the number of import bindings.
    #[must_use]
    pub fn import_count(&self) -> usize {
        self.imports.len()
    }

    /// Return the number of explicitly registered exports.
    #[must_use]
    pub fn export_count(&self) -> usize {
        self.exports.len()
    }

    /// Register a module and associate it with an existing canonical scope.
    ///
    /// Parent-module existence is checked during final validation so modules
    /// may be collected in any source or traversal order.
    pub fn register_module(
        &mut self,
        record: ModuleRecord,
    ) -> Result<(), ImportError> {
        if self.modules.contains_key(&record.path) {
            return Err(ImportError::DuplicateModule {
                module: record.path,
            });
        }

        if self.scope_to_module.contains_key(&record.scope) {
            return Err(ImportError::ScopeAlreadyAssigned {
                scope: record.scope,
            });
        }

        self.scope_to_module
            .insert(record.scope, record.path.clone());
        self.modules.insert(record.path.clone(), record);
        Ok(())
    }

    /// Register a single exported declaration.
    ///
    /// The declaration identity must already exist in the canonical symbol
    /// table when `validate_against` is called. A name can be exported only
    /// once per module and namespace.
    pub fn add_export(
        &mut self,
        module: ModulePath,
        name: impl Into<String>,
        namespace: SymbolNamespace,
        symbol_id: Id,
    ) -> Result<(), ImportError>
    where
        Id: Clone,
    {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ImportError::EmptyExportName);
        }

        if !self.modules.contains_key(&module) {
            return Err(ImportError::UnknownModule { module });
        }

        let key = (module.clone(), namespace.clone(), name.clone());
        if self.exports.contains_key(&key) {
            return Err(ImportError::DuplicateExport {
                module,
                namespace,
                name,
            });
        }

        self.exports.insert(
            key,
            ExportRecord {
                module,
                name,
                namespace,
                symbol_id,
            },
        );
        Ok(())
    }

    /// Register an import binding.
    ///
    /// Duplicate aliases within one module are rejected instead of silently
    /// choosing an import based on insertion order. Language-specific rules
    /// for legal alias shadowing against local declarations remain the
    /// responsibility of declaration collection and name resolution.
    pub fn add_import(
        &mut self,
        binding: ImportBinding,
    ) -> Result<(), ImportError> {
        if !self.modules.contains_key(&binding.source) {
            return Err(ImportError::UnknownModule {
                module: binding.source,
            });
        }

        let key = (binding.source.clone(), binding.local_name.clone());
        if self.imports.contains_key(&key) {
            return Err(ImportError::DuplicateImportAlias {
                module: binding.source,
                alias: binding.local_name,
            });
        }

        self.imports.insert(key, binding);
        Ok(())
    }

    /// Look up a registered module by its canonical path.
    #[must_use]
    pub fn module(&self, path: &ModulePath) -> Option<&ModuleRecord> {
        self.modules.get(path)
    }

    /// Iterate over modules in deterministic path order.
    pub fn modules(&self) -> impl Iterator<Item = &ModuleRecord> {
        self.modules.values()
    }

    /// Iterate over imports in deterministic module/alias order.
    pub fn imports(&self) -> impl Iterator<Item = &ImportBinding> {
        self.imports.values()
    }

    /// Iterate over explicitly registered exports in deterministic order.
    pub fn exports(&self) -> impl Iterator<Item = &ExportRecord<Id>> {
        self.exports.values()
    }

    /// Resolve a one-segment name against symbol imports in a module.
    ///
    /// Ordinary lexical declarations should be checked by the canonical
    /// lexical resolver first, according to the language's shadowing rules.
    /// This method does not search local declarations or parent scopes.
    pub fn resolve_unqualified(
        &self,
        module: &ModulePath,
        local_name: &str,
        namespace: &SymbolNamespace,
    ) -> ImportLookup<Id>
    where
        Id: Clone,
    {
        let Some(binding) = self
            .imports
            .get(&(module.clone(), local_name.to_owned()))
        else {
            return ImportLookup::NotFound;
        };

        match &binding.target {
            ImportTarget::Module(_) => ImportLookup::ModuleAlias,
            ImportTarget::Symbol {
                module: target_module,
                name,
                namespace: target_namespace,
            } if target_namespace == namespace => {
                match self.find_export(
                    target_module,
                    name,
                    target_namespace,
                    &mut BTreeSet::new(),
                ) {
                    ExportSearch::Found(id) => ImportLookup::Resolved(id),
                    ExportSearch::Missing => ImportLookup::NotFound,
                    ExportSearch::Ambiguous => ImportLookup::Ambiguous,
                }
            }
            ImportTarget::Symbol { .. } => ImportLookup::NotFound,
        }
    }

    /// Validate module parent relationships, import targets, and export
    /// references against the canonical scope tree and symbol table.
    ///
    /// Call this after declaration, module, import, and export collection has
    /// finished. An error means the registry must not be treated as complete.
    pub fn validate_against<T>(
        &self,
        scopes: &ScopeTree,
        symbols: &SymbolTable<Id, ScopeId, T>,
    ) -> Result<(), ImportError>
    where
        Id: Ord + Clone,
    {
        for record in self.modules.values() {
            if scopes.get(record.scope).is_none() {
                return Err(ImportError::UnknownScope {
                    module: record.path.clone(),
                    scope: record.scope,
                });
            }

            if let Some(parent) = record.path.parent() {
                if !self.modules.contains_key(&parent) {
                    return Err(ImportError::MissingParentModule {
                        module: record.path.clone(),
                        parent,
                    });
                }
            }
        }

        for export in self.exports.values() {
            let symbol = symbols
                .get(&export.symbol_id)
                .ok_or_else(|| ImportError::UnknownExportedSymbol {
                    module: export.module.clone(),
                    name: export.name.clone(),
                })?;

            let module = self.modules.get(&export.module).ok_or_else(|| {
                ImportError::UnknownModule {
                    module: export.module.clone(),
                }
            })?;

            if symbol.scope() != &module.scope
                || symbol.name() != export.name
                || symbol.namespace() != &export.namespace
            {
                return Err(ImportError::ExportDoesNotMatchSymbol {
                    module: export.module.clone(),
                    name: export.name.clone(),
                });
            }
        }

        for binding in self.imports.values() {
            match &binding.target {
                ImportTarget::Module(path) => {
                    if !self.modules.contains_key(path) {
                        return Err(ImportError::UnknownImportTarget {
                            source: binding.source.clone(),
                            target: path.clone(),
                        });
                    }
                }
                ImportTarget::Symbol {
                    module,
                    name,
                    namespace,
                } => match self.find_export(
                    module,
                    name,
                    namespace,
                    &mut BTreeSet::new(),
                ) {
                    ExportSearch::Found(_) => {}
                    ExportSearch::Missing => {
                        return Err(ImportError::UnknownImportedSymbol {
                            source: binding.source.clone(),
                            module: module.clone(),
                            name: name.clone(),
                        });
                    }
                    ExportSearch::Ambiguous => {
                        return Err(ImportError::AmbiguousImportedSymbol {
                            source: binding.source.clone(),
                            module: module.clone(),
                            name: name.clone(),
                        });
                    }
                },
            }
        }

        Ok(())
    }

    fn module_for_scope_chain(&self, chain: &[ScopeId]) -> Option<&ModulePath> {
        chain.iter().find_map(|scope| self.scope_to_module.get(scope))
    }

    fn find_export(
        &self,
        module: &ModulePath,
        name: &str,
        namespace: &SymbolNamespace,
        visited: &mut BTreeSet<(ModulePath, SymbolNamespace, String)>,
    ) -> ExportSearch<Id>
    where
        Id: Clone,
    {
        let key = (module.clone(), namespace.clone(), name.to_owned());

        if !visited.insert(key.clone()) {
            // A re-export cycle must not recurse indefinitely.
            return ExportSearch::Missing;
        }

        if let Some(export) = self.exports.get(&key) {
            return ExportSearch::Found(export.symbol_id.clone());
        }

        // Re-exported symbol imports expose the imported declaration under
        // their local alias. The declaration's original identity is retained.
        let Some(binding) = self
            .imports
            .get(&(module.clone(), name.to_owned()))
            .filter(|binding| binding.reexport)
        else {
            return ExportSearch::Missing;
        };

        match &binding.target {
            ImportTarget::Module(_) => ExportSearch::Missing,
            ImportTarget::Symbol {
                module: target_module,
                name: target_name,
                namespace: target_namespace,
            } if target_namespace == namespace => self.find_export(
                target_module,
                target_name,
                target_namespace,
                visited,
            ),
            ImportTarget::Symbol { .. } => ExportSearch::Missing,
        }
    }

    fn resolve_from_module(
        &self,
        module: &ModulePath,
        module_segments: &[String],
        final_name: &str,
        namespace: &SymbolNamespace,
    ) -> QualifiedLookup<Id>
    where
        Id: Clone,
    {
        let mut current = module.clone();

        for segment in module_segments {
            let Ok(next) = current.child(segment.clone()) else {
                return QualifiedLookup::NotFound;
            };

            if !self.modules.contains_key(&next) {
                return QualifiedLookup::NotFound;
            }
            current = next;
        }

        match self.find_export(
            &current,
            final_name,
            namespace,
            &mut BTreeSet::new(),
        ) {
            ExportSearch::Found(id) => QualifiedLookup::Resolved(id),
            ExportSearch::Missing => QualifiedLookup::NotFound,
            ExportSearch::Ambiguous => QualifiedLookup::Ambiguous,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ExportSearch<Id> {
    Found(Id),
    Missing,
    Ambiguous,
}

impl<Id, T> QualifiedPathResolver<Id, T> for ImportResolver<Id>
where
    Id: Ord + Clone,
{
    fn resolve_qualified(
        &self,
        reference: &super::resolution::NameReference,
        scope_chain: &[ScopeId],
        _scopes: &ScopeTree,
        _symbols: &SymbolTable<Id, ScopeId, T>,
    ) -> QualifiedLookup<Id> {
        let segments = reference.segments();

        if segments.len() < 2 {
            return QualifiedLookup::NotFound;
        }

        let Some(current_module) = self.module_for_scope_chain(scope_chain) else {
            return QualifiedLookup::NotFound;
        };

        let first = &segments[0];

        // Import aliases take precedence for qualified lookup. This method
        // handles only paths with at least two segments; unqualified imports
        // are queried through `resolve_unqualified`.
        if let Some(binding) = self
            .imports
            .get(&(current_module.clone(), first.clone()))
        {
            if let ImportTarget::Module(target_module) = &binding.target {
                return self.resolve_from_module(
                    target_module,
                    &segments[1..segments.len() - 1],
                    segments.last().map(String::as_str).unwrap_or(""),
                    reference.namespace(),
                );
            }

            // A symbol import cannot be extended as a module path.
            return QualifiedLookup::NotFound;
        }

        let module_segments = &segments[..segments.len() - 1];
        let final_name = segments.last().map(String::as_str).unwrap_or("");

        // First interpret the path relative to the containing module.
        let relative = self.resolve_from_module(
            current_module,
            module_segments,
            final_name,
            reference.namespace(),
        );

        if !matches!(relative, QualifiedLookup::NotFound) {
            return relative;
        }

        // Also support a fully qualified path beginning at a registered
        // top-level module. The root path is represented by the first module
        // segment, not by an empty ModulePath.
        let absolute_module = match ModulePath::new(module_segments.iter().cloned()) {
            Ok(path) => path,
            Err(_) => return QualifiedLookup::NotFound,
        };

        if self.modules.contains_key(&absolute_module) {
            return match self.find_export(
                &absolute_module,
                final_name,
                reference.namespace(),
                &mut BTreeSet::new(),
            ) {
                ExportSearch::Found(id) => QualifiedLookup::Resolved(id),
                ExportSearch::Missing => QualifiedLookup::NotFound,
                ExportSearch::Ambiguous => QualifiedLookup::Ambiguous,
            };
        }

        QualifiedLookup::NotFound
    }
}

/// Failures in module/import/export registration or validation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ImportError {
    /// A module path contains no segments.
    EmptyModulePath,
    /// A path segment is empty or whitespace-only.
    EmptyPathSegment { index: usize },
    /// An import alias is empty or whitespace-only.
    EmptyImportAlias,
    /// An imported symbol name is empty or whitespace-only.
    EmptyImportedName,
    /// An exported symbol name is empty or whitespace-only.
    EmptyExportName,
    /// The same module path was registered more than once.
    DuplicateModule { module: ModulePath },
    /// A canonical scope was assigned to more than one module.
    ScopeAlreadyAssigned { scope: ScopeId },
    /// An import or export refers to a module that has not been registered.
    UnknownModule { module: ModulePath },
    /// A child module's parent was not registered.
    MissingParentModule {
        module: ModulePath,
        parent: ModulePath,
    },
    /// Two imports in one module use the same local alias.
    DuplicateImportAlias { module: ModulePath, alias: String },
    /// Two declarations use the same exported name and namespace.
    DuplicateExport {
        module: ModulePath,
        namespace: SymbolNamespace,
        name: String,
    },
    /// A registered module refers to a scope absent from the scope tree.
    UnknownScope { module: ModulePath, scope: ScopeId },
    /// An import refers to a module that was never registered.
    UnknownImportTarget { source: ModulePath, target: ModulePath },
    /// An import refers to a symbol that is not exported.
    UnknownImportedSymbol {
        source: ModulePath,
        module: ModulePath,
        name: String,
    },
    /// An imported name resolves to multiple re-export candidates.
    AmbiguousImportedSymbol {
        source: ModulePath,
        module: ModulePath,
        name: String,
    },
    /// An export refers to an unknown declaration identity.
    UnknownExportedSymbol { module: ModulePath, name: String },
    /// Export metadata does not match its canonical symbol record.
    ExportDoesNotMatchSymbol { module: ModulePath, name: String },
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyModulePath => f.write_str("module path must not be empty"),
            Self::EmptyPathSegment { index } => {
                write!(f, "module path segment {index} must not be empty")
            }
            Self::EmptyImportAlias => f.write_str("import alias must not be empty"),
            Self::EmptyImportedName => f.write_str("imported symbol name must not be empty"),
            Self::EmptyExportName => f.write_str("exported symbol name must not be empty"),
            Self::DuplicateModule { module } => {
                write!(f, "module `{module}` is already registered")
            }
            Self::ScopeAlreadyAssigned { scope } => {
                write!(f, "scope `{scope}` is already assigned to a module")
            }
            Self::UnknownModule { module } => {
                write!(f, "module `{module}` is not registered")
            }
            Self::MissingParentModule { module, parent } => {
                write!(f, "parent module `{parent}` of `{module}` is not registered")
            }
            Self::DuplicateImportAlias { module, alias } => {
                write!(f, "import alias `{alias}` is duplicated in module `{module}`")
            }
            Self::DuplicateExport { module, namespace, name } => {
                write!(
                    f,
                    "export `{name}` in namespace `{namespace}` is duplicated in `{module}`"
                )
            }
            Self::UnknownScope { module, scope } => {
                write!(f, "module `{module}` refers to unknown scope `{scope}`")
            }
            Self::UnknownImportTarget { source, target } => {
                write!(f, "module `{source}` imports unknown module `{target}`")
            }
            Self::UnknownImportedSymbol { source, module, name } => {
                write!(f, "module `{source}` imports `{name}` which is not exported by `{module}`")
            }
            Self::AmbiguousImportedSymbol { source, module, name } => {
                write!(f, "import `{name}` in `{source}` is ambiguous in `{module}`")
            }
            Self::UnknownExportedSymbol { module, name } => {
                write!(f, "export `{name}` in `{module}` refers to an unknown symbol")
            }
            Self::ExportDoesNotMatchSymbol { module, name } => {
                write!(f, "export metadata for `{name}` in `{module}` does not match its symbol")
            }
        }
    }
}

impl Error for ImportError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_paths_preserve_segments_and_format_deterministically() {
        let path = ModulePath::new(["math", "linear"]).expect("valid path");

        assert_eq!(path.to_string(), "math::linear");
        assert_eq!(path.name(), "linear");
        assert_eq!(
            path.parent().expect("parent exists").to_string(),
            "math"
        );
    }

    #[test]
    fn empty_module_paths_and_segments_are_rejected() {
        assert_eq!(
            ModulePath::new(Vec::<String>::new()),
            Err(ImportError::EmptyModulePath)
        );

        assert_eq!(
            ModulePath::new(["math", " "]),
            Err(ImportError::EmptyPathSegment { index: 1 })
        );
    }

    #[test]
    fn duplicate_module_paths_are_rejected() {
        let path = ModulePath::new(["math"]).expect("valid path");
        let mut imports = ImportResolver::<u64>::new();

        imports
            .register_module(ModuleRecord::new(path.clone(), ScopeId::root()))
            .expect("first registration succeeds");

        assert_eq!(
            imports.register_module(ModuleRecord::new(path.clone(), ScopeId::root())),
            Err(ImportError::DuplicateModule { module: path })
        );
    }
}
