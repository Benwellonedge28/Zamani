#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Semantic Contracts
//!
//! Path: `src/frontend/semantic/contracts/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide the public module boundary for target-independent semantic
//! contracts, preconditions, postconditions, and invariants.
//!
//! This module composes the canonical contract model and its analysis
//! modules. It does not introduce another contract representation,
//! expression tree, obligation allocator, or verification engine.
//!
//! ## AUTHORITY
//!
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/grammar.md`
//! - `grammar/Zamani.g4`
//! - `grammar/statements/contract.g4`
//! - `grammar/functions/contracts.g4`
//! - `grammar/validation/preconditions.g4`
//! - `grammar/validation/postconditions.g4`
//! - `src/frontend/ast/node/`
//! - The sibling modules declared below.
//!
//! The normative specifications define semantic meaning. The canonical
//! grammar defines source syntax. The canonical AST defines source identity
//! and locations. `model.rs` owns the shared semantic contract types.
//!
//! ## OWNS
//!
//! - Contract submodule composition and visibility.
//! - Explicit, stable re-exports of supported contract APIs.
//! - The public contract-module schema identifier.
//! - The dependency boundary between shared contract data and analyses.
//!
//! ## DOES_NOT_OWN
//!
//! - Source grammar, parsing, or AST construction.
//! - A competing semantic AST or semantic type system.
//! - Name resolution, expression type inference, or effect inference.
//! - The implementation of a proof engine or runtime assertion executor.
//! - Target discovery, capability discovery, resource allocation,
//!   scheduling, routing, lowering, or runtime execution.
//! - Diagnostic rendering or the semantic pipeline's pass ordering.
//! - Classical IR, quantum IR, HDL IR, or another intermediate representation.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `model.rs` is the sole owner of canonical contract and obligation types.
//! - `preconditions.rs` assesses preconditions using existing semantic facts.
//! - `postconditions.rs` constructs pending postcondition obligations.
//! - `invariants.rs` collects pending invariant obligations.
//! - All analysis modules use the canonical `ObligationIdAllocator`.
//! - Contract extraction validates source references against the canonical
//!   AST before passing contracts to these analysis modules.
//! - The parent semantic module exposes this module only after its public
//!   API and sibling dependencies have been integrated.
//! - The semantic pipeline owns analysis ordering and finalization.
//! - The diagnostic subsystem adapts structured errors to user diagnostics.
//! - Verification outcomes do not establish target feasibility.
//!
//! ## SCALABILITY
//!
//! This module defines no machine-capacity ceilings, domain-count limits,
//! fixed contract-count limits, or hardware-specific types.
//!
//! The contract model preserves source-level meaning independently of the
//! target that may eventually realize a program. Operational limits belong
//! to explicit compiler policy. An interrupted or incomplete analysis must
//! never be reported as complete verification.
//!
//! No finite implementation can promise infinite computation. The
//! scalability objective is to avoid arbitrary language-level ceilings
//! while respecting representational limits, available memory, processing
//! capacity, explicit policies, and the semantics of the program.
//!
//! ## DETERMINISM
//!
//! Re-exports do not change analysis order or semantics. The underlying
//! modules are responsible for deterministic clause processing, stable
//! identities within their allocation domain, and explicit completion
//! states.
//!
//! ## ERROR HANDLING
//!
//! This facade introduces no panics or error conversions. Errors and
//! incomplete results remain owned by their respective analysis modules.
//! A successful obligation-collection call does not mean that a condition
//! has been proven.
//!
//! ## COMPATIBILITY
//!
//! `CONTRACTS_MODULE_SCHEMA_VERSION` versions this facade independently
//! of the language version, AST schema, and contract-model schema.
//! Public API changes must be deliberate and covered by compatibility tests.
//!
//! ## FREEZE CRITERIA
//!
//! This file is ready to freeze when:
//!
//! 1. each declared sibling module exists and compiles;
//! 2. the re-exports match the actual public interfaces;
//! 3. the parent frontend and semantic module hierarchy exports this module;
//! 4. contract-model and analysis tests pass;
//! 5. the invariant collection path contains no avoidable panic;
//! 6. no test attempts to exhaust a numeric identity space;
//! 7. the relevant workspace and integration tests pass.

// =============================================================================
// Module schema
// =============================================================================

/// Schema version for the public semantic-contract module boundary.
///
/// Increment this value when the facade's compatibility contract changes.
/// This is not the language version or the contract-model schema version.
pub const CONTRACTS_MODULE_SCHEMA_VERSION: u16 = 1;

/// Stable architectural name of this module.
pub const CONTRACTS_MODULE_NAME: &str =
    "zamani.frontend.semantic.contracts";

/// Return the schema version of the public contract-module boundary.
#[must_use]
pub const fn schema_version() -> u16 {
    CONTRACTS_MODULE_SCHEMA_VERSION
}

/// Return the stable architectural name of this module.
#[must_use]
pub const fn module_name() -> &'static str {
    CONTRACTS_MODULE_NAME
}

// =============================================================================
// Canonical module owners
// =============================================================================

/// Canonical target-independent contract and obligation data model.
pub mod model;

/// Preconditions analysis over canonical contract clauses.
pub mod preconditions;

/// Postcondition-obligation construction.
pub mod postconditions;

/// Invariant-obligation collection.
pub mod invariants;

// =============================================================================
// Canonical model re-exports
// =============================================================================
//
// These explicit exports provide a convenient public entry point while
// preserving `model.rs` as the sole owner of the underlying definitions.
// Wildcard re-exports are intentionally avoided.

pub use model::{
    Contract,
    ContractClause,
    ContractExpression,
    ContractKind,
    ContractKindId,
    ContractMessage,
    ContractModelError,
    ContractObligation,
    ObligationId,
    ObligationIdAllocator,
    ObligationOrigin,
    VerificationOutcome,
    CONTRACT_MODEL_NAME,
    CONTRACT_MODEL_SCHEMA_VERSION,
};

// =============================================================================
// Preconditions API
// =============================================================================

pub use preconditions::{
    analyze_preconditions,
    PredicateAnalysis,
    PredicateEffects,
    PredicateTruth,
    PredicateType,
    PreconditionAnalysis,
    PreconditionAnalysisError,
    PreconditionCompletion,
    PreconditionIssue,
    PreconditionOptions,
    PreconditionReport,
    PreconditionSemantics,
    PreconditionStatus,
};

// =============================================================================
// Postconditions API
// =============================================================================

pub use postconditions::{
    collect_postconditions,
    postcondition_count,
    PostconditionError,
    PostconditionObligations,
};

// =============================================================================
// Invariants API
// =============================================================================

pub use invariants::{
    collect_all_invariants,
    collect_invariants,
    invariant_count,
    is_invariant_kind,
    InvariantAnalysis,
    InvariantCompletion,
    InvariantError,
    InvariantKind,
    InvariantOptions,
    InvariantReport,
};

// =============================================================================
// Architectural API assertions
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            module_name(),
            "zamani.frontend.semantic.contracts"
        );
        assert_eq!(schema_version(), 1);
    }

    #[test]
    fn canonical_model_types_are_reachable_from_facade() {
        // Compile-time API check: these values use the shared model types,
        // not duplicate definitions introduced by this facade.
        let allocator = ObligationIdAllocator::new();
        assert_eq!(allocator, ObligationIdAllocator::default());

        let kind = ContractKind::Invariant;
        assert!(!kind.is_extension());

        let outcome = VerificationOutcome::Pending;
        assert!(!outcome.is_proven());
        assert!(!outcome.is_decided());
        assert!(outcome.requires_attention());
    }

    #[test]
    fn analysis_options_default_to_no_local_count_budget() {
        assert_eq!(PreconditionOptions::unrestricted().max_preconditions(), None);
        assert_eq!(InvariantOptions::unrestricted().max_invariants(), None);
    }

    #[test]
    fn explicit_budgets_are_operational_not_language_limits() {
        assert_eq!(
            PreconditionOptions::with_max_preconditions(0)
                .max_preconditions(),
            Some(0)
        );
        assert_eq!(
            InvariantOptions::with_max_invariants(0).max_invariants(),
            Some(0)
        );
    }
}