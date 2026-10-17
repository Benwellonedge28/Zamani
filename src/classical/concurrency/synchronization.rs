
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical concurrency: synchronization contracts and safe host primitives.
//!
//! File: `src/classical/concurrency/synchronization.rs`
//! Edition: Rust 2021
//! Minimum Rust version: repository-declared Rust 1.97
//!
//! # Purpose
//!
//! This module provides:
//! - target-independent synchronization intent and memory-order metadata;
//! - safe host mutexes and read/write locks;
//! - condition variables and reusable barriers;
//! - one-time initialization;
//! - checked, explicit atomic operations;
//! - structured errors for invalid requests and poisoned locks.
//!
//! # Authority and ownership
//!
//! The synchronization grammar owns source syntax. The frontend AST owns parsed
//! source structure. Semantic analysis validates names, types, effects, memory
//! ordering, ownership, and target feasibility. The canonical IR represents
//! synchronization dependencies. Execution planning and target adapters choose
//! the realization.
//!
//! This module does not define an AST, a second IR, task identities, a scheduler,
//! a distributed protocol, or a hardware-specific synchronization mechanism.
//!
//! # POCO-REAF
//!
//! Source-level synchronization intent does not prescribe a thread, CPU, node,
//! GPU, FPGA, QPU, or physical address. The safe host primitives in this file
//! are one possible local realization, not the universal implementation.
//!
//! No language-level maximum is imposed on synchronization-object counts,
//! barrier participants, timeout durations, or collection sizes. Host
//! representations and real resource availability still impose finite limits.
//!
//! # Failure policy
//!
//! Lock poisoning is reported instead of silently ignored. Callers may make an
//! explicit recovery decision using the standard library's poisoning APIs at a
//! carefully reviewed integration boundary.
//!
//! Timeouts are caller-supplied; this module does not invent a global timeout.
//! Cancellation is owned by the task/execution context and is not duplicated
//! here.
//!
//! Distributed and device synchronization must be implemented by downstream
//! adapters. A local lock or barrier does not synchronize separate processes,
//! remote nodes, device queues, or QPU providers.

use std::error::Error;
use std::fmt;
use std::num::NonZeroUsize;
use std::sync::{
    Barrier, BarrierWaitResult, Condvar, LockResult, Mutex, MutexGuard, OnceLock,
    PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard, WaitTimeoutResult,
};
use std::sync::atomic::{
    AtomicBool, AtomicUsize, Ordering as AtomicOrdering,
};
use std::time::Duration;

// =============================================================================
// Portable semantic descriptions
// =============================================================================

/// Logical scope of a synchronization requirement.
///
/// Scope is semantic metadata. It does not identify a physical thread, process,
/// machine, accelerator, or network endpoint.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SynchronizationScope {
    /// The relevant execution task or logical task group.
    Task,
    /// Participants within one process.
    Process,
    /// Participants sharing a host memory domain.
    SharedMemory,
    /// Participants that may span machines or processes.
    Distributed,
    /// Participants involving an accelerator or device execution domain.
    Accelerator,
    /// Coordination between classical and non-classical execution domains.
    Hybrid,
    /// An extensible scope defined by a registered implementation.
    Custom(String),
}

/// Semantic kind of synchronization operation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SynchronizationKind {
    Mutex,
    ReadWriteLock,
    ConditionVariable,
    Barrier,
    Atomic,
    Fence,
    Once,
    /// A named extension point. It is not automatically executable.
    Custom(String),
}

/// Memory ordering expressed independently of a particular atomic object.
///
/// Semantic analysis must reject an ordering that is invalid for the relevant
/// operation. This enum deliberately mirrors Rust's portable ordering concepts
/// without making Rust's implementation the language's semantic authority.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MemoryOrder {
    Relaxed,
    Acquire,
    Release,
    AcquireRelease,
    SequentiallyConsistent,
}

impl MemoryOrder {
    const fn to_rust(self) -> AtomicOrdering {
        match self {
            Self::Relaxed => AtomicOrdering::Relaxed,
            Self::Acquire => AtomicOrdering::Acquire,
            Self::Release => AtomicOrdering::Release,
            Self::AcquireRelease => AtomicOrdering::AcqRel,
            Self::SequentiallyConsistent => AtomicOrdering::SeqCst,
        }
    }

    const fn is_valid_for_load(self) -> bool {
        matches!(
            self,
            Self::Relaxed | Self::Acquire | Self::SequentiallyConsistent
        )
    }

    const fn is_valid_for_store(self) -> bool {
        matches!(
            self,
            Self::Relaxed | Self::Release | Self::SequentiallyConsistent
        )
    }

    const fn is_valid_failure_order(self, failure: Self) -> bool {
        match self {
            Self::Relaxed | Self::Release => matches!(failure, Self::Relaxed),
            Self::Acquire | Self::AcquireRelease => {
                matches!(failure, Self::Relaxed | Self::Acquire)
            }
            Self::SequentiallyConsistent => matches!(
                failure,
                Self::Relaxed | Self::Acquire | Self::SequentiallyConsistent
            ),
        }
    }
}

/// Declarative synchronization intent for semantic analysis and planning.
///
/// This descriptor does not perform synchronization. A compiler may preserve it
/// in the canonical IR or lower it through the repository's existing IR path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SynchronizationIntent {
    scope: SynchronizationScope,
    kind: SynchronizationKind,
    memory_order: Option<MemoryOrder>,
    required: bool,
}

impl SynchronizationIntent {
    pub fn new(
        scope: SynchronizationScope,
        kind: SynchronizationKind,
        memory_order: Option<MemoryOrder>,
        required: bool,
    ) -> Result<Self, SynchronizationError> {
        if memory_order.is_some() && kind != SynchronizationKind::Atomic {
            return Err(SynchronizationError::MemoryOrderNotApplicable);
        }

        Ok(Self {
            scope,
            kind,
            memory_order,
            required,
        })
    }

    pub fn scope(&self) -> &SynchronizationScope {
        &self.scope
    }

    pub fn kind(&self) -> &SynchronizationKind {
        &self.kind
    }

    pub const fn memory_order(&self) -> Option<MemoryOrder> {
        self.memory_order
    }

    pub const fn is_required(&self) -> bool {
        self.required
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors returned by this module's synchronization APIs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SynchronizationError {
    /// A lock became poisoned after a panic while it was held.
    LockPoisoned,
    /// A memory ordering is invalid for the requested operation.
    InvalidMemoryOrder {
        operation: &'static str,
        order: MemoryOrder,
    },
    /// The failure ordering for compare-exchange is stronger than permitted.
    InvalidCompareExchangeFailureOrder {
        success: MemoryOrder,
        failure: MemoryOrder,
    },
    /// Memory ordering was supplied for a non-atomic intent.
    MemoryOrderNotApplicable,
    /// A custom synchronization extension has no local implementation here.
    UnsupportedLocalImplementation {
        kind: SynchronizationKind,
        scope: SynchronizationScope,
    },
}

impl fmt::Display for SynchronizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LockPoisoned => {
                f.write_str("synchronization lock is poisoned")
            }
            Self::InvalidMemoryOrder { operation, order } => {
                write!(f, "memory order {order:?} is invalid for {operation}")
            }
            Self::InvalidCompareExchangeFailureOrder { success, failure } => {
                write!(
                    f,
                    "compare-exchange failure order {failure:?} is invalid \
                     for success order {success:?}"
                )
            }
            Self::MemoryOrderNotApplicable => {
                f.write_str("memory ordering applies only to atomic intent")
            }
            Self::UnsupportedLocalImplementation { kind, scope } => {
                write!(
                    f,
                    "no local host implementation for {kind:?} at scope {scope:?}"
                )
            }
        }
    }
}

impl Error for SynchronizationError {}

fn map_poison<T>(_: PoisonError<T>) -> SynchronizationError {
    SynchronizationError::LockPoisoned
}

// =============================================================================
// Mutex
// =============================================================================

/// Safe mutex for local host execution.
///
/// This is a local realization. It does not establish distributed or
/// device-memory synchronization.
#[derive(Debug)]
pub struct SyncMutex<T> {
    inner: Mutex<T>,
}

impl<T> SyncMutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: Mutex::new(value),
        }
    }

    pub fn lock(&self) -> Result<MutexGuard<'_, T>, SynchronizationError> {
        self.inner.lock().map_err(map_poison)
    }

    pub fn try_lock(
        &self,
    ) -> Result<Option<MutexGuard<'_, T>>, SynchronizationError> {
        match self.inner.try_lock() {
            Ok(guard) => Ok(Some(guard)),
            Err(std::sync::TryLockError::WouldBlock) => Ok(None),
            Err(std::sync::TryLockError::Poisoned(_)) => {
                Err(SynchronizationError::LockPoisoned)
            }
        }
    }

    /// Returns whether the standard-library mutex is poisoned.
    ///
    /// This reports status only; it does not recover or clear poisoning.
    pub fn is_poisoned(&self) -> bool {
        self.inner.is_poisoned()
    }

    /// Consumes the mutex and returns its value, reporting poison explicitly.
    pub fn into_inner(self) -> Result<T, SynchronizationError> {
        self.inner.into_inner().map_err(map_poison)
    }
}

// =============================================================================
// Read/write lock
// =============================================================================

/// Safe local read/write lock.
#[derive(Debug)]
pub struct SyncRwLock<T> {
    inner: RwLock<T>,
}

impl<T> SyncRwLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            inner: RwLock::new(value),
        }
    }

    pub fn read(&self) -> Result<RwLockReadGuard<'_, T>, SynchronizationError> {
        self.inner.read().map_err(map_poison)
    }

    pub fn write(&self) -> Result<RwLockWriteGuard<'_, T>, SynchronizationError> {
        self.inner.write().map_err(map_poison)
    }

    pub fn try_read(
        &self,
    ) -> Result<Option<RwLockReadGuard<'_, T>>, SynchronizationError> {
        match self.inner.try_read() {
            Ok(guard) => Ok(Some(guard)),
            Err(std::sync::TryLockError::WouldBlock) => Ok(None),
            Err(std::sync::TryLockError::Poisoned(_)) => {
                Err(SynchronizationError::LockPoisoned)
            }
        }
    }

    pub fn try_write(
        &self,
    ) -> Result<Option<RwLockWriteGuard<'_, T>>, SynchronizationError> {
        match self.inner.try_write() {
            Ok(guard) => Ok(Some(guard)),
            Err(std::sync::TryLockError::WouldBlock) => Ok(None),
            Err(std::sync::TryLockError::Poisoned(_)) => {
                Err(SynchronizationError::LockPoisoned)
            }
        }
    }

    pub fn is_poisoned(&self) -> bool {
        self.inner.is_poisoned()
    }

    pub fn into_inner(self) -> Result<T, SynchronizationError> {
        self.inner.into_inner().map_err(map_poison)
    }
}

// =============================================================================
// Condition variable
// =============================================================================

/// Condition variable paired with a compatible `SyncMutex`.
///
/// Callers must always test their condition in a loop while holding the lock.
/// A notification is not proof that the condition is true.
#[derive(Debug, Default)]
pub struct SyncCondvar {
    inner: Condvar,
}

impl SyncCondvar {
    pub const fn new() -> Self {
        Self {
            inner: Condvar::new(),
        }
    }

    pub fn notify_one(&self) {
        self.inner.notify_one();
    }

    pub fn notify_all(&self) {
        self.inner.notify_all();
    }

    pub fn wait<'a, T>(
        &self,
        guard: MutexGuard<'a, T>,
    ) -> Result<MutexGuard<'a, T>, SynchronizationError> {
        self.inner.wait(guard).map_err(map_poison)
    }

    /// Waits for at most `timeout`, returning the reacquired guard and whether
    /// the wait timed out. Spurious wakeups remain possible.
    pub fn wait_timeout<'a, T>(
        &self,
        guard: MutexGuard<'a, T>,
        timeout: Duration,
    ) -> Result<(MutexGuard<'a, T>, bool), SynchronizationError> {
        let (guard, result): (MutexGuard<'a, T>, WaitTimeoutResult) =
            self.inner.wait_timeout(guard, timeout).map_err(map_poison)?;
        Ok((guard, result.timed_out()))
    }
}

// =============================================================================
// Barrier
// =============================================================================

/// Reusable local barrier with an explicit, nonzero participant count.
///
/// The participant count is selected by the caller; it is not a universal
/// hardware ceiling. All participants must agree on the same barrier instance
/// and phase protocol. A local barrier cannot coordinate separate processes.
#[derive(Debug)]
pub struct SyncBarrier {
    participants: NonZeroUsize,
    inner: Barrier,
}

impl SyncBarrier {
    pub fn new(
        participants: NonZeroUsize,
    ) -> Result<Self, SynchronizationError> {
        Ok(Self {
            participants,
            inner: Barrier::new(participants.get()),
        })
    }

    pub const fn participants(&self) -> NonZeroUsize {
        self.participants
    }

    pub fn wait(&self) -> BarrierWaitResult {
        self.inner.wait()
    }
}

// =============================================================================
// One-time initialization
// =============================================================================

/// Safe, one-time local initialization.
///
/// The initializer may be called by one caller; other callers wait for its
/// completion. If initialization panics, the cell remains uninitialized and
/// a later call may attempt initialization again.
#[derive(Debug)]
pub struct SyncOnceCell<T> {
    inner: OnceLock<T>,
}

impl<T> Default for SyncOnceCell<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SyncOnceCell<T> {
    pub const fn new() -> Self {
        Self {
            inner: OnceLock::new(),
        }
    }

    pub fn get(&self) -> Option<&T> {
        self.inner.get()
    }

    pub fn get_or_init<F>(&self, initializer: F) -> &T
    where
        F: FnOnce() -> T,
    {
        self.inner.get_or_init(initializer)
    }

    pub fn set(&self, value: T) -> Result<(), T> {
        self.inner.set(value)
    }

    pub fn is_initialized(&self) -> bool {
        self.inner.get().is_some()
    }
}

// =============================================================================
// Atomic boolean
// =============================================================================

/// Atomic boolean for local host execution.
///
/// Ordering validation occurs before delegating to Rust atomics, so invalid
/// load/store orderings return structured errors rather than panicking.
#[derive(Debug, Default)]
pub struct SyncAtomicBool {
    inner: AtomicBool,
}

impl SyncAtomicBool {
    pub const fn new(value: bool) -> Self {
        Self {
            inner: AtomicBool::new(value),
        }
    }

    pub fn load(&self, order: MemoryOrder) -> Result<bool, SynchronizationError> {
        if !order.is_valid_for_load() {
            return Err(SynchronizationError::InvalidMemoryOrder {
                operation: "atomic load",
                order,
            });
        }
        Ok(self.inner.load(order.to_rust()))
    }

    pub fn store(
        &self,
        value: bool,
        order: MemoryOrder,
    ) -> Result<(), SynchronizationError> {
        if !order.is_valid_for_store() {
            return Err(SynchronizationError::InvalidMemoryOrder {
                operation: "atomic store",
                order,
            });
        }
        self.inner.store(value, order.to_rust());
        Ok(())
    }

    pub fn swap(
        &self,
        value: bool,
        order: MemoryOrder,
    ) -> bool {
        self.inner.swap(value, order.to_rust())
    }

    pub fn compare_exchange(
        &self,
        current: bool,
        new: bool,
        success: MemoryOrder,
        failure: MemoryOrder,
    ) -> Result<Result<bool, bool>, SynchronizationError> {
        validate_compare_exchange_orders(success, failure)?;
        Ok(self.inner.compare_exchange(
            current,
            new,
            success.to_rust(),
            failure.to_rust(),
        ))
    }
}

// =============================================================================
// Atomic usize
// =============================================================================

/// Atomic host-sized unsigned integer.
///
/// `usize` is a host representation, not Zamani's universal integer type.
/// Use the language's canonical integer/value layer for language-level integer
/// semantics. This wrapper is for synchronization counters and local atomics.
#[derive(Debug, Default)]
pub struct SyncAtomicUsize {
    inner: AtomicUsize,
}

impl SyncAtomicUsize {
    pub const fn new(value: usize) -> Self {
        Self {
            inner: AtomicUsize::new(value),
        }
    }

    pub fn load(&self, order: MemoryOrder) -> Result<usize, SynchronizationError> {
        if !order.is_valid_for_load() {
            return Err(SynchronizationError::InvalidMemoryOrder {
                operation: "atomic load",
                order,
            });
        }
        Ok(self.inner.load(order.to_rust()))
    }

    pub fn store(
        &self,
        value: usize,
        order: MemoryOrder,
    ) -> Result<(), SynchronizationError> {
        if !order.is_valid_for_store() {
            return Err(SynchronizationError::InvalidMemoryOrder {
                operation: "atomic store",
                order,
            });
        }
        self.inner.store(value, order.to_rust());
        Ok(())
    }

    pub fn swap(
        &self,
        value: usize,
        order: MemoryOrder,
    ) -> usize {
        self.inner.swap(value, order.to_rust())
    }

    /// Atomically adds with the underlying atomic type's wrapping arithmetic.
    ///
    /// Use `checked_fetch_add` when wrapping is not part of the intended
    /// counter semantics.
    pub fn fetch_add_wrapping(
        &self,
        value: usize,
        order: MemoryOrder,
    ) -> usize {
        self.inner.fetch_add(value, order.to_rust())
    }

    pub fn fetch_sub_wrapping(
        &self,
        value: usize,
        order: MemoryOrder,
    ) -> usize {
        self.inner.fetch_sub(value, order.to_rust())
    }

    /// Adds only if the result is representable; returns the previous value.
    ///
    /// The update is atomic. On overflow, the value is left unchanged and
    /// `None` is returned.
    pub fn checked_fetch_add(
        &self,
        value: usize,
    ) -> Option<usize> {
        self.inner.fetch_update(
            AtomicOrdering::SeqCst,
            AtomicOrdering::SeqCst,
            |current| current.checked_add(value),
        ).ok()
    }

    pub fn compare_exchange(
        &self,
        current: usize,
        new: usize,
        success: MemoryOrder,
        failure: MemoryOrder,
    ) -> Result<Result<usize, usize>, SynchronizationError> {
        validate_compare_exchange_orders(success, failure)?;
        Ok(self.inner.compare_exchange(
            current,
            new,
            success.to_rust(),
            failure.to_rust(),
        ))
    }
}

fn validate_compare_exchange_orders(
    success: MemoryOrder,
    failure: MemoryOrder,
) -> Result<(), SynchronizationError> {
    if !success.is_valid_failure_order(failure) {
        return Err(
            SynchronizationError::InvalidCompareExchangeFailureOrder {
                success,
                failure,
            },
        );
    }
    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn intent_preserves_scope_and_requirement() {
        let intent = SynchronizationIntent::new(
            SynchronizationScope::Distributed,
            SynchronizationKind::Barrier,
            None,
            true,
        )
        .expect("valid intent");

        assert_eq!(intent.scope(), &SynchronizationScope::Distributed);
        assert_eq!(intent.kind(), &SynchronizationKind::Barrier);
        assert!(intent.is_required());
    }

    #[test]
    fn rejects_memory_order_on_non_atomic_intent() {
        let result = SynchronizationIntent::new(
            SynchronizationScope::SharedMemory,
            SynchronizationKind::Mutex,
            Some(MemoryOrder::Acquire),
            true,
        );

        assert_eq!(
            result,
            Err(SynchronizationError::MemoryOrderNotApplicable)
        );
    }

    #[test]
    fn mutex_protects_shared_state() {
        let value = SyncMutex::new(0usize);
        {
            let mut guard = value.lock().expect("lock should not be poisoned");
            *guard = 41;
        }
        assert_eq!(*value.lock().expect("lock should remain usable"), 41);
    }

    #[test]
    fn mutex_try_lock_reports_contention_without_panicking() {
        let value = SyncMutex::new(1usize);
        let guard = value.lock().expect("lock");
        assert!(value.try_lock().expect("try lock").is_none());
        drop(guard);
        assert!(value.try_lock().expect("try lock").is_some());
    }

    #[test]
    fn read_write_lock_supports_read_and_write() {
        let value = SyncRwLock::new(3usize);
        assert_eq!(*value.read().expect("read lock"), 3);
        *value.write().expect("write lock") = 9;
        assert_eq!(*value.read().expect("read lock"), 9);
    }

    #[test]
    fn condition_variable_waits_for_predicate() {
        let pair = Arc::new((SyncMutex::new(false), SyncCondvar::new()));
        let producer_pair = Arc::clone(&pair);

        let producer = thread::spawn(move || {
            let (lock, condition) = &*producer_pair;
            let mut ready = lock.lock().expect("producer lock");
            *ready = true;
            condition.notify_one();
        });

        let (lock, condition) = &*pair;
        let mut ready = lock.lock().expect("consumer lock");
        while !*ready {
            ready = condition.wait(ready).expect("condition wait");
        }

        drop(ready);
        producer.join().expect("producer thread");
    }

    #[test]
    fn condition_variable_timeout_is_explicit() {
        let lock = SyncMutex::new(false);
        let condition = SyncCondvar::new();
        let guard = lock.lock().expect("lock");

        let (_guard, timed_out) = condition
            .wait_timeout(guard, Duration::ZERO)
            .expect("timed wait");

        assert!(timed_out);
    }

    #[test]
    fn barrier_releases_all_participants() {
        let participants = NonZeroUsize::new(2).expect("nonzero");
        let barrier = Arc::new(SyncBarrier::new(participants).expect("barrier"));
        let worker_barrier = Arc::clone(&barrier);

        let worker = thread::spawn(move || {
            worker_barrier.wait();
        });

        barrier.wait();
        worker.join().expect("worker thread");
        assert_eq!(barrier.participants().get(), 2);
    }

    #[test]
    fn once_cell_initializes_once() {
        let cell = SyncOnceCell::new();
        assert_eq!(*cell.get_or_init(|| 17), 17);
        assert_eq!(*cell.get_or_init(|| 99), 17);
        assert!(cell.is_initialized());
    }

    #[test]
    fn atomic_boolean_rejects_invalid_load_order() {
        let atomic = SyncAtomicBool::new(false);
        assert_eq!(
            atomic.load(MemoryOrder::Release),
            Err(SynchronizationError::InvalidMemoryOrder {
                operation: "atomic load",
                order: MemoryOrder::Release,
            })
        );
    }

    #[test]
    fn atomic_boolean_store_and_load() {
        let atomic = SyncAtomicBool::new(false);
        atomic
            .store(true, MemoryOrder::Release)
            .expect("valid store");
        assert!(atomic.load(MemoryOrder::Acquire).expect("valid load"));
    }

    #[test]
    fn atomic_usize_checked_add_detects_overflow() {
        let atomic = SyncAtomicUsize::new(usize::MAX);
        assert_eq!(atomic.checked_fetch_add(1), None);
        assert_eq!(
            atomic.load(MemoryOrder::SequentiallyConsistent),
            Ok(usize::MAX)
        );
    }

    #[test]
    fn compare_exchange_rejects_stronger_failure_order() {
        let atomic = SyncAtomicUsize::new(1);
        assert_eq!(
            atomic.compare_exchange(
                1,
                2,
                MemoryOrder::Release,
                MemoryOrder::Acquire,
            ),
            Err(SynchronizationError::InvalidCompareExchangeFailureOrder {
                success: MemoryOrder::Release,
                failure: MemoryOrder::Acquire,
            })
        );
    }

    #[test]
    fn atomic_compare_exchange_updates_expected_value() {
        let atomic = SyncAtomicUsize::new(4);
        let result = atomic
            .compare_exchange(
                4,
                8,
                MemoryOrder::AcquireRelease,
                MemoryOrder::Acquire,
            )
            .expect("valid ordering");

        assert_eq!(result, Ok(4));
        assert_eq!(
            atomic.load(MemoryOrder::Acquire),
            Ok(8)
        );
    }

    #[test]
    fn custom_scopes_are_not_restricted_to_known_hardware() {
        let scope = SynchronizationScope::Custom("future-domain".to_owned());
        let intent = SynchronizationIntent::new(
            scope.clone(),
            SynchronizationKind::Custom("provider_collective".to_owned()),
            None,
            false,
        )
        .expect("extensible intent");

        assert_eq!(intent.scope(), &scope);
    }
}
