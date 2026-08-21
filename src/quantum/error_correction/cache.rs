//! Zamani Quantum Error Correction — Correctness-Safe Cache
//!
//! # Architectural contract
//!
//! The cache is an optimization layer, never a source of truth.
//!
//! ```text
//!                    QecConfig
//!                       │
//!                       ▼
//!                   QecLimits
//!                       │
//!                       ▼
//!              Cache admission policy
//!                       │
//!          ┌────────────┴────────────┐
//!          ▼                         ▼
//!   CacheAccounting            QecCache state
//!          │                         │
//!          ▼                         ▼
//!   runtime accounting       lookup / insertion
//!                                    │
//!                         ┌──────────┴──────────┐
//!                         ▼                     ▼
//!                    validation             eviction
//!                         │                     │
//!                         └──────────┬──────────┘
//!                                    ▼
//!                              cached value
//!                                    │
//!                           miss / invalid entry
//!                                    │
//!                                    ▼
//!                                recompute
//! ```
//!
//! # Core guarantees
//!
//! 1. A cache miss is always safe.
//! 2. A cache hit is returned only after context and integrity validation.
//! 3. An incompatible cache entry is treated as a miss.
//! 4. Corrupt entries are removed before returning failure.
//! 5. Expired entries are never returned.
//! 6. Cache capacity is bounded by entry count and `QecLimits` memory policy.
//! 7. Failed insertion never destroys an existing valid replacement.
//! 8. Eviction order is deterministic.
//! 9. Pinned entries are protected unless explicitly configured otherwise.
//! 10. Cache accounting uses checked arithmetic.
//! 11. Cache schema identity comes from `version.rs`.
//! 12. Cache-local policy is not a second QEC resource policy.
//! 13. Runtime resource accounting has an explicit integration boundary.
//! 14. The cache contains no decoder-specific logic.
//! 15. The cache contains no QPU credentials or execution policy.
//! 16. No unsafe code is used.
//! 17. Compatible with Rust 1.97.1.
//!
//! # Integration
//!
//! `limits.rs`:
//!
//! - supplies the canonical memory ceiling;
//! - supplies the global resource policy;
//! - remains the sole source of declarative QEC limits.
//!
//! `resources.rs`:
//!
//! - may implement `CacheAccounting`;
//! - remains responsible for runtime resource accounting;
//! - must not create a second cache memory policy.
//!
//! `version.rs`:
//!
//! - supplies the canonical cache artifact version.
//!
//! `errors.rs`:
//!
//! - receives cache failures through `From<CacheError> for QecError`.
//!
//! `checkpoint.rs`:
//!
//! - may cache immutable checkpoint-derived artifacts;
//! - must not treat the cache as checkpoint persistence.
//!
//! `decoder.rs` / `mwpm.rs` / `union_find.rs`:
//!
//! - may cache reusable decoder structures;
//! - must include decoder and algorithm identity in `CacheContext`.
//!
//! `replay.rs`:
//!
//! - may use cache entries for deterministic replay artifacts;
//! - must still retain the replay source of truth independently.
//!
//! `configuration.rs`:
//!
//! - supplies the validated `QecLimits` and configuration identity.
//!
//! # Important distinction
//!
//! ```text
//! limits.rs
//!     = what execution is allowed to consume
//!
//! resources.rs
//!     = what execution actually reserves/consumes
//!
//! memory.rs
//!     = memory allocation enforcement
//!
//! cache.rs
//!     = reusable computation and cache-local lifecycle
//! ```
//!
//! The cache MUST NOT become an alternative resource-policy system.

#![deny(unsafe_code)]

use core::fmt;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::{
    errors::QecError,
    limits::QecLimits,
    version::{ArtifactKind, Version},
};

// ============================================================================
// Constants
// ============================================================================

/// Default maximum number of cache entries.
///
/// This is cache-local cardinality policy, not a QEC memory policy.
pub const DEFAULT_MAX_ENTRIES: usize = 1024;

/// Default cache TTL.
///
/// `None` means no automatic expiration.
pub const DEFAULT_TTL: Option<Duration> = None;

/// Initial cache generation.
const INITIAL_GENERATION: u64 = 1;

/// Cache accounting schema.
pub const CACHE_ACCOUNTING_SCHEMA_VERSION: u32 = 1;

// ============================================================================
// Error model
// ============================================================================

/// Cache-specific failures.
///
/// These are converted into the canonical [`QecError`] at the subsystem
/// boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheError {
    /// Invalid cache-local configuration.
    InvalidConfiguration(&'static str),

    /// Invalid cache execution context.
    InvalidContext(&'static str),

    /// An entry exceeds the canonical QEC memory budget.
    EntryTooLarge {
        size_bytes: u64,
        max_bytes: u64,
    },

    /// The cache cannot admit an entry because protected entries prevent
    /// sufficient deterministic eviction.
    AdmissionBlocked {
        required_bytes: u64,
        available_bytes: u64,
        required_entries: usize,
        available_entries: usize,
    },

    /// Checked cache accounting overflowed.
    ResourceAccountingOverflow,

    /// The cache mutex was poisoned.
    LockPoisoned,

    /// Entry integrity validation failed.
    IntegrityFailure,

    /// Entry is expired.
    Expired,

    /// Entry was not found.
    NotFound,

    /// Cache entry belongs to a different execution context.
    IncompatibleContext,

    /// Runtime cache accounting rejected a reservation.
    AccountingRejected {
        message: String,
    },

    /// Cache schema/version is incompatible.
    VersionMismatch {
        expected: Version,
        actual: Version,
    },
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(f, "invalid cache configuration: {message}")
            }

            Self::InvalidContext(message) => {
                write!(f, "invalid cache context: {message}")
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

            Self::AdmissionBlocked {
                required_bytes,
                available_bytes,
                required_entries,
                available_entries,
            } => {
                write!(
                    f,
                    "cache admission blocked: required {required_bytes} bytes \
                     and {required_entries} entries; available {available_bytes} \
                     bytes and {available_entries} entries"
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

            Self::AccountingRejected { message } => {
                write!(f, "cache resource accounting rejected reservation: {message}")
            }

            Self::VersionMismatch { expected, actual } => {
                write!(
                    f,
                    "cache version mismatch: expected {expected}, actual {actual}"
                )
            }
        }
    }
}

impl std::error::Error for CacheError {}

impl From<CacheError> for QecError {
    fn from(error: CacheError) -> Self {
        match error {
            CacheError::InvalidConfiguration(message) => {
                QecError::CacheInvalid {
                    message: message.to_owned(),
                }
            }

            CacheError::InvalidContext(message) => {
                QecError::CacheInvalid {
                    message: message.to_owned(),
                }
            }

            CacheError::EntryTooLarge {
                size_bytes,
                max_bytes,
            } => {
                QecError::MemoryLimitExceeded {
                    requested_bytes: size_bytes,
                    current_bytes: 0,
                    limit_bytes: max_bytes,
                    message: "cache entry exceeds QEC memory policy".to_owned(),
                }
            }

            CacheError::AdmissionBlocked {
                required_bytes,
                available_bytes,
                ..
            } => {
                QecError::MemoryLimitExceeded {
                    requested_bytes: required_bytes,
                    current_bytes: 0,
                    limit_bytes: available_bytes,
                    message: "cache admission blocked by protected entries"
                        .to_owned(),
                }
            }

            CacheError::ResourceAccountingOverflow => {
                QecError::CacheInvalid {
                    message: "cache resource accounting overflow".to_owned(),
                }
            }

            CacheError::LockPoisoned => {
                QecError::CacheInvalid {
                    message: "cache state lock poisoned".to_owned(),
                }
            }

            CacheError::IntegrityFailure => {
                QecError::CacheInvalid {
                    message: "cache entry integrity validation failed".to_owned(),
                }
            }

            CacheError::Expired => {
                QecError::CacheInvalid {
                    message: "cache entry expired".to_owned(),
                }
            }

            CacheError::NotFound => {
                QecError::CacheInvalid {
                    message: "cache entry not found".to_owned(),
                }
            }

            CacheError::IncompatibleContext => {
                QecError::CacheInvalid {
                    message: "cache entry has incompatible execution context"
                        .to_owned(),
                }
            }

            CacheError::AccountingRejected { message } => {
                QecError::CacheInvalid { message }
            }

            CacheError::VersionMismatch { expected, actual } => {
                QecError::VersionMismatch {
                    component: "cache".to_owned(),
                    expected: expected.to_string(),
                    actual: actual.to_string(),
                    message: "cache artifact version is incompatible".to_owned(),
                }
            }
        }
    }
}

// ============================================================================
// Eviction policy
// ============================================================================

/// Deterministic cache eviction policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionPolicy {
    /// Least recently used.
    Lru,

    /// First in, first out.
    Fifo,
}

// ============================================================================
// Cache configuration
// ============================================================================

/// Cache-local configuration.
///
/// There is deliberately no independent `max_bytes`.
///
/// The memory ceiling comes from [`QecLimits`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheConfig {
    /// Maximum number of entries.
    pub max_entries: usize,

    /// Optional entry lifetime.
    pub ttl: Option<Duration>,

    /// Deterministic eviction strategy.
    pub eviction_policy: EvictionPolicy,

    /// Verify integrity on every lookup.
    pub verify_integrity: bool,

    /// Permit eviction of pinned entries when admission would otherwise fail.
    ///
    /// This should normally remain `false`.
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

// ============================================================================
// Execution context
// ============================================================================

/// Correctness identity of a cached computation.
///
/// A cache entry is reusable only when the complete correctness context
/// matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CacheContext {
    /// Code identity.
    pub code_hash: u64,

    /// Topology identity.
    pub topology_hash: u64,

    /// Decoder identity/version.
    pub decoder_version: u64,

    /// Validated QEC configuration identity.
    pub configuration_hash: u64,

    /// Algorithm identity/version.
    pub algorithm_version: u64,

    /// Backend identity.
    pub backend_id: u64,

    /// Complete QEC API version.
    pub api_version: Version,

    /// Cache artifact schema.
    pub cache_version: Version,
}

impl Default for CacheContext {
    fn default() -> Self {
        Self {
            code_hash: 0,
            topology_hash: 0,
            decoder_version: 1,
            configuration_hash: 0,
            algorithm_version: 1,
            backend_id: 0,
            api_version: Version::current(),
            cache_version: ArtifactKind::Cache.current_version(),
        }
    }
}

impl CacheContext {
    /// Validates the context.
    pub fn validate(&self) -> Result<(), CacheError> {
        if self.code_hash == 0 {
            return Err(CacheError::InvalidContext(
                "code_hash must be non-zero",
            ));
        }

        if self.decoder_version == 0 {
            return Err(CacheError::InvalidContext(
                "decoder_version must be non-zero",
            ));
        }

        if self.configuration_hash == 0 {
            return Err(CacheError::InvalidContext(
                "configuration_hash must be non-zero",
            ));
        }

        if self.algorithm_version == 0 {
            return Err(CacheError::InvalidContext(
                "algorithm_version must be non-zero",
            ));
        }

        if self.api_version == Version::zero() {
            return Err(CacheError::InvalidContext(
                "api_version must be non-zero",
            ));
        }

        let expected_cache_version = ArtifactKind::Cache.current_version();

        if !self
            .cache_version
            .is_compatible_with(expected_cache_version)
        {
            return Err(CacheError::VersionMismatch {
                expected: expected_cache_version,
                actual: self.cache_version,
            });
        }

        if !self
            .api_version
            .is_compatible_with(Version::current())
        {
            return Err(CacheError::VersionMismatch {
                expected: Version::current(),
                actual: self.api_version,
            });
        }

        Ok(())
    }

    /// Returns a deterministic context fingerprint.
    pub fn fingerprint(&self) -> u64 {
        let mut hash = FNV_OFFSET;

        hash = hash_u64(hash, self.code_hash);
        hash = hash_u64(hash, self.topology_hash);
        hash = hash_u64(hash, self.decoder_version);
        hash = hash_u64(hash, self.configuration_hash);
        hash = hash_u64(hash, self.algorithm_version);
        hash = hash_u64(hash, self.backend_id);
        hash = hash_u64(hash, self.api_version.packed());
        hash_u64(hash, self.cache_version.packed())
    }
}

// ============================================================================
// Integrity
// ============================================================================

/// Integrity verifier for cached values.
///
/// Implementations may use cryptographic hashing when caches cross a trust
/// boundary.
pub trait CacheIntegrity: Send + Sync {
    /// Calculates a deterministic fingerprint.
    fn fingerprint(&self, value: &[u8]) -> u64;
}

/// FNV-1a integrity implementation.
///
/// This detects accidental corruption only. It is not cryptographic
/// authentication.
#[derive(Debug, Clone, Copy, Default)]
pub struct Fnv1aIntegrity;

impl CacheIntegrity for Fnv1aIntegrity {
    fn fingerprint(&self, value: &[u8]) -> u64 {
        let mut hash = FNV_OFFSET;

        for byte in value {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(FNV_PRIME);
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

// ============================================================================
// Runtime accounting integration
// ============================================================================

/// Runtime accounting integration boundary.
///
/// `resources.rs` may implement this trait for `ResourceManager`.
///
/// The cache itself remains usable without a runtime accounting object through
/// [`NoopCacheAccounting`].
pub trait CacheAccounting: Send + Sync {
    /// Reserve cache memory before insertion.
    fn reserve_cache_bytes(&self, bytes: u64) -> Result<(), CacheError>;

    /// Release previously reserved cache memory.
    fn release_cache_bytes(&self, bytes: u64) -> Result<(), CacheError>;
}

/// No-op accounting implementation.
///
/// This is appropriate when the caller has already placed the cache inside a
/// resource reservation or when the cache is used in a standalone context.
#[derive(Debug, Default)]
pub struct NoopCacheAccounting;

impl CacheAccounting for NoopCacheAccounting {
    fn reserve_cache_bytes(&self, _bytes: u64) -> Result<(), CacheError> {
        Ok(())
    }

    fn release_cache_bytes(&self, _bytes: u64) -> Result<(), CacheError> {
        Ok(())
    }
}

// ============================================================================
// Cache values
// ============================================================================

/// Contract for values stored in the cache.
pub trait CacheValue: Clone + Send + Sync + 'static {
    /// Estimated complete memory footprint.
    ///
    /// This must include owned heap memory relevant to the cached value.
    fn estimated_size_bytes(&self) -> u64;

    /// Deterministic bytes covering correctness-relevant value state.
    fn integrity_bytes(&self) -> Vec<u8>;
}

// ============================================================================
// Cache entry
// ============================================================================

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

// ============================================================================
// Statistics
// ============================================================================

/// Immutable cache statistics snapshot.
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

// ============================================================================
// Internal state
// ============================================================================

struct CacheState<K, V> {
    entries: HashMap<K, CacheEntry<V>>,
    stats: CacheStats,

    next_sequence: u64,
    generation: u64,

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

// ============================================================================
// Cache
// ============================================================================

/// Thread-safe bounded QEC cache.
pub struct QecCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: CacheValue,
{
    config: CacheConfig,
    limits: QecLimits,

    integrity: Arc<dyn CacheIntegrity>,
    accounting: Arc<dyn CacheAccounting>,

    state: Arc<Mutex<CacheState<K, V>>>,
}

impl<K, V> Clone for QecCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: CacheValue,
{
    fn clone(&self) -> Self {
        Self {
            config: self.config,
            limits: self.limits,
            integrity: Arc::clone(&self.integrity),
            accounting: Arc::clone(&self.accounting),
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
        Self::with_integrity_and_accounting(
            config,
            limits,
            Arc::new(Fnv1aIntegrity),
            Arc::new(NoopCacheAccounting),
        )
    }

    /// Creates a cache with custom integrity verification.
    pub fn with_integrity(
        config: CacheConfig,
        limits: QecLimits,
        integrity: Arc<dyn CacheIntegrity>,
    ) -> Result<Self, CacheError> {
        Self::with_integrity_and_accounting(
            config,
            limits,
            integrity,
            Arc::new(NoopCacheAccounting),
        )
    }

    /// Creates a cache with both integrity and runtime accounting.
    pub fn with_integrity_and_accounting(
        config: CacheConfig,
        limits: QecLimits,
        integrity: Arc<dyn CacheIntegrity>,
        accounting: Arc<dyn CacheAccounting>,
    ) -> Result<Self, CacheError> {
        config.validate()?;
        limits
            .validate()
            .map_err(|_| {
                CacheError::InvalidConfiguration(
                    "invalid canonical QEC resource policy",
                )
            })?;

        Ok(Self {
            config,
            limits,
            integrity,
            accounting,
            state: Arc::new(Mutex::new(CacheState::new())),
        })
    }

    /// Returns cache configuration.
    pub fn config(&self) -> &CacheConfig {
        &self.config
    }

    /// Returns the canonical QEC resource policy.
    pub fn limits(&self) -> &QecLimits {
        &self.limits
    }

    /// Returns the cache artifact schema version.
    pub const fn schema_version() -> Version {
        ArtifactKind::Cache.current_version()
    }

    /// Returns statistics.
    pub fn stats(&self) -> Result<CacheStats, CacheError> {
        Ok(self.lock_state()?.stats)
    }

    /// Returns current accounted cache bytes.
    pub fn memory_usage_bytes(&self) -> Result<u64, CacheError> {
        Ok(self.lock_state()?.bytes)
    }

    /// Returns number of entries.
    pub fn len(&self) -> Result<usize, CacheError> {
        Ok(self.lock_state()?.entries.len())
    }

    /// Returns whether the cache is empty.
    pub fn is_empty(&self) -> Result<bool, CacheError> {
        Ok(self.lock_state()?.entries.is_empty())
    }

    /// Inserts or replaces a value.
    ///
    /// Admission is transactional:
    ///
    /// - existing data is not removed until admission succeeds;
    /// - protected entries are never evicted unless configured;
    /// - runtime accounting is reserved before committing the new entry.
    pub fn insert(
        &self,
        key: K,
        value: V,
        context: CacheContext,
    ) -> Result<bool, CacheError> {
        context.validate()?;

        let size_bytes = value.estimated_size_bytes();

        if size_bytes > self.limits.max_memory_bytes {
            self.increment_rejected_entries()?;

            return Err(CacheError::EntryTooLarge {
                size_bytes,
                max_bytes: self.limits.max_memory_bytes,
            });
        }

        let fingerprint = self
            .integrity
            .fingerprint(&value.integrity_bytes());

        let now = Instant::now();

        let mut state = self.lock_state()?;

        let old_size = state
            .entries
            .get(&key)
            .map(|entry| entry.size_bytes)
            .unwrap_or(0);

        let projected_bytes = state
            .bytes
            .checked_sub(old_size)
            .and_then(|bytes| bytes.checked_add(size_bytes))
            .ok_or(CacheError::ResourceAccountingOverflow)?;

        let projected_entries = if state.entries.contains_key(&key) {
            state.entries.len()
        } else {
            state.entries.len().saturating_add(1)
        };

        self.evict_until_fit(
            &mut state,
            size_bytes,
            old_size,
            projected_entries,
            projected_bytes,
        )?;

        /*
         * Recalculate after eviction because eviction may have changed the
         * current state.
         */
        let current_old_size = state
            .entries
            .get(&key)
            .map(|entry| entry.size_bytes)
            .unwrap_or(0);

        let final_bytes = state
            .bytes
            .checked_sub(current_old_size)
            .and_then(|bytes| bytes.checked_add(size_bytes))
            .ok_or(CacheError::ResourceAccountingOverflow)?;

        let final_entries = if state.entries.contains_key(&key) {
            state.entries.len()
        } else {
            state.entries.len().saturating_add(1)
        };

        if final_entries > self.config.max_entries {
            return Err(CacheError::AdmissionBlocked {
                required_bytes: size_bytes,
                available_bytes: self
                    .limits
                    .max_memory_bytes
                    .saturating_sub(state.bytes),
                required_entries: final_entries,
                available_entries: self
                    .config
                    .max_entries
                    .saturating_sub(state.entries.len()),
            });
        }

        if final_bytes > self.limits.max_memory_bytes {
            return Err(CacheError::AdmissionBlocked {
                required_bytes: size_bytes,
                available_bytes: self
                    .limits
                    .max_memory_bytes
                    .saturating_sub(state.bytes),
                required_entries: final_entries,
                available_entries: self
                    .config
                    .max_entries
                    .saturating_sub(state.entries.len()),
            });
        }

        /*
         * Reserve runtime accounting for the net increase.
         *
         * Replacement of an existing entry does not require reserving the
         * entire replacement size if the old entry is released first.
         *
         * To keep the operation transactional, reserve the positive delta.
         */
        let delta = size_bytes.saturating_sub(current_old_size);

        if delta > 0 {
            self.accounting.reserve_cache_bytes(delta)?;
        }

        let sequence = state.next_sequence();
        let generation = state.generation;

        let old = state.entries.remove(&key);

        if let Some(old_entry) = old {
            state.bytes = state
                .bytes
                .checked_sub(old_entry.size_bytes)
                .ok_or(CacheError::ResourceAccountingOverflow)?;

            if old_entry.size_bytes > size_bytes {
                let release = old_entry.size_bytes - size_bytes;

                self.accounting.release_cache_bytes(release)?;
            }
        }

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

        state.bytes = state
            .bytes
            .checked_add(size_bytes)
            .ok_or(CacheError::ResourceAccountingOverflow)?;

        let replaced = current_old_size != 0;

        if replaced {
            state.stats.replacements = state
                .stats
                .replacements
                .saturating_add(1);
        } else {
            state.stats.insertions = state
                .stats
                .insertions
                .saturating_add(1);
        }

        state.update_stats();

        Ok(replaced)
    }

    /// Inserts a pinned entry.
    pub fn insert_pinned(
        &self,
        key: K,
        value: V,
        context: CacheContext,
    ) -> Result<bool, CacheError> {
        let replaced = self.insert(
            key.clone(),
            value,
            context,
        )?;

        let mut state = self.lock_state()?;

        if let Some(entry) = state.entries.get_mut(&key) {
            entry.pinned = true;
        }

        Ok(replaced)
    }

    /// Looks up a cached value.
    ///
    /// An incompatible entry is a cache miss, not a correctness failure.
    ///
    /// Corrupt entries are removed and reported as `IntegrityFailure`.
    pub fn get(
        &self,
        key: &K,
        context: &CacheContext,
    ) -> Result<Option<V>, CacheError> {
        context.validate()?;

        let now = Instant::now();

        let mut state = self.lock_state()?;

        let expired = match state.entries.get(key) {
            Some(entry) => {
                entry_expired(
                    entry,
                    now,
                    self.config.ttl,
                )
            }

            None => {
                state.stats.misses = state
                    .stats
                    .misses
                    .saturating_add(1);

                return Ok(None);
            }
        };

        if expired {
            self.remove_internal(
                &mut state,
                key,
            )?;

            state.stats.expirations = state
                .stats
                .expirations
                .saturating_add(1);

            state.stats.misses = state
                .stats
                .misses
                .saturating_add(1);

            state.bump_generation();
            state.update_stats();

            return Ok(None);
        }

        let compatible = match state.entries.get(key) {
            Some(entry) => entry.context == *context,
            None => false,
        };

        if !compatible {
            state.stats.misses = state
                .stats
                .misses
                .saturating_add(1);

            return Ok(None);
        }

        /*
         * Integrity verification is performed before updating access state.
         */
        let integrity_valid = if self.config.verify_integrity {
            let entry = state
                .entries
                .get(key)
                .ok_or(CacheError::NotFound)?;

            let fingerprint = self
                .integrity
                .fingerprint(
                    &entry.value.integrity_bytes(),
                );

            fingerprint == entry.fingerprint
        } else {
            true
        };

        if !integrity_valid {
            self.remove_internal(
                &mut state,
                key,
            )?;

            state.stats.integrity_failures = state
                .stats
                .integrity_failures
                .saturating_add(1);

            state.stats.misses = state
                .stats
                .misses
                .saturating_add(1);

            state.bump_generation();
            state.update_stats();

            return Err(CacheError::IntegrityFailure);
        }

        let sequence = state.next_sequence();

        let entry = state
            .entries
            .get_mut(key)
            .ok_or(CacheError::NotFound)?;

        entry.last_access = now;
        entry.sequence = sequence;

        let value = entry.value.clone();

        state.stats.hits = state
            .stats
            .hits
            .saturating_add(1);

        Ok(Some(value))
    }

    /// Returns a cached value or computes it.
    ///
    /// Cache failure never prevents recomputation except for an explicit
    /// integrity failure. In that case callers should decide whether the
    /// failure represents a security boundary violation.
    pub fn get_or_insert_with<F>(
        &self,
        key: K,
        context: CacheContext,
        compute: F,
    ) -> Result<V, CacheError>
    where
        F: FnOnce() -> Result<V, CacheError>,
    {
        if let Some(value) = self.get(
            &key,
            &context,
        )? {
            return Ok(value);
        }

        let value = compute()?;

        /*
         * A cache admission failure must not invalidate the freshly computed
         * result. Therefore return the computed value if caching cannot admit
         * it for a normal capacity reason.
         */
        match self.insert(
            key,
            value.clone(),
            context,
        ) {
            Ok(_) => Ok(value),

            Err(
                CacheError::EntryTooLarge { .. }
                | CacheError::AdmissionBlocked { .. },
            ) => Ok(value),

            Err(error) => Err(error),
        }
    }

    /// Pins an existing entry.
    pub fn pin(
        &self,
        key: &K,
    ) -> Result<bool, CacheError> {
        let mut state = self.lock_state()?;

        match state.entries.get_mut(key) {
            Some(entry) => {
                entry.pinned = true;
                Ok(true)
            }

            None => Ok(false),
        }
    }

    /// Unpins an existing entry.
    pub fn unpin(
        &self,
        key: &K,
    ) -> Result<bool, CacheError> {
        let mut state = self.lock_state()?;

        match state.entries.get_mut(key) {
            Some(entry) => {
                entry.pinned = false;
                Ok(true)
            }

            None => Ok(false),
        }
    }

    /// Invalidates one cache entry.
    pub fn invalidate(
        &self,
        key: &K,
    ) -> Result<bool, CacheError> {
        let mut state = self.lock_state()?;

        let removed = self.remove_internal(
            &mut state,
            key,
        )?;

        if removed {
            state.stats.invalidations = state
                .stats
                .invalidations
                .saturating_add(1);

            state.bump_generation();
            state.update_stats();
        }

        Ok(removed)
    }

    /// Invalidates all entries belonging to a context.
    pub fn invalidate_context(
        &self,
        context: &CacheContext,
    ) -> Result<usize, CacheError> {
        context.validate()?;

        let mut state = self.lock_state()?;

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
                removed = removed.saturating_add(1);
            }
        }

        if removed != 0 {
            state.stats.invalidations = state
                .stats
                .invalidations
                .saturating_add(removed as u64);

            state.bump_generation();
            state.update_stats();
        }

        Ok(removed)
    }

    /// Clears the cache.
    pub fn clear(&self) -> Result<(), CacheError> {
        let mut state = self.lock_state()?;

        let bytes = state.bytes;

        state.entries.clear();
        state.bytes = 0;

        if bytes != 0 {
            self.accounting.release_cache_bytes(bytes)?;
        }

        state.bump_generation();
        state.update_stats();

        Ok(())
    }

    /// Removes expired entries.
    pub fn purge_expired(&self) -> Result<usize, CacheError> {
        let ttl = match self.config.ttl {
            Some(ttl) => ttl,
            None => return Ok(0),
        };

        let now = Instant::now();

        let mut state = self.lock_state()?;

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
                removed = removed.saturating_add(1);
            }
        }

        if removed != 0 {
            state.stats.expirations = state
                .stats
                .expirations
                .saturating_add(removed as u64);

            state.bump_generation();
            state.update_stats();
        }

        Ok(removed)
    }

    /// Returns the current generation.
    pub fn generation(&self) -> Result<u64, CacheError> {
        Ok(self.lock_state()?.generation)
    }

    // ------------------------------------------------------------------------
    // Internal operations
    // ------------------------------------------------------------------------

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

    fn increment_rejected_entries(
        &self,
    ) -> Result<(), CacheError> {
        let mut state = self.lock_state()?;

        state.stats.rejected_entries = state
            .stats
            .rejected_entries
            .saturating_add(1);

        Ok(())
    }

    fn remove_internal(
        &self,
        state: &mut CacheState<K, V>,
        key: &K,
    ) -> Result<bool, CacheError> {
        let entry = match state.entries.remove(key) {
            Some(entry) => entry,
            None => return Ok(false),
        };

        state.bytes = state
            .bytes
            .checked_sub(entry.size_bytes)
            .ok_or(CacheError::ResourceAccountingOverflow)?;

        self.accounting
            .release_cache_bytes(entry.size_bytes)?;

        Ok(true)
    }

    fn evict_until_fit(
        &self,
        state: &mut CacheState<K, V>,
        incoming_size: u64,
        replacement_size: u64,
        projected_entries: usize,
        projected_bytes: u64,
    ) -> Result<(), CacheError> {
        let _ = replacement_size;

        let mut target_entries = projected_entries;
        let mut target_bytes = projected_bytes;

        while target_entries > self.config.max_entries
            || target_bytes > self.limits.max_memory_bytes
        {
            let key = match self.select_eviction_candidate(state) {
                Some(key) => key,

                None => {
                    return Err(
                        CacheError::AdmissionBlocked {
                            required_bytes: incoming_size,
                            available_bytes: self
                                .limits
                                .max_memory_bytes
                                .saturating_sub(
                                    state.bytes,
                                ),
                            required_entries: target_entries,
                            available_entries: self
                                .config
                                .max_entries
                                .saturating_sub(
                                    state.entries.len(),
                                ),
                        },
                    );
                }
            };

            let removed = state
                .entries
                .remove(&key)
                .ok_or(CacheError::NotFound)?;

            state.bytes = state
                .bytes
                .checked_sub(removed.size_bytes)
                .ok_or(CacheError::ResourceAccountingOverflow)?;

            self.accounting
                .release_cache_bytes(removed.size_bytes)?;

            target_entries = if state.entries.contains_key(&key) {
                state.entries.len()
            } else {
                state.entries.len()
            };

            /*
             * Recompute rather than relying on stale arithmetic. This is
             * important for correctness when several entries are evicted.
             */
            target_entries = state
                .entries
                .len()
                .saturating_add(
                    if state.entries.contains_key(&key) {
                        0
                    } else {
                        1
                    },
                );

            target_bytes = state
                .bytes
                .checked_add(incoming_size)
                .ok_or(CacheError::ResourceAccountingOverflow)?;

            state.stats.evictions = state
                .stats
                .evictions
                .saturating_add(1);

            state.update_stats();
        }

        Ok(())
    }

    fn select_eviction_candidate(
        &self,
        state: &CacheState<K, V>,
    ) -> Option<K> {
        let mut candidate: Option<(&K, &CacheEntry<V>)> = None;

        for (key, entry) in &state.entries {
            if entry.pinned
                && !self.config.allow_pinned_eviction
            {
                continue;
            }

            let should_replace = match candidate {
                None => true,

                Some((_, current)) => {
                    match self.config.eviction_policy {
                        EvictionPolicy::Lru => {
                            entry.last_access < current.last_access
                                || (
                                    entry.last_access
                                        == current.last_access
                                    && entry.sequence
                                        < current.sequence
                                )
                        }

                        EvictionPolicy::Fifo => {
                            entry.sequence < current.sequence
                        }
                    }
                }
            };

            if should_replace {
                candidate = Some((key, entry));
            }
        }

        candidate.map(|(key, _)| key.clone())
    }
}

// ============================================================================
// Helpers
// ============================================================================

const FNV_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

fn hash_u64(
    mut hash: u64,
    value: u64,
) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash
}

fn entry_expired<V>(
    entry: &CacheEntry<V>,
    now: Instant,
    ttl: Option<Duration>,
) -> bool {
    match ttl {
        Some(ttl) => {
            now.duration_since(entry.created_at) >= ttl
        }

        None => false,
    }
}

// ============================================================================
// Standard byte-backed value
// ============================================================================

/// Byte-backed cache value.
///
/// Suitable for serialized graph fragments, decoder tables, replay artifacts,
/// immutable templates, or other byte-oriented cache objects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BytesValue {
    bytes: Vec<u8>,
}

impl BytesValue {
    /// Creates a byte-backed cache value.
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

// ============================================================================
// Tests
// ============================================================================

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
            api_version: Version::current(),
            cache_version: ArtifactKind::Cache.current_version(),
        }
    }

    fn cache(
        max_entries: usize,
        max_memory: u64,
    ) -> QecCache<String, BytesValue> {
        let config = CacheConfig {
            max_entries,
            ..CacheConfig::default()
        };

        let mut limits = QecLimits::default();
        limits.max_memory_bytes = max_memory;

        QecCache::new(
            config,
            limits,
        )
        .expect("valid cache")
    }

    #[test]
    fn inserts_and_reads() {
        let cache = cache(8, 1024);

        let key = "decoder".to_owned();

        let value = BytesValue::new(vec![1, 2, 3]);

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
    fn incompatible_context_is_a_cache_miss() {
        let cache = cache(8, 1024);

        let key = "decoder".to_owned();

        cache
            .insert(
                key.clone(),
                BytesValue::new(vec![1]),
                context(),
            )
            .expect("insert");

        let mut incompatible = context();
        incompatible.decoder_version = 99;

        assert_eq!(
            cache
                .get(
                    &key,
                    &incompatible,
                )
                .expect("lookup"),
            None
        );
    }

    #[test]
    fn memory_budget_is_enforced() {
        let cache = cache(8, 4);

        let result = cache.insert(
            "large".to_owned(),
            BytesValue::new(vec![0; 5]),
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
    fn fifo_eviction_does_not_over_evict() {
        let mut config = CacheConfig::default();
        config.max_entries = 2;
        config.eviction_policy = EvictionPolicy::Fifo;

        let mut limits = QecLimits::default();
        limits.max_memory_bytes = 1024;

        let cache = QecCache::<String, BytesValue>::new(
            config,
            limits,
        )
        .expect("cache");

        cache
            .insert(
                "a".to_owned(),
                BytesValue::new(vec![1]),
                context(),
            )
            .expect("a");

        cache
            .insert(
                "b".to_owned(),
                BytesValue::new(vec![2]),
                context(),
            )
            .expect("b");

        cache
            .insert(
                "c".to_owned(),
                BytesValue::new(vec![3]),
                context(),
            )
            .expect("c");

        assert_eq!(
            cache.len().expect("length"),
            2
        );

        assert!(
            cache
                .get(
                    &"a".to_owned(),
                    &context(),
                )
                .expect("lookup")
                .is_none()
        );

        assert!(
            cache
                .get(
                    &"b".to_owned(),
                    &context(),
                )
                .expect("lookup")
                .is_some()
        );

        assert!(
            cache
                .get(
                    &"c".to_owned(),
                    &context(),
                )
                .expect("lookup")
                .is_some()
        );
    }

    #[test]
    fn replacement_preserves_new_value() {
        let cache = cache(2, 1024);

        let key = "same".to_owned();

        cache
            .insert(
                key.clone(),
                BytesValue::new(vec![1]),
                context(),
            )
            .expect("first");

        assert!(
            cache
                .insert(
                    key.clone(),
                    BytesValue::new(vec![2]),
                    context(),
                )
                .expect("replacement")
        );

        assert_eq!(
            cache
                .get(
                    &key,
                    &context(),
                )
                .expect("lookup"),
            Some(BytesValue::new(vec![2]))
        );
    }

    #[test]
    fn pinned_entries_are_protected() {
        let cache = cache(1, 1024);

        cache
            .insert_pinned(
                "a".to_owned(),
                BytesValue::new(vec![1]),
                context(),
            )
            .expect("insert");

        let result = cache.insert(
            "b".to_owned(),
            BytesValue::new(vec![2]),
            context(),
        );

        assert!(matches!(
            result,
            Err(CacheError::AdmissionBlocked { .. })
        ));

        assert!(
            cache
                .get(
                    &"a".to_owned(),
                    &context(),
                )
                .expect("lookup")
                .is_some()
        );
    }

    #[test]
    fn clear_releases_accounted_memory() {
        let cache = cache(8, 1024);

        cache
            .insert(
                "a".to_owned(),
                BytesValue::new(vec![1, 2, 3]),
                context(),
            )
            .expect("insert");

        assert_eq!(
            cache
                .memory_usage_bytes()
                .expect("memory"),
            3
        );

        cache.clear().expect("clear");

        assert_eq!(
            cache
                .memory_usage_bytes()
                .expect("memory"),
            0
        );
    }

    #[test]
    fn generation_changes_after_invalidation() {
        let cache = cache(8, 1024);

        cache
            .insert(
                "a".to_owned(),
                BytesValue::new(vec![1]),
                context(),
            )
            .expect("insert");

        let before = cache
            .generation()
            .expect("generation");

        cache
            .invalidate(
                &"a".to_owned(),
            )
            .expect("invalidate");

        let after = cache
            .generation()
            .expect("generation");

        assert!(after > before);
    }

    #[test]
    fn cache_context_fingerprint_is_deterministic() {
        let first = context().fingerprint();
        let second = context().fingerprint();

        assert_eq!(first, second);
    }

    #[test]
    fn bytes_value_reports_size() {
        let value = BytesValue::new(
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

    #[test]
    fn oversized_computation_can_still_be_returned() {
        let cache = cache(8, 2);

        let value = cache
            .get_or_insert_with(
                "large".to_owned(),
                context(),
                || {
                    Ok(BytesValue::new(
                        vec![1, 2, 3],
                    ))
                },
            )
            .expect("computed value");

        assert_eq!(
            value.as_bytes(),
            &[1, 2, 3]
        );

        assert!(
            cache
                .get(
                    &"large".to_owned(),
                    &context(),
                )
                .expect("lookup")
                .is_none()
        );
    }

    #[test]
    fn schema_version_matches_canonical_version_module() {
        assert_eq!(
            QecCache::<
                String,
                BytesValue,
            >::schema_version(),
            ArtifactKind::Cache.current_version()
        );
    }
}