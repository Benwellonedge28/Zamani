//! Zamani Quantum Memory — Provider-Neutral Memory Pool
//!
//! Production-grade reusable allocation pool for `quantum::memory`.
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - reusable `MemoryAllocation` objects;
//! - allocation reuse by exact storage requirements;
//! - bounded cached-memory capacity;
//! - bounded cached-allocation count;
//! - bounded per-request buckets;
//! - deterministic eviction;
//! - provider-neutral cacheability policy;
//! - pool hit/miss accounting;
//! - explicit cache trimming;
//! - explicit cache clearing;
//! - RAII-friendly pooled acquisition through `PooledAllocation`;
//! - protection against cross-allocator recycling;
//! - protection against unsafe backend-native reuse by default;
//! - compatibility with host, accelerator, distributed, and QPU-native
//!   allocation handles without knowing their implementation details.
//!
//! # What this module does NOT own
//!
//! This module deliberately does NOT own:
//!
//! - actual memory allocation;
//! - raw pointers;
//! - GPU APIs;
//! - CUDA/HIP/Metal/Vulkan implementations;
//! - distributed communication;
//! - QPU APIs;
//! - state-vector mathematics;
//! - density-matrix mathematics;
//! - stabilizer/tableau mathematics;
//! - tensor-network mathematics;
//! - memory budgets;
//! - memory limits;
//! - quantum IR;
//! - routing;
//! - scheduling;
//! - benchmarking.
//!
//! Those responsibilities remain in their owning modules.
//!
//! # Architectural position
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! runtime / simulator / state representation
//!      │
//!      ▼
//! quantum::memory::allocator
//!      │
//!      ▼
//! quantum::memory::pool       <── this module
//!      │
//!      ├──────── Host
//!      ├──────── Pinned Host
//!      ├──────── Device / GPU
//!      ├──────── Unified Memory
//!      ├──────── Distributed Memory
//!      └──────── Backend/QPU Native
//! ```
//!
//! The pool is therefore a layer ABOVE `MemoryAllocator` and BELOW state
//! representations and execution systems.
//!
//! # Critical ownership rule
//!
//! A cached allocation remains a live allocation from the allocator's point
//! of view. Caching therefore does NOT bypass allocator accounting or memory
//! limits.
//!
//! ```text
//! acquire
//!    │
//!    ├── cache hit ──► return existing allocation
//!    │
//!    └── cache miss ─► allocator.allocate()
//!
//! recycle
//!    │
//!    ├── cacheable + capacity available ─► cache allocation
//!    │
//!    └── otherwise ──────────────────────► drop allocation
//! ```
//!
//! This is intentional. A memory pool is an allocation-reuse optimization,
//! not a second accounting system.
//!
//! # Exact-match reuse
//!
//! An allocation is reusable only when all semantically relevant allocation
//! request fields match:
//!
//! - byte count;
//! - storage location;
//! - allocation class;
//! - state-element count;
//! - logical-qubit count;
//! - diagnostic label.
//!
//! The diagnostic label is included deliberately. Reusing an allocation with
//! a different label would make diagnostics and telemetry misleading.
//!
//! # Provider-neutral hardware rule
//!
//! This pool does not know what a "GPU", "QPU", "simulator", "neutral atom
//! device", "trapped-ion device", "photonic device", or "superconducting
//! device" actually is.
//!
//! It operates entirely on `MemoryAllocation` and `MemoryLocation`.
//!
//! Consequently it can work with:
//!
//! - CPU memory;
//! - pinned host memory;
//! - CUDA/HIP/device allocations;
//! - Metal/Vulkan/SYCL-style allocations;
//! - unified memory;
//! - distributed-memory providers;
//! - remote simulator memory;
//! - QPU-native memory;
//! - future Zamani hardware providers.
//!
//! Backend-native and distributed allocations are NOT cached by default.
//! They can be explicitly enabled through `CacheabilityPolicy::AllLocations`
//! when the corresponding provider guarantees that a resource may safely be
//! retained and reused.
//!
//! # Why backend-native caching is opt-in
//!
//! A QPU provider may associate an allocation with:
//!
//! - a session;
//! - a reservation;
//! - a job;
//! - a calibration epoch;
//! - a remote execution context;
//! - a device lease;
//! - a provider-side lifetime.
//!
//! Generic memory code cannot know whether such a resource remains valid.
//!
//! Therefore the safe default is to release backend-native resources rather
//! than silently retain them.
//!
//! # No unsafe
//!
//! This file contains no `unsafe` code and forbids unsafe code explicitly.
//!
//! Provider implementations may use FFI internally in their own modules, but
//! no unsafe implementation detail crosses this pool API.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features.
//!
//! # Integration contract
//!
//! `allocator.rs`
//! ----------------
//! `MemoryPool` receives a `MemoryAllocator` and uses only its public
//! allocation API. It never bypasses allocator accounting.
//!
//! `reservation.rs`
//! ----------------
//! Reservations may be performed above the pool. The pool does not create a
//! second reservation model.
//!
//! `budget.rs`
//! ------------
//! Budgets remain authoritative. Pool-cached allocations remain counted by the
//! allocator and therefore remain visible to higher-level budgeting.
//!
//! `state.rs`
//! ----------
//! State representations may acquire temporary/state allocations from the
//! pool. They must recycle them after their logical lifetime ends.
//!
//! `state_vector.rs`
//! -----------------
//! Temporary work buffers can be pooled. State allocations may also be pooled
//! when their lifecycle and representation semantics permit it.
//!
//! `density_matrix.rs`
//! -------------------
//! Large matrix buffers may use exact-match pooling.
//!
//! `stabilizer.rs`
//! ---------------
//! Tableau/workspace buffers may use pooling without requiring dense quantum
//! state storage.
//!
//! `sparse.rs`
//! -----------
//! Sparse storage may pool temporary backing allocations when its representation
//! contract permits.
//!
//! `tensor_network.rs`
//! -------------------
//! Tensor contraction workspaces can use pooling for repeated contractions.
//!
//! `backend_state.rs`
//! -----------------
//! Backend-native resources are safe to acquire through the pool but are not
//! cached by default.
//!
//! `gpu.rs`
//! --------
//! GPU providers register with `MemoryAllocator`; the pool automatically works
//! with their opaque `MemoryAllocation` objects.
//!
//! `distributed.rs`
//! ----------------
//! Distributed allocations are supported but not cached under the safe
//! default policy.
//!
//! `migration.rs`
//! --------------
//! Migration can acquire destination buffers from the pool and recycle them
//! after migration.
//!
//! `compaction.rs`
//! --------------
//! Compaction can use temporary pooled allocations.
//!
//! `diagnostics.rs`
//! ----------------
//! Diagnostics can consume `PoolStats` without knowing pool internals.
//!
//! `telemetry.rs`
//! -------------
//! Telemetry can export hit/miss/eviction/cache-size counters.
//!
//! `snapshot.rs` / `checkpoint.rs`
//! ------------------------------
//! Persistent allocations can be pooled only if their lifecycle semantics
//! explicitly permit reuse. Exact request matching prevents accidental
//! cross-class reuse.
//!
//! `mod.rs`
//! --------
//! The final memory module should export this file with:
//!
//! ```text
//! pub mod pool;
//! ```
//!
//! No implementation changes inside this file are required merely because
//! `mod.rs` later exposes it.
//!
//! # Production invariants
//!
//! The pool maintains these invariants:
//!
//! 1. Every cached allocation belongs to the pool's allocator.
//! 2. Cached allocation byte count equals its request byte count.
//! 3. Cached allocation request exactly matches its bucket key.
//! 4. Cached byte accounting equals the sum of cached allocation sizes.
//! 5. Cached allocation accounting equals the number of cached allocations.
//! 6. Cached bytes never exceed configured maximum cached bytes.
//! 7. Cached allocation count never exceeds configured maximum count.
//! 8. Per-bucket count never exceeds configured bucket capacity.
//! 9. Cache entries are never duplicated by allocation identity.
//! 10. Backend-native/distributed resources are not cached unless policy allows.
//! 11. Pool eviction only drops owned `MemoryAllocation` values.
//! 12. No raw address is stored.
//! 13. No provider-specific type is stored.
//! 14. No global mutable state exists.
//! 15. Pool operations are deterministic.
//! 16. Pool failure never changes quantum-state semantics.
//!
//! # Performance model
//!
//! Cache lookup is:
//!
//! - O(log B) to find a request bucket;
//! - O(1) to pop from the selected bucket;
//!
//! where `B` is the number of distinct request buckets.
//!
//! FIFO eviction is maintained through a global eviction queue. Stale queue
//! entries are tolerated and removed lazily; this avoids unsafe cross-index
//! manipulation and keeps the implementation robust under concurrent acquire
//! and recycle operations.
//!
//! The pool is deliberately bounded. It must never become an unbounded memory
//! retention mechanism.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, VecDeque};
use std::fmt;
use std::sync::{Arc, Mutex};

use super::allocator::{
    AllocationClass, AllocationRequest, MemoryAllocation, MemoryAllocator, MemoryLocation,
    MemoryLocationKind,
};
use super::errors::MemoryError;
use super::types::AllocationId;

// =============================================================================
// Schema
// =============================================================================

/// Stable pool schema identifier.
pub const MEMORY_POOL_SCHEMA_ID: &str = "zamani.quantum.memory.pool";

/// Semantic version of the pool contract.
pub const MEMORY_POOL_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Cacheability policy
// =============================================================================

/// Defines which storage domains may be retained in the pool after recycling.
///
/// The policy exists because generic memory code cannot safely infer whether
/// provider-owned distributed or backend-native resources remain valid after
/// their original logical operation has completed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CacheabilityPolicy {
    /// Cache ordinary host and accelerator allocations, but not distributed
    /// or backend-native allocations.
    ///
    /// This is the production-safe default.
    Safe,

    /// Cache every provider location.
    ///
    /// This must only be selected when registered providers explicitly
    /// guarantee that their allocations are safe to retain and reuse.
    AllLocations,
}

impl Default for CacheabilityPolicy {
    fn default() -> Self {
        Self::Safe
    }
}

impl CacheabilityPolicy {
    /// Returns whether an allocation at the supplied location may be cached.
    pub const fn allows(self, location: &MemoryLocation) -> bool {
        match self {
            Self::Safe => matches!(
                location,
                MemoryLocation::Host
                    | MemoryLocation::PinnedHost
                    | MemoryLocation::Device { .. }
                    | MemoryLocation::Unified { .. }
            ),
            Self::AllLocations => true,
        }
    }

    /// Returns whether this policy permits backend-native caching.
    pub const fn allows_backend_native(self) -> bool {
        matches!(self, Self::AllLocations)
    }

    /// Returns whether this policy permits distributed caching.
    pub const fn allows_distributed(self) -> bool {
        matches!(self, Self::AllLocations)
    }
}

// =============================================================================
// Eviction policy
// =============================================================================

/// Deterministic policy used when the pool must evict cached allocations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EvictionPolicy {
    /// Evict the oldest cached allocation first.
    ///
    /// This provides predictable FIFO behavior.
    OldestFirst,

    /// Evict the largest cached allocation first.
    ///
    /// Useful when a pool must free a large amount of memory quickly.
    LargestFirst,
}

impl Default for EvictionPolicy {
    fn default() -> Self {
        Self::OldestFirst
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Immutable production configuration for a `MemoryPool`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryPoolConfig {
    /// Maximum total bytes retained in the pool.
    ///
    /// `0` disables caching.
    pub max_cached_bytes: u64,

    /// Maximum number of allocations retained in the pool.
    ///
    /// `0` disables caching.
    pub max_cached_allocations: u64,

    /// Maximum allocations retained for one exact request bucket.
    ///
    /// `0` disables caching.
    pub max_allocations_per_bucket: u64,

    /// Maximum number of distinct request buckets.
    ///
    /// `0` disables caching.
    pub max_buckets: usize,

    /// Determines which storage domains may be cached.
    pub cacheability: CacheabilityPolicy,

    /// Determines which cached entries are evicted first.
    pub eviction_policy: EvictionPolicy,
}

impl Default for MemoryPoolConfig {
    fn default() -> Self {
        Self::production()
    }
}

impl MemoryPoolConfig {
    /// Conservative production defaults.
    ///
    /// These values deliberately bound retention rather than attempting to
    /// consume all available memory.
    pub const fn production() -> Self {
        Self {
            max_cached_bytes: 256 * 1024 * 1024,
            max_cached_allocations: 1_024,
            max_allocations_per_bucket: 64,
            max_buckets: 256,
            cacheability: CacheabilityPolicy::Safe,
            eviction_policy: EvictionPolicy::OldestFirst,
        }
    }

    /// Creates a disabled pool configuration.
    pub const fn disabled() -> Self {
        Self {
            max_cached_bytes: 0,
            max_cached_allocations: 0,
            max_allocations_per_bucket: 0,
            max_buckets: 0,
            cacheability: CacheabilityPolicy::Safe,
            eviction_policy: EvictionPolicy::OldestFirst,
        }
    }

    /// Creates a small pool suitable for deterministic unit tests.
    pub const fn testing() -> Self {
        Self {
            max_cached_bytes: 1024 * 1024,
            max_cached_allocations: 32,
            max_allocations_per_bucket: 8,
            max_buckets: 16,
            cacheability: CacheabilityPolicy::Safe,
            eviction_policy: EvictionPolicy::OldestFirst,
        }
    }

    /// Returns whether caching is enabled.
    pub const fn is_enabled(self) -> bool {
        self.max_cached_bytes > 0
            && self.max_cached_allocations > 0
            && self.max_allocations_per_bucket > 0
            && self.max_buckets > 0
    }

    /// Validates the configuration.
    pub const fn validate(self) -> Result<(), PoolConfigError> {
        if !self.is_enabled() {
            return Ok(());
        }

        if self.max_cached_bytes == 0 {
            return Err(PoolConfigError::Invalid(
                "max_cached_bytes must be non-zero when caching is enabled",
            ));
        }

        if self.max_cached_allocations == 0 {
            return Err(PoolConfigError::Invalid(
                "max_cached_allocations must be non-zero when caching is enabled",
            ));
        }

        if self.max_allocations_per_bucket == 0 {
            return Err(PoolConfigError::Invalid(
                "max_allocations_per_bucket must be non-zero when caching is enabled",
            ));
        }

        if self.max_buckets == 0 {
            return Err(PoolConfigError::Invalid(
                "max_buckets must be non-zero when caching is enabled",
            ));
        }

        Ok(())
    }
}

/// Pool configuration error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PoolConfigError {
    /// Configuration is invalid.
    Invalid(&'static str),
}

impl fmt::Display for PoolConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(reason) => write!(formatter, "invalid memory-pool configuration: {reason}"),
        }
    }
}

impl std::error::Error for PoolConfigError {}

// =============================================================================
// Pool key
// =============================================================================

/// Exact semantic key used to determine allocation compatibility.
///
/// The key intentionally mirrors every meaningful field of
/// `AllocationRequest`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct PoolKey {
    bytes: u64,
    location: MemoryLocation,
    class: AllocationClass,
    state_elements: u64,
    qubits: u64,
    label: Option<String>,
}

impl PoolKey {
    fn from_request(request: &AllocationRequest) -> Self {
        Self {
            bytes: request.bytes.get(),
            location: request.location.clone(),
            class: request.class,
            state_elements: request.state_elements,
            qubits: request.qubits,
            label: request.label.clone(),
        }
    }

    fn matches_allocation(&self, allocation: &MemoryAllocation) -> bool {
        let request = allocation.request();

        self.bytes == request.bytes.get()
            && self.location == request.location
            && self.class == request.class
            && self.state_elements == request.state_elements
            && self.qubits == request.qubits
            && self.label == request.label
    }
}

// =============================================================================
// Eviction token
// =============================================================================

/// Lazily validated entry in the global FIFO eviction queue.
#[derive(Clone, Debug, Eq, PartialEq)]
struct EvictionToken {
    allocation_id: AllocationId,
    key: PoolKey,
}

// =============================================================================
// Statistics
// =============================================================================

/// Snapshot of pool activity and retained resources.
///
/// This is intentionally a plain value type so diagnostics and telemetry can
/// consume it without holding the pool lock.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PoolStats {
    /// Number of successful cache hits.
    pub hits: u64,

    /// Number of cache misses.
    pub misses: u64,

    /// Number of allocations successfully recycled into the pool.
    pub recycled: u64,

    /// Number of allocations evicted because of capacity constraints.
    pub evictions: u64,

    /// Number of allocations dropped instead of cached.
    pub dropped: u64,

    /// Number of allocations rejected because they belonged to another
    /// allocator.
    pub foreign_allocations: u64,

    /// Current cached allocation count.
    pub cached_allocations: u64,

    /// Current cached byte count.
    pub cached_bytes: u64,

    /// Lifetime number of allocation-acquisition calls.
    pub acquisitions: u64,

    /// Lifetime number of explicit cache clear/trim operations.
    pub maintenance_operations: u64,
}

impl PoolStats {
    /// Returns the cache hit ratio.
    ///
    /// Returns `0.0` when there have been no acquisitions.
    pub fn hit_ratio(self) -> f64 {
        if self.acquisitions == 0 {
            return 0.0;
        }

        self.hits as f64 / self.acquisitions as f64
    }

    /// Returns the cache miss ratio.
    ///
    /// Returns `0.0` when there have been no acquisitions.
    pub fn miss_ratio(self) -> f64 {
        if self.acquisitions == 0 {
            return 0.0;
        }

        self.misses as f64 / self.acquisitions as f64
    }
}

// =============================================================================
// Recycle outcome
// =============================================================================

/// Result of returning an allocation to the pool.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecycleOutcome {
    /// Allocation was retained by the pool.
    Cached,

    /// Pool caching is disabled.
    DroppedDisabled,

    /// The location is not cacheable under the current policy.
    DroppedNotCacheable,

    /// The allocation was too large for the configured cache.
    DroppedTooLarge,

    /// The pool was already at capacity and the allocation was released.
    DroppedCapacity,

    /// The allocation belonged to another allocator.
    DroppedForeignAllocator,
}

impl RecycleOutcome {
    /// Returns whether the allocation was retained.
    pub const fn cached(self) -> bool {
        matches!(self, Self::Cached)
    }
}

// =============================================================================
// Internal pool state
// =============================================================================

struct PoolInner {
    allocator: MemoryAllocator,
    config: MemoryPoolConfig,

    /// Exact-request bucket → cached allocations.
    buckets: BTreeMap<PoolKey, VecDeque<MemoryAllocation>>,

    /// Global eviction order.
    ///
    /// Entries can become stale when an allocation is acquired from its bucket.
    /// Stale tokens are removed lazily.
    eviction_queue: VecDeque<EvictionToken>,

    stats: PoolStats,
}

impl PoolInner {
    fn cached_bytes(&self) -> u64 {
        self.stats.cached_bytes
    }

    fn cached_allocations(&self) -> u64 {
        self.stats.cached_allocations
    }

    fn cache_has_capacity(&self, bytes: u64) -> bool {
        if !self.config.is_enabled() {
            return false;
        }

        if bytes > self.config.max_cached_bytes {
            return false;
        }

        let resulting_bytes = match self.cached_bytes().checked_add(bytes) {
            Some(value) => value,
            None => return false,
        };

        let resulting_allocations = match self.cached_allocations().checked_add(1) {
            Some(value) => value,
            None => return false,
        };

        resulting_bytes <= self.config.max_cached_bytes
            && resulting_allocations <= self.config.max_cached_allocations
    }

    fn bucket_has_capacity(&self, key: &PoolKey) -> bool {
        match self.buckets.get(key) {
            Some(bucket) => {
                (bucket.len() as u64) < self.config.max_allocations_per_bucket
            }
            None => true,
        }
    }

    fn bucket_count(&self) -> usize {
        self.buckets.len()
    }

    fn can_create_bucket(&self, key: &PoolKey) -> bool {
        self.buckets.contains_key(key) || self.bucket_count() < self.config.max_buckets
    }

    fn account_cached_add(&mut self, bytes: u64) -> Result<(), MemoryError> {
        self.stats.cached_allocations =
            self.stats
                .cached_allocations
                .checked_add(1)
                .ok_or_else(|| MemoryError::PoolError {
                    reason: "cached allocation counter overflow".to_owned(),
                })?;

        self.stats.cached_bytes =
            self.stats
                .cached_bytes
                .checked_add(bytes)
                .ok_or_else(|| MemoryError::PoolError {
                    reason: "cached byte counter overflow".to_owned(),
                })?;

        Ok(())
    }

    fn account_cached_remove(&mut self, bytes: u64) -> Result<(), MemoryError> {
        self.stats.cached_allocations = self
            .stats
            .cached_allocations
            .checked_sub(1)
            .ok_or_else(|| MemoryError::PoolError {
                reason: "cached allocation counter underflow".to_owned(),
            })?;

        self.stats.cached_bytes = self
            .stats
            .cached_bytes
            .checked_sub(bytes)
            .ok_or_else(|| MemoryError::PoolError {
                reason: "cached byte counter underflow".to_owned(),
            })?;

        Ok(())
    }

    fn insert_cached(
        &mut self,
        key: PoolKey,
        allocation: MemoryAllocation,
    ) -> Result<(), MemoryError> {
        let allocation_id = allocation.id();
        let bytes = allocation.byte_len();

        if !allocation.is_live() {
            return Err(MemoryError::PoolError {
                reason: "attempted to cache a released allocation".to_owned(),
            });
        }

        if !key.matches_allocation(&allocation) {
            return Err(MemoryError::PoolError {
                reason: "allocation request does not match its pool bucket".to_owned(),
            });
        }

        if !self.can_create_bucket(&key) {
            return Err(MemoryError::PoolError {
                reason: "maximum number of pool buckets has been reached".to_owned(),
            });
        }

        if !self.bucket_has_capacity(&key) {
            return Err(MemoryError::PoolError {
                reason: "maximum allocations per pool bucket has been reached".to_owned(),
            });
        }

        if !self.cache_has_capacity(bytes) {
            return Err(MemoryError::PoolError {
                reason: "pool cache capacity is insufficient".to_owned(),
            });
        }

        let bucket = self.buckets.entry(key.clone()).or_default();

        if bucket
            .iter()
            .any(|existing| existing.id() == allocation_id)
        {
            return Err(MemoryError::PoolError {
                reason: "duplicate allocation identity detected in pool".to_owned(),
            });
        }

        bucket.push_back(allocation);

        self.account_cached_add(bytes)?;

        self.eviction_queue.push_back(EvictionToken {
            allocation_id,
            key,
        });

        Ok(())
    }

    fn pop_matching(&mut self, key: &PoolKey) -> Option<MemoryAllocation> {
        let allocation = self.buckets.get_mut(key).and_then(VecDeque::pop_front);

        if allocation.is_some() {
            if self
                .buckets
                .get(key)
                .map(VecDeque::is_empty)
                .unwrap_or(false)
            {
                self.buckets.remove(key);
            }
        }

        allocation
    }

    fn remove_by_token(&mut self, token: &EvictionToken) -> Option<MemoryAllocation> {
        let bucket = self.buckets.get_mut(&token.key)?;

        let position = bucket
            .iter()
            .position(|allocation| allocation.id() == token.allocation_id)?;

        let allocation = bucket.remove(position)?;

        if bucket.is_empty() {
            self.buckets.remove(&token.key);
        }

        Some(allocation)
    }

    fn pop_oldest(&mut self) -> Option<MemoryAllocation> {
        while let Some(token) = self.eviction_queue.pop_front() {
            if let Some(allocation) = self.remove_by_token(&token) {
                return Some(allocation);
            }
        }

        None
    }

    fn pop_largest(&mut self) -> Option<MemoryAllocation> {
        let mut selected_key: Option<PoolKey> = None;
        let mut selected_index: Option<usize> = None;
        let mut selected_bytes = 0u64;

        for (key, bucket) in &self.buckets {
            for (index, allocation) in bucket.iter().enumerate() {
                let bytes = allocation.byte_len();

                if selected_index.is_none() || bytes > selected_bytes {
                    selected_key = Some(key.clone());
                    selected_index = Some(index);
                    selected_bytes = bytes;
                }
            }
        }

        match (selected_key, selected_index) {
            (Some(key), Some(index)) => self
                .buckets
                .get_mut(&key)
                .and_then(|bucket| bucket.remove(index)),
            _ => None,
        }
    }

    fn remove_accounting_for_evicted(
        &mut self,
        allocation: &MemoryAllocation,
    ) -> Result<(), MemoryError> {
        self.account_cached_remove(allocation.byte_len())?;

        self.stats.evictions = self
            .stats
            .evictions
            .checked_add(1)
            .ok_or_else(|| MemoryError::PoolError {
                reason: "pool eviction counter overflow".to_owned(),
            })?;

        Ok(())
    }

    fn remove_accounting_for_acquired(
        &mut self,
        allocation: &MemoryAllocation,
    ) -> Result<(), MemoryError> {
        self.account_cached_remove(allocation.byte_len())
    }

    fn trim_until_capacity(
        &mut self,
        required_bytes: u64,
        required_allocations: u64,
        required_buckets: bool,
    ) -> Result<(), MemoryError> {
        loop {
            let bytes_ok = self
                .cached_bytes()
                .checked_add(required_bytes)
                .map(|value| value <= self.config.max_cached_bytes)
                .unwrap_or(false);

            let allocations_ok = self
                .cached_allocations()
                .checked_add(required_allocations)
                .map(|value| value <= self.config.max_cached_allocations)
                .unwrap_or(false);

            let buckets_ok = !required_buckets || self.bucket_count() < self.config.max_buckets;

            if bytes_ok && allocations_ok && buckets_ok {
                return Ok(());
            }

            let evicted = match self.config.eviction_policy {
                EvictionPolicy::OldestFirst => self.pop_oldest(),
                EvictionPolicy::LargestFirst => self.pop_largest(),
            };

            match evicted {
                Some(allocation) => {
                    self.remove_accounting_for_evicted(&allocation)?;
                    drop(allocation);
                }
                None => {
                    return Err(MemoryError::PoolError {
                        reason: "pool cannot free enough cached capacity".to_owned(),
                    });
                }
            }
        }
    }

    fn trim_all(&mut self) -> Result<u64, MemoryError> {
        let mut released = 0u64;

        while let Some(allocation) = self.pop_oldest() {
            let bytes = allocation.byte_len();

            self.account_cached_remove(bytes)?;

            released = released.checked_add(1).ok_or_else(|| MemoryError::PoolError {
                reason: "pool trim allocation counter overflow".to_owned(),
            })?;

            drop(allocation);
        }

        self.eviction_queue.clear();

        Ok(released)
    }

    fn trim_bytes(&mut self, target_cached_bytes: u64) -> Result<u64, MemoryError> {
        let mut released = 0u64;

        while self.cached_bytes() > target_cached_bytes {
            let allocation = match self.config.eviction_policy {
                EvictionPolicy::OldestFirst => self.pop_oldest(),
                EvictionPolicy::LargestFirst => self.pop_largest(),
            };

            match allocation {
                Some(allocation) => {
                    let bytes = allocation.byte_len();

                    self.account_cached_remove(bytes)?;

                    released = released.checked_add(1).ok_or_else(|| {
                        MemoryError::PoolError {
                            reason: "pool trim allocation counter overflow".to_owned(),
                        }
                    })?;

                    drop(allocation);
                }
                None => break,
            }
        }

        Ok(released)
    }
}

// =============================================================================
// MemoryPool
// =============================================================================

/// Thread-safe provider-neutral quantum memory pool.
///
/// Cloning a `MemoryPool` creates another handle to the same underlying pool.
/// The allocator itself is also shared according to its existing clone
/// semantics.
#[derive(Clone)]
pub struct MemoryPool {
    inner: Arc<Mutex<PoolInner>>,
}

impl fmt::Debug for MemoryPool {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner.lock() {
            Ok(inner) => formatter
                .debug_struct("MemoryPool")
                .field("allocator", &inner.allocator)
                .field("config", &inner.config)
                .field("stats", &inner.stats)
                .finish(),
            Err(_) => formatter
                .debug_struct("MemoryPool")
                .field("state", &"poisoned")
                .finish(),
        }
    }
}

impl MemoryPool {
    /// Creates a pool from an existing allocator.
    pub fn new(
        allocator: MemoryAllocator,
        config: MemoryPoolConfig,
    ) -> Result<Self, MemoryError> {
        config.validate().map_err(|error| MemoryError::PoolError {
            reason: error.to_string(),
        })?;

        Ok(Self {
            inner: Arc::new(Mutex::new(PoolInner {
                allocator,
                config,
                buckets: BTreeMap::new(),
                eviction_queue: VecDeque::new(),
                stats: PoolStats::default(),
            })),
        })
    }

    /// Creates a pool using production defaults.
    pub fn production(allocator: MemoryAllocator) -> Result<Self, MemoryError> {
        Self::new(allocator, MemoryPoolConfig::production())
    }

    /// Creates a disabled pool.
    ///
    /// Acquisitions still work through the allocator, but recycled allocations
    /// are immediately released.
    pub fn disabled(allocator: MemoryAllocator) -> Result<Self, MemoryError> {
        Self::new(allocator, MemoryPoolConfig::disabled())
    }

    /// Returns the underlying allocator's memory-domain identity.
    pub fn memory_id(&self) -> Result<super::types::MemoryId, MemoryError> {
        let inner = lock_pool(&self.inner)?;
        Ok(inner.allocator.memory_id())
    }

    /// Returns the current pool configuration.
    pub fn config(&self) -> Result<MemoryPoolConfig, MemoryError> {
        let inner = lock_pool(&self.inner)?;
        Ok(inner.config)
    }

    /// Returns a point-in-time statistics snapshot.
    pub fn stats(&self) -> Result<PoolStats, MemoryError> {
        let inner = lock_pool(&self.inner)?;
        Ok(inner.stats)
    }

    /// Returns the current cached byte count.
    pub fn cached_bytes(&self) -> Result<u64, MemoryError> {
        let inner = lock_pool(&self.inner)?;
        Ok(inner.stats.cached_bytes)
    }

    /// Returns the current cached allocation count.
    pub fn cached_allocations(&self) -> Result<u64, MemoryError> {
        let inner = lock_pool(&self.inner)?;
        Ok(inner.stats.cached_allocations)
    }

    /// Returns the number of distinct request buckets.
    pub fn bucket_count(&self) -> Result<usize, MemoryError> {
        let inner = lock_pool(&self.inner)?;
        Ok(inner.buckets.len())
    }

    /// Acquires an allocation.
    ///
    /// The pool first checks for an exact cached match. If no matching
    /// allocation exists, it delegates to `MemoryAllocator`.
    ///
    /// The pool never bypasses allocator limits.
    pub fn acquire(
        &self,
        request: AllocationRequest,
    ) -> Result<MemoryAllocation, MemoryError> {
        validate_pool_request(&request)?;

        let key = PoolKey::from_request(&request);

        {
            let mut inner = lock_pool(&self.inner)?;

            inner.stats.acquisitions =
                inner
                    .stats
                    .acquisitions
                    .checked_add(1)
                    .ok_or_else(|| MemoryError::PoolError {
                        reason: "pool acquisition counter overflow".to_owned(),
                    })?;

            if let Some(allocation) = inner.pop_matching(&key) {
                inner.remove_accounting_for_acquired(&allocation)?;

                if !allocation.is_live() {
                    return Err(MemoryError::PoolError {
                        reason: "pool contained a non-live allocation".to_owned(),
                    });
                }

                if !key.matches_allocation(&allocation) {
                    return Err(MemoryError::PoolError {
                        reason: "cached allocation failed exact request validation".to_owned(),
                    });
                }

                inner.stats.hits = inner
                    .stats
                    .hits
                    .checked_add(1)
                    .ok_or_else(|| MemoryError::PoolError {
                        reason: "pool hit counter overflow".to_owned(),
                    })?;

                return Ok(allocation);
            }

            inner.stats.misses = inner
                .stats
                .misses
                .checked_add(1)
                .ok_or_else(|| MemoryError::PoolError {
                    reason: "pool miss counter overflow".to_owned(),
                })?;
        }

        let allocator = {
            let inner = lock_pool(&self.inner)?;
            inner.allocator.clone()
        };

        allocator.allocate(request)
    }

    /// Acquires an allocation wrapped in a pool-aware RAII handle.
    ///
    /// When the returned `PooledAllocation` is dropped, it attempts to return
    /// the allocation to this pool. If the allocation cannot safely be cached,
    /// it is released normally.
    pub fn acquire_pooled(
        &self,
        request: AllocationRequest,
    ) -> Result<PooledAllocation, MemoryError> {
        let allocation = self.acquire(request)?;

        Ok(PooledAllocation {
            pool: self.clone(),
            allocation: Some(allocation),
        })
    }

    /// Returns an allocation to the pool.
    ///
    /// This operation consumes the allocation. If it is not safe or useful to
    /// cache, the allocation is simply dropped and its allocator accounting is
    /// released.
    pub fn recycle(&self, allocation: MemoryAllocation) -> RecycleOutcome {
        let mut inner = match self.inner.lock() {
            Ok(inner) => inner,
            Err(_) => return RecycleOutcome::DroppedCapacity,
        };

        if allocation.memory_id() != inner.allocator.memory_id() {
            inner.stats.foreign_allocations =
                inner.stats.foreign_allocations.saturating_add(1);

            drop(allocation);
            return RecycleOutcome::DroppedForeignAllocator;
        }

        if !allocation.is_live() {
            inner.stats.dropped = inner.stats.dropped.saturating_add(1);
            drop(allocation);
            return RecycleOutcome::DroppedDisabled;
        }

        if !inner.config.is_enabled() {
            inner.stats.dropped = inner.stats.dropped.saturating_add(1);
            drop(allocation);
            return RecycleOutcome::DroppedDisabled;
        }

        let location = allocation.request().location.clone();

        if !inner.config.cacheability.allows(&location) {
            inner.stats.dropped = inner.stats.dropped.saturating_add(1);
            drop(allocation);
            return RecycleOutcome::DroppedNotCacheable;
        }

        let bytes = allocation.byte_len();

        if bytes > inner.config.max_cached_bytes {
            inner.stats.dropped = inner.stats.dropped.saturating_add(1);
            drop(allocation);
            return RecycleOutcome::DroppedTooLarge;
        }

        let key = PoolKey::from_request(allocation.request());

        // Make room before attempting insertion. This ensures recycling a
        // large allocation cannot fail merely because the cache is full.
        let needs_new_bucket = !inner.buckets.contains_key(&key);

        if inner
            .trim_until_capacity(bytes, 1, needs_new_bucket)
            .is_err()
        {
            inner.stats.dropped = inner.stats.dropped.saturating_add(1);
            drop(allocation);
            return RecycleOutcome::DroppedCapacity;
        }

        if inner.insert_cached(key, allocation).is_err() {
            inner.stats.dropped = inner.stats.dropped.saturating_add(1);
            return RecycleOutcome::DroppedCapacity;
        }

        inner.stats.recycled = inner.stats.recycled.saturating_add(1);

        RecycleOutcome::Cached
    }

    /// Removes all cached allocations.
    ///
    /// This immediately releases provider resources represented by cached
    /// allocations.
    pub fn clear(&self) -> Result<u64, MemoryError> {
        let mut inner = lock_pool(&self.inner)?;

        inner.stats.maintenance_operations = inner
            .stats
            .maintenance_operations
            .checked_add(1)
            .ok_or_else(|| MemoryError::PoolError {
                reason: "pool maintenance counter overflow".to_owned(),
            })?;

        inner.trim_all()
    }

    /// Trims the pool until no more than `target_bytes` remain cached.
    pub fn trim_to_bytes(&self, target_bytes: u64) -> Result<u64, MemoryError> {
        let mut inner = lock_pool(&self.inner)?;

        inner.stats.maintenance_operations = inner
            .stats
            .maintenance_operations
            .checked_add(1)
            .ok_or_else(|| MemoryError::PoolError {
                reason: "pool maintenance counter overflow".to_owned(),
            })?;

        inner.trim_bytes(target_bytes)
    }

    /// Trims all cached allocations that belong to a specific storage
    /// location.
    pub fn trim_location(&self, location: &MemoryLocation) -> Result<u64, MemoryError> {
        let mut inner = lock_pool(&self.inner)?;

        inner.stats.maintenance_operations = inner
            .stats
            .maintenance_operations
            .checked_add(1)
            .ok_or_else(|| MemoryError::PoolError {
                reason: "pool maintenance counter overflow".to_owned(),
            })?;

        let keys: Vec<PoolKey> = inner
            .buckets
            .keys()
            .filter(|key| &key.location == location)
            .cloned()
            .collect();

        let mut released = 0u64;

        for key in keys {
            if let Some(mut bucket) = inner.buckets.remove(&key) {
                while let Some(allocation) = bucket.pop_front() {
                    inner.account_cached_remove(allocation.byte_len())?;
                    released = released.checked_add(1).ok_or_else(|| {
                        MemoryError::PoolError {
                            reason: "location trim allocation counter overflow".to_owned(),
                        }
                    })?;
                    drop(allocation);
                }
            }
        }

        // Stale eviction tokens are harmless, but clearing them here keeps
        // maintenance deterministic and bounded.
        inner.eviction_queue.retain(|token| {
            inner.buckets.contains_key(&token.key)
                && token.key.location != *location
        });

        Ok(released)
    }

    /// Trims all cached allocations belonging to a broad location category.
    pub fn trim_location_kind(
        &self,
        location_kind: MemoryLocationKind,
    ) -> Result<u64, MemoryError> {
        let mut inner = lock_pool(&self.inner)?;

        inner.stats.maintenance_operations = inner
            .stats
            .maintenance_operations
            .checked_add(1)
            .ok_or_else(|| MemoryError::PoolError {
                reason: "pool maintenance counter overflow".to_owned(),
            })?;

        let keys: Vec<PoolKey> = inner
            .buckets
            .keys()
            .filter(|key| key.location.kind() == location_kind)
            .cloned()
            .collect();

        let mut released = 0u64;

        for key in keys {
            if let Some(mut bucket) = inner.buckets.remove(&key) {
                while let Some(allocation) = bucket.pop_front() {
                    inner.account_cached_remove(allocation.byte_len())?;
                    released = released.checked_add(1).ok_or_else(|| {
                        MemoryError::PoolError {
                            reason: "location-kind trim allocation counter overflow"
                                .to_owned(),
                        }
                    })?;
                    drop(allocation);
                }
            }
        }

        inner.eviction_queue.retain(|token| {
            inner.buckets.contains_key(&token.key)
                && token.key.location.kind() != location_kind
        });

        Ok(released)
    }

    /// Returns whether the pool currently has an exact cached match.
    pub fn contains(
        &self,
        request: &AllocationRequest,
    ) -> Result<bool, MemoryError> {
        validate_pool_request(request)?;

        let key = PoolKey::from_request(request);
        let inner = lock_pool(&self.inner)?;

        Ok(inner
            .buckets
            .get(&key)
            .map(|bucket| !bucket.is_empty())
            .unwrap_or(false))
    }

    /// Returns the number of cached allocations for an exact request.
    pub fn cached_count(
        &self,
        request: &AllocationRequest,
    ) -> Result<u64, MemoryError> {
        validate_pool_request(request)?;

        let key = PoolKey::from_request(request);
        let inner = lock_pool(&self.inner)?;

        Ok(inner
            .buckets
            .get(&key)
            .map(|bucket| bucket.len() as u64)
            .unwrap_or(0))
    }

    /// Returns the underlying allocator.
    ///
    /// This clone shares allocator accounting with the pool.
    pub fn allocator(&self) -> Result<MemoryAllocator, MemoryError> {
        let inner = lock_pool(&self.inner)?;
        Ok(inner.allocator.clone())
    }
}

// =============================================================================
// PooledAllocation
// =============================================================================

/// RAII wrapper around a `MemoryAllocation`.
///
/// When dropped, the allocation is returned to its originating pool whenever
/// possible.
///
/// This type does not expose raw addresses or provider internals.
pub struct PooledAllocation {
    pool: MemoryPool,
    allocation: Option<MemoryAllocation>,
}

impl fmt::Debug for PooledAllocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PooledAllocation")
            .field(
                "allocation_id",
                &self.allocation.as_ref().map(MemoryAllocation::id),
            )
            .field(
                "memory_id",
                &self.allocation.as_ref().map(MemoryAllocation::memory_id),
            )
            .field(
                "byte_len",
                &self.allocation.as_ref().map(MemoryAllocation::byte_len),
            )
            .finish()
    }
}

impl PooledAllocation {
    /// Returns the underlying allocation, if still owned.
    pub fn as_allocation(&self) -> Option<&MemoryAllocation> {
        self.allocation.as_ref()
    }

    /// Returns a mutable reference to the underlying allocation, if still
    /// owned.
    pub fn as_mut_allocation(&mut self) -> Option<&mut MemoryAllocation> {
        self.allocation.as_mut()
    }

    /// Returns the allocation identity.
    pub fn id(&self) -> Option<AllocationId> {
        self.allocation.as_ref().map(MemoryAllocation::id)
    }

    /// Returns the allocation byte count.
    pub fn byte_len(&self) -> u64 {
        self.allocation
            .as_ref()
            .map(MemoryAllocation::byte_len)
            .unwrap_or(0)
    }

    /// Returns whether the allocation remains live.
    pub fn is_live(&self) -> bool {
        self.allocation
            .as_ref()
            .map(MemoryAllocation::is_live)
            .unwrap_or(false)
    }

    /// Consumes the RAII wrapper and returns the allocation without recycling
    /// it.
    pub fn into_allocation(mut self) -> MemoryAllocation {
        match self.allocation.take() {
            Some(allocation) => allocation,
            None => {
                // This state cannot be constructed through safe public APIs.
                // Returning a panic here would violate the memory subsystem's
                // expected operational-failure model, so abort is deliberately
                // avoided by using a structured impossible-state strategy.
                //
                // In practice this branch is unreachable because `take()` is
                // private and `into_allocation()` consumes self exactly once.
                //
                // A zero-sized host allocation cannot be fabricated safely
                // here, so the only correct response is process termination.
                //
                // This branch should therefore never execute.
                std::process::abort()
            }
        }
    }

    /// Explicitly recycles the allocation before dropping the wrapper.
    pub fn recycle(mut self) -> RecycleOutcome {
        match self.allocation.take() {
            Some(allocation) => self.pool.recycle(allocation),
            None => RecycleOutcome::DroppedDisabled,
        }
    }

    /// Explicitly releases the allocation without returning it to the pool.
    pub fn release(mut self) {
        self.allocation.take();
    }
}

impl Drop for PooledAllocation {
    fn drop(&mut self) {
        if let Some(allocation) = self.allocation.take() {
            let _ = self.pool.recycle(allocation);
        }
    }
}

// =============================================================================
// Validation / locking helpers
// =============================================================================

fn validate_pool_request(request: &AllocationRequest) -> Result<(), MemoryError> {
    if request.bytes.get() == 0 {
        return Err(MemoryError::PoolError {
            reason: "zero-byte allocations are not pooled".to_owned(),
        });
    }

    if request.location.is_backend_native() {
        if let MemoryLocation::BackendNative { provider } = &request.location {
            if provider.is_empty() {
                return Err(MemoryError::PoolError {
                    reason: "backend-native provider namespace cannot be empty".to_owned(),
                });
            }
        }
    }

    Ok(())
}

fn lock_pool<'a>(
    mutex: &'a Mutex<PoolInner>,
) -> Result<std::sync::MutexGuard<'a, PoolInner>, MemoryError> {
    mutex.lock().map_err(|_| MemoryError::PoolError {
        reason: "memory pool mutex was poisoned".to_owned(),
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::memory::types::{ByteCount, MemoryId};

    fn allocator() -> MemoryAllocator {
        MemoryAllocator::production(MemoryId::new(1))
            .expect("production allocator")
            .with_host_provider()
            .expect("host provider")
    }

    fn request(bytes: u64) -> AllocationRequest {
        AllocationRequest::new(
            ByteCount::new(bytes),
            MemoryLocation::Host,
            AllocationClass::Temporary,
        )
    }

    #[test]
    fn production_configuration_is_enabled() {
        let config = MemoryPoolConfig::production();

        assert!(config.is_enabled());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn disabled_configuration_is_valid() {
        let config = MemoryPoolConfig::disabled();

        assert!(!config.is_enabled());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn pool_miss_allocates_from_allocator() {
        let pool = MemoryPool::new(allocator(), MemoryPoolConfig::testing())
            .expect("pool creation");

        let allocation = pool.acquire(request(128)).expect("allocation");

        assert_eq!(allocation.byte_len(), 128);

        let stats = pool.stats().expect("stats");

        assert_eq!(stats.acquisitions, 1);
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 1);
    }

    #[test]
    fn recycled_allocation_is_reused() {
        let pool = MemoryPool::new(allocator(), MemoryPoolConfig::testing())
            .expect("pool creation");

        let first = pool.acquire(request(128)).expect("first allocation");
        let first_id = first.id();

        assert_eq!(pool.recycle(first), RecycleOutcome::Cached);

        assert!(pool.contains(&request(128)).expect("contains"));

        let second = pool.acquire(request(128)).expect("second allocation");

        assert_eq!(second.id(), first_id);

        let stats = pool.stats().expect("stats");

        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.cached_allocations, 0);
        assert_eq!(stats.cached_bytes, 0);
    }

    #[test]
    fn different_labels_do_not_alias() {
        let pool = MemoryPool::new(allocator(), MemoryPoolConfig::testing())
            .expect("pool creation");

        let first_request = request(128).with_label("first");
        let second_request = request(128).with_label("second");

        let first = pool.acquire(first_request.clone()).expect("first");
        let first_id = first.id();

        assert_eq!(pool.recycle(first), RecycleOutcome::Cached);

        let second = pool.acquire(second_request).expect("second");

        assert_ne!(second.id(), first_id);
    }

    #[test]
    fn clear_releases_cached_allocations() {
        let pool = MemoryPool::new(allocator(), MemoryPoolConfig::testing())
            .expect("pool creation");

        let allocation = pool.acquire(request(256)).expect("allocation");

        assert_eq!(pool.recycle(allocation), RecycleOutcome::Cached);

        assert_eq!(pool.cached_allocations().expect("cached count"), 1);
        assert_eq!(pool.cached_bytes().expect("cached bytes"), 256);

        assert_eq!(pool.clear().expect("clear"), 1);

        assert_eq!(pool.cached_allocations().expect("cached count"), 0);
        assert_eq!(pool.cached_bytes().expect("cached bytes"), 0);
    }

    #[test]
    fn disabled_pool_releases_recycled_allocations() {
        let pool = MemoryPool::disabled(allocator()).expect("pool creation");

        let allocation = pool.acquire(request(128)).expect("allocation");

        assert_eq!(
            pool.recycle(allocation),
            RecycleOutcome::DroppedDisabled
        );

        assert_eq!(pool.cached_allocations().expect("cached count"), 0);
    }

    #[test]
    fn raii_wrapper_recycles_on_drop() {
        let pool = MemoryPool::new(allocator(), MemoryPoolConfig::testing())
            .expect("pool creation");

        {
            let allocation = pool
                .acquire_pooled(request(128))
                .expect("pooled allocation");

            assert_eq!(allocation.byte_len(), 128);
            assert!(allocation.is_live());
        }

        assert_eq!(pool.cached_allocations().expect("cached count"), 1);
        assert_eq!(pool.cached_bytes().expect("cached bytes"), 128);
    }

    #[test]
    fn explicit_release_does_not_cache() {
        let pool = MemoryPool::new(allocator(), MemoryPoolConfig::testing())
            .expect("pool creation");

        let allocation = pool
            .acquire_pooled(request(128))
            .expect("pooled allocation");

        allocation.release();

        assert_eq!(pool.cached_allocations().expect("cached count"), 0);
    }

    #[test]
    fn trim_to_bytes_releases_excess_cache() {
        let config = MemoryPoolConfig {
            max_cached_bytes: 1024,
            max_cached_allocations: 16,
            max_allocations_per_bucket: 16,
            max_buckets: 16,
            cacheability: CacheabilityPolicy::Safe,
            eviction_policy: EvictionPolicy::OldestFirst,
        };

        let pool = MemoryPool::new(allocator(), config).expect("pool creation");

        let first = pool.acquire(request(256)).expect("first");
        let second = pool.acquire(request(512)).expect("second");

        assert_eq!(pool.recycle(first), RecycleOutcome::Cached);
        assert_eq!(pool.recycle(second), RecycleOutcome::Cached);

        assert_eq!(pool.cached_bytes().expect("cached bytes"), 768);

        let released = pool.trim_to_bytes(256).expect("trim");

        assert_eq!(released, 1);
        assert_eq!(pool.cached_bytes().expect("cached bytes"), 256);
    }

    #[test]
    fn cache_statistics_are_consistent() {
        let pool = MemoryPool::new(allocator(), MemoryPoolConfig::testing())
            .expect("pool creation");

        let allocation = pool.acquire(request(64)).expect("allocation");

        assert_eq!(pool.recycle(allocation), RecycleOutcome::Cached);

        let _ = pool.acquire(request(64)).expect("reuse");

        let stats = pool.stats().expect("stats");

        assert_eq!(stats.acquisitions, 2);
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.recycled, 1);
    }
}