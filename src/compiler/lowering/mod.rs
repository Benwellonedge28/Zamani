
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Meta-Compiler — Lowering
//!
//! Path: src/compiler/lowering/mod.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only. Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module is the canonical composition root for the Zamani compiler's
//! lowering subsystem.
//!
//! It integrates the existing lowering implementations into one stable,
//! extensible compiler-facing API.
//!
//! The subsystem supports the architectural objective:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever (POCO-REAF).
//!
//! Lowering transforms verified computational representations through
//! explicitly authorized, semantics-preserving transformations.
//!
//! Target-specific realization is supported through registered or explicitly
//! supplied domain-owned lowering implementations.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! THIS MODULE OWNS:
//!
//! - Lowering subsystem composition.
//! - Public module declarations.
//! - Stable module paths.
//! - Public API re-exports.
//! - Lowering subsystem API version.
//! - Lowering subsystem integration documentation.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Source-language grammar.
//! - Lexer or parser.
//! - AST construction.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership analysis.
//! - Canonical ZUIR.
//! - Classical ZCIR.
//! - Quantum IR.
//! - HDLIR.
//! - AI or tensor IR.
//! - Domain-specific IR definitions.
//! - IR verification algorithms.
//! - Conversion algorithms.
//! - Legalization algorithms.
//! - Decomposition algorithms.
//! - Outlining algorithms.
//! - Target discovery.
//! - Capability discovery.
//! - Resource allocation.
//! - Hardware placement.
//! - Quantum routing.
//! - Physical scheduling.
//! - Backend implementation.
//! - Code generation.
//! - Runtime execution.
//!
//! These responsibilities remain with their authoritative repository
//! subsystems.
//!
//! ============================================================================
//! SUBSYSTEM ARCHITECTURE
//! ============================================================================
//!
//!                         VERIFIED IR
//!                              |
//!                              v
//!                         LEGALIZATION
//!                              |
//!                              v
//!                    REPRESENTATION CONVERSION
//!                              |
//!                              v
//!                       LOWERING ANALYSIS
//!                              |
//!                   +----------+----------+
//!                   |                     |
//!                   v                     v
//!               DECOMPOSITION          OUTLINING
//!                   |                     |
//!                   +----------+----------+
//!                              |
//!                              v
//!                       TARGET LOWERING
//!                              |
//!                              v
//!                     DOMAIN BACKEND
//!                              |
//!                              v
//!                    DESTINATION VALIDATION
//!                              |
//!                              v
//!                    ARTIFACT INTEGRATION
//!
//! The diagram describes architectural responsibilities.
//! It does not mandate that every program execute every stage.
//!
//! The compiler pipeline determines the applicable ordering and stages.
//!
//! ============================================================================
//! CHILD MODULE RESPONSIBILITIES
//! ============================================================================
//!
//! legalization
//!
//! Provides verified legalization orchestration, legality outcomes,
//! destination verification admission, and structured errors.
//!
//! conversion
//!
//! Provides the compiler-facing conversion request and invocation boundary.
//!
//! decomposition
//!
//! Coordinates domain-owned, semantics-preserving decomposition providers.
//!
//! outlining
//!
//! Coordinates domain-owned, semantics-preserving outlining providers.
//!
//! target_lowering
//!
//! Provides target descriptor validation, backend identity checking,
//! target admission, and delegation to canonical verified IR lowering.
//!
//! No child module may silently acquire another child's ownership.
//!
//! ============================================================================
//! CANONICAL REPOSITORY AUTHORITIES
//! ============================================================================
//!
//! Source grammar:
//!     grammar/compile/lowering.g4
//!
//! Source semantics:
//!     frontend semantic authorities
//!
//! Canonical universal IR:
//!     crate::zuir
//!
//! Classical IR:
//!     crate::classical::zcir
//!
//! Quantum IR:
//!     crate::quantum::ir
//!
//! Hardware description IR:
//!     crate::hdl::hdlir
//!
//! AI:
//!     crate::ai
//!
//! Hybrid computation:
//!     crate::hybrid
//!
//! Verified compiler IR input:
//!     crate::compiler::ir::input
//!
//! Representation adapters:
//!     crate::compiler::ir::adapters
//!
//! Canonical compiler IR lowering:
//!     crate::compiler::ir::lowering
//!
//! Canonical lowering legality:
//!     crate::zuir::lowering::legality
//!
//! Canonical lowering context:
//!     crate::zuir::lowering::context
//!
//! Target descriptors:
//!     crate::compiler::configuration::target
//!
//! Target discovery:
//!     crate::compiler::planning::target_discovery
//!
//! Capability matching:
//!     crate::compiler::planning::capability_matching
//!
//! Backend:
//!     crate::backend
//!
//! Runtime:
//!     crate::runtime
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! 1. Source and semantic analysis establish program meaning.
//!
//! 2. The appropriate canonical IR owner establishes the representation.
//!
//! 3. compiler::ir::input establishes the verified-input boundary.
//!
//! 4. compiler::ir::adapters supplies representation compatibility contracts.
//!
//! 5. compiler::ir::lowering delegates to canonical translation and lowering.
//!
//! 6. legalization coordinates lowering and destination verification.
//!
//! 7. conversion invokes explicitly supplied conversion implementations.
//!
//! 8. decomposition invokes explicitly supplied decomposition providers.
//!
//! 9. outlining invokes explicitly supplied outlining providers.
//!
//! 10. target_lowering establishes target/backend admission.
//!
//! 11. Backend implementations produce their actual target-specific output.
//!
//! 12. The appropriate destination verifier validates the resulting output.
//!
//! 13. Artifact integration and subsequent compiler stages consume only
//!     outputs that satisfy their respective verification contracts.
//!
//! The composition root does not bypass any of these boundaries.
//!
//! ============================================================================
//! POCO-REAF AND SCALABILITY
//! ============================================================================
//!
//! This module introduces no fixed limits on:
//!
//! - Lowering stages.
//! - Transformations.
//! - Representations.
//! - Computing domains.
//! - Modules.
//! - Functions.
//! - Operations.
//! - Values.
//! - Registers.
//! - Signals.
//! - Qubits.
//! - Tensor dimensions.
//! - Tensor rank.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - ASICs.
//! - QPUs.
//! - Accelerators.
//! - Distributed nodes.
//! - Hybrid combinations.
//!
//! No arbitrary MAX_* hardware-capacity constants are introduced.
//!
//! The subsystem uses the growable data structures and resource policies
//! belonging to its authoritative implementations.
//!
//! Resource exhaustion, unsupported transformations, and infeasible target
//! realization must be reported explicitly.
//!
//! Literal infinity is not a guarantee of finite software.
//!
//! ============================================================================
//! SEMANTIC PRESERVATION
//! ============================================================================
//!
//! All lowering implementations must preserve applicable:
//!
//! - Observable behavior.
//! - Types.
//! - Values.
//! - Control flow.
//! - Data dependencies.
//! - Effects.
//! - Ownership guarantees.
//! - Memory semantics.
//! - Numerical guarantees.
//! - Concurrency semantics.
//! - Synchronization.
//! - Communication.
//! - Resource requirements.
//! - Quantum measurement semantics.
//! - HDL signal semantics.
//! - AI and tensor semantics.
//! - Provenance.
//!
//! An unsupported transformation must not be silently replaced with an
//! unrelated transformation.
//!
//! The canonical legality and verification authorities remain responsible
//! for their respective correctness decisions.
//!
//! ============================================================================
//! FAILURE CONTRACT
//! ============================================================================
//!
//! The subsystem preserves the distinction between:
//!
//! - Invalid source verification.
//! - Incomplete verification.
//! - Unsupported representation conversion.
//! - Illegal lowering.
//! - Required decomposition.
//! - Required outlining.
//! - Backend target mismatch.
//! - Target admission failure.
//! - Backend conversion failure.
//! - Destination verification failure.
//! - Resource exhaustion.
//! - Artifact integration failure.
//!
//! A successful Rust Result containing an incomplete lowering outcome
//! must never be interpreted as successful artifact production.
//!
//! ============================================================================
//! DEPENDENCY DIRECTION
//! ============================================================================
//!
//! compiler::lowering
//!       |
//!       +--> compiler::ir
//!       |
//!       +--> compiler::configuration
//!       |
//!       +--> compiler::planning
//!       |
//!       +--> zuir::lowering
//!       |
//!       +--> domain-owned conversion/decomposition/outlining providers
//!       |
//!       +--> backend interfaces
//!
//! Lowering must not become the owner of planning, IR, backend, or runtime.
//!
//! ============================================================================
//! COMPATIBILITY
//! ============================================================================
//!
//! Existing child module paths remain available:
//!
//! compiler::lowering::legalization
//! compiler::lowering::conversion
//! compiler::lowering::decomposition
//! compiler::lowering::outlining
//! compiler::lowering::target_lowering
//!
//! The composition root does not replace their implementations.
//!
//! Existing public child APIs remain accessible through their module paths.
//!
//! ============================================================================
//! COMPLETION REQUIREMENTS
//! ============================================================================
//!
//! This composition root is complete when:
//!
//! - All declared child files exist.
//! - All public exports resolve.
//! - Existing child APIs remain accessible.
//! - No duplicate lowering implementation is introduced.
//! - No unsafe Rust is introduced.
//! - Module API tests pass.
//! - Repository compilation succeeds.
//! - Repository tests pass.
//!
//! This module's completion does not certify every child implementation,
//! domain adapter, backend, or destination verifier as production-ready.

// ============================================================================
// MODULE DECLARATIONS
// ============================================================================

/// Verified legalization orchestration.
///
/// Coordinates canonical lowering and destination verification.
pub mod legalization;

/// Representation conversion orchestration.
///
/// Delegates conversion algorithms to authoritative implementations.
pub mod conversion;

/// Semantics-preserving decomposition orchestration.
///
/// Delegates decomposition algorithms to domain-owned providers.
pub mod decomposition;

/// Semantics-preserving outlining orchestration.
///
/// Delegates outlining algorithms to domain-owned providers.
pub mod outlining;

/// Target-specific lowering admission and orchestration.
///
/// Validates explicit target descriptors and backend admission before
/// delegating to canonical verified IR lowering.
pub mod target_lowering;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable identity of the compiler lowering subsystem.
pub const LOWERING_MODULE_NAME: &str =
    "zamani.compiler.lowering";

/// Public composition API version.
///
/// Independent of:
///
/// - Zamani language version.
/// - Grammar version.
/// - Semantic-model version.
/// - ZUIR version.
/// - Domain IR versions.
/// - Backend ABI versions.
///
/// Increment when the public composition contract changes incompatibly.
pub const LOWERING_API_VERSION: u16 = 1;

/// Return the lowering composition API version.
#[must_use]
pub const fn api_version() -> u16 {
    LOWERING_API_VERSION
}

/// Return the stable lowering subsystem identity.
#[must_use]
pub const fn module_name() -> &'static str {
    LOWERING_MODULE_NAME
}

// ============================================================================
// PUBLIC LEGALIZATION API
// ============================================================================

pub use self::legalization::{
    legalize_verified_with,
    permits_legalization,
    DestinationVerification,
    LegalizationError,
    LegalizationOutcome,
    LegalizationResult,
    LegalizationStatus,
    LEGALIZATION_API_VERSION,
    LEGALIZATION_MODULE_NAME,
};

// ============================================================================
// PUBLIC CONVERSION API
// ============================================================================

pub use self::conversion::{
    convert_verified_with,
    ConversionError,
    ConversionOutcome,
    ConversionRequest,
    ConversionResult,
};

// ============================================================================
// PUBLIC DECOMPOSITION API
// ============================================================================

pub use self::decomposition::{
    decompose_expansion_with,
    DecompositionError,
    DecompositionOutcome,
    DecompositionOutput,
    DecompositionProvider,
    DecompositionResult,
    PreservationContract as DecompositionPreservationContract,
    DECOMPOSITION_API_VERSION,
};

// ============================================================================
// PUBLIC OUTLINING API
// ============================================================================

pub use self::outlining::{
    outline_with,
    OutliningError,
    OutliningOutcome,
    OutliningOutput,
    OutliningProvider,
    OutliningRequest,
    OutliningResult,
    PreservationContract as OutliningPreservationContract,
    OUTLINING_API_VERSION,
};

// ============================================================================
// PUBLIC TARGET LOWERING API
// ============================================================================

pub use self::target_lowering::{
    TargetLowering,
    TargetLoweringBackend,
    TargetLoweringError,
    TargetLoweringOutcome,
    TargetLoweringResult,
    TARGET_LOWERING_API_VERSION,
};

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowering_module_identity_is_stable() {
        assert_eq!(
            module_name(),
            "zamani.compiler.lowering"
        );

        assert_eq!(
            LOWERING_MODULE_NAME,
            "zamani.compiler.lowering"
        );
    }

    #[test]
    fn lowering_api_version_is_available() {
        assert_eq!(
            api_version(),
            LOWERING_API_VERSION
        );

        assert_ne!(
            LOWERING_API_VERSION,
            0
        );
    }

    #[test]
    fn all_lowering_subsystems_are_exposed() {
        assert!(LEGALIZATION_API_VERSION > 0);

        assert!(DECOMPOSITION_API_VERSION > 0);

        assert!(OUTLINING_API_VERSION > 0);

        assert!(TARGET_LOWERING_API_VERSION > 0);

        assert!(conversion::api_version() > 0);
    }

    #[test]
    fn legalization_module_identity_is_stable() {
        assert_eq!(
            legalization::module_name(),
            LEGALIZATION_MODULE_NAME
        );
    }

    #[test]
    fn public_target_lowering_service_is_accessible() {
        assert_eq!(
            TargetLowering::api_version(),
            TARGET_LOWERING_API_VERSION
        );

        assert_eq!(
            TargetLowering::module_name(),
            "zamani.compiler.lowering.target"
        );
    }

    #[test]
    fn decomposition_and_outlining_contracts_remain_distinct() {
        let decomposition_contract =
            core::any::type_name::<DecompositionPreservationContract>();

        let outlining_contract =
            core::any::type_name::<OutliningPreservationContract>();

        assert_ne!(
            decomposition_contract,
            outlining_contract
        );
    }

    #[test]
    fn canonical_lowering_error_types_remain_accessible() {
        let _ = core::any::type_name::<
            TargetLoweringError<std::convert::Infallible>
        >();

        let _ = core::any::type_name::<
            LegalizationStatus
        >();

        let _ = core::any::type_name::<
            ConversionError<std::convert::Infallible>
        >();
    }
}
