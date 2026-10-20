
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # HDLIR Conformance Framework
//!
//! Path: `src/hdl/hdlir/tests/conformance.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define a deterministic, reusable conformance harness for the hardware-domain
//! intermediate representation (HDLIR).
//!
//! The harness deliberately does not assume that HDLIR builders, operations,
//! verifiers, serializers, or lowering APIs exist. An adapter must connect
//! conformance cases to the real implementation once those APIs are available.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its delegate grammars
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//!
//! The grammar and frontend own source syntax and source-level semantics.
//! ZUIR owns universal computational representation. HDLIR owns hardware-domain
//! representation and its own verification and transformations.
//!
//! This test framework owns neither language semantics nor an IR implementation.
//!
//! ## Important implementation boundary
//!
//! A conformance case describes a required behavior; it does not prove that
//! behavior is implemented. A test is meaningful only when its adapter invokes
//! the real implementation under test.
//!
//! The repository's current HDLIR module is not yet a usable implementation.
//! Therefore this file establishes the harness and tests repository contracts,
//! but does not claim that hardware graph construction, HDLIR verification,
//! serialization, lowering, or synthesis already conform.
//!
//! ## Scalability and determinism
//!
//! No universal hardware capacities, signal counts, widths, module counts,
//! instance counts, or operation counts are imposed here.
//!
//! Cases are processed in supplied order. IDs must be unique within a suite.
//! The framework has no global mutable state, random ordering, hardware
//! discovery, or hidden target assumptions.
//!
//! The caller remains responsible for operational budgets. An incomplete,
//! unsupported, or operationally failed evaluation cannot count as acceptance.
//!
//! ## Integration
//!
//! Register this module from `src/hdl/hdlir/tests/mod.rs` only when that module
//! exists, and register the tests module from the HDLIR composition root when
//! the root has been repaired into a valid Rust module.
//!
//! An HDLIR adapter should use the actual checked builder, verifier,
//! serialization, or lowering APIs. It must not substitute a mock result for
//! execution of those APIs.
//!
//! Repository contract tests use `CARGO_MANIFEST_DIR` to inspect the real
//! grammar and existing architecture files. They do not parse source code or
//! replace the canonical frontend.
//!
//! ## Freeze criteria
//!
//! - Case definitions reject empty and duplicate IDs.
//! - Observed outcomes are compared exactly with declared expectations.
//! - Incomplete and unsupported outcomes cannot pass acceptance cases.
//! - Adapter failures remain distinguishable from design rejection.
//! - Repository contract tests inspect the actual authority files.
//! - Future HDLIR tests use real implementations and explicit assertions.
//! - Formatting, compilation, tests, and Clippy pass on the supported toolchain.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

// -----------------------------------------------------------------------------
// Stable conformance model
// -----------------------------------------------------------------------------

/// The HDLIR subsystem or contract being tested.
///
/// Stages describe testing responsibility. They do not imply that the
/// corresponding production implementation currently exists.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ConformanceStage {
    /// Repository specifications and module ownership boundaries.
    RepositoryContract,

    /// Creation of a design or other HDLIR-owned object.
    Construction,

    /// Structural, type, connectivity, process, clock, or memory verification.
    Verification,

    /// Analysis results and explicit incomplete-analysis behavior.
    Analysis,

    /// Semantics-preserving transformations and pass contracts.
    Transformation,

    /// Deterministic artifact encoding and safe decoding.
    Serialization,

    /// Conversion across the established frontend or ZUIR boundary.
    Lowering,

    /// Stable canonical representation and fingerprint behavior.
    Determinism,

    /// Resource-aware operation on designs of varying size.
    Scalability,

    /// Schema, dialect, and serialized-artifact evolution.
    Compatibility,
}

/// The outcome a conformance case requires.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExpectedOutcome {
    /// The operation must complete successfully.
    Accept,

    /// The operation must reject the input or design as invalid.
    Reject,

    /// The operation must explicitly report that it could not complete.
    Incomplete,

    /// The operation must explicitly report an unsupported capability.
    Unsupported,
}

/// The result actually observed from the implementation under test.
///
/// `Rejected`, `Incomplete`, `Unsupported`, and `Failed` are intentionally
/// distinct. An invalid design is not the same as an unsupported operation,
/// an exhausted budget, or a broken test environment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Observation {
    /// The tested operation completed successfully.
    Accepted,

    /// The implementation rejected the input or design.
    Rejected {
        /// Stable diagnostic identifiers, when available.
        diagnostic_codes: Vec<String>,
    },

    /// The implementation could not finish the required analysis.
    Incomplete {
        /// A useful explanation of what prevented completion.
        reason: String,
    },

    /// The implementation explicitly does not support the requested feature.
    Unsupported {
        /// The unsupported feature or contract.
        feature: String,
    },

    /// The operation failed for an operational or infrastructure reason.
    Failed {
        /// A useful explanation of the failure.
        reason: String,
    },
}

impl Observation {
    /// Return `true` only when the operation completed successfully.
    #[must_use]
    pub const fn is_accepted(&self) -> bool {
        matches!(self, Self::Accepted)
    }

    /// Return `true` when the result is explicitly incomplete.
    #[must_use]
    pub const fn is_incomplete(&self) -> bool {
        matches!(self, Self::Incomplete { .. })
    }

    /// Return `true` when the requested feature is unsupported.
    #[must_use]
    pub const fn is_unsupported(&self) -> bool {
        matches!(self, Self::Unsupported { .. })
    }

    /// Return `true` when the evaluation itself failed operationally.
    #[must_use]
    pub const fn is_failed(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }

    /// Compare this observation with a case's expected outcome.
    ///
    /// Only the outcome category is compared. Diagnostic text and ordering
    /// are preserved for reporting and may be asserted separately by tests
    /// that require exact diagnostics.
    #[must_use]
    pub const fn matches(&self, expected: ExpectedOutcome) -> bool {
        matches!(
            (self, expected),
            (Self::Accepted, ExpectedOutcome::Accept)
                | (Self::Rejected { .. }, ExpectedOutcome::Reject)
                | (Self::Incomplete { .. }, ExpectedOutcome::Incomplete)
                | (Self::Unsupported { .. }, ExpectedOutcome::Unsupported)
        )
    }
}

/// A single HDLIR conformance case.
///
/// The case records an identifier and testing intent. Its adapter is
/// responsible for mapping that intent to actual implementation operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConformanceCase {
    /// Stable identifier for reports and regression tracking.
    pub id: String,

    /// Subsystem or contract being exercised.
    pub stage: ConformanceStage,

    /// Required outcome.
    pub expected: ExpectedOutcome,

    /// Stable scenario identifier understood by the selected adapter.
    ///
    /// This is metadata, not HDL source and not an IR serialization format.
    pub scenario: String,

    /// Human-readable explanation of the behavior under test.
    pub rationale: String,
}

impl ConformanceCase {
    /// Construct a conformance case after validating its required identifiers.
    ///
    /// The constructor does not claim that the scenario is implemented or
    /// that the expected outcome is correct. Those are verified by running
    /// the case against the authoritative implementation and specification.
    pub fn new(
        id: impl Into<String>,
        stage: ConformanceStage,
        expected: ExpectedOutcome,
        scenario: impl Into<String>,
        rationale: impl Into<String>,
    ) -> Result<Self, DefinitionError> {
        let id = id.into();
        let scenario = scenario.into();
        let rationale = rationale.into();

        if id.trim().is_empty() {
            return Err(DefinitionError::EmptyId);
        }

        if scenario.trim().is_empty() {
            return Err(DefinitionError::EmptyScenario { id });
        }

        if rationale.trim().is_empty() {
            return Err(DefinitionError::EmptyRationale { id });
        }

        Ok(Self {
            id,
            stage,
            expected,
            scenario,
            rationale,
        })
    }
}

/// Errors in a conformance suite definition.
///
/// These errors describe the test definition itself, not an HDLIR design.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DefinitionError {
    /// A case identifier was empty or whitespace-only.
    EmptyId,

    /// A scenario identifier was empty or whitespace-only.
    EmptyScenario {
        /// ID of the case containing the invalid scenario.
        id: String,
    },

    /// A case rationale was empty or whitespace-only.
    EmptyRationale {
        /// ID of the case containing the invalid rationale.
        id: String,
    },

    /// A case ID occurred more than once in the same suite.
    DuplicateId {
        /// Repeated stable case identifier.
        id: String,
    },
}

impl fmt::Display for DefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyId => f.write_str("conformance case ID cannot be empty"),
            Self::EmptyScenario { id } => {
                write!(f, "conformance case `{id}` has an empty scenario")
            }
            Self::EmptyRationale { id } => {
                write!(f, "conformance case `{id}` has an empty rationale")
            }
            Self::DuplicateId { id } => {
                write!(f, "duplicate conformance case ID `{id}`")
            }
        }
    }
}

impl Error for DefinitionError {}

// -----------------------------------------------------------------------------
// Adapter and execution contract
// -----------------------------------------------------------------------------

/// Adapter between this framework and a real HDLIR implementation.
///
/// Implementations must call the actual code under test. They must not return
/// predetermined observations merely to make the suite pass.
///
/// Adapters should preserve structured diagnostics, report incomplete work
/// explicitly, and avoid hidden dependencies on target hardware or host
/// machine capacity.
pub trait ConformanceAdapter {
    /// Evaluate one case using the actual implementation.
    fn evaluate(&self, case: &ConformanceCase) -> Observation;
}

/// The result of evaluating one conformance case.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaseResult {
    /// Stable case identifier.
    pub id: String,

    /// Stage being tested.
    pub stage: ConformanceStage,

    /// Required outcome.
    pub expected: ExpectedOutcome,

    /// Actual outcome returned by the adapter.
    pub observed: Observation,

    /// Whether the observed outcome matches the required outcome.
    pub passed: bool,
}

/// The result of running a complete conformance suite.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConformanceReport {
    results: Vec<CaseResult>,
}

impl ConformanceReport {
    /// Borrow all case results in the original suite order.
    #[must_use]
    pub fn results(&self) -> &[CaseResult] {
        &self.results
    }

    /// Return the number of evaluated cases.
    #[must_use]
    pub fn total(&self) -> usize {
        self.results.len()
    }

    /// Return the number of cases that matched their expected outcomes.
    #[must_use]
    pub fn passed(&self) -> usize {
        self.results.iter().filter(|result| result.passed).count()
    }

    /// Return the number of cases that did not match their expectations.
    #[must_use]
    pub fn failed(&self) -> usize {
        self.total() - self.passed()
    }

    /// Return `true` only when every case passed.
    ///
    /// An empty suite is not considered successful. This avoids treating
    /// missing conformance coverage as evidence of correctness.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.total() > 0 && self.failed() == 0
    }

    /// Borrow results whose evaluation was explicitly incomplete.
    pub fn incomplete_results(&self) -> impl Iterator<Item = &CaseResult> {
        self.results
            .iter()
            .filter(|result| result.observed.is_incomplete())
    }

    /// Borrow results that reported an unsupported capability.
    pub fn unsupported_results(&self) -> impl Iterator<Item = &CaseResult> {
        self.results
            .iter()
            .filter(|result| result.observed.is_unsupported())
    }

    /// Borrow results where the adapter or infrastructure failed.
    pub fn operational_failures(&self) -> impl Iterator<Item = &CaseResult> {
        self.results
            .iter()
            .filter(|result| result.observed.is_failed())
    }
}

/// Validate the suite definition and run every case in supplied order.
///
/// The suite is rejected before evaluation if an ID is duplicated. This
/// prevents ambiguous reports and prevents adapters from running part of an
/// invalidly defined suite.
///
/// There is no fixed case-count limit. Memory and runtime are bounded by
/// the supplied suite, the adapter, and available resources.
pub fn run_conformance_suite<A: ConformanceAdapter>(
    adapter: &A,
    cases: &[ConformanceCase],
) -> Result<ConformanceReport, DefinitionError> {
    let mut ids = BTreeSet::new();

    for case in cases {
        if !ids.insert(case.id.as_str()) {
            return Err(DefinitionError::DuplicateId {
                id: case.id.clone(),
            });
        }
    }

    let mut results = Vec::with_capacity(cases.len());

    for case in cases {
        let observed = adapter.evaluate(case);
        let passed = observed.matches(case.expected);

        results.push(CaseResult {
            id: case.id.clone(),
            stage: case.stage,
            expected: case.expected,
            observed,
            passed,
        });
    }

    Ok(ConformanceReport { results })
}

// -----------------------------------------------------------------------------
// Repository contract helpers
// -----------------------------------------------------------------------------

/// Return the repository root for contract tests.
///
/// Cargo defines `CARGO_MANIFEST_DIR` for the package that compiles this
/// module. No current working directory or machine-specific path is assumed.
fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Read a required repository file as UTF-8 text for contract tests.
///
/// Failures identify the exact path instead of silently skipping a missing
/// authority file.
fn read_required_text(root: &Path, relative_path: &str) -> String {
    let path = root.join(relative_path);
    std::fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "required repository contract file `{}` could not be read: {error}",
            path.display()
        )
    })
}

// -----------------------------------------------------------------------------
// Harness tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    struct FixedAdapter {
        observation: Observation,
    }

    impl ConformanceAdapter for FixedAdapter {
        fn evaluate(&self, _case: &ConformanceCase) -> Observation {
            self.observation.clone()
        }
    }

    fn case(
        id: &str,
        expected: ExpectedOutcome,
    ) -> ConformanceCase {
        ConformanceCase::new(
            id,
            ConformanceStage::Verification,
            expected,
            "test.scenario",
            "Exercise the conformance harness.",
        )
        .expect("test case definition must be valid")
    }

    #[test]
    fn case_constructor_rejects_empty_id() {
        let result = ConformanceCase::new(
            "  ",
            ConformanceStage::Construction,
            ExpectedOutcome::Accept,
            "design.minimal",
            "A valid rationale.",
        );

        assert_eq!(result, Err(DefinitionError::EmptyId));
    }

    #[test]
    fn case_constructor_rejects_empty_scenario() {
        let result = ConformanceCase::new(
            "design-minimal",
            ConformanceStage::Construction,
            ExpectedOutcome::Accept,
            " ",
            "A valid rationale.",
        );

        assert_eq!(
            result,
            Err(DefinitionError::EmptyScenario {
                id: "design-minimal".to_owned(),
            })
        );
    }

    #[test]
    fn case_constructor_rejects_empty_rationale() {
        let result = ConformanceCase::new(
            "design-minimal",
            ConformanceStage::Construction,
            ExpectedOutcome::Accept,
            "design.minimal",
            "",
        );

        assert_eq!(
            result,
            Err(DefinitionError::EmptyRationale {
                id: "design-minimal".to_owned(),
            })
        );
    }

    #[test]
    fn matching_observation_passes() {
        let adapter = FixedAdapter {
            observation: Observation::Accepted,
        };

        let report = run_conformance_suite(
            &adapter,
            &[case("accept-minimal-design", ExpectedOutcome::Accept)],
        )
        .expect("suite definition must be valid");

        assert_eq!(report.total(), 1);
        assert_eq!(report.passed(), 1);
        assert_eq!(report.failed(), 0);
        assert!(report.is_successful());
    }

    #[test]
    fn rejection_is_not_acceptance() {
        let adapter = FixedAdapter {
            observation: Observation::Rejected {
                diagnostic_codes: vec!["hdlir.invalid-design".to_owned()],
            },
        };

        let report = run_conformance_suite(
            &adapter,
            &[case("valid-design", ExpectedOutcome::Accept)],
        )
        .expect("suite definition must be valid");

        assert_eq!(report.passed(), 0);
        assert_eq!(report.failed(), 1);
        assert!(!report.is_successful());
    }

    #[test]
    fn incomplete_analysis_never_passes_acceptance_case() {
        let adapter = FixedAdapter {
            observation: Observation::Incomplete {
                reason: "verification budget exhausted".to_owned(),
            },
        };

        let report = run_conformance_suite(
            &adapter,
            &[case("complete-verification", ExpectedOutcome::Accept)],
        )
        .expect("suite definition must be valid");

        assert_eq!(report.failed(), 1);
        assert_eq!(report.incomplete_results().count(), 1);
        assert!(!report.is_successful());
    }

    #[test]
    fn unsupported_capability_is_not_success() {
        let adapter = FixedAdapter {
            observation: Observation::Unsupported {
                feature: "unknown required operation".to_owned(),
            },
        };

        let report = run_conformance_suite(
            &adapter,
            &[case("required-operation", ExpectedOutcome::Accept)],
        )
        .expect("suite definition must be valid");

        assert_eq!(report.failed(), 1);
        assert_eq!(report.unsupported_results().count(), 1);
    }

    #[test]
    fn expected_unsupported_outcome_can_pass_explicitly() {
        let adapter = FixedAdapter {
            observation: Observation::Unsupported {
                feature: "optional target extension".to_owned(),
            },
        };

        let report = run_conformance_suite(
            &adapter,
            &[case(
                "optional-target-extension",
                ExpectedOutcome::Unsupported,
            )],
        )
        .expect("suite definition must be valid");

        assert_eq!(report.passed(), 1);
        assert!(report.is_successful());
    }

    #[test]
    fn operational_failure_is_not_design_rejection() {
        let adapter = FixedAdapter {
            observation: Observation::Failed {
                reason: "test adapter configuration error".to_owned(),
            },
        };

        let report = run_conformance_suite(
            &adapter,
            &[case("adapter-health", ExpectedOutcome::Reject)],
        )
        .expect("suite definition must be valid");

        assert_eq!(report.failed(), 1);
        assert_eq!(report.operational_failures().count(), 1);
    }

    #[test]
    fn duplicate_ids_reject_the_suite_before_adapter_execution() {
        struct MustNotRun;

        impl ConformanceAdapter for MustNotRun {
            fn evaluate(&self, _case: &ConformanceCase) -> Observation {
                panic!("an invalid suite must be rejected before evaluation");
            }
        }

        let first = case("duplicate", ExpectedOutcome::Accept);
        let second = first.clone();

        let result = run_conformance_suite(&MustNotRun, &[first, second]);

        assert_eq!(
            result,
            Err(DefinitionError::DuplicateId {
                id: "duplicate".to_owned(),
            })
        );
    }

    #[test]
    fn suite_results_preserve_case_order() {
        struct AcceptAll;

        impl ConformanceAdapter for AcceptAll {
            fn evaluate(&self, _case: &ConformanceCase) -> Observation {
                Observation::Accepted
            }
        }

        let cases = [
            case("first", ExpectedOutcome::Accept),
            case("second", ExpectedOutcome::Accept),
            case("third", ExpectedOutcome::Accept),
        ];

        let report =
            run_conformance_suite(&AcceptAll, &cases).expect("valid suite");

        let ids: Vec<_> = report
            .results()
            .iter()
            .map(|result| result.id.as_str())
            .collect();

        assert_eq!(ids, ["first", "second", "third"]);
    }

    #[test]
    fn empty_suite_is_not_evidence_of_conformance() {
        struct UnusedAdapter;

        impl ConformanceAdapter for UnusedAdapter {
            fn evaluate(&self, _case: &ConformanceCase) -> Observation {
                panic!("no cases should be evaluated");
            }
        }

        let report = run_conformance_suite(&UnusedAdapter, &[])
            .expect("an empty suite is structurally valid");

        assert_eq!(report.total(), 0);
        assert!(!report.is_successful());
    }

    #[test]
    fn outcome_matching_is_exact_by_category() {
        assert!(Observation::Accepted.matches(ExpectedOutcome::Accept));

        assert!(
            Observation::Rejected {
                diagnostic_codes: Vec::new()
            }
            .matches(ExpectedOutcome::Reject)
        );

        assert!(
            Observation::Incomplete {
                reason: "budget".to_owned()
            }
            .matches(ExpectedOutcome::Incomplete)
        );

        assert!(
            Observation::Unsupported {
                feature: "extension".to_owned()
            }
            .matches(ExpectedOutcome::Unsupported)
        );

        assert!(
            !Observation::Failed {
                reason: "infrastructure".to_owned()
            }
            .matches(ExpectedOutcome::Reject)
        );
    }

    // -------------------------------------------------------------------------
    // Actual repository authority checks
    // -------------------------------------------------------------------------

    #[test]
    fn normative_hdl_specification_exists_and_is_nonempty() {
        let root = repository_root();
        let text = read_required_text(&root, "grammar/spec/hdl.md");

        assert!(
            !text.trim().is_empty(),
            "the normative HDL specification must not be empty"
        );
    }

    #[test]
    fn canonical_hdl_composition_grammar_exists_and_is_nonempty() {
        let root = repository_root();
        let text = read_required_text(&root, "grammar/hdl/hdl.g4");

        assert!(
            !text.trim().is_empty(),
            "the canonical HDL composition grammar must not be empty"
        );

        assert!(
            text.contains("tokenVocab"),
            "the HDL grammar must retain its canonical token-vocabulary contract"
        );
    }

    #[test]
    fn universal_ir_specification_exists_and_is_nonempty() {
        let root = repository_root();
        let text = read_required_text(&root, "src/zuir/ZUIR-SPEC.md");

        assert!(
            !text.trim().is_empty(),
            "the ZUIR specification must not be empty"
        );
    }

    #[test]
    fn existing_hdl_api_and_identity_contracts_exist() {
        let root = repository_root();

        for relative_path in ["src/hdl/api.rs", "src/hdl/ids.rs"] {
            let text = read_required_text(&root, relative_path);

            assert!(
                !text.trim().is_empty(),
                "repository contract must not be empty: {relative_path}"
            );
        }
    }

    #[test]
    fn parser_and_semantic_authorities_exist() {
        let root = repository_root();

        for relative_path in [
            "src/frontend/parser/domains/hdl.rs",
            "src/frontend/semantic/domains/hdl.rs",
        ] {
            let text = read_required_text(&root, relative_path);

            assert!(
                !text.trim().is_empty(),
                "canonical frontend authority must not be empty: {relative_path}"
            );
        }
    }

    #[test]
    fn hdl_composition_root_preserves_hdlir_ownership_boundary() {
        let root = repository_root();
        let text = read_required_text(&root, "src/hdl/mod.rs");

        assert!(
            text.contains("src/hdl/hdlir/"),
            "the composition root must identify the HDLIR ownership boundary"
        );

        assert!(
            text.contains("pub mod hdlir;"),
            "the composition root must retain the HDLIR module boundary"
        );

        assert!(
            text.contains("forbid(unsafe_code)"),
            "the HDL composition root must forbid unsafe Rust"
        );
    }

    #[test]
    fn repository_manifest_declares_supported_rust_baseline() {
        let root = repository_root();
        let manifest = read_required_text(&root, "Cargo.toml");

        assert!(
            manifest.lines().any(|line| {
                line.trim().strip_prefix("rust-version")
                    .is_some_and(|value| value.trim() == "= \"1.97\"" || value.trim() == "\"1.97\"")
            }),
            "Cargo.toml must declare the audited Rust baseline"
        );
    }
}
