
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZCIR Effect Verification
//!
//! Path: `src/classical/zcir/verify/effects.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Expose canonical ZUIR effect verification through the ZCIR classical
//! computing boundary.
//!
//! ZCIR is a classical-facing dialect and integration facade over ZUIR.
//! It must not introduce a competing effect system, verifier, effect-name
//! registry, or interpretation of operation effects.
//!
//! ## Authority
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ir/effects.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/function.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/program.rs`
//! - `src/zuir/ir/module.rs`
//! - `src/zuir/verify/effects.rs`
//! - `src/zuir/verify/full.rs`
//! - `src/zuir/dialect/registry.rs`
//! - `src/zuir/schema/types.rs`
//! - `src/classical/zcir/ir/effects.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The ZCIR-facing effect-verification API.
//! - Stable aliases for the canonical verifier's public input, report,
//!   and diagnostic types.
//! - A descriptive entry point for classical effect verification.
//!
//! The canonical ZUIR implementation owns:
//!
//! - Effect verification algorithms and diagnostic codes.
//! - Effect declaration and function effect-coverage checks.
//! - Validation of the supplied effect-definition environment.
//! - Canonical effect names and definitions.
//! - Traversal of functions, regions, blocks, and operations.
//!
//! ## Does not own
//!
//! This module does not implement:
//!
//! - Source parsing, AST construction, name resolution, or effect inference.
//! - Effect identity, effect properties, or effect policy definitions.
//! - Dialect registration or trust decisions.
//! - Whole-program interprocedural effect inference.
//! - Structural, control-flow, dominance, SSA, type, ownership, or resource
//!   verification.
//! - Target capability checks, physical resource allocation, or scheduling.
//! - Runtime enforcement or proof that an implementation obeys its effects.
//!
//! ## Semantic invariants
//!
//! 1. Unknown effects must not silently be accepted as harmless.
//! 2. An empty effect set does not establish that effect information is
//!    complete or that a function is pure.
//! 3. Declared effects must remain intact through optimization, lowering,
//!    serialization, and classical/quantum interoperability.
//! 4. Effect sets do not define execution order.
//! 5. Function effect coverage is not a substitute for interprocedural
//!    effect inference or verification of external implementations.
//! 6. An effect is not a target capability or resource requirement.
//! 7. A successful report establishes only the checks implemented by the
//!    canonical verifier.
//!
//! ## Integration contract
//!
//! The caller must supply the canonical program and complete entity view
//! required by `crate::zuir::verify::effects::EffectVerificationInput`.
//!
//! The effect-definition map must come from the authoritative registry or
//! another explicitly trusted definition source. A syntactically valid
//! qualified name does not prove that the corresponding effect is registered.
//!
//! The intended pipeline is:
//!
//! ```text
//! Frontend and semantic analysis
//!              |
//!              v
//! Canonical ZUIR construction
//!              |
//!              v
//! Structural and dialect verification
//!              |
//!              v
//! Type and control-flow verification
//!              |
//!              v
//! Canonical effect verification
//!              |
//!              v
//! Ownership and remaining verification
//!              |
//!              v
//! Full-program verification
//!              |
//!              v
//! Legal optimization and lowering
//! ```
//!
//! The full-verification orchestrator owns the actual stage order. This
//! documentation does not override the implementation of that orchestrator.
//!
//! ## Scalability and POCO-REAF
//!
//! This adapter introduces no fixed effect-count, operation-count,
//! function-count, region-count, device-count, processor-count, or memory
//! ceiling.
//!
//! It adds no allocation proportional to program size. All traversal,
//! indexing, diagnostics, and worklist costs are those of the canonical
//! verifier.
//!
//! Scalability remains subject to finite address space, available memory,
//! computation time, configured compilation budgets, and the capabilities
//! of the selected implementation.
//!
//! No finite machine can guarantee literal infinite computation. POCO-REAF
//! requires avoiding unjustified universal capacity limits and preserving
//! program meaning across compatible realizations.
//!
//! ## Error handling
//!
//! This facade does not discard, rewrite, or downgrade canonical findings.
//! Callers must inspect `is_valid()` and the returned diagnostics before
//! relying on effect verification.
//!
//! Allocation failure, incomplete input, and other operational failures
//! must not be represented as successful proof. The canonical verifier's
//! current result contract must be respected; this facade does not claim
//! to add failure modes that its underlying API cannot represent.
//!
//! ## Compatibility
//!
//! The canonical API remains available through this module. ZCIR-specific
//! aliases are provided without creating new runtime types.
//!
//! Changes to the canonical input, report, or verifier signature must be
//! reviewed alongside this facade and its downstream callers.
//!
//! ## Freeze criteria
//!
//! - The public types are identical to their canonical ZUIR counterparts.
//! - No second effect-verification algorithm is introduced.
//! - The existing operation and function effect representations remain
//!   authoritative.
//! - Parent modules register this file only after they exist.
//! - Cross-repository integration and test gates pass.
//! - No unsafe Rust or universal hardware-size limits are introduced.

// =============================================================================
// Canonical API
// =============================================================================

/// Canonical effect-verification input.
///
/// The input includes the canonical program, regions, blocks, operations,
/// and trusted effect-definition environment.
pub use crate::zuir::verify::effects::EffectVerificationInput;

/// Canonical effect-verification diagnostic.
pub use crate::zuir::verify::effects::EffectIssue;

/// Canonical effect-verification report.
pub use crate::zuir::verify::effects::EffectVerificationReport;

/// Canonical effect-verification entry point.
///
/// This is the authoritative implementation. It is re-exported rather
/// than wrapped in a second verification algorithm.
pub use crate::zuir::verify::effects::verify_effects;

// =============================================================================
// ZCIR-facing API aliases
// =============================================================================

/// ZCIR-facing alias for the canonical effect-verification input.
///
/// This is a type alias, not a separate representation. The caller must
/// provide the complete canonical verification view expected by ZUIR.
pub type ZcirEffectVerificationInput<'a> = EffectVerificationInput<'a>;

/// ZCIR-facing alias for a canonical effect-verification diagnostic.
pub type ZcirEffectIssue = EffectIssue;

/// ZCIR-facing alias for the canonical effect-verification report.
pub type ZcirEffectVerificationReport = EffectVerificationReport;

// =============================================================================
// Classical-facing entry point
// =============================================================================

/// Verify classical effect declarations using canonical ZUIR semantics.
///
/// This entry point intentionally delegates to the canonical ZUIR verifier.
/// It does not filter effects to a closed list of classical effect names:
/// shared operations, extension-defined effects, and cross-domain effects
/// must retain their canonical identities.
///
/// # Arguments
///
/// * `input` — The complete canonical verification view and trusted effect
///   definitions required by ZUIR.
///
/// # Returns
///
/// The canonical report, including all findings and verification counters.
///
/// # Important
///
/// A valid report is not proof that the complete program is valid, that an
/// effect declaration is complete, or that an implementation obeys its
/// declaration. Other required verification stages remain necessary.
///
/// # Example
///
/// ```ignore
/// use crate::classical::zcir::verify::effects::{
///     verify_zcir_effects,
///     ZcirEffectVerificationInput,
/// };
///
/// let input = ZcirEffectVerificationInput {
///     program: &program,
///     regions: &regions,
///     blocks: &blocks,
///     operations: &operations,
///     effect_definitions: &effect_definitions,
/// };
///
/// let report = verify_zcir_effects(&input);
///
/// if !report.is_valid() {
///     for issue in report.issues() {
///         eprintln!("{issue}");
///     }
/// }
/// ```
///
/// The example assumes that all referenced entities have already been
/// constructed using the actual canonical ZUIR APIs.
#[must_use]
pub fn verify_zcir_effects(
    input: &ZcirEffectVerificationInput<'_>,
) -> ZcirEffectVerificationReport {
    verify_effects(input)
}

// =============================================================================
// API contract tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// The ZCIR-facing aliases must remain the exact canonical types.
    ///
    /// These compile-time assignments catch accidental replacement of an
    /// alias with a second, incompatible representation.
    #[test]
    fn zcir_effect_api_uses_canonical_types() {
        let canonical_input: Option<EffectVerificationInput<'_>> = None;
        let zcir_input: Option<ZcirEffectVerificationInput<'_>> =
            canonical_input;

        let canonical_report: Option<EffectVerificationReport> = None;
        let zcir_report: Option<ZcirEffectVerificationReport> =
            canonical_report;

        let canonical_issue: Option<EffectIssue> = None;
        let zcir_issue: Option<ZcirEffectIssue> = canonical_issue;

        assert!(zcir_input.is_none());
        assert!(zcir_report.is_none());
        assert!(zcir_issue.is_none());
    }

    /// Both public entry points must accept the same canonical input and
    /// return the same canonical report type.
    #[test]
    fn canonical_and_zcir_entry_points_share_the_contract() {
        let canonical_entry: fn(
            &EffectVerificationInput<'_>,
        ) -> EffectVerificationReport = verify_effects;

        let zcir_entry: fn(
            &ZcirEffectVerificationInput<'_>,
        ) -> ZcirEffectVerificationReport = verify_zcir_effects;

        // Function pointers are checked for API compatibility here.
        // End-to-end behavior is covered by the canonical verifier tests.
        let _ = canonical_entry;
        let _ = zcir_entry;
    }
}
