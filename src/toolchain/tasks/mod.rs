
#![forbid(unsafe_code)]

//! # Zamani Toolchain — Task Subsystem
//!
//! Path: `src/toolchain/tasks/mod.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Unsafe Rust is prohibited.
//!
//! ## Purpose
//!
//! This module is the canonical composition and public API boundary
//! for domain-independent Zamani toolchain tasks.
//!
//! It connects task definitions, dependency planning, graph access,
//! provider registration, execution, cancellation and execution reports.
//!
//! It does not implement those subsystems a second time.
//!
//! ## POCO-REAF
//!
//! Task orchestration must support programs and build operations across:
//!
//! - Classical computation.
//! - Quantum computation.
//! - HDL and hardware realization.
//! - AI and tensor computation.
//! - Hybrid computation.
//! - Distributed computation.
//! - Networking and cryptographic computation.
//! - Embedded systems and accelerators.
//! - Future registered computational domains.
//!
//! Task kinds and providers are extensible. No closed enumeration of
//! computational domains or hardware implementations is introduced.
//!
//! ## Ownership boundaries
//!
//! | Responsibility | Canonical owner |
//! |---|---|
//! | Task identities | `toolchain::ids` |
//! | Task definitions and lifecycle | `tasks::model` |
//! | Task dependency relationships | `build::graph` |
//! | Input-to-output dependency bindings | `tasks::dependencies` |
//! | Provider registration | `tasks::registry` |
//! | Task execution | `tasks::executor` |
//! | Cancellation | `tasks::cancellation` |
//! | Build orchestration | `toolchain::build` |
//! | Resource budgets | Canonical compiler/toolchain budget infrastructure |
//! | Artifact verification | Canonical compiler/backend artifact infrastructure |
//! | Physical resource allocation | Canonical capability and hardware providers |
//! | Runtime execution | `zur` |
//!
//! ## Dependency direction
//!
//! ```text
//!                  tasks/mod.rs
//!                       |
//!          +------------+------------+
//!          |            |            |
//!          v            v            v
//!        model       registry     cancellation
//!          |            |            |
//!          +------------+------------+
//!                       |
//!                       v
//!                    executor
//!                       |
//!             +---------+---------+
//!             |                   |
//!             v                   v
//!         dependencies         reporting
//!             |
//!             v
//!       canonical build graph
//! ```
//!
//! The diagram describes API relationships, not additional graph
//! implementations. The canonical graph remains `build::graph::BuildGraph`.
//!
//! ## Scalability
//!
//! This module imposes no application-defined maximum on:
//!
//! - Tasks.
//! - Task kinds.
//! - Inputs or outputs.
//! - Providers.
//! - Dependencies.
//! - Computational domains.
//! - Targets.
//! - Hardware devices.
//! - Distributed participants.
//!
//! Actual operation is constrained by available memory, address space,
//! provider capabilities, semantic requirements and configured budgets.
//!
//! It does not promise literal infinite memory or computation.
//!
//! ## Concurrency
//!
//! This module contains no global mutable state and creates no implicit
//! worker pools or background threads.
//!
//! Thread safety is provided by the individual subsystem contracts.
//! Provider implementations are invoked outside registry locks.
//!
//! ## Error handling
//!
//! All task-subsystem operations use the canonical
//! `toolchain::errors::ToolchainError` and
//! `toolchain::result::ToolchainResult` contracts.
//!
//! Errors must not be silently converted into successful task outcomes.
//!
//! ## Integration
//!
//! Build orchestration consumes the public task API through this module.
//!
//! Task execution consumes validated dependency plans, registered
//! providers, shared cancellation observations and canonical budgets.
//!
//! Backend, compiler, hardware and runtime implementations remain owned
//! by their existing repository subsystems.

// ============================================================================
// SUBMODULE DECLARATIONS
// ============================================================================

/// Canonical task definitions, input/output declarations,
/// execution contracts and lifecycle state.
pub mod model;

/// Access to the canonical build dependency graph.
///
/// This module is a facade over `build::graph::BuildGraph`.
/// It does not implement a second graph.
pub mod graph;

/// Validated task dependency planning.
///
/// Owns named input-to-output bindings and validates producer
/// definitions before execution.
pub mod dependencies;

/// Thread-safe task-provider registration and resolution.
///
/// Supports multiple providers for the same task kind and
/// explicit provider selection.
pub mod registry;

/// Domain-independent task execution orchestration.
///
/// Invokes registered providers and validates execution results.
pub mod executor;

/// Shared task-operation cancellation authority.
///
/// Integrates with the canonical backend process cancellation token.
pub mod cancellation;

// ============================================================================
// CANONICAL IDENTITIES
// ============================================================================

/// Canonical task identity.
///
/// The identity is owned by `toolchain::ids`, not by the task subsystem.
pub use crate::toolchain::ids::TaskId;

/// Canonical provider identity.
pub use crate::toolchain::ids::ProviderId;

/// Canonical artifact identity.
pub use crate::toolchain::ids::ArtifactId;

/// Canonical workspace identity.
pub use crate::toolchain::ids::WorkspaceId;

// ============================================================================
// TASK MODEL API
// ============================================================================

/// Extensible task-kind identifier.
pub use model::TaskKind;

/// Origin of a declared task input.
pub use model::TaskInputSource;

/// Declared task input.
pub use model::TaskInput;

/// Declared task output.
pub use model::TaskOutput;

/// Task concurrency-safety contract.
pub use model::ConcurrencySafety;

/// Task cancellation-behavior contract.
pub use model::CancellationBehavior;

/// Task retry-safety contract.
pub use model::RetrySafety;

/// Task resumability contract.
pub use model::Resumability;

/// Complete task execution contract.
pub use model::TaskExecutionContract;

/// Immutable task definition.
pub use model::TaskDefinition;

/// Structured task failure.
pub use model::TaskFailure;

/// Canonical task lifecycle status.
pub use model::TaskStatus;

/// Validated mutable task lifecycle state.
pub use model::TaskState;

// ============================================================================
// CANONICAL GRAPH API
// ============================================================================

/// Canonical task dependency graph.
///
/// This is an alias of `build::graph::BuildGraph`.
pub use graph::TaskGraph;

// ============================================================================
// TASK DEPENDENCY API
// ============================================================================

/// A validated named input-to-output dependency binding.
pub use dependencies::TaskDependencyBinding;

/// Validated task dependency plan.
///
/// Retains individual input bindings while using the canonical
/// build graph for task-level dependency relationships.
pub use dependencies::TaskDependencyPlan;

/// Derives and validates a task dependency graph from definitions.
///
/// Use `TaskDependencyPlan::from_definitions` when input bindings
/// must also be retained.
pub use dependencies::derive_task_graph;

// ============================================================================
// PROVIDER REGISTRY API
// ============================================================================

/// Immutable task-provider registration descriptor.
pub use registry::TaskProviderDescriptor;

/// Generic thread-safe task-provider registry.
///
/// The implementation contract is supplied by the registry consumer.
pub use registry::TaskRegistry;

// ============================================================================
// EXECUTION API
// ============================================================================

/// Identifier for one logical task-execution operation.
pub use executor::ExecutionId;

/// Task-provider execution interface.
pub use executor::TaskExecutorProvider;

/// Context supplied to task providers.
pub use executor::TaskExecutionContext;

/// Resolved input artifacts supplied to a provider.
pub use executor::ResolvedTaskInputs;

/// Outputs returned by a task provider.
pub use executor::TaskExecutionOutputs;

/// Request to execute a validated task operation.
pub use executor::TaskExecutionRequest;

/// Structured result of an individual task execution.
pub use executor::TaskExecutionRecord;

/// Aggregate task-execution report.
pub use executor::TaskExecutionReport;

/// Domain-independent task executor.
pub use executor::TaskExecutor;

/// Cancellation observation interface used by the executor.
pub use executor::CancellationProbe;

/// Cancellation token interface used by the executor.
pub use executor::CancellationToken;

/// Default cancellation probe for operations without
/// an external cancellation authority.
pub use executor::NeverCancelled;

/// Canonical execution-budget interface.
pub use executor::ExecutionBudget;

/// Lifetime-bound execution-budget reservation.
pub use executor::BudgetReservation;

/// Canonical task artifact-verification interface.
pub use executor::TaskArtifactVerifier;

// ============================================================================
// CANCELLATION API
// ============================================================================

/// Authority allowed to request cancellation.
pub use cancellation::CancellationSource;

/// Read-oriented cancellation observation token.
pub use cancellation::CancellationToken as SharedCancellationToken;

// ============================================================================
// PROVIDER REGISTRY TYPE ALIAS
// ============================================================================

/// Canonical registry for task-execution providers.
///
/// This alias connects the generic registry implementation to the
/// executor-owned provider interface.
///
/// It avoids repeating the trait-object type throughout build
/// orchestration and application integration.
///
/// Multiple providers may support the same task kind. Resolution
/// must remain explicit when provider selection is ambiguous.
pub type TaskProviderRegistry =
    TaskRegistry<dyn TaskExecutorProvider>;

// ============================================================================
// COMPOSITION CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the task graph uses the canonical build graph.
    #[test]
    fn task_graph_is_the_canonical_build_graph() {
        let graph = TaskGraph::new();

        assert!(graph.is_empty());
        assert_eq!(graph.task_count(), 0);
        assert_eq!(graph.dependency_count(), 0);
        assert!(graph.validate().is_ok());
    }

    /// Verifies that the task identity is the canonical toolchain ID.
    #[test]
    fn task_identity_uses_canonical_identity_type() {
        fn accepts_task_id(_: &crate::toolchain::ids::TaskId) {}

        fn accepts_public_task_id(_: &TaskId) {}

        let _ = accepts_task_id;
        let _ = accepts_public_task_id;
    }

    /// Verifies that provider registration uses the executor contract.
    #[test]
    fn provider_registry_uses_executor_provider_contract() {
        fn accepts_registry(_: &TaskProviderRegistry) {}

        let _ = accepts_registry;
    }

    /// Verifies that dependency planning is available through
    /// the canonical task composition boundary.
    #[test]
    fn dependency_plan_is_publicly_available() {
        fn accepts_plan(_: Option<TaskDependencyPlan>) {}

        fn accepts_binding(_: Option<TaskDependencyBinding>) {}

        accepts_plan(None);
        accepts_binding(None);
    }

    /// Verifies that the lifecycle model is publicly available.
    #[test]
    fn lifecycle_contracts_are_publicly_available() {
        fn accepts_status(_: Option<TaskStatus>) {}

        fn accepts_state(_: Option<TaskState>) {}

        fn accepts_failure(_: Option<TaskFailure>) {}

        accepts_status(None);
        accepts_state(None);
        accepts_failure(None);
    }

    /// Verifies that task execution reports remain accessible.
    #[test]
    fn execution_reporting_is_publicly_available() {
        fn accepts_report(_: Option<TaskExecutionReport>) {}

        fn accepts_record(_: Option<TaskExecutionRecord>) {}

        accepts_report(None);
        accepts_record(None);
    }

    /// Verifies that cancellation authorities are publicly available.
    #[test]
    fn cancellation_contracts_are_publicly_available() {
        fn accepts_source(_: Option<CancellationSource>) {}

        fn accepts_token(_: Option<SharedCancellationToken>) {}

        accepts_source(None);
        accepts_token(None);
    }

    /// Verifies that task kinds are extensible values rather
    /// than a closed domain-specific enumeration.
    #[test]
    fn task_kinds_are_extensible() {
        let classical = TaskKind::new(
            "zamani.classical",
            "compile",
            "1",
        )
        .expect("valid classical task kind");

        let quantum = TaskKind::new(
            "zamani.quantum",
            "compile",
            "1",
        )
        .expect("valid quantum task kind");

        let hdl = TaskKind::new(
            "zamani.hdl",
            "synthesize",
            "1",
        )
        .expect("valid HDL task kind");

        let ai = TaskKind::new(
            "zamani.ai",
            "train",
            "1",
        )
        .expect("valid AI task kind");

        assert_ne!(classical, quantum);
        assert_ne!(quantum, hdl);
        assert_ne!(hdl, ai);
    }
}
