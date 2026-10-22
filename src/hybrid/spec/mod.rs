
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Hybrid Computing — Specification Module
//!
//! Path: `src/hybrid/spec/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide the public composition boundary for the normative hybrid
//! specification contracts implemented by this module tree.
//!
//! The specification module describes architectural obligations and
//! exposes machine-readable definitions that implementation modules and
//! conformance tests can consume.
//!
//! It does not independently implement hybrid semantics, construct or
//! validate hybrid graphs, negotiate hardware capabilities, perform
//! compatibility migrations, or execute computations.
//!
//! ## Repository authority
//!
//! Hybrid specifications must remain consistent with:
//!
//! - `grammar/specification/`: language-wide semantic authority.
//! - `grammar/spec/hybrid.md`: normative hybrid-computing contract.
//! - `grammar/Zamani.g4` and its canonical composed grammar components:
//!   source-language grammar authority.
//! - `grammar/hybrid/`: hybrid grammar and integration contract.
//! - `src/frontend/ast/`: canonical source representation.
//! - The canonical semantic layer: types, effects, ownership, capabilities,
//!   requirements, constraints, policies, and provenance.
//! - `src/quantum/ir/`: canonical quantum semantic representation.
//! - Canonical classical and other domain IR implementations.
//! - `src/compiler/hybrid_pipeline.rs`: existing hybrid compilation boundary.
//! - `src/runtime/hybrid_kernel.rs`: existing runtime integration point.
//!
//! This module does not override any of those authorities.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The public specification-module boundary.
//! - The stable re-export surface for hybrid architectural invariants.
//! - Documentation of the relationship between specification contracts
//!   and their implementation.
//!
//! `invariants.rs` owns the invariant catalog, stable invariant identifiers,
//! enforcement-stage classifications, catalog lookup, and catalog
//! structural validation.
//!
//! Future specification modules must have their own explicit ownership
//! contracts and must be declared here only when their implementation files
//! exist and their public interfaces have been reviewed.
//!
//! ## Enforcement contract
//!
//! An invariant definition describes an obligation. It does not prove that
//! the obligation is already enforced by the compiler, runtime, semantic
//! layer, graph verifier, or interchange implementation.
//!
//! Consumers must not interpret successful catalog validation as proof that
//! a hybrid program is semantically valid or executable on a target.
//!
//! Actual enforcement remains the responsibility of the subsystem named by
//! each invariant's enforcement contract.
//!
//! ## Scalability and portability
//!
//! This module introduces no hardware-capacity or workload limits.
//!
//! It must not introduce fixed universal maxima for domain count, graph size,
//! operations, edges, dependencies, qubits, processors, devices, memory,
//! tensor rank, register width, nesting, or other program-defined quantities.
//!
//! Operational budgets, when required, belong to explicit configuration and
//! the operation consuming each budget. Budget exhaustion must be reported
//! explicitly and must not silently change program meaning.
//!
//! POCO-REAF means that portable source intent and semantics are preserved
//! across compatible compilation and execution environments. It does not
//! guarantee that every target has the capabilities or resources needed to
//! execute every possible program.
//!
//! ## Determinism
//!
//! The public catalog is static and read-only. Its order and stable
//! identifiers are defined by `invariants.rs`, not by filesystem order,
//! target discovery, runtime registration, or hash-map iteration.
//!
//! ## Integration
//!
//! - Hybrid implementation modules may import the invariant definitions
//!   through this module's public re-exports.
//! - Graph verification and semantic adapters must enforce applicable
//!   invariants using their own canonical models.
//! - Compiler and runtime adapters must preserve structured failures.
//! - Conformance tests must distinguish catalog integrity from actual
//!   enforcement evidence.
//! - `src/hybrid/mod.rs` must expose this module through the agreed
//!   hybrid public API.
//! - `src/lib.rs` must expose the hybrid subsystem without duplicating
//!   existing module paths.
//!
//! Compatibility policy and migration logic belong in a dedicated
//! `compatibility.rs` module when that file and its contracts are
//! implemented. This module intentionally does not declare a nonexistent
//! child module.
//!
//! ## Completion criteria
//!
//! - The invariant catalog is accessible through a stable public surface.
//! - Catalog validation remains owned by `invariants.rs`.
//! - No competing semantic model or IR is introduced.
//! - No absent module is declared.
//! - Public exports resolve and are covered by tests.
//! - The module uses safe Rust and introduces no hardware-size ceilings.

// ---------------------------------------------------------------------------
// Specification modules
// ---------------------------------------------------------------------------

/// Machine-readable architectural invariants for hybrid computation.
///
/// This module is the authoritative owner of invariant definitions and
/// catalog validation. Its definitions describe required behavior; they do
/// not replace the validators that enforce those requirements.
pub mod invariants;

// ---------------------------------------------------------------------------
// Stable public specification surface
// ---------------------------------------------------------------------------

pub use invariants::{
    all_invariants,
    contains_invariant,
    invariant_by_code,
    validate_catalog,
    CatalogError,
    EnforcementStage,
    InvariantDefinition,
    InvariantId,
    HYBRID_INVARIANTS,
};

// ---------------------------------------------------------------------------
// Public-surface conformance tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_catalog_is_available() {
        assert!(!all_invariants().is_empty());
        assert_eq!(all_invariants(), HYBRID_INVARIANTS);
    }

    #[test]
    fn public_catalog_validation_succeeds() {
        assert_eq!(validate_catalog(), Ok(()));
    }

    #[test]
    fn public_lookup_uses_stable_machine_readable_codes() {
        for definition in all_invariants() {
            let found = invariant_by_code(definition.id.as_str())
                .expect("every published invariant must be discoverable");

            assert_eq!(found.id, definition.id);
        }
    }

    #[test]
    fn public_membership_check_matches_catalog() {
        for definition in all_invariants() {
            assert!(contains_invariant(definition.id));
        }
    }

    #[test]
    fn unknown_invariants_are_not_reported_as_present() {
        assert!(!contains_invariant(InvariantId::SafeImplementation)
            || all_invariants()
                .iter()
                .any(|entry| entry.id == InvariantId::SafeImplementation));

        assert!(invariant_by_code("HYB-INV-UNKNOWN").is_none());
    }

    #[test]
    fn enforcement_metadata_is_exposed_through_the_public_api() {
        for definition in all_invariants() {
            assert!(!definition.title.trim().is_empty());
            assert!(!definition.statement.trim().is_empty());
            assert!(!definition.owner.trim().is_empty());
            assert!(!definition.failure_behavior.trim().is_empty());
            assert!(!definition.verification.trim().is_empty());
        }
    }
}
