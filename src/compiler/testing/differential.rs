
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Differential Compiler Testing
//!
//! Path: src/compiler/testing/differential.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97.
//!
//! ## Purpose
//!
//! Provides a reusable differential-testing framework for comparing
//! equivalent or intentionally distinct Zamani programs across:
//!
//! - Compiler implementations.
//! - Compiler versions.
//! - Optimization configurations.
//! - IR transformations.
//! - Backend implementations.
//! - Hardware targets.
//! - Runtime implementations.
//! - Classical and hybrid execution environments.
//! - Quantum and probabilistic execution environments.
//! - Distributed execution environments.
//!
//! The framework compares observations supplied by authoritative
//! compiler, semantic, IR, backend, or runtime adapters.
//!
//! It does not assume that compiler-internal representations are
//! directly comparable.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Differential test case contracts.
//! - Execution variant descriptions.
//! - Differential execution interfaces.
//! - Comparison contracts.
//! - Comparison verdicts.
//! - Differential result classification.
//! - Streaming differential execution.
//! - Aggregate statistics.
//! - Differential failure reporting.
//!
//! This module does not own:
//!
//! - Fixtures or fixture catalogs.
//! - Zamani grammar.
//! - Lexical or parsing rules.
//! - AST definitions.
//! - Semantic definitions.
//! - Type or ownership analysis.
//! - Canonical IR definitions.
//! - IR serialization.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Resource allocation.
//! - Backend implementations.
//! - Runtime execution.
//! - Domain-specific equivalence mathematics.
//!
//! Fixture ownership remains in `fixtures.rs`.
//! Conformance ownership remains in `conformance.rs`.
//! Semantic and IR equivalence remain with their authoritative owners.
//!
//! ## Differential correctness
//!
//! Differential testing is meaningful only when both sides are compared
//! using an appropriate equivalence contract.
//!
//! A byte-for-byte comparison is suitable only when the compared bytes
//! have a canonical representation and byte equality is the required
//! semantic relation.
//!
//! Quantum, probabilistic, floating-point, distributed, and concurrent
//! computations may require domain-specific equivalence relations.
//! Such relations must be supplied by a comparator implementation.
//!
//! An inconclusive comparison is never a passing comparison.
//!
//! ## POCO-REAF
//!
//! Differential testing supports:
//!
//! Program Once
//!     -> Portable semantic meaning
//!     -> Multiple realizations
//!     -> Comparable observable behavior
//!
//! It does not claim that every program can execute on every target.
//! Unsupported or unavailable execution must be reported explicitly.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary maximum for:
//!
//! - Differential cases.
//! - Fixtures.
//! - Execution variants.
//! - Domains.
//! - Targets.
//! - Comparison evidence.
//! - Test results.
//! - Source length.
//! - IR size.
//! - Hardware capacity.
//!
//! Streaming execution avoids retaining the complete result set.
//! Actual execution remains bounded by available resources.
//!
//! ## Determinism
//!
//! The runner preserves the order of the supplied cases.
//! It introduces no random ordering, global mutable state,
//! implicit target selection, or wall-clock-dependent expectations.
//!
//! Executors and comparators are responsible for declaring and
//! implementing their own determinism guarantees.

use core::fmt;
use std::error::Error;

use super::fixtures::{Fixture, FixtureId};

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of the differential-testing framework.
///
/// Independent of the Zamani language, grammar, AST, semantic model,
//! IR, backend, and artifact versions.
pub const DIFFERENTIAL_API_VERSION: u16 = 1;

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validates a public differential-testing identifier.
///
/// Identifiers must not be empty or contain control characters.
/// No arbitrary length ceiling is imposed.
fn validate_identifier(
    value: &str,
    field: &'static str,
) -> Result<(), DifferentialError> {
    if value.trim().is_empty() || value.chars().any(char::is_control) {
        return Err(DifferentialError::InvalidIdentifier { field });
    }

    Ok(())
}

// ============================================================================
// EXECUTION VARIANT
// ============================================================================

/// Identifies one compiler or execution configuration.
///
/// Examples include:
///
/// - An unoptimized compiler.
/// - An optimized compiler.
/// - A reference compiler.
/// - A candidate compiler.
/// - A CPU backend.
/// - A GPU backend.
/// - A quantum simulator.
/// - A quantum processor.
/// - A distributed runtime.
///
/// The identifier is descriptive. It does not select or instantiate
/// a backend by itself.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExecutionVariant {
    identifier: String,
    description: Option<String>,
}

impl ExecutionVariant {
    /// Creates an execution variant.
    pub fn new(
        identifier: impl Into<String>,
    ) -> Result<Self, DifferentialError> {
        let identifier = identifier.into();
        validate_identifier(&identifier, "execution variant")?;

        Ok(Self {
            identifier,
            description: None,
        })
    }

    /// Adds a human-readable description.
    #[must_use]
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Returns the stable variant identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

// ============================================================================
// COMPARISON CONTRACT
// ============================================================================

/// The relation expected between two differential observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExpectedRelation {
    /// Both observations must be semantically equivalent.
    Equivalent,

    /// The observations must be observably different.
    Distinct,
}

impl fmt::Display for ExpectedRelation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equivalent => formatter.write_str("equivalent"),
            Self::Distinct => formatter.write_str("distinct"),
        }
    }
}

/// Defines what a differential test is expected to establish.
///
/// `scope` identifies the meaning being compared, such as:
///
/// - `classical.observable-result`
/// - `quantum.measurement-distribution`
/// - `hybrid.observable-behavior`
/// - `ir.semantic-preservation`
/// - `backend.execution-result`
/// - `distributed.consistency`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparisonContract {
    identifier: String,
    scope: String,
    expected_relation: ExpectedRelation,
    rationale: Option<String>,
}

impl ComparisonContract {
    /// Creates a comparison contract.
    pub fn new(
        identifier: impl Into<String>,
        scope: impl Into<String>,
        expected_relation: ExpectedRelation,
    ) -> Result<Self, DifferentialError> {
        let identifier = identifier.into();
        let scope = scope.into();

        validate_identifier(&identifier, "comparison contract identifier")?;
        validate_identifier(&scope, "comparison scope")?;

        Ok(Self {
            identifier,
            scope,
            expected_relation,
            rationale: None,
        })
    }

    /// Adds the reason for the expected relation.
    #[must_use]
    pub fn with_rationale(
        mut self,
        rationale: impl Into<String>,
    ) -> Self {
        self.rationale = Some(rationale.into());
        self
    }

    /// Returns the contract identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the semantic comparison scope.
    #[must_use]
    pub fn scope(&self) -> &str {
        &self.scope
    }

    /// Returns the expected relation.
    #[must_use]
    pub const fn expected_relation(&self) -> ExpectedRelation {
        self.expected_relation
    }

    /// Returns the rationale, if provided.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }
}

// ============================================================================
// DIFFERENTIAL CASE
// ============================================================================

/// One differential comparison between two fixture executions.
///
/// The fixtures may contain identical source with different execution
/// variants, or independently authored source programs that are expected
/// to have equivalent or distinct observable behavior.
///
/// Fixtures are borrowed from the canonical fixture subsystem.
/// This module does not duplicate fixture storage.
#[derive(Debug, Clone)]
pub struct DifferentialCase<'a> {
    identifier: String,
    left_fixture: &'a Fixture,
    right_fixture: &'a Fixture,
    left_variant: ExecutionVariant,
    right_variant: ExecutionVariant,
    contract: ComparisonContract,
}

impl<'a> DifferentialCase<'a> {
    /// Creates a differential case.
    pub fn new(
        identifier: impl Into<String>,
        left_fixture: &'a Fixture,
        right_fixture: &'a Fixture,
        left_variant: ExecutionVariant,
        right_variant: ExecutionVariant,
        contract: ComparisonContract,
    ) -> Result<Self, DifferentialError> {
        let identifier = identifier.into();
        validate_identifier(&identifier, "differential case identifier")?;

        Ok(Self {
            identifier,
            left_fixture,
            right_fixture,
            left_variant,
            right_variant,
            contract,
        })
    }

    /// Returns the differential case identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the left fixture.
    #[must_use]
    pub fn left_fixture(&self) -> &'a Fixture {
        self.left_fixture
    }

    /// Returns the right fixture.
    #[must_use]
    pub fn right_fixture(&self) -> &'a Fixture {
        self.right_fixture
    }

    /// Returns the left execution variant.
    #[must_use]
    pub fn left_variant(&self) -> &ExecutionVariant {
        &self.left_variant
    }

    /// Returns the right execution variant.
    #[must_use]
    pub fn right_variant(&self) -> &ExecutionVariant {
        &self.right_variant
    }

    /// Returns the comparison contract.
    #[must_use]
    pub fn contract(&self) -> &ComparisonContract {
        &self.contract
    }
}

// ============================================================================
// EXECUTION OUTCOME
// ============================================================================

/// Result of attempting to execute one side of a differential case.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DifferentialExecutionOutcome<T> {
    /// The executor completed and produced an observation.
    Completed(T),

    /// The executor deliberately did not perform the operation.
    ///
    /// Examples include unavailable hardware, unsupported target
    /// capabilities, or an explicitly disabled integration.
    ///
    /// A skipped execution cannot establish differential equivalence.
    NotRun {
        /// Explanation for not executing the operation.
        reason: String,
    },
}

impl<T> DifferentialExecutionOutcome<T> {
    /// Returns whether execution completed.
    #[must_use]
    pub const fn is_completed(&self) -> bool {
        matches!(self, Self::Completed(_))
    }

    /// Returns the completed observation, if available.
    #[must_use]
    pub fn observation(&self) -> Option<&T> {
        match self {
            Self::Completed(observation) => Some(observation),
            Self::NotRun { .. } => None,
        }
    }
}

// ============================================================================
// EXECUTION ERROR
// ============================================================================

/// Error returned by a differential executor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DifferentialExecutionError {
    message: String,
}

impl DifferentialExecutionError {
    /// Creates an execution error.
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

impl fmt::Display for DifferentialExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for DifferentialExecutionError {}

// ============================================================================
// EXECUTOR CONTRACT
// ============================================================================

/// Executes fixtures using explicitly supplied execution variants.
///
/// Implementations integrate the relevant repository authorities.
///
/// For example:
///
/// - A compiler adapter may compile through `crate::compile`.
/// - An IR adapter may use the canonical IR verifier.
/// - A backend adapter may invoke registered backend providers.
/// - A runtime adapter may execute through `src/runtime/`.
/// - A quantum adapter may compare canonical quantum observations.
///
/// The executor must not claim that a stage ran unless it actually ran.
///
/// The executor must not silently substitute a different target or
/// change the program's semantic requirements.
pub trait DifferentialExecutor {
    /// The observation type produced by this executor.
    ///
    /// It should represent the semantic or observable result needed
    /// by the associated comparator, not an arbitrary debug dump.
    type Observation: fmt::Debug;

    /// Executes one fixture using one execution variant.
    fn execute(
        &mut self,
        fixture: &Fixture,
        variant: &ExecutionVariant,
    ) -> Result<
        DifferentialExecutionOutcome<Self::Observation>,
        DifferentialExecutionError,
    >;
}

// ============================================================================
// COMPARISON VERDICT
// ============================================================================

/// Result of applying a semantic comparison policy.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ComparisonVerdict {
    /// The observations satisfy the comparator's equivalence relation.
    Equivalent {
        /// Evidence explaining the equivalence decision.
        evidence: String,
    },

    /// The observations are observably different.
    Different {
        /// Evidence explaining the difference.
        evidence: String,
    },

    /// The comparator cannot establish either relation.
    ///
    /// This is not a pass.
    Inconclusive {
        /// Explanation for the inconclusive result.
        reason: String,
    },
}

impl ComparisonVerdict {
    /// Returns whether the comparator established equivalence.
    #[must_use]
    pub const fn is_equivalent(&self) -> bool {
        matches!(self, Self::Equivalent { .. })
    }

    /// Returns whether the comparator established a difference.
    #[must_use]
    pub const fn is_different(&self) -> bool {
        matches!(self, Self::Different { .. })
    }

    /// Returns the evidence or explanation.
    #[must_use]
    pub fn explanation(&self) -> &str {
        match self {
            Self::Equivalent { evidence } => evidence,
            Self::Different { evidence } => evidence,
            Self::Inconclusive { reason } => reason,
        }
    }
}

// ============================================================================
// COMPARATOR ERROR
// ============================================================================

/// Error returned when a comparison policy cannot be applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DifferentialComparisonError {
    message: String,
}

impl DifferentialComparisonError {
    /// Creates a comparison error.
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

impl fmt::Display for DifferentialComparisonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for DifferentialComparisonError {}

// ============================================================================
// COMPARATOR CONTRACT
// ============================================================================

/// Compares two observations under an explicit comparison contract.
///
/// Implementations own the mathematics and normalization rules required
/// for their domain.
///
/// The framework deliberately does not impose a universal floating-point
/// tolerance, statistical confidence level, quantum measurement policy,
/// or distributed consistency model.
pub trait DifferentialComparator<T> {
    /// Compares two observations.
    fn compare(
        &self,
        left: &T,
        right: &T,
        contract: &ComparisonContract,
    ) -> Result<ComparisonVerdict, DifferentialComparisonError>;
}

// ============================================================================
// EXACT COMPARATOR
// ============================================================================

/// Exact equality comparator for observations implementing `Eq`.
///
/// Use only when exact equality is the correct semantic contract.
/// It is not a substitute for quantum, probabilistic, floating-point,
/// or domain-specific equivalence policies.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExactComparator;

impl ExactComparator {
    /// Creates an exact comparator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl<T: Eq> DifferentialComparator<T> for ExactComparator {
    fn compare(
        &self,
        left: &T,
        right: &T,
        _contract: &ComparisonContract,
    ) -> Result<ComparisonVerdict, DifferentialComparisonError> {
        if left == right {
            Ok(ComparisonVerdict::Equivalent {
                evidence: "Observations are exactly equal.".to_owned(),
            })
        } else {
            Ok(ComparisonVerdict::Different {
                evidence: "Observations are not exactly equal.".to_owned(),
            })
        }
    }
}

// ============================================================================
// SIDE RESULT
// ============================================================================

/// Result of executing one side of a differential case.
#[derive(Debug)]
pub struct DifferentialSideResult<T> {
    fixture_id: String,
    fixture_name: String,
    variant: ExecutionVariant,
    outcome: Result<
        DifferentialExecutionOutcome<T>,
        DifferentialExecutionError,
    >,
}

impl<T> DifferentialSideResult<T> {
    fn new(
        fixture: &Fixture,
        variant: &ExecutionVariant,
        outcome: Result<
            DifferentialExecutionOutcome<T>,
            DifferentialExecutionError,
        >,
    ) -> Self {
        Self {
            fixture_id: fixture.id().as_str().to_owned(),
            fixture_name: fixture.name().to_owned(),
            variant: variant.clone(),
            outcome,
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

    /// Returns the execution variant.
    #[must_use]
    pub fn variant(&self) -> &ExecutionVariant {
        &self.variant
    }

    /// Returns the execution outcome or execution error.
    #[must_use]
    pub fn outcome(
        &self,
    ) -> &Result<
        DifferentialExecutionOutcome<T>,
        DifferentialExecutionError,
    > {
        &self.outcome
    }
}

// ============================================================================
// RESULT STATUS
// ============================================================================

/// Final classification of a differential case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DifferentialStatus {
    /// The observed relation matched the expected relation.
    Passed,

    /// The observed relation contradicted the expected relation,
    /// or an execution/comparison error occurred.
    Failed,

    /// Execution or comparison could not establish a result.
    Skipped,
}

impl fmt::Display for DifferentialStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Passed => formatter.write_str("passed"),
            Self::Failed => formatter.write_str("failed"),
            Self::Skipped => formatter.write_str("skipped"),
        }
    }
}

// ============================================================================
// FAILURE
// ============================================================================

/// Structured failure reasons for differential testing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DifferentialFailure {
    /// The left-side execution failed.
    LeftExecution {
        /// Error message.
        message: String,
    },

    /// The right-side execution failed.
    RightExecution {
        /// Error message.
        message: String,
    },

    /// The comparator failed.
    Comparison {
        /// Error message.
        message: String,
    },

    /// The observed relation did not match the expected relation.
    RelationMismatch {
        /// Expected relation.
        expected: ExpectedRelation,

        /// Actual comparator verdict.
        actual: ComparisonVerdict,
    },
}

impl fmt::Display for DifferentialFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LeftExecution { message } => {
                write!(formatter, "left execution failed: {message}")
            }
            Self::RightExecution { message } => {
                write!(formatter, "right execution failed: {message}")
            }
            Self::Comparison { message } => {
                write!(formatter, "comparison failed: {message}")
            }
            Self::RelationMismatch { expected, actual } => {
                write!(
                    formatter,
                    "expected {expected}, observed {:?}",
                    actual
                )
            }
        }
    }
}

// ============================================================================
// CASE RESULT
// ============================================================================

/// Complete result of one differential case.
#[derive(Debug)]
pub struct DifferentialCaseResult<T> {
    identifier: String,
    contract: ComparisonContract,
    status: DifferentialStatus,
    left: DifferentialSideResult<T>,
    right: DifferentialSideResult<T>,
    verdict: Option<ComparisonVerdict>,
    failure: Option<DifferentialFailure>,
}

impl<T> DifferentialCaseResult<T> {
    fn new(
        identifier: &str,
        contract: &ComparisonContract,
        status: DifferentialStatus,
        left: DifferentialSideResult<T>,
        right: DifferentialSideResult<T>,
        verdict: Option<ComparisonVerdict>,
        failure: Option<DifferentialFailure>,
    ) -> Self {
        Self {
            identifier: identifier.to_owned(),
            contract: contract.clone(),
            status,
            left,
            right,
            verdict,
            failure,
        }
    }

    /// Returns the differential case identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Returns the comparison contract.
    #[must_use]
    pub fn contract(&self) -> &ComparisonContract {
        &self.contract
    }

    /// Returns the final status.
    #[must_use]
    pub const fn status(&self) -> DifferentialStatus {
        self.status
    }

    /// Returns the left execution result.
    #[must_use]
    pub fn left(&self) -> &DifferentialSideResult<T> {
        &self.left
    }

    /// Returns the right execution result.
    #[must_use]
    pub fn right(&self) -> &DifferentialSideResult<T> {
        &self.right
    }

    /// Returns the comparator verdict, if comparison was performed.
    #[must_use]
    pub fn verdict(&self) -> Option<&ComparisonVerdict> {
        self.verdict.as_ref()
    }

    /// Returns the failure, if one occurred.
    #[must_use]
    pub fn failure(&self) -> Option<&DifferentialFailure> {
        self.failure.as_ref()
    }
}

// ============================================================================
// STATISTICS
// ============================================================================

/// Aggregate differential-testing statistics.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DifferentialStatistics {
    total: usize,
    passed: usize,
    failed: usize,
    skipped: usize,
}

impl DifferentialStatistics {
    fn record(
        &mut self,
        status: DifferentialStatus,
    ) -> Result<(), DifferentialError> {
        self.total = self
            .total
            .checked_add(1)
            .ok_or(DifferentialError::CounterOverflow)?;

        let counter = match status {
            DifferentialStatus::Passed => &mut self.passed,
            DifferentialStatus::Failed => &mut self.failed,
            DifferentialStatus::Skipped => &mut self.skipped,
        };

        *counter = counter
            .checked_add(1)
            .ok_or(DifferentialError::CounterOverflow)?;

        Ok(())
    }

    /// Returns the total number of processed cases.
    #[must_use]
    pub const fn total(&self) -> usize {
        self.total
    }

    /// Returns the number of passing cases.
    #[must_use]
    pub const fn passed(&self) -> usize {
        self.passed
    }

    /// Returns the number of failing cases.
    #[must_use]
    pub const fn failed(&self) -> usize {
        self.failed
    }

    /// Returns the number of skipped cases.
    #[must_use]
    pub const fn skipped(&self) -> usize {
        self.skipped
    }

    /// Returns whether the non-empty suite passed completely.
    ///
    /// Empty suites and suites containing skipped cases are not
    /// conformance evidence.
    #[must_use]
    pub const fn is_successful(&self) -> bool {
        self.total > 0 && self.failed == 0 && self.skipped == 0
    }

    /// Returns whether at least one case failed.
    #[must_use]
    pub const fn has_failures(&self) -> bool {
        self.failed > 0
    }
}

// ============================================================================
// REPORT
// ============================================================================

/// In-memory differential-testing report.
#[derive(Debug)]
pub struct DifferentialReport<T> {
    statistics: DifferentialStatistics,
    results: Vec<DifferentialCaseResult<T>>,
}

impl<T> DifferentialReport<T> {
    /// Returns aggregate statistics.
    #[must_use]
    pub fn statistics(&self) -> &DifferentialStatistics {
        &self.statistics
    }

    /// Returns individual case results.
    #[must_use]
    pub fn results(&self) -> &[DifferentialCaseResult<T>] {
        &self.results
    }

    /// Returns whether the complete non-empty suite passed.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.statistics.is_successful()
    }
}

// ============================================================================
// FRAMEWORK ERRORS
// ============================================================================

/// Errors produced by differential-test orchestration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DifferentialError {
    /// An identifier was empty or contained a control character.
    InvalidIdentifier {
        /// Identifier field.
        field: &'static str,
    },

    /// A statistics counter overflowed.
    CounterOverflow,
}

impl fmt::Display for DifferentialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid {field}")
            }
            Self::CounterOverflow => {
                formatter.write_str(
                    "differential-testing statistics counter overflow",
                )
            }
        }
    }
}

impl Error for DifferentialError {}

// ============================================================================
// SINGLE CASE EXECUTION
// ============================================================================

fn execute_case<E, C>(
    case: &DifferentialCase<'_>,
    executor: &mut E,
    comparator: &C,
) -> DifferentialCaseResult<E::Observation>
where
    E: DifferentialExecutor,
    C: DifferentialComparator<E::Observation>,
{
    // Both sides are attempted independently. A failure on one side
    // does not prevent collecting evidence from the other side.
    let left_outcome = executor.execute(
        case.left_fixture(),
        case.left_variant(),
    );

    let right_outcome = executor.execute(
        case.right_fixture(),
        case.right_variant(),
    );

    let left = DifferentialSideResult::new(
        case.left_fixture(),
        case.left_variant(),
        left_outcome,
    );

    let right = DifferentialSideResult::new(
        case.right_fixture(),
        case.right_variant(),
        right_outcome,
    );

    // Execution errors are failures, not successful skips.
    if let Err(error) = &left.outcome {
        return DifferentialCaseResult::new(
            case.identifier(),
            case.contract(),
            DifferentialStatus::Failed,
            left,
            right,
            None,
            Some(DifferentialFailure::LeftExecution {
                message: error.to_string(),
            }),
        );
    }

    if let Err(error) = &right.outcome {
        return DifferentialCaseResult::new(
            case.identifier(),
            case.contract(),
            DifferentialStatus::Failed,
            left,
            right,
            None,
            Some(DifferentialFailure::RightExecution {
                message: error.to_string(),
            }),
        );
    }

    let left_observation = match left.outcome.as_ref() {
        Ok(DifferentialExecutionOutcome::Completed(value)) => value,
        Ok(DifferentialExecutionOutcome::NotRun { .. }) => {
            return DifferentialCaseResult::new(
                case.identifier(),
                case.contract(),
                DifferentialStatus::Skipped,
                left,
                right,
                None,
                None,
            );
        }
        Err(_) => unreachable!("execution errors were checked above"),
    };

    let right_observation = match right.outcome.as_ref() {
        Ok(DifferentialExecutionOutcome::Completed(value)) => value,
        Ok(DifferentialExecutionOutcome::NotRun { .. }) => {
            return DifferentialCaseResult::new(
                case.identifier(),
                case.contract(),
                DifferentialStatus::Skipped,
                left,
                right,
                None,
                None,
            );
        }
        Err(_) => unreachable!("execution errors were checked above"),
    };

    let verdict = match comparator.compare(
        left_observation,
        right_observation,
        case.contract(),
    ) {
        Ok(verdict) => verdict,
        Err(error) => {
            return DifferentialCaseResult::new(
                case.identifier(),
                case.contract(),
                DifferentialStatus::Failed,
                left,
                right,
                None,
                Some(DifferentialFailure::Comparison {
                    message: error.to_string(),
                }),
            );
        }
    };

    match &verdict {
        ComparisonVerdict::Inconclusive { .. } => {
            DifferentialCaseResult::new(
                case.identifier(),
                case.contract(),
                DifferentialStatus::Skipped,
                left,
                right,
                Some(verdict),
                None,
            )
        }

        ComparisonVerdict::Equivalent { .. }
            if case.contract().expected_relation()
                == ExpectedRelation::Equivalent =>
        {
            DifferentialCaseResult::new(
                case.identifier(),
                case.contract(),
                DifferentialStatus::Passed,
                left,
                right,
                Some(verdict),
                None,
            )
        }

        ComparisonVerdict::Different { .. }
            if case.contract().expected_relation()
                == ExpectedRelation::Distinct =>
        {
            DifferentialCaseResult::new(
                case.identifier(),
                case.contract(),
                DifferentialStatus::Passed,
                left,
                right,
                Some(verdict),
                None,
            )
        }

        _ => DifferentialCaseResult::new(
            case.identifier(),
            case.contract(),
            DifferentialStatus::Failed,
            left,
            right,
            Some(verdict.clone()),
            Some(DifferentialFailure::RelationMismatch {
                expected: case.contract().expected_relation(),
                actual: verdict,
            }),
        ),
    }
}

// ============================================================================
// RUNNER
// ============================================================================

/// Stateless differential-testing runner.
///
/// The executor and comparator are explicit dependencies. This prevents
/// the testing framework from silently selecting a compiler, backend,
/// runtime, target, or domain-specific equivalence policy.
#[derive(Debug, Clone, Copy, Default)]
pub struct DifferentialRunner;

impl DifferentialRunner {
    /// Creates a differential runner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Executes cases and retains every result in memory.
    ///
    /// Use `run_streaming` for large suites when retaining every result
    /// is undesirable.
    pub fn run<'a, I, E, C>(
        &self,
        cases: I,
        executor: &mut E,
        comparator: &C,
    ) -> Result<DifferentialReport<E::Observation>, DifferentialError>
    where
        I: IntoIterator<Item = DifferentialCase<'a>>,
        E: DifferentialExecutor,
        C: DifferentialComparator<E::Observation>,
    {
        let mut statistics = DifferentialStatistics::default();
        let mut results = Vec::new();

        self.run_streaming(
            cases,
            executor,
            comparator,
            |result| {
                // The callback cannot fail. Statistics are maintained
                // by the runner itself in the streaming implementation.
                results.push(result);
            },
        )
        .map(|streamed_statistics| {
            statistics = streamed_statistics;
            DifferentialReport {
                statistics,
                results,
            }
        })
    }

    /// Executes cases in input order without retaining the complete
    /// result collection.
    ///
    /// Each completed result is delivered synchronously to `on_result`.
    ///
    /// This is the preferred interface for large differential suites.
    pub fn run_streaming<'a, I, E, C, F>(
        &self,
        cases: I,
        executor: &mut E,
        comparator: &C,
        mut on_result: F,
    ) -> Result<DifferentialStatistics, DifferentialError>
    where
        I: IntoIterator<Item = DifferentialCase<'a>>,
        E: DifferentialExecutor,
        C: DifferentialComparator<E::Observation>,
        F: FnMut(DifferentialCaseResult<E::Observation>),
    {
        let mut statistics = DifferentialStatistics::default();

        for case in cases {
            let result = execute_case(&case, executor, comparator);

            statistics.record(result.status())?;

            on_result(result);
        }

        Ok(statistics)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::testing::fixtures::{
        ExpectedDisposition,
        FixtureCategory,
    };

    fn fixture(id: &str, source: &str) -> Fixture {
        Fixture::new(
            FixtureId::new(id).expect("valid fixture ID"),
            id,
            "Differential test fixture",
            source,
            FixtureCategory::Conformance,
            super::super::fixtures::FixtureExpectation::accept(),
        )
        .expect("valid fixture")
    }

    fn contract(relation: ExpectedRelation) -> ComparisonContract {
        ComparisonContract::new(
            "test.comparison",
            "test.observable-result",
            relation,
        )
        .expect("valid comparison contract")
    }

    #[derive(Debug)]
    struct StringExecutor;

    impl DifferentialExecutor for StringExecutor {
        type Observation = String;

        fn execute(
            &mut self,
            fixture: &Fixture,
            _variant: &ExecutionVariant,
        ) -> Result<
            DifferentialExecutionOutcome<Self::Observation>,
            DifferentialExecutionError,
        > {
            Ok(DifferentialExecutionOutcome::Completed(
                fixture.source().to_owned(),
            ))
        }
    }

    #[derive(Debug)]
    struct NotRunExecutor;

    impl DifferentialExecutor for NotRunExecutor {
        type Observation = String;

        fn execute(
            &mut self,
            _fixture: &Fixture,
            _variant: &ExecutionVariant,
        ) -> Result<
            DifferentialExecutionOutcome<Self::Observation>,
            DifferentialExecutionError,
        > {
            Ok(DifferentialExecutionOutcome::NotRun {
                reason: "Required target is unavailable.".to_owned(),
            })
        }
    }

    #[derive(Debug)]
    struct FailingExecutor;

    impl DifferentialExecutor for FailingExecutor {
        type Observation = String;

        fn execute(
            &mut self,
            _fixture: &Fixture,
            _variant: &ExecutionVariant,
        ) -> Result<
            DifferentialExecutionOutcome<Self::Observation>,
            DifferentialExecutionError,
        > {
            Err(DifferentialExecutionError::new(
                "Simulated execution failure.",
            ))
        }
    }

    #[derive(Debug)]
    struct InconclusiveComparator;

    impl DifferentialComparator<String> for InconclusiveComparator {
        fn compare(
            &self,
            _left: &String,
            _right: &String,
            _contract: &ComparisonContract,
        ) -> Result<ComparisonVerdict, DifferentialComparisonError> {
            Ok(ComparisonVerdict::Inconclusive {
                reason: "Insufficient evidence.".to_owned(),
            })
        }
    }

    #[test]
    fn exact_equivalent_observations_pass() {
        let left = fixture("test.left", "same");
        let right = fixture("test.right", "same");

        let case = DifferentialCase::new(
            "test.equivalent",
            &left,
            &right,
            ExecutionVariant::new("reference").expect("variant"),
            ExecutionVariant::new("candidate").expect("variant"),
            contract(ExpectedRelation::Equivalent),
        )
        .expect("case");

        let mut executor = StringExecutor;

        let report = DifferentialRunner::new()
            .run(
                [case],
                &mut executor,
                &ExactComparator::new(),
            )
            .expect("run");

        assert_eq!(report.statistics().total(), 1);
        assert_eq!(report.statistics().passed(), 1);
        assert_eq!(report.statistics().failed(), 0);
        assert!(report.is_successful());
    }

    #[test]
    fn unexpected_difference_fails_equivalence_contract() {
        let left = fixture("test.left", "left");
        let right = fixture("test.right", "right");

        let case = DifferentialCase::new(
            "test.difference",
            &left,
            &right,
            ExecutionVariant::new("reference").expect("variant"),
            ExecutionVariant::new("candidate").expect("variant"),
            contract(ExpectedRelation::Equivalent),
        )
        .expect("case");

        let mut executor = StringExecutor;

        let report = DifferentialRunner::new()
            .run(
                [case],
                &mut executor,
                &ExactComparator::new(),
            )
            .expect("run");

        assert_eq!(report.statistics().failed(), 1);
        assert!(!report.is_successful());
    }

    #[test]
    fn expected_difference_passes_distinct_contract() {
        let left = fixture("test.left", "left");
        let right = fixture("test.right", "right");

        let case = DifferentialCase::new(
            "test.distinct",
            &left,
            &right,
            ExecutionVariant::new("reference").expect("variant"),
            ExecutionVariant::new("candidate").expect("variant"),
            contract(ExpectedRelation::Distinct),
        )
        .expect("case");

        let mut executor = StringExecutor;

        let report = DifferentialRunner::new()
            .run(
                [case],
                &mut executor,
                &ExactComparator::new(),
            )
            .expect("run");

        assert_eq!(report.statistics().passed(), 1);
        assert!(report.is_successful());
    }

    #[test]
    fn unavailable_target_is_skipped_not_passed() {
        let left = fixture("test.left", "same");
        let right = fixture("test.right", "same");

        let case = DifferentialCase::new(
            "test.unavailable",
            &left,
            &right,
            ExecutionVariant::new("reference").expect("variant"),
            ExecutionVariant::new("unavailable-target").expect("variant"),
            contract(ExpectedRelation::Equivalent),
        )
        .expect("case");

        let mut executor = NotRunExecutor;

        let report = DifferentialRunner::new()
            .run(
                [case],
                &mut executor,
                &ExactComparator::new(),
            )
            .expect("run");

        assert_eq!(report.statistics().skipped(), 1);
        assert!(!report.is_successful());
    }

    #[test]
    fn execution_error_is_a_failure() {
        let left = fixture("test.left", "same");
        let right = fixture("test.right", "same");

        let case = DifferentialCase::new(
            "test.execution_error",
            &left,
            &right,
            ExecutionVariant::new("reference").expect("variant"),
            ExecutionVariant::new("candidate").expect("variant"),
            contract(ExpectedRelation::Equivalent),
        )
        .expect("case");

        let mut executor = FailingExecutor;

        let report = DifferentialRunner::new()
            .run(
                [case],
                &mut executor,
                &ExactComparator::new(),
            )
            .expect("run");

        assert_eq!(report.statistics().failed(), 1);
        assert!(matches!(
            report.results()[0].failure(),
            Some(DifferentialFailure::LeftExecution { .. })
        ));
    }

    #[test]
    fn inconclusive_comparison_is_not_a_pass() {
        let left = fixture("test.left", "same");
        let right = fixture("test.right", "same");

        let case = DifferentialCase::new(
            "test.inconclusive",
            &left,
            &right,
            ExecutionVariant::new("reference").expect("variant"),
            ExecutionVariant::new("candidate").expect("variant"),
            contract(ExpectedRelation::Equivalent),
        )
        .expect("case");

        let mut executor = StringExecutor;

        let report = DifferentialRunner::new()
            .run(
                [case],
                &mut executor,
                &InconclusiveComparator,
            )
            .expect("run");

        assert_eq!(report.statistics().skipped(), 1);
        assert!(!report.is_successful());
    }

    #[test]
    fn streaming_processes_cases_without_collecting_results() {
        let left = fixture("test.left", "same");
        let right = fixture("test.right", "same");

        let cases = (0..3).map(|index| {
            DifferentialCase::new(
                format!("test.case.{index}"),
                &left,
                &right,
                ExecutionVariant::new("reference").expect("variant"),
                ExecutionVariant::new("candidate").expect("variant"),
                contract(ExpectedRelation::Equivalent),
            )
            .expect("case")
        });

        let mut executor = StringExecutor;
        let mut delivered = 0usize;

        let statistics = DifferentialRunner::new()
            .run_streaming(
                cases,
                &mut executor,
                &ExactComparator::new(),
                |_| delivered += 1,
            )
            .expect("streaming run");

        assert_eq!(statistics.total(), 3);
        assert_eq!(statistics.passed(), 3);
        assert_eq!(delivered, 3);
    }

    #[test]
    fn empty_suite_is_not_successful() {
        let mut executor = StringExecutor;

        let report = DifferentialRunner::new()
            .run(
                std::iter::empty(),
                &mut executor,
                &ExactComparator::new(),
            )
            .expect("run");

        assert_eq!(report.statistics().total(), 0);
        assert!(!report.is_successful());
    }

    #[test]
    fn fixture_expectation_is_owned_by_fixture_subsystem() {
        let item = fixture("test.fixture", "source");

        assert_eq!(
            item.expectation().disposition(),
            ExpectedDisposition::Accept
        );
    }
}
