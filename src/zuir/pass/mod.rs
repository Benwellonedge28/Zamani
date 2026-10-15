#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Pass Subsystem
//!
//! Path: `src/zuir/pass/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! STATUS
//!
//! Proposed canonical module boundary. Freeze after the integration
//! requirements documented below have been completed and verified.
//!
//! PURPOSE
//!
//! Expose the ZUIR pass subsystem as a maintainable collection of modules
//! with explicit ownership boundaries:
//!
//! - `contracts`: canonical pass identity, descriptors, outcomes, and errors.
//! - `determinism`: determinism-policy validation and pipeline assessment.
//! - `invalidation`: deterministic analysis-invalidation policy.
//! - `manager`: ordered pass execution, budgets, cancellation, and
//!   verification orchestration.
//! - `registry`: pass registration, identity lookup, and registration order.
//!
//! AUTHORITY
//!
//! This module is governed by:
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/compilation-model.md`
//! - `grammar/specification/extensibility.md`
//! - `grammar/specification/grammar-authority.md`
//!
//! OWNERSHIP
//!
//! This module owns module composition and the pass subsystem's public
//! export boundary. It does not own pass execution algorithms, analysis
//! invalidation algorithms, descriptor validation, or IR verification.
//!
//! DEPENDENCY DIRECTION
//!
//! The intended dependency direction is:
//!
//! `contracts`
//!     -> `determinism`
//!     -> `invalidation`
//!
//! `contracts`
//!     -> `registry`
//!     -> `manager`
//!
//! `manager` may use `determinism` and `invalidation`.
//! `registry` and `manager` must share the same canonical pass contracts.
//!
//! Foundational contracts must not depend on the manager or registry.
//!
//! INTEGRATION STATUS
//!
//! The current repository has an integration inconsistency: `manager.rs`
//! defines duplicate pass types, while `contracts.rs` owns canonical
//! definitions. `registry.rs` currently imports pass types from `manager`.
//!
//! The aliases below expose that legacy manager API explicitly without
//! misrepresenting it as the canonical contract API. They are a migration
//! bridge, not the intended final architecture.
//!
//! FINAL INTEGRATION REQUIREMENT
//!
//! Migrate `manager.rs` and `registry.rs` to import the canonical types
//! from `super::contracts`. After migration, remove the `Legacy*` aliases
//! and export the canonical manager-facing API directly. Do not maintain
//! two independent definitions of pass identity, descriptors, outcomes,
//! or errors.
//!
//! SCALABILITY
//!
//! This module introduces no fixed limits on the number of passes,
//! analyses, computational domains, or target types. Capacity, time,
//! cancellation, and allocation policies belong to the relevant subsystem.
//!
//! POCO-REAF
//!
//! Passes may operate on classical, quantum, HDL, tensor, distributed,
//! networking, cryptographic, or future IR domains. This module must not
//! hard-code hardware inventory or impose domain-specific execution
//! semantics.
//!
//! SAFETY
//!
//! Unsafe Rust is forbidden.
//!
//! FREEZE CRITERIA
//!
//! - Every declared module exists and compiles.
//! - Canonical contracts have one authoritative definition.
//! - The manager and registry use the canonical contracts.
//! - Determinism and invalidation use the canonical contract types.
//! - Public exports do not create ambiguous type identities.
//! - Unit, integration, and documentation tests pass.
//! - Formatting, linting, and the repository's supported test suite pass.

/// Canonical pass identity, descriptors, outcomes, and errors.
///
/// These are the authoritative contract definitions. New pass consumers
/// should use these types rather than defining equivalent local types.
pub mod contracts;

/// Determinism declarations, policy checks, and pipeline assessment.
///
/// Assessment preserves pipeline order; it never reorders passes.
pub mod determinism;

/// Analysis invalidation policy.
///
/// Computes invalidations without mutating manager-owned analysis state.
pub mod invalidation;

/// Pass registration and lookup.
///
/// The registry must validate descriptors before changing registry state.
pub mod registry;

/// Pass execution, budgets, cancellation, verification, and reporting.
///
/// This module is currently undergoing migration to the canonical contracts.
pub mod manager;

// -----------------------------------------------------------------------------
// Canonical contract exports
// -----------------------------------------------------------------------------

pub use contracts::{
    PassDescriptor,
    PassError,
    PassErrorKind,
    PassId,
    PassOutcome,
    PassVersion,
};

// -----------------------------------------------------------------------------
// Determinism API
// -----------------------------------------------------------------------------

pub use determinism::{
    assess_pipeline_determinism,
    validate_pass_determinism,
    DeterminismAssessment,
    DeterminismPolicy,
};

// -----------------------------------------------------------------------------
// Analysis invalidation API
// -----------------------------------------------------------------------------

pub use invalidation::compute_invalidated_analyses;

// -----------------------------------------------------------------------------
// Registry API
// -----------------------------------------------------------------------------

pub use registry::PassRegistry;

// -----------------------------------------------------------------------------
// Manager API
// -----------------------------------------------------------------------------
//
// The manager currently owns duplicate contract types. Export its operational
// API while naming the duplicate types explicitly as legacy types. This keeps
// the transition visible and avoids silently shadowing canonical exports.

pub use manager::{
    AnalysisState,
    CancellationToken,
    ExecutionBudget,
    IrVerifier,
    Pass as LegacyPass,
    PassContext,
    PassDescriptor as LegacyPassDescriptor,
    PassError as LegacyPassError,
    PassErrorKind as LegacyPassErrorKind,
    PassExecutionRecord,
    PassId as LegacyPassId,
    PassManager,
    PassManagerConfig,
    PassOutcome as LegacyPassOutcome,
    PassVersion as LegacyPassVersion,
    PipelineReport,
    VerificationPolicy,
};