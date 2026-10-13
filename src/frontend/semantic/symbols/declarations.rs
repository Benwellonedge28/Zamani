
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Analysis — Declaration Collection
//!
//! Path: `src/frontend/semantic/symbols/declarations.rs`
//! Edition: Rust 2021
//! Minimum toolchain: Rust 1.97
//! Safety: safe Rust only.
//!
//! ## PURPOSE
//!
//! Collect canonical source declarations into the existing semantic symbol
//! table before name resolution, type inference, and domain validation.
//!
//! This module provides a domain-neutral declaration collection boundary.
//! The AST adapter translates concrete AST declaration variants into
//! `DeclarationSpec` values; this module validates collection invariants and
//! registers their semantic identities and metadata.
//!
//! ## STATUS
//!
//! Implementation candidate. It must pass the integration checks documented
//! below against the exact versions of `table.rs`, `scopes.rs`, the canonical
//! AST, and the semantic module tree before being declared frozen.
//!
//! ## AUTHORITY
//!
//! - Canonical source declarations:
//!   `src/frontend/ast/node/declarations/`
//! - Canonical AST identities and spans:
//!   `src/frontend/ast/node/`
//! - Symbol storage:
//!   `src/frontend/semantic/symbols/table.rs`
//! - Scope ownership:
//!   `src/frontend/semantic/symbols/scopes.rs`
//! - Import/export registration:
//!   `src/frontend/semantic/symbols/imports.rs`
//! - Name resolution:
//!   `src/frontend/semantic/symbols/resolution.rs`
//! - Source syntax:
//!   `grammar/modules/` and the normative language specification.
//!
//! ## OWNS
//!
//! - The adapter-facing declaration input contract.
//! - Extensible declaration-kind metadata.
//! - Source visibility metadata without inventing visibility defaults.
//! - Declaration registration against the canonical symbol table.
//! - Duplicate AST declaration identity detection within a collection run.
//! - Deterministic per-declaration collection outcomes.
//! - Collection errors that preserve source identity and location.
//!
//! ## DOES NOT OWN
//!
//! - AST node definitions, traversal, or AST identity allocation.
//! - Symbol table indexes or lexical scope storage.
//! - Name resolution, import resolution, or export expansion.
//! - Type inference, type compatibility, effects, capabilities, or resources.
//! - The language's normative visibility rules.
//! - Hardware discovery, allocation, routing, scheduling, or IR generation.
//! - A second semantic diagnostic framework.
//!
//! ## INTEGRATION CONTRACT
//!
//! 1. The AST adapter walks the canonical AST and constructs a
//!    `DeclarationSpec` for each declaration supported by the language.
//! 2. The semantic session creates one collector per declaration-collection
//!    pass and supplies the existing `ScopeTree` and `SymbolTable`.
//! 3. The caller supplies semantic declaration IDs. AST `NodeId` values are
//!    provenance and are never silently reused as semantic IDs.
//! 4. Every supplied scope must already exist in the canonical `ScopeTree`.
//! 5. Successful records are inserted into the existing `SymbolTable`.
//! 6. Failed records produce collection issues; they are never counted as
//!    successfully registered declarations.
//! 7. Name resolution runs only after declaration collection has finished.
//! 8. A report with issues is not proof of a complete, valid symbol table.
//! 9. Imports and exports remain owned by `imports.rs` and the relevant
//!    adapter; this file does not create a parallel export registry.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! There is no language-level maximum declaration count, scope count,
//! domain count, machine size, qubit count, or resource capacity here.
//!
//! Collection grows with the supplied declarations and available memory.
//! The standard library and the existing symbol table still have finite
//! representational and allocation limits. No implementation can promise
//! literal infinite memory or execution.
//!
//! Operational budgets must be supplied by the semantic session and must
//! cause explicit incomplete/aborted analysis rather than false success.
//!
//! ## DETERMINISM
//!
//! Results preserve input order. The underlying symbol table uses its
//! deterministic ordering contract for indexed lookup. This module does not
//! use hash-map iteration, randomness, wall-clock time, global mutable state,
//! filesystem access, network access, or hardware discovery.
//!
//! ## ERROR POLICY
//!
//! Invalid names, duplicate AST declaration identities, unknown scopes,
//! duplicate symbol IDs, and duplicate scoped names are reported as
//! collection issues. Collection continues for independent declarations.
//! The caller must not certify the program as semantically valid when issues
//! remain or when the analysis session was otherwise incomplete.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::frontend::ast::node::{NodeId, Span};

use super::scopes::{ScopeId, ScopeTree};
use super::table::{
    SymbolNamespace,
    SymbolRecord,
    SymbolTable,
    SymbolTableError,
};

/// An extensible source declaration category.
///
/// The category is metadata, not a closed enumeration of Zamani constructs.
/// New language or computational-domain extensions can introduce new
/// namespace/name pairs without modifying this type.
///
/// Examples include `core:function`, `core:type`, `core:constant`,
/// `quantum:operation`, `hdl:signal`, and extension-defined categories.
/// These examples are illustrative identifiers, not a registry of all
/// supported declaration kinds.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeclarationKind {
    namespace: String,
    name: String,
}

impl DeclarationKind {
    /// Construct a declaration kind from a namespace and category name.
    ///
    /// Both components must be nonempty and must not contain whitespace or
    /// control characters. The supplied spelling is otherwise preserved.
    ///
    /// Identifier normalization and the set of registered categories remain
    /// the responsibility of the language specification and extension
    /// registry.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, DeclarationMetadataError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_component(&namespace, "declaration namespace")?;
        validate_component(&name, "declaration kind")?;

        Ok(Self { namespace, name })
    }

    /// Return the exact declaration-kind namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the exact declaration-kind name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for DeclarationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.name)
    }
}

/// Source visibility information retained during declaration collection.
///
/// `Unspecified` is intentional: the collector must not guess a language
/// default. The normative visibility rules determine how unspecified
/// visibility is interpreted in a particular declaration context.
///
/// `Restricted` preserves an opaque source-level restriction for a later
/// visibility checker. It does not itself authorize access.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum DeclarationVisibility {
    /// No explicit visibility information was supplied by the adapter.
    Unspecified,

    /// The source declaration explicitly specifies public visibility.
    Public,

    /// The source declaration explicitly specifies private visibility.
    Private,

    /// The source declaration explicitly specifies protected visibility.
    Protected,

    /// A language-defined or extension-defined visibility restriction.
    ///
    /// The string is retained without interpreting its semantics here.
    Restricted(String),
}

impl DeclarationVisibility {
    /// Construct a restricted visibility marker.
    ///
    /// The restriction must be nonempty. Its syntax and meaning are validated
    /// by the adapter and the language-specific visibility checker.
    pub fn restricted(
        restriction: impl Into<String>,
    ) -> Result<Self, DeclarationMetadataError> {
        let restriction = restriction.into();

        if restriction.trim().is_empty() {
            return Err(DeclarationMetadataError::EmptyVisibilityRestriction);
        }

        Ok(Self::Restricted(restriction))
    }
}

/// Metadata associated with a successfully collected declaration.
///
/// This payload is stored inside the existing `SymbolTable`; it is not a
/// second symbol registry. Symbol identity, scope, namespace, and lookup name
/// remain owned by `SymbolRecord`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclarationMetadata {
    node_id: NodeId,
    span: Span,
    kind: DeclarationKind,
    visibility: DeclarationVisibility,
}

impl DeclarationMetadata {
    /// Construct metadata for one canonical AST declaration.
    #[must_use]
    pub fn new(
        node_id: NodeId,
        span: Span,
        kind: DeclarationKind,
        visibility: DeclarationVisibility,
    ) -> Self {
        Self {
            node_id,
            span,
            kind,
            visibility,
        }
    }

    /// Return the canonical AST node identity for this declaration.
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Return the original source span.
    #[must_use]
    pub const fn span(&self) -> &Span {
        &self.span
    }

    /// Return the extensible declaration kind.
    #[must_use]
    pub const fn kind(&self) -> &DeclarationKind {
        &self.kind
    }

    /// Return the source visibility information as collected.
    #[must_use]
    pub const fn visibility(&self) -> &DeclarationVisibility {
        &self.visibility
    }
}

/// Adapter-facing input for one declaration.
///
/// The adapter supplies a semantic declaration ID and the canonical AST
/// node identity separately. This preserves the distinction between syntax
/// identity and semantic declaration identity.
///
/// The name and namespace are the exact source-level spelling after the
/// parser's identifier processing. This type does not normalize identifiers,
/// apply case folding, or decide language-specific name equivalence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclarationSpec<Id> {
    id: Id,
    node_id: NodeId,
    span: Span,
    scope: ScopeId,
    namespace: SymbolNamespace,
    name: String,
    kind: DeclarationKind,
    visibility: DeclarationVisibility,
}

impl<Id> DeclarationSpec<Id> {
    /// Construct a declaration input record.
    ///
    /// This validates only the fields owned by the collection boundary.
    /// Declaration-specific syntax and semantic validity remain the
    /// responsibility of the canonical AST validator and semantic passes.
    pub fn new(
        id: Id,
        node_id: NodeId,
        span: Span,
        scope: ScopeId,
        namespace: SymbolNamespace,
        name: impl Into<String>,
        kind: DeclarationKind,
        visibility: DeclarationVisibility,
    ) -> Result<Self, DeclarationMetadataError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(DeclarationMetadataError::EmptyDeclarationName);
        }

        if name.chars().any(char::is_control) {
            return Err(DeclarationMetadataError::ControlCharacterInName);
        }

        Ok(Self {
            id,
            node_id,
            span,
            scope,
            namespace,
            name,
            kind,
            visibility,
        })
    }

    /// Return the semantic declaration ID supplied by the caller.
    #[must_use]
    pub const fn id(&self) -> &Id {
        &self.id
    }

    /// Return the canonical AST node identity.
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Return the source span.
    #[must_use]
    pub const fn span(&self) -> &Span {
        &self.span
    }

    /// Return the declaring scope.
    #[must_use]
    pub const fn scope(&self) -> ScopeId {
        self.scope
    }

    /// Return the symbol namespace.
    #[must_use]
    pub const fn namespace(&self) -> &SymbolNamespace {
        &self.namespace
    }

    /// Return the exact declaration name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the declaration kind.
    #[must_use]
    pub const fn kind(&self) -> &DeclarationKind {
        &self.kind
    }

    /// Return the collected visibility marker.
    #[must_use]
    pub const fn visibility(&self) -> &DeclarationVisibility {
        &self.visibility
    }
}

/// Errors detected while constructing declaration metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeclarationMetadataError {
    /// A required string component was empty.
    EmptyComponent {
        /// Name of the invalid component.
        component: &'static str,
    },

    /// A component contained whitespace or control characters.
    InvalidComponent {
        /// Name of the invalid component.
        component: &'static str,
    },

    /// A declaration name was empty or whitespace-only.
    EmptyDeclarationName,

    /// A declaration name contained a control character.
    ControlCharacterInName,

    /// A restricted visibility marker was empty.
    EmptyVisibilityRestriction,
}

impl fmt::Display for DeclarationMetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyComponent { component } => {
                write!(f, "{component} must not be empty")
            }
            Self::InvalidComponent { component } => {
                write!(
                    f,
                    "{component} must not contain whitespace or control characters"
                )
            }
            Self::EmptyDeclarationName => {
                f.write_str("declaration name must not be empty")
            }
            Self::ControlCharacterInName => {
                f.write_str("declaration name contains a control character")
            }
            Self::EmptyVisibilityRestriction => {
                f.write_str("visibility restriction must not be empty")
            }
        }
    }
}

impl Error for DeclarationMetadataError {}

fn validate_component(
    value: &str,
    component: &'static str,
) -> Result<(), DeclarationMetadataError> {
    if value.trim().is_empty() {
        return Err(DeclarationMetadataError::EmptyComponent { component });
    }

    if value.chars().any(char::is_whitespace)
        || value.chars().any(char::is_control)
    {
        return Err(DeclarationMetadataError::InvalidComponent { component });
    }

    Ok(())
}

/// The reason a declaration could not be collected.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeclarationCollectionError {
    /// The AST node identity was already encountered by this collector.
    DuplicateAstNode {
        /// Repeated AST node identity.
        node_id: NodeId,
    },

    /// The supplied scope does not exist in the canonical scope tree.
    UnknownScope {
        /// The declaration's AST node identity.
        node_id: NodeId,

        /// The unknown scope identity.
        scope: ScopeId,
    },

    /// The canonical symbol table rejected the declaration.
    SymbolTable {
        /// The declaration's AST node identity.
        node_id: NodeId,

        /// The table's specific rejection reason.
        source: SymbolTableError,
    },
}

impl fmt::Display for DeclarationCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateAstNode { node_id } => {
                write!(
                    f,
                    "AST declaration node {node_id:?} was collected more than once"
                )
            }
            Self::UnknownScope { node_id, scope } => {
                write!(
                    f,
                    "declaration node {node_id:?} refers to unknown scope {scope}"
                )
            }
            Self::SymbolTable { node_id, source } => {
                write!(
                    f,
                    "could not register declaration node {node_id:?}: {source}"
                )
            }
        }
    }
}

impl Error for DeclarationCollectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SymbolTable { source, .. } => Some(source),
            Self::DuplicateAstNode { .. } | Self::UnknownScope { .. } => None,
        }
    }
}

/// Outcome for one declaration input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeclarationCollectionOutcome<Id> {
    /// The declaration was registered in the canonical symbol table.
    Registered {
        /// Semantic declaration identity.
        id: Id,

        /// Canonical AST node identity.
        node_id: NodeId,
    },

    /// The declaration could not be registered.
    Rejected {
        /// Canonical AST node identity.
        node_id: NodeId,

        /// Collection failure.
        error: DeclarationCollectionError,
    },
}

/// Summary of one collection batch.
///
/// A report is complete only with respect to the supplied batch: callers must
/// still account for AST traversal failures, parser errors, configured budget
/// exhaustion, and later semantic validation failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeclarationCollectionReport<Id> {
    outcomes: Vec<DeclarationCollectionOutcome<Id>>,
    registered: usize,
    rejected: usize,
}

impl<Id> DeclarationCollectionReport<Id> {
    fn new() -> Self {
        Self {
            outcomes: Vec::new(),
            registered: 0,
            rejected: 0,
        }
    }

    /// Return per-declaration outcomes in input order.
    #[must_use]
    pub fn outcomes(&self) -> &[DeclarationCollectionOutcome<Id>] {
        &self.outcomes
    }

    /// Return the number of successfully registered declarations.
    #[must_use]
    pub const fn registered_count(&self) -> usize {
        self.registered
    }

    /// Return the number of rejected declarations.
    #[must_use]
    pub const fn rejected_count(&self) -> usize {
        self.rejected
    }

    /// Return whether every input in this batch was registered.
    ///
    /// This does not mean the entire program is semantically valid.
    #[must_use]
    pub const fn batch_succeeded(&self) -> bool {
        self.rejected == 0
    }

    /// Consume the report and return its ordered outcomes.
    #[must_use]
    pub fn into_outcomes(self) -> Vec<DeclarationCollectionOutcome<Id>> {
        self.outcomes
    }
}

/// Collects declarations into the existing symbol table.
///
/// Create one collector for a complete declaration-collection pass. The
/// collector tracks AST declaration identities already encountered during
/// that pass; it does not replace the symbol table's own identity and
/// scoped-name indexes.
///
/// `Id` is the semantic declaration identity type selected by the semantic
/// model. It must be ordered and cloneable because the canonical symbol table
/// uses ordered indexes.
#[derive(Clone, Debug)]
pub struct DeclarationCollector<Id> {
    seen_nodes: BTreeMap<NodeId, Id>,
}

impl<Id> Default for DeclarationCollector<Id> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Id> DeclarationCollector<Id>
where
    Id: Clone + Ord,
{
    /// Create a collector for one declaration-collection pass.
    #[must_use]
    pub fn new() -> Self {
        Self {
            seen_nodes: BTreeMap::new(),
        }
    }

    /// Return the number of distinct AST declaration nodes encountered by
    /// this collector, including nodes whose registration was rejected.
    #[must_use]
    pub fn encountered_count(&self) -> usize {
        self.seen_nodes.len()
    }

    /// Collect an ordered batch of declarations.
    ///
    /// Valid declarations are registered even if another declaration in the
    /// batch is rejected. The returned report identifies every outcome.
    /// The caller must not treat a partially successful report as proof that
    /// all declarations were collected or that the program is valid.
    ///
    /// Before calling this method:
    ///
    /// - AST structural validation should have run;
    /// - semantic IDs should have been allocated by the semantic ID owner;
    /// - scopes should have been created in `ScopeTree`;
    /// - names should already have been processed by the canonical parser
    ///   and language-specific identifier rules.
    pub fn collect<ScopePayload>(
        &mut self,
        declarations: impl IntoIterator<Item = DeclarationSpec<Id>>,
        scopes: &ScopeTree,
        symbols: &mut SymbolTable<Id, ScopeId, DeclarationMetadata>,
    ) -> DeclarationCollectionReport<Id> {
        let mut report = DeclarationCollectionReport::new();

        for declaration in declarations {
            let node_id = declaration.node_id;

            if self.seen_nodes.contains_key(&node_id) {
                report.rejected += 1;
                report.outcomes.push(
                    DeclarationCollectionOutcome::Rejected {
                        node_id,
                        error: DeclarationCollectionError::DuplicateAstNode {
                            node_id,
                        },
                    },
                );
                continue;
            }

            // Record that this AST node has been encountered even if a later
            // validation step rejects it. A malformed declaration must not
            // be silently retried as if it were a different AST node.
            self.seen_nodes
                .insert(node_id, declaration.id.clone());

            if scopes.get(declaration.scope).is_none() {
                report.rejected += 1;
                report.outcomes.push(
                    DeclarationCollectionOutcome::Rejected {
                        node_id,
                        error: DeclarationCollectionError::UnknownScope {
                            node_id,
                            scope: declaration.scope,
                        },
                    },
                );
                continue;
            }

            let metadata = DeclarationMetadata::new(
                declaration.node_id,
                declaration.span,
                declaration.kind,
                declaration.visibility,
            );

            let record = match SymbolRecord::new(
                declaration.id.clone(),
                declaration.scope,
                declaration.namespace,
                declaration.name,
                metadata,
            ) {
                Ok(record) => record,
                Err(source) => {
                    report.rejected += 1;
                    report.outcomes.push(
                        DeclarationCollectionOutcome::Rejected {
                            node_id,
                            error: DeclarationCollectionError::SymbolTable {
                                node_id,
                                source,
                            },
                        },
                    );
                    continue;
                }
            };

            match symbols.insert(record) {
                Ok(()) => {
                    report.registered += 1;
                    report.outcomes.push(
                        DeclarationCollectionOutcome::Registered {
                            id: declaration.id,
                            node_id,
                        },
                    );
                }
                Err(source) => {
                    report.rejected += 1;
                    report.outcomes.push(
                        DeclarationCollectionOutcome::Rejected {
                            node_id,
                            error: DeclarationCollectionError::SymbolTable {
                                node_id,
                                source,
                            },
                        },
                    );
                }
            }
        }

        report
    }
}
