
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Builders
//!
//! Path: `src/hdl/hdlir/builder/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide the composition root for checked HDLIR construction APIs.
//!
//! This module registers the builders that construct the canonical HDLIR
//! design, module, and operation representations. It does not define a
//! competing IR model, duplicate schema definitions, or implement
//! graph-wide verification.
//!
//! ## Architectural authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/ports.g4`
//! - `grammar/specification/semantic-model.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/ir/design.rs`
//! - `src/hdl/hdlir/ir/module.rs`
//! - `src/hdl/hdlir/ir/operations.rs`
//! - `src/hdl/hdlir/schema/operation.rs`
//! - `src/hdl/hdlir/schema/registry.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns HDL source syntax. The frontend owns canonical ASTs,
//! source locations, and source-level semantic validation. ZUIR owns the
//! universal computational representation. HDLIR owns hardware-domain
//! representation and construction contracts.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Registration of the implemented HDLIR builder modules.
//! - Deliberate public exports of the builder APIs.
//! - The shared safe-Rust policy for this builder subsystem.
//!
//! It does not own:
//!
//! - Design, module, or operation storage.
//! - ID allocation or persistent identity.
//! - Schema registration or operation definitions.
//! - Source parsing, name resolution, or semantic analysis.
//! - Graph-wide verification or semantic-equivalence proofs.
//! - Synthesis, scheduling, placement, routing, or physical allocation.
//! - Serialization or target-specific realization.
//!
//! Each builder delegates representation invariants to its canonical owner.
//!
//! ## Construction boundaries
//!
//! - `design` constructs the canonical design registry.
//! - `module` constructs the canonical logical module representation.
//! - `operation` constructs operations against existing schema contracts.
//!
//! Builders may reject locally invalid construction requests. A successfully
//! built object is not automatically a fully verified design. Whole-graph
//! verification must resolve references and check all applicable type,
//! connectivity, region, behavioral, clock, and dialect invariants.
//!
//! ## Scalability and POCO-REAF
//!
//! This module imposes no universal maximum on designs, modules, operations,
//! ports, operands, results, regions, or hardware resource quantities.
//!
//! Builders must use the canonical typed identities and representations.
//! They must not assume that host collection lengths represent hardware
//! capacity, or that every symbolic width or resource quantity is concrete.
//!
//! Concrete conversions and arithmetic must be checked. Any operational
//! budget imposed by a caller must be enforced by its owning subsystem.
//! Budget exhaustion or incomplete verification must never be reported as
//! successful completion.
//!
//! Finite software remains constrained by available memory, execution time,
//! representable values, and supported semantics. The objective is to avoid
//! arbitrary universal hardware ceilings, not to claim literal infinity.
//!
//! ## Determinism
//!
//! This composition root introduces no mutable global state, implicit ID
//! allocation, environment-dependent registration, or filesystem-order
//! dependency. Construction ordering and canonical ordering are defined by
//! the owning builder and IR contracts.
//!
//! ## Integration contract
//!
//! `design.rs` must delegate to `ir/design.rs`.
//!
//! `module.rs` must delegate to `ir/module.rs` and retain the existing
//! `DesignModule` integration contract.
//!
//! `operation.rs` must delegate to `ir/operations.rs` and the existing
//! schema contract in `schema/operation.rs`. It must not bypass schema
//! validation or claim graph-wide verification.
//!
//! These modules must use the existing HDLIR errors and typed IDs. They must
//! not introduce duplicate identity, type, value, region, or diagnostic
//! systems.
//!
//! The parent `src/hdl/hdlir/mod.rs` must register the `builder` module only
//! after the required HDLIR dependencies are valid Rust modules and compile.
//!
//! ## Public API policy
//!
//! Builder implementations remain accessible through their explicit module
//! paths. The convenience re-exports below expose only the three named
//! builder types; no wildcard re-exports are used.
//!
//! ## Safety
//!
//! Unsafe Rust is forbidden in this module and its descendants. Invalid
//! construction requests must return structured errors through the owning
//! builder APIs rather than silently weakening invariants.
//!
//! ## Freeze criteria
//!
//! - Each declared child module exists at the declared path.
//! - The module builder is stored in `module.rs`, not in this composition
//!   root.
//! - All three builders use their canonical IR and schema owners.
//! - Public exports are deliberate and do not introduce ambiguous names.
//! - The module compiles with Rust 1.97 or later.
//! - Formatting, all-target compilation, tests, and Clippy pass in the
//!   actual repository checkout.
//!
//! This file does not claim that the complete HDLIR subsystem is already
//! production-ready or that repository checks have been executed.

/// Checked construction of canonical HDLIR designs.
pub mod design;

/// Checked construction of canonical HDLIR logical modules.
pub mod module;

/// Checked construction of schema-described HDLIR operations.
pub mod operation;

/// Convenient access to the checked design builder.
pub use design::DesignBuilder;

/// Convenient access to the checked logical module builder.
pub use module::ModuleBuilder;

/// Convenient access to the checked operation builder.
pub use operation::OperationBuilder;
