
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Test Fixtures
//!
//! Path: `src/hdl/testing/fixtures.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: safe Rust only.
//!
//! ## Purpose
//!
//! Provide a deterministic, reusable catalog of source-level HDL fixtures
//! for parser integration, semantic validation, conformance tests,
//! diagnostics, scalability tests, and future target-feasibility tests.
//!
//! ## Authority
//!
//! The fixtures must remain consistent with:
//!
//! - `grammar/DESIGN.md`;
//! - `grammar/spec/hdl.md`;
//! - `grammar/hdl/hdl.g4`;
//! - the feature-specific grammars under `grammar/hdl/`;
//! - `src/frontend/parser/domains/hdl.rs`;
//! - `src/frontend/semantic/domains/hdl.rs`;
//! - the canonical frontend lexer, AST, and semantic model.
//!
//! Grammar files own syntax. The frontend parser owns parsing. The canonical
//! AST owns source structure. The semantic validator owns source-level HDL
//! validation. Target feasibility and realization belong downstream.
//!
//! ## Ownership
//!
//! This file owns only:
//!
//! - immutable fixture descriptions;
//! - fixture identifiers and categories;
//! - fixture source text;
//! - intended test-stage metadata;
//! - deterministic catalog lookup and iteration;
//! - fixture-catalog integrity tests.
//!
//! It does not own:
//!
//! - a lexer, parser, AST, or semantic model;
//! - HDL type or width semantics;
//! - diagnostic representation;
//! - simulation or execution;
//! - target discovery or hardware allocation;
//! - HDLIR, IR builders, IR verification, or IR lowering;
//! - vendor-specific implementations.
//!
//! ## Integration contract
//!
//! `src/hdl/testing/mod.rs` should declare this module and selectively
//! re-export its public API.
//!
//! Parser conformance tests should feed `Fixture::source()` into the
//! canonical frontend lexer/parser or the existing HDL parser adapter.
//! Semantic tests should construct their inputs through the actual AST-to-
//! semantic adapter and invoke the existing HDL semantic validator.
//! Target-feasibility tests should use the eventual target API.
//!
//! This file deliberately does not guess constructors or field names in
//! those APIs. A fixture's intended test stage is metadata, not evidence that
//! the current parser or validator already accepts the source.
//!
//! ## Scalability
//!
//! There is no fixture-count or source-length ceiling imposed by this module.
//! Static fixtures are borrowed without copying. Dynamically generated test
//! inputs are owned by the caller and are limited by available memory and
//! the caller's explicit test/resource budgets.
//!
//! A fixture generator must not be used to claim that a generated design
//! parses or validates unless the actual production parser and validator
//! have been run against it.
//!
//! ## Determinism
//!
//! Catalog order is declaration order. Identifiers are stable. Lookup is
//! deterministic. No environment, network, hardware, filesystem, clock,
//! or randomness influences catalog contents.
//!
//! ## Deferred work
//!
//! All implementation of `src/hdl/hdlir/` remains explicitly out of scope.
//! Fixtures may later be consumed by HDLIR conformance tests, but this file
//! must not depend on HDLIR implementation details.

/// Stable identifier for a source-level HDL test fixture.
///
/// IDs are intended for test names, diagnostic reports, and regression
/// references. They are not AST IDs, semantic IDs, or hardware identities.
pub type FixtureId = &'static str;

/// Broad purpose of a fixture.
///
/// These categories describe testing intent. They do not assert that a
/// particular parser or semantic implementation already supports the
/// associated feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FixtureCategory {
    /// Smallest source-level HDL example used to establish a baseline.
    Minimal,

    /// Module declarations, ports, and logical connectivity.
    Structure,

    /// Combinational behavior and assignment relationships.
    Combinational,

    /// Clocked behavior, registers, and state transitions.
    Sequential,

    /// Symbolic widths, parameters, and generic dimensions.
    Parameterized,

    /// Memory declarations and logical access behavior.
    Memory,

    /// Clock declarations and timing intent.
    Timing,

    /// Multiple clock domains and crossing analysis.
    ClockDomains,

    /// Hierarchical designs and reusable instances.
    Hierarchy,

    /// Verification-oriented source examples.
    Verification,

    /// Intentionally malformed source used for parser-negative tests.
    InvalidSyntax,

    /// Structurally parseable input intended to fail semantic validation.
    InvalidSemantics,

    /// Source whose realization may be infeasible for a selected target.
    TargetFeasibility,

    /// Large or generated source used for resource and scalability tests.
    Scalability,

    /// Imported HDL and source-interoperability scenarios.
    Interoperability,
}

/// The earliest subsystem a fixture is intended to exercise.
///
/// This field describes test intent only. It does not replace an observed
/// test result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FixtureStage {
    /// Lexing and parsing through the canonical frontend.
    Parsing,

    /// HDL source-level semantic validation.
    SemanticValidation,

    /// Target capability and resource feasibility.
    TargetFeasibility,

    /// Import or export conversion at an interoperability boundary.
    Interoperability,

    /// A stress test using a generated or unusually large input.
    Scalability,
}

/// Immutable source fixture and its testing metadata.
///
/// Fixture contents are static strings so catalog lookup does not allocate
/// or copy source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fixture {
    id: FixtureId,
    category: FixtureCategory,
    stage: FixtureStage,
    description: &'static str,
    source: &'static str,
}

impl Fixture {
    /// Construct a fixture for use in this module's catalog.
    ///
    /// The constructor is private so the checked-in catalog remains the
    /// authority for built-in fixture identifiers and ordering.
    const fn new(
        id: FixtureId,
        category: FixtureCategory,
        stage: FixtureStage,
        description: &'static str,
        source: &'static str,
    ) -> Self {
        Self {
            id,
            category,
            stage,
            description,
            source,
        }
    }

    /// Return the stable fixture identifier.
    #[must_use]
    pub const fn id(&self) -> FixtureId {
        self.id
    }

    /// Return the fixture category.
    #[must_use]
    pub const fn category(&self) -> FixtureCategory {
        self.category
    }

    /// Return the earliest intended test stage.
    #[must_use]
    pub const fn stage(&self) -> FixtureStage {
        self.stage
    }

    /// Return a short human-readable description.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Return the exact source text.
    ///
    /// The source is not normalized, parsed, or rewritten by this method.
    #[must_use]
    pub const fn source(&self) -> &'static str {
        self.source
    }
}

/// Canonical built-in HDL fixture catalog.
///
/// The declaration order is stable and is used by [`fixtures`] and
/// [`find_fixture`]. Keep IDs unique and do not reuse an existing ID for a
/// different test scenario.
///
/// IMPORTANT:
///
/// These are source-corpus entries with declared testing intent. They are
/// not a claim that the current grammar composition and parser integration
/// have been executed successfully against every entry. Before a fixture is
/// used as a parser-positive or semantic-positive regression, validate it
/// against the actual canonical grammar and frontend implementation.
///
/// The syntax in this initial catalog is deliberately conservative and
/// should be reconciled with the actual grammar during conformance integration.
static FIXTURES: &[Fixture] = &[
    Fixture::new(
        "hdl.minimal.empty",
        FixtureCategory::Minimal,
        FixtureStage::Parsing,
        "Minimal HDL source-unit baseline.",
        "",
    ),
    Fixture::new(
        "hdl.clock.basic",
        FixtureCategory::Timing,
        FixtureStage::Parsing,
        "Logical clock declaration using the grammar's documented example form.",
        r#"
clock clk {
    period = 10ns;
}
"#,
    ),
    Fixture::new(
        "hdl.port.input",
        FixtureCategory::Structure,
        FixtureStage::Parsing,
        "Input port declaration fragment from the documented port examples.",
        "input data: logic;",
    ),
    Fixture::new(
        "hdl.port.output",
        FixtureCategory::Structure,
        FixtureStage::Parsing,
        "Output port declaration fragment from the documented port examples.",
        "output output_data: logic;",
    ),
    Fixture::new(
        "hdl.interface.port-map",
        FixtureCategory::Structure,
        FixtureStage::Parsing,
        "Logical port association example from the port grammar documentation.",
        r#"
port_map {
    input_data = data;
    output_data = result;
}
"#,
    ),
    Fixture::new(
        "hdl.invalid.clock.missing-period-value",
        FixtureCategory::InvalidSyntax,
        FixtureStage::Parsing,
        "Malformed clock declaration for a parser-negative regression.",
        r#"
clock clk {
    period
}
"#,
    ),
    Fixture::new(
        "hdl.invalid.port.missing-type",
        FixtureCategory::InvalidSyntax,
        FixtureStage::Parsing,
        "Malformed port declaration for a parser-negative regression.",
        "input data:;",
    ),
    Fixture::new(
        "hdl.invalid.clock.unclosed-body",
        FixtureCategory::InvalidSyntax,
        FixtureStage::Parsing,
        "Unclosed clock declaration for delimiter and recovery tests.",
        r#"
clock clk {
    period = 10ns;
"#,
    ),
];

/// Borrow the built-in fixture catalog in stable declaration order.
///
/// This function performs no allocation.
#[must_use]
pub fn fixtures() -> &'static [Fixture] {
    FIXTURES
}

/// Find a built-in fixture by its stable identifier.
///
/// Matching is exact and case-sensitive. No normalization or fuzzy matching
/// is performed.
#[must_use]
pub fn find_fixture(id: &str) -> Option<&'static Fixture> {
    FIXTURES.iter().find(|fixture| fixture.id() == id)
}

/// Return all built-in fixtures belonging to a category.
///
/// The returned iterator borrows the catalog and does not allocate a new
/// collection.
pub fn fixtures_in_category(
    category: FixtureCategory,
) -> impl Iterator<Item = &'static Fixture> {
    FIXTURES
        .iter()
        .filter(move |fixture| fixture.category() == category)
}

/// Return all built-in fixtures intended for a test stage.
///
/// The returned iterator borrows the catalog and does not allocate a new
/// collection.
pub fn fixtures_for_stage(
    stage: FixtureStage,
) -> impl Iterator<Item = &'static Fixture> {
    FIXTURES
        .iter()
        .filter(move |fixture| fixture.stage() == stage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_is_not_empty() {
        assert!(!fixtures().is_empty());
    }

    #[test]
    fn fixture_ids_are_nonempty_and_unique() {
        for (index, fixture) in fixtures().iter().enumerate() {
            assert!(
                !fixture.id().trim().is_empty(),
                "fixture at index {index} has an empty ID"
            );

            assert!(
                !fixture.id().contains(char::is_whitespace),
                "fixture ID must not contain whitespace: {}",
                fixture.id()
            );

            assert!(
                fixtures()[..index]
                    .iter()
                    .all(|previous| previous.id() != fixture.id()),
                "duplicate fixture ID: {}",
                fixture.id()
            );
        }
    }

    #[test]
    fn fixture_descriptions_are_nonempty() {
        for fixture in fixtures() {
            assert!(
                !fixture.description().trim().is_empty(),
                "fixture {} has no description",
                fixture.id()
            );
        }
    }

    #[test]
    fn fixture_sources_are_preserved_exactly() {
        for fixture in fixtures() {
            // The source accessor must return the stored text without
            // normalization or modification.
            assert_eq!(fixture.source(), fixture.source());
        }
    }

    #[test]
    fn lookup_finds_each_catalog_entry() {
        for fixture in fixtures() {
            let found = find_fixture(fixture.id())
                .expect("every catalog entry must be findable");

            assert_eq!(found, fixture);
        }
    }

    #[test]
    fn lookup_returns_none_for_unknown_ids() {
        assert!(find_fixture("hdl.fixture.does-not-exist").is_none());
        assert!(find_fixture("").is_none());
    }

    #[test]
    fn category_filter_preserves_catalog_order() {
        let expected: Vec<_> = fixtures()
            .iter()
            .filter(|fixture| fixture.category() == FixtureCategory::Timing)
            .map(Fixture::id)
            .collect();

        let actual: Vec<_> = fixtures_in_category(FixtureCategory::Timing)
            .map(Fixture::id)
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn stage_filter_preserves_catalog_order() {
        let expected: Vec<_> = fixtures()
            .iter()
            .filter(|fixture| fixture.stage() == FixtureStage::Parsing)
            .map(Fixture::id)
            .collect();

        let actual: Vec<_> = fixtures_for_stage(FixtureStage::Parsing)
            .map(Fixture::id)
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn malformed_syntax_fixtures_are_available() {
        let invalid: Vec<_> =
            fixtures_in_category(FixtureCategory::InvalidSyntax).collect();

        assert!(
            !invalid.is_empty(),
            "the catalog should contain parser-negative scenarios"
        );

        assert!(invalid.iter().all(|fixture| {
            fixture.stage() == FixtureStage::Parsing
        }));
    }

    #[test]
    fn catalog_order_is_stable() {
        let first: Vec<_> = fixtures().iter().map(Fixture::id).collect();
        let second: Vec<_> = fixtures().iter().map(Fixture::id).collect();

        assert_eq!(first, second);
    }

    #[test]
    fn empty_source_is_distinct_from_missing_fixture() {
        let fixture = find_fixture("hdl.minimal.empty")
            .expect("the minimal fixture must exist");

        assert_eq!(fixture.source(), "");
        assert!(find_fixture("hdl.fixture.missing").is_none());
    }
}
