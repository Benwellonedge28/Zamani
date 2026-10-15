
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Incremental Compilation — Dependency Graph
//!
//! Path: `src/zuir/incremental/dependencies.rs`
//! Edition: Rust 2021
//! Minimum Rust baseline: Rust 1.97 or later.
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Track logical dependencies between compilation artifacts so incremental
//! compilation can determine which artifacts may be stale after a change.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/version.rs`
//! - `src/zuir/fingerprint/stable_hash.rs`
//! - `src/zuir/serialization/canonical.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//!
//! OWNERSHIP
//!
//! This module owns dependency identity, dependency edges, reverse edges,
//! graph mutation, graph consistency checks, and deterministic dependency
//! traversal.
//!
//! It does not own content fingerprints, cache storage, invalidation policy,
//! IR semantics, source parsing, name resolution, or target realization.
//!
//! DEPENDENCY DIRECTION
//!
//! `dependencies.rs` is a foundational incremental-compilation component.
//! It intentionally depends only on the Rust standard library. This avoids
//! circular dependencies on future cache, fingerprint, verifier, and pass
//! manager modules.
//!
//! INTEGRATION
//!
//! - `cache_keys.rs` derives cache keys from stable artifact identities,
//!   content fingerprints, compiler/schema versions, and relevant config.
//! - `invalidation.rs` consumes `dependents_of` and `invalidation_closure`.
//! - `pass/manager.rs` records dependencies between analysis/pass artifacts.
//! - frontend and semantic stages publish stable logical artifact keys.
//! - grammar files remain owned by the grammar/frontend specification.
//!
//! SCALABILITY
//!
//! There are no hardware-specific limits or fixed graph-size constants.
//! Collections grow with available resources. Traversals use explicit work
//! lists and visited sets so dependency cycles cannot cause infinite loops.
//! Checked arithmetic prevents generation counters from silently wrapping.
//! Every actual process remains bounded by its available resources.
//!
//! STABILITY
//!
//! Dependency keys are caller-defined logical identities. They are not
//! content fingerprints. The graph's in-memory generation counter is not a
//! persistent cache version. Callers must not serialize iteration-dependent
//! or process-local identities as stable cache keys.
//!
//! FREEZE CRITERIA
//!
//! - Dependency and reverse-dependency edges remain consistent.
//! - Duplicate edge insertion is idempotent.
//! - Multiple dependency reasons can coexist for one node pair.
//! - Transitive traversal is deterministic and cycle-safe.
//! - Mutations never silently wrap the generation counter.
//! - Removal cannot leave dangling graph references.
//! - Unit and integration tests pass.
//!
//! NOTE
//!
//! This file is a standalone implementation contract. Integration still
//! requires declaring it from `incremental/mod.rs` and connecting the
//! repository's cache-key and invalidation modules.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;

/// A stable, caller-defined logical key for one compilation artifact.
///
/// Examples:
///
/// - `source:src/main.zm`
/// - `semantic:module:math`
/// - `zuir:module:math`
/// - `analysis:effects:module:math`
/// - `pass:canonicalize:module:math`
/// - `dialect:quantum@1.0.0`
///
/// Keys must identify the artifact namespace and identity, not merely its
/// current content. Content changes belong in fingerprints and cache keys.
///
/// The key is deliberately independent of concrete IR node IDs, which may
/// change between compilations.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyKey(String);

impl DependencyKey {
    /// Construct a key after validating its portable representation.
    ///
    /// Empty strings and strings containing NUL are rejected. Other
    /// characters are allowed so callers can choose a namespace grammar
    /// appropriate to their artifact domain.
    pub fn new(value: impl Into<String>) -> Result<Self, DependencyError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(DependencyError::InvalidKey {
                reason: "dependency key must not be empty or whitespace".into(),
            });
        }

        if value.contains('\0') {
            return Err(DependencyError::InvalidKey {
                reason: "dependency key must not contain NUL".into(),
            });
        }

        Ok(Self(value))
    }

    /// Return the exact key representation supplied by the caller.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the key and return its owned representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for DependencyKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Why one artifact depends on another.
///
/// These are graph-level dependency categories, not a closed list of ZUIR
/// operations or programming-language features. Add a new category only
/// when its invalidation meaning is defined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DependencyKind {
    /// Source text, source identity, or imported source changed.
    Source,

    /// Grammar, token contract, parser configuration, or syntax version.
    Grammar,

    /// Resolved names, imports, exports, or symbol relationships.
    SymbolResolution,

    /// Type definitions, generic constraints, or type compatibility.
    Type,

    /// Effects, ownership, capabilities, contracts, or resource semantics.
    Semantic,

    /// ZUIR structure or semantic operation representation.
    Ir,

    /// Dialect definition, operation schema, or dialect version.
    Dialect,

    /// Compiler configuration that changes an artifact's meaning.
    Configuration,

    /// A pass or analysis result depends on another artifact.
    Analysis,

    /// A transformation or lowering result depends on an earlier artifact.
    Transformation,

    /// A serialized/interchange representation or schema dependency.
    Serialization,

    /// A caller-defined dependency not represented by a built-in category.
    Extension,
}

/// A dependency edge from an artifact to one of its prerequisites.
///
/// The graph API uses the relationship:
///
/// `dependent -> prerequisite`
///
/// Reverse traversal is provided for invalidation:
///
/// `prerequisite -> dependent`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyEdge {
    dependent: DependencyKey,
    prerequisite: DependencyKey,
    kind: DependencyKind,
}

impl DependencyEdge {
    /// Artifact whose validity depends on the prerequisite.
    #[must_use]
    pub fn dependent(&self) -> &DependencyKey {
        &self.dependent
    }

    /// Artifact that must remain valid for the dependent to be reusable.
    #[must_use]
    pub fn prerequisite(&self) -> &DependencyKey {
        &self.prerequisite
    }

    /// Reason for the dependency.
    #[must_use]
    pub fn kind(&self) -> DependencyKind {
        self.kind
    }
}

/// Errors returned by dependency-graph operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyError {
    /// A key is malformed.
    InvalidKey {
        /// Explanation of the rejected representation.
        reason: String,
    },

    /// A mutation would overflow the graph's generation counter.
    GenerationExhausted,

    /// A caller requested an operation that requires an existing node.
    UnknownNode {
        /// The missing node.
        key: DependencyKey,
    },

    /// A graph invariant was violated.
    InvariantViolation {
        /// Human-readable invariant failure.
        reason: String,
    },
}

impl fmt::Display for DependencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidKey { reason } => {
                write!(f, "invalid dependency key: {reason}")
            }
            Self::GenerationExhausted => {
                f.write_str("dependency graph generation counter exhausted")
            }
            Self::UnknownNode { key } => {
                write!(f, "unknown dependency node: {key}")
            }
            Self::InvariantViolation { reason } => {
                write!(f, "dependency graph invariant violation: {reason}")
            }
        }
    }
}

impl Error for DependencyError {}

/// Result alias for dependency-graph operations.
pub type DependencyResult<T> = Result<T, DependencyError>;

/// Deterministic dependency graph for incremental compilation.
///
/// Each node stores:
///
/// - its direct prerequisites, keyed by prerequisite;
/// - its reverse dependents, keyed by dependent;
/// - all dependency reasons for each node pair.
///
/// `BTreeMap` and `BTreeSet` provide deterministic ordering. This is useful
/// for stable diagnostics, reproducible invalidation plans, and testing.
///
/// The graph is not internally synchronized. Keep mutation under one
/// compilation-session owner or protect it with an external synchronization
/// primitive. This avoids imposing locking costs on every graph operation.
#[derive(Debug, Clone, Default)]
pub struct DependencyGraph {
    nodes: BTreeMap<DependencyKey, NodeEdges>,
    generation: u64,
}

#[derive(Debug, Clone, Default)]
struct NodeEdges {
    /// Direct prerequisites of this node.
    prerequisites: BTreeMap<DependencyKey, BTreeSet<DependencyKind>>,

    /// Direct artifacts that depend on this node.
    dependents: BTreeMap<DependencyKey, BTreeSet<DependencyKind>>,
}

impl DependencyGraph {
    /// Create an empty graph.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of registered nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Return the number of distinct dependency reasons (edges).
    ///
    /// If two artifacts are related by multiple dependency kinds, each kind
    /// is counted separately.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.nodes
            .values()
            .flat_map(|node| node.prerequisites.values())
            .map(BTreeSet::len)
            .sum()
    }

    /// Return the in-memory mutation generation.
    ///
    /// This counter is useful for detecting changes during one compilation
    /// session. It is not a persistent version, a content hash, or a cache key.
    #[must_use]
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Return whether a node exists.
    #[must_use]
    pub fn contains_node(&self, key: &DependencyKey) -> bool {
        self.nodes.contains_key(key)
    }

    /// Register a node without adding any dependency edges.
    ///
    /// Re-registering an existing node is idempotent and does not change the
    /// graph generation.
    pub fn add_node(&mut self, key: DependencyKey) -> DependencyResult<bool> {
        if self.nodes.contains_key(&key) {
            return Ok(false);
        }

        self.ensure_next_generation()?;
        self.nodes.insert(key, NodeEdges::default());
        self.advance_generation();
        Ok(true)
    }

    /// Register a dependency relationship.
    ///
    /// `dependent` needs `prerequisite` to remain valid.
    ///
    /// Both nodes are registered automatically. Inserting an existing
    /// relationship with the same kind is idempotent.
    ///
    /// Cycles are permitted because symbol and module dependency graphs may
    /// legitimately contain cycles. Traversal and invalidation remain
    /// cycle-safe. Whether a particular cycle is legal is a higher-level
    /// semantic or build-system policy.
    pub fn add_dependency(
        &mut self,
        dependent: DependencyKey,
        prerequisite: DependencyKey,
        kind: DependencyKind,
    ) -> DependencyResult<bool> {
        let edge = DependencyEdge {
            dependent: dependent.clone(),
            prerequisite: prerequisite.clone(),
            kind,
        };

        if self.has_edge(&edge) {
            return Ok(false);
        }

        // Check generation exhaustion before making any changes, so a failed
        // mutation leaves the graph unchanged.
        self.ensure_next_generation()?;

        self.nodes.entry(dependent.clone()).or_default();
        self.nodes.entry(prerequisite.clone()).or_default();

        self.nodes
            .get_mut(&dependent)
            .expect("dependent was inserted above")
            .prerequisites
            .entry(prerequisite.clone())
            .or_default()
            .insert(kind);

        self.nodes
            .get_mut(&prerequisite)
            .expect("prerequisite was inserted above")
            .dependents
            .entry(dependent)
            .or_default()
            .insert(kind);

        self.advance_generation();
        Ok(true)
    }

    /// Remove one dependency reason.
    ///
    /// Other dependency kinds between the same pair are preserved. Returns
    /// `true` if an edge was removed.
    pub fn remove_dependency(
        &mut self,
        dependent: &DependencyKey,
        prerequisite: &DependencyKey,
        kind: DependencyKind,
    ) -> DependencyResult<bool> {
        let edge = DependencyEdge {
            dependent: dependent.clone(),
            prerequisite: prerequisite.clone(),
            kind,
        };

        if !self.has_edge(&edge) {
            return Ok(false);
        }

        self.ensure_next_generation()?;

        remove_kind(
            &mut self
                .nodes
                .get_mut(dependent)
                .expect("edge dependent must exist")
                .prerequisites,
            prerequisite,
            kind,
        );

        remove_kind(
            &mut self
                .nodes
                .get_mut(prerequisite)
                .expect("edge prerequisite must exist")
                .dependents,
            dependent,
            kind,
        );

        self.advance_generation();
        Ok(true)
    }

    /// Remove a node and every incoming/outgoing relationship.
    ///
    /// Returns `true` if the node existed. This is appropriate when an
    /// artifact is removed from the compilation graph. Callers should first
    /// compute any required invalidation closure if dependents still need to
    /// be rebuilt.
    pub fn remove_node(&mut self, key: &DependencyKey) -> DependencyResult<bool> {
        let Some(existing) = self.nodes.get(key) else {
            return Ok(false);
        };

        // Removing a node with edges changes the graph. Removing an isolated
        // node also changes the graph.
        let _ = existing;
        self.ensure_next_generation()?;

        let removed = self
            .nodes
            .remove(key)
            .expect("node existence checked above");

        for prerequisite in removed.prerequisites.keys() {
            if let Some(node) = self.nodes.get_mut(prerequisite) {
                node.dependents.remove(key);
            }
        }

        for dependent in removed.dependents.keys() {
            if let Some(node) = self.nodes.get_mut(dependent) {
                node.prerequisites.remove(key);
            }
        }

        self.advance_generation();
        Ok(true)
    }

    /// Return the direct prerequisites of a node, in deterministic order.
    ///
    /// An unknown node is an error, rather than an empty dependency list,
    /// because confusing a missing artifact with a dependency-free artifact
    /// can produce incorrect incremental reuse.
    pub fn prerequisites_of(
        &self,
        key: &DependencyKey,
    ) -> DependencyResult<Vec<DependencyEdge>> {
        let node = self
            .nodes
            .get(key)
            .ok_or_else(|| DependencyError::UnknownNode { key: key.clone() })?;

        let mut result = Vec::new();

        for (prerequisite, kinds) in &node.prerequisites {
            for kind in kinds {
                result.push(DependencyEdge {
                    dependent: key.clone(),
                    prerequisite: prerequisite.clone(),
                    kind: *kind,
                });
            }
        }

        Ok(result)
    }

    /// Return direct dependents of a prerequisite, in deterministic order.
    ///
    /// These are the artifacts that may need invalidation if `key` changes.
    pub fn dependents_of(
        &self,
        key: &DependencyKey,
    ) -> DependencyResult<Vec<DependencyEdge>> {
        let node = self
            .nodes
            .get(key)
            .ok_or_else(|| DependencyError::UnknownNode { key: key.clone() })?;

        let mut result = Vec::new();

        for (dependent, kinds) in &node.dependents {
            for kind in kinds {
                result.push(DependencyEdge {
                    dependent: dependent.clone(),
                    prerequisite: key.clone(),
                    kind: *kind,
                });
            }
        }

        Ok(result)
    }

    /// Return all transitive prerequisites of an artifact.
    ///
    /// The root is excluded from the result, even when a cycle leads back to
    /// it. Traversal order is deterministic and each node appears once.
    pub fn dependency_closure(
        &self,
        root: &DependencyKey,
    ) -> DependencyResult<Vec<DependencyKey>> {
        self.require_node(root)?;

        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();

        if let Some(node) = self.nodes.get(root) {
            for key in node.prerequisites.keys() {
                queue.push_back(key.clone());
            }
        }

        while let Some(current) = queue.pop_front() {
            if &current == root || !visited.insert(current.clone()) {
                continue;
            }

            if let Some(node) = self.nodes.get(&current) {
                for prerequisite in node.prerequisites.keys() {
                    if !visited.contains(prerequisite) {
                        queue.push_back(prerequisite.clone());
                    }
                }
            }
        }

        Ok(visited.into_iter().collect())
    }

    /// Return the transitive set of artifacts that depend on a changed node.
    ///
    /// The changed root is excluded. Cycles are handled safely, and each
    /// dependent appears exactly once.
    pub fn invalidation_closure(
        &self,
        changed: &DependencyKey,
    ) -> DependencyResult<Vec<DependencyKey>> {
        self.require_node(changed)?;

        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();

        if let Some(node) = self.nodes.get(changed) {
            for key in node.dependents.keys() {
                queue.push_back(key.clone());
            }
        }

        while let Some(current) = queue.pop_front() {
            if &current == changed || !visited.insert(current.clone()) {
                continue;
            }

            if let Some(node) = self.nodes.get(&current) {
                for dependent in node.dependents.keys() {
                    if !visited.contains(dependent) {
                        queue.push_back(dependent.clone());
                    }
                }
            }
        }

        Ok(visited.into_iter().collect())
    }

    /// Return the dependency graph's nodes in deterministic order.
    #[must_use]
    pub fn nodes(&self) -> impl Iterator<Item = &DependencyKey> {
        self.nodes.keys()
    }

    /// Check all forward/reverse edge invariants.
    ///
    /// Run this in tests, after decoding a persisted graph, or at a diagnostic
    /// boundary. Ordinary callers need not perform a full check after every
    /// individual mutation.
    pub fn validate(&self) -> DependencyResult<()> {
        for (dependent, node) in &self.nodes {
            for (prerequisite, kinds) in &node.prerequisites {
                if kinds.is_empty() {
                    return Err(DependencyError::InvariantViolation {
                        reason: format!(
                            "empty dependency-kind set for {dependent} -> {prerequisite}"
                        ),
                    });
                }

                let reverse = self
                    .nodes
                    .get(prerequisite)
                    .ok_or_else(|| DependencyError::InvariantViolation {
                        reason: format!(
                            "missing prerequisite node {prerequisite} referenced by {dependent}"
                        ),
                    })?;

                match reverse.dependents.get(dependent) {
                    Some(reverse_kinds) if reverse_kinds == kinds => {}
                    _ => {
                        return Err(DependencyError::InvariantViolation {
                            reason: format!(
                                "forward/reverse dependency mismatch for {dependent} -> {prerequisite}"
                            ),
                        });
                    }
                }
            }

            for (other_dependent, kinds) in &node.dependents {
                if kinds.is_empty() {
                    return Err(DependencyError::InvariantViolation {
                        reason: format!(
                            "empty reverse dependency-kind set for {other_dependent} -> {dependent}"
                        ),
                    });
                }

                let forward = self
                    .nodes
                    .get(other_dependent)
                    .ok_or_else(|| DependencyError::InvariantViolation {
                        reason: format!(
                            "missing dependent node {other_dependent} references {dependent}"
                        ),
                    })?;

                match forward.prerequisites.get(dependent) {
                    Some(forward_kinds) if forward_kinds == kinds => {}
                    _ => {
                        return Err(DependencyError::InvariantViolation {
                            reason: format!(
                                "reverse/forward dependency mismatch for {other_dependent} -> {dependent}"
                            ),
                        });
                    }
                }
            }
        }

        Ok(())
    }

    fn require_node(&self, key: &DependencyKey) -> DependencyResult<()> {
        if self.nodes.contains_key(key) {
            Ok(())
        } else {
            Err(DependencyError::UnknownNode { key: key.clone() })
        }
    }

    fn has_edge(&self, edge: &DependencyEdge) -> bool {
        self.nodes
            .get(&edge.dependent)
            .and_then(|node| node.prerequisites.get(&edge.prerequisite))
            .is_some_and(|kinds| kinds.contains(&edge.kind))
    }

    fn ensure_next_generation(&self) -> DependencyResult<()> {
        self.generation
            .checked_add(1)
            .map(|_| ())
            .ok_or(DependencyError::GenerationExhausted)
    }

    fn advance_generation(&mut self) {
        // Every mutating method checks this before modifying graph state.
        self.generation = self
            .generation
            .checked_add(1)
            .expect("generation capacity was checked before mutation");
    }
}

fn remove_kind(
    edges: &mut BTreeMap<DependencyKey, BTreeSet<DependencyKind>>,
    other: &DependencyKey,
    kind: DependencyKind,
) {
    let remove_entry = if let Some(kinds) = edges.get_mut(other) {
        kinds.remove(&kind);
        kinds.is_empty()
    } else {
        false
    };

    if remove_entry {
        edges.remove(other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(value: &str) -> DependencyKey {
        DependencyKey::new(value).expect("test key should be valid")
    }

    #[test]
    fn rejects_empty_and_nul_keys() {
        assert!(DependencyKey::new("").is_err());
        assert!(DependencyKey::new("  ").is_err());
        assert!(DependencyKey::new("source:\0main.zm").is_err());
        assert!(DependencyKey::new("source:main.zm").is_ok());
    }

    #[test]
    fn node_registration_is_idempotent() {
        let mut graph = DependencyGraph::new();
        let node = key("source:main.zm");

        assert!(graph.add_node(node.clone()).unwrap());
        let generation = graph.generation();

        assert!(!graph.add_node(node).unwrap());
        assert_eq!(graph.generation(), generation);
        assert_eq!(graph.node_count(), 1);
    }

    #[test]
    fn dependency_edges_are_idempotent_and_reverse_indexed() {
        let mut graph = DependencyGraph::new();
        let source = key("source:main.zm");
        let semantic = key("semantic:main");

        assert!(graph
            .add_dependency(
                semantic.clone(),
                source.clone(),
                DependencyKind::Source
            )
            .unwrap());

        assert!(!graph
            .add_dependency(
                semantic.clone(),
                source.clone(),
                DependencyKind::Source
            )
            .unwrap());

        assert_eq!(graph.edge_count(), 1);
        assert_eq!(
            graph.dependents_of(&source).unwrap(),
            vec![DependencyEdge {
                dependent: semantic,
                prerequisite: source,
                kind: DependencyKind::Source,
            }]
        );
        graph.validate().unwrap();
    }

    #[test]
    fn different_dependency_reasons_coexist() {
        let mut graph = DependencyGraph::new();
        let dependent = key("zuir:module:a");
        let prerequisite = key("semantic:module:a");

        graph
            .add_dependency(
                dependent.clone(),
                prerequisite.clone(),
                DependencyKind::Type,
            )
            .unwrap();

        graph
            .add_dependency(
                dependent.clone(),
                prerequisite.clone(),
                DependencyKind::Semantic,
            )
            .unwrap();

        assert_eq!(graph.edge_count(), 2);

        graph
            .remove_dependency(
                &dependent,
                &prerequisite,
                DependencyKind::Type,
            )
            .unwrap();

        assert_eq!(graph.edge_count(), 1);
        assert_eq!(
            graph.prerequisites_of(&dependent).unwrap()[0].kind(),
            DependencyKind::Semantic
        );
        graph.validate().unwrap();
    }

    #[test]
    fn invalidation_closure_is_transitive_and_deterministic() {
        let mut graph = DependencyGraph::new();

        let source = key("source:a");
        let semantic = key("semantic:a");
        let ir = key("zuir:a");
        let optimized = key("optimized:a");

        graph
            .add_dependency(
                semantic.clone(),
                source.clone(),
                DependencyKind::Source,
            )
            .unwrap();
        graph
            .add_dependency(ir.clone(), semantic.clone(), DependencyKind::Ir)
            .unwrap();
        graph
            .add_dependency(
                optimized.clone(),
                ir.clone(),
                DependencyKind::Transformation,
            )
            .unwrap();

        assert_eq!(
            graph.invalidation_closure(&source).unwrap(),
            vec![optimized, ir, semantic]
        );
        assert_eq!(
            graph.dependency_closure(&optimized).unwrap(),
            vec![ir, semantic, source]
        );
    }

    #[test]
    fn cycles_do_not_cause_infinite_traversal() {
        let mut graph = DependencyGraph::new();
        let a = key("module:a");
        let b = key("module:b");
        let c = key("module:c");

        graph
            .add_dependency(a.clone(), b.clone(), DependencyKind::SymbolResolution)
            .unwrap();
        graph
            .add_dependency(b.clone(), c.clone(), DependencyKind::SymbolResolution)
            .unwrap();
        graph
            .add_dependency(c.clone(), a.clone(), DependencyKind::SymbolResolution)
            .unwrap();

        assert_eq!(graph.invalidation_closure(&a).unwrap(), vec![b.clone(), c.clone()]);
        assert_eq!(graph.dependency_closure(&a).unwrap(), vec![b, c]);
        graph.validate().unwrap();
    }

    #[test]
    fn removing_a_node_removes_all_incident_edges() {
        let mut graph = DependencyGraph::new();
        let a = key("source:a");
        let b = key("semantic:b");
        let c = key("ir:c");

        graph
            .add_dependency(b.clone(), a.clone(), DependencyKind::Source)
            .unwrap();
        graph
            .add_dependency(c.clone(), b.clone(), DependencyKind::Ir)
            .unwrap();

        assert!(graph.remove_node(&b).unwrap());
        assert!(!graph.contains_node(&b));
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.dependents_of(&a).unwrap().is_empty());
        graph.validate().unwrap();

        assert!(!graph.remove_node(&b).unwrap());
    }

    #[test]
    fn unknown_nodes_are_not_silently_treated_as_empty() {
        let graph = DependencyGraph::new();

        assert!(matches!(
            graph.invalidation_closure(&key("missing")),
            Err(DependencyError::UnknownNode { .. })
        ));
        assert!(matches!(
            graph.prerequisites_of(&key("missing")),
            Err(DependencyError::UnknownNode { .. })
        ));
    }

    #[test]
    fn graph_mutations_advance_generation_only_when_changed() {
        let mut graph = DependencyGraph::new();
        let a = key("a");
        let b = key("b");

        assert_eq!(graph.generation(), 0);
        graph.add_node(a.clone()).unwrap();
        assert_eq!(graph.generation(), 1);

        graph.add_node(a.clone()).unwrap();
        assert_eq!(graph.generation(), 1);

        graph
            .add_dependency(b.clone(), a.clone(), DependencyKind::Ir)
            .unwrap();
        assert_eq!(graph.generation(), 2);

        graph
            .add_dependency(b, a.clone(), DependencyKind::Ir)
            .unwrap();
        assert_eq!(graph.generation(), 2);

        graph.remove_node(&a).unwrap();
        assert_eq!(graph.generation(), 3);
        graph.validate().unwrap();
    }

    #[test]
    fn independent_graphs_are_deterministic() {
        fn build() -> DependencyGraph {
            let mut graph = DependencyGraph::new();
            let a = key("source:a");
            let b = key("semantic:b");
            let c = key("ir:c");

            graph
                .add_dependency(c.clone(), b.clone(), DependencyKind::Ir)
                .unwrap();
            graph
                .add_dependency(b, a, DependencyKind::Source)
                .unwrap();

            graph
        }

        let first = build();
        let second = build();

        let first_nodes: Vec<_> = first.nodes().map(|k| k.as_str()).collect();
        let second_nodes: Vec<_> = second.nodes().map(|k| k.as_str()).collect();

        assert_eq!(first_nodes, second_nodes);
        assert_eq!(
            first.invalidation_closure(&key("source:a")).unwrap(),
            second.invalidation_closure(&key("source:a")).unwrap()
        );
    }
}
