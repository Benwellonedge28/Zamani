#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Memory Allocator
//!
//! Path: `src/zur/memory/allocator.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Exposes the canonical provider-neutral memory-allocation
//! implementation through the Zamani Universal Runtime (ZUR).
//!
//! This module deliberately reuses the existing implementation in:
//!
//!     src/classical/memory/allocation.rs
//!
//! It does not introduce a second allocator, accounting system,
//! allocation-handle implementation, backend contract, or host
//! allocation implementation.
//!
//! # ARCHITECTURAL OWNERSHIP
//!
//! The canonical allocation implementation currently owns:
//!
//! - AllocationRequest.
//! - AllocationLimits.
//! - AllocationAccounting.
//! - AllocationError.
//! - AllocationResult.
//! - AllocationStorage.
//! - AllocationBackend.
//! - MemoryAllocator.
//! - MemoryAllocation.
//! - HostAllocationBackend.
//!
//! This module exposes those existing types through the ZUR API.
//!
//! The re-exported types retain their original implementation and
//! ownership. They are not copied, wrapped, or independently
//! reimplemented here.
//!
//! # RESPONSIBILITY BOUNDARY
//!
//! This module provides the ZUR-facing allocation API.
//!
//! It does NOT own:
//!
//! - Source-language memory syntax.
//! - Memory AST nodes.
//! - Ownership semantics.
//! - Borrowing semantics.
//! - Lifetime inference.
//! - Semantic memory regions.
//! - Canonical ZUIR region identities.
//! - Classical tensor storage.
//! - Quantum state representations.
//! - Quantum memory semantics.
//! - HDL or hardware resource semantics.
//! - Device discovery.
//! - Hardware selection.
//! - Backend selection.
//! - Physical placement.
//! - Resource feasibility.
//! - Resource reservation planning.
//! - Scheduling.
//! - Routing.
//! - Garbage collection.
//! - A universal hardware abstraction layer.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! # ALLOCATION MODEL
//!
//! Allocation is performed through an explicitly supplied backend.
//!
//! An allocation request describes:
//!
//! - requested byte quantity;
//! - resolved memory-space identity;
//! - allocation-class identity;
//! - optional diagnostic label.
//!
//! Memory-space and allocation-class identifiers are open-world
//! identifiers. They are not closed hardware enumerations.
//!
//! A backend may implement host, pinned-host, device, unified,
//! shared, distributed, remote, persistent, or future storage.
//!
//! The allocator does not assume that every allocation is
//! host-addressable.
//!
//! The allocator does not silently substitute one memory space
//! for another.
//!
//! # RESOURCE ACCOUNTING
//!
//! The canonical implementation supports optional operational
//! budgets for:
//!
//! - aggregate live allocation bytes;
//! - simultaneously live allocation handles.
//!
//! An absent budget does not impose an allocator-level limit.
//!
//! It does not imply that physical resources are unlimited.
//!
//! Accounting uses checked arithmetic and reports overflow.
//!
//! Backend allocation occurs only after accounting reservation.
//!
//! Backend failure and storage-contract mismatch cause the
//! accounting reservation to be rolled back.
//!
//! # OWNERSHIP AND LIFETIME
//!
//! MemoryAllocation is an owned, non-Clone allocation handle.
//!
//! The handle owns its opaque AllocationStorage.
//!
//! Dropping the handle releases its storage and updates the
//! allocator's shared accounting.
//!
//! Explicit release is available when deterministic early
//! release is required.
//!
//! The generic API does not expose raw pointers.
//!
//! # PROVIDER INTEGRATION
//!
//! AllocationBackend is the provider-neutral allocation boundary.
//!
//! A backend implementation must:
//!
//! - provide a stable backend identity;
//! - declare the memory spaces it supports;
//! - allocate storage for a validated request;
//! - return storage whose byte count matches the request;
//! - return storage whose resolved space matches the request;
//! - report failures instead of fabricating successful allocations.
//!
//! Backend support is not proof of current capacity or availability.
//!
//! A successful allocation is evidence that the backend returned
//! storage satisfying the allocation contract.
//!
//! # DOMAIN INTEGRATION
//!
//! Classical:
//!
//!     src/classical/memory/allocation.rs
//!     src/classical/memory/address_space.rs
//!     src/classical/collections/tensor.rs
//!
//! Quantum:
//!
//!     src/quantum/memory/
//!     src/quantum/ir/
//!     src/quantum/hardware/
//!
//! HDL and hardware:
//!
//!     src/hdl/
//!     src/backend/
//!     src/zuir/
//!
//! Hybrid:
//!
//!     src/hybrid/
//!
//! Distributed:
//!
//!     src/distributed/
//!
//! The allocator supplies opaque storage ownership.
//!
//! It does not reinterpret the semantic meaning of those domains.
//! Domain-specific providers must implement the appropriate
//! allocation contract.
//!
//! In particular, a quantum-native state handle is not necessarily
//! a byte-addressable host allocation. Such resources must retain
//! their domain-specific contracts or use an explicitly compatible
//! AllocationBackend implementation.
//!
//! # GRAMMAR INTEGRATION
//!
//! This module implements no grammar rules and performs no parsing.
//!
//! Relevant source-level authorities include:
//!
//! - grammar/memory/allocation.g4
//! - grammar/memory/deallocation.g4
//! - grammar/memory/memory.g4
//! - grammar/memory/regions.g4
//! - grammar/memory/address-spaces.g4
//! - grammar/memory/shared-memory.g4
//! - grammar/memory/distributed-memory.g4
//! - grammar/memory/memory-capabilities.g4
//! - grammar/memory/memory-constraints.g4
//! - grammar/spec/resources.md
//!
//! These grammars describe source-level intent.
//!
//! The frontend and semantic layer validate ownership, borrowing,
//! lifetime, requirements, constraints, capabilities, preferences,
//! hints, and resource meaning before allocation is requested.
//!
//! Source syntax never directly invokes this allocator.
//!
//! # SEMANTIC INVARIANTS
//!
//! The following concepts remain distinct:
//!
//! Requirement
//! Capability
//! Constraint
//! Preference
//! Hint
//! Feasibility
//! Resource reservation
//! Actual allocation
//! Actual execution
//!
//! This module implements allocation only.
//!
//! It does not decide whether a source-level requirement is valid,
//! whether a target is feasible, or which provider should be selected.
//!
//! # SCALABILITY
//!
//! This module introduces no arbitrary universal ceilings for:
//!
//! - allocation bytes;
//! - allocation count;
//! - memory spaces;
//! - allocation classes;
//! - providers;
//! - devices;
//! - CPUs;
//! - GPUs;
//! - QPUs;
//! - FPGAs;
//! - distributed participants;
//! - computational domains.
//!
//! Quantities use the canonical implementation's u128 accounting
//! representation. A particular host backend must still convert
//! requests into its platform's representable allocation size.
//!
//! Actual operation is bounded by available resources, provider
//! capabilities, platform address-space limits, configured budgets,
//! and Rust's representational constraints.
//!
//! Literal infinite physical memory or computation is not promised.
//!
//! # SAFETY
//!
//! This module introduces:
//!
//! - no unsafe blocks;
//! - no unsafe functions;
//! - no unsafe implementations;
//! - no raw-pointer API;
//! - no global mutable allocator;
//! - no implicit provider selection;
//! - no fabricated allocation success;
//! - no hidden host fallback.
//!
//! # FREEZE RULE
//!
//! This module is a public composition boundary.
//!
//! Future ZUR memory modules must consume these canonical types.
//!
//! They must not create another MemoryAllocator, AllocationStorage,
//! AllocationBackend, MemoryAllocation, or allocation-accounting
//! implementation.
//!
//! If the common implementation is later moved from
//! classical::memory into zur::memory, that must be a deliberate
//! ownership migration with compatibility adapters. It must not
//! create two active allocation implementations.

// =============================================================================
// CANONICAL ALLOCATION API
// =============================================================================

/// Canonical allocation accounting snapshot.
pub use crate::classical::memory::allocation::AllocationAccounting;

/// Canonical allocation backend contract.
pub use crate::classical::memory::allocation::AllocationBackend;

/// Canonical allocation error type.
pub use crate::classical::memory::allocation::AllocationError;

/// Canonical optional allocation-budget policy.
pub use crate::classical::memory::allocation::AllocationLimits;

/// Canonical allocation request.
pub use crate::classical::memory::allocation::AllocationRequest;

/// Canonical allocation result type.
pub use crate::classical::memory::allocation::AllocationResult;

/// Canonical opaque allocation-storage contract.
pub use crate::classical::memory::allocation::AllocationStorage;

/// Canonical owned allocation handle.
pub use crate::classical::memory::allocation::MemoryAllocation;

/// Canonical allocator facade.
pub use crate::classical::memory::allocation::MemoryAllocator;

/// Safe standard-library host allocation backend.
pub use crate::classical::memory::allocation::HostAllocationBackend;

// =============================================================================
// INTEGRATION TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Confirms that ZUR and classical memory use the same allocator API.
    #[test]
    fn zur_uses_the_canonical_classical_allocation_implementation() {
        let allocator =
            MemoryAllocator::host(AllocationLimits::unbounded());

        let allocation = allocator
            .allocate(AllocationRequest::host(64))
            .expect("host allocation should succeed");

        assert!(allocation.is_live());
        assert_eq!(allocation.byte_len(), 64);
        assert_eq!(allocation.space(), Some("host"));

        let accounting = allocator
            .accounting()
            .expect("allocation accounting should be available");

        assert_eq!(accounting.live_bytes, 64);
        assert_eq!(accounting.live_allocations, 1);

        drop(allocation);

        let accounting = allocator
            .accounting()
            .expect("allocation accounting should be available");

        assert_eq!(accounting.live_bytes, 0);
        assert_eq!(accounting.live_allocations, 0);
    }

    /// Confirms that explicit budgets remain enforced through ZUR.
    #[test]
    fn zur_preserves_explicit_allocation_budgets() {
        let limits =
            AllocationLimits::unbounded().with_max_live_bytes(32);

        let allocator = MemoryAllocator::host(limits);

        let result = allocator.allocate(AllocationRequest::host(64));

        assert!(matches!(
            result,
            Err(AllocationError::BudgetExceeded { .. })
        ));

        let accounting = allocator
            .accounting()
            .expect("allocation accounting should be available");

        assert_eq!(accounting.live_bytes, 0);
        assert_eq!(accounting.live_allocations, 0);
    }

    /// Confirms that allocation handles are released through their
    /// canonical owner rather than a second ZUR accounting system.
    #[test]
    fn allocation_release_updates_shared_accounting() {
        let allocator =
            MemoryAllocator::host(AllocationLimits::unbounded());

        let allocation = allocator
            .allocate(AllocationRequest::host(128))
            .expect("host allocation should succeed");

        allocation.release();

        let accounting = allocator
            .accounting()
            .expect("allocation accounting should be available");

        assert_eq!(accounting.live_bytes, 0);
        assert_eq!(accounting.live_allocations, 0);
    }
}