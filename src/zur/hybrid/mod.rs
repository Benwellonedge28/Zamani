#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Hybrid Execution
//!
//! Path: `src/zur/hybrid/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//!
//! ## Purpose
//!
//! This module is the composition root for ZUR's hybrid execution
//! integration.
//!
//! It connects:
//!
//! - The canonical hybrid computation subsystem.
//! - The existing ZUR hybrid execution coordinator.
//! - The ZUR hybrid execution adapter.
//! - The existing runtime execution contracts.
//! - Explicitly injected hybrid execution authorities.
//!
//! It does not introduce another hybrid execution engine, graph,
//! planner, provider registry, resource manager, synchronization
//! implementation, or domain representation.
//!
//! ## Architectural ownership
//!
//! The authoritative responsibilities remain:
//!
//! | Responsibility | Authority |
//! | --- | --- |
//! | Hybrid source syntax | `grammar/hybrid/` |
//! | Hybrid language specification | `grammar/spec/hybrid.md` |
//! | Frontend parsing | `crate::frontend` |
//! | Semantic meaning | `crate::frontend::semantic` |
//! | Classical computation | `crate::classical` |
//! | Quantum computation | `crate::quantum` |
//! | HDL and hardware representation | Existing HDL and hardware subsystems |
//! | Hybrid graph | `crate::hybrid::graph` |
//! | Hybrid domain composition | `crate::hybrid::domain` |
//! | Hybrid data exchange | `crate::hybrid::data` |
//! | Hybrid control | `crate::hybrid::control` |
//! | Hybrid synchronization contracts | `crate::hybrid::sync` |
//! | Hybrid requirements and capabilities | `crate::hybrid::resources` |
//! | Hybrid realization planning | `crate::hybrid::planning` |
//! | Compilation and lowering | `crate::compiler` |
//! | Code generation | `crate::backend` |
//! | Distributed execution | `crate::distributed` |
//! | Runtime provider contracts | `crate::zur::contracts::provider` |
//! | Runtime execution contracts | `crate::zur::contracts::execution` |
//! | Hybrid runtime coordination | Existing ZUR coordinator |
//! | Hybrid runtime adaptation | `adapter` |
//!
//! This module only composes and exposes the ZUR-owned integration.
//!
//! ## Execution boundary
//!
//! The intended execution path is:
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! Canonical frontend and semantic validation
//!       |
//!       v
//! Canonical hybrid graph and dependency relationships
//!       |
//!       v
//! Canonical hybrid requirements and realization plan
//!       |
//!       v
//! Compiler and backend artifact production
//!       |
//!       v
//! ZUR hybrid execution adapter
//!       |
//!       v
//! ZUR hybrid execution coordinator
//!       |
//!       v
//! Explicit HybridExecutionAuthority
//!       |
//!       v
//! Existing execution engines and domain providers
//!       |
//!       v
//! Actual execution observations
//!       |
//!       v
//! Hybrid execution receipt
//! ```
//!
//! The coordinator validates the canonical request and delegates
//! execution. The injected authority is responsible for connecting
//! the request to actual execution infrastructure.
//!
//! This composition module does not claim that an authority, provider,
//! executable artifact, device, or resource is available merely because
//! its corresponding interface exists.
//!
//! ## Hybrid execution responsibilities
//!
//! The execution authority must preserve:
//!
//! - Every required computation.
//! - Every validated dependency.
//! - Cross-domain data ownership and lifetime contracts.
//! - Required synchronization guarantees.
//! - Control dependencies and feedback.
//! - Execution cancellation.
//! - Partial completion.
//! - Provider and device failures.
//! - Security and authorization requirements.
//! - Actual execution results and their provenance.
//!
//! It must not silently weaken requirements, remove dependencies,
//! replace unsupported operations with successful no-ops, or report
//! completion merely because work was submitted.
//!
//! ## Cross-domain support
//!
//! This module is domain-neutral.
//!
//! It does not enumerate supported computational domains in a closed
//! Rust enum.
//!
//! The underlying hybrid and provider contracts can represent
//! combinations involving:
//!
//! - Classical computation.
//! - Quantum computation.
//! - HDL and hardware execution.
//! - AI and tensor computation.
//! - Accelerators.
//! - Distributed execution.
//! - Networking.
//! - Photonics.
//! - Future computational domains.
//!
//! Actual support depends on validated semantic representations,
//! compatible compiler artifacts, provider implementations,
//! capabilities, authorization, and available resources.
//!
//! Adding a new domain must not require modifying this composition
//! module merely to add another domain name.
//!
//! ## Requirement and execution distinctions
//!
//! The integration preserves the following distinctions:
//!
//! - Requirement is not capability.
//! - Capability is not capability evidence.
//! - Capability evidence is not current availability.
//! - Compatibility is not feasibility.
//! - Feasibility is not resource reservation.
//! - Resource reservation is not execution.
//! - Submission is not completion.
//! - Partial completion is not full completion.
//! - Cancellation requested is not cancellation confirmed.
//!
//! Those distinctions are enforced by the canonical hybrid, provider,
//! resource, and execution contracts.
//!
//! ## Scalability
//!
//! This module imposes no arbitrary maximum on:
//!
//! - Hybrid graphs.
//! - Graph nodes.
//! - Dependency edges.
//! - Computational domains.
//! - Providers.
//! - Devices.
//! - Execution operations.
//! - Hybrid sessions.
//! - Distributed participants.
//! - Cross-domain interactions.
//! - Synchronization events.
//!
//! It introduces no fixed CPU, GPU, QPU, FPGA, memory, device,
//! thread, node, or domain capacity.
//!
//! Actual limits are determined by available address space, memory,
//! processing capacity, provider implementations, configured budgets,
//! and the canonical resource-management authorities.
//!
//! No finite machine can guarantee literally infinite computation.
//! POCO-REAF means preserving portable program intent and adapting
//! execution across compatible environments without arbitrary
//! language-level hardware ceilings.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden.
//!
//! This module introduces:
//!
//! - No unsafe blocks.
//! - No raw-pointer ownership.
//! - No global mutable state.
//! - No implicit runtime initialization.
//! - No hidden provider discovery.
//! - No physical resource allocation.
//! - No duplicate hybrid graph or plan.
//! - No fabricated execution results.
//!
//! ## Module compatibility
//!
//! The existing coordinator is retained in:
//!
//! `src/zur/hybrid.rs`
//!
//! It is included as a private implementation module below.
//! This avoids copying its implementation or changing its ownership.
//!
//! The existing adapter remains in:
//!
//! `src/zur/hybrid/adapter.rs`
//!
//! The parent ZUR module must explicitly select this file as the
//! hybrid module composition root. See the integration instructions
//! accompanying this file.

// ============================================================================
// EXISTING COORDINATOR IMPLEMENTATION
// ============================================================================

/// Existing ZUR hybrid coordinator implementation.
///
/// The implementation remains in `src/zur/hybrid.rs` so this composition
/// root does not duplicate or fork its execution contracts.
///
/// This module is private because callers should use the stable public
/// exports below.
#[path = "../hybrid.rs"]
mod coordinator;

// ============================================================================
// PUBLIC HYBRID EXECUTION ADAPTER
// ============================================================================

/// ZUR adapter for canonical hybrid computation requests.
///
/// This adapter validates canonical hybrid inputs and delegates
/// execution to the existing coordinator and its injected authority.
///
/// It does not implement a second hybrid execution engine.
pub mod adapter;

// ============================================================================
// STABLE PUBLIC COORDINATOR API
// ============================================================================

/// Asynchronous future returned by a hybrid execution authority.
pub use coordinator::HybridExecutionFuture;

/// Authority responsible for actual hybrid execution.
pub use coordinator::HybridExecutionAuthority;

/// Validated request referencing canonical hybrid representations.
pub use coordinator::HybridExecutionRequest;

/// Authoritative receipt for an observed hybrid execution.
pub use coordinator::HybridExecutionReceipt;

/// Coordinator that delegates validated requests to an injected authority.
pub use coordinator::HybridExecutionCoordinator;

/// Documentation of the canonical hybrid integration boundaries.
pub use coordinator::HybridIntegrationContract;

// ============================================================================
// STABLE PUBLIC ADAPTER API
// ============================================================================

/// Public adapter for constructing and executing canonical hybrid requests.
pub use adapter::HybridExecutionAdapter;

// ============================================================================
// ARCHITECTURAL INVARIANTS
// ============================================================================

/// Architectural invariants for ZUR hybrid integration.
///
/// These invariants are descriptive checks over the established
/// ownership boundaries. They do not replace validation performed
/// by the canonical hybrid or execution subsystems.
pub mod invariants {
    /// Canonical hybrid computation authority.
    pub const HYBRID_AUTHORITY: &str = "crate::hybrid";

    /// Canonical hybrid graph authority.
    pub const GRAPH_AUTHORITY: &str = "crate::hybrid::graph";

    /// Canonical hybrid planning authority.
    pub const PLANNING_AUTHORITY: &str = "crate::hybrid::planning";

    /// Canonical hybrid data-contract authority.
    pub const DATA_AUTHORITY: &str = "crate::hybrid::data";

    /// Canonical hybrid synchronization authority.
    pub const SYNCHRONIZATION_AUTHORITY: &str = "crate::hybrid::sync";

    /// Canonical hybrid resource-contract authority.
    pub const RESOURCE_AUTHORITY: &str = "crate::hybrid::resources";

    /// Canonical ZUR execution-contract authority.
    pub const EXECUTION_AUTHORITY: &str =
        "crate::zur::contracts::execution";

    /// Canonical ZUR provider-contract authority.
    pub const PROVIDER_AUTHORITY: &str =
        "crate::zur::contracts::provider";

    /// Canonical distributed-computation authority.
    pub const DISTRIBUTED_AUTHORITY: &str =
        "crate::distributed";

    /// Canonical quantum-computation authority.
    pub const QUANTUM_AUTHORITY: &str =
        "crate::quantum";

    /// Canonical compiler authority.
    pub const COMPILER_AUTHORITY: &str =
        "crate::compiler";

    /// Canonical backend authority.
    pub const BACKEND_AUTHORITY: &str =
        "crate::backend";
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_hybrid_authority_is_preserved() {
        assert_eq!(
            invariants::HYBRID_AUTHORITY,
            "crate::hybrid",
        );

        assert_eq!(
            HybridIntegrationContract::hybrid_authority(),
            invariants::HYBRID_AUTHORITY,
        );
    }

    #[test]
    fn canonical_execution_authority_is_preserved() {
        assert_eq!(
            HybridIntegrationContract::execution_authority(),
            invariants::EXECUTION_AUTHORITY,
        );
    }

    #[test]
    fn canonical_provider_authority_is_preserved() {
        assert_eq!(
            HybridIntegrationContract::provider_authority(),
            invariants::PROVIDER_AUTHORITY,
        );
    }

    #[test]
    fn canonical_graph_and_planning_owners_are_distinct() {
        assert_ne!(
            invariants::GRAPH_AUTHORITY,
            invariants::PLANNING_AUTHORITY,
        );
    }

    #[test]
    fn canonical_data_and_synchronization_owners_are_distinct() {
        assert_ne!(
            invariants::DATA_AUTHORITY,
            invariants::SYNCHRONIZATION_AUTHORITY,
        );
    }

    #[test]
    fn distributed_execution_remains_separately_owned() {
        assert_ne!(
            invariants::DISTRIBUTED_AUTHORITY,
            invariants::EXECUTION_AUTHORITY,
        );
    }

    #[test]
    fn quantum_execution_remains_separately_owned() {
        assert_ne!(
            invariants::QUANTUM_AUTHORITY,
            invariants::EXECUTION_AUTHORITY,
        );
    }

    #[test]
    fn compiler_and_backend_remain_separate_authorities() {
        assert_ne!(
            invariants::COMPILER_AUTHORITY,
            invariants::BACKEND_AUTHORITY,
        );
    }

    #[test]
    fn adapter_is_exposed_through_the_composition_root() {
        let adapter_type =
            core::any::type_name::<HybridExecutionAdapter>();

        assert!(
            adapter_type.contains("HybridExecutionAdapter"),
        );
    }

    #[test]
    fn coordinator_is_exposed_through_the_composition_root() {
        let coordinator_type =
            core::any::type_name::<HybridExecutionCoordinator>();

        assert!(
            coordinator_type.contains("HybridExecutionCoordinator"),
        );
    }

    #[test]
    fn execution_request_is_exposed_through_the_composition_root() {
        let request_type =
            core::any::type_name::<HybridExecutionRequest<'static>>();

        assert!(
            request_type.contains("HybridExecutionRequest"),
        );
    }

    #[test]
    fn execution_receipt_is_exposed_through_the_composition_root() {
        let receipt_type =
            core::any::type_name::<HybridExecutionReceipt>();

        assert!(
            receipt_type.contains("HybridExecutionReceipt"),
        );
    }
}