
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Compatibility Contract Tests
//!
//! Path: `src/hdl/hdlir/tests/compatibility.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: Architectural compatibility tests for the HDLIR foundation.
//!
//! ## Purpose
//!
//! Establish regression checks for the repository authorities that HDLIR
//! must obey before versioned HDLIR artifacts and migrations are implemented.
//!
//! These tests deliberately distinguish:
//!
//! - repository architecture compatibility;
//! - source-language semantic compatibility;
//! - IR schema and serialized-artifact compatibility;
//! - frontend and ZUIR integration compatibility.
//!
//! The first category can be tested against the current repository sources.
//! The remaining implementation-level categories require actual HDLIR
//! schema, serialization, migration, and lowering APIs. They must not be
//! simulated by inventing a second implementation in this test file.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/Zamani.g4`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/lexer/tokens.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/hdl/mod.rs`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The specifications define required semantics and ownership. These tests
//! do not replace those specifications or claim that documented features
//! are implemented.
//!
//! ## Ownership
//!
//! This file owns only compatibility regression tests.
//!
//! It does not own:
//!
//! - HDLIR schema versions or version negotiation;
//! - serialized artifact encoding or decoding;
//! - schema migration implementations;
//! - operation or type registration;
//! - frontend parsing or semantic validation;
//! - AST, semantic-model, or ZUIR identities;
//! - hardware realization or target compatibility.
//!
//! Those contracts belong to their authoritative implementations under
//! `src/hdl/hdlir/` or the existing repository modules.
//!
//! ## Integration contract
//!
//! Include this file from the HDLIR test module after `src/hdl/hdlir/mod.rs`
//! becomes a real Rust module. All source paths below are relative to this
//! file and refer to existing repository files verified during the audit.
//!
//! When HDLIR serialization is implemented, extend these tests to use its
//! actual public version, schema registry, encoder, decoder, and migration
//! APIs. Do not introduce test-only substitutes for those production APIs.
//!
//! ## Scalability
//!
//! These architectural checks impose no hardware-capacity limits. Actual
//! artifact processing must use the operational budgets defined by the
//! production serialization API and must report incomplete work explicitly.

const HDL_SPEC: &str = include_str!("../../../../grammar/spec/hdl.md");
const GRAMMAR_DESIGN: &str = include_str!("../../../../grammar/DESIGN.md");
const ROOT_GRAMMAR: &str = include_str!("../../../../grammar/Zamani.g4");
const HDL_GRAMMAR: &str = include_str!("../../../../grammar/hdl/hdl.g4");
const TOKEN_REGISTRY: &str = include_str!("../../../../grammar/lexer/tokens.g4");
const SEMANTIC_MODEL: &str =
    include_str!("../../../../grammar/specification/semantic-model.md");
const SEMANTIC_BOUNDARIES: &str =
    include_str!("../../../../grammar/validation/semantic-boundaries.md");
const HDL_COMPOSITION_ROOT: &str = include_str!("../../mod.rs");
const HDL_PUBLIC_API: &str = include_str!("../../api.rs");
const HDL_IDENTITIES: &str = include_str!("../../ids.rs");
const ZUIR_SPEC: &str = include_str!("../../../zuir/ZUIR-SPEC.md");
const HDLIR_MODULE: &str = include_str!("../mod.rs");

/// Assert that a repository contract contains the required invariant.
///
/// Keep assertions specific enough to detect a meaningful regression,
/// without coupling the tests to paragraph formatting or line numbers.
fn assert_contract_contains(contract: &str, required: &str, name: &str) {
    assert!(
        contract.contains(required),
        "{name} is missing the required compatibility invariant: {required:?}"
    );
}

/// Assert that a repository contract does not contain a prohibited design.
///
/// The caller must supply a distinctive phrase rather than a generic word
/// that might legitimately occur in explanatory documentation.
fn assert_contract_excludes(contract: &str, prohibited: &str, name: &str) {
    assert!(
        !contract.contains(prohibited),
        "{name} contains a prohibited contract or stale implementation: \
         {prohibited:?}"
    );
}

#[test]
fn normative_hdl_specification_exists_and_defines_portability() {
    assert_contract_contains(
        HDL_SPEC,
        "POCO-REAF",
        "HDL specification",
    );
    assert_contract_contains(
        HDL_SPEC,
        "symbolic",
        "HDL specification",
    );
    assert_contract_contains(
        HDL_SPEC,
        "physical",
        "HDL specification",
    );
}

#[test]
fn grammar_root_remains_a_composition_boundary() {
    assert_contract_contains(
        ROOT_GRAMMAR,
        "CANONICAL ANTLR ROOT",
        "Zamani root grammar",
    );

    // The root grammar must not become the owner of HDLIR implementation.
    assert_contract_excludes(
        ROOT_GRAMMAR,
        "struct HDLIR",
        "Zamani root grammar",
    );
}

#[test]
fn hdl_grammar_remains_the_hdl_composition_authority() {
    assert_contract_contains(
        HDL_GRAMMAR,
        "COMPOSITION ROOT",
        "HDL grammar",
    );

    // Syntax and grammar composition must not become artifact serialization.
    assert_contract_excludes(
        HDL_GRAMMAR,
        "HDLIRArtifactDecoder",
        "HDL grammar",
    );
}

#[test]
fn token_registry_does_not_own_ir_or_target_realization() {
    assert_contract_contains(
        TOKEN_REGISTRY,
        "EXTENSIBILITY",
        "Canonical token registry",
    );
    assert_contract_contains(
        TOKEN_REGISTRY,
        "IR definitions or target realization",
        "Canonical token registry",
    );
}

#[test]
fn semantic_model_remains_separate_from_target_realization() {
    assert_contract_contains(
        SEMANTIC_MODEL,
        "semantic model",
        "Canonical semantic model",
    );
    assert_contract_contains(
        SEMANTIC_MODEL,
        "MUST NOT",
        "Canonical semantic model",
    );

    // HDLIR compatibility must not redefine source-level meaning.
    assert_contract_contains(
        SEMANTIC_BOUNDARIES,
        "TARGET REALIZATION",
        "Semantic-boundary specification",
    );
}

#[test]
fn hdlir_implementation_has_a_single_repository_boundary() {
    assert_contract_contains(
        HDL_COMPOSITION_ROOT,
        "src/hdl/hdlir/",
        "HDL composition root",
    );

    assert_contract_contains(
        HDL_COMPOSITION_ROOT,
        "All future HDLIR-owned functionality belongs under",
        "HDL composition root",
    );
}

#[test]
fn stale_placeholder_registration_is_not_a_compatibility_contract() {
    // The audited parent module contains a test referencing
    // `super::hdlir::Placeholder`, while the audited HDLIR module does not
    // define a Rust item. Once HDLIR is implemented, the parent test must
    // verify an actual public API instead of retaining that placeholder.
    assert_contract_excludes(
        HDL_COMPOSITION_ROOT,
        "super::hdlir::Placeholder",
        "HDL composition root",
    );

    assert_contract_excludes(
        HDLIR_MODULE,
        "hardware description language intermediate representation (HDLIR)",
        "HDLIR module root",
    );
}

#[test]
fn existing_hdl_public_api_remains_the_frontend_integration_boundary() {
    assert_contract_contains(
        HDL_PUBLIC_API,
        "HDL_API_SCHEMA_VERSION",
        "HDL public API",
    );

    assert_contract_contains(
        HDL_PUBLIC_API,
        "validate_hdl",
        "HDL public API",
    );

    // HDLIR must consume established contracts, not duplicate the parser
    // or source-level semantic validator.
    assert_contract_contains(
        HDL_COMPOSITION_ROOT,
        "crate::frontend::parser::domains::hdl",
        "HDL composition root",
    );

    assert_contract_contains(
        HDL_COMPOSITION_ROOT,
        "crate::frontend::semantic::domains::hdl",
        "HDL composition root",
    );
}

#[test]
fn hdl_identity_ownership_is_explicit() {
    assert_contract_contains(
        HDL_IDENTITIES,
        "HdlDesignId",
        "HDL identity infrastructure",
    );

    assert_contract_contains(
        HDL_IDENTITIES,
        "HdlModuleId",
        "HDL identity infrastructure",
    );

    // Compatibility tests must not assume that every ID belongs to one
    // globally shared numeric namespace.
    assert_contract_contains(
        HDL_IDENTITIES,
        "canonical",
        "HDL identity infrastructure",
    );
}

#[test]
fn zuir_remains_distinct_from_the_hardware_domain_ir() {
    assert_contract_contains(
        ZUIR_SPEC,
        "Universal Intermediate Representation",
        "ZUIR specification",
    );

    assert_contract_contains(
        ZUIR_SPEC,
        "target-independent",
        "ZUIR specification",
    );

    assert_contract_contains(
        HDL_SPEC,
        "hardware",
        "HDL specification",
    );

    // HDLIR compatibility must not be used to redefine ZUIR as a
    // physical hardware representation.
    assert_contract_excludes(
        ZUIR_SPEC,
        "HDLIR is the canonical universal IR",
        "ZUIR specification",
    );
}

#[test]
fn architecture_preserves_explicit_failure_and_incomplete_results() {
    assert_contract_contains(
        GRAMMAR_DESIGN,
        "incomplete",
        "Grammar architecture specification",
    );

    assert_contract_contains(
        HDL_SPEC,
        "MUST NOT silently",
        "HDL specification",
    );

    assert_contract_contains(
        SEMANTIC_BOUNDARIES,
        "MUST NOT silently",
        "Semantic-boundary specification",
    );
}

#[test]
fn source_authorities_prohibit_unsafe_implementation() {
    assert_contract_contains(
        GRAMMAR_DESIGN,
        "unsafe",
        "Grammar architecture specification",
    );

    assert_contract_contains(
        HDL_COMPOSITION_ROOT,
        "Unsafe Rust is forbidden",
        "HDL composition root",
    );

    // This is a guard against accidentally weakening the repository's
    // documented safety contract; crate attributes still enforce the rule.
    assert_contract_contains(
        HDL_PUBLIC_API,
        "#![forbid(unsafe_code)]",
        "HDL public API",
    );
}

// ---------------------------------------------------------------------------
// Future artifact-compatibility test contract
// ---------------------------------------------------------------------------
//
// The tests below are intentionally specified here before implementation,
// but cannot be honestly implemented against production APIs until those APIs
// exist. Add them in a reviewed follow-up when the corresponding modules are
// available:
//
// 1. Current-version artifacts decode and verify successfully.
// 2. Every supported historical version has a real, versioned fixture.
// 3. Unsupported major versions fail with a structured compatibility error.
// 4. Unknown required dialects or operations are rejected, not misinterpreted.
// 5. Unknown optional extensions are preserved only under an explicit format
//    contract; preservation must never imply semantic understanding.
// 6. Every migration either preserves defined semantics or reports failure.
// 7. Failed migrations do not expose a partially migrated artifact as valid.
// 8. Decode budgets produce explicit exhaustion results, not silent truncation.
// 9. Equivalent canonical designs encode identically when the format promises
//    deterministic encoding.
// 10. Fingerprints include the schema and semantic inputs that affect meaning.
// 11. Malformed, truncated, oversized, and inconsistent artifacts fail safely.
// 12. Symbolic widths and parameterized hardware are not narrowed merely to
//     satisfy a fixed-width artifact field.
// 13. Provenance survives migrations to the extent guaranteed by the format.
// 14. A target's inability to realize a design is not misclassified as an
//     artifact-version incompatibility.
// 15. Migration coverage is tested for every supported version transition.
//
// These are acceptance criteria, not claims that a serializer or migration
// implementation already exists.
