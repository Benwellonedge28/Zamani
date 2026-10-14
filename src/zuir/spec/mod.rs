
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Specification Module
//!
//! Path: `src/zuir/spec/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97 or later
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide the public module boundary for ZUIR's normative contracts,
//! invariant catalogue, compatibility policy, and conformance framework.
//!
//! This module composes specification modules. It does not implement a
//! second invariant catalogue, duplicate compatibility decisions, or
//! replace the executable checks owned by the verifier and other subsystems.
//!
//! ## AUTHORITY
//!
//! This module is subordinate to:
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/spec/invariants.rs`
//! - `src/zuir/spec/compatibility.rs`
//! - `src/zuir/spec/conformance.rs`
//! - `src/zuir/version.rs`
//! - `src/zuir/config.rs`
//! - `src/zuir/errors.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/compatibility.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//!
//! If two specifications conflict, resolve the conflict in the authoritative
//! specification and record the change explicitly. This module must not
//! silently redefine the meaning established by those specifications.
//!
//! ## OWNS
//!
//! - Public module declarations for ZUIR specification contracts.
//! - The stable paths used to access those contracts.
//! - Module-level integration tests for the specification layer.
//!
//! ## DOES_NOT_OWN
//!
//! - Concrete IR data structures or builders.
//! - The canonical invariant identifiers or their definitions.
//! - Compatibility algorithms, version negotiation, or migrations.
//! - Execution of conformance checks or verification of actual IR programs.
//! - Grammar productions, token definitions, AST construction, or semantic
//!   analysis.
//! - Serialization, fingerprinting, optimization, lowering, or code generation.
//! - Target capability discovery or physical resource allocation.
//!
//! ## PUBLIC MODULE CONTRACT
//!
//! `invariants` owns the canonical invariant catalogue.
//! `compatibility` owns compatibility dimensions, decisions, and policy.
//! `conformance` owns requirement descriptors, check execution, result
//! aggregation, and conformance reporting.
//!
//! Consumers should use these modules directly. This parent module must not
//! create competing aliases, duplicate stable identifiers, or expose private
//! implementation details from their owning modules.
//!
//! ## INTEGRATION
//!
//! - `src/zuir/mod.rs` declares `pub mod spec;`.
//! - `src/zuir/api.rs` may expose selected specification types through the
//!   supported public API, but must not duplicate their definitions.
//! - `verify/*` implementations consume invariant identifiers and descriptors
//!   from `invariants`; they remain responsible for checking concrete IR.
//! - Conformance checks consume the canonical invariant catalogue and map
//!   its requirements to actual verification, compatibility, serialization,
//!   determinism, and integration checks.
//! - Compatibility consumers use `compatibility` together with the canonical
//!   version types in `src/zuir/version.rs`.
//! - `config.rs` owns invocation policies and operational budgets; this
//!   module must not introduce global mutable configuration.
//! - The grammar and its specifications define source syntax and language
//!   semantics. They do not define the implementation details of these Rust
//!   module boundaries.
//!
//! The existence of a declared module or requirement does not prove that its
//! owning implementation exists, is wired into the compiler, or passes.
//! Conformance reports must be based on checks that actually ran.
//!
//! ## DEPENDENCY DIRECTION
//!
//! This module may declare its child modules, but must not depend on builders,
//! IR nodes, verifier internals, domain dialect implementations, lowering,
//! runtimes, or target-specific code.
//!
//! Child modules own their APIs and must not require this parent module to
//! re-export their types in order to function.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This module defines no universal limits on program size, operation count,
//! qubits, processors, devices, memory, register widths, tensor dimensions,
//! nodes, or dialects.
//!
//! Operational limits, when required, belong to the responsible subsystem
//! and its explicit configuration. Exhausting a work budget must not be
//! reported as successful verification or as proof that a program is invalid.
//!
//! ## DETERMINISM
//!
//! This module performs no I/O, environment inspection, hardware discovery,
//! time-dependent work, random selection, or implicit registration.
//! Deterministic ordering and reporting are owned by the child modules.
//!
//! ## FREEZE CRITERIA
//!
//! - All three specification modules are declared exactly once.
//! - Their ownership boundaries remain distinct.
//! - The invariant catalogue remains the single source of core invariant IDs.
//! - The compatibility module remains the owner of compatibility policy.
//! - Conformance execution never treats missing or inconclusive evidence as
//!   proof of success.
//! - Module integration tests verify the public specification contracts.
//! - No unsafe code or fixed hardware-capacity assumptions are introduced.

// ---------------------------------------------------------------------------
// Canonical specification modules
// ---------------------------------------------------------------------------

/// Canonical compatibility dimensions, decisions, policies, and requirements.
///
/// Use this module when deciding whether versions, schemas, or representation
/// changes may be accepted without an explicit migration.
pub mod compatibility;

/// Reusable conformance requirements, checks, outcomes, and reports.
///
/// A conformance report describes checks and evidence; it does not replace
/// the owning subsystem's verifier or prove untested behavior.
pub mod conformance;

/// Canonical invariant identifiers, applicability, and verification ownership.
///
/// Concrete IR verification remains the responsibility of the verifier
/// subsystem, not this catalogue.
pub mod invariants;

// ---------------------------------------------------------------------------
// Specification-layer integration tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{compatibility, conformance, invariants};

    /// The invariant catalogue must be available through the canonical
    /// specification module path and must contain defined entries.
    #[test]
    fn invariant_catalogue_is_exposed() {
        assert!(
            !invariants::CORE_INVARIANTS.is_empty(),
            "the canonical invariant catalogue must not be empty"
        );
    }

    /// Core invariant requirements must be derivable from the canonical
    /// catalogue rather than maintained as an unrelated second list.
    #[test]
    fn core_invariant_requirements_are_available() {
        let requirements = conformance::core_invariant_requirements();

        assert!(
            !requirements.is_empty(),
            "core invariant conformance requirements must be available"
        );
    }

    /// Compatibility policy and reporting must be accessible through the
    /// specification boundary without redefining their implementation.
    #[test]
    fn compatibility_contract_is_exposed() {
        let _strict_policy = compatibility::CompatibilityPolicy::strict();
        let _report = compatibility::CompatibilityReport::new();
    }

    /// The invariant catalogue must retain stable identifiers that can be
    /// looked up by downstream verification and conformance implementations.
    #[test]
    fn canonical_invariant_lookup_is_available() {
        let identifier = invariants::id::REFERENCE_INTEGRITY;

        assert!(invariants::is_core_invariant(identifier));
        assert!(invariants::find_core_invariant(identifier).is_some());
    }
}
