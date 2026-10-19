
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Value Subsystem
//!
//! Path: `src/hdl/values/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define the public composition root for HDL value representations.
//!
//! This module registers the existing implementations for:
//!
//! - Exact constants, including arbitrary-length integer magnitudes.
//! - Four-state digital logic values and dynamically sized vectors.
//! - Symbolic values and parameter-dependent dependency resolution.
//!
//! It provides convenient, documented public exports without creating a
//! second implementation of any value representation.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/spec/type-system.md`
//! - `grammar/hdl/hdl.g4` and its delegate grammars.
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/types/logic.rs`
//! - `src/hdl/types/integer.rs`
//!
//! The grammar owns source syntax. The frontend AST and semantic pipeline
//! own parsed source structure, name resolution, and canonical semantic
//! identities. The HDL type modules own type-level interpretation.
//!
//! This module composes the value implementations; it does not replace
//! those existing authorities.
//!
//! ## Owns
//!
//! - Registration of the HDL value modules.
//! - The deliberate public API for constants, four-state values, and
//!   symbolic-value infrastructure.
//! - The module-level boundary between concrete and symbolic values.
//!
//! ## Does not own
//!
//! - Source parsing, AST construction, or general name resolution.
//! - A replacement HDL type system or symbolic-width representation.
//! - General expression parsing, type inference, or expression evaluation.
//! - Simulation scheduling, event queues, or sequential execution.
//! - Synthesis interpretation, physical design, or target allocation.
//! - Hardware discovery, vendor APIs, or backend handles.
//! - HDLIR structures, builders, verification, passes, serialization,
//!   or lowering. All HDLIR implementation remains under
//!   `src/hdl/hdlir/`.
//!
//! ## Value ownership
//!
//! `constant` represents values already known to the caller.
//! `four_state` represents digital logic values that may include unknown
//! or high-impedance states.
//! `symbolic` represents values whose definitions depend on other symbols
//! and whose concrete results require an explicit evaluator.
//!
//! These categories are related but are not interchangeable.
//!
//! In particular:
//!
//! - A symbolic value must not be treated as a concrete constant before
//!   successful resolution.
//! - An unknown or high-impedance logic state must not be silently
//!   converted to a known Boolean or integer.
//! - A stored constant's width must not be mistaken for a symbolic
//!   semantic width or a physical target capacity.
//! - Value representation does not establish that a target can realize
//!   a design containing that value.
//!
//! `constant::LogicBit` and `four_state::LogicState` are distinct types
//! with different ownership roles. Conversions between them, if required
//! by consumers, must be explicit and preserve all four logic states.
//!
//! ## Scalability
//!
//! This module introduces no universal maximum for value width, number of
//! symbolic definitions, or dependency count.
//!
//! The concrete value modules use dynamically sized standard-library
//! collections. Symbolic dependency resolution uses the implementation
//! contract documented by `symbolic` and reports unresolved references,
//! dependency cycles, and evaluator failures explicitly.
//!
//! Actual processing is bounded by available memory, execution time,
//! allocator behavior, and caller-defined operational budgets. No module
//! can guarantee literally infinite storage or computation.
//!
//! Operational budgets must not silently change language semantics.
//! A budget-exhausted operation must not be reported as successfully
//! resolved or fully evaluated.
//!
//! ## Integration
//!
//! The intended dependency direction is:
//!
//! ```text
//! Canonical grammar
//!       |
//!       v
//! Frontend AST and semantic validation
//!       |
//!       v
//! Canonical HDL identities and type contracts
//!       |
//!       +----------------------+---------------------+
//!       |                      |                     |
//!       v                      v                     v
//! values::constant     values::four_state    values::symbolic
//!       |                      |                     |
//!       +----------------------+---------------------+
//!                              |
//!                              v
//!                 HDL expression and behavior consumers
//!                              |
//!                              v
//!                   downstream implementation layers
//! ```
//!
//! The frontend adapter is responsible for constructing these values from
//! validated source constructs. Expression evaluation owns operator-specific
//! arithmetic and conversion rules. Type checking owns compatibility with
//! `LogicType`, `IntegerType`, and canonical semantic types.
//!
//! The parent `src/hdl/mod.rs` must register this module with `pub mod
//! values;`. It must also register other implemented sibling modules, such
//! as `types`, when their integration is ready. This file deliberately
//! does not edit or replace the parent composition root.
//!
//! No sibling value module may depend on this file to obtain its own
//! implementation. Re-exports below are public access paths to existing
//! definitions, not duplicate definitions.
//!
//! ## Freeze criteria
//!
//! - Every registered module exists and has one implementation owner.
//! - Public exports refer to the canonical sibling definitions.
//! - Concrete, symbolic, and four-state values remain distinguishable.
//! - No source parser, semantic analyzer, or IR implementation is duplicated.
//! - No universal hardware-size ceiling is introduced.
//! - Public API and module registration pass repository-wide checks.

/// Exact known HDL constants and arbitrary-length integer values.
///
/// This module owns constant storage and local constant operations. It does
/// not parse Zamani source syntax or resolve symbolic expressions.
pub mod constant;

/// Four-state digital logic values and dynamically sized logic vectors.
///
/// This module owns the `0`, `1`, `X`, and `Z` value operations it defines.
/// Type-level state-model legality remains owned by `crate::hdl::types::logic`.
pub mod four_state;

/// Symbolic values, concrete bindings, and dependency resolution.
///
/// This module preserves symbolic identity and delegates expression
/// evaluation to a caller-supplied evaluator. It does not replace the
/// canonical AST or semantic expression representation.
pub mod symbolic;

// ---------------------------------------------------------------------------
// Constant value API
//
// These are the existing definitions in constant.rs. Re-exports do not
// create a second constant representation.
// ---------------------------------------------------------------------------

pub use constant::{
    ConstantError,
    ConstantFit,
    HdlConstant,
    IntegerConstant,
    IntegerMagnitude,
    LogicBit,
    LogicVectorConstant,
};

// ---------------------------------------------------------------------------
// Four-state value API
//
// LogicState describes an actual value state, not a type declaration.
// LogicType and LogicStateModel remain owned by crate::hdl::types::logic.
// ---------------------------------------------------------------------------

pub use four_state::{
    FourStateError,
    FourStateVector,
    LogicState,
};

// ---------------------------------------------------------------------------
// Symbolic value API
//
// HdlId remains owned by the canonical frontend semantic HDL contract.
// The generic environment does not introduce another identity type.
// ---------------------------------------------------------------------------

pub use symbolic::{
    SymbolicEnvironment,
    SymbolicEnvironmentError,
    SymbolicResolutionError,
    SymbolicValue,
    HDL_SYMBOLIC_SCHEMA_VERSION,
};
