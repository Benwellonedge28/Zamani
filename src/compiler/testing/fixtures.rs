
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Meta-Compiler — Testing Fixtures
//!
//! Path:
//!     src/compiler/testing/fixtures.rs
//!
//! Rust:
//!     Edition 2021
//!     Minimum supported version: Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!     Unsafe Rust is forbidden.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Provides a reusable, deterministic and extensible catalog of compiler
//! test fixtures.
//!
//! A fixture is a named, independently identifiable test input together
//! with its expected compilation disposition and test metadata.
//!
//! Fixtures allow the compiler testing infrastructure to exercise:
//!
//! - Empty and minimal programs.
//! - Classical computation.
//! - Quantum computation.
//! - AI and tensor computation.
//! - HDL and hardware computation.
//! - Hybrid computation.
//! - Distributed computation.
//! - Networking.
//! - Cryptographic computation.
//! - Future computing domains.
//! - Invalid source programs.
//! - Regression cases.
//! - Portability requirements.
//! - Determinism.
//! - Compilation-stage behavior.
//! - Resource-sensitive compilation.
//!
//! This module stores fixtures and their expectations.
//!
//! It does not implement parsing, semantic analysis, IR generation,
//! optimization, lowering, backend compilation or execution.
//!
//! ============================================================================
//! ARCHITECTURAL OWNERSHIP
//! ============================================================================
//!
//! THIS MODULE OWNS:
//!
//! - Fixture identifiers.
//! - Fixture metadata.
//! - Fixture source text.
//! - Fixture categories.
//! - Expected fixture outcomes.
//! - Fixture-domain identifiers.
//! - Fixture tags.
//! - Required-stage metadata.
//! - Deterministic fixture registration.
//! - Fixture lookup.
//! - Fixture filtering.
//! - Built-in generic compiler fixtures.
//! - Fixture catalog validation.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Zamani grammar.
//! - Lexer implementation.
//! - Parser implementation.
//! - AST implementation.
//! - Semantic type definitions.
//! - Semantic validation.
//! - Ownership analysis.
//! - Effect analysis.
//! - Capability analysis.
//! - Resource requirement semantics.
//! - ZUIR.
//! - Classical IR.
//! - Quantum IR.
//! - HDL IR.
//! - AI or tensor IR.
//! - Hybrid execution semantics.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Physical resource allocation.
//! - Scheduling or routing.
//! - Backend implementations.
//! - Runtime execution.
//!
//! Those responsibilities remain with their authoritative subsystems.
//!
//! ============================================================================
//! CANONICAL REPOSITORY INTEGRATION
//! ============================================================================
//!
//! Grammar:
//!
//!     grammar/Zamani.g4
//!     grammar/antlr/ZamaniParser.g4
//!     grammar/antlr/ZamaniLexer.g4
//!     grammar/core/program.g4
//!     grammar/core/source-unit.g4
//!     grammar/compile/compilation.g4
//!
//! Frontend:
//!
//!     src/lexer.rs
//!     src/parser.rs
//!     src/frontend/ast/
//!     src/frontend/semantic/
//!
//! Compiler:
//!
//!     src/compiler/analysis/
//!     src/compiler/configuration/
//!     src/compiler/context/
//!     src/compiler/pipeline/
//!     src/compiler/passes/
//!     src/compiler/ir/
//!     src/compiler/planning/
//!     src/compiler/lowering/
//!     src/compiler/backends/
//!     src/compiler/artifacts/
//!     src/compiler/optimization/
//!     src/compiler/incremental/
//!     src/compiler/parallel/
//!
//! Canonical IR:
//!
//!     src/zuir/
//!     src/classical/
//!     src/quantum/ir/
//!     src/hdl/hdlir/
//!
//! Runtime and backends:
//!
//!     src/backend/
//!     src/runtime/
//!     src/distributed/
//!
//! This module references none of those implementation details directly.
//! It provides reusable test data to their respective test harnesses.
//!
//! ============================================================================
//! POCO-REAF AND SCALABILITY
//! ============================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! This module introduces no universal maximum on:
//!
//! - Number of fixtures.
//! - Number of source files.
//! - Source length.
//! - Number of computing domains.
//! - Number of fixture categories.
//! - Number of fixture tags.
//! - Number of compilation stages.
//! - Number of required capabilities.
//! - Number of test cases.
//! - Number of hardware devices.
//! - Number of CPUs.
//! - Number of GPUs.
//! - Number of FPGAs.
//! - Number of QPUs.
//! - Number of distributed nodes.
//! - Number of qubits.
//! - Tensor rank.
//! - Tensor dimensions.
//! - IR operations.
//! - Compilation units.
//!
//! All collections grow according to available resources.
//!
//! No arbitrary MAX_* hardware or fixture-count constants are introduced.
//!
//! A finite computer cannot provide literally infinite memory or processing
//! capacity.
//!
//! This module therefore supports workloads as large as the available
//! implementation resources permit.
//!
//! Operational resource budgets are enforced by the compiler execution
//! infrastructure, not by fixture identity or category definitions.
//!
//! Resource exhaustion must not be interpreted as successful test execution.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Fixture registration and iteration use ordered collections.
//!
//! Fixture identifiers are stable strings.
//!
//! Identical fixture registrations produce identical catalog ordering.
//!
//! Duplicate fixture identifiers are rejected.
//!
//! No randomness, timestamps, global mutable registries or implicit
//! environment-dependent behavior are introduced.
//!
//! ============================================================================
//! EXPECTATION CONTRACT
//! ============================================================================
//!
//! A fixture may expect:
//!
//! - Acceptance.
//! - Rejection.
//! - Incomplete analysis.
//! - A target-dependent result.
//!
//! Incomplete analysis is never equivalent to successful compilation.
//!
//! A target-dependent fixture must not be classified as universally
//! executable.
//!
//! Expected outcomes describe test assertions, not guarantees that a
//! particular backend or compiler stage currently implements the feature.
//!
//! ============================================================================
//! SOURCE AUTHORITY
//! ============================================================================
//!
//! Fixture source must follow the actual Zamani grammar.
//!
//! The grammar hierarchy remains authoritative.
//!
//! This module must not invent another source-language syntax.
//!
//! Domain-specific fixtures must be added using syntax supported by their
//! authoritative grammar and the corresponding frontend implementation.
//!
//! A fixture's presence in this catalog does not establish that the current
//! compiler implements the associated feature.
//!
//! ============================================================================

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version for the fixture catalog.
///
/// Independent of the Zamani language, grammar, AST, IR and artifact versions.
pub const FIXTURE_CATALOG_API_VERSION: u16 = 1;

// ============================================================================
// VALIDATION
// ============================================================================

/// Validates a fixture metadata string.
///
/// Empty and whitespace-only values are rejected.
///
/// Control characters are rejected.
///
/// No arbitrary string-length limit is imposed.
fn validate_label(
    value: &str,
    field: &'static str,
) -> Result<(), FixtureError> {
    if value.trim().is_empty() {
        return Err(FixtureError::EmptyField { field });
    }

    if value.chars().any(char::is_control) {
        return Err(FixtureError::ControlCharacter { field });
    }

    Ok(())
}

// ============================================================================
// FIXTURE IDENTIFIER
// ============================================================================

/// Stable, globally unique identifier for a compiler fixture.
///
/// Identifiers are case-sensitive and namespaced.
///
/// Examples:
///
/// - `core.empty_source`
/// - `core.minimal_function`
/// - `classical.scalar_arithmetic`
/// - `quantum.measurement`
/// - `ai.tensor_computation`
/// - `hdl.hardware_description`
/// - `hybrid.classical_quantum`
/// - `distributed.execution`
///
/// These examples do not constitute a closed registry.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
pub struct FixtureId(String);

impl FixtureId {
    /// Constructs a validated fixture identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, FixtureError> {
        let value = value.into();

        validate_label(&value, "fixture identifier")?;

        Ok(Self(value))
    }

    /// Returns the fixture identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for FixtureId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// FIXTURE CATEGORY
// ============================================================================

/// Identifies the testing purpose of a fixture.
///
/// This enum describes testing methodology, not computing domains.
///
/// Computing domains are represented by extensible string identifiers.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash
)]
#[non_exhaustive]
pub enum FixtureCategory {
    /// Basic compiler smoke tests.
    Smoke,

    /// Language and compiler conformance tests.
    Conformance,

    /// Tests reproducing previously identified defects.
    Regression,

    /// Inputs expected to be rejected.
    Negative,

    /// Tests involving large or resource-intensive inputs.
    Stress,

    /// Tests of reproducible compiler behavior.
    Determinism,

    /// Tests of target-independent program meaning.
    Portability,

    /// Tests involving compiler security boundaries.
    Security,

    /// Tests of extension and registration mechanisms.
    Extensibility,
}

// ============================================================================
// EXPECTED OUTCOME
// ============================================================================

/// Expected disposition of a compiler fixture.
///
/// The disposition describes what the test harness should assert.
///
/// It does not claim that the current implementation already passes the test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExpectedDisposition {
    /// The applicable compilation stages should accept the input.
    Accept,

    /// The applicable compilation stage should reject the input.
    Reject,

    /// The applicable analysis must report incomplete verification.
    ///
    /// Incomplete analysis must never be treated as success.
    Incomplete,

    /// The result depends on the capabilities and requirements of a
    /// particular target or execution environment.
    TargetDependent,
}

/// Complete expected-result contract for one fixture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureExpectation {
    disposition: ExpectedDisposition,
    stage: Option<String>,
    rationale: Option<String>,
}

impl FixtureExpectation {
    /// Creates an acceptance expectation.
    #[must_use]
    pub fn accept() -> Self {
        Self {
            disposition: ExpectedDisposition::Accept,
            stage: None,
            rationale: None,
        }
    }

    /// Creates a rejection expectation.
    ///
    /// `stage` identifies the expected rejecting stage, such as
    /// `frontend.parse` or `semantic.validation`.
    pub fn reject(
        stage: impl Into<String>,
        rationale: impl Into<String>,
    ) -> Result<Self, FixtureError> {
        Self::with_stage(
            ExpectedDisposition::Reject,
            stage,
            rationale,
        )
    }

    /// Creates an incomplete-analysis expectation.
    ///
    /// Incomplete verification must not be promoted to success.
    pub fn incomplete(
        stage: impl Into<String>,
        rationale: impl Into<String>,
    ) -> Result<Self, FixtureError> {
        Self::with_stage(
            ExpectedDisposition::Incomplete,
            stage,
            rationale,
        )
    }

    /// Creates a target-dependent expectation.
    ///
    /// The rationale should identify the relevant capability or
    /// compatibility condition.
    pub fn target_dependent(
        rationale: impl Into<String>,
    ) -> Result<Self, FixtureError> {
        let rationale = rationale.into();

        validate_label(&rationale, "expectation rationale")?;

        Ok(Self {
            disposition: ExpectedDisposition::TargetDependent,
            stage: None,
            rationale: Some(rationale),
        })
    }

    fn with_stage(
        disposition: ExpectedDisposition,
        stage: impl Into<String>,
        rationale: impl Into<String>,
    ) -> Result<Self, FixtureError> {
        let stage = stage.into();
        let rationale = rationale.into();

        validate_label(&stage, "expected stage")?;
        validate_label(&rationale, "expectation rationale")?;

        Ok(Self {
            disposition,
            stage: Some(stage),
            rationale: Some(rationale),
        })
    }

    /// Returns the expected disposition.
    #[must_use]
    pub const fn disposition(&self) -> ExpectedDisposition {
        self.disposition
    }

    /// Returns the expected stage, when applicable.
    #[must_use]
    pub fn stage(&self) -> Option<&str> {
        self.stage.as_deref()
    }

    /// Returns the explanation of the expected result.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }
}

// ============================================================================
// FIXTURE ERRORS
// ============================================================================

/// Errors produced by fixture construction and catalog operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FixtureError {
    /// A required metadata field is empty.
    EmptyField {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A metadata field contains control characters.
    ControlCharacter {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A fixture with the same identifier already exists.
    DuplicateFixture {
        /// Conflicting fixture identifier.
        id: FixtureId,
    },
}

impl fmt::Display for FixtureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(formatter, "{field} cannot be empty")
            }

            Self::ControlCharacter { field } => {
                write!(
                    formatter,
                    "{field} cannot contain control characters"
                )
            }

            Self::DuplicateFixture { id } => {
                write!(formatter, "duplicate compiler fixture: {id}")
            }
        }
    }
}

impl Error for FixtureError {}

/// Result type for fixture operations.
pub type FixtureResult<T> = Result<T, FixtureError>;

// ============================================================================
// FIXTURE DEFINITION
// ============================================================================

/// A complete, independently identifiable compiler test fixture.
///
/// Fixture source is stored as owned text.
///
/// No parser, AST, semantic model, IR or backend implementation is owned
/// by this structure.
///
/// Domains, tags and required stages use extensible identifiers rather
/// than closed hardware or computing-domain enumerations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fixture {
    id: FixtureId,
    name: String,
    description: String,
    source: String,
    category: FixtureCategory,
    expectation: FixtureExpectation,
    domains: BTreeSet<String>,
    tags: BTreeSet<String>,
    required_stages: BTreeSet<String>,
}

impl Fixture {
    /// Creates a new fixture.
    ///
    /// The source may be empty when testing the grammar's valid empty
    /// source-unit case.
    ///
    /// No arbitrary source-size limit is imposed.
    pub fn new(
        id: FixtureId,
        name: impl Into<String>,
        description: impl Into<String>,
        source: impl Into<String>,
        category: FixtureCategory,
        expectation: FixtureExpectation,
    ) -> FixtureResult<Self> {
        let name = name.into();
        let description = description.into();

        validate_label(&name, "fixture name")?;
        validate_label(&description, "fixture description")?;

        Ok(Self {
            id,
            name,
            description,
            source: source.into(),
            category,
            expectation,
            domains: BTreeSet::new(),
            tags: BTreeSet::new(),
            required_stages: BTreeSet::new(),
        })
    }

    /// Adds an extensible computing-domain identifier.
    ///
    /// Examples include `classical`, `quantum`, `ai`, `hdl`,
    /// `hybrid`, `distributed` and future domain identifiers.
    pub fn with_domain(
        mut self,
        domain: impl Into<String>,
    ) -> FixtureResult<Self> {
        let domain = domain.into();

        validate_label(&domain, "fixture domain")?;

        self.domains.insert(domain);

        Ok(self)
    }

    /// Adds a descriptive fixture tag.
    pub fn with_tag(
        mut self,
        tag: impl Into<String>,
    ) -> FixtureResult<Self> {
        let tag = tag.into();

        validate_label(&tag, "fixture tag")?;

        self.tags.insert(tag);

        Ok(self)
    }

    /// Adds a required compiler-stage identifier.
    ///
    /// Stage identifiers must correspond to the identifiers used by
    /// the actual compiler pipeline.
    ///
    /// This module does not create another pipeline-stage registry.
    pub fn with_required_stage(
        mut self,
        stage: impl Into<String>,
    ) -> FixtureResult<Self> {
        let stage = stage.into();

        validate_label(&stage, "required stage")?;

        self.required_stages.insert(stage);

        Ok(self)
    }

    /// Returns the fixture identifier.
    #[must_use]
    pub fn id(&self) -> &FixtureId {
        &self.id
    }

    /// Returns the fixture name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the fixture description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the original source text.
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Returns the fixture category.
    #[must_use]
    pub const fn category(&self) -> FixtureCategory {
        self.category
    }

    /// Returns the expected compilation disposition.
    #[must_use]
    pub fn expectation(&self) -> &FixtureExpectation {
        &self.expectation
    }

    /// Returns all associated computing domains.
    #[must_use]
    pub fn domains(&self) -> &BTreeSet<String> {
        &self.domains
    }

    /// Returns all fixture tags.
    #[must_use]
    pub fn tags(&self) -> &BTreeSet<String> {
        &self.tags
    }

    /// Returns all required compiler stages.
    #[must_use]
    pub fn required_stages(&self) -> &BTreeSet<String> {
        &self.required_stages
    }

    /// Tests whether this fixture belongs to a particular domain.
    #[must_use]
    pub fn belongs_to_domain(&self, domain: &str) -> bool {
        self.domains.contains(domain)
    }

    /// Tests whether this fixture has a particular tag.
    #[must_use]
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }
}

// ============================================================================
// FIXTURE SELECTOR
// ============================================================================

/// Optional filtering criteria for fixture selection.
///
/// Multiple domains are interpreted as an OR condition.
///
/// Categories and tags are additional filters.
///
/// An empty selector matches every fixture.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FixtureSelector {
    categories: BTreeSet<FixtureCategory>,
    domains: BTreeSet<String>,
    tags: BTreeSet<String>,
}

impl FixtureSelector {
    /// Creates an empty selector.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a category filter.
    #[must_use]
    pub fn with_category(mut self, category: FixtureCategory) -> Self {
        self.categories.insert(category);
        self
    }

    /// Adds a domain filter.
    pub fn with_domain(
        mut self,
        domain: impl Into<String>,
    ) -> FixtureResult<Self> {
        let domain = domain.into();

        validate_label(&domain, "selector domain")?;

        self.domains.insert(domain);

        Ok(self)
    }

    /// Adds a tag filter.
    pub fn with_tag(
        mut self,
        tag: impl Into<String>,
    ) -> FixtureResult<Self> {
        let tag = tag.into();

        validate_label(&tag, "selector tag")?;

        self.tags.insert(tag);

        Ok(self)
    }

    fn matches(&self, fixture: &Fixture) -> bool {
        let category_matches = self.categories.is_empty()
            || self.categories.contains(&fixture.category);

        let domain_matches = self.domains.is_empty()
            || self.domains.iter().any(|domain| {
                fixture.domains.contains(domain)
            });

        let tags_match = self.tags.is_empty()
            || self.tags.iter().all(|tag| {
                fixture.tags.contains(tag)
            });

        category_matches && domain_matches && tags_match
    }
}

// ============================================================================
// FIXTURE CATALOG
// ============================================================================

/// Deterministic registry of compiler test fixtures.
///
/// The catalog owns fixture definitions and their identifiers.
///
/// It does not execute fixtures.
///
/// The BTreeMap provides stable identifier-based iteration and lookup.
///
/// There is no fixed maximum fixture count.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FixtureCatalog {
    fixtures: BTreeMap<FixtureId, Fixture>,
}

impl FixtureCatalog {
    /// Creates an empty fixture catalog.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a catalog containing the built-in core fixtures.
    ///
    /// Domain-specific fixtures may subsequently be registered by
    /// their respective test modules.
    pub fn with_builtins() -> FixtureResult<Self> {
        let mut catalog = Self::new();

        for fixture in builtin_fixtures()? {
            catalog.register(fixture)?;
        }

        Ok(catalog)
    }

    /// Registers a fixture.
    ///
    /// Duplicate identifiers are rejected.
    ///
    /// Existing fixtures are never silently overwritten.
    pub fn register(&mut self, fixture: Fixture) -> FixtureResult<()> {
        if self.fixtures.contains_key(fixture.id()) {
            return Err(FixtureError::DuplicateFixture {
                id: fixture.id().clone(),
            });
        }

        self.fixtures.insert(
            fixture.id().clone(),
            fixture,
        );

        Ok(())
    }

    /// Returns a fixture by its complete identifier.
    #[must_use]
    pub fn get(&self, id: &FixtureId) -> Option<&Fixture> {
        self.fixtures.get(id)
    }

    /// Returns a fixture by its identifier string.
    #[must_use]
    pub fn get_by_name(&self, id: &str) -> Option<&Fixture> {
        self.fixtures
            .iter()
            .find(|(fixture_id, _)| fixture_id.as_str() == id)
            .map(|(_, fixture)| fixture)
    }

    /// Returns whether a fixture identifier is registered.
    #[must_use]
    pub fn contains(&self, id: &FixtureId) -> bool {
        self.fixtures.contains_key(id)
    }

    /// Returns the number of registered fixtures.
    #[must_use]
    pub fn len(&self) -> usize {
        self.fixtures.len()
    }

    /// Returns whether the catalog contains no fixtures.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fixtures.is_empty()
    }

    /// Returns fixtures in deterministic identifier order.
    pub fn iter(&self) -> impl Iterator<Item = &Fixture> {
        self.fixtures.values()
    }

    /// Returns fixtures matching the supplied selector.
    ///
    /// Results retain deterministic identifier ordering.
    pub fn select<'a>(
        &'a self,
        selector: &'a FixtureSelector,
    ) -> impl Iterator<Item = &'a Fixture> + 'a {
        self.fixtures
            .values()
            .filter(move |fixture| selector.matches(fixture))
    }

    /// Returns fixtures belonging to a computing domain.
    pub fn by_domain(
        &self,
        domain: &str,
    ) -> impl Iterator<Item = &Fixture> {
        self.fixtures
            .values()
            .filter(move |fixture| fixture.belongs_to_domain(domain))
    }

    /// Returns fixtures belonging to a category.
    pub fn by_category(
        &self,
        category: FixtureCategory,
    ) -> impl Iterator<Item = &Fixture> {
        self.fixtures
            .values()
            .filter(move |fixture| fixture.category == category)
    }

    /// Returns fixtures carrying a particular tag.
    pub fn by_tag(
        &self,
        tag: &str,
    ) -> impl Iterator<Item = &Fixture> {
        self.fixtures
            .values()
            .filter(move |fixture| fixture.has_tag(tag))
    }

    /// Returns the fixture catalog API version.
    #[must_use]
    pub const fn api_version(&self) -> u16 {
        FIXTURE_CATALOG_API_VERSION
    }
}

// ============================================================================
// BUILT-IN FIXTURES
// ============================================================================

/// Constructs the built-in generic compiler fixtures.
///
/// These fixtures deliberately use basic source constructs instead of
/// claiming that every computing-domain grammar is already implemented
/// by every frontend parser.
///
/// Domain-specific conformance fixtures must be supplied by the
/// corresponding domain testing modules.
pub fn builtin_fixtures() -> FixtureResult<Vec<Fixture>> {
    let empty = Fixture::new(
        FixtureId::new("core.empty_source")?,
        "Empty source unit",
        "Checks the canonical source-unit boundary with no source elements.",
        "",
        FixtureCategory::Conformance,
        FixtureExpectation::accept(),
    )?
    .with_domain("core")?
    .with_tag("source-unit")?
    .with_tag("minimal")?
    .with_required_stage("frontend.parse")?;

    let minimal = Fixture::new(
        FixtureId::new("core.minimal_function")?,
        "Minimal function",
        "Checks basic function declaration and source parsing.",
        "fn main() {}",
        FixtureCategory::Smoke,
        FixtureExpectation::accept(),
    )?
    .with_domain("core")?
    .with_tag("smoke")?
    .with_tag("function")?
    .with_required_stage("frontend.parse")?;

    let arithmetic = Fixture::new(
        FixtureId::new("core.scalar_arithmetic")?,
        "Scalar arithmetic",
        "Checks a basic parameterized arithmetic function.",
        "fn add(left: i64, right: i64) -> i64 { left + right }",
        FixtureCategory::Conformance,
        FixtureExpectation::accept(),
    )?
    .with_domain("classical")?
    .with_tag("scalar")?
    .with_tag("arithmetic")?
    .with_tag("portability")?
    .with_required_stage("frontend.parse")?;

    let malformed = Fixture::new(
        FixtureId::new("core.unclosed_function")?,
        "Malformed function",
        "Checks rejection of incomplete function syntax.",
        "fn main( {",
        FixtureCategory::Negative,
        FixtureExpectation::reject(
            "frontend.parse",
            "The function declaration contains incomplete syntax.",
        )?,
    )?
    .with_domain("core")?
    .with_tag("invalid-source")?
    .with_required_stage("frontend.parse")?;

    let portable = Fixture::new(
        FixtureId::new("poco.portable_scalar")?,
        "Portable scalar computation",
        "Provides a basic target-independent computation for portability tests.",
        "fn increment(value: i64) -> i64 { value + 1 }",
        FixtureCategory::Portability,
        FixtureExpectation::accept(),
    )?
    .with_domain("classical")?
    .with_tag("poco-reaf")?
    .with_tag("target-independent")?
    .with_tag("portability")?
    .with_required_stage("frontend.parse")?;

    Ok(vec![
        empty,
        minimal,
        arithmetic,
        malformed,
        portable,
    ])
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_fixture(id: &str) -> Fixture {
        Fixture::new(
            FixtureId::new(id).expect("valid fixture ID"),
            "Sample fixture",
            "Fixture used by catalog unit tests.",
            "fn main() {}",
            FixtureCategory::Smoke,
            FixtureExpectation::accept(),
        )
        .expect("valid fixture")
    }

    #[test]
    fn fixture_identifier_rejects_empty_values() {
        assert!(FixtureId::new("").is_err());
        assert!(FixtureId::new("   ").is_err());
    }

    #[test]
    fn fixture_identifier_rejects_control_characters() {
        assert!(FixtureId::new("core\nfixture").is_err());
    }

    #[test]
    fn fixture_identifier_preserves_namespace() {
        let id = FixtureId::new("hybrid.classical_quantum")
            .expect("valid identifier");

        assert_eq!(
            id.as_str(),
            "hybrid.classical_quantum"
        );
    }

    #[test]
    fn fixture_source_is_preserved_exactly() {
        let source = "fn main() {\n    let value = 42;\n}";

        let fixture = Fixture::new(
            FixtureId::new("test.source").expect("valid ID"),
            "Source preservation",
            "Checks exact source preservation.",
            source,
            FixtureCategory::Conformance,
            FixtureExpectation::accept(),
        )
        .expect("valid fixture");

        assert_eq!(fixture.source(), source);
    }

    #[test]
    fn empty_source_is_supported() {
        let fixture = Fixture::new(
            FixtureId::new("test.empty").expect("valid ID"),
            "Empty source",
            "An empty source-unit fixture.",
            "",
            FixtureCategory::Conformance,
            FixtureExpectation::accept(),
        )
        .expect("valid fixture");

        assert!(fixture.source().is_empty());
    }

    #[test]
    fn duplicate_fixture_registration_is_rejected() {
        let mut catalog = FixtureCatalog::new();

        catalog
            .register(sample_fixture("core.example"))
            .expect("first registration");

        let result = catalog.register(
            sample_fixture("core.example")
        );

        assert!(matches!(
            result,
            Err(FixtureError::DuplicateFixture { .. })
        ));

        assert_eq!(catalog.len(), 1);
    }

    #[test]
    fn fixture_lookup_works() {
        let mut catalog = FixtureCatalog::new();

        catalog
            .register(sample_fixture("core.lookup"))
            .expect("registration");

        let fixture = catalog
            .get_by_name("core.lookup")
            .expect("fixture exists");

        assert_eq!(fixture.name(), "Sample fixture");
    }

    #[test]
    fn fixture_order_is_deterministic() {
        let mut catalog = FixtureCatalog::new();

        catalog
            .register(sample_fixture("core.z"))
            .expect("registration");

        catalog
            .register(sample_fixture("core.a"))
            .expect("registration");

        let identifiers: Vec<&str> = catalog
            .iter()
            .map(|fixture| fixture.id().as_str())
            .collect();

        assert_eq!(
            identifiers,
            vec!["core.a", "core.z"]
        );
    }

    #[test]
    fn fixture_domain_filtering_works() {
        let mut catalog = FixtureCatalog::new();

        let fixture = sample_fixture("quantum.example")
            .with_domain("quantum")
            .expect("domain");

        catalog.register(fixture).expect("registration");

        assert_eq!(
            catalog.by_domain("quantum").count(),
            1
        );

        assert_eq!(
            catalog.by_domain("classical").count(),
            0
        );
    }

    #[test]
    fn fixture_tag_filtering_works() {
        let mut catalog = FixtureCatalog::new();

        let fixture = sample_fixture("core.tagged")
            .with_tag("regression")
            .expect("tag");

        catalog.register(fixture).expect("registration");

        assert_eq!(
            catalog.by_tag("regression").count(),
            1
        );
    }

    #[test]
    fn selector_combines_category_domain_and_tags() {
        let mut catalog = FixtureCatalog::new();

        let fixture = sample_fixture("hybrid.example")
            .with_domain("hybrid")
            .expect("domain")
            .with_tag("portability")
            .expect("tag");

        catalog.register(fixture).expect("registration");

        let selector = FixtureSelector::new()
            .with_category(FixtureCategory::Smoke)
            .with_domain("hybrid")
            .expect("selector domain")
            .with_tag("portability")
            .expect("selector tag");

        assert_eq!(
            catalog.select(&selector).count(),
            1
        );
    }

    #[test]
    fn incomplete_is_distinct_from_acceptance() {
        let expectation = FixtureExpectation::incomplete(
            "ir.verification",
            "Required verification evidence is unavailable.",
        )
        .expect("expectation");

        assert_eq!(
            expectation.disposition(),
            ExpectedDisposition::Incomplete
        );

        assert_eq!(
            expectation.stage(),
            Some("ir.verification")
        );
    }

    #[test]
    fn target_dependent_expectations_preserve_rationale() {
        let expectation = FixtureExpectation::target_dependent(
            "The target must provide the required operation.",
        )
        .expect("expectation");

        assert_eq!(
            expectation.disposition(),
            ExpectedDisposition::TargetDependent
        );

        assert!(expectation.rationale().is_some());
    }

    #[test]
    fn builtin_catalog_is_valid() {
        let catalog = FixtureCatalog::with_builtins()
            .expect("built-in catalog");

        assert!(!catalog.is_empty());

        assert!(catalog.contains(
            &FixtureId::new("core.minimal_function")
                .expect("identifier")
        ));

        assert!(catalog.contains(
            &FixtureId::new("poco.portable_scalar")
                .expect("identifier")
        ));
    }

    #[test]
    fn builtin_fixture_order_is_stable() {
        let first = FixtureCatalog::with_builtins()
            .expect("first catalog");

        let second = FixtureCatalog::with_builtins()
            .expect("second catalog");

        let first_ids: Vec<_> = first
            .iter()
            .map(|fixture| fixture.id().clone())
            .collect();

        let second_ids: Vec<_> = second
            .iter()
            .map(|fixture| fixture.id().clone())
            .collect();

        assert_eq!(first_ids, second_ids);
    }

    #[test]
    fn catalog_api_version_is_available() {
        let catalog = FixtureCatalog::new();

        assert_eq!(
            catalog.api_version(),
            FIXTURE_CATALOG_API_VERSION
        );
    }
}
