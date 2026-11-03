
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Differentiation
//!
//! Path: `src/ai/differentiation/mod.rs`
//!
//! Rust baseline: 1.97+
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the composition root for the Zamani AI
//! differentiation subsystem.
//!
//! It exposes the independently implemented differentiation
//! components through one stable module boundary.
//!
//! Differentiation is a computational capability, not a hardware
//! implementation.
//!
//! The subsystem describes and coordinates derivative computation
//! while leaving physical realization to the compiler, execution
//! infrastructure, and available target capabilities.
//!
//! ## Architecture
//!
//! ```text
//! Zamani source
//!     |
//!     v
//! Canonical frontend AST
//!     |
//!     v
//! AI semantic validation
//!     |
//!     v
//! AI model and operation contracts
//!     |
//!     v
//! Differentiation
//!     |
//!     +--> rules
//!     +--> forward mode
//!     +--> reverse mode
//!     +--> dependency graph
//!     +--> checkpointing
//!     +--> validation
//!     |
//!     v
//! Canonical ZUIR
//!     |
//!     v
//! Optimization and lowering
//!     |
//!     v
//! Runtime / distributed / hybrid execution
//! ```
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Differentiation subsystem composition.
//! - Public differentiation module boundaries.
//! - Differentiation subsystem documentation.
//!
//! It does not own:
//!
//! - Source grammar.
//! - AST construction.
//! - Semantic type checking.
//! - Model graph construction.
//! - Tensor storage.
//! - Tensor arithmetic.
//! - Canonical intermediate representation.
//! - Hardware selection.
//! - Resource allocation.
//! - Scheduling.
//! - Runtime execution.
//!
//! ## Canonical integration authorities
//!
//! Source syntax:
//! `grammar/ai/differentiation.g4`
//!
//! Differentiability contracts:
//! `grammar/ai/differentiable.g4`
//!
//! AI semantic validation:
//! `crate::frontend::semantic::domains::ai`
//!
//! AI model operations:
//! `crate::ai::model::operations`
//!
//! AI gradient requests:
//! `crate::ai::learning::gradients`
//!
//! Canonical tensor storage:
//! `crate::classical::collections::tensor`
//!
//! Universal computational representation:
//! `crate::zuir`
//!
//! Quantum computational representation:
//! `crate::quantum::ir`
//!
//! Execution realization:
//! `crate::runtime`
//! `crate::distributed`
//! `crate::hybrid`
//!
//! These authorities must not be duplicated by this subsystem.
//!
//! ## POCO-REAF and scalability
//!
//! The differentiation subsystem introduces no universal maximum
//! for:
//!
//! - Differentiation variables.
//! - Differentiation operations.
//! - Graph nodes.
//! - Graph edges.
//! - Model parameters.
//! - Tensor rank.
//! - Tensor dimensions.
//! - Checkpoints.
//! - Differentiation rules.
//! - Operation providers.
//! - Execution targets.
//! - Distributed workers.
//!
//! Collection sizes are determined by the underlying representation,
//! available resources, and applicable operational policies.
//!
//! Resource exhaustion must be reported explicitly.
//!
//! It must not silently alter mathematical semantics.
//!
//! Literal infinite computation is not promised.
//!
//! ## Integration contract
//!
//! The frontend supplies canonical semantic facts.
//!
//! The AI model subsystem supplies operation contracts.
//!
//! The differentiation rule registry resolves derivative operations.
//!
//! Forward and reverse mode coordinate derivative propagation.
//!
//! The differentiation graph represents dependencies.
//!
//! Checkpointing describes retention and recomputation policies.
//!
//! Validation checks differentiation legality.
//!
//! The compiler lowers the resulting computation through canonical
//! ZUIR and its existing optimization infrastructure.
//!
//! Runtime and distributed infrastructure determine execution
//! feasibility and physical realization.
//!
//! ## Compatibility
//!
//! This module does not introduce a second public AI entry point.
//!
//! The crate-level declaration remains owned by `src/lib.rs`.
//!
//! `src/lib.rs` already declares `pub mod ai;`.
//!
//! This module is therefore registered through the existing AI
//! module hierarchy.
//!
//! ## Completion criteria
//!
//! This composition root is complete when:
//!
//! - Every declared child module exists.
//! - Child modules compile against their declared contracts.
//! - The module does not introduce duplicate authorities.
//! - Public module paths remain stable.
//! - The crate builds with the supported Rust baseline.
//! - The differentiation integration tests pass.

/// Differentiation checkpointing policies and plans.
///
/// This module describes which intermediate values may be retained
/// or recomputed during differentiation.
///
/// Actual value storage and checkpoint persistence remain owned
/// by the execution infrastructure.
pub mod checkpoints;

/// Differentiation dependency graphs.
///
/// This module owns graph structure, dependency relationships,
/// structural validation, and deterministic traversal.
///
/// It does not replace the canonical AI model graph.
pub mod graph;

/// Forward-mode automatic differentiation.
///
/// This module coordinates primal and tangent propagation through
/// registered derivative operation providers.
///
/// It does not implement tensor arithmetic kernels.
pub mod forward;

/// Differentiation rule definitions and registration.
///
/// This module owns rule identities, versions, applicability
/// declarations, compatibility, registration, and resolution.
///
/// A resolved rule is not proof of target feasibility.
pub mod rules;

/// Reverse-mode automatic differentiation.
///
/// This module coordinates reverse propagation, cotangent
/// accumulation, and requested gradient extraction.
///
/// Mathematical derivative operations are delegated to
/// registered providers.
pub mod reverse;

/// Differentiation validation.
///
/// This module coordinates structural and semantic differentiation
/// checks using canonical semantic facts and supplied resolvers.
///
/// It does not create a second semantic authority.
pub mod validation;

/// Public differentiation subsystem prelude.
///
/// This namespace provides convenient access to the primary
/// differentiation components without defining additional types
/// or duplicating their implementations.
///
/// Individual modules remain the canonical owners of their APIs.
pub mod prelude {
    pub use super::checkpoints;
    pub use super::forward;
    pub use super::graph;
    pub use super::rules;
    pub use super::reverse;
    pub use super::validation;
}
