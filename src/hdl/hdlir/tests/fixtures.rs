
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Test Fixtures
//!
//! Path: `src/hdl/hdlir/tests/fixtures.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide deterministic, reusable test fixtures for the HDL-domain
//! intermediate representation. This module distinguishes actual
//! constructible designs from scenario descriptions that require further
//! graph, dialect, verifier, or serialization integration.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its applicable delegate grammars
//! - `grammar/specification/semantic-model.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/compatibility.md`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ir/design.rs`
//! - `src/hdl/hdlir/ir/module.rs`
//! - `src/hdl/hdlir/serialization/`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar and frontend own source syntax and source-level meaning.
//! ZUIR owns the universal computational representation. HDLIR owns the
//! hardware-domain representation. These fixtures do not replace any of
//! those authorities.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Stable fixture identifiers and scenario metadata.
//! - A minimal constructible HDLIR design.
//! - Deterministic scenario lookup and iteration.
//! - Malformed artifact byte sequences for decoder rejection tests.
//! - Fixture-catalog integrity tests.
//!
//! This module does not own:
//!
//! - Source parsing, AST construction, or semantic validation.
//! - Alternative HDLIR design, type, value, or identity implementations.
//! - Hardware simulation, synthesis, target discovery, or realization.
//! - A claim that a scenario is implemented merely because it is catalogued.
//! - Replacement serialization or decoding logic.
//!
//! ## Integration
//!
//! Register this module from `src/hdl/hdlir/tests/mod.rs` after that
//! composition root exists. Its parent must itself be registered by the
//! HDLIR Rust module root.
//!
//! The minimal fixture uses the current public `HdlirDesign` and
//! `HdlirModule` constructors. Future fixtures must use the canonical
//! builders and graph APIs after their contracts have been verified.
//!
//! Decoder rejection tests call the existing `decode_artifact` function.
//! They do not implement an alternative artifact decoder.
//!
//! ## Scalability
//!
//! There is no fixture-count, module-count, source-size, signal-width,
//! or hardware-capacity ceiling introduced by this module.
//!
//! The minimal fixture uses small identifiers for readability. Those
//! identifiers are local test identities, not maximum values or reserved
//! production IDs. Each independently constructed fixture owns its own
//! logical identity scope.
//!
//! Large or generated test inputs must be bounded by the caller's explicit
//! test budget and available resources. A resource-exhausted test is not
//! evidence that the corresponding design is semantically invalid.
//!
//! ## Determinism
//!
//! Catalog order is declaration order. IDs and fixture identifiers are
//! stable. No randomness, wall-clock time, environment state, network
//! access, or hardware discovery affects fixture construction.
//!
//! ## Freeze criteria
//!
//! - Constructible fixtures use existing canonical HDLIR APIs.
//! - Planned scenarios are explicitly distinguished from real designs.
//! - Malformed artifacts are passed to the canonical decoder.
//! - Fixture identifiers are nonempty and unique.
//! - Catalog lookup and iteration agree.
//! - No unsafe code or arbitrary hardware maxima are introduced.
//! - Formatting, compilation, tests, and Clippy pass after integration.

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{HdlirDesignId, HdlirModuleId};
use crate::hdl::hdlir::ir::design::HdlirDesign;
use crate::hdl::hdlir::ir::module::HdlirModule;

/// Stable identifier for a built-in HDLIR fixture.
///
/// This is test metadata, not a source identifier, semantic ID, or hardware
/// resource identifier.
pub type FixtureId = &'static str;

/// Indicates whether a fixture can currently be constructed through the
/// verified public HDLIR API.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FixtureAvailability {
    /// The fixture has a concrete constructor using existing public APIs.
    Constructible,

    /// The scenario is catalogued, but the required graph construction or
    /// verification contract is not yet established for this fixture.
    ScenarioOnly,
}

/// Hardware-domain scenario represented by a fixture.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum FixtureCategory {
    /// Smallest complete design accepted by the current design API.
    Minimal,

    /// Logical modules and their interfaces.
    Structure,

    /// Combinational hardware behavior.
    Combinational,

    /// Clocked state and sequential behavior.
    Sequential,

    /// Symbolic parameter and width relationships.
    Parameterized,

    /// Abstract memory declarations and access behavior.
    Memory,

    /// Clock and reset relationships.
    Clocking,

    /// Multiple logical clock domains.
    ClockDomains,

    /// Reusable module hierarchy and instances.
    Hierarchy,

    /// Invalid graph references or structural invariants.
    InvalidGraph,

    /// Unknown, unsupported, or incompatible operation definitions.
    UnsupportedOperation,

    /// Serialization and malformed artifact handling.
    Serialization,

    /// Compatibility and artifact-version migration.
    Compatibility,

    /// Deterministic construction and canonical representation.
    Determinism,

    /// Large designs and explicitly budgeted workloads.
    Scalability,

    /// Interoperation with source-level semantic models or ZUIR.
    Interoperability,
}

/// Intended testing boundary for a fixture.
///
/// This identifies the principal subsystem to exercise; it does not claim
/// that the subsystem has passed a test.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FixtureStage {
    /// Design or graph construction.
    Construction,

    /// Whole-graph and local-invariant verification.
    Verification,

    /// Hardware behavior or state-preservation tests.
    Semantics,

    /// Lowering between established IR boundaries.
    Lowering,

    /// Artifact encoding, decoding, and compatibility.
    Serialization,

    /// Determinism and scalability.
    Stress,
}

/// Immutable metadata describing a fixture or planned scenario.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixtureSpec {
    id: FixtureId,
    category: FixtureCategory,
    stage: FixtureStage,
    availability: FixtureAvailability,
    description: &'static str,
}

impl FixtureSpec {
    /// Construct a fixture specification.
    ///
    /// This constructor is private so built-in fixture IDs and catalog
    /// ordering remain controlled by this module.
    const fn new(
        id: FixtureId,
        category: FixtureCategory,
        stage: FixtureStage,
        availability: FixtureAvailability,
        description: &'static str,
    ) -> Self {
        Self {
            id,
            category,
            stage,
            availability,
            description,
        }
    }

    /// Return the stable fixture identifier.
    #[must_use]
    pub const fn id(&self) -> FixtureId {
        self.id
    }

    /// Return the scenario category.
    #[must_use]
    pub const fn category(&self) -> FixtureCategory {
        self.category
    }

    /// Return the intended test stage.
    #[must_use]
    pub const fn stage(&self) -> FixtureStage {
        self.stage
    }

    /// Return whether the fixture can currently be constructed.
    #[must_use]
    pub const fn availability(&self) -> FixtureAvailability {
        self.availability
    }

    /// Return the human-readable scenario description.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }

    /// Return whether this specification has a concrete fixture constructor.
    #[must_use]
    pub const fn is_constructible(&self) -> bool {
        matches!(self.availability, FixtureAvailability::Constructible)
    }
}

/// The canonical catalog of HDLIR fixture scenarios.
///
/// Only entries marked `Constructible` are promised to have a concrete
/// fixture constructor in this module. Other entries document test coverage
/// that must be implemented as the relevant HDLIR APIs become available.
///
/// Adding an entry does not prove the corresponding feature works.
pub const FIXTURE_CATALOG: &[FixtureSpec] = &[
    FixtureSpec::new(
        "hdlir.minimal",
        FixtureCategory::Minimal,
        FixtureStage::Construction,
        FixtureAvailability::Constructible,
        "A complete design with one empty logical module and an entry module.",
    ),
    FixtureSpec::new(
        "hdlir.structure.modules",
        FixtureCategory::Structure,
        FixtureStage::Verification,
        FixtureAvailability::ScenarioOnly,
        "Multiple modules with unique IDs and names and checked references.",
    ),
    FixtureSpec::new(
        "hdlir.combinational",
        FixtureCategory::Combinational,
        FixtureStage::Semantics,
        FixtureAvailability::ScenarioOnly,
        "Combinational operations, operands, results, and logical connectivity.",
    ),
    FixtureSpec::new(
        "hdlir.sequential",
        FixtureCategory::Sequential,
        FixtureStage::Semantics,
        FixtureAvailability::ScenarioOnly,
        "Stateful operations with explicitly defined clocked behavior.",
    ),
    FixtureSpec::new(
        "hdlir.parameterized",
        FixtureCategory::Parameterized,
        FixtureStage::Verification,
        FixtureAvailability::ScenarioOnly,
        "Symbolic widths, parameter bindings, and unresolved constraints.",
    ),
    FixtureSpec::new(
        "hdlir.memory",
        FixtureCategory::Memory,
        FixtureStage::Semantics,
        FixtureAvailability::ScenarioOnly,
        "Abstract memory ports, read/write behavior, and collision semantics.",
    ),
    FixtureSpec::new(
        "hdlir.clocking",
        FixtureCategory::Clocking,
        FixtureStage::Verification,
        FixtureAvailability::ScenarioOnly,
        "Clock edges, reset behavior, and logical clock relationships.",
    ),
    FixtureSpec::new(
        "hdlir.clock_domains",
        FixtureCategory::ClockDomains,
        FixtureStage::Verification,
        FixtureAvailability::ScenarioOnly,
        "Multiple clock domains and explicit domain-crossing facts.",
    ),
    FixtureSpec::new(
        "hdlir.hierarchy",
        FixtureCategory::Hierarchy,
        FixtureStage::Verification,
        FixtureAvailability::ScenarioOnly,
        "Reusable module definitions, instances, and port bindings.",
    ),
    FixtureSpec::new(
        "hdlir.invalid_graph",
        FixtureCategory::InvalidGraph,
        FixtureStage::Verification,
        FixtureAvailability::ScenarioOnly,
        "Dangling references, duplicate definitions, and invalid connectivity.",
    ),
    FixtureSpec::new(
        "hdlir.unsupported_operation",
        FixtureCategory::UnsupportedOperation,
        FixtureStage::Verification,
        FixtureAvailability::ScenarioOnly,
        "An operation whose required schema or semantics are unavailable.",
    ),
    FixtureSpec::new(
        "hdlir.serialization",
        FixtureCategory::Serialization,
        FixtureStage::Serialization,
        FixtureAvailability::ScenarioOnly,
        "Deterministic encoding and decoding of a verified design artifact.",
    ),
    FixtureSpec::new(
        "hdlir.compatibility",
        FixtureCategory::Compatibility,
        FixtureStage::Serialization,
        FixtureAvailability::ScenarioOnly,
        "Compatible, incompatible, and explicitly migrated artifact versions.",
    ),
    FixtureSpec::new(
        "hdlir.determinism",
        FixtureCategory::Determinism,
        FixtureStage::Stress,
        FixtureAvailability::ScenarioOnly,
        "Equivalent inputs produce equivalent canonical representations.",
    ),
    FixtureSpec::new(
        "hdlir.scalability",
        FixtureCategory::Scalability,
        FixtureStage::Stress,
        FixtureAvailability::ScenarioOnly,
        "Progressively larger designs and explicit operational-budget outcomes.",
    ),
    FixtureSpec::new(
        "hdlir.interoperability",
        FixtureCategory::Interoperability,
        FixtureStage::Lowering,
        FixtureAvailability::ScenarioOnly,
        "Semantics-preserving conversion across supported IR boundaries.",
    ),
];

/// Return the canonical fixture catalog in deterministic declaration order.
#[must_use]
pub fn fixtures() -> &'static [FixtureSpec] {
    FIXTURE_CATALOG
}

/// Find a fixture by its exact, stable identifier.
#[must_use]
pub fn find_fixture(id: &str) -> Option<&'static FixtureSpec> {
    FIXTURE_CATALOG.iter().find(|fixture| fixture.id() == id)
}

/// Return fixture specifications belonging to a category.
///
/// The returned iterator preserves catalog order and does not allocate.
pub fn fixtures_in_category(
    category: FixtureCategory,
) -> impl Iterator<Item = &'static FixtureSpec> {
    FIXTURE_CATALOG
        .iter()
        .filter(move |fixture| fixture.category() == category)
}

/// Return fixture specifications intended for a particular test stage.
///
/// The returned iterator preserves catalog order and does not allocate.
pub fn fixtures_for_stage(
    stage: FixtureStage,
) -> impl Iterator<Item = &'static FixtureSpec> {
    FIXTURE_CATALOG
        .iter()
        .filter(move |fixture| fixture.stage() == stage)
}

/// Construct the minimal complete HDLIR design using the current public API.
///
/// The result contains one empty logical module designated as the entry
/// module. This is a valid baseline for design-registry tests; it does not
/// claim that the design contains ports, operations, behavior, or a
/// synthesizable circuit.
///
/// IDs are deliberately small for readable tests. They are local to this
/// fixture and impose no limits on production identity allocation.
///
/// # Errors
///
/// Returns the canonical HDLIR error if construction or entry registration
/// fails.
pub fn minimal_design() -> HdlirResult<HdlirDesign<HdlirModule>> {
    let design_id = HdlirDesignId::new(1).ok_or_else(|| {
        HdlirError::invalid_input(
            "fixture_design_id",
            "the minimal fixture requires a non-zero design identity",
        )
    })?;

    let module_id = HdlirModuleId::new(1).ok_or_else(|| {
        HdlirError::invalid_input(
            "fixture_module_id",
            "the minimal fixture requires a non-zero module identity",
        )
    })?;

    let mut design = HdlirDesign::new(design_id, "fixture.minimal")?;
    let module = HdlirModule::new(module_id, "top")?;

    design.insert_module(module)?;
    design.set_entry_module(module_id)?;
    design.validate_complete()?;

    Ok(design)
}

/// Stable identifier for a malformed artifact test case.
pub type MalformedArtifactId = &'static str;

/// A malformed serialized artifact intended for the canonical decoder.
///
/// The fixture records the bytes and the expectation that decoding must
/// reject them. It does not prescribe a particular decoder error message,
/// which may evolve while the structured error contract remains compatible.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MalformedArtifactFixture {
    id: MalformedArtifactId,
    bytes: &'static [u8],
    description: &'static str,
}

impl MalformedArtifactFixture {
    const fn new(
        id: MalformedArtifactId,
        bytes: &'static [u8],
        description: &'static str,
    ) -> Self {
        Self {
            id,
            bytes,
            description,
        }
    }

    /// Return the stable malformed-artifact identifier.
    #[must_use]
    pub const fn id(&self) -> MalformedArtifactId {
        self.id
    }

    /// Return the exact artifact bytes.
    #[must_use]
    pub const fn bytes(&self) -> &'static [u8] {
        self.bytes
    }

    /// Return the reason this input is expected to be rejected.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        self.description
    }
}

/// Artifact with no bytes.
pub const EMPTY_ARTIFACT: MalformedArtifactFixture =
    MalformedArtifactFixture::new(
        "artifact.empty",
        b"",
        "An empty input cannot contain a complete HDLIR artifact.",
    );

/// Artifact with a truncated magic prefix.
pub const TRUNCATED_MAGIC_ARTIFACT: MalformedArtifactFixture =
    MalformedArtifactFixture::new(
        "artifact.truncated_magic",
        b"ZHD",
        "The artifact magic prefix is truncated.",
    );

/// Artifact containing the correct magic but no framing length fields.
pub const TRUNCATED_FRAME_ARTIFACT: MalformedArtifactFixture =
    MalformedArtifactFixture::new(
        "artifact.truncated_frame",
        b"ZHDLIR\r\n",
        "The magic is present but the fixed framing prefix is incomplete.",
    );

/// Artifact with an invalid magic prefix and otherwise present framing bytes.
pub const INVALID_MAGIC_ARTIFACT: MalformedArtifactFixture =
    MalformedArtifactFixture::new(
        "artifact.invalid_magic",
        b"NOTHDLIR000000000000000000000000",
        "The artifact does not start with the required HDLIR magic prefix.",
    );

/// Canonical malformed-artifact catalog in deterministic declaration order.
pub const MALFORMED_ARTIFACT_FIXTURES: &[MalformedArtifactFixture] = &[
    EMPTY_ARTIFACT,
    TRUNCATED_MAGIC_ARTIFACT,
    TRUNCATED_FRAME_ARTIFACT,
    INVALID_MAGIC_ARTIFACT,
];

/// Return malformed-artifact fixtures in deterministic order.
#[must_use]
pub fn malformed_artifact_fixtures() -> &'static [MalformedArtifactFixture] {
    MALFORMED_ARTIFACT_FIXTURES
}

/// Find a malformed-artifact fixture by its exact identifier.
#[must_use]
pub fn find_malformed_artifact(
    id: &str,
) -> Option<&'static MalformedArtifactFixture> {
    MALFORMED_ARTIFACT_FIXTURES
        .iter()
        .find(|fixture| fixture.id() == id)
}

#[cfg(test)]
mod tests {
    use super::{
        find_fixture, find_malformed_artifact, fixtures, fixtures_for_stage,
        fixtures_in_category, malformed_artifact_fixtures, minimal_design,
        FixtureAvailability, FixtureCategory, FixtureStage,
    };
    use crate::hdl::hdlir::serialization::decode::{
        decode_artifact, DecodeOptions,
    };

    #[test]
    fn minimal_design_is_complete_and_has_one_entry_module() {
        let design = minimal_design()
            .expect("the canonical minimal HDLIR fixture should construct");

        assert_eq!(design.module_count(), 1);
        assert!(!design.is_empty());
        assert!(design.entry_module_id().is_some());
        assert!(design.validate_complete().is_ok());
    }

    #[test]
    fn minimal_design_lookup_matches_its_entry_module() {
        let design = minimal_design()
            .expect("the canonical minimal HDLIR fixture should construct");

        let entry_id = design
            .entry_module_id()
            .expect("the minimal design must designate an entry module");

        let entry = design
            .module(entry_id)
            .expect("the entry module must be registered");

        assert_eq!(entry.name_str(), "top");
    }

    #[test]
    fn fixture_ids_are_nonempty_and_unique() {
        let catalog = fixtures();

        for (index, fixture) in catalog.iter().enumerate() {
            assert!(!fixture.id().trim().is_empty());

            assert!(
                !catalog[..index]
                    .iter()
                    .any(|previous| previous.id() == fixture.id()),
                "duplicate fixture ID: {}",
                fixture.id()
            );
        }
    }

    #[test]
    fn fixture_lookup_agrees_with_catalog_iteration() {
        for fixture in fixtures() {
            let found = find_fixture(fixture.id())
                .expect("every catalog fixture must be discoverable");

            assert_eq!(found, fixture);
        }
    }

    #[test]
    fn fixture_filters_preserve_catalog_membership() {
        for fixture in fixtures() {
            assert!(
                fixtures_in_category(fixture.category())
                    .any(|candidate| candidate.id() == fixture.id())
            );

            assert!(
                fixtures_for_stage(fixture.stage())
                    .any(|candidate| candidate.id() == fixture.id())
            );
        }
    }

    #[test]
    fn only_the_minimal_fixture_is_claimed_constructible() {
        let constructible: Vec<_> = fixtures()
            .iter()
            .filter(|fixture| {
                fixture.availability() == FixtureAvailability::Constructible
            })
            .map(|fixture| fixture.id())
            .collect();

        assert_eq!(constructible, ["hdlir.minimal"]);

        assert_eq!(
            fixtures()
                .iter()
                .filter(|fixture| {
                    fixture.category() == FixtureCategory::Minimal
                })
                .count(),
            1
        );
    }

    #[test]
    fn fixture_catalog_includes_required_domain_scenarios() {
        let required = [
            "hdlir.combinational",
            "hdlir.sequential",
            "hdlir.parameterized",
            "hdlir.memory",
            "hdlir.clock_domains",
            "hdlir.hierarchy",
            "hdlir.invalid_graph",
            "hdlir.unsupported_operation",
            "hdlir.serialization",
            "hdlir.compatibility",
            "hdlir.determinism",
            "hdlir.scalability",
            "hdlir.interoperability",
        ];

        for id in required {
            assert!(
                find_fixture(id).is_some(),
                "required scenario is absent: {id}"
            );
        }
    }

    #[test]
    fn malformed_artifact_ids_are_nonempty_and_unique() {
        let catalog = malformed_artifact_fixtures();

        for (index, fixture) in catalog.iter().enumerate() {
            assert!(!fixture.id().trim().is_empty());

            assert!(
                !catalog[..index]
                    .iter()
                    .any(|previous| previous.id() == fixture.id()),
                "duplicate malformed-artifact fixture ID: {}",
                fixture.id()
            );
        }
    }

    #[test]
    fn malformed_artifact_lookup_is_consistent() {
        for fixture in malformed_artifact_fixtures() {
            let found = find_malformed_artifact(fixture.id())
                .expect("every malformed-artifact fixture must be discoverable");

            assert_eq!(found, fixture);
        }
    }

    #[test]
    fn canonical_decoder_rejects_malformed_artifact_fixtures() {
        let options = DecodeOptions::default();

        for fixture in malformed_artifact_fixtures() {
            let result = decode_artifact(fixture.bytes(), &options);

            assert!(
                result.is_err(),
                "decoder unexpectedly accepted malformed fixture `{}`: {}",
                fixture.id(),
                fixture.description()
            );
        }
    }

    #[test]
    fn malformed_fixtures_are_not_empty_as_a_catalog() {
        assert!(!malformed_artifact_fixtures().is_empty());
    }

    #[test]
    fn stress_stage_contains_scalability_scenario() {
        assert!(
            fixtures_for_stage(FixtureStage::Stress)
                .any(|fixture| fixture.id() == "hdlir.scalability")
        );
    }
}
