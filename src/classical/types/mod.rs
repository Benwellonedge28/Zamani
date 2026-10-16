#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical type subsystem.
//!
//! Path: `src/classical/types/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Status
//!
//! Module registry implementation. Repository integration and compilation
//! must pass before this module is marked verified.
//!
//! # Purpose
//!
//! Provide a stable namespace for classical-facing type descriptors,
//! generic integration, constraint integration, and layout descriptions.
//! This file declares module boundaries; it does not implement type
//! semantics or duplicate the canonical frontend type system.
//!
//! # Architectural authority
//!
//! Normative specifications:
//! - `grammar/DESIGN.md`
//! - `grammar/specification/types.md`
//! - `grammar/spec/type-system.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/types/generic.g4`
//! - `grammar/types/bounds.g4`
//!
//! Canonical source-level type expressions:
//! - `crate::frontend::ast::node::types::type_expr::TypeExpr`
//!
//! Canonical resolved semantic types:
//! - `crate::frontend::semantic::types::ty::SemanticType`
//!
//! Canonical generic and constraint implementations:
//! - `crate::frontend::semantic::types::generics`
//! - `crate::frontend::semantic::types::constraints`
//! - `crate::frontend::semantic::types::unification`
//!
//! Canonical classical IR:
//! - `crate::quantum::ir::classical`
//!
//! # Ownership
//!
//! This module owns:
//! - Registration and visibility of the classical type submodules.
//! - The stable `crate::classical::types::*` namespace.
//!
//! This module does not own:
//! - Source syntax, parsing, AST nodes, or name resolution.
//! - Canonical semantic types, type inference, or type unification.
//! - Generic substitution or constraint-solving algorithms.
//! - Numeric value representations or arithmetic implementations.
//! - Collection storage, ownership, or memory allocation.
//! - Physical layout policy, target ABI rules, or hardware capabilities.
//! - Classical IR definitions, lowering, optimization, or execution.
//!
//! # Registered modules
//!
//! `scalar`
//!     Classical-facing scalar type descriptors.
//!
//! `aggregate`
//!     Classical-facing aggregate type descriptors.
//!
//! `generic`
//!     Adapter to the canonical semantic generic type implementation.
//!
//! `constraints`
//!     Adapter to the canonical semantic constraint implementation.
//!
//! `layout`
//!     Target-independent layout quantities and checked layout composition.
//!
//! Each module is the exclusive owner of its documented classical-facing
//! API. Canonical semantic implementations remain authoritative for their
//! respective semantics.
//!
//! # Dependency direction
//!
//! ```text
//! Source syntax / canonical AST
//!            |
//!            v
//! Canonical semantic type system
//!            |
//!            v
//! Classical type adapters
//!   |        |         |
//! scalar  aggregate  generic/constraints
//!            |
//!            v
//!     Layout descriptions
//!            |
//!            v
//! Target-aware lowering and canonical IR integration
//! ```
//!
//! The diagram describes responsibility boundaries, not a requirement that
//! every type operation call every module. In particular, `layout` must not
//! become a substitute for semantic type validation, and the classical
//! adapters must not create a second semantic type system.
//!
//! # Public API policy
//!
//! Submodules are exposed through explicit module paths, for example:
//!
//! - `crate::classical::types::scalar::ScalarTypeDescriptor`
//! - `crate::classical::types::aggregate::AggregateTypeDescriptor`
//! - `crate::classical::types::layout::TypeLayout`
//!
//! This registry intentionally does not flatten all submodule symbols into
//! this namespace. Explicit paths reduce name collisions, preserve ownership
//! visibility, and allow modules to evolve without silently changing the
//! meaning of unrelated exports.
//!
//! It also deliberately avoids wildcard re-exports. A new public symbol
//! becomes part of the intended API only through its owning module's
//! documented contract and an explicit compatibility review.
//!
//! # POCO-REAF and scalability
//!
//! This registry introduces no maximum for type count, scalar width,
//! aggregate fields, generic arguments, constraints, layout size, target
//! count, or supported computational domains.
//!
//! Concrete representation limits, explicit operational budgets, available
//! memory, and target capabilities must remain separate from language-level
//! type meaning. Symbolic dimensions and unresolved layouts must not be
//! silently narrowed or treated as proven concrete representations.
//!
//! POCO-REAF does not mean physically infinite resources or that every
//! target can realize every type. It means a program's semantics must not
//! depend on arbitrary universal hardware-capacity ceilings.
//!
//! # Integration contract
//!
//! 1. `crate::classical::mod` must expose this module with `pub mod types;`.
//! 2. Existing consumers may use the explicit `crate::classical::types::*`
//!    module paths documented above.
//! 3. `scalar` and `aggregate` remain descriptors, not replacements for
//!    canonical semantic types.
//! 4. `generic` and `constraints` delegate semantic operations to the
//!    frontend's canonical implementations.
//! 5. `layout` describes representation metadata; target-aware lowering
//!    must verify concrete layout requirements before using concrete offsets.
//! 6. The existing canonical classical IR under
//!    `crate::quantum::ir::classical` remains the IR authority.
//! 7. Future modules may be registered only after their contracts are
//!    defined and their files exist. Do not register placeholder modules
//!    merely to anticipate future features.
//!
//! # Compatibility and freeze rule
//!
//! Removing or renaming a public module is a public API change. Adding a
//! module is normally additive, but its ownership and dependency direction
//! must be reviewed. Flattening exports or moving canonical semantic types
//! into this namespace requires an explicit compatibility and architecture
//! review.
//!
//! This file is complete when the registered modules compile together,
//! the parent module exposes `types`, the API paths are tested, and the
//! repository's supported checks pass.

/// Classical-facing scalar type descriptors.
pub mod scalar;

/// Classical-facing aggregate type descriptors.
pub mod aggregate;

/// Adapter to the canonical semantic generic type system.
pub mod generic;

/// Adapter to the canonical semantic constraint system.
pub mod constraints;

/// Target-independent type-layout descriptions and composition.
pub mod layout;