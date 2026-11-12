
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Synchronization
//!
//! Path: `src/zur/concurrency/synchronization.rs`
//!
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! Provides portable, safe synchronization primitives for ZUR.
//!
//! This module supports:
//!
//! - Mutual exclusion.
//! - Reader/writer synchronization.
//! - Condition-variable coordination.
//! - Reusable barriers.
//! - Atomic operations.
//! - Atomic memory ordering.
//! - One-time initialization.
//!
//! ## Ownership
//!
//! This module owns the ZUR synchronization API boundary.
//!
//! It does not own:
//!
//! - Task lifecycle.
//! - Task scheduling.
//! - Executor implementation.
//! - Thread-pool implementation.
//! - Hardware discovery.
//! - Resource allocation.
//! - Distributed placement.
//! - Network synchronization protocols.
//! - Quantum synchronization semantics.
//! - HDL synchronization semantics.
//! - Source-language semantic validation.
//! - Compiler synchronization IR.
//!
//! Those responsibilities remain with their authoritative
//! repository subsystems.
//!
//! ## Existing authorities
//!
//! - `zur/error.rs`:
//!   ZurError and ZurResult.
//!
//! - `zur/concurrency/task.rs`:
//!   Task lifecycle and completion.
//!
//! - `zur/concurrency/executor.rs`:
//!   Task execution.
//!
//! - `zur/concurrency/channel.rs`:
//!   Message communication.
//!
//! - `zur/execution/cancellation.rs`:
//!   Execution cancellation.
//!
//! - `zur/execution/session.rs`:
//!   Execution sessions.
//!
//! - `classical/concurrency/`:
//!   Classical concurrency facilities.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No unsafe implementations.
//! No raw-pointer synchronization APIs.
//! No global mutable synchronization state.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary application-defined
//! maximum for:
//!
//! - Mutex instances.
//! - Reader/writer locks.
//! - Barriers.
//! - Atomic objects.
//! - Synchronization operations.
//! - Synchronization participants.
//! - Synchronization scopes.
//! - Waiting operations.
//!
//! Actual limits are determined by available memory,
//! the Rust standard library, platform facilities and
//! configured operational budgets.
//!
//! No finite machine promises literal infinite computation.
//!
//! ## Portability
//!
//! The public API is independent of:
//!
//! - CPU vendor.
//! - Processor count.
//! - Operating system.
//! - Hardware topology.
//! - GPU vendor.
//! - Accelerator architecture.
//! - Distributed topology.
//!
//! The underlying Rust standard library provides the
//! platform-specific synchronization implementation.
//!
//! ## Memory ordering
//!
//! Atomic memory ordering is represented by Rust's
//! standard atomic Ordering type.
//!
//! The frontend and semantic subsystem remain responsible
//! for validating Zamani source-level memory-order intent.
//!
//! A runtime synchronization primitive must not reinterpret
//! invalid semantic memory-order combinations as valid.
//!
//! ## Poisoning
//!
//! Mutex and RwLock poisoning is reported as a structured
//! ZUR error.
//!
//! This module does not silently recover potentially
//! inconsistent application state.
//!
//! Recovery requires an explicit application decision.
//!
//! ## Blocking operations
//!
//! Mutex, RwLock, Condvar and Barrier operations may block
//! the calling operating-system thread.
//!
//! They must not be used indiscriminately inside asynchronous
//! executor workers.
//!
//! Asynchronous coordination belongs to the appropriate
//! asynchronous executor and synchronization provider.

use std::num::NonZeroUsize;
use std::sync::{
    Barrier as StdBarrier,
    Condvar as StdCondvar,
    Mutex as StdMutex,
    MutexGuard as StdMutexGuard,
    OnceLock as StdOnceLock,
    RwLock as StdRwLock,
    RwLockReadGuard as StdRwLockReadGuard,
    RwLockWriteGuard as StdRwLockWriteGuard,
};
use std::time::Duration;

use crate::zur::error::{ZurError, ZurResult};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.concurrency.synchronization";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

/// Creates a structured synchronization error.
///
/// Poisoning is not automatically repaired because the protected
/// application state may no longer satisfy its invariants.
fn synchronization_error(
    operation: &'static str,
    message: &'static str,
) -> ZurError {
    ZurError::internal(message)
        .with_component(COMPONENT)
        .with_operation(operation)
}

/// Converts a poisoned synchronization primitive into a ZUR error.
fn poisoned_error(operation: &'static str) -> ZurError {
    synchronization_error(
        operation,
        "synchronization primitive was poisoned",
    )
}

// ============================================================================
// MUTEX
// ============================================================================

/// Exclusive access guard for a synchronization mutex.
///
/// The guard releases its lock when dropped.
///
/// It must not be retained longer than necessary.
pub type MutexGuard<'a, T> = StdMutexGuard<'a, T>;

/// Thread-safe mutual exclusion primitive.
///
/// A mutex permits only one successful lock holder at a time.
///
/// The mutex does not determine:
///
/// - Which task should acquire it.
/// - Which thread should acquire it.
/// - Execution priority.
/// - Hardware placement.
/// - Distributed ownership.
///
/// Those decisions belong to their respective subsystems.
#[derive(Debug)]
pub struct Mutex<T> {
    inner: StdMutex<T>,
}

impl<T> Mutex<T> {
    /// Creates a new unlocked mutex containing `value`.
    ///
    /// Construction does not create a thread or executor.
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self {
            inner: StdMutex::new(value),
        }
    }

    /// Acquires exclusive access.
    ///
    /// The calling thread blocks until the lock becomes available.
    ///
    /// Returns an error if the mutex is poisoned.
    pub fn lock(&self) -> ZurResult<MutexGuard<'_, T>> {
        self.inner
            .lock()
            .map_err(|_| poisoned_error("mutex_lock"))
    }

    /// Attempts to acquire exclusive access without blocking.
    ///
    /// Returns:
    ///
    /// - `Ok(Some(guard))` when the lock was acquired.
    /// - `Ok(None)` when another thread currently holds the lock.
    /// - `Err(error)` when the mutex is poisoned.
    pub fn try_lock(&self) -> ZurResult<Option<MutexGuard<'_, T>>> {
        match self.inner.try_lock() {
            Ok(guard) => Ok(Some(guard)),

            Err(std::sync::TryLockError::WouldBlock) => Ok(None),

            Err(std::sync::TryLockError::Poisoned(_)) => {
                Err(poisoned_error("mutex_try_lock"))
            }
        }
    }

    /// Returns whether the mutex has been poisoned.
    ///
    /// Poisoning indicates that a thread panicked while holding
    /// the mutex. It does not independently establish whether
    /// application data is inconsistent.
    #[must_use]
    pub fn is_poisoned(&self) -> bool {
        self.inner.is_poisoned()
    }

    /// Clears the mutex poisoning indicator.
    ///
    /// The caller is responsible for establishing that the
    /// protected data has been inspected and repaired.
    ///
    /// This operation does not repair application data.
    pub fn clear_poison(&self) {
        self.inner.clear_poison();
    }

    /// Provides mutable access when the mutex itself is
    /// exclusively borrowed.
    ///
    /// No other mutex guard can coexist with this exclusive borrow.
    pub fn get_mut(&mut self) -> ZurResult<&mut T> {
        self.inner
            .get_mut()
            .map_err(|_| poisoned_error("mutex_get_mut"))
    }

    /// Consumes the mutex and returns its contained value.
    ///
    /// Returns an error if the mutex is poisoned.
    pub fn into_inner(self) -> ZurResult<T> {
        self.inner
            .into_inner()
            .map_err(|_| poisoned_error("mutex_into_inner"))
    }
}

impl<T: Default> Default for Mutex<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

// ============================================================================
// READER / WRITER LOCK
// ============================================================================

/// Shared reader-access guard.
pub type RwLockReadGuard<'a, T> = StdRwLockReadGuard<'a, T>;

/// Exclusive writer-access guard.
pub type RwLockWriteGuard<'a, T> = StdRwLockWriteGuard<'a, T>;

/// Thread-safe reader/writer synchronization primitive.
///
/// Multiple readers may hold read access concurrently.
///
/// A writer requires exclusive access.
///
/// Fairness and starvation guarantees are platform and
/// standard-library implementation dependent.
///
/// No application-specific fairness guarantee is introduced.
#[derive(Debug)]
pub struct RwLock<T> {
    inner: StdRwLock<T>,
}

impl<T> RwLock<T> {
    /// Creates a new reader/writer lock containing `value`.
    #[must_use]
    pub const fn new(value: T) -> Self {
        Self {
            inner: StdRwLock::new(value),
        }
    }

    /// Acquires shared read access.
    ///
    /// Multiple readers may coexist.
    pub fn read(&self) -> ZurResult<RwLockReadGuard<'_, T>> {
        self.inner
            .read()
            .map_err(|_| poisoned_error("rwlock_read"))
    }

    /// Acquires exclusive write access.
    ///
    /// Existing readers and writers must release their guards
    /// before this operation can succeed.
    pub fn write(&self) -> ZurResult<RwLockWriteGuard<'_, T>> {
        self.inner
            .write()
            .map_err(|_| poisoned_error("rwlock_write"))
    }

    /// Attempts to acquire read access without blocking.
    ///
    /// Returns `None` when the lock cannot currently be acquired.
    pub fn try_read(
        &self,
    ) -> ZurResult<Option<RwLockReadGuard<'_, T>>> {
        match self.inner.try_read() {
            Ok(guard) => Ok(Some(guard)),

            Err(std::sync::TryLockError::WouldBlock) => Ok(None),

            Err(std::sync::TryLockError::Poisoned(_)) => {
                Err(poisoned_error("rwlock_try_read"))
            }
        }
    }

    /// Attempts to acquire exclusive write access without blocking.
    ///
    /// Returns `None` when the lock cannot currently be acquired.
    pub fn try_write(
        &self,
    ) -> ZurResult<Option<RwLockWriteGuard<'_, T>>> {
        match self.inner.try_write() {
            Ok(guard) => Ok(Some(guard)),

            Err(std::sync::TryLockError::WouldBlock) => Ok(None),

            Err(std::sync::TryLockError::Poisoned(_)) => {
                Err(poisoned_error("rwlock_try_write"))
            }
        }
    }

    /// Returns whether the lock has been poisoned.
    #[must_use]
    pub fn is_poisoned(&self) -> bool {
        self.inner.is_poisoned()
    }

    /// Clears the poisoning indicator.
    ///
    /// The caller must first establish that the protected
    /// application data is consistent.
    pub fn clear_poison(&self) {
        self.inner.clear_poison();
    }

    /// Returns mutable access when exclusively borrowed.
    pub fn get_mut(&mut self) -> ZurResult<&mut T> {
        self.inner
            .get_mut()
            .map_err(|_| poisoned_error("rwlock_get_mut"))
    }

    /// Consumes the lock and returns its contained value.
    pub fn into_inner(self) -> ZurResult<T> {
        self.inner
            .into_inner()
            .map_err(|_| poisoned_error("rwlock_into_inner"))
    }
}

impl<T: Default> Default for RwLock<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

// ============================================================================
// CONDITION VARIABLE
// ============================================================================

/// Condition-variable wait result.
///
/// The boolean indicates whether the timeout elapsed.
pub type ConditionWaitTimeoutResult = (bool,);

/// Condition-variable synchronization primitive.
///
/// Condition variables allow threads to wait for a condition
/// protected by a mutex.
///
/// The condition itself belongs to the application.
///
/// A condition variable does not independently represent or
/// guarantee that a condition has become true.
///
/// Notifications are not stored events. Waiters must always
/// check their predicates in a loop.
///
/// A condition variable should be paired consistently with
/// the same mutex during concurrent waits.
#[derive(Debug, Default)]
pub struct Condvar {
    inner: StdCondvar,
}

impl Condvar {
    /// Creates a new condition variable.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: StdCondvar::new(),
        }
    }

    /// Waits until another thread notifies this condition variable.
    ///
    /// The mutex is atomically released while waiting and
    /// reacquired before this method returns.
    ///
    /// The caller must recheck its condition after waking.
    ///
    /// Returns an error if the mutex becomes poisoned.
    pub fn wait<'a, T>(
        &self,
        guard: MutexGuard<'a, T>,
    ) -> ZurResult<MutexGuard<'a, T>> {
        self.inner
            .wait(guard)
            .map_err(|_| poisoned_error("condvar_wait"))
    }

    /// Waits while the supplied predicate remains true.
    ///
    /// This method handles spurious wakeups by checking the
    /// predicate again after every wakeup.
    pub fn wait_while<'a, T>(
        &self,
        guard: MutexGuard<'a, T>,
        condition: impl FnMut(&mut T) -> bool,
    ) -> ZurResult<MutexGuard<'a, T>> {
        self.inner
            .wait_while(guard, condition)
            .map_err(|_| poisoned_error("condvar_wait_while"))
    }

    /// Waits for notification up to a specified duration.
    ///
    /// Returns the reacquired mutex guard and whether the
    /// timeout elapsed.
    ///
    /// A timeout does not imply that the application condition
    /// is false or that the operation has been cancelled.
    pub fn wait_timeout<'a, T>(
        &self,
        guard: MutexGuard<'a, T>,
        timeout: Duration,
    ) -> ZurResult<(MutexGuard<'a, T>, bool)> {
        let (guard, result) = self
            .inner
            .wait_timeout(guard, timeout)
            .map_err(|_| poisoned_error("condvar_wait_timeout"))?;

        Ok((guard, result.timed_out()))
    }

    /// Waits while the supplied predicate remains true,
    /// subject to a timeout.
    ///
    /// The returned boolean indicates whether the timeout
    /// elapsed.
    ///
    /// The predicate must be checked by the caller when the
    /// method returns.
    pub fn wait_timeout_while<'a, T>(
        &self,
        guard: MutexGuard<'a, T>,
        timeout: Duration,
        condition: impl FnMut(&mut T) -> bool,
    ) -> ZurResult<(MutexGuard<'a, T>, bool)> {
        let (guard, result) = self
            .inner
            .wait_timeout_while(guard, timeout, condition)
            .map_err(|_| poisoned_error("condvar_wait_timeout_while"))?;

        Ok((guard, result.timed_out()))
    }

    /// Wakes one waiting thread, if any.
    ///
    /// This does not guarantee which waiter will be selected.
    pub fn notify_one(&self) {
        self.inner.notify_one();
    }

    /// Wakes all currently waiting threads.
    pub fn notify_all(&self) {
        self.inner.notify_all();
    }
}

// ============================================================================
// BARRIER
// ============================================================================

/// Result of a barrier wait.
pub type BarrierWaitResult = std::sync::BarrierWaitResult;

/// Reusable synchronization barrier.
///
/// A barrier coordinates a fixed participant count for each
/// generation.
///
/// Every participant must arrive before any participant
/// proceeds to the next generation.
///
/// The participant count is supplied by the caller and is
/// not a universal machine or language limit.
///
/// Cancellation and participant failure require a separate
/// coordination protocol. The standard barrier does not
/// automatically remove missing participants.
#[derive(Debug)]
pub struct Barrier {
    participants: NonZeroUsize,
    inner: StdBarrier,
}

impl Barrier {
    /// Creates a barrier with the supplied nonzero participant count.
    ///
    /// The count is a property of this particular barrier instance.
    ///
    /// No global participant ceiling is introduced.
    #[must_use]
    pub fn new(participants: NonZeroUsize) -> Self {
        Self {
            participants,
            inner: StdBarrier::new(participants.get()),
        }
    }

    /// Returns the configured participant count.
    #[must_use]
    pub const fn participants(&self) -> NonZeroUsize {
        self.participants
    }

    /// Waits until all participants in the current generation arrive.
    ///
    /// The returned result identifies one arbitrary participant
    /// as the leader of this generation.
    ///
    /// The leader designation does not establish scheduling priority.
    pub fn wait(&self) -> BarrierWaitResult {
        self.inner.wait()
    }
}

// ============================================================================
// ONE-TIME INITIALIZATION
// ============================================================================

/// Thread-safe one-time initialization cell.
///
/// This wrapper reuses Rust's standard OnceLock.
///
/// Initialization is performed lazily when requested.
///
/// No global initialization occurs merely because this
/// module is imported.
#[derive(Debug)]
pub struct OnceLock<T> {
    inner: StdOnceLock<T>,
}

impl<T> OnceLock<T> {
    /// Creates an uninitialized one-time cell.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: StdOnceLock::new(),
        }
    }

    /// Returns the initialized value, if available.
    #[must_use]
    pub fn get(&self) -> Option<&T> {
        self.inner.get()
    }

    /// Attempts to initialize the cell with the supplied value.
    ///
    /// Returns the value unchanged if initialization has
    /// already occurred.
    pub fn set(&self, value: T) -> Result<(), T> {
        self.inner.set(value)
    }

    /// Initializes the cell on first access.
    ///
    /// Concurrent callers share the successfully initialized value.
    ///
    /// If the initialization closure panics, the standard library
    /// permits a later caller to attempt initialization again.
    pub fn get_or_init(
        &self,
        initialize: impl FnOnce() -> T,
    ) -> &T {
        self.inner.get_or_init(initialize)
    }

    /// Removes and returns the initialized value when exclusively
    /// borrowed.
    ///
    /// The cell becomes uninitialized again.
    pub fn take(&mut self) -> Option<T> {
        self.inner.take()
    }

    /// Consumes the cell and returns its initialized value.
    #[must_use]
    pub fn into_inner(self) -> Option<T> {
        self.inner.into_inner()
    }
}

impl<T> Default for OnceLock<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ATOMIC MEMORY ORDERING
// ============================================================================

/// Standard Rust atomic memory-ordering semantics.
///
/// Zamani source-level memory-order expressions must be
/// validated by the frontend semantic authority before
/// reaching this implementation boundary.
///
/// In particular, the compiler must not blindly translate
/// every source-level ordering name into a Rust Ordering value.
///
/// Rust does not expose a distinct Consume ordering.
pub use std::sync::atomic::Ordering as MemoryOrdering;

// ============================================================================
// TARGET-SUPPORTED ATOMIC PRIMITIVES
// ============================================================================
//
// Atomic types are exposed only when the compilation target
// advertises the corresponding atomic capability.
//
// This avoids assuming that every target supports every
// atomic width.
//
// These cfg conditions describe Rust target support, not
// universal Zamani hardware ceilings.

#[cfg(target_has_atomic = "8")]
/// Atomic boolean value on targets supporting 8-bit atomics.
pub use std::sync::atomic::AtomicBool;

#[cfg(target_has_atomic = "8")]
/// Atomic signed 8-bit integer.
pub use std::sync::atomic::AtomicI8;

#[cfg(target_has_atomic = "8")]
/// Atomic unsigned 8-bit integer.
pub use std::sync::atomic::AtomicU8;

#[cfg(target_has_atomic = "16")]
/// Atomic signed 16-bit integer.
pub use std::sync::atomic::AtomicI16;

#[cfg(target_has_atomic = "16")]
/// Atomic unsigned 16-bit integer.
pub use std::sync::atomic::AtomicU16;

#[cfg(target_has_atomic = "32")]
/// Atomic signed 32-bit integer.
pub use std::sync::atomic::AtomicI32;

#[cfg(target_has_atomic = "32")]
/// Atomic unsigned 32-bit integer.
pub use std::sync::atomic::AtomicU32;

#[cfg(target_has_atomic = "64")]
/// Atomic signed 64-bit integer.
pub use std::sync::atomic::AtomicI64;

#[cfg(target_has_atomic = "64")]
/// Atomic unsigned 64-bit integer.
pub use std::sync::atomic::AtomicU64;

#[cfg(target_has_atomic = "ptr")]
/// Atomic signed pointer-sized integer.
pub use std::sync::atomic::AtomicIsize;

#[cfg(target_has_atomic = "ptr")]
/// Atomic unsigned pointer-sized integer.
pub use std::sync::atomic::AtomicUsize;

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;
    use std::thread;

    // ------------------------------------------------------------------------
    // MUTEX
    // ------------------------------------------------------------------------

    #[test]
    fn mutex_provides_exclusive_access() {
        let mutex = Mutex::new(10_u64);

        {
            let mut value = mutex.lock().expect("mutex lock");

            *value += 5;
        }

        assert_eq!(*mutex.lock().expect("mutex lock"), 15);
    }

    #[test]
    fn mutex_try_lock_reports_contention() {
        let mutex = Mutex::new(10_u64);

        let guard = mutex.lock().expect("mutex lock");

        assert!(mutex.try_lock().expect("try lock").is_none());

        drop(guard);

        assert!(mutex.try_lock().expect("try lock").is_some());
    }

    #[test]
    fn mutex_supports_concurrent_updates() {
        let counter = Arc::new(Mutex::new(0_u64));

        let workers = thread::available_parallelism()
            .map(|value| value.get())
            .unwrap_or(1);

        let mut handles = Vec::new();

        for _ in 0..workers {
            let counter = Arc::clone(&counter);

            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    let mut value = counter.lock().expect("lock");

                    *value += 1;
                }
            }));
        }

        for handle in handles {
            handle.join().expect("worker");
        }

        assert_eq!(
            *counter.lock().expect("final lock"),
            (workers * 100) as u64
        );
    }

    #[test]
    fn mutex_reports_poisoning() {
        let mutex = Arc::new(Mutex::new(0_u64));

        let worker_mutex = Arc::clone(&mutex);

        let worker = thread::spawn(move || {
            let _guard = worker_mutex.lock().expect("lock");

            panic!("intentional poisoning");
        });

        assert!(worker.join().is_err());

        assert!(mutex.is_poisoned());

        assert!(mutex.lock().is_err());

        mutex.clear_poison();

        assert!(mutex.lock().is_ok());
    }

    // ------------------------------------------------------------------------
    // READER / WRITER LOCK
    // ------------------------------------------------------------------------

    #[test]
    fn rwlock_allows_shared_reading() {
        let lock = RwLock::new(42_u64);

        let first = lock.read().expect("first reader");

        let second = lock.read().expect("second reader");

        assert_eq!(*first, 42);

        assert_eq!(*second, 42);
    }

    #[test]
    fn rwlock_allows_exclusive_writing() {
        let lock = RwLock::new(42_u64);

        {
            let mut value = lock.write().expect("writer");

            *value = 84;
        }

        assert_eq!(*lock.read().expect("reader"), 84);
    }

    #[test]
    fn rwlock_try_write_reports_contention() {
        let lock = RwLock::new(42_u64);

        let reader = lock.read().expect("reader");

        assert!(lock.try_write().expect("try write").is_none());

        drop(reader);

        assert!(lock.try_write().expect("try write").is_some());
    }

    // ------------------------------------------------------------------------
    // CONDITION VARIABLE
    // ------------------------------------------------------------------------

    #[test]
    fn condition_variable_coordinates_threads() {
        let state = Arc::new(Mutex::new(false));

        let condition = Arc::new(Condvar::new());

        let worker_state = Arc::clone(&state);

        let worker_condition = Arc::clone(&condition);

        let worker = thread::spawn(move || {
            let mut ready = worker_state.lock().expect("worker lock");

            *ready = true;

            worker_condition.notify_one();
        });

        let mut ready = state.lock().expect("main lock");

        while !*ready {
            ready = condition.wait(ready).expect("condition wait");
        }

        assert!(*ready);

        drop(ready);

        worker.join().expect("worker");
    }

    #[test]
    fn condition_variable_timeout_is_reported() {
        let mutex = Mutex::new(false);

        let condition = Condvar::new();

        let guard = mutex.lock().expect("lock");

        let (guard, timed_out) = condition
            .wait_timeout(guard, Duration::from_millis(1))
            .expect("timed wait");

        assert!(timed_out);

        assert!(!*guard);
    }

    #[test]
    fn condition_variable_wait_while_checks_predicate() {
        let mutex = Mutex::new(false);

        let condition = Condvar::new();

        let guard = mutex.lock().expect("lock");

        let guard = condition
            .wait_while(guard, |ready| !*ready)
            .unwrap_or_else(|_| panic!("wait should only return after notification"));

        assert!(*guard);
    }

    // ------------------------------------------------------------------------
    // BARRIER
    // ------------------------------------------------------------------------

    #[test]
    fn barrier_waits_for_all_participants() {
        let barrier = Arc::new(Barrier::new(
            NonZeroUsize::new(3).expect("nonzero"),
        ));

        let completed = Arc::new(Mutex::new(0_u64));

        let mut handles = Vec::new();

        for _ in 0..3 {
            let barrier = Arc::clone(&barrier);

            let completed = Arc::clone(&completed);

            handles.push(thread::spawn(move || {
                barrier.wait();

                let mut value = completed.lock().expect("lock");

                *value += 1;
            }));
        }

        for handle in handles {
            handle.join().expect("worker");
        }

        assert_eq!(*completed.lock().expect("final lock"), 3);
    }

    #[test]
    fn barrier_is_reusable() {
        let barrier = Barrier::new(
            NonZeroUsize::new(1).expect("nonzero"),
        );

        barrier.wait();

        barrier.wait();

        assert_eq!(barrier.participants().get(), 1);
    }

    // ------------------------------------------------------------------------
    // ONCE LOCK
    // ------------------------------------------------------------------------

    #[test]
    fn once_lock_initializes_once() {
        let cell = OnceLock::new();

        assert!(cell.get().is_none());

        let first = cell.get_or_init(|| 42_u64);

        assert_eq!(*first, 42);

        let second = cell.get_or_init(|| 100_u64);

        assert_eq!(*second, 42);
    }

    #[test]
    fn once_lock_supports_concurrent_initialization() {
        let cell = Arc::new(OnceLock::new());

        let workers = thread::available_parallelism()
            .map(|value| value.get())
            .unwrap_or(1);

        let mut handles = Vec::new();

        for _ in 0..workers {
            let cell = Arc::clone(&cell);

            handles.push(thread::spawn(move || {
                *cell.get_or_init(|| 42_u64)
            }));
        }

        for handle in handles {
            assert_eq!(handle.join().expect("worker"), 42);
        }

        assert_eq!(cell.get(), Some(&42));
    }

    // ------------------------------------------------------------------------
    // ATOMIC OPERATIONS
    // ------------------------------------------------------------------------

    #[cfg(target_has_atomic = "64")]
    #[test]
    fn atomic_operations_support_concurrent_updates() {
        use std::sync::atomic::AtomicU64;

        let counter = Arc::new(AtomicU64::new(0));

        let workers = thread::available_parallelism()
            .map(|value| value.get())
            .unwrap_or(1);

        let mut handles = Vec::new();

        for _ in 0..workers {
            let counter = Arc::clone(&counter);

            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    counter.fetch_add(1, MemoryOrdering::Relaxed);
                }
            }));
        }

        for handle in handles {
            handle.join().expect("worker");
        }

        assert_eq!(
            counter.load(MemoryOrdering::Relaxed),
            (workers * 100) as u64
        );
    }

    #[cfg(target_has_atomic = "8")]
    #[test]
    fn atomic_boolean_operations_work() {
        let value = AtomicBool::new(false);

        assert!(!value.load(MemoryOrdering::Relaxed));

        value.store(true, MemoryOrdering::Release);

        assert!(value.load(MemoryOrdering::Acquire));
    }

    // ------------------------------------------------------------------------
    // DEFAULT CONSTRUCTION
    // ------------------------------------------------------------------------

    #[test]
    fn synchronization_primitives_support_default_values() {
        let mutex = Mutex::<u64>::default();

        let rwlock = RwLock::<u64>::default();

        let once = OnceLock::<u64>::default();

        let condition = Condvar::default();

        assert_eq!(*mutex.lock().expect("mutex"), 0);

        assert_eq!(*rwlock.read().expect("rwlock"), 0);

        assert!(once.get().is_none());

        condition.notify_all();
    }
}
