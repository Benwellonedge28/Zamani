
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # ZCIR Builders
//!
//! Canonical path: `src/classical/zcir/builder/mod.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety policy: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the public composition boundary for constructing classical-facing
//! ZUIR modules and operations. Builders in this directory are adapters over
//! canonical ZUIR entities, not an independent intermediate representation.
//!
//! ## Architectural authority
//!
//! This module must remain consistent with:
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/`
//! - `src/zuir/schema/`
//! - `src/classical/zcir/ir/mod.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! The grammar defines source syntax. Semantic analysis establishes the
//! meaning of a source program. ZUIR owns canonical IR representation and
//! identity. These builders construct canonical IR entities after the
//! applicable semantic information has been resolved.
//!
//! ## Ownership
//!
//! This module owns only:
//!
//! - Registration of builder implementation modules.
//! - The stable public re-exports of supported builder APIs.
//!
//! It does not own:
//!
//! - IR storage, identities, values, types, operands, or results.
//! - Source parsing, AST construction, or semantic analysis.
//! - Global identity allocation or cross-program uniqueness.
//! - Whole-IR verification, dominance, use-def, or type inference.
//! - Dialect registration, optimization, lowering, or execution.
//! - Physical resource allocation, scheduling, or target selection.
//!
//! ## Public API
//!
//! `ModuleBuilder` constructs a canonical ZUIR-backed module and validates
//! module-local construction invariants.
//!
//! `ZcirOperationBuilder` constructs a canonical ZUIR operation and validates
//! operation-local invariants.
//!
//! The `BuiltZcirModule` and `BuiltZcirOperation` names are aliases of the
//! canonical representations returned by the builders. They do not define
//! new IR types.
//!
//! The concrete construction contracts, errors, and tests belong to the
//! respective child modules. This file deliberately adds no wrapper state,
//! conversion layer, duplicate error taxonomy, or second validation path.
//!
//! ## Validation boundary
//!
//! Successful builder completion does not establish whole-program validity.
//! Before trusted optimization, serialization, lowering, or execution, the
//! completed IR must pass the authoritative verifier and applicable dialect
//! checks. Those checks must resolve references and establish the relevant
//! identity, type, SSA, control-flow, effect, ownership, resource, and
//! dialect invariants.
//!
//! A valid operation name alone does not establish that an operation is
//! registered or supported by a target. Unsupported or infeasible operations
//! must be reported explicitly; their meaning must not be silently changed.
//!
//! ## Scalability and POCO-REAF
//!
//! This module introduces no fixed maximum for modules, functions, operations,
//! operands, results, regions, or other IR entities.
//!
//! Scalability is bounded by the canonical representation, available memory
//! and processing resources, finite identity spaces, explicit operational
//! budgets, and target capabilities. Literal infinite execution is not
//! promised. No arbitrary hardware-size ceiling is introduced as language
//! semantics.
//!
//! Operational budgets, where applicable, must be configured through their
//! owning APIs rather than added as constants in this composition module.
//!
//! ## Integration contract
//!
//! - `module_builder.rs` must use the canonical module, function, and ID APIs.
//! - `operation_builder.rs` must use canonical ZUIR operation, operand,
//!   result, region, attribute, and effect APIs.
//! - The parent `zcir/mod.rs` must register `builder` before this public path
//!   can be used through `crate::classical::zcir::builder`.
//! - `src/classical/mod.rs` and `src/lib.rs` must expose the parent modules
//!   only after their declarations and dependencies are valid.
//! - Existing classical and quantum IR consumers must continue to use their
//!   designated canonical representation boundaries.
//!
//! Child modules are registered here only because the corresponding builder
//! files exist in the inspected repository. New builder modules must not be
//! declared here before their files and contracts exist.
//!
//! ## Freeze criteria
//!
//! This module is ready to freeze when:
//!
//! - Both child modules compile against the actual canonical ZUIR APIs.
//! - All public re-exports resolve and are unambiguous.
//! - The parent module chain is correctly registered.
//! - Builder and integration tests pass.
//! - Formatting, compilation, testing, and linting pass at the declared
//!   minimum Rust version.
//! - No competing IR types, identities, or arbitrary scalability ceilings
//!   have been introduced.

/// Builder for canonical ZUIR-backed classical modules.
pub mod module_builder;

/// Builder for canonical ZUIR operations used by classical compilation.
pub mod operation_builder;

// ---------------------------------------------------------------------------
// Stable public builder API
// ---------------------------------------------------------------------------

pub use module_builder::{
    BuiltZcirModule,
    ModuleBuilder,
    ModuleBuilderError,
};

pub use operation_builder::{
    BuiltZcirOperation,
    ZcirOperationBuilder,
};
