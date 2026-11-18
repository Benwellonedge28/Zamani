
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Task Dependency Graph
//!
//! Path: `src/toolchain/tasks/graph.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides the task-subsystem access path to the canonical task dependency
//! graph.
//!
//! The graph implementation is owned by:
//!
//!     crate::toolchain::build::graph
//!
//! This module deliberately reuses that implementation instead of defining
//! another graph representation or another graph algorithm.
//!
//! ## Ownership
//!
//! The canonical graph owns:
//!
//! - Task membership.
//! - Task prerequisite relationships.
//! - Dependency insertion and removal.
//! - Task removal and relationship cleanup.
//! - Structural graph validation.
//! - Cycle prevention and validation.
//! - Deterministic dependency-first topological ordering.
//! - Deterministic dependency-ready waves.
//!
//! This module does not own:
//!
//! - Task identity allocation.
//! - Task definitions or task execution contracts.
//! - Task lifecycle state.
//! - Task registration or provider discovery.
//! - Task execution.
//! - Retry authorization.
//! - Cancellation coordination.
//! - Worker pools or concurrency limits.
//! - Physical resource allocation.
//! - Compiler dependency graphs.
//! - Package dependency resolution.
//! - Runtime scheduling.
//! - Hardware discovery or execution.
//!
//! ## Canonical authorities
//!
//! Task identities:
//!
//!     crate::toolchain::ids::TaskId
//!
//! Task definitions and execution contracts:
//!
//!     crate::toolchain::tasks::model
//!
//! Canonical graph implementation:
//!
//!     crate::toolchain::build::graph::BuildGraph
//!
//! Error classification:
//!
//!     crate::toolchain::errors::ToolchainError
//!
//! Result contract:
//!
//!     crate::toolchain::result::ToolchainResult
//!
//! ## Graph direction
//!
//! Every edge points from a dependent task to its prerequisite.
//!
//!     application -> library
//!
//! The library must complete before the application becomes eligible.
//!
//! ## Determinism
//!
//! The canonical graph maintains deterministic task ordering and provides
//! deterministic dependency-first ordering and dependency-ready waves.
//!
//! This facade does not introduce another ordering policy.
//!
//! ## Scalability
//!
//! This module introduces no fixed limits on:
//!
//! - Tasks.
//! - Dependencies.
//! - Dependents.
//! - Graph depth.
//! - Computational domains.
//! - Targets.
//! - Hardware resources.
//! - Distributed participants.
//!
//! Storage and computation remain subject to the canonical graph's actual
//! representation, available memory, address space, and configured
//! operational budgets.
//!
//! Literal infinite memory or execution is not promised.
//!
//! ## Integration
//!
//! Task-oriented consumers may import:
//!
//!     crate::toolchain::tasks::graph::TaskGraph
//!
//! Build-oriented consumers may continue to import:
//!
//!     crate::toolchain::build::graph::BuildGraph
//!
//! Both names refer to the same canonical graph type.
//!
//! No conversion, synchronization, duplicated storage, or graph
//! reconstruction is required.
//!
//! ## Serialization
//!
//! This module introduces no serialization format.
//!
//! Persistent build-plan and task-graph serialization must use the
//! appropriate separately versioned toolchain contracts.

/// Canonical task dependency graph.
///
/// `TaskGraph` is an alias of the existing `BuildGraph`. Both names refer
/// to the same implementation and representation.
///
/// Use this name in task-subsystem code. Build orchestration may continue
/// using `BuildGraph`.
///
/// The graph determines dependency relationships and structural execution
/// eligibility. It does not execute tasks, allocate resources, or schedule
/// physical hardware.
pub use crate::toolchain::build::graph::BuildGraph as TaskGraph;

#[cfg(test)]
mod tests {
    use super::TaskGraph;

    /// The task-oriented graph API must construct the canonical empty graph.
    #[test]
    fn task_graph_constructs_canonical_empty_graph() {
        let graph = TaskGraph::new();

        assert!(graph.is_empty());
        assert_eq!(graph.task_count(), 0);
        assert_eq!(graph.dependency_count(), 0);
        assert!(graph.validate().is_ok());
    }
}
