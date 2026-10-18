
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZCIR Verification Facade
//!
//! Path: `src/classical/zcir/verify/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Expose canonical ZUIR verification through the classical-facing ZCIR
//! namespace. ZCIR is an integration facade over the canonical IR, not a
//! second intermediate representation or verification implementation.
//!
//! ## Authority
//!
//! - `src/zuir/verify/mod.rs` owns the canonical verification API.
//! - `src/zuir/verify/full.rs` owns full-program verification orchestration.
//! - Individual `src/zuir/verify/*.rs` modules own their verification stages.
//! - `src/zuir/ir/` owns canonical IR entities and their contracts.
//! - `grammar/DESIGN.md` and `grammar/Zamani.g4` govern grammar architecture.
//! - `grammar/specification/semantic-model.md` governs semantic boundaries.
//! - `grammar/validation/semantic-boundaries.md` governs ownership boundaries.
//! - `grammar/specification/scalability-model.md` and
//!   `grammar/specification/poco-reaf.md` govern portability and scalability.
//!
//! ## Owns
//!
//! - The ZCIR verification namespace.
//! - Stable access to canonical verification modules and entry points.
//! - Documentation of verification-stage responsibilities.
//!
//! ## Does not own
//!
//! - A duplicate IR, identity system, type system, or effect model.
//! - Duplicate structural, dominance, effect, or resource algorithms.
//! - Source parsing, AST construction, or semantic name resolution.
//! - Hardware discovery, allocation, scheduling, routing, or execution.
//! - Claims that an unimplemented or incomplete verification stage succeeded.
//!
//! ## Verification contract
//!
//! Full verification is the canonical orchestration entry point. Individual
//! stages are available for callers that deliberately need focused checks.
//! Passing one stage does not establish that the entire program is valid.
//!
//! Callers must inspect the full report's status, validity, and incompleteness
//! using the actual canonical report contract. They must not equate absence
//! of reported errors with proof that unsupported or deferred checks passed.
//!
//! Resource requirements, capabilities, preferences, constraints, effects,
//! and target feasibility remain distinct concepts. Verification must not
//! weaken requirements or silently change program semantics to accommodate
//! a target.
//!
//! ## Scalability and safety
//!
//! This facade imposes no universal limit on program size, operation count,
//! region depth, resource quantity, device count, or hardware scale.
//! Actual scalability depends on canonical algorithms, available resources,
//! representation limits, and explicit operational budgets.
//!
//! Resource exhaustion or an unsupported verification capability must not
//! silently turn an unverified program into a verified program.
//! No unsafe Rust is permitted.
//!
//! ## Integration
//!
//! `src/classical/zcir/mod.rs` must register this module with `pub mod verify;`
//! once the ZCIR root has been converted into a valid Rust module.
//!
//! The canonical ZUIR module remains authoritative. New verification stages
//! should first be implemented and registered in ZUIR, then exposed here
//! deliberately if they belong in the stable ZCIR public API.
//!
//! ## Freeze conditions
//!
//! - Every re-export resolves to the actual canonical ZUIR API.
//! - No parallel verifier or competing canonical type is introduced.
//! - The ZCIR root registers this module correctly.
//! - Public API and integration tests compile.
//! - Formatting, workspace checks, tests, and Clippy pass.

/// Canonical full-program verification orchestration.
pub use crate::zuir::verify::full;

/// Canonical control-flow verification.
pub use crate::zuir::verify::control_flow;

/// Canonical dialect registration and operation-schema verification.
pub use crate::zuir::verify::dialects;

/// Canonical dominance analysis and verification.
pub use crate::zuir::verify::dominance;

/// Canonical effect verification.
pub use crate::zuir::verify::effects;

/// Canonical ownership verification.
pub use crate::zuir::verify::ownership;

/// Canonical resource-intent verification.
pub use crate::zuir::verify::resources;

/// Canonical structural verification.
pub use crate::zuir::verify::structural;

/// Canonical type verification.
pub use crate::zuir::verify::types;

// -----------------------------------------------------------------------------
// Stable top-level verification entry points
// -----------------------------------------------------------------------------
//
// These names match the existing public exports from `src/zuir/verify/mod.rs`.
// Re-exporting preserves the canonical input and report types; it introduces
// no conversion layer and no competing verifier.

pub use crate::zuir::verify::{
    verify_control_flow,
    verify_dialects,
    verify_dominance,
    verify_effects,
    verify_full,
    verify_ownership,
    verify_resources,
    verify_structural,
    verify_types,
};

// -----------------------------------------------------------------------------
// Public input and report contracts
// -----------------------------------------------------------------------------

pub use crate::zuir::verify::{
    ControlFlowInput,
    DialectIssueSeverity,
    DominanceInput,
    EffectVerificationInput,
    FullVerificationInput,
    FullVerificationReport,
    OwnershipVerificationInput,
    ResourceIntentEntry,
    ResourceIssueSeverity,
    StructuralInput,
    TypeVerificationInput,
    VerificationIssue,
    VerificationSeverity,
    VerificationStage,
    VerificationStatus,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zcir_entry_points_use_canonical_verifier_signatures() {
        let _: fn(&FullVerificationInput<'_>) -> FullVerificationReport =
            verify_full;

        let _: fn(&StructuralInput<'_>) -> _ = verify_structural;

        let _: fn(&ControlFlowInput<'_>) -> _ = verify_control_flow;

        let _: fn(&DominanceInput<'_>) -> _ = verify_dominance;

        let _: fn(&TypeVerificationInput<'_>) -> _ = verify_types;

        let _: fn(&EffectVerificationInput<'_>) -> _ = verify_effects;

        let _: fn(&OwnershipVerificationInput<'_>) -> _ = verify_ownership;

        let _: fn(&[ResourceIntentEntry<'_>]) -> _ = verify_resources;

        let _: fn(&_) -> _ = verify_dialects;
    }

    #[test]
    fn zcir_and_zuir_share_the_same_full_verification_types() {
        let canonical_input: Option<
            crate::zuir::verify::FullVerificationInput<'_>,
        > = None;

        let zcir_input: Option<FullVerificationInput<'_>> = canonical_input;

        let canonical_report: Option<
            crate::zuir::verify::FullVerificationReport,
        > = None;

        let zcir_report: Option<FullVerificationReport> = canonical_report;

        assert!(zcir_input.is_none());
        assert!(zcir_report.is_none());
    }
}
