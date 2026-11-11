#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]


```rust
//! Zamani Standard Library — Synchronization
//!
//! Portable synchronization primitives for Zamani programs.
//!
//! # Ownership
//!
//! This module owns:
//! - Portable access to classical synchronization primitives.
//! - Mutex and read/write lock APIs.
//! - Condition-variable APIs.
//! - Reusable local thread barriers.
//! - One-time initialization primitives.
//! - Atomic primitive and memory-ordering re-exports.
//!
//! This module does not own:
//! - Task creation or task scheduling.
//! - Async executor implementation.
//! - Thread-pool management.
//! - Hardware discovery or allocation.
//! - Distributed lock protocols.
//! - Quantum operation scheduling.
//! - HDL synthesis or physical synchronization.
//! - Semantic validation or concurrency IR.
//!
//! Those responsibilities belong to the compiler, semantic,
//! runtime, distributed, quantum, HDL, and HAL authorities.
//!
//! # Portability
//!
//! The primitives in this module are portable Rust abstractions.
//! Their implementation may vary by target platform.
//!
//! Atomic types are available only when supported by the target's
//! Rust standard library. Availability of an atomic type does not
//! imply that the operation is lock-free on every target.
//!
//! # Scalability
//!
//! This module imposes no application-level maximum on:
//! - Locks.
//! - Atomic objects.
//! - Barriers.
//! - Synchronization operations.
//! - Threads.
//! - Participants.
//!
//! Actual limits arise from target capabilities, available memory,
//! operating-system resources, and runtime policy.
//!
//! # Poisoning
//!
//! Mutex and RwLock poisoning follows Rust's standard-library
//! behavior. Callers must explicitly decide whether and how to
//! recover from a poisoned lock.
//!
//! # Safety
//!
//! This module uses safe Rust only.

use std::num::NonZeroUsize;
use std::sync;

/// Standard mutex, including its standard poisoning behavior.
///
/// A mutex provides exclusive access to the protected value.
/// It does not provide distributed mutual exclusion.
pub use std::sync::{
    Condvar,
    Mutex,
    MutexGuard,
    Once,
    OnceLock,
    PoisonError,
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard,
    TryLockError,
    WaitTimeoutResult,
};

/// Atomic primitives supported by the current compilation target.
///
/// This re-export intentionally follows the target's available
/// standard-library atomic types rather than imposing a fixed
/// set of universally available widths.
///
/// `Ordering` is available through this module.
pub mod atomic {
    pub use std::sync::atomic::*;
}

/// A reusable, local synchronization barrier.
///
/// A barrier releases participating threads when the configured
/// number of participants have called [`Barrier::wait`].
///
/// This primitive is local to the synchronization domain supported
/// by the underlying Rust target. It is not a distributed barrier.
///
/// The participant count is supplied by the caller and is not
/// restricted by a Zamani-specific fixed ceiling.
#[derive(Debug)]
pub struct Barrier {
    inner: sync::Barrier,
    participants: NonZeroUsize,
}

/// Result returned by a barrier wait.
///
/// Exactly one participant receives the leader result for each
/// completed barrier generation.
pub use std::sync::BarrierWaitResult;

/// Error returned when constructing a barrier with no participants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidBarrierParticipants;

impl std::fmt::Display for InvalidBarrierParticipants {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        formatter.write_str(
            "a synchronization barrier requires at least one participant",
        )
    }
}

impl std::error::Error for InvalidBarrierParticipants {}

impl Barrier {
    /// Creates a barrier with a non-zero participant count.
    ///
    /// `NonZeroUsize` makes the invalid zero-participant state
    /// unrepresentable through this constructor.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZeroUsize;
    /// use zamani_compiler::stdlib::concurrency::synchronization::Barrier;
    ///
    /// let barrier = Barrier::new(
    ///     NonZeroUsize::new(2).expect("two is non-zero")
    /// );
    ///
    /// assert_eq!(barrier.participant_count().get(), 2);
    /// ```
    #[must_use]
    pub fn new(participants: NonZeroUsize) -> Self {
        Self {
            inner: sync::Barrier::new(participants.get()),
            participants,
        }
    }

    /// Attempts to create a barrier from a runtime-supplied count.
    ///
    /// Returns an error for zero rather than panicking.
    pub fn try_new(
        participants: usize,
    ) -> Result<Self, InvalidBarrierParticipants> {
        let participants = NonZeroUsize::new(participants)
            .ok_or(InvalidBarrierParticipants)?;

        Ok(Self::new(participants))
    }

    /// Returns the configured number of participants.
    #[must_use]
    pub const fn participant_count(&self) -> NonZeroUsize {
        self.participants
    }

    /// Waits until all participants in the current generation arrive.
    ///
    /// The returned result identifies the participant that became
    /// the leader for this generation.
    ///
    /// A barrier is reusable. Each completed generation releases
    /// its participants before the next generation proceeds.
    ///
    /// This operation blocks the calling operating-system thread.
    /// Async tasks should not call it directly on an executor thread
    /// unless the executor explicitly permits blocking operations.
    #[must_use]
    pub fn wait(&self) -> BarrierWaitResult {
        self.inner.wait()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn barrier_rejects_zero_participants() {
        assert!(Barrier::try_new(0).is_err());
    }

    #[test]
    fn barrier_accepts_nonzero_participants() {
        let barrier = Barrier::try_new(1)
            .expect("one participant is valid");

        assert_eq!(barrier.participant_count().get(), 1);
    }

    #[test]
    fn barrier_reports_exactly_one_leader() {
        let barrier = Arc::new(
            Barrier::new(
                NonZeroUsize::new(4).expect("four is non-zero"),
            ),
        );

        let leaders = Arc::new(AtomicUsize::new(0));

        let workers: Vec<_> = (0..4)
            .map(|_| {
                let barrier = Arc::clone(&barrier);
                let leaders = Arc::clone(&leaders);

                thread::spawn(move || {
                    if barrier.wait().is_leader() {
                        leaders.fetch_add(1, Ordering::Relaxed);
                    }
                })
            })
            .collect();

        for worker in workers {
            worker.join().expect("worker should complete");
        }

        assert_eq!(leaders.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn barrier_can_be_reused() {
        let barrier = Arc::new(
            Barrier::new(
                NonZeroUsize::new(2).expect("two is non-zero"),
            ),
        );

        let worker_barrier = Arc::clone(&barrier);

        let worker = thread::spawn(move || {
            worker_barrier.wait();
            worker_barrier.wait();
        });

        barrier.wait();
        barrier.wait();

        worker.join().expect("worker should complete");
    }

    #[test]
    fn mutex_provides_mutual_exclusion() {
        let value = Arc::new(Mutex::new(0usize));

        let workers: Vec<_> = (0..8)
            .map(|_| {
                let value = Arc::clone(&value);

                thread::spawn(move || {
                    for _ in 0..1_000 {
                        let mut guard = value
                            .lock()
                            .expect("mutex should not be poisoned");

                        *guard += 1;
                    }
                })
            })
            .collect();

        for worker in workers {
            worker.join().expect("worker should complete");
        }

        assert_eq!(
            *value.lock().expect("mutex should not be poisoned"),
            8_000
        );
    }

    #[test]
    fn read_write_lock_supports_shared_reads_and_exclusive_writes() {
        let value = RwLock::new(10usize);

        {
            let first = value.read().expect("read lock should succeed");
            let second = value.read().expect("read lock should succeed");

            assert_eq!(*first, 10);
            assert_eq!(*second, 10);
        }

        {
            let mut writer = value.write().expect("write lock should succeed");
            *writer = 20;
        }

        assert_eq!(
            *value.read().expect("read lock should succeed"),
            20
        );
    }

    #[test]
    fn condition_variable_waits_for_predicate() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let worker_pair = Arc::clone(&pair);

        let worker = thread::spawn(move || {
            let (lock, condition) = &*worker_pair;

            let mut ready = lock
                .lock()
                .expect("mutex should not be poisoned");

            *ready = true;
            condition.notify_one();
        });

        let (lock, condition) = &*pair;

        let mut ready = lock
            .lock()
            .expect("mutex should not be poisoned");

        while !*ready {
            ready = condition
                .wait(ready)
                .expect("mutex should not be poisoned");
        }

        assert!(*ready);

        drop(ready);
        worker.join().expect("worker should complete");
    }

    #[test]
    fn once_executes_initialization_once() {
        static INITIALIZE: Once = Once::new();
        static VALUE: AtomicUsize = AtomicUsize::new(0);

        let workers: Vec<_> = (0..8)
            .map(|_| {
                thread::spawn(|| {
                    INITIALIZE.call_once(|| {
                        VALUE.store(42, Ordering::SeqCst);
                    });
                })
            })
            .collect();

        for worker in workers {
            worker.join().expect("worker should complete");
        }

        assert_eq!(VALUE.load(Ordering::SeqCst), 42);
    }

    #[test]
    fn once_lock_initializes_and_returns_shared_value() {
        let value = OnceLock::new();

        let initialized = value.get_or_init(|| String::from("Zamani"));

        assert_eq!(initialized, "Zamani");
        assert_eq!(value.get().map(String::as_str), Some("Zamani"));
    }

    #[test]
    fn poisoned_mutex_reports_poisoning() {
        let value = Arc::new(Mutex::new(0usize));
        let worker_value = Arc::clone(&value);

        let worker = thread::spawn(move || {
            let _guard = worker_value
                .lock()
                .expect("initial lock should succeed");

            panic!("deliberately poison the mutex");
        });

        assert!(worker.join().is_err());
        assert!(value.lock().is_err());
    }

    #[test]
    fn atomic_operations_use_explicit_ordering() {
        let value = atomic::AtomicUsize::new(0);

        value.store(7, atomic::Ordering::Release);

        assert_eq!(
            value.load(atomic::Ordering::Acquire),
            7
        );

        assert_eq!(
            value.fetch_add(1, atomic::Ordering::SeqCst),
            7
        );

        assert_eq!(
            value.load(atomic::Ordering::SeqCst),
            8
        );
    }
}
```
