
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Frontend Semantic Analysis — Symbol Infrastructure
//!
//! Path: `src/frontend/semantic/symbols/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide the single public module boundary for semantic symbol
//! infrastructure. This module declares and re-exports the existing
//! implementations for declaration collection, dependency-cycle analysis,
//! module imports and exports, name resolution, scope management, and
//! symbol storage.
//!
//! This file is an aggregation boundary, not a second implementation of
//! any of those services.
//!
//! ## Architectural position
//!
//! ```text
//! Canonical frontend AST
//!          |
//!          v
//! Declaration collection
//!          |
//!          +------------------------+
//!          |                        |
//!          v                        v
//!    Scope management         Symbol table
//!          |                        |
//!          +------------+-----------+
//!                       |
//!                       v
//!                Name resolution
//!                       |
//!            +----------+----------+
//!            |                     |
//!            v                     v
//!     Imports / exports     Dependency cycles
//!            |                     |
//!            +----------+----------+
//!                       |
//!                       v
//!                Semantic analysis
//!                       |
//!                       v
//!          Target-independent semantic model
//!                       |
//!                       v
//!             Canonical downstream IRs
//! ```
//!
//! This diagram describes ownership and dependency direction. It does not
//! imply that the complete semantic pipeline or all downstream IR adapters
//! are already integrated.
//!
//! ## Authority and ownership
//!
//! The canonical AST under `crate::frontend::ast::node` owns syntax nodes,
//! AST identities, and source spans.
//!
//! This module does not create a replacement AST, source-span model,
//! declaration-ID allocator, or semantic result model.
//!
//! The child modules retain exclusive responsibility for:
//!
//! - `declarations`: declaration specifications, collection, and collection
//!   outcomes;
//! - `cycles`: generic dependency graphs, cycle policy, and cycle analysis;
//! - `imports`: module paths, module registration, imports, exports,
//!   re-exports, and qualified module lookup;
//! - `resolution`: name-reference records, resolution outcomes, and the
//!   resolution engine;
//! - `scopes`: scope identity, scope-tree structure, and active scope state;
//! - `table`: symbol records, symbol indexes, and consistent symbol storage.
//!
//! Type inference, effect checking, capability checking, resource analysis,
//! contract checking, domain validation, target selection, scheduling,
//! hardware allocation, and IR generation belong to their respective layers.
//!
//! ## Dependency rules
//!
//! 1. `table` is the canonical owner of symbol storage.
//! 2. `scopes` is the canonical owner of scope-tree structure.
//! 3. `declarations` registers declarations using the existing scope and
//!    symbol-table contracts.
//! 4. `resolution` resolves references using the existing scope and symbol
//!    infrastructure, delegating qualified lookup through its resolver
//!    interface.
//! 5. `imports` owns module/import/export records and uses the existing
//!    resolution, scope, and table interfaces.
//! 6. `cycles` analyzes caller-supplied dependency identities and edges; it
//!    does not inspect or duplicate private indexes from other modules.
//!
//! These modules are declared together here, but their responsibilities
//! remain separate. No child module may bypass another module's ownership
//! boundary by maintaining a competing canonical store.
//!
//! ## Grammar integration
//!
//! Source syntax is governed by the canonical grammar and language
//! specifications, including:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/modules/imports.g4`
//! - `grammar/modules/exports.g4`
//! - `grammar/core/names.g4`
//! - `grammar/core/qualified-names.g4`
//! - the normative module, visibility, and semantic specifications.
//!
//! The grammar determines which import, export, alias, qualified-name, and
//! re-export forms are legal. These semantic modules consume canonical AST
//! data produced by the frontend; they must not parse source text or define
//! alternative grammar rules.
//!
//! In particular, aliases and re-exports must preserve their declared public
//! names, and wildcard forms must be expanded only by the appropriate
//! semantic resolution logic.
//!
//! ## Determinism
//!
//! Each child module documents its ordering guarantees. This aggregation
//! boundary introduces no global mutable state and performs no analysis
//! itself. Callers must not infer semantic declaration order from the order
//! in which names happen to be re-exported here.
//!
//! ## Scalability and operational limits
//!
//! This module imposes no language-level ceiling on the number of symbols,
//! scopes, modules, imports, exports, dependencies, computational domains,
//! qubits, processors, devices, or resource requirements.
//!
//! Practical execution is bounded by available memory, CPU time, Rust's
//! representational limits, and any explicitly configured compiler budgets.
//! Those operational constraints must be reported by the responsible
//! analysis pass; they must not be converted into an incorrect claim that
//! incomplete analysis succeeded.
//!
//! No finite implementation can guarantee literal infinity. The design goal
//! is to avoid arbitrary language or hardware ceilings and to scale with
//! available resources.
//!
//! ## Error handling
//!
//! Child modules expose structured errors and explicit result types where
//! applicable. Callers must preserve those errors or translate them into the
//! canonical diagnostic model at the semantic API boundary.
//!
//! Unresolved references, duplicate declarations, invalid imports, forbidden
//! cycles, and exhausted analysis budgets must not silently become successful
//! semantic results.
//!
//! ## Compatibility and integration
//!
//! This module must be included by `src/frontend/semantic/mod.rs`, which in
//! turn must be included by `src/frontend/mod.rs`. Those parent files are
//! separate integration points and must not be assumed to exist merely
//! because this file exists.
//!
//! The existing legacy analyzer in `src/semantic.rs` remains separate until
//! the native frontend's compiler entry points and compatibility tests
//! explicitly complete the migration.
//!
//! Public re-exports below provide convenient paths for consumers. The child
//! modules remain public so advanced consumers can use their full documented
//! APIs without this file duplicating every implementation detail.

// ---------------------------------------------------------------------------
// Canonical child modules
// ---------------------------------------------------------------------------

/// Declaration specifications, declaration collection, and collection
/// reports. AST traversal and translation into declaration specifications
/// belong to the appropriate frontend adapter.
pub mod declarations;

/// Generic directed dependency graphs and configurable cycle analysis.
pub mod cycles;

/// Module registration, imports, exports, aliases, re-exports, and qualified
/// module lookup.
pub mod imports;

/// Source-name references, lexical and qualified name resolution, and
/// resolution outcomes.
pub mod resolution;

/// Lexical scope identity, scope-tree structure, and active scope management.
pub mod scopes;

/// Deterministic storage and indexing of canonical semantic symbol records.
pub mod table;

// ---------------------------------------------------------------------------
// Stable convenience re-exports
// ---------------------------------------------------------------------------

/// Common declaration-collection types.
///
/// The `declarations` module remains the authoritative owner of these types.
pub use declarations::{
    DeclarationCollectionError,
    DeclarationCollectionOutcome,
    DeclarationCollectionReport,
    DeclarationCollector,
    DeclarationKind,
    DeclarationMetadata,
    DeclarationMetadataError,
    DeclarationSpec,
    DeclarationVisibility,
};

/// Common dependency-cycle analysis types.
///
/// Dependency nodes are supplied by the caller. They may represent module
/// paths, semantic declaration IDs, type-definition IDs, or other canonical
/// identities already owned by their respective subsystems.
pub use cycles::{
    CycleAnalysis,
    CycleDisposition,
    CycleGraphError,
    CyclePolicy,
    DependencyCycle,
    DependencyEdge,
    DependencyGraph,
    DependencyKind,
};

/// Common module, import, and export types.
///
/// Module paths preserve parsed name segments; their syntax and normalization
/// remain governed by the language and parser contracts.
pub use imports::{
    ExportRecord,
    ImportBinding,
    ImportError,
    ImportLookup,
    ImportResolver,
    ImportTarget,
    ModulePath,
    ModuleRecord,
};

/// Common name-resolution types.
///
/// Resolution outcomes preserve unresolved and unavailable states rather
/// than treating them as successful resolutions.
pub use resolution::{
    NameReference,
    NameReferenceError,
    QualifiedLookup,
    QualifiedPathResolver,
    ResolutionEngine,
    ResolutionError,
    ResolutionRecord,
    ResolutionStatus,
};

/// Common scope-management types.
///
/// `ScopeId` is local to its originating `ScopeTree`; it is not an AST node
/// ID or a globally unique declaration ID.
pub use scopes::{
    ScopeError,
    ScopeId,
    ScopeInfo,
    ScopeKind,
    ScopeTree,
};

/// Common symbol-table types.
///
/// The symbol table receives identity values from its caller and does not
/// allocate semantic IDs itself.
pub use table::{
    SymbolNamespace,
    SymbolRecord,
    SymbolTable,
    SymbolTableError,
};

// ---------------------------------------------------------------------------
// Public integration contract tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_and_symbol_table_types_are_available_from_the_boundary() {
        let scopes = ScopeTree::new();
        let root = scopes.root_id();

        assert_eq!(root, ScopeId::root());

        let mut symbols: SymbolTable<u64, ScopeId, ()> = SymbolTable::new();
        let namespace =
            SymbolNamespace::new("value").expect("namespace is valid");

        let record = SymbolRecord::new(
            1_u64,
            root,
            namespace,
            "item",
            (),
        )
        .expect("symbol record is valid");

        symbols.insert(record).expect("symbol insertion succeeds");

        assert_eq!(symbols.len(), 1);
        assert!(symbols.validate_invariants().is_ok());
    }

    #[test]
    fn module_path_types_are_available_from_the_boundary() {
        let path =
            ModulePath::new(["library", "math"]).expect("valid module path");

        assert_eq!(path.to_string(), "library::math");
        assert_eq!(path.name(), "math");
        assert_eq!(
            path.parent().expect("parent path exists").to_string(),
            "library"
        );
    }

    #[test]
    fn cycle_analysis_types_are_available_from_the_boundary() {
        let mut graph = DependencyGraph::new();

        graph.add_edge(DependencyEdge::new(
            "module_a",
            "module_b",
            DependencyKind::import(),
            None,
        ));
        graph.add_edge(DependencyEdge::new(
            "module_b",
            "module_a",
            DependencyKind::import(),
            None,
        ));

        let analysis = graph.analyze(&CyclePolicy::new());

        assert!(analysis.has_cycles());
        assert!(!analysis.is_valid());
        assert_eq!(analysis.forbidden_cycles().count(), 1);
    }

    #[test]
    fn recursive_function_dependency_kind_is_not_forbidden_by_default() {
        let policy = CyclePolicy::new();

        assert!(!policy.is_forbidden(&DependencyKind::function_call()));
    }

    #[test]
    fn symbol_namespace_validation_is_preserved() {
        assert!(SymbolNamespace::new(" ").is_err());
        assert!(SymbolNamespace::new("type").is_ok());
    }
}
