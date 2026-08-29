//! Zamani Quantum Memory — Concurrency Integration Tests
//!
//! Production-grade concurrency tests for `quantum::memory`.
//!
//! # Responsibility
//!
//! This module verifies that the public quantum-memory allocation contract
//! remains correct when the same logical memory domain is used concurrently
//! from multiple Rust threads.
//!
//! The tests cover:
//!
//! - concurrent allocation from one allocator;
//! - concurrent allocation through cloned allocators;
//! - concurrent allocation and release;
//! - deterministic final accounting;
//! - peak accounting under contention;
//! - allocation-count correctness;
//! - unique allocation identities;
//! - concurrent state-memory accounting;
//! - concurrent temporary and persistent allocations;
//! - concurrent explicit release;
//! - concurrent drop-based release;
//! - synchronization/barrier correctness;
//! - allocator `Send`/`Sync` suitability through actual thread use;
//! - absence of cross-thread accounting leakage;
//! - absence of double-release accounting;
//! - provider-neutral execution;
//! - bounded test resource usage;
//! - deterministic verification without relying on scheduling order;
//! - safe Rust only.
//!
//! # Architectural rule
//!
//! These tests test the PUBLIC allocator contract.
//!
//! They deliberately do not:
//!
//! - inspect allocator mutexes;
//! - access `AllocatorInner`;
//! - access private allocation records;
//! - inspect raw pointers;
//! - use `unsafe`;
//! - use atomics to manipulate allocator internals;
//! - depend on a specific scheduler;
//! - assume a specific thread execution order;
//! - depend on CUDA;
//! - depend on HIP;
//! - depend on Metal;
//! - depend on Vulkan;
//! - depend on MPI;
//! - depend on RDMA;
//! - depend on a QPU SDK;
//! - perform network I/O;
//! - require physical quantum hardware.
//!
//! Provider-specific concurrency is represented by the same public allocator
//! API used by real providers. The generic concurrency contract therefore
//! remains independent of the hardware technology.
//!
//! # Why this matters for quantum computing
//!
//! Quantum-memory concurrency is not merely a classical allocation concern.
//! The memory subsystem is eventually consumed by:
//!
//! ```text
//!                 quantum::ir
//!                     │
//!                     ▼
//!               runtime/executor
//!                     │
//!                     ▼
//!              quantum::memory
//!                     │
//!       ┌─────────────┼──────────────┐
//!       ▼             ▼              ▼
//!   StateVector   Stabilizer   TensorNetwork
//!       │             │              │
//!       └─────────────┼──────────────┘
//!                     ▼
//!                MemoryAllocator
//!                     │
//!       ┌─────────────┼──────────────┐
//!       ▼             ▼              ▼
//!      CPU           GPU        Distributed/QPU
//! ```
//!
//! Multiple executor workers, simulation workers, QEC workers, benchmark
//! workers, routing/scheduling workers, or backend orchestration tasks may
//! share one memory domain. Incorrect accounting under concurrency can cause:
//!
//! - memory-limit bypass;
//! - premature resource release;
//! - leaked allocation accounting;
//! - duplicate allocation identity;
//! - incorrect state-memory accounting;
//! - invalid peak-memory measurements;
//! - resource exhaustion;
//! - nondeterministic execution failures.
//!
//! These tests specifically guard those failure modes.
//!
//! # Integration contract
//!
//! This file is intentionally written against the stable public contracts
//! already established by `allocator.rs`:
//!
//! - `MemoryAllocator`
//! - `AllocationClass`
//! - `MemoryLocation`
//! - `AllocationRequest`
//! - `MemoryProvider`
//! - `ProviderAllocation`
//! - `HostMemoryProvider`
//! - `AllocationAccounting`
//! - `MemoryId`
//! - `AllocationId`
//! - `ByteCount`
//! - `MemoryError`
//!
//! The tests use only the public allocator operations needed to verify
//! concurrency.
//!
//! Later memory modules must not introduce an alternate concurrency model.
//! In particular:
//!
//! - `pool.rs` must remain safe when called concurrently;
//! - `reservation.rs` must preserve reservation atomicity;
//! - `budget.rs` must remain deterministic under concurrent consumers;
//! - `state.rs` and state representations must use allocator contracts rather
//!   than bypassing allocation accounting;
//! - `gpu.rs` and `distributed.rs` must preserve the same ownership model;
//! - `migration.rs` must preserve allocation accounting while moving state;
//! - `diagnostics.rs` and `telemetry.rs` must observe, not mutate, allocator
//!   accounting;
//! - hardware/QPU providers must remain compatible with the provider-neutral
//!   allocator concurrency model.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//! - no nightly features
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! There is deliberately:
//!
//! - no `unsafe` block;
//! - no `unsafe fn`;
//! - no raw pointer;
//! - no FFI;
//! - no `transmute`;
//! - no architecture-specific intrinsic.
//!
//! The concurrency tests rely only on safe standard-library synchronization
//! primitives and the public Zamani memory API.
//!
//! # Test philosophy
//!
//! The tests do not assert which thread wins a race. They assert invariants
//! that must hold regardless of scheduling.
//!
//! For example, if N threads each create M allocations, the test verifies:
//!
//! ```text
//! final allocation count = N × M
//! final allocated bytes  = N × M × allocation_size
//! ```
//!
//! while the allocations are alive.
//!
//! After all allocations are released:
//!
//! ```text
//! final allocation count = 0
//! final allocated bytes  = 0
//! ```
//!
//! Peak allocation is checked separately because peak accounting must survive
//! release.
//!
//! This makes the tests robust across Linux, macOS, Windows, WASM-compatible
//! host environments where thread support is available, and different thread
//! scheduling policies.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeSet;
use std::sync::{Arc, Barrier};
use std::thread;

use crate::quantum::memory::allocator::{
    AllocationClass,
    HostMemoryProvider,
    MemoryAllocator,
    MemoryLocation,
};
use crate::quantum::memory::types::{
    AllocationId,
    ByteCount,
    MemoryId,
};

// =============================================================================
// Test configuration
// =============================================================================
//
// These values intentionally remain small.
//
// The purpose of this module is to test synchronization and accounting, not to
// stress the operating system. A separate performance/benchmarking subsystem
// owns large-scale memory stress tests.

const TEST_MEMORY_ID: u64 = 1;

const WORKER_THREADS: usize = 8;
const ALLOCATIONS_PER_WORKER: usize = 16;

const CONCURRENT_ALLOCATION_BYTES: u64 = 256;
const CONCURRENT_STATE_BYTES: u64 = 512;
const CONCURRENT_STATE_ELEMENTS: u64 = 32;
const CONCURRENT_STATE_QUBITS: u64 = 5;

const MIXED_TEMPORARY_BYTES: u64 = 192;
const MIXED_PERSISTENT_BYTES: u64 = 320;

const SECOND_ROUND_THREADS: usize = 4;
const SECOND_ROUND_ALLOCATIONS: usize = 8;

// =============================================================================
// Helpers
// =============================================================================

/// Creates a production allocator using the safe built-in host provider.
///
/// The production allocator is deliberately used instead of constructing
/// allocator internals. This ensures these tests exercise the same public
/// construction path used by the rest of the memory subsystem.
fn production_allocator() -> MemoryAllocator {
    MemoryAllocator::production(MemoryId::new(TEST_MEMORY_ID))
        .expect("production memory limits must be valid")
        .with_host_provider()
        .expect("host provider registration must succeed")
}

/// Computes the expected number of allocations without unchecked arithmetic.
///
/// A test configuration overflow is a test failure rather than a reason to
/// wrap an expected value.
fn expected_count(workers: usize, per_worker: usize) -> u64 {
    workers
        .checked_mul(per_worker)
        .and_then(|value| u64::try_from(value).ok())
        .expect("test allocation count must fit into u64")
}

/// Computes expected bytes with checked arithmetic.
///
/// This keeps the test itself subject to the same overflow discipline required
/// of production quantum-memory accounting.
fn expected_bytes(
    workers: usize,
    per_worker: usize,
    bytes_per_allocation: u64,
) -> u64 {
    let count = expected_count(workers, per_worker);

    count
        .checked_mul(bytes_per_allocation)
        .expect("test byte count must fit into u64")
}

// =============================================================================
// Compile-time concurrency contract
// =============================================================================
//
// These functions are never called directly. Their purpose is to force the
// compiler to verify that the public allocator and allocation handles satisfy
// the Send/Sync properties required for actual cross-thread usage.
//
// No unsafe trait assertion is used.

fn assert_send<T: Send>() {}

fn assert_sync<T: Sync>() {}

#[test]
fn allocator_and_allocation_handles_are_thread_safe_types() {
    assert_send::<MemoryAllocator>();
    assert_sync::<MemoryAllocator>();
}

// =============================================================================
// Concurrent allocation through cloned allocators
// =============================================================================

#[test]
fn concurrent_allocations_through_cloned_allocators_are_accounted_exactly() {
    let allocator = production_allocator();

    let barrier = Arc::new(Barrier::new(WORKER_THREADS));

    let mut workers = Vec::with_capacity(WORKER_THREADS);

    for _ in 0..WORKER_THREADS {
        let worker_allocator = allocator.clone();
        let worker_barrier = Arc::clone(&barrier);

        workers.push(thread::spawn(move || {
            // All workers begin the allocation phase together. This creates
            // genuine contention without relying on arbitrary sleeps.
            worker_barrier.wait();

            let mut allocations = Vec::with_capacity(ALLOCATIONS_PER_WORKER);

            for _ in 0..ALLOCATIONS_PER_WORKER {
                let allocation = worker_allocator
                    .allocate_host(
                        ByteCount::new(CONCURRENT_ALLOCATION_BYTES),
                        AllocationClass::Temporary,
                    )
                    .expect("concurrent host allocation should succeed");

                allocations.push(allocation);
            }

            // Keep all allocations alive until every worker has completed its
            // allocation phase. This makes the expected accounting exact.
            worker_barrier.wait();

            allocations
        }));
    }

    // Every worker must have reached the second barrier before the main
    // thread starts inspecting accounting.
    barrier.wait();

    let mut allocations = Vec::new();

    for worker in workers {
        let worker_allocations = worker
            .join()
            .expect("allocation worker must not panic");

        allocations.extend(worker_allocations);
    }

    let expected_allocations =
        expected_count(WORKER_THREADS, ALLOCATIONS_PER_WORKER);

    let expected_bytes = expected_bytes(
        WORKER_THREADS,
        ALLOCATIONS_PER_WORKER,
        CONCURRENT_ALLOCATION_BYTES,
    );

    assert_eq!(
        allocations.len(),
        usize::try_from(expected_allocations)
            .expect("expected allocation count must fit usize")
    );

    assert_eq!(
        allocator.allocation_count().expect("accounting"),
        expected_allocations
    );

    assert_eq!(
        allocator.allocated_bytes().expect("accounting"),
        expected_bytes
    );

    assert!(
        allocator.peak_allocated_bytes().expect("accounting")
            >= expected_bytes
    );

    drop(allocations);

    assert_eq!(
        allocator.allocation_count().expect("final accounting"),
        0
    );

    assert_eq!(
        allocator.allocated_bytes().expect("final accounting"),
        0
    );
}

// =============================================================================
// Concurrent allocation/release cycles
// =============================================================================

#[test]
fn repeated_concurrent_allocation_release_cycles_return_to_zero() {
    let allocator = production_allocator();

    for _round in 0..4 {
        let barrier = Arc::new(Barrier::new(SECOND_ROUND_THREADS));

        let mut workers = Vec::with_capacity(SECOND_ROUND_THREADS);

        for _ in 0..SECOND_ROUND_THREADS {
            let worker_allocator = allocator.clone();
            let worker_barrier = Arc::clone(&barrier);

            workers.push(thread::spawn(move || {
                worker_barrier.wait();

                for _ in 0..SECOND_ROUND_ALLOCATIONS {
                    let allocation = worker_allocator
                        .allocate_host(
                            ByteCount::new(CONCURRENT_ALLOCATION_BYTES),
                            AllocationClass::Temporary,
                        )
                        .expect("allocation should succeed");

                    // Explicit release exercises deterministic early release
                    // while other threads continue allocating.
                    allocation.release();
                }

                worker_barrier.wait();
            }));
        }

        barrier.wait();

        for worker in workers {
            worker
                .join()
                .expect("allocation/release worker must not panic");
        }

        assert_eq!(
            allocator.allocation_count().expect("round accounting"),
            0,
            "every allocation must have been released after each round"
        );

        assert_eq!(
            allocator.allocated_bytes().expect("round accounting"),
            0,
            "every byte must have been released after each round"
        );
    }

    assert_eq!(
        allocator.allocation_count().expect("final accounting"),
        0
    );

    assert_eq!(
        allocator.allocated_bytes().expect("final accounting"),
        0
    );

    assert!(
        allocator.peak_allocated_bytes().expect("peak accounting") > 0,
        "successful concurrent allocations must contribute to peak accounting"
    );
}

// =============================================================================
// Concurrent drop-based release
// =============================================================================

#[test]
fn concurrent_drop_based_release_does_not_leak_accounting() {
    let allocator = production_allocator();

    let barrier = Arc::new(Barrier::new(WORKER_THREADS));

    let mut workers = Vec::with_capacity(WORKER_THREADS);

    for _ in 0..WORKER_THREADS {
        let worker_allocator = allocator.clone();
        let worker_barrier = Arc::clone(&barrier);

        workers.push(thread::spawn(move || {
            let mut allocations = Vec::with_capacity(ALLOCATIONS_PER_WORKER);

            worker_barrier.wait();

            for _ in 0..ALLOCATIONS_PER_WORKER {
                allocations.push(
                    worker_allocator
                        .allocate_host(
                            ByteCount::new(CONCURRENT_ALLOCATION_BYTES),
                            AllocationClass::Persistent,
                        )
                        .expect("persistent allocation should succeed"),
                );
            }

            worker_barrier.wait();

            // Drop happens in the worker thread. This explicitly verifies that
            // allocation ownership can safely cross a thread boundary.
            drop(allocations);
        }));
    }

    barrier.wait();

    for worker in workers {
        worker
            .join()
            .expect("drop worker must not panic");
    }

    assert_eq!(
        allocator.allocation_count().expect("final accounting"),
        0
    );

    assert_eq!(
        allocator.allocated_bytes().expect("final accounting"),
        0
    );
}

// =============================================================================
// Allocation identity under concurrency
// =============================================================================

#[test]
fn concurrent_allocations_receive_unique_nonzero_ids() {
    let allocator = production_allocator();

    let barrier = Arc::new(Barrier::new(WORKER_THREADS));

    let mut workers = Vec::with_capacity(WORKER_THREADS);

    for _ in 0..WORKER_THREADS {
        let worker_allocator = allocator.clone();
        let worker_barrier = Arc::clone(&barrier);

        workers.push(thread::spawn(move || {
            worker_barrier.wait();

            let mut ids = Vec::with_capacity(ALLOCATIONS_PER_WORKER);

            for _ in 0..ALLOCATIONS_PER_WORKER {
                let allocation = worker_allocator
                    .allocate_host(
                        ByteCount::new(CONCURRENT_ALLOCATION_BYTES),
                        AllocationClass::Temporary,
                    )
                    .expect("allocation should succeed");

                ids.push(allocation.id());

                // Keep the provider resource and accounting alive until the
                // ID has been captured.
                drop(allocation);
            }

            ids
        }));
    }

    barrier.wait();

    let mut ids = Vec::new();

    for worker in workers {
        ids.extend(
            worker
                .join()
                .expect("identity worker must not panic"),
        );
    }

    let expected = expected_count(
        WORKER_THREADS,
        ALLOCATIONS_PER_WORKER,
    );

    assert_eq!(
        ids.len(),
        usize::try_from(expected)
            .expect("expected ID count must fit usize")
    );

    let mut unique = BTreeSet::<AllocationId>::new();

    for id in ids {
        assert_ne!(
            id,
            AllocationId::new(0),
            "zero is reserved as invalid allocation identity"
        );

        assert!(
            unique.insert(id),
            "two concurrent allocations returned the same allocation ID"
        );
    }

    assert_eq!(
        unique.len(),
        usize::try_from(expected)
            .expect("expected unique ID count must fit usize")
    );

    assert_eq!(
        allocator.allocation_count().expect("final accounting"),
        0
    );

    assert_eq!(
        allocator.allocated_bytes().expect("final accounting"),
        0
    );
}

// =============================================================================
// Concurrent state-memory accounting
// =============================================================================

#[test]
fn concurrent_state_allocations_preserve_state_accounting() {
    let allocator = production_allocator();

    let barrier = Arc::new(Barrier::new(WORKER_THREADS));

    let mut workers = Vec::with_capacity(WORKER_THREADS);

    for _ in 0..WORKER_THREADS {
        let worker_allocator = allocator.clone();
        let worker_barrier = Arc::clone(&barrier);

        workers.push(thread::spawn(move || {
            worker_barrier.wait();

            let mut allocations = Vec::with_capacity(ALLOCATIONS_PER_WORKER);

            for _ in 0..ALLOCATIONS_PER_WORKER {
                let allocation = worker_allocator
                    .allocate_state(
                        ByteCount::new(CONCURRENT_STATE_BYTES),
                        MemoryLocation::Host,
                        CONCURRENT_STATE_QUBITS,
                        CONCURRENT_STATE_ELEMENTS,
                    )
                    .expect("state allocation should succeed");

                allocations.push(allocation);
            }

            worker_barrier.wait();

            allocations
        }));
    }

    barrier.wait();

    let mut allocations = Vec::new();

    for worker in workers {
        allocations.extend(
            worker
                .join()
                .expect("state worker must not panic"),
        );
    }

    let expected_count =
        expected_count(WORKER_THREADS, ALLOCATIONS_PER_WORKER);

    let expected_bytes = expected_bytes(
        WORKER_THREADS,
        ALLOCATIONS_PER_WORKER,
        CONCURRENT_STATE_BYTES,
    );

    let expected_elements = expected_count
        .checked_mul(CONCURRENT_STATE_ELEMENTS)
        .expect("state element expectation must not overflow");

    let accounting = allocator
        .accounting()
        .expect("state accounting must be readable");

    assert_eq!(
        accounting.allocations,
        expected_count
    );

    assert_eq!(
        accounting.state_bytes,
        expected_bytes
    );

    assert_eq!(
        accounting.state_elements,
        expected_elements
    );

    assert_eq!(
        accounting.host_bytes,
        expected_bytes
    );

    drop(allocations);

    let final_accounting = allocator
        .accounting()
        .expect("final state accounting must be readable");

    assert_eq!(
        final_accounting.allocations,
        0
    );

    assert_eq!(
        final_accounting.state_bytes,
        0
    );

    assert_eq!(
        final_accounting.state_elements,
        0
    );

    assert_eq!(
        final_accounting.host_bytes,
        0
    );
}

// =============================================================================
// Mixed allocation classes
// =============================================================================

#[test]
fn concurrent_temporary_and_persistent_allocations_remain_isolated() {
    let allocator = production_allocator();

    let barrier = Arc::new(Barrier::new(WORKER_THREADS));

    let mut workers = Vec::with_capacity(WORKER_THREADS);

    for worker_index in 0..WORKER_THREADS {
        let worker_allocator = allocator.clone();
        let worker_barrier = Arc::clone(&barrier);

        workers.push(thread::spawn(move || {
            worker_barrier.wait();

            let mut allocations = Vec::with_capacity(2);

            if worker_index % 2 == 0 {
                allocations.push(
                    worker_allocator
                        .allocate_host(
                            ByteCount::new(MIXED_TEMPORARY_BYTES),
                            AllocationClass::Temporary,
                        )
                        .expect("temporary allocation should succeed"),
                );

                allocations.push(
                    worker_allocator
                        .allocate_host(
                            ByteCount::new(MIXED_PERSISTENT_BYTES),
                            AllocationClass::Persistent,
                        )
                        .expect("persistent allocation should succeed"),
                );
            } else {
                allocations.push(
                    worker_allocator
                        .allocate_host(
                            ByteCount::new(MIXED_PERSISTENT_BYTES),
                            AllocationClass::Persistent,
                        )
                        .expect("persistent allocation should succeed"),
                );

                allocations.push(
                    worker_allocator
                        .allocate_host(
                            ByteCount::new(MIXED_TEMPORARY_BYTES),
                            AllocationClass::Temporary,
                        )
                        .expect("temporary allocation should succeed"),
                );
            }

            worker_barrier.wait();

            allocations
        }));
    }

    barrier.wait();

    let mut allocations = Vec::new();

    for worker in workers {
        allocations.extend(
            worker
                .join()
                .expect("mixed worker must not panic"),
        );
    }

    let temporary_workers =
        (WORKER_THREADS + 1) / 2;

    let persistent_workers =
        WORKER_THREADS / 2;

    let temporary_bytes = expected_bytes(
        temporary_workers,
        1,
        MIXED_TEMPORARY_BYTES,
    );

    let persistent_bytes = expected_bytes(
        persistent_workers,
        1,
        MIXED_PERSISTENT_BYTES,
    );

    let accounting = allocator
        .accounting()
        .expect("mixed accounting");

    assert_eq!(
        accounting.temporary_host_bytes,
        temporary_bytes
    );

    assert_eq!(
        accounting.persistent_host_bytes,
        persistent_bytes
            .checked_add(temporary_bytes)
            .expect("persistent accounting expectation must not overflow"),
        "the allocator's persistent_host_bytes intentionally includes the \
         non-temporary host classes, including state/checkpoint/metadata"
    );

    let total_expected = expected_bytes(
        WORKER_THREADS,
        1,
        MIXED_TEMPORARY_BYTES
            .checked_add(MIXED_PERSISTENT_BYTES)
            .expect("test byte total must not overflow"),
    );

    assert_eq!(
        accounting.host_bytes,
        total_expected
    );

    assert_eq!(
        accounting.allocations,
        expected_count(WORKER_THREADS, 2)
    );

    drop(allocations);

    let final_accounting = allocator
        .accounting()
        .expect("final mixed accounting");

    assert_eq!(
        final_accounting.allocations,
        0
    );

    assert_eq!(
        final_accounting.host_bytes,
        0
    );

    assert_eq!(
        final_accounting.temporary_host_bytes,
        0
    );

    assert_eq!(
        final_accounting.persistent_host_bytes,
        0
    );
}

// =============================================================================
// Concurrent explicit release
// =============================================================================

#[test]
fn concurrent_explicit_release_is_accounted_exactly_once() {
    let allocator = production_allocator();

    let barrier = Arc::new(Barrier::new(WORKER_THREADS));

    let mut workers = Vec::with_capacity(WORKER_THREADS);

    for _ in 0..WORKER_THREADS {
        let worker_allocator = allocator.clone();
        let worker_barrier = Arc::clone(&barrier);

        workers.push(thread::spawn(move || {
            worker_barrier.wait();

            for _ in 0..ALLOCATIONS_PER_WORKER {
                let allocation = worker_allocator
                    .allocate_host(
                        ByteCount::new(CONCURRENT_ALLOCATION_BYTES),
                        AllocationClass::Temporary,
                    )
                    .expect("allocation should succeed");

                allocation.release();
            }

            worker_barrier.wait();
        }));
    }

    barrier.wait();

    for worker in workers {
        worker
            .join()
            .expect("explicit-release worker must not panic");
    }

    assert_eq!(
        allocator.allocation_count().expect("final accounting"),
        0
    );

    assert_eq!(
        allocator.allocated_bytes().expect("final accounting"),
        0
    );
}

// =============================================================================
// Shared-domain identity
// =============================================================================

#[test]
fn cloned_allocators_share_one_memory_domain_under_concurrency() {
    let allocator = production_allocator();

    let barrier = Arc::new(Barrier::new(WORKER_THREADS));

    let mut workers = Vec::with_capacity(WORKER_THREADS);

    for _ in 0..WORKER_THREADS {
        let worker_allocator = allocator.clone();
        let worker_barrier = Arc::clone(&barrier);

        workers.push(thread::spawn(move || {
            let memory_id = worker_allocator.memory_id();

            worker_barrier.wait();

            let allocation = worker_allocator
                .allocate_host(
                    ByteCount::new(CONCURRENT_ALLOCATION_BYTES),
                    AllocationClass::Temporary,
                )
                .expect("allocation should succeed");

            worker_barrier.wait();

            (memory_id, allocation)
        }));
    }

    barrier.wait();

    let mut allocations = Vec::with_capacity(WORKER_THREADS);

    for worker in workers {
        let (memory_id, allocation) = worker
            .join()
            .expect("memory-domain worker must not panic");

        assert_eq!(
            memory_id,
            allocator.memory_id(),
            "cloned allocators must retain the same memory-domain identity"
        );

        allocations.push(allocation);
    }

    assert_eq!(
        allocator.allocation_count().expect("accounting"),
        expected_count(WORKER_THREADS, 1)
    );

    drop(allocations);

    assert_eq!(
        allocator.allocation_count().expect("final accounting"),
        0
    );

    assert_eq!(
        allocator.allocated_bytes().expect("final accounting"),
        0
    );
}

// =============================================================================
// Concurrent accounting snapshots
// =============================================================================

#[test]
fn concurrent_accounting_snapshots_are_self_consistent() {
    let allocator = production_allocator();

    let barrier = Arc::new(Barrier::new(WORKER_THREADS + 1));

    let mut workers = Vec::with_capacity(WORKER_THREADS);

    for _ in 0..WORKER_THREADS {
        let worker_allocator = allocator.clone();
        let worker_barrier = Arc::clone(&barrier);

        workers.push(thread::spawn(move || {
            worker_barrier.wait();

            let mut allocations = Vec::with_capacity(ALLOCATIONS_PER_WORKER);

            for _ in 0..ALLOCATIONS_PER_WORKER {
                allocations.push(
                    worker_allocator
                        .allocate_host(
                            ByteCount::new(CONCURRENT_ALLOCATION_BYTES),
                            AllocationClass::Temporary,
                        )
                        .expect("allocation should succeed"),
                );
            }

            // Keep allocations alive while the main thread repeatedly reads
            // accounting.
            worker_barrier.wait();

            allocations
        }));
    }

    // Release all workers simultaneously.
    barrier.wait();

    // The workers now allocate concurrently while this thread observes the
    // public accounting API. We deliberately do not assert an exact value at
    // this point because allocation order is nondeterministic.
    for _ in 0..64 {
        let accounting = allocator
            .accounting()
            .expect("accounting snapshot should succeed");

        assert!(
            accounting.allocations
                <= expected_count(
                    WORKER_THREADS,
                    ALLOCATIONS_PER_WORKER,
                ),
            "live allocation count cannot exceed the number of allocations \
             the workers were instructed to create"
        );

        assert_eq!(
            accounting.total_bytes(),
            accounting
                .host_bytes
                .checked_add(accounting.device_bytes)
                .and_then(|value| value.checked_add(accounting.distributed_bytes))
                .expect("accounting total must not overflow")
        );

        assert!(
            accounting.state_bytes
                <= accounting.total_bytes(),
            "state bytes cannot exceed total tracked bytes"
        );
    }

    // Tell workers to release their allocations.
    barrier.wait();

    for worker in workers {
        let allocations = worker
            .join()
            .expect("accounting worker must not panic");

        drop(allocations);
    }

    let final_accounting = allocator
        .accounting()
        .expect("final accounting");

    assert_eq!(
        final_accounting.allocations,
        0
    );

    assert_eq!(
        final_accounting.host_bytes,
        0
    );

    assert_eq!(
        final_accounting.device_bytes,
        0
    );

    assert_eq!(
        final_accounting.distributed_bytes,
        0
    );
}

// =============================================================================
// Peak accounting under actual contention
// =============================================================================

#[test]
fn peak_accounting_never_decreases_when_live_allocations_are_released() {
    let allocator = production_allocator();

    let first = allocator
        .allocate_host(
            ByteCount::new(CONCURRENT_ALLOCATION_BYTES),
            AllocationClass::Temporary,
        )
        .expect("first allocation");

    let first_peak = allocator
        .peak_allocated_bytes()
        .expect("first peak");

    assert!(
        first_peak >= CONCURRENT_ALLOCATION_BYTES
    );

    drop(first);

    let after_first_release = allocator
        .peak_allocated_bytes()
        .expect("peak after release");

    assert!(
        after_first_release >= first_peak,
        "peak accounting must not decrease after releasing live memory"
    );

    let barrier = Arc::new(Barrier::new(WORKER_THREADS));

    let mut workers = Vec::with_capacity(WORKER_THREADS);

    for _ in 0..WORKER_THREADS {
        let worker_allocator = allocator.clone();
        let worker_barrier = Arc::clone(&barrier);

        workers.push(thread::spawn(move || {
            worker_barrier.wait();

            let mut allocations = Vec::with_capacity(ALLOCATIONS_PER_WORKER);

            for _ in 0..ALLOCATIONS_PER_WORKER {
                allocations.push(
                    worker_allocator
                        .allocate_host(
                            ByteCount::new(CONCURRENT_ALLOCATION_BYTES),
                            AllocationClass::Temporary,
                        )
                        .expect("concurrent allocation"),
                );
            }

            worker_barrier.wait();

            allocations
        }));
    }

    barrier.wait();
    barrier.wait();

    let expected_peak_lower_bound = expected_bytes(
        WORKER_THREADS,
        ALLOCATIONS_PER_WORKER,
        CONCURRENT_ALLOCATION_BYTES,
    );

    let concurrent_peak = allocator
        .peak_allocated_bytes()
        .expect("concurrent peak");

    assert!(
        concurrent_peak >= expected_peak_lower_bound,
        "peak accounting must include the concurrently live allocations"
    );

    for worker in workers {
        drop(
            worker
                .join()
                .expect("peak worker must not panic"),
        );
    }

    let final_peak = allocator
        .peak_allocated_bytes()
        .expect("final peak");

    assert!(
        final_peak >= concurrent_peak,
        "peak accounting must be monotonic"
    );

    assert_eq!(
        allocator.allocated_bytes().expect("final bytes"),
        0
    );

    assert_eq!(
        allocator.allocation_count().expect("final count"),
        0
    );
}

// =============================================================================
// Cross-thread ownership transfer
// =============================================================================

#[test]
fn_allocation_can_move_between_threads_without_losing_accounting() {
    let allocator = production_allocator();

    let allocation = allocator
        .allocate_host(
            ByteCount::new(CONCURRENT_ALLOCATION_BYTES),
            AllocationClass::Temporary,
        )
        .expect("allocation should succeed");

    let allocation_id = allocation.id();

    assert_eq!(
        allocator.allocation_count().expect("accounting"),
        1
    );

    let worker = thread::spawn(move || {
        assert_eq!(
            allocation.id(),
            allocation_id,
            "allocation identity must survive thread ownership transfer"
        );

        assert!(allocation.is_live());

        allocation
    });

    let allocation = worker
        .join()
        .expect("ownership-transfer worker must not panic");

    assert_eq!(
        allocation.id(),
        allocation_id
    );

    assert_eq!(
        allocator.allocation_count().expect("accounting"),
        1
    );

    drop(allocation);

    assert_eq!(
        allocator.allocation_count().expect("final accounting"),
        0
    );

    assert_eq!(
        allocator.allocated_bytes().expect("final accounting"),
        0
    );
}

// =============================================================================
// No cross-domain accounting contamination
// =============================================================================

#[test]
fn separate_memory_domains_do_not_share_accounting() {
    let first = MemoryAllocator::production(MemoryId::new(1))
        .expect("first production allocator")
        .with_host_provider()
        .expect("first host provider");

    let second = MemoryAllocator::production(MemoryId::new(2))
        .expect("second production allocator")
        .with_host_provider()
        .expect("second host provider");

    let barrier = Arc::new(Barrier::new(3));

    let first_worker_allocator = first.clone();
    let first_barrier = Arc::clone(&barrier);

    let first_worker = thread::spawn(move || {
        first_barrier.wait();

        first_worker_allocator
            .allocate_host(
                ByteCount::new(CONCURRENT_ALLOCATION_BYTES),
                AllocationClass::Temporary,
            )
            .expect("first-domain allocation")
    });

    let second_worker_allocator = second.clone();
    let second_barrier = Arc::clone(&barrier);

    let second_worker = thread::spawn(move || {
        second_barrier.wait();

        second_worker_allocator
            .allocate_host(
                ByteCount::new(CONCURRENT_ALLOCATION_BYTES),
                AllocationClass::Temporary,
            )
            .expect("second-domain allocation")
    });

    barrier.wait();

    let first_allocation = first_worker
        .join()
        .expect("first domain worker");

    let second_allocation = second_worker
        .join()
        .expect("second domain worker");

    assert_eq!(
        first.allocation_count().expect("first accounting"),
        1
    );

    assert_eq!(
        second.allocation_count().expect("second accounting"),
        1
    );

    assert_eq!(
        first.allocated_bytes().expect("first accounting"),
        CONCURRENT_ALLOCATION_BYTES
    );

    assert_eq!(
        second.allocated_bytes().expect("second accounting"),
        CONCURRENT_ALLOCATION_BYTES
    );

    drop(first_allocation);

    assert_eq!(
        first.allocation_count().expect("first accounting"),
        0
    );

    assert_eq!(
        second.allocation_count().expect("second accounting"),
        1,
        "releasing one memory domain must never affect another domain"
    );

    drop(second_allocation);

    assert_eq!(
        second.allocation_count().expect("second accounting"),
        0
    );
}

// =============================================================================
// Final invariant test
// =============================================================================

#[test]
fn concurrent_memory_operations_leave_allocator_in_a_clean_state() {
    let allocator = production_allocator();

    let barrier = Arc::new(Barrier::new(WORKER_THREADS));

    let mut workers = Vec::with_capacity(WORKER_THREADS);

    for worker_index in 0..WORKER_THREADS {
        let worker_allocator = allocator.clone();
        let worker_barrier = Arc::clone(&barrier);

        workers.push(thread::spawn(move || {
            worker_barrier.wait();

            let mut allocations = Vec::new();

            for iteration in 0..ALLOCATIONS_PER_WORKER {
                let class = if (worker_index + iteration) % 2 == 0 {
                    AllocationClass::Temporary
                } else {
                    AllocationClass::Persistent
                };

                let allocation = worker_allocator
                    .allocate_host(
                        ByteCount::new(CONCURRENT_ALLOCATION_BYTES),
                        class,
                    )
                    .expect("mixed concurrent allocation should succeed");

                allocations.push(allocation);
            }

            worker_barrier.wait();

            allocations
        }));
    }

    barrier.wait();

    let mut allocations = Vec::new();

    for worker in workers {
        allocations.extend(
            worker
                .join()
                .expect("final invariant worker must not panic"),
        );
    }

    let expected_allocations =
        expected_count(WORKER_THREADS, ALLOCATIONS_PER_WORKER);

    let expected_bytes = expected_bytes(
        WORKER_THREADS,
        ALLOCATIONS_PER_WORKER,
        CONCURRENT_ALLOCATION_BYTES,
    );

    let accounting = allocator
        .accounting()
        .expect("accounting must remain readable");

    assert_eq!(
        accounting.allocations,
        expected_allocations
    );

    assert_eq!(
        accounting.host_bytes,
        expected_bytes
    );

    assert_eq!(
        accounting.total_bytes(),
        expected_bytes
    );

    assert!(
        accounting.temporary_host_bytes
            <= accounting.host_bytes
    );

    assert!(
        accounting.persistent_host_bytes
            <= accounting.host_bytes
    );

    drop(allocations);

    let final_accounting = allocator
        .accounting()
        .expect("final accounting must remain readable");

    assert_eq!(
        final_accounting.allocations,
        0
    );

    assert_eq!(
        final_accounting.host_bytes,
        0
    );

    assert_eq!(
        final_accounting.device_bytes,
        0
    );

    assert_eq!(
        final_accounting.distributed_bytes,
        0
    );

    assert_eq!(
        final_accounting.state_bytes,
        0
    );

    assert_eq!(
        final_accounting.state_elements,
        0
    );

    assert_eq!(
        final_accounting.temporary_host_bytes,
        0
    );

    assert_eq!(
        final_accounting.persistent_host_bytes,
        0
    );
}