
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Semantic Memory Regions
//!
//! Path: `src/classical/memory/regions.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Maintain a deterministic, target-independent index of semantic lifetime
//! scopes and their enclosing-scope relationships.
//!
//! This module supplies local region facts for ownership, borrowing, and
//! lifetime analysis. It deliberately does not implement a complete borrow
//! checker or lifetime inference engine.
//!
//! # Authority
//!
//! - `src/zuir/ids.rs` owns the canonical `RegionId` type.
//! - `src/zuir/ir/region.rs` owns canonical structural ZUIR regions.
//! - `src/zuir/ir/ownership.rs` owns canonical ownership contracts.
//! - `grammar/memory/memory.g4` owns memory-region source syntax.
//! - `grammar/memory/lifetimes.g4` owns lifetime source syntax.
//! - `grammar/memory/borrowing.g4` owns borrowing source syntax.
//! - `grammar/specification/semantic-model.md` owns normative language
//!   semantics.
//! - The semantic verifier owns whole-program lifetime and borrow checking.
//!
//! # Owns
//!
//! - The semantic scope relationship index.
//! - Scope declaration and parent relationships.
//! - Deterministic scope lookup and iteration.
//! - Local validation of scope identity, parent references, and cycles.
//! - Queries for ancestor relationships and lifetime outliving relationships.
//!
//! # Does not own
//!
//! - Region identifiers or their allocation.
//! - Canonical ZUIR region structure, blocks, captures, or operations.
//! - Source-level region names or grammar rules.
//! - Reference types, pointer types, or expression syntax.
//! - Ownership contracts or borrow-event records.
//! - Lifetime-name resolution or lifetime inference.
//! - Alias analysis, dominance, or control-flow-sensitive borrow checking.
//! - Allocation, deallocation, garbage collection, or physical addresses.
//! - Hardware capabilities, resource discovery, placement, or scheduling.
//! - Runtime memory management.
//!
//! # Semantics
//!
//! A semantic scope may be a root or a child of another scope.
//! A parent scope outlives its descendants. A scope outlives itself.
//! Sibling scopes do not outlive one another merely because they share a
//! parent. Unrelated roots have no outliving relationship in this index.
//!
//! `RegionId` is the canonical identity type. A semantic scope relationship
//! is analysis metadata, not a second representation of `ZuirRegion`.
//! The semantic analyzer must explicitly establish which canonical IDs
//! represent lifetime scopes in the analysis being performed.
//!
//! A successful `outlives` query establishes only the relationship recorded
//! in this index. It does not prove that a reference is otherwise valid,
//! that a borrow is exclusive, or that all control-flow paths are safe.
//!
//! # POCO-REAF and scalability
//!
//! This module imposes no fixed maximum number of scopes or nesting depth.
//! Its index grows with the number of declared scopes and uses ordered
//! collections for deterministic traversal.
//!
//! No finite implementation can guarantee literal infinity. Actual memory,
//! address-space, time, and identifier-space limits remain real constraints.
//! This module does not impose additional universal hardware limits.
//!
//! # Integration contract
//!
//! - `memory/mod.rs` declares `pub mod regions;`.
//! - `memory/ownership.rs` continues to re-export canonical ownership APIs.
//! - `memory/borrowing.rs` continues to record borrow declarations.
//! - Semantic analysis builds this index from resolved scope relationships.
//! - Lifetime and borrow verification consume it alongside ownership,
//!   borrow records, resolved uses, and control-flow information.
//! - IR lowering preserves established semantic relationships where required.
//! - `src/zuir/ir/region.rs` remains the owner of structural IR regions.
//! - Runtime and backend code must not interpret this index as a physical
//!   memory map or as proof that a borrow is safe.
//!
//! # Mutation contract
//!
//! A scope is declared exactly once. A child can be declared only after its
//! parent exists. The public API does not reparent or remove scopes, avoiding
//! accidental invalidation of descendant relationships.
//!
//! Deserialized indexes are validated before they are returned.
//!
//! # Freeze criteria
//!
//! The module is ready for integration when its tests pass, semantic analysis
//! supplies correctly resolved canonical IDs, and the canonical verifier
//! consumes the relationships without duplicating region identity or
//! redefining lifetime semantics.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use thiserror::Error;

use crate::zuir::ids::RegionId;

/// Result type for semantic region operations.
pub type RegionScopeResult<T> = Result<T, RegionScopeError>;

/// Errors raised by the semantic scope index.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RegionScopeError {
    /// A scope identity has already been declared.
    #[error("semantic region {0:?} is already declared")]
    DuplicateRegion(RegionId),

    /// A requested scope is not present in the index.
    #[error("semantic region {0:?} has not been declared")]
    MissingRegion(RegionId),

    /// A child references a parent that has not been declared.
    #[error("parent region {parent:?} of region {child:?} is not declared")]
    MissingParent {
        /// Scope whose declaration references the missing parent.
        child: RegionId,
        /// Parent scope that must be declared first.
        parent: RegionId,
    },

    /// An index key does not match the identity stored in its scope.
    #[error(
        "region index key {key:?} does not match scope identity {scope:?}"
    )]
    ScopeKeyMismatch {
        /// Identity used as the map key.
        key: RegionId,
        /// Identity recorded in the scope.
        scope: RegionId,
    },

    /// The scope relationships contain a cycle.
    #[error("semantic region relationships contain a cycle at {0:?}")]
    ParentCycle(RegionId),
}

/// A semantic lifetime scope and its optional enclosing scope.
///
/// This is analysis metadata. It is not a replacement for the canonical
/// structural `ZuirRegion` in `src/zuir/ir/region.rs`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionScope {
    id: RegionId,
    parent: Option<RegionId>,
}

impl RegionScope {
    /// Return this scope's canonical identity.
    #[must_use]
    pub const fn id(&self) -> RegionId {
        self.id
    }

    /// Return this scope's enclosing scope, if any.
    #[must_use]
    pub const fn parent(&self) -> Option<RegionId> {
        self.parent
    }

    /// Whether this scope is a root scope.
    #[must_use]
    pub const fn is_root(&self) -> bool {
        self.parent.is_none()
    }
}

/// Deterministic index of semantic lifetime scopes.
///
/// Scope identities are keys in ascending `RegionId` order. The index
/// supports multiple independent roots, allowing separate functions,
/// modules, or other semantic contexts to have independent scope trees.
///
/// The index does not assume that the numerical value of a `RegionId`
/// implies lexical nesting, declaration order, or execution order.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct RegionScopeIndex {
    scopes: BTreeMap<RegionId, RegionScope>,
}

/// Deserialization representation.
///
/// Validation after deserialization prevents malformed parent relationships
/// from entering the index through serialized input.
#[derive(Deserialize)]
struct RegionScopeIndexWire {
    scopes: BTreeMap<RegionId, RegionScope>,
}

impl<'de> Deserialize<'de> for RegionScopeIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = RegionScopeIndexWire::deserialize(deserializer)?;
        let index = Self {
            scopes: wire.scopes,
        };

        index.validate().map_err(serde::de::Error::custom)?;
        Ok(index)
    }
}

impl RegionScopeIndex {
    /// Construct an empty scope index.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Declare an independent root scope.
    ///
    /// A root has no parent. Multiple roots are supported; a relationship
    /// between distinct roots must not be inferred without additional
    /// semantic information.
    ///
    /// # Errors
    ///
    /// Returns `DuplicateRegion` if this identity already exists.
    pub fn declare_root(
        &mut self,
        id: RegionId,
    ) -> RegionScopeResult<()> {
        self.insert_scope(RegionScope { id, parent: None })
    }

    /// Declare a child scope of an existing parent.
    ///
    /// The parent must already be present. This declaration order ensures
    /// that ordinary API use cannot introduce parent cycles.
    ///
    /// # Errors
    ///
    /// Returns `DuplicateRegion` if the child identity already exists.
    /// Returns `MissingParent` if the parent has not been declared.
    pub fn declare_child(
        &mut self,
        id: RegionId,
        parent: RegionId,
    ) -> RegionScopeResult<()> {
        if self.scopes.contains_key(&id) {
            return Err(RegionScopeError::DuplicateRegion(id));
        }

        if !self.scopes.contains_key(&parent) {
            return Err(RegionScopeError::MissingParent {
                child: id,
                parent,
            });
        }

        self.insert_scope(RegionScope {
            id,
            parent: Some(parent),
        })
    }

    /// Return a scope by identity.
    #[must_use]
    pub fn get(&self, id: RegionId) -> Option<&RegionScope> {
        self.scopes.get(&id)
    }

    /// Whether a scope has been declared.
    #[must_use]
    pub fn contains(&self, id: RegionId) -> bool {
        self.scopes.contains_key(&id)
    }

    /// Number of declared scopes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.scopes.len()
    }

    /// Whether the index contains no scopes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.scopes.is_empty()
    }

    /// Iterate over scopes in deterministic identity order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&RegionId, &RegionScope)> {
        self.scopes.iter()
    }

    /// Return the direct parent of a declared scope.
    ///
    /// Returns `Ok(None)` for a root scope and an error for an unknown scope.
    pub fn parent_of(
        &self,
        id: RegionId,
    ) -> RegionScopeResult<Option<RegionId>> {
        self.scopes
            .get(&id)
            .map(|scope| scope.parent)
            .ok_or(RegionScopeError::MissingRegion(id))
    }

    /// Whether `ancestor` is the same as or an ancestor of `descendant`.
    ///
    /// Both scopes must be declared. The relationship is reflexive: a scope
    /// is an ancestor-or-self of itself.
    ///
    /// This query does not rely on numeric identifier ordering.
    pub fn is_ancestor_or_self(
        &self,
        ancestor: RegionId,
        descendant: RegionId,
    ) -> RegionScopeResult<bool> {
        self.require_scope(ancestor)?;
        let mut current = Some(descendant);

        while let Some(id) = current {
            let scope = self
                .scopes
                .get(&id)
                .ok_or(RegionScopeError::MissingRegion(id))?;

            if id == ancestor {
                return Ok(true);
            }

            current = scope.parent;
        }

        Ok(false)
    }

    /// Whether `ancestor` is a strict ancestor of `descendant`.
    ///
    /// A scope is not a strict ancestor of itself.
    pub fn is_strict_ancestor(
        &self,
        ancestor: RegionId,
        descendant: RegionId,
    ) -> RegionScopeResult<bool> {
        if ancestor == descendant {
            self.require_scope(ancestor)?;
            return Ok(false);
        }

        self.is_ancestor_or_self(ancestor, descendant)
    }

    /// Whether `longer` outlives `shorter` according to this scope index.
    ///
    /// In the lexical scope model, an enclosing scope outlives its nested
    /// scopes. A scope also outlives itself. Unrelated scopes do not outlive
    /// each other solely because they happen to share an ancestor.
    ///
    /// `Ok(false)` means that the recorded scope relationship does not
    /// establish the requested outliving relation. It is not a substitute
    /// for complete lifetime or borrow verification.
    pub fn outlives(
        &self,
        longer: RegionId,
        shorter: RegionId,
    ) -> RegionScopeResult<bool> {
        self.is_ancestor_or_self(longer, shorter)
    }

    /// Validate all locally checkable scope-index invariants.
    ///
    /// This verifies map-key consistency, parent existence, and absence of
    /// cycles. It does not validate the complete IR or prove borrow safety.
    pub fn validate(&self) -> RegionScopeResult<()> {
        for (key, scope) in &self.scopes {
            if *key != scope.id {
                return Err(RegionScopeError::ScopeKeyMismatch {
                    key: *key,
                    scope: scope.id,
                });
            }

            if let Some(parent) = scope.parent {
                if !self.scopes.contains_key(&parent) {
                    return Err(RegionScopeError::MissingParent {
                        child: scope.id,
                        parent,
                    });
                }
            }
        }

        // Iterative traversal avoids recursion and therefore avoids using
        // the host call stack as a limit on semantic nesting depth.
        //
        // State: 1 = active in the current path, 2 = completely validated.
        let mut states: BTreeMap<RegionId, u8> = BTreeMap::new();

        for start in self.scopes.keys().copied() {
            if states.get(&start) == Some(&2) {
                continue;
            }

            let mut path: Vec<RegionId> = Vec::new();
            let mut current = Some(start);

            while let Some(id) = current {
                match states.get(&id).copied() {
                    Some(2) => break,
                    Some(1) => {
                        return Err(RegionScopeError::ParentCycle(id));
                    }
                    _ => {}
                }

                let scope = self
                    .scopes
                    .get(&id)
                    .ok_or(RegionScopeError::MissingRegion(id))?;

                states.insert(id, 1);
                path.push(id);
                current = scope.parent;
            }

            for id in path {
                states.insert(id, 2);
            }
        }

        Ok(())
    }

    fn require_scope(&self, id: RegionId) -> RegionScopeResult<()> {
        if self.scopes.contains_key(&id) {
            Ok(())
        } else {
            Err(RegionScopeError::MissingRegion(id))
        }
    }

    fn insert_scope(
        &mut self,
        scope: RegionScope,
    ) -> RegionScopeResult<()> {
        if self.scopes.contains_key(&scope.id) {
            return Err(RegionScopeError::DuplicateRegion(scope.id));
        }

        self.scopes.insert(scope.id, scope);
        Ok(())
    }
}

impl fmt::Display for RegionScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.parent {
            Some(parent) => {
                write!(f, "region {} (parent {})", self.id, parent)
            }
            None => write!(f, "region {} (root)", self.id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn region(raw: u128) -> RegionId {
        RegionId::new(raw).expect("test region ID must be nonzero")
    }

    #[test]
    fn declares_root_and_nested_scopes() {
        let root = region(10);
        let child = region(20);
        let grandchild = region(30);

        let mut index = RegionScopeIndex::new();
        index.declare_root(root).expect("root declaration");
        index
            .declare_child(child, root)
            .expect("child declaration");
        index
            .declare_child(grandchild, child)
            .expect("grandchild declaration");

        assert_eq!(index.len(), 3);
        assert_eq!(index.parent_of(root), Ok(None));
        assert_eq!(index.parent_of(child), Ok(Some(root)));
        assert_eq!(index.parent_of(grandchild), Ok(Some(child)));
        index.validate().expect("valid scope index");
    }

    #[test]
    fn enclosing_scope_outlives_descendants() {
        let root = region(1);
        let child = region(2);
        let grandchild = region(3);

        let mut index = RegionScopeIndex::new();
        index.declare_root(root).expect("root");
        index.declare_child(child, root).expect("child");
        index
            .declare_child(grandchild, child)
            .expect("grandchild");

        assert_eq!(index.outlives(root, root), Ok(true));
        assert_eq!(index.outlives(root, child), Ok(true));
        assert_eq!(index.outlives(root, grandchild), Ok(true));
        assert_eq!(index.outlives(child, grandchild), Ok(true));

        assert_eq!(index.outlives(grandchild, child), Ok(false));
        assert_eq!(index.outlives(child, root), Ok(false));
    }

    #[test]
    fn sibling_scopes_do_not_outlive_each_other() {
        let root = region(1);
        let left = region(2);
        let right = region(3);

        let mut index = RegionScopeIndex::new();
        index.declare_root(root).expect("root");
        index.declare_child(left, root).expect("left");
        index.declare_child(right, root).expect("right");

        assert_eq!(index.outlives(left, right), Ok(false));
        assert_eq!(index.outlives(right, left), Ok(false));
        assert_eq!(index.outlives(root, left), Ok(true));
        assert_eq!(index.outlives(root, right), Ok(true));
    }

    #[test]
    fn independent_roots_have_no_inferred_relationship() {
        let first = region(100);
        let second = region(200);

        let mut index = RegionScopeIndex::new();
        index.declare_root(first).expect("first root");
        index.declare_root(second).expect("second root");

        assert_eq!(index.outlives(first, second), Ok(false));
        assert_eq!(index.outlives(second, first), Ok(false));
    }

    #[test]
    fn duplicate_scope_is_rejected_without_replacing_existing_scope() {
        let id = region(1);
        let mut index = RegionScopeIndex::new();
        index.declare_root(id).expect("initial declaration");

        assert_eq!(
            index.declare_root(id),
            Err(RegionScopeError::DuplicateRegion(id))
        );

        assert_eq!(index.len(), 1);
        assert_eq!(index.parent_of(id), Ok(None));
    }

    #[test]
    fn child_requires_an_existing_parent() {
        let child = region(2);
        let missing_parent = region(1);
        let mut index = RegionScopeIndex::new();

        assert_eq!(
            index.declare_child(child, missing_parent),
            Err(RegionScopeError::MissingParent {
                child,
                parent: missing_parent,
            })
        );

        assert!(index.is_empty());
    }

    #[test]
    fn unknown_scope_queries_return_errors() {
        let known = region(1);
        let unknown = region(2);
        let mut index = RegionScopeIndex::new();
        index.declare_root(known).expect("root");

        assert_eq!(
            index.parent_of(unknown),
            Err(RegionScopeError::MissingRegion(unknown))
        );

        assert_eq!(
            index.outlives(known, unknown),
            Err(RegionScopeError::MissingRegion(unknown))
        );

        assert_eq!(
            index.outlives(unknown, known),
            Err(RegionScopeError::MissingRegion(unknown))
        );
    }

    #[test]
    fn ancestor_queries_distinguish_strict_and_reflexive_relations() {
        let root = region(1);
        let child = region(2);
        let mut index = RegionScopeIndex::new();

        index.declare_root(root).expect("root");
        index.declare_child(child, root).expect("child");

        assert_eq!(index.is_ancestor_or_self(root, root), Ok(true));
        assert_eq!(index.is_strict_ancestor(root, root), Ok(false));
        assert_eq!(index.is_strict_ancestor(root, child), Ok(true));
        assert_eq!(index.is_strict_ancestor(child, root), Ok(false));
    }

    #[test]
    fn iteration_is_deterministic_by_canonical_identity() {
        let first = region(1);
        let second = region(2);
        let third = region(3);

        let mut index = RegionScopeIndex::new();
        index.declare_root(third).expect("third");
        index.declare_root(first).expect("first");
        index.declare_root(second).expect("second");

        let ids: Vec<RegionId> =
            index.iter().map(|(id, _)| *id).collect();

        assert_eq!(ids, vec![first, second, third]);
    }

    #[test]
    fn serialization_round_trip_preserves_scope_relationships() {
        let root = region(1);
        let child = region(2);

        let mut index = RegionScopeIndex::new();
        index.declare_root(root).expect("root");
        index.declare_child(child, root).expect("child");

        let encoded =
            serde_json::to_string(&index).expect("serialize index");
        let decoded: RegionScopeIndex =
            serde_json::from_str(&encoded).expect("deserialize index");

        assert_eq!(decoded, index);
        assert_eq!(decoded.outlives(root, child), Ok(true));
        decoded.validate().expect("round-trip index is valid");
    }

    #[test]
    fn empty_index_is_valid() {
        let index = RegionScopeIndex::new();

        assert!(index.is_empty());
        assert_eq!(index.len(), 0);
        index.validate().expect("empty index is valid");
    }
}
