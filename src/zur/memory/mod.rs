
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Memory
//!
//! Path: `src/zur/memory/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! This module is the public composition boundary for ZUR memory
//! infrastructure.
//!
//! It connects the existing ZUR-facing memory components:
//!
//! - Provider-neutral allocation.
//! - Owned allocation handles.
//! - Allocation accounting and budgets.
//! - Logical memory-region descriptors.
//! - Explicit memory reclamation.
//!
//! This file composes existing implementations. It does not create
//! another allocator, ownership system, accounting engine, region
//! hierarchy, garbage collector, or storage backend.
//!
//! # ARCHITECTURAL OWNERSHIP
//!
//! The authoritative implementation boundaries are:
//!
//! | Responsibility | Authoritative implementation |
//! |---|---|
//! | Allocation requests and allocation execution | `classical::memory::allocation` |
//! | Allocation backends and opaque storage | `classical::memory::allocation` |
//! | Allocation handles and release | `classical::memory::allocation` |
//! | Allocation accounting and budgets | `classical::memory::allocation` |
//! | ZUR allocation facade | `zur::memory::allocator` |
//! | ZUR handle facade | `zur::memory::handle` |
//! | ZUR accounting facade | `zur::memory::accounting` |
//! | Logical runtime memory-region descriptors | `zur::memory::region` |
//! | Canonical region identity | `zuir::ids` |
//! | Semantic ownership and borrowing | Frontend semantic authority and canonical ownership contracts |
//! | Semantic region and lifetime validity | Classical memory and semantic verification |
//! | Explicit reclamation coordination | `zur::memory::reclamation` |
//! | Physical storage realization | Selected allocation backend or domain provider |
//!
//! Re-exporting a type does not transfer its implementation ownership.
//!
//! # MODULE RESPONSIBILITIES
//!
//! ## allocator
//!
//! Exposes the canonical provider-neutral allocator and its contracts.
//! It does not introduce a second allocation implementation.
//!
//! ## handle
//!
//! Exposes the canonical owned allocation handle under the ZUR-facing
//! `MemoryHandle` name.
//!
//! The handle is non-Clone. Moving it transfers ownership. Dropping
//! it releases its underlying storage through the canonical allocator.
//!
//! ## accounting
//!
//! Exposes the canonical allocation-accounting snapshot, limits,
//! errors, and result types.
//!
//! It does not maintain a second byte counter or allocation counter.
//!
//! ## region
//!
//! Exposes validated logical memory-region descriptors and their
//! runtime-relevant metadata.
//!
//! A region descriptor is not an allocation, physical address,
//! hardware placement decision, or proof of resource availability.
//!
//! ## reclamation
//!
//! Coordinates reclamation only for allocation handles explicitly
//! transferred into a reclamation manager.
//!
//! It does not implement tracing garbage collection, ownership
//! inference, borrow checking, lifetime inference, or a second
//! allocation-release mechanism.
//!
//! # OWNERSHIP AND LIFETIME CONTRACT
//!
//! The canonical `MemoryAllocation` remains the sole owner of its
//! opaque storage and canonical allocation accounting.
//!
//! `MemoryHandle` is an alias for that same type.
//!
//! Ordinary Rust ownership and destruction remain the default
//! memory-management mechanism.
//!
//! Reclamation is opt-in. An allocation becomes eligible for
//! reclamation only when its owner explicitly transfers its handle
//! to `ReclamationManager::register_evictable`.
//!
//! An allocation that has not been transferred to a reclamation
//! manager cannot be reclaimed by that manager.
//!
//! Successful `unregister` returns ownership to the caller without
//! releasing the allocation.
//!
//! # SEMANTIC BOUNDARIES
//!
//! This module does not implement or redefine:
//!
//! - Source-language memory syntax.
//! - AST memory declarations.
//! - Ownership inference.
//! - Borrow checking.
//! - Lifetime inference.
//! - Semantic region hierarchy validation.
//! - Resource requirements.
//! - Resource capabilities.
//! - Resource constraints.
//! - Resource preferences.
//! - Resource hints.
//! - Resource feasibility.
//! - Resource reservation planning.
//! - Hardware discovery.
//! - Device selection.
//! - Physical placement.
//! - Quantum-state ownership.
//! - HDL storage semantics.
//! - Distributed-memory semantics.
//! - Tensor storage semantics.
//! - Scheduling or routing.
//!
//! These responsibilities remain with their existing authorities.
//!
//! In particular, a valid allocation request is not proof that a
//! physical resource is available. A registered backend is not proof
//! that it can satisfy every request. A successful allocation is
//! established only by the selected backend returning storage that
//! satisfies the canonical allocation contract.
//!
//! # DOMAIN INTEGRATION
//!
//! Classical:
//! `crate::classical::memory`
//!
//! ZUIR:
//! `crate::zuir::ids`
//! `crate::zuir::ir::ownership`
//! `crate::zuir::ir::region`
//!
//! Quantum:
//! `crate::quantum::memory`
//! `crate::quantum::ir`
//! `crate::quantum::hardware`
//!
//! HDL and hardware:
//! `crate::hdl`
//! `crate::backend`
//!
//! AI and tensors:
//! `crate::ai`
//! `crate::classical::collections::tensor`
//!
//! Hybrid:
//! `crate::hybrid`
//!
//! Distributed:
//! `crate::distributed`
//!
//! ZUR supplies common opaque allocation ownership where the
//! resource is compatible with the canonical allocation contract.
//!
//! Domain-specific resources that do not have ordinary byte-addressable
//! storage must retain their authoritative domain-specific handles,
//! ownership rules, and lifecycle contracts.
//!
//! This module does not force quantum-native state, physical qubits,
//! HDL resources, or remote resources into a host-memory abstraction.
//!
//! # GRAMMAR AND SEMANTIC INTEGRATION
//!
//! Relevant source-level authorities include:
//!
//! - `grammar/memory/memory.g4`
//! - `grammar/memory/allocation.g4`
//! - `grammar/memory/deallocation.g4`
//! - `grammar/memory/regions.g4`
//! - `grammar/memory/ownership.g4`
//! - `grammar/memory/borrowing.g4`
//! - `grammar/memory/lifetimes.g4`
//! - `grammar/memory/address-spaces.g4`
//! - `grammar/memory/shared-memory.g4`
//! - `grammar/memory/distributed-memory.g4`
//! - `grammar/memory/memory-capabilities.g4`
//! - `grammar/memory/memory-constraints.g4`
//! - `grammar/spec/resources.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! The grammar describes source-level intent.
//!
//! The frontend and semantic authority validate types, ownership,
//! borrowing, lifetimes, regions, requirements, capabilities,
//! constraints, preferences, hints, and provenance.
//!
//! Canonical IR and verification preserve validated semantics.
//!
//! Lowering and execution planning determine whether a compatible
//! allocation operation can be requested.
//!
//! The selected provider performs the actual allocation.
//!
//! This module does not parse grammar, infer semantic validity,
//! or translate source declarations directly into physical allocation.
//!
//! # SCALABILITY AND POCO-REAF
//!
//! This module introduces no arbitrary universal ceilings for:
//!
//! - Allocation sizes.
//! - Allocation counts.
//! - Reclaimable allocation counts.
//! - Memory regions.
//! - Memory spaces.
//! - Allocation backends.
//! - Providers.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Distributed participants.
//! - Computational domains.
//!
//! The canonical allocator uses its established `u128` accounting
//! representation. Reclamation uses dynamically growing metadata
//! collections and reports representational or allocation failures.
//!
//! Actual operation remains bounded by:
//!
//! - Available physical or virtual resources.
//! - Backend capabilities.
//! - Platform address-space limits.
//! - Rust collection and integer representation.
//! - Explicitly configured allocation budgets.
//! - Provider-specific constraints.
//!
//! No finite implementation promises literal infinite memory.
//!
//! # CONCURRENCY
//!
//! Thread safety and synchronization are owned by the underlying
//! allocation implementation and reclamation manager.
//!
//! This composition root introduces no global mutable state,
//! shared mutable singleton, or additional synchronization layer.
//!
//! Reclamation implementations must release allocation storage
//! outside their metadata locks.
//!
//! # SAFETY
//!
//! This module introduces:
//!
//! - No unsafe blocks.
//! - No unsafe functions.
//! - No unsafe implementations.
//! - No raw-pointer ownership API.
//! - No global mutable allocator.
//! - No duplicate allocation accounting.
//! - No implicit garbage collection.
//! - No implicit host fallback.
//! - No fabricated allocation or reclamation success.
//!
//! # INTEGRATION CONTRACT
//!
//! The parent `src/zur/mod.rs` must declare:
//!
//! ```rust
//! pub mod memory;
//! ```
//!
//! The crate root must expose the ZUR module through a valid
//! `pub mod zur;` declaration once the ZUR composition root is
//! integrated.
//!
//! The current repository's `src/zur/mod.rs` is not yet a valid
//! Rust module implementation; it currently contains the text:
//!
//!     This directory replaces src/runtime/
//!
//! That parent module must be replaced with a real ZUR composition
//! root before this module can be compiled through `crate::zur`.
//!
//! The existing `src/lib.rs` currently declares `pub mod runtime;`
//! rather than `pub mod zur;`. The runtime-to-ZUR migration must
//! reconcile that crate-root declaration and its compatibility API.
//!
//! Those parent-module changes are integration prerequisites, not
//! responsibilities of this memory composition file.
//!
//! # FREEZE CRITERIA
//!
//! This file is complete when:
//!
//! - Every declared child module exists.
//! - Every re-export resolves to its canonical implementation.
//! - No duplicate allocation or accounting implementation exists.
//! - MemoryHandle remains non-Clone.
//! - RegionId remains the canonical ZUIR identifier.
//! - Reclamation remains explicitly opt-in.
//! - Domain-specific memory semantics remain with their authorities.
//! - No unsafe code or arbitrary hardware ceilings are introduced.
//! - Public API and module-path tests pass.
//! - The parent ZUR composition root declares this module exactly once.
//!
//! Successful compilation of this file alone does not establish
//! production readiness of every allocator or memory provider.

// ============================================================================
// MEMORY MODULES
// ============================================================================

/// Canonical provider-neutral allocation facade.
///
/// Reuses `crate::classical::memory::allocation`.
pub mod allocator;

/// Canonical allocation-handle facade.
///
/// Exposes the owned `MemoryAllocation` and its ZUR-facing alias.
pub mod handle;

/// Canonical allocation-accounting facade.
///
/// Does not implement independent accounting.
pub mod accounting;

/// Runtime-facing logical memory-region descriptors.
///
/// Reuses canonical ZUIR region identities.
pub mod region;

/// Explicit reclamation coordination for transferred allocation handles.
///
/// Does not implement tracing garbage collection or reclaim ordinary
/// program-owned allocations.
pub mod reclamation;

// ============================================================================
// CANONICAL ALLOCATION API
// ============================================================================

/// Canonical allocation request.
pub use allocator::AllocationRequest;

/// Canonical allocation limits.
pub use allocator::AllocationLimits;

/// Canonical allocation accounting snapshot.
pub use allocator::AllocationAccounting;

/// Canonical allocation error.
pub use allocator::AllocationError;

/// Canonical allocation result.
pub use allocator::AllocationResult;

/// Canonical opaque storage contract.
pub use allocator::AllocationStorage;

/// Canonical allocation backend contract.
pub use allocator::AllocationBackend;

/// Canonical allocator facade.
pub use allocator::MemoryAllocator;

/// Canonical safe host allocation backend.
pub use allocator::HostAllocationBackend;

// ============================================================================
// CANONICAL OWNED HANDLE API
// ============================================================================

/// Canonical owned allocation handle implementation.
pub use handle::MemoryAllocation;

/// ZUR-facing alias for the canonical owned allocation handle.
pub use handle::MemoryHandle;

// ============================================================================
// ACCOUNTING API
// ============================================================================

/// ZUR-facing name for canonical allocation accounting.
pub use accounting::MemoryAccounting;

/// ZUR-facing name for canonical allocation limits.
pub use accounting::MemoryAccountingLimits;

/// ZUR-facing accounting operation result.
pub use accounting::MemoryAccountingResult;

// ============================================================================
// REGION API
// ============================================================================

/// Canonical ZUIR memory-region identity.
///
/// This is re-exported for convenience; identity ownership remains
/// with `crate::zuir::ids`.
pub use crate::zuir::ids::RegionId;

/// Runtime-facing logical memory-region descriptor.
pub use region::MemoryRegion;

/// Memory-region access mode.
pub use region::RegionAccess;

/// Memory-region validation error.
pub use region::RegionError;

/// Memory-region operation result.
pub use region::RegionResult;

// ============================================================================
// RECLAMATION API
// ============================================================================

/// Manager-local identity for an explicitly reclaimable allocation.
pub use reclamation::ReclamationId;

/// Operational priority used for reclamation ordering.
pub use reclamation::ReclamationPriority;

/// Reclamation coordination error.
pub use reclamation::ReclamationError;

/// Reclamation operation result.
pub use reclamation::ReclamationResult;

/// Error that preserves allocation ownership when registration fails.
pub use reclamation::RegistrationError;

/// Result type for ownership-preserving registration.
pub use reclamation::RegistrationResult;

/// Report of actual bytes and allocations released.
pub use reclamation::ReclamationReport;

/// Coordinator for explicitly reclaimable allocations.
pub use reclamation::ReclamationManager;

// ============================================================================
// COMPOSITION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Confirms that the ZUR memory root exposes the canonical allocator.
    #[test]
    fn exposes_canonical_allocator() {
        let allocator =
            MemoryAllocator::host(AllocationLimits::unbounded());

        let allocation = allocator
            .allocate(AllocationRequest::host(64))
            .expect("canonical host allocation should succeed");

        assert!(allocation.is_live());
        assert_eq!(allocation.byte_len(), 64);

        let snapshot = allocator
            .accounting()
            .expect("canonical accounting should be available");

        assert_eq!(snapshot.live_bytes, 64);
        assert_eq!(snapshot.live_allocations, 1);

        drop(allocation);

        let snapshot = allocator
            .accounting()
            .expect("canonical accounting should remain available");

        assert_eq!(snapshot.live_bytes, 0);
        assert_eq!(snapshot.live_allocations, 0);
    }

    /// Confirms that the public handle name is the canonical allocation.
    #[test]
    fn memory_handle_is_the_canonical_owned_allocation() {
        let allocator =
            MemoryAllocator::host(AllocationLimits::unbounded());

        let handle: MemoryHandle = allocator
            .allocate(AllocationRequest::host(32))
            .expect("allocation should succeed");

        assert!(handle.is_live());
        assert_eq!(handle.byte_len(), 32);

        handle.release();

        let snapshot = allocator
            .accounting()
            .expect("accounting should be available");

        assert_eq!(snapshot.live_bytes, 0);
        assert_eq!(snapshot.live_allocations, 0);
    }

    /// Confirms that region identity is the canonical ZUIR identity.
    #[test]
    fn region_identity_is_canonical_zuir_identity() {
        let id = RegionId::new(1).expect("nonzero region identity");

        let region = MemoryRegion::new(id, "memory::runtime")
            .expect("valid logical memory region");

        let _: RegionId = region.id();

        assert_eq!(region.id(), id);
    }

    /// Confirms that reclamation is exposed without becoming mandatory.
    #[test]
    fn reclamation_is_explicit_and_opt_in() {
        let allocator =
            MemoryAllocator::host(AllocationLimits::unbounded());

        let manager = ReclamationManager::new();

        let ordinary = allocator
            .allocate(AllocationRequest::host(48))
            .expect("allocation should succeed");

        let report = manager
            .reclaim(48)
            .expect("empty-manager reclamation should be reported");

        assert!(!report.satisfied());
        assert_eq!(report.reclaimed_bytes, 0);
        assert!(ordinary.is_live());

        drop(ordinary);
    }

    /// Confirms that the root exports the canonical accounting aliases.
    #[test]
    fn accounting_aliases_preserve_canonical_types() {
        let _: Option<MemoryAccounting> = None;
        let _: Option<MemoryAccountingLimits> = None;
        let _: Option<MemoryAccountingResult<()>> = None;
    }

    /// Confirms that a registered allocation is reclaimed through
    /// its canonical handle and allocator accounting.
    #[test]
    fn explicitly_registered_allocation_uses_canonical_release() {
        let allocator =
            MemoryAllocator::host(AllocationLimits::unbounded());

        let manager = ReclamationManager::new();

        let allocation = allocator
            .allocate(AllocationRequest::host(96))
            .expect("allocation should succeed");

        let id = manager
            .register_evictable(
                allocation,
                ReclamationPriority::default(),
            )
            .expect("explicit registration should succeed");

        assert_eq!(manager.registered_allocations().unwrap(), 1);

        let report = manager
            .reclaim(96)
            .expect("reclamation should complete");

        assert!(report.satisfied());
        assert_eq!(report.reclaimed_bytes, 96);
        assert_eq!(report.reclaimed_allocations, 1);

        assert!(matches!(
            manager.unregister(id),
            Err(ReclamationError::UnknownAllocation { .. })
        ));

        let snapshot = allocator
            .accounting()
            .expect("canonical accounting should be available");

        assert_eq!(snapshot.live_bytes, 0);
        assert_eq!(snapshot.live_allocations, 0);
    }
}
