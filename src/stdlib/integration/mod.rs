
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Repository Integration Boundary
//!
//! Canonical path:
//! `src/stdlib/integration/mod.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97 or later.
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Establish the standard library's integration boundary with the
//! authoritative Zamani frontend, semantic system, intermediate
//! representations, compiler, and provider subsystems.
//!
//! This module is an aggregation and architectural boundary.
//! It does not implement another compiler, semantic analyzer,
//! intermediate representation, provider registry, scheduler,
//! runtime, hardware abstraction layer, or execution engine.
//!
//! # Architectural authorities
//!
//! The following repository subsystems remain authoritative.
//!
//! ## Language and grammar
//!
//! - grammar/DESIGN.md
//! - grammar/Zamani.g4
//! - grammar/antlr/ZamaniParser.g4
//! - grammar/specification/
//! - grammar/spec/
//!
//! ## Frontend
//!
//! - crate::frontend::lexer
//! - crate::frontend::parser
//! - crate::frontend::ast
//!
//! ## Semantic analysis
//!
//! - crate::frontend::semantic
//!
//! ## Intermediate representations
//!
//! - crate::zuir
//! - crate::classical::zcir
//! - crate::quantum::ir
//! - crate::hdl::hdlir
//!
//! ## Compilation
//!
//! - crate::compiler
//! - crate::backend
//!
//! ## Provider infrastructure
//!
//! - crate::stdlib::provider
//! - crate::stdlib::registry
//! - crate::compiler::extensions
//! - crate::quantum::hardware
//!
//! ## Execution
//!
//! - crate::runtime
//! - The authoritative HAL and hardware-provider implementations.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - The standard library's integration module hierarchy.
//! - Stable paths to the existing integration adapters.
//! - The integration architecture documentation.
//! - Integration-wide architectural invariants.
//! - Integration namespace contract tests.
//!
//! # Non-ownership
//!
//! This module MUST NOT own:
//!
//! - Language grammar or language syntax.
//! - Lexer or parser implementations.
//! - AST definitions or AST storage.
//! - Semantic types or semantic analysis.
//! - Requirements, capabilities, effects, policies or provenance
//!   definitions already owned by the semantic subsystem.
//! - A second universal intermediate representation.
//! - Classical, quantum, HDL, hybrid or distributed IR definitions.
//! - Compiler algorithms or optimization passes.
//! - Backend implementations.
//! - Provider identities or provider registries.
//! - Hardware discovery or hardware allocation.
//! - Quantum routing, scheduling, calibration or QEC.
//! - Distributed scheduling or resource allocation.
//! - Runtime execution or lifecycle management.
//!
//! Each responsibility remains with its canonical owner.
//!
//! # Dependency direction
//!
//! ```text
//!              Zamani source
//!                    |
//!                    v
//!             Canonical grammar
//!                    |
//!                    v
//!             Native frontend
//!                    |
//!                    v
//!              Canonical AST
//!                    |
//!                    v
//!           Semantic authority
//!                    |
//!                    v
//!        Standard-library integration
//!                    |
//!          +---------+---------+
//!          |         |         |
//!          v         v         v
//!        Compiler  Provider   Domain APIs
//!          |       adapters      |
//!          v                     |
//!       Canonical IR             |
//!          |                     |
//!          +----------+----------+
//!                     |
//!                     v
//!             Compiler planning
//!                     |
//!                     v
//!             Optimization/lowering
//!                     |
//!                     v
//!             Backend realization
//!                     |
//!                     v
//!                Runtime/HAL
//!                     |
//!          +----------+-----------+
//!          |          |           |
//!          v          v           v
//!        Classical  Quantum      HDL
//!          |          |           |
//!          +----------+-----------+
//!                     |
//!                     v
//!              Hybrid execution
//!                     |
//!                     v
//!             Distributed systems
//!
//! Future domains integrate through their respective authorities.
//! ```
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This integration boundary preserves target-independent program
//! meaning and delegates realization to the appropriate subsystems.
//!
//! It introduces no universal maximum for:
//!
//! - CPUs, cores or threads.
//! - GPUs, FPGAs, ASICs or QPUs.
//! - Quantum or physical qubits.
//! - Registers or register widths.
//! - Memory or storage.
//! - Tensor dimensions or tensor rank.
//! - Modules, functions, operations or values.
//! - Distributed nodes or participants.
//! - Devices, providers or computational domains.
//!
//! Actual execution is subject to available resources, address-space
//! and representation limits, target capabilities, semantic
//! requirements, policies and explicitly configured budgets.
//!
//! No finite implementation can guarantee literally infinite
//! computation. The architecture must avoid arbitrary universal
//! ceilings and preserve resource-parametric computation.
//!
//! # Integration rules
//!
//! 1. Reuse canonical types rather than creating duplicate types.
//! 2. Preserve original subsystem errors and diagnostics.
//! 3. Do not convert unknown capabilities into supported capabilities.
//! 4. Do not convert requirements into preferences.
//! 5. Do not convert incomplete analysis into successful validation.
//! 6. Do not treat semantic validity as target feasibility.
//! 7. Do not treat provider registration as provider activation.
//! 8. Do not treat IR construction as IR verification.
//! 9. Do not treat compilation as successful execution.
//! 10. Do not treat a supported target as proof that every program
//!     is executable on that target.
//! 11. Do not silently discard effects, constraints, provenance,
//!     numerical guarantees or resource requirements.
//! 12. Keep future computational domains extensible.
//!
//! # Initialization
//!
//! Importing this module performs no initialization.
//!
//! It does not:
//!
//! - Discover providers.
//! - Initialize the compiler.
//! - Initialize the runtime.
//! - Allocate resources.
//! - Connect to hardware.
//! - Register global mutable state.
//! - Execute a program.
//!
//! Lifecycle management belongs to the respective subsystem owners.
//!
//! # Runtime integration status
//!
//! A runtime integration adapter is intentionally not declared here.
//!
//! The existing integration/runtime.rs currently contains a textual
//! placeholder rather than a valid Rust module.
//!
//! Declaring it would make this module tree uncompilable.
//!
//! A runtime adapter may be added after the canonical runtime exposes
//! a stable, documented and implemented public integration contract.
//!
//! # Frontend integration status
//!
//! The existing frontend integration adapter is included as a
//! subordinate module, but its implementation must pass its own
//! completion and correctness requirements before the overall
//! integration subsystem can be certified production-ready.
//!
//! In particular, incomplete parse-result validation must never
//! reach semantic analysis or panic during normal operation.
//!
//! # Compatibility
//!
//! The canonical subsystem paths remain available.
//!
//! This integration module does not replace or rename them.
//!
//! Existing public integration files retain their individual
//! versioning and compatibility contracts.
//!
//! # Freeze criteria
//!
//! This module is complete when:
//!
//! - Every declared child module exists.
//! - Every declared child module is valid Rust.
//! - Every public integration path has a defined owner.
//! - No duplicate subsystem authority is introduced.
//! - All child integration contracts are verified.
//! - The runtime placeholder is replaced before runtime integration
//!   is publicly declared.
//! - Frontend integration has no incomplete or panic-based success path.
//! - All integration tests pass.
//! - Formatting, compilation, tests and Clippy pass.
//! - Safe Rust is preserved.
//! - No arbitrary universal hardware-capacity ceiling is introduced.

// =============================================================================
// CANONICAL INTEGRATION MODULES
// =============================================================================

/// Integration with the native Zamani frontend.
///
/// Connects canonical parsing and semantic-analysis workflows.
///
/// The frontend remains authoritative for lexical analysis,
/// parsing, AST construction and source diagnostics.
pub mod frontend;

/// Integration with the canonical semantic subsystem.
///
/// Reuses the frontend's semantic model, diagnostics,
/// analysis services and validity contracts.
pub mod semantic;

/// Integration with canonical Zamani intermediate representations.
///
/// Exposes existing ZUIR, classical ZCIR, quantum IR and HDLIR
/// authorities without creating competing representations.
pub mod ir;

/// Integration with the canonical compiler.
///
/// Exposes compiler-owned compilation requests, configuration,
/// results and compilation services.
pub mod compiler;

/// Integration with existing provider authorities.
///
/// Preserves separate standard-library, compiler-extension
/// and quantum-hardware provider systems.
pub mod providers;

// =============================================================================
// PUBLIC INTEGRATION CONTRACT
// =============================================================================

/// Stable identifier for the standard-library integration namespace.
pub const INTEGRATION_NAMESPACE: &str =
    "zamani.stdlib.integration";

/// Version of this aggregation module's integration contract.
///
/// This version is independent of the language, grammar, semantic,
/// IR, compiler, provider and runtime versions.
pub const INTEGRATION_API_VERSION: u16 = 1;

/// Returns the integration namespace identifier.
#[must_use]
pub const fn integration_namespace() -> &'static str {
    INTEGRATION_NAMESPACE
}

/// Returns the aggregation module's contract version.
#[must_use]
pub const fn integration_api_version() -> u16 {
    INTEGRATION_API_VERSION
}

/// Returns the architectural invariants that all integration
/// adapters must preserve.
///
/// This is descriptive metadata only. It performs no initialization,
/// discovery, registration, compilation or execution.
#[must_use]
pub const fn architectural_invariants() -> &'static [&'static str] {
    &[
        "grammar owns source syntax",
        "frontend owns parsing and AST construction",
        "semantic subsystem owns source meaning",
        "canonical IR subsystems own representations",
        "compiler owns compilation and lowering",
        "providers own their provider identities and registries",
        "runtime owns scheduling and execution",
        "HAL owns hardware communication",
        "requirements remain distinct from preferences",
        "constraints remain distinct from hints",
        "capabilities require authoritative evidence",
        "feasibility remains distinct from semantic validity",
        "IR construction does not imply verification",
        "compilation does not imply execution",
        "resource allocation remains outside this namespace",
        "new computational domains remain extensible",
        "no arbitrary universal hardware ceilings are introduced",
        "unsafe Rust is forbidden",
    ]
}

// =============================================================================
// CONTRACT TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_namespace_is_stable() {
        assert_eq!(
            integration_namespace(),
            "zamani.stdlib.integration"
        );
    }

    #[test]
    fn integration_api_version_is_nonzero() {
        assert_ne!(
            integration_api_version(),
            0
        );
    }

    #[test]
    fn integration_version_matches_constant() {
        assert_eq!(
            integration_api_version(),
            INTEGRATION_API_VERSION
        );
    }

    #[test]
    fn architectural_invariants_are_available() {
        let invariants = architectural_invariants();

        assert!(!invariants.is_empty());

        assert!(
            invariants.iter().any(|invariant| {
                invariant.contains("grammar owns source syntax")
            })
        );

        assert!(
            invariants.iter().any(|invariant| {
                invariant.contains("semantic subsystem owns source meaning")
            })
        );

        assert!(
            invariants.iter().any(|invariant| {
                invariant.contains("no arbitrary universal hardware ceilings")
            })
        );
    }

    #[test]
    fn integration_does_not_define_an_independent_ir() {
        fn accepts_program(
            _: ir::StandardIrProgram<
                ir::StandardIrModule<ir::StandardIrFunction>
            >
        ) {}

        let _ = accepts_program;
    }

    #[test]
    fn canonical_compiler_api_is_exposed() {
        fn accepts_compiler_api(
            _: &compiler::CompilerApi
        ) {}

        let _ = accepts_compiler_api;
    }

    #[test]
    fn canonical_semantic_result_is_exposed() {
        fn accepts_status(
            _: semantic::SemanticStatus
        ) {}

        let _ = accepts_status;
    }

    #[test]
    fn provider_integration_preserves_existing_authorities() {
        fn accepts_registry(
            _: &providers::standard_library::ProviderRegistry
        ) {}

        let _ = accepts_registry;
    }
}
