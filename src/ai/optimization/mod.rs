#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Optimization
//!
//! Path: `src/ai/optimization/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! This module is the composition root for target-independent AI
//! optimization contracts and analyses.
//!
//! It exposes the existing optimization modules through one stable
//! namespace without creating another optimizer, graph representation,
//! semantic system, tensor implementation, IR, or execution engine.
//!
//! ## Module ownership
//!
//! | Module | Responsibility |
//! |---|---|
//! | `graph` | Structural graph analysis |
//! | `transformations` | Transformation contracts and transactional plans |
//! | `legality` | Structural and evidence-based transformation legality |
//! | `precision` | Numerical precision transformation contracts |
//! | `quantization` | Quantization semantics and calibration contracts |
//! | `sparsity` | Sparse representation and transformation contracts |
//! | `cost_model` | Extensible computational cost estimation |
//!
//! ## Ownership boundaries
//!
//! This module does not own:
//!
//! - AI model or pipeline graph representations;
//! - source-language AST;
//! - semantic analysis;
//! - canonical tensor storage;
//! - universal computational IR;
//! - hardware discovery;
//! - device allocation;
//! - physical placement;
//! - scheduling;
//! - distributed execution;
//! - runtime execution;
//! - backend-specific optimization.
//!
//! Those responsibilities remain with their respective repository
//! authorities.
//!
//! ## Canonical integration
//!
//! AI semantics:
//!     `crate::frontend::semantic::domains::ai`
//!
//! AI model and pipeline representations:
//!     `crate::ai::model`
//!     `crate::ai::pipeline`
//!
//! AI graph transformations:
//!     `crate::ai::optimization::transformations`
//!
//! Transformation legality:
//!     `crate::ai::optimization::legality`
//!
//! Numerical precision:
//!     `crate::ai::optimization::precision`
//!
//! Quantization:
//!     `crate::ai::optimization::quantization`
//!
//! Sparsity:
//!     `crate::ai::optimization::sparsity`
//!
//! Cost estimation:
//!     `crate::ai::optimization::cost_model`
//!
//! Universal computational representation:
//!     `crate::zuir`
//!
//! Classical tensor storage:
//!     `crate::classical::collections`
//!
//! Distributed realization:
//!     `crate::distributed`
//!
//! Runtime realization:
//!     `crate::runtime`
//!
//! ## Dependency rules
//!
//! The optimization modules may depend on shared AI foundation contracts
//! and on their explicitly declared canonical authorities.
//!
//! The composition root must not:
//!
//! - implement optimization algorithms;
//! - construct or mutate canonical graphs;
//! - perform semantic analysis;
//! - select execution hardware;
//! - allocate resources;
//! - perform scheduling;
//! - introduce cyclic dependencies between optimization modules.
//!
//! ## Legality invariant
//!
//! A structurally valid transformation is not necessarily semantically
//! legal.
//!
//! Transformations must pass the independent legality boundary before
//! they are committed.
//!
//! Missing, unknown, contradictory, or incomplete proof evidence must
//! never be interpreted as proof of legality.
//!
//! ## POCO-REAF and scalability
//!
//! This module imposes no universal maximum on:
//!
//! - graph nodes or edges;
//! - graph depth;
//! - transformation count;
//! - optimization passes;
//! - tensor rank;
//! - tensor dimensions;
//! - model parameters;
//! - quantization groups;
//! - sparse elements;
//! - execution targets;
//! - accelerators;
//! - distributed participants.
//!
//! Resource budgets and feasibility checks belong to the appropriate
//! compilation, planning, and execution authorities.
//!
//! Operational limits must not become language-level semantic limits.
//!
//! ## Determinism
//!
//! Deterministic ordering and validation are responsibilities of the
//! individual modules.
//!
//! This composition root introduces no runtime registration order,
//! global mutable state, or nondeterministic initialization.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No unsafe implementation is permitted.

// ============================================================================
// GRAPH ANALYSIS
// ============================================================================

/// Structural analysis of canonical AI computational graphs.
///
/// Provides graph topology analysis and deterministic structural reports.
/// Does not own or construct the canonical graph.
pub mod graph;

// ============================================================================
// TRANSFORMATION CONTRACTS
// ============================================================================

/// Target-independent AI graph transformation contracts.
///
/// Provides transformation identities, policies, plans, graph edits,
/// and transactional transformation orchestration.
pub mod transformations;

// ============================================================================
// TRANSFORMATION LEGALITY
// ============================================================================

/// Independent, fail-closed transformation legality validation.
///
/// Validates structural edit consistency and coordinates evidence-based
/// semantic preservation assessment.
///
/// This module must be consulted before a transformation is committed.
pub mod legality;

// ============================================================================
// NUMERICAL PRECISION
// ============================================================================

/// Target-independent numerical precision transformation contracts.
///
/// Defines numerical formats, rounding behavior, overflow and underflow
/// policies, and precision-preservation requirements.
pub mod precision;

// ============================================================================
// QUANTIZATION
// ============================================================================

/// Target-independent quantization semantics and contracts.
///
/// Defines quantization schemes, granularity, calibration evidence,
/// and quantization transformation descriptions.
pub mod quantization;

// ============================================================================
// SPARSITY
// ============================================================================

/// Target-independent sparse computation and transformation contracts.
///
/// Defines sparse encodings, implicit-value semantics, and the evidence
/// required for sparse transformations.
pub mod sparsity;

// ============================================================================
// COST MODEL
// ============================================================================

/// Extensible computational cost estimation.
///
/// Describes abstract workload metrics and externally supplied cost
/// profiles without selecting or allocating physical resources.
pub mod cost_model;