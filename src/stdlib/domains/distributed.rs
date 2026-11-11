
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Standard Library — Distributed Computing
//!
//! Path: `src/stdlib/domains/distributed.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the stable standard-library entry point for distributed
//! computing in Zamani.
//!
//! This module exposes the canonical distributed subsystem implemented
//! under `crate::distributed`. It does not implement a second distributed
//! execution engine or redefine distributed domain concepts.
//!
//! The portability objective is:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).
//!
//! A distributed program expresses portable computational intent.
//! The compiler, planner, runtime and execution providers determine
//! how that intent is realized using available resources.
//!
//! ## Architectural ownership
//!
//! `crate::distributed` remains the authoritative implementation for:
//!
//! - Distributed configuration and operational budgets.
//! - Distributed errors and delivery-outcome classification.
//! - Logical distributed identities.
//! - Capability discovery and evidence integration.
//! - Aggregated distributed requirements.
//! - Participants, tasks, dependencies and communication models.
//! - Feasibility and distributed planning.
//! - Partitioning, placement, scheduling and routing.
//! - Distributed execution and execution lifecycle.
//! - Transport, message envelopes and codecs.
//! - Participant membership and health.
//! - Retry, checkpointing and recovery integration.
//! - Distributed identity, authentication and authorization.
//! - Distributed observability.
//!
//! This standard-library facade owns only:
//!
//! - The stable standard-library import path.
//! - Deliberate re-exports of canonical distributed APIs.
//! - Documentation of the standard-library integration boundary.
//!
//! ## Responsibilities that remain outside this module
//!
//! This module MUST NOT implement:
//!
//! - A second AST or semantic model.
//! - A second distributed task or participant model.
//! - A second distributed identity allocator.
//! - A second resource or capability model.
//! - A second universal intermediate representation.
//! - A second distributed planner.
//! - A second partitioning, placement, scheduling or routing algorithm.
//! - A second transport or networking implementation.
//! - A second execution engine or runtime.
//! - Hardware discovery or physical resource allocation.
//! - Device drivers or hardware control.
//! - Cryptographic algorithm implementations.
//! - Quantum error correction or quantum execution.
//! - Global mutable state or runtime singletons.
//! - Implicit network connections or participant discovery.
//!
//! These responsibilities remain with their existing repository
//! authorities.
//!
//! ## Canonical integration
//!
//! The dependency direction is:
//!
//! ```text
//! Zamani source and grammar
//!           |
//!           v
//! Canonical frontend and semantic analysis
//!           |
//!           v
//! Canonical semantic facts and domain IRs
//!           |
//!           v
//! crate::distributed
//!           |
//!           v
//! Standard-library distributed facade
//!           |
//!           v
//! Public Zamani standard-library API
//! ```
//!
//! The facade re-exports the canonical implementation. It does not
//! transform, copy, wrap or independently validate canonical domain
//! values.
//!
//! ## POCO-REAF and scalability
//!
//! This module introduces no arbitrary universal limits on:
//!
//! - Participants, machines, processes or workers.
//! - Tasks, dependencies, partitions or channels.
//! - Messages, connections or communication routes.
//! - Replicas, services or execution contexts.
//! - Providers, devices or computational domains.
//! - Memory, storage or network capacity.
//!
//! No finite implementation can promise literal infinite physical
//! resources. Actual execution is bounded by available resources,
//! address-space representation, provider capabilities, semantic
//! requirements and explicitly configured operational budgets.
//!
//! Such limits must be reported by their owning subsystem. This facade
//! must not introduce additional global ceilings or silently truncate
//! distributed workloads.
//!
//! ## Semantic distinctions
//!
//! The standard library does not redefine these concepts:
//!
//! - Requirement: a condition that must be satisfied.
//! - Capability: an ability supported by a realization.
//! - Preference: a non-mandatory optimization preference.
//! - Constraint: a restriction on valid realizations.
//! - Hint: non-binding implementation guidance.
//! - Feasibility: an evidence-based assessment of whether a realization
//!   can satisfy the applicable conditions.
//!
//! Their authoritative definitions remain in the semantic and
//! distributed subsystems.
//!
//! ## Integration with other computing domains
//!
//! Distributed computation may coordinate classical, quantum, HDL,
//! AI, tensor, hybrid, embedded and future computational domains.
//!
//! This module does not enumerate those domains or assume that every
//! provider exists. Domain-specific semantics and IRs remain owned by
//! their respective subsystems.
//!
//! ## Lifecycle
//!
//! This module performs no initialization or shutdown.
//!
//! It creates no threads, connections, registries, tasks or global
//! state. The caller explicitly constructs and manages the services
//! supplied by the canonical distributed subsystem.
//!
//! ## Compatibility
//!
//! Existing canonical paths remain available through `canonical` and
//! the explicit module re-exports below.
//!
//! New distributed functionality must first be implemented in its
//! canonical owning module. It may then be exposed here through a
//! deliberate re-export.
//!
//! This module must not use wildcard re-exports because doing so would
//! make the standard-library public API change implicitly whenever
//! internal canonical modules change.
//!
//! ## Completion criteria
//!
//! This facade is complete when:
//!
//! - The canonical distributed module is publicly accessible.
//! - Every re-export resolves to the canonical implementation.
//! - The standard-library module hierarchy exposes this file.
//! - The crate's frontend and distributed dependencies compile.
//! - No duplicate distributed implementation is introduced.
//! - The facade's tests and repository integration checks pass.

// ============================================================================
// CANONICAL DISTRIBUTED API
// ============================================================================

/// The authoritative distributed subsystem.
///
/// Use this namespace when an API is not explicitly re-exported below.
/// All types and implementations remain owned by `crate::distributed`.
pub use crate::distributed as canonical;

/// Version of the canonical distributed composition API.
pub use crate::distributed::DISTRIBUTED_API_VERSION;

// ============================================================================
// FOUNDATIONAL CONTRACTS
// ============================================================================

/// Distributed capability discovery and evidence integration.
pub use crate::distributed::capabilities;

/// Distributed operational configuration and budgets.
pub use crate::distributed::config;

/// Canonical distributed errors and result contracts.
pub use crate::distributed::error;

/// Stable logical identities for distributed entities.
pub use crate::distributed::ids;

/// Aggregated distributed requirements.
pub use crate::distributed::requirements;

// ============================================================================
// DISTRIBUTED DOMAIN MODEL
// ============================================================================

/// Target-independent distributed participants, tasks, dependencies,
/// communication intent and state descriptors.
pub use crate::distributed::model;

// ============================================================================
// PLANNING
// ============================================================================

/// Distributed feasibility, partitioning, placement, scheduling
/// and routing.
pub use crate::distributed::planning;

// ============================================================================
// EXECUTION AND COMMUNICATION
// ============================================================================

/// Distributed execution coordination and provider integration.
pub use crate::distributed::execution;

/// Protocol-independent distributed communication and transport.
pub use crate::distributed::transport;

// ============================================================================
// MEMBERSHIP AND RESILIENCE
// ============================================================================

/// Participant discovery, membership and health integration.
pub use crate::distributed::membership;

/// Retry, checkpointing, recovery and execution-outcome reconciliation.
pub use crate::distributed::resilience;

// ============================================================================
// SECURITY AND OBSERVABILITY
// ============================================================================

/// Distributed identity, authentication, authorization and policy.
pub use crate::distributed::security;

/// Distributed events, metrics and tracing.
pub use crate::distributed::observability;

// ============================================================================
// FACADE TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// The standard-library version is the canonical distributed API version.
    #[test]
    fn api_version_matches_canonical_authority() {
        assert_eq!(
            DISTRIBUTED_API_VERSION,
            canonical::DISTRIBUTED_API_VERSION
        );
    }

    /// The facade exposes the canonical distributed error type.
    #[test]
    fn error_type_is_the_canonical_type() {
        let facade_type =
            core::any::type_name::<error::DistributedError>();

        let canonical_type =
            core::any::type_name::<canonical::error::DistributedError>();

        assert_eq!(facade_type, canonical_type);
    }

    /// The facade exposes the canonical distributed task model.
    #[test]
    fn task_type_is_the_canonical_type() {
        let facade_type =
            core::any::type_name::<model::Task>();

        let canonical_type =
            core::any::type_name::<canonical::model::Task>();

        assert_eq!(facade_type, canonical_type);
    }

    /// Planning is exposed through the canonical planning subsystem.
    #[test]
    fn planning_api_is_reachable() {
        assert_eq!(
            planning::DISTRIBUTED_PLANNING_API_VERSION,
            canonical::planning::DISTRIBUTED_PLANNING_API_VERSION
        );
    }

    /// Execution is exposed through the canonical execution subsystem.
    #[test]
    fn execution_api_is_reachable() {
        assert_eq!(
            execution::DISTRIBUTED_EXECUTOR_API_VERSION,
            canonical::execution::DISTRIBUTED_EXECUTOR_API_VERSION
        );
    }

    /// Transport remains the canonical transport implementation.
    #[test]
    fn transport_api_is_reachable() {
        let facade_type =
            core::any::type_name::<transport::ProtocolVersion>();

        let canonical_type =
            core::any::type_name::<canonical::transport::ProtocolVersion>();

        assert_eq!(facade_type, canonical_type);
    }

    /// Security remains the canonical distributed security implementation.
    #[test]
    fn security_api_is_reachable() {
        let facade_type =
            core::any::type_name::<security::PrincipalId>();

        let canonical_type =
            core::any::type_name::<canonical::security::PrincipalId>();

        assert_eq!(facade_type, canonical_type);
    }
}
