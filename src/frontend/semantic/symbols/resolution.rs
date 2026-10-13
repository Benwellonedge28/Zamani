
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Analysis — Name Resolution
//!
//! Path: `src/frontend/semantic/symbols/resolution.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Resolve source-level name references to declarations collected in the
//! canonical semantic symbol table.
//!
//! This module owns the resolution pass and its result types. It does not
//! collect declarations, define scopes, implement imports, parse source text,
//! infer types, or select hardware.
//!
//! ## Architectural boundaries
//!
//! ```text
//! Canonical Zamani AST
//!         |
//!         v
//! Declaration collection
//!         |
//!         v
//! ScopeTree + SymbolTable
//!         |
//!         v
//! Name resolution  <-- this module
//!         |
//!         +--> lexical names: SymbolTable + ScopeTree
//!         |
//!         +--> qualified paths: imports/module resolver
//!         |
//!         v
//! Resolved declaration identities
//!         |
//!         v
//! Types / effects / capabilities / resources / contracts
//!         |
//!         v
//! Canonical semantic model
//!         |
//!         v
//! Classical IR / quantum::ir / HDL and other domain IRs
//! ```
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - canonical input records for collected name references;
//! - resolution outcomes and pass-level infrastructure errors;
//! - deterministic lexical lookup through the canonical scope tree;
//! - qualified-name resolver integration;
//! - deterministic resolution results;
//! - checks that qualified resolvers return registered declarations.
//!
//! This module does not own:
//!
//! - AST node definitions, `NodeId`, or `Span`;
//! - scope creation or scope ancestry;
//! - symbol storage or semantic ID allocation;
//! - declaration collection or duplicate-declaration policy;
//! - module/import graphs, exports, aliases, or visibility policy;
//! - the grammar for identifiers or qualified paths;
//! - type inference, effects, capabilities, or resource validation;
//! - target feasibility, scheduling, routing, or IR generation.
//!
//! ## Grammar and AST contract
//!
//! The grammar and canonical AST determine which source constructs introduce
//! name references and how their paths are represented. An upstream AST
//! traversal/declaration pass must translate those constructs into
//! [`NameReference`] records.
//!
//! A path is represented as ordered segments. This module does not split source
//! strings on punctuation, normalize identifiers, or infer grammar rules.
//! The parser and language specification own those decisions.
//!
//! ## Declaration-order contract
//!
//! This pass expects declaration collection to have completed for the relevant
//! analysis unit. This permits forward references wherever the language
//! specifies them. Incomplete declaration collection must not be reported as
//! successful resolution.
//!
//! ## Resolution policy
//!
//! A single-segment name is looked up from the reference scope outward through
//! its ancestors. The nearest matching declaration wins, subject to the
//! language's namespace and shadowing rules.
//!
//! A multi-segment path is delegated to [`QualifiedPathResolver`]. This keeps
//! module traversal, imports, exports, aliases, visibility and qualified-name
//! ambiguity under their appropriate owner.
//!
//! Namespaces are supplied by the caller. This module has no closed list of
//! source-language domains or hardware-specific name categories.
//!
//! ## Determinism
//!
//! Results are ordered deterministically by reference identity and stable
//! reference attributes. Lexical lookup uses the canonical scope chain and
//! deterministic symbol table. No hash-map iteration or global mutable state
//! is used here.
//!
//! ## Scalability and POCO-REAF
//!
//! This module imposes no language-level ceiling on the number of declarations,
//! references, scopes, domains, devices, qubits, tensor dimensions or machines.
//! Collection sizes grow according to available resources and Rust's
//! representational limits.
//!
//! Operational budgets belong to semantic configuration and the analysis
//! session. If a budget is exceeded, the caller must mark analysis incomplete
//! or aborted rather than certify unresolved input as valid.
//!
//! No finite implementation can guarantee literally infinite memory or
//! execution. The requirement here is to avoid arbitrary language or hardware
//! ceilings.
//!
//! ## Error contract
//!
//! An unresolved or ambiguous name is a resolution outcome, not a panic.
//! Invalid scope trees and inconsistent qualified-resolver results are
//! infrastructure failures and cause the pass to return an error.
//!
//! The caller converts resolution outcomes into the repository's canonical
//! structured diagnostics, using the retained AST node identity and source
//! span. This module deliberately does not introduce a competing diagnostic
//! framework.
//!
//! ## Integration contract
//!
//! - `symbols/scopes.rs` owns `ScopeTree`, `ScopeId`, and `ancestor_chain()`.
//! - `symbols/table.rs` owns `SymbolTable`, `SymbolRecord`, and
//!   `SymbolNamespace`.
//! - `symbols/declarations.rs` collects declarations and reference inputs.
//! - `symbols/imports.rs` supplies qualified-path resolution.
//! - `symbols/cycles.rs` checks dependency cycles separately.
//! - the semantic session owns this pass's invocation and diagnostics.
//! - the semantic model records the final resolved declaration identities.
//!
//! No second symbol table, scope tree, AST identity or source-span type may be
//! introduced here.

use core::fmt;
use std::error::Error;

use crate::frontend::ast::node::{NodeId, Span};

use super::scopes::{ScopeError, ScopeId, ScopeTree};
use super::table::{SymbolNamespace, SymbolRecord, SymbolTable};

/// A source-level reference that must be resolved.
///
/// The reference is created by an AST-aware collection pass. `segments`
/// contains the already-parsed path components in source order.
///
/// An empty path or an empty path segment is rejected. Other spelling and
/// normalization rules remain the responsibility of the language and parser.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameReference {
    node_id: NodeId,
    span: Span,
    scope: ScopeId,
    namespace: SymbolNamespace,
    segments: Vec<String>,
}

impl NameReference {
    /// Construct a name reference from canonical AST identity, source span,
    /// lexical scope, namespace and parsed path segments.
    ///
    /// This constructor checks structural validity only. It does not attempt
    /// to validate source-language identifier syntax.
    pub fn new(
        node_id: NodeId,
        span: Span,
        scope: ScopeId,
        namespace: SymbolNamespace,
        segments: Vec<String>,
    ) -> Result<Self, NameReferenceError> {
        if segments.is_empty() {
            return Err(NameReferenceError::EmptyPath);
        }

        for (index, segment) in segments.iter().enumerate() {
            if segment.trim().is_empty() {
                return Err(NameReferenceError::EmptySegment { index });
            }
        }

        Ok(Self {
            node_id,
            span,
            scope,
            namespace,
            segments,
        })
    }

    /// Return the canonical AST identity of the reference.
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Return the canonical source span of the reference.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Return the scope from which the reference is resolved.
    #[must_use]
    pub const fn scope(&self) -> ScopeId {
        self.scope
    }

    /// Return the namespace in which the reference must be resolved.
    #[must_use]
    pub const fn namespace(&self) -> &SymbolNamespace {
        &self.namespace
    }

    /// Return the parsed path segments in source order.
    #[must_use]
    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    /// Return the final path segment.
    ///
    /// This is the complete name for a single-segment reference and the
    /// terminal member name for a qualified path.
    #[must_use]
    pub fn terminal_name(&self) -> &str {
        // Construction guarantees that the path contains at least one segment.
        // Using last() keeps this accessor safe if the representation changes.
        self.segments
            .last()
            .map(String::as_str)
            .unwrap_or_default()
    }

    /// Return whether this reference contains more than one path segment.
    #[must_use]
    pub fn is_qualified(&self) -> bool {
        self.segments.len() > 1
    }
}

/// Errors detected when constructing a [`NameReference`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum NameReferenceError {
    /// The path contained no segments.
    EmptyPath,

    /// A path segment was empty or whitespace-only.
    EmptySegment {
        /// Zero-based index of the invalid segment.
        index: usize,
    },
}

impl fmt::Display for NameReferenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPath => formatter.write_str("name reference path is empty"),
            Self::EmptySegment { index } => {
                write!(formatter, "name reference contains an empty segment at index {index}")
            }
        }
    }
}

impl Error for NameReferenceError {}

/// The outcome of resolving a qualified path.
///
/// The module/import subsystem decides which outcome applies, including
/// visibility and ambiguity rules. This enum does not decide whether a
/// declaration is type-correct or executable on a particular target.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum QualifiedLookup<Id> {
    /// The qualified path identifies one declaration.
    Resolved(Id),

    /// No declaration matches the path.
    NotFound,

    /// The path identifies more than one eligible declaration.
    Ambiguous,

    /// A declaration exists but is not accessible from this reference.
    Inaccessible,
}

/// Resolves qualified names using the canonical symbol and scope structures.
///
/// Implementations belong to the module/import subsystem. They may use module
/// exports, imports, aliases, visibility rules and module dependency
/// information, but must not create a competing symbol table or scope tree.
///
/// The resolver returns a declaration identity, not a new symbol record.
/// The caller verifies that the identity exists in the canonical table.
pub trait QualifiedPathResolver<Id, T>
where
    Id: Ord + Clone,
{
    /// Resolve a path containing multiple segments.
    ///
    /// `scope_chain` is ordered from the reference scope to the root.
    /// `reference` retains the canonical AST identity and source span.
    ///
    /// Implementations must be deterministic for equivalent inputs and
    /// equivalent module/import state.
    fn resolve_qualified(
        &self,
        reference: &NameReference,
        scope_chain: &[ScopeId],
        scopes: &ScopeTree,
        symbols: &SymbolTable<Id, ScopeId, T>,
    ) -> QualifiedLookup<Id>;
}

/// The status of one name-resolution attempt.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ResolutionStatus<Id> {
    /// The reference resolved to a registered declaration.
    Resolved {
        /// Stable semantic declaration identity.
        symbol_id: Id,

        /// Scope in which the declaration was registered.
        declaration_scope: ScopeId,
    },

    /// No eligible declaration matched the reference.
    Unresolved,

    /// More than one eligible declaration matched the qualified path.
    Ambiguous,

    /// The declaration exists but is inaccessible from this reference.
    Inaccessible,

    /// The reference is qualified but no qualified-path resolver was supplied.
    QualifiedResolverUnavailable,
}

/// A resolution result associated with its original source reference.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolutionRecord<Id> {
    reference: NameReference,
    status: ResolutionStatus<Id>,
}

impl<Id> ResolutionRecord<Id> {
    /// Construct a result record.
    ///
    /// This constructor is public to support controlled integration and tests.
    /// Normal production results should be produced by [`ResolutionEngine`].
    #[must_use]
    pub const fn new(
        reference: NameReference,
        status: ResolutionStatus<Id>,
    ) -> Self {
        Self { reference, status }
    }

    /// Return the original reference and its source information.
    #[must_use]
    pub const fn reference(&self) -> &NameReference {
        &self.reference
    }

    /// Return the resolution status.
    #[must_use]
    pub const fn status(&self) -> &ResolutionStatus<Id> {
        &self.status
    }

    /// Consume this result and return its original reference.
    #[must_use]
    pub fn into_reference(self) -> NameReference {
        self.reference
    }

    /// Consume this result and return its resolution status.
    #[must_use]
    pub fn into_status(self) -> ResolutionStatus<Id> {
        self.status
    }

    /// Return whether this reference resolved successfully.
    #[must_use]
    pub fn is_resolved(&self) -> bool {
        matches!(self.status, ResolutionStatus::Resolved { .. })
    }
}

/// Infrastructure failures that prevent a complete resolution pass.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ResolutionError {
    /// The scope tree could not provide a valid ancestor chain.
    InvalidScopeChain {
        /// AST node whose reference triggered the failure.
        reference: NodeId,

        /// Explanation supplied by the scope subsystem.
        message: String,
    },

    /// A qualified resolver returned an identity absent from the symbol table.
    ///
    /// This indicates inconsistent module/import state or an implementation
    /// defect. It must not be converted into an ordinary unresolved-name
    /// diagnostic.
    UnknownResolvedSymbol {
        /// AST node whose reference triggered the failure.
        reference: NodeId,
    },
}

impl fmt::Display for ResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidScopeChain { reference, message } => {
                write!(
                    formatter,
                    "cannot resolve reference at AST node {reference:?}: invalid scope chain: {message}"
                )
            }
            Self::UnknownResolvedSymbol { reference } => {
                write!(
                    formatter,
                    "qualified resolver returned an unregistered declaration for AST node {reference:?}"
                )
            }
        }
    }
}

impl Error for ResolutionError {}

impl From<ScopeError> for ResolutionError {
    fn from(error: ScopeError) -> Self {
        Self::InvalidScopeChain {
            reference: NodeId::first(),
            message: error.to_string(),
        }
    }
}

/// Stateless name-resolution engine.
///
/// The engine borrows the authoritative scope tree and symbol table supplied
/// for one analysis. It does not retain mutable state between invocations.
#[derive(Clone, Copy, Debug, Default)]
pub struct ResolutionEngine;

impl ResolutionEngine {
    /// Create a resolution engine.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Resolve a collection of references against the canonical scope tree
    /// and symbol table.
    ///
    /// Single-segment names use lexical lookup. Qualified paths require the
    /// module/import subsystem to supply a [`QualifiedPathResolver`].
    ///
    /// All declarations relevant to the analyzed source unit must already
    /// have been collected. An invalid scope chain or inconsistent qualified
    /// resolver aborts the pass with an infrastructure error.
    ///
    /// The returned records are deterministically ordered. Unresolved,
    /// ambiguous and inaccessible references remain explicit outcomes so the
    /// caller can emit structured diagnostics without confusing an incomplete
    /// analysis with a successful one.
    pub fn resolve_all<Id, T, I>(
        &self,
        references: I,
        scopes: &ScopeTree,
        symbols: &SymbolTable<Id, ScopeId, T>,
        qualified_resolver: Option<&dyn QualifiedPathResolver<Id, T>>,
    ) -> Result<Vec<ResolutionRecord<Id>>, ResolutionError>
    where
        Id: Ord + Clone,
        I: IntoIterator<Item = NameReference>,
    {
        let mut results = Vec::new();

        for reference in references {
            let scope_chain = scopes
                .ancestor_chain(reference.scope())
                .map_err(|error| ResolutionError::InvalidScopeChain {
                    reference: reference.node_id(),
                    message: error.to_string(),
                })?;

            let status = if reference.is_qualified() {
                self.resolve_qualified(
                    &reference,
                    &scope_chain,
                    scopes,
                    symbols,
                    qualified_resolver,
                )?
            } else {
                self.resolve_lexical(&reference, &scope_chain, symbols)
            };

            results.push(ResolutionRecord::new(reference, status));
        }

        // Use stable semantic attributes rather than input iteration order.
        // A stable sort preserves the order of truly identical references,
        // whose records are semantically interchangeable.
        results.sort_by(|left, right| {
            left.reference
                .node_id()
                .cmp(&right.reference.node_id())
                .then_with(|| left.reference.scope().get().cmp(&right.reference.scope().get()))
                .then_with(|| {
                    left.reference
                        .namespace()
                        .as_str()
                        .cmp(right.reference.namespace().as_str())
                })
                .then_with(|| {
                    left.reference
                        .segments()
                        .cmp(right.reference.segments())
                })
        });

        Ok(results)
    }

    fn resolve_lexical<Id, T>(
        &self,
        reference: &NameReference,
        scope_chain: &[ScopeId],
        symbols: &SymbolTable<Id, ScopeId, T>,
    ) -> ResolutionStatus<Id>
    where
        Id: Ord + Clone,
    {
        let Some(name) = reference.segments().first() else {
            // NameReference's constructor prevents this state. Keep the
            // resolver total even if the representation changes later.
            return ResolutionStatus::Unresolved;
        };

        match symbols.lookup_in_scopes(
            scope_chain.iter(),
            reference.namespace(),
            name,
        ) {
            Some(record) => ResolutionStatus::Resolved {
                symbol_id: record.id().clone(),
                declaration_scope: *record.scope(),
            },
            None => ResolutionStatus::Unresolved,
        }
    }

    fn resolve_qualified<Id, T>(
        &self,
        reference: &NameReference,
        scope_chain: &[ScopeId],
        scopes: &ScopeTree,
        symbols: &SymbolTable<Id, ScopeId, T>,
        resolver: Option<&dyn QualifiedPathResolver<Id, T>>,
    ) -> Result<ResolutionStatus<Id>, ResolutionError>
    where
        Id: Ord + Clone,
    {
        let Some(resolver) = resolver else {
            return Ok(ResolutionStatus::QualifiedResolverUnavailable);
        };

        let lookup = resolver.resolve_qualified(
            reference,
            scope_chain,
            scopes,
            symbols,
        );

        match lookup {
            QualifiedLookup::Resolved(symbol_id) => {
                let Some(record) = symbols.get(&symbol_id) else {
                    return Err(ResolutionError::UnknownResolvedSymbol {
                        reference: reference.node_id(),
                    });
                };

                Ok(ResolutionStatus::Resolved {
                    symbol_id,
                    declaration_scope: *record.scope(),
                })
            }
            QualifiedLookup::NotFound => Ok(ResolutionStatus::Unresolved),
            QualifiedLookup::Ambiguous => Ok(ResolutionStatus::Ambiguous),
            QualifiedLookup::Inaccessible => Ok(ResolutionStatus::Inaccessible),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::source::SourceId;
    use super::super::scopes::ScopeTree;
    use super::super::table::{SymbolRecord, SymbolTable};

    fn namespace() -> SymbolNamespace {
        SymbolNamespace::new("value").expect("valid namespace")
    }

    fn reference(
        node_id: NodeId,
        scope: ScopeId,
        name: &str,
    ) -> NameReference {
        NameReference::new(
            node_id,
            Span::from_usize(SourceId::from_raw(1), 0, name.len())
                .expect("valid source span"),
            scope,
            namespace(),
            vec![name.to_owned()],
        )
        .expect("valid name reference")
    }

    #[test]
    fn lexical_lookup_finds_a_root_declaration() {
        let scopes = ScopeTree::new();
        let mut symbols: SymbolTable<u64, ScopeId, ()> = SymbolTable::new();

        symbols
            .insert(
                SymbolRecord::new(
                    10,
                    ScopeId::root(),
                    namespace(),
                    "value",
                    (),
                )
                .expect("valid symbol"),
            )
            .expect("insert symbol");

        let input = reference(NodeId::first(), ScopeId::root(), "value");
        let results = ResolutionEngine::new()
            .resolve_all([input], &scopes, &symbols, None)
            .expect("resolution succeeds");

        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].status(),
            &ResolutionStatus::Resolved {
                symbol_id: 10,
                declaration_scope: ScopeId::root(),
            }
        );
    }

    #[test]
    fn lexical_lookup_prefers_the_nearest_declaration() {
        let mut scopes = ScopeTree::new();
        let child = scopes
            .create_child(ScopeId::root(), super::super::scopes::ScopeKind::Lexical, None)
            .expect("create child scope");

        let mut symbols: SymbolTable<u64, ScopeId, ()> = SymbolTable::new();

        symbols
            .insert(
                SymbolRecord::new(
                    1,
                    ScopeId::root(),
                    namespace(),
                    "item",
                    (),
                )
                .expect("valid root symbol"),
            )
            .expect("insert root symbol");

        symbols
            .insert(
                SymbolRecord::new(
                    2,
                    child,
                    namespace(),
                    "item",
                    (),
                )
                .expect("valid child symbol"),
            )
            .expect("insert child symbol");

        let input = reference(NodeId::first(), child, "item");
        let results = ResolutionEngine::new()
            .resolve_all([input], &scopes, &symbols, None)
            .expect("resolution succeeds");

        assert_eq!(
            results[0].status(),
            &ResolutionStatus::Resolved {
                symbol_id: 2,
                declaration_scope: child,
            }
        );
    }

    #[test]
    fn unresolved_names_remain_explicit_outcomes() {
        let scopes = ScopeTree::new();
        let symbols: SymbolTable<u64, ScopeId, ()> = SymbolTable::new();

        let input = reference(NodeId::first(), ScopeId::root(), "missing");
        let results = ResolutionEngine::new()
            .resolve_all([input], &scopes, &symbols, None)
            .expect("resolution pass completes");

        assert_eq!(results[0].status(), &ResolutionStatus::Unresolved);
        assert!(!results[0].is_resolved());
    }

    #[test]
    fn qualified_paths_require_the_module_resolution_boundary() {
        let scopes = ScopeTree::new();
        let symbols: SymbolTable<u64, ScopeId, ()> = SymbolTable::new();

        let input = NameReference::new(
            NodeId::first(),
            Span::from_usize(SourceId::from_raw(1), 0, 9)
                .expect("valid span"),
            ScopeId::root(),
            namespace(),
            vec!["module".to_owned(), "member".to_owned()],
        )
        .expect("valid qualified reference");

        let results = ResolutionEngine::new()
            .resolve_all([input], &scopes, &symbols, None)
            .expect("resolution pass completes");

        assert_eq!(
            results[0].status(),
            &ResolutionStatus::QualifiedResolverUnavailable
        );
    }

    #[test]
    fn empty_paths_are_rejected() {
        let result = NameReference::new(
            NodeId::first(),
            Span::from_usize(SourceId::from_raw(1), 0, 0)
                .expect("valid span"),
            ScopeId::root(),
            namespace(),
            Vec::new(),
        );

        assert_eq!(result, Err(NameReferenceError::EmptyPath));
    }

    #[test]
    fn result_order_does_not_depend_on_input_order_for_distinct_nodes() {
        let scopes = ScopeTree::new();
        let mut symbols: SymbolTable<u64, ScopeId, ()> = SymbolTable::new();

        symbols
            .insert(
                SymbolRecord::new(
                    1,
                    ScopeId::root(),
                    namespace(),
                    "first",
                    (),
                )
                .expect("valid symbol"),
            )
            .expect("insert symbol");

        let first = reference(NodeId::first(), ScopeId::root(), "first");
        let second = reference(
            NodeId::new(2).expect("nonzero node ID"),
            ScopeId::root(),
            "unknown",
        );

        let results = ResolutionEngine::new()
            .resolve_all(
                [second, first],
                &scopes,
                &symbols,
                None,
            )
            .expect("resolution succeeds");

        assert_eq!(results[0].reference().node_id(), NodeId::first());
        assert_eq!(
            results[1].reference().node_id(),
            NodeId::new(2).expect("nonzero node ID")
        );
    }
}
