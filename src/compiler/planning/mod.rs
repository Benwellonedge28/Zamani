
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — Planning
//!
//! Path: `src/compiler/planning/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! This module is the public composition boundary for compiler-side
//! target discovery, capability matching, feasibility assessment,
//! resource planning, workload partitioning, data movement planning,
//! routing planning, and abstract scheduling.
//!
//! It exposes the existing planning implementations. It does not
//! recreate their algorithms or establish competing authorities.
//!
//! The architecture supports:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! (POCO-REAF).
//!
//! A source program retains its semantic meaning while the compiler
//! evaluates compatible realization possibilities using explicit
//! requirements, capabilities, evidence, constraints, preferences,
//! hints, and resource information.
//!
//! A successful planning result is a proposal. It is not proof that
//! a physical resource has been reserved or that execution has begun.
//!
//! Literal infinite computation is not promised. Actual processing
//! is bounded by available implementation resources, representable
//! values, physical infrastructure, and explicitly configured
//! operational budgets.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Planning subsystem composition.
//! - Public planning module paths.
//! - Stable composition identity.
//! - Composition API version.
//! - Public re-exports of established subsystem contracts.
//! - Cross-subsystem architectural documentation.
//!
//! It does not own:
//!
//! - Source grammar or AST construction.
//! - Semantic analysis or semantic name resolution.
//! - Canonical capability definitions.
//! - Canonical resource requirement definitions.
//! - Canonical target descriptor definitions.
//! - Canonical IR definitions.
//! - Domain-specific IR definitions.
//! - Optimization algorithms.
//! - Backend implementations or code generation.
//! - Physical hardware discovery.
//! - Physical resource allocation or reservation.
//! - Physical placement.
//! - Network transport.
//! - Physical execution scheduling.
//! - Quantum physical routing.
//! - Quantum error correction.
//! - Runtime execution.
//!
//! These responsibilities remain with their authoritative subsystems.
//!
//! # Canonical authorities
//!
//! Source syntax:
//!     grammar/
//!
//! Semantic meaning and requirements:
//!     crate::frontend::semantic
//!
//! Canonical target descriptors and target identity:
//!     crate::compiler::configuration::target
//!
//! Canonical capability semantics:
//!     crate::frontend::semantic::capabilities
//!
//! Canonical resource semantics:
//!     crate::frontend::semantic::resources
//!
//! Canonical universal IR:
//!     crate::zuir
//!
//! Classical representation:
//!     crate::classical
//!
//! Quantum representation:
//!     crate::quantum::ir
//!
//! HDL representation:
//!     crate::hdl::hdlir
//!
//! AI and tensor computation:
//!     crate::ai
//!
//! Hybrid computation:
//!     crate::hybrid
//!
//! Distributed computation and partitioning:
//!     crate::distributed
//!
//! Backend implementations:
//!     crate::backend
//!
//! Physical resource acquisition and execution:
//!     crate::runtime
//!
//! # Planning subsystem responsibilities
//!
//! The public planning architecture is composed of independent modules.
//!
//! ## target_discovery
//!
//! Discovers and aggregates registered target descriptors.
//!
//! Discovery does not establish capability compatibility, resource
//! feasibility, authorization, physical availability, or execution.
//!
//! ## capability_matching
//!
//! Evaluates canonical semantic capability requirements against
//! explicitly supplied target capability evidence.
//!
//! Capability declarations and capability evidence are not interchangeable.
//!
//! ## feasibility
//!
//! Aggregates authoritative semantic, capability, resource, and
//! realization evidence to assess whether a proposed realization
//! is established, rejected, or unresolved.
//!
//! Unknown or incomplete evidence must never establish feasibility.
//!
//! ## resource_planning
//!
//! Evaluates canonical resource requirements against explicitly
//! supplied target-scoped resource evidence.
//!
//! It does not allocate or reserve resources.
//!
//! ## partitioning
//!
//! Partitions abstract workloads into target-compatible logical
//! segments while preserving canonical operation identities,
//! dependencies, candidate targets, and unresolved boundaries.
//!
//! It does not perform physical placement.
//!
//! ## data_movement
//!
//! Plans logical data transfers across partition boundaries using
//! explicit transfer requirements, placement evidence, and supplied
//! routing information.
//!
//! It does not serialize, allocate, transfer, or transport data.
//!
//! ## routing
//!
//! Coordinates abstract routes across partition boundaries using
//! supplied placement and topology evidence.
//!
//! It does not discover physical topology, reserve network resources,
//! or perform quantum physical routing.
//!
//! ## scheduling
//!
//! Produces dependency-aware abstract schedules from validated
//! partitioning results and explicit scheduling policies.
//!
//! It does not schedule physical devices or execute operations.
//!
//! # Planning order
//!
//! The usual dependency flow is:
//!
//!     Target discovery
//!           |
//!           v
//!     Capability matching
//!           |
//!           v
//!     Feasibility assessment
//!           |
//!           v
//!     Resource planning
//!           |
//!           v
//!     Workload partitioning
//!           |
//!           +--------------------+
//!           |                    |
//!           v                    v
//!     Data movement          Scheduling
//!           |
//!           v
//!        Routing
//!
//! This is a logical dependency guide, not a requirement that every
//! compilation request execute every planning stage in this exact order.
//!
//! Some stages may be inapplicable to a particular program.
//! Required stages must not be silently skipped.
//!
//! Data movement, routing, and scheduling may have additional ordering
//! dependencies established by their input contracts. Their owning
//! implementations remain authoritative for those dependencies.
//!
//! # Conceptual distinctions
//!
//! Planning integrations must preserve these separate concepts:
//!
//! Requirement:
//!     What the program needs.
//!
//! Capability:
//!     What a target or implementation can provide.
//!
//! Constraint:
//!     What an acceptable realization must obey.
//!
//! Preference:
//!     What is desirable but not mandatory.
//!
//! Hint:
//!     Optional implementation guidance.
//!
//! Feasibility:
//!     Whether supplied evidence establishes that a proposed
//!     realization satisfies applicable mandatory conditions.
//!
//! A preference must not become a mandatory requirement.
//! A hint must not become a correctness condition.
//! A capability declaration must not be treated as evidence.
//! Missing evidence must not be interpreted as unlimited capacity.
//! Unknown feasibility must not be reported as feasible.
//!
//! # Integration contract
//!
//! Inputs originate from authoritative subsystems:
//!
//! - Validated semantic requirements and constraints.
//! - Canonical capability requirements and evidence.
//! - Canonical resource requirements and resource evidence.
//! - Canonical target descriptors.
//! - Canonical operation identities and dependency graphs.
//! - Domain-specific IR and validated data relationships.
//! - Explicit placement and topology snapshots where applicable.
//!
//! Planning reports are consumed by downstream compiler stages.
//!
//! Downstream consumers must preserve the report's completeness,
//! evidence scope, unresolved conditions, provenance, and errors.
//!
//! A consumer must not reconstruct a planning decision from partial
//! report fields while discarding its failure or uncertainty status.
//!
//! # Scalability and POCO-REAF
//!
//! This composition module introduces no fixed maximum for:
//!
//! - Registered target providers.
//! - Discovered targets.
//! - Capability identities.
//! - Capability requirements.
//! - Resource dimensions.
//! - Resource quantities.
//! - Computing domains.
//! - Work items.
//! - Operations.
//! - Dependencies.
//! - Partitions.
//! - Transfers.
//! - Routes.
//! - Network nodes or links.
//! - Schedule steps.
//! - Processors or accelerators.
//! - Quantum resources.
//! - Memory capacity.
//! - Tensor dimensions.
//! - Future computing technologies.
//!
//! Collection sizes are governed by available implementation resources
//! and explicitly configured operational budgets.
//!
//! Operational budgets are request or policy limits. They must not
//! silently become language-level or universal hardware ceilings.
//!
//! The absence of a configured budget does not imply unlimited
//! physical capacity.
//!
//! Arithmetic overflow, allocation failure, incomplete evidence,
//! resource exhaustion, and infeasibility must remain explicit.
//!
//! # Determinism
//!
//! This module introduces no global mutable registry, random identity,
//! timestamp-dependent decision, or registration-order-dependent
//! selection policy.
//!
//! Each subsystem retains its own deterministic ordering contract.
//!
//! Composition must preserve those guarantees and must not silently
//! reorder or discard reports.
//!
//! # Concurrency
//!
//! This module does not create a global execution scheduler.
//!
//! Independent planning requests may use independent subsystem instances
//! and immutable evidence snapshots.
//!
//! Shared mutable state and compilation-session synchronization belong
//! to the compiler context and session.
//!
//! Physical execution concurrency belongs to the runtime.
//!
//! # Error handling
//!
//! Each subsystem retains its own error type.
//!
//! This module does not flatten all errors into one generic planning error.
//!
//! Callers must preserve distinctions between:
//!
//! - Discovery failure.
//! - Invalid target descriptor.
//! - Target identity conflict.
//! - Capability incompatibility.
//! - Unknown capability evidence.
//! - Invalid feasibility evidence.
//! - Incomplete feasibility assessment.
//! - Resource infeasibility.
//! - Unknown resource capacity.
//! - Incomplete partitioning.
//! - Unassigned work.
//! - Unresolved dependencies.
//! - Invalid transfer requirements.
//! - Missing placement information.
//! - Routing failure.
//! - Scheduling cycle.
//! - Operational budget exhaustion.
//! - Arithmetic overflow.
//! - Implementation resource exhaustion.
//!
//! # Security
//!
//! Planning reports and provider evidence are not automatically trusted.
//!
//! Authentication, authorization, provider trust, evidence provenance,
//! extension isolation, and artifact verification remain the responsibility
//! of the authoritative security and provider subsystems.
//!
//! No planning result grants permission to access a physical resource.
//!
//! # Compatibility
//!
//! Existing public module-qualified paths are retained.
//!
//! The additional public modules are additive composition exports.
//! Existing subsystem API versions remain independently governed by
//! their owning modules.
//!
//! Changes to a subsystem's public contract must be coordinated with
//! its consumers. A composition-module change must not silently alter
//! source semantics or domain IR contracts.

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================

/// Target discovery and aggregation.
///
/// Owns discovery provider registration, discovery requests, descriptor
/// validation coordination, deterministic aggregation, and discovery
/// diagnostics. Does not allocate physical devices.
pub mod target_discovery;

/// Canonical capability requirement matching.
///
/// Connects discovered targets to canonical semantic capability
/// requirements and explicitly supplied capability evidence.
/// Does not create a competing capability model.
pub mod capability_matching;

/// Evidence-based realization feasibility assessment.
///
/// Combines authoritative semantic, capability, resource, and
/// realization evidence. Unknown evidence never establishes feasibility.
pub mod feasibility;

/// Abstract resource feasibility planning.
///
/// Evaluates resource demands against explicitly supplied evidence.
/// Does not allocate or reserve physical resources.
pub mod resource_planning;

/// Heterogeneous logical workload partitioning.
///
/// Divides abstract workloads into target-compatible segments while
/// preserving operation identity, dependencies, and unresolved boundaries.
pub mod partitioning;

/// Abstract data movement planning.
///
/// Plans logical data movement across partition boundaries without
/// performing serialization, allocation, or physical transfers.
pub mod data_movement;

/// Abstract routing across partition boundaries.
///
/// Consumes supplied placement and topology evidence. Does not perform
/// physical placement, network transport, or quantum physical routing.
pub mod routing;

/// Dependency-aware abstract scheduling.
///
/// Constructs schedules from partitioning results and explicit policies.
/// Does not execute operations or schedule physical devices.
pub mod scheduling;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable architectural identity of compiler planning.
pub const PLANNING_MODULE_NAME: &str =
    "zamani.compiler.planning";

/// Public composition API version.
///
/// This identifies the composition boundary, not the language version,
/// semantic-model version, IR schema version, backend ABI, or runtime ABI.
///
/// Additive module exposure does not require changing this version.
/// Increment it when the composition contract changes incompatibly.
pub const PLANNING_API_VERSION: u16 = 1;

/// Returns the public planning composition API version.
#[must_use]
pub const fn api_version() -> u16 {
    PLANNING_API_VERSION
}

/// Returns the stable planning module identity.
#[must_use]
pub const fn module_name() -> &'static str {
    PLANNING_MODULE_NAME
}

// ============================================================================
// TARGET DISCOVERY PUBLIC API
// ============================================================================

pub use self::target_discovery::{
    DiscoveredTarget,
    DiscoveryError,
    DiscoveryPolicy,
    DiscoveryProviderId,
    DiscoveryReport,
    DiscoveryRequest,
    ProviderFailure,
    RejectedDescriptor,
    RejectionReason,
    TargetDiscovery,
    TargetDiscoveryProvider,
    TargetDiscoveryRegistry,
    TargetIdentityConflict,
    TARGET_DISCOVERY_API_VERSION,
    TARGET_DISCOVERY_MODULE_NAME,
};

// ============================================================================
// CAPABILITY MATCHING PUBLIC API
// ============================================================================

pub use self::capability_matching::{
    target_evidence_subject,
    CapabilityMatchError,
    CapabilityMatcher,
    CapabilityMatchReport,
    TargetCapabilityMatch,
    TargetMatchDisposition,
    CAPABILITY_MATCHING_API_VERSION,
    CAPABILITY_MATCHING_MODULE_NAME,
};

// ============================================================================
// FEASIBILITY PUBLIC API
// ============================================================================

/// Stable identity of the feasibility subsystem.
pub use self::feasibility::FEASIBILITY_MODULE_NAME;

/// Feasibility subsystem API version.
pub use self::feasibility::FEASIBILITY_API_VERSION;

// The detailed evidence, assessment, and report types remain available
// through `planning::feasibility`. This avoids creating a second
// feasibility type namespace at the planning root.

// ============================================================================
// RESOURCE PLANNING PUBLIC API
// ============================================================================

pub use self::resource_planning::{
    DemandDisposition,
    ResourceDemandClass,
    ResourceEvidenceError,
    ResourceEvidenceProvider,
    ResourceEvidenceSnapshot,
    ResourceDimensionAssessment,
    ResourcePlanner,
    ResourcePlanningError,
    ResourcePlanningReport,
    ResourcePolicyObligation,
    TargetResourceDisposition,
    TargetResourcePlan,
    RESOURCE_PLANNING_API_VERSION,
    RESOURCE_PLANNING_MODULE_NAME,
};

// ============================================================================
// PARTITIONING PUBLIC API
// ============================================================================

pub use self::partitioning::{
    PartitionBoundary,
    PartitionSegment,
    PartitionSegmentId,
    Partitioner,
    PartitioningDisposition,
    PartitioningError,
    PartitioningOptions,
    PartitioningReport,
    PartitioningWorkload,
    UnassignedReason,
    UnassignedWorkItem,
    UnresolvedDependency,
    WorkItem,
    PARTITIONING_API_VERSION,
    PARTITIONING_MODULE_NAME,
};

// ============================================================================
// DATA MOVEMENT PUBLIC API
// ============================================================================

/// Stable identity of the data-movement subsystem.
pub use self::data_movement::DATA_MOVEMENT_MODULE_NAME;

/// Data-movement subsystem API version.
pub use self::data_movement::DATA_MOVEMENT_API_VERSION;

// Detailed transfer requirements, placement evidence, movement plans,
// reports, and errors remain available through `planning::data_movement`.

// ============================================================================
// ROUTING PUBLIC API
// ============================================================================

/// Stable identity of the compiler routing subsystem.
pub use self::routing::COMPILER_ROUTING_MODULE_NAME;

/// Compiler routing subsystem API version.
pub use self::routing::COMPILER_ROUTING_API_VERSION;

// Detailed route policies, budgets, route reports, and errors remain
// available through `planning::routing`.

// ============================================================================
// SCHEDULING PUBLIC API
// ============================================================================

pub use self::scheduling::{
    ScheduleReport,
    ScheduleStep,
    ScheduledOperation,
    Scheduler,
    SchedulingBudget,
    SchedulingError,
    SchedulingOptions,
    SCHEDULING_API_VERSION,
    SCHEDULING_MODULE_NAME,
};

// ============================================================================
// COMPOSITION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planning_module_has_stable_identity() {
        assert_eq!(
            module_name(),
            "zamani.compiler.planning"
        );
    }

    #[test]
    fn planning_api_version_is_available() {
        assert_eq!(
            api_version(),
            PLANNING_API_VERSION
        );

        assert!(api_version() > 0);
    }

    #[test]
    fn all_planning_subsystems_are_exposed() {
        assert_eq!(
            TARGET_DISCOVERY_MODULE_NAME,
            "zamani.compiler.planning.target_discovery"
        );

        assert_eq!(
            CAPABILITY_MATCHING_MODULE_NAME,
            "zamani.compiler.planning.capability_matching"
        );

        assert_eq!(
            FEASIBILITY_MODULE_NAME,
            "zamani.compiler.planning.feasibility"
        );

        assert_eq!(
            RESOURCE_PLANNING_MODULE_NAME,
            "zamani.compiler.planning.resource_planning"
        );

        assert_eq!(
            PARTITIONING_MODULE_NAME,
            "zamani.compiler.planning.partitioning"
        );

        assert_eq!(
            DATA_MOVEMENT_MODULE_NAME,
            "zamani.compiler.planning.data_movement"
        );

        assert_eq!(
            COMPILER_ROUTING_MODULE_NAME,
            "zamani.compiler.planning.routing"
        );

        assert_eq!(
            SCHEDULING_MODULE_NAME,
            "zamani.compiler.planning.scheduling"
        );
    }

    #[test]
    fn subsystem_api_versions_are_available() {
        assert!(TARGET_DISCOVERY_API_VERSION > 0);
        assert!(CAPABILITY_MATCHING_API_VERSION > 0);
        assert!(FEASIBILITY_API_VERSION > 0);
        assert!(RESOURCE_PLANNING_API_VERSION > 0);
        assert!(PARTITIONING_API_VERSION > 0);
        assert!(DATA_MOVEMENT_API_VERSION > 0);
        assert!(COMPILER_ROUTING_API_VERSION > 0);
        assert!(SCHEDULING_API_VERSION > 0);
    }

    #[test]
    fn subsystem_module_names_are_stable() {
        assert_eq!(
            target_discovery::TARGET_DISCOVERY_MODULE_NAME,
            TARGET_DISCOVERY_MODULE_NAME
        );

        assert_eq!(
            capability_matching::CAPABILITY_MATCHING_MODULE_NAME,
            CAPABILITY_MATCHING_MODULE_NAME
        );

        assert_eq!(
            feasibility::FEASIBILITY_MODULE_NAME,
            FEASIBILITY_MODULE_NAME
        );

        assert_eq!(
            resource_planning::RESOURCE_PLANNING_MODULE_NAME,
            RESOURCE_PLANNING_MODULE_NAME
        );

        assert_eq!(
            partitioning::PARTITIONING_MODULE_NAME,
            PARTITIONING_MODULE_NAME
        );

        assert_eq!(
            data_movement::DATA_MOVEMENT_MODULE_NAME,
            DATA_MOVEMENT_MODULE_NAME
        );

        assert_eq!(
            routing::COMPILER_ROUTING_MODULE_NAME,
            COMPILER_ROUTING_MODULE_NAME
        );

        assert_eq!(
            scheduling::SCHEDULING_MODULE_NAME,
            SCHEDULING_MODULE_NAME
        );
    }
}
