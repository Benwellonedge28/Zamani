
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Synchronization
//!
//! Path: `src/hybrid/sync/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Compose and expose the canonical synchronization contracts used by
//! Zamani's hybrid-computation subsystem.
//!
//! This module is the public module boundary for synchronization semantics.
//! It does not implement a synchronization engine. Its child modules own
//! the distinct synchronization responsibilities:
//!
//! - [`model`] declares target-independent synchronization requirements.
//! - [`ordering`] analyzes ordering guarantees using the canonical hybrid
//!   dependency graph.
//! - [`completion`] represents operation completion and result availability.
//! - [`cancellation`] represents cancellation requests and their outcomes.
//!
//! ## Ownership boundaries
//!
//! Synchronization declarations and runtime synchronization are different
//! responsibilities. This module describes and exposes contracts; it does
//! not execute operations or choose how a target realizes those contracts.
//!
//! The following existing repository components remain authoritative:
//!
//! - `crate::hybrid::ids` owns hybrid synchronization-event identities.
//! - `crate::hybrid::errors` owns structured hybrid errors.
//! - `crate::hybrid::graph::edges` owns hybrid dependency relationships.
//! - `crate::hybrid::graph::analysis` owns canonical graph-analysis data.
//! - `crate::hybrid::data::ownership` owns cross-domain ownership contracts.
//! - `crate::compiler::hybrid_pipeline` retains its existing compiler-facing
//!   compatibility API and adapts it to canonical hybrid contracts.
//! - `crate::runtime::hybrid_kernel` and other runtime components own actual
//!   execution, polling, cancellation, synchronization and resource release.
//! - `crate::quantum::ir` remains the canonical quantum semantic boundary.
//!
//! This module must not introduce a second dependency graph, IR, scheduler,
//! memory model, completion engine, cancellation engine or resource manager.
//!
//! ## Dependency direction
//!
//! The synchronization modules may depend on foundational hybrid contracts,
//! graph interfaces, and canonical semantic types where required.
//!
//! Compiler and runtime integrations may depend on these synchronization
//! contracts. The synchronization module must not depend on a concrete
//! backend, vendor SDK, physical device, runtime session or target scheduler.
//!
//! ## Semantic guarantees
//!
//! - A synchronization declaration does not itself establish a dependency
//!   absent from the canonical graph or semantic model.
//! - An asynchronous operation does not imply that its result is available.
//! - A missing known ordering path does not prove that reordering is safe.
//! - A cancellation request does not prove that execution has stopped.
//! - A barrier is an abstract coordination requirement, not a hardware
//!   barrier instruction.
//! - Completion and cancellation outcomes must reflect authoritative
//!   execution observations, not assumptions made by this module.
//!
//! ## Scalability
//!
//! This module introduces no fixed limits on synchronization events,
//! participants, domains, graph nodes, completion records or cancellation
//! records. Collection growth, work budgets and allocation failures are
//! handled by the owning implementations and their explicit policies.
//!
//! No finite implementation can guarantee infinite execution or unlimited
//! memory. Operational exhaustion must not silently change program meaning.
//!
//! ## Determinism and compatibility
//!
//! Determinism guarantees are defined by the child modules and the canonical
//! graph and identifier contracts. This module does not infer execution order
//! from declaration order or identifier order.
//!
//! Public re-exports provide a convenient API without transferring ownership
//! of the underlying types. Changes to public types or serialized contracts
//! must follow the repository's compatibility and interchange policies.
//!
//! ## Safety
//!
//! This module forbids unsafe Rust. It introduces no global mutable state,
//! implicit runtime initialization, background work, hardware discovery,
//! I/O, or hidden resource limits.

// ---------------------------------------------------------------------------
// Canonical synchronization modules
// ---------------------------------------------------------------------------

/// Cancellation requests, propagation policies and observed outcomes.
///
/// Actual cancellation and resource cleanup remain runtime responsibilities.
pub mod cancellation;

/// Operation completion, lifecycle validation and result availability.
///
/// This module represents lifecycle state; it does not execute or schedule
/// the operation being tracked.
pub mod completion;

/// Target-independent synchronization declarations and their invariants.
///
/// This is the canonical declaration model for hybrid synchronization.
pub mod model;

/// Read-only ordering analysis over canonical hybrid dependencies.
///
/// This module does not own or reconstruct the dependency graph.
pub mod ordering;

// ---------------------------------------------------------------------------
// Stable convenience exports
// ---------------------------------------------------------------------------

/// Cancellation contracts and in-memory cancellation bookkeeping.
pub use cancellation::{
    CancellationFailure,
    CancellationPropagation,
    CancellationRecord,
    CancellationRequest,
    CancellationStatus,
    CancellationTracker,
};

/// Completion contracts and in-memory completion bookkeeping.
pub use completion::{
    CompletionFailure,
    CompletionRecord,
    CompletionStatus,
    CompletionTracker,
};

/// Canonical synchronization declarations.
pub use model::{
    ExecutionMode,
    SynchronizationEvent,
    SynchronizationKind,
    SynchronizationModel,
    SynchronizationScope,
};

/// Ordering guarantees and analysis results.
pub use ordering::{
    OrderingAnalysis,
    OrderingAssessment,
    OrderingConstraint,
    OrderingGuarantee,
};
