
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # ZCIR Lowering and Compatibility Boundaries
//!
//! Canonical path: `src/classical/zcir/lowering/mod.rs`.
//! Edition: Rust 2021.
//! Minimum intended Rust version: 1.97.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Define the public module boundary for conversions between the Zamani
//! Classical Intermediate Representation (ZCIR), canonical ZUIR entities,
//! and the existing legacy IR representation.
//!
//! This module composes adapters. It does not implement a second IR,
//! allocate identifiers, perform semantic analysis, select hardware,
//! schedule execution, or duplicate canonical verification.
//!
//! ## Architectural authority
//!
//! The following existing modules remain authoritative for their respective
//! responsibilities:
//!
//! - `crate::zuir::ir`: canonical shared IR entities.
//! - `crate::zuir::ids`: canonical entity identities.
//! - `crate::zuir::verify`: canonical IR verification.
//! - `crate::ir_gen`: legacy AST-to-IR representation and generation.
//! - `crate::classical::zcir::ir`: ZCIR-facing canonical IR API.
//! - `crate::classical::zcir::ids`: approved identity integration.
//! - `crate::classical::zcir::lowering::to_zuir`: ZCIR-facing to ZUIR
//!   identity adapter.
//! - `crate::classical::zcir::lowering::from_zuir`: ZUIR to ZCIR-facing
//!   identity adapter.
//! - `crate::classical::zcir::lowering::legacy_adapter`: legacy
//!   compatibility assessment.
//!
//! Source syntax and semantic meaning remain governed by the grammar,
//! frontend, and semantic model. Lowering must consume resolved information
//! rather than recreate parsing or type inference.
//!
//! ## Public modules
//!
//! - [`to_zuir`] exposes identity conversion for entities already backed by
//!   canonical ZUIR.
//! - [`from_zuir`] exposes the reverse identity boundary for those same
//!   canonical entities.
//! - [`legacy_adapter`] assesses compatibility risks in the independent
//!   legacy IR representation.
//!
//! These adapters have deliberately different contracts. The first two
//! preserve canonical entities by identity; the legacy adapter does not
//! claim that the legacy representation has been converted.
//!
//! ## Ownership and non-goals
//!
//! This module owns only the composition and public exposure of lowering
//! adapters. It does not own:
//!
//! - A parallel classical IR graph or duplicate identifier system.
//! - Type conversion rules or semantic inference.
//! - Dialect operation definitions or legalization.
//! - Whole-program structural, SSA, type, effect, or resource verification.
//! - Target-specific layout, device allocation, scheduling, or code emission.
//! - Serialization formats or version negotiation.
//!
//! ## Integration contract
//!
//! 1. Keep each adapter in its own file with a documented, narrow contract.
//! 2. Do not add an adapter to this module until its source file exists.
//! 3. Do not route a legacy IR artifact through an identity adapter.
//! 4. Do not infer that a successful identity conversion makes an entity
//!    valid, portable, verified, or executable.
//! 5. Any future legacy-to-ZUIR converter must have a separate API and must
//!    preserve or explicitly reject unsupported types, values, control flow,
//!    effects, provenance, and target-sensitive semantics.
//! 6. Verify converted entities with canonical ZUIR verification and the
//!    relevant dialect contracts before optimization or execution.
//! 7. Register this module through `src/classical/zcir/mod.rs`, then register
//!    ZCIR through `src/classical/mod.rs` and the crate root as appropriate.
//!    Do not register missing modules or introduce cyclic dependencies.
//!
//! ## Scalability and POCO-REAF
//!
//! This module introduces no fixed maximum for functions, blocks, operations,
//! values, modules, devices, threads, or other resource populations.
//!
//! Identity adapters add no graph traversal or reconstruction. Legacy
//! assessment work is proportional to the artifact inspected. Actual
//! compilation and execution remain subject to available memory, address
//! space, identifier representation, implementation support, explicit
//! operational budgets, and target feasibility.
//!
//! POCO-REAF requires preserving program meaning across supported targets;
//! it does not imply unlimited physical resources or that every target
//! supports every operation.
//!
//! ## Stability policy
//!
//! The public adapter modules are exposed without duplicating their functions
//! at this module root. This keeps each API's namespace explicit and avoids
//! accidental collisions as additional conversion boundaries are added.
//!
//! A new public adapter must document its ownership, supported conversions,
//! failure behavior, semantic-preservation guarantees, scalability behavior,
//! and verification requirements before it is registered here.

/// Identity adapter from the ZCIR-facing API to canonical ZUIR entities.
///
/// This adapter accepts only entities implementing its explicit canonical
/// representation contract. It does not convert an independent legacy IR.
pub mod to_zuir;

/// Identity adapter from canonical ZUIR entities to the ZCIR-facing API.
///
/// This adapter preserves the canonical entity rather than reconstructing a
/// second representation. It does not validate the containing IR graph.
pub mod from_zuir;

/// Compatibility assessment for the existing legacy IR representation.
///
/// This adapter reports migration risks. It does not claim to construct a
/// canonical ZUIR artifact or to establish lossless conversion.
pub mod legacy_adapter;
