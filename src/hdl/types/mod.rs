
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Type Subsystem
//!
//! Path: `src/hdl/types/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define the public module boundary for HDL-specific type adapters,
//! logical type layout, and packed-field layout.
//!
//! This module composes the existing HDL type implementations. It does not
//! introduce a new semantic type system or change the meaning of canonical
//! frontend or ZUIR types.
//!
//! ## Authority
//!
//! Source-language and architectural authority:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/spec/type-system.md`
//! - `grammar/spec/user-defined-types.md`
//! - `grammar/hdl/hdl.g4` and its delegate grammars
//! - `grammar/types/` and `grammar/declarations/`
//!
//! Implementation authority:
//!
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/zuir/types/mod.rs`
//! - `src/zuir/types/type_def.rs`
//! - `src/zuir/types/layout.rs`
//! - The registered sibling modules declared below.
//!
//! The grammar owns source syntax. The frontend AST and semantic pipeline
//! own source structure, name resolution, and canonical HDL semantic facts.
//! ZUIR owns canonical semantic type definitions. This module exposes the
//! HDL-specific adapters that consume those existing contracts.
//!
//! ## Owns
//!
//! - Registration of the existing HDL type modules.
//! - A deliberate, documented public API for those modules.
//! - The boundary between HDL type adapters and their consumers.
//!
//! ## Does not own
//!
//! - A replacement AST or semantic type system.
//! - Canonical type identity, name resolution, or generic inference.
//! - A duplicate HDL width representation.
//! - Runtime values or simulation state.
//! - Target ABI, physical memory layout, alignment, or device allocation.
//! - Synthesis, placement, routing, backend implementation, or HDLIR.
//!
//! All HDL intermediate-representation implementation remains under
//! `src/hdl/hdlir/` and is outside this module's implementation scope.
//!
//! ## Module ownership
//!
//! - `aggregate`: validated adapter for canonical aggregate types.
//! - `integer`: fixed-width and symbolic HDL integer type semantics.
//! - `layout`: HDL logical layout facts and symbolic extents.
//! - `logic`: HDL logic-type semantics and state-model classification.
//! - `packed`: packed-field ordering and checked logical bit offsets.
//!
//! Each sibling module remains the sole owner of its implementation details.
//! Re-exports below are convenience paths to those same definitions, not
//! additional type definitions.
//!
//! ## Integration
//!
//! The intended dependency direction is:
//!
//! ```text
//! Grammar
//!   -> domain-neutral frontend AST
//!   -> frontend semantic validation
//!   -> canonical semantic types and HdlWidth
//!   -> src/hdl/types/
//!   -> downstream analysis and realization
//! ```
//!
//! `aggregate` adapts canonical ZUIR aggregate definitions.
//! `integer` and `logic` reuse the canonical HDL semantic width and
//! signedness contracts.
//! `packed` consumes canonical record fields through a caller-provided
//! type-width resolver.
//! `layout` composes logical layout facts without selecting a target ABI.
//!
//! The parent `src/hdl/mod.rs` must register this module using
//! `pub mod types;`. This file must not register itself in the crate root
//! or modify the parent module's ownership of other HDL subsystems.
//!
//! ## Scalability and portability
//!
//! No universal maximum is introduced for field counts, aggregate sizes,
//! or logical HDL widths. Existing symbolic dimensions and width expressions
//! remain symbolic until resolved by their canonical semantic owners.
//!
//! The existing `HdlWidth::Known(u128)` representation limits concrete
//! widths representable by that upstream type. This module does not silently
//! narrow values or claim arbitrary-precision concrete widths. Extending
//! that representation requires a coordinated change to the canonical
//! semantic contract and its consumers.
//!
//! Operational budgets and resource-exhaustion behavior belong to the
//! responsible compilation or analysis boundary. They must not silently
//! change source-language meaning.
//!
//! ## Compatibility
//!
//! The module paths and explicit re-exports below form a public Rust API.
//! Removing, renaming, or changing an exported item requires an explicit
//! compatibility review. New type implementations must first establish
//! their ownership and integration contract before being registered here.
//!
//! ## Freeze criteria
//!
//! - All registered sibling files exist.
//! - Every explicit re-export resolves to its canonical implementation.
//! - The parent module registers `types` exactly once.
//! - The workspace compiles and relevant tests pass.
//! - No duplicate semantic type or HDLIR implementation is introduced.
//! - The public API is reviewed for downstream compatibility.
//! - Rust formatting, linting, and safe-Rust requirements pass.

// -------------------------------------------------------------------------
// Canonical HDL type adapters
// -------------------------------------------------------------------------

/// Validated adapters for canonical aggregate type definitions.
pub mod aggregate;

/// Fixed-width and symbolic HDL integer type semantics.
pub mod integer;

/// Logical HDL type layout and symbolic extent calculations.
pub mod layout;

/// Target-independent digital logic type semantics.
pub mod logic;

/// Portable packed-field ordering and logical bit-offset calculations.
pub mod packed;

// -------------------------------------------------------------------------
// Aggregate type API
//
// The canonical definitions remain owned by `crate::zuir::types`.
// -------------------------------------------------------------------------

pub use aggregate::{
    AggregateKind,
    AggregateTypeDescriptor,
    AggregateTypeError,
};

// -------------------------------------------------------------------------
// Integer type API
//
// Integer types reuse the canonical HDL width and signedness contracts.
// -------------------------------------------------------------------------

pub use integer::{
    IntegerSignedness,
    IntegerType,
    IntegerTypeCompatibility,
    IntegerTypeError,
    IntegerWidthRelation,
};

// -------------------------------------------------------------------------
// Logical layout API
//
// These are logical layout facts, not target ABI or physical layout.
// -------------------------------------------------------------------------

pub use layout::{
    HdlLayoutError,
    HdlLayoutKind,
    HdlTypeLayout,
    LayoutExtent,
    LayoutExtentExpr,
};

// -------------------------------------------------------------------------
// Digital logic type API
//
// Type semantics are separate from the actual logic values in a design.
// -------------------------------------------------------------------------

pub use logic::{
    LogicStateModel,
    LogicType,
    LogicTypeCompatibility,
    LogicTypeError,
};

// -------------------------------------------------------------------------
// Packed layout API
//
// Offsets describe logical bit positions, not serialized byte order.
// -------------------------------------------------------------------------

pub use packed::{
    PackedExtent,
    PackedExtentExpr,
    PackedFieldLayout,
    PackedFieldSpec,
    PackedLayout,
    PackedLayoutError,
    PackedOrder,
};
