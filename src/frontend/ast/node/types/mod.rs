//! Zamani Native AST — Type Module
//!
//! This module is the canonical module boundary for source-level type
//! expressions in the Zamani frontend AST.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer
//!     │
//!     ▼
//! parser
//!     │
//!     ▼
//! frontend::ast::node::types
//!     │
//!     ├── TypeExpr
//!     ├── named / primitive / generic / composite façades
//!     ├── bounds
//!     └── type-level source constructs
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic type model
//!     │
//!     ▼
//! ZUIR / domain IR
//!     │
//!     ├── classical
//!     ├── quantum
//!     ├── hybrid
//!     ├── HDL
//!     ├── accelerator
//!     ├── distributed
//!     └── future domains
//! ```
//!
//! # Responsibility
//!
//! This file owns only the module boundary for source-level type AST nodes.
//! The actual representations live in the individual child modules.
//!
//! It deliberately does not own:
//!
//! - name resolution;
//! - type inference;
//! - type checking;
//! - generic substitution;
//! - trait resolution;
//! - ownership analysis;
//! - resource allocation;
//! - quantum routing;
//! - quantum scheduling;
//! - quantum error correction;
//! - calibration;
//! - pulse generation;
//! - hardware topology;
//! - backend selection;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - ZUIR implementation;
//! - runtime execution.
//!
//! Those responsibilities belong to later compiler layers.
//!
//! # POCO-REAF
//!
//! Types are source-level abstractions. This module introduces no fixed:
//!
//! - machine size;
//! - pointer width;
//! - register width;
//! - qubit count;
//! - quantum-register size;
//! - processor count;
//! - topology;
//! - vendor;
//! - backend;
//! - instruction set;
//! - accelerator size.
//!
//! Consequently a type such as a generic resource, array, tuple, function,
//! quantum abstraction, or symbolic cardinality can remain independent of the
//! eventual execution resource.
//!
//! Compiler resource limits, when required for safety, belong to explicit
//! compiler policies and must never be silently encoded in these AST types.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! lexer
//!   ↓
//! parser
//!   ↓
//! frontend::ast::node::types
//!   ↓
//! structural validation
//!   ↓
//! semantic analysis
//!   ↓
//! ZUIR
//!   ↓
//! domain IR
//!   ↓
//! target/backend
//! ```
//!
//! This module must never depend on later phases.
//!
//! # Compatibility
//!
//! The repository currently contains individual type modules including:
//!
//! - `type_expr`;
//! - `primitive`;
//! - `named`;
//! - `generic`;
//! - `array`;
//! - `slice`;
//! - `tuple`;
//! - `function`;
//! - `reference`;
//! - `pointer`;
//! - `optional`;
//! - `result`;
//! - `never`;
//! - `unit`;
//! - `bounds`.
//!
//! These modules are intentionally retained as separate compilation units so
//! each responsibility can evolve independently without turning this module
//! into a monolithic type implementation.
//!
//! `type_expr.rs` remains the canonical source-level representation. The
//! specialised modules are typed façades and focused APIs over that canonical
//! representation; they must not create competing semantic type systems.
//!
//! # Integration contract
//!
//! Parser code may import canonical types from this module:
//
//! ```text
//! use crate::frontend::ast::node::types::TypeExpr;
//! ```
//!
//! Declaration, expression, statement and parameter AST nodes may similarly
//! consume the re-exported type vocabulary.
//!
//! Semantic analysis should consume the canonical `TypeExpr` and resolve it
//! into the semantic type model. It must not require this module to know about
//! hardware or execution targets.
//!
//! ZUIR lowering must operate on resolved semantic types rather than making
//! this source-level module aware of ZUIR implementation details.
//!
//! # Rust compatibility
//!
//! Target toolchain:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Safety
//!
//! This module contains no unsafe code and introduces no unsafe abstractions.
//!
//! The crate-level AST modules may independently enforce their own safety
//! policies, but this boundary explicitly forbids unsafe code here.
//!
//! # Determinism
//!
//! Module declarations and public re-exports have deterministic ordering.
//! Child type modules are responsible for preserving source ordering inside
//! their own structures.
//!
//! # Extension policy
//!
//! New source-level type forms should normally be added by:
//!
//! 1. extending `type_expr.rs` when the construct is part of the canonical
//!    Zamani type vocabulary;
//! 2. adding a dedicated façade module when the construct warrants an isolated
//!    API;
//! 3. registering parser support;
//! 4. adding structural validation;
//! 5. adding semantic resolution;
//! 6. adding ZUIR/domain lowering;
//! 7. adding tests.
//!
//! A new hardware backend, quantum technology, machine size, vendor or target
//! must not require modification of this module.
//!
//! # Important invariant
//!
//! There must be exactly one authoritative source-level type representation.
//! Specialised modules must remain façades/helpers around that representation.
//!
//! In particular:
//!
//! ```text
//! TypeExpr
//!     │
//!     ├── PrimitiveType
//!     ├── NamedType
//!     ├── GenericType
//!     ├── ArrayType
//!     ├── SliceType
//!     ├── TupleType
//!     ├── FunctionType
//!     ├── ReferenceType
//!     ├── PointerType
//!     ├── OptionalType
//!     ├── ResultType
//!     ├── NeverType
//!     └── UnitType
//! ```
//!
//! must not become multiple incompatible AST representations.
//!
//! # No target-specific coupling
//!
//! This module intentionally contains no imports from:
//!
//! ```text
//! crate::quantum
//! crate::hardware
//! crate::backend
//! crate::runtime
//! crate::compiler
//! crate::semantic
//! crate::ir
//! crate::zuiR
//! ```
//!
//! or any vendor-specific quantum/hardware subsystem.
//!
//! Quantum types already represented by the canonical `TypeExpr` remain
//! source-level abstractions. Physical realization is a downstream concern.
//!
//! # File completion contract
//!
//! This module is complete when:
//!
//! - every existing type module has exactly one module declaration;
//! - canonical public types are available through this boundary;
//! - specialised façades remain independently addressable;
//! - no duplicate type representation is introduced;
//! - no circular dependency is introduced;
//! - no downstream compiler dependency is introduced;
//! - no machine-size assumption is introduced;
//! - no fixed qubit/resource limit is introduced;
//! - the module compiles on Rust 1.97/1.97.1;
//! - the module contains no unsafe code;
//! - existing consumers can migrate through stable public re-exports;
//! - new domain/backend implementations do not require changes here.
//!
//! # Testing contract
//!
//! This module requires no runtime tests of its own beyond compile-time/module
//! integration. Behavioural tests belong to the individual type modules and
//! AST integration tests.
//!
//! The complete AST test suite should additionally verify that:
//!
//! - all type modules are reachable;
//! - canonical `TypeExpr` remains reachable;
//! - specialised façades do not introduce duplicate semantic representations;
//! - parser-produced type expressions can flow into declarations and semantic
//!   analysis;
//! - large/symbolic type structures do not encounter hidden machine-size
//!   limits.
//!
//! =============================================================================
//! Module declarations
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod array;
pub mod bounds;
pub mod function;
pub mod generic;
pub mod named;
pub mod never;
pub mod optional;
pub mod pointer;
pub mod primitive;
pub mod reference;
pub mod result;
pub mod slice;
pub mod tuple;
pub mod type_expr;
pub mod unit;

// =============================================================================
// Canonical type representation
// =============================================================================

/// Canonical source-level type expression.
///
/// `TypeExpr` is the authoritative representation consumed by the rest of the
/// frontend AST. Specialised modules provide focused APIs over this type.
pub use type_expr::{
    LifetimeName,
    TypeExpr,
    TypeExprError,
    TypeExtension,
    TypeName,
    TypeParameterName,
    TypePath,
    TypeValidationPolicy,
    TypeValueExtension,
    TypeValueExpr,
};

// =============================================================================
// Specialised type APIs
// =============================================================================
//
// Re-export only stable, explicitly named public façade types. This avoids
// wildcard re-exports and prevents unrelated public symbols from different
// modules from silently colliding.
//
// The child modules remain public, so consumers can always use their complete
// APIs through:
//
//     crate::frontend::ast::node::types::<module>
//
// The canonical `TypeExpr` above remains the single source of truth.

pub use array::ArrayType;
pub use bounds::TypeBound;
pub use function::FunctionType;
pub use generic::GenericType;
pub use named::NamedType;
pub use never::NeverType;
pub use optional::OptionalType;
pub use pointer::PointerType;
pub use primitive::PrimitiveType;
pub use reference::ReferenceType;
pub use result::ResultType;
pub use slice::SliceType;
pub use tuple::TupleType;
pub use unit::UnitType;

// =============================================================================
// Public module-level metadata
// =============================================================================

/// Schema version of the `frontend::ast::node::types` module boundary.
///
/// This version is deliberately independent from:
///
/// - Zamani language version;
/// - compiler version;
/// - `TypeExpr`'s internal schema version;
/// - serialized AST version;
/// - semantic type version;
/// - ZUIR version.
///
/// Changing this value is appropriate only when the public module contract
/// itself changes in a way that affects downstream consumers.
pub const TYPE_AST_MODULE_SCHEMA_VERSION: u16 = 1;

/// Stable architectural name of the canonical source-level type layer.
pub const TYPE_AST_MODULE_NAME: &str = "zamani.frontend.ast.node.types";

/// Returns the schema version of this module boundary.
///
/// This function exists as a small stable API for tooling and compatibility
/// checks without exposing implementation details.
#[must_use]
pub const fn schema_version() -> u16 {
    TYPE_AST_MODULE_SCHEMA_VERSION
}

/// Returns the stable architectural name of this module.
#[must_use]
pub const fn module_name() -> &'static str {
    TYPE_AST_MODULE_NAME
}

// =============================================================================
// Compile-time architectural assertions
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_schema_is_stable_and_nonzero() {
        assert!(schema_version() > 0);
        assert_eq!(module_name(), "zamani.frontend.ast.node.types");
    }

    #[test]
    fn canonical_type_expr_is_reachable_through_module_boundary() {
        let ty = TypeExpr::Unit;
        assert!(matches!(ty, TypeExpr::Unit));
    }

    #[test]
    fn unit_facade_is_reachable_through_module_boundary() {
        let ty = UnitType::new();
        assert!(ty.is_unit());
        assert!(matches!(ty.as_type_expr(), TypeExpr::Unit));
    }

    #[test]
    fn type_path_preserves_source_order() {
        let path = TypePath::from_names(["quantum", "State"]);
        assert_eq!(path.to_source_string(), "quantum::State");
        assert_eq!(path.len(), 2);
    }

    #[test]
    fn type_name_is_target_independent() {
        let name = TypeName::from_str("Quantum");
        assert_eq!(name.as_str(), "Quantum");
    }
}