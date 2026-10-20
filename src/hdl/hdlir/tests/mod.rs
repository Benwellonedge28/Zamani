
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Integration Test Composition Root
//!
//! Path: `src/hdl/hdlir/tests/mod.rs`
//! Repository: `Benwellonedge28/Zamani`
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! Status: Integration-test composition root.
//!
//! ## Purpose
//!
//! Register the existing HDLIR test suites under one test-module boundary.
//! This file coordinates test ownership; it does not implement an alternative
//! intermediate representation, serializer, verifier, grammar, or frontend.
//!
//! ## Normative authority
//!
//! Tests registered here must conform to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/Zamani.g4`
//! - `grammar/hdl/hdl.g4` and its applicable delegate grammars
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/lexer/tokens.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/hdl/hdlir/` implementation contracts
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns source syntax. The canonical frontend owns source
//! locations, AST construction, and source-level semantic analysis. ZUIR
//! owns universal computational representation. HDLIR owns the hardware-
//! domain representation and its own construction, verification,
//! transformations, serialization, and lowering.
//!
//! Tests must use the real implementation belonging to each boundary.
//!
//! ## Test ownership
//!
//! - `fixtures`: deterministic fixtures and scenario-catalog integrity.
//! - `conformance`: reusable conformance contracts and applicable adapters.
//! - `determinism`: fingerprint and artifact reproducibility.
//! - `scalability`: progressively larger workloads and explicit budgets.
//! - `malformed_input`: artifact-decoder resilience and malformed inputs.
//! - `compatibility`: repository architecture and artifact compatibility.
//!
//! Focused unit tests remain beside the implementation they exercise.
//! End-to-end tests belong here only when they genuinely cross module
//! boundaries.
//!
//! ## Integration contract
//!
//! This module is intended to be registered by the HDLIR Rust composition
//! root under `#[cfg(test)]`. It must not be declared from production builds.
//!
//! The parent module must be a valid Rust source file and must expose the
//! implementation modules consumed by the registered tests. In particular,
//! tests importing `crate::hdl::hdlir::*` require that `crate::hdl` and its
//! HDLIR module are reachable through the actual crate module tree.
//!
//! This file deliberately does not create placeholder types or conditional
//! substitutes to conceal missing production implementations. If a sibling
//! suite refers to a missing API, repair the owning implementation or revise
//! the suite to test the actual established contract.
//!
//! ## Scalability and correctness
//!
//! No universal hardware-capacity limits are introduced here. Test workload
//! sizes are test inputs, not language limits or production capacity claims.
//!
//! Tests must distinguish:
//!
//! - invalid source semantics;
//! - invalid HDLIR structure or types;
//! - unsupported operations or extensions;
//! - target infeasibility;
//! - operational-budget exhaustion;
//! - incomplete verification or analysis.
//!
//! A skipped check, unsupported operation, incomplete analysis, or exhausted
//! budget must never be counted as successful verification.
//!
//! Large tests must be bounded by explicit test-workload and environment
//! budgets. No test should require literally infinite memory or execution.
//!
//! ## Determinism and isolation
//!
//! Suites must not rely on execution order, global mutable test state,
//! wall-clock time, random iteration order, network availability, or physical
//! hardware being present.
//!
//! Shared fixtures must be immutable or independently constructed. Test
//! identities must not be mistaken for persistent cross-build identities or
//! physical hardware-resource identifiers.
//!
//! ## Safety and failure handling
//!
//! No unsafe Rust is permitted. Malformed external data must be handled by
//! the production parser, decoder, verifier, or other owning API.
//! Tests must not implement a competing decoder or silently reinterpret
//! unsupported input.
//!
//! Panics should be used only by the test framework to report failed
//! assertions, not as the production implementation's error-handling policy.
//!
//! ## Completion criteria
//!
//! This composition root is complete for its current test inventory when:
//!
//! - Every registered source file exists.
//! - Every suite tests its declared ownership boundary.
//! - All test imports resolve to canonical implementation APIs.
//! - No duplicate IR, semantic model, or serialization implementation exists.
//! - Unsupported and incomplete outcomes remain distinguishable from success.
//! - The registered suites compile and run on the supported Rust toolchain.
//! - Formatting, repository tests, and applicable Clippy checks pass.
//!
//! Registering a suite does not establish that its tests pass or that HDLIR
//! is production-ready.

// -----------------------------------------------------------------------------
// Cross-module HDLIR test suites
// -----------------------------------------------------------------------------
//
// Keep these modules private. They are test infrastructure, not part of
// Zamani's public HDLIR API.
//
// The module declarations intentionally correspond to the existing sibling
// files in this directory. Adding a new suite requires adding its source file
// and defining its ownership and integration contract before registration.

mod fixtures;
mod conformance;
mod determinism;
mod scalability;
mod malformed_input;
mod compatibility;
