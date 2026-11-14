
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime (ZUR)
//!
//! Path: `src/zur/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! Primary architectural objective:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module is the canonical composition root for the Zamani Universal
//! Runtime (ZUR).
//!
//! ZUR provides the operational infrastructure required to execute
//! compatible Zamani programs across available computing environments.
//!
//! It supports integration with:
//!
//! - Classical computing.
//! - Quantum computing.
//! - Quantum simulation.
//! - HDL and hardware execution.
//! - AI and machine learning.
//! - Tensor computation.
//! - GPU and other accelerators.
//! - Hybrid computation.
//! - Distributed computation.
//! - Embedded computing.
//! - Networking.
//! - Cryptographic computation.
//! - Photonic computing.
//! - Future computational domains.
//!
//! These are extensibility objectives, not claims that every physical
//! implementation or provider is already available.
//!
//! This file composes existing ZUR implementations.
//!
//! It does not implement a second runtime, compiler, intermediate
//! representation, hardware abstraction layer, scheduler, or execution
//! engine.
//!
//! ============================================================================
//! ARCHITECTURAL AUTHORITIES
//! ============================================================================
//!
//! ZUR integrates with the following canonical repository authorities.
//!
//! LANGUAGE:
//!     grammar/
//!
//! FRONTEND:
//!     crate::frontend
//!
//! SEMANTICS:
//!     crate::frontend::semantic
//!
//! UNIFIED INTERMEDIATE REPRESENTATION:
//!     crate::zuir
//!
//! CLASSICAL COMPUTATION:
//!     crate::classical
//!
//! QUANTUM COMPUTATION:
//!     crate::quantum
//!
//! HARDWARE DESCRIPTION:
//!     Existing HDL and hardware subsystems.
//!
//! ARTIFICIAL INTELLIGENCE:
//!     crate::ai
//!
//! HYBRID COMPUTATION:
//!     crate::hybrid
//!
//! DISTRIBUTED COMPUTATION:
//!     crate::distributed
//!
//! COMPILER:
//!     crate::compiler
//!
//! BACKEND:
//!     crate::backend
//!
//! STANDARD LIBRARY:
//!     crate::stdlib
//!
//! ZUR owns operational runtime coordination and execution integration.
//!
//! It consumes the representations, plans, requirements, capabilities,
//! artifacts and contracts supplied by their respective authorities.
//!
//! ============================================================================
//! FUNDAMENTAL ARCHITECTURE
//! ============================================================================
//!
//! ```text
//!                   ZAMANI SOURCE
//!                         |
//!                         v
//!                  CANONICAL FRONTEND
//!                         |
//!                         v
//!                 SEMANTIC VALIDATION
//!                         |
//!                         v
//!                 CANONICAL IR SYSTEMS
//!                         |
//!             +-----------+-----------+
//!             |           |           |
//!             v           v           v
//!         Classical    Quantum       HDL
//!             |           |           |
//!             +-----------+-----------+
//!                         |
//!                         v
//!                        ZUIR
//!                         |
//!                         v
//!                COMPILER AND PLANNING
//!                         |
//!                         v
//!                 BACKEND AND ARTIFACTS
//!                         |
//!                         v
//!               VALIDATED EXECUTION INPUT
//!                         |
//!                         v
//!                        ZUR
//!                         |
//!          +--------------+--------------+
//!          |              |              |
//!          v              v              v
//!       Direct         Managed        Provider
//!      Execution      Execution      Execution
//!          |              |              |
//!          +--------------+--------------+
//!                         |
//!                         v
//!               ACTUAL EXECUTION RESULTS
//!                         |
//!                         v
//!                OBSERVATION / RECOVERY
//! ```
//!
//! ZUR does not independently reconstruct source-language meaning,
//! optimize compiler representations, select compiler backends,
//! or construct domain-specific execution plans.
//!
//! ============================================================================
//! POCO-REAF EXECUTION MODELS
//! ============================================================================
//!
//! Zamani must support three complementary execution models.
//!
//! 1. RUNTIME-FREE EXECUTION
//!
//!    Compatible AOT artifacts execute without requiring a separately
//!    installed ZUR runtime.
//!
//!    Runtime-free execution remains possible when the ZUR module is
//!    not included in the application build.
//!
//! 2. RUNTIME-ENABLED EXECUTION
//!
//!    Programs explicitly use the operational services provided by ZUR.
//!
//!    Services are activated only when required and configured.
//!
//! 3. HYBRID EXECUTION
//!
//!    Direct execution, runtime-managed execution and external execution
//!    providers may participate in one compatible execution workflow.
//!
//! A runtime-enabled application does not automatically require every
//! runtime service.
//!
//! JIT, distributed, quantum, HDL, AI and other integrations remain
//! optional according to their implementations and build configuration.
//!
//! ============================================================================
//! OWNERSHIP BOUNDARIES
//! ============================================================================
//!
//! ZUR owns:
//!
//! - Runtime composition.
//! - Runtime lifecycle.
//! - Runtime configuration consumption.
//! - Runtime operational policy enforcement.
//! - Runtime service management.
//! - Execution sessions.
//! - Runtime execution coordination.
//! - Runtime provider integration.
//! - Operational resource leases.
//! - Operational memory and concurrency integration.
//! - Runtime security integration.
//! - Runtime reliability coordination.
//! - Runtime observability.
//! - Runtime interoperability.
//!
//! ZUR DOES NOT OWN:
//!
//! - Source-language grammar.
//! - Lexical analysis or parsing.
//! - AST construction.
//! - Source-language semantic meaning.
//! - Type inference or semantic validation.
//! - Canonical requirements.
//! - Capability definitions.
//! - Capability evidence authorities.
//! - Preference or constraint semantics.
//! - Feasibility analysis.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - AI semantic representation.
//! - Tensor storage authority.
//! - ZUIR.
//! - Compiler optimization.
//! - Compiler planning.
//! - Backend code generation.
//! - Backend registration.
//! - Quantum routing or error correction.
//! - Hybrid graph construction or planning.
//! - Distributed placement or scheduling.
//! - Network transport implementation.
//! - Physical hardware discovery.
//! - Hardware drivers.
//! - Physical resource allocation algorithms owned by device providers.
//!
//! Those responsibilities remain with their canonical repository owners.
//!
//! ============================================================================
//! REQUIREMENT / CAPABILITY / FEASIBILITY SEPARATION
//! ============================================================================
//!
//! Every integration must preserve the distinction between:
//!
//! - Requirement.
//! - Capability.
//! - Preference.
//! - Constraint.
//! - Hint.
//! - Feasibility.
//! - Resource availability.
//! - Resource reservation.
//! - Execution submission.
//! - Actual execution result.
//!
//! These concepts are not interchangeable.
//!
//! In particular:
//!
//! A registered provider does not prove that a device is available.
//!
//! A declared capability does not independently prove feasibility.
//!
//! Feasibility does not reserve physical resources.
//!
//! Resource reservation does not establish execution completion.
//!
//! Execution submission does not establish successful execution.
//!
//! Unknown or unsupported operations must produce explicit outcomes.
//!
//! ============================================================================
//! SCALABILITY AND RESOURCE-PARAMETRIC EXECUTION
//! ============================================================================
//!
//! ZUR introduces no arbitrary universal maximum for:
//!
//! - CPUs.
//! - Processor cores.
//! - Threads.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Accelerators.
//! - Distributed participants.
//! - Nodes.
//! - Memory.
//! - Storage.
//! - Qubits.
//! - Registers.
//! - Tensor dimensions.
//! - Tensor rank.
//! - Providers.
//! - Computational domains.
//! - Execution sessions.
//! - Runtime services.
//! - Tasks.
//! - Execution graph components.
//!
//! Actual implementation capacity is determined by:
//!
//! - Available memory.
//! - Available processing capacity.
//! - Available physical devices.
//! - Available providers.
//! - Address-space limitations.
//! - Platform capabilities.
//! - Semantic requirements.
//! - Execution policies.
//! - Operational budgets.
//! - Implementation capabilities.
//!
//! Optional operational budgets are configuration, not permanent
//! computational-domain or hardware restrictions.
//!
//! Budget exhaustion must produce an explicit result or error.
//!
//! No finite machine can provide literally infinite computation.
//!
//! POCO-REAF means that Zamani does not impose arbitrary universal
//! hardware ceilings and that compatible programs can adapt to
//! different execution environments without unnecessary source changes.
//!
//! ============================================================================
//! SAFETY
//! ============================================================================
//!
//! Unsafe Rust is prohibited throughout the ZUR module tree.
//!
//! This composition root does not weaken the crate-wide safety policy.
//!
//! Global mutable runtime state must not be introduced.
//!
//! Runtime dependencies must be explicit.
//!
//! External inputs, provider descriptors, execution requests and
//! serialized artifacts must be validated at their appropriate boundaries.
//!
//! Unsupported functionality must never be reported as successful.
//!
//! ============================================================================
//! MODULE COMPOSITION
//! ============================================================================
//!
//! The modules below expose the existing ZUR implementations.
//!
//! Each module retains responsibility for its own implementation.
//!
//! This file does not declare child implementation files individually
//! when their own composition roots already own those declarations.

// ============================================================================
// FOUNDATIONAL RUNTIME CONTRACTS
// ============================================================================

/// Canonical ZUR operational error model.
///
/// Owns structured operational errors and result types.
pub mod error;

/// Runtime-specific identities.
///
/// Does not duplicate domain, IR, backend, device or distributed identities.
pub mod ids;

/// Runtime contract and ABI versioning.
///
/// Keeps runtime contract versions distinct from language, package,
/// grammar, IR, compiler and backend versions.
pub mod version;

/// Immutable runtime configuration and optional operational budgets.
pub mod config;

/// Runtime operational policies.
///
/// Does not redefine source-language semantic policies.
pub mod policy;

/// Runtime execution ABI, execution, provider and service contracts.
pub mod contracts;

// ============================================================================
// RUNTIME CORE AND LIFECYCLE
// ============================================================================

/// Runtime context, lifecycle, service registration, service management
/// and explicit runtime construction.
pub mod core;

// ============================================================================
// EXECUTION INFRASTRUCTURE
// ============================================================================

/// Artifact validation, loading, sessions, execution instances,
/// dispatch, cancellation and execution outcomes.
pub mod execution;

// ============================================================================
// OPTIONAL OPERATIONAL SERVICES
// ============================================================================

/// Runtime service dependency resolution.
pub mod services;

/// Runtime memory integration and safe allocation interfaces.
pub mod memory;

/// Runtime concurrency and task execution.
pub mod concurrency;

/// Operational runtime scheduling.
///
/// Does not replace compiler, quantum or distributed scheduling.
pub mod scheduling;

/// Operational resource leases and accounting interfaces.
pub mod resources;

/// Host and device input/output integration.
pub mod io;

// ============================================================================
// PLATFORM INTEGRATION
// ============================================================================

/// Host platform descriptions, platform providers and compatibility.
pub mod platforms;

// ============================================================================
// JUST-IN-TIME AND ADAPTIVE EXECUTION
// ============================================================================

/// Optional JIT compilation and adaptive execution infrastructure.
///
/// Uses canonical compiler and backend authorities.
pub mod jit;

// ============================================================================
// COMPUTATIONAL-DOMAIN INTEGRATION
// ============================================================================

/// Open-ended computational-domain provider integration.
///
/// Supports existing and future computational domains without
/// requiring a closed domain enumeration.
pub mod domains;

/// Hybrid execution coordination and canonical hybrid adapters.
pub mod hybrid;

/// Distributed execution and recovery integration.
///
/// Delegates distributed execution to crate::distributed.
pub mod distributed;

// ============================================================================
// SECURITY AND RELIABILITY
// ============================================================================

/// Runtime security and isolation enforcement.
pub mod security;

/// Runtime faults, checkpoint integration and recovery coordination.
pub mod reliability;

// ============================================================================
// OBSERVABILITY
// ============================================================================

/// Runtime logging, metrics, tracing and profiling.
pub mod observability;

// ============================================================================
// INTEROPERABILITY
// ============================================================================

/// Safe interoperability with external execution environments.
pub mod interop;

// ============================================================================
// REAL-TIME EXECUTION
// ============================================================================

/// Deadline representation and real-time execution monitoring.
pub mod realtime;

// ============================================================================
// TESTING AND CONFORMANCE
// ============================================================================

/// Runtime provider conformance, portability, scalability
/// and fault-injection testing infrastructure.
pub mod testing;

// ============================================================================
// FOUNDATIONAL PUBLIC API
// ============================================================================
//
// Keep these exports deliberately small.
//
// Detailed functionality remains accessible through its owning
// module rather than being duplicated at the crate root.

/// Runtime configuration.
pub use config::RuntimeConfig;

/// Runtime construction and instance management.
pub use core::{
    RuntimeBuilder,
    RuntimeContext,
    RuntimeInstance,
    RuntimeLifecycle,
    RuntimeState,
};

/// Canonical operational errors and result types.
pub use error::{
    RuntimeError,
    RuntimeErrorKind,
    RuntimeResult,
    ZurError,
    ZurResult,
};

/// Runtime operational policies.
pub use policy::{
    EffectiveRuntimePolicy,
    RuntimePolicy,
};

// ============================================================================
// ARCHITECTURAL INVARIANTS
// ============================================================================
//
// These invariants govern every module composed above.
//
// 1. Semantic authority remains with the frontend.
// 2. Canonical domain IRs remain independently owned.
// 3. ZUIR remains the universal IR authority.
// 4. Compiler planning remains separate from runtime execution.
// 5. Backend registration remains separate from runtime providers.
// 6. Runtime-free execution remains supported.
// 7. Runtime-enabled execution is modular.
// 8. Optional services are not initialized unconditionally.
// 9. Provider integration is open-ended.
// 10. Computational domains are not represented by a closed inventory.
// 11. Requirements remain distinct from capabilities.
// 12. Capability evidence remains distinct from availability.
// 13. Feasibility remains distinct from reservation.
// 14. Reservation remains distinct from execution.
// 15. Execution submission remains distinct from completion.
// 16. Partial execution outcomes are preserved.
// 17. Unsupported operations return explicit failures.
// 18. Recovery is delegated to authoritative recovery systems.
// 19. Quantum routing and QEC remain quantum subsystem responsibilities.
// 20. Hybrid planning remains hybrid subsystem responsibility.
// 21. Distributed planning and transport remain distributed subsystem
//     responsibilities.
// 22. Resource budgets do not become universal hardware ceilings.
// 23. No arbitrary fixed hardware limits are introduced.
// 24. No global mutable runtime singleton is introduced.
// 25. Unsafe Rust remains prohibited.
// 26. New computational domains integrate through explicit contracts.
// 27. Runtime module composition does not duplicate implementations.
// 28. Every public API must have a documented compatibility contract.
// 29. Every production execution path must be verifiable.
// 30. No placeholder implementation may claim successful execution.

// ============================================================================
// COMPOSITION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foundational_runtime_types_are_exposed() {
        fn accepts_config(_: &RuntimeConfig) {}

        fn accepts_policy(_: &RuntimePolicy) {}

        fn accepts_effective_policy(_: &EffectiveRuntimePolicy) {}

        fn accepts_context(_: &RuntimeContext) {}

        fn accepts_lifecycle(_: &RuntimeLifecycle) {}

        fn accepts_state(_: &RuntimeState) {}

        fn accepts_builder(_: &RuntimeBuilder) {}

        fn accepts_instance(_: &RuntimeInstance) {}

        let _ = (
            accepts_config,
            accepts_policy,
            accepts_effective_policy,
            accepts_context,
            accepts_lifecycle,
            accepts_state,
            accepts_builder,
            accepts_instance,
        );
    }

    #[test]
    fn operational_error_api_is_exposed() {
        fn accepts_error(_: &ZurError) {}

        fn accepts_runtime_error(_: &RuntimeError) {}

        fn accepts_kind(_: RuntimeErrorKind) {}

        let _ = (
            accepts_error,
            accepts_runtime_error,
            accepts_kind,
        );
    }

    #[test]
    fn runtime_contract_modules_are_composed() {
        fn accepts_abi(_: &contracts::abi::RuntimeAbi) {}

        let _ = accepts_abi;
    }

    #[test]
    fn execution_modules_are_composed() {
        fn accepts_session_module(
            _: &execution::session::ExecutionSession,
        ) {
        }

        let _ = accepts_session_module;
    }

    #[test]
    fn computational_domain_modules_are_composed() {
        fn accepts_domain_registry(
            _: &domains::registry::DomainProviderRegistry,
        ) {
        }

        let _ = accepts_domain_registry;
    }

    #[test]
    fn hybrid_and_distributed_integrations_are_composed() {
        fn accepts_hybrid_adapter(
            _: &hybrid::HybridExecutionAdapter,
        ) {
        }

        fn accepts_distributed_adapter(
            _: &distributed::adapter::DistributedExecutionAdapter,
        ) {
        }

        let _ = (
            accepts_hybrid_adapter,
            accepts_distributed_adapter,
        );
    }

    #[test]
    fn optional_runtime_services_are_composed() {
        fn accepts_memory_module(
            _: &memory::MemoryAllocator,
        ) {
        }

        fn accepts_concurrency_module(
            _: &concurrency::TaskExecutor,
        ) {
        }

        fn accepts_resource_module(
            _: &resources::ResourceLeaseManager,
        ) {
        }

        let _ = (
            accepts_memory_module,
            accepts_concurrency_module,
            accepts_resource_module,
        );
    }
}
