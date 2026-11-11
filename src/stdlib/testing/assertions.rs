
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Testing Assertions
//!
//! Path: `src/stdlib/testing/assertions.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide reusable, deterministic, target-independent assertion primitives
//! for Zamani's standard-library testing infrastructure.
//!
//! The assertion API supports tests for:
//!
//! - Classical computation.
//! - Scientific and numerical computation.
//! - Quantum computation.
//! - HDL and hardware-description systems.
//! - Classical-quantum hybrid computation.
//! - Distributed computation.
//! - AI and machine learning.
//! - Networking and storage.
//! - Cryptography and security.
//! - Concurrency and asynchronous systems.
//! - Embedded and resource-constrained systems.
//! - Future computational domains.
//!
//! Assertions describe and evaluate expected behavior. They do not select
//! hardware, execute programs, discover providers, or schedule tests.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! Testing subsystem:
//!     src/stdlib/testing/
//!
//! Existing testing compatibility module:
//!     src/stdlib/test_framework.rs
//!
//! Standard-library error authority:
//!     src/stdlib/error.rs
//!
//! Canonical grammar authority:
//!     grammar/DESIGN.md
//!     grammar/specification/
//!     grammar/spec/
//!     grammar/Zamani.g4
//!
//! Canonical AST authority:
//!     src/frontend/ast/
//!
//! Canonical semantic authority:
//!     src/frontend/semantic/
//!
//! ## OWNS
//!
//! - Assertion results.
//! - Assertion failures.
//! - Expected-versus-actual reporting.
//! - Reusable assertion functions.
//! - Numerical comparison tolerances.
//! - Aggregation of assertion failures.
//!
//! ## DOES NOT OWN
//!
//! - Test discovery or registration.
//! - Test execution or scheduling.
//! - Property-based value generation.
//! - Fuzzing engines.
//! - Test fixtures.
//! - Test lifecycle.
//! - AST or source-span representations.
//! - Parsing or grammar definitions.
//! - Semantic validation.
//! - IR construction or verification.
//! - Runtime execution.
//! - Hardware or provider selection.
//!
//! These responsibilities remain with their designated modules.
//!
//! ## FAILURE MODEL
//!
//! Assertion failure is an expected testing outcome. It is not an
//! infrastructure failure.
//!
//! The API distinguishes:
//!
//! - An assertion that succeeds.
//! - An assertion that fails.
//! - An assertion that cannot be evaluated because the supplied test
//!   operation itself failed.
//!
//! Unsupported operations, exhausted budgets, and incomplete verification
//! must never be converted into successful assertions.
//!
//! ## SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Number of assertions.
//! - Number of failures.
//! - Number of assertion contexts.
//! - Expected or actual value lengths.
//! - Collection lengths.
//! - String lengths.
//! - Number of testing domains.
//!
//! Storage and computation are subject to available resources.
//!
//! This does not claim that finite hardware can process literally infinite
//! workloads.
//!
//! Operational limits belong to the caller's explicitly configured testing
//! policy, not to universal assertion semantics.
//!
//! ## DETERMINISM
//!
//! Assertion functions are deterministic for the same inputs and comparison
//! implementations.
//!
//! No global mutable state is maintained.
//!
//! No implicit randomness is introduced.
//!
//! No test ordering is imposed.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe code.
//!
//! No raw pointers.
//!
//! No global mutable state.
//!
//! No hidden I/O.
//!
//! No implicit logging.
//!
//! No automatic retries.
//!
//! No process termination.
//!
//! Ordinary assertion failure never causes a panic.
//!
//! ## INTEGRATION CONTRACT
//!
//! The testing module exposes this file through:
//!
//!     crate::stdlib::testing::assertions
//!
//! The testing framework may consume AssertionResult and AssertionReport.
//!
//! Conformance, portability, parser, semantic, and domain-specific tests
//! may use the same assertion functions.
//!
//! This module deliberately has no dependency on unfinished testing
//! infrastructure.
//!
//! ## FREEZE CRITERIA
//!
//! 1. All public assertion functions have documented contracts.
//! 2. Assertion failures preserve expected and actual information.
//! 3. Numerical comparisons handle NaN and infinity correctly.
//! 4. Option and Result assertions preserve references to their values.
//! 5. Multiple assertion failures can be aggregated.
//! 6. No ordinary failure requires a panic.
//! 7. No universal workload ceiling is introduced.
//! 8. No AST, semantic, IR, or runtime authority is duplicated.
//! 9. Unit tests cover positive, negative, boundary, and regression cases.
//! 10. Rust 1.97 or later compiles the module.
//! 11. Formatting and Clippy checks pass.
//!
//! Compiling this file does not establish that the entire testing subsystem
//! or the repository is production-ready.

// =============================================================================
// Imports
// =============================================================================

use std::error::Error;
use std::fmt;

// =============================================================================
// Public result type
// =============================================================================

/// Result of evaluating one assertion.
///
/// `Ok(())` means the assertion completed and passed.
///
/// `Err(AssertionFailure)` means the assertion completed and failed.
///
/// This type does not represent infrastructure failures that occur before
/// an assertion can be evaluated. Those failures must remain explicit in
/// the test adapter or execution layer.
pub type AssertionResult<T = ()> = Result<T, AssertionFailure>;

// =============================================================================
// Assertion failure
// =============================================================================

/// Structured description of a failed assertion.
///
/// Expected and actual values are retained as descriptive strings.
/// The original values are not retained because assertion operands may
/// contain non-cloneable or non-serializable data.
///
/// The assertion failure is independent of source-language diagnostics
/// and standard-library operational errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssertionFailure {
    name: String,
    message: String,
    expected: Option<String>,
    actual: Option<String>,
    context: Vec<String>,
}

impl AssertionFailure {
    /// Creates an assertion failure.
    ///
    /// An empty assertion name is permitted because callers may use
    /// dynamically generated or anonymous assertions.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            message: message.into(),
            expected: None,
            actual: None,
            context: Vec::new(),
        }
    }

    /// Creates an assertion failure containing expected and actual values.
    #[must_use]
    pub fn with_values(
        name: impl Into<String>,
        message: impl Into<String>,
        expected: impl Into<String>,
        actual: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            message: message.into(),
            expected: Some(expected.into()),
            actual: Some(actual.into()),
            context: Vec::new(),
        }
    }

    /// Adds contextual information to this failure.
    ///
    /// Context entries are appended in the order supplied.
    #[must_use]
    pub fn with_context(
        mut self,
        context: impl Into<String>,
    ) -> Self {
        self.context.push(context.into());
        self
    }

    /// Returns the assertion name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the human-readable failure message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the expected value when available.
    #[must_use]
    pub fn expected(&self) -> Option<&str> {
        self.expected.as_deref()
    }

    /// Returns the actual value when available.
    #[must_use]
    pub fn actual(&self) -> Option<&str> {
        self.actual.as_deref()
    }

    /// Returns the contextual information.
    #[must_use]
    pub fn context(&self) -> &[String] {
        &self.context
    }

    /// Returns whether the failure includes expected and actual values.
    #[must_use]
    pub fn has_values(&self) -> bool {
        self.expected.is_some() && self.actual.is_some()
    }
}

impl fmt::Display for AssertionFailure {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if self.name.is_empty() {
            write!(formatter, "assertion failed: {}", self.message)?;
        } else {
            write!(
                formatter,
                "assertion '{}' failed: {}",
                self.name,
                self.message
            )?;
        }

        if let Some(expected) = &self.expected {
            write!(formatter, "\n  expected: {}", expected)?;
        }

        if let Some(actual) = &self.actual {
            write!(formatter, "\n  actual:   {}", actual)?;
        }

        for context in &self.context {
            write!(formatter, "\n  context:  {}", context)?;
        }

        Ok(())
    }
}

impl Error for AssertionFailure {}

// =============================================================================
// Assertion report
// =============================================================================

/// Collection of assertion failures from a group of evaluations.
///
/// A report is successful when it contains no failures.
///
/// The report deliberately stores failures rather than maintaining a
/// potentially misleading global test count.
///
/// The caller owns test execution counts and scheduling.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AssertionReport {
    failures: Vec<AssertionFailure>,
}

impl AssertionReport {
    /// Creates an empty successful report.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            failures: Vec::new(),
        }
    }

    /// Adds one failed assertion.
    pub fn push(&mut self, failure: AssertionFailure) {
        self.failures.push(failure);
    }

    /// Records the result of one assertion.
    ///
    /// Successful assertions do not modify the report.
    pub fn record(
        &mut self,
        result: AssertionResult,
    ) {
        if let Err(failure) = result {
            self.push(failure);
        }
    }

    /// Adds all failures from another report.
    ///
    /// The original report is consumed.
    pub fn extend(&mut self, other: Self) {
        self.failures.extend(other.failures);
    }

    /// Returns whether all recorded assertions succeeded.
    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.failures.is_empty()
    }

    /// Returns whether at least one assertion failed.
    #[must_use]
    pub fn has_failures(&self) -> bool {
        !self.failures.is_empty()
    }

    /// Returns the number of recorded failures.
    #[must_use]
    pub fn failure_count(&self) -> usize {
        self.failures.len()
    }

    /// Returns the recorded failures in insertion order.
    #[must_use]
    pub fn failures(&self) -> &[AssertionFailure] {
        &self.failures
    }

    /// Consumes the report and returns its failures.
    #[must_use]
    pub fn into_failures(self) -> Vec<AssertionFailure> {
        self.failures
    }

    /// Converts the report into a result.
    ///
    /// Returns success when there are no failures.
    ///
    /// Otherwise, returns all recorded failures.
    #[must_use]
    pub fn into_result(
        self,
    ) -> Result<(), Vec<AssertionFailure>> {
        if self.failures.is_empty() {
            Ok(())
        } else {
            Err(self.failures)
        }
    }
}

/// Evaluates an existing sequence of assertion results and collects
/// every failure.
///
/// Evaluation of the supplied iterator is performed in iterator order.
///
/// The caller controls whether assertion evaluation itself is eager or lazy.
#[must_use]
pub fn evaluate_all(
    results: impl IntoIterator<Item = AssertionResult>,
) -> AssertionReport {
    let mut report = AssertionReport::new();

    for result in results {
        report.record(result);
    }

    report
}

// =============================================================================
// Boolean assertions
// =============================================================================

/// Asserts that a condition is true.
#[must_use]
pub fn assert_true(
    condition: bool,
    message: impl Into<String>,
) -> AssertionResult {
    if condition {
        Ok(())
    } else {
        Err(AssertionFailure::with_values(
            "assert_true",
            message,
            true.to_string(),
            false.to_string(),
        ))
    }
}

/// Asserts that a condition is false.
#[must_use]
pub fn assert_false(
    condition: bool,
    message: impl Into<String>,
) -> AssertionResult {
    if !condition {
        Ok(())
    } else {
        Err(AssertionFailure::with_values(
            "assert_false",
            message,
            false.to_string(),
            true.to_string(),
        ))
    }
}

// =============================================================================
// Equality assertions
// =============================================================================

/// Asserts that two values are equal.
///
/// Values are compared using their canonical `PartialEq` implementation.
///
/// Debug representations are retained in the failure report.
#[must_use]
pub fn assert_eq<T>(
    expected: &T,
    actual: &T,
) -> AssertionResult
where
    T: PartialEq + fmt::Debug + ?Sized,
{
    if expected == actual {
        Ok(())
    } else {
        Err(AssertionFailure::with_values(
            "assert_eq",
            "values are not equal",
            format!("{expected:?}"),
            format!("{actual:?}"),
        ))
    }
}

/// Asserts that two values are not equal.
#[must_use]
pub fn assert_ne<T>(
    left: &T,
    right: &T,
) -> AssertionResult
where
    T: PartialEq + fmt::Debug + ?Sized,
{
    if left != right {
        Ok(())
    } else {
        Err(AssertionFailure::with_values(
            "assert_ne",
            "values are unexpectedly equal",
            "different values",
            format!("{left:?}"),
        ))
    }
}

// =============================================================================
// Predicate assertions
// =============================================================================

/// Asserts that a value satisfies a supplied predicate.
///
/// The predicate is evaluated exactly once.
///
/// The predicate may perform arbitrary caller-defined computation.
/// This module does not retry it or hide its side effects.
#[must_use]
pub fn assert_predicate<T, F>(
    name: impl Into<String>,
    value: &T,
    predicate: F,
    message: impl Into<String>,
) -> AssertionResult
where
    T: fmt::Debug + ?Sized,
    F: FnOnce(&T) -> bool,
{
    if predicate(value) {
        Ok(())
    } else {
        Err(AssertionFailure::with_values(
            name,
            message,
            "predicate satisfied",
            format!("{value:?}"),
        ))
    }
}

// =============================================================================
// Numerical assertions
// =============================================================================

/// Explicit absolute and relative comparison tolerances.
///
/// Both tolerances must be finite and non-negative.
///
/// Absolute tolerance is useful near zero.
///
/// Relative tolerance scales with the magnitude of the compared values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tolerance {
    absolute: f64,
    relative: f64,
}

impl Tolerance {
    /// Creates a validated comparison tolerance.
    ///
    /// # Errors
    ///
    /// Returns an error if either tolerance is negative, NaN, or infinite.
    pub fn new(
        absolute: f64,
        relative: f64,
    ) -> Result<Self, ToleranceError> {
        if !absolute.is_finite() || absolute < 0.0 {
            return Err(ToleranceError::InvalidAbsolute);
        }

        if !relative.is_finite() || relative < 0.0 {
            return Err(ToleranceError::InvalidRelative);
        }

        Ok(Self {
            absolute,
            relative,
        })
    }

    /// Creates an exact comparison tolerance.
    #[must_use]
    pub const fn exact() -> Self {
        Self {
            absolute: 0.0,
            relative: 0.0,
        }
    }

    /// Returns the absolute tolerance.
    #[must_use]
    pub const fn absolute(self) -> f64 {
        self.absolute
    }

    /// Returns the relative tolerance.
    #[must_use]
    pub const fn relative(self) -> f64 {
        self.relative
    }

    /// Returns whether two floating-point values are within tolerance.
    ///
    /// Equal infinities compare equal.
    ///
    /// NaN never compares approximately equal, including to itself.
    ///
    /// Different infinities never compare approximately equal.
    #[must_use]
    pub fn matches(
        self,
        expected: f64,
        actual: f64,
    ) -> bool {
        if expected == actual {
            return true;
        }

        if !expected.is_finite() || !actual.is_finite() {
            return false;
        }

        let difference = (expected - actual).abs();

        if difference <= self.absolute {
            return true;
        }

        let scale = expected.abs().max(actual.abs());

        if scale == 0.0 {
            return difference <= self.absolute;
        }

        difference / scale <= self.relative
    }
}

/// Errors produced when constructing an invalid numerical tolerance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToleranceError {
    /// The absolute tolerance is negative or non-finite.
    InvalidAbsolute,

    /// The relative tolerance is negative or non-finite.
    InvalidRelative,
}

impl fmt::Display for ToleranceError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidAbsolute => {
                formatter.write_str(
                    "absolute tolerance must be finite and non-negative",
                )
            }
            Self::InvalidRelative => {
                formatter.write_str(
                    "relative tolerance must be finite and non-negative",
                )
            }
        }
    }
}

impl Error for ToleranceError {}

/// Asserts that two floating-point values are approximately equal.
///
/// A caller must explicitly supply the tolerance. No machine-dependent
/// default precision is assumed.
#[must_use]
pub fn assert_approx_eq(
    expected: f64,
    actual: f64,
    tolerance: Tolerance,
) -> AssertionResult {
    if tolerance.matches(expected, actual) {
        Ok(())
    } else {
        Err(AssertionFailure::with_values(
            "assert_approx_eq",
            format!(
                "values differ beyond absolute tolerance {} and relative tolerance {}",
                tolerance.absolute(),
                tolerance.relative()
            ),
            format!("{expected:?}"),
            format!("{actual:?}"),
        ))
    }
}

// =============================================================================
// Option assertions
// =============================================================================

/// Asserts that an Option contains a value.
///
/// Returns a reference to the contained value.
pub fn assert_some<T>(
    value: &Option<T>,
) -> AssertionResult<&T> {
    match value {
        Some(value) => Ok(value),
        None => Err(AssertionFailure::new(
            "assert_some",
            "expected Some(value), received None",
        )),
    }
}

/// Asserts that an Option contains no value.
#[must_use]
pub fn assert_none<T>(
    value: &Option<T>,
) -> AssertionResult {
    match value {
        None => Ok(()),
        Some(_) => Err(AssertionFailure::new(
            "assert_none",
            "expected None, received Some(value)",
        )),
    }
}

// =============================================================================
// Result assertions
// =============================================================================

/// Asserts that a Result is successful.
///
/// Returns a reference to the successful value.
pub fn assert_ok<T, E>(
    value: &Result<T, E>,
) -> AssertionResult<&T>
where
    E: fmt::Debug,
{
    match value {
        Ok(value) => Ok(value),
        Err(error) => Err(AssertionFailure::new(
            "assert_ok",
            format!("expected Ok(value), received Err({error:?})"),
        )),
    }
}

/// Asserts that a Result contains an error.
///
/// Returns a reference to the error value.
pub fn assert_err<T, E>(
    value: &Result<T, E>,
) -> AssertionResult<&E> {
    match value {
        Err(error) => Ok(error),
        Ok(_) => Err(AssertionFailure::new(
            "assert_err",
            "expected Err(error), received Ok(value)",
        )),
    }
}

// =============================================================================
// String assertions
// =============================================================================

/// Asserts that a string contains the requested substring.
#[must_use]
pub fn assert_str_contains(
    actual: &str,
    expected_substring: &str,
) -> AssertionResult {
    if actual.contains(expected_substring) {
        Ok(())
    } else {
        Err(AssertionFailure::with_values(
            "assert_str_contains",
            "string does not contain the expected substring",
            format!("{expected_substring:?}"),
            format!("{actual:?}"),
        ))
    }
}

/// Asserts that a string starts with the requested prefix.
#[must_use]
pub fn assert_str_starts_with(
    actual: &str,
    expected_prefix: &str,
) -> AssertionResult {
    if actual.starts_with(expected_prefix) {
        Ok(())
    } else {
        Err(AssertionFailure::with_values(
            "assert_str_starts_with",
            "string does not start with the expected prefix",
            format!("{expected_prefix:?}"),
            format!("{actual:?}"),
        ))
    }
}

/// Asserts that a string ends with the requested suffix.
#[must_use]
pub fn assert_str_ends_with(
    actual: &str,
    expected_suffix: &str,
) -> AssertionResult {
    if actual.ends_with(expected_suffix) {
        Ok(())
    } else {
        Err(AssertionFailure::with_values(
            "assert_str_ends_with",
            "string does not end with the expected suffix",
            format!("{expected_suffix:?}"),
            format!("{actual:?}"),
        ))
    }
}

// =============================================================================
// Collection assertions
// =============================================================================

/// Asserts that an iterator contains a particular value.
///
/// The iterator is consumed in its supplied order.
///
/// The search stops at the first matching value.
#[must_use]
pub fn assert_iter_contains<I, T>(
    values: I,
    expected: &T,
) -> AssertionResult
where
    I: IntoIterator<Item = T>,
    T: PartialEq + fmt::Debug,
{
    if values.into_iter().any(|value| value == *expected) {
        Ok(())
    } else {
        Err(AssertionFailure::new(
            "assert_iter_contains",
            format!("expected value was not found: {expected:?}"),
        ))
    }
}

/// Asserts that a slice contains exactly the expected elements in order.
#[must_use]
pub fn assert_slice_eq<T>(
    expected: &[T],
    actual: &[T],
) -> AssertionResult
where
    T: PartialEq + fmt::Debug,
{
    if expected == actual {
        Ok(())
    } else {
        Err(AssertionFailure::with_values(
            "assert_slice_eq",
            "slices differ in length or element values",
            format!("{expected:?}"),
            format!("{actual:?}"),
        ))
    }
}

/// Asserts that a slice has the expected length.
#[must_use]
pub fn assert_slice_len<T>(
    actual: &[T],
    expected_length: usize,
) -> AssertionResult {
    if actual.len() == expected_length {
        Ok(())
    } else {
        Err(AssertionFailure::with_values(
            "assert_slice_len",
            "slice length differs from the expected length",
            expected_length.to_string(),
            actual.len().to_string(),
        ))
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Failure model
    // -------------------------------------------------------------------------

    #[test]
    fn failure_preserves_expected_and_actual_values() {
        let failure = AssertionFailure::with_values(
            "example",
            "values differ",
            "10",
            "20",
        );

        assert_eq!(failure.name(), "example");
        assert_eq!(failure.message(), "values differ");
        assert_eq!(failure.expected(), Some("10"));
        assert_eq!(failure.actual(), Some("20"));
        assert!(failure.has_values());
    }

    #[test]
    fn failure_preserves_context_order() {
        let failure = AssertionFailure::new(
            "example",
            "failed",
        )
        .with_context("outer")
        .with_context("inner");

        assert_eq!(
            failure.context(),
            &["outer".to_string(), "inner".to_string()]
        );
    }

    #[test]
    fn failure_display_contains_useful_information() {
        let failure = AssertionFailure::with_values(
            "comparison",
            "mismatch",
            "expected",
            "actual",
        );

        let message = failure.to_string();

        assert!(message.contains("comparison"));
        assert!(message.contains("mismatch"));
        assert!(message.contains("expected"));
        assert!(message.contains("actual"));
    }

    // -------------------------------------------------------------------------
    // Boolean assertions
    // -------------------------------------------------------------------------

    #[test]
    fn true_assertion_accepts_true() {
        assert!(assert_true(true, "condition").is_ok());
    }

    #[test]
    fn true_assertion_rejects_false() {
        assert!(assert_true(false, "condition").is_err());
    }

    #[test]
    fn false_assertion_accepts_false() {
        assert!(assert_false(false, "condition").is_ok());
    }

    #[test]
    fn false_assertion_rejects_true() {
        assert!(assert_false(true, "condition").is_err());
    }

    // -------------------------------------------------------------------------
    // Equality assertions
    // -------------------------------------------------------------------------

    #[test]
    fn equality_assertion_accepts_equal_values() {
        assert!(assert_eq(&42_u64, &42_u64).is_ok());
    }

    #[test]
    fn equality_assertion_rejects_different_values() {
        assert!(assert_eq(&42_u64, &43_u64).is_err());
    }

    #[test]
    fn inequality_assertion_accepts_different_values() {
        assert!(assert_ne(&42_u64, &43_u64).is_ok());
    }

    #[test]
    fn inequality_assertion_rejects_equal_values() {
        assert!(assert_ne(&42_u64, &42_u64).is_err());
    }

    // -------------------------------------------------------------------------
    // Predicate assertions
    // -------------------------------------------------------------------------

    #[test]
    fn predicate_assertion_evaluates_once() {
        use std::cell::Cell;

        let calls = Cell::new(0);

        let result = assert_predicate(
            "positive",
            &10,
            |value| {
                calls.set(calls.get() + 1);
                *value > 0
            },
            "value must be positive",
        );

        assert!(result.is_ok());
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn predicate_assertion_reports_failure() {
        let result = assert_predicate(
            "positive",
            &-10,
            |value| *value > 0,
            "value must be positive",
        );

        assert!(result.is_err());
    }

    // -------------------------------------------------------------------------
    // Numerical comparisons
    // -------------------------------------------------------------------------

    #[test]
    fn tolerance_accepts_exact_values() {
        let tolerance = Tolerance::exact();

        assert!(tolerance.matches(10.0, 10.0));
    }

    #[test]
    fn tolerance_accepts_absolute_difference() {
        let tolerance = Tolerance::new(0.01, 0.0)
            .expect("valid tolerance");

        assert!(tolerance.matches(1.0, 1.005));
    }

    #[test]
    fn tolerance_rejects_excessive_difference() {
        let tolerance = Tolerance::new(0.001, 0.0)
            .expect("valid tolerance");

        assert!(!tolerance.matches(1.0, 2.0));
    }

    #[test]
    fn tolerance_accepts_relative_difference() {
        let tolerance = Tolerance::new(0.0, 0.01)
            .expect("valid tolerance");

        assert!(tolerance.matches(100.0, 100.5));
    }

    #[test]
    fn tolerance_rejects_negative_absolute_value() {
        assert_eq!(
            Tolerance::new(-1.0, 0.0),
            Err(ToleranceError::InvalidAbsolute)
        );
    }

    #[test]
    fn tolerance_rejects_nan_absolute_value() {
        assert_eq!(
            Tolerance::new(f64::NAN, 0.0),
            Err(ToleranceError::InvalidAbsolute)
        );
    }

    #[test]
    fn tolerance_rejects_infinite_relative_value() {
        assert_eq!(
            Tolerance::new(0.0, f64::INFINITY),
            Err(ToleranceError::InvalidRelative)
        );
    }

    #[test]
    fn nan_never_matches_approximately() {
        let tolerance = Tolerance::new(1.0, 1.0)
            .expect("valid tolerance");

        assert!(!tolerance.matches(f64::NAN, f64::NAN));
        assert!(!tolerance.matches(1.0, f64::NAN));
        assert!(!tolerance.matches(f64::NAN, 1.0));
    }

    #[test]
    fn equal_infinities_match() {
        let tolerance = Tolerance::exact();

        assert!(tolerance.matches(
            f64::INFINITY,
            f64::INFINITY
        ));

        assert!(tolerance.matches(
            f64::NEG_INFINITY,
            f64::NEG_INFINITY
        ));
    }

    #[test]
    fn different_infinities_do_not_match() {
        let tolerance = Tolerance::new(1.0, 1.0)
            .expect("valid tolerance");

        assert!(!tolerance.matches(
            f64::INFINITY,
            f64::NEG_INFINITY
        ));
    }

    // -------------------------------------------------------------------------
    // Option and Result
    // -------------------------------------------------------------------------

    #[test]
    fn some_assertion_returns_original_reference() {
        let value = Some(42);

        let result = assert_some(&value)
            .expect("value should exist");

        assert_eq!(*result, 42);
    }

    #[test]
    fn some_assertion_rejects_none() {
        let value: Option<u64> = None;

        assert!(assert_some(&value).is_err());
    }

    #[test]
    fn none_assertion_accepts_none() {
        let value: Option<u64> = None;

        assert!(assert_none(&value).is_ok());
    }

    #[test]
    fn none_assertion_rejects_some() {
        let value = Some(42);

        assert!(assert_none(&value).is_err());
    }

    #[test]
    fn ok_assertion_returns_original_reference() {
        let value: Result<u64, &str> = Ok(42);

        let result = assert_ok(&value)
            .expect("result should be successful");

        assert_eq!(*result, 42);
    }

    #[test]
    fn ok_assertion_preserves_error_information() {
        let value: Result<u64, &str> = Err("failure");

        let failure = assert_ok(&value)
            .expect_err("result should fail");

        assert!(failure.message().contains("failure"));
    }

    #[test]
    fn err_assertion_returns_original_error_reference() {
        let value: Result<u64, &str> = Err("failure");

        let result = assert_err(&value)
            .expect("result should contain an error");

        assert_eq!(*result, "failure");
    }

    #[test]
    fn err_assertion_rejects_successful_result() {
        let value: Result<u64, &str> = Ok(42);

        assert!(assert_err(&value).is_err());
    }

    // -------------------------------------------------------------------------
    // String assertions
    // -------------------------------------------------------------------------

    #[test]
    fn string_contains_expected_substring() {
        assert!(
            assert_str_contains(
                "Zamani programming language",
                "programming"
            )
            .is_ok()
        );
    }

    #[test]
    fn string_contains_empty_substring() {
        assert!(
            assert_str_contains(
                "Zamani",
                ""
            )
            .is_ok()
        );
    }

    #[test]
    fn string_rejects_missing_substring() {
        assert!(
            assert_str_contains(
                "Zamani",
                "Python"
            )
            .is_err()
        );
    }

    #[test]
    fn string_prefix_is_checked() {
        assert!(
            assert_str_starts_with(
                "Zamani",
                "Zam"
            )
            .is_ok()
        );
    }

    #[test]
    fn string_suffix_is_checked() {
        assert!(
            assert_str_ends_with(
                "Zamani",
                "ani"
            )
            .is_ok()
        );
    }

    // -------------------------------------------------------------------------
    // Collection assertions
    // -------------------------------------------------------------------------

    #[test]
    fn iterator_contains_expected_value() {
        let values = [1, 2, 3, 4, 5];

        assert!(
            assert_iter_contains(
                values,
                &3
            )
            .is_ok()
        );
    }

    #[test]
    fn iterator_rejects_missing_value() {
        let values = [1, 2, 3];

        assert!(
            assert_iter_contains(
                values,
                &4
            )
            .is_err()
        );
    }

    #[test]
    fn slice_equality_checks_order() {
        assert!(
            assert_slice_eq(
                &[1, 2, 3],
                &[1, 2, 3]
            )
            .is_ok()
        );

        assert!(
            assert_slice_eq(
                &[1, 2, 3],
                &[3, 2, 1]
            )
            .is_err()
        );
    }

    #[test]
    fn slice_length_is_checked() {
        assert!(
            assert_slice_len(
                &[1, 2, 3],
                3
            )
            .is_ok()
        );

        assert!(
            assert_slice_len(
                &[1, 2, 3],
                4
            )
            .is_err()
        );
    }

    // -------------------------------------------------------------------------
    // Assertion reports
    // -------------------------------------------------------------------------

    #[test]
    fn empty_report_is_successful() {
        let report = AssertionReport::new();

        assert!(report.is_successful());
        assert!(!report.has_failures());
        assert_eq!(report.failure_count(), 0);
    }

    #[test]
    fn report_records_failures() {
        let mut report = AssertionReport::new();

        report.record(Ok(()));
        report.record(Err(
            AssertionFailure::new(
                "example",
                "failure",
            )
        ));

        assert!(!report.is_successful());
        assert!(report.has_failures());
        assert_eq!(report.failure_count(), 1);
    }

    #[test]
    fn reports_can_be_combined() {
        let mut first = AssertionReport::new();
        first.push(
            AssertionFailure::new("first", "failure")
        );

        let mut second = AssertionReport::new();
        second.push(
            AssertionFailure::new("second", "failure")
        );

        first.extend(second);

        assert_eq!(first.failure_count(), 2);
    }

    #[test]
    fn report_result_preserves_all_failures() {
        let mut report = AssertionReport::new();

        report.push(
            AssertionFailure::new("first", "failure")
        );

        report.push(
            AssertionFailure::new("second", "failure")
        );

        let failures = report
            .into_result()
            .expect_err("report should fail");

        assert_eq!(failures.len(), 2);
    }

    #[test]
    fn evaluate_all_collects_every_failure() {
        let report = evaluate_all([
            Ok(()),
            Err(AssertionFailure::new("one", "failed")),
            Ok(()),
            Err(AssertionFailure::new("two", "failed")),
        ]);

        assert_eq!(report.failure_count(), 2);
    }

    // -------------------------------------------------------------------------
    // Scalability and boundary behavior
    // -------------------------------------------------------------------------

    #[test]
    fn empty_collection_length_is_supported() {
        assert!(
            assert_slice_len::<u64>(
                &[],
                0
            )
            .is_ok()
        );
    }

    #[test]
    fn empty_slices_compare_correctly() {
        assert!(
            assert_slice_eq::<u64>(
                &[],
                &[]
            )
            .is_ok()
        );
    }

    #[test]
    fn empty_strings_are_supported() {
        assert!(
            assert_str_contains(
                "",
                ""
            )
            .is_ok()
        );

        assert!(
            assert_str_starts_with(
                "",
                ""
            )
            .is_ok()
        );

        assert!(
            assert_str_ends_with(
                "",
                ""
            )
            .is_ok()
        );
    }

    #[test]
    fn zero_tolerance_requires_exact_finite_equality() {
        let tolerance = Tolerance::exact();

        assert!(tolerance.matches(0.0, -0.0));
        assert!(!tolerance.matches(0.0, f64::EPSILON));
    }
}
