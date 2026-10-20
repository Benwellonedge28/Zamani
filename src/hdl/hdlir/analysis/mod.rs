
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Analysis
//!
//! Path: `src/hdl/hdlir/analysis/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97 or later.
//! Safety: Safe Rust only.
//!
//! Status: Analysis composition root. Integrate and validate against the
//! complete repository before declaring this module production-ready.
//!
//! ## Purpose
//!
//! Compose the independent, read-only analyses that derive facts from the
//! existing hardware-domain intermediate representation (HDLIR).
//!
//! This module provides one discoverable namespace for:
//!
//! - Dependency analysis.
//! - Operation-effect analysis.
//! - Logical clock-domain analysis.
//! - Logical resource analysis.
//! - Symbolic and concrete width analysis.
//!
//! Individual analysis modules own their algorithms, data structures,
//! budgets, results, diagnostics, and failure behavior. This module only
//! registers and exposes those implementations.
//!
//! ## Normative authority
//!
//! Implementations registered here must conform to:
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`.
//! - `grammar/specification/semantic-model.md`.
//! - `grammar/hdl/hdl.g4` and applicable delegate grammars.
//! - `grammar/hdl/memories.g4`.
//! - `grammar/hdl/sequential.g4`.
//! - `src/hdl/api.rs`.
//! - `src/hdl/ids.rs`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, once established.
//! - `src/hdl/hdlir/config.rs`.
//! - `src/hdl/hdlir/errors.rs`.
//! - `src/hdl/hdlir/ir/`.
//! - `src/hdl/hdlir/types/`.
//! - `src/hdl/hdlir/schema/`.
//! - `src/hdl/hdlir/verify/`.
//! - `src/zuir/ZUIR-SPEC.md`.
//!
//! The grammar owns source syntax. The canonical frontend owns source-level
//! semantic meaning. ZUIR owns the universal computational representation.
//! HDLIR owns the hardware-domain representation and its domain-specific
//! analyses.
//!
//! ## Ownership
//!
//! This composition root owns:
//!
//! - Registration of implemented analysis modules.
//! - The public namespace boundary for HDLIR analysis.
//! - Documentation of cross-analysis responsibilities.
//!
//! It does not own:
//!
//! - The authoritative HDLIR graph or its entity definitions.
//! - Source parsing, AST construction, or semantic validation.
//! - A competing type, width, resource, effect, or identity system.
//! - Resource-contract verification.
//! - Target discovery, capability inventories, or physical allocation.
//! - Scheduling, synthesis, placement, routing, or timing closure.
//! - Transformation algorithms or proof of behavioral equivalence.
//!
//! ## Analysis contracts
//!
//! ### Dependency analysis
//!
//! `dependencies` derives a deterministic graph of relationships among
//! supplied HDLIR entities. It does not replace the authoritative IR graph
//! or establish that every referenced entity exists.
//!
//! ### Effect analysis
//!
//! `effects` summarizes known operation effects and preserves uncertainty.
//! An unknown effect must not be represented as an empty effect set.
//!
//! ### Clock analysis
//!
//! `clocks` derives facts from logical clock domains and process/event
//! declarations. It does not prove clock-domain-crossing safety, insert
//! synchronizers, build clock trees, or establish timing closure.
//!
//! ### Resource analysis
//!
//! `resources` estimates logical resource quantities and aggregates
//! explicitly supplied resource observations. Symbolic or unavailable
//! quantities remain unresolved. An estimate is not a physical allocation
//! or proof that a target can implement the design.
//!
//! Resource-contract consistency remains the responsibility of
//! `verify::resources` and the canonical semantic resource model.
//!
//! ### Width analysis
//!
//! `widths` evaluates or classifies concrete and symbolic width queries
//! using the existing HDLIR width representation. An unresolved width is
//! not proof of type compatibility, incompatibility, or target feasibility.
//!
//! ## Integration and dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! canonical grammar and frontend semantics
//!                  |
//!                  v
//!                 ZUIR
//!                  |
//!                  v
//!                HDLIR
//!                  |
//!                  v
//!          HDLIR analysis modules
//!                  |
//!                  v
//! verification / transformations / lowering
//!                  |
//!                  v
//!      target-specific realization stages
//! ```
//!
//! This diagram describes responsibility, not a claim that every stage is
//! already implemented or fully connected.
//!
//! The analysis modules must consume the actual APIs of their authoritative
//! dependencies. This composition root must not invent adapters, assume
//! future types exist, or make lower-level frontend modules depend on HDLIR.
//!
//! ## Scalability and budgets
//!
//! No universal hardware maximum is introduced here for widths, signals,
//! ports, operations, clock domains, memories, or other design entities.
//!
//! Each analysis is responsible for its own checked arithmetic, collection
//! growth, work accounting, and explicit budget behavior. Where supported,
//! callers must be able to configure operational budgets independently of
//! hardware semantics.
//!
//! Exhaustion must be reported using the relevant analysis's documented
//! incomplete-result or error contract. Partial results must never be
//! mistaken for complete results.
//!
//! An unbounded configured budget does not promise infinite memory or
//! execution time. Every finite implementation remains subject to available
//! resources, representable values, and supported semantics.
//!
//! ## Determinism
//!
//! Each analysis defines its own determinism contract. Where deterministic
//! output is promised, results must not depend on pointer addresses,
//! incidental hash-map iteration order, or unstable traversal order.
//!
//! This module introduces no shared global state and does not alter the
//! ordering or semantics of an individual analysis.
//!
//! ## Public API policy
//!
//! Each analysis is exposed through its own public submodule. Callers should
//! use explicit paths such as `analysis::widths` or `analysis::resources`.
//!
//! This deliberate namespace separation prevents unrelated analysis types,
//! budgets, findings, and helper names from colliding through broad glob
//! re-exports. Stable convenience re-exports may be introduced later only
//! when their naming, compatibility, and ownership contracts are reviewed.
//!
//! No module is declared here unless its implementation exists. Adding a
//! future analysis requires implementing and testing its own contract first,
//! then registering it here.
//!
//! ## Completion criteria
//!
//! - Every declared child module exists.
//! - Child modules compile against the actual repository APIs.
//! - No competing IR, semantic model, or resource-contract system is added.
//! - Analysis-specific budgets and incomplete outcomes remain explicit.
//! - No universal hardware ceilings or unsafe code are introduced.
//! - Formatting, compilation, tests, and Clippy pass in the actual checkout.

// Independent analysis modules. Their implementations remain authoritative
// for their own public APIs, algorithms, budgets, and result types.

/// Deterministic analysis of dependencies among supplied HDLIR entities.
pub mod dependencies;

/// Analysis of declared operation effects and uncertainty.
pub mod effects;

/// Read-only analysis of logical clock domains and process/event relations.
pub mod clocks;

/// Analysis of logical resource estimates and normalized resource facts.
pub mod resources;

/// Analysis of concrete and symbolic HDLIR widths.
pub mod widths;
