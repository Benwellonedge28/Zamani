
#![cfg(test)]
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Internal Test Registry
//!
//! Path: src/classical/tests/mod.rs
//! Edition: Rust 2021
//! Intended toolchain: Rust 1.97 or later
//! Safety: Unsafe Rust is forbidden.
//!
//! STATUS
//!
//! Test registration module. It owns no production semantics or IR types.
//!
//! PURPOSE
//!
//! Register the classical subsystem's existing internal conformance and
//! stress-test modules in one place. Only files that exist and are intended
//! to compile as Rust test modules may be registered here.
//!
//! AUTHORITY
//!
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/validation/semantic-boundaries.md
//! - The public contracts of the production modules under src/classical/.
//!
//! OWNERSHIP
//!
//! This module owns test-suite registration only.
//!
//! It does not own:
//! - classical language semantics;
//! - compatibility rules or production diagnostics;
//! - AST or semantic-model definitions;
//! - canonical classical or quantum IR;
//! - resource allocation, scheduling, or hardware capabilities.
//!
//! INTEGRATION
//!
//! The parent classical module must declare this module under `#[cfg(test)]`.
//! The crate root must expose the classical module, and the compatibility
//! production module must expose the API used by the registered tests.
//!
//! The compatibility test suite checks the compatibility contract.
//! The stress test suite exercises scale, deterministic behavior, and
//! boundary conditions of that same contract.
//!
//! Do not register proposed test files until those files exist and compile.
//! Do not duplicate tests already owned by another module merely to increase
//! the apparent size of this suite.
//!
//! SCALABILITY
//!
//! Workload sizes belong to individual tests or explicit test configuration.
//! This registry imposes no semantic ceiling on program size, feature count,
//! memory, processor count, device count, or execution scale.
//!
//! COMPLETION
//!
//! This file is integrated when the parent module can discover these tests,
//! the registered modules compile against the actual production API, and
//! the supported Cargo test commands pass.

mod compatibility;
mod stress;
