
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Testing
//!
//! Path: `src/hdl/testing/mod.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! Provide the public composition boundary for HDL test fixtures and
//! conformance testing. This module lets unit tests, integration tests,
//! regression suites, and downstream tooling share the same stable testing
//! APIs without implementing competing parsers or semantic validators.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - The feature-specific grammars under `grammar/hdl/`
//! - `grammar/Zamani.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - The canonical frontend lexer, AST, and semantic model
//!
//! The grammar defines accepted syntax. The canonical frontend parser and
//! AST define parsed source structure. The existing semantic validator owns
//! the HDL semantic checks it implements. This module only composes and
//! exposes testing facilities for those authorities.
//!
//! ## Owns
//!
//! - Public registration of the fixture and conformance modules.
//! - Deliberate re-exports of their supported testing APIs.
//! - The integration boundary for future HDL test facilities.
//!
//! ## Does not own
//!
//! - A lexer, parser, AST, semantic model, or diagnostic authority.
//! - HDL language semantics or target-feasibility rules.
//! - Simulation, synthesis, physical hardware access, or allocation.
//! - HDLIR data structures, builders, verification, passes, or lowering.
//! - Claims that a fixture has passed a stage unless the real implementation
//!   under test has actually evaluated it.
//!
//! All HDLIR implementation remains exclusively under `src/hdl/hdlir/`
//! and is outside the scope of this module.
//!
//! ## Integration contract
//!
//! - `fixtures` provides immutable source fixtures and catalog queries.
//! - `conformance` provides conformance cases, adapter contracts, observations,
//!   reports, and the semantic-validation entry point.
//! - Parser adapters must use the canonical lexer and the existing
//!   `HdlDomainParser` contract. They must not invent token constructors.
//! - Semantic adapters must preserve source information and use the existing
//!   AST-to-semantic integration boundary and `validate_hdl` implementation.
//! - Future test modules must be registered here only when their files and
//!   contracts exist.
//!
//! ## Scalability and determinism
//!
//! This module imposes no universal limit on fixture count, source length,
//! design dimensions, hardware width, or target capacity. Actual execution
//! remains bounded by available resources and the explicit operational
//! budgets of the implementation under test.
//!
//! The module introduces no random ordering, environment-dependent behavior,
//! hardware discovery, or hidden resource assumptions. Ordering and
//! determinism are governed by the fixture catalog and conformance runner.
//!
//! ## Completion and freeze criteria
//!
//! - Every registered module exists and compiles.
//! - Public exports correspond to implemented APIs.
//! - No competing parser, semantic authority, or IR is introduced.
//! - Incomplete analysis is never reported as successful conformance.
//! - The repository's formatting, compilation, tests, and Clippy checks pass.
//!
//! Adding a future test facility must not silently change the meaning of an
//! existing fixture, conformance expectation, or public API.

/// Reusable conformance cases and evaluation facilities.
///
/// This module defines the adapter boundary between the test suite and the
/// real parser or semantic implementation. An adapter must invoke the actual
/// implementation under test; a placeholder or mocked result is not evidence
/// of production conformance.
pub mod conformance;

/// Reusable, source-level HDL fixtures.
///
/// Fixtures document intended test scenarios. Their presence in the catalog
/// does not by itself mean the parser, semantic validator, or any target
/// implementation supports the corresponding feature.
pub mod fixtures;

// -----------------------------------------------------------------------------
// Stable public conformance API
// -----------------------------------------------------------------------------

pub use conformance::{
    run_conformance_suite,
    validate_semantic_input,
    ConformanceCaseResult,
    ConformanceDefinitionError,
    ConformanceExpectation,
    ConformanceObservation,
    ConformanceReport,
    ConformanceStage,
    ConformanceStatus,
    HdlConformanceAdapter,
    HdlConformanceCase,
};

// -----------------------------------------------------------------------------
// Stable public fixture API
// -----------------------------------------------------------------------------

pub use fixtures::{
    find_fixture,
    fixtures,
    fixtures_for_stage,
    fixtures_in_category,
    Fixture,
    FixtureCategory,
    FixtureId,
    FixtureStage,
};

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensure the public fixture API is connected to the canonical catalog.
    #[test]
    fn public_fixture_api_exposes_catalog() {
        assert!(
            !fixtures().is_empty(),
            "the HDL fixture catalog must contain at least one fixture"
        );
    }

    /// Ensure fixture identifiers are stable, nonempty, and unique.
    #[test]
    fn public_fixture_api_exposes_unique_identifiers() {
        let catalog = fixtures();

        for (index, fixture) in catalog.iter().enumerate() {
            assert!(
                !fixture.id().trim().is_empty(),
                "fixture at index {index} has an empty identifier"
            );

            assert!(
                !catalog[..index]
                    .iter()
                    .any(|previous| previous.id() == fixture.id()),
                "duplicate HDL fixture identifier: {}",
                fixture.id()
            );
        }
    }

    /// Ensure catalog lookup returns the same fixture as catalog iteration.
    #[test]
    fn public_fixture_lookup_is_consistent() {
        for fixture in fixtures() {
            let found = find_fixture(fixture.id());

            assert!(
                found.is_some(),
                "fixture lookup failed for `{}`",
                fixture.id()
            );

            assert_eq!(
                found.map(Fixture::id),
                Some(fixture.id()),
                "fixture lookup returned an inconsistent entry"
            );
        }
    }
}
