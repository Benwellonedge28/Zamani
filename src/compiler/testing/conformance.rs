
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler Conformance Testing
//!
//! Path: src/compiler/testing/conformance.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Provides a production-oriented conformance execution framework for
//! testing the Zamani language and compiler against registered fixtures.
//!
//! The framework validates observable compiler behavior against explicit
//! fixture expectations.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Conformance execution contracts.
//! - Stage observations.
//! - Expected-versus-observed comparisons.
//! - Conformance results.
//! - Aggregate statistics.
//! - Streaming conformance execution.
//! - Executor error reporting.
//! - Integration with the existing fixture catalog.
//! - Integration with the public Zamani compilation API.
//!
//! This module does NOT own:
//!
//! - The Zamani grammar.
//! - Lexical rules.
//! - Parsing rules.
//! - AST definitions.
//! - Semantic types.
//! - Semantic analysis.
//! - Ownership analysis.
//! - Domain-specific IRs.
//! - IR verification.
//! - Optimization.
//! - Hardware discovery.
//! - Backend implementations.
//! - Runtime execution.
//!
//! Those remain with their authoritative repository subsystems.
//!
//! # Grammar authority
//!
//! The conformance architecture follows:
//!
//! grammar/specification/
//!          |
//!          v
//! grammar/spec/
//!          |
//!          v
//! grammar/Zamani.g4
//!          |
//!          v
//! grammar/antlr/
//!          |
//!          v
//! src/lexer.rs
//!          |
//!          v
//! src/parser.rs
//!          |
//!          v
//! src/ast/
//!          |
//!          v
//! semantic analysis
//!          |
//!          v
//! canonical IR
//!          |
//!          v
//! compiler and runtime
//!
//! The Rust parser and lexer are tested as implemented. An ANTLR grammar
//! production is not considered implemented merely because it exists.
//!
//! # POCO-REAF
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Conformance testing must distinguish:
//!
//! - Language conformance.
//! - Semantic conformance.
//! - IR conformance.
//! - Backend conformance.
//! - Runtime conformance.
//! - Cross-target portability.
//!
//! Passing frontend tests does not establish backend portability.
//!
//! # Scalability
//!
//! This module introduces no universal maximum for:
//!
//! - Fixtures.
//! - Source files.
//! - Source bytes.
//! - Computing domains.
//! - Compilation stages.
//! - Hardware targets.
//! - Test results.
//! - Required capabilities.
//! - IR operations.
//! - Test suites.
//!
//! Collections grow according to available resources.
//!
//! Streaming execution is provided to avoid requiring all test results
//! to remain resident in memory.
//!
//! Resource exhaustion is an execution failure, never a passing test.
//!
//! # Determinism
//!
//! Fixtures are consumed in the deterministic order provided by
//! FixtureCatalog.
//!
//! This module introduces no random test ordering, wall-clock-dependent
//! expectations, implicit target selection or global mutable state.

use core::fmt;
use std::error::Error;
use std::time::Duration;

use super::fixtures::{
    ExpectedDisposition,
    Fixture,
    FixtureCatalog,
    FixtureError,
    FixtureExpectation,
    FixtureSelector,
};

// ============================================================================
// API VERSION
// ============================================================================

/// Public conformance API version.
///
/// Independent of the Zamani language, grammar, IR and artifact versions.
pub const CONFORMANCE_API_VERSION: u16 = 1;

// ============================================================================
// CANONICAL STAGE IDENTIFIERS
// ============================================================================

/// Canonical stage identifiers for the currently integrated Rust frontend.
///
/// These identifiers describe the existing compiler integration.
/// They do not constitute a competing pipeline registry.
pub mod stage {
    /// Frontend parsing.
    pub const FRONTEND_PARSE: &str = "frontend.parse";

    /// Semantic validation.
    pub const SEMANTIC_VALIDATION: &str = "semantic.validation";

    /// IR generation.
    pub const IR_GENERATION: &str = "ir.generation";

    /// IR verification.
    pub const IR_VERIFICATION: &str = "ir.verification";

    /// Optimization.
    pub const OPTIMIZATION: &str = "optimization";

    /// Target lowering.
    pub const TARGET_LOWERING: &str = "target.lowering";

    /// Backend generation.
    pub const BACKEND_GENERATION: &str = "backend.generation";

    /// Runtime execution.
    pub const RUNTIME_EXECUTION: &str = "runtime.execution";
}

// ============================================================================
// OBSERVED DISPOSITION
// ============================================================================

/// Actual disposition reported by a conformance executor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ObservedDisposition {
    /// The tested operation completed successfully.
    Accepted,

    /// The tested operation rejected the input.
    Rejected,

    /// Analysis could not establish a complete result.
    Incomplete,

    /// The result depends on a target or execution capability.
    TargetDependent,

    /// The operation was not evaluated.
    NotEvaluated,
}

impl fmt::Display for ObservedDisposition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accepted => formatter.write_str("accepted"),
            Self::Rejected => formatter.write_str("rejected"),
            Self::Incomplete => formatter.write_str("incomplete"),
            Self::TargetDependent => {
                formatter.write_str("target-dependent")
            }
            Self::NotEvaluated => {
                formatter.write_str("not evaluated")
            }
        }
    }
}

// ============================================================================
// STAGE OBSERVATIONS
// ============================================================================

/// Actual status of an individual compilation stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum StageStatus {
    /// Stage completed successfully.
    Passed,

    /// Stage rejected the input.
    Failed,

    /// Stage could not complete verification.
    Incomplete,

    /// Stage was intentionally not executed.
    Skipped,
}

impl fmt::Display for StageStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Passed => formatter.write_str("passed"),
            Self::Failed => formatter.write_str("failed"),
            Self::Incomplete => formatter.write_str("incomplete"),
            Self::Skipped => formatter.write_str("skipped"),
        }
    }
}

/// Evidence produced by one compilation stage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageObservation {
    identifier: String,
    status: StageStatus,
    message: Option<String>,
    duration: Option<Duration>,
}

impl StageObservation {
    /// Creates a stage observation.
    pub fn new(
        identifier: impl Into<String>,
        status: StageStatus,
    ) -> Result<Self, ConformanceError> {
        let identifier = identifier.into();

        validate_identifier(&identifier, "stage identifier")?;

        Ok(Self {
            identifier,
            status,
            message: None,
            duration: None,
        })
    }

    /// Adds an explanatory message.
    #[must_use]
    pub fn with_message(
        mut self,
        message: impl Into<String>,
    ) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Adds measured stage duration.
    #[must_use]
    pub fn with_duration(
        mut self,
        duration: Duration,
    ) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Returns the stage identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the stage status.
    #[must_use]
    pub const fn status(&self) -> StageStatus {
        self.status
    }

    /// Returns the explanatory message.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Returns the measured duration.
    #[must_use]
    pub const fn duration(&self) -> Option<Duration> {
        self.duration
    }
}

// ============================================================================
// EXECUTION REPORT
// ============================================================================

/// Complete observation returned by a conformance executor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionObservation {
    disposition: ObservedDisposition,
    stages: Vec<StageObservation>,
    diagnostics: Vec<String>,
    duration: Option<Duration>,
}

impl ExecutionObservation {
    /// Creates an execution observation.
    #[must_use]
    pub fn new(disposition: ObservedDisposition) -> Self {
        Self {
            disposition,
            stages: Vec::new(),
            diagnostics: Vec::new(),
            duration: None,
        }
    }

    /// Adds a stage observation.
    #[must_use]
    pub fn with_stage(
        mut self,
        observation: StageObservation,
    ) -> Self {
        self.stages.push(observation);
        self
    }

    /// Adds a diagnostic.
    #[must_use]
    pub fn with_diagnostic(
        mut self,
        diagnostic: impl Into<String>,
    ) -> Self {
        self.diagnostics.push(diagnostic.into());
        self
    }

    /// Adds execution duration.
    #[must_use]
    pub fn with_duration(
        mut self,
        duration: Duration,
    ) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Returns the actual disposition.
    #[must_use]
    pub const fn disposition(&self) -> ObservedDisposition {
        self.disposition
    }

    /// Returns stage observations.
    #[must_use]
    pub fn stages(&self) -> &[StageObservation] {
        &self.stages
    }

    /// Returns execution diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[String] {
        &self.diagnostics
    }

    /// Returns execution duration.
    #[must_use]
    pub const fn duration(&self) -> Option<Duration> {
        self.duration
    }
}

// ============================================================================
// EXECUTOR ERRORS
// ============================================================================

/// Errors produced by a conformance executor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutorError {
    message: String,
}

impl ExecutorError {
    /// Creates an executor error.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Returns the error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ExecutorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for ExecutorError {}

// ============================================================================
// EXECUTOR CONTRACT
// ============================================================================

/// Executes a fixture against an actual compiler or validation subsystem.
///
/// Implementations may integrate:
///
/// - The Rust frontend.
/// - Semantic analysis.
/// - IR verification.
/// - Optimization.
/// - Backends.
/// - Runtime execution.
/// - External target providers.
///
/// Executors must report actual observations.
///
/// They must not fabricate successful stage observations.
///
/// The executor owns its own implementation and resource-management policy.
pub trait ConformanceExecutor {
    /// Executes one fixture and returns its actual observations.
    fn execute(
        &mut self,
        fixture: &Fixture,
    ) -> Result<ExecutionObservation, ExecutorError>;
}

// ============================================================================
// RESULT STATUS
// ============================================================================

/// Final conformance classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConformanceStatus {
    /// Actual behavior matched the expected behavior.
    Passed,

    /// Actual behavior violated an expectation.
    Failed,

    /// The test could not establish the required result.
    Skipped,
}

impl fmt::Display for ConformanceStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Passed => formatter.write_str("passed"),
            Self::Failed => formatter.write_str("failed"),
            Self::Skipped => formatter.write_str("skipped"),
        }
    }
}

// ============================================================================
// FAILURE REASONS
// ============================================================================

/// Structured reasons for conformance failures.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConformanceFailure {
    /// Expected and observed dispositions differed.
    DispositionMismatch {
        expected: ExpectedDisposition,
        observed: ObservedDisposition,
    },

    /// A required stage was not reported.
    MissingRequiredStage {
        identifier: String,
    },

    /// A required stage did not pass.
    RequiredStageFailed {
        identifier: String,
        observed: StageStatus,
    },

    /// The expected rejection occurred at a different stage.
    RejectionStageMismatch {
        expected: String,
        observed: String,
    },

    /// The executor returned an error.
    ExecutorFailure {
        message: String,
    },

    /// The executor returned contradictory stage observations.
    DuplicateStageObservation {
        identifier: String,
    },

    /// An expected incomplete analysis was not reported correctly.
    IncompleteAnalysisMismatch,
}

impl fmt::Display for ConformanceFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DispositionMismatch {
                expected,
                observed,
            } => {
                write!(
                    formatter,
                    "expected {expected:?}, observed {observed}"
                )
            }

            Self::MissingRequiredStage { identifier } => {
                write!(
                    formatter,
                    "required stage was not observed: {identifier}"
                )
            }

            Self::RequiredStageFailed {
                identifier,
                observed,
            } => {
                write!(
                    formatter,
                    "required stage {identifier} did not pass: {observed}"
                )
            }

            Self::RejectionStageMismatch {
                expected,
                observed,
            } => {
                write!(
                    formatter,
                    "expected rejection at {expected}, observed rejection at {observed}"
                )
            }

            Self::ExecutorFailure { message } => {
                write!(formatter, "executor failure: {message}")
            }

            Self::DuplicateStageObservation { identifier } => {
                write!(
                    formatter,
                    "stage was reported more than once: {identifier}"
                )
            }

            Self::IncompleteAnalysisMismatch => {
                formatter.write_str(
                    "expected incomplete analysis but the executor did not report it",
                )
            }
        }
    }
}

// ============================================================================
// FIXTURE RESULT
// ============================================================================

/// Result for an individual conformance fixture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureConformanceResult {
    fixture_id: String,
    fixture_name: String,
    status: ConformanceStatus,
    observation: Option<ExecutionObservation>,
    failure: Option<ConformanceFailure>,
}

impl FixtureConformanceResult {
    fn new(
        fixture: &Fixture,
        status: ConformanceStatus,
        observation: Option<ExecutionObservation>,
        failure: Option<ConformanceFailure>,
    ) -> Self {
        Self {
            fixture_id: fixture.id().as_str().to_owned(),
            fixture_name: fixture.name().to_owned(),
            status,
            observation,
            failure,
        }
    }

    /// Returns the fixture identifier.
    #[must_use]
    pub fn fixture_id(&self) -> &str {
        &self.fixture_id
    }

    /// Returns the fixture name.
    #[must_use]
    pub fn fixture_name(&self) -> &str {
        &self.fixture_name
    }

    /// Returns the final status.
    #[must_use]
    pub const fn status(&self) -> ConformanceStatus {
        self.status
    }

    /// Returns the actual execution observation.
    #[must_use]
    pub fn observation(&self) -> Option<&ExecutionObservation> {
        self.observation.as_ref()
    }

    /// Returns the failure reason.
    #[must_use]
    pub fn failure(&self) -> Option<&ConformanceFailure> {
        self.failure.as_ref()
    }
}

// ============================================================================
// STATISTICS
// ============================================================================

/// Aggregate conformance statistics.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConformanceStatistics {
    total: usize,
    passed: usize,
    failed: usize,
    skipped: usize,
}

impl ConformanceStatistics {
    fn record(
        &mut self,
        status: ConformanceStatus,
    ) -> Result<(), ConformanceError> {
        self.total = self.total.checked_add(1).ok_or(
            ConformanceError::CounterOverflow,
        )?;

        let counter = match status {
            ConformanceStatus::Passed => &mut self.passed,
            ConformanceStatus::Failed => &mut self.failed,
            ConformanceStatus::Skipped => &mut self.skipped,
        };

        *counter = counter.checked_add(1).ok_or(
            ConformanceError::CounterOverflow,
        )?;

        Ok(())
    }

    /// Returns the number of processed fixtures.
    #[must_use]
    pub const fn total(&self) -> usize {
        self.total
    }

    /// Returns the number of passing fixtures.
    #[must_use]
    pub const fn passed(&self) -> usize {
        self.passed
    }

    /// Returns the number of failing fixtures.
    #[must_use]
    pub const fn failed(&self) -> usize {
        self.failed
    }

    /// Returns the number of skipped fixtures.
    #[must_use]
    pub const fn skipped(&self) -> usize {
        self.skipped
    }

    /// Returns whether all evaluated fixtures passed.
    ///
    /// An empty suite is not considered evidence of conformance.
    #[must_use]
    pub const fn is_successful(&self) -> bool {
        self.total > 0
            && self.failed == 0
            && self.skipped == 0
    }

    /// Returns whether at least one fixture failed.
    #[must_use]
    pub const fn has_failures(&self) -> bool {
        self.failed > 0
    }
}

// ============================================================================
// CONFORMANCE REPORT
// ============================================================================

/// Complete in-memory conformance report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceReport {
    statistics: ConformanceStatistics,
    results: Vec<FixtureConformanceResult>,
}

impl ConformanceReport {
    /// Returns aggregate statistics.
    #[must_use]
    pub fn statistics(&self) -> &ConformanceStatistics {
        &self.statistics
    }

    /// Returns individual fixture results.
    #[must_use]
    pub fn results(&self) -> &[FixtureConformanceResult] {
        &self.results
    }

    /// Returns whether the complete evaluated suite passed.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.statistics.is_successful()
    }
}

// ============================================================================
// CONFORMANCE ERRORS
// ============================================================================

/// Errors from conformance orchestration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConformanceError {
    /// A required identifier was invalid.
    InvalidIdentifier {
        field: &'static str,
    },

    /// A statistics counter could not be incremented.
    CounterOverflow,

    /// Fixture catalog construction failed.
    FixtureCatalog {
        message: String,
    },
}

impl fmt::Display for ConformanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid {field}")
            }

            Self::CounterOverflow => {
                formatter.write_str(
                    "conformance statistics counter overflow",
                )
            }

            Self::FixtureCatalog { message } => {
                write!(formatter, "fixture catalog error: {message}")
            }
        }
    }
}

impl Error for ConformanceError {}

impl From<FixtureError> for ConformanceError {
    fn from(error: FixtureError) -> Self {
        Self::FixtureCatalog {
            message: error.to_string(),
        }
    }
}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    field: &'static str,
) -> Result<(), ConformanceError> {
    if value.trim().is_empty()
        || value.chars().any(char::is_control)
    {
        return Err(ConformanceError::InvalidIdentifier {
            field,
        });
    }

    Ok(())
}

// ============================================================================
// EXPECTATION EVALUATION
// ============================================================================

fn evaluate_observation(
    fixture: &Fixture,
    observation: &ExecutionObservation,
) -> Result<(), ConformanceFailure> {
    // A target-dependent fixture cannot be declared portable merely
    // because the host frontend accepted its source.
    if fixture.expectation().disposition()
        == ExpectedDisposition::TargetDependent
    {
        if observation.disposition()
            == ObservedDisposition::TargetDependent
        {
            return Ok(());
        }

        return Err(ConformanceFailure::DispositionMismatch {
            expected: ExpectedDisposition::TargetDependent,
            observed: observation.disposition(),
        });
    }

    // Reject duplicate stage identifiers. An executor must provide
    // unambiguous stage evidence.
    for (index, stage) in observation.stages().iter().enumerate() {
        if observation.stages()[..index]
            .iter()
            .any(|previous| {
                previous.identifier() == stage.identifier()
            })
        {
            return Err(
                ConformanceFailure::DuplicateStageObservation {
                    identifier: stage.identifier().to_owned(),
                },
            );
        }
    }

    // Confirm that every stage explicitly required by the fixture
    // has been observed.
    for required in fixture.required_stages() {
        let observed = observation
            .stages()
            .iter()
            .find(|stage| stage.identifier() == required);

        let Some(observed) = observed else {
            return Err(
                ConformanceFailure::MissingRequiredStage {
                    identifier: required.clone(),
                },
            );
        };

        if observed.status() == StageStatus::Skipped {
            return Err(
                ConformanceFailure::RequiredStageFailed {
                    identifier: required.clone(),
                    observed: observed.status(),
                },
            );
        }
    }

    match fixture.expectation().disposition() {
        ExpectedDisposition::Accept => {
            if observation.disposition()
                != ObservedDisposition::Accepted
            {
                return Err(
                    ConformanceFailure::DispositionMismatch {
                        expected: ExpectedDisposition::Accept,
                        observed: observation.disposition(),
                    },
                );
            }

            // Acceptance must not conceal incomplete required stages.
            for required in fixture.required_stages() {
                let stage = observation
                    .stages()
                    .iter()
                    .find(|stage| stage.identifier() == required)
                    .expect("required stage checked above");

                if stage.status() != StageStatus::Passed {
                    return Err(
                        ConformanceFailure::RequiredStageFailed {
                            identifier: required.clone(),
                            observed: stage.status(),
                        },
                    );
                }
            }
        }

        ExpectedDisposition::Reject => {
            if observation.disposition()
                != ObservedDisposition::Rejected
            {
                return Err(
                    ConformanceFailure::DispositionMismatch {
                        expected: ExpectedDisposition::Reject,
                        observed: observation.disposition(),
                    },
                );
            }

            if let Some(expected_stage) =
                fixture.expectation().stage()
            {
                let actual_stage = observation
                    .stages()
                    .iter()
                    .find(|stage| {
                        stage.status() == StageStatus::Failed
                    })
                    .map(StageObservation::identifier);

                if actual_stage != Some(expected_stage) {
                    return Err(
                        ConformanceFailure::RejectionStageMismatch {
                            expected: expected_stage.to_owned(),
                            observed: actual_stage
                                .unwrap_or("unknown")
                                .to_owned(),
                        },
                    );
                }
            }
        }

        ExpectedDisposition::Incomplete => {
            if observation.disposition()
                != ObservedDisposition::Incomplete
            {
                return Err(
                    ConformanceFailure::IncompleteAnalysisMismatch,
                );
            }
        }

        ExpectedDisposition::TargetDependent => {
            // Handled above.
        }
    }

    Ok(())
}

// ============================================================================
// CONFORMANCE RUNNER
// ============================================================================

/// Stateless conformance orchestration service.
///
/// An executor is supplied explicitly so that frontend, IR, backend and
/// runtime testing can share the same conformance framework.
#[derive(Debug, Clone, Copy, Default)]
pub struct ConformanceRunner;

impl ConformanceRunner {
    /// Creates a conformance runner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Executes every fixture in a catalog.
    ///
    /// Results are stored in memory. Use `run_streaming` for large
    /// catalogs when retaining every individual result is undesirable.
    pub fn run<E: ConformanceExecutor>(
        &self,
        catalog: &FixtureCatalog,
        executor: &mut E,
    ) -> Result<ConformanceReport, ConformanceError> {
        let mut statistics = ConformanceStatistics::default();
        let mut results = Vec::new();

        self.run_streaming(
            catalog,
            &FixtureSelector::new(),
            executor,
            |result| {
                statistics
                    .record(result.status())
                    .expect("statistics overflow");
                results.push(result.clone());
            },
        )?;

        Ok(ConformanceReport {
            statistics,
            results,
        })
    }

    /// Executes fixtures selected by the existing fixture selector.
    ///
    /// The selector retains the semantics defined by fixtures.rs.
    pub fn run_selected<E: ConformanceExecutor>(
        &self,
        catalog: &FixtureCatalog,
        selector: &FixtureSelector,
        executor: &mut E,
    ) -> Result<ConformanceReport, ConformanceError> {
        let mut statistics = ConformanceStatistics::default();
        let mut results = Vec::new();

        self.run_streaming(
            catalog,
            selector,
            executor,
            |result| {
                statistics
                    .record(result.status())
                    .expect("statistics overflow");
                results.push(result.clone());
            },
        )?;

        Ok(ConformanceReport {
            statistics,
            results,
        })
    }

    /// Executes fixtures without retaining the complete result collection.
    ///
    /// Results are delivered individually to `on_result`.
    ///
    /// This is the preferred interface for very large conformance suites.
    ///
    /// The callback executes synchronously and must not silently discard
    /// failures if the caller requires complete reporting.
    pub fn run_streaming<E, F>(
        &self,
        catalog: &FixtureCatalog,
        selector: &FixtureSelector,
        executor: &mut E,
        mut on_result: F,
    ) -> Result<ConformanceStatistics, ConformanceError>
    where
        E: ConformanceExecutor,
        F: FnMut(&FixtureConformanceResult),
    {
        let mut statistics = ConformanceStatistics::default();

        for fixture in catalog.select(selector) {
            let result = execute_fixture(fixture, executor);

            statistics.record(result.status())?;

            on_result(&result);
        }

        Ok(statistics)
    }
}

// ============================================================================
// SINGLE FIXTURE EXECUTION
// ============================================================================

fn execute_fixture<E: ConformanceExecutor>(
    fixture: &Fixture,
    executor: &mut E,
) -> FixtureConformanceResult {
    match executor.execute(fixture) {
        Ok(observation) => {
            // Target-dependent tests require a genuine target-aware
            // executor. They must not be marked successful by a
            // frontend-only test.
            if fixture.expectation().disposition()
                == ExpectedDisposition::TargetDependent
                && observation.disposition()
                    != ObservedDisposition::TargetDependent
            {
                return FixtureConformanceResult::new(
                    fixture,
                    ConformanceStatus::Skipped,
                    Some(observation),
                    None,
                );
            }

            match evaluate_observation(fixture, &observation) {
                Ok(()) => {
                    FixtureConformanceResult::new(
                        fixture,
                        ConformanceStatus::Passed,
                        Some(observation),
                        None,
                    )
                }

                Err(failure) => {
                    FixtureConformanceResult::new(
                        fixture,
                        ConformanceStatus::Failed,
                        Some(observation),
                        Some(failure),
                    )
                }
            }
        }

        Err(error) => {
            FixtureConformanceResult::new(
                fixture,
                ConformanceStatus::Failed,
                None,
                Some(ConformanceFailure::ExecutorFailure {
                    message: error.to_string(),
                }),
            )
        }
    }
}

// ============================================================================
// ACTUAL ZAMANI COMPILER INTEGRATION
// ============================================================================

/// Executor using the repository's existing public compilation API.
///
/// Integration:
///
///     crate::compile(source)
///
/// The actual public API currently performs:
///
///     Lexer
///       -> Parser
///       -> SemanticAnalyzer
///       -> IrGenerator
///
/// It returns:
///
///     Result<IrModule, Vec<String>>
///
/// This adapter deliberately does not claim:
///
/// - IR verification.
/// - Optimization verification.
/// - Backend compilation.
/// - Hardware realization.
/// - Runtime execution.
///
/// Those require additional executor implementations.
#[derive(Debug, Clone, Copy, Default)]
pub struct RepositoryCompilerExecutor;

impl RepositoryCompilerExecutor {
    /// Creates an executor for the existing Zamani compiler API.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn classify_diagnostics(
        diagnostics: &[String],
    ) -> (String, String) {
        let message = diagnostics.join("\n");

        let stage_identifier = if diagnostics
            .iter()
            .any(|item| item.starts_with("ParseError:"))
        {
            stage::FRONTEND_PARSE
        } else if diagnostics
            .iter()
            .any(|item| item.starts_with("SemanticError:"))
        {
            stage::SEMANTIC_VALIDATION
        } else {
            "compiler.compile"
        };

        (stage_identifier.to_owned(), message)
    }
}

impl ConformanceExecutor for RepositoryCompilerExecutor {
    fn execute(
        &mut self,
        fixture: &Fixture,
    ) -> Result<ExecutionObservation, ExecutorError> {
        let started = std::time::Instant::now();

        // A target-dependent expectation cannot be established by
        // the target-independent frontend compilation API.
        if fixture.expectation().disposition()
            == ExpectedDisposition::TargetDependent
        {
            return Ok(
                ExecutionObservation::new(
                    ObservedDisposition::NotEvaluated,
                )
                .with_diagnostic(
                    "Target realization requires a target-aware executor.",
                )
                .with_duration(started.elapsed()),
            );
        }

        match crate::compile(fixture.source()) {
            Ok(_module) => {
                // The current public compiler function runs these
                // operations in this order. It does not yet expose
                // independent stage-verification tokens.
                let observation = ExecutionObservation::new(
                    ObservedDisposition::Accepted,
                )
                .with_stage(
                    StageObservation::new(
                        stage::FRONTEND_PARSE,
                        StageStatus::Passed,
                    )
                    .map_err(|error| {
                        ExecutorError::new(error.to_string())
                    })?,
                )
                .with_stage(
                    StageObservation::new(
                        stage::SEMANTIC_VALIDATION,
                        StageStatus::Passed,
                    )
                    .map_err(|error| {
                        ExecutorError::new(error.to_string())
                    })?,
                )
                .with_stage(
                    StageObservation::new(
                        stage::IR_GENERATION,
                        StageStatus::Passed,
                    )
                    .map_err(|error| {
                        ExecutorError::new(error.to_string())
                    })?,
                )
                .with_duration(started.elapsed());

                Ok(observation)
            }

            Err(diagnostics) => {
                let (failed_stage, message) =
                    Self::classify_diagnostics(&diagnostics);

                let mut observation = ExecutionObservation::new(
                    ObservedDisposition::Rejected,
                )
                .with_diagnostic(message.clone())
                .with_duration(started.elapsed());

                if failed_stage != stage::FRONTEND_PARSE {
                    observation = observation.with_stage(
                        StageObservation::new(
                            stage::FRONTEND_PARSE,
                            StageStatus::Passed,
                        )
                        .map_err(|error| {
                            ExecutorError::new(error.to_string())
                        })?,
                    );
                }

                observation = observation.with_stage(
                    StageObservation::new(
                        failed_stage,
                        StageStatus::Failed,
                    )
                    .map_err(|error| {
                        ExecutorError::new(error.to_string())
                    })?
                    .with_message(message),
                );

                Ok(observation)
            }
        }
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::testing::fixtures::{
        FixtureCategory,
        FixtureExpectation,
        FixtureId,
    };

    fn fixture(
        id: &str,
        source: &str,
        expectation: FixtureExpectation,
    ) -> Fixture {
        Fixture::new(
            FixtureId::new(id).expect("valid fixture ID"),
            id,
            "Conformance test fixture",
            source,
            FixtureCategory::Conformance,
            expectation,
        )
        .expect("valid fixture")
    }

    fn catalog_with(fixture: Fixture) -> FixtureCatalog {
        let mut catalog = FixtureCatalog::new();

        catalog.register(fixture).expect("registration");

        catalog
    }

    #[derive(Debug)]
    struct AcceptingExecutor;

    impl ConformanceExecutor for AcceptingExecutor {
        fn execute(
            &mut self,
            _fixture: &Fixture,
        ) -> Result<ExecutionObservation, ExecutorError> {
            Ok(
                ExecutionObservation::new(
                    ObservedDisposition::Accepted,
                )
                .with_stage(
                    StageObservation::new(
                        stage::FRONTEND_PARSE,
                        StageStatus::Passed,
                    )
                    .expect("valid stage"),
                ),
            )
        }
    }

    #[derive(Debug)]
    struct RejectingExecutor;

    impl ConformanceExecutor for RejectingExecutor {
        fn execute(
            &mut self,
            _fixture: &Fixture,
        ) -> Result<ExecutionObservation, ExecutorError> {
            Ok(
                ExecutionObservation::new(
                    ObservedDisposition::Rejected,
                )
                .with_stage(
                    StageObservation::new(
                        stage::FRONTEND_PARSE,
                        StageStatus::Failed,
                    )
                    .expect("valid stage"),
                ),
            )
        }
    }

    #[derive(Debug)]
    struct IncompleteExecutor;

    impl ConformanceExecutor for IncompleteExecutor {
        fn execute(
            &mut self,
            _fixture: &Fixture,
        ) -> Result<ExecutionObservation, ExecutorError> {
            Ok(
                ExecutionObservation::new(
                    ObservedDisposition::Incomplete,
                )
                .with_stage(
                    StageObservation::new(
                        stage::FRONTEND_PARSE,
                        StageStatus::Incomplete,
                    )
                    .expect("valid stage"),
                ),
            )
        }
    }

    #[test]
    fn accepting_fixture_passes() {
        let fixture = fixture(
            "test.accept",
            "fn main() {}",
            FixtureExpectation::accept(),
        )
        .with_required_stage(stage::FRONTEND_PARSE)
        .expect("required stage");

        let catalog = catalog_with(fixture);

        let mut executor = AcceptingExecutor;

        let report = ConformanceRunner::new()
            .run(&catalog, &mut executor)
            .expect("conformance run");

        assert_eq!(report.statistics().total(), 1);
        assert_eq!(report.statistics().passed(), 1);
        assert_eq!(report.statistics().failed(), 0);
        assert!(report.is_successful());
    }

    #[test]
    fn expected_rejection_passes() {
        let expectation = FixtureExpectation::reject(
            stage::FRONTEND_PARSE,
            "Malformed source must be rejected.",
        )
        .expect("expectation");

        let fixture = fixture(
            "test.reject",
            "fn main( {",
            expectation,
        )
        .with_required_stage(stage::FRONTEND_PARSE)
        .expect("required stage");

        let catalog = catalog_with(fixture);

        let mut executor = RejectingExecutor;

        let report = ConformanceRunner::new()
            .run(&catalog, &mut executor)
            .expect("conformance run");

        assert_eq!(report.statistics().passed(), 1);
        assert_eq!(report.statistics().failed(), 0);
    }

    #[test]
    fn unexpected_rejection_fails() {
        let fixture = fixture(
            "test.unexpected",
            "fn main() {}",
            FixtureExpectation::accept(),
        )
        .with_required_stage(stage::FRONTEND_PARSE)
        .expect("required stage");

        let catalog = catalog_with(fixture);

        let mut executor = RejectingExecutor;

        let report = ConformanceRunner::new()
            .run(&catalog, &mut executor)
            .expect("conformance run");

        assert_eq!(report.statistics().failed(), 1);
        assert!(!report.is_successful());
    }

    #[test]
    fn missing_required_stage_fails() {
        let fixture = fixture(
            "test.missing_stage",
            "fn main() {}",
            FixtureExpectation::accept(),
        )
        .with_required_stage(stage::SEMANTIC_VALIDATION)
        .expect("required stage");

        let catalog = catalog_with(fixture);

        let mut executor = AcceptingExecutor;

        let report = ConformanceRunner::new()
            .run(&catalog, &mut executor)
            .expect("conformance run");

        assert_eq!(report.statistics().failed(), 1);

        assert!(matches!(
            report.results()[0].failure(),
            Some(ConformanceFailure::MissingRequiredStage { .. })
        ));
    }

    #[test]
    fn incomplete_analysis_is_not_success() {
        let expectation = FixtureExpectation::incomplete(
            stage::FRONTEND_PARSE,
            "Parser could not establish a complete result.",
        )
        .expect("expectation");

        let fixture = fixture(
            "test.incomplete",
            "incomplete",
            expectation,
        )
        .with_required_stage(stage::FRONTEND_PARSE)
        .expect("required stage");

        let catalog = catalog_with(fixture);

        let mut executor = IncompleteExecutor;

        let report = ConformanceRunner::new()
            .run(&catalog, &mut executor)
            .expect("conformance run");

        assert_eq!(report.statistics().failed(), 0);
        assert_eq!(report.statistics().passed(), 1);
    }

    #[test]
    fn target_dependent_test_is_skipped_without_target_executor() {
        let expectation = FixtureExpectation::target_dependent(
            "Requires an actual compatible execution target.",
        )
        .expect("expectation");

        let fixture = fixture(
            "test.target",
            "fn main() {}",
            expectation,
        );

        let catalog = catalog_with(fixture);

        let mut executor = AcceptingExecutor;

        let report = ConformanceRunner::new()
            .run(&catalog, &mut executor)
            .expect("conformance run");

        assert_eq!(report.statistics().skipped(), 1);
        assert!(!report.is_successful());
    }

    #[test]
    fn streaming_does_not_require_result_collection() {
        let mut catalog = FixtureCatalog::new();

        for identifier in [
            "test.a",
            "test.b",
            "test.c",
        ] {
            catalog.register(
                fixture(
                    identifier,
                    "fn main() {}",
                    FixtureExpectation::accept(),
                )
                .with_required_stage(stage::FRONTEND_PARSE)
                .expect("required stage"),
            ).expect("registration");
        }

        let mut executor = AcceptingExecutor;
        let mut delivered = 0usize;

        let statistics = ConformanceRunner::new()
            .run_streaming(
                &catalog,
                &FixtureSelector::new(),
                &mut executor,
                |_| {
                    delivered += 1;
                },
            )
            .expect("streaming run");

        assert_eq!(statistics.total(), 3);
        assert_eq!(statistics.passed(), 3);
        assert_eq!(delivered, 3);
    }

    #[test]
    fn duplicate_stage_observations_are_rejected() {
        let fixture = fixture(
            "test.duplicate_stage",
            "fn main() {}",
            FixtureExpectation::accept(),
        )
        .with_required_stage(stage::FRONTEND_PARSE)
        .expect("required stage");

        let observation = ExecutionObservation::new(
            ObservedDisposition::Accepted,
        )
        .with_stage(
            StageObservation::new(
                stage::FRONTEND_PARSE,
                StageStatus::Passed,
            )
            .expect("stage"),
        )
        .with_stage(
            StageObservation::new(
                stage::FRONTEND_PARSE,
                StageStatus::Passed,
            )
            .expect("stage"),
        );

        assert!(matches!(
            evaluate_observation(&fixture, &observation),
            Err(ConformanceFailure::DuplicateStageObservation { .. })
        ));
    }

    #[test]
    fn repository_executor_uses_actual_compile_api() {
        let fixture = fixture(
            "test.repository_compile",
            "fn main() {}",
            FixtureExpectation::accept(),
        )
        .with_required_stage(stage::FRONTEND_PARSE)
        .expect("required stage")
        .with_required_stage(stage::SEMANTIC_VALIDATION)
        .expect("required stage")
        .with_required_stage(stage::IR_GENERATION)
        .expect("required stage");

        let mut executor = RepositoryCompilerExecutor::new();

        let observation = executor
            .execute(&fixture)
            .expect("repository executor");

        // This checks that the adapter reports an actual compiler
        // observation. It does not presume every repository fixture
        // is already implemented correctly.
        assert_eq!(
            observation.disposition(),
            ObservedDisposition::Accepted,
            "Current compiler rejected the integration fixture: {:?}",
            observation.diagnostics()
        );
    }

    #[test]
    fn empty_statistics_are_not_conformance_evidence() {
        let statistics = ConformanceStatistics::default();

        assert_eq!(statistics.total(), 0);
        assert!(!statistics.is_successful());
    }
}
