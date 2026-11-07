#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Intermediate Representation Integration
//!
//! Path: `src/compiler/ir/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! This module is the composition root for compiler-facing
//! intermediate-representation integration.
//!
//! It provides stable module paths for the existing compiler
//! IR integration components:
//!
//! - Canonical IR input admission.
//! - Representation adapter contracts.
//! - Legacy representation compatibility assessment.
//! - Canonical IR verification integration.
//! - Representation translation orchestration.
//! - Verified IR lowering orchestration.
//!
//! This module composes existing implementations. It does not
//! implement another intermediate representation, semantic
//! model, verifier, adapter engine, lowering engine, optimizer,
//! backend, scheduler, or runtime.
//!
//! # Architectural ownership
//!
//! The compiler IR integration layer owns:
//!
//! - Composition of compiler-facing IR integration modules.
//! - Stable public module paths.
//! - The compiler IR facade API version.
//! - Documentation of integration boundaries.
//!
//! It does not own:
//!
//! - Source grammar, parsing, or AST construction.
//! - Source-level semantic analysis.
//! - Canonical IR definitions or domain-specific IR definitions.
//! - Type inference, ownership semantics, or effect semantics.
//! - Domain-specific conversion algorithms.
//! - IR verification algorithms.
//! - Optimization algorithms.
//! - Physical hardware discovery or allocation.
//! - Quantum routing or error correction.
//! - HDL synthesis or physical design.
//! - Scheduling or runtime execution.
//! - Backend code generation.
//!
//! Those responsibilities remain with their existing authoritative
//! repository subsystems.
//!
//! # Canonical authorities
//!
//! ## Language and semantic authority
//!
//! - `grammar/Zamani.g4`
//! - `grammar/lexer/tokens.g4`
//! - `grammar/compile/lowering.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/portability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `crate::frontend`
//! - `crate::frontend::semantic`
//!
//! ## Universal representation
//!
//! - `crate::zuir`
//! - `crate::zuir::ir`
//! - `crate::zuir::verify`
//! - `crate::zuir::lowering`
//!
//! ## Domain representations and authorities
//!
//! - `crate::classical::zcir`
//! - `crate::quantum::ir`
//! - `crate::hdl::hdlir`
//! - `crate::ai`
//! - `crate::hybrid`
//! - `crate::distributed`
//!
//! ## Legacy representation
//!
//! - `crate::ir_gen`
//! - `crate::ir_verify`
//!
//! ## Backend and execution
//!
//! - `crate::backend`
//! - `crate::runtime`
//!
//! The compiler integration layer consumes these authorities.
//! It must not establish competing definitions.
//!
//! # Module contracts
//!
//! ## input
//!
//! `input.rs` owns compiler-facing IR input admission.
//!
//! It currently integrates canonical ZUIR and delegates full
//! verification to the authoritative ZUIR verifier.
//!
//! Its verification-gated input must retain the report associated
//! with the exact representation snapshot that was verified.
//!
//! A Rust type or caller-supplied Boolean must not be treated as
//! independent proof that arbitrary IR is verified.
//!
//! ## adapters
//!
//! `adapters.rs` owns compiler-facing representation adapter
//! descriptions and the currently implemented adapter boundary.
//!
//! Its existing ZCIR/ZUIR integration delegates to:
//!
//! - `crate::classical::zcir::lowering::to_zuir`
//! - `crate::classical::zcir::lowering::from_zuir`
//!
//! The current ZCIR integration uses canonical ZUIR entities.
//!
//! The existence of a representation identifier does not prove
//! that a conversion implementation exists.
//!
//! Unsupported conversions must remain unavailable.
//!
//! ## compatibility
//!
//! `compatibility.rs` owns compiler-facing compatibility
//! assessment and target compatibility integration.
//!
//! It delegates legacy representation assessment to the
//! authoritative ZCIR legacy adapter.
//!
//! It also integrates the existing backend compatibility
//! contracts where appropriate.
//!
//! Compatibility assessment is not:
//!
//! - IR verification.
//! - Semantic equivalence proof.
//! - Representation conversion.
//! - Target feasibility proof.
//! - Permission to execute.
//!
//! Unknown or incomplete hard requirements must not be silently
//! treated as satisfied.
//!
//! ## verification
//!
//! `verification.rs` owns compiler-facing verification
//! coordination and verification-point identification.
//!
//! Canonical ZUIR verification delegates to:
//!
//! `crate::zuir::verify::verify_full`
//!
//! Only a valid canonical verification result may pass strict
//! verification admission.
//!
//! Invalid and incomplete results must remain distinguishable.
//!
//! This module must not claim that ZUIR verification automatically
//! verifies quantum, HDL, AI, hybrid, or distributed representations.
//!
//! Each domain must use its own authoritative verifier.
//!
//! ## translation
//!
//! `translation.rs` owns compiler-facing translation orchestration.
//!
//! It delegates conversion legality and actual lowering work
//! to the canonical lowering subsystem and supplied backend.
//!
//! A translation descriptor is a declared contract, not
//! independently verified proof of semantic preservation.
//!
//! Potentially lossy conversions must not silently become
//! portable semantic translations.
//!
//! A translated destination must undergo its own applicable
//! authoritative verification before downstream consumers
//! trust it.
//!
//! ## lowering
//!
//! `lowering.rs` owns the compiler-facing verified-input
//! lowering entry point and its structured outcomes.
//!
//! It delegates to:
//!
//! - `crate::zuir::lowering::api`
//! - `crate::zuir::lowering::context`
//! - `crate::zuir::lowering::legality`
//! - Domain-owned lowering backend implementations.
//!
//! It must not implement another lowering algorithm.
//!
//! A successful lowering request does not establish physical
//! resource availability, successful linking, or execution.
//!
//! # Dependency and integration boundaries
//!
//! The intended integration flow is:
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! grammar and frontend
//!      |
//!      v
//! AST and semantic validation
//!      |
//!      v
//! canonical semantic and domain representations
//!      |
//!      +--------------------------+
//!      |             |            |
//!      v             v            v
//!     ZUIR          ZCIR       Domain IRs
//!      |             |       /     |     \
//!      |             +---- Quantum HDL   AI
//!      |                            |
//!      +----------------------------+
//!                   |
//!                   v
//!          compiler::ir::input
//!                   |
//!                   v
//!       compiler::ir::verification
//!                   |
//!                   v
//!       compiler::ir::adapters
//!                   |
//!          +--------+---------+
//!          |                  |
//!          v                  v
//! compiler::ir::compatibility  domain verification
//!          |
//!          v
//! compiler::ir::translation
//!          |
//!          v
//! compiler::ir::lowering
//!          |
//!          v
//! downstream compiler stages
//!          |
//!          v
//! backend and runtime boundaries
//! ```
//!
//! This diagram describes integration responsibilities.
//! It does not require every representation to pass through
//! every child module.
//!
//! Legacy compatibility assessment is a separate path;
//! it is not a mandatory step between canonical verification
//! and translation.
//!
//! Domain-specific representations must retain their own
//! verification, conversion, and lowering authority.
//!
//! # Trust and semantic-preservation rules
//!
//! The following rules apply to every consumer of this module:
//!
//! 1. An IR representation is not automatically trusted.
//!
//! 2. Compatibility assessment is not verification.
//!
//! 3. Identity conversion is not verification.
//!
//! 4. Verification evidence applies to the exact snapshot
//!    that was supplied to the verifier.
//!
//! 5. A transformation that changes a representation requires
//!    applicable fresh verification.
//!
//! 6. A translated destination requires its own applicable
//!    authoritative verification.
//!
//! 7. A legality decision is not physical feasibility.
//!
//! 8. A produced artifact is not necessarily executable.
//!
//! 9. Incomplete verification must never be treated as success.
//!
//! 10. Resource exhaustion must never be treated as successful
//!     verification, conversion, or lowering.
//!
//! 11. Unknown hard capabilities or requirements must not be
//!     interpreted as unlimited support.
//!
//! 12. No conversion may silently discard observable semantics,
//!     effects, ownership, control dependencies, resource
//!     requirements, provenance, or domain-specific guarantees.
//!
//! # Domain extensibility
//!
//! Future representations may be integrated without creating
//! another compiler-owned universal IR.
//!
//! Potential domains include:
//!
//! - Quantum computation.
//! - HDL and hardware description.
//! - AI and machine learning.
//! - Tensor computation.
//! - Hybrid computation.
//! - Distributed computation.
//! - Networking.
//! - Cryptographic computation.
//! - Neuromorphic computation.
//! - Future computational models.
//!
//! Before a domain adapter is exposed through this integration
//! layer, its authoritative owner must establish:
//!
//! - A stable representation contract.
//! - Source and destination identity rules.
//! - Conversion preconditions and postconditions.
//! - Semantic-preservation guarantees.
//! - Applicable verification.
//! - Unsupported-operation behavior.
//! - Error and incomplete-result behavior.
//! - Version compatibility.
//! - Determinism requirements.
//! - Conformance tests.
//!
//! Adding a domain must not require this module to enumerate
//! every operation, device, instruction, gate, or provider.
//!
//! # Scalability and POCO-REAF
//!
//! This module introduces no arbitrary universal limits on:
//!
//! - Programs, modules, functions, regions, or blocks.
//! - Operations, values, types, or effects.
//! - Qubits, signals, registers, or tensor dimensions.
//! - Tensor rank.
//! - Devices, processors, or accelerators.
//! - Distributed participants.
//! - Computing domains.
//!
//! It introduces no fixed-size IR buffers or hardware inventories.
//!
//! Operational budgets belong to the appropriate compiler,
//! verification, and lowering contexts.
//!
//! A configured operational budget is a resource policy,
//! not a semantic capacity limit.
//!
//! Actual address-space, memory, time, representation,
//! implementation, and target constraints remain real constraints
//! and must be reported explicitly.
//!
//! No finite implementation can promise literal infinity.
//! The architecture must scale with available resources without
//! imposing arbitrary architecture-level ceilings.
//!
//! # Grammar boundary
//!
//! This module does not parse source text or define source syntax.
//!
//! The relevant authority chain is:
//!
//! ```text
//! grammar/Zamani.g4
//!       |
//!       v
//! grammar parser and lexer composition
//!       |
//!       v
//! frontend AST
//!       |
//!       v
//! frontend semantic model
//!       |
//!       v
//! canonical IR and domain representations
//!       |
//!       v
//! compiler IR integration
//! ```
//!
//! `grammar/Zamani.g4` composes the language grammar.
//!
//! `grammar/lexer/tokens.g4` belongs to lexical syntax.
//!
//! `grammar/compile/lowering.g4` owns source-level lowering intent.
//!
//! The lowering grammar describes declarations and intent;
//! it does not implement conversion, select hardware, allocate
//! resources, schedule, route, synthesize, or execute.
//!
//! The semantic model defines program meaning.
//!
//! The compilation model defines compilation responsibilities.
//!
//! The semantic-boundaries specification separates source meaning,
//! semantic representation, target realization, and execution.
//!
//! Grammar acceptance does not prove that an IR adapter,
//! verifier, lowering implementation, or backend exists.
//!
//! # Public API and compatibility
//!
//! The existing child-module paths remain public:
//!
//! - `compiler::ir::input`
//! - `compiler::ir::adapters`
//! - `compiler::ir::compatibility`
//! - `compiler::ir::verification`
//! - `compiler::ir::translation`
//! - `compiler::ir::lowering`
//!
//! Each child module remains the owner of its public types.
//!
//! This root intentionally avoids wildcard re-exports.
//!
//! Consumers should import public types from their owning
//! child module.
//!
//! The facade version is independent of the language version,
//! grammar version, semantic-model version, ZUIR schema version,
//! domain IR versions, and backend ABI versions.
//!
//! # Parent integration
//!
//! Parent module:
//!
//! `src/compiler/mod.rs`
//!
//! Required declaration:
//!
//! `pub mod ir;`
//!
//! Crate root:
//!
//! `src/lib.rs`
//!
//! The crate root must expose `compiler` exactly once.
//!
//! This file does not require an additional crate-root declaration.
//!
//! # Freeze criteria
//!
//! This module may be frozen when:
//!
//! - All six declared child modules exist.
//! - All six child modules compile together.
//! - All public type references in composition tests resolve.
//! - The compiler parent exposes this module.
//! - No competing IR or semantic authority is introduced.
//! - Existing public child-module paths remain compatible.
//! - No unsupported domain conversion is claimed as implemented.
//! - No unsafe code or arbitrary universal capacity limit exists.
//! - Repository formatting, compilation, tests, and Clippy pass.

// =============================================================================
// PUBLIC MODULE COMPOSITION
// =============================================================================

/// Verification-gated compiler IR input contracts.
///
/// Owns canonical input admission and verification evidence.
pub mod input;

/// Compiler-facing representation adapter contracts.
///
/// Delegates supported conversions to their canonical owners.
pub mod adapters;

/// Compiler-facing compatibility assessment.
///
/// Integrates legacy representation assessment and target
/// compatibility contracts without duplicating their algorithms.
pub mod compatibility;

/// Compiler-facing integration with authoritative IR verifiers.
///
/// Does not implement verification algorithms.
pub mod verification;

/// Compiler-facing representation translation orchestration.
///
/// Delegates conversion and legality handling to canonical
/// lowering interfaces.
pub mod translation;

/// Compiler-facing lowering of verified IR.
///
/// Delegates actual lowering to domain-owned implementations.
pub mod lowering;

// =============================================================================
// FACADE API VERSION
// =============================================================================

/// Version of the compiler IR integration facade.
///
/// Increment this version when the public composition contract
/// changes incompatibly.
///
/// This version is independent of all language, grammar,
/// semantic, IR schema, and backend ABI versions.
pub const COMPILER_IR_API_VERSION: u16 = 1;

/// Return the compiler IR integration facade API version.
#[must_use]
pub const fn api_version() -> u16 {
    COMPILER_IR_API_VERSION
}

// =============================================================================
// COMPOSITION TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::any::type_name;

    #[test]
    fn compiler_ir_api_version_is_available() {
        assert_eq!(
            api_version(),
            COMPILER_IR_API_VERSION
        );

        assert_ne!(COMPILER_IR_API_VERSION, 0);
    }

    #[test]
    fn input_module_exposes_its_owned_representation_type() {
        let _ = type_name::<input::IrRepresentationKind>();
    }

    #[test]
    fn adapter_module_exposes_its_owned_representation_type() {
        let _ = type_name::<adapters::Representation>();
    }

    #[test]
    fn compatibility_module_exposes_its_owned_assessment_type() {
        let _ = type_name::<compatibility::CompatibilityAssessment>();
    }

    #[test]
    fn verification_module_exposes_its_owned_verification_point() {
        let _ = type_name::<verification::VerificationPoint>();
    }

    #[test]
    fn translation_module_exposes_its_owned_outcome_type() {
        let _ = type_name::<translation::TranslationOutcome<()>>();
    }

    #[test]
    fn lowering_module_exposes_its_owned_outcome_type() {
        let _ = type_name::<lowering::IrLoweringOutcome<()>>();
    }

    #[test]
    fn all_existing_ir_integration_modules_are_public() {
        // These references ensure all intended module paths
        // remain accessible without wildcard re-exports.

        let modules = [
            type_name::<input::IrRepresentationKind>(),
            type_name::<adapters::Representation>(),
            type_name::<compatibility::CompatibilityAssessment>(),
            type_name::<verification::VerificationPoint>(),
            type_name::<translation::TranslationOutcome<()>>(),
            type_name::<lowering::IrLoweringOutcome<()>>(),
        ];

        assert_eq!(modules.len(), 6);
    }
}