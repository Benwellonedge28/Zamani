
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Classical Intermediate Representation — Analysis
//!
//! Path: `src/classical/zcir/analysis/mod.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97
//!
//! ## Purpose
//!
//! Compose the public ZCIR analysis modules. Each analysis derives facts from
//! canonical IR or validated analysis inputs; this module does not define an
//! alternative IR, identity system, or program-semantics authority.
//!
//! ## Ownership boundaries
//!
//! - Canonical IR identities and IR structures belong to ZUIR.
//! - Canonical definition-use indexing belongs to `crate::zuir::ir::use_def`.
//! - `control_flow` exposes ZCIR's current CFG analysis boundary.
//! - `liveness` computes block-level value liveness from supplied facts.
//! - `complexity` computes structural counts and caller-supplied cost estimates.
//! - Cross-analysis orchestration belongs in a later integration layer, not
//!   in this module root.
//!
//! ## Integration contract
//!
//! This module is registered by `src/classical/zcir/mod.rs` using
//! `pub mod analysis;` once that parent is valid Rust.
//!
//! Consumers should import the narrowest module that owns the required
//! analysis. Avoid flat re-exports of every analysis type: explicit paths make
//! ownership and API evolution clearer and prevent accidental name collisions.
//!
//! The modules must continue to use canonical ZUIR IDs. Any future CFG adapter
//! should translate verified canonical blocks and terminators into analysis
//! inputs without creating a competing identity or IR representation.
//!
//! ## Scalability and safety
//!
//! This composition root defines no graph-size, operation-count, hardware,
//! memory, or concurrency ceiling. Individual analyses must document their
//! own complexity, arithmetic behavior, invalidation rules, and failure modes.
//! Safe Rust only; no `unsafe` code is permitted in this module tree.
//!
//! ## Test organization
//!
//! Unit tests belong beside their implementations. Cross-module and
//! repository-level conformance tests belong in the crate's established test
//! layout. Do not declare a `tests` module here unless a corresponding Rust
//! source module actually exists and is intended to be compiled as a child
//! module.

/// Structural complexity and abstract cost analysis.
pub mod complexity;

/// Control-flow graph construction and analysis.
pub mod control_flow;

/// Block-level value liveness analysis.
pub mod liveness;

/// Classical-facing access to canonical ZUIR definition-use analysis.
pub mod use_def;
