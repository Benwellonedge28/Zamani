//! Zamani Quantum Memory — Cache
//!
//! Production-grade, representation-independent caching infrastructure for
//! `quantum::memory`.
//!
//! # Purpose
//!
//! This module provides bounded, deterministic, hardware-neutral caching for
//! reusable quantum-memory objects and derived memory data.
//!
//! It is intentionally independent of:
//!
//! - state-vector implementation;
//! - density-matrix implementation;
//! - stabilizer implementation;
//! - sparse-state implementation;
//! - tensor-network implementation;
//! - CPU implementation;
//! - SIMD implementation;
//! - GPU implementation;
//! - distributed-memory implementation;
//! - QPU/vendor implementation;
//! - routing;
//! - scheduling;
//! - benchmarking;
//! - frontend parsing.
//!
//! Those modules may depend on this cache. This cache must not depend on them.
//!
//! # Architectural position
//!
//! ```text
//!                     quantum::ir
//!                         │
//!                         ▼
//!                 execution / memory
//!                         │
//!              ┌──────────┴──────────┐
//!              │      cache.rs       │
//!              └──────────┬──────────┘
//!                         │
//!          ┌──────────────┼──────────────┐
//!          ▼              ▼              ▼
//!     StateVector     TensorNetwork    BackendState
//!          │              │              │
//!          ▼              ▼              ▼
//!         CPU            GPU        distributed/QPU
//! ```
//!
//! # Design principles
//!
//! 1. Cache correctness must never depend on cache hits.
//! 2. A cache hit must return exactly the value associated with the complete
//!    cache key.
//! 3. Hardware/backend identity must be part of the key when a value is
//!    hardware-context dependent.
//! 4. Representation, precision, layout and semantic generation must be part
//!    of the key when they affect correctness.
//! 5. Cache capacity is bounded.
//! 6. Byte capacity is bounded independently from entry capacity.
//! 7. Expiration is lazy and explicit cleanup is also available.
//! 8. Invalidation is deterministic.
//! 9. No global mutable state is used.
//! 10. No `unsafe` code is used.
//! 11. No threads are spawned.
//! 12. No filesystem/network access occurs.
//! 13. The cache never owns quantum semantics.
//! 14. The cache never silently converts representations.
//! 15. The cache never silently crosses backend/QPU boundaries.
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
//! No external crates are required.
//!
//! # Integration contract
//!
//! Later memory modules should use the types in this file rather than creating
//! their own cache keys, cache policies, cache statistics, or cache lifecycle
//! mechanisms.
//!
//! In particular:
//!
//! - `state.rs` may use `MemoryCache` for derived-state data;
//! - `state_vector.rs` may cache reusable transformed/indexed data;
//! - `density_matrix.rs` may cache derived tensor/index structures;
//! - `tensor_network.rs` may cache contraction/planning results;
//! - `gpu.rs` may cache host/device-derived objects;
//! - `distributed.rs` may cache partition metadata;
//! - `migration.rs` may cache migration plans;
//! - `snapshot.rs` may use cache fingerprints;
//! - `diagnostics.rs` and `telemetry.rs` may consume `CacheStatistics`.
//!
//! The cache itself remains independent of those modules.
//!
//! # Important semantic rule
//!
//! This module caches values. It does not decide whether a value is valid.
//!
//! The caller is responsible for constructing a `CacheKey` containing every
//! semantic input that can affect the cached value.
//!
//! If a quantum operation, calibration, layout, backend configuration, scalar
//! precision, representation, or algorithmic policy changes the result, the
//! corresponding identity must change in the key or the relevant generation
//! must be invalidated.
//!
//! This is deliberately conservative: a false cache miss is a performance
//! problem; a false cache hit can be a correctness problem.

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// =============================================================================
// Constants
// =============================================================================

/// Cache implementation version.
pub const CACHE_VERSION: &str = "1";

/// Default maximum number of entries.
pub const DEFAULT_MAX_ENTRIES: usize = 4096;

/// Default maximum retained bytes.
///
/// This is a policy limit, not a promise that the process has this much memory.
pub const DEFAULT_MAX_BYTES: u64 = 256 * 1024 * 1024;

/// Maximum accepted entry count.
///
/// This prevents accidental configuration values from creating unreasonable
/// metadata overhead.
pub const MAX_MAX_ENTRIES: usize = 16_777_216;

/// Maximum accepted byte capacity.
pub const MAX_MAX_BYTES: u64 = 1 << 50;

/// Default generation.
pub const INITIAL_GENERATION: u64 = 0;

// =============================================================================
// Result and errors
// =============================================================================

/// Result type used by this module.
pub type CacheResult<T> = Result<T, CacheError>;

/// Errors produced by the memory cache.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CacheError {
    /// Cache configuration is invalid.
    InvalidConfiguration(String),

    /// Entry size exceeds the cache's maximum single-entry size.
    EntryTooLarge {
        size_bytes: u64,
        maximum_bytes: u64,
    },

    /// Cache accounting would overflow.
    AccountingOverflow,

    /// An operation requires an entry that is not present.
    EntryNotFound,

    /// A requested operation cannot be performed because of an invalid key.
    InvalidKey(String),

    /// The supplied namespace is invalid.
    InvalidNamespace(String),
}

impl fmt::Display for CacheError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(formatter, "invalid cache configuration: {message}")
            }
            Self::EntryTooLarge {
                size_bytes,
                maximum_bytes,
            } => {
                write!(
                    formatter,
                    "cache entry size {size_bytes} bytes exceeds maximum \
                     single-entry size {maximum_bytes} bytes"
                )
            }
            Self::AccountingOverflow => {
                write!(formatter, "cache accounting overflow")
            }
            Self::EntryNotFound => {
                write!(formatter, "cache entry not found")
            }
            Self::InvalidKey(message) => {
                write!(formatter, "invalid cache key: {message}")
            }
            Self::InvalidNamespace(message) => {
                write!(formatter, "invalid cache namespace: {message}")
            }
        }
    }
}

impl std::error::Error for CacheError {}

// =============================================================================
// Stable identifiers
// =============================================================================

/// Stable cache namespace.
///
/// Namespaces allow independent invalidation domains without requiring the
/// cache to understand the semantics of the cached value.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CacheNamespace(String);

impl CacheNamespace {
    /// Creates a namespace.
    ///
    /// Empty namespaces are rejected because they make diagnostics and
    /// invalidation policies ambiguous.
    pub fn new(value: impl Into<String>) -> CacheResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(CacheError::InvalidNamespace(
                "namespace must not be empty".to_string(),
            ));
        }

        if value.len() > 256 {
            return Err(CacheError::InvalidNamespace(
                "namespace must not exceed 256 UTF-8 bytes".to_string(),
            ));
        }

        Ok(Self(value))
    }

    /// Returns the namespace string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CacheNamespace {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Cache generation.
///
/// A generation change invalidates entries whose generation no longer matches
/// the current namespace generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CacheGeneration(u64);

impl CacheGeneration {
    /// Creates a generation.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric generation.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Advances the generation using checked arithmetic.
    pub fn next(self) -> CacheResult<Self> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(CacheError::AccountingOverflow)
    }
}

impl Default for CacheGeneration {
    fn default() -> Self {
        Self::new(INITIAL_GENERATION)
    }
}

// =============================================================================
// Hardware and representation identity
// =============================================================================

/// Quantum-state representation identity.
///
/// This is descriptive metadata used for cache separation. It does not
/// implement the representation itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CacheRepresentation {
    /// Dense pure-state representation.
    StateVector,

    /// Mixed-state density matrix.
    DensityMatrix,

    /// Stabilizer/tableau representation.
    Stabilizer,

    /// Sparse state representation.
    Sparse,

    /// Tensor-network representation.
    TensorNetwork,

    /// Backend-owned state.
    BackendNative,

    /// Generic/other representation.
    Other(u16),
}

/// Numerical precision identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CachePrecision {
    /// 32-bit real component.
    F32,

    /// 64-bit real component.
    F64,

    /// 16-bit real component where supported by a backend.
    F16,

    /// 128-bit real component where supported by a backend.
    F128,

    /// Arbitrary/backend-defined precision identified by an opaque ID.
    Custom(u16),
}

/// Storage location identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CacheStorageLocation {
    /// Normal host memory.
    Host,

    /// Pinned host memory.
    PinnedHost,

    /// Accelerator/device memory.
    Device,

    /// Unified/shared memory.
    Unified,

    /// Distributed memory.
    Distributed,

    /// Remote/backend-owned memory.
    Remote,
}

/// Backend/QPU execution domain.
///
/// The cache does not interpret vendor names. The caller supplies a stable
/// identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BackendDomainId(u64);

impl BackendDomainId {
    /// Creates a backend domain identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Hardware execution context.
///
/// This allows the same logical cache operation to coexist for different
/// hardware contexts without embedding vendor-specific types into memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CacheHardwareContext {
    /// Generic backend/QPU identity.
    pub backend: Option<BackendDomainId>,

    /// Device/accelerator identity inside that backend domain.
    pub device_id: Option<u64>,

    /// Runtime/partition identity.
    pub execution_domain: Option<u64>,

    /// Calibration/configuration generation.
    ///
    /// A changed calibration/configuration should use a new value.
    pub configuration_generation: u64,
}

impl CacheHardwareContext {
    /// Creates a hardware-neutral context.
    #[must_use]
    pub const fn generic() -> Self {
        Self {
            backend: None,
            device_id: None,
            execution_domain: None,
            configuration_generation: 0,
        }
    }

    /// Creates a context for a backend/QPU.
    #[must_use]
    pub const fn backend(backend: BackendDomainId) -> Self {
        Self {
            backend: Some(backend),
            device_id: None,
            execution_domain: None,
            configuration_generation: 0,
        }
    }

    /// Returns whether this context is hardware-specific.
    #[must_use]
    pub const fn is_hardware_specific(self) -> bool {
        self.backend.is_some()
            || self.device_id.is_some()
            || self.execution_domain.is_some()
            || self.configuration_generation != 0
    }
}

// =============================================================================
// Layout identity
// =============================================================================

/// Opaque layout fingerprint.
///
/// A layout can encode logical-to-physical order, strides, tensor placement,
/// distributed partitioning, or another memory-layout contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CacheLayoutId(u64);

impl CacheLayoutId {
    /// Creates a layout identity.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the identity.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

// =============================================================================
// Semantic fingerprint
// =============================================================================

/// Opaque semantic fingerprint.
///
/// Higher layers should derive this from every semantic input that can change
/// the cached result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CacheFingerprint(u64);

impl CacheFingerprint {
    /// Creates a fingerprint from a stable value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw fingerprint.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Computes a stable standard-library hash fingerprint for a hashable
    /// value.
    ///
    /// The fingerprint is intended for cache identity, not cryptographic
    /// integrity. Callers requiring cryptographic identity must provide their
    /// own externally computed fingerprint.
    #[must_use]
    pub fn of<T: Hash>(value: &T) -> Self {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        value.hash(&mut hasher);
        Self(hasher.finish())
    }
}

// =============================================================================
// Cache key
// =============================================================================

/// Complete cache identity.
///
/// Every field exists to prevent a class of false cache hits.
///
/// Higher-level modules should construct keys once and pass them through the
/// cache without teaching the cache quantum semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CacheKey {
    /// Cache namespace.
    ///
    /// A separate namespace can be used for state transforms, tensor plans,
    /// backend buffers, migration plans, etc.
    namespace_hash: u64,

    /// User-defined operation/object identity.
    operation: u64,

    /// Semantic input fingerprint.
    semantic: CacheFingerprint,

    /// Quantum-state representation.
    representation: CacheRepresentation,

    /// Numerical precision.
    precision: CachePrecision,

    /// Memory storage location.
    location: CacheStorageLocation,

    /// Memory layout identity.
    layout: CacheLayoutId,

    /// Hardware/backend context.
    hardware: CacheHardwareContext,

    /// State/configuration generation.
    generation: CacheGeneration,
}

impl CacheKey {
    /// Constructs a cache key.
    ///
    /// The namespace itself is converted to a stable hash. The original
    /// namespace is not stored in every entry, reducing per-entry memory
    /// overhead. `namespace` remains part of the complete identity.
    #[must_use]
    pub fn new(
        namespace: &CacheNamespace,
        operation: u64,
        semantic: CacheFingerprint,
        representation: CacheRepresentation,
        precision: CachePrecision,
        location: CacheStorageLocation,
        layout: CacheLayoutId,
        hardware: CacheHardwareContext,
        generation: CacheGeneration,
    ) -> Self {
        Self {
            namespace_hash: CacheFingerprint::of(namespace).value(),
            operation,
            semantic,
            representation,
            precision,
            location,
            layout,
            hardware,
            generation,
        }
    }

    /// Returns the namespace fingerprint.
    #[must_use]
    pub const fn namespace_hash(self) -> u64 {
        self.namespace_hash
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(self) -> u64 {
        self.operation
    }

    /// Returns the semantic fingerprint.
    #[must_use]
    pub const fn semantic(self) -> CacheFingerprint {
        self.semantic
    }

    /// Returns the representation.
    #[must_use]
    pub const fn representation(self) -> CacheRepresentation {
        self.representation
    }

    /// Returns the precision.
    #[must_use]
    pub const fn precision(self) -> CachePrecision {
        self.precision
    }

    /// Returns the storage location.
    #[must_use]
    pub const fn location(self) -> CacheStorageLocation {
        self.location
    }

    /// Returns the layout identity.
    #[must_use]
    pub const fn layout(self) -> CacheLayoutId {
        self.layout
    }

    /// Returns the hardware context.
    #[must_use]
    pub const fn hardware(self) -> CacheHardwareContext {
        self.hardware
    }

    /// Returns the generation.
    #[must_use]
    pub const fn generation(self) -> CacheGeneration {
        self.generation
    }
}

// =============================================================================
// Time abstraction
// =============================================================================

/// Cache expiration timestamp.
///
/// `Instant` is intentionally used internally so expiration is monotonic and
/// is not affected by wall-clock adjustments.
#[derive(Debug, Clone, Copy)]
struct Expiration {
    instant: Instant,
}

impl Expiration {
    fn from_duration(duration: Duration) -> Option<Self> {
        Instant::now().checked_add(duration).map(|instant| Self { instant })
    }

    fn is_expired(self) -> bool {
        Instant::now() >= self.instant
    }
}

// =============================================================================
// Cache entry
// =============================================================================

/// Cached value.
///
/// `Arc` provides cheap immutable sharing without requiring the cache to know
/// the value's concrete type.
#[derive(Debug, Clone)]
pub struct CacheValue<T> {
    value: Arc<T>,
    size_bytes: u64,
}

impl<T> CacheValue<T> {
    /// Creates a cache value with an explicit byte-accounting size.
    ///
    /// The caller must provide the size actually attributable to this cached
    /// value. This allows the cache to represent GPU/device/distributed values
    /// without knowing their implementation.
    pub fn new(value: T, size_bytes: u64) -> Self {
        Self {
            value: Arc::new(value),
            size_bytes,
        }
    }

    /// Returns a shared reference to the cached value.
    #[must_use]
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Returns a cloned `Arc`.
    #[must_use]
    pub fn into_arc(self) -> Arc<T> {
        self.value
    }

    /// Returns the accounted byte size.
    #[must_use]
    pub const fn size_bytes(&self) -> u64 {
        self.size_bytes
    }
}

/// Internal cache entry.
struct Entry<T> {
    value: CacheValue<T>,
    inserted_at: Instant,
    last_access: Instant,
    expiration: Option<Expiration>,
    generation: CacheGeneration,
    pinned: bool,
    access_sequence: u64,
}

impl<T> Entry<T> {
    fn is_expired(&self) -> bool {
        self.expiration
            .map(Expiration::is_expired)
            .unwrap_or(false)
    }
}

// =============================================================================
// Statistics
// =============================================================================

/// Immutable cache statistics snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CacheStatistics {
    /// Number of successful lookups.
    pub hits: u64,

    /// Number of failed lookups.
    pub misses: u64,

    /// Number of entries inserted.
    pub insertions: u64,

    /// Number of replacements of existing entries.
    pub replacements: u64,

    /// Number of capacity evictions.
    pub evictions: u64,

    /// Number of expired entries removed.
    pub expirations: u64,

    /// Number of explicit invalidations.
    pub invalidations: u64,

    /// Number of generation invalidations.
    pub generation_invalidations: u64,

    /// Number of namespace invalidations.
    pub namespace_invalidations: u64,

    /// Number of rejected insertions.
    pub rejected_insertions: u64,

    /// Number of bypasses because caching was disabled.
    pub bypasses: u64,

    /// Number of pin/unpin operations.
    pub pin_operations: u64,

    /// Number of failed oversized-entry attempts.
    pub oversized_rejections: u64,
}

impl CacheStatistics {
    /// Total lookup operations.
    #[must_use]
    pub const fn total_lookups(self) -> u64 {
        self.hits + self.misses
    }

    /// Hit rate in `[0.0, 1.0]`.
    #[must_use]
    pub fn hit_rate(self) -> f64 {
        let total = self.total_lookups();

        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
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
            "hits={}, misses={}, hit_rate={:.4}, insertions={}, \
             replacements={}, evictions={}, expirations={}, \
             invalidations={}, generation_invalidations={}, \
             namespace_invalidations={}, rejected_insertions={}, \
             bypasses={}",
            self.hits,
            self.misses,
            self.hit_rate(),
            self.insertions,
            self.replacements,
            self.evictions,
            self.expirations,
            self.invalidations,
            self.generation_invalidations,
            self.namespace_invalidations,
            self.rejected_insertions,
            self.bypasses
        )
    }
}

// =============================================================================
// Cache configuration
// =============================================================================

/// Cache admission policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheAdmissionPolicy {
    /// Admit every valid entry.
    AdmitAll,

    /// Admit only entries at or below the configured maximum size.
    SizeBounded,

    /// Disable storage while preserving lookup/invalidation API behavior.
    Disabled,
}

/// Cache configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheConfig {
    /// Maximum number of entries.
    pub max_entries: usize,

    /// Maximum total accounted bytes.
    pub max_bytes: u64,

    /// Maximum size of one entry.
    pub max_entry_bytes: u64,

    /// Whether cache hits update LRU recency.
    pub update_recency_on_hit: bool,

    /// Admission policy.
    pub admission: CacheAdmissionPolicy,

    /// Default time-to-live for newly inserted entries.
    ///
    /// `None` means no expiration unless supplied explicitly.
    pub default_ttl: Option<Duration>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: DEFAULT_MAX_ENTRIES,
            max_bytes: DEFAULT_MAX_BYTES,
            max_entry_bytes: DEFAULT_MAX_BYTES,
            update_recency_on_hit: true,
            admission: CacheAdmissionPolicy::AdmitAll,
            default_ttl: None,
        }
    }
}

impl CacheConfig {
    /// Creates production defaults.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_entries: DEFAULT_MAX_ENTRIES,
            max_bytes: DEFAULT_MAX_BYTES,
            max_entry_bytes: DEFAULT_MAX_BYTES,
            update_recency_on_hit: true,
            admission: CacheAdmissionPolicy::AdmitAll,
            default_ttl: None,
        }
    }

    /// Validates the configuration.
    pub fn validate(&self) -> CacheResult<()> {
        if self.max_entries > MAX_MAX_ENTRIES {
            return Err(CacheError::InvalidConfiguration(format!(
                "max_entries {} exceeds maximum {}",
                self.max_entries, MAX_MAX_ENTRIES
            )));
        }

        if self.max_bytes > MAX_MAX_BYTES {
            return Err(CacheError::InvalidConfiguration(format!(
                "max_bytes {} exceeds maximum {}",
                self.max_bytes, MAX_MAX_BYTES
            )));
        }

        if self.max_entry_bytes > self.max_bytes && self.max_bytes != 0 {
            return Err(CacheError::InvalidConfiguration(
                "max_entry_bytes cannot exceed max_bytes".to_string(),
            ));
        }

        Ok(())
    }

    /// Sets the maximum entry count.
    #[must_use]
    pub const fn with_max_entries(
        mut self,
        value: usize,
    ) -> Self {
        self.max_entries = value;
        self
    }

    /// Sets maximum retained bytes.
    #[must_use]
    pub const fn with_max_bytes(
        mut self,
        value: u64,
    ) -> Self {
        self.max_bytes = value;
        self
    }

    /// Sets maximum size for one entry.
    #[must_use]
    pub const fn with_max_entry_bytes(
        mut self,
        value: u64,
    ) -> Self {
        self.max_entry_bytes = value;
        self
    }

    /// Enables or disables recency updates on hits.
    #[must_use]
    pub const fn with_recency_updates(
        mut self,
        enabled: bool,
    ) -> Self {
        self.update_recency_on_hit = enabled;
        self
    }

    /// Sets the admission policy.
    #[must_use]
    pub const fn with_admission_policy(
        mut self,
        policy: CacheAdmissionPolicy,
    ) -> Self {
        self.admission = policy;
        self
    }

    /// Sets the default TTL.
    #[must_use]
    pub const fn with_default_ttl(
        mut self,
        ttl: Option<Duration>,
    ) -> Self {
        self.default_ttl = ttl;
        self
    }
}

// =============================================================================
// Cache insertion options
// =============================================================================

/// Options controlling insertion of one cache entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CacheInsertOptions {
    /// Optional TTL overriding the configured default.
    pub ttl: Option<Duration>,

    /// Whether the entry is pinned against normal LRU eviction.
    pub pinned: bool,

    /// If true, an existing entry is replaced.
    ///
    /// If false, insertion still replaces an existing key because maintaining
    /// one canonical value per key is required. This flag exists to make
    /// caller intent explicit and is retained for API compatibility.
    pub replace_existing: bool,
}

impl CacheInsertOptions {
    /// Creates ordinary insertion options.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            ttl: None,
            pinned: false,
            replace_existing: true,
        }
    }

    /// Sets TTL.
    #[must_use]
    pub const fn with_ttl(
        mut self,
        ttl: Option<Duration>,
    ) -> Self {
        self.ttl = ttl;
        self
    }

    /// Pins the entry.
    #[must_use]
    pub const fn pinned(mut self, value: bool) -> Self {
        self.pinned = value;
        self
    }
}

// =============================================================================
// Cache state
// =============================================================================

/// Generic bounded quantum-memory cache.
///
/// `T` is deliberately unconstrained. The cache can therefore hold:
///
/// - state buffers;
/// - tensor plans;
/// - immutable state metadata;
/// - device handles wrapped by safe types;
/// - migration plans;
/// - distributed partition metadata;
/// - backend-native objects;
/// - compiled reusable memory transformations.
///
/// The cache never assumes what `T` means.
pub struct MemoryCache<T> {
    config: CacheConfig,
    entries: HashMap<CacheKey, Entry<T>>,
    current_bytes: u64,
    access_sequence: u64,
    statistics: CacheStatistics,
}

impl<T> MemoryCache<T> {
    /// Creates a cache using the supplied configuration.
    pub fn new(config: CacheConfig) -> CacheResult<Self> {
        config.validate()?;

        Ok(Self {
            config,
            entries: HashMap::new(),
            current_bytes: 0,
            access_sequence: 0,
            statistics: CacheStatistics::default(),
        })
    }

    /// Creates a cache with production defaults.
    pub fn default_cache() -> Self {
        Self::new(CacheConfig::default())
            .expect("MemoryCache default configuration must be valid")
    }

    /// Returns the cache configuration.
    #[must_use]
    pub const fn config(&self) -> CacheConfig {
        self.config
    }

    /// Returns the number of currently stored entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether the cache is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns currently accounted bytes.
    #[must_use]
    pub const fn current_bytes(&self) -> u64 {
        self.current_bytes
    }

    /// Returns statistics.
    #[must_use]
    pub const fn statistics(&self) -> CacheStatistics {
        self.statistics
    }

    /// Returns the number of available entry slots.
    #[must_use]
    pub fn available_entry_slots(&self) -> usize {
        self.config
            .max_entries
            .saturating_sub(self.entries.len())
    }

    /// Returns remaining byte capacity.
    #[must_use]
    pub fn available_bytes(&self) -> u64 {
        self.config
            .max_bytes
            .saturating_sub(self.current_bytes)
    }

    /// Returns whether caching is enabled.
    #[must_use]
    pub const fn enabled(&self) -> bool {
        !matches!(
            self.config.admission,
            CacheAdmissionPolicy::Disabled
        ) && self.config.max_entries != 0
            && self.config.max_bytes != 0
    }

    /// Looks up a value.
    ///
    /// Expired entries are treated as misses and removed.
    pub fn get(
        &mut self,
        key: &CacheKey,
    ) -> Option<Arc<T>> {
        let expired = self
            .entries
            .get(key)
            .map(Entry::is_expired)
            .unwrap_or(false);

        if expired {
            self.remove_expired_key(*key);
            self.statistics.misses =
                self.statistics.misses.saturating_add(1);
            return None;
        }

        let entry = match self.entries.get_mut(key) {
            Some(entry) => entry,
            None => {
                self.statistics.misses =
                    self.statistics.misses.saturating_add(1);
                return None;
            }
        };

        if self.config.update_recency_on_hit {
            self.access_sequence =
                self.access_sequence.saturating_add(1);

            entry.last_access = Instant::now();
            entry.access_sequence = self.access_sequence;
        }

        self.statistics.hits =
            self.statistics.hits.saturating_add(1);

        Some(Arc::clone(&entry.value.value))
    }

    /// Returns a borrowed cache value when present.
    ///
    /// This method does not update LRU recency and therefore is useful for
    /// inspection APIs where mutation of cache ordering is undesirable.
    #[must_use]
    pub fn peek(
        &self,
        key: &CacheKey,
    ) -> Option<&T> {
        let entry = self.entries.get(key)?;

        if entry.is_expired() {
            return None;
        }

        Some(entry.value.get())
    }

    /// Checks whether a non-expired key exists.
    #[must_use]
    pub fn contains(&self, key: &CacheKey) -> bool {
        self.entries
            .get(key)
            .map(|entry| !entry.is_expired())
            .unwrap_or(false)
    }

    /// Inserts a value using default insertion options.
    pub fn insert(
        &mut self,
        key: CacheKey,
        value: T,
        size_bytes: u64,
    ) -> CacheResult<()> {
        self.insert_with_options(
            key,
            CacheValue::new(value, size_bytes),
            CacheInsertOptions::default(),
        )
    }

    /// Inserts a value with explicit cache-value metadata.
    pub fn insert_value(
        &mut self,
        key: CacheKey,
        value: CacheValue<T>,
        options: CacheInsertOptions,
    ) -> CacheResult<()> {
        self.insert_with_options(key, value, options)
    }

    fn insert_with_options(
        &mut self,
        key: CacheKey,
        value: CacheValue<T>,
        options: CacheInsertOptions,
    ) -> CacheResult<()> {
        let size_bytes = value.size_bytes();

        if size_bytes > self.config.max_entry_bytes {
            self.statistics.oversized_rejections = self
                .statistics
                .oversized_rejections
                .saturating_add(1);

            return Err(CacheError::EntryTooLarge {
                size_bytes,
                maximum_bytes: self.config.max_entry_bytes,
            });
        }

        if matches!(
            self.config.admission,
            CacheAdmissionPolicy::Disabled
        ) || self.config.max_entries == 0
            || self.config.max_bytes == 0
        {
            self.statistics.bypasses =
                self.statistics.bypasses.saturating_add(1);
            return Ok(());
        }

        if self.config.admission == CacheAdmissionPolicy::SizeBounded
            && size_bytes > self.config.max_entry_bytes
        {
            self.statistics.rejected_insertions = self
                .statistics
                .rejected_insertions
                .saturating_add(1);
            return Ok(());
        }

        // Remove an existing entry before accounting for the replacement.
        if let Some(existing) = self.entries.remove(&key) {
            self.current_bytes =
                self.current_bytes.saturating_sub(existing.value.size_bytes());

            self.statistics.replacements =
                self.statistics.replacements.saturating_add(1);
        }

        // Remove expired entries before deciding what to evict.
        self.remove_expired_entries();

        // Ensure enough space.
        self.make_room_for(size_bytes)?;

        self.access_sequence =
            self.access_sequence.saturating_add(1);

        let now = Instant::now();

        let expiration = options
            .ttl
            .or(self.config.default_ttl)
            .and_then(Expiration::from_duration);

        let generation = key.generation();

        let entry = Entry {
            value,
            inserted_at: now,
            last_access: now,
            expiration,
            generation,
            pinned: options.pinned,
            access_sequence: self.access_sequence,
        };

        self.current_bytes = self
            .current_bytes
            .checked_add(size_bytes)
            .ok_or(CacheError::AccountingOverflow)?;

        self.entries.insert(key, entry);

        self.statistics.insertions =
            self.statistics.insertions.saturating_add(1);

        Ok(())
    }

    /// Removes a key.
    ///
    /// Returns the removed value if present and valid.
    pub fn remove(
        &mut self,
        key: &CacheKey,
    ) -> Option<Arc<T>> {
        let entry = self.entries.remove(key)?;

        self.current_bytes =
            self.current_bytes.saturating_sub(entry.value.size_bytes());

        self.statistics.invalidations =
            self.statistics.invalidations.saturating_add(1);

        Some(entry.value.value)
    }

    /// Invalidates all entries belonging to the supplied generation.
    ///
    /// Returns the number of removed entries.
    pub fn invalidate_generation(
        &mut self,
        generation: CacheGeneration,
    ) -> usize {
        let keys: Vec<CacheKey> = self
            .entries
            .iter()
            .filter_map(|(key, entry)| {
                if entry.generation == generation {
                    Some(*key)
                } else {
                    None
                }
            })
            .collect();

        let count = keys.len();

        for key in keys {
            self.remove_without_general_stat(key);
        }

        self.statistics.generation_invalidations = self
            .statistics
            .generation_invalidations
            .saturating_add(count as u64);

        count
    }

    /// Invalidates entries belonging to a namespace fingerprint.
    ///
    /// The caller should use the same `CacheNamespace` used to construct the
    /// keys.
    pub fn invalidate_namespace(
        &mut self,
        namespace: &CacheNamespace,
    ) -> usize {
        let namespace_hash =
            CacheFingerprint::of(namespace).value();

        let keys: Vec<CacheKey> = self
            .entries
            .keys()
            .filter_map(|key| {
                if key.namespace_hash() == namespace_hash {
                    Some(*key)
                } else {
                    None
                }
            })
            .collect();

        let count = keys.len();

        for key in keys {
            self.remove_without_general_stat(key);
        }

        self.statistics.namespace_invalidations = self
            .statistics
            .namespace_invalidations
            .saturating_add(count as u64);

        count
    }

    /// Invalidates all entries matching a caller-defined predicate.
    ///
    /// This is intentionally key-only: the cache must not interpret the
    /// cached value.
    pub fn invalidate_where<F>(
        &mut self,
        predicate: F,
    ) -> usize
    where
        F: FnMut(&CacheKey) -> bool,
    {
        let mut predicate = predicate;

        let keys: Vec<CacheKey> = self
            .entries
            .keys()
            .filter_map(|key| {
                if predicate(key) {
                    Some(*key)
                } else {
                    None
                }
            })
            .collect();

        let count = keys.len();

        for key in keys {
            self.remove_without_general_stat(key);
        }

        self.statistics.invalidations = self
            .statistics
            .invalidations
            .saturating_add(count as u64);

        count
    }

    /// Invalidates every entry.
    pub fn clear(&mut self) {
        let count = self.entries.len();

        self.entries.clear();
        self.current_bytes = 0;

        self.statistics.invalidations = self
            .statistics
            .invalidations
            .saturating_add(count as u64);
    }

    /// Removes all expired entries.
    ///
    /// Returns the number of removed entries.
    pub fn remove_expired_entries(&mut self) -> usize {
        let keys: Vec<CacheKey> = self
            .entries
            .iter()
            .filter_map(|(key, entry)| {
                if entry.is_expired() && !entry.pinned {
                    Some(*key)
                } else {
                    None
                }
            })
            .collect();

        let count = keys.len();

        for key in keys {
            self.remove_expired_key(key);
        }

        count
    }

    /// Pins an existing entry against LRU eviction.
    ///
    /// Expiration still applies. Pinning is not a correctness bypass.
    pub fn pin(
        &mut self,
        key: &CacheKey,
    ) -> CacheResult<()> {
        let entry = self
            .entries
            .get_mut(key)
            .ok_or(CacheError::EntryNotFound)?;

        entry.pinned = true;

        self.statistics.pin_operations = self
            .statistics
            .pin_operations
            .saturating_add(1);

        Ok(())
    }

    /// Removes an entry's eviction pin.
    pub fn unpin(
        &mut self,
        key: &CacheKey,
    ) -> CacheResult<()> {
        let entry = self
            .entries
            .get_mut(key)
            .ok_or(CacheError::EntryNotFound)?;

        entry.pinned = false;

        self.statistics.pin_operations = self
            .statistics
            .pin_operations
            .saturating_add(1);

        Ok(())
    }

    /// Returns whether an entry is pinned.
    #[must_use]
    pub fn is_pinned(
        &self,
        key: &CacheKey,
    ) -> bool {
        self.entries
            .get(key)
            .map(|entry| entry.pinned)
            .unwrap_or(false)
    }

    /// Returns the insertion time of an entry.
    #[must_use]
    pub fn inserted_at(
        &self,
        key: &CacheKey,
    ) -> Option<Instant> {
        self.entries.get(key).map(|entry| entry.inserted_at)
    }

    /// Returns the last-access time of an entry.
    #[must_use]
    pub fn last_access(
        &self,
        key: &CacheKey,
    ) -> Option<Instant> {
        self.entries.get(key).map(|entry| entry.last_access)
    }

    /// Returns the accounted size of an entry.
    #[must_use]
    pub fn entry_size_bytes(
        &self,
        key: &CacheKey,
    ) -> Option<u64> {
        self.entries
            .get(key)
            .map(|entry| entry.value.size_bytes())
    }

    /// Removes entries until a new entry of `required_bytes` can fit.
    fn make_room_for(
        &mut self,
        required_bytes: u64,
    ) -> CacheResult<()> {
        if required_bytes > self.config.max_bytes {
            return Err(CacheError::EntryTooLarge {
                size_bytes: required_bytes,
                maximum_bytes: self.config.max_bytes,
            });
        }

        while self.entries.len() >= self.config.max_entries
            || self
                .current_bytes
                .checked_add(required_bytes)
                .ok_or(CacheError::AccountingOverflow)?
                > self.config.max_bytes
        {
            let candidate = self.oldest_evictable_key();

            match candidate {
                Some(key) => {
                    self.remove_without_general_stat(key);

                    self.statistics.evictions = self
                        .statistics
                        .evictions
                        .saturating_add(1);
                }
                None => {
                    self.statistics.rejected_insertions = self
                        .statistics
                        .rejected_insertions
                        .saturating_add(1);

                    return Err(CacheError::InvalidConfiguration(
                        "cache has insufficient evictable capacity; \
                         all retained entries may be pinned"
                            .to_string(),
                    ));
                }
            }
        }

        Ok(())
    }

    /// Finds the least-recently-used non-pinned entry.
    ///
    /// Ties are resolved by cache key, making eviction deterministic.
    fn oldest_evictable_key(&self) -> Option<CacheKey> {
        self.entries
            .iter()
            .filter(|(_, entry)| !entry.pinned)
            .min_by(|(key_a, entry_a), (key_b, entry_b)| {
                entry_a
                    .access_sequence
                    .cmp(&entry_b.access_sequence)
                    .then_with(|| key_a.cmp(key_b))
            })
            .map(|(key, _)| *key)
    }

    fn remove_expired_key(
        &mut self,
        key: CacheKey,
    ) {
        if let Some(entry) = self.entries.remove(&key) {
            self.current_bytes =
                self.current_bytes.saturating_sub(entry.value.size_bytes());

            self.statistics.expirations = self
                .statistics
                .expirations
                .saturating_add(1);
        }
    }

    fn remove_without_general_stat(
        &mut self,
        key: CacheKey,
    ) {
        if let Some(entry) = self.entries.remove(&key) {
            self.current_bytes =
                self.current_bytes.saturating_sub(entry.value.size_bytes());
        }
    }
}

// =============================================================================
// Namespace generation registry
// =============================================================================

/// Generation registry for independently invalidatable cache domains.
///
/// This is deliberately separate from `MemoryCache` so a higher-level
/// `QuantumMemory` manager can maintain generations without making cache
/// storage responsible for semantic lifecycle.
#[derive(Debug, Default)]
pub struct CacheGenerationRegistry {
    generations: HashMap<u64, CacheGeneration>,
}

impl CacheGenerationRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            generations: HashMap::new(),
        }
    }

    /// Returns the current generation for a namespace.
    #[must_use]
    pub fn current(
        &self,
        namespace: &CacheNamespace,
    ) -> CacheGeneration {
        let key = CacheFingerprint::of(namespace).value();

        self.generations
            .get(&key)
            .copied()
            .unwrap_or_default()
    }

    /// Advances a namespace generation.
    pub fn invalidate(
        &mut self,
        namespace: &CacheNamespace,
    ) -> CacheResult<CacheGeneration> {
        let key = CacheFingerprint::of(namespace).value();

        let current = self
            .generations
            .get(&key)
            .copied()
            .unwrap_or_default();

        let next = current.next()?;

        self.generations.insert(key, next);

        Ok(next)
    }

    /// Resets a namespace generation to zero.
    pub fn reset(
        &mut self,
        namespace: &CacheNamespace,
    ) {
        let key = CacheFingerprint::of(namespace).value();

        self.generations
            .insert(key, CacheGeneration::default());
    }

    /// Clears all generation state.
    pub fn clear(&mut self) {
        self.generations.clear();
    }
}

// =============================================================================
// Cache scope
// =============================================================================

/// Reusable cache scope.
///
/// This avoids repeatedly reconstructing the stable identity fields when a
/// caller inserts many related values.
#[derive(Debug, Clone, Copy)]
pub struct CacheScope {
    namespace_hash: u64,
    representation: CacheRepresentation,
    precision: CachePrecision,
    location: CacheStorageLocation,
    layout: CacheLayoutId,
    hardware: CacheHardwareContext,
    generation: CacheGeneration,
}

impl CacheScope {
    /// Creates a cache scope.
    #[must_use]
    pub fn new(
        namespace: &CacheNamespace,
        representation: CacheRepresentation,
        precision: CachePrecision,
        location: CacheStorageLocation,
        layout: CacheLayoutId,
        hardware: CacheHardwareContext,
        generation: CacheGeneration,
    ) -> Self {
        Self {
            namespace_hash: CacheFingerprint::of(namespace).value(),
            representation,
            precision,
            location,
            layout,
            hardware,
            generation,
        }
    }

    /// Constructs a complete key from an operation and semantic fingerprint.
    #[must_use]
    pub fn key(
        self,
        operation: u64,
        semantic: CacheFingerprint,
    ) -> CacheKey {
        CacheKey {
            namespace_hash: self.namespace_hash,
            operation,
            semantic,
            representation: self.representation,
            precision: self.precision,
            location: self.location,
            layout: self.layout,
            hardware: self.hardware,
            generation: self.generation,
        }
    }

    /// Returns the generation.
    #[must_use]
    pub const fn generation(self) -> CacheGeneration {
        self.generation
    }

    /// Returns the representation.
    #[must_use]
    pub const fn representation(self) -> CacheRepresentation {
        self.representation
    }

    /// Returns the storage location.
    #[must_use]
    pub const fn location(self) -> CacheStorageLocation {
        self.location
    }

    /// Returns the hardware context.
    #[must_use]
    pub const fn hardware(self) -> CacheHardwareContext {
        self.hardware
    }
}

// =============================================================================
// Deterministic cache metadata
// =============================================================================

/// Metadata describing the current cache state.
///
/// This type intentionally excludes individual values so diagnostics can
/// inspect the cache without exposing potentially large state objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheMetadata {
    /// Implementation version.
    pub cache_version: &'static str,

    /// Number of entries.
    pub entries: usize,

    /// Accounted bytes.
    pub bytes: u64,

    /// Maximum entries.
    pub max_entries: usize,

    /// Maximum bytes.
    pub max_bytes: u64,

    /// Number of available entry slots.
    pub available_entry_slots: usize,

    /// Remaining byte capacity.
    pub available_bytes: u64,
}

impl<T> MemoryCache<T> {
    /// Returns cache metadata.
    #[must_use]
    pub fn metadata(&self) -> CacheMetadata {
        CacheMetadata {
            cache_version: CACHE_VERSION,
            entries: self.len(),
            bytes: self.current_bytes,
            max_entries: self.config.max_entries,
            max_bytes: self.config.max_bytes,
            available_entry_slots: self.available_entry_slots(),
            available_bytes: self.available_bytes(),
        }
    }
}

// =============================================================================
// Optional wall-clock cache timestamp
// =============================================================================

/// Returns a Unix timestamp in nanoseconds.
///
/// This helper is intended for diagnostics/reproducibility metadata only.
/// Expiration logic uses `Instant`, never wall-clock time.
#[must_use]
pub fn unix_time_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn namespace() -> CacheNamespace {
        CacheNamespace::new("quantum.memory.test")
            .expect("test namespace must be valid")
    }

    fn key(operation: u64) -> CacheKey {
        CacheKey::new(
            &namespace(),
            operation,
            CacheFingerprint::new(operation + 1000),
            CacheRepresentation::StateVector,
            CachePrecision::F64,
            CacheStorageLocation::Host,
            CacheLayoutId::new(1),
            CacheHardwareContext::generic(),
            CacheGeneration::new(0),
        )
    }

    #[test]
    fn default_configuration_is_valid() {
        assert!(CacheConfig::default().validate().is_ok());
    }

    #[test]
    fn insert_and_get() {
        let mut cache =
            MemoryCache::<u64>::new(CacheConfig::default())
                .expect("cache must construct");

        cache
            .insert(key(1), 42, 8)
            .expect("insert must succeed");

        let value = cache.get(&key(1));

        assert_eq!(value.as_deref(), Some(&42));
        assert_eq!(cache.statistics().hits, 1);
    }

    #[test]
    fn missing_key_is_miss() {
        let mut cache =
            MemoryCache::<u64>::new(CacheConfig::default())
                .expect("cache must construct");

        assert!(cache.get(&key(99)).is_none());
        assert_eq!(cache.statistics().misses, 1);
    }

    #[test]
    fn cache_is_bounded_by_entries() {
        let config = CacheConfig::default()
            .with_max_entries(2)
            .with_max_bytes(1024)
            .with_max_entry_bytes(1024);

        let mut cache =
            MemoryCache::<u64>::new(config)
                .expect("cache must construct");

        cache.insert(key(1), 1, 8).unwrap();
        cache.insert(key(2), 2, 8).unwrap();
        cache.insert(key(3), 3, 8).unwrap();

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.statistics().evictions, 1);
        assert!(!cache.contains(&key(1)));
        assert!(cache.contains(&key(2)));
        assert!(cache.contains(&key(3)));
    }

    #[test]
    fn lru_order_is_deterministic() {
        let config = CacheConfig::default()
            .with_max_entries(2)
            .with_max_bytes(1024)
            .with_max_entry_bytes(1024);

        let mut cache =
            MemoryCache::<u64>::new(config)
                .expect("cache must construct");

        cache.insert(key(1), 1, 8).unwrap();
        cache.insert(key(2), 2, 8).unwrap();

        let _ = cache.get(&key(1));

        cache.insert(key(3), 3, 8).unwrap();

        assert!(cache.contains(&key(1)));
        assert!(!cache.contains(&key(2)));
        assert!(cache.contains(&key(3)));
    }

    #[test]
    fn byte_limit_is_enforced() {
        let config = CacheConfig::default()
            .with_max_entries(100)
            .with_max_bytes(16)
            .with_max_entry_bytes(16);

        let mut cache =
            MemoryCache::<u64>::new(config)
                .expect("cache must construct");

        cache.insert(key(1), 1, 8).unwrap();
        cache.insert(key(2), 2, 8).unwrap();
        cache.insert(key(3), 3, 8).unwrap();

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.current_bytes(), 16);
    }

    #[test]
    fn oversized_entry_is_rejected() {
        let config = CacheConfig::default()
            .with_max_entries(4)
            .with_max_bytes(16)
            .with_max_entry_bytes(8);

        let mut cache =
            MemoryCache::<u64>::new(config)
                .expect("cache must construct");

        let result = cache.insert(key(1), 1, 9);

        assert!(matches!(
            result,
            Err(CacheError::EntryTooLarge { .. })
        ));

        assert_eq!(
            cache.statistics().oversized_rejections,
            1
        );
    }

    #[test]
    fn pinned_entries_are_not_evicted() {
        let config = CacheConfig::default()
            .with_max_entries(2)
            .with_max_bytes(1024)
            .with_max_entry_bytes(1024);

        let mut cache =
            MemoryCache::<u64>::new(config)
                .expect("cache must construct");

        cache.insert(key(1), 1, 8).unwrap();
        cache.insert(key(2), 2, 8).unwrap();
        cache.pin(&key(1)).unwrap();

        cache.insert(key(3), 3, 8).unwrap();

        assert!(cache.contains(&key(1)));
        assert!(!cache.contains(&key(2)));
        assert!(cache.contains(&key(3)));
    }

    #[test]
    fn generation_invalidation_removes_matching_generation() {
        let config = CacheConfig::default();

        let mut cache =
            MemoryCache::<u64>::new(config)
                .expect("cache must construct");

        cache.insert(key(1), 1, 8).unwrap();

        let removed =
            cache.invalidate_generation(CacheGeneration::new(0));

        assert_eq!(removed, 1);
        assert!(cache.is_empty());
    }

    #[test]
    fn namespace_invalidation_is_isolated() {
        let config = CacheConfig::default();

        let namespace_a =
            CacheNamespace::new("a").unwrap();
        let namespace_b =
            CacheNamespace::new("b").unwrap();

        let key_a = CacheKey::new(
            &namespace_a,
            1,
            CacheFingerprint::new(1),
            CacheRepresentation::StateVector,
            CachePrecision::F64,
            CacheStorageLocation::Host,
            CacheLayoutId::new(1),
            CacheHardwareContext::generic(),
            CacheGeneration::new(0),
        );

        let key_b = CacheKey::new(
            &namespace_b,
            1,
            CacheFingerprint::new(1),
            CacheRepresentation::StateVector,
            CachePrecision::F64,
            CacheStorageLocation::Host,
            CacheLayoutId::new(1),
            CacheHardwareContext::generic(),
            CacheGeneration::new(0),
        );

        let mut cache =
            MemoryCache::<u64>::new(config)
                .expect("cache must construct");

        cache.insert(key_a, 1, 8).unwrap();
        cache.insert(key_b, 2, 8).unwrap();

        assert_eq!(cache.invalidate_namespace(&namespace_a), 1);

        assert!(!cache.contains(&key_a));
        assert!(cache.contains(&key_b));
    }

    #[test]
    fn disabled_cache_does_not_store() {
        let config = CacheConfig::default()
            .with_admission_policy(
                CacheAdmissionPolicy::Disabled,
            );

        let mut cache =
            MemoryCache::<u64>::new(config)
                .expect("cache must construct");

        cache.insert(key(1), 1, 8).unwrap();

        assert!(cache.is_empty());
        assert_eq!(cache.statistics().bypasses, 1);
    }

    #[test]
    fn representation_is_part_of_identity() {
        let namespace = namespace();

        let state_vector = CacheKey::new(
            &namespace,
            1,
            CacheFingerprint::new(1),
            CacheRepresentation::StateVector,
            CachePrecision::F64,
            CacheStorageLocation::Host,
            CacheLayoutId::new(1),
            CacheHardwareContext::generic(),
            CacheGeneration::new(0),
        );

        let density_matrix = CacheKey::new(
            &namespace,
            1,
            CacheFingerprint::new(1),
            CacheRepresentation::DensityMatrix,
            CachePrecision::F64,
            CacheStorageLocation::Host,
            CacheLayoutId::new(1),
            CacheHardwareContext::generic(),
            CacheGeneration::new(0),
        );

        assert_ne!(state_vector, density_matrix);
    }

    #[test]
    fn hardware_context_is_part_of_identity() {
        let namespace = namespace();

        let cpu = CacheKey::new(
            &namespace,
            1,
            CacheFingerprint::new(1),
            CacheRepresentation::BackendNative,
            CachePrecision::F64,
            CacheStorageLocation::Remote,
            CacheLayoutId::new(1),
            CacheHardwareContext::generic(),
            CacheGeneration::new(0),
        );

        let qpu = CacheKey::new(
            &namespace,
            1,
            CacheFingerprint::new(1),
            CacheRepresentation::BackendNative,
            CachePrecision::F64,
            CacheStorageLocation::Remote,
            CacheLayoutId::new(1),
            CacheHardwareContext::backend(
                BackendDomainId::new(7),
            ),
            CacheGeneration::new(0),
        );

        assert_ne!(cpu, qpu);
    }

    #[test]
    fn scope_builds_consistent_keys() {
        let namespace = namespace();

        let scope = CacheScope::new(
            &namespace,
            CacheRepresentation::TensorNetwork,
            CachePrecision::F64,
            CacheStorageLocation::Device,
            CacheLayoutId::new(55),
            CacheHardwareContext::backend(
                BackendDomainId::new(10),
            ),
            CacheGeneration::new(3),
        );

        let first =
            scope.key(1, CacheFingerprint::new(100));
        let second =
            scope.key(1, CacheFingerprint::new(100));

        assert_eq!(first, second);
    }

    #[test]
    fn generation_registry_advances() {
        let namespace = namespace();

        let mut registry =
            CacheGenerationRegistry::new();

        assert_eq!(
            registry.current(&namespace),
            CacheGeneration::new(0)
        );

        let next =
            registry.invalidate(&namespace).unwrap();

        assert_eq!(next, CacheGeneration::new(1));
        assert_eq!(
            registry.current(&namespace),
            CacheGeneration::new(1)
        );
    }

    #[test]
    fn cache_value_uses_shared_ownership() {
        let value = CacheValue::new(String::from("state"), 5);
        let first = value.clone();
        let second = value.into_arc();

        assert_eq!(first.get(), "state");
        assert_eq!(second.as_str(), "state");
    }

    #[test]
    fn metadata_is_consistent() {
        let mut cache =
            MemoryCache::<u64>::default_cache();

        cache.insert(key(1), 1, 8).unwrap();

        let metadata = cache.metadata();

        assert_eq!(metadata.entries, 1);
        assert_eq!(metadata.bytes, 8);
        assert_eq!(
            metadata.available_entry_slots,
            DEFAULT_MAX_ENTRIES - 1
        );
        assert_eq!(
            metadata.available_bytes,
            DEFAULT_MAX_BYTES - 8
        );
    }
}