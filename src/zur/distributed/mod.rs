
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # Zamani Universal Runtime — Distributed Integration
//!
//! Path: src/zur/distributed/mod.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # 1. PURPOSE
//!
//! This module is the composition root for distributed execution
//! integration within the Zamani Universal Runtime (ZUR).
//!
//! It connects ZUR to the canonical distributed computing
//! implementation owned by:
//!
//!     crate::distributed
//!
//! It provides the integration boundaries required for:
//!
//! - Distributed execution.
//! - Distributed execution-plan resolution.
//! - Distributed provider integration.
//! - Execution-session association.
//! - Execution lifecycle integration.
//! - Cancellation delegation.
//! - Partial-result preservation.
//! - Distributed recovery.
//! - Runtime service integration.
//! - Cross-domain distributed execution.
//!
//! Its architectural objective is:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module is an integration layer.
//!
//! It is NOT another distributed computing implementation.
//!
//! # 2. AUTHORITATIVE REPOSITORY SUBSYSTEMS
//!
//! The following subsystems remain authoritative.
//!
//! Distributed computation:
//!     crate::distributed
//!
//! Distributed execution:
//!     crate::distributed::execution
//!
//! Distributed execution plans:
//!     crate::distributed::execution::executor::ExecutablePlan
//!
//! Distributed executor:
//!     crate::distributed::execution::executor::DistributedExecutor
//!
//! Distributed planning:
//!     crate::distributed::planning
//!
//! Distributed membership:
//!     crate::distributed::membership
//!
//! Distributed transport:
//!     crate::distributed::transport
//!
//! Distributed security:
//!     crate::distributed::security
//!
//! Distributed resilience:
//!     crate::distributed::resilience
//!
//! Distributed recovery:
//!     crate::distributed::resilience::recovery
//!
//! Distributed identities:
//!     crate::distributed::ids
//!
//! Distributed errors:
//!     crate::distributed::error
//!
//! ZUR execution:
//!     crate::zur::execution
//!
//! ZUR provider contracts:
//!     crate::zur::contracts::provider
//!
//! ZUR execution contracts:
//!     crate::zur::contracts::execution
//!
//! ZUR execution sessions:
//!     crate::zur::execution::session
//!
//! ZUR operational context:
//!     crate::zur::core::context
//!
//! ZUR errors:
//!     crate::zur::error
//!
//! ZUR resource management:
//!     crate::zur::resources
//!
//! # 3. SOURCE AND GRAMMAR AUTHORITY
//!
//! Normative distributed semantics:
//!     grammar/spec/distributed.md
//!
//! Distributed grammar composition:
//!     grammar/distributed/distributed.g4
//!
//! Distributed grammar architecture:
//!     grammar/distributed/README.md
//!
//! Distributed node intent:
//!     grammar/distributed/nodes.g4
//!
//! Distributed task intent:
//!     grammar/distributed/tasks.g4
//!
//! Distributed remote execution intent:
//!     grammar/distributed/remote-execution.g4
//!
//! Distributed fault tolerance:
//!     grammar/distributed/fault-tolerance.g4
//!
//! Distributed execution intent:
//!     grammar/execution/distributed-execution.g4
//!
//! Distributed memory intent:
//!     grammar/memory/distributed-memory.g4
//!
//! Distributed effect intent:
//!     grammar/effects/distributed.g4
//!
//! Grammar defines source syntax.
//!
//! Semantic analysis establishes program meaning.
//!
//! Canonical IR represents computation.
//!
//! Distributed planning determines a feasible realization.
//!
//! Distributed execution performs the actual computation.
//!
//! ZUR integrates the execution facilities.
//!
//! This module MUST NOT:
//!
//! - Parse grammar.
//! - Reinterpret source syntax.
//! - Construct another distributed AST.
//! - Redefine distributed semantics.
//! - Construct another distributed IR.
//! - Independently reinterpret resource requirements.
//! - Override semantic capabilities or constraints.
//!
//! # 4. OWNERSHIP
//!
//! This module owns:
//!
//! - Distributed ZUR module composition.
//! - Stable distributed integration module paths.
//! - Distributed adapter exposure.
//! - Distributed session integration exposure.
//! - Distributed recovery integration exposure.
//! - Existing distributed integration API compatibility.
//! - Module-level integration invariants.
//!
//! This module DOES NOT own:
//!
//! - Distributed task identities.
//! - Distributed participant identities.
//! - Distributed execution plans.
//! - Distributed workload graphs.
//! - Distributed scheduling.
//! - Distributed placement.
//! - Distributed partitioning.
//! - Distributed routing.
//! - Participant discovery.
//! - Membership management.
//! - Physical resource allocation.
//! - Resource reservation.
//! - Network transport.
//! - Network protocols.
//! - Message encoding.
//! - Authentication.
//! - Authorization.
//! - Consensus.
//! - Replication.
//! - Checkpoint storage.
//! - Recovery algorithms.
//! - Compiler optimization.
//! - Backend selection.
//! - Domain-specific IR.
//! - Quantum execution.
//! - HDL execution.
//! - AI computation.
//! - Hybrid computation.
//!
//! These responsibilities remain with their canonical owners.
//!
//! # 5. EXECUTION ARCHITECTURE
//!
//! ```text
//!               ZAMANI SOURCE
//!                     |
//!                     v
//!              Canonical frontend
//!                     |
//!                     v
//!              Semantic validation
//!                     |
//!                     v
//!             Canonical semantic facts
//!                     |
//!                     v
//!                Canonical IR
//!                     |
//!                     v
//!           Distributed planning
//!                     |
//!          +----------+----------+
//!          |          |          |
//!          v          v          v
//!       Feasibility Placement Routing
//!          |          |          |
//!          +----------+----------+
//!                     |
//!                     v
//!               Scheduling
//!                     |
//!                     v
//!           Validated execution plan
//!                     |
//!                     v
//!              ZUR execution
//!                     |
//!                     v
//!          Distributed integration
//!                     |
//!                     v
//!            Plan resolution
//!                     |
//!                     v
//!           Canonical distributed
//!                 executor
//!                     |
//!          +----------+----------+
//!          |          |          |
//!          v          v          v
//!       Security  Transport  Resilience
//!          |          |          |
//!          +----------+----------+
//!                     |
//!                     v
//!            Execution providers
//!                     |
//!                     v
//!             Actual computation
//!                     |
//!                     v
//!            Verified outcomes
//! ```
//!
//! # 6. EXECUTION INVARIANTS
//!
//! The following distinctions are mandatory.
//!
//! Registration != initialization.
//! Initialization != availability.
//! Availability != capability.
//! Capability != feasibility.
//! Feasibility != resource reservation.
//! Resource reservation != execution.
//! Submission != acceptance.
//! Acceptance != completion.
//! Delivery != successful execution.
//! Cancellation request != cancellation confirmation.
//! Recovery action != recovered computation.
//! Recovery completion != recovery verification.
//!
//! ZUR MUST preserve these distinctions.
//!
//! # 7. SCALABILITY AND POCO-REAF
//!
//! This module MUST NOT impose arbitrary universal maximums on:
//!
//! - Distributed plans.
//! - Participants.
//! - Logical nodes.
//! - Tasks.
//! - Processes.
//! - Workers.
//! - Providers.
//! - Devices.
//! - Execution contexts.
//! - Computational domains.
//! - Communication channels.
//! - Results.
//! - Recovery operations.
//! - Resource quantities.
//!
//! No closed hardware catalogue is permitted.
//!
//! No closed computational-domain inventory is permitted.
//!
//! No fixed participant-to-machine relationship is permitted.
//!
//! No first-registered-provider execution assumption is permitted.
//!
//! Actual execution is governed by:
//!
//! - Semantic requirements.
//! - Validated execution plans.
//! - Provider capabilities.
//! - Available resources.
//! - Available communication facilities.
//! - Security authorization.
//! - Configured operational policies.
//! - Platform representation limits.
//!
//! Operational budgets belong to their authoritative owners.
//!
//! Resource exhaustion MUST produce an explicit failure.
//!
//! Literal infinite physical computation is not promised.
//!
//! # 8. FAILURE INTEGRITY
//!
//! The integration MUST preserve:
//!
//! - Original distributed errors.
//! - Provider failure information.
//! - Transport failure information.
//! - Partial execution reports.
//! - Successfully completed work.
//! - Uncompleted work.
//! - Unknown execution outcomes.
//! - Recovery status.
//! - Cancellation status.
//!
//! An unknown remote execution outcome MUST NOT be converted into
//! successful or definitely failed execution without evidence.
//!
//! A partial result MUST NOT be promoted to full success.
//!
//! # 9. SECURITY
//!
//! Distributed execution must use the canonical security subsystem.
//!
//! Discovery is not authentication.
//!
//! Authentication is not authorization.
//!
//! Membership is not permission to execute.
//!
//! A valid plan is not authorization.
//!
//! Security checks remain at the protected execution boundary.
//!
//! This module must not manufacture authentication evidence,
//! authorization permits, trusted identities or security context.
//!
//! # 10. CONCURRENCY AND LIFECYCLE
//!
//! This composition root:
//!
//! - Creates no global mutable state.
//! - Starts no background threads.
//! - Opens no network connections.
//! - Does not perform implicit discovery.
//! - Does not allocate physical resources.
//! - Does not create another task queue.
//! - Does not initialize a runtime singleton.
//!
//! Runtime construction and shutdown remain controlled by ZUR.
//!
//! # 11. MODULE COMPOSITION
//!
//! Each child module has one authoritative responsibility.
//!
//! adapter.rs:
//!     Resolves ZUR plan references and delegates execution to
//!     the canonical DistributedExecutor.
//!
//! session.rs:
//!     Associates existing ZUR execution sessions with canonical
//!     distributed execution plans.
//!
//! recovery.rs:
//!     Delegates recovery operations to the canonical distributed
//!     resilience and recovery subsystem.
//!
//! integration:
//!     Preserves the original distributed provider and domain
//!     integration API from src/zur/distributed.rs.
//!
//! The compatibility integration is retained temporarily to
//! avoid discarding existing public functionality.
//!
//! # 12. SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe.
//! No raw-pointer ownership.
//! No static mutable state.
//! No fabricated execution results.
//! No hidden global runtime.
//!
//! # 13. COMPLETION CRITERIA
//!
//! This module is complete when:
//!
//! - All declared child modules exist.
//! - The original public integration API remains accessible.
//! - Canonical distributed execution is reused.
//! - Canonical recovery is reused.
//! - Session ownership remains with ZUR.
//! - Errors and partial results are preserved.
//! - No duplicate distributed subsystem is introduced.
//! - No arbitrary scalability ceiling is introduced.
//! - No unsafe Rust is permitted.
//! - Parent ZUR module composition is correct.
//! - Repository integration tests pass.

// ============================================================================
// API IDENTITY
// ============================================================================

/// Stable public API version for the ZUR distributed composition boundary.
///
/// This identifies module composition, not the distributed execution
/// plan schema, runtime ABI, or network protocol.
pub const DISTRIBUTED_ZUR_API_VERSION: u16 = 1;

/// Stable architectural identity of the ZUR distributed integration.
pub const DISTRIBUTED_ZUR_COMPONENT: &str =
    "zamani.zur.distributed";

// ============================================================================
// EXISTING IMPLEMENTATION COMPATIBILITY
// ============================================================================

// IMPORTANT:
//
// src/zur/distributed.rs already exists and contains the original
// distributed provider/domain integration implementation.
//
// The parent src/zur/mod.rs MUST explicitly select this directory's
// mod.rs using:
//
// #[path = "distributed/mod.rs"]
// pub mod distributed;
//
// The path attribute prevents the ordinary distributed.rs versus
// distributed/mod.rs module-resolution conflict.
//
// This private module preserves the existing implementation without
// creating another copy.
//
// The old file MUST NOT be deleted until its implementation has been
// deliberately migrated and its compatibility requirements tested.

#[path = "../distributed.rs"]
mod integration;

// ============================================================================
// DISTRIBUTED INTEGRATION COMPONENTS
// ============================================================================

/// ZUR adapter over the canonical distributed execution engine.
///
/// This module owns plan resolution, execution delegation and
/// preservation of canonical distributed execution reports.
pub mod adapter;

/// ZUR adapter over canonical distributed recovery.
///
/// Recovery algorithms, durable journaling and reconciliation remain
/// owned by the distributed resilience subsystem.
pub mod recovery;

/// Association between ZUR execution sessions and canonical
/// distributed execution plans.
pub mod session;

// ============================================================================
// PRESERVED PUBLIC DISTRIBUTED INTEGRATION API
// ============================================================================

/// Canonical distributed domain identity.
pub use self::integration::distributed_domain;

/// Constructs the canonical ZUR distributed domain descriptor.
pub use self::integration::distributed_domain_descriptor;

/// Validates a distributed domain descriptor.
pub use self::integration::validate_distributed_domain_descriptor;

/// Validates a ZUR distributed provider.
pub use self::integration::validate_distributed_provider;

/// Validates an existing canonical distributed executable plan.
pub use self::integration::validate_distributed_plan;

/// Validates an operation against the provider's declared contract.
pub use self::integration::validate_distributed_operation;

/// Invokes an explicitly supplied distributed provider.
pub use self::integration::invoke_distributed;

/// Submits a distributed operation without confusing acceptance
/// with successful completion.
pub use self::integration::submit_distributed;

/// Typed envelope around an existing canonical distributed plan.
///
/// This does not introduce a competing execution-plan representation.
pub use self::integration::DistributedPlanPayload;

/// Canonical distributed execution interface identifier.
pub use self::integration::DISTRIBUTED_EXECUTION_INTERFACE;

/// Canonical distributed plan payload contract identifier.
pub use self::integration::DISTRIBUTED_PLAN_CONTRACT;

/// Canonical distributed result contract identifier.
pub use self::integration::DISTRIBUTED_RESULT_CONTRACT;

/// Existing distributed integration prelude.
pub use self::integration::prelude;

// ============================================================================
// MODULE-LEVEL TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::version::ServiceContractVersion;

    #[test]
    fn distributed_api_version_is_stable_and_nonzero() {
        assert_ne!(DISTRIBUTED_ZUR_API_VERSION, 0);

        assert_eq!(
            DISTRIBUTED_ZUR_COMPONENT,
            "zamani.zur.distributed"
        );
    }

    #[test]
    fn distributed_domain_uses_canonical_identity() {
        let domain = distributed_domain();

        assert!(
            domain.is("zamani", "distributed")
        );

        assert_eq!(
            domain.qualified_name(),
            "zamani.distributed"
        );
    }

    #[test]
    fn distributed_domain_descriptor_is_valid() {
        let version = ServiceContractVersion::new(
            "1",
            "0",
            "0",
        )
        .expect("valid contract version");

        let descriptor = distributed_domain_descriptor(
            version,
        )
        .expect("distributed descriptor must be constructible");

        validate_distributed_domain_descriptor(
            &descriptor,
        )
        .expect("distributed descriptor must validate");
    }

    #[test]
    fn distributed_contract_identities_are_distinct() {
        assert_ne!(
            DISTRIBUTED_EXECUTION_INTERFACE,
            DISTRIBUTED_PLAN_CONTRACT
        );

        assert_ne!(
            DISTRIBUTED_PLAN_CONTRACT,
            DISTRIBUTED_RESULT_CONTRACT
        );
    }

    #[test]
    fn existing_integration_prelude_is_preserved() {
        let _ = prelude::distributed_domain;
        let _ = prelude::validate_distributed_plan;
        let _ = prelude::validate_distributed_provider;
        let _ = prelude::submit_distributed;
    }
}
