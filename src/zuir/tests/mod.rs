
#![cfg(test)]
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Integration Test Registry
//!
//! Path: `src/zuir/tests/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! STATUS
//!
//! Proposed canonical test-module entry point. This file must be connected
//! to the real ZUIR module root before the tests can run.
//!
//! PURPOSE
//!
//! Register the existing ZUIR integration, conformance, property,
//! determinism, serialization, scalability, and robustness test suites.
//!
//! This module is intentionally a registry, not a second implementation of
//! ZUIR functionality. Test logic belongs in the specialized child modules.
//!
//! AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `ZAMANI_IR_SPEC.md`
//! - `src/zuir/ZUIR-SPEC.md`, when present and authoritative
//! - The production modules exercised by each test suite
//!
//! OWNERSHIP
//!
//! OWNS:
//! - Registration of ZUIR test modules.
//! - The high-level test-suite organization.
//! - Documentation of test boundaries and integration requirements.
//!
//! DOES NOT OWN:
//! - Production ZUIR types, builders, or operation semantics.
//! - The verifier, dialect registry, pass manager, or lowering algorithms.
//! - Serialization, canonicalization, or fingerprint implementations.
//! - Source grammar, lexer, parser, AST, or semantic-analysis behavior.
//! - Runtime execution, target discovery, or hardware resource allocation.
//!
//! DEPENDENCY CONTRACT
//!
//! Every child module must test the actual production API and implementation.
//! Tests must not introduce duplicate production types or weaken validation
//! to make a test pass.
//!
//! Child test modules may use `crate::zuir` only when the library exposes
//! that module and the corresponding production modules are declared.
//!
//! SCALABILITY AND POCO-REAF
//!
//! Registering a test suite must not introduce fixed hardware limits.
//! Finite test workloads are test-runner budgets, not language-level limits.
//! Tests must distinguish explicit operational budgets from semantic rules.
//!
//! No finite test suite proves literal infinite scalability. Production
//! implementations remain subject to available memory, time, address space,
//! configured budgets, and target capabilities.
//!
//! GRAMMAR BOUNDARY
//!
//! Grammar tests, when implemented, belong to the frontend's actual lexer
//! and parser test harness. Serialization tests do not establish that source
//! text parses or that decoded data is a semantically valid ZUIR program.
//!
//! INTEGRATION
//!
//! 1. Ensure `src/lib.rs` exposes the ZUIR module.
//! 2. Ensure `src/zuir/mod.rs` is valid Rust and declares the production
//!    modules required by these tests.
//! 3. Declare this file exactly once from `src/zuir/mod.rs` under `cfg(test)`.
//! 4. Keep each child file in this registry at its existing canonical path.
//! 5. Repair `tests/conformance/mod.rs` before registering it.
//! 6. Do not register these modules a second time from `src/lib.rs`.
//!
//! FREEZE CRITERIA
//!
//! - Every declared child module exists and contains valid Rust.
//! - Each suite is registered exactly once.
//! - All child imports resolve against production APIs.
//! - The ZUIR module is part of the compiled library.
//! - Formatting, compilation, tests, and linting pass on Rust 1.97 or later.
//! - No unsafe code or hardware-specific capacity ceilings are introduced.

// -----------------------------------------------------------------------------
// Public API and construction
// -----------------------------------------------------------------------------

mod api;
mod builders;

// -----------------------------------------------------------------------------
// Schema, type, and compatibility contracts
// -----------------------------------------------------------------------------

mod compatibility;
mod schema;
mod types;

// -----------------------------------------------------------------------------
// Core semantics and verification
// -----------------------------------------------------------------------------

mod control_flow;
mod effects;
mod ownership;
mod resources;
mod verifier;

// -----------------------------------------------------------------------------
// Dialect and lowering integration
// -----------------------------------------------------------------------------

mod dialects;
mod lowering;

// -----------------------------------------------------------------------------
// Persistence, repeatability, and incremental compilation
// -----------------------------------------------------------------------------

mod determinism;
mod incremental;
mod serialization;

// -----------------------------------------------------------------------------
// Generated, adversarial, and scalability coverage
// -----------------------------------------------------------------------------

mod property;
mod fuzz;
mod stress;

// -----------------------------------------------------------------------------
// Specification conformance
// -----------------------------------------------------------------------------
//
// The existing `tests/conformance/mod.rs` must be valid Rust before this
// module can be compiled. Its current repository contents are invalid Rust.
// Repair that file as described in the integration notes below.

mod conformance;
