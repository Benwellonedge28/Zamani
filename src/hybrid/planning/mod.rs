
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Hybrid Computing — Planning Subsystem
//!
//! Path: `src/hybrid/planning/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! This module is the public composition root for hybrid-computation
//! planning in Zamani.
//!
//! It connects the existing planning contracts that:
//!
//! - represent a target-independent hybrid realization plan;
//! - validate a plan against the canonical hybrid graph and edge set;
//! - prepare a plan for downstream realization;
//! - assess portability using explicit validation and feasibility evidence.
//!
//! The planning subsystem preserves the meaning and dependencies of
//! computations across classical, quantum, accelerator, tensor, AI/ML,
//! HDL/hardware, distributed, networking, and future computational domains.
//!
//! ## Architectural ownership
//!
//! This module organizes and selectively re-exports the existing planning
//! implementations. It does not introduce another representation or
//! implementation of their responsibilities.
//!
//! Canonical ownership remains:
//!
//! - `plan`: abstract hybrid realization plans and plan-local dependencies.
//! - `validation`: structural and semantic plan-validation evidence.
//! - `interface`: validated preparation requests and prepared artifacts.
//! - `portability`: portability conclusions derived from supplied evidence.
//! - `crate::hybrid::graph`: the canonical hybrid composition graph and edges.
//! - `crate::hybrid::resources`: canonical requirements, capabilities,
//!   constraints, and feasibility assessments.
//! - `crate::hybrid::config`: explicit operational policies and work budgets.
//! - canonical semantic and domain IR implementations: program meaning.
//! - compiler and backend modules: target-specific lowering and realization.
//! - runtime modules: execution, lifecycle, and physical resource handling.
//!
//! ## Explicit non-responsibilities
//!
//! This module must not:
//!
//! - define another hybrid graph or universal computational IR;
//! - duplicate ZUIR, ZCIR, HDLIR, or `quantum::ir`;
//! - introduce a competing resource or capability model;
//! - choose physical hardware or allocate devices;
//! - implement target placement, physical scheduling, routing, or QEC;
//! - generate target-specific instructions or executable code;
//! - duplicate `src/compiler/hybrid_pipeline.rs`;
//! - own runtime execution or vendor-specific integrations;
//! - interpret source text or introduce a second semantic model.
//!
//! ## Integration contract
//!
//! The compiler constructs requests from canonical hybrid graphs,
//! edge sets, resource requirements, resource assessments, capability
//! evidence, and explicit configuration.
//!
//! `interface` coordinates plan construction and validation. The resulting
//! artifact can be passed to downstream realization components, which must
//! independently satisfy the program's semantic requirements and verify
//! target feasibility before claiming that execution is supported.
//!
//! `portability` assesses supplied validation and feasibility evidence.
//! It must not manufacture missing evidence or interpret the absence of
//! evidence as proof of portability.
//!
//! Structural validation alone is not proof of semantic completeness,
//! target feasibility, or successful execution.
//!
//! ## Scalability and operational limits
//!
//! This composition module imposes no fixed universal limits on graph size,
//! domain count, node count, edge count, capability count, or hardware size.
//!
//! Operational limits belong to `crate::hybrid::config` and are applied by
//! the individual operations that consume resources. Budget exhaustion must
//! be reported explicitly and must never silently change program semantics.
//!
//! No finite machine can guarantee literally infinite computation. Zamani's
//! scalability goal is to avoid arbitrary language-level ceilings and to
//! use available resources subject to explicit operational policies.
//!
//! ## Grammar authority
//!
//! Source syntax and language meaning remain governed by the canonical
//! grammar, frontend, and semantic layers, including:
//!
//! - `grammar/Zamani.g4`;
//! - `grammar/hybrid/hybrid.g4`;
//! - `grammar/hybrid/README.md`;
//! - `grammar/spec/hybrid.md`.
//!
//! This module defines Rust API composition only. It does not create or
//! imply support for source-language syntax absent from the canonical grammar.
//!
//! ## Compatibility
//!
//! Public re-exports are deliberately limited to the established planning
//! contracts. Internal implementation details should remain accessible
//! through their owning submodules rather than being duplicated here.
//!
//! Changes to the public surface must preserve compatibility with existing
//! compiler and runtime integrations or be accompanied by an explicit,
//! tested migration.

/// Abstract, target-independent hybrid realization plans.
pub mod plan;

/// Structural and semantic validation of hybrid realization plans.
pub mod validation;

/// Planning request, validation policy, and preparation interface.
pub mod interface;

/// Portability assessment based on supplied validation and feasibility
/// evidence.
pub mod portability;

// -----------------------------------------------------------------------------
// Abstract plan API
// -----------------------------------------------------------------------------

pub use plan::{
    HybridPlan,
    PlanDependency,
    PlanNode,
};

// -----------------------------------------------------------------------------
// Validation API
// -----------------------------------------------------------------------------

pub use validation::{
    HybridPlanValidator,
    PlanValidationMode,
    PlanValidationReport,
};

// -----------------------------------------------------------------------------
// Planning preparation API
// -----------------------------------------------------------------------------

pub use interface::{
    CanonicalHybridPlanningInterface,
    HybridPlanningInterface,
    HybridPlanningRequest,
    PlanningReadiness,
    PlanningValidationRequirement,
    PreparedHybridPlanning,
    HYBRID_PLANNING_INTERFACE_NAME,
    HYBRID_PLANNING_INTERFACE_VERSION,
};

// -----------------------------------------------------------------------------
// Portability assessment API
// -----------------------------------------------------------------------------

pub use portability::{
    HybridPortabilityAssessor,
    PortabilityFinding,
    PortabilityFindingKind,
    PortabilityReport,
    PortabilityStatus,
    HYBRID_PORTABILITY_MODULE_NAME,
    HYBRID_PORTABILITY_SCHEMA_VERSION,
};
