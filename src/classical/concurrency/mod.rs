
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical concurrency subsystem.
//!
//! File: `src/classical/concurrency/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only; no `unsafe`.
//!
//! # Purpose
//!
//! Expose the existing classical-concurrency components through one stable,
//! documented module boundary. This module composes APIs; it does not implement
//! scheduling, execution, resource discovery, or hardware-specific behavior.
//!
//! # Authority and ownership
//!
//! The normative language contract is `grammar/spec/concurrency.md`.
//! Determinism requirements are specified by `grammar/spec/determinism.md`.
//! The grammar files under `grammar/concurrency/` own source syntax and
//! parser composition. The frontend owns the AST, semantic analysis owns
//! source-level legality, and the canonical compiler IR owns the representation
//! consumed by later compiler stages.
//!
//! This module does not define a second AST, type system, IR, task identity
//! system, scheduler, executor, runtime, or hardware abstraction layer.
//!
//! # Module ownership
//!
//! - `task`: logical task IDs, descriptors, lifecycle, dependencies,
//!   cooperative cancellation, and task-body contracts.
//! - `data_parallel`: logical index domains, partitions, and reference
//!   data-parallel algorithms.
//! - `synchronization`: synchronization semantics and safe local primitives.
//! - `atomics`: atomic-operation descriptions and memory-order validation,
//!   reusing synchronization's canonical types.
//! - `channels`: safe local message channels and their buffering semantics.
//! - `futures`: canonical executor-independent future composition.
//! - `determinism`: deterministic ordering and policy contracts.
//!
//! None of these modules, by itself, guarantees parallel execution. An
//! executor must poll futures; a scheduler must plan tasks; a backend must
//! realize the selected operations.
//!
//! # POCO-REAF and scalability
//!
//! Concurrency intent is independent of a fixed number of CPUs, workers,
//! devices, nodes, tasks, futures, or partitions. This module introduces no
//! universal architecture-size limits. Actual host representations, memory,
//! configured operational budgets, semantic requirements, and target
//! capabilities still constrain what can be compiled or executed.
//!
//! Scaling across targets is valid only when the target satisfies the program's
//! semantic, effect, resource, capability, ordering, and numerical contracts.
//! Different valid execution strategies must preserve the required observable
//! behavior.
//!
//! # Integration contract
//!
//! Upstream:
//! - `grammar/concurrency/*.g4` defines the relevant source constructs.
//! - The frontend AST preserves parsed structure and source locations.
//! - Semantic analysis checks ownership, effects, types, dependencies,
//!   ordering, determinism, and target-independent requirements.
//! - The canonical IR represents validated operations and dependencies.
//!
//! Downstream:
//! - The compiler lowers canonical IR without re-parsing source syntax here.
//! - Execution planning and scheduling select a legal realization.
//! - Executors and runtimes own polling, task execution, cancellation
//!   integration, and runtime resource use.
//! - Target adapters determine whether local, distributed, accelerator, or
//!   future-substrate realizations can satisfy the contract.
//!
//! # Future API compatibility
//!
//! `futures` is the canonical implementation module. `asynchronization` is
//! retained as a module-path alias for source compatibility, so both paths
//! resolve to the same future-composition API. Do not declare the legacy
//! `asynchronization.rs` implementation as a second module here; migrate that
//! file to a re-export facade or remove it through a reviewed repository
//! change. Its existing tests must be moved or retained in the canonical
//! implementation before the old implementation is retired.
//!
//! # Freeze rule
//!
//! A public export is part of the concurrency API. Removing or changing an
//! exported symbol requires compatibility review. New modules may be declared
//! here only after their files and ownership contracts exist. This module
//! must not declare missing modules merely because they appear in a proposed
//! future directory tree.

// ---------------------------------------------------------------------------
// Canonical component modules
// ---------------------------------------------------------------------------

pub mod atomics;
pub mod channels;
pub mod data_parallel;
pub mod determinism;
pub mod futures;
pub mod synchronization;
pub mod task;

// ---------------------------------------------------------------------------
// Compatibility module path
// ---------------------------------------------------------------------------

/// Compatibility alias for the canonical [`futures`] module.
///
/// This is a module re-export, not a second future implementation.
pub use self::futures as asynchronization;

// ---------------------------------------------------------------------------
// Deliberate public API
// ---------------------------------------------------------------------------

/// Common task contracts shared by classical concurrency consumers.
///
/// `TaskId` is a logical identity, not a worker, thread, process, or device ID.
pub use self::task::{
    validate_task_graph,
    CancellationToken,
    DependencyKind,
    TaskContext,
    TaskDefinition,
    TaskDependency,
    TaskDescriptor,
    TaskError,
    TaskId,
    TaskOptions,
    TaskState,
};

/// Logical data-parallel domains and plans.
///
/// These types describe work independently of physical worker allocation.
pub use self::data_parallel::{
    DataParallelHint,
    DataParallelPlan,
    IndexDomain,
    PartitionGrain,
};

/// Determinism policy and stable logical ordering.
///
/// These contracts do not themselves guarantee that arbitrary program
/// execution is deterministic or bitwise reproducible.
pub use self::determinism::{
    DeterminismError,
    DeterminismMode,
    DeterminismPolicy,
    NumericReproducibility,
    ObservableOrder,
    OrderKey,
    OrderedEvent,
    OrderedEvents,
};

/// Executor-independent future helpers and combinators.
///
/// Joining futures does not spawn tasks or guarantee parallel execution.
pub use self::futures::{
    boxed,
    boxed_local,
    join_all,
    try_join_all,
    BoxFuture,
    LocalBoxFuture,
};

// ---------------------------------------------------------------------------
// Module-boundary tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name;
    use std::future::Ready;

    #[test]
    fn future_compatibility_path_is_the_canonical_module() {
        assert_eq!(
            type_name::<asynchronization::JoinAll<Ready<()>>>(),
            type_name::<futures::JoinAll<Ready<()>>>(),
        );

        assert_eq!(
            type_name::<asynchronization::BoxFuture<'static, ()>>(),
            type_name::<futures::BoxFuture<'static, ()>>(),
        );
    }

    #[test]
    fn canonical_task_types_are_exposed() {
        let task_id = TaskId::new(1);
        let options = TaskOptions::new();

        let descriptor =
            TaskDescriptor::new(task_id, options)
                .expect("an empty task descriptor should be valid");

        assert_eq!(descriptor.id(), task_id);
        assert_eq!(descriptor.options().dependencies().len(), 0);
    }

    #[test]
    fn logical_data_domain_is_exposed() {
        let domain =
            IndexDomain::new(0, 0).expect("an empty domain should be valid");

        assert!(domain.is_empty());
        assert_eq!(domain.start(), domain.end());
    }

    #[test]
    fn determinism_contract_types_are_exposed() {
        let policy = DeterminismPolicy::default();

        assert_eq!(
            policy.mode(),
            DeterminismMode::Unspecified,
        );
    }
}
