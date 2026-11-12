
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — JIT Code Cache
//!
//! Path: `src/zur/jit/code_cache.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides a concurrent, configurable in-memory cache for verified JIT
//! compilation artifacts.
//!
//! The cache implements the `JitCodeCache` contract defined by
//! `crate::zur::jit::engine`.
//!
//! It stores opaque artifacts owned by the compiler/backend integration.
//! It does not define another artifact format, manifest, compiler IR,
//! backend registry, code generator, or executable-memory implementation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - In-memory JIT artifact retention.
//! - Cache lookup.
//! - Atomic insert-if-absent publication.
//! - Explicit invalidation.
//! - Optional entry-count budgets.
//! - Configurable eviction behavior.
//! - Cache operation statistics.
//! - Synchronization of cache state.
//!
//! It does not own:
//!
//! - JIT compilation.
//! - Compiler IR or optimization.
//! - Backend selection or code generation.
//! - Artifact manifests or serialization.
//! - Artifact integrity verification.
//! - Runtime ABI validation.
//! - Executable memory.
//! - Program execution.
//! - Hardware discovery.
//! - Physical resource reservation.
//! - Persistent cache storage.
//! - Cross-process or distributed cache consistency.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ## Integration
//!
//! `crate::zur::jit::engine`:
//!     JitCodeCache
//!     JitCacheKey
//!     CompiledJitArtifact
//!
//! `crate::compiler::artifacts`:
//!     Canonical compiler artifact model and metadata.
//!
//! `crate::backend`:
//!     Backend-produced artifact and manifest contracts.
//!
//! `crate::zur::execution`:
//!     Consumes verified artifacts for subsequent execution handling.
//!
//! `crate::zur::security`:
//!     Supplies artifact verification and security policy.
//!
//! ## Cache identity
//!
//! The compiler integration supplies the canonical JitCacheKey.
//! That key must include all inputs that can affect artifact correctness
//! or compatibility, including compiler, optimization, ABI, target,
//! provider, domain and specialization identity where applicable.
//!
//! This cache does not invent a hash or infer compatibility from a
//! source filename, domain name, or target label.
//!
//! ## Scalability
//!
//! The default configuration has no artificial entry-count ceiling.
//!
//! An optional entry-count budget can be configured by the application.
//! The cache does not claim to measure the memory occupied by an opaque
//! artifact: the existing CompiledJitArtifact contract does not expose
//! a trustworthy payload-size measurement.
//!
//! Memory budgets and byte-level accounting therefore remain with the
//! resource-management and artifact-storage authorities.
//!
//! The cache grows subject to available process resources and any
//! explicitly configured operational budget.
//!
//! No finite process provides literal infinite storage.
//!
//! ## Concurrency
//!
//! All cache mutations and publication decisions are synchronized.
//! `insert_if_absent` is atomic with respect to other cache operations.
//!
//! The implementation uses one mutex to preserve simple, auditable
//! publication and eviction invariants. It does not create threads.
//! The runtime concurrency and scheduling systems own execution policy.
//!
//! A future sharded implementation may replace the internal synchronization
//! without changing the public JitCodeCache contract.
//!
//! ## Safety and correctness
//!
//! - No unsafe Rust.
//! - No raw pointers.
//! - No global mutable state.
//! - No fabricated artifacts.
//! - No implicit compilation.
//! - No implicit artifact verification.
//! - No silent identity substitution.
//! - No automatic semantic fallback.
//! - No fixed hardware or domain inventory.
//! - No fixed default cache capacity.
//!
//! A cache hit is not proof that an artifact is valid. The JIT engine
//! must continue to verify cached artifacts before returning them.

use std::collections::HashMap;
use std::fmt;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::zur::error::{ZurError, ZurResult};

use crate::zur::jit::engine::{
    CompiledJitArtifact,
    JitCacheKey,
    JitCodeCache,
};

// =============================================================================
// COMPONENT IDENTITY
// =============================================================================

const COMPONENT: &str = "zur.jit.code_cache";

// =============================================================================
// EVICTION POLICY
// =============================================================================

/// Policy applied when an insertion would exceed the configured
/// cache entry-count budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CacheEvictionPolicy {
    /// Remove the least recently used entry to make room.
    ///
    /// This policy is meaningful only when an entry-count budget is set.
    LeastRecentlyUsed,

    /// Reject the new entry when the configured budget is exhausted.
    ///
    /// Existing entries remain unchanged.
    RejectNew,
}

// =============================================================================
// CACHE CONFIGURATION
// =============================================================================

/// Configuration for an in-memory JIT code cache.
///
/// The default configuration has no artificial entry-count limit.
/// Applications may supply a limit based on their operational
/// resource policy.
///
/// This configuration does not claim to enforce a byte-level memory
/// budget because compiled artifacts are opaque to this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JitCodeCacheConfig {
    max_entries: Option<NonZeroUsize>,
    eviction_policy: CacheEvictionPolicy,
}

impl Default for JitCodeCacheConfig {
    fn default() -> Self {
        Self {
            max_entries: None,
            eviction_policy: CacheEvictionPolicy::LeastRecentlyUsed,
        }
    }
}

impl JitCodeCacheConfig {
    /// Creates an unbounded-by-entry-count cache configuration.
    ///
    /// Actual storage remains subject to available memory and operating
    /// environment limits.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_entries: None,
            eviction_policy: CacheEvictionPolicy::LeastRecentlyUsed,
        }
    }

    /// Sets an optional maximum number of retained entries.
    ///
    /// `None` means that this cache imposes no entry-count budget.
    ///
    /// This is an operational budget, not a semantic or hardware limit.
    #[must_use]
    pub const fn with_max_entries(
        mut self,
        max_entries: Option<NonZeroUsize>,
    ) -> Self {
        self.max_entries = max_entries;
        self
    }

    /// Sets the behavior when the entry-count budget is exhausted.
    #[must_use]
    pub const fn with_eviction_policy(
        mut self,
        eviction_policy: CacheEvictionPolicy,
    ) -> Self {
        self.eviction_policy = eviction_policy;
        self
    }

    /// Returns the configured entry-count budget.
    #[must_use]
    pub const fn max_entries(self) -> Option<NonZeroUsize> {
        self.max_entries
    }

    /// Returns the configured eviction policy.
    #[must_use]
    pub const fn eviction_policy(self) -> CacheEvictionPolicy {
        self.eviction_policy
    }

    /// Validates this configuration.
    ///
    /// The type prevents a zero entry-count budget from being represented.
    /// No additional arbitrary limits are imposed.
    pub fn validate(self) -> ZurResult<()> {
        if self.max_entries.is_some()
            && self.eviction_policy == CacheEvictionPolicy::RejectNew
        {
            // This combination is valid. It is intentionally not rejected:
            // it provides a bounded cache that refuses new entries when full.
        }

        Ok(())
    }
}

// =============================================================================
// CACHE STATISTICS
// =============================================================================

/// Snapshot of cache operation statistics.
///
/// Counters saturate rather than wrap. Saturation affects diagnostic
/// precision only; it never changes cache correctness or capacity.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct JitCodeCacheStatistics {
    entries: usize,
    hits: u128,
    misses: u128,
    insertions: u128,
    invalidations: u128,
    evictions: u128,
}

impl JitCodeCacheStatistics {
    /// Returns the current number of retained entries.
    #[must_use]
    pub const fn entries(self) -> usize {
        self.entries
    }

    /// Returns the number of successful lookups.
    #[must_use]
    pub const fn hits(self) -> u128 {
        self.hits
    }

    /// Returns the number of unsuccessful lookups.
    #[must_use]
    pub const fn misses(self) -> u128 {
        self.misses
    }

    /// Returns the number of newly published entries.
    #[must_use]
    pub const fn insertions(self) -> u128 {
        self.insertions
    }

    /// Returns the number of explicit invalidations that removed entries.
    #[must_use]
    pub const fn invalidations(self) -> u128 {
        self.invalidations
    }

    /// Returns the number of entries removed by eviction.
    #[must_use]
    pub const fn evictions(self) -> u128 {
        self.evictions
    }
}

// =============================================================================
// INTERNAL CACHE ENTRY
// =============================================================================

/// One retained artifact and its most-recently-used ordering stamp.
struct CacheEntry {
    artifact: Arc<dyn CompiledJitArtifact>,
    last_used: u128,
}

impl fmt::Debug for CacheEntry {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("CacheEntry")
            .field("artifact", &self.artifact)
            .field("last_used", &self.last_used)
            .finish()
    }
}

// =============================================================================
// INTERNAL CACHE STATE
// =============================================================================

/// Mutable state protected by the cache mutex.
struct CacheState {
    entries: HashMap<JitCacheKey, CacheEntry>,
    clock: u128,
    hits: u128,
    misses: u128,
    insertions: u128,
    invalidations: u128,
    evictions: u128,
}

impl CacheState {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
            clock: 0,
            hits: 0,
            misses: 0,
            insertions: 0,
            invalidations: 0,
            evictions: 0,
        }
    }

    /// Advances the access-order clock.
    ///
    /// If the counter reaches its representational boundary, existing
    /// entries are renumbered in their current relative access order.
    /// This avoids wrapping and avoids treating a machine integer width
    /// as a lifetime limit on cache operations.
    fn next_stamp(&mut self) -> ZurResult<u128> {
        if self.clock == u128::MAX {
            self.renumber_stamps()?;
        }

        self.clock += 1;

        Ok(self.clock)
    }

    /// Reassigns access stamps while preserving their relative order.
    fn renumber_stamps(&mut self) -> ZurResult<()> {
        let mut ordered = Vec::new();

        ordered
            .try_reserve_exact(self.entries.len())
            .map_err(|error| {
                cache_resource_error(
                    "unable to reserve memory while renumbering cache access order",
                )
                .with_source(error)
            })?;

        ordered.extend(
            self.entries
                .iter()
                .map(|(key, entry)| {
                    (key.clone(), entry.last_used)
                }),
        );

        ordered.sort_unstable_by_key(|(_, stamp)| *stamp);

        for (index, (key, _)) in ordered.into_iter().enumerate() {
            let stamp = (index as u128) + 1;

            if let Some(entry) = self.entries.get_mut(&key) {
                entry.last_used = stamp;
            }
        }

        self.clock = self.entries.len() as u128;

        Ok(())
    }

    /// Returns the least recently used key, if any.
    fn least_recently_used_key(&self) -> Option<JitCacheKey> {
        self.entries
            .iter()
            .min_by_key(|(_, entry)| entry.last_used)
            .map(|(key, _)| key.clone())
    }

    fn statistics(&self) -> JitCodeCacheStatistics {
        JitCodeCacheStatistics {
            entries: self.entries.len(),
            hits: self.hits,
            misses: self.misses,
            insertions: self.insertions,
            invalidations: self.invalidations,
            evictions: self.evictions,
        }
    }
}

// =============================================================================
// CACHE IMPLEMENTATION
// =============================================================================

/// Thread-safe in-memory implementation of the ZUR JIT code-cache contract.
///
/// The cache retains opaque compiler/backend artifacts through `Arc`.
///
/// Cloning an artifact reference does not copy its payload. Invalidating
/// an entry removes the cache's reference; it does not forcibly destroy
/// an artifact still referenced by an executing or inspecting component.
///
/// The cache does not claim that invalidation terminates active execution.
pub struct InMemoryJitCodeCache {
    configuration: JitCodeCacheConfig,
    state: Mutex<CacheState>,
}

impl fmt::Debug for InMemoryJitCodeCache {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let statistics = self.statistics().ok();

        formatter
            .debug_struct("InMemoryJitCodeCache")
            .field("configuration", &self.configuration)
            .field("statistics", &statistics)
            .finish()
    }
}

impl InMemoryJitCodeCache {
    /// Constructs an in-memory cache with the supplied configuration.
    pub fn new(
        configuration: JitCodeCacheConfig,
    ) -> ZurResult<Self> {
        configuration.validate()?;

        Ok(Self {
            configuration,
            state: Mutex::new(CacheState::new()),
        })
    }

    /// Constructs a cache with the default configuration.
    ///
    /// The default configuration imposes no artificial entry-count
    /// ceiling. Applications should select an explicit operational
    /// budget when their deployment requires one.
    pub fn unbounded() -> Self {
        Self {
            configuration: JitCodeCacheConfig::default(),
            state: Mutex::new(CacheState::new()),
        }
    }

    /// Returns the immutable cache configuration.
    #[must_use]
    pub const fn configuration(&self) -> JitCodeCacheConfig {
        self.configuration
    }

    /// Returns a consistent snapshot of cache statistics.
    pub fn statistics(&self) -> ZurResult<JitCodeCacheStatistics> {
        let state = self.lock_state("statistics")?;
        Ok(state.statistics())
    }

    /// Returns whether an entry currently exists for the supplied key.
    ///
    /// This diagnostic query does not update the entry's LRU position.
    pub fn contains_key(
        &self,
        key: &JitCacheKey,
    ) -> ZurResult<bool> {
        let state = self.lock_state("contains_key")?;
        Ok(state.entries.contains_key(key))
    }

    /// Removes every retained entry.
    ///
    /// This does not terminate executions holding their own `Arc`
    /// references to previously returned artifacts.
    ///
    /// The operation is explicit. It is not performed automatically
    /// during construction, shutdown, or compilation.
    pub fn clear(&self) -> ZurResult<usize> {
        let mut state = self.lock_state("clear")?;

        let removed = state.entries.len();

        state.entries.clear();

        state.invalidations = state
            .invalidations
            .saturating_add(removed as u128);

        Ok(removed)
    }

    /// Acquires the cache state lock.
    ///
    /// A poisoned mutex is reported as an operational failure. The cache
    /// does not silently assume that invariants survived an interrupted
    /// mutation.
    fn lock_state(
        &self,
        operation: &'static str,
    ) -> ZurResult<MutexGuard<'_, CacheState>> {
        self.state.lock().map_err(|_| {
            ZurError::internal(
                "JIT code-cache state lock is poisoned",
            )
            .with_component(COMPONENT)
            .with_operation(operation)
        })
    }

    /// Validates that the supplied key identifies the artifact.
    fn validate_artifact_key(
        key: &JitCacheKey,
        artifact: &dyn CompiledJitArtifact,
    ) -> ZurResult<()> {
        if artifact.cache_key() != key {
            return Err(
                ZurError::compatibility(
                    "JIT artifact cache identity does not match insertion key",
                )
                .with_component(COMPONENT)
                .with_operation("insert_if_absent"),
            );
        }

        Ok(())
    }

    /// Makes room for one new entry according to the configured policy.
    ///
    /// The caller must already hold the cache lock and must have reserved
    /// the required HashMap capacity before invoking this method.
    fn make_room(
        &self,
        state: &mut CacheState,
    ) -> ZurResult<()> {
        let Some(max_entries) = self.configuration.max_entries() else {
            return Ok(());
        };

        if state.entries.len() < max_entries.get() {
            return Ok(());
        }

        match self.configuration.eviction_policy() {
            CacheEvictionPolicy::RejectNew => {
                Err(
                    ZurError::budget_exhausted(
                        "JIT code-cache entry budget is exhausted",
                    )
                    .with_component(COMPONENT)
                    .with_operation("insert_if_absent"),
                )
            }

            CacheEvictionPolicy::LeastRecentlyUsed => {
                let key = state
                    .least_recently_used_key()
                    .ok_or_else(|| {
                        ZurError::internal(
                            "cache capacity is exhausted but no eviction candidate exists",
                        )
                        .with_component(COMPONENT)
                        .with_operation("make_room")
                    })?;

                state.entries.remove(&key);

                state.evictions = state.evictions.saturating_add(1);

                Ok(())
            }
        }
    }
}

// =============================================================================
// JIT CODE CACHE CONTRACT
// =============================================================================

impl JitCodeCache for InMemoryJitCodeCache {
    /// Looks up an artifact using its complete canonical cache identity.
    ///
    /// A successful lookup updates its LRU position when LRU eviction
    /// is configured.
    ///
    /// The returned artifact is not implicitly considered verified.
    /// The JIT engine remains responsible for verification.
    fn get(
        &self,
        key: &JitCacheKey,
    ) -> ZurResult<Option<Arc<dyn CompiledJitArtifact>>> {
        let mut state = self.lock_state("get")?;

        let stamp = state.next_stamp()?;

        let artifact = match state.entries.get_mut(key) {
            Some(entry) => {
                entry.last_used = stamp;

                state.hits = state.hits.saturating_add(1);

                Some(Arc::clone(&entry.artifact))
            }

            None => {
                state.misses = state.misses.saturating_add(1);

                None
            }
        };

        Ok(artifact)
    }

    /// Atomically publishes an artifact if its key is absent.
    ///
    /// If an entry already exists, the existing canonical artifact is
    /// returned and is not replaced.
    ///
    /// If the entry is absent, this method applies the configured
    /// capacity policy and publishes the supplied artifact.
    ///
    /// The key/artifact identity is checked before publication.
    fn insert_if_absent(
        &self,
        key: &JitCacheKey,
        artifact: Arc<dyn CompiledJitArtifact>,
    ) -> ZurResult<Arc<dyn CompiledJitArtifact>> {
        Self::validate_artifact_key(
            key,
            artifact.as_ref(),
        )?;

        let mut state = self.lock_state("insert_if_absent")?;

        // Resolve concurrent publication before considering eviction.
        // The first successful publication remains canonical.
        if let Some(existing) = state.entries.get(key) {
            let existing = Arc::clone(&existing.artifact);

            let stamp = state.next_stamp()?;

            if let Some(entry) = state.entries.get_mut(key) {
                entry.last_used = stamp;
            }

            return Ok(existing);
        }

        // Reserve before eviction. If allocation cannot be reserved,
        // existing cache contents remain unchanged.
        state
            .entries
            .try_reserve(1)
            .map_err(|error| {
                cache_resource_error(
                    "unable to reserve storage for a JIT cache entry",
                )
                .with_source(error)
            })?;

        self.make_room(&mut state)?;

        let stamp = state.next_stamp()?;

        // The mutex remains held across the capacity decision and insert.
        // No competing writer can publish the same key between them.
        state.entries.insert(
            key.clone(),
            CacheEntry {
                artifact: Arc::clone(&artifact),
                last_used: stamp,
            },
        );

        state.insertions = state.insertions.saturating_add(1);

        Ok(artifact)
    }

    /// Invalidates one cache entry.
    ///
    /// Returns `true` if an entry was removed.
    ///
    /// An artifact already returned to another component remains alive
    /// while that component retains its own reference.
    fn invalidate(
        &self,
        key: &JitCacheKey,
    ) -> ZurResult<bool> {
        let mut state = self.lock_state("invalidate")?;

        let removed = state.entries.remove(key).is_some();

        if removed {
            state.invalidations =
                state.invalidations.saturating_add(1);
        }

        Ok(removed)
    }
}

// =============================================================================
// RESOURCE ERROR
// =============================================================================

/// Creates a structured cache-storage resource error.
fn cache_resource_error(
    message: &'static str,
) -> ZurError {
    ZurError::resource_unavailable(message)
        .with_component(COMPONENT)
        .with_operation("cache_storage")
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::jit::engine::{
        CompiledJitArtifact,
        JitCacheKey,
    };

    use std::sync::Arc;
    use std::thread;

    #[derive(Debug)]
    struct TestArtifact {
        key: JitCacheKey,
        domain: String,
        provider: String,
        format: String,
        compiler_version: String,
        marker: String,
    }

    impl TestArtifact {
        fn new(
            key: JitCacheKey,
            marker: &str,
        ) -> Self {
            Self {
                key,
                domain: "zamani.classical".to_owned(),
                provider: "test.compiler".to_owned(),
                format: "test.native".to_owned(),
                compiler_version: "1".to_owned(),
                marker: marker.to_owned(),
            }
        }
    }

    impl CompiledJitArtifact for TestArtifact {
        fn cache_key(&self) -> &JitCacheKey {
            &self.key
        }

        fn domain_identity(&self) -> &str {
            &self.domain
        }

        fn provider_identity(&self) -> &str {
            &self.provider
        }

        fn format_identity(&self) -> &str {
            &self.format
        }

        fn compiler_contract_version(&self) -> &str {
            &self.compiler_version
        }
    }

    fn key(value: &str) -> JitCacheKey {
        JitCacheKey::new(value).expect("valid test cache key")
    }

    fn artifact(
        cache_key: JitCacheKey,
        marker: &str,
    ) -> Arc<dyn CompiledJitArtifact> {
        Arc::new(TestArtifact::new(cache_key, marker))
    }

    fn unbounded_cache() -> InMemoryJitCodeCache {
        InMemoryJitCodeCache::unbounded()
    }

    #[test]
    fn default_configuration_has_no_entry_limit() {
        let configuration = JitCodeCacheConfig::default();

        assert_eq!(configuration.max_entries(), None);

        assert_eq!(
            configuration.eviction_policy(),
            CacheEvictionPolicy::LeastRecentlyUsed
        );

        assert!(configuration.validate().is_ok());
    }

    #[test]
    fn stores_and_retrieves_an_artifact() {
        let cache = unbounded_cache();
        let cache_key = key("compiler.fingerprint.one");

        let compiled = artifact(
            cache_key.clone(),
            "first",
        );

        let inserted = cache
            .insert_if_absent(
                &cache_key,
                Arc::clone(&compiled),
            )
            .expect("insertion succeeds");

        assert!(Arc::ptr_eq(&compiled, &inserted));

        let retrieved = cache
            .get(&cache_key)
            .expect("lookup succeeds")
            .expect("entry exists");

        assert!(Arc::ptr_eq(&compiled, &retrieved));
    }

    #[test]
    fn missing_key_returns_none() {
        let cache = unbounded_cache();

        assert!(
            cache
                .get(&key("compiler.missing"))
                .expect("lookup succeeds")
                .is_none()
        );
    }

    #[test]
    fn insert_if_absent_preserves_first_publication() {
        let cache = unbounded_cache();
        let cache_key = key("compiler.shared");

        let first = artifact(
            cache_key.clone(),
            "first",
        );

        let second = artifact(
            cache_key.clone(),
            "second",
        );

        let published_first = cache
            .insert_if_absent(
                &cache_key,
                Arc::clone(&first),
            )
            .expect("first publication succeeds");

        let published_second = cache
            .insert_if_absent(
                &cache_key,
                Arc::clone(&second),
            )
            .expect("existing publication is returned");

        assert!(Arc::ptr_eq(&first, &published_first));

        assert!(Arc::ptr_eq(&first, &published_second));

        assert!(!Arc::ptr_eq(&second, &published_second));
    }

    #[test]
    fn rejects_artifact_key_mismatch() {
        let cache = unbounded_cache();

        let insertion_key = key("compiler.expected");

        let wrong_artifact = artifact(
            key("compiler.different"),
            "wrong",
        );

        assert!(
            cache
                .insert_if_absent(
                    &insertion_key,
                    wrong_artifact,
                )
                .is_err()
        );

        assert_eq!(
            cache.statistics().unwrap().entries(),
            0
        );
    }

    #[test]
    fn invalidation_removes_the_cache_reference() {
        let cache = unbounded_cache();
        let cache_key = key("compiler.invalidate");

        cache
            .insert_if_absent(
                &cache_key,
                artifact(cache_key.clone(), "value"),
            )
            .unwrap();

        assert!(cache.invalidate(&cache_key).unwrap());

        assert!(!cache.invalidate(&cache_key).unwrap());

        assert!(
            cache
                .get(&cache_key)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn invalidation_does_not_destroy_external_artifact_references() {
        let cache = unbounded_cache();
        let cache_key = key("compiler.external-reference");

        let compiled = artifact(
            cache_key.clone(),
            "retained",
        );

        cache
            .insert_if_absent(
                &cache_key,
                Arc::clone(&compiled),
            )
            .unwrap();

        assert!(cache.invalidate(&cache_key).unwrap());

        // The caller still owns its independent Arc reference.
        assert_eq!(Arc::strong_count(&compiled), 1);
    }

    #[test]
    fn bounded_cache_rejects_new_entries_when_configured() {
        let configuration =
            JitCodeCacheConfig::new()
                .with_max_entries(
                    NonZeroUsize::new(1),
                )
                .with_eviction_policy(
                    CacheEvictionPolicy::RejectNew,
                );

        let cache =
            InMemoryJitCodeCache::new(configuration)
                .unwrap();

        let first_key = key("compiler.first");
        let second_key = key("compiler.second");

        cache
            .insert_if_absent(
                &first_key,
                artifact(first_key.clone(), "first"),
            )
            .unwrap();

        let result = cache.insert_if_absent(
            &second_key,
            artifact(second_key.clone(), "second"),
        );

        assert!(result.is_err());

        assert!(cache.contains_key(&first_key).unwrap());

        assert!(!cache.contains_key(&second_key).unwrap());
    }

    #[test]
    fn bounded_cache_evicts_least_recently_used_entry() {
        let configuration =
            JitCodeCacheConfig::new()
                .with_max_entries(
                    NonZeroUsize::new(2),
                )
                .with_eviction_policy(
                    CacheEvictionPolicy::LeastRecentlyUsed,
                );

        let cache =
            InMemoryJitCodeCache::new(configuration)
                .unwrap();

        let first_key = key("compiler.first");
        let second_key = key("compiler.second");
        let third_key = key("compiler.third");

        cache
            .insert_if_absent(
                &first_key,
                artifact(first_key.clone(), "first"),
            )
            .unwrap();

        cache
            .insert_if_absent(
                &second_key,
                artifact(second_key.clone(), "second"),
            )
            .unwrap();

        // Make the first entry more recently used.
        cache.get(&first_key).unwrap();

        cache
            .insert_if_absent(
                &third_key,
                artifact(third_key.clone(), "third"),
            )
            .unwrap();

        assert!(cache.contains_key(&first_key).unwrap());

        assert!(!cache.contains_key(&second_key).unwrap());

        assert!(cache.contains_key(&third_key).unwrap());

        assert_eq!(
            cache.statistics().unwrap().evictions(),
            1
        );
    }

    #[test]
    fn clear_removes_all_entries() {
        let cache = unbounded_cache();

        for name in [
            "compiler.one",
            "compiler.two",
            "compiler.three",
        ] {
            let cache_key = key(name);

            cache
                .insert_if_absent(
                    &cache_key,
                    artifact(cache_key.clone(), name),
                )
                .unwrap();
        }

        assert_eq!(cache.clear().unwrap(), 3);

        assert_eq!(
            cache.statistics().unwrap().entries(),
            0
        );
    }

    #[test]
    fn statistics_track_cache_operations() {
        let cache = unbounded_cache();
        let cache_key = key("compiler.statistics");

        cache.get(&cache_key).unwrap();

        cache
            .insert_if_absent(
                &cache_key,
                artifact(cache_key.clone(), "value"),
            )
            .unwrap();

        cache.get(&cache_key).unwrap();

        cache.invalidate(&cache_key).unwrap();

        let statistics = cache.statistics().unwrap();

        assert_eq!(statistics.entries(), 0);
        assert_eq!(statistics.hits(), 1);
        assert_eq!(statistics.misses(), 1);
        assert_eq!(statistics.insertions(), 1);
        assert_eq!(statistics.invalidations(), 1);
    }

    #[test]
    fn concurrent_publication_returns_one_canonical_artifact() {
        let cache = Arc::new(unbounded_cache());

        let cache_key = key("compiler.concurrent");

        let workers = thread::available_parallelism()
            .map(|value| value.get())
            .unwrap_or(1);

        let mut handles = Vec::new();

        for worker in 0..workers {
            let cache = Arc::clone(&cache);
            let cache_key = cache_key.clone();

            handles.push(thread::spawn(move || {
                let candidate = artifact(
                    cache_key.clone(),
                    &format!("worker-{worker}"),
                );

                cache
                    .insert_if_absent(
                        &cache_key,
                        candidate,
                    )
                    .expect("publication succeeds")
            }));
        }

        let published = handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect::<Vec<_>>();

        let canonical = cache
            .get(&cache_key)
            .unwrap()
            .expect("canonical entry exists");

        assert!(
            published
                .iter()
                .all(|candidate| Arc::ptr_eq(candidate, &canonical))
        );

        assert_eq!(
            cache.statistics().unwrap().entries(),
            1
        );
    }

    #[test]
    fn repeated_invalidation_is_idempotent() {
        let cache = unbounded_cache();
        let cache_key = key("compiler.idempotent");

        cache
            .insert_if_absent(
                &cache_key,
                artifact(cache_key.clone(), "value"),
            )
            .unwrap();

        assert!(cache.invalidate(&cache_key).unwrap());

        assert!(!cache.invalidate(&cache_key).unwrap());
    }
}
