#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR semantic type subsystem
//!
//! Path: `src/zuir/types/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide the canonical public module boundary for ZUIR semantic types.
//! This module declares the type subsystem's authoritative implementations
//! and re-exports their supported public APIs.
//!
//! ## STATUS
//!
//! Integration implementation. Freeze only after the repository's complete
//! build, tests, public API review, and downstream integration checks pass.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md` defines ZUIR representation requirements.
//! - `grammar/DESIGN.md` defines source-language and semantic boundaries.
//! - `schema/types.rs` owns schema-level qualified names and schema types.
//! - The sibling modules declared below own their respective contracts.
//!
//! ## OWNS
//!
//! - Declaration and visibility of the canonical type subsystem modules.
//! - Deliberate public re-exports for the type subsystem.
//! - The module-level API boundary between type implementations and callers.
//!
//! ## DOES_NOT_OWN
//!
//! - Semantic type definitions or type-definition validation.
//! - Canonical identifier allocation or type registration.
//! - Constraint solving or generic inference.
//! - Logical or physical layout computation.
//! - Type equivalence, assignment compatibility, or conversion policy.
//! - Source parsing, AST construction, or name resolution.
//! - IR verification, dialect registration, or target lowering.
//!
//! ## MODULE OWNERSHIP
//!
//! - `type_def`: semantic type definitions and local validation.
//! - `type_id`: type-specific identity utilities and lookup keys.
//! - `constraints`: symbolic constraints and their evaluation.
//! - `layout`: target-independent logical type layout.
//! - `compatibility`: semantic equivalence, assignability, and
//!   explicit conversion-policy queries.
//!
//! ## INTEGRATION
//!
//! Source grammar and AST
//!     -> frontend semantic analysis
//!     -> validated semantic types
//!     -> ZUIR type subsystem
//!     -> IR verification and domain-specific lowering
//!
//! The grammar and frontend are upstream consumers and producers of
//! semantic information. This module must not depend on grammar-generated
//! parser types, frontend AST types, quantum IR, or target backends.
//!
//! The parent `crate::zuir` module must declare this module using its
//! existing module hierarchy. This file must not declare another ZUIR root.
//!
//! ## SCALABILITY AND PORTABILITY
//!
//! This module imposes no hardware-size limits. Its submodules must preserve
//! symbolic widths, dimensions, generic constraints, and domain-defined
//! semantic types without assuming fixed register widths, tensor ranks,
//! processor counts, qubit counts, memory sizes, or device counts.
//!
//! Practical resource exhaustion and finite identifier representations
//! must be reported by their owning implementations rather than hidden
//! behind compatibility success or silent truncation.
//!
//! ## COMPATIBILITY
//!
//! Re-exports are part of the Rust public API. Removing or renaming an
//! exported item requires an explicit compatibility review. The module
//! does not independently define serialized-schema compatibility.
//!
//! ## FREEZE CRITERIA
//!
//! - Every declared module exists exactly once.
//! - Re-exports resolve to the canonical owning implementations.
//! - No competing semantic type or identifier definitions are introduced.
//! - The crate compiles and all applicable tests pass.
//! - Public API and downstream import paths are reviewed.
//! - No unsafe code or hardware-specific assumptions are introduced.

// -------------------------------------------------------------------------
// Canonical implementations
// -------------------------------------------------------------------------

pub mod compatibility;
pub mod constraints;
pub mod layout;
pub mod type_def;
pub mod type_id;

// -------------------------------------------------------------------------
// Semantic type definitions
//
// These types are defined exclusively by type_def.rs.
// -------------------------------------------------------------------------

pub use type_def::{
    ConstantArgument,
    Dimension,
    FieldDef,
    FunctionParameter,
    IntegerSignedness,
    IntegerWidth,
    ParameterMode,
    SymbolicExpression,
    TypeArgument,
    TypeAttribute,
    TypeConstraint,
    TypeDef,
    TypeDefError,
    TypeKind,
    TypeRef,
    TypeVisibility,
    VariantDef,
};

// -------------------------------------------------------------------------
// Type identity
//
// TypeId remains owned by crate::zuir::ids.
// type_id.rs provides the type-specific identity façade.
// -------------------------------------------------------------------------

pub use type_id::{
    TypeId,
    TypeIdAllocator,
    TypeIdentityError,
    TypeKey,
};

// -------------------------------------------------------------------------
// Symbolic constraints
//
// Constraint evaluation does not imply physical resource allocation.
// -------------------------------------------------------------------------

pub use constraints::{
    Constraint,
    ConstraintEnvironment,
    ConstraintError,
    ConstraintEvaluator,
    ConstraintExpression,
    ConstraintOutcome,
    ConstraintSet,
    ConstraintSetOutcome,
    ConstraintVariable,
    DimensionConstraint,
    LiteralConstraintEvaluator,
    Natural,
    Relation,
};

// -------------------------------------------------------------------------
// Logical layout
//
// These APIs describe logical structure, not target ABI or physical layout.
// -------------------------------------------------------------------------

pub use layout::{
    LayoutError,
    LayoutField,
    LayoutIntegerWidth,
    LayoutParameter,
    LayoutPolicy,
    LayoutShape,
    LayoutTypeArgument,
    LayoutVariant,
    LogicalLayout,
};

// -------------------------------------------------------------------------
// Semantic compatibility
//
// Compatibility and conversion are separate from physical layout.
// Unknown must never be treated as compatible.
// -------------------------------------------------------------------------

pub use compatibility::{
    Compatibility,
    CompatibilityBudget,
    CompatibilityChecker,
    CompatibilityReason,
    ConversionDecision,
    ConversionPolicy,
    IdentityOnlyConversionPolicy,
    TypeResolver,
};