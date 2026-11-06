
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Compiler Regression Testing
//!
//! Path: `src/compiler/testing/regression.rs`
//!
//! Rust edition: 2021.
//! Minimum supported Rust version: 1.97.
//!
//! ## Purpose
//!
//! Provides a reusable regression-test execution framework for the Zamani
//! compiler and its integrated computing domains.
//!
//! A regression case executes an actual operation supplied by its caller.
//! The operation may exercise the frontend, semantic analysis, canonical IR,
//! optimization, lowering, backend generation, linking, artifact validation,
//! or runtime integration.
//!
//! The framework records whether each operation passed or failed and
//! supports streaming execution so that the complete result set does not
//! need to remain in memory.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Regression-case identity and metadata.
//! - Regression-case execution contracts.
//! - Streaming regression execution.
//! - Regression outcomes.
//! - Regression statistics.
//! - Duplicate-case detection.
//! - Regression execution failure reporting.
//!
//! This module does not own:
//!
//! - Source fixtures.
//! - Grammar definitions.
//! - Parsing.
//! - Semantic analysis.
//! - Ownership analysis.
//! - Canonical IR.
//! - Optimization algorithms.
//! - Hardware discovery.
//! - Resource allocation.
//! - Backend implementations.
//! - Artifact serialization.
//! - Runtime execution.
//!
//! Those responsibilities remain with their existing repository owners.
//!
//! ## Integration authorities
//!
//! - `testing/fixtures.rs` owns fixture identity, source and expectations.
//! - `testing/conformance.rs` owns language and compiler conformance.
//! - `testing/differential.rs` owns cross-implementation comparisons.
//! - `testing/determinism.rs` owns determinism conformance.
//! - `testing/scalability.rs` owns scalability workloads.
//! - `configuration/budget.rs` owns compiler resource-budget semantics.
//! - `pipeline/` owns compilation-stage orchestration.
//! - `ir/` integrates canonical IR representations.
//! - `backends/` integrates backend providers.
//! - `artifacts/` owns compiler artifact contracts.
//!
//! This module is an execution harness, not a replacement for those systems.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Regression cases.
//! - Source files.
//! - Source bytes.
//! - Compilation units.
//! - IR operations.
//! - Functions.
//! - Modules.
//! - Computing domains.
//! - Hardware targets.
//! - Devices.
//! - Workers.
//! - Test-suite size.
//!
//! The runner consumes cases incrementally.
//!
//! It retains the identifiers already encountered to detect duplicate
//! regression cases. Therefore, identifier-tracking memory grows with the
//! number of distinct cases. It does not retain all regression results.
//!
//! Actual execution remains subject to available memory, execution time,
//! representational limits and explicitly configured resource budgets.
//!
//! No finite computer can execute literally infinite work.
//!
//! ## Determinism
//!
//! Cases execute in the order supplied by the caller.
//!
//! This module does not sort cases, randomize execution, or impose a
//! different scheduling policy.
//!
//! Deterministic test ordering does not prove deterministic compiler output.
//! That property is tested by `testing/determinism.rs`.
//!
//! ## Failure behavior
//!
//! A failed regression case is recorded and execution continues so that
//! independent failures can be reported in one run.
//!
//! Invalid identifiers, duplicate identifiers and counter overflow are
//! framework errors and terminate the run.
//!
//! A panic inside a case is not converted into a passing or ordinary failed
//! result. It propagates to the surrounding Rust test process.
//!
//! The caller must not interpret an incomplete run as a successful suite.
//!
//! ## Grammar relationship
//!
//! This module does not define or parse Zamani syntax.
//!
//! Its test cases must use the canonical language implementation and respect
//! the grammar/specification authority chain:
//!
//! `grammar/specification/`
//! `grammar/spec/`
//! `grammar/Zamani.g4`
//! `grammar/antlr/`
//! `src/lexer.rs`
//! `src/parser.rs`
//! `src/ast/`
//! semantic analysis
//! canonical IR
//! compiler and runtime.
//!
//! Deterministic-build intent is specified by
//! `grammar/compile/deterministic-builds.g4`.
//!
//! Determinism semantics are specified by
//! `grammar/spec/determinism.md`.
//!
//! Regression tests verify implementations of those contracts; they do not
//! create new source-language semantics.

use core::fmt;
use std::collections::BTreeSet;
use std::error::Error;
use std::time::{Duration, Instant};

/// Public API version of the regression-testing framework.
///
/// This version is independent of the Zamani language, grammar, semantic
/// model, IR, backend and artifact versions.
pub const REGRESSION_API_VERSION: u16 = 1;

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// A stable identifier for one regression case.
///
/// Identifiers are descriptive and must be unique within one execution.
/// No arbitrary identifier-length ceiling is imposed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegressionCaseId(String);

impl RegressionCaseId {
    /// Creates a regression-case identifier.
    ///
    /// Empty identifiers and identifiers containing control characters
    /// are rejected.
    pub fn new(value: impl Into<String>) -> Result<Self, RegressionError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(RegressionError::InvalidIdentifier {
                reason: "identifier must not be empty",
            });
        }

        if value.chars().any(char::is_control) {
            return Err(RegressionError::InvalidIdentifier {
                reason: "identifier must not contain control characters",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RegressionCaseId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// REGRESSION CASE
// ============================================================================

/// One executable regression case.
///
/// A case owns its identifier, optional description and execution callback.
///
/// The callback must exercise the actual implementation being tested.
/// It must return an error when the expected behavior is not observed.
///
/// The callback may borrow data from the surrounding test scope.
///
/// A case is deliberately not tied to one compiler subsystem. This permits
/// callers to adapt existing fixtures, conformance tests, differential tests,
/// backend tests and runtime tests without introducing duplicate authorities.
pub struct RegressionCase<'a> {
    id: RegressionCaseId,
    description: Option<String>,
    execute: Box<dyn FnMut() -> Result<(), String> + 'a>,
}

impl<'a> RegressionCase<'a> {
    /// Creates a regression case.
    ///
    /// The callback may return any error type implementing `Display`.
    /// The error is converted to a diagnostic string for reporting.
    pub fn new<F, E>(
        id: impl Into<String>,
        execute: F,
    ) -> Result<Self, RegressionError>
    where
        F: FnMut() -> Result<(), E> + 'a,
        E: fmt::Display + 'a,
    {
        let execute = execute;

        Ok(Self {
            id: RegressionCaseId::new(id)?,
            description: None,
            execute: Box::new(move || {
                execute().map_err(|error| error.to_string())
            }),
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

    /// Returns the regression-case identifier.
    #[must_use]
    pub fn id(&self) -> &RegressionCaseId {
        &self.id
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

// ============================================================================
// OUTCOMES
// ============================================================================

/// Outcome of executing one regression case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegressionOutcome {
    /// The regression case completed successfully.
    Passed,

    /// The regression case detected a regression or execution failure.
    Failed {
        /// Human-readable failure information.
        reason: String,
    },
}

impl RegressionOutcome {
    /// Returns whether the case passed.
    #[must_use]
    pub fn is_passed(&self) -> bool {
        matches!(self, Self::Passed)
    }

    /// Returns whether the case failed.
    #[must_use]
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed { .. })
    }

    /// Returns the failure reason, if one exists.
    #[must_use]
    pub fn failure_reason(&self) -> Option<&str> {
        match self {
            Self::Passed => None,
            Self::Failed { reason } => Some(reason),
        }
    }
}

/// Result of one regression-case execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegressionResult {
    id: RegressionCaseId,
    description: Option<String>,
    outcome: RegressionOutcome,
    elapsed: Duration,
}

impl RegressionResult {
    /// Returns the case identifier.
    #[must_use]
    pub fn id(&self) -> &RegressionCaseId {
        &self.id
    }

    /// Returns the optional case description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the case outcome.
    #[must_use]
    pub fn outcome(&self) -> &RegressionOutcome {
        &self.outcome
    }

    /// Returns the observed execution duration.
    ///
    /// Duration is diagnostic information, not a deterministic correctness
    /// assertion. Tests must not require identical durations.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Returns whether the case passed.
    #[must_use]
    pub fn is_passed(&self) -> bool {
        self.outcome.is_passed()
    }
}

// ============================================================================
// STATISTICS
// ============================================================================

/// Aggregate statistics for a regression run.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RegressionStatistics {
    total: u128,
    passed: u128,
    failed: u128,
}

impl RegressionStatistics {
    /// Returns the number of attempted cases.
    #[must_use]
    pub fn total(&self) -> u128 {
        self.total
    }

    /// Returns the number of passed cases.
    #[must_use]
    pub fn passed(&self) -> u128 {
        self.passed
    }

    /// Returns the number of failed cases.
    #[must_use]
    pub fn failed(&self) -> u128 {
        self.failed
    }

    /// Returns whether at least one case ran and every case passed.
    ///
    /// An empty suite is not evidence of regression correctness.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.total > 0 && self.failed == 0 && self.passed == self.total
    }

    fn record(
        &mut self,
        outcome: &RegressionOutcome,
    ) -> Result<(), RegressionError> {
        self.total = self
            .total
            .checked_add(1)
            .ok_or(RegressionError::CounterOverflow)?;

        match outcome {
            RegressionOutcome::Passed => {
                self.passed = self
                    .passed
                    .checked_add(1)
                    .ok_or(RegressionError::CounterOverflow)?;
            }
            RegressionOutcome::Failed { .. } => {
                self.failed = self
                    .failed
                    .checked_add(1)
                    .ok_or(RegressionError::CounterOverflow)?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// RUNNER
// ============================================================================

/// Configuration for a regression run.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RegressionRunOptions {
    /// Optional expected number of cases.
    ///
    /// When supplied, the runner verifies that exactly this many cases were
    /// consumed. It is an expectation, not a maximum workload limit.
    expected_cases: Option<u128>,
}

impl RegressionRunOptions {
    /// Creates default run options without an expected case count.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            expected_cases: None,
        }
    }

    /// Sets the expected number of cases.
    #[must_use]
    pub const fn with_expected_cases(mut self, expected: u128) -> Self {
        self.expected_cases = Some(expected);
        self
    }

    /// Returns the expected number of cases, if configured.
    #[must_use]
    pub const fn expected_cases(&self) -> Option<u128> {
        self.expected_cases
    }
}

/// Streaming regression runner.
///
/// Results are delivered to the supplied callback as soon as each case
/// completes. The runner does not retain the complete result collection.
///
/// The runner retains case identifiers in a `BTreeSet` to detect duplicates.
/// This requires memory proportional to the number of distinct case IDs.
///
/// Cases are executed sequentially in iterator order. This runner does not
/// implement a second parallel scheduler.
#[derive(Debug, Clone, Copy, Default)]
pub struct RegressionRunner {
    options: RegressionRunOptions,
}

impl RegressionRunner {
    /// Creates a runner with default options.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            options: RegressionRunOptions::new(),
        }
    }

    /// Creates a runner with explicit options.
    #[must_use]
    pub const fn with_options(options: RegressionRunOptions) -> Self {
        Self { options }
    }

    /// Returns the runner options.
    #[must_use]
    pub const fn options(&self) -> RegressionRunOptions {
        self.options
    }

    /// Executes cases incrementally and delivers each result immediately.
    ///
    /// Failed cases do not stop the run. This allows independent regressions
    /// to be reported together.
    ///
    /// Framework errors stop execution immediately.
    ///
    /// The callback receives a borrowed result. It may persist, serialize,
    /// aggregate or report the result without requiring this runner to retain
    /// all results.
    ///
    /// The caller controls the case iterator and therefore controls the
    /// execution order.
    pub fn run_streaming<'a, I, F>(
        &self,
        cases: I,
        mut on_result: F,
    ) -> Result<RegressionStatistics, RegressionError>
    where
        I: IntoIterator<Item = RegressionCase<'a>>,
        F: FnMut(&RegressionResult),
    {
        let mut identifiers = BTreeSet::new();
        let mut statistics = RegressionStatistics::default();

        for mut case in cases {
            if !identifiers.insert(case.id.clone()) {
                return Err(RegressionError::DuplicateIdentifier {
                    id: case.id,
                });
            }

            let started = Instant::now();

            let outcome = match (case.execute)() {
                Ok(()) => RegressionOutcome::Passed,
                Err(reason) => RegressionOutcome::Failed { reason },
            };

            let result = RegressionResult {
                id: case.id,
                description: case.description,
                outcome,
                elapsed: started.elapsed(),
            };

            statistics.record(&result.outcome)?;

            on_result(&result);
        }

        if let Some(expected) = self.options.expected_cases {
            if statistics.total != expected {
                return Err(RegressionError::UnexpectedCaseCount {
                    expected,
                    actual: statistics.total,
                });
            }
        }

        Ok(statistics)
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by the regression-testing framework.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegressionError {
    /// A case identifier was invalid.
    InvalidIdentifier {
        /// Explanation of the invalid identifier.
        reason: &'static str,
    },

    /// A regression-case identifier appeared more than once.
    DuplicateIdentifier {
        /// Duplicate identifier.
        id: RegressionCaseId,
    },

    /// An internal statistics counter exceeded its representable range.
    CounterOverflow,

    /// The number of executed cases differed from the configured expectation.
    UnexpectedCaseCount {
        /// Expected number of cases.
        expected: u128,

        /// Actual number of cases.
        actual: u128,
    },
}

impl fmt::Display for RegressionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { reason } => {
                write!(formatter, "invalid regression identifier: {reason}")
            }
            Self::DuplicateIdentifier { id } => {
                write!(formatter, "duplicate regression case identifier: {id}")
            }
            Self::CounterOverflow => {
                formatter.write_str("regression statistics counter overflow")
            }
            Self::UnexpectedCaseCount { expected, actual } => {
                write!(
                    formatter,
                    "regression case count mismatch: expected {expected}, executed {actual}"
                )
            }
        }
    }
}

impl Error for RegressionError {}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn passing_case(id: &str) -> RegressionCase<'static> {
        RegressionCase::new(id, || Ok::<(), String>(()))
            .expect("valid regression case")
    }

    #[test]
    fn accepts_non_empty_identifiers() {
        let id = RegressionCaseId::new("compiler.frontend.parse")
            .expect("valid identifier");

        assert_eq!(id.as_str(), "compiler.frontend.parse");
    }

    #[test]
    fn rejects_empty_identifiers() {
        assert!(matches!(
            RegressionCaseId::new("   "),
            Err(RegressionError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn rejects_control_characters_in_identifiers() {
        assert!(matches!(
            RegressionCaseId::new("compiler\nparse"),
            Err(RegressionError::InvalidIdentifier { .. })
        ));
    }

    #[test]
    fn executes_a_passing_case() {
        let runner = RegressionRunner::new();
        let mut delivered = Vec::new();

        let statistics = runner
            .run_streaming(
                [passing_case("compiler.parse")],
                |result| delivered.push(result.clone()),
            )
            .expect("regression run");

        assert_eq!(statistics.total(), 1);
        assert_eq!(statistics.passed(), 1);
        assert_eq!(statistics.failed(), 0);
        assert!(statistics.is_successful());

        assert_eq!(delivered.len(), 1);
        assert!(delivered[0].is_passed());
    }

    #[test]
    fn records_failed_cases_and_continues() {
        let cases = [
            passing_case("compiler.first"),
            RegressionCase::new("compiler.second", || {
                Err::<(), _>("expected behavior was not observed")
            })
            .expect("valid case"),
            passing_case("compiler.third"),
        ];

        let mut observed = Vec::new();

        let statistics = RegressionRunner::new()
            .run_streaming(cases, |result| {
                observed.push(result.clone());
            })
            .expect("regression run");

        assert_eq!(statistics.total(), 3);
        assert_eq!(statistics.passed(), 2);
        assert_eq!(statistics.failed(), 1);
        assert!(!statistics.is_successful());

        assert_eq!(observed.len(), 3);
        assert_eq!(
            observed[1].outcome().failure_reason(),
            Some("expected behavior was not observed")
        );
    }

    #[test]
    fn preserves_caller_supplied_execution_order() {
        let order = Rc::new(RefCell::new(Vec::new()));

        let cases = ["first", "second", "third"].into_iter().map(|name| {
            let order = Rc::clone(&order);
            RegressionCase::new(name, move || {
                order.borrow_mut().push(name);
                Ok::<(), String>(())
            })
            .expect("valid case")
        });

        let statistics = RegressionRunner::new()
            .run_streaming(cases, |_| {})
            .expect("regression run");

        assert_eq!(statistics.total(), 3);
        assert_eq!(
            *order.borrow(),
            vec!["first", "second", "third"]
        );
    }

    #[test]
    fn rejects_duplicate_case_identifiers() {
        let cases = [
            passing_case("compiler.duplicate"),
            passing_case("compiler.duplicate"),
        ];

        let error = RegressionRunner::new()
            .run_streaming(cases, |_| {})
            .expect_err("duplicate identifier must fail");

        assert!(matches!(
            error,
            RegressionError::DuplicateIdentifier { .. }
        ));
    }

    #[test]
    fn supports_an_empty_iterator_without_claiming_success() {
        let statistics = RegressionRunner::new()
            .run_streaming(
                std::iter::empty::<RegressionCase<'static>>(),
                |_| {},
            )
            .expect("empty run is structurally valid");

        assert_eq!(statistics.total(), 0);
        assert!(!statistics.is_successful());
    }

    #[test]
    fn validates_expected_case_count() {
        let options =
            RegressionRunOptions::new().with_expected_cases(2);

        let error = RegressionRunner::with_options(options)
            .run_streaming([passing_case("compiler.only")], |_| {})
            .expect_err("incorrect case count must fail");

        assert_eq!(
            error,
            RegressionError::UnexpectedCaseCount {
                expected: 2,
                actual: 1,
            }
        );
    }

    #[test]
    fn accepts_expected_case_count_when_satisfied() {
        let options =
            RegressionRunOptions::new().with_expected_cases(2);

        let statistics = RegressionRunner::with_options(options)
            .run_streaming(
                [
                    passing_case("compiler.first"),
                    passing_case("compiler.second"),
                ],
                |_| {},
            )
            .expect("expected case count");

        assert_eq!(statistics.total(), 2);
        assert!(statistics.is_successful());
    }

    #[test]
    fn supports_borrowed_test_state() {
        let observed = RefCell::new(false);

        let case = RegressionCase::new("compiler.borrowed_state", || {
            *observed.borrow_mut() = true;
            Ok::<(), String>(())
        })
        .expect("valid case");

        let statistics = RegressionRunner::new()
            .run_streaming([case], |_| {})
            .expect("regression run");

        assert!(statistics.is_successful());
        assert!(*observed.borrow());
    }

    #[test]
    fn reports_execution_duration_as_observation() {
        let mut elapsed = None;

        RegressionRunner::new()
            .run_streaming(
                [passing_case("compiler.duration")],
                |result| elapsed = Some(result.elapsed()),
            )
            .expect("regression run");

        assert!(elapsed.is_some());
    }

    #[test]
    fn descriptions_are_preserved() {
        let case = passing_case("compiler.description")
            .with_description("Checks frontend compatibility");

        assert_eq!(
            case.description(),
            Some("Checks frontend compatibility")
        );
    }

    #[test]
    fn failure_reason_is_available_without_reinterpreting_the_error() {
        let result = RegressionOutcome::Failed {
            reason: String::from("IR verification failed"),
        };

        assert!(result.is_failed());
        assert!(!result.is_passed());
        assert_eq!(
            result.failure_reason(),
            Some("IR verification failed")
        );
    }
}
