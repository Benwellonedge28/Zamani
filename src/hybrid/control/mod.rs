
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Control Composition
//!
//! Path: `src/hybrid/control/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//!
//! ## Purpose
//!
//! This module is the public composition root for hybrid control contracts.
//! It connects structural control flow, dependency analysis, feedback
//! relationships, and conditional-execution requirements without creating
//! a second control-flow language or a competing intermediate representation.
//!
//! The hybrid control layer describes relationships among canonical
//! computation entities. It does not execute operations, evaluate
//! expressions, determine types, schedule hardware, or manage runtime state.
//!
//! ## Canonical ownership
//!
//! - `flow` owns hybrid control-region descriptors.
//! - `dependencies` indexes the canonical dependency edges owned by
//!   `crate::hybrid::graph::edges`.
//! - `feedback` describes and validates causal feedback contracts.
//! - `conditions` associates canonical predicate references with expected
//!   conditional-execution outcomes.
//! - `crate::hybrid::graph::model` owns the canonical hybrid graph.
//! - `crate::hybrid::graph::edges` owns dependency-edge definitions.
//! - `crate::hybrid::ids` owns hybrid identity types.
//! - The canonical frontend and semantic layer own source expressions,
//!   name resolution, types, effects, and predicate meaning.
//! - `crate::quantum::ir` remains authoritative for quantum semantics.
//! - The compiler and runtime remain responsible for realization and
//!   execution, respectively.
//!
//! This module only composes and re-exports these existing authorities.
//!
//! ## Integration contract
//!
//! The frontend must pass canonical, semantically resolved references into
//! the hybrid layer. Structural validation must not be mistaken for proof
//! that a predicate has the correct type or that a target supports it.
//!
//! Graph construction and validation use the canonical hybrid graph and
//! edge set. Feedback contracts refer to those same graph-scoped identities.
//! Conditional requirements refer to canonical predicates; they do not
//! contain a second expression representation.
//!
//! Compiler integration may consume these interfaces to preserve control
//! dependencies and feedback requirements during planning. Runtime
//! integration must preserve their meaning when implementing execution,
//! completion, synchronization, failure, and cancellation.
//!
//! The existing `crate::compiler::hybrid_pipeline` and
//! `crate::runtime::hybrid_kernel` remain separate integration authorities.
//! This module does not replace their APIs or claim that either is already
//! fully integrated.
//!
//! ## Correctness and cycles
//!
//! A dependency is not proof that a producer has completed or that a value
//! is available. Consumers must use the relevant semantic and runtime
//! contracts to establish those facts.
//!
//! Feedback, loops, and repeated computation can legitimately create cycles.
//! This module imposes no universal directed-acyclic-graph restriction.
//! Cycle legality belongs to semantic-aware graph and control-flow validation.
//!
//! ## Scalability and resource policy
//!
//! This module imposes no fixed maximum on domains, nodes, edges, control
//! regions, feedback flows, conditions, or loop iterations.
//!
//! Its child modules use the canonical graph structures and growable
//! collections. Any operational work or resource budget must be supplied
//! through the relevant configuration and validation APIs. Exhaustion must
//! be reported explicitly rather than silently changing program semantics.
//!
//! This is an architectural scalability objective, not a claim that finite
//! hardware can provide infinite memory, time, or execution capacity.
//!
//! ## Compatibility
//!
//! The child modules remain independently addressable through this public
//! module. The re-exports below provide convenient imports while retaining
//! their canonical definitions in their owning files.
//!
//! No target-specific implementation, vendor dependency, hardware limit,
//! global mutable registry, or runtime execution policy belongs here.

/// Structural control-flow composition over canonical hybrid graph entities.
///
/// Owns control-region descriptors and their structural validation.
/// Does not own dependency edges, predicate evaluation, or scheduling.
pub mod flow;

/// Read-only dependency analysis over the canonical hybrid edge set.
///
/// Does not create a second dependency graph or infer runtime readiness.
pub mod dependencies;

/// Causal feedback contracts across computational domains.
///
/// Does not execute measurement, evaluate decisions, or implement loops.
pub mod feedback;

/// Conditional-execution requirements referring to canonical predicates.
///
/// Does not parse, evaluate, type-check, or duplicate predicate expressions.
pub mod conditions;

// -----------------------------------------------------------------------------
// Structural control-flow API
// -----------------------------------------------------------------------------

pub use flow::{
    ControlFlowForm,
    HybridControlFlow,
    HybridControlRegion,
};

// -----------------------------------------------------------------------------
// Dependency-analysis API
// -----------------------------------------------------------------------------

pub use dependencies::{
    classify_edge_kind,
    DependencyClassification,
    DependencySemantics,
    HybridDependencyIndex,
};

// -----------------------------------------------------------------------------
// Feedback API
// -----------------------------------------------------------------------------

pub use feedback::{
    HybridFeedbackFlow,
    HybridFeedbackFlowSet,
};

// -----------------------------------------------------------------------------
// Conditional-execution API
// -----------------------------------------------------------------------------

pub use conditions::{
    CanonicalPredicateResolver,
    ConditionalExecutionRequirement,
    PredicateOutcome,
};
