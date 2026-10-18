
#![cfg(test)]
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Test Suite Registry
//!
//! Canonical path: `src/classical/zcir/tests/mod.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety policy: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Register the focused test suites for the Zamani Classical Intermediate
//! Representation (ZCIR).
//!
//! This module is a composition point for tests. It must not implement
//! production IR entities, duplicate canonical ZUIR behavior, or replace
//! focused assertions with a single broad test that is difficult to
//! diagnose.
//!
//! ## Architectural authority
//!
//! The suites registered here must respect the following boundaries:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/`
//! - `src/zuir/verify/`
//! - `src/zuir/serialization/`
//! - `src/classical/zcir/ir/`
//! - `src/classical/zcir/lowering/`
//! - `src/classical/zcir/serialization/`
//! - `src/classical/zcir/verify/`
//!
//! The grammar defines source syntax. The semantic model defines the
//! established meaning of a program. ZUIR owns the canonical shared
//! computational representation and its applicable infrastructure.
//! ZCIR exposes the classical integration boundary without introducing
//! a competing IR graph or identity system.
//!
//! ## Ownership
//!
//! This file owns:
//!
//! - Registration of the ZCIR test modules.
//! - The test-suite composition boundary.
//! - Documentation of suite responsibilities and integration requirements.
//! - The test-side prohibition on unsafe Rust.
//!
//! This file does not own:
//!
//! - Production IR types, IDs, values, operations, or control-flow graphs.
//! - Source parsing, AST construction, name resolution, or semantic analysis.
//! - Verification, optimization, lowering, or serialization algorithms.
//! - Hardware discovery, resource allocation, scheduling, or execution.
//! - Claims that a target is supported merely because a test compiles.
//!
//! ## Registered test suites
//!
//! - `conformance`: public ZCIR contracts and canonical representation
//!   compatibility.
//! - `invariants`: local structural invariants and rejection behavior.
//! - `scalability`: dynamic growth and integrity under finite test workloads.
//! - `determinism`: stable serialization behavior under the canonical
//!   ZUIR serialization contract.
//! - `interoperability`: ZCIR/ZUIR adapter and serialization integration.
//! - `portability`: target-independent representation contracts and
//!   explicit operational-budget behavior.
//!
//! Each suite owns its own fixtures and assertions. Shared production
//! behavior must be tested through its authoritative implementation rather
//! than reimplemented in this registry.
//!
//! ## Integration contract
//!
//! The parent `src/classical/zcir/mod.rs` must declare this registry under
//! `#[cfg(test)] mod tests;` after the parent file has been made valid Rust.
//!
//! The repository's `src/classical/mod.rs` must also expose the ZCIR module
//! if ZCIR is intended to be part of the public classical subsystem.
//!
//! Each child declaration below requires the corresponding source file to
//! exist at this directory's path. Do not register a planned or nonexistent
//! test file before its implementation is added.
//!
//! The child modules remain private to the test registry. They do not need
//! to become part of ZCIR's public API.
//!
//! ## Scalability and POCO-REAF
//!
//! Test workload sizes are finite operational choices made to keep tests
//! executable on available development machines. They are not universal
//! language, IR, memory, operation-count, processor-count, or device-count
//! limits.
//!
//! No finite test suite proves literal infinity or universal target support.
//! Scalability claims must instead be supported by explicit resource
//! policies, checked operations, dynamic storage where appropriate,
//! structured failure behavior, and representative stress testing.
//!
//! ## Determinism and failure behavior
//!
//! Tests must not rely on nondeterministic iteration order unless that order
//! is explicitly part of the tested contract. Invalid inputs and exhausted
//! operational budgets should produce the owning API's documented errors,
//! not silent truncation or test-only production behavior.
//!
//! ## Freeze criteria
//!
//! - Every registered file exists and is compiled by the test build.
//! - Each suite tests its stated responsibility through actual production
//!   APIs.
//! - No duplicate IR implementation or identity system is introduced.
//! - Unsafe Rust remains forbidden.
//! - Test budgets are not promoted into universal semantic limits.
//! - Formatting, compilation, tests, and linting pass at the repository's
//!   declared Rust-version baseline.

// Public API and representation conformance.
mod conformance;

// Structural and cross-entity invariants.
mod invariants;

// Dynamic growth under explicit test workloads.
mod scalability;

// Canonical serialization determinism.
mod determinism;

// ZCIR/ZUIR adapter and representation interoperability.
mod interoperability;

// Target-independent representation and portability contracts.
mod portability;
