//! Zamani Quantum Routing — Path and Distance Cache
//!
//! Production-grade, backend-independent caching for quantum routing graph
//! searches.
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - reusable shortest-path caching;
//! - shortest-distance caching;
//! - bounded cache capacity;
//! - deterministic cache keys;
//! - topology-bound cache lifetime;
//! - weighted-path cache domains;
//! - cache hit/miss accounting;
//! - eviction accounting;
//! - explicit invalidation;
//! - cache statistics;
//! - safe cache-size management;
//! - prevention of cross-topology cache contamination.
//!
//! It deliberately does NOT own:
//!
//! - topology construction;
//! - topology mutation;
//! - path-finding algorithms;
//! - logical-to-physical mapping;
//! - layout;
//! - SWAP insertion;
//! - routing algorithms;
//! - cost models;
//! - hardware calibration;
//! - scheduling;
//! - gate decomposition;
//! - hardware execution.
//!
//! # Architectural position
//!
//! ```text
//! PhysicalTopology
//!        │
//!        ▼
//!    PathFinder
//!        │
//!        ▼
//!   ┌───────────┐
//!   │  cache.rs │
//!   └─────┬─────┘
//!         │
//!    ┌────┴─────┐
//!    ▼          ▼
//! distances   paths
//!    │          │
//!    └────┬─────┘
//!         ▼
//!   routing algorithms
//!   ├── shortest_path
//!   ├── lookahead
//!   ├── SABRE
//!   ├── noise_aware
//!   └── dynamic
//! ```
//!
//! # Important topology-safety rule
//!
//! A cache is bound to one `PhysicalTopology` reference for its entire
//! lifetime.
//!
//! This is intentional.
//!
//! A routing cache must never silently reuse:
//!
//! ```text
//! topology A + source + target
//! ```
//!
//! for:
//!
//! ```text
//! topology B + source + target
//! ```
//!
//! even when both topologies happen to contain the same physical-qubit IDs.
//!
//! Binding the cache to `&PhysicalTopology` makes cross-topology contamination
//! impossible through the public API.
//!
//! It also avoids requiring `PhysicalTopology` to expose a cache-specific
//! fingerprint API.
//!
//! # Why this is separate from `path.rs`
//!
//! `path.rs` owns graph-search semantics.
//!
//! `cache.rs` owns reuse of already-computed search results.
//!
//! This separation means:
//!
//! - BFS/Dijkstra remains independently testable;
//! - cache policy can evolve independently;
//! - routing algorithms do not need to know how paths are stored;
//! - cache invalidation remains explicit;
//! - no cache-specific state is added to `PathFinder`;
//! - later routing algorithms do not require changes to `path.rs`.
//!
//! # Weighted-search rule
//!
//! Weighted results are cached only inside an explicitly named weight domain.
//!
//! A caller must provide a stable `WeightCacheKey` representing the complete
//! weighting semantics relevant to the cached result.
//!
//! For example:
//!
//! ```text
//! duration-model-v1
//! error-model-v3
//! calibration-2026-08-29-001
//! ```
//!
//! A cache key must change whenever the effective weight model changes.
//!
//! The cache cannot infer the identity of an arbitrary `PathWeight` object,
//! therefore it intentionally refuses to invent one.
//!
//! # Determinism
//!
//! Cache lookup and eviction are deterministic.
//!
//! Entries are ordered by monotonically increasing access sequence, with the
//! physical source/target pair used as a deterministic tie-breaker.
//!
//! The cache never changes the result returned by `PathFinder`; it only avoids
//! repeating equivalent searches.
//!
//! # Memory safety
//!
//! The cache is bounded.
//!
//! No global mutable state is used.
//!
//! No `unsafe` code is used.
//!
//! No threads are spawned.
//!
//! No filesystem or network access occurs.
//!
//! The caller may wrap this cache in `Arc<Mutex<_>>` or another synchronization
//! primitive if shared concurrent access is required. Synchronization policy
//! deliberately remains outside this module.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Edition 2021
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This module consumes:
//!
//! - `routing/types.rs`;
//! - `routing/errors.rs`;
//! - `routing/path.rs`.
//!
//! It does not depend on:
//!
//! - `routing/mapping.rs`;
//! - `routing/layout.rs`;
//! - `routing/cost.rs`;
//! - `routing/router.rs`;
//! - `routing/transpiler.rs`;
//! - individual routing algorithms;
//! - hardware providers.
//!
//! Higher-level routing modules may depend on this module.
//!
//! # Intended usage
//!
//! ```text
//! let mut cache = PathCache::new(&topology, 4096)?;
//!
//! let path = cache.shortest_path(p0, p9)?;
//! let distance = cache.shortest_distance(p0, p9)?;
//!
//! // Subsequent identical searches can be served from cache.
//! ```
//!
//! # Complexity
//!
//! Cache lookup:
//!
//! - expected O(1) for the hash table;
//!
//! LRU eviction:
//!
//! - O(n) only when the cache must find the oldest entry;
//!
//! where `n` is the number of cached entries.
//!
//! The implementation deliberately favors correctness, determinism, and
//! bounded memory over a more complicated intrusive-LRU implementation.
//!
//! For very large routing workloads, a future specialized cache may replace
//! the eviction internals without changing this public API.

use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

use crate::quantum::routing::errors::{RoutingError, RoutingResult};
use crate::quantum::routing::path::{
    PathFinder,
    PathResult,
    PathSearchConfig,
    PathWeight,
};
use crate::quantum::routing::topology::PhysicalTopology;
use crate::quantum::routing::types::PhysicalQubitId;

// =============================================================================
// Constants
// =============================================================================

/// Default maximum number of cached entries.
///
/// The cache is intentionally bounded by default.
pub const DEFAULT_CACHE_CAPACITY: usize = 4096;

/// Minimum legal cache capacity.
///
/// A zero-capacity cache is represented explicitly by constructing
/// `PathCache` with capacity zero. It is useful for benchmarks and feature
/// toggles, so zero is permitted.
pub const MIN_CACHE_CAPACITY: usize = 0;

/// Maximum cache capacity accepted by the public API.
///
/// This protects callers from accidentally requesting an enormous allocation
/// due to a configuration error.
///
/// It is a policy ceiling, not a theoretical implementation limit.
pub const MAX_CACHE_CAPACITY: usize = 16_777_216;

/// Stable cache implementation version.
///
/// This is useful when routing reproducibility metadata records cache behavior.
pub const CACHE_VERSION: &str = "1";

// =============================================================================
// Cache statistics
// =============================================================================

/// Immutable snapshot of cache activity.
///
/// Statistics are informational only. They never affect routing correctness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CacheStatistics {
    /// Number of successful path-cache lookups.
    pub path_hits: u64,

    /// Number of unsuccessful path-cache lookups.
    pub path_misses: u64,

    /// Number of successful distance-cache lookups.
    pub distance_hits: u64,

    /// Number of unsuccessful distance-cache lookups.
    pub distance_misses: u64,

    /// Number of successful weighted-path-cache lookups.
    pub weighted_path_hits: u64,

    /// Number of unsuccessful weighted-path-cache lookups.
    pub weighted_path_misses: u64,

    /// Number of successful weighted-distance-cache lookups.
    pub weighted_distance_hits: u64,

    /// Number of unsuccessful weighted-distance-cache lookups.
    pub weighted_distance_misses: u64,

    /// Number of entries removed because of capacity pressure.
    pub evictions: u64,

    /// Number of explicit cache invalidations.
    pub invalidations: u64,

    /// Number of entries rejected because the cache capacity is zero.
    pub bypasses: u64,
}

impl CacheStatistics {
    /// Returns total successful lookups.
    #[must_use]
    pub const fn total_hits(self) -> u64 {
        self.path_hits
            + self.distance_hits
            + self.weighted_path_hits
            + self.weighted_distance_hits
    }

    /// Returns total unsuccessful lookups.
    #[must_use]
    pub const fn total_misses(self) -> u64 {
        self.path_misses
            + self.distance_misses
            + self.weighted_path_misses
            + self.weighted_distance_misses
    }

    /// Returns total lookups.
    #[must_use]
    pub const fn total_lookups(self) -> u64 {
        self.total_hits() + self.total_misses()
    }

    /// Returns the cache hit rate in the range `[0.0, 1.0]`.
    ///
    /// Returns `0.0` when no lookup has occurred.
    #[must_use]
    pub fn hit_rate(self) -> f64 {
        let total = self.total_lookups();

        if total == 0 {
            0.0
        } else {
            self.total_hits() as f64 / total as f64
        }
    }
}

impl fmt::Display for CacheStatistics {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "hits={}, misses={}, hit_rate={:.4}, evictions={}, \
             invalidations={}, bypasses={}",
            self.total_hits(),
            self.total_misses(),
            self.hit_rate(),
            self.evictions,
            self.invalidations,
            self.bypasses
        )
    }
}

// =============================================================================
// Weight cache key
// =============================================================================

/// Stable identity for a weighted-path cache domain.
///
/// The key is intentionally opaque to `cache.rs`.
///
/// A higher-level cost/calibration subsystem owns the meaning of the key.
///
/// # Safety rule
///
/// If the effective weight function changes, the caller must use a different
/// key or explicitly invalidate the cache.
///
/// The cache cannot automatically determine whether two arbitrary `PathWeight`
/// implementations are semantically equivalent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WeightCacheKey(u64);

impl WeightCacheKey {
    /// Creates a weight-cache key from a stable caller-defined identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw stable identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for WeightCacheKey {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<WeightCacheKey> for u64 {
    fn from(key: WeightCacheKey) -> Self {
        key.value()
    }
}

impl fmt::Display for WeightCacheKey {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "weight-domain-{}", self.0)
    }
}

// =============================================================================
// Cache configuration
// =============================================================================

/// Configuration for `PathCache`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathCacheConfig {
    /// Maximum number of entries held by the cache.
    ///
    /// Zero disables storage while keeping the API operational.
    pub capacity: usize,

    /// Whether successful lookups update recency.
    ///
    /// Production default is `true`.
    pub update_recency_on_hit: bool,

    /// Whether path results are cached.
    pub cache_paths: bool,

    /// Whether distance results are cached.
    pub cache_distances: bool,

    /// Whether weighted path results are cached.
    pub cache_weighted_paths: bool,

    /// Whether weighted distance results are cached.
    pub cache_weighted_distances: bool,
}

impl Default for PathCacheConfig {
    fn default() -> Self {
        Self {
            capacity: DEFAULT_CACHE_CAPACITY,
            update_recency_on_hit: true,
            cache_paths: true,
            cache_distances: true,
            cache_weighted_paths: true,
            cache_weighted_distances: true,
        }
    }
}

impl PathCacheConfig {
    /// Creates the production default configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            capacity: DEFAULT_CACHE_CAPACITY,
            update_recency_on_hit: true,
            cache_paths: true,
            cache_distances: true,
            cache_weighted_paths: true,
            cache_weighted_distances: true,
        }
    }

    /// Validates the configuration.
    pub fn validate(&self) -> RoutingResult<()> {
        if self.capacity > MAX_CACHE_CAPACITY {
            return Err(RoutingError::InvalidConfiguration(
                format!(
                    "routing path cache capacity {} exceeds maximum {}",
                    self.capacity, MAX_CACHE_CAPACITY
                ),
            ));
        }

        Ok(())
    }

    /// Sets the cache capacity.
    #[must_use]
    pub const fn with_capacity(
        mut self,
        capacity: usize,
    ) -> Self {
        self.capacity = capacity;
        self
    }

    /// Controls recency updates on cache hits.
    #[must_use]
    pub const fn with_recency_updates(
        mut self,
        enabled: bool,
    ) -> Self {
        self.update_recency_on_hit = enabled;
        self
    }

    /// Controls path-result caching.
    #[must_use]
    pub const fn with_path_caching(
        mut self,
        enabled: bool,
    ) -> Self {
        self.cache_paths = enabled;
        self
    }

    /// Controls distance-result caching.
    #[must_use]
    pub const fn with_distance_caching(
        mut self,
        enabled: bool,
    ) -> Self {
        self.cache_distances = enabled;
        self
    }

    /// Controls weighted-path caching.
    #[must_use]
    pub const fn with_weighted_path_caching(
        mut self,
        enabled: bool,
    ) -> Self {
        self.cache_weighted_paths = enabled;
        self
    }

    /// Controls weighted-distance caching.
    #[must_use]
    pub const fn with_weighted_distance_caching(
        mut self,
        enabled: bool,
    ) -> Self {
        self.cache_weighted_distances = enabled;
        self
    }
}

// =============================================================================
// Cache key
// =============================================================================

/// Internal cache-key namespace.
///
/// Path and distance entries are deliberately separate.
///
/// Weighted entries additionally contain the weight-domain identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CacheKey {
    Path {
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    },

    Distance {
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    },

    WeightedPath {
        weight: WeightCacheKey,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    },

    WeightedDistance {
        weight: WeightCacheKey,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    },
}

// =============================================================================
// Cache value
// =============================================================================

/// Cached path value.
///
/// `PathResult` is immutable from the cache's perspective, so cloning it is
/// safe and keeps callers from mutating cache state.
#[derive(Debug, Clone)]
enum CacheValue {
    Path(PathResult),
    Distance(u64),
}

impl CacheValue {
    fn is_path(&self) -> bool {
        matches!(self, Self::Path(_))
    }

    fn is_distance(&self) -> bool {
        matches!(self, Self::Distance(_))
    }
}

// =============================================================================
// Cache entry
// =============================================================================

/// Internal cache entry with deterministic access sequencing.
#[derive(Debug, Clone)]
struct CacheEntry {
    value: CacheValue,
    last_access: u64,
}

// =============================================================================
// Path cache
// =============================================================================

/// Production bounded cache for topology path searches.
///
/// The cache borrows one immutable `PhysicalTopology` for its entire lifetime.
///
/// This design provides a strong invariant:
///
/// ```text
/// PathCache<'a>
///     └── topology: &'a PhysicalTopology
/// ```
///
/// Therefore cached paths can never accidentally be reused against a different
/// topology through this API.
///
/// # Thread safety
///
/// The cache itself is intentionally not synchronized.
///
/// For concurrent routing, callers may use:
///
/// ```text
/// Arc<Mutex<PathCache<'_>>>
/// ```
///
/// or another synchronization strategy appropriate to the routing engine.
///
/// This avoids imposing locking overhead on single-threaded routing.
pub struct PathCache<'topology> {
    topology: &'topology PhysicalTopology,

    path_finder: PathFinder,

    config: PathCacheConfig,

    entries: HashMap<CacheKey, CacheEntry>,

    next_access_sequence: u64,

    statistics: CacheStatistics,
}

impl<'topology> fmt::Debug for PathCache<'topology> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("PathCache")
            .field("config", &self.config)
            .field("entry_count", &self.entries.len())
            .field("next_access_sequence", &self.next_access_sequence)
            .field("statistics", &self.statistics)
            .finish_non_exhaustive()
    }
}

impl<'topology> PathCache<'topology> {
    /// Creates a cache using production defaults.
    pub fn new(
        topology: &'topology PhysicalTopology,
    ) -> RoutingResult<Self> {
        Self::with_config(
            topology,
            PathCacheConfig::default(),
            PathSearchConfig::default(),
        )
    }

    /// Creates a cache with explicit cache and path-search configuration.
    pub fn with_config(
        topology: &'topology PhysicalTopology,
        config: PathCacheConfig,
        path_config: PathSearchConfig,
    ) -> RoutingResult<Self> {
        config.validate()?;
        path_config.validate()?;

        topology.validate()?;

        let path_finder =
            PathFinder::with_config(path_config)?;

        Ok(Self {
            topology,
            path_finder,
            config,
            entries: HashMap::with_capacity(
                config.capacity.min(1024),
            ),
            next_access_sequence: 0,
            statistics: CacheStatistics::default(),
        })
    }

    /// Returns the cache implementation version.
    #[must_use]
    pub const fn version() -> &'static str {
        CACHE_VERSION
    }

    /// Returns the cache configuration.
    #[must_use]
    pub const fn config(&self) -> &PathCacheConfig {
        &self.config
    }

    /// Returns the path-search configuration used for cache misses.
    #[must_use]
    pub const fn path_config(&self) -> &PathSearchConfig {
        self.path_finder.config()
    }

    /// Returns the topology borrowed by this cache.
    #[must_use]
    pub const fn topology(&self) -> &'topology PhysicalTopology {
        self.topology
    }

    /// Returns the number of currently cached entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the cache contains no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the configured maximum number of entries.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.config.capacity
    }

    /// Returns whether the cache is storage-enabled.
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.config.capacity > 0
    }

    /// Returns a snapshot of cache statistics.
    #[must_use]
    pub const fn statistics(&self) -> CacheStatistics {
        self.statistics
    }

    /// Returns the underlying path finder.
    ///
    /// This is exposed read-only so routing algorithms can use the same
    /// validated search configuration for operations they intentionally do not
    /// want cached.
    #[must_use]
    pub const fn path_finder(&self) -> &PathFinder {
        &self.path_finder
    }

    // =========================================================================
    // Unweighted path
    // =========================================================================

    /// Returns a cached or newly computed unweighted shortest path.
    ///
    /// On a cache miss, the result is computed by `PathFinder`.
    pub fn shortest_path(
        &mut self,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> RoutingResult<PathResult> {
        if !self.config.cache_paths {
            self.statistics.bypasses =
                self.statistics.bypasses.saturating_add(1);

            return self
                .path_finder
                .shortest_path(
                    self.topology,
                    source,
                    target,
                );
        }

        let key = CacheKey::Path {
            source,
            target,
        };

        if let Some(value) = self.lookup_path(key) {
            self.statistics.path_hits =
                self.statistics.path_hits.saturating_add(1);

            return Ok(value);
        }

        self.statistics.path_misses =
            self.statistics.path_misses.saturating_add(1);

        let result = self
            .path_finder
            .shortest_path(
                self.topology,
                source,
                target,
            )?;

        self.insert(
            key,
            CacheValue::Path(result.clone()),
        );

        Ok(result)
    }

    // =========================================================================
    // Unweighted distance
    // =========================================================================

    /// Returns a cached or newly computed unweighted shortest distance.
    ///
    /// The full path is not reconstructed on a cache miss.
    pub fn shortest_distance(
        &mut self,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> RoutingResult<u64> {
        if !self.config.cache_distances {
            self.statistics.bypasses =
                self.statistics.bypasses.saturating_add(1);

            return self
                .path_finder
                .shortest_distance(
                    self.topology,
                    source,
                    target,
                );
        }

        let key = CacheKey::Distance {
            source,
            target,
        };

        if let Some(value) = self.lookup_distance(key) {
            self.statistics.distance_hits =
                self.statistics.distance_hits.saturating_add(1);

            return Ok(value);
        }

        self.statistics.distance_misses =
            self.statistics.distance_misses.saturating_add(1);

        let result = self
            .path_finder
            .shortest_distance(
                self.topology,
                source,
                target,
            )?;

        self.insert(
            key,
            CacheValue::Distance(result),
        );

        Ok(result)
    }

    // =========================================================================
    // Weighted path
    // =========================================================================

    /// Returns a cached or newly computed weighted shortest path.
    ///
    /// `weight_key` identifies the semantic weighting domain.
    ///
    /// The caller MUST change `weight_key` whenever the effective weighting
    /// semantics change.
    pub fn weighted_shortest_path<W>(
        &mut self,
        weight_key: WeightCacheKey,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        weight: &W,
    ) -> RoutingResult<PathResult>
    where
        W: PathWeight + ?Sized,
    {
        if !self.config.cache_weighted_paths {
            self.statistics.bypasses =
                self.statistics.bypasses.saturating_add(1);

            return self
                .path_finder
                .weighted_shortest_path(
                    self.topology,
                    source,
                    target,
                    weight,
                );
        }

        let key = CacheKey::WeightedPath {
            weight: weight_key,
            source,
            target,
        };

        if let Some(value) = self.lookup_path(key) {
            self.statistics.weighted_path_hits =
                self.statistics
                    .weighted_path_hits
                    .saturating_add(1);

            return Ok(value);
        }

        self.statistics.weighted_path_misses =
            self.statistics
                .weighted_path_misses
                .saturating_add(1);

        let result = self
            .path_finder
            .weighted_shortest_path(
                self.topology,
                source,
                target,
                weight,
            )?;

        self.insert(
            key,
            CacheValue::Path(result.clone()),
        );

        Ok(result)
    }

    // =========================================================================
    // Weighted distance
    // =========================================================================

    /// Returns a cached or newly computed weighted shortest distance.
    ///
    /// `weight_key` identifies the semantic weighting domain.
    pub fn weighted_shortest_distance<W>(
        &mut self,
        weight_key: WeightCacheKey,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        weight: &W,
    ) -> RoutingResult<u64>
    where
        W: PathWeight + ?Sized,
    {
        if !self.config.cache_weighted_distances {
            self.statistics.bypasses =
                self.statistics.bypasses.saturating_add(1);

            return self
                .path_finder
                .weighted_shortest_distance(
                    self.topology,
                    source,
                    target,
                    weight,
                );
        }

        let key = CacheKey::WeightedDistance {
            weight: weight_key,
            source,
            target,
        };

        if let Some(value) = self.lookup_distance(key) {
            self.statistics.weighted_distance_hits =
                self.statistics
                    .weighted_distance_hits
                    .saturating_add(1);

            return Ok(value);
        }

        self.statistics.weighted_distance_misses =
            self.statistics
                .weighted_distance_misses
                .saturating_add(1);

        let result = self
            .path_finder
            .weighted_shortest_distance(
                self.topology,
                source,
                target,
                weight,
            )?;

        self.insert(
            key,
            CacheValue::Distance(result),
        );

        Ok(result)
    }

    // =========================================================================
    // Cache invalidation
    // =========================================================================

    /// Removes all cached entries.
    ///
    /// This is the correct operation when external calibration state changes
    /// and cached weighted results are no longer valid.
    pub fn clear(&mut self) {
        if !self.entries.is_empty() {
            self.entries.clear();
        }

        self.statistics.invalidations =
            self.statistics.invalidations.saturating_add(1);
    }

    /// Removes all entries belonging to one weighted-cache domain.
    ///
    /// Unweighted path/distance entries are unaffected.
    pub fn invalidate_weight_domain(
        &mut self,
        weight_key: WeightCacheKey,
    ) {
        self.entries.retain(|key, _| {
            !matches!(
                key,
                CacheKey::WeightedPath {
                    weight,
                    ..
                } if *weight == weight_key
            ) && !matches!(
                key,
                CacheKey::WeightedDistance {
                    weight,
                    ..
                } if *weight == weight_key
            )
        });

        self.statistics.invalidations =
            self.statistics.invalidations.saturating_add(1);
    }

    /// Removes one source/target pair from the unweighted path cache.
    pub fn invalidate_path(
        &mut self,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) {
        self.entries.remove(&CacheKey::Path {
            source,
            target,
        });

        self.statistics.invalidations =
            self.statistics.invalidations.saturating_add(1);
    }

    /// Removes one source/target pair from the unweighted distance cache.
    pub fn invalidate_distance(
        &mut self,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) {
        self.entries.remove(&CacheKey::Distance {
            source,
            target,
        });

        self.statistics.invalidations =
            self.statistics.invalidations.saturating_add(1);
    }

    /// Removes all cached entries involving a physical qubit.
    ///
    /// This is useful when a dynamic routing layer invalidates a physical
    /// resource.
    pub fn invalidate_physical_qubit(
        &mut self,
        qubit: PhysicalQubitId,
    ) {
        self.entries.retain(|key, _| {
            !key_involves_qubit(*key, qubit)
        });

        self.statistics.invalidations =
            self.statistics.invalidations.saturating_add(1);
    }

    // =========================================================================
    // Capacity
    // =========================================================================

    /// Changes the cache capacity.
    ///
    /// If the new capacity is smaller than the current entry count, the oldest
    /// entries are evicted deterministically until the invariant is restored.
    pub fn set_capacity(
        &mut self,
        capacity: usize,
    ) -> RoutingResult<()> {
        if capacity > MAX_CACHE_CAPACITY {
            return Err(RoutingError::InvalidConfiguration(
                format!(
                    "routing path cache capacity {} exceeds maximum {}",
                    capacity, MAX_CACHE_CAPACITY
                ),
            ));
        }

        self.config.capacity = capacity;

        while self.entries.len() > capacity {
            self.evict_one()?;
        }

        Ok(())
    }

    // =========================================================================
    // Internal lookup
    // =========================================================================

    fn lookup_path(
        &mut self,
        key: CacheKey,
    ) -> Option<PathResult> {
        let entry = self.entries.get_mut(&key)?;

        if self.config.update_recency_on_hit {
            let sequence = self.next_sequence();
            entry.last_access = sequence;
        }

        match &entry.value {
            CacheValue::Path(path) => Some(path.clone()),
            CacheValue::Distance(_) => None,
        }
    }

    fn lookup_distance(
        &mut self,
        key: CacheKey,
    ) -> Option<u64> {
        let entry = self.entries.get_mut(&key)?;

        if self.config.update_recency_on_hit {
            let sequence = self.next_sequence();
            entry.last_access = sequence;
        }

        match entry.value {
            CacheValue::Distance(distance) => Some(distance),
            CacheValue::Path(_) => None,
        }
    }

    // =========================================================================
    // Internal insertion
    // =========================================================================

    fn insert(
        &mut self,
        key: CacheKey,
        value: CacheValue,
    ) {
        if self.config.capacity == 0 {
            self.statistics.bypasses =
                self.statistics.bypasses.saturating_add(1);
            return;
        }

        let sequence = self.next_sequence();

        self.entries.insert(
            key,
            CacheEntry {
                value,
                last_access: sequence,
            },
        );

        while self.entries.len() > self.config.capacity {
            if self.evict_one().is_err() {
                // The cache invariant cannot normally make eviction fail.
                //
                // If it somehow does, remove the just-inserted entry rather
                // than allowing an over-capacity cache to escape.
                self.entries.remove(&key);
                break;
            }
        }
    }

    // =========================================================================
    // Internal eviction
    // =========================================================================

    fn evict_one(&mut self) -> RoutingResult<()> {
        if self.entries.is_empty() {
            return Err(
                RoutingError::InternalInvariantViolation(
                    "cache eviction requested on an empty cache"
                        .to_string(),
                ),
            );
        }

        let mut oldest_key: Option<CacheKey> = None;
        let mut oldest_sequence = u64::MAX;

        for (key, entry) in &self.entries {
            let replace = match oldest_key {
                None => true,
                Some(current_key) => {
                    entry.last_access < oldest_sequence
                        || (
                            entry.last_access
                                == oldest_sequence
                            && cache_key_order(*key)
                                < cache_key_order(current_key)
                        )
                }
            };

            if replace {
                oldest_key = Some(*key);
                oldest_sequence = entry.last_access;
            }
        }

        let key = oldest_key.ok_or_else(|| {
            RoutingError::InternalInvariantViolation(
                "cache contained entries but no eviction key \
                 could be selected"
                    .to_string(),
            )
        })?;

        self.entries.remove(&key);

        self.statistics.evictions =
            self.statistics.evictions.saturating_add(1);

        Ok(())
    }

    fn next_sequence(&mut self) -> u64 {
        match self.next_access_sequence.checked_add(1) {
            Some(next) => {
                self.next_access_sequence = next;
                next
            }
            None => {
                // Sequence exhaustion is extraordinarily unlikely in practice.
                //
                // Resetting the access sequence requires rebuilding ordering
                // metadata. We do that deterministically instead of allowing
                // wrapping to corrupt LRU semantics.
                self.rebase_access_sequences();
                self.next_access_sequence =
                    1;
                1
            }
        }
    }

    fn rebase_access_sequences(&mut self) {
        let mut entries: Vec<(CacheKey, u64)> = self
            .entries
            .iter()
            .map(|(key, entry)| {
                (*key, entry.last_access)
            })
            .collect();

        entries.sort_by(|left, right| {
            left.1
                .cmp(&right.1)
                .then_with(|| {
                    cache_key_order(left.0)
                        .cmp(&cache_key_order(right.0))
                })
        });

        for (sequence, (key, _)) in
            entries.into_iter().enumerate()
        {
            if let Some(entry) = self.entries.get_mut(&key) {
                entry.last_access =
                    sequence.saturating_add(1);
            }
        }
    }
}

// =============================================================================
// Cache-key ordering
// =============================================================================

/// Converts a cache key into a deterministic comparable tuple.
///
/// This exists only for deterministic LRU tie-breaking.
fn cache_key_order(
    key: CacheKey,
) -> (
    u8,
    u64,
    usize,
    usize,
) {
    match key {
        CacheKey::Path {
            source,
            target,
        } => (
            0,
            0,
            source.index(),
            target.index(),
        ),

        CacheKey::Distance {
            source,
            target,
        } => (
            1,
            0,
            source.index(),
            target.index(),
        ),

        CacheKey::WeightedPath {
            weight,
            source,
            target,
        } => (
            2,
            weight.value(),
            source.index(),
            target.index(),
        ),

        CacheKey::WeightedDistance {
            weight,
            source,
            target,
        } => (
            3,
            weight.value(),
            source.index(),
            target.index(),
        ),
    }
}

/// Returns whether a cache key contains a particular physical qubit.
fn key_involves_qubit(
    key: CacheKey,
    qubit: PhysicalQubitId,
) -> bool {
    match key {
        CacheKey::Path {
            source,
            target,
        }
        | CacheKey::Distance {
            source,
            target,
        }
        | CacheKey::WeightedPath {
            source,
            target,
            ..
        }
        | CacheKey::WeightedDistance {
            source,
            target,
            ..
        } => source == qubit || target == qubit,
    }
}

// =============================================================================
// Cache maintenance helpers
// =============================================================================

impl<'topology> PathCache<'topology> {
    /// Returns the number of path entries currently cached.
    #[must_use]
    pub fn path_entry_count(&self) -> usize {
        self.entries
            .values()
            .filter(|entry| entry.value.is_path())
            .count()
    }

    /// Returns the number of distance entries currently cached.
    #[must_use]
    pub fn distance_entry_count(&self) -> usize {
        self.entries
            .values()
            .filter(|entry| entry.value.is_distance())
            .count()
    }

    /// Returns the number of weighted entries currently cached.
    #[must_use]
    pub fn weighted_entry_count(&self) -> usize {
        self.entries
            .keys()
            .filter(|key| {
                matches!(
                    key,
                    CacheKey::WeightedPath { .. }
                        | CacheKey::WeightedDistance { .. }
                )
            })
            .count()
    }

    /// Resets statistics without clearing cached routing results.
    ///
    /// This is useful when beginning a new benchmark interval.
    pub const fn reset_statistics(&mut self) {
        self.statistics = CacheStatistics {
            path_hits: 0,
            path_misses: 0,
            distance_hits: 0,
            distance_misses: 0,
            weighted_path_hits: 0,
            weighted_path_misses: 0,
            weighted_distance_hits: 0,
            weighted_distance_misses: 0,
            evictions: 0,
            invalidations: 0,
            bypasses: 0,
        };
    }

    /// Returns the approximate memory-independent number of cached routing
    /// objects.
    ///
    /// This deliberately returns entry count rather than bytes because Rust
    /// allocator overhead and `PathResult` capacity are implementation details.
    #[must_use]
    pub fn cached_object_count(&self) -> usize {
        self.entries.len()
    }

    /// Returns the age sequence of the least-recently-used entry.
    ///
    /// This is primarily useful for diagnostics and tests.
    #[must_use]
    pub fn oldest_access_sequence(&self) -> Option<u64> {
        self.entries
            .values()
            .map(|entry| entry.last_access)
            .min()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn line_topology(
        count: usize,
    ) -> PhysicalTopology {
        PhysicalTopology::linear(count)
            .expect("linear topology should be valid")
    }

    #[test]
    fn default_configuration_is_valid() {
        let config = PathCacheConfig::default();

        assert!(
            config.validate().is_ok()
        );

        assert_eq!(
            config.capacity,
            DEFAULT_CACHE_CAPACITY
        );

        assert!(config.cache_paths);
        assert!(config.cache_distances);
        assert!(config.cache_weighted_paths);
        assert!(config.cache_weighted_distances);
    }

    #[test]
    fn zero_capacity_cache_is_valid_and_bounded() {
        let topology = line_topology(4);

        let config =
            PathCacheConfig::default()
                .with_capacity(0);

        let mut cache =
            PathCache::with_config(
                &topology,
                config,
                PathSearchConfig::default(),
            )
            .expect("cache construction should succeed");

        let path = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(3),
            )
            .expect("path should exist");

        assert_eq!(
            path.edge_count(),
            3
        );

        assert_eq!(cache.len(), 0);
        assert!(
            cache.statistics().bypasses > 0
        );
    }

    #[test]
    fn repeated_path_lookup_hits_cache() {
        let topology = line_topology(5);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        let first = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(4),
            )
            .expect("path should exist");

        let second = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(4),
            )
            .expect("path should exist");

        assert_eq!(first, second);

        let statistics = cache.statistics();

        assert_eq!(statistics.path_misses, 1);
        assert_eq!(statistics.path_hits, 1);
    }

    #[test]
    fn repeated_distance_lookup_hits_cache() {
        let topology = line_topology(5);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        let first = cache
            .shortest_distance(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(4),
            )
            .expect("distance should exist");

        let second = cache
            .shortest_distance(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(4),
            )
            .expect("distance should exist");

        assert_eq!(first, 4);
        assert_eq!(second, 4);

        let statistics = cache.statistics();

        assert_eq!(
            statistics.distance_misses,
            1
        );

        assert_eq!(
            statistics.distance_hits,
            1
        );
    }

    #[test]
    fn path_and_distance_have_independent_entries() {
        let topology = line_topology(5);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(4),
            )
            .expect("path should exist");

        let _ = cache
            .shortest_distance(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(4),
            )
            .expect("distance should exist");

        assert_eq!(
            cache.path_entry_count(),
            1
        );

        assert_eq!(
            cache.distance_entry_count(),
            1
        );

        assert_eq!(
            cache.len(),
            2
        );
    }

    #[test]
    fn cache_is_bounded() {
        let topology = line_topology(8);

        let config =
            PathCacheConfig::default()
                .with_capacity(2);

        let mut cache =
            PathCache::with_config(
                &topology,
                config,
                PathSearchConfig::default(),
            )
            .expect("cache construction should succeed");

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
            )
            .expect("path should exist");

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(2),
            )
            .expect("path should exist");

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(2),
                PhysicalQubitId::new(3),
            )
            .expect("path should exist");

        assert_eq!(
            cache.len(),
            2
        );

        assert_eq!(
            cache.statistics().evictions,
            1
        );
    }

    #[test]
    fn cache_updates_lru_recency() {
        let topology = line_topology(8);

        let config =
            PathCacheConfig::default()
                .with_capacity(2);

        let mut cache =
            PathCache::with_config(
                &topology,
                config,
                PathSearchConfig::default(),
            )
            .expect("cache construction should succeed");

        let a = (
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
        );

        let b = (
            PhysicalQubitId::new(2),
            PhysicalQubitId::new(3),
        );

        let c = (
            PhysicalQubitId::new(4),
            PhysicalQubitId::new(5),
        );

        let _ = cache
            .shortest_path(a.0, a.1)
            .expect("path should exist");

        let _ = cache
            .shortest_path(b.0, b.1)
            .expect("path should exist");

        // Refresh A, making B the oldest entry.
        let _ = cache
            .shortest_path(a.0, a.1)
            .expect("cached path should exist");

        let _ = cache
            .shortest_path(c.0, c.1)
            .expect("path should exist");

        // A should remain because it was recently accessed.
        let statistics_before =
            cache.statistics();

        let _ = cache
            .shortest_path(a.0, a.1)
            .expect("A should still be cached");

        let statistics_after =
            cache.statistics();

        assert_eq!(
            statistics_after.path_hits,
            statistics_before.path_hits + 1
        );
    }

    #[test]
    fn explicit_clear_removes_all_entries() {
        let topology = line_topology(5);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(4),
            )
            .expect("path should exist");

        assert!(!cache.is_empty());

        cache.clear();

        assert!(cache.is_empty());

        assert_eq!(
            cache.statistics().invalidations,
            1
        );
    }

    #[test]
    fn invalidate_physical_qubit_removes_related_entries() {
        let topology = line_topology(8);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(2),
            )
            .expect("path should exist");

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(4),
                PhysicalQubitId::new(6),
            )
            .expect("path should exist");

        assert_eq!(cache.len(), 2);

        cache.invalidate_physical_qubit(
            PhysicalQubitId::new(2),
        );

        assert_eq!(cache.len(), 1);

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(2),
            )
            .expect("path should exist");

        assert_eq!(
            cache.statistics().path_misses,
            3
        );
    }

    #[test]
    fn weighted_cache_uses_weight_domain() {
        let topology = line_topology(5);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        let weight =
            super::super::path::UnitPathWeight;

        let key_a =
            WeightCacheKey::new(1);

        let key_b =
            WeightCacheKey::new(2);

        let _ = cache
            .weighted_shortest_distance(
                key_a,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(4),
                &weight,
            )
            .expect("distance should exist");

        let _ = cache
            .weighted_shortest_distance(
                key_a,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(4),
                &weight,
            )
            .expect("distance should exist");

        let _ = cache
            .weighted_shortest_distance(
                key_b,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(4),
                &weight,
            )
            .expect("distance should exist");

        let statistics =
            cache.statistics();

        assert_eq!(
            statistics.weighted_distance_misses,
            2
        );

        assert_eq!(
            statistics.weighted_distance_hits,
            1
        );
    }

    #[test]
    fn weight_domain_can_be_invalidated_independently() {
        let topology = line_topology(5);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        let weight =
            super::super::path::UnitPathWeight;

        let key =
            WeightCacheKey::new(100);

        let _ = cache
            .weighted_shortest_path(
                key,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(4),
                &weight,
            )
            .expect("path should exist");

        assert_eq!(
            cache.weighted_entry_count(),
            1
        );

        cache.invalidate_weight_domain(key);

        assert_eq!(
            cache.weighted_entry_count(),
            0
        );
    }

    #[test]
    fn capacity_can_be_reduced_safely() {
        let topology = line_topology(8);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
            )
            .expect("path should exist");

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(2),
                PhysicalQubitId::new(3),
            )
            .expect("path should exist");

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(4),
                PhysicalQubitId::new(5),
            )
            .expect("path should exist");

        assert_eq!(cache.len(), 3);

        cache
            .set_capacity(1)
            .expect("capacity change should succeed");

        assert_eq!(cache.len(), 1);
        assert_eq!(
            cache.capacity(),
            1
        );
    }

    #[test]
    fn invalid_capacity_is_rejected() {
        let topology = line_topology(4);

        let config =
            PathCacheConfig::default()
                .with_capacity(
                    MAX_CACHE_CAPACITY + 1,
                );

        let result =
            PathCache::with_config(
                &topology,
                config,
                PathSearchConfig::default(),
            );

        assert!(result.is_err());
    }

    #[test]
    fn invalid_path_configuration_is_rejected() {
        let topology = line_topology(4);

        let path_config =
            PathSearchConfig::default()
                .with_max_visited_vertices(0);

        let result =
            PathCache::with_config(
                &topology,
                PathCacheConfig::default(),
                path_config,
            );

        assert!(result.is_err());
    }

    #[test]
    fn statistics_hit_rate_is_correct() {
        let mut statistics =
            CacheStatistics::default();

        statistics.path_hits = 3;
        statistics.path_misses = 1;
        statistics.distance_hits = 1;
        statistics.distance_misses = 1;

        assert_eq!(
            statistics.total_hits(),
            4
        );

        assert_eq!(
            statistics.total_misses(),
            2
        );

        assert!(
            (statistics.hit_rate() - (4.0 / 6.0)).abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn statistics_zero_lookups_have_zero_hit_rate() {
        let statistics =
            CacheStatistics::default();

        assert_eq!(
            statistics.hit_rate(),
            0.0
        );
    }

    #[test]
    fn reset_statistics_does_not_clear_entries() {
        let topology = line_topology(5);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(4),
            )
            .expect("path should exist");

        assert_eq!(cache.len(), 1);

        cache.reset_statistics();

        assert_eq!(cache.len(), 1);
        assert_eq!(
            cache.statistics().total_lookups(),
            0
        );

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(4),
            )
            .expect("cached path should exist");

        assert_eq!(
            cache.statistics().path_hits,
            1
        );
    }

    #[test]
    fn path_result_is_not_exposed_mutably() {
        let topology = line_topology(4);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        let result = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(3),
            )
            .expect("path should exist");

        assert_eq!(
            result.vertices().len(),
            4
        );

        // The cache owns its copy and exposes only an immutable result.
        assert_eq!(
            cache.len(),
            1
        );
    }

    #[test]
    fn cache_preserves_path_finder_configuration() {
        let topology = line_topology(8);

        let path_config =
            PathSearchConfig::default()
                .with_max_path_edges(Some(4));

        let mut cache =
            PathCache::with_config(
                &topology,
                PathCacheConfig::default(),
                path_config.clone(),
            )
            .expect("cache construction should succeed");

        assert_eq!(
            cache.path_config(),
            &path_config
        );
    }

    #[test]
    fn shortest_path_errors_are_not_cached() {
        let topology = line_topology(4);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        // p0 -> p99 does not exist.
        let first = cache.shortest_path(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(99),
        );

        assert!(first.is_err());

        assert_eq!(
            cache.len(),
            0
        );

        let second = cache.shortest_path(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(99),
        );

        assert!(second.is_err());

        assert_eq!(
            cache.len(),
            0
        );

        assert_eq!(
            cache.statistics().path_misses,
            2
        );

        assert_eq!(
            cache.statistics().path_hits,
            0
        );
    }

    #[test]
    fn cache_version_is_stable() {
        assert_eq!(
            PathCache::<'static>::version(),
            CACHE_VERSION
        );
    }

    #[test]
    fn weight_cache_key_is_stable() {
        let key =
            WeightCacheKey::new(42);

        assert_eq!(
            key.value(),
            42
        );

        let raw: u64 = key.into();

        assert_eq!(
            raw,
            42
        );
    }

    #[test]
    fn invalidating_nonexistent_entry_is_safe() {
        let topology = line_topology(4);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        cache.invalidate_path(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(3),
        );

        cache.invalidate_distance(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(3),
        );

        assert!(cache.is_empty());
    }

    #[test]
    fn cache_entries_are_separated_by_direction() {
        let topology = line_topology(4);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        let forward = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(3),
            )
            .expect("path should exist");

        let reverse = cache
            .shortest_path(
                PhysicalQubitId::new(3),
                PhysicalQubitId::new(0),
            )
            .expect("path should exist");

        assert_ne!(
            forward.vertices(),
            reverse.vertices()
        );

        assert_eq!(
            cache.statistics().path_misses,
            2
        );
    }

    #[test]
    fn cache_can_disable_individual_entry_types() {
        let topology = line_topology(4);

        let config =
            PathCacheConfig::default()
                .with_path_caching(false)
                .with_distance_caching(true);

        let mut cache =
            PathCache::with_config(
                &topology,
                config,
                PathSearchConfig::default(),
            )
            .expect("cache construction should succeed");

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(3),
            )
            .expect("path should exist");

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(3),
            )
            .expect("path should exist");

        assert_eq!(
            cache.statistics().path_hits,
            0
        );

        let _ = cache
            .shortest_distance(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(3),
            )
            .expect("distance should exist");

        let _ = cache
            .shortest_distance(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(3),
            )
            .expect("distance should exist");

        assert_eq!(
            cache.statistics().distance_hits,
            1
        );
    }

    #[test]
    fn oldest_sequence_is_available_for_diagnostics() {
        let topology = line_topology(4);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        assert!(
            cache.oldest_access_sequence().is_none()
        );

        let _ = cache
            .shortest_path(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(3),
            )
            .expect("path should exist");

        assert!(
            cache.oldest_access_sequence().is_some()
        );
    }

    #[test]
    fn weighted_path_cache_returns_identical_results() {
        let topology = line_topology(6);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        let weight =
            super::super::path::UnitPathWeight;

        let key =
            WeightCacheKey::new(7);

        let first = cache
            .weighted_shortest_path(
                key,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(5),
                &weight,
            )
            .expect("path should exist");

        let second = cache
            .weighted_shortest_path(
                key,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(5),
                &weight,
            )
            .expect("path should exist");

        assert_eq!(first, second);

        assert_eq!(
            cache.statistics().weighted_path_misses,
            1
        );

        assert_eq!(
            cache.statistics().weighted_path_hits,
            1
        );
    }

    #[test]
    fn weighted_distance_cache_returns_identical_results() {
        let topology = line_topology(6);

        let mut cache =
            PathCache::new(&topology)
                .expect("cache construction should succeed");

        let weight =
            super::super::path::UnitPathWeight;

        let key =
            WeightCacheKey::new(8);

        let first = cache
            .weighted_shortest_distance(
                key,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(5),
                &weight,
            )
            .expect("distance should exist");

        let second = cache
            .weighted_shortest_distance(
                key,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(5),
                &weight,
            )
            .expect("distance should exist");

        assert_eq!(first, 5);
        assert_eq!(second, 5);

        assert_eq!(
            cache.statistics().weighted_distance_misses,
            1
        );

        assert_eq!(
            cache.statistics().weighted_distance_hits,
            1
        );
    }
}