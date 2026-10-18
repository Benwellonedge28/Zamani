
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # ZCIR Effect Integration
//!
//! Canonical path: `src/classical/zcir/ir/effects.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Expose the canonical ZUIR effect model through the ZCIR namespace.
//! Classical operations use the same effect identities, effect sets,
//! summaries, policies, and validation contracts as other computation
//! domains in Zamani.
//!
//! This module is deliberately a facade. It does not implement a second
//! effect system or maintain a separate copy of an operation's effects.
//!
//! ## Authority
//!
//! - `src/zuir/ir/effects.rs` — canonical effect definitions, sets, summaries,
//!   policies, validation and operation-effect adapters.
//! - `src/zuir/ir/operation.rs` — canonical operation effect declarations.
//! - `src/zuir/ir/function.rs` — canonical function-level effect declarations.
//! - `src/zuir/schema/types.rs` — canonical `QualifiedName`.
//! - `src/zuir/ZUIR-SPEC.md` — universal IR contract.
//! - `grammar/DESIGN.md` — grammar authority and architectural boundaries.
//! - `grammar/Zamani.g4` — source grammar composition.
//! - `grammar/functions/functions.g4` — function syntax boundary.
//! - `grammar/specification/semantic-model.md` — semantic ownership.
//! - `grammar/specification/scalability-model.md` — resource-parametric scale.
//! - `grammar/specification/poco-reaf.md` — portability and compatibility.
//! - `grammar/validation/semantic-boundaries.md` — normative boundaries.
//!
//! Grammar defines syntax, not the executable meaning of effects. The
//! frontend and semantic model resolve source-level declarations; ZUIR
//! represents canonical operation effects; verification and analysis
//! establish whether declarations are valid and complete.
//!
//! ## Owns
//!
//! - ZCIR-facing public aliases for canonical effect types.
//! - The namespace boundary through which classical consumers access them.
//! - Adapter-level tests ensuring that canonical effects remain interoperable.
//!
//! ## Does not own
//!
//! - A second effect-name representation or effect registry.
//! - A new closed list of classical effects.
//! - Source parsing, name resolution, type checking, or effect inference.
//! - The authoritative operation or function representation.
//! - Interprocedural effect analysis or proof of effect completeness.
//! - Control-flow, dominance, alias, or memory-dependence analysis.
//! - Runtime effect enforcement, target selection, or hardware scheduling.
//! - Capability discovery or physical resource allocation.
//!
//! ## Semantic contract
//!
//! `ZcirEffectSet` is the canonical `EffectSet`. Effect names reuse the
//! canonical `QualifiedName` representation.
//!
//! An effect set represents a collection of known effect names. It does not
//! itself establish that the collection is complete. `ZcirEffectSummary`
//! carries an explicit completeness state.
//!
//! In particular:
//!
//! - An empty, incomplete summary does not prove purity.
//! - Unknown effect names must not silently be treated as harmless.
//! - An effect name is not a capability or a resource requirement.
//! - A deterministic set iteration order does not define execution order.
//! - Transformations must preserve observable behavior, dependencies,
//!   synchronization, exceptions and other applicable semantic guarantees.
//! - An effect declaration does not prove that an implementation actually
//!   obeys the declaration.
//!
//! Registry-backed validation must use an authoritative dialect/effect
//! definition source. This facade does not invent definitions for unknown
//! effects and does not execute extension code.
//!
//! ## Integration contract
//!
//! - `ir/operation.rs` remains the canonical operation-effect storage boundary.
//! - `ir/function.rs` remains the canonical function-effect storage boundary.
//! - `verify/effects.rs` should check operation declarations against registered
//!   effect definitions and applicable policies.
//! - `analysis/effects.rs`, if introduced after repository audit, should infer
//!   effects across reachable operations and callees while tracking whether
//!   the result is complete.
//! - `passes/legality.rs` must reject transformations that violate declared
//!   effects, observable ordering, synchronization, or relevant dependencies.
//! - `lowering/to_zuir.rs` must preserve canonical effect declarations.
//! - Serialization must preserve effect names and summary completeness where
//!   the selected format represents summaries.
//! - Quantum/classical integration must preserve shared effect identities and
//!   not reinterpret measurement, I/O, memory, or synchronization effects.
//!
//! The verifier and analysis modules are responsible for whole-program
//! guarantees. Merely importing this module does not validate an IR graph.
//!
//! ## Scalability and POCO-REAF
//!
//! This facade imposes no fixed universal ceiling on effect names, operations,
//! functions, devices, threads, or other resource populations. It introduces
//! no fixed-size effect arrays and does not allocate a second effect registry.
//!
//! The canonical implementation uses dynamically sized collections. Actual
//! operations remain subject to available memory, address space, allocator
//! behavior, representation limits and explicit operational budgets.
//!
//! POCO-REAF does not mean literal infinite execution. It requires portable
//! semantics and explicit reporting of unsupported effects, target
//! incompatibility and resource exhaustion rather than silently changing
//! program meaning.
//!
//! ## Freeze criteria
//!
//! This file may be frozen when:
//!
//! 1. Its parent module registers it without duplicate effect exports.
//! 2. The canonical ZUIR effect API remains authoritative.
//! 3. Adapter tests compile and pass against the repository's actual API.
//! 4. Effect verification, analysis, lowering and serialization are integrated.
//! 5. Formatting, compilation, tests and lint gates pass.
//!
//! Passing the local tests does not by itself establish production readiness
//! for the entire ZCIR subsystem or every POCO-REAF target.

// -----------------------------------------------------------------------------
// Canonical effect API
// -----------------------------------------------------------------------------

/// Canonical effect result and structured error types.
pub use crate::zuir::ir::effects::{
    EffectError,
    EffectResult,
};

/// Canonical effect classifications and effect-definition properties.
pub use crate::zuir::ir::effects::{
    EffectClass,
    EffectDefinition,
    EffectProperties,
};

/// Canonical deterministic effect set.
pub use crate::zuir::ir::effects::EffectSet;

/// Canonical effect-summary completeness state and summary representation.
pub use crate::zuir::ir::effects::{
    EffectCompleteness,
    EffectSummary,
};

/// Canonical policy for allowed, forbidden, required, and registered effects.
pub use crate::zuir::ir::effects::EffectPolicy;

/// Canonical adapters for validating and reading operation effect declarations.
pub use crate::zuir::ir::effects::{
    declared_operation_effects,
    validate_operation_effects,
};

// -----------------------------------------------------------------------------
// ZCIR-facing names
// -----------------------------------------------------------------------------

/// ZCIR-facing alias for the canonical effect error.
pub type ZcirEffectError = EffectError;

/// ZCIR-facing alias for canonical effect-operation results.
pub type ZcirEffectResult<T> = EffectResult<T>;

/// ZCIR-facing alias for the canonical effect classification.
pub type ZcirEffectClass = EffectClass;

/// ZCIR-facing alias for the canonical effect properties.
pub type ZcirEffectProperties = EffectProperties;

/// ZCIR-facing alias for the canonical effect definition.
pub type ZcirEffectDefinition = EffectDefinition;

/// ZCIR-facing alias for the canonical effect set.
pub type ZcirEffectSet = EffectSet;

/// ZCIR-facing alias for the canonical completeness state.
pub type ZcirEffectCompleteness = EffectCompleteness;

/// ZCIR-facing alias for the canonical effect summary.
pub type ZcirEffectSummary = EffectSummary;

/// ZCIR-facing alias for the canonical effect policy.
pub type ZcirEffectPolicy = EffectPolicy;

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zuir::ids::OperationId;
    use crate::zuir::ir::operation::ZuirOperation;
    use crate::zuir::schema::types::QualifiedName;

    fn name(parts: &[&str]) -> QualifiedName {
        QualifiedName::new(
            parts
                .iter()
                .map(|part| (*part).to_owned())
                .collect::<Vec<_>>(),
        )
        .expect("test qualified name must be valid")
    }

    fn operation() -> ZuirOperation {
        ZuirOperation::new(
            OperationId::new(1).expect("test operation ID must be nonzero"),
            name(&["classical", "load"]),
        )
        .expect("test operation must be valid")
    }

    #[test]
    fn zcir_effect_types_are_canonical_zuir_types() {
        fn accepts_zcir_set(_: ZcirEffectSet) {}
        fn accepts_zuir_set(_: EffectSet) {}

        let set = ZcirEffectSet::new();

        accepts_zcir_set(set.clone());
        accepts_zuir_set(set);

        assert_eq!(
            core::any::type_name::<ZcirEffectSet>(),
            core::any::type_name::<EffectSet>(),
        );
        assert_eq!(
            core::any::type_name::<ZcirEffectSummary>(),
            core::any::type_name::<EffectSummary>(),
        );
        assert_eq!(
            core::any::type_name::<ZcirEffectPolicy>(),
            core::any::type_name::<EffectPolicy>(),
        );
    }

    #[test]
    fn effect_set_preserves_canonical_validation_and_determinism() {
        let read = name(&["memory", "read"]);
        let write = name(&["memory", "write"]);

        let set = ZcirEffectSet::from_names([
            write.clone(),
            read.clone(),
            write.clone(),
        ])
        .expect("valid effect names should be accepted");

        assert_eq!(set.len(), 2);
        assert!(set.contains(&read));
        assert!(set.contains(&write));

        let ordered: Vec<String> =
            set.iter().map(ToString::to_string).collect();

        let mut sorted = ordered.clone();
        sorted.sort();

        assert_eq!(ordered, sorted);
        assert!(set.validate().is_ok());
    }

    #[test]
    fn an_empty_unknown_summary_does_not_prove_purity() {
        let summary = ZcirEffectSummary::unknown();

        assert!(!summary.is_complete());
        assert!(summary.is_proven_pure().is_err());
    }

    #[test]
    fn only_a_complete_empty_summary_can_prove_purity() {
        let summary =
            ZcirEffectSummary::complete(ZcirEffectSet::new());

        assert_eq!(summary.is_proven_pure(), Ok(true));
    }

    #[test]
    fn a_complete_nonempty_summary_is_not_pure() {
        let mut effects = ZcirEffectSet::new();

        effects
            .insert(name(&["io", "write"]))
            .expect("valid effect name should be accepted");

        let summary = ZcirEffectSummary::complete(effects);

        assert_eq!(summary.is_proven_pure(), Ok(false));
    }

    #[test]
    fn operation_effect_adapter_reads_canonical_declarations() {
        let mut op = operation();
        let io = name(&["io", "write"]);

        op.add_effect(io.clone())
            .expect("valid effect declaration should be accepted");

        let effects = ZcirEffectSet::from_operation(&op)
            .expect("canonical operation effects should be valid");

        assert_eq!(effects.len(), 1);
        assert!(effects.contains(&io));
        assert!(validate_operation_effects(&op).is_ok());
    }

    #[test]
    fn declared_operation_effects_are_not_mistaken_for_complete_analysis() {
        let mut op = operation();

        op.add_effect(name(&["memory", "read"]))
            .expect("valid effect declaration should be accepted");

        let summary = declared_operation_effects(&op)
            .expect("declared effect names should be valid");

        assert_eq!(
            summary.completeness(),
            ZcirEffectCompleteness::Partial,
        );
        assert!(summary.effects().contains(
            &name(&["memory", "read"]),
        ));
    }

    #[test]
    fn effect_policy_rejects_forbidden_effects() {
        let forbidden = name(&["io", "write"]);
        let mut policy = ZcirEffectPolicy::permissive();

        policy
            .forbid(forbidden.clone())
            .expect("valid effect name should be accepted");

        let declared = ZcirEffectSet::from_names([forbidden])
            .expect("valid effect name should be accepted");

        assert!(matches!(
            policy.validate(&declared),
            Err(ZcirEffectError::ForbiddenEffect { .. }),
        ));
    }

    #[test]
    fn unknown_registered_effects_are_not_silently_accepted() {
        let custom = name(&["vendor", "special_effect"]);
        let declared = ZcirEffectSet::from_names([custom])
            .expect("qualified extension name should be valid");

        let policy = ZcirEffectPolicy::permissive();

        assert!(matches!(
            policy.validate_with_registry(&declared, |_| None),
            Err(ZcirEffectError::UnknownEffect { .. }),
        ));
    }
}
