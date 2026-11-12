
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Memory Handles
//!
//! Path: `src/zur/memory/handle.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Provides the ZUR-facing API for owned memory-allocation handles.
//!
//! This module reuses the canonical allocation-handle implementation
//! defined in:
//!
//!     src/classical/memory/allocation.rs
//!
//! It does not create another allocation implementation.
//!
//! # ARCHITECTURAL OWNERSHIP
//!
//! The canonical classical allocation subsystem owns:
//!
//! - MemoryAllocation.
//! - AllocationStorage.
//! - AllocationBackend.
//! - MemoryAllocator.
//! - AllocationRequest.
//! - AllocationAccounting.
//! - AllocationLimits.
//! - AllocationError.
//!
//! This module exposes the existing MemoryAllocation through the
//! ZUR memory namespace.
//!
//! MemoryHandle is a type alias, not a new wrapper or independently
//! managed resource.
//!
//! Consequently, both names have exactly the same ownership,
//! lifetime, release, and storage semantics.
//!
//! # OWNERSHIP CONTRACT
//!
//! A memory handle:
//!
//! - owns one allocation;
//! - cannot be cloned;
//! - does not expose raw pointers;
//! - keeps its allocation alive while it owns the storage;
//! - supports borrowing the opaque storage interface;
//! - supports explicit release;
//! - releases its storage when dropped;
//! - participates in the canonical allocator's accounting.
//!
//! Ownership transfer uses ordinary Rust move semantics.
//!
//! No additional reference-counting layer is introduced.
//!
//! # RESOURCE AND PROVIDER CONTRACT
//!
//! A handle represents successfully acquired storage.
//!
//! It does not represent:
//!
//! - a resource requirement;
//! - a capability;
//! - a resource preference;
//! - a resource constraint;
//! - a feasibility result;
//! - a resource reservation proposal;
//! - a physical address;
//! - a hardware-selection decision.
//!
//! Those concepts remain with their authoritative subsystems.
//!
//! A handle is created only after the explicitly selected allocation
//! backend has successfully satisfied an AllocationRequest.
//!
//! This module never silently substitutes host storage for another
//! requested memory space.
//!
//! # DOMAIN INTEGRATION
//!
//! Classical:
//!
//!     src/classical/memory/allocation.rs
//!     src/classical/memory/ownership.rs
//!     src/classical/memory/address_space.rs
//!     src/classical/memory/regions.rs
//!
//! ZUIR:
//!
//!     src/zuir/ids.rs
//!     src/zuir/ir/ownership.rs
//!     src/zuir/ir/region.rs
//!
//! Quantum:
//!
//!     src/quantum/memory/
//!     src/quantum/ir/
//!     src/quantum/hardware/
//!
//! Hybrid:
//!
//!     src/hybrid/
//!
//! Distributed:
//!
//!     src/distributed/
//!
//! The handle does not redefine any of these subsystems.
//!
//! In particular, quantum-native resources that are not ordinary
//! byte-addressable allocations must retain their domain-specific
//! ownership and lifecycle contracts.
//!
//! # GRAMMAR INTEGRATION
//!
//! Relevant grammar authorities:
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
//!
//! The grammar describes source-level intent.
//!
//! Semantic analysis and verification establish ownership,
//! borrowing, lifetime, resource and capability validity.
//!
//! Lowering and execution infrastructure subsequently construct
//! appropriate allocation requests.
//!
//! This module does not parse source code or interpret grammar rules.
//!
//! # SCALABILITY
//!
//! No additional fixed limits are introduced for:
//!
//! - allocation size;
//! - allocation count;
//! - memory spaces;
//! - memory regions;
//! - providers;
//! - devices;
//! - processors;
//! - distributed participants;
//! - computational domains.
//!
//! The underlying canonical allocation implementation uses u128
//! for allocation accounting.
//!
//! Actual allocations remain subject to:
//!
//! - available resources;
//! - provider capabilities;
//! - platform address-space constraints;
//! - Rust representation constraints;
//! - explicitly configured allocation budgets.
//!
//! No finite implementation promises literal infinite physical memory.
//!
//! # SAFETY
//!
//! This module introduces:
//!
//! - no unsafe blocks;
//! - no unsafe functions;
//! - no unsafe implementations;
//! - no raw-pointer ownership API;
//! - no global mutable state;
//! - no independent allocation accounting;
//! - no independent memory reclamation.
//!
//! # FREEZE CONTRACT
//!
//! This file is a stable public composition boundary.
//!
//! Future ZUR memory modules must use this handle or the canonical
//! allocation implementation.
//!
//! They must not introduce another allocation handle with competing
//! ownership or release behavior.


// =============================================================================
// CANONICAL MEMORY HANDLE
// =============================================================================

/// Canonical owned memory-allocation handle.
///
/// This is the original implementation from
/// `crate::classical::memory::allocation`.
///
/// The handle owns its opaque allocation storage.
///
/// It is deliberately non-Clone. Ownership transfers through Rust
/// move semantics.
///
/// Dropping the handle releases the allocation and updates the
/// canonical allocator's shared accounting.
///
/// Explicit early release is available through [`MemoryAllocation::release`].
///
/// The handle does not expose raw pointers or assume that every
/// memory space is host-addressable.
pub use crate::classical::memory::allocation::MemoryAllocation;


// =============================================================================
// ZUR PUBLIC HANDLE NAME
// =============================================================================

/// ZUR-facing name for the canonical owned memory-allocation handle.
///
/// This is a type alias, not a second allocation implementation.
///
/// It preserves all properties of [`MemoryAllocation`]:
///
/// - exclusive ownership of the allocation handle;
/// - non-Clone semantics;
/// - opaque storage;
/// - automatic release through `Drop`;
/// - explicit consuming release;
/// - canonical allocation accounting;
/// - provider-neutral storage ownership.
///
/// # Ownership
///
/// A `MemoryHandle` cannot be duplicated using `Clone`.
///
/// Passing it by value transfers ownership. Borrowing it permits
/// access to the opaque allocation-storage interface while the
/// handle remains alive.
///
/// # Release
///
/// Use [`MemoryHandle::release`] when immediate deterministic
/// release is required.
///
/// Otherwise, ordinary Rust scope and ownership rules determine
/// when the allocation is released.
///
/// # Integration
///
/// Allocations are obtained through:
///
/// `crate::zur::memory::allocator::MemoryAllocator`.
///
/// This alias does not provide another allocator or backend.
pub type MemoryHandle = MemoryAllocation;


// =============================================================================
// CONTRACT TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::memory::allocator::{
        AllocationLimits,
        AllocationRequest,
        MemoryAllocator,
    };

    /// Confirms that ZUR uses the canonical allocation handle.
    #[test]
    fn zur_handle_uses_canonical_allocation_implementation() {
        let allocator =
            MemoryAllocator::host(AllocationLimits::unbounded());

        let handle: MemoryHandle = allocator
            .allocate(AllocationRequest::host(64))
            .expect("allocation should succeed");

        assert!(handle.is_live());
        assert_eq!(handle.byte_len(), 64);
        assert_eq!(handle.space(), Some("host"));

        let accounting = allocator
            .accounting()
            .expect("accounting should be available");

        assert_eq!(accounting.live_bytes, 64);
        assert_eq!(accounting.live_allocations, 1);

        handle.release();

        let accounting = allocator
            .accounting()
            .expect("accounting should remain available");

        assert_eq!(accounting.live_bytes, 0);
        assert_eq!(accounting.live_allocations, 0);
    }

    /// Confirms that dropping a ZUR handle releases its allocation.
    #[test]
    fn dropping_handle_releases_canonical_storage() {
        let allocator =
            MemoryAllocator::host(AllocationLimits::unbounded());

        {
            let handle = allocator
                .allocate(AllocationRequest::host(128))
                .expect("allocation should succeed");

            assert!(handle.is_live());

            let accounting = allocator
                .accounting()
                .expect("accounting should be available");

            assert_eq!(accounting.live_bytes, 128);
            assert_eq!(accounting.live_allocations, 1);
        }

        let accounting = allocator
            .accounting()
            .expect("accounting should remain available");

        assert_eq!(accounting.live_bytes, 0);
        assert_eq!(accounting.live_allocations, 0);
    }

    /// Confirms that an explicitly configured budget is preserved.
    #[test]
    fn handle_respects_canonical_allocation_budget() {
        let limits = AllocationLimits::unbounded()
            .with_max_live_bytes(32);

        let allocator = MemoryAllocator::host(limits);

        let result = allocator.allocate(
            AllocationRequest::host(64),
        );

        assert!(matches!(
            result,
            Err(
                crate::zur::memory::allocator::AllocationError::BudgetExceeded {
                    resource: "live bytes",
                    ..
                }
            )
        ));

        let accounting = allocator
            .accounting()
            .expect("accounting should be available");

        assert_eq!(accounting.live_bytes, 0);
        assert_eq!(accounting.live_allocations, 0);
    }

    /// Confirms that the public ZUR name and canonical name
    /// represent the same underlying type.
    #[test]
    fn zur_handle_and_canonical_handle_are_identical_types() {
        fn accepts_zur_handle(_: MemoryHandle) {}

        let allocator =
            MemoryAllocator::host(AllocationLimits::unbounded());

        let canonical: MemoryAllocation = allocator
            .allocate(AllocationRequest::host(16))
            .expect("allocation should succeed");

        accepts_zur_handle(canonical);
    }

    /// Confirms that a zero-byte allocation is not rejected by
    /// the ZUR handle facade when the canonical backend accepts it.
    #[test]
    fn zero_byte_handle_preserves_canonical_behavior() {
        let allocator =
            MemoryAllocator::host(AllocationLimits::unbounded());

        let handle = allocator
            .allocate(AllocationRequest::host(0))
            .expect("zero-byte allocation should succeed");

        assert!(handle.is_live());
        assert_eq!(handle.byte_len(), 0);
        assert_eq!(handle.space(), Some("host"));

        handle.release();

        let accounting = allocator
            .accounting()
            .expect("accounting should be available");

        assert_eq!(accounting.live_allocations, 0);
    }
}
