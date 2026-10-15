
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Universal Intermediate Representation (ZUIR)
//! Verification module facade.
//!
//! Canonical path: `src/zuir/verify/mod.rs`
//! Language edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97 or later.
//! Safety policy: unsafe Rust is prohibited.
//!
//! ## PURPOSE
//!
//! Provide the canonical public module boundary for ZUIR verification.
//! Declare and expose the individual verification stages and the
//! full-program verification orchestrator without duplicating their
//! implementations.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ir/`
//! - `src/zuir/types/`
//! - `src/zuir/dialect/`
//! - `src/zuir/verify/structural.rs`
//! - `src/zuir/verify/control_flow.rs`
//! - `src/zuir/verify/dominance.rs`
//! - `src/zuir/verify/types.rs`
//! - `src/zuir/verify/effects.rs`
//! - `src/zuir/verify/ownership.rs`
//! - `src/zuir/verify/resources.rs`
//! - `src/zuir/verify/dialects.rs`
//! - `src/zuir/verify/full.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/grammar.md`
//! - `grammar/specification/semantic-model.md`
//!
//! The ZUIR specification owns IR semantics. The grammar documents
//! define the source-language and semantic boundaries. This module
//! owns neither language semantics nor verification algorithms.
//!
//! ## OWNS
//!
//! - Verification submodule declarations.
//! - The public verification-module facade.
//! - Deliberate re-exports of supported verifier entry points and types.
//! - Documentation of verification-stage boundaries.
//!
//! ## DOES_NOT_OWN
//!
//! - IR data structures, identifiers, or type definitions.
//! - Structural, control-flow, dominance, or type-checking algorithms.
//! - Effect, ownership, resource, or dialect verification algorithms.
//! - Source parsing, AST construction, or semantic name resolution.
//! - SSA verification unless a dedicated SSA verifier is implemented
//!   and explicitly integrated into the full-program verification path.
//! - Target feasibility, hardware allocation, scheduling, routing,
//!   code generation, or runtime execution.
//!
//! ## DEPENDENCY DIRECTION
//!
//! ```text
//! ZUIR specification and IR contracts
//!                 |
//!                 v
//!        Individual verifiers
//!                 |
//!                 v
//!        verify/full.rs
//!                 |
//!                 v
//!         verify/mod.rs API
//!                 |
//!                 v
//!     Compiler, tests, and tooling
//! ```
//!
//! This diagram describes ownership and intended dependency direction.
//! It does not require individual verifiers to depend on this facade.
//!
//! ## VERIFICATION POLICY
//!
//! - Individual verifier modules own their algorithms and diagnostics.
//! - Full verification orchestrates existing stages in a deterministic
//!   order and aggregates their findings.
//! - A successful individual stage does not imply that the complete
//!   program is valid.
//! - A warning is not automatically an error.
//! - An unresolved requirement or unavailable verification capability
//!   must not silently be interpreted as proven valid.
//! - Unsupported or unimplemented checks must be reported honestly.
//! - Target-independent verification must not assume a particular
//!   processor, qubit count, register width, memory capacity, tensor
//!   rank, device count, or distributed topology.
//!
//! ## SCALABILITY
//!
//! This facade imposes no fixed limit on program size, operations,
//! blocks, regions, types, resources, or computational domains.
//! Scalability depends on the individual algorithms, available memory,
//! execution time, address space, and explicitly configured budgets.
//!
//! A finite machine cannot process literally infinite input. The
//! implementation must handle resource exhaustion explicitly rather
//! than silently truncating the program or changing its semantics.
//!
//! ## COMPATIBILITY
//!
//! Public re-exports are part of the Rust API. Renaming or removing
//! them requires an intentional compatibility review. Adding a new
//! verifier must not change the meaning of an existing verifier.
//!
//! ## FREEZE CRITERIA
//!
//! - Every declared module exists in the repository.
//! - Public re-exports match the implementations' actual APIs.
//! - No duplicate verification algorithms are introduced here.
//! - The facade compiles with the repository's actual module tree.
//! - Existing callers can access the supported verification API.
//! - Tests cover module integration and full-verification invocation.
//! - `cargo fmt`, `cargo check`, relevant tests, and Clippy pass.
//! - No unsafe code is introduced.

/// Verify explicit control-flow structure, terminators, and successor
/// relationships.
pub mod control_flow;

/// Verify registered dialect manifests and operation schemas.
pub mod dialects;

/// Analyze control-flow dominance and the properties implemented by
/// the repository's dominance verifier.
///
/// Dominance analysis alone does not establish complete SSA legality.
pub mod dominance;

/// Verify effect declarations and effect contracts.
pub mod effects;

/// Orchestrate all currently integrated verification stages and aggregate
/// their findings into one full-program report.
///
/// This module is the canonical entry point for the existing
/// full-program verification pipeline.
pub mod full;

/// Verify ownership contracts and recorded ownership events.
pub mod ownership;

/// Verify symbolic resource-intent declarations and related constraints.
pub mod resources;

/// Verify structural integrity and containment relationships in ZUIR.
pub mod structural;

/// Verify semantic type definitions and type references.
pub mod types;

// -----------------------------------------------------------------------------
// Supported public verification API
// -----------------------------------------------------------------------------
//
// Re-export the canonical full-verification entry point and its report
// types. Consumers that need an individual stage can import that stage
// directly from its public submodule.

pub use full::{
    verify_full,
    FullVerificationInput,
    FullVerificationReport,
    VerificationIssue,
    VerificationSeverity,
    VerificationStage,
    VerificationStatus,
};

// Re-export the existing stage inputs and entry points so compiler
// integrations and external callers can invoke focused verification
// without depending on internal implementation details.

pub use control_flow::{
    verify_control_flow,
    ControlFlowInput,
};

pub use dialects::{
    verify_dialects,
    DialectIssueSeverity,
};

pub use dominance::{
    verify_dominance,
    DominanceInput,
};

pub use effects::{
    verify_effects,
    EffectVerificationInput,
};

pub use ownership::{
    verify_ownership,
    OwnershipVerificationInput,
};

pub use resources::{
    verify_resources,
    ResourceIntentEntry,
    ResourceIssueSeverity,
};

pub use structural::{
    verify_structural,
    StructuralInput,
};

pub use types::{
    verify_types,
    TypeVerificationInput,
};
