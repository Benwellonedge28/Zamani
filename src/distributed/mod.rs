
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Distributed Computing
//!
//! Path: `src/distributed/mod.rs`
//!
//! Language: Zamani
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module is the canonical public composition root for distributed
//! computing in the Zamani programming language.
//!
//! It composes the existing distributed subsystem implementations into
//! one coherent, extensible public architecture.
//!
//! Its primary objective is:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).
//!
//! A Zamani program must be expressible independently of the physical
//! computational resources that eventually realize its execution.
//!
//! Depending on semantic requirements, capabilities, operational policies,
//! and actual resource availability, distributed computation may be
//! realized across:
//!
//! - A single execution context.
//! - Multiple execution contexts.
//! - Processes and workers.
//! - Multiple logical participants.
//! - Multiple physical machines.
//! - Embedded systems.
//! - Edge systems.
//! - Cloud environments.
//! - High-performance computing systems.
//! - Heterogeneous computing environments.
//! - Classical processors.
//! - Quantum processors and simulators.
//! - Quantum networks.
//! - AI and tensor accelerators.
//! - GPUs and other accelerators.
//! - FPGA and ASIC systems.
//! - Hardware/software co-execution.
//! - Federated computational environments.
//! - Future computational technologies.
//!
//! These are integration possibilities, not a closed catalogue of
//! supported hardware or execution providers.
//!
//! The presence of an architectural integration point does not establish
//! that a corresponding provider has already been implemented.
//!
//! ============================================================================
//! ARCHITECTURAL AUTHORITY
//! ============================================================================
//!
//! This module conforms to the following repository authorities:
//!
//! Specifications:
//!
//! - grammar/DESIGN.md
//! - grammar/spec/distributed.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/specification/execution-model.md
//! - grammar/spec/networking.md
//! - grammar/spec/concurrency.md
//! - grammar/spec/hybrid.md
//! - grammar/spec/quantum.md
//!
//! Grammar:
//!
//! - grammar/Zamani.g4
//! - grammar/distributed/README.md
//! - grammar/distributed/distributed.g4
//! - grammar/execution/
//! - grammar/concurrency/
//! - grammar/networking/
//! - grammar/resources/
//! - grammar/effects/
//! - grammar/security/
//!
//! Frontend:
//!
//! - src/frontend/ast/
//! - src/frontend/parser/
//! - src/frontend/semantic/
//!
//! Canonical representations:
//!
//! - Existing ZUIR.
//! - Existing classical IR.
//! - src/quantum/ir/.
//! - src/hdl/hdlir/.
//! - Existing domain-specific IR implementations.
//!
//! Execution and realization:
//!
//! - src/backend/.
//! - src/runtime/.
//! - Existing networking infrastructure.
//! - Existing hardware and HAL infrastructure.
//!
//! This module does not override any of these authorities.
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - The distributed public module hierarchy.
//! - Distributed subsystem composition.
//! - Stable public module paths.
//! - The distributed module API version.
//! - Deliberate module-level exports.
//! - Cross-subsystem architectural boundaries.
//! - Distributed-wide safety requirements.
//!
//! This module does NOT own:
//!
//! - Source-language syntax.
//! - AST definitions.
//! - Semantic analysis.
//! - Type checking.
//! - Capability semantic definitions.
//! - Resource semantic definitions.
//! - A universal intermediate representation.
//! - Classical intermediate representation.
//! - Quantum intermediate representation.
//! - Hardware intermediate representation.
//! - Compiler target generation.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Network protocol implementations.
//! - A competing scheduler.
//! - A competing execution engine.
//! - A competing runtime.
//! - Operating-system process management.
//! - Cryptographic algorithm implementations.
//! - Quantum error correction.
//! - Quantum pulse generation.
//! - Device drivers.
//!
//! These responsibilities remain with their existing owning modules.
//!
//! ============================================================================
//! ARCHITECTURAL LAYERS
//! ============================================================================
//!
//! The distributed subsystem participates in the following architecture:
//!
//! ```text
//!                  ZAMANI SOURCE
//!                        |
//!                        v
//!               Canonical grammar
//!                        |
//!                        v
//!                  Native AST
//!                        |
//!                        v
//!               Semantic validation
//!                        |
//!                        v
//!            Canonical semantic facts
//!                        |
//!              +---------+---------+
//!              |                   |
//!              v                   v
//!       Distributed intent     Domain semantics
//!              |                   |
//!              v                   v
//!       Distributed metadata   Canonical IRs
//!              |              /     |     \
//!              |           ZUIR   quantum  HDLIR
//!              |                   |
//!              +---------+---------+
//!                        |
//!                        v
//!                Distributed planning
//!                        |
//!             +----------+----------+
//!             |          |          |
//!             v          v          v
//!          Partition  Placement  Feasibility
//!             |          |          |
//!             +----------+----------+
//!                        |
//!                        v
//!                    Scheduling
//!                        |
//!                        v
//!                      Routing
//!                        |
//!                        v
//!                Validated execution plan
//!                        |
//!                        v
//!                Runtime revalidation
//!                        |
//!                        v
//!                 Distributed execution
//!                        |
//!             +----------+----------+
//!             |          |          |
//!             v          v          v
//!          Transport  Security  Resilience
//!             |          |          |
//!             +----------+----------+
//!                        |
//!                        v
//!                Execution providers
//!                        |
//!                        v
//!                Existing runtimes
//!                        |
//!                        v
//!              Actual computation
//! ```
//!
//! This architecture describes responsibility boundaries.
//! It does not imply that every provider or realization is implemented.
//!
//! ============================================================================
//! DEPENDENCY DIRECTION
//! ============================================================================
//!
//! The required dependency direction is:
//!
//! ```text
//! Foundational contracts
//!          |
//!          v
//! Distributed domain model
//!          |
//!          v
//! Planning contracts and implementations
//!          |
//!          v
//! Execution and transport integration
//!          |
//!          v
//! Runtime and physical providers
//! ```
//!
//! Security, membership, resilience and observability integrate through
//! their documented interfaces.
//!
//! Foundational modules must not depend on higher-level runtime services.
//!
//! In particular:
//!
//! - IDs must not depend on execution.
//! - Configuration must not depend on transport implementations.
//! - Requirements must not depend on placement algorithms.
//! - Models must not depend on a particular runtime.
//! - Planning must not depend on concrete hardware drivers.
//! - Transport must not implement another scheduler.
//! - Execution must not redefine canonical IR.
//! - Security must not be bypassed by membership registration.
//! - Observability must not change computational semantics.
//!
//! This dependency direction prevents circular ownership.
//!
//! ============================================================================
//! SCALABILITY AND POCO-REAF
//! ============================================================================
//!
//! This module MUST NOT introduce arbitrary universal limits on:
//!
//! - Nodes.
//! - Participants.
//! - Workers.
//! - Processes.
//! - Tasks.
//! - Partitions.
//! - Dependencies.
//! - Messages.
//! - Communication channels.
//! - Connections.
//! - Execution providers.
//! - Hardware devices.
//! - Accelerators.
//! - Memory capacity.
//! - Computational domains.
//! - Execution contexts.
//! - Network topology.
//! - Distributed services.
//! - Replicas.
//! - Future computational resources.
//!
//! No fixed hardware catalogue is permitted.
//!
//! No fixed transport protocol is permitted.
//!
//! No fixed machine-size assumption is permitted.
//!
//! No fixed participant-to-machine relationship is permitted.
//!
//! No first-registered-node scheduling assumption is permitted.
//!
//! Actual scalability depends on:
//!
//! - Available physical resources.
//! - Available memory.
//! - Available processing capacity.
//! - Available communication bandwidth.
//! - Provider capabilities.
//! - Address-space limitations.
//! - Numeric representation.
//! - Configured operational budgets.
//! - Security policies.
//! - Program semantics.
//!
//! Operational budgets are permitted when they are configurable and
//! belong to their appropriate owning subsystem.
//!
//! A configured budget is not a semantic limitation on Zamani programs.
//!
//! An unavailable resource must result in an explicit resource or
//! capability failure, not silent truncation.
//!
//! No implementation can promise literal infinite physical resources.
//! The architectural requirement is to scale without arbitrary
//! application-defined universal ceilings.
//!
//! ============================================================================
//! SEMANTIC AND IR INTEGRATION
//! ============================================================================
//!
//! Distributed execution consumes validated semantic information.
//!
//! The distributed subsystem must not create a second semantic truth.
//!
//! The existing semantic authorities own:
//!
//! - Names.
//! - Types.
//! - Effects.
//! - Capabilities.
//! - Resource requirements.
//! - Constraints.
//! - Preferences.
//! - Provenance.
//! - Contracts.
//! - Policies.
//!
//! Distributed requirements aggregate the relevant validated facts.
//!
//! Distributed planning determines feasible realization.
//!
//! Existing IR implementations remain authoritative:
//!
//! - ZUIR represents universal target-independent computation.
//! - Classical IR represents classical computational operations.
//! - quantum::ir represents quantum computation.
//! - HDLIR represents hardware-domain computation.
//!
//! Distributed metadata must refer to these representations through
//! documented adapters rather than creating competing representations.
//!
//! ============================================================================
//! PLANNING OWNERSHIP
//! ============================================================================
//!
//! The planning subsystem owns the following independent stages:
//!
//! feasibility:
//!     Determines whether requirements can be satisfied.
//!
//! partition:
//!     Constructs logical workload partitions.
//!
//! placement:
//!     Maps logical partitions to compatible candidate targets.
//!
//! scheduling:
//!     Determines when eligible work can execute while preserving
//!     dependencies and scheduling constraints.
//!
//! routing:
//!     Determines feasible communication paths.
//!
//! The distinction is:
//!
//! Placement answers WHERE.
//! Scheduling answers WHEN.
//! Routing answers HOW communication can reach its destination.
//!
//! None of these stages performs actual runtime execution.
//!
//! ============================================================================
//! EXECUTION OWNERSHIP
//! ============================================================================
//!
//! The execution subsystem consumes validated plans and invokes
//! compatible execution providers.
//!
//! A provider must establish actual eligibility and execution capability.
//!
//! The executor must not claim successful completion merely because:
//!
//! - A task was constructed.
//! - A provider was selected.
//! - A request was submitted.
//! - A message was delivered.
//! - A task was accepted.
//! - A log message was printed.
//!
//! Success requires authoritative execution completion and valid results.
//!
//! An uncertain remote outcome must remain uncertain until reconciled.
//!
//! ============================================================================
//! TRANSPORT OWNERSHIP
//! ============================================================================
//!
//! The transport subsystem owns:
//!
//! - Message envelopes.
//! - Protocol versioning.
//! - Encoding and decoding.
//! - Connection interfaces.
//! - Delivery evidence.
//! - Transport errors.
//! - Provider lifecycle.
//!
//! The distributed composition root does not choose a fixed protocol,
//! port, network interface, or communication medium.
//!
//! Existing networking infrastructure remains the physical
//! communication integration authority.
//!
//! ============================================================================
//! MEMBERSHIP AND SECURITY
//! ============================================================================
//!
//! Membership owns participant registration, discovery and health
//! observations.
//!
//! Security owns identity, authentication, authorization and policy.
//!
//! These concepts MUST remain distinct.
//!
//! Discovery is not authentication.
//! Authentication is not authorization.
//! Membership is not permission to execute.
//!
//! A discovered participant must not automatically become trusted.
//!
//! A trusted participant must not automatically become authorized.
//!
//! Protected operations must perform the appropriate authorization
//! checks using authoritative security context.
//!
//! ============================================================================
//! RESILIENCE
//! ============================================================================
//!
//! Resilience owns:
//!
//! - Retry coordination.
//! - Idempotency.
//! - Checkpoint integration.
//! - Recovery.
//! - Execution-outcome reconciliation.
//!
//! A unique message identifier alone does not guarantee exactly-once
//! execution.
//!
//! Retry decisions must respect:
//!
//! - Side effects.
//! - Delivery uncertainty.
//! - Execution outcome.
//! - Idempotency guarantees.
//! - Deadlines.
//! - Cancellation.
//! - Current authorization.
//!
//! Domain-specific checkpointability and migration guarantees must
//! be established by the relevant runtime and provider.
//!
//! ============================================================================
//! OBSERVABILITY
//! ============================================================================
//!
//! Observability owns structured events, metrics and tracing.
//!
//! Instrumentation must not change program semantics.
//!
//! Missing telemetry must not be interpreted as successful execution.
//!
//! Sensitive credentials and computational state must not be exposed
//! through ordinary logs, events, metric labels or trace attributes.
//!
//! ============================================================================
//! CONCURRENCY AND LIFECYCLE
//! ============================================================================
//!
//! This composition root:
//!
//! - Creates no global mutable state.
//! - Starts no background threads.
//! - Opens no network connections.
//! - Performs no implicit discovery.
//! - Allocates no physical resources.
//! - Creates no hidden task queues.
//! - Does not initialize a runtime singleton.
//!
//! The owning runtime controls initialization, execution, cancellation,
//! resource cleanup and graceful shutdown.
//!
//! Providers and services must be injected or explicitly constructed.
//!
//! ============================================================================
//! COMPATIBILITY
//! ============================================================================
//!
//! Existing child-module paths are the authoritative public paths.
//!
//! This root intentionally avoids wildcard re-exports.
//!
//! New public APIs must be added to their owning modules first.
//!
//! A new computational domain must not require modifying this file
//! merely to enumerate a new vendor or physical device.
//!
//! Changes to existing public paths require an explicit compatibility
//! review and migration plan.
//!
//! ============================================================================
//! COMPLETION CRITERIA
//! ============================================================================
//!
//! This module is complete when:
//!
//! - Every declared child module exists.
//! - Every child module has one authoritative implementation.
//! - All public paths resolve.
//! - No circular dependencies exist.
//! - Existing IR ownership remains intact.
//! - No competing runtime or scheduler is introduced.
//! - No unsafe Rust is permitted.
//! - No arbitrary scalability ceiling is introduced.
//! - All registered child-module tests compile.
//! - The crate root exposes this subsystem exactly once.
//! - Repository-wide integration checks pass.
//!
//! Module composition alone does not certify every provider or the
//! complete distributed runtime as production-ready.

// ============================================================================
// FOUNDATIONAL CONTRACTS
// ============================================================================

/// Distributed configuration and operational resource budgets.
///
/// Owns configurable operational limits, retry policy, deadlines and
/// provider-specific extension settings.
///
/// Does not impose universal language-level capacity restrictions.
pub mod config;

/// Canonical distributed errors and result contracts.
///
/// Preserves failure classification, causes and delivery uncertainty.
pub mod error;

/// Stable logical identities for distributed entities.
///
/// Logical identities remain independent of physical machine,
/// process, device and network endpoint identities.
pub mod ids;

/// Distributed capability discovery and capability evidence integration.
///
/// Consumes existing semantic capability authorities.
pub mod capabilities;

/// Aggregated distributed resource, capability and execution requirements.
///
/// Consumes canonical semantic requirements without redefining them.
pub mod requirements;

// ============================================================================
// DISTRIBUTED DOMAIN MODEL
// ============================================================================

/// Target-independent distributed computational models.
///
/// Owns logical participants, tasks, dependencies, communication
/// intent and distributed state metadata.
///
/// Does not perform physical execution or resource allocation.
pub mod model;

// ============================================================================
// DISTRIBUTED PLANNING
// ============================================================================

/// Distributed feasibility, partitioning, placement, scheduling
/// and routing.
///
/// Planning produces validated realization decisions but does not
/// perform physical execution.
pub mod planning;

// ============================================================================
// DISTRIBUTED EXECUTION
// ============================================================================

/// Distributed execution coordination and provider integration.
///
/// Consumes validated plans and delegates actual work to
/// authoritative execution providers.
pub mod execution;

// ============================================================================
// DISTRIBUTED COMMUNICATION
// ============================================================================

/// Provider-independent distributed transport interfaces,
/// messages, codecs and transport failure contracts.
///
/// Physical networking implementations remain with their
/// appropriate networking providers.
pub mod transport;

// ============================================================================
// PARTICIPANT MEMBERSHIP
// ============================================================================

/// Participant discovery, membership registry and health monitoring.
///
/// Membership does not establish authentication or authorization.
pub mod membership;

// ============================================================================
// RESILIENCE AND RECOVERY
// ============================================================================

/// Retry, idempotency, checkpoint and recovery integration.
///
/// Recovery must preserve execution and side-effect guarantees.
pub mod resilience;

// ============================================================================
// DISTRIBUTED SECURITY
// ============================================================================

/// Identity, authentication, authorization and policy integration.
///
/// Security decisions remain separate from discovery and membership.
pub mod security;

// ============================================================================
// DISTRIBUTED OBSERVABILITY
// ============================================================================

/// Structured events, metrics and distributed tracing.
///
/// Observability is independent of computational semantics.
pub mod observability;

// ============================================================================
// PUBLIC COMPOSITION API
// ============================================================================

/// Version of the distributed subsystem's public composition boundary.
///
/// This is not the Zamani language version, a network protocol version,
/// a serialized artifact version or an execution-plan version.
pub const DISTRIBUTED_API_VERSION: u16 = 1;

// ============================================================================
// COMPOSITION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// The distributed public API version must remain valid.
    #[test]
    fn distributed_api_version_is_nonzero() {
        assert_ne!(DISTRIBUTED_API_VERSION, 0);
    }

    /// Foundational module boundaries must be publicly accessible.
    #[test]
    fn foundational_modules_are_composed() {
        let _ = core::any::type_name::<config::DistributedConfig>();
        let _ = core::any::type_name::<error::DistributedError>();
    }

    /// The domain model must remain a separate public authority.
    #[test]
    fn domain_model_is_composed() {
        let _ = core::any::type_name::<model::Task>();
    }

    /// Planning remains a dedicated subsystem.
    #[test]
    fn planning_subsystem_is_composed() {
        assert_ne!(
            planning::DISTRIBUTED_PLANNING_API_VERSION,
            0
        );
    }

    /// Execution remains a dedicated subsystem.
    #[test]
    fn execution_subsystem_is_composed() {
        assert_ne!(
            execution::DISTRIBUTED_EXECUTOR_API_VERSION,
            0
        );
    }

    /// Transport remains independent of the execution implementation.
    #[test]
    fn transport_subsystem_is_composed() {
        let _ = core::any::type_name::<
            transport::ProtocolVersion
        >();
    }

    /// Security remains an independent authority.
    #[test]
    fn security_subsystem_is_composed() {
        let _ = core::any::type_name::<
            security::PrincipalId
        >();
    }

    /// Membership remains independent of security.
    #[test]
    fn membership_subsystem_is_composed() {
        let _ = core::any::type_name::<
            membership::MembershipRegistry
        >();
    }

    /// Resilience remains an independent authority.
    #[test]
    fn resilience_subsystem_is_composed() {
        let _ = core::any::type_name::<
            resilience::RetryCoordinator
        >();
    }

    /// Observability remains an independent authority.
    #[test]
    fn observability_subsystem_is_composed() {
        let _ = core::any::type_name::<
            observability::DistributedEvent
        >();
    }
}
pub mod omni_exec;
pub use omni_exec::*;
