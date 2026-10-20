
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Transformations
//!
//! Path: `src/hdl/hdlir/transform/mod.rs`
//! Rust edition: 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the composition root for transformations owned by HDLIR.
//! It registers existing transformation implementations and provides
//! deliberate public re-exports without duplicating their algorithms.
//!
//! ## Architectural authority
//!
//! Implementations exposed here must conform to:
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`.
//! - `grammar/hdl/hdl.g4` and applicable HDL delegate grammars.
//! - `grammar/hdl/hardware-generics.g4`.
//! - `grammar/hdl/hardware-modules.g4`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`.
//! - `src/hdl/hdlir/compatibility.md`.
//! - `src/hdl/hdlir/passes/contracts.rs`.
//! - `src/hdl/hdlir/passes/manager.rs`.
//! - `src/hdl/hdlir/passes/preservation.rs`.
//! - `src/zuir/ZUIR-SPEC.md`.
//!
//! The grammar owns syntax, the canonical frontend owns source semantics,
//! ZUIR owns the universal computational representation, and HDLIR owns
//! hardware-domain representations and transformations.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Registration of implemented transformation modules.
//! - The transformation subsystem's public re-export surface.
//! - Documentation of the integration and safety boundaries.
//!
//! This module does not own:
//!
//! - The design or operation graph.
//! - Source parsing, AST construction, or semantic validation.
//! - A competing pass manager or preservation framework.
//! - Hardware equivalence proofs.
//! - Target selection, resource allocation, synthesis, placement, routing,
//!   timing closure, or hardware execution.
//!
//! ## Registered transformations
//!
//! - `canonicalize`: generic, budget-accounted canonicalization through an
//!   explicit implementation contract.
//! - `simplify`: conservative expression-graph compaction from explicit
//!   observable roots.
//! - `dead_code`: dead-code planning from caller-supplied operation facts.
//! - `specialize`: deterministic parameter-dependency and specialization
//!   planning.
//!
//! These capabilities have deliberately different scopes. In particular,
//! a plan is not a graph mutation, and an invocation report is not proof of
//! semantic equivalence.
//!
//! ## Integration contract
//!
//! - Each implementation remains authoritative for its own data types,
//!   errors, validation rules, and operational budgets.
//! - Callers may use the fully qualified child-module paths or the curated
//!   re-exports below.
//! - The shared pass contracts, manager, and preservation framework remain
//!   under `crate::hdl::hdlir::passes`.
//! - A mutating pass must declare its preconditions, postconditions,
//!   analysis invalidation, and preservation obligations before it is
//!   registered with the pass manager.
//! - The caller must run the applicable HDLIR verification after a
//!   transformation whenever required by that transformation's contract.
//! - Unsupported operations, unresolved parameters, incomplete analysis,
//!   budget exhaustion, and unproven preservation must never be reported
//!   as successful completion.
//!
//! ## Scalability and safety
//!
//! This composition module imposes no fixed limit on hardware width,
//! operation count, signal count, module count, clock domains, memory size,
//! or physical resources.
//!
//! Operational budgets belong to the individual transformation or the
//! shared HDLIR configuration. A missing budget means no explicit limit
//! is imposed by that setting; it does not promise infinite time or memory.
//! Implementations must use checked accounting and report budget exhaustion
//! instead of returning a misleading successful result.
//!
//! No `unsafe` Rust is permitted.
//!
//! ## Stability
//!
//! This module exports only APIs that exist in the registered child files.
//! A transformation is not considered production-ready merely because it
//! is registered here. Its own implementation, tests, integration contracts,
//! and preservation obligations must also be complete.

/// Budget-accounted canonicalization framework.
///
/// This module provides an explicit canonicalization contract. A concrete
/// design must implement [`Canonicalizable`] before this framework can
/// canonicalize that design.
pub mod canonicalize;

/// Conservative expression-graph compaction.
///
/// This transformation preserves retained expression kinds and operand
/// order. The caller is responsible for supplying all required observable
/// expression roots.
pub mod simplify;

/// Dead-code analysis and removal planning.
///
/// This module produces a plan from supplied operation facts; it does not
/// independently rewrite a complete hardware design.
pub mod dead_code;

/// Parameter-specialization planning.
///
/// This module computes a deterministic dependency order and reports
/// unresolved planning conditions. It does not mutate a design graph.
pub mod specialize;

// -----------------------------------------------------------------------------
// Canonicalization API
// -----------------------------------------------------------------------------

pub use canonicalize::{
    run_canonicalization,
    Canonicalizable,
    CanonicalizationBudget,
    CanonicalizationContext,
    CanonicalizationError,
    CanonicalizationReport,
};

// -----------------------------------------------------------------------------
// Expression simplification API
// -----------------------------------------------------------------------------

pub use simplify::{
    simplify_expressions,
    SimplifiedExpressions,
    SimplifyError,
    SimplifyResult,
    SimplifyStats,
};

// -----------------------------------------------------------------------------
// Dead-code planning API
// -----------------------------------------------------------------------------

pub use dead_code::{
    plan_dead_code,
    DeadCodeConfig,
    DeadCodePlan,
    DeadCodeStats,
    OperationEffectKnowledge,
    OperationFacts,
};

// -----------------------------------------------------------------------------
// Parameter-specialization planning API
// -----------------------------------------------------------------------------

pub use specialize::{
    plan_specialization,
    SpecializationConfig,
    SpecializationError,
    SpecializationPlan,
    SpecializationStats,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implemented_transformation_modules_are_registered() {
        // Compile-time references ensure the intended public entry points
        // remain available through this composition module.
        let _canonicalization = run_canonicalization::<TestCanonicalizable>;
        let _simplification = simplify_expressions;
        let _dead_code_planning = plan_dead_code;
        let _specialization_planning = plan_specialization;
    }

    struct TestCanonicalizable;

    impl Canonicalizable for TestCanonicalizable {
        fn canonicalize(
            &mut self,
            _context: &mut CanonicalizationContext,
        ) -> Result<CanonicalizationReport, CanonicalizationError> {
            Ok(CanonicalizationReport::new())
        }
    }

    #[test]
    fn canonicalization_composition_entry_point_is_callable() {
        let mut design = TestCanonicalizable;

        let report = run_canonicalization(
            &mut design,
            CanonicalizationBudget::unlimited(),
        )
        .expect("the empty test canonicalization should succeed");

        assert_eq!(report.work_used, 0);
    }
}
