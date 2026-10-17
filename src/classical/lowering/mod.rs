
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Classical Lowering
//!
//! Canonical path: `src/classical/lowering/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: Unsafe Rust is forbidden throughout this module tree.
//!
//! ## Purpose
//!
//! Compose the target-independent classical lowering subsystem. This module
//! exposes stable paths to the lowering context, scalar and aggregate
//! planners, control-flow adapter, and target-independent planning
//! coordinator.
//!
//! This is a composition root, not an implementation of an additional
//! intermediate representation.
//!
//! ## Architectural authority
//!
//! The lowering subsystem must respect the repository's existing authority
//! boundaries:
//!
//! - `DESIGN.md` and the applicable architecture specifications govern
//!   repository-wide architecture.
//! - `grammar/Zamani.g4` defines the root grammar composition contract.
//! - `grammar/antlr/ZamaniLexer.g4` and
//!   `grammar/antlr/ZamaniParser.g4` define the lexer and parser boundaries.
//! - `grammar/specification/semantic-model.md` governs source-language
//!   semantic meaning.
//! - `grammar/specification/compilation-model.md` governs compilation-stage
//!   responsibilities.
//! - `grammar/specification/scalability-model.md` governs scalability.
//! - `grammar/specification/poco-reaf.md` governs portability and POCO-REAF.
//! - `grammar/validation/semantic-boundaries.md` governs semantic ownership
//!   boundaries.
//!
//! Grammar defines syntax; semantic analysis establishes source meaning;
//! lowering prepares a representation for a downstream canonical IR
//! builder and verifier.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Registration and public composition of the classical lowering modules.
//! - Deliberate re-exports of stable lowering entry points.
//! - The lowering subsystem's module-level safety and documentation policy.
//!
//! It does not own:
//!
//! - Source syntax, parsing, AST nodes, or semantic analysis.
//! - Canonical semantic types or source-level value semantics.
//! - Canonical identifiers or identity allocation.
//! - A second AST, CFG, IR, or runtime.
//! - Concrete machine layouts or backend-specific representations.
//! - Target discovery, physical resource allocation, scheduling, or execution.
//!
//! ## Canonical representation boundary
//!
//! `src/quantum/ir/classical/` remains an existing classical IR integration
//! boundary. `src/zuir/` owns the universal intermediate representation and
//! its identity, verification, and extension contracts.
//!
//! The classical lowering subsystem must adapt to those existing owners.
//! It must not introduce competing IR nodes, identifiers, type definitions,
//! or verifier implementations.
//!
//! ## Module responsibilities
//!
//! - `context`: identity mappings, provenance, symbolic requirements,
//!   capabilities, lowering dispositions, notes, and explicit work budgets.
//! - `scalar`: target-independent scalar planning that preserves width and
//!   representation obligations.
//! - `aggregate`: target-independent aggregate planning that preserves
//!   member order, nested structure, and unresolved layout/type obligations.
//! - `control_flow`: adapter contracts for forwarding validated control-flow
//!   plans to a canonical destination builder.
//! - `target_independent`: coordination of target-independent scalar and
//!   aggregate plans using the shared lowering context.
//!
//! Each implementation module remains responsible for its own invariants,
//! validation, errors, and tests.
//!
//! ## Resource and scalability contract
//!
//! This module introduces no fixed semantic limits on program size, number
//! of types, aggregate members, control-flow entities, hardware resources,
//! or computational domains.
//!
//! Operational budgets are explicit policy inputs. An absent work budget
//! means this layer imposes no work-unit budget; it does not promise infinite
//! memory, time, address space, or target capacity.
//!
//! Symbolic and unresolved target-dependent requirements must remain explicit
//! until the appropriate downstream stage can resolve or diagnose them.
//!
//! ## Integration sequence
//!
//! 1. The existing frontend produces source AST nodes.
//! 2. Semantic analysis validates names, types, effects, ownership, and
//!    program-level constraints.
//! 3. The compiler adapts validated semantic entities into classical
//!    descriptors and lowering plans.
//! 4. This module exposes the relevant planning and adapter APIs.
//! 5. A canonical-IR adapter uses the actual `src/zuir/` or applicable
//!    existing classical IR builder and verifier.
//! 6. Target-specific lowering, feasibility checks, layout, scheduling,
//!    resource allocation, and runtime execution occur downstream.
//!
//! This module does not itself connect the compiler pipeline to a backend.
//! That integration must be implemented and tested by the component that
//! owns the actual compiler-to-IR boundary.
//!
//! ## Completion contract
//!
//! This file is complete when every declared module exists, all exports
//! resolve, the public API is documented, the crate compiles with the
//! supported Rust toolchain, and the module-level integration tests pass.
//! Repository production readiness additionally requires end-to-end compiler,
//! canonical-IR verification, portability, resource, and regression tests.
//!
//! Changes to module ownership or public exports require compatibility and
//! dependency review.

// Existing lowering implementations. These declarations intentionally
// reference only files present in the inspected repository snapshot.
pub mod aggregate;
pub mod context;
pub mod control_flow;
pub mod scalar;
pub mod target_independent;

// Stable public entry points for consumers that do not need to name the
// implementation submodule for common lowering operations.
pub use aggregate::{
    AggregateFieldPlan,
    AggregateLoweringError,
    AggregateLoweringObligation,
    AggregateLoweringOptions,
    AggregateLoweringPlan,
    AggregateMemberPlan,
    AggregateNodeIndex,
    AggregateNodePlan,
    AggregateVariantPlan,
};
pub use context::{
    CapabilityRequirement,
    EntityKey,
    EntityKind,
    EntityMapping,
    LoweringContext,
    LoweringContextError,
    LoweringContextResult,
    LoweringDisposition,
    LoweringNote,
    LoweringPolicy,
    ProvenanceOrigin,
    RequirementRelation,
    ResourceRequirement,
};
pub use control_flow::{
    ControlFlowLowerer,
    ControlFlowLoweringOptions,
    ControlFlowLoweringSink,
};
pub use scalar::{
    LoweringObligation,
    ScalarLoweringError,
    ScalarLoweringPlan,
    ScalarRepresentation,
    ScalarLoweringResult,
};
pub use target_independent::{
    TargetIndependentLowerer,
    TargetIndependentLoweringError,
    TargetIndependentPlan,
    TargetIndependentPlanKind,
    TargetIndependentResult,
};
