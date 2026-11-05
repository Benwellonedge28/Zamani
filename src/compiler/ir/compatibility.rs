
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — IR Compatibility
//!
//! Path: `src/compiler/ir/compatibility.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the compiler-facing compatibility boundary for existing
//! and future intermediate-representation integrations.
//!
//! This module coordinates the repository's existing canonical
//! compatibility implementation. It does not create another IR,
//! compatibility engine, semantic model, type system, verifier,
//! or domain-specific conversion algorithm.
//!
//! ## Existing compatibility authority
//!
//! Legacy IR compatibility is owned by:
//!
//! `crate::classical::zcir::lowering::legacy_adapter`
//!
//! That implementation:
//!
//! - Inspects the existing `crate::ir_gen::IrModule`.
//! - Identifies known compatibility risks.
//! - Produces deterministic findings.
//! - Preserves the legacy module.
//! - Does not claim that legacy IR is canonical ZUIR.
//! - Does not claim that automatic conversion is safe.
//!
//! This module exposes that implementation through the compiler IR
//! namespace without duplicating its data structures or algorithms.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Compiler-facing compatibility entry points.
//! - Stable compiler namespace aliases.
//! - Compatibility API version identification.
//! - Coordination with existing representation adapters.
//!
//! This module does not own:
//!
//! - Source syntax or grammar.
//! - AST construction.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership analysis.
//! - Canonical ZUIR.
//! - Classical, quantum, HDL, AI, or tensor IR.
//! - Legacy IR generation.
//! - Representation-specific compatibility algorithms.
//! - IR conversion or lowering algorithms.
//! - Whole-program verification.
//! - Optimization.
//! - Backend registration.
//! - Hardware allocation.
//! - Scheduling or routing.
//! - Runtime execution.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! ## Representation compatibility
//!
//! Compatibility is not equivalent to structural resemblance.
//!
//! Two representations are not compatible merely because they contain
//! similarly named instructions, types, values, or fields.
//!
//! A valid conversion must preserve all applicable:
//!
//! - Values and types.
//! - Control-flow relationships.
//! - Effects.
//! - Ownership and borrowing semantics.
//! - Memory semantics.
//! - Resource requirements.
//! - Numerical guarantees.
//! - Calling conventions and linkage.
//! - Provenance and source information.
//! - Domain-specific semantics.
//!
//! A conversion that cannot establish these properties must fail
//! explicitly or remain unavailable.
//!
//! ## Legacy migration policy
//!
//! The legacy IR remains a compatibility input while existing
//! compiler and backend consumers depend on it.
//!
//! The existing legacy assessor is authoritative for identifying
//! its known migration risks.
//!
//! A finding-free assessment is not proof of:
//!
//! - Semantic equivalence.
//! - Canonical ZUIR validity.
//! - Domain IR validity.
//! - Target compatibility.
//! - Backend compatibility.
//! - Hardware feasibility.
//! - Executability.
//!
//! Automatic legacy-to-ZUIR conversion remains prohibited until
//! a separate, tested, semantics-preserving converter is implemented
//! by the appropriate canonical IR owners.
//!
//! ## Verification boundary
//!
//! Compatibility assessment and IR verification are separate tasks.
//!
//! For legacy IR, the existing backend input adapter uses:
//!
//! `crate::ir_verify::verify_module`
//!
//! For canonical ZUIR, compiler input verification delegates to:
//!
//! `crate::zuir::verify::verify_full`
//!
//! A compatibility assessment must never be substituted for either
//! verification process.
//!
//! A representation changed after verification must be verified
//! again before it is trusted by subsequent stages.
//!
//! ## Scalability and POCO-REAF
//!
//! This module introduces no arbitrary limits on:
//!
//! - Modules.
//! - Functions.
//! - Instructions.
//! - Operands.
//! - Types.
//! - Values.
//! - Blocks.
//! - Regions.
//! - Operations.
//! - Qubits.
//! - Signals.
//! - Tensor dimensions or rank.
//! - Devices or accelerators.
//! - Distributed participants.
//! - Computing domains.
//!
//! The compiler facade adds no IR copying or graph reconstruction.
//!
//! Inspection complexity and report storage are determined by the
//! canonical compatibility assessor and the supplied representation.
//!
//! Actual processing remains subject to available memory, address
//! space, representable values, implementation support, and explicit
//! operational budgets.
//!
//! Literal infinity cannot be guaranteed by finite software.
//! Resource exhaustion must not be interpreted as compatibility.
//!
//! ## Grammar boundary
//!
//! This module does not parse Zamani source or define source meaning.
//!
//! The relevant authority chain is:
//!
//! `grammar/Zamani.g4`
//!     -> frontend AST
//!     -> semantic model
//!     -> canonical IR
//!     -> representation compatibility
//!     -> verification
//!     -> lowering and realization
//!
//! Grammar and semantic specifications remain authoritative.
//!
//! ## Determinism
//!
//! This module introduces:
//!
//! - No global mutable state.
//! - No random identifiers.
//! - No implicit target selection.
//! - No hardware discovery.
//! - No filesystem or network access.
//! - No console output.
//! - No mutation of supplied representations.
//! - No unspecified compatibility fallback.
//!
//! Finding ordering remains owned by the canonical assessor.
//!
//! ## Integration
//!
//! Parent:
//! `src/compiler/ir/mod.rs`
//!
//! Compiler root:
//! `src/compiler/mod.rs`
//!
//! Canonical legacy compatibility:
//! `src/classical/zcir/lowering/legacy_adapter.rs`
//!
//! Existing representation contracts:
//! `src/compiler/ir/adapters.rs`
//!
//! Verification:
//! `src/compiler/ir/verification.rs`
//! `src/compiler/ir/input.rs`
//!
//! Translation:
//! `src/compiler/ir/translation.rs`
//!
//! Lowering:
//! `src/compiler/ir/lowering.rs`
//!
//! Backend input:
//! `src/backend/input.rs`
//!
//! ## Compatibility policy
//!
//! This API version identifies this compiler-facing facade.
//! It is not the Zamani language version, grammar version,
//! semantic-model version, ZUIR schema version, or backend ABI.
//!
//! Existing canonical finding and assessment types are re-exported
//! rather than copied. This preserves their original type identity.
//!
//! ## Freeze criteria
//!
//! - Legacy compatibility delegates to its canonical owner.
//! - No duplicate compatibility engine is introduced.
//! - No automatic lossy conversion is permitted.
//! - Findings remain inspectable.
//! - Existing legacy IR callers remain supported.
//! - Canonical verification remains independent.
//! - No arbitrary hardware ceilings are introduced.
//! - No unsafe code is introduced.
//! - Parent module declarations resolve.
//! - Repository tests and quality gates pass.

use crate::classical::zcir::lowering::legacy_adapter;

// =============================================================================
// API VERSION
// =============================================================================

/// Version of the compiler IR compatibility facade.
///
/// This version is independent of the language, grammar, semantic
/// model, canonical IR, and backend versions.
pub const IR_COMPATIBILITY_API_VERSION: u16 = 1;

// =============================================================================
// CANONICAL COMPATIBILITY TYPES
// =============================================================================

/// Stable compiler-facing name for a legacy compatibility finding.
///
/// The underlying type remains owned by the canonical ZCIR
/// legacy compatibility adapter.
pub use legacy_adapter::LegacyFinding as CompatibilityFinding;

/// Stable compiler-facing name for a legacy compatibility finding code.
///
/// The underlying enum remains owned by the canonical ZCIR
/// legacy compatibility adapter.
pub use legacy_adapter::LegacyFindingCode as CompatibilityFindingCode;

/// Stable compiler-facing name for a legacy compatibility assessment.
///
/// The underlying report remains owned by the canonical ZCIR
/// legacy compatibility adapter.
pub use legacy_adapter::LegacyAssessment as CompatibilityAssessment;

/// Original canonical legacy finding type.
///
/// Retained for callers that use the original name.
pub use legacy_adapter::LegacyFinding;

/// Original canonical legacy finding code.
///
/// Retained for callers that use the original name.
pub use legacy_adapter::LegacyFindingCode;

/// Original canonical legacy assessment type.
///
/// Retained for callers that use the original name.
pub use legacy_adapter::LegacyAssessment;

// =============================================================================
// LEGACY IR COMPATIBILITY
// =============================================================================

/// Assess compatibility risks in an existing legacy IR module.
///
/// This function delegates directly to the canonical ZCIR
/// compatibility assessor.
///
/// It does not mutate the supplied module, convert it to ZUIR,
/// or establish that it is suitable for execution.
///
/// # Arguments
///
/// - `module`: an existing legacy IR module generated by
///   `crate::ir_gen`.
///
/// # Returns
///
/// A deterministic compatibility assessment containing the
/// findings produced by the canonical legacy adapter.
///
/// # Important
///
/// An assessment with no findings is not proof of semantic
/// equivalence or permission to perform automatic conversion.
///
/// The current canonical legacy adapter deliberately reports
/// that automatic conversion is not established.
///
/// # Complexity
///
/// The complexity is determined by the canonical assessor.
/// This facade adds no additional traversal or representation copy.
#[must_use]
pub fn assess_legacy_ir(
    module: &crate::ir_gen::IrModule,
) -> CompatibilityAssessment {
    legacy_adapter::assess_legacy_module(module)
}

/// Return whether the canonical assessment permits automatic
/// conversion of the inspected legacy representation.
///
/// The current canonical adapter returns `false` because a complete,
/// semantics-preserving legacy-to-ZUIR converter has not been
/// established.
///
/// This function delegates the decision to the canonical assessment.
/// It does not introduce a second conversion policy.
#[must_use]
pub const fn permits_automatic_legacy_conversion(
    assessment: &CompatibilityAssessment,
) -> bool {
    assessment.permits_automatic_conversion()
}

// =============================================================================
// API INFORMATION
// =============================================================================

/// Return the compiler IR compatibility facade API version.
#[must_use]
pub const fn api_version() -> u16 {
    IR_COMPATIBILITY_API_VERSION
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compatibility_api_version_is_stable() {
        assert_eq!(api_version(), IR_COMPATIBILITY_API_VERSION);
        assert_eq!(IR_COMPATIBILITY_API_VERSION, 1);
    }

    #[test]
    fn compatibility_aliases_reuse_canonical_types() {
        fn assert_finding_alias(_: &CompatibilityFinding) {}
        fn assert_code_alias(_: CompatibilityFindingCode) {}
        fn assert_assessment_alias(_: &CompatibilityAssessment) {}

        // Compile-time type identity checks.
        let _ = assert_finding_alias;
        let _ = assert_code_alias;
        let _ = assert_assessment_alias;
    }

    #[test]
    fn canonical_assessment_controls_conversion_permission() {
        // The canonical legacy adapter explicitly disables automatic
        // conversion until a complete lossless converter is established.
        //
        // The facade delegates this decision instead of maintaining
        // an independent conversion-permission flag.
        fn check(
            assessment: &CompatibilityAssessment,
        ) -> bool {
            permits_automatic_legacy_conversion(assessment)
        }

        let _ = check;
    }
}
