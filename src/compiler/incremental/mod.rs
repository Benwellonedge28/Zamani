
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler — Incremental Compilation
//!
//! Path: src/compiler/incremental/mod.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Coordinate incremental compilation state, dependency invalidation,
//! artifact fingerprints, and successful artifact-cache integration.
//!
//! This module is an orchestration boundary. It does not implement
//! compilation, parsing, semantic analysis, IR construction, optimization,
//! verification, backend generation, or parallel scheduling.
//!
//! ARCHITECTURAL AUTHORITIES
//!
//! - src/zuir/incremental/dependencies.rs:
//!   Canonical dependency identities and dependency graph.
//!
//! - src/zuir/incremental/invalidation.rs:
//!   Canonical transitive invalidation planning.
//!
//! - src/zuir/incremental/cache_keys.rs:
//!   Canonical ZUIR cache-key derivation.
//!
//! - src/zuir/fingerprint/stable_hash.rs:
//!   Stable content fingerprints.
//!
//! - src/backend/cache.rs:
//!   Backend artifact cache storage and integrity verification.
//!
//! - src/compiler/parallel_build.rs:
//!   Parallel build execution.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Incremental compilation coordination.
//! - Artifact dirty/clean lifecycle.
//! - Association between artifact identities and cache keys.
//! - Coordination of dependency changes and cache invalidation.
//! - Optimistic invalidation-plan freshness validation.
//! - Compiler-facing incremental APIs.
//!
//! This module does not own:
//!
//! - Dependency graph algorithms.
//! - Content fingerprint algorithms.
//! - Cache storage algorithms.
//! - Source-language semantics.
//! - Grammar or parser semantics.
//! - AST construction.
//! - Type inference or ownership checking.
//! - Canonical IR definitions.
//! - Optimization.
//! - Backend code generation.
//! - Physical resource allocation.
//! - Runtime execution.
//! - Parallel build scheduling.
//!
//! SCALABILITY
//!
//! There are no fixed limits on:
//!
//! - Artifacts.
//! - Dependencies.
//! - Modules.
//! - Operations.
//! - Quantum resources.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - Distributed nodes.
//! - Tensor dimensions.
//!
//! Collections grow according to available process resources.
//!
//! Actual operating-system and allocation limits still apply.
//!
//! SAFETY
//!
//! No unsafe Rust is permitted.
//!
//! CONCURRENCY
//!
//! The coordinator protects its graph and associated incremental metadata
//! with one mutex. This ensures that graph mutation, fingerprint updates,
//! dirty-state changes, and invalidation planning observe a consistent
//! coordinator state.
//!
//! Cache operations are delegated to BackendCache.
//!
//! Lock poisoning is recovered because coordinator operations do not invoke
//! caller-provided code while holding the lock.
//!
//! INVALIDATION
//!
//! If A depends on B, a change to B invalidates A.
//!
//! Invalidation plans describe stale artifacts, not rebuild order.
//! Actual dependency-aware rebuild scheduling belongs elsewhere.
//!
//! CLEAN STATE
//!
//! An artifact may be marked clean only after its corresponding compilation
//! and external artifact commit have succeeded.
//!
//! Cache admission is an optimization, not a correctness requirement.
//! A rejected cache insertion must never turn a failed compilation into
//! a successful one.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::backend::cache::{
    BackendCache,
    CacheConfig,
    CacheKey,
    InsertOutcome,
};

use crate::zuir::fingerprint::stable_hash::Fingerprint;

use crate::zuir::incremental::{
    DependencyError,
    DependencyGraph,
    DependencyKey,
    DependencyKind,
    InvalidationPlan,
    InvalidationPlanner,
    InvalidationError,
};

// -----------------------------------------------------------------------------
// Public result types
// -----------------------------------------------------------------------------

/// Result returned by incremental compilation operations.
pub type IncrementalResult<T> = Result<T, IncrementalError>;

/// Errors produced by the compiler incremental subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncrementalError {
    /// An artifact identity is invalid.
    InvalidArtifactIdentity {
        identity: String,
    },

    /// The requested artifact is not registered.
    UnknownArtifact {
        identity: String,
    },

    /// An artifact has no recorded content fingerprint.
    MissingFingerprint {
        identity: String,
    },

    /// A dependency graph operation failed.
    Dependency {
        message: String,
    },

    /// Invalidation planning failed.
    Invalidation {
        message: String,
    },

    /// An invalidation plan was generated against an older graph generation.
    StaleInvalidationPlan {
        planned_generation: u64,
        current_generation: u64,
    },

    /// A required compilation identity was incomplete.
    InvalidCompilationIdentity {
        message: String,
    },
}

impl fmt::Display for IncrementalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArtifactIdentity { identity } => {
                write!(
                    f,
                    "invalid incremental artifact identity: {identity}"
                )
            }

            Self::UnknownArtifact { identity } => {
                write!(
                    f,
                    "incremental artifact is not registered: {identity}"
                )
            }

            Self::MissingFingerprint { identity } => {
                write!(
                    f,
                    "incremental artifact has no fingerprint: {identity}"
                )
            }

            Self::Dependency { message } => {
                write!(
                    f,
                    "incremental dependency error: {message}"
                )
            }

            Self::Invalidation { message } => {
                write!(
                    f,
                    "incremental invalidation error: {message}"
                )
            }

            Self::StaleInvalidationPlan {
                planned_generation,
                current_generation,
            } => {
                write!(
                    f,
                    "stale invalidation plan: planned against generation \
                     {planned_generation}, current generation is \
                     {current_generation}"
                )
            }

            Self::InvalidCompilationIdentity { message } => {
                write!(
                    f,
                    "invalid compilation identity: {message}"
                )
            }
        }
    }
}

impl std::error::Error for IncrementalError {}

impl From<DependencyError> for IncrementalError {
    fn from(error: DependencyError) -> Self {
        Self::Dependency {
            message: error.to_string(),
        }
    }
}

impl From<InvalidationError> for IncrementalError {
    fn from(error: InvalidationError) -> Self {
        Self::Invalidation {
            message: error.to_string(),
        }
    }
}

// -----------------------------------------------------------------------------
// Artifact state
// -----------------------------------------------------------------------------

/// Current incremental state of an artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactStatus {
    /// The artifact has not yet been successfully compiled and committed.
    Dirty,

    /// The artifact corresponds to its currently recorded inputs.
    Clean,
}

/// Compiler-facing artifact information.
///
/// Fingerprints represent content identity, not physical resource identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactState {
    identity: DependencyKey,
    fingerprint: Option<Fingerprint>,
    status: ArtifactStatus,
}

impl ArtifactState {
    /// Return the artifact identity.
    #[must_use]
    pub fn identity(&self) -> &DependencyKey {
        &self.identity
    }

    /// Return the current fingerprint, if one has been recorded.
    #[must_use]
    pub const fn fingerprint(&self) -> Option<Fingerprint> {
        self.fingerprint
    }

    /// Return the current compilation state.
    #[must_use]
    pub const fn status(&self) -> ArtifactStatus {
        self.status
    }

    /// Whether this artifact requires recompilation.
    #[must_use]
    pub const fn is_dirty(&self) -> bool {
        matches!(self.status, ArtifactStatus::Dirty)
    }
}

// -----------------------------------------------------------------------------
// Invalidation result
// -----------------------------------------------------------------------------

/// Result of processing one or more changed compilation inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidationResultInfo {
    /// Canonical invalidation plan.
    plan: InvalidationPlan,

    /// Number of cache entries actually removed.
    invalidated_cache_entries: usize,

    /// Number of distinct artifacts marked dirty.
    dirty_artifacts: usize,
}

impl InvalidationResultInfo {
    /// Return the underlying canonical invalidation plan.
    #[must_use]
    pub fn plan(&self) -> &InvalidationPlan {
        &self.plan
    }

    /// Return the affected artifacts.
    #[must_use]
    pub fn invalidated_artifacts(&self) -> &[DependencyKey] {
        self.plan.invalidated()
    }

    /// Number of cache entries removed.
    #[must_use]
    pub const fn invalidated_cache_entries(&self) -> usize {
        self.invalidated_cache_entries
    }

    /// Number of artifacts marked dirty.
    #[must_use]
    pub const fn dirty_artifacts(&self) -> usize {
        self.dirty_artifacts
    }
}

// -----------------------------------------------------------------------------
// Internal coordinator state
// -----------------------------------------------------------------------------

#[derive(Debug, Default)]
struct CoordinatorState {
    graph: DependencyGraph,

    artifacts: BTreeMap<DependencyKey, ArtifactState>,

    cache_keys: BTreeMap<DependencyKey, CacheKey>,

    dirty: BTreeSet<DependencyKey>,
}

// -----------------------------------------------------------------------------
// Incremental compiler coordinator
// -----------------------------------------------------------------------------

/// Thread-safe compiler incremental compilation coordinator.
///
/// This type coordinates the canonical ZUIR dependency graph and the
/// authoritative backend artifact cache.
///
/// It does not execute compilation itself.
pub struct IncrementalCompiler {
    state: Mutex<CoordinatorState>,

    cache: Arc<BackendCache>,

    planner: InvalidationPlanner,
}

impl fmt::Debug for IncrementalCompiler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = self.lock_state();

        f.debug_struct("IncrementalCompiler")
            .field("artifact_count", &state.artifacts.len())
            .field("dirty_count", &state.dirty.len())
            .field("dependency_generation", &state.graph.generation())
            .finish()
    }
}

impl IncrementalCompiler {
    /// Construct a coordinator using an existing backend cache.
    ///
    /// Sharing the cache allows the regular backend compilation path and
    /// incremental compilation path to use the same artifact storage.
    #[must_use]
    pub fn new(cache: Arc<BackendCache>) -> Self {
        Self {
            state: Mutex::new(CoordinatorState::default()),
            cache,
            planner: InvalidationPlanner::new(),
        }
    }

    /// Construct a coordinator with an explicitly configured backend cache.
    #[must_use]
    pub fn with_cache_config(config: CacheConfig) -> Self {
        Self::new(Arc::new(BackendCache::new(config)))
    }

    /// Construct a coordinator with caching disabled.
    ///
    /// Incremental dependency tracking remains functional.
    #[must_use]
    pub fn without_cache() -> Self {
        Self::with_cache_config(CacheConfig::default())
    }

    /// Return the shared backend cache.
    #[must_use]
    pub fn cache(&self) -> &Arc<BackendCache> {
        &self.cache
    }

    // -------------------------------------------------------------------------
    // Artifact registration
    // -------------------------------------------------------------------------

    /// Register an artifact with the incremental dependency system.
    ///
    /// New artifacts begin dirty because no successful compilation has
    /// yet been committed.
    ///
    /// Re-registering an existing artifact is idempotent.
    pub fn register_artifact(
        &self,
        identity: impl Into<String>,
    ) -> IncrementalResult<DependencyKey> {
        let key = DependencyKey::new(identity.into())?;

        let mut state = self.lock_state();

        if state.graph.contains_node(&key) {
            return Ok(key);
        }

        state.graph.add_node(key.clone())?;

        state.artifacts.insert(
            key.clone(),
            ArtifactState {
                identity: key.clone(),
                fingerprint: None,
                status: ArtifactStatus::Dirty,
            },
        );

        state.dirty.insert(key.clone());

        Ok(key)
    }

    /// Return an artifact's incremental state.
    pub fn artifact(
        &self,
        identity: &DependencyKey,
    ) -> IncrementalResult<ArtifactState> {
        let state = self.lock_state();

        state
            .artifacts
            .get(identity)
            .cloned()
            .ok_or_else(|| IncrementalError::UnknownArtifact {
                identity: identity.as_str().to_owned(),
            })
    }

    /// Return the total number of registered compiler artifacts.
    #[must_use]
    pub fn artifact_count(&self) -> usize {
        self.lock_state().artifacts.len()
    }

    /// Return the dependency graph generation.
    #[must_use]
    pub fn graph_generation(&self) -> u64 {
        self.lock_state().graph.generation()
    }

    // -------------------------------------------------------------------------
    // Dependency management
    // -------------------------------------------------------------------------

    /// Register a dependency.
    ///
    /// `dependent` requires `prerequisite`.
    ///
    /// Changing the prerequisite invalidates the dependent.
    ///
    /// Both graph nodes are registered automatically by the authoritative
    /// ZUIR dependency graph.
    pub fn add_dependency(
        &self,
        dependent: &DependencyKey,
        prerequisite: &DependencyKey,
        kind: DependencyKind,
    ) -> IncrementalResult<bool> {
        let mut state = self.lock_state();

        let changed = state.graph.add_dependency(
            dependent.clone(),
            prerequisite.clone(),
            kind,
        )?;

        if changed {
            ensure_artifact(&mut state, dependent);
            ensure_artifact(&mut state, prerequisite);

            mark_dirty(&mut state, dependent);
        }

        Ok(changed)
    }

    /// Remove one dependency relationship.
    ///
    /// Removing a dependency can change the compilation result, so the
    /// dependent is marked dirty.
    pub fn remove_dependency(
        &self,
        dependent: &DependencyKey,
        prerequisite: &DependencyKey,
        kind: DependencyKind,
    ) -> IncrementalResult<bool> {
        let mut state = self.lock_state();

        let removed = state.graph.remove_dependency(
            dependent,
            prerequisite,
            kind,
        )?;

        if removed {
            mark_dirty(&mut state, dependent);
        }

        Ok(removed)
    }

    // -------------------------------------------------------------------------
    // Fingerprint management
    // -------------------------------------------------------------------------

    /// Record the current fingerprint of an artifact's compilation input.
    ///
    /// If the fingerprint changes, the artifact and all transitive
    /// dependents are invalidated.
    ///
    /// Fingerprint generation itself belongs to the canonical fingerprint
    /// subsystem. This method only consumes its result.
    pub fn update_fingerprint(
        &self,
        identity: &DependencyKey,
        fingerprint: Fingerprint,
    ) -> IncrementalResult<Option<InvalidationResultInfo>> {
        let mut state = self.lock_state();

        if !state.graph.contains_node(identity) {
            return Err(IncrementalError::UnknownArtifact {
                identity: identity.as_str().to_owned(),
            });
        }

        ensure_artifact(&mut state, identity);

        let previous = state
            .artifacts
            .get(identity)
            .and_then(|artifact| artifact.fingerprint);

        if previous == Some(fingerprint) {
            return Ok(None);
        }

        let plan = self.planner.plan(
            &state.graph,
            identity,
        )?;

        let result = apply_plan_locked(
            &mut state,
            &self.cache,
            plan,
        )?;

        if let Some(artifact) = state.artifacts.get_mut(identity) {
            artifact.fingerprint = Some(fingerprint);
        }

        Ok(Some(result))
    }

    /// Retrieve an artifact's current fingerprint.
    pub fn fingerprint(
        &self,
        identity: &DependencyKey,
    ) -> IncrementalResult<Fingerprint> {
        let state = self.lock_state();

        let artifact = state
            .artifacts
            .get(identity)
            .ok_or_else(|| IncrementalError::UnknownArtifact {
                identity: identity.as_str().to_owned(),
            })?;

        artifact
            .fingerprint
            .ok_or_else(|| IncrementalError::MissingFingerprint {
                identity: identity.as_str().to_owned(),
            })
    }

    // -------------------------------------------------------------------------
    // Invalidation planning
    // -------------------------------------------------------------------------

    /// Create an invalidation plan for one changed artifact.
    ///
    /// The plan does not mutate the graph, cache, or dirty state.
    pub fn plan_invalidation(
        &self,
        changed: &DependencyKey,
    ) -> IncrementalResult<InvalidationPlan> {
        let state = self.lock_state();

        Ok(self.planner.plan(
            &state.graph,
            changed,
        )?)
    }

    /// Create one invalidation plan for multiple changed artifacts.
    ///
    /// The authoritative ZUIR planner handles root deduplication,
    /// deterministic ordering, transitive traversal, and cycle safety.
    pub fn plan_invalidation_many<I>(
        &self,
        changed: I,
    ) -> IncrementalResult<InvalidationPlan>
    where
        I: IntoIterator<Item = DependencyKey>,
    {
        let state = self.lock_state();

        Ok(self.planner.plan_many(
            &state.graph,
            changed,
        )?)
    }

    /// Apply a previously generated invalidation plan.
    ///
    /// A plan generated against an older graph generation is rejected.
    ///
    /// Cache invalidation and dirty-state changes are performed together
    /// while coordinator state is locked.
    pub fn apply_invalidation(
        &self,
        plan: InvalidationPlan,
    ) -> IncrementalResult<InvalidationResultInfo> {
        let mut state = self.lock_state();

        apply_plan_locked(
            &mut state,
            &self.cache,
            plan,
        )
    }

    // -------------------------------------------------------------------------
    // Cache operations
    // -------------------------------------------------------------------------

    /// Retrieve a previously committed artifact from the backend cache.
    ///
    /// Dirty artifacts are never returned as cache hits.
    ///
    /// The caller must still validate artifact format, target compatibility,
    /// and any applicable verification requirements before using the bytes.
    #[must_use]
    pub fn get_cached(
        &self,
        identity: &DependencyKey,
    ) -> Option<Arc<[u8]>> {
        let state = self.lock_state();

        if state.dirty.contains(identity) {
            return None;
        }

        let key = state.cache_keys.get(identity)?;

        self.cache.get(key)
    }

    /// Commit a successfully compiled and externally committed artifact.
    ///
    /// IMPORTANT:
    ///
    /// Call this only after:
    ///
    /// 1. Compilation has succeeded.
    /// 2. Required semantic and IR verification has succeeded.
    /// 3. Backend generation has succeeded.
    /// 4. The artifact has been successfully committed to its authoritative
    ///    artifact destination.
    ///
    /// Cache admission may be rejected because of resource budgets.
    /// That does not invalidate the successfully committed artifact.
    ///
    /// `fingerprint` must identify the complete inputs relevant to this
    /// artifact, including relevant dependency and configuration changes.
    pub fn commit_successful_artifact(
        &self,
        identity: &DependencyKey,
        fingerprint: Fingerprint,
        cache_key: CacheKey,
        artifact_bytes: Vec<u8>,
    ) -> IncrementalResult<InsertOutcome> {
        let mut state = self.lock_state();

        if !state.graph.contains_node(identity) {
            return Err(IncrementalError::UnknownArtifact {
                identity: identity.as_str().to_owned(),
            });
        }

        // Cache only successful, already-verified artifacts.
        let outcome = self.cache.insert(
            cache_key,
            artifact_bytes,
        );

        let artifact = state
            .artifacts
            .get_mut(identity)
            .ok_or_else(|| IncrementalError::UnknownArtifact {
                identity: identity.as_str().to_owned(),
            })?;

        artifact.fingerprint = Some(fingerprint);
        artifact.status = ArtifactStatus::Clean;

        state.dirty.remove(identity);

        match outcome {
            InsertOutcome::Inserted | InsertOutcome::Replaced => {
                state.cache_keys.insert(
                    identity.clone(),
                    cache_key,
                );
            }

            InsertOutcome::RejectedDisabled
            | InsertOutcome::RejectedTooLarge
            | InsertOutcome::RejectedSizeOverflow => {
                state.cache_keys.remove(identity);
            }
        }

        Ok(outcome)
    }

    /// Mark an artifact clean after an external successful artifact commit.
    ///
    /// This method does not insert cache bytes.
    ///
    /// It is useful when the artifact is persisted or managed by a separate
    /// artifact store.
    pub fn mark_clean_after_commit(
        &self,
        identity: &DependencyKey,
        fingerprint: Fingerprint,
    ) -> IncrementalResult<()> {
        let mut state = self.lock_state();

        if !state.graph.contains_node(identity) {
            return Err(IncrementalError::UnknownArtifact {
                identity: identity.as_str().to_owned(),
            });
        }

        ensure_artifact(&mut state, identity);

        let artifact = state
            .artifacts
            .get_mut(identity)
            .expect("artifact was registered above");

        artifact.fingerprint = Some(fingerprint);
        artifact.status = ArtifactStatus::Clean;

        state.dirty.remove(identity);

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Dirty-state management
    // -------------------------------------------------------------------------

    /// Mark an artifact dirty.
    pub fn mark_dirty(
        &self,
        identity: &DependencyKey,
    ) -> IncrementalResult<()> {
        let mut state = self.lock_state();

        if !state.graph.contains_node(identity) {
            return Err(IncrementalError::UnknownArtifact {
                identity: identity.as_str().to_owned(),
            });
        }

        mark_dirty(&mut state, identity);

        Ok(())
    }

    /// Mark all registered artifacts dirty.
    ///
    /// This is useful when a global compilation contract changes, such as
    /// a compiler version, grammar contract, or semantic environment.
    pub fn mark_all_dirty(&self) {
        let mut state = self.lock_state();

        let identities: Vec<DependencyKey> =
            state.artifacts.keys().cloned().collect();

        for identity in identities {
            mark_dirty(&mut state, &identity);
        }
    }

    /// Return whether an artifact is dirty.
    #[must_use]
    pub fn is_dirty(
        &self,
        identity: &DependencyKey,
    ) -> bool {
        self.lock_state().dirty.contains(identity)
    }

    /// Return all dirty artifact identities in deterministic order.
    #[must_use]
    pub fn dirty_artifacts(&self) -> Vec<DependencyKey> {
        self.lock_state()
            .dirty
            .iter()
            .cloned()
            .collect()
    }

    /// Number of dirty artifacts.
    #[must_use]
    pub fn dirty_count(&self) -> usize {
        self.lock_state().dirty.len()
    }

    // -------------------------------------------------------------------------
    // Artifact removal
    // -------------------------------------------------------------------------

    /// Remove an artifact from incremental tracking.
    ///
    /// Before removal, its transitive dependents are invalidated.
    ///
    /// The graph removal itself is delegated to the canonical dependency
    /// graph implementation.
    pub fn remove_artifact(
        &self,
        identity: &DependencyKey,
    ) -> IncrementalResult<bool> {
        let mut state = self.lock_state();

        if !state.graph.contains_node(identity) {
            return Ok(false);
        }

        let plan = self.planner.plan(
            &state.graph,
            identity,
        )?;

        apply_plan_locked(
            &mut state,
            &self.cache,
            plan,
        )?;

        if let Some(cache_key) = state.cache_keys.remove(identity) {
            self.cache.invalidate(&cache_key);
        }

        state.artifacts.remove(identity);
        state.dirty.remove(identity);

        state.graph.remove_node(identity)?;

        Ok(true)
    }

    // -------------------------------------------------------------------------
    // Internal synchronization
    // -------------------------------------------------------------------------

    fn lock_state(&self) -> MutexGuard<'_, CoordinatorState> {
        match self.state.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }
}

// -----------------------------------------------------------------------------
// Internal helpers
// -----------------------------------------------------------------------------

fn ensure_artifact(
    state: &mut CoordinatorState,
    identity: &DependencyKey,
) {
    state
        .artifacts
        .entry(identity.clone())
        .or_insert_with(|| ArtifactState {
            identity: identity.clone(),
            fingerprint: None,
            status: ArtifactStatus::Dirty,
        });

    state.dirty.insert(identity.clone());
}

fn mark_dirty(
    state: &mut CoordinatorState,
    identity: &DependencyKey,
) {
    ensure_artifact(state, identity);

    if let Some(artifact) = state.artifacts.get_mut(identity) {
        artifact.status = ArtifactStatus::Dirty;
    }
}

fn apply_plan_locked(
    state: &mut CoordinatorState,
    cache: &BackendCache,
    plan: InvalidationPlan,
) -> IncrementalResult<InvalidationResultInfo> {
    let current_generation = state.graph.generation();
    let planned_generation = plan.graph_generation();

    if planned_generation != current_generation {
        return Err(IncrementalError::StaleInvalidationPlan {
            planned_generation,
            current_generation,
        });
    }

    let affected: Vec<DependencyKey> =
        plan.invalidated().to_vec();

    let mut invalidated_cache_entries = 0usize;

    for identity in &affected {
        if let Some(cache_key) = state.cache_keys.remove(identity) {
            if cache.invalidate(&cache_key) {
                invalidated_cache_entries =
                    invalidated_cache_entries.saturating_add(1);
            }
        }

        mark_dirty(state, identity);
    }

    let dirty_artifacts = affected.len();

    Ok(InvalidationResultInfo {
        plan,
        invalidated_cache_entries,
        dirty_artifacts,
    })
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::fingerprint::stable_hash::Fingerprint;

    fn fingerprint(value: u8) -> Fingerprint {
        let mut bytes = [0_u8; 32];
        bytes[0] = value;
        Fingerprint::from_bytes(bytes)
    }

    fn coordinator() -> IncrementalCompiler {
        IncrementalCompiler::without_cache()
    }

    #[test]
    fn new_coordinator_starts_empty() {
        let compiler = coordinator();

        assert_eq!(compiler.artifact_count(), 0);
        assert_eq!(compiler.dirty_count(), 0);
    }

    #[test]
    fn registered_artifacts_start_dirty() {
        let compiler = coordinator();

        let key = compiler
            .register_artifact("source:main.zm")
            .expect("artifact registration");

        assert!(compiler.is_dirty(&key));

        let artifact = compiler
            .artifact(&key)
            .expect("artifact state");

        assert_eq!(artifact.status(), ArtifactStatus::Dirty);
        assert_eq!(artifact.fingerprint(), None);
    }

    #[test]
    fn duplicate_registration_is_idempotent() {
        let compiler = coordinator();

        let first = compiler
            .register_artifact("source:main.zm")
            .expect("first registration");

        let second = compiler
            .register_artifact("source:main.zm")
            .expect("second registration");

        assert_eq!(first, second);
        assert_eq!(compiler.artifact_count(), 1);
    }

    #[test]
    fn dependency_changes_invalidate_dependents() {
        let compiler = coordinator();

        let source = compiler
            .register_artifact("source:main.zm")
            .expect("source");

        let semantic = compiler
            .register_artifact("semantic:main")
            .expect("semantic");

        let ir = compiler
            .register_artifact("ir:main")
            .expect("IR");

        compiler
            .add_dependency(
                &semantic,
                &source,
                DependencyKind::Source,
            )
            .expect("semantic dependency");

        compiler
            .add_dependency(
                &ir,
                &semantic,
                DependencyKind::Semantic,
            )
            .expect("IR dependency");

        let plan = compiler
            .plan_invalidation(&source)
            .expect("invalidation plan");

        assert!(plan.contains(&source));
        assert!(plan.contains(&semantic));
        assert!(plan.contains(&ir));
    }

    #[test]
    fn invalidation_marks_transitive_dependents_dirty() {
        let compiler = coordinator();

        let source = compiler
            .register_artifact("source:main.zm")
            .expect("source");

        let semantic = compiler
            .register_artifact("semantic:main")
            .expect("semantic");

        let ir = compiler
            .register_artifact("ir:main")
            .expect("IR");

        compiler
            .add_dependency(
                &semantic,
                &source,
                DependencyKind::Source,
            )
            .expect("dependency");

        compiler
            .add_dependency(
                &ir,
                &semantic,
                DependencyKind::Semantic,
            )
            .expect("dependency");

        compiler
            .apply_invalidation(
                compiler
                    .plan_invalidation(&source)
                    .expect("plan"),
            )
            .expect("apply");

        assert!(compiler.is_dirty(&source));
        assert!(compiler.is_dirty(&semantic));
        assert!(compiler.is_dirty(&ir));
    }

    #[test]
    fn unchanged_fingerprint_does_not_invalidate() {
        let compiler = coordinator();

        let key = compiler
            .register_artifact("source:main.zm")
            .expect("registration");

        compiler
            .update_fingerprint(&key, fingerprint(1))
            .expect("first fingerprint");

        let result = compiler
            .update_fingerprint(&key, fingerprint(1))
            .expect("unchanged fingerprint");

        assert!(result.is_none());
    }

    #[test]
    fn changed_fingerprint_invalidates_artifact() {
        let compiler = coordinator();

        let key = compiler
            .register_artifact("source:main.zm")
            .expect("registration");

        compiler
            .update_fingerprint(&key, fingerprint(1))
            .expect("first fingerprint");

        let result = compiler
            .update_fingerprint(&key, fingerprint(2))
            .expect("changed fingerprint");

        assert!(result.is_some());
        assert!(compiler.is_dirty(&key));
    }

    #[test]
    fn multiple_roots_are_supported() {
        let compiler = coordinator();

        let first = compiler
            .register_artifact("source:first.zm")
            .expect("first");

        let second = compiler
            .register_artifact("source:second.zm")
            .expect("second");

        let plan = compiler
            .plan_invalidation_many(vec![
                first.clone(),
                second.clone(),
                first.clone(),
            ])
            .expect("multi-root plan");

        assert!(plan.contains(&first));
        assert!(plan.contains(&second));
        assert_eq!(plan.changed_roots().len(), 2);
    }

    #[test]
    fn unknown_artifacts_are_not_silently_accepted() {
        let compiler = coordinator();

        let unknown = DependencyKey::new("source:unknown.zm")
            .expect("valid identity");

        assert!(compiler.plan_invalidation(&unknown).is_err());
    }

    #[test]
    fn dirty_cache_entries_are_never_returned() {
        let compiler = coordinator();

        let key = compiler
            .register_artifact("source:main.zm")
            .expect("registration");

        assert!(compiler.get_cached(&key).is_none());
    }

    #[test]
    fn removing_unknown_artifact_is_idempotent() {
        let compiler = coordinator();

        let key = DependencyKey::new("source:missing.zm")
            .expect("identity");

        assert!(!compiler
            .remove_artifact(&key)
            .expect("removal"));
    }
}
