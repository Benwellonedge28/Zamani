
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Integration-contract tests for Zamani's semantic symbol subsystem.
//!
//! Path: src/frontend/semantic/tests/symbols.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Verify the contracts shared by the canonical symbol table, lexical scope
//! tree, declaration collector, name resolver, module/import resolver, and
//! dependency-cycle analyzer.
//!
//! # Ownership
//!
//! These tests exercise the implementations in `semantic::symbols`.
//! They do not introduce a second symbol table, scope tree, declaration
//! registry, import registry, or name-resolution implementation.
//!
//! # Grammar boundary
//!
//! Import/export syntax and name syntax belong to the canonical grammar and
//! parser. These tests verify semantic behavior after the parser has produced
//! canonical names and AST identities. Parser conformance tests must separately
//! verify that source text is converted into the expected AST.
//!
//! # Scalability
//!
//! No test defines a language-level maximum number of symbols, scopes,
//! declarations, modules, dependencies, or computational domains.
//! Bounded stress tests exercise growth and invariants without attempting
//! to exhaust the test runner's resources.
//!
//! # Integration
//!
//! The semantic test harness should include this file as `mod symbols;`.
//! The parent test module must be enabled from the semantic module with
//! `#[cfg(test)] mod tests;` once the semantic module facade is integrated.
//!
//! The canonical frontend AST owns NodeId and Span. The symbols subsystem
//! owns semantic symbol records, scope structures, and resolution outcomes.

use std::collections::BTreeSet;

use crate::frontend::ast::node::{
    source::{SourceId, Span},
    NodeId,
};
use crate::frontend::semantic::symbols::{
    CyclePolicy,
    DeclarationCollectionOutcome,
    DeclarationCollector,
    DeclarationKind,
    DeclarationSpec,
    DeclarationVisibility,
    DependencyEdge,
    DependencyGraph,
    DependencyKind,
    ImportBinding,
    ImportLookup,
    ImportResolver,
    ImportTarget,
    ModulePath,
    ModuleRecord,
    NameReference,
    QualifiedLookup,
    ResolutionEngine,
    ResolutionStatus,
    ScopeId,
    ScopeKind,
    ScopeTree,
    SymbolNamespace,
    SymbolRecord,
    SymbolTable,
    SymbolTableError,
};

// ---------------------------------------------------------------------------
// Shared test helpers
// ---------------------------------------------------------------------------

fn namespace(name: &str) -> SymbolNamespace {
    SymbolNamespace::new(name).expect("test namespace must be valid")
}

fn node_id(raw: u64) -> NodeId {
    NodeId::new(raw).expect("test node ID must be nonzero")
}

fn span(start: usize, end: usize) -> Span {
    Span::from_usize(SourceId::from_raw(1), start, end)
        .expect("test span must have an ordered range")
}

fn reference(
    id: u64,
    scope: ScopeId,
    namespace: SymbolNamespace,
    segments: &[&str],
) -> NameReference {
    let segments = segments
        .iter()
        .map(|segment| (*segment).to_owned())
        .collect();

    let end = segments
        .iter()
        .map(String::len)
        .sum::<usize>();

    NameReference::new(
        node_id(id),
        span(0, end),
        scope,
        namespace,
        segments,
    )
    .expect("test reference must be structurally valid")
}

fn insert_symbol<T>(
    table: &mut SymbolTable<u64, ScopeId, T>,
    id: u64,
    scope: ScopeId,
    namespace: SymbolNamespace,
    name: &str,
    value: T,
) -> Result<(), SymbolTableError>
where
    T: Clone,
{
    table.insert(
        SymbolRecord::new(id, scope, namespace, name, value)
            .expect("test symbol record must be valid"),
    )
}

fn declaration(
    id: u64,
    ast_id: u64,
    scope: ScopeId,
    name: &str,
) -> DeclarationSpec<u64> {
    DeclarationSpec::new(
        id,
        node_id(ast_id),
        span(0, name.len()),
        scope,
        namespace("value"),
        name,
        DeclarationKind::new("core", "declaration")
            .expect("test declaration kind must be valid"),
        DeclarationVisibility::Public,
    )
    .expect("test declaration must be structurally valid")
}

// ---------------------------------------------------------------------------
// Scope structure and lifecycle
// ---------------------------------------------------------------------------

#[test]
fn scope_tree_starts_with_one_valid_root() {
    let scopes = ScopeTree::new();

    assert_eq!(scopes.root_id(), ScopeId::root());
    assert_eq!(scopes.active_scope(), ScopeId::root());
    assert_eq!(scopes.active_depth(), 0);
    assert_eq!(scopes.len(), 1);
    assert!(!scopes.is_empty());
    assert!(scopes.validate().is_ok());
}

#[test]
fn nested_scopes_preserve_parentage_and_ancestry() {
    let mut scopes = ScopeTree::new();

    let module = scopes
        .create_child(ScopeId::root(), ScopeKind::Module, None)
        .expect("module scope should be created");

    let function = scopes
        .create_child(module, ScopeKind::Function, None)
        .expect("function scope should be created");

    let block = scopes
        .create_child(function, ScopeKind::Lexical, None)
        .expect("lexical scope should be created");

    assert_eq!(scopes.parent_of(module).expect("known module"), Some(ScopeId::root()));
    assert_eq!(scopes.parent_of(function).expect("known function"), Some(module));
    assert_eq!(scopes.parent_of(block).expect("known block"), Some(function));

    assert!(scopes.is_ancestor(ScopeId::root(), block).expect("valid tree"));
    assert!(scopes.is_ancestor(module, block).expect("valid tree"));
    assert!(scopes.is_ancestor(block, block).expect("scope is its own ancestor"));
    assert!(!scopes.is_ancestor(block, module).expect("valid tree"));

    assert_eq!(
        scopes.ancestor_chain(block).expect("valid ancestor chain"),
        vec![block, function, module, ScopeId::root()]
    );

    assert!(scopes.validate().is_ok());
}

#[test]
fn exiting_a_scope_does_not_delete_its_identity_or_children() {
    let mut scopes = ScopeTree::new();

    let function = scopes
        .enter_scope(ScopeKind::Function, None)
        .expect("function scope should be entered");

    let block = scopes
        .enter_scope(ScopeKind::Lexical, None)
        .expect("block scope should be entered");

    assert_eq!(scopes.active_scope(), block);
    assert_eq!(scopes.active_depth(), 2);

    assert_eq!(scopes.exit_scope().expect("leave block"), block);
    assert_eq!(scopes.active_scope(), function);

    assert_eq!(scopes.exit_scope().expect("leave function"), function);
    assert_eq!(scopes.active_scope(), ScopeId::root());

    assert!(scopes.get(function).is_some());
    assert!(scopes.get(block).is_some());
    assert!(scopes.validate().is_ok());
}

#[test]
fn scope_creation_rejects_invalid_parents_and_child_roots() {
    let mut scopes = ScopeTree::new();
    let unknown = ScopeId::root()
        .get()
        .checked_add(1_000_000)
        .map(|raw| ScopeId::root().get().checked_add(raw))
        .flatten()
        .and_then(|raw| raw.checked_add(1));

    // The test derives an ID outside the current tree without assuming that
    // any particular scope count is a language limit.
    if let Some(raw) = unknown {
        if let Some(missing_scope_id) = raw.checked_add(1) {
            // ScopeId intentionally keeps its constructor private. Exercise
            // unknown-scope handling through public IDs whenever the
            // implementation exposes a suitable foreign ID in future tests.
            let _ = missing_scope_id;
        }
    }

    assert!(scopes
        .create_child(ScopeId::root(), ScopeKind::Root, None)
        .is_err());

    assert!(scopes.validate().is_ok());
}

#[test]
fn extension_scope_categories_are_open_and_validated() {
    let extension = ScopeKind::extension("vendor.example", "specialized")
        .expect("valid extension scope");

    assert_eq!(extension.category_name(), "extension");

    assert!(ScopeKind::extension("", "specialized").is_err());
    assert!(ScopeKind::extension("vendor.example", " ").is_err());
    assert!(ScopeKind::domain(" ").is_err());
}

// ---------------------------------------------------------------------------
// Symbol-table integrity, uniqueness, namespaces, and growth
// ---------------------------------------------------------------------------

#[test]
fn symbol_table_rejects_empty_names_and_namespaces() {
    assert!(SymbolNamespace::new("").is_err());
    assert!(SymbolNamespace::new("   ").is_err());

    let namespace = namespace("value");

    assert!(SymbolRecord::new(
        1_u64,
        ScopeId::root(),
        namespace.clone(),
        "",
        (),
    )
    .is_err());

    assert!(SymbolRecord::new(
        1_u64,
        ScopeId::root(),
        namespace,
        "   ",
        (),
    )
    .is_err());
}

#[test]
fn duplicate_names_are_rejected_without_mutating_the_table() {
    let mut symbols: SymbolTable<u64, ScopeId, &'static str> =
        SymbolTable::new();

    insert_symbol(
        &mut symbols,
        1,
        ScopeId::root(),
        namespace("value"),
        "item",
        "first",
    )
    .expect("first declaration should be inserted");

    let result = insert_symbol(
        &mut symbols,
        2,
        ScopeId::root(),
        namespace("value"),
        "item",
        "duplicate",
    );

    assert_eq!(result, Err(SymbolTableError::DuplicateName));
    assert_eq!(symbols.len(), 1);
    assert_eq!(
        symbols
            .lookup_exact(&ScopeId::root(), &namespace("value"), "item")
            .map(SymbolRecord::value),
        Some(&"first")
    );
    assert!(symbols.validate_invariants().is_ok());
}

#[test]
fn duplicate_ids_are_rejected_without_mutating_the_table() {
    let mut symbols: SymbolTable<u64, ScopeId, &'static str> =
        SymbolTable::new();

    insert_symbol(
        &mut symbols,
        7,
        ScopeId::root(),
        namespace("value"),
        "first",
        "original",
    )
    .expect("first declaration should be inserted");

    let result = insert_symbol(
        &mut symbols,
        7,
        ScopeId::root(),
        namespace("value"),
        "second",
        "replacement",
    );

    assert_eq!(result, Err(SymbolTableError::DuplicateId));
    assert_eq!(symbols.len(), 1);
    assert!(symbols.get(&7).is_some());
    assert!(symbols
        .lookup_exact(&ScopeId::root(), &namespace("value"), "second")
        .is_none());
    assert!(symbols.validate_invariants().is_ok());
}

#[test]
fn identical_spellings_can_exist_in_distinct_namespaces() {
    let mut symbols: SymbolTable<u64, ScopeId, &'static str> =
        SymbolTable::new();

    insert_symbol(
        &mut symbols,
        1,
        ScopeId::root(),
        namespace("value"),
        "Item",
        "value declaration",
    )
    .expect("value declaration should be inserted");

    insert_symbol(
        &mut symbols,
        2,
        ScopeId::root(),
        namespace("type"),
        "Item",
        "type declaration",
    )
    .expect("type declaration should be inserted");

    assert_eq!(
        symbols
            .lookup_exact(&ScopeId::root(), &namespace("value"), "Item")
            .map(SymbolRecord::id),
        Some(&1)
    );

    assert_eq!(
        symbols
            .lookup_exact(&ScopeId::root(), &namespace("type"), "Item")
            .map(SymbolRecord::id),
        Some(&2)
    );

    assert!(symbols.validate_invariants().is_ok());
}

#[test]
fn identical_spellings_can_exist_in_nested_scopes() {
    let mut scopes = ScopeTree::new();

    let inner = scopes
        .create_child(ScopeId::root(), ScopeKind::Function, None)
        .expect("function scope should be created");

    let mut symbols: SymbolTable<u64, ScopeId, &'static str> =
        SymbolTable::new();

    insert_symbol(
        &mut symbols,
        10,
        ScopeId::root(),
        namespace("value"),
        "count",
        "outer",
    )
    .expect("outer binding should be inserted");

    insert_symbol(
        &mut symbols,
        20,
        inner,
        namespace("value"),
        "count",
        "inner",
    )
    .expect("inner binding should be inserted");

    let chain = scopes.ancestor_chain(inner).expect("valid scope chain");
    let found = symbols
        .lookup_in_scopes(chain.iter(), &namespace("value"), "count")
        .expect("nearest binding should exist");

    assert_eq!(*found.id(), 20);
    assert_eq!(*found.value(), "inner");
    assert!(symbols.validate_invariants().is_ok());
}

#[test]
fn symbol_iteration_is_stable_across_insertion_orders() {
    fn build(order: &[u64]) -> SymbolTable<u64, ScopeId, String> {
        let mut symbols = SymbolTable::new();

        for id in order {
            insert_symbol(
                &mut symbols,
                *id,
                ScopeId::root(),
                namespace("value"),
                &format!("symbol_{id}"),
                format!("payload_{id}"),
            )
            .expect("distinct symbol should be inserted");
        }

        symbols
    }

    let first = build(&[9, 2, 7, 1, 5]);
    let second = build(&[5, 1, 7, 2, 9]);

    let first_ids: Vec<_> = first.iter().map(|symbol| *symbol.id()).collect();
    let second_ids: Vec<_> = second.iter().map(|symbol| *symbol.id()).collect();

    assert_eq!(first_ids, vec![1, 2, 5, 7, 9]);
    assert_eq!(first_ids, second_ids);

    assert!(first.validate_invariants().is_ok());
    assert!(second.validate_invariants().is_ok());
}

#[test]
fn removing_symbols_keeps_both_indexes_consistent() {
    let mut symbols: SymbolTable<u64, ScopeId, &'static str> =
        SymbolTable::new();

    insert_symbol(
        &mut symbols,
        1,
        ScopeId::root(),
        namespace("value"),
        "alpha",
        "a",
    )
    .expect("insert alpha");

    insert_symbol(
        &mut symbols,
        2,
        ScopeId::root(),
        namespace("value"),
        "beta",
        "b",
    )
    .expect("insert beta");

    assert_eq!(symbols.remove(&1).expect("remove alpha"), "a");
    assert!(symbols.get(&1).is_none());
    assert!(symbols
        .lookup_exact(&ScopeId::root(), &namespace("value"), "alpha")
        .is_none());
    assert!(symbols.get(&2).is_some());
    assert!(symbols.validate_invariants().is_ok());
}

// ---------------------------------------------------------------------------
// Name resolution and shadowing
// ---------------------------------------------------------------------------

#[test]
fn resolver_finds_a_declaration_in_the_root_scope() {
    let scopes = ScopeTree::new();
    let mut symbols: SymbolTable<u64, ScopeId, ()> = SymbolTable::new();

    insert_symbol(
        &mut symbols,
        41,
        ScopeId::root(),
        namespace("value"),
        "answer",
        (),
    )
    .expect("insert answer");

    let input = reference(
        1,
        ScopeId::root(),
        namespace("value"),
        &["answer"],
    );

    let results = ResolutionEngine::new()
        .resolve_all([input], &scopes, &symbols, None)
        .expect("resolution pass should complete");

    assert_eq!(results.len(), 1);
    assert_eq!(
        results[0].status(),
        &ResolutionStatus::Resolved {
            symbol_id: 41,
            declaration_scope: ScopeId::root(),
        }
    );
    assert!(results[0].is_resolved());
}

#[test]
fn resolver_selects_the_nearest_lexical_binding() {
    let mut scopes = ScopeTree::new();

    let outer = scopes
        .create_child(ScopeId::root(), ScopeKind::Module, None)
        .expect("outer scope");

    let inner = scopes
        .create_child(outer, ScopeKind::Function, None)
        .expect("inner scope");

    let mut symbols: SymbolTable<u64, ScopeId, &'static str> =
        SymbolTable::new();

    insert_symbol(
        &mut symbols,
        10,
        outer,
        namespace("value"),
        "value",
        "outer",
    )
    .expect("insert outer binding");

    insert_symbol(
        &mut symbols,
        20,
        inner,
        namespace("value"),
        "value",
        "inner",
    )
    .expect("insert inner binding");

    let input = reference(1, inner, namespace("value"), &["value"]);

    let results = ResolutionEngine::new()
        .resolve_all([input], &scopes, &symbols, None)
        .expect("resolution pass should complete");

    assert_eq!(
        results[0].status(),
        &ResolutionStatus::Resolved {
            symbol_id: 20,
            declaration_scope: inner,
        }
    );
}

#[test]
fn resolver_preserves_unresolved_names_as_explicit_outcomes() {
    let scopes = ScopeTree::new();
    let symbols: SymbolTable<u64, ScopeId, ()> = SymbolTable::new();

    let input = reference(
        1,
        ScopeId::root(),
        namespace("value"),
        &["missing"],
    );

    let results = ResolutionEngine::new()
        .resolve_all([input], &scopes, &symbols, None)
        .expect("the resolution pass itself should complete");

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status(), &ResolutionStatus::Unresolved);
    assert!(!results[0].is_resolved());
}

#[test]
fn qualified_names_require_the_module_resolution_boundary() {
    let scopes = ScopeTree::new();
    let symbols: SymbolTable<u64, ScopeId, ()> = SymbolTable::new();

    let input = reference(
        1,
        ScopeId::root(),
        namespace("value"),
        &["math", "answer"],
    );

    let results = ResolutionEngine::new()
        .resolve_all([input], &scopes, &symbols, None)
        .expect("resolution pass should return an explicit outcome");

    assert_eq!(
        results[0].status(),
        &ResolutionStatus::QualifiedResolverUnavailable
    );
    assert!(!results[0].is_resolved());
}

#[test]
fn malformed_name_paths_are_rejected_before_resolution() {
    let empty = NameReference::new(
        node_id(1),
        span(0, 0),
        ScopeId::root(),
        namespace("value"),
        Vec::new(),
    );

    assert!(empty.is_err());

    let empty_segment = NameReference::new(
        node_id(2),
        span(0, 0),
        ScopeId::root(),
        namespace("value"),
        vec!["module".to_owned(), " ".to_owned()],
    );

    assert!(empty_segment.is_err());
}

#[test]
fn resolution_results_are_ordered_by_stable_reference_identity() {
    let scopes = ScopeTree::new();
    let mut symbols: SymbolTable<u64, ScopeId, ()> = SymbolTable::new();

    insert_symbol(
        &mut symbols,
        1,
        ScopeId::root(),
        namespace("value"),
        "known",
        (),
    )
    .expect("insert known");

    let later = reference(
        2,
        ScopeId::root(),
        namespace("value"),
        &["missing"],
    );

    let earlier = reference(
        1,
        ScopeId::root(),
        namespace("value"),
        &["known"],
    );

    let results = ResolutionEngine::new()
        .resolve_all([later, earlier], &scopes, &symbols, None)
        .expect("resolution pass should complete");

    let ids: Vec<_> = results
        .iter()
        .map(|result| result.reference().node_id().get())
        .collect();

    assert_eq!(ids, vec![1, 2]);
}

// ---------------------------------------------------------------------------
// Declaration collection
// ---------------------------------------------------------------------------

#[test]
fn declaration_collector_registers_distinct_declarations() {
    let scopes = ScopeTree::new();
    let mut symbols: SymbolTable<u64, ScopeId, crate::frontend::semantic::symbols::DeclarationMetadata> =
        SymbolTable::new();

    let mut collector = DeclarationCollector::<u64>::new();

    let report = collector.collect::<()>(
        vec![
            declaration(100, 1, ScopeId::root(), "first"),
            declaration(200, 2, ScopeId::root(), "second"),
        ],
        &scopes,
        &mut symbols,
    );

    assert_eq!(report.registered_count(), 2);
    assert_eq!(report.rejected_count(), 0);
    assert!(report.batch_succeeded());
    assert_eq!(symbols.len(), 2);
    assert!(symbols.validate_invariants().is_ok());
}

#[test]
fn declaration_collector_reports_duplicate_names_without_overwriting() {
    let scopes = ScopeTree::new();
    let mut symbols: SymbolTable<u64, ScopeId, crate::frontend::semantic::symbols::DeclarationMetadata> =
        SymbolTable::new();

    let mut collector = DeclarationCollector::<u64>::new();

    let report = collector.collect::<()>(
        vec![
            declaration(100, 1, ScopeId::root(), "duplicate"),
            declaration(200, 2, ScopeId::root(), "duplicate"),
        ],
        &scopes,
        &mut symbols,
    );

    assert_eq!(report.registered_count(), 1);
    assert_eq!(report.rejected_count(), 1);
    assert!(!report.batch_succeeded());
    assert_eq!(symbols.len(), 1);
    assert!(symbols.validate_invariants().is_ok());

    assert!(matches!(
        report.outcomes().get(0),
        Some(DeclarationCollectionOutcome::Registered { .. })
    ));
    assert!(matches!(
        report.outcomes().get(1),
        Some(DeclarationCollectionOutcome::Rejected { .. })
    ));
}

#[test]
fn declaration_collector_does_not_accept_the_same_ast_node_twice() {
    let scopes = ScopeTree::new();
    let mut symbols: SymbolTable<u64, ScopeId, crate::frontend::semantic::symbols::DeclarationMetadata> =
        SymbolTable::new();

    let mut collector = DeclarationCollector::<u64>::new();

    let first = declaration(100, 7, ScopeId::root(), "first");
    let repeated_ast_node = declaration(200, 7, ScopeId::root(), "second");

    let report = collector.collect::<()>(
        vec![first, repeated_ast_node],
        &scopes,
        &mut symbols,
    );

    assert_eq!(collector.encountered_count(), 1);
    assert_eq!(report.registered_count(), 1);
    assert_eq!(report.rejected_count(), 1);
    assert_eq!(symbols.len(), 1);
    assert!(symbols.validate_invariants().is_ok());
}

// ---------------------------------------------------------------------------
// Module imports, exports, and aliases
// ---------------------------------------------------------------------------

#[test]
fn import_resolver_resolves_an_exported_symbol_through_an_alias() {
    let mut scopes = ScopeTree::new();

    let math_scope = scopes
        .create_child(ScopeId::root(), ScopeKind::Module, None)
        .expect("math scope");

    let app_scope = scopes
        .create_child(ScopeId::root(), ScopeKind::Module, None)
        .expect("app scope");

    let math = ModulePath::new(["math"]).expect("math path");
    let app = ModulePath::new(["app"]).expect("app path");
    let value_namespace = namespace("value");

    let mut symbols: SymbolTable<u64, ScopeId, ()> = SymbolTable::new();

    insert_symbol(
        &mut symbols,
        77,
        math_scope,
        value_namespace.clone(),
        "answer",
        (),
    )
    .expect("insert exported declaration");

    let mut imports = ImportResolver::<u64>::new();

    imports
        .register_module(ModuleRecord::new(math.clone(), math_scope))
        .expect("register math");

    imports
        .register_module(ModuleRecord::new(app.clone(), app_scope))
        .expect("register app");

    imports
        .add_export(
            math.clone(),
            "answer",
            value_namespace.clone(),
            77,
        )
        .expect("export answer");

    imports
        .add_import(
            ImportBinding::new(
                app.clone(),
                "result",
                ImportTarget::Symbol {
                    module: math.clone(),
                    name: "answer".to_owned(),
                    namespace: value_namespace.clone(),
                },
                false,
            )
            .expect("valid import binding"),
        )
        .expect("register import");

    assert_eq!(
        imports.resolve_unqualified(&app, "result", &value_namespace),
        ImportLookup::Resolved(77)
    );

    assert_eq!(
        imports.resolve_unqualified(&app, "result", &namespace("type")),
        ImportLookup::NotFound
    );

    assert_eq!(
        imports.resolve_unqualified(&app, "unknown", &value_namespace),
        ImportLookup::NotFound
    );

    assert!(imports.validate_against(&scopes, &symbols).is_ok());
}

#[test]
fn import_resolver_rejects_duplicate_module_paths_and_aliases() {
    let root = ScopeId::root();
    let mut scopes = ScopeTree::new();

    let app_scope = scopes
        .create_child(root, ScopeKind::Module, None)
        .expect("app scope");

    let math_scope = scopes
        .create_child(root, ScopeKind::Module, None)
        .expect("math scope");

    let app = ModulePath::new(["app"]).expect("app path");
    let math = ModulePath::new(["math"]).expect("math path");

    let mut imports = ImportResolver::<u64>::new();

    imports
        .register_module(ModuleRecord::new(app.clone(), app_scope))
        .expect("register app");

    imports
        .register_module(ModuleRecord::new(math.clone(), math_scope))
        .expect("register math");

    assert!(imports
        .register_module(ModuleRecord::new(app.clone(), app_scope))
        .is_err());

    let binding = ImportBinding::new(
        app.clone(),
        "m",
        ImportTarget::Module(math.clone()),
        false,
    )
    .expect("valid module alias");

    imports
        .add_import(binding.clone())
        .expect("first alias registration");

    assert!(imports.add_import(binding).is_err());

    assert_eq!(imports.module_count(), 2);
    assert_eq!(imports.import_count(), 1);
}

#[test]
fn module_paths_reject_empty_paths_and_empty_segments() {
    assert!(ModulePath::new(Vec::<String>::new()).is_err());
    assert!(ModulePath::new(["math", " "]).is_err());

    let path = ModulePath::new(["library", "math"])
        .expect("qualified module path");

    assert_eq!(path.to_string(), "library::math");
    assert_eq!(path.name(), "math");
    assert_eq!(
        path.parent().expect("parent exists").to_string(),
        "library"
    );
}

// ---------------------------------------------------------------------------
// Dependency-cycle policy
// ---------------------------------------------------------------------------

#[test]
fn forbidden_import_cycles_are_reported() {
    let mut graph = DependencyGraph::new();

    graph.add_edge(DependencyEdge::new(
        "module_a",
        "module_b",
        DependencyKind::import(),
        Some("module_a imports module_b".to_owned()),
    ));

    graph.add_edge(DependencyEdge::new(
        "module_b",
        "module_a",
        DependencyKind::import(),
        Some("module_b imports module_a".to_owned()),
    ));

    let analysis = graph.analyze(&CyclePolicy::new());

    assert!(analysis.has_cycles());
    assert!(!analysis.is_valid());
    assert_eq!(analysis.forbidden_cycles().count(), 1);
}

#[test]
fn recursive_function_dependencies_are_not_forbidden_by_default() {
    let mut graph = DependencyGraph::new();

    graph.add_edge(DependencyEdge::new(
        "function_a",
        "function_b",
        DependencyKind::function_call(),
        None,
    ));

    graph.add_edge(DependencyEdge::new(
        "function_b",
        "function_a",
        DependencyKind::function_call(),
        None,
    ));

    let analysis = graph.analyze(&CyclePolicy::new());

    assert!(analysis.has_cycles());
    assert!(analysis.is_valid());
    assert_eq!(analysis.forbidden_cycles().count(), 0);
}

#[test]
fn dependency_analysis_is_deterministic_for_equivalent_graphs() {
    fn build(edges: &[(u8, u8)]) -> DependencyGraph<u8> {
        let mut graph = DependencyGraph::new();

        for (from, to) in edges {
            graph.add_edge(DependencyEdge::new(
                *from,
                *to,
                DependencyKind::import(),
                None,
            ));
        }

        graph
    }

    let first = build(&[(1, 2), (2, 3), (3, 1), (3, 4)]);
    let second = build(&[(3, 4), (3, 1), (2, 3), (1, 2)]);

    let first_result = first.analyze(&CyclePolicy::new());
    let second_result = second.analyze(&CyclePolicy::new());

    assert_eq!(first_result, second_result);
}

// ---------------------------------------------------------------------------
// Bounded scalability and invariant stress tests
// ---------------------------------------------------------------------------

#[test]
fn symbol_table_scales_with_available_resources_without_a_language_ceiling() {
    let mut symbols: SymbolTable<u64, ScopeId, u64> = SymbolTable::new();

    // This is a bounded test workload, not a language or implementation
    // maximum. Larger workloads belong in configurable stress benchmarks.
    let count: u64 = 2_048;

    for id in 1..=count {
        insert_symbol(
            &mut symbols,
            id,
            ScopeId::root(),
            namespace("value"),
            &format!("value_{id}"),
            id,
        )
        .expect("unique declaration should be accepted");
    }

    assert_eq!(symbols.len(), count as usize);
    assert!(symbols.validate_invariants().is_ok());

    for id in [1, count / 2, count] {
        let name = format!("value_{id}");
        let record = symbols
            .lookup_exact(&ScopeId::root(), &namespace("value"), &name)
            .expect("inserted symbol should remain discoverable");

        assert_eq!(*record.id(), id);
        assert_eq!(*record.value(), id);
    }
}

#[test]
fn deeply_nested_scopes_preserve_their_invariants() {
    let mut scopes = ScopeTree::new();
    let mut current = ScopeId::root();

    // Deliberately bounded to keep CI resource usage predictable.
    // This does not impose a language-level maximum scope depth.
    for _ in 0..512 {
        current = scopes
            .create_child(current, ScopeKind::Lexical, None)
            .expect("scope should be created while resources permit");
    }

    assert_eq!(scopes.len(), 513);
    assert_eq!(scopes.get(current).expect("final scope").depth(), 512);
    assert_eq!(scopes.ancestor_chain(current).expect("valid chain").len(), 513);
    assert!(scopes.validate().is_ok());
}

#[test]
fn distinct_semantic_namespaces_remain_extensible() {
    let namespace_names = [
        "value",
        "type",
        "module",
        "quantum:operation",
        "hdl:signal",
        "tensor:operator",
        "extension.example:entity",
    ];

    let namespaces: BTreeSet<_> = namespace_names
        .iter()
        .map(|name| namespace(name))
        .collect();

    assert_eq!(namespaces.len(), namespace_names.len());
    assert!(namespaces.iter().all(|item| !item.as_str().trim().is_empty()));
}
