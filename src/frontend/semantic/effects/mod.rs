
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Effects
//!
//! File: `src/frontend/semantic/effects/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Define the public module boundary for target-independent semantic effect
//! representation, inference, compatibility checking, and authorization.
//!
//! This module aggregates the effect subsystem's existing implementations.
//! It does not implement a second effect model, infer effects itself, or
//! introduce a second authorization or policy system.
//!
//! ## AUTHORITY
//!
//! Normative semantic authority:
//! - `grammar/spec/effects.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! Source syntax:
//! - `grammar/effects/effect-sets.g4`
//! - `grammar/effects/effect-composition.g4`
//! - `grammar/effects/effect-declarations.g4`
//! - `grammar/effects/effect-operations.g4`
//! - `grammar/effects/effect-types.g4`
//!
//! Canonical source-level effect identity:
//! - `src/frontend/ast/node/effects/effect_kind.rs`
//!
//! ## OWNERSHIP
//!
//! This module owns:
//! - public aggregation of the semantic effect subsystem;
//! - stable, documented module paths for effect APIs;
//! - convenient re-exports of the canonical effect interfaces;
//! - the boundary between effect representation, inference, checking,
//!   and authorization.
//!
//! This module does not own:
//! - AST construction or source parsing;
//! - canonical effect identity or effect-row algorithms;
//! - name resolution or generic substitution;
//! - type inference or type compatibility;
//! - policy-language parsing or policy-scope resolution;
//! - capability evidence or resource feasibility;
//! - physical resource allocation or target selection;
//! - classical IR, `quantum::ir`, HDL IR, routing, scheduling, or execution.
//!
//! ## MODULE CONTRACTS
//!
//! `model`
//! - Owns canonical effect arguments, instances, sets, variables, and rows.
//! - Performs structural validation of effect-model values.
//!
//! `inference`
//! - Infers effect rows from an explicitly constructed semantic dependency
//!   graph.
//! - Handles recursive propagation using deterministic fixed-point analysis.
//! - Does not authorize effects or decide declared-effect compatibility.
//!
//! `checking`
//! - Checks inferred effect rows against declared effect contracts.
//! - Uses exact identity by default and explicit policy for permitted widening.
//! - Does not decide whether an effect is authorized.
//!
//! `authorization`
//! - Evaluates inferred effects against explicit authorization rules.
//! - Defaults to deny when no applicable allow rule exists.
//! - Reports unresolved effect variables rather than treating them as pure.
//! - Does not prove target capability or resource availability.
//!
//! ## DEPENDENCY DIRECTION
//!
//! ```text
//! canonical AST EffectKind
//!          |
//!          v
//!        model
//!          |
//!          +-------------------+
//!          |                   |
//!          v                   v
//!      inference            checking
//!          |                   |
//!          +---------+---------+
//!                    |
//!                    v
//!              authorization
//!                    |
//!                    v
//!         semantic result / diagnostics
//! ```
//!
//! This diagram describes responsibility and data flow, not a requirement
//! that the Rust module import graph follow every arrow. In particular,
//! authorization remains independently callable and must not depend on
//! inference having executed internally.
//!
//! ## INTEGRATION CONTRACT
//!
//! The frontend semantic pipeline must:
//!
//! 1. Resolve source-level effect references to canonical AST effect kinds.
//! 2. Construct and validate canonical effect instances and rows.
//! 3. Build the inference graph from resolved semantic dependencies.
//! 4. Infer effects and retain inference errors explicitly.
//! 5. Check inferred rows against declared effect contracts.
//! 6. Resolve policy declarations and scopes outside this module.
//! 7. Evaluate authorization using the resulting explicit policy.
//! 8. Attach results and diagnostics to canonical AST node identities.
//! 9. Prevent invalid, denied, or incomplete required analysis from being
//!    finalized as a successfully validated semantic program.
//!
//! The parent semantic pipeline owns orchestration, diagnostic conversion,
//! operational budgets, and finalization.
//!
//! ## POCO-REAF AND SCALABILITY
//!
//! The public API introduces no fixed catalogue of effects, computational
//! domains, devices, operations, or hardware capacities.
//!
//! The effect model is open to new qualified effect identities. Classical,
//! quantum, HDL, hybrid, distributed, accelerator, AI/data, and future-domain
//! effects use the same canonical interfaces.
//!
//! Collection sizes are governed by available resources and explicit
//! operational budgets owned by the analysis session. If a budget prevents
//! required analysis from completing, the pipeline must report incomplete
//! analysis rather than claim semantic success.
//!
//! No finite implementation can guarantee processing an actually infinite
//! input. This module does not impose artificial hardware-capacity limits on
//! the language's semantic meaning.
//!
//! ## DETERMINISM AND ERROR HANDLING
//!
//! The canonical model and its analysis modules own their respective ordering
//! and validation guarantees. This aggregator must not reorder results,
//! swallow errors, replace failures with empty effect rows, or silently
//! convert unresolved effects into valid results.
//!
//! ## COMPATIBILITY
//!
//! The `model`, `inference`, `checking`, and `authorization` module paths are
//! public integration paths. Public re-exports below provide a concise API;
//! the defining modules remain accessible for explicit imports.
//!
//! Public API changes must follow the repository's compatibility policy.
//!
//! ## FREEZE CRITERIA
//!
//! - Every declared child module exists and compiles.
//! - Re-exported items match the public declarations in their defining files.
//! - The canonical AST `EffectKind` integration compiles.
//! - Effect-model, inference, checking, and authorization tests pass.
//! - The parent semantic pipeline can consume the APIs without duplicate
//!   effect representations or authorization logic.
//! - Workspace checks pass on the supported Rust toolchain.
//! - No unsafe code or fixed hardware-capacity limits are introduced.

// =============================================================================
// Canonical subsystem modules
// =============================================================================

/// Canonical effect identities, instances, arguments, sets, variables,
/// and rows.
pub mod model;

/// Effect propagation and fixed-point inference over semantic dependencies.
pub mod inference;

/// Compatibility checking between inferred and declared effect rows.
pub mod checking;

/// Policy-based authorization of inferred effect rows.
pub mod authorization;

// =============================================================================
// Canonical effect-model API
// =============================================================================

pub use model::{
    ConstantKey,
    DeclarationKey,
    EffectArgument,
    EffectInstance,
    EffectModelError,
    EffectRow,
    EffectSet,
    EffectVariable,
    TypeKey,
    SEMANTIC_EFFECT_MODEL_SCHEMA_VERSION,
};

// =============================================================================
// Effect-inference API
// =============================================================================

pub use inference::{
    infer_effects,
    EffectDependency,
    EffectInferenceError,
    EffectInferenceGraph,
    EffectInferenceNode,
    EffectInferenceResult,
    EffectPropagation,
};

// =============================================================================
// Effect-checking API
// =============================================================================

pub use checking::{
    check_effect_compatibility,
    check_effect_compatibility_with,
    same_effect_kind,
    validate_effect_row,
    EffectCheckIssue,
    EffectCheckReport,
    EffectCheckingError,
    EffectCheckingPolicy,
    EffectRelation,
    EffectWidening,
    EffectWideningPolicy,
    ExactEffectRelation,
};

// =============================================================================
// Effect-authorization API
// =============================================================================

pub use authorization::{
    AuthorizationDecision,
    AuthorizationDecisionSource,
    AuthorizationPolicyError,
    AuthorizationRule,
    EffectAuthorizationError,
    EffectAuthorizationPolicy,
    EffectAuthorizationRecord,
    EffectAuthorizationReport,
};
