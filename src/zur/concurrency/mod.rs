
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Concurrency
//!
//! Path: `src/zur/concurrency/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//!
//! ## Purpose
//!
//! Provides the public composition boundary for concurrency
//! infrastructure within the Zamani Universal Runtime (ZUR).
//!
//! This module integrates task lifecycle management, task execution,
//! local message communication and synchronization primitives.
//!
//! It establishes a reusable, domain-neutral concurrency foundation
//! for classical, quantum, HDL, hybrid, distributed and future
//! computational environments.
//!
//! ## Architectural objective
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Concurrency describes computational intent independently of
//! the physical execution resources eventually used.
//!
//! A task may execute through an explicitly supplied execution
//! facility. Its implementation may ultimately use local execution,
//! multiple workers, an accelerator, a domain provider or another
//! execution substrate.
//!
//! This module does not prescribe that implementation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Public concurrency module composition.
//! - Stable ZUR concurrency module paths.
//! - Public re-exports of established concurrency contracts.
//! - Documentation of concurrency integration boundaries.
//!
//! Its child modules retain their individual responsibilities.
//!
//! ### task
//!
//! Owns:
//!
//! - Task.
//! - TaskId.
//! - TaskStatus.
//! - TaskSnapshot.
//! - TaskOutcome.
//! - Task lifecycle transitions.
//! - Task-local completion observation.
//! - Cooperative task cancellation requests.
//!
//! ### executor
//!
//! Owns:
//!
//! - TaskDispatcher.
//! - TaskExecutor.
//! - TaskExecutionContext.
//! - InlineTaskDispatcher.
//! - Task submission coordination.
//! - Execution outcome publication.
//! - Execution panic containment.
//!
//! ### channel
//!
//! Reuses the canonical standard-library channel implementation.
//!
//! Owns the ZUR-facing local channel integration boundary.
//!
//! ### synchronization
//!
//! Provides safe synchronization primitives backed by the
//! Rust standard library.
//!
//! ## Existing authoritative dependencies
//!
//! The concurrency subsystem integrates with:
//!
//! - `zur/error.rs`:
//!   Canonical ZurError and ZurResult.
//!
//! - `zur/ids.rs`:
//!   Canonical RuntimeOperationId.
//!
//! - `zur/execution/cancellation.rs`:
//!   ExecutionCancellationController.
//!
//! - `zur/execution/session.rs`:
//!   Execution session and cancellation ownership.
//!
//! - `zur/contracts/execution.rs`:
//!   ExecutionEngine and execution contracts.
//!
//! - `zur/core/context.rs`:
//!   RuntimeContext.
//!
//! - `zur/resources/`:
//!   Runtime resource management.
//!
//! - `zur/scheduling/`:
//!   Operational scheduling.
//!
//! - `zur/distributed/`:
//!   Distributed execution and communication.
//!
//! - `classical/concurrency/`:
//!   Classical concurrency computation primitives.
//!
//! - `stdlib/concurrency/`:
//!   Developer-facing concurrency APIs.
//!
//! - `grammar/concurrency/`:
//!   Source-level concurrency syntax.
//!
//! - `grammar/spec/concurrency.md`:
//!   Normative concurrency semantics.
//!
//! ## Ownership boundaries
//!
//! This module does NOT:
//!
//! - Define another task identity system.
//! - Define another runtime error system.
//! - Define another cancellation controller.
//! - Implement another future or promise abstraction.
//! - Implement an asynchronous event loop.
//! - Create implicit worker threads.
//! - Implement a thread pool.
//! - Implement work stealing.
//! - Define scheduling policies.
//! - Select CPU cores.
//! - Discover physical devices.
//! - Allocate physical resources.
//! - Implement distributed message transport.
//! - Implement actor-system semantics.
//! - Define classical, quantum or HDL intermediate representations.
//! - Perform compiler planning.
//! - Interpret source-language grammar.
//! - Implement domain-specific execution.
//!
//! Those responsibilities remain with their existing
//! authoritative repository subsystems.
//!
//! ## Concurrency invariants
//!
//! 1. Task identity is allocated by the canonical ZUR identity system.
//!
//! 2. Task lifecycle is owned by task.rs.
//!
//! 3. Actual task execution is delegated to an explicit dispatcher.
//!
//! 4. Task submission is not equivalent to task completion.
//!
//! 5. Cancellation requests are not proof of execution termination.
//!
//! 6. Cancellation is acknowledged only after execution has stopped.
//!
//! 7. A task failure must never be reported as successful completion.
//!
//! 8. Local channels must never be mistaken for distributed transport.
//!
//! 9. Blocking synchronization must not be confused with asynchronous
//!    executor coordination.
//!
//! 10. Synchronization primitives must preserve their documented
//!     memory-ordering and poisoning behavior.
//!
//! 11. Runtime concurrency must respect semantic ownership,
//!     effects, requirements and validated execution contracts.
//!
//! 12. Resource acquisition and scheduling remain outside this
//!     module's ownership.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary universal ceilings on:
//!
//! - Tasks.
//! - Concurrent operations.
//! - Channels.
//! - Messages.
//! - Senders.
//! - Receivers.
//! - Synchronization objects.
//! - Execution facilities.
//! - Workers.
//! - Threads.
//! - Processors.
//! - Devices.
//! - Computational domains.
//! - Distributed participants.
//!
//! Growth is constrained by:
//!
//! - Available memory.
//! - Address-space capacity.
//! - Allocator capacity.
//! - Operating-system capabilities.
//! - Execution-provider capabilities.
//! - Explicit operational budgets.
//! - Actual hardware resources.
//!
//! No finite implementation promises literal infinite computation.
//!
//! A workload that exceeds available resources must produce an
//! explicit, structured failure or an appropriate incomplete result.
//!
//! It must never silently truncate work or fabricate completion.
//!
//! ## POCO-REAF integration
//!
//! The same logical concurrency model must remain usable across:
//!
//! - Small embedded systems.
//! - Single-processor environments.
//! - Multicore systems.
//! - Multiprocessor systems.
//! - GPU and accelerator environments.
//! - Quantum/classical environments.
//! - HDL and hardware execution.
//! - Distributed environments.
//! - Future computational domains.
//!
//! The actual execution strategy is determined downstream through
//! validated execution plans and explicitly supplied providers.
//!
//! Adding a new computational domain must not require modifying
//! this module merely to introduce that domain.
//!
//! ## Grammar integration
//!
//! Source-level concurrency constructs are owned by:
//!
//! - `grammar/concurrency/concurrency.g4`
//! - `grammar/concurrency/tasks.g4`
//! - `grammar/concurrency/spawn.g4`
//! - `grammar/concurrency/await.g4`
//! - `grammar/concurrency/futures.g4`
//! - `grammar/concurrency/channels.g4`
//! - `grammar/concurrency/synchronization.g4`
//! - `grammar/concurrency/cancellation.g4`
//! - Other specialized concurrency grammars.
//!
//! Their normative semantics are owned by:
//!
//! `grammar/spec/concurrency.md`
//!
//! The grammar expresses portable programmer intent.
//!
//! Frontend and semantic analysis validate that intent.
//!
//! Compiler and domain authorities construct compatible
//! representations and execution plans.
//!
//! ZUR realizes accepted execution requests.
//!
//! This module must never parse grammar files or introduce
//! grammar-specific execution behavior.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No unsafe functions.
//! No unsafe implementations.
//! No raw-pointer concurrency APIs.
//! No static mutable state.
//! No global mutable task registries.
//! No implicit execution facilities.
//!
//! ## Module composition
//!
//! Each implementation remains independently owned and testable.
//! This file only exposes the established implementation boundaries.

// ============================================================================
// CONCURRENCY COMPONENTS
// ============================================================================

/// Canonical task identity, lifecycle, cancellation requests
/// and completion observation.
///
/// This module does not create or schedule execution workers.
pub mod task;

/// Explicit task execution and dispatcher integration.
///
/// This module does not implement a worker pool or scheduling policy.
pub mod executor;

/// Local typed message channels.
///
/// Reuses the canonical standard-library channel implementation.
/// Does not implement distributed communication.
pub mod channel;

/// Safe local synchronization primitives.
///
/// Reuses Rust standard-library synchronization facilities.
pub mod synchronization;

// ============================================================================
// TASK API
// ============================================================================

/// Canonical runtime-managed task.
pub use task::Task;

/// Canonical runtime task identity.
///
/// This is an alias of RuntimeOperationId.
pub use task::TaskId;

/// Observable task lifecycle state.
pub use task::TaskStatus;

/// Immutable task state observation.
pub use task::TaskSnapshot;

/// Immutable task completion outcome.
pub use task::TaskOutcome;

// ============================================================================
// EXECUTION API
// ============================================================================

/// Explicit task execution facility contract.
pub use executor::TaskDispatcher;

/// Task execution coordinator.
pub use executor::TaskExecutor;

/// Read-only operational context supplied to a running task.
pub use executor::TaskExecutionContext;

/// Inline execution facility.
///
/// Executes operations on the calling thread without creating workers.
pub use executor::InlineTaskDispatcher;

/// Owned executable task operation.
pub use executor::TaskOperation;

/// Type-erased job transferred to an execution facility.
pub use executor::DispatchJob;

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::ids::{
        IdNamespace,
        RuntimeOperationId,
    };

    #[test]
    fn task_identity_reuses_canonical_runtime_identity() {
        let namespace = IdNamespace::new(
            "zamani.test.concurrency",
        )
        .expect("valid namespace");

        let identity = RuntimeOperationId::new(
            namespace,
            vec![1],
        )
        .expect("valid runtime identity");

        let task_id: TaskId = identity.clone();

        assert_eq!(task_id, identity);
    }

    #[test]
    fn task_status_exposes_terminal_classification() {
        assert!(!TaskStatus::Created.is_terminal());
        assert!(!TaskStatus::Ready.is_terminal());
        assert!(!TaskStatus::Running.is_terminal());

        assert!(TaskStatus::Completed.is_terminal());
        assert!(TaskStatus::Failed.is_terminal());
        assert!(TaskStatus::Cancelled.is_terminal());
    }

    #[test]
    fn inline_dispatcher_is_available_without_worker_creation() {
        let executor = TaskExecutor::inline();

        let _dispatcher: &InlineTaskDispatcher =
            executor.dispatcher();
    }

    #[test]
    fn concurrency_modules_are_composed() {
        fn assert_task_type<T: Send + Sync + 'static>() {}

        assert_task_type::<Task<u64>>();
    }
}
