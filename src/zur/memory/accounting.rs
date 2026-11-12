
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Memory Accounting
//!
//! Path: `src/zur/memory/accounting.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Expose the canonical memory-allocation accounting model through
//! the Zamani Universal Runtime (ZUR).
//!
//! This module deliberately reuses the accounting implementation
//! owned by:
//!
//! `crate::classical::memory::allocation`
//!
//! It does not implement another accounting engine.
//!
//! # Architectural ownership
//!
//! The canonical classical allocation subsystem owns:
//!
//! - `AllocationAccounting`;
//! - `AllocationLimits`;
//! - allocation accounting synchronization;
//! - checked accounting arithmetic;
//! - allocation-budget enforcement;
//! - allocation reservation;
//! - rollback after failed allocation;
//! - accounting release after deallocation.
//!
//! ZUR exposes these types without copying their implementation.
//!
//! # Responsibilities
//!
//! This module:
//!
//! - exposes the canonical accounting snapshot;
//! - exposes the canonical allocation-budget policy;
//! - exposes the canonical allocation result and error types;
//! - provides ZUR-facing type aliases;
//! - documents the accounting integration boundary.
//!
//! # Does not own
//!
//! This module does not own:
//!
//! - allocation algorithms;
//! - physical memory capacity discovery;
//! - memory-provider discovery;
//! - provider selection;
//! - resource feasibility;
//! - resource reservation planning;
//! - physical placement;
//! - memory-region hierarchy;
//! - ownership inference;
//! - borrowing or lifetime analysis;
//! - garbage collection;
//! - allocation handles;
//! - device-specific accounting;
//! - quantum-state accounting;
//! - distributed resource coordination;
//! - hardware scheduling;
//! - execution results.
//!
//! These responsibilities remain with their authoritative
//! repository subsystems.
//!
//! # Accounting semantics
//!
//! The canonical `AllocationAccounting` represents:
//!
//! - current live allocation bytes;
//! - current live allocation handles;
//! - peak live allocation bytes;
//! - peak live allocation handles.
//!
//! It is an accounting snapshot, not a prediction of available
//! physical memory.
//!
//! `AllocationLimits` represents optional operational budgets.
//! An absent limit means that this allocator policy does not impose
//! that particular budget. It does not mean physical resources are
//! unlimited.
//!
//! # Important distinctions
//!
//! The following concepts must remain separate:
//!
//! - Requirement;
//! - Capability;
//! - Constraint;
//! - Preference;
//! - Hint;
//! - Feasibility;
//! - Resource reservation;
//! - Actual allocation;
//! - Accounting observation;
//! - Actual execution.
//!
//! A feasibility result does not reserve memory.
//! A reservation does not prove that allocation succeeded.
//! An accounting snapshot does not prove that physical capacity
//! remains available.
//!
//! # Ownership and release
//!
//! The canonical allocator reserves accounting before invoking its
//! selected backend.
//!
//! If allocation fails, the canonical allocator rolls back the
//! accounting reservation.
//!
//! A successful `MemoryAllocation` owns its storage. Its explicit
//! release operation or destructor updates the same accounting
//! state maintained by the allocator.
//!
//! This module must not add another counter or release mechanism.
//!
//! # Scalability
//!
//! This module adds no fixed limits on:
//!
//! - allocation sizes;
//! - allocation counts;
//! - memory regions;
//! - memory spaces;
//! - providers;
//! - devices;
//! - processors;
//! - distributed participants;
//! - computational domains.
//!
//! The canonical accounting representation uses `u128`.
//! Actual operations remain subject to representational limits,
//! available memory, provider capabilities, platform constraints,
//! and explicitly configured budgets.
//!
//! Literal infinite physical memory is not promised.
//!
//! # Grammar and semantic integration
//!
//! Source-level intent is defined by:
//!
//! - `grammar/memory/allocation.g4`;
//! - `grammar/memory/deallocation.g4`;
//! - `grammar/memory/memory.g4`;
//! - `grammar/memory/regions.g4`;
//! - `grammar/memory/address-spaces.g4`;
//! - `grammar/memory/memory-capabilities.g4`;
//! - `grammar/memory/memory-constraints.g4`;
//! - `grammar/spec/resources.md`;
//! - `grammar/specification/semantic-model.md`.
//!
//! The frontend and semantic layer validate source-level meaning.
//! Lowering and execution planning determine whether a suitable
//! allocation request can be made.
//! The selected allocation backend performs the actual allocation.
//! The canonical allocator accounts for that allocation.
//!
//! This module does not parse grammar or interpret source syntax.
//!
//! # Domain integration
//!
//! Classical:
//! `crate::classical::memory::allocation`
//!
//! Quantum:
//! `crate::quantum::memory`
//!
//! HDL and hardware:
//! `crate::backend` and the relevant hardware providers
//!
//! Hybrid:
//! `crate::hybrid`
//!
//! Distributed:
//! `crate::distributed`
//!
//! Unified representation:
//! `crate::zuir`
//!
//! Domain-specific resources that are not ordinary byte-addressable
//! allocations must retain their canonical domain-specific
//! accounting and ownership contracts.
//!
//! # Freeze contract
//!
//! Future ZUR memory modules must use these canonical accounting
//! types and the allocator's accounting operations.
//!
//! They must not introduce another live-byte counter, allocation
//! counter, budget-enforcement engine, or release-accounting path.

/// Canonical allocation-accounting snapshot.
///
/// This is the original type defined by
/// `crate::classical::memory::allocation`.
///
/// It reports current and peak allocation accounting maintained
/// by the canonical allocator.
///
/// It is not a physical-memory-capacity report.
pub use crate::classical::memory::allocation::AllocationAccounting;

/// Canonical allocation-accounting error type.
///
/// Errors are produced by the canonical allocation implementation.
pub use crate::classical::memory::allocation::AllocationError;

/// Canonical allocation operation result type.
pub use crate::classical::memory::allocation::AllocationResult;

/// Canonical optional allocation-budget policy.
///
/// An absent budget does not impose an allocator-level limit.
/// Physical and provider constraints still apply.
pub use crate::classical::memory::allocation::AllocationLimits;

/// Canonical allocator facade.
///
/// All clones of an allocator share its canonical accounting state.
pub use crate::classical::memory::allocation::MemoryAllocator;

/// ZUR-facing name for the canonical allocation-accounting snapshot.
///
/// This is a type alias, not a second accounting structure.
pub type MemoryAccounting = AllocationAccounting;

/// ZUR-facing name for the canonical allocation-budget policy.
///
/// This is a type alias, not a second budget implementation.
pub type MemoryAccountingLimits = AllocationLimits;

/// ZUR-facing result type for accounting-related operations.
///
/// This preserves the canonical allocation error type.
pub type MemoryAccountingResult<T> = AllocationResult<T>;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::classical::memory::allocation::AllocationRequest;

    /// Confirms that ZUR exposes the canonical accounting implementation.
    #[test]
    fn zur_accounting_uses_canonical_allocation_accounting() {
        let allocator =
            MemoryAllocator::host(MemoryAccountingLimits::unbounded());

        let allocation = allocator
            .allocate(AllocationRequest::host(64))
            .expect("host allocation should succeed");

        let snapshot: MemoryAccounting = allocator
            .accounting()
            .expect("accounting snapshot should be available");

        assert_eq!(snapshot.live_bytes, 64);
        assert_eq!(snapshot.live_allocations, 1);
        assert_eq!(snapshot.peak_live_bytes, 64);
        assert_eq!(snapshot.peak_live_allocations, 1);

        drop(allocation);

        let snapshot = allocator
            .accounting()
            .expect("accounting snapshot should remain available");

        assert_eq!(snapshot.live_bytes, 0);
        assert_eq!(snapshot.live_allocations, 0);

        // Peak values describe historical high-water marks.
        assert_eq!(snapshot.peak_live_bytes, 64);
        assert_eq!(snapshot.peak_live_allocations, 1);
    }

    /// Confirms that allocator clones observe the same accounting state.
    #[test]
    fn allocator_clones_share_canonical_accounting() {
        let allocator =
            MemoryAllocator::host(MemoryAccountingLimits::unbounded());

        let observer = allocator.clone();

        let allocation = allocator
            .allocate(AllocationRequest::host(128))
            .expect("host allocation should succeed");

        let snapshot = observer
            .accounting()
            .expect("shared accounting should be available");

        assert_eq!(snapshot.live_bytes, 128);
        assert_eq!(snapshot.live_allocations, 1);

        drop(allocation);

        let snapshot = observer
            .accounting()
            .expect("shared accounting should remain available");

        assert_eq!(snapshot.live_bytes, 0);
        assert_eq!(snapshot.live_allocations, 0);
    }

    /// Confirms that ZUR preserves explicit canonical allocation budgets.
    #[test]
    fn accounting_limits_are_the_canonical_allocator_limits() {
        let limits =
            MemoryAccountingLimits::unbounded().with_max_live_bytes(32);

        let allocator = MemoryAllocator::host(limits);

        let result = allocator.allocate(AllocationRequest::host(64));

        assert!(matches!(
            result,
            Err(AllocationError::BudgetExceeded {
                resource: "live bytes",
                ..
            })
        ));

        let snapshot = allocator
            .accounting()
            .expect("accounting should remain available");

        assert_eq!(snapshot.live_bytes, 0);
        assert_eq!(snapshot.live_allocations, 0);
    }

    /// Confirms that explicit handle release updates the same accounting.
    #[test]
    fn explicit_release_updates_canonical_accounting() {
        let allocator =
            MemoryAllocator::host(MemoryAccountingLimits::unbounded());

        let allocation = allocator
            .allocate(AllocationRequest::host(256))
            .expect("host allocation should succeed");

        allocation.release();

        let snapshot = allocator
            .accounting()
            .expect("accounting should remain available");

        assert_eq!(snapshot.live_bytes, 0);
        assert_eq!(snapshot.live_allocations, 0);
        assert_eq!(snapshot.peak_live_bytes, 256);
        assert_eq!(snapshot.peak_live_allocations, 1);
    }
}
