
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Semantic Analysis — Test Orchestrator
//!
//! File: `src/frontend/semantic/tests/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! This module is the single aggregation boundary for the semantic-analysis
//! test suite. It registers the subsystem's focused conformance, integration,
//! compatibility, determinism, and scalability tests.
//!
//! The Rust test harness discovers and executes tests declared in these
//! child modules. This file deliberately does not implement a second test
//! runner or duplicate the assertions owned by individual test modules.
//!
//! ## AUTHORITY
//!
//! Language and grammar authority:
//! - `grammar/Zamani.g4`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/syntax.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! Implementation authority:
//! - `src/frontend/ast/node/`
//! - `src/frontend/parser/`
//! - `src/frontend/semantic/`
//! - The canonical downstream IR interfaces.
//!
//! The normative specifications define language meaning. The canonical AST
//! defines source structure and identity. The semantic implementation defines
//! the analysis interfaces exercised by these tests. A test must not silently
//! replace any of those authorities.
//!
//! ## OWNS
//!
//! - Registration of semantic test modules.
//! - Test-suite organization and ownership documentation.
//! - The integration contract between the semantic root and its test suite.
//! - Suite-wide safety, determinism, portability, and scalability principles.
//!
//! ## DOES_NOT_OWN
//!
//! This module does not implement:
//! - Semantic analysis or type checking.
//! - AST construction, parsing, or grammar productions.
//! - Symbol resolution or type inference.
//! - Effect, capability, resource, or contract evaluation.
//! - Classical, quantum, HDL, hybrid, AI/data, or distributed semantics.
//! - Diagnostic production or formatting.
//! - Incremental-analysis algorithms or caches.
//! - Target discovery, physical allocation, routing, scheduling, or execution.
//! - A competing semantic model, AST, diagnostic type, or IR.
//!
//! Those responsibilities remain with their authoritative implementation
//! modules. Test helpers that are specific to a subsystem belong in that
//! subsystem's test file unless a separately established shared helper module
//! is introduced with an explicit contract.
//!
//! ## ARCHITECTURAL POSITION
//!
//! ```text
//! Canonical grammar
//!        |
//!        v
//! Lexer / parser
//!        |
//!        v
//! Canonical frontend AST
//!        |
//!        v
//! Semantic analysis
//!        |
//!        +-- symbols and name resolution
//!        +-- types, generics, and ownership
//!        +-- effects, capabilities, and resources
//!        +-- contracts and domain validation
//!        +-- diagnostics and semantic provenance
//!        +-- deterministic and incremental analysis
//!        |
//!        v
//! Validated semantic result
//!        |
//!        v
//! Canonical domain IR boundaries
//!        |
//!        v
//! Target-specific lowering and execution
//! ```
//!
//! The tests verify contracts at their appropriate boundaries. Passing a
//! semantic test does not, by itself, establish that parsing, IR lowering,
//! target feasibility, or execution has been verified.
//!
//! ## MODULE OWNERSHIP
//!
//! Each child module owns its focused tests:
//!
//! - `api`: public analyzer API, result states, configuration, and invocation
//!   isolation.
//! - `symbols`: declarations, scopes, symbol tables, imports, and resolution.
//! - `types`: semantic types, compatibility, constraints, and unification.
//! - `generics`: generic arguments, substitutions, bounds, and obligations.
//! - `ownership`: the ownership and lifetime rules specified by Zamani.
//! - `effects`: effect identities, inference, composition, and checking.
//! - `capabilities`: capability requirements, evidence, and compatibility.
//! - `resources`: symbolic resource quantities, requirements, and constraints.
//! - `contracts`: preconditions, postconditions, invariants, and obligations.
//! - `classical`: classical computation semantics.
//! - `quantum`: quantum computation semantics and classical/quantum boundaries.
//! - `hdl`: hardware-description-language semantic contracts.
//! - `hybrid`: cross-domain composition and semantic boundaries.
//! - `ai_data`: AI, machine-learning, tensor, and data semantics.
//! - `distributed`: distributed computation and communication semantics.
//! - `extensions`: extensible semantic identities, registration, and dispatch.
//! - `diagnostics`: structured diagnostics, locations, and stable codes.
//! - `determinism`: repeatability and stable semantic outputs.
//! - `incremental`: dependency tracking, invalidation, and full-analysis
//!   equivalence.
//! - `compatibility`: supported legacy interfaces and migration contracts.
//!
//! Each test module must use the canonical production interfaces. It must not
//! introduce a second implementation merely to make a test pass.
//!
//! ## INTEGRATION CONTRACT
//!
//! The semantic root must register this file only in test builds:
//!
//! ```rust
//! #[cfg(test)]
//! mod tests;
//! ```
//!
//! Place that declaration in `src/frontend/semantic/mod.rs` after the
//! semantic module hierarchy has been established.
//!
//! The parent module owns the decision to include the test suite. This file
//! owns registration of the child test modules. Do not register these same
//! files a second time through an alternative test hierarchy.
//!
//! The child modules use the canonical path
//! `crate::frontend::semantic::...` for production semantic APIs and
//! `crate::frontend::ast::node` for AST-owned types.
//!
//! These paths must be reconciled with the actual public module declarations
//! before the semantic subsystem is considered integrated.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! The suite must preserve the distinction between:
//!
//! - Language semantics and compiler operational budgets.
//! - Source validity and target feasibility.
//! - Resource requirements and physical resource allocation.
//! - Capabilities and evidence of actual target support.
//! - Preferences and mandatory constraints.
//! - Semantic facts and target-specific implementation decisions.
//!
//! No test workload in this suite defines a language-level maximum for
//! hardware resources, type parameters, domain extensions, declarations,
//! symbols, tensor dimensions, or computational scale.
//!
//! Tests may use finite generated workloads to exercise growth, overflow
//! handling, resource accounting, and algorithmic behavior. Such workloads
//! are test inputs, not universal language limits.
//!
//! No implementation can guarantee completion for literally infinite input.
//! The required behavior is to support representable computations subject to
//! available resources, language semantics, implementation capabilities,
//! configured operational budgets, and physical constraints.
//!
//! When an operational budget is exhausted, analysis must return or record an
//! explicit incomplete, failed, or resource-exhausted outcome. It must never
//! silently certify incomplete analysis as semantically valid.
//!
//! ## DETERMINISM AND TEST ISOLATION
//!
//! Tests must not rely on:
//! - Execution order between test functions or test modules.
//! - Shared mutable global state.
//! - A particular machine, vendor, device, or execution provider.
//! - Hash-map iteration order as a semantic ordering guarantee.
//! - Environment-specific resource availability.
//! - Timing assumptions for correctness.
//!
//! Equivalent analysis inputs, configurations, and extension-registry
//! snapshots must yield equivalent semantic results and deterministically
//! ordered diagnostics wherever the public contract requires determinism.
//!
//! Parallel test execution must not change correctness.
//!
//! ## ERROR HANDLING
//!
//! Invalid, unresolved, unsupported, incomplete, and resource-exhausted
//! inputs must remain distinguishable according to the owning API contract.
//!
//! Tests must not treat a missing API, an unimplemented feature, or a failed
//! test setup as evidence that the language construct is invalid.
//!
//! Missing production interfaces are integration failures and must be
//! resolved in their owning modules rather than hidden by duplicate test-only
//! production implementations.
//!
//! ## FREEZE CRITERIA
//!
//! This file is ready to freeze when:
//!
//! 1. Every registered child file exists at the expected path.
//! 2. Every child module is included exactly once in the unit-test hierarchy.
//! 3. The parent semantic module includes this file only for test builds.
//! 4. Child modules use the agreed canonical AST and semantic APIs.
//! 5. The test suite compiles with the supported Rust toolchain.
//! 6. The full suite passes without relying on test execution order.
//! 7. No unsafe code or machine-capacity ceiling is introduced.
//! 8. Missing features fail visibly rather than being silently skipped.
//! 9. All test-only helpers remain subordinate to their owning contracts.
//!
//! A successful module declaration is not proof that the entire semantic
//! subsystem is production-ready. Repository integration and test execution
//! are separate acceptance gates.

// =============================================================================
// Foundational semantic API and analysis
// =============================================================================

mod api;

// =============================================================================
// Name resolution and type-system contracts
// =============================================================================

mod symbols;
mod types;
mod generics;
mod ownership;

// =============================================================================
// Cross-cutting semantic obligations
// =============================================================================

mod effects;
mod capabilities;
mod resources;
mod contracts;

// =============================================================================
// Computational-domain conformance
// =============================================================================

mod classical;
mod quantum;
mod hdl;
mod hybrid;
mod ai_data;
mod distributed;

// =============================================================================
// Extension and diagnostic contracts
// =============================================================================

mod extensions;
mod diagnostics;

// =============================================================================
// Determinism, incremental analysis, and migration compatibility
// =============================================================================

mod determinism;
mod incremental;
mod compatibility;
