
#![cfg(test)]
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Hybrid subsystem test-module composition for Zamani.
//!
//! Path: `src/hybrid/tests/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//!
//! # Purpose
//!
//! Compose the hybrid subsystem's focused test suites into one unit-test
//! module. This file owns test-module wiring only; production behavior,
//! test fixtures specific to individual suites, and assertions belong to
//! their respective modules.
//!
//! # Canonical integration boundaries
//!
//! - Hybrid language contract: `grammar/spec/hybrid.md`
//! - Hybrid grammar architecture: `grammar/hybrid/README.md`
//! - Hybrid syntax adapter: `crate::frontend::parser::domains::hybrid`
//! - Existing hybrid compilation pipeline:
//!   `crate::compiler::hybrid_pipeline`
//! - Hybrid compilation profiles: `crate::compiler::hybrid_profiles`
//! - Canonical quantum semantics: `crate::quantum::ir`
//! - Hybrid runtime integration: `crate::runtime::hybrid_kernel`
//!
//! These tests must exercise existing authoritative implementations rather
//! than introducing a second parser, AST, semantic model, quantum IR,
//! compiler pipeline, runtime, or hardware-capability registry.
//!
//! # Scalability and resource policy
//!
//! Tests may use finite workloads because every individual test execution
//! consumes finite resources. Such workloads are test inputs, not universal
//! architectural limits.
//!
//! Stress tests must use explicitly configurable workload sizes where
//! appropriate. Operational exhaustion must be reported distinctly from
//! semantic invalidity. No test may establish an arbitrary maximum number
//! of domains, operations, qubits, accelerators, nodes, or other resources
//! as a language-level restriction.
//!
//! # Test-suite responsibilities
//!
//! - `conformance`: hybrid syntax and specification conformance.
//! - `portability`: portable compilation-profile and artifact contracts.
//! - `determinism`: reproducibility of currently supported compiler outputs.
//! - `stress`: configurable workload and resource-exhaustion behavior.
//! - `compatibility`: existing API and integration regression protection.
//!
//! These suites must not claim to prove end-to-end hardware execution,
//! semantic preservation by every backend, or universal target feasibility
//! unless those behaviors are exercised through their canonical APIs.
//!
//! # Integration
//!
//! The parent `src/hybrid/mod.rs` must register this module only for tests:
//
//! ```rust,ignore
//! #[cfg(test)]
//! mod tests;
//! ```
//!
//! The crate must also expose the existing modules required by the child
//! tests. In particular, their `crate::frontend` imports must resolve to
//! the canonical frontend implementation rather than a parallel test-only
//! implementation.

mod compatibility;
mod conformance;
mod determinism;
mod portability;
mod stress;
