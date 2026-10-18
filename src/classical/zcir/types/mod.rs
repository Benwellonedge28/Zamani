
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR type adapters
//!
//! Path: `src/classical/zcir/types/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Defines the public module boundary for ZCIR's classical type adapters.
//!
//! ZCIR is a classical-computing dialect and integration layer. It reuses
//! canonical ZUIR semantic types instead of defining a competing universal
//! type system.
//!
//! ## Canonical ownership
//!
//! The following remain canonical and are not redefined here:
//!
//! - `crate::zuir::types::TypeKind` — semantic type variants;
//! - `crate::zuir::types::TypeRef` — named and inline type references;
//! - `crate::zuir::types::TypeDef` — named type definitions and validation;
//! - `crate::zuir::types::IntegerWidth` and `IntegerSignedness`;
//! - `crate::zuir::types::layout` — logical type layout;
//! - `crate::zuir::schema::types::QualifiedName` — qualified type names.
//!
//! The descriptors exported here adapt those canonical representations for
//! classical-specific validation and integration. They do not create new
//! type identities or imply that a type is supported by every target.
//!
//! ## Responsibilities
//!
//! - `scalar`: scalar-type classification and validation;
//! - `integer`: integer signedness and width validation;
//! - `float`: floating-point format-name adaptation;
//! - `aggregate`: tuple, array, record, sum, and union adaptation;
//! - `function`: function signature adaptation and validation;
//! - `reference`: named/inline type references and semantic reference types;
//! - `layout`: ZCIR access to canonical ZUIR logical layouts.
//!
//! ## Explicit non-responsibilities
//!
//! This module does not own source parsing, AST construction, semantic name
//! resolution, type inference, canonical type registries, value storage,
//! numeric evaluation, ownership checking, physical layout, ABI, allocation,
//! target selection, scheduling, or backend lowering.
//!
//! ## POCO-REAF and scalability
//!
//! This module introduces no fixed ceiling on the number of types, aggregate
//! members, function parameters, type formats, dimensions, integer widths,
//! or supported targets.
//!
//! Any operational budgets belong to the relevant validator, compiler
//! configuration, decoder, or resource-management boundary. Such budgets
//! must not silently become language-level semantic restrictions.
//!
//! Portability means preserving the program's defined meaning across valid
//! implementations. It does not imply that every target has enough memory,
//! execution time, or capabilities to realize every program.
//!
//! ## Integration contract
//!
//! - The parent `src/classical/zcir/mod.rs` registers this module as
//!   `pub mod types;` when its module structure is finalized.
//! - Frontend semantic analysis supplies resolved canonical types.
//! - Verification checks named references, semantic invariants, effects,
//!   and cross-definition relationships at the appropriate boundary.
//! - The canonical ZUIR registry resolves named types; these adapters do not.
//! - Logical layout remains owned by `crate::zuir::types::layout`.
//! - Lowering separately determines whether and how a target can implement
//!   a verified type without silently changing its semantics.
//! - Values and arithmetic remain owned by the corresponding value and
//!   execution subsystems.
//!
//! ## File freeze criteria
//!
//! Before freezing this module:
//!
//! - rename the existing `scalar.rs ` file to `scalar.rs`;
//! - ensure every declared child module exists at its exact path;
//! - ensure every public re-export below matches the child module API;
//! - resolve the canonical semantic-reference layout distinction in ZUIR;
//! - run formatting, checking, tests, and linting on Rust 1.97 or later;
//! - test the module on supported operating systems, including Windows.

pub mod aggregate;
pub mod float;
pub mod function;
pub mod integer;
pub mod layout;
pub mod reference;
pub mod scalar;

// -------------------------------------------------------------------------
// Aggregate types
// -------------------------------------------------------------------------

pub use aggregate::{
    AggregateKind,
    AggregateTypeDescriptor,
    AggregateTypeError,
};

// -------------------------------------------------------------------------
// Floating-point types
// -------------------------------------------------------------------------

pub use float::{
    FloatTypeDescriptor,
    FloatTypeError,
};

// -------------------------------------------------------------------------
// Function types
// -------------------------------------------------------------------------

pub use function::{
    validate_function_type,
    FunctionTypeDescriptor,
    FunctionTypeError,
};

// -------------------------------------------------------------------------
// Integer types
// -------------------------------------------------------------------------

pub use integer::{
    validate_integer_type,
    validate_integer_width,
    IntegerTypeDescriptor,
    IntegerTypeError,
};

// -------------------------------------------------------------------------
// Logical layout
//
// These are aliases/adapters over canonical ZUIR layout types.
// They do not define physical size, alignment, ABI, or allocation.
// -------------------------------------------------------------------------

pub use layout::{
    ClassicalLayoutError,
    ClassicalLayoutPolicy,
    ClassicalLayoutShape,
    ClassicalTypeLayout,
};

// -------------------------------------------------------------------------
// References
//
// TypeReferenceDescriptor adapts TypeRef.
// ReferenceTypeDescriptor adapts the semantic TypeKind::Reference variant.
// These concepts must not be conflated.
// -------------------------------------------------------------------------

pub use reference::{
    validate_semantic_reference_type,
    validate_type_reference,
    ReferenceTypeDescriptor,
    ReferenceTypeError,
    TypeReferenceDescriptor,
};

// -------------------------------------------------------------------------
// Scalar types
// -------------------------------------------------------------------------

pub use scalar::{
    validate_scalar_kind,
    ScalarTypeDescriptor,
    ScalarTypeError,
};
