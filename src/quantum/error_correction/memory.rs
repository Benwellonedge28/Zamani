//! Production-grade memory management for Zamani QEC.
//!
//! This module is the memory/allocation boundary for the quantum-error-
//! correction subsystem.
//!
//! # Architectural contract
//!
//! ```text
//!                    QecConfig
//!                       │
//!                       ▼
//!                   QecLimits
//!                       │
//!             ┌─────────┴─────────┐
//!             │                   │
//!             ▼                   ▼
//!         Preflight         MemoryManager
//!                                 │
//!                 ┌───────────────┼────────────────┐
//!                 │               │                │
//!                 ▼               ▼                ▼
//!             Reservation     Scope/Quota       Buffers
//!                 │               │                │
//!                 ├───────────────┼────────────────┤
//!                 │               │                │
//!                 ▼               ▼                ▼
//!              Arena       SparseAllocation    Streaming
//!                 │               │                │
//!                 └───────────────┼────────────────┘
//!                                 ▼
//!                         MemorySnapshot
//! ```
//!
//! # Design principles
//!
//! * `QecLimits` is the canonical declarative policy.
//! * This module does not create a second global resource policy.
//! * All public allocations are checked before capacity growth.
//! * Reservation accounting is atomic.
//! * RAII reservations cannot outlive their manager.
//! * Cloning a manager creates a shared accounting handle rather than a
//!   silently independent accounting universe.
//! * Cancellation is cooperative.
//! * Arithmetic is checked.
//! * Buffers are bounded.
//! * Sparse allocations are explicitly accounted.
//! * Arena growth is transactional.
//! * Eviction accounting never claims memory was reclaimed until the caller
//!   actually releases the corresponding reservation.
//! * `unsafe` is forbidden.
//!
//! "Unlimited" means that this module imposes no additional application-level
//! memory ceiling. It does not imply infinite physical memory.
//!
//! # Integration
//!
//! The canonical production path is:
//!
//! ```text
//! QecConfig
//!     ↓
//! QecLimits
//!     ↓
//! MemoryBudget::from_qec_limits()
//!     ↓
//! MemoryManager
//!     ↓
//! allocation / reservation
//! ```
//!
//! Runtime consumption should additionally be reported to `resources.rs`
//! through the surrounding `ResourceManager`. This module deliberately owns
//! memory accounting only.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::VecDeque;
use std::fmt;
use std::marker::PhantomData;
use std::mem::size_of;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};

use super::limits::QecLimits;

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------

/// Explicit application-level unlimited-memory sentinel.
pub const UNLIMITED_MEMORY: u64 = u64::MAX;

/// Legacy/default standalone memory budget.
///
/// Production QEC execution should prefer
/// `MemoryBudget::from_qec_limits(&limits)`.
pub const DEFAULT_MEMORY_BUDGET: u64 = 1024 * 1024 * 1024;

/// Default bounded-buffer capacity.
///
/// Production QEC execution should prefer the value derived from
/// `QecLimits::max_stream_buffer_events`.
pub const DEFAULT_BUFFER_CAPACITY: usize = 4096;

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Errors produced by the memory subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryError {
    /// A memory policy is invalid.
    InvalidBudget {
        reason: &'static str,
    },

    /// Requested memory exceeds the global budget.
    BudgetExceeded {
        requested: u64,
        allocated: u64,
        available: u64,
        limit: u64,
    },

    /// A scoped/per-operation quota was exceeded.
    QuotaExceeded {
        requested: u64,
        reserved: u64,
        quota: u64,
    },

    /// Memory arithmetic overflowed.
    ArithmeticOverflow,

    /// A collection capacity cannot be represented safely.
    CapacityOverflow,

    /// A bounded buffer is full.
    BufferFull {
        capacity: usize,
    },

    /// A bounded buffer is empty.
    BufferEmpty,

    /// An operation is invalid.
    InvalidOperation {
        reason: &'static str,
    },

    /// The operation was cancelled.
    Cancelled,

    /// An eviction request could not be satisfied.
    EvictionInsufficient {
        requested: u64,
        reclaimed: u64,
    },
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBudget { reason } => {
                write!(f, "invalid memory budget: {reason}")
            }

            Self::BudgetExceeded {
                requested,
                allocated,
                available,
                limit,
            } => {
                write!(
                    f,
                    "memory budget exceeded: requested={requested}, \
                     allocated={allocated}, available={available}, \
                     limit={limit}"
                )
            }

            Self::QuotaExceeded {
                requested,
                reserved,
                quota,
            } => {
                write!(
                    f,
                    "memory quota exceeded: requested={requested}, \
                     reserved={reserved}, quota={quota}"
                )
            }

            Self::ArithmeticOverflow => {
                f.write_str("memory accounting arithmetic overflow")
            }

            Self::CapacityOverflow => {
                f.write_str("requested collection capacity overflow")
            }

            Self::BufferFull { capacity } => {
                write!(f, "bounded buffer is full: capacity={capacity}")
            }

            Self::BufferEmpty => {
                f.write_str("bounded buffer is empty")
            }

            Self::InvalidOperation { reason } => {
                write!(f, "invalid memory operation: {reason}")
            }

            Self::Cancelled => {
                f.write_str("memory operation cancelled")
            }

            Self::EvictionInsufficient {
                requested,
                reclaimed,
            } => {
                write!(
                    f,
                    "eviction reclaimed insufficient memory: \
                     requested={requested}, reclaimed={reclaimed}"
                )
            }
        }
    }
}

impl std::error::Error for MemoryError {}

// -----------------------------------------------------------------------------
// Memory budget
// -----------------------------------------------------------------------------

/// Memory-specific view derived from the canonical `QecLimits`.
///
/// This is intentionally narrower than `QecLimits`.
///
/// `QecLimits` remains the source of truth for the complete QEC execution
/// policy. `MemoryBudget` only carries information required by memory
/// primitives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryBudget {
    /// Maximum simultaneously reserved bytes.
    pub max_bytes: u64,

    /// Optional per-operation memory quota.
    pub max_operation_bytes: Option<u64>,

    /// Optional arena-specific memory limit.
    pub max_arena_bytes: Option<u64>,

    /// Maximum number of elements in a bounded buffer.
    pub max_buffer_elements: usize,

    /// Whether eviction may be requested by higher-level components.
    pub eviction_enabled: bool,
}

impl Default for MemoryBudget {
    fn default() -> Self {
        Self {
            max_bytes: DEFAULT_MEMORY_BUDGET,
            max_operation_bytes: None,
            max_arena_bytes: None,
            max_buffer_elements: DEFAULT_BUFFER_CAPACITY,
            eviction_enabled: true,
        }
    }
}

impl MemoryBudget {
    /// Creates an explicitly unlimited application-level memory budget.
    pub const fn unlimited() -> Self {
        Self {
            max_bytes: UNLIMITED_MEMORY,
            max_operation_bytes: None,
            max_arena_bytes: None,
            max_buffer_elements: usize::MAX,
            eviction_enabled: true,
        }
    }

    /// Derives the memory policy from canonical QEC limits.
    #[must_use]
    pub const fn from_qec_limits(limits: &QecLimits) -> Self {
        Self {
            max_bytes: limits.max_memory_bytes,
            max_operation_bytes: Some(limits.max_memory_bytes),
            max_arena_bytes: Some(limits.max_memory_bytes),
            max_buffer_elements: limits.max_stream_buffer_events,
            eviction_enabled: true,
        }
    }

    /// Validates the memory policy.
    pub fn validate(&self) -> Result<(), MemoryError> {
        if self.max_bytes == 0 {
            return Err(MemoryError::InvalidBudget {
                reason: "max_bytes must be greater than zero",
            });
        }

        if self.max_operation_bytes == Some(0) {
            return Err(MemoryError::InvalidBudget {
                reason: "max_operation_bytes must be greater than zero",
            });
        }

        if self.max_arena_bytes == Some(0) {
            return Err(MemoryError::InvalidBudget {
                reason: "max_arena_bytes must be greater than zero",
            });
        }

        if self.max_buffer_elements == 0 {
            return Err(MemoryError::InvalidBudget {
                reason: "max_buffer_elements must be greater than zero",
            });
        }

        if let Some(operation) = self.max_operation_bytes {
            if operation > self.max_bytes {
                return Err(MemoryError::InvalidBudget {
                    reason:
                        "operation memory limit cannot exceed global memory limit",
                });
            }
        }

        if let Some(arena) = self.max_arena_bytes {
            if arena > self.max_bytes {
                return Err(MemoryError::InvalidBudget {
                    reason:
                        "arena memory limit cannot exceed global memory limit",
                });
            }
        }

        Ok(())
    }

    /// Whether the application-level global memory policy is unlimited.
    pub const fn is_unlimited(&self) -> bool {
        self.max_bytes == UNLIMITED_MEMORY
    }
}

// -----------------------------------------------------------------------------
// Memory statistics
// -----------------------------------------------------------------------------

/// Immutable memory statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemorySnapshot {
    /// Currently reserved bytes.
    pub allocated_bytes: u64,

    /// Highest observed simultaneous reservation.
    pub peak_bytes: u64,

    /// Number of successful reservation operations.
    pub allocation_count: u64,

    /// Number of releases.
    pub release_count: u64,

    /// Number of rejected reservations.
    pub failed_allocations: u64,

    /// Total bytes successfully reserved over the manager lifetime.
    pub cumulative_allocated_bytes: u64,

    /// Total bytes released over the manager lifetime.
    pub cumulative_released_bytes: u64,

    /// Number of eviction requests reported.
    pub eviction_requests: u64,

    /// Bytes reported as reclaimed by eviction.
    ///
    /// This is informational. It does not itself change
    /// `allocated_bytes`.
    pub evicted_bytes: u64,
}

#[derive(Debug, Default)]
struct MemoryCounters {
    allocated_bytes: AtomicU64,
    peak_bytes: AtomicU64,
    allocation_count: AtomicU64,
    release_count: AtomicU64,
    failed_allocations: AtomicU64,
    cumulative_allocated_bytes: AtomicU64,
    cumulative_released_bytes: AtomicU64,
    eviction_requests: AtomicU64,
    evicted_bytes: AtomicU64,
}

/// Shared, thread-safe memory accounting state.
#[derive(Debug)]
struct MemoryState {
    budget: MemoryBudget,
    counters: MemoryCounters,
    cancelled: AtomicBool,
}

/// Thread-safe memory manager.
///
/// Cloning a `MemoryManager` creates another handle to the same accounting
/// state. This is critical: cloning must never silently reset resource
/// accounting.
#[derive(Debug, Clone)]
pub struct MemoryManager {
    state: Arc<MemoryState>,
}

impl MemoryManager {
    /// Creates a memory manager from a memory-specific policy.
    pub fn new(budget: MemoryBudget) -> Result<Self, MemoryError> {
        budget.validate()?;

        Ok(Self {
            state: Arc::new(MemoryState {
                budget,
                counters: MemoryCounters::default(),
                cancelled: AtomicBool::new(false),
            }),
        })
    }

    /// Creates a memory manager directly from canonical QEC limits.
    pub fn from_qec_limits(
        limits: &QecLimits,
    ) -> Result<Self, MemoryError> {
        limits
            .validate()
            .map_err(|_| MemoryError::InvalidBudget {
                reason: "invalid canonical QEC resource policy",
            })?;

        Self::new(MemoryBudget::from_qec_limits(limits))
    }

    /// Returns the memory policy.
    pub const fn budget(&self) -> MemoryBudget {
        self.state.budget
    }

    /// Requests cancellation of future memory operations.
    pub fn cancel(&self) {
        self.state.cancelled.store(true, Ordering::Release);
    }

    /// Clears cancellation before starting a new logical operation.
    ///
    /// Higher-level schedulers should generally prefer creating a new
    /// operation context rather than globally resetting shared cancellation.
    pub fn reset_cancellation(&self) {
        self.state.cancelled.store(false, Ordering::Release);
    }

    /// Whether the manager is cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.state.cancelled.load(Ordering::Acquire)
    }

    /// Checks cooperative cancellation.
    pub fn check(&self) -> Result<(), MemoryError> {
        if self.is_cancelled() {
            Err(MemoryError::Cancelled)
        } else {
            Ok(())
        }
    }

    /// Returns a point-in-time snapshot.
    pub fn snapshot(&self) -> MemorySnapshot {
        MemorySnapshot {
            allocated_bytes: self
                .state
                .counters
                .allocated_bytes
                .load(Ordering::Acquire),

            peak_bytes: self
                .state
                .counters
                .peak_bytes
                .load(Ordering::Acquire),

            allocation_count: self
                .state
                .counters
                .allocation_count
                .load(Ordering::Acquire),

            release_count: self
                .state
                .counters
                .release_count
                .load(Ordering::Acquire),

            failed_allocations: self
                .state
                .counters
                .failed_allocations
                .load(Ordering::Acquire),

            cumulative_allocated_bytes: self
                .state
                .counters
                .cumulative_allocated_bytes
                .load(Ordering::Acquire),

            cumulative_released_bytes: self
                .state
                .counters
                .cumulative_released_bytes
                .load(Ordering::Acquire),

            eviction_requests: self
                .state
                .counters
                .eviction_requests
                .load(Ordering::Acquire),

            evicted_bytes: self
                .state
                .counters
                .evicted_bytes
                .load(Ordering::Acquire),
        }
    }

    /// Returns available global memory.
    pub fn available_bytes(&self) -> u64 {
        if self.budget().is_unlimited() {
            return u64::MAX;
        }

        self.budget()
            .max_bytes
            .saturating_sub(
                self.state
                    .counters
                    .allocated_bytes
                    .load(Ordering::Acquire),
            )
    }

    /// Whether the requested amount can currently fit.
    pub fn can_reserve(&self, bytes: u64) -> bool {
        if self.is_cancelled() {
            return false;
        }

        self.available_bytes() >= bytes
    }

    /// Reserves memory using the configured operation quota.
    pub fn reserve(
        &self,
        bytes: u64,
    ) -> Result<MemoryReservation, MemoryError> {
        self.reserve_with_quota(
            bytes,
            self.budget().max_operation_bytes,
        )
    }

    /// Reserves memory using an explicit quota.
    ///
    /// The quota may tighten the global budget but can never expand it.
    pub fn reserve_with_quota(
        &self,
        bytes: u64,
        quota: Option<u64>,
    ) -> Result<MemoryReservation, MemoryError> {
        self.check()?;

        if bytes == 0 {
            return Ok(MemoryReservation {
                manager: self.clone(),
                bytes: 0,
                active: true,
            });
        }

        if let Some(limit) = quota {
            if bytes > limit {
                self.record_failed_allocation();

                return Err(MemoryError::QuotaExceeded {
                    requested: bytes,
                    reserved: 0,
                    quota: limit,
                });
            }
        }

        self.try_reserve_atomic(bytes)?;

        Ok(MemoryReservation {
            manager: self.clone(),
            bytes,
            active: true,
        })
    }

    /// Atomically reserves bytes against the global memory ceiling.
    fn try_reserve_atomic(
        &self,
        bytes: u64,
    ) -> Result<(), MemoryError> {
        loop {
            self.check()?;

            let current = self
                .state
                .counters
                .allocated_bytes
                .load(Ordering::Acquire);

            let next = current
                .checked_add(bytes)
                .ok_or_else(|| {
                    self.record_failed_allocation();
                    MemoryError::ArithmeticOverflow
                })?;

            if !self.budget().is_unlimited()
                && next > self.budget().max_bytes
            {
                self.record_failed_allocation();

                return Err(MemoryError::BudgetExceeded {
                    requested: bytes,
                    allocated: current,
                    available: self
                        .budget()
                        .max_bytes
                        .saturating_sub(current),
                    limit: self.budget().max_bytes,
                });
            }

            match self
                .state
                .counters
                .allocated_bytes
                .compare_exchange_weak(
                    current,
                    next,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
            {
                Ok(_) => {
                    self.state
                        .counters
                        .allocation_count
                        .fetch_add(1, Ordering::Relaxed);

                    self.state
                        .counters
                        .cumulative_allocated_bytes
                        .fetch_add(bytes, Ordering::Relaxed);

                    update_peak(
                        &self.state.counters.peak_bytes,
                        next,
                    );

                    return Ok(());
                }

                Err(_) => {
                    std::hint::spin_loop();
                }
            }
        }
    }

    /// Releases bytes previously reserved by this manager.
    pub fn release(&self, bytes: u64) {
        if bytes == 0 {
            return;
        }

        let mut current = self
            .state
            .counters
            .allocated_bytes
            .load(Ordering::Acquire);

        loop {
            let released = bytes.min(current);
            let next = current.saturating_sub(released);

            match self
                .state
                .counters
                .allocated_bytes
                .compare_exchange_weak(
                    current,
                    next,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
            {
                Ok(_) => {
                    self.state
                        .counters
                        .release_count
                        .fetch_add(1, Ordering::Relaxed);

                    self.state
                        .counters
                        .cumulative_released_bytes
                        .fetch_add(
                            released,
                            Ordering::Relaxed,
                        );

                    return;
                }

                Err(actual) => {
                    current = actual;
                    std::hint::spin_loop();
                }
            }
        }
    }

    /// Records an externally performed eviction.
    ///
    /// This records the event only. It deliberately does not reduce current
    /// allocations because only the owner of the corresponding reservation
    /// can safely release it.
    pub fn record_eviction(
        &self,
        reclaimed_bytes: u64,
    ) {
        self.state
            .counters
            .eviction_requests
            .fetch_add(1, Ordering::Relaxed);

        self.state
            .counters
            .evicted_bytes
            .fetch_add(
                reclaimed_bytes,
                Ordering::Relaxed,
            );
    }

    fn record_failed_allocation(&self) {
        self.state
            .counters
            .failed_allocations
            .fetch_add(1, Ordering::Relaxed);
    }
}

// -----------------------------------------------------------------------------
// RAII reservation
// -----------------------------------------------------------------------------

/// RAII memory reservation.
#[derive(Debug)]
pub struct MemoryReservation {
    manager: MemoryManager,
    bytes: u64,
    active: bool,
}

impl MemoryReservation {
    /// Reserved byte count.
    pub const fn bytes(&self) -> u64 {
        self.bytes
    }

    /// Whether the reservation is still active.
    pub const fn is_active(&self) -> bool {
        self.active
    }

    /// Explicitly releases the reservation.
    pub fn release(mut self) {
        if self.active {
            self.manager.release(self.bytes);
            self.active = false;
        }
    }

    /// Prevents automatic release and returns the byte count.
    ///
    /// This is intentionally explicit. Callers should use it only when
    /// ownership of the reservation is transferred to another component.
    pub fn into_bytes(mut self) -> u64 {
        self.active = false;
        self.bytes
    }
}

impl Drop for MemoryReservation {
    fn drop(&mut self) {
        if self.active {
            self.manager.release(self.bytes);
            self.active = false;
        }
    }
}

// -----------------------------------------------------------------------------
// Scoped operation
// -----------------------------------------------------------------------------

/// Operation-scoped memory accounting.
///
/// A scope cannot consume more than its quota even when global memory is
/// available.
#[derive(Debug)]
pub struct MemoryScope {
    manager: MemoryManager,
    quota: Option<u64>,
    reserved: u64,
}

impl MemoryScope {
    /// Creates a scoped memory context.
    pub fn new(
        manager: MemoryManager,
        quota: Option<u64>,
    ) -> Result<Self, MemoryError> {
        if quota == Some(0) {
            return Err(MemoryError::InvalidBudget {
                reason:
                    "memory scope quota must be greater than zero",
            });
        }

        if let Some(limit) = quota {
            if !manager.budget().is_unlimited()
                && limit > manager.budget().max_bytes
            {
                return Err(MemoryError::InvalidBudget {
                    reason:
                        "scope quota cannot exceed global memory budget",
                });
            }
        }

        Ok(Self {
            manager,
            quota,
            reserved: 0,
        })
    }

    /// Creates a scope using the manager's operation quota.
    pub fn from_manager(
        manager: MemoryManager,
    ) -> Result<Self, MemoryError> {
        let quota = manager.budget().max_operation_bytes;

        Self::new(manager, quota)
    }

    /// Returns the manager used by the scope.
    pub fn manager(&self) -> &MemoryManager {
        &self.manager
    }

    /// Reserves memory inside this operation.
    pub fn reserve(
        &mut self,
        bytes: u64,
    ) -> Result<ScopedReservation, MemoryError> {
        let next = self
            .reserved
            .checked_add(bytes)
            .ok_or(MemoryError::ArithmeticOverflow)?;

        if let Some(quota) = self.quota {
            if next > quota {
                return Err(MemoryError::QuotaExceeded {
                    requested: bytes,
                    reserved: self.reserved,
                    quota,
                });
            }
        }

        let reservation = self
            .manager
            .reserve_with_quota(bytes, self.quota)?;

        self.reserved = next;

        Ok(ScopedReservation {
            manager: self.manager.clone(),
            bytes,
            reservation: Some(reservation),
        })
    }

    /// Currently reserved bytes.
    pub const fn reserved_bytes(&self) -> u64 {
        self.reserved
    }

    /// Remaining scoped quota.
    pub fn remaining_bytes(&self) -> u64 {
        match self.quota {
            Some(quota) => quota.saturating_sub(self.reserved),
            None => self.manager.available_bytes(),
        }
    }

    /// Returns the scope quota.
    pub const fn quota(&self) -> Option<u64> {
        self.quota
    }
}

/// RAII reservation owned by a memory scope.
///
/// It owns the underlying global reservation independently, avoiding the
/// problematic mutable-reference lifetime coupling of the previous design.
#[derive(Debug)]
pub struct ScopedReservation {
    manager: MemoryManager,
    bytes: u64,
    reservation: Option<MemoryReservation>,
}

impl ScopedReservation {
    pub const fn bytes(&self) -> u64 {
        self.bytes
    }

    pub const fn is_active(&self) -> bool {
        self.reservation.is_some()
    }

    /// Explicitly releases the reservation.
    pub fn release(mut self) {
        self.release_internal();
    }

    fn release_internal(&mut self) {
        if let Some(reservation) = self.reservation.take() {
            reservation.release();
        }
    }
}

impl Drop for ScopedReservation {
    fn drop(&mut self) {
        self.release_internal();
    }
}

// -----------------------------------------------------------------------------
// Bounded buffer
// -----------------------------------------------------------------------------

/// Memory-accounted bounded FIFO.
///
/// The buffer provides memory backpressure and never grows beyond its
/// configured element capacity.
#[derive(Debug)]
pub struct BoundedBuffer<T> {
    manager: MemoryManager,
    queue: VecDeque<T>,
    capacity: usize,
    bytes_per_element: u64,
}

impl<T> BoundedBuffer<T> {
    /// Creates a bounded buffer.
    pub fn new(
        manager: MemoryManager,
        capacity: usize,
        bytes_per_element: u64,
    ) -> Result<Self, MemoryError> {
        if capacity == 0 {
            return Err(MemoryError::InvalidOperation {
                reason:
                    "buffer capacity must be greater than zero",
            });
        }

        if capacity > manager.budget().max_buffer_elements {
            return Err(MemoryError::InvalidOperation {
                reason:
                    "buffer capacity exceeds configured memory-buffer limit",
            });
        }

        if bytes_per_element == 0 {
            return Err(MemoryError::InvalidOperation {
                reason:
                    "bytes_per_element must be greater than zero",
            });
        }

        let queue_capacity_bytes =
            estimated_vecdeque_bytes::<T>(capacity)?;

        let _queue_capacity_reservation =
            manager.reserve(queue_capacity_bytes)?;

        /*
         * We intentionally do not retain the reservation for the empty
         * VecDeque. Its allocation can change as elements are inserted.
         *
         * Element reservations are the authoritative runtime accounting
         * mechanism. The initial capacity estimate is therefore used only
         * as a preflight guard rather than as permanently consumed memory.
         */
        drop(_queue_capacity_reservation);

        Ok(Self {
            manager,
            queue: VecDeque::with_capacity(capacity),
            capacity,
            bytes_per_element,
        })
    }

    /// Number of buffered elements.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Maximum element capacity.
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Whether the buffer is full.
    pub fn is_full(&self) -> bool {
        self.queue.len() >= self.capacity
    }

    /// Whether producer backpressure should be applied.
    pub fn needs_backpressure(&self) -> bool {
        self.is_full()
    }

    /// Pushes one element.
    pub fn push(
        &mut self,
        value: T,
    ) -> Result<(), MemoryError> {
        if self.is_full() {
            return Err(MemoryError::BufferFull {
                capacity: self.capacity,
            });
        }

        self.manager.check()?;

        let reservation =
            self.manager.reserve(self.bytes_per_element)?;

        if let Err(_) = self.queue.try_reserve(1) {
            reservation.release();

            return Err(MemoryError::InvalidOperation {
                reason:
                    "allocator rejected bounded-buffer growth",
            });
        }

        self.queue.push_back(value);

        Ok(())
    }

    /// Removes the oldest element.
    pub fn pop(&mut self) -> Result<T, MemoryError> {
        match self.queue.pop_front() {
            Some(value) => {
                self.manager.release(
                    self.bytes_per_element,
                );

                Ok(value)
            }

            None => Err(MemoryError::BufferEmpty),
        }
    }

    /// Clears all buffered elements.
    pub fn clear(&mut self) {
        let count = self.queue.len();

        self.queue.clear();

        let bytes =
            self.bytes_per_element
                .saturating_mul(count as u64);

        self.manager.release(bytes);
    }
}

impl<T> Drop for BoundedBuffer<T> {
    fn drop(&mut self) {
        let count = self.queue.len();

        let bytes =
            self.bytes_per_element
                .saturating_mul(count as u64);

        self.manager.release(bytes);
    }
}

// -----------------------------------------------------------------------------
// Streaming buffer
// -----------------------------------------------------------------------------

/// Explicit bounded streaming buffer.
///
/// Intended for syndrome and detection-event pipelines.
#[derive(Debug)]
pub struct StreamingBuffer<T> {
    inner: BoundedBuffer<T>,
}

impl<T> StreamingBuffer<T> {
    /// Creates a bounded stream buffer.
    pub fn new(
        manager: MemoryManager,
        capacity: usize,
        bytes_per_element: u64,
    ) -> Result<Self, MemoryError> {
        Ok(Self {
            inner: BoundedBuffer::new(
                manager,
                capacity,
                bytes_per_element,
            )?,
        })
    }

    /// Creates a stream buffer using the canonical QEC stream limit.
    pub fn from_qec_limits(
        limits: &QecLimits,
        manager: MemoryManager,
        bytes_per_element: u64,
    ) -> Result<Self, MemoryError> {
        Self::new(
            manager,
            limits.max_stream_buffer_events,
            bytes_per_element,
        )
    }

    pub fn push(
        &mut self,
        item: T,
    ) -> Result<(), MemoryError> {
        self.inner.push(item)
    }

    pub fn pop(&mut self) -> Result<T, MemoryError> {
        self.inner.pop()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub const fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn is_full(&self) -> bool {
        self.inner.is_full()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn needs_backpressure(&self) -> bool {
        self.inner.needs_backpressure()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

// -----------------------------------------------------------------------------
// Arena
// -----------------------------------------------------------------------------

/// Safe typed arena-style allocator.
///
/// The arena accounts for its backing capacity rather than merely the number
/// of initialized elements.
#[derive(Debug)]
pub struct Arena<T> {
    manager: MemoryManager,
    values: Vec<T>,
    reserved_bytes: u64,
    _marker: PhantomData<T>,
}

impl<T> Arena<T> {
    /// Creates an empty arena.
    pub fn new(manager: MemoryManager) -> Self {
        Self {
            manager,
            values: Vec::new(),
            reserved_bytes: 0,
            _marker: PhantomData,
        }
    }

    /// Creates an arena with bounded capacity.
    pub fn with_capacity(
        manager: MemoryManager,
        capacity: usize,
    ) -> Result<Self, MemoryError> {
        let bytes =
            estimated_vec_bytes::<T>(capacity)?;

        check_arena_limit(&manager, bytes, 0)?;

        let reservation =
            manager.reserve(bytes)?;

        let values =
            match Vec::try_with_capacity(capacity) {
                Ok(values) => values,

                Err(_) => {
                    reservation.release();

                    return Err(
                        MemoryError::CapacityOverflow
                    );
                }
            };

        /*
         * Transfer the reservation into the arena's accounting. The
         * reservation object itself must not subsequently release the same
         * bytes.
         */
        let reserved_bytes = reservation.into_bytes();

        Ok(Self {
            manager,
            values,
            reserved_bytes,
            _marker: PhantomData,
        })
    }

    /// Adds a value and grows capacity transactionally when necessary.
    pub fn push(
        &mut self,
        value: T,
    ) -> Result<usize, MemoryError> {
        let next_len = self
            .values
            .len()
            .checked_add(1)
            .ok_or(MemoryError::CapacityOverflow)?;

        if next_len > self.values.capacity() {
            self.grow_reservation(next_len)?;
        }

        self.values.push(value);

        Ok(self.values.len() - 1)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.values.get(index)
    }

    pub fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut T> {
        self.values.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn capacity(&self) -> usize {
        self.values.capacity()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Clears initialized values while retaining capacity.
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Bytes currently reserved for arena capacity.
    pub const fn reserved_bytes(&self) -> u64 {
        self.reserved_bytes
    }

    fn grow_reservation(
        &mut self,
        required_elements: usize,
    ) -> Result<(), MemoryError> {
        let new_capacity = required_elements
            .checked_next_power_of_two()
            .ok_or(MemoryError::CapacityOverflow)?;

        let new_bytes =
            estimated_vec_bytes::<T>(new_capacity)?;

        check_arena_limit(
            &self.manager,
            new_bytes,
            self.reserved_bytes,
        )?;

        let additional =
            new_bytes.saturating_sub(
                self.reserved_bytes,
            );

        if additional == 0 {
            return Ok(());
        }

        /*
         * Reserve first. If Vec growth fails, the reservation is released
         * and the arena remains unchanged.
         */
        let reservation =
            self.manager.reserve(additional)?;

        if let Err(_) = self.values.try_reserve_exact(
            new_capacity
                .saturating_sub(
                    self.values.capacity(),
                ),
        ) {
            reservation.release();

            return Err(
                MemoryError::InvalidOperation {
                    reason:
                        "allocator rejected arena growth",
                },
            );
        }

        let committed =
            reservation.into_bytes();

        self.reserved_bytes = self
            .reserved_bytes
            .checked_add(committed)
            .ok_or_else(|| {
                self.manager.release(committed);
                MemoryError::ArithmeticOverflow
            })?;

        Ok(())
    }
}

impl<T> Drop for Arena<T> {
    fn drop(&mut self) {
        self.manager.release(
            self.reserved_bytes,
        );
    }
}

// -----------------------------------------------------------------------------
// Sparse allocation
// -----------------------------------------------------------------------------

/// Memory accounting for sparse QEC structures.
///
/// This type intentionally does not dictate the sparse representation.
/// `sparse.rs` can use it for sparse Paulis, stabilizer matrices, graphs,
/// syndromes and corrections.
#[derive(Debug)]
pub struct SparseAllocation {
    manager: MemoryManager,
    entry_bytes: u64,
    entries: u64,
    reserved_bytes: u64,
}

impl SparseAllocation {
    /// Creates an empty sparse allocation.
    pub fn new(
        manager: MemoryManager,
        entry_bytes: u64,
    ) -> Result<Self, MemoryError> {
        if entry_bytes == 0 {
            return Err(MemoryError::InvalidOperation {
                reason:
                    "sparse entry size must be greater than zero",
            });
        }

        Ok(Self {
            manager,
            entry_bytes,
            entries: 0,
            reserved_bytes: 0,
        })
    }

    /// Reserves additional sparse entries.
    pub fn reserve_entries(
        &mut self,
        count: u64,
    ) -> Result<(), MemoryError> {
        if count == 0 {
            return Ok(());
        }

        self.manager.check()?;

        let additional = count
            .checked_mul(self.entry_bytes)
            .ok_or(MemoryError::ArithmeticOverflow)?;

        let next_entries = self
            .entries
            .checked_add(count)
            .ok_or(MemoryError::ArithmeticOverflow)?;

        let next_reserved = self
            .reserved_bytes
            .checked_add(additional)
            .ok_or(MemoryError::ArithmeticOverflow)?;

        let reservation =
            self.manager.reserve(additional)?;

        /*
         * Commit accounting only after the manager has accepted the
         * reservation.
         */
        let _committed =
            reservation.into_bytes();

        self.entries = next_entries;
        self.reserved_bytes = next_reserved;

        Ok(())
    }

    /// Releases sparse entries.
    pub fn release_entries(
        &mut self,
        count: u64,
    ) {
        let released_entries =
            count.min(self.entries);

        if released_entries == 0 {
            return;
        }

        let bytes =
            released_entries
                .saturating_mul(
                    self.entry_bytes,
                );

        self.entries -= released_entries;

        self.reserved_bytes =
            self.reserved_bytes
                .saturating_sub(bytes);

        self.manager.release(bytes);
    }

    pub const fn entries(&self) -> u64 {
        self.entries
    }

    pub const fn reserved_bytes(&self) -> u64 {
        self.reserved_bytes
    }

    pub const fn entry_bytes(&self) -> u64 {
        self.entry_bytes
    }
}

impl Drop for SparseAllocation {
    fn drop(&mut self) {
        self.manager.release(
            self.reserved_bytes,
        );
    }
}

// -----------------------------------------------------------------------------
// Eviction
// -----------------------------------------------------------------------------

/// Eviction priority.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
)]
pub enum EvictionPriority {
    /// Must not normally be evicted.
    Critical,

    /// Ordinary cache data.
    Normal,

    /// Recomputable intermediate data.
    Recomputable,

    /// Lowest-value temporary data.
    Ephemeral,
}

/// Metadata for an evictable allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvictionCandidate {
    pub id: u64,
    pub bytes: u64,
    pub priority: EvictionPriority,
}

/// Registry of eviction candidates.
///
/// The registry does not own the actual allocation. A higher-level cache or
/// scheduler must perform the actual eviction and then release the associated
/// memory reservation.
#[derive(Debug, Default)]
pub struct EvictionRegistry {
    candidates: Vec<EvictionCandidate>,
}

impl EvictionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(
        &mut self,
        candidate: EvictionCandidate,
    ) -> Result<(), MemoryError> {
        if candidate.bytes == 0 {
            return Err(MemoryError::InvalidOperation {
                reason:
                    "eviction candidate must contain non-zero bytes",
            });
        }

        if self
            .candidates
            .iter()
            .any(|existing| {
                existing.id == candidate.id
            })
        {
            return Err(MemoryError::InvalidOperation {
                reason:
                    "duplicate eviction candidate ID",
            });
        }

        self.candidates.push(candidate);

        Ok(())
    }

    pub fn remove(
        &mut self,
        id: u64,
    ) -> Option<EvictionCandidate> {
        let index = self
            .candidates
            .iter()
            .position(|candidate| {
                candidate.id == id
            })?;

        Some(self.candidates.swap_remove(index))
    }

    /// Selects candidates from lowest-value to highest-value.
    ///
    /// Selection alone does not reclaim memory.
    pub fn select_for_eviction(
        &mut self,
        required_bytes: u64,
    ) -> Vec<EvictionCandidate> {
        if required_bytes == 0 {
            return Vec::new();
        }

        /*
         * `Ephemeral` should be selected before `Recomputable`, then
         * `Normal`, with `Critical` last.
         */
        self.candidates
            .sort_by_key(|candidate| {
                candidate.priority
            });

        let mut selected =
            Vec::new();

        let mut reclaimed =
            0_u64;

        while reclaimed < required_bytes {
            let Some(candidate) =
                self.candidates.pop()
            else {
                break;
            };

            reclaimed =
                reclaimed.saturating_add(
                    candidate.bytes,
                );

            selected.push(candidate);
        }

        selected
    }

    pub fn len(&self) -> usize {
        self.candidates.len()
    }

    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }
}

// -----------------------------------------------------------------------------
// Estimates
// -----------------------------------------------------------------------------

/// Estimates memory required for `count` values.
///
/// This estimates element storage only. Allocator metadata and container
/// overhead are intentionally excluded.
pub fn estimated_bytes<T>(
    count: usize,
) -> Result<u64, MemoryError> {
    let count =
        u64::try_from(count)
            .map_err(|_| {
                MemoryError::CapacityOverflow
            })?;

    count
        .checked_mul(
            size_of::<T>() as u64,
        )
        .ok_or(
            MemoryError::ArithmeticOverflow,
        )
}

/// Estimates a Vec backing allocation.
pub fn estimated_vec_bytes<T>(
    capacity: usize,
) -> Result<u64, MemoryError> {
    estimated_bytes::<T>(capacity)
}

/// Conservative estimate for VecDeque storage.
///
/// The exact allocator layout is implementation-dependent, so this uses the
/// element-storage estimate as a lower-bound preflight value. Runtime element
/// reservations remain authoritative.
pub fn estimated_vecdeque_bytes<T>(
    capacity: usize,
) -> Result<u64, MemoryError> {
    estimated_bytes::<T>(capacity)
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn check_arena_limit(
    manager: &MemoryManager,
    requested: u64,
    already_reserved: u64,
) -> Result<(), MemoryError> {
    if let Some(limit) =
        manager.budget().max_arena_bytes
    {
        let total = already_reserved
            .checked_add(requested)
            .ok_or(
                MemoryError::ArithmeticOverflow,
            )?;

        if total > limit {
            return Err(
                MemoryError::BudgetExceeded {
                    requested,
                    allocated:
                        already_reserved,
                    available:
                        limit.saturating_sub(
                            already_reserved,
                        ),
                    limit,
                },
            );
        }
    }

    Ok(())
}

fn update_peak(
    peak: &AtomicU64,
    current: u64,
) {
    let mut previous =
        peak.load(Ordering::Acquire);

    while current > previous {
        match peak.compare_exchange_weak(
            previous,
            current,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => break,

            Err(actual) => {
                previous = actual;
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn manager(max_bytes: u64) -> MemoryManager {
        MemoryManager::new(
            MemoryBudget {
                max_bytes,
                ..MemoryBudget::default()
            },
        )
        .unwrap()
    }

    #[test]
    fn budget_rejects_zero() {
        let budget = MemoryBudget {
            max_bytes: 0,
            ..MemoryBudget::default()
        };

        assert!(budget.validate().is_err());
    }

    #[test]
    fn qec_limits_are_canonical_memory_source() {
        let mut limits =
            QecLimits::default();

        limits.max_memory_bytes = 4096;
        limits.max_stream_buffer_events = 128;

        let budget =
            MemoryBudget::from_qec_limits(
                &limits,
            );

        assert_eq!(
            budget.max_bytes,
            4096
        );

        assert_eq!(
            budget.max_buffer_elements,
            128
        );
    }

    #[test]
    fn reservation_is_raii() {
        let manager =
            manager(1024);

        {
            let reservation =
                manager.reserve(512)
                    .unwrap();

            assert_eq!(
                manager
                    .snapshot()
                    .allocated_bytes,
                512
            );

            assert_eq!(
                reservation.bytes(),
                512
            );
        }

        assert_eq!(
            manager
                .snapshot()
                .allocated_bytes,
            0
        );
    }

    #[test]
    fn cloned_manager_shares_accounting() {
        let manager =
            manager(1024);

        let clone =
            manager.clone();

        let _reservation =
            clone.reserve(512)
                .unwrap();

        assert_eq!(
            manager
                .snapshot()
                .allocated_bytes,
            512
        );
    }

    #[test]
    fn concurrent_limit_cannot_be_exceeded() {
        use std::thread;

        let manager =
            MemoryManager::new(
                MemoryBudget {
                    max_bytes: 1024,
                    ..MemoryBudget::default()
                },
            )
            .unwrap();

        let mut handles =
            Vec::new();

        for _ in 0..16 {
            let manager =
                manager.clone();

            handles.push(
                thread::spawn(
                    move || {
                        manager
                            .reserve(128)
                            .ok()
                            .map(|reservation| {
                                std::thread::sleep(
                                    std::time::Duration::from_millis(
                                        1,
                                    ),
                                );

                                reservation
                            })
                    },
                ),
            );
        }

        let reservations =
            handles
                .into_iter()
                .filter_map(|handle| {
                    handle.join().unwrap()
                })
                .collect::<Vec<_>>();

        assert!(
            reservations.len() <= 8
        );

        assert!(
            manager
                .snapshot()
                .allocated_bytes
                <= 1024
        );
    }

    #[test]
    fn budget_is_enforced() {
        let manager =
            manager(100);

        let reservation =
            manager.reserve(100)
                .unwrap();

        assert_eq!(
            manager
                .snapshot()
                .allocated_bytes,
            100
        );

        let result =
            manager.reserve(1);

        assert!(matches!(
            result,
            Err(
                MemoryError::BudgetExceeded {
                    ..
                }
            )
        ));

        drop(reservation);
    }

    #[test]
    fn peak_usage_is_retained() {
        let manager =
            manager(1024);

        {
            let _reservation =
                manager.reserve(700)
                    .unwrap();
        }

        let snapshot =
            manager.snapshot();

        assert_eq!(
            snapshot.allocated_bytes,
            0
        );

        assert_eq!(
            snapshot.peak_bytes,
            700
        );
    }

    #[test]
    fn cancellation_blocks_new_reservations() {
        let manager =
            manager(1024);

        manager.cancel();

        assert!(matches!(
            manager.reserve(1),
            Err(MemoryError::Cancelled)
        ));
    }

    #[test]
    fn scoped_quota_is_enforced() {
        let manager =
            manager(1024);

        let mut scope =
            MemoryScope::new(
                manager.clone(),
                Some(256),
            )
            .unwrap();

        let reservation =
            scope.reserve(128)
                .unwrap();

        assert_eq!(
            scope.reserved_bytes(),
            128
        );

        let result =
            scope.reserve(129);

        assert!(matches!(
            result,
            Err(
                MemoryError::QuotaExceeded {
                    ..
                }
            )
        ));

        drop(reservation);
    }

    #[test]
    fn bounded_buffer_applies_backpressure() {
        let manager =
            MemoryManager::new(
                MemoryBudget {
                    max_bytes: 1024,
                    max_buffer_elements: 2,
                    ..MemoryBudget::default()
                },
            )
            .unwrap();

        let mut buffer =
            BoundedBuffer::new(
                manager.clone(),
                2,
                8,
            )
            .unwrap();

        buffer.push(1_u64)
            .unwrap();

        buffer.push(2_u64)
            .unwrap();

        assert!(matches!(
            buffer.push(3_u64),
            Err(
                MemoryError::BufferFull {
                    ..
                }
            )
        ));

        assert_eq!(
            buffer.pop().unwrap(),
            1
        );

        assert_eq!(
            buffer.pop().unwrap(),
            2
        );

        assert!(buffer.is_empty());

        assert_eq!(
            manager
                .snapshot()
                .allocated_bytes,
            0
        );
    }

    #[test]
    fn streaming_buffer_uses_qec_limit() {
        let mut limits =
            QecLimits::default();

        limits.max_memory_bytes =
            1024;

        limits.max_stream_buffer_events =
            2;

        let manager =
            MemoryManager::from_qec_limits(
                &limits,
            )
            .unwrap();

        let mut buffer =
            StreamingBuffer::from_qec_limits(
                &limits,
                manager,
                8,
            )
            .unwrap();

        buffer.push(1_u64)
            .unwrap();

        buffer.push(2_u64)
            .unwrap();

        assert!(matches!(
            buffer.push(3_u64),
            Err(
                MemoryError::BufferFull {
                    ..
                }
            )
        ));
    }

    #[test]
    fn sparse_allocation_is_accounted() {
        let manager =
            manager(1024);

        let mut sparse =
            SparseAllocation::new(
                manager.clone(),
                16,
            )
            .unwrap();

        sparse
            .reserve_entries(10)
            .unwrap();

        assert_eq!(
            sparse.entries(),
            10
        );

        assert_eq!(
            sparse.reserved_bytes(),
            160
        );

        assert_eq!(
            manager
                .snapshot()
                .allocated_bytes,
            160
        );

        sparse.release_entries(4);

        assert_eq!(
            sparse.entries(),
            6
        );

        assert_eq!(
            sparse.reserved_bytes(),
            96
        );
    }

    #[test]
    fn arena_accounts_capacity() {
        let manager =
            manager(4096);

        let mut arena =
            Arena::<u64>::with_capacity(
                manager.clone(),
                4,
            )
            .unwrap();

        assert!(
            arena.reserved_bytes()
                >= 32
        );

        arena.push(1)
            .unwrap();

        arena.push(2)
            .unwrap();

        assert_eq!(
            arena.len(),
            2
        );

        drop(arena);

        assert_eq!(
            manager
                .snapshot()
                .allocated_bytes,
            0
        );
    }

    #[test]
    fn arena_rejects_excessive_capacity() {
        let manager =
            MemoryManager::new(
                MemoryBudget {
                    max_bytes: 1024,
                    max_arena_bytes: Some(16),
                    ..MemoryBudget::default()
                },
            )
            .unwrap();

        let result =
            Arena::<u64>::with_capacity(
                manager,
                3,
            );

        assert!(result.is_err());
    }

    #[test]
    fn eviction_selection_does_not_fake_reclamation() {
        let manager =
            manager(1024);

        let reservation =
            manager.reserve(512)
                .unwrap();

        let mut registry =
            EvictionRegistry::new();

        registry
            .register(
                EvictionCandidate {
                    id: 1,
                    bytes: 256,
                    priority:
                        EvictionPriority::Ephemeral,
                },
            )
            .unwrap();

        let selected =
            registry
                .select_for_eviction(256);

        assert_eq!(
            selected.len(),
            1
        );

        /*
         * Selecting an eviction candidate must not silently modify actual
         * memory accounting.
         */
        assert_eq!(
            manager
                .snapshot()
                .allocated_bytes,
            512
        );

        drop(reservation);
    }

    #[test]
    fn arithmetic_estimates_are_checked() {
        let result =
            estimated_bytes::<u64>(
                usize::MAX,
            );

        if size_of::<usize>() >= 8 {
            assert!(
                result.is_err()
            );
        }
    }
}