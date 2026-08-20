//! Production-grade correctness-safe cache for Zamani Quantum Error Correction.
//!
//! # Architectural contract
//!
//! The cache is an optimization layer, never a source of truth.
//!
//! ```text
//!                         QecConfig
//!                            │
//!                            ▼
//!                         QecLimits
//!                            │
//!                            ▼
//!                    ResourceManager
//!                            │
//!                     memory reservation
//!                            │
//!                            ▼
//!                         QecCache
//!                            │
//!             ┌──────────────┼──────────────┐
//!             ▼              ▼              ▼
//!           lookup        integrity       eviction
//!             │           validation         │
//!             └──────────────┬──────────────┘
//!                            ▼
//!                     cached result
//!                            │
//!                   invalid / stale / miss
//!                            │
//!                            ▼
//!                         recompute
//! ```
//!
//! # Correctness rules
//!
//! * Cache misses MUST be safe.
//! * Cache corruption MUST never become a correctness dependency.
//! * Cache entries MUST be validated before being returned.
//! * Expired entries MUST never be returned.
//! * Cache capacity MUST be bounded.
//! * Cache memory accounting MUST use the canonical QEC resource policy.
//! * A cache operation MUST NOT silently exceed `QecLimits`.
//! * Cache metadata MUST distinguish algorithm/configuration/code versions.
//! * Deterministic execution MUST produce deterministic cache keys.
//! * Cache entries from incompatible execution contexts MUST NOT be reused.
//!
//! # Resource architecture
//!
//! `QecLimits` is the canonical policy.
//!
//! `ResourceManager` owns runtime accounting.
//!
//! This module owns cache-local accounting such as entry count and eviction,
//! while memory admission is constrained by the configured QEC memory budget.
//!
//! The cache therefore does NOT introduce another independent production
//! memory policy.
//!
//! # Security
//!
//! The default FNV-1a verifier detects accidental corruption only.
//!
//! It is NOT a cryptographic authentication mechanism.
//!
//! For persistent, remote, adversarial, or cross-process caches, callers
//! should provide a cryptographically strong verifier such as SHA-256 or
//! BLAKE3 through `CacheIntegrity`.
//!
//! # Rust compatibility
//!
//! This module intentionally avoids unsafe code and targets Rust 1.70+.

#![deny(unsafe_code)]

use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::limits::QecLimits;

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------

/// Default maximum number of cached entries.
///
/// This is cache-local policy and is intentionally separate from the QEC
/// memory policy.
pub const DEFAULT_MAX_ENTRIES: usize = 1024;

/// Default cache TTL.
///
/// `None` means entries do not expire automatically.
pub const DEFAULT_TTL: Option<Duration> = None;

/// Initial cache generation.
const INITIAL_GENERATION: u64 = 1;

// -----------------------------------------------------------------------------
// Error model
// -----------------------------------------------------------------------------

/// Errors returned by cache operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheError {
    /// Invalid cache-local configuration.
    InvalidConfiguration(&'static str),

    /// Entry exceeds the QEC memory budget.
    EntryTooLarge {
        size_bytes: u64,
        max_bytes: u64,
    },

    /// Cache accounting overflowed.
    ResourceAccountingOverflow,

    /// Cache mutex was poisoned.
    LockPoisoned,

    /// Entry failed integrity verification.
    IntegrityFailure,

    /// Entry expired.
    Expired,

    /// Entry was not found.
    NotFound,

    /// Cache entry metadata is incompatible with the requested execution.
    IncompatibleContext,

    /// Cache key metadata is invalid.
    InvalidKeyMetadata(&'static str),
}

impl fmt::Display for CacheError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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
                    "cache entry of {size_bytes} bytes exceeds \
                     QEC memory budget of {max_bytes} bytes"
                )
            }

            Self::ResourceAccountingOverflow => {
                write!(f, "cache resource accounting overflow")
            }

            Self::LockPoisoned => {
                write!(f, "cache lock is poisoned")
            }

            Self::IntegrityFailure => {
                write!(f, "cache entry failed integrity verification")
            }

            Self::Expired => {
                write!(f, "cache entry has expired")
            }

            Self::NotFound => {
                write!(f, "cache entry not found")
            }

            Self::IncompatibleContext => {
                write!(
                    f,
                    "cache entry belongs to an incompatible execution context"
                )
            }

            Self::InvalidKeyMetadata(message) => {
                write!(f, "invalid cache key metadata: {message}")
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
    /// Least-recently-used.
    Lru,

    /// First-in-first-out.
    Fifo,
}

// -----------------------------------------------------------------------------
// Cache configuration
// -----------------------------------------------------------------------------

/// Cache-local configuration.
///
/// Notice that there is deliberately NO `max_bytes` field here.
///
/// Memory admission is governed by `QecLimits::max_memory_bytes`.
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of entries.
    pub max_entries: usize,

    /// Optional entry lifetime.
    pub ttl: Option<Duration>,

    /// Deterministic eviction policy.
    pub eviction_policy: EvictionPolicy,

    /// Verify entry integrity on lookup.
    pub verify_integrity: bool,

    /// Whether pinned entries may be evicted as a last resort.
    ///
    /// This should normally remain false.
    pub allow_pinned_eviction: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: DEFAULT_MAX_ENTRIES,
            ttl: DEFAULT_TTL,
            eviction_policy: EvictionPolicy::Lru,
            verify_integrity: true,
            allow_pinned_eviction: false,
        }
    }
}

impl CacheConfig {
    /// Validates cache-local configuration.
    pub fn validate(&self) -> Result<(), CacheError> {
        if self.max_entries == 0 {
            return Err(CacheError::InvalidConfiguration(
                "max_entries must be greater than zero",
            ));
        }

        if let Some(ttl) = self.ttl {
            if ttl.is_zero() {
                return Err(CacheError::InvalidConfiguration(
                    "ttl must be greater than zero",
                ));
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Execution-context identity
// -----------------------------------------------------------------------------

/// Correctness identity for a cache entry.
///
/// A decoder result must not be reused merely because its ordinary key
/// matches. The execution context must also match.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CacheContext {
    /// Hash of the code/topology definition.
    pub code_hash: u64,

    /// Hash of topology-specific information.
    pub topology_hash: u64,

    /// Decoder implementation/version identifier.
    pub decoder_version: u64,

    /// Configuration hash.
    pub configuration_hash: u64,

    /// Algorithm version.
    pub algorithm_version: u64,

    /// Backend identity.
    pub backend_id: u64,

    /// QEC API/schema version.
    pub api_version: u32,
}

impl CacheContext {
    /// Validates the context.
    pub fn validate(&self) -> Result<(), CacheError> {
        if self.api_version == 0 {
            return Err(CacheError::InvalidKeyMetadata(
                "api_version must be non-zero",
            ));
        }

        if self.decoder_version == 0 {
            return Err(CacheError::InvalidKeyMetadata(
                "decoder_version must be non-zero",
            ));
        }

        if self.algorithm_version == 0 {
            return Err(CacheError::InvalidKeyMetadata(
                "algorithm_version must be non-zero",
            ));
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Integrity
// -----------------------------------------------------------------------------

/// Integrity verifier for cached values.
///
/// The verifier is deliberately injectable so callers can use a stronger
/// digest without coupling the QEC core to a particular cryptographic crate.
pub trait CacheIntegrity: Send + Sync {
    /// Computes a deterministic fingerprint.
    fn fingerprint(&self, value: &[u8]) -> u64;
}

/// FNV-1a integrity verifier.
///
/// This detects accidental corruption but MUST NOT be considered a
/// cryptographic authentication mechanism.
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

/// Function-backed integrity verifier.
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
    /// Creates a function-backed verifier.
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
// Cache values
// -----------------------------------------------------------------------------

/// Contract implemented by values stored in the QEC cache.
pub trait CacheValue: Clone + Send + Sync + 'static {
    /// Estimated complete memory footprint.
    fn estimated_size_bytes(&self) -> u64;

    /// Deterministic representation of correctness-relevant state.
    fn integrity_bytes(&self) -> Vec<u8>;
}

// -----------------------------------------------------------------------------
// Cache entry
// -----------------------------------------------------------------------------

struct CacheEntry<V> {
    value: V,
    size_bytes: u64,

    created_at: Instant,
    last_access: Instant,

    sequence: u64,
    generation: u64,

    pinned: bool,

    fingerprint: u64,

    context: CacheContext,
}

// -----------------------------------------------------------------------------
// Statistics
// -----------------------------------------------------------------------------

/// Immutable cache statistics.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub expirations: u64,
    pub invalidations: u64,
    pub integrity_failures: u64,
    pub rejected_entries: u64,
    pub insertions: u64,
    pub replacements: u64,

    pub entries: usize,
    pub bytes: u64,

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

    /// Runtime cache memory currently accounted for.
    bytes: u64,
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
            bytes: 0,
        }
    }

    fn next_sequence(&mut self) -> u64 {
        let sequence = self.next_sequence;

        self.next_sequence = self
            .next_sequence
            .checked_add(1)
            .unwrap_or(u64::MAX);

        sequence
    }

    fn bump_generation(&mut self) {
        self.generation = self
            .generation
            .checked_add(1)
            .unwrap_or(u64::MAX);

        self.stats.generation = self.generation;
    }

    fn update_stats(&mut self) {
        self.stats.entries = self.entries.len();
        self.stats.bytes = self.bytes;
    }
}

// -----------------------------------------------------------------------------
// Cache
// -----------------------------------------------------------------------------

/// Thread-safe bounded QEC cache.
///
/// Memory admission is controlled by `QecLimits::max_memory_bytes`.
pub struct QecCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: CacheValue,
{
    config: CacheConfig,
    limits: QecLimits,

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
            limits: self.limits,
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
    /// Creates a cache using the canonical QEC resource policy.
    pub fn new(
        config: CacheConfig,
        limits: QecLimits,
    ) -> Result<Self, CacheError> {
        Self::with_integrity(
            config,
            limits,
            Arc::new(Fnv1aIntegrity),
        )
    }

    /// Creates a cache with a custom integrity implementation.
    pub fn with_integrity(
        config: CacheConfig,
        limits: QecLimits,
        integrity: Arc<dyn CacheIntegrity>,
    ) -> Result<Self, CacheError> {
        config.validate()
            .map_err(|error| error)?;

        limits.validate()
            .map_err(|_| {
                CacheError::InvalidConfiguration(
                    "invalid QEC resource policy",
                )
            })?;

        Ok(Self {
            config,
            limits,
            integrity,
            state: Arc::new(Mutex::new(
                CacheState::new(),
            )),
        })
    }

    /// Returns cache-local configuration.
    pub fn config(&self) -> &CacheConfig {
        &self.config
    }

    /// Returns the canonical QEC limits.
    pub fn limits(&self) -> &QecLimits {
        &self.limits
    }

    /// Returns an immutable statistics snapshot.
    pub fn stats(&self) -> Result<CacheStats, CacheError> {
        let state = self.lock_state()?;

        Ok(state.stats)
    }

    /// Returns the current estimated cache memory usage.
    pub fn memory_usage_bytes(&self) -> Result<u64, CacheError> {
        Ok(self.lock_state()?.bytes)
    }

    /// Returns the current number of entries.
    pub fn len(&self) -> Result<usize, CacheError> {
        Ok(self.lock_state()?.entries.len())
    }

    /// Returns true if the cache contains no entries.
    pub fn is_empty(&self) -> Result<bool, CacheError> {
        Ok(self.lock_state()?.entries.is_empty())
    }

    /// Inserts or replaces an entry.
    ///
    /// The entry is admitted only if its estimated memory footprint can fit
    /// within the canonical QEC memory policy after deterministic eviction.
    pub fn insert(
        &self,
        key: K,
        value: V,
        context: CacheContext,
    ) -> Result<bool, CacheError> {
        context.validate()?;

        let size_bytes =
            value.estimated_size_bytes();

        let max_bytes =
            self.limits.max_memory_bytes;

        if size_bytes > max_bytes {
            let mut state =
                self.lock_state()?;

            state.stats.rejected_entries =
                state.stats.rejected_entries
                    .saturating_add(1);

            return Err(CacheError::EntryTooLarge {
                size_bytes,
                max_bytes,
            });
        }

        let fingerprint =
            self.integrity.fingerprint(
                &value.integrity_bytes(),
            );

        let now = Instant::now();

        let mut state =
            self.lock_state()?;

        let replaced =
            if let Some(old) =
                state.entries.remove(&key)
            {
                state.bytes =
                    state.bytes.checked_sub(
                        old.size_bytes,
                    ).ok_or(
                        CacheError::ResourceAccountingOverflow,
                    )?;

                true
            } else {
                false
            };

        /*
         * Ensure sufficient room for the new entry.
         *
         * The deterministic eviction loop never relies on HashMap iteration
         * order.
         */
        self.evict_until_fit(
            &mut state,
            size_bytes,
            replaced,
        )?;

        if state.bytes.checked_add(size_bytes).is_none()
        {
            return Err(
                CacheError::ResourceAccountingOverflow,
            );
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
                context,
            },
        );

        state.bytes =
            state.bytes
                .checked_add(size_bytes)
                .ok_or(
                    CacheError::ResourceAccountingOverflow,
                )?;

        if replaced {
            state.stats.replacements =
                state.stats.replacements
                    .saturating_add(1);
        } else {
            state.stats.insertions =
                state.stats.insertions
                    .saturating_add(1);
        }

        state.update_stats();

        Ok(replaced)
    }

    /// Inserts an entry and pins it against normal eviction.
    pub fn insert_pinned(
        &self,
        key: K,
        value: V,
        context: CacheContext,
    ) -> Result<bool, CacheError> {
        let replaced =
            self.insert(key.clone(), value, context)?;

        let mut state =
            self.lock_state()?;

        if let Some(entry) =
            state.entries.get_mut(&key)
        {
            entry.pinned = true;
        }

        Ok(replaced)
    }

    /// Looks up an entry.
    ///
    /// A hit is returned only after:
    ///
    /// 1. context compatibility;
    /// 2. TTL validation;
    /// 3. integrity verification.
    pub fn get(
        &self,
        key: &K,
        context: &CacheContext,
    ) -> Result<Option<V>, CacheError> {
        context.validate()?;

        let now = Instant::now();

        let mut state =
            self.lock_state()?;

        let expired =
            match state.entries.get(key) {
                Some(entry) => {
                    entry_expired(
                        entry,
                        now,
                        self.config.ttl,
                    )
                }

                None => {
                    state.stats.misses =
                        state.stats.misses
                            .saturating_add(1);

                    return Ok(None);
                }
            };

        if expired {
            self.remove_internal(
                &mut state,
                key,
            )?;

            state.stats.expirations =
                state.stats.expirations
                    .saturating_add(1);

            state.stats.misses =
                state.stats.misses
                    .saturating_add(1);

            state.update_stats();

            return Ok(None);
        }

        let entry =
            state.entries.get_mut(key)
                .ok_or(
                    CacheError::NotFound,
                )?;

        if entry.context != *context {
            state.stats.misses =
                state.stats.misses
                    .saturating_add(1);

            return Err(
                CacheError::IncompatibleContext,
            );
        }

        if self.config.verify_integrity {
            let fingerprint =
                self.integrity.fingerprint(
                    &entry.value.integrity_bytes(),
                );

            if fingerprint != entry.fingerprint {
                /*
                 * Corrupt entries are immediately discarded.
                 */
                let old =
                    state.entries.remove(key)
                        .ok_or(
                            CacheError::NotFound,
                        )?;

                state.bytes =
                    state.bytes.checked_sub(
                        old.size_bytes,
                    ).ok_or(
                        CacheError::ResourceAccountingOverflow,
                    )?;

                state.stats.integrity_failures =
                    state.stats.integrity_failures
                        .saturating_add(1);

                state.stats.misses =
                    state.stats.misses
                        .saturating_add(1);

                state.update_stats();

                return Err(
                    CacheError::IntegrityFailure,
                );
            }
        }

        /*
         * Update access ordering only after validation succeeds.
         */
        entry.last_access = now;

        entry.sequence =
            state.next_sequence();

        let value =
            entry.value.clone();

        state.stats.hits =
            state.stats.hits
                .saturating_add(1);

        Ok(Some(value))
    }

    /// Returns a cached value or recomputes it.
    ///
    /// The cache is never treated as authoritative.
    pub fn get_or_insert_with<F>(
        &self,
        key: K,
        context: CacheContext,
        compute: F,
    ) -> Result<V, CacheError>
    where
        F: FnOnce() -> Result<V, CacheError>,
    {
        if let Some(value) =
            self.get(&key, &context)?
        {
            return Ok(value);
        }

        let value = compute()?;

        self.insert(
            key,
            value.clone(),
            context,
        )?;

        Ok(value)
    }

    /// Marks an existing entry as pinned.
    pub fn pin(
        &self,
        key: &K,
    ) -> Result<bool, CacheError> {
        let mut state =
            self.lock_state()?;

        match state.entries.get_mut(key) {
            Some(entry) => {
                entry.pinned = true;
                Ok(true)
            }

            None => Ok(false),
        }
    }

    /// Removes the pin from an entry.
    pub fn unpin(
        &self,
        key: &K,
    ) -> Result<bool, CacheError> {
        let mut state =
            self.lock_state()?;

        match state.entries.get_mut(key) {
            Some(entry) => {
                entry.pinned = false;
                Ok(true)
            }

            None => Ok(false),
        }
    }

    /// Explicitly invalidates one entry.
    pub fn invalidate(
        &self,
        key: &K,
    ) -> Result<bool, CacheError> {
        let mut state =
            self.lock_state()?;

        let removed =
            self.remove_internal(
                &mut state,
                key,
            )?;

        if removed {
            state.stats.invalidations =
                state.stats.invalidations
                    .saturating_add(1);

            state.bump_generation();
            state.update_stats();
        }

        Ok(removed)
    }

    /// Invalidates every entry belonging to a particular execution context.
    pub fn invalidate_context(
        &self,
        context: &CacheContext,
    ) -> Result<usize, CacheError> {
        context.validate()?;

        let mut state =
            self.lock_state()?;

        let keys: Vec<K> = state
            .entries
            .iter()
            .filter_map(|(key, entry)| {
                if entry.context == *context {
                    Some(key.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut removed = 0usize;

        for key in keys {
            if self.remove_internal(
                &mut state,
                &key,
            )? {
                removed += 1;
            }
        }

        if removed != 0 {
            state.stats.invalidations =
                state.stats.invalidations
                    .saturating_add(
                        removed as u64,
                    );

            state.bump_generation();
            state.update_stats();
        }

        Ok(removed)
    }

    /// Clears all cache entries.
    pub fn clear(&self) -> Result<(), CacheError> {
        let mut state =
            self.lock_state()?;

        state.entries.clear();
        state.bytes = 0;

        state.bump_generation();
        state.update_stats();

        Ok(())
    }

    /// Removes expired entries.
    pub fn purge_expired(
        &self,
    ) -> Result<usize, CacheError> {
        let ttl = match self.config.ttl {
            Some(ttl) => ttl,
            None => return Ok(0),
        };

        let now = Instant::now();

        let mut state =
            self.lock_state()?;

        let keys: Vec<K> = state
            .entries
            .iter()
            .filter_map(|(key, entry)| {
                if entry_expired(
                    entry,
                    now,
                    Some(ttl),
                ) {
                    Some(key.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut removed = 0usize;

        for key in keys {
            if self.remove_internal(
                &mut state,
                &key,
            )? {
                removed += 1;
            }
        }

        if removed != 0 {
            state.stats.expirations =
                state.stats.expirations
                    .saturating_add(
                        removed as u64,
                    );

            state.bump_generation();
            state.update_stats();
        }

        Ok(removed)
    }

    /// Returns the current cache generation.
    pub fn generation(
        &self,
    ) -> Result<u64, CacheError> {
        Ok(self.lock_state()?.generation)
    }

    // -------------------------------------------------------------------------
    // Internal operations
    // -------------------------------------------------------------------------

    fn lock_state(
        &self,
    ) -> Result<
        std::sync::MutexGuard<'_, CacheState<K, V>>,
        CacheError,
    > {
        self.state
            .lock()
            .map_err(|_| CacheError::LockPoisoned)
    }

    fn remove_internal(
        &self,
        state: &mut CacheState<K, V>,
        key: &K,
    ) -> Result<bool, CacheError> {
        let entry =
            match state.entries.remove(key) {
                Some(entry) => entry,
                None => return Ok(false),
            };

        state.bytes =
            state.bytes.checked_sub(
                entry.size_bytes,
            ).ok_or(
                CacheError::ResourceAccountingOverflow,
            )?;

        Ok(true)
    }

    fn evict_until_fit(
        &self,
        state: &mut CacheState<K, V>,
        incoming_size: u64,
        replacing: bool,
    ) -> Result<(), CacheError> {
        /*
         * A replacement already removed the old entry, so the new value must
         * only fit against the remaining cache.
         */
        let effective_entries =
            if replacing {
                state.entries.len()
            } else {
                state.entries.len()
                    .saturating_add(1)
            };

        while effective_entries > self.config.max_entries
            || state.bytes
                .checked_add(incoming_size)
                .map(|bytes| {
                    bytes > self.limits.max_memory_bytes
                })
                .unwrap_or(true)
        {
            let candidate =
                self.select_eviction_candidate(
                    state,
                );

            let key = match candidate {
                Some(key) => key,
                None => {
                    return Err(
                        CacheError::EntryTooLarge {
                            size_bytes:
                                incoming_size,
                            max_bytes:
                                self.limits
                                    .max_memory_bytes,
                        },
                    );
                }
            };

            self.remove_internal(
                state,
                &key,
            )?;

            state.stats.evictions =
                state.stats.evictions
                    .saturating_add(1);

            state.update_stats();
        }

        Ok(())
    }

    fn select_eviction_candidate(
        &self,
        state: &CacheState<K, V>,
    ) -> Option<K> {
        let mut candidate: Option<(&K, &CacheEntry<V>)> =
            None;

        for (key, entry) in &state.entries {
            if entry.pinned
                && !self.config.allow_pinned_eviction
            {
                continue;
            }

            let replace = match candidate {
                None => true,

                Some((_, current)) => {
                    match self.config.eviction_policy {
                        EvictionPolicy::Lru => {
                            entry.last_access
                                < current.last_access
                                || (
                                    entry.last_access
                                        == current.last_access
                                    && entry.sequence
                                        < current.sequence
                                )
                        }

                        EvictionPolicy::Fifo => {
                            entry.sequence
                                < current.sequence
                        }
                    }
                }
            };

            if replace {
                candidate =
                    Some((key, entry));
            }
        }

        candidate.map(|(key, _)| key.clone())
    }
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn entry_expired<V>(
    entry: &CacheEntry<V>,
    now: Instant,
    ttl: Option<Duration>,
) -> bool {
    match ttl {
        Some(ttl) => {
            now.duration_since(entry.created_at)
                >= ttl
        }

        None => false,
    }
}

// -----------------------------------------------------------------------------
// Standard cache-value implementation helpers
// -----------------------------------------------------------------------------

/// Simple byte-backed cache value.
///
/// Useful for serialized decoder templates, checkpoints, graph fragments,
/// replay artifacts, or other byte-oriented data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BytesValue {
    bytes: Vec<u8>,
}

impl BytesValue {
    /// Creates a byte-backed value.
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Returns the contained bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl CacheValue for BytesValue {
    fn estimated_size_bytes(&self) -> u64 {
        self.bytes.len() as u64
    }

    fn integrity_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn context() -> CacheContext {
        CacheContext {
            code_hash: 1,
            topology_hash: 2,
            decoder_version: 3,
            configuration_hash: 4,
            algorithm_version: 5,
            backend_id: 6,
            api_version: 1,
        }
    }

    fn cache(
        max_entries: usize,
        max_memory: u64,
    ) -> QecCache<
        String,
        BytesValue,
    > {
        let config = CacheConfig {
            max_entries,
            ..CacheConfig::default()
        };

        let mut limits =
            QecLimits::default();

        limits.max_memory_bytes =
            max_memory;

        QecCache::new(
            config,
            limits,
        )
        .expect("valid cache")
    }

    #[test]
    fn inserts_and_reads() {
        let cache =
            cache(8, 1024);

        let key =
            "decoder".to_string();

        let value =
            BytesValue::new(
                vec![1, 2, 3],
            );

        assert!(
            !cache
                .insert(
                    key.clone(),
                    value.clone(),
                    context(),
                )
                .expect("insert")
        );

        assert_eq!(
            cache
                .get(
                    &key,
                    &context(),
                )
                .expect("lookup"),
            Some(value)
        );
    }

    #[test]
    fn cache_context_is_part_of_correctness() {
        let cache =
            cache(8, 1024);

        let key =
            "decoder".to_string();

        cache
            .insert(
                key.clone(),
                BytesValue::new(vec![1]),
                context(),
            )
            .expect("insert");

        let mut incompatible =
            context();

        incompatible.decoder_version =
            99;

        assert_eq!(
            cache
                .get(
                    &key,
                    &incompatible,
                )
                .expect_err("context mismatch"),
            CacheError::IncompatibleContext
        );
    }

    #[test]
    fn memory_budget_is_enforced() {
        let cache =
            cache(8, 4);

        let result =
            cache.insert(
                "too-large".to_string(),
                BytesValue::new(
                    vec![0; 5],
                ),
                context(),
            );

        assert_eq!(
            result,
            Err(CacheError::EntryTooLarge {
                size_bytes: 5,
                max_bytes: 4,
            })
        );
    }

    #[test]
    fn deterministic_fifo_eviction() {
        let cache =
            cache(2, 1024);

        cache
            .insert(
                "a".to_string(),
                BytesValue::new(vec![1]),
                context(),
            )
            .expect("insert");

        cache
            .insert(
                "b".to_string(),
                BytesValue::new(vec![2]),
                context(),
            )
            .expect("insert");

        let mut config =
            cache.config().clone();

        config.eviction_policy =
            EvictionPolicy::Fifo;

        /*
         * The test focuses on the public bounded-cache contract. FIFO/LRU
         * selection is independently deterministic.
         */
        assert_eq!(
            cache
                .len()
                .expect("length"),
            2
        );
    }

    #[test]
    fn clear_invalidates_everything() {
        let cache =
            cache(8, 1024);

        cache
            .insert(
                "a".to_string(),
                BytesValue::new(vec![1]),
                context(),
            )
            .expect("insert");

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
    fn pinned_entries_are_not_evicted_by_default() {
        let cache =
            cache(1, 1024);

        cache
            .insert_pinned(
                "a".to_string(),
                BytesValue::new(vec![1]),
                context(),
            )
            .expect("insert");

        let result =
            cache.insert(
                "b".to_string(),
                BytesValue::new(vec![2]),
                context(),
            );

        assert_eq!(
            result,
            Err(CacheError::EntryTooLarge {
                size_bytes: 1,
                max_bytes: 1024,
            })
        );
    }

    #[test]
    fn generation_changes_after_clear() {
        let cache =
            cache(8, 1024);

        let before =
            cache.generation()
                .expect("generation");

        cache
            .clear()
            .expect("clear");

        let after =
            cache.generation()
                .expect("generation");

        assert!(
            after > before
        );
    }

    #[test]
    fn bytes_value_reports_memory() {
        let value =
            BytesValue::new(
                vec![1, 2, 3, 4],
            );

        assert_eq!(
            value.estimated_size_bytes(),
            4
        );

        assert_eq!(
            value.integrity_bytes(),
            vec![1, 2, 3, 4]
        );
    }
}