//! Production-grade caching infrastructure for Zamani Quantum Error Correction.
//!
//! The cache subsystem is deliberately correctness-first:
//!
//! ```text
//! Request
//!   |
//!   v
//! Cache lookup
//!   |
//!   +---- hit ----> Integrity verification
//!   |                    |
//!   |                    +---- valid ----> return value
//!   |                    |
//!   |                    +---- invalid --> discard
//!   |
//!   +---- miss ---------------------------> recompute
//!                                             |
//!                                             v
//!                                          validate
//!                                             |
//!                                             v
//!                                            cache
//! ```
//!
//! # Design goals
//!
//! * bounded memory usage;
//! * deterministic eviction;
//! * explicit resource accounting;
//! * cache corruption must never become a correctness dependency;
//! * cache misses must always remain safe;
//! * cache entries may be independently invalidated;
//! * optional TTL expiration;
//! * optional integrity verification;
//! * thread-safe access;
//! * no unsafe code;
//! * no panic-based public cache operations.
//!
//! # Correctness rule
//!
//! A cache is an optimization, never a source of truth.
//!
//! ```text
//! cache miss      -> recompute
//! expired entry   -> discard/recompute
//! invalid entry   -> discard/recompute
//! corrupted entry -> discard/recompute
//! ```
//!
//! A corrupted or stale cache entry must therefore never be trusted merely
//! because it exists.
//!
//! # Determinism
//!
//! Eviction is deterministic:
//!
//! 1. expired entries are removed first;
//! 2. otherwise the configured eviction policy is used;
//! 3. ties are resolved using the monotonically increasing insertion/access
//!    sequence number.
//!
//! Hash-map iteration order is never used to determine eviction order.
//!
//! # Resource safety
//!
//! The cache enforces both:
//!
//! * maximum number of entries;
//! * maximum estimated byte usage.
//!
//! A caller supplies an entry-size estimate. The cache never assumes that
//! `size_of::<V>()` represents the actual heap footprint of a QEC object.
//!
//! This is important for sparse graphs, topology objects, decoder templates,
//! and other structures containing heap allocations.
//!
//! # Rust compatibility
//!
//! This module targets Rust 1.70+ and intentionally avoids newer standard
//! library APIs.

#![deny(unsafe_code)]

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------

/// Default maximum number of cache entries.
pub const DEFAULT_MAX_ENTRIES: usize = 1024;

/// Default maximum estimated cache memory.
///
/// This is a policy default, not an assertion about actual allocator usage.
pub const DEFAULT_MAX_BYTES: u64 = 64 * 1024 * 1024;

/// Default TTL.
///
/// `None` means entries do not expire automatically.
pub const DEFAULT_TTL: Option<Duration> = None;

/// Initial generation number.
const INITIAL_GENERATION: u64 = 1;

// -----------------------------------------------------------------------------
// Error model
// -----------------------------------------------------------------------------

/// Errors returned by cache operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheError {
    /// The cache configuration is invalid.
    InvalidConfiguration(&'static str),

    /// The supplied entry size exceeds the configured byte budget.
    EntryTooLarge {
        size_bytes: u64,
        max_bytes: u64,
    },

    /// Resource accounting would overflow.
    ResourceAccountingOverflow,

    /// The cache mutex was poisoned by a previous panic.
    LockPoisoned,

    /// The requested entry exists but failed integrity validation.
    IntegrityFailure,

    /// The requested entry was found but has expired.
    Expired,

    /// The requested entry was not found.
    NotFound,
}

impl std::fmt::Display for CacheError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(f, "invalid cache configuration: {message}")
            }

            Self::EntryTooLarge {
                size_bytes,
                max_bytes,
            } => {
                write!(
                    f,
                    "cache entry of {size_bytes} bytes exceeds maximum \
                     cache size of {max_bytes} bytes"
                )
            }

            Self::ResourceAccountingOverflow => {
                write!(f, "cache resource accounting overflow")
            }

            Self::LockPoisoned => {
                write!(f, "cache lock is poisoned")
            }

            Self::IntegrityFailure => {
                write!(f, "cache entry failed integrity validation")
            }

            Self::Expired => {
                write!(f, "cache entry has expired")
            }

            Self::NotFound => {
                write!(f, "cache entry not found")
            }
        }
    }
}

impl std::error::Error for CacheError {}

// -----------------------------------------------------------------------------
// Eviction policy
// -----------------------------------------------------------------------------

/// Deterministic cache eviction policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionPolicy {
    /// Least recently used.
    Lru,

    /// First inserted, first evicted.
    Fifo,
}

// -----------------------------------------------------------------------------
// Cache configuration
// -----------------------------------------------------------------------------

/// Configuration for a [`QecCache`].
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of entries.
    pub max_entries: usize,

    /// Maximum estimated bytes.
    pub max_bytes: u64,

    /// Optional entry lifetime.
    pub ttl: Option<Duration>,

    /// Eviction policy.
    pub eviction_policy: EvictionPolicy,

    /// Whether integrity verification should be performed on lookup.
    pub verify_integrity: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: DEFAULT_MAX_ENTRIES,
            max_bytes: DEFAULT_MAX_BYTES,
            ttl: DEFAULT_TTL,
            eviction_policy: EvictionPolicy::Lru,
            verify_integrity: true,
        }
    }
}

impl CacheConfig {
    /// Validates the configuration.
    pub fn validate(&self) -> Result<(), CacheError> {
        if self.max_entries == 0 {
            return Err(
                CacheError::InvalidConfiguration(
                    "max_entries must be greater than zero",
                ),
            );
        }

        if self.max_bytes == 0 {
            return Err(
                CacheError::InvalidConfiguration(
                    "max_bytes must be greater than zero",
                ),
            );
        }

        if let Some(ttl) = self.ttl {
            if ttl.is_zero() {
                return Err(
                    CacheError::InvalidConfiguration(
                        "ttl must be greater than zero when configured",
                    ),
                );
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Integrity
// -----------------------------------------------------------------------------

/// Integrity verifier for cached values.
///
/// Implementations should return a deterministic digest or fingerprint of
/// the value's correctness-relevant state.
///
/// The digest is deliberately represented as `u64` so that this module does
/// not force a cryptographic dependency onto callers.
///
/// For adversarial persistence or untrusted cache storage, callers should
/// provide a cryptographically strong integrity mechanism at a higher layer.
pub trait CacheIntegrity: Send + Sync {
    /// Computes an integrity fingerprint for `value`.
    fn fingerprint(&self, value: &[u8]) -> u64;
}

/// Deterministic FNV-1a integrity helper.
///
/// This is intended for detecting accidental corruption, not for
/// cryptographic authentication.
#[derive(Debug, Clone, Copy, Default)]
pub struct Fnv1aIntegrity;

impl CacheIntegrity for Fnv1aIntegrity {
    fn fingerprint(&self, value: &[u8]) -> u64 {
        const OFFSET: u64 = 0xcbf29ce484222325;
        const PRIME: u64 = 0x100000001b3;

        let mut hash = OFFSET;

        for byte in value {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }

        hash
    }
}

/// User-provided integrity function.
///
/// The function must be deterministic for the lifetime of the cache.
pub struct FunctionIntegrity<F>
where
    F: Fn(&[u8]) -> u64 + Send + Sync,
{
    function: F,
}

impl<F> FunctionIntegrity<F>
where
    F: Fn(&[u8]) -> u64 + Send + Sync,
{
    /// Creates a function-backed integrity verifier.
    pub fn new(function: F) -> Self {
        Self { function }
    }
}

impl<F> CacheIntegrity for FunctionIntegrity<F>
where
    F: Fn(&[u8]) -> u64 + Send + Sync,
{
    fn fingerprint(&self, value: &[u8]) -> u64 {
        (self.function)(value)
    }
}

// -----------------------------------------------------------------------------
// Cache value policy
// -----------------------------------------------------------------------------

/// Describes how a value participates in cache accounting and integrity
/// verification.
///
/// The default implementation intentionally cannot be provided for arbitrary
/// Rust values because heap usage is application-specific.
///
/// QEC structures should implement this trait explicitly.
pub trait CacheValue: Clone + Send + Sync + 'static {
    /// Returns an estimate of the total memory footprint of the cached value.
    ///
    /// The estimate should include owned heap allocations where practical.
    fn estimated_size_bytes(&self) -> u64;

    /// Returns a deterministic byte representation of correctness-relevant
    /// state for integrity verification.
    ///
    /// This does not have to be a serialization format suitable for
    /// persistence.
    fn integrity_bytes(&self) -> Vec<u8>;
}

// -----------------------------------------------------------------------------
// Cache entry
// -----------------------------------------------------------------------------

#[derive(Debug)]
struct CacheEntry<V> {
    value: V,
    size_bytes: u64,
    created_at: Instant,
    last_access: Instant,
    sequence: u64,
    generation: u64,
    pinned: bool,
    fingerprint: u64,
}

impl<V> CacheEntry<V> {
    fn is_expired(
        &self,
        now: Instant,
        ttl: Option<Duration>,
    ) -> bool {
        match ttl {
            Some(ttl) => {
                now.duration_since(self.created_at) >= ttl
            }

            None => false,
        }
    }
}

// -----------------------------------------------------------------------------
// Metrics
// -----------------------------------------------------------------------------

/// Immutable cache statistics snapshot.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CacheStats {
    /// Number of successful lookups.
    pub hits: u64,

    /// Number of lookup misses.
    pub misses: u64,

    /// Number of entries evicted by capacity policy.
    pub evictions: u64,

    /// Number of entries removed because their TTL expired.
    pub expirations: u64,

    /// Number of entries explicitly invalidated.
    pub invalidations: u64,

    /// Number of integrity failures detected.
    pub integrity_failures: u64,

    /// Number of insertion attempts rejected because the entry was too large.
    pub rejected_entries: u64,

    /// Number of entries currently stored.
    pub entries: usize,

    /// Current estimated byte usage.
    pub bytes: u64,

    /// Total number of successful insertions.
    pub insertions: u64,

    /// Total number of replacements.
    pub replacements: u64,

    /// Current cache generation.
    pub generation: u64,
}

// -----------------------------------------------------------------------------
// Internal state
// -----------------------------------------------------------------------------

struct CacheState<K, V> {
    entries: HashMap<K, CacheEntry<V>>,
    stats: CacheStats,
    next_sequence: u64,
    generation: u64,
}

impl<K, V> CacheState<K, V> {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
            stats: CacheStats {
                generation: INITIAL_GENERATION,
                ..CacheStats::default()
            },
            next_sequence: 0,
            generation: INITIAL_GENERATION,
        }
    }

    fn next_sequence(
        &mut self,
    ) -> u64 {
        let sequence = self.next_sequence;
        self.next_sequence =
            self.next_sequence.saturating_add(1);

        sequence
    }

    fn bump_generation(
        &mut self,
    ) {
        self.generation =
            self.generation.saturating_add(1);

        self.stats.generation =
            self.generation;
    }

    fn update_usage(
        &mut self,
    ) {
        self.stats.entries =
            self.entries.len();

        self.stats.bytes =
            self.entries
                .values()
                .fold(0u64, |total, entry| {
                    total.saturating_add(
                        entry.size_bytes,
                    )
                });
    }
}

// -----------------------------------------------------------------------------
// Cache
// -----------------------------------------------------------------------------

/// Thread-safe, bounded QEC cache.
///
/// The cache stores values by key and enforces both entry-count and estimated
/// memory limits.
///
/// # Correctness
///
/// Cache entries are never assumed to be authoritative. Integrity checking,
/// TTL expiration, explicit invalidation, and bounded resource usage all
/// exist to ensure that the cache remains an optimization rather than a
/// correctness dependency.
pub struct QecCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: CacheValue,
{
    config: CacheConfig,
    integrity: Arc<dyn CacheIntegrity>,
    state: Arc<Mutex<CacheState<K, V>>>,
}

impl<K, V> Clone for QecCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: CacheValue,
{
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            integrity: Arc::clone(&self.integrity),
            state: Arc::clone(&self.state),
        }
    }
}

impl<K, V> QecCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: CacheValue,
{
    /// Creates a new cache using the default FNV-1a integrity verifier.
    pub fn new(
        config: CacheConfig,
    ) -> Result<Self, CacheError> {
        Self::with_integrity(
            config,
            Arc::new(Fnv1aIntegrity),
        )
    }

    /// Creates a cache with a custom integrity verifier.
    pub fn with_integrity(
        config: CacheConfig,
        integrity: Arc<dyn CacheIntegrity>,
    ) -> Result<Self, CacheError> {
        config.validate()?;

        Ok(Self {
            config,
            integrity,
            state: Arc::new(
                Mutex::new(
                    CacheState::new(),
                ),
            ),
        })
    }

    /// Returns the immutable configuration.
    pub fn config(&self) -> &CacheConfig {
        &self.config
    }

    /// Inserts or replaces an entry.
    ///
    /// Returns `true` when an existing entry was replaced and `false` when
    /// a new entry was inserted.
    pub fn insert(
        &self,
        key: K,
        value: V,
    ) -> Result<bool, CacheError> {
        let size_bytes =
            value.estimated_size_bytes();

        if size_bytes > self.config.max_bytes {
            let mut state =
                self.lock_state()?;

            state.stats.rejected_entries =
                state
                    .stats
                    .rejected_entries
                    .saturating_add(1);

            return Err(
                CacheError::EntryTooLarge {
                    size_bytes,
                    max_bytes:
                        self.config.max_bytes,
                },
            );
        }

        let fingerprint =
            self.integrity
                .fingerprint(
                    &value.integrity_bytes(),
                );

        let now = Instant::now();

        let mut state =
            self.lock_state()?;

        let replacing =
            state.entries.contains_key(&key);

        if replacing {
            state.entries.remove(&key);

            state.stats.replacements =
                state
                    .stats
                    .replacements
                    .saturating_add(1);
        }

        let sequence =
            state.next_sequence();

        let generation =
            state.generation;

        state.entries.insert(
            key,
            CacheEntry {
                value,
                size_bytes,
                created_at: now,
                last_access: now,
                sequence,
                generation,
                pinned: false,
                fingerprint,
            },
        );

        state.stats.insertions =
            state
                .stats
                .insertions
                .saturating_add(1);

        state.update_usage();

        self.enforce_limits(&mut state)?;

        Ok(replacing)
    }

    /// Inserts a value and pins it against normal eviction.
    pub fn insert_pinned(
        &self,
        key: K,
        value: V,
    ) -> Result<bool, CacheError> {
        let size_bytes =
            value.estimated_size_bytes();

        if size_bytes > self.config.max_bytes {
            let mut state =
                self.lock_state()?;

            state.stats.rejected_entries =
                state
                    .stats
                    .rejected_entries
                    .saturating_add(1);

            return Err(
                CacheError::EntryTooLarge {
                    size_bytes,
                    max_bytes:
                        self.config.max_bytes,
                },
            );
        }

        let fingerprint =
            self.integrity
                .fingerprint(
                    &value.integrity_bytes(),
                );

        let now = Instant::now();

        let mut state =
            self.lock_state()?;

        let replacing =
            state.entries.contains_key(&key);

        if replacing {
            state.entries.remove(&key);

            state.stats.replacements =
                state
                    .stats
                    .replacements
                    .saturating_add(1);
        }

        let sequence =
            state.next_sequence();

        let generation =
            state.generation;

        state.entries.insert(
            key,
            CacheEntry {
                value,
                size_bytes,
                created_at: now,
                last_access: now,
                sequence,
                generation,
                pinned: true,
                fingerprint,
            },
        );

        state.stats.insertions =
            state
                .stats
                .insertions
                .saturating_add(1);

        state.update_usage();

        self.enforce_limits(&mut state)?;

        Ok(replacing)
    }

    /// Retrieves an entry.
    ///
    /// The returned value is cloned so the internal cache lock is never held
    /// while the caller uses the value.
    pub fn get(
        &self,
        key: &K,
    ) -> Result<V, CacheError> {
        let mut state =
            self.lock_state()?;

        let now = Instant::now();

        let expired =
            match state.entries.get(key) {
                Some(entry) => {
                    entry.is_expired(
                        now,
                        self.config.ttl,
                    )
                }

                None => {
                    state.stats.misses =
                        state
                            .stats
                            .misses
                            .saturating_add(1);

                    return Err(
                        CacheError::NotFound,
                    );
                }
            };

        if expired {
            state.entries.remove(key);

            state.stats.expirations =
                state
                    .stats
                    .expirations
                    .saturating_add(1);

            state.stats.misses =
                state
                    .stats
                    .misses
                    .saturating_add(1);

            state.update_usage();

            return Err(
                CacheError::Expired,
            );
        }

        let verify =
            self.config.verify_integrity;

        if let Some(entry) =
            state.entries.get_mut(key)
        {
            if verify {
                let fingerprint =
                    self.integrity
                        .fingerprint(
                            &entry
                                .value
                                .integrity_bytes(),
                        );

                if fingerprint
                    != entry.fingerprint
                {
                    state.entries.remove(key);

                    state.stats
                        .integrity_failures =
                        state
                            .stats
                            .integrity_failures
                            .saturating_add(1);

                    state.stats.misses =
                        state
                            .stats
                            .misses
                            .saturating_add(1);

                    state.update_usage();

                    return Err(
                        CacheError::IntegrityFailure,
                    );
                }
            }

            entry.last_access = now;

            state.stats.hits =
                state
                    .stats
                    .hits
                    .saturating_add(1);

            return Ok(
                entry.value.clone(),
            );
        }

        state.stats.misses =
            state
                .stats
                .misses
                .saturating_add(1);

        Err(CacheError::NotFound)
    }

    /// Returns a value if present and valid, otherwise computes and inserts it.
    ///
    /// This is the preferred API for expensive QEC structures:
    ///
    /// ```text
    /// cache hit
    ///     -> use cached topology
    ///
    /// cache miss
    ///     -> recompute topology
    ///     -> validate
    ///     -> cache
    /// ```
    pub fn get_or_insert_with<F>(
        &self,
        key: K,
        compute: F,
    ) -> Result<V, CacheError>
    where
        F: FnOnce() -> Result<V, CacheError>,
    {
        match self.get(&key) {
            Ok(value) => {
                return Ok(value);
            }

            Err(
                CacheError::NotFound
                | CacheError::Expired
                | CacheError::IntegrityFailure,
            ) => {}

            Err(error) => {
                return Err(error);
            }
        }

        let value = compute()?;

        self.insert(
            key,
            value.clone(),
        )?;

        Ok(value)
    }

    /// Returns whether a valid entry exists.
    pub fn contains_key(
        &self,
        key: &K,
    ) -> Result<bool, CacheError> {
        match self.get(key) {
            Ok(_) => Ok(true),

            Err(
                CacheError::NotFound
                | CacheError::Expired
                | CacheError::IntegrityFailure,
            ) => Ok(false),

            Err(error) => Err(error),
        }
    }

    /// Removes one entry.
    pub fn remove(
        &self,
        key: &K,
    ) -> Result<Option<V>, CacheError> {
        let mut state =
            self.lock_state()?;

        let removed =
            state.entries.remove(key);

        if removed.is_some() {
            state.stats.invalidations =
                state
                    .stats
                    .invalidations
                    .saturating_add(1);

            state.update_usage();
        }

        Ok(
            removed.map(
                |entry| entry.value,
            ),
        )
    }

    /// Removes all entries matching a predicate.
    pub fn invalidate_where<F>(
        &self,
        predicate: F,
    ) -> Result<usize, CacheError>
    where
        F: Fn(&K) -> bool,
    {
        let mut state =
            self.lock_state()?;

        let keys: Vec<K> =
            state
                .entries
                .keys()
                .filter(|key| predicate(key))
                .cloned()
                .collect();

        let count =
            keys.len();

        for key in keys {
            state.entries.remove(&key);
        }

        state.stats.invalidations =
            state
                .stats
                .invalidations
                .saturating_add(
                    count as u64,
                );

        state.update_usage();

        Ok(count)
    }

    /// Removes expired entries.
    pub fn purge_expired(
        &self,
    ) -> Result<usize, CacheError> {
        let mut state =
            self.lock_state()?;

        let now = Instant::now();

        let keys: Vec<K> =
            state
                .entries
                .iter()
                .filter_map(
                    |(key, entry)| {
                        if entry.is_expired(
                            now,
                            self.config.ttl,
                        ) {
                            Some(key.clone())
                        } else {
                            None
                        }
                    },
                )
                .collect();

        let count =
            keys.len();

        for key in keys {
            state.entries.remove(&key);
        }

        state.stats.expirations =
            state
                .stats
                .expirations
                .saturating_add(
                    count as u64,
                );

        state.update_usage();

        Ok(count)
    }

    /// Pins an existing entry.
    ///
    /// Pinned entries are not selected for normal eviction.
    pub fn pin(
        &self,
        key: &K,
    ) -> Result<(), CacheError> {
        let mut state =
            self.lock_state()?;

        match state.entries.get_mut(key) {
            Some(entry) => {
                entry.pinned = true;
                Ok(())
            }

            None => {
                Err(CacheError::NotFound)
            }
        }
    }

    /// Unpins an existing entry.
    pub fn unpin(
        &self,
        key: &K,
    ) -> Result<(), CacheError> {
        let mut state =
            self.lock_state()?;

        match state.entries.get_mut(key) {
            Some(entry) => {
                entry.pinned = false;
                Ok(())
            }

            None => {
                Err(CacheError::NotFound)
            }
        }
    }

    /// Clears the complete cache.
    pub fn clear(
        &self,
    ) -> Result<(), CacheError> {
        let mut state =
            self.lock_state()?;

        let count =
            state.entries.len();

        state.entries.clear();

        state.stats.invalidations =
            state
                .stats
                .invalidations
                .saturating_add(
                    count as u64,
                );

        state.bump_generation();
        state.update_usage();

        Ok(())
    }

    /// Returns an immutable metrics snapshot.
    pub fn stats(
        &self,
    ) -> Result<CacheStats, CacheError> {
        let state =
            self.lock_state()?;

        Ok(state.stats)
    }

    /// Returns the number of cached entries.
    pub fn len(
        &self,
    ) -> Result<usize, CacheError> {
        let state =
            self.lock_state()?;

        Ok(state.entries.len())
    }

    /// Returns the current estimated memory usage.
    pub fn bytes_used(
        &self,
    ) -> Result<u64, CacheError> {
        let state =
            self.lock_state()?;

        Ok(state.stats.bytes)
    }

    /// Returns whether the cache is empty.
    pub fn is_empty(
        &self,
    ) -> Result<bool, CacheError> {
        Ok(self.len()? == 0)
    }

    /// Manually enforces the configured limits.
    pub fn enforce_limits_now(
        &self,
    ) -> Result<(), CacheError> {
        let mut state =
            self.lock_state()?;

        self.enforce_limits(
            &mut state,
        )
    }

    // -------------------------------------------------------------------------
    // Internal implementation
    // -------------------------------------------------------------------------

    fn lock_state(
        &self,
    ) -> Result<
        std::sync::MutexGuard<
            '_,
            CacheState<K, V>,
        >,
        CacheError,
    > {
        self.state
            .lock()
            .map_err(
                |_| CacheError::LockPoisoned,
            )
    }

    fn enforce_limits(
        &self,
        state: &mut CacheState<K, V>,
    ) -> Result<(), CacheError> {
        self.purge_expired_locked(
            state,
        );

        loop {
            let over_entries =
                state.entries.len()
                    > self.config.max_entries;

            let over_bytes =
                state.stats.bytes
                    > self.config.max_bytes;

            if !over_entries
                && !over_bytes
            {
                break;
            }

            let candidate =
                self.select_eviction_candidate(
                    state,
                );

            let Some(key) = candidate else {
                // Every remaining entry is pinned.
                //
                // This is intentionally not treated as corruption. The cache
                // may temporarily exceed its configured budget when callers
                // explicitly pin more data than the budget allows.
                break;
            };

            if let Some(entry) =
                state.entries.remove(&key)
            {
                state.stats.evictions =
                    state
                        .stats
                        .evictions
                        .saturating_add(1);

                state.stats.bytes =
                    state.stats.bytes
                        .saturating_sub(
                            entry.size_bytes,
                        );
            }
        }

        state.update_usage();

        Ok(())
    }

    fn purge_expired_locked(
        &self,
        state: &mut CacheState<K, V>,
    ) {
        let now =
            Instant::now();

        let keys: Vec<K> =
            state
                .entries
                .iter()
                .filter_map(
                    |(key, entry)| {
                        if entry.is_expired(
                            now,
                            self.config.ttl,
                        ) {
                            Some(key.clone())
                        } else {
                            None
                        }
                    },
                )
                .collect();

        for key in keys {
            if state.entries.remove(&key).is_some() {
                state.stats.expirations =
                    state
                        .stats
                        .expirations
                        .saturating_add(1);
            }
        }

        state.update_usage();
    }

    fn select_eviction_candidate(
        &self,
        state: &CacheState<K, V>,
    ) -> Option<K> {
        state
            .entries
            .iter()
            .filter(
                |(_, entry)| !entry.pinned,
            )
            .min_by(
                |(_, left), (_, right)| {
                    match self.config.eviction_policy {
                        EvictionPolicy::Lru => {
                            left.last_access
                                .cmp(
                                    &right.last_access,
                                )
                                .then_with(
                                    || {
                                        left.sequence
                                            .cmp(
                                                &right.sequence,
                                            )
                                    },
                                )
                        }

                        EvictionPolicy::Fifo => {
                            left.sequence
                                .cmp(
                                    &right.sequence,
                                )
                        }
                    }
                },
            )
            .map(
                |(key, _)| key.clone(),
            )
    }
}

// -----------------------------------------------------------------------------
// Built-in CacheValue implementations
// -----------------------------------------------------------------------------

impl CacheValue for Vec<u8> {
    fn estimated_size_bytes(
        &self,
    ) -> u64 {
        self.len() as u64
    }

    fn integrity_bytes(
        &self,
    ) -> Vec<u8> {
        self.clone()
    }
}

impl CacheValue for String {
    fn estimated_size_bytes(
        &self,
    ) -> u64 {
        self.capacity() as u64
    }

    fn integrity_bytes(
        &self,
    ) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
    )]
    struct TestValue {
        id: u64,
        payload: Vec<u8>,
    }

    impl CacheValue for TestValue {
        fn estimated_size_bytes(
            &self,
        ) -> u64 {
            self.payload.len() as u64
                + std::mem::size_of::<u64>()
                    as u64
        }

        fn integrity_bytes(
            &self,
        ) -> Vec<u8> {
            let mut bytes =
                Vec::with_capacity(
                    8 + self.payload.len(),
                );

            bytes.extend_from_slice(
                &self.id.to_le_bytes(),
            );

            bytes.extend_from_slice(
                &self.payload,
            );

            bytes
        }
    }

    fn value(
        id: u64,
    ) -> TestValue {
        TestValue {
            id,
            payload: vec![
                id as u8;
                8
            ],
        }
    }

    fn cache(
        max_entries: usize,
        max_bytes: u64,
    ) -> QecCache<
        u64,
        TestValue,
    > {
        QecCache::new(
            CacheConfig {
                max_entries,
                max_bytes,
                ttl: None,
                eviction_policy:
                    EvictionPolicy::Lru,
                verify_integrity: true,
            },
        )
        .expect("valid cache")
    }

    #[test]
    fn configuration_rejects_zero_entries() {
        let result =
            QecCache::<
                u64,
                TestValue,
            >::new(
                CacheConfig {
                    max_entries: 0,
                    ..CacheConfig::default()
                },
            );

        assert!(matches!(
            result,
            Err(
                CacheError::InvalidConfiguration(
                    _
                )
            )
        ));
    }

    #[test]
    fn insert_and_get_round_trip() {
        let cache =
            cache(8, 1024);

        let original =
            value(42);

        assert_eq!(
            cache
                .insert(
                    42,
                    original.clone(),
                )
                .expect("insert"),
            false
        );

        assert_eq!(
            cache
                .get(&42)
                .expect("get"),
            original
        );
    }

    #[test]
    fn cache_miss_is_not_found() {
        let cache =
            cache(8, 1024);

        assert_eq!(
            cache.get(&99),
            Err(CacheError::NotFound)
        );
    }

    #[test]
    fn get_or_insert_recomputes_on_miss() {
        let cache =
            cache(8, 1024);

        let mut computations =
            0u32;

        let first =
            cache
                .get_or_insert_with(
                    1,
                    || {
                        computations += 1;

                        Ok(value(1))
                    },
                )
                .expect("first");

        let second =
            cache
                .get_or_insert_with(
                    1,
                    || {
                        computations += 1;

                        Ok(value(2))
                    },
                )
                .expect("second");

        assert_eq!(
            first,
            second
        );

        assert_eq!(
            computations,
            1
        );
    }

    #[test]
    fn lru_eviction_is_deterministic() {
        let cache =
            cache(2, 1024);

        cache
            .insert(1, value(1))
            .expect("insert");

        cache
            .insert(2, value(2))
            .expect("insert");

        let _ =
            cache
                .get(&1)
                .expect("touch entry 1");

        cache
            .insert(3, value(3))
            .expect("insert");

        assert!(
            cache
                .contains_key(&1)
                .expect("contains")
        );

        assert!(
            !cache
                .contains_key(&2)
                .expect("contains")
        );

        assert!(
            cache
                .contains_key(&3)
                .expect("contains")
        );
    }

    #[test]
    fn fifo_eviction_is_deterministic() {
        let cache =
            QecCache::new(
                CacheConfig {
                    max_entries: 2,
                    max_bytes: 1024,
                    ttl: None,
                    eviction_policy:
                        EvictionPolicy::Fifo,
                    verify_integrity: true,
                },
            )
            .expect("valid cache");

        cache
            .insert(1, value(1))
            .expect("insert");

        cache
            .insert(2, value(2))
            .expect("insert");

        let _ =
            cache
                .get(&1)
                .expect("touch entry 1");

        cache
            .insert(3, value(3))
            .expect("insert");

        assert!(
            !cache
                .contains_key(&1)
                .expect("contains")
        );

        assert!(
            cache
                .contains_key(&2)
                .expect("contains")
        );
    }

    #[test]
    fn byte_budget_is_enforced() {
        let cache =
            cache(100, 20);

        cache
            .insert(1, value(1))
            .expect("insert");

        cache
            .insert(2, value(2))
            .expect("insert");

        assert!(
            cache
                .bytes_used()
                .expect("bytes")
                <= 20
        );
    }

    #[test]
    fn oversized_entry_is_rejected() {
        let cache =
            cache(10, 4);

        let result =
            cache.insert(
                1,
                TestValue {
                    id: 1,
                    payload: vec![
                        0;
                        100
                    ],
                },
            );

        assert!(matches!(
            result,
            Err(
                CacheError::EntryTooLarge {
                    ..
                }
            )
        ));
    }

    #[test]
    fn pinned_entries_are_not_evicted() {
        let cache =
            cache(2, 1024);

        cache
            .insert_pinned(
                1,
                value(1),
            )
            .expect("insert");

        cache
            .insert(
                2,
                value(2),
            )
            .expect("insert");

        cache
            .insert(
                3,
                value(3),
            )
            .expect("insert");

        assert!(
            cache
                .contains_key(&1)
                .expect("contains")
        );

        assert!(
            cache
                .contains_key(&3)
                .expect("contains")
        );
    }

    #[test]
    fn explicit_invalidation_works() {
        let cache =
            cache(8, 1024);

        cache
            .insert(1, value(1))
            .expect("insert");

        assert!(
            cache
                .remove(&1)
                .expect("remove")
                .is_some()
        );

        assert_eq!(
            cache.get(&1),
            Err(CacheError::NotFound)
        );
    }

    #[test]
    fn predicate_invalidation_works() {
        let cache =
            cache(8, 1024);

        for id in 0..6 {
            cache
                .insert(
                    id,
                    value(id),
                )
                .expect("insert");
        }

        let removed =
            cache
                .invalidate_where(
                    |key| *key % 2 == 0,
                )
                .expect("invalidate");

        assert_eq!(
            removed,
            3
        );

        assert_eq!(
            cache.len().expect("len"),
            3
        );
    }

    #[test]
    fn clear_removes_everything() {
        let cache =
            cache(8, 1024);

        for id in 0..5 {
            cache
                .insert(
                    id,
                    value(id),
                )
                .expect("insert");
        }

        cache
            .clear()
            .expect("clear");

        assert!(
            cache
                .is_empty()
                .expect("empty")
        );
    }

    #[test]
    fn stats_are_updated() {
        let cache =
            cache(2, 1024);

        cache
            .insert(1, value(1))
            .expect("insert");

        let _ =
            cache
                .get(&1)
                .expect("hit");

        let _ =
            cache
                .get(&99);

        let stats =
            cache
                .stats()
                .expect("stats");

        assert_eq!(
            stats.hits,
            1
        );

        assert_eq!(
            stats.misses,
            1
        );

        assert_eq!(
            stats.insertions,
            1
        );
    }

    #[test]
    fn generation_changes_after_clear() {
        let cache =
            cache(8, 1024);

        let before =
            cache
                .stats()
                .expect("stats")
                .generation;

        cache
            .clear()
            .expect("clear");

        let after =
            cache
                .stats()
                .expect("stats")
                .generation;

        assert!(
            after > before
        );
    }

    #[test]
    fn integrity_failure_discards_entry() {
        let cache =
            cache(8, 1024);

        cache
            .insert(
                1,
                value(1),
            )
            .expect("insert");

        // The test intentionally uses a custom verifier whose output changes
        // between calls to emulate detected cache corruption.
        let counter =
            Arc::new(
                Mutex::new(0u64),
            );

        let counter_clone =
            Arc::clone(&counter);

        let integrity =
            FunctionIntegrity::new(
                move |bytes| {
                    let mut count =
                        counter_clone
                            .lock()
                            .expect("lock");

                    *count += 1;

                    Fnv1aIntegrity
                        .fingerprint(
                            bytes,
                        )
                        .wrapping_add(
                            *count,
                        )
                },
            );

        let corruptible =
            QecCache::with_integrity(
                CacheConfig::default(),
                Arc::new(integrity),
            )
            .expect("cache");

        corruptible
            .insert(
                1,
                value(1),
            )
            .expect("insert");

        let result =
            corruptible.get(&1);

        assert!(matches!(
            result,
            Err(
                CacheError::IntegrityFailure
            )
        ));

        assert!(
            corruptible
                .is_empty()
                .expect("empty")
        );
    }

    #[test]
    fn concurrent_access_is_safe() {
        let cache =
            cache(128, 64 * 1024);

        let mut handles =
            Vec::new();

        for worker in 0..8u64 {
            let cache =
                cache.clone();

            handles.push(
                thread::spawn(
                    move || {
                        for index in 0..100u64 {
                            let key =
                                worker
                                    * 100
                                    + index;

                            cache
                                .insert(
                                    key,
                                    value(key),
                                )
                                .expect(
                                    "insert",
                                );

                            let _ =
                                cache
                                    .get(&key);
                        }
                    },
                ),
            );
        }

        for handle in handles {
            handle
                .join()
                .expect(
                    "worker thread",
                );
        }

        assert!(
            cache
                .len()
                .expect("len")
                <= 128
        );
    }

    #[test]
    fn pinned_entries_can_temporarily_exceed_budget() {
        let cache =
            cache(1, 1_000);

        cache
            .insert_pinned(
                1,
                value(1),
            )
            .expect("insert");

        cache
            .insert_pinned(
                2,
                value(2),
            )
            .expect("insert");

        // No unpinned entry is available for eviction.
        assert_eq!(
            cache.len().expect("len"),
            2
        );
    }

    #[test]
    fn purge_expired_is_safe_without_ttl() {
        let cache =
            cache(8, 1024);

        cache
            .insert(1, value(1))
            .expect("insert");

        assert_eq!(
            cache
                .purge_expired()
                .expect("purge"),
            0
        );

        assert!(
            cache
                .contains_key(&1)
                .expect("contains")
        );
    }

    #[test]
    fn ttl_expiration_removes_entries() {
        let cache =
            QecCache::new(
                CacheConfig {
                    max_entries: 8,
                    max_bytes: 1024,
                    ttl: Some(
                        Duration::from_millis(
                            1,
                        ),
                    ),
                    eviction_policy:
                        EvictionPolicy::Lru,
                    verify_integrity: true,
                },
            )
            .expect("cache");

        cache
            .insert(1, value(1))
            .expect("insert");

        thread::sleep(
            Duration::from_millis(5),
        );

        assert_eq!(
            cache.get(&1),
            Err(CacheError::Expired)
        );
    }

    #[test]
    fn unpin_allows_eviction() {
        let cache =
            cache(1, 1024);

        cache
            .insert_pinned(
                1,
                value(1),
            )
            .expect("insert");

        cache
            .unpin(&1)
            .expect("unpin");

        cache
            .insert(
                2,
                value(2),
            )
            .expect("insert");

        assert!(
            !cache
                .contains_key(&1)
                .expect("contains")
        );

        assert!(
            cache
                .contains_key(&2)
                .expect("contains")
        );
    }

    #[test]
    fn repeated_clear_operations_are_safe() {
        let cache =
            cache(8, 1024);

        cache
            .clear()
            .expect("clear");

        cache
            .clear()
            .expect("clear");

        assert!(
            cache
                .is_empty()
                .expect("empty")
        );
    }

    #[test]
    fn eviction_statistics_are_recorded() {
        let cache =
            cache(1, 1024);

        cache
            .insert(1, value(1))
            .expect("insert");

        cache
            .insert(2, value(2))
            .expect("insert");

        let stats =
            cache
                .stats()
                .expect("stats");

        assert_eq!(
            stats.evictions,
            1
        );
    }
}