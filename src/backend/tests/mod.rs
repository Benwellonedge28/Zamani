
#![cfg(test)]
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Backend test-suite composition root for Zamani.
//!
//! Path: `src/backend/tests/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Compose the existing backend contract and regression tests into one
//! discoverable test suite. This module owns test organization only;
//! it does not implement production backend behavior or define a competing
//! backend API.
//!
//! # Test coverage
//!
//! - `artifacts`: artifact identity, metadata, encoding, payload integrity,
//!   and artifact-local validation contracts.
//! - `compatibility`: capability requirements, resource requirements,
//!   compatibility reports, and explicit incompatibility.
//! - `contract`: shared behavior of the existing backend API.
//! - `determinism`: repeatability of backend generation.
//! - `registry`: registration and discovery behavior.
//! - `regression`: known backend defects and their acceptance criteria.
//! - `scalability`: configurable resource budgets and large-input behavior.
//! - `security`: safe handling of paths, artifacts, and external-tool
//!   boundaries.
//! - `targets`: target identity, architecture dispatch, and target-specific
//!   behavior.
//!
//! # Integration boundary
//!
//! The canonical backend implementation and its public exports must be
//! resolved before this suite can be considered integrated into a passing
//! repository build.
//!
//! The repository currently contains both `src/backend.rs` and
//! `src/backend/mod.rs`. Rust cannot use both files as the implementation
//! root for the same `backend` module. This file deliberately does not
//! attempt to repair that conflict or create a second backend registry.
//!
//! Once the module ownership migration is approved, the selected canonical
//! backend root must expose the production modules required by the existing
//! tests, and mount this file exactly once under `#[cfg(test)]`.
//!
//! # Resource and portability contract
//!
//! These tests must not impose universal maximum counts for CPUs, threads,
//! qubits, accelerators, devices, nodes, tensor dimensions, or memory.
//! Finite test fixtures are examples, not language-level resource limits.
//!
//! Resource exhaustion, missing target capabilities, unavailable toolchains,
//! and unsupported operations must remain distinguishable outcomes.
//! Tests must not treat silent semantic substitution as successful fallback.
//!
//! # Offline behavior
//!
//! Test discovery and ordinary contract tests must not require access to a
//! physical QPU, FPGA, GPU, cluster, vendor account, or network service.
//! Tests that need an external compiler or simulator must explicitly detect
//! the toolchain, distinguish an unavailable prerequisite from a passing
//! validation, and never fabricate successful validation evidence.
//!
//! # Grammar and semantic ownership
//!
//! The grammar and semantic specifications define source-language meaning.
//! Backend tests verify that target integration respects those contracts.
//! They must not duplicate grammar rules, replace semantic analysis, or
//! move routing, scheduling, quantum error correction, or physical resource
//! allocation into this test module.
//!
//! # Completion criteria
//!
//! This composition root is complete when every existing suite is mounted
//! exactly once, all declared file paths resolve, and the canonical backend
//! module exposes the interfaces those suites actually exercise.
//!
//! A successful compilation of this file alone does not establish that
//! individual tests pass, target artifacts are valid, or POCO-REAF has been
//! achieved.

// The repository currently stores the artifact test file under the literal
// filename " artifacts.rs" (including its leading space). Preserve the
// existing file and reference it explicitly. Renaming it should be a
// separate, deliberate repository change.
#[path = " artifacts.rs"]
mod artifacts;

mod compatibility;
mod contract;
mod determinism;
mod registry;
mod regression;
mod scalability;
mod security;
mod targets;
