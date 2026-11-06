
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Meta-Compiler — Compiler Testing
//!
//! Path:
//!     src/compiler/testing/mod.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! This module is the composition boundary for compiler testing.
//!
//! It exposes the independently implemented testing frameworks through
//! one stable compiler-testing namespace.
//!
//! The testing infrastructure supports:
//!
//! - Compiler fixtures and fixture catalogs.
//! - Language and compiler conformance.
//! - Differential compilation and execution testing.
//! - Determinism and reproducibility testing.
//! - Scalability testing.
//! - Regression testing.
//! - Classical, quantum, AI, tensor, HDL and hybrid computing tests.
//! - Distributed and heterogeneous computing tests.
//! - Cross-target portability testing.
//! - Explicit failure and infeasibility testing.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Testing-module composition.
//! - Public testing-module exposure.
//! - Testing API version identification.
//! - Module-boundary validation.
//!
//! This module does not own:
//!
//! - Source-language grammar.
//! - Lexical analysis.
//! - Parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Type inference.
//! - Ownership analysis.
//! - Canonical IR definitions.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Backend implementations.
//! - Runtime execution.
//! - Production compilation orchestration.
//!
//! Those responsibilities remain with their canonical repository owners.
//!
//! # Component ownership
//!
//! fixtures.rs:
//!     Fixture identity, metadata, expectations and fixture catalogs.
//!
//! conformance.rs:
//!     Expected-versus-observed compiler behavior.
//!
//! differential.rs:
//!     Comparison of observations from different compiler and execution
//!     configurations.
//!
//! determinism.rs:
//!     Internal Rust tests for the compiler's determinism integration.
//!
//! scalability.rs:
//!     Streaming scalability workloads and resource-budget validation.
//!
//! regression.rs:
//!     Regression-case execution, results and statistics.
//!
//! This module does not reimplement any of these components.
//!
//! # Grammar authority
//!
//! Testing must respect the repository's established authority chain:
//!
//!     grammar/specification/
//!             |
//!             v
//!     grammar/spec/
//!             |
//!             v
//!     grammar/Zamani.g4
//!             |
//!             v
//!     grammar/antlr/
//!             |
//!             v
//!     lexer and parser
//!             |
//!             v
//!     AST
//!             |
//!             v
//!     semantic analysis
//!             |
//!             v
//!     canonical IR
//!             |
//!             v
//!     compiler and backend
//!             |
//!             v
//!     runtime
//!
//! The testing framework tests the implementation against these
//! authorities. It does not replace them.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Testing must establish that a program's meaning is preserved across
//! supported compilation configurations and target realizations.
//!
//! A passing frontend test does not prove backend portability.
//!
//! A successful backend invocation does not prove runtime correctness.
//!
//! An unavailable target must not be reported as a successful execution.
//!
//! # Scalability
//!
//! This module imposes no universal maximum on:
//!
//! - Test cases.
//! - Fixtures.
//! - Source bytes.
//! - Compilation units.
//! - Compilation stages.
//! - Computing domains.
//! - Hardware targets.
//! - Devices.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Distributed nodes.
//! - Tensor dimensions.
//! - IR operations.
//! - Regression records.
//! - Scalability workloads.
//!
//! Individual components use streaming interfaces where appropriate.
//!
//! Actual workload capacity remains subject to available memory,
//! execution time, operating-system limits and explicitly configured
//! resource budgets.
//!
//! No finite implementation promises infinite physical resources.
//!
//! # Determinism
//!
//! This module does not define a competing determinism policy.
//!
//! Determinism tests use the existing compiler parallel-determinism
//! integration and the canonical classical concurrency determinism
//! authority.
//!
//! Test ordering must not depend on random ordering, worker completion
//! timing or unordered collection iteration.
//!
//! # Integration
//!
//! The compiler root must expose this module:
//!
//!     crate::compiler::testing
//!
//! The existing compiler configuration, context and parallel modules
//! must also be registered by the compiler root because the existing
//! testing components depend on those modules.
//!
//! # Freeze criteria
//!
//! This module is ready to freeze when:
//!
//! - Every declared child module exists.
//! - Every child module compiles.
//! - All component APIs remain owned by their respective files.
//! - Compiler-root registration is complete.
//! - Module-boundary tests pass.
//! - The complete compiler test suite passes.
//! - Formatting, compilation and lint checks pass.

// ============================================================================
// TESTING API VERSION
// ============================================================================

/// Public API version of the compiler testing composition contract.
///
/// This version is independent of the Zamani language, grammar, semantic
/// model, IR, backend and artifact versions.
///
/// Individual testing components retain their own API versions.
pub const COMPILER_TESTING_API_VERSION: u16 = 1;

// ============================================================================
// FIXTURE MANAGEMENT
// ============================================================================

/// Compiler fixture definitions, metadata, expectations and catalogs.
///
/// This is the canonical fixture owner for compiler testing.
pub mod fixtures;

// ============================================================================
// CONFORMANCE TESTING
// ============================================================================

/// Language and compiler conformance execution.
///
/// This component compares actual compiler observations with
/// fixture expectations.
pub mod conformance;

// ============================================================================
// DIFFERENTIAL TESTING
// ============================================================================

/// Differential compilation and execution testing.
///
/// This component delegates semantic equivalence decisions to
/// caller-supplied comparison contracts.
pub mod differential;

// ============================================================================
// SCALABILITY TESTING
// ============================================================================

/// Streaming scalability testing and resource-budget validation.
///
/// This component executes caller-supplied workloads and records
/// their actual completion and failure outcomes.
pub mod scalability;

// ============================================================================
// REGRESSION TESTING
// ============================================================================

/// Regression-case definitions, execution and reporting.
///
/// This component is the canonical regression-test runner.
pub mod regression;

// ============================================================================
// INTERNAL DETERMINISM TESTS
// ============================================================================

/// Compiler determinism integration tests.
///
/// The determinism implementation is owned by
/// `crate::compiler::parallel::determinism`.
///
/// This module contains tests rather than another production
/// determinism implementation.
#[cfg(test)]
mod determinism;

// ============================================================================
// MODULE-BOUNDARY TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// All independently versioned testing components must expose
    /// valid, non-zero API versions.
    #[test]
    fn testing_component_api_versions_are_available() {
        assert_ne!(COMPILER_TESTING_API_VERSION, 0);
        assert_ne!(fixtures::FIXTURE_CATALOG_API_VERSION, 0);
        assert_ne!(conformance::CONFORMANCE_API_VERSION, 0);
        assert_ne!(differential::DIFFERENTIAL_API_VERSION, 0);
        assert_ne!(scalability::SCALABILITY_TESTING_API_VERSION, 0);
        assert_ne!(regression::REGRESSION_API_VERSION, 0);
    }

    /// The built-in fixture catalog must be constructible and non-empty.
    #[test]
    fn built_in_fixture_catalog_is_available() {
        let catalog = fixtures::FixtureCatalog::with_builtins()
            .expect("the built-in compiler fixture catalog must be valid");

        assert!(!catalog.is_empty());
    }

    /// The regression framework must be available through the
    /// compiler testing composition boundary.
    #[test]
    fn regression_runner_is_exposed() {
        let runner = regression::RegressionRunner::new();

        assert_eq!(
            runner.options().expected_cases(),
            None
        );
    }

    /// The scalability framework must be available through the
    /// compiler testing composition boundary.
    #[test]
    fn scalability_options_are_exposed() {
        let options = scalability::ScalabilityRunOptions::new();

        assert_eq!(options.expected_items(), None);
        assert!(!options.has_budget());
    }

    /// The conformance framework must expose its canonical stage
    /// identifiers through this module.
    #[test]
    fn conformance_stage_identifiers_are_available() {
        assert_eq!(
            conformance::stage::FRONTEND_PARSE,
            "frontend.parse"
        );

        assert_eq!(
            conformance::stage::SEMANTIC_VALIDATION,
            "semantic.validation"
        );

        assert_eq!(
            conformance::stage::IR_VERIFICATION,
            "ir.verification"
        );
    }
}
