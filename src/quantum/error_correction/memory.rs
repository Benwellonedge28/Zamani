//! Production memory reservation and bounded-allocation primitives for Zamani QEC.
//!
//! # Architectural contract
//!
//! `limits.rs` owns declarative QEC policy.
//!
//! `memory.rs` owns:
//!
//! - memory admission;
//! - memory reservations;
//! - per-operation memory scopes;
//! - allocation guards;
//! - bounded buffers;
//! - accounted vectors;
//! - byte arenas;
//! - memory-specific runtime statistics.
//!
//! `resources.rs` owns broader runtime resource accounting.
//!
//! `cancellation.rs` owns cancellation state and propagation.
//!
//! `errors.rs` owns the canonical public QEC error boundary.
//!
//! Dependency direction:
//!
//! ```text
//! QecLimits
//!     │
//!     ▼
//! MemoryBudget
//!     │
//!     ▼
//! MemoryManager ───────────────┐
//!     │                        │
//!     ├── MemoryScope          │
//!     ├── MemoryReservation    │
//!     ├── AccountedVec         │
//!     ├── BoundedBuffer        │
//!     └── MemoryArena          │
//!                              ▼
//!                     allocation enforcement
//!
//! CancellationToken ──────────► admission checks
//!
//! MemorySnapshot ─────────────► resources.rs / metrics.rs
//!
//! MemoryError ────────────────► QecError
//! ```
//!
//! # Important invariants
//!
//! 1. `QecLimits` remains the canonical declarative policy.
//! 2. This module does not introduce another global QEC resource policy.
//! 3. Reservations are checked atomically before they are granted.
//! 4. A successful reservation always has an RAII owner.
//! 5. Dropping a reservation releases its accounting.
//! 6. Cloning `MemoryManager` shares the same accounting state.
//! 7. Operation scopes can only tighten the global memory policy.
//! 8. Cancellation is delegated to `CancellationToken`.
//! 9. Arithmetic used for admission is checked.
//! 10. Collection allocation uses fallible reservation APIs where available.
//! 11. Bounded buffers cannot grow beyond their admitted capacity.
//! 12. Arena growth is admitted before the underlying allocation is attempted.
//! 13. Eviction reporting never falsely decreases allocated memory.
//! 14. `unsafe` is forbidden.
//! 15. This module does not perform network, QPU, decoder, or scheduler work.
//!
//! # Integration contract
//!
//! `limits.rs`:
//!     `MemoryBudget::from_qec_limits()` derives the memory policy.
//!
//! `cancellation.rs`:
//!     `MemoryManager::with_cancellation()` attaches the execution token.
//!
//! `errors.rs`:
//!     `MemoryError` converts into `QecError` at the public boundary.
//!
//! `resources.rs`:
//!     consumes `MemorySnapshot` for broader runtime accounting.
//!
//! `surface_code.rs`:
//!     reserves estimated code memory before constructing large topology data.
//!
//! `sparse.rs`:
//!     reserves sparse storage before capacity growth.
//!
//! `syndrome.rs`:
//!     reserves bounded syndrome storage.
//!
//! `decoding_graph.rs`:
//!     reserves graph node/edge storage before allocation.
//!
//! `decoder.rs`, `mwpm.rs`, `union_find.rs`:
//!     use `MemoryManager` or `MemoryScope` before decoder work.
//!
//! `streaming.rs`:
//!     uses `BoundedBuffer` for bounded backpressure.
//!
//! `checkpoint.rs`:
//!     uses reservations before constructing large checkpoint buffers.
//!
//! No later module should create another independent memory-limit system.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::{fmt, mem::size_of};
use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use super::{
    cancellation::CancellationToken,
    errors::{QecError, QecErrorKind, QecResult, ResourceKind},
    limits::QecLimits,
};

/// Explicit application-level unlimited-memory sentinel.
///
/// This does not mean the machine has infinite physical memory.
pub const UNLIMITED_MEMORY: u64 = u64::MAX;

/// Standalone memory budget.
///
/// Production QEC execution should derive the budget from `QecLimits`.
pub const DEFAULT_MEMORY_BUDGET: u64 = 1024 * 1024 * 1024;

/// Standalone bounded-buffer capacity.
pub const DEFAULT_BUFFER_CAPACITY: usize = 4096;

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the memory subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryError {
    /// The memory policy itself is invalid.
    InvalidBudget {
        reason: &'static str,
    },

    /// The global memory budget would be exceeded.
    BudgetExceeded {
        requested: u64,
        allocated: u64,
        available: u64,
        limit: u64,
    },

    /// A per-operation memory quota would be exceeded.
    QuotaExceeded {
        requested: u64,
        reserved: u64,
        quota: u64,
    },

    /// Memory accounting arithmetic overflowed.
    ArithmeticOverflow,

    /// A collection capacity calculation overflowed.
    CapacityOverflow,

    /// The bounded buffer has reached its admitted capacity.
    BufferFull {
        capacity: usize,
    },

    /// The bounded buffer contains no elements.
    BufferEmpty,

    /// An operation violates the memory API contract.
    InvalidOperation {
        reason: &'static str,
    },

    /// The associated cancellation token requested termination.
    Cancelled,

    /// Accounting attempted to release more memory than was reserved.
    ReleaseOverflow,

    /// The underlying allocator rejected a fallible allocation request.
    AllocationFailed,

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
            } => write!(
                f,
                "memory budget exceeded: requested={requested}, \
                 allocated={allocated}, available={available}, limit={limit}"
            ),

            Self::QuotaExceeded {
                requested,
                reserved,
                quota,
            } => write!(
                f,
                "memory quota exceeded: requested={requested}, \
                 reserved={reserved}, quota={quota}"
            ),

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

            Self::ReleaseOverflow => {
                f.write_str("memory release accounting underflow")
            }

            Self::AllocationFailed => {
                f.write_str("underlying allocation failed after policy admission")
            }

            Self::EvictionInsufficient {
                requested,
                reclaimed,
            } => write!(
                f,
                "eviction reclaimed insufficient memory: \
                 requested={requested}, reclaimed={reclaimed}"
            ),
        }
    }
}

impl std::error::Error for MemoryError {}

// ============================================================================
// Canonical error integration
// ============================================================================

impl From<MemoryError> for QecError {
    fn from(error: MemoryError) -> Self {
        match error {
            MemoryError::InvalidBudget { reason } => {
                QecError::InvalidInput {
                    message: format!("invalid memory budget: {reason}"),
                }
            }

            MemoryError::BudgetExceeded {
                requested,
                allocated,
                limit,
                ..
            } => QecError::MemoryLimitExceeded {
                requested_bytes: requested,
                current_bytes: allocated,
                limit_bytes: limit,
                message: "memory budget exceeded".to_owned(),
            },

            MemoryError::QuotaExceeded {
                requested,
                reserved,
                quota,
            } => QecError::MemoryLimitExceeded {
                requested_bytes: requested,
                current_bytes: reserved,
                limit_bytes: quota,
                message: "memory operation quota exceeded".to_owned(),
            },

            MemoryError::ArithmeticOverflow
            | MemoryError::CapacityOverflow => QecError::NumericalFailure {
                operation:
                    super::errors::NumericalOperation::MemorySizeCalculation,
                message: error.to_string(),
            },

            MemoryError::BufferFull { capacity } => {
                QecError::ResourceLimitExceeded {
                    resource: ResourceKind::StreamBuffer,
                    requested: capacity as u128 + 1,
                    current: capacity as u128,
                    limit: capacity as u128,
                    message: "bounded memory buffer is full".to_owned(),
                }
            }

            MemoryError::BufferEmpty => QecError::InvalidInput {
                message: "bounded buffer is empty".to_owned(),
            },

            MemoryError::InvalidOperation { reason } => {
                QecError::InvalidInput {
                    message: reason.to_owned(),
                }
            }

            MemoryError::Cancelled => QecError::CancellationRequested {
                message: "memory operation cancelled".to_owned(),
            },

            MemoryError::ReleaseOverflow => {
                QecError::InternalInvariantViolation {
                    invariant: "memory release cannot exceed reservation".to_owned(),
                    message: "memory accounting state is inconsistent".to_owned(),
                }
            }

            MemoryError::AllocationFailed => {
                QecError::MemoryLimitExceeded {
                    requested_bytes: 0,
                    current_bytes: 0,
                    limit_bytes: 0,
                    message:
                        "underlying allocator rejected a fallible allocation"
                            .to_owned(),
                }
            }

            MemoryError::EvictionInsufficient {
                requested,
                reclaimed,
            } => QecError::ResourceLimitExceeded {
                resource: ResourceKind::MemoryBytes,
                requested: requested as u128,
                current: reclaimed as u128,
                limit: reclaimed as u128,
                message: "requested eviction could not be satisfied".to_owned(),
            },
        }
    }
}

// ============================================================================
// Memory budget
// ============================================================================

/// Memory-specific policy derived from canonical [`QecLimits`].
///
/// This is deliberately narrower than `QecLimits`.
///
/// It must never become a second independent QEC policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryBudget {
    /// Maximum simultaneously reserved bytes.
    pub max_bytes: u64,

    /// Optional per-operation memory quota.
    pub max_operation_bytes: Option<u64>,

    /// Optional memory limit for one arena.
    pub max_arena_bytes: Option<u64>,

    /// Maximum number of elements admitted to one bounded buffer.
    pub max_buffer_elements: usize,

    /// Whether higher-level code may report completed eviction.
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
    /// Creates an explicitly unlimited application-level policy.
    pub const fn unlimited() -> Self {
        Self {
            max_bytes: UNLIMITED_MEMORY,
            max_operation_bytes: None,
            max_arena_bytes: None,
            max_buffer_elements: usize::MAX,
            eviction_enabled: true,
        }
    }

    /// Derives memory policy from canonical QEC limits.
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

    /// Validates internal policy consistency.
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

        if self
            .max_operation_bytes
            .is_some_and(|value| value > self.max_bytes)
        {
            return Err(MemoryError::InvalidBudget {
                reason:
                    "operation memory limit cannot exceed global memory limit",
            });
        }

        if self
            .max_arena_bytes
            .is_some_and(|value| value > self.max_bytes)
        {
            return Err(MemoryError::InvalidBudget {
                reason:
                    "arena memory limit cannot exceed global memory limit",
            });
        }

        Ok(())
    }

    /// Whether this policy deliberately removes its finite application limit.
    pub const fn is_unlimited(&self) -> bool {
        self.max_bytes == UNLIMITED_MEMORY
    }
}

// ============================================================================
// Memory statistics
// ============================================================================

/// Immutable point-in-time memory statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemorySnapshot {
    /// Currently reserved bytes.
    pub allocated_bytes: u64,

    /// Highest simultaneous reservation observed.
    pub peak_bytes: u64,

    /// Successful reservation operations.
    pub allocation_count: u64,

    /// Successful release operations.
    pub release_count: u64,

    /// Reservations rejected by policy/accounting.
    pub failed_allocations: u64,

    /// Lifetime successfully reserved bytes.
    pub cumulative_allocated_bytes: u64,

    /// Lifetime successfully released bytes.
    pub cumulative_released_bytes: u64,

    /// Number of reported eviction operations.
    pub eviction_requests: u64,

    /// Bytes reported as reclaimed by higher-level eviction.
    pub evicted_bytes: u64,
}

#[derive(Debug, Default)]
struct MemoryCounters {
    allocated: AtomicU64,
    peak: AtomicU64,
    allocations: AtomicU64,
    releases: AtomicU64,
    failures: AtomicU64,
    cumulative_allocated: AtomicU64,
    cumulative_released: AtomicU64,
    eviction_requests: AtomicU64,
    evicted: AtomicU64,
}

#[derive(Debug)]
struct MemoryState {
    budget: MemoryBudget,
    counters: MemoryCounters,
}

// ============================================================================
// Memory manager
// ============================================================================

/// Thread-safe memory reservation manager.
///
/// Cloning this object shares the exact same accounting state.
#[derive(Debug, Clone)]
pub struct MemoryManager {
    state: Arc<MemoryState>,
    cancellation: CancellationToken,
}

impl MemoryManager {
    /// Creates a manager with its own cancellation token.
    pub fn new(budget: MemoryBudget) -> Result<Self, MemoryError> {
        budget.validate()?;

        Ok(Self {
            state: Arc::new(MemoryState {
                budget,
                counters: MemoryCounters::default(),
            }),
            cancellation: CancellationToken::new(),
        })
    }

    /// Creates a manager from canonical QEC limits.
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

    /// Creates a manager attached to an existing operation cancellation token.
    pub fn with_cancellation(
        budget: MemoryBudget,
        cancellation: CancellationToken,
    ) -> Result<Self, MemoryError> {
        budget.validate()?;

        Ok(Self {
            state: Arc::new(MemoryState {
                budget,
                counters: MemoryCounters::default(),
            }),
            cancellation,
        })
    }

    /// Creates a manager from QEC limits and an existing cancellation token.
    pub fn from_qec_limits_with_cancellation(
        limits: &QecLimits,
        cancellation: CancellationToken,
    ) -> Result<Self, MemoryError> {
        limits
            .validate()
            .map_err(|_| MemoryError::InvalidBudget {
                reason: "invalid canonical QEC resource policy",
            })?;

        Self::with_cancellation(
            MemoryBudget::from_qec_limits(limits),
            cancellation,
        )
    }

    /// Returns the memory policy.
    pub const fn budget(&self) -> MemoryBudget {
        self.state.budget
    }

    /// Returns the shared cancellation token.
    pub fn cancellation(&self) -> CancellationToken {
        self.cancellation.clone()
    }

    /// Checks cancellation.
    pub fn check(&self) -> Result<(), MemoryError> {
        self.cancellation
            .check()
            .map_err(|_| MemoryError::Cancelled)
    }

    /// Returns whether this manager's execution context is cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }

    /// Compatibility helper for locally-owned managers.
    ///
    /// Cancellation remains owned by `cancellation.rs`; this merely delegates
    /// to the token rather than implementing another cancellation mechanism.
    pub fn cancel(&self) -> bool {
        self.cancellation.request()
    }

    /// Returns a point-in-time snapshot.
    pub fn snapshot(&self) -> MemorySnapshot {
        let counters = &self.state.counters;

        MemorySnapshot {
            allocated_bytes: counters.allocated.load(Ordering::Acquire),
            peak_bytes: counters.peak.load(Ordering::Acquire),
            allocation_count: counters.allocations.load(Ordering::Acquire),
            release_count: counters.releases.load(Ordering::Acquire),
            failed_allocations: counters.failures.load(Ordering::Acquire),
            cumulative_allocated_bytes: counters
                .cumulative_allocated
                .load(Ordering::Acquire),
            cumulative_released_bytes: counters
                .cumulative_released
                .load(Ordering::Acquire),
            eviction_requests: counters
                .eviction_requests
                .load(Ordering::Acquire),
            evicted_bytes: counters.evicted.load(Ordering::Acquire),
        }
    }

    /// Returns currently available global memory.
    pub fn available_bytes(&self) -> u64 {
        let allocated = self
            .state
            .counters
            .allocated
            .load(Ordering::Acquire);

        self.state
            .budget
            .max_bytes
            .saturating_sub(allocated)
    }

    /// Checks whether a reservation can currently be admitted.
    pub fn can_reserve(&self, bytes: u64) -> bool {
        self.check().is_ok() && self.available_bytes() >= bytes
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

    /// Reserves memory with an explicit operation quota.
    pub fn reserve_with_quota(
        &self,
        bytes: u64,
        quota: Option<u64>,
    ) -> Result<MemoryReservation, MemoryError> {
        self.reserve_inner(bytes, quota, None)
    }

    /// Creates a stricter per-operation memory scope.
    pub fn scope(
        &self,
        quota: u64,
    ) -> Result<MemoryScope, MemoryError> {
        if quota == 0 {
            return Err(MemoryError::InvalidBudget {
                reason: "scope quota must be greater than zero",
            });
        }

        if !self.budget().is_unlimited()
            && quota > self.budget().max_bytes
        {
            return Err(MemoryError::InvalidBudget {
                reason:
                    "scope quota cannot exceed global memory limit",
            });
        }

        Ok(MemoryScope {
            manager: self.clone(),
            state: Arc::new(ScopeState {
                quota,
                reserved: AtomicU64::new(0),
            }),
        })
    }

    fn reserve_inner(
        &self,
        bytes: u64,
        quota: Option<u64>,
        scope: Option<Arc<ScopeState>>,
    ) -> Result<MemoryReservation, MemoryError> {
        self.check()?;

        if bytes == 0 {
            return Err(MemoryError::InvalidOperation {
                reason:
                    "zero-byte reservations are not permitted",
            });
        }

        if let Some(limit) = quota {
            if bytes > limit {
                self.record_failure();

                return Err(MemoryError::QuotaExceeded {
                    requested: bytes,
                    reserved: 0,
                    quota: limit,
                });
            }
        }

        if let Some(scope_state) = scope.as_ref() {
            reserve_scope(scope_state, bytes)?;
        }

        if let Err(error) = self.try_global_reserve(bytes) {
            if let Some(scope_state) = scope.as_ref() {
                let _ = release_scope(scope_state, bytes);
            }

            return Err(error);
        }

        Ok(MemoryReservation {
            manager: self.clone(),
            scope,
            bytes,
            released: false,
        })
    }

    fn try_global_reserve(
        &self,
        bytes: u64,
    ) -> Result<(), MemoryError> {
        let current = &self.state.counters.allocated;
        let maximum = self.state.budget.max_bytes;

        let result = current.fetch_update(
            Ordering::AcqRel,
            Ordering::Acquire,
            |old| {
                let new = old.checked_add(bytes)?;

                if maximum != UNLIMITED_MEMORY && new > maximum {
                    return None;
                }

                Some(new)
            },
        );

        match result {
            Ok(old) => {
                let new = old
                    .checked_add(bytes)
                    .ok_or(MemoryError::ArithmeticOverflow)?;

                update_peak(
                    &self.state.counters.peak,
                    new,
                );

                checked_fetch_add(
                    &self.state.counters.allocations,
                    1,
                )?;

                checked_fetch_add(
                    &self.state.counters.cumulative_allocated,
                    bytes,
                )?;

                Ok(())
            }

            Err(_) => {
                self.record_failure();

                let allocated =
                    current.load(Ordering::Acquire);

                let available =
                    maximum.saturating_sub(allocated);

                if maximum != UNLIMITED_MEMORY
                    && bytes > available
                {
                    Err(MemoryError::BudgetExceeded {
                        requested: bytes,
                        allocated,
                        available,
                        limit: maximum,
                    })
                } else {
                    Err(MemoryError::ArithmeticOverflow)
                }
            }
        }
    }

    fn record_failure(&self) {
        let _ = self.state.counters.failures.fetch_update(
            Ordering::AcqRel,
            Ordering::Acquire,
            |value| value.checked_add(1),
        );
    }

    fn release_bytes(
        &self,
        bytes: u64,
    ) -> Result<(), MemoryError> {
        let current = &self.state.counters.allocated;

        current
            .fetch_update(
                Ordering::AcqRel,
                Ordering::Acquire,
                |old| old.checked_sub(bytes),
            )
            .map_err(|_| MemoryError::ReleaseOverflow)?;

        checked_fetch_add(
            &self.state.counters.releases,
            1,
        )?;

        checked_fetch_add(
            &self.state.counters.cumulative_released,
            bytes,
        )?;

        Ok(())
    }

    /// Reports memory that was actually reclaimed by higher-level eviction.
    ///
    /// This does not directly modify `allocated_bytes`.
    /// Reservations must still be released by their owners.
    pub fn report_eviction(
        &self,
        requested: u64,
        reclaimed: u64,
    ) -> Result<(), MemoryError> {
        self.check()?;

        if !self.budget().eviction_enabled {
            return Err(MemoryError::InvalidOperation {
                reason: "eviction is disabled",
            });
        }

        if reclaimed < requested {
            return Err(MemoryError::EvictionInsufficient {
                requested,
                reclaimed,
            });
        }

        checked_fetch_add(
            &self.state.counters.eviction_requests,
            1,
        )?;

        checked_fetch_add(
            &self.state.counters.evicted,
            reclaimed,
        )?;

        Ok(())
    }
}

// ============================================================================
// Memory scope
// ============================================================================

#[derive(Debug)]
struct ScopeState {
    quota: u64,
    reserved: AtomicU64,
}

/// Per-operation memory scope.
///
/// A scope can only tighten the global memory policy.
#[derive(Debug, Clone)]
pub struct MemoryScope {
    manager: MemoryManager,
    state: Arc<ScopeState>,
}

impl MemoryScope {
    /// Returns the scope quota.
    pub const fn quota(&self) -> u64 {
        self.state.quota
    }

    /// Returns currently reserved scope memory.
    pub fn reserved_bytes(&self) -> u64 {
        self.state.reserved.load(Ordering::Acquire)
    }

    /// Returns currently available scope memory.
    pub fn available_bytes(&self) -> u64 {
        self.state
            .quota
            .saturating_sub(self.reserved_bytes())
    }

    /// Reserves memory inside this scope.
    pub fn reserve(
        &self,
        bytes: u64,
    ) -> Result<MemoryReservation, MemoryError> {
        self.manager.reserve_inner(
            bytes,
            Some(self.state.quota),
            Some(Arc::clone(&self.state)),
        )
    }

    /// Returns a shared manager handle.
    pub fn manager(&self) -> MemoryManager {
        self.manager.clone()
    }
}

fn reserve_scope(
    scope: &ScopeState,
    bytes: u64,
) -> Result<(), MemoryError> {
    scope
        .reserved
        .fetch_update(
            Ordering::AcqRel,
            Ordering::Acquire,
            |old| {
                let new = old.checked_add(bytes)?;

                if new > scope.quota {
                    return None;
                }

                Some(new)
            },
        )
        .map(|_| ())
        .map_err(|_| MemoryError::QuotaExceeded {
            requested: bytes,
            reserved: scope.reserved.load(Ordering::Acquire),
            quota: scope.quota,
        })
}

fn release_scope(
    scope: &ScopeState,
    bytes: u64,
) -> Result<(), MemoryError> {
    scope
        .reserved
        .fetch_update(
            Ordering::AcqRel,
            Ordering::Acquire,
            |old| old.checked_sub(bytes),
        )
        .map(|_| ())
        .map_err(|_| MemoryError::ReleaseOverflow)
}

// ============================================================================
// Reservation
// ============================================================================

/// RAII memory reservation.
///
/// The reservation must remain alive for as long as the corresponding
/// allocation is considered admitted by the QEC subsystem.
#[must_use = "a memory reservation must remain alive while the allocation is live"]
pub struct MemoryReservation {
    manager: MemoryManager,
    scope: Option<Arc<ScopeState>>,
    bytes: u64,
    released: bool,
}

impl fmt::Debug for MemoryReservation {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.debug_struct("MemoryReservation")
            .field("bytes", &self.bytes)
            .field("released", &self.released)
            .finish()
    }
}

impl MemoryReservation {
    /// Returns reserved bytes.
    pub const fn bytes(&self) -> u64 {
        self.bytes
    }

    /// Returns whether the reservation has already been released.
    pub const fn is_released(&self) -> bool {
        self.released
    }

    /// Explicitly releases this reservation.
    pub fn release(
        mut self,
    ) -> Result<(), MemoryError> {
        self.release_inner()
    }

    fn release_inner(
        &mut self,
    ) -> Result<(), MemoryError> {
        if self.released {
            return Ok(());
        }

        self.manager.release_bytes(self.bytes)?;

        if let Some(scope) = self.scope.as_ref() {
            release_scope(scope, self.bytes)?;
        }

        self.released = true;

        Ok(())
    }
}

impl Drop for MemoryReservation {
    fn drop(&mut self) {
        if self.released {
            return;
        }

        let _ = self.manager.release_bytes(self.bytes);

        if let Some(scope) = self.scope.as_ref() {
            let _ = release_scope(scope, self.bytes);
        }

        self.released = true;
    }
}

// ============================================================================
// Checked allocation helpers
// ============================================================================

fn checked_layout_bytes<T>(
    elements: usize,
) -> Result<u64, MemoryError> {
    let element_size = size_of::<T>() as u64;

    if element_size == 0 {
        return Ok(0);
    }

    (elements as u64)
        .checked_mul(element_size)
        .ok_or(MemoryError::CapacityOverflow)
}

fn checked_fetch_add(
    cell: &AtomicU64,
    amount: u64,
) -> Result<u64, MemoryError> {
    cell.fetch_update(
        Ordering::AcqRel,
        Ordering::Acquire,
        |value| value.checked_add(amount),
    )
    .map_err(|_| MemoryError::ArithmeticOverflow)
}

fn update_peak(
    peak: &AtomicU64,
    value: u64,
) {
    let _ = peak.fetch_update(
        Ordering::AcqRel,
        Ordering::Acquire,
        |old| {
            if value > old {
                Some(value)
            } else {
                None
            }
        },
    );
}

// ============================================================================
// Accounted vector
// ============================================================================

/// A vector whose requested capacity is admitted before allocation.
///
/// The reservation is held for the lifetime of the vector.
#[derive(Debug)]
pub struct AccountedVec<T> {
    data: Vec<T>,
    reservation: Option<MemoryReservation>,
}

impl<T> AccountedVec<T> {
    /// Creates an accounted vector using the manager's operation policy.
    pub fn with_capacity(
        manager: &MemoryManager,
        capacity: usize,
    ) -> Result<Self, MemoryError> {
        let bytes = checked_layout_bytes::<T>(capacity)?;

        let reservation = if bytes == 0 {
            None
        } else {
            Some(manager.reserve(bytes)?)
        };

        let mut data = Vec::new();

        data.try_reserve_exact(capacity)
            .map_err(|_| MemoryError::AllocationFailed)?;

        Ok(Self {
            data,
            reservation,
        })
    }

    /// Creates an accounted vector inside a memory scope.
    pub fn with_capacity_in(
        scope: &MemoryScope,
        capacity: usize,
    ) -> Result<Self, MemoryError> {
        let bytes = checked_layout_bytes::<T>(capacity)?;

        let reservation = if bytes == 0 {
            None
        } else {
            Some(scope.reserve(bytes)?)
        };

        let mut data = Vec::new();

        data.try_reserve_exact(capacity)
            .map_err(|_| MemoryError::AllocationFailed)?;

        Ok(Self {
            data,
            reservation,
        })
    }

    /// Pushes an element without allowing unaccounted capacity growth.
    pub fn push(
        &mut self,
        value: T,
    ) -> Result<(), MemoryError> {
        if self.data.len() == self.data.capacity() {
            return Err(MemoryError::CapacityOverflow);
        }

        self.data.push(value);

        Ok(())
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn reservation_bytes(&self) -> u64 {
        self.reservation
            .as_ref()
            .map_or(0, MemoryReservation::bytes)
    }

    /// Consumes the wrapper and returns the underlying vector.
    ///
    /// The memory reservation is dropped after the vector is moved out.
    /// Therefore callers should treat the returned vector as outside this
    /// module's accounting boundary.
    pub fn into_inner(self) -> Vec<T> {
        self.data
    }
}

impl<T> std::ops::Deref for AccountedVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> std::ops::DerefMut for AccountedVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

// ============================================================================
// Bounded buffer
// ============================================================================

/// Bounded FIFO buffer with memory admission.
///
/// This is intended for `streaming.rs` and other incremental QEC pipelines.
#[derive(Debug)]
pub struct BoundedBuffer<T> {
    queue: VecDeque<T>,
    manager: MemoryManager,
    reservation: Option<MemoryReservation>,
    element_bytes: u64,
    capacity: usize,
}

impl<T> BoundedBuffer<T> {
    /// Creates a bounded buffer.
    ///
    /// The complete requested element capacity is reserved before allocation.
    pub fn new(
        manager: &MemoryManager,
        capacity: usize,
    ) -> Result<Self, MemoryError> {
        if capacity == 0 {
            return Err(MemoryError::InvalidBudget {
                reason:
                    "buffer capacity must be greater than zero",
            });
        }

        if capacity > manager.budget().max_buffer_elements {
            return Err(MemoryError::BudgetExceeded {
                requested: capacity as u64,
                allocated: 0,
                available:
                    manager.budget().max_buffer_elements as u64,
                limit:
                    manager.budget().max_buffer_elements as u64,
            });
        }

        let element_bytes = size_of::<T>() as u64;

        let reservation = if element_bytes == 0 {
            None
        } else {
            let bytes = element_bytes
                .checked_mul(capacity as u64)
                .ok_or(MemoryError::CapacityOverflow)?;

            Some(manager.reserve(bytes)?)
        };

        let mut queue = VecDeque::new();

        queue
            .try_reserve_exact(capacity)
            .map_err(|_| MemoryError::AllocationFailed)?;

        Ok(Self {
            queue,
            manager: manager.clone(),
            reservation,
            element_bytes,
            capacity,
        })
    }

    /// Pushes one element.
    pub fn push(
        &mut self,
        value: T,
    ) -> Result<(), MemoryError> {
        self.manager.check()?;

        if self.queue.len() >= self.capacity {
            return Err(MemoryError::BufferFull {
                capacity: self.capacity,
            });
        }

        self.queue.push_back(value);

        Ok(())
    }

    /// Removes the oldest element.
    pub fn pop(&mut self) -> Result<T, MemoryError> {
        self.queue
            .pop_front()
            .ok_or(MemoryError::BufferEmpty)
    }

    /// Non-erroring pop.
    pub fn try_pop(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.queue.len() == self.capacity
    }

    pub fn remaining_capacity(&self) -> usize {
        self.capacity - self.queue.len()
    }

    pub fn element_bytes(&self) -> u64 {
        self.element_bytes
    }

    pub fn reserved_bytes(&self) -> u64 {
        self.reservation
            .as_ref()
            .map_or(0, MemoryReservation::bytes)
    }
}

// ============================================================================
// Memory arena
// ============================================================================

/// Byte arena with admission-controlled growth.
///
/// Each successful capacity expansion obtains an additional RAII reservation.
/// This avoids silently growing beyond the configured memory policy.
#[derive(Debug)]
pub struct MemoryArena {
    bytes: Vec<u8>,
    reservations: Vec<MemoryReservation>,
    manager: MemoryManager,
    max_bytes: u64,
}

impl MemoryArena {
    /// Creates an arena with an optional initial capacity.
    pub fn new(
        manager: &MemoryManager,
        initial_capacity: usize,
    ) -> Result<Self, MemoryError> {
        let max_bytes = manager
            .budget()
            .max_arena_bytes
            .unwrap_or(UNLIMITED_MEMORY);

        let requested = initial_capacity as u64;

        if requested > max_bytes {
            return Err(MemoryError::BudgetExceeded {
                requested,
                allocated: 0,
                available: max_bytes,
                limit: max_bytes,
            });
        }

        let mut arena = Self {
            bytes: Vec::new(),
            reservations: Vec::new(),
            manager: manager.clone(),
            max_bytes,
        };

        if initial_capacity > 0 {
            arena.reserve_capacity(initial_capacity)?;
        }

        Ok(arena)
    }

    /// Admits and allocates at least the requested capacity.
    pub fn reserve_capacity(
        &mut self,
        requested_capacity: usize,
    ) -> Result<(), MemoryError> {
        if requested_capacity <= self.bytes.capacity() {
            return Ok(());
        }

        let requested = requested_capacity as u64;

        if requested > self.max_bytes {
            return Err(MemoryError::BudgetExceeded {
                requested,
                allocated: self.bytes.capacity() as u64,
                available: self
                    .max_bytes
                    .saturating_sub(self.bytes.capacity() as u64),
                limit: self.max_bytes,
            });
        }

        let current_capacity = self.bytes.capacity() as u64;

        let delta = requested
            .checked_sub(current_capacity)
            .ok_or(MemoryError::ArithmeticOverflow)?;

        let reservation = self.manager.reserve(delta)?;

        self.bytes
            .try_reserve_exact(
                requested_capacity - self.bytes.capacity(),
            )
            .map_err(|_| MemoryError::AllocationFailed)?;

        self.reservations.push(reservation);

        Ok(())
    }

    /// Appends bytes, growing the arena through the admission boundary.
    pub fn append(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), MemoryError> {
        self.manager.check()?;

        let required = self
            .bytes
            .len()
            .checked_add(bytes.len())
            .ok_or(MemoryError::CapacityOverflow)?;

        if required > self.bytes.capacity() {
            self.reserve_capacity(required)?;
        }

        self.bytes.extend_from_slice(bytes);

        Ok(())
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn capacity(&self) -> usize {
        self.bytes.capacity()
    }

    pub fn reserved_bytes(&self) -> u64 {
        self.reservations
            .iter()
            .fold(0u64, |total, reservation| {
                total.saturating_add(reservation.bytes())
            })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::error_correction::limits::QecLimits;

    fn test_budget() -> MemoryBudget {
        MemoryBudget {
            max_bytes: 1024,
            max_operation_bytes: None,
            max_arena_bytes: Some(1024),
            max_buffer_elements: 16,
            eviction_enabled: true,
        }
    }

    #[test]
    fn reservation_is_released_by_drop() {
        let manager =
            MemoryManager::new(test_budget()).expect("valid budget");

        let reservation =
            manager.reserve(512).expect("reservation succeeds");

        assert_eq!(
            manager.snapshot().allocated_bytes,
            512
        );

        drop(reservation);

        assert_eq!(
            manager.snapshot().allocated_bytes,
            0
        );
    }

    #[test]
    fn cloned_managers_share_accounting() {
        let manager =
            MemoryManager::new(test_budget()).expect("valid budget");

        let clone = manager.clone();

        let _reservation =
            clone.reserve(128).expect("reservation succeeds");

        assert_eq!(
            manager.snapshot().allocated_bytes,
            128
        );
    }

    #[test]
    fn global_budget_is_enforced_atomically() {
        let manager =
            MemoryManager::new(test_budget()).expect("valid budget");

        let _first =
            manager.reserve(768).expect("first reservation");

        let result = manager.reserve(512);

        assert!(matches!(
            result,
            Err(MemoryError::BudgetExceeded { .. })
        ));

        assert_eq!(
            manager.snapshot().allocated_bytes,
            768
        );
    }

    #[test]
    fn operation_scope_is_stricter_than_global_policy() {
        let manager =
            MemoryManager::new(test_budget()).expect("valid budget");

        let scope =
            manager.scope(256).expect("valid scope");

        let result = scope.reserve(257);

        assert!(matches!(
            result,
            Err(MemoryError::QuotaExceeded { .. })
        ));
    }

    #[test]
    fn scope_accounting_is_released_with_reservation() {
        let manager =
            MemoryManager::new(test_budget()).expect("valid budget");

        let scope =
            manager.scope(256).expect("valid scope");

        {
            let _reservation =
                scope.reserve(128).expect("reservation");
            assert_eq!(scope.reserved_bytes(), 128);
        }

        assert_eq!(scope.reserved_bytes(), 0);
        assert_eq!(
            manager.snapshot().allocated_bytes,
            0
        );
    }

    #[test]
    fn cancellation_is_honoured() {
        let token =
            CancellationToken::with_timeout(
                std::time::Duration::ZERO,
            );

        let manager =
            MemoryManager::with_cancellation(
                test_budget(),
                token,
            )
            .expect("valid manager");

        assert!(matches!(
            manager.reserve(1),
            Err(MemoryError::Cancelled)
        ));
    }

    #[test]
    fn bounded_buffer_never_exceeds_capacity() {
        let manager =
            MemoryManager::new(test_budget()).expect("valid");

        let mut buffer =
            BoundedBuffer::<u64>::new(&manager, 2)
                .expect("buffer");

        buffer.push(1).expect("push");
        buffer.push(2).expect("push");

        assert!(matches!(
            buffer.push(3),
            Err(MemoryError::BufferFull { .. })
        ));
    }

    #[test]
    fn accounted_vector_does_not_allow_unaccounted_growth() {
        let manager =
            MemoryManager::new(test_budget()).expect("valid");

        let mut vector =
            AccountedVec::<u64>::with_capacity(
                &manager,
                2,
            )
            .expect("vector");

        vector.push(1).expect("push");
        vector.push(2).expect("push");

        assert!(vector.push(3).is_err());
    }

    #[test]
    fn arena_growth_is_accounted() {
        let manager =
            MemoryManager::new(test_budget()).expect("valid");

        let mut arena =
            MemoryArena::new(&manager, 8)
                .expect("arena");

        arena.append(&[1, 2, 3, 4])
            .expect("append");

        assert_eq!(arena.len(), 4);
        assert!(arena.reserved_bytes() >= 8);
    }

    #[test]
    fn canonical_limits_create_memory_budget() {
        let mut limits = QecLimits::new();
        limits.max_memory_bytes = 4096;
        limits.max_stream_buffer_events = 32;

        let budget =
            MemoryBudget::from_qec_limits(&limits);

        assert_eq!(budget.max_bytes, 4096);
        assert_eq!(
            budget.max_buffer_elements,
            32
        );
    }

    #[test]
    fn snapshot_tracks_peak_memory() {
        let manager =
            MemoryManager::new(test_budget()).expect("valid");

        {
            let _a =
                manager.reserve(200).expect("reservation");

            let _b =
                manager.reserve(300).expect("reservation");

            assert_eq!(
                manager.snapshot().allocated_bytes,
                500
            );

            assert_eq!(
                manager.snapshot().peak_bytes,
                500
            );
        }

        assert_eq!(
            manager.snapshot().allocated_bytes,
            0
        );

        assert_eq!(
            manager.snapshot().peak_bytes,
            500
        );
    }
}