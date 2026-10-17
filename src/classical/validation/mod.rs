
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Validation Subsystem
//!
//! Path: `src/classical/validation/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide the single public aggregation boundary for classical-computing
//! validation. This module registers and re-exports the existing validation
//! domains without introducing competing semantic models or IRs.
//!
//! # Authority
//!
//! Language and semantic contracts:
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/types.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! Canonical implementations:
//! - `src/frontend/semantic/`: canonical semantic types, effects, and resources.
//! - `src/frontend/ast/node/validation/`: source AST structural validation.
//! - `src/zuir/`: universal IR and its verification contracts.
//! - `src/quantum/ir/classical/`: canonical classical semantics used by the
//!   quantum IR; this module must not duplicate those semantics.
//!
//! Existing classical validation modules:
//! - `structural.rs`: structural graph validation.
//! - `types.rs`: canonical semantic type validation.
//! - `shapes.rs`: shape and dimension validation.
//! - `effects.rs`: effect-row and effect-contract validation.
//! - `resources.rs`: resource requirement and constraint validation.
//!
//! # Ownership
//!
//! This module owns:
//! - Registration of the classical validation modules.
//! - Their public aggregation and re-export boundary.
//! - The documented validation-domain boundaries.
//!
//! This module does not own:
//! - Source grammar, parsing, AST construction, or name resolution.
//! - Canonical semantic types, effect rows, or resource models.
//! - A new classical IR, ZUIR, or quantum IR.
//! - Type inference, generic constraint solving, or effect inference.
//! - Hardware discovery, target selection, placement, or scheduling.
//! - Runtime execution, memory allocation, or optimization passes.
//!
//! # Validation boundaries
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! grammar -> frontend AST -> semantic analysis
//!                              |
//!                              v
//!                    canonical semantic models
//!                              |
//!              +---------------+---------------+
//!              |               |               |
//!              v               v               v
//!         structural        types/shapes      effects/resources
//!              |               |               |
//!              +---------------+---------------+
//!                              |
//!                              v
//!                canonical IR verification
//!                              |
//!                              v
//!                 optimization and lowering
//!                              |
//!                              v
//!                   target realization
//! ```
//!
//! The diagram describes responsibilities, not a mandatory call sequence.
//! Callers must invoke the checks appropriate to the object and compilation
//! stage they are validating.
//!
//! # Validation policy and completeness
//!
//! A successful check proves only the properties covered by that check.
//! Structural validity does not prove type correctness, effect compatibility,
//! resource feasibility, target capability, or runtime safety.
//!
//! A validator that exhausts an explicitly configured operational budget must
//! report that validation was incomplete. It must not silently treat partial
//! validation as complete success.
//!
//! `None` in an optional budget means that this validation invocation has no
//! additional limit for that budget. It does not promise infinite memory,
//! execution time, representable identifiers, or computational capacity.
//!
//! Operational budgets are policy, not language-level maxima. They must not
//! impose universal ceilings on machine counts, integer widths, collection
//! sizes, tensor ranks, or other valid program constructs.
//!
//! # Determinism
//!
//! Determinism is inherited from each validator's documented contract.
//! This aggregation module must not reorder inputs, rewrite diagnostics,
//! conceal errors, or reinterpret a validator's result.
//!
//! # Integration
//!
//! 1. The parent `src/classical/mod.rs` declares `pub mod validation;`.
//! 2. The crate root exposes `classical` through `src/lib.rs` once the parent
//!    module is a valid Rust module and its declarations are reconciled.
//! 3. Frontend semantic analysis produces canonical semantic facts.
//! 4. Callers select the relevant validation APIs below.
//! 5. IR-specific verification remains owned by the corresponding canonical
//!    IR subsystem.
//! 6. Target feasibility and physical resource checks remain downstream.
//!
//! # Compatibility
//!
//! Existing functions remain available through their defining modules.
//! The root re-exports only established public APIs; it does not introduce
//! replacement signatures or duplicate error types.
//!
//! # Completion criteria
//!
//! This file is complete when:
//! - Every declared module exists and compiles in the crate.
//! - Every re-export resolves to its canonical definition.
//! - The parent module and crate root expose the intended public path.
//! - Existing validation tests and public consumers remain compatible.
//! - Incomplete validation is never reported as complete success.
//! - The supported formatting, compilation, test, and lint checks pass.
//!
//! The source file's presence alone does not establish repository integration
//! or production readiness.

// ---------------------------------------------------------------------------
// Canonical validation domains
// ---------------------------------------------------------------------------

/// Structural graph and reference validation.
///
/// This validator is separate from AST structural validation. A provider
/// adapter must expose the real graph and its canonical identities.
pub mod structural;

/// Canonical semantic type validation.
///
/// This module delegates type representation and structural type semantics
/// to the frontend semantic type system.
pub mod types;

/// Shape, dimension, shape-comparison, and broadcasting validation.
///
/// Symbolic dimensions remain symbolic when static proof is unavailable.
pub mod shapes;

/// Semantic effect-row and effect-contract validation.
///
/// Effect compatibility remains distinct from target capabilities and
/// authorization policy.
pub mod effects;

/// Resource requirement and resource constraint validation.
///
/// Resource checks use canonical semantic resource models and explicit
/// operational policies rather than universal machine-size limits.
pub mod resources;

// ---------------------------------------------------------------------------
// Stable public aggregation API
// ---------------------------------------------------------------------------

// Structural validation.
pub use self::structural::{
    validate_structure,
    validate_structure_with_config,
    StructuralBudget,
    StructuralGraph,
    StructuralNode,
    StructuralValidationConfig,
    StructuralValidationError,
    StructuralValidationStatistics,
};

// Semantic type validation.
pub use self::types::{
    validate_classical_type,
    validate_classical_type_with_config,
    validate_classical_types,
    ClassicalTypeValidationConfig,
    ClassicalTypeValidationError,
    TypeValidationStage,
};

// Shape validation.
pub use self::shapes::{
    broadcast_shapes,
    compare_dimensions,
    compare_shapes,
    validate_dimension,
    validate_shape,
    DimensionRelation,
    ShapeValidationError,
};

// Effect validation.
pub use self::effects::{
    validate_effect_contract,
    validate_effect_contract_with,
    validate_effect_contract_with_exact_matching,
    validate_effect_row,
    validate_effect_row_with_role,
    ClassicalEffectValidationError,
    ClassicalEffectValidationResult,
    EffectRowRole,
};

// Resource validation.
pub use self::resources::{
    ensure_valid,
    validate_resource_requirements,
    ResourceSemanticsValidator,
    ResourceValidationCompleteness,
    ResourceValidationError,
    ResourceValidationIssue,
    ResourceValidationIssueKind,
    ResourceValidationPolicy,
    ResourceValidationReport,
    ResourceValidationResult,
    ResourceValidationSubject,
};

// ---------------------------------------------------------------------------
// Aggregation-boundary contract tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_domains_are_registered() {
        // These references verify that the aggregation boundary exposes
        // the existing validator entry points.
        let _ = validate_structure::<TestGraph>;
        let _ = validate_classical_type;
        let _ = validate_classical_type_with_config;
        let _ = validate_classical_types;
        let _ = validate_shape;
        let _ = validate_dimension;
        let _ = compare_dimensions;
        let _ = compare_shapes;
        let _ = broadcast_shapes;
        let _ = validate_effect_row;
        let _ = validate_effect_row_with_role;
        let _ = validate_effect_contract;
        let _ = validate_effect_contract_with;
        let _ = validate_effect_contract_with_exact_matching;
        let _ = validate_resource_requirements;
        let _ = ensure_valid;
    }

    // This type is used only to ensure the structural validator's generic
    // entry point remains visible through the aggregation module.
    struct TestGraph;

    impl structural::StructuralGraph for TestGraph {
        type Id = u8;
        type Error = core::convert::Infallible;

        fn get_node(
            &self,
            _id: Self::Id,
        ) -> Result<Option<structural::StructuralNode<Self::Id>>, Self::Error> {
            Ok(None)
        }
    }

    #[test]
    fn unrestricted_resource_policy_adds_no_count_budget() {
        let policy = ResourceValidationPolicy::unbounded();

        assert_eq!(policy.max_resource_requirements(), None);
        assert_eq!(policy.max_resource_constraints(), None);
    }

    #[test]
    fn structural_policy_can_represent_unbounded_validation() {
        let policy = StructuralValidationConfig::unrestricted();

        // The unrestricted policy is a validation policy, not a promise
        // that the process has infinite memory or execution time.
        let _ = policy;
    }
}
