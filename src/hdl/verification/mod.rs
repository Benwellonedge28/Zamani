
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Verification
//!
//! Path: `src/hdl/verification/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: HDL verification composition root.
//!
//! ## Purpose
//!
//! Provide the public module boundary for Zamani's target-independent HDL
//! verification models and result aggregators.
//!
//! This file registers existing verification modules. It does not implement
//! a parser, a second AST, a semantic analyzer, a verification engine, or an
//! intermediate representation.
//!
//! ## Normative authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/assertions.g4`
//! - `grammar/hdl/verification.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/frontend/semantic/model/`
//! - `src/hdl/contract.rs`
//!
//! The grammar owns source syntax. The canonical AST owns source structure
//! and expression nodes. The frontend semantic model owns canonical semantic
//! identities and shared semantic facts. The HDL contract module owns the
//! HDL behavioral-contract representation.
//!
//! This composition root must not override those authorities.
//!
//! ## Module ownership
//!
//! - `assertions`: assertion declarations, reusable source properties,
//!   reference relationships, and structural validation.
//! - `properties`: portable temporal-property structures and validation of
//!   property structure and temporal bounds.
//! - `invariants`: invariant-specific validation and aggregation of canonical
//!   HDL contract assessments.
//! - `coverage`: coverage goals, provider observations, and coverage
//!   aggregation.
//! - `equivalence`: equivalence claims, provider assessments, and result
//!   aggregation.
//!
//! Each module retains its own public types under its module path. This
//! boundary intentionally does not flatten those types into one namespace.
//!
//! ## Integration contract
//!
//! The intended integration flow is:
//!
//! 1. The canonical parser and AST adapter preserve HDL source meaning,
//!    expression references, identities, and source locations.
//! 2. The semantic pipeline resolves names, checks types, validates relevant
//!    assumptions, and determines which verification requests are well-formed.
//! 3. The caller constructs the appropriate verification model using the
//!    module-specific public API.
//! 4. A simulation, formal-verification, or coverage provider performs the
//!    requested analysis and returns provider-reported results.
//! 5. The appropriate module validates record relationships and aggregates
//!    results without claiming to independently prove provider evidence.
//! 6. The caller interprets the result alongside diagnostics, assumptions,
//!    analysis completion, source version, and provider trust policy.
//!
//! This module does not invent constructors for canonical AST or semantic
//! types. It does not create a competing expression, identifier, contract,
//! source-location, or diagnostic system.
//!
//! ## Result soundness
//!
//! Structural validity is not proof of behavioral correctness.
//!
//! Provider-reported satisfaction, equivalence, or coverage is not
//! independently authenticated or proved by this composition root.
//! Consumers must preserve distinctions between:
//!
//! - invalid input and valid-but-unproven input;
//! - a reported counterexample and an inconclusive result;
//! - no assessment and an assessment that found no violation;
//! - complete analysis and incomplete or budget-exhausted analysis;
//! - source-language validity and target feasibility.
//!
//! No module may silently convert unknown, missing, contradictory, or
//! incomplete evidence into successful verification.
//!
//! ## Scalability and POCO-REAF
//!
//! This composition root imposes no language-level maximum on assertions,
//! properties, invariants, coverage goals, equivalence claims, subjects,
//! or assessment records.
//!
//! It introduces no fixed hardware-width, signal-count, device-count,
//! memory-capacity, or target-specific assumptions.
//!
//! Any operational budgets belong to the relevant module or its caller.
//! Exhausting a budget must remain distinguishable from successful,
//! complete verification. Actual processing is necessarily bounded by
//! available time, memory, and the representational limits of the host.
//!
//! ## Explicit exclusions
//!
//! This module does not own:
//!
//! - grammar rules, lexing, parsing, or AST construction;
//! - general name resolution, expression typing, or semantic analysis;
//! - simulation execution or event scheduling;
//! - formal proof engines, SAT/SMT solvers, or model checkers;
//! - verification-provider authentication or proof-certificate checking;
//! - target selection, physical allocation, synthesis, or timing closure;
//! - HDLIR structures, builders, verification, passes, serialization,
//!   transformations, or lowering.
//!
//! All HDLIR implementation remains exclusively under `src/hdl/hdlir/`
//! and is outside the scope of this module.
//!
//! ## Public API policy
//!
//! Modules are exposed by their qualified paths, for example:
//!
//! - `crate::hdl::verification::assertions`
//! - `crate::hdl::verification::properties`
//! - `crate::hdl::verification::invariants`
//! - `crate::hdl::verification::coverage`
//! - `crate::hdl::verification::equivalence`
//!
//! No wildcard re-exports are used. This avoids collisions between types
//! owned by different modules and allows each module's API to evolve under
//! its own documented compatibility contract.
//!
//! ## Freeze criteria
//!
//! This composition root is ready to freeze when:
//!
//! - every registered module exists and compiles;
//! - every registered module has one clear ownership boundary;
//! - canonical frontend and contract types are reused;
//! - no duplicate AST, semantic model, or IR is introduced;
//! - incomplete analysis cannot be presented as complete verification;
//! - module visibility and public API integration pass workspace checks;
//! - the HDL grammar conformance suite and relevant tests pass.

/// Assertion declarations and reusable source-property validation.
///
/// This module owns assertion/property input structures, identity and
/// reference checks, and validation through the supplied semantic
/// environment. It does not execute assertions or prove properties.
pub mod assertions;

/// Coverage goals and provider-reported observation aggregation.
///
/// Coverage results describe the supplied observations; they do not by
/// themselves prove reachability or unreachability.
pub mod coverage;

/// Equivalence claims and provider-reported assessment aggregation.
///
/// This module records and checks the structure of equivalence assessments.
/// It does not implement an equivalence solver or independently validate
/// proof certificates.
pub mod equivalence;

/// Invariant validation and aggregation over canonical HDL contracts.
///
/// Contract declarations and assessment types remain owned by
/// `crate::hdl::contract`; this module does not redefine them.
pub mod invariants;

/// Portable temporal-property structures and structural validation.
///
/// This module does not replace the canonical AST or perform general
/// expression resolution, type checking, simulation, or formal proof.
pub mod properties;

#[cfg(test)]
mod tests {
    //! Composition-root tests.
    //!
    //! Behavioral and semantic tests belong to their owning modules.
    //! These checks ensure the expected module namespaces remain available
    //! through the verification boundary.

    use super::{assertions, coverage, equivalence, invariants, properties};

    #[test]
    fn verification_modules_are_registered() {
        // Taking references to each module's public API verifies that the
        // composition root resolves the existing submodules without creating
        // a second type namespace or relying on wildcard re-exports.
        let _ = assertions::validate_assertions;
        let _ = coverage::analyze_coverage;
        let _ = equivalence::analyze_equivalence;
        let _ = invariants::analyze_invariants;
        let _ = properties::PropertySet::new;
    }
}
