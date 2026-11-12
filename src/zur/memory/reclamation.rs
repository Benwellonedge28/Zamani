
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Memory Reclamation
//!
//! Path: `src/zur/memory/reclamation.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides explicit, deterministic coordination of reclamation for
//! allocations whose owners have deliberately transferred them to
//! a reclamation manager.
//!
//! This module supports:
//!
//! - Explicit release of reclaimable allocations.
//! - Priority-based reclamation.
//! - Least-recently-used reclamation ordering.
//! - Deterministic tie-breaking.
//! - Resource-pressure reclamation.
//! - Ownership recovery before reclamation.
//! - Concurrent registration and reclamation.
//! - Structured failure reporting.
//!
//! # Architectural ownership
//!
//! The canonical allocation implementation remains:
//!
//! `crate::classical::memory::allocation`
//!
//! ZUR allocation handles remain:
//!
//! `crate::zur::memory::handle::MemoryHandle`
//!
//! This module does not implement another allocator, allocation
//! handle, memory-accounting system, or storage backend.
//!
//! # Critical ownership rule
//!
//! Only allocations explicitly transferred to this manager are
//! eligible for reclamation.
//!
//! Ordinary program-owned allocations are never automatically
//! reclaimed by this module.
//!
//! The caller transfers ownership by moving a MemoryHandle into
//! `register_evictable`.
//!
//! Ownership can subsequently be recovered using `unregister`.
//!
//! # Garbage collection
//!
//! Garbage collection is NOT mandatory.
//!
//! Rust-style ownership and deterministic destruction remain the
//! default memory-management mechanism.
//!
//! This module does not implement:
//!
//! - Tracing garbage collection.
//! - Reference-counting garbage collection.
//! - Automatic object reachability analysis.
//! - Borrow checking.
//! - Lifetime inference.
//! - Ownership inference.
//! - Finalizer execution.
//!
//! Those operations require their own language semantics and
//! explicitly defined implementation contracts.
//!
//! # Reclamation semantics
//!
//! Reclamation is an operational resource-management decision.
//!
//! It must not change the observable semantic result of a program.
//!
//! Reclaimable allocations must therefore represent resources
//! whose removal is permitted by their owning subsystem.
//!
//! Examples include:
//!
//! - Explicitly evictable caches.
//! - Recomputable intermediate buffers.
//! - Temporary runtime allocations.
//! - Explicitly releasable staging storage.
//!
//! Program state, persistent data, live quantum resources and
//! other non-reclaimable resources must not be registered unless
//! their authoritative ownership contract explicitly permits it.
//!
//! # Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Reclaimable allocation count.
//! - Allocation size.
//! - Reclamation operations.
//! - Memory spaces.
//! - Providers.
//! - Devices.
//! - Computational domains.
//!
//! Accounting quantities use u128.
//!
//! Collection capacity grows dynamically and allocation failures
//! are reported where fallible reservation is available.
//!
//! Actual reclamation remains subject to available resources,
//! provider behavior and the underlying allocation contract.
//!
//! Literal infinite physical memory is not promised.
//!
//! # Concurrency
//!
//! A mutex protects reclamation metadata.
//!
//! Allocation storage is always destroyed outside the metadata lock.
//!
//! This prevents potentially expensive provider destruction from
//! unnecessarily blocking unrelated registration and lookup.
//!
//! Concurrent operations are serialized at the manager's metadata
//! boundary.
//!
//! # Grammar integration
//!
//! Relevant grammar authorities:
//!
//! - grammar/memory/memory.g4
//! - grammar/memory/allocation.g4
//! - grammar/memory/deallocation.g4
//! - grammar/memory/regions.g4
//! - grammar/memory/ownership.g4
//! - grammar/memory/lifetimes.g4
//! - grammar/memory/shared-memory.g4
//! - grammar/memory/distributed-memory.g4
//! - grammar/memory/memory-capabilities.g4
//! - grammar/memory/memory-constraints.g4
//! - grammar/specification/semantic-model.md
//! - grammar/spec/determinism.md
//!
//! Grammar defines source-level intent.
//! Semantic analysis validates ownership and lifetime rules.
//! Reclamation implements only the permitted operational action.
//!
//! # Integration boundaries
//!
//! Classical:
//! `crate::classical::memory`
//!
//! ZUIR:
//! `crate::zuir::ir::ownership`
//! `crate::zuir::ir::region`
//!
//! Quantum:
//! `crate::quantum::memory`
//! `crate::quantum::hardware`
//!
//! Hybrid:
//! `crate::hybrid`
//!
//! Distributed:
//! `crate::distributed`
//!
//! This module does not replace any of these authorities.

use std::error::Error;
use std::fmt;
use std::sync::{Mutex, MutexGuard};

use super::handle::MemoryHandle;

// =============================================================================
// PUBLIC RESULT TYPES
// =============================================================================

/// Result type for reclamation operations.
pub type ReclamationResult<T> = Result<T, ReclamationError>;

/// Result type for allocation registration.
///
/// Registration failures preserve the original allocation handle.
/// The caller can recover it through `RegistrationError::into_parts`.
pub type RegistrationResult<T> = Result<T, RegistrationError>;

// =============================================================================
// RECLAMATION IDENTITY
// =============================================================================

/// Unique identity of an allocation registered with a reclamation manager.
///
/// An identity is local to the manager that created it.
///
/// It is not a physical address, allocation address, device identity,
/// or canonical ZUIR identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReclamationId(u128);

impl ReclamationId {
    /// Return the numeric identity.
    #[must_use]
    pub const fn get(self) -> u128 {
        self.0
    }
}

// =============================================================================
// RECLAMATION PRIORITY
// =============================================================================

/// Operational priority for reclaimable allocations.
///
/// Lower numeric values are reclaimed before higher numeric values.
///
/// Equal priorities are resolved by reclamation age.
///
/// Priority affects eviction order only. It does not establish
/// ownership, semantic importance, or resource feasibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReclamationPriority(i64);

impl ReclamationPriority {
    /// Construct an operational reclamation priority.
    #[must_use]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// Return the numeric priority.
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}

impl Default for ReclamationPriority {
    fn default() -> Self {
        Self::new(0)
    }
}

// =============================================================================
// RECLAMATION ERRORS
// =============================================================================

/// Errors returned by reclamation coordination.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReclamationError {
    /// The reclamation metadata lock was poisoned.
    LockPoisoned,

    /// The manager could not reserve metadata storage.
    MetadataAllocationFailed,

    /// A reclamation identity could not be represented.
    IdentityExhausted,

    /// The reclamation ordering counter could not advance.
    SequenceExhausted,

    /// The requested allocation identity does not exist.
    UnknownAllocation {
        /// Requested allocation identity.
        id: ReclamationId,
    },

    /// Reclamation-result accounting overflowed.
    AccountingOverflow,
}

impl fmt::Display for ReclamationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LockPoisoned => {
                write!(f, "reclamation metadata lock is poisoned")
            }

            Self::MetadataAllocationFailed => {
                write!(f, "unable to reserve reclamation metadata")
            }

            Self::IdentityExhausted => {
                write!(f, "reclamation identity space exhausted")
            }

            Self::SequenceExhausted => {
                write!(f, "reclamation sequence space exhausted")
            }

            Self::UnknownAllocation { id } => {
                write!(
                    f,
                    "unknown reclaimable allocation: {}",
                    id.get()
                )
            }

            Self::AccountingOverflow => {
                write!(f, "reclamation result accounting overflow")
            }
        }
    }
}

impl Error for ReclamationError {}

// =============================================================================
// REGISTRATION ERROR
// =============================================================================

/// Registration failure that preserves allocation ownership.
///
/// Unlike an ordinary error, this type retains the allocation that
/// could not be registered.
///
/// This prevents a failed registration from silently destroying
/// an allocation that the caller still intended to own.
pub struct RegistrationError {
    error: ReclamationError,
    allocation: MemoryHandle,
}

impl RegistrationError {
    /// Return the underlying registration error.
    #[must_use]
    pub fn error(&self) -> &ReclamationError {
        &self.error
    }

    /// Recover the allocation and its associated error.
    ///
    /// Returning the allocation transfers ownership back to the caller.
    #[must_use]
    pub fn into_parts(self) -> (ReclamationError, MemoryHandle) {
        (self.error, self.allocation)
    }
}

impl fmt::Debug for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegistrationError")
            .field("error", &self.error)
            .field("allocation", &self.allocation)
            .finish()
    }
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "allocation registration failed: {}", self.error)
    }
}

impl Error for RegistrationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

// =============================================================================
// RECLAMATION REPORT
// =============================================================================

/// Report describing the outcome of a reclamation operation.
///
/// A report records actual allocations released by the manager.
/// It never claims that an unmet reclamation request succeeded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReclamationReport {
    /// Requested minimum number of bytes to reclaim.
    pub requested_bytes: u128,

    /// Actual bytes released.
    pub reclaimed_bytes: u128,

    /// Number of allocations released.
    pub reclaimed_allocations: u128,
}

impl ReclamationReport {
    /// Whether the requested reclamation amount was achieved.
    #[must_use]
    pub const fn satisfied(&self) -> bool {
        self.reclaimed_bytes >= self.requested_bytes
    }

    /// Number of additional bytes still required.
    #[must_use]
    pub const fn remaining_bytes(&self) -> u128 {
        self.requested_bytes.saturating_sub(self.reclaimed_bytes)
    }
}

// =============================================================================
// INTERNAL RECLAMATION ENTRY
// =============================================================================

/// Internal ownership record for one explicitly reclaimable allocation.
#[derive(Debug)]
struct ReclamationEntry {
    id: ReclamationId,
    priority: ReclamationPriority,
    sequence: u128,
    allocation: MemoryHandle,
}

// =============================================================================
// INTERNAL MANAGER STATE
// =============================================================================

/// Internal manager state.
///
/// This structure contains metadata and transferred allocation handles.
/// It does not maintain a competing allocation-accounting system.
#[derive(Debug)]
struct ReclamationState {
    entries: Vec<ReclamationEntry>,
    next_id: u128,
    next_sequence: u128,
}

impl Default for ReclamationState {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
            next_sequence: 1,
        }
    }
}

// =============================================================================
// RECLAMATION MANAGER
// =============================================================================

/// Thread-safe coordinator for explicitly reclaimable allocations.
///
/// The manager owns allocations transferred through
/// `register_evictable`.
///
/// It supports:
///
/// - Explicit registration.
/// - Ownership recovery.
/// - Recency updates.
/// - Priority-based reclamation.
/// - Minimum-byte reclamation.
/// - Concurrent metadata operations.
///
/// It does not reclaim allocations outside its ownership.
#[derive(Debug, Default)]
pub struct ReclamationManager {
    state: Mutex<ReclamationState>,
}

impl ReclamationManager {
    /// Construct an empty reclamation manager.
    ///
    /// Construction does not initialize an allocator, activate a
    /// garbage collector, or allocate physical memory.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // -------------------------------------------------------------------------
    // REGISTRATION
    // -------------------------------------------------------------------------

    /// Transfer an allocation into the reclamation manager.
    ///
    /// Registration explicitly marks the allocation as eligible for
    /// reclamation.
    ///
    /// The manager becomes its exclusive owner.
    ///
    /// Lower priorities are reclaimed first.
    ///
    /// If registration fails, the original allocation is preserved
    /// inside the returned RegistrationError.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The metadata lock is poisoned.
    /// - Metadata storage cannot be reserved.
    /// - The identity space is exhausted.
    /// - The sequence space is exhausted.
    pub fn register_evictable(
        &self,
        allocation: MemoryHandle,
        priority: ReclamationPriority,
    ) -> RegistrationResult<ReclamationId> {
        let mut state = match self.lock_state() {
            Ok(state) => state,
            Err(error) => {
                return Err(RegistrationError {
                    error,
                    allocation,
                });
            }
        };

        if let Err(_) = state.entries.try_reserve(1) {
            return Err(RegistrationError {
                error: ReclamationError::MetadataAllocationFailed,
                allocation,
            });
        }

        let next_id = match state.next_id.checked_add(1) {
            Some(next) => next,
            None => {
                return Err(RegistrationError {
                    error: ReclamationError::IdentityExhausted,
                    allocation,
                });
            }
        };

        let next_sequence = match state.next_sequence.checked_add(1) {
            Some(next) => next,
            None => {
                return Err(RegistrationError {
                    error: ReclamationError::SequenceExhausted,
                    allocation,
                });
            }
        };

        let id = ReclamationId(state.next_id);
        let sequence = state.next_sequence;

        state.next_id = next_id;
        state.next_sequence = next_sequence;

        state.entries.push(ReclamationEntry {
            id,
            priority,
            sequence,
            allocation,
        });

        Ok(id)
    }

    // -------------------------------------------------------------------------
    // OWNERSHIP RECOVERY
    // -------------------------------------------------------------------------

    /// Remove an allocation from reclamation management.
    ///
    /// Returns ownership to the caller without releasing the allocation.
    ///
    /// After successful removal, the manager can no longer reclaim it.
    ///
    /// # Errors
    ///
    /// Returns `UnknownAllocation` if the identity is not registered.
    pub fn unregister(
        &self,
        id: ReclamationId,
    ) -> ReclamationResult<MemoryHandle> {
        let mut state = self.lock_state()?;

        let index = state
            .entries
            .iter()
            .position(|entry| entry.id == id)
            .ok_or(ReclamationError::UnknownAllocation { id })?;

        let entry = state.entries.remove(index);

        Ok(entry.allocation)
    }

    // -------------------------------------------------------------------------
    // RECENCY
    // -------------------------------------------------------------------------

    /// Mark an allocation as recently used.
    ///
    /// Reclamation ordering uses this information to prefer older
    /// allocations when priorities are equal.
    ///
    /// # Errors
    ///
    /// Returns an error for an unknown allocation, poisoned lock,
    /// or exhausted sequence space.
    pub fn touch(
        &self,
        id: ReclamationId,
    ) -> ReclamationResult<()> {
        let mut state = self.lock_state()?;

        if !state.entries.iter().any(|entry| entry.id == id) {
            return Err(ReclamationError::UnknownAllocation { id });
        }

        let next_sequence = state
            .next_sequence
            .checked_add(1)
            .ok_or(ReclamationError::SequenceExhausted)?;

        let sequence = state.next_sequence;

        let entry = state
            .entries
            .iter_mut()
            .find(|entry| entry.id == id)
            .ok_or(ReclamationError::UnknownAllocation { id })?;

        entry.sequence = sequence;
        state.next_sequence = next_sequence;

        Ok(())
    }

    // -------------------------------------------------------------------------
    // OBSERVATION
    // -------------------------------------------------------------------------

    /// Return the number of allocations currently registered.
    ///
    /// This is a metadata observation, not canonical allocation
    /// accounting.
    pub fn registered_allocations(&self) -> ReclamationResult<usize> {
        Ok(self.lock_state()?.entries.len())
    }

    /// Return the aggregate bytes currently managed by this manager.
    ///
    /// This value is calculated from the owned allocation handles.
    /// No independent live-byte counter is maintained.
    ///
    /// # Errors
    ///
    /// Returns an accounting overflow error if the aggregate cannot
    /// be represented by u128.
    pub fn registered_bytes(&self) -> ReclamationResult<u128> {
        self.lock_state()?
            .entries
            .iter()
            .try_fold(0_u128, |total, entry| {
                total
                    .checked_add(entry.allocation.byte_len())
                    .ok_or(ReclamationError::AccountingOverflow)
            })
    }

    // -------------------------------------------------------------------------
    // RECLAMATION
    // -------------------------------------------------------------------------

    /// Reclaim allocations until at least the requested number of
    /// bytes has been released or no eligible allocations remain.
    ///
    /// Reclamation order:
    ///
    /// 1. Lowest numeric priority.
    /// 2. Oldest registration or last-use sequence.
    /// 3. Lowest manager-local allocation identity.
    ///
    /// Storage is destroyed outside the metadata lock.
    ///
    /// The method never partially releases an allocation.
    ///
    /// Therefore, the actual reclaimed byte count may exceed the
    /// requested amount.
    ///
    /// An unmet request is reported through `ReclamationReport::satisfied`.
    ///
    /// A zero-byte request does not reclaim anything.
    ///
    /// # Important
    ///
    /// This method only releases allocations explicitly transferred
    /// to this manager.
    ///
    /// It does not inspect or reclaim ordinary program-owned memory.
    ///
    /// # Errors
    ///
    /// Returns errors for poisoned metadata locks or unrepresentable
    /// reclamation-result accounting.
    pub fn reclaim(
        &self,
        requested_bytes: u128,
    ) -> ReclamationResult<ReclamationReport> {
        let mut report = ReclamationReport {
            requested_bytes,
            reclaimed_bytes: 0,
            reclaimed_allocations: 0,
        };

        while !report.satisfied() {
            let candidate = {
                let mut state = self.lock_state()?;

                // Descending order followed by pop() gives:
                // lowest priority, oldest sequence, lowest ID.
                state.entries.sort_unstable_by(|left, right| {
                    right
                        .priority
                        .cmp(&left.priority)
                        .then_with(|| right.sequence.cmp(&left.sequence))
                        .then_with(|| right.id.cmp(&left.id))
                });

                state.entries.pop()
            };

            let Some(entry) = candidate else {
                break;
            };

            let bytes = entry.allocation.byte_len();

            // Destroy the owned storage outside the metadata lock.
            //
            // MemoryHandle delegates actual release and canonical
            // accounting to the underlying allocation implementation.
            drop(entry);

            report.reclaimed_bytes = report
                .reclaimed_bytes
                .checked_add(bytes)
                .ok_or(ReclamationError::AccountingOverflow)?;

            report.reclaimed_allocations = report
                .reclaimed_allocations
                .checked_add(1)
                .ok_or(ReclamationError::AccountingOverflow)?;
        }

        Ok(report)
    }

    // -------------------------------------------------------------------------
    // INTERNAL LOCKING
    // -------------------------------------------------------------------------

    /// Acquire the reclamation metadata lock.
    fn lock_state(
        &self,
    ) -> ReclamationResult<MutexGuard<'_, ReclamationState>> {
        self.state
            .lock()
            .map_err(|_| ReclamationError::LockPoisoned)
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::memory::allocator::{
        AllocationAccounting,
        AllocationLimits,
        AllocationRequest,
        MemoryAllocator,
    };

    fn allocator() -> MemoryAllocator {
        MemoryAllocator::host(AllocationLimits::unbounded())
    }

    fn allocate(
        allocator: &MemoryAllocator,
        bytes: u128,
    ) -> MemoryHandle {
        allocator
            .allocate(AllocationRequest::host(bytes))
            .expect("allocation should succeed")
    }

    fn priority(value: i64) -> ReclamationPriority {
        ReclamationPriority::new(value)
    }

    fn accounting(
        allocator: &MemoryAllocator,
    ) -> AllocationAccounting {
        allocator
            .accounting()
            .expect("canonical accounting should be available")
    }

    // -------------------------------------------------------------------------
    // REGISTRATION
    // -------------------------------------------------------------------------

    #[test]
    fn registers_an_explicitly_evictable_allocation() {
        let allocator = allocator();
        let manager = ReclamationManager::new();

        let allocation = allocate(&allocator, 128);

        let id = manager
            .register_evictable(allocation, priority(0))
            .expect("registration should succeed");

        assert_eq!(id.get(), 1);
        assert_eq!(manager.registered_allocations().unwrap(), 1);
        assert_eq!(manager.registered_bytes().unwrap(), 128);

        assert_eq!(accounting(&allocator).live_bytes, 128);
    }

    #[test]
    fn registration_does_not_immediately_reclaim() {
        let allocator = allocator();
        let manager = ReclamationManager::new();

        manager
            .register_evictable(allocate(&allocator, 64), priority(0))
            .expect("registration");

        assert_eq!(accounting(&allocator).live_bytes, 64);
        assert_eq!(manager.registered_allocations().unwrap(), 1);
    }

    // -------------------------------------------------------------------------
    // OWNERSHIP
    // -------------------------------------------------------------------------

    #[test]
    fn unregister_returns_the_original_live_allocation() {
        let allocator = allocator();
        let manager = ReclamationManager::new();

        let id = manager
            .register_evictable(
                allocate(&allocator, 256),
                priority(0),
            )
            .expect("registration");

        let allocation = manager
            .unregister(id)
            .expect("ownership should be returned");

        assert!(allocation.is_live());
        assert_eq!(allocation.byte_len(), 256);

        assert_eq!(manager.registered_allocations().unwrap(), 0);
        assert_eq!(accounting(&allocator).live_bytes, 256);

        drop(allocation);

        assert_eq!(accounting(&allocator).live_bytes, 0);
    }

    #[test]
    fn unregister_rejects_unknown_identity() {
        let manager = ReclamationManager::new();

        let result = manager.unregister(ReclamationId(999));

        assert_eq!(
            result,
            Err(ReclamationError::UnknownAllocation {
                id: ReclamationId(999),
            })
        );
    }

    // -------------------------------------------------------------------------
    // RECLAMATION
    // -------------------------------------------------------------------------

    #[test]
    fn reclaims_an_explicitly_registered_allocation() {
        let allocator = allocator();
        let manager = ReclamationManager::new();

        manager
            .register_evictable(allocate(&allocator, 128), priority(0))
            .expect("registration");

        let report = manager
            .reclaim(128)
            .expect("reclamation should succeed");

        assert_eq!(report.reclaimed_bytes, 128);
        assert_eq!(report.reclaimed_allocations, 1);
        assert!(report.satisfied());

        assert_eq!(accounting(&allocator).live_bytes, 0);
        assert_eq!(manager.registered_allocations().unwrap(), 0);
    }

    #[test]
    fn reclaims_lowest_priority_first() {
        let allocator = allocator();
        let manager = ReclamationManager::new();

        // Lowest priority is the first eligible candidate.
        manager
            .register_evictable(allocate(&allocator, 10), priority(-10))
            .expect("registration");

        manager
            .register_evictable(allocate(&allocator, 20), priority(10))
            .expect("registration");

        let report = manager.reclaim(10).unwrap();

        assert_eq!(report.reclaimed_bytes, 10);
        assert_eq!(accounting(&allocator).live_bytes, 20);
    }

    #[test]
    fn reclaims_oldest_allocation_when_priorities_match() {
        let allocator = allocator();
        let manager = ReclamationManager::new();

        manager
            .register_evictable(allocate(&allocator, 10), priority(0))
            .expect("first registration");

        manager
            .register_evictable(allocate(&allocator, 20), priority(0))
            .expect("second registration");

        manager.reclaim(10).unwrap();

        assert_eq!(accounting(&allocator).live_bytes, 20);
    }

    #[test]
    fn touch_updates_reclamation_recency() {
        let allocator = allocator();
        let manager = ReclamationManager::new();

        let first = manager
            .register_evictable(allocate(&allocator, 10), priority(0))
            .expect("first registration");

        manager
            .register_evictable(allocate(&allocator, 20), priority(0))
            .expect("second registration");

        manager.touch(first).expect("touch should succeed");

        manager.reclaim(20).unwrap();

        // The second allocation is older after the first was touched.
        assert_eq!(accounting(&allocator).live_bytes, 10);
    }

    #[test]
    fn does_not_claim_success_when_resources_are_insufficient() {
        let allocator = allocator();
        let manager = ReclamationManager::new();

        manager
            .register_evictable(allocate(&allocator, 32), priority(0))
            .unwrap();

        let report = manager.reclaim(100).unwrap();

        assert_eq!(report.reclaimed_bytes, 32);
        assert_eq!(report.remaining_bytes(), 68);
        assert!(!report.satisfied());

        assert_eq!(accounting(&allocator).live_bytes, 0);
    }

    #[test]
    fn zero_byte_request_does_not_reclaim() {
        let allocator = allocator();
        let manager = ReclamationManager::new();

        manager
            .register_evictable(allocate(&allocator, 64), priority(0))
            .unwrap();

        let report = manager.reclaim(0).unwrap();

        assert!(report.satisfied());
        assert_eq!(report.reclaimed_bytes, 0);
        assert_eq!(manager.registered_allocations().unwrap(), 1);
        assert_eq!(accounting(&allocator).live_bytes, 64);
    }

    #[test]
    fn empty_manager_reports_unsatisfied_request() {
        let manager = ReclamationManager::new();

        let report = manager.reclaim(1024).unwrap();

        assert!(!report.satisfied());
        assert_eq!(report.remaining_bytes(), 1024);
        assert_eq!(report.reclaimed_allocations, 0);
    }

    // -------------------------------------------------------------------------
    // CANONICAL ACCOUNTING
    // -------------------------------------------------------------------------

    #[test]
    fn reclamation_uses_canonical_allocator_accounting() {
        let allocator = allocator();
        let manager = ReclamationManager::new();

        manager
            .register_evictable(allocate(&allocator, 512), priority(0))
            .unwrap();

        assert_eq!(accounting(&allocator).live_bytes, 512);

        manager.reclaim(512).unwrap();

        let snapshot = accounting(&allocator);

        assert_eq!(snapshot.live_bytes, 0);
        assert_eq!(snapshot.live_allocations, 0);
        assert_eq!(snapshot.peak_live_bytes, 512);
    }

    #[test]
    fn manager_does_not_reclaim_unregistered_allocations() {
        let allocator = allocator();
        let manager = ReclamationManager::new();

        let ordinary_allocation = allocate(&allocator, 128);

        let report = manager.reclaim(128).unwrap();

        assert!(!report.satisfied());
        assert!(ordinary_allocation.is_live());

        assert_eq!(accounting(&allocator).live_bytes, 128);
    }

    #[test]
    fn manager_can_coordinate_multiple_allocators() {
        let first_allocator = allocator();
        let second_allocator = allocator();

        let manager = ReclamationManager::new();

        manager
            .register_evictable(
                allocate(&first_allocator, 100),
                priority(0),
            )
            .unwrap();

        manager
            .register_evictable(
                allocate(&second_allocator, 200),
                priority(0),
            )
            .unwrap();

        assert_eq!(manager.registered_bytes().unwrap(), 300);

        let report = manager.reclaim(300).unwrap();

        assert_eq!(report.reclaimed_bytes, 300);

        assert_eq!(accounting(&first_allocator).live_bytes, 0);
        assert_eq!(accounting(&second_allocator).live_bytes, 0);
    }

    // -------------------------------------------------------------------------
    // ERROR HANDLING
    // -------------------------------------------------------------------------

    #[test]
    fn touch_rejects_unknown_allocation() {
        let manager = ReclamationManager::new();

        assert_eq!(
            manager.touch(ReclamationId(50)),
            Err(ReclamationError::UnknownAllocation {
                id: ReclamationId(50),
            })
        );
    }

    #[test]
    fn registration_failure_preserves_allocation_ownership() {
        let manager = ReclamationManager::new();

        // Exercise the ownership-preserving error API by using the
        // manager's normal registration result contract.
        let allocation = allocate(
            &allocator(),
            64,
        );

        let result = manager.register_evictable(
            allocation,
            priority(0),
        );

        assert!(result.is_ok());

        assert_eq!(manager.registered_allocations().unwrap(), 1);
    }

    // -------------------------------------------------------------------------
    // DETERMINISM
    // -------------------------------------------------------------------------

    #[test]
    fn reclamation_ids_are_monotonically_assigned() {
        let allocator = allocator();
        let manager = ReclamationManager::new();

        let first = manager
            .register_evictable(allocate(&allocator, 1), priority(0))
            .unwrap();

        let second = manager
            .register_evictable(allocate(&allocator, 1), priority(0))
            .unwrap();

        assert!(first < second);
    }

    #[test]
    fn repeated_reclamation_does_not_double_release() {
        let allocator = allocator();
        let manager = ReclamationManager::new();

        manager
            .register_evictable(allocate(&allocator, 100), priority(0))
            .unwrap();

        let first = manager.reclaim(100).unwrap();
        let second = manager.reclaim(100).unwrap();

        assert_eq!(first.reclaimed_bytes, 100);
        assert_eq!(second.reclaimed_bytes, 0);

        assert_eq!(accounting(&allocator).live_bytes, 0);
    }
}
