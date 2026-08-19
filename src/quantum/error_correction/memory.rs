//! Production-grade memory management for Zamani QEC.
//!
//! This module provides bounded, observable and failure-safe memory primitives
//! for large quantum-error-correction workloads.
//!
//! # Design goals
//!
//! * explicit memory budgets;
//! * checked arithmetic;
//! * no unsafe code;
//! * no unbounded allocations through public APIs;
//! * RAII memory reservations;
//! * peak-memory accounting;
//! * bounded buffers;
//! * bounded streaming buffers;
//! * arena-style typed allocation;
//! * sparse allocation accounting;
//! * eviction accounting/policies;
//! * thread-safe budget accounting;
//! * deterministic failure when budgets are exceeded;
//! * graceful operation under resource pressure;
//! * integration-friendly with `resources.rs`, `limits.rs`, `errors.rs`,
//!   `streaming.rs`, `cache.rs`, and `scheduler.rs`.
//!
//! # Important
//!
//! "Unlimited" means that this module does not impose an additional
//! application-level limit. It does not mean physical memory is infinite.
//!
//! The preferred production model is:
//!
//! ```text
//! QEC operation
//!      │
//!      ▼
//! MemoryBudget
//!      │
//!      ├── reservation
//!      ├── bounded allocation
//!      ├── streaming buffer
//!      ├── sparse allocation
//!      └── arena allocation
//!      │
//!      ▼
//! deterministic ResourceError
//! ```
//!
//! No QEC algorithm should blindly allocate enormous collections before
//! consulting the memory policy.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::VecDeque;
use std::fmt;
use std::marker::PhantomData;
use std::mem::size_of;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

// -----------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------

/// Explicit application-level unlimited-memory sentinel.
pub const UNLIMITED_MEMORY: u64 = u64::MAX;

/// Default memory budget: 1 GiB.
pub const DEFAULT_MEMORY_BUDGET: u64 = 1024 * 1024 * 1024;

/// Default bounded-buffer capacity.
pub const DEFAULT_BUFFER_CAPACITY: usize = 4096;

// -----------------------------------------------------------------------------
// Memory errors
// -----------------------------------------------------------------------------

/// Errors produced by the memory subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryError {
    /// A memory policy is invalid.
    InvalidBudget {
        reason: &'static str,
    },

    /// A requested allocation exceeds the configured memory budget.
    BudgetExceeded {
        requested: u64,
        allocated: u64,
        available: u64,
        limit: u64,
    },

    /// A reservation quota was exceeded.
    QuotaExceeded {
        requested: u64,
        reserved: u64,
        quota: u64,
    },

    /// Memory arithmetic overflowed.
    ArithmeticOverflow,

    /// A requested collection cannot be represented safely.
    CapacityOverflow,

    /// A bounded buffer is full.
    BufferFull {
        capacity: usize,
    },

    /// A bounded buffer has no element available.
    BufferEmpty,

    /// A requested operation is invalid.
    InvalidOperation {
        reason: &'static str,
    },

    /// The memory subsystem was cancelled.
    Cancelled,

    /// An eviction request could not free enough memory.
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

/// Explicit memory policy for a QEC workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryBudget {
    /// Maximum simultaneously reserved bytes.
    pub max_bytes: u64,

    /// Optional per-operation reservation limit.
    pub max_operation_bytes: Option<u64>,

    /// Maximum arena capacity in bytes.
    pub max_arena_bytes: Option<u64>,

    /// Maximum bounded-buffer capacity in elements.
    pub max_buffer_elements: usize,

    /// Whether memory pressure may request eviction.
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
    /// Creates an explicitly unlimited application-level budget.
    pub const fn unlimited() -> Self {
        Self {
            max_bytes: UNLIMITED_MEMORY,
            max_operation_bytes: None,
            max_arena_bytes: None,
            max_buffer_elements: usize::MAX,
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
                    reason: "operation memory limit cannot exceed global memory limit",
                });
            }
        }

        if let Some(arena) = self.max_arena_bytes {
            if arena > self.max_bytes {
                return Err(MemoryError::InvalidBudget {
                    reason: "arena memory limit cannot exceed global memory limit",
                });
            }
        }

        Ok(())
    }

    /// Returns whether the budget represents an application-level unlimited
    /// global memory policy.
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

    /// Highest observed reservation.
    pub peak_bytes: u64,

    /// Total successful reservation operations.
    pub allocation_count: u64,

    /// Total release operations.
    pub release_count: u64,

    /// Total failed allocation attempts.
    pub failed_allocations: u64,

    /// Total bytes ever reserved.
    pub cumulative_allocated_bytes: u64,

    /// Total bytes released.
    pub cumulative_released_bytes: u64,

    /// Number of eviction requests.
    pub eviction_requests: u64,

    /// Bytes reported as reclaimed by eviction.
    pub evicted_bytes: u64,
}

/// Internal atomic counters.
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

/// Thread-safe memory manager.
///
/// It is safe to share through `Arc`.
#[derive(Debug)]
pub struct MemoryManager {
    budget: MemoryBudget,
    counters: MemoryCounters,
    cancelled: AtomicBool,
}

impl MemoryManager {
    /// Creates a memory manager after validating the budget.
    pub fn new(budget: MemoryBudget) -> Result<Self, MemoryError> {
        budget.validate()?;

        Ok(Self {
            budget,
            counters: MemoryCounters::default(),
            cancelled: AtomicBool::new(false),
        })
    }

    /// Returns the configured memory policy.
    pub const fn budget(&self) -> MemoryBudget {
        self.budget
    }

    /// Requests cancellation of future allocations.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Clears cancellation before starting a new logical operation.
    pub fn reset_cancellation(&self) {
        self.cancelled.store(false, Ordering::Release);
    }

    /// Returns whether allocation has been cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    /// Checks whether an operation may continue.
    pub fn check(&self) -> Result<(), MemoryError> {
        if self.is_cancelled() {
            Err(MemoryError::Cancelled)
        } else {
            Ok(())
        }
    }

    /// Returns a point-in-time memory snapshot.
    pub fn snapshot(&self) -> MemorySnapshot {
        MemorySnapshot {
            allocated_bytes: self
                .counters
                .allocated_bytes
                .load(Ordering::Acquire),

            peak_bytes: self
                .counters
                .peak_bytes
                .load(Ordering::Acquire),

            allocation_count: self
                .counters
                .allocation_count
                .load(Ordering::Acquire),

            release_count: self
                .counters
                .release_count
                .load(Ordering::Acquire),

            failed_allocations: self
                .counters
                .failed_allocations
                .load(Ordering::Acquire),

            cumulative_allocated_bytes: self
                .counters
                .cumulative_allocated_bytes
                .load(Ordering::Acquire),

            cumulative_released_bytes: self
                .counters
                .cumulative_released_bytes
                .load(Ordering::Acquire),

            eviction_requests: self
                .counters
                .eviction_requests
                .load(Ordering::Acquire),

            evicted_bytes: self
                .counters
                .evicted_bytes
                .load(Ordering::Acquire),
        }
    }

    /// Returns currently available memory under the global budget.
    pub fn available_bytes(&self) -> u64 {
        if self.budget.is_unlimited() {
            return u64::MAX;
        }

        self.budget
            .max_bytes
            .saturating_sub(
                self.counters
                    .allocated_bytes
                    .load(Ordering::Acquire),
            )
    }

    /// Attempts to reserve bytes.
    pub fn reserve(
        &self,
        bytes: u64,
    ) -> Result<MemoryReservation<'_>, MemoryError> {
        self.reserve_with_quota(bytes, self.budget.max_operation_bytes)?;

        Ok(MemoryReservation {
            manager: self,
            bytes,
            active: true,
        })
    }

    /// Attempts to reserve bytes under an optional operation quota.
    pub fn reserve_with_quota(
        &self,
        bytes: u64,
        quota: Option<u64>,
    ) -> Result<(), MemoryError> {
        self.check()?;

        if bytes == 0 {
            return Ok(());
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

        if !self.budget.is_unlimited() {
            let current = self
                .counters
                .allocated_bytes
                .load(Ordering::Acquire);

            let next = current
                .checked_add(bytes)
                .ok_or_else(|| {
                    self.record_failed_allocation();
                    MemoryError::ArithmeticOverflow
                })?;

            if next > self.budget.max_bytes {
                self.record_failed_allocation();

                return Err(MemoryError::BudgetExceeded {
                    requested: bytes,
                    allocated: current,
                    available: self.available_bytes(),
                    limit: self.budget.max_bytes,
                });
            }
        }

        let previous = self
            .counters
            .allocated_bytes
            .fetch_add(bytes, Ordering::AcqRel);

        let next = previous.checked_add(bytes).ok_or_else(|| {
            self.counters
                .allocated_bytes
                .fetch_sub(bytes, Ordering::AcqRel);

            self.record_failed_allocation();

            MemoryError::ArithmeticOverflow
        })?;

        if !self.budget.is_unlimited()
            && next > self.budget.max_bytes
        {
            self.counters
                .allocated_bytes
                .fetch_sub(bytes, Ordering::AcqRel);

            self.record_failed_allocation();

            return Err(MemoryError::BudgetExceeded {
                requested: bytes,
                allocated: previous,
                available: self
                    .budget
                    .max_bytes
                    .saturating_sub(previous),
                limit: self.budget.max_bytes,
            });
        }

        self.counters
            .allocation_count
            .fetch_add(1, Ordering::Relaxed);

        self.counters
            .cumulative_allocated_bytes
            .fetch_add(bytes, Ordering::Relaxed);

        update_peak(
            &self.counters.peak_bytes,
            next,
        );

        Ok(())
    }

    /// Releases previously reserved bytes.
    ///
    /// This method is saturating by design so accounting cleanup cannot
    /// panic if a caller attempts to release more than the currently tracked
    /// amount.
    pub fn release(&self, bytes: u64) {
        if bytes == 0 {
            return;
        }

        let mut current = self
            .counters
            .allocated_bytes
            .load(Ordering::Acquire);

        loop {
            let next = current.saturating_sub(bytes);

            match self.counters.allocated_bytes.compare_exchange_weak(
                current,
                next,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(actual) => current = actual,
            }
        }

        self.counters
            .release_count
            .fetch_add(1, Ordering::Relaxed);

        self.counters
            .cumulative_released_bytes
            .fetch_add(
                bytes.min(current),
                Ordering::Relaxed,
            );
    }

    fn record_failed_allocation(&self) {
        self.counters
            .failed_allocations
            .fetch_add(1, Ordering::Relaxed);
    }

    /// Records an externally performed eviction.
    ///
    /// The manager does not know how an eviction is implemented; callers
    /// report the successfully reclaimed amount.
    pub fn record_eviction(&self, reclaimed_bytes: u64) {
        self.counters
            .eviction_requests
            .fetch_add(1, Ordering::Relaxed);

        self.counters
            .evicted_bytes
            .fetch_add(reclaimed_bytes, Ordering::Relaxed);
    }

    /// Checks whether a requested amount fits.
    pub fn can_reserve(&self, bytes: u64) -> bool {
        self.available_bytes() >= bytes
    }
}

impl Clone for MemoryManager {
    fn clone(&self) -> Self {
        Self {
            budget: self.budget,
            counters: MemoryCounters::default(),
            cancelled: AtomicBool::new(
                self.is_cancelled(),
            ),
        }
    }
}

// -----------------------------------------------------------------------------
// RAII reservation
// -----------------------------------------------------------------------------

/// RAII memory reservation.
///
/// Memory is automatically released when the reservation is dropped.
#[derive(Debug)]
pub struct MemoryReservation<'a> {
    manager: &'a MemoryManager,
    bytes: u64,
    active: bool,
}

impl<'a> MemoryReservation<'a> {
    pub const fn bytes(&self) -> u64 {
        self.bytes
    }

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
}

impl Drop for MemoryReservation<'_> {
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
/// This prevents an individual decoder operation from silently consuming the
/// entire global memory budget.
#[derive(Debug)]
pub struct MemoryScope<'a> {
    manager: &'a MemoryManager,
    quota: Option<u64>,
    reserved: u64,
}

impl<'a> MemoryScope<'a> {
    pub fn new(
        manager: &'a MemoryManager,
        quota: Option<u64>,
    ) -> Result<Self, MemoryError> {
        if quota == Some(0) {
            return Err(MemoryError::InvalidBudget {
                reason: "memory scope quota must be greater than zero",
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

    /// Reserves memory inside this operation.
    pub fn reserve(
        &mut self,
        bytes: u64,
    ) -> Result<ScopedReservation<'_>, MemoryError> {
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

        self.manager
            .reserve_with_quota(bytes, self.quota)?;

        self.reserved = next;

        Ok(ScopedReservation {
            scope: self,
            bytes,
            active: true,
        })
    }

    pub const fn reserved_bytes(&self) -> u64 {
        self.reserved
    }

    pub fn remaining_bytes(&self) -> u64 {
        match self.quota {
            Some(quota) => quota.saturating_sub(self.reserved),
            None => self.manager.available_bytes(),
        }
    }
}

/// RAII reservation belonging to a `MemoryScope`.
#[derive(Debug)]
pub struct ScopedReservation<'a> {
    scope: &'a mut MemoryScope<'a>,
    bytes: u64,
    active: bool,
}

impl ScopedReservation<'_> {
    pub const fn bytes(&self) -> u64 {
        self.bytes
    }
}

impl Drop for ScopedReservation<'_> {
    fn drop(&mut self) {
        if self.active {
            self.scope.manager.release(self.bytes);
            self.scope.reserved =
                self.scope.reserved.saturating_sub(self.bytes);
            self.active = false;
        }
    }
}

// -----------------------------------------------------------------------------
// Bounded buffer
// -----------------------------------------------------------------------------

/// Memory-accounted bounded FIFO buffer.
///
/// The buffer refuses new elements when its configured element limit or
/// memory budget would be exceeded.
#[derive(Debug)]
pub struct BoundedBuffer<T> {
    manager: Arc<MemoryManager>,
    queue: VecDeque<T>,
    capacity: usize,
    bytes_per_element: u64,
}

impl<T> BoundedBuffer<T> {
    /// Creates a bounded buffer.
    ///
    /// `bytes_per_element` is the accounting estimate used for capacity
    /// reservation. It should conservatively estimate the memory consumed
    /// by each buffered element.
    pub fn new(
        manager: Arc<MemoryManager>,
        capacity: usize,
        bytes_per_element: u64,
    ) -> Result<Self, MemoryError> {
        if capacity == 0 {
            return Err(MemoryError::InvalidOperation {
                reason: "buffer capacity must be greater than zero",
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

        Ok(Self {
            manager,
            queue: VecDeque::with_capacity(capacity),
            capacity,
            bytes_per_element,
        })
    }

    /// Number of currently buffered elements.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.queue.len() >= self.capacity
    }

    /// Pushes an element if both element and memory bounds permit it.
    pub fn push(&mut self, value: T) -> Result<(), MemoryError> {
        if self.is_full() {
            return Err(MemoryError::BufferFull {
                capacity: self.capacity,
            });
        }

        self.manager.reserve(self.bytes_per_element)?;

        if let Err(error) = self.queue.try_reserve(1) {
            self.manager.release(self.bytes_per_element);

            return Err(MemoryError::InvalidOperation {
                reason: allocation_error_reason(error),
            });
        }

        self.queue.push_back(value);

        Ok(())
    }

    /// Removes the oldest element.
    pub fn pop(&mut self) -> Result<T, MemoryError> {
        match self.queue.pop_front() {
            Some(value) => {
                self.manager.release(self.bytes_per_element);
                Ok(value)
            }

            None => Err(MemoryError::BufferEmpty),
        }
    }

    /// Removes all elements and releases their accounting.
    pub fn clear(&mut self) {
        let count = self.queue.len();

        self.queue.clear();

        let bytes = self
            .bytes_per_element
            .saturating_mul(count as u64);

        self.manager.release(bytes);
    }
}

impl<T> Drop for BoundedBuffer<T> {
    fn drop(&mut self) {
        let count = self.queue.len();

        let bytes = self
            .bytes_per_element
            .saturating_mul(count as u64);

        self.manager.release(bytes);
    }
}

// -----------------------------------------------------------------------------
// Streaming buffer
// -----------------------------------------------------------------------------

/// Explicit bounded streaming buffer.
///
/// Unlike an ordinary collection, this structure is intended for syndrome
/// streams and detection-event pipelines where backpressure is preferable to
/// unbounded memory growth.
#[derive(Debug)]
pub struct StreamingBuffer<T> {
    inner: BoundedBuffer<T>,
}

impl<T> StreamingBuffer<T> {
    pub fn new(
        manager: Arc<MemoryManager>,
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

    pub fn push(&mut self, item: T) -> Result<(), MemoryError> {
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

    /// Backpressure indicator.
    pub fn needs_backpressure(&self) -> bool {
        self.is_full()
    }

    /// Clears the stream.
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

// -----------------------------------------------------------------------------
// Arena allocation
// -----------------------------------------------------------------------------

/// Safe typed arena-style allocator.
///
/// This allocator stores values in a contiguous `Vec` and accounts for the
/// configured element size. It intentionally does not expose raw pointers.
#[derive(Debug)]
pub struct Arena<T> {
    manager: Arc<MemoryManager>,
    values: Vec<T>,
    reserved_bytes: u64,
    _marker: PhantomData<T>,
}

impl<T> Arena<T> {
    /// Creates an empty arena.
    pub fn new(
        manager: Arc<MemoryManager>,
    ) -> Self {
        Self {
            manager,
            values: Vec::new(),
            reserved_bytes: 0,
            _marker: PhantomData,
        }
    }

    /// Creates an arena with a bounded capacity.
    pub fn with_capacity(
        manager: Arc<MemoryManager>,
        capacity: usize,
    ) -> Result<Self, MemoryError> {
        let bytes = estimated_vec_bytes::<T>(capacity)?;

        if let Some(limit) = manager.budget().max_arena_bytes {
            if bytes > limit {
                return Err(MemoryError::BudgetExceeded {
                    requested: bytes,
                    allocated: 0,
                    available: limit,
                    limit,
                });
            }
        }

        manager.reserve(bytes)?;

        let values = Vec::try_with_capacity(capacity)
            .map_err(|_| {
                manager.release(bytes);

                MemoryError::CapacityOverflow
            })?;

        Ok(Self {
            manager,
            values,
            reserved_bytes: bytes,
            _marker: PhantomData,
        })
    }

    /// Adds a value.
    pub fn push(&mut self, value: T) -> Result<usize, MemoryError> {
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

    /// Returns an immutable value.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.values.get(index)
    }

    /// Returns a mutable value.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
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

    /// Clears the arena while retaining capacity.
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Returns the currently accounted capacity.
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

        if let Some(limit) = self.manager.budget().max_arena_bytes {
            if new_bytes > limit {
                return Err(MemoryError::BudgetExceeded {
                    requested: new_bytes,
                    allocated: self.reserved_bytes,
                    available: limit.saturating_sub(
                        self.reserved_bytes,
                    ),
                    limit,
                });
            }
        }

        let additional =
            new_bytes.saturating_sub(self.reserved_bytes);

        if additional > 0 {
            self.manager.reserve(additional)?;

            if let Err(error) =
                self.values.try_reserve_exact(
                    new_capacity
                        .saturating_sub(self.values.capacity()),
                )
            {
                self.manager.release(additional);

                return Err(MemoryError::InvalidOperation {
                    reason: allocation_error_reason(error),
                });
            }

            self.reserved_bytes = new_bytes;
        }

        Ok(())
    }
}

impl<T> Drop for Arena<T> {
    fn drop(&mut self) {
        self.manager.release(self.reserved_bytes);
    }
}

// -----------------------------------------------------------------------------
// Sparse allocation
// -----------------------------------------------------------------------------

/// Memory accounting for sparse QEC structures.
///
/// This does not dictate a particular sparse representation. It gives sparse
/// graph, stabilizer and syndrome implementations a safe way to reserve and
/// release memory for entries.
#[derive(Debug)]
pub struct SparseAllocation {
    manager: Arc<MemoryManager>,
    entry_bytes: u64,
    entries: u64,
    reserved_bytes: u64,
}

impl SparseAllocation {
    /// Creates an empty sparse allocation.
    pub fn new(
        manager: Arc<MemoryManager>,
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
        let additional = count
            .checked_mul(self.entry_bytes)
            .ok_or(MemoryError::ArithmeticOverflow)?;

        self.manager.reserve(additional)?;

        self.entries = self
            .entries
            .checked_add(count)
            .ok_or_else(|| {
                self.manager.release(additional);
                MemoryError::ArithmeticOverflow
            })?;

        self.reserved_bytes = self
            .reserved_bytes
            .checked_add(additional)
            .ok_or_else(|| {
                self.manager.release(additional);
                MemoryError::ArithmeticOverflow
            })?;

        Ok(())
    }

    /// Releases sparse entries.
    pub fn release_entries(
        &mut self,
        count: u64,
    ) {
        let released_entries = count.min(self.entries);

        let bytes = released_entries
            .saturating_mul(self.entry_bytes);

        self.entries -= released_entries;
        self.reserved_bytes =
            self.reserved_bytes.saturating_sub(bytes);

        self.manager.release(bytes);
    }

    pub const fn entries(&self) -> u64 {
        self.entries
    }

    pub const fn reserved_bytes(&self) -> u64 {
        self.reserved_bytes
    }
}

impl Drop for SparseAllocation {
    fn drop(&mut self) {
        self.manager.release(self.reserved_bytes);
    }
}

// -----------------------------------------------------------------------------
// Eviction
// -----------------------------------------------------------------------------

/// Eviction priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum EvictionPriority {
    /// Evict only as a last resort.
    Critical,

    /// Normal cache data.
    Normal,

    /// Temporary/recomputable data.
    Recomputable,

    /// Lowest-value data.
    Ephemeral,
}

/// Metadata for an evictable allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvictionCandidate {
    pub id: u64,
    pub bytes: u64,
    pub priority: EvictionPriority,
}

/// Tracks eviction candidates without owning their underlying data.
///
/// Actual cache eviction remains the responsibility of the cache/backend.
#[derive(Debug, Default)]
pub struct EvictionRegistry {
    candidates: Vec<EvictionCandidate>,
}

impl EvictionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an eviction candidate.
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
            .any(|existing| existing.id == candidate.id)
        {
            return Err(MemoryError::InvalidOperation {
                reason: "duplicate eviction candidate ID",
            });
        }

        self.candidates.push(candidate);

        Ok(())
    }

    /// Removes a candidate.
    pub fn remove(&mut self, id: u64) -> Option<EvictionCandidate> {
        let index = self
            .candidates
            .iter()
            .position(|candidate| candidate.id == id)?;

        Some(self.candidates.swap_remove(index))
    }

    /// Selects candidates from lowest-value to highest-value.
    pub fn select_for_eviction(
        &mut self,
        required_bytes: u64,
    ) -> Vec<EvictionCandidate> {
        self.candidates.sort_by(|a, b| {
            b.priority.cmp(&a.priority)
        });

        let mut selected = Vec::new();
        let mut reclaimed = 0_u64;

        while reclaimed < required_bytes {
            let Some(candidate) = self.candidates.pop() else {
                break;
            };

            reclaimed =
                reclaimed.saturating_add(candidate.bytes);

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
// Allocation estimates
// -----------------------------------------------------------------------------

/// Estimates memory required for `count` values.
pub fn estimated_bytes<T>(
    count: usize,
) -> Result<u64, MemoryError> {
    let count =
        u64::try_from(count)
            .map_err(|_| MemoryError::CapacityOverflow)?;

    count
        .checked_mul(size_of::<T>() as u64)
        .ok_or(MemoryError::ArithmeticOverflow)
}

/// Estimates Vec backing storage.
///
/// This intentionally includes only element storage; allocator-specific
/// metadata remains implementation-dependent.
pub fn estimated_vec_bytes<T>(
    capacity: usize,
) -> Result<u64, MemoryError> {
    estimated_bytes::<T>(capacity)
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn update_peak(
    peak: &AtomicU64,
    current: u64,
) {
    let mut previous = peak.load(Ordering::Acquire);

    while current > previous {
        match peak.compare_exchange_weak(
            previous,
            current,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => break,
            Err(actual) => previous = actual,
        }
    }
}

fn allocation_error_reason(
    _error: std::collections::TryReserveError,
) -> &'static str {
    "allocator rejected requested capacity"
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budget_rejects_zero() {
        let budget = MemoryBudget {
            max_bytes: 0,
            ..MemoryBudget::default()
        };

        assert!(budget.validate().is_err());
    }

    #[test]
    fn reservation_is_raii() {
        let manager = MemoryManager::new(
            MemoryBudget {
                max_bytes: 1024,
                ..MemoryBudget::default()
            },
        )
        .unwrap();

        {
            let reservation =
                manager.reserve(512).unwrap();

            assert_eq!(
                manager.snapshot().allocated_bytes,
                512
            );

            assert_eq!(reservation.bytes(), 512);
        }

        assert_eq!(
            manager.snapshot().allocated_bytes,
            0
        );
    }

    #[test]
    fn budget_is_enforced() {
        let manager = MemoryManager::new(
            MemoryBudget {
                max_bytes: 100,
                ..MemoryBudget::default()
            },
        )
        .unwrap();

        let reservation = manager.reserve(100).unwrap();
        assert_eq!(
            manager.snapshot().allocated_bytes,
            100
        );

        let result = manager.reserve(1);
        assert!(matches!(
            result,
            Err(MemoryError::BudgetExceeded { .. })
        ));

        drop(reservation);
    }

    #[test]
    fn peak_usage_is_retained() {
        let manager = MemoryManager::new(
            MemoryBudget {
                max_bytes: 1024,
                ..MemoryBudget::default()
            },
        )
        .unwrap();

        {
            let _reservation =
                manager.reserve(700).unwrap();
        }

        let snapshot = manager.snapshot();

        assert_eq!(snapshot.allocated_bytes, 0);
        assert_eq!(snapshot.peak_bytes, 700);
    }

    #[test]
    fn bounded_buffer_applies_backpressure() {
        let manager = Arc::new(
            MemoryManager::new(
                MemoryBudget {
                    max_bytes: 1024,
                    max_buffer_elements: 2,
                    ..MemoryBudget::default()
                },
            )
            .unwrap(),
        );

        let mut buffer =
            BoundedBuffer::new(
                manager.clone(),
                2,
                8,
            )
            .unwrap();

        buffer.push(1_u64).unwrap();
        buffer.push(2_u64).unwrap();

        assert!(matches!(
            buffer.push(3_u64),
            Err(MemoryError::BufferFull { .. })
        ));

        assert_eq!(buffer.pop().unwrap(), 1);
        assert_eq!(buffer.pop().unwrap(), 2);
        assert!(buffer.is_empty());
    }

    #[test]
    fn sparse_allocation_is_accounted() {
        let manager = Arc::new(
            MemoryManager::new(
                MemoryBudget {
                    max_bytes: 1024,
                    ..MemoryBudget::default()
                },
            )
            .unwrap(),
        );

        let mut sparse =
            SparseAllocation::new(
                manager.clone(),
                16,
            )
            .unwrap();

        sparse.reserve_entries(10).unwrap();

        assert_eq!(sparse.entries(), 10);
        assert_eq!(sparse.reserved_bytes(), 160);
        assert_eq!(
            manager.snapshot().allocated_bytes,
            160
        );

        sparse.release_entries(5);

        assert_eq!(sparse.entries(), 5);
        assert_eq!(
            manager.snapshot().allocated_bytes,
            80
        );
    }

    #[test]
    fn arena_is_bounded() {
        let manager = Arc::new(
            MemoryManager::new(
                MemoryBudget {
                    max_bytes: 4096,
                    max_arena_bytes: Some(2048),
                    ..MemoryBudget::default()
                },
            )
            .unwrap(),
        );

        let mut arena =
            Arena::<u64>::with_capacity(
                manager.clone(),
                8,
            )
            .unwrap();

        for value in 0..8 {
            arena.push(value).unwrap();
        }

        assert_eq!(arena.len(), 8);
        assert_eq!(arena.capacity(), 8);
    }

    #[test]
    fn cancellation_blocks_new_allocations() {
        let manager =
            MemoryManager::new(
                MemoryBudget::default(),
            )
            .unwrap();

        manager.cancel();

        assert!(matches!(
            manager.reserve(1),
            Err(MemoryError::Cancelled)
        ));
    }

    #[test]
    fn eviction_registry_orders_recomputable_data_first() {
        let mut registry =
            EvictionRegistry::new();

        registry
            .register(EvictionCandidate {
                id: 1,
                bytes: 100,
                priority: EvictionPriority::Critical,
            })
            .unwrap();

        registry
            .register(EvictionCandidate {
                id: 2,
                bytes: 100,
                priority: EvictionPriority::Ephemeral,
            })
            .unwrap();

        let selected =
            registry.select_for_eviction(100);

        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].id, 2);
    }

    #[test]
    fn estimated_bytes_checks_overflow() {
        let result =
            estimated_bytes::<u64>(usize::MAX);

        if usize::MAX as u128
            * size_of::<u64>() as u128
            > u64::MAX as u128
        {
            assert!(result.is_err());
        }
    }

    #[test]
    fn unlimited_budget_is_valid() {
        let budget =
            MemoryBudget::unlimited();

        assert!(budget.validate().is_ok());
        assert!(budget.is_unlimited());
    }
}