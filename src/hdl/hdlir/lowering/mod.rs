
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Lowering
//!
//! Path: `src/hdl/hdlir/lowering/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Lowering subsystem composition boundary.
//!
//! ## Purpose
//!
//! Compose the conversion boundaries that connect the canonical HDL
//! semantic representation, ZUIR, and HDLIR.
//!
//! This module declares the ownership and public module paths for lowering.
//! It does not implement conversions or create another intermediate
//! representation.
//!
//! ## Architectural authority
//!
//! Lowering must conform to:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and the applicable HDL delegate grammars
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar owns syntax. The canonical frontend semantic model owns
//! validated source-level meaning. ZUIR owns universal computational
//! representation. HDLIR owns hardware-domain representation. Target
//! feasibility and physical realization remain downstream.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Registration of the implemented lowering modules.
//! - Their stable paths beneath `crate::hdl::hdlir::lowering`.
//! - The subsystem-level separation between conversion directions.
//!
//! It does not own:
//!
//! - Parsing, AST construction, name resolution, or semantic validation.
//! - HDLIR or ZUIR graph, type, value, identity, or schema definitions.
//! - Operation mapping tables or dialect semantics.
//! - A competing verification framework.
//! - Physical allocation, placement, routing, synthesis, or timing closure.
//!
//! ## Public module contract
//!
//! - `from_semantic` lowers canonical validated HDL semantic input into the
//!   HDLIR subset supported by its implementation.
//! - `from_zuir` lowers explicitly mapped ZUIR operation batches into HDLIR.
//! - `to_zuir` defines the checked HDLIR-to-ZUIR conversion boundary.
//!
//! Each child owns its conversion-specific options, errors, mapping rules,
//! and preservation checks. Shared definitions must be extracted into a
//! sibling module only when an actual shared contract has been designed;
//! they must not be duplicated here.
//!
//! ## Semantic-preservation contract
//!
//! A successful conversion must preserve the meaning of every construct
//! accepted by that conversion's documented contract. Unsupported operations,
//! unmapped types, missing references, unrepresented state, unsupported
//! effects, and ambiguous mappings must produce explicit failures.
//!
//! A conversion must not claim whole-design validity merely because local
//! structure checks passed. Required source and destination verification
//! must be completed at the appropriate owning boundary.
//!
//! Hardware state, signal connectivity, clock/reset relationships, memory
//! behavior, constraints, effects, and provenance must be preserved by
//! explicit mappings or rejected when they cannot be represented losslessly.
//! Encoding such information as descriptive metadata is not a substitute
//! for executable IR semantics.
//!
//! ## Scalability and determinism
//!
//! This composition root introduces no limits on hardware width, signal
//! count, operation count, module count, memory size, clock-domain count,
//! or device capacity.
//!
//! Conversion implementations may accept explicit operational budgets.
//! Budget exhaustion must be reported as failure or an explicitly incomplete
//! outcome; conversion must never truncate a design and report success.
//!
//! Concrete identifiers and collection sizes remain subject to their
//! documented representational limits and available resources. No finite
//! implementation promises literal infinity.
//!
//! Determinism, canonical ordering, identity mapping, and fingerprint
//! behavior are defined by the owning IR and conversion contracts, not by
//! this module declaration.
//!
//! ## Dependency direction
//!
//! ```text
//! canonical HDL semantic model -----> from_semantic -----> HDLIR
//!
//! verified ZUIR + explicit mappings -> from_zuir -------> HDLIR
//!
//! HDLIR + registered mapping rules --> to_zuir ----------> ZUIR
//! ```
//!
//! These are conversion boundaries, not a claim that every HDL feature has
//! a complete bidirectional mapping.
//!
//! ## Integration
//!
//! The parent `src/hdl/hdlir/mod.rs` must declare this module using
//! `pub mod lowering;` after that composition root has been made valid Rust.
//!
//! This file declares only the child modules verified to exist in the
//! repository. It intentionally does not declare a `contracts` module:
//! the attempted fetch of `src/hdl/hdlir/lowering/contracts.rs` returned
//! 404, so that module must not be assumed to exist.
//!
//! This module does not modify `src/hdl/api.rs`, the canonical frontend,
//! ZUIR, or the HDL grammar.
//!
//! ## Completion criteria
//!
//! - Every declared child file exists in the checked-out revision.
//! - The parent HDLIR composition root registers `lowering` exactly once.
//! - Child modules compile against the actual sibling APIs.
//! - Unsupported mappings remain explicit failures.
//! - No lowering implementation is duplicated in this file.
//! - Formatting, compilation, tests, and Clippy pass in the actual checkout.

/// Lower canonical HDL semantic input into the supported HDLIR representation.
///
/// This adapter reuses the frontend's validated HDL semantic model. It must
/// not parse source text, repeat name resolution, or silently omit constructs
/// that the current HDLIR graph cannot represent.
pub mod from_semantic;

/// Lower explicitly mapped ZUIR operation batches into HDLIR.
///
/// The caller is responsible for providing authoritative operation and type
/// mappings and for meeting the source-verification preconditions documented
/// by the adapter. This module does not infer mappings from operation names.
pub mod from_zuir;

/// Provide the checked HDLIR-to-ZUIR conversion boundary.
///
/// A concrete conversion must preserve supported hardware semantics through
/// registered ZUIR dialect contracts and reject unsupported or lossy mappings.
pub mod to_zuir;
