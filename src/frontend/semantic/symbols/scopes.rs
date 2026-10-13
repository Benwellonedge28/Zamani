
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Analysis — Scope Management
//!
//! Path: `src/frontend/semantic/symbols/scopes.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Own the semantic scope tree and the active lexical-scope stack used by
//! semantic analysis. Scopes describe the regions in which declarations can
//! be visible. Declaration storage and name lookup belong to `table.rs` and
//! `resolution.rs`, respectively.
//!
//! ## Architectural position
//!
//! ```text
//! Canonical AST (`frontend::ast::node`)
//!                 |
//!                 v
//!       Declaration collection
//!                 |
//!                 v
//!          Scope management  <---- this module
//!                 |
//!                 +--------------------+
//!                 |                    |
//!                 v                    v
//!         Symbol table          Name resolution
//!                 |                    |
//!                 +---------+----------+
//!                           |
//!                           v
//!                 Semantic facts/model
//!                           |
//!                           v
//!               Canonical domain IRs
//! ```
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - `ScopeId` and its checked allocation;
//! - `ScopeKind`;
//! - the immutable identity and parent/child structure of recorded scopes;
//! - the active-scope stack for a single analysis context;
//! - scope ancestry, descendant, and integrity queries;
//! - errors for invalid scope operations and allocation failures.
//!
//! It does not own:
//!
//! - AST nodes, AST identities, or source spans;
//! - declarations, symbols, symbol tables, or name resolution;
//! - type inference, generic substitution, effects, or contracts;
//! - target capabilities, hardware allocation, scheduling, or lowering;
//! - parser grammar or language syntax.
//!
//! ## Canonical AST integration
//!
//! `NodeId` is imported from the canonical frontend AST. A scope may refer to
//! the AST node that introduces it, but this module never creates or replaces
//! AST identities. Synthetic scopes may have no owner node.
//!
//! ## Scope semantics
//!
//! - A tree starts with one root scope.
//! - Every non-root scope has exactly one parent.
//! - A scope's parent and kind never change after creation.
//! - Leaving an active scope removes it from the active stack but does not
//!   delete it from the scope tree.
//! - Shadowing is represented by declarations in descendant scopes. This
//!   module does not decide whether a particular declaration may shadow
//!   another; that rule belongs to declaration/name-resolution semantics.
//! - Forward declarations are supported by collecting declarations into the
//!   appropriate scopes before resolving references.
//! - Scope identifiers are stable for the lifetime of this tree.
//!
//! ## Scalability and POCO-REAF
//!
//! There is no language-defined maximum scope count, nesting depth, domain
//! count, qubit count, device count, or machine capacity here. Scope storage
//! grows with available resources. Identifier exhaustion, address-space
//! limits, and allocation failures are reported explicitly where detectable.
//!
//! An operational budget, if required, belongs to semantic configuration
//! and the analysis session; it must not change the meaning of a program.
//!
//! No finite implementation can promise literally infinite memory or
//! execution. This module imposes no additional arbitrary scope ceiling.
//!
//! ## Determinism
//!
//! Scope IDs are allocated monotonically from the root. Children preserve
//! creation order. No hash-map iteration or process-global mutable state is
//! involved. Equivalent scope-construction sequences produce equivalent IDs
//! and traversal order.
//!
//! ## Integration contract
//!
//! `symbols/table.rs` should key its per-scope declaration buckets by
//! `ScopeId`. `symbols/resolution.rs` should use `parent_of` or
//! `ancestor_chain` to search outward, consulting the symbol table at each
//! scope. Neither consumer should maintain a second parent/child tree.
//!
//! Declaration collection should use `create_child` when it needs to record
//! a scope without changing the active traversal position, and `enter_scope`
//! / `exit_scope` for structured lexical traversal.
//!
//! Before integration, `symbols/mod.rs` must declare this module and export
//! the intended public scope types. The semantic session should own one
//! `ScopeTree` per analysis invocation; independent analyses must not share
//! mutable scope state.

use core::fmt;
use core::num::TryFromIntError;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::error::Error;
use std::io;
use std::mem;

use crate::frontend::ast::node::NodeId;

/// Stable identity of a scope within one [`ScopeTree`].
///
/// Scope IDs are local to a tree, not globally unique across compilations.
/// The root has ID zero. IDs are never recycled during the lifetime of a
/// tree, including after a scope is exited.
///
/// Do not interpret the numeric value as source order, nesting depth, a
/// declaration ID, an AST `NodeId`, or a hardware resource identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScopeId(u64);

impl ScopeId {
    /// Returns the numeric representation for diagnostics or serialization.
    ///
    /// The returned value has no semantic meaning beyond identifying this
    /// scope within its originating tree.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// The root scope identifier.
    #[must_use]
    pub const fn root() -> Self {
        Self(0)
    }
}

impl fmt::Display for ScopeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "scope:{}", self.0)
    }
}

/// Semantic category of a scope.
///
/// The enum describes scope behavior at a broad structural level. It is not
/// a closed list of all Zamani declarations or computational domains.
/// `Domain` and `Extension` allow registered language extensions to express
/// their own scope categories without modifying this enum.
///
/// Whether a construct actually introduces a scope is determined by the
/// normative language rules and the AST construct being analyzed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ScopeKind {
    /// The single root scope of an analysis unit.
    Root,

    /// A module or source-unit scope.
    Module,

    /// A general lexical region, such as a block.
    Lexical,

    /// A function or callable-body scope.
    Function,

    /// A generic parameter scope.
    Generic,

    /// A type, record, interface, or other type-member scope.
    Type,

    /// An implementation or trait-implementation scope.
    Implementation,

    /// A closure or lambda scope.
    Closure,

    /// A loop scope.
    Loop,

    /// A branch-specific scope.
    Branch,

    /// An asynchronous body scope.
    Async,

    /// An actor or actor-like declaration scope.
    Actor,

    /// A registered computational-domain scope.
    Domain {
        /// Qualified domain identifier supplied by the language extension.
        name: String,
    },

    /// A scope introduced by a registered language extension.
    Extension {
        /// Namespace identifying the extension owner.
        namespace: String,

        /// Extension-defined scope category.
        name: String,
    },
}

impl ScopeKind {
    /// Returns a stable diagnostic-oriented category name.
    ///
    /// This is not a source-language keyword and must not be used to parse
    /// source text.
    #[must_use]
    pub const fn category_name(&self) -> &'static str {
        match self {
            Self::Root => "root",
            Self::Module => "module",
            Self::Lexical => "lexical",
            Self::Function => "function",
            Self::Generic => "generic",
            Self::Type => "type",
            Self::Implementation => "implementation",
            Self::Closure => "closure",
            Self::Loop => "loop",
            Self::Branch => "branch",
            Self::Async => "async",
            Self::Actor => "actor",
            Self::Domain { .. } => "domain",
            Self::Extension { .. } => "extension",
        }
    }

    /// Constructs a domain scope.
    ///
    /// The identifier must be non-empty after trimming. Its detailed syntax
    /// and registration are validated by the domain-extension subsystem.
    pub fn domain(name: impl Into<String>) -> Result<Self, ScopeError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ScopeError::EmptyScopeCategory);
        }

        Ok(Self::Domain { name })
    }

    /// Constructs an extension-defined scope category.
    ///
    /// Namespace and name must be non-empty after trimming. This method does
    /// not register or authorize the extension.
    pub fn extension(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, ScopeError> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.trim().is_empty() || name.trim().is_empty() {
            return Err(ScopeError::EmptyScopeCategory);
        }

        Ok(Self::Extension { namespace, name })
    }
}

/// Immutable structural information recorded for one scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeInfo {
    id: ScopeId,
    parent: Option<ScopeId>,
    kind: ScopeKind,
    owner: Option<NodeId>,
    children: Vec<ScopeId>,
    depth: usize,
}

impl ScopeInfo {
    /// Returns this scope's identity.
    #[must_use]
    pub const fn id(&self) -> ScopeId {
        self.id
    }

    /// Returns the parent scope, or `None` for the root.
    #[must_use]
    pub const fn parent(&self) -> Option<ScopeId> {
        self.parent
    }

    /// Returns the semantic kind of this scope.
    #[must_use]
    pub const fn kind(&self) -> &ScopeKind {
        &self.kind
    }

    /// Returns the AST node that introduced this scope, if any.
    #[must_use]
    pub const fn owner(&self) -> Option<NodeId> {
        self.owner
    }

    /// Returns child scope IDs in deterministic creation order.
    #[must_use]
    pub fn children(&self) -> &[ScopeId] {
        &self.children
    }

    /// Returns the number of parent edges between this scope and the root.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    /// Returns `true` if this is the root scope.
    #[must_use]
    pub const fn is_root(&self) -> bool {
        self.parent.is_none()
    }
}

/// Failures from scope-tree construction, navigation, or validation.
///
/// Allocation failures are surfaced instead of being converted into
/// successful semantic results. Standard-library allocation reporting is
/// best-effort: an allocator abort cannot be recovered from by Rust code.
#[derive(Debug)]
#[non_exhaustive]
pub enum ScopeError {
    /// A requested parent or scope ID does not belong to this tree.
    UnknownScope {
        /// ID supplied by the caller.
        scope: ScopeId,
    },

    /// Attempted to leave the root scope.
    CannotExitRoot,

    /// Attempted to create a second root or used `Root` for a child scope.
    InvalidRootOperation,

    /// Scope ID space or the platform's index representation was exhausted.
    IdentifierExhausted,

    /// A collection could not reserve the required additional capacity.
    AllocationFailed {
        /// Collection being grown.
        collection: &'static str,
    },

    /// A domain or extension scope was supplied an empty category.
    EmptyScopeCategory,

    /// The supplied scope tree failed an integrity check.
    InvalidTree {
        /// Human-readable invariant violation.
        reason: String,
    },

    /// A platform integer conversion failed.
    IntegerConversion(TryFromIntError),
}

impl fmt::Display for ScopeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownScope { scope } => {
                write!(formatter, "unknown scope {scope}")
            }
            Self::CannotExitRoot => {
                write!(formatter, "cannot exit the root scope")
            }
            Self::InvalidRootOperation => {
                write!(
                    formatter,
                    "invalid root-scope operation: the root is unique and cannot be nested"
                )
            }
            Self::IdentifierExhausted => {
                write!(formatter, "scope identifier space exhausted")
            }
            Self::AllocationFailed { collection } => {
                write!(formatter, "unable to grow {collection} for scope analysis")
            }
            Self::EmptyScopeCategory => {
                write!(formatter, "scope category must not be empty")
            }
            Self::InvalidTree { reason } => {
                write!(formatter, "invalid scope tree: {reason}")
            }
            Self::IntegerConversion(error) => {
                write!(formatter, "scope integer conversion failed: {error}")
            }
        }
    }
}

impl Error for ScopeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IntegerConversion(error) => Some(error),
            _ => None,
        }
    }
}

impl From<TryFromIntError> for ScopeError {
    fn from(error: TryFromIntError) -> Self {
        Self::IntegerConversion(error)
    }
}

/// A single-analysis scope tree.
///
/// The tree is the sole owner of scope parent/child relationships. The symbol
/// table should maintain declaration buckets keyed by `ScopeId`, but must not
/// maintain a competing scope hierarchy.
///
/// The active stack represents the current traversal position. The recorded
/// tree persists after `exit_scope`, allowing later passes to inspect the
/// complete scope structure.
#[derive(Debug, Clone)]
pub struct ScopeTree {
    scopes: Vec<ScopeInfo>,
    active: Vec<ScopeId>,
}

impl ScopeTree {
    /// Creates a scope tree with a root scope.
    ///
    /// This convenience constructor follows ordinary Rust collection
    /// allocation behavior. Use [`ScopeTree::try_new`] when initialization
    /// must report recoverable capacity-reservation failures.
    #[must_use]
    pub fn new() -> Self {
        let root = ScopeInfo {
            id: ScopeId::root(),
            parent: None,
            kind: ScopeKind::Root,
            owner: None,
            children: Vec::new(),
            depth: 0,
        };

        Self {
            scopes: vec![root],
            active: vec![ScopeId::root()],
        }
    }

    /// Fallibly creates a scope tree with a root scope.
    ///
    /// This method reports allocation failures detectable through
    /// `try_reserve`. It cannot recover from process-level allocator aborts.
    pub fn try_new() -> Result<Self, ScopeError> {
        let mut scopes = Vec::new();
        scopes
            .try_reserve(1)
            .map_err(|_| ScopeError::AllocationFailed {
                collection: "scope storage",
            })?;

        let mut active = Vec::new();
        active
            .try_reserve(1)
            .map_err(|_| ScopeError::AllocationFailed {
                collection: "active scope stack",
            })?;

        scopes.push(ScopeInfo {
            id: ScopeId::root(),
            parent: None,
            kind: ScopeKind::Root,
            owner: None,
            children: Vec::new(),
            depth: 0,
        });
        active.push(ScopeId::root());

        Ok(Self { scopes, active })
    }

    /// Returns the root scope ID.
    #[must_use]
    pub const fn root_id(&self) -> ScopeId {
        ScopeId::root()
    }

    /// Returns the currently active scope.
    ///
    /// A correctly initialized tree always has an active root unless an
    /// internal invariant has been violated.
    #[must_use]
    pub fn active_scope(&self) -> ScopeId {
        self.active
            .last()
            .copied()
            .unwrap_or_else(ScopeId::root)
    }

    /// Returns the active lexical nesting depth, with the root at depth zero.
    #[must_use]
    pub fn active_depth(&self) -> usize {
        self.active.len().saturating_sub(1)
    }

    /// Returns the number of scopes recorded in this tree.
    #[must_use]
    pub fn len(&self) -> usize {
        self.scopes.len()
    }

    /// Returns whether only the root scope has been recorded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.scopes.len() == 1
    }

    /// Looks up immutable scope metadata.
    #[must_use]
    pub fn get(&self, id: ScopeId) -> Option<&ScopeInfo> {
        let index = usize::try_from(id.0).ok()?;
        self.scopes.get(index).filter(|scope| scope.id == id)
    }

    /// Returns the parent of a scope.
    ///
    /// The root has no parent. An unknown ID returns `ScopeError`.
    pub fn parent_of(&self, id: ScopeId) -> Result<Option<ScopeId>, ScopeError> {
        self.get(id)
            .map(ScopeInfo::parent)
            .ok_or(ScopeError::UnknownScope { scope: id })
    }

    /// Creates a child of `parent` without changing the active scope.
    ///
    /// Use this when collecting declarations or constructing a scope tree
    /// independently of source-order traversal.
    ///
    /// The parent must already exist in this tree. `ScopeKind::Root` is
    /// reserved for the unique root.
    pub fn create_child(
        &mut self,
        parent: ScopeId,
        kind: ScopeKind,
        owner: Option<NodeId>,
    ) -> Result<ScopeId, ScopeError> {
        if matches!(kind, ScopeKind::Root) {
            return Err(ScopeError::InvalidRootOperation);
        }

        let parent_index = self
            .index_of(parent)
            .ok_or(ScopeError::UnknownScope { scope: parent })?;

        let depth = self.scopes[parent_index]
            .depth
            .checked_add(1)
            .ok_or(ScopeError::IdentifierExhausted)?;

        // IDs correspond to stable vector indices. Never reuse an ID.
        let raw_id =
            u64::try_from(self.scopes.len()).map_err(|_| ScopeError::IdentifierExhausted)?;
        let id = ScopeId(raw_id);

        // Verify that this ID is representable as an index on this platform.
        let _ = usize::try_from(raw_id).map_err(|_| ScopeError::IdentifierExhausted)?;

        // Reserve all required storage before mutating either relationship.
        self.scopes
            .try_reserve(1)
            .map_err(|_| ScopeError::AllocationFailed {
                collection: "scope storage",
            })?;

        self.scopes[parent_index]
            .children
            .try_reserve(1)
            .map_err(|_| ScopeError::AllocationFailed {
                collection: "scope child list",
            })?;

        self.scopes.push(ScopeInfo {
            id,
            parent: Some(parent),
            kind,
            owner,
            children: Vec::new(),
            depth,
        });

        self.scopes[parent_index].children.push(id);

        Ok(id)
    }

    /// Creates and activates a child of the current scope.
    ///
    /// Pair this with `exit_scope`. Exiting removes only the active-stack
    /// entry; it does not delete the scope or its declarations.
    pub fn enter_scope(
        &mut self,
        kind: ScopeKind,
        owner: Option<NodeId>,
    ) -> Result<ScopeId, ScopeError> {
        // Reserve stack space first so a capacity failure does not leave a
        // newly created scope active only in the tree but absent from stack.
        self.active
            .try_reserve(1)
            .map_err(|_| ScopeError::AllocationFailed {
                collection: "active scope stack",
            })?;

        let parent = self.active_scope();
        let id = self.create_child(parent, kind, owner)?;
        self.active.push(id);

        Ok(id)
    }

    /// Leaves the current scope and returns the scope that was exited.
    ///
    /// The root cannot be exited. The complete tree remains available for
    /// subsequent passes, diagnostics, and tooling.
    pub fn exit_scope(&mut self) -> Result<ScopeId, ScopeError> {
        if self.active.len() <= 1 {
            return Err(ScopeError::CannotExitRoot);
        }

        self.active.pop().ok_or(ScopeError::CannotExitRoot)
    }

    /// Returns the ancestor chain beginning with `id` and ending with root.
    ///
    /// The result includes the scope itself. It is ordered from innermost to
    /// outermost, which is the order name resolution normally searches.
    ///
    /// A malformed parent chain is reported instead of looping indefinitely.
    pub fn ancestor_chain(&self, id: ScopeId) -> Result<Vec<ScopeId>, ScopeError> {
        if self.get(id).is_none() {
            return Err(ScopeError::UnknownScope { scope: id });
        }

        let mut result = Vec::new();
        result
            .try_reserve(self.get(id).map_or(0, ScopeInfo::depth).saturating_add(1))
            .map_err(|_| ScopeError::AllocationFailed {
                collection: "scope ancestor chain",
            })?;

        let mut current = Some(id);
        let mut visited = HashSet::new();

        while let Some(scope_id) = current {
            if !visited.insert(scope_id) {
                return Err(ScopeError::InvalidTree {
                    reason: format!("cycle detected while walking from {id}"),
                });
            }

            let info = self
                .get(scope_id)
                .ok_or(ScopeError::UnknownScope { scope: scope_id })?;

            result.push(scope_id);
            current = info.parent;

            if result.len() > self.scopes.len() {
                return Err(ScopeError::InvalidTree {
                    reason: "ancestor chain exceeds recorded scope count".to_owned(),
                });
            }
        }

        if result.last().copied() != Some(self.root_id()) {
            return Err(ScopeError::InvalidTree {
                reason: format!("scope {id} is not connected to the root"),
            });
        }

        Ok(result)
    }

    /// Returns whether `ancestor` is an ancestor of `descendant`.
    ///
    /// A scope is considered its own ancestor for this query.
    pub fn is_ancestor(
        &self,
        ancestor: ScopeId,
        descendant: ScopeId,
    ) -> Result<bool, ScopeError> {
        if self.get(ancestor).is_none() {
            return Err(ScopeError::UnknownScope { scope: ancestor });
        }

        self.ancestor_chain(descendant)
            .map(|chain| chain.contains(&ancestor))
    }

    /// Returns the recorded children of a scope in creation order.
    pub fn children_of(&self, id: ScopeId) -> Result<&[ScopeId], ScopeError> {
        self.get(id)
            .map(ScopeInfo::children)
            .ok_or(ScopeError::UnknownScope { scope: id })
    }

    /// Returns all recorded scopes in deterministic creation order.
    ///
    /// The root is first. Scopes remain in this sequence after being exited.
    pub fn iter(&self) -> impl Iterator<Item = &ScopeInfo> {
        self.scopes.iter()
    }

    /// Returns the currently active scope stack in root-to-leaf order.
    #[must_use]
    pub fn active_stack(&self) -> &[ScopeId] {
        &self.active
    }

    /// Validates the tree and active-stack invariants.
    ///
    /// Intended for tests, debug assertions, and integration gates. It checks
    /// root uniqueness, ID/index correspondence, parent/child reciprocity,
    /// reachability, depth consistency, duplicate children, and stack edges.
    pub fn validate(&self) -> Result<(), ScopeError> {
        if self.scopes.is_empty() {
            return Err(ScopeError::InvalidTree {
                reason: "scope storage is empty".to_owned(),
            });
        }

        if self.active.is_empty() || self.active.first().copied() != Some(self.root_id()) {
            return Err(ScopeError::InvalidTree {
                reason: "active stack does not begin with the root".to_owned(),
            });
        }

        let root = &self.scopes[0];

        if root.id != self.root_id()
            || root.parent.is_some()
            || !matches!(root.kind, ScopeKind::Root)
            || root.depth != 0
        {
            return Err(ScopeError::InvalidTree {
                reason: "root scope invariant is violated".to_owned(),
            });
        }

        for (index, scope) in self.scopes.iter().enumerate() {
            let expected_id =
                u64::try_from(index).map_err(|_| ScopeError::IdentifierExhausted)?;

            if scope.id.0 != expected_id {
                return Err(ScopeError::InvalidTree {
                    reason: format!(
                        "scope index {index} does not match ID {}",
                        scope.id
                    ),
                });
            }

            if index == 0 {
                continue;
            }

            let parent_id = scope.parent.ok_or_else(|| ScopeError::InvalidTree {
                reason: format!("non-root scope {} has no parent", scope.id),
            })?;

            let parent_index = self.index_of(parent_id).ok_or_else(|| {
                ScopeError::InvalidTree {
                    reason: format!(
                        "scope {} references missing parent {}",
                        scope.id, parent_id
                    ),
                }
            })?;

            let parent = &self.scopes[parent_index];

            if !parent.children.contains(&scope.id) {
                return Err(ScopeError::InvalidTree {
                    reason: format!(
                        "parent {} does not list child {}",
                        parent.id, scope.id
                    ),
                });
            }

            let expected_depth = parent
                .depth
                .checked_add(1)
                .ok_or(ScopeError::IdentifierExhausted)?;

            if scope.depth != expected_depth {
                return Err(ScopeError::InvalidTree {
                    reason: format!("incorrect depth recorded for scope {}", scope.id),
                });
            }
        }

        for parent in &self.scopes {
            let mut unique = HashSet::new();

            for child_id in &parent.children {
                if !unique.insert(*child_id) {
                    return Err(ScopeError::InvalidTree {
                        reason: format!(
                            "scope {} lists child {} more than once",
                            parent.id, child_id
                        ),
                    });
                }

                let child = self.get(*child_id).ok_or_else(|| ScopeError::InvalidTree {
                    reason: format!(
                        "scope {} lists missing child {}",
                        parent.id, child_id
                    ),
                })?;

                if child.parent != Some(parent.id) {
                    return Err(ScopeError::InvalidTree {
                        reason: format!(
                            "child {} does not point back to parent {}",
                            child.id, parent.id
                        ),
                    });
                }
            }
        }

        for pair in self.active.windows(2) {
            let parent_id = pair[0];
            let child_id = pair[1];

            let child = self.get(child_id).ok_or(ScopeError::UnknownScope {
                scope: child_id,
            })?;

            if child.parent != Some(parent_id) {
                return Err(ScopeError::InvalidTree {
                    reason: format!(
                        "active stack contains non-parent edge {} -> {}",
                        parent_id, child_id
                    ),
                });
            }
        }

        // Ensure every scope is reachable from root and that parent chains
        // cannot contain cycles. This also checks each recorded chain's root.
        for scope in &self.scopes {
            let chain = self.ancestor_chain(scope.id)?;
            if chain.last().copied() != Some(self.root_id()) {
                return Err(ScopeError::InvalidTree {
                    reason: format!("scope {} is disconnected from root", scope.id),
                });
            }
        }

        Ok(())
    }

    /// Clears the active traversal position back to the root.
    ///
    /// This does not delete scopes and is useful when restarting a traversal
    /// over an already-built scope tree. Callers must ensure that their
    /// declaration-collection algorithm is ready for this transition.
    pub fn reset_active_to_root(&mut self) {
        self.active.truncate(1);

        if self.active.is_empty() {
            self.active.push(self.root_id());
        }
    }

    /// Returns the number of bytes used by the recorded scope structures'
    /// inline storage. This is a lower-bound estimate: it excludes heap
    /// capacity and the allocations owned by strings and child vectors.
    #[must_use]
    pub fn inline_storage_bytes(&self) -> usize {
        self.scopes
            .len()
            .saturating_mul(mem::size_of::<ScopeInfo>())
            .saturating_add(
                self.active
                    .len()
                    .saturating_mul(mem::size_of::<ScopeId>()),
            )
    }

    fn index_of(&self, id: ScopeId) -> Option<usize> {
        let index = usize::try_from(id.0).ok()?;
        self.scopes
            .get(index)
            .filter(|scope| scope.id == id)
            .map(|_| index)
    }
}

impl Default for ScopeTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_exactly_one_root() {
        let tree = ScopeTree::new();

        assert_eq!(tree.len(), 1);
        assert!(tree.is_empty());
        assert_eq!(tree.root_id(), ScopeId::root());
        assert_eq!(tree.active_scope(), ScopeId::root());
        assert_eq!(tree.active_depth(), 0);
        assert_eq!(tree.get(ScopeId::root()).map(ScopeInfo::kind), Some(&ScopeKind::Root));
        assert!(tree.validate().is_ok());
    }

    #[test]
    fn creates_nested_scopes_and_tracks_depth() {
        let mut tree = ScopeTree::new();

        let function = tree
            .enter_scope(ScopeKind::Function, None)
            .expect("function scope");
        let block = tree
            .enter_scope(ScopeKind::Lexical, None)
            .expect("block scope");

        assert_eq!(tree.active_scope(), block);
        assert_eq!(tree.active_depth(), 2);
        assert_eq!(tree.get(function).map(ScopeInfo::depth), Some(1));
        assert_eq!(tree.get(block).map(ScopeInfo::depth), Some(2));
        assert_eq!(tree.parent_of(block).expect("known scope"), Some(function));
        assert!(tree.validate().is_ok());
    }

    #[test]
    fn exiting_preserves_the_scope_tree() {
        let mut tree = ScopeTree::new();
        let child = tree
            .enter_scope(ScopeKind::Lexical, None)
            .expect("child scope");

        assert_eq!(tree.exit_scope().expect("exit child"), child);
        assert_eq!(tree.active_scope(), tree.root_id());
        assert!(tree.get(child).is_some());
        assert_eq!(tree.len(), 2);
        assert!(tree.validate().is_ok());
    }

    #[test]
    fn root_cannot_be_exited_or_recreated() {
        let mut tree = ScopeTree::new();

        assert!(matches!(
            tree.exit_scope(),
            Err(ScopeError::CannotExitRoot)
        ));

        assert!(matches!(
            tree.create_child(ScopeId::root(), ScopeKind::Root, None),
            Err(ScopeError::InvalidRootOperation)
        ));
    }

    #[test]
    fn records_creation_order_and_ancestry() {
        let mut tree = ScopeTree::new();

        let first = tree
            .create_child(ScopeId::root(), ScopeKind::Module, None)
            .expect("first child");
        let second = tree
            .create_child(first, ScopeKind::Function, None)
            .expect("second child");
        let sibling = tree
            .create_child(ScopeId::root(), ScopeKind::Lexical, None)
            .expect("sibling");

        assert_eq!(
            tree.children_of(ScopeId::root()).expect("root children"),
            &[first, sibling]
        );
        assert_eq!(
            tree.ancestor_chain(second).expect("ancestor chain"),
            vec![second, first, ScopeId::root()]
        );
        assert!(tree.is_ancestor(first, second).expect("ancestor query"));
        assert!(!tree.is_ancestor(sibling, second).expect("sibling query"));
        assert!(tree.is_ancestor(second, second).expect("self ancestry"));
        assert!(tree.validate().is_ok());
    }

    #[test]
    fn rejects_unknown_scope_ids() {
        let tree = ScopeTree::new();
        let unknown = ScopeId(42);

        assert!(tree.get(unknown).is_none());
        assert!(matches!(
            tree.parent_of(unknown),
            Err(ScopeError::UnknownScope { scope }) if scope == unknown
        ));
        assert!(matches!(
            tree.ancestor_chain(unknown),
            Err(ScopeError::UnknownScope { scope }) if scope == unknown
        ));
    }

    #[test]
    fn rejects_empty_extension_categories() {
        assert!(matches!(
            ScopeKind::domain("  "),
            Err(ScopeError::EmptyScopeCategory)
        ));
        assert!(matches!(
            ScopeKind::extension("vendor", ""),
            Err(ScopeError::EmptyScopeCategory)
        ));
    }

    #[test]
    fn supports_extension_defined_scope_categories() {
        let kind = ScopeKind::extension("example.domain", "pipeline")
            .expect("valid extension category");

        assert_eq!(kind.category_name(), "extension");

        let mut tree = ScopeTree::new();
        let id = tree
            .create_child(ScopeId::root(), kind, None)
            .expect("extension scope");

        assert_eq!(
            tree.get(id).map(ScopeInfo::kind),
            Some(&ScopeKind::Extension {
                namespace: "example.domain".to_owned(),
                name: "pipeline".to_owned(),
            })
        );
        assert!(tree.validate().is_ok());
    }

    #[test]
    fn reset_returns_active_position_to_root_without_deleting_scopes() {
        let mut tree = ScopeTree::new();

        let _first = tree
            .enter_scope(ScopeKind::Function, None)
            .expect("function scope");
        let _second = tree
            .enter_scope(ScopeKind::Lexical, None)
            .expect("lexical scope");

        let count = tree.len();
        tree.reset_active_to_root();

        assert_eq!(tree.active_stack(), &[ScopeId::root()]);
        assert_eq!(tree.len(), count);
        assert!(tree.validate().is_ok());
    }

    #[test]
    fn deterministic_ids_follow_creation_order() {
        fn build() -> Vec<ScopeId> {
            let mut tree = ScopeTree::new();

            let a = tree
                .create_child(ScopeId::root(), ScopeKind::Module, None)
                .expect("module");
            let b = tree
                .create_child(a, ScopeKind::Function, None)
                .expect("function");
            let c = tree
                .create_child(ScopeId::root(), ScopeKind::Lexical, None)
                .expect("lexical");

            vec![tree.root_id(), a, b, c]
        }

        assert_eq!(build(), build());
    }

    #[test]
    fn fallible_constructor_produces_a_valid_tree() {
        let tree = ScopeTree::try_new().expect("initialize scope tree");

        assert_eq!(tree.len(), 1);
        assert!(tree.validate().is_ok());
    }
}
