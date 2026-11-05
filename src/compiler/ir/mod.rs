#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — IR Integration
//!
//! Canonical path: `src/compiler/ir/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later
//! Safety: Unsafe Rust is forbidden.
//!
//! ## 1. Purpose
//!
//! This module is the composition root for the compiler-facing
//! intermediate-representation integration layer.
//!
//! It connects the existing compiler IR components responsible for:
//!
//! - Canonical IR input.
//! - Representation adapters.
//! - Legacy IR compatibility assessment.
//! - IR verification.
//! - Representation translation.
//! - Verified IR lowering.
//!
//! It provides one stable namespace through which compiler stages
//! can access these capabilities.
//!
//! This module composes existing implementations. It does not
//! implement another intermediate representation, verifier,
//! adapter engine, lowering engine, optimizer, or backend.
//!
//! ## 2. Architectural objective
//!
//! The compiler must support:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! The IR integration layer contributes to this objective by
//! preserving representation identity, verification evidence,
//! conversion guarantees, and semantic-preservation boundaries.
//!
//! The same source program must retain its established meaning
//! while moving through compatible representations and target
//! realization stages.
//!
//! Portability is conditional on:
//!
//! - Preserving source semantics.
//! - Preserving applicable types and effects.
//! - Preserving control-flow and data dependencies.
//! - Preserving ownership and resource requirements.
//! - Satisfying representation compatibility.
//! - Passing the applicable verification processes.
//! - Having an implementation capable of realizing the required
//!   computation.
//!
//! Unsupported or infeasible conversions must be reported rather
//! than silently changing program meaning.
//!
//! ## 3. Canonical ownership
//!
//! The compiler IR integration layer owns:
//!
//! - Composition of compiler-facing IR modules.
//! - Stable module paths for compiler IR integration.
//! - The compiler IR facade API version.
//! - Documentation of integration boundaries.
//!
//! It does not own:
//!
//! - Source grammar.
//! - Lexer or parser.
//! - AST construction.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership semantics.
//! - Canonical ZUIR definitions.
//! - Classical IR definitions.
//! - Quantum IR definitions.
//! - HDLIR definitions.
//! - AI or tensor IR definitions.
//! - Domain-specific conversion algorithms.
//! - Verification algorithms.
//! - Optimization algorithms.
//! - Physical hardware discovery.
//! - Resource allocation.
//! - Scheduling or routing.
//! - Backend code generation.
//! - Runtime execution.
//!
//! These responsibilities remain with their existing
//! authoritative repository subsystems.
//!
//! ## 4. Canonical repository authorities
//!
//! ### Source language
//!
//! - `grammar/Zamani.g4`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! ### Universal IR
//!
//! - `crate::zuir`
//! - `crate::zuir::ir`
//! - `crate::zuir::verify`
//! - `crate::zuir::lowering`
//!
//! ### Classical IR
//!
//! - `crate::classical::zcir`
//! - `crate::classical::zcir::lowering`
//!
//! ### Other domain representations
//!
//! - `crate::quantum::ir`
//! - `crate::hdl::hdlir`
//! - `crate::ai`
//! - `crate::hybrid`
//!
//! ### Existing legacy representation
//!
//! - `crate::ir_gen`
//! - `crate::ir_verify`
//!
//! ### Backend and execution boundaries
//!
//! - `crate::backend`
//! - `crate::runtime`
//!
//! This module must consume these authorities rather than
//! establish competing definitions.
//!
//! ## 5. Module responsibilities
//!
//! The child modules have deliberately separate responsibilities.
//!
//! ### input
//!
//! `input.rs`
//!
//! Owns the compiler-facing, verification-gated IR input contract.
//!
//! Its current supported canonical input is ZUIR.
//!
//! It delegates full-program verification to the canonical
//! ZUIR verifier.
//!
//! A compiler stage must not interpret an arbitrary IR value
//! as verified merely because it has the expected Rust type.
//!
//! ### adapters
//!
//! `adapters.rs`
//!
//! Owns the compiler-facing adapter boundary.
//!
//! Its currently supported ZCIR/ZUIR identity integration
//! delegates to the existing canonical ZCIR lowering adapters.
//!
//! It must not claim that unsupported conversions are implemented.
//!
//! In particular, the existence of this module does not establish
//! automatic conversion for:
//!
//! - Quantum IR.
//! - HDLIR.
//! - AI IR.
//! - Tensor representations.
//! - Legacy IR.
//!
//! Such conversions must be implemented and verified by their
//! authoritative domain owners before being exposed here.
//!
//! ### compatibility
//!
//! `compatibility.rs`
//!
//! Owns the compiler-facing compatibility facade for legacy IR.
//!
//! It delegates compatibility assessment to:
//!
//! `crate::classical::zcir::lowering::legacy_adapter`
//!
//! It does not claim that an assessed legacy module has been
//! converted into canonical ZUIR.
//!
//! Compatibility assessment is not verification, semantic
//! equivalence, target feasibility, or execution permission.
//!
//! ### verification
//!
//! `verification.rs`
//!
//! Owns the compiler-facing verification integration boundary.
//!
//! Canonical ZUIR verification is delegated to:
//!
//! `crate::zuir::verify::verify_full`
//!
//! Invalid or incomplete verification must not be treated as
//! successful verification.
//!
//! Domain-specific representations must use their own
//! authoritative verification APIs.
//!
//! ### translation
//!
//! `translation.rs`
//!
//! Owns compiler-facing translation orchestration.
//!
//! It delegates actual conversion and legality handling to
//! the canonical lowering subsystem.
//!
//! Potentially lossy conversion must not be silently accepted.
//!
//! A successful translation does not automatically prove that
//! the destination representation is valid.
//!
//! ### lowering
//!
//! `lowering.rs`
//!
//! Owns the compiler-facing verified-input lowering entry point.
//!
//! It connects verified compiler input to translation and
//! domain-owned lowering implementations.
//!
//! It must not implement another lowering algorithm.
//!
//! A completed lowering artifact is not proof of target
//! feasibility, linking success, or successful execution.
//!
//! ## 6. Dependency direction
//!
//! The intended dependency structure is:
//!
//! ```text
//!             Zamani source
//!                   |
//!                   v
//!           grammar / frontend
//!                   |
//!                   v
//!           semantic analysis
//!                   |
//!                   v
//!             canonical IR
//!                   |
//!          +--------+--------+
//!          |                 |
//!          v                 v
//!         ZUIR           Domain IRs
//!          |          /      |      \
//!          |      ZCIR    Quantum   HDLIR
//!          |                 |
//!          +--------+--------+
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
//!                   v
//!       compiler::ir::compatibility
//!                   |
//!                   v
//!       compiler::ir::translation
//!                   |
//!                   v
//!         compiler::ir::lowering
//!                   |
//!                   v
//!       downstream compiler stages
//!                   |
//!                   v
//!            backend / runtime
//! ```
//!
//! The diagram describes architectural responsibilities.
//! It does not require every domain representation to pass through
//! every child module.
//!
//! A stage must invoke only the interfaces required by the
//! representation and operation it is processing.
//!
//! ## 7. Trust and verification rules
//!
//! The module composition must preserve the following rules:
//!
//! 1. An IR representation is not automatically trusted.
//!
//! 2. Compatibility assessment is not verification.
//!
//! 3. Identity conversion is not verification.
//!
//! 4. Verification applies to the exact representation snapshot
//!    supplied to the verifier.
//!
//! 5. A transformation that changes an IR invalidates the
//!    assumption that an earlier verification report describes
//!    the changed representation.
//!
//! 6. A translated destination must pass its applicable
//!    authoritative verifier before it is trusted.
//!
//! 7. A lowering legality decision is not a physical hardware
//!    feasibility decision.
//!
//! 8. A successfully produced artifact is not necessarily
//!    executable on a particular target.
//!
//! 9. Unsupported or incomplete checks must remain distinguishable
//!    from successful checks.
//!
//! 10. Resource exhaustion must never be reported as successful
//!     verification or conversion.
//!
//! ## 8. Domain extensibility
//!
//! The composition root must support the progressive integration
//! of additional canonical representations.
//!
//! Future integration may include:
//!
//! - Quantum IR.
//! - HDLIR.
//! - AI dialects.
//! - Tensor representations.
//! - Distributed computation representations.
//! - Networking representations.
//! - Cryptographic representations.
//! - Neuromorphic representations.
//! - Future computing domains.
//!
//! Adding a domain must not require creating another compiler IR
//! authority or modifying unrelated existing domain implementations.
//!
//! The domain owner must first provide:
//!
//! - A stable representation contract.
//! - A documented conversion contract.
//! - Semantic-preservation guarantees.
//! - Applicable verification.
//! - Explicit unsupported-operation behavior.
//! - Version compatibility.
//! - Determinism guarantees.
//! - Tests.
//!
//! The compiler facade can then expose the established integration.
//!
//! ## 9. Scalability and resource policy
//!
//! This module introduces no arbitrary maximum for:
//!
//! - Programs.
//! - Modules.
//! - Functions.
//! - Regions.
//! - Blocks.
//! - Operations.
//! - Values.
//! - Types.
//! - Effects.
//! - Qubits.
//! - Signals.
//! - Registers.
//! - Tensor dimensions.
//! - Tensor rank.
//! - Devices.
//! - Accelerators.
//! - Distributed participants.
//! - Computing domains.
//!
//! It introduces no fixed-size IR buffers, fixed hardware inventories,
//! or artificial machine-count limits.
//!
//! Resource budgets belong to the appropriate compilation,
//! verification, and lowering contexts.
//!
//! Such budgets are operational policies, not semantic limits.
//!
//! No finite software implementation can promise literal infinity.
//! The architecture must avoid arbitrary ceilings and scale with
//! available resources while reporting genuine exhaustion honestly.
//!
//! ## 10. Grammar boundary
//!
//! This module does not parse Zamani source or define language syntax.
//!
//! The source-language authority chain is:
//!
//! ```text
//! grammar/Zamani.g4
//!         |
//!         v
//!       frontend
//!         |
//!         v
//!       AST
//!         |
//!         v
//!   semantic model
//!         |
//!         v
//!    canonical IR
//!         |
//!         v
//! compiler IR integration
//! ```
//!
//! `grammar/Zamani.g4` is the root grammar composition contract.
//!
//! `grammar/DESIGN.md` defines grammar architecture and ownership.
//!
//! `grammar/specification/semantic-model.md` defines source meaning.
//!
//! `grammar/specification/compilation-model.md` defines the
//! compilation architecture.
//!
//! `grammar/validation/semantic-boundaries.md` defines the
//! boundaries between syntax, semantics, IR, compilation,
//! realization, and execution.
//!
//! Grammar productions must not be interpreted as proof that
//! a corresponding IR adapter or lowering implementation exists.
//!
//! ## 11. Compatibility
//!
//! This module introduces a stable compiler IR namespace.
//!
//! Existing child-module paths remain public:
//!
//! - `compiler::ir::input`
//! - `compiler::ir::adapters`
//! - `compiler::ir::compatibility`
//! - `compiler::ir::verification`
//! - `compiler::ir::translation`
//! - `compiler::ir::lowering`
//!
//! The child modules retain ownership of their own public types.
//!
//! This root deliberately avoids wildcard re-exports.
//!
//! That prevents:
//!
//! - Name collisions.
//! - Ambiguous type ownership.
//! - Accidental exposure of implementation details.
//! - Unintentional public API expansion.
//!
//! Consumers should import a type from the child module that
//! actually owns it.
//!
//! ## 12. Integration requirements
//!
//! Parent:
//!
//! `src/compiler/mod.rs`
//!
//! Required parent declaration:
//!
//! `pub mod ir;`
//!
//! Existing child modules:
//!
//! - `src/compiler/ir/input.rs`
//! - `src/compiler/ir/adapters.rs`
//! - `src/compiler/ir/compatibility.rs`
//! - `src/compiler/ir/verification.rs`
//! - `src/compiler/ir/translation.rs`
//! - `src/compiler/ir/lowering.rs`
//!
//! Crate root:
//!
//! `src/lib.rs` already declares `pub mod compiler;`.
//!
//! No additional `src/lib.rs` declaration is required.
//!
//! ## 13. Freeze criteria
//!
//! This file is ready to freeze when:
//!
//! - Every declared child module exists.
//! - Every child module has a valid public contract.
//! - The compiler root exposes this module.
//! - No duplicate IR or lowering implementation is introduced.
//! - All public paths resolve.
//! - Child modules compile together.
//! - Existing callers remain compatible.
//! - No unsafe code is introduced.
//! - No arbitrary hardware ceiling is introduced.
//! - Tests and repository quality gates pass.

// =============================================================================
// PUBLIC MODULE COMPOSITION
// =============================================================================

/// Verification-gated compiler IR input contracts.
///
/// This module accepts canonical IR through explicit input
/// abstractions and preserves verification evidence.
pub mod input;

/// Compiler-facing representation adapters.
///
/// This module delegates supported conversions to their
/// authoritative domain implementations.
pub mod adapters;

/// Compiler-facing compatibility assessment for legacy IR.
///
/// This module delegates to the canonical ZCIR legacy adapter.
pub mod compatibility;

/// Compiler-facing integration with authoritative IR verifiers.
///
/// This module does not implement verification algorithms.
pub mod verification;

/// Compiler-facing representation translation orchestration.
///
/// This module delegates conversion and legality handling
/// to canonical lowering interfaces.
pub mod translation;

/// Compiler-facing lowering of verified IR.
///
/// This module delegates actual lowering to domain-owned
/// implementations.
pub mod lowering;

// =============================================================================
// COMPILER IR FACADE VERSION
// =============================================================================

/// Version of the compiler IR integration facade.
///
/// This version is independent of the Zamani language version,
/// grammar version, semantic-model version, ZUIR schema version,
/// domain IR versions, and backend ABI versions.
///
/// Increment this version only when the public composition
/// contract changes incompatibly.
pub const COMPILER_IR_API_VERSION: u16 = 1;

/// Return the compiler IR integration facade API version.
#[must_use]
pub const fn api_version() -> u16 {
    COMPILER_IR_API_VERSION
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiler_ir_api_version_is_available() {
        assert_eq!(api_version(), COMPILER_IR_API_VERSION);
        assert_ne!(COMPILER_IR_API_VERSION, 0);
    }

    #[test]
    fn all_compiler_ir_subsystems_are_exposed() {
        // These type references ensure the intended module paths
        // remain accessible without introducing wildcard exports.

        let _input_module: fn() = || {
            let _ = core::any::type_name::<
                input::IrRepresentationKind,
            >();
        };

        let _adapter_module: fn() = || {
            let _ = core::any::type_name::<
                adapters::RepresentationKind,
            >();
        };

        let _compatibility_module: fn() = || {
            let _ = core::any::type_name::<
                compatibility::CompatibilityAssessment,
            >();
        };

        let _verification_module: fn() = || {
            let _ = core::any::type_name::<
                verification::VerificationPoint,
            >();
        };

        let _translation_module: fn() = || {
            let _ = core::any::type_name::<
                translation::TranslationOutcome<()>,
            >();
        };

        let _lowering_module: fn() = || {
            let _ = core::any::type_name::<
                lowering::IrLoweringOutcome<()>,
            >();
        };

        let _ = (
            _input_module,
            _adapter_module,
            _compatibility_module,
            _verification_module,
            _translation_module,
            _lowering_module,
        );
    }
}