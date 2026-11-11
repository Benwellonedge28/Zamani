
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Testing
//!
//! Path: `src/stdlib/testing/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later.
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Establish the public module boundary for Zamani's standard-library
//! testing infrastructure.
//!
//! The testing subsystem provides reusable facilities for validating
//! the behavior, correctness, compatibility, conformance, and portability
//! of Zamani programs and their implementations.
//!
//! It supports testing across:
//!
//! - Classical computation.
//! - Scientific and numerical computation.
//! - Quantum computation.
//! - Hardware description and HDL.
//! - Classical/quantum hybrid computation.
//! - Distributed computation.
//! - AI and machine learning.
//! - Networking and communication.
//! - Cryptography and security.
//! - Concurrency and asynchronous computation.
//! - Embedded and resource-constrained systems.
//! - Future computational domains.
//!
//! This file composes existing testing modules. It does not implement
//! assertion algorithms, fixture storage, test execution, conformance
//! evaluation, or portability comparison.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! Language and grammar:
//!     grammar/DESIGN.md
//!     grammar/Zamani.g4
//!     grammar/antlr/ZamaniLexer.g4
//!     grammar/antlr/ZamaniParser.g4
//!     grammar/lexer/tokens.g4
//!     grammar/specification/
//!     grammar/spec/
//!
//! Canonical frontend:
//!     src/frontend/lexer/
//!     src/frontend/parser/
//!     src/frontend/ast/
//!     src/frontend/semantic/
//!
//! Canonical computational authorities:
//!     src/classical/
//!     src/quantum/
//!     src/hdl/
//!     src/hybrid/
//!     src/distributed/
//!     src/ai/
//!     src/zuir/
//!
//! Compilation and execution:
//!     src/compiler/
//!     src/backend/
//!     src/runtime/
//!
//! Existing standard-library compatibility framework:
//!     src/stdlib/test_framework.rs
//!
//! ## OWNS
//!
//! This module owns:
//!
//! - The testing subsystem's public module hierarchy.
//! - The public paths to the testing implementations.
//! - The boundary between testing facilities.
//! - The testing subsystem's safe-Rust policy.
//!
//! The child modules own their respective implementations.
//!
//! ## DOES NOT OWN
//!
//! This module does not implement or redefine:
//!
//! - Source grammar or lexical tokens.
//! - Parsing or parser recovery.
//! - AST nodes, source spans, or source identities.
//! - Semantic types, effects, requirements, or capabilities.
//! - Classical, quantum, HDL, or universal IR.
//! - Compiler transformations or optimization.
//! - Runtime execution or scheduling.
//! - Hardware discovery or resource allocation.
//! - Provider registration or provider implementations.
//! - Property-based value generation.
//! - Fuzzing algorithms.
//! - A second test runner or execution scheduler.
//! - A competing assertion or conformance framework.
//!
//! These responsibilities remain with their designated authorities.
//!
//! ## MODULE OWNERSHIP
//!
//! `assertions`:
//!     Assertion evaluation, failure descriptions, numerical tolerances,
//!     and aggregation of assertion failures.
//!
//! `fixtures`:
//!     Validated fixture identities, portable virtual paths, immutable
//!     fixture files, multi-file fixture construction, tags, and metadata.
//!
//! `conformance`:
//!     Conformance identities, domains, dimensions, registration,
//!     execution coordination, outcomes, cancellation, and reports.
//!
//! `portability`:
//!     Execution-environment descriptions, portability cases,
//!     observation comparison, portability outcomes, and reports.
//!
//! Each module is independently responsible for its public contracts.
//!
//! ## DEPENDENCY DIRECTION
//!
//! The intended composition is:
//!
//! ```text
//!                 testing
//!                    |
//!          +---------+---------+
//!          |         |         |
//!          v         v         v
//!     assertions  fixtures  conformance
//!                               |
//!                               v
//!                          portability
//! ```
//!
//! More precisely:
//!
//! - Assertions are an independent testing primitive.
//! - Fixtures are independent test inputs.
//! - Conformance may consume assertion results.
//! - Portability may consume conformance contracts.
//! - Fixtures do not depend on conformance execution.
//! - Assertions do not depend on fixtures or execution.
//!
//! Rust module declaration order does not determine dependency order.
//! The implementation dependencies are defined by the child modules.
//!
//! ## GRAMMAR AND FRONTEND INTEGRATION
//!
//! Testing source-language behavior must use the canonical repository
//! pipeline:
//!
//! ```text
//! Zamani source fixture
//!          |
//!          v
//! canonical lexer and parser
//!          |
//!          v
//! canonical AST
//!          |
//!          v
//! semantic analysis
//!          |
//!          v
//! canonical IR representations
//!          |
//!          v
//! compiler and runtime adapters
//!          |
//!          v
//! observable test results
//!          |
//!          v
//! assertions / conformance / portability
//! ```
//!
//! The testing subsystem supplies test inputs and evaluates results.
//! It does not replace any stage of this pipeline.
//!
//! Grammar tests must consume the authoritative grammar and its generated
//! or repository-maintained parser implementation. They must not introduce
//! an alternative grammar embedded in this module.
//!
//! ## POCO-REAF AND SCALABILITY
//!
//! The testing subsystem supports the objective:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! It must permit the same test specification and fixture to be evaluated
//! against multiple compatible implementations and environments.
//!
//! This module introduces no universal maximum for:
//!
//! - Test cases.
//! - Assertions.
//! - Fixtures.
//! - Fixture files.
//! - Source lengths.
//! - Conformance domains.
//! - Conformance dimensions.
//! - Execution environments.
//! - Providers.
//! - Observations.
//! - Diagnostics.
//! - Comparison results.
//!
//! No fixed CPU, GPU, FPGA, QPU, qubit, memory, register, tensor-rank,
//! device, or distributed-node ceiling is introduced here.
//!
//! Operational test budgets may be imposed by an explicit caller-owned
//! policy. Such budgets must not become language-level capacity limits.
//!
//! Actual capacity is bounded by available memory, address space,
//! implementation capabilities, configured operational budgets,
//! and physical resource availability.
//!
//! This subsystem does not promise literal infinite computation.
//!
//! ## CORRECTNESS AND OUTCOME INVARIANTS
//!
//! Testing results must preserve the difference between:
//!
//! - Passed.
//! - Failed.
//! - Unsupported.
//! - Inconclusive.
//! - Skipped.
//! - Cancelled.
//! - Infrastructure failure.
//!
//! A missing observation must not be treated as a passing result.
//!
//! An incomplete verification must not establish conformance.
//!
//! An unsupported execution environment must not establish portability.
//!
//! A test infrastructure failure must not be silently converted into
//! an ordinary assertion success or program result.
//!
//! Deterministic reports must use the ordering guarantees of their
//! owning implementation.
//!
//! ## SAFETY AND SIDE EFFECTS
//!
//! Safe Rust only.
//!
//! No unsafe code.
//! No raw pointers.
//! No global mutable testing state.
//! No implicit filesystem access.
//! No implicit network access.
//! No implicit hardware discovery.
//! No implicit provider selection.
//! No implicit runtime initialization.
//! No implicit test execution from module import.
//!
//! The testing module must not initialize every standard-library
//! subsystem merely because a test API is imported.
//!
//! ## LEGACY COMPATIBILITY
//!
//! `src/stdlib/test_framework.rs` remains a separate legacy compatibility
//! module during migration.
//!
//! This module does not copy its implementations or silently replace
//! its public API.
//!
//! Migration must be performed through explicit adapters and tests
//! demonstrating compatibility with the canonical testing contracts.
//!
//! In particular, placeholder property-generation behavior must not
//! be treated as a production fixture-generation implementation.
//!
//! ## PARENT INTEGRATION
//!
//! `src/stdlib/mod.rs` must declare this module exactly once:
//!
//!     pub mod testing;
//!
//! `src/lib.rs` already exposes the standard library through:
//!
//!     pub mod stdlib;
//!
//! No additional crate-root declaration is required.
//!
//! The testing module has no initialization or shutdown function.
//! It must not be added to the standard library's manual initialization
//! sequence merely to make its public APIs available.
//!
//! ## PUBLIC API PATHS
//!
//! Assertion API:
//!     crate::stdlib::testing::assertions
//!
//! Fixture API:
//!     crate::stdlib::testing::fixtures
//!
//! Conformance API:
//!     crate::stdlib::testing::conformance
//!
//! Portability API:
//!     crate::stdlib::testing::portability
//!
//! The child modules remain explicitly namespaced. This avoids wildcard
//! re-exports, ambiguous names, and accidental public API collisions.
//!
//! ## FREEZE CRITERIA
//!
//! This module may be frozen when:
//!
//! 1. Every declared child module exists.
//! 2. Every child module compiles independently.
//! 3. All public module paths resolve through `crate::stdlib::testing`.
//! 4. `src/stdlib/mod.rs` declares `pub mod testing;` exactly once.
//! 5. Existing assertion, fixture, conformance, and portability ownership
//!    remains unchanged.
//! 6. Conformance can use the assertion authority without circular
//!    implementation ownership.
//! 7. Portability can use conformance contracts.
//! 8. Fixtures remain independent of parser and runtime implementations.
//! 9. Grammar and frontend authorities are not duplicated.
//! 10. No universal hardware-capacity ceilings are introduced.
//! 11. No implicit initialization or execution occurs.
//! 12. Safe Rust is maintained.
//! 13. Formatting, compilation, tests, and Clippy checks pass.

// =============================================================================
// Public testing modules
// =============================================================================

/// Assertion primitives, structured assertion failures, and assertion
/// report aggregation.
///
/// This module does not execute tests or own test scheduling.
pub mod assertions;

/// Portable, deterministic, in-memory test fixtures.
///
/// This module does not parse source code or access the host filesystem.
pub mod fixtures;

/// Extensible conformance registration, execution coordination,
/// outcomes, and reports.
///
/// This module consumes the shared assertion authority.
pub mod conformance;

/// Cross-environment portability testing and observation comparison.
///
/// This module consumes the conformance contracts and does not discover
/// hardware or select execution providers.
pub mod portability;
