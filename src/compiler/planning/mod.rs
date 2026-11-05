
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — Planning
//!
//! Path:
//!     src/compiler/planning/mod.rs
//!
//! Rust:
//!     Rust 1.97+
//!     Edition 2021
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module is the public composition boundary for compiler-side
//! target discovery, capability matching, resource feasibility,
//! heterogeneous workload partitioning, and abstract scheduling.
//!
//! Its responsibility is to expose and integrate the existing planning
//! subsystems without introducing duplicate implementations.
//!
//! The planning architecture supports Zamani's:
//
//!     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//!
//! (POCO-REAF).
//!
//! The same semantic program may be considered for different compatible
//! execution targets and different resource configurations without
//! requiring source-level rewriting.
//!
//! Target realization is conditional on:
//!
//! - Program semantics.
//! - Required capabilities.
//! - Available capability evidence.
//! - Resource requirements.
//! - Resource evidence.
//! - Target compatibility.
//! - Explicit constraints.
//! - Operational budgets.
//! - Physical and implementation feasibility.
//!
//! No claim of physically infinite computation is made.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//
//! - Planning subsystem composition.
//! - Stable planning module paths.
//! - Public planning API exports.
//! - Planning subsystem API version identification.
//! - Documented integration boundaries.
//!
//! This module does NOT own:
//
//! - Source-language grammar.
//! - AST construction.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership analysis.
//! - Canonical capability semantics.
//! - Canonical resource semantics.
//! - Canonical IR definitions.
//! - Optimization implementations.
//! - Backend implementations.
//! - Physical hardware discovery.
//! - Physical device allocation.
//! - Runtime execution.
//! - Physical execution scheduling.
//! - Quantum physical routing.
//! - Quantum error correction.
//! - Network transport.
//! - Hardware synthesis.
//!
//! Those responsibilities remain with their authoritative repository
//! subsystems.
//!
//! ============================================================================
//! SUBSYSTEM ARCHITECTURE
//! ============================================================================
//!
//! target_discovery
//!     Discovers and aggregates registered target descriptors.
//!
//! capability_matching
//!     Evaluates canonical semantic capability requirements against
//!     explicitly supplied target capability evidence.
//!
//! resource_planning
//!     Evaluates resource requirements against supplied resource evidence.
//!
//! partitioning
//!     Partitions abstract workloads while preserving operation identity,
//!     dependencies, candidate targets, and unresolved boundaries.
//!
//! scheduling
//!     Constructs dependency-aware abstract execution schedules from
//!     partitioning results.
//!
//! These modules retain independent implementation ownership.
//!
//! This parent module must not recreate their algorithms.
//!
//! ============================================================================
//! AUTHORITATIVE REPOSITORY DEPENDENCIES
//! ============================================================================
//!
//! Compiler configuration:
//!     crate::compiler::configuration
//!
//! Compiler analysis:
//!     crate::compiler::analysis
//!
//! Canonical semantic capabilities:
//!     crate::frontend::semantic::capabilities
//!
//! Canonical target descriptors:
//!     crate::compiler::configuration::target
//!
//! Distributed workload partitioning:
//!     crate::distributed::planning
//!
//! Canonical hybrid computation:
//!     crate::hybrid
//!
//! Canonical quantum IR:
//!     crate::quantum::ir
//!
//! Canonical classical computation:
//!     crate::classical
//!
//! Canonical universal IR:
//!     crate::zuir
//!
//! Compiler optimization:
//!     crate::compiler::optimization
//!
//! Backend:
//!     crate::backend
//!
//! Runtime:
//!     crate::runtime
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! Target discovery produces DiscoveryReport.
//!
//! Capability matching consumes discovered target information and
//! canonical semantic capability requirements.
//!
//! Resource planning consumes capability-matching results and explicitly
//! supplied resource evidence.
//!
//! Partitioning consumes abstract workloads and target compatibility
//! evidence.
//!
//! Scheduling consumes partitioning results and their dependencies.
//!
//! Downstream compiler stages consume planning reports rather than
//! reconstructing their internal decisions.
//!
//! Every incomplete, incompatible, infeasible, or unresolved result must
//! remain distinguishable from a successful realization.
//!
//! A partial inventory must never be silently presented as a complete
//! inventory.
//!
//! A missing capability must never be silently treated as available.
//!
//! An unknown resource quantity must never be treated as unlimited.
//!
//! A successful abstract plan must never be represented as proof that
//! physical execution or allocation has succeeded.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! This module introduces no fixed maximum for:
//!
//! - Registered targets.
//! - Discovery providers.
//! - Computing domains.
//! - CPUs.
//! - CPU cores.
//! - Threads.
//! - GPUs.
//! - FPGAs.
//! - ASICs.
//! - QPUs.
//! - Accelerators.
//! - Processing elements.
//! - Distributed nodes.
//! - Work items.
//! - Operations.
//! - Dependencies.
//! - Partitions.
//! - Schedule steps.
//! - Resources.
//! - Memory.
//! - Tensor dimensions.
//! - Quantum resources.
//! - Hardware resources.
//!
//! Collection capacity is determined by available implementation
//! resources and explicitly configured operational budgets.
//!
//! An absent operational budget does not imply infinite physical capacity.
//!
//! The compiler must report allocation failures, budget exhaustion,
//! arithmetic overflow, and infeasibility explicitly.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! The composed public API preserves the determinism guarantees of the
//! underlying planning implementations.
//!
//! Stable identifiers and deterministic ordering must be preserved.
//!
//! Provider registration order must not silently determine target identity.
//!
//! Equivalent inputs and equivalent evidence must produce equivalent
//! planning results wherever the underlying contracts guarantee
//! deterministic behavior.
//!
//! ============================================================================
//! CONCURRENCY
//! ============================================================================
//!
//! This module does not create a global scheduler or global mutable
//! target registry.
//!
//! Concurrency policies belong to explicit planning requests and their
//! owning subsystems.
//!
//! Physical execution concurrency belongs to the runtime.
//!
//! ============================================================================
//! ERROR HANDLING
//! ============================================================================
//!
//! The underlying modules retain their specific error types.
//!
//! This module does not collapse all failures into a single ambiguous
//! planning error.
//!
//! Callers should preserve:
//!
//! - Discovery failures.
//! - Invalid target descriptors.
//! - Target identity conflicts.
//! - Capability incompatibility.
//! - Unresolved capability evidence.
//! - Resource infeasibility.
//! - Unresolved resource evidence.
//! - Incomplete partitioning.
//! - Unresolved dependencies.
//! - Scheduling cycles.
//! - Operational budget exhaustion.
//! - Implementation resource failures.
//!
//! ============================================================================
//! PUBLIC API STABILITY
//! ============================================================================
//!
//! Existing module-qualified APIs remain available.
//!
//! The root-level re-exports provide a convenient public interface.
//!
//! Implementation details that are not part of the documented public
//! contract should continue to be accessed through their owning modules.
//!
//! Changes to individual subsystem contracts must be coordinated with
//! their consumers and must not silently change program semantics.

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================

/// Target discovery and aggregation.
///
/// Owns the provider registry, discovery request, discovery policy,
/// target aggregation, and discovery diagnostics.
///
/// Does not allocate physical devices.
pub mod target_discovery;

/// Canonical capability requirement matching.
///
/// Connects discovered targets to canonical semantic capability
/// requirements and explicitly supplied capability evidence.
///
/// Does not introduce a competing capability model or compatibility engine.
pub mod capability_matching;

/// Abstract resource feasibility planning.
///
/// Evaluates resource demands against explicitly supplied resource
/// evidence and canonical resource contracts.
///
/// Does not allocate physical resources.
pub mod resource_planning;

/// Heterogeneous workload partitioning.
///
/// Divides abstract workloads into target-compatible segments while
/// preserving dependencies and reporting incomplete assignments.
///
/// Does not perform physical device placement.
pub mod partitioning;

/// Dependency-aware abstract scheduling.
///
/// Constructs schedules from partitioning information and explicit
/// scheduling policies.
///
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
/// This version identifies the composition interface. It is independent
/// of the individual subsystem API versions.
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
// INTEGRATION TESTS
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

        assert_eq!(
            api_version(),
            1
        );
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
            RESOURCE_PLANNING_MODULE_NAME,
            "zamani.compiler.planning.resource_planning"
        );

        assert_eq!(
            PARTITIONING_MODULE_NAME,
            "zamani.compiler.planning.partitioning"
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
        assert!(RESOURCE_PLANNING_API_VERSION > 0);
        assert!(PARTITIONING_API_VERSION > 0);
        assert!(SCHEDULING_API_VERSION > 0);
    }

    #[test]
    fn planning_subsystems_have_stable_module_identity() {
        assert_eq!(
            target_discovery::TARGET_DISCOVERY_MODULE_NAME,
            TARGET_DISCOVERY_MODULE_NAME
        );

        assert_eq!(
            capability_matching::CAPABILITY_MATCHING_MODULE_NAME,
            CAPABILITY_MATCHING_MODULE_NAME
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
            scheduling::SCHEDULING_MODULE_NAME,
            SCHEDULING_MODULE_NAME
        );
    }
}
