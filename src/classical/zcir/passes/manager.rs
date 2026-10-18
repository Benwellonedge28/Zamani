
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR Pass Manager Integration
//!
//! Path: `src/classical/zcir/passes/manager.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97 or later.
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Status
//!
//! Compatibility façade over the repository's existing ZUIR pass manager.
//! This module intentionally does not implement a second pass engine.
//!
//! ## Purpose
//!
//! Make the shared pass-management facilities accessible from the ZCIR
//! namespace while preserving the single canonical ZUIR representation and
//! making the current pass-contract migration explicit.
//!
//! ## Architectural authority
//!
//! This module follows:
//!
//! - `src/zuir/pass/mod.rs`
//! - `src/zuir/pass/contracts.rs`
//! - `src/zuir/pass/manager.rs`
//! - `src/zuir/pass/registry.rs`
//! - `src/zuir/pass/invalidation.rs`
//! - `src/zuir/pass/determinism.rs`
//! - `src/zuir/verify/mod.rs`
//! - `src/zuir/verify/full.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! The grammar defines source syntax. The semantic model defines program
//! meaning. ZUIR owns the shared IR and pass-contract infrastructure.
//! ZCIR exposes the classical-facing integration boundary.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The ZCIR-facing pass-manager import boundary.
//! - Explicit names for canonical and legacy pass-contract types.
//! - Documentation of the migration requirements.
//!
//! This module does not own:
//!
//! - A second IR, pass registry, pass identity system, or pass engine.
//! - Pass implementation or semantic-preservation proofs.
//! - Verification algorithms or verification completeness claims.
//! - Analysis algorithms, analysis result storage, or invalidation policy.
//! - Optimization rewrite rules.
//! - Target selection, physical resource allocation, scheduling, routing,
//!   hardware discovery, or runtime execution.
//! - Source parsing, AST construction, or semantic analysis.
//!
//! ## Canonical versus legacy contracts
//!
//! `src/zuir/pass/contracts.rs` is the intended authority for pass IDs,
//! versions, descriptors, outcomes, and structured pass errors.
//!
//! The inspected implementation of `src/zuir/pass/manager.rs` still defines
//! duplicate legacy versions of these types, and the current registry imports
//! the legacy types. Therefore this façade exports:
//!
//! - Canonical contracts under their ordinary descriptive names.
//! - The currently usable manager and operational types.
//! - Manager-owned legacy contracts with explicit `Legacy` names.
//!
//! The `Legacy` names are deliberate. They must not be mistaken for aliases
//! of the canonical contract types.
//!
//! This is an integration boundary, not a claim that the ZUIR migration is
//! complete. The legacy types must be removed from the shared manager only
//! after the manager and registry have both migrated to `contracts.rs` and
//! the repository's consumers and tests have been updated.
//!
//! ## Execution contract
//!
//! The shared manager preserves explicit pass registration order. This
//! façade does not reorder passes, infer dependencies, or execute passes
//! concurrently.
//!
//! A pass descriptor is a declaration, not proof of correctness. A pass
//! implementation remains responsible for preserving semantic meaning,
//! accurately declaring analysis preservation and invalidation, and
//! cooperating with cancellation and configured work budgets.
//!
//! When verification is required, absence of a configured verifier must
//! fail closed. A failed or interrupted pass may have partially modified
//! its input; callers must not assume the IR remains valid or unchanged.
//!
//! ## Configuration boundary
//!
//! The shared manager uses its own `PassManagerConfig` and
//! `ExecutionBudget` contracts. It does not implicitly translate
//! `ZcirConfig` or ZUIR's general configuration into manager settings.
//!
//! A compiler integration layer must perform any deliberate configuration
//! mapping and document the meaning of every mapped setting. It must not
//! silently drop a budget, weaken verification, or change determinism policy.
//!
//! ## Scalability
//!
//! This module introduces no universal limits on:
//!
//! - Registered passes or analyses.
//! - Functions, blocks, regions, operations, or values.
//! - CPU cores, threads, accelerators, devices, or distributed nodes.
//! - Memory capacity, register width, tensor rank, or topology size.
//!
//! Operational budgets may be explicitly configured by callers. An absent
//! budget means that this manager imposes no explicit limit for that
//! dimension; it does not mean infinite time, memory, or address space.
//!
//! Budget exhaustion and cancellation must be reported rather than silently
//! truncating work or changing program semantics.
//!
//! ## Determinism
//!
//! Registration order is execution order. Pass IDs are not inferred from
//! registration positions. Reproducibility also depends on pass versions,
//! IR inputs, relevant configuration, dialect contracts, and external inputs.
//!
//! A descriptor declaring a pass deterministic is a contract, not a proof
//! that its implementation is deterministic.
//!
//! ## Dependency direction
//!
//! ```text
//! Frontend and semantic model
//!           |
//!           v
//!     Canonical ZUIR
//!           |
//!           v
//!   ZUIR pass contracts
//!           |
//!           v
//!   ZUIR pass manager
//!           |
//!           v
//!   ZCIR integration façade
//!           |
//!           v
//! Classical compiler pipeline
//! ```
//!
//! The dependency must not be reversed: foundational ZUIR contracts and
//! manager infrastructure must never depend on ZCIR.
//!
//! ## Integration contract
//!
//! 1. `src/classical/zcir/passes/mod.rs` declares this module.
//! 2. `src/classical/zcir/mod.rs` declares `pub mod passes;`.
//! 3. Consumers that need canonical pass metadata import it from the
//!    `canonical` namespace exported below.
//! 4. Consumers using the currently existing manager trait must use the
//!    explicitly named `LegacyPass` contract until the upstream migration
//!    is complete.
//! 5. The pass registry and manager must eventually use the same canonical
//!    contract types without adapters or duplicate definitions.
//! 6. ZCIR must use the existing ZUIR verifier and canonical IR types where
//!    applicable; this façade does not claim to implement those algorithms.
//! 7. Classical passes must not assume a fixed target, processor count,
//!    memory size, or hardware inventory.
//!
//! ## Freeze criteria
//!
//! This file can be frozen as a compatibility façade once its module
//! declarations compile and its consumers use the exports consistently.
//! Full production readiness additionally requires resolving the upstream
//! duplicate-contract migration, compiling the actual repository, and
//! passing the relevant unit, integration, conformance, and lint tests.

/// Canonical, shared pass-contract definitions.
///
/// These types are defined by `src/zuir/pass/contracts.rs`. This nested
/// namespace prevents them from being confused with the legacy types that
/// the current manager implementation still consumes.
pub mod canonical {
    pub use crate::zuir::pass::contracts::{
        PassDescriptor,
        PassError,
        PassErrorKind,
        PassId,
        PassOutcome,
        PassVersion,
    };
}

/// Shared ZUIR pass-manager operational facilities.
///
/// The operational types are re-exported rather than copied into ZCIR.
/// The `Legacy` prefix on the pass trait and contract types is intentional:
/// the current ZUIR manager still defines its own versions of those types.
pub use crate::zuir::pass::manager::{
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
    PassManager as ZcirPassManager,
    PassManagerConfig,
    PassOutcome as LegacyPassOutcome,
    PassVersion as LegacyPassVersion,
    PipelineReport,
    VerificationPolicy,
};

/// Shared ZUIR pass-registry implementation.
///
/// Registration and lookup remain owned by ZUIR. Do not add a separate ZCIR
/// registry or assign identities based on registration order.
pub use crate::zuir::pass::registry::PassRegistry;

/// Shared pass determinism policy and assessment APIs.
pub use crate::zuir::pass::determinism::{
    assess_pipeline_determinism,
    validate_pass_determinism,
    DeterminismAssessment,
    DeterminismPolicy,
};

/// Canonical analysis-invalidation calculation.
///
/// This function uses the canonical contracts. The legacy manager must be
/// migrated to these contracts before it can call this function directly.
pub use crate::zuir::pass::invalidation::compute_invalidated_analyses;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_contract_namespace_is_accessible() {
        let id = canonical::PassId::new("classical.test")
            .expect("test pass ID must be valid");

        let descriptor = canonical::PassDescriptor::new(
            id,
            canonical::PassVersion::new(1, 0, 0),
            "ZCIR integration test",
        );

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn manager_configuration_has_no_implicit_execution_ceiling() {
        let config = PassManagerConfig::default();

        assert!(config.budget.max_pass_executions.is_none());
        assert!(config.budget.max_elapsed.is_none());
    }

    #[test]
    fn cancellation_is_cooperative_and_shareable() {
        let token = CancellationToken::new();

        assert!(!token.is_cancelled());

        token.cancel();

        assert!(token.is_cancelled());
    }
}
