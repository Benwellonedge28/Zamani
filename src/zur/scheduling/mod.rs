
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Scheduling
//!
//! Path: `src/zur/scheduling/mod.rs`
//!
//! Language: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! # Purpose
//!
//! Provides the public composition boundary for operational scheduling
//! within the Zamani Universal Runtime (ZUR).
//!
//! This module integrates:
//!
//! - Declarative scheduling policies.
//! - Local operational scheduling.
//! - The canonical concurrency execution interface.
//! - Runtime resource-management infrastructure.
//! - Runtime configuration and policy infrastructure.
//!
//! Its purpose is to provide a scalable, extensible and domain-neutral
//! operational scheduling foundation.
//!
//! The architecture supports Zamani's:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//!
//! (POCO-REAF).
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Public scheduling module composition.
//! - Stable scheduling module paths.
//! - Public scheduling API re-exports.
//! - Scheduling integration documentation.
//!
//! Its child modules retain their individual responsibilities.
//!
//! ## policy
//!
//! `policy.rs` owns:
//!
//! - Scheduling algorithm declarations.
//! - Scheduling objectives.
//! - Objective ordering.
//! - Tie-breaking policies.
//! - Fairness policies.
//! - Starvation-prevention policies.
//! - Preemption policies.
//! - Extensible scheduling policy values.
//! - Scheduling policy validation.
//!
//! Policy declarations express operational scheduling preferences
//! and requirements. They do not execute scheduling algorithms.
//!
//! ## local
//!
//! `local.rs` owns:
//!
//! - LocalScheduler.
//! - LocalSchedulerConfig.
//! - Local FIFO job dispatch.
//! - Local worker lifecycle.
//! - Optional queue-capacity enforcement.
//! - Graceful shutdown.
//! - Worker-level panic containment.
//!
//! The local scheduler implements the canonical TaskDispatcher
//! interface from `zur::concurrency::executor`.
//!
//! It does not own task lifecycle or task completion.
//!
//! # Existing architectural authorities
//!
//! This module integrates with the following existing subsystems.
//!
//! ## Concurrency
//!
//! `zur::concurrency::task`:
//!
//! - Task identity.
//! - Task lifecycle.
//! - Task status.
//! - Task completion.
//! - Task outcomes.
//!
//! `zur::concurrency::executor`:
//!
//! - TaskDispatcher.
//! - TaskExecutor.
//! - DispatchJob.
//! - TaskExecutionContext.
//! - Execution coordination.
//!
//! Scheduling supplies an execution facility through the existing
//! TaskDispatcher contract. It does not implement another executor.
//!
//! ## Runtime policies
//!
//! `zur::policy`:
//!
//! Owns runtime-wide operational policies.
//!
//! `zur::scheduling::policy`:
//!
//! Owns scheduling-specific policy declarations.
//!
//! These are complementary authorities, not competing policy systems.
//!
//! ## Resource management
//!
//! `zur::resources`:
//!
//! Owns resource availability, requirements, reservations,
//! leases and accounting through its canonical interfaces.
//!
//! Scheduling must not assume that a resource is available merely
//! because a scheduling policy requests it.
//!
//! A scheduling decision does not constitute a resource reservation.
//!
//! ## Runtime context
//!
//! `zur::core::context`:
//!
//! Owns the immutable runtime context and injected operational
//! dependencies.
//!
//! Scheduler instances must receive their configuration and
//! dependencies explicitly.
//!
//! No process-wide scheduler singleton is required.
//!
//! ## Cancellation
//!
//! `zur::execution::cancellation`:
//!
//! Owns execution cancellation.
//!
//! Scheduling must cooperate with the existing cancellation
//! infrastructure without inventing a second cancellation model.
//!
//! # Separation from other scheduling authorities
//!
//! This module MUST NOT replace:
//!
//! - `compiler::planning`.
//! - `distributed::planning`.
//! - `hybrid::planning`.
//! - `quantum::scheduling`.
//! - `frontend::semantic`.
//! - `zur::resources`.
//!
//! Their respective owners determine compilation plans, distributed
//! execution plans, hybrid composition, quantum schedules, source
//! semantics and physical resource management.
//!
//! ZUR scheduling handles operational execution of work supplied
//! through the appropriate validated execution boundary.
//!
//! # Grammar integration
//!
//! Scheduling-related source-language intent is specified by:
//!
//! - `grammar/execution/scheduling.g4`.
//! - `grammar/spec/concurrency.md`.
//! - `grammar/spec/determinism.md`.
//! - `grammar/spec/resources.md`.
//! - `grammar/concurrency/tasks.g4`.
//! - `grammar/concurrency/parallel.g4`.
//! - `grammar/concurrency/task-parallel.g4`.
//!
//! The grammar and semantic layer own the meaning of scheduling intent.
//!
//! This module MUST NOT:
//!
//! - Parse Zamani source code.
//! - Interpret grammar rules.
//! - Define AST nodes.
//! - Redefine semantic scheduling requirements.
//! - Create a competing scheduling IR.
//! - Select physical hardware.
//!
//! # Scalability
//!
//! Scheduling must operate from the smallest meaningful computation
//! to the limits of resources actually available.
//!
//! There are no artificial architecture-level ceilings for:
//!
//! - Tasks.
//! - Jobs.
//! - Worker threads.
//! - Scheduling instances.
//! - Scheduling policies.
//! - Policy extensions.
//! - Execution providers.
//! - Computational domains.
//! - Devices.
//! - CPUs.
//! - GPUs.
//! - QPUs.
//! - Distributed participants.
//! - Resource quantities.
//!
//! Actual operational budgets may be configured explicitly.
//!
//! Such budgets describe operational resource constraints and must
//! not become permanent language or architecture limitations.
//!
//! No finite machine guarantees literal infinite physical computation.
//!
//! # Portability
//!
//! Logical parallelism is independent of physical parallelism.
//!
//! The same logical Zamani computation may execute using:
//!
//! - One execution context.
//! - Multiple local workers.
//! - Available CPU parallelism.
//! - An accelerator provider.
//! - A hybrid execution provider.
//! - An explicitly integrated distributed execution facility.
//! - A future computational substrate.
//!
//! The scheduler must preserve the semantics and execution contracts
//! established by the authoritative upstream systems.
//!
//! # Safety
//!
//! This module requires safe Rust.
//!
//! It introduces:
//!
//! - No unsafe code.
//! - No raw pointer ownership.
//! - No static mutable state.
//! - No global scheduler.
//! - No implicit worker creation.
//! - No fabricated resource availability.
//! - No fabricated task completion.
//!
//! # Execution guarantees
//!
//! The scheduling module distinguishes:
//!
//! - Policy declaration.
//! - Scheduling implementation availability.
//! - Job acceptance.
//! - Actual execution.
//! - Task completion.
//! - Resource reservation.
//!
//! These are separate events.
//!
//! In particular, successful job dispatch does not establish that
//! the underlying task has completed.
//!
//! # Integration flow
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Frontend and semantic analysis
//!      |
//!      v
//! Canonical computational representations
//!      |
//!      v
//! Compiler / domain planning
//!      |
//!      v
//! Validated execution request
//!      |
//!      v
//! ZUR execution
//!      |
//!      +-----------------------+
//!      |                       |
//!      v                       v
//! Scheduling policy      Resource authority
//!      |                       |
//!      +-----------+-----------+
//!                  |
//!                  v
//!          Operational scheduling
//!                  |
//!                  v
//!             TaskDispatcher
//!                  |
//!                  v
//!             TaskExecutor
//!                  |
//!                  v
//!         Task lifecycle and result
//! ```
//!
//! # Public API stability
//!
//! This module deliberately re-exports scheduling-specific APIs
//! while retaining the canonical ownership of concurrency types.
//!
//! Consumers requiring task execution should use:
//!
//! `crate::zur::concurrency`
//!
//! Consumers requiring scheduling should use:
//!
//! `crate::zur::scheduling`
//!
//! This avoids creating duplicate ownership or ambiguous module paths.

// ============================================================================
// CHILD MODULES
// ============================================================================

/// Declarative operational scheduling policies.
///
/// Owns scheduling algorithms, objectives, fairness, starvation,
/// tie-breaking and preemption policy declarations.
///
/// This module does not implement task execution.
pub mod policy;

/// Local operational scheduling implementation.
///
/// Provides explicitly configured local workers and job dispatch
/// through the canonical TaskDispatcher contract.
pub mod local;

// ============================================================================
// PUBLIC SCHEDULING POLICY API
// ============================================================================

/// Scheduling algorithm declaration.
pub use policy::SchedulingAlgorithm;

/// Direction of a scheduling objective.
pub use policy::ObjectiveDirection;

/// Declarative scheduling objective.
pub use policy::SchedulingObjective;

/// Scheduling tie-breaking rule.
pub use policy::TieBreakRule;

/// Scheduling preemption policy.
pub use policy::PreemptionPolicy;

/// Scheduling fairness policy.
pub use policy::FairnessPolicy;

/// Scheduling starvation-prevention policy.
pub use policy::StarvationPolicy;

/// Extensible scheduling policy values.
pub use policy::SchedulingExtensions;

/// Complete declarative scheduling policy.
pub use policy::SchedulingPolicy;

// ============================================================================
// PUBLIC LOCAL SCHEDULER API
// ============================================================================

/// Explicit local scheduler implementation.
pub use local::LocalScheduler;

/// Configuration for the local scheduler.
pub use local::LocalSchedulerConfig;

// ============================================================================
// MODULE CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::concurrency::executor::TaskDispatcher;

    /// Ensures the canonical scheduling policy is available through
    /// the public scheduling module.
    #[test]
    fn scheduling_policy_is_publicly_available() {
        let policy = SchedulingPolicy::default();

        assert!(policy.validate().is_ok());
    }

    /// Ensures scheduling policy types remain independently
    /// accessible through the public module.
    #[test]
    fn scheduling_policy_components_are_publicly_available() {
        let objective = SchedulingObjective::new(
            "zur.scheduling.objective.throughput",
            ObjectiveDirection::Maximize,
        )
        .expect("valid scheduling objective");

        let policy = SchedulingPolicy::new()
            .with_objective(objective)
            .expect("valid scheduling policy");

        assert!(policy.validate().is_ok());

        assert_eq!(
            policy.preemption(),
            PreemptionPolicy::Cooperative,
        );
    }

    /// Ensures the local scheduler remains connected to the
    /// canonical concurrency execution interface.
    #[test]
    fn local_scheduler_implements_canonical_dispatcher() {
        fn assert_dispatcher<T: TaskDispatcher>() {}

        assert_dispatcher::<LocalScheduler>();
    }

    /// Ensures local scheduler configuration is available through
    /// the public scheduling module.
    #[test]
    fn local_scheduler_configuration_is_publicly_available() {
        use std::num::NonZeroUsize;

        let config = LocalSchedulerConfig::new(
            NonZeroUsize::new(1).expect("one is nonzero"),
        );

        assert_eq!(
            config.worker_count().get(),
            1,
        );

        assert_eq!(
            config.queue_capacity(),
            None,
        );
    }

    /// Ensures policy defaults remain valid and do not imply
    /// physical resource allocation.
    #[test]
    fn default_policy_does_not_claim_resource_availability() {
        let policy = SchedulingPolicy::default();

        assert!(policy.validate().is_ok());

        assert_eq!(
            policy.algorithm(),
            &SchedulingAlgorithm::default(),
        );
    }
}
