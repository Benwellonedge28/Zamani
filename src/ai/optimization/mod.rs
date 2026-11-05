
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
//! Composition root for target-independent AI optimization contracts,
//! analyses, and transformation-related facilities.
//!
//! This module exposes the existing optimization implementations through
//! stable module paths. It does not introduce a second optimizer,
//! computational graph, semantic system, tensor implementation, IR,
//! execution engine, or hardware abstraction.
//!
//! ## Module ownership
//!
//! | Module | Responsibility |
//! |---|---|
//! | `graph` | Structural analysis of canonical computational graphs |
//! | `transformations` | Transformation contracts, plans, and orchestration |
//! | `legality` | Structural and evidence-based transformation legality |
//! | `precision` | Numerical precision transformation contracts |
//! | `quantization` | Quantization semantics and calibration contracts |
//! | `sparsity` | Sparse representation and transformation contracts |
//! | `cost_model` | Extensible computational cost estimation |
//!
//! Each implementation module owns its own types, errors, validation,
//! algorithms, and tests.
//!
//! This composition root owns only module exposure and the integration
//! contracts documented here.
//!
//! ## Architectural ownership
//!
//! This module does not own:
//!
//! - source-language syntax or AST;
//! - AI semantic analysis;
//! - canonical AI model or pipeline graphs;
//! - canonical tensor storage;
//! - universal computational IR;
//! - physical hardware discovery;
//! - device selection or allocation;
//! - physical placement;
//! - execution scheduling;
//! - distributed execution;
//! - runtime execution;
//! - backend-specific code generation.
//!
//! These responsibilities remain with their canonical repository
//! authorities.
//!
//! ## Canonical integration authorities
//!
//! AI semantic analysis:
//!     `crate::frontend::semantic::domains::ai`
//!
//! AI model definitions and graphs:
//!     `crate::ai::model`
//!
//! AI pipeline definitions and graphs:
//!     `crate::ai::pipeline`
//!
//! Canonical tensor infrastructure:
//!     `crate::classical::collections`
//!
//! Universal computational representation:
//!     `crate::zuir`
//!
//! Distributed realization:
//!     `crate::distributed`
//!
//! Runtime realization:
//!     `crate::runtime`
//!
//! Backend realization:
//!     `crate::backend`
//!
//! Optimization contracts:
//!     `crate::ai::optimization`
//!
//! ## Dependency direction
//!
//! Optimization modules may consume canonical AI model, pipeline,
//! semantic, tensor, and IR contracts through explicitly declared
//! dependencies.
//!
//! The composition root must not:
//!
//! - implement optimization algorithms;
//! - construct or mutate canonical model or pipeline graphs;
//! - replace canonical semantic validation;
//! - define a competing tensor representation;
//! - define a competing universal IR;
//! - select physical execution hardware;
//! - allocate physical resources;
//! - schedule execution;
//! - introduce implicit global state;
//! - introduce cyclic module dependencies.
//!
//! ## Transformation legality contract
//!
//! Structural validity and semantic legality are distinct properties.
//!
//! A transformation that is structurally well-formed is not necessarily
//! semantically equivalent or otherwise permitted.
//!
//! Transformation legality is owned by `legality`.
//!
//! A transformation must not be committed merely because its edit plan
//! is structurally valid.
//!
//! Missing, unknown, contradictory, incomplete, or invalid proof
//! evidence must not be interpreted as proof of legality.
//!
//! The transformation orchestration layer must preserve the distinction
//! between:
//!
//! - transformation construction;
//! - structural validation;
//! - legality assessment;
//! - transformation application;
//! - post-transformation verification;
//! - successful completion.
//!
//! Failure at any required stage must not be silently converted into
//! successful transformation completion.
//!
//! ## Numerical correctness
//!
//! Precision reduction, quantization, and sparsity transformations can
//! change numerical behavior.
//!
//! Their contracts must explicitly represent the applicable numerical
//! assumptions, preservation requirements, error bounds, calibration
//! evidence, and validation outcomes.
//!
//! This composition root does not declare that a numerical
//! transformation is automatically semantics-preserving.
//!
//! Numerical guarantees remain the responsibility of the owning
//! precision, quantization, sparsity, and legality modules.
//!
//! ## Cost estimation contract
//!
//! Cost estimation describes expected or measured computational costs.
//!
//! A cost estimate is not:
//!
//! - proof of resource availability;
//! - proof of execution feasibility;
//! - a hardware allocation;
//! - a scheduling decision;
//! - a runtime execution result;
//! - a guarantee of measured performance.
//!
//! Cost models must remain extensible and must not require a closed
//! list of processor vendors, accelerators, execution targets, or
//! resource categories.
//!
//! Physical resource discovery and allocation remain downstream.
//!
//! ## POCO-REAF and scalability
//!
//! This module introduces no application-defined maximum for:
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
//! - processors;
//! - accelerators;
//! - distributed participants.
//!
//! Actual capacity is determined by available memory, allocator
//! capacity, platform addressability, and explicit operational
//! resource policies.
//!
//! Configurable resource budgets are permitted where appropriate.
//! They must not become language-level semantic ceilings.
//!
//! A resource or feasibility failure must be represented explicitly.
//! It must not silently alter the meaning of a computation.
//!
//! POCO-REAF does not imply physically unlimited computation.
//!
//! ## Determinism
//!
//! Deterministic ordering, validation, and reporting are responsibilities
//! of the individual implementation modules.
//!
//! This composition root introduces no registration-order dependency,
//! global mutable state, or nondeterministic initialization.
//!
//! ## Error handling
//!
//! Each optimization module retains its specialized error types.
//!
//! Errors must not be silently discarded or converted into success.
//!
//! Incomplete validation must not be reported as complete validation.
//!
//! Unknown outcomes must remain distinguishable from successful outcomes.
//!
//! ## Compatibility
//!
//! The existing public module paths are preserved:
//!
//! - `crate::ai::optimization::graph`;
//! - `crate::ai::optimization::transformations`;
//! - `crate::ai::optimization::legality`;
//! - `crate::ai::optimization::precision`;
//! - `crate::ai::optimization::quantization`;
//! - `crate::ai::optimization::sparsity`;
//! - `crate::ai::optimization::cost_model`.
//!
//! This module intentionally avoids broad type re-exports.
//!
//! Types remain owned by their defining modules. This prevents the
//! composition root from creating duplicate names or accidentally
//! establishing a second public API.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! Unsafe Rust is prohibited.

// ============================================================================
// GRAPH ANALYSIS
// ============================================================================

/// Structural analysis of canonical AI computational graphs.
///
/// Analyzes supplied canonical node and edge identities without taking
/// ownership of, constructing, or replacing the canonical graph.
pub mod graph;

// ============================================================================
// TRANSFORMATION CONTRACTS
// ============================================================================

/// Target-independent AI graph transformation contracts.
///
/// Owns transformation descriptions, policies, plans, edits, and
/// transformation orchestration contracts.
pub mod transformations;

// ============================================================================
// TRANSFORMATION LEGALITY
// ============================================================================

/// Independent, fail-closed transformation legality validation.
///
/// Owns structural edit consistency and evidence-based assessment
/// of transformation legality.
///
/// Structural validity alone must never be treated as semantic proof.
pub mod legality;

// ============================================================================
// NUMERICAL PRECISION
// ============================================================================

/// Target-independent numerical precision transformation contracts.
///
/// Owns numerical format, rounding, overflow, underflow, and precision
/// preservation contracts.
pub mod precision;

// ============================================================================
// QUANTIZATION
// ============================================================================

/// Target-independent quantization semantics and contracts.
///
/// Owns quantization schemes, granularity, calibration evidence,
/// and quantization transformation descriptions.
pub mod quantization;

// ============================================================================
// SPARSITY
// ============================================================================

/// Target-independent sparse computation and transformation contracts.
///
/// Owns sparse encodings, implicit-value semantics, and evidence
/// required for sparse transformations.
pub mod sparsity;

// ============================================================================
// COST MODEL
// ============================================================================

/// Extensible computational cost estimation.
///
/// Describes abstract workload metrics and externally supplied cost
/// profiles without selecting hardware, allocating resources, or
/// scheduling execution.
pub mod cost_model;

// ============================================================================
// COMPOSITION-ROOT REGRESSION TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    // Import the module namespace explicitly. This makes the test
    // compilation verify that every established module path remains
    // present and publicly accessible.
    #[allow(unused_imports)]
    use super::{
        cost_model,
        graph,
        legality,
        precision,
        quantization,
        sparsity,
        transformations,
    };

    /// The optimization module must preserve every established child
    /// module path.
    ///
    /// This is a compile-time integration regression test: removing,
    /// renaming, or making one of these modules private breaks the test.
    #[test]
    fn established_optimization_module_paths_remain_available() {
        let _ = core::any::type_name::<
            super::graph::OptimizationGraphAnalysis<u64>,
        >();

        assert_eq!(
            module_path!(),
            "zamani::ai::optimization::tests"
        );
    }

    /// The composition root must not create a second graph authority.
    ///
    /// The graph analysis module remains the designated owner of
    /// optimization graph analysis.
    #[test]
    fn graph_analysis_remains_in_its_owning_module() {
        let _ = super::graph::analyze_graph::<
            u64,
            [u64; 0],
            [(u64, u64); 0],
        >;
    }

    /// All optimization facilities remain separate public modules.
    ///
    /// Their implementation contracts must not be collapsed into
    /// this composition root.
    #[test]
    fn optimization_facilities_have_separate_module_owners() {
        let graph_path = core::any::type_name::<
            super::graph::OptimizationGraphAnalysis<u64>,
        >();

        assert!(graph_path.contains("optimization::graph"));
    }
}
