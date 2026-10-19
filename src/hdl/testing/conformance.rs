
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Conformance Testing
//!
//! Path: `src/hdl/testing/conformance.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide a reusable, deterministic conformance framework for checking HDL
//! syntax, semantic validation, portability contracts, and analysis
//! completeness against the authoritative Zamani grammar and specification.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - The delegate grammars under `grammar/hdl/`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - The canonical frontend AST and diagnostic contracts.
//!
//! The grammar owns syntax. The existing frontend parser owns parsing.
//! The existing semantic validator owns the HDL semantic rules it implements.
//! This module owns test cases, conformance expectations, result evaluation,
//! and regression tests. It does not redefine those authorities.
//!
//! ## Does not own
//!
//! - A second HDL parser or lexer.
//! - A replacement AST or semantic model.
//! - HDLIR data structures, builders, passes, or lowering.
//! - Synthesis, placement, routing, or physical hardware allocation.
//! - Target discovery or assumptions about target capacities.
//! - A claim that every feature described by the specification is already
//!   implemented.
//!
//! All HDLIR implementation remains exclusively under
//! `src/hdl/hdlir/` and is outside this file's scope.
//!
//! ## Integration
//!
//! The conformance runner accepts an implementation adapter. The adapter
//! must invoke the repository's actual parser and semantic pipeline rather
//! than a substitute implementation.
//!
//! The semantic regression tests in this file invoke
//! `crate::frontend::semantic::domains::hdl::validate_hdl` directly.
//!
//! The parser adapter is deliberately an interface: the existing
//! `HdlDomainParser` consumes canonical lexer tokens, so source-to-token
//! construction must use the actual frontend lexer contract. This file does
//! not invent lexer constructors or AST conversion APIs.
//!
//! ## Scalability
//!
//! - No fixed maximum number of conformance cases.
//! - No fixed maximum source length or HDL design size.
//! - No fixed maximum signal count or bit width.
//! - No hardware-capacity assumptions.
//! - Operational budgets remain the responsibility of the parser and
//!   semantic analysis configuration.
//! - Incomplete analysis never counts as a successful conformance result.
//!
//! Memory and execution time remain bounded by available resources and
//! explicitly configured policies.
//!
//! ## Determinism
//!
//! Cases retain their supplied order. Each case has a stable identifier.
//! Evaluation does not use randomness, wall-clock time, or hardware discovery.
//! Adapters are responsible for deterministic diagnostics and for avoiding
//! hidden environmental dependencies.
//!
//! ## Completion contract
//!
//! This file is complete when:
//!
//! 1. Its semantic regression tests pass.
//! 2. Its grammar/specification integrity tests pass.
//! 3. The real frontend implements the adapter contract.
//! 4. The conformance corpus includes positive, negative, boundary,
//!    symbolic-width, and incomplete-analysis cases.
//! 5. Parser and semantic results preserve source diagnostics.
//! 6. No incomplete or unsupported analysis is silently accepted.
//! 7. The module is registered by the HDL testing module.
//! 8. Rust 1.97+ formatting, compilation, tests, and Clippy pass.

use std::fmt;

// -----------------------------------------------------------------------------
// Stable conformance model
// -----------------------------------------------------------------------------

/// The stage at which a conformance case is evaluated.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConformanceStage {
    /// Validate that source text is accepted or rejected by the HDL parser.
    Parsing,

    /// Validate source-level HDL semantics.
    SemanticValidation,

    /// Validate a property that requires a complete analysis result.
    AnalysisCompleteness,

    /// Validate repository grammar and specification integrity.
    SpecificationIntegrity,
}

/// The expected outcome for a conformance case.
///
/// An expectation is intentionally distinct from the observed result:
/// unsupported syntax or incomplete analysis must not be treated as success
/// merely because the implementation cannot decide the case.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConformanceExpectation {
    /// The tested operation must succeed completely.
    Accept,

    /// The tested operation must reject the input.
    Reject,

    /// The tested operation must explicitly report incomplete analysis.
    Incomplete,
}

/// The result observed from a conformance adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConformanceObservation {
    /// The tested operation completed successfully.
    Accepted,

    /// The tested operation rejected the input.
    Rejected {
        /// Stable diagnostic codes, when available.
        diagnostic_codes: Vec<String>,
    },

    /// The tested operation could not establish a complete result.
    Incomplete {
        /// Explanation of why the result is inconclusive.
        reason: String,
    },

    /// The operation failed before producing a trustworthy conformance
    /// result, for example because of an infrastructure or configuration
    /// error.
    Failed {
        /// Explanation of the operational failure.
        reason: String,
    },
}

impl ConformanceObservation {
    /// Return whether the observation is a complete successful result.
    #[must_use]
    pub const fn is_accepted(&self) -> bool {
        matches!(self, Self::Accepted)
    }

    /// Return whether the observation is explicitly inconclusive.
    #[must_use]
    pub const fn is_incomplete(&self) -> bool {
        matches!(self, Self::Incomplete { .. })
    }

    /// Return whether the observation represents an operational failure.
    #[must_use]
    pub const fn is_failed(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }
}

/// A single reproducible HDL conformance case.
///
/// `id` must be unique within a suite. It should remain stable when the case
/// is edited so that reports and CI results can refer to it reliably.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlConformanceCase {
    /// Stable identifier for this case.
    pub id: String,

    /// Conformance stage being tested.
    pub stage: ConformanceStage,

    /// Expected outcome.
    pub expectation: ConformanceExpectation,

    /// Source text, if the case exercises HDL source.
    ///
    /// Specification-integrity cases may omit source text.
    pub source: Option<String>,

    /// Human-readable explanation of the behavior being tested.
    pub rationale: String,
}

impl HdlConformanceCase {
    /// Construct a source-based conformance case.
    ///
    /// Empty identifiers are rejected. Source text is preserved exactly;
    /// the constructor does not normalize whitespace or alter source meaning.
    pub fn source(
        id: impl Into<String>,
        stage: ConformanceStage,
        expectation: ConformanceExpectation,
        source: impl Into<String>,
        rationale: impl Into<String>,
    ) -> Result<Self, ConformanceDefinitionError> {
        let id = id.into();

        if id.trim().is_empty() {
            return Err(ConformanceDefinitionError::EmptyId);
        }

        if stage == ConformanceStage::SpecificationIntegrity {
            return Err(ConformanceDefinitionError::InvalidStage {
                id,
                reason: "specification-integrity cases do not accept source cases",
            });
        }

        Ok(Self {
            id,
            stage,
            expectation,
            source: Some(source.into()),
            rationale: rationale.into(),
        })
    }

    /// Construct a specification-integrity conformance case.
    pub fn specification(
        id: impl Into<String>,
        expectation: ConformanceExpectation,
        rationale: impl Into<String>,
    ) -> Result<Self, ConformanceDefinitionError> {
        let id = id.into();

        if id.trim().is_empty() {
            return Err(ConformanceDefinitionError::EmptyId);
        }

        Ok(Self {
            id,
            stage: ConformanceStage::SpecificationIntegrity,
            expectation,
            source: None,
            rationale: rationale.into(),
        })
    }
}

/// Invalid conformance case definitions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConformanceDefinitionError {
    /// A case identifier is empty or whitespace-only.
    EmptyId,

    /// A case uses an incompatible stage or source representation.
    InvalidStage {
        /// Identifier of the invalid case.
        id: String,

        /// Explanation of the invalid definition.
        reason: &'static str,
    },

    /// Two cases in one suite have the same identifier.
    DuplicateId(String),

    /// A case has no source text although its stage requires it.
    MissingSource(String),
}

impl fmt::Display for ConformanceDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyId => {
                f.write_str("HDL conformance case identifier cannot be empty")
            }
            Self::InvalidStage { id, reason } => {
                write!(f, "invalid conformance case `{id}`: {reason}")
            }
            Self::DuplicateId(id) => {
                write!(f, "duplicate HDL conformance case identifier `{id}`")
            }
            Self::MissingSource(id) => {
                write!(f, "HDL conformance case `{id}` requires source text")
            }
        }
    }
}

impl std::error::Error for ConformanceDefinitionError {}

// -----------------------------------------------------------------------------
// Adapter and suite execution
// -----------------------------------------------------------------------------

/// Adapter to the real Zamani HDL frontend and semantic pipeline.
///
/// Implementations must delegate to the existing authoritative components.
/// They must not use a test-only parser or weaker semantic validator to
/// establish production conformance.
///
/// A new adapter implementation should map the actual parser and semantic
/// result types into [`ConformanceObservation`], preserving diagnostic codes
/// and reporting aborted, unsupported, or incomplete analysis explicitly.
pub trait HdlConformanceAdapter {
    /// Evaluate one case using the actual implementation under test.
    fn evaluate(
        &self,
        case: &HdlConformanceCase,
    ) -> ConformanceObservation;
}

/// The conformance status of one case.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConformanceStatus {
    /// The observed result matches the expected outcome.
    Passed,

    /// The observed result differs from the expected outcome.
    Failed {
        /// Expected outcome.
        expected: ConformanceExpectation,

        /// Actual outcome.
        observed: ConformanceObservation,
    },
}

/// Result for one conformance case.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConformanceCaseResult {
    /// Stable case identifier.
    pub id: String,

    /// Stage tested.
    pub stage: ConformanceStage,

    /// Evaluation result.
    pub status: ConformanceStatus,
}

/// Aggregated results for a suite.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConformanceReport {
    /// Per-case results in the order supplied by the caller.
    pub cases: Vec<ConformanceCaseResult>,
}

impl ConformanceReport {
    /// Return the number of cases evaluated.
    #[must_use]
    pub fn total(&self) -> usize {
        self.cases.len()
    }

    /// Return the number of cases that passed.
    #[must_use]
    pub fn passed(&self) -> usize {
        self.cases
            .iter()
            .filter(|case| matches!(case.status, ConformanceStatus::Passed))
            .count()
    }

    /// Return the number of cases that failed.
    #[must_use]
    pub fn failed(&self) -> usize {
        self.cases
            .iter()
            .filter(|case| matches!(case.status, ConformanceStatus::Failed { .. }))
            .count()
    }

    /// Return whether every case passed.
    ///
    /// An empty suite is vacuously successful. Callers requiring a non-empty
    /// suite should enforce that policy when defining the test corpus.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.failed() == 0
    }

    /// Return the failed cases without changing their order.
    pub fn failures(&self) -> impl Iterator<Item = &ConformanceCaseResult> {
        self.cases
            .iter()
            .filter(|case| matches!(case.status, ConformanceStatus::Failed { .. }))
    }
}

/// Run a conformance suite through the supplied implementation adapter.
///
/// Duplicate identifiers are rejected before any case is evaluated, avoiding
/// ambiguous CI reports and partially executed suites.
///
/// A rejection is successful only when rejection was expected. An incomplete
/// result is successful only when incompleteness was expected explicitly.
pub fn run_conformance_suite<A: HdlConformanceAdapter>(
    adapter: &A,
    cases: &[HdlConformanceCase],
) -> Result<ConformanceReport, ConformanceDefinitionError> {
    use std::collections::BTreeSet;

    let mut identifiers = BTreeSet::new();

    for case in cases {
        if !identifiers.insert(case.id.as_str()) {
            return Err(ConformanceDefinitionError::DuplicateId(
                case.id.clone(),
            ));
        }

        if case.stage != ConformanceStage::SpecificationIntegrity
            && case.source.is_none()
        {
            return Err(ConformanceDefinitionError::MissingSource(
                case.id.clone(),
            ));
        }
    }

    let mut report = ConformanceReport {
        cases: Vec::with_capacity(cases.len()),
    };

    for case in cases {
        let observed = adapter.evaluate(case);

        let passed = matches!(
            (case.expectation, &observed),
            (ConformanceExpectation::Accept, ConformanceObservation::Accepted)
                | (ConformanceExpectation::Reject, ConformanceObservation::Rejected { .. })
                | (ConformanceExpectation::Incomplete, ConformanceObservation::Incomplete { .. })
        );

        let status = if passed {
            ConformanceStatus::Passed
        } else {
            ConformanceStatus::Failed {
                expected: case.expectation,
                observed,
            }
        };

        report.cases.push(ConformanceCaseResult {
            id: case.id.clone(),
            stage: case.stage,
            status,
        });
    }

    Ok(report)
}

// -----------------------------------------------------------------------------
// Semantic regression helpers
// -----------------------------------------------------------------------------

/// Run the repository's existing HDL semantic validator.
///
/// This convenience function intentionally accepts the validator's canonical
/// semantic input rather than parsing source text itself. Source parsing and
/// AST adaptation remain owned by the frontend.
pub fn validate_semantic_input(
    program: &crate::frontend::semantic::domains::hdl::HdlProgram,
) -> crate::frontend::semantic::domains::hdl::HdlValidationResult {
    crate::frontend::semantic::domains::hdl::validate_hdl(program)
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::semantic::domains::hdl::{
        HdlAssignment, HdlDirection, HdlId, HdlProcess, HdlProcessKind,
        HdlProgram, HdlSignal, HdlSignedness, HdlValueType, HdlWidth,
        HdlValidationStatus,
    };

    use std::path::Path;

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identifier must be valid")
    }

    fn signal(
        name: &str,
        direction: HdlDirection,
        value_type: HdlValueType,
    ) -> HdlSignal {
        HdlSignal {
            id: id(name),
            name: name.to_owned(),
            direction,
            value_type,
            location: None,
        }
    }

    #[test]
    fn empty_program_is_semantically_valid() {
        let result = validate_semantic_input(&HdlProgram::default());

        assert_eq!(result.status, HdlValidationStatus::Valid);
        assert!(result.complete);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn zero_width_is_rejected() {
        let program = HdlProgram {
            signals: vec![signal(
                "payload",
                HdlDirection::Internal,
                HdlValueType::BitVector {
                    width: HdlWidth::Known(0),
                    signedness: HdlSignedness::Unsigned,
                },
            )],
            ..HdlProgram::default()
        };

        let result = validate_semantic_input(&program);

        assert_eq!(result.status, HdlValidationStatus::Invalid);
        assert!(result.complete);
        assert!(!result.diagnostics.is_empty());
    }

    #[test]
    fn symbolic_width_is_preserved_without_a_fixed_width_ceiling() {
        let width = HdlWidth::Multiply(
            Box::new(HdlWidth::Symbol(id("lanes"))),
            Box::new(HdlWidth::Known(8)),
        );

        assert_eq!(width.as_known(), None);

        let program = HdlProgram {
            signals: vec![signal(
                "payload",
                HdlDirection::Internal,
                HdlValueType::BitVector {
                    width,
                    signedness: HdlSignedness::Unsigned,
                },
            )],
            ..HdlProgram::default()
        };

        let result = validate_semantic_input(&program);

        assert_eq!(result.status, HdlValidationStatus::Valid);
        assert!(result.complete);
    }

    #[test]
    fn process_cannot_drive_an_input_signal() {
        let program = HdlProgram {
            signals: vec![signal(
                "external",
                HdlDirection::Input,
                HdlValueType::Bit,
            )],
            processes: vec![HdlProcess {
                id: id("comb"),
                kind: HdlProcessKind::Combinational,
                assignments: vec![HdlAssignment {
                    target: id("external"),
                    reads: Vec::new(),
                    continuous: false,
                    location: None,
                }],
                clocks: Vec::new(),
                resets: Vec::new(),
                location: None,
            }],
            ..HdlProgram::default()
        };

        let result = validate_semantic_input(&program);

        assert_eq!(result.status, HdlValidationStatus::Invalid);
    }

    #[test]
    fn combinational_feedback_is_reported() {
        let program = HdlProgram {
            signals: vec![signal(
                "feedback",
                HdlDirection::Internal,
                HdlValueType::Bit,
            )],
            processes: vec![HdlProcess {
                id: id("comb"),
                kind: HdlProcessKind::Combinational,
                assignments: vec![HdlAssignment {
                    target: id("feedback"),
                    reads: vec![id("feedback")],
                    continuous: true,
                    location: None,
                }],
                clocks: Vec::new(),
                resets: Vec::new(),
                location: None,
            }],
            ..HdlProgram::default()
        };

        let result = validate_semantic_input(&program);

        assert_eq!(result.status, HdlValidationStatus::Invalid);
    }

    #[test]
    fn sequential_process_without_a_clock_is_rejected() {
        let program = HdlProgram {
            processes: vec![HdlProcess {
                id: id("seq"),
                kind: HdlProcessKind::Sequential,
                assignments: Vec::new(),
                clocks: Vec::new(),
                resets: Vec::new(),
                location: None,
            }],
            ..HdlProgram::default()
        };

        let result = validate_semantic_input(&program);

        assert_eq!(result.status, HdlValidationStatus::Invalid);
    }

    #[test]
    fn semantic_diagnostics_are_deterministic() {
        let program = HdlProgram {
            signals: vec![
                signal("z", HdlDirection::Internal, HdlValueType::Bit),
                signal("a", HdlDirection::Internal, HdlValueType::Bit),
            ],
            processes: vec![HdlProcess {
                id: id("p"),
                kind: HdlProcessKind::Combinational,
                assignments: vec![HdlAssignment {
                    target: id("missing"),
                    reads: vec![id("also_missing")],
                    continuous: false,
                    location: None,
                }],
                clocks: Vec::new(),
                resets: Vec::new(),
                location: None,
            }],
            ..HdlProgram::default()
        };

        let first = validate_semantic_input(&program);
        let second = validate_semantic_input(&program);

        assert_eq!(first, second);
    }

    #[test]
    fn incomplete_is_not_equivalent_to_acceptance() {
        let observation = ConformanceObservation::Incomplete {
            reason: "analysis budget exhausted".to_owned(),
        };

        assert!(observation.is_incomplete());
        assert!(!observation.is_accepted());
    }

    #[test]
    fn failed_operation_is_not_equivalent_to_rejection() {
        let observation = ConformanceObservation::Failed {
            reason: "frontend configuration error".to_owned(),
        };

        assert!(observation.is_failed());
        assert!(!observation.is_accepted());
    }

    #[test]
    fn duplicate_case_ids_are_rejected_before_execution() {
        struct MustNotRun;

        impl HdlConformanceAdapter for MustNotRun {
            fn evaluate(
                &self,
                _: &HdlConformanceCase,
            ) -> ConformanceObservation {
                panic!("adapter must not run for an invalid suite")
            }
        }

        let first = HdlConformanceCase::source(
            "same",
            ConformanceStage::Parsing,
            ConformanceExpectation::Accept,
            "hdl module Example {}",
            "first case",
        )
        .expect("valid case");

        let second = first.clone();

        let result = run_conformance_suite(&MustNotRun, &[first, second]);

        assert!(matches!(
            result,
            Err(ConformanceDefinitionError::DuplicateId(_))
        ));
    }

    #[test]
    fn unexpected_incomplete_result_fails_an_acceptance_case() {
        struct IncompleteAdapter;

        impl HdlConformanceAdapter for IncompleteAdapter {
            fn evaluate(
                &self,
                _: &HdlConformanceCase,
            ) -> ConformanceObservation {
                ConformanceObservation::Incomplete {
                    reason: "analysis did not finish".to_owned(),
                }
            }
        }

        let case = HdlConformanceCase::source(
            "complete-acceptance",
            ConformanceStage::SemanticValidation,
            ConformanceExpectation::Accept,
            "hdl module Example {}",
            "A successful case must complete analysis.",
        )
        .expect("valid case");

        let report = run_conformance_suite(&IncompleteAdapter, &[case])
            .expect("suite definition is valid");

        assert_eq!(report.total(), 1);
        assert_eq!(report.passed(), 0);
        assert_eq!(report.failed(), 1);
        assert!(!report.is_successful());
    }

    #[test]
    fn grammar_and_specification_files_exist() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let specification = root.join("grammar/spec/hdl.md");
        let composition = root.join("grammar/hdl/hdl.g4");

        assert!(
            specification.is_file(),
            "missing normative HDL specification: {}",
            specification.display()
        );

        assert!(
            composition.is_file(),
            "missing canonical HDL composition grammar: {}",
            composition.display()
        );

        let spec_text =
            std::fs::read_to_string(&specification).expect("read HDL specification");
        let grammar_text =
            std::fs::read_to_string(&composition).expect("read HDL grammar");

        assert!(
            !spec_text.trim().is_empty(),
            "HDL specification must not be empty"
        );

        assert!(
            !grammar_text.trim().is_empty(),
            "HDL composition grammar must not be empty"
        );

        assert!(
            grammar_text.contains("tokenVocab"),
            "HDL grammar must retain the canonical lexer contract"
        );
    }

    #[test]
    fn delegate_grammar_directory_contains_hdl_grammars() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let directory = root.join("grammar/hdl");

        let entries = std::fs::read_dir(&directory)
            .expect("HDL grammar directory must exist");

        let mut grammar_count = 0usize;

        for entry in entries {
            let entry = entry.expect("grammar directory entry must be readable");
            let path = entry.path();

            if path.extension().is_some_and(|extension| extension == "g4") {
                let text = std::fs::read_to_string(&path)
                    .expect("grammar files must be UTF-8 readable");

                assert!(
                    !text.trim().is_empty(),
                    "empty delegate grammar: {}",
                    path.display()
                );

                grammar_count += 1;
            }
        }

        assert!(
            grammar_count > 0,
            "the HDL delegate grammar directory must contain grammar files"
        );
    }
}
