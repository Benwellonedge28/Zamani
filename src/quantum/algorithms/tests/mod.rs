//! Cross-module test suite for `crate::quantum::algorithms`.
//!
//! This module is intentionally limited to test-module assembly. Production
//! contracts remain owned by their implementation modules, while each test
//! file owns one verification concern:
//!
//! - [`determinism`] verifies reproducibility of deterministic executions.
//! - [`integration`] verifies cross-module contract composition and accounting.
//! - [`properties`] verifies subsystem-wide invariants of shared contracts.
//!
//! The tests remain backend-independent. They must not require a QPU, vendor
//! SDK, simulator implementation, network access, calibration data, routing,
//! transpilation, or error-correction hardware.
//!
//! # Module ownership
//!
//! This file only assembles the test modules. It must not define production
//! types, duplicate validation rules, or contain implementation-specific test
//! logic. Adding or changing a test concern therefore does not require changes
//! to the other test modules.
//!
//! # Integration
//!
//! `src/quantum/algorithms/mod.rs` must include this test tree with:
//!
//! ```text
//! #[cfg(test)]
//! mod tests;
//! ```
//!
//! The declarations below use explicit paths so the intended file layout is
//! unambiguous and `properties.rs` is part of the same test module tree.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1. No nightly features or external test dependencies.
//!
//! # Safety
//!
//! This module contains no unsafe code.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

/// Determinism and reproducibility contract tests.
#[path = "determinism.rs"]
mod determinism;

/// Cross-module algorithm contract and accounting tests.
#[path = "integration.rs"]
mod integration;

/// Shared type and subsystem invariant/property tests.
#[path = "properties.rs"]
mod properties;