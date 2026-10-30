
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Distributed Planning
//!
//! Path: `src/distributed/planning/mod.rs`
//! Language: Zamani
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the public composition boundary for distributed execution
//! planning in Zamani.
//!
//! It exposes the existing planning stages responsible for:
//!
//! 1. Feasibility analysis.
//! 2. Logical workload partitioning.
//! 3. Execution-target placement.
//! 4. Dependency-aware execution scheduling.
//! 5. Communication-route planning.
//!
//! It does not introduce another planning algorithm or duplicate the
//! responsibilities of the existing planning modules.
//!
//! ## POCO-REAF
//!
//! The distributed planning architecture supports:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//!
//! A logical workload must remain independent of the physical realization
//! selected for its execution.
//!
//! Planning may operate across:
//!
//! - A single execution context.
//! - Multiple processes and workers.
//! - Distributed machines and clusters.
//! - Edge and cloud environments.
//! - Classical and quantum execution providers.
//! - AI and tensor accelerators.
//! - FPGA and ASIC execution environments.
//! - Heterogeneous and hybrid computational systems.
//! - Future execution domains introduced through compatible providers.
//!
//! Support for a particular realization depends on its actual capabilities,
//! available resources, applicable policies, and semantic requirements.
//!
//! This module does not promise physically infinite capacity. It imposes no
//! arbitrary universal maximum on the number of participants, work units,
//! partitions, targets, resources, or communication routes.
//!
//! Operational budgets belong to the relevant planning stage or its caller.
//! Actual memory, address-space, numeric representation, and available
//! computational resources remain finite.
//!
//! ## Architectural ownership
//!
//! The planning subsystem consumes existing repository authorities.
//!
//! | Concern | Authoritative owner |
//! |---------|---------------------|
//! | Distributed semantics | grammar/spec/distributed.md |
//! | Source grammar | grammar/distributed/ |
//! | Execution intent | grammar/execution/ |
//! | Native AST | frontend/ast/ |
//! | Semantic validation | frontend/semantic/ |
//! | Semantic capabilities | frontend/semantic/capabilities/ |
//! | Semantic resources | frontend/semantic/resources/ |
//! | Distributed requirements | distributed/requirements.rs |
//! | Capability discovery | distributed/capabilities.rs |
//! | Distributed task model | distributed/model/ |
//! | Universal IR | Existing ZUIR |
//! | Classical IR | Existing classical IR |
//! | Quantum IR | quantum::ir |
//! | Hardware IR | Existing HDLIR |
//! | Backend generation | backend/ |
//! | Physical resource discovery | Existing resource/HAL infrastructure |
//! | Networking and transport | Existing networking infrastructure |
//! | Runtime execution | runtime/ |
//!
//! This module MUST NOT create competing definitions of these authorities.
//!
//! ## Planning responsibilities
//!
//! The planning stages have distinct responsibilities.
//!
//! ### Feasibility
//!
//! `feasibility` determines whether the supplied semantic requirements can
//! be satisfied using authoritative capability evidence, resource
//! observations, and directive evaluators.
//!
//! It does not allocate resources or execute programs.
//!
//! ### Partitioning
//!
//! `partition` constructs logical partitions from a validated workload graph.
//!
//! It preserves work-unit identity and dependency relationships.
//!
//! A partition is a logical grouping, not a physical machine, device,
//! worker, or network endpoint.
//!
//! ### Placement
//!
//! `placement` maps logical partitions to supplied candidate targets using
//! an injected target evaluator.
//!
//! Target snapshots are observations, not reservations.
//!
//! Placement does not implement hardware allocation or network routing.
//!
//! ### Scheduling
//!
//! `scheduling` establishes a valid dependency-respecting execution
//! schedule using supplied resource capacities and scheduling profiles.
//!
//! Scheduling does not perform runtime dispatch or physical allocation.
//!
//! ### Routing
//!
//! `routing` computes communication paths from a supplied topology snapshot
//! using provider-owned edge evaluation.
//!
//! A route is a planning result, not a transport connection or reservation.
//!
//! ## Canonical planning sequence
//!
//! The intended integration sequence is:
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! Canonical grammar and frontend
//!       |
//!       v
//! Native AST
//!       |
//!       v
//! Semantic analysis and validation
//!       |
//!       v
//! Canonical semantic representation
//!       |
//!       +-------------------------------+
//!       |                               |
//!       v                               v
//! Distributed requirements        Domain IRs
//!       |                       ZUIR / ZCIR / quantum::ir / HDLIR
//!       |                               |
//!       +---------------+---------------+
//!                       |
//!                       v
//!                Feasibility
//!                       |
//!                       v
//!                Workload graph
//!                       |
//!                       v
//!                 Partitioning
//!                       |
//!                       v
//!                   Placement
//!                       |
//!                       v
//!                  Scheduling
//!                       |
//!                       v
//!                    Routing
//!                       |
//!                       v
//!              Plan validation
//!                       |
//!                       v
//!             Runtime revalidation
//!                       |
//!                       v
//!                Execution
//! ```
//!
//! This sequence describes the intended integration contract. Individual
//! stages remain independently callable when their inputs are available.
//!
//! The order of scheduling and routing may be coordinated or iterated by
//! a higher-level planning coordinator when communication costs affect
//! scheduling decisions. This module does not impose an artificial
//! single-pass optimization strategy.
//!
//! ## Cross-stage invariants
//!
//! Every integration using these modules MUST preserve the following:
//!
//! 1. Semantic requirements cannot be silently weakened.
//! 2. Unknown mandatory capability evidence is not proof of support.
//! 3. Unknown mandatory resource evidence is not proof of feasibility.
//! 4. Logical task identities survive partitioning.
//! 5. Dependency relationships survive partition boundaries.
//! 6. Physical placement does not change program semantics.
//! 7. Scheduling preserves mandatory dependency ordering.
//! 8. Routing does not silently invent network connectivity.
//! 9. Planning decisions are based on identifiable input snapshots.
//! 10. Stale resource, capability, or topology observations must be
//!     revalidated before execution.
//! 11. Planning failures must not be reported as successful execution.
//! 12. Operational budget exhaustion must not produce a falsely complete
//!     planning result.
//!
//! ## Error ownership
//!
//! Errors remain owned by their respective planning stages.
//!
//! - Feasibility errors belong to `feasibility`.
//! - Partitioning errors belong to `partition`.
//! - Placement errors belong to `placement`.
//! - Scheduling errors belong to `scheduling`.
//! - Routing errors belong to `routing`.
//!
//! This composition module does not flatten distinct failure conditions
//! into a generic success/failure result.
//!
//! A higher-level coordinator may aggregate these errors while preserving
//! their original stage and diagnostic information.
//!
//! ## Determinism
//!
//! Determinism is governed by the individual planning-stage contracts.
//!
//! For reproducible planning, callers must supply:
//!
//! - Stable logical identifiers.
//! - Consistent input snapshots.
//! - Deterministic evaluator implementations.
//! - Explicit planning policies.
//! - Consistent cost and resource units.
//!
//! This module does not introduce global mutable state or hidden
//! environment-dependent decisions.
//!
//! ## Scalability
//!
//! This module MUST NOT introduce:
//!
//! - Fixed node counts.
//! - Fixed worker counts.
//! - Fixed task counts.
//! - Fixed partition counts.
//! - Fixed hardware catalogues.
//! - Fixed accelerator counts.
//! - Fixed memory capacities.
//! - Fixed quantum-device counts.
//! - Fixed network sizes.
//! - Fixed routing hop limits.
//! - Fixed universal retry counts.
//!
//! Optional operational budgets remain configurable through the existing
//! planning-stage interfaces.
//!
//! An implementation must report actual arithmetic overflow, resource
//! exhaustion, unavailable capabilities, and budget exhaustion rather
//! than silently truncating a workload.
//!
//! ## Extension contract
//!
//! New planning strategies should be introduced in their owning stage or
//! through an explicitly defined provider/policy interface.
//!
//! New computational domains must integrate through their existing
//! semantic, IR, resource, capability, backend, and runtime authorities.
//!
//! Adding a new domain must not require rewriting this composition module
//! merely to enumerate another hardware vendor or execution technology.
//!
//! ## Completion criteria
//!
//! This module is complete when:
//!
//! - Every declared child module exists.
//! - Every child module compiles independently.
//! - The public module paths remain stable.
//! - No competing planning implementation is introduced.
//! - The module has no dependency on concrete hardware or transport.
//! - The canonical planning ownership boundaries are preserved.
//! - The distributed crate root exports this module.
//! - Repository integration tests verify the planning stages.
//!
//! This module uses safe Rust only.

/// Evaluates distributed resource and capability feasibility.
///
/// This module consumes canonical distributed requirements and
/// authoritative semantic capability/resource evidence.
///
/// It does not perform physical allocation.
pub mod feasibility;

/// Constructs logical workload partitions.
///
/// This module preserves work-unit identity and dependency relationships
/// without selecting physical execution targets.
pub mod partition;

/// Assigns logical partitions to supplied candidate execution targets.
///
/// This module delegates resource, capability, policy, and domain-specific
/// eligibility decisions to injected evaluators.
pub mod placement;

/// Produces dependency-aware execution schedules.
///
/// This module consumes workload graphs and caller-supplied scheduling
/// profiles and resource-capacity observations.
pub mod scheduling;

/// Computes provider-neutral communication routes.
///
/// This module consumes caller-supplied topology snapshots and injected
/// edge evaluators. It does not perform network transport.
pub mod routing;

// -------------------------------------------------------------------------
// Stable public API version information
// -------------------------------------------------------------------------

/// Public API version for the distributed planning composition boundary.
///
/// This version describes the module composition API. It is not a
/// serialized-plan version or a network protocol version.
pub const DISTRIBUTED_PLANNING_API_VERSION: u16 = 1;

/// Public planning-stage identifiers.
///
/// These identifiers are descriptive metadata. They do not determine
/// whether a stage is enabled or supported by a particular provider.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum PlanningStage {
    /// Requirement and resource feasibility evaluation.
    Feasibility,

    /// Logical workload partitioning.
    Partitioning,

    /// Logical partition placement.
    Placement,

    /// Dependency-aware execution scheduling.
    Scheduling,

    /// Communication-route planning.
    Routing,
}

impl PlanningStage {
    /// Returns the stable textual identifier of the planning stage.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Feasibility => "feasibility",
            Self::Partitioning => "partitioning",
            Self::Placement => "placement",
            Self::Scheduling => "scheduling",
            Self::Routing => "routing",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planning_api_version_is_nonzero() {
        assert_ne!(DISTRIBUTED_PLANNING_API_VERSION, 0);
    }

    #[test]
    fn planning_stage_identifiers_are_stable() {
        assert_eq!(PlanningStage::Feasibility.as_str(), "feasibility");
        assert_eq!(PlanningStage::Partitioning.as_str(), "partitioning");
        assert_eq!(PlanningStage::Placement.as_str(), "placement");
        assert_eq!(PlanningStage::Scheduling.as_str(), "scheduling");
        assert_eq!(PlanningStage::Routing.as_str(), "routing");
    }

    #[test]
    fn planning_stages_have_deterministic_ordering() {
        assert!(PlanningStage::Feasibility < PlanningStage::Partitioning);
        assert!(PlanningStage::Partitioning < PlanningStage::Placement);
        assert!(PlanningStage::Placement < PlanningStage::Scheduling);
        assert!(PlanningStage::Scheduling < PlanningStage::Routing);
    }
}
