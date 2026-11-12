
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Resources
//!
//! Path: `src/zur/resources/mod.rs`
//!
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! This module is the public composition root for ZUR resource
//! management.
//!
//! It exposes the existing resource-lease implementation and
//! establishes the architectural boundary between:
//!
//! - Semantic resource requirements.
//! - Resource capabilities.
//! - Resource feasibility.
//! - Physical resource availability.
//! - Resource acquisition and reservation.
//! - Resource lease lifecycle.
//! - Resource release.
//! - Execution-session accounting.
//!
//! The module deliberately reuses the existing implementations
//! rather than introducing competing resource-management systems.
//!
//! # ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - Resource subsystem composition.
//! - Stable public resource module paths.
//! - Public resource-lease API re-exports.
//! - Documentation of resource integration boundaries.
//!
//! This module does NOT own:
//!
//! - Source-language resource syntax.
//! - Resource AST definitions.
//! - Semantic resource requirements.
//! - Semantic resource constraints.
//! - Resource capabilities.
//! - Resource preferences.
//! - Resource hints.
//! - Resource feasibility.
//! - Compiler resource planning.
//! - Backend selection.
//! - Hardware discovery.
//! - Physical resource inventory.
//! - Provider registration.
//! - Domain-specific resource representations.
//! - Physical resource allocation algorithms.
//! - Classical memory allocation.
//! - Quantum memory semantics.
//! - HDL resource semantics.
//! - Distributed resource membership.
//! - Compiler scheduling.
//! - Quantum scheduling.
//! - Hybrid planning.
//!
//! These responsibilities remain with their respective
//! authoritative repository subsystems.
//!
//! # CANONICAL IMPLEMENTATION
//!
//! ## lease
//!
//! `lease.rs` owns the runtime resource-lease lifecycle.
//!
//! It provides:
//!
//! - ResourceLeaseAuthority.
//! - ResourceLeaseManager.
//! - ResourceLease.
//! - ResourceLeaseId integration.
//! - LeaseOwner.
//! - LeaseGrant.
//! - LeaseStatus.
//! - Explicit acquisition.
//! - Explicit renewal.
//! - Explicit release.
//! - Expiration tracking.
//! - Uncertain external-operation states.
//! - Shared lease handles.
//! - Execution-session references.
//!
//! ResourceLeaseAuthority is the authority responsible for
//! performing and confirming actual resource acquisition.
//!
//! ResourceLeaseManager coordinates lease identities and
//! delegates physical reservation to that authority.
//!
//! ResourceLease represents an acquired reservation.
//!
//! This composition root does not implement another manager,
//! allocator, provider registry, or reservation engine.
//!
//! # SEMANTIC INTEGRATION
//!
//! Canonical source-language resource semantics:
//!
//! `crate::frontend::semantic::resources`
//!
//! Canonical source-language capability semantics:
//!
//! `crate::frontend::semantic::capabilities`
//!
//! Resource grammar:
//!
//! `grammar/resources/resources.g4`
//! `grammar/resources/resource.g4`
//! `grammar/resources/resource-expressions.g4`
//! `grammar/policies/resource.g4`
//!
//! The grammar describes resource intent.
//!
//! The frontend resolves and validates that intent.
//!
//! The compiler evaluates resource feasibility against
//! appropriate target and capability evidence.
//!
//! ZUR performs operational resource acquisition only
//! when an authorized execution path requires it.
//!
//! # COMPILER INTEGRATION
//!
//! Canonical compiler resource planning:
//!
//! `crate::compiler::planning::resource_planning`
//!
//! Canonical compiler feasibility:
//!
//! `crate::compiler::planning::feasibility`
//!
//! Compiler planning results are not resource reservations.
//!
//! Runtime acquisition must obtain fresh confirmation from
//! the appropriate resource authority.
//!
//! A stale feasibility result must never be treated as
//! proof of current resource availability.
//!
//! # EXECUTION INTEGRATION
//!
//! Canonical runtime identities:
//!
//! `crate::zur::ids::ResourceLeaseId`
//! `crate::zur::ids::IdAllocator`
//!
//! Canonical execution-session integration:
//!
//! `crate::zur::execution::session::ResourceAccountingReference`
//!
//! A resource lease may provide an execution-session
//! accounting reference.
//!
//! The execution session retains that reference without
//! becoming the owner of physical resource allocation.
//!
//! # MEMORY INTEGRATION
//!
//! Ordinary memory allocation belongs to:
//!
//! `crate::zur::memory`
//!
//! Its underlying implementation integrates with:
//!
//! `crate::classical::memory`
//!
//! Resource leases must not replace MemoryAllocation,
//! MemoryHandle, memory accounting, or memory reclamation.
//!
//! Memory allocation and physical resource reservation
//! are related but distinct operational responsibilities.
//!
//! A memory allocation handle must retain its canonical
//! ownership and release behavior.
//!
//! # DOMAIN INTEGRATION
//!
//! Classical:
//! `crate::classical`
//!
//! Quantum:
//! `crate::quantum::memory`
//! `crate::quantum::hardware`
//! `crate::quantum::ir`
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
//! ZUR supplies common lease lifecycle infrastructure
//! where the resource's operational contract supports it.
//!
//! Domain-specific resources must retain their own
//! canonical identities, ownership rules, and lifecycle
//! implementations.
//!
//! In particular, quantum states, physical qubits,
//! HDL resources, and remote resources must not
//! automatically be represented as ordinary host-memory
//! allocations.
//!
//! # OPEN-WORLD EXTENSIBILITY
//!
//! Resource categories and computational domains are
//! not enumerated in this module.
//!
//! New resource technologies must integrate through
//! their existing semantic, capability, provider,
//! and domain-specific contracts.
//!
//! Examples of future integrations include:
//!
//! - New classical processors.
//! - New accelerators.
//! - Quantum hardware.
//! - Photonic resources.
//! - Neuromorphic resources.
//! - Reconfigurable hardware.
//! - Distributed computing resources.
//! - Future computational technologies.
//!
//! Adding a new resource category must not require
//! modifying this composition root merely to recognize
//! its existence.
//!
//! # SCALABILITY AND POCO-REAF
//!
//! This module introduces no arbitrary universal maximum
//! for:
//!
//! - Resource leases.
//! - Resource categories.
//! - Resource quantities.
//! - Computational domains.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Accelerators.
//! - Devices.
//! - Distributed participants.
//! - Execution sessions.
//! - Resource authorities.
//!
//! Collections and resource-management operations grow
//! subject to available memory, provider capabilities,
//! operational budgets, and actual platform limitations.
//!
//! Explicit operational budgets are permitted.
//!
//! Such budgets must not become universal language-level
//! or architectural capacity restrictions.
//!
//! Literal infinite physical computation is not promised.
//!
//! The objective is to preserve program intent across
//! compatible execution environments.
//!
//! # FAILURE AND RESOURCE AVAILABILITY
//!
//! Resource acquisition must be explicitly confirmed.
//!
//! The following must remain distinguishable:
//!
//! - Unknown availability.
//! - Known unavailability.
//! - Unsupported resource.
//! - Infeasible requirement.
//! - Policy rejection.
//! - Budget exhaustion.
//! - Acquisition failure.
//! - Confirmed acquisition.
//! - Expiration.
//! - Uncertain external state.
//! - Confirmed release.
//!
//! Failed acquisition must not produce an active lease.
//!
//! Failed or ambiguous release must not be reported
//! as confirmed physical release.
//!
//! A resource lease must not be used after expiration,
//! confirmed release, or an uncertain lifecycle transition.
//!
//! # CONCURRENCY AND SAFETY
//!
//! The lease implementation provides synchronized
//! shared lease state using safe Rust abstractions.
//!
//! This composition root introduces:
//!
//! - No unsafe code.
//! - No raw pointer ownership.
//! - No static mutable state.
//! - No global resource manager.
//! - No implicit hardware discovery.
//! - No implicit resource acquisition.
//! - No fabricated availability.
//! - No fabricated reservation.
//! - No fabricated release.
//!
//! Resource authorities must be injected explicitly.
//!
//! # PUBLIC API STABILITY
//!
//! Existing lease types retain their canonical ownership
//! inside `lease.rs`.
//!
//! Re-exporting them does not create new implementations
//! or change their lifecycle contracts.
//!
//! Consumers may use the stable paths:
//!
//! `crate::zur::resources::ResourceLease`
//! `crate::zur::resources::ResourceLeaseManager`
//! `crate::zur::resources::ResourceLeaseAuthority`
//!
//! The detailed implementation remains available through:
//!
//! `crate::zur::resources::lease`
//!
//! # COMPLETION CRITERIA
//!
//! This module is complete when:
//!
//! - The existing lease implementation is exposed.
//! - No duplicate resource subsystem is introduced.
//! - Canonical identities are reused.
//! - Canonical execution-session references are reused.
//! - Semantic ownership remains with the frontend.
//! - Feasibility ownership remains with compiler planning.
//! - Physical reservation remains with the injected authority.
//! - Public module paths are stable.
//! - Safe Rust requirements are enforced.
//! - No arbitrary hardware ceilings are introduced.
//! - The parent ZUR composition root exports this module.

// ============================================================================
// RESOURCE LEASE IMPLEMENTATION
// ============================================================================

/// Explicit resource reservation and lease lifecycle.
///
/// This module is the authoritative implementation of
/// ZUR resource leases.
pub mod lease;

// ============================================================================
// PUBLIC RESOURCE-LEASE API
// ============================================================================

/// Authority responsible for actual resource acquisition,
/// renewal, and release.
pub use lease::ResourceLeaseAuthority;

/// Confirmed resource reservation grant.
pub use lease::LeaseGrant;

/// Explicit owner and lifetime scope of a resource lease.
pub use lease::LeaseOwner;

/// Observable resource lease lifecycle status.
pub use lease::LeaseStatus;

/// Safely owned handle to a confirmed resource reservation.
pub use lease::ResourceLease;

/// Manager coordinating resource lease identities
/// and acquisition through an injected authority.
pub use lease::ResourceLeaseManager;

// ============================================================================
// ARCHITECTURAL INVARIANTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensures the public lease API remains available
    /// through the canonical resource module path.
    #[test]
    fn public_lease_api_is_exposed() {
        fn assert_authority<T: ResourceLeaseAuthority + ?Sized>() {}

        fn assert_lease_type<T>() {}

        assert_authority::<dyn ResourceLeaseAuthority>();

        assert_lease_type::<ResourceLease>();
        assert_lease_type::<ResourceLeaseManager>();
        assert_lease_type::<LeaseOwner>();
        assert_lease_type::<LeaseGrant>();
        assert_lease_type::<LeaseStatus>();
    }

    /// Ensures the implementation remains accessible
    /// through its original module path.
    #[test]
    fn canonical_lease_module_is_available() {
        let _: Option<lease::ResourceLease> = None;
        let _: Option<lease::ResourceLeaseManager> = None;
        let _: Option<lease::LeaseOwner> = None;
    }
}
