
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Incremental Semantic-Fact Cache
//!
//! Path: `src/frontend/semantic/incremental/cache.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Cache semantic-analysis results by canonical AST node identity.
//! Coordinate cache validity with `InvalidationPlan` without taking
//! ownership of the AST, dependency graph, semantic pipeline, or IR.
//!
//! This module is generic over the cached value type. The semantic model
//! defines the meaning and structure of the values stored here.
//!
//! ## STATUS
//!
//! Implementation candidate. Freeze only after the module is integrated
//! into `incremental/mod.rs` and its acceptance tests pass.
//!
//! ## AUTHORITY
//!
//! - `frontend::ast::node::node_id::NodeId` owns AST node identity.
//! - `dependencies.rs` owns semantic dependency relationships.
//! - `invalidation.rs` owns invalidation planning and recomputation order.
//! - The semantic model owns semantic facts and their meaning.
//! - The semantic session/coordinator owns analysis execution and publication.
//! - The grammar and language specifications own source-language meaning.
//! - Downstream IR and target-planning modules own their own artifacts.
//!
//! ## CACHE VALIDITY CONTRACT
//!
//! Only entries in `Valid` state may be returned by `get`.
//!
//! Invalidated or failed entries may retain an older value for explicit
//! inspection or reuse, but that value is never returned by `get`.
//! Consumers that inspect stale values must call `get_stale` explicitly
//! and must not treat its result as a validated semantic fact.
//!
//! A value may be published through `publish` only after its entry has
//! entered `Computing` state through `begin_recompute`.
//!
//! ## STATE TRANSITIONS
//!
//! ```text
//! Missing ────────────────> Computing
//! Valid ── invalidation ──> Invalidated
//! Invalidated ────────────> Computing
//! Failed ─────────────────> Computing
//! Computing ── success ───> Valid
//! Computing ── failure ───> Failed
//! Any tracked state ──────> Invalidated (when affected)
//! ```
//!
//! `insert_valid` is an explicit seeding/replacement operation. It can
//! install a value as valid without a recomputation lifecycle, so callers
//! must use it only when they already possess a trustworthy result.
//!
//! ## REVISION CONTRACT
//!
//! The cache maintains a checked, monotonically increasing revision.
//! Successful mutating operations advance it. An empty invalidation plan
//! and removal of a missing entry are no-ops and do not advance it.
//!
//! If the revision cannot advance, a mutating operation returns an error
//! before changing cache state. Revisions are cache-local mutation markers;
//! they are not persistent identities, source versions, or global clocks.
//!
//! ## SCALABILITY
//!
//! The cache imposes no language-level limit on AST nodes, semantic facts,
//! qubits, tensor dimensions, devices, or hardware capacities.
//!
//! Its memory and execution costs are proportional to cached entries and
//! affected nodes, subject to available resources and Rust's collection
//! and address-space limits. Configurable operational budgets belong to
//! the semantic coordinator, not to a fixed language-level ceiling here.
//!
//! ## DETERMINISM
//!
//! Entries are stored in `BTreeMap<NodeId, _>`. Operations over an
//! `InvalidationPlan` follow its deterministic affected-node ordering.
//! This module performs no parallel scheduling and relies on no global
//! mutable state, clock, randomness, filesystem, or hardware discovery.
//!
//! ## DOES NOT OWN
//!
//! - AST construction or mutation.
//! - Dependency graph construction or mutation.
//! - Invalidation closure calculation.
//! - Semantic recomputation or pass scheduling.
//! - Semantic type/effect/capability/resource rules.
//! - Source grammar or parsing.
//! - Classical, quantum, or HDL IR.
//! - Physical hardware allocation, routing, or scheduling.
//!
//! ## INTEGRATION CONTRACT
//!
//! 1. The coordinator tracks semantic dependencies in `DependencyGraph`.
//! 2. It builds an `InvalidationPlan` before destructive graph changes.
//! 3. It calls `apply_invalidation` before relying on cached results.
//! 4. It invokes `begin_recompute` before computing each affected fact.
//! 5. It calls `publish` only after the semantic pass succeeds.
//! 6. It calls `mark_failed` when a started computation fails.
//! 7. It updates dependencies and publishes complete analysis results
//!    only through the owning coordinator/session.
//!
//! This cache does not prove that a plan matches the current graph version.
//! The coordinator must serialize graph/cache changes or otherwise ensure
//! that a plan remains applicable while it is being applied.
//!
//! ## FREEZE CRITERIA
//!
//! - The incremental module exports this cache.
//! - Invalidated values cannot be read through `get`.
//! - Invalidations cover the complete plan and are atomic with respect
//!   to all ordinary, fallible cache operations.
//! - Illegal state transitions return structured errors.
//! - Revision exhaustion never wraps or silently mutates state.
//! - Tests cover invalidation, recomputation, failure, removal, empty plans,
//!   deterministic behavior, and retained stale values.
//! - The workspace, relevant tests, formatting, and lint checks pass.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::frontend::ast::node::node_id::NodeId;

use super::invalidation::InvalidationPlan;

/// State of one cache entry.
///
/// A missing entry has no state and is represented by `None` from
/// [`SemanticCache::state`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheState {
    /// The value is available for ordinary semantic-fact reads.
    Valid,

    /// The old result must not be used as a current semantic fact.
    Invalidated,

    /// A semantic pass has started recomputing this entry.
    Computing,

    /// The most recent recomputation attempt failed.
    Failed,
}

/// One cached semantic value and its validity metadata.
///
/// Fields are private so callers cannot change validity without going
/// through the cache's checked state-transition methods.
#[derive(Clone, Debug)]
struct CacheEntry<V> {
    value: Option<V>,
    state: CacheState,
    revision: u64,
}

/// Deterministic cache for semantic facts keyed by canonical AST node IDs.
///
/// `V` is intentionally generic. A caller may store an individual semantic
/// fact, a finalized node result, or another value whose ownership belongs
/// to the semantic model. The cache does not interpret `V`.
///
/// The cache is not internally synchronized. Each analysis session should
/// own its cache or provide synchronization at the coordinator boundary.
/// Independent sessions can use independent caches without shared state.
#[derive(Clone, Debug)]
pub struct SemanticCache<V> {
    entries: BTreeMap<NodeId, CacheEntry<V>>,
    revision: u64,
}

/// Summary of one successfully applied invalidation plan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidationReport {
    /// Cache revision after applying the operation.
    ///
    /// For an empty plan this is the unchanged current revision.
    pub revision: u64,

    /// Number of affected node IDs recorded in the cache after invalidation.
    ///
    /// This includes affected nodes that had no previous cache entry.
    pub affected_entries: usize,

    /// Number of affected entries whose previous values were retained.
    ///
    /// Retained values are stale and cannot be retrieved through `get`.
    pub retained_values: usize,
}

/// Errors from cache lifecycle and revision operations.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CacheError {
    /// The cache revision reached its representational limit.
    ///
    /// The attempted operation did not mutate cache state.
    RevisionExhausted,

    /// A lifecycle operation was attempted from an unsupported state.
    InvalidTransition {
        node: NodeId,
        current: Option<CacheState>,
        operation: &'static str,
    },
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RevisionExhausted => {
                write!(f, "semantic cache revision exhausted")
            }
            Self::InvalidTransition {
                node,
                current,
                operation,
            } => write!(
                f,
                "cannot {operation} cache entry {node:?} from state {current:?}"
            ),
        }
    }
}

impl Error for CacheError {}

impl<V> Default for SemanticCache<V> {
    fn default() -> Self {
        Self {
            entries: BTreeMap::new(),
            revision: 0,
        }
    }
}

impl<V> SemanticCache<V> {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Current cache-local mutation revision.
    ///
    /// This is not a source revision and must not be compared across
    /// unrelated cache instances.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Number of tracked entries, including entries without a value.
    ///
    /// Invalidated, computing, and failed entries remain tracked until
    /// explicitly removed or replaced.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether no entries are tracked.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Number of entries currently eligible for ordinary reads.
    pub fn valid_len(&self) -> usize {
        self.entries
            .values()
            .filter(|entry| entry.state == CacheState::Valid)
            .count()
    }

    /// Whether a node currently has a valid cached value.
    pub fn contains_valid(&self, node: NodeId) -> bool {
        self.entries
            .get(&node)
            .is_some_and(|entry| {
                entry.state == CacheState::Valid && entry.value.is_some()
            })
    }

    /// Current state of a tracked entry, or `None` if the node is untracked.
    pub fn state(&self, node: NodeId) -> Option<CacheState> {
        self.entries.get(&node).map(|entry| entry.state)
    }

    /// Revision at which this entry was most recently changed.
    pub fn entry_revision(&self, node: NodeId) -> Option<u64> {
        self.entries.get(&node).map(|entry| entry.revision)
    }

    /// Read a value only when its entry is valid.
    ///
    /// Invalidated, computing, failed, and missing entries return `None`.
    /// This is the safe default for semantic analysis consumers.
    pub fn get(&self, node: NodeId) -> Option<&V> {
        let entry = self.entries.get(&node)?;

        if entry.state != CacheState::Valid {
            return None;
        }

        entry.value.as_ref()
    }

    /// Explicitly inspect a retained value, even if it is stale.
    ///
    /// This method does not certify validity. A value returned for an
    /// invalidated, computing, or failed entry must not be used as a
    /// current semantic fact without an independent validity check.
    pub fn get_stale(&self, node: NodeId) -> Option<&V> {
        self.entries.get(&node)?.value.as_ref()
    }

    /// Install or replace a value already known to be valid.
    ///
    /// This operation is intended for trusted initial results, validated
    /// imports, or coordinator-controlled cache seeding. It deliberately
    /// bypasses the recomputation lifecycle; ordinary recomputation should
    /// use `begin_recompute` followed by `publish`.
    ///
    /// Returns the previous retained value, if any. A successful operation
    /// advances the cache revision.
    pub fn insert_valid(
        &mut self,
        node: NodeId,
        value: V,
    ) -> Result<Option<V>, CacheError> {
        let next_revision = self.next_revision()?;

        let previous = self.entries.insert(
            node,
            CacheEntry {
                value: Some(value),
                state: CacheState::Valid,
                revision: next_revision,
            },
        );

        self.revision = next_revision;

        Ok(previous.and_then(|entry| entry.value))
    }

    /// Invalidate every node in a complete invalidation plan.
    ///
    /// The plan should have been built from the current dependency graph
    /// before destructive graph edits. Every affected node is recorded,
    /// even when the cache did not previously contain that node.
    ///
    /// Old values are retained for explicit inspection but are hidden from
    /// `get`. This lets a coordinator choose whether to reuse or discard
    /// stale storage without accidentally presenting it as valid.
    ///
    /// An empty plan is a no-op and does not advance the revision.
    ///
    /// The operation checks revision exhaustion before changing any entry.
    /// It returns a report with the number of affected entries and retained
    /// old values.
    pub fn apply_invalidation(
        &mut self,
        plan: &InvalidationPlan,
    ) -> Result<InvalidationReport, CacheError> {
        let affected = plan.affected_nodes();

        if affected.is_empty() {
            return Ok(InvalidationReport {
                revision: self.revision,
                affected_entries: 0,
                retained_values: 0,
            });
        }

        // Check before mutation so revision exhaustion leaves the cache
        // unchanged.
        let next_revision = self.next_revision()?;

        let mut retained_values = 0usize;

        for node in affected {
            let entry = self.entries.entry(*node).or_insert_with(|| {
                CacheEntry {
                    value: None,
                    state: CacheState::Invalidated,
                    revision: next_revision,
                }
            });

            if entry.value.is_some() {
                retained_values += 1;
            }

            entry.state = CacheState::Invalidated;
            entry.revision = next_revision;
        }

        self.revision = next_revision;

        Ok(InvalidationReport {
            revision: next_revision,
            affected_entries: affected.len(),
            retained_values,
        })
    }

    /// Begin recomputation for one node.
    ///
    /// Allowed starting states:
    ///
    /// - missing;
    /// - invalidated;
    /// - failed.
    ///
    /// A valid entry must first be invalidated, and a currently computing
    /// entry cannot be started a second time. On success the old value, if
    /// present, remains available only through `get_stale`.
    pub fn begin_recompute(
        &mut self,
        node: NodeId,
    ) -> Result<(), CacheError> {
        let current = self.state(node);

        match current {
            None | Some(CacheState::Invalidated) | Some(CacheState::Failed) => {}
            Some(CacheState::Valid) | Some(CacheState::Computing) => {
                return Err(CacheError::InvalidTransition {
                    node,
                    current,
                    operation: "begin recomputation",
                });
            }
        }

        let next_revision = self.next_revision()?;

        let entry = self.entries.entry(node).or_insert_with(|| CacheEntry {
            value: None,
            state: CacheState::Computing,
            revision: next_revision,
        });

        entry.state = CacheState::Computing;
        entry.revision = next_revision;
        self.revision = next_revision;

        Ok(())
    }

    /// Publish a successfully recomputed semantic value.
    ///
    /// The entry must be in `Computing` state. Publication advances the
    /// revision and makes the new value available through `get`.
    pub fn publish(
        &mut self,
        node: NodeId,
        value: V,
    ) -> Result<(), CacheError> {
        if self.state(node) != Some(CacheState::Computing) {
            return Err(CacheError::InvalidTransition {
                node,
                current: self.state(node),
                operation: "publish a recomputed value",
            });
        }

        let next_revision = self.next_revision()?;

        // The state check above guarantees the entry exists.
        if let Some(entry) = self.entries.get_mut(&node) {
            entry.value = Some(value);
            entry.state = CacheState::Valid;
            entry.revision = next_revision;
        }

        self.revision = next_revision;

        Ok(())
    }

    /// Mark a started recomputation as failed.
    ///
    /// Any older value is retained for explicit stale inspection, but is
    /// never returned by `get`. A missing or non-computing entry is an error.
    pub fn mark_failed(&mut self, node: NodeId) -> Result<(), CacheError> {
        if self.state(node) != Some(CacheState::Computing) {
            return Err(CacheError::InvalidTransition {
                node,
                current: self.state(node),
                operation: "mark recomputation failed",
            });
        }

        let next_revision = self.next_revision()?;

        if let Some(entry) = self.entries.get_mut(&node) {
            entry.state = CacheState::Failed;
            entry.revision = next_revision;
        }

        self.revision = next_revision;

        Ok(())
    }

    /// Remove an entry and return its retained value, if any.
    ///
    /// Removing a missing entry is a no-op and does not advance the revision.
    /// Removal of a tracked entry advances the revision. The caller is
    /// responsible for ensuring that dependent facts have already been
    /// invalidated or recomputed through the dependency coordinator.
    pub fn remove(
        &mut self,
        node: NodeId,
    ) -> Result<Option<V>, CacheError> {
        if !self.entries.contains_key(&node) {
            return Ok(None);
        }

        let next_revision = self.next_revision()?;
        let removed = self.entries.remove(&node);

        self.revision = next_revision;

        Ok(removed.and_then(|entry| entry.value))
    }

    /// Remove all tracked entries.
    ///
    /// This does not mutate the dependency graph. A non-empty clear advances
    /// the revision once; clearing an empty cache is a no-op.
    pub fn clear(&mut self) -> Result<(), CacheError> {
        if self.entries.is_empty() {
            return Ok(());
        }

        let next_revision = self.next_revision()?;
        self.entries.clear();
        self.revision = next_revision;

        Ok(())
    }

    /// Return an iterator over tracked node IDs in deterministic order.
    pub fn nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.entries.keys().copied()
    }

    /// Compute the next revision without mutating cache state.
    fn next_revision(&self) -> Result<u64, CacheError> {
        self.revision
            .checked_add(1)
            .ok_or(CacheError::RevisionExhausted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::incremental::dependencies::DependencyGraph;
    use crate::frontend::semantic::incremental::invalidation::InvalidationPlan;

    fn id(value: u64) -> NodeId {
        NodeId::new(value).expect("test node IDs must be nonzero")
    }

    fn plan_for(
        graph: &DependencyGraph,
        changed: impl IntoIterator<Item = NodeId>,
    ) -> InvalidationPlan {
        InvalidationPlan::build(graph, changed)
            .expect("test graph should yield a complete invalidation plan")
    }

    #[test]
    fn new_cache_is_empty() {
        let cache = SemanticCache::<String>::new();

        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.valid_len(), 0);
        assert_eq!(cache.revision(), 0);
    }

    #[test]
    fn valid_values_can_be_read() {
        let mut cache = SemanticCache::new();

        cache
            .insert_valid(id(1), String::from("integer"))
            .unwrap();

        assert_eq!(cache.get(id(1)).map(String::as_str), Some("integer"));
        assert!(cache.contains_valid(id(1)));
        assert_eq!(cache.state(id(1)), Some(CacheState::Valid));
    }

    #[test]
    fn invalidation_hides_stale_values() {
        let mut graph = DependencyGraph::new();

        // Node 2 depends on node 1.
        graph.add_dependency(id(2), id(1)).unwrap();

        let mut cache = SemanticCache::new();
        cache.insert_valid(id(1), String::from("old prerequisite")).unwrap();
        cache.insert_valid(id(2), String::from("old dependent")).unwrap();

        let plan = plan_for(&graph, [id(1)]);
        let report = cache.apply_invalidation(&plan).unwrap();

        assert_eq!(report.affected_entries, 2);
        assert_eq!(report.retained_values, 2);
        assert_eq!(cache.state(id(1)), Some(CacheState::Invalidated));
        assert_eq!(cache.state(id(2)), Some(CacheState::Invalidated));

        assert_eq!(cache.get(id(1)), None);
        assert_eq!(cache.get(id(2)), None);

        assert_eq!(
            cache.get_stale(id(1)).map(String::as_str),
            Some("old prerequisite")
        );
        assert_eq!(
            cache.get_stale(id(2)).map(String::as_str),
            Some("old dependent")
        );
    }

    #[test]
    fn invalidation_tracks_affected_nodes_not_previously_cached() {
        let graph = DependencyGraph::new();
        let plan = plan_for(&graph, [id(100)]);

        let mut cache = SemanticCache::<String>::new();
        let report = cache.apply_invalidation(&plan).unwrap();

        assert_eq!(report.affected_entries, 1);
        assert_eq!(cache.state(id(100)), Some(CacheState::Invalidated));
        assert_eq!(cache.get(id(100)), None);
    }

    #[test]
    fn recomputation_must_start_before_publication() {
        let mut cache = SemanticCache::new();

        let error = cache
            .publish(id(1), String::from("result"))
            .expect_err("publishing without computation must fail");

        assert_eq!(
            error,
            CacheError::InvalidTransition {
                node: id(1),
                current: None,
                operation: "publish a recomputed value",
            }
        );

        assert!(cache.is_empty());
    }

    #[test]
    fn recomputation_lifecycle_publishes_new_value() {
        let mut cache = SemanticCache::new();

        cache.begin_recompute(id(1)).unwrap();
        assert_eq!(cache.state(id(1)), Some(CacheState::Computing));
        assert_eq!(cache.get(id(1)), None);

        cache
            .publish(id(1), String::from("computed result"))
            .unwrap();

        assert_eq!(cache.state(id(1)), Some(CacheState::Valid));
        assert_eq!(
            cache.get(id(1)).map(String::as_str),
            Some("computed result")
        );
    }

    #[test]
    fn valid_entry_cannot_be_recomputed_without_invalidation() {
        let mut cache = SemanticCache::new();
        cache.insert_valid(id(1), String::from("current")).unwrap();

        let error = cache
            .begin_recompute(id(1))
            .expect_err("valid entries must first be invalidated");

        assert!(matches!(
            error,
            CacheError::InvalidTransition {
                node,
                current: Some(CacheState::Valid),
                ..
            } if node == id(1)
        ));

        assert_eq!(cache.get(id(1)).map(String::as_str), Some("current"));
    }

    #[test]
    fn failed_recomputation_does_not_expose_old_value_as_valid() {
        let mut cache = SemanticCache::new();
        cache.insert_valid(id(1), String::from("previous")).unwrap();

        let graph = DependencyGraph::new();
        let plan = plan_for(&graph, [id(1)]);
        cache.apply_invalidation(&plan).unwrap();

        cache.begin_recompute(id(1)).unwrap();
        cache.mark_failed(id(1)).unwrap();

        assert_eq!(cache.state(id(1)), Some(CacheState::Failed));
        assert_eq!(cache.get(id(1)), None);
        assert_eq!(
            cache.get_stale(id(1)).map(String::as_str),
            Some("previous")
        );
    }

    #[test]
    fn failed_entry_can_be_recomputed_again() {
        let mut cache = SemanticCache::<String>::new();

        cache.begin_recompute(id(1)).unwrap();
        cache.mark_failed(id(1)).unwrap();

        cache.begin_recompute(id(1)).unwrap();
        cache.publish(id(1), String::from("recovered")).unwrap();

        assert_eq!(
            cache.get(id(1)).map(String::as_str),
            Some("recovered")
        );
    }

    #[test]
    fn empty_invalidation_is_a_no_op() {
        let mut cache = SemanticCache::new();
        cache.insert_valid(id(1), String::from("current")).unwrap();

        let graph = DependencyGraph::new();
        let plan = plan_for(&graph, std::iter::empty());

        let previous_revision = cache.revision();
        let report = cache.apply_invalidation(&plan).unwrap();

        assert_eq!(report.affected_entries, 0);
        assert_eq!(report.retained_values, 0);
        assert_eq!(report.revision, previous_revision);
        assert_eq!(cache.revision(), previous_revision);
        assert_eq!(cache.get(id(1)).map(String::as_str), Some("current"));
    }

    #[test]
    fn duplicate_changed_nodes_do_not_duplicate_invalidation() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency(id(2), id(1)).unwrap();

        let plan = plan_for(&graph, [id(1), id(1), id(1)]);

        assert_eq!(plan.affected_count(), 2);

        let mut cache = SemanticCache::new();
        let report = cache.apply_invalidation(&plan).unwrap();

        assert_eq!(report.affected_entries, 2);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn entry_revision_tracks_mutations() {
        let mut cache = SemanticCache::new();

        cache.insert_valid(id(1), String::from("v1")).unwrap();
        let first = cache.entry_revision(id(1)).unwrap();

        let graph = DependencyGraph::new();
        let plan = plan_for(&graph, [id(1)]);
        cache.apply_invalidation(&plan).unwrap();

        let second = cache.entry_revision(id(1)).unwrap();
        assert!(second > first);

        cache.begin_recompute(id(1)).unwrap();
        let third = cache.entry_revision(id(1)).unwrap();
        assert!(third > second);
    }

    #[test]
    fn removal_returns_value_and_updates_revision() {
        let mut cache = SemanticCache::new();
        cache.insert_valid(id(1), String::from("value")).unwrap();

        let before = cache.revision();
        assert_eq!(cache.remove(id(1)).unwrap().as_deref(), Some("value"));
        assert!(cache.revision() > before);
        assert_eq!(cache.state(id(1)), None);

        let after = cache.revision();
        assert_eq!(cache.remove(id(1)).unwrap(), None);
        assert_eq!(cache.revision(), after);
    }

    #[test]
    fn clear_removes_all_states() {
        let mut cache = SemanticCache::<String>::new();

        cache.begin_recompute(id(1)).unwrap();
        cache.begin_recompute(id(2)).unwrap();

        cache.clear().unwrap();

        assert!(cache.is_empty());
        assert_eq!(cache.state(id(1)), None);
        assert_eq!(cache.state(id(2)), None);
    }

    #[test]
    fn node_iteration_is_deterministic() {
        let mut cache = SemanticCache::<String>::new();

        cache.insert_valid(id(9), String::from("nine")).unwrap();
        cache.insert_valid(id(2), String::from("two")).unwrap();
        cache.insert_valid(id(5), String::from("five")).unwrap();

        assert_eq!(
            cache.nodes().collect::<Vec<_>>(),
            vec![id(2), id(5), id(9)]
        );
    }
}
