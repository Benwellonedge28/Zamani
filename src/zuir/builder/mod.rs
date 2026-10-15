#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Universal Intermediate Representation — Builders
//!
//! Canonical path: `src/zuir/builder/mod.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: Rust 1.97.1 or later.
//!
//! STATUS
//!
//! Proposed canonical builder-module boundary. Create this file because it
//! was not present on the inspected repository default branch. Freeze it
//! only after repository-wide compilation and integration tests succeed.
//!
//! PURPOSE
//!
//! Expose the canonical, safe, extensible construction APIs for ZUIR
//! programs, operations, regions, and builder-level validation.
//!
//! This module composes the implementations in the builder submodules.
//! It does not define a second IR, duplicate builder logic, or replace the
//! authoritative types owned by `crate::zuir::ir`, `crate::zuir::dialect`,
//! `crate::zuir::schema`, or the whole-IR verifier.
//!
//! ARCHITECTURAL AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/version.rs`
//! - `src/zuir/config.rs`
//! - `src/zuir/errors.rs`
//! - `src/zuir/ir/`
//! - `src/zuir/dialect/`
//! - `src/zuir/schema/`
//! - `src/zuir/verify/`, where implemented
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/compatibility.md`
//!
//! PURPOSES OWNED HERE
//!
//! - Declare the canonical builder submodules.
//! - Provide deliberate, documented re-exports of builder-owned APIs.
//! - Preserve the existing submodule paths for callers that need them.
//! - Establish the dependency and integration contract for builder APIs.
//!
//! RESPONSIBILITIES NOT OWNED HERE
//!
//! - Defining or storing canonical IR entities.
//! - Allocating globally unique identifiers across independent compilation
//!   sessions.
//! - Defining operation semantics, types, effects, ownership, or resources.
//! - Registering dialects or deciding whether an extension is trusted.
//! - Parsing source code or implementing grammar productions.
//! - Reimplementing semantic analysis or name resolution.
//! - Replacing authoritative whole-IR verification.
//! - Choosing physical devices, allocating hardware resources, routing,
//!   scheduling, or executing compiled programs.
//! - Defining serialization formats, migrations, or fingerprints.
//!
//! MODULE OWNERSHIP
//!
//! `operation` owns operation construction from registered operation
//! definitions and canonical IR types.
//!
//! `program` owns the program-construction workflow and delegates container
//! invariants to the canonical program representation.
//!
//! `regions` owns region construction and local region-membership checks.
//!
//! `validation` owns the reusable builder-validation framework. It does not
//! own whole-IR semantic verification.
//!
//! DEPENDENCY DIRECTION
//!
//! The intended dependency direction is:
//!
//!   canonical IDs, schemas, dialect definitions, and IR types
//!                           |
//!                           v
//!                    builder submodules
//!                           |
//!                           v
//!                 whole-IR verification
//!                           |
//!                           v
//!               transformations and lowering
//!
//! This module is an API composition boundary, not an implementation layer.
//! Foundational IR and schema modules must not depend on this module.
//!
//! PUBLIC API POLICY
//!
//! - Canonical definitions remain in their owning submodules.
//! - Re-exports are convenience paths, not duplicate type definitions.
//! - Existing imports through `builder::operation`, `builder::program`,
//!   `builder::regions`, and `builder::validation` remain available.
//! - New public re-exports require a deliberate API and compatibility review.
//! - No placeholder API may claim to implement verification, lowering,
//!   or construction behavior that its owning module does not provide.
//!
//! VALIDITY CONTRACT
//!
//! Successful construction establishes only the invariants guaranteed by
//! the particular builder operation. It does not, by itself, prove that a
//! complete program is semantically valid.
//!
//! Before optimization, lowering, serialization as a trusted artifact,
//! or execution, the enclosing compilation pipeline must perform all
//! applicable canonical verification, dialect checks, version checks,
//! and target-feasibility checks.
//!
//! SCALABILITY AND POCO-REAF
//!
//! This module imposes no fixed limit on the number of programs, modules,
//! operations, operands, results, regions, blocks, captures, qubits,
//! processors, devices, nodes, tensor dimensions, or graph elements.
//!
//! Capacity management belongs to the owning builders and canonical data
//! structures. Where supported, they must expose fallible reservation or
//! return structured errors for detectable allocation failures.
//!
//! This module introduces no fixed hardware capacities, target-specific
//! assumptions, or closed list of all computational operations.
//!
//! Literal infinity cannot be guaranteed: memory, address space, execution
//! time, identifier spaces, and physical resources are finite. The contract
//! is to avoid arbitrary universal ceilings and report detectable failures
//! without silently changing program semantics.
//!
//! GRAMMAR BOUNDARY
//!
//! The Zamani grammar defines source syntax; semantic analysis establishes
//! meaning. Builders consume resolved construction inputs and canonical
//! schema/IR contracts. They must not infer source-language semantics from
//! grammar productions or turn grammar keywords into a closed ZUIR
//! operation list.
//!
//! ERROR POLICY
//!
//! Each builder retains its own structured error and result types.
//! This module does not collapse errors into strings or invent a second
//! repository-wide error hierarchy. Higher-level integration boundaries
//! may map builder errors into the canonical ZUIR error model.
//!
//! FREEZE CRITERIA
//!
//! - Every declared child module exists exactly once.
//! - Every re-export resolves to a public item in its owning module.
//! - Existing submodule import paths remain usable.
//! - No canonical IR or validation logic is duplicated here.
//! - All real builder call sites compile against the published API.
//! - Whole-IR verification remains a distinct required trust boundary.
//! - Formatting, compilation, tests, and linting pass on the supported
//!   Rust toolchain.
//! - No unsafe code or fixed hardware-size assumptions are introduced.

// =============================================================================
// Canonical builder modules
// =============================================================================

/// Constructs canonical ZUIR operations from registered definitions.
pub mod operation;

/// Constructs canonical ZUIR programs from their canonical components.
pub mod program;

/// Constructs regions and manages their local block membership.
pub mod regions;

/// Provides reusable builder-level validation rules and reports.
pub mod validation;

// =============================================================================
// Deliberate public re-exports
// =============================================================================

// Operation construction.
//
// These types are defined by `operation.rs`; this module does not introduce
// another operation representation.
pub use operation::{
    OperationBuildError,
    OperationBuildResult,
    OperationBuilder,
    RegionBinding,
};

// Program construction.
//
// `ProgramBuilder<M>` uses the canonical `ZuirProgram<M>` container and
// leaves whole-IR verification to the authoritative verification boundary.
pub use program::{
    ProgramBuilder,
    ProgramBuilderError,
    ProgramBuilderResult,
};

// Region construction.
//
// The region builder references canonical blocks and regions; it does not
// replace their storage or global verification.
pub use regions::{
    RegionBuildError,
    RegionBuildResult,
    RegionBuilder,
};

// Builder-level validation.
//
// These exports describe local validation infrastructure, not proof that
// a complete ZUIR program is valid.
pub use validation::{
    ValidationContext,
    ValidationFrameworkError,
    ValidationIssue,
    ValidationMode,
    ValidationPhase,
    ValidationPipeline,
    ValidationReport,
    ValidationResult,
    ValidationRule,
    ValidationSeverity,
};